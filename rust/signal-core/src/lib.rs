#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Generic,
    Camera,
    Biomagnetic,
    Mechanical,
    Cardiac,
    Muscular,
    Acoustic,
    Thermal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SensorFrame {
    pub sensor_id: String,
    pub source_kind: SourceKind,
    pub timestamp_ns: i128,
    pub sample_rate_hz: f64,
    pub channels: Vec<Vec<f32>>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ObservationWindow {
    pub window_id: Uuid,
    pub start_ns: i128,
    pub end_ns: i128,
    pub frames: Vec<SensorFrame>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignalTemplate {
    pub template_id: Uuid,
    pub feature_schema: String,
    pub feature_version: String,
    pub features: Vec<f32>,
    pub quality: f32,
    pub uncertainty: f32,
    pub source_digest: [u8; 32],
}

/// Produce a stable SHA-256 digest over the observation's canonical binary fields.
/// The digest is provenance metadata, not a person identifier.
pub fn digest_observation(window: &ObservationWindow) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(window.window_id.as_bytes());
    hasher.update(window.start_ns.to_le_bytes());
    hasher.update(window.end_ns.to_le_bytes());
    hasher.update((window.frames.len() as u64).to_le_bytes());

    for frame in &window.frames {
        hash_bytes(&mut hasher, frame.sensor_id.as_bytes());
        hasher.update([source_kind_tag(&frame.source_kind)]);
        hasher.update(frame.timestamp_ns.to_le_bytes());
        hasher.update(frame.sample_rate_hz.to_bits().to_le_bytes());
        hasher.update((frame.channels.len() as u64).to_le_bytes());

        for channel in &frame.channels {
            hasher.update((channel.len() as u64).to_le_bytes());
            for sample in channel {
                hasher.update(sample.to_bits().to_le_bytes());
            }
        }

        hasher.update((frame.metadata.len() as u64).to_le_bytes());
        for (key, value) in &frame.metadata {
            hash_bytes(&mut hasher, key.as_bytes());
            hash_bytes(&mut hasher, value.as_bytes());
        }
    }

    hasher.finalize().into()
}

fn hash_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn source_kind_tag(kind: &SourceKind) -> u8 {
    match kind {
        SourceKind::Generic => 0,
        SourceKind::Camera => 1,
        SourceKind::Biomagnetic => 2,
        SourceKind::Mechanical => 3,
        SourceKind::Cardiac => 4,
        SourceKind::Muscular => 5,
        SourceKind::Acoustic => 6,
        SourceKind::Thermal => 7,
    }
}
