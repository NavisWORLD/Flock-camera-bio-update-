#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Occlusion {
    Present,
    Absent,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalCameraEvent {
    pub event_id: Uuid,
    pub camera_id: String,
    pub observed_at_ns: i128,
    pub zone_id: Option<String>,
    pub event_kind: String,
    pub attributes: BTreeMap<String, String>,
    pub source_uri: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NormalizedEvent {
    pub event_id: Uuid,
    pub camera_id: String,
    pub observed_at_ns: i128,
    pub zone_id: Option<String>,
    pub event_kind: String,
    pub attributes: BTreeMap<String, String>,
    pub face_occlusion: Occlusion,
    pub source_uri: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AdapterError {
    #[error("camera_id must not be empty")]
    MissingCameraId,
    #[error("event_kind must not be empty")]
    MissingEventKind,
    #[error("face_occlusion must be present, absent, or unknown; got {0}")]
    InvalidOcclusion(String),
}

pub fn normalize_event(event: ExternalCameraEvent) -> Result<NormalizedEvent, AdapterError> {
    if event.camera_id.trim().is_empty() {
        return Err(AdapterError::MissingCameraId);
    }
    if event.event_kind.trim().is_empty() {
        return Err(AdapterError::MissingEventKind);
    }

    let face_occlusion = match event.attributes.get("face_occlusion").map(String::as_str) {
        Some("present") => Occlusion::Present,
        Some("absent") => Occlusion::Absent,
        Some("unknown") | None => Occlusion::Unknown,
        Some(value) => return Err(AdapterError::InvalidOcclusion(value.to_string())),
    };

    Ok(NormalizedEvent {
        event_id: event.event_id,
        camera_id: event.camera_id,
        observed_at_ns: event.observed_at_ns,
        zone_id: event.zone_id,
        event_kind: event.event_kind,
        attributes: event.attributes,
        face_occlusion,
        source_uri: event.source_uri,
    })
}
