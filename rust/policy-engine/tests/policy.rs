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
fn production_engine_denies_identity_resolution() {
    let denied = PolicyEngine::default().evaluate(
        PolicyAction::IdentityResolution,
        &PolicyContext::default(),
    );
    assert!(!denied.allowed);
}

#[test]
fn identity_resolution_requires_explicit_research_engine_configuration() {
    let authorization = ResearchAuthorization::new(
        "controlled-study-board",
        "study-001",
        "blinded repeatability validation",
    )
    .unwrap();
    let research_engine = PolicyEngine::for_authorized_research(authorization);
    let allowed = research_engine.evaluate(
        PolicyAction::IdentityResolution,
        &PolicyContext::default(),
    );
    assert!(allowed.allowed);
    assert!(allowed.reason.contains("study-001"));
}

#[test]
fn incomplete_research_authorization_is_rejected() {
    assert!(ResearchAuthorization::new("", "study-001", "validation").is_err());
    assert!(ResearchAuthorization::new("board", "", "validation").is_err());
    assert!(ResearchAuthorization::new("board", "study-001", "").is_err());
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
