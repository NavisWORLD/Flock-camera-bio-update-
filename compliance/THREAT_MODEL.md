# Threat Model

## Security objectives

Protect evidence integrity, prevent unauthorized identity resolution, minimize data collected about non-suspects, prevent silent policy bypass, protect signing credentials, and make operator activity auditable.

## Assets

- sensor and camera event metadata;
- anonymous signal templates;
- evidence ledger records;
- signing keys and API credentials;
- policy decisions and authorization references;
- audit records;
- deployment configuration;
- research datasets when separately authorized.

## Trust boundaries

1. External camera/sensor provider -> adapter.
2. Adapter -> anonymous processing pipeline.
3. Processing pipeline -> persistence and evidence ledger.
4. Operator/API client -> gateway.
5. Production namespace -> research namespace.
6. Application -> signing key/secret manager.
7. Evidence export -> external investigator or court workflow.

## Principal threats and controls

### Unauthorized re-identification

Threat: an operator attempts to convert anonymous production templates into named civilian records.

Controls: production schema contains no identity mapping; research route is deny-by-default; identity resolver is not shipped; authorization references are audited; research namespace is isolated.

### Mask-as-guilt automation

Threat: facial covering is treated as proof of wrongdoing.

Controls: occlusion is context only; policy tests require independent restricted-zone and after-hours predicates before an elevated review event; all elevated outputs require human review.

### Evidence tampering

Threat: payloads, ordering, or provenance are modified after collection.

Controls: SHA-256 record digests, previous-record chaining, Ed25519 signatures, offline verification, export manifests, immutable audit events.

### Credential theft

Threat: provider/API or signing credentials are committed or exposed in logs.

Controls: environment/secret-manager loading, no repository credentials, no plaintext secret logging, least-privilege accounts, key rotation procedures.

### Sensor spoofing and replay

Threat: stale or synthesized signals are injected as current observations.

Controls: sensor IDs, timestamps, provenance digest, calibration metadata, adapter validation, replay-window rules, signed provider metadata where available, anomaly review.

### Correlation abuse

Threat: anonymous events are queried indefinitely to reconstruct movements unrelated to a valid purpose.

Controls: configurable retention, purpose-bound access policy, logged exports/searches, zone/time scoping, retention jobs, administrative review.

### Research-mode escape

Threat: experimental identity code becomes reachable from ordinary production configuration.

Controls: no production resolver implementation, separate namespace, authorization object requirement, deny-by-default policy, independent deployment review.

### Supply-chain compromise

Threat: dependency or build artifact is maliciously modified.

Controls: lockfiles after successful dependency resolution, dependency scanning, SBOM generation, reproducible CI commands, signed release artifacts when release infrastructure is available.

## Residual risks

No software control can establish the scientific validity of an unvalidated sensor modality. Human review can still be biased or incorrect. Compromised endpoint hardware can falsify inputs before application-level hashing. Operational deployment therefore requires sensor assurance, access governance, documented retention, incident response, and independent evaluation in addition to this codebase.
