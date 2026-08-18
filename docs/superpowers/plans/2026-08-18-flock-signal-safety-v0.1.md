# Flock Signal Safety v0.1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a testable first release of the anonymous multimodal forensic signal safety platform with Rust core processing, tamper-evident evidence, policy isolation, a stable C ABI/C++20 wrapper, simulator inputs, PostgreSQL schema, and deployment/CI scaffolding.

**Architecture:** A Rust workspace owns domain types, feature extraction, policy decisions, evidence hashing/signing, and the C ABI. C++20 wraps the ABI with RAII. External camera/sensor integrations normalize data into anonymous observation and event types. Identity resolution remains a research-only interface with no shipping implementation that maps involuntary physiological templates to civilian identities.

**Tech Stack:** Rust 1.80+, Cargo workspace, serde, uuid, sha2, ed25519-dalek, rustfft, thiserror; C++20/CMake 3.24+; PostgreSQL 15+; Docker Compose; GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-08-18-flock-signal-safety-platform-design.md`

## Global Constraints

- The shipping product must not maintain a covert `person -> involuntary physiological template` database.
- Mask or facial occlusion is contextual metadata only and never independently establishes criminality or identity.
- `SignalTemplate` contains no name, government identifier, face embedding, or person record key.
- Research identity resolution is represented only by authorization-aware interfaces, synthetic tests, and validation documentation.
- Rust panics must never cross the C ABI boundary.
- Evidence hashing and verification must be deterministic and available offline.
- Secrets and signing keys must never be committed.

---

### Task 1: Rust workspace and anonymous signal core

**Files:**
- Create: `Cargo.toml`
- Create: `rust/signal-core/Cargo.toml`
- Create: `rust/signal-core/src/lib.rs`
- Create: `rust/signal-features/Cargo.toml`
- Create: `rust/signal-features/src/lib.rs`

**Interfaces:**
- Produces: `SensorFrame`, `ObservationWindow`, `SignalTemplate`, `FeatureExtractor::extract(&ObservationWindow) -> Result<SignalTemplate, FeatureError>`.

- [ ] Write unit tests first for anonymous template fields, deterministic digesting, RMS energy, zero-crossing rate, spectral centroid, and invalid sample-rate handling.
- [ ] Run `cargo test -p signal-core -p signal-features` and confirm RED failures are caused by missing implementation.
- [ ] Implement the minimal domain types and deterministic feature extraction to satisfy the tests.
- [ ] Run `cargo test -p signal-core -p signal-features` and require zero failures.

### Task 2: Policy engine and regulated research boundary

**Files:**
- Create: `rust/policy-engine/Cargo.toml`
- Create: `rust/policy-engine/src/lib.rs`
- Create: `research/authorized-identity-resolution/README.md`

**Interfaces:**
- Produces: `PolicyAction`, `PolicyContext`, `PolicyDecision`, `PolicyEngine::evaluate`, `ResearchAuthorization`, and the `AuthorizedIdentityResolver` trait definition without a production resolver implementation.

- [ ] Write tests first proving anonymous correlation is allowed, identity resolution is denied without research authorization, and mask presence alone cannot produce a criminal classification.
- [ ] Run `cargo test -p policy-engine` and confirm expected RED failures.
- [ ] Implement deny-by-default policy logic and research-only authorization types.
- [ ] Run `cargo test -p policy-engine` and require zero failures.

### Task 3: Tamper-evident forensic ledger

**Files:**
- Create: `rust/signal-ledger/Cargo.toml`
- Create: `rust/signal-ledger/src/lib.rs`

**Interfaces:**
- Produces: `LedgerRecord`, `LedgerSigner`, `LedgerVerifier`, `LedgerError`.

- [ ] Write tests first for deterministic record digests, valid Ed25519 verification, tamper detection, previous-record-chain verification, and reordered-record detection.
- [ ] Run `cargo test -p signal-ledger` and confirm expected RED failures.
- [ ] Implement SHA-256 digesting, Ed25519 signing, and offline chain verification.
- [ ] Run `cargo test -p signal-ledger` and require zero failures.

### Task 4: Stable C ABI and C++20 compatibility SDK

**Files:**
- Create: `rust/ffi/Cargo.toml`
- Create: `rust/ffi/src/lib.rs`
- Create: `cpp/include/flock_signal/flock_signal.h`
- Create: `cpp/include/flock_signal/flock_signal.hpp`
- Create: `cpp/tests/smoke.cpp`
- Create: `cpp/CMakeLists.txt`
- Create: `CMakeLists.txt`

**Interfaces:**
- Produces: `fs_engine_create`, `fs_engine_destroy`, `fs_engine_extract_template`, `fs_template_destroy`, `fs_template_quality`, and C++ `flock_signal::Engine` RAII wrapper.

- [ ] Write Rust FFI tests first for null-pointer rejection, lifecycle safety, extraction success, and panic containment.
- [ ] Run `cargo test -p flock-signal-ffi` and confirm expected RED failures.
- [ ] Implement the minimal C ABI with explicit status codes and `catch_unwind` boundaries.
- [ ] Add C++20 headers and a smoke executable that creates an engine and extracts a template.
- [ ] Run Rust tests and `cmake`/`ctest` when the native toolchain is available.

### Task 5: Normalized camera adapter, simulator, database, and install stack

**Files:**
- Create: `rust/flock-adapter/Cargo.toml`
- Create: `rust/flock-adapter/src/lib.rs`
- Create: `simulator/sample-events.jsonl`
- Create: `database/migrations/0001_init.sql`
- Create: `install/docker/docker-compose.yml`
- Create: `install/docker/Dockerfile`
- Create: `.env.example`

**Interfaces:**
- Produces: `CameraEvent`, `NormalizedEvent`, validation for `face_occlusion=present|absent|unknown`, database tables for anonymous event/evidence metadata, and a local simulator deployment profile.

- [ ] Write adapter tests first for event normalization, invalid occlusion values, and preservation of source provenance.
- [ ] Run `cargo test -p flock-adapter` and confirm expected RED failures.
- [ ] Implement the normalized event adapter with no real Flock credentials or undocumented API assumptions.
- [ ] Add synthetic simulator data, PostgreSQL schema, Docker build, Compose stack, and environment template.
- [ ] Validate schema and configuration syntactically.

### Task 6: Documentation, compliance scaffolding, CI, and end-to-end verification

**Files:**
- Modify: `README.md`
- Create: `compliance/SCIENTIFIC_CLAIMS.md`
- Create: `compliance/THREAT_MODEL.md`
- Create: `compliance/PRIVACY_IMPACT_TEMPLATE.md`
- Create: `.github/workflows/ci.yml`

**Interfaces:**
- CI must build/test the Rust workspace, build the C++ smoke target, and reject accidental committed secrets using simple repository checks.

- [ ] Document demonstrated vs experimental capabilities and the research-only identity-resolution extension point.
- [ ] Add threat-model/privacy controls for non-suspects, auditability, retention, and operator authorization.
- [ ] Add CI for Rust formatting/build/tests and C++20 smoke compilation.
- [ ] Run fresh local verification: `cargo fmt --check`, `cargo test --workspace`, `cargo build --workspace` and CMake build where available.
- [ ] Inspect the branch diff against the spec and report any remaining gaps explicitly before claiming completion.
