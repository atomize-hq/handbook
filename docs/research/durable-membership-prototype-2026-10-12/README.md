# Durable membership prototype comparison

Non-production experiment authorized on 2026-10-12. See [comparison.md](comparison.md)
for the result, [contract.md](contract.md) for modeled obligations, and
[reuse.md](reuse.md) for pinned source/license/reuse accounting.

This directory is an independent Cargo workspace. It does not import Handbook
production crates or alter their dependencies, contracts, or 32-slot capacity.
Both variants use the same semantic/publication/object layer. `chunk` reuses
immutable-chunkmap 2.1.4; `disk` is the new bounded lazy B+tree.

## Reproduce on Windows

From this directory in PowerShell (new output/work names are required):

```powershell
cargo test --locked
cargo build --release --locked
python scripts/run.py --binary target/release/membership-prototype.exe --output runs/repro-evidence --work runs/repro-data --samples 30 --sizes 1000,10000,100000
python scripts/summarize.py runs/repro-evidence --output runs/repro-summary
```

For large-history agent overlap, use the native 100k stores produced by that run
(the suffix is recorded in its samples), a new output name, and the existing work
ancestor:

```powershell
python scripts/agent_growth.py --binary target/release/membership-prototype.exe --chunk-store runs/repro-data/chunk-growth-100000-3 --disk-store runs/repro-data/disk-growth-100000-57 --work runs/repro-data --output runs/repro-agents --size 100000 --trials 2 --pause-ms 5000
```

Inspect the actual store names before this command; an earlier Windows harness
produced disk suffix 54. This adds distinct requests to the existing experiment
stores. It samples actual overlapping prepared processes at 1/2/4/8 agents and
checks exact selected-root increments and retained bindings.

## Reproduce on Linux

Copy this directory's Cargo files, `src/`, and `scripts/` to a Linux filesystem;
do not use a Windows-mounted directory as equivalent Linux storage evidence.
For this run the existing Ubuntu-24.04 WSL2 distribution used
`/home/spenser/membership-prototype-20261012` on ext4.

```sh
cargo test --locked
cargo build --release --locked
python3 scripts/run.py --binary target/release/membership-prototype --output runs/repro-evidence --work runs/repro-data --samples 30 --sizes 1000,10000,100000
python3 scripts/summarize.py runs/repro-evidence --output runs/repro-summary
```

## Drive a case

```powershell
'{"op":"init","n":1000}' | target/release/membership-prototype.exe runs/demo chunk
'{"op":"run","request":"R1","item":"I1","content":"original","proof":1,"stage":"terminal"}' | target/release/membership-prototype.exe runs/demo chunk
'{"op":"authority","epoch":2,"scope":false}' | target/release/membership-prototype.exe runs/demo chunk
'{"op":"run","request":"R1","item":"I1","content":"original","proof":1,"stage":"terminal"}' | target/release/membership-prototype.exe runs/demo chunk
```

The last command returns the original outcome as replay after revocation.
Changed original content/proof/item refuses. `reserve`, `accept`, and `attempt`
exercise prefixes; attempted without terminal stays uncertain. `put` accepts
only unrelated `u:` keys for structure measurements. `snapshot` exposes the
selected root; `audit` validates its reachable tree and every named object.

`PROTO_FAULT` selects abrupt exit(86) at a publication checkpoint. `PROTO_FAIL`
selects an injected error, including `short_write`. The harness enumerates these
cases and checks that each checkpoint was reached. Staging/orphan objects and old
roots remain; no store is deleted or rebuilt by the harness. Never use a
production store. Native identities intentionally make store copies fail.

Evidence records cache conditions, OS/filesystem/toolchain, binary/source hashes,
commands, raw child responses, checks, and instrumentation overhead. Allocation
counts and physical I/O are unavailable. Process death preserves OS cache; these
tests do not qualify hardware power cuts or native macOS.
See [verification.md](verification.md) for harness-version provenance and the
retained compressed evidence layout. The summarizer reads plain or gzip logs.
