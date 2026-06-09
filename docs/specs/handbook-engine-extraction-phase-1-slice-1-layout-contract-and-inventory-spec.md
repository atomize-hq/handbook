# Spec: Handbook Engine Extraction Phase 1 Slice 1 - Layout Contract And Inventory

## Assumptions

1. This slice is a **contract-and-inventory** slice, not a caller-migration slice.
2. The root extraction plan remains authoritative for phase order, and `docs/specs/handbook-engine-extraction-slice-map.md` remains authoritative for the Phase -> Slice -> Packet breakdown.
3. The locked decision in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` stands: use **separate layout types**, not one global layout object.
4. Slice 1.1 may land docs and behavior-neutral contract scaffolding, but it must not widen into the adoption work reserved for Slices 1.2, 1.3, or 1.4.
5. For this slice, the required inventory corpus is the reusable-internal `.handbook/**` and `.handbook/state/**` references in the compiler sources named in the slice map; CLI/product-shell references are auxiliary context and must be called out explicitly if excluded.

## Objective

Freeze the Phase 1 layout contract and produce the reusable-internal storage inventory that later Phase 1 slices will adopt.

The user is the maintainer of the handbook workspace and the future reviewer of the extraction sequence. The immediate outcome is not a functional migration. The immediate outcome is a durable slice-local authority set that answers:

- which layout ownership domains exist
- which files currently own `.handbook/**` and `.handbook/state/**` assumptions
- which later slice is responsible for adopting each assumption behind the approved layout contract
- which references are intentionally excluded because they are CLI/product-shell-only or otherwise outside Slice 1.1

Success means future migration slices do not have to rediscover the path/storage shape while editing production code.

## Slice Scope

In scope:

- freeze the layout type-family contract for Phase 1
- freeze the reusable-internal inventory for the targeted compiler files
- classify references by ownership domain
- mark temporary exceptions, exclusions, and indirect dependencies explicitly
- hand off a clear adoption map to Slices 1.2, 1.3, and 1.4

Out of scope:

- changing production path behavior
- moving callers to the new layout contract
- rewriting CLI/product wording
- parameterizing orchestration targets or template resolution
- Phase 4 crate-boundary work or Phase 6 Substrate migration planning

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Current compiler sources named by Slice 1.1:
  - `crates/compiler/src/canonical_artifacts.rs`
  - `crates/compiler/src/route_state.rs`
  - `crates/compiler/src/pipeline_capture.rs`
  - `crates/compiler/src/pipeline_handoff.rs`
  - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
  - `crates/compiler/src/setup.rs`
  - `crates/compiler/src/author/charter.rs`
  - `crates/compiler/src/author/project_context.rs`
  - `crates/compiler/src/author/environment_inventory.rs`
- Auxiliary exclusion/context corpus:
  - `crates/cli/src/main.rs`

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Repo-local canonical storage under `.handbook/**`
- Repo-local runtime state under `.handbook/state/**`
- Additional artifact outputs such as `artifacts/**` that must still fit the future layout type family even when they are not part of the slice verifier corpus

## Commands

Slice 1.1 inventory query:

```bash
rg -n "\.handbook|\.handbook/state" crates/compiler/src crates/cli/src
```

Targeted compiler-only inventory query:

```bash
rg -n "\.handbook|\.handbook/state" \
  crates/compiler/src/canonical_artifacts.rs \
  crates/compiler/src/route_state.rs \
  crates/compiler/src/pipeline_capture.rs \
  crates/compiler/src/pipeline_handoff.rs \
  crates/compiler/src/stage_10_feature_spec_provenance.rs \
  crates/compiler/src/setup.rs \
  crates/compiler/src/author/charter.rs \
  crates/compiler/src/author/project_context.rs \
  crates/compiler/src/author/environment_inventory.rs
```

Behavior-neutral compile rail if contract scaffolding is added later:

```bash
cargo check --workspace
```

Repo verification wall if a later implementation packet introduces code:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace
```

## Project Structure

```text
HANDBOOK_ENGINE_EXTRACTION_PLAN.md                                              -> Root phase-order authority
 docs/specs/handbook-engine-extraction-slice-map.md                              -> Phase -> Slice -> Packet authority
 docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-*.md
                                                                                -> Slice 1.1 authority set
 crates/compiler/src/canonical_artifacts.rs                                      -> Canonical `.handbook/` root and artifact namespace ownership
 crates/compiler/src/setup.rs                                                    -> Canonical root establishment and runtime-state reset behavior
 crates/compiler/src/route_state.rs                                              -> Runtime-state path ownership and reset/inventory traversal
 crates/compiler/src/pipeline_capture.rs                                         -> Capture-cache path ownership under `.handbook/state/**`
 crates/compiler/src/stage_10_feature_spec_provenance.rs                         -> Stage-capture provenance path ownership
 crates/compiler/src/pipeline_handoff.rs                                         -> Future handoff-bundle layout consumer / indirect dependency in this slice
 crates/compiler/src/author/charter.rs                                           -> Canonical charter path and authoring lock ownership
 crates/compiler/src/author/project_context.rs                                   -> Canonical project-context path and authoring lock ownership
 crates/compiler/src/author/environment_inventory.rs                             -> Canonical environment-inventory path and authoring lock ownership
 crates/cli/src/main.rs                                                          -> Product-shell `.handbook` references; excluded from Slice 1.1 acceptance except for classification
```

## Code Style

Prefer separate ownership types anchored to one repo root over scattered string literals or one oversized global layout object.

```rust
let repo = RepoLayoutRoot::new(repo_root)?;
let canonical = CanonicalLayout::new(&repo);
let runtime_state = RuntimeStateLayout::new(&repo);
let authoring = AuthoringLayout::new(&repo);
```

Conventions for this slice:

- one layout type per ownership domain
- one domain owner per path family
- later callers consume typed accessors; Slice 1.1 does not migrate them yet
- path literals remain allowed only inside the approved owner until later slices complete adoption
- inventory rows must distinguish reusable-internal ownership from product-shell wording

## Testing Strategy

Primary verification for this slice is **corpus completeness and contract consistency**, not runtime behavior change.

Test levels:

- document review against the root plan and slice map
- `rg`-based inventory verification against the targeted compiler corpus
- explicit exclusion review for CLI/product-shell references
- `cargo check --workspace` only if a later Slice 1.1 implementation packet adds behavior-neutral code scaffolding

Coverage expectation:

- every targeted compiler file named in the slice map appears in the inventory freeze below
- every direct `.handbook/**` or `.handbook/state/**` ownership assumption in those files is classified by owner and follow-on slice
- excluded references are called out explicitly, not silently omitted

## Boundaries

- Always:
  - preserve the locked decision to use separate layout types rather than one global layout object
  - keep Slice 1.1 bounded to contract definition and inventory freeze
  - classify references by owner and by follow-on adoption slice
  - mark product-shell-only, indirect, or no-hit files explicitly
- Ask first:
  - changing production path semantics
  - adding new crates or dependencies
  - widening into Slice 1.2, 1.3, or 1.4 adoption work
  - changing public CLI help text or operator wording
- Never:
  - collapse the future layout family into one monolithic layout object
  - start migrating callers in Slice 1.1
  - silently omit inventory hits or exclusions
  - treat reusable-internal storage ownership and product-shell wording as the same class of reference

## Success Criteria

- Slice 1.1 defines the approved layout ownership domains for Phase 1.
- Slice 1.1 explicitly rejects a single global layout object and freezes a **layout type family** approach.
- The targeted compiler corpus has a durable inventory table with one row per file and explicit ownership classification.
- Each inventoried assumption names the follow-on adoption slice: 1.2, 1.3, or 1.4.
- CLI/product-shell references are explicitly marked as exclusions or auxiliary context.
- No caller migration is required for Slice 1.1 completion.
- The slice produces exactly two implementation packets:
  - `Packet 1.1.1: Layout Type Family And Ownership Boundary`
  - `Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze`

## Open Questions

- Should the future layout contract live in one `layout.rs` module, a `layout/` module tree, or remain docs-only until Slice 1.2 starts adoption?
- Should the future handoff bundle layout owner be introduced during Phase 1 adoption work even though the Slice 1.1 verifier corpus is primarily `.handbook/**` and `.handbook/state/**`?
- Should CLI fixture-path references eventually receive their own product-shell-only inventory appendix, or is explicit exclusion in this slice sufficient?

## Layout Ownership Domains Frozen By Slice 1.1

The exact Rust type names may still change during implementation, but the ownership domains are fixed by this slice.

Packet 1.1.1 is the Ownership domain freeze for Slice 1.1: it keeps the approved separate layout types boundary, permits no caller migration in Slice 1.1, hands Canonical root layout adoption to Slice 1.2, hands Runtime state layout / Capture provenance layout / Handoff bundle layout adoption to Slice 1.3, and hands Authoring layout adoption to Slice 1.4.

| Ownership domain | What it owns | Primary current sources | Expected follow-on adoption slice |
| --- | --- | --- | --- |
| Canonical root layout | canonical `.handbook` root and canonical artifact namespace roots | `canonical_artifacts.rs`, `setup.rs` | Slice 1.2 |
| Runtime state layout | `.handbook/state/**` root and runtime-state subtree ownership | `setup.rs`, `route_state.rs` | Slice 1.3 |
| Capture provenance layout | stage-capture and capture-cache storage under runtime state | `pipeline_capture.rs`, `stage_10_feature_spec_provenance.rs` | Slice 1.3 |
| Handoff bundle layout | handoff bundle roots and future bundle ownership | `pipeline_handoff.rs` | Slice 1.3 |
| Authoring layout | canonical authoring paths and authoring lock paths | `author/charter.rs`, `author/project_context.rs`, `author/environment_inventory.rs` | Slice 1.4 |

## Slice 1.1 Inventory Freeze

### Reusable-internal compiler corpus

| File | Current assumption shape | Ownership domain | Classification | Follow-on slice | Notes |
| --- | --- | --- | --- | --- | --- |
| `crates/compiler/src/canonical_artifacts.rs` | hardcoded `.handbook` root plus canonical artifact relative paths such as `.handbook/charter/CHARTER.md` and namespace dirs | Canonical root layout | direct reusable-internal owner | Slice 1.2 | This file is the current canonical-root authority and must become a layout consumer later without losing artifact identity semantics. |
| `crates/compiler/src/setup.rs` | direct `repo_root.join(".handbook")` root establishment and runtime-state reset language around `.handbook/state/**` | Canonical root layout + Runtime state layout | direct reusable-internal owner | Slice 1.2 for canonical root; Slice 1.3 for runtime state | Setup owns establishment and repair today; later slices should separate owner from setup-specific mutation flow. |
| `crates/compiler/src/route_state.rs` | `.handbook/state` root, runtime-state inventory/reset traversal, and runtime-state file expectations | Runtime state layout | direct reusable-internal owner | Slice 1.3 | Keep route-state domain logic local later; only storage ownership should move behind the layout contract. |
| `crates/compiler/src/pipeline_capture.rs` | capture-cache repo-relative path `.handbook/state/pipeline/capture/{capture_id}.yaml` | Capture provenance layout | direct reusable-internal owner | Slice 1.3 | Capture cache belongs to the runtime-state subtree but should have a narrower owner than generic route state. |
| `crates/compiler/src/stage_10_feature_spec_provenance.rs` | stage-capture provenance path `.handbook/state/pipeline/stage_capture/...json` | Capture provenance layout | direct reusable-internal owner | Slice 1.3 | This is the cleanest dedicated provenance-path owner in the current compiler corpus. |
| `crates/compiler/src/pipeline_handoff.rs` | no direct `.handbook/**` literal in the current slice verifier corpus; indirect dependency on capture provenance and bundle-root ownership | Handoff bundle layout | indirect dependency / no direct hit | Slice 1.3 | Included so the slice records that handoff ownership exists even though the current `.handbook` inventory query does not hit this file directly. |
| `crates/compiler/src/author/charter.rs` | canonical authoring target `.handbook/charter/CHARTER.md`, default exception location, and `.handbook/state/authoring/charter.lock` | Authoring layout | direct reusable-internal owner | Slice 1.4 | Includes both storage ownership and user-facing path mentions; later adoption should move only the owner rules first. |
| `crates/compiler/src/author/project_context.rs` | canonical authoring target `.handbook/project_context/PROJECT_CONTEXT.md` and `.handbook/state/authoring/project_context.lock` | Authoring layout | direct reusable-internal owner | Slice 1.4 | Same owner class as charter, but project-context-specific wording remains local. |
| `crates/compiler/src/author/environment_inventory.rs` | canonical authoring target `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`, `.handbook/state/authoring/environment_inventory.lock`, and references to upstream canonical authoring files | Authoring layout | direct reusable-internal owner | Slice 1.4 | This file also references other canonical authoring paths in prompts and validations; storage ownership still belongs to the authoring layout owner. |

### Explicit exclusions and auxiliary context

| File | Reason it is not part of Slice 1.1 reusable-internal acceptance | Classification |
| --- | --- | --- |
| `crates/cli/src/main.rs` | contains many `.handbook/**` path mentions, fixture paths, and operator wording, but those are product-shell concerns rather than reusable-internal layout ownership | explicit product-shell exclusion |

### Temporary exceptions frozen by this slice

- User-facing and prompt-facing `.handbook/**` string mentions inside the authoring modules remain allowed in Slice 1.1 as long as the storage owner is classified correctly.
- `pipeline_handoff.rs` remains a recorded indirect dependency even though the current verifier query does not produce a direct `.handbook` hit there.
- Non-`.handbook` layout owners such as `artifacts/handoff/**` are part of the future type family but are not the primary verifier corpus for Packet 1.1.2.

## Packet Breakdown

### Packet 1.1.1: Layout Type Family And Ownership Boundary

Goal:

- freeze the ownership domains and no-migration boundary for Slice 1.1

Required outcome:

- the slice authority set names the layout family and the adoption handoff for later slices
- the separate layout types contract freezes the Canonical root layout, Runtime state layout, Capture provenance layout, Handoff bundle layout, and Authoring layout Ownership domain boundaries without caller migration

### Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze

Goal:

- capture the full reusable-internal `.handbook/**` / `.handbook/state/**` inventory for the targeted compiler corpus and mark exclusions explicitly

Required outcome:

- later slices can adopt the approved contract without rediscovering ownership or missing deferred references
