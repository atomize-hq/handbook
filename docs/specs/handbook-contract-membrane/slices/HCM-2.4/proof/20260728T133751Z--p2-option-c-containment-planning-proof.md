# HCM-2.4 P2 Option C containment planning proof

Timestamp: `20260728T133751Z`

Mode: planning containment and product-decision reassessment only

## Authority

- selected handoff:
  `20260728T011713Z--HCM-2-4--orchestration--p2-course-correction-decision-required`;
- orchestration-control repair handoff:
  `20260728T041820Z--HCM-0-8--orchestration--causal-review-budget-lineage-hardening-completed`;
- control repair primary commit:
  `4e6a9243aa0e64a8c8edee5b345b5cdd2c1fa7df`;
- control repair mechanical closeout:
  `b770a20b2e8ff8c96c60caf928f91691ee4bdd54`; and
- baseline branch: `feat/handbook-contract-membrane`.

The selected authority permits an Option C planning amendment, a bounded
product-decision reassessment, independent review, reviewed planning commits,
and one parent-owned closeout handoff. It does not permit runtime,
schema/definition, dependency, Cargo, unsafe, native integration, P4, P6, P7,
HCM-2.4 completion, or Phase 2 exit work.

## Baseline verified

1. Released HCM-0.6 requires deterministic fail-closed Environment Context
   applicability from current independent authoritative fact or admitted
   evidence. Unknown, stale, refused, or unresolved evidence remains
   indeterminate. Artifact presence and profile opt-in are insufficient, and
   Environment Context is excluded from proving its own applicability.
2. Native WebAuthn/FIDO/CTAP ceremony entered only through later HCM-2.4
   planning. It is not released P2 authority.
3. P2S and P2A are review-clean historical evidence. No production native
   adapter, adapter-specific unsafe code, Cargo/lockfile edit, or dependency
   change exists.
4. Both immutable shipped-root profiles `1.1.0` and `1.2.0` select three root
   instances, including conditional Environment Context. The live
   `shipped_profile_request` selects exact `1.2.0`.
5. The resolver returns `unresolved`,
   `EvidenceContractUnavailable`, and `indeterminate` for that conditional
   instance. Missing or otherwise structurally valid selected artifacts
   therefore produce repository readiness `INDETERMINATE`. Structurally
   invalid, unsafe, or unreadable artifacts take the higher-precedence
   `INVALID` path.

GitNexus was indexed at the product-code commit
`6734e52`; later stale commits affect orchestration documents, not this traced
runtime path. Concept query was degraded by the local missing FTS extension, so
exact symbol context, upstream impact, live source, and focused tests were used
instead. Current indexed upstream impact is CRITICAL for
`shipped_profile_request` (111 impacted, 59 direct, 10 processes, 16 modules),
`resolve_shipped_profile_decisions` (108 impacted, 54 direct, 9 processes,
17 modules), and `artifact_decision` (108 impacted, 54 direct, 9 processes,
17 modules). No code symbol is edited by this packet.

## Independent product-path trace

The user-visible path is:

1. `crates/engine/src/profile_decision.rs::shipped_profile_request` selects
   exact `handbook.profile.shipped-root@1.2.0`.
2. `resolve_shipped_profile_decisions` resolves its three instances.
   `artifact_decision` emits
   `ProjectConditionOutcome::Unresolved`,
   `ProjectConditionDecisionReason::EvidenceContractUnavailable`, no closure
   fingerprint, and `ArtifactApplicability::Indeterminate` for Environment
   Context.
3. `crates/engine/src/profile_inspection.rs` reports a missing conditional path
   as `NotInspected / ConditionalEvidenceUnavailablePathMissing`. If a valid
   Environment Context already exists, it reports
   `StructurallyValid / ConditionalEvidenceUnavailablePathPresent`; neither
   state changes applicability.
4. `crates/compiler/src/profile_readiness.rs::classify_readiness` first returns
   `RepositoryReadinessStatus::Invalid` for structurally invalid, unsafe, or
   unreadable artifacts. Only when no such artifact exists does any
   indeterminate applicability produce
   `RepositoryReadinessStatus::Indeterminate`.
5. `crates/compiler/src/setup.rs::setup_action` maps that state to
   `ConditionIndeterminate`. Setup may establish the `.handbook` root and
   repository identity, but it writes no selected artifact and cannot apply a
   reset while readiness is indeterminate.
6. CLI setup renders `OUTCOME: INDETERMINATE` and
   `ACTION: condition_indeterminate`. CLI doctor renders
   `OUTCOME: INDETERMINATE`; its JSON condition row is
   `outcome: unresolved`, `reason: evidence_contract_unavailable`, with no
   closure fingerprint.
7. `crates/cli/src/exit_policy.rs::repository_status` maps READY to exit 0 and
   ACTION_REQUIRED, INDETERMINATE, or INVALID to exit 1.

Exact effect: every repository using the hardcoded shipped-root 1.2 profile
remains unable to reach full READY while the released evaluator authority is
absent. Creating `.handbook/project/environment.yaml`, changing its
`applicability_basis`, or toggling a profile flag cannot repair this. At most,
the artifact row changes from missing/not-inspected to
present/structurally-valid while the repository outcome and nonzero exit remain
unchanged. Malformed, unsafe, or unreadable selected artifacts instead make the
repository INVALID and still exit nonzero; they do not create a READY path.

## Option C containment decision

This planning amendment freezes the following:

- future native-adapter selection is superseded;
- no native WebAuthn/FIDO/CTAP, USB/HID, device, platform UI,
  cancellation/error, unsafe, Cargo, lockfile, or dependency work is
  authorized;
- P2S schemas/evaluator and P2A feasibility evidence remain immutable
  historical evidence;
- P2 remains unresolved, fail-closed, and not GREEN;
- no Environment Context applicability is inferred, coerced, or self-declared
  under the released contract;
- independently authorized P4 or unrelated work may proceed only under its own
  later exact selector;
- P6 remains blocked until P2 and P4 are genuinely GREEN;
- P7 and Phase 2 exit remain blocked behind P6; and
- Option C is temporary containment, not a product-complete state.

Closeout of this packet means only that the planning containment decision is
reviewed and landed. It does not complete P2 or HCM-2.4.

## Product-decision reassessment

The bounded analysis is
[`decision/20260728-p2-product-decision-reassessment.md`](../decision/20260728-p2-product-decision-reassessment.md).
It compares:

- A, indefinite fail-closed behavior, as a safe baseline/non-solution;
- B, an additive default-profile successor without Environment Context;
- C, an explicit project-owner declaration with a complete lifecycle;
- D, a named real external or already-owned evidence authority through a
  verifier-only successor; and
- E, existing Charter operational-reality evidence as positive support only.

“Course-correction Option C” in the containment decision and comparison option
C in the A–E analysis are distinct namespaces. The former is the selected
temporary planning containment; the latter is an unselected project-owner
declaration alternative.

Direction B is recommended as the simplest usable product direction. It can
preserve exact Charter and Project Context authority and leave shipped-root 1.1
and 1.2 immutable without creating a trust system or native ceremony. It is not
selected. A later explicit human decision must amend HCM-0.6, freeze the exact
successor ref/fingerprint and compatibility boundary, and rebaseline the
P2/P6/P7/Phase 2 gates.

## Review and proof record

The same stable causal budget declares three ordered review stages:

1. Option C containment planning;
2. product-decision analysis; and
3. final integrated closeout.

Each stage uses one frozen subject and one discovery lineage. Valid P1/P2
findings are consolidated and remediated once, followed by a different-fresh
closure review when remediation is material.

Review results before final integration:

| Stage | Dispatch | Result |
| --- | --- | --- |
| Option C containment planning | `20260728T134152Z--HCM-2-4--p2-option-c-containment-review` | CLEAN; no P1–P4 findings |
| Product-decision analysis discovery | `20260728T135058Z--HCM-2-4--p2-product-decision-reassessment-review` | one P2, `PRODUCT-STATUS-001`, for overbroad always-indeterminate wording |
| Product-decision analysis closure | `20260728T140357Z--HCM-2-4--p2-product-decision-reassessment-closure-review` | different-fresh CLEAN; `PRODUCT-STATUS-001` closed; no unmasked findings |

Focused proof passed:

- CLI setup/doctor typed-row, JSON, and exit-policy path: 1 passed;
- engine profile selection/decision/inspection wall: 25 passed, 0 failed; and
- exact source-order assertion in
  `crates/compiler/src/profile_readiness.rs::classify_readiness`: the
  `Invalid` return precedes the `Indeterminate` return.

The focused CLI test proves the missing-path `INDETERMINATE` rendering and
nonzero exit; it does not prove the full readiness precedence. The relevant
invalid-path compiler/CLI runtime cases are Unix-gated and produced zero
runnable cases on this Windows host, so they are not counted as passing
runtime proof here.

The final integrated review dispatch, verdict, final subject fingerprint,
GitNexus change detection, and commit IDs are recorded in the parent-owned
closeout handoff. They are not self-recorded here after review because that
would mutate the frozen subject.
