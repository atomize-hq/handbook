---
name: orchestrate-top-level-tasks
description: Coordinate an explicitly authorized sequence of Handbook slices or true-stop resumptions across fresh top-level Codex tasks. Use a persistent meta orchestrator, Handbook v1.4 causal review and handoff authority, local-only commit publication, structured terminal receipts, and independent verification before bounded adjudication or subsequent dispatch.
---

# Orchestrate Handbook Top-Level Tasks

Run a persistent scheduling and verification task above Handbook's existing slice protocol. The
meta orchestrator creates fresh user-visible top-level tasks for whole slices or genuine true-stop
resumptions. Each increment task runs the repository's `07`/`08` protocol internally and owns its
selected slice through the v1.4 two-commit closeout.

The meta orchestrator is not a replacement Handbook authority. Its receipts are scheduling
evidence. The repository's selector, review dispatches, completed v1.4 handoff, and ledger remain
durable product and control-plane authority.

## Load the complete contract

Before creating or messaging any top-level task:

1. Read `references/protocol.md` completely.
2. Read `references/handbook-adapter.md` completely.
3. Read `references/platform-dispatch.md` when a gate is host- or OS-specific.
4. Read the live repository files:
   - `docs/specs/handbook-contract-membrane/07-orchestration-onboarding-prompt.md`;
   - `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`;
   - `docs/specs/handbook-contract-membrane/09-review-finding-inventory.md`.
5. Use the prompt templates under `assets/`.

Do not create a top-level task unless the user explicitly authorized task creation and a closed
ordered sequence of exact slice IDs, which may contain one slice. Never derive, reorder, or extend
that sequence. Setup-only authorization does not authorize dispatch.

## Preserve the two orchestration levels

- **Meta orchestrator:** persistent top-level task that schedules, verifies, and performs bounded
  adjudication for the user-authorized sequence.
- **Increment orchestrator:** fresh top-level task that owns one whole Handbook slice or one
  genuine true-stop resumption.
- **Handbook internal subagent:** fresh built-in agent dispatched through a schema-valid v1.4 JSON
  dispatch by the increment orchestrator.
- **Evidence task:** read-only top-level task for an exact native/platform proof gate.

Never create top-level tasks for ordinary Handbook child packets, reviews, remediation, proof
gaps, or cross-document repairs that remain inside the selected slice. The increment orchestrator
keeps those internal and stays active as required by `07` and `08`.

Create increment orchestrators with `create_thread`, never `fork_thread`.

## Apply delegated meta authority correctly

A terminal receipt does not authorize the next slice by itself. After independently validating
the receipt and live repository/control-plane state, the meta orchestrator may:

- launch the next task in the exact user-preauthorized sequence when no blocking issue is visible;
- send a correction to an active increment when the correction remains within its authority;
- adjudicate a bounded blocker and dispatch a same-slice true-stop resumption when the existing
  slice authority, causal budget, risk ceiling, and sequence envelope already permit it.

The meta orchestrator may not invent a slice, expand the sequence, broaden authority or risk,
waive P1/P2 findings, rewrite immutable evidence, or start work beyond the user-authorized
envelope. Those conditions require user authority. Every fresh slice task must still establish or
consume and validate its reviewed slice selector before implementation.

## Enforce causal review cadence

Treat the live `07`/`08`/`09` cadence as mandatory, not advisory. For each registered integrated
outcome and typed stage, permit one complete-subject discovery review or same-fingerprint burst,
one consolidated remediation, and one different-fresh delta-focused closure. Permit at most two
`supplemental_causal` remediation/closure cycles only for P1/P2 findings directly caused or
unmasked by the immediately preceding remediation while scope, authority, and risk stay fixed.

Never reopen general discovery during closure, run a cycle after CLEAN, split or rename packets to
reset a budget, relabel a remediation-unmasked gap as new discovery, or use a new top-level task to
erase causal lineage. Mechanical closeout is not a review cycle. Exhaustion stops bounded work; it
never waives a valid P1/P2. A larger budget requires an explicit reviewed authority reference.

## Use local-only publication

Handbook completion uses `publication_mode=local_only`:

1. Seed a dedicated, unshared local integration ref from the exact authorized local base.
2. Never use a branch checked out in the protected product checkout as that integration ref.
3. Run each increment in its own task worktree at the expected commit and tree.
4. Let the increment create a reviewed primary commit or commit stack and a separate mechanical
   v1.4 handoff/ledger closeout commit.
5. Publish locally with a compare-and-swap ref update only after all gates pass.
6. Do not push. The configured remote ref must remain at its recorded baseline.

The final closeout commit is the completed increment checkpoint. The final primary tip remains the
`reviewed_state.baseline_head` in the completed v1.4 handoff.

## Dispatch an increment

1. Verify the local integration ref equals `state.expected_base.commit` and its tree matches.
2. Verify the configured remote ref still equals the recorded remote baseline; do not fetch and
   reinterpret a moved remote as local authority.
3. Load only the selected slice contract and common Handbook workflow envelope.
4. Render the prompt with `scripts/render_increment_prompt.py`.
5. Generate a unique dispatch nonce.
6. Create a fresh top-level task at the exact starting checkpoint.
7. Resolve and record the real task/thread ID and host ID.
8. Send an identity-binding follow-up authorizing that nonce-bound dispatch.
9. End the meta turn after dispatch.

The increment orchestrator may use built-in subagents and must satisfy all live `AGENTS.md`, `07`,
`08`, skill, GitNexus, review, proof, and protected-path requirements. It sends its receipt only
after the closeout commit and all validation exist. `send_message_to_thread` is its last tool
action.

## Accept a completed receipt

Validate structure with:

```powershell
python scripts/validate_receipt.py <receipt.json> --expected-next <value>
```

Then independently verify at minimum:

- meta workflow ID, nonce, slice, task identity, expected base, and target ref;
- task terminal barrier;
- exact primary commit sequence, primary tip, closeout commit, final tree, and ancestry;
- local integration ref equals the closeout commit through an expected-old compare-and-swap;
- configured remote ref remains unchanged and no push occurred;
- exact changed-path aggregate and protected-path hashes;
- completed repository-relative v1.4 handoff and ledger entry;
- final CLEAN dispatch, reviewed subject fingerprint, full parent dispatch population, P1/P2
  closure, and P3/P4 disposition;
- causal-budget identity and v1.4 proof that the cadence was respected, general discovery was not
  reopened, no cycle followed CLEAN, and no budget reset occurred;
- ordinary handoff validation, both self-tests, proof wall, formatting, and scoped change
  detection;
- any unavailable GitNexus comparison is recorded as unavailable rather than GREEN.

Only then persist the receipt and advance state. The meta orchestrator decides whether the verified
state permits dispatch; `next_increment` is a correlation claim checked against the preauthorized
sequence, not independent product authority.

## Handle a true stop

Accept the blocked statuses listed in `references/protocol.md`. A blocked receipt must identify
whether the meta orchestrator can adjudicate inside existing authority or user authority is
required. A genuine Handbook true stop must reference its committed repository-relative v1.4
handoff and any safe primary/closeout commits. A receipt alone is not durable Handbook closeout.

Do not advance the sequence for a blocked result. Same-slice resumption retains the cursor and uses
a new dispatch nonce and exact new base.

## Dispatch platform evidence

Use direct, read-only platform tasks only when the declared slice gate requires them and the exact
local checkpoint is already accessible on that host. Local-only policy never silently pushes a
checkpoint to make it accessible. If the host cannot access the exact commit/tree, stop with
`BLOCKED_PLATFORM_HANDOFF_REQUIRED` and provide the human transfer/evidence package.

Evidence receipts are scheduling evidence. The increment that consumes them must incorporate the
durable proof references into its v1.4 handoff.

## Isolate orchestration state

Persist meta state, rendered prompts, and receipts on an isolated local orchestration ref or in a
dedicated control repository:

```text
orchestration/
├── meta-orchestration-prompt.md
├── increment-orchestrator-prompt-template.md
├── increments/
├── state.json
└── receipts/
```

Do not merge runtime orchestration state into the Handbook product branch. The repository-local
skill itself is versioned developer tooling; its runtime state is not.

## Validate and render

```powershell
python scripts/validate_state.py <state.json>
python scripts/validate_receipt.py <receipt.json> --expected-next <value>
python scripts/validate_evidence_receipt.py <evidence-receipt.json>
python scripts/render_increment_prompt.py `
  --state <state.json> `
  --template assets/increment-orchestrator-prompt-template.md `
  --variables <variables.json> `
  --contract <increment-contract.md> `
  --output <rendered-prompt.md>
```

Reject unresolved placeholders. Never infer missing authority, repository identity, causal
lineage, or evidence.
