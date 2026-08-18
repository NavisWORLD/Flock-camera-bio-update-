use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerPolicyContext {
    pub operator_role: String,
    pub authorization_ref: Option<String>,
    pub purpose: String,
}

#[derive(Debug, Clone)]
pub struct NewLedgerRecord {
    pub observed_at_ns: i128,
    pub event_id: Uuid,
    pub sensor_ids: Vec<String>,
    pub template_ids: Vec<Uuid>,
    pub payload_digest: [u8; 32],
    pub previous_record_digest: [u8; 32],
    pub policy_context: LedgerPolicyContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerRecord {
    pub record_id: Uuid,
    pub observed_at_ns: i128,
    pub event_id: Uuid,
    pub sensor_ids: Vec<String>,
    pub template_ids: Vec<Uuid>,
    pub payload_digest: [u8; 32],
    pub previous_record_digest: [u8; 32],
    pub record_digest: [u8; 32],
    pub signer_key_id: String,
    pub signature: Vec<u8>,
    pub policy_context: LedgerPolicyContext,
}

#[derive(Debug, Serialize)]
struct RecordSigningPayload<'a> {
    record_id: Uuid,
    observed_at_ns: i128,
    event_id: Uuid,
    sensor_ids: &'a [String],
    template_ids: &'a [Uuid],
    payload_digest: [u8; 32],
    previous_record_digest: [u8; 32],
    signer_key_id: &'a str,
    policy_context: &'a LedgerPolicyContext,
}

pub struct LedgerSigner {
    key_id: String,
    signing_key: SigningKey,
}

impl LedgerSigner {
    pub fn generate(key_id: impl Into<String>) -> Self {
        Self {
            key_id: key_id.into(),
            signing_key: SigningKey::generate(&mut OsRng),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn verifying_key_bytes(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }

    pub fn sign_record(&self, input: NewLedgerRecord) -> LedgerRecord {
        let record_id = Uuid::new_v4();
        let payload = RecordSigningPayload {
            record_id,
            observed_at_ns: input.observed_at_ns,
            event_id: input.event_id,
            sensor_ids: &input.sensor_ids,
            template_ids: &input.template_ids,
            payload_digest: input.payload_digest,
            previous_record_digest: input.previous_record_digest,
            signer_key_id: &self.key_id,
            policy_context: &input.policy_context,
        };
        let record_digest = hash_payload(&payload);
        let signature = self.signing_key.sign(&record_digest).to_bytes().to_vec();

        LedgerRecord {
            record_id,
            observed_at_ns: input.observed_at_ns,
            event_id: input.event_id,
            sensor_ids: input.sensor_ids,
            template_ids: input.template_ids,
            payload_digest: input.payload_digest,
            previous_record_digest: input.previous_record_digest,
            record_digest,
            signer_key_id: self.key_id.clone(),
            signature,
            policy_context: input.policy_context,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LedgerError {
    #[error("first record must point to the zero digest")]
    InvalidGenesis,
    #[error("record chain is discontinuous at index {0}")]
    ChainDiscontinuity(usize),
    #[error("record digest mismatch at index {0}")]
    DigestMismatch(usize),
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("invalid signature encoding at index {0}")]
    InvalidSignatureEncoding(usize),
    #[error("signature verification failed at index {0}")]
    SignatureVerification(usize),
}

pub fn verify_chain_with_public_key_bytes(
    records: &[LedgerRecord],
    public_key: &[u8; 32],
) -> Result<(), LedgerError> {
    let key = VerifyingKey::from_bytes(public_key).map_err(|_| LedgerError::InvalidPublicKey)?;
    verify_chain(records, &key)
}

pub fn verify_chain(records: &[LedgerRecord], key: &VerifyingKey) -> Result<(), LedgerError> {
    for (index, record) in records.iter().enumerate() {
        let expected_previous = if index == 0 {
            [0_u8; 32]
        } else {
            records[index - 1].record_digest
        };
        if record.previous_record_digest != expected_previous {
            return Err(if index == 0 {
                LedgerError::InvalidGenesis
            } else {
                LedgerError::ChainDiscontinuity(index)
            });
        }

        let payload = RecordSigningPayload {
            record_id: record.record_id,
            observed_at_ns: record.observed_at_ns,
            event_id: record.event_id,
            sensor_ids: &record.sensor_ids,
            template_ids: &record.template_ids,
            payload_digest: record.payload_digest,
            previous_record_digest: record.previous_record_digest,
            signer_key_id: &record.signer_key_id,
            policy_context: &record.policy_context,
        };
        let digest = hash_payload(&payload);
        if digest != record.record_digest {
            return Err(LedgerError::DigestMismatch(index));
        }
        let signature = Signature::from_slice(&record.signature)
            .map_err(|_| LedgerError::InvalidSignatureEncoding(index))?;
        key.verify(&record.record_digest, &signature)
            .map_err(|_| LedgerError::SignatureVerification(index))?;
    }
    Ok(())
}

fn hash_payload(payload: &RecordSigningPayload<'_>) -> [u8; 32] {
    let bytes = serde_json::to_vec(payload).expect("ledger signing payload serialization is infallible");
    Sha256::digest(bytes).into()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(verify_chain_with_public_key_bytes(
            &[first.clone()],
            &signer.verifying_key_bytes()
        )
        .is_ok());

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

    #[test]
    fn malformed_public_key_is_rejected() {
        let signer = LedgerSigner::generate("test-key");
        let record = signer.sign_record(NewLedgerRecord {
            observed_at_ns: 1,
            event_id: Uuid::new_v4(),
            sensor_ids: vec![],
            template_ids: vec![],
            payload_digest: [4_u8; 32],
            previous_record_digest: [0_u8; 32],
            policy_context: LedgerPolicyContext::default(),
        });
        let invalid = [0xff_u8; 32];
        let result = verify_chain_with_public_key_bytes(&[record], &invalid);
        assert!(result.is_err());
    }
}
