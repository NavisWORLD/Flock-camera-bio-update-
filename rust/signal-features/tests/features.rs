use signal_core::{ObservationWindow, SensorFrame, SourceKind};
use signal_features::FeatureExtractor;
use std::collections::BTreeMap;
use uuid::Uuid;

fn window(samples: Vec<f32>, sample_rate_hz: f64) -> ObservationWindow {
    ObservationWindow {
        window_id: Uuid::nil(),
        start_ns: 0,
        end_ns: 1_000,
        frames: vec![SensorFrame {
            sensor_id: "sim-1".into(),
            source_kind: SourceKind::Generic,
            timestamp_ns: 0,
            sample_rate_hz,
            channels: vec![samples],
            metadata: BTreeMap::new(),
        }],
    }
}

#[test]
fn extracts_rms_zero_crossing_and_spectral_centroid() {
    let template = FeatureExtractor::default()
        .extract(&window(vec![1.0, -1.0, 1.0, -1.0], 4.0))
        .unwrap();

    assert_eq!(template.feature_schema, "fs-basic");
    assert_eq!(template.feature_version, "1");
    assert_eq!(template.features.len(), 3);
    assert!((template.features[0] - 1.0).abs() < 1e-6);
    assert!((template.features[1] - 1.0).abs() < 1e-6);
    assert!((template.features[2] - 2.0).abs() < 1e-5);
    assert!(template.quality > 0.0 && template.quality <= 1.0);
}

#[test]
fn rejects_non_positive_sample_rate() {
    let err = FeatureExtractor::default()
        .extract(&window(vec![0.0, 1.0], 0.0))
        .unwrap_err();
    assert!(err.to_string().contains("sample rate"));
}
