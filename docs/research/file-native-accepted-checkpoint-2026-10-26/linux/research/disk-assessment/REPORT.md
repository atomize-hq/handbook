The native filesystem lost 97.22 GB (90.54 GiB) of non-root available space between the verified 110.60 GB reading at 2026-10-25 04:35:27 UTC and 2026-10-26T01:35:49.463888+00:00. Current native availability is 13.38 GB (12.46 GiB). No files were removed, moved or truncated; no source, harness, Git state, locks or processes were changed, and no builds, tests or pilot were started. Only new investigation artifacts were written under this separate tmpfs report directory.

All matched Handbook/file-native roots on native ext4 occupy 6.17 GB (5.75 GiB). The October 25 /var/tmp task roots total 0.59 GB (0.55 GiB). The latest blocked build/pilot root is 856,064 bytes; it cannot explain the native decline. The shared Cargo cache occupies 2.01 GB (1.87 GiB) and is not exclusive to Handbook. These figures use allocated blocks with hardlinks counted once within the engine pass.

The strongest observed recent contributors are the following build targets. Linux statx birth times establish 77.60 GB (72.27 GiB) of current allocation in surviving inodes created after 2026-10-25 04:35:27 UTC. All three UAA targets were created after that anchor, including copied artifacts whose preserved modification times were older. Their .git pointers identify Unified Agent API worktrees. File creation and project ownership do not prove which process wrote them. The cohort is about 79.8% of the net free-space decline in magnitude, but cannot be subtracted as a causal decomposition: overwritten/deleted files, other allocations and differing sampling times are not captured.

| Build target | Total allocated | Allocated in inodes born after anchor |
|---|---:|---:|
| `/home/spenser/.codex/task-worktrees/uaa-depth-isolated-binding-20261025/target` | 20.31 GB (18.91 GiB) | 20.31 GB (18.91 GiB) |
| `/home/spenser/.codex/task-worktrees/uaa-depth-packet-intake-20261025/target` | 16.63 GB (15.49 GiB) | 16.63 GB (15.49 GiB) |
| `/home/spenser/.codex/task-worktrees/uaa-depth-evidence-bridge-20261025/target` | 20.25 GB (18.86 GiB) | 20.25 GB (18.86 GiB) |
| `/home/spenser/__Active_code/.worktrees/substrate-runtime-refactor-product-authority-integration/target` | 33.39 GB (31.10 GiB) | 11.82 GB (11.01 GiB) |
| `/home/spenser/__Active_code/atomize-hq/sentinel-p2-compat-20261023/target` | 10.67 GB (9.94 GiB) | 8.59 GB (8.00 GiB) |

The Substrate cohort includes 11.04 GB in inodes born during 2026-10-26 01:00–01:59 UTC. Sampled UAA packet-intake binaries were born around 00:34–00:35 UTC, after the blocked attempt’s 00:27 artifact. Availability nevertheless recovered from 12.01 GB at that blocked observation to 13.38 GB during this investigation. This shows why the current cohort cannot reconstruct the whole net history. Exact whole-host writer attribution remains unresolved.

Historical native availability is recorded below. All times are UTC; subtract four hours for Indianapolis EDT. Artifact mtime is an approximate timestamp where the record has no embedded observation time. No exact 111.03 GB primary record was located; the verified 110.60 GB record independently establishes the reported over-100-GB baseline.

| UTC | Available | Evidence |
|---|---:|---|
| 2026-10-25T04:35:27Z | 110.60 GB (103.00 GiB) | Build/test recorded started_utc and native_free_before; [result.json](/var/tmp/file-native-final062-buildtest-20261025-01a16fd3/result.json) |
| 2026-10-25T17:28:15Z | 63.99 GB (59.60 GiB) | Telemetry rerun recorded started_utc and native_free_before; [result.json](/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/result.json) |
| 2026-10-25T20:14:25Z | 41.95 GB (39.06 GiB) | CLI build native_free_after; timestamp from artifact mtime; [cli-build-binding.json](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/cli-build-binding.json) |
| 2026-10-25T22:19:36Z | 36.51 GB (34.00 GiB) | Bootstrap preflight free_bytes; timestamp from artifact mtime; [bootstrap-preflight.json](/var/tmp/file-native-online-pilot-n64-h3-bootstrap-20261025-01a16fd3/bootstrap-preflight.json) |
| 2026-10-26T00:15:50Z | 30.70 GB (28.59 GiB) | Repair final native_free_bytes; timestamp from artifact mtime; [final-preservation.json](/var/tmp/file-native-checkpoint-repair-20261025-01a16fd3/final-preservation.json) |
| 2026-10-26T00:27:23Z | 12.01 GB (11.19 GiB) | Blocked attempt final free; timestamp from artifact mtime; [result.json](/var/tmp/file-native-online-pilot-checkpoint-resume-20261025-01a16fd3/result.json) |
| 2026-10-26T01:23:44Z | 13.38 GB (12.46 GiB) | Investigation initial statvfs/df observation |
| 2026-10-26T01:35:49.463888+00:00 | 13.38 GB (12.46 GiB) | Investigation later statvfs observation; [filesystem-final-sample.json](/tmp/file-native-disk-assessment-20261025-01a16fd3/filesystem-final-sample.json) |

The largest current storage families below are a footprint inventory, not evidence that they grew during this interval. Rows overlap where explicitly indicated; do not add parent totals and their children.

| Current storage family | Allocated | Interpretation |
|---|---:|---|
| `/var/tmp/sentinel-sw17-runtime-20261018` | 189.51 GB (176.49 GiB) | Large retained Sentinel runtime/evidence tree; no baseline proving recent growth. |
| `/home/spenser/.cache` | 109.07 GB (101.58 GiB) | 109 GB cache family; many large timing/refactor directories have older metadata. No blanket deletion conclusion. |
| `/home/spenser/.codex/orchestrations` | 69.66 GB (64.88 GiB) | Mostly sentinel-sw17-20261019-0689166f8f21; preserve task records and evidence. |
| `/home/spenser/__Active_code/.worktrees` | 67.09 GB (62.48 GiB) | Includes the 35.71 GB Substrate integration worktree and its 33.39 GB target above. |
| `/home/spenser/.codex/task-worktrees` | 57.86 GB (53.88 GiB) | Includes the three UAA targets above; do not count both. |
| `/home/spenser/__Active_code/.build` | 20.07 GB (18.69 GiB) | 20.07 GB, largely Substrate e3d builds with older metadata. |
| `/home/spenser/.codex/worktrees` | 16.23 GB (15.12 GiB) | 16.23 GB, includes 11.18 GB application landing evidence. |

The native df-used total is 919.65 GB (856.49 GiB); the unprivileged allocated-block traversal sees 862.55 GB (803.31 GiB). The 57.10 GB (53.18 GiB) gap is unresolved and includes possible inaccessible data and filesystem overhead. The scan reports permission denials, including /var/lib/docker, /var/lib/containerd, and private service directories. No privileged access was attempted. Ext4 additionally reserves 50.01 GB (46.58 GiB) between all free blocks and non-root available blocks; this is separate from df-used minus visible-du and is not evidence that the reserve changed today. Inodes are plentiful.

The accessible /proc sample found only 4,194,304 bytes of deleted-but-open files on native ext4, far too little to explain the decline. Process coverage has permission and race gaps. Codex has writable state/session descriptors; its cumulative lifetime I/O is not a measurement of today’s allocation growth. No complete process-to-directory attribution is claimed.

/tmp is a separate tmpfs, not the native ext4 filesystem. Its latest usage is 27.53 GB (25.64 GiB) with 6.07 GB (5.65 GiB) available. Most visible usage is UAA test targets and fixtures: /tmp/uaa-d5-defect-lao8h6fe (3.82 GB), /tmp/slice2a-adversarial-1srdt4v_ (2.27 GB), /tmp/slice2a-defect.PrOGFa (2.25 GB), and several roughly 2.2 GB UAA/check9 test roots. Cleaning these would free tmpfs capacity/RAM, not native capacity needed by the pilot.

Conditional native cleanup candidates are listed for a later owner decision. None is authorized for removal by this report. Rust incremental compilation directories are the narrowest candidates because builds can regenerate them. Their respective project/task owner must first confirm they are no longer required or in use. Release and preserve necessary binaries/evidence before broader target cleanup. Source trees, .git directories, evidence, stored engine data, original archives, sole binaries, installed packages and the locked offline Cargo dependency cache should be retained unless separately dispositioned.

| Conditional candidate | Current allocated |
|---|---:|
| `/home/spenser/.codex/task-worktrees/uaa-depth-isolated-binding-20261025/target/debug/incremental` | 4.17 GB (3.88 GiB) |
| `/home/spenser/.codex/task-worktrees/uaa-depth-packet-intake-20261025/target/debug/incremental` | 0.01 GB (0.01 GiB) |
| `/home/spenser/.codex/task-worktrees/uaa-depth-evidence-bridge-20261025/target/debug/incremental` | 4.15 GB (3.87 GiB) |
| `/home/spenser/__Active_code/.worktrees/substrate-runtime-refactor-product-authority-integration/target/debug/incremental` | 14.28 GB (13.30 GiB) |

These incremental directories total 22.61 GB (21.06 GiB). Allocated sizes are not guaranteed reclaimable bytes if external hardlinks or other references remain; remeasure after any later approved action. Other full Rust targets above are broader conditional candidates, with more retention and rebuild consequences. The 189.51 GB Sentinel runtime tree is not automatically disposable.

The pilot floor remains 20 GiB (21.47 GB), currently short by 8.10 GB (7.54 GiB) before new build/store overhead. A later cleanup decision should leave substantial operating headroom; simply crossing the floor does not qualify the machine or authorize a retry. During the approximately 12-minute initial-to-later investigation sample, availability declined only 0.00 GB (0.00 GiB) net; that does not prove absence of concurrent writes or future stability. The pilot remains blocked/unrun with zero new engine calls.

Measurements: one engine-scoped GNU du pass (11.3 s), one native-filesystem pass (214.6 s), one tmpfs pass (2.7 s), followed by targeted build-tree allocation and Linux statx birth-time inspections. du uses allocated blocks, does not follow symlinks, stays on each filesystem, and deduplicates hardlinks within a pass. Independent detail passes are subsets, not totals to add. Live scans are not atomic. Native and tmpfs du returned exit 1 because of permission/race errors; those errors are retained. The engine pass returned 0.

Detailed machine-readable findings are in [result.json](result.json). Raw traversal results, permission errors, target cohort measurements, process observations and copied historical free-space records are retained alongside this report. Source/evidence preservation is based on performing no mutations to those paths in this investigation; no new whole-corpus hash verification is claimed.
