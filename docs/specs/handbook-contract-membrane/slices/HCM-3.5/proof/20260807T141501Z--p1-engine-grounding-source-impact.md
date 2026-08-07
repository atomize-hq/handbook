# HCM-3.5 P1 engine grounding — source and impact record

**Dispatch:** `20260807T141501Z--HCM-3-5--engine-grounding-bounded-delta-implementation`
**Status:** BLOCKED before source edits
**Scope:** the selector's P1 engine-only source/test/proof ceiling

## Baseline and GitNexus evidence

- `npx --no-install gitnexus status` reported the assigned detached checkout at
  `7e0836a` and the local index as up to date.
- The local CLI could not resolve `projection` or `snapshot_memory` as module
  targets, so those module-name requests produced `UNKNOWN`, not a GREEN
  result. The exact source symbols relevant to the permitted private adapters
  resolved as follows:

| Target | Direct callers | Affected processes | Risk | Disposition |
|---|---:|---:|---|---|
| `ContextMemorySnapshot` (`record.rs`) | 1 | 1 | LOW | No edit made. |
| `SnapshotDelta` (`delta.rs`) | 1 | 1 | LOW | No edit made. |
| `ContextResolutionEnvelope` | 0 | 0 | LOW | No edit made. |
| `execute_projection_with_live_observer` (`projection.rs`) | 4 | 4 | HIGH | Not edited and not used as a P1 shortcut. |

The HIGH executor affects the source-pair/currentness processes
`snapshot_source_pair_requires_exact_dependency_state_and_five_family_currentness`,
`currentness_none_and_exact_captured_revision_closures_refuse_substitution`,
`exact_currentness_requires_snapshot_selection_and_independent_live_observation`,
and `stale_envelope_stack_custom_kind_policy_and_evaluator_dependencies_refuse`.
Per the selector and parent direction, this run did not retrofit or call that
executor.

## Blocking source facts

The immutable authority requires `DeltaSignalSummary` to be governed by an
exact definition that fixes maximum cardinality, eligibility/order, overflow
behavior, permitted metadata, and the included/omitted/refused partition. The
selected documents describe those required fields as future-definition
responsibilities, but select neither their values nor an exact definition
artifact. In particular, `SPEC.md` lines 217–223 and the P1 selector state the
requirements without providing a cardinality, order, overflow action, or
definition reference that a `GroundingDefinitionRef` can validate.

The permitted immutable fixtures cannot supply the missing source pair:

- `crates/engine/src/projection/fixtures/snapshot-source-pair.json` contains
  projection-source documents but no `signals`, `delta_fingerprint`, or
  `handbook.snapshot-delta` value.
- `crates/engine/src/snapshot_memory/fixtures` contains one snapshot record and
  a drift catalog, but no compatible derived delta document. Deriving one would
  require a second snapshot/source or a fixture/runtime input not selected by
  P1.
- The selector forbids changing the fixed HCM-3.4 fixture and forbids adding a
  schema/configuration surface. Inventing a private source layout, synthetic
  delta, summary limit, ordering rule, or overflow policy would therefore
  violate the frozen contract rather than implement it.

## Result

No existing source module, function, class, method, or public surface was
edited. No dependency, fixture, configuration, consumer, protected checkout,
staging, commit, ref update, or remote operation occurred.

The minimum unblocking authority is a fresh selector that identifies the exact
summary definition (including cardinality, eligibility/order, overflow,
permitted metadata, and omission/refusal policy) and admits a validated current
snapshot plus compatible derived-delta source pair, either by selecting an
existing source layout or by separately authorizing a reviewed fixture delta.
