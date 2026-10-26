**Yes—specific generated compiler files are candidates for removal, but the whole root should stay.** A bounded cleanup could reclaim **8.66 GB allocated**, restoring approximately **21.41 GiB free**. Nothing was deleted, archived, moved, or resumed.

[Saved assessment](/tmp/file-native-host-inventory-linux-20261024-01a16fd3/sentinel-retention-assessment/REPORT.md) · [Exact proposed file list](/tmp/file-native-host-inventory-linux-20261024-01a16fd3/sentinel-retention-assessment/cache-candidate-files.json)

# Sentinel SW17 retention/removal assessment

Assessment only, on `spenser-linux`, 2026-10-24 EDT (recorded resource summary `2026-10-25T02:23:24Z`). No deletion, move, truncation, archiving, source modification, process termination, build or test occurred. Original Handbook inventory and blocked-build reports remain unchanged.

## Recommendation

Do **not** remove `/var/tmp/sentinel-sw17-runtime-20261018` or entire checkpoint/review/input directories. It is a retained, unfinished Sentinel implementation/evidence area, not an expendable temporary corpus.

A narrowly defined **conditional cleanup of 7,906 generated compiler-cache/library files** in four completed review areas could reclaim **8,664,956,928 allocated bytes (8.07 GiB)**. Preserve all source, inputs, fixtures, emitted receipts, command logs and actual executable variants. The exact candidates and preservation prerequisites are below. This is a recommendation for later explicit authorization, not permission already given to delete them.

There is an existing instruction to keep this evidence root intact. Authorizing these exact generated files would deliberately narrow that retention instruction and retire their complete-tree verification coverage. Keep original manifests unchanged; record the authorized retirement separately. If complete byte-for-byte reconstruction of every old sealed artifact tree must remain possible, retain these files too; no independently verified archive covering all of them was established.

## Identity, provenance and live-use observations

The root occupies **189,509,623,808 allocated bytes (176.49 GiB)** on ext4 `/dev/nvme0n1p2`. `du -x -B1 --max-depth=1` measured allocated blocks, not apparent file lengths. The reported ~176 GiB is supported. Other `/var/tmp` entries were not assessed because this root contains enough conditional candidates for the immediate space goal.

Applicable instructions were read at both `/var/tmp/sentinel-sw17-runtime-20261018/AGENTS.md` and `/home/spenser/__Active_code/atomize-hq/sentinel/AGENTS.md`, plus the paused handoff and later planning record. No symbol edit or indexing was needed for this retention assessment.

Key retention authorities:

- `/home/spenser/.codex/handoffs/sentinel-sw17-20261019/2026-10-19-112553-sw17-paused-meta-pickup.md:11`: **“Full retained evidence root (keep intact): /var/tmp/sentinel-sw17-runtime-20261018.”** The same handoff preserves every failed receipt, frozen fixture/oracle and historical proof, and requires actual source/config/binary variants to remain represented.
- `/var/tmp/sentinel-sw17-runtime-20261018/active-checkpoint.json`: explicit user pause; analyzer K/P unfinished; group027 sealed, incomplete and unapplied; final candidate not frozen and publication gate closed. Nine foundational stages were independently closed, but whole SW17 was not complete.
- `/home/spenser/__Active_code/atomize-hq/sentinel/.scratch/codex-stable-window-delivery/version-window-first/plan.md`, dated October 22: the stopped implementation remains stopped; automatic continuation is **deferred, not canceled**. This does not release the old evidence for disposal.
- Current product checkout: `/home/spenser/__Active_code/atomize-hq/sentinel`, branch `feat/sw17-verified-content`, HEAD `d899a6b6204a69c74a832efa3fc9036a75f631c5`, 68 porcelain entries. The old pause records identify HEAD `4e399f6c3f2ce047ee4b1c24120e6add655ed57e`; that is historical, not current Git truth. No Git mutation/fetch occurred.

A read-only check of visible `/proc` process cwd/exe/open-FD symlink targets found **no process using the evidence root or product checkout**, no permission-denied paths, and no rows in the visible `/proc/locks`. No locks were acquired or broken. Cargo and incremental lock-file existence is not proof of active ownership. These observations apply only to the visible process namespace at inspection time; they are not a reservation against later activity. Recheck immediately before any authorized cleanup. Historical pause records also say writers stopped, but those old statements are not substituted for today's observation.

## Immediate artifact families

Largest immediate areas, allocated bytes from the root-wide metadata scan:

| Absolute path | Allocated bytes | Retention significance |
| --- | ---: | --- |
| `/var/tmp/sentinel-sw17-runtime-20261018/analyzer-checkpoint` | 33,557,356,544 | Unfinished K/P work, source variants, failed/passing runs, current binaries, unapplied group027; retain entirely. |
| `/var/tmp/sentinel-sw17-runtime-20261018/compactor-package-checkpoint` | 17,146,802,176 | Source slices, registrations, failures, delivered source and large build tree; retain in this proposal. |
| `/var/tmp/sentinel-sw17-runtime-20261018/family-repair-closure` | 16,295,260,160 | 10,546,528,256 bytes frozen input; 5,748,719,616 bytes review output including build trees. Only selected generated files proposed. |
| `/var/tmp/sentinel-sw17-runtime-20261018/provider-read-correction-closure` | 11,925,835,776 | 6,833,004,544 bytes frozen input; 5,092,818,944 bytes review output. Only selected generated files proposed. |
| `/var/tmp/sentinel-sw17-runtime-20261018/normalization-checkpoint` | 9,466,847,232 | Source/test/build checkpoint; retain. |
| `/var/tmp/sentinel-sw17-runtime-20261018/admitted-body-checkpoint` | 9,289,625,600 | Historical body source variants and build evidence; retain. |
| `/var/tmp/sentinel-sw17-runtime-20261018/provider-read-correction` | 8,562,470,912 | Before/final/red/private source, binary variants and accounting receipts; retain. |
| `/var/tmp/sentinel-sw17-runtime-20261018/producer-build` | 7,629,316,096 | Producer build/fixture provenance not independently qualified for removal here. |
| `/var/tmp/sentinel-sw17-runtime-20261018/structural-review-1` | 7,333,572,608 | Accepted focused structural review; only selected generated files proposed. |
| `/var/tmp/sentinel-sw17-runtime-20261018/provider-repair-closure` | 6,479,364,096 | Earlier mandatory read-accounting repair finding and evidence; retain. |
| `/var/tmp/sentinel-sw17-runtime-20261018/compactor-package-repair-1` | 6,360,485,888 | Source/build/repair evidence; retain. |
| `/var/tmp/sentinel-sw17-runtime-20261018/admitted-body-review-1` | 5,964,128,256 | Accepted bounded body dependency with historical qualifications; only selected generated files proposed. |
| `/var/tmp/sentinel-sw17-runtime-20261018/provider-review-1` | 5,943,394,304 | Original failed findings, overwritten/recovered variants and external-write observations; retain. |
| `/var/tmp/sentinel-sw17-runtime-20261018/structural-checkpoint` | 5,206,429,696 | Source/failure/build provenance; retain. |
| `/var/tmp/sentinel-sw17-runtime-20261018/family-review-1` | 4,814,811,136 | Original family findings and probes; retain in this proposal. |
| `/var/tmp/sentinel-sw17-runtime-20261018/normalization-review-1` | 4,576,874,496 | Earlier normalization source/binaries/harness incidents; retain. |

Smaller families include decoder/schema/packaging checkpoints and repairs, old readers, literal freezes, source seams, native platform preparation, oracle revisions, current/old ledgers and preservation records. The complete immediate allocated-size listing is `root-du.stdout` in this assessment directory. Size is not a disposal classification.

## Exact conditional removal candidates

For each **base directory below**, select only:

1. Regular files recursively inside its `incremental/` subdirectory.
2. Immediate regular files under its `deps/` subdirectory whose suffix is exactly `.rlib` or `.rmeta`.

The exact existing file paths are frozen in `cache-candidate-files.json`; a future deletion must use that file list after revalidation, not broad globs. Do not delete the base, the whole `deps/`, executable files, `.so` libraries, `build/`, `.fingerprint/`, `tmp/`, or other artifacts. Directory blocks are not included in the reclaim estimate.

| Exact base directory | Incremental allocated file bytes | `.rlib`/`.rmeta` allocated file bytes | Expected reclaimable file blocks |
| --- | ---: | ---: | ---: |
| `/var/tmp/sentinel-sw17-runtime-20261018/family-repair-closure/review/target-stock/debug` | 1,330,790,400 | 405,065,728 | 1,735,856,128 |
| `/var/tmp/sentinel-sw17-runtime-20261018/family-repair-closure/review/target-probes/debug` | 444,768,256 | 405,065,728 | 849,833,984 |
| `/var/tmp/sentinel-sw17-runtime-20261018/family-repair-closure/review/target-musl/x86_64-unknown-linux-musl/debug` | 43,319,296 | 53,514,240 | 96,833,536 |
| `/var/tmp/sentinel-sw17-runtime-20261018/provider-read-correction-closure/review/target-fresh-provider-accounting/debug` | 1,349,087,232 | 463,765,504 | 1,812,852,736 |
| `/var/tmp/sentinel-sw17-runtime-20261018/provider-read-correction-closure/review/target-fresh-native-boundary/debug` | 217,665,536 | 267,091,968 | 484,757,504 |
| `/var/tmp/sentinel-sw17-runtime-20261018/structural-review-1/review/target/debug` | 1,065,803,776 | 402,284,544 | 1,468,088,320 |
| `/var/tmp/sentinel-sw17-runtime-20261018/admitted-body-review-1/review/target/debug` | 1,658,564,608 | 558,170,112 | 2,216,734,720 |
| **Total** | **6,109,999,104** | **2,554,957,824** | **8,664,956,928** |

All **7,906 files are regular files with distinct `(device,inode)` identities and link count 1**. Allocated credit is `st_blocks × 512`; no hardlink, apparent-size or sparse-file saving was assumed. All 7,906 have historical SHA-256 entries in the original review artifact manifests, and historical sizes match current stat sizes. Their contents were not freshly rehashed; byte equality is not claimed from size alone. The allocation total excludes directory blocks and remains an estimate of future free-space gain; concurrent writes, new open files and filesystem accounting can change the outcome.

Rationale: these are generated Rust incremental state and compilation libraries. Exact source/configuration and command history remain, as do actual executable variants. Rebuilding them is possible in principle; exact bit-for-bit reproducibility was not tested. The selected review stages have bounded acceptance records: family-repair closure accepted; provider read-accounting closure accepted; structural review accepted; body dependency accepted with historical qualifications. This is not a reason to remove their original failing controls, probes or evidence.

## Preservation prerequisites before any deletion

For **each of the four review areas**, preserve its original sealed manifest/delivery record and parent adjudication outside the cleanup selection, together with an immutable cleanup addendum listing exact removed paths, historical SHA-256, current size/allocation/inode/link count and reason. Keep those old manifests byte-for-byte unchanged. Do not relabel the original complete-tree seal as still fully verifiable after removing its compiler files.

Specific records:

| Review area | Original preservation records that must remain |
| --- | --- |
| Family repair closure | `/var/tmp/sentinel-sw17-runtime-20261018/family-repair-closure/adjudication.json`; `/var/tmp/sentinel-sw17-runtime-20261018/family-repair-closure/review/delivery-receipt.json` SHA `190368588fe7958e6b2698380f8186fae8ad861a4c99e154b28f49ee41ee1892`; `failed-attempts.json`, source/probe variants, command logs, retained failed-build/probe binaries, input snapshots and all test executables. |
| Provider read correction closure | `/var/tmp/sentinel-sw17-runtime-20261018/provider-read-correction-closure/adjudication.json`; `/var/tmp/sentinel-sw17-runtime-20261018/provider-read-correction-closure/review/delivery-receipt.json` SHA `76a3343454c028dafc3ab8157ef818cf1dfe9711ee987e54ee71d535c2651911`; pristine/independent/private/tested binary bindings, actual ELF files in `debug/deps`, source snapshots, configs, receipts, failed-attempt records and all commands/logs. Its `review/binaries/` is empty: do not assume executable backups exist there. |
| Structural review | `/var/tmp/sentinel-sw17-runtime-20261018/structural-review-1/adjudication.json`; `/var/tmp/sentinel-sw17-runtime-20261018/structural-review-1/review/artifact-manifest.json` SHA `5ecbbe34231b51608a2d8d59893a6cf273765eabac7628aef8c205bb9d87ef4a`; delivery receipt, toolchain, capture provenance, intended/actual registrations, execution receipts, logs, source and executable files. |
| Admitted-body review | `/var/tmp/sentinel-sw17-runtime-20261018/admitted-body-review-1/adjudication.json`; `/var/tmp/sentinel-sw17-runtime-20261018/admitted-body-review-1/review/artifact-manifest.json` SHA `9f1a1ad76bb38d7f863927d845459d0499c76ce3499d140ab61058ae815a85f5`; delivery receipt, source variants, binaries, original failed runner/inspection evidence, primary audits and test logs. |

The assessment already retains a roughly 6.1 MB derivative candidate manifest with historical hashes and current allocation metadata. This is not a byte backup. Before cleanup, the small original records above should be secured with the cleanup addendum and candidate manifest, and source/executable preservation paths checked. No new archive was created by this assessment. Full log/fixture/binary trees are preserved in place by excluding them from the deletion list, not by claiming a new backup exists.

## What must stay, and archive/duplicate coverage

Keep the entire unfinished `/var/tmp/sentinel-sw17-runtime-20261018/analyzer-checkpoint`, all original inputs/frozen fixtures/oracles, checkpoint histories, current ledgers, original failed source/configuration/outputs, all actual binary variants and private originals. Do not execute or inspect interrupted private controls. The original handoff records historical missing intermediate binary and overwritten-receipt qualifications; neither can be repaired by deleting the remaining originals.

One existing source archive has historical parent verification of 951 members:

`/var/tmp/sentinel-sw17-runtime-20261018/analyzer-checkpoint/run-001/variants/p-owner-repair-002/source.tar.gz`, 232,316,560 bytes, recorded SHA `78266f142713c5275ca9bd8fe7214bce95468c84af35c3fd3b479f08c3d01419`.

The corresponding source manifest SHA is `cff97f3c0f4ab9b3cfd1a83495a821e4026764e8b006af93fad7fa7bd5ed4a88`. The retained three-executable manifest SHA is `2d039644c1a0f2cf06f02f23c6460a0d8c284a8d666156d9319691f338a69094`. These are historical validation statements, not fresh full archive/binary rehashes. That archive covers a source variant, not the entire runtime root, all failed-run evidence, saved states, private originals or selected compiler caches.

Several frozen `input/evidence` trees contain copied earlier evidence/build families. Duplication by directory name/size is not verified independent coverage. No whole-input deletion is recommended, and no complete independent archive or duplicate for the 176.49 GiB root was established.

## Space outcome and bounded next decision

At the recorded summary, ext4 available space was **14,327,140,352 bytes**. Adding only the measured selected file blocks predicts **22,992,097,280 bytes (21.41 GiB)** available, or **1,517,260,800 bytes (1.41 GiB)** above the 20 GiB floor. A practical **1 GiB new-work allowance** leaves about 443.5 MB above the floor, assuming other activity does not consume it. Prior Handbook target/fixture records are roughly 130 MB + 81 MB before additional release-test/log overhead, so this is sufficient for that small build/test attempt with continuous space checks; it is not room for an unrestricted matrix or new corpus.

Recommended decision: authorize only the exact generated-file list, explicitly accept retirement of their complete-tree artifact coverage, preserve the small records first, then recheck live use and actual free space before and after deletion. If actual space or workload differs, stop and reassess rather than widening cleanup. **Do not resume Handbook build/tests as part of this assessment or cleanup authorization unless separately instructed.**

If the keep-intact requirement must remain absolute, do not remove any assessed file; a separate location or externally authorized disk reclamation is needed. No existing original has been altered to force the build to proceed.

## Report artifacts and coverage limits

Assessment directory: `/tmp/file-native-host-inventory-linux-20261024-01a16fd3/sentinel-retention-assessment`.

- `root-du.stdout` / `root-du.stderr`: complete immediate allocated-size scan.
- `cache-candidate-files.json`: exact 7,906-file proposal with inode/link/allocation metadata and original manifest/hash references.
- `cache-candidate-sizes.json`: seven directory subtotals.
- `assessment-summary.json`: scope, authority pins, allocation totals and free-space forecast.
- `live-use-observation.json`: current visible process/FD/lock observation.
- `authority-references.json`: retention and pause sources.

This was bounded metadata/provenance assessment, not a full source/evidence integrity audit or machine census. No candidate-content hashing, archive extraction, full duplicate verification or private-session search occurred. Other `/var/tmp` roots and remote platforms were not assessed. Future deletion requires current path/identity/liveness revalidation and explicit authority; no cleanup was performed.
