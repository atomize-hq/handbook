# Native Linux and macOS prerequisite: combined findings

Date: October 18, 2026. Both native prerequisites completed, and both result bundles are available and verified in this workspace. This is a standalone research baseline; no production architecture, migration, public API, or durability policy is selected. No Pro consultation or optimization experiment was launched.

Common engine source: `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`. The coordinator published that existing revision as `origin/codex/file-native-baseline-379af4e`. Both native runs kept the Rust engine unchanged. Platform-specific resource, telemetry, and extraction adaptations are in separate prerequisite harnesses.

## Result

The landed implementation builds and passes its bounded publication, recovery, graph-conflict, retention, reclamation, and readable-extraction checks on native Linux/ext4 and macOS/APFS. This closes the first-native-run gap before the proposed validation/publication/history optimizations. It does not establish power-loss durability, large native growth performance, long-history behavior, or production readiness.

| Area | Native Linux | Native Mac |
| --- | --- | --- |
| Environment | Manjaro Linux 6.16.8-1, x86_64 GNU, ext4 | macOS 26.6.1 / Darwin 25.6.0, arm64, APFS |
| Locked build and common Rust source | Passed; source files bound to pinned Git blobs | Passed; source files bound to pinned Git blobs |
| Full manifest and COW correctness | Both passed | Both passed |
| A/B and project/research cases | Represented on both backends | Represented on both backends |
| Focused boundary regressions | Both backends passed separately | Both backends passed separately |
| Retirement, reclamation, restoration, selected outcome recovery | Passed | Passed |
| Edited packet/provenance, missing navigation, interrupted preservation | Two supplemental backend cases passed | Supplemental cases passed |
| Ordinary extraction/copy | Four walkthroughs passed after wrapper repair | Four walkthroughs passed with Bash 3.2 adaptation |
| Contention and queued cancellation | Both backends passed | Both backends passed |
| Performance smoke | Four 1k stores; seven retained publications each | Four 1k stores; seven retained publications each |
| New Rust portability patch | None | None |

Each full backend suite on each host executed 12 race observations, 48 semantic-conflict rows, and 233 main fault/retry rows, plus 25 manifest / 37 COW extended fault rows. Focused regression repetitions remain separate datasets. Observation/event counts are not totals of independent tests.

The smoke allocation uses initial state plus six 1/8/64-document changes with packet=false/true. Each platform also completed eight selected-revision packet assemblies and 48 current/initial retained positive/negative fresh-process point reads. Histories are deliberately short. Thousands of process events across fixtures do not establish a long history in one store.

## Platform findings

Linux ext4 distinguishes case variants and composed/decomposed Unicode names. The tested Mac APFS volume is case-insensitive and normalization-insensitive; exclusive alias creation refused while preserving the existing file. The engine's portable stable-ID filenames retained distinct similarly titled documents on both.

Both native Unix builds exercised same-filesystem no-replace hard-link installation of a completed private stage, existing-winner verification, rename selection, and directory synchronization. The Mac run additionally identified Rust 1.89's Apple File::sync_all implementation using F_FULLFSYNC, associated the native binary with that call, and ran direct probes. Source/API association and successful process tests are distinct from a complete syscall trace or persistence proof after power removal.

macOS Bash 3.2 lacks mapfile. A scoped walkthrough adaptation preserves the selected exact path, manifest binding, copy, and hash checks. The engine's Linux /proc peak-memory probe is unavailable on Mac; separately captured Darwin memory observations use byte units and must not be treated as the same measurement field or as zero memory usage.

Neither platform changes A/B semantics: A's ordinary tree may mix/lag and can be explicitly rebuilt while preserving external edits; B's selected-once generation and retained packets must remain coherent. A per-file current selector is not a snapshot. Both options remain open. Dolt remains comparator-only and was not integrated.

## Controller and artifact corrections

The Linux run preserved two original navigation-wrapper failures caused by a missing copy-destination parent. The engine navigation/paused-copy steps had passed. Four unchanged Bash walkthroughs subsequently passed against fresh extraction destinations. Original phase records remain 12/14 passes; later extraction success does not relabel the original failures. The packet-edit supplement is another separately recorded successful check.

Returning native evidence to Windows exposed deliberate Case/case probe filenames that NTFS could not materialize distinctly. The compressed source evidence remained intact. Verification checks original archive members, and encoded local probe names plus a reversible map retain both original names/bytes. These are probe artifact names, not the engine's document paths.

The Mac closeout observed a concurrent change in the shared Mac project checkout. Its experiment ran in a separate Git clone, kept its source pinned, left the shared checkout untouched, and recorded that observation. The Windows tracked checkout also remained at the baseline with no staged or modified tracked files during coordination.

## Evidence

- [Linux report, native timings, hashes, and reproduction](file-native-platform-prerequisite-linux-2026-10-18/report.md).
- [Linux base return verification](file-native-platform-prerequisite-linux-2026-10-18/verification-return.json): 5,291 real-engine events, zero censors, 5,535 exported files and four copied packets verified.
- [Linux packet-edit supplement verification](file-native-platform-prerequisite-linux-2026-10-18/verification-supplement.json): two cases, 12 additional native events, zero censors. Combined native events: 5,303.
- [Mac report, native timings, hashes, and reproduction](file-native-platform-prerequisite-macos-2026-10-18/README.md).
- [Mac native evidence verification](file-native-platform-prerequisite-macos-2026-10-18/evidence-verification.json): 5,311 real-engine events with zero censors; four synthetic transport events remain separately counted. All 16 scheduled phases, four supplemental cases, and four adapted Bash extractions completed.
- Mac result commit: `1ad8841b82f3c378838c96621fed8f2061e0c26b`, directly parented by the common baseline, published at `origin/codex/file-native-macos-prerequisite-20261018`. Its 230 files are confined to the Mac prerequisite namespace. The coordinator fetched the ref and materialized the exact Git blobs locally without changing HEAD or the index.

The coordinator independently read all 50 Linux source-archive members, checked their byte sizes and SHA-256 values, computed their Git blob IDs, compared those IDs with the baseline Git objects, and verified both returned archive hashes against their receipts. All checks passed without modifying the source or runtime evidence.

For Mac retrieval, the coordinator independently checked the result commit's parent and namespace, rejected nonportable/colliding paths and link members, and verified all 230 materialized Git blobs. A second check matched all 42 source pins to baseline Git bytes, verified compressed and uncompressed hashes/counts for 19 raw archives, and checked the binary identity, zero-censor status, lossless stdout hash/length, and type-preserving decoded response for all 5,311 native events. The four synthetic events stayed separate. All checks passed. This verifies transport and recorded associations; it is not a second independent native execution or code audit.

Linux used approximately 1.383 GiB of scoped native file allocation, with approximately 285 GiB free at closeout. Mac used 1.484 GiB including the fetched locked-crate cache footprint, with 34.64 GiB free. Both stayed below a 6 GiB allocation and above a 20 GiB reserve. Counts exclude filesystem metadata and do not measure physical device writes. Host hardware, API work, caches, and sampling differ: no cross-host OS speed ranking is inferred.

GitNexus coverage of new prerequisite harnesses is unknown/stale. No Rust protocol change was made. Controller path/caller checks and native execution are bounded evidence, not a clean graph-impact verdict or independent comprehensive code audit.

## Next decision

After the human reviews this prerequisite result, include these native findings, the common source pin, harness limitations, and the [next-experiment proposal](file-native-engine-next-experiment-2026-10-18.md) in the Pro approach review. No Pro request or optimization experiment was launched by this prerequisite.

Long histories beyond the current 1,000-publication traversal limit, incremental invariant validation, delta-native updates, compact encoding, incremental readable publication, strict candidate quotas, and a small library boundary remain the proposed next work. Power/reboot qualification, hostile namespace/root identity hardening, general source/schema validation, and production integration remain explicit later concerns.
