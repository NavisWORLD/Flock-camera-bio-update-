#![forbid(unsafe_code)]

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerInput {
    pub record_id: Uuid,
    pub observed_at_ns: i128,
    pub event_id: Uuid,
    pub sensor_ids: Vec<String>,
    pub template_ids: Vec<Uuid>,
    pub payload_digest: [u8; 32],
    pub previous_record_digest: [u8; 32],
    pub policy_context: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    pub policy_context: String,
}

pub struct LedgerSigner {
    key_id: String,
    signing_key: SigningKey,
}

impl LedgerSigner {
    pub fn from_seed(key_id: impl Into<String>, seed: [u8; 32]) -> Self {
        Self {
            key_id: key_id.into(),
            signing_key: SigningKey::from_bytes(&seed),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn sign(&self, input: LedgerInput) -> LedgerRecord {
        let record_digest = compute_digest(&input, &self.key_id);
        let signature = self.signing_key.sign(&record_digest).to_bytes().to_vec();
        LedgerRecord {
            record_id: input.record_id,
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
    #[error("record digest does not match record content")]
    DigestMismatch,
    #[error("record signature is invalid")]
    InvalidSignature,
    #[error("ledger chain mismatch at record index {0}")]
    ChainMismatch(usize),
}

pub struct LedgerVerifier;

impl LedgerVerifier {
    pub fn verify_record(
        record: &LedgerRecord,
        verifying_key: &VerifyingKey,
    ) -> Result<(), LedgerError> {
        let input = LedgerInput {
            record_id: record.record_id,
            observed_at_ns: record.observed_at_ns,
            event_id: record.event_id,
            sensor_ids: record.sensor_ids.clone(),
            template_ids: record.template_ids.clone(),
            payload_digest: record.payload_digest,
            previous_record_digest: record.previous_record_digest,
            policy_context: record.policy_context.clone(),
        };
        let expected = compute_digest(&input, &record.signer_key_id);
        if expected != record.record_digest {
            return Err(LedgerError::DigestMismatch);
        }

        let signature = Signature::try_from(record.signature.as_slice())
            .map_err(|_| LedgerError::InvalidSignature)?;
        verifying_key
            .verify(&record.record_digest, &signature)
            .map_err(|_| LedgerError::InvalidSignature)
    }

    pub fn verify_chain(
        records: &[LedgerRecord],
        verifying_key: &VerifyingKey,
    ) -> Result<(), LedgerError> {
        let mut previous = [0_u8; 32];
        for (index, record) in records.iter().enumerate() {
            if record.previous_record_digest != previous {
                return Err(LedgerError::ChainMismatch(index));
            }
            Self::verify_record(record, verifying_key)?;
            previous = record.record_digest;
        }
        Ok(())
    }
}

fn compute_digest(input: &LedgerInput, signer_key_id: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input.record_id.as_bytes());
    hasher.update(input.observed_at_ns.to_le_bytes());
    hasher.update(input.event_id.as_bytes());
    hash_strings(&mut hasher, &input.sensor_ids);
    hasher.update((input.template_ids.len() as u64).to_le_bytes());
    for template_id in &input.template_ids {
        hasher.update(template_id.as_bytes());
    }
    hasher.update(input.payload_digest);
    hasher.update(input.previous_record_digest);
    hash_bytes(&mut hasher, input.policy_context.as_bytes());
    hash_bytes(&mut hasher, signer_key_id.as_bytes());
    hasher.finalize().into()
}

fn hash_strings(hasher: &mut Sha256, values: &[String]) {
    hasher.update((values.len() as u64).to_le_bytes());
    for value in values {
        hash_bytes(hasher, value.as_bytes());
    }
}

fn hash_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}
