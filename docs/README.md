# Handbook Docs

## Start Here (Supported)

Reduced v1 is a **Rust-first multi-crate CLI workspace**.

- Repo overview: [`README.md`](../README.md)
- Start here: [`docs/START_HERE.md`](START_HERE.md)
- Command surface: [`docs/SUPPORTED_COMMANDS.md`](SUPPORTED_COMMANDS.md)
- CLI interaction contract: [`DESIGN.md`](../DESIGN.md)
- CLI product vocabulary: [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md)
- CLI command hierarchy and front door: [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md)
- Baseline authoring commands: `handbook author charter`, `handbook author project-context`, `handbook author environment-inventory`
- Ownership posture: `handbook-engine`, `handbook-flow`, and `handbook-pipeline` are the direct owner crates for extracted logic; `handbook-compiler` remains a narrow compatibility/support seam for residual CLI-facing glue that still spans those owners
- Reviewed orchestration surface: `pipeline` for route resolution, explicit stage compilation, explicit stage-output capture, and narrow route-state mutation
- First shipped compile wedge: `pipeline compile --id <pipeline-id> --stage <stage-id>` with payload-only stdout, plus `pipeline compile --explain` for proof-only stdout
- First shipped writer wedge: `pipeline capture --id <pipeline-id> --stage <stage-id>` plus `pipeline capture --preview` and `pipeline capture apply --capture-id <capture-id>`
- Shipped downstream adoption wedge: `pipeline handoff emit --id <pipeline-id> --consumer <consumer-id>`
- Packet proof remains `inspect`; compile proof lives under `pipeline compile --explain`
- Capture preview/apply semantics live under [`docs/contracts/pipeline-capture-preview-and-apply.md`](contracts/pipeline-capture-preview-and-apply.md)
- CLI tone rules: [`docs/CLI_TONE_RULES.md`](CLI_TONE_RULES.md)
- CLI output anatomy: [`docs/CLI_OUTPUT_ANATOMY.md`](CLI_OUTPUT_ANATOMY.md)
- CLI operator journey and conformance review: [`docs/CLI_OPERATOR_JOURNEY.md`](CLI_OPERATOR_JOURNEY.md)
- Implementation plan: [`PLAN.md`](../PLAN.md)
- Release notes: [`CHANGELOG.md`](../CHANGELOG.md)
- Current backlog: [`TODOS.md`](../TODOS.md)
- Repo surface contract: [`C-01`](contracts/C-01-approved-repo-surface.md)
- CLI command surface: [`C-02`](contracts/C-02-rust-workspace-and-cli-command-surface.md)
- Canonical `.handbook/` manifest + freshness: [`C-03`](contracts/C-03-canonical-artifact-manifest-contract.md)
- Doctor baseline readiness + blockers: [`C-04`](contracts/C-04-resolver-result-and-doctor-blockers.md)
- Renderer and proof surfaces: [`C-05`](contracts/C-05-renderer-and-proof-surfaces.md)
- Fixture-backed execution demo boundary: [`C-06`](contracts/C-06-fixture-execution-demo-boundary.md)
- Conformance rails and docs cutover: [`C-07`](contracts/C-07-conformance-rails-and-docs-cutover.md)
- Pipeline operator surface and ID resolution: [`C-09`](contracts/pipeline-operator-surface-and-id-resolution.md)
- Vision (broader, non-binding): [`docs/VISION.md`](VISION.md)
- Glossary for legacy harness terms still referenced in frozen docs: [`docs/GLOSSARY.md`](GLOSSARY.md)

## Legacy (Reference Only)

The Python harness and its stage-based workflow remain in the repo as frozen reference material until cutover.

Start from the legacy index when you need the old scaffold behavior:

- [`docs/legacy/`](legacy/README.md)
- Harness mechanics: [`docs/legacy/HARNESS.md`](legacy/HARNESS.md)
- Legacy system model: [`docs/legacy/SYSTEM_MODEL.md`](legacy/SYSTEM_MODEL.md)
- Legacy stage reference: [`docs/legacy/stages/README.md`](legacy/stages/README.md)
- Legacy workflow guides: [`docs/legacy/guides/README.md`](legacy/guides/README.md)
