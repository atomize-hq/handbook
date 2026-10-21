# Independent frozen Windows045 review

**SOURCE NO_GO / REMEDIATE.** No memory/import/native/runtime GO. Bounded source improvements below do not close the actual executable dependency chain. Read-only review after the fixed source cutoff grants no authority to change code after cutoff.

Root: C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/windows-final-platform.
Exactly27 flat files /362547 bytes. Receipt5774 / SHA256580d7098ea13db2d52748a5d2f80d1705e4c60f0d54e4784476596cd23a16f9c.
Canonical26-entry manifest de21c589a22fa24906b86b21e0eeef60f5a43207689d95af39edcb636a5006a7.
All entry sizes/hashes and individual/group/whole caps independently verified. Recorded freeze2026-10-21T04:09:55.824302Z precedes source cutoff04:10:14Z.

## Source findings

### WF1 [P1] Prelaunch forensic abort raises NameError

windows_growth.py:278-291 uses time.perf_counter_ns at290, but the module never imports time. WindowsDriver.call:117-123 calls Evidence.abort_pending when a prepared call fails before completion. NameError therefore replaces the intended closed prelaunch row/NativeCallFailure reference, and finish_call is skipped. The pending lease remains held and stream close refuses.

The failure path also assumes Evidence's launch_value exists when native-call metadata is built at windows_evidence.py:178. A future authorized revision must cover failures before LAUNCH, source/binary verification, admission and partial writes. The new handshake name alone cannot establish complete forensic capture. No path was executed here.

### WF2 [P1] Frozen035 profiles cannot satisfy045 launch/metadata

Compare long_source_profiles.py:737-756 (c3a99f69...) with windows_evidence.py:66-69,189,219-234.

035 returns no argv_max_bytes, source_files_max_bytes or native_metadata_max_bytes.045 directly indexes those fields for LAUNCH/native metadata.035 also computes profile_sha256 per action at755-756, while045 requires that hash to equal one stream-global binding profile_sha256. Different action profiles cannot all equal that same identity without a defined source-bound mapping. No adapter/closed typed schema resolving these differences is supplied.

Coalescing and actual canonical DATA overhead improve the local bound. They do not close all BEGIN/LAUNCH/START/RSS/END/metadata/reference/helper/outer/finalization sinks or establish whole-phase fit. Arbitrarily adding fields or relabeling digests is not admission.

### WF3 [P1] Windows ports,036,035 and HISTORY disagree at mandatory joins

Verified current runtime_contracts.py de770285..., profiles c3a99f69..., history_lane.py100fe59f... and Windows hooks:

-036 _check_ports:426-439 requires native_meta_schema and case_observations_scope. Windows HistoryPorts.bindings:94-100 omits both; exact closure refuses.
-036 preflight:478-480 requires case_index_scope="closed-case-all-engine-and-checker-calls".035 history_phase_profile:1077-1098 omits it.
-036 seal_context shape:1265-1272 requires case_id/scope_id/case_scope and complete case start/end clocks. Windows seal_context:275-285 returns none, selects the last same-root case and assumes an active phase namespace. It cannot also seal six initial lookup/bootstrap cases.
-036 emits {id,body_ref} at1330. Windows append_phase_index:288 reads index["phase"], so that exact output raises KeyError.
-HISTORY:535-536 requires drain(...).closure containing actual owned closure, two empty observations and post-close census. Windows drain:323-325 supplies no such member/proof. These facts must not be synthesized.
-HISTORY now calls final phase-body sealing but never Runner.seal_case or seal_trace.036:655-693 requires six initial indices and one complete22-case trace; sixteen phase bodies do not satisfy it.

Windows guard_prefix returns window_start_ns/window_end_ns and omits budget_sha256/finalizer_sha256.036:1291-1300 requires phase_start_ns/phase_end_ns, exact budget/finalizer and owned drain links. Prefix remains provisional: no inside-host final Job/gzip/finalJSON/census qualification.

### WF4 [P1] Current LONG checker and quarantine APIs still mismatch

long_lane.py bc018fec... engine path:195 uses lane-invoke-v2, but run_oracle:283 sends lane-invoke-v1. LaneHost.invoke:210 requiresv2, so the first legacy independent oracle refuses.

LONG release_engine:416-423 requires engine_open_allowed/root/oracle_sha256 in host.authorize's return. LaneHost.authorize returns only go/lane/final_pin/config/claim/budget/source/receipt fields. Its narrow quarantine-release contract cannot be satisfied by that return. Exact stream/archive adapter/CLI/final engine and complete terminal-v2/quota-oracle closure remain unreleased.

### WF5 [P2] Mandatory case dispatch and Windows ABI remain incomplete

The final maps retain166 unique PENDING IDs,165 named entries and no D-N64-N1k-AandB-payload-title body. Concrete independent checks:

-X1000: LaneHost.run_case:281 opens harness.case, then mapping_stress:286 opens the same root again through shared focused_protocol.py:806. Shared case:221 refuses an active root. Current extension shared() also calls self-owning small/schedule bodies; route ownership is unresolved.
-CaseTemplates.dispatch:390 always passes two arguments; unit/ordinary run functions accept one. NativeCase/Adapter lack their budget/commands/emit and backup_restore interfaces. Ordinary remains Bash plus /usr/bin/python3, with no native PowerShell implementation.
-Inner-import rejection propagates to callers. Packing strict/focused, recovery selected_once/dispatch, boundary-unit and historical producer closures depend on rejected functions/missing providers. Constructor decoder/refuse processes, actual native unit catalog, historical Producer, derivative lineage and backup/restore are not implemented here.
-Current extension maintainer_death requests adapter.interrupt for packing; Driver.interrupt permits READERS only.
-Case maps still pin older packing a551bdec..., extensions26ca560f..., HISTORYca134929... and LONG416c98fa..., rather than the current frozen files. Constructor mappings also retain old donor bindings. Measuring current bytes does not replace those exact identities or make constructors reviewed.

The initial missing strict import is fixed. Other callable and versioned dependency gaps remain.

### WF6 [P2] Expected reader interruption becomes a drain failure

windows_capsule.py:313-314 records every retained handle.error as technical failure. NativeAdapter.finish_interrupted validates the exact same-call cancel/death AssertionError row, caches a closed proof, and leaves that expected exception in the handle. Later drain records failure and finalize.own_closed becomesFalse. A future revision needs an exact consumed-expected-row distinction, preserving true start/join/RSS/output/closure failures. This acknowledged initial defect remains in the final source.

### WF7 [P2] Memory fixtures miss new required Driver state

source_controls.py:292-297 creates WindowsDriver via __new__ but never initializes phase_limits. New sample:196 calls _phase_clamp, which reads that attribute. The callback regression fails before its requested callback fault; source_controls:336-339's exact-fault-path check rejects that different failure. Old64-group names do not establish that the current proposal runs.

The isolated new v2 trace prepares short-read coalescing, seq and env checks. It does not exercise complete launch/END/metadata/extract/abort,035 profiles, expected interrupt -> drain or runtime/guard/sealer joins. No tests were run and no finite-control GO follows.

## Bounded source observations

Useful source pieces, without whole-platform acceptance:

-W1-W4 native ownership changes remain: common lifetime lock, retired HANDLE capabilities, unconditional tail deadline, partial acquisitions and exact reader-end transport proof. Native/QPC25843 bytes /a4c02beb... is byte-identical to fully reviewed034, still native-unqualified.
-W5 ident-aware isolated joins plus bootstrap nested finally address the original unstarted-thread abort. WF6 is the remaining expected-error interaction.
-Evidence._write uses explicit frame membership; only frames change call hash/count/end, and frames after END refuse. Initial post-END metadata corruption is addressed.
-Short reads coalesce to16KiB plus one final partial. Accepted/written offsets remain distinct; failed physical writes poison the stream. No actual-byte credit.
-Evidence.close holds the append lock and refuses pending/poisoned writers. Host settlement now inspects actual registry/callback closure or the specifically closed owned record stream. Initial blanket-True/unfenced settlement is addressed in these inspected paths; whole forecast/schema agreement remains blocked.
-Prefix hashing now uses completed decompressed rows and refuses a last sample older40ms. The current bounded guard has matching nonce, suspended-root creation and completed-sample fields. This is source inspection, not native publication/cadence/containment/closeout qualification.

## Frozen dependencies and review limits

Full observed hashes/sizes for11 current external files are in evidence.json. Key current files:
03681001/de7702855dbb17b5cd00832aff457215ddcc4419bd77c3490faa1764c3eb932d;
03564788/c3a99f696786483989a5e170110eb483dd82953a5e0ec04867caaf3bf6b05797;
HISTORY54727/100fe59f34524b15c1033e5f1a93849a69766f36362ff37da3ef090278e02c3e;
LONG29946/bc018fecaed9a9c691506e1c2406eeb51f2c9a37181d89fb10059b6bc32bd433;
guard60355/c6dc7a797af9cc4288f29740818c224de7da6b2215084efd8224c138559feb78.

Reviewed all27 platform files through complete changed-body inspection against preserved initial/034 and full prior bodies for byte-identical files. Read current packing/extension case bodies, matching recovery/unit/ordinary, full current LONG, shared Harness contracts and relevant035/036/HISTORY/guard producer-consumer boundaries. Current observed files were not substituted into stale source bindings. No separate certification of every unrelated runtime/profile/guard function, their independent controls or finalizers is claimed. Complete executable transitive acceptance is impossible with null runtime_source_files/release/final engine, missing implementations and stale maps.

Case coverage:52 packing +24 recovery +24 strict +12 focused/Windows +48 protocol +6 workspace/ordinary. All166 PENDING and20 constructor obligations pending. Five identity slots are not a ceiling on actual final native tests. Four Windows sharing bodies have no-delete-share HANDLE acquisition, held-byte hashes, per-HANDLE close and release/retry source, but no native result. No required case or matrix is qualified.

No supplied module import, AST/compile/control/test/native/QPC/provider, engine/build/Job, source/Git/index/bootstrap, corpus/fixture write, delegation/chat/worktree or source copy. Standard-library hash/JSON/gzip/tar/diff source reads and this report pair only. Owner syntax/method hashes remain claims. GitNexus exact-checkout remains UNKNOWN/manual HIGH across transport, ownership, adapter/host/helpers and closeout; no graph count, symbol edit or commit.

## Admission and verdict

Budget0453435 bytes / SHA1e32a7be51fcef6520ce2785e66292ce115c585ffc6062575b12cb04c98a3800.
Actualsource213103/221184 in11, case102129/237568 in8, bindings including receipt47315/65536 in8, whole362547/524288 in27. All per-file caps pass.

Fresh physical META census 2026-10-21T04:19:16.453182+00:00: 9837450 bytes/1148 files. META10v5 seal ce01f5bc510f5472feaac941b82da16bf78b3c09d737231c4d2f2c9e4d01b3be, cap10485760. All recursive P copies/archives, closed Landing membership minus exclusions and exact20 promoted files counted. Full20480 report reserve +262144 future projects10120074, leaving365686. No deletion/relabel/expiry/compression/retroqualification credit.

**Final frozen packet remains SOURCE NO_GO.** Preserve negative packets/reviews and incomplete status. No postcut code fixes or runtime/import/control authority. Source cutoff04:10:14Z, functional11:10:14Z, protected closeout13:10:14Z on2026-10-21 fixed; no reset. Structured-data standard remains PROPOSED, no migration/default approval.

Budget reconciliation received during final verification: immutable meta-pending-20261021T041910Z.json (3119 bytes, SHA167eb2be9df2224654332076ce9992708a937401dfa14ad3f2ad05246335b81c) holds435863 total pending, including this Windows pair's20480. Generic future256000 and extra runtime12288 are duplicate balances, not additive. Fresh full census plus remaining435863 minus our actual pair fits META10. This pair consumes its matching allowance once; prior reserve arithmetic above is a historical prewrite check, not a second future grant. No budget sufficiency supplies runtime authority.

