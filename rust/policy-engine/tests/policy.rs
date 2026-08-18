use policy_engine::{PolicyAction, PolicyContext, PolicyEngine, ResearchAuthorization};

#[test]
fn anonymous_correlation_is_allowed_by_default() {
    let decision = PolicyEngine::default().evaluate(
        PolicyAction::AnonymousCorrelation,
        &PolicyContext::default(),
    );
    assert!(decision.allowed);
}

#[test]
fn identity_resolution_requires_explicit_research_authorization() {
    let denied = PolicyEngine::default().evaluate(
        PolicyAction::IdentityResolution,
        &PolicyContext::default(),
    );
    assert!(!denied.allowed);

    let context = PolicyContext {
        research_authorization: Some(ResearchAuthorization {
            authority: "controlled-study-board".into(),
            case_reference: "study-001".into(),
            purpose: "blinded repeatability validation".into(),
        }),
        ..PolicyContext::default()
    };
    let allowed = PolicyEngine::default().evaluate(PolicyAction::IdentityResolution, &context);
    assert!(allowed.allowed);
}

#[test]
fn mask_presence_alone_never_authorizes_criminal_classification() {
    let context = PolicyContext {
        face_occlusion_present: true,
        independent_safety_predicates: 0,
        ..PolicyContext::default()
    };
    let decision = PolicyEngine::default().evaluate(PolicyAction::CriminalClassification, &context);
    assert!(!decision.allowed);
    assert!(decision.reason.contains("does not adjudicate guilt"));
}
