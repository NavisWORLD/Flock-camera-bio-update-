# Security Policy

## Supported branch

Security fixes for the current engineering prototype are developed against `main` and active release branches after review.

## Reporting a vulnerability

Do not post exploitable vulnerabilities, credentials, personal data, or live public-safety data in a public issue.

Preferred reporting path:

1. Use GitHub's private vulnerability reporting / Security Advisory workflow for this repository when enabled.
2. If a government or commercial evaluation is operating under a signed agreement, use the security contact and incident channel defined in that agreement.
3. Include the affected version/commit, reproduction conditions, expected impact, and a minimal proof that does not expose real victim or investigative data.

## Sensitive classes

Please report privately if you discover:

- authentication or authorization bypass;
- evidence-ledger forgery or verification bypass;
- unsafe FFI memory behavior;
- secret/key disclosure;
- unintended person-identity linkage;
- research-mode isolation bypass;
- retention/deletion bypass;
- audit-log bypass or tampering;
- remote code execution or dependency compromise;
- a path that converts mask/occlusion metadata into an unsupported identity or guilt decision.

## Development security expectations

- Never commit real Flock credentials, government credentials, signing keys, private keys, or live case data.
- Use synthetic fixtures for development unless a separate authorized test environment exists.
- Keep real integration secrets in an approved secret manager.
- Use protected review/CI gates before a production release.
- Treat the reference bearer token as development/demo authentication only; production deployments require purchaser-approved IAM and TLS/mTLS architecture.
- Treat the reference Ed25519 implementation as an evidence-integrity example, not a claim of FIPS-validated cryptography.

## Response process

Security reports should be triaged for exploitability, affected versions, data exposure, and required containment. A production operator should follow its own incident-response plan, notification duties, key rotation/revocation procedures, evidence-preservation requirements, and legal/policy obligations.
