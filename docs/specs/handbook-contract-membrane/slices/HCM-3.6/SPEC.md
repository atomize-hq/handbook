# HCM-3.6-PLANNING — Project posture resolution and recommendation loop

**Packet ID:** `HCM-3.6-PLANNING`
**Status:** planning authority only; no product implementation is authorized
**Integrated outcome:** `hcm-3-6-project-posture-planning`

## Authority and boundary

This packet is the Phase-3 `HCM-3.6` planning authority from the row in
[`04-phase-slice-map.md`](../../04-phase-slice-map.md), the posture sections of
`01`, `02`, and `05`, the `PG-POSTURE-01` and `PG-POSTURE-02` rows in `06`,
and the Project posture kernel seam in `03`. It records an implementation-ready
future owner/call-path contract without authorizing Rust, schemas, generated
artifacts, dependencies, configuration, SDK, CLI, Tauri, Substrate, Flow,
pipeline, gates, release, or remote work.

The completed HCM-3.4 source-pair handoff
`20260806T191500Z--HCM-3-4--orchestration--private-projection-source-pair-implementation-completed`
at `cc6d84926e6435c3960af86e5927a18991dd04fd`, and the completed HCM-3.5
audit-corrective handoff
`20260807T213000Z--HCM-3-5--orchestration--audit-corrective-source-ingestion-completed`
at reviewed commit `50b16e30b6b5ffd1d3b12d54f395b673c38359f2`, are read-only
input/adoption boundaries. They do not authorize HCM-3.6 implementation,
reopen their causal lineage, or authorize HCM-3.5 P5/P6.

Canonical Charter YAML remains the only editable constitutional authority.
`ProjectPostureKernel` is derived, never a second editable truth. Existing
canonical `00`–`06`, `09`, historical handoffs, and HCM-3.4/HCM-3.5 subject
files are read-only context. A contradiction in them is an authority blocker;
this packet must not repair it.

## Frozen future owner and call paths

1. `handbook-engine` is the sole future semantic owner. A purpose-named
   `project_posture` engine module owns exact input resolution, currentness,
   normalization, fingerprinting, global-dimension derivation, policy
   evaluation, recommendation identity, and transition admission. It may use
   canonical Charter, profile-selection, condition, freshness,
   artifact-lineage, and approval/authority transaction capabilities only as
   internal dependencies. Existing Flow resolution, grounding, HCM-3.4 private
   snapshot types, and HCM-3.5 views never own posture semantics.
2. The future read path is
   `ProjectPostureKernelRequest` → exact canonical
   Charter/profile/approved-override/condition/contract/evidence/snapshot
   references → explicit `FreshnessEvaluationBasis` →
   `resolve_project_posture_kernel` → `ProjectPostureKernel` →
   `evaluate_posture_recommendation` →
   `PostureEvaluationOutcome::{NoRecommendation, Recommended, Refused}`.
   Snapshot and delta signals are descriptive evidence only; the engine
   resolves exact references and fingerprints before use.
3. The future mutation path is separate and narrow: a reviewed
   `PostureRecommendation` plus exact approval, reassessment, and current
   target fingerprint → `apply_posture_transition` → one compare-and-write
   update of constitutional Charter → immutable `PostureTransition` semantic
   record → re-resolved kernel. Evaluator, notification adapter,
   acknowledgement writer, Flow, pipeline, CLI, Tauri, SDK, Substrate, gate,
   and recommendation cannot mutate Charter.
4. Future adapters receive only typed recommendation and acknowledgement
   intent/outcome DTOs. They may deliver a notification or collect an
   acknowledgement; they cannot choose recipients, alter acknowledgement or
   escalation deadlines, approve a transition, reinterpret evidence, or turn
   delivery into policy authority. No adapter is selected here.
5. HCM-5 remains sole future owner of contract-gate evaluation and parent
   promotion. Recommendation, local completion, advisory score, packet result,
   handoff, or acknowledgement never yields a gate verdict or parent promotion.

## Frozen internal Rust boundary

These future internal names are implementation-ready selections, not authority
to change source, public APIs, schemas, or dependencies.

```text
resolve_project_posture_kernel(
  repo_root: &Path,
  request: ProjectPostureKernelRequest,
) -> Result<ProjectPostureKernel, PostureOperationError>

evaluate_posture_recommendation(
  repo_root: &Path,
  request: PostureEvaluationRequest,
) -> Result<PostureEvaluationOutcome, PostureOperationError>

apply_posture_transition(
  repo_root: &Path,
  request: ApprovedPostureTransitionRequest,
) -> Result<PostureTransitionOutcome, PostureOperationError>
```

### Exact input and kernel records

`ExactPostureRef` is an opaque validated `{ref, fingerprint}` pair. Every
constructor rejects blank, malformed, mutable-only, ambiguous, or
non-canonical references. `ProjectPostureKernelRequest` contains the exact
Charter pair, selected profile pair with `resolved_profile_fingerprint`,
zero-or-more approved override/condition/contract/evidence/snapshot pairs, and
an explicit `FreshnessEvaluationBasis` pair whenever an input is
freshness-qualified.

`ProjectPostureKernel` has the exact semantic shape frozen by `05`:

- one constitutional artifact pair and one profile pair;
- typed override, condition, contract, evidence, and snapshot input lists;
- an optional freshness pair only when permitted;
- global `EngineeringPostureDimension` entries with level, floor, red-line,
  trigger, shortcut, and proof references;
- applicability, omission, and unresolved-condition explanation references;
- recommendation references, `input_fingerprint`, and `kernel_fingerprint`.

`kernel_id`, recommendation references, and `resolved_at_utc` are excluded
from semantic fingerprint input as prescribed by `05`. All semantic lists with
no ordering meaning are stably sorted before hashing. Changed bytes behind a
reference without a corresponding fingerprint, missing profile currentness,
unknown freshness, absent required evidence, or unresolved required condition
refuses fail-closed.

### Trigger, policy, recommendation, and event records

`PostureTriggerDefinition` is an immutable hard-trigger record with exact
signal/evidence/freshness pairs, one global dimension, one causal scope, one
transition, and one fingerprint. `AccumulatedSignalRule` is local to an exact
`PostureEvaluationPolicy` version and has exact threshold, window, count,
comparator, transition, guidance, and fingerprint.

`PostureEvaluationPolicy` owns ordered hard-trigger inputs, accumulated rules,
lowering evidence window, cooldown, recipients, acknowledgement-required flag,
escalation deadline, extensions, and policy fingerprint. Ambient trigger
registries, wall clock, delivery defaults, and model inference do not
participate in a result.

`PostureRecommendation` is immutable and advisory. It binds one kernel pair,
one policy pair, exactly one global-dimension transition, causal scope,
mutually exclusive hard-trigger or accumulated-rule identity, exact evidence
and snapshot-delta pairs, confidence, urgency, approval requirement,
notification intent, suggested actions, `recommendation_only` eligibility, and
its fingerprint. One event affecting two dimensions yields two
recommendations; grouping is presentation only.

`PostureNotificationIntent` copies exact recipient, acknowledgement, and
escalation semantics from a recommendation. An append-only
`PostureAcknowledgement` records recommendation pair, recipient, observed
delivery/acknowledgement/escalation disposition, and timestamp. Neither record
changes a recommendation nor supplies approval. A missing required
acknowledgement may produce only the policy-bound escalation intent.

### Transition record and closed outcome algebra

`PostureTransition` binds exact recommendation/kernel/target Charter pairs,
`constitutional_root` class, one `replace` at the mapped global-dimension path,
expected and proposed levels, approval pairs and authorized actor, targeted
Charter-intake reassessment, resulting authority/kernel pairs, and transition
fingerprint. Overrides are read-only v1 inputs and are invalid targets.

`PostureEvaluationOutcome` is closed:

```text
NoRecommendation { reason }
Recommended { recommendation, notification_intent }
Refused { refusal }
```

`PostureRefusalKind` distinguishes malformed reference;
unresolved/missing/unknown condition; stale or fingerprint-mismatched input;
insufficient or expired evidence; incompatible policy/trigger;
threshold/window/count miss; cooldown; hysteresis/floor/red-line breach;
multi-dimension or scope-path attempt; acknowledgement-policy mismatch;
unauthorized actor; stale compare-and-write target; failed reassessment; and
invalid transition. `PostureOperationError` is reserved for repository, I/O,
or atomic-operation integrity failure, never an expected semantic refusal.

## Constitutional, evidence, and mutation rules

`EngineeringPostureDimension` and `PostureLevel` are global engineering
rigor/guardrail state, never mappings onto the six Context Resolution
dimensions. `causal_scope_ref` explains evidence scope only; it is not a
scoped posture coordinate, mutation path, or authority selector.

HCM-3.4 Snapshot Memory and HCM-3.5 grounding/delta-summary outputs may be
used only as exact, current, redacted evidence pairs. They must not supply raw
snapshot payload, full delta signals, hidden data, or promotion claims.

A matching hard trigger may recommend an immediate raise only while the exact
policy/trigger/evidence/freshness closure is current. Lowering needs its
configured sustained evidence window and cooldown and cannot cross the
effective floor or red line. Stale or insufficient evidence yields a typed
refusal or no-recommendation, never a silent lower level.

Only `apply_posture_transition` with current exact approval and target
fingerprints can mutate canonical Charter. It revalidates mapped intake
coverage, enforces floors/red lines/hysteresis, atomically compares expected
level and target fingerprint, writes the immutable transition record in the
same atomic group, then re-resolves the kernel. A failure writes no canonical
policy change and returns the typed refusal or operation error.

## Future implementation packets and test seams

The parent slice stays documentation-only. A later separately authorized
implementation may use these bounded packets in order:

1. **Kernel resolution:** implement exact-reference validation, source/profile
   currentness, explicit freshness basis, normalization, global dimensions, and
   deterministic kernel fingerprints in `handbook-engine::project_posture`.
2. **Evaluation:** implement hard and accumulated policy evaluation, immutable
   recommendations, typed notification intent, no-recommendation, and the
   closed refusal algebra in the same owner.
3. **Transition:** implement the sole constitutional-root compare-and-write,
   mapped reassessment, immutable transition record, and re-resolved kernel;
   overrides remain invalid targets.
4. **Adapters and gate separation:** prove adapter DTO delivery/acknowledgement
   boundaries without selecting an adapter or permitting HCM-5 gate/promotion
   behavior.

Future tests must be owner-local first and prove the `P01`–`P12` matrix in
`proof/20260807T000000Z--hcm-3-6-project-posture-planning-proof-matrix.md`.
They test only explicit source pairs and evaluation instants; neither ambient
clock nor current Flow/pipeline/SDK/CLI/Tauri/Substrate behavior is authority.

## Explicit non-goals and stops

Do not implement HCM-3.6; alter an existing Rust symbol; edit product tests or
fixtures; change Charter content; select a production schema; emit a public
DTO/JSON schema; expose HCM-3.4 private types; add HCM-3.5 P5/P6; claim
Snapshot/grounding adoption; build HCM-5; choose Phase-3 exit; start HCM-4+;
or touch protected/remote state.

Stop at the authority boundary if a canonical-source contradiction, missing
predecessor/currentness proof, product/runtime/schema/dependency/public-owner
need, non-planning surface, unavailable mandatory built-in delegation, or
unresolvable P1/P2 requires work beyond this packet.
