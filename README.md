# Handbook

Rust-first planning workspace and CLI for the reduced-v1 `handbook` product.

The supported path is the Rust workspace in `crates/`. The older Python harness still ships in this repo, but only as frozen reference material while the cutover finishes.

## Current Status:

- The governing repo-surface truth is [C-01 Approved Repo Surface](docs/contracts/C-01-approved-repo-surface.md).
- The command-surface truth is [C-02 Rust Workspace and CLI Command Surface](docs/contracts/C-02-rust-workspace-and-cli-command-surface.md).
- The interaction contract lives in [DESIGN.md](DESIGN.md).
- The current top-level CLI surface is `setup`, `author`, `pipeline`, `generate`, `inspect`, and `doctor`.
- The public setup family is `handbook setup`, `handbook setup init`, and `handbook setup refresh`.
- Bare `handbook setup` routes to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it routes to `setup refresh`.
- `setup` remains the durable product term. `init` is only the concrete first-run subcommand name.
- `setup refresh` preserves canonical files by default. `setup refresh --rewrite` rewrites only setup-owned starter files, and `setup refresh --reset-state` resets only `.handbook/state/**`.
- The canonical setup-created starter files are exactly `.handbook/charter/CHARTER.md`, `.handbook/project_context/PROJECT_CONTEXT.md`, and `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`.
- The shipped starter templates are scaffolding only. Setup establishes the baseline file set; `doctor` classifies baseline readiness as `SCAFFOLDED`, `PARTIAL_BASELINE`, `INVALID_BASELINE`, or `BASELINE_COMPLETE`.
- `FEATURE_SPEC.md` remains outside baseline setup and baseline `doctor`. The shipped feature-spec path stays packet-driven: `pipeline compile stage.10_feature_spec` emits model input payload, an external model or operator produces the completed markdown, and `pipeline capture stage.10_feature_spec` materializes it.
- Setup hands off to `handbook doctor`, which renders the ordered baseline checklist and the next exact authoring command.
- The repo-owned charter authoring method lives at `core/library/authoring/charter_authoring_method.md`.
- This repository does not ship completed canonical `.handbook/` truth at repo root. On a fresh clone, start with `handbook setup`; only after replacing starter text with real canonical truth does `handbook doctor` become the ready-path next step.
- The public baseline authoring family is `handbook author charter`, `handbook author project-context`, and `handbook author environment-inventory`.
- The automation-safe structured-input authoring paths are `handbook author charter --from-inputs <path|->` and `handbook author project-context --from-inputs <path|->`.
- `handbook author charter --validate --from-inputs <path|->` is the mutation-free charter preflight surface. `--validate` is not supported on guided `handbook author charter`.
- `handbook author charter --from-inputs <path|->` is deterministic and compiler-owned. The guided `handbook author charter` path remains the Codex-backed interview surface.
- `handbook doctor --json` is the only machine-readable readiness surface for the installed `handbook-charter-intake` skill.
- Codex packaging authored skill inputs live under `install/handbook-home/`. Repo-local `.agents/skills/*` trees are thin generated projections only. The installed home is `~/handbook/`, installed thin projections live under `~/handbook/.agents/skills/*`, and `~/.codex/skills/handbook*` is discovery glue only.
- `tools/codex/install.sh` owns the curated installed `~/handbook/` home: `~/handbook/bin/handbook` is the only installed executable, `~/handbook/runtime-manifest.json` remains part of the runtime contract, and installed static guidance lives under `~/handbook/resources/**`.
- Tagged GitHub Releases now publish curated `~/handbook/` bundles for `macOS arm64` and `Linux x86_64`, together with `SHA256SUMS`.
- `scripts/handbook/install.sh` is the thin public installer wrapper. It downloads the correct tagged release bundle, verifies the checksum, installs `~/handbook/`, and refreshes `~/.codex/skills/handbook*` without requiring a preinstalled `handbook` binary.
- `tools/codex/dev-setup.sh` is the dev-only symlink path. Normal install is copy-based, and re-running normal install after dev setup replaces those symlinks with copied directories cleanly.
- `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and the shipped command family `list`, `show`, `resolve`, `compile`, `capture`, `handoff emit`, and `state set`.
- Approved docs and contracts teach one declarative namespace root under `core/**`, including `core/pipelines/`, `core/profiles/`, and `core/runners/`.
- Any retained references to top-level `pipelines/`, `profiles/`, `runners/`, or repo-root `pipeline.yaml` are historical-only wording and are not the approved declarative surface.
- Planning packet generation reads canonical repo-local `.handbook/` inputs.
- `execution.demo.packet` is fixture-backed demo only. Live execution is explicitly refused.
- Stage `10` stays truthful: `pipeline compile` emits payload-only model input, external model output produces the completed `FEATURE_SPEC.md`, and `pipeline capture` materializes that body.
- `pipeline handoff emit --id <pipeline-id> --consumer <consumer-id>` is the shipped downstream handoff-emission wedge for the named consumer flow.

Historical reference only:

- Earlier M4/M5 docs used the phrase "`setup` is still a placeholder". Treat that as superseded wording, not active product authority.

## Start Here

- Supported docs index: [docs/README.md](docs/README.md)
- Supported product entrypoint: [docs/START_HERE.md](docs/START_HERE.md)
- Exact command surface: [docs/SUPPORTED_COMMANDS.md](docs/SUPPORTED_COMMANDS.md)
- CLI vocabulary: [docs/CLI_PRODUCT_VOCABULARY.md](docs/CLI_PRODUCT_VOCABULARY.md)
- Command hierarchy and front door: [docs/CLI_COMMAND_HIERARCHY.md](docs/CLI_COMMAND_HIERARCHY.md)
- Output anatomy: [docs/CLI_OUTPUT_ANATOMY.md](docs/CLI_OUTPUT_ANATOMY.md)
- Operator journey and conformance notes: [docs/CLI_OPERATOR_JOURNEY.md](docs/CLI_OPERATOR_JOURNEY.md)

## Repo Entry Points

- Release notes: [CHANGELOG.md](CHANGELOG.md)
- Active implementation plan: [PLAN.md](PLAN.md)
- Broader long-range vision: [docs/VISION.md](docs/VISION.md)
- Current backlog: [TODOS.md](TODOS.md)
- Reduced-v1 seam pack: [artifacts/planning/reduced-v1-seam-pack/README.md](artifacts/planning/reduced-v1-seam-pack/README.md)
- Legacy docs index: [docs/legacy/README.md](docs/legacy/README.md)

## Useful Commands In This Repo

Inspect the shipped command surface:

```bash
cargo run -p handbook-cli -- --help
cargo run -p handbook-cli -- pipeline --help
```

Inspect the current pipeline inventory:

```bash
cargo run -p handbook-cli -- pipeline list
cargo run -p handbook-cli -- pipeline show --id pipeline.foundation_inputs
```

See the M6 setup family and recovery handoff:

```bash
cargo run -p handbook-cli -- setup
cargo run -p handbook-cli -- setup init
cargo run -p handbook-cli -- setup refresh
cargo run -p handbook-cli -- doctor
cargo run -p handbook-cli -- doctor --json
```

Exercise the shipped deterministic charter surfaces:

```bash
cargo run -p handbook-cli -- author charter --validate --from-inputs tools/fixtures/charter_inputs/runtime_smoke_valid.yaml
cargo run -p handbook-cli -- author charter --from-inputs tools/fixtures/charter_inputs/runtime_smoke_valid.yaml
```

Generate and install the Codex packaging layer:

```bash
bash tools/codex/generate.sh
bash tools/codex/install.sh
bash tools/codex/dev-setup.sh
```

Install the latest tagged public release without building locally:

```bash
curl -fsSL https://raw.githubusercontent.com/atomize-hq/handbook/main/scripts/handbook/install.sh | bash
```

Exercise the packet surfaces once canonical `.handbook/` artifacts exist:

```bash
cargo run -p handbook-cli -- generate
cargo run -p handbook-cli -- inspect
```

## Reduced-v1 Boundaries

- `pipeline compile --id <pipeline-id> --stage <stage-id>` is the bounded M2 compile surface.
- Plain `pipeline compile` success is payload-only stdout.
- `pipeline compile --explain` is the compile proof surface for that same result.
- `pipeline capture --id <pipeline-id> --stage <stage-id>` is the bounded M3/M3.5 writer surface.
- `pipeline capture --preview` validates stdin, caches one typed materialization plan, and returns a deterministic `capture_id`.
- `pipeline capture apply --capture-id <capture-id>` revalidates freshness and applies the cached plan transactionally.
- For `pipeline.foundation_inputs`, the shipped capture stages are `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, and `stage.10_feature_spec`.
- `pipeline capture` remains the only supported stage-output writer surface.
- Stage `10` materialization stays `compile -> external model output -> capture`.
- Compile and capture freshness recovery are explicit. Re-run `pipeline resolve` before retrying when route basis is missing, stale, or inactive.
- Capture apply safety is scoped to `handbook`-coordinated single-writer flows.
- `inspect` is the packet proof surface.
- `doctor` is the recovery surface.

## Repo Layout

- `crates/cli/`: CLI binary and command-surface tests
- `crates/compiler/`: narrow compatibility/support seam for CLI-facing authoring, setup/doctor, rendering, and template-library glue that still spans owner crates; direct callers should import `crates/engine/`, `crates/flow/`, and `crates/pipeline/` for extracted logic instead of routing through this crate
- `crates/engine/`: canonical artifact, freshness, and structured authoring core
- `crates/flow/`: resolver, packet-result, and budget runtime surfaces
- `crates/pipeline/`: declarative pipeline loading, route state, compile/capture, and handoff runtime surfaces
- `core/pipelines/`: approved declarative pipeline definitions
- `core/profiles/`: approved declarative profile packs and profile command surfaces
- `core/runners/`: approved declarative runner allowlist and execution-guidance modules
- `core/stages/`: approved stage source documents used by the compiler
- `core/library/`: reusable authoring directives, templates, and shared planning inputs
- `core/schemas/`: structured YAML contracts for generated artifacts
- `docs/`: supported docs, contracts, and frozen legacy docs
- `tests/fixtures/foundation_flow_demo/`: committed proof corpus for the `pipeline.foundation_inputs` happy/skip journey
- `artifacts/planning/reduced-v1-seam-pack/`: reviewed reduced-v1 planning pack
- `tools/`: legacy harness helpers plus current QA helpers

## Legacy Reference

The Python harness remains in the repo to preserve prior scaffold behavior and generated-artifact examples. It is not the supported product path.

- Legacy docs index: [docs/legacy/README.md](docs/legacy/README.md)
- Harness mechanics: [docs/legacy/HARNESS.md](docs/legacy/HARNESS.md)
- Legacy system model: [docs/legacy/SYSTEM_MODEL.md](docs/legacy/SYSTEM_MODEL.md)
