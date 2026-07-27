# HCM-2.4 P4 Decision Record implementation proof

Status: **STOPPED — runtime authority required**

The bounded stop was independently reproduced by
`/root/hcm_2_4_p4_stop_review` from dispatch
`20260727T004037Z--HCM-2-4--p4-decision-record-stop-review`. The reviewer
returned one Required runtime-prerequisite finding; P4 remains incomplete.

Dispatch:
`20260727T001100Z--HCM-2-4--p4-decision-record-implementation`

Subject:
`sha256:778464084def625c25ec95f2a1268c1793ed71e9138ff213f363e9043ddd3161`

## Scope and baseline

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

## Successful bounded proof

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

## Stop-condition proof

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

Correcting the mismatch requires a production/runtime token derivation or
validation change. That surface is forbidden by this dispatch, so the packet
stopped without a workaround.

The default target now locks the refusal with
`generic_decision_record_mutation_refuses_invalid_coverage_token_derivation`.
The positive end-to-end mutation test is ignored by default, remains
executable with `--ignored`, and is not counted as satisfied proof.

## Unavailable proof and required disposition

Because mutation cannot establish an intake record unchanged, P4 cannot yet
prove candidate append, promotion, post-promotion read/validate, stale-basis
refusal, or atomic retained-byte replacement for this family. Renderer
definition/golden and the remaining negative-surface/path cases were not
expanded after the mandatory stop.

The parent must either:

1. authorize a fresh, separately impacted same-scope runtime correction for
   generic intake-value token derivation/validation and redispatch P4; or
2. accept that the exact Decision Record intake is not currently supported by
   the unchanged generic HCM-2.3 mutation path.

The shared authority disposition is
`decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`. This record
does not claim P4 completion or slice completion.
