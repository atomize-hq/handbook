# P0 Absolute Rules (Scoped)

These rules are **non-negotiable** across languages, stacks, and runners.

Some P0 rules are scoped by **work level** so you can run parallel workstreams
at higher planning levels without creating execution drift.

## Work levels (hierarchy)

- **L0 Program**: roadmap/portfolio decisions and sequencing. Parallelism is normal.
- **L1 Project/Planning**: charter, project context, foundation, feature specs, phase plans. Parallelism is normal.
- **L2 Slice Execution**: implementing *one* slice in a single working context (worktree/branch). Keep execution single-threaded.
- **L3 Quality Gate & Merge**: final verification/integration/merge discipline.

### Scoped rule blocks

This ruleset supports optional scoped blocks in the form:

- `<!-- SCOPE: L2,L3 -->` (start)
- `<!-- END_SCOPE -->` (end)

Only include scoped blocks when the current stage `work_level` matches.

## P0-ABSOLUTE: Safety & Security
- **Never compromise security** for speed or convenience.
- **Never introduce secret leakage**:
  - do not hardcode credentials/tokens/keys
  - do not print secrets in logs
  - prefer secure defaults

## P0-ABSOLUTE: Truthfulness & Evidence
- **No evidence, no pass**: do not claim tests/lint/typecheck/security passed unless you have evidence.
- If you cannot run commands (no tools), you must:
  - state that you did not run them
  - provide the exact command(s) to run (from the selected profile)

## P0-ABSOLUTE: Scope & Determinism
- Do only what the current slice/stage requires.
- If requirements are ambiguous and would change behavior, **ask** (do not guess).
- Prefer deterministic, machine-checkable outputs:
  - avoid vague acceptance criteria
  - prefer explicit checks and exit-code based gates

## P0-ABSOLUTE: Change Discipline
- Keep changes minimal and reversible when touching contracts or production systems.
- If a change introduces/changes env vars, services, ports, or runtime assumptions:
  - add new named environments or capabilities to `.handbook/project/environment.yaml` before a work contract references them.

<!-- SCOPE: L2 -->
- **L2 Execution:** One slice implementation in flight per worktree/agent context.
  - Parallel workstreams are allowed across worktrees/branches.
  - Do not run multiple slice executions in the same working context.
<!-- END_SCOPE -->

<!-- SCOPE: L3 -->
- **L3 Merge:** Do not merge multiple slices simultaneously into the same target branch unless:
  - CI proves independence (conflict-free + gates pass), AND
  - you have an explicit integration plan.
<!-- END_SCOPE -->

## P0-ABSOLUTE: Output Contract
- When a stage/template requires “output only the final document”, do not add extra commentary.
- When a stage requires multi-file output blocks, follow the wrapper format exactly.
