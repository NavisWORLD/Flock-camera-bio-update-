# Security Controls Baseline

## Identity and access

- unique operator identities;
- least-privilege roles;
- separate service accounts;
- explicit evidence-export permission;
- deny-by-default research authorization;
- periodic access review;
- immediate credential revocation on offboarding.

## Secrets and cryptography

- API credentials and signing material remain outside source control;
- service-to-service TLS/mTLS is required for distributed production deployments;
- evidence payloads are hashed with SHA-256;
- evidence records are signed with Ed25519;
- key IDs are stored with signatures;
- key rotation preserves historical verification keys;
- plaintext secrets must not appear in ordinary application logs.

## Audit

Audit records should capture request ID, actor/service identity, action, object, authorization reference where applicable, timestamp, and relevant policy decision. Evidence retrieval/export and research-route access are auditable operations.

## Host/container hardening

- non-root runtime user;
- read-only container filesystem where feasible;
- `no-new-privileges` in the supplied Compose profile;
- restricted systemd capabilities;
- production database not exposed publicly by default;
- network access limited to explicitly required providers/services.

## Application controls

- external event payload validation;
- bounded quality/uncertainty values;
- explicit C ABI status codes;
- panic containment at Rust FFI boundaries;
- human review for elevated safety events;
- occlusion alone cannot elevate a safety event;
- no production identity resolver implementation.

## Build and supply chain

The intended release pipeline includes formatting, linting, unit tests, cross-language tests, dependency scanning, SBOM generation, provenance, and artifact signing. A release must distinguish controls that are defined from controls that were actually executed successfully for that release.
