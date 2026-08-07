# HCM-3.5 P1 implementation-admission recovery — proof matrix

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P1

**Status:** CLEAN

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user grant
`p1-definition-and-implementation-admission-recovery`, preserving the P1
causal identity under
`handbook-hcm-3-5-continuation-implementation-20260807`.

## Frozen choices

| Subject | Frozen P1 choice |
| --- | --- |
| Public operation | `ground_resolution(repo_root: &Path, GroundingRequest) -> Result<GroundingOutcome, GroundingOperationError>` |
| Opaque inputs | `GroundingSnapshotRef`, `GroundingDeltaRef`, `GroundingDefinitionRef`, and `GroundingDisclosureRef`, each parsed only from `<exact-definition-ref>#sha256:<64-lowercase-hex>` |
| Bound definition | `handbook.grounding.summary.hcm-3-5-p1@1.0.0#sha256:e553ba87b1182df8d4bc587259bcbe227e208b2e1ef670a8b6dbc35f4ed75e1d` |
| Bound disclosure | `handbook.grounding.disclosure.hcm-3-5-p1@1.0.0#sha256:ec46c655d094a69f7d46aa1788f337d2feb0acb72398526f4570d470d0e66948` |
| Summary maximum / order | 2 entries; ascending `(kind, signal_id)` |
| Eligible kinds | `expected_progress`, `proof_drift`, `scope_expansion` |
| Omission partition | `ineligible`, `redacted`, `overflow`, with `UnaccountedSignal` refusal for malformed or duplicate accounting |
| Refusal boundary | missing/malformed/mismatched source, malformed or unsupported definition, incompatible delta, stale currentness, insufficient resolution, and unaccounted signals are typed payload-free refusals |
| Evidence | `Unavailable` or `False` only, with `authority_effect == none` |
| Persisted source route | fixed `.handbook/grounding/hcm-3.5/v1` policy, prior/current capture/record, catalog, route, and currentness witness; exact endpoint/canonical route fingerprints are verified before summary construction |

## Required proof evidence

| Requirement | Evidence | Result |
| --- | --- | --- |
| P1 is engine-only and creates the named module | `cargo check -p handbook-engine` | PASS |
| Exact opaque reference grammar admits only a ref plus SHA-256 binding | `cargo test -p handbook-engine --test hcm_3_5_grounding` | PASS (1 test) |
| Exact persisted prior/current inputs derive the compatible delta before grounding | `cargo test -p handbook-engine --lib grounding::tests` | PASS: `persisted_source_route_derives_the_compatible_delta_before_grounding` |
| Summary is bounded, ordered, and partitions all signals | same focused library test | PASS: `summary_is_stably_bounded_and_partitions_every_source_signal` |
| Redaction precedes entry materialization and five-family/slot currentness is exact | same focused library test | PASS: `redaction_precedes_entry_materialization_and_currentness_is_exact` |
| Existing crate compiles after the private adapter and public module addition | `cargo check -p handbook-engine` | PASS |
| Rust formatting and whitespace are valid | `cargo fmt --all -- --check`; `git diff --check` | PASS |
| HIGH projection executor is not modified or used | local GitNexus upstream impact and changed-subject inspection | PASS |
| Flow/pipeline/CLI/compiler/SDK/Substrate/Cargo/schema/config/gate runtime are untouched | changed-subject allowlist inspection | PASS |
| Staged changed-scope scan is executed before closeout | `npx --no-install gitnexus detect-changes --scope staged` (local FTS-disabled output: `No changes detected`) plus staged Git path manifest | PASS: command executed; GitNexus emitted no per-symbol rows, while the manifest contains only the declared allowlist |

The positive persisted-source proof deliberately exercises the same resolver
core reached only after `ContextResolutionEnvelope::projection_authority_view`
has produced current ranks. The P1 integration test independently proves the
public reference membrane. This keeps the HCM-3.2 authority fixture and the
HIGH projection executor outside P1 while retaining no raw consumer input or
shortcut around envelope currentness.

## Closure disposition

The discovery review, its single consolidated remediation, and the distinct
delta closure above found no remaining P1-scope defect. P1 is CLEAN. No P2,
P3, P4, P5, P6, publication, remote operation, protected-checkout mutation, or
gate runtime is included in this P1 proof.
