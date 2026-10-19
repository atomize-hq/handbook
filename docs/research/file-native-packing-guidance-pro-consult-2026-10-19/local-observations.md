# Local observations before receiving Pro's guidance

October 19, 2026. These are coordinator observations and candidate questions, not Pro's conclusions or an approved implementation brief.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Three independent axes

1. **Platform parity** checks whether the current final recovery delta behaves under native Linux/ext4 and macOS/APFS. It should start from an exact source pin rather than from whatever branch happens to be checked out remotely. Prior baseline platform results are useful but cannot qualify this delta.
2. **Navigation scaling** concerns graph metadata and tool-free human/agent discovery. The new collection YAML is rebuilt by `collection()` and called by `delta()` on ordinary preparation. This can dominate an otherwise incremental edit. Its renderer, metrics and browsing contracts need an independent comparison.
3. **Physical packing/compression** concerns the file count, bytes, lookup and maintenance costs of retained internal records. It should preserve logical history and compare identical retention sets. Changing navigation or retirement policy at the same time confounds whole-publication measurements.

A fixed navigation profile can still provide a valid packing control for cold outcome lookup, history scans, packed-record reads and maintenance. It cannot support an end-to-end claim that ordinary large-collection publication has become incremental.

## Source-visible integration questions

Current `store.rs` reads publication and result records directly from regular paths. `accepted()` uses selected history membership/outcome indexes to establish commitment; `result()` then verifies retained result bytes. A physical locator must not bypass that logical admission check.

`Snapshot` and `resolve()` acquire the shared preparation gate. Collection uses its exclusive side before reclaiming content. Any pack-generation lifetime scheme should inspect whether this existing protection can establish the first working slice, before inventing a second generic lease system. Long maintenance pauses and independent writers still need explicit measurements.

`lifetime.rs` walks permanent evidence and live content separately. Attempt/candidate records can also remain necessary for pending work, packet ownership, exact replay and allocation accounting. Current directory enumeration and checked reads do not automatically work after loose records are removed; resolver integration must include verification, GC, replay and backup paths.

The current metrics classify bytes partly by directory path. Packed I/O needs meaningful record-role and physical-read counters; simply moving all bytes under a packs directory could destroy the existing measurement categories.

## Narrow primary-source checks during the consultation wait

- [Git pack format](https://git-scm.com/docs/pack-format) documents immutable object containers, indexes and optional deltas. It is a useful design reference; implementing Git's full format or copying GPL implementation is a separate choice. The first comparison need not adopt delta chains.
- [Dolt NBS table representation](https://github.com/dolthub/dolt/blob/main/go/store/nbs/table.go) describes hash-addressed chunks, lengths, indexed presence checks and checked data loading. This is closer to the physical-layout question than its SQL layer or a wholesale prolly-tree replacement.
- [Dolt file-manifest publication](https://github.com/dolthub/dolt/blob/main/go/store/nbs/file_manifest.go) separates consistent manifest reads from writer-excluded expected-state updates, and checks table-file presence and GC generation during updates. It motivates explicit catalog publication and reclamation ordering. It does not prove our native Windows crash contract.
- [LevelDB immutable table format](https://github.com/google/leveldb/blob/main/doc/table_format.md) documents independently addressable data blocks and an index/footer. That is a useful additional reference for bounded compressed reads without selecting a full LSM engine. Its [performance documentation](https://github.com/google/leveldb/blob/main/doc/index.md) makes block size a workload tradeoff.

These current web reads establish specific representation/publication patterns only. No upstream implementation was built, benchmarked, license-audited for copying, or selected as a dependency in this consultation.

## Questions the final brief must settle

- Which exact record families are packed in the first integrated cell, and which remain loose as explicit controls or necessary pending state?
- Is a catalog a physical locator only, or does a proposed integration inadvertently create a second commitment authority?
- What protects a reader that captured the previous catalog before loose files or an old pack are removed?
- What happens when a valid loose copy and a damaged packed copy coexist? Integrity checks must not become an undocumented repair/fallback policy.
- Can ordinary new publications append loose records while a historical pack is built, and what exact lock/recheck makes that safe?
- Is each block independently bounded, checked and decompressible? Is the canonical byte identity unchanged?
- Can an interrupted conversion, catalog switch, backup/restore or pack retirement preserve the same original outcomes without scanning all predecessor publications on a point read?
- What is the explicit physical-space peak while loose and packed copies coexist, including one preserved original, archive and owned test artifacts?

These questions remain open pending the Pro response and local reconciliation. No successor task has been dispatched.

## Platform and storage measurement clarifications

The earlier macOS prerequisite README reports a native arm64/APFS suite on the unchanged `379af4e` baseline: recovery, contention, lifecycle and ordinary extraction passed, with separately labeled Bash portability adaptations. This establishes a usable native route and relevant prior coverage. It does not qualify `f0904a63` or a future pack resolver. The native Linux prerequisite similarly belongs to that older baseline. Exact-delta testing can be focused without claiming that old tests ran on new code.

For allocated-space measurements, the collection harness needs an explicit per-platform method and calibration fixtures. [Microsoft's FILE_STANDARD_INFO documentation](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_standard_info) distinguishes allocation size from end-of-file. Linux's [inode documentation](https://man7.org/linux/man-pages/man7/inode.7.html) distinguishes allocated blocks from preferred I/O block size. Apple's [allocated-block property](https://developer.apple.com/documentation/system/stat/blocksallocated) warns that semantics depend on the underlying filesystem. Summed file allocation is not total unique volume usage in the presence of shared extents, compression or snapshots, and does not establish physical write amplification. Record unsupported measurements honestly.
