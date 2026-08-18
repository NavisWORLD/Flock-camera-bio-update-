# Flock Signal Safety Platform

**Rust + C + C++20 multimodal forensic signal infrastructure for authorized public-safety integrations.**

This repository is an engineering prototype for correlating authorized camera events with anonymous sensor-derived signal features, generating human-review safety events from independent contextual predicates, preserving evidence provenance, and exposing the signal engine to Rust, C, and C++20 software.

The architecture deliberately separates **observation**, **signal representation**, **safety correlation**, **evidence integrity**, and any future **identity-resolution research**.

> **Scientific boundary:** human biomagnetic activity is measurable with specialized high-sensitivity sensors, but this project does not claim that ordinary optical cameras measure biomagnetic fields or that a stable remote person-unique magnetic identifier has been scientifically validated. See [`compliance/SCIENTIFIC_CLAIMS.md`](compliance/SCIENTIFIC_CLAIMS.md).

## Current implementation status

The v0.1 implementation is on `feat/flock-signal-safety-v0.1` in draft PR #1.

Implemented:

- anonymous Rust signal-domain types;
- deterministic SHA-256 observation provenance hashing;
- RMS, zero-crossing-rate, and spectral-centroid feature extraction;
- feature quality and uncertainty fields;
- deny-by-default production policy engine;
- separately configured research-policy mode with validated authorization metadata;
- SHA-256 + Ed25519 tamper-evident ledger and offline chain verification;
- C ABI with panic containment, explicit status codes, output-handle clearing, feature-vector copy-out, and source-digest copy-out;
- C++20 RAII wrapper exposing `quality()`, `features()`, and `source_digest()`;
- normalized camera-event adapter with strict `face_occlusion=present|absent|unknown` validation;
- safety-event correlator where mask alone remains informational and independent restricted-zone + after-hours predicates can elevate an event for human review;
- authenticated reference HTTP gateway;
- OpenAPI 3.1 contract;
- anonymous PostgreSQL forensic metadata schema;
- synthetic camera/safety-event fixtures;
- Docker Compose deployment scaffold;
- Linux native runtime + C/C++ SDK installer and hardened systemd unit;
- Windows PowerShell runtime + SDK installer;
- CI definition for Rust, native C++ integration, secret hygiene, and a runner-start diagnostic;
- threat model, privacy template, retention baseline, security controls, scientific-claims boundary, Flock integration boundary, procurement-readiness matrix, deployment acceptance checklist, verification ledger, and vulnerability-reporting policy.

### Verification status

The current C header and expanded C++20 header/smoke source compile successfully in the available local environment with warnings treated as errors:

```text
EXPANDED_CPP_SYNTAX_OK
C11_HEADER_SYNTAX_OK
```

The Linux installer also passes Bash syntax checking.

Full Rust/Cargo, Rust↔C++ link/runtime, Docker, PostgreSQL, and PowerShell execution are **not yet claimed as passed** because the available local container does not include those runtimes and the repository's GitHub Actions jobs are currently failing before step 1 executes—even the no-checkout `runner-smoke` job containing only `echo`.

See [`docs/verification/VERIFICATION_STATUS.md`](docs/verification/VERIFICATION_STATUS.md). PR #1 intentionally remains draft until the clean Rust/native release gate can execute.

## What v0.1 does not claim

- It does not identify an unknown person from an ordinary camera using a magnetic field.
- It does not contain a production database mapping involuntary physiological templates to civilian names.
- It does not treat mask wearing as evidence of criminality.
- It does not automatically adjudicate guilt.
- It does not ship a covert population identity resolver.
- It does not train/evaluate an ML identity model on Flock Data.
- It is not represented as FIPS validated, FedRAMP authorized, CJIS certified/compliant, government approved, or scientifically validated for physiological identity matching.

## Architecture

```text
Authorized camera/event source          Authorized physical sensor
             |                                    |
             v                                    v
      flock-adapter                     Rust / C / C++ input
             |                                    |
             +----------> normalized context <----+
                              |
                  +-----------+-----------+
                  |                       |
                  v                       v
          event-correlator       anonymous SignalTemplate
                  |                       |
                  v                       v
        human-review SafetyEvent     signed evidence ledger
                                          |
                                          v
                                    offline verifier
```

The safety correlator does not create identities or guilt determinations:

```text
mask/occlusion only                         -> informational
restricted_zone=true + after_hours=true     -> elevated + human review
mask + restricted zone + after hours        -> elevated, but mask remains context only
```

Future regulated research remains a separate extension point:

```text
anonymous SignalTemplate
          |
          v
explicitly research-configured PolicyEngine
          |
          v
AuthorizedIdentityResolver interface
(no production resolver implementation)
          |
          v
blinded evaluation result + uncertainty + audit record
```

See [`research/authorized-identity-resolution/README.md`](research/authorized-identity-resolution/README.md).

## Repository map

```text
rust/
  signal-core/          domain types + canonical observation digest
  signal-features/      deterministic feature extraction
  signal-ledger/        signed tamper-evident evidence chain
  policy-engine/        production/research action policy
  flock-adapter/        normalized authorized camera events
  event-correlator/     contextual human-review safety events
  ffi/                  C ABI backed by Rust
cpp/
  include/              C and C++20 SDK headers
  tests/                native Rust/C++ smoke target
services/gateway/       authenticated reference HTTP ingress
database/migrations/    anonymous PostgreSQL schema
simulator/              synthetic authorized event fixtures
install/docker/         container/Compose stack
install/linux/          native Linux runtime + SDK installer
install/windows/        native Windows runtime + SDK installer
compliance/             claims, vendor, threat/privacy/security/procurement docs
docs/api/               OpenAPI contract
docs/deployment/        production acceptance checklist
docs/verification/      explicit evidence and release-gate status
research/                separately governed experimental interface
```

## Quick start: Docker Compose

Requirements: Docker Engine with Compose support.

```bash
cp .env.example .env
```

Replace the placeholder secrets in `.env`. For a local development token, use a cryptographically secure generator available on your system, for example:

```bash
openssl rand -hex 32
```

Start the stack:

```bash
docker compose --env-file .env -f install/docker/docker-compose.yml up --build
```

Check the service:

```bash
curl http://127.0.0.1:8080/healthz
```

Expected response:

```json
{
  "status": "ok",
  "identity_resolution": "disabled",
  "criminal_classification": "disabled"
}
```

Send the synthetic safety events in another shell:

```bash
export FLOCK_SIGNAL_API_TOKEN='<same token from .env>'
chmod +x simulator/send_sample.sh
./simulator/send_sample.sh
```

The first event should remain informational. The second contains independent `restricted_zone=true` and `after_hours=true` predicates and may be elevated for human review. Its `face_occlusion=present` value remains contextual rather than the reason for elevation.

## Reference HTTP API

Machine-readable contract: [`docs/api/openapi.yaml`](docs/api/openapi.yaml).

Endpoints:

```text
GET  /healthz
POST /v1/camera-events
POST /v1/safety-events
```

The two POST endpoints require the reference bearer token. That token mechanism is for development/evaluation; a production government deployment must use purchaser-approved IAM and TLS/mTLS architecture.

## Native Rust build

Requires Rust 1.80+.

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo build --workspace
```

## C/C++20 SDK build

Requires Rust/Cargo, CMake 3.24+, and a C++20 compiler.

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure
```

CMake builds the Rust `cdylib`, links the C++ wrapper against it, and runs the native smoke target.

Core C ABI:

```c
int32_t fs_engine_create(fs_engine** out_engine);
void fs_engine_destroy(fs_engine* engine);

int32_t fs_engine_extract_template(
    fs_engine* engine,
    const fs_channel_frame* channels,
    size_t channel_count,
    fs_template** out_template
);

float fs_template_quality(const fs_template* value);
size_t fs_template_feature_count(const fs_template* value);

int32_t fs_template_copy_features(
    const fs_template* value,
    float* out_features,
    size_t capacity
);

int32_t fs_template_copy_source_digest(
    const fs_template* value,
    uint8_t* out_digest,
    size_t capacity
);

void fs_template_destroy(fs_template* value);
```

C++ usage:

```cpp
flock_signal::Engine engine;
auto result = engine.extract(channels);

auto quality = result.quality();
auto features = result.features();
auto source_digest = result.source_digest();
```

`source_digest` is observation provenance, **not a person identifier**.

## Linux installation

The native installer is designed to build/test the workspace before installing the gateway and SDK:

```bash
chmod +x install/linux/install.sh
./install/linux/install.sh
```

Default install layout:

```text
/opt/flock-signal/bin/flock-signal-gateway
/opt/flock-signal/lib/libflock_signal_ffi.so
/opt/flock-signal/include/flock_signal/flock_signal.h
/opt/flock-signal/include/flock_signal/flock_signal.hpp
/opt/flock-signal/etc/flock-signal.env
```

It provisions a restricted `flocksignal` service account and a hardened systemd unit. Replace `REPLACE_ME` in the environment file before starting the service; the gateway also refuses known placeholder token values.

## Windows installation

From PowerShell with Rust, CMake, and a C++ toolchain installed:

```powershell
.\install\windows\install.ps1
```

Then configure a strong development/evaluation token and run:

```powershell
$env:FLOCK_SIGNAL_API_TOKEN = '<strong-secret>'
& "$env:LOCALAPPDATA\FlockSignal\start.ps1"
```

## Flock integration boundary

`rust/flock-adapter` intentionally defines a normalized event contract instead of guessing undocumented/private vendor endpoints.

A live Flock integration must use current customer-authorized API access and documented schemas. It must follow the controlling Flock terms, including restrictions on reverse engineering, bulk/database-like extraction, unauthorized sharing/resale, rate-limit circumvention, and prohibited ML use of Flock Implementation/Data.

See [`compliance/FLOCK_INTEGRATION_BOUNDARY.md`](compliance/FLOCK_INTEGRATION_BOUNDARY.md).

## Physical signal sources

Numeric signal arrays can enter through Rust directly or through the C/C++ ABI. Separately authorized adapters may represent modalities such as mechanical, cardiac, muscular, acoustic, thermal, or biomagnetic measurements.

A conventional optical camera is **not** a magnetometer. A biomagnetic deployment requires dedicated physical sensing hardware, calibration, environmental-interference characterization, operating-distance validation, and scientific validation for every claimed capability.

## Evidence ledger

`signal-ledger` binds evidence content to:

- record/event IDs;
- observation time;
- sensor/template references;
- payload digest;
- previous-record digest;
- signer key ID;
- policy context.

The reference implementation computes a deterministic SHA-256 record digest and signs it with Ed25519. `LedgerVerifier` checks record integrity and chain ordering offline.

For a government environment requiring a specific validated cryptographic boundary, route signing/key operations through the purchaser-approved module/KMS/HSM. The reference implementation is not itself a FIPS-validation claim.

## Non-suspect protection

Production defaults are deliberately minimization-oriented:

- `SignalTemplate` has no person identity field;
- the PostgreSQL `signal_templates` table has no civilian/person foreign key;
- mask/occlusion is contextual metadata only;
- mask alone does not elevate a safety event;
- the policy engine always refuses `CriminalClassification`;
- the default production `PolicyEngine` refuses identity resolution;
- a research-policy engine must be explicitly constructed from validated authorization metadata;
- there is no shipping identity resolver;
- raw physiological sample persistence is not required by the production schema;
- safety events always require human review;
- evidence, exports, retention, and privileged access are designed to be auditable.

## Government/security evaluation

Start here:

- [`compliance/PROCUREMENT_READINESS.md`](compliance/PROCUREMENT_READINESS.md)
- [`docs/deployment/DEPLOYMENT_ACCEPTANCE_CHECKLIST.md`](docs/deployment/DEPLOYMENT_ACCEPTANCE_CHECKLIST.md)
- [`docs/verification/VERIFICATION_STATUS.md`](docs/verification/VERIFICATION_STATUS.md)
- [`compliance/SECURITY_CONTROLS.md`](compliance/SECURITY_CONTROLS.md)
- [`compliance/THREAT_MODEL.md`](compliance/THREAT_MODEL.md)
- [`compliance/PRIVACY_IMPACT_TEMPLATE.md`](compliance/PRIVACY_IMPACT_TEMPLATE.md)
- [`compliance/DATA_RETENTION_POLICY.md`](compliance/DATA_RETENTION_POLICY.md)
- [`compliance/SCIENTIFIC_CLAIMS.md`](compliance/SCIENTIFIC_CLAIMS.md)
- [`compliance/FLOCK_INTEGRATION_BOUNDARY.md`](compliance/FLOCK_INTEGRATION_BOUNDARY.md)
- [`SECURITY.md`](SECURITY.md)

The strongest accurate current positioning is:

> **An auditable multimodal forensic signal and public-safety integration platform with Rust/C/C++ edge compatibility, contextual human-review safety events, tamper-evident evidence primitives, privacy-aware anonymous signal representations, and a separately governed experimental biomagnetic/physiological research interface.**

Before a high-assurance government deployment, the acceptance/procurement documents require closure of the remaining production controls, including clean Rust/native runtime verification, production IAM/mTLS, approved cryptographic boundary/KMS/HSM, dependency lock/SBOM/provenance, independent security testing, automatic retention enforcement, authorized vendor sandbox validation, and scientific validation for every physical sensor claim.

## Project documentation

- Architecture: `docs/superpowers/specs/2026-08-18-flock-signal-safety-platform-design.md`
- Implementation plan: `docs/superpowers/plans/2026-08-18-flock-signal-safety-v0.1.md`
- API contract: `docs/api/openapi.yaml`
- Verification ledger: `docs/verification/VERIFICATION_STATUS.md`
- Deployment acceptance: `docs/deployment/DEPLOYMENT_ACCEPTANCE_CHECKLIST.md`

## Development rule

**Be aggressive about detecting threats and preserving evidence; be conservative about identifying people.**

A safety event is a prompt for authorized human review, not a mathematical declaration of guilt.
