# Linux V5.2 independent full source review

**READ_COMPLETE / SOURCE_DEFECTS_NO_GO.** Two regressions require correction: a P1 memory-observation integration failure in the outer phase and a P2 missing live command-role memory check. This review grants no source publication, private import, native controls, cache copy, build, units or engine qualification.

Frozen candidate 2026-10-21T07:56:32.924Z: source_receipt.json 8152 B / SHA256 a39fbc029011dcb21ef46f03dd59391fab34240588da99a63e1d5976512651fb; suite_manifest.json 16073 B / SHA256 c189dc78904fb7a14e4540b355409a1e856186c1fed4c9ec23e284184584faef. Rust/core remains 0628322e4b48e0efad19cdc32238a622d4aaa7a8. The prior V5.1 source snapshot and negative review remain preserved. No old cell is rescored and no clock is reset.

## Findings

1. **[P1] Refresh memory for the same ownership-verified birth before returning tree members.** suite_driver.py:32 returns the original kept snapshot rows. Its outer caller, suite_adapter.py:530-531, obtains that snapshot from the actual frozen native_guard.processes(), whose implementation at native_guard.py:85-89 explicitly calls process(..., with_memory=False). Those rows therefore contain rss=None and hwm=None. Nothing in retain_birth or retain_tree fills them. The phase observer at suite_adapter.py:545-546 requires a positive integer RSS for every live returned row; its next lines sum those values. As soon as it observes a live phase child, it takes phase-owned-positive-memory censorship and kills the owned workload. Successful native process observation is mandatory, so this is not a missing optional metric and is not repaired by the independent caller-only observer. Finite8, cache and Cargo phases all use this outer route. Preserve the original ownership-observed start and parent association, but obtain current RSS/HWM only from a fresh row whose start still matches that original birth; refuse uncertainty. Alternatively collect a memory-enabled snapshot while retaining the same ownership proof. Do not restore the V5.1 mistake of registering an arbitrary replacement birth.

2. **[P2] Restore live command-role RSS censorship in Commands.memory.** suite_recovery.py:215-218 checks that RSS/HWM exist and records them, but the rewrite removed the previous row['rss'] > limit branch that marked command-role-rss-censor and killed retained processes. limit is now checked at the initial actual-exec proof (234) and reported in the final row, while later raw-packet verification checks the recorded peak after exit. A command can cross its role limit after the initial proof and continue running/writing until it ends. The outer 2 GiB heavy/host limits do not enforce the ordinary/reader command's 256 MiB role limit. Restore the live comparison and retained-handle kill while preserving the new ownership checks. This affects Commands routes in the full suite, including ordinary/workspace/oracle; it is independent of the finite8 blocker.

Both are static source findings, not claimed native reproductions. The first was sent immediately during the full read; the second was sent on tracing the rewritten command watcher. No candidate code was imported or executed.

## Full coverage and exact identities

All 13 current V5.2 files were freshly read in full, including the four changed Python files, seven unchanged Python/data files and both JSON metadata files. No partial output or hash-only scan was counted as a full file read. The complete caller/callee paths for tree retention, direct-child discovery, guard and phase finalization, native Commands, terminal proof, C1 and prior31 were inspected.

The 27 dependency files explicitly reuse earlier full reads in this same conversation after exact byte/SHA equality checks. These are the five actual frozen native donors, four shared files, seven harness/oracle files and eleven historical research files. Historical sources were checked via exact read-only Git blobs at 062, without extraction. The frozen native process()/processes() implementation was revisited directly for the P1 proof. No module was reconstructed or replaced.

Initial check 2026-10-21T08:07:52.245659+00:00; final check 2026-10-21T08:14:31.199195+00:00. All40 logical paths agree at both boundaries, all38 declared source rows match their manifest, and all27 unchanged dependencies match their prior full-read identities. Complete identities are retained in evidence.json. Authored13 =276504 B; NEW35 runtime-copy paths =586788 B; existing native5 =99512 B; logical40 =686300 B; external22 =310284 B.

All ten Python files passed stdlib AST parse and compile without import/execute. JSON was parsed as data. No package bootstrap, wrapper, index command, source edit, native/control invocation, build, Git mutation or new chat was performed for this review.

## V5.1 findings and retained terminal design

The ownership association finding is repaired at source level: retain_tree passes the original snapshot row, never a fresh replacement birth; retain_birth checks the parent edge, verifies a retained parent birth when appropriate, checks that parent live/start before and after opening the descriptor, checks the child's post-open start/ppid and fdinfo Pid, and publishes only after verification. observed_children verifies the parent birth before/after enumeration and validates each freshly read child's ppid. RSS and Commands scanners carry those parent rows and reject a reused retained birth. The new P1 concerns the memory fields returned by this otherwise improved ownership path.

The discovery-failure cleanup finding is repaired at source level: phase cleanup records discovery uncertainty as UNKNOWN nonempty closure, continues known-handle termination/reaping, attempts reader/observer joins and memory/descriptor closes, and permanently retains errors. kill_owned catches new-discovery uncertainty before signalling existing verified descriptors. Guard finally separately attempts its cleanup steps even during an active exception. A failed join causes failed process exit after cleanup attempts; no failed or uncertain closure can become PASSED. These are code-path assessments, not observed failure-cleanup qualification.

The bootstrap caller observer still remains live through gzip/proof parsing, hashes, own receipt write/fsync and inclusive census. Its positive RSS/HWM and <=40 ms cadence checks cover the caller through substantive final work. The outer observer is intended to cover owned aggregate/role memory, subject to the P1 integration failure.

The final on-disk receipt remains PENDING_TERMINAL. The bounded terminal handoff still hashes that actual receipt while the caller observer is live, then joins it and validates an overlapping 2..256-sample tail plus synchronous RSS/HWM endpoint. It emits exactly two stdout records under 16 KiB with <=4096 B writes and post-write memory/cadence/deadline checks. The absolute OS alarm stays armed through immediate exit0. verify_phase_terminal requires the parent-sealed actual returncode0 and native witness path/hash, exact two records, receipt/manifest/witness associations, caller birth, overlap, positive bounded memory and cadence. Source review alone cannot supply that native completion witness.

C1 and prior31 consumers explicitly equate terminal.receipt_sha256 to their actual referenced receipt SHA before terminal verification. C1 remains default-false and source-bound to the 45 locked archives, 32 unpacked roots and sparse/config closure:1653 files/39205355 B, manifest SHA7f0443c355439d2a45ddac9b7ce646f5c4067ffaf94493d61e039e9dcfa90f78. Prior31 reuse requires exact frozen sources, all31 identities/full raw results and unchanged full fixture census. No automatic heavy rerun or substitution of old passes is authorized.

The own-receipt fixed point, exclusive evidence-writer lock, write/fsync and checked actual post-write census remain. The retained closure prefix followed by a numeric empty pair remains required. These repairs do not turn old failed/censored artifacts into positive evidence.

## Admission boundaries

Finite8 remains NO_GO because of the P1. It still has seven resolve calls plus one packing/build against the existing peer, forty pre-peer refusals, typed incomplete-input and BrokenPipe negatives, explicit86 and native cancel/death closure with full raw reconstruction. It qualifies zero engine cases.

Cache/build/list/unfiltered31 have separate source-bound providers and require distinct actual source/tool/cache/target/fixture/phase admission and positive native terminal evidence. Fresh target ceilings remain conditional, not proof of compiler completion. Full158 remains independently NO_GO pending exact source releases and retained-growth feasibility; the unchanged coarse proposal is not a 512 MiB retained feasibility proof. Required31/158/17/native-pressure/compatibility obligations and retained prefixes are not reduced.

Before native source/control work, every original, future SDK/runtime copy, sealed authority/GO file, terminal native/local witness and return artifact must fit the actual all-physical-copy claims. The40-path logical arithmetic is not that certificate. Sampled disk censorship is not a kernel per-write quota.

Functional 2026-10-21T11:10:14Z and closeout 2026-10-21T13:10:14Z remain unchanged. The 04:10 intermediate checkpoint stays recorded as missed.

## Report allocation

Exactly two new reports in linux-final-source-V5.2-review, jointly <=24576 B, are authorized. Fresh META v9 census 2026-10-21T08:14:30.936623+00:00: actual10570174 B/1270 files plus other pending160957 B (Context included) and this new pair24576 B =10755707 B <=10878976 B; headroom123269 B. Membership SHA842fb0c2f51715ad4af257738df9dbabb6e97f74659aeccad0795f012ee49386; no aliases found. Old V5.1/R4 pair remainders were closed by parent and are not added again. Actual new report bytes consume this pair allowance once. Prior report files and original C3/Linux sources are not modified or double-counted.
