# Exact-final native macOS Q qualification

October 19, 2026. Standalone experiment; native Apple M2 Max / macOS 26.6.1 / APFS, Rust/Cargo 1.89.0, aarch64-apple-darwin. Source f0904a63cb2500b9bea3a4034d19cbe38bfac729; unchanged Rust/Cargo core 07a8574e581e66ba58d7caac87238aa5f93918be. All 15 pinned source/build-input hashes match. Executable SHA-256 `c6030f5bd02af83744d98f451e109cbf5ade67b6708f27fa4a9049a8175710bb`. Full source, harness and legacy binary bindings are in evidence/binding.json.

The complete strengthened capsule passed 24 cases / 282 process events. This includes recreated divergence after namespace interruption, genuinely interrupted A transition backup, fences, pins, eligible candidates, packets, restored successors, transfer/checkpoint/retire/repeated GC, namespace refusals and retained selected-once B generations. Explicit supplements passed external empty-directory preservation with divergent bytes, strict missing/corrupt evidence and damaged transport refusal, independent verification of all four A/B N16/N128 restores, restored fence/intent refusal, native immutable-install winner/corrupt-winner refusal, replacement/directory sync, killed-holder lock release and waiting cancellation. Pinned legacy 5a65a215 fixtures with sealed packet candidates replayed unchanged before new profile-2 targets on both A and B.

Ordinary Bash find/grep/awk/cp/cmp/shasum discovery and exact extraction passed on A/B. The dedicated shell supplement verifies stable IDs, locations, typed directed ordered duplicate local relationships and captured external relationships, with the independent complete oracle. No engine calls occur during the shell discovery/extraction phase.

| Stage | Result | Seconds |
| --- | --- | ---: |
| bash-extraction | success | 0.166 |
| capsule | success | 68.265 |
| cargo-build | success | 14.649 |
| cargo-test-native | success | 0.412 |
| cargo-test | failure | 10.489 |
| delta | success | 1.408 |
| extra | success | 16.23 |
| legacy-build | success | 8.913 |
| legacy-create | success | 8.321 |
| legacy-replay | success | 2.917 |
| lock-and-checkpoint | success | 28.001 |
| native-fileops-build | success | 0.252 |
| native-fileops | success | 0.418 |
| restored-fences | success | 0.411 |
| shell-semantics | success | 2.338 |

The initial cargo-test wrapper returned failure because `/usr/bin/time -l` could not read sandboxed `kern.clockrate`; its retained stdout shows both tests passed. A fresh native monitored rerun passed. This is an external harness correction, not an engine change. A harness-preparation NameError occurred after creating scripts and copying the final executable, before writing the binary receipt; the missing import/receipt was corrected before execution. All original command logs are retained.

Native external memory sampling uses ps descendant RSS at 50 ms intervals and wait4 rusage per wrapped engine, in macOS byte units. The baseline peak_memory implementation remains exact and returns null here. Sampled aggregate peak was 529416192 bytes; engine wait4 maximum was 4882432 bytes over 467 wrapped invocations. Sampled aggregate peaks can miss short-lived maxima; these are not physical-memory or device-write measurements. The first test wrapper has no valid native RSS observation.

The original Q deadline is 2026-10-19 22:39:11 UTC (18:39:11 EDT), from the original 20:39:11 UTC dispatch. Admission/approval waiting consumed most of it. Resource GO did not reset it. Capsule ran once in 68.265 seconds, below its 1800-second ceiling; stage caps shrink against the same deadline with 90 seconds reserved for closeout. The global ceiling was explicitly raised to 40 GiB by the human before GO; this lane's admitted total new cap is 1 GiB / 50000 files including source and fetched Git data. No original store or archive was collected or changed.

GitNexus indexes the shared baseline, not this isolated experimental delta. Its detect_changes reported no indexed changes; scope coverage for added harness symbols is UNKNOWN. Manual tracing confines additions to this result directory; Rust/Cargo baseline hashes remain exact. Parent owns independent integration review.

No physical power-loss, reboot, full cross-platform performance or production qualification is claimed. No H100k fixture, migration, navigation implementation or packing change was made. Small A/B artifacts and binaries remain in /private/tmp/file-native-q-macos-20261019; only bounded harness/reports/raw receipts are committed.

## Closeout timing and preserved harness failures

All functional stages completed before the original 22:39:11 UTC deadline. The artifact-copy request was submitted before the deadline but returned after platform approval at epoch 1792452805.767122 (2026-10-19 23:33:25 UTC). Artifact closeout therefore exceeded the original two-hour budget. The coordinator explicitly authorized artifact-only closeout after expiry; no further functional tests ran. Current closeout preparation: 2026-10-19T23:34:08.730090+00:00. The deadline was not reset.

The first artifact whitespace check refused a raw cargo-test stdout log because it ends with a blank line. Raw stdout/stderr bytes are preserved exactly; new source/report whitespace checks are separate. The first preparation command also raised NameError for a missing json import after script generation; this was corrected before its binary-receipt write. These are harness/closeout failures, not engine functional failures.
