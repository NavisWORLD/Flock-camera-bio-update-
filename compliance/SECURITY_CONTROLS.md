# Security Controls

## Reference frameworks

This project uses the following as engineering references, not certifications:

- NIST SP 800-53 Rev. 5 security/privacy control catalog and current Release 5.2.0 materials: https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- NIST SP 800-218 SSDF Version 1.1 (final): https://csrc.nist.gov/pubs/sp/800/218/final
- NIST SP 800-218 Rev. 1 / SSDF Version 1.2 is an Initial Public Draft as of this document date and should be tracked, not represented as a final baseline: https://csrc.nist.gov/pubs/sp/800/218/r1/ipd
- CISA/OMB Secure Software Development Attestation Form: https://www.cisa.gov/resources-tools/resources/secure-software-development-attestation-form

## Control objectives implemented or scaffolded

### Access control

- gateway refuses startup without an API token;
- unauthorized event ingestion is denied;
- production policy engine denies identity-resolution operations without explicit research authorization metadata;
- evidence-export decisions require a privileged operator role at the policy layer.

The bearer-token gateway is a development/reference mechanism. A production federal deployment should integrate agency-approved identity, short-lived credentials, MFA where applicable, and service-to-service mTLS.

### Audit and accountability

The database includes audit and policy-decision tables. Production implementations should make audit storage append-only or independently protected, synchronize clocks, capture request/case references, and alert on anomalous access/export behavior.

### System and communications protection

The application does not terminate TLS itself in v0.1. Production deployment must place the service behind an approved TLS/mTLS ingress or service mesh and encrypt database connections. Plain HTTP is acceptable only for isolated local development.

### Cryptographic integrity

The reference ledger uses SHA-256 and Ed25519. That demonstrates tamper-evident chaining and signature verification but is **not** a claim that this implementation or its crypto module is FIPS 140 validated. A deployment requiring FIPS-validated cryptography must substitute/route cryptographic operations through an approved validated module and key-management boundary.

### Key and secret management

- no private keys or live credentials are committed;
- `.env.example` contains placeholders only;
- production secrets should be delivered through agency-approved secret management;
- evidence-signing keys should be non-exportable in an approved KMS/HSM when required;
- key rotation/revocation and verifier key history must be documented.

### Data minimization

The production schema contains no person foreign key for `signal_templates`. Raw physiological samples are not required by the production database schema. Retention is configurable by deployment policy.

### Supply-chain security

Required release controls:

- pinned/locked dependencies;
- automated dependency and vulnerability review;
- SBOM generation (CycloneDX or SPDX as accepted by the purchaser);
- protected branch/review rules;
- CI build/test evidence;
- release artifact signing/provenance;
- documented vulnerability disclosure and patch process.

## NIST family mapping (engineering aid)

The architecture is intended to support control implementation in families including AC (Access Control), AU (Audit and Accountability), IA (Identification and Authentication), SC (System and Communications Protection), SI (System and Information Integrity), SA (System and Services Acquisition), SR (Supply Chain Risk Management), RA (Risk Assessment), and PT (PII Processing and Transparency).

Actual control selection, tailoring, implementation statements, assessment evidence, and authorization remain the responsibility of the deploying organization and its authorizing officials.
