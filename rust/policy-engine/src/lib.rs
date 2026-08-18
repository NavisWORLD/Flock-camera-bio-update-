use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    AnonymousObservation,
    EventCorrelation,
    EvidenceExport,
    RetentionExpiry,
    ResearchRoute,
    RestrictedZoneReview,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyContext {
    pub operator_role: String,
    pub export_authorized: bool,
    pub research_authorized: bool,
    pub restricted_zone: bool,
    pub after_hours: bool,
    pub face_occlusion: Option<bool>,
    pub authorization_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub elevated: bool,
    pub requires_human_review: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Default)]
pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(&self, action: PolicyAction, context: &PolicyContext) -> PolicyDecision {
        match action {
            PolicyAction::AnonymousObservation | PolicyAction::EventCorrelation | PolicyAction::RetentionExpiry => {
                PolicyDecision {
                    allowed: true,
                    elevated: false,
                    requires_human_review: false,
                    reasons: vec!["production-anonymous-path".into()],
                }
            }
            PolicyAction::EvidenceExport => PolicyDecision {
                allowed: context.export_authorized,
                elevated: false,
                requires_human_review: context.export_authorized,
                reasons: vec![if context.export_authorized {
                    "authorized-evidence-export"
                } else {
                    "evidence-export-denied"
                }
                .into()],
            },
            PolicyAction::ResearchRoute => PolicyDecision {
                allowed: context.research_authorized && context.authorization_ref.is_some(),
                elevated: false,
                requires_human_review: true,
                reasons: vec![if context.research_authorized && context.authorization_ref.is_some() {
                    "explicit-research-authorization"
                } else {
                    "research-deny-by-default"
                }
                .into()],
            },
            PolicyAction::RestrictedZoneReview => {
                let contextual_elevation = context.restricted_zone
                    && context.after_hours
                    && context.face_occlusion == Some(true);
                PolicyDecision {
                    allowed: true,
                    elevated: contextual_elevation,
                    requires_human_review: contextual_elevation,
                    reasons: if contextual_elevation {
                        vec![
                            "restricted-zone".into(),
                            "after-hours".into(),
                            "face-occlusion-context".into(),
                        ]
                    } else {
                        vec!["insufficient-context-for-elevation".into()]
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_only_does_not_create_elevated_safety_decision() {
        let engine = PolicyEngine;
        let context = PolicyContext {
            face_occlusion: Some(true),
            ..PolicyContext::default()
        };
        let decision = engine.evaluate(PolicyAction::RestrictedZoneReview, &context);
        assert!(!decision.elevated);
        assert!(decision.allowed);
    }

    #[test]
    fn contextual_restricted_zone_rule_requires_human_review() {
        let engine = PolicyEngine;
        let context = PolicyContext {
            restricted_zone: true,
            after_hours: true,
            face_occlusion: Some(true),
            ..PolicyContext::default()
        };
        let decision = engine.evaluate(PolicyAction::RestrictedZoneReview, &context);
        assert!(decision.elevated);
        assert!(decision.requires_human_review);
    }

    #[test]
    fn research_routes_are_denied_without_explicit_authorization() {
        let engine = PolicyEngine;
        let decision = engine.evaluate(PolicyAction::ResearchRoute, &PolicyContext::default());
        assert!(!decision.allowed);
    }

    #[test]
    fn research_route_requires_both_flag_and_authorization_reference() {
        let engine = PolicyEngine;
        let context = PolicyContext {
            research_authorized: true,
            authorization_ref: Some("IRB-OR-AGENCY-CASE-001".into()),
            ..PolicyContext::default()
        };
        assert!(engine.evaluate(PolicyAction::ResearchRoute, &context).allowed);
    }
}
