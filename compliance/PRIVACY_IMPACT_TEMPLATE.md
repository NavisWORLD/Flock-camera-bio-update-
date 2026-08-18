# Privacy Impact Assessment Template

This template is for a deploying agency, school-safety organization, laboratory, or integrator to complete before operational use.

## 1. System and owner

- System name: Flock Signal Safety Platform
- Deploying organization: **Complete before deployment**
- System owner: **Complete before deployment**
- Privacy/security contacts: **Complete before deployment**

## 2. Purpose limitation

Describe the specific safety or forensic purpose, the locations covered, the categories of authorized users, and the circumstances in which data may be queried or exported.

Mask or facial occlusion alone must not be defined as criminal conduct.

## 3. Data categories

Production data may include camera-event metadata, zone/time metadata, authorized sensor samples when configured, derived anonymous signal templates, safety-event reasons, quality/uncertainty metrics, cryptographic evidence metadata, and audit events.

The production schema does not contain a person-to-physiological-template identity table.

## 4. Non-suspect impact

Document how observations unrelated to an authorized safety purpose are minimized, expired, or deleted. State the default retention window and the process for approving an exception.

## 5. Notice and legal authority

The deploying organization must document applicable authority, notice requirements, contractual restrictions, and any warrant/court/administrative process relevant to its jurisdiction and use case. This repository does not supply legal authorization.

## 6. Access controls

Document operator roles, service accounts, evidence-export permissions, research permissions, key custody, access-review cadence, and offboarding procedures.

## 7. Retention

Record retention periods separately for raw observations, derived templates, safety events, audit events, and evidentiary exports. Raw signals are disabled by default in the product design.

## 8. Accuracy and human review

Describe signal-quality thresholds, uncertainty handling, false-alert review, operator training, and the rule that elevated events require human review rather than automatic guilt or identity conclusions.

## 9. Research boundary

If experimental physiological identity research is performed, document enrollment authority, participant/cohort scope, ethics/review process, authorization references, research namespace isolation, deletion policy, error metrics, and independent validation.

## 10. Audit and redress

Describe immutable audit review, incident investigation, correction of erroneous records, complaint/redress procedures where applicable, and evidence-chain verification.

## 11. Approval

Operational deployment should not proceed until the deploying organization has completed its legal, privacy, civil-rights/civil-liberties, security, records-management, and scientific-review requirements appropriate to the use case.
