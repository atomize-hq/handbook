# HCM-2.4 planning amendment supplemental causal remediation 1

Recorded: 2026-07-26T12:59:38Z
Reviewer: `/root/hcm_2_4_amendment_review_2`
Reviewed fingerprint:
`sha256:2f1ab30e763da83e2d405be3d7e00fcf6e478a92f256fbee2016dc7ae609d6e4`
Dispatch:
`20260726T125058Z--HCM-2-4--planning-amendment-closure-review`

The different-fresh closure review confirmed
`HCM-2.4-AMEND-REV1-002` closed and found one directly unmasked P2 in
`HCM-2.4-AMEND-REV1-001`: the first caller inventory omitted compiler
`doctor_report_from_inspection`.

## Live evidence

Repository-wide source inspection proves six existing production calls to
`CharterDefinitionRegistry::validate_selected_decisions`:

1. `resolve_shipped_profile_decisions`;
2. `CharterApprovalServiceV1::approve_inner`;
3. `CharterPromotionWorkflowServiceV1::promote_at`;
4. `validate_definition_authority`;
5. `CharterAuthorityTransactionServiceV1::preflight`; and
6. compiler `doctor_report_from_inspection`.

The compiler anchor has exact GitNexus UID
`Function:crates/compiler/src/doctor.rs:doctor_report_from_inspection`. Fresh
upstream impact with tests at depth 3 is HIGH: 12 impacted, 2 direct, 1 process,
and 3 modules. It is read/proof-only; the compatibility boundary change already
flows through its existing call, so no compiler production edit is authorized.

## Consolidated correction

- The live baseline is six existing registry callers.
- P1B adds exactly one seventh caller from `evaluate_charter_intake` so the
  public intake boundary fails closed before producing records.
- An eighth caller, new process/module/authority class/public surface, or wider
  unexplained HIGH/CRITICAL result is RED.
- Compiler `doctor_report_from_inspection` is present in the P0/P1B
  manifest/impact table and its existing definition-closure behavior is
  preserved under direct 1.1 and selected 1.2 decisions.
- The exact proof wall adds the compiler doctor unit and CLI typed
  profile/doctor command.
- The new intake negative and all other compatibility constraints remain
  unchanged.

This is supplemental causal remediation 1 for the amendment review. It does not
reopen discovery or any unrelated HCM-2.4 decision. A new different-fresh
read-only causal closure review is required. No implementation was performed.
