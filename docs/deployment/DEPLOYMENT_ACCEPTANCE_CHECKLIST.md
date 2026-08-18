# Deployment Acceptance Checklist

Use this checklist before representing a deployment as operational. Development/demo success is not the same as production authorization.

## Build and release

- [ ] Rust workspace builds from a clean checkout.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] CMake/C++20 smoke build and `ctest` pass on every supported target OS.
- [ ] `Cargo.lock` is generated, reviewed, and committed for the application release.
- [ ] Dependency/SCA scan has no unaccepted critical/high findings.
- [ ] SBOM is generated and archived with the release.
- [ ] Release artifacts have integrity hashes and approved signatures/provenance.

## Authentication and network

- [ ] Development bearer-token ingress is replaced or fronted by purchaser-approved IAM.
- [ ] TLS/mTLS is enabled using purchaser-approved certificates and policy.
- [ ] Database connections are encrypted.
- [ ] Service identities and least-privilege network rules are configured.
- [ ] Rate limiting and replay protections are enabled at the production ingress.
- [ ] No placeholder token/password remains.

## Key management and evidence

- [ ] Evidence-signing keys are stored in the approved KMS/HSM or equivalent boundary.
- [ ] Required cryptographic/FIPS boundary is documented.
- [ ] Key rotation, revocation, backup, and verifier trust history are tested.
- [ ] Valid evidence-chain verification succeeds offline.
- [ ] Modified payload, modified signature, missing record, and reordered record tests fail verification.
- [ ] Evidence export creates an audit record and follows case/purpose policy.

## Flock/vendor integration

- [ ] Flock Customer Authorization / integration authorization is documented.
- [ ] Current Flock API/Integration Terms and Developer Hub were re-reviewed.
- [ ] Only authorized documented API functions are used.
- [ ] No scraping, bulk database replication, rate-limit circumvention, or prohibited ML use of Flock Data occurs.
- [ ] Integration credentials are in approved secret management, never source control.
- [ ] Vendor sandbox/test event schemas and error behavior are validated.
- [ ] Vendor rate limits and retention/data-use restrictions are documented.

## Signal/sensor validation

- [ ] Every physical sensor type is identified by make/model/firmware/calibration method.
- [ ] Clock synchronization and calibration are verified.
- [ ] Environmental noise/interference limits are documented.
- [ ] Signal quality and uncertainty thresholds are validated for the intended environment.
- [ ] Ordinary optical camera data is not represented as a biomagnetic measurement.
- [ ] Any person-specific physiological identification claim remains disabled unless separately and independently validated.

## Non-suspect protection

- [ ] Production `SignalTemplate` and database schema contain no routine person identity link.
- [ ] Mask/occlusion alone does not elevate an event.
- [ ] Elevated safety events require independent configured predicates and human review.
- [ ] System does not output a guilt determination.
- [ ] Raw physiological sample persistence is disabled unless separately authorized.
- [ ] Retention/expiration values are approved and automatic cleanup is actually installed/tested.
- [ ] False-positive correction and review procedures are documented.

## Audit, privacy, and oversight

- [ ] Agency Privacy Impact Assessment or equivalent review is complete where required.
- [ ] Collection authority, purpose, locations/zones, access roles, sharing, and retention are documented.
- [ ] Civil-rights/civil-liberties and legal review are complete as applicable.
- [ ] Audit logs are independently protected/append-only as required.
- [ ] Monitoring and alerting cover authentication failures, privileged exports, policy denials, and unusual access.
- [ ] Backup/restore and disaster-recovery tests pass.
- [ ] Incident-response contacts and procedures are current.

## Production acceptance record

- Deployment name:
- Environment:
- Release commit/tag:
- System owner:
- Security approver:
- Privacy/civil-rights reviewer:
- Vendor/Flock authorization reference:
- Acceptance date:
- Expiration/review date:
- Exceptions/risk acceptances:
