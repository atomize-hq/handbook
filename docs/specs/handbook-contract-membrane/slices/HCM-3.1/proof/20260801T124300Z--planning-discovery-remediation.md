# HCM-3.1 Planning Discovery Remediation

Recorded at: 2026-08-01T12:43:00Z

Review run: `/root/hcm31_planning_review`
Dispatch:
`20260801T123319Z--HCM-3-1--vocabulary-resolution-planning-review`
Reviewed subject:
`sha256:3ab7d1ba51bb9e87ef0911a8b3b08db5d702ab18ab5acd5adda2bfde04da0672`
Verdict: `FINDINGS`

The reviewer was fresh, built-in, read-only, and changed no files.

## Consolidated disposition

### HCM31-PLN-DISC-001 — P1 — remediated

The selector incorrectly added one LF before hashing while also requiring the
immutable shipped fingerprint. HCM-3.1 now retains the existing uniform RFC 8785
producer with no appended delimiter. The spec freezes both exact canonical
preimages and their hashes:

- shipped empty:
  `sha256:69113b1a9271ce207d45bdb91ebae8d6516249e16b59891b292546078364a22b`;
- non-empty HCM-3.1:
  `sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4`.

### HCM31-PLN-DISC-002 — P2 — remediated

The production ceiling now names the actual selected consumer at
`crates/pipeline/src/pipeline_capture.rs`. The exact profile integration path in
`crates/engine/src/profile_selection.rs` is also explicitly bounded so the
vocabulary can validate against the selected registry instance rather than an
ambient registry.

### HCM31-PLN-DISC-003 — P2 — remediated

The spec now freezes the non-empty Markdown bytes: heading rule, existing-body
preservation, section placement/omission, lexical order, unit and role line
grammar, controlled escaping, edge-count assertion, and terminal LF. The exact
non-empty suffix is included as the RED-test authority.

## Remaining gate

No Rust/runtime edit is permitted until a different fresh read-only reviewer
closes all three finding IDs for the remediated subject. A closure finding that
is caused or unmasked by this remediation uses the permitted planning
supplemental causal lineage; it does not create a new discovery cycle.
