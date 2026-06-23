# Tasks: Handbook Published-Import Decoupling — Set 1: Pipeline Boundary Authority Reconciliation

Plan reference: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md)

Spec reference: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md)

---

## Packet 1.1: Current-State Evidence Matrix

- [x] Task: Record a side-by-side claim matrix for the MAP, audit, Packet 4.2 proof, archived parameterization docs, archived published-boundary docs, and live crate source
  - Acceptance: The active Set 1 docs contain an explicit matrix that shows, for each source, the specific claim it makes about the published boundary, how that claim aligns or conflicts with `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`, and whether live code/published behavior validates, narrows, or invalidates that claim.
  - Verify: Source inspection of `docs/specs/MAP.md`, `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`, `docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`, `crates/pipeline/src/lib.rs`, `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/layout.rs`, and the relevant archived docs; `rg -n "frozen subset|public/import-facing|declarative_roots|layout|Packet 4.2|Expose capabilities, not guts" docs/specs/MAP.md docs/specs/archive/ HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`, optionally `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` if a completion note is required

- [x] Task: Reproduce the external-consumer proofs that distinguish the proven `engine + flow` seam from the still-private `pipeline` seam
  - Acceptance: Set 1 records one successful compile against crates.io `handbook-engine = "=0.1.1"` + `handbook-flow = "=0.1.1"` and one failing compile against crates.io `handbook-pipeline = "=0.1.1"` importing `handbook_pipeline::layout::PipelineStorageLayoutContract`, with the failure classified as current truth rather than as a transient environment issue.
  - Verify: Run the temp-crate `cargo check` commands from the spec exactly; capture the success for `engine + flow` and the `E0603: module layout is private` failure for `pipeline`.
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`

### Packet 1.1 completion notes

- Status: complete on 2026-06-23
- Evidence matrix: recorded in the active Set 1 spec and summarized in the active Set 1 plan
- Positive proof:
  - PASS — crates.io scratch consumer compiled with `handbook-engine = "=0.1.1"` + `handbook-flow = "=0.1.1"`
  - Confirmed imports: `handbook_engine::CanonicalLayoutContract` and `handbook_flow::{resolve_with_contract, ResolveRequest}`
- Negative proof:
  - FAIL AS EXPECTED — crates.io scratch consumer depending on `handbook-pipeline = "=0.1.1"` failed to import `handbook_pipeline::layout::PipelineStorageLayoutContract`
  - Failure classification: current published-boundary truth
  - Failure text: `error[E0603]: module 'layout' is private`
- Packet 1.1 conclusion:
  - `engine + flow` are proven published-consumer seams
  - `handbook-pipeline` declarative-root and storage-layout control are still private at crates.io `0.1.1`
  - Packet 4.2 remains valid only as a narrow `engine + flow` downstream proof, not as pipeline-adoption proof

---

## Packet 1.2: Boundary-Shape Decision And Set 2 Target

- [x] Task: Lock the requirement that Set 2 must deliver the full reusable `handbook-pipeline` capability Substrate needs through a reviewed published boundary
  - Acceptance: The active Set 1 authority explicitly states that this capability is a requirement from `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`, not an optional future branch, and explains why Packet 4.2 does not yet satisfy it.
  - Verify: Human review of the Set 1 triplet after the Packet 1.1 matrix is complete; `rg -n "full reusable|minimum reviewed public surface|Packet 4.2|Set 2|handbook-pipeline" docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`

- [x] Task: Define the exact Set 2 acceptance wall for the minimal public capability boundary
  - Acceptance: Set 1 leaves behind exact Set 2 proof requirements, including required source surfaces, required external-consumer proof, required downstream Substrate revalidation inputs, and the minimum guard rails needed to prevent another false-complete state while preserving the MAP rule of minimum unnecessary public surface.
  - Verify: Source inspection of the Set 1 triplet; `rg -n "Set 2|external-consumer|guard rail|Substrate|pipeline|minim|Expose capabilities, not guts" docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`

### Packet 1.2 completion notes

- Status: complete on 2026-06-23
- Decision: Set 2 will expose a narrower public façade, not wholesale direct promotion of the private `declarative_roots` / `layout` modules.
- Requirement lock:
  - full reusable `handbook-pipeline` capability for Substrate remains mandatory active authority work
  - Packet 4.2 remains valid only as narrow published `engine + flow` proof
- Bounded Set 2 candidate proof surface:
  - public declarative-roots contract surface, with candidate first-wave public proof bounded to `handbook_pipeline::pipeline::SupportedTargetRegistry::load`, `handbook_pipeline::pipeline::load_pipeline_catalog`, `handbook_pipeline::pipeline::load_pipeline_catalog_metadata`, `handbook_pipeline::pipeline::load_pipeline_selection_metadata`, `handbook_pipeline::pipeline::load_pipeline_definition`, and `handbook_pipeline::pipeline::load_selected_pipeline_definition` for declarative-root control plus stage-root-aware catalog/loading behavior
  - public storage-layout contract surface, with candidate first-wave public proof bounded to `handbook_pipeline::route_state::{load_route_state, set_route_state, load_trusted_pipeline_session, persist_route_basis}`, `handbook_pipeline::pipeline_capture::{preview_pipeline_capture, capture_pipeline_output, apply_pipeline_capture, load_pipeline_capture_cache_entry}`, and `handbook_pipeline::pipeline_handoff::{emit_pipeline_handoff_bundle, validate_pipeline_handoff_bundle}` for route-state, capture, and handoff storage-layout control
  - contract-aware entrypoints on that candidate existing public pipeline surface
  - only the typed results/errors downstream must actually handle
- Keep private by default:
  - `RepoLayoutRoot`
  - nested storage-layout helper structs
  - handbook-product default helpers/constants unless later external-consumer proof names a concrete need, plus repo/file/path plumbing
  - product-shell wording and CLI/product-only behavior
- Set 2 acceptance wall:
  - published `handbook-pipeline` consumer can construct non-default contracts through public APIs only
  - published consumer must either justify each retained candidate `handbook_pipeline::pipeline::*`, `handbook_pipeline::route_state::*`, `handbook_pipeline::pipeline_capture::*`, and `handbook_pipeline::pipeline_handoff::*` path against declarative-root control, stage-root-aware catalog/loading, route-state control, capture control, or handoff control for the intended consumer shape, or shrink that candidate surface explicitly
  - Set 2 may not widen beyond that bounded candidate proof surface without reopening Set 1 authority
  - Packet 4.2 stays classified as `engine + flow` proof only
  - downstream Substrate adoption remains a later dedicated-worktree proof, not something Packet 1.2 or Set 2 may overclaim

---

## Packet 1.3: Active Authority Reconciliation

- [ ] Task: Create active superseding authority without rewriting archived history
  - Acceptance: A fresh session can use `docs/specs/MAP.md` plus the active Set 1 triplet as the main authority for this seam, while the archive remains provenance only. If the root audit needs an explicit supersession note, that note must point at the MAP and active Set 1 triplet rather than trying to retroactively clean every archived claim.
  - Verify: Source inspection of `docs/specs/MAP.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md`; optional source inspection of `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`; `rg -n "docs/specs/MAP.md|supersede|superseded|archive|provenance" docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`, optionally `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`

- [ ] Task: Bound the `9b83` Substrate notes as stale but useful non-authority context
  - Acceptance: The active Set 1 docs preserve the useful long-term provider-boundary takeaways from `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/` while clearly labeling them as non-canonical and subordinate to live `system` repo truth.
  - Verify: `rg -n "non-canonical|stale|9b83|provider boundary|context" docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`

---

## Packet 1.4: Final Set Proof And Handoff

- [ ] Task: Run the Set 1 docs-only proof wall and record the completion state honestly
  - Acceptance: The final Set 1 notes confirm:
    - `docs/specs/MAP.md` remains the governing objective/intent authority,
    - live code still matches the boundary claims recorded in Set 1,
    - the positive and negative external-consumer proofs were rerun,
    - no Rust source changed in this set,
    - and Set 2 plus Set 3 now have stable start points.
  - Verify: `cargo check --workspace`; `git diff -- docs/specs/MAP.md docs/specs HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`; source inspection of the final Set 1 triplet
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`, optionally `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`

- [ ] Task: Record the explicit Set 2 and Set 3 stop boundaries
  - Acceptance: The Set 1 completion notes name what Set 2 and Set 3 may do and what they must not silently widen into, especially:
    - no CLI/compiler/product-shell redesign,
    - no claiming Packet 4.2 proved pipeline adoption,
    - no treating internal parameterization as public-boundary proof without external-consumer verification,
    - no overexposing internals when a narrower façade can provide the same capability.
  - Verify: `rg -n "Stop Boundary|Set 2|Set 3|must not|Packet 4.2|narrower façade|minimum unnecessary public surface" docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`, `docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`

---

## Set-Level Guardrail

Stop after Set 1 produces a reconciled active authority, a concrete Set 2 proof wall, and a concrete Set 3 proof/guard-rail wall. Do not:

- edit `crates/engine/**`, `crates/flow/**`, or `crates/pipeline/**`
- widen any published crate boundary in this set
- rewrite archived docs as if they were never written
- treat full reusable pipeline capability for Substrate as optional
- widen into CLI/compiler/product-shell redesign
- start downstream Substrate implementation work

---

## Packet 1.4 completion notes

- Status: pending
- MAP alignment status:
  - pending
- Evidence matrix status:
  - pending
- External-consumer proof status:
  - pending
- Boundary-shape decision status:
  - pending
- Active-authority reconciliation status:
  - pending
- Set 2 handoff status:
  - pending
- Set 3 proof/guard-rail handoff status:
  - pending
