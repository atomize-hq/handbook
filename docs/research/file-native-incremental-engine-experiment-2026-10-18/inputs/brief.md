# Proposed next experiment: incremental validation, publication, and long histories

Status: proposed experiment after an independent Pro approach review, October 18, 2026. This file does not launch an experiment or select a production architecture. The [saved consultation and local reconciliation](file-native-engine-plan-pro-consult-2026-10-18/README.md) record the review and the amendments below.

The separately authorized native Linux/macOS prerequisite has completed on the common baseline. See the [combined findings](file-native-platform-prerequisite-2026-10-18.md). Pro recommends adjusting this plan while keeping the architecture. The optimization scope remains proposed; no experimental implementation has begun.

## Evidence and constraints

Baseline commit: `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`.

- [Comparative findings](file-native-engine-followup-2026-10-17/comparison.md), [verification](file-native-engine-followup-2026-10-17/verification.md), [review](file-native-engine-followup-2026-10-17/review.md), and [protocol](file-native-engine-followup-2026-10-17/protocol.md).
- Current full validation: [model.rs](file-native-engine-followup-2026-10-17/src/model.rs). Full-state loading/diff: [backend.rs](file-native-engine-followup-2026-10-17/src/backend.rs). Linear history, selection, and view rendering: [main.rs](file-native-engine-followup-2026-10-17/src/main.rs).
- [Structured format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md) remains proposed. Preserve existing owner/versioned contracts, Contracts JSON, and exact authored source bytes.

Measured Windows 100k A metadata writes fell from 48.0 MB to 0.38 MB with COW, but metadata reads grew from 192 MB to 512 MB during complete update work. The current adapter loads the graph and compares complete before/after maps. Selection and rendering reload/revalidate state. Positive historical membership and request resolution walk a predecessor chain capped at 1,000 publications. The large-tier histories actually measured contain only three publications.

At Windows 10k A, a manifest-backed one-document commit without a new packet took 486 ms, versus 5.66 seconds with the 5,000-constituent packet. At WSL 100k B, both backends took roughly 657 seconds for a full-generation successor. These observations have uncontrolled caches; they identify work to isolate rather than establish product latency targets.

Preserve both browsing options: A has exact selected state and a mutable readable view that may mix/lag; B promises a complete retained readable generation at selection. Preserve exact packets and their required closure. Dolt remains comparator-only. Windows/NTFS and WSL2/ext4 have prior baseline evidence; the bounded native Linux/ext4 and Mac/APFS prerequisite below has now passed on the same unchanged Rust baseline. PowerShell/Bash walkthroughs are sufficient; no GUI automation gate.

GitNexus exploration was explicitly bound to this checkout. Its index remains at c7f54e6, three commits behind the baseline, with degraded query results. Source observations here come from direct inspection; no indexed coverage is inferred for the new experiment. No implementation symbol is changed by this proposal.

## Separate prerequisite: native Linux and macOS baseline

The user identified restored native Linux SSH access and a directly connected Codex Mac project. The referenced chat, Research file-based durable membership (local thread 01a12df3-39a5-7a52-a8fe-71217078691e), was read before adopting its findings. On October 18 this chat independently confirmed SSH commands against spenser-linux-codex: Linux 6.16.8-1-MANJARO and ext4 for /home/spenser. Codex project discovery now exposes Handbook on Spensers-MacBook-Pro.local at /Users/spensermcconnell/__Active_Code/atomize-hq/handbook. The Mac filesystem, architecture, and toolchain still require native inspection; project availability is not an executed test result.

This was executed as a separate bounded baseline qualification phase with two directly launched top-level chats, outside Meta. Both completed without Rust/core patches. Linux's controller setup/export corrections and the Mac's Bash/resource adaptations are recorded in their reports. The pinned baseline was published as codex/file-native-baseline-379af4e; the Mac report-only result is commit 1ad8841b82f3c378838c96621fed8f2061e0c26b on codex/file-native-macos-prerequisite-20261018 and is verified locally. The bounded assignment below records the scope that was executed; it does not dispatch a rerun. Native platform evidence and all its limits are in the combined findings above.

1. Pin the existing committed experiment at 379af4eda9b95ee82be9d3afcfdcdca2d6d132f9. Inspect remote availability before deciding whether a push is needed. The experiment is already committed; do not blanket-commit unrelated untracked research. A named Git ref can convey the exact tested revision to the Mac; preserve its existing working tree and build/test in an isolated checkout. Record source/harness/native hashes on both hosts. No Mac SSH setup is necessary when Codex executes there.
2. Record OS/kernel, architecture, toolchain, actual local filesystem and case behavior, available space, and relevant flush/name-install APIs. Use /home/spenser on native Linux ext4. Discover the Mac volume rather than presuming APFS or case sensitivity. Do not benchmark against network or Windows-mounted storage as if it were local ext4/APFS.
3. Build the landed standalone crate and execute its small correctness and affected-boundary regressions for manifest/COW and A/B, with both consumer vocabularies represented. Cover immutable no-replace installation and short-write retry, nested names, selected-versus-orphan membership, same-request dedup/different-intent refusal, cancellation, lost reply beneath successors, graph conflicts, and independent stores.
4. Exercise phase-specific process interruption around COW root preparation, publication selection, A refresh, and original-outcome delivery. Exercise retained readers, retirement/reclamation, interrupted recovery, and preserved damaged-view edits in disposable fixtures. State actual native synchronization calls and any unsupported operation. Process death is not OS-crash or power-loss qualification.
5. Run Bash directory/text/copy/hash discovery and selected-once copy checks using only files and included instructions. Cover existing Unicode/case/reserved-title/deep-attachment fixtures. No Finder or Linux GUI automation is required.
6. Run a modest performance smoke allocation: 1k documents, representative 1/8/64-document updates and fresh-process current/retained reads; at most one paired A/B 10k closure sample if preliminary costs fit the declared cap. Record counters and engine/process times without claiming cross-machine OS rankings. Do not regenerate the full 100k matrix or attempt long histories in this prerequisite.
7. Freeze finite per-case and total runtime/data budgets before execution, reserve reporting time, and preserve failed/censored observations. A normal build/correctness pass should not expand into a toolchain port or broad platform installation. Diagnose and repair a bounded experimental portability defect if needed; freeze a new common revision and rerun affected Windows/WSL checks before treating different host results as one implementation.

Deliver a compact per-host build/correctness/recovery/resource matrix, actual API behavior, pins, reproducible commands, raw evidence, and any required portability fixes. The prerequisite establishes a known platform baseline, not complete production certification. A functional failure needs resolution or an explicit design decision before optimizing that path; unfamiliar timing alone is not a correctness failure. Keep results from each host distinct, and continue useful work on one host if the other is unavailable.

Feed the completed baseline and recorded harness adaptations into the Pro approach review after human review of the prerequisite result. Then use the optimization experiment for its intended algorithmic questions, with affected regression reruns on these hosts rather than a repeated full platform-discovery effort. No optimization implementation or Pro consultation was authorized by the prerequisite closeout alone.

## Main question and bounds

Can a small COW core make ordinary small transactions depend on their actual changes, affected constraints, and bounded index paths, while preserving publication/outcome/retention guarantees across long histories?

Reuse the landed COW tree and protocol. Keep the full validator, full renderer, and linear-history implementation as bounded reference oracles. The next experiment is not another backend tournament or a production migration. Do not add prolly, distributed consensus, automatic disjoint merge, a general query language, or a mandatory daemon.

Implement changes in separately measurable steps: compact internal encoding; selected-root-bound constraint indexes and delta updates; bounded publication/history lookup; incremental view rendering. Measure each step before combining them. Otherwise a faster representation can hide a more expensive validation algorithm, or vice versa.

## 1. Incremental validation with a complete oracle

Start from a fully validated base. Persist these experimental indexes/summaries under the same selected root as the document map:

| Rule | Bounded evidence to consult/update |
| --- | --- |
| Document identity and kind-specific local validity | Changed documents and the selected validator version |
| Portable path uniqueness | Normalized path-to-stable-ID index; old and new keys |
| Label uniqueness | Label-to-stable-ID index; old and new keys |
| Local target existence and deletion safety | Target lookups and reverse-reference index, including in-batch changes |
| Edge-kind/target rules | Changed edges, relevant source/target kinds, declared rule dependencies |
| Aggregate budget | Checked delta against a selected exact aggregate; overflow/underflow refusal |

Apply the complete batch logically before checking its affected constraints. Test removals, swaps, two documents moving into one namespace, delete-target/add-edge races, budget write skew, rename/edge races, and high-degree nodes. Do not limit testing to the prior contiguous IDs or overview-changing update pattern.

Keep exact expected-publication conflict detection and per-store coordination initially. Separate valid stale candidates must not combine without explicit revalidation. Automatic read-set OCC or merging is not needed to learn whether incremental validation works.

The indexes are committed semantic evidence, not independently authoritative caches. Missing/corrupt required evidence is an integrity error; a rebuild is explicit and checks against source state. A root hash alone does not prove index correctness. Define initialization, validator-version changes, and rebuild behavior.

Use seeded differential tests against complete validation on small/medium graphs, comparing acceptance/refusal and resulting state. Run independent complete audits after larger sequences and recovery. Unsupported opaque/global domain rules must retain an explicit full-validation fallback; extensible kinds do not make every arbitrary rule incrementally decidable.

State the integrity boundary clearly: incremental validation preserves declared invariants from a checked base; it does not reread every unrelated stored page on every commit. Full corruption audits remain distinct operations. Do not present this change as retaining a per-commit whole-store corruption scan.

## 2. Delta-native metadata and internal encoding

Apply operations to point/range reads and a bounded overlay; carry the changed keys and dependency deltas forward rather than constructing and comparing two whole maps. Coalesce COW path updates. Keep large adjacency data independently addressable so one edge edit does not necessarily rewrite a 5,000-edge document value.

Compare the current JSON numeric-byte-array page representation with one compact, versioned, length-bounded representation. Keep exact source YAML/JSON/Markdown untouched. Include checksums, unsupported-version refusal, malformed/truncated pages, indirect-object damage, and bounds before allocating. Use fresh experimental files; this does not authorize migrating old evidence or production formats.

Measure encoding alone before attributing gains to incremental validation. Report payload versus metadata reads/writes, touched pages, index updates, CPU, sync calls, and actual retained storage.

## 3. Publication without repeated full-state work

For A, use a retained change description and path/navigation indexes to update affected files and affected navigation sections. The normal path should not enumerate and rewrite the whole collection for an unrelated edit. Keep names and navigation useful through ordinary terminal tools. Compare the resulting view to a complete-render oracle.

Resume must work across skipped refreshes, multiple selected successors, and death during removal/rename/navigation replacement. Preserve divergent external bytes before repair. An incremental change description is not proof that unrelated view files remain unmodified; retain explicit view audit/rebuild behavior.

Prepare state, constraint indexes, and any required publication evidence together. Selection binds the prepared roots and exact expected publication. Repeated full validation at preparation, gate, and render should be replaced only by a stated trust/verification contract, with malformed or stale prepared candidates refused. Do not make a mutable preparation seal sufficient authority.

Packet assembly remains an explicit workload. Cache/reuse exact dependency descriptions only when bound to selected roots and validator/recipe versions; changes to required transitive closure must invalidate the affected description. Materialized exported packets retain their declared destination verification.

B remains an honest full-generation control. Complete independent ordinary files require namespace/materialization work; the COW metadata tree alone cannot remove it. Measure B on a small number of large publications. On-demand exports or narrower snapshot scopes may be separately labeled workloads, but do not silently redefine B. Avoid hardlinking mutable readable files into authoritative originals or other retained generations. Platform copy acceleration is optional only if it preserves the same independence and verification guarantees.

## 4. Indexed positive history and exact outcomes

Replace mandatory predecessor-chain traversal with selected-root-bound publication membership and request/outcome indexes, using checkpoints only with an explicit authority and recovery model. A plain receipt or an orphan publication file is insufficient. Keep hash dependencies acyclic when an accepted result identifies its publication.

Test 1k, 10k, and a bounded 100k publication-history tier on a small corpus. At least 10k must be actual durable selected publications to exceed the old ceiling materially. Clearly distinguish genuine metadata/content changes, equal-content publications, and any synthetic or bulk-built structural stress fixture. A bulk-generated chain is not evidence for 100k successful live commits.

At each depth, reopen in a fresh process; query current and retained publications; resolve early/middle/recent requests; test never-selected complete candidates, missing requests, repeated same-ID/same-intent, different-intent reuse, cancellation, restored old content under a new publication, retired snapshots, and lost replies beneath many successors.

Time appends, positive/negative resolution, historical membership, open/recovery, checkpoint creation, audits, and reclamation. Record publication/outcome/index bytes separately from document history. Checkpoint installation, interrupted checkpointing, missing/corrupt indexes, and fallback/rebuild behavior need fault cases.

Keep exact accepted outcomes unless a separate explicit horizon is selected. Retiring content does not silently erase accepted results or prove a request never published. Long-history tests must not materialize 100k full B generations or a 5,000-source packet on every commit; test those materialization contracts separately.

## 5. Retention, quotas, and failure behavior

Carry document, validation-index, history-index, checkpoint, pending-candidate, packet, and retained-reader roots into reclamation. Exercise explicit retirement and recovery while old requests remain resolvable. Use disposable owned roots for deletions; preserve baseline runs.

Enforce bounded candidate count/bytes over the declared categories, including stale sealed work, rather than equating an eight-worker limit with an artifact quota. Test refusal/backpressure, cancellation while waiting, killed lock holders, duplicate retries, and recovery from interrupted quota accounting. Do not infer death from TTL or erase still-eligible candidate protection.

Use 1/2/4/8-process contention at a representative medium size, including scattered edits and slow/high-degree requests. Retain whole-publication conflict detection initially. Report complete per-request latency, all attempts, lock wait/hold, discarded work, pending bytes, and refusal outcomes. No automatic replay of external effects.

Keep checkpoint-specific process-fault assertions at every new root/selection/history boundary: before selection must not select; visible incomplete boundary is qualified uncertain; after the required boundary the original accepted outcome must survive beneath successors. Recovered success must not erase a censored original observation.

## 6. A small standalone library boundary

Extract only enough library structure for two small consumers to use the same store/snapshot/transaction core and typed publication outcomes. Keep the CLI as a harness. Two concrete kind adapters can declare local validation and supported invariant dependencies; readable projection, packet assembly, and export are explicit capabilities around that core. Avoid a broad plugin ABI or promising arbitrary global predicates are cheap.

Include owned-store identification, format/version limits, explicit source/import validation routes, and exact-byte preservation. Document the distinction between operating without the engine to read useful files and importing changes into authority. Keep Handbook owners and root workspace dependencies untouched.

Produce a durability contract distinguishing published authority, readable-view completion, process-interruption recovery, and stronger OS/power-loss persistence. Windows directory-sync limitations and the WSL host/VHDX stack remain explicit. The prerequisite adds bounded native Linux/macOS build and process-level evidence. Broader native qualification, actual reboot/power-loss qualification, hostile namespace/root-lock identity hardening, and production migration remain separately bounded work; do not advertise production readiness from this experiment alone.

## Workload and success criteria

Use Windows/NTFS and Ubuntu WSL2/ext4 for the established measured comparison, with data/builds on ext4 rather than /mnt/c. Carry representative changed-protocol and portability regressions onto the native Linux and Mac hosts qualified by the prerequisite; do not multiply every large performance case across every host by default. Preserve source/native/harness pins and raw bytes. PowerShell/Bash discovery and extraction checks remain required; GUI automation is unnecessary.

- Document-growth cases: 1k/10k/100k documents with fixed short history, ordinary low-degree edits and an explicit 5,000-degree case; 1/8/64 scattered changes where practical.
- History-growth cases: fixed small corpus with 1k/10k/100k history depths, bounded real mutation mixes, no automatic packet generation. Set finite resource/time caps before runs; a censored stress case is not a completed benchmark.
- Do not multiply every document size, history depth, browsing contract, consumer, packet policy, and process count. Use one declared representative matrix and reuse a newly owned immutable corpus safely where possible; preserve old evidence.
- Small ordinary edits should visit work proportional to affected constraints plus index paths, not load every document. History point operations should avoid reading every preceding publication. Verify these with read/page/namespace counters in addition to wall time.
- Large/high-degree edits, audits, explicit context closure, and B full-generation copying may legitimately scale with their required data. Report those separately rather than promising all commits are constant-time.
- Use matched operation and integrity scopes, quiet serial benchmark lanes where feasible, repeated small homogeneous observations, and individual sparse stress timings. Fresh-process is not device-cold. State actual throughput/latency improvements and regressions without inventing a product SLA.

Deliver a focused implementation/report/evidence directory and a decision table for validation, metadata, history, rendering, lifecycle, and the library boundary. A full validator/renderer/history oracle mismatch, lost accepted result, orphan-based membership, incorrect index lifetime, or hidden weakened guarantee blocks the proposed optimization. No production adoption or successor dispatch follows automatically.

## Reconciled Pro amendments and execution gates

The [complete answer](file-native-engine-plan-pro-consult-2026-10-18/answer.md) and [local reconciliation](file-native-engine-plan-pro-consult-2026-10-18/reconciliation.md) preserve the advice and source checks. Carry these explicit requirements into the sections above:

1. A checked base includes exact logical constraint-index projections and aggregates. Compare missing/extra index entries, edge multiplicity and source bindings, as well as documents and acceptance decisions. Preserve baseline lowercase-path and unchanged-label comparisons. New domain rules/operations are explicit versioned extensions. Exercise one full-rule fallback.
2. Separate one edge edit in a 5,000-edge adjacency from a target change affecting 5,000 incoming references. Count physical/tombstone keys. Keep codec measurements honest about changed packing/occupancy; use one checked decoder/reference enumerator for point/full/audit/GC routes and enforce bounds before allocation.
3. Use a directly represented current head plus predecessor-only membership/outcome indexes as the initial experimental history shape. A successor inserts only its already-selected predecessor. This avoids self-referential hashes. Bind original result objects independently of document retention, and keep large result bodies out of unrelated point lookups. Define checkpointing as a verified index rebuild/repack selected through an ordinary expected-head successor. Explicit repair/reference scans may be linear; normal reopen and resolution may not.
4. Give in-process preparation an opaque validated capability. Reopened candidates undergo bounded pure transition replay from captured inputs and the exact checked base, deriving affected work rather than trusting caller-supplied lists or a seal. Compare roots/versions/evidence, measure replay, preserve early dedup, and retain B's destination verification before selection.
5. Persist a target-bound A refresh plan before changing files. Every path possibly touched by unfinished work remains recoverable, including a partial write later reversed by a successor. Start with one coordinated refresher completing its pending plan before advancing. Retain required roots and stable divergence-preservation identities. Use indexed change ranges or structural projection-map diff for skipped revisions. Derived sections need dependencies beyond content/path deltas. Preserve full audit/rebuild; distinguish fast pending refresh if exposed separately.
6. Type retention edges so permanent accepted evidence does not permanently retain all retired document content. Protect pending checkpoint/refresh/candidate roots. Reserve candidate count/bytes before allocating, transfer protection/charge without gaps, fence abandonment, and release only after deletion or explicit transfer. Recover pessimistically. A released lock is not abandonment. Replace or bound the growing abandoned-attempt registry on normal admission paths. Keep total-run storage caps and free-space reserves separate from candidate quotas.
7. Freeze live payload sizes, active key space, degree and mutation/retention/refresh policy for history runs. Use bounded byte variants rather than unbounded appended whitespace. Label maintenance/equal-content/content/metadata/restoration selections separately. A resident harness can execute real per-publication synchronization; fresh-process checkpoint reads remain required. Provide a test-only full walker or independent outcome ledger beyond the old 1,000 cap. At least 10k actual completed selections remain mandatory; bounded/synthetic/censored 100k results stay distinct.
8. Define the private library boundary early, extract after the protocol stabilizes and before the final matrix, and demonstrate two independently compiled consumers. B readiness participates before core selection even though readable projection is an explicit capability. No public API/format freeze or general plugin ABI follows.

Use these stages and gates without multiplying the large matrix:

| Stage | Work | Gate |
| --- | --- | --- |
| 0 | Freeze rule/dependency, selected-index coverage, lifetime, candidate/checkpoint/refresh/quota transitions and finite workload/resource tables | Every authority transition has a named finite verification case |
| 1 | Compact encoding with existing adapter/validation/rendering | Checked decoding/reachability agree; malformed cases and isolated measurements pass |
| 2 | Delta-native updates, constraint indexes and adjacency | Zero independent document/index/acceptance discrepancies; counters show affected work |
| 3 | Indexed history, checkpointing, new-root retention and quotas | Outcome/orphan/interruption tests pass before long-history allocation |
| 4 | Durable A pending work and shared library consumers | Full-render, divergence, B/packet extraction and shared-core checks pass |
| 5 | Integrated document/history growth, contention, reclamation and affected native regressions | Required observations/audits complete, with original failures and censors preserved |

Measure small/medium matched traces at each stage. Use expensive integrated tiers and selected controls only after correctness gates. An individual optimization can be rejected without discarding the other experiment questions. These are experiment amendments, not a production architecture decision.

## Pro consultation timing and questions

The independent approach review is complete. Its verdict is to adjust the bounded plan and keep the existing architecture. The review received the common baseline, completed native findings, qualified Windows/WSL results, this proposal, clarified A/B/Dolt preferences and exact bounded source excerpts. Its questions and original timing rationale follow for provenance; another pre-experiment consultation is unnecessary unless a material design conflict emerges.

This checkpoint was placed before committing to new validation/history roots or a durable format. It used an approach review rather than a comprehensive repository audit or patch exchange, in a fresh conversation.

Ask Pro for keep/adjust/pivot guidance on:

1. Incremental invariant preservation from a checked base; write skew, reverse edges, aggregate summaries, and domain rules requiring fallback.
2. One selected authority binding document/constraint/history roots; acyclic identity and exact-outcome recovery, including interrupted checkpoints.
3. Long-history membership/outcome proof without linear replay or accidental pruning of accepted evidence.
4. Whether the workload separates metadata, audit, packet, namespace, and materialization costs fairly while preserving both browsing options.
5. A minimal library boundary and the concrete evidence still needed before production adoption.

The consultation must preserve the requested task, including the instruction: "Do not reduce the task or replace it with an easier alternative." Its advice is advisory and must be reconciled with local truth and the user's preferences.

A second useful external checkpoint is after this experiment demonstrates the proposed mechanisms, before freezing a public API/on-disk format or migrating a Handbook consumer. That can be a semantically bounded review of the changed validation/publication/history implementation. Routine intermediate tuning does not require repeated Pro rounds.
