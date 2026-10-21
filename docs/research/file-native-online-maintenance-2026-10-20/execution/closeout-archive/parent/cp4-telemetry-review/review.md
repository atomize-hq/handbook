# CP4 final v3 independent source review

**INTEGRATE — bounded source candidate only.** No remaining production-code blocker found in freeze `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`. This does not authorize build/test/pacing execution or establish native/full-matrix qualification.

The author manifest is [cp4-final31-source-freeze-final-v3.json](C:/Users/spmcc/.codex/visualizations/2026/10/20/01a15b15-3362-7133-b789-1ff556b75d04/cp4-final31-source-freeze-final-v3.json), 9,252 bytes. All 30 files independently matched length/SHA twice; total 478,740 bytes. Base is a3f9805eccc19ede8dd5c735b54e675e98e63714. Ten existing Rust paths carry the telemetry amendment; author delta 7,817 <=8,192 bytes against the boundary29 freeze. Cargo/package/dependency/profile inputs remain unchanged. Five checkpoint controls total 77,161 <=81,920 and fit their approved individual caps.

Review covered actual counter event sites, every W/Q-exclusive acquisition and drop/error path, direct filesystem/hash paths and callers, body read/classification, immutable Work I/O, Discovery validation, six new production boundary constructors, two positive telemetry units, complete control/forecast/README/protocol sources, and the prior accepted pacing implementation. Full io.rs and rolling.rs were read; affected production functions and their call context in all ten Rust paths were examined. Existing unchanged code carries the prior reviews' limits. No claim of graph-backed impact coverage is made.

## Findings closed before final freeze

1. Projection execute_plan still directly removed empty view directories at projection.rs:357 while Store::select_locked could own W. Final v3 calls the existing io::remove_dir hook there; operation/result handling is unchanged.
2. Store::checkpoint walked predecessor bodies under W at store.rs:440 without a direct history event. A corrupt/cyclic predecessor could return before later entries/bulk hooks. Final v3 calls io::forbidden on each entered predecessor iteration, before the body access.

Both were reported before the source cutoff and independently inspected in the retargeted freeze. Observed final source mtimes: Store 2026-10-21T04:09:56.8156005Z; projection 04:08:18.2043880Z. Author final write assertion is 04:09:56.816601Z, before 04:10:14Z. The old 9d49b3df and intermediate 7757d1fc manifests remain retained. No reviewer source edits occurred.

## Open documentation/configuration discrepancy

**P2, nonblocking for source code:** forecast.json pacing.new_control_file_cap_bytes is still 8,192 and README:307 still says <=8,192, although pace_controls.py is 11,490 bytes. The approved current checkpoint per-file cap is 14,336, also stated in forecast.source_amendment.checkpoint_individual_caps. The proposed correction hit the deadline guard before its first write; no post-cutoff source edit is authorized or implied.

Neither control.py nor pace_controls.py reads that stale forecast limit. control.py selects explicit reserve values and delegates admission to the SHA-bound budget/guard; pace_controls.py requires its admitted platform adapter before fixture initialization. Thus this discrepancy does not create an additional 8,192-byte runtime refusal in these sources. A consumer treating the stale forecast field as current authority would get the wrong result and must use the sealed current admission contract. Correct the explanatory/current-cap declarations only in a future separately allowed source increment. Other historical stage prose is superseded by the explicit final telemetry/boundary sections.

## Telemetry assessment

The nine new scalar Metrics fields are u64 and actually serialized; discovery_context is skipped. These are measured events, not constant zero aliases.

- Successful bounded packed and loose publication body returns call record_body, including eligible's second read. Head-target, suffix(expected sequence/prior completed cutoff), and explicit deferred contexts differ. Only suffix at/below completed increments unchanged_historical_body_opens; explicit deferred body opens have their own field. Rc/Cell RAII restores previous context on success, error and unwind without unsafe pointers. These body counters are per Metrics/discovery scope, not a claim to count every historical body read in arbitrary semantic operations.
- rolling_deferred_rechecked snapshots actual rolling_deferred_visited, an explicitly documented same-event name, rather than manufacturing zero.
- Completed native Work-plan/work-stage reads and writes increment process-global byte totals at the actual I/O completion sites. Their sum is the WHOLE Work envelope upper bound for Discovery metadata; it is not a literal Discovery JSON span. Production validation's actual canonical Discovery serialization bytes are separately counted.
- GlobalGate is created only after real native W/Q-exclusive acquisition. It closes the native File before decreasing depth. Both writer acquisition paths and both Q-exclusive paths are covered; errors/unwind close guards. Process-global ownership union catches nested/fresh Metrics; independent reader processes remain permitted.
- Actual directory opens, directory/file deletion attempts, copies, Disk bulk/range/entries/reachable/audit, full legacy index read/hash/parse, whole-file paged audit and cohort hashing are hooked. Small legacy indexes are counted independently of the supplemental >64 KiB generic digest hook. Paged Reader.audit reaches verify_file, so its full-tree audit does not bypass classification. Bounded control/head/point-route operations are distinguished; no blanket context/hash exemption was added.
- finish snapshots process-global cumulative counters without reset. API main finishes on success and error. Resident producer rows can therefore contain cumulative global totals, not independent per-row increments; downstream sums must respect this documented scope. Separate CLI calls start separate process totals.

The unused all_mapped full paged traversal has no hook and no callers in this freeze. It is not an active-path blocker; reactivating it would require an actual traversal event hook/review. Current forbidden event counts are a conservative event count (operations can trigger several hooks), not a unique-I/O count or an exhaustive OS syscall tracer.

## Boundaries and paced control

Independent static catalogue extraction found exactly 31 test declarations and matched every file/name in the final catalogue. All are UNRUN. The six boundary additions call production constructors/validators:
- 128 structurally valid deferred rows hit the byte cap; 129 hit the count cap. This is not 128 reservations (live cap remains 8).
- Legal padded Discovery serializes to 32,768 accepted /32,769 refused.
- Builder accepts 8 MiB raw and refuses 8 MiB+1 without mutation.
- Catalog uses 64 real tiny pairs and rejects descriptor 65. This checks count, not the separate 59 KiB serialized catalog cap.
- Work persists 131,072 bytes and refuses 131,073 before a write, with valid metadata names but no claim of 1,100 real garbage files.
- Real create-new installation refuses a stale/restarted owner, accepts fresh birth and verifies reuse only in the same live owner. Protocol adds real pack-index death, unfinished-journal refusal, abort/fresh-birth and preserved old output.

The two telemetry positives use a real writer lock/native directory enumeration/small full legacy index with nested Metrics and after-drop nonincrement, and a real accepted older publication under suffix versus deferred context. They are meaningful source constructors; source inspection is not a test result.

The previous no-cleanup control blocker is addressed: exactly four cooperative cleanup calls follow each of ten actual builds, with no retry/drain loop. Actual immutable Work hash/length/membership, <=96 KiB headroom checks, ordinal advancement, owned processes and ordinary-reader milestone bracketing support the required absent middle plan while the initial pin is held. This makes the finite workload plausible, not guaranteed to meet native cadence/backlog deadlines. Original immutable origin/global ordinal, 40,000 warm +40 first queries and raw Driver stdout retention remain required.

## Qualification and resource limits

No build, typecheck, test, Python import, native engine, Git, indexing or source/fixture mutation was performed. No heavy test was run. The ~80,976,318-byte heavy fixture remains once-only on the final reviewed Rust under separate admission; all 31 native test receipts, protocol/crash/ownership behavior, actual cache/RSS bounds, measured D/H deltas, same-clock native adapter, paced middle cleanup/cadence, and the full native/long/backup/restore matrix remain pending.

The real NativeOwnedDriver/Native4 ownership registry adapter is still mandatory before pacing. Its contract string is insufficient proof. The 4 MiB pacing evidence request still has zero reservation and a stated 1,709,823-byte deficit; no runtime authorization follows from this review.

GitNexus is unavailable/unregistered for this isolated checkout: impact UNKNOWN, manual risk HIGH. Review assurance is source/caller tracing. The structured-data standard remains PROPOSED; no public format/default/migration approval is granted.

This report's prewrite fresh META10-v5 census: 1139 regular files /9706306 bytes, including all P copies/archives and the full closed Landing membership plus twenty promoted harness files. With <=40,960 bytes for new review artifacts and a 256,000-byte future reserve, projection is 10003266 <=10,485,760. No deletion/expiry/relabel credit was taken. Older failure evidence, the temporary 464-byte checkpoint-doc overrun and all previous negative reviews are retained. Evidence records the second fresh prewrite census and exact report sizes.

The final source-read freeze was released only after the second all-30 hash check. No source reread followed release.
