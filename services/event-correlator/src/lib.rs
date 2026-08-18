use policy_engine::{PolicyAction, PolicyContext, PolicyEngine};
use signal_core::{CameraEvent, ObservationWindow, SafetyEvent, SignalTemplate};

#[derive(Debug, Clone)]
pub struct Correlator {
    pub max_clock_skew_ns: i128,
}

impl Default for Correlator {
    fn default() -> Self {
        Self {
            max_clock_skew_ns: 500_000_000,
        }
    }
}

impl Correlator {
    pub fn correlate(
        &self,
        event: &CameraEvent,
        window: &ObservationWindow,
        template: &SignalTemplate,
    ) -> Option<SafetyEvent> {
        let earliest = window.start_ns.saturating_sub(self.max_clock_skew_ns);
        let latest = window.end_ns.saturating_add(self.max_clock_skew_ns);
        if event.observed_at_ns < earliest || event.observed_at_ns > latest {
            return None;
        }
        if !zone_matches(event, window) {
            return None;
        }

        let context = PolicyContext {
            restricted_zone: attribute_flag(event, "restricted_zone"),
            after_hours: attribute_flag(event, "after_hours"),
            face_occlusion: match event.attributes.get("face_occlusion").map(String::as_str) {
                Some("present") => Some(true),
                Some("absent") => Some(false),
                _ => None,
            },
            ..PolicyContext::default()
        };
        let decision = PolicyEngine.evaluate(PolicyAction::RestrictedZoneReview, &context);
        let mut reasons = vec!["time-zone-correlated".into()];
        reasons.extend(decision.reasons);
        let severity = if decision.elevated { 70 } else { 10 };

        Some(SafetyEvent::new(
            event.event_id,
            vec![template.template_id],
            severity,
            reasons,
        ))
    }
}

fn attribute_flag(event: &CameraEvent, key: &str) -> bool {
    event.attributes.get(key).is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "true" | "yes" | "1" | "present"
        )
    })
}

fn zone_matches(event: &CameraEvent, window: &ObservationWindow) -> bool {
    let Some(event_zone) = event.zone_id.as_deref() else {
        return true;
    };

    let mut saw_sensor_zone = false;
    for sensor_zone in window
        .frames
        .iter()
        .filter_map(|frame| frame.metadata.get("zone_id"))
    {
        saw_sensor_zone = true;
        if sensor_zone == event_zone {
            return true;
        }
    }

    !saw_sensor_zone
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::{SensorFrame, SourceKind};
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
        SignalTemplate::new(
            "schema".into(),
            "1".into(),
            vec![0.2],
            1.0,
            0.0,
            [1; 32],
        )
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
        assert!(Correlator::default()
            .correlate(&event, &window("north"), &template())
            .is_some());
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
        assert!(Correlator::default()
            .correlate(&event, &window("north"), &template())
            .is_none());
    }

    #[test]
    fn occlusion_alone_remains_low_severity() {
        let mut attrs = BTreeMap::new();
        attrs.insert("face_occlusion".into(), "present".into());
        let event = CameraEvent::new(
            "cam".into(),
            1_000,
            Some("north".into()),
            "observation".into(),
            attrs,
        );
        let safety = Correlator::default()
            .correlate(&event, &window("north"), &template())
            .expect("correlate");
        assert!(safety.severity < 50);
    }
}
