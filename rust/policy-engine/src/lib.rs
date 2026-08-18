#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_only_does_not_create_elevated_safety_decision() {
        let engine = PolicyEngine::default();
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
        let engine = PolicyEngine::default();
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
        let engine = PolicyEngine::default();
        let decision = engine.evaluate(PolicyAction::ResearchRoute, &PolicyContext::default());
        assert!(!decision.allowed);
    }
}
