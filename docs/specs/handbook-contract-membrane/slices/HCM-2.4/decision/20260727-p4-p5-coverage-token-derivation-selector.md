# P4/P5 coverage-token derivation selector

Status: operator-approved selector; selector review CLEAN and RED/GREEN
implementation complete; supplemental causal closure pending.

Date: 2026-07-27

## Trigger and authority

The P4 and P5 fixture profiles, definitions, coverage, renderer proof, and
available canonical read/validate proof were already review-clean. P5's
negative-surface proof was complete; P4's separate no-root, generated-command,
inferred-filename, Projection, and persistent-view proof remained open outside
this selector. At the selected baseline, positive generic mutation stopped
before establishment because the unchanged planner copied underscore-bearing
final coverage-ID segments directly into an otherwise unchanged lineage
output-token grammar. The operator approved only the selector below while
resuming from
`20260727T070426Z--HCM-2-4--orchestration--authority-prerequisites-required`.

This decision supersedes only the implementation recommendation in
`20260727-p4-p5-generic-mutation-token-authority-stop.md`. That stop remains
historical evidence for the reproduced refusal and its preserved boundaries.

## Exact production selector

Edit only
`Function:crates/engine/src/artifact_mutation.rs:intake_commit_plan`.

For each satisfied coverage result:

1. select only the final segment after the last ASCII `.` in its released
   coverage ID, exactly as the current implementation does;
2. replace each ASCII `_` in that final segment with ASCII `-`;
3. append the exact suffix `-value`.

No other character, case, Unicode, separator, segment, prefix, or suffix
normalization is allowed. The existing absence refusal, token grammar,
collision/uniqueness fallback, distinct-value suppression, output limit,
lineage-store validation, authority classes, references, fingerprints, and
install/receipt behavior remain unchanged. Released coverage IDs, schemas,
kind/intake/renderer/profile definitions, and their fingerprints remain
byte-identical. No helper, public API, dependency, new module, or second
production symbol is authorized.

The implementation is one expression-local derivation change inside the
existing loop. If correctness requires another production symbol or broader
schema/coverage inference, stop.

## Exact RED/GREEN proof selector

The selector authorized edits to exactly four existing test functions. The two
blocker-specific functions were renamed to their current proof names:

- `crates/engine/tests/hcm_2_4_decision_record.rs::
  generic_decision_record_mutation_derives_exact_coverage_tokens`;
- `crates/engine/tests/hcm_2_4_decision_record.rs::
  generic_decision_record_mutation_retains_real_bytes_and_rejects_stale_basis`;
- `crates/engine/tests/hcm_2_4_risk_record.rs::
  generic_risk_record_mutation_derives_exact_coverage_tokens`; and
- `crates/engine/tests/hcm_2_4_risk_record.rs::
  generic_mutation_promotes_then_reads_and_validates_new_real_bytes`.

Convert each blocker-specific refusal test into a successful intake proof that
reads its committed intake transaction intent and asserts the complete ordered
token list. Rename those two functions to describe exact token derivation. The
Decision list is:

1. `schema-id-value`;
2. `schema-version-value`;
3. `record-id-value`;
4. `context-value`;
5. `decision-value`;
6. `status-value`;
7. `consequences-value`;
8. `supersedes-value`;
9. `intake-record`.

The Risk list is:

1. `schema-id-value`;
2. `schema-version-value`;
3. `record-id-value`;
4. `uncertainty-value`;
5. `evidence-refs-value`;
6. `owner-value`;
7. `treatment-value`;
8. `status-value`;
9. `review-basis-value`;
10. `intake-record`.

Remove `#[ignore]` only from the two named positive tests. Preserve their
existing generic candidate, promotion, canonical-byte, stale-basis, read, and
validate assertions. Preserve every unrelated negative case in both targets,
including coverage, schema, selection, surface, stale-basis, and lineage-store
refusals.

RED is the four selected active tests failing at the reproduced lineage-store
token refusal before the production edit. GREEN is those same four tests
passing after the exact derivation edit, followed by both complete integration
targets and the applicable HCM-2.3 generic-lineage preservation wall.

## Impact and reviewability ceiling

Fresh upstream impact at commit `fb70b71bc758677a88ad33380f3e46c65051b2fb`
for `intake_commit_plan` is LOW: 3 impacted symbols, 1 direct caller, 0 indexed
processes, and 2 modules. The direct caller is `build_intake_plan`; the two
transitive impacts are `intake_append` and the CLI artifact execution path.
Each of the four selected test functions is LOW with no indexed upstream
impact.

These counts are hard ceilings. Stop on wider impact, another production
symbol, public/API/schema/dependency/Cargo/version change, released-definition
byte change, token-grammar change, or a required edit outside the three named
files and slice-local control/proof records.

P4 and P5 are one atomic production packet because both released intakes
converge through the same single derivation expression and the two separate
test targets independently prove their exact token sets and end-to-end paths.
Separate fixtures, assertions, and proof evidence are retained.

## Non-goals and exit

This selector does not authorize P2 runtime work, P6, bridge deletion, generic
token normalization, coverage-ID edits, schema inference, or another artifact
family. P2 may receive only a separate review-clean planning amendment before
returning for human runtime approval.

The selector exits only after a fresh built-in read-only review returns no
unresolved P1/P2 finding. Implementation exits only after targeted RED/GREEN,
packet proof, fresh independent implementation review, any required
remediation and different-fresh closure, GitNexus change detection, and a
scoped reviewed commit.
