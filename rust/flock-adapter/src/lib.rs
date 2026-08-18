#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_authorized_camera_event_fixture() {
        let json = r#"{
            "camera_id":"flock-sim-17",
            "observed_at_ns":1724012345000000000,
            "zone_id":"school-north",
            "event_kind":"vehicle_or_person_observation",
            "attributes":{"face_occlusion":"present","restricted_zone":"true"},
            "source_uri":"fixture://event-17"
        }"#;
        let event = FlockAdapter::normalize_json(json).expect("normalize");
        assert_eq!(event.camera_id, "flock-sim-17");
        assert_eq!(event.zone_id.as_deref(), Some("school-north"));
        assert_eq!(event.attributes.get("face_occlusion").map(String::as_str), Some("present"));
    }

    #[test]
    fn invalid_occlusion_value_is_reduced_to_unknown() {
        let json = r#"{
            "camera_id":"sim",
            "observed_at_ns":1,
            "event_kind":"observation",
            "attributes":{"face_occlusion":"maybe-person"}
        }"#;
        let event = FlockAdapter::normalize_json(json).expect("normalize");
        assert_eq!(event.attributes.get("face_occlusion").map(String::as_str), Some("unknown"));
    }

    #[test]
    fn missing_camera_id_is_rejected() {
        let json = r#"{"observed_at_ns":1,"event_kind":"observation"}"#;
        assert!(FlockAdapter::normalize_json(json).is_err());
    }
}
