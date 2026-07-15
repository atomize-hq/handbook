# Contracts, Schemas, and Gates

## Status

The HCM-0.2 sections are frozen design contracts: schema policy, instance-profile composition for stable-role/schema/kind/instance/vocabulary truth, stable-role and schema registries, artifact kinds, artifact instances, intake records/candidates/promotion, Charter/constitutional-root semantics, validation layers, vocabulary, and the project-posture owner/transition boundary. HCM-0.3 additionally freezes the Context Resolution stack/envelope/escalation/promotion, deterministic Projection definition/request/result, Snapshot Memory capture/record/delta/projection, and redaction/retention contracts. HCM-0.4 freezes crate ownership, the SDK ordinary-use-case inventory, operation/DTO identities, CLI JSON and Tauri adapter behavior, the transitional Substrate process bridge, and the permanent published-Rust proof plan. They are implementation authority for later slice packets, not published API guarantees or evidence that the runtime types exist.

Contract/dock semantic sections remain preliminary until HCM-0.5 closes. The HCM-0.4 owner, SDK/transport, and public-proof-plan sections are frozen design authority but do not prove any implemented or published API. The shipped artifact-kind/default-instance/requiredness set and shipped Resolution labels/default policy remain unresolved rather than being selected by illustrative examples.

## Schema policy

- Canonical human-authored structured records use YAML where appropriate.
- YAML canonical artifacts are parsed into the JSON data model and validated by versioned JSON Schema plus separate semantic validators.
- Every structured record has a stable `schema_id` and version.
- Transport DTOs serialize to JSON and publish JSON Schema.
- OpenAPI is an optional HTTP adapter output, not the CLI/Tauri/SDK authority.
- Schema versions and semantic contract versions are explicit and independently reviewable.
- Unknown required semantics fail closed; optional extension fields require a declared extension mechanism.
- Repository-local schema references are trusted only after repo-relative/no-follow resolution and meta-schema validation; remote fetching, executable schema hooks, and ambient unversioned overrides are initially refused.
- First-version schema, kind, stable-role registry, intake, renderer, capability-contract, posture-trigger/policy, and profile-parent references are exact ID/version references with their required semantic fingerprints. Version ranges, implicit latest selection, and undeclared fallback are refused.
- Canonical normalization and fingerprints exclude absolute machine paths, timestamps unless time is itself semantic input, and presentation-only ordering. They include every semantic value and exact referenced definition fingerprint.

### Uniform exact-definition identity

Every HCM-0.2/HCM-0.3 versioned definition uses a namespaced stable identity field and SemVer field. Its exact ref is mechanically `identity + "@" + version`; for example, `profile_id: handbook.profile.example` plus `profile_version: "1.0.0"` yields `handbook.profile.example@1.0.0`. A ref that cannot be derived this way, resolves to different identity fields, or resolves more than one record is invalid.

Every such definition also exposes one derived fingerprint field named by its contract (`profile_fingerprint`, `entry_fingerprint`, `registry_fingerprint`, `definition_fingerprint`, `capability_fingerprint`, `intake_definition_fingerprint`, `vocabulary_fingerprint`, `trigger_fingerprint`, `policy_fingerprint`, or another explicitly declared `*_fingerprint`). The common derivation is:

1. parse YAML into the JSON data model;
2. remove only the definition's own derived fingerprint field and fields explicitly classified as presentation-only by its matrix;
3. resolve every exact referenced definition and include its recomputed fingerprint in a typed closure entry, failing on a missing producer, mismatch, cycle, or duplicate identity;
4. sort object keys; preserve semantically ordered arrays; sort arrays declared unordered by their stable identity key;
5. serialize as UTF-8 RFC 8785 JSON Canonicalization Scheme bytes and emit lowercase `sha256:<64-hex>`.

Type-specific matrices may add closure inputs but may not weaken this base. A referenced renderer, lifecycle policy, semantic validator, approval policy, freshness policy, signal, or evidence requirement is admissible even when its later behavioral contract is not yet frozen only if its exact versioned definition already satisfies this identity/fingerprint envelope. Changed bytes behind the same exact ref must change the recomputed fingerprint and invalidate every stale consumer. Bare refs, caller-trusted fingerprints, and versioned records with no fingerprint producer fail closed.

## Instance profile contract

Conceptual minimum:

```yaml
schema_id: handbook.instance-profile
schema_version: "1.0"
profile_id: handbook.profile.example
profile_version: "1.0.0"
profile_scope: named
extends_profile_ref: null
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
schema_registry_sources:
  - handbook.schemas.artifacts.charter@1.0.0
  - handbook.schemas.artifacts.project-context@1.0.0
  - handbook.schemas.projection.agent-packet@1.0.0
  - handbook.schemas.projection.snapshot-grounding@1.0.0
  - handbook.schemas.context-memory-snapshot@1.0.0
  - handbook.schemas.snapshot-delta@1.0.0
artifact_kind_sources:
  - handbook.artifact-kind.charter@1.0.0
  - handbook.artifact-kind.project-context@1.0.0
artifact_instances:
  - schema_id: handbook.artifact-instance-descriptor
    schema_version: "1.0"
    id: example_constitutional_root
    kind_ref: handbook.artifact-kind.charter@1.0.0
    role_ref: constitutional_authority
    capability_refs:
      - constitutional_root
    label: Example Constitutional Root
    canonical_path: .handbook/example/root.yaml
    requiredness:
      mode: always
      condition_ref: null
    depends_on: []
    lifecycle_policy_ref: handbook.lifecycle.constitutional-review-lock@1.0.0
    intake_definition_ref: handbook.intake.charter@1.0.0
    renderer_definition_refs: []
    projection_definition_refs: []
    validation_overlay_refs: []
    extensions: {}
vocabulary_ref: handbook.vocabulary.example@1.0.0
context_resolution_ref: handbook.context-resolution.example@1.0.0
projection_catalog_refs:
  - handbook.projection.example-agent-packet@1.0.0
  - handbook.projection.snapshot-grounding@1.0.0
posture_evaluation_policy:
  ref: handbook.posture-policy.example@1.0.0
  fingerprint: sha256:...
dock_requirement_refs: []
adapter_overlay_refs: []
extensions: {}
profile_fingerprint: sha256:...
```

This is one internally consistent illustrative authored profile, not a proposed shipped profile. Its artifact kind, instance, label, path, renderer selection, and requiredness are examples only and do not freeze the shipped set; that decision remains reserved until `HCM-0.6` completes research and a user brainstorming/decision session.

Required gates:

- unique stable IDs;
- exact parent/source refs with an acyclic profile inheritance chain;
- one exact stable-role registry ref/fingerprint pair shared by every resolved kind and vocabulary record;
- one of `shipped`, `named`, or `repository` scope, with parent scope ordered no narrower than child scope;
- valid repo-relative paths;
- exactly one `always`-required instance selecting the `constitutional_root` capability;
- dependency graph has no invalid references or cycles;
- custom artifacts reference valid schemas;
- artifact instance IDs resolve valid exact kind definitions, stable roles, and compatible semantic capabilities;
- vocabulary conflations are explicit;
- Resolution stack is ordered and connected;
- every projection definition resolves exactly from the profile catalog; its typed source selectors, target schema, allowed surfaces/operations, mandatory currentness, exact disclosure policy/support evaluator, and per-rule six-dimension/classification closure validate against the resolved profile's registered kinds, capabilities, schemas, and exact Context Resolution selection.

Field authority/defaulting matrix:

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies the profile record shape | none | exact supported pair | not the profile's semantic version |
| `profile_id`, `profile_version` | profile author identifies one versioned semantic configuration | none | stable ID plus SemVer; unique tuple | no implicit `default` or latest profile |
| `profile_scope` | profile author declares `shipped`, `named`, or `repository` layer scope | none | allowlisted value; ancestry order is shipped -> named -> repository | not runtime authority horizon |
| `extends_profile_ref` | profile author selects one exact parent | `null` | exact resolvable ref; acyclic | no ambient or multi-parent merge |
| `stable_role_registry.ref`/fingerprint | stable-role registry owns role IDs/default labels; profile author selects exact semantics | root: required; child omission inherits; present pair replaces whole | exact versioned ref; supplied fingerprint matches; every kind/vocabulary pair is identical | no ambient registry or label drift |
| `schema_registry_sources` | profile author supplies a complete ordered source list when present | root: required; child omission inherits; present list replaces whole; explicit empty clears | exact refs; unique identities; conflicts within replacement fail | no append merge or remote discovery |
| `artifact_kind_sources` | profile author supplies a complete ordered kind-source list when present | root: required; child omission inherits; present list replaces whole; explicit empty clears | exact refs; unique identities; conflicts within replacement fail | no append merge or enum/template inference |
| `artifact_instances` | profile author supplies a complete instance registry when present | root: required; child omission inherits; present list replaces whole; explicit empty clears and then fails final constitutional validation | unique IDs/paths; valid kinds/roles/capabilities/requiredness/dependencies | no keyed merge/tombstone or reusable kind authority |
| `vocabulary_ref` | profile author selects one exact vocabulary profile | root: required; child omission inherits; present value replaces | resolves and validates | does not rename machine identifiers or commands |
| `context_resolution_ref` | Context Resolution stack-definition author owns comparison/default semantics; profile author selects one exact definition | root: required exact ref; child omission inherits; present ref replaces | exact ref/fingerprint closure; ordered connected levels; all six ranked domains/defaults complete | not an invocation override or a fixed L0-L3 mapping |
| `projection_catalog_refs` | Projection-definition authors own deterministic mapping semantics; profile author selects a complete compatible catalog | root: explicit list, which may be empty; child omission inherits; present list replaces whole | exact unique refs/fingerprints; sources, surfaces, schemas, derivations, disclosure policies/support evaluators/classifications/minimum ranks, and stack compatibility resolve | fixed renderer refs and executable hooks do not belong here |
| `posture_evaluation_policy` | profile author selects an approved policy ref/fingerprint pair | root: exact pair or `null`; child omission inherits; present pair/null replaces | exact compatible current policy fingerprint | null disables recommendations; no automatic Charter mutation |
| `dock_requirement_refs` | later HCM-0.5 authority | root: explicit empty; child omission inherits; present list replaces whole | exact refs after later contract lands | no executable validator in a profile |
| `adapter_overlay_refs` | adapter/profile integration authority | root: explicit empty; child omission inherits; present list replaces whole | exact declared refs; cannot change domain truth | no transport-owned semantics |
| `extensions` | declaring schema owns namespaced optional additions | root: explicit map; child omission inherits; present map replaces whole | registered namespace and optional-field rules | no key merge or unknown required semantics |
| `profile_fingerprint` | Handbook derives immutable source-layer identity | none; derived for every source profile | uniform exact-definition normalization over authored fields except this fingerprint; referenced definitions are pinned by their exact refs/fingerprints | not the fully resolved profile fingerprint |

All authored fields after `extends_profile_ref` through `extensions` are optional in a child source record specifically so omission and explicit empty/null remain distinguishable. Root profiles materialize every authored field, and every source profile carries a derived `profile_fingerprint`. No v1 invocation request may override these fields; invocation selects an exact profile ref only.

### Resolved instance profile

```yaml
schema_id: handbook.resolved-instance-profile
schema_version: "1.0"
profile_ref: handbook.profile.example@1.0.0
ancestry:
  - profile_ref: handbook.profile.example@1.0.0
    profile_fingerprint: sha256:...
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
schema_registry_entries:
  - entry_ref: handbook.schemas.artifacts.charter@1.0.0
    entry_fingerprint: sha256:...
    closure_fingerprint: sha256:...
  - { entry_ref: handbook.schemas.artifacts.project-context@1.0.0, entry_fingerprint: sha256:..., closure_fingerprint: sha256:... }
  - { entry_ref: handbook.schemas.projection.agent-packet@1.0.0, entry_fingerprint: sha256:..., closure_fingerprint: sha256:... }
  - { entry_ref: handbook.schemas.projection.snapshot-grounding@1.0.0, entry_fingerprint: sha256:..., closure_fingerprint: sha256:... }
  - { entry_ref: handbook.schemas.context-memory-snapshot@1.0.0, entry_fingerprint: sha256:..., closure_fingerprint: sha256:... }
  - { entry_ref: handbook.schemas.snapshot-delta@1.0.0, entry_fingerprint: sha256:..., closure_fingerprint: sha256:... }
artifact_kind_definitions:
  - kind_ref: handbook.artifact-kind.charter@1.0.0
    definition_fingerprint: sha256:...
  - kind_ref: handbook.artifact-kind.project-context@1.0.0
    definition_fingerprint: sha256:...
artifact_instances:
  - schema_id: handbook.artifact-instance-descriptor
    schema_version: "1.0"
    id: example_constitutional_root
    kind_ref: handbook.artifact-kind.charter@1.0.0
    role_ref: constitutional_authority
    capability_refs:
      - constitutional_root
    label: Example Constitutional Root
    canonical_path: .handbook/example/root.yaml
    requiredness:
      mode: always
      condition_ref: null
    depends_on: []
    lifecycle_policy_ref: handbook.lifecycle.constitutional-review-lock@1.0.0
    intake_definition_ref: handbook.intake.charter@1.0.0
    renderer_definition_refs: []
    projection_definition_refs: []
    validation_overlay_refs: []
    extensions: {}
vocabulary:
  vocabulary_ref: handbook.vocabulary.example@1.0.0
  vocabulary_fingerprint: sha256:...
context_resolution:
  ref: handbook.context-resolution.example@1.0.0
  fingerprint: sha256:...
projection_catalog:
  - { ref: handbook.projection.example-agent-packet@1.0.0, fingerprint: sha256:... }
  - { ref: handbook.projection.snapshot-grounding@1.0.0, fingerprint: sha256:... }
posture_evaluation_policy:
  ref: handbook.posture-policy.example@1.0.0
  fingerprint: sha256:...
dock_requirement_refs: []
adapter_overlay_refs: []
extensions: {}
layer_decisions:
  - field: stable_role_registry
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: schema_registry_sources
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: artifact_kind_sources
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: artifact_instances
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: vocabulary_ref
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: context_resolution_ref
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: projection_catalog_refs
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: posture_evaluation_policy
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: dock_requirement_refs
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: adapter_overlay_refs
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: extensions
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
diagnostics: []
resolved_profile_fingerprint: sha256:...
```

This is a generic conformance example, not a shipped profile/default decision.

| Resolved field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| schema identity and `profile_ref` | Handbook resolver identifies the derived record and selected exact source profile | none | exact supported schema; source profile exists | resolved view is not editable profile authority |
| `ancestry` | resolver records ordered source identity/fingerprint chain | none; at least selected profile | exact refs/fingerprints; acyclic; legal scope order | no hidden/ambient layer |
| `stable_role_registry` | winning profile layer selects exact role semantics; registry remains authority | none | exact ref/fingerprint; every kind/vocabulary pair matches; all role refs resolve | no ambient registry/default label |
| `schema_registry_entries` | resolver materializes schema definitions and document closures from the winning complete source field | empty only if no kind needs a schema | unique exact refs; current entry fingerprints plus safe document closure fingerprints | a document-only closure fingerprint cannot stand in for definition identity; no source-order conflict repair |
| `artifact_kind_definitions` | resolver materializes exact kinds and fingerprints | empty only if no instance exists | unique exact kinds; all instance refs resolve | no inferred enum kinds |
| `artifact_instances` | winning profile layer remains configuration authority; resolver materializes full descriptors | none; final list must be explicit | all descriptor gates plus exactly one constitutional capability | no keyed merge/tombstone |
| `vocabulary` | winning vocabulary record remains display/workflow mapping authority | none | exact ref/fingerprint and vocabulary gates | no capability/command renaming |
| `context_resolution` | winning `context_resolution_ref` layer selects exact stack semantics; resolver materializes its pair | none | exact ref/fingerprint; ordered levels, six ranked domains, and defaults resolve | no ambient stack, fixed L0-L3 inference, or invocation mutation |
| `projection_catalog` | winning `projection_catalog_refs` layer selects a complete catalog; resolver materializes pairs | explicit list, which may be empty | exact compatible refs/fingerprints; every advertised kind/instance Projection and its disclosure-policy/support-evaluator/classification/rank closure resolves through the catalog and exact profile stack | no fixed renderer, ambient disclosure/support registry, or dynamic executable behavior |
| remaining later-phase refs/lists | their named profile field/later contract owns them | explicit null/empty as shown until later freezes | later contract compatibility | no premature dock behavior |
| `posture_evaluation_policy` | winning profile layer selects policy ref/fingerprint pair | explicit pair or null | compatible exact current policy | null disables recommendations, not policy authority |
| `adapter_overlay_refs` | winning profile layer selects declared overlays | explicit list | exact refs cannot change domain truth | no transport-owned semantics |
| `extensions` | winning profile layer supplies complete namespaced map | explicit map | declared optional namespaces | no key merge/unknown required semantics |
| `layer_decisions` | resolver explains the winning source for every layerable field | none; one decision per field | exact ancestry source and `inherited` or `replaced` disposition | diagnostics do not change authority |
| `diagnostics` | resolver records deterministic warnings/refusals | empty on clean resolution | typed, stable ordering | no silent conflict repair |
| `resolved_profile_fingerprint` | Handbook derives complete semantic identity | none | normalized record plus stable-role registry and every exact definition fingerprint; excludes diagnostics, absolute paths, timestamps, invocation requests | no caller-supplied trust |

Profile conformance scenarios required by implementation packets:

| Scenario | Required result |
|---|---|
| child omits a field | inherits the complete parent value and records `inherited` |
| child supplies a field | replaces the complete parent value and records `replaced` |
| child supplies empty instances or removes constitutional capability | final resolution refuses; no tombstone semantics |
| replacement list contains duplicate/conflicting schema, kind, instance, or path identities | refuses without source-order fallback |
| repository layer replaces paths/requiredness | allowed only as a complete versioned replacement that passes every descriptor/constitutional gate |
| invocation attempts field mutation | refuses; invocation may select only an exact profile plus operation DTO fields |
| same layer bytes/definition fingerprints resolve twice | identical resolved-profile fingerprint and ordered decisions |
| selected role-registry bytes change behind the same ref | stale fingerprint refuses; a new fingerprint changes resolved-profile and vocabulary-default identity |

## Schema registry entry contract

```yaml
schema_id: handbook.schema-registry-entry
schema_version: "1.0"
content_schema_id: handbook.schemas.artifacts.charter
content_schema_version: "1.0.0"
document_ref: handbook.schemas/artifacts/charter/1.0.0.schema.json
document_fingerprint: sha256:...
closure_fingerprint: sha256:...
meta_schema_ref: https://json-schema.org/draft/2020-12/schema
media_type: application/schema+json
compatibility: exact
extensions: {}
entry_fingerprint: sha256:...
```

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies the entry shape | none | exact supported pair | not the content schema identity |
| `content_schema_id`, `content_schema_version` | schema author owns structural schema identity | none | stable ID plus SemVer; unique tuple | no filename-derived identity |
| `document_ref` | schema entry owns local resolution | none | normalized repository-relative regular file; no-follow; inside allowed roots | no URL/network fetch |
| `document_fingerprint` | Handbook derives content identity from exact bytes | none; recomputed on load | lowercase SHA-256 matches resolved document | no trust in a caller-supplied digest alone |
| `closure_fingerprint` | Handbook derives structural-schema closure identity | none | uniform hash over root document and every transitively resolved local `$ref` document fingerprint in deterministic ref order | no omitted or remote dependency |
| `meta_schema_ref` | Handbook schema policy selects the validator dialect | Draft 2020-12 only in v1 | exact allowlisted meta-schema; document passes it | no executable custom meta-schema hook |
| `media_type` | schema entry declares parser expectations | `application/schema+json` | exact allowlisted value | canonical artifact YAML is not the schema document format |
| `compatibility` | Handbook version policy owns resolution posture | `exact` | only `exact` is valid in v1 | no range/latest fallback |
| `extensions` | declaring schema owns namespaced optional additions | empty | declared namespace | no unknown required behavior |
| `entry_fingerprint` | Handbook derives exact versioned registry-entry identity | none | uniform exact-definition fingerprint including document/closure fingerprints | no caller-supplied or document-only identity |

Registry gates:

- `(content_schema_id, content_schema_version)` resolves to exactly one document and fingerprint;
- conflicting sources for the same tuple fail closed rather than using source order to mask drift;
- every local `$ref` follows the same normalized repo-relative/no-follow policy and participates in the resolved fingerprint closure;
- remote `$ref`, executable formats/hooks, unresolved refs, symlink escapes, and byte/fingerprint mismatch are refused;
- passing structural schema validation does not satisfy semantic validation, intake coverage, approval, or a contract gate.

## Stable-role registry contract

```yaml
schema_id: handbook.stable-role-registry
schema_version: "1.0"
registry_id: handbook.roles.core
registry_version: "1.0.0"
roles:
  - role_id: constitutional_authority
    canonical_display_label: Constitutional Authority
    category: governance
  - role_id: project_context
    canonical_display_label: Project Context
    category: artifact
  - role_id: coordination_horizon
    canonical_display_label: Coordination Horizon
    category: workflow
  - role_id: delivery_unit
    canonical_display_label: Delivery Unit
    category: workflow
  - role_id: implementation_unit
    canonical_display_label: Implementation Unit
    category: workflow
  - role_id: execution_envelope
    canonical_display_label: Execution Envelope
    category: workflow
  - role_id: atomic_action
    canonical_display_label: Atomic Action
    category: workflow
registry_fingerprint: sha256:...
```

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies registry shape | none | exact supported pair | not the role-registry semantic version |
| `registry_id`, `registry_version` | Handbook stable-role authority identifies one immutable version | none | stable ID plus SemVer; unique tuple | no implicit core/latest registry |
| `roles[].role_id` | stable-role authority owns machine identity | none; at least one | stable and unique within registry | vocabulary cannot add/rename a role |
| `roles[].canonical_display_label` | stable-role authority supplies deterministic fallback display text | none | non-empty normalized display string | not a profile-selected label |
| `roles[].category` | stable-role authority classifies legal uses/absorptions | none | `artifact`, `workflow`, `governance`, `evidence`, or `organizational` | category does not grant a capability |
| `registry_fingerprint` | Handbook derives exact registry identity | none | SHA-256 over normalized registry fields except this fingerprint | no mutable registry behind a stable ref |

Profiles select exactly one registry ref/fingerprint pair. Every resolved kind and vocabulary record must cite that same pair. Unknown roles, mismatched registry versions/fingerprints, category-invalid absorptions, and changed bytes behind an unchanged ref fail closed. Exact selection, not process-global code, determines canonical fallback labels and role validation.

## Artifact kind definition contract

```yaml
schema_id: handbook.artifact-kind-definition
schema_version: "1.0"
kind_id: handbook.artifact-kind.charter
kind_version: "1.0.0"
compatibility: exact
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
canonical_schema_ref: handbook.schemas.artifacts.charter@1.0.0
supported_role_refs:
  - constitutional_authority
semantic_capabilities:
  - capability_id: constitutional_root
    contract_ref: handbook.capabilities.constitutional-root@1.0.0
    bindings:
      policy_root: /policy
      policy_revision: /policy/revision
      decision_authority: /governance/decision_authority
      required_approvals: /governance/required_approvals
      exception_policy: /governance/exceptions
      engineering_posture_dimensions: /engineering_posture/dimensions
      red_lines: /engineering_posture/red_lines
      review_triggers: /governance/review_triggers
      reassessment_triggers: /governance/reassessment_triggers
structural_validation_profile_ref: json-schema.draft-2020-12
semantic_validation_profile_refs:
  - handbook.semantic-validation.constitutional-root@1.0.0
renderer_definition_refs:
  - handbook.renderer.charter-review-markdown@1.0.0
projection_definition_refs: []
lifecycle_policy_ref: handbook.lifecycle.constitutional-review-lock@1.0.0
review_triggers:
  - handbook.intake-trigger.production-posture-changed@1.0.0
  - handbook.intake-trigger.trust-boundary-changed@1.0.0
required_capabilities: []
extensions: {}
definition_fingerprint: sha256:...
```

Kind gates:

- stable kind/schema IDs and versions are unique;
- canonical schema resolves safely and passes its declared meta-schema;
- semantic capabilities cite exact contracts and valid schema bindings rather than being inferred from filenames;
- fixed renderer and future Projection refs are distinct and resolve compatible exact versions; intake definitions target kinds in the opposite direction and are not kind fields;
- repository-defined kinds pass the same definition validation as shipped kinds;
- no new Rust enum variant or CLI command is required;
- schemas contain no executable hooks or undeclared remote references.

The actual shipped kind/default-instance set is not defined by this illustrative Charter example. It is frozen only by the research and user decision in `HCM-0.6`.

`definition_fingerprint` is derived over the normalized kind record plus the stable-role registry fingerprint and the resolved fingerprints of its schema, capability contracts, semantic validators, renderer definitions, lifecycle policy, and declared extension schemas. It does not include or reference intake definitions; intake definitions point to kinds. It is emitted on the validated definition and excluded from its own hash; authors do not choose it.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies the kind record shape | none | exact supported pair | not kind semantic identity |
| `kind_id`, `kind_version` | kind author owns reusable kind identity | none | stable ID plus SemVer; unique tuple | no path/label/requiredness state |
| `compatibility` | Handbook version policy | `exact` | only `exact` in v1 | no range/latest selection |
| `stable_role_registry.ref`/fingerprint | stable-role registry owns supported-role meaning; kind author pins it | none | exact pair matches resolved profile | no ambient role validation |
| `canonical_schema_ref` | kind author selects structural content authority | none | exact schema-registry ref and fingerprint closure | schema alone does not grant semantic authority |
| `supported_role_refs` | Handbook stable-role registry owns role meaning; kind author declares compatible roles | empty | every ref is a registered stable role compatible with the kind schema | roles do not grant behavioral capabilities |
| `semantic_capabilities[].capability_id` | Handbook semantic registry owns stable capability identity | empty capability list; no per-entry default | registered capability ID | no label/filename-derived capability |
| `semantic_capabilities[].contract_ref` | Handbook capability contract owns meaning and conformance rules | none per entry | exact ref matching the capability ID | no implicit latest contract |
| `semantic_capabilities[].bindings` | kind author maps capability slots into its content schema | none per required slot | every required key maps to a valid compatible JSON Pointer | bindings do not change capability meaning |
| `structural_validation_profile_ref` | Handbook schema policy | Draft 2020-12 | exact allowlisted ref compatible with schema entry | no arbitrary executable validator |
| `semantic_validation_profile_refs` | Handbook or declared built-in semantic validators own cross-field rules | empty | exact refs; all compatible with kind/capabilities | no repository executable hook |
| `renderer_definition_refs` | renderer-definition authors own fixed deterministic pre-Phase-3 renderers | empty | exact refs; deterministic inputs declared | not a capitalized Projection |
| `projection_definition_refs` | Projection-definition authors own deterministic mapping semantics; kind author declares compatible definitions | empty | exact refs/fingerprint closure; source kind/capability selectors match and target schemas resolve | declaration does not implement or authorize a pre-Phase-3 engine |
| `lifecycle_policy_ref` | kind author selects a Handbook lifecycle policy | `null` only when lifecycle is unrestricted by the kind | exact compatible ref or explicit null | not instance lock state |
| `review_triggers` | kind author declares semantic reassessment triggers | empty | exact versioned trigger refs with fingerprint producers; no duplicates | no direct transport notification behavior |
| `required_capabilities[].capability_id` | kind author declares a cross-artifact semantic prerequisite | empty list; no per-entry default | registered capability ID | no filename dependency |
| `required_capabilities[].contract_ref` | kind author binds the prerequisite to one exact capability contract | none per entry | exact ref whose capability ID matches; one exact version | no range/latest or provider-specific fallback |
| `required_capabilities[].cardinality` | capability contract/kind author declares how many distinct resolved instances must satisfy the prerequisite | none per entry | `exactly_one` or `at_least_one`; count is satisfiable | not shipped-default selection |
| `extensions` | kind schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required semantics |
| derived `definition_fingerprint` | Handbook resolver identifies the complete resolved definition | no authored default | deterministic SHA-256 over semantic closure | no timestamp or absolute-path input |

### Semantic capability contract

```yaml
schema_id: handbook.semantic-capability-contract
schema_version: "1.0"
contract_id: handbook.capabilities.constitutional-root
contract_version: "1.0.0"
capability_id: constitutional_root
required_bindings:
  - policy_root
  - policy_revision
  - decision_authority
  - required_approvals
  - exception_policy
  - engineering_posture_dimensions
  - red_lines
  - review_triggers
  - reassessment_triggers
semantic_validation_profile_refs:
  - handbook.semantic-validation.constitutional-root@1.0.0
allowed_instance_cardinality: exactly_one
extensions: {}
capability_fingerprint: sha256:...
```

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies capability-contract shape | none | exact supported pair | not capability semantic identity |
| `contract_id`, `contract_version` | Handbook semantic registry identifies one exact versioned contract definition | none | namespaced ID plus SemVer; exact ref derives as `contract_id@contract_version`; unique tuple | no implicit latest or ID reconstruction |
| `capability_id` | Handbook semantic registry owns stable capability meaning selected by instances | none | registered stable capability ID; contract ref resolves this same ID | no user label/filename authority |
| `required_bindings` | capability contract owns required semantic slots | none | unique registered binding keys | does not prescribe one artifact schema layout |
| `semantic_validation_profile_refs` | capability contract owns conformance invariants | none for constitutional authority | exact built-in/declarative refs | no repository executable hook |
| `allowed_instance_cardinality` | capability contract owns resolved-profile cardinality | explicit | `exactly_one` or `at_least_one`; `constitutional_root` is `exactly_one` | not artifact-kind or shipped-default cardinality |
| `extensions` | capability schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |
| `capability_fingerprint` | Handbook derives exact capability-contract identity | none | uniform exact-definition fingerprint including every semantic-validator fingerprint | no stable ref with mutable contract meaning |

A kind's `semantic_capabilities[].bindings` must provide every required binding as a valid JSON Pointer into the exact canonical content schema. Required slots may map to differently named/schema-shaped fields, but the declared semantic validators must prove the capability invariants. This is how custom kinds can conform without copying Charter's filename or schema layout.

V1 cardinality has exactly two values. `exactly_one` requires one distinct resolved provider instance. `at_least_one` requires one or more and resolves the complete provider set in stable instance-ID order; it never chooses one provider implicitly. A kind may contain at most one `semantic_capabilities` entry for a capability ID. Multiple registry versions may exist, but a kind selection or dependency always cites one exact contract version, and the capability ID in the contract must match. Duplicate declarations, zero providers, excess providers for `exactly_one`, mismatched versions, and any definition-closure cycle fail closed.

## Artifact instance descriptor contract

```yaml
schema_id: handbook.artifact-instance-descriptor
schema_version: "1.0"
id: project_context
kind_ref: handbook.artifact-kind.project-context@1.0.0
role_ref: project_context
capability_refs: []
label: Project Context
canonical_path: .handbook/project_context/project-context.yaml
requiredness:
  mode: always
  condition_ref: null
depends_on:
  - target_kind: capability
    target_ref: constitutional_root
    target_contract_ref: handbook.capabilities.constitutional-root@1.0.0
    cardinality: exactly_one
lifecycle_policy_ref: null
intake_definition_ref: null
renderer_definition_refs:
  - handbook.renderer.project-context-review-markdown@1.0.0
projection_definition_refs: []
validation_overlay_refs: []
extensions: {}
```

Instance gates:

- `schema_id` and `schema_version` identify the descriptor record contract independently from the referenced artifact-kind version;
- `kind_ref` resolves exactly;
- path, label, requiredness, and dependencies are repository-instance concerns rather than kind-definition fields;
- selected `role_ref` is supported by the kind and every selected `capability_ref` has a kind capability contract;
- dependencies resolve exact instance IDs or exact semantic-capability contracts without cycles, source-order fallback, or ambiguous provider selection;
- overlays cannot weaken kind schema, constitutional floors, or red lines.

Custom instances use generic operations. Specialized first-party behavior must key off stable capability/role semantics, not filename matching.

The `project_context` descriptor above is a shape example only. It does not select a shipped kind, instance, path, label, renderer, or requiredness decision.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies descriptor shape | none | exact supported pair | not kind schema identity |
| `id` | profile author owns repository instance identity | none | unique stable ID | no filename-derived identity |
| `kind_ref` | profile author selects reusable kind | none | exact resolvable kind and definition fingerprint | no version range/latest fallback |
| `role_ref` | profile author selects one stable semantic role | explicit registered value or explicit `null` | referenced kind lists the role in `supported_role_refs` | role selection does not grant a capability |
| `capability_refs` | profile author selects behavioral/conformance capabilities exposed by the kind | explicit list, possibly empty | unique registered IDs; kind supplies each exact capability contract/binding | capability IDs are not vocabulary roles |
| `label` | profile vocabulary/presentation authority owns instance display text | none | non-empty display string | does not rename ID, path, schema, command, or capability |
| `canonical_path` | profile author owns concrete repository placement | none | normalized repo-relative no-follow path; unique among instances; no template remains after resolution | no absolute path or filename dispatch |
| `requiredness.mode` | profile author owns `always`, `conditional`, or `optional` | none | allowlisted enum; constitutional root is `always` | no inference from current setup/templates |
| `requiredness.condition_ref` | profile author selects deterministic project-condition truth | `null` unless mode is `conditional` | exact condition ref; required iff conditional | no arbitrary expression/script |
| `depends_on[].target_kind` | profile author selects dependency namespace | empty dependency list; no per-entry default | `instance` or `capability` | no ambiguous bare string |
| `depends_on[].target_ref` | profile author selects exact instance ID or registered capability ID | none per entry | matches target namespace and resolves | no filename dependency |
| `depends_on[].target_contract_ref` | profile author binds a capability dependency to one exact contract | exact ref when `target_kind=capability`; explicit `null` when `instance` | contract capability ID matches `target_ref`; every provider exposes that exact contract | no compatible-range or source-order provider selection |
| `depends_on[].cardinality` | profile author declares required target count | none per entry | `exactly_one` or `at_least_one` for capability; only `exactly_one` for unique instance ID; acyclic and satisfiable | not artifact requiredness or an implicit provider choice |
| `lifecycle_policy_ref` | profile author selects instance policy override | explicit exact ref or `null` | compatible with kind; cannot weaken kind/constitutional policy | not current mutable lock state |
| `intake_definition_ref` | profile author explicitly selects intake | exact ref or explicit `null`; no implicit inheritance | definition's exact `artifact_kind_ref` equals selected kind; capability-required constitutional coverage cannot be disabled | kind does not list/fingerprint intakes; no prompt-owned behavior |
| `renderer_definition_refs` | profile author explicitly selects fixed renderers | explicit list, possibly empty | subset of compatible kind refs | not Projection selection |
| `projection_definition_refs` | profile author selects compatible catalog definitions for this instance | explicit list, possibly empty | subset of kind-compatible definitions; exact refs/fingerprints; profile catalog contains each selection | selection does not create a Phase-2 view or executable hook |
| `validation_overlay_refs` | profile author selects declarative repository constraints | empty | exact schema/declarative refs; only equal/stricter rules | no executable hook or weakened floor |
| `extensions` | descriptor schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |

The constitutional-root instance adds these gates: it is the only instance whose `capability_refs` contains `constitutional_root`, its requiredness is `always`, its kind capability bindings satisfy the versioned constitutional contract, and no overlay, condition, role/vocabulary mapping, or adapter can remove or weaken that authority. Its `role_ref` remains a separate registered role, normally `constitutional_authority`.

Dependency conformance scenarios required by implementation packets include: zero/one/two providers against both cardinalities; an exact contract-version match and mismatch; duplicate same-capability declarations in one kind; deterministic ordering of multiple `at_least_one` providers; and refusal when an instance dependency uses any cardinality other than `exactly_one`.

## Artifact intake definition contract

```yaml
schema_id: handbook.artifact-intake-definition
schema_version: "1.0"
intake_id: handbook.intake.charter
intake_version: "1.0.0"
artifact_kind_ref: handbook.artifact-kind.charter@1.0.0
candidate_schema_ref: handbook.schemas.artifacts.charter@1.0.0
supported_modes:
  - guided_adaptive
  - express
  - agent_assisted
coverage:
- coverage_id: project_shape.definition
  target_paths:
  - /project_shape
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: delivery.constraints
  target_paths:
  - /delivery_constraints
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: delivery.default_implications
  target_paths:
  - /default_implications/backward_compatibility
  - /default_implications/migration_planning
  - /default_implications/rollout_controls
  - /default_implications/deprecation_policy
  - /default_implications/observability_threshold
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: operational_reality.production_state
  target_paths:
  - /project_conditions/production_state
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: risk.domains
  target_paths:
  - /risk_domains
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: policy.authority_and_revision
  target_paths:
  - /policy
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.decision_authority
  target_paths:
  - /governance/decision_authority
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: named_role_or_owner
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.required_approvals
  target_paths:
  - /governance/required_approvals
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.exception_policy
  target_paths:
  - /governance/exceptions
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: engineering_posture.dimensions
  target_paths:
  - /engineering_posture/dimensions
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_levels
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: engineering_posture.red_lines
  target_paths:
  - /engineering_posture/red_lines
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.review_triggers
  target_paths:
  - /governance/review_triggers
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.reassessment_triggers
  target_paths:
  - /governance/reassessment_triggers
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: debt.register
  target_paths:
  - /debt
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: decisions.records
  target_paths:
  - /decision_records
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
approval_policy_ref: handbook.approval.constitutional-candidate@1.0.0
reassessment_triggers:
  - trigger_ref: handbook.intake-trigger.production-posture-changed@1.0.0
    affected_coverage_ids:
      - operational_reality.production_state
  - trigger_ref: handbook.intake-trigger.trust-boundary-changed@1.0.0
    affected_coverage_ids:
      - governance.exception_policy
extensions: {}
intake_definition_fingerprint: sha256:...
```

The exact illustrative Charter intake is coverage-complete for the frozen constitutional capability and retained Charter domains:

| Required constitutional binding | Required coverage ID |
|---|---|
| `policy_root`, `policy_revision` | `policy.authority_and_revision` |
| `decision_authority` | `governance.decision_authority` |
| `required_approvals` | `governance.required_approvals` |
| `exception_policy` | `governance.exception_policy` |
| `engineering_posture_dimensions` | `engineering_posture.dimensions` |
| `red_lines` | `engineering_posture.red_lines` |
| `review_triggers` | `governance.review_triggers` |
| `reassessment_triggers` | `governance.reassessment_triggers` |

| Retained Charter domain | Required coverage ID |
|---|---|
| project shape | `project_shape.definition` |
| delivery constraints | `delivery.constraints` |
| default delivery implications (backward compatibility, migration planning, rollout controls, deprecation policy, observability threshold) | `delivery.default_implications` |
| operational reality | `operational_reality.production_state` |
| risk domains | `risk.domains` |
| posture and engineering dimensions | `engineering_posture.dimensions`, `engineering_posture.red_lines` |
| exceptions and governance | `governance.decision_authority`, `governance.required_approvals`, `governance.exception_policy`, `governance.review_triggers`, `governance.reassessment_triggers` |
| debt | `debt.register` |
| decision records | `decisions.records` |

Meta-validation requires every constitutional binding and every retained domain in these matrices to resolve at least one required coverage item whose target path equals or is an ancestor of the bound schema pointer. Every populated candidate field is source-mapped. Missing, optionalized, unmapped, or conflicting coverage refuses initial promotion.

Intake gates:

- every v1 coverage item maps to one or more valid candidate-schema paths;
- constitutional intake meta-validation proves complete required-binding and retained-domain coverage before the definition fingerprint is admitted;
- shorter modes cannot weaken required coverage or hide unknowns;
- inferable observations record evidence, confidence, freshness, and sensitivity;
- normative/constitutional decisions identify the authority required to approve them;
- question wording may vary by skill/projection, but stable coverage IDs and evaluation semantics do not;
- no intake definition embeds a model/provider call.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies definition shape | none | exact supported pair | not intake semantic identity |
| `intake_id`, `intake_version` | intake author owns coverage-contract identity | none | stable ID plus SemVer; unique tuple | no prompt/version inference |
| `artifact_kind_ref` | intake author selects target kind | none | exact kind ref | no cross-kind candidate mutation |
| `candidate_schema_ref` | intake author selects candidate structural shape | none | exact schema ref matching the kind's canonical schema unless a reviewed normalization contract says otherwise | no weaker mode-specific schema |
| `supported_modes` | intake author permits acquisition paths | none; at least one | subset of `guided_adaptive`, `express`, `agent_assisted`; unique | mode does not change quality bar |
| `coverage` | intake author owns stable coverage semantics | none; at least one for rich intake | unique IDs; deterministic dependency/applicability graph | not terminal question order |
| `coverage[].coverage_id` | intake author owns semantic coverage identity | none | stable unique ID | wording is not identity |
| `coverage[].target_paths` | intake author maps coverage to candidate fields | none; non-empty in v1 | unique valid JSON Pointers in candidate schema | rationale-only/evidence-only non-field coverage awaits a later typed disposition contract; no implicit free-text placement |
| `coverage[].applicability` | intake author owns deterministic applicability | none | `always` or exact declarative condition ref | no script/model judgment |
| `coverage[].authority_class` | Handbook authority taxonomy classifies the value | none | `observational`, `rationale`, or `normative` | evidence does not become normative authority |
| `coverage[].acquisition.inferable` | intake author allows evidence-backed inference | `false` | boolean consistent with authority class | normative policy is not silently inferable |
| `coverage[].acquisition.user_declaration_required` | intake author marks declaration authority | `false` only when omission is explicit | boolean; normative constitutional fields require it unless the approval policy supplies stronger authority | no mode-specific bypass |
| `coverage[].acquisition.evidence_kinds` | intake author allows evidence classes | empty | registered evidence kinds | no embedded validator execution |
| `coverage[].acquisition.freshness` | intake author sets evidence freshness | `null` means no time-based claim, not indefinitely fresh | registered freshness rule | no wall-clock-only authority |
| `coverage[].acquisition.sensitivity` | intake author sets handling class | `internal` | registered sensitivity class and redaction policy | no unrestricted secret capture |
| `coverage[].acquisition.deterministic_default` | intake author may declare a non-inferred default with authority effect | `null` | value validates; source is recorded; forbidden where declaration/approval is required | no silent constitutional default |
| `coverage[].evaluation.required` | intake author sets promotion coverage requirement | `false` | boolean; shorter modes cannot change it | not artifact-instance requiredness |
| `coverage[].evaluation.minimum_specificity` | intake author sets deterministic quality floor | none when required | registered evaluator | no subjective prompt-only judgment |
| `coverage[].evaluation.minimum_confidence` | intake author sets the evidence/declaration confidence floor | none when required | registered ordered confidence value; source kind may impose a higher floor | no agent confidence as authority by itself |
| `coverage[].evaluation.unknown_policy` | intake author sets `block`, `allow_explicit`, or `require_waiver` | `block` for required coverage; otherwise omission is invalid | allowlisted value compatible with authority class and approval policy | no hidden unknown |
| `coverage[].evaluation.contradiction_policy` | intake author sets `block`, `require_resolution`, or `record_warning` | `block` for required normative coverage; otherwise omission is invalid | allowlisted value compatible with authority class | no silent last-write-wins |
| `coverage[].evaluation.waiver_policy_ref` | intake author selects waiver authority when waivers are permitted | `null` means not waivable | exact compatible policy ref | waiver does not equal satisfied truth |
| `coverage[].prompt_guidance_refs` | skill/presentation contract owns optional wording guidance | empty | safe exact refs; guidance cannot change coverage/evaluation | no prompt text as semantic identity |
| `approval_policy_ref` | intake author selects promotion authority policy | exact ref or `null` for non-governed kinds | compatible with kind/capabilities; constitutional intake requires non-null | no agent self-approval |
| `reassessment_triggers[].trigger_ref` | intake author declares a targeted reopen condition | empty trigger list; no per-entry default | exact versioned trigger definition with fingerprint producer; unique | no implicit whole-artifact reassessment |
| `reassessment_triggers[].affected_coverage_ids` | intake author owns the exact coverage subset reopened by the trigger | none per entry; non-empty | every ID exists in this definition; unique and deterministically ordered | an unrelated coverage item does not reopen |
| `extensions` | intake schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |
| `intake_definition_fingerprint` | Handbook derives exact intake-definition identity | none | uniform exact-definition fingerprint including target kind/schema, approval policy, and trigger-definition fingerprints | no stable intake ref with mutable coverage/authority semantics |

Typed coverage results are `satisfied`, `unknown`, `contradicted`, `waived`, `not_applicable`, or `blocked`. `not_applicable` includes applicability evidence; `waived` includes exact waiver authority and expiry/review rules; neither is equivalent to `satisfied`. Required `unknown`, `contradicted`, or `blocked` coverage prevents promotion.

## Intake record and artifact candidate contracts

```yaml
schema_id: handbook.artifact-intake-record
schema_version: "1.0"
intake_record_id: intake_...
intake_definition_ref: handbook.intake.charter@1.0.0
acquisition_mode: guided_adaptive
target_kind_ref: handbook.artifact-kind.charter@1.0.0
target_instance_id: example_constitutional_root
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
consumer:
  kind: handbook_skill
  id: handbook
  version: "..."
coverage_results:
- coverage_id: project_shape.definition
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/project_shape.definition.json
  evidence_refs:
  - evidence.project_shape.definition
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: delivery.constraints
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/delivery.constraints.json
  evidence_refs:
  - evidence.delivery.constraints
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: delivery.default_implications
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/delivery.default_implications.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: operational_reality.production_state
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/operational_reality.production_state.json
  evidence_refs:
  - evidence.operational_reality.production_state
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: risk.domains
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/risk.domains.json
  evidence_refs:
  - evidence.risk.domains
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: policy.authority_and_revision
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/policy.authority_and_revision.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.decision_authority
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.decision_authority.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.required_approvals
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.required_approvals.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.exception_policy
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.exception_policy.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: engineering_posture.dimensions
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/engineering_posture.dimensions.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: engineering_posture.red_lines
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/engineering_posture.red_lines.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.review_triggers
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.review_triggers.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.reassessment_triggers
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.reassessment_triggers.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: debt.register
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/debt.register.json
  evidence_refs:
  - evidence.debt.register
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: decisions.records
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/decisions.records.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
prompt_event_refs: []
finalized_at_utc: "..."
record_fingerprint: sha256:...
```

`source_kind` is exactly one of `user_declaration`, `evidenced_inference`, `deterministic_default`, `known_unknown`, `contradiction`, or `waiver`. Approval is a separate authority record, never a value source. Prompt events are descriptive audit references and do not become coverage identity or canonical truth.

```yaml
schema_id: handbook.artifact-candidate
schema_version: "1.0"
candidate_id: candidate_...
intake_record_ref: intake_...
target_kind_ref: handbook.artifact-kind.charter@1.0.0
target_instance_id: example_constitutional_root
target_schema_ref: handbook.schemas.artifacts.charter@1.0.0
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
normalized_content_ref: candidates/...yaml
field_sources:
- target_path: /project_shape
  coverage_id: project_shape.definition
  source_kind: evidenced_inference
- target_path: /delivery_constraints
  coverage_id: delivery.constraints
  source_kind: evidenced_inference
- target_path: /default_implications/backward_compatibility
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/migration_planning
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/rollout_controls
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/deprecation_policy
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/observability_threshold
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /project_conditions/production_state
  coverage_id: operational_reality.production_state
  source_kind: evidenced_inference
- target_path: /risk_domains
  coverage_id: risk.domains
  source_kind: evidenced_inference
- target_path: /policy
  coverage_id: policy.authority_and_revision
  source_kind: user_declaration
- target_path: /governance/decision_authority
  coverage_id: governance.decision_authority
  source_kind: user_declaration
- target_path: /governance/required_approvals
  coverage_id: governance.required_approvals
  source_kind: user_declaration
- target_path: /governance/exceptions
  coverage_id: governance.exception_policy
  source_kind: user_declaration
- target_path: /engineering_posture/dimensions
  coverage_id: engineering_posture.dimensions
  source_kind: user_declaration
- target_path: /engineering_posture/red_lines
  coverage_id: engineering_posture.red_lines
  source_kind: user_declaration
- target_path: /governance/review_triggers
  coverage_id: governance.review_triggers
  source_kind: user_declaration
- target_path: /governance/reassessment_triggers
  coverage_id: governance.reassessment_triggers
  source_kind: user_declaration
- target_path: /debt
  coverage_id: debt.register
  source_kind: evidenced_inference
- target_path: /decision_records
  coverage_id: decisions.records
  source_kind: user_declaration
validation_result_refs:
  - validation_...
unresolved_coverage_ids: []
promotion_eligibility: requires_approval
required_approval_policy_ref: handbook.approval.constitutional-candidate@1.0.0
candidate_fingerprint: sha256:...
```

```yaml
schema_id: handbook.artifact-approval-record
schema_version: "1.0"
approval_id: approval_...
candidate_ref: candidate_...
candidate_fingerprint: sha256:...
approval_policy_ref: handbook.approval.constitutional-candidate@1.0.0
decision: approved
actor_ref: user.project_owner
authority_ref: charter.governance.project_owner
conditions: []
decided_at_utc: "..."
approval_fingerprint: sha256:...
```

```yaml
schema_id: handbook.artifact-promotion-record
schema_version: "1.0"
promotion_id: promotion_...
candidate_ref: candidate_...
candidate_fingerprint: sha256:...
target_instance_id: example_constitutional_root
expected_current_artifact_fingerprint: null
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
resolved_definitions:
  - definition_ref: handbook.artifact-kind.charter@1.0.0
    definition_fingerprint: sha256:...
approval_refs:
  - approval_...
validation_result_refs:
  - validation_...
decision: approved
authorized_by_ref: charter.governance.project_owner
canonical_artifact_ref: .handbook/example/root.yaml
canonical_artifact_fingerprint: sha256:...
promotion_fingerprint: sha256:...
```

| Record/fields | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| intake record identity/definition/mode/target, `profile_ref`, `resolved_profile_fingerprint` | Handbook intake engine records the exact acquisition envelope | none | exact refs and supported mode; current fully resolved profile fingerprint | no source-layer fingerprint or mutable session scratchpad as authority |
| `consumer` and `prompt_event_refs` | adapter supplies provenance; Handbook records it | prompt refs empty | typed identity/version; safe refs | agent wording does not define coverage |
| `coverage_results` | Handbook evaluator owns typed per-item result; source records own declared/observed values | one result per applicable/considered item | definition coverage ID; allowed source; evidence/confidence/freshness/sensitivity; typed evaluation | no collapse of declaration, inference, default, unknown, contradiction, or waiver |
| intake `finalized_at_utc` and `record_fingerprint` | Handbook closes the record after acquisition/evaluation and derives immutable provenance identity | none | valid UTC plus deterministic normalized record | no later candidate/approval/promotion forward link or mutation |
| candidate identity/target/schema, `profile_ref`, `resolved_profile_fingerprint` | Handbook candidate builder owns normalized proposal identity | none | exact refs/fingerprints match intake and current fully resolved profile | candidate is not canonical truth |
| `normalized_content_ref` | candidate record owns content-addressed proposal ref | none | safe repo/content-store ref; bytes validate | no untracked inline model output |
| `field_sources` | Handbook maps candidate fields to coverage and value sources | none for populated fields | every populated field has a valid mapping or declared deterministic derivation | no source-free policy value |
| candidate validation/gaps/eligibility/approval policy | Handbook validators and intake policy own readiness | empty gaps only when actually complete; eligibility is explicit | current structural, semantic, coverage, and authority checks | no shorter-mode quality exemption |
| `candidate_fingerprint` | Handbook derives proposal identity | none | normalized content plus exact semantic refs | no timestamp/presentation-only input |
| approval identity/candidate/policy/decision | declared approval policy and authorized actor own a decision over one immutable candidate | none | candidate fingerprint matches; policy current; decision `approved` or `rejected` | approval does not rewrite candidate/intake |
| approval actor/authority/conditions/time | authorized human/role supplies authority; Handbook records event | conditions empty; other fields explicit | actor satisfies authority/policy; conditions validate; UTC valid | agent/validator cannot self-authorize constitutional truth |
| `approval_fingerprint` | Handbook derives immutable approval identity | none | deterministic normalized approval record | no mutable approval state |
| promotion candidate/current, `profile_ref`, `resolved_profile_fingerprint`, and definition fields | Handbook transition engine binds compare-and-write preconditions | none; expected current fingerprint may be explicit null only for create | candidate unchanged; target state/fully resolved profile/definitions current | no stale-candidate overwrite or source-layer fingerprint |
| promotion approvals/authorization/decision | declared approval policy and authorized human/role own authority | no approvals; decision cannot be approved without required refs | approver satisfies current policy; decision allowlisted | agent/validator cannot self-authorize constitutional truth |
| promotion validation refs | Handbook validators own proof of current checks | none | structural, semantic, coverage, authority, and path/write preconditions pass | no reuse of stale validation after drift |
| canonical artifact ref/fingerprint | canonical artifact store owns approved truth after atomic write | absent unless approved write succeeds | safe concrete path; bytes validate; fingerprint matches | promotion record is not a second editable artifact |
| `promotion_fingerprint` | Handbook derives immutable transition identity | none | deterministic normalized record | no wall-clock identity |

Promotion re-resolves the exact profile, kind, schema, intake, capability, renderer, lifecycle, and approval-policy refs; compares the candidate and current target fingerprints; reruns every current validation layer; writes canonical YAML atomically; then emits the immutable promotion record. Any drift, missing approval, unresolved required coverage, unknown required semantic, or failed write leaves canonical truth unchanged and returns a typed refusal.

Record finalization and lineage are strictly append-only:

1. finalize and fingerprint `ArtifactIntakeRecord` after coverage evaluation;
2. build/finalize `ArtifactCandidate` with `intake_record_ref`;
3. write one or more immutable approval records with candidate ref/fingerprint;
4. write promotion only after validating the candidate and required approvals;
5. derive any forward index from downstream refs without rewriting prior records.

Required conformance scenarios include: an empty `target_paths` item refuses in v1; one trigger reopens only its non-empty mapped coverage set; unknown/unmapped triggers refuse; creating a candidate/approval/promotion leaves prior record bytes and fingerprints unchanged; reordered or stale lineage refuses; rejected/wrong-authority approval cannot promote; changing a parent/repository profile layer while retaining the selected `profile_ref` invalidates the old `resolved_profile_fingerprint`; and current candidate/target/fully-resolved-profile/definition fingerprints are required at compare-and-write.

## Charter intake and canonical contract

`CharterIntakeDefinition` is the first rich first-party intake definition. Its coverage must account for the historical domains of project shape, delivery constraints, operational reality, posture and delivery implications, risk domains, engineering dimensions, exceptions/governance, debt, and decision records. Research/design may revise questions and branches, but omissions are explicit decisions. This contract does not decide the shipped default kind/instance set reserved for HCM-0.6.

Guided-adaptive, express, and agent-assisted modes all produce the same canonical Charter candidate schema. The skill-directed LLM agent conducts the conversation and invokes Handbook CLI/SDK operations; Handbook owns coverage/evaluation/promotion and performs no hidden nested synthesis.

Canonical Charter YAML owns approved constitutional truth. Intake provenance explains how it was reached. Before Phase 3, Markdown and other fixed human-review outputs are renderer-derived views; Phase-3 Resolution-aware GUI, packet, and agent-context outputs are Projections. Neither output class is an independently editable authority.

The versioned `constitutional_root` capability contract requires schema bindings for:

- policy root and policy revision;
- decision authority and required approval classes;
- exception/waiver authority and record requirements;
- effective engineering-posture dimensions, floors, and red lines;
- review and targeted reassessment triggers.

Exactly one resolved artifact instance selects this capability and is `always` required. A custom kind may expose the capability only when its exact capability contract, bindings, semantic validators, intake/approval posture, and lifecycle policy pass. A role ref, label, filename, current enum variant, or requiredness flag alone never grants constitutional authority.

Charter-specific defaults are intentionally narrow: absent optional rationale/evidence collections are empty only when the Charter content schema allows it; policy, authority, posture floors/red lines, required approvals, and required coverage have no implicit default. The HCM-0.6 decision may choose a shipped binding/path/label but cannot weaken these semantics.

Explicit non-goals are a rigid CLI questionnaire, prompt-owned policy, agent self-approval, Markdown authority, a second editable posture document, automatic whole-Charter regeneration on one changed condition, or default-set selection by example.

## Artifact validation layers

Do not ask one schema mechanism to own every kind of correctness:

1. **Structural schema** — fields, types, enumerations, requiredness, and local shape after YAML parsing.
2. **Semantic validation** — cross-field, cross-artifact, lifecycle, capability, and authority invariants owned by Handbook.
3. **Intake coverage evaluation** — whether required information was established with appropriate evidence, specificity, confidence, and approval.
4. **External validator docks** — domain-specific witnesses that emit normalized evidence without becoming artifact or contract authority.

A custom kind may need only structural and semantic validation. Human-authored constitutional/governance kinds commonly add intake coverage. Docks remain optional unless the kind/profile/contract requires them.

In Phase 2, a fixed deterministic first-party renderer may produce a renderer-derived human-review view only from validated canonical truth. That view accepts no Context Resolution envelope and is outside the capitalized Phase-3 `Projection` request/result/provenance contract.

Validation order is structural schema -> Handbook semantic/capability validation -> intake coverage/approval when applicable -> required external evidence gates when a later contract demands them. Passing a later layer cannot waive an earlier failure. Unknown required semantics fail closed; warnings never promote blocked truth.

## Vocabulary contract

The vocabulary schema distinguishes labels, aliases, and structural absorption:

```yaml
schema_id: handbook.vocabulary-profile
schema_version: "1.0"
vocabulary_id: handbook.vocabulary.example
vocabulary_version: "1.0.0"
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
labels:
  coordination_horizon: phase
  delivery_unit: feature
  implementation_unit: slice
  execution_envelope: packet
  atomic_action: task
aliases:
  implementation_unit:
    - task
absorptions:
  - unit_id: implementation
    absorbs:
      - implementation_unit
      - execution_envelope
      - atomic_action
extensions: {}
vocabulary_fingerprint: sha256:...
```

Duplicate displayed labels are legal. Ambiguity matters only when a machine operation cannot resolve a stable role from the surrounding typed context.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies vocabulary record shape | none | exact supported pair | not vocabulary semantic identity |
| `vocabulary_id`, `vocabulary_version` | vocabulary author owns one versioned mapping | none | stable ID plus SemVer; unique tuple | no implicit local vocabulary |
| `stable_role_registry.ref`/fingerprint | stable-role registry owns keys/default labels/categories; vocabulary author pins it | none | exact pair matches resolved profile and consuming kinds | no ambient registry or mutable fallback label |
| `labels` | vocabulary author owns display labels for registered stable roles | missing role uses the stable-role registry's canonical display label | keys are registered roles; values non-empty; duplicate values allowed | no change to stable role meaning or machine IDs |
| `aliases` | vocabulary author owns accepted display/search aliases | empty map/list | keys registered; aliases normalized; ambiguity surfaced in untyped input | aliases do not select typed operations by themselves |
| `absorptions[].unit_id` | vocabulary author names one local workflow unit | none | stable local ID; unique | not a new Handbook stable role |
| `absorptions[].absorbs` | vocabulary author explicitly combines registered workflow roles for presentation/execution mapping | empty absorptions | registered roles; directed acyclic mapping; one owning absorption per role; adapter preservation checked | cannot absorb constitutional authority, erase approval/evidence boundaries, or change schemas/commands |
| `extensions` | vocabulary schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |
| `vocabulary_fingerprint` | Handbook derives exact vocabulary identity | none | normalized SHA-256 over all semantic fields plus stable-role registry fingerprint except this field | no unchanged fingerprint after registry/label/absorption drift |

Stable role resolution always uses typed context first. Untyped ambiguous input returns candidates or a typed refusal. Profile vocabulary never renames schema IDs, profile/kind/instance IDs, capability IDs, SDK/JSON operation IDs, or CLI commands. An adapter that cannot preserve structural absorption reports typed loss/refusal rather than silently flattening it.

## Context Resolution stack definition

```yaml
schema_id: handbook.context-resolution-stack-definition
schema_version: "1.0"
stack_id: handbook.context-resolution.example
stack_version: "1.0.0"
levels:
  - level_id: strategic
    display_label: Strategic
    defaults:
      scope_horizon: program
      detail_resolution: full
      temporal_horizon: long_range
      authority_horizon: program_policy
      memory_horizon: strategic
      validation_horizon: program_gate
  - level_id: coordination
    display_label: Coordination
    defaults:
      scope_horizon: slice
      detail_resolution: normal
      temporal_horizon: current_slice
      authority_horizon: slice_write
      memory_horizon: coordination
      validation_horizon: slice_closeout
  - level_id: execution
    display_label: Execution
    defaults:
      scope_horizon: assigned_unit
      detail_resolution: normal
      temporal_horizon: immediate
      authority_horizon: local_write
      memory_horizon: execution
      validation_horizon: unit_closeout
  - level_id: operation
    display_label: Operation
    defaults:
      scope_horizon: local_observation
      detail_resolution: identifier_only
      temporal_horizon: current_operation
      authority_horizon: read_only
      memory_horizon: operation
      validation_horizon: observation_only
dimension_domains:
  scope_horizon:
    - { value_id: local_observation, rank: 0 }
    - { value_id: assigned_unit, rank: 1 }
    - { value_id: slice, rank: 2 }
    - { value_id: program, rank: 3 }
  detail_resolution:
    - { value_id: identifier_only, rank: 0 }
    - { value_id: summary, rank: 1 }
    - { value_id: normal, rank: 2 }
    - { value_id: full, rank: 3 }
  temporal_horizon:
    - { value_id: current_operation, rank: 0 }
    - { value_id: immediate, rank: 1 }
    - { value_id: current_slice, rank: 2 }
    - { value_id: long_range, rank: 3 }
  authority_horizon:
    - { value_id: read_only, rank: 0 }
    - { value_id: local_write, rank: 1 }
    - { value_id: slice_write, rank: 2 }
    - { value_id: program_policy, rank: 3 }
  memory_horizon:
    - { value_id: operation, rank: 0 }
    - { value_id: execution, rank: 1 }
    - { value_id: coordination, rank: 2 }
    - { value_id: strategic, rank: 3 }
  validation_horizon:
    - { value_id: observation_only, rank: 0 }
    - { value_id: unit_closeout, rank: 1 }
    - { value_id: slice_closeout, rank: 2 }
    - { value_id: program_gate, rank: 3 }
mutation_matcher:
  ref: handbook.mutation-matcher.core@1.0.0
  fingerprint: sha256:...
escalation_policy:
  ref: handbook.resolution-escalation.core@1.0.0
  fingerprint: sha256:...
memory_promotion_policy:
  ref: handbook.memory-promotion.core@1.0.0
  fingerprint: sha256:...
extensions: {}
definition_fingerprint: sha256:...
```

This is a complete internally consistent conformance example, not a shipped stack decision. HCM-0.3 freezes the contract and comparison semantics without selecting product labels or a shipped stack.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| schema identity | Handbook schema registry | none | exact supported pair | not stack semantic version |
| `stack_id`, `stack_version` | stack author owns semantic identity | none | stable ID plus SemVer; unique tuple; exact ref derived from both | no implicit `default`/latest stack |
| `levels` | stack author owns the named linear stack and defaults | none; at least one | unique stable IDs in declared broad-to-narrow order; each has all six valid defaults | no arbitrary graph or L0-L3 inference |
| `dimension_domains` | stack author owns profile-specific value IDs and total order | none; exactly six non-empty domains | every domain has contiguous unique ranks from zero; rank zero grants least reach/disclosure/durability/claim authority | no seventh aggregate score or reversed per-profile privilege meaning |
| level `defaults` | stack author selects one value from each domain | none | exactly six values; every value belongs to its domain; a narrower level cannot default to a higher rank than its predecessor | defaults do not override an envelope's explicit complete dimensions |
| policy/matcher refs | named Handbook definitions own comparison, selector, escalation, and promotion semantics | none | exact refs/fingerprints; compatible versions; no executable/remote hook | no invocation-selected policy |
| `extensions` | definition schema owns namespaced optional additions | empty | declared optional namespaces | no unknown required semantics |
| `definition_fingerprint` | Handbook derives exact stack identity | none | RFC 8785/SHA-256 over normalized record and exact referenced-definition fingerprints except itself | no timestamp, path, or display-only drift hidden behind an unchanged fingerprint |

## Context Resolution envelope

```yaml
schema_id: handbook.context-resolution-envelope
schema_version: "1.0"
envelope_id: envelope.example.execution
resolved_profile:
  ref: handbook.profile.example@1.0.0
  fingerprint: sha256:...
resolution_stack:
  ref: handbook.context-resolution.example@1.0.0
  fingerprint: sha256:...
active_level_id: execution
objective_ref: work.unit.example
dimensions:
  scope_horizon: assigned_unit
  detail_resolution: normal
  temporal_horizon: immediate
  authority_horizon: local_write
  memory_horizon: execution
  validation_horizon: unit_closeout
parent_envelope: null
constraint_inputs:
  - ref: contract.example@1.0.0
    fingerprint: sha256:...
mutation_rules:
  - rule_id: allow_unit_files
    effect: allow
    target_kind: repository_path
    selector: crates/example/**
  - rule_id: deny_canonical_policy
    effect: deny
    target_kind: repository_path
    selector: docs/canon/**
escalation_triggers:
  - trigger_id: parent_contract_conflict
    policy_ref: handbook.resolution-trigger.parent-contract-conflict@1.0.0
    policy_fingerprint: sha256:...
envelope_fingerprint: sha256:...
```

An envelope is fully materialized; omission never means “inherit later.” A child cites at most one exact parent envelope and repeats the complete effective state. Independent constraints are exact ref/fingerprint inputs rather than merge parents.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| schema identity and `envelope_id` | Handbook schema registry identifies shape; creating authority identifies this immutable envelope | none | exact schema; stable unique ID | not a mutable session setting |
| `resolved_profile` | selected profile remains semantic authority | none | exact ref/fingerprint; profile selects the cited stack/catalog | no invocation-time profile mutation |
| `resolution_stack` | selected stack definition owns level/domain comparison | none | exact ref/fingerprint equals resolved profile selection | no ambient or CLI-owned stack |
| `active_level_id` | envelope creator selects a named default position | none | exists in stack | not an aggregate authority score |
| `objective_ref` | parent work/operation authority identifies purpose | none | stable typed ref resolvable within scope | does not grant mutation authority |
| `dimensions` | envelope creator selects complete operating bounds under stack semantics | none; all six required | each value resolves; every child rank is less than or equal to parent rank | no token-budget proxy or omitted inherited dimension |
| `parent_envelope` | parent execution/view authority constrains child | root: `null`; child: exact ref/fingerprint | at most one; acyclic; same exact profile/stack; cited fingerprint matches | no multi-parent merge |
| `constraint_inputs` | named contracts/artifacts own applicable constraints | empty | exact typed ref/fingerprint pairs; deterministic stable ordering | no prose-only or bare mutable ref |
| `mutation_rules` | parent/creating authority grants or denies typed targets | explicit list, possibly empty | known target kinds; selectors validate through exact matcher; child allow set is a subset of parent; deny is union and wins | no unknown resource namespace, absolute-path escape, or child relaxation |
| `escalation_triggers` | exact trigger definitions own detection semantics | explicit list | unique IDs; exact refs/fingerprints; missing-context/authority condition is reportable | trigger detection does not approve escalation |
| `envelope_fingerprint` | Handbook derives immutable resolved identity | none | normalized complete record plus exact profile/stack/parent/constraint/policy fingerprints except itself | no caller-supplied or timestamp-based identity |

Envelope conformance scenarios:

| Scenario | Required result |
|---|---|
| identical complete inputs resolve twice | identical envelope fingerprint |
| child narrows any subset of dimensions and mutation allows | passes and preserves every parent deny |
| child increases one dimension rank or adds a mutation target | refuses and emits an escalation candidate; no partial widening |
| parent fingerprint is stale or profile/stack differs | refuses |
| allow and deny both match | deny wins and the effective set is unambiguous |
| validation claim exceeds `validation_horizon` | claim remains `not_authorized`, never passed |
| memory write exceeds `memory_horizon` | requires a separately authorized `ResolutionPromotionRequest` and terminal applied disposition |

## Resolution escalation and memory promotion

```yaml
schema_id: handbook.resolution-escalation-request
schema_version: "1.0"
escalation_request_id: escalation-request.example
current_envelope: { ref: envelope.example.execution, fingerprint: sha256:... }
proposed_envelope: { ref: envelope.example.coordination, fingerprint: sha256:... }
trigger: { ref: handbook.resolution-trigger.parent-contract-conflict@1.0.0, fingerprint: sha256:... }
missing_condition: broader_contract_authority
requested_authority_ref: work.slice.owner
evidence_refs: []
request_fingerprint: sha256:...
```

```yaml
schema_id: handbook.resolution-escalation-disposition
schema_version: "1.0"
escalation_disposition_id: escalation-disposition.example
request: { ref: escalation-request.example, fingerprint: sha256:... }
outcome: approved
decision: { ref: decision.example, fingerprint: sha256:... }
authorized_envelope: { ref: envelope.example.coordination, fingerprint: sha256:... }
superseding_request: null
disposition_fingerprint: sha256:...
```

```yaml
schema_id: handbook.resolution-promotion-request
schema_version: "1.0"
promotion_request_id: promotion-request.example
source_inputs:
  - { ref: snapshot.example, fingerprint: sha256:... }
source_envelope: { ref: envelope.example.execution, fingerprint: sha256:... }
target_memory_horizon: coordination
target_record_ref: semantic-memory.example
expected_target_fingerprint: null
requested_authority_ref: work.slice.owner
request_fingerprint: sha256:...
```

```yaml
schema_id: handbook.resolution-promotion-disposition
schema_version: "1.0"
promotion_disposition_id: promotion-disposition.example
request: { ref: promotion-request.example, fingerprint: sha256:... }
outcome: applied
decision: { ref: decision.example, fingerprint: sha256:... }
validation_evidence_refs:
  - { ref: evidence.example, fingerprint: sha256:... }
approving_authority_ref: work.slice.owner
result_record: { ref: semantic-memory.example@1, fingerprint: sha256:... }
disposition_fingerprint: sha256:...
```

Requests and dispositions are separate append-only records. A request is pending only while no terminal disposition exists; it never changes status in place. The escalation registry admits exactly one terminal disposition per exact request: `approved`, `refused`, or `superseded`. `approved` requires the exact decision and authorized replacement envelope; `refused` requires the decision and null authorized/superseding fields; `superseded` requires the decision and one exact distinct superseding-request pair. The promotion registry likewise admits exactly one `applied`, `refused`, or `stale` disposition per exact request. `applied` requires target-horizon validation, an authorized approver, compare-and-write against the request's `expected_target_fingerprint`, and one exact new semantic-memory result pair; refused/stale dispositions have a null result. Duplicate dispositions, in-place request mutation, reuse of an ID for changed bytes, and a disposition whose request fingerprint is stale all fail closed. Snapshots, deltas, and Projections remain immutable evidence and never become canonical artifacts/contracts merely because they are promotion inputs.

| Field family | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| request/disposition IDs and schema identity | Handbook schema registry/creating orchestrator | none | exact schemas; every record has a stable unique ID and fingerprint | no lifecycle mutation behind a reused ID |
| current/proposed envelope pairs | exact envelope records own bounds | none | refs/fingerprints match; proposal uses same profile/stack and changes at least one bound | no in-place envelope mutation |
| trigger/missing condition/requested authority | escalation policy and parent authority own why/who | none | exact trigger pair; typed missing condition; authority resolves above current bound | no “need more context” without a concrete condition |
| escalation disposition | requested authority owns terminal decision | absent while pending; exactly one terminal record | exact request/decision pair; outcome-specific authorized envelope or superseding request; registry uniqueness | request cannot self-approve or mutate into a disposition |
| promotion source inputs/envelope | immutable source records and envelope own observed boundary | none; non-empty sources | exact pairs; source envelope admits observation | no source promotion into canonical authority |
| target horizon/record/expected fingerprint | target semantic-memory authority owns write location | expected fingerprint `null` only for create | target rank exceeds source only through approved policy; compare-and-write basis exact | no artifact/contract/posture mutation shortcut |
| promotion disposition | target validation and authority own terminal admission | absent while pending; result null unless applied | exact request/decision; applied has complete paired evidence, authorized approver, successful compare-and-write, exact new result; stale/refused has no result | no unreviewed semantic-memory write or in-place outcome change |
| request/disposition fingerprints | Handbook derives immutable transition identity | none | normalized complete record except its own fingerprint; exact request pair enters disposition fingerprint | no mutable transition history |

Transition conformance requires request-only pending state, one valid terminal disposition, byte-identical replay of every prior request/disposition, and refusal of a second terminal disposition. Tests cover escalation approval/refusal/supersession and promotion application/refusal/stale compare-and-write without changing prior bytes.

## Projection disclosure policy

```yaml
schema_id: handbook.projection-disclosure-policy
schema_version: "1.0"
policy_id: handbook.projection-disclosure.core
policy_version: "1.0.0"
classification_registry: { ref: handbook.projection-classification.core@1.0.0, fingerprint: sha256:... }
matcher_definition: { ref: handbook.projection-disclosure-matcher.core@1.0.0, fingerprint: sha256:... }
unmatched_action: redact
indeterminate_match_action: refuse
overlap_precedence: redact_wins
rules:
  - { rule_id: public_allow, disclosure_classifications: [public], source_kinds: [artifact_kind, artifact_instance, semantic_capability, snapshot, snapshot_delta, semantic_record], source_pointer_selector: "*", action: allow }
  - { rule_id: internal_allow, disclosure_classifications: [internal], source_kinds: [artifact_kind, artifact_instance, semantic_capability, snapshot, snapshot_delta, semantic_record], source_pointer_selector: "*", action: allow }
  - { rule_id: sensitive_redact, disclosure_classifications: [sensitive, secret], source_kinds: [artifact_kind, artifact_instance, semantic_capability, snapshot, snapshot_delta, semantic_record], source_pointer_selector: "*", action: redact }
extensions: {}
policy_fingerprint: sha256:...
```

Every `ProjectionDefinition` binds one exact disclosure-policy ref/fingerprint. Each field rule binds one classification registered by the policy's exact classification registry and a complete six-dimension `minimum_resolution`. An unregistered definition classification invalidates the definition and refuses before any result/evaluation record or payload read; it never reaches policy fallback. Before protected payload bytes are read, the engine compares every request-envelope dimension rank with the rule minimum under the exact envelope stack: any lower rank yields `out_of_resolution`. It then evaluates the policy only over stable source metadata `(source_kind, exact source ref, source_pointer, disclosure_classification)`. A registered tuple matching no policy rule uses `unmatched_action: redact`; matching `redact` wins over matching `allow`; matcher indeterminacy, missing/stale policy, stale classification registry, or incompatible matcher refuses the request with no payload read. Invocation cannot replace or weaken the policy.

An immutable source may already carry an exact upstream redaction-disposition pair. Coverage uses the disposition's `original_pointer` and JSON Pointer segment boundaries. A request for that original pointer or its descendants maps any upstream `omit`, `fingerprint_only`, `artifact_ref_only`, or `redacted_summary` action to a Projection `redacted` omission before generic policy evaluation; it never maps to `unavailable`, and the engine never rereads the hidden original. The disposition's optional typed `retained.pointer` is outside the original subtree and is never covered merely because it shares earlier path segments; a request for that exact retained field evaluates it independently through its own rule/classification. Only an allowed pointer with no covering upstream disposition that cannot be read becomes `unavailable`. Runtime source-kind/schema/pointer or derivation-I/O rejection becomes `unsupported` before value access.

Every definition also binds one exact built-in `support_evaluator` ref/fingerprint. That pure metadata-only contract owns source-kind/schema compatibility, source-pointer existence/type, target-pointer/schema compatibility, and derivation input/output compatibility. Semantic-capability compatibility is not an evaluator input or `unsupported` reason: exact profile/definition/source validation resolves every selected capability contract and binding before per-rule evaluation, and invalid capability semantics refuse with no result. A valid evaluator can return `supported` or the typed per-rule `unsupported` outcome without reading value bytes. Missing, stale, unresolved, or schema-incompatible evaluator identity refuses before result construction. Changing only evaluator identity or semantics changes its fingerprint, the enclosing definition fingerprint, every applicable disclosure-evaluation fingerprint, and the result fingerprint.

Deterministic per-applicable-rule order is: validate exact definition/profile/source identity, support-evaluator identity, and mandatory currentness -> compare `minimum_resolution` -> map upstream redaction disposition -> evaluate exact disclosure policy -> evaluate runtime source-path/schema support through the bound evaluator -> read allowed bytes -> include/derive or record `unavailable`. Operation mismatch is classified before this disclosure sequence as the sole `not_applicable` case. A protected-source spy must observe zero value reads for `out_of_resolution`, `redacted`, policy/support refusal, or `unsupported` outcomes.

| Field family | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| policy identity and exact refs | security/disclosure policy author owns classification and matching semantics | none | stable ID plus SemVer; exact classification/matcher refs/fingerprints; compatible with definition/profile stack | no ambient/latest/invocation policy |
| policy actions/defaults | policy author owns allow/redact decision | v1 fixes a registered unmatched tuple to `redact`, indeterminate to `refuse`, and overlap to `redact_wins` | known actions; definition classifications registered before evaluation; complete deterministic precedence; no payload-dependent matcher input | no fallback for an unregistered definition classification, permissive unmatched default, or caller override |
| policy rules | policy author maps stable metadata tuples to allow/redact | none; non-empty | registered classifications/source kinds; deterministic pointer selectors; stable IDs; overlap replay | no prompt/model/transport rule or content inspection |
| field-rule `minimum_resolution` | Projection-definition author owns the least envelope that may disclose the rule | none; all six dimensions | every value belongs to the exact request-envelope stack domain; complete rank comparison | no named-level shortcut or aggregate score |
| field-rule `disclosure_classification` | classification registry and definition author own sensitivity class | none | exact registered class evaluated by bound policy | no filename/label inference or value sniffing |
| upstream redaction mapping | immutable source disposition owns prior hiding; Projection engine maps it | absent when no upstream disposition covers the pointer | exact disposition/source/pointer coverage; every non-raw upstream action maps original pointer to `redacted`; no reread | no downgrade to `unavailable`, recovery of omitted bytes, or policy weakening |
| `policy_fingerprint` | Handbook derives disclosure closure | none | normalized complete policy plus exact registry/matcher fingerprints except itself | no unchanged fingerprint after rule/default drift |

## Projection support evaluator definition

```yaml
schema_id: handbook.projection-support-evaluator-definition
schema_version: "1.0"
support_evaluator_id: handbook.projection-support.core
support_evaluator_version: "1.0.0"
schema_registry_contract: { ref: handbook.schema-registry.core@1.0.0, fingerprint: sha256:... }
pointer_semantics: { ref: handbook.json-pointer.rfc6901@1.0.0, fingerprint: sha256:... }
derivation_compatibility: { ref: handbook.projection-derivation-compatibility.core@1.0.0, fingerprint: sha256:... }
supported_source_kinds: [artifact_kind, artifact_instance, semantic_capability, snapshot, snapshot_delta, semantic_record]
decision_input_fields:
  - source_kind
  - source_schema_ref
  - source_schema_fingerprint
  - source_pointer
  - source_pointer_schema
  - derivation_ref
  - derivation_fingerprint
  - target_schema_ref
  - target_schema_fingerprint
  - target_pointer
  - target_pointer_schema
unsupported_reason_precedence:
  - source_kind_unsupported
  - source_schema_unregistered
  - source_pointer_missing
  - source_pointer_type_incompatible
  - target_schema_unregistered
  - target_pointer_missing
  - target_pointer_type_incompatible
  - derivation_unregistered
  - derivation_io_incompatible
extensions: {}
evaluator_fingerprint: sha256:...
```

The exact ref is `support_evaluator_id + "@" + support_evaluator_version`. The evaluator receives only the listed normalized metadata fields, resolved from the exact already-validated selected source metadata, field rule, target schema, and derivation pair; it never receives source-definition or semantic-capability identity and never receives payload values. Exact profile/definition/source validation has already resolved every source-definition identity plus semantic-capability contract and binding for every admitted source kind; a missing, stale, unregistered, or incompatible definition/capability pair refuses before this evaluator and produces neither `unsupported` nor a result. The evaluator first validates its own exact schema-registry/pointer/derivation-contract pairs. Missing, stale, unresolved, or incompatible evaluator/registry/contract state refuses before result construction. Otherwise it evaluates `unsupported_reason_precedence` in order. The first matching reason returns `support_status: unsupported` plus that exact `support_reason`; if none matches, it returns `support_status: supported` with `support_reason: null`. A prior disclosure short-circuit records `support_status: not_evaluated` and `support_reason: null` without invoking the evaluator.

`evaluator_fingerprint` is RFC 8785/SHA-256 over the normalized complete definition plus exact schema-registry, pointer-semantics, and derivation-compatibility fingerprints except itself. Changing the input list, supported source kinds, reason order, or any exact dependency therefore changes the evaluator pair. Extensions follow the shared HCM-0.3 optional-only rule and enter the evaluator fingerprint; they cannot add required support behavior or reorder base reasons.

| Field family | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| schema/evaluator identity | Handbook support-evaluator definition schema and author | none | exact schema; stable ID plus SemVer; ref derived mechanically; unique | no bare alias, ambient/latest evaluator, or invocation ID |
| registry/semantic contract pairs | their exact definitions own schema, pointer, and derivation semantics | none | exact refs/fingerprints; mutually compatible; all resolve before evaluation | no capability-registry ownership, payload content, network lookup, or transport registry |
| `supported_source_kinds` | evaluator definition owns admitted metadata families | none; non-empty | registered kinds; deterministic set normalization | no filename/label inference or implicit future kind |
| `decision_input_fields` | evaluator definition owns the complete metadata tuple | none; exact ordered v1 list shown | allowlist equals the shown fields; every field is derivable from already-validated source/schema/rule/derivation metadata; no extra/omitted input | no source-definition/capability identity, payload value, clock, environment, or caller hint |
| `unsupported_reason_precedence` | evaluator definition owns deterministic first-failure outcome | none; exact non-empty ordered reason set | known exhaustive reason codes; first match wins; supported iff none match | no multi-reason ordering drift, warning-based success, or caller-selected reason |
| `extensions` | evaluator schema owns optional namespaces | empty map | registered optional-only extension schemas; fingerprinted | no added required support rule or base-order override |
| `evaluator_fingerprint` | Handbook derives exact evaluator identity | none | normalized complete definition plus exact dependency fingerprints except itself | no unchanged fingerprint after evaluator semantic drift |

## Projection definition

```yaml
schema_id: handbook.projection-definition
schema_version: "1.0"
projection_definition_id: handbook.projection.example-agent-packet
projection_definition_version: "1.0.0"
source_selectors:
  - selector_id: project_context
    source_kind: artifact_kind
    source_ref: handbook.artifact-kind.project-context@1.0.0
    cardinality: exactly_one
allowed_surfaces:
  - agent_packet
allowed_operations:
  - reveal
  - derive
target_schema:
  ref: handbook.schemas.projection.agent-packet@1.0.0
  fingerprint: sha256:...
disclosure_policy: { ref: handbook.projection-disclosure.core@1.0.0, fingerprint: sha256:... }
support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }
currentness_requirements:
  mode: none
  revision_basis: null
  families: []
field_rules:
  - rule_id: reveal_objective
    operation: reveal
    source_selector_id: project_context
    source_pointer: /objective
    target_pointer: /objective
    minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: normal, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: operation, validation_horizon: observation_only }
    disclosure_classification: internal
    required_for_result: true
    claim_refs: []
    derivation: null
  - rule_id: derive_constraints
    operation: derive
    source_selector_id: project_context
    source_pointer: /constraints
    target_pointer: /constraint_summary
    minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: normal, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: operation, validation_horizon: observation_only }
    disclosure_classification: internal
    required_for_result: false
    claim_refs: []
    derivation:
      ref: handbook.derivation.normalized-summary@1.0.0
      fingerprint: sha256:...
extensions: {}
definition_fingerprint: sha256:...
```

Source kinds are `artifact_kind`, `artifact_instance`, `semantic_capability`, `snapshot`, `snapshot_delta`, or another registered semantic record class. Canonical artifacts and immutable semantic/observation records are both valid typed sources, but Projection preserves the source's existing authority class; accepting a snapshot or delta never labels it canonical truth or peer authority. V1 selector cardinality is always `exactly_one`; zero or multiple identity matches refuse rather than choosing by source order. A rule may reveal one compatible source path or derive through one exact allowlisted Handbook derivation. Rule graphs are acyclic; every target pointer has one producer; required rules cannot be silently dropped. Definitions contain no executable hook, remote code, command, prompt, or transport renderer.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| definition identity | Projection-definition author owns mapping identity | none | stable ID plus SemVer; unique exact ref | not a request/result ID |
| `source_selectors` | definition author declares accepted typed sources | none; at least one | unique selector IDs; exact compatible source refs/fingerprint closure; `cardinality=exactly_one` | no plural/source-order/filename/label inference |
| `allowed_surfaces` | definition author limits transport-neutral view purposes | none | registered stable surface IDs | surface does not own domain semantics |
| `allowed_operations` | definition author selects deterministic operations | none | non-empty subset of `reveal`, `derive` | no synthesis in v1 |
| `target_schema` | schema registry owns output structure | none | exact safe local schema ref/fingerprint | no prose-only output contract |
| `disclosure_policy` | exact Projection disclosure policy owns metadata-only allow/redact/default/refusal semantics | none | exact ref/fingerprint; compatible matcher/classification registry and profile stack; invocation cannot replace it | no ambient policy, value inspection, or transport-owned redaction |
| `support_evaluator` | exact built-in Projection support contract owns metadata-only source-kind/path/schema and derivation-I/O compatibility | none | exact ref/fingerprint; compatible with source and target schema registries; profile catalog closes it; invocation cannot replace it | no semantic-capability ownership, ambient registry, payload/content inspection, or transport-owned support decision |
| `currentness_requirements` | definition author owns any exact pre-output source-currentness closure | none; every definition declares it | `none` requires null basis/empty families; `exact_revision_check` is snapshot-selector-only, fixes `revision_basis: captured_revision`, and requires non-empty unique family, exact adapter, and declared source-slot bindings | no extension-supplied required behavior, request-selected family/value set, post-revision proxy, or purpose-derived currentness |
| `field_rules` | definition author owns deterministic mapping, v1 applicability, minimum disclosure ranks, and classification | none; at least one | declared operation is allowed and is the sole applicability condition; valid source/target JSON Pointers; complete valid six-dimension minimum; registered disclosure class; unique producers; acyclic; exact derivation when used | no arbitrary condition/expression, named-level shortcut, content sniffing, or executable plugin |
| `required_for_result`/`claim_refs` | contract/definition author declares decision-completeness and claim-observation consequences | `false`/empty | claims resolve and source path can observe them; exact inclusion/omission truth table below; every potentially omitted target pointer is structurally omittable in the target schema | `required_for_result` does not suppress the Projection result, make the target pointer unconditionally schema-required, or create automatic passing evidence |
| `definition_fingerprint` | Handbook derives definition closure | none | normalized definition plus exact source/schema/derivation/disclosure-policy/support-evaluator fingerprints except itself | no mutable alias/latest selection |

## Projection request

This capitalized `Projection` request/result pair belongs only to the Phase 3 generic Resolution-aware Projection engine. A Phase 2 fixed deterministic first-party renderer produces a renderer-derived human-review view, not a Projection; it accepts no Resolution envelope and is excluded from this request/result/provenance contract. Generic configured custom-kind Projections and every Resolution-aware view are deferred until the `HCM-3.2` Context Resolution kernel and `HCM-3.3` deterministic Projection engine land.

```yaml
schema_id: handbook.projection-request
schema_version: "1.0"
request_id: projection-request.example
sources:
  - selector_id: project_context
    ref: artifact.project-context@7
    fingerprint: sha256:...
resolved_profile: { ref: handbook.profile.example@1.0.0, fingerprint: sha256:... }
vocabulary: { ref: handbook.vocabulary.example@1.0.0, fingerprint: sha256:... }
projection_definition: { ref: handbook.projection.example-agent-packet@1.0.0, fingerprint: sha256:... }
resolution_envelope: { ref: envelope.example.execution, fingerprint: sha256:... }
operation: reveal
surface: agent_packet
purpose: delegated_execution_context
currentness:
  mode: none
  revision_basis: null
  expected_family_revisions: []
```

The request must use the exact profile-selected vocabulary, stack, catalog, and complete envelope; every source selector binds exactly one stable source identity through its ref/fingerprint pair. Zero/multiple identity matches, an invalid pair, stale definition/profile/capability semantics, or a failed mandatory currentness precondition refuses. Operation and surface must be allowed by the definition. For a valid exactly bound source, envelope access is evaluated per applicable field rule before protected payload bytes are read: envelope denial produces `out_of_resolution`, redaction produces `redacted`, payload unavailability produces `unavailable`, and a runtime-unsupported source kind/path/schema or derivation I/O produces `unsupported`. Each is a typed omission with its proof effect, never a request refusal or `not_applicable`; hidden content is not loaded and then concealed. Invocation cannot replace profile, vocabulary, definition, or envelope semantics.

## Projection result

This result is Phase-3-only and is never required for a fixed pre-Phase-3 renderer-derived view.

```yaml
schema_id: handbook.projection-result
schema_version: "1.0"
result_id: projection-result.example
request_ref: projection-request.example
sources:
  - selector_id: project_context
    ref: artifact.project-context@7
    fingerprint: sha256:...
resolved_profile: { ref: handbook.profile.example@1.0.0, fingerprint: sha256:... }
vocabulary: { ref: handbook.vocabulary.example@1.0.0, fingerprint: sha256:... }
projection_definition: { ref: handbook.projection.example-agent-packet@1.0.0, fingerprint: sha256:... }
resolution_envelope: { ref: envelope.example.execution, fingerprint: sha256:... }
surface: agent_packet
operation: reveal
currentness_validation:
  mode: none
  revision_basis: null
  checks: []
disclosure_evaluations:
  - rule_id: reveal_objective
    definition_rule_ordinal: 1
    minimum_resolution_outcome: sufficient
    upstream_redaction_check: none
    upstream_redaction_disposition: null
    policy_evaluation: matched
    matched_policy_rule_ids: [internal_allow]
    policy_action: allow
    support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }
    support_status: supported
    support_reason: null
    payload_access: read
    final_disposition: included
    evaluation_fingerprint: sha256:...
lossiness: lossless
included:
  - rule_id: reveal_objective
    source_pointer: /objective
    target_pointer: /objective
    claim_refs: []
omissions: []
not_applicable:
  - rule_id: derive_constraints
    reason: operation_mismatch
derivations: []
output:
  schema_ref: handbook.schemas.projection.agent-packet@1.0.0
  content_ref: projection-output.example
  content_fingerprint: sha256:...
authority_effect: none
result_fingerprint: sha256:...
```

Allowed `operation` values initially:

- `reveal`;
- `derive`.

Reserve `synthesize_candidate` in the conceptual model but do not implement it in the first projection slice.

Omission entries contain `rule_id`, source selector/path, the rule's exact claim refs, one of `out_of_resolution`, `redacted`, `unavailable`, or `unsupported`, and exactly one `proof_effect`: `not_observed` or `none`. `required_for_result` means required for a complete decision/evidence-bearing view, not required for the existence of the Projection result record. An omitted target pointer is absent from `output`; every Projection target schema must allow that typed absence for every potentially omitted rule, or the definition is invalid before execution. A result with an omitted required rule is still emitted and schema-valid but is incomplete for any positive decision/proof use and carries `not_observed`. In v1, each field rule's declared `operation` is its complete applicability condition. A rule whose operation differs from the request appears exactly once in `not_applicable` with `reason: operation_mismatch`; it is not an omission and has no proof effect. The definition rule IDs must equal the disjoint union of `included`, `omissions`, and `not_applicable` rule IDs.

`disclosure_evaluations` contains exactly one entry for every applicable rule, sorted by the rule's one-based `definition_rule_ordinal`, and none for operation-mismatch rules. No ordinal may be duplicated, skipped among applicable rules, or reordered. Each entry contains `rule_id`, `definition_rule_ordinal`, `minimum_resolution_outcome`, `upstream_redaction_check`, `upstream_redaction_disposition`, `policy_evaluation`, ordered `matched_policy_rule_ids`, resolved `policy_action`, exact `support_evaluator` ref/fingerprint, `support_status`, `support_reason`, `payload_access`, `final_disposition`, and `evaluation_fingerprint`. The support pair is always present and equals the definition pair even when a prior short-circuit makes `support_status: not_evaluated`; `support_reason` is non-null exactly for `unsupported` and is the evaluator's first matching frozen reason. Policy/definition/support/currentness refusal occurs before result construction and therefore emits no result or disclosure-evaluation record.

`evaluation_fingerprint` is RFC 8785/SHA-256 over the normalized evaluation entry except itself plus the exact result-level source pair selected by the rule, Projection-definition pair/fingerprint (which closes over the disclosure policy and rule), and envelope pair/fingerprint. A non-null upstream-disposition pair is already inside the entry. The enclosing `result_fingerprint` hashes the complete ordered evaluation list. Derivation entries bind rule/derivation refs, exact input fingerprints, output fingerprint, and `lossy: true|false` declared by the derivation definition.

Exact inclusion/omission/proof table:

| Rule applicability/outcome | `required_for_result` | `claim_refs` | Target/output treatment | `proof_effect` and result treatment |
|---|---:|---|---|---|
| operation mismatch | either | either | target absent; one `not_applicable(operation_mismatch)` entry | no `proof_effect`; requiredness is irrelevant because the rule is not applicable |
| applicable and included | either | either | target present and schema-valid | no `proof_effect`; included claims may be evaluated only from the cited source/output |
| applicable and omitted | `true` | empty or non-empty | target absent through schema-declared typed omission | `not_observed`; result is emitted but incomplete/non-passing for decision or proof use |
| applicable and omitted | `false` | non-empty | target absent through schema-declared typed omission | `not_observed` for every named claim; result is emitted and no named claim is satisfied |
| applicable and omitted | `false` | empty | target absent through schema-declared typed omission | `none`; result is emitted with lossiness determined by the omission reason |

Exact disclosure-evaluation outcome matrix (`upstream disposition` is always an exact `{ref, fingerprint}` pair when non-null, and every row carries the same exact definition-bound `support_evaluator` pair):

| Final disposition | `minimum_resolution_outcome` | `upstream_redaction_check` / disposition | `policy_evaluation` / matched IDs / action | `support_status` / reason | `payload_access` |
|---|---|---|---|---|---|
| `out_of_resolution` | `insufficient` | `not_evaluated` / `null` | `not_evaluated` / `[]` / `not_evaluated` | `not_evaluated` / `null` | `not_attempted` |
| `redacted` by upstream disposition | `sufficient` | `covered` / exact pair | `not_evaluated` / `[]` / `not_evaluated` | `not_evaluated` / `null` | `not_attempted` |
| `redacted` by matched policy rule(s) | `sufficient` | `none` / `null` | `matched` / non-empty definition order / `redact` | `not_evaluated` / `null` | `not_attempted` |
| `redacted` by registered unmatched tuple | `sufficient` | `none` / `null` | `unmatched_default` / `[]` / `redact` | `not_evaluated` / `null` | `not_attempted` |
| `unsupported` | `sufficient` | `none` / `null` | `matched` / non-empty definition order / `allow` | `unsupported` / exact first reason | `not_attempted` |
| `unavailable` | `sufficient` | `none` / `null` | `matched` / non-empty definition order / `allow` | `supported` / `null` | `attempted_unavailable` |
| `included` | `sufficient` | `none` / `null` | `matched` / non-empty definition order / `allow` | `supported` / `null` | `read` |

For overlapping matched rules, `matched_policy_rule_ids` retains exact policy-definition order and the resolved action is `redact` if any match redacts. A policy refusal (`indeterminate` matcher, missing/stale/incompatible policy or registry), invalid/unregistered definition classification, invalid source identity/cardinality, or failed mandatory currentness check emits no Projection result. Mixed-rule fixtures must reproduce byte-identical evaluation ordering, evaluation fingerprints, and the enclosing result fingerprint.

Lossiness uses this fixed precedence:

| Highest matching condition | Result lossiness |
|---|---|
| any omission is `redacted` | `redacted` |
| otherwise any omission is `unavailable` or `unsupported` | `partial` |
| otherwise any omission is `out_of_resolution`, or any included derivation is lossy | `collapsed` |
| otherwise every applicable rule is included and every derivation is non-lossy | `lossless` |

Mixed reasons always take the highest row, so the caller cannot choose a friendlier label. `not_applicable` entries do not affect lossiness. Every generic request/result carries currentness basis/evidence: definition mode `none` requires null basis plus request `none`/empty expected revisions and result `none`/empty checks. `exact_revision_check` is valid only for definitions whose required families bind snapshot source selectors and fixes `revision_basis: captured_revision`. Each request expected family revision must equal that selected snapshot's family `captured_revision`, and each expected slot revision must equal its corresponding captured slot revision. Result observations must equal those same request/bound-snapshot values; pre/post revisions and arbitrary caller-supplied live values cannot substitute. Exact mode also requires definition/request/result tuple equality. The result fingerprint covers request/provenance, currentness validation, complete disclosure evaluations, included/omitted/not-applicable/derived entries, output fingerprint, lossiness, and authority effect except itself.

Projection conformance scenarios:

| Scenario | Required result |
|---|---|
| same exact request and source bytes execute twice | identical output and result fingerprints |
| a selector resolves zero or multiple sources | refuses; v1 never selects a source-order winner |
| an exactly bound applicable source field is outside the envelope | `out_of_resolution` omission with its proof effect; result remains completely accounted |
| an exactly bound applicable source field is redacted | `redacted` omission with its proof effect; protected bytes are not disclosed |
| an exactly bound applicable source payload is unavailable | `unavailable` omission with its proof effect; no request refusal unless a separate mandatory currentness check fails |
| an exactly bound applicable source kind/path/schema or derivation I/O is unsupported at runtime | `unsupported` omission with its proof effect |
| reveal request executes the example definition | `reveal_objective` is included and `derive_constraints` is exactly `not_applicable` because of operation mismatch |
| equivalent definitions reveal/derive from a canonical artifact, snapshot, or delta source | each result preserves the source's prior authority class and records `authority_effect: none`; observation sources do not become canonical/peer truth |
| each omission reason crosses required/non-required rules with empty/non-empty claims | output remains schema-valid with target absent; exact table selects `not_observed` or `none`; rule partition and lossiness remain complete; no false pass |
| definition/profile/vocabulary/envelope fingerprint is stale | refuses |
| request asks for unsupported operation/surface | refuses |
| expansion needs a higher dimension or mutation authority | returns a `ResolutionEscalationRequest` candidate; engine does not widen itself |
| definition contains a prompt, executable hook, remote ref, cycle, or two target producers | definition validation refuses |

Projection request/result field authority:

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| request/result schema and IDs | Handbook schema registry owns shape; caller/engine allocate immutable operation IDs | none | exact schema; unique stable IDs; result cites exact request | IDs do not grant authority |
| request `sources` | caller selects exact typed source identities allowed by definition; source records retain their prior authority class | none | paired ref/fingerprint and selector/cardinality satisfied; envelope/redaction/availability/support are evaluated per applicable rule into typed omissions | no parallel arrays, ambient discovery, request-wide envelope-visibility precondition, snapshot-to-canonical promotion, or hidden-payload read before access decision |
| profile/vocabulary/definition/envelope pairs | their exact definitions remain semantic/display/view/authority owners | none | exact matching fingerprints and cross-compatibility; request and result pairs identical | no invocation override or result-side substitution |
| request `operation`, `surface`, `purpose` | caller selects within definition | none | operation/surface allowed; purpose registered and non-authoritative | purpose does not change mapping rules |
| request `currentness` | exact definition owns mode/basis and family/selector/adapter/slot closure; bound snapshot owns expected values | none; field is always present | mode/basis equal definition; `none` is null/empty; exact tuples equal definition and values equal selected snapshot captured composite/slots | no extension/purpose-derived basis, caller-selected closure/value, or pre/post proxy |
| result `currentness_validation` | exact source adapters own observed live revisions; engine records evidence | none; field is always present | mode/basis equal definition/request; `none` is null/empty; exact tuples equal definition/request; observed equals request and bound captured values; all pass | no stale snapshot greened by unrelated equal live values, omitted evidence, or post-request source substitution |
| result `disclosure_evaluations` | exact stack, field rule, upstream source disposition, disclosure policy, and definition-bound support evaluator own decisions; engine records replay | none; one per applicable rule | exact applicable-rule set; exact support pair always equals definition; `unsupported` has first-precedence reason and other statuses have null reason; fixed evaluation order/nullability; final disposition equals included/omission entry; payload access consistent; evaluation fingerprint recomputes | no ambient support registry, unowned/free-form reason, evaluation for operation mismatch, hidden read, unrecorded policy default, or result-side reclassification |
| result `included` | engine records each executed reveal/derive rule | empty only when definition/request allow an empty result | unique rule IDs; exact source/target pointers/claims | no unproven claim implication |
| result `omissions` | engine records every applicable unexecuted rule | empty on fully included result | typed reason; exact rule/claim refs; target absent under an omission-compatible target schema; proof effect follows the complete requiredness/claim table; every applicable rule is included or omitted exactly once | no result suppression for a valid omission, silent omission, invented placeholder value, or green default |
| result `not_applicable` | engine records definition rules whose operation differs from the request | empty only when every rule uses the request operation | exact rule ID and `operation_mismatch`; disjoint-union accounting equals all definition rules | no omission reason, proof effect, or caller-defined condition |
| result `derivations` | exact derivation definition owns algorithm | empty for reveal-only result | exact derivation/input/output fingerprints and matching rule | no model/executable hook |
| result `output` | target schema owns shape; engine owns derived bytes | none | exact schema; content ref/fingerprint; output validates with omitted rule targets absent; a definition whose schema cannot represent any allowed typed omission is invalid | output is not canonical source authority and omission never inserts a fabricated placeholder |
| `lossiness` | engine derives from rule outcomes | none | exact computed enum consistent with omissions/derivations | caller cannot request `lossless` |
| `authority_effect` | architecture contract fixes view authority | always `none` | exact literal | no auto-promotion or source mutation |
| `result_fingerprint` | Handbook derives immutable result identity | none | normalized complete result plus exact provenance/output fingerprints except itself | no timestamp or mutable alias |

## Snapshot capture policy

```yaml
schema_id: handbook.snapshot-capture-policy
schema_version: "1.0"
policy_id: handbook.snapshot-policy.session-boundary
policy_version: "1.0.0"
triggers:
  - session_start
  - session_end
allowed_memory_horizons:
  - execution
  - operation
state_families:
  git:
    source_adapter: { ref: handbook.snapshot-source.git@1.0.0, fingerprint: sha256:... }
    include_paths: true
    include_diff_stats: true
    full_diff: artifact_ref_only
  handbook:
    source_adapter: { ref: handbook.snapshot-source.handbook@1.0.0, fingerprint: sha256:... }
    include_profile: true
    include_artifact_fingerprints: true
    include_contract_state: true
  work:
    source_adapter: { ref: handbook.snapshot-source.work@1.0.0, fingerprint: sha256:... }
    source_slots:
      - work_ledger
      - active_plan
    composite_revision_rule: { ref: handbook.snapshot-composite.work-slots@1.0.0, fingerprint: sha256:... }
    recent_completed:
      window_id: recent_completed
      count: 10
      source_slot: work_ledger
      cursor_mode: exclusive
      ordering: completed_at_then_id
    queued_next:
      window_id: queued_next
      count: 10
      source_slot: active_plan
      cursor_mode: exclusive
      ordering: canonical_queue_order
  session:
    source_adapter: { ref: handbook.snapshot-source.session@1.0.0, fingerprint: sha256:... }
    include_orchestration_state: true
  evidence:
    source_adapter: { ref: handbook.snapshot-source.evidence@1.0.0, fingerprint: sha256:... }
    include_latest_gate_refs: true
comparison_contract: { ref: handbook.snapshot-comparison.core@1.0.0, fingerprint: sha256:... }
drift_rule_catalog: { ref: handbook.snapshot-drift.core@1.0.0, fingerprint: sha256:... }
predecessor_rule: { ref: handbook.snapshot-predecessor.boundary-stream@1.0.0, fingerprint: sha256:... }
redaction_policy: { ref: handbook.snapshot-redaction.default@1.0.0, fingerprint: sha256:... }
consistency:
  retries: 2
  bounded_skew_rule: { ref: handbook.snapshot-bound.exact-revision-per-family@1.0.0, fingerprint: sha256:... }
  unstable_action: persist_non_promotable
retention_policy: { ref: handbook.snapshot-retention.session@1.0.0, fingerprint: sha256:... }
extensions: {}
policy_fingerprint: sha256:...
```

The versioned policy owns only static window semantics: stable window ID, source slot, count, cursor mode, and total ordering. At capture time each window binds that slot to one exact source ref/revision and supplies an opaque source-issued cursor or `null` for the rule-defined initial boundary. `exclusive` means the item named by a non-null cursor is not repeated. Reusing the same source revision, cursor, count, and ordering selects the same window. Source revision/cursor changes alter snapshot capture input and state/record fingerprints, not the policy definition fingerprint.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| policy identity | capture-policy author owns selected-state semantics | none | stable ID plus SemVer; exact derived ref | no implicit local/default policy |
| `triggers` | policy author selects strategic capture boundaries | none; non-empty | registered trigger IDs; deterministic stable order; no command-by-command default | trigger does not change authority |
| `allowed_memory_horizons` | policy author limits where records produced by this policy may be stored | none; non-empty | unique valid horizons; capture envelope's memory horizon is a member; every horizon/trigger/record-class tuple is covered by retention | no invocation-selected durability |
| `state_families` | policy author selects exact source adapters and normalized fields | none; non-empty | registered family; exact adapter ref/fingerprint; adapter declares source/revision/payload schema | no ambient plugin or unrestricted payload |
| multi-source family slots/composite rule | family adapter definition owns source-slot identities; exact composite rule owns family revision | omitted only for single-slot families | unique declared slots; exact rule pair; composite covers every slot in stable order | no plan/ledger drift hidden behind one slot's revision |
| bounded-window definitions | policy author owns window ID/source slot/count/cursor mode/ordering | omitted when no window | unique window ID; positive bound; declared source slot; deterministic total order; exclusive cursor mode | no live revision/cursor in reusable policy and no nondeterministic “last N” |
| comparison/drift/predecessor refs | exact definitions own compatibility, signal, and predecessor-applicability rules | none | exact refs/fingerprints compatible with all families/triggers | no model interpretation, free-form causal classifier, or heuristic predecessor |
| redaction/retention refs | exact policies own disclosure/storage limits | none | exact refs/fingerprints; redaction fail-closed; retention honors referenced records/holds | no invocation override to expose denied content |
| `consistency` | policy author owns retries, bounded rule, and unstable disposition | none | bounded retries; declared skew rule; unstable action only `persist_non_promotable` or `refuse` | no unstable closeout/grounding claim |
| `policy_fingerprint` | Handbook derives policy closure | none | normalized policy plus every exact source/policy/rule fingerprint except itself | no timestamp or mutable source alias |

Changing any selected field, source adapter, source-slot/composite rule, memory horizon, static window rule, comparison/drift/predecessor rule, redaction/retention policy, or consistency behavior requires a changed policy version/fingerprint. Capture invocation selects an exact policy/trigger and supplies only exact live source revisions/cursors for declared slots/windows; it cannot widen the policy or memory horizon.

## Context Memory Snapshot

Conceptual minimum (family payloads are abbreviated but their envelopes are required):

```yaml
schema_id: handbook.context-memory-snapshot
schema_version: "1.0"
snapshot_id: snap_...
capture:
  trigger: session_end
  policy: { ref: handbook.snapshot-policy.session-boundary@1.0.0, fingerprint: sha256:... }
  started_at: "..."
  completed_at: "..."
  producer_version: "..."
  consistency: stable
  retry_count: 0
repository_identity:
  repository_id: repo.example
  workspace_id: worktree.example
boundary_stream_ref: orchestration.example
boundary_sequence: 2
resolved_profile: { ref: handbook.profile.example@1.0.0, fingerprint: sha256:... }
context_resolution_envelope: { ref: envelope.example.execution, fingerprint: sha256:... }
family_observations:
  git:
    source_adapter: { ref: handbook.snapshot-source.git@1.0.0, fingerprint: sha256:... }
    pre_revision: git:abc
    captured_revision: git:abc
    post_revision: git:abc
    consistency: stable
    bound_evaluation: null
    payload_fingerprint: sha256:...
    payload:
      branch: main
      head: abc
      upstream: origin/main
      operation_state: clean
      dirty_paths: []
      untracked_paths: []
      diff_summary: {}
      diff_artifact_refs: []
  handbook:
    source_adapter: { ref: handbook.snapshot-source.handbook@1.0.0, fingerprint: sha256:... }
    pre_revision: handbook-state:7
    captured_revision: handbook-state:7
    post_revision: handbook-state:7
    consistency: stable
    bound_evaluation: null
    payload_fingerprint: sha256:...
    payload:
      artifact_kind_registry_fingerprint: sha256:...
      vocabulary_fingerprint: sha256:...
      resolution_stack_fingerprint: sha256:...
      artifacts: []
      intake_refs: []
      unresolved_intake_coverage: []
      posture_kernel_ref: null
      posture_recommendation_refs: []
      contracts: []
      verdict_refs: []
      gate_refs: []
  work:
    source_adapter: { ref: handbook.snapshot-source.work@1.0.0, fingerprint: sha256:... }
    pre_revision: composite:sha256:...
    captured_revision: composite:sha256:...
    post_revision: composite:sha256:...
    source_slot_revisions:
      - { source_slot: work_ledger, pre_revision: "work:4", captured_revision: "work:4", post_revision: "work:4" }
      - { source_slot: active_plan, pre_revision: "plan:4", captured_revision: "plan:4", post_revision: "plan:4" }
    consistency: stable
    bound_evaluation: null
    payload_fingerprint: sha256:...
    payload:
      window_inputs:
        - window_id: recent_completed
          source_ref: work-ledger
          source_revision: work:4
          cursor: null
          count: 10
          ordering: completed_at_then_id
        - window_id: queued_next
          source_ref: active-plan
          source_revision: plan:4
          cursor: null
          count: 10
          ordering: canonical_queue_order
      active_refs: []
      recent_completed: []
      queued_next: []
      blocked_refs: []
  session:
    source_adapter: { ref: handbook.snapshot-source.session@1.0.0, fingerprint: sha256:... }
    pre_revision: session:3
    captured_revision: session:3
    post_revision: session:3
    consistency: stable
    bound_evaluation: null
    payload_fingerprint: sha256:...
    payload:
      parent_orchestration_ref: orchestration.example
      handoff_ref: null
      active_dispatch_refs: []
      unresolved_escalation_refs: []
  evidence:
    source_adapter: { ref: handbook.snapshot-source.evidence@1.0.0, fingerprint: sha256:... }
    pre_revision: evidence:9
    captured_revision: evidence:9
    post_revision: evidence:9
    consistency: stable
    bound_evaluation: null
    payload_fingerprint: sha256:...
    payload:
      validation_refs: []
      unresolved_proof_refs: []
redaction:
  policy: { ref: handbook.snapshot-redaction.default@1.0.0, fingerprint: sha256:... }
  dispositions: []
retention_policy: { ref: handbook.snapshot-retention.session@1.0.0, fingerprint: sha256:... }
excluded_families: []
previous_snapshot:
  ref: snap_previous
  record_fingerprint: sha256:...
  boundary_sequence: 1
admissibility: grounding_and_evidence
state_fingerprint: sha256:...
record_fingerprint: sha256:...
```

Every real family payload uses its source-adapter schema. The `work` family composite revision is derived by its exact rule over both declared source slots, and each window's `source_revision` equals the corresponding slot's captured revision. Work windows carry source revision, cursor, count, ordering, and selected stable IDs. Session state names the parent orchestration, applicable true-stop handoff, active internal dispatches, delegated-run reconciliation, and unresolved escalation refs. Evidence state contains refs/statuses rather than unrestricted command output.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| schema identity and `snapshot_id` | Handbook schema registry identifies shape; capture service allocates immutable record identity | none | exact schema; unique stable ID | snapshot ID is not state identity |
| `capture` | exact capture policy owns observation semantics; engine derives top-level consistency | none | allowed trigger; matching policy fingerprint; monotonic start/end; bounded retry count; consistency equals aggregation table | caller cannot label the record more favorably |
| repository/workspace identity | capture source identifies observed workspace | none | stable normalized IDs | no absolute machine path in durable identity |
| boundary stream/sequence | capture service identifies transition stream and allocates record order | none | stable stream ref; sequence is unique and strictly increasing in repository/workspace/stream; allocation collision retries or refuses | no timestamp ordering, tie-breaker, or state-fingerprint input |
| profile/envelope pairs | selected semantic authority and execution/view boundary | none | exact refs/fingerprints; envelope cites profile and policy-compatible stack | snapshot does not alter either authority |
| `family_observations` | exact source adapter owns payload/revision schema | every policy-selected family required | exact adapter pair; pre/captured/post revisions; per-slot revisions and exact composite for multi-source families; policy-selected bound evaluation; family consistency; payload matches schema/fingerprint | no ambient or unversioned source or hidden source-slot drift |
| window capture inputs | capture invocation binds declared static window slots to live state | every policy window represented in its family payload | matching window ID/count/ordering; exact source identity/revision; opaque cursor/null under declared cursor mode | live revision/cursor do not mutate policy identity |
| `excluded_families` | capture records a selected family that was not observed | empty | every policy family is observed or appears once with exactly one of `unavailable`, `unsupported`, `redacted`, `unstable`; any entry forces top-level unstable/diagnostic-only or policy refusal | missing is never unchanged, observed, stable, or bounded |
| redaction dispositions | exact redaction policy owns disclosure; immutable disposition owns original/retained mapping | empty only when nothing matched | exact disposition pairs; original pointer/subtree, action/nullability matrix, matched rules, optional safe pre-fingerprint, retained pointer/ref/fingerprint, disposition fingerprint | no secret value, ambiguous path-prefix coverage, or retained field inside original subtree |
| retention policy | exact policy owns storage lifecycle | none | exact ref/fingerprint | no mutation of retained record bytes |
| `previous_snapshot` | exact predecessor rule selects the applicable prior boundary observation | `null` only when no eligible predecessor exists | exact ref/record fingerprint; same repository/workspace/boundary stream; recorded sequence is the greatest eligible value below current | no self/future/cycle/wrong-stream/wrong-boundary link or causal claim |
| `admissibility` | consistency rules derive allowed use | none | `grounding_and_evidence` for stable/bounded; `diagnostic_only` for unstable | never canonical/contract authority |
| `state_fingerprint` | Handbook derives selected observed-state identity | none | normalization below | no trigger/time/snapshot-ID sensitivity |
| `record_fingerprint` | Handbook derives complete immutable-record identity | none | normalization below | no mutable record after publication |

### Snapshot consistency

Supported consistency values:

- `stable` — selected authorities/revisions did not change during capture;
- `bounded` — every separately captured payload is bound to an exact immutable revision and cross-source skew satisfies the policy's explicit comparison bound;
- `unstable` — one or more authorities changed and the retry policy could not obtain a stable/bounded record.

Top-level consistency and admissibility are derived exactly:

| Observed family classifications | Selected-family exclusions | Top-level consistency | Admissibility/result |
|---|---|---|---|
| all `stable` | none | `stable` | `grounding_and_evidence` |
| at least one `bounded`, all others `stable` or `bounded`, every bound evaluation valid | none | `bounded` | `grounding_and_evidence` |
| any `unstable` | any | `unstable` | `diagnostic_only`, or no record when policy says `refuse` |
| all observed families `stable`/`bounded` | any `unavailable`, `unsupported`, `redacted`, or `unstable` exclusion | `unstable` | `diagnostic_only`, or no record when policy says `refuse` |

Field-level redaction within an otherwise observed family does not create a family exclusion; it remains represented by redaction dispositions and may coexist with stable/bounded source consistency. A whole selected family that cannot be represented is excluded and makes the capture incomplete, so it cannot be labeled grounding-and-evidence. The engine applies the table after retries; invocation cannot supply the top-level value or admissibility.

`stable` requires equal pre/captured/post revisions for every selected family, equal pre/captured/post revisions for every declared source slot, an exact family composite over those slot revisions, and `bound_evaluation: null`. A bounded family records `bound_evaluation` with the exact same bound-rule ref/fingerprint selected by the capture policy, evaluated composite/per-slot revision set, `within_bound` outcome, and evidence fingerprint; its payload must be attributable to `captured_revision`. A missing, substituted, or stale rule pair refuses bounded classification. A changed source is not `bounded` merely because the reader finished. An out-of-bound evaluation triggers the declared retry and ultimately `unstable`/diagnostic-only or refusal; it cannot ground a closeout, promotion, hard gate, or stable delta. Active-plan-only drift therefore changes the work composite and yields retry or unstable/stale refusal rather than a current work observation.

The bounded evaluation has this exact semantic shape:

```yaml
bound_evaluation:
  rule: { ref: handbook.snapshot-bound.exact-revision-per-family@1.0.0, fingerprint: sha256:... }
  evaluated_revisions:
    family_revision: composite:sha256:...
    source_slot_revisions: { work_ledger: "work:4", active_plan: "plan:4" }
  outcome: within_bound
  evidence_fingerprint: sha256:...
```

The `rule` pair must equal the capture policy's `consistency.bounded_skew_rule`; the evaluation is capture-local and therefore does not enter the reusable policy definition.

### Snapshot fingerprints

- `state_fingerprint` covers schema identity, repository/workspace identity, exact policy/profile/envelope fingerprints, every selected family composite and per-slot pre/captured/post revision, bound evaluation, window capture inputs, payload fingerprint, exclusions, and redaction outcomes. It excludes snapshot ID, boundary-stream ref, boundary sequence, trigger, timestamps, retry count, previous-snapshot link, and both fingerprint fields.
- `record_fingerprint` covers the normalized complete immutable record except itself.
- Map keys, paths, unordered semantic sets, work-item windows, and evidence refs use their contract-defined canonical deterministic ordering.
- Two records captured at different times may have equal state fingerprints.

### Snapshot authority

Snapshot Memory is descriptive evidence. It cannot:

- lock or mutate a contract;
- replace canonical artifacts;
- rewrite a queue or handoff;
- infer why a divergence occurred;
- pass claims beyond captured/observed state.

Snapshot refs in handoffs, evidence, deltas, and projections always include or resolve the exact `record_fingerprint`; a bare mutable snapshot ID is insufficient.

The policy's exact predecessor rule defines eligible trigger transitions and ordering within `boundary_stream_ref`. Sequence values are unique and strictly increasing within each repository/workspace/stream; concurrent allocation collision must retry or refuse rather than introduce a tie-breaker. When `previous_snapshot` is present it must be the highest eligible sequence lower than the current record and its recorded sequence must match the referenced record. Self-links, forward links, duplicate sequences, cycles, wrong-stream/workspace links, skipped eligible predecessors, and disallowed trigger transitions fail closed.

## Snapshot delta

```yaml
schema_id: handbook.snapshot-delta
schema_version: "1.0"
delta_id: delta_...
from_snapshot: { ref: snap_previous, record_fingerprint: sha256:..., state_fingerprint: sha256:... }
to_snapshot: { ref: snap_current, record_fingerprint: sha256:..., state_fingerprint: sha256:... }
compatibility:
  comparison_contract: { ref: handbook.snapshot-comparison.core@1.0.0, fingerprint: sha256:... }
  drift_rule_catalog: { ref: handbook.snapshot-drift.core@1.0.0, fingerprint: sha256:... }
  compared_state_families:
    - git
    - handbook
    - work
    - session
    - evidence
  excluded_state_families: []
changes:
  - change_id: work.completed.example
    family: work
    stable_key: work-item.example
    change_kind: completed
    before_fingerprint: sha256:...
    after_fingerprint: sha256:...
rule_evaluations:
  - { rule: { ref: handbook.snapshot-drift.expected-progress@1.0.0, fingerprint: sha256:... }, outcome: matched, signal_id: signal.expected-progress.example }
  - { rule: { ref: handbook.snapshot-drift.justified-divergence@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
  - { rule: { ref: handbook.snapshot-drift.unexplained-drift@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
  - { rule: { ref: handbook.snapshot-drift.scope-expansion@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
  - { rule: { ref: handbook.snapshot-drift.execution-inefficiency@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
  - { rule: { ref: handbook.snapshot-drift.planning-inaccuracy@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
  - { rule: { ref: handbook.snapshot-drift.proof-drift@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
  - { rule: { ref: handbook.snapshot-drift.semantic-drift@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
  - { rule: { ref: handbook.snapshot-drift.stale-handoff@1.0.0, fingerprint: sha256:... }, outcome: not_matched, signal_id: null }
signals:
  - signal_id: signal.expected-progress.example
    kind: expected_progress
    rule: { ref: handbook.snapshot-drift.expected-progress@1.0.0, fingerprint: sha256:... }
    change_ids:
      - work.completed.example
    evidence_refs: []
    justification_refs: []
delta_fingerprint: sha256:...
```

The two snapshots must have stable/bounded admissibility, the same repository/workspace identity, supported snapshot schema versions, and policies/state-family adapters admitted by the exact comparison contract. The delta's exact drift-rule catalog must equal the catalog selected by both endpoint policies or be explicitly admitted as compatible by that comparison contract. Every family selected by either policy is compared or appears once in `excluded_state_families` with `absent`, `redacted`, `unstable`, or `incompatible`. A missing family or path is never silently treated as unchanged.

Every change carries a deterministic stable key, typed change kind, and before/after fingerprints (`null` only for creation/deletion). Domain-specific derived lists such as completed work, queue changes, proof gates, artifacts, contracts, blockers, and handoff drift are views over this normalized change set, not second delta authorities.

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

Every exact rule in the bound catalog is evaluated exactly once in catalog order and recorded in `rule_evaluations` as `matched`, `not_matched`, or `not_applicable`; applicability is owned by the rule definition, not the caller. Every matched evaluation names exactly one unique signal, every signal resolves back to exactly one matched evaluation with the same rule pair, and non-matched/not-applicable evaluations have null signal IDs. Missing, duplicate, stale, refused, or contradictory evaluation prevents delta creation. Thus `signals` is exactly the deterministic set of all matching catalog rules, including legitimate overlaps rather than a caller-selected subset.

Signals identify their stable ID, exact drift-rule ref/fingerprint, matching change IDs, evidence, and durable justification refs. Every signal rule is a member of the bound catalog. An uncataloged rule, stale catalog fingerprint, or catalog not selected/admitted by the endpoints and comparison contract refuses delta creation. `justified_divergence` requires at least one authoritative decision/handoff/escalation/child-packet justification ref admitted by its rule. Signals do not make an unreviewed causal claim, and free-form explanation never changes deterministic classification.

`delta_fingerprint` covers both exact snapshot input pairs, comparison contract, drift-rule catalog, compared/excluded families, normalized changes, complete rule evaluations, signals, and justification refs except itself. Reversing inputs is a different delta. Incompatible or unstable inputs refuse rather than emitting an empty/green delta.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| schema identity and `delta_id` | Handbook schema registry/engine | none | exact schema; stable unique ID | not a mutable comparison cursor |
| snapshot input pairs | immutable snapshots own observed endpoints | none | exact record/state fingerprints; ordered from/to; admissible consistency; same repository/workspace | no bare snapshot ID or reversed equivalence |
| comparison contract and drift catalog | comparison definition owns compatibility/key semantics; endpoint policies/catalog own deterministic signal closure | none | exact pairs; comparison admits both policies/schemas/adapters and any compatible catalog; every signal rule is a catalog member | no best-effort comparison or ambient drift rule |
| compared/excluded families | engine records comparison coverage | none | every selected family appears exactly once; exclusions typed | no missing-as-unchanged behavior |
| `changes` | engine derives normalized observed differences | empty only when compared state is equal | stable keys; typed kind; correct before/after nullability/fingerprints; deterministic order | no causal interpretation |
| `rule_evaluations` | exact catalog/rules own applicability and matching; engine records closure | none; one entry per catalog rule | exact catalog order and complete unique rule set; typed outcome; matched-to-signal bijection; no refused/duplicate/missing evaluation | no caller-selected evaluation subset |
| `signals` | exact drift rules own classification | empty when no rule matches | exact rule pair is a member of the bound catalog; referenced change IDs exist; required evidence/justification present | no free-form/model-owned or uncataloged reclassification |
| `delta_fingerprint` | Handbook derives immutable delta identity | none | normalized complete delta except itself | no timestamp or diagnostic input |

## Snapshot-grounding Projection definition

Snapshot grounding uses an ordinary exact `ProjectionDefinition`; the required currentness-family set is base definition behavior, not an extension or request hint:

```yaml
schema_id: handbook.projection-definition
schema_version: "1.0"
projection_definition_id: handbook.projection.snapshot-grounding
projection_definition_version: "1.0.0"
source_selectors:
  - { selector_id: snapshot_current, source_kind: snapshot, source_ref: handbook.context-memory-snapshot@1.0, cardinality: exactly_one }
  - { selector_id: snapshot_delta, source_kind: snapshot_delta, source_ref: handbook.snapshot-delta@1.0, cardinality: exactly_one }
allowed_surfaces: [agent_packet]
allowed_operations: [reveal]
target_schema: { ref: handbook.schemas.projection.snapshot-grounding@1.0.0, fingerprint: sha256:... }
disclosure_policy: { ref: handbook.projection-disclosure.core@1.0.0, fingerprint: sha256:... }
support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }
currentness_requirements:
  mode: exact_revision_check
  revision_basis: captured_revision
  families:
    - { family: git, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.git@1.0.0, fingerprint: sha256:... }, source_slots: [] }
    - { family: handbook, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.handbook@1.0.0, fingerprint: sha256:... }, source_slots: [] }
    - { family: work, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.work@1.0.0, fingerprint: sha256:... }, source_slots: [work_ledger, active_plan] }
    - { family: session, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.session@1.0.0, fingerprint: sha256:... }, source_slots: [] }
    - { family: evidence, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.evidence@1.0.0, fingerprint: sha256:... }, source_slots: [] }
field_rules:
  - { rule_id: reveal_active_work, operation: reveal, source_selector_id: snapshot_current, source_pointer: /family_observations/work/payload/active_refs, target_pointer: /active_work, minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: normal, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: execution, validation_horizon: unit_closeout }, disclosure_classification: internal, required_for_result: true, claim_refs: [], derivation: null }
  - { rule_id: reveal_changed_paths, operation: reveal, source_selector_id: snapshot_current, source_pointer: /family_observations/git/payload/dirty_paths, target_pointer: /changed_paths, minimum_resolution: { scope_horizon: local_observation, detail_resolution: summary, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: operation, validation_horizon: observation_only }, disclosure_classification: internal, required_for_result: true, claim_refs: [], derivation: null }
  - { rule_id: reveal_unresolved_blockers, operation: reveal, source_selector_id: snapshot_current, source_pointer: /family_observations/work/payload/blocked_refs, target_pointer: /unresolved_blockers, minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: normal, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: execution, validation_horizon: unit_closeout }, disclosure_classification: internal, required_for_result: true, claim_refs: [], derivation: null }
  - { rule_id: reveal_queued_next, operation: reveal, source_selector_id: snapshot_current, source_pointer: /family_observations/work/payload/queued_next, target_pointer: /queued_next, minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: summary, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: execution, validation_horizon: observation_only }, disclosure_classification: internal, required_for_result: true, claim_refs: [], derivation: null }
  - { rule_id: reveal_applicable_contracts, operation: reveal, source_selector_id: snapshot_current, source_pointer: /family_observations/handbook/payload/contracts, target_pointer: /applicable_contracts, minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: normal, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: execution, validation_horizon: unit_closeout }, disclosure_classification: internal, required_for_result: true, claim_refs: [], derivation: null }
  - { rule_id: reveal_proof_obligations, operation: reveal, source_selector_id: snapshot_current, source_pointer: /family_observations/evidence/payload/unresolved_proof_refs, target_pointer: /proof_obligations, minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: normal, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: execution, validation_horizon: unit_closeout }, disclosure_classification: internal, required_for_result: true, claim_refs: [], derivation: null }
  - { rule_id: reveal_delta_signals, operation: reveal, source_selector_id: snapshot_delta, source_pointer: /signals, target_pointer: /recent_signals, minimum_resolution: { scope_horizon: assigned_unit, detail_resolution: summary, temporal_horizon: immediate, authority_horizon: read_only, memory_horizon: execution, validation_horizon: observation_only }, disclosure_classification: internal, required_for_result: true, claim_refs: [], derivation: null }
extensions: {}
definition_fingerprint: sha256:...
```

The request's `currentness.expected_family_revisions` and the result's `currentness_validation.checks` must each equal the definition's required family set exactly. Family IDs, source-selector IDs, adapters, and source-slot sets must match. Each request value is copied from the exact bound snapshot's captured composite/per-slot revisions, and every result observed value must equal it. Omission, extra entries, duplicate families, selector/adapter/slot substitution, or values not equal to the bound captured state refuses before output. Because this definition reveals the delta's unfiltered `/signals`, its currentness set covers every family compared by that delta, including `session`; a definition that omits one must instead filter/omit signals derived from unchecked families with typed proof effects.

## Snapshot projection request/result

```yaml
schema_id: handbook.snapshot-projection-request
schema_version: "1.0"
request_id: snapshot-projection-request.example
sources:
  - { selector_id: snapshot_current, ref: snap_current, fingerprint: sha256:..., state_fingerprint: sha256:... }
  - { selector_id: snapshot_delta, ref: delta_previous_to_current, fingerprint: sha256:... }
resolved_profile: { ref: handbook.profile.example@1.0.0, fingerprint: sha256:... }
vocabulary: { ref: handbook.vocabulary.example@1.0.0, fingerprint: sha256:... }
projection_definition: { ref: handbook.projection.snapshot-grounding@1.0.0, fingerprint: sha256:... }
resolution_envelope: { ref: envelope.example.execution, fingerprint: sha256:... }
purpose: session_grounding
surface: agent_packet
operation: reveal
currentness:
  mode: exact_revision_check
  revision_basis: captured_revision
  expected_family_revisions:
    - { family: git, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.git@1.0.0, fingerprint: sha256:... }, revision: "git:abc" }
    - { family: handbook, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.handbook@1.0.0, fingerprint: sha256:... }, revision: "handbook-state:7" }
    - family: work
      source_selector_id: snapshot_current
      source_adapter: { ref: handbook.snapshot-source.work@1.0.0, fingerprint: sha256:... }
      revision: "composite:sha256:..."
      source_slot_revisions: { work_ledger: "work:4", active_plan: "plan:4" }
    - { family: session, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.session@1.0.0, fingerprint: sha256:... }, revision: "session:3" }
    - { family: evidence, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.evidence@1.0.0, fingerprint: sha256:... }, revision: "evidence:9" }
```

```yaml
schema_id: handbook.snapshot-projection-result
schema_version: "1.0"
result_id: snapshot-projection-result.example
request_ref: snapshot-projection-request.example
sources:
  - { selector_id: snapshot_current, ref: snap_current, fingerprint: sha256:..., state_fingerprint: sha256:... }
  - { selector_id: snapshot_delta, ref: delta_previous_to_current, fingerprint: sha256:... }
resolved_profile: { ref: handbook.profile.example@1.0.0, fingerprint: sha256:... }
vocabulary: { ref: handbook.vocabulary.example@1.0.0, fingerprint: sha256:... }
projection_definition: { ref: handbook.projection.snapshot-grounding@1.0.0, fingerprint: sha256:... }
resolution_envelope: { ref: envelope.example.execution, fingerprint: sha256:... }
surface: agent_packet
operation: reveal
disclosure_evaluations:
  - { rule_id: reveal_active_work, definition_rule_ordinal: 1, minimum_resolution_outcome: sufficient, upstream_redaction_check: none, upstream_redaction_disposition: null, policy_evaluation: matched, matched_policy_rule_ids: [internal_allow], policy_action: allow, support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }, support_status: supported, support_reason: null, payload_access: read, final_disposition: included, evaluation_fingerprint: sha256:... }
  - { rule_id: reveal_changed_paths, definition_rule_ordinal: 2, minimum_resolution_outcome: sufficient, upstream_redaction_check: none, upstream_redaction_disposition: null, policy_evaluation: matched, matched_policy_rule_ids: [internal_allow], policy_action: allow, support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }, support_status: supported, support_reason: null, payload_access: read, final_disposition: included, evaluation_fingerprint: sha256:... }
  - { rule_id: reveal_unresolved_blockers, definition_rule_ordinal: 3, minimum_resolution_outcome: sufficient, upstream_redaction_check: none, upstream_redaction_disposition: null, policy_evaluation: matched, matched_policy_rule_ids: [internal_allow], policy_action: allow, support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }, support_status: supported, support_reason: null, payload_access: read, final_disposition: included, evaluation_fingerprint: sha256:... }
  - { rule_id: reveal_queued_next, definition_rule_ordinal: 4, minimum_resolution_outcome: sufficient, upstream_redaction_check: none, upstream_redaction_disposition: null, policy_evaluation: matched, matched_policy_rule_ids: [internal_allow], policy_action: allow, support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }, support_status: supported, support_reason: null, payload_access: read, final_disposition: included, evaluation_fingerprint: sha256:... }
  - { rule_id: reveal_applicable_contracts, definition_rule_ordinal: 5, minimum_resolution_outcome: sufficient, upstream_redaction_check: none, upstream_redaction_disposition: null, policy_evaluation: matched, matched_policy_rule_ids: [internal_allow], policy_action: allow, support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }, support_status: supported, support_reason: null, payload_access: read, final_disposition: included, evaluation_fingerprint: sha256:... }
  - { rule_id: reveal_proof_obligations, definition_rule_ordinal: 6, minimum_resolution_outcome: sufficient, upstream_redaction_check: none, upstream_redaction_disposition: null, policy_evaluation: matched, matched_policy_rule_ids: [internal_allow], policy_action: allow, support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }, support_status: supported, support_reason: null, payload_access: read, final_disposition: included, evaluation_fingerprint: sha256:... }
  - { rule_id: reveal_delta_signals, definition_rule_ordinal: 7, minimum_resolution_outcome: sufficient, upstream_redaction_check: none, upstream_redaction_disposition: null, policy_evaluation: matched, matched_policy_rule_ids: [internal_allow], policy_action: allow, support_evaluator: { ref: handbook.projection-support.core@1.0.0, fingerprint: sha256:... }, support_status: supported, support_reason: null, payload_access: read, final_disposition: included, evaluation_fingerprint: sha256:... }
included:
  - { rule_id: reveal_active_work, source_pointer: /family_observations/work/payload/active_refs, target_pointer: /active_work, claim_refs: [] }
  - { rule_id: reveal_changed_paths, source_pointer: /family_observations/git/payload/dirty_paths, target_pointer: /changed_paths, claim_refs: [] }
  - { rule_id: reveal_unresolved_blockers, source_pointer: /family_observations/work/payload/blocked_refs, target_pointer: /unresolved_blockers, claim_refs: [] }
  - { rule_id: reveal_queued_next, source_pointer: /family_observations/work/payload/queued_next, target_pointer: /queued_next, claim_refs: [] }
  - { rule_id: reveal_applicable_contracts, source_pointer: /family_observations/handbook/payload/contracts, target_pointer: /applicable_contracts, claim_refs: [] }
  - { rule_id: reveal_proof_obligations, source_pointer: /family_observations/evidence/payload/unresolved_proof_refs, target_pointer: /proof_obligations, claim_refs: [] }
  - { rule_id: reveal_delta_signals, source_pointer: /signals, target_pointer: /recent_signals, claim_refs: [] }
omissions: []
not_applicable: []
derivations: []
output: { schema_ref: handbook.schemas.projection.snapshot-grounding@1.0.0, content_ref: grounding.example, content_fingerprint: sha256:... }
currentness_validation:
  mode: exact_revision_check
  revision_basis: captured_revision
  checks:
    - { family: git, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.git@1.0.0, fingerprint: sha256:... }, expected_revision: "git:abc", observed_revision: "git:abc", outcome: current, evidence_fingerprint: sha256:... }
    - { family: handbook, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.handbook@1.0.0, fingerprint: sha256:... }, expected_revision: "handbook-state:7", observed_revision: "handbook-state:7", outcome: current, evidence_fingerprint: sha256:... }
    - family: work
      source_selector_id: snapshot_current
      source_adapter: { ref: handbook.snapshot-source.work@1.0.0, fingerprint: sha256:... }
      expected_revision: "composite:sha256:..."
      observed_revision: "composite:sha256:..."
      expected_source_slot_revisions: { work_ledger: "work:4", active_plan: "plan:4" }
      observed_source_slot_revisions: { work_ledger: "work:4", active_plan: "plan:4" }
      outcome: current
      evidence_fingerprint: sha256:...
    - { family: session, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.session@1.0.0, fingerprint: sha256:... }, expected_revision: "session:3", observed_revision: "session:3", outcome: current, evidence_fingerprint: sha256:... }
    - { family: evidence, source_selector_id: snapshot_current, source_adapter: { ref: handbook.snapshot-source.evidence@1.0.0, fingerprint: sha256:... }, expected_revision: "evidence:9", observed_revision: "evidence:9", outcome: current, evidence_fingerprint: sha256:... }
lossiness: lossless
authority_effect: none
result_fingerprint: sha256:...
```

These are specialized capitalized Projection DTOs, not a second projection model. Their schemas extend the generic request/result schemas without removing or renaming any generic required field. The example definition declares exactly-one `snapshot_current` and `snapshot_delta` selectors, so both entries appear in generic `sources`; a different exact definition may omit the delta selector entirely, but v1 never makes one exactly-one selector optional at request time. Snapshot sources use `fingerprint` for the exact record fingerprint and additionally carry `state_fingerprint`. Generic `resolution_envelope`, surface, operation, `disclosure_evaluations`, included/omissions/not-applicable/derivations, output, lossiness, authority, and result-fingerprint fields retain their generic meanings. The snapshot-grounding definition also declares the exact source families whose current revisions must be revalidated.

Additional snapshot-projection gates:

- included fields fit the target Resolution authority and detail horizons;
- omitted sensitive, unavailable, or out-of-scope fields remain enumerated with proof effect;
- comprehensive capture does not imply comprehensive disclosure;
- grounding projection never mutates the source snapshot;
- snapshot/delta fingerprints remain traceable through their generic source-selector entries;
- when the definition includes a `snapshot_delta` selector, that delta names the `snapshot_current` source as its `to_snapshot`;
- a caller that requires fresh capture completes it before constructing the Projection request, then binds the resulting exact snapshot/delta sources; every current-grounding request uses definition-matching `exact_revision_check`, the result retains identical sources and records one typed check per definition-required family including exact selector/composite/per-slot revisions, and any observed mismatch returns a typed stale refusal instead of a result.

| Specialized field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `sources` entries for `snapshot_current`/`snapshot_delta` | immutable snapshot/delta own observed and changed state | exactly as declared by definition; both required in this example | generic exactly-one selector rules; exact fingerprints; snapshot stable/bounded; delta `to_snapshot` equals selected snapshot | no specialized alias, implicit optional selector, or full-record injection |
| definition disclosure policy/rule minima/classifications and result `disclosure_evaluations` | generic Projection disclosure contract owns metadata-only decision order; snapshot/delta dispositions own prior redaction | no specialization default; every generic field remains required | exact policy/fingerprint; complete six-dimension/classification rules; one replayable evaluation per applicable rule; covering upstream action maps original pointer to `redacted`; no protected read before allow | no snapshot-only policy, `unavailable` downgrade, hidden reread, content sniff, or disclosure field drop |
| specialized included/omission/not-applicable/derivation/output | generic Projection contract owns view semantics | as generic contract | all generic disjoint rule accounting plus exact snapshot/delta provenance | no second snapshot-only projection semantics |
| request `currentness` | snapshot-grounding definition owns tuple set/basis; bound snapshot owns expected values | none; mode `exact_revision_check`, basis `captured_revision` | mode/basis equal definition; tuples equal definition; values equal exact bound snapshot family/slot captured revisions | no caller-supplied alternate live value, omitted/extra family, selector/adapter/slot substitution, placeholder source, or post-request substitution |
| result `currentness_validation` | source adapters own current observed revisions; engine records proof | none | check set equals definition/request; expected values equal bound captured state; observed equals expected; all `current`; request/result sources identical | no result on stale/partial validation, plan/session-only drift, or bounded captured/post mismatch |

V1 has no caller-selected family or rule subset. For the requested operation, the exact definition's applicable field rules are the complete rule set that must be evaluated and partitioned. The resolved envelope, redaction policy, source availability, and schema support yield the already-defined typed omissions with their proof effects; they never yield `not_applicable` and do not remove a rule from accounting. Operation mismatch alone yields `not_applicable`. An `include_families` field or any other caller-supplied subset selector is unknown input and produces a typed refusal with no Projection result.

`currentness_validation` is included in `result_fingerprint`. Fresh capture is a pre-request operation: only after it completes may its exact snapshot fingerprint populate `sources`. The engine never swaps that source after request construction. The engine reads captured values from that bound record before observing live revisions; it never trusts caller equality alone. In a bounded snapshot whose captured and post revisions differ, exact-current grounding refuses unless the live observed value again equals the captured value. A mismatch, missing family, unchecked delta-signal family, stale adapter, unavailable check, placeholder pre-capture source, or request/result source difference emits a typed refusal with no Projection output/result fingerprint.

## Snapshot redaction and retention

```yaml
schema_id: handbook.snapshot-redaction-policy
schema_version: "1.0"
policy_id: handbook.snapshot-redaction.default
policy_version: "1.0.0"
fail_closed: true
unmatched_action: omit
rules:
  - rule_id: secret_values
    matcher: { surface_kind: classified_secret, selector: "*" }
    action: omit
  - rule_id: unrestricted_environment
    matcher: { surface_kind: environment_value, selector: unrestricted }
    action: omit
  - rule_id: secret_files
    matcher: { surface_kind: secret_file, selector: "**" }
    action: omit
  - rule_id: raw_command_arguments
    matcher: { surface_kind: command_argument, selector: raw }
    action: omit
  - rule_id: raw_command_output
    matcher: { surface_kind: command_output, selector: unrestricted }
    action: omit
  - rule_id: unrestricted_diff
    matcher: { surface_kind: repository_diff, selector: unrestricted_full_content }
    action: omit
  - rule_id: environment_metadata
    matcher: { surface_kind: environment_value, selector: allowlisted_non_secret }
    action: fingerprint_only
extensions: {}
policy_fingerprint: sha256:...
```

Actions are `omit`, `fingerprint_only`, `artifact_ref_only`, or `redacted_summary`. V1 requires `fail_closed: true` and `unmatched_action: omit`; unknown classification, matcher failure, and a known surface matching no rule therefore all omit. Rule order makes matching replay-stable but does not select among actions. Multiple matching rules with the same action are valid; if any matching action is `omit`, `omit` wins; two or more distinct matching non-omit actions are incomparable and capture refuses.

```yaml
schema_id: handbook.snapshot-redaction-disposition
schema_version: "1.0"
disposition_id: snapshot-redaction-disposition.example
source_family: session
source_adapter: { ref: handbook.snapshot-source.session@1.0.0, fingerprint: sha256:... }
original_pointer: /family_observations/session/payload/environment/PATH
matched_rule_ids: [environment_metadata]
action: fingerprint_only
reason: matched_policy
pre_redaction_value_fingerprint: sha256:...
retained:
  kind: fingerprint
  pointer: /family_observations/session/payload/environment_fingerprints/PATH
  content_ref: null
  content_fingerprint: sha256:...
disposition_fingerprint: sha256:...
```

Every snapshot `redaction.dispositions` entry is an exact `{ref, fingerprint}` pair to one immutable disposition. `original_pointer` is the exact JSON Pointer hidden by the action and covers that pointer plus its descendants by JSON Pointer segment boundaries, never by string-prefix matching. `retained` uses this exact action matrix:

| `action` | `retained.kind` | `retained.pointer` | `retained.content_ref` | `retained.content_fingerprint` |
|---|---|---|---|---|
| `omit` | `none` | `null` | `null` | `null` |
| `fingerprint_only` | `fingerprint` | required | `null` | required |
| `artifact_ref_only` | `artifact_ref` | required | required | required |
| `redacted_summary` | `redacted_summary` | required | required | required |

A non-null retained pointer is a schema-valid pointer outside the original pointer's subtree; it may share earlier path segments but is never equal to or a descendant of `original_pointer`. Therefore the disposition covers the original value/subtree only and never automatically covers the retained field. Projection maps a request for the original pointer or its descendants to `redacted` with zero value reads, while a request for the exact retained pointer evaluates that retained field independently through its own rule classification and disclosure policy. Matched rule IDs retain policy order. `pre_redaction_value_fingerprint` is non-null only when safe to compute. `disposition_fingerprint` covers the normalized complete record and exact adapter fingerprint except itself. Missing/ambiguous original/retained mapping, action-nullability mismatch, duplicate disposition identity, or a retained pointer inside the original subtree refuses snapshot creation.

By default snapshots exclude:

- secret values and credential material;
- unrestricted environment variables;
- `.env` and secret-file contents;
- raw command arguments/output that may carry secrets;
- full diffs when normalized statistics/fingerprints and evidence refs suffice.

No invocation flag may weaken those floors. A stricter selected policy may omit more.

```yaml
schema_id: handbook.snapshot-retention-policy
schema_version: "1.0"
policy_id: handbook.snapshot-retention.session
policy_version: "1.0.0"
rules:
  - rule_id: strategic_milestone
    memory_horizons: [strategic]
    triggers: [gate_complete, publish]
    record_classes: [context_memory_snapshot]
    action: retain_indefinitely
    minimum_retention_seconds: null
  - rule_id: execution_session
    memory_horizons: [execution, operation]
    triggers: [session_start, session_end]
    record_classes: [context_memory_snapshot]
    action: retention_window
    minimum_retention_seconds: 2592000
deduplication: content_addressed_payload_only
compaction:
  allowed: true
  requires_review: true
  preserve_source_refs: true
extensions: {}
policy_fingerprint: sha256:...
```

Retention actions are `retain_indefinitely` or `retention_window`; content-addressed payload deduplication is a storage optimization, not a record action. Every `(memory_horizon, trigger, record_class)` selected by the capture policy resolves exactly one retention rule. A record referenced by a handoff, evidence, verdict, gate, active promotion, legal hold, or unexpired stricter rule cannot be deleted. Deduplication preserves distinct record identities. Compaction creates a new reviewed aggregate with exact source ref/fingerprint pairs and never rewrites retained source bytes.

Both policy fingerprints use uniform exact-definition identity over the normalized policy and referenced matcher/classification definitions, including `unmatched_action` and excluding only their own fingerprint. Secret fixtures and unsafe paths/arguments/output must have explicit negative tests before implementation gates can close.

| Field family | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| policy schema/identity | Handbook schema registry and policy author | none | exact schema; stable ID plus SemVer; unique exact ref | no ambient/latest policy |
| redaction `fail_closed`/`unmatched_action`/rules | security policy owns minimum disclosure | v1 requires true/`omit`/non-empty | explicit secret, unrestricted environment, secret-file, raw command argument/output, and unrestricted full-diff floors; known matcher surfaces/actions; identical overlap valid; `omit` wins; distinct non-omit overlap refuses | no undeclared action ranking, permissive unmatched surface, invocation weakening, or secret-bearing diagnostic |
| redaction dispositions | capture engine records immutable applied/default outcomes | empty matched-rule IDs only for default omission | exact source adapter; original JSON Pointer/subtree; action-specific retained kind/pointer/ref/fingerprint; policy-order matched IDs; safe pre-fingerprint; disposition fingerprint; snapshot stores exact pair | no hidden omission, string-prefix coverage, ambiguous original/retained identity, retained descendant, or later mutation |
| retention rules | retention authority owns minimum storage lifetime by horizon/trigger/record-class tuple | none | every selected tuple resolves exactly one rule without overlap; known record class; positive window when used | no automatic deletion policy from frequency alone |
| deduplication/compaction | storage policy owns safe physical optimization | dedupe optional; compaction disabled unless explicit | distinct records preserved; reviewed aggregate/source refs; holds/references/floors honored | no byte rewrite or authority merge |
| policy fingerprints | Handbook derives exact policy identity | none | normalized policy plus exact referenced matcher/classification fingerprints except itself | no unchanged fingerprint after semantic drift |

### Shared HCM-0.3 extension rule

Every HCM-0.3 definition that exposes `extensions` uses the same field contract: the declaring record schema owns namespaced optional additions; the default is an explicit empty map; each populated namespace resolves an exact extension schema and may add only optional semantics; the complete normalized map and extension-schema fingerprints enter the record's definition/policy fingerprint. Unknown required behavior, unregistered namespaces, executable hooks, remote code, and extensions that weaken a base invariant fail closed. Records that do not show an `extensions` field do not accept one.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| HCM-0.3 `extensions` | declaring schema owns namespaced optional additions | explicit empty map | registered exact extension schemas; optional-only semantics; included in enclosing fingerprint closure | no unknown required behavior, base-rule override, executable hook, or ambient namespace |

### HCM-0.3 required conformance scenarios

Later implementation packets must turn these design examples into mechanical schema/semantic tests:

| Contract | Positive scenarios | Negative/fail-closed scenarios |
|---|---|---|
| stack/envelope | compare every adjacent level across all six domains; equal/narrow child; exact and wildcard allow/deny overlap resolves deny | one rank increase; malformed/indeterminate selector; stale parent; changed stack/profile |
| escalation/promotion transitions | request-only pending state followed by exactly one terminal disposition; approval/application creates exact new envelope/memory result while prior bytes replay unchanged | in-place status/outcome mutation; duplicate disposition; reused ID with changed bytes; stale request; invalid outcome-specific nullability |
| Projection sources | one exact source identity per typed selector; canonical artifacts and immutable semantic/observation records including snapshot/delta sources retain their authority class; definition belongs to resolved-profile catalog; target schema, allowed surfaces/operations, mandatory currentness, profile stack/catalog, and source/schema/kind/capability closure resolve | zero/multiple identity matches; source-order fallback; snapshot/delta rejected merely for being non-canonical or promoted to canonical/peer authority; definition absent from profile catalog; unresolved source/target schema; catalog gate invents a source-role or target-level field absent from `ProjectionDefinition` |
| Projection disclosure | same canonical/snapshot/delta rule included under a sufficient envelope and `out_of_resolution` under a narrower envelope; matched allow/redact plus registered-unmatched redaction; evaluator fingerprint recomputes from exact dependencies/input/reason order, supported/unsupported plus first reason replay metadata-only, and evaluator-only drift changes evaluator/definition/profile/evaluation/result fingerprints; exact evaluation-outcome/nullability matrix and mixed-rule ordering replay; all four upstream actions map original pointer/subtree to `redacted`; retained fields sharing path segments evaluate independently; protected-source spy records zero reads before allow | unregistered definition classification accepted or converted to result; missing/stale/incompatible policy, classification registry, evaluator, or evaluator dependency produces a result; ambient evaluator substitution; unsupported free-form/wrong-precedence reason; incomplete/invalid minimum ranks; permissive registered-unmatched tuple; action-nullability/ordinal/fingerprint mismatch; string-prefix original/retained confusion; upstream redaction mapped `unavailable`; hidden payload reread; fingerprint unchanged after disclosure/support drift |
| Projection accounting/lossiness | request-operation partitions definition rules exactly across included/omissions/not-applicable; exactly bound fields denied by envelope, redacted, unavailable, or runtime-unsupported produce typed omissions; every reason crosses required/non-required plus empty/non-empty claims using the exact proof-effect/output-schema table; non-lossy/lossy derivations; each mixed-reason precedence row | request-wide envelope-visibility precondition; valid bound applicable source refused solely for envelope/redaction/availability/support; target schema cannot represent typed absence; missing/duplicate rule accounting; those four outcomes classified `not_applicable`; non-operation applicability condition; caller-selected lossiness; required omission suppresses result or falsely passes |
| capture windows/families | same policy with changed live source revisions/cursors produces new snapshot fingerprints; exclusive cursor boundary replay; selected families equal observed plus excluded; work windows bind declared slots | revision/cursor mutates policy identity; ambient extra family; silently missing family/slot |
| consistency | all-stable/no-exclusion aggregation; mixed stable/bounded/no-exclusion aggregation; exact composite/per-slot revisions and policy-selected bound evaluation | any excluded/unstable family labeled stable/bounded or grounding; active-plan-only drift; substituted/stale bound rule; out-of-bound retry; completed-read-only bounded claim |
| predecessor | valid immediate prior-end/new-start or start/end boundary transition with unique strictly increasing sequence under exact rule | self, future, duplicate sequence/collision tie-breaker, cycle, skipped eligible predecessor, wrong stream/workspace/trigger |
| delta | every selected family appears once in compared/excluded; normalized changes replay; every catalog rule is evaluated once and matched evaluations map bijectively to signals | incompatible/unstable inputs; missing family treated unchanged; reversed input equivalence; uncataloged signal; omitted/duplicate/contradictory evaluation; stale/incompatible catalog |
| snapshot Projection | definition owns the complete operation rule set and captured-revision family/selector/adapter/slot closure; request values equal bound snapshot captured state; observations equal request; unfiltered delta signals cover every compared family; fresh capture precedes request | caller family/rule subset selector including unknown `include_families`; stale snapshot plus unrelated equal live values; bounded captured/post mismatch; session-only drift; unchecked-family signal; omitted/extra/duplicate tuple; selector/adapter/slot mismatch; post-request substitution; incomplete rule accounting |
| redaction | identical-action overlap; `omit` overlap; explicit sensitive-surface floors; unknown, matcher-failed, and known-unmatched surfaces default to omit; every action satisfies exact original/retained pointer/nullability matrix including containing-value coverage and shared-prefix retained fields | distinct non-omit overlap; permissive unmatched surface; known-unmatched environment/secret-file/command/diff content retained; retained pointer equal to/under original subtree; ambiguous prefix coverage; action/nullability mismatch |
| retention | every allowed memory-horizon/declared-trigger/context-memory-snapshot tuple resolves exactly one rule | envelope horizon outside policy; uncovered/overlapping tuple; deletion under reference/hold/floor |
| field matrix | mechanical example-key-to-owning matrix coverage including shared `extensions` | undefined fingerprint input or unmatrixed semantic field |

## Project posture kernel and recommendation contracts

V1 posture levels are global per registered engineering dimension. A `causal_scope_ref` records where trigger evidence applies, but it is not a second posture-state coordinate, does not select a scoped level, and never changes the constitutional authority path. If scoped posture state is later required, it needs a separate contract and migration.

### Posture trigger definition

```yaml
schema_id: handbook.posture-trigger-definition
schema_version: "1.0"
trigger_id: handbook.posture-trigger.public-api-critical
trigger_version: "1.0.0"
trigger_kind: hard
signal_input:
  ref: handbook.signal.public-api-risk@1.0.0
  fingerprint: sha256:...
evidence_requirement_input:
  ref: handbook.evidence-requirement.public-api-risk@1.0.0
  fingerprint: sha256:...
freshness_policy_input:
  ref: handbook.freshness.release-risk@1.0.0
  fingerprint: sha256:...
dimension_id: testing_rigor
causal_scope_ref: artifact.public_api
proposed_transition:
  from: 4
  to: 5
default_confidence: high
default_urgency: before_next_release
required_approval_ref: charter.governance.project_owner
suggested_actions: []
trigger_fingerprint: sha256:...
```

A hard trigger is an immutable versioned definition. `trigger_fingerprint` is SHA-256 over every normalized field except itself and includes the exact signal, evidence-requirement, and freshness-policy fingerprints. One trigger binds one global dimension transition and one causal scope. It cannot authorize mutation, notification delivery, a second dimension, or a scoped authority path.

### Freshness evaluation basis

```yaml
schema_id: handbook.freshness-evaluation-basis
schema_version: "1.0"
basis_id: freshness_basis_...
evaluated_at_utc: "..."
source_states:
  - source_ref: evidence.public-api-risk
    source_fingerprint: sha256:...
    freshness_policy_ref: handbook.freshness.release-risk@1.0.0
    freshness_policy_fingerprint: sha256:...
    valid_from_utc: "..."
    expires_at_utc: "..."
    classification: fresh
basis_fingerprint: sha256:...
```

The freshness basis is an immutable deterministic semantic input, not Snapshot Memory. `classification` is `fresh`, `stale`, or `unknown` as of the explicit evaluation instant under the exact policy. Its fingerprint covers the evaluation instant, every source ref/fingerprint, freshness-policy ref/fingerprint, validity bound, and classification; it excludes `basis_id` and its own fingerprint. Reusing the same basis is stable. Evaluating before versus after expiry produces a different basis/fingerprint.

### Project posture kernel

```yaml
schema_id: handbook.project-posture-kernel
schema_version: "1.0"
kernel_id: posture_kernel_...
constitutional_artifact_ref: .handbook/example/root.yaml
constitutional_artifact_fingerprint: sha256:...
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
override_inputs: []
condition_inputs: []
contract_inputs: []
evidence_inputs:
  - ref: evidence.public-api-risk
    fingerprint: sha256:...
snapshot_inputs: []
freshness_basis_ref: freshness_basis_...
freshness_basis_fingerprint: sha256:...
dimensions:
  testing_rigor:
    level: 4
    floor: 3
    red_line_refs: []
    trigger_refs: []
    allowed_shortcut_refs: []
    proof_obligation_refs: []
applicable_scope_refs: []
omitted_condition_refs: []
unresolved_condition_refs: []
recommendation_refs: []
resolved_at_utc: "..."
input_fingerprint: sha256:...
kernel_fingerprint: sha256:...
```

`ProjectPostureKernel` is derived, not independently editable canonical truth. Every `*_inputs` entry is a typed `{ref, fingerprint}` pair. `input_fingerprint` covers schema identity, constitutional ref/fingerprint, exact profile ref plus `resolved_profile_fingerprint`, every normalized input pair, and the freshness-basis ref/fingerprint whenever any input is freshness-qualified. It excludes `kernel_id`, resolved output, recommendation refs, `resolved_at_utc`, and both fingerprint fields. `kernel_fingerprint` covers `input_fingerprint` plus global dimension state, applicability/explanation refs, and other resolved semantic output; it excludes `kernel_id`, recommendation refs, `resolved_at_utc`, and its own field. Lists whose order has no meaning are sorted by stable keys before hashing. Changed bytes behind a stable ref require a changed fingerprint or refusal.

### Posture evaluation policy

```yaml
schema_id: handbook.posture-evaluation-policy
schema_version: "1.0"
policy_id: handbook.posture-policy.example
policy_version: "1.0.0"
hard_trigger_inputs:
  - ref: handbook.posture-trigger.public-api-critical@1.0.0
    fingerprint: sha256:...
accumulated_signal_rules:
  - rule_id: repeated_release_regressions
    signal_input:
      ref: handbook.signal.release-regression@1.0.0
      fingerprint: sha256:...
    evidence_requirement_input:
      ref: handbook.evidence-requirement.release-regression@1.0.0
      fingerprint: sha256:...
    freshness_policy_input:
      ref: handbook.freshness.release-risk@1.0.0
      fingerprint: sha256:...
    dimension_id: testing_rigor
    causal_scope_ref: artifact.public_api
    comparator: at_least
    threshold: 3
    window: P30D
    minimum_observations: 3
    proposed_transition:
      from: 4
      to: 5
    default_confidence: high
    default_urgency: before_next_release
    required_approval_ref: charter.governance.project_owner
    suggested_actions: []
    rule_fingerprint: sha256:...
lowering_evidence_window: P90D
cooldown: P30D
recipient_refs:
  - charter.governance.project_owner
acknowledgment_required: true
escalate_after: P7D
extensions: {}
policy_fingerprint: sha256:...
```

Each accumulated rule is local to one exact policy version. `rule_fingerprint` covers every normalized rule field and cited input fingerprint except itself. A recommendation cites the policy pair, `rule_id`, and `rule_fingerprint`; it does not invent a synthetic external trigger ref. `policy_fingerprint` covers normalized policy identity/configuration, ordered hard-trigger pairs, accumulated rules including their fingerprints, notification semantics, hysteresis, extensions, and referenced definition fingerprints, excluding only itself. Hard triggers and accumulated rules each bind one global dimension transition and one causal scope.

### Posture recommendation

```yaml
schema_id: handbook.posture-recommendation
schema_version: "1.0"
recommendation_id: posture_rec_...
kernel_ref: posture_kernel_...
kernel_fingerprint: sha256:...
evaluation_policy_ref: handbook.posture-policy.example@1.0.0
evaluation_policy_fingerprint: sha256:...
proposed_transition:
  dimension_id: testing_rigor
  from: 4
  to: 5
causal_scope_ref: artifact.public_api
trigger_source:
  kind: hard_trigger
  ref: handbook.posture-trigger.public-api-critical@1.0.0
  fingerprint: sha256:...
  rule_id: null
  rule_fingerprint: null
triggering_observations:
  - public API blast radius is critical
evidence_inputs:
  - ref: evidence.public-api-risk
    fingerprint: sha256:...
snapshot_delta_inputs: []
confidence: high
urgency: before_next_release
required_approval_ref: charter.governance.project_owner
notification:
  recipient_refs:
    - charter.governance.project_owner
  acknowledgment_required: true
  escalate_after: P7D
suggested_actions: []
promotion_eligibility: recommendation_only
recommendation_fingerprint: sha256:...
```

V1 permits exactly one `proposed_transition` per recommendation. For `trigger_source.kind: hard_trigger`, `ref` and `fingerprint` are required and both rule fields are null. For `accumulated_rule`, the trigger ref/fingerprint fields are null while `rule_id` and `rule_fingerprint` identify a rule inside the cited exact policy. The transition, causal scope, confidence, urgency, approval, actions, evidence/freshness requirements, and notification must match the selected trigger/rule and policy. Two dimension changes produce two recommendations; recommendation grouping is presentation-only.

### Posture transition

```yaml
schema_id: handbook.posture-transition
schema_version: "1.0"
transition_id: posture_transition_...
recommendation_ref: posture_rec_...
recommendation_fingerprint: sha256:...
source_kernel_ref: posture_kernel_...
source_kernel_fingerprint: sha256:...
target_authority_ref: .handbook/example/root.yaml
target_authority_fingerprint: sha256:...
target_authority_class: constitutional_root
change:
  authority_path: /engineering_posture/dimensions/testing_rigor/level
  dimension_id: testing_rigor
  operation: replace
  expected_value: 4
  proposed_value: 5
approval_inputs:
  - ref: approval_...
    fingerprint: sha256:...
authorized_by_ref: charter.governance.project_owner
reassessment:
  intake_definition_ref: handbook.intake.charter@1.0.0
  affected_coverage_ids:
    - engineering_posture.dimensions
  validation_result_inputs:
    - ref: validation_...
      fingerprint: sha256:...
effective_at_utc: "..."
resulting_authority_fingerprint: sha256:...
resulting_kernel_ref: posture_kernel_...
resulting_kernel_fingerprint: sha256:...
transition_fingerprint: sha256:...
```

| Record/fields | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| trigger identity/kind | trigger author owns one immutable hard-trigger definition | none | exact schema; SemVer identity; `hard` is the only external trigger kind in v1 | accumulated rules are policy-local, not fake external triggers |
| trigger signal/evidence/freshness input pairs | source definition owners retain authority; trigger pins them | none | exact ref/fingerprint pairs; compatible definitions | no bare mutable ref or model-only evidence rule |
| trigger dimension/causal scope/transition | trigger author binds one deterministic proposal | none | registered global dimension; typed causal scope; `from`/`to` obey floors/red lines/direction | scope is not a posture-state coordinate or authority path |
| trigger guidance and fingerprint | trigger/policy author owns defaults; Handbook derives identity | actions empty; other guidance explicit | confidence/urgency/approval allowlisted; normalized fingerprint covers full semantic closure | trigger cannot authorize mutation or notification delivery |
| freshness-basis identity/evaluation instant | Handbook freshness evaluator records one immutable evaluation boundary | none when freshness-qualified input exists | exact schema; valid UTC; unique ID | presentation time is not a substitute |
| freshness source/policy pairs, validity, classification | source/policy remain authority; evaluator records state | bounds only as policy permits; classification required | exact current fingerprints; interval/order/classification agree at instant | stale does not silently count as fresh |
| `basis_fingerprint` | Handbook derives semantic time/applicability input | none | normalization stated above | no wall-clock lookup outside the record |
| kernel identity/constitutional/profile inputs | resolver records derived identity and exact constitutional plus fully resolved-profile inputs | none | exact ref/fingerprint pairs; `resolved_profile_fingerprint` current | no source-layer `profile_fingerprint` or second authority |
| kernel override/condition/contract/evidence/snapshot inputs | each source remains its own authority; resolver records exact pairs | lists empty except as applicable | safe refs; current fingerprints; changed bytes change/refuse | no observations rewriting Charter policy |
| kernel freshness pair | freshness evaluator supplies applicability truth | explicit null pair only when no input is freshness-qualified | basis covers every qualified source | `resolved_at_utc` cannot drive freshness |
| kernel global `dimensions` | resolver derives effective global posture | none for required dimensions | registered dimension; levels/floors/red lines/triggers/shortcuts/proofs obey authority | no per-scope posture level in v1 |
| kernel applicability/condition/recommendation refs | resolver explains applicability and advisory outputs | lists empty | typed refs; unresolved required condition prevents overclaim | scope refs do not create state coordinates; recommendation refs excluded from fingerprint |
| kernel timestamp and fingerprints | resolver records presentation time; Handbook derives semantic identity | none | explicit normalization above; same inputs/basis replay exactly | no ambient clock or presentation timestamp input |
| policy identity/hard-trigger pairs | policy author owns mechanics and pins exact hard triggers | hard-trigger list empty | exact schema/SemVer/ref/fingerprint; unique trigger refs | no ambient trigger registry |
| accumulated rule identity and input pairs | policy author owns one local repeated-signal rule | rule list empty; no per-rule defaults | unique ID; exact signal/evidence/freshness pairs | local rule is not an external trigger ref |
| accumulated dimension/scope/comparator/threshold/window/count | policy author owns deterministic bounded semantics | none per rule | registered global dimension; causal scope; allowlisted comparator; bounded numeric threshold/window/count | no unscoped, unbounded, or model-only inference |
| accumulated transition/guidance/rule fingerprint | policy author binds one proposal; Handbook derives rule identity | actions empty; other fields explicit | `from` matches kernel; `to` respects authority; full normalized fingerprint | no multi-dimension or per-scope state mutation |
| policy hysteresis/notification/extensions | policy owns lowering and delivery obligations | lowering disabled unless both window/cooldown set; recipients empty; acknowledgment false; escalation null; extensions empty | durations/refs valid; lowering cannot cross floors/red lines | adapters deliver but cannot change semantics |
| `policy_fingerprint` | Handbook derives exact policy identity | none | normalization stated above includes trigger/rule/notification closure | no changed policy behind same fingerprint |
| recommendation kernel/policy pairs | evaluator owns advisory proposal; sources remain authority | none | exact current pairs | recommendation is not enacted policy |
| recommendation `proposed_transition` and causal scope | evaluator copies exactly one trigger/rule-bound proposal | none | one global dimension; `from` matches kernel; `to` valid; causal scope matches source | no second transition, scoped level, or scoped authority path |
| recommendation trigger source | evaluator selects hard trigger or policy-local accumulated rule | mutually exclusive null fields as stated above | exact current trigger pair or exact policy rule fingerprint | no ambiguous trigger namespace |
| recommendation evidence/snapshot inputs | source records remain authority | evidence required unless bound definition supplies sufficient basis; snapshot list empty | exact fingerprints; freshness sufficient; unexplained drift not causal alone | no evidence-free/bare-ref authority |
| recommendation guidance/notification | selected trigger/rule and policy remain authority | actions empty; other fields explicit | values match source and policy exactly | no per-transition ambiguity because v1 has exactly one transition |
| recommendation eligibility/fingerprint | Handbook fixes advisory-only status and derives immutable identity | `recommendation_only` | normalized SHA-256 over full semantic closure except ID/fingerprint | no direct mutation or omitted mutable input |
| transition recommendation/kernel/target pairs and class | transition engine binds compare-and-write state | none | current fingerprints; class exactly `constitutional_root`; target satisfies capability bindings | no approved-override target or bare ref |
| transition `change` | canonical policy owner identifies one bound global-dimension mutation | none | exactly one `replace`; path matches constitutional dimension binding; expected/proposed match recommendation/kernel | no patch script, scoped path, or unrelated field |
| transition approval inputs/authorized actor | constitutional authority owns approval | none | exact approval pairs and required authority; hysteresis/floors/red lines hold | agent/evaluator cannot self-approve |
| transition reassessment | intake definition owns affected coverage; transition records proof | none | exact intake ref; non-empty mapped coverage IDs; current validation pairs pass | no whole-Charter or unrelated reopen |
| transition effective/result pairs/fingerprint | transition engine records atomic write and re-resolution | none after success | authority/kernel results replay; fingerprint covers full record/input fingerprints except itself | no mutable history or success without resulting-kernel proof |
| all `extensions` | declaring schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required semantics |

Recommendation and transition gates:

- every recommendation cites one current kernel/policy pair, one exact hard trigger or accumulated rule, and sufficient exact evidence inputs;
- hard triggers and accumulated rules remain distinct; unexplained snapshot drift cannot silently become causal evidence;
- a causal event requiring multiple dimensions produces separate recommendations with independent approvals and fingerprints;
- recommendations are advisory and never modify canonical Charter or override state;
- lowering requires configured sustained evidence and cooldown and cannot cross floors or red lines;
- notification delivery is adapter-owned and cannot change recipient, acknowledgment, approval, or escalation semantics;
- only an authorized compare-and-write `PostureTransition` revalidates mapped intake coverage, updates the canonical constitutional-root authority atomically, and re-resolves the kernel; override mutation is refused in v1;
- a failed, stale, unapproved, fingerprint-mismatched, scoped-path, or invalid transition leaves canonical authority unchanged and returns a typed refusal.

Required conformance scenarios for later implementation packets:

| Scenario | Required result |
|---|---|
| same semantic sources and same freshness basis resolve twice | identical input/kernel fingerprints and global dimensions |
| parent/repository profile changes behind same selected profile ref | old `resolved_profile_fingerprint` refuses; new one changes kernel input fingerprint |
| source/policy/trigger bytes change behind same ref | stale fingerprint refuses; new fingerprint changes dependent identities |
| same sources evaluated immediately before and after expiry | different basis/input fingerprints; classification/output changes honestly |
| hard trigger and normalized policy resolve twice | identical trigger/policy fingerprints |
| accumulated rule misses count/window/freshness threshold | no recommendation |
| accumulated rule meets exact bounded semantics | recommendation cites policy/rule fingerprints and exact evidence |
| one event warrants two dimension changes | two recommendations, never one multi-transition record |
| causal scope differs while global dimension state is the same | scope is retained as evidence metadata; `from` still reads the global dimension |
| recommendation notification differs from policy or guidance differs from trigger/rule | refusal |
| lowering lacks sustained window/cooldown or crosses floor/red line | refusal |
| transition path is scoped/outside binding, operation is not `replace`, or target is an override | refusal without canonical mutation |
| transition expected value/kernel/target/recommendation fingerprint is stale | compare-and-write refusal |
| actor/approval, mapped reassessment proof, or resulting-kernel fingerprint is missing/mismatched | invalid transition proof and no success claim |

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

## Development orchestration contracts

These contracts govern development of Handbook through this control pack. They do not define a Handbook product-runtime agent engine.

### Dispatch execution envelope

The only normative current machine example is
`handoffs/internal-dispatch-template.json`. It is validated against
`handoffs/internal-dispatch.v1.1.schema.json` by `validate_handoffs.py`; do not
maintain a second partial YAML shape that can drift from the schema. Every
current internal dispatch includes the complete identity and timing fields,
parent and source lineage, execution contract, ordered skill chain, explicit
phase/slice/packet authority, replayable subject manifest, active Resolution,
authority and repo-truth statements, bounded scope/tasks/gates/stops, and the
complete structured return contract.

`execution_target` is one of:

- `internal_subagent` — default for delegable implementation, documentation, proof, remediation, and review work; the active parent executes it immediately through built-in subagent tools;
- `top_level_resume` — used only when a genuine context/runtime or authorization boundary requires another top-level task;
- `human_interactive` — used only when the next action requires user judgment, interactive observation, approval, or another non-delegable human action.

Rules:

1. `internal_subagent` uses the built-in fresh `default` agent capability with isolated context; shell-launched Codex processes, background jobs, temporary-file reviewer transport, and filesystem polling do not satisfy it.
2. Every dispatch declares an ordered `required_skills` chain beginning with `using-agent-skills`; the resolved skill workflows are mandatory, not advisory labels.
3. Review agents are read-only and receive authority, diff/evidence, gates, and non-goals without implementation reasoning or prior reviewer conclusions.
4. Internal agents return structured results to the parent and never append the global handoff ledger.
5. The parent validates findings against live truth, remediates valid findings directly or through a fresh fix agent, reruns verification, and sends the resulting state to a different fresh reviewer.
6. The parent owns integration, control-pack truth, final proof, commit, and top-level closeout.

### Parent-owned delegated-run evidence

New top-level handoffs record each proof-relevant built-in delegation:

```json
{
  "run_id": "review-round-1",
  "dispatch_id": "20260714T120000Z--HCM-0-1--independent-review",
  "dispatch_ref": "docs/specs/handbook-contract-membrane/handoffs/dispatches/20260714T120000Z--HCM-0-1--independent-review.json",
  "dispatch_fingerprint": "sha256:...",
  "role": "review",
  "agent_id": "independent_review",
  "agent_type": "default",
  "fresh_context": true,
  "required_skills": ["using-agent-skills", "code-review-and-quality"],
  "subject_fingerprint": "sha256:...",
  "result_subject_fingerprint": "sha256:...",
  "review_round": 1,
  "predecessor_run_id": null,
  "remediation_for_run_ids": [],
  "final_status": "completed",
  "verdict": "clean",
  "finding_refs": [],
  "evidence_refs": ["docs/specs/handbook-contract-membrane/handoffs/dispatches/...json"]
}
```

Handoff schema v1.2 adds required `orchestration_id`, `source_handoff_ids`, `delegation_capability`, `delegated_runs`, typed `remediations`, `reviewed_state`, `stop_reason`, and `resume`. `stop_reason` is one of `completed`, `human_input`, `external_blocker`, `authority_boundary`, `context_boundary`, or `capability_unavailable`.

A v1.2 handoff with `status: completed` must use `stop_reason: completed`, contain a completed clean fresh review run bound to `reviewed_state.subject_fingerprint` and its replayable manifest, and require no top-level resume. Human, external, authority, context, and capability stop reasons map to explicit permitted status/resume targets; v1.2 cannot emit historical `review_required`. A `capability_unavailable` stop is `blocked`. Dispatch/result parent, slice, packet, role, skills, and subject fingerprints must match. Earlier records and schemas remain immutable historical evidence.

The subject manifest sorts repository-relative paths and encodes each entry as `path + NUL + lowercase SHA-256 + newline`; the aggregate is SHA-256 over the concatenated encoded entries. Dispatch validation always recomputes the aggregate from the stored entries. At execution, the parent and reviewer verify every entry against the live subject. A completed v1.2 closeout verifies the final clean manifest against `reviewed_state.baseline_head`, the primary reviewed commit, while ledger parity is validated separately against the post-closeout record set. This preserves exact review binding even when the mechanical second commit adds the parent record and rebuilds `ledger.jsonl`. Earlier review manifests remain immutable identities of superseded pre-remediation subjects and are not incorrectly compared with the later repaired tree.

`delegation_capability.status: unavailable` and `stop_reason: capability_unavailable` are bidirectionally coupled. Unavailable mandatory delegation cannot be hidden under a human, external, authority, or context stop reason.

Every findings review in a completed closeout has a typed remediation record. Its owner is either `parent_orchestrator` with durable evidence or a completed delegated `remediation` run. The remediation names the findings run, result fingerprint, and different-fresh completed re-review run. That re-review may itself report findings; if it does, it receives its own typed remediation and another different fresh review. The last completed review must be clean. Failed or wrong-role work cannot satisfy remediation lineage.

### Review choreography

```text
implement or document
  -> verify
  -> fresh read-only reviewer
      -> clean: proof wall and closeout
      -> actionable findings:
           validate findings
           -> parent repair or fresh remediation agent
           -> verify
           -> different fresh reviewer
           -> repeat until clean or genuinely blocked
```

The orchestrator may not self-approve. A dispatch artifact proves a bounded job was specified; only captured built-in agent identity/status plus reconciled results prove that the job was executed.

## HCM-0.4 crate ownership and dependency contract

The target owner matrix is exact:

| Crate/surface | Owns | May depend on | Must not own or depend on |
|---|---|---|---|
| `handbook-engine` | canonical semantic records; profile/kind/instance/intake/posture/Resolution/Snapshot types, validation, trusted repository access, normalization, fingerprints, deltas, and deterministic transforms | no other Handbook workspace crate; standard/third-party libraries | flow/pipeline/SDK/CLI/Tauri/Substrate; process execution; product wording |
| `handbook-flow` | request-scoped selection, context assembly, Resolution application, Projection execution, omission accounting, packet/grounding results | `handbook-engine` | canonical mutation, CLI/Tauri concerns, Substrate orchestration |
| `handbook-contracts` | contract identity/lifecycle, claims/invariants, normalized evidence, verdicts, hard gates, and protocol-neutral dock types/evaluation | `handbook-engine` exact semantic types | process-dock execution, CLI/Tauri/Substrate wording or orchestration; `handbook-sdk` |
| `handbook-pipeline` | declarative catalog/route/compile/capture/handoff/state sequencing and later sequencing of contract use cases | `handbook-engine`; `handbook-contracts` only when Phase 5 execution lands | ordinary-consumer facade, CLI rendering, Substrate runtime authority |
| `handbook-sdk` | typed ordinary-consumer use cases, request/result/outcome DTOs, capability/schema reporting, and composition over owners | engine, flow, contracts, pipeline | canonical semantic truth, transport-specific wording, generic public `Value` dispatch, process spawning |
| `handbook-cli` | command grammar/help, argument and cwd/repo discovery, SDK invocation, human rendering, JSON/stdout/stderr discipline, exit mapping | `handbook-sdk` plus executable-shell libraries | domain decisions, owner-crate composition, contract/projection/evidence evaluation |
| Tauri adapter | stable Tauri command mapping, async/scheduling boundary, SDK invocation, DTO serialization | `handbook-sdk`, Tauri runtime | CLI subprocesses in normal operation, frontend domain authority, transport-specific DTO forks |
| Substrate process bridge | exact binary invocation and exact JSON protocol validation inside an isolated replaceable Substrate adapter | published Handbook binary only | human-output parsing, Handbook domain reimplementation, permanent integration claims |
| Direct Substrate integration | Substrate-owned orchestration and product rendering over exact published SDK/owner APIs | exact crates.io versions of `handbook-sdk` and/or advanced owner crates | sibling/path/patch fallback, CLI process dependency in the proved seam, competing contract authority |

The workspace dependency graph is acyclic. Owner crates never depend on `handbook-sdk`; no Handbook crate depends on Substrate; `handbook-contracts` never depends on pipeline; transports never become semantic owners. `handbook-compiler` is not in the target graph. HCM-4.1 retires it after moving ordinary composition to SDK, executable-shell behavior to CLI, and already-owned behavior to its owner crate.

HCM-0.4 selects `handbook-contracts` as the contract-membrane owner but does not preempt HCM-0.5 semantics. HCM-0.5 freezes the exact contract/dock types and appends their ordinary operation definitions. Process-dock implementation crates stay separable and are named only in their implementation packet.

## SDK ordinary-use-case contract

The Rust SDK exposes one typed method per ordinary operation. It may share internal orchestration helpers, but the public surface is not a stringly typed dispatcher and does not return arbitrary JSON values. Every method binds one exact operation definition, typed request/result schemas, structured expected outcomes, and deterministic transport serialization.

The frozen inventory below is data-oriented: custom kind IDs, instance IDs, vocabulary, profile selections, Resolution definitions, and pipeline IDs are request fields. They never create operation IDs, Rust methods, CLI commands, or Tauri commands.

All listed operation IDs start at operation version `1.0.0`; an implemented definition adds exact schema/capability refs and a recomputable definition fingerprint before it becomes discoverable.

| Family | Stable operation ID | Semantic owner/composition | Effect |
|---|---|---|---|
| Capability | `capabilities.describe` | SDK registry over all implemented owners | read-only bootstrap catalog |
| Profile | `profile.list` | engine | read-only exact profile catalog |
| Profile | `profile.resolve` | engine | read-only |
| Schema registry | `schema.list` | engine | read-only exact schema catalog |
| Schema registry | `schema.read` | engine | read-only exact schema document |
| Vocabulary | `vocabulary.read` | engine | read-only exact vocabulary definition |
| Resolution | `resolution.stack.read` | engine | read-only exact Context Resolution stack definition |
| Projection | `projection.definition.read` | flow over engine definition registry | read-only exact Projection definition |
| Artifact registry | `artifact.kind.list` | engine | read-only |
| Artifact registry | `artifact.instance.list` | engine | read-only |
| Artifact | `artifact.read` | engine | read-only |
| Artifact | `artifact.validate` | engine | read-only |
| Artifact | `artifact.render` | engine | read-only fixed renderer-derived view; capitalized Projections use `projection.create` |
| Intake | `intake.definition.read` | engine | read-only |
| Intake | `intake.coverage.evaluate` | engine | read-only |
| Intake | `intake.record.append` | engine through SDK repository transaction | append-only |
| Governed records | `record.list` | engine | read-only snapshot-bound typed catalog with family/state selectors |
| Governed records | `record.read` | engine | read-only exact typed record |
| Candidate | `artifact.candidate.validate` | engine | read-only |
| Candidate | `artifact.candidate.append` | engine through SDK repository transaction | append-only immutable candidate |
| Approval | `artifact.approval.append` | engine through SDK repository transaction | append-only immutable approval |
| Candidate | `artifact.candidate.promote` | engine through SDK repository transaction | compare-and-write canonical mutation |
| Posture | `posture.resolve` | engine | read-only |
| Posture | `posture.recommendation.evaluate` | engine | read-only typed recommendation-or-no-recommendation result |
| Posture | `posture.recommendation.append` | engine through SDK repository transaction | append-only immutable recommendation |
| Posture | `posture.recommendation.acknowledge` | engine through SDK repository transaction | append-only |
| Posture | `posture.transition.apply` | engine through SDK repository transaction | authorized compare-and-write mutation |
| Projection | `projection.create` | flow over engine definitions | read-only derived result |
| Resolution | `resolution.escalation.request.append` | engine through SDK repository transaction | append-only immutable request |
| Resolution | `resolution.escalation.disposition.append` | engine through SDK repository transaction | append-only terminal disposition |
| Memory | `memory.promotion.request.append` | engine through SDK repository transaction | append-only immutable request |
| Memory | `memory.promotion.disposition.append` | engine through SDK repository transaction | compare-and-write terminal disposition and, only for `applied`, semantic-memory result |
| Snapshot | `snapshot.capture` | SDK orchestration over engine normalization and repository readers | append-only immutable capture |
| Snapshot | `snapshot.read` | engine storage boundary through SDK | read-only |
| Snapshot | `snapshot.delta` | engine | read-only derived result |
| Snapshot | `snapshot.project` | flow over engine snapshot semantics | read-only derived result |
| Snapshot | `snapshot.verify_current` | SDK orchestration over engine comparison | read-only |
| Snapshot | `snapshot.resolve_applicable` | SDK orchestration over engine policy/profile/Resolution comparison | read-only exact prior snapshot or typed no-applicable result |
| Repository | `repository.setup.plan` | SDK composition over engine/profile owners | read-only plan |
| Repository | `repository.setup.apply` | SDK repository transaction | compare-and-write mutation |
| Repository | `repository.doctor` | SDK composition over owners | read-only |
| Flow | `flow.resolve` | flow | read-only |
| Pipeline catalog | `pipeline.catalog.list` | pipeline | read-only |
| Pipeline catalog | `pipeline.catalog.read` | pipeline | read-only |
| Pipeline route | `pipeline.route.resolve` | pipeline | read-only |
| Pipeline compile | `pipeline.compile` | pipeline | read-only in-memory result/artifact production; no repository or content-store write |
| Pipeline capture | `pipeline.capture.plan` | pipeline | read-only plan |
| Pipeline capture | `pipeline.capture.apply` | pipeline through SDK repository transaction | compare-and-write mutation |
| Pipeline handoff | `pipeline.handoff.emit` | pipeline through SDK repository transaction | append-only |
| Pipeline state | `pipeline.state.apply` | pipeline through SDK repository transaction | compare-and-write mutation |

`contract.*`, `dock.*`, evidence, verdict, and gate operation IDs are intentionally absent until HCM-0.5 freezes their semantics. Adding them later extends this catalog; it cannot change the owner/DTO/transport rules in this section. The SDK may expose advanced owner APIs through direct crate imports, but only catalogued ordinary operations owe CLI JSON and Tauri parity.

The intake lifecycle is mechanically reachable through ordinary operations: append an immutable intake record, validate a candidate without writing, append that exact candidate, append zero or more immutable approval records over its exact fingerprint, then compare-and-write promotion against the current target. The promotion request cites the intake-record, candidate, and approval ref/fingerprint pairs and the current target fingerprint; no step mutates or replaces an earlier record. Posture evaluation is pure and always returns one typed `recommendation` or `no_recommendation` variant; only `posture.recommendation.append` persists an exact evaluated recommendation. Resolution escalation and memory promotion similarly expose separate request/disposition operations. Escalation dispositions are append-only terminal records. Memory-promotion dispositions use compare-and-write because an `applied` disposition atomically appends both the unique terminal disposition and the newly derived semantic-memory record; refusal or stale outcomes append only the terminal disposition. The HCM-0.2 and HCM-0.3 lineage, uniqueness, authority, nullability, and byte-identical replay rules remain authoritative.

`record.list` and `record.read` are generic but not untyped. `record_family` is a closed discriminant over intake record, artifact candidate, artifact approval, artifact promotion, posture recommendation/acknowledgment, posture transition, Resolution-escalation request/disposition, memory-promotion request/disposition, and semantic-memory record. Each variant selects one exact request/result schema and Rust enum case. `record.list` uses the snapshot-bound catalog-page contract and an allowlisted family-specific state filter; `pending` is valid only for a request family and means no exact terminal disposition exists in that immutable catalog snapshot. `record.read` requires the expected family plus exact record ref/fingerprint and returns that variant only. Unknown families, invalid filters, family/ref mismatch, and stale fingerprints refuse. These operations let a restarted client or separate authority actor recover exact promotion/transition/memory lineage without direct repository reads.

`snapshot.resolve_applicable` takes exact repository/profile/Resolution/capture-policy refs/fingerprints plus the required boundary/horizon selector. It deterministically returns `applicable` with one exact retained snapshot pair or `none` with typed reasons; incompatible, unstable, redacted-beyond-selection, ambiguous, or retention-expired candidates do not silently win. Ordering is policy definition first, then greatest admissible boundary sequence, then exact record fingerprint as the deterministic tie check; multiple distinct candidates at the winning identity refuse. It never captures implicitly or widens Resolution.

## Operation definition contract

```yaml
schema_id: handbook.operation-definition
schema_version: "1.0.0"
operation_id: artifact.validate
operation_version: 1.0.0
owner_crate: handbook-engine
request_schema:
  ref: handbook.operation.artifact-validate-request@1.0.0
  fingerprint: sha256:...
result_schema:
  ref: handbook.operation.artifact-validate-result@1.0.0
  fingerprint: sha256:...
outcome_schemas:
  blocker:
    ref: handbook.operation-blocker@1.0.0
    fingerprint: sha256:...
  refusal:
    ref: handbook.operation-refusal@1.0.0
    fingerprint: sha256:...
  error:
    ref: handbook.operation-error@1.0.0
    fingerprint: sha256:...
mutability: read_only
idempotency: safe
required_capabilities: []
transport_targets: [rust_sdk, cli_json, tauri]
authority_effect: none
write_set: []
idempotency_retention: null
deprecation: null
definition_fingerprint: sha256:...
```

The example illustrates shape, not an implemented API or runtime proof.

| Field | Authority/default | Validation/refusal rule |
|---|---|---|
| `schema_id` / `schema_version` | operation-definition schema owns exact values | only `handbook.operation-definition` / `1.0.0` routes here; the exact schema ref is derived as `handbook.operation-definition@1.0.0` |
| `operation_id` | HCM catalog/approved extension owns stable machine identity | lowercase dot-separated snake-case segments matching `^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$`; exact match; never derived from CLI/Tauri/vocabulary/custom IDs |
| `operation_version` | operation owner publishes SemVer | exact version only; no ranges, latest, or ambient fallback |
| `owner_crate` | owner matrix above | exact target owner; SDK composition does not change semantic ownership |
| request/result/outcome schema pairs | operation owner pins exact ref and fingerprint | every ref resolves locally/published with matching recomputable fingerprint; no bare ref |
| `mutability` | operation definition | one of `read_only`, `append_only`, `compare_and_write`; implemented behavior must match |
| `idempotency` | operation definition | `safe`, `idempotency_key_required`, or `compare_and_write_required`; must match the legal-combination matrix below |
| `required_capabilities` | exact capability refs/fingerprints | all resolve and are reported by `capabilities.describe`; missing/mismatch refuses before body access |
| `transport_targets` | ordinary-use-case catalog | non-empty subset of `rust_sdk`, `cli_json`, `tauri`; every catalogued operation eventually targets all three unless an approved advanced-owner exception removes it from the ordinary catalog |
| `authority_effect` | semantic owner | one of `none`, `append_record`, `compare_and_write`; it describes write mechanics, not the authority of affected records |
| `write_set` | semantic owner | closed list of `{record_kind, authority_class, condition, cardinality, atomic_group}`; exact operation grouping and conditional writes are declared below, and read-only operations require `[]` |
| `idempotency_retention` | operation owner | null only for read-only operations; mutations declare a positive duration supported by the durable replay ledger |
| `deprecation` | operation owner | null or `{replacement: ExactBinding, announced_in_api_version, removal_api_major, migration: ArtifactRef}`; replacement and migration fingerprints are mandatory and no silent removal or prose-only migration is valid |
| `definition_fingerprint` | derived | normalized SHA-256 over every semantic field except itself; any field change changes the fingerprint |

Definition graphs are acyclic. A response/request schema may reference shared DTO schemas, but no DTO schema may reference an operation definition or transport adapter. Implemented capability discovery reports only definitions whose exact request/result/outcome schemas are present and whose transport status is honest.

HCM-0.4 public transport and generated JSON Schema identities use one full-SemVer domain: `schema_id` is the stable identity, `schema_version` is full SemVer, and the exact schema ref is mechanically `schema_id + "@" + schema_version`. Operation refs are mechanically `operation_id + "@" + operation_version`. API compatibility versions are also full SemVer but are not schema or operation refs. The already-frozen HCM-0.2/HCM-0.3 records whose payload contains `schema_version: "1.0"` use that legacy field solely as a two-component **record-routing tag**; it is not a public JSON Schema identity, is never converted or aliased to `@1.0.0`, and cannot appear as a `SchemaManifestEntry`. When such a record is carried in an operation, its public wire schema is the distinct full-SemVer operation result schema cited by the operation definition and response manifest; kind/definition-specific exact bindings inside the record remain independently fingerprinted. A deprecation replacement is an exact operation ref/definition-fingerprint pair; its migration artifact is an exact bounded artifact ref/fingerprint pair. Clients refuse a public schema ref whose declared identity/version does not derive the cited ref, any fingerprint mismatch, any attempt to treat a record-routing tag as a schema ref, and a deprecated operation after its declared removal major.

The legal mutation combinations are closed:

| Mutability | Idempotency | Authority effect | Allowed write-set classes | Required request fields | Replay behavior |
|---|---|---|---|---|---|
| `read_only` | `safe` | `none` | empty write set | no idempotency key or mutation basis | repeat evaluation is side-effect free |
| `append_only` | `idempotency_key_required` | `append_record` | `semantic_record`, `observation_evidence`, or `operational_state` | `idempotency_key` | one immutable append; exact retry replays the semantic result |
| `compare_and_write` | `compare_and_write_required` | `compare_and_write` | `canonical_truth`, `semantic_memory`, `semantic_record`, `observation_evidence`, or `operational_state`; multiple classes may share one exact atomic group | `idempotency_key` plus operation-defined exact expected fingerprints | one successful atomic write set; exact retry replays even after the basis advanced |

No other combination is valid. Each write-set item uses authority class `canonical_truth`, `semantic_memory`, `semantic_record`, `observation_evidence`, or `operational_state`; condition is `always` or one exact operation-result discriminant; cardinality is `exactly_one`; and all items realized by one operation share one atomic group. Exact mutation classification is:

| Authority class | Write-set item and condition |
|---|---|
| `canonical_truth` | canonical artifact for `artifact.candidate.promote` always; canonical constitutional-root artifact update for `posture.transition.apply` always |
| `semantic_memory` | semantic-memory record for `memory.promotion.disposition.append` only when `data.disposition=applied`; it can never create artifact, contract, posture, or gate authority |
| `semantic_record` | one always-written record for `intake.record.append`, `artifact.candidate.append`, `artifact.approval.append`, `posture.recommendation.append`, both Resolution-escalation append operations, and `memory.promotion.request.append`; successful `artifact.candidate.promote` also writes its immutable `ArtifactPromotionRecord` in the same atomic group as the canonical artifact; successful `posture.transition.apply` also writes its immutable `PostureTransition` in the same atomic group as the constitutional-root update; `memory.promotion.disposition.append` always writes its terminal disposition here in the same atomic group as its conditional semantic-memory item |
| `observation_evidence` | snapshot for `snapshot.capture` always; capture record for `pipeline.capture.apply` always |
| `operational_state` | acknowledgment for `posture.recommendation.acknowledge` always; setup state for `repository.setup.apply` always; handoff for `pipeline.handoff.emit` always; pipeline state for `pipeline.state.apply` always |

Any operation not named in this table is read-only with an empty write set. Repository setup and pipeline state do not become canonical semantic authority merely because they use compare-and-write. `artifact.candidate.promote` and `posture.transition.apply` each realize exactly two receipts on success; refusal before commit realizes none. The posture result may return a newly derived `ProjectPostureKernel` and fingerprint, but the kernel is not written as canonical authority; its inputs are the updated constitutional-root artifact plus exact semantic inputs. For memory-promotion `refused`/`stale`, only the always-written semantic-record disposition is realized; for `applied`, both exact items are committed atomically. Every operation-definition fixture and result fixture asserts the realized write set, atomic group, exact record identities, and absence of undeclared writes.

An idempotency key is scoped to the exact repository-identity fingerprint, negotiated API version/bootstrap-descriptor pair, operation ref, and operation-definition fingerprint and is durably bound to the request fingerprint. The outcome matrix is total:

| Ledger state | Same request fingerprint | Different request fingerprint |
|---|---|---|
| no consumed key | establish immediately before mutation execution | establish for this request immediately before mutation execution |
| retained result | replay the same semantic result/record refs with no second write | refuse `idempotency.key_conflict` |
| consumed-key tombstone after result retention | refuse `idempotency.expired` | refuse `idempotency.key_conflict` |

Concurrent same-fingerprint requests serialize so exactly one establishes/wins and the rest replay that result. Concurrent different-fingerprint requests serialize one establishment and every nonmatching request conflicts. A compare-and-write retry with the original key replays a prior success before rechecking an advanced basis; a new key with a stale basis refuses unless the exact operation definition declares stale comparison as an authorized terminal-record result. At result-retention expiry, the ledger compacts the result to a non-expiring consumed-key tombstone containing the key scope, request fingerprint, and original result fingerprint but no sensitive result body. Tombstones may be removed only when the repository-identity namespace is irreversibly retired and cannot be reused. Transport correlation IDs may differ on replay and therefore do not enter the semantic replay binding.

`original_result_fingerprint` is the lowercase `sha256:<64-hex>` of the RFC 8785 canonical object containing exactly `{schema_id, schema_version, request_fingerprint, operation_ref, operation_definition_fingerprint, status, data, blockers, refusals, errors, diagnostics, next_actions, artifact_refs, write_receipts, provenance, schema_manifest}` from the committed ordinary response. It excludes exactly `request_id`, the complete `idempotency` member, and `response_fingerprint`. The domain commit atomically stores this computed identity with the retained committed-result closure and receipts even though the first response's `idempotency.original_result_fingerprint` is null; every replay exposes the stored value there, and tombstone compaction preserves the same value. Thus a different replay correlation ID or `replayed` flag changes the outer response fingerprint but not original-result identity, while any change to semantic data, outcome arrays, provenance, schemas, or realized receipts fails recomputation.

`memory.promotion.disposition.append` is the sole initial stale-terminal exception. After request identity, no-prior-terminal-disposition, disposition schema, and requested-actor authority validate, the SDK establishes the idempotency key before evaluating the target compare. An authorized `refused` decision returns `status=ok`, `data.disposition=refused`, and the terminal semantic-record receipt. An authorized `applied` decision whose expected target matches writes the terminal disposition plus semantic memory atomically. The same decision whose target basis is stale returns `status=ok`, `data.disposition=stale`, and writes the terminal semantic-record disposition, leaving no request pending and no semantic-memory receipt. An unauthorized actor, duplicate terminal disposition, malformed decision, or stale request fingerprint is an ordinary pre-establishment refusal and writes nothing. Same-key stale retry replays the recorded stale result; same key with a different request fingerprint conflicts.

## Capability bootstrap and catalog discovery

Capability discovery has a non-circular root. API major `M` publishes one immutable `handbook.bootstrap-descriptor@M.0.0` alongside the SDK, CLI, generated schemas, Tauri adapter, and bridge bundle; therefore API major 1 uses `handbook.bootstrap-descriptor@1.0.0` and major 2 uses `@2.0.0`. The separately versioned descriptor schema is an exact full-SemVer binding inside the descriptor. Its fingerprint closure contains the exact same-major `capabilities.describe` definition, bootstrap request/response/refusal schemas, generic response schema, and supported API major. The descriptor is compiled into typed clients and separately checksum-pinned by a process bridge; it is not discovered from the endpoint it validates.

A cold client sends only the descriptor-pinned bootstrap request. A matching runtime returns exact `CatalogRoot` values for implemented operation definitions, schemas, and profiles plus owner-crate versions and the first bounded capability page. The client then uses paged `capabilities.describe`, `schema.list`/`schema.read`, and `profile.list` to retrieve closed exact catalogs/documents; `vocabulary.read`, `resolution.stack.read`, and `projection.definition.read` retrieve the exact definitions referenced by a resolved profile, so the client never reads repository files or infers shipped defaults.

Every catalog/list request carries `page_size` and an exact-or-null cursor. `CatalogRoot` contains catalog ref/fingerprint, total entry count, entry schema binding, and canonical sort definition. A page repeats that root, contains a bounded duplicate-free contiguous entry sequence, and returns a null terminal cursor or a cursor cryptographically bound to the catalog fingerprint and next sort key. The first request atomically materializes an immutable content-addressed catalog snapshot; later pages read that same snapshot despite concurrent registry changes. If its declared retention expires before traversal finishes, the next page is `blocked` with a typed restart condition rather than mixing versions. Page-size overflow, cursor/catalog mismatch, gaps, duplicates, reordered entries, and total-count mismatch refuse. Bootstrap/catalog conformance proves empty, single-page, multi-page, concurrent-change, expiry/restart, and oversized cases.

A stale descriptor fingerprint or unsupported API major returns the descriptor-defined typed bootstrap refusal without deserializing an ordinary operation body. After a valid bootstrap, an unknown/malformed ordinary operation ref, incompatible requested API context, or invalid/stale definition fingerprint returns the descriptor-bound admission-refusal envelope below rather than fabricating trusted ordinary operation fields. An unknown bootstrap operation, invalid descriptor checksum, cross-major descriptor substitution, or tampered/unparseable bootstrap response is an adapter/bootstrap failure and yields no fabricated Handbook domain result. The bootstrap descriptor may change only under a new exact descriptor ref/fingerprint, and a breaking bootstrap change requires a new API major.

### Preselection admission-refusal envelope

This envelope is separate from `handbook.operation-response` and is valid only before an exact operation definition is accepted:

```json
{
  "schema_id": "handbook.operation-admission-refusal",
  "schema_version": "1.0.0",
  "request_id": "req_example",
  "request_fingerprint": "sha256:...",
  "operation_ref": null,
  "operation_definition_fingerprint": null,
  "bootstrap_descriptor": {
    "ref": "handbook.bootstrap-descriptor@1.0.0",
    "fingerprint": "sha256:..."
  },
  "admission": {
    "state": "not_selected",
    "stage": "operation_selection",
    "reason": "operation_unknown",
    "request_object_digest": "sha256:...",
    "selection_evidence": {
      "requested_operation_ref": "unknown.operation@1.0.0"
    },
    "terminal_problem": {
      "problem_id": "problem_admission",
      "problem_fingerprint": "sha256:..."
    }
  },
  "status": "refused",
  "data": null,
  "blockers": [],
  "refusals": [
    {
      "problem_id": "problem_admission",
      "code": "admission.operation_unknown",
      "category": "compatibility",
      "subject": {
        "ref": "handbook.bootstrap-descriptor@1.0.0",
        "fingerprint": "sha256:..."
      },
      "rule": {
        "ref": "handbook.admission-rule.operation-unknown@1.0.0",
        "fingerprint": "sha256:..."
      },
      "details_schema": {
        "ref": "handbook.admission-details.operation-unknown@1.0.0",
        "fingerprint": "sha256:..."
      },
      "details": {
        "request_object_digest": "sha256:...",
        "stage": "operation_selection",
        "reason": "operation_unknown",
        "selection_evidence": {
          "requested_operation_ref": "unknown.operation@1.0.0"
        }
      },
      "details_fingerprint": "sha256:...",
      "evidence": [],
      "retry": "after_request_change",
      "recheck": null,
      "correlation_ref": null,
      "problem_fingerprint": "sha256:..."
    }
  ],
  "errors": [],
  "schema_manifest": [
    {
      "schema": {"ref": "handbook.operation-admission-refusal@1.0.0", "fingerprint": "sha256:..."},
      "draft": "2020-12",
      "byte_length": 2048,
      "media_type": "application/schema+json"
    },
    {
      "schema": {"ref": "handbook.operation-refusal@1.0.0", "fingerprint": "sha256:..."},
      "draft": "2020-12",
      "byte_length": 1024,
      "media_type": "application/schema+json"
    },
    {
      "schema": {"ref": "handbook.admission-details.operation-unknown@1.0.0", "fingerprint": "sha256:..."},
      "draft": "2020-12",
      "byte_length": 512,
      "media_type": "application/schema+json"
    }
  ],
  "response_fingerprint": "sha256:..."
}
```

The `admission` member is a closed reason-discriminated union. Every variant contains exactly `state`, `stage`, `reason`, `request_object_digest`, `selection_evidence`, and `terminal_problem`; `state` is always `not_selected`. To compute `request_object_digest`, copy the complete bounded parseable request object, delete exactly the top-level `request_id` correlation member and top-level client-supplied `request_fingerprint` derived member whether their values are absent, null, valid, or invalid, RFC 8785-normalize the remaining object, then SHA-256 those bytes. No other field is deleted. The digest therefore preserves compound-invalid semantic identity without exposing fields that admission never evaluated, while correlation-only or client-supplied-fingerprint-only changes cannot alter it. `selection_evidence` contains only the winning field evidence listed below; later-selection fields are omitted, not set to null or opportunistically normalized.

The admission envelope's `request_id` has the closed output type `string | null`. A supplied value is valid only when it is a JSON string matching `^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$`; that exact string is echoed. An absent member, explicit null, non-string, overlong string, or string outside the allowlist emits null and is never echoed or included in any semantic fingerprint. Correlation validation does not outrank or replace the admission reason. For otherwise identical input, two different valid IDs produce identical object/request/Problem semantic fingerprints but different echoed IDs and outer response fingerprints; absent, null, and every invalid ID all emit null and produce identical complete envelopes/fingerprints; changing only client-supplied `request_fingerprint` changes no emitted field or fingerprint; combined changes follow only the emitted valid-ID-versus-null distinction.

The reason/evidence variants are:

| Stage | Reason | Exact `selection_evidence` shape | Meaning |
|---|---|---|---|
| `operation_selection` | `operation_ref_missing` | `{ "operation_ref": null }` | the client omitted the operation ref |
| `operation_selection` | `operation_ref_malformed` | `{ "attempted_operation_ref_digest": "sha256:..." }` | the supplied value cannot be normalized as an operation ref; digest ownership is the attempted operation-ref value only |
| `operation_selection` | `operation_unknown` | `{ "requested_operation_ref": "unknown.operation@1.0.0" }` | the normalized ref is absent from the descriptor-bound operation catalog |
| `api_negotiation` | `api_context_missing` | `{ "requested_operation_ref": "...", "missing_api_context_fields": ["client.api_version", ...] }` | at least one required API-context leaf is absent or null; the array is non-empty, unique, and sorted in the fixed field order below |
| `api_negotiation` | `api_context_malformed` | `{ "requested_operation_ref": "...", "malformed_api_context_field": "client.api_version", "attempted_api_context_digest": "sha256:..." }` | the first present non-null API-context parent or leaf fails its exact type/syntax rule; the path and digest bind that one winning raw value |
| `api_negotiation` | `api_incompatible` | `{ "requested_operation_ref": "...", "requested_api_context": { "client_api_version": "1.0.0", "negotiated_api_version": "1.0.0", "bootstrap_descriptor": { "ref": "handbook.bootstrap-descriptor@1.0.0", "fingerprint": "sha256:..." } } }` | the concrete normalized context is internally inconsistent, unsupported, or differs from the exact outer bootstrap-descriptor binding |
| `definition_validation` | `definition_pin_missing` | `{ "requested_operation_ref": "...", "requested_api_context": { "client_api_version": "1.0.0", "negotiated_api_version": "1.0.0", "bootstrap_descriptor": { "ref": "...", "fingerprint": "sha256:..." } }, "requested_definition_fingerprint": null }` | the client omitted or set null the `operation_definition_fingerprint` pin |
| `definition_validation` | `definition_fingerprint_malformed` | `{ "requested_operation_ref": "...", "requested_api_context": { "client_api_version": "1.0.0", "negotiated_api_version": "1.0.0", "bootstrap_descriptor": { "ref": "...", "fingerprint": "sha256:..." } }, "attempted_definition_fingerprint_digest": "sha256:..." }` | the supplied definition fingerprint cannot be normalized; its digest owns only that supplied field value |
| `definition_validation` | `definition_registry_missing` | `{ "requested_operation_ref": "...", "requested_api_context": { "client_api_version": "1.0.0", "negotiated_api_version": "1.0.0", "bootstrap_descriptor": { "ref": "...", "fingerprint": "sha256:..." } }, "requested_definition_fingerprint": "sha256:..." }` | the client supplied a valid exact pin but the runtime registry does not contain that definition |
| `definition_validation` | `definition_mismatch` | `{ "requested_operation_ref": "...", "requested_api_context": { "client_api_version": "1.0.0", "negotiated_api_version": "1.0.0", "bootstrap_descriptor": { "ref": "...", "fingerprint": "sha256:..." } }, "requested_definition_fingerprint": "sha256:..." }` | the valid registry-resolvable pin does not bind the selected operation/API pair |
| `definition_validation` | `definition_stale` | `{ "requested_operation_ref": "...", "requested_api_context": { "client_api_version": "1.0.0", "negotiated_api_version": "1.0.0", "bootstrap_descriptor": { "ref": "...", "fingerprint": "sha256:..." } }, "requested_definition_fingerprint": "sha256:..." }` | the exact binding is known but no longer admitted by the selected API context |

The API-context selector is not an inferred object. It reads exactly these request paths in this order: `client`, `client.api_version`, `negotiated_api_context`, `negotiated_api_context.api_version`, `negotiated_api_context.bootstrap_descriptor`, `negotiated_api_context.bootstrap_descriptor.ref`, and `negotiated_api_context.bootstrap_descriptor.fingerprint`. The three parent paths must be objects. The four leaves must be non-null strings with full-SemVer, full-SemVer, exact descriptor-ref, and SHA-256 fingerprint syntax respectively. An absent or null parent contributes all of its required descendant leaves to `missing_api_context_fields`; an absent or null leaf contributes itself. A present non-object parent or present non-null invalid leaf is malformed, and only the first invalid path in the fixed order wins. Once valid, the four leaves normalize to the exact `requested_api_context` DTO shown in the table. Unequal client and negotiated API versions, an API major/version unsupported by the descriptor, or a normalized descriptor pair unequal to the envelope's outer `bootstrap_descriptor` is `api_incompatible`. Thus absent, explicit null, partial, malformed, internally mismatched, descriptor-mismatched, and unsupported inputs each have one deterministic classification.

Every attempted-value digest is `sha256(RFC8785({"field_path": <exact request path>, "value": <raw JSON value>}))`. For `operation_ref_malformed` the path is `operation_ref`; for `definition_fingerprint_malformed` it is `operation_definition_fingerprint`; for `api_context_malformed` it is the exact `malformed_api_context_field`. This wrapper, including the raw JSON type, is the complete digest input; bytes outside the parsed value and any later field are excluded.

The admission algorithm applies those variants in this total order: operation-ref presence, syntax, then existence; API-context presence, syntax, then compatibility; definition-pin presence, syntax, registry existence, selected-operation/API match, then currentness. Compound-invalid input emits only the first winning variant and never includes evidence for a later step. `definition_pin_missing` is exclusively an absent or null client `operation_definition_fingerprint`; `definition_registry_missing` is exclusively a valid client pin absent from the runtime registry.

Every admission refusal has exactly one descriptor-owned stage-details schema selected by `(stage, reason)`. The mapping is injective and mechanical: Problem `code` is `admission.<reason>`; rule ref is `handbook.admission-rule.<reason-with-underscores-replaced-by-hyphens>@1.0.0`; details-schema ref is `handbook.admission-details.<reason-with-underscores-replaced-by-hyphens>@1.0.0`; and the descriptor supplies each exact fingerprint. Its sole Problem has the exact bootstrap descriptor as subject and `details` equal byte-for-byte in canonical form to `{ "request_object_digest": admission.request_object_digest, "stage": admission.stage, "reason": admission.reason, "selection_evidence": admission.selection_evidence }`; its `details_fingerprint` and `problem_fingerprint` bind that exact object plus the injective code/rule/details-schema bindings. Registry-missing, mismatch, and stale therefore cannot alias even when their requested values match. The same Problem matches `admission.terminal_problem`; blockers/errors remain empty. The schema manifest closes over only the admission-refusal, Problem, and selected stage-details schemas. There is no operation result schema, operation-specific Problem schema, ordinary `idempotency`, or write receipt.

Admission evaluation first requires bounded input bytes, UTF-8, rejection of duplicate object-member names at every nesting level by the tokenizing parser, and one parseable JSON object. Duplicate names are rejected before object construction, admission selection, or fingerprinting; they are an adapter/bootstrap failure with no Handbook response, including duplicates of excluded correlation/derived members. Failure of framing, size, UTF-8, duplicate-name uniqueness, or JSON parsing therefore occurs before request fingerprinting and yields no fabricated Handbook result. Direct SDK structured requests cannot represent duplicates; their conformance fixture proves such a state is unconstructible. Every byte-oriented SDK dynamic-JSON, CLI JSON, Tauri, and bridge entry point uses duplicate-rejecting parsing and proves identical no-response behavior for top-level and nested duplicates.

For a parseable unique-member object, the non-null server-computed `request_fingerprint` covers the exact descriptor binding, `request_object_digest`, and the selected `(stage, reason, selection_evidence)` tuple. It excludes the top-level `request_id` and any top-level client-supplied `request_fingerprint` transitively because both were deleted before `request_object_digest`; the envelope always emits the recomputed value rather than echoing the supplied value. `response_fingerprint` covers the complete emitted envelope except itself, so a different valid echoed `request_id` may change only that outer response fingerprint. Unknown semantic request fields remain in `request_object_digest`; non-null trusted operation fields, missing or extra terminal Problems, a mismatch between Problem details and admission evidence, an operation schema in the manifest, or use after a definition is accepted refuse serialization. Direct SDK dynamic admission, CLI JSON adapter, Tauri, and bridge fixtures prove every variant, all missing/malformed distinctions, registry-missing versus client-pin-missing, compound-error precedence, and byte-equivalent semantics. Additional fixtures cover valid changed IDs, absent/null/malformed IDs, client-fingerprint-only changes, combined changes, and duplicate top-level/nested selection or excluded fields against the exact correlation and no-response matrices above.

## Transport request contract

Transport adapters construct the same generic envelope around one operation-specific typed body. Rust callers normally use the typed method/request directly; the SDK derives this envelope for transport parity and proof.

```json
{
  "schema_id": "handbook.operation-request",
  "schema_version": "1.0.0",
  "request_id": "req_example",
  "operation_ref": "artifact.validate@1.0.0",
  "operation_definition_fingerprint": "sha256:...",
  "client": {
    "client_id": "handbook-cli",
    "client_version": "0.0.0-example",
    "api_version": "1.0.0"
  },
  "expected_response_schema": {
    "ref": "handbook.operation-response@1.0.0",
    "fingerprint": "sha256:..."
  },
  "negotiated_api_context": {
    "api_version": "1.0.0",
    "bootstrap_descriptor": {
      "ref": "handbook.bootstrap-descriptor@1.0.0",
      "fingerprint": "sha256:..."
    }
  },
  "context": {
    "repository_root": ".",
    "repository_identity_fingerprint": "sha256:...",
    "resolution_envelope": null,
    "snapshot": null
  },
  "body": {},
  "extensions": {},
  "request_fingerprint": "sha256:..."
}
```

Request rules:

1. `request_id` is opaque correlation metadata, unique within the client session, and excluded from semantic replay decisions. A valid ordinary request uses the same safe non-null string syntax used by admission echoing. After exact definition acceptance, absent, null, non-string, overlong, or disallowed correlation input is an ordinary `request_validation` refusal under the total response rule below; it never re-enters the admission envelope.
2. `operation_ref`, definition fingerprint, expected response schema pair, and `negotiated_api_context` are mandatory exact pins. The API context must match the descriptor used for bootstrap and `client.api_version`; a mismatch refuses before deserializing the operation body.
3. The adapter resolves cwd/repository discovery before SDK invocation and records an explicit trusted root plus identity fingerprint. Owner logic never reads ambient cwd.
4. Resolution/snapshot refs are null only when the operation definition permits them; any non-null ref is exact and fingerprint-validated by the body schema.
5. `body` validates against the exact operation request schema. Mutation bodies carry operation-owned idempotency keys and/or current compare-and-write fingerprints; the generic envelope never invents mutation authority.
6. `extensions` is exactly `{}` in v1. Any non-empty map refuses as `transport.extensions_unsupported`; extensions cannot add semantics outside the operation-definition fingerprint closure.
7. `request_fingerprint` is normalized SHA-256 over operation ref/fingerprint, expected response schema, exact negotiated API version/descriptor binding, context, body, and extensions. It excludes `request_id`, `client_id`, and `client_version`; `client.api_version` is semantic, must equal the negotiated API version, and is covered through that exact context.

Transport schemas use closed objects (`additionalProperties: false`); the v1 `extensions` property has `maxProperties: 0`. Normalization sorts object keys, preserves array order, uses UTF-8 and canonical JSON scalar forms, rejects non-finite numbers, and excludes only fields explicitly named by that schema's fingerprint rule.

## Transport response and expected-outcome contract

```json
{
  "schema_id": "handbook.operation-response",
  "schema_version": "1.0.0",
  "request_id": "req_example",
  "request_fingerprint": "sha256:...",
  "operation_ref": "artifact.validate@1.0.0",
  "operation_definition_fingerprint": "sha256:...",
  "status": "ok",
  "data": {},
  "blockers": [],
  "refusals": [],
  "errors": [],
  "diagnostics": [],
  "next_actions": [],
  "artifact_refs": [],
  "write_receipts": [],
  "idempotency": {
    "state": "not_applicable"
  },
  "provenance": {
    "source_bindings": [],
    "resolution_envelope": null,
    "snapshot": null,
    "omissions": []
  },
  "schema_manifest": [
    {
      "schema": {
        "ref": "handbook.operation-response@1.0.0",
        "fingerprint": "sha256:..."
      },
      "draft": "2020-12",
      "byte_length": 2048,
      "media_type": "application/schema+json"
    },
    {
      "schema": {
        "ref": "handbook.operation.artifact-validate-result@1.0.0",
        "fingerprint": "sha256:..."
      },
      "draft": "2020-12",
      "byte_length": 1024,
      "media_type": "application/schema+json"
    }
  ],
  "response_fingerprint": "sha256:..."
}
```

| `status` | `data` | `blockers` | `refusals` | `errors` | Meaning |
|---|---|---|---|---|---|
| `ok` | required, exact result schema | empty | empty | empty | operation executed as specified; domain results such as a failed validation/verdict remain typed inside data |
| `blocked` | null | non-empty | empty | empty | request is valid but named prerequisite/external state is missing; each blocker includes a typed recheck condition |
| `refused` | null | empty | non-empty | empty | request violates schema, capability, Resolution, authority, precondition, or safety contract and must change before retry |
| `error` | null | empty | empty | non-empty | unexpected implementation/adapter failure prevented a trustworthy result; no domain decision may be inferred |

All four terminal fields are always present. Mixed terminal outcome arrays refuse response construction. `request_fingerprint` is mandatory and must equal the normalized accepted-definition request; failures before bounded UTF-8/JSON-object normalization are adapter/bootstrap failures with no Handbook response, while selection-field failures use the non-null-fingerprint admission envelope above. `idempotency` is a closed discriminated union:

The ordinary response's `request_id` is also `string | null`. A supplied ID matching `^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$` is echoed exactly. After exact definition acceptance, absent, null, non-string, overlong, or disallowed input emits null and terminates as `status=refused` with one `request_validation`/`schema` Problem whose exact code is `transport.request_id_invalid` and whose bounded details are the constant `{ "field": "request_id", "rule": "safe_correlation" }`; no raw correlation value or invalid-shape discriminator is included. The ordinary semantic `request_fingerprint`, details fingerprint, and Problem fingerprint therefore remain invariant across all invalid correlation representations. A read-only operation uses `idempotency.state=not_applicable`; a mutation uses `not_established` at `request_validation`, before key establishment or any write. The outer response fingerprint binds the emitted valid string or null. Cross-adapter fixtures cover the full valid/absent/null/non-string/overlong/disallowed matrix for both mutability classes.

- `not_applicable` is valid if and only if an accepted operation definition is read-only, regardless of terminal response status;
- `not_established` is valid if and only if an accepted mutating operation terminates before a domain commit and has exactly `{state, stage, terminal_problem}` with no key/result fields. `stage` is one of `capability_validation`, `request_validation`, `resolution_validation`, `authority_validation`, `precondition_validation`, `safety_validation`, `idempotency_validation`, or `execution_start`. `terminal_problem: ProblemBinding` must match exactly one terminal Problem in the response by ID and instance fingerprint; and
- `established` carries `{state, key, request_fingerprint, replayed, original_result_fingerprint, result_retention_until_utc}`, with original result null on first execution and equal to the persisted committed-result closure fingerprint defined above on replay.

The ordinary-response variants are mutually exclusive and total after exact definition acceptance: every response for an accepted read-only operation, including blocked/refused/error, uses only `not_applicable`; only an accepted mutating pre-commit blocked/refused/error uses `not_established`; and only a committed mutation result/replay uses `established`. Pre-definition failures use only the separate admission envelope. After selection, stage/category binding is exact: `capability_validation` permits blocker `prerequisite` or refusal `capability`; `request_validation` only refusal `schema`; `resolution_validation` blocker `prerequisite` or refusal `resolution`; `authority_validation` blocker `prerequisite` or refusal `authority`; `precondition_validation` blocker `prerequisite` or refusal `precondition`; `safety_validation` only refusal `safety`; `idempotency_validation` only refusal `idempotency`; and `execution_start` only error `implementation` or `adapter`. An implementation/adapter error detected after selection but before domain commit uses `execution_start` even if the underlying adapter failed while serving an earlier validation step. No other stage/status/category triple is valid.

Key establishment normally occurs only after API/definition/capability/envelope/body/key/precondition validation and immediately before mutation execution. The memory-promotion stale-terminal exception above establishes after structural/admission/actor-authority validation but before its domain compare. Every committed mutation response is `established`.

`write_receipts` is always present: it is empty for read-only and pre-execution non-`ok` outcomes, while a successful mutation lists exactly the operation write-set items realized by its typed data variant. A post-commit adapter failure emits no ordinary response; retry uses the established key to recover the schema-valid `ok` response and receipts. Every non-null Resolution, snapshot, source, schema, evidence, next-action subject, and artifact binding carries exact identity/version plus fingerprint; a bare ref is invalid. Large/binary outputs and sensitive details use bounded artifact refs rather than inline payloads.

The shared DTO schemas are closed and exact:

| DTO | Required fields and closed semantics |
|---|---|
| `ExactBinding` | `ref`, `fingerprint`; both non-empty, mutually matching, and no bare/relative-latest ref |
| `Problem` | unique response-local `problem_id`, `code`, closed `category`, `subject: ExactBinding`, `rule: ExactBinding|null`, `details_schema: ExactBinding`, schema-valid bounded `details`, derived `details_fingerprint`, `evidence: ArtifactRef[]`, `retry`, `recheck`, `correlation_ref`, and derived `problem_fingerprint` |
| `Diagnostic` | `code`, `severity` (`information` or `warning`), `subject: ExactBinding`, `details_schema: ExactBinding`, bounded `details`, derived `details_fingerprint`, optional non-authoritative `display_message` |
| `NextAction` | stable `action_id`, allowlisted `kind`, `subject: ExactBinding`, `parameter_schema: ExactBinding`, schema-valid bounded `parameters`, optional non-authoritative `display_label` |
| `ArtifactRef` | `artifact: ExactBinding`, `media_type`, `byte_length`, `sensitivity` (`public`, `internal`, or `restricted`), and closed tagged `locator`; artifact fingerprint is the exact content fingerprint and restricted bytes are never inline |
| `SourceBinding` | `source: ExactBinding`, `captured_revision: ExactBinding`, `adapter: ExactBinding`; no ambient or label-only source |
| `Omission` | `item: ExactBinding`, allowlisted `reason`, `proof_effect` (`none`, `partial`, `blocked`, or `refused`), `source: SourceBinding|null`, and stable `ordinal` |
| `SchemaManifestEntry` | `schema: ExactBinding`, `draft` (exactly `2020-12`), `byte_length`, and `media_type` (exactly `application/schema+json`) |
| `CatalogRoot` | `catalog: ExactBinding`, `entry_schema: ExactBinding`, nonnegative `total_entries`, `canonical_sort: ExactBinding`, and non-null future `retention_until_utc`; `catalog.fingerprint` is the independently recomputable catalog-snapshot digest defined below |
| `CatalogCursor` | `catalog: ExactBinding`, bounded `next_sort_key`, and derived `cursor_fingerprint`; it is valid only for that catalog fingerprint and exact next key |
| `CatalogPage` | `root: CatalogRoot`, schema-valid bounded `entries`, `next_cursor: CatalogCursor|null`, and derived `page_fingerprint`; entries are one duplicate-free contiguous sequence in the root's exact sort |
| `RecheckCondition` | `condition_schema: ExactBinding`, schema-valid bounded `condition`, and derived `condition_fingerprint`; evaluation is deterministic and cannot perform an implicit mutation |
| `CorrelationRef` | opaque non-secret token matching `^corr_[A-Za-z0-9_-]{16,128}$`; it reveals no path, input, trace, or credential |
| `WriteReceipt` | `record_kind`, `record: ExactBinding`, `authority_class`, `condition`, `atomic_group`, and derived `receipt_fingerprint`; it must match one realized operation-definition write-set item exactly |
| `IdempotencyState` | ordinary-response closed union `not_applicable`, `not_established`, or `established` with the exact mutability/stage/Problem-binding fields defined above; preselection uses the separate admission envelope and no ordinary idempotency field |
| `ProblemBinding` | `{problem_id, problem_fingerprint}`; both must match exactly one terminal Problem instance in the same response, and problem IDs are unique across all terminal arrays |

For `Problem`, category is a closed enum partitioned by terminal variant: blocker permits only `prerequisite`; refusal permits only `schema`, `compatibility`, `capability`, `resolution`, `authority`, `precondition`, `safety`, or `idempotency`; error permits only `implementation` or `adapter`. A blocker requires `rule: null`, a non-null `RecheckCondition`, `correlation_ref: null`, and retry `after_recheck`. A refusal requires a non-null violated `rule`, `recheck: null`, `correlation_ref: null`, and retry `after_request_change`, `after_authority_change`, or `never`. An error requires `rule: null`, `recheck: null`, a non-null `CorrelationRef`, retry `transient` or `never`, and redacted details. No other category or conditional field combination is schema-valid. `diagnostics` can never contain a blocker/refusal/error-only severity or supply the sole decision. `next_actions` cannot grant authority or weaken the associated problem.

`ArtifactRef.locator` is a discriminated union. `repo_relative` requires `{kind, repository_identity_fingerprint, path, no_follow: true}`; the path is UTF-8 NFC with `/` separators, is non-empty and relative, and contains no empty, `.`, `..`, NUL, platform-prefix, or percent-encoded traversal segment. Resolution opens from the already trusted repository root, rejects symlinks at every component with no-follow semantics, and verifies bytes against `artifact.fingerprint`. `content_addressed` requires `{kind, store: ExactBinding, algorithm: "sha256", digest}` where digest equals the artifact fingerprint and fetched bytes recompute identically. Unknown locator kinds/fields, unsafe paths, symlinks, store mismatch, digest mismatch, or post-open identity change refuse.

All DTO collections have operation-schema byte/item limits and reject overflow before serialization. Blockers/refusals/errors are unique and sorted by `(code, subject.ref, details_schema.ref, details fingerprint)`; diagnostics by `(severity, code, subject.ref, details fingerprint)`; next actions by `(action_id, subject.ref)`; artifact/source/schema bindings by `(ref-or-id, fingerprint)`; omissions by unique ordinal. Collections with domain-significant order must name that order in the operation result schema and preserve it; unordered duplicates refuse. Optional display fields are nullable/omittable only where named above and never enter machine decisions. Unknown enum values, unknown fields, invalid nulls, secret-bearing inline details, unsafe locators, unbounded values, or non-canonical ordering refuse response construction.

Every shared DTO instance fingerprint uses RFC 8785 canonical JSON and lowercase `sha256:<64-hex>`. A schema names exactly one fingerprint field; recomputation includes every other schema field and every nested exact binding and excludes only that instance's own fingerprint field. `details_fingerprint` covers `{details_schema, details}`; diagnostic details use the same rule; `problem_fingerprint` covers the complete Problem including `details_fingerprint` but excluding only itself; `condition_fingerprint` covers `{condition_schema, condition}`; `cursor_fingerprint` covers `{catalog, next_sort_key}`; `page_fingerprint` covers `{root, entries, next_cursor}`; and `receipt_fingerprint` covers `{record_kind, record, authority_class, condition, atomic_group}`. A catalog snapshot is the one exception to avoid circularity: `catalog.fingerprint` covers `{catalog.ref, entry_schema, total_entries, canonical_sort, retention_until_utc, ordered_entry_bindings}` and does not include the enclosing `CatalogRoot` or `catalog.fingerprint` itself. A client validates each page immediately and, after collecting the declared total entry set, recomputes the catalog fingerprint before treating traversal as complete. The response fingerprint then covers these already-validated DTO values. Any inner mismatch refuses response construction even when the outer document would otherwise validate; transplanted Problems/cursors/pages/conditions/receipts fail their exact input closure.

The idempotency ledger is **protocol-control metadata**, not a governed/domain record: it has no authority class, never appears in an operation write set or `write_receipts`, cannot be returned by `record.list`/`record.read`, and cannot satisfy evidence or authority requirements. Its internal closed states are `reserved`, `committed_result`, `aborted`, and `consumed_tombstone`. Reservation and the domain transaction share one crash-recoverable transaction journal. A proven pre-domain failure atomically aborts/removes the reservation and returns mutation `not_established`; a domain commit atomically records `committed_result` with the exact retained closure, realized write receipts, and recomputable original-result fingerprint; an unknown commit outcome emits no ordinary response and recovery resolves the journal to committed replay or safe abort before retry; retained-result compaction atomically replaces only control metadata with a tombstone carrying key scope, request fingerprint, and that same original-result fingerprint. Therefore establishment failure, replay retention, and tombstone compaction are durable protocol mechanics but never undeclared domain writes or receipts.

`provenance` always contains canonically ordered `source_bindings`, exact-or-null Resolution and Snapshot bindings, and ordered `omissions`; it cannot be replaced by free-form text. `schema_manifest` lists every response/result/problem/details/parameter schema as `SchemaManifestEntry` values. `response_fingerprint` covers every field except itself using the request normalization rule. Producers validate the response against the generic, shared-DTO, and operation-result schemas before serialization; failure becomes an adapter invariant error and must never emit a partial success document.

## DTO and JSON Schema generation contract

1. SDK request/result/problem/reference DTOs are public typed Rust structs/enums with Serde and Draft 2020-12 JSON Schema generation.
2. The reviewed Rust type plus its checked-in generated schema are one public contract. CI regenerates into a temporary tree and requires byte identity, recomputes schema fingerprints, and fails if either side drifts.
3. Discriminated unions use explicit tag fields; public data shapes do not use unbounded `serde_json::Value`, flatten unknown fields, or infer variants from field presence.
4. Every public schema has exact ID/version, closed-object policy, explicit nullability/default rules, and a recomputable fingerprint producer.
5. Breaking field/removal/meaning changes require a new operation/schema major. Additive optional fields require a new schema/operation minor and remain absent-by-default. Patch releases cannot change serialized meaning.
6. Clients refuse unsupported majors and definition/schema fingerprint mismatch. They may accept a higher compatible minor only when capability negotiation explicitly advertises it; no range/latest fallback occurs.
7. OpenAPI may later describe an HTTP adapter but cannot become authority for SDK, CLI, or Tauri DTOs.

The generated package includes the immutable bootstrap descriptor and every discoverable schema document/index. Capability and profile/schema catalog fixtures prove cold, compatible, stale, unsupported-major, unknown-operation, missing-profile/schema, and tampered-fingerprint behavior. A machine client discovers behavior from these typed catalogs or exact compile-time Rust types, never by parsing CLI help, examples, error prose, or Tauri command names.

## CLI JSON contract

Each recognized nontrivial CLI invocation maps to exactly one operation definition. Multiple command paths may map to one operation, but command grammar is not operation identity. Custom kinds/profiles/vocabulary are arguments or body data and never add/rename commands.

In JSON mode:

1. stdout contains exactly one complete UTF-8 JSON response document followed only by zero or one LF byte;
2. stdout is empty until the validated response can be emitted atomically;
3. progress, human diagnostics, tracing, and logs use stderr and do not alter the response;
4. `ok`, `blocked`, `refused`, and `error` all serialize the response contract; no expected outcome is prose-only;
5. no ANSI styling, prompts, banners, or progress frames appear on stdout;
6. human output is rendered from the same typed SDK result/outcome, not from a second domain path;
7. schema/operation/capability pins and response fingerprints are preserved byte-for-value;
8. binary/large output uses bounded artifact refs.

Exit mapping is exact:

| Exit | Meaning |
|---|---|
| `0` | response `status=ok`, including a successfully evaluated domain result whose typed verdict/validation is negative |
| `3` | response `status=blocked` |
| `4` | response `status=refused` |
| `5` | response `status=error` |
| `64` | CLI grammar/usage failed before an operation could be selected; help/version are also shell-only, not operation responses |
| `70` | the CLI could not validate/serialize one complete response; stdout remains empty and stderr reports only a safe adapter correlation code |

For selected JSON operations, exit and response status must agree. Machine consumers parse and validate the response; exit status alone is not the domain result. The CLI never maps a contract/validation `fail` in successful `data` to transport `error`.

## Tauri adapter contract

The Tauri adapter maintains a checked mapping from each exposed Tauri command ID to one exact SDK operation ref/fingerprint. It deserializes the operation request DTO, invokes the typed SDK method, and serializes the exact SDK response DTO without reshaping domain fields.

- command naming, window state, cancellation wiring, and async/blocking scheduling are adapter concerns;
- repository discovery becomes an explicit trusted request context before the SDK call;
- expected blocked/refused outcomes resolve as response DTOs rather than rejected prose promises;
- an unexpected adapter failure proven to occur after operation selection but before domain commit uses the same typed `error` response;
- post-commit serialization or delivery uncertainty emits no ordinary Handbook response and only a bounded host-level adapter failure; retrying the established idempotency key recovers the committed result and exact receipts without another write;
- normal operation never shells out to `handbook`, parses CLI output, or reimplements capability/Resolution/authority checks;
- the frontend may render and request operations but cannot mutate canonical truth outside an authorized SDK operation.

Tauri parity is proven per operation by serializing the same request fixture through direct SDK, CLI JSON, and Tauri adapter paths and requiring schema-valid semantic equivalence after excluding only declared transport correlation fields. Failure injection separately proves pre-commit adapter failure yields the typed error variant, while post-commit serialization/delivery failure yields no false domain response and an established-key retry replays the committed result, fingerprint, and receipts without a duplicate write.

## Substrate integration contracts

### Transitional bundled-CLI bridge

The Tier-2 bridge is one isolated Substrate-owned process adapter. It must:

1. pin the exact Handbook binary build/version/checksum and reject substitution;
2. call only recognized `--json` operations and preflight `capabilities.describe` against exact operation-definition/schema fingerprints;
3. set an explicit repository root and bounded environment; never let domain behavior depend on ambient cwd;
4. bound wall time, stdout/stderr bytes, input size, process count, and cancellation/termination behavior;
5. accept exactly one complete response on stdout followed by zero or one LF byte, validate both generic and operation schemas/fingerprints, and reject every other trailing byte including spaces, tabs, CRLF, a second LF, banners, data, or another document;
6. treat stderr as non-authoritative diagnostics only and never parse human wording;
7. map spawn/timeout/truncation/version/schema/serialization failures into typed Substrate adapter failures without fabricating a Handbook domain result;
8. carry non-sensitive bounded request metadata only in arguments, pass sensitive request bodies only through bounded stdin or an exact trusted content-addressed/repo-relative ref, and prohibit sensitive values in argv, environment variables, stderr, ambient temporary files, or process titles;
9. redact adapter diagnostics and crash reports by classification before persistence, delete any explicitly allocated private input file on every exit path, and prove with leakage fixtures that secrets are absent from argv/env/stderr/process listings/temp roots; and
10. keep all process-specific behavior behind one replaceable interface and carry `BR-SUB-CLI-01` as its deletion gate.

For a mutating bridge call, spawn failure or a validated response proving pre-establishment termination is a proven pre-commit adapter outcome. Once the child may have entered mutation execution, timeout, cancellation, truncation, invalid exit/status agreement, missing/extra stdout, schema/fingerprint failure, or serialization/delivery failure is **commit-uncertain**. The bridge emits no fabricated Handbook `error`, retains the exact original request envelope, idempotency key, and request fingerprint in bounded private non-authoritative recovery state under the sensitive-input rules above, and retries only that byte-identical semantic request with the same key. Recovery must resolve to safe pre-commit abort or an `established` replay carrying the persisted original-result fingerprint and exact receipts; it never chooses a new key, rebinds the request, or performs a second domain write. Failure injection covers every listed uncertainty both before establishment and after commit-before-valid-stdout.

Bridge proof satisfies only `PG-SUB-CLI-01`. It does not satisfy SDK publication, Tauri parity, direct Rust adoption, or a contract/dock real path.

### Permanent published-Rust boundary

The Tier-4 boundary invokes exact published SDK/owner APIs directly in a real current-tip Substrate product seam. It does not invoke the CLI in that seam. Substrate owns when the use case runs, runtime authority, failure policy, agent orchestration, and final product wording; Handbook owns operation semantics and typed results.

Normal ordinary-consumer integration prefers `handbook-sdk`. Direct imports of `handbook-engine`, `handbook-flow`, `handbook-pipeline`, or `handbook-contracts` are advanced boundaries and must justify why the SDK operation is insufficient without recreating composition. No Substrate crate becomes a Handbook semantic owner.

The direct boundary satisfies `PG-SUB-RUST-01` only after the published API and real-seam proof below pass. At that point the Tier-2 adapter is removed from the normal product path and `BR-SUB-CLI-01` closes; retaining the standalone Handbook CLI product is unrelated to retaining the Substrate bridge.

## Published Rust API proof plan

A downstream-intended API is not complete because its symbol is `pub`, workspace tests pass, or a path dependency compiles. Each new/changed downstream-intended SDK or owner API must pass the following ordered proof:

1. **Compute the release DAG:** reject cycles and order `handbook-engine` before `handbook-flow`/`handbook-contracts`, those owners before pipeline where applicable, all owners before SDK, and SDK before CLI/adapters. Record the exact node/version/dependency plan before publishing the first node.
2. **Interleave proof and publication per node:** visit one node in topological order. Every changed Handbook dependency of that node must already have completed this step and resolve at its exact checksum from crates.io. Only then run public-type/docs/SemVer/generated-schema/unit/integration/negative proof, package include/exclude audit, and `cargo package` for the node from a clean cache with no path/git/patch/workspace fallback.
3. **Isolate then publish that node:** unpack only that node's `.crate` into a clean external consumer, resolve its Handbook dependencies only at their already-published exact crates.io versions, and prove no unlisted workspace/sibling file is required. Then publish the node, wait until its exact version/checksum resolves from crates.io, record the evidence, and advance to the next DAG node. A dependent is never packaged before its new dependency is registry-resolvable. Abort the chain on mismatch/unavailability; resuming starts at the first unpublished node after revalidating all earlier checksums.
4. **Registry-only external consumer:** in a clean directory outside both workspaces, depend on exact `=x.y.z` versions, run locked fetch/metadata/build/tests, and assert each Handbook package source is the crates.io registry with no path, git, `[patch]`, or workspace override.
5. **Current-tip Substrate worktree:** create a dedicated clean worktree from the then-current Substrate tip, pin the exact registry versions, inspect the lockfile/source graph, and implement one named real product seam. A toy binary, manifest-only dependency, or dead test helper does not count.
6. **Real-seam proof wall:** exercise positive, typed blocked/refused, incompatible-version/schema, and no-fallback cases; run Substrate format/lint/build/tests and the seam's runtime proof; demonstrate Substrate-owned wording/orchestration and Handbook-owned semantics.
7. **Evidence record:** preserve Handbook and Substrate commit/tree IDs, crate versions/checksums, registry metadata/source assertions, lockfile diff, exact seam path/symbol, commands/results, negative proof, and independent review.

`PG-PUBLISH-01` closes for the exact published API only after steps 1-4. `PG-SUB-RUST-01` closes for the exact real seam only after steps 5-7. A prior release proof remains evidence only for the exact versions/APIs/seam it exercised. The Tier-2 bridge and Tier-4 boundary have separate subject fingerprints and cannot substitute proof for each other.

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

The bridge must satisfy the complete **Transitional bundled-CLI bridge** contract above: exact binary version/checksum, exact operation/schema/capability fingerprints, bounded process behavior, one validated response document, a real Substrate seam, no human-output parsing, typed fail-closed adapter errors, and `BR-SUB-CLI-01`.

### Published Rust API gate

A downstream-intended API is complete only after the complete **Published Rust API proof plan** above passes, including packaged-artifact isolation, exact crates.io publication, registry-only external consumption, a dedicated current-tip Substrate worktree, a named real seam, negative/no-fallback proof, and preserved review evidence.

The CLI bridge may ship earlier, but it cannot satisfy this Rust API gate.

## Schema compatibility posture

This greenfield program does not promise compatibility with legacy Handbook artifact formats.

Compatibility rules apply only after a new schema/API is deliberately published as supported. Temporary internal cutover types must name their deletion gate and cannot become implicit public contracts.
