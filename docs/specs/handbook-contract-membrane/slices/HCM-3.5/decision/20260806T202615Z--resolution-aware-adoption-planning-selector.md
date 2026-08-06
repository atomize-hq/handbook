# HCM-3.5 Resolution-aware adoption planning selector

**Decision status:** frozen documentation-only candidate pending one fresh
complete-subject discovery review and any strictly causal remediation/closure.

## Selected outcome and allowed paths

This selects exactly one planning outcome:
`hcm-3-5-resolution-aware-adoption-planning` /
`HCM-3.5-P1-resolution-aware-adoption-planning`.

The reviewed-primary subject is limited to:

- `docs/specs/handbook-contract-membrane/slices/HCM-3.5/SPEC.md`;
- `docs/specs/handbook-contract-membrane/slices/HCM-3.5/tasks/plan.md`;
- `docs/specs/handbook-contract-membrane/slices/HCM-3.5/tasks/todo.md`;
- this selector and HCM-3.5-local planning proof records; and
- the two exact current-truth corrections in `00-README.md` and
  `03-seam-crosswalk.md`.

The required discovery/closure dispatches are immutable v1.4 audit artifacts.
Mechanical closeout may add only one parent-owned v1.4 handoff, the rebuilt
ledger entry, and an exact P3/P4 inventory transcription if applicable.

## Predecessor and frozen dependency facts

Consume, but do not resume, rewrite, supersede, or broaden,
`20260806T191500Z--HCM-3-4--orchestration--private-projection-source-pair-implementation-completed`.
The predecessor is validated ledger evidence for the HCM-3.4 private source
pair only. Its source `ca765cc` private Snapshot Memory primitives and `cc6d849`
source-pair addition are present at this base, but `crates/engine/src/lib.rs`
keeps both `snapshot_memory` and `projection` private. No Flow/pipeline
consumer can use those types without a newly selected exported contract.

The HCM-3.2 public Context Resolution kernel is an available dependency;
Flow and pipeline already depend on `handbook-engine`. The existing public
Flow/pipeline inputs and results have current callers. No new dependency is
selected by this planning subject.

## Frozen authority boundary

This selector intentionally does not choose:

- whether `handbook-engine` exports a typed Snapshot Memory/Projection
  operation or value surface;
- whether existing `ResolveRequest`, `ResolverResult`, pipeline compile
  request/results, and their CLI/compiler callers are extended, versioned, or
  left unchanged in favor of new purpose-named operations; or
- whether HCM-3.5 consumes a concrete gate runtime now or preserves the
  local-closeout/parent-promotion distinction as an unimplemented boundary
  until its selected owner exists.

Each is a public API/compatibility or product-operation decision explicitly
outside this planning grant. The exact authority request and recommended
non-breaking option are in `../SPEC.md`.

## Causal outcome and proof posture

| Integrated outcome | Packet | Authority reference |
|---|---|---|
| `hcm-3-5-resolution-aware-adoption-planning` | `HCM-3.5-P1-resolution-aware-adoption-planning` | this selector |

One complete-subject discovery review, one consolidated P1/P2 remediation,
and one different-fresh closure are available. At most two immediately causal
supplemental cycles may follow a valid P1/P2 directly caused or unmasked by the
previous repair. No cycle may follow CLEAN. GitNexus MCP/index support is
unavailable in this checkout, so code impact and compare artifacts are
recorded as unavailable; no code symbol is edited by this subject.

The proof wall is documentation-only: predecessor ledger/record validation,
live source and call-path inspection, protected-boundary evidence, manifest
replay, UTF-8/whitespace and `git diff --check`, dispatch validation, fresh
review, and v1.4 closeout validation. It does not run or claim product tests.

## Stop conditions

Stop at an authority boundary after the reviewed documentation subject when
the public compatibility/gate-operation decision remains absent. Do not infer
it from private module visibility, existing dependencies, the nullable
handoff fields, historical code, or the Phase-3 map. No Rust, test, fixture,
schema, dependency, public API, configuration, consumer, SDK, CLI, transport,
remote, protected-checkout, HCM-3.6, HCM-4+, HCM-5, or Phase-3 exit work is
authorized.
