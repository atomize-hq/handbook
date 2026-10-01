# HCM coverage and landing research

Status: research and synthesis complete; independently VALIDATED. Proposed
planning input for human review, not implementation authority.

Baseline: `b0e73fc54f28d80c3f8f58d37b939db41bbbf0e4`, branch
`feat/handbook-contract-membrane-clean`. Research date: 2026-10-01.
Product and approved semantic files are held at that baseline during this research.
Reports added here do not change those authorities or certify runtime conformance.

## Assignment and authority

Compare the approved semantic model, current source/tests, and remaining membrane
plans. Produce one compact proposed coverage-and-landing map with traceable
supporting evidence. Preserve completed phase/slice history. Do not define every
future packet, select deferred product policies/APIs, or implement product changes.

The user approved this research structure and explicitly requires human review of
each later implementation increment before progression. Semantic approval does not
authorize implementation. Research agents may write only their assigned reports;
the coordinator owns synthesis, navigation, review remediation, and commits.

Sources, relative to the repository root:

| Role | Sources and use |
| --- | --- |
| Approved semantics | `docs/specs/handbook-contract-membrane/semantic-model/README.md`, its four owning chapters, and `review-decision-register.md`. The chapters own rules; the guide explains them; HR-001 through HR-006 are resolved. |
| Current product truth | Relevant Rust source, tests, Cargo manifests, public exports, declarative assets under `core/`, schema/catalog/operation manifests. Read end-to-end paths needed for a claim. |
| Current program direction | HCM `README.md`, `phase-and-slice-map.md`, `architecture-context.md`, `seam-crosswalk.md`, `implemented-baseline.md`; verify inventory/implementation claims against source. |
| Delivery rules | HCM lifecycle, review policy, prompts, and local routing skill. Automatic implementation continuation is overridden by the user's supervised delivery requirement. This study does not amend routing policy. |
| Completed delivery evidence | Relevant HCM-4.x/5.x slice authorities and packet records; historical closeout is not a fresh runtime check. |
| Decision provenance | Focused research records, adopted historical sections, or earlier user corrections only where needed. Pre-reset documents and idea documents are lineage, not current execution authority. |

Known corrections to preserve:

- Dock integrates established tools/formats with Handbook's contract/evidence model.
  The first implemented integration is Rust `jsonschema` validation. General hostile-
  code confinement is not an assumed remaining program obligation. Commit `b0e73fc`
  corrected three stale passages and received clean independent review.
- HCM-4.6 grounding is completed; earlier unavailable/inert descriptions cannot be
  copied forward. External triggers/adopter placement remain separate.
- HCM-5.2 completed Contracts-owned local-closeout truth but no parent eligibility.
  Approved Work Progression ownership describes the target, not past delivery.
- Existing V1 representation is evidence, not a compatibility obligation imposed on
  the greenfield semantic model. Do not invent migration bridges or dual behavior.
- No general IAM platform is selected. Policy-permitted advisory instruction can
  authorize effects while exact eligibility, currentness, and commit checks remain.

## Report responsibilities

| Report | Primary coverage |
| --- | --- |
| [future-obligations.md](future-obligations.md) | Active future outcomes/dependencies, unresolved choices, superseded history, and delivery-policy mismatch. |
| [rules-and-definitions.md](rules-and-definitions.md) | All Objects/governance sections: admission, permitted use, ontology, authored families/relationships. |
| [starting-evaluations.md](starting-evaluations.md) | All Assurance/preparation sections, including posture, both requirements sources, authorization, capability/readiness, prospective formation. |
| [collection-and-evidence.md](collection-and-evidence.md) | Evidence chapter's identity/evidence-closure section: occurrence, pruning, commitment, later checks, attempts, frontier, evidence set, final context, replay/currentness. |
| [results-and-explanations.md](results-and-explanations.md) | Evidence chapter's claims/metrics/gates/explanation section. Consumes collection closure rather than redefining it. |
| [reliance-and-actions.md](reliance-and-actions.md) | All Operational reliance/actions sections: governing completion, determinations, work progression, effects, receipts, posture feedback. |
| [lifecycle-cross-check.md](lifecycle-cross-check.md) | Cross-report lifecycle, ownership, identity, currentness, refusal, persistence, replay and boundary consistency. |
| [future-interface-cross-check.md](future-interface-cross-check.md) | Cross-report compatibility with remaining consumers/adapters, public interfaces, and future obligations. |

Every owning semantic subsection, including scenario corpora and settled/deferred
sections, must have an explicit disposition in its primary report. Supporting
reports may reference the primary claim rather than repeat it. Guide and HR topics
must resolve to owning-section claims. Identity, currentness, refusal, persistence,
and public interfaces must be assigned within applicable areas.

Each report includes:

1. Baseline/scope and sections read (a compact section-to-claim coverage table).
2. Stable report-local claim IDs; precise behavioral obligation; semantic citation.
3. Coverage: **already covered**, **needs changing**, **missing**, or **unverified**.
   Split compound claims where support differs. Existing machinery is not full
   conformance merely because its names resemble approved objects.
4. Implementation citations, including source, export/manifest and test coverage
   where applicable. Negative claims state which plausible owners/paths were
   inspected; a failed keyword search is not sufficient proof of absence.
5. Future relationship: fits planned direction, change needed, or decision-dependent,
   with supporting current plan reference and the boundary that must be preserved.
6. Dependencies, unresolved questions, and bounded implications for landing order.
7. Verification performed and limitations. Distinguish test source inspected,
   historical test outcomes, and commands actually run. No omnibus runtime suite
   is required for this source-grounded research.

Use repository-relative Markdown links with semantic anchors and precise source
locations in prose (or line fragments) so the commit baseline fixes their meaning.
For recovered decisions, identify the record/commit/task and relevant passage.
Do not treat another report as sole evidence for a material claim.

## Evidence limits and completion

GitNexus MCP tools are not exposed in this session. The local CLI status attempt
failed while hashing its cached analyzer input; prior detection also warned of a
foreign-worktree registry entry. Local metadata records index baseline `a35297d`
(2026-08-25), before HCM-4.6. The future-obligations concept query also returned
degraded FTS errors and no usable graph evidence. Agents should attempt concept query via the CLI
where useful and record any failure, then inspect source/exports/tests directly.
Do not rely on a wrong/stale graph, reindex or change product files during research.
The coordinator retains the required pre-commit change-detection gate.

Cross-area passes reconcile evidence before synthesis. One fresh controlling
reviewer then checks the proposed map and primary-source support. Valid P1/P2
findings require remediation and review of the changed subject; P3/P4 remain
advisory. The final proposed map returns to the user before implementation planning.

This directory is a source-backed research trail requested by the user, not a new
orchestration ledger or mandatory artifact scheme for later packets.

## Synthesis and review subject

The [proposed compact map](../../specs/handbook-contract-membrane/coverage-and-landing-map.md)
is the routine reading surface. The six primary reports above are its evidence
trail; the two cross-area reports reconcile their lifecycle and future-interface
claims. Stable IDs identify propositions, not backlog items or selected packets.

Coordinator corrections before final review:

- R-02/E5 follows the actual promoted Context Resolution authority capsule;
  category-2 definitions and category-3 private authority lineage are distinct.
  The entire capsule is not proven to be one category-2 definition.
- C-14 distinguishes no automatic numeric value/pass/completion from the exact
  admitted policies' N/A meanings. Historical Binding checks apply to consumed
  roles whose closure binds one; F-11 uses the same qualified rule. T-22 likewise
  distinguishes what pruning establishes from policy-defined result states.
- The map requires each selected behavior's actual public consumer boundary,
  not universal SDK/CLI/Tauri parity or a public operation for every object.

The final review subject is the map, this index/navigation, all primary and
cross-area reports, and their primary-source support. No Rust source, approved
semantic chapter, phase status, or general execution-routing policy is changed.
The user-specific stop-after-packet rule is explicit in the map; conflicting
general routing must be reconciled before later product dispatch.

Cross-area reconciliation mapped all 12 guide decisions, HR-001–HR-006, and 167
owning scenario rows/examples across the primary reports, with no omitted owning
subsection found. This establishes research coverage, not implemented conformance.
Future reconciliation covers F-01–F-19 and all five Phase-6 outcomes. Current local
foundations, target gaps, deliberate choices and external evidence remain distinct.

The [fresh controlling review](final-review.md) returned **VALIDATED**, with no
unresolved P1/P2 finding. Its one P4 advisory was addressed by clarifying the
collection overview's time/comparison wording; UTC technology itself is not
prohibited. That editorial clarification changes no behavior, coverage label or
authority boundary and requires no new causal round. Human review of the proposed
map remains the next boundary. No implementation or successor dispatch is authorized.
