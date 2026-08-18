#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn default_bind_is_stable() {
        let config = AppConfig::from_map(&BTreeMap::new());
        assert_eq!(config.bind, "0.0.0.0:8080");
    }

    #[test]
    fn explicit_bind_overrides_default() {
        let mut values = BTreeMap::new();
        values.insert("FLOCK_SIGNAL_BIND".into(), "127.0.0.1:9090".into());
        let config = AppConfig::from_map(&values);
        assert_eq!(config.bind, "127.0.0.1:9090");
    }

    #[test]
    fn health_payload_reports_ok() {
        let payload = health_payload();
        assert_eq!(payload.status, "ok");
        assert_eq!(payload.service, "flock-signal-gateway");
    }
}
