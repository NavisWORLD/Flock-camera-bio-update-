# Security Policy

## Reporting a vulnerability

Do not publish credentials, exploitable security details, sensitive camera/provider data, or personally identifying information in a public issue.

Use the repository's private GitHub security-advisory reporting channel when available, or contact the repository owner through an established private channel before public disclosure.

Include:

- affected commit/version;
- component and deployment mode;
- reproduction steps using synthetic/non-sensitive data where possible;
- impact;
- proposed mitigation if known.

## High-priority findings

Please treat these as high priority:

- authentication bypass;
- unauthorized research-route access;
- hidden person-identity resolution in production paths;
- signing-key or provider credential exposure;
- evidence-chain verification bypass;
- remote code execution;
- privilege escalation;
- retention/audit bypass;
- cross-tenant or cross-deployment data exposure.

## Sensitive research

Reports involving experimental physiological or biomagnetic identification must not include real-person biometric datasets unless the reporter is authorized to share them. Prefer synthetic reproductions.

## Release status

A source branch, draft pull request, or development artifact is not a production security certification. Production deployments require deployment-specific review in addition to repository-level testing.
