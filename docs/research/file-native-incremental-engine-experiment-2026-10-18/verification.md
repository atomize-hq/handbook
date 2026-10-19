# Verification and reproduction

Core pin: `5a65a2155f9eb297d2cbb1b7c900fa749629605c`. Earlier measurements keep
their own source/executable pins; see [the report](report.md) and
[independent review](review.md). Final growth/history/WSL evidence is pending.

## Executed final correctness

The main suites below use `7ac70f9`. The strict audit correction uses `7e00d22`;
the later focused accounting correction uses `5a65a21`. These are versioned
observations, not a claim that every suite was repeated on the latest pin.

| Lane/suite | Engine responses | Result |
| --- | ---: | --- |
| Windows checks/faults | 414 | Four A/B × consumer configurations; 23 named publication exits |
| Windows causal supplement | 109 | Retention, actual reclamation, readers, packet, divergence, quotas, fallback |
| Windows adversarial | 42 | Forged candidates, damaged graph/history/result and indirect values |
| Windows deep cases | 120 | 16 checkpoint exits; waiting cancellation and holder exit/recovery |
| Windows PowerShell copies | 4 | A packet copy; B selected-once old generation under successor |
| Windows replay lifetime | 40 | A/B N512; eligible GC/reopen, payload intermediates, forged sets, abandoned deletion |
| Windows namespace | 5 | Valid root containing ancestor `objects/results`; 16 originals retained |
| Windows differential | 8 | 2,000 seeded batches at N128/1024; independent complete acceptance/state/index checks |
| Windows total | 742 | 618 exit0, 71 expected exit2, 53 injected exit86; zero censors |
| Native Linux targeted final | 610 | Locked tests/build plus replay, namespace, checks/faults, supplement, adversarial; zero censors |
| Native Mac targeted final | 610 | Same affected suites; zero censors; original deadline unchanged |
| Windows/Linux/Mac strict audit correction | 42 each | Each has 18 damaged-evidence refusals without repair and 18 restored audits; zero censors |
| Windows/Linux/Mac paired-intent accounting correction | 181 each | 32 paired A/B, 109 causal supplement, 40 replay/GC; locked tests; zero censors |

Each response is an observation, not an independent trial or assertion count.
Both workspace unit tests passed. Original boundary exits remain exits even when
recovery later succeeds. Preselection must not publish, visible/incomplete state
is qualified uncertain, and completed-boundary outcomes must remain exact beneath
successors. The separate source-retirement regressions check actual deletion,
not merely a quota decrement.

Earlier F3 native capsules retain two independently linked consumer executions,
1k A/B smoke and Bash copies. Corrected WSL runs will execute the common capsule
and companions; do not infer a final 1k native rerun from the affected-only suites.

## Fresh-path commands

Use this report's Git revision for the finalized reproduction/proof helpers;
its Rust core is unchanged from `5a65a21`. Historical measurements retain their
own captured harnesses and source pins, so this is an affected correctness rerun,
not a byte-identical replay of every old controller. Run from this standalone
directory, with a **new** build target and **new** runtime roots. Never reuse
preserved paths. In particular, the finalized history script's disposable paths
differ from the already loaded long-running harness preserved in its source ZIP.

```bash
cargo test --locked --workspace --target-dir /new/owned/build
cargo build --release --locked --workspace --target-dir /new/owned/build
python3 checks.py --exe /new/owned/build/release/file-native-incremental --base /new/owned/checks --faults
python3 replay_lifetime.py --exe /new/owned/build/release/file-native-incremental --base /new/owned/replay
python3 audit_evidence.py --exe /new/owned/build/release/file-native-incremental --base /new/owned/audit --strict
python3 run_differential.py --exe /new/owned/build/release/file-native-incremental --base /new/owned/differential
```

Use `.exe` on Windows and an explicit Python executable. `supplement.py`,
`adversarial.py`, `deep_cases.py` and `namespace_cases.py` use the same `--exe` /
`--base` shape. `walk_cases.py` additionally takes `--source` and
`--shell powershell` or `--shell bash`; the former chooses installed `pwsh` first.

The finite large-workload controllers are `matrix.py`, `finish_matrix.py` and
`history_run.py`. `serial_windows_v2.py` retains actual Windows continuation argv,
caps, local paths and ordering. `run_lane.py` retains the native/WSL outer guard.
These are reproduction records, not permission to reset an expired deadline or
start a new experiment automatically. Preserve original censors.

The ordinary walkthroughs read/copy/hash useful files without invoking an engine:

```bash
bash walkthrough.sh /retained/store /fresh/copied-output
```

```powershell
pwsh -NoProfile -File ./walkthrough.ps1 -Root C:/retained/store -Destination C:/fresh/copied-output -Python C:/path/to/python.exe
```

For A, create/select an explicit packet first; a mutable readable view can mix or
lag. For B, binding reads one exact selected generation. The fixture deliberately
pauses B copying, selects a successor, then verifies that the original copy stays
bound to its exact old generation. Retention/pin policy still controls retirement.

## Independent and transport verification

`verify_store.py` is a separate Python C1 decoder/full walker. `verify_genuine.py`
authenticates selected-chain membership and unequal old/final source digests.
`codec_geometry.py` compares JSON/C1 logical document maps and physical geometry.
`pack_evidence.py` verifies exact stdout bytes, hashes and JSON types before
compressing raw events; gzip round trips verify lossless bytes. Session ZIPs retain
working source at harness creation and bind executable SHA-256; separate
build/source receipts associate compiled inputs. An older executable can run
beside a newer working source snapshot. Full mutable stores/builds stay in owned
runtime roots.

`pack_history.py` exports selected evidence, final history-index pages and live
graph/projection/source roots in bounded chunks. The per-document renderer does
not independently specify the static root navigation prose or packet recipe;
those have selected digest checks and separate packet assertions. Verify and restore a proof capsule
to a **fresh** path, then run the independent history verifier:

```bash
python3 pack_history.py --verify /evidence/history-proof --restore /fresh/proof-restored
```

This proof does not contain every mutable view/generation/quota/intermediate file
required to restart the engine. It establishes its explicitly recorded evidence
scope. A cross-host H82 sample has already restored and independently verified 417
files; final durable-history proof is a separate artifact.

Mac's portable verifier runs from this experiment directory:

```bash
python3 native-macos/helpers/verify_export.py native-macos
```

It verifies all source TARs/session ZIPs and response/binary-hash associations,
including the final 610-response lane. No reproducible build or absent physical
executable verification is implied. Rehashing retained native executables is
separate from checking the recorded association.
