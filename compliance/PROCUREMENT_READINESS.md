# Procurement Readiness Matrix

## Current classification

**Engineering prototype / evaluation release.**

This repository is structured so a public-safety or government technical team can evaluate the architecture, SDK boundaries, evidence model, security controls, and scientific research pathway. It is not currently represented as government-certified, production-authorized, FIPS-validated, FedRAMP-authorized, CJIS-compliant/certified, or scientifically validated for physiological person identification.

## Existing evaluation artifacts

- Rust workspace with anonymous signal-domain types;
- deterministic feature extraction and provenance hashing;
- deny-by-default policy engine;
- signed tamper-evident evidence ledger;
- C ABI and C++20 SDK surface;
- normalized authorized camera-event adapter;
- authenticated reference gateway;
- PostgreSQL anonymous metadata schema;
- Docker Compose and Linux/Windows native installation paths;
- synthetic event fixtures;
- automated CI configuration;
- threat model;
- privacy-impact template;
- data-retention baseline;
- scientific claims/validation boundary;
- regulated identity-research extension documentation.

## Federal software-acquisition references

Engineering work should be mapped to the purchaser's requirements and current federal guidance, including as applicable:

- NIST SP 800-218 SSDF Version 1.1 (final): https://csrc.nist.gov/pubs/sp/800/218/final
- NIST SP 800-53 Rev. 5 and current Release 5.2.0 materials: https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- CISA/OMB Secure Software Development Attestation Form: https://www.cisa.gov/resources-tools/resources/secure-software-development-attestation-form

The NIST SSDF Version 1.2 document published as SP 800-218 Rev. 1 is an Initial Public Draft at the time of this repository update, so procurement material should distinguish it from the final Version 1.1 baseline.

## Required closure items before a high-assurance government production proposal

### Build and supply chain

- produce and commit a reviewed dependency lockfile for application releases;
- run SCA/dependency vulnerability scanning;
- generate CycloneDX/SPDX SBOM artifacts;
- produce signed build provenance and release artifacts;
- document patch SLAs and vulnerability disclosure handling;
- implement protected-branch/reviewer requirements.

### Cryptography

- define the purchaser's cryptographic/FIPS boundary;
- replace/reference-route crypto through validated modules where required;
- place signing and service keys in approved KMS/HSM infrastructure;
- document key rotation, revocation, backup, and verifier trust chains.

### Identity and network security

- replace the reference bearer-token ingress with purchaser-approved IAM;
- deploy TLS/mTLS using approved certificates and cipher policy;
- implement network segmentation, rate limits, replay protection, and service identities;
- perform independent penetration testing.

### Operations

- implement automatic retention/deletion worker or database lifecycle equivalent;
- implement protected append-only/immutable audit storage;
- implement backup/restore and disaster-recovery testing;
- implement monitoring, alerting, incident response, and configuration baselines.

### Vendor/Flock integration

- obtain authorized API access and applicable contractual permissions;
- validate event schemas against current vendor documentation;
- complete integration tests using authorized sandbox/test data;
- document rate limits, error behavior, data-use restrictions, and vendor security boundary.

### Scientific validation

- identify the actual physical sensor hardware for each physiological modality;
- perform calibration/noise characterization;
- validate operational distance and environmental constraints;
- establish repeatability/accuracy for every claimed feature;
- keep person-identification claims experimental unless blinded independent studies establish operational error rates.

### Privacy, civil rights, and legal review

- complete the purchaser's Privacy Impact Assessment where required;
- document collection authority, purpose limitation, retention, access, sharing, and oversight;
- complete civil-rights/civil-liberties and counsel review appropriate to the mission;
- prohibit mask wearing alone from being treated as criminal evidence;
- define false-positive correction, audit, and human-review processes.

## Demonstration package target

A procurement demonstration should show:

1. clean install;
2. authenticated health check;
3. ingestion of synthetic/authorized camera events;
4. conversion of numeric sensor data into anonymous signal features;
5. C++ calling the Rust feature engine;
6. creation and offline verification of a signed evidence chain;
7. rejection of tampered evidence;
8. policy denial of unauthorized identity resolution;
9. database schema inspection showing no production person-template mapping;
10. export of test/CI/SBOM/security evidence for reviewer inspection.

## Commercial positioning

The strongest present product claim is an **auditable multimodal forensic signal and public-safety integration platform with an experimental biomagnetic/physiological research interface**. Procurement material should sell demonstrated engineering capabilities and the research pathway separately rather than depending on an unvalidated biological-identity claim.
