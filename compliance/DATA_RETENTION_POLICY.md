# Default Data Retention Policy

## Principles

The platform defaults to minimization. Retention is based on data purpose rather than indefinite collection.

## Default development/production policy

- Raw sensor samples: disabled unless explicitly enabled for an authorized collection.
- Ephemeral observation windows: 24 hours by default when persistence is enabled.
- Anonymous derived templates: 30 days by default.
- Safety events: organization-defined based on the documented operational purpose.
- Audit events: retained according to the deploying organization's security and records policy.
- Evidentiary exports: retained according to the associated investigation/case and applicable records requirements.

## Holds and exceptions

Retention exceptions require a documented authorization or evidence-hold reference. The exception must be recorded as a policy/audit event and must not silently change system-wide defaults.

## Deletion

Deletion jobs identify the object type, object ID, scheduled time, completion time, and status. Deleting an operational object must not silently rewrite previously signed evidence: evidence retention and operational-data retention are separate policy decisions.

## Research data

Research datasets use a separate policy describing enrollment/cohort authority, research purpose, expiration, access, and deletion. Enabling research mode does not automatically extend production retention.
