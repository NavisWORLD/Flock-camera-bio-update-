# Privacy Impact Assessment Template

> This is an engineering template, not legal advice or a completed agency Privacy Impact Assessment. The deploying organization must complete it against its laws, policies, mission, jurisdiction, and system boundary.

## 1. System and mission

- Deploying organization:
- System owner:
- Mission/business purpose:
- Authorized locations/zones:
- Operational dates:
- Data controller/custodian:
- Legal/policy authority identified by agency counsel:

## 2. Data inventory

For each data type document source, purpose, retention, access, and whether it can identify a person.

### Production data types

- camera event ID, camera ID, timestamp, zone, event kind;
- contextual attributes such as `face_occlusion=present|absent|unknown`;
- sensor observation IDs and provenance;
- anonymous signal feature vectors;
- signal quality and uncertainty;
- source digests;
- safety-event rules/rationale;
- signed evidence-ledger records;
- operator/audit metadata.

### Data not required by the production schema

- civilian name linked to an involuntary signal template;
- government identifier linked to an involuntary signal template;
- face embedding linked to a signal template;
- protected-trait inference;
- raw physiological sample retention by default.

## 3. Purpose limitation

Describe the exact public-safety purpose. Define uses that are prohibited. Document whether the system is limited to restricted zones, specific incidents, time windows, or case references.

Mask/face occlusion must be documented as contextual visibility information only. It is not independent evidence of criminal intent or identity.

## 4. Non-suspect minimization

Document:

- how anonymous observations are assigned random IDs;
- how unrelated observations expire;
- how raw data is disabled/minimized;
- how identity fields are kept outside the production template schema;
- how operators are prevented from bulk identity resolution;
- how false associations are corrected;
- how complaints/appeals are handled where applicable;
- how audits identify inappropriate access.

## 5. Access controls

List roles and permitted operations:

- system administrator;
- investigator;
- evidence auditor;
- maintenance operator;
- research operator, if separately approved.

For each role define authentication method, approval path, searchable fields, export permissions, and logging requirements.

## 6. Retention and deletion

Specify retention separately for:

- transient/raw samples;
- anonymous observations;
- derived signal templates;
- camera events;
- safety events;
- evidence retained for an active case;
- audit records.

Document legal holds, automatic expiry, deletion verification, backup expiry, and exceptions.

## 7. Research boundary

If physiological identity research is proposed, document it as a separate activity. Identify oversight/ethics process, enrollment/collection authority, participant population, dataset governance, matching metrics, bias analysis, open-set evaluation, and criteria for stopping the study.

Production deployment must not be used as an undeclared substitute for an identity-research protocol.

## 8. Data sharing

List every external recipient/system and the minimum fields shared. Define contractual/policy authority, encryption, retention, redisclosure rules, and audit mechanisms.

## 9. Accuracy and human review

Document expected sensor error, feature uncertainty, correlation uncertainty, false-positive management, operator training, and the decision points requiring human review.

The application is not designed to adjudicate guilt.

## 10. Security controls

Document production IAM, encryption in transit/at rest, KMS/HSM, network segmentation, log protection, incident response, vulnerability management, backups, disaster recovery, and supply-chain controls.

## 11. Transparency and oversight

Document required public notice, agency policy publication, audit cadence, independent oversight, records request handling, and any jurisdiction-specific transparency obligations.

## 12. Approval record

- Privacy officer:
- Security officer:
- Civil rights/civil liberties reviewer where applicable:
- Agency counsel:
- System owner:
- Approval date:
- Review/expiration date:
