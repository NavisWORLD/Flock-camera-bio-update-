# Flock Signal Safety Platform Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a compiling Rust/C++ public-safety signal platform with anonymous feature extraction, tamper-evident evidence records, policy enforcement, a Flock-compatible adapter boundary, native SDKs, simulator support, deployment assets, and a research-only identity-resolution interface.

**Architecture:** A Rust workspace owns domain types, deterministic DSP features, cryptographic ledger logic, policy decisions, external camera-event normalization, and a stable C ABI. A C++20 SDK wraps that ABI. Production data paths produce anonymous `SignalTemplate` and `SafetyEvent` objects only; identity resolution remains a research-only trait and documentation surface.

**Tech Stack:** Rust stable edition 2021, rustfft, serde, sha2, ed25519-dalek, uuid, thiserror, C ABI, C++20/CMake, PostgreSQL SQL migrations, Docker Compose, GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-08-18-flock-signal-safety-platform-design.md`

## Global Constraints

- Production code must not map involuntary physiological templates to civilian identities.
- Mask or facial occlusion may be an event attribute but may not independently classify criminality.
- `SignalTemplate` is anonymous and carries no person-name or government-identifier field.
- Rust panics must never cross the C ABI.
- Evidence hashing uses SHA-256; signatures use Ed25519.
- C++ SDK targets C++20.
- Research identity-resolution support is interfaces, synthetic evaluation tooling, and documentation only.
- Secrets are loaded from environment or secret-manager interfaces and never committed.

---

### Task 1: Rust workspace and anonymous domain model

**Files:**
- Create: `Cargo.toml`
- Create: `rust/signal-core/Cargo.toml`
- Create: `rust/signal-core/src/lib.rs`

**Interfaces:**
- Produces: `SensorFrame`, `ObservationWindow`, `SignalTemplate`, `CameraEvent`, `SafetyEvent`, `SourceKind`.

- [ ] **Step 1: Write failing domain tests** for anonymous template construction and camera-event occlusion attributes.
- [ ] **Step 2: Run** `cargo test -p signal-core` and confirm failure before implementation.
- [ ] **Step 3: Implement serializable domain types** with UUID identifiers, nanosecond timestamps, bounded quality/uncertainty helpers, and no person identity fields.
- [ ] **Step 4: Run** `cargo test -p signal-core` and require all tests to pass.
- [ ] **Step 5: Commit** with `feat: add anonymous signal domain model`.

### Task 2: Deterministic signal feature extraction

**Files:**
- Create: `rust/signal-features/Cargo.toml`
- Create: `rust/signal-features/src/lib.rs`

**Interfaces:**
- Consumes: `signal_core::{ObservationWindow, SignalTemplate}`.
- Produces: `FeatureExtractor::extract(&ObservationWindow) -> Result<SignalTemplate, FeatureError>`.

- [ ] **Step 1: Write failing tests** for a known sine wave, RMS energy, spectral centroid stability, empty-window rejection, and digest determinism.
- [ ] **Step 2: Run** `cargo test -p signal-features` and confirm the new tests fail.
- [ ] **Step 3: Implement** DC removal, Hann windowing, FFT magnitude summaries, RMS, zero-crossing rate, spectral centroid, band-energy summaries, quality score, uncertainty score, and SHA-256 source digest.
- [ ] **Step 4: Run** `cargo test -p signal-features` and require all tests to pass.
- [ ] **Step 5: Commit** with `feat: add deterministic signal feature extraction`.

### Task 3: Tamper-evident ledger and policy engine

**Files:**
- Create: `rust/signal-ledger/Cargo.toml`
- Create: `rust/signal-ledger/src/lib.rs`
- Create: `rust/policy-engine/Cargo.toml`
- Create: `rust/policy-engine/src/lib.rs`

**Interfaces:**
- Produces: `LedgerRecord`, `LedgerSigner`, `verify_chain`, `PolicyAction`, `PolicyContext`, `PolicyDecision`, `PolicyEngine::evaluate`.

- [ ] **Step 1: Write failing ledger tests** proving payload mutation, record reordering, bad previous digests, and bad signatures are rejected.
- [ ] **Step 2: Write failing policy tests** proving mask-only observations cannot elevate to a criminal classification and research routes default to deny.
- [ ] **Step 3: Implement** canonical record hashing, Ed25519 signing/verification, chain verification, deny-by-default research policy, export policy, retention policy, and restricted-zone contextual rules.
- [ ] **Step 4: Run** `cargo test -p signal-ledger -p policy-engine` and require all tests to pass.
- [ ] **Step 5: Commit** with `feat: add evidence ledger and policy engine`.

### Task 4: Flock-compatible adapter boundary and simulator

**Files:**
- Create: `rust/flock-adapter/Cargo.toml`
- Create: `rust/flock-adapter/src/lib.rs`
- Create: `simulator/README.md`
- Create: `simulator/sample_camera_event.json`

**Interfaces:**
- Consumes: authorized external event JSON.
- Produces: `FlockAdapter::normalize_json(&str) -> Result<CameraEvent, AdapterError>`.

- [ ] **Step 1: Write failing fixture tests** for timestamps, camera IDs, zones, unknown attributes, and `face_occlusion=present|absent|unknown`.
- [ ] **Step 2: Run** `cargo test -p flock-adapter` and confirm failure.
- [ ] **Step 3: Implement** a provider-neutral normalized adapter with a Flock-compatible JSON fixture contract; do not hard-code undocumented private endpoints or credentials.
- [ ] **Step 4: Run** `cargo test -p flock-adapter` and require all tests to pass.
- [ ] **Step 5: Commit** with `feat: add camera event adapter and simulator fixtures`.

### Task 5: Stable C ABI and C++20 SDK

**Files:**
- Create: `rust/ffi/Cargo.toml`
- Create: `rust/ffi/src/lib.rs`
- Create: `cpp/CMakeLists.txt`
- Create: `cpp/include/flock_signal/flock_signal.h`
- Create: `cpp/include/flock_signal/flock_signal.hpp`
- Create: `cpp/src/flock_signal.cpp`
- Create: `cpp/tests/smoke.cpp`
- Create: `CMakeLists.txt`

**Interfaces:**
- Produces C functions: `fs_engine_create`, `fs_engine_destroy`, `fs_engine_extract_template`, `fs_template_destroy`, `fs_template_quality`, `fs_template_uncertainty`.
- Produces C++ class: `flock_signal::Engine` with RAII lifetime management.

- [ ] **Step 1: Add Rust FFI tests** for null pointers, invalid channel input, panic containment, ownership, and deterministic quality output.
- [ ] **Step 2: Implement** opaque handles and `catch_unwind` on every exported operation.
- [ ] **Step 3: Add C++ smoke test** that constructs `Engine`, extracts a template from synthetic samples, and verifies quality is in `[0,1]`.
- [ ] **Step 4: Run** `cargo test -p signal-ffi`, then configure/build CMake and run `ctest --test-dir build --output-on-failure` where a native compiler is available.
- [ ] **Step 5: Commit** with `feat: add C ABI and C++20 SDK`.

### Task 6: Persistence, gateway skeleton, deployment assets, and research boundary

**Files:**
- Create: `database/migrations/001_init.sql`
- Create: `services/gateway/Cargo.toml`
- Create: `services/gateway/src/main.rs`
- Create: `install/docker/docker-compose.yml`
- Create: `install/docker/Dockerfile`
- Create: `install/linux/flock-signal-gateway.service`
- Create: `install/windows/install.ps1`
- Create: `research/authorized-identity-resolution/README.md`
- Create: `research/authorized-identity-resolution/interfaces/README.md`
- Create: `compliance/SCIENTIFIC_CLAIMS.md`
- Create: `compliance/THREAT_MODEL.md`
- Create: `compliance/PRIVACY_IMPACT_TEMPLATE.md`

**Interfaces:**
- Gateway produces `/healthz` and `/readyz` only in the first milestone.
- Database tables: `camera_events`, `sensor_observations`, `signal_templates`, `safety_events`, `ledger_records`, `audit_events`, `retention_jobs`, `policy_decisions`.

- [ ] **Step 1: Add gateway unit/integration tests** for health endpoints and configuration parsing.
- [ ] **Step 2: Implement SQL schema** without a production `person -> physiological template` mapping.
- [ ] **Step 3: Implement gateway skeleton** with structured request IDs and environment-based configuration.
- [ ] **Step 4: Add Docker/native installation assets** and research documentation describing the non-operational `AuthorizedIdentityResolver` extension boundary.
- [ ] **Step 5: Run** workspace tests and gateway smoke tests.
- [ ] **Step 6: Commit** with `feat: add deployment skeleton and regulated research boundary`.

### Task 7: CI, README, and verification

**Files:**
- Create: `.github/workflows/ci.yml`
- Modify: `README.md`

**Interfaces:**
- CI runs Rust formatting, clippy, workspace tests, Linux CMake build/test, and validates required repository files.

- [ ] **Step 1: Add CI workflow** for Ubuntu with Rust stable, CMake, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, CMake configure/build, and CTest.
- [ ] **Step 2: Rewrite README** with architecture, build commands, scientific-claims boundary, deployment instructions, evidence model, and research-extension explanation.
- [ ] **Step 3: Run/trigger CI** and inspect every failed job rather than assuming success.
- [ ] **Step 4: Fix failures** until the available CI checks are green.
- [ ] **Step 5: Commit** with `ci: verify Rust and C++ signal platform`.
