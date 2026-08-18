#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyAction {
    AnonymousCorrelation,
    EvidenceExport,
    IdentityResolution,
    CriminalClassification,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResearchAuthorization {
    pub authority: String,
    pub case_reference: String,
    pub purpose: String,
}

impl ResearchAuthorization {
    pub fn is_complete(&self) -> bool {
        !self.authority.trim().is_empty()
            && !self.case_reference.trim().is_empty()
            && !self.purpose.trim().is_empty()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyContext {
    pub operator_role: Option<String>,
    pub face_occlusion_present: bool,
    pub independent_safety_predicates: u8,
    pub research_authorization: Option<ResearchAuthorization>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(&self, action: PolicyAction, context: &PolicyContext) -> PolicyDecision {
        match action {
            PolicyAction::AnonymousCorrelation => PolicyDecision {
                allowed: true,
                reason: "anonymous event correlation is a production capability".into(),
            },
            PolicyAction::EvidenceExport => {
                let allowed = context
                    .operator_role
                    .as_deref()
                    .map(|role| matches!(role, "investigator" | "auditor" | "administrator"))
                    .unwrap_or(false);
                PolicyDecision {
                    allowed,
                    reason: if allowed {
                        "operator role permits audited evidence export".into()
                    } else {
                        "evidence export requires an authorized operator role".into()
                    },
                }
            }
            PolicyAction::IdentityResolution => {
                let allowed = context
                    .research_authorization
                    .as_ref()
                    .map(ResearchAuthorization::is_complete)
                    .unwrap_or(false);
                PolicyDecision {
                    allowed,
                    reason: if allowed {
                        "explicit regulated research authorization is present".into()
                    } else {
                        "identity resolution is disabled outside an explicitly authorized research context".into()
                    },
                }
            }
            PolicyAction::CriminalClassification => PolicyDecision {
                allowed: false,
                reason: "this system does not adjudicate guilt; mask or occlusion metadata is contextual only".into(),
            },
        }
    }
}
