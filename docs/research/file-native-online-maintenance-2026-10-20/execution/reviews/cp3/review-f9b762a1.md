# Independent CP3 review: exact f9b762a1

**Bounded engineering disposition: REMEDIATE. Full experiment qualification: INCOMPLETE.** Four required findings follow. No accepted-outcome data loss was established. The reviewed checkpoint is not ready for an engineering integration verdict, and the small receipts cannot qualify the experiment.

Reviewed exact `f9b762a1a0d352564542bb4850ad77ee14a7c6e4` against reviewed CP2 `7a6ee9db03f5cd2e388129a9019c0d5f274a1844`, in `C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook`. HEAD matched and worktree was clean. The complete six-engine-file diff, complete paged/pack/packing/physical-lifetime implementations and direct API/Store/I/O causal neighbors were read. All 25 source/Cargo entries match manifest sizes and SHA-256. Manifest SHA-256 is `0b4e3ad991c3b5e453ec9a0443fe427376cdbdcb51f00a3ad93fc2507b652293`. Binary: 3,202,560 bytes, SHA-256 `e7f648ec50bd0facab8c9e20eb0a0fe0d8d80dc32ecbe4cfb3986822bd2604d9`. All eight manifest run bindings match.

The five full consultation documents, proposed structured-data standard, CP1 bd6 and CP2 b876/7a6 reviews, full CP3 README, manifest and reproducible harnesses were read. Existing owner/version contracts control; the format standard remains proposed. The five-axis code-review-and-quality skill was applied locally, with the explicit no-subagents instruction controlling the review workflow. No engine, Cargo, fixture mutation/copy/recovery, independent oracle, indexing/bootstrap, worktree, commit, source edit, subagent or additional chat was used. Only this review directory receives output. Source-read completion and freeze release were sent to the coordinator before report generation; source is not reread after release.

GitNexus is **UNKNOWN**. The exact-checkout retained impact/detect records report an unregistered repository. Manual blast radius remains **HIGH**: all eligible record reads/exists, resolve/retry/select and rolling eligibility -> captured PhysicalRead/index cache/paged Reader; build/consolidate/convert -> flush/activation; immutable Work -> ordinal cleanup and retirement. This is manual causal tracing, not an inferred graph result.

All engine line citations below are within `docs/research/file-native-incremental-engine-experiment-2026-10-18/src/` at the reviewed pin.

## Required findings

### F1 [P2] Enforce the caller's bound before decoding the first mapped record

Primary: `packing.rs:85-86,115-121`; callers: `io.rs:117-125`, `store.rs:263-264`.

Before CP3, locate returned an entry and the chosen body was read with the incoming read limit. CP3's locate eagerly invokes `index.read(...pack::MAX_RAW...)` for every hit, including the first. Only after the entire route and raw decode does read check `e.raw > limit`. A mapped 8 MiB record passed to a 64 KiB bounded hint read, or any smaller read_bounded caller, therefore allocates encoded/raw buffers and runs decompression before the intended caller limit refuses it. No malformed advertised frame length is needed: the descriptor and entry can be internally valid. Such a hint would eventually fail its semantic parser, but its read limit must already protect that input boundary.

Fix: carry the caller limit to the first raw read or validate its authenticated entry length before decoding. Preserve all relevant-segment authentication and duplicate raw identity checking on otherwise successful reads; never turn oversize/corrupt mapped input into loose fallback. Exists/mapped callers need an explicit applicable bound.

Proof required: a correctly bound mapped record above a small caller limit refuses before record-read/decode counters or raw allocation advance; test raw and Zstd, exact-bound success, and a later relevant corrupt/conflicting duplicate. Source-only trace confirms 8,388,608 is supplied at line86 before 65,536 can be checked at120. No runtime test is claimed.

### F2 [P1] Charge construction index buffers before allocation under the 64 MiB envelope

Primary: `packing.rs:207-227`; construction: `pack.rs:175-180`, `paged.rs:141-147`; accounting: `packing.rs:54-73`.

Flush finishes the builder and retains the complete serialized `index: Vec<u8>` through installation and verification, but charges only the new paged Reader's 1 MiB at213-216. paged::build allocates the full output at147 before any construction-index reservation, while builder entries and existing physical caches are still live. Neither output Vec capacity nor the index-construction entries/tree/scratch appears in cache_bytes or borrowed_index_bytes. Thus a reported <=64 MiB charged peak is not the promised cache/transient bound including Vec allocations.

Static boundary example: the admitted 55,366 maximum-length225-byte keys serialize to 16,191,779 index bytes (1,154 leaves,25 intermediates,1 root). With60 MiB of legacy index residency, make_room sees60+1=61 MiB and permits the verifier. Legacy residency plus the emitted output alone is79,106,339 bytes, above67,108,864 even before reader/parser/scratch. During build, entries and the output also coexist before the 1 MiB reservation is attempted. This is an explicit arithmetic/state simulation, not a claim about measured RSS or a runtime fixture.

Fix: reserve a conservative complete construction/transient index bound before builder allocation or evict/stream to keep the whole admitted live set within64 MiB. Explicitly drop installed serialized buffers before verification where possible, but dropping them only after construction does not fix the earlier peak. Keep codec payload limits and maintenance-bookkeeping/RSS accounting separately honest; do not raise the cache limit or redefine index output out of the selected transient contract.

Proof required: an actual admitted near64 MiB legacy/mixed cache plus large paged construction/conversion, with simultaneous buffer capacities and relevant native allocation/scratch charged before allocation, meaningful eviction/refusal assertions and all retained records checked. The existing filtered cache unit alone exercises legacy loading and cannot prove construction accounting.

### F3 [P2] Full conversion collides on supported same-content physical instances

Primary: `packing.rs:315-335 ->207-212 ->197-204`; sink: `io.rs:141-145`. Supported shape: catalog ordering/uniqueness at `packing.rs:34-39`, four-duplicate-segment test at `packing.rs:240-250`.

A catalog may contain identical raw mappings in separate segments and independent physical birth instances; lookup deliberately verifies these duplicate routes and the new test constructs exactly that shape. convert_pairs rebuilds each original descriptor separately but assigns all outputs the same new stage birth. Two same-content source descriptors therefore generate the same pack hash and output path `packs/<newbirth>/<packSHA>.pack`. The second flush calls create_new on that already installed path and returns AlreadyExists. The same happens if different partitions share pack bytes even when their index keys differ.

This is a deterministic source-level path collision on an admitted representation, not a hash collision. Initial or repeated full conversion can fail although all records/identities are valid and capacity is available, leaving an owned incomplete candidate for abort. No selected-data deletion is alleged.

Fix: give conversion outputs unambiguous independently owned pair instances or deliberately support same-build verified reuse with a proven retirement/ownership rule. Preserve the selected no-re-adoption protocol, exact grouping/segment identities and normal pair paths. Do not adopt an arbitrary previously existing retired path.

Proof required: full conversion of at least two valid same-content input births in separate segments, and equal pack bytes with different index keys; exact discovery (including None), raw/codec identities, all outcomes, resumed/aborted ownership and eventual old-pair removal remain correct. The static proof follows identical hash output to the second create_new; no filesystem/engine simulation was run.

### F4 [P2] Complete the operation work measurements before using the counters as gates

Primary: `packing.rs:40-41,77,95`; `physical_lifetime.rs:98-101`; `pack.rs:94`; `paged.rs:76,95-96`.

The new pack_metadata_stats counter misses the two explicit metadata calls per descriptor in catalog validation and the selected pin metadata call. At64 descriptors this omits at least129 actual source-level calls at capture. Legacy Index::load's pack-size stat is also omitted. These are explicit successful calls, not unobservable kernel retries or failed-call noise covered by the README caveat.

The root-open binding parse is disclosed as excluded from pack_pages_parsed, but its parse time/allocation is also omitted from the dedicated parse timing/peak metrics. A cold four-height2-route lookup performs16 parses (four root-open plus12 descent), though12 physical reads remains correct because descent reuses cached root bytes. Report that distinction, rather than interpreting12 as all parsing work.

Per-key index-byte accounting also subtracts only pack_index_metadata_bytes; legacy index reads increment a different cumulative counter. In a supported mixed catalog4, a relevant whole index can consume megabytes while pack_key_index_bytes_max reports only the paged part or zero. Preserve legacy eager verification, but expose its per-key work and clearly limit the192 KiB gate to fully paged routes until conversion.

Fix/proof: count every explicit source-level physical metadata call, include root-open parse work, and expose or accurately name per-key paged versus legacy metadata totals. A cold known descriptor-count mixed capture/lookup should reconcile all successful stat calls, physical page reads, bytes and parses with the causal source trace; warm hits need their distinct counts. The point reader has no hidden root *read* regression; this is incomplete accounting, not a claim that its12 physical reads failed.

## Reviewed paths with no further established blocker

- **Typed format and allocation before page trust:** closed Root/Descriptor/catalog parsing; catalog4 required for page fields; old whole-index absence discriminator retained. Root version/height/bytes, complete key grammar and225-byte maximum validated before page buffer allocation. Slots <=48 bound parser Vec capacity. Page ranges use checked offset+length and <=16 KiB reads, pack/raw/codec bounds remain selected. Full-index constructors reject the incoming extent before allocation.
- **Positive/negative proof:** every relevant descriptor (<=4) is checked after a healthy hit. Parent hashes bind child first keys, counts and pack spans; leaf sorting/first/last and inherited sibling upper/last fences constrain misses. Height strictly decreases. Duplicate bodies are decoded and identities compared; errors propagate before loose fallback. Full operation raw bytes remain outside the index metadata192 KiB count.
- **Whole audit versus point:** audit clears cached pages, streams whole index SHA, then verifies contiguous preorder offsets and every node/fence/coverage. Activation/new flush additionally verifies whole pack SHA and every raw record outside Q/W. Ordinal traversal uses authenticated subtree counts and bounded cached page bytes. It reparses routes per ordinal; large throughput/cleanup measurements remain necessary. An unrelated damaged leaf can remain undiscovered by a point route by the expressly changed contract.
- **Cache ownership:** ordinary locate retains owned Entry/raw values rather than an Rc across another cache load. Page cache keys include offset,length,hash and index cache keys include physical birth/root/version/length/height. At most12 cached page buffers per Reader; recursion depth <=3 and <=48 entries bound parser state. The1 MiB Reader reservation is conservative for that bounded Reader itself; F2 concerns uncharged construction allocations outside it.
- **Initial upgrade:** exact selected/pre-upgrade pin handles/identities prepared outside Q/W under the maintenance owner. Candidate validation and stage creation precede exclusion. The builder drops its old PhysicalRead at441, avoiding its own pin in the drain; candidate pin is separate. Under Q it checks physical-parent STORE bytes, tries legacy/pre-upgrade pins and W without waiting, and retries outside Q. Routine catalog4 successors omit the drain. Actual native pre-generation schedules are still required.
- **Conversion:** full descriptor-by-descriptor conversion preserves the prior optional Discovery exactly; no global mapped key set or routine collateral-limit increase. Each input index/pack and raw record is fully verified; streaming key length/key/raw length/raw SHA correspondence is compared with verified outputs. F3 prevents claiming all supported full-conversion cases. Actual long-cohort size/quota/Work/census remains unmeasured here.
- **Semantic/lifetime preservation:** semantic publisher code and rolling discovery are byte-identical to7a6. STORE physical CAS remains separate from SELECTED conflict authority. Work still has128 KiB/discovery32 KiB bounds; added page fields are serialized into those same limits. Old loose-fallback frontier, all retained generations, extension-specific retirement, immutable Work checkpoints, never-reused birth claims, terminal exclusions/automatic-refusal ownership and stable-base policy remain inherited. Native deletion credit requires actual successful removal.
- **Cooperative cleanup:** one entry budget starts before Store::open; conservative35ms/800phase work cutoff and50ms/1000phase ceiling remain. Page loads check the same budget; no full index audit, history, rewrite or deletion moves under Q/W. Single native-call overruns remain indivisible and must be reported. Final large Work/drainage feasibility is not established by small smoke.

## Exact evidence inspected

Compact hashes/counts and static simulations are in evidence-f9b762a1.json. These receipts establish only their bound source/binary and workload.

| Run | Independently parsed evidence | Guard/sample limits |
|---|---|---|
| build7 | Offline final build and10 small units pass;1 heavy cache unit filtered | exit0/errors[]/censor null/two empty closure checks;454,582,272 B sampled aggregate peak;47ms max interval,8 >40ms |
| conversion | 28 raw events:14 exact final binary,14 retained old binary, all exit0/no violations. A/B actual20 mapped records and4 exact outcomes each; prior Discovery None; full engine physical and graph/source audit | exit0/errors[]/censor null/two empty closures;25,653,248 B peak;32ms maximum interval |
| smoke3 (historical build3) | 267 events:250 binary bb18f8b01340a61cb7228f33528a9e5152281475cf5a10fe6ee1e28e51983fb6,17 old binary.262 exit0,5 intended exit2,zero violations. Three+16 cycles/mode,52 outcomes/mode, held legacy/refusal/reaping/corrupt-root checks | exit0/errors[]/censor null/two empty closures;30,580,736 B peak;63ms max interval,7 >40ms |

Smoke3 is bound to its separate retained build3 source map and **is not final-f9b execution evidence**. The final conversion has only small leaf indexes; its max recorded per-key pages1/index bytes740 and capture7169 do not qualify the maximum geometry/cache. Smoke3 observed maxima are pages4/index bytes8964/capture26804; the source unit supplies12 cold-page geometry only.

Preserved smoke1 exits1 because its harness demanded an integrity-prefix from an old client that correctly refused unknown physical fields with zero bytes written. Smoke2 exits1 because its harness assumed an emitted segment0 instead of the valid omitted default. Build6 exits101 for unwrap_err requiring Entry Debug. These original stderr/guards were read; their failed whole runs remain failed. Engine audit calls are not independent Python full-oracle receipts.

## Required next action and qualification

Remediate F1-F4 within the existing source/resource envelope, freeze new exact source/binary/run bindings, and obtain a bounded remediation review. Source freeze release is not integration approval or runtime/resource admission. No clock reset, source allocation increase, protected disposal or automatic merge is granted.

The actual64 MiB cache boundary (including construction), complete independent v5 decoder/oracles, N64/H64 A/B matrix, native Windows/Linux/macOS final-pin lifetime/upgrade/recovery/compatibility/workspace/strict/packing capsules,1000 real physical activations, scaled H1002/H10002 rolling/concurrency, six-category H101007 lookup, full chosen long-history oracle, long streaming conversion/actual coexistence accounting, backup/restore plus accepted successor and ordinary B selected-once extraction remain mandatory. Full qualification is INCOMPLETE independently of these source findings. Existing CP1/CP2 and old failure evidence retains its original pin and meaning.

