# Linux cleanup: bounded decision on the frozen targets

This record reproduces the remaining drainage failure on accepted source `38e246500ff60e0d45821ec4de1ed11d886b3091`. It does not establish a successful product repair or complete Step 2. The accepted 26 source files remain authoritative under manifest `43a298bb575b724187eea89de6d55176d67992c2ed3f025e78f8e46d90c981cb`; no rejected optimization is promoted.

The first exact-two-call discriminator fails with four existing paths remaining: H2's result and G1's catalog, pin identity and immutable Work. H2's loose publication is physically removed. Selected Work still contains five jobs at cursor zero. Model times are 56.25/56.20 ms; actual elapsed times in the instrumented unit fixture are 77.165292/78.010771 ms. Those actual times include test observation overhead and are **not** an unmodified CLI pilot or native performance qualification. All original failure output is retained under `logs/` and in `red-sync-observations.json`.

The complete unfiltered workspace run on the restored, unchanged accepted26 source passes: 40 tests, zero failed/ignored/filtered, all workspace targets and doctests, with the actual 80,976,318-byte fixture. See `accepted26-unfiltered-workspace.json`, its exact command and `boundary-fixture.json`. This establishes Linux test results for these bytes; it does not qualify the repair on Mac/Windows, establish the final pilot oracle, or turn the drainage target green.

## Contract and feedback loop

The original brief calls 50 ms a cooperative batch target and two admitted passes the initial drainage target. `gates-and-watchdogs.md` freezes both for this experiment. Native calls are indivisible and their overruns must be reported. Nothing here changes the literal 35/50 ms or 800/1000 production limits, the caller's two final invocations, the backlog classifier or the independent oracle.

The regression uses the real `api::run` packing cleanup boundary: initialize a small A store, establish G1, publish H2, capture a real G1 reader, establish G2 and H3, assert the old reader and H2 loose fallback survive cleanup, release the reader, then issue exactly two cleanup calls. It checks actual H2 deletion and independently enumerates existing loose/selected-garbage and unprotected pack paths. The reader-held setup clock is zero as in the retained discriminator; each final call uses the **unchanged** profile from the rejected research: 10 ms per file sync, 9 ms per changed directory namespace, 50 us per unchanged namespace. Actual native syscalls still execute. A file sync credits its parent directory in that model; its subsequent unchanged parent sync is not charged another 9 ms. No profile revision was made after failure.

The diagnostic is kept as `reproduction.patch.gz`, outside the ordinary source/test suite. It adds only test observation hooks and one Linux test. `reproduction-apply-check.json` binds the decoded patch and a successful read-only applicability check. The original accepted repair patch and all earlier research are unchanged.

## Conditional cost accounting

The following is a lower bound for the **current checkpoint protocol with serial per-name namespace durability, evaluated under that frozen cost profile**. It is not a universal theorem that every safe design or every device must miss the target.

| Required work after reader release | Count | Model cost |
| --- | ---: | ---: |
| New selector and Work payload file syncs for even one checkpoint | 2 | 20 ms |
| Immutable plan-name installation directory barrier | 1 | 9 ms |
| Work-stage unlink in the separate progress directory | 1 | 9 ms |
| STORE selection replacement/root directory barrier | 1 | 9 ms |
| Five H2 loose names, in five distinct parent directories | 5 | 45 ms |
| G1 catalog, pin identity and Work, in three distinct directories | 3 | 27 ms |
| **Conditional subtotal** | | **119 ms** |

Clean parent barriers, reads, validation, recovery, CPU work, old current-root deletion and further cursor/garbage checkpoint publication are all excluded from that subtotal. The table does not add inclusive recovery/metadata/delete timing aggregates to sync timings. It counts operations in the protocol, not overlapping telemetry totals. G1 retirement must become selected before removing its catalog/pin/Work: otherwise recovery of the selected Work can require the missing identities. The immutable Work namespace must be durable before selecting it through STORE. Real old readers, the native drain recheck and parent CAS cannot be omitted.

Purely combining retirement saves or moving them around cannot fit even this favorable one-checkpoint subtotal into the two calls' combined 100 ms target. The actual code may save retirement, loose cursor coverage and metadata progress separately. Eliminating the cheap existing-directory root barriers does not eliminate the operations in the table.

Moving construction into the immutable directory and combining link/unlink into one namespace barrier is a possible narrower representation-preserving direction for investigation, **not implemented or accepted here**. Even optimistically removing the separate 9 ms Work-stage namespace barrier leaves 110 ms under this profile, before the excluded work. A design that batches multiple unlinks in the same parent can share a barrier; these eight loose/G1 names have distinct parents. The plan-install and old-plan-removal barriers are on opposite sides of STORE selection and cannot simply be counted once. Filesystem-wide journal behavior might make grouped directory syncs cheaper on a particular native filesystem; that has not been proved by this profile, this implementation or a portable durability argument. Neither native cost persistence nor the frozen model represents every conceivable storage design.

## Bounded disposition and product choice

Keep the accepted26 source unchanged and preserve the frozen-target **FAIL**. No repair candidate has met the focused discriminator, so no new CLI pilot is launched and no new interruption or final-oracle claim is made. The prior accepted repair's nine tests remain in the successful complete40 run; the rejected optimization's missing immutable-install interruption proof remains a gap for that rejected candidate, not a new claim about a repaired design.

The owner must choose between (a) accepting this as the measured negative experiment outcome while preserving correctness and deciding KEEP/ADJUST/PIVOT, or (b) selecting a further bounded product change that actually reduces the durability dependency graph. Examples requiring a separate concrete design/proof are shared deletion namespaces or a checkpoint representation avoiding the second durable payload; neither is a routine scheduling edit. The user has not authorized silently weakening the gates or changing the format to obtain a pass. A reviewer is asked to challenge whether a smaller safe mechanism using the existing immutable representation was missed before accepting this bounded negative conclusion.

If a supported mechanism is selected, required proof remains: unchanged costed held-reader/release/two-call discriminator with no hidden backlog, stops immediately before/after actual immutable installation and STORE CAS, native reader/frontier/CAS safety, all existing checkpoint/Work-bound tests, complete unfiltered workspace tests with the real fixture, and one small native A/N64/H3 pilot with old reader >15 s, actual H2 removal/backlog zero, unchanged independent full oracle and per-batch timings. No broader experiment matrix is implied.

## Provenance and limits

Commands use the existing Rust/Cargo 1.89.0 installation, `--locked --offline --release -j2`, and a new writable target below `/var/tmp/file-native-cleanup-step2-20261026-01a16fd3`. An existing supervisor supplies ordinary timeout/process-group control, native free-space checks and approximately 200 ms resource samples. Those samples are not hard peak or 40 ms qualification. No packages, indexing, new top-level task, origin push, main merge, filesystem cleanup or deletion of originals occurred. The complete test fixture, test executable and target remain local; hashes are bindings, not backup copies.

`ORIGINS.json` binds copied raw records and gzip-decoded originals. `red-instrumented-source-manifest.json` binds the test-instrumented source used for the red command. Its test executable from the first Cargo invocation was overwritten by the subsequent accepted26 rebuild in the shared target; the first run has source/command/compiler-output binding, not a retained executable digest. A separately bound repeat records the diagnostic executable before/after its actual run; that does not retrospectively invent a digest for the first run. All accepted source bytes and prior checkpoint evidence remain unchanged.

GitNexus query and upstream impact requests for the exact new checkout returned repository-not-found. Graph risk/coverage is UNKNOWN. Manual scope is `packing::run -> run_inner -> cleanup`, `physical_lifetime::{load,save,persist_plan,commit_checkpoint,reap_generation}`, `lifetime::remove_checked`, and their native I/O helpers. No product symbol was changed. The structured-format standard was read; it remains proposed and does not authorize a JSON contract migration.

## Independent observation and all-host correctness closeout

The fresh read-only Linux observation found no P1/P2 against this bounded negative research claim. It did not accept Step 2 as a successful repair. [Full observation](independent-review.md) and [owner dispositions](review-dispositions.json) retain the precise scope. S2R-01/P3 is recorded without changing this failed diagnostic: only pass2 has an explicit50ms assertion, so any future green must additionally assert pass1. Both recorded calls already exceed50ms and the backlog assertion fails first.

The119ms subtotal specifically concerns the current **post-release** serial protocol under the frozen profile. Speculatively preparing a retirement checkpoint while a reader remains held is conceivable because recovery rechecks drained native pins before selecting it. That changes when costs occur;119ms is therefore not a lower bound over every future scheduling design. No complete safe two-call plan covering cursor progress, metadata/old-root deletion, crash recovery and unchanged35/50ms admission has been established. The smallest concrete next product decision is whether to accept the frozen-target negative experiment result or separately select/prove such a dependency-reducing design. Nothing here implements or rejects every future variant.

| Host, same accepted26 source at38e2465 | Complete workspace tests | Real boundary fixture | Evidence |
| --- | --- | --- | --- |
| Linux |40passed,0failed/ignored/filtered |80,976,318bytes inventoried locally |[raw command/result](accepted26-unfiltered-workspace.json) |
| Mac |40passed,0failed/ignored/filtered |80,976,318bytes recorded at native origin |[native report](macos/report.md), [result](macos/result.json) |
| Windows |40passed,0failed/ignored/filtered |80,976,318bytes recorded at native origin |[native report](windows/report.md.gz), [receipt](windows/verification-receipt.json.gz) |

`HOST-IMPORTS.json` binds every stored payload to the exact original native bytes and transferred archive. Some originals are gzip encoded solely to preserve whitespace/CRLF; decode to their recorded original hashes. Before/after source26 snapshots, all40 raw outcomes, commands, source/binary bindings and native exit/closure records were checked by the Linux owner. Fixtures and executables remain at their origins; their receipt hashes do not recreate those bytes on Linux. The Mac/Windows evidence was added after the frozen Linux review and was **not** independently reviewed by that observer. These results close all-host unit/workspace correctness on accepted26; no new cleanup pilot or performance qualification is claimed.

The earlier reviewed README bytes are retained at `reviewed/README.md.gz` to reconstruct `FROZEN-SUBJECT.json`; every other frozen member remains unchanged. See `final-result.json` for the owner closeout status. Old research, standalone runtime and unrelated worktrees remain untouched. **Step2 repair is NOT COMPLETE.**
