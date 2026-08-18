#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn signed_chain_verifies_and_payload_mutation_fails() {
        let signer = LedgerSigner::generate("test-key");
        let first = signer.sign_record(NewLedgerRecord {
            observed_at_ns: 1,
            event_id: Uuid::new_v4(),
            sensor_ids: vec!["sensor-a".into()],
            template_ids: vec![Uuid::new_v4()],
            payload_digest: [3_u8; 32],
            previous_record_digest: [0_u8; 32],
            policy_context: LedgerPolicyContext::default(),
        });
        assert!(verify_chain(&[first.clone()], &signer.verifying_key()).is_ok());

        let mut tampered = first;
        tampered.payload_digest = [9_u8; 32];
        assert!(verify_chain(&[tampered], &signer.verifying_key()).is_err());
    }

    #[test]
    fn reordered_chain_is_rejected() {
        let signer = LedgerSigner::generate("test-key");
        let first = signer.sign_record(NewLedgerRecord {
            observed_at_ns: 1,
            event_id: Uuid::new_v4(),
            sensor_ids: vec!["sensor-a".into()],
            template_ids: vec![],
            payload_digest: [1_u8; 32],
            previous_record_digest: [0_u8; 32],
            policy_context: LedgerPolicyContext::default(),
        });
        let second = signer.sign_record(NewLedgerRecord {
            observed_at_ns: 2,
            event_id: Uuid::new_v4(),
            sensor_ids: vec!["sensor-b".into()],
            template_ids: vec![],
            payload_digest: [2_u8; 32],
            previous_record_digest: first.record_digest,
            policy_context: LedgerPolicyContext::default(),
        });
        assert!(verify_chain(&[second, first], &signer.verifying_key()).is_err());
    }
}
