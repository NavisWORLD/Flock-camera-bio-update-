use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Camera,
    Acoustic,
    Mechanical,
    Magnetic,
    Thermal,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorFrame {
    pub sensor_id: String,
    pub source_kind: SourceKind,
    pub timestamp_ns: i128,
    pub sample_rate_hz: f64,
    pub channels: Vec<Vec<f32>>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObservationWindow {
    pub window_id: Uuid,
    pub start_ns: i128,
    pub end_ns: i128,
    pub frames: Vec<SensorFrame>,
}

impl ObservationWindow {
    pub fn new(start_ns: i128, end_ns: i128, frames: Vec<SensorFrame>) -> Self {
        Self {
            window_id: Uuid::new_v4(),
            start_ns,
            end_ns,
            frames,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignalTemplate {
    pub template_id: Uuid,
    pub feature_schema: String,
    pub feature_version: String,
    pub features: Vec<f32>,
    pub quality: f32,
    pub uncertainty: f32,
    pub source_digest: [u8; 32],
}

impl SignalTemplate {
    pub fn new(
        feature_schema: String,
        feature_version: String,
        features: Vec<f32>,
        quality: f32,
        uncertainty: f32,
        source_digest: [u8; 32],
    ) -> Self {
        Self {
            template_id: Uuid::new_v4(),
            feature_schema,
            feature_version,
            features,
            quality: quality.clamp(0.0, 1.0),
            uncertainty: uncertainty.clamp(0.0, 1.0),
            source_digest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CameraEvent {
    pub event_id: Uuid,
    pub camera_id: String,
    pub observed_at_ns: i128,
    pub zone_id: Option<String>,
    pub event_kind: String,
    pub attributes: BTreeMap<String, String>,
    pub source_uri: Option<String>,
}

impl CameraEvent {
    pub fn new(
        camera_id: String,
        observed_at_ns: i128,
        zone_id: Option<String>,
        event_kind: String,
        attributes: BTreeMap<String, String>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            camera_id,
            observed_at_ns,
            zone_id,
            event_kind,
            attributes,
            source_uri: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SafetyEvent {
    pub event_id: Uuid,
    pub camera_event_id: Uuid,
    pub template_ids: Vec<Uuid>,
    pub severity: u8,
    pub requires_human_review: bool,
    pub reasons: Vec<String>,
}

impl SafetyEvent {
    pub fn new(
        camera_event_id: Uuid,
        template_ids: Vec<Uuid>,
        severity: u8,
        reasons: Vec<String>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            camera_event_id,
            template_ids,
            severity: severity.min(100),
            requires_human_review: true,
            reasons,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_template_clamps_quality_and_uncertainty() {
        let template = SignalTemplate::new(
            "schema-v1".into(),
            "1.0.0".into(),
            vec![0.1, 0.2, 0.3],
            1.5,
            -0.2,
            [7_u8; 32],
        );
        assert_eq!(template.quality, 1.0);
        assert_eq!(template.uncertainty, 0.0);
    }

    #[test]
    fn camera_event_accepts_occlusion_as_context_only() {
        let mut attributes = BTreeMap::new();
        attributes.insert("face_occlusion".to_string(), "present".to_string());
        let event = CameraEvent::new(
            "camera-7".into(),
            123,
            Some("playground-north".into()),
            "observation".into(),
            attributes,
        );
        assert_eq!(
            event.attributes.get("face_occlusion").map(String::as_str),
            Some("present")
        );
        assert_eq!(event.event_kind, "observation");
    }

    #[test]
    fn signal_template_serialization_contains_no_identity_fields() {
        let template = SignalTemplate::new(
            "schema-v1".into(),
            "1.0.0".into(),
            vec![0.5],
            0.9,
            0.1,
            [1_u8; 32],
        );
        let json = serde_json::to_string(&template).expect("serialize template");
        for forbidden in [
            "person_name",
            "person_id",
            "government_id",
            "face_embedding",
        ] {
            assert!(
                !json.contains(forbidden),
                "forbidden field leaked: {forbidden}"
            );
        }
    }

    #[test]
    fn safety_event_always_requires_human_review() {
        let event = SafetyEvent::new(
            Uuid::new_v4(),
            Vec::new(),
            250,
            vec!["restricted-zone".into()],
        );
        assert_eq!(event.severity, 100);
        assert!(event.requires_human_review);
    }
}
