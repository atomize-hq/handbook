# HCM-2.4 P4 Decision Record implementation proof

Status: **GREEN — positive prerequisite and independent five-part
negative-surface/path proof are review-clean**

## Current bounded negative-proof candidate (2026-07-29)

The operator-authorized selector is
`decision/20260729-p4-decision-record-negative-proof-selector.md`. Fresh
selector reviewer `/root/p4_negative_selector_review` returned CLEAN from
dispatch
`20260729T164759Z--HCM-2-4--p4-negative-proof-selector-review` after replaying
all 18 subject hashes and the aggregate
`sha256:e29d121bceec54d3721feecfd1eab16a0cfe3de4be15ea62d00d253962a584f1`.
That result authorized exactly five additive Decision-specific negative tests
and one positive renderer replay in the existing P4 integration target.

The five forbidden surfaces are now independently executable:

1. `shipped_root_has_no_decision_record_instance_or_default` resolves the
   built-in shipped-root 1.2 inventory, proves neither `decision_record` nor
   `.handbook/records/decision.yaml` is present there, and proves the selected
   repository child adds exactly the one `decision_record` descriptor.
2. `author_command_inventory_has_no_decision_record_surface` inventories the
   authoritative `AuthorCommand` enum and consumed author-help snapshot,
   proves their only emitted commands are `charter` and `project-context`, and
   rejects identifier, label, kebab-case, and snake-case Decision spellings.
3. `missing_exact_decision_filename_refuses_inferred_and_dynamic_decoys`
   removes the exact canonical file, installs valid-byte
   `decision-record.yaml`, `decision_record.yaml`, and
   `example.record.decision.yaml` decoys, and receives the exact
   `ArtifactRead` refusal from generic repository read.
4. `decision_record_projection_widening_is_refused` proves the admitted
   descriptor has no projection refs, injects a capitalized Projection ref
   into a cloned Decision inventory, and proves the existing registry rejects
   that widened inventory.
5. `markdown_decoys_have_zero_decision_record_authority` proves no Markdown
   mirror exists or is emitted by generic read/validate, proves plausible
   Markdown decoys cannot change content or fingerprint, and proves removal of
   the canonical YAML still returns `ArtifactRead` rather than falling back.

`fixed_decision_renderer_golden_is_deterministic_and_resolution_free`
independently recomputes renderer closure fingerprint
`sha256:81f616aeecb1eac6c5c9207bcd2eb82c8e79ee159d57b97e2f15492a6d6a12b3`,
validates the frozen canonical input through the selected Decision schema,
proves `resolution_input: null` and zero external inputs, and reproduces the
152-byte Markdown SHA-256
`sha256:5296400baf82eda7d22c2d1d754af0c25081516b876a9bf511a87f210adf378e`.

Each five-part negative test passed independently with `--exact` (1 passed,
0 failed, 9 filtered for each). The positive renderer replay independently
passed with the same exact count. The complete live Decision target then
passed without ignored tests:

```text
cargo test -p handbook-engine --test hcm_2_4_decision_record
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The retained generic and wider preservation walls passed:

```text
cargo test -p handbook-engine --test hcm_2_3_generic_lineage -- --test-threads=1
test result: ok. 54 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p handbook-engine --test hcm_2_3_registration_kernel
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Strict Clippy passed for the engine library and the Decision integration
target. `cargo fmt --all -- --check` and `git diff --check` passed. GitNexus
upstream impact queries returned `UNKNOWN`/not found with zero impacted symbols
for the nine proposed new test-only helper/test names, as expected before
their addition; this packet edits no existing production function, class, or
method and changes no runtime behavior.

The same-subject implementation discovery burst used dispatches
`20260729T171423Z--HCM-2-4--p4-negative-proof-implementation-spec-review`
and
`20260729T171424Z--HCM-2-4--p4-negative-proof-implementation-standards-review`
over
`sha256:e81d9dd64c00a4cf0f707550035cc0bc03a0f94c5811e9cc7f0960da09676d34`.
The spec lens returned CLEAN. The standards lens returned one P2,
`P4-NEGATIVE-ROOT-INVENTORY-001`: the initial one-sided child-minus-parent
assertion did not prove preservation of every shipped-root descriptor. The
parent replaced it with exact set equality against shipped-root plus only
`decision_record`. The remediated focused test passed 1/1, the complete target
passed 10/10, strict target Clippy passed, and formatting/whitespace checks
passed. At that point, different-fresh closure review of the changed subject
was the only remaining acceptance wall before P4 could become GREEN.

Different-fresh reviewer `/root/p4_negative_impl_closure_review` then returned
CLEAN with no advisory from dispatch
`20260729T172756Z--HCM-2-4--p4-negative-proof-implementation-closure-review`
over
`sha256:840ac542df61453d18b6579e0c698d9ff9b383269d97ae69f27c9c0656ba5bf4`.
The reviewer reproduced all 22 hashes, confirmed the equality rejects both a
missing inherited descriptor and any extra descriptor, reran focused 1/1 and
complete 10/10 Decision tests, and closed
`P4-NEGATIVE-ROOT-INVENTORY-001`. The bounded P4 proof is GREEN. This result
does not authorize P6, P7, full HCM-2.4, Phase 2 exit, or any automatic
continuation.

The historical bounded stop was independently reproduced by
`/root/hcm_2_4_p4_stop_review` from dispatch
`20260727T004037Z--HCM-2-4--p4-decision-record-stop-review`. The reviewer
returned one Required runtime-prerequisite finding; P4 remained incomplete at
that baseline.

That historical stop was resolved only by the operator-approved selector in
`decision/20260727-p4-p5-coverage-token-derivation-selector.md`. Discovery
reviewer `/root/hcm_2_4_p4_p5_token_selector_review` returned one P2
cross-document finding and one P4 wording advisory. After parent remediation,
different-fresh reviewer `/root/hcm_2_4_p4_p5_token_selector_closure` returned
CLEAN over subject
`sha256:5f2909db89e6725bf36ba47e1f0965e1bfe51437d297e3234e4d4ad77919b506`.
Different-fresh supplemental causal reviewer
`/root/hcm_2_4_p4_p5_token_impl_supplemental_1` returned CLEAN over
`sha256:00c19024eeb94a2e93263489fd255f0ea47c3730bfc34da2e9cf6a054de1e5b9`;
the token prerequisite is committed at `00dde01`. At that historical
checkpoint, P4 remained incomplete because its no-root, generated-command,
inferred-filename, Projection, and persistent-view proof was still open.

## Authorized resolution and current proof

Only `artifact_mutation::intake_commit_plan` changed in production. It still
selects the final dot-delimited coverage-ID segment and appends `-value`, but
now replaces ASCII `_` with `-` inside that final segment first. The token
grammar, collision/uniqueness fallback, distinct-value suppression,
lineage-store validation, released coverage IDs, definitions, and fingerprints
are unchanged.

Before the production edit, the two selected active Decision tests established
RED at the reproduced Store boundary:

```text
cargo test -p handbook-engine --test hcm_2_4_decision_record
test result: FAILED. 2 passed; 2 failed; 0 ignored
generic_decision_record_mutation_derives_exact_coverage_tokens:
  generic lineage store refused: intake output tuple is not exact or unique
generic_decision_record_mutation_retains_real_bytes_and_rejects_stale_basis:
  generic lineage store refused: intake output tuple is not exact or unique
```

After the one-expression edit, the same complete target was GREEN:

```text
cargo test -p handbook-engine --test hcm_2_4_decision_record
test result: ok. 4 passed; 0 failed; 0 ignored
```

`generic_decision_record_mutation_derives_exact_coverage_tokens` reads the
committed intake transaction intent and proves the complete ordered tokens:
`schema-id-value`, `schema-version-value`, `record-id-value`,
`context-value`, `decision-value`, `status-value`,
`consequences-value`, `supersedes-value`, and `intake-record`. The activated
end-to-end test proves intake, candidate validation/append, promotion,
byte-exact canonical replacement, read/validate, and stale-basis refusal.

The affected generic-lineage preservation wall passed:

```text
cargo test -p handbook-engine --test hcm_2_3_generic_lineage
test result: ok. 54 passed; 0 failed; 0 ignored
```

`cargo fmt --all -- --check` and `git diff --check` passed. Strict Clippy
passed for the engine library and this Decision target. The all-target command
is blocked only by two pre-existing `needless_as_bytes` warnings whose exact
bytes are present at HEAD; neither intersects an authorized function.

Historical dispatch:
`20260727T001100Z--HCM-2-4--p4-decision-record-implementation`

Historical subject:
`sha256:778464084def625c25ec95f2a1268c1793ed71e9138ff213f363e9043ddd3161`

## Historical scope and baseline

The exact P4 path manifest controls this packet. The packet created only the
three named fixture files, the named integration target, and this proof record.
It did not edit production/library source, an existing test, a released
definition, Cargo metadata, a schema, the shipped-root profile, a command, or a
public API.

Before edits, every one of the 12 dispatch-manifest SHA-256 values matched.
Replaying the manifest in its ordinal path order reproduced the subject
fingerprint above. The subject hygiene check found no trailing whitespace.
The five exact P4 paths were untouched and did not overlap P5.

## RED proof

Command:

```text
CARGO_TARGET_DIR=target/hcm-2-4-p4 cargo test -p handbook-engine --test hcm_2_4_decision_record -- --exact repository_selected_decision_record_fixture_resolves_and_reads_real_bytes --nocapture
```

The command compiled the new target, ran one test, and failed because the exact
selection fixture did not exist:

```text
Decision Record selection fixture must exist: Os { code: 3, kind: NotFound,
message: "The system cannot find the path specified." }
test result: FAILED. 0 passed; 1 failed
```

## Historical successful bounded proof

The fixture selects
`example.profile.hcm-2-4-decision-record@1.0.0`, extends
`handbook.profile.shipped-root@1.2.0`, retains the shipped-root descriptor
closure required by replace-whole profile layering, and adds only the exact
`decision_record` row. The child typed-closure fingerprint is
`sha256:09517ce70b7563be1eba9065794c1839d9b3a5902b648a6e247e8e451fd913dd`.

Command:

```text
CARGO_TARGET_DIR=target/hcm-2-4-p4-p5-stop cargo test -p handbook-engine --test hcm_2_4_decision_record -- --nocapture
```

Result:

```text
running 4 tests
test generic_decision_record_mutation_refuses_invalid_coverage_token_derivation ... ok
test repository_selected_decision_record_fixture_resolves_and_reads_real_bytes ... ok
test decision_record_intake_modes_share_the_selected_schema_and_closed_coverage ... ok
test generic_decision_record_mutation_retains_real_bytes_and_rejects_stale_basis ... ignored
test result: ok. 3 passed; 0 failed; 1 ignored
```

These tests prove:

- exact repository selection and profile/source fingerprint closure;
- the exact kind 1.1, null role, safe canonical path, intake 1.0, singleton
  renderer 1.0, always requiredness, and empty later-owned descriptor fields;
- retained canonical YAML bytes, exact source fingerprint, safe generic read,
  stable second observation, and selected-schema validation;
- exact kind/schema/intake operation-context identity; and
- schema-backed coverage convergence for `guided_adaptive`, `express`, and
  `agent_assisted`, plus missing, duplicate, unknown, and wrong-typed coverage
  refusal.

The unchanged HCM-2.3 preservation target also remained green:

```text
CARGO_TARGET_DIR=target/hcm-2-4-p4 cargo test -p handbook-engine --test hcm_2_3_registration_kernel -- --nocapture
test result: ok. 16 passed; 0 failed
```

The existing positive generic promotion/recovery path remained green:

```text
CARGO_TARGET_DIR=target/hcm-2-4-p4 cargo test -p handbook-engine --test hcm_2_3_generic_lineage -- --exact ordinary_repository_read_recovers_markerless_installed_promotion_first --nocapture
test result: ok. 1 passed; 0 failed; 53 filtered out
```

## Historical stop-condition proof

Exact failing command:

```text
CARGO_TARGET_DIR=target/hcm-2-4-p4-p5-stop cargo test -p handbook-engine --test hcm_2_4_decision_record -- --ignored --exact generic_decision_record_mutation_retains_real_bytes_and_rejects_stale_basis --nocapture
```

Exact failure:

```text
generic intake append: ArtifactMutationErrorV1 {
  kind: Store,
  detail: "generic lineage store refused: intake output tuple is not exact or unique"
}
test result: FAILED. 0 passed; 1 failed; 3 filtered out
```

This is an unchanged generic HCM-2.3 runtime gap, not a fixture or schema
failure:

1. `artifact_mutation::intake_commit_plan` derives subordinate output tokens
   from each coverage-ID suffix and appends `-value`
   (`crates/engine/src/artifact_mutation.rs:1593`).
2. The exact Decision Record coverage contains the required suffixes
   `schema_id`, `schema_version`, and `record_id`, producing tokens such as
   `schema_id-value`.
3. `artifact_lineage_store::valid_intake_value_token` accepts only lowercase
   ASCII letters, digits, and hyphens
   (`crates/engine/src/artifact_lineage_store.rs:1500`).
4. `validate_output_contract` therefore rejects the underscore-bearing token
   tuple before establishment
   (`crates/engine/src/artifact_lineage_store.rs:630`).

At that baseline, correcting the mismatch required a production/runtime token
derivation or validation change. That surface was forbidden by the historical
dispatch, so the packet stopped without a workaround.

The historical default target locked the refusal with
`generic_decision_record_mutation_refuses_invalid_coverage_token_derivation`.
The positive end-to-end mutation test was ignored by default, remained
executable with `--ignored`, and was not counted as satisfied proof.

## Historical unavailable-proof disposition

At the historical baseline, mutation could not establish an intake record, so
P4 could not prove candidate append, promotion, post-promotion read/validate,
stale-basis refusal, or atomic retained-byte replacement for this family.
Renderer definition/golden and the remaining negative-surface/path cases were
not expanded after the mandatory stop.

The parent selected the first historical option through the later exact
operator authorization. The current GREEN proof above replaces only the prior
unavailable-proof recommendation. At this historical stop, the token
prerequisite was review-clean and committed while the separate P4
negative-surface/path gate remained outstanding; the current completion does
not confer P6 authority or slice completion.
