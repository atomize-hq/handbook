# Windows platform: complete 27-file source review

Verdict: **REMEDIATE**. The 10:34:23.782700Z replacement freeze has three P1 source blockers. It is not accepted for runtime admission. All166 cases and all64 prepared memory groups remain unexecuted/PENDING.

Root: C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/windows-final-platform
Closure: 454455 bytes /27 regular files. Receipt6089 bytes / SHA256 6617f3ea8b06d223fdeef4908d3f1af9d693472b76dc3671596118005c5c384b. Canonical compact sorted local26 map, no LF: db239b7936802ba5e933466e3888b14bb3d30a677d87b65adaed812e7902693d.

## W1 [P1] Admit the exact reviewed checker CLI within the caller's source-read bound
windows_capsule.py:525-527; windows_growth.py:29-40.

checker_capture calls regular_read(path,32768), then verifies the returned bytes against the reviewed CLI descriptor. The accepted current checker is34171 bytes, SHA a49175b347445f79360c9ab90be043377fb8683fc6d9b0c781dce6245cfcabd5. regular_read rejects its size before reading it. Thus every checker mode, including the first full/legacy oracle and refusal routes, stops with bounded-regular-read before creating its request or launching the checker. This also applied to the prior33763-byte CLI.

Repair: derive a finite read bound from the exact parent-selected reviewed CLI descriptor and its approved source allocation, validate that bound, and keep exact byte count/SHA verification. Reconcile the descriptor and source-policy closure with the accepted checker freeze. No arbitrary unlimited source read is needed.

## W2 [P1] Compare legacy restore progress against the archive contract, not stream capacity
windows_hooks.py:869-878; windows_capsule.py:168-170.
Dependency legacy_archive_restore.py:146-168,265-271; long_lane.py:410-423,469.

The archive session derives its c.max_destination_bytes and c.max_files from the authenticated original archive, fixed592458971 bytes /5807 files. Its phase() passes those exact remaining values to host.checkpoint('long:legacy-restore',...). The host already holds a matching long-archive lease.

legacy_forecast instead subtracts current source inventory from long_config.stream.max_destination_bytes/max_files, which bound the later converted stream stage. An admitted stream envelope651179227/7908 yields a different expected request even when the fresh source has zero bytes/files. The actual first archive checkpoint consequently fails actual-legacy-remaining before restore starts. Increasing a prospective later stream cap should not invalidate the fixed legacy archive phase.

Repair: bind this checkpoint to the authenticated original archive/session totals and actual restore progress, preserving its existing archive lease and zero new incremental growth. Keep the later stream envelope independent. Do not duplicate the archive root or introduce deletion/expiration credit.

Static dependency bytes: legacy_archive_restore.py14204/79e7ccac897d4c38aa53bbecbb049c7e6d327e834cf806b77c1a22c4d7f826bc; long_lane.py32413/33ebdba457002c2b1e2b5b3d9ff6e64985e570b1621769179f4e59fa1106fa60. No archive was opened or restored.

## W3 [P1] Include required capture data in the closed bootstrap manifest
windows_bootstrap.py:50-54; windows_capsule.py:60,108; windows_adapter.py:579-598.
final-engine-bindings.json runtime_manifest_fields repeats the omission.

verify_manifest accepts exactly fields that omit capture. LaneHost then calls verify_capture_bounds, which unconditionally reads host.manifest['capture']['guard_bounds']; initialization later reads the same member again. A manifest containing capture fails the closed shape check; one omitting it fails initialization with KeyError. No parent runtime JSON can satisfy both interfaces through the documented bootstrap.

Repair: reconcile the closed bootstrap/runtime manifest contract with the mandatory capture.guard_bounds structure and keep the exact sealed inner/outer budget comparison before Evidence opens. Update the binding description together; do not simply bypass verification.

## Complete source and data coverage

Read every line of all11 Python files: source_controls649; adapter600; bootstrap158; capsule736; deadlines99; driver681; evidence367; growth385; hooks878; native796; oracle_bridge307. The WIRE_CONTRACT literal and all prepared fake-control paths were read as source, not executed.

Read all8 case-data files: protocol, packing52, recovery24, strict24, focused_windows, workspace_ordinary, source_constructors, growth_actions. The six case maps contain48+52+24+24+12+6=166 unique IDs, all PENDING at engine062. All assertions/routes remain obligations. Null constructor/review/growth/runtime fields are unreleased data; they are not proof of qualification.

Read all8 binding/document files including receipt: README, interface, platform-source-review-notes, source-bindings, final-engine-bindings, controls-forecast, static-controls-result, source-receipt. The exact26 map matches all actual files. Some named historical/binding descriptions retain older unreviewed/null descriptors; actual parent runtime/source acceptance and quantitative data are still prerequisites, not supplied by these progress records.

Relevant external frozen byte-provider source was also traced: stream_snapshot.py34729/f8d223618d869bf85a11ec648254830e643aa1abbb86374233ac9f31fe46cc20 Runtime165-260 and produce/consume/capture426-733. Archive/session and LONG caller ranges above were read. This is dependency tracing for the new provider, not a fresh full qualification of all unchanged external code. The accepted checker34171 and unchanged14-source closure retain the prior source review; this Windows review catches its caller size mismatch.

## Concrete paths that are now present

The two withdrawn-freeze LONG defects are fixed in these exact bytes. OracleBridge checker_settlement282-285 publishes checker_results after successful output/gate settlement and before return. Capsule run_lane698-720 hash-reads the original LONG RangeRef, keeps parsed body separately, and dependency723-736 rehash-reads the same reference before using it. These fixes do not close W1-W3.

D dispatch supports the actual run(case,args) source constructor (Hooks580-591; Capsule307). NativeCase supplies the actual Harness/Adapter/case/root interfaces. Alias phase inheritance occurs before filesystem mutation (Adapter468-470), with a D-specific parent phase requirement and family closure/scope tracking. The corrected Matrix-owned D body is separately reviewed; its execution and final parent template binding are not granted here.

The new stream implementation is confined to one freshly hash-verified PRIVATE byte-module object. Runtime and Popen providers replace names only on that object; no stdlib global is patched by the runtime provider. Fixed two-worker role/argv/control-stdio topology is checked. The unchanged byte encoder/decoder use a dedicated inherited payload pipe rather than retained compressed transport. Control stdout/stderr go through real Driver raw capture.

Endpoint startupinfo carries only the source-owned payload HANDLE; ownership is creator-bound. Borrowed birth, close-once retirement, uncertain-close and creation-unknown paths fence further Driver calls. Callback ownership precedes thread start and native pipe ownership precedes acquisitions. Producer/consumer use the real startup gate and isolated -I -S -B -u launch, source/manifest/config refs, corpus-root gate binding, positive suspended identity/RSS and actual native START before release. Stream Runtime publishes helper refs only after both actual worker closures and both endpoint closures; frozen capture byte/inventory/CRC/EOF checks precede verifier access. Failure cleanup traverses proxies/callbacks/endpoints and retains evidence. These are source assessments; no Win32 ABI or creator/closure-fault qualification was run.

Driver uses retained creation-bound Popen/duplicate HANDLEs, exact current-Job membership, suspended launch, native RSS, phase/total deadlines, bounded overlapped I/O and final-tail capture. Cancellation/death is limited to exact owned reader identities, plus the authenticated S-maintainer-death grant. Cleanup isolates failures, retires termination capabilities before closing, records real EOF/reap/callback status and fences uncertain resources. Whole outer Job emptiness remains a separate post-host guard obligation.

Evidence v2 persists exact planned IN once, BEGIN/LAUNCH/START/data/RSS/END, strict per-call sequence/hash, source/image/clock/closure metadata refs, authenticated action/group profiles, scoped sidebands and actual callback/native-response joins. Native-call failure rows remain distinct from semantic refusal. Prelaunch aborts record no-child-started without pretending an executed call.

GrowthLedger anchors nonoverlapping roots once and holds all concurrent incremental leases. Settlement releases only unwritten potential after positive writer closure and keeps actual bytes. Checker consumption verifies final/partial membership, original request/index invocation refs, full/footprint unchanged proof, source closure, output bounds and actual payload. Known/quota/provenance refs and private helper source budgets require parent binding. The archive forecast error in W2 blocks the intended path despite those protections.

Guard-prefix reading retains a same-handle compressed prefix, source schema and completed cadence samples; its result stays PROVISIONAL and requires final guard joins. Inner/outer evidence bounds are verified before Evidence opens. Accepted source manifests and machine review acceptance remain separate from local progress receipts. All-sink actual limits, binary/build receipts and parent runtime data remain missing/unqualified.

## Method, scope and authority

Read-only static work only: standard-library source/data reads, JSON, gzip/base64 inspection and hashes. No supplied module imports, AST/compile, tests/controls, Win32/DLL/native calls, engine/build/checker/runtime, corpus/archive/fixture access, Git/index/bootstrap, source edits or source copies. Only this authorized report/evidence pair is written.

Exact-checkout GitNexus is unavailable; the owner's UNKNOWN graph/manual HIGH impact evidence is retained. No alternative repository graph was substituted. The structured data standard remains PROPOSED and existing private owner/versioned contracts govern. No migration/default or public format approval is implied.

The source070 budget2728/7e876aa667a0bcd4fa4168e2879a26eecccf4db530c8fe0a292a7e12cbe10b0b allows524288 bytes /27 files: source335872, case_data122880, bindings65536. Actual454455: source320782/11, case_data89098/8, bindings44575/8 including receipt. All individual/group/whole caps are checked again before report writing. No new source output is created here.

The new071 review pair alone is funded24576 bytes /2 files at P/windows-platform-final-source-review. Whole META v15 physical selectors include all P copies, specified Landing roots/subtrees/promoted files and exclude the original Windows source subtree. Reservation accounting from v15 and sealed076 (4317/d1d0ebac0924b1fb34d262d27afb430673ae4f7dbb0c351fc4892ed59e78435b), including074/076 holders and conservatively retained new unused portions, is recorded in evidence with a fresh prewrite census; own actual pair bytes consume that allowance once. No closed hold is revived.

After writing, hash both files, recount META and recheck the exact full27 closure/receipt/budget. Release the source read hold only after those checks. Repairs require parental preservation and authorization, new exact freeze and review; this report does not edit source. Engine062, functional11:10:14Z and closeout13:10:14Z remain fixed. No Native166/70/control/runtime GO.
