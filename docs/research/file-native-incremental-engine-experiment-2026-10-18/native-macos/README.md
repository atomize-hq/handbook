# Native macOS incremental engine result — October 18, 2026

**The requested affected proof passed on corrected pin `a1108c634cb8362aca50e1a6228766802b96f087`.** Locked workspace tests (two Rust tests), release build, default fixed namespace regression, checks with faults, causal supplement and adversarial suite passed. There were **570 uncensored engine responses**: 466 exit 0, 68 expected refusal exit 2 and 36 injected process exits 86. These are response counts, not independent trials or assertion counts.

The corrected executable SHA-256 is `858e596e75b17c6e409fc40dea5492f1e858d79105547e6542db26de56e6ed97`. All 53 capsule source files match the exact common Git pin; no Rust or pinned harness source was edited on Mac. This report-only commit is parented directly to a1108c6 and limited to `native-macos/`.

## Final affected proof

| Suite | Events | Result |
| --- | ---: | --- |
| Namespace regression, default fixed mode | 5 | Store under ancestor `objects/results/disposable/store`; all 16 selected originals retained after collection; original hashes and audit verified |
| Checks with faults | 414 | Four A/B × project/research configurations; 23 publication exit boundaries |
| Causal supplement | 109 | Reader overlap, reclamation, restoration, divergence, packet/history, sealed roots, orphan cleanup, quota, checkpoint/pin/cancel/abandon/GC and fallback regressions passed |
| Adversarial | 42 | Nine grouped forged/missing/corrupt/indirection cases passed |
| Total | **570** | **Zero native censors** |

The phase oracle retains the 14 must-not-select, one visible/incomplete uncertain and eight completed-boundary must-retain distinctions; original outcomes are checked beneath successors. Expected failures and injected exits remain visible in the lossless raw data. [affected-verification.json](affected-verification.json) and [the corrected evidence lane](evidence/corrected/verification.json) retain summaries and exact command/session/source bindings. Build warnings are retained, including four unused-import warnings; the locked tests and release build exited 0.

The namespace init wrote 340,085 total bytes: 309,226 payload, 15,523 control, 14,862 metadata and 474 result bytes. Initial reads were 2,201,691 bytes: 460,778 payload, 6,873 control, 1,734,040 metadata and zero result. All **534 JSON responses** in the corrected lane conserve read/written role totals; the remaining 36 responses are injected exits without JSON. The payload category follows the experiment's file-role policy and includes derived/view/packet files, not only authored documents.

## Why the pin changed

F3 (`f3e942f6a2e84ff33fd6ea860caed49ad650f86f`) has two ancestor-component classification defects. Its byte instrumentation searches any ancestor `results` and assigns ordinary payload/control work to the result role. Its collector searches any ancestor `objects`, which can delete selected originals in a legitimate store root. The coordinator reproduced the lifetime failure on Linux. This is a real root-relative classification bug, not an unsupported-path policy. Mac did not run the F3 `--expect-bug` destructive reproduction.

The supplied a1108c6 corrects collection classification relative to the store and binds byte classification to a skipped `Metrics.store_root`. No graph/history/projection contract change is claimed. The native default fixed regression under both ancestor components passed. This closes the requested affected proof; it does not retroactively repair F3 measurements.

## Retained earlier observations

| Version/lane | Events | Interpretation |
| --- | ---: | --- |
| F3 primary capsule | 602 | Completed small/fault/supplement/adversarial, one A/B 1k smoke pair, separately linked consumer runs and three Bash copies |
| F3 deep companion | 120 | 16 checkpoint-specific exits, waiting cancellation and holder-exit recovery |
| F3 walk companion | 4 | Two ordinary A/B copies; B binds an old generation while a real successor selects |
| F3 bounded total | **726** | 604 exit 0, 69 refusal exit 2, 53 injected exit 86; zero censors |
| fb73e75 development | 117 | Partial run stopped on the common-pin hold before smoke; excluded from final measurements |

All five evidence lanes retain **1,413 responses** with zero censors. Never combine them as one corrected executable run. F3 companions use separately pinned helpers from f2c45c16886ae3c1c89d1084e43b05133383c087 and the unchanged F3 binary. Five ordinary shell copies succeeded across F3 primary/companions. Holder interruption was `EXP_EXIT=lock-acquired` / exit 86, not SIGKILL, reboot or power loss.

F3 primary byte-role payload/control/result bins are invalid because its runner places stores beneath `results`. Raw values remain unchanged; total bytes, objects metadata bytes, correctness outcomes and times remain available with that qualification. Path-neutral F3 companion roles are valid small observations, not replacements for 1k values. The first 16-document init comparison is preserved in [counter-path-comparison.json](counter-path-comparison.json).

[performance.md](performance.md), [smoke-traces.json](smoke-traces.json) and [smoke-operations.json](smoke-operations.json) are explicitly **F3-only**. Requested A batches 1/8/64 actually changed 1/7/56 IDs because overlapping imports reused bytes. Full-reference load plus semantic validation and complete incremental preparation have different scopes; their ratio is not end-to-end speedup. Publication and explicit A-refresh clocks remain separate. B copied 2,003 files in each complete generation. Inclusive timers overlap. There is no p95, device-cold or causal OS ranking claim. A can lag/mix until explicit refresh; B keeps complete independent readable generations; both experimental contracts remain open.

No corrected 1k smoke, consumer/copy, 10k/100k growth, large-history or broad backend rerun was requested or performed. The earlier [F3 report](f3-bounded-path-report.md) is retained as a superseded snapshot; this README controls final pin and defect interpretation.

## Environment, durability and limits

Native Darwin 25.6.0 / macOS 26.6.1 arm64, APFS, Rust/Cargo 1.89.0, Python 3.11.9 and system Bash 3.2. The volume is case- and Unicode-normalization-insensitive. Actual file/directory fsync, Darwin F_FULLFSYNC and same-filesystem hard-link probes succeeded. [development-environment.json](development-environment.json) preserves these probes. F3 executable disassembly in native-api-disassembly.txt retains fcntl(F_FULLFSYNC), linkat, rename and flock; this is F3 API evidence, not newly sampled corrected disassembly.

These are API/process-interruption observations. Kernel/device caches survive; no power-cut/reboot persistence or physical write-amplification proof follows. Fresh process is not device-cold. The unchanged `/proc` peak-memory helper returns null on Mac; no borrowed prior-task RSS or zero-memory claim is substituted. Source/schema semantics remain experimental; the structured format standard remains proposed, owner/versioned contracts control and no production migration is authorized.

## Bounded execution and preserved paths

allocation-final.json records the prospectively authorized F3 final lane: 105 minutes work plus 15 minutes reporting. The a1108c6 affected-only amendment in [affected-plan.json](affected-plan.json) uses **the same original deadline**, without restarting the clock. Separate corrected source/target/runtime paths preserve every earlier artifact. The owned whole-root guard includes source/worktrees/build/runtime/evidence/report, with <=6 GiB and >=20 GiB free-space bounds; the original common runner additionally uses a 5 GiB run-root cap. Actual argv, caps, exits, launcher hashes and resource samples are retained. Ordinary engine calls have 180-second caps; build/tests use 600 seconds. No new broad benchmark or competing run was launched by this chat.

- Whole owned root: `/private/tmp/file-native-incremental-macos-20261018`
- Corrected worktree: `/private/tmp/file-native-incremental-macos-20261018/corrected-checkout`
- Corrected source: `/private/tmp/file-native-incremental-macos-20261018/source-corrected/docs/research/file-native-incremental-engine-experiment-2026-10-18`
- Corrected build: `/private/tmp/file-native-incremental-macos-20261018/corrected-lane/build`
- Corrected runtimes: `corrected-lane/namespace`, `checks`, `supplement`, `adversarial` beneath the owned root
- Corrected local evidence: `/private/tmp/file-native-incremental-macos-20261018/evidence-corrected`
- Earlier F3 source/build/runtime: `source-final`, `final-lane/build`, `final-lane/results`, `final-lane/deep-cases`, `final-lane/walk-cases` beneath the owned root
- The three tiny unit-test temporary roots are separately enumerated in unit-temporary-roots.json and preserved.

Every predecessor prerequisite root remains preserved. The shared Mac checkout was read only at HEAD 379af4eda9b95ee82be9d3afcfdcdca2d6d132f9; no switch/reset/source write or cleanup was performed there. Resource and precommit scope receipts accompany this report. No PR, force push, production merge/deploy, successor dispatch or Pro consultation was performed.

## Reproduction and transport verification

Use a fresh isolated worktree at a1108c6 and archive the capsule into fresh owned paths. [helpers/launch_affected.py](helpers/launch_affected.py) preserves the actual affected-only commands and original allocation binding; it depends on the retained top-level outer helper/allocation. Its recorded runs use `cargo test --locked --workspace`, `cargo build --release --locked --workspace`, `namespace_cases.py` (default fixed), `checks.py --faults`, `supplement.py` and `adversarial.py`, with the distinct corrected target. Consult evidence/corrected/outer--runs.jsonl for exact argv. Do not rerun into preserved roots.

The transport includes compressed raw JSONL, exact stdout/stderr byte bindings, source/session ZIPs, source TARs, source Git-blob/SHA-256 pins, executable hashes, summaries and wrapper code. Full runtime corpora/build binaries remain on Mac. Local `.gitattributes` disables text conversion. Artifact names are checked for NFC/casefold collisions and portable characters; intentional native filesystem case probes are not unpacked into Git artifacts.

After exact-byte Git materialization, run:

```bash
python3 helpers/verify_export.py .
```

The verifier checks raw gzip hashes/counts, stdout/stderr hashes and lengths, JSON type/parse association, session ZIP bytes against pinned sources, source TAR bytes and session/executable-hash associations. It does not claim a reproducible build or verify an absent physical executable. GitNexus coverage of this isolated report is unknown rather than zero risk; exact staged paths and pinned source bytes supply the manual scope check. Reporting does not alter engine symbols or execution flows.
