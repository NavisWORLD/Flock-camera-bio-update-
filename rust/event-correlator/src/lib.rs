#![forbid(unsafe_code)]

use flock_adapter::{NormalizedEvent, Occlusion};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SafetySeverity {
    Informational,
    Elevated,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SafetyEvent {
    pub safety_event_id: Uuid,
    pub source_event_id: Uuid,
    pub rule_id: String,
    pub severity: SafetySeverity,
    pub rationale: BTreeMap<String, String>,
    pub human_review_required: bool,
}

pub fn evaluate_event(event: &NormalizedEvent) -> SafetyEvent {
    let restricted_zone = attribute_is_true(event, "restricted_zone");
    let after_hours = attribute_is_true(event, "after_hours");
    let elevated = restricted_zone && after_hours;

    let mut rationale = BTreeMap::new();
    rationale.insert("restricted_zone".into(), restricted_zone.to_string());
    rationale.insert("after_hours".into(), after_hours.to_string());
    rationale.insert(
        "face_occlusion".into(),
        format!("context_only:{}", occlusion_label(&event.face_occlusion)),
    );
    rationale.insert("guilt_determination".into(), "none".into());

    SafetyEvent {
        safety_event_id: Uuid::new_v4(),
        source_event_id: event.event_id,
        rule_id: if elevated {
            "restricted-zone-after-hours".into()
        } else {
            "context-observation".into()
        },
        severity: if elevated {
            SafetySeverity::Elevated
        } else {
            SafetySeverity::Informational
        },
        rationale,
        human_review_required: true,
    }
}

fn attribute_is_true(event: &NormalizedEvent, key: &str) -> bool {
    event
        .attributes
        .get(key)
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn occlusion_label(value: &Occlusion) -> &'static str {
    match value {
        Occlusion::Present => "present",
        Occlusion::Absent => "absent",
        Occlusion::Unknown => "unknown",
    }
}
