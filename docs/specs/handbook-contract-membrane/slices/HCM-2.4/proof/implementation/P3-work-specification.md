# HCM-2.4 P3 — Work Specification / Stage 10

Status: **implemented and closure-review CLEAN**

Closure review:
`20260727T014349Z--HCM-2-4--p3-work-specification-closure-review`;
reviewer `/root/hcm_2_4_p3_closure_review`; subject
`sha256:7bdfef55f7e531abc1f06661ba4675dceb08fb8d9fa14f8e93456312f8344086`.

## Authority switch

Stage 10 now accepts one duplicate-free Work Specification YAML object, resolves the exact `example.profile.hcm-2-4-work-specification@1.0.0` descriptor through `ArtifactRepositoryV1`, submits the exact eight fields to `handbook.intake.work-specification@1.0.0` in Express mode, requires a Complete result, and canonicalizes with `handbook_engine::canonical_yaml`.

The authoritative artifact is `artifacts/work-specification/work-specification.yaml`. `artifacts/feature_spec/FEATURE_SPEC.md` is a deterministic generated review view. Capture identity, provenance hashes, handoff input bytes, and record-derived feature identity bind the canonical YAML. Existing public provenance and handoff field names remain unchanged; the handoff copy retains `ExternalManualDerived`.

## RED proof

Before implementation:

```text
cargo test -p handbook-pipeline --test pipeline_capture capture_preview_stage_10_rejects_markdown_as_canonical_input -- --exact --nocapture
FAILED: preview succeeded and planned artifacts/feature_spec/FEATURE_SPEC.md
```

Discovery review then found that schema-valid multiline/tab text reached the
selected intake but was rejected only by the Markdown renderer:

```text
CARGO_TARGET_DIR=target/hcm-2-4-p3-remediation-red cargo test -p handbook-pipeline --test pipeline_capture capture_apply_stage_10_renders_schema_valid_multiline_and_tab_text -- --exact --nocapture
FAILED: InvalidCaptureInput, "admitted Work Specification text contains a Markdown control character"
```

## Validation

All commands used `CARGO_TARGET_DIR=target/hcm-2-4-p3-work-specification`.

```text
cargo test -p handbook-pipeline --lib                         26 passed
cargo test -p handbook-pipeline --test pipeline_capture       47 passed
cargo test -p handbook-pipeline --test pipeline_handoff       10 passed
cargo test -p handbook-compiler --test pipeline_capture       39 passed
cargo test -p handbook-compiler --test pipeline_handoff        8 passed
cargo test -p handbook-pipeline --test pipeline_compile       20 passed
cargo test -p handbook-compiler --test pipeline_compile       20 passed
cargo test -p handbook-cli --test pipeline_handoff_refusals    4 passed
cargo test -p handbook-cli --test feature_spec_contract        1 passed
```

Negative proof covers Markdown-as-authority, duplicate YAML keys, missing and
incorrect repository profile selection, incomplete intake, unknown fields,
schema-invalid exact-field values, raw compile payloads, and capture-cache view
tampering. Positive proof covers descriptor-selected admission, canonical YAML
persistence, schema-valid multiline/tab normalization, byte-exact Markdown
rendering, YAML-bound provenance, record-ID identity, handoff trust, custom
storage layout, and successful handoff after the persisted Markdown view is
mutated or deleted.

After the permanent proof-corpus mirrors were materialized, the shared
capture-ready installers began seeding both post-capture outputs into temporary
repositories. Six capture tests and one missing-input CLI refusal then failed
at their setup boundary. The parent removed only those two seeded outputs from
the authorized capture-ready test helpers and made the CLI negative delete the
canonical YAML authority path instead of the disposable Markdown view. The
complete selector wall above is the post-remediation result.

The discovery remediation changed only the LOW-risk
`controlled_markdown_text` renderer helper, focused capture/handoff tests, and
the architect directive. Whitespace continues to normalize deterministically;
any remaining schema-valid control code point is rendered through its stable
escaped form rather than rejected. The directive now places its required
approach and alternative inside `objective`, preserving the exact eight-field
contract.

Additional packet checks passed:

```text
cargo fmt --all -- --check
cargo clippy -p handbook-pipeline -p handbook-compiler -p handbook-cli --all-targets -- -D warnings
git diff --check
```

## Impact analysis

- `build_capture_plan`: LOW, 5 upstream, 2 direct, no process.
- `canonicalize_capture_plan_for_apply`: HIGH, 6 upstream, `apply_capture_plan`.
- `validate_single_file_capture_content`: HIGH, 4 upstream, `build_capture_plan`.
- `build_stage_10_capture_provenance_for_apply`: HIGH, 6 upstream, `apply_capture_plan`.
- `build_stage_10_feature_spec_capture_provenance`: CRITICAL, 7 upstream across capture apply and handoff emit.
- `build_input_copy_plans`, `expected_trust_class_for_source`, and `derive_feature_id`: HIGH within the dispatched handoff flow.
- `controlled_markdown_text`: LOW, 3 upstream, 1 direct, no indexed process expansion.

No public handoff schema/API was changed. No P6 aggregate bridge or fixed-selector deletion was attempted.
