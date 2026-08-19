use serde::Deserialize;
use signal_core::CameraEvent;
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("invalid camera event JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("camera_id must not be blank")]
    MissingCameraId,
    #[error("event_kind must not be blank")]
    MissingEventKind,
}

#[derive(Debug, Deserialize)]
struct RawCameraEvent {
    camera_id: String,
    observed_at_ns: i128,
    #[serde(default)]
    zone_id: Option<String>,
    event_kind: String,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
    #[serde(default)]
    source_uri: Option<String>,
}

pub struct FlockAdapter;

impl FlockAdapter {
    pub fn normalize_json(input: &str) -> Result<CameraEvent, AdapterError> {
        let raw: RawCameraEvent = serde_json::from_str(input)?;
        if raw.camera_id.trim().is_empty() {
            return Err(AdapterError::MissingCameraId);
        }
        if raw.event_kind.trim().is_empty() {
            return Err(AdapterError::MissingEventKind);
        }

        let mut attributes = raw.attributes;
        if let Some(value) = attributes.get_mut("face_occlusion") {
            *value = match value.trim().to_ascii_lowercase().as_str() {
                "present" | "true" | "yes" => "present".into(),
                "absent" | "false" | "no" => "absent".into(),
                _ => "unknown".into(),
            };
        }

        let mut event = CameraEvent::new(
            raw.camera_id,
            raw.observed_at_ns,
            raw.zone_id,
            raw.event_kind,
            attributes,
        );
        event.source_uri = raw.source_uri;
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_authorized_camera_event_fixture() {
        let json = r#"{
            "camera_id":"flock-sim-17",
            "observed_at_ns":1724012345000000000,
            "zone_id":"school-north",
            "event_kind":"vehicle_or_person_observation",
            "attributes":{"face_occlusion":"present","restricted_zone":"true"},
            "source_uri":"fixture://event-17"
        }"#;
        let event = FlockAdapter::normalize_json(json).expect("normalize");
        assert_eq!(event.camera_id, "flock-sim-17");
        assert_eq!(event.zone_id.as_deref(), Some("school-north"));
        assert_eq!(
            event.attributes.get("face_occlusion").map(String::as_str),
            Some("present")
        );
    }

    #[test]
    fn invalid_occlusion_value_is_reduced_to_unknown() {
        let json = r#"{
            "camera_id":"sim",
            "observed_at_ns":1,
            "event_kind":"observation",
            "attributes":{"face_occlusion":"maybe-person"}
        }"#;
        let event = FlockAdapter::normalize_json(json).expect("normalize");
        assert_eq!(
            event.attributes.get("face_occlusion").map(String::as_str),
            Some("unknown")
        );
    }

    #[test]
    fn missing_camera_id_is_rejected() {
        let json = r#"{"observed_at_ns":1,"event_kind":"observation"}"#;
        assert!(FlockAdapter::normalize_json(json).is_err());
    }
}
