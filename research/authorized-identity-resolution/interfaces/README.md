# Research Resolver Interface Contract

A future accredited research implementation may target an interface equivalent to:

```rust
pub trait AuthorizedIdentityResolver {
    fn evaluate(
        &self,
        template: &SignalTemplate,
        authorization: &ResearchAuthorization,
    ) -> Result<ResearchMatchResult, ResolverError>;
}

pub struct ResearchAuthorization {
    pub organization: String,
    pub authorization_ref: String,
    pub purpose: String,
    pub valid_from_ns: i128,
    pub valid_until_ns: i128,
    pub cohort_scope: String,
    pub operator_role: String,
    pub audit_sink: String,
}

pub struct ResearchMatchResult {
    pub candidate_reference: String,
    pub similarity: f32,
    pub uncertainty: f32,
    pub feature_schema: String,
    pub resolver_version: String,
}
```

This is an **interface specification**, not a production implementation.

Rules for any future implementation:

1. It must live outside the normal production request path.
2. It must reject expired or absent authorization.
3. It must emit an immutable audit event for every evaluation.
4. It must report similarity and uncertainty, never certainty.
5. It must use an explicitly governed enrollment dataset rather than silently enrolling ordinary passersby.
6. It must pass independent scientific and security review before operational use is considered.
