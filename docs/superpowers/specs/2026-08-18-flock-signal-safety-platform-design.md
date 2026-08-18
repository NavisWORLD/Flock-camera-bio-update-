# Flock Signal Safety Platform Design

**Date:** 2026-08-18  
**Status:** Approved architecture, implementation pending spec review  
**Repository:** `NavisWORLD/Flock-camera-bio-update-`

## 1. Purpose

Build a full-stack public-safety signal analysis platform that can integrate with authorized Flock Safety event feeds and future camera/sensor systems while remaining scientifically honest, auditable, portable, and suitable for security review.

The deployable product will correlate camera events with non-identifying multimodal signal observations, generate anonymous signal templates, detect anomalies, maintain a tamper-evident forensic ledger, and expose Rust and C++ SDKs.

The repository will also preserve a clearly separated **future regulated identity-resolution pathway** for government, academic, and forensic research. That pathway will be documented and represented by interfaces and test harnesses, but the shipping product will not implement covert population-scale biometric identification.

## 2. Scientific Position

Human cardiac, neural, muscular, mechanical, thermal, acoustic, and electromagnetic activity can produce measurable signals. Whether any remote combination of these measurements can become a stable, person-specific identifier across time, environment, clothing, motion, sensor drift, physiology, and population scale is an empirical question that must be validated rather than assumed.

Accordingly, the platform treats physiological or biomagnetic identity resolution as an **experimental research hypothesis** until controlled studies demonstrate operationally useful false-match and false-non-match rates.

The production system may derive frequency-domain and time-domain features from authorized sensors. It must not label those features as a unique human identity unless an independently validated and authorized resolver exists.

## 3. Product Boundaries

### 3.1 Shipping capability

The production system will provide:

- authorized Flock event ingestion;
- generic camera and sensor adapters;
- synchronized multimodal event windows;
- FFT and spectral feature extraction;
- time-domain feature extraction;
- signal-quality and uncertainty scoring;
- anonymous session and template identifiers;
- anomaly and event correlation;
- mask/occlusion observation as a contextual event attribute;
- restricted-zone and time-window rules;
- append-only forensic evidence ledger;
- cryptographic integrity verification;
- Rust and C++ SDKs;
- REST/gRPC service interfaces;
- PostgreSQL persistence;
- Docker and native installation paths;
- simulators and synthetic datasets;
- automated tests, benchmarks, threat model, SBOM, and deployment documentation.

### 3.2 Explicit non-goals for the shipping product

The production system will not:

- maintain a covert database mapping involuntary physiological templates to civilian names;
- automatically identify arbitrary passersby from biological or magnetic signals;
- treat mask wearing alone as evidence of criminal conduct;
- infer protected traits from physiological measurements;
- hide collection, matching, or access activity from authorized auditors;
- bypass legal process, policy controls, retention controls, or human review.

## 4. Design Principle

The system separates four concerns:

1. **Observation** — collect authorized camera/sensor events.
2. **Representation** — convert measurements into reproducible numerical features.
3. **Evidence** — preserve provenance and cryptographic integrity.
4. **Identity resolution** — remain absent from the production path unless a separately governed, validated, and authorized module is supplied.

Core pipeline:

```text
Authorized Sensor/Event Source
        |
        v
Normalization + Clock Alignment
        |
        v
Feature Extraction
        |
        v
Anonymous Signal Template
        |
        +------> Event Correlator ------> Safety Event
        |
        v
Signed Forensic Ledger
```

Future regulated research extension:

```text
Anonymous Signal Template
        |
        v
AuthorizedIdentityResolver interface
        |
        v
Research-only match result + uncertainty + audit record
```

The resolver interface exists so a future authorized program can evaluate identity science without redesigning the platform.

## 5. Repository Structure

```text
/
├── Cargo.toml
├── CMakeLists.txt
├── README.md
├── LICENSE
├── rust/
│   ├── signal-core/
│   ├── signal-features/
│   ├── signal-ledger/
│   ├── policy-engine/
│   ├── flock-adapter/
│   └── ffi/
├── cpp/
│   ├── include/flock_signal/
│   ├── src/
│   ├── examples/
│   └── tests/
├── services/
│   ├── gateway/
│   ├── ledger-api/
│   └── event-correlator/
├── database/
│   ├── migrations/
│   └── schema/
├── simulator/
│   ├── sensor-generator/
│   └── flock-event-generator/
├── research/
│   └── authorized-identity-resolution/
│       ├── README.md
│       ├── interfaces/
│       ├── evaluation/
│       └── synthetic-tests/
├── compliance/
│   ├── THREAT_MODEL.md
│   ├── PRIVACY_IMPACT_TEMPLATE.md
│   ├── DATA_RETENTION_POLICY.md
│   ├── SECURITY_CONTROLS.md
│   ├── SCIENTIFIC_CLAIMS.md
│   └── PROCUREMENT_READINESS.md
├── docs/
│   ├── architecture/
│   ├── deployment/
│   ├── api/
│   └── superpowers/
├── install/
│   ├── docker/
│   ├── linux/
│   └── windows/
├── tests/
│   ├── integration/
│   ├── interoperability/
│   ├── tamper/
│   └── performance/
└── .github/workflows/
```

## 6. Rust Architecture

### 6.1 `signal-core`

Owns common domain types and deterministic processing primitives.

Primary types:

```rust
pub struct SensorFrame {
    pub sensor_id: String,
    pub source_kind: SourceKind,
    pub timestamp_ns: i128,
    pub sample_rate_hz: f64,
    pub channels: Vec<Vec<f32>>,
    pub metadata: BTreeMap<String, String>,
}

pub struct ObservationWindow {
    pub window_id: Uuid,
    pub start_ns: i128,
    pub end_ns: i128,
    pub frames: Vec<SensorFrame>,
}

pub struct SignalTemplate {
    pub template_id: Uuid,
    pub feature_schema: String,
    pub feature_version: String,
    pub features: Vec<f32>,
    pub quality: f32,
    pub uncertainty: f32,
    pub source_digest: [u8; 32],
}
```

`SignalTemplate` is anonymous by design and contains no name, government identifier, face embedding, or person record key.

### 6.2 `signal-features`

Provides deterministic feature transforms including:

- DC removal;
- configurable band-pass filtering;
- windowing;
- FFT magnitude and phase-safe summaries;
- band energy;
- spectral centroid;
- spectral rolloff;
- zero-crossing rate;
- RMS energy;
- autocorrelation peaks;
- cross-channel correlation;
- short-window temporal statistics;
- signal-to-noise estimates;
- missing-data indicators.

All feature schemas are versioned.

### 6.3 `signal-ledger`

Implements append-only evidence records.

```rust
pub struct LedgerRecord {
    pub record_id: Uuid,
    pub observed_at_ns: i128,
    pub event_id: Uuid,
    pub sensor_ids: Vec<String>,
    pub template_ids: Vec<Uuid>,
    pub payload_digest: [u8; 32],
    pub previous_record_digest: [u8; 32],
    pub record_digest: [u8; 32],
    pub signer_key_id: String,
    pub signature: Vec<u8>,
    pub policy_context: PolicyContext,
}
```

Ledger integrity uses SHA-256 for content hashing and Ed25519 signatures. Verification must be deterministic and available offline.

### 6.4 `policy-engine`

Evaluates whether an action is permitted before identity-adjacent, retention, export, or investigative operations are attempted.

Production policies include:

- anonymous observation creation;
- event correlation;
- evidence export;
- retention expiry;
- restricted-zone rules;
- operator role checks;
- research-mode isolation.

### 6.5 `flock-adapter`

Provides an adapter boundary for authorized Flock data sources.

The adapter must be designed around documented and authorized APIs, test fixtures, and mocks. Secrets are loaded through environment/secret-manager interfaces and never committed to the repository.

The adapter converts external events into an internal normalized event form:

```rust
pub struct CameraEvent {
    pub event_id: Uuid,
    pub camera_id: String,
    pub observed_at_ns: i128,
    pub zone_id: Option<String>,
    pub event_kind: String,
    pub attributes: BTreeMap<String, String>,
    pub source_uri: Option<String>,
}
```

Mask or facial occlusion may be represented only as an event attribute such as `face_occlusion=present|absent|unknown`. It must not independently trigger a criminal classification.

### 6.6 `ffi`

Exports a stable C ABI for C++ and other native environments.

Initial ABI surface:

```c
typedef struct fs_engine fs_engine;
typedef struct fs_template fs_template;

typedef struct {
    const float* samples;
    size_t sample_count;
    double sample_rate_hz;
    int64_t timestamp_ns;
} fs_channel_frame;

int fs_engine_create(fs_engine** out_engine);
void fs_engine_destroy(fs_engine* engine);
int fs_engine_extract_template(
    fs_engine* engine,
    const fs_channel_frame* channels,
    size_t channel_count,
    fs_template** out_template
);
void fs_template_destroy(fs_template* value);
```

Every exported function returns explicit status codes. Panics must never cross the FFI boundary.

## 7. C++ SDK

The C++ layer wraps the C ABI with RAII types and standard CMake packaging.

Primary user-facing classes:

```cpp
namespace flock_signal {

class Engine {
public:
    Engine();
    SignalTemplate extract(const Observation& observation) const;
};

class LedgerVerifier {
public:
    VerificationResult verify(const LedgerRecord& record) const;
};

}
```

The SDK must compile with C++20 and support Linux and Windows first. macOS may be validated in CI when a runner is available.

## 8. Services

### 8.1 Gateway

Responsibilities:

- authentication;
- API versioning;
- request IDs;
- rate limits;
- health/readiness endpoints;
- structured audit events.

### 8.2 Event Correlator

Correlates camera events and sensor observations by:

- time window;
- sensor/camera zone;
- known topology;
- observation quality;
- configured rule predicates.

It produces a `SafetyEvent` rather than a person identity.

### 8.3 Ledger API

Supports:

- append record;
- retrieve record by ID;
- verify record chain;
- export signed evidence bundle;
- inspect provenance;
- retention status.

Every retrieval and export creates an audit event.

## 9. Database Model

PostgreSQL stores operational metadata.

Primary tables:

- `camera_events`;
- `sensor_observations`;
- `signal_templates`;
- `safety_events`;
- `ledger_records`;
- `audit_events`;
- `retention_jobs`;
- `policy_decisions`.

Production schema contains no table equivalent to `person -> involuntary physiological template`.

Raw samples are disabled by default and retained only when explicitly configured for an authorized research or forensic collection.

## 10. Forensic Evidence Bundle

Evidence exports use a deterministic directory or archive structure:

```text
evidence/<event-id>/
├── manifest.json
├── event.json
├── observations.json
├── templates.json
├── ledger.jsonl
├── signatures/
└── verification.txt
```

`manifest.json` contains hashes for every included file.

The verifier must detect:

- modified payloads;
- missing ledger entries;
- reordered ledger entries;
- invalid signatures;
- incorrect previous-record hashes;
- unknown feature schema versions.

## 11. Mask and Occlusion Handling

The system may record visible occlusion because it can affect the confidence of ordinary camera observations. Occlusion is not itself proof of wrongdoing.

A policy rule may combine occlusion with independent authorized conditions, for example:

```text
restricted_zone == true
AND after_hours == true
AND face_occlusion == present
```

The resulting output is an elevated safety event requiring human review, not an automatic identity or guilt determination.

## 12. Future Regulated Identity-Resolution Pathway

The repository preserves a research-only extension point:

```rust
pub trait AuthorizedIdentityResolver {
    fn evaluate(
        &self,
        template: &SignalTemplate,
        context: &ResearchAuthorization,
    ) -> Result<ResearchMatchResult, ResolverError>;
}
```

`ResearchAuthorization` must carry explicit authorization metadata and cannot be constructed by the normal production API.

The repository will include:

- interface definitions;
- synthetic-template match experiments;
- evaluation metrics;
- dataset protocol templates;
- repeatability tests;
- false-match and false-non-match analysis tools;
- bias and cohort-performance reporting templates;
- adversarial robustness evaluation guidance;
- documentation describing conditions required before operational consideration.

The repository will **not** include a production implementation that silently resolves arbitrary anonymous templates to civilian identities.

A future government or accredited research program can supply its own independently governed implementation only after legal, scientific, security, and deployment review.

## 13. Scientific Validation Protocol

Any claim of person-specific physiological identification must be evaluated using controlled enrollment and blinded testing.

Required dimensions include:

- large and diverse participant cohorts;
- repeat recordings across days and months;
- different sensor units;
- different operators and environments;
- walking and stationary conditions;
- clothing variation;
- temperature and weather variation;
- heart-rate and exertion variation;
- illness and medication variation where ethically approved;
- sensor drift and calibration variation;
- open-set testing with unseen subjects;
- false-match rate;
- false-non-match rate;
- equal-error rate where applicable;
- confidence calibration;
- subgroup performance reporting;
- independent replication.

The system must report uncertainty and must never convert a research similarity score into certainty.

## 14. Security Architecture

Baseline controls:

- mTLS for service-to-service communication;
- TLS for external APIs;
- Ed25519 signing keys protected outside source control;
- SHA-256 evidence hashing;
- role-based access control;
- least-privilege service accounts;
- secret-manager integration;
- immutable audit events;
- request correlation IDs;
- configurable retention;
- deny-by-default research routes;
- dependency scanning;
- SBOM generation;
- signed release artifacts where CI supports signing;
- no plaintext credentials in logs;
- no raw sensor payloads in ordinary application logs.

## 15. Privacy and Non-Suspect Protection

Production behavior defaults to data minimization.

- Anonymous observations receive random UUIDs.
- Identity fields are absent from `SignalTemplate`.
- Raw signals expire according to policy and are off by default.
- Derived templates have configurable retention.
- All exports are logged.
- Search access is auditable.
- Research namespaces are isolated.
- Operators cannot silently enable a person-identity resolver through ordinary configuration.

## 16. Installation and Deployment

Supported deployment targets for the first release:

### Docker

Docker Compose stack containing:

- gateway;
- correlator;
- ledger API;
- PostgreSQL;
- simulator profile.

### Linux

Native binaries plus systemd service files and environment templates.

### Windows

Native Rust/C++ binaries and PowerShell installation scripts.

All installers perform:

- dependency checks;
- config generation;
- database migration;
- health verification;
- example/simulator smoke test.

## 17. Configuration

Configuration must be explicit and environment-independent.

Example sections:

```toml
[server]
bind = "0.0.0.0:8080"

[database]
url_env = "FLOCK_SIGNAL_DATABASE_URL"

[ledger]
signing_key_env = "FLOCK_SIGNAL_SIGNING_KEY"

[retention]
observation_hours = 24
template_days = 30

[research]
enabled = false
```

`research.enabled` does not activate identity resolution. It only enables local research/evaluation endpoints when additional authorization configuration is present.

## 18. Testing Strategy

### Unit tests

- DSP transforms;
- schema versioning;
- digest stability;
- signature verification;
- policy evaluation;
- FFI lifetime behavior;
- error conversion.

### Interoperability tests

- Rust producer -> C++ consumer;
- C++ producer -> Rust verifier;
- C ABI misuse protection;
- Windows/Linux binary compatibility expectations.

### Integration tests

- synthetic camera event + sensor window -> safety event;
- safety event -> ledger record;
- ledger record -> signed evidence bundle;
- database restart/recovery;
- retention expiry;
- Flock adapter mock authorization failure;
- simulator end-to-end scenario.

### Tamper tests

- change a feature value;
- remove a ledger entry;
- reorder records;
- replace a signature;
- alter a manifest hash;
- use an unknown signer.

Each must fail verification.

### Performance tests

Measure:

- feature extraction throughput;
- end-to-end event latency;
- ledger append throughput;
- verification throughput;
- memory use;
- C++/Rust FFI overhead.

## 19. CI/CD

GitHub Actions will run:

- `cargo fmt --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --workspace`;
- CMake configure/build/test;
- Rust/C++ interoperability tests;
- integration tests with PostgreSQL service container;
- dependency audit;
- SBOM generation;
- release build on tagged versions.

No CI job receives production credentials.

## 20. Demonstration Mode

A safe demo must work without Flock credentials.

The simulator creates:

1. synthetic camera events;
2. synthetic multichannel sensor waveforms;
3. anonymous feature templates;
4. correlated safety events;
5. signed ledger entries;
6. evidence bundles;
7. integrity verification output.

This demo is the primary reproducible proof that the stack works before any external integration is configured.

## 21. Government / Enterprise Readiness Documentation

The repository will contain documents suitable for technical review:

- architecture overview;
- threat model;
- privacy impact template;
- data-retention policy;
- scientific claims matrix;
- known limitations;
- API specification;
- deployment guide;
- administrator guide;
- operator guide;
- incident-response guidance;
- vulnerability-disclosure policy;
- SBOM generation instructions;
- reproducible demo procedure;
- procurement-readiness checklist.

No document will claim government approval, certification, or validated biometric identification unless supporting evidence exists.

## 22. Error Handling

All services use structured error categories:

- invalid input;
- authorization denied;
- policy denied;
- unsupported feature schema;
- sensor quality insufficient;
- external adapter unavailable;
- database unavailable;
- cryptographic verification failed;
- serialization failed;
- internal invariant violation.

Errors exposed to clients must not leak credentials, raw secrets, signing material, or internal stack traces.

## 23. Acceptance Criteria

The first release is complete when all of the following are true:

1. Rust workspace builds cleanly.
2. C++20 SDK builds cleanly through CMake.
3. Rust/C++ interoperability tests pass.
4. Synthetic sensor data produces deterministic feature templates.
5. Camera events and sensor observations correlate into safety events.
6. Ledger records form a verifiable signed hash chain.
7. Tampering is detected by automated tests.
8. PostgreSQL migrations apply from an empty database.
9. Docker Compose demo starts successfully.
10. Linux installation path completes a local smoke test.
11. Windows installation scripts are present and tested in CI where runners support them.
12. Flock integration has a credential-free mock implementation and a documented authorized adapter boundary.
13. Research identity-resolution interfaces compile, but no covert production resolver ships.
14. Scientific-claims documentation distinguishes demonstrated features from hypotheses.
15. Security, privacy, threat-model, and procurement documents are present.
16. CI passes on the default branch.

## 24. Implementation Order

Implementation should proceed in this sequence:

1. workspace and CI foundation;
2. `signal-core` domain model;
3. deterministic signal-feature engine;
4. ledger hashing/signatures;
5. policy engine;
6. C ABI;
7. C++20 wrapper SDK;
8. PostgreSQL schema/migrations;
9. event correlator;
10. mock Flock adapter;
11. gateway and ledger API;
12. simulator;
13. evidence export/verifier;
14. research-only resolver interface and evaluation harness;
15. Docker/Linux/Windows packaging;
16. compliance/procurement documentation;
17. full integration, tamper, and performance test pass.

## 25. Final Safety Boundary

The architecture intentionally preserves future scientific and government research possibilities without representing an unvalidated hypothesis as an operational biometric capability.

The deployable platform therefore provides the sensing, mathematical representation, interoperability, evidence, security, simulation, and evaluation infrastructure needed to study advanced forensic signal systems while keeping identity resolution outside the ordinary production path unless a separately authorized and validated implementation is introduced.