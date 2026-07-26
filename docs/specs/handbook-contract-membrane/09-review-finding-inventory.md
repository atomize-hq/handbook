# Review Finding Inventory

**Status:** active control-pack authority for review severity, non-blocking
advisory disposition, duplicate comparison, and durable review debt.

## Purpose

This document is the single canonical inventory for valid review findings that
the active parent does not repair in the reviewed change. It prevents Optional
and Nit findings from becoming implicit merge blockers, preserves them as
visible engineering debt, and lets later reviewers compare a candidate finding
with known prior evidence before creating a duplicate.

The inventory does not waive defects. A prior entry cannot downgrade a new
Critical or Required finding, make stale evidence current, or authorize work
outside the selected slice.

## Decision and retained rigor

The calibrated process reduces serial review transaction cost rather than its
quality standard:

- P1/P2 remain blocking and retain mandatory remediation plus different-fresh
  re-review;
- the final aggregate subject still receives independent review and a complete
  proof wall;
- HIGH/CRITICAL impact analysis, fail-closed evidence, selector boundaries, and
  genuine authority stops remain;
- P3/P4 become durable, searchable advisories instead of implicit blockers;
- independently reviewable child packets, same-subject review bursts,
  delta-focused re-review, and tiered verification shift discovery earlier and
  batch remediation.

The default automatic review budget is one complete-subject discovery review
or same-fingerprint burst, one consolidated remediation pass, and one
different-fresh delta-focused closure review. The selected plan or dispatch may
declare a smaller or explicitly approved larger budget. Exhausting that budget
does not waive a valid P1/P2: it ends automatic churn and returns a bounded
partial/blocked result for parent or human scope/priority adjudication.

Rejected alternatives are eliminating independent review, treating failing
selected proof as backlog, allowing self-approval, or automatically waiving a
finding because similar debt already exists.

## Review priority rubric

Reviewer labels map to inventory priorities exactly:

| Priority | Reviewer label | V1.3 finding severity | Closeout effect | Required disposition |
|---|---|---|---|---|
| `P1` | Critical | `critical` | blocking | Repair and obtain different-fresh re-review, or stop at a genuine authority/external boundary. A P1 cannot be deferred into this inventory as the reason to complete the current subject. |
| `P2` | Required | `major` | blocking | Repair and obtain different-fresh re-review, or stop at a genuine authority/external boundary. A P2 cannot be deferred into this inventory as the reason to complete the current subject. |
| `P3` | Optional / Consider | `warning` | non-blocking advisory | Repair when it materially improves the selected subject; otherwise add or update one inventory entry before true-stop closeout. |
| `P4` | Nit | `info` | non-blocking advisory | Repair only when it is safe and local; otherwise add or update one inventory entry before true-stop closeout. |

`CLEAN` means there is no unresolved valid P1 or P2 finding. A clean review may
return P3/P4 advisories. In v1.3 delegated-run evidence, such a run uses
`verdict: clean`; its advisory IDs may appear in `finding_refs`.
`verdict: findings` is reserved for a review containing at least one valid P1
or P2 and therefore retains the existing mandatory remediation and
different-fresh re-review lineage.

An informational observation with no requested change is not a review finding
and does not enter this inventory.

## Priority decision matrix

Use the highest applicable row:

| Signal | P1 | P2 | P3 | P4 |
|---|---|---|---|---|
| Correctness or safety | demonstrated data loss, security boundary break, authority corruption, unsafe mutation, or incorrect success | selected contract violation, supported or repository-reachable incorrect result, authority bypass, missing required fail-closed behavior, or proof that cannot support the claimed status | defense-in-depth gap, malformed-input hardening, speculative robustness, or plausible improvement without a demonstrated violation in the integrated selected gate | no correctness effect |
| Scope or architecture | unauthorized public/API/dependency/unsafe-policy expansion or ownership inversion | selected-packet boundary violation, material coupling regression, or required owner/path mismatch | maintainability or simplification opportunity that preserves the current contract | naming, phrasing, or local consistency preference |
| Verification | claimed completion is unsupported or required destructive/negative path is untested | a required gate, regression, platform proof, or exact subject check is missing | additional useful coverage beyond the selected proof wall | cosmetic test organization or message quality |
| Immediacy | merge/closeout would be unsafe or materially false | selected work is incomplete against approved authority | safe to close while retaining explicit debt | safe to close; very low-cost polish |

Uncertainty does not justify lowering priority. If evidence cannot distinguish
P2 from P3, the reviewer reports the missing evidence and the parent resolves
the uncertainty before closeout.

Priority follows the integrated selected outcome, not the most severe isolated
component observation. A redundant schema/semantic layer disagreeing on input
that the combined gate already refuses is normally P3. It rises to P2 only
when the selected contract explicitly requires independent enforcement, the
disagreement bypasses authority, or a supported/repository-reachable state can
produce an incorrect accepted result. The parent records evidence when it
reclassifies a reviewer label; reviewer wording alone does not set priority.

## Canonical entry shape

Each open or historical entry uses:

| Field | Meaning |
|---|---|
| `finding_id` | Stable `HCM-RF-####` identity allocated once. |
| `priority` | `P1` through `P4`. An open P1/P2 records a blocked/partial/escalated stop and cannot appear as accepted debt in a completed closeout. |
| `status` | `open`, `accepted`, `scheduled`, `resolved`, or `superseded`. |
| `comparison_key` | Stable tuple `owner-or-seam \| surface \| contract-or-gate \| failure-mode` used for duplicate comparison. |
| `summary` | One precise statement of the improvement or debt. |
| `source` | Slice, review dispatch/run, and original reviewer label. |
| `evidence_refs` | Repository-relative file, line, proof, test, issue, or dispatch references. |
| `affected_scope` | Exact owner, paths, symbols, or seams; no inferred program-wide scope. |
| `disposition` | Why it was not fixed in the selected subject and the safe current behavior. |
| `target` | Future slice/packet/issue or `unassigned`. |
| `occurrences` | Later review refs that independently rediscovered the same comparison key. |
| `resolution_refs` | Commit/proof/review refs when resolved or superseded. |

This document contains exactly one `## Inventory` section. Only the complete
twelve-field table directly inside that section is registration evidence;
later sections, appendices, duplicate Inventory headings, and table-shaped
text elsewhere are not inventory entries.

## Admission and duplicate rules

1. The parent validates a reviewer finding against current authority and live
   truth before admitting or updating an entry.
2. Reviewers identify findings independently, then compare P3/P4 candidates
   against this inventory. They cite an existing `finding_id` when the
   comparison key and underlying defect are the same.
3. A duplicate does not create a new row. Append its review reference to
   `occurrences` and update evidence, priority, or target only when new facts
   justify the change.
4. Similar wording is not sufficient for deduplication. Different owners,
   violated contracts, failure modes, or required remedies remain distinct.
5. New evidence may raise an existing P3/P4 to P1/P2. The active review then
   uses `verdict: findings`; the old entry is not a waiver.
6. Lowering priority or marking an entry resolved requires evidence. Resolution
   cites the implementing commit, verification, and independent review when the
   underlying change was material.
7. Reviewers do not flag a known unchanged P3/P4 merely to force its repair.
   They re-report it only when the selected change worsens it, makes it newly
   relevant, changes its evidence, or justifies a different priority.

## Registration and review-subject boundary

During an active orchestration run, the parent collects validated findings. At
a completed true-stop it appends or updates unfixed P3/P4 rows in the mechanical
closeout commit beside the parent handoff and ledger. At a blocked, partial, or
escalated true-stop it may also register open P1/P2, which remain blockers rather
than accepted debt. The final review or blocker record identifies the findings
and their intended inventory disposition.

An exact inventory registration is an administrative transcription and does
not alter the reviewed product/control-pack subject fingerprint. A semantic
rewrite of this rubric, a priority downgrade, a changed finding meaning, or a
claimed resolution is material and requires independent review.

At slice start, load only inventory entries whose affected scope or
comparison key intersects the selected packet. At phase exit, triage all open
P3 entries for that phase. P4 entries remain searchable history and do not
automatically rise in priority merely because time passed.

## Inventory

No unresolved advisory finding has been registered under this calibrated
policy yet.

| Finding ID | Priority | Status | Comparison key | Summary | Source | Evidence refs | Affected scope | Disposition | Target | Occurrences | Resolution refs |
|---|---|---|---|---|---|---|---|---|---|---|---|
