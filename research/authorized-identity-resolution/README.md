# Authorized Identity-Resolution Research Boundary

This directory preserves an extension point for future government, academic, or accredited forensic research. It does **not** ship an operational resolver that maps involuntary physiological observations to civilian identities.

## Scientific status

Human physiological, mechanical, thermal, acoustic, and biomagnetic signals can be measured under appropriate conditions. This project does not assume those signals form a stable, remotely observable, person-unique identifier. Any identity claim requires controlled enrollment, blinded evaluation, open-set testing, repeatability across time and sensors, false-match/false-non-match reporting, subgroup analysis, and independent replication.

## Intended interface

A future separately governed module may implement an interface conceptually equivalent to:

```rust
pub trait AuthorizedIdentityResolver {
    fn evaluate(
        &self,
        template: &SignalTemplate,
        authorization: &ResearchAuthorization,
    ) -> Result<ResearchMatchResult, ResolverError>;
}
```

The production application does not provide an implementation. The policy engine denies identity-resolution requests unless explicit research authorization metadata is present, and ordinary production configuration cannot convert mask/occlusion metadata into an identity or guilt determination.

## Required evidence before operational consideration

A candidate resolver must demonstrate repeatability across days/months, devices, operators, movement, clothing, environment, physiology, and sensor drift. Reports must include false-match rate, false-non-match rate, open-set performance, confidence calibration, cohort/subgroup performance, adversarial robustness, and independent replication.

## Non-suspect protection

Production observations remain anonymous. Identity resolution, if ever independently developed and lawfully authorized, must be a separate privileged operation with purpose limitation, audit logging, retention limits, human review, and correction/appeal mechanisms appropriate to the deployment context.
