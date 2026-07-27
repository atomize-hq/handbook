# HCM-2.4 P3B shared compile-golden selector repair

Status: **proposed exact two-golden amendment; implementation paused for fresh
review**

Date: 2026-07-27

## Trigger

After the reviewed P3B Stage 10 fixture synchronization and M5 canonical
consumer repair, the complete `handbook-cli --test cli_surface` wall reached
95/98. The three remaining failures are:

- `pipeline_compile_feature_spec_payload_matches_shared_golden`;
- `pipeline_compile_ignores_unrelated_malformed_stage_files`; and
- `pipeline_compile_feature_spec_explain_matches_shared_golden`.

The first two compare the same payload golden and the third compares the paired
explain golden. Actual output is the reviewed P3 Work Specification compile
contract from the synchronized fixture core; expected bytes still describe the
superseded Markdown-only output.

## Exact additive selector

Add exactly:

- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.payload.full_context.txt`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.explain.full_context.txt`

Both files must be mechanically regenerated from the unchanged compile command,
fixed `HANDBOOK_PIPELINE_COMPILE_NOW_UTC`, synchronized P3 fixture core, and
existing path normalization. No assertion, helper, production, or fixture input
edit is selected by this amendment.

## Boundary and exit proof

These shared golden bytes are consumed by the exact three CLI tests above and
the existing compiler and pipeline `pipeline_compile` regression families.
Read/proof-only direct consumers are:

- `crates/compiler/tests/pipeline_compile.rs`
- `crates/pipeline/tests/pipeline_compile.rs`

The update must make all three harnesses agree on the same Work Specification
outputs and must preserve the malformed-unrelated-stage negative.

## Review finding remediation

Discovery finding
`p3b-shared-compile-golden-selector-repair-discovery-1-P2-1` correctly
identified the omitted pipeline test consumer. That path is now an explicit
read/proof-only consumer and remains uneditable. The implementation selector is
still exactly the two golden files.

No runtime, compiler/CLI source, schema, released definition, profile, shared
fixture input, public API, dependency, Cargo/version, unsafe policy, or sibling
slice is selected. Entry requires fresh built-in review. Exit remains all 98
`cli_surface` tests, complete compiler and pipeline `pipeline_compile` tests, P3
packet wall, workspace wall, formatting, clippy, archive boundary, diff checks,
and different-fresh implementation review.
