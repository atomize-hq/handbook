# Linux corrected bootstrap pilot — FAIL at initial cleanup watchdog

On 2026-10-25, one explicitly authorized corrected A/N64/H3 attempt used **12 engine invocations** against the same source-bound CLI. All 12 returned native exit 0. The initial packing request included `bootstrap:true` and successfully registered physical generation `physical-1-birth-769131-1792966800367777795-0` (ordinal 1). The unchanged initial cleanup then returned `cleaning` / `deferred:cleanup-entry-budget` for all eight allowed batches. The existing SDK raised `RuntimeError('cleanup-batch-watchdog')` before the H2 publication. The worker returned exit 0 after recording that semantic FAIL; this is not a passing pilot.

No further engine invocation, batch-limit change, repair or pilot replay followed this new failure. The requested online proof remains incomplete. H2/H3, the distinct held old reader/maintainer/publisher, capture-before-H3-before-activation ordering, reader hold beyond 15 seconds, old/current result comparison, the two final drainage calls, post-release deletion and complete independent H3 oracle were **not reached**. Checker processes: 0. Initialization supplied 64 documents and accepted the initial publication; no independent final N64/H3 claim is made. Linux remains PENDING; the full matrix is unreleased.

## Exact authorized delta and preflight

The new task-owned recipe differs from the prior Linux task-owned recipe only by `h.pack(root)` → `h.pack(root, bootstrap=True)` for its first physical build. An AST comparison removing this single added keyword establishes the rest of the recipe is identical, including all assertions, cleanup bounds and final exact-two-pass successor. The worker, checker and oracle mapping bridge are byte-identical to their prior retained versions. Paths change through their existing dynamic ROOT binding; no harness redesign occurred.

Mac's retained thread `01a16fd3-4723-7f62-828a-265fa095e28a`, command `exec-b6a3f548-fe61-4d55-ad92-92b79562d46d`, applies exactly that same initial-bootstrap substitution. Its relevant source/command observation is retained in [mac-bootstrap-adaptation-evidence.json](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/mac-bootstrap-adaptation-evidence.json). Final source `src/rolling.rs:76–77` defines first discovery by absent prior state and requires JSON `bootstrap:true`; the unchanged SDK `Harness.pack` forwards supplied fields. Preflight captured the exact forwarded request without executing an engine, and actual call 4 matches it. See [preflight](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/bootstrap-preflight.json), [request](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/expected-initial-packing-request.json), and [single-delta patch](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/initial-bootstrap-only.patch).

The new disposable store is `/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/disposable/store` on `/dev/nvme0n1p2`, ext4. The original delivered recipe remains untouched inside the copied SDK; the prior task-owned pre-bootstrap recipe is also retained in this root. The previous failed call-4 store, logs, reports and evidence remain at `/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3`. All **657 files in that attempt's retained evidence manifest** were checked unchanged after this run. The two earlier zero-engine startup failures remain in that prior root.

## Initial cleanup results and performance observations

Every cleanup returned the authenticated engine response `maintenance_deferral: deferred:cleanup-entry-budget`, `completed:false`, `cooperative_limit_ms:50`, `legacy_whole_index_indivisible:true`. All eight exceeded the 50 ms cooperative target, with reported batch times **53.140526–66.001971 ms**. Those misses remain failures of that target; they are not discarded or converted into hard-bound performance qualification. The functional pilot also failed its existing eight-batch prerequisite. These observations alone do not establish why cleanup did not finish within eight batches.

| Engine call | Reported candidates | Batch ms | Status |
|---|---:|---:|---|
| engine-05-packing | 96 | 56.462540 | cleaning |
| engine-06-packing | 82 | 66.001971 | cleaning |
| engine-07-packing | 76 | 55.586397 | cleaning |
| engine-08-packing | 76 | 53.140526 | cleaning |
| engine-09-packing | 76 | 55.718268 | cleaning |
| engine-10-packing | 76 | 55.789659 | cleaning |
| engine-11-packing | 76 | 56.060344 | cleaning |
| engine-12-packing | 76 | 54.201253 | cleaning |

Exact responses, full metrics and raw transport records are retained in [cleanup-target-misses.json](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/cleanup-target-misses.json), [pilot-result.json](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/pilot-result.json) and [logs](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/logs). No extra engine audit or alternate checker was used to reinterpret the outcome.

## Source, SDK and executable binding

Source identity is baseline `0628322e4b48e0efad19cdc32238a622d4aaa7a8` **plus only the previously authorized telemetry fixture n1-to-n10 patch**. Corrected `src/io.rs` SHA-256 is `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`; corrected 25-file manifest SHA-256 is `1076b20ec662bc01a90119f15504789599fc3057f9014e9d5016c0d663fb646b`. All 25 source files matched before and after, and original SOURCE_FREEZE SHA-256 `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3` remains unchanged. This is not represented as unchanged baseline062.

The actual CLI SHA-256 is `b46977f9440b5dea662c0ce547a7ca9b9b581358bf7755abf0a54b6c4df5873b`. It was copied byte-identically from the prior native source-bound build; no rebuild or test rerun was needed. [Origin build binding](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/cli-origin-build-binding.json) retains its exact locked/offline release j2 build/source correspondence. All 26 copied SDK members and task adapter bindings match after execution; original independent oracle semantics and sources were not edited. The earlier corrected 31/31 tests remain prior evidence, not a substitute for this failed pilot. See [source before](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/corrected-source-before.json), [source after](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/corrected-source-after.json), [adapter hashes](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/adapter-source-bindings.json), [verification](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/final-verification.json).

## Native supervision, closure and resources

Every call retained real PID/start identity, a positive RSS observation before stdin delivery, complete stdin, both output EOFs, original native wait status, joined output readers and empty owned process group. All twelve identities were confirmed gone. No concurrent callback actor was reached; callback closure is therefore vacuous for those three roles. Host monitor joined without resource error; existing lock files had no conflicting live holders. The outer supervisor retained two empty owned-tree observations and needed no stop signal.

Finite outer timeout remained 480 seconds; per-call limits, 3 GiB aggregate RSS and Linux 20 GiB free floor remained unchanged. The workload stage lasted 5.937008 seconds. About-200ms sampling recorded tree-plus-supervisor peak **50,167,808 bytes**, with no observed cap breach. This is sampled functional containment, not a hard all-instants peak or 40ms qualification. The old-reader 256 MiB role was not exercised; the separate checker 2 GiB role was not exercised.

Practical native free room was 36,507,254,784 bytes before execution and 36,503,228,416 at final verification, above the 21,474,836,480-byte floor. New output allocated 6,942,720 bytes / 267 files at stage close. The retained store has 169 files / 1,650,669 logical bytes. This small copied-CLI/SDK/store footprint does not restore a cohort or consume protected originals; the human combined 44 GiB / 5 million-file ceiling and inside reserve remain controlling. No new global resource census or experiment-wide qualification is claimed.

All raw inputs/results, target-miss events, native identities/EOFs, samples and closure records are retained under this root. [Supervisor record](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/pilot-bootstrap-supervision.json), [worker trace](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/pilot-exception.txt), [retained store inventory](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/retained-store-inventory.json), [pending matrix ledger](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/matrix-ledger-pending.json).

## Delivery and stop state

At completion, discovery found no supported `send_message_to_thread` tool in this session. The substantive handoff is saved in [COORDINATOR-HANDOFF.json](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/COORDINATOR-HANDOFF.json); no delivery is claimed. No blind messaging retry, Git publication, Sentinel cleanup, engine change, assertion relaxation, new owner or further campaign occurred.
