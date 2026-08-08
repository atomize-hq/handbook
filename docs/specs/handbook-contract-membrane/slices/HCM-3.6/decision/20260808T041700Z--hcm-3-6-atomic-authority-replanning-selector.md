# HCM-3.6 atomic-authority replanning selector

**Selector ID:** `HCM-3.6-ATOMIC-AUTHORITY-REPLANNING-SELECTOR-01`

**Packet ID:** `HCM-3.6-ATOMIC-AUTHORITY-REPLANNING`

**Integrated outcome:** `hcm-3-6-atomic-authority-replanning`

## Bound authority and immutable lineage

This selector is bound to task `019fdf87-108a-7001-849f-842de3cdbbaf`, host
`local`, meta workflow
`handbook-hcm-3-6-atomic-authority-replanning-20260808`, and dispatch nonce
`11ed24851c1e66c6e226c18e5898a895879aa9846c34912a5f60bab72e699e66`.
It admits one documentation-only authority-repair subject. It does not admit
Rust, product tests, fixtures, schemas, dependencies, configuration, public
APIs, transports, adapters, HCM-5 behavior, release, or remote work.

The direct source handoff is
`20260808T031746Z--HCM-3-6--orchestration--project-posture-implementation-authority-boundary`.
Its P0 primary commit `cab94769bcc14049ecd42378cf34462079fae386`,
subject fingerprint
`sha256:4ddc7eec16f3eb79af909416b2306f5f9b2c9b31bf25087edea8d58d351cc3a1`,
fresh CLEAN discovery review, blocker
`HCM36-P3-ATOMIC-TRANSITION-PRIMITIVE`, and dispatch population remain
immutable evidence. This selector resolves the named authority question; it
does not rename, rerun, waive, or continue the predecessor review cycle.

| Field | Bound value |
| --- | --- |
| Phase / slice | `HCM-3` / `HCM-3.6` |
| Parent orchestration | `handbook-hcm-3-6-atomic-authority-replanning-20260808` |
| Expected base/tree | `88a3e78a1eb5b840e64248856cf129ea929021ab` / `0d0e7ad2961ba94240da0f3fe628bc66784825d2` |
| Required ancestor | `1256e724a2b7da6b6250f57d6f63fced1e2cf949` |
| Local integration ref | `refs/heads/orchestration/handbook-hcm-3-6-atomic-authority-replanning-20260808` |
| Publication | local-only expected-old CAS; no network or remote mutation |

## Selected decision

A later, separately selected HCM-3.6 implementation may remain private,
transport-free, and schema-internal. The minimum compatible repair is:

1. keep canonical Charter YAML as the only editable constitutional authority
   and `ProjectPostureKernel` as a derived value;
2. add a private posture transaction under the existing Charter authority lock
   order, without changing the public `CommittedCharterAuthorityV1` shape;
3. derive one heterogeneous canonical-authority chain from committed promotion
   journal `1.2` edges and committed posture journal `1.0` edges;
4. persist one private `handbook.posture-transition` `1.0` record and one
   private `handbook.lifecycle-transition` `1.1` posture-rebase record in the
   same atomic output group as the new Charter bytes;
5. retain existing lifecycle-transition `1.0` and promotion records byte for
   byte, while allowing a later candidate promotion and lifecycle event to use
   the posture-derived head; and
6. prove the private seam with crate-local tests, never an external integration
   test that cannot access a private module.

The three new durable identities are internal Rust-decoded records, not public
transport schemas or package definitions:

- `handbook.posture-transition` / `1.0`, content-addressed beneath
  `.handbook/state/posture-transitions/`;
- `handbook.lifecycle-transition` / `1.1`, content-addressed beneath the
  existing `.handbook/state/lifecycle-transitions/` partition and admitted only
  by the private lifecycle loader; and
- `handbook.charter-posture-transaction-intent` / `1.0`, journaled beneath
  `.handbook/state/transactions/posture-transitions/`.

No `definitions/schemas` asset, `LineageRecordClassV1` public variant, public
Rust type/export, JSON/DTO operation, dependency, or configuration change is
required. Any implementation that discovers such a need must stop for new
authority rather than widen this decision.

## Physical Charter mutation

The canonical Charter `1.1` shape stores the nine dimensions as one fixed-order
array. A posture transition may change only the selected array member's
`level_override` leaf. The exact map is:

| Index | `dimension_id` | RFC 6901 authority path |
| ---: | --- | --- |
| 0 | `speed_vs_quality` | `/engineering_posture/dimensions/0/level_override` |
| 1 | `type_safety_static_analysis` | `/engineering_posture/dimensions/1/level_override` |
| 2 | `testing_rigor` | `/engineering_posture/dimensions/2/level_override` |
| 3 | `scalability_performance` | `/engineering_posture/dimensions/3/level_override` |
| 4 | `reliability_operability` | `/engineering_posture/dimensions/4/level_override` |
| 5 | `security_privacy` | `/engineering_posture/dimensions/5/level_override` |
| 6 | `observability` | `/engineering_posture/dimensions/6/level_override` |
| 7 | `dx_tooling_automation` | `/engineering_posture/dimensions/7/level_override` |
| 8 | `ux_polish_api_usability` | `/engineering_posture/dimensions/8/level_override` |

The engine must verify the `dimension_id` at the fixed index. The physical old
value is `null` or an integer `1..=5`; the effective old level is
`level_override.unwrap_or(posture.baseline_level)`. The proposed physical value
is `null` exactly when the proposed effective level equals the unchanged
baseline, otherwise the proposed integer. A same-effective-level request is a
refused no-op, including a physical-only `Some(baseline) -> null` rewrite.
`posture.baseline_level` is never a posture-transition target because changing
it could change multiple effective dimensions.

CAS binds the exact current Charter bytes, byte length, SHA-256/fingerprint,
the expected physical leaf, baseline, and expected effective level. The engine
parses and deterministically reserializes the whole Charter, then proves a
typed deep diff containing exactly the one mapped leaf before staging. It does
not patch text by substring. Any other value, order, byte, fingerprint, or
effective-level change refuses before a domain write.

## Heterogeneous current-head and transaction decision

Every committed promotion or posture transaction contributes one edge
`basis canonical fingerprint -> resulting canonical fingerprint`, plus its
source record pair and resulting lifecycle-head pair. The merged, bounded
history must have one promotion genesis, no fork, cycle, duplicate successor,
unreachable edge, or more than 4,096 committed edges. Pending transactions are
recovered before traversal. Rolled-back transactions are validated but never
participate. Current Charter bytes must equal the unique terminal edge by
fingerprint, document SHA-256, and byte length.

The compatibility projection returned by the existing public read retains the
most recent promotion ancestor in `promotion_ref`, the actual current lifecycle
head in `lifecycle_transition_ref`, and the actual current Charter bytes and
fingerprint. Internal consumers additionally receive the terminal source kind,
source ref/fingerprint, and lifecycle ref/fingerprint. Thus current reads,
posture evaluation, lifecycle observation/event recording, later candidate
promotion, and both recovery engines select one identical current Charter
without a new public field.

The posture transaction atomically owns:

1. deterministic resulting canonical Charter bytes;
2. one immutable `PostureTransition` record;
3. one lifecycle-transition `1.1` posture-rebase record that requires prior
   lifecycle state `current`, no active observations, preserves that state,
   binds the posture transition, and records exactly
   `engineering_posture.dimensions` as reassessed coverage; and
4. one exact JCS+LF transaction intent plus progress/terminal markers and the
   old canonical rollback snapshot.

All semantic/currentness/approval/reassessment/resulting-kernel checks occur
before the pending journal is created. Normal refusal therefore changes none
of the canonical, posture-record, lifecycle-record, or posture-journal
partitions. Once prepared, exact stages are absent or complete. Recovery uses
only the intent, exact stages/finals, old snapshot, and current canonical:

- basis canonical plus no installed final rolls back owned stages and records a
  rolled-back terminal;
- resulting canonical plus exact stages/finals installs any missing final and
  rolls forward to committed;
- a committed marker requires resulting canonical and both exact finals;
- a rolled-back marker requires basis canonical and no transaction-owned final;
- any other, mismatched, unsafe, forked, or incomplete state is preserved and
  refused, never guessed, deleted, or reported committed.

The intent retains the normalized, bounded, non-payload kernel replay closure.
Admission and recovery recompute the resulting kernel from the proposed/current
Charter plus that exact closure. The committed `PostureTransition` resulting
kernel ref/fingerprint must match replay before success is returned.

## Acyclic identity and closed durable grammar

The construction order is normative and cannot be inverted:

```text
closed PostureTransition without a resulting lifecycle pair
  -> posture fingerprint/ref/exact JCS+LF bytes
  -> closed lifecycle-transition 1.1 binding the completed posture pair
  -> lifecycle fingerprint/ref/exact JCS+LF bytes
  -> closed posture intent binding Charter + both exact records
  -> exact stages/intent-bound markers -> committed merged edge
```

`PostureTransition` retains the prior `AuthorityHead`, expected/result
`CanonicalDocument`s, exact `Change`, recommendation/source-kernel/policy and
approval pairs, actor, exact `Reassessment`, bounded `KernelReplay`, resulting
kernel pair, UTC-second audit time, empty extensions, and semantic fingerprint.
It contains no resulting lifecycle ref, fingerprint, hash, or length.
Lifecycle `1.1` contains the prior lifecycle pair, current-state fingerprints,
empty observation arrays, prior/result canonical fingerprints, completed
PostureTransition pair, byte-equal reassessment, equal audit time, empty
extensions, and its own derived identity. The intent contains the same basis,
expected/change/authority/reassessment/replay values, exact canonical output,
and both output record ref/fingerprint/document-hash/length tuples.

The exact closed shapes and bounds are those in synchronized `SPEC.md` and
`05`: `sha256:` plus 64 lowercase hex; content refs whose 64-hex basename
equals the paired fingerprint; safe generic refs at most 512 bytes; canonical
bytes `1..=1_048_576`; each private JSON record `1..=262_144`; level/index
bounds `1..=5` / `0..=8`; approvals and validation results `1..=16`; replay
pair/ref lists `0..=256`; exactly nine ordered replay dimensions; sorted unique
unordered lists; UTC-second `Z`; all fields required; unknown/missing/extra
fields and unknown enums refused; and only the two stored `Change` values and
replay freshness may be JSON null.
Every record is RFC 8785 JCS plus exactly one LF. Posture/lifecycle semantic
fingerprints exclude only ID, their own fingerprint, and audit time; intent
fingerprint excludes only itself and includes its internally allocated
`posture-transaction_<32-lower-hex>` ID.

Every semantically unordered `Pair` array is unique and strictly sorted by the
lexicographic tuple (`ref` UTF-8 bytes, `fingerprint` UTF-8 bytes), comparing
the ref first and the fingerprint only for byte-equal refs. Ref-only or
fingerprint-only alternatives are invalid; other unordered ref arrays sort by
their element UTF-8 bytes.

The root also has only fixed `.intent-staging` and `.output-staging` scratch
directories. Independently random 32-lower-hex scratch names use `.intent`,
`.canonical`, `.posture-transition`, or `.lifecycle-transition`; partial
scratch is non-authoritative and enters pending only after exact verification
and no-replace rename. Unknown/unsafe scratch is preserved and refused.

The pending journal has one exact name grammar: `intent.json`,
`canonical.old`, `canonical.new`, `posture-transition.new`,
`lifecycle-transition.new`, and temp/published `prepared`,
`canonical-installed`, `records-installed`, `committed`, or `rolled-back`
markers. A marker is exactly
`sha256:<SHA-256 of exact intent.json bytes>\n` (72 ASCII bytes); temp content
is only an exact prefix. Stages admit only canonical -> posture -> lifecycle;
forward markers admit only prepared -> canonical-installed ->
records-installed -> committed; rollback is mutually exclusive. Committed and
rolled-back terminal name sets are exact, terminal publication is no-replace,
and any unknown/unsafe/missing/substituted byte preserves evidence and refuses.

Before pending creation and on every recovery pass, equality is transitive and
exact: intent basis equals PostureTransition `prior_authority_head`, intent
expected equals PostureTransition `expected_canonical`, and the complete JSON
values satisfy `intent.basis_head.canonical == intent.expected_canonical ==
PostureTransition.prior_authority_head.canonical ==
PostureTransition.expected_canonical == intent.recovery.old_canonical`.
Intent change/authority/reassessment/replay equals PostureTransition; intent
canonical output equals the posture result; the lifecycle prior head, canonical
pair, authority pair, reassessment, and time equal their named basis and posture
values; output hashes/lengths equal selected JCS+LF bytes; and the committed
merged edge uses intent basis -> result, the posture source pair, the lifecycle
result-head pair, and the retained promotion ancestor. Thus the intent, every
marker, and the merged edge bind both finals without a semantic fingerprint
cycle.

## Exact future implementation ceiling

This planning selector grants no source work. A later implementation selector
may authorize only these product paths, and only after repeating impact and
acceptance of every HIGH/CRITICAL seam:

- `crates/engine/src/lib.rs` — private `mod` declarations only;
- new `crates/engine/src/project_posture.rs`;
- new `crates/engine/src/project_posture_tests.rs` as a `#[cfg(test)]`
  crate-local child module;
- new `crates/engine/src/charter_posture_transaction_intent_v1.rs`;
- new `crates/engine/src/charter_lifecycle_transition_v11.rs`;
- existing `crates/engine/src/charter_authority_transaction.rs` only for the
  shared heterogeneous head, private posture transaction, and recovery hooks;
- existing `crates/engine/src/charter_lifecycle_store.rs` only for current-head
  selection, private v1.1 lifecycle-head validation, and retained-record reads;
  and
- new `crates/engine/src/charter_posture_transaction_tests.rs` as a
  `#[cfg(test)]` child of the transaction owner.

No external `crates/engine/tests/hcm_3_6_project_posture.rs` is selected: it
cannot access the private owner without a public export. Existing HCM-2.2
promotion, approval, lifecycle, Flow, and workspace tests remain unchanged but
must pass as regression consumers. Any required edit to them, any fixture or
schema asset, or any other product path is a new-authority stop.

## Ownership and non-goals

`handbook-engine::project_posture` owns posture semantics. The existing Charter
authority transaction remains the physical canonical-write owner. Charter is
constitutional authority; posture is a derived kernel; the six Context
Resolution dimensions remain unrelated to the nine engineering-posture
dimensions. Snapshot/grounding inputs remain exact redacted evidence pairs.
HCM-5 remains the only contract-gate verdict and parent-promotion owner.

This selector does not authorize partial P1/P2 work, product implementation,
public schema/API/transport, adapter delivery, Flow/pipeline adoption, HCM-5,
release, protected-checkout work, or remote publication. A later source
selector must admit the complete P1-P4 route and all acceptance proof together.

## Planning subject and review population

The complete reviewed planning subject is the current `SPEC.md`, active
`tasks/plan.md`, `tasks/todo.md`, this selector, its paired source-impact and
proof-matrix records, and the exact synchronized HCM-3.6 sections in `04`,
`05`, and `06`. The outcome registry contains one tuple:
`hcm-3-6-atomic-authority-replanning` /
`HCM-3.6-ATOMIC-AUTHORITY-REPLANNING` / this selector.

One fresh complete-subject discovery review is admitted. Valid P1/P2 findings
receive one consolidated remediation and a different-fresh delta closure; no
review runs after CLEAN. P3/P4 follow `09`. Mechanical handoff/ledger closeout
is outside the reviewed semantic manifest and must be a separate commit.
