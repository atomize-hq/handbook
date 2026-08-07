# HCM-3.5 audit-corrective source-ingestion selector

**Phase / slice / packet:** HCM-3 / HCM-3.5 /
audit-corrective-source-ingestion-negative-proof-and-replayable-closeout

**Status:** ADMITTED — read-only boundary freeze before corrective discovery

**Corrective parent:** `handbook-hcm-3-5-audit-corrective-implementation-20260807`

**Authority binding:** direct user authorization for task
`019fddfa-610b-7512-b591-654ba4f5130a` on host `local`, issued by
meta task `019fda65-3892-7900-a692-2dd0a0fccc7a`, nonce
`dd04871aad668887df84520236dca6db213aa2975a5353f38f0ee3f7f11db63c`.

**Historical context only:**
`20260807T181740Z--HCM-3-5--orchestration--p1-p4-transition-materialization-completed`
and its `handbook-hcm-3-5-continuation-implementation-20260807` parent
remain immutable source context. This new corrective parent neither resumes,
rewrites, nor claims cross-parent supersession of that terminal history.

## Frozen admission boundary

The corrective implementation may change only these production paths:

- `crates/engine/src/definition_identity.rs`, for a narrow private/shared
  bounded regular-file, non-symlink source adapter using the existing
  `SourceByteBudget` and duplicate-key-safe schema JSON machinery;
- `crates/engine/src/grounding.rs`, for P1 source ingestion and its existing
  typed refusal mapping;
- `crates/engine/src/grounding_transition.rs`, for P4 source ingestion and
  its existing `InvalidSource` surface; and
- `crates/engine/src/snapshot_memory/mod.rs`, only where the existing source
  JSON parser is integrated into HCM-3.5 derivation.

The only admitted test paths are `crates/engine/tests/hcm_3_5_grounding.rs`
and, if it is necessary, one new
`crates/engine/tests/hcm_3_5_transition_refs.rs`. Focused unit coverage in an
admitted production module may exercise the same private adapter and must not
create a new public API. Audit-corrective selector, proof, dispatch, record,
and ledger files are documentation/closeout capacity only.

## Parser, reader, and refusal freeze

The complete source boundary under review is:

| Boundary | Existing entry points | Required corrective posture |
| --- | --- | --- |
| Source byte accounting and schema JSON | `SourceByteBudget`, `parse_schema_json`, duplicate-key rejection in `definition_identity` | Every HCM-3.5 source is admitted through one bounded regular-file/non-symlink reader and parsed with duplicate-key rejection. |
| P1 public grounding | `ground_resolution`, `resolve_with_authority`, `load_source_pair`, source/currentness readers and JSON parser in `grounding` | Invalid or malformed input maps only to the existing typed P1 refusal kinds; the operation returns no usable outcome and never exposes raw source bytes. |
| P4 materialization | `materialize_grounding_transition_refs`, its source reader, and exact-ref parser in `grounding_transition` | Every malformed, unsafe, or mismatched source maps only to existing `GroundingTransitionMaterializationError::InvalidSource`; no ref materializes. |
| Shared HCM-3.5 derivation | `json_value`, `derive_grounding_source_pair`, `derive_grounding_transition`, and `validate_grounding_route` in `snapshot_memory/mod.rs` | HCM-3.5 source JSON retains the same duplicate-key-safe parser, with invalid input remaining on the existing internal invalid-source path. |

No public JSON/Serde transport, handoff schema/template/validator, Flow,
pipeline, resolver/compiler, SDK, CLI, Substrate, gate runtime, Cargo file,
dependency, configuration, or retained projection/resolver/compiler seam is
admitted. The protected checkout is not a source of truth for this task and
must never be accessed.

## Complete final-review subject

The final corrective review must bind a byte-hashed manifest over the complete
HCM-3.5 final code path introduced by original P1–P4 plus this correction: the
four production files above; existing public HCM-3.5 grounding coverage; the
tracked `.handbook/grounding/hcm-3.5/v1` P4 source package; the original P1–P4
selectors/proofs and historical completed record as read-only context; this
selector and its corrective proof; every current corrective v1.4 dispatch; and
the primary corrective diff. It must review the final path, not merely the
corrective delta, and must precede the separately mechanical completed record
and ledger closeout.

## Preconditions and evidence posture

The assigned worktree was clean and detached at
`38097b69d1f1c5b1ab857e4252a442f45834bae6` (tree
`53d172cebcb20d43f9a01298ea14df436efef485`), with required ancestor
`1a10bfd59e8433fa344dd2baa4880b4b87c7f8f6`. The local target ref had the same
expected old value before this corrective parent began. Local GitNexus graph
MCP and `.gitnexus/run.cjs` are unavailable in this checkout, so required
upstream impact and later detection are recorded as unavailable rather than
green; no inferred risk classification substitutes for them.

The old Packet 0 historical global-validator contradiction remains transparent
as unavailable/not-green if it persists. It is not ordinary full-validator
GREEN, and it does not waive the available targeted corrective checks,
dispatch/record/ledger validation, or fresh CLEAN review requirement.
