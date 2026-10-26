# Minimal physical checkpoint resume repair — completed

The authorized repair is implemented in the standalone native Linux engine copy. A complete validated same-generation/same-parent work checkpoint now resumes its guarded commit after cleanup budget deferral, instead of deleting/rebuilding that checkpoint. Focused validation and one fresh independent read-only review establish **CLEAN for this bounded repair**. No pilot, full matrix, cross-host propagation or Git publication followed.

## Changed source and behavior

Only two files differ from the pre-repair baseline:

- [src/physical_lifetime.rs](/home/spenser/file-native-online-linux-20261020/source/final-engine/src/physical_lifetime.rs): recovery verifies the pending selector/plan and immutable generation identity, then uses a shared guarded checkpoint commit. It retains strict bounded parsing, digest/store/generation/ordinal/catalog/transition-parent checks and adds comparison of the remaining frozen selector/generation fields before promoting persistent staged state. Removed retained generations need a fresh native-pin drain check. Recovery syncs the immutable plan directory before selecting a previously linked candidate. Q/W exclusion, actual STORE-parent CAS, selector-byte revalidation, rename and destination-directory sync remain in the shared commit. The original work-load return contract is preserved; a resumed result replaces stale caller work so that a subsequent save cannot overwrite recovered progress. Non-cleanup/prospective loads retain their prior non-promotion behavior. Partial work stages are not adopted.
- [src/physical_checkpoint_tests.rs](/home/spenser/file-native-online-linux-20261020/source/final-engine/src/physical_checkpoint_tests.rs): nine focused behavioral regressions using small new real filesystem fixtures, native fs2 gates/pins, production packing cleanup and test-only deterministic budget-expiry/parent-race hooks.

Production thresholds remain **50 ms / 1,000 entry checks** and **35 ms / 800 work checks**. No format, schema, public API, dependency, batch/watchdog limit, cache/transient contract or existing assertion changed. No syncing was removed. The added virtual elapsed clock and fault hooks exist only under cfg(test); without those test scopes the production clock remains Instant elapsed. This is a deterministic liveness repair, not a claim that real-device cleanup now meets 50 ms.

## Failing-before and passing-after evidence

The first test was written and run against original checkpoint recovery with only test-scoped deferral hooks. It forces budget expiry after a full candidate is durably installed but before guarded selection, resets the test clock for the next actual packing cleanup, and checks selected progress plus preservation of the candidate bytes/inode and absence of checkpoint persistence replay. The baseline failed with **1,782 checkpoint bytes rewritten**, against expected zero. Its complete original test source, runtime source, stdout/stderr and failure fixtures remain under this output root (`red-physical_*.rs`, `logs/red.*`, `red-supervision.json`).

The unchanged liveness-test body passed after the repair. Its body SHA-256 is `1d4129a0a6cb156e919ae293ad98d0df44e2a006bc9d217236c498a7d3bc78e8`. The final nine-test run proves the same behavior on the final frozen candidate: next real packing::run(cleanup) selects the staged plan, authoritative pending jobs advance 1→0 and reconciled_absent advances 0→1, candidate bytes/inode remain, checkpoint bytes_written=0 and file_syncs=0. Necessary directory-sync/selection activity is retained; zero file-syncs here means no candidate rewrite, not removal of durability requirements.

Focused cases also cover: partial work-stage refusal/non-adoption; sixteen malformed/foreign/stale ownership, generation, parent, catalog, frozen mode, path, digest and duplicate-key variants; actual native capture/writer contention retaining resumable state; actual parent change through a native writer before Q/W CAS; stale caller-save protection; ordinary non-cleanup load non-promotion; a real captured old reader remaining readable and pinning its retained generation through commit, with a fresh reader still correct; and refusal to drop a native-pinned older generation. No injected/mock PID or nominal lock receipt substitutes for these native handles.

## Commands and outcomes

All commands used installed Cargo, exact existing Cargo.lock/dependencies, locked/offline release mode, j2 and the new task-owned target. Test fixtures and temp paths are new directories under this output root. Exact argv/environment records are in `red-command.json`, `focused-command.json`, `production-check-command.json` and each stage's supervision JSON; full stdout/stderr are in logs/.

| Stage | Exit | Supervised seconds | Actual outcome |
|---|---:|---:|---|
| red | 101 | 24.867 | test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 31 filtered out; finished in 0.27s |
| green-liveness | 0 | 11.623 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 31 filtered out; finished in 0.24s |
| focused | 0 | 17.499 | test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 31 filtered out; finished in 5.66s |
| affected-work-bound | 0 | 0.438 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 39 filtered out; finished in 0.08s |
| production-check | 0 | 2.921 | non-test lib check passed |

The focused run has **9 passed / 0 failed / 0 ignored / 31 filtered**. The directly affected existing structural work-bound check has **1 passed / 0 failed / 0 ignored / 39 filtered**: 131,072 bytes persist, 131,073 bytes refuse before writing. No original test was removed, ignored or weakened. The original 31-test catalog remains present; most existing tests were intentionally outside this focused assignment, and no full unfiltered rerun or real 80,976,318-byte heavy fixture was performed. Pre-existing 31/31 results remain attached to the previous source identity and do not qualify this amended source. Compiler warnings remain in raw logs; no unrelated cleanup was made.

Non-test release cargo check --lib passed, verifying cfg(test)-free compilation. No amended production CLI was built. The prior pilot CLI remains the previous baseline+fixture binary, not a build of this repair. The actual final test executable is `/var/tmp/file-native-checkpoint-repair-20261025-01a16fd3/target/release/deps/file_native_incremental-f5485a3108d76dc6`, 4,709,416 bytes, SHA-256 `7b9a81d23692c60febc12f285eadde92a3fa6fa331c55a48549201f16198ab52`.

## Frozen identity, patch and preservation

New source identity: **0628322e4b48e0efad19cdc32238a622d4aaa7a8 + pre-existing telemetry fixture10 + checkpoint resume repair**, identified by a separate 26-file manifest, not a new Git commit or a claim of matching historical freeze.

- [candidate-manifest.json](/var/tmp/file-native-checkpoint-repair-20261025-01a16fd3/candidate-manifest.json) SHA-256: `43a298bb575b724187eea89de6d55176d67992c2ed3f025e78f8e46d90c981cb`. The frozen full code copy is candidate/; the pre-repair 25-file source copy is baseline/.
- [repair.patch](/var/tmp/file-native-checkpoint-repair-20261025-01a16fd3/repair.patch) SHA-256: `4ee1eceb2416a9c3d1124e453079992c46be6882e886693d8a56f93939909571`. It contains only the runtime repair and new focused tests.
- Current physical_lifetime.rs SHA-256: `36603ef499aa29234441f28b2095627ba8c93e3a1657a6f35aaa3a3198c4d8e0`.
- New physical_checkpoint_tests.rs SHA-256: `52edbe54705cea456b55d9b14f32b63582b2d3d0017ad392b7eb95954ebe4824`.
- Existing telemetry-corrected io.rs remains `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`; the other 24 existing files except physical_lifetime.rs are unchanged. Original SOURCE_FREEZE remains SHA-256 `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3` and is historical authority for the original import; it was not rewritten to bless this patch.

Both prior pilot families remain intact: all **657** files in the first failed attempt's evidence manifest and all **277** in the bootstrap-corrected failed attempt match retained hashes. Their stores/logs/results and the read-only eight-batch assessment were not modified or rescored. The earlier corrected 31-test executable remains SHA-256 `bc90a7eaf47eb246ea2c26a07f7a6db0fc8bbd7659f782a981d9be31ac70b790`. No existing build/cache directory was used as this task's target. Current source was checked against the frozen candidate before/after review; no concurrent work was reverted.

## Standards, impact and independent review

The TDD and code-review skills were applied. The human assignment explicitly selected the physical checkpoint recovery/commit seam and the native reader/CAS safety cases, so no additional seam-approval step was introduced. Relevant retained proposed structured-data-format-standard.md was read: existing machine-owned JSON, exact serialization/hash/framing and owner activation semantics are preserved. No new representation, format registry, parser framework or migration was introduced.

Exact-copy GitNexus impact for recover_stage returned **Repository not found**, so graph impact remains UNKNOWN. No indexing/package bootstrap was attempted. Manual caller analysis covered load/save/recover_stage, cleanup/adopted work, build capture, activation verification and status in packing.rs; recovery promotion is scoped to cleanup and immutable generation/parent checks. The actual standalone source has no closer AGENTS/CONTEXT; the ancestor AGENTS describes an unrelated Next/FastAPI stack. The task's explicit bounded Rust/source-preservation instructions control this work. This is not orchestration V1 implementation.

One fresh read-only reviewer assessed the frozen candidate against both the requirements and relevant persistence/review standards, independently verified all hashes and directly affected callers, and found **no P1/P2 blockers and no P3/P4 advisories; required proof satisfied**. No repair cycle or extra final review was needed. [independent-review.md](/var/tmp/file-native-checkpoint-repair-20261025-01a16fd3/independent-review.md) retains the owner summary of that response; the original response is in local subagent task /root/checkpoint_review. The requested GPT-5.4/xhigh setting was unavailable and surfaced; the available reviewer_v1 used GPT-6.1-sol/high/default. No fast mode or new top-level owner was used.

## Resource and measurement limits

Existing finite native supervision retained the 20 GiB Linux free floor, 3 GiB aggregate RSS and 2 GiB per-process RSS limits, task-owned footprint checks, original wait statuses and two empty owned-tree closure observations. All five stages closed positively without a resource termination. Highest sampled tree-plus-supervisor RSS was 599,191,552 bytes. These approximately 200 ms samples are not hard peaks or 40 ms qualification. Stage records retain exact timings, native PIDs/groups, samples and closure. No old-reader256MiB process-role claim is made for these in-process native-pin regressions.

The new task output is roughly 111 MB allocated including small source snapshots, fixtures and records; exact final footprint is in final-preservation.json. Native free space stayed above the recorded 20 GiB floor during supervised work. Human combined44GiB/5million-file ceiling and inside reserve remain controlling; no protected cohort was restored/deleted or global census claimed. Scope included no Sentinel work, Git mutation/publication, cross-host edits, pilot, native matrix or qualification campaign.

## Next recommendation and delivery

A separately authorized next action would build a CLI from this exact amended source and run the single small A/N64/H3 pilot, retaining all actual 50 ms target misses and independent functional proof. This assignment did neither. The prior pilot remains **FAIL/PENDING**; deterministic regression success cannot promote it or establish real-device 50 ms, all-instant resource peaks, cross-host compatibility or full experiment completion.

Completion-time tool discovery found no supported send_message_to_thread. The substantive coordinator handoff is retained in COORDINATOR-HANDOFF.json; no outbound delivery is claimed.
