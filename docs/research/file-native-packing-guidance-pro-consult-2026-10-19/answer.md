# Recommendation: **adjust the sequence; keep the architecture**

**The current COW architecture remains the best baseline for the next experiment.** The recent results justify testing a smaller physical representation of retained records. They do not justify replacing the tree, transaction model, publication boundary, or retention model.

I recommend **three separately reviewable tasks**:

1. **Finish exact-final native Linux/macOS qualification.**
2. **Remove the ordinary navigation scan**, with development allowed to run concurrently with the native qualification task.
3. **Run a bounded packing/compression experiment after those two tasks close**, using one pinned navigation implementation throughout its comparison.

Read-only packing design and corpus inspection can proceed during the first two tasks. Navigation is not a scientific prerequisite for measuring historical-record packing: an unchanged profile-2 control can isolate that question. Finishing navigation first is nevertheless the better project sequence because it closes a known ordinary-publication cost and prevents the packing experiment from becoming another combined layout, recovery, and storage investigation.

The proposed storage change should be **immutable packs plus a bounded index, a private record resolver, and a small physical catalog**. Keep the logical selected state, accepted outcomes, and content-retention decisions independent of where their bytes reside.

I read the packet in its stated order, including the supplied current source files. This recommendation uses that bounded evidence; it does not assume access to the excluded repository, stores, logs, or archives.

---

## 1. What the recent results establish

The recovery, restoration, and reclamation work has materially improved the experimental baseline:

- The operational backup captured the cooperating, quiescent store and was restored, independently verified, reopened, and followed by a valid successor.
- Actual collection removed approximately **83.7% of logical bytes without initially retiring content history**.
- All **101,003 original accepted outcomes** remained exact, with **101,006 publications** after the three test publications.
- The retirement pilot demonstrated the intended distinction between unavailable content snapshots and retained accepted outcomes.
- The fresh coordinator build and strengthened Windows capsule found no additional blocking recovery defect within their scope. file-native-packing-evidence

That is sufficient evidence to undertake a bounded physical-representation experiment. It does **not** establish physical allocation savings, large-collection navigation scaling, exact-final Linux/macOS parity, hot-backup behavior, or power-loss durability. Those limitations should remain attached to the next report. file-native-packing-evidence

### The remaining storage question has changed

Subtracting the **246,560,316-byte test ledger** leaves **641,363,366 logical bytes of engine/runtime data** in the final reclaimed clone. The publication, result, and hint files together account for **303,018 files and 176,379,124 bytes**. The separately classified attempt files account for **201,810 files and 402,632,280 bytes**, about **62.8%** of that engine/runtime footprint. The remaining object files account for about **9.7%**. These calculations use the packet’s final role census. file-native-packing-evidence

This suggests two distinct packing questions:

1. **Can packing sharply reduce the namespace cost of retained publication/result/hint records?**
2. **Can separately managed packs reduce the footprint of retained attempt records without making their content lifetimes permanent?**

Packing only COW pages would miss most of the opportunity. The selected source also reads publications, results, hints, and attempts through direct paths, so changing only `Objects::read` would miss most of the necessary integration. file-native-packing-evidence

The post-GC store is the correct baseline. Comparing packed storage against the original 5.46 GB tree would attribute already-demonstrated GC savings to packing.

---

## 2. Keep the core; introduce only the physical seams the experiment needs

Keep:

- The disk-oriented immutable COW tree.
- Independent preparation and the whole-publication expected-head check.
- Selected membership and outcome indexes as acceptance authority.
- Exact original result bodies and immutable publication identities.
- Explicit snapshot, candidate, packet, and content lifetimes.
- Both A and B, with their deliberately different browsing guarantees.
- Kind-specific validation above the shared core.

The smallest useful new boundaries are these:

| Boundary | Responsibility |
|---|---|
| **Typed record identity** | Names an existing immutable record and its exact expected bytes. |
| **Record resolver** | Reads that record from its selected physical location and verifies bounds and identity. |
| **Immutable pack/index reader and writer** | Frames, locates, optionally compresses, and verifies records. |
| **Physical catalog and maintenance operation** | Registers complete packs, changes canonical locations, and coordinates subsequent reclamation. |

These can remain concrete private modules. There is no need for a general storage-provider framework, codec plugin system, or public API commitment.

### Why packs are enough for the first experiment

A bounded immutable pack answers the immediate question: replace many individual files with indexed records while preserving exact bytes.

A general segment/LSM design would additionally introduce choices about levels, overlapping runs, compaction scheduling, tombstones, write buffers, and background maintenance. Those choices may eventually become useful, but the supplied residual census does not establish their necessity. **Keep new writes loose initially and pack explicitly selected immutable cohorts.** Measure how quickly the loose population regrows before deciding whether automatic maintenance is warranted.

Git’s published format already distinguishes undeltified entries from delta representations and provides sorted indexes mapping object identities to physical offsets. Delta chains are therefore not a prerequisite for borrowing the useful packing idea. [pack-format Documentation](https://git-scm.com/docs/pack-format?utm_source=chatgpt.com)

---

## 3. Task boundaries and order

### My preferred arrangement

| Arrangement | Assessment |
|---|---|
| **Finish native parity, then navigation, then packing, entirely serially** | Clear evidence boundaries, but unnecessarily delays independent navigation work. |
| **One task containing all three stages** | Workable with frozen checkpoints and separate budgets, but risks repeating the inherited-deadline problem and blurring which revision passed which tests. |
| **Separate native and navigation tasks in parallel, followed by packing** | **Recommended.** Independent work proceeds concurrently; packing receives a qualified, pinned baseline. |
| **Concurrent navigation and live packing implementation on changing source** | Poor experimental isolation and more recovery integration risk. Restrict concurrent packing work to design and read-only inspection. |

Separate tasks do not require three new architectural systems or elaborate coordination. One parent plan can name the dependency edges and require a short evidence handoff.

### Task 1: exact-final native parity

Use:

- **Source/harness checkout:** `f0904a63cb2500b9bea3a4034d19cbe38bfac729`
- **Expected Rust/Cargo core:** `07a8574e581e66ba58d7caac87238aa5f93918be`

Verify the file hashes from the final binding, rather than treating a branch name or executable filename as sufficient provenance. The `fe1ee22a…` measurements remain evidence for the unchanged GC/accounting implementation, but do not close the final guard/capsule gap. file-native-packing-evidence

Run independent **native Linux/ext4** and **native macOS/APFS** lanes:

1. Establish the actual OS, filesystem, Rust target, compiler, source hashes, and executable hash.
2. Run the minimal existing native prerequisites for the mechanisms actually used: store locking, immutable installation, replacement, directory synchronization, and namespace handling.
3. Build and run the meaningful Rust bounds tests.
4. Run the complete strengthened recovery/backup capsule on the exact final core.
5. Confirm the named final-delta cases are represented: external empty directories, recreated namespace incarnations, genuinely interrupted A backup state, legacy/new-target A/B transitions, strict corrupt/missing evidence refusal, and allocation-transfer reconciliation.
6. Perform ordinary shell discovery, copying, and exact-source extraction on small A/B stores.

The known build command is `cargo test --locked --offline --workspace` from the isolated experimental workspace. Use the pinned repository’s actual harness entry points; their excluded implementation should not be reconstructed from this packet.

**Do not create another 100k history for these lanes.** Small operational recovery, backup, retirement, and GC cases qualify the new mechanisms. Windows remains the existing long-history measurement lane. WSL evidence remains separately labeled.

One measurement repair is necessary on macOS: the supplied `peak_memory()` Unix path reads `/proc/self/status`; it does not provide an APFS-host RSS measurement. Use a platform-correct external sampler or resource-usage interface and report missing measurements as missing, not zero. file-native-packing-evidence

### Task 2: bounded navigation incrementality

The source confirms that `projection::delta()` calls `collection()`, which walks document headers and outgoing relationships. The later comparison against the prior file detects unchanged output **after the scan has occurred**. file-native-packing-evidence

Make two improvements, in order.

**First, skip generation when exported navigation facts did not change.** Compare the affected membership, identity, title, kind, source-location, and relationship facts. Payload-only changes, equal-content operations, or unrelated metadata changes should not regenerate navigation when none of those exported facts changed.

**Second, bound rewriting when navigation does change.** Incrementally deriving a new monolithic `collection.yaml` still leaves an output-size rewrite of the whole file. For larger collections, test readable YAML parts grouped under an ordinary, meaningful directory, with bounded ID ranges and local splits. For example, the experiment could target approximately 256–512 records and 256 KiB per part, with explicit handling for unusually large relationship lists.

Preserve these properties:

- Source paths remain directly discoverable.
- Stable IDs connect relationships to their source-location records.
- Direction, type, ordering, multiplicity, and external-reference status remain exact.
- A target rename updates its location record without forcing unrelated incoming relationship descriptions to embed a new path.
- A small collection descriptor is generated only when needed to explain the parts; it should not list every document or require a global rewrite.
- Kinds whose source files and folders already provide the necessary facts produce no supplementary navigation file.

A large adjacency remains a legitimate output-size cost. If changing one document requires emitting thousands of relationships, measure that work explicitly.

Use a new immutable projection profile for changed rendering semantics, preserving old selected projections, sealed preparations, and pending plans. Because navigation files are optional, profile identification should not depend solely on the presence of an optional filename.

Validate against an independent complete renderer/discovery oracle, and exercise ordinary tools on A and B. Include no generated Markdown navigation or compulsory per-document guides. B still constructs and verifies a complete generation.

### What navigation would confound during packing

Changing navigation and packing together confounds:

- Whole-publication latency.
- Preparation and replay time.
- Metadata read/write volume.
- A refresh work.
- B generation bytes, files, and construction time.
- Backup size and copy time.

It does **not** inherently confound a packing comparison over the same frozen historical-record inventory with the same pinned projection implementation. That is why a profile-2 control remains scientifically usable. My sequencing recommendation is about completing the project cleanly, not inventing an absolute prerequisite.

---

## 4. Ranked source and pattern donors

### Inspect now: two donor families, a few specific boundaries

| Rank | Source and inspection | Pattern to borrow | Benefit and mismatch |
|---|---|---|---|
| **1** | **[Git pack/index/MIDX format](https://git-scm.com/docs/pack-format)**; freshly read documentation. | Sorted identity-to-offset indexes, independently identifiable immutable packs, separate representation integrity and object identity, explicit duplicate-location handling. | Directly relevant to file-count reduction. Adapt the published concepts; Git object types, repository semantics, refs, CLI dependency, and delta machinery are unnecessary here. Git implementation is GPLv2, so it is outside the stated MIT/Apache source-copying allowance. [pack-format Documentation](https://git-scm.com/docs/pack-format?utm_source=chatgpt.com) |
| **2** | **[Dolt `go/store/nbs/table.go`](https://github.com/dolthub/dolt/blob/main/go/store/nbs/table.go)**; freshly read source. | Chunk records followed by index/footer; logical address ordering independent of physical record order; record integrity and total decoded-size accounting. | Useful compact-table structure. Its 20-byte hashes, Go interfaces, storage generations, and surrounding machinery are not requirements. The inspected file carries Apache-2.0 and Attic Labs attribution. [GitHub](https://github.com/dolthub/dolt/blob/main/go/store/nbs/table.go?utm_source=chatgpt.com) |
| **3** | **Dolt [relocation correction #11892](https://github.com/dolthub/dolt/pull/11892)** and **[GC read-barrier correction #11891](https://github.com/dolthub/dolt/pull/11891)**; primary explanations and selected patch/test material freshly inspected. | Tie lookup ordering to relocation; include already-running reads in the reclamation barrier; cancellation releases parked readers. | These directly improve the proposed fault schedules. Inspect `GenerationalNBS.Get` and the tests `TestBeginGCDrainsOutstandingReads`, `TestBeginGCInstallBarrierDoesNotSplitAReadInTwo`, and `TestBeginGCDrainRespectsContext`. Adapt invariants and tests, not Dolt’s full session/GC system. [GitHub](https://github.com/dolthub/dolt/pull/11892?utm_source=chatgpt.com) |

The relocation correction is especially relevant. Dolt documented a retained chunk being missed when a reader checked the destination generation, relocation completed, and the reader then checked the source generation. Its source-first fix addresses that particular movement order. **Our stronger local rule should be a captured catalog plus protected physical lifetimes**, so a lookup cannot straddle incompatible representations. [GitHub](https://github.com/dolthub/dolt/pull/11892?utm_source=chatgpt.com)

Current NBS index, reader/writer, table-set, and manifest implementation files were not successfully retrieved in this review. Do not call them freshly inspected. If dispatch still needs more donor detail, limit it to the index validator and manifest-install boundary.

Dolt’s current release page and recent storage corrections provide evidence of ongoing maintenance, not a correctness guarantee for an adaptation. [GitHub](https://github.com/dolthub/dolt/releases?utm_source=chatgpt.com)

### Later if measurements identify a need

| Priority | Repository; observed license | Specific possible contribution | Why it waits |
|---|---|---|---|
| **Later 1** | **[parcopy](https://github.com/lucifer1004/parcopy)**; MIT OR Apache-2.0. | `copy_file_internal`, `CopyBuilder`, `plan_copy`, `execute_plan`: bounded copy staging, cancellation, per-file results. | Useful if B construction or export remains copy-bound. The inspected copy path supplies per-file mechanics, not multi-file selection or the engine’s directory-sync/backup contract. [GitHub](https://github.com/lucifer1004/parcopy?utm_source=chatgpt.com) |
| **Later 2** | **[mmap-io](https://github.com/jamesgober/mmap-io)**; Apache-2.0. | `Segment::{new, as_slice}` and mapping APIs, if ordinary pack reads show material copying/syscall cost. | The inspected segment retains a parent mapping through `Arc`; it is not a complete mapped-window or retirement protocol. Windows mapped-handle and cross-process lifetime obligations remain ours. [GitHub](https://github.com/jamesgober/mmap-io?utm_source=chatgpt.com) |
| **Later 3** | **[immutable-chunkmap](https://github.com/estokes/immutable-chunkmap)**; MIT OR Apache-2.0. | `Map::{insert_many, update_many}`: group sorted updates to avoid intermediate allocations. | Potential narrow algorithm donation if profiling finds that cost. The completed cold-disk backend comparison remains controlling; do not reopen it to obtain a batch-update idea. [source](https://docs.rs/immutable-chunkmap/latest/src/immutable_chunkmap/map.rs.html?utm_source=chatgpt.com) |
| **Later 4** | **[file-locks](https://github.com/dolthub/file-locks)**; Apache-2.0. | `New`/`Close`, bounded acquisition and native advisory-lock lifecycle. | Its documentation covers Linux/Darwin/Windows, but a Go lock dependency offers no demonstrated benefit over the current Rust mechanism. Inspect only for a concrete native locking gap. [Go Packages](https://pkg.go.dev/github.com/dolthub/file-locks?utm_source=chatgpt.com) |
| **Comparator** | **[DoltLite](https://github.com/dolthub/doltlite)**; SQLite public-domain portions, Apache additions, separately licensed vendors. | Later inspect `src/chunk_store.c/.h` for compact-store comparison. | Current docs describe a beta version-controlled SQLite system. The chunk-store implementation was **not freshly inspected**. Its single-file/WAL machinery is broader than this pack slice; engine distributions do not automatically qualify its Rust wrapper. [Version Controlled SQLite · GitHub](https://github.com/dolthub/doltlite?utm_source=chatgpt.com) |
| **Comparator** | **[DumboDB](https://github.com/dolthub/dumbodb)**; repository declares Apache-2.0. | Document-consumer and finalization patterns, if a later consumer question needs them. | Current README says pre-1.0/not production-ready. No current implementation module was freshly inspected. NBS is the more direct donor for this experiment. [Version Controlled Document Database · GitHub](https://github.com/dolthub/dumbodb?utm_source=chatgpt.com) |
| **Later UX pattern** | **[GitButler](https://github.com/gitbutlerapp/gitbutler)**; FSL-1.1-MIT. | Operation snapshots and scoped workspace ownership. | Current oplog/locking implementation was not freshly inspected. The future MIT grant applies to particular versions; it is not blanket permission to copy the current core. Its workspace lock model does not establish storage isolation here. [GitHub](https://github.com/gitbutlerapp/gitbutler/blob/master/LICENSE.md?utm_source=chatgpt.com) |
| **Later cache only** | **[Starshard](https://github.com/houseme/starshard)**; MIT OR Apache-2.0. | `ShardedHashMap`, `shared_snapshot`, `SnapshotMode`. | In-process caching only. Current README describes lazy whole-map snapshot rebuilding; do not treat it as a durable or process-shared index. Source bodies were not freshly inspected. [GitHub](https://github.com/houseme/starshard?utm_source=chatgpt.com) |

### Not relevant to the next slice

| Repository | Assessment |
|---|---|
| **[Noms](https://github.com/attic-labs/noms)** — Apache-2.0 | Archived historical lineage. Current modules were not freshly inspected; prefer current Dolt corrections when learning storage-lifetime patterns. [GitHub](https://github.com/attic-labs/noms?utm_source=chatgpt.com) |
| **[EchoDB](https://github.com/surrealdb/echodb)** — Apache-2.0 | The inspected `Transaction::commit` publishes an in-memory snapshot through `ArcSwap`. The earlier transaction-shell comparison already consumed its main value; no missing transaction experiment remains. [source](https://docs.rs/echodb/latest/src/echodb/tx.rs.html?utm_source=chatgpt.com) |
| **[cow-utils-rs](https://github.com/RReverser/cow-utils-rs)** — MIT | String-allocation utilities such as `cow_replace`; unrelated to persistent-tree COW or packing. Source body not freshly inspected. [GitHub](https://github.com/RReverser/cow-utils-rs?utm_source=chatgpt.com) |
| **[OpenRaft](https://github.com/databendlabs/openraft)** — MIT OR Apache-2.0 | Deterministic fault-testing discipline is a broad inspiration, but consensus is outside this local-store problem. No immediate module inspection or dependency. [GitHub](https://github.com/databendlabs/openraft?utm_source=chatgpt.com) |
| **[d-engine](https://github.com/deventlab/d-engine)** — MIT OR Apache-2.0 | Distributed coordination and default RocksDB storage add unrelated machinery. Its documented platform list names Linux/macOS, not Windows; that is a documentation gap, not proof of compilation impossibility. Source bodies not freshly inspected. [GitHub](https://github.com/deventlab/d-engine?utm_source=chatgpt.com) |

**No storage dependency from this donor list is needed now.** Small Apache/MIT source reuse remains possible after pinning the exact files and retaining their applicable licenses, attribution, NOTICE material, and modification notices. A Rust adaptation would also make parser bounds, concurrency, platform behavior, and future bug tracking our responsibility.

---

## 5. The proposed packing/compression experiment

### Research question and cells

**Question:** With retention and logical state held fixed, how much do indexed immutable packs reduce files, allocated space, and maintenance cost, and is compression worth its additional read and ownership cost?

| Cell | Representation | What it isolates |
|---|---|---|
| **L** | Post-GC loose records | Existing baseline |
| **P** | Immutable indexed packs, uncompressed | Packing and namespace effects |
| **Z** | Same record membership and grouping, independently compressed records | Incremental compression effect |

Use **per-record compression for the first operational slice**. It is the simplest way to keep point-read decompression tied to the requested record.

Before implementing Z, permit one bounded sample comparison—at most 16 MiB of representative data per role—against **independent 64 KiB blocks**. This reveals whether cross-record compression provides enough extra benefit to justify read amplification. Do not build two complete compression systems or introduce delta chains.

### Codec recommendation

For the proposed Z cell, use a pinned standard Zstandard dependency rather than copying a compressor. The currently inspected Rust documentation and manifest identify **`zstd` 0.14.3**. Its current license is **BSD-3-Clause**, and the underlying reference C library offers BSD or GPLv2 licensing. This is a dependency proposal under the BSD terms, distinct from the permitted MIT/Apache source-donor category. [GitHub](https://github.com/gyscos/zstd-rs/blob/main/Cargo.toml?utm_source=chatgpt.com)

The two additional upstream references I recommend are **[zstd-rs](https://github.com/gyscos/zstd-rs)** and **[Zstandard](https://github.com/facebook/zstd)**. The observed repository has Linux, Windows, and macOS workflow evidence, including a successful macOS run; that does not qualify our pinned build or use of the codec. [gyscos/zstd-rs](https://github.com/gyscos/zstd-rs/actions/workflows/windows.yml?utm_source=chatgpt.com)

Own the locked transitive versions and license inventory, BSD notice obligations, native C build integration, decoder limits, and upgrade testing. Start with one compression level, no dictionary training, no experimental allocator features, and no background codec threads.

### Exact byte and identity domain

Freeze an inventory containing:

`(record role, stable logical key, exact raw length, raw SHA-256)`

The raw bytes are the existing bytes **before** framing or compression. Do not parse and reserialize JSON, normalize whitespace, shorten identifiers, or replace original outcome bodies with reconstructed equivalents.

For content-addressed objects, preserve the current object hash. Publications, hints, and attempt paths also need their typed stable key: a digest alone does not establish their namespace or ownership.

Include two sequential slices within the packing task:

1. **Selected publications, original results, and accepted retry hints.**
2. **Finalized retained attempt/candidate/allocation records**, separately classified after transfer reconciliation and resolver/GC integration.

Keep active or incomplete preparations, reservations, fences, pins, selectors, current authored sources, A views, B generations, and packet files in their existing forms for this experiment. Historical COW pages and source payloads can remain loose; their representation is not the dominant residual question.

Hints remain behaviorally preserved even though they may later be reconstructible. Do not combine packing with a new hint-reconstruction or attempt-retention policy.

---

## 6. Catalog activation, lookup, and lifetimes

### A concrete minimal catalog integration

My preferred hypothesis is to place the physical catalog digest in a **new explicitly versioned `STORE.json`**, installed atomically.

This leaves `SELECTED.json`, publication bytes, original result bytes, graph roots, and selected membership/outcome semantics unchanged. `STORE.json` supplies physical-location information; it does not supply an alternative acceptance authority.

The current `Store::open` rejects unsupported store versions. That provides a useful refusal boundary for old binaries opening a packing-enabled store. An already-open old process may have cached the old configuration, however, so **initial activation must be quiescent, with old engine processes closed**. Mixed-version live operation is not part of this experiment. file-native-packing-evidence

Required behavior:

- A legacy version explicitly means the existing loose representation.
- A packing-enabled version requires its catalog. Missing catalog means integrity refusal.
- Each operation captures the current catalog **after acquiring its lifetime lease**, rather than caching it permanently in `Store::open`.
- Catalog changes serialize under the existing writer coordination and check the expected catalog.
- Ordinary publication retains its existing expected-head conflict behavior.
- Packing an immutable historical cohort can coexist with later loose publications, provided registration rechecks the relevant ownership and lifetime conditions.

This avoids the dangerous rule “catalog absent means no packs,” which could silently misinterpret a damaged packed store as a valid legacy store.

### Construction and registration

Use this order:

1. Capture the eligible input inventory and reserve bounded maintenance space.
2. Build private pack and index files.
3. Verify every raw record and the complete index/inventory correspondence.
4. Flush and install immutable pack/index files using the native protocol.
5. Build and verify the immutable catalog.
6. Atomically register the catalog.
7. After reader/lifetime gates permit, remove superseded loose copies or packs.

Before registration, new files are unselected maintenance candidates. After registration, they are required storage. Deletion is a later step, with independently recoverable progress and accounting.

### Lookup and duplicate rules

Use **one canonical location per mapped key in a captured catalog**:

- Mapped keys resolve through that location.
- Unmapped keys resolve through their ordinary loose path.
- Exact duplicates may exist during staging and deferred deletion.
- Conflicting bytes for the same typed key refuse registration or audit.
- A corrupt or missing canonical packed record produces an integrity error. Ordinary lookup must not silently fall back to an old duplicate and conceal damage.
- An orphan record inside a valid pack cannot establish selected membership or acceptance.

A missing request remains `absent-as-of`; a cancelled request remains a terminal fenced noncommit. A storage failure must not be converted into either outcome.

### Bounded indexes and decoding

Proposed prototype bounds:

- **64 MiB maximum encoded pack size.**
- **65,536 records per pack**, rotating earlier for byte/index limits.
- **16 MiB maximum index size per pack.**
- **64 registered packs** for the bounded corpus.
- **64 MiB total reader index/cache allowance.**
- **8 MiB maximum raw record admitted to packing**, with larger valid records retained loose under their existing limits.

The packing eligibility limit must not lower the engine’s existing input or result limits. Report excluded records and their bytes so a favorable result cannot be manufactured by omitting expensive data.

Validate counts, key ordering, duplicate keys, offsets, checked offset-plus-length arithmetic, encoded lengths, raw lengths, codec/version, and pack/index binding before trusting them. Limit decompressor output and internal window requirements independently of frame claims. Zstandard’s own API documentation explicitly warns that a frame’s advertised decoded size may be invalid or untrusted. [Rust](https://docs.rs/zstd-sys/latest/zstd_sys/?utm_source=chatgpt.com)

Opening should validate the small catalog and required-file presence/bindings. Index data can be verified when loaded into a bounded cache; touched records verify exact raw digests. Full payload verification remains an explicit audit. Do not replace predecessor-chain startup work with an eager load of all historical payloads.

### Retirement and reclamation

Keep semantic retirement separate from physical deletion.

A retired snapshot remains unavailable even if some bytes persist temporarily inside a partly live pack. Reclaim that space by copying retained records into a new verified pack, registering the replacement, and deleting the old pack after readers drain.

Separate permanent publication/result evidence from content-lifetime attempts. Otherwise a few permanent records can unnecessarily retain a large pack of retired content.

Use the existing shared operation leases and exclusive lifecycle gate initially. Protect complete reads, including reads crossing loose and packed storage. Durable candidates retain **keys and hashes**, not physical offsets or an expired catalog.

Checkpoint/history enumeration and GC must enumerate logical records through the appropriate resolver/inventory. Raw directory listings cease to describe the complete population after packing.

Maintenance allocations need durable ownership and charging. Credit reclaimed space only after deletion succeeds; do not rewrite historical `allocation_owner` fields merely to make accounting smaller.

---

## 7. Correctness, interruption, and backup gates

The independent checker should decode the pack representation **without calling the engine’s new resolver**, then compare the complete sorted key/length/digest inventory against the frozen loose baseline. Reuse the independent semantic history walker above that byte comparison.

Required coverage includes:

| Area | Required cases |
|---|---|
| **Identity and acceptance** | Every original outcome; all three later publications; same-ID/different-intent refusal; orphan publication/result/hint cannot establish acceptance. |
| **Retry behavior** | Lose a reply, pack and remove loose copies, reopen, and obtain the exact accepted result. |
| **Content lifetime** | Retained snapshots readable; the existing 101 retired snapshots unavailable; current graph unchanged; unique retired source remains absent. |
| **Dependencies** | Reader lease, explicit pin, eligible candidate, packet ownership, allocation transfer, and genuinely interrupted A plan across maintenance. |
| **Malformed storage** | Missing index, wrong pack binding, omitted/duplicate index entry, truncated record, overflowing lengths/offsets, invalid codec, excessive decoded size, and raw-hash mismatch. |
| **Creation/publication interruption** | Exit during payload/index creation, after immutable installation, before/after catalog replacement, and before/after accounting transfer. |
| **Deletion/repacking interruption** | Exit after the first loose deletion, partial cleanup/restart, and replacement of a partly live attempt pack. |
| **Concurrency** | Reader paused between lookup stages; catalog replacement; competing maintenance candidate; semantic successor during pack construction; cancelled maintenance releasing barriers. |

On **native Windows**, additionally hold a file handle that does not permit deletion. Sharing refusal must leave the object accounted for and cleanup retryable. Catalog unreachability alone is not successful physical reclamation. Start with ordinary file reads; mmap adds no necessary capability to this first slice.

### The preservation gate needs the final post-GC state

The original operational archive covers **101,003 publications**, while the final clone contains **101,006**. It is not an exact rollback artifact for the later publications and retirement state. file-native-packing-evidence

Before destructive work:

1. Preserve a quiescent operational image of the exact post-GC baseline, including those additional publications and current controls.
2. Restore it into a fresh, owned disposable destination.
3. Independently verify it and reopen/recover it.
4. Establish disposable permission anew at that exact destination.
5. Run the small packing correctness capsule before admitting long-store cleanup.

Preserve the original store and original archive. Do not relocate or collect the original.

At the end, perform **one full H101k packed operational backup/restore**, including reopening, exact outcome verification, retained-content verification, and a valid successor. Small A/B cases should also capture registered packs, unregistered pack candidates, deferred deletions, reservations, and interrupted projection state.

The backup remains a quiescent regular-file operational capture. Packing does not turn it into a hot backup or filesystem-metadata image.

---

## 8. Measurement matrix without a Cartesian explosion

### Datasets and runs

| Dataset | Purpose | Planned runs |
|---|---|---|
| **N128 small operational fixtures** | A/B compatibility, recovery, ordinary edits, backup, retirement, and fault injection | All three native platforms; L/P/Z on Windows, candidate correctness on Linux/macOS |
| **Authenticated H1k and H10k record corpora** | Index scaling, record/block compression sample, point lookup, malformed-format tests | Main measurement host; bounded portable format checks elsewhere |
| **Existing valid post-GC H101k store** | Actual long-history lookup, packing, cleanup, and operational restore | Windows L/P/Z comparison; one chosen candidate completes destructive cleanup and packed restore |
| **N8k–N10k, H≤32 representative collection** | Collection-size effects, ordinary metadata/relationship edits, A refresh and full B construction | Windows paired controls; small native compatibility cases elsewhere |

The H1k/H10k corpora can be authenticated subsets of records. **Do not label them shortened operational stores**, and do not rewind `SELECTED` to old publications whose historical index roots may legitimately have been collected.

The actual H101k history is valuable for history pressure, but its repeated small content roots do not establish large-collection behavior. Keep the larger-N fixture as a separate axis.

### Metrics

Record, by phase and logical role:

- Logical retained bytes and actual regular-file counts.
- Physical allocation where the filesystem exposes an appropriate measurement.
- Index/catalog bytes and startup RSS.
- Requested raw bytes, storage bytes read, decompressed bytes, opens, and cache hits.
- Early, middle, recent, missing, cancelled, and lost-reply lookup latency.
- Preparation, replay, selection, A refresh, and full B generation separately.
- Pack construction, verification, registration lock time, deletion, repacking, backup, and restore.
- Peak temporary space, deferred-delete bytes, and pack occupancy after retirement.
- Instrumented writes, separately from physical storage write amplification.

Preserve logical role attribution after records move into `packs/`; otherwise the existing path-based counters could relabel result reads as generic control I/O. The supplied metrics implementation makes that a real integration concern. file-native-packing-evidence

Use a modest fixed sample plan: approximately 500 warm lookups per category, 20 fresh-process starts per representation, and a small number of cache-qualified cold runs. Report observed distributions without turning them into production SLAs.

**Fresh process is not cold storage.** Where cache eviction/control can be established, record the method and distinguish OS-cache cold from device-cache claims. Otherwise label results warm or cache-uncontrolled. Do not obtain a “cold” label merely by renaming or copying files.

For physical bytes, use allocation measurements appropriate to each native filesystem. On Windows, distinguish allocation size from the compressed-file-size field that was previously mislabeled. Directory metadata and device writes may remain separately unmeasured.

### Avoid repeated full clones

Use one working long-history destination and frozen input inventories. Build raw and compressed candidates additively before cleanup, and compare the representations with forced resolver selection.

While duplicates coexist, report their space as **candidate/peak space**, not as a completed compacted-store footprint. Obtain the actual final census only after the chosen representation’s cleanup.

Small fixtures can exercise destructive P and Z variants independently. A new full durable H100k history per codec or platform is unnecessary.

---

## 9. Prospective limits and stop criteria

These are proposed research bounds, not product guarantees.

| Resource or stage | Proposed limit |
|---|---|
| Combined owned logical bytes | **32 GiB**, retaining the existing accounting scope and counting new clones, archives, packs, stages, builds, and evidence |
| Combined file count | **5,000,000** |
| Reserved inside those ceilings | **1 GiB and 100,000 files** for termination, preservation, and closeout |
| New full-history destinations | **At most one active working/restore destination at a time** |
| Point-reader RSS | **256 MiB**, including bounded index/cache residency |
| Maintenance/checker RSS | **2 GiB per process**, with concurrent work admitted against a host aggregate limit |
| Ordinary functional probe | **120 seconds** |
| Native qualification capsule | **1,800 seconds per host**, prospectively admitted |
| H101k pack build, capture, or restore | **1,800 seconds per stage** |
| Complete independent verifier | **900 seconds** |
| Native parity task | **2 hours**, including closeout |
| Navigation task | **4 hours**, including closeout |
| Packing task | **6 hours**, final **30 minutes reserved** for termination and reporting |

Re-census before dispatch. The previous 24.59 GB closeout is historical evidence, not current available capacity. Preserve the existing host free-space reserves and establish a finite Mac reserve; do not convert separate lanes into additional grants beyond the combined ceilings.

Before each large stage, admit its worst-case input/output coexistence, index/inventory overhead, and cleanup reserve. If it does not fit, stop admission and report the required space. Do not delete preserved evidence or silently enlarge the limits.

### Distinguish three outcomes

**Correctness failure — stop destructive work immediately.**

Examples: changed accepted bytes, lost retained content, altered request semantics, catalog loss misclassified as absence, uncontained deletion, incorrect allocation release, or changed original/archive hashes. Preserve the failing clone and receipts.

**Performance failure — finish safe closeout and report the result.**

Examples: packing provides little namespace benefit, compression saves too little, index startup becomes expensive, lookup latency regresses materially, or retirement creates excessive repacking.

**Censored run — report the boundary and achieved prefix.**

A time, byte, file, or RSS limit reached before completion is a censored measurement. It is not a pass and is not automatically a correctness defect. Do not reset the deadline and present the continuation as the original bounded run.

### Suggested success criteria

For the first experiment, I would prospectively use:

- **Exact equivalence and recovery:** every required invariant passes.
- **Packing:** at least **95% fewer files in the packed eligible classes**, with total-store improvement and excluded-record coverage reported separately.
- **Compression:** at least **20% additional reduction in pack-plus-index bytes** over P to justify selecting Z.
- **Read cost:** no more than roughly **2× the matched warm lookup latency** for Z versus P, and no unexplained large startup regression.
- **Ordinary publication:** no material regression attributable to packing under the unchanged navigation control; preparation, selection, and refresh remain separately visible.
- **Lifecycle:** successful interrupted cleanup, partly live pack reclamation, and packed operational restore within the declared bounds.

Those thresholds are proposed decision rules. Freeze them before measurement; do not tune them to the observed winner. **An outcome favoring uncompressed packs is a successful experiment.**

---

## 10. What I would authorize for review before dispatch

The next proposal should contain a compact dispatch packet with:

1. The three-task dependency order and exact baseline pins.
2. The navigation candidate and its tool-free discovery oracle.
3. The typed record inventory, retention classes, and catalog-activation protocol.
4. L/P/Z cells, one codec pin, index/decoder bounds, and duplicate rules.
5. The fault matrix, independent equivalence checks, and operational restore gates.
6. The finite measurement/resource budget and explicit pass/fail/censor rules.

Keep production power-loss qualification, hot backup, automatic compaction scheduling, global retention policy, mmap, delta compression, public format/API freeze, and Handbook migration for subsequent decisions.

**The next architectural move should be smaller physical storage for the same logical engine.** Close native parity and navigation incrementality as separate bounded work, then test whether exact indexed packs provide enough file-count and allocation benefit to justify their resolver, catalog, and reclamation obligations.
