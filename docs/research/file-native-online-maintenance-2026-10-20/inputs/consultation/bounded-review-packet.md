Apply these instructions throughout this session.

Look beyond the immediate task to understand the outcome I want it to produce. Treat my requested approach as a starting hypothesis about how to achieve that outcome.

For each substantial task:
- Identify the immediate deliverable, the outcome it serves, and the decision or action that should follow.
- Consider whether a higher-level approach would achieve that outcome more effectively.
- Use available context and evidence to identify what changed, why it matters, and what should happen next.
- Surface the most consequential decisions, with practical options, tradeoffs, and supporting evidence.
- When work recurs, consider whether a durable workflow, feedback loop, or monitoring system would serve the goal better than repeated one-off tasks.
- Identify the data, tools, access, and verification needed to make that workflow reliable.

Briefly explain any proposed change in approach. Continue within my authorized scope; broader outcomes do not grant permission for additional external actions or ongoing automation.

Respect explicit deliverables and creative intent. Raise the level of abstraction when doing so helps accomplish what I actually want.

---

We are already working on this project task.

Task/spec: Advise on the completed standalone file-native document/graph engine experiment and scope the next bounded experiment around online cleanup, rolling history-tail compaction, and fresh-process lookup. Immediate deliverable: a decision-ready experiment brief and risk review. Outcome: determine whether this storage design can maintain low-cost operation and bounded physical growth under concurrent local agents, while retaining current readable files, exact accepted outcomes, and crash recovery. Next decision: whether to authorize and dispatch that bounded implementation experiment. This consultation does not authorize worker dispatch, code changes, migrations, or automation.

Current approach: A small Rust disk-oriented immutable COW tree and selected-publication transaction layer, with separately versioned immutable physical packing catalogs. The working hypothesis is to keep that core and improve the physical maintenance/read protocol. Inference from current code and tests: packing's main obstacles are its operation-wide reader barrier, deliberately frozen comparison cohort, and full relevant-index authentication/parsing on first access. Challenge that diagnosis and suggest a better abstraction or sequence if justified. This is an approach review; give architectural guidance grounded in the bounded evidence rather than a broad repository audit or a patch.

Project sources: The complete available review packet is inline below. There is no configured Project source sync, ZIP, or other attached repository content. Read the prior-state/invariant summary, measurements, source excerpts, then proposed experiment. Excluded source is unavailable and must not be inferred. Excerpts are genuine partial source from the pin below, with omissions explicit. Label design inferences and distinguish measurements from guarantees.

Evidence, blockers, constraints, and validation: See the bounded packet below. Keep both readable-browsing contracts, all accepted-outcome semantics, native platform requirements, explicit crash/recovery coverage, and honest resource accounting. You may propose better sequencing or split dependency-ordered slices, explaining why and where the complete required coverage lands. Do not silently turn required work into optional future work.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Is the current approach the best way forward? Give a direct keep, adjust, or pivot recommendation; tradeoffs; what to keep/change first; inspections; validation; and pitfalls.

## Prior state and invariants

- This is an isolated private experiment toward a reusable standalone file-based database primitive with graph relationships and extensible document kinds. Handbook is one future consumer, not the engine's domain model. No production adoption, disk-format freeze, or public API freeze is approved.
- Research integration: a0b92fe5dc64269508601dcddf7e2e2ca477e30d. Final 18-file Rust/Cargo core: d70f3864f2888d46d9112213b4a03f8d82745a8e. The code lives under docs/research/file-native-incremental-engine-experiment-2026-10-18/. Final integration has only later evidence/harness changes after that core. We independently matched all 18 file hashes and the retained Windows executable.
- Semantic commit authority is a selected immutable publication plus its logical membership/outcome indexes. A fully written candidate, pack, or catalog is not acceptance. Physical repacking must not alter selected semantic roots, exact accepted results, request/intent matching, cancellation semantics, deduplication, or conflict rules.
- Every accepted outcome remains recoverable, including lost replies and old requests. Retiring snapshot content can refuse that content without erasing the original accepted result. Physical bytes surviving in an old or partially live pack must not resurrect retired content. Protected readers, pending preparations, retained snapshots and saved context packets keep their existing lifetime protections.
- The current transaction conflict rule is whole-publication expected-head comparison. Keep it. Automatic merging, distributed consensus, and fine-grained transaction concurrency are outside this next experiment.
- A is a live incrementally refreshed readable tree; ordinary browsing may span an in-progress refresh, with explicit provenance/recovery. B is a fully prepared immutable readable generation, found by reading its selected binding once. Keep both contracts. Do not silently make B depend on an installed engine, remove required loose publication controls, or claim A has B's coherence guarantee.
- Current YAML/JSON and other authored document payload bytes remain exact, ordinary, human/agent-readable files with meaningful paths. Navigation is supplementary YAML when needed; no generated Markdown navigation guides or compulsory per-document index. Authored Markdown/context-packet prose are separate. A narrow self-describing JSON kind can omit navigation; general YAML completeness detection is not selected for this task.
- Historical/internal immutable records can be packed. Sources, current readable views, B generations, active reservations/work, COW pages, and controls remain outside the current eligible packing cohort. Packing is not the semantic authority or a general source-format migration.
- Windows, native Linux/ext4, and macOS/APFS are required platforms. WSL is supplementary. No GUI browsing tests are needed: PowerShell/Bash listing, exact extraction and readable-instruction checks suffice. Physical power-loss/reboot qualification remains explicitly unproven; Windows directory sync remains unsupported. Do not imply the proposed experiment completes that separate qualification.
- Dolt remains a comparator/inspiration only, not a preferred implementation dependency. No broad database/crate survey is requested. Suggest a narrowly useful existing abstraction/component only if it materially simplifies one of the three problems while preserving these contracts.

## What completed

1. Native qualification: final core passed five workspace tests on each platform. Windows packing capsule: 54 cases / 275 engine events. Native Linux and macOS: 52 applicable cases / 267 events each, excluding Windows-only sharing cases. Strict input, ordinary PowerShell/Bash extraction, and focused B controls passed. The full recovery capsule passed 24 cases / 282 events on all three platforms. The original Mac run was time-censored; a separately authorized follow-up completed it and parent raw-evidence review. Do not call that coverage still incomplete. The later Mac aggregate RSS peak was not persisted (null), though its live ceiling and native group samples were retained.

2. Navigation: profile 4 has bounded stable-ID hash-prefix YAML groups, local split/merge, and separate large-adjacency parts. At 10,000 documents in A, measured preparation changed from 2847.41 ms to 39.04 ms for a payload edit, and from 2800.65 ms to 220.77 ms for a title edit. Title output fell from about 1.94 MB to 50 KB, consulting 322 rather than 10,000 headers. It still visited 963 edge-index pages / 10,272 leaf pairs while returning no outgoing edges. N10k-B initialization hit 120 seconds; its edit matrix was unperformed. The later N8000-B packing comparisons are not a substitute for that missing navigation matrix. Navigation's low-level edge-range optimization and the larger B navigation matrix are separate follow-ups, not additions to this experiment.

3. Long-history packing: one authentic A operational store at H101007 (101,007 accepted publications) supplied 504,833 exact eligible records / 579,015,428 raw bytes. P0 raw and PZ record-Zstandard used identical role/range/count grouping: 16 packs, 16 indexes, one catalog. Pack/index bytes were 641,986,037 P0 versus 283,460,393 PZ: 55.85% smaller. Cleanup reduced eligible files from 504,833 to 33 (99.9935%). The whole post-cleanup engine, excluding its 246,560,316-byte test ledger, had 5,811 files / 345,898,733 logical bytes. COW objects/current views/etc remain part of that whole count. All 101,007 publications, 100,000 test-ledger rows, 101 retired-publication refusals and current graph/source/navigation checks passed independent verification. No whole-store pre-activation loose allocated-space baseline was measured; do not invent one. P0 and PZ build clocks were not a clean causal speed comparison.

4. Packed operational backup, fresh restore, complete verification and an accepted successor passed. Byte restore: 3.469 seconds. Full restored verifier: 89.140 seconds. Neither figure alone is end-to-end recovery latency. Every accepted outcome and exact source bytes were preserved.

5. Matched operational comparisons: N8000/H8 baseline, identical complete 28-record cohort, seven real publication requests, six original lookup categories (early, middle, recent, missing, cancelled, lost reply), 500 warm lookups/category, 20 new processes/category. All A/B loose-old, new-loose, P0 and PZ comparisons completed, including a later authorized B follow-up. Correctness and ordinary/warm gates passed. A PZ ordinary median ~386 ms; B PZ ~37.55 seconds versus loose-old ~39.99 seconds. These are small controlled observations, not SLAs or proof of a compression speedup. B still prepares a complete generation.

6. Frozen small-cohort file gate failed: A 28 records became 11 representation files, 60.71% reduction; B became 19 files, 32.14% reduction, including eight required loose publication-control duplicates / 10,268 bytes. Both miss the old 95% gate. PZ pack/index bytes were about 77.18% lower than P0 in A and B. Do not retroactively relabel the old gate passed. For the new experiment consider a measured activation threshold and realistic scale-specific criteria. B retains a loose publication binding for every accepted publication, including historical/retired selectors, to preserve tool-free selected-once browsing. Charge that ongoing file growth honestly.

7. Warm long-history lookup medians stayed close to loose (roughly 0.6-1.05 ms by category). Fresh-process P0/PZ medians were ~18-45 ms versus qualified loose ~1.0-1.4 ms. OS/device caches were uncontrolled; new process is not device-cold I/O. Current first access authenticates/parses the entire relevant index, not all indexes. Report engine operation time and total spawned-process latency separately.

## Additional review findings and supporting source

### F1. Long cleanup blocks normal agents

The actual full-cohort cleanup response records:
- wall: 187.2865864 seconds;
- deletion phase: 187.2824929 seconds;
- writer lock held: 187.2846957 seconds;
- catalog registration phase: 0.0022856 seconds;
- reader drain: 0.000055 seconds.

A fresh small-store barrier after a real old-container deletion made an ordinary resolve with a 100 ms wait return busy:physical-readers. It succeeded after release. Default wait is 15,000 ms. This demonstrates the mechanism; the original large run did not separately time a concurrent outage. The code retains the exclusive physical reader lease and writer guard through deletion. Simply moving a drop without preserving capture/reader/catalog/lifetime protections is not an acceptable fix.

Source: src/packing.rs, relevant actual excerpts:

```rust
// gate(): default bounded wait
let start=Instant::now();let cap=std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000);
// ... shared/exclusive try-lock loop; timeout returns busy:physical-readers ...

// run(), registration phase
// Never hold writer.lock while draining API readers (which may need that writer).
m.physical=None;io::checkpoint("pack-reader-drain")?;let drain=Instant::now();let exclusive=gate(root,true)?;m.elapsed("pack-reader-drain",drain);let registration=Instant::now();
let guard=store.lock(m)?;let mut c=config(root,m)?;let current_hash=io::digest(&pack::native(&root.join("STORE.json"),65536,m)?);
// ... validate expected physical configuration; atomically replace STORE.json ...
m.physical=Some(PhysicalRead::from_lease(root,exclusive,m)?);
journal.status="registered".into();save(root,&journal,m)?;io::checkpoint("pack-accounting")?;m.elapsed("pack-registration-inclusive",registration);
// ... under the same locks: all_mapped(); verify duplicates; delete loose records;
//     retain all B publication duplicates; delete obsolete pack/index files;
//     record Windows sharing deferrals and successful deletion bytes ...
drop(guard);drop(protection);
// physical lease stays owned by Metrics through operation completion
```

### F2. Cohort stays frozen after first packing

A separate fresh-store test confirmed that a later accepted publication remains loose and unmapped even after another build/repack. The exact-cohort policy was useful for fair P0/PZ measurements, but is not rolling compaction. Source: src/packing.rs, cohort() excerpt:

```rust
let mut keys:BTreeSet<_>=all_mapped(m)?.into_iter().collect();let frozen=!keys.is_empty();
if frozen&&!include_attempts&&!repack{return Ok(keys);}
let mut attempts=BTreeMap::new();let mut live_owners=BTreeSet::new();let head=store.snapshot(m)?;
let mut selected=store.membership_entries(m)?;selected.push((head.selected.publication.clone(),head.selected));
for (_,s) in selected {
    let p=store.publication(&s,m)?;
    // ... live owner and packet-owner protection ...
    if !frozen||keys.contains(&Key::new(&format!("publications/{}.json",p.id))?) {
        attempts.insert(p.attempt.clone(),(p.request.clone(),p.intent.clone()));
        if !frozen {
            for name in [format!("publications/{}.json",p.id),format!("results/{}.json",p.result_hash),format!("attempt-hints/{}.json",p.attempt)] {keys.insert(Key::new(&name)?);}
        }
    }
}
// ... only finalized, non-reserved accepted candidate/allocation records are eligible;
// repack drops retired attempt records, preserving permanent publication/results/hints ...
```

### F3. First lookup authenticates a whole relevant index

Physical capture currently takes a shared global pack-reader lease before reading config/catalog and keeps it in operation Metrics. locate() selects the one role/range descriptor and lazily loads that full index, with a 64 MiB cache budget including transient allocation. Index::load reads the entire index, checks its SHA against the catalog, validates pack binding, then parses every entry for typed key, ordering, contiguous offsets, lengths, first/last and coverage before binary-search lookup. Record reads independently bound/decode and hash-check raw bytes. Mapped missing/corrupt data refuses; unmapped keys alone can fall back to loose files.

Source: src/pack.rs, abbreviated relevant actual excerpts:

```rust
let b=native(&root.join(format!("packs/{}.idx",d.index)),usize::try_from(d.index_bytes)?,m)?;
if b.len()!=d.index_bytes as usize||io::digest(&b)!=d.index||b.len()<HEADER||&b[..8]!=MAGIC||hex(&b[8..40])!=d.pack||u64::from_le_bytes(b[40..48].try_into()?)!=d.pack_bytes||b[48]!=d.codec||u32::from_le_bytes(b[49..53].try_into()?)!=d.records {
    return Err("integrity:index-pack-binding".into());
}
// ... actual pack length binding ...
let mut index=Self{bytes:b,offsets:Vec::with_capacity(d.records as usize),descriptor:d.clone()};
let mut position=HEADER;let mut last=String::new();let mut end=0u64;
for _ in 0..d.records {
    index.offsets.push(position);let (e,next)=index.at(position)?;
    if e.key.0<=last||e.key.role()!=Some(d.role.as_str())||e.offset!=end||e.raw>MAX_RAW||e.encoded>MAX_RAW+65536||e.offset.checked_add(e.encoded as u64).is_none_or(|n|n>d.pack_bytes) {
        return Err("integrity:index-order-offset-length".into());
    }
    // ... raw-codec length check, advance position/end ...
}
// ... complete index/pack coverage and descriptor first/last checks ...
```

Existing bounds: at most 64 packs/catalog; 64 MiB/pack; 16 MiB/index; 65,536 records/pack; 8 MiB/eligible raw record; bounded 64 MiB index cache. Cohort maintenance currently collects all mapped keys in memory and walks membership. These are experimental limits, not an unlimited-history promise.

## Starting hypothesis for the next bounded experiment (challenge and refine)

Deliver one dependency-ordered experiment with three checkpoints, or explain a better split while retaining all three required outcomes. Preserve a baseline at the exact core pin. Use a small correctness corpus first, then separate document-count and history-depth axes; reuse authenticated long-history evidence/images read-only rather than regenerating 100k publications merely to benchmark a reader.

1. ONLINE CLEANUP: make generation/catalog capture and retirement protection explicit. Keep the semantic writer/catalog activation critical section short. Move verified loose-duplicate and obsolete-container reclamation into bounded resumable work outside global publication/read exclusion when safe. Account for old readers, new readers, pending preparations, Windows no-delete-sharing handles, cancelled/dead readers, two maintenance contenders, stale physical compare-and-swap, interrupted registration and cleanup, and restart. Explain lock order, reader acquisition/revalidation, liveness/fairness and the precise point each file can be deleted. Avoid inventing a universal daemon, distributed lease service, or plugin framework to solve a local-file protocol.

2. ROLLING TAIL: define an authenticated selected cutoff and absorb newly accepted eligible records across at least three real publish/compact/reopen cycles. Publications beyond the cutoff remain a readable loose tail or a later segment. Keep every earlier accepted outcome and intent/dedup identity. Require selected membership, never directory presence or a physical catalog, to establish eligibility/acceptance. Include crashes before/after activation and partial cleanup, retry/cancel/lost reply, two maintenance candidates, protected retired content and saved packets. Show maintenance discovery/work does not require re-reading every historical publication each cycle. Test small quotas, tail/backlog growth, temporary coexistence, bounded backpressure, and later recovery/reclamation.

3. FRESH-PROCESS LOOKUP: compare the existing relevant-index load with at most two credible smaller authenticated lookup designs, choosing the simplest that actually reduces first-operation work. Possibilities to evaluate rather than mandates: bounded independently authenticated index pages/ranges, or a simpler range-partitioned format. Do not claim mmap alone removes full authentication cost. Do not trust mtime/size, skip integrity validation, or rely on a daemon's already warm cache. Preserve honest positive and negative lookup proofs, role/range/offset bounds, mapped-corruption refusal, raw exact identities and bounded decode/cache allocation. Keep full audits explicit and distinct from touched-path guarantees. Charge index/catalog file count and startup work, including any replacement metadata or verification cache. Private format changes need explicit versions, compatibility/rejection behavior, and old-image restore tests.

Use the same logical cohort, requests/outcomes and comparable guarantees for performance comparisons. Keep codec/layout improvements separable from lifecycle changes. A large-cohort 95% file-reduction result does not make that target meaningful for a 28-record store. Suggest an activation threshold and scale-specific new criteria without rewriting the previous failure. Keep B's unavoidable loose controls and full-generation publication costs visible; redesigning B's browsing contract is outside this slice.

Suggested bounded workloads to refine: one small A/B correctness fixture, actual history around 1k and 10k, read-only compatibility/fresh-process lookup against the retained ~101k history, and at least three tail cycles. Exercise 1 and 4 publishers with several independent reader processes plus a deliberately held old reader. Count accepted requests, expected semantic conflicts, maintenance-induced busy/refusals, latency distribution and maximum pauses separately. Native Windows and Linux should run the primary correctness/availability checks; macOS must get the same pinned protocol's focused native recovery and file-lifetime checks, with no unsupported blanket platform claims. Do not multiply every history/corpus/codec/concurrency/platform combination into a giant matrix.

For acceptance propose numerical availability/lookup and resource gates, then mark which require calibration. Starting hypotheses to challenge: no history-sized work while holding semantic/global reader exclusion; zero maintenance-induced timeout/refusal at the admitted workload; measure maximum as well as p95/p99 activation hold; bound metadata bytes/pages touched per fresh lookup independently of total record count rather than demand a hardware-independent millisecond SLA. Separate total process startup from engine operation time and OS/device-cold from fresh process. Record samples, cache state, selected roots, actual durable publication count, physical byte/file counts, native allocation where available, peak RSS, temporary coexistence and successful deletion credit. Oracles must check exact accepted outcomes, membership, graph/source/navigation and old selected-once B extraction after every critical recovery phase.

Resource reality: prior combined ceiling was 40 GiB / 5,000,000 files, with 1 GiB / 100,000 files reserved inside it. The prior closeout's conservative prospective forecast was almost full (5,600,492 bytes unassigned before this small review). Do not assume a new multi-GB allocation or silently reclaim retained fixtures/sole backups. A fresh actual census and prospective admission must precede execution, including any released unused reservations, retained evidence, branch/build growth and closeout. This is planning now, not a fresh execution budget. Recommend a realistic fixed functional/closeout clock and a stop/continue decision if larger evidence cannot be admitted. Preserve protected originals; use authenticated disposable clones only when admitted, with truthful censors rather than silently extending deadlines.

## Please return

1. Keep/adjust/pivot with the immediate deliverable, intended outcome and next decision explicitly identified. Name any higher-level simplification and its practical costs.
2. Minimal viable protocol/state machine, authorities, reader/maintenance lock order and crash/recovery cases. Identify what source must be inspected before implementation because the packet does not establish it.
3. Recommend whether the three improvements belong in one staged experiment or ordered bounded tasks, and why. Provide tracer steps, narrow code areas, explicit non-goals and completion criteria, without dropping required outcomes.
4. A small discriminating matrix with exact correctness oracles, adversarial schedules, numerical or work-based acceptance gates, compatibility/native qualification, resource admission, and explicit censors. Distinguish hard correctness conditions from proposed performance targets and unknowns.
5. The two or three architecture decisions the human needs to settle before dispatch, if any; resolve routine implementation details by recommendation. Say whether an embedded library pattern would materially simplify this versus custom maintenance, without requiring a database migration or broad dependency investigation.
6. What would falsify this approach or justify an explicit maintenance-window contract or a different storage abstraction; what should be deferred until the next evidence review. Give one concise draft implementation brief we can reconcile locally before authorizing dispatch.
