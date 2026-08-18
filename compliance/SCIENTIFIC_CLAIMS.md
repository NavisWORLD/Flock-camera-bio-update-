# Scientific Claims Register

## Purpose

This document prevents experimental hypotheses from being presented as demonstrated product capabilities.

## Demonstrated by the software design

The software is designed to:

- ingest authorized camera/sensor event data through explicit adapters;
- transform sampled numeric signals into deterministic time/frequency-domain feature vectors;
- attach quality and uncertainty values;
- create anonymous template identifiers;
- correlate observations by time, zone, topology, and policy predicates;
- record tamper-evident evidence chains using SHA-256 and Ed25519;
- expose the same feature engine through Rust, C, and C++ interfaces;
- preserve audit and retention metadata.

These are software capabilities and must still be verified by successful builds/tests before a release is described as validated.

## Established scientific premise used by the project

Biological systems can produce measurable electrical, mechanical, acoustic, thermal, and magnetic phenomena. Signal-processing techniques can characterize sampled measurements in time and frequency domains.

The project does not infer from that premise that every person possesses a remotely measurable, stable, unique magnetic identifier.

## Experimental hypothesis

A future multimodal sensor system may discover feature combinations with person-specific repeatability sufficient for a narrowly governed forensic matching task.

That hypothesis requires controlled experiments. It is not a current product claim.

## Claims prohibited without new evidence

Do not state that this repository currently:

- identifies every human by a unique magnetic frequency;
- identifies masked strangers remotely from biomagnetism;
- defeats all disguise or occlusion;
- provides zero false matches;
- proves guilt or criminal intent;
- has received Flock Safety endorsement or integration certification;
- has received U.S. government approval, accreditation, authorization, or procurement certification.

## Evidence required to change status

A claim may move from experimental to validated only when the repository contains or references a reproducible evaluation package containing the protocol, cohort description, sensor configuration, raw/derived-data handling method, blinded evaluation procedure, error metrics, confidence intervals where appropriate, repeatability analysis, limitations, and independent replication status.
