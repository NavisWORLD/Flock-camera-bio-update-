use signal_ledger::{LedgerInput, LedgerSigner, LedgerVerifier};
use uuid::Uuid;

fn input(previous_record_digest: [u8; 32], marker: u8) -> LedgerInput {
    LedgerInput {
        record_id: Uuid::from_u128(marker as u128 + 1),
        observed_at_ns: 1_000 + marker as i128,
        event_id: Uuid::from_u128(marker as u128 + 100),
        sensor_ids: vec![format!("sensor-{marker}")],
        template_ids: vec![Uuid::from_u128(marker as u128 + 200)],
        payload_digest: [marker; 32],
        previous_record_digest,
        policy_context: "anonymous-correlation".into(),
    }
}

#[test]
fn signing_same_input_is_deterministic() {
    let signer = LedgerSigner::from_seed("test-key", [7_u8; 32]);
    let a = signer.sign(input([0_u8; 32], 1));
    let b = signer.sign(input([0_u8; 32], 1));
    assert_eq!(a.record_digest, b.record_digest);
    assert_eq!(a.signature, b.signature);
}

#[test]
fn verifier_detects_modified_payload() {
    let signer = LedgerSigner::from_seed("test-key", [7_u8; 32]);
    let key = signer.verifying_key();
    let mut record = signer.sign(input([0_u8; 32], 2));
    record.payload_digest[0] ^= 0xff;
    assert!(LedgerVerifier::verify_record(&record, &key).is_err());
}

#[test]
fn chain_verification_detects_reordering() {
    let signer = LedgerSigner::from_seed("test-key", [7_u8; 32]);
    let key = signer.verifying_key();
    let first = signer.sign(input([0_u8; 32], 3));
    let second = signer.sign(input(first.record_digest, 4));

    assert!(LedgerVerifier::verify_chain(&[first.clone(), second.clone()], &key).is_ok());
    assert!(LedgerVerifier::verify_chain(&[second, first], &key).is_err());
}
