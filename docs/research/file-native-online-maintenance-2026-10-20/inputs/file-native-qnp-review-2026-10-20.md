# Review of native qualification, navigation incrementality, and packing

Reviewed 2026-10-20. Recommendation: retain the navigation work and native qualification evidence; continue packing as a private engine experiment. The packing hypothesis has a substantial long-history storage win, but its online maintenance and fresh-process lookup behavior need further work before production adoption.

## What landed and what was reviewed

Integration branch `codex/file-native-qnp-20261019` is clean and pushed at `a0b92fe5dc64269508601dcddf7e2e2ca477e30d`. The final 18-file Rust/Cargo core is `d70f3864f2888d46d9112213b4a03f8d82745a8e`. This remains an isolated research branch, not a merge into the product. The shared project checkout is still at `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`.

I read the meta closeout, relevant worker closeouts, native reports, navigation and packing reports, the later Mac/B follow-up, changed core source, and recovery/publication evidence. I verified all 18 current source hashes against the pinned core bindings and verified the retained Windows executable hash. GitNexus has no enabled MCP tools for this review, and the experimental worktree is unregistered; no clean indexed-impact claim is made. No engine source was edited, no commits were made, and no new sessions or worker messages were dispatched.

Independent local review checks:

- Two codec/typed-key Rust tests passed using the authenticated retained test executable. I did not rebuild or rerun the large cache fixture.
- Twenty-eight engine calls on two new 10-document A/B stores passed: navigation rename, accepted-outcome resolution, raw packing, compressed repacking, cleanup, reopen, subsequent loose-tail publication, and audit. Independent Python packed-record, history, graph, navigation, and ordinary-source checks passed. Original payload hashes remained identical.
- The later Mac return was checked through all 31 data member hashes, its self-excluded manifest membership, exact archive SHA, and complete gzip EOF: 32 regular members total.
- The later B delta was checked through all 172 member sizes/hashes, exact archive SHA, and complete gzip EOF.
- A separate controlled deletion-barrier test reproduced reader refusal during packing cleanup and success after release. The historical full-cohort cleanup timing was read directly from its preserved response.

The fresh review used about 1 MiB of new disposable files under a 4 MiB forecast, within the prior closeout's 5,600,492 unassigned bytes. Initial review-harness mistakes are recorded separately: an invalid eight-document fixture was refused without writes; a same-representation cleanup had no deletion to reach the first chosen barrier. Neither required an engine change.

[Fresh review receipt](C:/Users/spmcc/.codex/visualizations/2026/10/18/01a14f52-1f53-7990-9b84-ebb0af672ff5/qnp-review-20261020/receipt.json) and [availability/archive receipt](C:/Users/spmcc/.codex/visualizations/2026/10/18/01a14f52-1f53-7990-9b84-ebb0af672ff5/qnp-review-20261020/availability-and-evidence.json).

## Native qualification

These are actual native Windows, Linux/ext4, and macOS/APFS runs. The final core passed five workspace tests on each platform. Windows packing qualification covered 54 cases / 275 engine events. Linux and macOS each covered the 52 applicable cases / 267 events, with Windows-specific sharing cases appropriately excluded. Strict-input and ordinary PowerShell/Bash extraction checks also passed.

The final core's complete recovery capsule passed 24 cases / 282 engine events on all three platforms. The original Mac run was time-censored; a separately authorized follow-up completed its full capsule and parent raw-evidence review. It does not retroactively change the original run. Likewise, all four N8000-B comparison cells were completed in the later follow-up. Claims that those two specific items are still incomplete are historical.

This qualifies the tested process-crash/recovery paths. Physical power interruption, reboot/device behavior, and a production power-loss durability contract remain unqualified. Windows directory synchronization is still reported as unsupported. The later Mac aggregate RSS peak was not persisted and remains null, although the live ceiling and native per-group evidence were retained. Earlier resource-monitoring and cutoff limitations remain in their original records.

## Navigation

The former collection-wide rebuild is avoided for ordinary payload/label changes. Metadata and relationship changes update bounded stable-ID groups, with local split/merge and separate large-adjacency parts. New profile-4 navigation uses supplementary YAML and no generated Markdown navigation guides or compulsory per-document index. Existing source bytes, including authored Markdown, are preserved. Supplementation can be omitted for a narrow strict `self-describing-json` kind whose identity, title, kind, and ordered relationships exactly match the graph. General self-describing YAML omission was not implemented.

At 10,000 documents in A, the measured payload preparation fell from 2,847.41 ms to 39.04 ms; title preparation fell from 2,800.65 ms to 220.77 ms. The title case consulted 322 headers and generated two parts / about 50 KB, rather than enumerating 10,000 headers and emitting about 1.94 MB. Actual changed navigation output was one path. These are individual observations with uncontrolled caches, not latency guarantees.

There are meaningful limits. The same title case still visited 963 edge-index pages / 10,272 leaf pairs despite returning no outgoing edges. High-degree and batched changes remain expensive. At N1000, the grouped title case was slightly slower than the ungrouped control. N10k-B initialization hit its 120-second bound and its edit matrix was not run. Completing the separate N8000-B packing comparisons does not fill that navigation measurement gap. Hash-prefix group filenames are an implementation-oriented supplement; meaningful authored source paths remain the main browsing surface.

## Packing/compression

Packing preserves accepted record bytes, request identities, semantic selection, and retirement decisions. It changes physical representation through a checked immutable catalog, bounded indexes, raw records or independent Zstandard frames, and a recoverable maintenance journal. COW pages, current source files, readable projections, generations, and active work remain loose. It is not compression of the entire database.

For the A store at 101,007 accepted publications, the exact cohort contained 504,833 records / 579,015,428 raw bytes. Both representations used 16 packs, 16 indexes, and one catalog:

| Measure | Raw packing P0 | Zstandard PZ |
|---|---:|---:|
| Pack and index bytes | 641,986,037 | 283,460,393 |
| Files in the selected representation | 33 | 33 |

PZ saved 55.85% versus P0. Cleanup reduced eligible files from 504,833 to 33, a 99.9935% reduction. Complete independent checks covered all 101,007 accepted publications, the 100,000-row test ledger, retired-content refusals, current graph, and source/navigation content. After cleanup, the engine excluding that test ledger occupied 5,811 files / 345,898,733 logical bytes. The reported 1,221,001,465 deleted bytes include old P0 containers as well as loose duplicates; they are not a pure loose-baseline saving. Whole-store loose allocated space before activation was not measured.

Packed backup, fresh restore, complete validation, and a subsequent accepted publication passed. Byte restore was 3.469 seconds; full restored verification was 89.140 seconds. These are separate costs, not an end-to-end 3.5-second recovery claim.

Warm lookup gates passed, with packed medians near their loose equivalents. Fresh-process packed lookup medians were about 18–45 ms versus 1.0–1.4 ms for qualified loose storage. A new process loads and authenticates the entire relevant index on its first access; it does not load every index. The current CLI usage makes that startup cost material.

The small-history operational comparison used the same complete 28-record cohort at N8000, with seven subsequent publication cases and matching original outcomes. A reduced the cohort to 11 files, saving 60.71%; B required 19 files, saving 32.14%. Both fail the frozen 95% file-reduction gate. B's count includes eight required loose publication-control duplicates. PZ saved about 77.18% of pack/index bytes in both modes. Ordinary publication medians passed their unchanged relative gates: A PZ was about 386 ms; B PZ about 37.55 seconds. B still constructs a complete readable generation; compression does not remove that cost. These measurements do not justify a claim that compression made publication faster.

An earlier cleanup defect removed publication controls needed by B's ordinary selected-once browsing. The final core fixes this by verifying and retaining every mapped B publication duplicate, including historical and retired selectors. This passed focused native shell regressions, but leaves one loose publication control per accepted publication. Its history-related file growth must remain charged.

## Additional production limits confirmed by this review

**Maintenance availability:** [packing.rs:307](C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18/src/packing.rs:307) takes the exclusive physical reader lease and writer lock before registration and retains both throughout deletion. The preserved primary cleanup response reports 187.282 seconds in deletion and 187.285 seconds of writer-lock hold; registration itself took only about 2.3 ms. A fresh barrier test paused after an actual old-container deletion: an ordinary resolve with a 100 ms wait refused `busy:physical-readers`, then succeeded after release. The source default is a 15-second wait. Thus a cleanup at the observed scale can make ordinary agents time out. The original full-history run did not separately measure a concurrent outage. A production protocol needs short activation barriers and bounded recoverable reclamation, or an explicit maintenance-window contract; simply releasing locks without preserving catalog/reader protection would be unsafe.

**Rolling history compaction is absent:** [packing.rs:145](C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18/src/packing.rs:145) freezes the eligible publication/result/hint key set once a catalog exists. Later publications remain loose, including after `repack`. I confirmed that behavior on a new disposable store. This was appropriate for an exact-cohort comparison, but does not prove sustainable repeated tail ingestion and compaction. A new bounded experiment must cover multiple cycles with concurrently accepted writes, recovery between catalog generations, protected readers, and honest temporary-space/accounting bounds.

**Prototype growth bounds remain:** one catalog supports at most 64 packs, each at most 64 MiB, with 16 MiB indexes and an 8 MiB eligible-record bound. Maintenance also enumerates the cohort in memory. These are explicit experimental limits, not an unlimited-history production design.

The next work should address maintenance pause duration, repeated loose-tail ingestion, and fresh-process index access before freezing disk formats or public APIs. Packing should have a measured activation threshold so small stores do not pay unnecessary container/index overhead. Navigation's underlying edge-range work and the missing larger B measurements remain separate performance follow-ups. Neither browsing option needs to be dropped to acknowledge its costs.

## Controlling evidence

- [Latest coordinated Mac/B follow-up](C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-qnp-2026-10-19/followup-20261020/README.md).
- [Navigation report](C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-navigation-2026-10-19/report.md).
- [Packing original report](C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-packing-2026-10-20/report.md) and [separate completed comparison report](C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-packing-2026-10-20/followup-20261020/report.md).

Original reports preserve their original censors and unperformed cells. Read them together with the later follow-up; their earlier closeout status is not the latest combined coverage result. No production format, migration, or adoption is approved by this review.
