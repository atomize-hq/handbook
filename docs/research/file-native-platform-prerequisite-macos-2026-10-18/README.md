# Native macOS file-native engine prerequisite — October 18, 2026

The native prerequisite passed on the unchanged common engine baseline. All 16
scheduled phases, four supplemental cases and four adapted Bash extractions
completed. No native engine invocation was censored. This directory is isolated
research, not a production backend/browsing/format decision or optimization run.
The coordinator consolidates native Linux and Mac results before any Pro consultation.

## Reached native matrix

| Required area | Actual result | Evidence / qualification |
| --- | --- | --- |
| Inventory/build | Native arm64 macOS/APFS; locked release build; 42 exact source/harness/doc blobs verified | [Environment](environment.json), [pins](pins.md); no Rust/core portability patch |
| Small correctness | Full manifest and COW passes: 12 race trials and 48 semantic rows each, A/B and both vocabularies | 2,184 / 2,232 recorded engine process events; [correctness summary](correctness-summary.json) |
| Process interruption | 233 main fault/retry rows per backend: 174 must-not-select, 6 visible-uncertain, 51 must-retain, 2 partial-write retries | Original results checked beneath successors; first/middle/last copy and COW prepared-root hooks in 25 / 37 extended rows per full suite |
| Final affected boundaries | Separate manifest/COW focused passes: 192 / 240 recorded events, including another 25 / 37 extended rows | Separate datasets on this same native binary; not pooled into the full-suite counts |
| Lifecycle/repair | A/B retained-reader/preparation/retirement/sweep checks in both full and focused suites; four restoration fixtures, two actual large-indirection fixtures, one unrelated-page-damage fixture | Original outcomes survive collection; old content has distinct publication identity; lazy point succeeds while broad audit refuses damage |
| Edited derived output | Two packet/provenance edit + missing-navigation probes, interrupted preservation/resume; two corrupt non-retired prior-metadata probes | 20 separate native events; edited bytes preserved, selection unchanged, retained corruption refuses authority repair |
| Ordinary extraction/copy | Four paused navigation copies and four Bash extractions; A mixed, B selected-once coherent; copied closure hashes verified | A: 12 hashed entries plus MANIFEST; B: 19 plus MANIFEST. Exact JSON 1.00/CRLF/Unicode and deep attachment verified; no engine during extraction |
| Bounded contention | 60 requests + four diagnostic rows per backend at widths 1/2/4/8 | Maximum observed request 20.984 s manifest / 22.380 s COW, including retries; at most two attempts observed, 22 pure retries per backend; no fairness/SLA claim |
| Performance smoke | Four 1k project stores; seven retained publications each; 24 commits, four initializations, eight selected-revision packets; 48 fresh-process point samples and four audits | All reached; [all operations/reads/storage](performance.md). 10k/100k and long histories unallocated |
| Evidence export | 5,311 native JSONL events associated with exact binary/source/session/adapter; all have native wait4 resource data; zero native censors | [Verifier result](evidence-verification.json), [raw index](evidence/raw-index.json); four synthetic transport events are separate |

Counts above describe fixture rows and recorded process events, not assertion
counts or independent statistical trials. Two inherited direct duplicate-key
probes additionally retain their decoded responses in the full suites' exported
`special-observations.json`; they are not included in the 5,311 lossless JSONL
events. Synthetic controller checks include one intentional partial-response
timeout. The separate adversarial reporting helper passed source/session/stdout
and completion-table refusal checks. No historical Windows/WSL counts are reused.

## Smoke samples

Each cell below is an individual engine-clock observation. Slash-separated commit
values are 1 / 8 / 64 changed documents in milliseconds. All 36 operation samples,
48 point observations, native process peaks, I/O counters, source associations and
retained inventories are in [performance.md](performance.md), [operations.json](operations.json)
and [points.json](points.json). No p95, device-cold or causal cross-machine ranking
is inferred.

| Backend / contract | Initialization s | Commit without packet ms (1 / 8 / 64) | Commit with packet ms (1 / 8 / 64) |
| --- | ---: | ---: | ---: |
| Manifest A | 29.820 | 195.30 / 408.59 / 1972.85 | 259.25 / 502.28 / 2235.80 |
| COW A | 29.807 | 259.75 / 454.58 / 2147.44 | 382.38 / 578.78 / 2239.40 |
| Manifest B | 25.221 | 6540.70 / 6517.76 / 7556.76 | 6578.70 / 6670.15 / 7989.57 |
| COW B | 27.472 | 6694.21 / 6453.97 / 9861.74 | 6509.38 / 6568.55 / 7637.68 |

For the one-document packet-free A observation, COW writes 81,631 metadata bytes
versus manifest's 469,026, while complete engine times are 259.75 / 195.30 ms.
Current positive point metadata reads are 83,462 / 469,026 bytes, with three-process
engine medians 1.974 / 3.955 ms. B copies 1,000 document files per commit on both
backends and packet flags. These observations preserve the difference between
component savings and complete graph/readable work; both browsing choices remain open.

## Exact subject and environment

Common engine baseline: `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`, fetched from
`origin/codex/file-native-baseline-379af4e` in a separate clone. The existing Mac
checkout was initially clean at `8605d6539355f350c55b921d95fb450bca30ee18`.
A read-only closeout audit found it clean on `feat/handbook-b04-source-review`
at the baseline SHA, with an 11:12 local-time checkout recorded during this run.
This task did not perform that shared-checkout switch or attempt to reverse it.
[The live observation](checkout-drift-observation.json) records both distinct Git
directories and reflogs. All engine work and the report commit use the isolated clone.
The isolated source is `/private/tmp/file-native-prerequisite-macos-20261018-source`;
builds, fixtures and original raw evidence are under
`/private/tmp/file-native-prerequisite-macos-20261018`.

Native build: Rust/Cargo 1.89.0, `--release --locked`, existing toolchain,
aarch64-apple-darwin Mach-O. Python 3.11.9 and system Bash 3.2 are used.
macOS 26.6.1 / Darwin 25.6.0 arm64; the runtime volume is APFS on
`/dev/disk3s5`, mounted at `/System/Volumes/Data`. Direct probes find case-insensitive
and Unicode-normalization-insensitive name lookup. Initial free space is recorded
in [environment.json](environment.json) and [filesystem-probe.json](filesystem-probe.json).
`diskutil` cannot access DiskManagement here; mount information and actual probes
are retained. Optional hardware sysctl queries were refused by this execution
environment; unavailable inventory is not reported as zero.

The native executable SHA-256 is
`37ac4fe7f6e84da07a16d46c9905b16266295df955d5928637f4924b7fdf3dcc`.
[binary-pin.json](binary-pin.json), [source-pins.json](source-pins.json), and
[source-blob-verification.json](source-blob-verification.json) associate it with
42 exact checked-out Git blobs. [prior-provenance-pins.json](prior-provenance-pins.json)
records the earlier local transaction/tree source solely for provenance. No
EchoDB source was imported, no core source patch was needed, and no root workspace
manifest or dependency was changed.

## Native API contract

The baseline's Unix immutable-name protocol uses Rust hard_link, which this
binary implements with `linkat`; it installs a fully written/synced private stage,
checks an existing winner, synchronizes containing directories, then removes and
synchronizes the stage name. Selector replacement uses `rename`; fs2 locking uses
`flock`. Same-content writers, conflicting/corrupt winners, inode-preserving retry,
nested mkdir retry and preparation/collection coordination are tested natively.

Rust 1.89's Apple `File::sync_all` calls `fcntl(F_FULLFSYNC)`, rather than the
other Unix implementation's fsync. The release binary's disassembly shows
fcntl with argument `0x33`; direct file and directory calls both returned success.
See [sync-api-probe.json](sync-api-probe.json),
[native-api-disassembly.txt](native-api-disassembly.txt), and the
[pinned Rust implementation](https://github.com/rust-lang/rust/blob/1.89.0/library/std/src/sys/fs/unix.rs#L1129-L1139).
The separate initial Python os.fsync probes also succeeded. Neither API success
nor process death tests qualify device/OS persistence after reboot or power loss.

The unchanged engine's Unix peak-memory helper reads `/proc/self/status`, which
is absent here; its JSON metric remains null. The adapter records each waited
process's Darwin wait4 `ru_maxrss` in bytes. That peak includes final stdout
serialization and process exit, whereas engine clocks stop before serialization.
These measurements are not pooled with Windows peak working set or Linux KiB RSS.

## Allocation and harness boundary

[allocation.json](allocation.json) is declared before build/fixture generation:
60 minutes total, launch cutoff at 50 minutes, 10 minutes reporting reserve,
6 GiB of new owned source/build/runtime/evidence files, 20 GiB free-space reserve,
at most eight correctness workers, serial measured cases, 120-second small and
300-second growth invocation caps. Outer scheduler caps are also finite. No 100k,
long history, old full growth matrix, external comparator, or new optimization
is allocated. The optional 10k pair remains unallocated.

Only original lifecycle helpers reclaim files, under newly owned disposable
subroots. No unrelated checkout/runtime is reset, cleaned, switched or staged.
All accepted results/fences remain retained; this is not production GC policy.
[resource-checks.jsonl](resource-checks.jsonl) records guards and inventories.
File allocation uses st_blocks*512 with inode deduplication, excludes directory
metadata, and does not measure APFS physical shared extents or device writes.

[native_runner.py](native_runner.py) adapts paths, caps, guards and native resource
accounting while running pinned helpers and assertions. Six updates plus initial
yield seven retained publications per 1k project smoke store. Requested versus
packet-free commits remain distinct, and B always copies complete readable files.
[harness-boundary-review.md](harness-boundary-review.md) records explicit caller/path
review and GitNexus UNKNOWN coverage. Per-session manifests retain original helper,
adapter, allocation, source and executable identities. Raw stdout is lossless
Base64 plus length/hash, including censored/malformed observations.
The required staged GitNexus call could not resolve the isolated clone; it does
not provide zero-impact clearance. Exact source-byte checks and the manually
verified staged namespace are the scope proof for this report-only commit.

## Ordinary extraction

The Bash walkthrough uses `ls`, `cat`, `find` (`-maxdepth`/`-exec`), `head`, `cp -r`
and Python's standard-library JSON/path/hash facilities. Additional ordinary
`grep` searches locate relationship/source IDs. It uses no engine API during
discovery or extraction. The original Bash script is executed to preserve its
native `mapfile` incompatibility before using the exact
[Bash 3.2 adaptation](walkthrough-macos.patch).

Each walkthrough starts at the store README, positively binds one selected
publication's exact closure path and manifest digest, reads navigation/overview,
requirements/notes, related targets and the deep attachment, then copies and
verifies the closure/provenance/output. CRLF, Unicode and literal JSON `1.00`
remain exact source bytes. Scripted checks are not unfamiliar-human usability.
Actual paused copies exercise mixed/lagging A versus selected-once coherent B;
the baseline also tests the B per-file reselect control that can mix.

## Scope and interpretation

A chooses immutable authority before incrementally refreshing its mutable readable
tree. Ordinary reads/copies can mix or lag; published/view-pending is distinct from
noncommit. Render-only recovery preserves divergent source/navigation/packet
bytes and does not replay the mutation. A complete marker is not a continuing
integrity assertion. Corrupt non-retired retained metadata refuses repair.

B selects a retained named directory only after required closure preparation and
verification. A reader binds that positively selected path once. Its files are
independent copies in the baseline. A mixing and B cost leave both product options
open. Exact snapshots/retained packets stay coherent. Stable identity differs
from title/path; Handbook project and research vocabularies are separate consumers.

COW point reads verify their selected descriptor/path; full audits detect damaged
unrelated pages. Commits still load/validate the full graph and do required readable
copy/render work. The page codec and full before/after diff are unchanged. Linear
selected history remains capped at 1,000 publications; this smoke measures only
seven per performance store. Fresh processes do not imply device-cold reads.
Inclusive timers are not disjoint totals; sparse samples do not support p95 or
causal OS rankings across different machines.

The existing backend helper conditionally scans pre-existing 10k/100k stores for
a 5,000-constituent overview point read. Those stores are unallocated here, so
that conditional probe has no observations. The small `indirect_gc.py` fixtures
separately exercise actual metadata objects above 4 KiB, protect both versions,
reclaim the obsolete version after explicit retirement, retain the current one,
and preserve the original accepted result. Windows-only selector handle sharing
is not applicable to the native Mac suite.

The structured-format standard remains proposed; Contracts JSON and established
owner/versioned exact source identities control. No source reserialization,
owner migration, production database, query language, daemon, plugin framework,
automatic merge, or production retention horizon is introduced.

## Reproduction and retained evidence

The commands below are the exact entry points used, with fresh roots required:

```bash
CARGO_TARGET_DIR=/private/tmp/file-native-prerequisite-macos-20261018/build \
  cargo build --release --locked \
  --manifest-path docs/research/file-native-engine-followup-2026-10-17/Cargo.toml
python3 docs/research/file-native-platform-prerequisite-macos-2026-10-18/native_runner.py correctness --backend manifest
python3 docs/research/file-native-platform-prerequisite-macos-2026-10-18/schedule.py
python3 docs/research/file-native-platform-prerequisite-macos-2026-10-18/postprocess.py
python3 docs/research/file-native-platform-prerequisite-macos-2026-10-18/collect_native.py
python3 docs/research/file-native-platform-prerequisite-macos-2026-10-18/collect_native.py --verify
python3 docs/research/file-native-platform-prerequisite-macos-2026-10-18/summarize_native.py
```

Never rerun into these retained roots. For reproduction use another isolated copy
of this result checkout, update its copied allocation with new owned source/runtime
paths and start/deadline epochs *before* generation, and retain its new allocation
hash in each session. Remove only the copied report's prior scheduler/output files,
or invoke named native_runner phases individually with a fresh `--run` namespace;
do not alter the original artifacts. The original runtime, release binary, source
clone and extracted copies remain intact for coordinated collection.

The result namespace's local `.gitattributes` disables text conversion so helper,
JSON, log and archive byte identities survive Git checkout on another platform.
New artifact names are checked under NFC plus casefold and portable Windows name
rules. Intentional native filesystem probe names remain in the runtime and JSON
receipts; gzip exports contain byte streams rather than native pathname members.

Initial offline build refusal, restricted-network DNS failures, permitted fetch/
locked crate download, Bash portability refusal, unavailable inventory operations,
all native phase logs and original failures are retained. Synthetic transport and
reporting-adversary checks are separate controller evidence, not engine correctness
or performance samples. No Pro consultation, production merge or deployment runs.

## Closeout accounting and transport

Before the result commit, owned roots account for 1,586,573,312 allocated file bytes; conservatively adding the fetched locked crate cache gives 1,593,364,480 bytes (1.484 GiB), below 6 GiB. Native volume free space is 34.64 GiB, above the 20 GiB reserve. This snapshot follows all native lanes at 49.6 minutes from the declared allocation clock. Its own small JSON and later report/Git metadata follow the inventory. See [resource-final.json](resource-final.json).

The isolated local/result ref is `codex/file-native-macos-prerequisite-20261018`, based directly on the common pin. Only this new prerequisite directory is committed. The coordinator authorized this dedicated result-ref push for retrieval; no core source, baseline ref, main, PR, merge, deployment or Pro consultation is involved. Raw logs, release executable, source clone and all native fixture/extraction roots remain on this Mac for coordinated collection.
