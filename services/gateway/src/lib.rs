use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Json, Router,
};
use serde::Serialize;
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

pub fn health_payload() -> HealthPayload {
    HealthPayload {
        status: "ok",
        service: "flock-signal-gateway",
    }
}

async fn health() -> Json<HealthPayload> {
    Json(health_payload())
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
}
