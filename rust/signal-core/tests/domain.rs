use signal_core::{digest_observation, ObservationWindow, SensorFrame, SignalTemplate, SourceKind};
use std::collections::BTreeMap;
use uuid::Uuid;

fn sample_window() -> ObservationWindow {
    ObservationWindow {
        window_id: Uuid::nil(),
        start_ns: 1_000,
        end_ns: 2_000,
        frames: vec![SensorFrame {
            sensor_id: "sensor-a".into(),
            source_kind: SourceKind::Generic,
            timestamp_ns: 1_000,
            sample_rate_hz: 4.0,
            channels: vec![vec![1.0, -1.0, 1.0, -1.0]],
            metadata: BTreeMap::new(),
        }],
    }
}

#[test]
fn observation_digest_is_deterministic() {
    let window = sample_window();
    assert_eq!(digest_observation(&window), digest_observation(&window));
}

#[test]
fn serialized_signal_template_contains_no_identity_fields() {
    let value = SignalTemplate {
        template_id: Uuid::nil(),
        feature_schema: "fs-basic".into(),
        feature_version: "1".into(),
        features: vec![1.0, 2.0],
        quality: 0.9,
        uncertainty: 0.1,
        source_digest: [7_u8; 32],
    };
    let json = serde_json::to_string(&value).unwrap();
    for forbidden in ["person_id", "government_id", "face_embedding", "full_name"] {
        assert!(!json.contains(forbidden), "unexpected identity field: {forbidden}");
    }
}
