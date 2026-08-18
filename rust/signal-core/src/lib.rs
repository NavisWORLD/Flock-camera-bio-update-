#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

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
        assert_eq!(event.attributes.get("face_occlusion").map(String::as_str), Some("present"));
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
        for forbidden in ["person_name", "person_id", "government_id", "face_embedding"] {
            assert!(!json.contains(forbidden), "forbidden field leaked: {forbidden}");
        }
    }
}
