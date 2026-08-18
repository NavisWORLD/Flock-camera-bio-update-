use event_correlator::{evaluate_event, SafetySeverity};
use flock_adapter::{NormalizedEvent, Occlusion};
use std::collections::BTreeMap;
use uuid::Uuid;

fn event(occlusion: Occlusion, restricted: bool, after_hours: bool) -> NormalizedEvent {
    let mut attributes = BTreeMap::new();
    attributes.insert("restricted_zone".into(), restricted.to_string());
    attributes.insert("after_hours".into(), after_hours.to_string());
    NormalizedEvent {
        event_id: Uuid::nil(),
        camera_id: "cam-test".into(),
        observed_at_ns: 1,
        zone_id: Some("test-zone".into()),
        event_kind: "motion".into(),
        attributes,
        face_occlusion: occlusion,
        source_uri: Some("simulator://test".into()),
    }
}

#[test]
fn mask_alone_does_not_elevate_event() {
    let result = evaluate_event(&event(Occlusion::Present, false, false));
    assert_eq!(result.severity, SafetySeverity::Informational);
    assert!(result.human_review_required);
}

#[test]
fn independent_restricted_zone_and_after_hours_predicates_elevate_event() {
    let result = evaluate_event(&event(Occlusion::Absent, true, true));
    assert_eq!(result.severity, SafetySeverity::Elevated);
    assert_eq!(result.rule_id, "restricted-zone-after-hours");
}

#[test]
fn occlusion_remains_context_only_when_event_is_elevated() {
    let result = evaluate_event(&event(Occlusion::Present, true, true));
    assert_eq!(result.severity, SafetySeverity::Elevated);
    assert_eq!(result.rationale.get("face_occlusion").map(String::as_str), Some("context_only:present"));
    assert_eq!(result.rationale.get("guilt_determination").map(String::as_str), Some("none"));
}
