# P5 Risk Record implementation proof — bounded stop

## Packet identity

- Dispatch:
  `20260727T001101Z--HCM-2-4--p5-risk-record-implementation`
- Subject:
  `sha256:bfd409f816fc79b69f750860ccea10b126d1a6ac0da9c174c059c02b129d0fa5`
- Parent orchestration:
  `20260726T195633Z--HCM-2-4--implementation-resume-orchestration`
- Status: bounded implementation stopped on an out-of-selector generic-runtime
  prerequisite.
- Independent stop review:
  `20260727T004038Z--HCM-2-4--p5-risk-record-stop-review`,
  reviewer `/root/hcm_2_4_p5_stop_review`, CLEAN on the truthful partial proof.

The dispatch's “four fixture files” and “six write paths” counts were clerical
errors. Parent authority confirmed that the exact P5 path manifest controls:
three named fixture files, this named test, and this proof record only.

## Baseline and impact replay

All 12 subject-manifest entries matched their listed SHA-256 values. The
ordinal `repo-path-null-sha256-newline-v1` aggregate replayed the dispatch
subject exactly, and all 12 text files passed the no-trailing-whitespace
hygiene check. No P4 or P5 fixture, target, or proof path existed before the
packet edits.

GitNexus reported an up-to-date index at commit `8c16a90`. Its MCP surface was
not available in the execution session, so the checked-in CLI wrapper provided
the exact symbol evidence:

| Read/proof-only symbol | Risk | Direct callers | Processes | Modules |
| --- | ---: | ---: | ---: | ---: |
| `resolve_profile_selection` | CRITICAL | 33 | 14 | 20 |
| `ArtifactRepositoryV1::open` | CRITICAL | 22 | 23 | 20 |
| `ArtifactRepositoryV1::read` | CRITICAL | 132 | 1 | 13 |
| `ArtifactMutationServiceV1::intake_append` | MEDIUM | 5 | 0 | 2 |
| `ArtifactRepositoryV1::validate` | LOW | 0 | 0 | 0 |

No indexed production symbol was edited.

## RED proof

The initial target contained only the exact selection-fixture assertion. With
the fixture absent, the isolated command was:

```text
$env:CARGO_TARGET_DIR='target/hcm-2-4-p5'; cargo test -p handbook-engine --test hcm_2_4_risk_record risk_record_fixture_is_repository_selected -- --exact --nocapture
```

It failed at the intended boundary:

```text
P5 Risk Record repository selection fixture: Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }
test result: FAILED. 0 passed; 1 failed
```

After adding only `.handbook/profile-selection.json`, the same command passed
`1 passed; 0 failed`.

A second focused RED then exercised the unchanged generic selector before the
profile existed:

```text
P5 profile resolves without runtime changes: ProfileLoadError {
  kind: MissingSource,
  location: Some(".handbook/definitions/profiles/risk-record-root-1.0.0.yaml"),
  detail: "source file does not exist"
}
```

The profile's complete typed-closure fingerprint was independently replayed as
`sha256:8fc23836913d053e2ca1a765ac30dbf1711ffbea8ecffb80f9c7ef7558ba30e4`.
The generic selector then admitted the exact fixture without a runtime edit.

## Available GREEN proof

The exact fixture selects
`example.profile.hcm-2-4-risk-record@1.0.0`, extends
`handbook.profile.shipped-root@1.2.0`, and differs from the parent instance set
only by `risk_record`. The descriptor has the exact 1.1 kind, null role,
`.handbook/records/risk.yaml`, the exact 1.0 intake, the singleton fixed
renderer, `always` requiredness, and empty capabilities, dependencies,
lifecycle, Projection, overlay, and extensions.

The bounded target proves:

1. exact repository-path profile source and built-in schema/kind/intake source
   identity;
2. complete typed profile closure and exact descriptor admission;
3. exact kind, schema-entry, schema-document, and intake fingerprints;
4. all nine coverage rows and all three supported intake modes converge on the
   same schema-valid canonical content;
5. exact retained source bytes, source fingerprint, safe canonical path,
   repository read, and repository validate;
6. malformed, duplicate, unknown-field, unsafe-source-path, descriptor
   widening, duplicate-key, schema-unknown-field, and inferred-filename
   refusal;
7. exact fixed renderer definition, null Resolution input, schema-valid golden
   input, full Markdown byte length, and SHA-256;
8. no shipped-root instance, generated Risk Record command, inferred filename,
   Projection, or persistent Markdown influence.

The green subset command was:

```text
$env:CARGO_TARGET_DIR='target/hcm-2-4-p4-p5-stop'; cargo test -p handbook-engine --test hcm_2_4_risk_record
```

Result:

```text
test result: ok. 12 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

The unchanged HCM-2.3 preservation target also passed:

```text
$env:CARGO_TARGET_DIR='target/hcm-2-4-p5'; cargo test -p handbook-engine --test hcm_2_3_registration_kernel
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`cargo fmt --all -- --check` passed.

## Unavailable mutation proof and exact stop reason

The positive mutation proof remains intentionally unavailable and is ignored
by the default target. Its exact explicit command remains red:

```text
$env:CARGO_TARGET_DIR='target/hcm-2-4-p4-p5-stop'; cargo test -p handbook-engine --test hcm_2_4_risk_record -- --ignored --exact generic_mutation_promotes_then_reads_and_validates_new_real_bytes --nocapture
test result: FAILED. 0 passed; 1 failed; 12 filtered out
```

The only failure is
`generic_mutation_promotes_then_reads_and_validates_new_real_bytes` at the
unchanged `ArtifactMutationServiceV1::intake_append` boundary:

```text
ArtifactMutationErrorV1 {
  kind: Store,
  detail: "generic lineage store refused: intake output tuple is not exact or unique"
}
```

The failure is deterministic and independent of fixture values:

1. The released Risk Record intake defines required coverage IDs including
   `risk_record.schema_id`, `risk_record.schema_version`,
   `risk_record.record_id`, `risk_record.evidence_refs`, and
   `risk_record.review_basis`
   (`crates/engine/definitions/intakes/handbook.intake.risk-record/1.0.0.yaml:10-18`).
2. `intake_commit_plan` derives retained-value tokens by taking the coverage ID
   suffix and appending `-value`
   (`crates/engine/src/artifact_mutation.rs:1605-1610`).
3. The generated tokens therefore include `schema_id-value`,
   `schema_version-value`, and other underscore-bearing values.
4. `valid_intake_value_token` permits only ASCII lowercase letters, digits,
   and hyphens (`crates/engine/src/artifact_lineage_store.rs:1500-1508`).
5. `validate_output_contract` refuses the planner-generated tuple at
   `crates/engine/src/artifact_lineage_store.rs:631-637`.

Changing fixture values cannot alter these fixed coverage IDs. Renaming the
coverage IDs would edit a released P1A intake definition and break its
fingerprint. Making the existing planner normalize token suffixes, or widening
the lineage-store token grammar, is a production/runtime edit outside P5.

Therefore the dispatch stop condition is met: the unchanged generic HCM-2.3
path cannot complete Risk Record mutation for the exact admitted fixture.
Candidate append, promotion, post-mutation retained bytes, read, and validate
remain unavailable until parent authority dispatches and accepts a separate
runtime prerequisite.

The default target locks that boundary with
`generic_risk_record_mutation_refuses_invalid_coverage_token_derivation`. The
ignored positive test is still executable and is not counted as satisfied
proof.

## Scope disposition

The shared authority disposition is
`decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`. This record
does not claim P5 completion or slice completion. No production/library source,
released definition, schema, Cargo file, command, public API, or sibling
packet was changed.
