# Transaction evidence and reproduction

This is the follow-up subject, separate from the predecessor verification and
review. All retained versions use new output/work directories; failed or
superseded evidence is not overwritten. No production store is used.

## Frozen source and native binaries

Both final builds use Rust 1.89.0 and the unchanged isolated Cargo manifest/lock.
The current source hashes are recorded in each final `environment.json` and the
EchoDB accounting snapshot. Final Windows binary SHA-256 is
`b4e3538f7c9192386b8de5f1f87a7c8922f679be67ade6b5a94c5e0fc5530716`;
final Linux binary SHA-256 is
`bfedfc910322a98329f68f1478b4fe0fb12f5056b3b05a6ef196cb04eb6146b2`.
Native Cargo test logs are retained in `windows-tx-units-20261012d` and
`linux-tx-units-20261012c`: 18 tests pass on each operating system.

| Evidence directories under `evidence/` | Windows / Linux assertions | Interpretation |
| --- | ---: | --- |
| `windows-tx-selected-baseline-20261012a`, `linux-tx-selected-baseline-20261012a` | 17 / 17 | Reproduces selected domain overflow and wrong-shard audit findings on predecessor binaries. |
| `windows-tx-final-20261012a` | 2,442 passes, 4 failures | First candidate; Windows child input used cp1252 for high-byte domain cases. Two assertion failures and two suite-abort notes are retained. |
| `windows-tx-final-20261012b`, `linux-tx-final-20261012a` | 2,476 / 2,464 | Explicit UTF-8 harness; earlier transaction implementation, superseded. |
| `windows-tx-final-20261012c`, `linux-tx-final-20261012b` | 2,476 / 2,464 | Typed commit-failure candidate; superseded by replay dispatch repair. |
| `windows-tx-replay-baseline-20261012a`, `linux-tx-replay-baseline-20261012a` | 24 / 24 | Reproduces the review's flagged semantic replay bypass on the preceding candidate. Ordinary replay provides a positive checkpoint control. |
| `windows-tx-final-20261012d`, `linux-tx-final-20261012c` | 2,476 / 2,464 | Final nine sections: selected repairs, API, maximal domain, retained snapshots, semantics, transaction faults, concurrency, publication faults, validation. |
| `windows-tx-races-final-20261012a`, `linux-tx-races-final-20261012a` | 1,278 / 1,278 | Final flagged replay repair, actual prepared fresh/resumed authority revocation at widths 1/2/4/8, lost reply followed by exact replay after revocation. |
| `windows-tx-repair-final-20261012c`, `linux-tx-repair-final-20261012c` | 116 / 116 | Final inherited publication repair regressions; earlier `a` and `b` versions remain. |
| `windows-tx-growth-20261012a`, `linux-tx-growth-20261012a` | 5,924 / 5,912 | Final complete main growth/correctness harness; 30 observations at 1k/10k/100k. |
| `windows-tx-measure-20261012a`, `linux-tx-measure-20261012a` | 6,870 / 6,870 | Final paired 1/64 writes, cancel/drop and lookup; 96 groups per OS; three generic overlap cohorts at each width. |
| `windows-tx-agents-20261012a`, `linux-tx-agents-20261012a` | 642 / 642 | Final prepared semantic AGENT cohorts at 100k, three trials at widths 1/2/4/8. |

All final rows above have zero failures and skips. Counts are assertions, not
independent statistical trials. Linux's nine-section count differs because the
native inherited fault/validation paths differ. Replay-bypass baseline environments explicitly
link the source manifests whose binary hashes match the old binary; their current
checkout hashes must not be used as old-binary provenance.

The superseded replay candidate's complete Cargo/Rust source is retained at
[replay-candidate-source](evidence/replay-candidate-source/provenance.json).
It was reconstructed after repair by reversing only the dispatch condition,
read-only replay guard and associated test block. All six byte hashes exactly
match both original candidate manifests; reconstruction is explicit provenance,
and those files are not the final implementation. The selected-finding predecessor
source remains available at `c7f54e63`.

The regenerated `tx-final-growth-summary-20261012a` has 716 groups from the two
final main runs only. Its `nonpassing_checks` contains 48 descriptive observations,
with no failed assertions. Native Linux measurement output IDs include
`-evidence-`, omitted from copied repository directory names; the records and
original argument paths are unchanged. The paired/cohort status flag
`full_harness_complete:false` means those are focused harnesses; their own
`complete:true` and zero failures/skips establish completion within that scope.

`transaction_tests.py` inherits the existing suites and uses explicit UTF-8 on all
child streams. Its SHA-256 is
`74111fb0c6ba1f6f171cb32abe4bac1e5928ce94894141df6ac624780b20428a`.
`transaction_measure.py` is
`58e1c5acb93ce4a3c5d7a80e022c6200e746dd7813bbd289ff70a7470f2b0938`;
`transaction_races.py` is
`76d3020cb1f2205ee83a1f14830edbbaa92a97e72b4da8c2b5d0a5feea876810`.
Earlier scripts and predecessor raw evidence are unchanged.

## Reproduce final correctness and measurements

From this independent workspace on Windows, with new directory names:

```powershell
cargo test --locked
cargo build --release --locked
python scripts/transaction_tests.py --binary target/release/membership-prototype.exe --output runs/repro-tx-tests --work runs/repro-tx-test-data
python scripts/transaction_races.py --binary target/release/membership-prototype.exe --output runs/repro-tx-races --work runs/repro-tx-race-data
python scripts/review_regression.py --binary target/release/membership-prototype.exe --output runs/repro-tx-repair --work runs/repro-tx-repair-data
python scripts/run.py --binary target/release/membership-prototype.exe --output runs/repro-tx-growth --work runs/repro-tx-growth-data --samples 30 --sizes 1000,10000,100000
python scripts/transaction_measure.py --binary target/release/membership-prototype.exe --output runs/repro-tx-measure --work runs/repro-tx-measure-data --samples 30 --sizes 1000,10000,100000 --cohort-trials 3 --processes 1,2,4,8 --pause-ms 5000
python scripts/summarize.py runs/repro-tx-growth --output runs/repro-tx-growth-summary
```

Then use the 100k stores created by that main run for the separate semantic
cohorts. The recorded full default run creates the following store names; check
the retained store manifest if changing variants, sizes or harness options:

```powershell
python scripts/agent_growth.py --binary target/release/membership-prototype.exe --chunk-store runs/repro-tx-growth-data/chunk-growth-100000-3 --disk-store runs/repro-tx-growth-data/disk-growth-100000-57 --work runs/repro-tx-growth-data --output runs/repro-tx-agents --size 100000 --trials 3 --processes 1,2,4,8 --pause-ms 5000
```

On Linux use `python3`, omit `.exe`, and build/run on a native Linux filesystem.
This experiment uses the existing Ubuntu-24.04 WSL2 `/home/spenser` ext4 mount.
Do not substitute `/mnt/c` as Linux filesystem qualification. Exact run arguments,
native inventory, command responses, source/binary hashes and cache/fault limits
are in the native manifests. Native macOS requires the qualification plan in
[platforms.md](platforms.md); it has not been executed here.

The measurements use 30 observations per case and size, fresh processes with
uncontrolled OS cache, and separate identical-history native stores for sequential
and coalesced updates. Their order alternates within each A/B pair. Three cohorts
per width/variant sample current RSS while all processes are alive in a common
prepared window. Preparation markers follow actual snapshot reads and logical
buffer construction; the 5-second pause only makes the window observable. The
pause is excluded from preparation/lock metrics and included in cohort wall time.
No result treats that wall time as throughput, sums child peak RSS, or interprets
shared-page RSS as unique memory. Application byte/helper counters do not measure
physical I/O, allocation or OS syscall counts.

Raw `samples.jsonl`, `checks.jsonl` and large derived `summary.json` are retained as deterministic, lossless gzip
using the unchanged `pack_evidence.py`; decompression is checked byte-for-byte.
Small manifests, statuses, Markdown/CSV summaries and simultaneous-memory records remain
directly readable. The original plain raw files, binaries and native stores remain
local/ignored. Packing does not select or discard observations.

The post-run [assembly verification](evidence/tx-final-verification-20261012a/verification.json)
checks packed/raw byte equality, retained check counts against every run status,
all final source manifests, unit-log hashes, and the two retained native release
binaries. It keeps final and superseded subjects separate. It is an evidence
integrity check, not a reproducible-build or hardware persistence qualification.

`cargo fmt --check`, Python AST parsing and staged path/whitespace checks pass.
The two verbatim native unit logs retain Cargo's terminal blank line; they are
excluded only from Git's blank-at-EOF whitespace check, with byte hashes verified.
No predecessor raw evidence, existing five harness scripts or current Cargo
manifest/lock was changed. GitNexus staged/default-branch results and their
partial-index limitation are recorded in [transaction-review.md](transaction-review.md).
