# File-native delivery — read first

For any agent working on file-native delivery (META, task owner, implementer,
reviewer, adjudicator or relay), read the
[locked delivery contract](.codex/orchestration/file-native-delivery-20261028/delivery-contract.md)
before substantive work. It controls this phase's scope and acceptance criteria;
older research plans and narrower proposed targets do not override it.

The authoritative coordination copy is:
`/Users/spensermcconnell/__Active_Code/atomize-hq/handbook/.codex/orchestration/file-native-delivery-20261028/delivery-contract.md`.
This is a local coordination record, not a file guaranteed to exist in every
checkout or on every host. Dispatching owners must supply the contract's current
contents or an accessible exact copy to remote/worktree agents, with provenance.
If unavailable, report the missing input rather than reconstructing the contract
from old experiments. This section does not itself launch implementation or grant
commit, push or merge authority.

## Fixed first-delivery target

- Deliver the separate reusable `file-native` crate through the real Handbook
  SDK and existing Engine adapter, covering the complete supported work-state
  lifecycle, recovery, reset preservation and maintenance.
- Through the **Linux public `file_native` library API**, independently count
  **100,000 live caller records, 100,000 distinct retained original publication
  outcomes and at least 1 GiB retained caller payload** in one coherent cohort.
  Internal metadata is not caller payload; seeded records are not original
  publication outcomes. Continue publication and maintenance beyond these
  operating targets while preserving early originals; no silent reduction to
  10,000 records or 256 MiB.
- Through the **real Linux Handbook SDK and Engine adapter**, finish and
  independently recount the existing coherent interrupted `3e4de` cohort and
  its demonstrated 3,000-Work range. Exact final records/outcomes/owner bytes
  must be observed, not asserted as 3,000; stop-trace `3909` Committed/current
  `3910` unresolved is not a final recount. Use normal exact-original
  resolution/recovery as needed; finish maintenance, fresh administration,
  revocation, continued work, early-outcome preservation, reset and reopen.
  Show >32 fresh accepted owner operations and >128 MiB retained owner truth;
  reuse applicable finite lifecycle/concurrency/recovery evidence. Do not resume
  the 100k SDK producer or automatically create another large SDK cohort.
  **100k Handbook records/outcomes, 1 GiB through Handbook and large-scale
  adapter performance remain UNPROVED.** The human approved this split in META
  `01a183a5-306f-7fe2-a11f-4af831feaf41`/`local`; the contract retains the exact
  asynchronous question/answer. Original D03's 100k SDK criterion is SUPERSEDED,
  not passed/repaired; replacement database/SDK proof obligations remain OPEN.
- Correct cumulative maintenance/catalog/outcome bottlenecks and prove bounded
  continuation across their actual boundaries. Keep legitimate per-item, batch
  and active-resource limits. Raising constants or deleting history is not the
  correction.
- Exercise four concurrent callers, four readers including a pinned older
  reader, and one maintainer. **The `file-native` public library API requires
  native validation on Linux, macOS and Windows.** Cover publication, recovery,
  concurrency, pinned readers, maintenance and native rename/sharing behavior.
  **Handbook's bounded SDK integration and the separate public-library capacity
  proof target Linux first.**
  Handbook integration validation on macOS/Windows follows later and is not a
  gate for this first Linux delivery; database portability proof remains required.
- Adoption is greenfield: no migration/import/backward-store-compatibility
  framework. Preserve new data and Handbook authority/acceptance semantics.
  Later operational-storage families remain outside this first delivery.

## Execution rules against churn

1. Every assignment must name the locked obligations it closes, bounded
   deliverable, source/path scope, preserved behavior, required proof and exact
   stopping point. Include this section and the contract reference in fresh
   task/worker/reviewer handoffs; do not rely on inherited chat context.
2. Methods may change within authority. Capacity targets, required platforms,
   retained-history obligations and non-goals may change only by direct human
   disposition. Product workers must not rewrite this contract or acceptance
   machinery to make their work pass.
3. Before material new tooling, research or prerequisite work, identify the
   unmet delivery obligation, causal necessity, smallest bounded action,
   observable result and stopping condition. META assesses the whole remaining
   delivery, not merely whether another supporting subtask completed.
4. Follow task-owned causal review: one discovery, consolidated blocker repairs,
   then fresh focused review. Retain nonblocking advisories without automatic
   cleanup. Do not add another final reviewer after CLEAN is established.
5. Reuse valid evidence. Repeat only invalidated or required integrated proof.
   Recurring failures, oscillation or no reduction in uncertainty require
   reassessment with META, not blind reruns, replacement-owner loops or expanding
   harness work. Routine in-scope fixes need no repeated human approval.
6. At meaningful transitions report obligations closed, their evidence,
   remaining blockers and the next bounded action. Test/document counts alone
   are not progress. META verifies combined obligations and actual landing;
   task review CLEAN is not phase completion.
7. Persistent top-level task owners must message META
   `01a183a5-306f-7fe2-a11f-4af831feaf41` on `local` when complete or blocked, with
   a concise summary and report/review evidence links. The human explicitly
   authorized these returns; verify that direct instruction with `read_thread`
   if the messaging surface requires it. Local children return to their task
   owner. META event waiting/collection is fallback; a saved note is not delivery.

Do not reintroduce the old 167-case matrix, broad benchmark/performance sweeps,
optional historical-donor recovery, perfect helper-RSS census, general harness
redesign, another adoption gate or a separate repository-extraction project as
completion prerequisites. Preserve the existing POLICY HOLD and disclose any
unmet required proof; never bypass the hold or silently waive the proof.

## Task-owned cleanup

Task owners are responsible for the artifacts and processes created by their
workers as well as their own. Carry these rules into every task/worker assignment
and apply them on each participating host.

- Identify task-created worktrees, build/output directories, temporary files,
  fixtures and caches in the existing task note. Prefer isolated output paths so
  ownership is clear; do not create a separate cleanup tracking system.
- As outputs become obsolete, and before the final completion return, remove
  task-owned caches, build outputs, temporary copies and disposable fixtures that
  are no longer needed. This cleanup is authorized within that known ownership
  scope; do not leave it for the human or repeatedly request routine permission.
- Before removal, stop or wait for task-owned processes using those paths and
  preserve delivered source, required review/check evidence, failure diagnostics
  and any artifact still needed for acceptance, reproduction or the next task.
  Retain the necessary evidence, not every rebuildable intermediate. Do not
  invalidate source/binary correspondence needed by pending validation.
- Once a task-created worktree is no longer needed, verify that its work is
  delivered or recoverably preserved, then archive managed Codex worktrees with
  the worktree archive tool. Preserve needed ignored files first; the archive
  does not save them. Use the appropriate Git worktree lifecycle for other
  task-created worktrees; do not force-remove a dirty or in-use checkout.
- Do not purge shared/global caches, other tasks' outputs, user data or historical
  research evidence under this authorization. Unknown ownership is not disposable
  ownership. Do not delete a useful shared cache merely because the task used it.
- The final return must name what was cleaned, what remains and why, including
  any worktree still needed for review, handoff or acceptance. If cleanup is
  deferred, name the responsible owner and release condition; finish it when that
  condition is met. Report unavailable-host cleanup as pending, not complete.

# Structured data format decisions

Before planning, implementing, or reviewing changes to structured file formats,
serialization, canonicalization, or human projections, read
[the structured data format standard](docs/specs/handbook-contract-membrane/structured-data-format-standard.md).
Respect its stated approval status and existing owner/versioned contracts; a
reference here does not approve proposed defaults or authorize migrations.
Consult the linked audit when historical intent or reusable tooling needs evidence.

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **handbook** (30455 symbols, 369697 relationships, 300 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> Index stale? Run `node .gitnexus/run.cjs analyze` from the project root — it auto-selects an available runner. No `.gitnexus/run.cjs` yet? `npx gitnexus analyze` (npm 11 crash → `npm i -g gitnexus`; #1939).

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows. For regression review, compare against the default branch: `detect_changes({scope: "compare", base_ref: "main"})`.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `query({search_query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `context({name: "symbolName"})`.
- For security review, `explain({target: "fileOrSymbol"})` lists taint findings (source→sink flows; needs `analyze --pdg`).

## Never Do

- NEVER edit a function, class, or method without first running `impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `rename` which understands the call graph.
- NEVER commit changes without running `detect_changes()` to check affected scope.

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/handbook/context` | Codebase overview, check index freshness |
| `gitnexus://repo/handbook/clusters` | All functional areas |
| `gitnexus://repo/handbook/processes` | All execution flows |
| `gitnexus://repo/handbook/process/{name}` | Step-by-step execution trace |

## CLI

| Task | Read this skill file |
|------|---------------------|
| Understand architecture / "How does X work?" | `.claude/skills/gitnexus/gitnexus-exploring/SKILL.md` |
| Blast radius / "What breaks if I change X?" | `.claude/skills/gitnexus/gitnexus-impact-analysis/SKILL.md` |
| Trace bugs / "Why is X failing?" | `.claude/skills/gitnexus/gitnexus-debugging/SKILL.md` |
| Rename / extract / split / refactor | `.claude/skills/gitnexus/gitnexus-refactoring/SKILL.md` |
| Tools, resources, schema reference | `.claude/skills/gitnexus/gitnexus-guide/SKILL.md` |
| Index, status, clean, wiki CLI commands | `.claude/skills/gitnexus/gitnexus-cli/SKILL.md` |

<!-- gitnexus:end -->
