# File-native engine follow-up — October 17, 2026

Standalone Rust research continuing the [publication experiment](../file-native-publication-experiment-2026-10-17/README.md).
Handbook and research/memory are separate correctness consumers. Both browsing
contracts remain open: A's incrementally refreshed ordinary tree may mix/lag;
B retains a complete readable directory generation for a selected-once reader.
Dolt remains comparator-only. This run allocates no external comparator, owner
migration, new root dependency, daemon or production storage selection.

Read [the comparison](comparison.md), [protocol](protocol.md),
[requirement matrix](verification.md), [allocation](allocation.md) and
[causal review](review.md). Exact source/native binary/environment pins are in
[evidence/pins.json](evidence/pins.json). Raw logs include failures and censored
attempts; development runs and final measurements are kept separate.
Reached [operation](evidence/operation-table.md), [batch](evidence/batch-table.md),
[lookup](evidence/lookup-table.md) and [storage](evidence/storage-table.md) tables
separate complete work from metadata components and retained history costs.
The [growth status table](evidence/growth-status-table.md) preserves censored
operations beside the independently validated selected-history counts.

The measured Windows matrix completed all 16 phases; WSL completed 14 of 16.
Both WSL 100k A initializations were accepted/selected but capped during view
refresh. Separate bounded recovery and retained probes preserve those original
censors. Both browsing contracts remain research options; COW metadata savings
do not remove full graph validation or complete readable-generation costs.

## What changed from the baseline

The isolated implementation reuses the landed publication model and October 12
small lazy disk COW tree. It adds Unix atomic no-replace immutable installation,
instrumented creation phases, optional per-request packet assembly, a COW metadata
descriptor governed solely by application selection, explicit retained-reader/
retirement/reclamation tests, cancellation and busy-wait instrumentation, and
ordinary PowerShell/Bash extraction. Damaged A projections can be detected and
explicitly rebuilt while preserving divergent bytes. Causal review also corrected
duplicate non-idempotent request handling and positive historical-selection proof.

The local disk tree is Handbook experimental source, not imported EchoDB source.
No prolly implementation, full engine fork, DoltLite/SQLite adapter or chunkmap
rerun is included. Metadata point reads and full graph validation are deliberately
different operations; a small metadata root does not make full readable closure
or graph validation free.

## Build and reproduce

This directory is an isolated Cargo workspace. `Cargo.lock` pins the same
dependencies as the baseline, with no root-workspace modification. Recorded
final native Windows build:

```powershell
$env:CARGO_TARGET_DIR='C:/file-native-followup-20261017-build-final-v4'
cargo build --release --locked --manifest-path docs/research/file-native-engine-followup-2026-10-17/Cargo.toml
```

WSL source was transferred through `/mnt/c` and compiled/run under
`/home/spenser/file-native-followup-20261017`, on Ubuntu-24.04 ext4. Build:

```bash
cd /home/spenser/file-native-followup-20261017/source-final-v4
CARGO_TARGET_DIR=/home/spenser/file-native-followup-20261017/build-final-v4 cargo build --release --locked
```

`experiment.py` supports correctness, contention, navigation and growth.
`regressions.py` is the focused final-byte regression set after the earlier full
process-oracle suites. `execute.py` can run all phases serially, pairing both
backends at each growth tier and continuing independent bounded cases after a
recorded failure. Supply a **fresh** `--base`; `init` refuses existing stores.
Existing selected growth stores can be explicitly resumed using positive history;
an interrupted unselected initialization is never reconstructed from survivors.
`continue_selected.py` separately attempts post-cap A view recovery (900 s) and
missing large-tier publications (900 s each) only from positively selected
authority, after the scheduler has exited. It retains the original censored init
and reserves measured missing/divergent view bytes, two full metadata/packet
copies and a 128 MiB margin within the unchanged global disk cap.
`retained_probe.py` then completes missing packet/lookup observations without
changing selection. Their helper/source hashes and sessions are archived.

```powershell
python docs/research/file-native-engine-followup-2026-10-17/execute.py `
  --base C:/file-native-followup-20261017/reproduction-fresh `
  --exe C:/file-native-followup-20261017-build-final-v4/release/file-native-publication-experiment.exe
```

Use a distinct `/home/spenser/.../data/reproduction-fresh` base for Linux, with
its Linux executable. Keep measured platform lanes serial on the shared host.
`--tiers 1000 10000` restricts reproduction to smaller growth tiers; this does
not claim a completed large-tier result. The allocation ledger includes preserved
runs under the declared native roots; free-space and combined allocation checks
can refuse further launches. No dated cutoff from the old harness applies to the
new Driver. The copied baseline harness is used for bounded behavioral helpers;
its legacy CLI/budget is not invoked by the new execution path.

`profile.py` preserves whole-manifest/full-copy profiling with baseline/sharded
original layouts. It was run before metadata integration; its recorded executable
is a development pin, not the final binary. `extend.py` records the initial source
construction step only; committed final source includes later reviewed repairs
and is compiled directly. Do not rerun the one-shot source construction on this
completed directory.
The earlier profile source was not separately snapshotted before later repairs.
Its preserved executable hash and baseline/construction lineage support the
diagnostic scope described in the comparison. Final source and native executable
associations are recorded separately and exactly.

WSL's observed 10k timings justified a pre-launch large-tier budget amendment:
2,400 s initialization and 900 s publication ceilings. Windows keeps its recorded
1,800/600 s ceilings. The [decision and exact prior harness](evidence/harness-before-wsl-large-budget.json)
preserve earlier source association; per-invocation hashes distinguish both
harness versions. Rust source, native executables, corpus and closure/sync work
remain unchanged. Disk and worker controls are unchanged.

`retained_probe.py` runs after `execute.py` has exited. If a growth case stopped
before its packet/lookup measurements, it uses positively selected retained
publications for those separate probes. It never selects an unfinished candidate
or relabels a censored commit as completed. Its invocation/session is recorded in
`retained-probe-observations.json` and the append-only growth events.

## Ordinary extraction

Start at a runtime store's `README.md`, then its one positively selected
publication. `walkthrough.ps1` and `walkthrough.sh` use only ordinary directory/
text/copy operations plus standard JSON/hash tools. They bind the exact retained
path and manifest digest once before copying; no engine executable is used.
`terminal_copy.py` suspends actual enumeration/read/copy across publication.
This is terminal discovery/extraction evidence, not an unfamiliar-human study.
The walkthrough and suspended-publication copy fixtures contain 16 research
documents; they do not measure manual discovery or ordinary whole-store copying
at 100k. Large-tier packets and retained point reads are separate native cases.

Retained source bytes, portable meaningful names, stable IDs, navigation,
attachments and ordered provenance are included in extracted packets. Explicit
fixture-local links are checked; arbitrary Markdown/YAML interpretation is outside
scope. Explicitly retired directories have no ongoing tool-free lifetime promise.

## Evidence handling

`collect.py` freezes JSONL with deterministic gzip and indexes exact raw/compressed
hashes, copies observations, and optionally inventories selected/unselected
candidates. `verify_evidence.py` verifies final source/harness pins and archive
integrity. Rejected preparations and development failures remain in their owned
runtime roots. Only disposable lifecycle fixtures delete eligible data. Preserved
older experiment roots and unrelated untracked repository work remain untouched.

After both measured schedulers exited, a controller-only amendment preserved
exact stdout before timeout or malformed-response refusal. The exact earlier
controller is in the budget archive; [the capture amendment](evidence/capture-amendment.json)
associates old/new source. `capture_checks.py` exercises valid JSON, partial JSON
at timeout, invalid UTF-8 and empty successful output using a small synthetic
Rust transport probe. Its Windows/WSL events are archived separately from real
engine evidence. `reporting_checks.py` tests evidence-session, stdout and table
interpretation refusals; these checks do not rerun or relabel native measurements.

Process kill leaves kernel/device caches alive. Windows directory sync is
unsupported; WSL has a Windows-host/VHDX persistence stack. Separate-host Linux,
macOS, OS reboot/power-cut qualification, stronger flush semantics, production
retention policy and rollback freshness witnesses remain outside this proof.
