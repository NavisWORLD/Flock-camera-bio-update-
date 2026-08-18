#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::{ObservationWindow, SensorFrame, SourceKind};
    use std::collections::BTreeMap;

    fn sine_window() -> ObservationWindow {
        let sample_rate = 64.0;
        let samples = (0..64)
            .map(|i| (2.0 * std::f32::consts::PI * 8.0 * i as f32 / sample_rate as f32).sin())
            .collect::<Vec<_>>();
        ObservationWindow::new(
            0,
            1_000_000_000,
            vec![SensorFrame {
                sensor_id: "sim-1".into(),
                source_kind: SourceKind::Mechanical,
                timestamp_ns: 0,
                sample_rate_hz: sample_rate,
                channels: vec![samples],
                metadata: BTreeMap::new(),
            }],
        )
    }

    #[test]
    fn extracts_stable_features_for_known_sine_wave() {
        let window = sine_window();
        let a = FeatureExtractor::default().extract(&window).expect("extract");
        let b = FeatureExtractor::default().extract(&window).expect("extract");
        assert_eq!(a.feature_schema, "flock-signal-basic-v1");
        assert_eq!(a.features, b.features);
        assert_eq!(a.source_digest, b.source_digest);
        assert!((a.features[2] - 8.0).abs() < 0.75, "centroid should be near 8 Hz");
    }

    #[test]
    fn rejects_empty_observation_windows() {
        let window = ObservationWindow::new(0, 1, Vec::new());
        assert!(matches!(FeatureExtractor::default().extract(&window), Err(FeatureError::NoSamples)));
    }

    #[test]
    fn quality_and_uncertainty_are_bounded() {
        let template = FeatureExtractor::default().extract(&sine_window()).expect("extract");
        assert!((0.0..=1.0).contains(&template.quality));
        assert!((0.0..=1.0).contains(&template.uncertainty));
    }
}
