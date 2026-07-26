# HCM-2.4 planning discovery remediation

Captured: `2026-07-26T04:47:02Z`

Discovery run: `/root/hcm24_planning_discovery_review`

Reviewed fingerprint:
`sha256:a49bb62cf5826d819fe94cdf29003e4dac0ca07c88a2d8a7f60b7e3957bbd57f`

Verdict: FINDINGS, two consolidated P2 findings, no P1/P3/P4.

## HCM-2.4-DISC-001 — cyclic implementation entry

Demonstrated failure: the reviewed SPEC required P0's exact live manifests and
selectors before implementation selection, while the plan allowed P0 only
after selection.

Remediation:

- separate implementation selection now authorizes read-only P0 preflight only;
- no production, test, definition, template, or documentation edit is
  authorized until P0 freezes and passes the exact live manifest/UID gate; and
- implementation-entry criteria now require the reviewed selector ceilings and
  make P0 responsible for exact live manifests after selection and before edits.

This removes the cycle without beginning implementation or weakening the entry
gate.

## HCM-2.4-DISC-002 — unresolved descriptor bindings

Demonstrated failure: the reviewed packet could publish definitions while
leaving selected Project Context and Environment Context intake/renderer fields
null or empty, and it named a fixed Work Specification path without freezing an
admitted repository-profile descriptor.

Remediation:

- froze all three shipped-root 1.2 instance rows, including exact kind, intake,
  renderer, path, requiredness, and empty Projection requirements;
- made Project Context and Environment Context non-null exact intake refs and
  singleton exact renderer refs explicit P1A/P1B RED/GREEN checks;
- added the exact descriptor dependency guard surface in
  `artifact_instance.rs`;
- froze the Work Specification fixture root, profile-selection path, repository
  profile source/ref, extension base, instance id, kind, role, canonical path,
  intake, renderer, requiredness, and empty later-owned fields; and
- required Stage 10 to resolve the admitted descriptor before capture,
  fingerprint, render, or handoff, with an explicit negative test that
  same-string hard-coding is insufficient.

The shipped-root membership remains exactly three. The Work Specification proof
uses a repository profile and does not add a default root instance, generated
command, dynamic path, Resolution input, or Projection.

## Scope and causal disposition

Both changes remediate the discovery findings directly. They do not add a new
artifact family, implementation surface category, public API, dependency,
Cargo/package boundary, unsafe policy, or HCM-3.x authority. The remediation
remains planning-document-only and within the original risk ceiling.

The resulting complete subject requires a new fingerprint and one
different-fresh delta-focused closure review. No supplemental causal cycle has
been consumed.
