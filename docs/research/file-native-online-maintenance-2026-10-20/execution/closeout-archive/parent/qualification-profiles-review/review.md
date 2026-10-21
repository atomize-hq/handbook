# INITIAL035 immutable snapshot — full static source audit

**REMEDIATE — INITIAL SNAPSHOT ONLY. No final-release acceptance or RUN GO.**

Fully read the two immutable files in P/qualification-profiles-review:
- long_source_profiles.py: 60286 bytes, SHA256 14b4c434c44d77fee1bb2e8d68dabf62b1779c0e84c6ad761515282db6d926d3.
- phase_finalizer.py: 41293 bytes, SHA256 8cca3a66468bdf2fb46ec71bec47bf59a3992ba4ced28ce45c8a099da2657c6f.

P is C:/Users/spmcc/.codex/visualizations/2026/10/20/01a15ab8-d85f-71b1-8d37-c3c5689790be. L is C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook. R is L/docs/research/file-native-online-maintenance-2026-10-20. Line references below identify these immutable INITIAL035 bytes; they are not findings against the owner's changing originals.

Context included the previously reviewed 027 helper, accepted v5 decoder interfaces/metadata, the entire existing shared-runtime-contracts/runtime_contracts.py, and the relevant native capture/Job/QPC producer paths. The structured-data standard remains PROPOSED; no format migration is approved. Only source text/hashes/filesystem metadata and official Job documentation were read. No reviewed import, compilation/AST, control, native process, engine, build, Git operation or source edit occurred. No agents/chats were created.

The old long API integration, source-expanded 12446-operation trace work, broad physical F05 NO_GO, missing metric aliases, and old 40/46/19 versus selected 44/48/23.6 policy are known pending work. They are not reported as new bugs. Today's mismatched body/wire producers may conservatively refuse before any join; this review does not claim an executed end-to-end bypass. Findings below identify independent arithmetic or positive-acceptance defects that schema/policy alignment alone will not repair.

## Required findings

### S1 [P1] Capture widths and auxiliary sinks remain caller-controlled

long_source_profiles.py:484–507 hashes every source_bindings file, but returns the caller's reference/raw/call/RSS/poll/QPC/final schemas without deriving or comparing them to capture_v2_schema:433–481 or any parsed schema artifact. The comment claiming byte-for-byte producer-schema binding is not implemented. _long_inputs and _derive_history_forecast call this permissive function directly.

Static counterexample: keep the valid producer source bindings, API names, 16384 chunk size and all closed top-level keys; replace the schema subtrees with {"literal":0}. schema_width accepts those values and computes one-byte values. The producer hashes still validate, while frame/receipt/sample widths no longer describe the producer. Thus a smaller E allocation can be admitted on a false serialization model.

The same ownership gap exists at 685–693: retained_artifacts requires eleven names but accepts zero bytes/files/directories for each without deriving their actual nonempty forecast/admission/sample/result/finalization records. The history artifact_sinks list also need not contain mandatory finalization/body/index classes. A key's presence is not an all-sink width proof.

Derive serializer dimensions, actual path/environment widths, cadence and mandatory sink multiplicities inside the source-owned calculator from reviewed producer contracts. If a schema file is the boundary, parse exact bound bytes and compare every consumed schema and dimension. Allow callers to allocate more capacity, not replace the minimum producer grammar or omit required sinks. Add schema-substitution and zero-mandatory-sink negative controls under separate finite GO.

### S2 [P1] frame_bound undercounts legal short pipe completions

long_source_profiles.py:536–542 assumes exactly ceil(payload/chunk_bytes) frames. That is a lower bound for a maximum chunk size, not an upper bound on framing count.

The reviewed windows_evidence.py:133–143 splits EACH incoming block independently into <=16384-byte pieces; it does not coalesce short blocks across callbacks. windows_native.Pipe.complete:327–341 returns the actual ReadFile completion byte count. Many one-byte completions are legal: 16384 payload bytes may produce 16384 data frames, not one. Base64 rounding and per-frame JSON/sequence/hash/offset/LF overhead grow with that actual frame count. The modeled single extra header cannot dominate them.

This affects every long/history raw sink, per-call transport envelope, and abort/drain escrow using frame_bound. Runtime prewrite caps may censor instead of overflowing; that does not make the source forecast or promised lossless failure tail valid.

Either implement/bind an actual coalescing writer with bounded partial buffers/flush semantics and escrow, or derive a safe frame-count bound from the real completion/producer contract (worst case one payload byte per frame if no stronger invariant exists). Bind the same frame limit to admission and capture. Test fragmented stdout/stderr/input and failure drains, not just full chunks. This is static serializer arithmetic; no pipe test was run.

### S3 [P2] Cleanup request input bound is nineteen bytes short

_basic_profiles:599 charges canonical({"action":"packing","operation":"cleanup"}) + canonical(root) +32. The 027 cleanup request also includes max_candidates=1000 and time_budget_ms=50 (long_lane.py:411), and invoke serializes the root field plus LF.

Relative to that base object, root's key/separator costs 8 bytes, max_candidates costs 22, time_budget_ms costs 20, and LF costs 1: total51, not32. The proposed input bound is therefore short by19 bytes for the exact intended request. This is distinct from the known old-long API integration mismatch.

Build the complete typed request object for each action and measure its exact selected serialization, including LF. Avoid undocumented fixed slack in place of actual fields. A valid cleanup request must fit its own source-derived profile before native input is released.

### S4 [P1] Final join does not establish the required native operation/semantic join

phase_finalizer.py:255–285 rehydrates recognized response refs, but accepts arbitrary inline evidence otherwise. _phase_windows:488–500 requires only a nonempty native_call_refs list, verifies those references/windows, and accumulates action counts. Those counts are returned, never compared with the required phase workload. It does not require every offer, reader batch, maintenance operation, terminal release and oracle to have a matching call, input and response in the raw capture.

join_history:614–623 checks a caller-provided native_invocations list is a bijection with captured calls, but source_operation is never interpreted against the source-expanded schedule. A self-consistent inventory of fewer/different calls still satisfies that join. Scalar evidence/summary counts are checked by history_module.validate_phase_evidence, not independently reconstructed from the actual calls. Rehydrating some refs does not authenticate unrelated inline result/metric/timing/reader/oracle claims.

An aligned-schema negative must pair plausible 100-slot/reader summary evidence with an incomplete native call set and require refusal. Do not treat this as a demonstrated full RUN bypass of today's known schema mismatches. It is a missing acceptance condition in the join algorithm.

Validate/rederive the canonical history plan and source-operation trace from bound implementations; distinguish native operations from coordinator records and bounded optional cleanup prefixes. Consume actual calls by exact phase/root/action/request/digest/role, enforce 4000 offers/1600 accepted/2400 authentic losers, all640 reader batches/640000 warm queries, and the required maintenance/oracle/control boundaries. Tie semantic results, metrics, PIDs/creation and clock intervals to those specific decoded responses. Reject unrelated, reused, omitted or inline substitutes rather than merely echoing their summary.

### S5 [P2] Phase attachments and observed prefixes are not bound to their phase

_phase_windows:475–486 reads owned_closure_ref/checkpoint_ref/case_closure_ref, but checks only positive booleans/scope. There is no comparison of their phase/run/PID+creation/source/budget/window to the body, no linkage of owned_closure_ref's exact hash to guard_observation.owned_drain_sha256, and no proof that checkpoint's census is the applicable phase/caps/reservations. The same unrelated positive attachments can be reused across all16 phases.

Likewise the provisional observed-prefix hash is not recomputed/linked to the exact final QPC/raw prefix represented by this phase; sampled coverage windows alone do not authenticate the claimed prefix receipt.

Define closed actual producer schemas, bind every attachment to the exact phase/native identity/final source/budget/clock/census and verify the declared receipt hash links. Reconstruct prefix membership/hash from the complete final raw stream according to the producer contract. Add cross-phase attachment/prefix substitution controls. These bindings remain necessary after the known shared-body schema alignment.

### S6 [P1] Signed reservations and unchecked carried receipt amounts can understate census

census:504–545 does not validate nonnegative integer types for remaining_reservation, foreign_carried_reservations, per-root/combined caps or carried totals. A negative reserve offsets actual bytes/files in both admission arithmetic and combined totals. A negative foreign carry can cancel otherwise excessive local use. These are accepted arithmetic inputs, not merely a conservatively oversized forecast.

At539 the foreign receipt is read/hash-verified, but its decoded contents are discarded. Setting the carry to zero or another smaller amount while pointing at a valid larger ledger receipt is not rejected. The census roots/caps also need exact admitted budget/membership authority, not just arbitrary labels in config.

Validate all resource quantities as nonnegative integers (reject bool), caps as bounded admitted values, and explicit whole membership against the actual budget/grant. Match the carried receipt's identity, amounts, reservation status and source/budget to the used carry values. Preserve all physical copies and valid remaining holds. Test negative carry, negative local reservation, wrong receipt totals, missing roots and cap substitution. No deletion/reassignment credits should be introduced to repair fit.

### S7 [P1] A fresh nonce/current immediate Job does not prove outside-original-Job status

validate_join_inputs:561–579 accepts a digest-shaped parent_postexit_grant_sha256 without loading/validating the grant. It accepts current-owner.json with a different nonce and self PID, checks self creation/stage liveness and members()=={self}. The reviewed Native.members queries QueryInformationJobObject(NULL), i.e. the current immediate Job.

On Windows, a finalizer can be in a new immediate child Job AND remain in an original ancestor Job. Fresh nonce/PID/creation and an immediate Job containing only the finalizer do not establish separation. A forged owner/grant declaration also is not ruled out by a hash-shaped string. Correct original guard receipts may independently reject some such schedules; this local check does not prove the guarantee it names.

Load the actual bound parent post-exit grant and validate its source/run/original Job identity, closure receipts, new finalizer identity, resource admission and clock ordering. Require a reviewed native proof of nonmembership in the specific original Job, including ancestor Jobs, or an equally strong authenticated parent launch/closure mechanism. Retain the needed native identity/handles and refuse when original identity cannot be established.

This finding is supported by Microsoft's [QueryInformationJobObject contract](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject), which selects the immediate Job for NULL, and [nested Job semantics](https://learn.microsoft.com/en-us/windows/win32/procthread/nested-jobs). A specific-handle membership test is documented by [IsProcessInJob](https://learn.microsoft.com/en-us/windows/win32/api/jobapi/nf-jobapi-isprocessinjob). No native membership test was executed here.

### S8 [P2] Raw native END peak and terminal chronology are not validated

raw_capture:203–238 checks exit/censor/EOF/digests/input completion, but does not cap or reconcile END.peak. It only requires qpc_end>=begin.qpc_start; it does not require end>=START.qpc and all native RSS observations or check terminal peak against those observations.

A canonical short-call tape can therefore carry below-cap START/RSS observations and END.peak above the role cap, or an end counter before a recorded START/RSS, without this parser rejecting that contradiction. An outer20ms sampler need not observe every short process; checking only observed QPC members does not repair the missing per-call terminal checks. A Rust-reported peak_memory_bytes field is a separate measurement and can be absent; it cannot authenticate native END.peak.

Require typed native END counters, peak>=all observed native RSS/peaks and peak<=the role cap; enforce begin<=START<=each relevant RSS/write completion<=END, valid sample phase/order and required positive beginning/end lifetime coverage/cadence. Add short-lived high-terminal-peak and reordered-time negatives. These are inspected malformed-receipt acceptance paths, not observed production driver faults.

## What the snapshot gets right, and what remains unqualified

The scalar JSON width helpers explicitly count delimiters/keys/escaping; u64/u128 widths and base64 maximum-size accounting are useful building blocks. The principal defect is who supplies dimensions and the missing bound on fragment count, not a requirement to execute a configurable formula. Exact request serialization should replace the cleanup slack.

Geometry reuses the existing independent v5 Decoder plus inherited CachedPoints/Tree routes. _long_inputs reruns authenticate_geometry for an external geometry root rather than accepting a checksum-only cached receipt. The fallback MAX_HEIGHT63 bounds and receipt object hashes are conservative source inputs. No second semantic decoder was added. The complete v5 oracle remains a real output artifact; width acceptance must still be tied to its actual nested emitted schema and bounded shapes. Generic asserted scalar dimensions/pretty-print slack are not a runtime measurement or permission to substitute a compact PASS.

The long operation schedule explicitly includes602 lookup calls,606+K engine calls and five full oracles, with zero compression/deletion/expiry credit. The history model returns an honest F05 refusal when broad physical bounds cannot fit. The known operation-trace expansion, caller/producer API alignment and metric alias repairs remain owner work; no new bug is inferred merely from those refusals.

The finalizer reads all16 referenced bodies, exact body bytes/hashes/canonical records and raw lossless call spans. Rehydrated response refs re-read captured output frames and verify parsed response hashes. Gzip parsing checks one member, CRC/EOF, compressed hash, bounded expansion/line/row limits and rejects trailing/concatenated data. Clock conversion preserves integer-rational uncertainty as intervals. None of these correct checks substitutes for the missing operation, attachment and authority joins above.

Guard scanning checks whole-stream sample counts, cadence/completion gaps, per-observed-process/aggregate/supervisor caps, host-present phase windows and root/Job tails. Old QPC-v2 empty observations are tied to exact producer control-flow positions; the reviewed historical producer hash is 50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378. New guard shapes still require full source/schema review; a revision prefix is not provenance. Keep two numeric actual empty-Job observations and final receipt/raw source linkage, without inventing counters in historical records.

Census walks reject links/reparse/hardlinked input files, use disjoint named local roots and include foreign reservations. Fix S6 before using its totals as authority. The output route reserves one file/max bytes before exclusive write and leaves its own finalizer closure pending. after-output census currently keeps the same remaining reservation in combined totals even when prospective=False, so it can double-charge the output hold; this is conservative and not presented as a new false-PASS finding. Define explicit consumed-hold settlement and bound CLI stdout/failure output, and do not accept a durable join body alone after a failed finalizer exit/post-census.

Enforce the producer's live-call/aggregate-buffer bounds as well as per-call limits. Original-history closure does not qualify this finalizer's own run or other matrix cells.

## Verification and handoff

This was complete static review of the two INITIAL035 snapshots, with no helper execution or source edits. Current changing owner sources were not reviewed as a release. The shared/native context hashes used are in evidence.json. GitNexus remains UNKNOWN; no index or Git operation was performed.

The existing two snapshots plus review.md/evidence.json must fit131072 bytes/4 files; reports<=24576, snapshots101579 plus full allowance126155. Before EACH report write, recount exact META10 v5 ce01f5bc510f5472feaac941b82da16bf78b3c09d737231c4d2f2c9e4d01b3be: ALL P, closed Landing membership,20 promoted harness files/203215 bytes, >=256000 future bytes. P copies remain charged; no deletion/compression/expiry credit. Excluded C3 originals keep their own allocation.

Resolve S1–S8 in owned source, preserve the INITIAL snapshots, then obtain a new frozen source review. This report does not release old negative incidents, admit controls, or authorize engine/native/restore/archive/retirement work.
