# Data Retention Policy Baseline

> Deployment policy must be approved by the operating organization. These values are conservative engineering defaults/examples, not a statement of legal retention requirements.

## Default classes

| Data class | Default target | Notes |
|---|---:|---|
| transient raw sensor samples | disabled / memory-only | Persist only under separately authorized collection policy. |
| anonymous sensor observations | 24 hours | Extend only for documented safety event, validation study, or case retention. |
| anonymous signal templates | 30 days | Must remain unlinked to a civilian identity in the production schema. |
| normalized camera-event metadata | 30 days | Purchaser policy may require shorter retention. |
| safety events | 90 days | Human-reviewed events may transition to case/evidence retention. |
| signed forensic evidence | case/policy controlled | Legal hold and evidence policy override automatic expiry. |
| audit/policy-decision records | 1 year minimum engineering target | Final value set by agency policy and oversight requirements. |

## Rules

1. Collection must be limited to an approved purpose and deployment zone.
2. Expiry must run automatically; operators should not have to remember to delete routine data manually.
3. Legal holds must identify the authorizing case/reference and data scope.
4. Backups must age out consistently with the approved backup-retention schedule.
5. Deletion jobs must be auditable with counts, timestamps, and failure states.
6. Research datasets are governed separately from production retention.
7. Converting anonymous templates into long-lived named records is not part of the production retention model.

## Database support

The schema includes `expires_at` for derived templates and a `retention_jobs` table for auditable cleanup work. The first release does not yet include a background deletion worker; deploying organizations must not represent automatic deletion as active until that worker or equivalent database policy has been installed and verified.
