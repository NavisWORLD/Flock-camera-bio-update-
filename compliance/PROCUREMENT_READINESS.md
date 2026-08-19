# Procurement Readiness

## Objective

Prepare the repository for technical evaluation by U.S. public-safety, school-safety, forensic, and government security organizations without implying endorsement or certification that has not occurred.

## Included in repository

- architecture and implementation specification;
- Rust anonymous signal domain model;
- deterministic signal feature engine;
- tamper-evident SHA-256 / Ed25519 evidence ledger;
- deny-by-default policy engine;
- provider-neutral Flock-compatible adapter boundary and synthetic fixture;
- C ABI and C++20 SDK;
- PostgreSQL schema;
- authenticated health/readiness and processing gateway;
- Docker, Linux, and Windows installation assets;
- threat model;
- privacy impact template;
- scientific claims register;
- regulated research pathway documentation;
- committed Rust dependency lockfile;
- automated Linux/Windows Rust+C++ CI;
- verified Linux and Windows evaluation artifacts.

## Verified engineering evidence

Release-hardening CI run `32201084255` completed successfully on both Linux and Windows using Rust 1.82 and the committed lockfile.

The verification gate covered:

- Rustfmt;
- Clippy with `-D warnings`;
- all Rust tests;
- locked optimized release builds;
- CMake configure/build;
- Rust↔C++ interoperability smoke testing;
- Linux/Windows artifact packaging and upload.

Verified evaluation-artifact digests:

- Linux SHA-256: `c01498f29e9a0e5c2c7f8d2b662e4ac824038ce992905b060b980396871321c6`
- Windows SHA-256: `8f9a8b02617c08f4c79548ddc2e3631672f72e16f9b65baa11d526d38eee9382`

## Must still be completed before a production procurement claim

- software bill of materials suitable for the target procurement process;
- vulnerability scan results and remediation record;
- signed production release artifacts and release provenance;
- deployment-specific penetration/security assessment;
- production Flock/API credentials and contract-authorized integration testing where applicable;
- deploying-agency privacy/legal/civil-rights review;
- retention and records-management approval;
- operational logging/SIEM integration;
- disaster recovery and incident response validation;
- performance/load test evidence on target hardware;
- any agency-specific accessibility, acquisition, hosting, or authorization requirements.

## Scientific capability status

Frequency-domain signal analysis is implemented as a mathematical/software capability. Person-specific physiological or biomagnetic identification remains an experimental research question and is not represented as a validated production capability.

## External dependency status

This repository does not represent itself as endorsed by Flock Safety or any U.S. government agency. An operational integration requires authorized provider/API access and whatever contractual, legal, security, privacy, records-management, and government approvals apply to the actual deployment.

## CI status

The earlier GitHub Actions runner-allocation problem is resolved. Cross-platform repository verification now executes successfully on Linux and Windows. This establishes build/test evidence for the repository; it does **not** establish Flock Safety certification, government authorization, evidentiary admissibility, procurement approval, or scientific validation of a remote person-unique magnetic/physiological identifier.
