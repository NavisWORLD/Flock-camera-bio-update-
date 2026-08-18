# Flock Signal Safety Platform

**Rust + C++ multimodal forensic signal infrastructure for authorized public-safety integrations.**

This repository is an engineering prototype for correlating authorized camera events with anonymous sensor-derived signal features, preserving provenance in a tamper-evident evidence ledger, and exposing the processing engine to Rust, C, and C++20 applications.

The production architecture deliberately separates **observation**, **signal representation**, **evidence integrity**, and any future **identity-resolution research**.

> **Scientific boundary:** human biomagnetic activity is measurable with specialized high-sensitivity sensors, but this project does not claim that ordinary cameras can measure biomagnetic fields or that a stable remote person-unique magnetic identifier has been scientifically validated. See [`compliance/SCIENTIFIC_CLAIMS.md`](compliance/SCIENTIFIC_CLAIMS.md).

## What v0.1 provides

- Rust anonymous signal-domain types;
- deterministic observation provenance hashing;
- RMS, zero-crossing, and spectral-centroid feature extraction;
- quality and uncertainty fields;
- deny-by-default policy engine;
- Ed25519-signed SHA-256 evidence ledger and offline chain verification;
- stable C ABI;
- C++20 RAII wrapper and smoke executable;
- normalized camera-event adapter with `face_occlusion=present|absent|unknown` validation;
- authenticated reference HTTP gateway;
- PostgreSQL anonymous forensic metadata schema;
- synthetic camera-event fixtures;
- Docker Compose deployment;
- Linux native installer/systemd unit;
- Windows native PowerShell installer/launcher;
- CI for Rust tests/build/format, C++ smoke build, and basic secret hygiene;
- threat model, privacy template, retention baseline, scientific claims, and procurement-readiness documentation.

## What v0.1 does not claim

- It does not identify an unknown person from an ordinary camera using a magnetic field.
- It does not contain a production database mapping involuntary physiological templates to civilian names.
- It does not treat mask wearing as evidence of criminality.
- It does not automatically adjudicate guilt.
- It does not ship a covert population identity resolver.
- It is not represented as FIPS validated, FedRAMP authorized, CJIS certified/compliant, government approved, or scientifically validated for physiological identity matching.

## Architecture

```text
Authorized camera/event source         Authorized physical sensor
             |                                   |
             v                                   v
      flock-adapter                    Rust/C/C++ signal input
             |                                   |
             +----------> normalized time/zone <-+
                                  |
                                  v
                         anonymous SignalTemplate
                          /                   \
                         v                     v
                event/safety logic       signed evidence ledger
                         |                     |
                         v                     v
                    human review          offline verifier
```

Future regulated research remains a separate extension point:

```text
anonymous SignalTemplate
          |
          v
AuthorizedIdentityResolver (interface/research boundary only)
          |
          v
blinded evaluation result + uncertainty + audit record
```

See [`research/authorized-identity-resolution/README.md`](research/authorized-identity-resolution/README.md).

## Repository map

```text
rust/
  signal-core/       domain types + canonical observation digest
  signal-features/   deterministic feature extraction
  signal-ledger/     signed tamper-evident evidence chain
  policy-engine/     production/research action policy
  flock-adapter/     normalized authorized camera events
  ffi/               C ABI backed by Rust
cpp/
  include/           C and C++20 SDK headers
  tests/             native smoke test
services/gateway/    authenticated reference HTTP ingress
database/migrations/ PostgreSQL schema
simulator/            synthetic authorized event fixtures
install/docker/       container/Compose stack
install/linux/        native Linux installer + systemd unit
install/windows/      native Windows installer + launcher
compliance/           claims, threat/privacy/security/procurement docs
research/             separately governed experimental interface
```

## Quick start: Docker Compose

Requirements: Docker Engine with Compose support.

From the repository root:

```bash
cp .env.example .env
```

Replace both placeholder secrets in `.env`. A local development token can be generated with a cryptographically secure tool available on your system, for example:

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

Expected response shape:

```json
{"status":"ok","identity_resolution":"disabled"}
```

Send the synthetic camera events in another shell:

```bash
export FLOCK_SIGNAL_API_TOKEN='<same token from .env>'
./simulator/send_sample.sh
```

The second sample deliberately contains `face_occlusion=present` together with independent restricted-zone/after-hours context. The adapter records the occlusion state but does not create an identity or guilt classification.

## Native Rust build

Requires Rust 1.80+.

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo build --workspace
```

## C++20 SDK build

Requires Rust/Cargo, CMake 3.24+, and a C++20 compiler.

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure
```

CMake builds the Rust `cdylib`, links the C++ wrapper against it, and runs the native smoke program.

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
void fs_template_destroy(fs_template* value);
```

The FFI validates null/invalid inputs and uses panic containment so a Rust panic is not intentionally allowed to unwind across the C ABI boundary.

## Linux installation

The native installer builds/tests the Rust workspace and C++ smoke target before copying the gateway and systemd unit.

```bash
chmod +x install/linux/install.sh
./install/linux/install.sh
```

The installer creates `/opt/flock-signal/etc/flock-signal.env` with a `REPLACE_ME` token if no config exists. Replace it before enabling the service.

## Windows installation

From PowerShell with Rust, CMake, and a C++ toolchain installed:

```powershell
.\install\windows\install.ps1
```

Then set a strong token and run the installed launcher:

```powershell
$env:FLOCK_SIGNAL_API_TOKEN = '<strong-secret>'
& "$env:LOCALAPPDATA\FlockSignal\start.ps1"
```

## Camera/Flock integration boundary

`rust/flock-adapter` intentionally implements a normalized event contract rather than guessing undocumented vendor endpoints.

A real Flock integration must use authorized vendor API access and current contractual/documented schemas. External records should be converted into `ExternalCameraEvent`, then validated by `normalize_event`.

The repository never requires or stores a real Flock credential in source control.

## Physical signal sources

Numeric signal arrays can enter through Rust directly or the C/C++ ABI. A production sensor adapter can represent authorized modalities such as mechanical, cardiac, muscular, acoustic, thermal, or biomagnetic measurements.

A conventional optical camera is not a magnetometer. A biomagnetic deployment would require dedicated physical sensing hardware, calibration, environmental interference characterization, and scientific validation for the claimed operating conditions.

## Evidence ledger

`signal-ledger` binds evidence content to:

- record/event IDs;
- observation time;
- sensor/template references;
- payload digest;
- previous-record digest;
- signer key ID;
- policy context.

The reference implementation computes a deterministic SHA-256 record digest and signs it with Ed25519. `LedgerVerifier` checks both record integrity and chain ordering offline.

For a government environment requiring a specific validated cryptographic boundary, route signing/key operations through the purchaser-approved module/KMS/HSM. The reference cryptographic implementation is not itself a FIPS-validation claim.

## Non-suspect protection

The production schema and domain model are designed around minimization:

- `SignalTemplate` has no person identity field;
- the PostgreSQL `signal_templates` table has no civilian/person foreign key;
- mask/occlusion is contextual metadata only;
- the policy engine refuses `CriminalClassification` decisions;
- identity resolution is denied without explicit research authorization metadata;
- raw physiological sample persistence is not required by the production schema;
- evidence and access are intended to be auditable and retention-limited.

See:

- [`compliance/PRIVACY_IMPACT_TEMPLATE.md`](compliance/PRIVACY_IMPACT_TEMPLATE.md)
- [`compliance/DATA_RETENTION_POLICY.md`](compliance/DATA_RETENTION_POLICY.md)
- [`compliance/THREAT_MODEL.md`](compliance/THREAT_MODEL.md)

## Government/security evaluation

Start with [`compliance/PROCUREMENT_READINESS.md`](compliance/PROCUREMENT_READINESS.md).

The strongest accurate current positioning is:

> **An auditable multimodal forensic signal and public-safety integration platform with Rust/C++ edge compatibility, tamper-evident evidence, privacy-aware anonymous signal representations, and a separately governed experimental biomagnetic/physiological research interface.**

Before a high-assurance federal deployment, the procurement matrix calls out the remaining work including production IAM/mTLS, approved cryptographic boundary/KMS, SBOM and signed provenance, independent penetration testing, automatic retention enforcement, authorized vendor sandbox validation, and scientific validation for any physical sensor claims.

## Documentation

- Approved architecture: `docs/superpowers/specs/2026-08-18-flock-signal-safety-platform-design.md`
- Implementation plan: `docs/superpowers/plans/2026-08-18-flock-signal-safety-v0.1.md`
- Scientific claims: `compliance/SCIENTIFIC_CLAIMS.md`
- Threat model: `compliance/THREAT_MODEL.md`
- Privacy template: `compliance/PRIVACY_IMPACT_TEMPLATE.md`
- Security controls: `compliance/SECURITY_CONTROLS.md`
- Retention baseline: `compliance/DATA_RETENTION_POLICY.md`
- Procurement readiness: `compliance/PROCUREMENT_READINESS.md`

## Development rule

**Be aggressive about detecting threats and preserving evidence; be conservative about identifying people.**

A safety event is a prompt for authorized human review, not a mathematical declaration of guilt.
