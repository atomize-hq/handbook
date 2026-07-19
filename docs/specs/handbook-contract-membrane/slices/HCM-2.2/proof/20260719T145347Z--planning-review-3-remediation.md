# HCM-2.2 Planning Review 3 Remediation

## Review identity

- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T141402Z--HCM-2-2--fresh-planning-review-3.json`
- Raw dispatch SHA-256:
  `c6ae4e621615ad70851e4b4e7871c04befcf53ff9c43704c703c5d645f9b5a8d`
- Reviewed subject fingerprint:
  `sha256:afa179b58089ad0493401dc4b8b57f457a261105b1184e5acdb37105ab172438`
- Fresh reviewer: `/root/hcm_2_2_planning_review_3`
- Built-in status: completed read-only review
- Verdict: `CHANGES_REQUIRED`
- Findings: six Required; no Critical, Optional, or Nit findings

Review 3 reproduced all thirteen subject entries, the aggregate fingerprint,
handoff validation, links, fixture hashes, literal key order, sixteen-row
coverage non-overlap, retained schema test, and planning-only scope. It made no
repository change. Its verdict authorized neither implementation nor commit.

## Parent validation and remediation

### R1 — amendment basis absent from immutable lineage

**Disposition:** valid.

The packet now adds exact `1.1` intake, candidate, approval, and promotion
record shapes with required `basis_artifact_fingerprint`. Null is initial-create
only; amendments require one non-null value from intake finalization onward.
Candidate, approval, current observation, CLI expected-current, intent,
promotion, retry, recovery, and lifecycle triggering must all match. Released
`1.0` record schemas remain unchanged and cannot carry HCM-2.2 lineage.

### R2 — subordinate definitions not byte-closed

**Disposition:** valid.

The packet now fixes the complete profile source lists and chooses semantic-
validator record schema `1.1`. It adds four exact closed schema documents, one
complete definition/profile closure-vector file, and eleven literal RFC 8785
JCS preimage lines. Every exact value, nested shape, order, dependency closure,
byte length, and expected fingerprint is planning authority. Independent
reproduction now has fixed target fingerprints for schema entries, validator,
approval, waiver, triggers, renderer, lifecycle, kind, intake, and profile.

### R3 — incomplete sixteen-row intake values

**Disposition:** valid.

The packet now includes the complete literal
`charter-intake-definition-1.0.0.yaml` record. Every row fixes all acquisition,
evaluation, waiver, and prompt-guidance fields in one order. Baseline uses
`minimum_specificity: explicit_levels` and is non-waivable. Debt is a homogeneous
observational row with evidenced inference admitted and no required declaration.
The record exactly equals the intake definition in the closure vector and
reproduces fingerprint
`sha256:77bb9b13195793fef6e637d60e2878329fbf41438dc12a5a444be4b5b253df00`.

### R4 — non-agent approval was unenforceable

**Disposition:** valid.

The file/stdin confirmation design is removed. Approval and registry mutation
now require a registered platform authenticator, fresh OS-CSPRNG challenge,
hardware-backed signature, user presence, and user verification. Exact closed
registry/assertion/transition schemas, challenge keys, algorithms, storage,
bootstrap, current-promotion-anchored transition chain, administration,
revocation, replay/counter checks, and fail-before-delta native unavailability
are frozen. There is no confirmation-string, `--yes`, stdin,
file, environment, recovery-code, software-signature, or unattended fallback.
The assurance claim is deliberately bounded to control of a repository-
registered user-verified authenticator, not external identity or employment.

### R5 — approval class lacked durable binding

**Disposition:** valid.

Additive approval schema `1.1` now carries exact `approval_class`, basis,
authenticator assertion ref/fingerprint, and accepted waiver refs. Authority and
class derive from current Charter plus the signed registry mapping. Promotion
requires one fresh approved record per distinct current required class, sorted
lexically; duplicates count once and never cover another class. A credential
may cover multiple mapped classes only through separate assertions/records.

### R6 — promotion recovery was not total

**Disposition:** valid.

The physical both-or-neither claim is replaced by one authority-visibility
commit point. Final immutable records use create-new/no-follow or preexisting-
equal reuse and never overwrite. A total observed target/final-record/commit-
marker table now covers pre-intent, post-canonical/pre-marker, partial-record,
preexisting-equal, pre-commit, committed, cleanup, replay, and disagreement
states. Progress markers are diagnostic; intent plus exact bytes govern.
Reader locks and committed-journal filtering hide every pre-commit physical
partial state. Exhaustive injection remains required on every persistence
boundary, with native mutation refusal where equivalent primitives are absent.

## Re-review requirement

Review 3 remains immutable findings evidence. The parent must replay the full
planning wall over the expanded exact-byte subject, compute a new sorted
manifest and aggregate, write a new immutable dispatch, and use a fourth
different fresh isolated read-only reviewer. No prior conclusion is inherited.
