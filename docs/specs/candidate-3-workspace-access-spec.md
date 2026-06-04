# Spec: Candidate 3 Workspace Access Deepening

## Assumptions

1. Candidate 2 is already landed and authoritative, so this change does not revisit trusted pipeline session ownership.
2. The change stays inside the existing Rust workspace and does not introduce a new crate.
3. The primary goal is to deepen the library seam around repo-relative workspace access, not to change CLI behavior or reduced-v1 workflow semantics.
4. The eventual `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate` consumer should be able to rely on one compiler-owned workspace module instead of learning multiple ad hoc path-validation and no-follow rules.
5. This packet can move code across existing compiler modules, but it should preserve current `.handbook/`, `core/**`, and contract-owned path semantics unless an explicit contract update says otherwise.

## Objective

Deepen the workspace access module inside `handbook-compiler` so repo-relative validation, no-follow file reads, trusted writes, and canonical `.handbook/` discovery stop leaking across multiple modules.

The current repo has already centralized route-basis trust in `load_trusted_pipeline_session`. The remaining architectural friction is elsewhere:

- `repo_file_access.rs` owns safe repo-relative reads and writes
- `canonical_artifacts.rs` duplicates no-follow reads and `.handbook/` discovery rules
- `pipeline.rs` keeps its own repo-relative path validator and stage-file/front-matter reads
- `route_state.rs` still owns additional filesystem traversal for inventory enumeration and runtime-state reset

The user is the maintainer of this repo today and the future library consumer inside `substrate`.

Success looks like one compiler-owned workspace module seam that:

- owns repo-relative path normalization and validation
- owns safe no-follow file reads and repo-relative writes
- owns canonical `.handbook/` discovery primitives
- lets pipeline, canonical artifact, and route-state modules read as adapters over that seam

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Existing contracts and repo truths that constrain this work:
  - `docs/contracts/C-01-approved-repo-surface.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
  - `docs/contracts/pipeline-route-and-state-core.md`

## Commands

Build:

```bash
cargo check --workspace
```

Targeted compiler verification:

```bash
cargo test -p handbook-compiler repo_file_access
cargo test -p handbook-compiler canonical_artifacts
cargo test -p handbook-compiler --test pipeline_loader
cargo test -p handbook-compiler --test pipeline_catalog
cargo test -p handbook-compiler --test pipeline_state_store
```

CLI fallout:

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
```

Repo verification wall:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
```

## Project Structure

```text
crates/compiler/src/repo_file_access.rs    -> Current repo-relative file validation, read, and write helpers
crates/compiler/src/canonical_artifacts.rs -> Canonical `.handbook/` discovery and artifact ingest
crates/compiler/src/pipeline.rs            -> Pipeline/stage loading and repo-relative stage path checks
crates/compiler/src/route_state.rs         -> Route-state persistence, runtime-state reset, and inventory traversal
crates/compiler/src/lib.rs                 -> Library exports for compiler consumers
crates/compiler/tests/                     -> Compiler integration coverage
crates/cli/tests/                          -> CLI regression coverage
docs/contracts/                            -> Current reduced-v1 contract authority
docs/specs/                                -> Candidate 3 spec/plan/tasks documents
```

## Code Style

Prefer one typed workspace module with narrow adapters over repeated inline filesystem ladders.

```rust
let workspace = CompilerWorkspace::new(repo_root)?;
let stage_path = workspace.stage_file(&stage.file)?;
let contents = workspace.read_text(&stage.file)?;
```

Conventions:

- keep workspace rules compiler-owned and library-first
- prefer typed operations over raw `std::fs` calls scattered through callers
- preserve current refusal and validation posture while shrinking caller knowledge
- keep normalized repo-relative paths as the shared interface, not caller-specific `PathBuf` conventions

## Testing Strategy

- Framework: existing Rust unit tests and integration tests in `crates/compiler/tests/` and `crates/cli/tests/`
- Primary test levels:
  - unit tests for workspace access invariants
  - compiler integration tests for pipeline loading, canonical artifacts, and route-state behavior
  - CLI regression tests only where public wording or help posture could drift
- Focus areas:
  - repo-relative path validation and normalization
  - symlink refusal and no-follow reads
  - canonical `.handbook/` discovery behavior
  - stage-file and front-matter loading through the deep workspace seam
  - runtime-state reset and inventory enumeration that still need filesystem traversal
- Coverage expectation:
  - every migrated caller path keeps explicit success and refusal coverage

## Boundaries

- Always:
  - preserve reduced-v1 `.handbook/`, `core/**`, and contract-owned path semantics
  - keep workspace rules compiler-owned and library-first
  - add or update targeted tests for every migrated caller
  - prefer migration onto one deep seam over adding a second helper stack
- Ask first:
  - introducing a new crate or external dependency
  - changing public CLI flags, help text, or contract semantics
  - widening scope into non-filesystem architectural cleanup
  - broadening the seam to alternate roots or virtual filesystems immediately
- Never:
  - reintroduce raw repo-relative validation ladders in migrated callers
  - loosen symlink or no-follow safety to make the refactor easier
  - couple `substrate` consumption to `handbook-cli` instead of `handbook-compiler`
  - silently change path normalization, artifact discovery, or refusal posture

## Success Criteria

- One compiler-owned workspace seam owns repo-relative validation, normalization, no-follow reads, trusted writes, and canonical `.handbook/` discovery primitives.
- `canonical_artifacts.rs`, `pipeline.rs`, and the remaining filesystem-owning parts of `route_state.rs` consume that seam instead of keeping independent path/file helpers.
- The implementation reads as a deeper library module suitable for future `substrate` consumption without forcing the consumer to learn multiple path-trust rules.
- Existing reduced-v1 contracts and public CLI behavior remain stable unless an explicit reviewed doc update says otherwise.
- Targeted compiler and CLI regression coverage passes after the migration.

## Open Questions

- Should the deep seam remain in `repo_file_access.rs`, or should it become a renamed workspace-focused module once it owns more than raw file access?
- Should directory traversal concerns such as runtime-state reset and inventory enumeration move fully under the same seam in this packet, or only the parts that directly overlap existing repo-relative rules?
- Should the first public library export for downstream consumers expose a workspace object, or should the deep seam remain internal until a real `substrate` call site needs it?
