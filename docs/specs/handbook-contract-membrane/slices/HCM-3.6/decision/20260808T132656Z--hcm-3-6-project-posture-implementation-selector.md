# HCM-3.6 project-posture implementation selector

**Selector ID:** \`HCM-3.6-PROJECT-POSTURE-IMPLEMENTATION-SELECTOR-01\`

**Packet ID:** \`HCM-3.6-PROJECT-POSTURE-IMPLEMENTATION\`

**Integrated outcome:** \`hcm-3-6-project-posture-implementation\`

## Authority, identity, and admission

This is the fresh source selector required by the operator-authorized HCM-3.6
implementation resumption. It is bound to task
\`019fe185-8682-72a3-9597-50398e97cd5b\` on host \`local\`, meta workflow
\`handbook-hcm-3-6-implementation-20260808\`, and dispatch nonce
\`8278ad5f3ce77ac927fe8679d17cde38ada79598a726cb9a066fe185042b7146\`.
It begins only from \`c38042434a66b083d57fedc5e99017a86d7ba7c6\` /
\`4a8a91d666019a704d7cc1cc039e60dd3ed31fa0\`; required ancestor
\`1256e724a2b7da6b6250f57d6f63fced1e2cf949\`; and dedicated local integration
ref \`refs/heads/orchestration/handbook-hcm-3-6-implementation-20260808\`.
Publication is local-only expected-old compare-and-swap. The protected checkout
\`C:/Users/spmcc/Documents/__Project_Code/handbook\` is excluded byte-for-byte.

The consumed planning closeout is
\`20260808T051657Z--HCM-3-6--orchestration--atomic-authority-replanning-completed\`.
Its terminal planning outcome, five-dispatch causal lineage, and CLEAN review
remain immutable evidence; this selector neither resumes that parent nor resets
or waives any finding. It establishes one new source-bearing implementation
outcome with its own v1.4 review population.

Before any Rust or test edit, this selector and its paired fresh source-impact
record must receive one schema-valid, different-fresh, complete-subject P0
discovery review with CLEAN and zero unresolved P1/P2. A review of the prior
documentation-only selector cannot substitute for this admission.

## Bound invariants and call boundary

Canonical \`.handbook/project/charter.yaml\` remains the only editable
constitutional authority. \`ProjectPostureKernel\` is private, derived, and
non-canonical. The nine engineering-posture dimensions remain distinct from the
six Context Resolution dimensions; exact redacted grounding pairs may be
evidence only. A recommendation is advisory. HCM-5 alone owns gate verdict and
parent-promotion behavior.

\`handbook-engine::project_posture\` owns private posture semantics, fixed-map
validation, normalization, kernel/recommendation evaluation, transition
admission, and replay. The private Charter authority transaction remains the
physical canonical-write owner.

\`\`\`text
exact Charter/profile/condition/contract/evidence/snapshot pairs + explicit basis
  -> project_posture kernel and one-dimension recommendation/admission
  -> CharterAuthorityTransactionServiceV1 under promotion -> registry -> lifecycle locks
  -> canonical Charter + PostureTransition 1.0 + lifecycle-transition 1.1
  -> private merged current head + re-resolved resulting kernel
\`\`\`

No public module/type/export, transport/DTO/JSON Schema, definition asset,
Cargo/dependency/configuration change, external test seam, Flow/pipeline/SDK/
CLI/Tauri/Substrate change, fixture change, HCM-5 behavior, remote operation,
or protected-path change is admitted. Discovery of any such need is a true
authority stop.

## Exact immutable mutation and record grammar

The selected dimension ID maps only to its fixed index and RFC-6901
\`/engineering_posture/dimensions/<0..8>/level_override\` leaf. The stored ID at
that index is rechecked. Baseline is unchanged; effective level is
\`level_override.unwrap_or(baseline_level)\`; proposed stored value is \`null\`
exactly when proposed effective value equals baseline. Same-effective requests,
baseline edits, keyed pseudo-paths, multi-leaf diffs, malformed IDs, stale
bytes/fingerprint/length, and every diff other than the selected leaf refuse
before a journal or domain write.

The private durable grammars are exactly the synchronized closed records in
\`SPEC.md\` and
\`05-contracts-schemas-and-gates.md#posture-transition-identity-and-atomic-recovery\`:

- \`handbook.posture-transition\` / \`1.0\`: closed top-level
  recommendation/kernel/policy pairs, target, \`AuthorityHead\`,
  expected/result canonical documents, \`Change\`, approval/actor/reassessment
  inputs, \`KernelReplay\`, resulting kernel pair, UTC-second audit time, empty
  extensions, and fingerprint;
- \`handbook.lifecycle-transition\` / \`1.1\`: closed posture-rebase fields
  with current/no-observation state, exact prior/result canonical fingerprints,
  completed posture pair, byte-equal reassessment, equal audit time, empty
  extensions, and fingerprint; and
- \`handbook.charter-posture-transaction-intent\` / \`1.0\`: closed basis,
  expected/change/authority inputs, canonical/posture/lifecycle outputs, replay
  closure, old canonical recovery document, and transaction fingerprint.

All use JCS plus exactly one LF, bounded closed objects, \`sha256:\` lowercase
fingerprints, safe bounded refs, exact content-addressed ref basenames, sorted
unique pair tuples by \`(ref UTF-8 bytes, fingerprint UTF-8 bytes)\`, and only
the two \`Change\` stored values plus replay freshness as nullable fields.
Construction is acyclic: PostureTransition pair first; lifecycle 1.1 pair
second; intent binding both exact bytes third; then stages, markers, and merged
edge. Posture identity never includes a resulting lifecycle field.

Under the existing lock order, the atomic group validates recommendation,
approval, reassessment, current/no-active lifecycle, CAS, exact deep diff, all
cross-record equality, and resulting-kernel replay before pending publication.
Recovery accepts only admitted exact pending states: it rolls back exact owned
state at the basis, rolls forward exact result/finals once, or preserves all
evidence and refuses. It never guesses, overwrites, deletes unknown data, or
reports an unsafe state committed.

## Current-head compatibility and existing-symbol ceiling

Committed promotion \`1.2\` and posture \`1.0\` journals contribute one bounded
heterogeneous no-fork/no-cycle/no-unreachable chain with a single promotion
genesis. Current reads, posture evaluation, lifecycle observation/events, later
promotion, and both recovery paths use the same terminal Charter. The retained
public read shape continues to expose the latest promotion ancestor through
\`promotion_ref\`, while its canonical bytes/fingerprint and lifecycle ref name
the actual current terminal. No posture head is fabricated as a promotion.

The only existing functions selected for modification are below. Their fresh
GitNexus upstream impact is paired in the P0 source-impact record; HIGH or
CRITICAL compatibility work requires the complete selected proof wall, not a
focused unit test alone.

| Owner | Selected private function | Purpose | Acceptance risk |
| --- | --- | --- | --- |
| Charter authority transaction | \`read_committed_charter\`, \`read_committed_charter_locked\` | project one heterogeneous terminal into unchanged public read shape | LOW, 4 direct callers for public read |
| Charter authority transaction | \`recover_pending_locked\` | recover promotion and posture journals before selecting authority | HIGH; approval/finalization |
| Charter authority transaction | \`validate_terminal_history\`, \`validate_terminal_inventory\` | retain promotion validation and contribute promotion edges | HIGH; approval/finalization |
| Charter authority transaction | \`current_committed_intent_locked\` | retain exact promotion lookup only where promotion history needs it | LOW |
| Lifecycle store | \`load_current_locked\`, \`load_canonical_bytes_locked\` | select one heterogeneous head and load retained 1.0/1.1 lifecycle authority | CRITICAL; event, retained-observation, recovery |
| Lifecycle store | \`current_promotion_anchor\`, \`retain_loaded_authority\` | replace promotion-only anchoring without public record-class expansion | HIGH / LOW |
| Lifecycle store | \`recover_pending_locked\`, \`recover_one_pending\`, \`record_event_inner\`, \`observe_retained_locked\` | keep recovery/event/retained reads on one Charter | HIGH / LOW |

\`validate_transition_shape\` remains unmodified; lifecycle 1.1 validation is a
separate private parser. \`parse_canonical_charter\`,
\`serialize_canonical_charter\`, \`resolve_profile_selection\`,
\`compute_freshness\`, \`charter_lifecycle_state_fingerprint\`, and promotion
workflow methods are read-only dependencies. Modifying any named read-only
dependency is a new-authority stop.

## Exact source ceiling and P1-P4 decomposition

After P0 CLEAN, only these product paths may change:

| Packet | Allowed paths and required proof |
| --- | --- |
| P1 private kernel/evaluation | new \`crates/engine/src/project_posture.rs\`, new crate-local \`project_posture_tests.rs\`, private \`lib.rs\` declarations; A01-A06, A19, A21, A23, A24 |
| P2 records/merged head | new \`charter_posture_transaction_intent_v1.rs\`, new \`charter_lifecycle_transition_v11.rs\`, selected authority/lifecycle hooks; A07-A14, privacy, public-read compatibility |
| P3 atomic commit/recovery | selected \`charter_authority_transaction.rs\` hooks plus new crate-local \`charter_posture_transaction_tests.rs\`; A15-A19 and no-write refusals |
| P4 compatibility/non-authority | selected \`charter_lifecycle_store.rs\` hooks and the same crate-local tests; A20-A24, events, later promotion, unchanged consumers |

The exact path allowlist is \`crates/engine/src/lib.rs\` (private \`mod\`
declarations only), the five new private source/test children named above,
\`crates/engine/src/charter_authority_transaction.rs\`, and
\`crates/engine/src/charter_lifecycle_store.rs\`. Existing integration tests are
regression commands only. No other source, test, fixture, or control-pack
semantic path is admitted after selector CLEAN.

The final code-bearing review subject must include a byte-hashed GitNexus
compare artifact naming baseline \`c38042434a66b083d57fedc5e99017a86d7ba7c6\`,
reviewed primary tip, actual changed paths, nonempty symbol/risk results, and
raw-output fingerprint. The required comparison to \`main\` may reflect
historical divergence and must be reported honestly rather than as a focused
green result.

## Required proof, review, and stops

The A01-A24 matrix at
\`proof/20260808T041700Z--hcm-3-6-atomic-authority-proof-matrix.md\` is binding.
Run focused crate-local mapping, record, chain, lifecycle, recovery, replay,
privacy, Context Resolution, and HCM-5 non-authority tests plus all required
unchanged HCM-2.2 consumer suites, engine/Flow/all-feature and full-workspace
walls. Run staged and compare GitNexus detection before the reviewed primary
commit. FTS remains unavailable unless it becomes actually available.

One complete-subject discovery review/burst, one consolidated P1/P2
remediation, and one different-fresh delta closure are allowed for this new
integrated outcome, with at most two immediate causal supplemental cycles. No
review follows CLEAN. P3/P4 follow the inventory protocol. Completion requires
one reviewed primary commit followed by a mechanical v1.4 handoff/ledger
closeout commit, local expected-old CAS publication, unchanged protected hashes,
unchanged remote-tracking baseline, and no push.

Stop for any new public/schema/dependency/configuration/consumer/test-seam
need, a path outside this ceiling, a fabricated promotion, an unsafe recovery
state, missing HIGH/CRITICAL proof, unresolved P1/P2, protected-path drift,
remote operation, or a validator failure other than the inherited HCM-3.5
ordinary-validator contradiction explicitly recorded by the operator exception.
