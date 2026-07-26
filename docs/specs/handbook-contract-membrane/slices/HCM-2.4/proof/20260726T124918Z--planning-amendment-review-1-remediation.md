# HCM-2.4 planning amendment review 1 remediation

Recorded: 2026-07-26T12:49:18Z
Reviewer: `/root/hcm_2_4_amendment_review_1`
Reviewed fingerprint:
`sha256:86a23488bb5c6556f38bc14dcb43a115371e424aa2588cc4ffc4544552711d92`
Dispatch:
`20260726T123404Z--HCM-2-4--planning-amendment-discovery-review`

The fresh isolated reviewer replayed the four-entry manifest, independently
reproduced the live P0 stop and GitNexus impacts, and returned two required P2
findings with no advisory P3/P4.

## HCM-2.4-AMEND-REV1-001

Accepted. The first subject required `evaluate_charter_intake` to validate the
compatibility membrane while also forbidding any new production caller of
`validate_selected_decisions`. The discovery review reported five existing
callers; the different-fresh closure review then proved that compiler
`doctor_report_from_inspection` was an omitted sixth existing caller. The
required fail-closed intake edge is therefore the seventh.

Remediation:

- inventory all six existing callers, including compiler
  `doctor_report_from_inspection` as a read/proof-only HIGH anchor;
- explicitly authorize exactly one seventh caller from
  `evaluate_charter_intake`;
- retain all recorded process/module ceilings and stop on an eighth production
  caller or any new process/module/authority class/public surface;
- name the exact new integration target
  `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs`; and
- require
  `invalid_compatible_profile_decisions_cannot_produce_charter_intake` to prove
  an invalid 1.2 tuple refuses before any intake/candidate evaluation.

## HCM-2.4-AMEND-REV1-002

Accepted. `author_help_matches_snapshot` is an inline assertion and does not
consume `handbook-author-help.txt`. Only
`author_environment_inventory_help_matches_snapshot` consumes a snapshot for
the affected command help.

Remediation:

- retain `crates/cli/src/main.rs` exact
  `AuthorCommand::EnvironmentInventory` help text;
- retain both exact `cli_surface.rs` tests, identifying the first as inline and
  the second as snapshot-consuming; and
- retain only the live-consumed
  `crates/cli/tests/snapshots/handbook-author-environment-inventory-help.txt`.

No command rename, argument change, additional snapshot wiring, or broader P2
surface is authorized.

## Supplemental causal correction

Different-fresh reviewer `/root/hcm_2_4_amendment_review_2` established the
omitted compiler doctor edge and returned
`HCM-2.4-AMEND-REV1-001` still open. The correction above supersedes only the
caller ordinal/manifest: six existing, intake as the sole seventh, eighth RED.
The exact compatibility design and discovery finding remain otherwise
unchanged.

## Review disposition

Both valid P2 findings are consolidated in this one remediation. The amended
SPEC, plan, checklist, and causal proof are synchronized. Because this is
material remediation, a different-fresh read-only closure review is required
against a new complete-subject fingerprint. No implementation was performed.
