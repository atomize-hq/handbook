# HCM-3.6 future conformance and refusal proof matrix

## Planning-only status

Every row below is future implementation proof. The row does not claim a test
exists, a runtime path has landed, or `PG-POSTURE-01`/`PG-POSTURE-02` is green.
Each future test must call the owner-local typed boundary with exact pairs and
an explicit evaluation instant/basis. Result labels name the required future
typed outcome, not present behavior.

| ID | Future input / conformance case | Future owner and typed result | Required proof and fail-closed assertion |
|---|---|---|---|
| P01 | Identical exact source pairs and freshness basis are resolved twice. | `resolve_project_posture_kernel` returns identical `ProjectPostureKernel` input/kernel fingerprints and global dimensions. | Deterministic fixture pairs; IDs and presentation timestamps excluded; no second editable authority. |
| P02 | Profile, Charter, condition, policy, trigger, contract, evidence, or snapshot bytes change behind a stable ref. | Resolver/evaluator returns `Refused { stale_or_fingerprint_mismatched_input }` until cited fingerprint changes. | Mutate one byte per pair; prove no old kernel/recommendation reuse and no hidden source read. |
| P03 | Same freshness-qualified sources are evaluated on opposite sides of expiry. | New `FreshnessEvaluationBasis` yields different basis/output identity or typed stale/unknown refusal. | Explicit before/after instants; prove ambient resolution time is not semantic input. |
| P04 | A current hard trigger occurs; accumulated rule is tested below and at exact threshold/window/count/currentness. | Evaluator returns one evidence-linked `Recommended` for hard/current satisfied rule, otherwise `NoRecommendation` or typed `Refused`. | Bind exact trigger or policy-local rule pair and evidence; no synthetic trigger registry. |
| P05 | One event affects two global engineering dimensions; causal scope varies independently. | Two immutable `PostureRecommendation` records, one transition each. | Assert scope is evidence metadata only; no scoped posture state/path or combined multi-transition record. |
| P06 | Hard-trigger raise; lower without sustained evidence/cooldown; lower across floor/red line. | Raise may be `Recommended`; insufficient/cooldown/floor/red-line lower is `NoRecommendation` or `Refused`. | Test configured window/cooldown and no silent lower fallback. |
| P07 | Recommendation, notification, acknowledgement, advisory score, packet/handoff completion, or local task completion is supplied without transition. | Recommendation/event remains advisory; no Charter/override/gate/promotion mutation outcome is possible. | No-write assertion across Charter, override, gate, promotion, packet, and handoff authority. |
| P08 | HCM-3.4/HCM-3.5 evidence is missing, stale, redacted-only, incompatible, unresolved, malformed, duplicate, or insufficient. | Resolver/evaluator returns exact `PostureRefusalKind` or `NoRecommendation`. | Assert no hidden/raw snapshot or full delta payload is read and no recommendation/promotion is created. |
| P09 | Transition is unapproved, stale, wrong actor/class, scoped-path, multi-change, non-`replace`, reassessment-failing, or CAS-mismatched. | `apply_posture_transition` returns typed invalid/refused transition. | Atomic no-write assertion for canonical Charter and transition record. |
| P10 | Exactly one approved, current constitutional-root transition meets all checks. | `PostureTransitionOutcome` writes exactly Charter plus immutable transition record atomically, then returns stated resulting-kernel pair. | Compare-and-write and crash/failure tests prove one mapped `replace`, no extra field, and re-resolution. |
| P11 | Adapter delivery/acknowledgement observes policy-bound recipients/deadline, delivery failure, duplicate/wrong recipient, and escalation. | Typed notification/append-only acknowledgement disposition or policy-bound escalation intent; never transition. | Assert adapters cannot self-approve, alter recipient/deadline, change recommendation, or bypass escalation semantics. |
| P12 | The public/private owner chain is inspected with all potential consumers. | Architecture proof shows acyclic `handbook-engine` semantic ownership. | Prove Flow, pipeline, SDK, CLI, Tauri, Substrate, HCM-5 gate runtime, and adapters are not posture semantic owners. |

## Gate mapping and review requirements

| Gate | Matrix coverage | Planning acceptance |
|---|---|---|
| `PG-POSTURE-01` | P01–P03, P05, P08 | Exact pair/currentness/freshness-derived kernel identity and one-authority boundary are fully planned. |
| `PG-POSTURE-02` | P04–P12 | Hard/accumulated advisory evaluation, notification/acknowledgement, hysteresis, authorized transition, and owner boundaries are fully planned. |

Future implementation cannot call either gate closed until owner-local positive
and fail-closed tests, atomic-transition proof, real consumer-boundary proof,
and fresh independent review satisfy the then-selected packet. This HCM-3.6
planning increment proves only that the required proof exists as a specified
obligation.
