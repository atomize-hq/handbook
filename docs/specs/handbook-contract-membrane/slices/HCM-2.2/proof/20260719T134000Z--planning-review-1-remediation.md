# HCM-2.2 Planning Review 1 Remediation

## Review identity

- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T131536Z--HCM-2-2--fresh-planning-review-1.json`
- Raw dispatch SHA-256:
  `ec4940a16aa92e13b7fec5ec9473368fe50fc378ef4913f47724263665892c60`
- Reviewed subject fingerprint:
  `sha256:361550eb2c7527f9cb397a12bcb5dbd1844565243f2d45b37b0829a2138368b8`
- Fresh reviewer: `/root/hcm_2_2_planning_review_1`
- Built-in status: completed read-only review
- Verdict: `CHANGES_REQUIRED`
- Findings: six Required; no Critical, Optional, or Nit findings

Review 1 replayed all seven subject entries and the aggregate fingerprint, read
the current control/definition/source truth, passed its bounded checks, and made
no repository change. The verdict did not authorize a commit or implementation.

## Parent validation and remediation

### R1 — forbidden kind-to-intake closure

**Disposition:** valid.

`05` explicitly makes intake compatibility one-way and excludes intake from the
kind fingerprint. The packet now keeps intake out of the `1.1.0` kind, has the
intake target the kind, and selects intake only through the Project Authority
descriptor. Definition proof now requires acyclic closure, kind-field refusal,
kind fingerprint independence, and descriptor-only selection.

### R2 — retained rich Charter semantic loss

**Disposition:** valid.

The first draft collapsed live project constraints, operational reality,
baseline/rationale, domains, and dimension details without an authority
decision. The canonical schema now retains every validated
`CharterStructuredInput@0.1.0` carrier: exact project enums and nested records,
posture baseline/rationale, domains, per-dimension level/stance/triggers/
shortcuts/red-lines/domain overrides, exceptions, debt, and decision records.
It also retains the capability-bound `1.0` policy/governance/posture fields.
Deletion is gated by a field-by-field preservation crosswalk and positive/
negative validation-value proof.

### R3 — insufficient reused semantic validator

**Disposition:** valid.

The released `1.0.0` validator only proves coarse binding types and remains
immutable. The packet now adds
`handbook.semantic-validation.constitutional-root@1.1.0`, selected by kind
`1.1.0`, with exact nine-dimension order/set, level, baseline, posture,
governance, and retained validation rules. Missing, extra, duplicate,
substituted, and reordered IDs plus new closure/old-byte tests are mandatory.

### R4 — subordinate definitions and approval bootstrap underspecified

**Disposition:** valid.

The packet now freezes closed type-specific semantic-validator, renderer,
lifecycle, approval, waiver, trigger, and intake definition matrices and their
acyclic fingerprint inputs. It enumerates the ten waivable coverage IDs and
waiver conditions. Approval now names an honest local trust root: explicit
non-agent repository-operator attestation, engine-derived actor/authority
bindings, exact bootstrap authority when the target is absent, and current-
Charter authority/approval-class resolution for amendments. Caller-supplied
actor/authority identities refuse, and JSON explicitly disclaims external
identity assurance. Bootstrap, forgery, stale authority, rejection, waiver,
and definition-substitution matrices are required.

### R5 — unrealizable atomic-promotion claim

**Disposition:** valid.

The packet replaces “one recoverable transaction” with an exact same-filesystem
promotion protocol: shared reader/writer lock and pre-read recovery; pending
journal; fsynced intent, staged bytes, and old-byte backup; ordered canonical
and record renames; fsynced install markers; final commit marker as the
authority point; committed-directory closeout; deterministic rollback/
roll-forward; and fail-closed disagreement. Fault injection covers every
persistence boundary, concurrent readers/writers, replay, and native-Windows
pre-delta refusal unless equivalent primitives are proven.

### R6 — incomplete canonical/render byte contracts

**Disposition:** valid.

The packet now freezes recursive canonical YAML object/sequence/scalar grammar,
field order, indentation, escaping, scalar encodings, forbidden forms, UTF-8,
and final LF. It freezes the Markdown heading, table, section, list, empty-
value, normalization, escaping, punctuation, and final-LF matrix. Two complete
literal planning-authority fixtures were added:

- `contracts/canonical-charter-boundary-v1.1.yaml`; and
- `contracts/charter-review-boundary-v1.0.md`.

Production plus independent reference implementations must reproduce the raw
fixture bytes and cover empty/boundary/control/Unicode/scalar cases.

## Re-review requirement

Review 1 is immutable historical evidence. The parent must replay the complete
planning proof over the remediated subject, compute a new sorted manifest and
aggregate fingerprint, create a new immutable dispatch, and use a different
fresh isolated read-only reviewer. No Review 1 conclusion is inherited.
