# HCM-0.8 Post-CLEAN Authority Continuation Plan

Status: proposed; requires fresh planning CLEAN before implementation

Authority: `../SPEC.md` and
`../decision/20260802-post-clean-authority-continuation-selector.md`

## Orchestration identity

- Parent: `20260802T025200Z--HCM-0-8--post-clean-authority-continuation`
- Outcome: `hcm-0.8-post-clean-authority-continuation-control-repair`
- Packets:
  - `HCM-0.8-P1-authority-continuation-planning`
  - `HCM-0.8-P2-authority-continuation-protocol`
  - `HCM-0.8-P3-authority-continuation-proof-closeout`

## Dependency graph and order

1. Freeze the exact independent authority provenance, canonical grant, slot,
   immutable Git baseline, diff/symbol/risk reconciliation, and terminality
   contract.
2. Obtain fresh planning discovery review; consolidate P1/P2 and use a
   different-fresh planning closure if required.
3. Add focused failing schema/semantic fixtures, including the exact HCM-3.2
   prefix, authority artifact/review, active successor, and completed
   consumption replay.
4. Add the exact HCM-3.2 authority artifact and optional dispatch/handoff schema
   and template fields; it remains inert until the completed HCM-0.8 handoff
   attests the final CLEAN review over its bytes.
5. Implement grant fingerprinting, direct predecessor admission, causal-slot
   validation, subject ceiling enforcement, and handoff-summary parity.
6. Converge focused fixtures, historical admission, ordinary validation, both
   self-tests, ledger parity, formatting, whitespace, and GitNexus scope proof.
7. Obtain fresh final aggregate review, remediate any P1/P2, and obtain a
   different-fresh closure when material remediation occurs.
8. Commit the reviewed primary state, create the mechanical v1.4 handoff and
   ledger commit, revalidate, and publish locally by expected-old CAS.

## Reviewability

Planning, protocol implementation, and final proof are separate packets. The
validator remains one existing file because the change extends its current
causal-sequence and handoff-chain owners; new helpers must isolate grant parsing
and extension validation rather than bolt conditionals into unrelated paths.
No production/runtime symbol is in scope.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| Extension becomes a generic budget reset | One grant per parent, exact authority-stop predecessor, unchanged budget, fixed slots, exact ceilings |
| Selector review is not truly pre-edit | First slot read-only; later roles rejected until its run is CLEAN |
| Old records become invalid | New schema fields optional; immutable corpus fingerprints replayed |
| Scope authority can drift | Independently issued/reviewed typed authority, canonical grant fingerprint, immutable Git baseline, and actual diff/symbol/risk reconciliation |
| Failed admission needs circular remediation | Admission P1/P2 terminates the immutable one-attempt grant and requires new external authority at a true stop |
| Claimed review is mistaken for proof | Replay a completed different-parent v1.4 handoff, completed CLEAN run, exact dispatch manifest entry, result subject, artifact bytes, and role separation |
| HCM-3.2 fixture becomes synthetic-only | Load the exact committed five dispatches and latest handoff, then build active and consumed direct successor handoffs |
| Burst membership escapes the allowance | Exactly one dispatch per cycle; fixed four-cycle ceiling per slot |
| Validator complexity obscures safety | Dedicated helpers, focused negatives, fresh review, and no unrelated refactor |

## Checkpoints

- Planning checkpoint: reviewed selector CLEAN; no schema/validator edit yet.
- RED checkpoint: exact positive fixture fails for the intended post-CLEAN
  reason and negative matrix is enumerated.
- GREEN checkpoint: focused fixture matrix passes without historical drift.
- Proof checkpoint: ordinary validation, both self-tests, ledger parity,
  formatting/whitespace, and change detection pass.
- Closeout checkpoint: final aggregate CLEAN, primary commit, mechanical
  closeout commit, CAS publication, unchanged remote/protected paths.
