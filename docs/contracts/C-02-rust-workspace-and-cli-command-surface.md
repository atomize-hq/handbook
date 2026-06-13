---
contract_id: C-02
seam_id: SEAM-2
owner_seam: SEAM-2
version: reduced-v1-m8
currentness: current
status: published
revalidation_triggers:
  - Any change to the supported verb set or help hierarchy in README.md, PLAN.md, docs/README.md, or this contract.
  - Any change to the workspace member list, crate ownership boundaries, or the location of CLI entrypoints.
  - Any change to the reduced-v1 local install target matrix or the supported-vs-legacy runtime boundary.
---

# C-02 Rust Workspace and CLI Command-Surface Contract

## Purpose

This contract defines the reduced-v1 Rust workspace and CLI command-surface truth for `SEAM-2`. It is the source of truth for crate ownership, supported verbs, help posture, and the runtime boundary between supported Rust code and legacy reference material.

## Canonical Location

- Canonical artifact: `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- Repo-facing truth surfaces that should point here: `README.md`, `PLAN.md`, and `docs/README.md`
- Downstream seams that consume this contract: `SEAM-4`, `SEAM-5`, `SEAM-6`, and `SEAM-7`

## Owned Surface

- Workspace root:
  - `Cargo.toml` at the repo root is the authoritative Rust workspace entrypoint.
  - The workspace members are `crates/cli`, `crates/compiler`, `crates/engine`, `crates/flow`, and `crates/pipeline`.
- Crate ownership:
  - `crates/cli` owns binary entrypoints, argument parsing, command dispatch, and user-facing help text.
  - `crates/engine` owns canonical-artifact, freshness, and structured authoring core.
  - `crates/flow` owns resolver, packet-result, and budget runtime surfaces.
  - `crates/pipeline` owns declarative pipeline loading, route state, compile/capture, and handoff runtime surfaces.
  - `crates/compiler` is a narrow compatibility/support crate for the remaining CLI-facing seams that still span owner crates, including setup/doctor orchestration, rendering/refusal/blocker adapters, and template-library support.
  - Direct callers MUST treat `crates/engine`, `crates/flow`, and `crates/pipeline` as the default owner surfaces for extracted logic rather than routing that logic through `crates/compiler`.
  - `crates/cli` MUST NOT become the home for resolver logic, packet selection, or shared domain types beyond thin wiring.
  - `crates/compiler` MUST NOT parse CLI arguments or own the supported help surface.

## Normative Rules

### Workspace and crate boundaries

- The workspace MUST expose one obvious split between the operator-facing CLI and the compiler core.
- The CLI crate MUST remain a thin orchestration layer that delegates logic to the crate that actually owns it.
- `crates/engine`, `crates/flow`, and `crates/pipeline` MUST remain the default import surfaces for the logic they own.
- `crates/compiler` MUST remain a narrow compatibility/support seam and MUST NOT revert to being an umbrella re-export crate for engine-, flow-, or pipeline-owned logic.
- Direct callers MUST keep using the direct owner crates for extracted logic and MUST use `crates/compiler` only for the retained CLI-facing compatibility/support seam.
- The compiler crate MUST remain the compile-time home for the small shared support types and adapters that still bind the CLI-facing seams together.
- For the first supported `M2` compile wedge, the CLI crate MUST stay thin and the retained compiler support seam MUST own compile-proof rendering and refusal adaptation without spreading that behavior across a new abstraction stack.
- For the first supported `M2` compile wedge, plain `pipeline compile` and `pipeline compile --explain` MUST still render from one shared typed compile result rather than maintaining separate assembly paths.
- For the first supported `M2` compile wedge, legacy Python compile behavior MAY be used as content reference, but the supported Rust payload shape, refusal wording, and proof wording MUST follow current Rust contracts rather than byte-for-byte legacy formatting.
- The Rust CLI MUST be the only supported packet-resolution authority once Rust setup exists.

### CLI verbs and help posture

- The reviewed reduced-v1 command surface consists of `setup`, `author`, `pipeline`, `generate`, `inspect`, and `doctor`.
- When the `pipeline` family lands, code, help text, docs, contracts, tests, and proof-corpus gates MUST land together before `pipeline` is treated as a supported surface.
- Help text MUST present the surfaces in setup-first order: `setup`, then `author`, then `pipeline`, then `generate`, then `inspect`, then `doctor`.
- Help text and docs MUST make clear that the public setup family is `handbook setup`, `handbook setup init`, and `handbook setup refresh`.
- Help text and docs MUST make clear that the public authoring family is `handbook author`, and that the shipped baseline authoring commands are `handbook author charter`, `handbook author project-context`, and `handbook author environment-inventory`.
- `setup` MUST remain the durable product term, and `init` MUST remain only the concrete first-run subcommand name.
- Bare `handbook setup` MUST route to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it MUST route to `setup refresh`.
- `setup refresh` MUST preserve canonical files by default.
- `setup refresh --rewrite` MUST rewrite only the setup-owned starter files:
  - `.handbook/charter/CHARTER.md`
  - `.handbook/project_context/PROJECT_CONTEXT.md`
  - `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
- `setup refresh --reset-state` MUST reset only `.handbook/state/**`.
- The shipped setup starter templates MUST be treated as scaffolding only. Setup establishes the baseline starter set and `doctor` owns baseline readiness.
- `doctor` MUST classify baseline readiness using exactly:
  - `SCAFFOLDED`
  - `PARTIAL_BASELINE`
  - `INVALID_BASELINE`
  - `BASELINE_COMPLETE`
- `doctor` checklist lines MUST include the artifact label, canonical path, status, and exact author command.
- `FEATURE_SPEC.md` MUST remain off the setup/bootstrap path and off baseline doctor readiness. It remains a packet-stage artifact.
- Setup-family success flows MUST hand off to `handbook doctor`.
- `author` MUST remain a thin CLI surface over compiler-owned authoring semantics.
- The shipped baseline authoring surface MUST expose exactly:
  - `handbook author charter`
  - `handbook author charter --validate --from-inputs <path|->`
  - `handbook author charter --from-inputs <path|->`
  - `handbook author project-context`
  - `handbook author project-context --from-inputs <path|->`
  - `handbook author environment-inventory`
- `handbook author charter` MUST be the human-guided charter-authoring surface.
- `handbook author charter --validate --from-inputs <path|->` MUST be the mutation-free charter preflight surface.
- `handbook author charter --validate` MUST be legal only when `--from-inputs <path|->` is also present.
- `handbook author charter --from-inputs <path|->` MUST be the agent and automation charter-authoring surface.
- `handbook author charter --from-inputs <path|->` MUST remain deterministic and compiler-owned.
- The public authoring wedge MUST write canonical charter truth only to `.handbook/charter/CHARTER.md`.
- `handbook author project-context` MUST be the guided project-context authoring surface.
- `handbook author project-context --from-inputs <path|->` MUST be the agent and automation project-context authoring surface.
- `handbook author project-context` MUST write canonical project-context truth only to `.handbook/project_context/PROJECT_CONTEXT.md`.
- `handbook author environment-inventory` MUST write canonical environment-inventory truth only to `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`.
- The public authoring wedge MUST refuse instead of overwriting existing non-starter charter truth.
- Help text MUST make clear that `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations, `generate` is the packet surface, `inspect` is the packet proof surface, and `doctor` is the recovery and baseline-readiness surface.
- Help text and command-surface copy MUST match the actual shipped boundary without underclaiming or overclaiming support.
- `pipeline` MUST own route resolution, explicit stage compilation, and narrow pipeline-run state mutation for the supported wedge.
- The shipped M2 compile wedge MUST expose exactly `handbook pipeline compile --id <pipeline-id> --stage <stage-id>` and `handbook pipeline compile --id <pipeline-id> --stage <stage-id> --explain`.
- Successful `pipeline compile` output MUST remain a payload surface, not a proof surface. Route-basis evidence, freshness detail, and decision proof remain the responsibility of refusal output and `pipeline compile --explain`.
- Compile-specific proof MUST be exposed through `pipeline compile --explain`, not by broadening `inspect` beyond its packet-proof meaning.
- The shipped M3 writer wedge MUST expose exactly:
  - `handbook pipeline capture --id <pipeline-id> --stage <stage-id>`
  - `handbook pipeline capture --id <pipeline-id> --stage <stage-id> --preview`
  - `handbook pipeline capture apply --capture-id <capture-id>`
- `pipeline capture` MUST remain the only supported stage-output writer surface in M3 / M3.5.
- The shipped `pipeline.foundation_inputs` capture target set for that writer wedge is:
  - `stage.04_charter_inputs`
  - `stage.05_charter_synthesize`
  - `stage.06_project_context_interview`
  - `stage.07_foundation_pack`
  - `stage.10_feature_spec`
- `pipeline compile` MUST remain payload-only stdout for the shipped M2 / M3.5 wedge.
- The shipped M4 stage-10 materialization path is:
  - `pipeline compile ... --stage stage.10_feature_spec` emits model input payload
  - `pipeline capture ... --stage stage.10_feature_spec` refuses raw `pipeline compile` payload as `invalid_capture_input`
  - an external operator or model runner produces the completed `FEATURE_SPEC.md`
  - `pipeline capture ... --stage stage.10_feature_spec` materializes that completed body
- The supported `foundation_inputs` operator sequence MUST keep `needs_project_context` as one explicit manual handoff:
  - capture `stage.05_charter_synthesize`
  - `handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`
  - `handbook pipeline resolve --id pipeline.foundation_inputs`
- `generate` MUST be the supported reduced-v1 packet-generation surface for canonical repo-local `.handbook/` inputs.
- `inspect` MUST be the supported proof surface for packet composition and decision evidence.
- `doctor` MUST be the supported recovery and baseline-readiness surface for blockers, checklist rendering, and next safe actions.
- `handbook doctor --json` MUST be the only machine-readable readiness surface for the installed charter-intake skill.
- Docs and help text MUST identify `install/handbook-home/` as the authored source of install-home skill content.
- Docs and help text MUST identify `~/handbook/` as the installed home for the Codex-facing install surface.
- Docs and help text MUST identify `~/handbook/bin/handbook` as the only installed executable for the Codex-facing install surface.
- Docs and help text MUST identify `~/handbook/runtime-manifest.json` as part of the installed runtime contract.
- Docs and help text MUST identify `~/handbook/resources/**` as the installed static guidance root.
- Repo-local `.agents/skills/*` trees MUST be described as thin generated projections only, never as the runtime payload root.
- Installed thin projections MUST be described as living under `~/handbook/.agents/skills/*`.
- `~/.codex/skills/handbook*` MUST be described as discovery glue only, pointing into `~/handbook/.agents/skills/*`.
- Docs and help text MUST make clear that there is no installed `~/handbook/bin/handbook-charter-intake` and no installed `~/handbook/share/**`.
- Missing-root, invalid-root, and missing-artifact recovery guidance MUST point to the setup family rather than to raw file-creation instructions.
- Fixture-backed execution demo support MUST remain scoped to the existing `generate` / `inspect` request surface and defer detailed boundary semantics to [`C-06`](C-06-fixture-execution-demo-boundary.md).
- Packet body structure, proof ordering, and renderer-specific output guarantees are owned by [`C-05`](C-05-renderer-and-proof-surfaces.md), not this contract.

### Runtime boundary

- The legacy Python harness remains legacy reference material only.
- The Rust CLI MUST NOT shell out to, wrap, or depend on the legacy harness as a supported runtime path.
- Docs and help text MUST preserve the distinction between supported Rust code and legacy reference material.

### Reduced-v1 install targets

- Reduced-v1 local install support MUST explicitly cover `macOS arm64` and `Linux x86_64`.
- The contract MUST NOT imply broader package-manager publishing or release automation beyond those local install targets.
- Any expansion of the target matrix or supported distribution story requires a contract revision.

### Compatibility and revalidation

- Changes to verb names, help ordering, crate ownership, or runtime boundary wording MUST trigger revalidation of downstream seams that consume `C-02`.
- Changes to the reduced-v1 install target matrix MUST also trigger revalidation because they affect the supported command story.
- This contract MAY be tightened by downstream implementation details, but it MUST NOT be broadened silently.

## Verification Checklist

- [ ] `Cargo.toml` defines a root workspace with `crates/cli`, `crates/compiler`, `crates/engine`, `crates/flow`, and `crates/pipeline` as members.
- [ ] `crates/cli` owns parsing, dispatch, and help text.
- [ ] `crates/engine`, `crates/flow`, and `crates/pipeline` own their extracted runtime logic directly.
- [ ] `crates/compiler` remains a narrow compatibility/support seam and is not the default umbrella import path for extracted logic.
- [ ] `--help` shows the supported surface in setup-first order and presents the setup family as `setup`, `setup init`, and `setup refresh`.
- [ ] `--help` shows `author` between `setup` and `pipeline`, and documents `handbook author charter`, `handbook author charter --validate --from-inputs <path|->`, `handbook author charter --from-inputs <path|->`, `handbook author project-context`, `handbook author project-context --from-inputs <path|->`, and `handbook author environment-inventory`.
- [ ] Help text matches the supported reduced-v1 command story, documents the routed setup family, documents `author` as the canonical authoring surface, documents `pipeline` as the orchestration surface, and exposes the M2 compile wedge plus the M3 capture wedge.
- [ ] `pipeline` owns route resolution, explicit stage compilation, and narrow pipeline-run state mutation once the family lands.
- [ ] `pipeline capture` is documented as the explicit stage-output writer surface for the bounded M3 / M3.5 wedge.
- [ ] The documented `pipeline.foundation_inputs` capture targets are `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, and `stage.10_feature_spec`.
- [ ] `pipeline compile` remains payload-only stdout, raw stage-10 compile payload is documented as refused by `pipeline capture` with `invalid_capture_input`, and stage `10` materialization is documented only as `compile -> external model output -> capture`.
- [ ] `needs_project_context` remains documented as a manual `pipeline state set` plus `pipeline resolve` step rather than an automatic capture side effect.
- [ ] Setup docs and help state that `setup refresh` preserves canonical files by default, that `--rewrite` touches only the three baseline starter files, and that `--reset-state` touches only `.handbook/state/**`.
- [ ] Setup docs and help state that setup success hands off to `handbook doctor`.
- [ ] `doctor` docs and help state the four baseline states exactly and document checklist lines as artifact label + canonical path + status + exact author command.
- [ ] `doctor` docs and help state that `handbook doctor --json` is the machine-readable readiness surface.
- [ ] Docs and help identify `install/handbook-home/` as the authored install-home source, `~/handbook/` as the installed home, `~/handbook/bin/handbook` as the only installed executable, `~/handbook/runtime-manifest.json` plus `~/handbook/resources/**` as installed runtime contract surfaces, repo `.agents/skills/*` as thin generated projections only, installed thin projections as living under `~/handbook/.agents/skills/*`, and `~/.codex/skills/handbook*` as discovery glue only.
- [ ] Docs and help state that there is no installed `~/handbook/bin/handbook-charter-intake` and no installed `~/handbook/share/**`.
- [ ] `generate` supports ready-path planning packet output from canonical repo-local `.handbook/` inputs.
- [ ] `inspect` is documented as the packet proof surface.
- [ ] `doctor` is documented as the recovery surface.
- [ ] Fixture-backed execution demo support is described only at the command-surface level here and defers detailed boundary semantics to `C-06`.
- [ ] The Rust CLI does not invoke the legacy Python harness as a supported runtime dependency.
- [ ] The reduced-v1 local install targets are explicitly limited to `macOS arm64` and `Linux x86_64`.
- [ ] `README.md`, `PLAN.md`, and `docs/README.md` each point to this contract.
- [ ] Downstream seams can revalidate against this contract without hidden context.
