#![forbid(unsafe_code)]

use flock_adapter::{normalize_event, AdapterError, ExternalCameraEvent, NormalizedEvent};
use subtle::ConstantTimeEq;

pub fn authorize_header(header: Option<&str>, expected_token: &str) -> bool {
    if expected_token.is_empty() {
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
