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
- health/readiness gateway skeleton;
- Docker, Linux, and Windows installation assets;
- threat model;
- privacy impact template;
- scientific claims register;
- regulated research pathway documentation;
- automated Rust/C++ CI definition.

## Must be completed before a production procurement claim

- successful clean CI build and test evidence;
- generated dependency lockfile from the verified build;
- software bill of materials from the verified dependency graph;
- vulnerability scan results;
- signed release artifacts and release provenance;
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

This repository does not represent itself as endorsed by Flock Safety or any U.S. government agency. An operational integration requires authorized provider/API access and whatever contractual or government approvals apply to the actual deployment.

## Current CI caveat

During the initial build session, GitHub Actions jobs for this private repository failed before the first runner step, including a runner-only diagnostic job. That is an external runner/repository Actions issue rather than a demonstrated test failure in the source code. The repository must not be labeled build-verified until a runner executes the defined tests successfully.
