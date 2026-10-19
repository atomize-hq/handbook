# Pro packing guidance reconciled with local evidence

October 19, 2026. Workflow: `chatgpt-pro-consult`, approach review, fresh independent conversation. Visible selection: **Pro, 5 of 5**. [Completed consultation](https://chatgpt.com/c/6ad66cc0-2a10-83e9-8d9a-444a226fd9c1). [Exact answer](answer.md), [sent prompt](prompt.md), and [transmission manifest](transmission-manifest.json) are preserved here.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Recommendation accepted for the proposed sequence

Keep the disk COW core, selected membership/outcome authority, short serialized publication, exact results, existing conflict behavior and explicit retention. Pro recommends three separately reviewable tasks: exact-final native parity, navigation incrementality, then packing/compression. Native qualification and navigation development can proceed concurrently in isolated worktrees. The navigation result must receive its own native delta checks before it becomes the pinned packing baseline.

The reason to finish navigation first is project sequencing and scope control. It is not an assertion that history packing is impossible to measure with profile 2 unchanged. A frozen navigation control can isolate physical-record storage, but cannot establish incremental large-collection publication.

No task, native run, dependency installation, source migration, store conversion, or production format/API selection was performed by this consultation. The proposed brief is in [next-experiment-brief.md](next-experiment-brief.md).

## Verified additions and qualifications

### Exact post-GC preservation is a new gate for the next experiment

The original operational archive captures 101,003 publications. The final post-GC clone has 101,006 publications and 101 content retirements. It is therefore not an exact restoration image for the newer baseline. This observation follows directly from the capture manifest and final verification, and does not invalidate the previous original-store preservation test.

Before the next destructive conversion, capture the exact final post-GC state, restore it independently, verify its semantic/operational obligations and reopen it. Preserve the original store and original archive as well. This smaller post-GC capture is a baseline preparation gate; live-history packing remains a separate operation.

### macOS RSS needs platform-correct measurement

Current `src/io.rs:195` has Windows process-memory handling and a generic Unix branch reading `/proc/self/status`. It returns `None` on a normal macOS host, not a measured zero. Use an external platform-correct sampler or a separately pinned harness correction, without relabeling a modified engine as unchanged baseline. The prior native Mac prerequisite already used native process resource observations, so there is reusable measurement precedent.

### Navigation needs two changes, not just an incremental input reader

`projection::delta()` unconditionally calls `collection()` for profile 2. A later equality comparison can avoid changing the stored projection entry, but cannot undo the scan already performed. First skip rendering when the exported identity/title/kind/path/relationship facts did not change. Then bound output rewriting when those facts do change. Streaming a replacement monolithic YAML from old YAML still performs whole-output work.

Pro's proposed grouped YAML layout is a candidate, not an approved permanent schema. Target roughly 256-512 records / 256 KiB per part initially, keep meaningful directory context and stable-ID routing, and handle unusually large adjacency separately. Do not reintroduce compulsory per-document files or generated Markdown. Kinds whose authored files already carry the required facts need no supplemental navigation.

Because navigation can be absent, a future projection profile must have explicit immutable identity rather than relying on an optional `collection.yaml` filename. Preserve old selected roots, sealed candidates, pending plans and packet/projection obligations.

### Dolt supplies particularly relevant relocation and read-barrier patterns

The coordinator independently read the primary descriptions of:

- [Dolt relocation correction #11892](https://github.com/dolthub/dolt/pull/11892): a lookup can miss retained data if relocation happens between checking the destination and source generations. Its generation-order fix is specific to its movement order. Our candidate should instead capture a catalog and protect the corresponding physical lifetimes for the complete read.
- [Dolt read-drain correction #11891](https://github.com/dolthub/dolt/pull/11891): outstanding reads must be included when installing a GC barrier; cancellation must release parked readers. Reuse these invariants and adversarial schedules, not a broad Go session/GC subsystem.
- [Dolt NBS table format](https://github.com/dolthub/dolt/blob/main/go/store/nbs/table.go) and [file-manifest boundary](https://github.com/dolthub/dolt/blob/main/go/store/nbs/file_manifest.go): immutable indexed records and checked manifest publication are directly relevant physical-storage patterns. The coordinator retrieved the file-manifest source, which Pro reported it could not retrieve. That supplies supporting pattern evidence, not a new native correctness qualification.

The [Git pack/index format](https://git-scm.com/docs/pack-format) remains another immediate reference. Adapt published concepts without adopting Git's object/ref semantics or copying GPL implementation under the user's MIT/Apache donor allowance. Dolt stays comparator-only as a database dependency.

### A codec proposal is separate from source-donor licensing

Pro proposes per-record Zstandard for the first integrated compressed cell, with a bounded read-only 64 KiB-block sample comparison before implementation. No delta chains, dictionaries, background codec threads or multiple complete codec systems are needed.

The coordinator verified the upstream [zstd-rs manifest](https://raw.githubusercontent.com/gyscos/zstd-rs/main/Cargo.toml), [BSD-3-Clause license](https://raw.githubusercontent.com/gyscos/zstd-rs/main/LICENSE), and published [crates.io 0.14.3 metadata](https://crates.io/api/v1/crates/zstd/0.14.3). The release is not yanked; checksum and metadata are saved in [codec-metadata-check.json](codec-metadata-check.json). This is a BSD dependency candidate, not MIT/Apache source copying. Transitive versions/licenses, native C build and decoder limits still need qualification before use. No dependency was selected or installed here.

## Minimal abstractions proposed

Use concrete private modules for typed record keys, a checked record resolver, immutable pack/index framing, and a physical catalog/maintenance operation. Avoid a generic backend framework, a general LSM implementation or a plugin interface.

A physical catalog locates bytes; it cannot make an orphan record accepted or revive retired content. Pro proposes a new explicitly versioned `STORE.json` referencing an immutable catalog so old binaries refuse a packing-enabled store. This is a credible experiment hypothesis, not an adopted disk format. Initial activation is quiescent with old binaries closed. Each operation captures the catalog after obtaining its lifetime lease; ordinary commits retain their existing expected-head behavior.

Pro's canonical-location policy is suitable for the experiment: mapped keys use their selected packed location, unmapped keys use loose paths, exact temporary duplicates are allowed, conflicting duplicates refuse, and corruption at the canonical location does not silently fall back to an old duplicate. A missing request, fenced request and integrity failure remain distinct.

Maintenance must have durable ownership/reservations and release allocation only after deletion succeeds. Existing historical owner records remain unchanged. Pack garbage collection, candidate replay, checkpointing and independent verification must enumerate logical records through checked inventory/resolution; directory listings no longer describe every internal record.

## Proposed experiment adjustments beyond Pro's numerical gates

Keep L/P/Z comparisons on identical retained input inventories and frozen navigation. Report both P versus L and Z versus P, including absolute latency, rather than checking compression against P while ignoring a slow P baseline. Freeze ordinary-publication regression criteria before dispatch; proposed criteria are recorded in the brief and are research decision rules, not production SLAs.

Treat the numerical pack, index, cache and time limits as prospectively selected research limits. Larger valid records remain loose and their excluded coverage is reported; this is not a reduction in supported input size. If role counts or peak coexistence do not fit those bounds, stop admission and report it rather than silently increasing caps or removing preserved data.

Maintain one active long-history mutation lane. A final packed restore may require a new destination while the packed source is quiescent; both physical datasets remain counted under the combined ceiling. An inactive/preserved label does not make its bytes disappear from accounting. Admit that restore separately and retain enough closeout reserve.

Authenticated H1k/H10k record subsets are not operational stores. Do not rewind `SELECTED` to historical publications whose historical index pages were legitimately collected. Reuse the valid H101k baseline for the actual operational measurement.

## Donor ranking after reconciliation

- **Inspect now:** Git pack/index concepts; Dolt NBS table/index-validator and manifest-install boundaries; the two Dolt relocation/read-barrier corrections. Limit deep inspection to these boundaries and pinned tests.
- **Later if measured:** parcopy for copy-bound publication/export; mmap-io for demonstrably expensive pack reads; immutable-chunkmap sorted batching algorithms; file-locks for a concrete native locking mismatch. Existing dependencies remain the control.
- **Later consumer/comparator:** GitButler operation/workspace ideas, DoltLite/DumboDB consumer or finalization semantics. Current GitButler's FSL license is not blanket MIT source permission. No current database adoption is recommended.
- **No immediate investigation:** EchoDB's main transaction-shell contribution was already tested; Noms is historical; Starshard is an in-process cache candidate; cow-utils is a string utility; OpenRaft/d-engine consensus is outside the selected local shared-store problem.

The coordinator also checked [LevelDB's immutable-table format](https://github.com/google/leveldb/blob/main/doc/table_format.md) as a bounded design reference. It does not add another implementation track. No repo popularity, README platform list or upstream test result is treated as qualification of our engine.

## Completion and provenance

The prompt was accepted at 19:07:31 UTC. Five scheduled timer/poll cycles established completion at approximately 19:34:03 UTC, with Stop absent and the final Read aloud checkbox present. The copied answer was saved unedited; its exact hash is in [consultation.json](consultation.json). The visible UI reported `Worked for 9m 18s`; this is separate from total wall-clock consultation time.

The result tab is retained. [Visual proof](pro-recommendation.png) shows the recommendation and three-task order. Only the bounded redacted prompt and 202,338-byte evidence artifact were transmitted. Production source, raw host logs, private connection details, full archives and live datasets were excluded.
