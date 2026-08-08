# HCM-3.6 atomic-authority future proof matrix

## Status

Every row is a future implementation obligation. No current Rust behavior,
test, schema asset, public API, or gate closure is claimed. The matrix is the
minimum acceptance wall for a later complete P1-P4 source selector.

| ID | Future case | Required result |
| --- | --- | --- |
| A01 | Each of the nine registered dimension IDs is selected. | The fixed index and RFC 6901 path match the selector table; the ID at that index is rechecked. |
| A02 | Current override is `null`, below/equal/above baseline, and proposed level is below/equal/above baseline. | Expected effective level is `override.unwrap_or(baseline)`; proposed storage is `null` iff proposed effective equals baseline. |
| A03 | Proposal targets baseline, another dimension field, a keyed pseudo-path, wrong index/ID, two dimensions, or a same-effective physical normalization. | Typed refusal before journal/domain write. |
| A04 | One valid transition is serialized. | Typed old/new Charter deep diff contains exactly one `level_override` leaf; every other field/order/value is equal. |
| A05 | Charter bytes, byte length, fingerprint, physical leaf, baseline, or effective level changes after admission. | Exact-byte/CAS refusal; canonical and both record partitions unchanged. |
| A06 | Recommendation/kernel/policy/evidence/freshness/approval/actor/reassessment input is missing, stale, mismatched, or unauthorized. | Semantic refusal before pending journal creation and no domain write. |
| A07 | Same PostureTransition semantic closure is built twice with different audit timestamps; a candidate posture record adds any resulting lifecycle pair; or construction tries to fingerprint lifecycle before freezing the posture pair. | Audit time changes only document hash, not posture fingerprint/ID/ref; the intent still binds the selected exact bytes. Every resulting-lifecycle field is unknown and refused in PostureTransition. Construction order is posture pair -> lifecycle pair -> intent, with no cyclic semantic dependency. |
| A08 | For PostureTransition or the posture intent, delete/add/rename a top-level or nested field; use an unknown enum, out-of-bound index/level/length/list, duplicate/reordered pair/ref list, forbidden null, malformed timestamp/ref/fingerprint, mismatched content-ref basename, substituted ID/semantic fingerprint/JCS+LF hash/length, or unequal basis/change/authority/reassessment/replay/output value. Independently substitute intent `basis_head.canonical`, intent `expected_canonical`, PostureTransition `prior_authority_head.canonical`, or PostureTransition `expected_canonical`; place same-ref/different-fingerprint pairs in descending fingerprint-byte order; or order different-ref pairs by fingerprint while reversing ref-byte order. | Closed decoding and exact derivation/equality validation refuse before journal creation, visibility, or replay; recovery rechecks the total basis/expected/prior-head canonical equality before rollback, roll-forward, or success. Every unordered `Pair` array admits only unique strict (`ref` UTF-8 bytes, `fingerprint` UTF-8 bytes) tuple order, never ref-only or fingerprint-only order. No record/canonical partition changes. Only the two stored `Change` values and the whole replay-freshness pair admit JSON null. |
| A09 | Promotion genesis -> posture -> posture -> later promotion is committed. | One merged chain, one current canonical head, most recent promotion ancestor retained, exact current lifecycle head selected. |
| A10 | Histories contain a cross-partition fork, cycle, duplicate successor, disconnected edge, wrong genesis, or more than 4,096 committed edges. | Current read, mutation, lifecycle observation, and promotion all refuse identically. |
| A11 | Lifecycle state is non-current or has active observations at posture admission. | Posture transition refuses; it does not clear unrelated lifecycle work. |
| A12 | Valid posture transition commits from current/no-active lifecycle state, then lifecycle `1.1` is replayed with each missing/extra/unknown field, nonempty observation array, unknown kind/state, bad bound/null/order, or substitution of policy/prior-state/prior-canonical/result-canonical/posture-authority/reassessment/time/ID/ref/fingerprint/hash/length in turn. | The valid record preserves `current`, binds the already completed posture pair, exact prior/result canonical heads, byte-equal singleton reassessment, and equal audit time. Every substitution refuses under the closed JCS+LF schema; lifecycle identity never feeds back into posture identity. |
| A13 | Lifecycle event is recorded after posture, then a candidate promotion is attempted. | Event chains from the v1.1 anchor; promotion consumes the resulting exact current head and emits an unchanged v1.0 promotion lifecycle transition. |
| A14 | Existing public committed-Charter read and Flow resolver inspect a posture head. | Current bytes/fingerprint and current lifecycle ref agree; `promotion_ref` remains the most recent promotion ancestor; no new public field/export is required. |
| A15 | Fault at every pre-intent/prepared boundary while canonical still equals basis. | Owned stages roll back; canonical and final records remain unchanged; rolled-back terminal is exact. |
| A16 | Fault after resulting canonical installation with exact staged/final records. | Recovery installs any missing final and rolls forward once to committed; replay is idempotent. |
| A17 | Replay absent/partial/exact named scratch, every admitted intent/snapshot/stage prefix, partial marker-prefix boundary, forward/rollback terminal, and exact-final substitution; then try an unknown scratch/root/journal name, non-regular/symlink/reparse entry, competing suffix, optimistic/reordered/contradictory marker, marker byte not equal to `sha256:<intent-bytes-hash>\n`, missing required byte, or cross-record ref/fingerprint/hash/length substitution. | Scratch never becomes recovery/head evidence and only exact verified scratch renames into pending. Only the exact canonical -> posture -> lifecycle stage prefix and prepared -> canonical-installed -> records-installed -> committed marker order advance. Exact basis/no-final states roll back; exact result with recoverable finals rolls forward once; exact terminal name sets replay idempotently. Every other observation preserves all evidence and refuses without guessed deletion, overwrite, or success. |
| A18 | Committed and rolled-back terminal matrices are replayed. | Committed requires new canonical plus both exact finals; rolled-back requires basis canonical and no transaction-owned final. |
| A19 | Resulting-kernel replay uses the retained normalized closure before commit and after recovery. | Exact resulting kernel ref/fingerprint equals the transition record; no raw snapshot payload, ambient time, or second authority is read. |
| A20 | Existing HCM-2.2 promotion, approval, lifecycle validation/event, authoring, and Flow regressions run unchanged. | All pass; existing record bytes/schema `1.0`, public APIs, and product behavior outside the internal head compatibility are preserved. |
| A21 | Rust visibility is inspected from an external test/crate. | `project_posture` and all new records/transaction types are inaccessible; only crate-local tests reach the seam. |
| A22 | Public/transport/schema/dependency/configuration diff inventory is replayed. | No definition/schema asset, Cargo change, public export, operation/DTO, CLI/SDK/Tauri/Substrate/Flow/HCM-5 edit, fixture, or dependency delta. |
| A23 | Context Resolution and engineering posture identities are cross-compared. | No conversion, shared enum, rank mapping, or authority transfer exists. |
| A24 | Gate/promotion effects are inspected. | Recommendation/transition/lifecycle completion creates no HCM-5 verdict or parent promotion; HCM-5 remains sole owner. |

## Required command wall

A later implementation must run focused crate-local mapping, semantic,
transaction, crash, recovery, chain, lifecycle, later-promotion, and privacy
tests; unchanged HCM-2.2 approval/promotion/lifecycle/Flow suites; engine and
full-workspace tests with all targets/features; formatting; strict Clippy;
rustdoc/doctests; platform checks already required by the Charter transaction;
package/archive and forbidden-scope scans; exact allowlist equality; no-secret
and whitespace checks; GitNexus staged/scoped detection plus an honest
compare-to-main observation; and a fresh complete-subject review.

The acceptance ceiling is private implementation evidence only. It does not by
itself close `PG-POSTURE-01` or `PG-POSTURE-02`, publish an API/schema, adopt an
adapter/consumer, or start HCM-5.
