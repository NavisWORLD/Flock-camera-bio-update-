use flock_adapter::{normalize_event, ExternalCameraEvent, Occlusion};
use std::collections::BTreeMap;
use uuid::Uuid;

fn base_event() -> ExternalCameraEvent {
    ExternalCameraEvent {
        event_id: Uuid::nil(),
        camera_id: "cam-001".into(),
        observed_at_ns: 123,
        zone_id: Some("school-perimeter".into()),
        event_kind: "motion".into(),
        attributes: BTreeMap::new(),
        source_uri: Some("authorized-adapter://event/123".into()),
    }
}

#[test]
fn normalizes_supported_occlusion_metadata() {
    let mut event = base_event();
    event.attributes.insert("face_occlusion".into(), "present".into());
    let normalized = normalize_event(event).unwrap();
    assert_eq!(normalized.face_occlusion, Occlusion::Present);
    assert_eq!(normalized.camera_id, "cam-001");
}

#[test]
fn rejects_unknown_occlusion_value() {
    let mut event = base_event();
    event.attributes.insert("face_occlusion".into(), "maybe".into());
    assert!(normalize_event(event).is_err());
}

#[test]
fn preserves_source_provenance_without_creating_identity() {
    let normalized = normalize_event(base_event()).unwrap();
    assert_eq!(normalized.source_uri.as_deref(), Some("authorized-adapter://event/123"));
    let json = serde_json::to_string(&normalized).unwrap();
    for forbidden in ["person_id", "full_name", "biometric_identity"] {
        assert!(!json.contains(forbidden));
    }
}
