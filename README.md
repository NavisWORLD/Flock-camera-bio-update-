# Flock Signal Safety Platform

> Independent public-safety integration research by NavisWORLD. This repository is **not an official Flock Safety product** and does not claim Flock Safety, federal, state, local, or other government endorsement, certification, authorization, or procurement approval.

A Rust-first, C++20-compatible platform for **anonymous multimodal signal analysis, authorized camera-event normalization, time/zone correlation, policy-controlled human-review events, and cryptographically verifiable forensic evidence**.

The shipping architecture intentionally separates observation, representation, evidence, policy, and any future identity-resolution research.

## Current implementation

| Component | Status | Purpose |
|---|---|---|
| Rust domain model | Source + tests | `SensorFrame`, `ObservationWindow`, anonymous `SignalTemplate`, `CameraEvent`, `SafetyEvent` |
| Signal feature engine | Source + tests | DC removal, Hann window, FFT, RMS, zero crossing, spectral centroid, band energy, provenance digest |
| Evidence ledger | Source + tests | SHA-256 chaining, Ed25519 signing, public-key verification |
| Policy engine | Source + tests | Contextual review rules, export controls, deny-by-default research path |
| Flock/provider adapter boundary | Source + fixtures | Normalize authorized provider JSON into internal `CameraEvent` objects |
| Event correlator | Source + tests | Correlate anonymous observations by time and zone |
| C ABI | Source + tests | Opaque native handles, explicit status codes, panic containment |
| C++20 SDK | Source + smoke test | RAII wrapper for native/edge integrations |
| HTTP gateway | Source + tests | Authenticated processing APIs plus public health/readiness |
| PostgreSQL schema | Implemented | Events, anonymous templates, ledger, audit, retention, policy decisions |
| Docker/Linux/Windows packaging | Implemented | Evaluation/deployment assets |
| Security/compliance package | Implemented | Threat model, controls, retention, privacy-impact and procurement templates |
| Future regulated identity pathway | Interface/docs only | Research extension point; no shipping population-identity engine |
| CI | Verified on Linux + Windows | Rust 1.82, strict Clippy, tests, release builds, C++ interoperability, packaged artifacts |

## Safety and scientific boundary

The production path does **not** maintain a `person -> involuntary physiological template` lookup and does not infer guilt from mask wearing or facial occlusion.

`face_occlusion` is only contextual metadata:

```text
present | absent | unknown
```

Occlusion alone remains non-elevated. A supplied example policy requires independent conditions such as restricted-zone presence and after-hours context before creating an elevated **human-review** event.

The repository also does **not** claim that a stable, remotely measurable, person-unique magnetic or physiological identifier has already been scientifically established. See `compliance/SCIENTIFIC_CLAIMS.md`.

## Production data flow

```text
Authorized camera / sensor input
            |
            v
Provider adapter / normalization
            |
            +------> CameraEvent
            |
            v
ObservationWindow
            |
            v
FeatureExtractor
            |
            v
Anonymous SignalTemplate
            |
            +------> time / zone Correlator
                         |
                         v
                     SafetyEvent
                         |
                 human review required
                         |
                         v
                Signed forensic ledger
```

## Gateway security

The gateway is fail-closed for processing routes.

Public routes:

```text
GET /healthz
GET /readyz
```

Protected routes:

```text
POST /v1/camera/normalize
POST /v1/signal/extract
POST /v1/policy/evaluate
POST /v1/correlate
POST /v1/evidence/verify
```

Set a deployment secret:

```text
FLOCK_SIGNAL_API_KEY=<secret-managed-value>
```

Clients send:

```http
Authorization: Bearer <secret-managed-value>
```

If no API key is configured, `/v1/*` requests are denied. Every gateway response receives an `x-request-id` for correlation/audit workflows.

See `docs/api/README.md` for request contracts.

## Anonymous signal representation

`SignalTemplate` contains:

- random UUID;
- feature schema and version;
- numerical features;
- quality score;
- uncertainty score;
- SHA-256 source digest.

It contains no person name, government identifier, face embedding, or production identity-record key.

The current feature schema derives:

1. RMS energy;
2. zero-crossing rate;
3. spectral centroid;
4. low-band energy ratio;
5. mid-band energy ratio;
6. high-band energy ratio.

Tests use synthetic signal fixtures rather than a real-person biometric database.

## Evidence ledger

`rust/signal-ledger` creates ordered records containing event/template references, source/payload digests, prior-record digests, policy context, signer key IDs, and Ed25519 signatures.

Verification rejects modified payloads, changed digests, broken/reordered chain links, malformed signatures, and invalid public keys.

The gateway exposes **verification only**. Production private signing-key custody belongs in the deploying organization's KMS/HSM or equivalent secret-management system.

## Flock/provider integration boundary

`rust/flock-adapter` is intentionally provider-contract based. It does not contain scraped credentials, undocumented/private Flock endpoints, or embedded production API secrets.

A real integrator must separately obtain and configure provider access it is contractually and legally authorized to use.

Synthetic example:

```text
simulator/sample_camera_event.json
```

## Rust verification

The canonical Linux verification command is:

```bash
bash scripts/verify.sh
```

It runs the locked dependency graph through:

```bash
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace --all-targets
cargo build --locked --workspace --release
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
ctest --test-dir build --output-on-failure
```

Windows equivalent:

```powershell
.\scripts\verify.ps1
```

### Current verification status

Cross-platform release verification is complete for the Rust 1.82 / C++20 implementation.

Verified CI run `32201084255` on release-hardening head `93a61a4fb82d716013d89d2c072f6eadfe9b9d41` completed successfully on both operating systems before squash merge into `main`:

- Linux: Rustfmt, Clippy with `-D warnings`, all Rust tests, locked release build, CMake/C++ build, Rust↔C++ CTest smoke test, packaging, artifact upload.
- Windows: Rustfmt, Clippy, all Rust tests, locked optimized Rust release build, CMake configure, MSVC C++ interoperability build, CTest smoke test, packaging, artifact upload.

Produced evaluation artifacts:

- `flock-signal-linux-x86_64` — SHA-256 `c01498f29e9a0e5c2c7f8d2b662e4ac824038ce992905b060b980396871321c6`
- `flock-signal-windows-x86_64` — SHA-256 `8f9a8b02617c08f4c79548ddc2e3631672f72e16f9b65baa11d526d38eee9382`

Build verification does **not** imply Flock Safety certification, government authorization, evidentiary admissibility, or scientific validation of a remote person-unique physiological/magnetic identifier.

## C++20 integration

C API:

```text
cpp/include/flock_signal/flock_signal.h
```

C++ RAII API:

```text
cpp/include/flock_signal/flock_signal.hpp
```

Build:

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
ctest --test-dir build --output-on-failure
```

## Deployment assets

Docker:

```bash
docker compose -f install/docker/docker-compose.yml up --build
```

Linux systemd:

```text
install/linux/flock-signal-gateway.service
install/linux/gateway.env.example
```

Windows:

```powershell
.\install\windows\install.ps1
```

The supplied deployment assets are evaluation/reference packaging. Production deployments still require organization-specific TLS termination, secret management, access policy, database hardening, monitoring, retention configuration, provider credentials, and security authorization.

## PostgreSQL model

Initial schema:

```text
database/migrations/001_init.sql
```

Primary tables:

- `camera_events`
- `sensor_observations`
- `signal_templates`
- `safety_events`
- `ledger_records`
- `audit_events`
- `retention_jobs`
- `policy_decisions`

There is intentionally no production named-civilian physiological-template table.

## Future regulated identity-resolution research

The reserved extension point lives under:

```text
research/authorized-identity-resolution/
```

Conceptually:

```text
Anonymous SignalTemplate
        |
        v
AuthorizedIdentityResolver
        |
        v
ResearchMatchResult + uncertainty + audit
```

This is an interface/evaluation specification, not a turnkey covert identity engine.

Before any future operational consideration, a government or accredited research program would need independent scientific validation, legal/policy authorization, security review, privacy/civil-rights review, auditability, and deployment approval. Validation should include blinded repeatability, cross-device testing, open-set evaluation, false-match/false-non-match rates, confidence calibration, cohort analysis, environmental/physiological variation, adversarial testing, and independent replication.

## Government / evaluator documentation

Start with:

- `docs/superpowers/specs/2026-08-18-flock-signal-safety-platform-design.md`
- `docs/superpowers/plans/2026-08-18-flock-signal-safety-platform.md`
- `docs/api/README.md`
- `compliance/SCIENTIFIC_CLAIMS.md`
- `compliance/THREAT_MODEL.md`
- `compliance/PRIVACY_IMPACT_TEMPLATE.md`
- `compliance/DATA_RETENTION_POLICY.md`
- `compliance/SECURITY_CONTROLS.md`
- `compliance/PROCUREMENT_READINESS.md`
- `SECURITY.md`

These documents distinguish repository deliverables from external actions still required before production deployment, provider certification, evidentiary reliance, or government authorization.

## License

This repository uses a proprietary all-rights-reserved notice. Review `LICENSE` before evaluation, copying, reuse, modification, integration, redistribution, deployment, or commercialization. Third-party dependencies remain governed by their own licenses.
