# Phase-3 exit audit selector

**Status:** reviewed selector. Discovery dispatch
`20260809T143200Z--HCM-3-6--phase-3-exit-selector-discovery-review` returned
`CLEAN` with zero P1/P2 and zero advisory findings.

**Parent orchestration:** `handbook-hcm-3-6-phase-3-exit-20260809`

**Selected outcome:** `hcm-3-phase-3-exit-audit-and-closeout`

## Authority and predecessor boundary

This selector authorizes exactly one documentation/control-pack outcome: a
cumulative, evidence-backed Phase-3 exit audit over completed HCM-3.1 through
HCM-3.6, followed by a two-commit local-only closeout. It consumes the
immutable predecessor handoff
`20260808T220000Z--HCM-3-6--orchestration--project-posture-implementation-completed`
as evidence only. It neither resumes nor alters that parent, its dispatch
population, its ledger entry, its admission hashes, or validator semantics.

The selected base is commit
`bfedb7131a66bbed6a574ee2a90336439161b117`, tree
`9e51ebdde2ed57ed7fa17c607e34d15dbba785da`; the required ancestor and local
remote-tracking observation are
`1256e724a2b7da6b6250f57d6f63fced1e2cf949`. Publication is `local_only` to
`refs/heads/orchestration/handbook-phase-3-exit-20260809` through an
expected-old compare-and-swap. The protected checkout
`C:\\Users\\spmcc\\Documents\\__Project_Code\\handbook` is read-only and
must remain byte-identical, clean, unstaged, and uncommitted.

## Allowed subject

The audit may add the durable Phase-3 exit authority decision and proof under
`docs/specs/handbook-contract-membrane/phase-3-exit/`, update the canonical
control-pack truth in `00-README.md`, `04-phase-slice-map.md`, and
`06-proof-and-regression-ledger.md` when the audit proves it, and update
`07-orchestration-onboarding-prompt.md`,
`08-handoff-ledger-and-escalation-protocol.md`, and
`09-review-finding-inventory.md` so future workflows discover and consume the
new authority. It may create current v1.4 internal dispatches, one completed
parent handoff, the deterministic ledger rebuild, and exact P3/P4 inventory
transcription.

The permanent operator-governance decision must state every term in the bound
increment contract, including the exact inherited raw failure
`20260806T202700Z--HCM-3-5--resolution-aware-adoption-planning: continuation writes or advances before selector CLEAN`.
It must classify that failure as
`accepted_permanent_operator_governance_exception`, preserve ordinary
non-GREEN semantics, permit closeout only when it is the sole ordinary
validator failure and every other applicable gate passes, require future
selector/proof/handoff citations, preserve immutable history, and waive no
other failure.

## Required audit proof

The audited subject includes the live `00` through `09` control-pack authority,
all HCM-3.1 through HCM-3.6 specifications, decisions, proofs, completed
handoffs, ledger entries, and v1.4 dispatch lineage. The evidence must prove
or truthfully stop on all Phase-3 exit gates:

1. canonical truth yields deterministic Resolution Projections;
2. stable snapshots and deterministic deltas ground transitions;
3. receiving-session envelopes project comprehensive snapshots;
4. instability and redaction are explicit and covered;
5. custom vocabulary is consistent in generated surfaces;
6. omitted claims cannot false-pass;
7. work-level behavior is intentionally represented or removed; and
8. posture recommendations are evidence-linked and advisory while resolved
   posture is not another editable authority.

The audit must rerun every applicable Phase-3 engine, Flow, pipeline,
workspace, formatting, negative/fail-closed, handoff, ledger, self-test, and
proof-wall validation. Missing or unavailable evidence is recorded as such,
never GREEN. It must triage each open Phase-3 P3 under `09`; P4 remains
searchable history unless independent evidence raises it.

## Non-goals and stop conditions

No product source, tests, fixtures, schemas, dependencies, lockfiles, runtime
configuration, public surfaces, generated product assets, immutable evidence,
remote state, protected checkout, publication, HCM-4+, HCM-5+, SDK, CLI,
Tauri, Substrate, gate runtime, or feature-branch promotion may change. A
product/schema/dependency/runtime/public-surface/immutable-evidence defect is
an authority stop with exact evidence, not a repair target here.

The primary commit contains reviewed selector/authority/proof/control-pack
truth only. The separate closeout commit contains only the completed v1.4
handoff, rebuilt ledger, directly required indexes, and exact inventory
transcription. Completion requires zero unresolved P1/P2, a final different-
fresh CLEAN review, ordinary validator truthfully recorded, both self-tests,
proof wall, GitNexus scoped detection, compare-to-main truth, protected-path
proof, and successful local CAS. No push is permitted.
