# Gateway API Contract

Base URL in the default configuration: `http://HOST:8080`.

There is no production person-identity lookup endpoint.

## Authentication

`GET /healthz` and `GET /readyz` are public health endpoints.

Every `/v1/*` route requires:

```text
Authorization: Bearer <FLOCK_SIGNAL_API_KEY>
```

If `FLOCK_SIGNAL_API_KEY` is not configured, all `/v1/*` requests are denied. Configure the secret outside source control through the deployment environment or secret manager.

The gateway also adds an `x-request-id` response header for request correlation.

## Health

### `GET /healthz`
### `GET /readyz`

```json
{
  "status": "ok",
  "service": "flock-signal-gateway"
}
```

## Normalize an authorized camera event

### `POST /v1/camera/normalize`

```json
{
  "camera_id": "flock-sim-17",
  "observed_at_ns": 1724012345000000000,
  "zone_id": "school-north",
  "event_kind": "vehicle_or_person_observation",
  "attributes": {
    "face_occlusion": "present",
    "restricted_zone": "true",
    "after_hours": "true"
  },
  "source_uri": "fixture://sample-camera-event"
}
```

`face_occlusion` is normalized to `present`, `absent`, or `unknown`. This field alone does not create an elevated safety decision.

## Extract an anonymous signal template

### `POST /v1/signal/extract`

The body is a serialized `ObservationWindow` containing authorized sensor frames. The response is an anonymous `SignalTemplate` containing a random UUID, versioned feature vector, quality, uncertainty, and SHA-256 source digest.

The response contains no name, government ID, face embedding, or production person-record key.

## Evaluate policy

### `POST /v1/policy/evaluate`

```json
{
  "action": "restricted_zone_review",
  "context": {
    "operator_role": "school-safety-reviewer",
    "export_authorized": false,
    "research_authorized": false,
    "restricted_zone": true,
    "after_hours": true,
    "face_occlusion": true,
    "authorization_ref": null
  }
}
```

A contextual elevation requires the independent restricted-zone and after-hours conditions in addition to occlusion. Elevated safety results require human review.

## Correlate a camera event and anonymous observation

### `POST /v1/correlate`

Request fields:

- `camera_event`: normalized `CameraEvent`;
- `observation_window`: authorized anonymous `ObservationWindow`;
- `signal_template`: derived anonymous `SignalTemplate`.

The correlator requires compatible time and zone context. The response is a `SafetyEvent`; it is not an identity or guilt determination.

## Verify signed forensic evidence

### `POST /v1/evidence/verify`

Example request shape:

```json
{
  "public_key": [1, 2, 3],
  "records": []
}
```

`public_key` must contain exactly 32 byte values in the real request. `records` contains serialized `LedgerRecord` entries.

A valid response:

```json
{
  "valid": true,
  "record_count": 4
}
```

Verification checks record ordering, previous-record hashes, deterministic record digests, and Ed25519 signatures. The endpoint accepts a public verification key only; private signing-key custody remains outside the gateway.

## Error behavior

- Missing or invalid bearer key: `401 Unauthorized`.
- Malformed input: `400 Bad Request`.
- Valid input that fails correlation or evidence verification: `422 Unprocessable Entity`.

## Provider authorization

This API does not grant access to Flock Safety or another camera provider. A deploying integrator must separately obtain and configure whatever provider API access it is contractually and legally authorized to use.
