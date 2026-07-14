# Contracts, Schemas, and Gates

## Status

These are preliminary target contracts to be frozen by Phase 0 design slices. They are architecture authority for direction, not yet published API guarantees.

## Schema policy

- Canonical human-authored structured records use YAML where appropriate.
- YAML canonical artifacts are parsed into the JSON data model and validated by versioned JSON Schema plus separate semantic validators.
- Every structured record has a stable `schema_id` and version.
- Transport DTOs serialize to JSON and publish JSON Schema.
- OpenAPI is an optional HTTP adapter output, not the CLI/Tauri/SDK authority.
- Schema versions and semantic contract versions are explicit and independently reviewable.
- Unknown required semantics fail closed; optional extension fields require a declared extension mechanism.
- Repository-local schema references are trusted only after repo-relative/no-follow resolution and meta-schema validation; remote fetching, executable schema hooks, and ambient unversioned overrides are initially refused.

## Instance profile contract

Conceptual minimum:

```yaml
schema_id: handbook.instance-profile
schema_version: "1.0"
profile_id: default
artifact_kind_sources: []
artifact_instances: []
vocabulary: {}
context_resolution: {}
projections: []
posture_evaluation_policy_ref: null
docks: []
```

The identifier `default` is illustrative. Its artifact kinds, instances, and requiredness are not frozen until `HCM-0.6` completes research and a user brainstorming/decision session.

Required gates:

- unique stable IDs;
- valid repo-relative paths;
- exactly one constitutional-root role unless Phase 0 explicitly changes this invariant;
- dependency graph has no invalid references or cycles;
- custom artifacts reference valid schemas;
- artifact instance IDs resolve valid exact kind definitions and compatible semantic capabilities;
- vocabulary conflations are explicit;
- Resolution stack is ordered and connected;
- projection definitions reference valid source roles and target levels.

## Artifact kind definition contract

```yaml
schema_id: handbook.artifact-kind-definition
schema_version: "1.0"
kind_id: handbook.artifact-kind.charter
kind_version: "1.0.0"
canonical_schema_ref: handbook.schemas/artifacts/charter/1.0
semantic_capabilities:
  - constitutional_root
structural_validation_profile_ref: json-schema.draft-2020-12
semantic_validation_profile_ref: charter.v1
intake_definition_ref: handbook.intake/charter/1.0
projections:
  - charter.review-markdown/v1
lifecycle_policy_ref: constitutional.review-lock/v1
review_triggers:
  - production_posture_changed
  - trust_boundary_changed
required_capabilities: []
extensions: {}
```

Kind gates:

- stable kind/schema IDs and versions are unique;
- canonical schema resolves safely and passes its declared meta-schema;
- semantic capabilities are declared rather than inferred from filenames;
- projection and optional intake refs resolve compatible versions;
- repository-defined kinds pass the same definition validation as shipped kinds;
- no new Rust enum variant or CLI command is required;
- schemas contain no executable hooks or undeclared remote references.

The actual shipped kind/default-instance set is not defined by this illustrative Charter example. It is frozen only by the research and user decision in `HCM-0.6`.

## Artifact instance descriptor contract

```yaml
id: project_context
kind_ref: handbook.artifact-kind.project-context@1.0.0
role: project_context
label: Project Context
canonical_path: .handbook/project_context/project-context.yaml
required_when: always
depends_on:
  - constitutional_root
authoring:
  intake_definition_ref: null
projections:
  - project_context.review_markdown
validation_overlays: []
```

Instance gates:

- `kind_ref` resolves exactly under the selected compatibility policy;
- path, label, requiredness, and dependencies are repository-instance concerns rather than kind-definition fields;
- selected role/capabilities are supported by the referenced kind;
- dependencies resolve artifact instance IDs or declared semantic capabilities without cycles;
- overlays cannot weaken kind schema, constitutional floors, or red lines.

Custom instances use generic operations. Specialized first-party behavior must key off stable capability/role semantics, not filename matching.

## Artifact intake definition contract

```yaml
schema_id: handbook.artifact-intake-definition
schema_version: "1.0"
intake_id: handbook.intake.charter
intake_version: "1.0.0"
artifact_kind_ref: handbook.artifact-kind.charter@1.0.0
candidate_schema_ref: handbook.schemas/artifacts/charter/1.0
supported_modes:
  - guided_adaptive
  - express
  - agent_assisted
coverage:
  - coverage_id: operational_reality.production_state
    target_paths:
      - /project_conditions/production_state
    applicability: always
    acquisition:
      inferable: true
      user_declaration_required: false
      evidence_kinds:
        - repository_configuration
      freshness: session
    evaluation:
      required: true
      minimum_specificity: concrete
      contradiction_policy: block
  - coverage_id: governance.exception_approvers
    target_paths:
      - /governance/exceptions/approvers
    applicability: always
    acquisition:
      inferable: false
      user_declaration_required: true
      evidence_kinds: []
    evaluation:
      required: true
      minimum_specificity: named_role_or_owner
      contradiction_policy: block
approval_policy_ref: constitutional-candidate/v1
reassessment_triggers:
  - production_posture_changed
  - trust_boundary_changed
```

Intake gates:

- every coverage item maps to valid candidate-schema paths or an explicit rationale/evidence-only outcome;
- shorter modes cannot weaken required coverage or hide unknowns;
- inferable observations record evidence, confidence, freshness, and sensitivity;
- normative/constitutional decisions identify the authority required to approve them;
- question wording may vary by skill/projection, but stable coverage IDs and evaluation semantics do not;
- no intake definition embeds a model/provider call.

## Intake record and artifact candidate contracts

An immutable `ArtifactIntakeRecord` records:

- intake definition and acquisition mode/version;
- skill/agent/consumer identity;
- questions or coverage prompts presented;
- user declarations separately from agent inferences and deterministic defaults;
- evidence, snapshot, confidence, freshness, and sensitivity refs;
- applicability decisions, known unknowns, contradictions, waivers, and evaluation results;
- candidate and approval/promotion refs.

An `ArtifactCandidate` records:

- target kind/instance/schema refs;
- normalized candidate content or content-addressed artifact ref;
- per-field source/coverage mapping;
- validation diagnostics and unresolved gaps;
- promotion eligibility and required approver;
- candidate fingerprint.

Neither record is canonical. Promotion validates the candidate again against current kind/profile truth, records explicit approval, writes canonical YAML atomically, and emits provenance/fingerprint refs.

## Charter intake and canonical contract

`CharterIntakeDefinition` is the first rich shipped intake definition. Its coverage must account for the historical domains of project shape, delivery constraints, operational reality, posture and delivery implications, risk domains, engineering dimensions, exceptions/governance, debt, and decision records. Research/design may revise questions and branches, but omissions are explicit decisions.

Guided-adaptive, express, and agent-assisted modes all produce the same canonical Charter candidate schema. The skill-directed LLM agent conducts the conversation and invokes Handbook CLI/SDK operations; Handbook owns coverage/evaluation/promotion and performs no hidden nested synthesis.

Canonical Charter YAML owns approved constitutional truth. Intake provenance explains how it was reached, and Markdown/GUI/packet/agent views are projections rather than independently editable authorities.

## Artifact validation layers

Do not ask one schema mechanism to own every kind of correctness:

1. **Structural schema** — fields, types, enumerations, requiredness, and local shape after YAML parsing.
2. **Semantic validation** — cross-field, cross-artifact, lifecycle, capability, and authority invariants owned by Handbook.
3. **Intake coverage evaluation** — whether required information was established with appropriate evidence, specificity, confidence, and approval.
4. **External validator docks** — domain-specific witnesses that emit normalized evidence without becoming artifact or contract authority.

A custom kind may need only structural and semantic validation. Human-authored constitutional/governance kinds commonly add intake coverage. Docks remain optional unless the kind/profile/contract requires them.

## Vocabulary contract

The vocabulary schema distinguishes labels, aliases, and structural absorption:

```yaml
labels:
  coordination_horizon: phase
  delivery_unit: feature
  implementation_unit: slice
  execution_envelope: packet
  atomic_action: task
aliases:
  implementation_unit:
    - task
absorptions: []
```

Duplicate displayed labels are legal. Ambiguity matters only when a machine operation cannot resolve a stable role from the surrounding typed context.

## Context Resolution envelope

```yaml
schema_id: handbook.context-resolution-envelope
schema_version: "1.0"
active_level: execution
dimensions:
  scope_horizon: assigned_unit
  detail_resolution: normal
  temporal_horizon: immediate
  authority_horizon: local_write
  memory_horizon: execution
  validation_horizon: unit_closeout
parent_refs: []
inherited_constraint_refs: []
allowed_mutations: []
forbidden_mutations: []
escalation_triggers: []
```

Envelope gates:

- active level exists in the selected profile;
- narrower authority cannot override inherited broader constraints;
- allowed and forbidden mutation sets cannot resolve ambiguously;
- validation claims cannot exceed the validation horizon;
- memory writes above the allowed horizon require promotion evidence;
- expansion requests identify the missing context/authority condition.

## Projection request

```yaml
schema_id: handbook.projection-request
schema_version: "1.0"
artifact_refs: []
projection_id: agent.execution-packet
vocabulary_profile_ref: default
resolution_envelope: {}
surface: agent_packet
```

## Projection result

```yaml
schema_id: handbook.projection-result
schema_version: "1.0"
projection_id: agent.execution-packet
source_refs: []
source_fingerprints: []
profile_fingerprint: ""
projection_definition_version: "1"
resolution_envelope: {}
operation: reveal
lossiness: lossless
included_claim_refs: []
omitted_claim_refs: []
derived_artifact_refs: []
promotion_eligibility: local_only
```

Allowed `operation` values initially:

- `reveal`;
- `derive`.

Reserve `synthesize_candidate` in the conceptual model but do not implement it in the first projection slice.

## Snapshot capture policy

```yaml
schema_id: handbook.snapshot-capture-policy
schema_version: "1.0"
policy_id: session-boundary.default
triggers:
  - session_start
  - session_end
state_families:
  git:
    include_paths: true
    include_diff_stats: true
    full_diff: artifact_ref_only
  handbook:
    include_profile: true
    include_artifact_fingerprints: true
    include_contract_state: true
  work:
    recent_completed:
      count: 10
      source_ref: work-ledger
      ordering: completed_at_then_id
    queued_next:
      count: 10
      source_ref: active-plan
      ordering: canonical_queue_order
  evidence:
    include_latest_gate_refs: true
redaction_policy_ref: snapshot-redaction.default
consistency:
  retries: 2
  unstable_action: persist_non_promotable
retention_policy_ref: snapshot-retention.session
```

Capture policy gates:

- every selected state family names its authority/source;
- bounded history windows name count, source, cursor, and ordering;
- sensitive content defaults to excluded or artifact-ref-only;
- capture hooks and retention are explicit;
- unstable captures cannot support promotion or closeout;
- changing capture policy changes its fingerprint/version.

## Context Memory Snapshot

Conceptual minimum:

```yaml
schema_id: handbook.context-memory-snapshot
schema_version: "1.0"
snapshot_id: snap_...
capture:
  trigger: session_end
  policy_ref: session-boundary.default
  started_at: "..."
  completed_at: "..."
  producer_version: "..."
  consistency: stable
  pre_revisions: {}
  post_revisions: {}
context_resolution: {}
repository:
  repository_id: "..."
  worktree_id: "..."
  branch: main
  head: "..."
  upstream: "..."
  operation_state: clean
  dirty_paths: []
  untracked_paths: []
  diff_summary: {}
  diff_artifact_refs: []
handbook:
  profile_ref: "..."
  profile_fingerprint: "..."
  artifact_kind_registry_fingerprint: "..."
  vocabulary_fingerprint: "..."
  resolution_stack_fingerprint: "..."
  artifacts: []
  intake_refs: []
  unresolved_intake_coverage: []
  posture_kernel_ref: null
  posture_recommendation_refs: []
  contracts: []
  verdict_refs: []
  gate_refs: []
work:
  active_refs: []
  recent_completed: []
  queued_next: []
  blocked_refs: []
  deferred_refs: []
  escalation_refs: []
session:
  session_ref: "..."
  handoff_ref: "..."
  dispatch_ref: "..."
evidence:
  validation_refs: []
  unresolved_proof_refs: []
redaction:
  policy_ref: snapshot-redaction.default
  excluded_surfaces: []
previous_snapshot_ref: null
state_fingerprint: sha256:...
record_fingerprint: sha256:...
promotion_eligibility: grounding_only
```

### Snapshot consistency

Supported consistency values:

- `stable` — selected authorities/revisions did not change during capture;
- `bounded` — separately captured surfaces are revision-bound and all remained within declared bounds;
- `unstable` — one or more authorities changed and the retry policy could not obtain a stable/bounded record.

An unstable snapshot remains useful for diagnostics but cannot ground a closeout, promotion, or hard gate.

### Snapshot fingerprints

- `state_fingerprint` covers normalized observed state and excludes volatile capture timestamp/trigger metadata.
- `record_fingerprint` covers the complete immutable record.
- Map keys, paths, work-item windows, and evidence refs use canonical deterministic ordering.
- Two records captured at different times may have equal state fingerprints.

### Snapshot authority

Snapshot Memory is descriptive evidence. It cannot:

- lock or mutate a contract;
- replace canonical artifacts;
- rewrite a queue or handoff;
- infer why a divergence occurred;
- pass claims beyond captured/observed state.

## Snapshot delta

```yaml
schema_id: handbook.snapshot-delta
schema_version: "1.0"
delta_id: delta_...
from_snapshot_ref: snap_previous
to_snapshot_ref: snap_current
compatibility:
  capture_policies_compatible: true
  compared_state_families: []
changes:
  git: {}
  artifacts: []
  contracts: []
  work_completed: []
  work_not_completed: []
  unplanned_work: []
  queue_changes: []
  blockers_added: []
  blockers_cleared: []
  proof_gates_gained: []
  proof_gates_lost: []
signals:
  - kind: expected_progress
    evidence_refs: []
    justification_refs: []
delta_fingerprint: sha256:...
```

Deterministic signal values:

- `expected_progress`;
- `justified_divergence`;
- `unexplained_drift`;
- `scope_expansion`;
- `execution_inefficiency_signal`;
- `planning_inaccuracy_signal`;
- `proof_drift`;
- `semantic_drift`;
- `stale_handoff`.

Signals identify evidence and durable justification refs. They do not make an unreviewed causal claim.

## Snapshot projection request/result

```yaml
schema_id: handbook.snapshot-projection-request
schema_version: "1.0"
snapshot_ref: snap_current
delta_ref: delta_previous_to_current
target_resolution_envelope: {}
purpose: session_grounding
include_families:
  - active_work
  - changed_paths
  - unresolved_blockers
  - queued_next
  - applicable_contracts
  - proof_obligations
```

```yaml
schema_id: handbook.snapshot-projection-result
schema_version: "1.0"
snapshot_ref: snap_current
snapshot_state_fingerprint: sha256:...
delta_ref: delta_previous_to_current
target_resolution_envelope: {}
included_paths: []
omitted_paths: []
grounding_data: {}
lossiness: collapsed
projection_fingerprint: sha256:...
promotion_eligibility: grounding_only
```

Projection gates:

- included fields fit the target Resolution authority and detail horizons;
- omitted sensitive or out-of-scope fields remain enumerated;
- comprehensive capture does not imply comprehensive disclosure;
- grounding projection never mutates the source snapshot;
- snapshot/delta fingerprints remain traceable;
- a new live capture or revision check detects staleness before acting.

## Snapshot redaction and retention

By default snapshots exclude:

- secret values and credential material;
- unrestricted environment variables;
- `.env` and secret-file contents;
- raw command arguments/output that may carry secrets;
- full diffs when normalized statistics/fingerprints and evidence refs suffice.

Snapshots record the redaction policy and excluded surfaces. Retention is profile/policy-driven by horizon and trigger. Immutable retained records may be content-addressed and deduplicated; compaction writes a new reviewed aggregate and never rewrites retained source snapshots.

## Project posture kernel and recommendation contracts

`ProjectPostureKernel` is a deterministic resolved result with:

- source Charter/profile/override/contract/evidence/snapshot refs and fingerprints;
- effective `EngineeringPostureDimension` levels, floors, red lines, triggers, shortcuts, and proof obligations;
- applicable domains/scopes and freshness-qualified conditions;
- resolution timestamp/revisions and kernel fingerprint;
- omitted/unresolved conditions and recommendation refs.

It is not independently editable canonical truth. Re-resolution from identical source state must produce the same kernel fingerprint.

```yaml
schema_id: handbook.posture-recommendation
schema_version: "1.0"
recommendation_id: posture_rec_...
kernel_ref: posture_kernel_...
evaluation_policy_ref: posture-policy.default/v1
affected_dimensions:
  - testing_rigor
  - rollout_controls
scope_ref: artifact.public_api
proposed_transition:
  from: 3
  to: 4
trigger_kind: hard
triggering_observations: []
evidence_refs: []
snapshot_delta_refs: []
confidence: high
urgency: before_next_release
required_approval_ref: charter.governance.project_owner
notification:
  recipient_refs: []
  acknowledgment_required: true
  escalate_after: null
suggested_actions: []
promotion_eligibility: recommendation_only
```

Recommendation gates:

- every proposal cites current kernel and observed trigger evidence;
- hard triggers and accumulated signals remain distinct;
- unexplained snapshot drift cannot silently become a causal conclusion;
- recommendations never modify canonical Charter/override state;
- lowering requires configured sustained evidence/cooldown and cannot cross floors/red lines;
- notification delivery is adapter-owned and cannot change the recommendation, acknowledgment, or approval semantics;
- acceptance writes an authorized `PostureTransition`, revalidates affected intake coverage, updates the proper canonical authority, and re-resolves the kernel.

## Optional synthesis-candidate contract

If later approved:

```yaml
operation: synthesize_candidate
producer:
  adapter: unified-agent-api
  provider_family: codex
  model: "..."
input_projection_refs: []
prompt_contract_ref: "..."
candidate_artifact_ref: "..."
promotion_eligibility: requires_review_and_lock
```

The exact UAA model/provider contract is loaded from the live `unified-agent-api` repository only for that future slice.

## SDK command contract

Every ordinary use case has:

- a typed request;
- a typed result;
- structured expected blocked/refused states;
- a stable operation identifier;
- schema and capability versions;
- deterministic serialization.

Snapshot use cases should include capture, compare/delta, project, inspect, and verify-current operations without forcing callers to parse git or handoff prose themselves.

Artifact/intake use cases should include kind/instance discovery, structural/semantic validation, coverage inspection, evidence/declaration submission, candidate validation, promotion, projection, posture inspection, and recommendation acknowledgment. Operation IDs remain generic and select kind/instance IDs; custom kinds and vocabulary never create or rename CLI commands.

Proposed common response envelope:

```json
{
  "schema_id": "handbook.command-response",
  "schema_version": "1.0",
  "operation": "contract.verify",
  "status": "ok",
  "data": {},
  "diagnostics": [],
  "next_actions": [],
  "artifact_refs": [],
  "provenance": {}
}
```

Status values:

- `ok`;
- `blocked`;
- `refused`;
- `error`.

Domain verdicts such as contract `fail` remain inside `data`; they are not collapsed into process/transport failure.

## CLI JSON contract

For every nontrivial command in JSON mode:

1. stdout contains exactly one complete JSON response document;
2. human progress and logs use stderr;
3. expected blocked/refused outcomes still serialize the envelope;
4. exit codes are stable and separately documented;
5. no ANSI styling appears in JSON;
6. large/binary outputs use artifact references;
7. the response reports capability/schema versions;
8. human output renders from the same typed result.

The initial Substrate CLI bridge consumes this contract only. It never parses human text.

## Contract record

```yaml
schema_id: handbook.contract
schema_version: "1.0"
contract_id: example.operation.v1
kind: operation
status: locked
authority_resolution: coordination
subject: {}
claims: []
required_evidence: []
scoring: {}
```

Lifecycle target:

```text
draft -> review_ready -> locked -> active -> passed|blocked -> closed
                                  \-> deprecated
```

Lifecycle transitions require typed evidence and authority. A rendered document or passing test cannot lock or close a contract by itself.

## Evidence record

Evidence identifies:

- contract and claim refs;
- producer/dock identity and version;
- subject and case;
- observed facts;
- artifact/trace refs;
- source fingerprints;
- Context Resolution envelope;
- included and unobserved claims;
- execution provenance;
- confidence/flakiness where applicable.
- snapshot and delta refs when point-in-time state supports the observation.

## Verdict contract

Supported verdicts:

- `pass`;
- `fail`;
- `blocked`;
- `warning`;
- `not_observed`;
- `not_applicable`;
- `flaky`.

Rules:

- hard-fail severity blocks regardless of aggregate score;
- required `not_observed` cannot become pass through weighting;
- evidence outside the required Resolution/subject boundary cannot satisfy a claim;
- flaky evidence remains visible and cannot silently average into green;
- verdicts cite evidence records.

## Gate contract

A gate identifies:

- target contract and lifecycle transition;
- claim verdicts;
- hard blockers;
- weighted score if used;
- local closeout eligibility;
- parent promotion eligibility;
- remediation/next actions;
- evidence and projection refs.
- snapshot/delta refs used for grounding, staleness, or drift decisions.

Local completion and broader promotion are separate decisions.

## Dock capability manifest

```json
{
  "schema_id": "handbook.dock-capability",
  "schema_version": "1.0",
  "dock_id": "example.validator",
  "dock_protocol_versions": ["1.0"],
  "contract_kinds": ["operation"],
  "evidence_kinds": ["schema_validation"],
  "input_media_types": ["application/json"],
  "resolution_support": {
    "minimum_level": "execution",
    "dimensions": ["scope_horizon", "detail_resolution", "validation_horizon"]
  },
  "execution": {
    "mode": "process",
    "supports_timeout": true,
    "supports_cancellation": true
  }
}
```

## Dock request/result

Request includes:

- protocol and schema versions;
- contract and selected claim refs;
- Resolution envelope;
- canonical/projection/evidence artifact refs;
- snapshot/delta refs when the validator needs a point-in-time state boundary;
- workspace capability grant;
- timeout/cancellation policy;
- requested evidence types.

Result includes:

- dock identity/version;
- structured status/refusal;
- normalized evidence refs;
- observed/unobserved claim refs;
- diagnostics;
- produced artifact refs;
- exact Resolution/provenance;
- execution timing and completion state.

## Public API proof gates

### CLI bridge gate

- exact Handbook binary version;
- exact JSON schema/capability versions;
- real Substrate seam;
- no human-output parsing;
- explicit replacement boundary.

### Published Rust API gate

A downstream-intended API is complete only when:

1. relevant crates are published to crates.io;
2. a registry-only external consumer resolves exact versions;
3. a dedicated Substrate worktree from current tip imports those exact versions;
4. a real seam uses the API;
5. the proof wall passes;
6. no sibling-path or unpublished fallback exists.

The CLI bridge may ship earlier, but it cannot satisfy this Rust API gate.

## Schema compatibility posture

This greenfield program does not promise compatibility with legacy Handbook artifact formats.

Compatibility rules apply only after a new schema/API is deliberately published as supported. Temporary internal cutover types must name their deletion gate and cannot become implicit public contracts.
