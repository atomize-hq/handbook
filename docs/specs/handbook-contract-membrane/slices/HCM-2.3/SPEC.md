# HCM-2.3 Specification: Generic Custom-Kind Registration, Intake, and Validation Proof

## Status and authority

This packet is the planning authority for HCM-2.3. It was prepared from the
explicit HCM-2.3 documentation selector at HCM-2.2 closeout HEAD
`5c31eeefb5adf71d75ec3059b1b6947025d2fd6b`. HCM-2.2 is a completed,
review-clean dependency: its primary implementation commit is
`6766d3ed4894aad6598faaa7c4a54b493f92c1f6`, its closeout commit is the entry
HEAD above, and its final Review 5 subject fingerprint is
`sha256:62c9fdae649a31d1538ec7770b0f8ce5b2cc0686cb33535fe8747bbe2ddc80ad`.

This documentation session does not authorize Rust, Cargo, runtime, test,
asset, CLI, SDK, or behavior changes. A later implementation may begin only
after this complete planning subject is review-clean, committed, closed by one
parent-owned handoff, and that exact implementation handoff is explicitly
selected by the user. No packet or handoff authorizes itself.

The approved HCM-2.2 candidate `1.3`, lifecycle-validation result `1.0`,
promotion intent `1.2`, exact-result binding, Charter lineage, approval,
promotion, recovery, and canonical bytes are immutable dependencies. HCM-2.3
must not reinterpret, migrate, generalize in place, or change them.

## Objective

Prove one repository-defined custom artifact kind through a stable generic
product path without adding a Rust enum variant, kind-specific command,
filename switch, shipped default, or caller-defined semantic authority. The
proof must:

1. load explicit repository schema, kind, profile, instance, and optional
   intake sources through the existing safe exact-ref model;
2. select the artifact by exact kind ref plus instance ID, with the descriptor
   owning its trusted canonical path and intake selection;
3. validate canonical YAML through the selected kind's exact registered schema;
4. evaluate supplied intake, retain one generic intake/candidate lineage, and
   support compare-and-write promotion for this non-governed proof kind;
5. exercise the boundary through stable generic CLI commands and public
   engine-owned typed operations; and
6. leave Phase 4's named `handbook-sdk`, public transport catalog, JSON/Tauri
   membrane, publication, and downstream consumer program unclaimed.

Success is one real CLI integration path whose kind ref and instance ID remain
ordinary request data from parse through engine result, whose repository
definitions are not package-owned defaults, and whose positive, negative,
attack, restart, replay, and concurrency proofs pass the full regression wall.

## Frozen assumptions and decisions

- Reuse the completed HCM-1.3 `registry-brief` lineage. Do not invent a new
  first-party kind and do not modify HCM-0.6 catalog bytes.
- Keep the existing capability-free kind: semantic validation is explicitly
  absent because its capability and semantic-validator sets are empty.
- Add one optional, observational, non-governed intake definition. Its
  `approval_policy_ref` is `null`; it does not inherit Charter candidate `1.3`,
  constitutional approval, lifecycle, waiver, or authority rules.
- Use a new exact repository-scoped profile version for the descriptor that
  selects intake. It extends the unchanged shipped root and materializes
  complete replacement lists; existing HCM-1.3 fixture definitions and
  fingerprints stay unchanged.
- Product adoption is bounded to new generic artifact commands. Existing
  setup, doctor, flow, author families, generated skills, and shipped-profile
  decisions do not select the proof artifact.
- The proof may add engine-owned typed use cases that a future SDK can compose,
  but it may not create, name as delivered, or claim `handbook-sdk`.
- No new dependency is anticipated. A Cargo or dependency change is a stop for
  explicit authority review.

## Authority and owner split

| Concern | Exact owner in HCM-2.3 | Adapter rule |
|---|---|---|
| definition source admission, exact refs, normalization, fingerprints | `handbook-engine` | CLI supplies no definition bytes or semantic overrides |
| schema/kind/intake registries and compatibility | `handbook-engine` | IDs are passed through unchanged |
| profile, instance, trusted path, requiredness, dependencies | `handbook-engine` selected-profile/descriptor owners | no filename, enum, label, or command inference |
| structural validation and generic intake evaluation | `handbook-engine` | CLI renders typed results only |
| immutable intake/candidate/promotion records and currentness | `handbook-engine` | CLI cannot write state directly |
| generic command parsing and human/JSON presentation | `handbook-cli` | no semantic checks duplicated in Clap or renderer code |
| future public SDK composition and transport DTO catalog | Phase 4 | not delivered or claimed by HCM-2.3 |

The kind owns reusable schema identity and structural behavior. The separate
`ArtifactIntakeDefinition` targets the kind and supplies optional reusable
intake semantics. The instance owns repository identity, canonical path, label,
requiredness, applicability, dependency bindings, and the exact selected
`intake_definition_ref`. The kind must not list or fingerprint intake
definitions; that direction would create a cycle. A path, label, filename,
Rust enum, or CLI spelling never grants kind or instance authority.

## Exact proof lineage

The fixture extends, rather than rewrites, the HCM-1.1/HCM-1.3 proof lineage:

| Role | Exact identity/value | Disposition |
|---|---|---|
| stable-role registry | `handbook.roles.core@1.1.0` | reuse package-owned exact bytes |
| stable role | `project_context` | reuse; it grants no first-party default membership |
| content schema | `example.schemas.registry-brief@1.0.0` | reuse HCM-1.3 logical schema and exact closure |
| artifact kind | `example.artifact-kind.registry-brief@1.0.0` | reuse capability-free custom kind |
| instance ID | `registry_brief` | reuse symbolic identity |
| instance label | `Registry Brief` | repository-profile data only |
| canonical path | `.handbook/project/registry-brief.yaml` | descriptor-owned trusted repository path |
| requiredness | `always`, `condition_ref: null` | descriptor-owned for the proof repository only |
| dependencies | empty | no implicit sibling authority |
| intake definition | `example.intake.registry-brief@1.0.0` | new exact repository definition |
| selected profile | `example.profile.registry-root@1.1.0` | new repository-scope child of `handbook.profile.shipped-root@1.1.0`; `1.0.0` fixture bytes remain unchanged |

`example.artifact-kind.registry-brief@1.0.0` continues to select
`example.schemas.registry-brief@1.0.0`, role registry
`handbook.roles.core@1.1.0`, stable role `project_context`, empty semantic
capabilities, empty required capabilities, empty semantic validators, and no
renderer, lifecycle, Projection, overlay, or extension behavior. Its content
schema is the closed Draft 2020-12 object used by HCM-1.3:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["title"],
  "properties": {
    "title": {"type": "string", "minLength": 3},
    "summary": {"type": "string", "minLength": 1}
  }
}
```

HCM-2.3 adds only the new repository profile version and the intake selection
on its `registry_brief` descriptor. The profile has `profile_scope: repository`,
`extends_profile_ref: handbook.profile.shipped-root@1.1.0`, and complete
replacement `schema_registry_sources`, `artifact_kind_sources`, and
`artifact_instances` lists containing every unchanged shipped selection plus
the custom schema, kind, and `registry_brief` descriptor. Other inherited fields
remain exact. Coexisting instance roles do not merge identities or grant the
custom instance shipped-default status. It does not mutate an existing exact definition.
Fixture equality tests must prove the HCM-1.3 `1.0.0` profile, schema, kind,
instance, and fingerprints are byte-identical before and after implementation.

## Repository source topology and trust boundary

The real-path fixture is a complete temporary repository rooted by the test,
not a package definition or installed default. It uses exactly these paths:

```text
.handbook/profile-selection.json
.handbook/definitions/profiles/registry-root-1.1.0.yaml
.handbook/definitions/schemas/registry-brief-1.0.0.entry.yaml
.handbook/definitions/schemas/registry-brief-1.0.0.schema.json
.handbook/definitions/artifact-kinds/registry-brief-1.0.0.yaml
.handbook/definitions/intakes/registry-brief-1.0.0.yaml
.handbook/project/registry-brief.yaml
```

`.handbook/profile-selection.json` is a closed serialized counterpart of the
engine selection request. Its exact Draft 2020-12 schema and semantic vectors
are
[`contracts/repository-profile-selection-1.0.0.schema.json`](contracts/repository-profile-selection-1.0.0.schema.json)
and
[`contracts/repository-profile-selection-vectors-v1.0.json`](contracts/repository-profile-selection-vectors-v1.0.json).
It contains exactly:

- `schema_id: handbook.repository-profile-selection`;
- record-routing `schema_version: "1.0"`;
- `selected_profile_ref: example.profile.registry-root@1.1.0`;
- explicit typed source-binding arrays. A binding is exactly
  `{exact_ref, source: {kind: built_in}}` or
  `{exact_ref, source: {kind: repository_path, path}}`; custom profile/schema/
  kind/intake bindings use only the repository-path variant above;
- all existing per-definition-class arrays needed by the shipped `1.1.0`
  parent: profiles, stable-role registries, schema entries, artifact kinds,
  semantic capabilities, semantic validators, project conditions,
  vocabularies, Context Resolution definitions, and Context Resolution policy
  definitions, plus the separate intake-definition source array;
- `allowed_schema_roots` containing only
  `.handbook/definitions/schemas`; and
- `extensions: {}`.

Every named array above is a required top-level field, including empty source
classes. The closed serializer maps a `built_in` binding to
`DefinitionSource::BuiltIn(binding.exact_ref)` and a `repository_path` binding
to `DefinitionSource::RepositoryPath(source.path)`; the additive
`intake_definition_sources` array maps only to the new typed request field of
that name. There is no second built-in ref, caller override object, inferred
empty array, omitted class, or field-name alias. Duplicate JSON members refuse
before object construction. Source-array order does not affect admitted
registries or fingerprints; the positive permutation vector proves the exact
typed round trip.

The selection record's normalized fingerprint has one executable preimage.
After duplicate-safe closed-schema decoding and semantic duplicate/class checks,
sort every `*_sources` array by the UTF-8 bytes of `exact_ref`, sort
`allowed_schema_roots` by UTF-8 bytes, leave every binding object and all other
values unchanged, RFC-8785-canonicalize the complete normalized record, and
SHA-256 those bytes without LF. No authored array order enters the fingerprint.
The positive and concrete fully reversed records must both normalize to the
same bytes and declared fingerprint; a hash of either raw authored record is
not the normalized fingerprint.

Every source list is explicit and complete for its class. The existing shipped
source bindings and bytes are consumed unchanged; the selected repository
profile's replacement lists cite the complete effective shipped-plus-custom
sets. Missing a required parent source or omitting one member from a present
replacement list refuses rather than inheriting by source discovery.

Admission also enforces semantic identity uniqueness that Draft 2020-12 cannot
express for object arrays: within each source class, `exact_ref` is unique even
when two bindings name different sources, and a ref cannot be rebound under a
different definition class. The resolver must execute the selected profile
against the complete typed source closure and require the exact counts and
replacement members in the positive vector. JSON-Schema acceptance alone is
insufficient; duplicate-ref/different-source, cross-class, incomplete-source,
and incomplete-replacement vectors must refuse before any profile is admitted.

The fixed path is a protocol location, not ambient discovery: a generic
artifact command requires an explicit `--repository-root`, opens exactly that
record beneath the retained root, and does not scan ancestors, cwd, Git state,
environment variables, sibling directories, filenames, or conventional kind
locations. The record binds paths to exact refs but cannot override profile
fields, schema content, kind behavior, descriptor fields, fingerprints, or
operation semantics. Each bound source must derive the bound exact ref and
recompute its authored fingerprint. The selected profile remains semantic
authority; the selection record is a closed effect binding.

The strict local source reader retains the explicitly selected repository root
as trust anchor and reuses HCM-1.1/HCM-1.2 descriptor-relative no-follow
admission. It accepts only normalized `/`-separated repository-relative paths,
at most 1,024 ASCII bytes and 64 nonempty components, with no leading/trailing
slash, empty component, `.`/`..` component, backslash, colon/URI/drive prefix,
or non-schema character. Exact definition refs reuse the complete HCM-1.1
identity contract: 3–255 lowercase ASCII identity bytes, at least two dot-
separated 1–63-byte segments matching
`[a-z][a-z0-9]*(?:-[a-z0-9]+)*`, one `@`, and a full SemVer lexeme equal to
`semver::Version::to_string()`. The closed schemas reject one-segment,
underscore, repeated-hyphen, noncanonical-version, dot-component, trailing-
slash, over-64-component, over-1,024-byte, and final-line-terminator mutations
directly; no exact ref or repository-relative path admits whitespace or a line
terminator. Each source is at most 1 MiB, the shared
selection budget is 8 MiB, profile sources are at most 64, all definition
bindings together are at most 512, allowed schema roots are at most 32, schema
closure contains at most 128 documents, and local `$ref` depth is at most 32.
Existing exact lower limits continue to apply where stricter.

There is no ambient source discovery, latest/range resolution, remote/file/data
fetch, network/TLS resolver, executable schema hook, custom keyword callback,
renderer process, or arbitrary caller-provided validator.

### Deterministic identity and conflict behavior

All YAML and JSON sources reject duplicate keys before conversion. Authored
definition fingerprints remove exactly their self-fingerprint field, normalize
the remaining parsed value through RFC 8785 JCS, and compute lowercase
`sha256:<64-hex>`. Schema-document fingerprints cover exact admitted bytes as
already frozen by HCM-1.1. Registry fingerprints cover the stable ordered array
of exact ref plus recomputed definition/closure fingerprints; source order
cannot affect them.

The intake registry is additive and separate. `ArtifactOperationContextV1`
binds the unchanged `resolved_profile_fingerprint`, kind-registry fingerprint,
intake-registry fingerprint, exact kind ref, instance ID, descriptor
fingerprint, canonical path, and selected intake ref/fingerprint. This context
fingerprint is the generic operation currentness anchor. HCM-2.3 must not change
the existing resolved-profile, HCM-2.2 Charter, result, intent, candidate, or
definition fingerprint algorithms.

Identical duplicate exact definitions refuse as `duplicate_identity`;
same-ref/different-normalized-content refuses as `conflicting_identity`.
Mismatched bound ref, declared fingerprint, schema entry/document pair, kind/
schema ref, intake/kind ref, descriptor/kind ref, or descriptor/intake ref
refuses before canonical content is read. Missing, absolute, escaping,
backslash, symlinked, non-regular, oversized, over-total, over-count, over-depth,
cyclic, mutated-during-read, or outside-allowed-root sources refuse with a typed
stable path and no partial registry. Both source orders must produce the same
error class and precedence.

## Exact optional intake definition

The repository definition is the exact `ArtifactIntakeDefinition` shape from
`05-contracts-schemas-and-gates.md`:

```yaml
schema_id: handbook.artifact-intake-definition
schema_version: "1.0"
intake_id: example.intake.registry-brief
intake_version: 1.0.0
artifact_kind_ref: example.artifact-kind.registry-brief@1.0.0
candidate_schema_ref: example.schemas.registry-brief@1.0.0
supported_modes: [guided_adaptive, express, agent_assisted]
coverage:
  - coverage_id: registry_brief.title
    target_paths: [/title]
    applicability: always
    authority_class: observational
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
      waiver_policy_ref: null
    prompt_guidance_refs: []
  - coverage_id: registry_brief.summary
    target_paths: [/summary]
    applicability: always
    authority_class: observational
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
      waiver_policy_ref: null
    prompt_guidance_refs: []
approval_policy_ref: null
reassessment_triggers: []
extensions: {}
intake_definition_fingerprint: sha256:<derived-by-uniform-definition-algorithm>
```

The implementation computes and freezes the literal digest before GREEN; a
placeholder may not enter a runtime fixture or proof manifest. The two target
paths are distinct, schema-resolvable, and cover every candidate leaf. The
schema's optional `summary` becomes required only when this optional intake is
selected; structural schema reuse is unchanged for instances that do not select
the intake. There are no defaults, inferred values, waivers, approvals,
reassessment triggers, prompt callbacks, evidence capture, or Charter coverage
inheritance.

If an instance has `intake_definition_ref: null`, `intake.definition.read` and
all intake/candidate mutation operations return typed `not_applicable`; ordinary
artifact read/validate remains available. If non-null, the selected exact intake
must exist, target the instance's selected kind, select the same exact schema as
the kind, and pass complete coverage meta-validation before any content read.

## Intake input, coverage, candidate, and canonical truth

All three authorized acquisition modes accept the same closed engine request:

```text
repository_root
kind_ref
instance_id
mode = guided_adaptive | express | agent_assisted
expected_current_fingerprint = sha256:... | absent
coverage_submissions[] = {
  coverage_id,
  state = supplied | known_unknown | contradicted,
  source_kind = user_declaration,
  value,
  specificity,
  confidence,
  contradiction_refs[]
}
```

The read-only evaluator and candidate validator have no idempotency key. Each
mutation request carries one required bounded UTF-8 `idempotency_key` inside the
closed request document, never argv. Exact domain replay scope, lookup-before-
currentness order, 30-day retained results, non-expiring tombstones, recovery
holds, raw-key exclusion, and the non-reinterpreting future Phase 4 wrapper are
frozen in
[`contracts/generic-artifact-runtime-contract-v1.0.md`](contracts/generic-artifact-runtime-contract-v1.0.md#direct-cli-domain-replay-and-phase-4).
The exact caller operation subject is request data; derived operation context
is neither key scope nor request identity. For a new key, establishment is
permitted only after exact repository/context resolution and deterministic
evaluation produce either the complete commit plan or one closed established
refusal. Establishment atomically publishes one closed intent; the active hold
is its exact logical lookup projection, never a second persisted record. The
runtime contract freezes scratch, established-intent, pending-intent, and crash
transitions byte-for-byte.
Pre-establishment control failures persist nothing; established refusals persist
an outputless but evidence-backed journal/result/ledger lineage. Lookup/replay
precedes fresh context resolution, so context drift cannot establish a second
mutation or change a retained refused result under the same key. The complete
case matrix, closed refusal payload, and recovery prefixes are normative in the
runtime contract and control schema.
HCM-2.3 creates no bootstrap
descriptor, public operation catalog, SDK ledger, or transport claim.

The direct CLI key is exactly 16–128 ASCII characters matching
`^[A-Za-z0-9_-]+$`. Admission requires the existing engine-owned durable
repository identity and the reviewed planning subject fingerprint named by the
implementation handoff. The real-path fixture obtains repository identity
through unchanged HCM-2.2 setup/identity behavior. Missing, unsafe, or changing
identity refuses before ledger lookup or domain mutation.

Mode affects orchestration metadata only. It cannot change field precedence,
coverage order, defaults, schema, candidate construction, persistence, or
promotion. For identical normalized submissions all three modes produce
byte-identical candidate content and the same content fingerprint; their intake
record fingerprints differ only through the explicit `mode` field and resulting
record identity.

Exactly one submission is required for each coverage ID, in definition order.
For this proof both must be `supplied`, `source_kind: user_declaration`,
`specificity: concrete` or stronger, and `confidence: high`. Missing, duplicate,
unknown, contradicted, evidence-derived, defaulted, inferred, waiver-backed,
wrong-source, low-specificity, or low-confidence coverage blocks candidate
creation. Unknown remains explicit in the returned coverage result and never
populates canonical content. Contradictions retain bounded refs and block;
source order or mode cannot choose a winner.

The pure evaluator constructs a normalized-content preview exactly as:

```json
{"summary":"<registry_brief.summary value>","title":"<registry_brief.title value>"}
```

The evaluator validates that value against the selected exact schema but writes
nothing and creates no intake/candidate identity. The executable order and
machine shapes are frozen by
[`contracts/generic-artifact-runtime-contract-v1.0.md`](contracts/generic-artifact-runtime-contract-v1.0.md#exact-executable-operation-sequence),
[`contracts/generic-artifact-runtime-records-1.0.0.schema.json`](contracts/generic-artifact-runtime-records-1.0.0.schema.json),
and
[`contracts/generic-artifact-runtime-vectors-v1.0.json`](contracts/generic-artifact-runtime-vectors-v1.0.json).
The closed operation-context, coverage-evaluation, domain request/result/ledger,
journal intent/verification/marker, and internal-evidence shapes are also
normative in
[`contracts/generic-artifact-control-records-1.0.0.schema.json`](contracts/generic-artifact-control-records-1.0.0.schema.json)
and
[`contracts/generic-artifact-control-vectors-v1.0.json`](contracts/generic-artifact-control-vectors-v1.0.json):

1. evaluate supplied coverage without persistence;
2. append/finalize intake `1.2`, whose sampled timestamp participates in its
   identity;
3. build and validate a candidate preview from that committed intake without a
   write;
4. append candidate `1.4` only after independently reproducing its expected
   fingerprint, persisting its exact normalized-content and validation-result
   closure; and
5. promote with exact promotion record `1.2`.

Candidate `1.4` binds exact kind/schema/instance, operation-context and resolved-
profile fingerprints, intake ref/fingerprint, normalized-content ref/fingerprint,
one validation-result ref, basis canonical fingerprint or explicit null, and a
bijective field-source map for `/title` and `/summary`. Inline candidate content
is not a second record shape.

Runtime admission enforces semantic uniqueness beyond JSON Schema `uniqueItems`:
candidate `field_sources` are unique by both target pointer and coverage ID, and
promotion `resolved_definitions` are unique by exact definition ref. Repeating
either identity with different companion values refuses; schema-valid object
inequality is not permission to create two authorities.

Promotion `resolved_definitions` must additionally equal the cited current
`ArtifactOperationContextV1.resolved_definitions` byte-for-byte, including its
UTF-8 `definition_ref` order. A different fingerprint, missing/extra ref, or
reordered but otherwise equal array is `operation_context_definition_mismatch`;
recomputing only the promotion fingerprint cannot authorize it.

The exact generic record specialization is:

- intake `consumer` is `{kind: handbook_cli, id: handbook, version:
  <current-release>}` for the CLI proof, `prompt_event_refs` is `[]`, every
  satisfied coverage result carries a content-addressed `value_ref`, empty
  `evidence_refs`, `confidence: high`, `freshness: null`,
  `sensitivity: internal`, `evaluation: satisfied`, empty contradiction refs,
  and null waiver; `finalized_at_utc` is sampled once by the engine when its
  append journal is established and is replayed, never resampled;
- validation result `1.0` is a new timestamp-free generic structural result;
  `artifact.candidate.validate` returns its exact derived preview without a
  write, candidate append recomputes and retains it as subordinate closure, and
  promotion reloads and recomputes it under current context;
- candidate `1.4` `validation_result_refs` binds exactly that one result,
  `unresolved_coverage_ids` is `[]`, `promotion_eligibility` is the
  closed value `eligible_without_approval`, and
  `required_approval_policy_ref` is `null`; and
- promotion `1.2` `approval_refs` is `[]`, `decision` is the closed value
  `not_required`, and `authorized_by_ref` is `null`; it still binds the current
  validation result, exact resolved definitions, expected current artifact
  fingerprint or explicit create-null, canonical ref/fingerprint, and its own
  fingerprint.

Intake `1.2`, validation result `1.0`, candidate `1.4`, and promotion `1.2` are
additive exact record versions. All earlier record schemas and bytes, especially
candidate `1.3` and promotion `1.1`, remain closed and unchanged. Cross-version
or cross-store fallback refuses. The non-governed values are available only
when the selected intake has null
approval policy and the selected kind lacks a capability requiring approval.
They are not aliases for `approved`, cannot appear on Charter records, and do
not relax any current validation.

There is no `ArtifactApprovalRecord` for this proof kind because the selected
intake's approval policy is null. Supplying, discovering, or synthesizing an
approval ref is `approval_not_applicable`. Promotion eligibility requires the
retained intake/candidate pair, complete coverage, structural validity, exact
current operation context, and compare-and-write basis; it never treats null
approval as an implicit Charter approval.

Canonical truth exists only at the descriptor path. Intake records and
candidates are evidence/proposals and never become truth by append, filename,
or CLI success. Promotion serializes candidate content through the engine's
generic deterministic YAML emitter, validates the emitted parsed value again
through the same exact selected schema, then compare-and-writes the descriptor
path. The emitter accepts only the JSON data model, sorts mapping keys by UTF-8
byte order, emits mappings/sequences deterministically, emits JSON-compatible
scalars with unambiguous YAML quoting, uses UTF-8 and LF, emits exactly one
document with one final LF, and emits no tags, anchors, aliases, directives, or
comments. Parse/emit/parse equality is mandatory. It contains no per-kind
branch.

## Generic operations and Phase 4 boundary

These stable operation IDs are inherited from the canonical operation table.
HCM-2.3 may implement the bounded owner-library and CLI subset shown here:

| Operation ID | HCM-2.3 owner/action |
|---|---|
| `artifact.kind.list` | engine read of the resolved kind registry |
| `artifact.instance.list` | engine read of the selected instance registry |
| `artifact.read` | engine safe read by kind ref + instance ID |
| `artifact.validate` | engine layered validation of descriptor-selected canonical YAML |
| `intake.definition.read` | engine read of the instance-selected intake definition or not-applicable |
| `intake.coverage.evaluate` | engine pure evaluation from supplied generic submissions |
| `intake.record.append` | engine transaction-owned immutable append |
| `artifact.candidate.validate` | engine structural/currentness validation |
| `artifact.candidate.append` | engine transaction-owned immutable append |
| `artifact.approval.append` | typed not-applicable for this null-policy proof; no generic approval implementation claim |
| `artifact.candidate.promote` | engine non-governed compare-and-write promotion for the selected proof instance |

Instance-targeted requests carry `kind_ref` and `instance_id`; the resolver
requires the instance's actual selected kind to equal the request ref. The two
list requests intentionally carry neither and return the complete selected
registry under the resolved repository profile. Operation routing is only by
the fixed operation ID. No definition creates a command, no command is looked
up dynamically, and no filename, enum variant, label, renderer, or kind-specific
adapter selects behavior.

The exact generic CLI surface is:

```text
handbook artifact list-kinds --repository-root <path> [--json]
handbook artifact list-instances --repository-root <path> [--json]
handbook artifact read --repository-root <path> --kind-ref <exact-ref> --instance-id <id> [--json]
handbook artifact validate --repository-root <path> --kind-ref <exact-ref> --instance-id <id> [--json]
handbook artifact intake-definition --repository-root <path> --kind-ref <exact-ref> --instance-id <id> [--json]
handbook artifact intake-evaluate --repository-root <path> --kind-ref <exact-ref> --instance-id <id> --mode <guided-adaptive|express|agent-assisted> --from-inputs <path|-> [--expected-current-fingerprint <sha256:...|absent>] [--json]
handbook artifact intake-append --repository-root <path> --kind-ref <exact-ref> --instance-id <id> --from-request <path|-> [--json]
handbook artifact candidate-validate --repository-root <path> --kind-ref <exact-ref> --instance-id <id> --intake-record-ref <ref> --intake-record-fingerprint <sha256:...> [--expected-current-fingerprint <sha256:...|absent>] [--json]
handbook artifact candidate-append --repository-root <path> --kind-ref <exact-ref> --instance-id <id> --from-request <path|-> [--json]
handbook artifact promote --repository-root <path> --kind-ref <exact-ref> --instance-id <id> --from-request <path|-> [--json]
```

CLI spellings `guided-adaptive` and `agent-assisted` map mechanically to the
closed request values `guided_adaptive` and `agent_assisted`; `express` is
identical in both forms. No other alias, case fold, or vocabulary label is
accepted.

`--from-inputs -` and `--from-request -` are bounded data input, not an
interactive prompt or authority callback. Mutation request documents carry the
raw idempotency key and all ref/currentness fields; raw keys are prohibited from
argv and every output. Intake append repeats the normalized submissions rather
than trusting the evaluator preview. Candidate append carries the committed
intake ref/fingerprint plus `expected_candidate_fingerprint`; it does not carry
caller-authored candidate, content, or validation-result bytes. Promotion
carries the committed candidate ref/fingerprint and exact expected current
canonical fingerprint/null. Human output is presentation only. JSON output is
an HCM-2.3 local CLI fixture contract, not proof of the complete HCM-0.4 public
transport envelope.

The public Rust types and synchronous engine methods are owner APIs that a
future SDK may compose. HCM-2.3 does not create `handbook-sdk`, negotiate an API
bootstrap, implement the complete operation catalog, freeze published DTO/JSON
Schema manifests, implement Tauri, add a Substrate bridge, publish crates, or
claim `PG-SDK-01`/`PG-JSON-01`. Phase 4 remains the sole owner of those surfaces.

## Validation layers and refusal precedence

Validation is ordered and non-waivable:

1. source/path/identity/fingerprint/schema-closure admission;
2. profile, kind, descriptor, intake, and operation-context compatibility;
3. canonical YAML syntax, duplicate-key, single-document, size, and safe-read
   checks;
4. structural validation through the selected exact JSON Schema;
5. semantic validation: explicitly absent for this capability-free kind;
6. intake coverage/currentness, when intake is selected and requested;
7. approval: explicitly not applicable for this proof; and
8. external evidence/docks: not implemented and not applicable.

Failure at an earlier layer prevents later execution. Later validity, complete
coverage, a retained candidate, a null approval policy, or CLI input cannot
waive an earlier failure. Results identify each layer distinctly; absence of a
semantic validator is `not_applicable`, never an implicit semantic pass.

Within a layer, refusal precedence is: unsafe/untrusted path or changing retained
observation; malformed/duplicate/unsupported record; missing exact dependency;
duplicate identity; conflicting identity/fingerprint; incompatible binding;
resource limit; content validation; currentness; then operation-specific
eligibility. Compound-invalid tests freeze this order.

Source, identity, lock, context-resolution, malformed-ledger, and ambiguous-
journal failures are pre-establishment control refusals and produce no durable
domain record. After exact context resolution, the only refusals that can become
retained domain results are the closed canonical-syntax, structural, intake,
currentness, and operation-eligibility decisions in `establishedRefusal`.
Neither class may be converted into the other during restart or replay.

## Persistence, currentness, crash recovery, and concurrency

The generic store is separate from HCM-2.2 Charter paths and record schemas:

```text
.handbook/state/artifacts/<instance-id>/intake-records/intake_<fingerprint>.json
.handbook/evidence/artifacts/<instance-id>/intake-values/value_<fingerprint>.json
.handbook/evidence/artifacts/<instance-id>/content/content_<fingerprint>.json
.handbook/evidence/artifacts/<instance-id>/validation-results/validation_<fingerprint>.json
.handbook/evidence/artifacts/<instance-id>/candidates/candidate_<fingerprint>.json
.handbook/state/artifacts/<instance-id>/promotion-records/promotion_<fingerprint>.json
.handbook/state/transactions/intake-records/intake_record_append_<fingerprint>.pending/
.handbook/state/transactions/intake-records/intake_record_append_<fingerprint>.committed/
.handbook/state/transactions/artifact-candidates/artifact_candidate_append_<fingerprint>.pending/
.handbook/state/transactions/artifact-candidates/artifact_candidate_append_<fingerprint>.committed/
.handbook/state/transactions/artifact-promotions/artifact_candidate_promote_<fingerprint>.pending/
.handbook/state/transactions/artifact-promotions/artifact_candidate_promote_<fingerprint>.committed/
.handbook/state/idempotency/generic-artifact-operations/
.handbook/state/locks/generic-artifact-operations.lock
```

Exact record identities, subordinate-closure status, canonical write sets,
authoritative-output/future-receipt cardinality, direct-CLI replay,
transaction-ID derivation, closed
intent/verified/marker/evidence/result fields, the atomic establishment boundary,
the refusal-prefix recovery table, deterministic inventory, count/byte/depth
ceilings, lock order, and the total commit power-set recovery table are normative in
[`contracts/generic-artifact-runtime-contract-v1.0.md`](contracts/generic-artifact-runtime-contract-v1.0.md#journal-identities-and-bounded-inventory).
The JSON Schema and positive vectors independently freeze the record bytes,
fingerprint exclusions, IDs, refs, and exact canonical YAML.

Every generic read or write takes the single repository-scoped generic-artifact
lock, recovers first, retains handles/metadata across inventory/open/decode/hash,
and validates the exact current operation context. The established order is
repository identity/authority locks, HCM-2.2 recovery locks, then this generic
lock. Reverse acquisition or shared Charter mutation is a stop.

Intake and candidate append each realize exactly one authoritative semantic-
record output; promotion realizes canonical truth plus one promotion semantic
record in one atomic group. The HCM-2.3 result reports those outputs and a
separate closed internal-evidence record, not a canonical `WriteReceipt`.
Phase 4 may later map one receipt per authoritative output using only the
canonical receipt fields; it cannot place closure refs or local transaction
fields in those bytes. Closure objects have no standalone list/read operation,
receipt, authority, adoption, deletion, or visibility without a committed
citing record.

Commit recovery is total over every installed subset of intake values plus intake,
all eight subsets of content/result/candidate, and all four subsets of canonical
YAML/promotion. Exact verified staging completes only the missing complement;
when the installed set is already complete, missing/corrupt staging is no longer
authority and recovery may derive the closed `installed_complete` verification
basis. An exact marker with an incomplete installed set is invalid. Exact
marker-backed pending state deterministically completes evidence/result/ledger
and renames to `.committed`; committed replay verifies installed bytes and the
retained ledger even if staging is absent/corrupt. Invalid/missing intent,
incomplete staging with an incomplete installed set, invalid marker, mismatched/
excess/unsafe inventory, or observed mutation refuses without further write.
Repeated restart is idempotent. Committed journals are retained under explicit
limits with no deletion/compaction promise. One committed intake without a
later candidate is valid history, not a partial transaction or canonical truth.

Established-refusal recovery is separately total over the only legal ordered
prefix: one established intent and its active-hold projection, verified refusal decision, refused marker,
internal evidence, refused result, retained-result ledger, and committed rename.
Every layer repeats the identical refusal and exact operation/transaction/
request/context fingerprint chain. It has zero staged, subordinate, semantic,
or canonical artifact outputs, but its result must bind non-null exact internal
evidence. A crash resumes only the missing suffix. Missing-middle, mismatched,
output-bearing, duplicate pending/committed, stale, or conflicting state refuses
without mutation. Retained refusal replays exact; tombstoning is a later locked
compare-and-write that preserves the journal and never re-evaluates the refusal.

Stale basis, profile/kind/schema/intake/descriptor/context drift, ABA change,
changed canonical bytes, conflicting transaction, or concurrent different
request loses by typed conflict. Equivalent concurrent requests yield one
commit and replay; a same key and different request conflicts. There is no
last-writer-wins, dual-read, fallback, migration, silent upgrade, pre-existing
state adoption, authority-record deletion, or automatic cleanup promise.

## Product real-path proof

One CLI integration scenario must create the complete fixture repository and
run the actual built `handbook` binary through this sequence:

1. unchanged setup/identity behavior initializes or preserves the durable
   repository identity without authoring the custom artifact;
2. `artifact list-kinds` and `list-instances` resolve the repository selection
   and return the exact custom kind/instance without a shipped definition;
3. `artifact validate` selects `registry_brief`, safe-reads its descriptor path,
   parses canonical YAML, and validates through
   `example.schemas.registry-brief@1.0.0`;
4. each `artifact intake-evaluate` mode receives equivalent supplied values,
   selects `example.intake.registry-brief@1.0.0`, and produces equal normalized-
   content previews with zero filesystem, journal, key, evidence, or receipt writes;
5. one `artifact intake-append` mutation independently re-evaluates the supplied
   inputs and commits its value closure plus finalized intake `1.2` under exactly
   one authoritative semantic-record output and internal transaction evidence;
6. `artifact candidate-validate` reads that committed intake, produces exact
   validation/content/candidate previews with zero writes, and returns the
   expected candidate fingerprint;
7. `artifact candidate-append` receives only the committed intake pair and
   expected candidate fingerprint, independently revalidates, and commits
   content/result closure plus candidate `1.4` under one authoritative semantic-
   record output and internal transaction evidence;
8. the retained candidate is promoted with the exact expected-current
   fingerprint, creating canonical truth plus promotion `1.2` and exactly two
   authoritative outputs; and
9. a fresh process repeats `artifact validate`, replays each retained mutation,
   and proves the promoted canonical bytes and lineage after restart.

The scenario terminates and resumes after every numbered operation and at every
frozen transaction fault point. It proves validation cannot append, append
cannot accept an unvalidated or preview-mismatched candidate, and a committed
intake without a candidate is valid immutable history rather than partial state.

The integration test captures parsed CLI arguments, engine request, resolved
operation context, record fields, and result envelope to assert the same exact
kind ref and instance ID throughout. A direct registry unit test, fixture-only
call, generated command, per-kind adapter, or invocation that bypasses the
actual CLI binary is insufficient for `RealPathAdopted`.

## Test-first proof contract

Every increment is RED -> GREEN -> REFACTOR. Before an existing symbol is
edited, refresh GitNexus and run upstream impact analysis. Planning-time live
results identify these CRITICAL stop/review surfaces:

| Symbol | Planning-time upstream result | Later implementation rule |
|---|---:|---|
| `load_artifact_kind_registry` | CRITICAL; 385 impacted, 1 direct, 35 processes, 20 modules | prefer consume unchanged; stop before edit |
| `load_artifact_kind_registry_admitted` | CRITICAL; 357 impacted, 1 direct, 35 processes, 20 modules | prefer consume unchanged; stop before edit |
| `SchemaRegistry::load_with_request_budget` | CRITICAL; 7 impacted, 1 direct, 30 processes, 20 modules | consume unchanged unless separately reviewed |
| `ArtifactInstanceRegistry::resolve` | CRITICAL; 427 impacted, 8 direct, 37 processes, 20 modules | expected narrow intake-selection change; warn and re-confirm exact blast radius before edit |
| `resolve_profile_selection` | CRITICAL; 458 impacted, 21 direct, 31 processes, 20 modules | consume unchanged; expansion is a stop |
| `ResolvedArtifactRegistry::validate_json` | CRITICAL; 351 impacted, 6 direct, 38 processes, 20 modules | consume unchanged; no duplicate validator |

The only anticipated CRITICAL edit is the narrow removal of the blanket
non-Charter `intake_definition_ref` rejection in
`ArtifactInstanceRegistry::resolve`, while retaining rejection of lifecycle,
renderer, Projection, and overlay dependencies and retaining the exact frozen
Charter `1.1` dependency closure. The new operation-context resolver must close
the admitted intake ref before content use. If live analysis requires any other
CRITICAL edit, or widens the listed processes, stop for explicit review.

Required positive, negative, and attack proof includes:

- exact custom source permutations, fingerprint replay, and installed-default
  non-membership;
- safe local schema closure and valid/invalid canonical YAML;
- descriptor-selected kind/intake/path and request mismatch refusal;
- null-intake not-applicable behavior;
- all three intake modes, coverage provenance, equal content, retained records,
  non-governed promotion, and restart replay;
- missing/duplicate/unknown/contradicted/default/inferred/wrong-source/low-
  confidence/low-specificity coverage;
- approval injection, Charter record/version/authority substitution, cross-kind
  candidate, stale context/profile/schema/kind/intake/canonical fingerprints;
- duplicate/conflict/missing/unsafe/escape/backslash/symlink/non-regular/size/
  count/depth/cycle/remote-ref/executable-hook/mutation source attacks;
- malformed/multi-document/duplicate-key YAML, structural errors, ambiguous
  serialization, and emitted parse inequality;
- intake/promotion commit crash points and proper subsets; every established-
  refusal prefix and crash boundary; pre-establishment zero-write cases;
  output-bearing/missing-evidence/mismatched refusal states; orphan/partial/
  extra/ambiguous state; retained committed/refused replay; tombstone exact/
  different requests; same/different-request races; ABA; stale writer; and replay
  after fresh processes; and
- proof that no operation generates a command, branches on filename/kind enum,
  changes shipped catalog/package assets, or enters HCM-2.2 Charter paths.

## Commands and verification wall

The later implementation records exact command/result evidence for at least:

```powershell
cargo fmt --all -- --check
cargo clippy -p handbook-engine -p handbook-cli --all-targets -- -D warnings
cargo test -p handbook-engine
cargo test -p handbook-cli
cargo test --workspace
cargo tree -p handbook-engine -e features
cargo package -p handbook-engine --allow-dirty --no-verify
cargo package -p handbook-cli --allow-dirty --no-verify
python tools/check_archive_boundary.py --self-test
python tools/check_archive_boundary.py
python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
git diff --check
npx gitnexus detect-changes
```

Use the repository's actual supported Python launcher. Validate every added
JSON/JSONL/schema/vector artifact with duplicate-key rejection and its declared
schema. Run Markdown link/anchor/fence/reference checks, fingerprint replay,
definition/package manifest equality, exact allowed-path/scope checks, archive
boundary, whitespace, all HCM-1.1 through HCM-2.2 focused regressions, native
Windows mutation/refusal proof, and the full workspace wall. GitNexus change
detection must show only expected implementation symbols and flows before each
commit.

## Expected implementation structure and style

Expected new areas, subject to fresh graph analysis, are focused engine modules
for the intake registry, operation context/service, generic lineage store, and
deterministic YAML; a focused generic CLI adapter; and one HCM-2.3 fixture plus
engine/CLI integration tests. Expected narrow existing areas are
`artifact_instance.rs`, `lib.rs`, and CLI command registration. No Cargo change
is expected. The implementation plan names the bounded file areas; it does not
authorize an edit before live impact analysis.

Public APIs use typed closed request/result/error values, exact refs rather than
strings where existing types apply, deterministic `BTreeMap`/`BTreeSet`
ordering, no panics for repository input, and no semantic string parsing in the
CLI. New behavior is factored by owner and tested through public behavior; no
kind-specific conditional may appear in generic modules.

## Compatibility and non-goals

Always preserve completed HCM-2.2 candidate `1.3`, result `1.0`, intent `1.2`,
Charter definition/lineage/promotion/recovery, and exact shipped profile/catalog
bytes. Preserve HCM-1.1/HCM-1.3 custom proof bytes and HCM-2.1 Project Context.

HCM-2.3 does not authorize:

- HCM-2.4, remaining canonical-YAML families, bridge deletion, or Phase 3+;
- a generic capitalized Projection engine, renderer catalog, lifecycle engine,
  Context Resolution, Snapshot Memory, posture, contract, gate, or dock work;
- generated/dynamic commands, filename switches, new enum variants, per-kind
  adapters, remote schema fetch, executable definitions, or ambient discovery;
- Charter-specific candidate `1.3`, constitutional approval, waiver, lifecycle,
  promotion authority, or schema rules in the generic path;
- Tauri, Substrate, SDK publication, public transport/API bootstrap, complete
  CLI JSON parity, or a broad SDK program;
- changing the HCM-0.6 shipped default catalog, package-owned example kind, or
  automatic setup/doctor/flow selection of `registry_brief`; or
- dual-read, migration, silent upgrade, legacy adoption, fallback, deletion, or
  compatibility promises for unjournaled generic state.

## Proof gates and classification ceiling

HCM-2.3 may support exactly these proof subsets after full proof and CLEAN final
review:

- `PG-KIND-01`: add evidence for exact intake-definition compatibility and one
  generic custom-kind operation context; leave semantic behavior for capable
  kinds, lifecycle, renderer/Projection, and program-wide closure open;
- `PG-KIND-02`: close the gate only for the exact repository-defined
  capability-free `registry-brief` kind/instance with supplied generic intake,
  canonical validation, persistence, and real CLI proof; no first-party or
  arbitrary extension ecosystem claim follows; and
- `PG-ARTIFACT-01`: add evidence for one descriptor-selected custom instance's
  generic read/validate/intake/promote participation; leave remaining artifact
  families, lifecycle, Projection, setup/doctor/flow adoption, and program-wide
  closure open.

The single maximum seam-classification change is one atomic HCM-2.3 change set
containing exactly two subset-cell promotions and no others:

1. `Artifact kind/schema registry`: the exact repository-defined
   `registry-brief` schema/kind/instance real path moves `TargetOnly ->
   RealPathAdopted`; and
2. `Charter intake coverage`: the exact `registry-brief` repository intake
   definition/evaluation/lineage real path moves `TargetOnly ->
   RealPathAdopted`, while the Charter subset remains
   `ContractCorrectAndProven` and every broader generic/custom intake remains
   `TargetOnly`.

Neither cell implies a generic ecosystem or first-party catalog claim. No other
seam row or classification cell may move. In particular,
`ContractCorrectAndProven` is unavailable for the broader generic system,
`PG-YAML-02` remains open program-wide, and no SDK/JSON/publication gate moves.
The implementation proof must assert this exact two-cell crosswalk diff and that all
other classification text is byte-unchanged except evidence prose naming this
bounded result.

## Exit gate

Implementation authority is sufficiently frozen only when this packet:

- names every exact proof identity, source, owner, operation, persistence path,
  refusal, currentness rule, and test class above;
- contains an ordered test-first plan and executable todo;
- reconciles Phase 4 without claiming an SDK or transport program;
- repairs only factually stale HCM-2.2 control-pack status;
- passes documentation/fingerprint/scope/archive/handoff/change-detection gates;
- receives a fresh isolated complete-subject review, remediation of every valid
  finding, and a different fresh CLEAN re-review; and
- lands as one reviewed planning commit followed by one parent-owned handoff/
  ledger-only closeout commit that still says implementation is unauthorized.

## Stop conditions

Stop and escalate without implementation authority if:

- a semantic/product decision cannot be derived from canonical pack authority;
- generic CLI/owner operations conflict irreconcilably with Phase 4 ownership;
- intake identity, record graph, lock order, or transaction graph cannot remain
  explicit, deterministic, secure, and acyclic;
- real-path proof requires generated commands, filename dispatch, a Rust enum
  variant, executable hooks, remote fetch, or a generic Projection engine;
- implementation needs to change shipped default bytes, HCM-2.2 authority,
  Cargo/dependencies, unexpected CRITICAL symbols, HCM-2.4, or Phase 3+;
- native-platform fail-closed persistence or complete product-path proof is not
  available;
- the complete planning or later implementation subject cannot reach CLEAN; or
- mandatory built-in delegation/review is unavailable.
