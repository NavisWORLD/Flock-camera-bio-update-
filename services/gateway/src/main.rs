use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use flock_adapter::{ExternalCameraEvent, NormalizedEvent};
use flock_signal_gateway::{authorize_header, process_camera_event};
use serde_json::{json, Value};
use std::{env, sync::Arc};

#[derive(Clone)]
struct AppState {
    api_token: String,
}

#[tokio::main]
async fn main() {
    let api_token = env::var("FLOCK_SIGNAL_API_TOKEN")
        .expect("FLOCK_SIGNAL_API_TOKEN must be set; refusing unauthenticated startup");
    if api_token.trim().is_empty() {
        panic!("FLOCK_SIGNAL_API_TOKEN must not be empty");
    }
    let bind = env::var("FLOCK_SIGNAL_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let state = Arc::new(AppState { api_token });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/camera-events", post(ingest_camera_event))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .unwrap_or_else(|error| panic!("failed to bind {bind}: {error}"));
    println!("flock-signal-gateway listening on {bind}");
    axum::serve(listener, app)
        .await
        .expect("gateway server failed");
}

async fn healthz() -> Json<Value> {
    Json(json!({"status": "ok", "identity_resolution": "disabled"}))
}

async fn ingest_camera_event(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(event): Json<ExternalCameraEvent>,
) -> Result<Json<NormalizedEvent>, (StatusCode, Json<Value>)> {
    let authorization = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    if !authorize_header(authorization, &state.api_token) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "unauthorized"})),
        ));
    }

    process_camera_event(event).map(Json).map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": error.to_string()})),
        )
    })
}
