# Flock Signal Safety Platform

> Independent public-safety integration research by NavisWORLD. This repository is **not an official Flock Safety product and does not claim Flock Safety or U.S. government endorsement, certification, or authorization**.

A Rust-first, C++20-compatible platform for **anonymous multimodal signal analysis, camera-event correlation, policy-controlled safety events, and cryptographically verifiable forensic evidence**.

The project is designed around a simple principle:

**Observe and preserve evidence without silently turning every person into a named biometric record.**

A separate research interface preserves a future pathway for government, forensic, academic, or accredited laboratories to evaluate person-specific physiological signal science if that capability is ever independently validated and lawfully authorized.

---

## What is implemented on the feature branch

| Component | Status | Purpose |
|---|---|---|
| Anonymous Rust domain model | Source implemented | Sensor frames, observation windows, signal templates, camera events, safety events |
| Deterministic DSP engine | Source + tests written | DC removal, Hann window, FFT, RMS, zero crossing, spectral centroid, band energy, provenance digest |
| Evidence ledger | Source + tests written | SHA-256 chaining and Ed25519 signatures |
| Policy engine | Source + tests written | Deny-by-default research path, export controls, contextual safety review |
| Camera/Flock adapter boundary | Source + fixtures written | Normalize authorized provider event payloads into internal `CameraEvent` objects |
| Event correlator | Source + tests written | Correlate anonymous observations by time and zone |
| C ABI | Source + tests written | Opaque native handles and panic-contained Rust FFI |
| C++20 SDK | Source implemented | RAII wrapper for native integrations |
| PostgreSQL schema | Implemented | Anonymous templates, events, ledger, audit, retention, policy decisions |
| HTTP gateway | Source implemented | Health, normalization, signal extraction, policy, and correlation routes |
| Docker deployment | Implemented | Gateway + PostgreSQL development/deployment profile |
| Linux systemd unit | Implemented | Hardened native gateway service |
| Windows installer script | Implemented | Rust/C++ build and installation workflow |
| Research identity pathway | Interface/docs only | Future regulated research extension; no production identity resolver |
| Compliance package | Implemented | Threat model, privacy template, retention, security controls, claims register, procurement matrix |
| CI | Defined, runner blocked | GitHub Actions currently fails before the first runner step on this private repository |

### Verification status

During the initial build session, GitHub Actions failed before any workflow step executed. A second diagnostic job containing only `echo "runner-started"` also failed before its first step. This isolates the current CI blocker to GitHub Actions runner/repository availability rather than a demonstrated source-code test failure.

The C++ SDK source was independently syntax-compiled with:

```bash
g++ -std=c++20 -Wall -Wextra -Werror
```

The Rust workspace must **not** be described as build-verified until `cargo fmt`, `cargo clippy`, and `cargo test` execute successfully on a real Rust runner.

---

## Architecture

```text
Authorized Camera / Sensor Sources
              |
              v
        Flock Adapter Boundary
              |
              v
         CameraEvent
              |
              +---------------------------+
              |                           |
              v                           v
       ObservationWindow          Context / Policy
              |                           |
              v                           |
       FeatureExtractor                   |
              |                           |
              v                           |
     Anonymous SignalTemplate             |
              |                           |
              +----------+----------------+
                         |
                         v
                    Correlator
                         |
                         v
                    SafetyEvent
                         |
                human review required
                         |
                         v
             Signed Forensic Ledger
```

There is no production `person -> involuntary physiological template` lookup in this path.

---

## Core Rust types

### `SensorFrame`

Represents samples from an authorized source with a sensor ID, source type, nanosecond timestamp, sample rate, channel data, and metadata.

### `ObservationWindow`

Groups synchronized sensor frames into a bounded time window.

### `SignalTemplate`

An anonymous derived representation containing:

- random template UUID;
- feature schema/version;
- numerical features;
- quality score;
- uncertainty score;
- SHA-256 source digest.

It intentionally contains no person name, government ID, face embedding, or civilian identity key.

### `CameraEvent`

Normalized camera/provider event metadata including camera ID, timestamp, zone, event kind, attributes, and optional provider source reference.

### `SafetyEvent`

A correlation result carrying reasons, severity, anonymous template references, and a mandatory human-review flag.

---

## Signal feature engine

`rust/signal-features` currently derives a deterministic basic schema containing:

1. RMS energy;
2. zero-crossing rate;
3. spectral centroid in Hz;
4. low-band energy ratio;
5. mid-band energy ratio;
6. high-band energy ratio.

The implementation performs DC removal and Hann windowing before FFT analysis. Tests use synthetic sine waves rather than real-person biometric datasets.

This is **signal processing**, not proof that people possess remotely usable unique magnetic IDs.

See `compliance/SCIENTIFIC_CLAIMS.md`.

---

## Mask / facial occlusion behavior

Occlusion is represented as contextual metadata:

```text
face_occlusion = present | absent | unknown
```

Occlusion by itself does **not** create an elevated event.

The supplied policy tests require independent context such as:

```text
restricted_zone == true
AND after_hours == true
AND face_occlusion == present
```

Even then, the result is a human-review safety event rather than a determination of identity, guilt, or criminal intent.

---

## Evidence ledger

`rust/signal-ledger` creates append-only evidence records containing event references, sensor/template references, payload digests, previous-record digests, signing-key IDs, policy context, and Ed25519 signatures.

The verifier is designed to reject:

- modified payloads;
- changed record digests;
- invalid signatures;
- missing/reordered chain links;
- a nonzero genesis predecessor.

Signing keys belong in an external secret/key-management system, not source control.

---

## HTTP gateway

Default bind:

```text
0.0.0.0:8080
```

Implemented routes:

```text
GET  /healthz
GET  /readyz
POST /v1/camera/normalize
POST /v1/signal/extract
POST /v1/policy/evaluate
POST /v1/correlate
```

Every response is designed to receive an `x-request-id` generated by gateway middleware.

### Example policy request

```json
{
  "action": "restricted_zone_review",
  "context": {
    "operator_role": "school-safety-reviewer",
    "export_authorized": false,
    "research_authorized": false,
    "restricted_zone": true,
    "after_hours": true,
    "face_occlusion": true,
    "authorization_ref": null
  }
}
```

The response is a policy decision, not a person identity.

---

## Flock integration boundary

`rust/flock-adapter` is intentionally based on an explicit adapter contract and synthetic fixtures.

The repository does **not** contain undocumented/private Flock endpoints, scraped credentials, or production API secrets. A real deployment must use API access that the integrator is authorized to use under the applicable provider agreement.

Offline fixture:

```text
simulator/sample_camera_event.json
```

---

## Rust build

Prerequisites:

- Rust toolchain compatible with the workspace `rust-version`;
- Cargo;
- native compiler for the C++ SDK when cross-language tests are desired.

Commands:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
```

---

## C++20 build

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
ctest --test-dir build --output-on-failure
```

The CMake project builds the real Rust `signal-ffi` library before linking the C++ SDK smoke test.

C interface:

```text
cpp/include/flock_signal/flock_signal.h
```

C++ RAII interface:

```text
cpp/include/flock_signal/flock_signal.hpp
```

---

## Docker install

From the repository root:

```bash
docker compose -f install/docker/docker-compose.yml up --build
```

Then inspect:

```text
http://localhost:8080/healthz
```

The supplied Compose profile runs PostgreSQL and the gateway. The default database password is a **development fallback only**; production deployments must provide their own secret.

---

## Linux deployment

Build the gateway in release mode, install it at:

```text
/opt/flock-signal/bin/flock-signal-gateway
```

and use:

```text
install/linux/flock-signal-gateway.service
```

The service definition drops capabilities, enables `NoNewPrivileges`, protects system/home paths, and runs as a dedicated `flocksignal` account.

---

## Windows deployment

From an appropriate PowerShell environment:

```powershell
.\install\windows\install.ps1
```

Optional elevated service registration:

```powershell
.\install\windows\install.ps1 -RegisterService
```

The script builds the Rust gateway, Rust FFI library, C++ SDK, headers, and installation directory. Production environment secrets are intentionally not generated by the repository.

---

## PostgreSQL model

Initial migration:

```text
database/migrations/001_init.sql
```

Tables:

- `camera_events`
- `sensor_observations`
- `signal_templates`
- `safety_events`
- `ledger_records`
- `audit_events`
- `retention_jobs`
- `policy_decisions`

There is intentionally no production table that maps involuntary signal templates to named civilians.

---

## Future regulated identity-resolution research

The future pathway is documented under:

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

This extension is **not implemented as an operational identity engine**.

The reason for preserving the interface is architectural: if future controlled science demonstrates a reliable person-specific signal modality, an authorized government/accredited research program could evaluate it without redesigning the evidence, policy, or interoperability layers.

Required research gates include blinded repeatability, cross-device testing, open-set testing, false-match/false-non-match measurement, confidence calibration, diverse cohorts, environmental variation, physiological variation, adversarial testing, and independent replication.

---

## Government / public-safety evaluation package

Start with:

- `docs/superpowers/specs/2026-08-18-flock-signal-safety-platform-design.md`
- `docs/superpowers/plans/2026-08-18-flock-signal-safety-platform.md`
- `compliance/SCIENTIFIC_CLAIMS.md`
- `compliance/THREAT_MODEL.md`
- `compliance/PRIVACY_IMPACT_TEMPLATE.md`
- `compliance/DATA_RETENTION_POLICY.md`
- `compliance/SECURITY_CONTROLS.md`
- `compliance/PROCUREMENT_READINESS.md`

The procurement document distinguishes repository deliverables from external actions still required before anyone should claim production readiness, government authorization, or provider certification.

---

## License

This repository uses a proprietary all-rights-reserved notice. Review `LICENSE` before evaluation, reuse, commercialization, integration, or redistribution.

Third-party dependencies remain governed by their respective licenses.
