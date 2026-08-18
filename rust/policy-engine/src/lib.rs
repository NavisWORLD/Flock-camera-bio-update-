#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyAction {
    AnonymousCorrelation,
    EvidenceExport,
    IdentityResolution,
    CriminalClassification,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResearchAuthorization {
    authority: String,
    case_reference: String,
    purpose: String,
}

impl ResearchAuthorization {
    pub fn new(
        authority: impl Into<String>,
        case_reference: impl Into<String>,
        purpose: impl Into<String>,
    ) -> Result<Self, PolicyError> {
        let value = Self {
            authority: authority.into(),
            case_reference: case_reference.into(),
            purpose: purpose.into(),
        };
        if value.authority.trim().is_empty()
            || value.case_reference.trim().is_empty()
            || value.purpose.trim().is_empty()
        {
            return Err(PolicyError::IncompleteResearchAuthorization);
        }
        Ok(value)
    }

    pub fn authority(&self) -> &str {
        &self.authority
    }

    pub fn case_reference(&self) -> &str {
        &self.case_reference
    }

    pub fn purpose(&self) -> &str {
        &self.purpose
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyContext {
    pub operator_role: Option<String>,
    pub face_occlusion_present: bool,
    pub independent_safety_predicates: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PolicyError {
    #[error("research authorization requires authority, case reference, and purpose")]
    IncompleteResearchAuthorization,
}

#[derive(Clone, Debug, Default)]
pub struct PolicyEngine {
    research_authorization: Option<ResearchAuthorization>,
}

impl PolicyEngine {
    pub fn for_authorized_research(authorization: ResearchAuthorization) -> Self {
        Self {
            research_authorization: Some(authorization),
        }
    }

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
            PolicyAction::IdentityResolution => match self.research_authorization.as_ref() {
                Some(authorization) => PolicyDecision {
                    allowed: true,
                    reason: format!(
                        "research-only identity-resolution evaluation enabled for authorization {}",
                        authorization.case_reference()
                    ),
                },
                None => PolicyDecision {
                    allowed: false,
                    reason: "identity resolution is disabled in the production policy engine; an explicitly research-configured engine is required".into(),
                },
            },
            PolicyAction::CriminalClassification => PolicyDecision {
                allowed: false,
                reason: "this system does not adjudicate guilt; mask or occlusion metadata is contextual only".into(),
            },
        }
    }
}
