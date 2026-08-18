use flock_adapter::ExternalCameraEvent;
use flock_signal_gateway::{authorize_header, process_camera_event};
use std::collections::BTreeMap;
use uuid::Uuid;

#[test]
fn bearer_authorization_is_deny_by_default() {
    assert!(!authorize_header(None, "secret-token"));
    assert!(!authorize_header(Some("Bearer wrong"), "secret-token"));
    assert!(authorize_header(Some("Bearer secret-token"), "secret-token"));
}

#[test]
fn gateway_normalizes_camera_event_without_identity_resolution() {
    let mut attributes = BTreeMap::new();
    attributes.insert("face_occlusion".into(), "present".into());
    let normalized = process_camera_event(ExternalCameraEvent {
        event_id: Uuid::nil(),
        camera_id: "camera-a".into(),
        observed_at_ns: 1,
        zone_id: Some("zone-a".into()),
        event_kind: "motion".into(),
        attributes,
        source_uri: Some("authorized-adapter://event/a".into()),
    })
    .unwrap();
    assert_eq!(normalized.camera_id, "camera-a");
}
