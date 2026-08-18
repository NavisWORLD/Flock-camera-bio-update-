# Authorized Identity Resolution — Research Boundary

## Status

**Experimental research interface only. Not a production identification capability.**

The production platform creates anonymous `SignalTemplate` values and safety events. This directory preserves a future extension point for a government, accredited laboratory, university, or forensic program to evaluate whether physiological or biomagnetic measurements can ever support reliable person-specific matching.

The repository does not claim that remote magnetic or physiological signals currently provide a stable unique identifier for arbitrary people in public spaces.

## Why the interface exists

Future sensors may improve. A regulated research program should be able to evaluate new measurements without rewriting the evidence, policy, C++ interoperability, or audit layers.

Conceptual boundary:

```text
Authorized sensor observation
        |
        v
SignalTemplate (anonymous)
        |
        +---- production correlation / evidence ledger
        |
        v
AuthorizedIdentityResolver  <-- research-only extension
        |
        v
ResearchMatchResult + uncertainty + audit record
```

## Required authorization object

A research resolver is expected to require an authorization object containing at minimum:

- authorizing organization;
- project or case reference;
- approved purpose;
- valid-from and valid-until timestamps;
- dataset or cohort scope;
- operator role;
- audit destination;
- explicit confirmation that the resolver is running in a research namespace.

A production configuration flag must never be sufficient to activate a resolver.

## Scientific gate

Operational consideration should require blinded, independently reproducible evidence that addresses:

- repeatability across days and months;
- cross-device and cross-location measurements;
- movement and stationary measurements;
- clothing and environmental changes;
- exertion, illness, medication, and ordinary physiological variability;
- sensor drift and calibration;
- open-set testing with people absent from the enrollment set;
- false-match rate and false-non-match rate;
- confidence calibration;
- subgroup performance;
- adversarial and replay testing.

Similarity is not identity. A research score must always retain uncertainty.

## Non-suspect protection

The intended architecture does not silently enroll passersby. Production observations are anonymous and retention-limited. Any future identity-capable research program must define its lawful enrollment source, authorization boundary, deletion process, access controls, audit process, and mechanism for correcting erroneous associations.

## Permitted repository content

This directory may contain:

- interfaces;
- synthetic template generators;
- blinded evaluation harnesses;
- statistical metrics;
- repeatability experiments;
- protocol templates;
- threat models;
- documentation.

It must not contain a turnkey covert population-identification database or a production module that silently maps arbitrary physiological observations to civilian identities.
