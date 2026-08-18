#![forbid(unsafe_code)]

use event_correlator::{evaluate_event, SafetyEvent};
use flock_adapter::{normalize_event, AdapterError, ExternalCameraEvent, NormalizedEvent};
use subtle::ConstantTimeEq;

pub fn token_is_acceptable(token: &str) -> bool {
    let trimmed = token.trim();
    !trimmed.is_empty()
        && !matches!(trimmed, "REPLACE_ME" | "replace-me" | "replace-with-a-random-secret")
}

pub fn authorize_header(header: Option<&str>, expected_token: &str) -> bool {
    if !token_is_acceptable(expected_token) {
        return false;
    }
    let expected = format!("Bearer {expected_token}");
    header
        .map(|provided| bool::from(provided.as_bytes().ct_eq(expected.as_bytes())))
        .unwrap_or(false)
}

pub fn process_camera_event(event: ExternalCameraEvent) -> Result<NormalizedEvent, AdapterError> {
    normalize_event(event)
}

pub fn process_safety_event(event: ExternalCameraEvent) -> Result<SafetyEvent, AdapterError> {
    let normalized = normalize_event(event)?;
    Ok(evaluate_event(&normalized))
}
