# Proposed next bounded engine work

October 19, 2026. Prepared after the completed Pro consultation and local reconciliation. Proposed for user review; no successor task has been dispatched and no production API, disk format, dependency or migration is selected.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Sequence

Use three separately reviewable tasks. Native baseline qualification and navigation development may run concurrently in isolated worktrees. Complete both, including native checks of the navigation delta, before packing implementation and its main measurements. Read-only donor inspection and packing design may happen earlier.

| Task | Main question | Prospective wall-clock bound |
| --- | --- | --- |
| Q: native parity | Does the final recovery delta behave on native Linux/ext4 and macOS/APFS? | 2 hours including closeout; 1,800-second capsule cap per host |
| N: navigation incrementality | Can ordinary edits avoid whole-collection scan/rewrite while preserving tool-free graph discovery? | 4 hours including closeout |
| P: packing/compression | Can we reduce retained internal file count and bytes without changing outcomes, lifetimes or recovery? | 6 hours, final 30 minutes reserved for termination/reporting |

These are prospective independent budgets. Do not reuse or reset an expired earlier run's deadline. A capacity/time refusal remains a visible censor, not a completed result.

## Shared requirements

- Standalone Rust file-native engine, with kind-specific semantics above its core. Keep the disk COW baseline, whole-publication conflict check and exact retained result bodies.
- Preserve A's incremental ordinary view and B's selected-once complete retained generation as distinct open choices. Full B construction remains a real full-generation cost.
- Keep exact current authored YAML/JSON and other payloads readable using ordinary folders and PowerShell/Bash listing/search/copy tools. No GUI browser, Explorer or Finder requirement.
- No new generated Markdown navigation or compulsory per-document index. YAML only where the source/tree lacks necessary facts. Packet/context assembly stays separate from ordinary commit.
- No original-store relocation/collection, automatic merge, consensus, hot backup, automatic compaction policy, broad plugin framework, API/format freeze or Handbook migration.
- Dolt-family databases remain comparators only. Source donors require exact pins and applicable licenses/attribution; dependency adoption is a separate documented choice.
- Read the proposed structured-format standard and preserve existing owner/versioned contracts; it does not approve global migrations.

## Q: exact-final native parity

Pin source/harness checkout `f0904a63cb2500b9bea3a4034d19cbe38bfac729` and expected Rust/Cargo core `07a8574e581e66ba58d7caac87238aa5f93918be`. Verify the final binding's source hashes and record the actual executable hashes. The long Windows GC core `fe1ee22a...` has unchanged GC/accounting code, but it does not qualify the last projection guard on the other hosts.

Run separate native Linux/ext4 and macOS/APFS lanes with small A/B fixtures:

1. Inventory filesystem, native target/compiler and dependencies. Preserve unrelated checkouts and evidence.
2. Build/test the pinned isolated experimental workspace. The known test command is `cargo test --locked --offline --workspace`; use actual existing harness entry points from this checkout.
3. Verify native file-lock lifecycle, immutable installation, replacement, directory-sync behavior and namespace handling.
4. Run the complete strengthened recovery/backup capsule and explicit final-delta supplements: external empty directories, recreated divergence, interrupted A plan backup, legacy/new-target replay, strict missing/corrupt evidence refusal, and transfer/checkpoint/retire/repeated-GC behavior.
5. Verify ordinary shell discovery/copy/extraction, fences, pins, candidates, packets and restored valid successors.

Use a platform-correct external resource sampler on Mac: the current generic Unix `peak_memory()` reads `/proc` and returns no Mac measurement. Label external harness corrections separately; do not change the pinned engine silently. No new H100k history or full cross-platform performance matrix is needed.

Output exact-source coverage tables, original failed/censored cases, raw bounded receipts and remaining durability limitations. Passing means the named mechanisms ran on those hosts, not physical power-loss qualification.

## N: remove ordinary navigation's full scan and full-output rewrite

Implement and measure two changes separately:

1. **Navigation-fact dirtiness.** Skip generation when the exported identity/title/kind/path/relationship facts did not change. Payload-only or unrelated metadata changes should preserve navigation bytes. Correct counters must include range scans and output work.
2. **Bounded output parts.** For navigation changes, compare a minimal readable grouped-YAML candidate against the existing complete renderer. Start around 256-512 records / 256 KiB per part, with stable-ID routing and local split behavior. These are experimental geometry bounds, not a permanent format choice. Handle large adjacency as an explicit measured exception or separately bounded part.

Preserve membership, source locations, stable IDs, directed typed ordered/multiple/external relationships and exact extraction. Target renames update their location records without rewriting unrelated incoming links. Generate no supplementary navigation for kinds that already expose the required facts. A small static/rarely changed descriptor is acceptable only when needed to explain parts; it must not become a global document list rebuilt every edit.

Version the new immutable projection profile explicitly. Optional navigation-file presence is not sufficient to identify it. Preserve historical profile bytes, sealed preparations, pending plans, packets and retained generation behavior.

Use N128/N1k/N10k, ordinary single and batched edits, payload-only edits, title/path/relationship changes, split/merge boundaries, high-degree documents, deletions and preserved external edits. Compare against an independent complete oracle and ordinary shell extraction. Report documents/relationships inspected, parts and bytes emitted, touched paths and replay dependency growth in addition to elapsed time.

Repeat focused delta recovery/compatibility on Windows, native Linux and native macOS. Keep B's complete materialization separately timed. Freeze the resulting source and navigation implementation before P. A missing N100k run remains unperformed; work counters at the bounded tiers must establish the intended local-update behavior without claiming a universal SLA.

## P: exact indexed packs with a bounded compression comparison

### Main cells and input sets

| Cell | Representation | Comparison |
| --- | --- | --- |
| L | Reclaimed loose store, unchanged retention and pinned navigation | Qualified baseline |
| P0 | Indexed immutable packs, original bytes without compression | File-count and physical-container effects |
| PZ | Same inventory/grouping, independently compressed records | Additional codec benefit/cost |

Use raw identity inventory `(role, stable logical key, raw length, SHA-256)`. Preserve original bytes before framing/compression, including JSON whitespace and exact result bodies. Do not reserialize, shorten identities, reconstruct hints, change retention or include the 246,560,316-byte test ledger as engine storage.

Implement two sequential integrated slices:

1. Selected publications, original accepted results and accepted retry hints.
2. Finalized retained attempt/candidate/allocation records, in separately managed packs after transfer reconciliation and lifetime integration.

Keep active/incomplete preparations, reservations, fences, durable pins, selectors, current authored sources, views, complete generations and packets in their existing forms. COW pages and historical source payloads may remain loose. Report per-role excluded records and bytes; an eligibility cap does not lower existing supported-input limits.

Pro's candidate codec is published `zstd` 0.14.3, checksum recorded in `codec-metadata-check.json`. It is BSD-3-Clause; assess locked transitive terms and native C build separately from MIT/Apache donor copying. Before PZ, permit a read-only comparison of at most 16 MiB per role against independent 64 KiB blocks. Choose one integrated layout, one compression level and no dictionaries, delta chains, background threads or experimental allocator features. Per-record compression is the initial candidate.

### Private modules and physical catalog

Introduce only typed record keys, a checked resolver, pack/index reader/writer and a physical catalog/maintenance operation. The catalog locates bytes, never establishes accepted membership or revives retired content.

Pro's catalog hypothesis is a new explicitly versioned `STORE.json` containing the immutable catalog digest. Initial activation is quiescent with old engine processes closed; old binaries must refuse the enabled version. Decide its exact representation within this bounded experiment and document compatibility; do not present it as an already selected production format.

Each operation acquires lifetime protection before capturing the current catalog. Use existing writer and preparation/lifecycle coordination initially. New semantic writes remain loose; building an immutable historical cohort may run outside the publication lock, but registration checks expected catalog/ownership/lifetimes. Preserve ordinary expected-head conflicts.

One canonical location per mapped key: packed mappings resolve there, unmapped keys resolve loose. Exact duplicates may coexist while staging/deleting; conflicting duplicates refuse. Missing/corrupt canonical data is integrity failure, never absence or automatic fallback. Candidate replay retains logical keys/hashes, not pack offsets.

Build private files, verify the complete input/index correspondence, flush/install immutable pack/index, build/verify catalog, register it, then reclaim superseded storage after readers drain. Maintenance reservations and ownership survive interruption. Release bytes only after actual successful deletion. Do not rewrite historical allocation-owner records.

### Format and memory bounds

| Bound | Candidate research limit |
| --- | ---: |
| Encoded pack | 64 MiB |
| Records per pack | 65,536 |
| Index per pack | 16 MiB |
| Registered packs | 64 |
| Reader index/cache budget | 64 MiB |
| Raw record packing eligibility | 8 MiB; larger valid records remain loose |
| Point-reader RSS | 256 MiB |
| Maintenance/checker RSS | 2 GiB/process, with finite host aggregate admission |

Validate types/versions, counts, ordering, duplicate keys, checked offsets/lengths, pack/index/catalog binding, codec and raw digest. Bound decoded output and decoder window/memory independently of advertised frame sizes. Index routing and cache behavior must not turn point lookup into full history loading. Full payload audit remains separate from bounded ordinary reads.

### Preservation, independent verification and faults

Before destructive conversion, capture and independently restore the exact **post-GC** operational baseline: 101,006 publications plus current retirement/control state. The older 101,003-publication archive is preserved but is not exact rollback for this baseline. Establish disposable authorization anew at the restored destination. Pass small correctness cases before long cleanup.

Independently decode packs without the new engine resolver and compare the complete sorted record inventory to L. Reuse the semantic history oracle above those byte checks. Verify every original accepted outcome, all three added publications, same/different intent, lost replies, absence, durable fences, retained snapshots, 101 retired-content refusals, current graph and unique-source removal.

Exercise interruption during pack/index construction, immutable installation, catalog replacement, accounting transfer, first loose deletion, partial cleanup, replacement of partly live attempt packs and old-pack deletion. Include orphan/complete-but-unselected artifacts and strict malformed/missing/corrupt indexes/records.

Exercise a read paused between lookup stages, old-catalog readers, semantic successor during build, competing maintenance and cancellation releasing barriers. Cover durable pins, eligible candidates, packets and pending A plans. On Windows, a handle without delete sharing must cause accounted deferred/retryable cleanup; catalog unreachability is not successful deletion. Use ordinary reads first, without mmap.

Finish with one packed H101k operational capture/restore, independent verification, reopening and a valid successor. Small A/B backups also cover staged/unregistered packs, deferred deletion and operational obligations. Admit a fresh restore destination separately while the source is quiescent, counting both datasets in peak space. Preserve original stores and archives.

### Workload and measurement matrix

- N128 operational A/B fixtures: full L/P0/PZ correctness on Windows; bounded candidate compatibility/recovery on native Linux/macOS.
- Authenticated H1k/H10k record subsets: index/compression/format geometry. These are not operational stores; do not rewind `SELECTED` to collected historical index roots.
- Existing valid post-GC H101k: primary Windows L/P0/PZ comparison, one selected destructive cleanup and packed restore. No new H100k history per codec/platform.
- N8k-N10k / H<=32: representative collection, ordinary edits and A/B publication under the frozen navigation profile.

Measure all three representations, including P0 versus L and PZ versus P0: logical bytes, calibrated file allocation, files by role, index/catalog bytes, startup/steady RSS, record/storage/decompressed bytes, opens/cache hits, early/middle/recent/missing/cancelled/lost-reply lookups, prepare/replay/select/A-refresh/B-generation, construction/verification/registration/deletion/repack, and operational backup/restore. Preserve role attribution after paths move under `packs/`.

Use about 500 warm lookups per category, 20 fresh-process starts per representation and a few cache-qualified cold runs where possible. Fresh process is not OS-cache cold or device cold. Unsupported cache control, allocation or physical-write measurements remain explicitly unmeasured.

Prefer one active long mutation destination, frozen inventories and additive candidates; use forced representation selection while duplicates coexist. Count duplicate bytes as candidate/peak space. Report final compacted footprint only after actual chosen cleanup. No multiple simultaneous long-history mutation lanes.

### Global resource and decision limits

- Retain the existing **32 GiB combined owned logical-byte** and **5,000,000-file** ceilings, with **1 GiB / 100,000 files reserved inside them** for closeout. Re-census current originals, archives, clones, stages, builds and evidence; prior closeout numbers are not free capacity.
- Preserve existing host free-space reserves and establish an explicit Mac reserve before dispatch. Do not silently delete evidence or increase allowances.
- Ordinary probe 120s; long pack/capture/restore stage 1,800s; complete independent verifier 900s. Admit worst-case coexistence before each stage, including raw/compressed candidates and restore.
- Stop destructive work immediately on identity/outcome/content loss, changed request semantics, unsafe deletion, incorrect allocation release or changed original/archive hashes. Preserve failing evidence.
- Performance shortfalls are measured results; resource/deadline exhaustion is a censor with its achieved prefix. Do not reset caps and relabel a continuation as the original run.

Prospective decision rules, to freeze before dispatch: exact equivalence/recovery must pass; at least 95% fewer files in eligible packed classes; PZ needs at least 20% smaller pack-plus-index bytes than P0 to justify compression; compare both P0 versus L and PZ versus P0 with roughly 2x matched warm-lookup limits, reporting absolute costs and distributions. For representative ordinary publication, propose no more than 20% median regression attributable to packing, with a 1ms absolute noise allowance; selection, preparation and view work remain separate. These are research selection criteria, not production SLAs. An uncompressed-pack winner is a successful experiment.

## Return package and deferred work

Return exact source/core/binary/codec pins; logical inventories and retention classes; format/catalog activation protocol; independent verifier evidence; platform/functional/fault coverage; per-cell storage/lookup/publication/maintenance/backup tables; peak coexistence; failed and censored attempts; and a keep/adjust/reject recommendation.

Physical power-loss qualification, hot backup, automatic compaction policy, mmap, delta compression, general retention policy, broad API/plugin design and production migration remain later decisions. The three proposed tasks require user dispatch authorization; this document does not launch them.
