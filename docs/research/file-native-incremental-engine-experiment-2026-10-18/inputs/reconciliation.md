# Local reconciliation of Pro's approach review

Date: October 18, 2026. Baseline: `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`. This is a plan review before experimental implementation, not an implementation audit or a production decision.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Disposition

Accept **adjust the plan and keep the architecture**. Keep the local disk-oriented COW tree, one selected authority, exact expected-publication conflicts, both browsing contracts, exact packets/sources, and full reference oracles. The native Linux/macOS prerequisite is complete. No new prerequisite run, backend tournament, production Dolt adoption, migration, public API freeze, or experiment dispatch follows from this consultation.

The advice mostly makes proposed obligations executable. Several additions are substantial enough to include explicitly in the brief before the experiment starts.

## Evidence checked locally

| Pro observation | Local evidence and consequence |
| --- | --- |
| Whole-state work remains | `backend.rs:33` compares complete maps; `backend.rs:57` enumerates the tree; `main.rs:25`, `main.rs:140`, `main.rs:188`, and `main.rs:231` load/validate state across preparation, selection and rendering. Supports the proposed delta/validation experiment; does not establish its performance. |
| Rule comparisons and edge multiplicity matter | `model.rs:52` lowercases paths, compares labels without that conversion, and permits duplicate `Vec<Edge>` entries. Indexing must preserve those semantics. New kind rules/edge operations must be identified as bounded versioned extensions, not retroactively attributed to the baseline. |
| Hash-cycle risk has a simple experimental solution | `main.rs:243` allocates the publication ID before constructing its result. Pro's predecessor-only membership/outcome indexes plus a directly represented head preserve acyclic hashes. This is a proposed experimental shape, not landed behavior or a frozen production format. |
| Replacing request lookup alone leaves linear work | `main.rs:30` has the 1,000-publication cap. Historical access, request resolution, prior-view lookup and lifecycle abandonment call the walker. `inspect` and full GC may remain explicit full operations; ordinary paths must not hide them. |
| Codec migration affects GC and allocation | `backend.rs:75` manually parses JSON pages for reachability. `io.rs:49` reads a complete file before `disk.rs:124` applies the object-size check. The new format needs one checked decoder/reference traversal and bounded reads before allocation. |
| Navigation has independent costs | `main.rs:61` renders one whole navigation document, including edge target paths. The candidate's `changed` list tracks path/content changes and is insufficient for all derived dependencies. Use stable readable sections or report monolithic rewrite cost. |
| A repair and fast resume are distinct | `main.rs:188` inspects actual files during rebuilding; `run` sets the rebuild flag for `resume`. A new fast pending-plan operation cannot silently replace the full repair behavior. The partial-write/reversal example is an optimization hazard, not a newly proven baseline failure. |
| Admission has another growing registry | `lifecycle.rs:8` keeps all abandoned attempts in `RETENTION.json`, parsed by `check_candidate`. Point-address or bound that normal-path evidence alongside the history optimization. |
| Outcome lifetime differs from content lifetime | The collector retains publication/result/fence records and explicitly reclaims retired state/closure data. New generic graph traversal must not accidentally keep all historical documents forever or collect accepted results. |

These checks were direct source inspection, including the navigation/closure bodies omitted from the uploaded excerpts. No Rust symbol was edited. GitNexus coverage remains stale/unknown for this experiment; these observations are not an indexed impact or independent full-audit verdict.

## Amendments to carry into the experiment

1. **Checked indexes, not only accepted documents.** Compare every logical index projection, missing/extra reverse references, multiplicity, aggregates and source bindings against an independent recomputation. Preserve final-batch semantics and existing comparison rules. Exercise one explicit unsupported-global-rule fallback. Separate a single edit inside a large adjacency from a target operation that genuinely affects thousands of references.
2. **One decoding/reference model.** Keep the encoding stage isolated. Record occupancy, height and page changes because byte-based splitting can change packing. Update point reads, full reads, audits and GC together; enforce limits before file/encoded-length allocation. Exact authored bytes stay unchanged.
3. **Acyclic indexed history.** Start with head-direct/predecessor-index coverage. Each successor inserts only its positively selected predecessor into immutable membership/outcome maps. Bind exact publication digests and immutable original results; keep large result bodies out of unrelated lookups. Missing required evidence is an integrity error, never evidence of absence. Define checkpoints as verified index rebuild/repack successors under the same selector and expected-head check. Full repair can explicitly walk authenticated history; normal open/recovery does not.
4. **Verifiable preparation.** Use an opaque in-process prepared capability; reopened candidates replay their pure captured transition against the exact checked base, deriving affected work from the request. Match roots, versions and evidence. Measure repeated incremental replay. Preserve dedup before stale/candidate failures and retain B destination verification at selection.
5. **Durable A refresh work.** Persist a target-bound plan before changing files and retain every touched path until reconciled. Include write-then-revert interruption, skipped successors, rename/swap destinations and derived dependencies. Start with one coordinated refresher finishing its pending plan before the next target. Protect required roots and divergence backups; use stable preservation identities. Keep explicit full audit/rebuild behavior. B and packet guarantees stay intact.
6. **Typed lifetimes and reservations.** Distinguish permanent accepted evidence from retireable document/readable content. Protect readers, candidates, checkpoints and pending refreshes. Reserve candidate slots/bytes before allocation, maintain charge through sealing/selection/abandonment/deletion, and recover pessimistically. Releasing an OS lock does not abandon a candidate. Duplicate charges may be conservative; report actual unique bytes separately. Quotas do not bound permanent history or replace total-run caps. Accepted duplicates must resolve even when new admission is full.
7. **Unconfounded long histories.** Freeze live payload size, active keys, degree, mutation/retention/refresh policy. Use bounded byte variants instead of endless whitespace appends. Label content, metadata-only, equal-content, restoration and maintenance publications. Require at least 10k actual completed per-publication selection/sync boundaries in one store; a resident harness is acceptable. Fresh-process reopening/resolution remains required at depth checkpoints. Extend the test-only reference walker or use an independent ledger beyond 1,000. Synthetic 100k and censored live 100k remain distinct. Count tombstones/physical keys and all control/result bytes.
8. **Small library with B preselection readiness.** Define the private boundary early and extract it after protocol paths stabilize, before the integrated matrix. Two independently compiled small consumers use the same publication/resolution/lifecycle core. Projection/assembly/export are capabilities, but B's required complete generation participates before selection. No general plugin framework or public format/API freeze.

## Staging and unchanged limits

Freeze protocol/semantic/lifetime/resource tables first; measure encoding alone; implement and compare delta validation; implement indexed history with retention/quotas; implement A pending work and the thin library; then run the integrated bounded matrix. Keep small/medium matched traces at each stage, and reserve expensive full-tier runs for declared controls and the integrated candidate. An individual optimization can fail its performance test without cancelling the other questions.

Windows/WSL remain the established measurement lanes. Native Linux/Mac need affected protocol faults and representative measurements on the same new source pin. Preserve Bash 3.2, safe extraction destinations, explicit memory/timer units, portable evidence transport, original failures/censors, and actual source/session/binary/raw-byte binding. No full cross-product matrix or new GUI gate.

Do not choose A over B, infer power-loss persistence, require a production Dolt variant, weaken exact outcomes, or replace real 10k durable publications with synthetic fixtures. Pro explicitly preserves those boundaries. Candidate/header/index format details and concrete caps are still the experiment's Stage 0 work; this advisory review does not certify their correctness.

The useful next Pro checkpoint is after the experiment, as a bounded review of the changed validation/publication/history/lifecycle implementation before public API/disk-format freezing or production migration.
