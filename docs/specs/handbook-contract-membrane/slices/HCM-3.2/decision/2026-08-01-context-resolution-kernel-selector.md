# HCM-3.2 Context Resolution Kernel Selector

Status: amended for same-parent Option 2 resumption; fresh implementation-stage CLEAN required before RED/Rust
Selected slice: HCM-3.2 only
Parent orchestration: `20260801T202515Z--HCM-3-2--context-resolution-kernel`

## Selection and predecessor boundary

The user explicitly selected the complete HCM-3.2 slice. HCM-3.1 is completed
predecessor context and its vocabulary/profile closure is immutable. This
selector does not select HCM-3.3 or any later work.

## Orchestration registry

The parent has one integrated outcome and three packets:

```json
[{"authority_ref":"docs/specs/handbook-contract-membrane/slices/HCM-3.2/decision/2026-08-01-context-resolution-kernel-selector.md","integrated_outcome_id":"hcm-3.2-context-resolution-kernel-full-slice","packet_ids":["HCM-3.2-P1-planning-selector","HCM-3.2-P2-kernel-implementation","HCM-3.2-P3-proof-control-closeout"]}]
```

The canonical JSON plus one terminal LF is the frozen outcome registry preimage.
Every v1.4 dispatch binds its recomputed registry fingerprint and the causal
budget derived from this parent orchestration plus the integrated outcome.

## Selected owner and call boundary

`handbook-engine` is the sole runtime owner. The new kernel consumes the exact
`ResolvedInstanceProfile::context_resolution()` selected by the existing
profile path and the profile's exact ref/fingerprint. It produces immutable
engine values only. No flow, pipeline, compiler, CLI, SDK, Tauri, Substrate,
HTTP, dock, or publication consumer is added.

Authority-bearing calls additionally consume opaque admissions from the exact
HCM-3.2 resolver selected below. The selector never substitutes caller
coherence for authenticated current authority.

## Authority-binding and admission selector

The only new authority source is one profile-selected canonical generic
artifact read under `ArtifactRepositoryAuthorityGuardV1` and
`GenericArtifactLineageStoreV1` recovery/currentness. The record is closed:

```yaml
schema_id: handbook.context-resolution-authority-binding
schema_version: "1.0"
binding_id: handbook.context-resolution-authority.example
binding_version: "1.0.0"
repository_identity_fingerprint: sha256:...
resolved_profile: { ref: handbook.profile.example@1.0.0, fingerprint: sha256:... }
resolution_stack: { ref: handbook.context-resolution.example@1.0.0, fingerprint: sha256:... }
approver_registry:
  state_ref: registry-state.example
  state_fingerprint: sha256:...
  head_transition_ref: registry-transition.example
  head_transition_fingerprint: sha256:...
authority_mappings:
  root_creation: { approval_class: context_resolution_root, authority_ref: work.root.owner }
  parent: { approval_class: context_resolution_parent, authority_ref: work.parent.owner }
  requested: { approval_class: context_resolution_requested, authority_ref: work.request.owner }
  approving: { approval_class: context_resolution_approving, authority_ref: work.approver }
  decision: { approval_class: context_resolution_decision, authority_ref: decision.owner }
  evidence: { approval_class: context_resolution_evidence, authority_ref: evidence.owner }
  trigger_condition: { approval_class: context_resolution_trigger, authority_ref: trigger.owner }
  constraint: { approval_class: context_resolution_constraint, authority_ref: constraint.owner }
  source: { approval_class: context_resolution_source, authority_ref: source.owner }
  target: { approval_class: context_resolution_target, authority_ref: target.owner }
  target_memory: { approval_class: context_resolution_target_memory, authority_ref: memory.owner }
candidate_mappings:
  dimension_rank_increase:
    trigger: { ref: trigger.dimension-rank-increase@1.0.0, fingerprint: sha256:... }
    missing_condition: one_or_more_dimension_ranks_exceed_parent
    evidence_requirement: current_and_proposed_envelopes
  mutation_allow_expansion:
    trigger: { ref: trigger.mutation-allow-expansion@1.0.0, fingerprint: sha256:... }
    missing_condition: child_allow_not_provably_contained
    evidence_requirement: parent_and_child_mutation_rules
  missing_context:
    trigger: { ref: trigger.missing-context@1.0.0, fingerprint: sha256:... }
    missing_condition: required_context_record_absent
    evidence_requirement: available_context_and_missing_ref
  missing_authority:
    trigger: { ref: trigger.missing-authority@1.0.0, fingerprint: sha256:... }
    missing_condition: required_authority_mapping_absent
    evidence_requirement: current_authority_and_requested_pair
extensions: {}
binding_fingerprint: sha256:...
```

The selected artifact schema may be repository-defined, but this parser and
semantic contract are HCM-3.2-specific and closed. No schema-catalog entry or
new shipped definition is created. Canonical artifact admission supplies the
exact raw-byte fingerprint; `binding_fingerprint` is RFC-8785/SHA-256 over the
complete semantic record excluding itself. Both must match. The record must
pin the exact repository identity, operation-context profile, supplied stack,
and current committed approver registry state/head. Missing, extra, duplicate,
stale, noncanonical, mismatched, or changed bytes refuse.

For an authority use, owner lookup takes its one exact
`ApproverAuthorityPairV1`, sorts active non-exhausted ES256 credentials that
cover it through `select_eligible_credentials`, and gives that exact allow-list
to `NativeAuthenticatorPortV1::get_assertion`. Before every uncached call the
resolver fills a fresh `[u8; 32]` nonce with the engine's existing direct
`getrandom` dependency; entropy failure refuses. The SHA-256 client-data
challenge domain-separates and length-prefixes that nonce, repository identity,
registry state/head, binding artifact and semantic fingerprints, use string,
and exact subject ref/fingerprint. `decode_and_verify_get_assertion_response`
must select exactly one requested credential, validate RP/user-presence/user-
verification, verify ES256, and satisfy the existing counter rule. Zero
candidates, authenticator refusal, multiple/unknown response selection, stale
currentness, entropy failure, counter rollback, or any verification error
refuses with no admission.

The resolver caches successful admissions by `(authority_use, subject_ref)`.
Exact fingerprint replay inside one resolver is idempotent and returns the same
admission; changed fingerprint reuse refuses. A fresh resolver always allocates
a new nonce before authentication, so a captured response cannot replay across
resolver lifetimes even when both retained and asserted counters are zero.
Nonzero sign counters must additionally increase during one resolver lifetime;
zero/zero remains the existing accepted authenticator behavior. This is not
enrollment or durable credential-use mutation.
`ApproverAuthorityPairV1`, the authenticator module, repository guard, generic
lineage store, artifact mutation owner, and all registry bytes remain
unchanged.

## Definition identity selector

Preserve byte-for-byte and do not version:

- shipped stack ref/fingerprint
  `handbook.context-resolution.shipped-root@1.0.0` /
  `sha256:9e95fdef90b98e28acb60bfe96a72f122418b56b1531eafe8ab6cff3eb7668b4`;
- matcher ref/fingerprint `handbook.mutation-matcher.core@1.0.0` /
  `sha256:be585006043ca46096e85ad76f20a87439e6984bcace997924982f0247aa29f4`;
- escalation ref/fingerprint `handbook.resolution-escalation.core@1.0.0` /
  `sha256:1f7e04ed7d8a68f338b2c421db8ef0c49f16d9b4dcb001145083414efa408121`;
- memory-promotion ref/fingerprint `handbook.memory-promotion.core@1.0.0` /
  `sha256:1f0e07938432a159d2ba9c131ed6dc28536ec8aa30b817f24066e246c06c0bca`.

General stack definitions use the same schema/policies and uniform exact
identity algorithm. They are test/repository definitions, not new shipped
defaults. Same ref with different bytes, stale dependency fingerprints,
range/latest lookup, remote loading, and executable hooks refuse.

## Ordered-stack and six-dimension selector

The list is broad-to-narrow, non-empty, and linear. Stable level IDs are unique;
display labels are presentation only and may repeat. Exactly six non-empty
domains exist. Each has unique value IDs and contiguous ranks from zero, where
zero grants least reach/disclosure/durability/claim authority. Every level has
all six defaults, and each adjacent narrower level may preserve or lower each
rank but never raise one.

The active level selects a named default position only. It is not an authority
score. Envelope dimensions remain complete and independently comparable, so
two envelopes at one level may differ in any subset of dimensions.

## Inheritance selector

Root envelopes have no parent. Children cite exactly one supplied parent by
ref/fingerprint and repeat complete effective state. Parent/profile/stack
identity is equality-checked. Every child rank must be less than or equal to
the corresponding parent rank. Any increase returns no envelope and one typed
`dimension_rank_increase` escalation candidate naming the changed dimensions.

Candidate construction uses only the four exact binding-record mappings above.
The requested authority is the admitted `requested` pair. Dimension and
mutation candidates bind the actual current/proposed envelope inputs; missing
context/authority candidates require caller-supplied exact evidence satisfying
their frozen requirement. No other candidate class or free-form trigger is
admitted.

Constraint inputs are exact sorted ref/fingerprint pairs and cannot become
merge parents. Lower-horizon observations cannot mutate higher authority.

## Mutation selector

Only `repository_path` is admitted. The exact core grammar and byte/segment
limits are enforced before evaluation. Matching is case-sensitive. Root grants
are allow-minus-deny; child grants are parent-intersection-child-minus-all-deny.
Empty allows deny all. Deny always wins a valid overlap.

Containment, if later authorized, compares the positive parent allow language;
inherited denies are accumulated separately and always subtract from the
result. A deny hole does not make an otherwise equal child allow an expansion,
and a child can never remove or weaken that hole. Child allow containment is
proved under the closed grammar. Exact equality,
literal specialization of a parent segment wildcard, and a child path/pattern
beneath a matching parent terminal `/**` prefix are admitted. A child terminal
recursive selector is admitted only beneath an equal or broader parent terminal
recursive selector. Any ambiguous pattern-language containment refuses as
indeterminate; it never guesses or grants. Possible expansion returns no child
and a typed `mutation_allow_expansion` candidate.

Malformed/unresolvable selector or target, unknown target kind, stale matcher,
or evaluation indeterminacy refuses. The matcher reads no filesystem state.

## Memory, validation, and escalation selector

Memory/validation compare requested and envelope ranks in their own domains.
At-or-below is authorized. Higher memory returns promotion-required; higher
validation returns not-authorized. Neither result mutates state or creates
green proof.

Escalation and promotion requests/dispositions are separate immutable records.
The registry admits one terminal disposition per exact request and rejects
duplicate, stale, changed-ID, invalid-outcome, or forbidden-authority records.
Requests have no preapproval effect. Approved escalation cites an exact
replacement envelope; applied promotion cites an exact new semantic-memory
result after compare-and-write validation. Other outcomes cite no result.

The registry is a pure semantic validator only. Durable storage, operation
catalogs, restart discovery, and adapter delivery are not selected.

The exact admission subjects are:

| use | one exact admitted subject |
|---|---|
| `root_creation` | proposed root envelope candidate binding |
| `parent` | proposed child envelope candidate binding, never the reusable parent binding |
| `requested` | exact escalation or promotion request binding |
| `approving` | exact terminal disposition binding |
| `decision` | the exact decision binding cited by that disposition |
| `evidence` | each exact escalation evidence or promotion validation-evidence binding |
| `trigger_condition` | exact trigger binding from the selected candidate mapping |
| `constraint` | each exact envelope constraint binding |
| `source` | each promotion source-input binding and, separately, the exact source-envelope binding |
| `target` | typed semantic-memory target candidate binding |
| `target_memory` | exact promotion request binding at request admission; typed result-record binding for `applied`, otherwise exact request binding |

The transition registry enforces the table and exact cardinality/order. Root
and child envelope resolvers enforce their rows directly. Escalation request
admission takes `requested`, `trigger_condition`, and one `evidence` per record.
Escalation disposition admission takes `approving` and `decision`. Promotion
request admission takes `requested`, ordered `source` admissions for every
source input followed by the source envelope, `target`, and `target_memory`.
Promotion disposition admission takes `approving`, `decision`, one `evidence`
per validation record, and the outcome-specific `target_memory` subject.

No-candidate behavior is exact: successful resolution emits none; a proven
dimension increase or unprovable/expanding child allow emits exactly one of
the mapped candidates; explicit missing context/authority emits its mapped
candidate. Valid allow/deny overlap returns deny and no candidate. Malformed or
unknown input, missing mapping, stale bytes/currentness, ambiguous cardinality,
authenticator failure, replay mismatch, duplicate terminal disposition, and
indeterminate evaluation outside the one containment class refuse with no
candidate. Requests have no authority effect. Exact request/disposition replay
is idempotent; changed-ID replay and a second terminal disposition refuse.

## Compatibility selector

HCM-3.1 stable-role/vocabulary/profile bytes and behavior remain exact. The
shipped empty vocabulary and profile's selected stack closure replay unchanged.

L0-L3 pipeline input/filtering is not a public Context Resolution mapping and
is not edited. Existing positive/negative scoped-filtering tests must pass.
This deliberately preserves the useful precursor until HCM-3.5 selects its
real migration; HCM-3.2 neither removes it nor freezes it into kernel semantics.

## Exact public symbols and signatures

All fields are private. Argument names below are descriptive; the Rust types,
ownership, return types, generic bounds, and method names are exact. `Binding`
means `ContextResolutionExactBinding`, `Admission` means
`ContextResolutionAuthorityAdmission`, and `Error` means
`ContextResolutionKernelError`.

```rust
pub struct ContextResolutionExactBinding;
impl ContextResolutionExactBinding {
    pub fn new(reference: &str, fingerprint: &str) -> Result<Self, Error>;
    pub fn reference(&self) -> &str;
    pub fn fingerprint(&self) -> &str;
}

pub enum ContextResolutionAuthorityUse {
    RootCreation, Parent, Requested, Approving, Decision, Evidence,
    TriggerCondition, Constraint, Source, Target, TargetMemory,
}
impl ContextResolutionAuthorityUse { pub const fn as_str(self) -> &'static str; }

pub struct ContextResolutionAuthorityAdmission;
impl ContextResolutionAuthorityAdmission {
    pub fn authority_use(&self) -> ContextResolutionAuthorityUse;
    pub fn subject(&self) -> &Binding;
    pub fn binding_fingerprint(&self) -> &str;
}

pub struct ContextResolutionAuthorityAdmissionResolver;
impl ContextResolutionAuthorityAdmissionResolver {
    pub fn new(
        repo_root: impl AsRef<std::path::Path>, binding_kind_ref: &str,
        binding_instance_id: &str, expected_artifact_fingerprint: &str,
    ) -> Result<Self, Error>;
    pub fn admit<P: NativeAuthenticatorPortV1>(
        &mut self, authenticator: &mut P,
        authority_use: ContextResolutionAuthorityUse, subject: &Binding,
    ) -> Result<Admission, Error>;
}

pub struct ContextResolutionDimensions;
impl ContextResolutionDimensions {
    pub fn new(values: [&str; 6]) -> Result<Self, Error>;
    pub fn values(&self) -> [&str; 6];
}

pub enum ContextResolutionMutationEffect { Allow, Deny }
pub struct ContextResolutionMutationRule;
impl ContextResolutionMutationRule {
    pub fn new(
        rule_id: &str, effect: ContextResolutionMutationEffect,
        target_kind: &str, selector: &str,
    ) -> Result<Self, Error>;
}
pub enum ContextResolutionMutationDecision { Allowed, Denied, Indeterminate }
pub enum ContextResolutionMemoryDecision { Authorized, PromotionRequired }
pub enum ContextResolutionValidationDecision { Authorized, NotAuthorized }

pub struct ContextResolutionEnvelopeInput;
impl ContextResolutionEnvelopeInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        envelope_id: &str, objective_ref: &str, resolved_profile: Binding,
        resolution_stack: Binding, active_level_id: &str,
        dimensions: ContextResolutionDimensions, parent: Option<Binding>,
        constraint_inputs: Vec<Binding>,
        mutation_rules: Vec<ContextResolutionMutationRule>,
        escalation_triggers: Vec<Binding>,
    ) -> Result<Self, Error>;
    pub fn candidate_binding(&self) -> &Binding;
}

pub struct ContextResolutionEnvelope;
impl ContextResolutionEnvelope {
    pub fn resolve_root(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        input: ContextResolutionEnvelopeInput, creator: &Admission,
        constraints: &[Admission],
    ) -> Result<Self, Error>;
    pub fn resolve_child(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        parent: &Self, input: ContextResolutionEnvelopeInput,
        parent_authority: &Admission, constraints: &[Admission],
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
    pub fn dimensions(&self) -> &ContextResolutionDimensions;
    pub fn evaluate_mutation(
        &self, target_kind: &str, target: &str,
    ) -> ContextResolutionMutationDecision;
    pub fn authorize_memory(
        &self, requested_value: &str,
    ) -> Result<ContextResolutionMemoryDecision, Error>;
    pub fn authorize_validation(
        &self, requested_value: &str,
    ) -> Result<ContextResolutionValidationDecision, Error>;
    pub fn missing_context_candidate(
        &self, proposed: Binding, evidence: Vec<Binding>,
    ) -> Result<ContextResolutionEscalationCandidate, Error>;
    pub fn missing_authority_candidate(
        &self, proposed: Binding, evidence: Vec<Binding>,
    ) -> Result<ContextResolutionEscalationCandidate, Error>;
}

pub struct ContextResolutionEscalationCandidate;
impl ContextResolutionEscalationCandidate {
    pub fn class(&self) -> &str;
    pub fn trigger(&self) -> &Binding;
    pub fn missing_condition(&self) -> &str;
    pub fn requested_authority_ref(&self) -> &str;
    pub fn evidence(&self) -> &[Binding];
}

pub struct ContextResolutionEscalationRequest;
impl ContextResolutionEscalationRequest {
    pub fn new(
        request_id: &str, current_envelope: Binding,
        proposed_envelope: Binding,
        candidate: ContextResolutionEscalationCandidate,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}
pub enum ContextResolutionEscalationOutcome { Approved, Refused, Superseded }
pub struct ContextResolutionEscalationDisposition;
impl ContextResolutionEscalationDisposition {
    pub fn new(
        disposition_id: &str, request: Binding,
        outcome: ContextResolutionEscalationOutcome, decision: Binding,
        authorized_envelope: Option<Binding>,
        superseding_request: Option<Binding>,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}

pub struct ContextResolutionPromotionRequest;
pub struct ContextResolutionSemanticMemoryTarget;
impl ContextResolutionSemanticMemoryTarget {
    pub fn new(
        target_record_ref: &str, expected_target_fingerprint: Option<&str>,
    ) -> Result<Self, Error>;
    pub fn candidate_binding(&self) -> &Binding;
}
pub struct ContextResolutionSemanticMemoryRecord;
impl ContextResolutionSemanticMemoryRecord {
    pub fn new(record: Binding) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}
impl ContextResolutionPromotionRequest {
    pub fn new(
        request_id: &str, source_inputs: Vec<Binding>,
        source_envelope: &ContextResolutionEnvelope,
        stack: &ContextResolutionStackDefinition,
        target_memory_horizon: &str,
        target: ContextResolutionSemanticMemoryTarget,
        requested_authority_ref: &str,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}
pub enum ContextResolutionPromotionOutcome { Applied, Refused, Stale }
pub struct ContextResolutionPromotionDisposition;
impl ContextResolutionPromotionDisposition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        disposition_id: &str, request: &ContextResolutionPromotionRequest,
        outcome: ContextResolutionPromotionOutcome, decision: Binding,
        validation_evidence: Vec<Binding>, approving_authority_ref: &str,
        observed_target_fingerprint: Option<&str>,
        result_record: Option<ContextResolutionSemanticMemoryRecord>,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}

pub struct ContextResolutionTransitionRegistry;
impl ContextResolutionTransitionRegistry {
    pub fn new() -> Self;
    pub fn admit_escalation_request(
        &mut self, request: ContextResolutionEscalationRequest,
        requested: &Admission, trigger: &Admission, evidence: &[Admission],
    ) -> Result<bool, Error>;
    pub fn admit_escalation_disposition(
        &mut self, disposition: ContextResolutionEscalationDisposition,
        approving: &Admission, decision: &Admission,
    ) -> Result<bool, Error>;
    pub fn admit_promotion_request(
        &mut self, request: ContextResolutionPromotionRequest,
        requested: &Admission, sources: &[Admission], target: &Admission,
        target_memory: &Admission,
    ) -> Result<bool, Error>;
    pub fn admit_promotion_disposition(
        &mut self, disposition: ContextResolutionPromotionDisposition,
        approving: &Admission, decision: &Admission,
        evidence: &[Admission], target_memory: &Admission,
    ) -> Result<bool, Error>;
}

pub enum ContextResolutionKernelErrorKind {
    InvalidInput, AuthorityRefused, StaleAuthority, Indeterminate,
    ExpansionRefused, Cardinality, ReplayMismatch, ForbiddenTarget,
}
pub struct ContextResolutionKernelError;
impl ContextResolutionKernelError {
    pub fn kind(&self) -> ContextResolutionKernelErrorKind;
    pub fn detail(&self) -> &str;
    pub fn candidate(&self) -> Option<&ContextResolutionEscalationCandidate>;
}
```

The only modified existing public type is
`ContextResolutionStackDefinition`; its existing methods keep their signatures.
Its representation may retain levels, domains, and policy pairs privately.
`ResolvedInstanceProfile` is consumed unchanged. No public fields, serde value
dispatcher, public trait, blanket implementation, generic framework, or symbol
outside this list is authorized.

## Exact ceilings

Production paths:

- `crates/engine/src/context_resolution_registry.rs`;
- `crates/engine/src/context_resolution.rs`;
- `crates/engine/src/lib.rs`.

Test/fixture paths:

- `crates/engine/tests/context_resolution_stack.rs`;
- `crates/engine/tests/context_resolution_kernel.rs`;
- `crates/engine/tests/fixtures/hcm_3_2_context_resolution/**`.

Control paths are the exact HCM-3.2 slice files, review/proof dispatches and
records, affected HCM 00-06 rows after earned proof, one v1.4 handoff, ledger,
and a new `09` row only when required by a validated P3/P4.

Maximums are: 3 production paths, 3 test families, 80 changed named production
declarations, and 1,800 hand-written production lines. A named declaration is
counted once by post-state `(module, impl owner, declaration name)` for each
new or edited struct, enum, function, method, or constant in the three
production paths. Fields, variants, imports, derives, compiler-generated
implementations, and tests do not count. The three accepted existing symbols
count; unchanged reused primitives do not. No Cargo/dependency/version/unsafe/
schema-catalog/transport surface or ancillary path is allowed. Protected paths,
frozen historical dispatch/record corpora, and every path outside the ceilings
remain unstaged/uncommitted.

Live GitNexus reports HIGH for `ContextResolutionStackDefinition` (4 upstream,
4 processes), CRITICAL for `AuthoredStack::resolve` (47 upstream, 5 processes),
and CRITICAL for `ContextResolutionStackDefinition::load_bytes` (166 upstream,
8 processes). Product authority explicitly accepts only these three existing
symbol edits. The full engine/profile/CLI/setup/promotion/intake/lifecycle/
workspace replay wall is mandatory. `ApproverAuthorityPairV1` and assertion
verification also have existing CRITICAL graph reach but remain byte-for-byte
unchanged; the authenticator port is MEDIUM and guard/store are LOW. Any need
to edit those primitives or any fourth existing HIGH/CRITICAL symbol stops.

## Test selector

RED must fail because configurable stack application and envelopes do not yet
exist. GREEN must prove every SPEC proof obligation, including exact shipped
replay, non-shipped configurability, six independent dimensions, deterministic
fingerprints, child narrowing, deny overlap, indeterminate refusal, typed
memory/validation outcomes, no-effect escalation, transition cardinality, and
unchanged HCM-3.1/L0-L3 behavior. Authority GREEN must additionally exercise
the real selected-artifact/guard/generic-recovery/committed-registry path and
all eleven exact subjects, plus raw/semantic/currentness/profile/stack/head
mismatch, malformed/extra/missing binding fields, credential cardinality and
coverage, authenticator refusal/RP/UV/credential/signature/counter failures,
entropy failure through an internal test seam, exact replay, changed subject,
and captured response refusal across fresh resolvers with zero and nonzero
counters. Promotion GREEN must use an actual resolved source envelope and stack
to prove strict target-memory rank, ordered source-envelope admissions, typed
semantic-memory target/result, compare-and-write applied/refused/stale behavior,
and refusal of artifact/contract/posture or durable-memory claims.

No test may claim Projection, Snapshot, flow/pipeline adoption, posture, SDK,
transport, release, publication, or downstream proof.

## Review and proof selector

The prior planning stage is already independently CLEAN and must not be
reopened. This amended selector must receive one fresh implementation-stage
discovery CLEAN before RED/Rust. Later implementation review
uses the complete converged subject and exact proof wall. P1/P2 findings receive
one consolidated remediation plus different-fresh closure; only directly
causal unmasked findings may consume the two supplemental cycles. Proof and
final-closeout stages use fresh agents, and no cycle follows CLEAN.

Before the primary commit: focused/full tests, strict Clippy/format/diff,
validator/self-tests, exact definition replay, path ceiling, GitNexus scoped
and compare-to-main detection, remote baseline, and protected-root
non-access/non-mutation all have honest results. An unavailable FTS or
comparison is unavailable, not GREEN.

## Non-goals

- HCM-3.3 Projection or any Resolution-aware view;
- HCM-3.4 Snapshot Memory/delta;
- HCM-3.5 packet/flow/pipeline adoption or L0-L3 migration;
- HCM-3.6 posture;
- SDK, CLI/Tauri/Substrate/HTTP/dock/transport work;
- new dependency, crate, module framework, unsafe/native/platform machinery;
- shipped-definition/profile/vocabulary mutation;
- release, publication, push, or remote reconciliation;
- unrelated cleanup or immutable-history repair.

## Stop conditions

Stop for any required scope beyond the ceilings, new public/dependency/schema/
unsafe/transport authority, HCM-3.3+ behavior, changed shipped identity,
unresolved HIGH/CRITICAL impact, contract contradiction not repairable inside
HCM-3.2, mandatory delegation failure, exhausted causal budget, or unresolved
P1/P2. Local findings, proof gaps, and bounded remediation remain parent-owned.

The authority-resolution, candidate-mapping, exact-public-signature, and named
HIGH/CRITICAL conditions are now selected. The same slice remains stopped only
until the fresh implementation-stage selector-admission review is CLEAN. A
validator refusal, P1/P2, additional public/path/dependency/risk expansion, or
causal-budget contradiction stops fail closed; the parent/outcome/packets may
not be renamed or reset.
