# FULL035 R2: frozen source review

**REMEDIATE. No source integration acceptance for the complete lane, no controls/RUN GO, and no runtime qualification.**

This is a fresh post-cutoff review of the actual ten qualification-lanes files, not a rereview of copied INITIAL035 bytes. Source cutoff remains 2026-10-21T04:10:14Z; functional cutoff remains 11:10:14Z. No source edits after cutoff are authorized by this report.

P = C:/Users/spmcc/.codex/visualizations/2026/10/20/01a15ab8-d85f-71b1-8d37-c3c5689790be.
E = C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution.
Q = E/qualification-lanes; R = E/shared-runtime-contracts/runtime_contracts.py; W = E/windows-final-platform; G = E/windows-bounded-guard/job_guard_bounded.py. All line references identify the frozen originals under these absolute roots.

Q is 252175 bytes/10 regular files under budget035 262144/10. All ten current Q files and runtime042 were read in full. evidence.json records exact hashes; INITIAL snapshots were not substituted. Dependency review inspected relevant actual Windows capture/ports/clock/extractor, guard receipt/lifetime and Rust NOOP selection/serialization/telemetry contracts; it does not claim full Windows/Rust implementation audits. All26 Windows manifest members and all30 named Rust/checkpoint members were independently size/hash verified. No imports, AST/compilation, controls, native/engine/build/archive/restore, Git/index or source edits occurred. GitNexus is UNKNOWN; the structured-data standard remains PROPOSED.

## INITIAL035 S1-S8 comparison

| Initial | Frozen R2 disposition |
|---|---|
| S1 widths/sinks | PARTIAL: literal wire grammar substitution fixed; mandatory auxiliary sinks/coordinator schemas still caller-controlled (B). Actual producer fails marker check (A). |
| S2 fragmented frames | REPAIRED IN SOURCE: Evidence.raw/flush_stream coalesces across callbacks into16384-byte chunks plus terminal partials. ceil bound now has an actual invariant. Unexecuted; marker mismatch still blocks. |
| S3 cleanup width | REPAIRED IN SOURCE: max_candidates=1000 and time_budget_ms=50 included. Old19-byte shortfall absent. |
| S4 semantic join | PARTIAL: exact workload counts,22 cases, unique consumption, raw offer/reader/maintenance links added. Oracle/release/origin replay incomplete (C). |
| S5 attachments/prefix | SUBSTANTIALLY REPAIRED ALGORITHM: actual rolling prefix SHA, run/source/window/drain binding and uniqueness. Same-attachment case_id contradiction blocks integration (A). |
| S6 signed/carry | Negative/bool quantities and decoded carry amounts FIXED; output hold consumed. Admitted membership/caps authority still unbound (D). |
| S7 Job separation | Initial nonce/hash shortcut REPLACED by parsed grant, original lifetime/owner audit, empties/close/completion and native current parent ordering. Conditional improvement; actual transcript/guard contracts unavailable or incompatible (A). No native/controller qualification. |
| S8 terminal | Peak type/cap/sample reconciliation FIXED; native chronology still incomplete (E). |

Original027 and INITIAL035 history remains preserved. Their known pending integration, conservative F05 refusal, unrun controls and absent native hooks/final build/runtime permission are not new successful bypasses. Below are specific final-source contradictions and residual local acceptance gaps; no end-to-end bypass was executed.

## R2-A [P1] The frozen producer/consumer contracts cannot complete the lane

1. **Profile preflight cannot accept its own calculator.** R:478 requires history_phase_profile.case_index_scope == "closed-case-all-engine-and-checker-calls". Q/long_source_profiles.py:1077-1098 returns no case_index_scope. R's exact ports binding at426-440 also requires native_meta_schema and case_observations_scope, absent from W/windows_hooks.py:103-111 bindings(). These are deterministic local refusals even if future grants are populated honestly.

2. **Observation, guard-prefix and sealer keys remain incompatible.** R:339-365 requires meta_ref, case_id, scope_id, case_scope, input_complete and start_observed. W/windows_hooks.py:192-205 _observation returns none of those six; both current and phase observation paths use it unchanged. R:1265-1276 requires the complete 13-field closed case context; W:274-287 returns the older eight-field context without the case identity/scope/window fields. W:242-264 guard_prefix returns window_start_ns/window_end_ns and prefix_ref/last_completed_rss_ns, while R and Q/history_lane require phase_start_ns/phase_end_ns plus source_pin/binary_sha256/budget_sha256/finalizer_sha256 and a closed shape. R:1330 sends append_phase_index {"id","body_ref"}; W:288-292 indexes ["phase"]. The actual provider has no normalization layer that reconciles these calls.

3. **Even ideal new ports cannot satisfy both attachment consumers.** R:1167-1176 authenticates proof.case_id against context.case_id; R:1269 fixes that context.case_id to b.case_catalog.history_case, the single parent case. Q/phase_finalizer.py:785-792 requires those SAME referenced raw proof bytes to have case_id == body.phase_id, each of sixteen distinct phase IDs. Both consumers cannot accept all sixteen attachments. A distinct parent-case and child-scope contract must be agreed; accepting renamed keys without bound producer evidence is not a repair.

4. **Calculator rejects the actual coalescer.** Q/long_source_profiles.py:524-532 requires the literal text "COALESCED_STREAM_V1 = True" in its hash-bound evidence producer. Actual W/windows_evidence.py has no such marker, although raw/flush_stream implements the coalescing needed for S2. This blocks both derived models on the admitted source. Do not fabricate an alternate producer or claim the marker alone proves semantics.

5. **Finalizer assumes a pure frame file; actual producer writes a composite tape.** Q/phase_finalizer.py:127-143 requires every line to be a closed v2 stream/begin/launch/start/data/rss/end frame. W/windows_evidence.py:69-76 calls _ensure_bridge before the first BEGIN, and :212-218 appends native-qpc-bridge-v1 with no frame kind. append also retains request-before-dispatch/coordinator records and native-call-meta-v2 on the same tape (:143-190). Thus raw_capture rejects the genuine bridge before any call can qualify. W/windows_oracle_bridge.extract_call already distinguishes per-call spans from sideband rows; Q does not use that contract. Any separate projection must be explicitly source-bound, lossless and allocated; no new uncharged copy is authorized here.

6. **Proof references and clocks describe different records.** W/windows_evidence.py:179-188 metadata points source_proof_ref to the stream header, image_proof_ref to START and closure_ref to END. Q/phase_finalizer.py:489-511 instead requires all three referenced records to contain call/run/executable_sha256 plus richer source-admission/image-handle/reap/writer-release assertions. The actual header has no call, START has no run/same_handle/contained, and END has no run/executable_sha256/raw_writer_released/raw_ref. case_call_pairs returns these original references unchanged. Likewise Q:307 expects a two-endpoint closed bridge with clock_kind/start/end/max_bracket_ns/sha256; W/windows_native.py:512-515 emits one flat qpc_before/perf_ns/qpc_after bracket, wrapped by Evidence in schema/run/frequency. A caller cannot honestly substitute either shape while preserving the actual referenced bytes.

7. **The actual guard receipt revision is rejected.** Q/phase_finalizer.py:353 accepts its historical QPC_V2 constant or a qpc-ns-v3; prefix. G:1138 emits sampling_revision="bounded-QPC-v3". The old-lifetime validator :887-943 also expects an original-job-native-lifetime-v1 transcript with normalized creator/query/flags/audit/host/empty/close fields; G's raw creator_lifetime/job_close at1235-1248 are different records. A separately reviewed parent transcript producer/normalizer is still necessary; raw fields and source flags cannot be invented. The guard controller's global-PASS qualification has separately been withdrawn; this review does not restore it.

These are frozen integration blockers, not observed overflow or false PASS. Some continue acknowledged integration work. The missing scope key and attachment-ID contradiction independently block the new runtime/profile/finalizer pairing.

## R2-B [P1] Mandatory auxiliary sinks can still be omitted or shrunk

Q/long_source_profiles.py:685-693 requires eleven retained_artifacts names but accepts zero bytes/files/directories for all of them. It never derives minimum sample/cleanup/result/failure/forecast/admission/finalizer/census/geometry artifacts from their producers. long_lane writes real forecast, archive proofs, samples, cleanup records, result and failure records (:443 onward); merely preserving zero-valued keys does not reserve those artifacts.

The history model :1021-1030 accepts artifact_sinks=[] and any caller nonnegative grammar/arithmetic without mandatory class membership or minima. history_phase_profile:1085-1094 accepts caller record_schemas for origin/slot/closure rather than literal producer schemas; the call-index floor alone does not establish all coordinator/body/global-index/closure/finalization multiplicities. This is the residual S1 ownership gap. Other caps or current marker/physical refusals may prevent a run; they do not make the local all-sink derivation complete.

Require source-owned mandatory sink membership/multiplicity/serialization/minima; permit extra allocation above them. Negative controls for zero mandatory sinks, omitted bodies/indices and shrunken coordinator schemas remain unrun.

## R2-C [P1] Counted oracle/release calls do not authenticate their claimed semantics

Q/phase_finalizer.py:550-652 now verifies substantial real data, but expected counts include one full-oracle and (P4) 300 terminal releases without consuming those responses to authenticate the phase oracle or terminal evidence. The only per-call semantic links in that function cover offers, reader results and build metrics/mappings. history_lane.validate_phase_evidence:235-283 validates terminal receipt fields and oracle booleans/H/receipt digest supplied in evidence; it does not read the counted oracle/release outputs.

The actual independent oracle output/receipt is not compared with evidence.oracle (H_end/intents/source/full hash proof), nor required after publishers/readers and boundary maintenance. A counted early/failing checker is not a quiescent final-H oracle. Each terminal receipt/attempt/charge/authentication also needs an actual release request/response/fence-byte link.

There is also only one ready/origin join at :629-638: ready_refs[0] and reader0/batch0. The other three ready refs are never read; all forty headers are not checked against the same fixed origin/anchor/frequency and first-batch native readiness. Runtime042 performs stronger live header checks, but this post-exit independent replay must authenticate those claims from retained records too.

These remain conditional replay gaps after ABI repair, not a demonstrated current bypass. Require final-oracle semantics/order, bijective release-to-terminal proof and four readiness/all-batch origin bindings; preserve full outputs.

## R2-D [P1] Census quantities are checked, but their admitted authority is not

Q/phase_finalizer.py:826-884 correctly rejects negative/bool quantities, compares decoded foreign receipt amounts, counts disjoint actual roots and consumes output escrow. However root membership, per-root caps, remaining reservations, combined caps and output allocation still come directly from config. validate_join_inputs:969-1000 validates a parent grant whose closed keys bind source/phase references and timing, but no digest/reference in that grant binds the census/output/limits allocation. budget_sha256 is an identity string on bodies/foreign receipt, not a parsed authority for the chosen roots or caps.

Thus omitted roots or enlarged caps are not detected against admitted authority. This does not allege intended parent misuse or acceptance by other guards. Bind exact membership, remaining holds and output/limits to parsed budget/grant bytes; preserve all physical copies and reservations without deletion/expiry credits.

## R2-E [P2] Native terminal chronology still admits contradictions

Q/phase_finalizer.py:183-226 orders RSS only against previous RSS and types its counters, without checking the sample phase against START. END at228 requires only qpc_end >= BEGIN.qpc_start. It never requires END >= START.qpc or END >= the last native RSS counter. The nonempty-input first_write check supplies a partial START-to-END constraint for engine calls; a checker has empty stdin and first_write=None, so END before START still is not locally rejected. A later-than-END RSS sample also remains possible. Outer guard sampling and perf envelope bounds do not impose the missing per-call order.

The initial above-cap END peak example is fixed at255; do not repeat it as current. Require typed BEGIN/START/sample/END counters, correct suspended versus running phases and all samples/writes inside their native lifetime. Proposed reordered-time negatives remain unrun.

## Preserved strengths and limits

Literal wire comparison, coalescing, complete cleanup fields,22-case inventory, raw timing/metric joins, prefix binding and the noncircular eighteen-field body are useful repairs. No planned-deletion credit is taken. Relevant Rust Store source checks stale expected selection before membership/outcome COW updates (store.rs:343 before357/360). The suspected stale-NOOP-loser COW growth path is NOT a finding. Source bound/minimum geometry and actual NOOP proofs remain prerequisites, not runtime results.

False/null examples and PENDING hooks/builds intentionally refuse. Pure controls were not run and cannot qualify full model/finalizer/native work. This report grants no matrix, archive/retirement, Context, source-freeze exception or controller acceptance. Deadlines,44GiB cap and protected bodies remain unchanged.

## Report allocation and handoff

Only P/qualification-profiles-R2-review/{review.md,evidence.json}, combined<=20480/2, may be written. INITIAL's four files stay unchanged. Before each write, recount ALL P/copies, Landing root files/admitted subtrees and exact20 promoted harness files/203215 bytes per META10-v5; only stream-snapshot-controls is excluded. No source copies.

META pending04:19:10Z (167eb2be) has435863, including this20480; other415383 exceeds the earlier256000 future floor. Consume the pair once. Generic256000 and extra runtime12288 are duplicate balances. First census04:22:54Z: actual9857193/1150 +435863 =10293056/10485760, headroom192704: budget-only report GO. Recount before the second write; evidence records that census.

**Handoff: REMEDIATE / source-only / all execution UNRUN.** No new source repair is authorized after cutoff. Preserve the failed review history and report the unresolved contracts honestly to the parent.

