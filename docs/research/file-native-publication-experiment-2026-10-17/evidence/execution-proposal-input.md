# Bounded file-native document/graph experiment

**Updated October 17, 2026 after Pro review and the user's clarifications.** The user has authorized updating this proposal and handing it to meta orchestration for a fresh top-level task to run the bounded experiment. This authorizes experimental implementation and measurement within the gates below; it selects no production architecture, browsing contract, dependency or migration.

## Direction and decisions kept open

Build one small fixture/harness for a standalone Rust file-native document/graph engine. Handbook is one possible consumer; a memory/research collection is another. Meaningful YAML/JSON/Markdown documents, names, folders and context assembly must remain useful to an inheriting person or agent without installing either Handbook or the engine. Existing owner/versioned formats remain controlling, including exact Contracts JSON identities; the [format standard](../../specs/handbook-contract-membrane/structured-data-format-standard.md) has proposed general defaults.

**Both browsing options remain genuine candidates.** The user neither rejected a mixed/stale incremental view nor selected named generations. Earlier wording about lack of acceptance overstated an undecided preference; it is not an approval gate or evidence against either option. Compare their actual experience and cost:

| Option | Publication and ordinary-reader contract | What the experiment must expose |
|---|---|---|
| A: immutable document revisions plus an incrementally refreshed readable tree | One authoritative state selects exact revisions, membership, relationships and placements. Refresh only changed readable paths. API/assembly readers select a retained state; ordinary reads of the changing view can be mixed or stale during refresh. | Useful tool-free navigation/extraction, exact originals and saved assemblies, bounded refresh/recovery, truthful published/view-pending outcomes, freshness and mixed-read intervals. A status marker is diagnostic, not a snapshot lock. |
| B: complete retained named generations | Prepare, verify and install a complete meaningful directory under a unique name, then select it. A reader selects the exact retained path once and stays there. | Coherent reading/copying of that historical snapshot while newer versions publish, positive discovery evidence, synchronous readable availability at selection, materialization and retention costs. The selected version is current at selection, not continuously newest for that reader. |

Use the same corpus, semantic mutations and validation obligations. Option A is the best incremental implementation, not an intentionally broken control. Its observed A/B reads are a capability tradeoff, not automatically a correctness failure. B's required A/A reads are an invariant of B, not an unstated universal requirement imposed on A. Neither option provides a portable always-current multi-file browsing session for arbitrary unmodified readers.

**Dolt-family software is a comparator only in this experiment.** A pinned DoltLite release may supply useful performance/behavior comparisons. It is not the preferred production dependency, and faster numbers do not select it. Production investigation favors the small COW design and appropriate non-Dolt components. Reopening adoption of a Dolt variant would require a concrete unmet requirement in practical alternatives and a separate decision. Narrow MIT/Apache reuse remains possible after exact source/license/dependency review; it must not quietly turn into maintaining a full Dolt-derived engine.

The [Pro answer](../file-engine-experiment-pro-consult-2026-10-17/answer.md) is advisory. This revised proposal and the user's later clarifications govern over its stronger named-generation preference and Dolt adoption language.

## Fixed logical protocol for the experiment

Gate 0 records the exact experimental encoding, native primitives and budgets implementing these rules. It is a short design step inside the authorized task, not another request for routine permission.

### Selected publication and history

A complete candidate, existing content commit, or merge parent does not prove publication. Every selected publication binds the exact previously selected publication compared at the gate, the logical state reference, the readable-generation reference/digest when using B, and request/outcome evidence. For A, bind the target revision/namespace state and enough immutable information to resume rendering; mutable view progress is separate.

Keep publication lineage distinct from content/branch ancestry. A readable historical catalog derives from the selected publication and positive predecessor evidence. Unselected merge parents and higher-number orphan directories are not admitted to history. Missing required evidence remains an integrity failure or declared retirement, not permission to choose the newest survivor.

Compare publication identity rather than only a document-state hash: identical content and restoration of old content must not discard intervening outcomes. Request, attempt, generation and publication identities are distinct. Two candidates based on A receive unique owned staging/final paths; they never both overwrite a presumed next-number directory. Define acyclic digest dependencies; a manifest cannot hash a commit that hashes that same manifest. Preassigned publication IDs and separate content digests are one simple experimental choice.

### Requests, retries and uncertain outcomes

Capture a stable request ID, fingerprint of the proposed intent, base and exact candidate bytes. Selected evidence retains the original result and its publication. Delivery-receipt files are reconstructible caches.

Recheck deduplication at the publication gate. Same ID/same fingerprint resolves one accepted result; same ID/different fingerprint refuses explicitly. A request whose reply was lost at B must resolve the original B outcome after C changes or deletes the affected records. Resolving it must not rerun its mutation or derive a different result from C.

Absence from a receipt cache proves no noncommit. Absence from selected history while another attempt can still publish is only an observation as of that publication. A terminal cancellation/noncommit assertion fences eligible attempts; otherwise report a bounded conflict, busy or unresolved outcome. A successor C observing B with uncertain final synchronization must resolve or establish the required durability/retention of B's predecessor evidence before acknowledging C.

Keep existing whole-root/publication conflict semantics. Safe fixture-level disjoint combination requires explicit revalidation; it is not read-set OCC or general automatic merge. Only authorized pure semantic commands may reprepare. Arbitrary agent work and external effects are never automatically replayed.

### Readable closure and authority

Document revisions, membership, placements and required relationships publish as one authoritative state. Names are meaningful presentation with stable identities, not implicit approval, ownership or unbounded context inheritance.

Specify the required readable closure: documents, relationship targets, attachments, assembly output/input provenance, and allowed external/cross-store locators. Required navigation in B must not escape to CURRENT, mutable originals, or latest-title lookup. Include an assembly and its required retained provenance inside B when the copy exercise promises a self-contained packet. For A, identify exact immutable originals and retained assemblies separately from the changing convenience view.

Verify installed names and exact payload bytes at the declared strength; constructing a manifest of expected values alone is not verification. Count destination rereads, hashing and namespace checks. No automatic admission of arbitrary editor writes: import captures a base, bytes and proposed moves before validation, and does not mutate selected originals or overwrite uncaptured edits.

One application publication authority selects state. A database may durably prepare a retained candidate before application selection, provided reopen, pending/selected retention, GC coordination and lock ordering are proven. A database-owned head followed by a text-selector update has a separate discovery interval; record it honestly. Delaying acknowledgement does not make that interval disappear. Compare logical guarantees, not identical physical COW layouts, and never present two independent heads as one atomic publication.

## Corpus, copy policy and bounded workload

Use two independent store instances with independent writer/retention domains:

1. Project: overview YAML, requirement YAML, Markdown decisions, exact-byte JSON constraint/contract-like payloads, sources, typed relationships and placements. At the 10k tier include a 5,000-constituent entity and high-degree node. Include title/placement moves, rename/delete/reference races, uniqueness and cross-document budget predicates.
2. Memory/research: notes, source locators/quotations, classifications, exact-revision citations and a readable research packet, with no work-item lifecycle. Include a permitted unavailable external locator and a prohibited missing local target. A cross-store citation records the two selected roots and an explicit retained copy/agreement, not global atomicity.

Use deterministic UTF-8 prose: 70% around 4 KiB, 20% around 16 KiB, 10% around 64 KiB, plus bounded larger sources. Preserve LF/CRLF and exact JSON bytes. Include non-ASCII titles, case/Unicode-equivalent collisions, reserved names, trailing dots/spaces, bounded deep paths and attachment references. Keep portable path mapping separate from full human titles and stable identities.

The baseline copies bytes through an explicit bounded userspace read/write loop into new regular files. Do not assume std::fs::copy is non-cloning: [Rust's documentation](https://doc.rust-lang.org/std/fs/fn.copy.html) describes platform-specific copying/cloning and alternate-stream behavior. Record compiler version, actual helper paths, logical/allocated storage and filesystem compression/deduplication. The loop does not disable storage-level deduplication. No mandatory links, reflinks, filesystem snapshots, elevated privileges or daemon in the baseline.

| Documents per store | B generations retained, including initial | Minimum readable-file count per store | Approximate retained payload floor per store | Assigned workload |
|---:|---:|---:|---:|---|
| 1,000 | 20 | 20,000 | 0.24 GiB | 1/8/64-document edits and 1/2/4/8 independent processes; small repeated races and preparation policies |
| 10,000 | 10 | 100,000 | 1.18 GiB | Large compound entity, fanout, representative batches and contention |
| 100,000 | 3 | 300,000 | 3.55 GiB | Initial state plus one 1-document and one 64-document publication; whole-namespace/copy scaling |

These are experiment allocations, not product limits or a rolling deletion policy. Explicitly map the 19/9/2 later publications to workloads at Gate 0. Isolated small correctness/race trials are separate fixture runs and count toward total resource caps; do not silently turn all size/batch/agent/fault dimensions into a Cartesian product. Any 100k combinations not measured remain labeled unmeasured. Two large publications do not support useful p95 estimates.

Inventory originals, readable copies, directories, relationships, manifests/catalogs, metadata, pending candidates and abandoned attempts, not just retained payloads. At the stated mix a full 100k candidate is about 1.18 GiB; eight concurrent candidates would add about 9.46 GiB before other storage. That arithmetic is a budget warning, not a required large-tier eight-agent run.

Declare hard generated-data and wall-clock caps before generating fixtures, based on actual host capacity, and record reserved free space, per-case timeout and integration effort budget. Run the two consumers/policies sequentially where useful to stay bounded. Keep advertised fixtures retained within their promised lifetime; do not delete them merely to manufacture favorable growth. Stop at the declared cap and report reached/unreached measurements, not a fabricated capacity failure. Do not silently expand caps.

## Four execution gates

### Gate 0: protocol and reproducibility sheet

Record exact logical records, authority, publication/synchronization boundary per platform, readable closure and verification, retry/fencing behavior, fixture mapping, process model, resource caps, source/build pins and commands. Resolve experimental mechanics within this proposal. Stop for a genuine contradiction or scope expansion, not merely because production preferences are undecided.

Require no circular hashes, orphan-based selection, independently authoritative heads, replay to discover an uncertain outcome, or acknowledged publication loss under the declared process-fault model. Fix unique staging-attempt ownership in reused experimental code without changing production code.

### Gate 1: small native file-only correctness and browsing

Implement both A and B over one small candidate-state fixture representation before adding a new metadata backend. Exercise both consumers and the observations below. Use independent OS processes and deterministic barriers; approximately 30 repetitions for small race cases, with actual trial counts. Exercise every relevant small file/directory checkpoint; at growth tiers sample declared first/middle/last boundaries rather than multiplying the suite.

| Exercise | Required common result and option-specific interpretation |
|---|---|
| Tool-free discovery/extraction | An unfamiliar person/agent can find overview, related requirement/decision/source, useful context and interpretation instructions with ordinary browser/text/rg/copy tools. A explains its view target/progress and exact-source route; B leads to a positively selected named generation. |
| Two complete competing candidates | Both prepared from A; select only one, leave the other installed, kill processes at controlled points. Discovery/catalogs distinguish the selected version using positive evidence, not numbering, timestamps or seals. |
| Suspended ordinary reader | B reads overview from named A, pauses while B/C publish, then reads A's requirement: A/A with unchanged bytes/relationships. A records actual mixed/stale observations while its exact API snapshot/retained assembly remains coherent. |
| Alias/per-open selector control | Demonstrate possible A/B when reselecting current for each file. Do not claim a writer queue or atomic alias binds an ordinary browsing session. |
| Visibility and interruption | B selection implies complete readable closure is available. A may report published/readable-pending or failed; resume rendering only, preserve exact committed state and avoid duplicate mutation. |
| Copy/enum reader | B copying selected A and its declared closure remains A while newer versions publish. A copying its mutable view reports actual coherence limits; copying retained assembly/original sources has the stated narrower guarantee. Copying the whole changing store is a separate backup problem. |
| Independent agents/stores | Private capture does not change peers' base; stale selection conflicts/revalidates explicitly; acknowledged changes are accounted for. Holding project publication coordination must not stall research-store lookup/publication. |
| Semantic combination | Revalidate safe disjoint fixture changes. Refuse delete-target/add-edge, uniqueness and cross-document-budget violations even after a conflict-free textual/row merge. |
| Context assembly | Resolve from one selected state; retain ordered revision IDs, byte digests, recipe/renderer versions, omissions/errors and output digest. Reassemble after source/title changes with identical retained inputs. Cross-store roots are a vector, not one transaction. |
| Damage versus view lag | Missing selected authoritative payload/state record/required target is a typed integrity/reference failure, not absence. A missing projection file during declared refresh is separate; do not confuse it with lost authoritative bytes. |
| Retry/identity | Repeated failures in one process do not wedge staging. Concurrent same-ID/same-intent attempts resolve once; different intent with same ID refuses. Fence eligible attempts before declaring terminal cancellation/noncommit. |
| Lost reply after later publication | Select B with original request/result, kill before receipt/reply, publish C, resolve B's exact original result without replay. Also cover uncertain B synchronization followed by C. |
| Editor/import | Capture bytes/base/moves; later editor changes cannot change admitted candidate. Preserve exact JSON identities, do not reserialize during reads, mutate snapshots or silently overwrite user edits. |
| Retention/suspension | Retain all advertised snapshots/assemblies and needed bases in this slice; a paused process or elapsed TTL is not death. This establishes retained-reader behavior, not actual GC safety. |
| Naming and ordinary applications | Preserve identity under title/path change, handle portable collisions, and separately observe Explorer on Windows and a normal Linux file browser. API pathname success alone is insufficient shell-usability evidence. |

Both options fail on lost/duplicate accepted changes, incorrect authoritative membership, broken exact-source provenance, or hidden authority changes. B additionally fails on mixed/missing advertised selected closure. A fails if it claims snapshot isolation for its mutable tree, misreports committed work as noncommit, cannot resume a refresh, or cannot expose useful exact sources. Mixed ordinary reads alone do not disqualify A as a product candidate.

### Gate 2: growth, preparation and contention

Measure both options with their best truthful incremental/full-generation behavior and identical semantic validation. Compare:

- Speculative final preparation outside the short publication lock: include discarded candidates and repeated validation after whole-root conflicts.
- One per-store admission slot around final refresh/revalidation/materialization/selection: agent thinking, private input capture and external work stay outside. A native process lock suffices to explore this scheduling policy; no durable queue service is required.

Include a slow/large request competing with a bounded sequence of small ones. Bound outstanding attempts, pending bytes, retries and waits, with explicit conflict/busy/backpressure/cancellation results. Record per-request outcome and maximum wait as well as throughput; do not infer FIFO fairness or universal starvation freedom.

Report initialization, fresh-process positive/negative lookup, preparation, semantic validation, diff/combination, copy/view refresh, destination verification, file/directory sync, lock wait/hold, selection, acknowledgement and recovery separately. Record live memory, bytes read/written, helper operations, namespace/file counts and retained/pending growth. Use p50/p95 only with suitable actual samples and named estimator; report sparse large-tier observations individually. Uncontrolled OS cache is not cold-device testing.

Return both browsing experiences and the measured tradeoffs without selecting either. Whole-store generation cost proportional to bytes/names cannot be fixed by a faster metadata tree. A's cheaper incremental refresh must be presented with its actual observation/recovery limits, not scored as equivalent snapshot isolation. There is no preselected product latency/storage SLA.

### Gate 3: bounded metadata comparison, conditional on earlier evidence and caps

Continue with the existing lazy Rust COW baseline plus **at most one integrated external comparator** when the harness has a valid logical contract and remaining budget. Neither A nor B needs to be selected as the final product to run comparable workloads. If full external-file adapter work would force an unresolved product choice or broad engine implementation, preserve earlier results and run only a clearly labeled narrower component comparison, or report the unmet gate. Do not invent comparability.

DoltLite 0.51.0 is the first optional **benchmark-only** comparator. Begin with a small native Windows C/Rust integration and authority-fit effort cap recorded at Gate 0. Identify the loaded build, close/reopen an exact retained revision from another process, exercise two-process contention and retention across later writes/GC, and prove the external publication/outcome strategy for any integrated comparison. Native MinGW/GNU success must not be called MSVC qualification. If this needs a broad wrapper port/private-internals dependency, stop that route. The alternative single integrated comparator may be non-Dolt SQLite; do not grow the task into three complete backend implementations.

Use fresh fixtures and one exact DoltLite build across processes. [0.51.0](https://github.com/dolthub/doltlite/releases/tag/v0.51.0) preserves format 12 but documents incompatible numeric-key encodings in some 0.50.15-0.50.18 data/history with no included migration. Release pin: 50d7b303b4490cacdd55e9deba9f37a5f00d50a4. The research HEAD b92803229f229125795dd6bee0a6f704e7f841f5 is newer; never silently assign its fixes to the release. The [release Rust wrapper](https://github.com/dolthub/doltlite/blob/50d7b303b4490cacdd55e9deba9f37a5f00d50a4/packaging/rust/README.md) documents its Windows gap and alternative custom-build rusqlite route.

Match external-file outcomes, historical reopen, conflict behavior and retention for each compared browsing option. A promising adapter durably prepares/pins backend candidates before application selection; prove cleanup/publication/GC coordination and lock ordering. Backend default branch state is not a competing application head. A database-authoritative publication with a lagging text selector may fit A's explicitly stated discovery contract; it does not satisfy B's synchronous selection contract by assertion. Score that difference, not a blanket database failure.

Add real metadata-page/root damage, fresh-process positive/negative lookup, historical reopen, pin-registration/GC races, pending candidate lifetime and deliberate retirement. Preserve metadata evidence distinguishing pruning from damage. SQLite needs explicit retained logical states/history/outcomes/pins; a live read transaction is insufficient. Pin a version with the [WAL-reset fix](https://www.sqlite.org/wal.html), synchronization settings, and actual library build. DoltLite uses its own chunk WAL, not stock SQLite WAL.

DoltLite cases: same branch versus private branches, stale read-to-write upgrade, expected-ref races, a suspended reader delaying final publication/busy timeout, open iterators during GC, named history retention/deletion, and reopen around sealed-root faults. Document source support separately from executed release behavior.

Only if independent branch diff/merge is an actual learning need within remaining bounds, construct equivalent document/edge maps through different histories and measure root equality, changed nodes, diff reads and semantic validation. Different roots are not a COW correctness failure. Prolly complexity must earn its cost in diff/sharing work; no new Rust prolly engine is required. No Dolt adoption or full-engine fork is authorized by any benchmark result.

## Fault oracle and native platform scope

Inject process exits, external kills and reported I/O failures around payload write/sync/install, generation/view file/dir work, destination verification/seal, unique final installation, publication-record/selector staging and sync, expected-publication/dedup checks, selection, containing-name synchronization, receipt-cache delivery and reply. A also injects interruption of post-publication refresh. Include Windows handles without delete sharing and bounded refusal/retry behavior.

| Checkpoint/condition | Required interpretation |
|---|---|
| This attempt failed before any possible selection | Previous selected authority is usable; this attempt did not publish. Resolve earlier eligible attempts before a request-level terminal noncommit claim. |
| Selection may be visible but required synchronization is incomplete | A qualified uncertain outcome is permitted; preserve candidate/publication evidence and resolve before replay or destructive cleanup. |
| Required boundary succeeded, then process died | Exact publication and original outcome remain recoverable, possibly beneath successors. Generic rollback/permanent uncertainty is not a pass. |
| Process-only interruption, functioning filesystem | Readable old or new authoritative selector/state, with B's selected generation complete. A's projection may need resumable refresh with truthful progress. |
| Deliberate corruption or identified storage error | Typed integrity/error result under the declared case; do not use it to excuse ordinary interrupted staging. |
| Explicit retirement | Recorded retirement semantics, distinct from unexpected missing retained evidence. |

For each OS record actual API ordering and evidence limitations. Flush new nested namespace entries as required; do not assume syncing one file or the top generation directory persists every nested name. B installs a new uniquely named directory, not portable replacement of a populated current directory. A uses complete file replacement rather than deliberate torn writes. Success/error interpretation is API-specific.

**Native Windows/NTFS and native Linux/ext4 runs are required.** Discover available execution hosts at the start and complete useful independent work if one is unavailable, but report that exact qualification gap instead of claiming completion. Linux on WSL2 ext4 is separately labeled; /mnt/c is not equivalent. Include an actual ordinary-browser exercise where available, with a recorded unmet UI check if unavailable. macOS/APFS is a required follow-on support target; carry the same logical suite plus native naming, open-handle, directory sync/full-flush and Finder checks. No macOS support claim before native evidence.

Process death leaves OS caches alive; simulated post-flush errors are not power-loss simulation. OS-crash/reboot, hardware power loss, production reclamation and full owner integration remain later qualification. Checked point reads and explicit audits are not silently equivalent to existing owners' whole-history checks. Local hashes/history alone do not detect coherent rollback without an outside witness.

## Deliverables and boundaries

Create one separate experimental Rust crate/harness and reports under a focused research directory, preserving the landed membership experiment and existing evidence. Select a nonconflicting output path at start. Reuse isolated experimental code where justified; no production owner migration, new service, general plugin ABI, graph query language, distributed consensus, automatic editor admission or production dependency adoption.

Deliver:

- Protocol/budget/platform sheet and exact source/toolchain/backend/settings pins.
- Reproducible commands, fixtures/seeds, deterministic concurrency/fault checkpoints and raw bounded results with trial counts.
- Side-by-side A/B browsing, recovery and performance results; fixed-snapshot and mutable-view observations clearly separated.
- Stage timings and storage/file counts including rejected work, measurement limits and unreached cases.
- Backend comparison labeled integrated versus component-only, with Dolt explicitly benchmark-only.
- A requirement-to-test/evidence matrix carrying graph, assembly, native platform, retention and remaining qualification obligations.
- A compact recommendation of what the evidence supports, what needs a product choice and what remains unproven. Production selection returns to the user/meta.

Resolve routine experimental decisions within these bounds; do not ask the user to reapprove the existing research request. Stop or return a partial-evidence report for genuine missing infrastructure, exceeded caps, correctness failures that cannot be repaired within scope, or required scope expansion. Do not silently remove obligations to claim success.

## Evidence to carry into the fresh task

- [Dolt/GitButler decision memo](README.md), [component matrix](components.md), [DoltLite/Dumbo evidence](doltlite-dumbo-evidence.md), [visibility/OS analysis](visibility-and-publication.md), [research review](review.md), [native Windows probe](visibility_probe.py) and [its results](visibility-probe-results.json). Historical recommendation wording in those source reports is superseded by this proposal.
- [Continuing Pro consultation and reconciliation](../file-engine-experiment-pro-consult-2026-10-17/README.md), with exact prompt, answer and uploaded packet retained unchanged as historical consultation evidence.
- [Earlier Pro consultation](../standalone-file-engine-pro-consult-2026-10-16/README.md).
- [Membership/transaction prototype](../durable-membership-prototype-2026-10-12/README.md), [transaction comparison](../durable-membership-prototype-2026-10-12/transaction-comparison.md), [verification](../durable-membership-prototype-2026-10-12/transaction-verification.md), [review](../durable-membership-prototype-2026-10-12/transaction-review.md), Cargo.toml and src/main.rs in that directory. Baseline commit b6b3aed9beb3f5c832d2cd10978e71fdce68fdcf.
- [Storage audit](../storage-persistence-audit-2026-10-12/README.md) and [original context-assembler reconstruction](../original-python-pipeline-2026-10-16/README.md).
