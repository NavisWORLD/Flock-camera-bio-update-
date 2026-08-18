use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use event_correlator::Correlator;
use flock_adapter::FlockAdapter;
use policy_engine::{PolicyAction, PolicyContext, PolicyDecision, PolicyEngine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use signal_core::{CameraEvent, ObservationWindow, SafetyEvent, SignalTemplate};
use signal_features::FeatureExtractor;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub bind: String,
}

impl AppConfig {
    pub fn from_map(values: &BTreeMap<String, String>) -> Self {
        Self {
            bind: values
                .get("FLOCK_SIGNAL_BIND")
                .cloned()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "0.0.0.0:8080".into()),
        }
    }

    pub fn from_env() -> Self {
        let mut values = BTreeMap::new();
        if let Ok(value) = std::env::var("FLOCK_SIGNAL_BIND") {
            values.insert("FLOCK_SIGNAL_BIND".into(), value);
        }
        Self::from_map(&values)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HealthPayload {
    pub status: &'static str,
    pub service: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct ApiError {
    error: String,
}

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<ApiError>)>;

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyRequest {
    pub action: PolicyAction,
    pub context: PolicyContext,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorrelationRequest {
    pub camera_event: CameraEvent,
    pub observation_window: ObservationWindow,
    pub signal_template: SignalTemplate,
}

pub fn health_payload() -> HealthPayload {
    HealthPayload {
        status: "ok",
        service: "flock-signal-gateway",
    }
}

async fn health() -> Json<HealthPayload> {
    Json(health_payload())
}

async fn normalize_camera(Json(value): Json<Value>) -> ApiResult<CameraEvent> {
    let encoded = serde_json::to_string(&value).map_err(internal_error)?;
    FlockAdapter::normalize_json(&encoded)
        .map(Json)
        .map_err(|error| bad_request(error.to_string()))
}

async fn extract_signal(Json(window): Json<ObservationWindow>) -> ApiResult<SignalTemplate> {
    FeatureExtractor::default()
        .extract(&window)
        .map(Json)
        .map_err(|error| bad_request(error.to_string()))
}

async fn evaluate_policy(Json(request): Json<PolicyRequest>) -> Json<PolicyDecision> {
    Json(PolicyEngine.evaluate(request.action, &request.context))
}

async fn correlate(Json(request): Json<CorrelationRequest>) -> ApiResult<SafetyEvent> {
    Correlator::default()
        .correlate(
            &request.camera_event,
            &request.observation_window,
            &request.signal_template,
        )
        .map(Json)
        .ok_or_else(|| unprocessable("camera event and observation did not satisfy time/zone correlation"))
}

fn bad_request(message: String) -> (StatusCode, Json<ApiError>) {
    (StatusCode::BAD_REQUEST, Json(ApiError { error: message }))
}

fn unprocessable(message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(ApiError {
            error: message.into(),
        }),
    )
}

fn internal_error(error: serde_json::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: error.to_string(),
        }),
    )
}

async fn request_id(mut request: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(request_id.clone());
    let mut response = next.run(request).await;
    if let Ok(value) = HeaderValue::from_str(&request_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-request-id"), value);
    }
    response
}

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/v1/camera/normalize", post(normalize_camera))
        .route("/v1/signal/extract", post(extract_signal))
        .route("/v1/policy/evaluate", post(evaluate_policy))
        .route("/v1/correlate", post(correlate))
        .layer(middleware::from_fn(request_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_is_stable() {
        let config = AppConfig::from_map(&BTreeMap::new());
        assert_eq!(config.bind, "0.0.0.0:8080");
    }

    #[test]
    fn explicit_bind_overrides_default() {
        let mut values = BTreeMap::new();
        values.insert("FLOCK_SIGNAL_BIND".into(), "127.0.0.1:9090".into());
        let config = AppConfig::from_map(&values);
        assert_eq!(config.bind, "127.0.0.1:9090");
    }

    #[test]
    fn health_payload_reports_ok() {
        let payload = health_payload();
        assert_eq!(payload.status, "ok");
        assert_eq!(payload.service, "flock-signal-gateway");
    }

    #[test]
    fn policy_engine_keeps_mask_only_non_elevated() {
        let request = PolicyRequest {
            action: PolicyAction::RestrictedZoneReview,
            context: PolicyContext {
                face_occlusion: Some(true),
                ..PolicyContext::default()
            },
        };
        let decision = PolicyEngine.evaluate(request.action, &request.context);
        assert!(decision.allowed);
        assert!(!decision.elevated);
    }
}
