# Threat Model

## Scope

This threat model covers the Flock Signal Safety v0.1 software boundary: camera-event ingestion, numeric sensor observations, anonymous signal templates, policy decisions, evidence ledger, PostgreSQL metadata, C/C++ FFI, and deployment tooling.

It does not treat a future identity resolver as part of the production trust boundary.

## Security objectives

1. Preserve evidence integrity and provenance.
2. Prevent unauthorized event ingestion, export, research-mode access, and administrative actions.
3. Prevent accidental or deliberate conversion of anonymous templates into an undocumented civilian identity database.
4. Limit effects on people unrelated to an authorized investigation.
5. Keep secrets, signing material, and raw measurements out of source control and ordinary logs.
6. Make operator access and evidence export auditable.

## Assets

- event and sensor provenance;
- anonymous signal templates;
- evidence-ledger records and signatures;
- signing/private keys;
- API and service credentials;
- policy configuration;
- audit records;
- retention configuration;
- deployment images and release artifacts.

## Primary adversaries and misuse cases

### Unauthorized external client

Attempts to submit fabricated events, enumerate records, or extract evidence.

Controls: deny-by-default gateway authentication, TLS/mTLS requirement for production edge termination, network segmentation, rate limiting at the production ingress, least-privilege service identities, and audit logging.

### Compromised operator credential

Attempts to search/export data outside authorized purpose.

Controls: production IAM/RBAC, short-lived credentials, purpose-bound authorization, export auditing, investigation/case references, alerting on unusual access, and revocation.

### Malicious or compromised sensor

Attempts to inject fabricated or replayed observations.

Controls: source identity, clock/provenance validation, signed sensor envelopes where available, replay detection, source digesting, calibration records, and quality thresholds.

### Evidence tampering

Attempts to modify, remove, reorder, or replace forensic records.

Controls: SHA-256 payload and record digests, previous-record hash chaining, Ed25519 signatures in the development/reference implementation, immutable/offline verification, and independent evidence copies.

### Covert identity-database expansion

Attempts to add names/person IDs to involuntary templates or silently enable identity resolution.

Controls: no identity field/foreign key in the production `SignalTemplate` or production database schema, separate research namespace, policy-engine deny-by-default identity action, code review, schema review, and audit requirements.

### Bias or unsupported inference

Attempts to convert weak signals, mask presence, appearance, or low-quality measurements into an accusation.

Controls: no guilt-classification action in the policy engine, explicit uncertainty, human review for elevated safety events, scientific-claims boundary, validation protocols, and prohibition on treating mask wearing alone as criminal evidence.

### Supply-chain compromise

Attempts to introduce malicious dependencies or modify release artifacts.

Controls targeted for release: locked dependencies, dependency review/scanning, SBOM, reproducible CI, signed release artifacts, protected branches, code review, vulnerability reporting, and provenance attestations.

## Trust boundaries

1. **External event/sensor sources -> gateway/adapter**: untrusted until authenticated and validated.
2. **Gateway -> internal processing**: authenticated service boundary in production.
3. **Processing -> database/ledger**: integrity- and authorization-sensitive boundary.
4. **Operator -> export/search APIs**: privileged boundary requiring strong IAM and auditing.
5. **Production -> research namespace**: hard boundary; identity-resolution research is not implicitly trusted by production.

## Abuse-resistant defaults

- research mode disabled;
- no raw physiological samples persisted by default;
- no person identity schema for signal templates;
- no automatic guilt decision;
- no mask-only alert rule;
- no default API token;
- gateway refuses startup with an empty/missing token;
- evidence exports require an authorized operator role at the policy layer;
- secrets must be supplied externally.

## Production gaps requiring closure before high-assurance deployment

- replace development bearer-token ingress with agency-approved IAM and TLS/mTLS architecture;
- place signing keys in an approved KMS/HSM and select cryptography appropriate to the agency's compliance boundary;
- implement database-level append-only/audit protections and backup/restore controls;
- implement production rate limiting and replay protection;
- complete independent penetration testing and software-composition analysis;
- produce SBOM/provenance artifacts in release CI;
- validate every real Flock/third-party API against authorized vendor documentation and contractual access;
- complete privacy/civil-rights/legal review for the specific deployment and jurisdiction;
- perform scientific validation for every sensor modality used operationally.
