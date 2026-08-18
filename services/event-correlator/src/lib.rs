#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::{CameraEvent, ObservationWindow, SensorFrame, SignalTemplate, SourceKind};
    use std::collections::BTreeMap;

    fn window(zone: &str) -> ObservationWindow {
        let mut metadata = BTreeMap::new();
        metadata.insert("zone_id".into(), zone.into());
        ObservationWindow::new(
            900,
            1_100,
            vec![SensorFrame {
                sensor_id: "sensor-a".into(),
                source_kind: SourceKind::Mechanical,
                timestamp_ns: 1_000,
                sample_rate_hz: 10.0,
                channels: vec![vec![0.0, 1.0]],
                metadata,
            }],
        )
    }

    fn template() -> SignalTemplate {
        SignalTemplate::new("schema".into(), "1".into(), vec![0.2], 1.0, 0.0, [1; 32])
    }

    #[test]
    fn matching_time_and_zone_produce_safety_event() {
        let event = CameraEvent::new(
            "cam".into(),
            1_000,
            Some("north".into()),
            "observation".into(),
            BTreeMap::new(),
        );
        assert!(Correlator::default().correlate(&event, &window("north"), &template()).is_some());
    }

    #[test]
    fn mismatched_zone_is_not_correlated() {
        let event = CameraEvent::new(
            "cam".into(),
            1_000,
            Some("south".into()),
            "observation".into(),
            BTreeMap::new(),
        );
        assert!(Correlator::default().correlate(&event, &window("north"), &template()).is_none());
    }

    #[test]
    fn occlusion_alone_remains_low_severity() {
        let mut attrs = BTreeMap::new();
        attrs.insert("face_occlusion".into(), "present".into());
        let event = CameraEvent::new("cam".into(), 1_000, Some("north".into()), "observation".into(), attrs);
        let safety = Correlator::default().correlate(&event, &window("north"), &template()).expect("correlate");
        assert!(safety.severity < 50);
    }
}
