# Flock Integration Boundary

## Purpose

This repository is designed to be **Flock-compatible at an authorized integration boundary**, not to reverse engineer, scrape, or bypass Flock Safety systems.

The production adapter deliberately normalizes externally supplied events into an internal `ExternalCameraEvent` contract and does not embed undocumented Flock endpoints, credentials, or data schemas.

## Current official terms reviewed

Flock Safety's API and Integrations Terms are published at:

https://www.flocksafety.com/legal/api-integration-terms

As of the version reviewed for this repository update (last updated October 13, 2025), the terms state that API/integration access is customer-authorized and for bona fide law-enforcement purposes; access is limited to functions specified in Flock's API Developer Hub; and use is subject to applicable laws and the customer's authorization.

The same terms include restrictions relevant to this project, including prohibitions on using the Flock Implementation/Data for machine-learning model development or evaluation, reverse engineering, selling/sharing API access or Data except as authorized, and bulk/database-like extraction or circumvention of rate limits.

The terms and vendor documentation can change. A deployment must re-review the current agreement and Developer Hub at integration time.

## Engineering consequences

1. **No scraping.** This repository does not scrape Flock web interfaces or bulk-export Flock data.
2. **No guessed private endpoints.** Real integration code must be based on the current authorized API Developer Hub and customer-granted access.
3. **No committed credentials.** API keys/tokens belong in the purchaser's approved secret-management boundary.
4. **No Flock-data ML training/evaluation.** Synthetic and independently authorized sensor data are used for development and validation unless a separate written agreement expressly permits something else.
5. **On-demand/minimum necessary data.** Integrations should request only the fields/events required for the authorized public-safety workflow.
6. **Customer authorization record.** Production configuration should record the customer/integration authorization reference and permitted purpose.
7. **Data ownership/retention.** Retention, sharing, and deletion must follow the controlling customer agreement, API terms, applicable law, and agency policy.
8. **No resale of Flock Data.** Commercialization of this software does not convey a right to resell or redistribute Flock Data or Flock API access.

## Adapter contract

The internal adapter accepts:

```rust
pub struct ExternalCameraEvent {
    pub event_id: Uuid,
    pub camera_id: String,
    pub observed_at_ns: i128,
    pub zone_id: Option<String>,
    pub event_kind: String,
    pub attributes: BTreeMap<String, String>,
    pub source_uri: Option<String>,
}
```

A purchaser/vendor integration module is responsible for converting an **authorized** Flock API response into this contract. The rest of the signal platform does not need to know or duplicate Flock's proprietary internal implementation.

## Biomagnetic/physiological sensor separation

Flock camera events and independently deployed physical sensors are separate data sources. Nothing in this repository assumes an ordinary Flock optical camera measures a human magnetic field.

A future biomagnetic or physiological research sensor must have its own lawful collection authority, hardware validation, calibration, noise characterization, and scientific study protocol. Flock API data must not be repurposed for prohibited ML evaluation or training.

## Procurement statement

A buyer can evaluate this repository without a Flock credential using the synthetic simulator. A live Flock integration is a separate customer-authorized deployment step and must be validated against the then-current Flock Developer Hub, contractual terms, rate limits, security requirements, and data-use restrictions.
