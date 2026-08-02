# HCM-0.8 Retained-Finding Lineage Extension Proof

## Authority and regression

Authority is the exact artifact
`../authority/20260802-retained-finding-lineage-extension.json`. The immutable
nine-dispatch population contains two reviews that retained an earlier P1/P2:
`HCM08-PLN-DISC-001` and `HCM08-DISC-P2-003`. Reassigning either finding to the
later review would duplicate or transfer ownership; omitting it would launder
the next cycle's exact trigger array.

## Selected representation

An optional `delegated_runs[].carried_finding_refs` contains stable finding IDs.
Each ID still has exactly one owner through `findings[].source_run_id` and that
owner's `finding_refs`. A carrier is valid only when it is a later FINDINGS
review, the finding is P1/P2, the owner is an exact trigger of the carrier's
typed review cycle, and completed remediation links that owner to the carrier.
The next cycle derives its exact P1/P2 array from the union of each immediate
trigger run's owned and carried IDs.

Absent `carried_finding_refs` means an empty set, preserving every old v1.4
record. Frozen v1.0-v1.3 schemas, records, dispatches, and admission hashes are
unchanged.

## Required proof

- The exact nine immutable dispatch hashes match the authority artifact.
- A completed HCM-0.8 handoff owns every finding once and carries exactly the
  two retained IDs on their later FINDINGS reviews.
- Duplicate ownership, fabricated/unowned carry, P3/P4 carry,
  non-predecessor carry, missing remediation, and trigger-array laundering fail.
- Existing cycle-after-CLEAN, third-supplemental, undeclared-outcome, budget,
  and identity-reset negatives remain green.
- Ordinary validation, historical admission, orchestration self-test, exact
  HCM-3.2 replay, live review dispatch replay, ledger parity, formatting,
  whitespace, protected paths, and scoped/compare GitNexus checks pass.
