# Handbook Top-Level Task Orchestration Protocol

## 1. Authority and terminology

Create top-level tasks only when the user explicitly authorizes task creation and a closed ordered
sequence of exact slice IDs, which may contain one slice. The meta orchestrator cannot derive,
reorder, or extend that sequence. The persistent meta orchestrator controls scheduling. Fresh increment
orchestrators own whole Handbook slices or true-stop resumptions. Built-in subagents remain inside
an increment under the repository's v1.4 dispatch protocol.

The meta workflow uses `meta_workflow_id`. Handbook parent records retain their own
`orchestration_id`; never reuse one identifier for both layers.

## 2. State machine

```text
INITIALIZING -> READY -> DISPATCHING -> RUNNING -> RECEIPT_RECEIVED -> VERIFYING
VERIFYING -> READY(next preauthorized slice)
VERIFYING -> READY(same-slice bounded resumption)
VERIFYING -> EVIDENCE_RUNNING -> EVIDENCE_VERIFYING -> READY(gated slice)
VERIFYING -> COMPLETE
Any authority-expanding, external, or non-recoverable condition -> BLOCKED
```

Exactly one mutating increment may run for the local integration ref. Read-only evidence tasks may
run concurrently only when their source bindings and checkouts are independent.

`dispatch_authorized=false` means configured but idle. `delegated_authority` records whether the
user has allowed automatic dispatch through the declared sequence and bounded blocker
adjudication. It never permits sequence expansion.

## 3. Dispatch lifecycle

Before dispatch:

1. Verify the dedicated local integration ref's exact commit and tree.
2. Verify its required ancestor and the recorded remote baseline.
3. Resolve the exact project path, project ID, host ID, and repository status.
4. Read the live `07`, `08`, `09`, selected slice authority, and chosen handoff.
5. Validate the state, then render a prompt whose slice, successor, meta
   identity, refs, and commit/tree bindings match that state exactly and which
   contains a random nonce.
6. Create a fresh top-level task and bind its real task/thread and host IDs.
7. Persist the dispatch and end the meta turn.

An increment may not edit before its identity is bound. It must establish the reviewed selector
gate required by the selected slice, perform all internal delegation through v1.4 dispatches, and
reach a genuine true stop before returning a terminal receipt.

## 4. Successful receipt

Use protocol `codex.top-level-task-receipt.v1`, completion profile `handbook_v1_4`, publication
mode `local_only`, and status `CLOSED_LOCAL_CLEAN`.

The receipt includes:

- meta workflow, nonce, slice, and increment-task identity;
- expected base commit/tree;
- ordered primary commits and primary tip;
- closeout/final commit, final tree, local integration ref, and live local value;
- unchanged remote baseline/live value and `push_performed=false`;
- exact changed files and subject fingerprint;
- completed v1.4 handoff, final CLEAN dispatch, ledger, and digests;
- validator/self-test, proof, protected-path, and GitNexus results;
- authoritative-sequence correlation in `next_increment`.

`next_increment` is not free-form authority and a receipt does not dispatch anything. The meta
validates it against the preauthorized sequence, adjudicates current state, and then decides whether
the delegated authority permits the next task.

## 5. Blocked receipt

Supported statuses:

- `BLOCKED_CONTRADICTION`
- `BLOCKED_SCOPE_EXPANSION`
- `BLOCKED_REVIEW`
- `BLOCKED_NATIVE_EVIDENCE`
- `BLOCKED_PLATFORM_HANDOFF_REQUIRED`
- `BLOCKED_TASK_NOT_TERMINAL`
- `AUTHORITY_REQUIRED`
- `BASE_DRIFT`

The blocker states whether it is `same_slice_adjudicable` and identifies the exact missing
authority or external state. When Handbook reached a durable true stop, the receipt references the
committed v1.4 handoff and safe commit stack. If durable closeout failed, it records the failure
explicitly; the meta cannot treat the receipt as a substitute handoff.

A bounded same-slice resumption uses a new nonce, keeps the sequence cursor unchanged, and starts
from the verified safe closeout commit. Scope or authority expansion requires the user.

## 6. Independent verification

Treat every receipt as untrusted. The meta verifies:

1. Structural schema and expected successor.
2. Meta workflow, dispatch nonce, slice, task identity, and terminal barrier.
3. Primary/closeout topology and exact commit/tree claims.
4. Expected-old local ref publication and unchanged remote baseline.
5. Exact changed paths and protected path exclusion/hash equality.
6. Completed handoff schema/semantics, final CLEAN review, subject fingerprint, complete parent
   dispatch population, and finding disposition.
7. Ledger parity, ordinary validation, both self-tests, proof wall, and formatting.
8. GitNexus staged/scoped result and truthful compare-to-main availability.

Only after verification may the meta persist the receipt and update state.

## 7. Local publication

The increment creates the reviewed primary commit stack, then the mechanical closeout commit. It
rechecks the integration ref against the expected base immediately before publication and updates
the dedicated local ref atomically with expected-old semantics. No push is permitted. A moved ref
returns `BASE_DRIFT`; never merge, rebase, reset, clean, or force-update it.

## 8. Concurrency and correction

Permit one mutating increment per integration ref, internal subagents inside that increment,
multiple read-only reviewers under v1.4 rules, and independent evidence tasks.

The meta may send a correction to an active task only inside existing authority. After a committed
true stop it may create a same-slice resumption only when delegated adjudication permits it. It may
not use a new task to reset a causal budget or erase prior findings.

Before accepting completion or dispatching a resumption, verify the completed/blocked v1.4 record
preserves the parent/outcome-derived cadence: one discovery/burst, consolidated remediation,
different-fresh closure, no more than two causal supplementals without an explicit reviewed budget
extension, no reopened general discovery, and no cycle after CLEAN. A new task continues the same
causal lineage; it does not create a new discovery allowance.

## 9. Bootstrap identity

Create the meta task in `INITIALIZING` with dispatch forbidden. Send a follow-up binding its real
task/thread ID, host ID, meta workflow ID, state path, integration ref, remote baseline, and start
posture. Run `bind_meta_identity.py`; this moves to `READY` without enabling dispatch.

Only explicit user authorization sets `dispatch_authorized=true` and the bounded delegated
authority fields.
