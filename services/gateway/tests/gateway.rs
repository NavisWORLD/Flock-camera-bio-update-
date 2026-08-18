use event_correlator::SafetySeverity;
use flock_adapter::ExternalCameraEvent;
use flock_signal_gateway::{authorize_header, process_camera_event, process_safety_event, token_is_acceptable};
use std::collections::BTreeMap;
use uuid::Uuid;

fn event(masked: bool, restricted: bool, after_hours: bool) -> ExternalCameraEvent {
    let mut attributes = BTreeMap::new();
    attributes.insert(
        "face_occlusion".into(),
        if masked { "present" } else { "absent" }.into(),
    );
    attributes.insert("restricted_zone".into(), restricted.to_string());
    attributes.insert("after_hours".into(), after_hours.to_string());
    ExternalCameraEvent {
        event_id: Uuid::nil(),
        camera_id: "camera-a".into(),
        observed_at_ns: 1,
        zone_id: Some("zone-a".into()),
        event_kind: "motion".into(),
        attributes,
        source_uri: Some("authorized-adapter://event/a".into()),
    }
}

#[test]
fn bearer_authorization_is_deny_by_default() {
    assert!(!authorize_header(None, "secret-token"));
    assert!(!authorize_header(Some("Bearer wrong"), "secret-token"));
    assert!(authorize_header(Some("Bearer secret-token"), "secret-token"));
}

#[test]
fn known_placeholder_tokens_are_rejected_for_startup() {
    assert!(!token_is_acceptable(""));
    assert!(!token_is_acceptable("REPLACE_ME"));
    assert!(!token_is_acceptable("replace-with-a-random-secret"));
    assert!(token_is_acceptable("a-real-local-development-secret"));
}

#[test]
fn gateway_normalizes_camera_event_without_identity_resolution() {
    let normalized = process_camera_event(event(true, false, false)).unwrap();
    assert_eq!(normalized.camera_id, "camera-a");
}

#[test]
fn gateway_keeps_mask_only_event_informational() {
    let safety = process_safety_event(event(true, false, false)).unwrap();
    assert_eq!(safety.severity, SafetySeverity::Informational);
}

#[test]
fn gateway_elevates_independent_restricted_zone_after_hours_context() {
    let safety = process_safety_event(event(true, true, true)).unwrap();
    assert_eq!(safety.severity, SafetySeverity::Elevated);
    assert!(safety.human_review_required);
}
