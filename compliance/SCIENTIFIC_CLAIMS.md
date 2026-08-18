# Scientific Claims and Validation Boundary

## Status

This document separates what this repository **demonstrates in software** from what biomagnetic/physiological research **supports as background science** and what remains an **unvalidated research hypothesis**.

## Demonstrated by this repository

The software is designed to:

- accept authorized camera-event metadata through a normalized adapter;
- accept numeric sensor sample arrays through the Rust/C ABI;
- calculate deterministic time/frequency-domain features such as RMS energy, zero-crossing rate, and spectral centroid;
- derive anonymous `SignalTemplate` records containing no person identity field;
- preserve source provenance using SHA-256 digests;
- create Ed25519-signed tamper-evident ledger records;
- deny identity-resolution operations by default in the production policy engine;
- expose Rust functionality to C and C++20 software;
- record facial occlusion only as contextual event metadata.

These are software behaviors. They do not establish that a camera can measure a person's biomagnetic field or that an anonymous signal template uniquely identifies a human being.

## Supported scientific background

Human organs, nerves, and muscles generate measurable biomagnetic phenomena. Magnetocardiography, magnetoencephalography, and magnetomyography are established measurement fields, but the signals are extremely weak relative to the Earth's magnetic field and ordinary environmental interference.

A historical/technical review reports cardiac magnetic signals measured above the chest on the order of tens of picotesla and brain signals around the picotesla scale, with biomagnetic signals many orders of magnitude below the Earth's field. See: https://pmc.ncbi.nlm.nih.gov/articles/PMC11139488/

Modern optically pumped magnetometers (OPMs) can measure human biomagnetic activity. A 2024 gradiometer study demonstrated human auditory-evoked brain response and real-time magnetocardiography: https://pmc.ncbi.nlm.nih.gov/articles/PMC11047143/

Magnetomyography research has demonstrated contactless measurement of muscle magnetic activity with OPMs, including proof-of-principle clinical and movement-discrimination studies:

- https://pmc.ncbi.nlm.nih.gov/articles/PMC9745080/
- https://pmc.ncbi.nlm.nih.gov/articles/PMC10719385/
- https://pmc.ncbi.nlm.nih.gov/articles/PMC11327291/

These studies support the proposition that physiological magnetic signals can be measured with specialized sensors. They do **not** establish a stable, person-unique, long-range biometric identifier suitable for identifying unknown masked people in unconstrained public environments.

## Important sensor distinction

A conventional optical camera does not become a biomagnetic sensor through software alone. Any future biomagnetic component would require dedicated physical sensing hardware with sufficient sensitivity, calibration, timing, environmental-noise rejection, and validated operating distance.

The `flock-adapter` in this repository therefore handles event metadata. Separate authorized sensor adapters would be required for magnetic, mechanical, cardiac, acoustic, thermal, or other measurements.

## Experimental identity hypothesis

The repository preserves a research interface for the hypothesis that a multimodal physiological representation could someday have person-specific repeatability. No production resolver is implemented.

Before any person-identification claim is made, an independently governed study must establish, at minimum:

1. controlled enrollment with informed/authorized collection;
2. blinded train/test separation;
3. repeat measurements across days and months;
4. multiple sensor units and operators;
5. motion, clothing, temperature, exertion, and environmental variation;
6. sensor drift and calibration variation;
7. open-set testing containing people never enrolled in the reference set;
8. false-match and false-non-match rates;
9. confidence calibration and threshold analysis;
10. subgroup/cohort performance analysis;
11. adversarial and interference testing;
12. independent replication by a separate research group.

A similarity score must never be represented as certainty.

## Mask/occlusion interpretation

Facial covering can reduce the information available to ordinary optical systems. This repository may record `face_occlusion=present|absent|unknown`, but mask wearing alone is never treated as evidence of guilt, criminal intent, or identity.

## Claims prohibited in product material until independently demonstrated

Do not claim that the software:

- identifies every human by a unique magnetic frequency;
- identifies unknown people through walls, clothing, or masks;
- provides zero-error biometric identification;
- can derive identity from ordinary Flock camera imagery using biomagnetism;
- is scientifically validated for forensic identity matching;
- is approved, certified, or endorsed by a government agency merely because its architecture targets public-safety use.

## Future evidence record

If a future authorized study supplies a candidate resolver, every result should include the sensor and calibration provenance, feature schema/version, model/version, reference population, matching threshold, similarity score, uncertainty, error-rate operating point, authorization reference, and signed audit record.
