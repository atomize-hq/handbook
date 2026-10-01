# HCM semantic proposals before consolidation

Historical source snapshot, 2026-09-24. These are the exact seven reconciled
proposal texts after the local and independent Pro corrections, before their
responsibility-based consolidation. The embedded Status labels describe that
historical snapshot; they are not another active proposal set. Current review
starts at the [combined package](../specs/handbook-contract-membrane/semantic-model/README.md).

Each UTF-8 source is reproduced verbatim inside a fence. The SHA-256 binds the
original bytes including its final newline. This preserves uncommitted draft
history without retaining competing editable normative copies at the old paths.

## canonical-object-ontology-and-ownership-semantic-decision.md

SHA-256: `2b35e9501feac21b22aeb5c6d8ae6d0fd6adfe4b72005f0f324b7301ff124517`

~~~~~~~~~~~~markdown
# HCM canonical-object ontology and ownership semantic decision

**Status: PROPOSED**

This record fixes the greenfield ontology and semantic ownership boundary for
the canonical objects, source provenance, projections, representations, and
operational state used by the Handbook Contract Membrane. It determines which
objects participate in Handbook's generic Artifact Kind / Artifact Instance
lifecycle and which remain under dedicated semantic owners.

It is a narrow companion to the proposed
[posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
Those records retain ownership of their detailed semantics. This record places
their important objects into one ontology and responsibility map without
redefining their stage order, algorithms, state machines, or truth rules.

This record is semantic authority for human review only. It is not
implementation authority, a slice or packet specification, a persistence
design, or rollout planning. Current source is evidence for ownership and
dependency direction, not a compatibility obligation. The product is
greenfield; no V1 or legacy model, alias, migration, bridge, or dual-write
behavior is selected.

## Purpose and scope

Handbook needs one precise answer to three easily conflated questions:

1. What kind of object is this?
2. Which semantic owner establishes the proposition the object asserts?
3. Which repository, projection, or transport may carry it without acquiring
   that authority?

This record settles those questions at the semantic-responsibility level. It
does not require all canonical objects to share one lifecycle or repository.

The canonical project-artifact families are exactly:

- **Project Authority / Charter**;
- **Project Context**;
- **Environment Context**;
- **Work Specification**;
- **Decision Record**; and
- **Risk Record**.

`Work Specification` is the canonical greenfield family name. `FeatureSpec`
does not name a seventh family and creates no compatibility requirement.
Whether a future model discovers a genuinely different artifact role remains a
new semantic decision rather than an alias inferred here.

This record classifies but does not redefine the detailed semantics fixed by
three exact companion proposals:

- the proposed
  [evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md)
  owns evaluation-occurrence identity, evidence-set formation and closure,
  prospective applicability, semantic times, and historical resolvability;
- the proposed
  [authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md)
  owns exact relationships among the six authored artifact families and owner
  definitions, records, results, and provenance; and
- the proposed
  [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
  owns present-use reliance, atomic eligibility/disposition, local closeout,
  parent eligibility, action authorization, attempts, terminals, and mutation
  receipts.

All remain `PROPOSED`. Their presence supplies exact companion references, not
human approval, implementation authority, physical placement, or rollout.

## Canonical vocabulary

The following terms are normative. Terms in separate rows are not synonyms.

| Term | Fixed meaning |
| --- | --- |
| **Semantic owner** | The bounded-context responsibility that alone establishes, rejects, verifies, or changes the exact proposition represented by an object. Storage, rendering, transport, or orchestration does not transfer this authority. |
| **Canonical object** | An exact object whose semantic owner recognizes its identity and content as authoritative for one bounded proposition. Canonicality is owner- and proposition-scoped; it does not mean “stored in the generic artifact repository,” “currently applicable,” “globally trusted,” or “safe for every use.” |
| **Authored canonical project artifact** | Human- or governance-authored project truth governed through the generic Artifact Kind / Artifact Instance lifecycle, including candidate formation, validation, promotion, exact revision, authorized mutation, and lineage. Only the six named families in this record have this status. |
| **Canonical semantic definition or policy** | An immutable, named, versioned rule or vocabulary definition admitted by the owner whose semantics it controls. It defines how later inputs are resolved, admitted, evaluated, or interpreted; it is not a result of applying itself. |
| **Policy Admission Closure** | The one immutable owner-produced canonical record that closes admission or refusal of one exact policy candidate under one exact owner admission-rule version. It binds exact candidate content; typed vocabulary and conformance; finite candidate selection and accounting; every relevant exact governance-source revision with proposition role, scope, and historical, formation-only, or continuing-current dependency mode; any separately typed and authorized approval or exception source required by that rule; and all non-relaxable invariant checks. The name **Closure** is selected because one exact category-3 owner result closes both basis and outcome without mandating separate `Policy Admission Basis` and `Policy Admission Result` runtime object types. |
| **Owner-produced canonical record or result** | An immutable determination, closure, verification, admission, disposition, or receipt produced by applying one semantic owner's rules to exact inputs. It remains owned by that domain even when another domain consumes it. |
| **Durable origin, attempt, or source provenance** | An immutable occurrence record of what an origin owner requested, attempted, observed, reported, applied, refused, or completed. It may be canonical for that bounded occurrence assertion while remaining untrusted or insufficient for a downstream semantic proposition. |
| **Deterministic projection** | A purpose-, audience-, disclosure-, and definition-bound view of exact source truth. It may select, redact, collapse, render, or assemble while recording its source identities, omissions, lossiness, and output identity. It is authoritative only about that projection operation, not the source proposition. |
| **Representation** | A Rust DTO, canonical JSON value, CLI text, Tauri payload, schema-valid message, or other encoding that carries an owner object or projection. Encoding identity and transport correlation do not create semantic owner identity or authority. |
| **Deterministic assembly** | A projection that combines multiple exact source objects or projections and owns only the selection, ordering, omission, budget, and assembly accounting. A Flow packet is the principal current example. |
| **Transient operational state** | Live execution state that coordinates work but has not been established as a durable owner assertion: for example a queued launch, open process handle, in-memory scheduler state, or transport-local failure. Logging it does not by itself promote it to canonical provenance. |
| **Generic Artifact Kind / Artifact Instance machinery** | Engine-owned definitions and lifecycle behavior for the six authored canonical project-artifact families. It is not a universal registry or repository for every canonical object. |
| **Owner repository** | The semantic owner's durable boundary for definitions, records, provenance, or results under that owner's identity, admission, lifecycle, currentness, and replay rules. Physical storage may be shared without merging semantic ownership. |
| **Cross-owner catalog** | A non-authoritative index or lineage projection that lets consumers discover heterogeneous objects by exact owner, category, identity, and relation. Catalog inclusion does not make an object an Artifact Instance or transfer authority. |

### Orthogonal properties

The following properties MUST be modeled independently:

| Property | Question it answers | It does not establish |
| --- | --- | --- |
| **Persistence** | Is the object retained durably or only live for an operation? | Canonicality, authority, authored-artifact status, or current applicability. |
| **Authority** | Which owner can establish the object's bounded proposition? | That every field or referenced object shares that authority. |
| **Canonicality** | Has that owner established the exact identity and content for the bounded proposition? | Generic artifact membership, present currentness, downstream admissibility, or operational permission. |
| **Authored-artifact status** | Does the object participate in the generic project-artifact authoring, candidate, validation, promotion, mutation, and lineage lifecycle? | Executable policy admission, evaluation truth, or authority outside that lifecycle. |
| **Projection identity** | Which exact source closure, projection definition, purpose, audience, disclosure bound, omissions, and output produced this view? | A replacement identity for any projected source. |
| **Transport representation** | Which encoding, schema, framing, or correlation carried an object or projection? | The object's semantic identity, trust, admission, or owner decision. |

A durable object can be noncanonical. A canonical object can be historical and
non-current. A source observation can be canonical for what a producer
reported yet remain inadmissible as evidence. A projection can be
content-addressed without becoming project truth. These distinctions are
required, not exceptional.

The ontology has exactly six top-level categories:

| Category | Native semantic role |
| --- | --- |
| **1. Authored canonical project artifacts** | Governed project truth authored through Handbook's project-artifact lifecycle. |
| **2. Canonical semantic definitions and policies** | Immutable owner-admitted rules that define later resolution, admission, evaluation, or action semantics. |
| **3. Owner-produced canonical records and results** | Immutable owner determinations produced from exact input closures. |
| **4. Durable origin, attempt, and source provenance** | Retained occurrence truth from requesters, producers, Docks, hosts, or other origins, without implied downstream admission. |
| **5. Deterministic projections, representations, and assemblies** | Purpose-bound views, encodings, and compositions that preserve source ownership without becoming source truth. |
| **6. Transient operational state** | Live coordination state that has not been closed as a durable owner assertion. |

Within category 5, a projection has a semantic view identity, a deterministic
assembly adds multi-source selection and composition accounting, and a
representation supplies an encoding or framing. Those are subordinate forms,
not additional top-level categories. A live, unclosed Evaluation Authorization
Request, evidence candidate, or other boundary submission is category 6. When
an origin owner closes and retains what occurred, it creates a distinct
category-4 provenance record that refers to the submission; it does not change
the submission's category. Downstream owner admission may separately create a
category-3 result. The original submission, origin-closed occurrence record,
and admission result remain three distinct objects and identities.

## Current product boundary

Current Engine definitions register all six authored artifact families. The
shipped-root profile currently instantiates only Project Authority / Charter,
Project Context, and optional Environment Context. Its instance descriptors
bind canonical project paths, requiredness, intake, renderer references, and
the available authored-artifact lifecycle. The profile currently selects no
projection catalog, and those three instances select renderers rather than
projection definitions.

Current projection source demonstrates a separate identity closure over exact
sources, definition, purpose, surface, Context Resolution authority,
currentness, disclosure decisions, omissions, lossiness, derivations, and
output. That is evidence for the source/view separation fixed here; it does not
establish a universal projection engine or require every owner to adopt the
current private representation.

## Normative ontology invariants

- Every semantically meaningful object MUST have exactly one semantic owner for
  each proposition it asserts. A composite object MUST preserve the distinct
  owners and exact identities of its constituent propositions.
- Canonicality MUST be scoped to an owner and proposition. No unqualified
  global `canonical` flag may imply authority across bounded contexts.
- Persistence, content addressing, versioning, project-local storage,
  human-readability, audit value, and discoverability MUST NOT independently
  confer authored-artifact status.
- The generic Artifact Kind / Artifact Instance lifecycle MUST remain limited
  to the six authored canonical project-artifact families named above.
- Canonical semantic definitions, owner-produced records, and durable source
  provenance MUST use their semantic owner's admission and lifecycle rules.
  They MUST NOT acquire candidate/promotion/mutation semantics merely to reuse
  the generic artifact repository.
- Shared canonicalization, fingerprint, exact-reference, repository-support,
  lineage, or pagination primitives MAY be reused across owners. Reusing a
  primitive MUST NOT merge repositories, lifecycle meanings, or authority.
- A user, agent, authored artifact, or another tool MAY author policy-candidate
  content. Candidate authorship supplies proposed content, provenance, and
  organizational judgment only. The semantic owner MUST independently and
  deterministically produce a Policy Admission Closure before the candidate is a
  category-2 definition or policy. Artifact promotion, prompt authority, agent
  role, a bare Markdown link, and human-readable approval prose alone never
  perform that admission.
- A Policy Admission Closure MUST bind only the exact governance sources relevant
  to the candidate's declared admission rule, not every artifact family by
  default. It MUST bind each consumed Charter, Project Context, Decision Record,
  or Risk Record revision by exact owner identity and revision, proposition role,
  scope, and currentness/dependency mode. When the admission rule requires an
  approval or exception, the closure MUST separately bind the exact typed,
  authorized source, its authority, scope, identity/version or revision as
  applicable, and currentness. Authored text remains governance input and MUST
  NOT be reclassified as runtime evidence or a waiver.
- Policy admission MUST apply typed vocabulary, complete finite candidate
  selection/accounting, scope and role completeness, and non-relaxable invariants
  mechanically. “Unsafe” means prohibited by one of those admitted constraints;
  it MUST NOT mean an AI's unbounded organizational judgment. The owner admission
  rule is an exact governance root outside the candidate population it decides,
  so a candidate cannot select or recursively admit its own admission rule.
- An owner-produced record MUST remain immutable historical truth for its exact
  input closure. Currentness, supersession, or current-use eligibility is a
  separate owner observation or relationship; it MUST NOT rewrite the record.
- Durable origin provenance states only what its origin owner can establish.
  Downstream admission MUST independently decide whether it can support a
  different proposition.
- A transient state becomes durable provenance only when the appropriate owner
  closes an exact occurrence record. A log line, process state, or transport
  error is not promoted implicitly.
- A projection MUST preserve exact source-owner attribution, exact source
  identity, projection-definition identity, and all material omission,
  lossiness, disclosure, compaction, and budget accounting.
- A projection or representation MUST NOT be fed back as source-owner truth.
  A consumer MUST either resolve the referenced source from its owner or submit
  the projection through a separately declared source/admission policy.
- SDK, CLI, Tauri, Flow, Pipeline, a Dock, a host, a schema validator, or a
  skill MUST NOT acquire semantic authority merely because it stores,
  orchestrates, validates, renders, or transports an owner object.
- Physical colocation of multiple responsibilities MAY be selected later.
  Their semantic owners, identities, decisions, and refusal boundaries MUST
  remain independently attributable.
- Ontology category alone MUST NOT select persistence or retention. Category 4
  is durable by definition and category 6 is not; retention and historical
  resolvability for category-2, category-3, and category-5 objects remain
  owner requirements or later decisions. When an object is persisted, its
  repository MUST preserve rather than replace its semantic owner and native
  lifecycle.

## Object classification

The table classifies the important objects already named by the companion
decisions. It does not redefine their detailed fields or behavior.

| Objects | Ontology category | Semantic owner and boundary |
| --- | --- | --- |
| Project Authority / Charter; Project Context; Environment Context; Work Specification; Decision Record; Risk Record | Authored canonical project artifacts | Engine's artifact/profile domain owns generic kind, instance, lifecycle, revision, validation, promotion, mutation, and lineage semantics; project authors and authorized governance supply the content. |
| Artifact Kind definitions; Artifact Instance Descriptors; profiles; renderer and projection definitions; Context Resolution definitions | Canonical semantic definitions or policies | Engine admits and resolves its system definitions. The descriptor defines an artifact instance; it is not itself that authored artifact. |
| Contract revisions; claim definitions and inventories; claim-set definitions; Evidence Evaluation Policies; Metric Definitions; Gate Policies | Canonical semantic definitions or policies | Contracts definition authorities own executable meaning and exact resolution. |
| Assurance Policy Selector; Assurance Policies; policy mappings; typed exception policies | Canonical semantic definitions or policies | Contracts-domain assurance-governance definition authorities own admission and currentness of their executable definitions, mappings, and candidate inputs. Assurance Selection resolves and applies only those admitted inputs; it cannot admit or authorize its own inputs. |
| Producer Semantic Capability definitions | Canonical semantic definitions or policies | A distinct Producer Capability Authority within the broader Contracts bounded context independently admits the semantic representational capability used for proof-plan compatibility. A producer or Dock cannot self-admit it. |
| Dock or adapter operating-ceiling definitions and producer retry/partial-completion policies | Canonical semantic definitions or policies | The Dock or adapter owner establishes its bounded operating contract. That contract is not Engine authorization, Producer Semantic Capability, or host enforcement. |
| Action Kind definitions; Operational Reliance, Local Closeout, Parent Eligibility, Action Authorization, posture-observation, posture-recommendation, workflow, recovery, release, or deployment policies | Canonical semantic definitions or policies | The responsible definition or downstream action domain owns each exact admitted definition. This record selects no universal enforcement owner and no policy becomes admitted merely because a user, agent, artifact, or transport authored it. |
| Live, unclosed policy-candidate submission | Transient operational state | It is proposed content at an admission boundary. Candidate authorship or transport does not create a category-2 definition. |
| Origin-closed policy-candidate submission occurrence | Durable origin, attempt, or source provenance | The user, agent, artifact, tool, or adapter origin owns the exact historical assertion that the candidate was submitted. It is not policy admission. |
| Policy Admission Closure | Owner-produced canonical record or result | The exact definition/admission owner deterministically closes admission or refusal under one exact owner rule version, with typed conformance, finite candidate accounting, relevant exact governance-source roles/scope/currentness, required typed approvals or exceptions, and non-relaxable invariant checks. It is one category-3 closure rather than a required pair of new runtime object types. |
| Private `ContextResolutionEnvelope`, Resolved Profile, exact stack, and Engine authority-admission lineage | Owner-produced canonical records | Engine owns private authority truth and exposes only bounded public projections or opaque resolvable references. |
| Posture Scope; Resolved Posture Snapshot; posture-transition result and resulting authority state | Owner-produced canonical records or results | Engine posture responsibilities own scoped posture resolution and separately authorized mutation truth. |
| Exact selection-basis closure; policy-resolution basis; Resolved Assurance Requirements | Owner-produced canonical records or results | Contracts-domain Assurance Selection closes the exact admitted candidate set and selection inputs, owns complete accounting, and produces the exact selection outcome. It does not establish governance admission or currentness for the definitions and mappings it consumes. |
| Admitted exception input | Owner-produced canonical record or result | A distinct Contracts-domain exception-admission responsibility establishes exact policy conformance, authority, identity, scope, and currentness. The exact issuer or governance authority authorizes the source record; Assurance Selection only applies an already admitted bounded effect. |
| Observation Plan; Producer Selections; Evaluation Purpose, including its bound Evaluation Use Intent; Readiness Basis and Assessment; Readiness Refusal; Attempt Disposition; Prospective Evaluation Basis | Owner-produced canonical records or results | Contracts-domain Evaluation Coordination owns these pre-truth closures and results. |
| Evaluation Authorization Binding; authorization refusal; Authorization Verification Observation; supersession relation | Owner-produced canonical records or results | Engine alone owns issuance, refusal, verification, historical validity, currentness, and supersession truth. |
| Canonical evidence record; canonical evidence-set closure; final evaluation context; applicability result; claim evaluation; metric result; semantic gate decision; canonical explanation | Owner-produced canonical records or results | Contracts evidence and evaluation responsibilities own admission, closure, evaluation truth, and complete canonical decision data. |
| Action Target Snapshot; Operational Reliance Basis; atomic Operational Reliance Determination with Current-Use Eligibility and Operational Disposition facets; Reliance Refusal | Owner-produced canonical records or results | The authoritative target/state owner establishes target truth; Work Progression or another exact downstream action domain owns reliance basis, determination, refusal, currentness, and replay. Eligibility and disposition are facets of one Determination, not independently current records. |
| Action Authorization Binding; Action Authorization Refusal | Owner-produced canonical records or results | The exact action-authority owner produces the semantic authorization result and provenance closure. A Binding may preserve advisory claims when policy permits, but it is not by itself a credential, authenticated identity, verified delegation, capability token, or host-enforcement proof. |
| Mutation Receipt | Owner-produced canonical record or result | The authoritative state or effect owner alone proves that one exact authorized Action Attempt committed the exact predecessor-to-successor mutation. |
| Closed Action Intent occurrence; Action Attempt Identity and durable attempt occurrence; Action Terminal | Durable origin, attempt, or source provenance | The requester owns the exact Intent occurrence; the executor owns attempt identity, attempt occurrence, and Terminal. None substitutes for reliance, authorization, target truth, or a state-owner Mutation Receipt. |
| Live, unclosed Action Intent request or in-progress Action Attempt | Transient operational state | Requesters, executors, and transports coordinate live work only. Closing durable provenance or an owner result creates a distinct object rather than reclassifying the live state. |
| Posture Observation and Posture Recommendation | Owner-produced canonical records or results | Their separately named owners establish non-mutating admitted observation and recommendation propositions; neither owns Engine transition authority. |
| Live, unclosed Evaluation Authorization Request | Transient operational state | The requester states what it asks Engine to admit. An origin-closed request-occurrence record is separate durable provenance, while Engine's Binding or refusal is a separate owner-produced result. Neither reclassifies the live request or makes it authorization. |
| Origin-closed request- or submission-occurrence record | Durable origin, attempt, or source provenance | The requester, producer, or adapter owns the exact historical assertion that a particular request or submission occurred. It refers to but does not reclassify the live submission, and it is not the receiving owner's admission result. |
| Producer Attempt; source observation; Actual Observed Reach; Dock terminal; Host Enforcement Fact | Durable origin, attempt, or source provenance | Producer, Dock, or host owns only its exact occurrence assertion. Attempts and observations remain bound to one exact Producer Selection and Prospective Evaluation Basis. |
| Live, unclosed evidence candidate | Transient operational state | It is an untrusted boundary submission. A producer- or adapter-closed submission-occurrence record is separate durable provenance; Contracts admission may create a separate canonical evidence record. Neither later object reclassifies the candidate. |
| Canonical Charter, Project Context, or Environment Context public view; owner public record view; grounding projection; explanation-density view; bounded, reorganized, redacted, or summarized Markdown view | Deterministic projection | The projection owner establishes selection, disclosure, omission, lossiness, and output identity. The exact source owner retains source truth. |
| Flow packet | Deterministic assembly | Flow owns packet selection, ordering, omissions, budget, and assembly identity; included sources retain their owners. |
| SDK DTO; CLI JSON or text; Tauri payload; Dock protocol message; Markdown encoding | Representation | The owner-defined public object or projection supplies semantic meaning. Schema and adapter owners establish only encoding and framing claims. When a Markdown projection is encoded as Markdown, the projection identity and representation identity remain distinct. |
| Queued launch; open process handle; in-progress scheduling state; transport-local error | Transient operational state | The active coordinator or adapter owns live handling only. A later exact terminal or occurrence record is a distinct durable object. |

Project Classification, Posture Level, Posture Dimension, Evaluation Use Intent,
Requested Observation Reach, and similar bounded values normally participate in
the identities of their containing definitions or records. They do not require
independent repository status merely because they have typed semantics.

## Generic artifact inclusion and exclusion

An object belongs in the generic Artifact Kind / Artifact Instance machinery
only when all of the following hold:

1. it is project truth intentionally authored or governed through Handbook's
   project-artifact workflow;
2. its native lifecycle is candidate formation, validation, promotion, exact
   revision, project-authorized mutation, and lineage;
3. it is an instance of exactly one of the six declared project-artifact
   families; and
4. promotion establishes only the artifact revision, without pretending to
   perform another semantic owner's executable admission or decision.

An object MUST remain outside that machinery when its native role is any of the
following:

- an immutable named/versioned executable definition admitted by its semantic
  owner;
- an owner-produced derivation, closure, verification, decision, disposition,
  or mutation receipt;
- an occurrence-bound request, attempt, observation, enforcement fact,
  terminal, or candidate;
- a projection, rendering, assembly, cache, index, DTO, or wire encoding; or
- live operational state.

Being durable, project-local, human-edited, content-addressed, schema-validated,
or useful in an audit is not sufficient for inclusion. Conversely, exclusion
from the generic artifact repository does not make an object noncanonical or
ephemeral.

No generic `derived artifact` category is selected. It would collapse authored
revision/promotion, immutable definition resolution, owner-result closure,
occurrence provenance, currentness verification, projection, and transport
lifecycles into one false abstraction. Unified discovery, navigation, and
lineage SHOULD instead use a cross-owner catalog or deterministic audit view
that preserves each object's exact category, semantic owner, identity, and
relationship.

## Semantic responsibility map

The stable dependency direction remains Engine inward of Contracts, and owner
crates inward of SDK and transports. The table assigns semantic
responsibilities, not physical crates, modules, storage engines, or APIs.

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Engine artifact and profile domain | The six Artifact Kind families, Artifact Instance Descriptors, profile conformance, authored artifact lifecycle and repositories, canonical project-artifact reads, and Engine-owned artifact render/projection definitions. | Contract definitions, assurance selection, evaluation readiness, evidence or gate truth, or generic authority over non-artifact owner records. |
| Engine posture domain | Exact Posture Scope resolution, Resolved Posture Snapshot, snapshot verification, and separately authorized posture transition. | Assurance selection, evidence evaluation, gate truth, or transition authority inferred from a recommendation. |
| Engine Context Resolution domain | Complete envelope admission, non-expansion, Evaluation Authorization issuance/refusal, private lineage, verification, historical validity, currentness, and supersession. | Requirements, producer capability, readiness, evidence adequacy, gate truth, host enforcement, or downstream action authority. |
| Definition and policy-admission owners | Exact candidate populations; owner admission-rule versions; deterministic Policy Admission Closures; typed vocabulary/conformance; relevant exact governance-source proposition roles, scopes, and dependency/currentness modes; required typed/authorized approvals or exceptions; non-relaxable invariant checks; admission/refusal, currentness, and supersession. | Treating authorship, prompt/agent role, artifact promotion, bare links, open-ended AI safety judgment, or policy-favorable outcomes as admission; requiring every artifact family; turning authored text into runtime evidence/waiver; or letting a candidate recursively admit its own admission rule. |
| Contracts-domain assurance governance and exception admission | Admission and currentness of Assurance Policy Selector, Assurance Policy, policy-mapping, candidate-policy, and typed exception-policy definitions through exact Policy Admission Closures; independently attributable admission of exact authorized exception records. | Posture resolution, closing the selection basis, selecting or applying requirements, allowing a selector to authorize its own inputs, evidence admission, or Engine authority. |
| Contracts-domain Assurance Selection | Exact resolution of one selector and policy over an Engine-owned Resolved Posture Snapshot and other declared admitted inputs; closure of the exact selection basis; application of only admitted exception effects; policy-resolution basis; Resolved Assurance Requirements; complete candidate and selection accounting. | Definition, mapping, candidate-input, policy, or exception admission/currentness; policy or exception invention; exception authorization; posture resolution or mutation; Engine authorization; capability-driven relaxation; evidence admission; or gate reinterpretation. |
| Contracts-domain Producer Capability Authority | Independent admission and currentness of named/versioned Producer Semantic Capability definitions against Contracts-owned observation roles, scales, dialects, and proof requirements. | Producer self-attestation, Dock ceiling, operational availability, Engine permission, actual reach, evidence sufficiency, or host enforcement. |
| Contracts-domain Evaluation Coordination | Observation Plan and Producer Selection closure, Evaluation Purpose, Readiness Basis and Assessment, Readiness Refusal, Attempt Disposition, and Prospective Evaluation Basis. | Engine authorization issuance, capability self-attestation, evidence admission, claim/metric/gate truth, or host launch by implication. |
| Contracts definition, evidence, and evaluation responsibilities | Contract/claim/evidence-policy/metric/gate definitions; candidate admission; canonical evidence; evidence-set and final-context closure; applicability, claim, metric, gate, and canonical-explanation truth. | Authored artifact mutation, Engine authorization, Dock operation, host enforcement, or downstream action. |
| Dock and adapter domains | Their manifests, operating-ceiling definitions, protocols, exact invocation, attempt terminals, source-value preservation, and candidate construction. | Producer Semantic Capability admission, Engine authorization, readiness, canonical evidence, or semantic gate truth. |
| Producer and enforcing host domains | Producers own exact attempt observations and Actual Observed Reach; the actual host owns exact applied/refused Host Enforcement Facts. | Canonical evidence, Engine authority, proof requirements, readiness reclassification, or invented enforcement. |
| Work Progression and other Operational Reliance domains | Their exact Action Kinds and admitted policies; action-target closure for their decisions; Operational Reliance Occurrences, Bases, atomic Determinations with eligibility/disposition facets, Reliance Refusals, currentness, and replay. | Rewriting gate truth, inheriting Engine evaluation authority, independently authorizing an action, mutating state, or creating a universal enforcement policy by default. |
| Action requester or origin owner | Exact Action Intent content, claimed principal/delegation/authority provenance, and durable request occurrence. | Target truth, Operational Reliance, authorization, attempt, or effect merely because it requested one. |
| Action-authority owner | Exact authorization rule and declared advisory/verified/enforced authority basis; principal/delegation provenance; Action Authorization Binding or Refusal; currentness, supersession, and permitted-attempt rule. | Improving eligibility, fabricating authentication/delegation/enforcement, requiring a general Handbook IAM platform, proving an attempt, or claiming a committed effect. |
| Action executor | Exact Action Attempt Identity, attempt-admission checks, execution, Action Terminal, delivery replay, failure, and effect/commit uncertainty. | Reconstructing another owner's authority/currentness, broadening the effect, retrying an uncertain non-idempotent effect without the admitted recovery conditions, or asserting commit without a state-owner receipt. |
| Authoritative state or effect owner | Exact target snapshots and expected predecessor; through-commit verification; atomic effect or state transition; durability, concurrency, exact successor, commit semantic time, and Mutation Receipt. | Inventing eligibility, broadening authorization, changing policy, or reporting an uncertain commit as success. |
| Flow | Deterministic packet assembly and its selection, omission, ordering, compaction, and budget accounting. | Any included artifact, posture, authorization, evidence, metric, gate, or disposition truth. |
| Pipeline | Pipeline capture, compile, handoff, lifecycle, and separately authorized Pipeline actions. | Contracts truth, generic parent eligibility, posture policy, or mutation merely because a gate passed. |
| SDK | Transport-free composition and owner-defined public projections. | Alternate semantic branches, owner verification, readiness recomputation, or independent DTO meaning. |
| CLI, Tauri, skills, and transports | Input acquisition, lexical validation, framing, correlation, rendering, streams, scheduling, and transport-local errors. | Semantic admission, task-wide enforcement, owner-result reinterpretation, or mutation authority. |
| Schema authorities | Exact structural schema definitions, schema assets, and conformance results for their bounded encoding domain. | Semantic authority, executable-policy admission, capability, evidence trust, or host-enforcement proof. |

Assurance governance and exception admission, Assurance Selection, and
Evaluation Coordination are distinct sibling responsibilities in the broader
Contracts bounded context. Exception record authorization remains attributable
to the exact issuer or governance authority defined by the admitted exception
policy. Governance owners admit and verify currentness of selector, policy,
mapping, candidate, and exception inputs. Assurance Selection consumes those
owner results and produces the distinct exact selection-basis closure,
accounting, and requirements result; it cannot produce the admission or
currentness it consumes. This prevents Engine from depending outward on
Contracts definitions, prevents a selector from authorizing its own inputs,
and prevents readiness or evidence evaluation from feeding capability
limitations back into assurance selection.

Producer Capability Authority is another independently attributable Contracts-
domain responsibility because capability compatibility is stated against
Contracts-owned observation roles, scales, dialects, and proof requirements.
Its admission MUST remain independent from producer self-description and a
Dock's operating ceiling. These semantic placements do not require three
physical crates or repositories; concrete placement remains deferred.

## Identity, lifecycle, and currentness by category

Only the distinctions necessary to preserve the ontology are settled here.

| Category or subordinate form | Identity and lifecycle rule | Currentness rule |
| --- | --- | --- |
| Authored canonical project artifact | Exact kind, instance, project scope, content/revision, and lineage under candidate, validation, promotion, and authorized-mutation rules. | The Engine artifact owner determines which exact revision is current for a named use; historical revisions remain immutable and readable. |
| Canonical semantic definition or policy | Stable owner/name/version plus exact semantic-content identity; immutable after admission. A changed meaning is another definition identity or version. | The definition owner separately verifies whether an exact definition is admitted and applicable at a named semantic time. |
| Owner-produced canonical record or result | Exact owner, record kind, bounded proposition, input closure, semantic time, and content identity. Creation is an owner decision, not artifact promotion. | The record remains historical truth. Currentness, verification, supersession, and current-use eligibility are separate relations or observations. |
| Durable origin, attempt, or source provenance | Exact origin owner, occurrence identity, bound purpose/basis/selection where applicable, semantic time, and reported content. Delivery replay and a new occurrence are distinct according to the origin owner's rules. | Later source validity or downstream eligibility is independently observed; retention does not imply continuing authority or freshness. |
| Deterministic projection or assembly | Exact source-owner identities, source revisions or records, projection/assembly definition, purpose, audience, disclosure bound, resolution/density where applicable, omissions/lossiness, and output identity. | A projection remains exact historical output for its source closure. A current view requires current source resolution and a new projection when declared inputs change. |
| Representation | Exact encoding/schema version and interface-local framing when needed; it carries rather than replaces the semantic identity. | Transport availability or schema conformance is independent of source currentness and semantic admissibility. |
| Transient operational state | Operation-local identity only as needed for coordination. | It expires with the operation unless an owner closes a separate durable provenance record. |

The proposed
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md)
owns exact occurrence identity, evidence-set membership and closure, prospective
applicability, and semantic-time/historical-resolvability rules. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
owns the present-use verification basis. This ontology only classifies their
objects, and neither proposal is human approval or implementation authority.

## Projection and representation rules

Every semantically meaningful deterministic projection MUST bind:

- exact source semantic owner and source identity;
- exact source revision, snapshot, closure, or semantic time as applicable;
- exact projection definition and version;
- purpose, audience, and surface;
- disclosure authority and any Context Resolution, Grounding Projection
  Resolution, or Explanation Density input that actually governs the view;
- explicit omissions, redactions, lossiness, derivations, compaction, and
  budget decisions; and
- exact output identity or fingerprint.

The projection owner may authoritatively state what it selected, omitted,
derived, rendered, or assembled. It MUST NOT establish or change the projected
source proposition.

Markdown is a medium, not an ontology category. A bounded, reorganized,
redacted, or summarized view expressed in Markdown is a projection. Encoding
an exact owner object or projection as Markdown is a representation. When a
projection operation is followed by Markdown encoding, its projection identity
and representation identity MUST remain distinct and MUST both preserve the
exact source-owner attribution.

The full **canonical explanation** produced with Contracts claim, metric, and
gate truth is owner-produced canonical decision data and one canonical source of
evaluation truth. It is not a projection. A concise default human explanation
and any requested deeper drill-down are deterministic explanation-density views
of that same source. Each MUST preserve the exact result identity, semantic
status, decisive reason, owner, and mandatory next-state distinctions; bind its
own disclosure authority and omissions; and expose no more than the highest
source detail both available and authorized for that viewer. Denied or
unavailable detail is a projection/disclosure outcome, not reevaluation.

Display density MUST NOT change inputs, rerun evaluation, create another gate,
or alter truth, authority, or current-use eligibility. These principles are
settled here. The exact number, names, omission contracts, transport shapes, and
interaction design of explanation projections remain bounded later decisions;
this record selects no fixed level count, UI layout, or API.

Context Resolution authorization, Grounding Projection Resolution, viewer
disclosure authority, posture strictness, producer capability, and Explanation
Density remain distinct inputs owned by their respective domains. Sharing a
typed coordinate or level vocabulary does not make them interchangeable.

A representation MAY add transport correlation, framing, stream, exit, or
interface-local error information. Such additions are not fields of the owner
object unless the semantic owner explicitly defines them. Parsing or
round-tripping an SDK DTO, CLI payload, Tauri payload, Dock message, or rendered
document proves only the declared structural and encoding properties. It does
not re-admit the source object or establish owner authority. The semantic
payload mapping MUST be deterministic for exact inputs; adapter-local
correlation, scheduling, or delivery metadata may vary and is excluded from the
source and projection identities unless an owner explicitly binds it.

## Discriminating scenario corpus

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Promoted Charter and resolved posture.** A Charter revision is promoted through the project-artifact lifecycle. Engine resolves a Posture Scope and produces a Resolved Posture Snapshot. | The Charter remains an authored canonical artifact. The snapshot is a distinct Engine-produced canonical record binding that revision; it is not another Artifact Instance and cannot be edited or promoted as one. | Authored source and owner derivation have different identities and lifecycles. |
| **User- or agent-authored policy candidate.** A Decision Record contains approved rationale and a user or agent submits candidate policy text stored beside project artifacts. The submission cites the record only by a Markdown link, omits exact proposition role/scope/currentness, and has no Policy Admission Closure. | The artifact and candidate may be governance provenance, but no Assurance, Metric, Gate, Operational Reliance, or Action Authorization policy exists for execution. The responsible owner must deterministically close admission or refusal against exact candidate content, typed rules, relevant exact governance revisions and dependency modes, required typed approvals/exceptions, finite candidate accounting, and non-relaxable invariants. | Candidate authorship and organizational judgment are permitted; artifact promotion, bare links, prompts, and agent roles do not substitute for executable owner admission. |
| **Selector receives an unadmitted exception.** An authorized actor authored an exact exception source, but the distinct exception-admission responsibility has not established policy conformance, scope, identity, and currentness. | Assurance Selection refuses the input and cannot admit, broaden, or self-authorize it. Only a separate admission record may supply a bounded effect. | Record authorization, admission, and selection-time application remain distinct. |
| **Durable failed attempt.** A Dock retains an exact timeout terminal and the producer retains a partial source observation. | Both may be durable occurrence provenance and canonical for what their origin owners reported. Neither is an authored artifact, canonical evidence, readiness result, or gate truth. | Durability and bounded canonicality do not imply artifact status or downstream admission. |
| **Self-described capability.** A producer manifest claims support for a role and a Dock ceiling permits the request, but no independent Producer Semantic Capability definition has been admitted. | Evaluation Coordination cannot treat the producer as semantically capable. The manifest and ceiling remain Dock/producer facts and cannot create the missing Contracts-domain capability authority. | Producer self-description and operating ceiling cannot substitute for admitted semantic capability. |
| **Conformant artifacts but unknown evaluation readiness.** Engine produces a successful Profile Conformance Assessment for the selected profile and its required project-artifact instances, but a required Producer Selection has a valid current capability fact that leaves a necessary semantic limit unknown. The Evaluation Purpose is decision-seeking. | Artifact/profile conformance remains successful. Evaluation Readiness is separately `unknown`, the Attempt Disposition is `held`, and no Prospective Evaluation Basis or launch follows. | Artifact/profile conformance does not imply Evaluation Readiness or attemptability. |
| **Concise explanation and authorized drill-down.** Contracts closes one gate result with complete canonical explanation. The default surface requests a concise human explanation; the user then requests deeper detail, while one source section is unavailable or outside the viewer's disclosure authority. | Both responses are projections of the same canonical explanation. Each binds source, density, disclosure, decisive status/reason, and omissions; the drill-down reaches the highest available and authorized detail and reports the bounded omission without reevaluating. Neither view alters the gate or becomes the only retained basis. | Canonical explanation truth, projection density, source availability, and disclosure authority are independent. |
| **Flow packet under a budget.** Flow includes a Charter rendering, a grounding projection, and a gate-summary projection, then omits lower-priority detail to fit a budget. | Flow owns the packet identity and omission/budget accounting. Engine and Contracts retain authority for each included source; the packet is not a replacement artifact or evaluation record. | Assembly authority is limited to assembly decisions. |
| **Transport round trip.** A valid Tauri payload containing a projected gate result is captured and later submitted to another consumer. | Schema validity and byte equality prove only representation integrity. The consumer must resolve and verify the referenced Contracts result or admit the payload through a declared source policy; the payload itself is not Contracts truth. | Representation does not transfer owner authority. |
| **Cross-owner search.** A user searches one catalog and sees a Work Specification, Evaluation Authorization Binding, Producer Attempt, gate result, and Flow packet. | The catalog preserves each object's category, owner, identity, and relationship. Search inclusion does not make the four non-artifact objects Artifact Instances or the catalog their owner. | Unified discovery does not require a universal repository. |
| **Logged process state.** A host logs that a process was running and later closes an exact Host Enforcement Fact and attempt terminal. | The live process state and raw log remain transient/diagnostic unless separately admitted. The closed fact and terminal are new durable provenance objects with exact host/Dock ownership. | Logging is not implicit semantic promotion. |
| **Historical owner result after a newer revision.** A newer Charter, policy, capability observation, or authorization relation exists after an earlier gate result closed. | The earlier artifacts, definitions, records, provenance, projections, and result retain their exact historical identities. Current applicability or reliance is assessed separately and does not rewrite them. | Historical identity and present currentness are distinct. |
| **Projection fed back as truth.** A compact gate explanation or Markdown Charter rendering is offered as the authoritative source for a new evaluation. | Direct reuse refuses. The consumer resolves the exact owner source, or a separately named policy treats the projection as a source observation/candidate with explicit provenance and admission. | A projection cannot become an authority shortcut. |
| **One process hosts several owners.** A future implementation places Assurance Selection, Producer Capability Authority, and Evaluation Coordination in one deployable component. | The three responsibilities retain separate definitions, records, decisions, refusals, and attribution. Colocation is permitted; semantic collapse is not. | Physical topology does not determine bounded-context ownership. |

## Settled versus deferred

This record settles:

- the six-category ontology plus the subordinate representation distinction;
- canonicality as owner- and proposition-scoped rather than repository-wide;
- the exact six authored canonical project-artifact families and `Work
  Specification` as the greenfield work-family vocabulary;
- the inclusion and exclusion rule for generic Artifact Kind / Artifact
  Instance machinery and rejection of a universal `derived artifact` mode;
- dedicated semantic-owner repositories with optional shared low-level
  primitives when durability is required, without requiring every canonical
  result to be persisted, plus a non-authoritative cross-owner catalog;
- the classifications of the important objects named across all companion
  decisions, including Policy Admission Closure and the complete Operational
  Reliance/action chain;
- Engine ownership inward of Contracts and all semantic owners inward of SDK,
  CLI, Tauri, skills, and other transports;
- assurance governance and exception admission, Assurance Selection, Producer
  Capability Authority, and Evaluation Coordination as distinct
  responsibilities within the broader Contracts bounded context, without
  selecting their physical placement or collapsing exception authorization,
  admission, and application;
- independent admission of Producer Semantic Capability from producer
  self-description, Dock ceiling, availability, actual reach, and host
  enforcement;
- deterministic policy-candidate admission as one owner-produced Policy
  Admission Closure without a mandatory Basis/Result pair, every-artifact
  requirement, authored-text waiver, open-ended AI safety judgment, or recursive
  self-admission;
- projection, assembly, representation, and canonical-explanation boundaries,
  including one canonical truth, concise default explanation, requested
  authorized drill-down, independent disclosure, and no density-driven
  reevaluation; and
- the category-specific identity, lifecycle, persistence, and currentness
  distinctions required to prevent authority collapse.

Detailed evaluation-occurrence identity, evidence closure, prospective
applicability, semantic time, and historical resolvability remain owned by the
proposed
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md).
Detailed authored-artifact fields, references, requiredness, profile instances,
subject/snapshot binding, and input/result relations remain owned by the proposed
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md).
Detailed present-use reliance, local and parent eligibility, explicit
`historical_validity_only` versus `continuing_current` Engine verification,
action authorization, attempts, terminals, and receipts remain owned by the
proposed
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
This record classifies those objects without redesigning them, approving those
proposals, selecting physical placement, or authorizing implementation. A
semantic gate remains pure and non-mutating.

Also deferred to later human-approved task-level planning are concrete schemas,
types, APIs, operation names, storage layouts, repository implementations,
catalog mechanics, crate/module placement, transports, cache implementations,
schema generation, task slices, acceptance planning, and rollout. Concrete
policy mappings, metrics, thresholds, producer selections, policy-admission
encodings, explanation-projection levels and interaction design, UI layout,
publication, and external host or Substrate adoption remain owned elsewhere or
deferred.

Human approval of this proposed record would make it semantic input to later
planning only. It would not authorize implementation, change any phase or
slice status, or establish a delivery outcome.
~~~~~~~~~~~~

## authored-artifact-evaluation-relationships-semantic-decision.md

SHA-256: `302ce44b8485de13304649943c735d5b8824351f53d0f37b659c11b73ee50e9e`

~~~~~~~~~~~~markdown
# HCM authored-artifact and evaluation-relationship semantic decision

**Status: PROPOSED**

This record fixes the greenfield semantic relationships between Handbook's six
authored canonical project-artifact families and the posture, assurance,
evaluation, metric, gate, and downstream-disposition domains. It defines how an
authored artifact may declare project intent, become an exact input, or refer to
a separately owned result without becoming runtime observation, evaluation
truth, executable policy, or action authority.

It is a narrow companion to the proposed
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
[posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
Those records retain their object classifications, semantic owners, lifecycle
stage order, authorization, readiness, evidence, truth, reliance, and action
boundaries.

This record is semantic authority for human review only. It is not
implementation authority, a slice or packet specification, a schema or API
design, or rollout planning. Current source is evidence for the existing
artifact taxonomy and owner direction, not a compatibility obligation. The
product is greenfield; no V1 or legacy artifact name, alias, migration, bridge,
or dual behavior is selected.

## Purpose and scope

The authored Artifact Kinds are exactly:

- **Project Authority / Charter**;
- **Project Context**;
- **Environment Context**;
- **Work Specification**;
- **Decision Record**; and
- **Risk Record**.

Only instances of these six families participate in Engine's generic Artifact
Kind / Artifact Instance lifecycle. An artifact revision is authoritative for
the bounded authored proposition its family owns. It is not authoritative for a
different semantic owner's derived, observed, evaluated, or operational
proposition.

The relationship chain fixed here is:

```text
authored Artifact Instance revision
  -> exact subject/snapshot and scope binding
  -> zero or more typed semantic references
  -> explicit resolution and currentness under each consuming owner
  -> exact evaluation or policy input binding, when declared
  -> separately owner-produced posture / assurance / evaluation / gate truth
  -> optional typed outcome or disposition reference from a later artifact revision
```

No arrow transfers ownership backward. A later reference to a result does not
make the artifact its producer, and a result does not silently mutate the
artifact that helped form its inputs.

This record settles semantic relationship kinds, reference direction,
per-family responsibilities, profile-instantiation meaning, currentness and
invalidation consequences, and anti-cycle rules. It does not select concrete
fields, schemas, APIs, repositories, crate placement, UI, or downstream action
policy.

## Canonical vocabulary

Terms in separate rows are not synonyms.

| Term | Fixed meaning |
| --- | --- |
| **Artifact Kind Identity** | The exact Engine-admitted identity and semantic-content version of one of the six authored project-artifact families. It defines the family contract; it is not an authored instance or revision. |
| **Artifact Instance Identity** | The exact Engine/profile-owned identity for one project-scoped instantiation of an Artifact Kind. It remains stable across its immutable revisions and does not mean the current revision. |
| **Artifact Revision Identity** | The immutable Engine-owned identity of one exact promoted content revision of one Artifact Instance, including its kind, instance, project/subject binding, content identity, and lineage. A path, label, record identifier, or current-head pointer is not this identity. |
| **Artifact Subject Binding** | The exact Engine artifact-domain-owned relationship between an Artifact Revision Identity and the project or other declared subject, any exact subject snapshot bound as part of the artifact's own authored proposition, and the artifact's authored applicability scope. It establishes only what the artifact purports to describe. A consuming owner's relation from that source binding to its own Posture Scope, evaluation subject or snapshot, evidence scope, authorization scope, or action scope is a distinct consuming-owner closure and cannot be supplied by the Engine artifact binding alone. |
| **Artifact Scope** | The exact bounded area to which an authored revision declares its proposition applies. It is not automatically a Posture Scope, evaluation scope, authorization scope, evidence scope, or action scope; a consuming owner must establish the exact relation it needs. |
| **Typed Semantic Reference** | An artifact-authored relation whose source is one exact Artifact Revision Identity and whose target names one exact semantic owner, target object kind, owner-recognized target identity, target revision or semantic version when that target kind has one, and declared relation role. The target may be historical, non-current, or not yet admitted only when its owner has issued or recognizes the exact target as an existing object or pre-admission candidate. The reference records none of those target states. Naming an identity does not let the artifact create, reserve, predict, or assert a future owner-produced object. Free-form prose, a path, label, unversioned name, latest lookup, or guessed future identity is not a Typed Semantic Reference. |
| **Declared Input Reference** | A Typed Semantic Reference by which an authored revision nominates an exact artifact, canonical definition or policy, owner-produced record or result, durable provenance object, owner-recognized candidate, or other exact owner object permitted by the consuming definition as relevant to a later named operation. It expresses authored intent only. An unresolved, unadmitted, non-current, incompatible, or refused target does not become usable because it is named. The consuming owner must independently resolve the exact target, obtain every required owner admission and currentness observation, and close it under the consumer's exact role, subject or snapshot, scope, compatibility, multiplicity, and accounting rules. Admission or derivation that creates a different owner identity does not retarget the reference. |
| **Bound Evaluation Input** | A closure-specific immutable relation by which a named consuming semantic owner accepts one exact Artifact Revision Identity, canonical semantic definition or policy, owner-produced record or result, durable provenance object, or other exact owner object permitted by that closure, together with its exact role, subject or snapshot, scope, compatibility, required admission and currentness observations, and complete accounting, into one named requirements, applicability, plan, Purpose, readiness, final-context, or other evaluation closure. It may be formed from a Declared Input Reference or directly from a consuming definition's declared input rule. It is not a mutable state of the target, does not alter the source artifact, and does not transfer target ownership. |
| **Outcome Reference** | A Typed Semantic Reference from a later artifact revision to an already closed owner-produced evaluation, metric, gate, explanation, current-use, eligibility, or action result. It records a relationship; it neither reproduces the result nor grants its authority. |
| **Disposition Reference** | An Outcome Reference to one exact Operational Reliance Determination and the exact Operational Disposition facet being documented. Eligibility and disposition are facets of that one atomic owner result, not independently formed or current records. The reference may document how work was treated, but it neither computes the Determination nor authorizes or performs a mutation. |
| **Governance Source Reference** | A Typed Semantic Reference in an artifact revision to an already owner-recognized target, declaring that the source revision supplies rationale, provenance, or source material for that target. A target owner's separately formed input or provenance binding from an owner-produced object back to a pre-existing artifact revision is not a Governance Source Reference authored into the earlier artifact. If the target did not yet have an owner-recognized identity when the artifact revision formed, only that later target-owned binding—or a later artifact revision that names the issued target—can establish the relationship. |
| **Authored declaration** | Project intent, desired state, rationale, classification, plan, risk proposition, treatment, or other content for which one of the six artifact families is authoritative. It is not an observation merely because it is precise or current. |
| **Owner-produced state** | A resolved, admitted, observed, verified, evaluated, or disposition proposition established only by its named semantic owner from exact inputs. It may cite an artifact revision without becoming authored artifact content. |

`Record` alone is not a sufficient category name. A Decision Record and Risk
Record are authored canonical project artifacts. An authorization record,
evidence record, gate result, or disposition record is an owner-produced object
under its own domain. Their shared noun does not create a shared lifecycle or
repository.

## Relationship invariants

### Exact references do not transfer authority

- Every semantic relationship that can affect posture resolution, assurance
  selection, prospective applicability, plan or purpose formation, readiness,
  evidence membership, claim/metric/gate truth, or a downstream disposition
  MUST bind an exact target owner, object kind, identity, semantic version or
  revision, relation role, subject/snapshot, and scope needed by that consumer.
- An artifact MAY contain a human-readable summary of a target. The summary is
  never the target's semantic content and MUST NOT be used when the exact owner
  object is unavailable, stale, ambiguous, or unverifiable.
- A Typed Semantic Reference establishes only that the source revision declares
  the relation. It does not establish that the target is admitted, current,
  applicable, compatible, sufficient, satisfied, passed, or authorized for use.
- Before use, the target's semantic owner MUST resolve or verify the exact target
  and the consuming owner MUST close the target under its own declared
  currentness, compatibility, and scope rule. Failure refuses that use; it does
  not rewrite the source artifact.
- A reference to a mutable head, ambient repository search, nearest compatible
  version, path, label, or prose description MUST NOT enter a semantic closure.
  A named exact resolver policy may resolve a candidate set, but its exact basis,
  target, and accounting must be closed before the consuming object forms.
- Project/profile conformance, structural schema validity, promotion, approval,
  and artifact currentness do not perform another owner's policy admission or
  result formation.

### Reference direction and target-owned lineage

An owner-produced definition, admission, snapshot, Purpose, result, or
disposition MAY bind an exact pre-existing Artifact Revision Identity as an
input or provenance source under that owner's own closure. A pointer stored in
the target object is the target owner's binding; it is not content added to the
artifact revision. The semantic dependency remains from the pre-existing
artifact revision into the owner-produced object.

A later artifact revision MAY separately add a Governance Source, Outcome, or
Disposition Reference to that already owner-recognized target. The valid causal
form is:

```text
pre-existing artifact revision A1
  -> owner-produced target T1
  -> later artifact revision A2 that records T1
```

`A2` is not an input to `T1`, and reciprocal stored references do not create a
same-occurrence semantic cycle.

If no owner-recognized target identity exists when an artifact revision forms,
the artifact revision MUST NOT claim a Typed Semantic Reference to the future
object. A later target-owned binding may cite the earlier artifact, or a later
artifact revision may name the issued target. When a reference names a pre-
admission candidate and admission produces a different target identity, the
original reference remains bound to the candidate; only an exact owner relation
or later artifact revision may name the admitted target.

A backlink, catalog edge, projection, rendered link, or navigation relation is
not a semantic relationship by itself. Failure to resolve, admit, verify, or use
a target refuses the consuming operation and does not rewrite the source
artifact. Any separately declared artifact/profile conformance consequence
remains an artifact-owner result and does not establish target admission or
currentness.

### Authored inputs and owner-produced truth remain distinct

An authored revision may be consumed in only one of these ways:

1. as exact authored project truth within its family's proposition;
2. as an explicit declared input whose consuming owner independently resolves
   and closes it;
3. as source material for a separately admitted definition, policy, exception,
   fact, evidence candidate, or other owner object; or
4. as a later provenance-bearing reference to an already owner-produced result.

These uses are distinct. In particular:

- Project Context and Environment Context are never ambient policy selectors,
  runtime observations, canonical evidence, producer capability, readiness, or
  Host Enforcement Facts.
- Work Specification acceptance language is not a Contract claim, Evidence
  Evaluation Policy, Metric Definition, or Gate Policy unless the responsible
  owner separately admits an exact definition and the Work Specification refers
  to it.
- An accepted Decision Record does not admit executable policy. A user or agent
  MAY author policy-candidate content or organizational rationale in a Decision
  Record, prompt, configured role, or another permitted source, but the
  responsible semantic owner must independently close admission.
- An accepted Risk Record does not admit an exception, waive a requirement, or
  authorize operational reliance.
- A separately named admission policy MAY accept an exact artifact revision as
  source material or an evidence candidate. The resulting admitted object has a
  different identity, owner, proposition, and lifecycle; the artifact does not
  become evidence or runtime truth by implication.

### Authored policy candidates require deterministic owner admission

A user, agent, authored artifact, or tool MAY propose policy content. Candidate
authorship supplies content, provenance, and organizational judgment; it does
not produce a canonical semantic definition or policy. The responsible policy
owner MUST form the exact **Policy Admission Closure** defined by the
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md)
before executable use.

That closure MUST bind the exact candidate; typed vocabulary and conformance;
finite candidate selection and accounting; every relevant exact Charter,
Project Context, Decision Record, or Risk Record revision with its proposition
role, scope, and historical, formation-only, or continuing-current dependency
mode; and every non-relaxable invariant required by the owner's exact admission
rule. When that rule requires an approval or exception, the closure MUST
separately bind the exact typed, authorized source, its authority, scope,
identity/version or revision as applicable, and currentness. It requires only
governance sources the rule declares relevant, not every artifact family for
every policy. A bare Markdown link, accepted prose, candidate author's authority
claim, or artifact promotion is insufficient proof. Authored text remains
governance input and MUST NOT be reclassified as runtime evidence, a waiver, or
an independently authenticated authority fact.

Admission is deterministic rather than an AI's open-ended judgment about what is
organizationally “safe.” A candidate is mechanically prohibited only when an
admitted constraint or non-relaxable invariant says so, and the owner refusal
identifies that rule. The exact admission-rule version is governed outside the
candidate population it decides, so a policy candidate cannot recursively admit
its own admission process.

## Profile instantiation and requiredness

Artifact Kind registration, profile instantiation, and semantic use are three
different decisions:

- Registering an Artifact Kind makes the family available; it creates no
  Artifact Instance.
- A selected profile establishes the exact Artifact Instance roster, roles,
  project binding, authored-artifact dependencies, and requiredness conditions
  for that profile. Each resolved descriptor has an exact identity and
  fingerprint.
- Profile requiredness governs artifact/profile conformance only. It does not
  make the artifact an assurance requirement, observation role, evidence item,
  readiness prerequisite, gate predicate, or action precondition.
- A consuming definition MAY either name one or more exact Artifact Revision
  Identities directly or declare a typed artifact-selection relation by Artifact
  Kind, instance role, subject or snapshot, and Artifact Scope. The formed
  consuming-owner closure MUST bind the exact revisions used. An Evaluation
  Purpose may bind only already resolved exact revisions; an unresolved
  family/role query, mutable head, or repository search is not a Purpose input.
- A consuming relationship MAY be required even when profile conformance makes
  that artifact optional. Failure to resolve and bind the required relationship
  refuses that policy or owner formation; it does not retroactively change
  profile conformance.
- When an authored revision supplies Declared Input References, the candidate
  population for that relationship is exactly those referenced targets unless
  the consuming definition separately declares an additional exact resolver
  relationship. A consumer MUST NOT add another same-kind or same-role artifact
  through ambient repository search, current-head lookup, or convenience.
- When a consuming definition resolves artifacts from a profile or role
  relation, its resolution basis MUST bind the exact selected profile and
  relevant Artifact Instance Descriptor identities and fingerprints; the finite
  eligible instance and revision population; the exact subject, scope,
  selection, and currentness rule; the required cardinality, multiplicity, or
  composition relation; and complete selected, excluded, missing, refused, and
  ambiguous accounting. Repository order, path order, descriptor order, and
  arbitrary first/current match have no semantic precedence.
- A profile-required artifact that no selected policy, resolution relationship,
  or Purpose declares as an input remains outside that semantic closure.
- A profile, descriptor, eligible-roster, cardinality, selection-rule, or
  currentness-basis change that changes or makes unverifiable the eligible or
  selected population requires a fresh affected consuming-owner closure. When
  that closure participates in Evaluation Purpose semantic content, use of the
  changed closure requires a new Evaluation Purpose and deliberately new
  Evaluation Occurrence Identity.

Current repository truth instantiates one always-required Charter, one
always-required Project Context, and one optional Environment Context in the
shipped-root profile while only registering the other three families. That is a
current profile choice, not a universal family cardinality and not proof that
Work Specification, Decision Record, or Risk Record are semantically absent.

## Per-family responsibility and relationship map

| Authored Artifact Kind | Authoritatively declares | May reference or supply as an explicit input | MUST NOT establish |
| --- | --- | --- | --- |
| **Project Authority / Charter** | Constitutional project authority, Project Classification, authored posture baseline and dimension values, governance nominations, descriptive constraints, and authorized Charter change provenance. | Exact governance definitions; source material for posture resolution; exact Decision/Risk revisions; separately admitted policy or exception records; other exact authority references declared by its kind. | Resolved Posture Snapshot; Posture Scope resolution; Assurance Policy selection; Resolved Assurance Requirements; Engine evaluation authorization; evidence, metric, gate, disposition, or mutation truth. |
| **Project Context** | Authored project/domain/product description, system boundaries, ownership, authoritative source references, and known unknowns for its exact project scope. | Exact context inputs to subject/scope resolution, policy selection, plan or purpose formation, grounding, or explanation only when the consuming definition explicitly declares that role and binds the exact revision. | Runtime/system observation; current external state; producer capability or availability; authorization; readiness; evidence; applicability; claim, metric, gate, or action truth. |
| **Environment Context** | Authored descriptions of named intended or known environments, declared constraints, expected capabilities, authoritative references, and known unknowns for exact environment scopes. | Exact declared environment input to Producer Selection, host-prerequisite definition selection, plan/purpose formation, or explanation when explicitly declared and independently resolved. | Producer Semantic Capability; current availability; actual prerequisite satisfaction; Dock ceiling; Requested or Actual Observed Reach; Host Enforcement Fact; readiness; canonical evidence; or proof that an environment exists now. |
| **Work Specification** | The objective, bounded scope, non-goals, planned work, acceptance intent, dependencies, and authored work-planning state for one feature, slice, delivery unit, or other exact body of work. Authored work-planning state is distinct from both the Engine-owned Artifact Instance lifecycle and every separately owned operational workflow, closeout, eligibility, promotion, or mutation state. | Exact Contract, claim, requirements, policy, Evaluation Purpose, evaluation result, semantic gate, Operational Reliance Determination and displayed eligibility/disposition facet, Action Authorization Refusal, Action Terminal, and Mutation Receipt references in the causally valid revision appropriate to each relation. | Evaluation Purpose or requirements merely by prose; evidence; claim/metric/gate computation; local closeout; parent eligibility; action disposition; action authorization; completion authority; promotion, deployment, or any mutation outside its authored-artifact lifecycle. |
| **Decision Record** | A governed human decision, status, context, rationale, consequences, supersession, participants or authority provenance, and the exact scope of that human decision. | Governance Source References to exact definitions, policies, selectors, exceptions, approvals, artifact revisions, owner results, or action records; exact supersession references to other Decision Record revisions. | Executable policy, exception admission, owner currentness, assurance selection, evaluation truth, action authorization, or mutation merely because its prose is accepted or resembles a rule. |
| **Risk Record** | A bounded authored risk or uncertainty proposition, owner, treatment intent, review basis, risk-governance status, and acceptance or monitoring rationale for an exact scope. | Exact source observations, canonical evidence, evaluations, gates, Decision Records, typed exception records, policies, treatments, and action results as provenance-bearing references. | Canonical evidence membership; finding or risk measurement; proof of mitigation; prospective applicability; Assurance Policy relaxation; exception/waiver authority; gate reinterpretation; or operational permission. |

### Project Authority / Charter to posture and assurance

The Charter is the authored authority source for persisted posture declarations,
not the resolved posture consumed by assurance selection. Engine's posture owner
MUST resolve one exact current Charter revision and exact Posture Scope into a
distinct immutable Resolved Posture Snapshot. That snapshot binds the Charter
revision and resolution/currentness basis.

Assurance Selection consumes only the Resolved Posture Snapshot and its other
declared admitted inputs. It MUST NOT reread the Charter, free-form governance,
shortcut, red-line, stance, or domain-override prose. A new Charter revision does
not mutate a prior snapshot or result. When the posture owner's declared source-
currentness rule requires the new revision at formation or a later checkpoint,
it requires a new snapshot, requirements closure, and downstream occurrence.

### Project Context and Environment Context to evaluation formation

Project Context and Environment Context MAY influence scope, requirements
selection, Producer Selection, prerequisites, or purpose only through a named
consumer definition that declares the exact relationship and binds the exact
Artifact Revision Identity, subject/snapshot, scope, compatibility, and
currentness basis. No owner may search ambient current artifacts after forming
its closure or treat an undeclared context revision as an input.

An Environment Context statement such as “production supports isolated
processes” is an authored declaration. Producer Capability Authority must still
admit exact Producer Semantic Capability; a Dock owner must supply its exact
operating ceiling; Evaluation Coordination must obtain current availability and
prerequisite facts; and only the enforcing host can produce a Host Enforcement
Fact for an attempt. Agreement among the statements does not merge them.

### Work Specification to evaluation and action results

Work Specification is the canonical greenfield name for the authored plan of
one bounded body of work. `FeatureSpec` is neither a synonym, alias, nor seventh
family. Current identifiers, paths, stages, or types that use `feature_spec` are
implementation vocabulary to reconcile in later planning, not architecture
requiring a compatibility bridge.

A Work Specification may declare acceptance intent in prose and may carry exact
Declared Input References to separately admitted Contracts definitions,
Assurance Selection inputs, or evaluation policies. Only those owners establish
executable semantics. If an Evaluation Purpose governs work in a Work
Specification, the Purpose MUST bind the exact pre-existing Work Specification
revision and every exact definition or owner result it consumes; it cannot bind
only a record id, path, current head, or summary.

A later Work Specification revision may add Outcome or Disposition References
to an already closed evaluation, gate, Operational Reliance Determination,
Action Authorization Refusal, Action Terminal, or Mutation Receipt. A
Disposition Reference identifies the shared Determination and the exact
Operational Disposition facet being documented; it does not make that facet or
its paired Current-Use Eligibility facet an independent result. The reference
means “this revision records that exact owner outcome.” It does not mean the
Work Specification computed the result, that a gate mutated work, or that an
authored `completed` label proves local closeout, parent eligibility, release,
or deployment. The downstream reliance, authorization, executor, and state
owners retain their respective results and authority.

### Decision Record to definitions, policies, and exceptions

A Decision Record may explain why a separately owned definition, policy,
selector, exception, or action was proposed, admitted, selected, or changed. It
may also carry policy-candidate content or organizational judgment and may bind
an already owner-recognized target through a Governance Source Reference. The
target owner MUST independently establish the target's exact semantics and
applicable admission or authorization. For a definition, policy, or selector
candidate, that includes its Policy Admission Closure with exact source roles,
scope, dependency/currentness modes, compatibility, required separately
authorized approvals or exceptions, and supersession. A particular exception
record or action instead follows its own separately typed authorization and
admission boundaries; referencing it does not turn it into a policy candidate
or substitute Policy Admission Closure for those owner results.

Decision Record acceptance establishes only the human-decision proposition
declared by that record. It cannot turn copied prose or a bare link into
executable policy, make an unadmitted policy current, authorize a particular
exception, replace a required approval source, or substitute for an owner
result. A policy admission rule may consume exact Decision Record revisions as
organizational-judgment inputs without requiring a new human prompt for every
candidate. If the decision changes executable semantics, a separate new
candidate, definition/policy identity, and owner admission are required;
neither target is mutated through the Decision Record reference.

### Risk Record to evidence, assurance, and exceptions

A Risk Record may cite exact source observations, canonical evidence, claim or
metric results, semantic gates, or action results as review provenance. The
reference does not admit evidence into an evaluation, add it to an already
closed evidence set, or establish a finding. Risk treatment and acceptance are
authored governance propositions, not proof that the risk is mitigated.

A Risk Record cannot establish prospective applicability, alter an already
formed Resolved Assurance Requirements identity, or exercise exception or
waiver authority.

When an admitted Assurance Policy or Assurance Policy Selector declares risk
information as a selection input, each declared risk-input role MUST state
whether it consumes:

- the exact Risk Record revision as an authored risk or governance proposition;
  or
- a separately owner-produced admitted fact, observation, classification, or
  result whose exact source closure may bind the Risk Record revision as
  provenance.

The two input modes are not substitutable. Direct artifact use does not turn
authored likelihood, impact, treatment, acceptance, monitoring, or rationale
into an observed or measured fact. Derived-object use does not transfer the
derived proposition, admission authority, or currentness to the Risk Record.

An admitted selector or policy may apply only its pre-existing immutable
ordinary selection semantics to that exact input. A Risk Record's acceptance,
treatment, or monitoring status MUST NOT be interpreted as a per-record
exception, waiver, proof of mitigation, or authority to omit or weaken an
otherwise applicable requirement. Any exception or waiver effect requires the
separately authorized and admitted typed exception policy and exact exception
record fixed by the posture and assurance-policy decision.

Neither an exact Risk Record revision nor an owner-produced object that merely
restates its authored proposition may support a prospective applicability
prune. Only an independently established pre-existing canonical fact that
satisfies the exact Prospective Applicability Policy, proposition, scope,
currentness, and reuse requirements may do so; the Risk Record may then be
provenance but is not the applicability proof.

## Revision, currentness, and invalidation

An Artifact Revision Identity is immutable historical truth. “Current” is an
Engine-owned relation for a named profile, instance, scope, use, and semantic
time; it is not a mutable flag inside the revision.

Every consuming definition MUST declare one of the following artifact-source
dependency modes for each relationship:

- **Exact historical revision.** The relationship requires only the exact
  referenced Artifact Revision Identity. It asserts no artifact-currentness
  proposition.
- **Direct required-current revision.** The relationship requires the same
  exact Artifact Revision Identity plus a positive Engine artifact-currentness
  verification at every named semantic checkpoint declared by the consumer.
- **Owner-derived object.** The consumer binds an exact separately owner-
  produced snapshot, admission, fact, record, or result whose source closure
  binds the artifact revision. The deriving owner MUST declare whether that
  source revision is historical or provenance-only with no source-currentness
  requirement, required current only when the derived object forms, or required
  current again at one or more named later semantic checkpoints.

The derived object's own admission, currentness, supersession, and applicability
remain separate target-owner observations. The words `snapshot`, `admitted`,
`approved`, `current`, or `latest`, content recency, and the existence of a newer
artifact revision MUST NOT imply an artifact-source dependency mode. No consumer
may infer one mode from recency or convenience. The consequences are closed:

| Change or condition | Required semantic consequence |
| --- | --- |
| A new revision appears for an artifact that was not a declared or transitive source input | No existing posture, assurance, purpose, evaluation, gate, or disposition identity changes by implication. |
| A consumer intentionally uses an exact historical revision under a relationship that permits historical use | The relationship remains bound to that revision. The current head is not substituted, and the historical relation grants no present authority beyond the consuming policy. |
| A required-current artifact revision is no longer current before snapshot, requirements, plan, or Purpose formation | That formation refuses or selects the new exact revision through a fresh declared resolution basis. It cannot bind the stale revision as current. |
| The same exact direct required-current Artifact Revision receives a fresh positive currentness verification at a later declared checkpoint | The new verification observation and every closure that explicitly binds that observation receive their own exact identities. The Artifact Revision does not change, and no replacement revision is inferred. An existing Purpose or occurrence may be retained only when its precommitted checkpoint rule and the applicable companion lifecycle explicitly permit that refresh before occurrence commitment; otherwise a fresh Purpose and occurrence are required. |
| A newer source revision appears after an owner-derived snapshot, admission, fact, record, or result has formed | The derived object remains immutable historical truth for its exact source closure. A historical/provenance-only or formation-only source relation does not silently invalidate it. A continuing-current source relation follows the deriving owner's exact checkpoint rule and may make the derived object unusable at that checkpoint or require a fresh derived object. No owner silently rebinds the old object to the new source revision. If the fresh derived object replaces an input in Purpose semantic content, a deliberately new occurrence is required. |
| The owner-derived target's admission, currentness, applicability, or supersession changes while the source artifact revision remains unchanged | The source artifact and its reference remain unchanged. The target owner's exact observation governs semantic use. Any fresh target or consuming closure follows that owner's rule, and replacement of a Purpose input requires a new Purpose and occurrence. |
| A bound required-current artifact changes after Purpose formation but before launch | The earlier Purpose and occurrence retain history but cannot silently rebind. A required artifact-currentness verification failure stops the existing path; using the new revision requires every affected owner-derived closure and a deliberately new evaluation occurrence. |
| A bound required-current artifact changes after collection but before final evaluation closure | Attempt and source provenance remain immutable. New semantic truth refuses when the declared currentness rule cannot be met; a replacement revision cannot retroactively authorize or reshape the collection. |
| An artifact changes after claim/metric/gate truth closes | The historical result remains exact for its bound revision closure. Present reliance is decided later and may require a fresh evaluation; this record makes no reliance decision. |
| A later artifact revision adds a reference to an earlier result or disposition | The new revision records the relationship prospectively from its own formation time. It does not alter the result, its input revision, or the occurrence that produced it. |
| A Decision Record supersedes another Decision Record | Both exact revisions remain historical. The supersession relation affects Decision Record governance only unless another owner explicitly consumes and verifies it. |
| A Risk Record treatment or status changes | A new risk revision forms. No evidence set, assurance requirement, gate, exception, or action truth changes unless its owner forms a fresh result under a declared relationship. |

Any artifact revision, owner snapshot, definition, or result that participates
in an Evaluation Purpose's semantic content participates in its Evaluation
Content Fingerprint through its exact owner identity and revision. Substitution
therefore requires a new occurrence even when rendered text or user-facing
meaning appears equal.

## Causal and anti-cycle rules

The semantic graph is acyclic within one evaluation occurrence:

```text
pre-existing artifact revisions and owner snapshots
  -> requirements / applicability / plan / purpose
  -> authorization / readiness / prospective basis
  -> attempts / evidence closure / final evaluation
  -> claim / metric / gate results
  -> later action results
  -> optional later artifact revisions that reference those results
```

The following are invalid:

- using a Work Specification revision as a Purpose input when that same revision
  refers to the Purpose, evidence, gate, or disposition that has not yet closed;
- revising Project Context or Environment Context from same-occurrence
  observations and feeding the revision back into that occurrence's scope,
  Producer Selection, readiness, or final context;
- treating a Decision Record written in response to an evaluation as the policy
  that governed that same evaluation;
- treating a Risk Record updated from a gate as a prospective applicability,
  assurance-relaxation, exception, or evidence input to that gate;
- adding a result reference to an artifact revision already bound by an
  immutable Purpose; or
- allowing any same-occurrence output to alter the Resolved Posture Snapshot,
  Resolved Assurance Requirements, Observation Plan, Formation Policy,
  evidence-set membership, Metric Definition, or Gate Policy that produced it.

Same-occurrence feedback MUST refuse at the earliest formation or congruence
boundary. A valid feedback path creates a later artifact revision or separately
owned observation/recommendation and, if used for evaluation, a deliberately
new occurrence with fresh closures. Exact historical linkage is preserved; it
is never rewritten into an earlier input.

## Semantic responsibility map

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Engine artifact/profile domain | Six Artifact Kinds; profile instantiation and requiredness; Artifact Instance and Revision identity; promotion, current-revision resolution, subject binding, lifecycle, mutation, and lineage. | Posture snapshot, executable policy admission, evaluation truth, operational disposition, or generic authority over owner-produced records. |
| Project authors and artifact governance | Exact authored content and approvals appropriate to each family. | Another owner's admission, currentness, observation, evaluation, or action result. |
| Engine posture domain | Resolution of exact Charter revision and Posture Scope into Resolved Posture Snapshot; snapshot verification and later separately authorized transition. | Charter authoring, assurance selection, or mutation inferred from an artifact/result reference. |
| Contracts assurance and definition responsibilities | Independent admission and resolution of selectors, policies, exceptions, contracts, claims, evidence policies, metrics, and gates; Resolved Assurance Requirements from exact admitted inputs. | Treating Project/Environment Context, Decision prose, Risk acceptance, or Work Specification criteria as executable by implication. |
| Contracts Evaluation Coordination | Exact artifact/snapshot input bindings used by applicability, plan, Purpose, readiness, and prospective-basis closure, while preserving occurrence and anti-cycle rules. | Artifact mutation, Engine authorization, evidence or gate truth, or downstream action. |
| Contracts evidence/evaluation responsibilities | Candidate admission, evidence closure, applicability, claims, metrics, semantic gate, and canonical explanation under exact closed inputs. | Ambient artifact rereads, artifact mutation, risk acceptance, Decision Record execution, or action disposition. |
| Producer Capability, Dock, producer, and host owners | Their separate capability definitions, ceilings, attempts, source observations, actual reach, prerequisites, terminals, and Host Enforcement Facts. | Promoting Environment Context declarations into runtime facts or altering artifact content by implication. |
| Work Progression and other Operational Reliance owners | Exact finite source-result and target closure, action-specific Policy Admission Closure and policy selection, Operational Reliance Bases, atomic Determinations with paired eligibility/disposition facets, Reliance Refusals, and present-use currentness/replay. Work Progression is the proposed owner for local closeout and parent eligibility. | Rewriting artifacts or gate truth, treating a Disposition Reference as a separate result, issuing action authority, or mutating state. |
| Action requester, authorization, executor, and state/effect owners | Exact Action Intent; Action Authorization Binding or Refusal with declared advisory, verified, or enforced authority provenance; Action Attempt Identity and Terminal; and authoritative Mutation Receipt for committed state change. | Treating artifact prose or a reference as independently authenticated identity/delegation, fabricating host enforcement, improving reliance eligibility, or reporting mutation success without the state-owner receipt. |
| SDK, CLI, Tauri, Flow, Pipeline, skills, projections, and transports | Bounded composition, presentation, deterministic views/assemblies, correlation, and transport of exact source and relationship identities. | Resolving ambient references, inventing owner truth, changing relation direction, or treating rendered prose as an exact target. |

Physical colocation does not collapse these responsibilities. A cross-owner
catalog may expose the graph, but catalog inclusion, backlinks, or navigation
do not create semantic relationships absent the exact source revision and typed
relation.

## Discriminating scenario corpus

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Stale Charter revision at posture resolution.** A request names Charter revision `C1` as current, while Engine reports `C2` is the current revision for the exact project and Posture Scope. | Required-current posture resolution refuses or deliberately resolves `C2` and forms a new snapshot basis. It never labels `C1` current or substitutes `C2` under a `C1` identity. | Artifact history, currentness, and owner snapshot formation are distinct. |
| **Charter changes after requirements selection.** Snapshot `P1` and requirements `R1` bind Charter revision `C1`; a new current revision `C2` changes a relevant posture dimension before a run forms. | `P1/R1` remain historical. A current run uses a new resolved snapshot and requirements closure and cannot reread or merge `C2` into `R1`. | Assurance consumes exact Resolved Posture Snapshot, not mutable Charter state. |
| **Different source-currentness modes for derived objects.** Admitted policy `Q1` binds Decision Record revision `D1` as historical rationale with no continuing source-currentness requirement. Resolved Posture Snapshot `P1` binds Charter revision `C1` under a declared continuing-currentness rule. Later revisions `D2` and `C2` become current. | `Q1` is not rewritten or made non-current merely because `D2` exists; the policy owner independently determines `Q1` currentness. `P1` remains immutable historical truth, but it cannot satisfy a checkpoint that requires its bound Charter source to remain current after `C2` replaces `C1`; a fresh snapshot and affected downstream closures are required. Neither target silently rebinds to the newer source revision. | Owner-derived identity, source provenance, source-currentness dependency, and target-owner currentness are distinct. |
| **Ambient Project Context.** A planner forms a Purpose using “the current project context” but binds no exact revision, scope, consumer policy, or resolution accounting. | Purpose formation refuses. The context cannot be discovered or substituted from ambient repository state after closure. | Declared contextual relevance is not a Bound Evaluation Input. |
| **Optional artifact required by one evaluation.** Environment Context is optional for profile conformance, but a selected planning policy explicitly requires exact revision `E1`. The artifact is absent. | Profile conformance retains its own optional result; the policy or evaluation formation refuses for its missing declared input. | Profile requiredness and semantic-input requiredness are independent. |
| **Environment declaration versus host fact.** Environment Context says production supports network denial. A selected host cannot prove it applied that restriction for the attempt. | The declaration remains exact authored context. Readiness, launch, or candidate admission follows its own facts and may refuse; no Host Enforcement Fact is fabricated from the artifact. | Desired/declared environment state is not enforced runtime state. |
| **Work Specification acceptance prose.** A Work Specification says “tests must pass” but names no exact Contract claims, Metric Definition, Gate Policy, or requirements closure. | The statement remains authored acceptance intent. Contracts cannot construct claims or a gate from the prose. | A plan is not an executable evaluation definition. |
| **Work Specification bound into an occurrence.** Purpose `P` binds Work Specification revision `W1`. A later edit creates `W2` with semantically similar text before collection. | `P` remains bound to `W1`; it cannot substitute `W2`. If the relationship requires direct artifact-currentness, the existing path stops and `W2` requires a new Purpose and occurrence. | Exact revision identity defeats mutable-prose substitution. |
| **Gate pass recorded by a later Work Specification.** Gate result `G1` closes for an occurrence whose Purpose bound `W1`. A later revision `W2` references `G1` and an action-domain closeout disposition. | `W2` records exact owner outcomes. It neither computed `G1` nor proves closeout from `pass`; `W2` cannot be rebound as an input to the completed occurrence. | Work Specification is a work plan and provenance consumer, not a result or action owner. |
| **Authored completed label without disposition.** A Work Specification author marks an authored planning state `completed`, but no separately owned current-use or closeout result exists. | The artifact revision may be valid authored project truth, but no local closeout, parent eligibility, promotion, release, deployment, or other operational proposition is established. | Authored lifecycle language is not operational authority. |
| **Decision predates target identity.** Decision Record revision `D1` supplies rationale for a proposed policy, but the policy owner has issued no policy or pre-admission candidate identity. Contracts later admits policy `Q1` and binds `D1` as source provenance. In a comparison case, `D1` names an exact owner-recognized candidate `Qc`, and admission later creates distinct policy identity `Q1`. | `D1` contains no Typed Semantic Reference to the not-yet-issued `Q1` and is not mutated when `Q1` forms. `Q1` may bind `D1` under its owner-produced source closure, and a later Decision Record revision may reference `Q1`. In the comparison case, the original reference remains to `Qc`; the owner's candidate-to-policy relation, not silent retargeting, connects `Qc` and `Q1`. | An authored artifact cannot mint a future owner result; target-owned provenance, candidate identity, and later artifact-authored reference are distinct relations. |
| **Decision prose resembles policy.** An accepted Decision Record says all security failures block release, but the candidate supplies only a Markdown link, no exact target scope or finite candidate accounting, and no exact admission-rule closure. | The human decision and rationale remain canonical authored truth. No executable Gate or Operational Reliance Policy exists until the responsible owner deterministically produces a Policy Admission Closure that binds the exact candidate, relevant governance-source revision and role, scope, dependency modes, required approvals or exceptions, complete accounting, and non-relaxable checks. | Governance acceptance and a bare link do not perform semantic-policy admission or turn authored text into runtime evidence or a waiver. |
| **Decision references admitted policy.** A later Decision Record revision cites exact admitted policy `Q1` and explains why it was chosen. `Q1` is later superseded by its owner. | The record's historical reference remains exact. It does not keep `Q1` current, supersede it itself, or update to the successor automatically. | A Governance Source Reference preserves rather than transfers target ownership/currentness. |
| **Risk acceptance offered as waiver.** A Risk Record marks a security risk accepted and is submitted directly to Assurance Selection to omit the related claim. | Selection refuses the shortcut. Only a declared admitted policy input, or an independently authorized and admitted typed exception record with bounded effect, can influence requirements. | Risk governance status is not exception authority. |
| **Authored Risk Record versus owner-produced risk fact.** Selector `S1` declares exact Risk Record revision `R1` in an authored-risk-classification role. Selector `S2` instead declares exact owner-produced risk fact `F1`, whose source closure cites `R1`. A caller supplies `F1` to `S1`, supplies `R1` to `S2`, or asks either selector to treat `R1`'s accepted status as a waiver. | Each substitution refuses. `S1` may interpret only the authored proposition declared for its role; `S2` may interpret only the independently owner-produced proposition of `F1`. `R1` does not become the owner of `F1`, and neither input creates an exception or prospective applicability proof. | Authored risk governance, owner-produced risk fact, assurance selection, typed exception authority, and prospective applicability are non-substitutable. |
| **Risk evidence reference after closure.** A Risk Record cites favorable evidence not included in the sealed Canonical Evidence Set Closure and asks the evaluator to consider it. | The reference cannot change membership or truth. A new occurrence is required if policy permits new collection; the Risk Record remains review provenance only. | Artifact linkage cannot reopen evidence closure. |
| **Same-run circular feedback.** Gate `G1` raises a Risk Record, whose new revision is offered as a requirements or applicability input to `G1`. | The feedback refuses. The new revision may inform only a later owner operation and, if evaluated, a deliberately new occurrence. | One occurrence has an acyclic semantic graph. |
| **Multiple same-role instances without cardinality.** A profile exposes two applicable Environment Context instances and a planner selects “the environment context” without an exact relation or complete accounting. | Selection or Purpose formation refuses as ambiguous. Neither repository order nor current-head order chooses one. | Repeatable artifact relationships require exact cardinality and membership. |
| **Explicit nomination versus profile-roster resolution.** A profile exposes applicable Environment Context revisions `E1` and `E2`. Work Specification `W1` explicitly supplies a Declared Input Reference to `E1`. Consumer definition A declares that exact reference as its complete candidate source; separate consumer definition B declares resolution of every applicable `environment_context` role from the profile roster. | A closes over `E1`; it does not add `E2` ambiently. B binds the exact profile and descriptor basis, accounts for both `E1` and `E2`, and selects or excludes each under its declared cardinality and selection rule. Either consumer refuses if its required membership or accounting cannot be established. | Explicit nomination and family/role resolution have different finite candidate boundaries; neither repository order nor an unrecorded current-head search selects membership. |
| **Retained projection after source-resolution loss.** A Flow packet or other projection was formed earlier from exact source artifact and owner-result identities with exact projection or assembly accounting. A later owner-resolution observation cannot resolve one source. | The already formed projection remains immutable historical output for its recorded source closure. Any current display preserves the exact source identities, carries the owner-attributed non-success resolution observation, and states that source dereference or exact replay is presently unavailable. It cannot be regenerated from rendered text, used as source-owner truth, supplied as a Bound Evaluation Input by substitution, or repair historical resolvability. | Projection history, source-owner resolution, exact replay availability, and semantic input use remain distinct. |

## Settled versus deferred

This record settles:

- the exact six authored Artifact Kinds and `Work Specification` as the sole
  greenfield work-family vocabulary;
- Artifact Kind, Instance, Revision, Subject Binding, Artifact Scope, Typed
  Semantic Reference, Declared Input, Bound Evaluation Input, Outcome,
  Disposition, and Governance Source vocabulary;
- the non-transfer of authority through a reference; owner-recognized pre-
  admission candidate boundary; target-owned input/provenance bindings; and the
  independent resolution, admission, compatibility, currentness, subject, and
  scope checks required before semantic use;
- profile instantiation and profile requiredness as artifact-conformance
  concerns distinct from semantic-input requiredness, with exact finite
  candidate populations and complete multiplicity/selection accounting;
- each authored family's authoritative declarations, permitted relationship
  roles, and forbidden runtime/evaluation/action interpretations;
- Charter revision to Resolved Posture Snapshot direction;
- explicit, exact use of Project Context and Environment Context without ambient
  selection or promotion into runtime facts;
- Work Specification as a feature/slice/work plan that may reference, but cannot
  compute or own, evaluation, atomic reliance, authorization, terminal, receipt,
  or downstream mutation results;
- user-, agent-, artifact-, and tool-authored policy candidates as permitted
  governance inputs that require deterministic owner-side Policy Admission
  Closure, exact relevant source roles/scope/dependency modes, complete
  selection/accounting, and mechanical invariant checks without bare-link proof,
  authored-text evidence/waiver conversion, open-ended AI judgment, or recursive
  self-admission;
- Decision Record as human-decision and organizational-judgment provenance rather
  than executable policy or per-policy prompt ceremony;
- Risk Record as authored risk governance rather than evidence, assurance
  relaxation, exception, or operational authority, including non-substitutable
  direct-artifact and owner-produced-risk input modes;
- exact historical, direct required-current, and owner-derived source-dependency
  modes; their currentness/invalidation rules; and prohibition on silent
  current-head substitution; and
- the within-occurrence acyclic graph and scenario outcomes above.

The proposed
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
owns the detailed semantics of Operational Reliance Basis and Determination,
atomic eligibility/disposition facets, local closeout, parent eligibility,
policy-selected Engine continuing-current verification, action authorization,
attempts, terminals, receipts, and mutation boundaries. It proposes Work
Progression as the semantic owner of local/parent eligibility. This record only
fixes exact artifact relationships to those objects; it does not approve that
proposal, select physical placement, define the work hierarchy or state model,
choose policy content, or authorize implementation or mutation.

The settled explanation principles are one canonical owner truth, a concise
default human explanation, requested drill-down to the highest available and
authorized detail, independent disclosure authority, and no reevaluation from
presentation density. Concrete explanation projections, density choices, UI,
and API representation remain bounded future decisions; no fixed level count or
layout is selected. Also deferred to later human-approved task-level planning
are concrete artifact fields,
schemas, Rust types, APIs, relationship encodings, registry/resolver mechanics,
repository and storage layouts, paths, profile changes, cardinality numbers,
operation names, crate/module placement, transport shapes, indexes, cache
mechanics, task decomposition, acceptance planning, and rollout. No concrete
policy, exception, contract, claim, metric, gate, producer, closeout, or action
definition is selected.

Human approval of this proposed record would make it semantic input to later
planning only. It would not authorize implementation, change any phase or slice
status, or establish a delivery outcome.
~~~~~~~~~~~~

## posture-assurance-policy-semantic-decision.md

SHA-256: `cd80954051e58e9330cc160726fe2477b193ddd286e79fb10c2fe8119ab91380`

~~~~~~~~~~~~markdown
# HCM posture and assurance-policy semantic decision

**Status: PROPOSED**

This record fixes the product vocabulary, responsibility boundaries,
posture-resolution and assurance-selection rules, and discriminating scenarios
for the Handbook Contract Membrane seam from project posture to assurance
requirements. It is semantic authority for human review only. It is not
implementation authority, a slice or packet specification, or rollout
planning.

This record is a companion to the proposed
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
The metric record owns metric and gate meaning. This record owns the earlier
selection seam: how an exact resolved posture may select exact assurance
requirements that metrics and gates later consume without reinterpreting
posture. The operational-reliance record owns the later present-use and action
seam. All seven records remain proposals and supply no implementation authority.

## Canonical model and vocabulary

The canonical forward path is:

```text
explicit Charter posture + exact scope
  -> Resolved Posture Snapshot P_n
  -> assurance-policy selection
  -> Resolved Assurance Requirements
  -> prospective evaluation purpose and requested observation reach
  -> Engine-owned Evaluation Authorization Binding
  -> separately owned readiness determination
  -> Prospective Evaluation Basis
  -> evidence collection and admission
  -> final Contracts evaluation-context closure
  -> claim evaluations
  -> named metrics
  -> semantic gate decision and canonical explanation
  -> action-specific Operational Reliance Basis, when used operationally
  -> one atomic Operational Reliance Determination
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> separate Action Intent and Action Authorization Binding or Refusal
  -> Action Attempt, Action Terminal, and state-owner Mutation Receipt only after committed mutation
```

The following terms are normative. Terms in different rows are not synonyms.
In particular, **posture class** is not a canonical term: Project
Classification already names a different concept.

| Term | Fixed meaning and ownership |
| --- | --- |
| **Project Classification** | The separately typed project-origin or change-context category `greenfield`, `brownfield`, `integration`, `modernization`, or `hardening`. Charter authoring may use it to suggest defaults, but the classification itself neither is nor computes posture. |
| **Posture Level** | One value on the Charter's fixed `1-5` rubric, used as a persisted baseline or an explicit value for one Posture Dimension. The current authoring rubric labels levels 1–5 `Exploratory`, `Prototype`, `Product`, `Production`, and `Hardened`; those labels name numeric authoring levels, not posture types or a project-wide scalar. A level has meaning only for its named dimension or for baseline inheritance; it is not a universal project strictness score. |
| **Posture Dimension** | One named, independent axis of engineering posture. The Charter currently has exactly nine ordered dimensions. A project's posture is the vector of their effective levels, not their maximum, minimum, average, or another ambient scalar. |
| **Posture Scope** | The exact bounded subject and applicability boundary for which posture is being resolved, including every identity and currentness binding required to distinguish that subject from another domain, component, operation, or work context. The posture-resolution owner owns this binding and refuses absent, ambiguous, stale, or unsupported scope. |
| **Resolved Posture Snapshot** | The posture-resolution owner's immutable, fingerprinted determination for one exact Posture Scope. It binds the exact Charter identity and revision, persisted baseline, effective value of every Posture Dimension, exact scope, resolution inputs, and currentness basis. It is the only posture form an Assurance Policy may consume. |
| **Assurance Policy** | An immutable, named, versioned semantic rule that declares the exact Posture Scope it supports, the Posture Dimensions it consumes, and how a supported Resolved Posture Snapshot selects assurance requirements. The assurance-policy authority owns the rule; it does not own posture mutation, evidence admission, metric computation, gate truth, or enforcement. |
| **Assurance Policy Selector** | The immutable, named, versioned selection rule that consumes one exact Resolved Posture Snapshot plus only its declared, admitted selection inputs and resolves exactly one applicable Assurance Policy or refuses. Its identity binds its canonical semantic content and currentness, independently of the exact admitted policy-resolution basis and selected policy even when they are packaged together. |
| **Resolved Assurance Requirements** | The immutable, fingerprinted output of applying one exact Assurance Policy to one Resolved Posture Snapshot. Its identity transitively binds the snapshot; exact selector identity, version, semantic-content identity, and currentness; exact admitted policy-resolution basis and currentness; exact selected-policy identity, version, semantic-content identity, and currentness; every admitted exception input; every other declared selection input; and complete candidate accounting and canonical explanation. Its output binds one exact contract revision and its complete contract-rooted expected-claim inventory; any exact claim-set references rooted in that revision; exact compatible Evidence Evaluation Policy bindings; Metric Definitions and complete configurations; one Gate Policy; and admitted exception requirements. Metrics and gates consume this result; they do not derive it. |
| **Engine Context Resolution** | The separately typed Engine-owned authority domain that establishes the bounded operating reach available to an exact objective and context, including its exact stack, dimensions, parent, constraints, mutation rules, and escalation relations. A Posture Level or Resolved Assurance Requirement is neither a Context Resolution coordinate nor authority to exercise one. |
| **Evaluation Authorization Binding** | The exact purpose-bounded Engine-owned result required by the companion metric/gate record for an authorized evaluation context. It is issued from one exact admitted `ContextResolutionEnvelope` and proves the Engine-owned relationship among that envelope, the evaluation purpose, subject or scope, intended use, and requested reach. Assurance selection may supply purpose provenance but neither produces nor replaces this binding. |
| **Prospective Evaluation Basis** | The exactly identified pre-observation provenance closure defined by the companion metric/gate record. It binds the evaluation purpose, subject or scope, objective/use, requested reach, Evaluation Authorization Binding, and required resolved-assurance/readiness/use-intent provenance, but no evidence-set identity or semantic truth. Resolved Assurance Requirements may be one bound input; they neither issue nor replace authorization. |
| **Evidence Evaluation Policy** | The immutable, named, versioned Contracts-owned role-specific policy defined by the companion record. Assurance selection may bind an exact policy only to an evidence-evaluation role and compatibility constraint declared by the selected contract, claim, or evidence requirement. It cannot alter canonical-admission trust, the claim proposition or finding scale, Metric Definition meaning, or Gate Policy meaning. |
| **Metric Definition** | The immutable, named, versioned, fully configured algorithm definition owned by the companion record. Assurance selection may choose an exact definition reference; it MUST NOT change the definition's scale, unit, population, algorithm, parameters, missing-input policy, or numeric meaning. |
| **Gate Policy** | The immutable semantic decision rule owned by the companion record. Assurance selection may choose an exact policy reference; gate evaluation remains a pure application of that resolved policy to compatible claim evaluations and metric results. |
| **Enforcement Policy** | An action-domain Operational Reliance Policy for an enforcement, workflow, or posture action question. It consumes the exact historical gate result and its declared target/currentness inputs to produce one atomic Operational Reliance Determination. It cannot alter gate truth, grant action authority, or authorize posture mutation by implication. |
| **Enforcement Disposition** | The Operational Disposition facet of that one Operational Reliance Determination: `allow`, `block`, `hold`, or `no_effect`, mechanically paired with its Current-Use Eligibility facet. It is not an independently issued, current, supersedable, or replayable result. A Disposition Reference identifies the shared Determination and its exact Operational Disposition facet. |
| **Posture Observation** | An immutable, provenance-bound, admitted fact that may inform later posture reasoning, including an exact gate result when a named observation policy permits it. An observation is neither a recommendation nor authority to mutate posture. |
| **Posture Recommendation** | A separately owned, explainable proposal derived from admitted observations and a named recommendation policy. It identifies a proposed transition but cannot apply it, and its production does not imply approval. |
| **Posture Transition** | The separately authorized, atomic Engine-owned mutation from one exact current posture authority state to the next. Before transition intent or authorization, one transition-specific Operational Reliance Determination MUST establish `eligible / allow` for the exact expected posture target, bounded transition, and scope. A congruent Action Intent, separate Action Authorization Binding, exact Action Attempt Identity, commit-time checks, Action Terminal, and Engine state-owner Mutation Receipt then govern the mutation. Neither an Assurance Policy, gate, observation, recommendation, nor reliance disposition performs or authorizes it. |

## Current product facts and semantic gap

These facts constrain truthful planning but do not make this proposal
implemented:

- The Charter schema fixes `rubric_scale` to `1-5`, bounds the persisted
  baseline to `1..=5`, and permits an optional `1..=5` override for each of the
  exact ordered dimensions `speed_vs_quality`, `type_safety_static_analysis`,
  `testing_rigor`, `scalability_performance`, `reliability_operability`,
  `security_privacy`, `observability`, `dx_tooling_automation`, and
  `ux_polish_api_usability`. Project Classification is a separately typed field
  with the five values named above. The current authoring rubric labels numeric
  levels 1–5 `Exploratory`, `Prototype`, `Product`, `Production`, and
  `Hardened`; these labels do not replace the effective dimension vector.
- The private `ProjectPostureKernel` resolves each effective dimension from the
  persisted baseline and optional level override. It fingerprints the Charter
  authority and resolved vector, but it does not structurally resolve the
  Charter's free-form `domain_overrides` or `default_stance` text into an exact
  Posture Scope or executable policy.
- Broader `PostureEvaluationPolicy` behavior and multi-source observation,
  recommendation, acknowledgement, and hysteresis behavior are absent. The
  shipped profile accepts `posture_evaluation_policy: null`.
- An atomic Engine-owned posture transition exists. It changes one fixed
  dimension leaf through exact authority, approval, compare-and-swap,
  persistence, recovery, and replay checks.
- Existing gate and local-closeout results remain pure derived truth. They do
  not carry posture policy or mutation authority.
- Typed Engine Context Resolution envelopes already bind objectives, exact
  stacks and dimensions, parents, constraints, mutation rules, escalation
  triggers, and private authority admission/currentness. Current Contracts run
  identity binds requested coordinates but no purpose-bounded Engine Evaluation
  Authorization Binding. This proposal therefore selects a semantic seam, not
  a currently implemented public object.

Therefore no current free-form Charter text, classification heuristic, gate
result, or null profile field may be treated as an implemented assurance-policy
mapping.

## Posture resolution and assurance selection invariants

### Classification does not control posture

- Project Classification MAY inform authoring-time defaults and suggestions.
- A suggestion MUST remain visibly provisional until persisted as explicit
  posture authority through the separately authorized Charter process.
- Persisted explicit posture controls resolution. Classification MUST NOT
  silently override, raise, lower, or relax a persisted baseline or dimension.
- Reclassification alone MUST NOT mutate posture or make an existing explicit
  posture disappear.

### Posture is a scoped dimension vector

- A Resolved Posture Snapshot MUST retain every effective Posture Dimension.
  It MUST NOT collapse the vector to a maximum, minimum, average, label, or
  universal strictness scalar.
- Each Assurance Policy MUST declare exactly which dimensions it consumes and
  the exact scope for which it can consume them. An undeclared dimension MUST
  NOT be interpreted implicitly.
- Scope resolution MUST be exact and MUST fail closed when any required
  binding is absent, ambiguous, stale, conflicting, or unsupported.
- The Charter's current free-form `domain_overrides` and `default_stance` are
  descriptive authoring inputs, not executable selectors or scope bindings.
  Current free-form `allowed_shortcuts` and `red_lines` likewise MUST NOT
  execute as exceptions or change resolved requirements without the admitted
  typed exception authority defined below.
- A Resolved Posture Snapshot MUST be immutable and fingerprinted. Its identity
  MUST bind the exact Charter identity and revision, persisted baseline,
  effective dimension vector, exact Posture Scope, and the inputs needed to
  establish currentness and deterministic replay.

### Assurance selection precedes evaluation

- Assurance-policy selection MUST occur before evidence selection, claim
  evaluation, metric computation, or gate evaluation for the run. Selection
  MUST complete before current evaluation authorization, producer capability,
  operating ceilings, actual observation, or readiness are consulted.
- The Assurance Policy Selector MUST consume only the exact Resolved Posture
  Snapshot and other exact, declared, admitted, and currentness-bound selection
  inputs. Project Classification, mutable posture or Charter state, free-form
  Charter prose, an Evaluation Authorization Binding, requested reach, producer
  capability, a Dock or adapter operating ceiling, actual observation,
  readiness, gate results, enforcement dispositions, and unbound ambient state
  MUST NOT become implicit selector inputs.
- The Assurance Policy Selector MUST resolve exactly one named, versioned,
  supported Assurance Policy or refuse. Missing, ambiguous, stale, substituted,
  or unsupported mapping MUST NOT fall back to a default policy.
- The selector MUST bind and completely account for the exact admitted
  policy-resolution basis it consulted, including every candidate or mapping
  considered and that basis's currentness. Changed selector content, resolution
  basis, selected-policy content, currentness, or another declared input MUST
  change the resolved identity or refuse even when the output references would
  otherwise look equivalent.
- The selected Assurance Policy MUST resolve one exact contract revision and
  its complete contract-rooted expected-claim inventory; any exact claim-set
  references rooted in that revision; exact compatible Evidence Evaluation
  Policy bindings; Metric Definitions and complete configurations; one Gate
  Policy; admitted exception requirements; and a canonical explanation.
- Assurance selection MAY bind an Evidence Evaluation Policy only to a role
  declared by the selected contract, claim, or evidence requirement and only
  when its exact semantics satisfy the declared compatibility constraints. It MUST NOT invent a policy
  role, override claim meaning, affect canonical-admission trust, or silently
  fall back. A missing, incompatible, conflicting, substituted, or unaccounted
  binding MUST refuse.
- Resolved Assurance Requirements MUST completely account for selected,
  excluded, inapplicable, ambiguous, and refused candidates. Their identity
  MUST transitively bind the exact snapshot, selector, policy-resolution basis,
  selected policy, admitted exception inputs, all other declared inputs, the
  currentness of each currentness-bearing input, and that complete accounting.
  An omission MUST NOT become an implicit requirement or relaxation.
- Metrics and gates MUST consume the Resolved Assurance Requirements. They
  MUST NOT reread mutable posture, reinterpret levels or free-form text, select
  alternate requirements, or change the meaning of a Metric Definition.
- When posture influences selection, the Prospective Evaluation Basis and the
  final evaluation context, fingerprint, currentness basis, and canonical
  explanation MUST bind the Resolved
  Posture Snapshot fingerprint; exact Assurance Policy Selector identity,
  version, semantic-content identity, and currentness; exact admitted
  policy-resolution basis identity and currentness; exact selected Assurance
  Policy identity, version, semantic-content identity, and currentness; and
  Resolved Assurance Requirements identity. The requirements identity
  transitively binds the admitted exception inputs, all other declared inputs,
  and complete selection accounting. Replay without those exact bindings MUST
  refuse.
- The corresponding final evaluation context is admissible only after its
  canonical evidence-set identity is fixed and as the exact and complete
  realization of that one Resolved Assurance Requirements identity.
  Its subject snapshot MUST satisfy the exact Posture Scope; its contract
  revision, complete expected-claim inventory, Evidence Evaluation Policy
  bindings, Metric Definitions and configurations, and Gate Policy MUST equal
  the selected outputs; and every selected requirement and every metric or gate
  dependency MUST be closed and completely accounted. Addition, omission,
  substitution, wrong-scope binding, incompatible multiplicity, or an
  unresolved dependency MUST refuse before claim, metric, or semantic-gate
  truth rather than become ordinary `not_applicable`, `not_computable`, or
  `indeterminate` truth.
- Exact realization of Resolved Assurance Requirements is necessary but not
  sufficient to authorize observation or form final semantic truth. Before
  observation, one exact Prospective Evaluation Basis MUST bind those
  requirements and one separately issued exact Engine Evaluation Authorization
  Binding congruent with its subject or scope, objective, intended use, exact
  stack, and requested observation reach. After the canonical evidence-set
  identity is fixed, the final evaluation context MUST bind that exact basis
  and evidence set. The requirements identity neither contains nor replaces
  the run's granted authority. Contracts may exact-check final congruence but
  MUST rely on Engine for issuance lineage, envelope admission, request-within-
  envelope, authority admission, currentness or historical validity, and
  supersession.
- Requirements spanning unrelated contract revisions MUST produce separate
  Resolved Assurance Requirements and evaluation contexts. This record does
  not authorize cross-context gate composition.

### Policy candidates require deterministic owner admission

A user, agent, authored artifact, or tool MAY author an Assurance Policy
Selector, Assurance Policy, Evidence Evaluation Policy, Metric Definition, Gate
Policy, Enforcement/Operational Reliance Policy, posture-observation policy,
recommendation policy, or exception-policy candidate. Candidate authorship may
supply organizational judgment, rationale, claimed provenance, and proposed
content. It does not make the candidate executable, independently authenticate
an author, or waive an invariant.

Each responsible semantic owner MUST deterministically produce the category-3
**Policy Admission Closure** defined by the companion
[canonical-object decision](canonical-object-ontology-and-ownership-semantic-decision.md)
before a candidate becomes an admitted category-2 definition or policy. The
closure binds the exact candidate; typed vocabulary and conformance; finite
candidate selection/accounting; proposition roles; exact scope; and every
relevant exact Charter, Project Context, Decision Record, or Risk Record revision
with its historical, formation-only, or continuing-current dependency mode.
Where an admission rule requires approval or exception authority, the closure
separately binds the exact typed, authorized source, its authority, scope,
identity/version or revision as applicable, and currentness; a bare Markdown link
or approval prose is insufficient proof. The admission rule requires only
relevant governance families, does not turn authored text into runtime evidence
or a waiver, and is a non-recursive root outside the candidate population it
decides. “Unsafe” means mechanically prohibited by admitted constraints, not an
AI's open-ended organizational judgment. Governance inputs supply organizational
judgment without requiring a fresh human prompt for every policy.

This is the cross-boundary invariant shared with the
[metric/gate record](metric-gate-semantic-decision.md): **posture may select
which exact assurance requirements govern a run, but it may never alter the
meaning of the selected contracts, evidence policies, metrics, or gate, and no
metric or gate may reinterpret posture.**

### Assurance and Context Resolution remain orthogonal

- An Assurance Policy and Resolved Assurance Requirements select proof
  obligations. They MUST NOT issue, grant, broaden, infer, or imply Engine
  Context Resolution authority. An observation requirement may state the reach
  needed to establish a claim; it does not grant that reach.
- An Evaluation Authorization Binding, requested reach, producer capability,
  Dock or adapter operating ceiling, actual observed reach, per-claim evidence
  adequacy, viewer disclosure authority, readiness, and the Prospective
  Evaluation Basis remain separate facts. None may select a lighter Assurance
  Policy or remove, weaken, or reinterpret a resolved requirement.
- If available authorization or capability is below a selected requirement,
  the requirement remains unchanged. The separately owned readiness or
  Prospective Evaluation Basis boundary MUST report or refuse the inability to
  attempt the exact proof plan; assurance selection MUST NOT choose an easier
  plan to fit current reach.
- Missing, stale, superseded, mismatched, or otherwise unverifiable Engine
  authorization before observation refuses Prospective Evaluation Basis
  formation and prevents collection. A change after evidence exists but before
  final evaluation-context closure refuses new claim, metric, or gate truth.
  Neither case may become an exception, implicit waiver, lighter policy,
  evidence insufficiency, `not_computable`, `indeterminate`, or
  `not_applicable` result.

### Trust invariants do not relax

At every numeric Posture Level, including level 1 (`Exploratory` in the current
authoring rubric), the following remain non-relaxable:

- authority and provenance checks;
- Engine-owned Context Resolution admission and non-expansion;
- canonical admission and exact identity;
- complete claim, evidence, metric, condition, and selection accounting;
- deterministic evaluation and replay; and
- fail-closed behavior for absent, ambiguous, stale, incompatible,
  unauthorized, or unsupported inputs.

Posture may select among admitted requirements. It MUST NOT weaken the trust
needed to establish the inputs, identities, policy, or truth of that selection.

### Exceptions are typed authority

- Exception governance, typed-policy authority, exact record authorization,
  record admission, and selection-time application are distinct semantic acts.
- Canonical Charter governance MAY nominate an authorization policy, required
  approval or issuer classes, process expectations, record location, and
  minimum authoring fields. Those declarations are governance inputs only;
  Charter prose, approver names, process metadata, record location, and minimum
  fields do not themselves constitute an executable exception policy,
  authorize a particular record, or admit an exception for selection.
- A designated exception-policy authority MUST own the exact typed, named,
  versioned policy that defines permitted issuers, scope, effects, record
  requirements, and currentness. Each exact exception record MUST be separately
  authorized under that policy and admitted through exact authority, identity,
  scope, currentness, and policy-conformance checks.
- Free-form shortcut, red-line, stance, domain-override, rationale, or other
  Charter prose MUST NOT execute as an exception.
- Only an admitted, typed, named, versioned exception policy and its exact
  authorized exception record may affect requirement selection.
- Assurance selection MAY apply only the admitted bounded effect. It MUST NOT
  mint, approve, broaden, infer, or self-authorize an exception.
- The selection explanation MUST identify every applied exception and its
  authority, scope, identity, currentness, and effect. A missing or invalid
  exception MUST fail closed rather than become an implicit waiver.

One eventual component may host more than one of these responsibilities, but
Charter governance nomination, typed-policy authority, exact record
authorization, record admission, and selection-time application MUST remain
semantically distinct and independently attributable.

## Bounded monotonicity and vector order

Monotonicity is bounded to one declared policy family and version, one exact
Posture Scope, and fixed non-posture inputs. Within that boundary, increasing a
dimension that the Assurance Policy declares relevant MUST NOT remove or weaken
any resolved requirement. Each policy family MUST make its requirement-strength
relation explicit enough to establish this property without inventing a global
score.

This rule does not create a total ordering over posture vectors. Componentwise
comparison is valid only when every compared dimension is equal or moves in the
same direction. If one relevant dimension increases while another decreases,
the vectors may be incomparable; selection MUST apply the exact policy rather
than collapsing them to a scalar or guessing which vector is stricter.

## Causal order and run binding

The only permitted causal cycle across evaluations is:

```text
P_n Resolved Posture Snapshot
  -> Resolved Assurance Requirements
  -> prospective evaluation purpose and requested reach
  -> Engine Evaluation Authorization Binding
  -> separately owned readiness determination
  -> Prospective Evaluation Basis
  -> evidence collection and admission
  -> final Contracts evaluation-context closure
  -> metrics / gate
  -> semantic gate decision and canonical explanation
  -> optional admitted Posture Observation
  -> separately owned Posture Recommendation
  -> transition-specific Operational Reliance Basis
  -> one Operational Reliance Determination for the exact posture transition
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> exact Posture Transition Action Intent, only after `eligible / allow`
  -> Action Authorization Binding or Action Authorization Refusal
  -> exact Action Attempt Identity and commit-bound Engine mutation checks
  -> Action Terminal + Engine state-owner Mutation Receipt after committed change
  -> P_{n+1}
```

- A run remains bound to `P_n` and its Resolved Assurance Requirements. A
  concurrent posture transition MUST NOT silently rebind or reselect that run.
  Its exact Evaluation Authorization Binding is separately fixed in the
  Prospective Evaluation Basis and then bound with the exact evidence set in
  the final evaluation context; neither step converts requirements into
  authority.
- A fresh evaluation under `P_{n+1}` MUST perform fresh scope resolution and
  assurance selection, Engine authorization, readiness, and context closure.
  Results under `P_n` and `P_{n+1}` are distinct contexts even when their other
  inputs happen to match.
- Approval of a Context Resolution escalation creates a new Engine
  authorization basis and requires a fresh Evaluation Authorization Binding,
  readiness determination, Prospective Evaluation Basis, and final evaluation
  context. It MUST NOT mutate the Resolved Assurance Requirements,
  retroactively authorize prior evidence, upgrade a prior semantic decision,
  or rewrite its historical explanation.
- A gate MUST NOT mutate posture, reselect the posture governing its own run,
  or emit a transition as part of pass/fail evaluation.
- Gate `pass`, `fail`, `indeterminate`, or `not_applicable` MUST NOT
  automatically raise or lower posture. A permitted gate result may become an
  admitted Posture Observation only through its separately owned policy.
- A Posture Recommendation remains non-mutating. Before any transition intent,
  one transition-specific Operational Reliance Basis and Determination MUST bind
  the exact expected posture target, proposed transition, current semantic time,
  historical results, target/currentness facts, policy, and complete accounting.
  Only `eligible / allow` permits formation of a congruent Action Intent.
- The Engine transition-authorization owner MUST then independently produce an
  Action Authorization Binding or Refusal for that exact Intent. Policy-permitted
  standalone CLI/SDK flows MAY carry advisory authority from a prompt or
  configured agent role as claimed principal, delegation, and declared authority
  provenance. Those claims are not independently authenticated identity,
  verified delegation, or host-enforced permission; ambient OS permissions are
  not fine-grained Handbook credentials. Advisory authority cannot satisfy a
  policy that requires stronger enforcement or waive expected-target,
  currentness, approval, transition-validity, attempt, or commit-time checks. A
  real enforcing host may supply stronger facts. No general Handbook identity,
  credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
  or ABAC platform is implied.

## Operational reliance and enforcement separation

Enforcement is downstream of immutable semantic gate truth. Each admitted
Enforcement Policy is an Operational Reliance Policy for one exact action
question and target. It MAY consume the exact gate decision, Resolved Posture
Snapshot, Resolved Assurance Requirements, and other declared inputs, but it
MUST form one exact Operational Reliance Basis and one atomic Operational
Reliance Determination with paired Current-Use Eligibility and Operational
Disposition facets. It MUST NOT change the gate, replace `indeterminate` with
`pass`, treat either facet as semantic evidence, or acquire action or posture-
transition authority.

Every present reliance universally resolves the exact Evaluation Authorization
Binding and Engine-confirmed historical validity. The admitted policy MUST then
select exactly one Engine continuing-status mode: `historical_validity_only` or
`continuing_current`. Only `continuing_current` requires fresh positive Engine
current verification at the reliance semantic time and any resulting policy-
required fresh evaluation. Neither mode weakens mandatory current authorization
at collection or final evaluation closure. A later effect always requires a
fresh separate Action Authorization Binding and exact target/currentness checks.

The same gate truth may legitimately yield different atomic Determinations
under different exact policies or action questions. Each Determination preserves
the shared historical gate identity, its own policy/action/target/currentness
closure, both inseparable facets, and a Disposition Reference to its exact
Operational Disposition facet. The canonical gate explanation remains unchanged; operational reliance
adds downstream accounting and does not reevaluate truth or choose an
explanation-density view.

## Responsibility map

| Responsibility | Semantic owner | MUST NOT own or infer |
| --- | --- | --- |
| Charter authoring and governance | Project authority authors and approvers own Project Classification, persisted baseline and dimension values, descriptive guidance, and authorized Charter changes. | Executable scope matching, assurance selection by prose, metric meaning, gate truth, or automatic posture mutation. |
| Posture resolution | The posture-resolution owner resolves exact scope and produces the immutable Resolved Posture Snapshot from current admitted Charter authority. | Classification-based override, assurance-policy invention, evidence interpretation, gate truth, or transition approval. |
| Policy-definition and admission owners | Accept user-, agent-, artifact-, and tool-authored candidates and deterministically produce a Policy Admission Closure or refusal against exact typed vocabulary, relevant governance revisions/proposition roles/scopes/dependency modes, finite accounting, required separately authorized approvals/exceptions, and non-relaxable invariants. The assurance-policy authority then resolves admitted selectors/policies and produces Resolved Assurance Requirements with complete explanation. | Treating authorship, prompts, agent roles, artifact promotion, bare links, approval prose, or favorable outcomes as admission; demanding every artifact family; turning governance text into runtime evidence/waiver; recursive self-admission; posture mutation; or gate evaluation. |
| Engine Context Resolution authorization | Engine alone admits the exact full envelope, establishes request-within-envelope and non-expansion, issues the purpose-bounded Evaluation Authorization Binding, and verifies its lineage, authority/currentness, historical validity, and supersession. | Assurance-policy selection or relaxation, producer capability or readiness, evidence adequacy, gate truth, explanation density, enforcement, or posture transition. |
| Contracts-domain Evaluation Coordination | The proposed Contracts-domain coordination responsibility compares already resolved requirements, exact Engine authorization, requested reach, and admitted producer/operating facts; owns readiness and Prospective Evaluation Basis formation; and determines whether the exact proof plan can proceed. Concrete crate, module, API, persistence, and transport placement remain deferred. | Granting Engine authority, selecting an easier policy, weakening requirements, establishing evidence or gate truth, or host enforcement by implication. |
| Exception governance, authority, and admission | Charter governance owns non-executable nominations and process declarations. The designated exception-policy authority owns admitted exception semantics, permitted issuers, scope, effects, record requirements, and currentness; the exact record authority authorizes each record; and the exception-admission owner establishes authority, identity, scope, currentness, and policy conformance before selection may apply the bounded effect. | Treating Charter prose, approver names, process metadata, record location, or minimum fields as executable authority; allowing assurance selection to mint, approve, broaden, infer, or self-authorize an exception. |
| Contract, evidence-policy, evidence, and claim evaluation | The respective semantic owners admit exact contract and evidence authority, own compatible role-specific Evidence Evaluation Policy semantics, and establish applicability and claim truth under the resolved requirements. | Rereading posture, inventing or substituting policy bindings, changing the selected requirements or claim meaning, gate disposition, recommendation, or posture mutation. |
| Metric computation and gate evaluation | The owners defined by the companion record purely apply exact Metric Definitions and Gate Policy inside the closed evaluation context. | Assurance reselection, mutable-posture interpretation, enforcement, recommendation, or mutation. |
| Work Progression and other Operational Reliance owners | Apply one admitted action-specific policy to exact historical results, target/currentness observations, and complete accounting; universally resolve historical Evaluation Authorization Binding validity; select `historical_validity_only` or `continuing_current`; and produce one atomic Determination with paired eligibility/disposition facets or a Reliance Refusal. Work Progression is the proposed semantic owner for local-closeout and parent eligibility. | Rewriting gate truth or canonical explanation, issuing action authority, mutating state, or silently selecting work hierarchy/state policy or physical placement. |
| Posture observation | A separately owned admission boundary establishes immutable, provenance-bound observations from permitted sources. | Recommendation, reliance, approval, or mutation merely because an observation exists. |
| Posture recommendation | A separately owned recommendation policy derives an explainable proposal from admitted observations. | Applying, approving, forming transition eligibility by itself, or implying a transition. |
| Posture-transition reliance owner | Forms the exact transition-specific Operational Reliance Basis and one atomic Determination before transition intent, with exact expected posture target, bounded transition, currentness, policy, and accounting. | Treating recommendation as eligibility, issuing action authority, mutating posture, or changing historical gate truth. |
| Transition requester and action-authorization owner | The requester forms one congruent Posture Transition Action Intent only after `eligible / allow`. The authorization owner independently produces an Action Authorization Binding or Refusal with exact advisory, verified, or host-enforced authority provenance and fresh conditions. | Treating reliance, Evaluation Authorization, a prompt, ambient OS permission, or a recommendation as authenticated identity, verified delegation, host enforcement, or a reason to mandate a general Handbook IAM platform. |
| Engine posture-transition executor and state owner | Owns one exact Action Attempt Identity and Action Terminal; enforces the same authorization, expected posture, exact transition, approvals, and declared currentness at the atomic mutation boundary; and alone issues the Mutation Receipt for committed `P_{n+1}`. | Ordinary retry after uncertain commit, broadening the authorized transition, recomputing prior gate truth, or reporting committed posture without the authoritative receipt. |
| SDK, CLI, host, and other transports | Preserve exact owner results and provenance, including policy-permitted prompt- or configured-role-supplied advisory claims; a real enforcing host may supply stronger independently verified facts. | Becoming a semantic owner by transport, fabricating authentication/delegation/enforcement, waiving target/currentness checks, or satisfying enforcement-required policy with advisory claims. |

## Discriminating scenario corpus

Each scenario assumes exact admitted identities and fixed inputs except where
the row names a failure.

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Mixed dimension levels without collapse.** A snapshot has a lower `speed_vs_quality` level, a higher `security_privacy` level, and distinct values for the other dimensions. A policy declares only those two dimensions relevant. | The snapshot retains all nine effective dimensions. Selection applies the two declared inputs independently; it does not use the vector's max, min, average, baseline label, or an invented project score. | Posture is a vector and policy declares its consumers. |
| **Classification versus explicit posture.** A project is classified `greenfield`, while its persisted `security_privacy` posture is explicit and higher than an authoring suggestion for that classification. | The explicit posture controls. Classification may explain a suggestion but cannot lower, replace, or mutate the persisted value. | Classification and posture are distinct authorities. |
| **PII adapter versus docs-only tool.** One exact scope binds a PII-handling adapter; another binds a docs-only tool. A third request says only “the project area” and could match either. | Each exact scope resolves independently under a supporting policy. The ambiguous request refuses before assurance selection; free-form domain text does not choose for it. | Scope is exact and fails closed. |
| **Lighter assurance selection and trust.** A supported Resolved Posture Snapshot selects a lighter admitted requirement set, but one candidate lacks authority provenance and another leaves expected claims unaccounted. | Selection or evaluation refuses. A lighter requirement set cannot waive authority, provenance, canonical admission, complete accounting, determinism, or fail-closed behavior. | Trust invariants are non-relaxable at every numeric level. |
| **Selected proof plan exceeds current reach.** A higher posture selects a proof plan requiring observation reach that the current Engine authorization, producer capability, or Dock operating ceiling cannot support. | The exact requirements remain selected and unchanged. Readiness or Prospective Evaluation Basis formation reports or refuses the inability to proceed before collection; no lighter Assurance Policy, omitted requirement, or implied waiver is selected. | Current authorization and capability cannot weaken the engineering standard. |
| **Posture changes mid-run.** A run closes on snapshot `P_n`; an authorized transition creates `P_{n+1}` before the gate finishes. | The existing Prospective Evaluation Basis and final context remain bound to `P_n` and its resolved requirements. They do not reselect. A fresh run under `P_{n+1}` resolves a new snapshot, requirements, authorization, basis, and final context; the two contexts cannot be combined. | Runs bind one immutable posture snapshot. |
| **Approved Context Resolution escalation.** The proof plan remains fixed, but Engine approves a broader or otherwise changed envelope after the first authorization binding was issued. | The approval produces a new authorization basis and requires a fresh Evaluation Authorization Binding, readiness determination, Prospective Evaluation Basis, and final evaluation context. The Resolved Assurance Requirements and all prior evidence, result truth, and explanation remain unchanged and bound to their original bases and contexts. | Escalation changes granted reach, not the proof plan or historical truth. |
| **Failed gate feeds posture reasoning.** A gate returns `fail`, and a named observation policy admits that exact result. A recommendation policy proposes raising a relevant dimension. | The gate truth remains `fail`; the admitted observation and recommendation are non-mutating. A transition-specific Operational Reliance Basis and atomic Determination must first establish `eligible / allow` for the exact expected posture and transition. Only a congruent Action Intent, separate Action Authorization Binding, exact attempt, commit-time Engine checks, Terminal, and state-owner Mutation Receipt can create `P_{n+1}`. | Feedback does not create a causal mutation or authority shortcut. |
| **Relevant-dimension monotonicity.** Within the same policy family/version and exact scope, all non-posture inputs and other dimensions are fixed while one declared relevant dimension increases. | The newly resolved requirements preserve or strengthen every prior requirement under the policy's declared strength relation; none is removed or weakened. | Monotonicity is bounded and dimension-relevant. |
| **Incomparable posture vectors.** For the same policy and scope, vector A is higher in security while vector B is higher in operability, and both dimensions are relevant. | Neither vector is declared globally stricter. The exact policy resolves each independently without scalar collapse; any unsupported combination refuses. | A dimension vector has no global total order. |
| **Missing policy mapping.** A valid snapshot and exact scope have no supported Assurance Policy mapping, or two selectors tie without deterministic precedence. | Selection refuses and produces no Resolved Assurance Requirements. It does not choose a default, nearest, lowest, or highest policy. | Missing or ambiguous mapping fails closed. |
| **Different enforcement, same truth.** The same exact semantic gate decision and context are consumed by two different admitted named/versioned Enforcement Policies for distinct action questions or targets. | The exact semantic gate decision and canonical explanation remain unchanged. Each policy closes its own Operational Reliance Basis and atomic Determination; different paired eligibility/disposition facets are possible and each Disposition Reference identifies its shared Determination and exact Operational Disposition facet. Neither result authorizes an effect. | Operational reliance is separate from gate semantics, and eligibility/disposition are inseparable facets rather than independent results. |
| **Free-form shortcut and red line.** Charter text says a shortcut is allowed for a named area and a red line applies to another, but no admitted typed exception policy and record binds the exact scope. A second input is typed but was issued by an unauthorized actor; a third was synthesized or approved by the selector itself. | The prose and both invalid records refuse and do not change requirement selection. Only an exact record that was separately authorized and admitted under the exact exception policy may have its bounded effect applied. | Governance prose, typing alone, and selector self-authorization are not exception authority. |
| **User- or agent-authored policy candidate.** A Decision Record contains organizational rationale and a user or configured agent supplies candidate Assurance Policy text. The submission cites the record by a bare Markdown link, omits exact proposition role/scope/currentness, and has no owner-produced Policy Admission Closure. | The candidate remains governance input and no executable Assurance Policy exists. The semantic owner must deterministically admit or refuse the exact candidate against typed vocabulary, finite candidate accounting, relevant exact governance revisions and dependency modes, required separately authorized approvals/exceptions, and non-relaxable invariants. | Authorship and organizational judgment are permitted; artifact promotion, links, prompts, and agent roles do not perform policy admission or become runtime evidence/waivers. |
| **Advisory CLI posture-transition request.** After an exact transition-specific Determination yields `eligible / allow`, a standalone CLI request carries a prompt-supplied claimed principal and delegation under an action-authorization policy that permits advisory authority. A comparison policy requires host-enforced authority. | Under the advisory policy, the authorization owner may issue a Binding that truthfully records the claims and advisory basis, but it does not assert authenticated identity, verified delegation, or host enforcement; all expected-target, currentness, approval, attempt, and commit checks still apply. Under the enforcing policy, the same advisory claims cannot satisfy authorization and the request refuses absent the required stronger facts. | Advisory authority may participate in the full action chain without fabricating verified facts, waiving safeguards, or creating a general Handbook IAM mandate. |
| **Deterministic requirement replay.** The exact Charter identity/revision, scope, snapshot fingerprint, selector identity/version/content/currentness, admitted policy-resolution basis/currentness, selected-policy identity/version/content/currentness, admitted exception inputs, and other declared inputs are replayed. A comparison case uses a different policy or resolution basis that yields apparently equivalent requirement references. | Exact replay produces the identical Resolved Assurance Requirements identity, references, accounting, and explanation. The comparison case has a distinct provenance-bound requirements identity or refuses; equivalent-looking references do not collapse different authority. Any changed or missing binding likewise changes identity or refuses. | Resolved requirements are exact, transitively closed, and reproducible. |
| **Exact resolved-requirements realization.** One Resolved Assurance Requirements identity for scope `S` selects contract-rooted expected-claim inventory `C`, exact Evidence Evaluation Policy bindings `E`, configured Metric Definitions `M`, and Gate Policy `G`. Before observation, the valid path forms one exact Prospective Evaluation Basis that binds those requirements and a separately issued congruent Evaluation Authorization Binding. After admission fixes the canonical evidence-set identity, the final context binds that evidence set, basis, and exactly `C/E/M/G`. Comparison cases alter either closure or omit, substitute, or transfer the Engine binding or basis. | The exact valid two-stage closure may evaluate reproducibly. Every comparison refuses at the applicable boundary before claim, metric, or semantic-gate truth; none falls back, reselects assurance, or becomes ordinary `not_applicable`, `not_computable`, or `indeterminate`. | Exact pre-observation provenance and later evidence-bound closure are both necessary and neither substitutes for the other. |

## Settled versus deferred

This record settles the canonical terms, semantic ownership, classification and
posture separation, vector and scope model, snapshot closure, fail-closed
selection, Evidence Evaluation Policy selection compatibility, the
one-contract-rooted requirements-to-evaluation conformance rule, evaluation
provenance, assurance/Context Resolution orthogonality, prohibition on
capability-driven relaxation, separate exact Engine authorization conformance,
the pre-observation Prospective Evaluation Basis and later evidence-bound
evaluation-context ordering, fresh-context escalation, non-relaxable trust
invariants, bounded monotonicity, forward-only causal ordering, transition-
specific Operational Reliance before posture-transition intent, the atomic
eligibility/disposition rule, universal historical Binding validity plus the
explicit `historical_validity_only` / `continuing_current` reliance modes,
separate action authorization and mutation receipts, advisory-authority truth
constraints without a general IAM mandate, deterministic Policy Admission
Closure, operational-reliance separation, typed-exception boundary, and scenario
outcomes above.

Deliberately deferred to later human-approved task-level authority are concrete
policy mappings and requirement sets; thresholds; schema shapes; Rust types or
APIs; storage and registry mechanics; expression languages; migrations;
feature flags; slice or packet decomposition; rollout; and implementation
sequence. This record also does not select concrete policy candidates or
mappings, a work hierarchy or state model, physical component placement,
observation or recommendation algorithms, acknowledgement or hysteresis model,
UI, publication, or external adopter.

The proposed
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md)
owns evaluation-purpose formation, Engine authorization issuance, the Contracts-
domain Evaluation Coordination semantic responsibility, Prospective Evaluation
Basis lifecycle, currentness checkpoints, readiness, capability/ceiling
separation, diagnostics, and retry/cache/supersession/escalation semantics. The
proposed [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
owns the common present-use rule, action chain, and Work Progression semantic
responsibility for local-closeout and parent eligibility. Concrete physical
placement, work hierarchy/state model, policy contents, transition mechanisms,
APIs, schemas, storage, transport, host/Substrate integration, implementation,
and delivery slices remain deferred. These proposals do not silently approve
any such choice or require a general IAM platform.

Human approval of this proposed record would make it semantic input to later
planning only. It would not authorize implementation, change any phase or slice
status, or establish a delivery outcome.
~~~~~~~~~~~~

## evaluation-authorization-capability-and-readiness-semantic-decision.md

SHA-256: `6b117a5f3504d12189ebf0d94360e3f1cf240a5d4505917d18bb4bb4faaf4cb0`

~~~~~~~~~~~~markdown
# HCM evaluation authorization, capability, and readiness semantic decision

**Status: PROPOSED**

This record fixes the greenfield semantic seam between selected assurance
requirements and evidence collection: evaluation-purpose formation, Engine
authorization, producer and operating capability facts, readiness, prospective
evaluation formation, and the currentness boundaries for collection, new
evaluation truth, and operational reliance.

It is a narrow companion to the proposed
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
[posture/assurance-policy decision](posture-assurance-policy-semantic-decision.md),
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
[metric/evaluation/gate decision](metric-gate-semantic-decision.md), and
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
Those records own object classification, assurance selection, exact occurrence
and evidence closure, authored relationships, evaluation truth, and downstream
reliance/action semantics. This record owns none of those detailed domains, and
it is not implementation authority until a human approves it and later task-
level planning separately authorizes delivery.

Current source and completed HCM authorities are evidence for owner direction
and trust boundaries only. Current DTOs, crate placement, the first-party Dock
equality rule, and advisory command behavior neither constrain nor implement
this proposed model.

## Canonical model and vocabulary

The canonical pre-truth chain is:

```text
resolved assurance requirements
  -> Observation Plan with one or more exact Producer Selections
  -> Evaluation Purpose closing over the exact Observation Plan
  -> Evaluation Authorization Request
  -> Engine envelope admission and non-expansion decision
  -> Evaluation Authorization Binding
  -> Readiness Basis
  -> Readiness Assessment
  -> Attempt Disposition
  -> Prospective Evaluation Basis, when attemptable
  -> host launch decision and producer attempt
  -> source observation / evidence candidate
  -> exact canonical evidence-set identity
  -> final Contracts evaluation-context closure
  -> claim / metric / semantic-gate truth
  -> action-specific Operational Reliance Basis and atomic Determination, if used now
  -> separate Action Intent / authorization / attempt / terminal / receipt, if an effect is requested
```

The following terms are normative and non-substitutable:

| Term | Fixed meaning |
| --- | --- |
| **Evaluation Purpose** | An immutable, exactly identified statement formed only after the exact Observation Plan exists. It closes one prospective evaluation's subject or scope, objective, intended evaluation use, exact Resolved Assurance Requirements when applicable, and complete Observation Plan identity. It states why that exact plan is requested; it grants no authority and establishes no readiness or truth. |
| **Evaluation Use Intent** | The closed distinction `decision_seeking` or `diagnostic`. It is fixed before Engine authorization and remains exact provenance through any result or downstream use. Changing it creates a new Evaluation Purpose. |
| **Observation Plan** | The immutable, exactly identified pre-authorization closure that assigns the unchanged proof plan's required observation roles to one or more exact Producer Selections. It completely accounts for every required role, its declared multiplicity or composition relation, and each selection's requested reach without scheduling attempts, granting authority, weakening requirements, or establishing evidence or truth. |
| **Producer Selection** | One immutable, uniquely identified entry in an Observation Plan. It binds one declared observation role or exact compatible role set to one producer kind/configuration, any Dock or adapter, Requested Observation Reach, admitted Producer Semantic Capability and operating-ceiling bases, and declared host prerequisites. Selection identities are ordered and unique; attempts, retries, observations, and limitations bind one exact selection and cannot be substituted or merged implicitly. |
| **Evaluation Authorization Request** | The exact request that asks Engine to admit one Evaluation Purpose and the Observation Plan's complete requested-reach closure under one exact `ContextResolutionEnvelope` basis. It is untrusted until Engine admits it. |
| **Evaluation Authorization Binding** | Engine's immutable, purpose-bounded, exactly identified issuance result. It binds the exact Evaluation Purpose, the bound Observation Plan's complete requested-reach closure, underlying admitted envelope, request-within-envelope decision, and opaque Engine authority/admission lineage. Issuance is historical fact; currentness and supersession are separate Engine-owned observations and relations. |
| **Authorization Verification Observation** | Engine's immutable answer at one named checkpoint, verification mode, and semantic time about an exact Evaluation Authorization Binding. Current-use verification returns exactly one of `current`, `superseded`, `not_current`, or `unverifiable`; historical verification returns exactly one of `historically_valid`, `historically_invalid`, or `unverifiable`. It is not a mutable field inside the binding. A historical outcome cannot authorize a new collection, truth closure, or operational effect. |
| **Producer Semantic Capability** | An independently admitted, exactly identified statement of the observation dimensions, scales, subjects, and limits a producer kind and configuration can semantically represent under declared conditions. It is neither permission to operate nor proof of what one attempt observed. |
| **Requested Observation Reach** | The exact Resolution-stack-bound reach requested for one Producer Selection within an Evaluation Purpose's Observation Plan. It must be within Engine authorization but is not itself authorization, producer capability, an operating ceiling, actual reach, or a claim minimum. |
| **Dock or Adapter Operating Ceiling** | An admitted, exactly identified operational boundary that a selected Dock or adapter promises not to exceed and may require a request to equal. It is not inherent producer capability, Engine authority, actual observation, or host enforcement. |
| **Host Enforcement Fact** | An exact host-owned observation that named translatable restrictions were actually applied for one attempt, or that launch was refused because they could not be applied. A request, protocol field, acknowledgement, or equality check is not this fact. |
| **Actual Observed Reach** | The exact Resolution-stack-bound reach a producer reports for one completed or partial observation under one exact Producer Selection. It is admitted later with source provenance. It may be lower than requested only when that selection's producer contract permits honest partial completion; it may never exceed the selection request, Engine authorization, or admitted operating ceiling. |
| **Readiness Basis** | The exactly identified, immutable input closure for complete pre-launch comparison: Evaluation Purpose, Observation Plan, current Engine authorization verification, Resolved Assurance Requirements, every Producer Selection and its admitted capability/operating-ceiling/host-prerequisite facts, use intent, applicability information available before observation, and every owner-currentness basis used. |
| **Readiness Assessment** | A pure, non-mutating result over one exact Readiness Basis: `ready`, `known_limited`, or `unknown`, with complete per-selection and per-required-role accounting. It predicts whether the complete Observation Plan can answer the unchanged proof plan; it is not evidence, claim truth, metric state, gate truth, permission, enforcement, or a guarantee of success. |
| **Readiness Refusal** | A provenance or formation refusal produced when readiness cannot validly be assessed because any required input is missing, malformed, unadmitted, mismatched, stale, superseded, bound to an unsupported role or dialect, or otherwise unverifiable. It is not a fourth Readiness Assessment state and is never evidence insufficiency or semantic-gate truth. |
| **Attempt Disposition** | The evaluation-coordination result `attemptable`, `held`, or `refused`. A valid Readiness Assessment plus Evaluation Use Intent produces `attemptable` or `held` under the closed table below. A failed authorization or readiness-formation boundary always produces `refused`. It remains advisory until an enforcing host actually honors it. |
| **Prospective Evaluation Basis** | The immutable, exactly identified pre-observation closure formed only for an `attemptable` disposition. It binds the Evaluation Purpose, complete Observation Plan, Evaluation Authorization Binding, launch-check verification, Resolved Assurance Requirements, Readiness Basis and Assessment, Attempt Disposition, every Producer Selection and operating fact, use intent, and required currentness provenance. It contains no evidence-set identity and establishes no claim, metric, gate, or downstream truth. |
| **Producer Attempt** | One exactly identified launch or collection attempt under one Prospective Evaluation Basis and exactly one Producer Selection. A non-idempotent retry is another attempt for that same selection even if it uses the same basis. |
| **Operational reliance** | A separately owned present-tense use of a closed result under one action-specific Operational Reliance Basis to produce one atomic Operational Reliance Determination with paired Current-Use Eligibility and Operational Disposition facets. Historical display and exact replay are not operational reliance; the Determination grants no action authority or mutation. |

`ready` requires positive proof from valid, admitted, current facts that, for
every required role and exact Producer Selection: the Engine-authorized and
requested reach are congruent; the producer capability can represent the
assigned role and dialect; the operating ceiling permits the attempt; every
required host prerequisite can be met; and the Observation Plan's declared
multiplicity or composition relation can be attempted. Absence of a known
blocker is insufficient. `ready` predicts neither collection success, evidence
sufficiency, nor `pass`.

`known_limited` means all formation inputs are valid, admitted, and current, but
one or more exact capability, ceiling, host-prerequisite, reach, or plan-
composition facts positively establish that a selected requirement cannot be
attempted by its assigned selection. It is not `fail` or `indeterminate`.
`unknown` means all required formation inputs are valid, admitted, and current,
but their expressly unknown or otherwise insufficient knowledge can prove
neither positive attemptability nor a positive limitation. Missing or invalid
formation input is Readiness Refusal, never `unknown` or `known_limited`.

## Semantic owner placement and identity

A distinct **Evaluation Coordination** responsibility owns Observation Plan and
Producer Selection closure, Evaluation Purpose, Readiness Basis, Readiness
Assessment, Attempt Disposition, and Prospective Evaluation Basis formation.
The broader Contracts domain is its semantic owner, inward of SDK and
transports, because this responsibility closes Contracts-owned proof
requirements against Engine-owned authorization before the same broader domain
later closes evidence-bound truth. Contracts evidence admission, claim
evaluators, metric computation, and gate evaluation are sibling
subresponsibilities; they consume exact coordination results and MUST NOT
recompute or reinterpret readiness. This settles semantic owner placement.
Concrete crate, module, API, persistence, and transport placement remains
deferred.

This placement is required because the Contracts domain already owns the proof
requirements and final evaluation closure, while the coordination
subresponsibility must consume Engine-owned authorization without acquiring it.
Engine cannot own readiness alone because it does not own contract requirements
or producer semantic adequacy. Assurance selection cannot own it because
present capability must not feed back into requirements. Producers, Docks,
hosts, gates, enforcement, SDK, CLI, and other adapters likewise cannot own the
cross-owner comparison.

Engine alone owns admission of the complete `ContextResolutionEnvelope`,
request non-expansion, issuance and identity of the Evaluation Authorization
Binding, private issuance lineage, authorization verification, historical
validity, not-current reasons, and supersession relations. Evaluation
Coordination and Contracts may exact-check owner-produced identities and
congruence among the subject, scope, purpose, use intent, stack, and requested
reach they own or consume. Engine authorization formation and every later
congruence check MUST refuse when a plan identity, selection, role assignment,
composition/multiplicity relation, or requested reach differs from the exact
Observation Plan bound by the Evaluation Purpose. Coordination and Contracts
MUST NOT reproduce Engine rank, parent, constraint, mutation, authority,
currentness, or escalation decisions.

Every exact formation input participates in identity. Equal coordinate labels
do not collapse different stack fingerprints, subjects, snapshots, purposes,
objectives, uses, parent contexts, constraint closures, authorization lineages,
producer configurations, capability bases, operating ceilings, readiness
times, or currentness bases.

Any changed Observation Plan, Producer Selection, role assignment, declared
plan composition or multiplicity, or Requested Observation Reach requires a
fresh Evaluation Purpose closing over that plan, a fresh Evaluation
Authorization Request and Engine binding, fresh readiness closure, and a fresh
Prospective Evaluation Basis. Neither an older purpose nor an older binding can
authorize a changed plan that happens to target the same subject or coordinates.

Every canonical definition or policy consumed by this record remains subject to
its semantic owner's deterministic **Policy Admission Closure** under the
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md).
A user, agent, artifact, or tool may author a candidate, but candidate text,
prompt authority, a bare link, or owner-unverified organizational judgment is
not an admitted capability, retry, partial-completion, readiness, or other
policy. Admission binds only the exact relevant governance revisions and roles,
complete candidate accounting, and non-relaxable invariant checks; it does not
turn authored text into runtime evidence or require recursive policy admission.

## Observation-plan, evaluation-purpose, and authorization lifecycle

Evaluation Coordination first forms the exact Observation Plan after exact
subject or scope and the unchanged proof plan are available. The plan MUST
completely account for every required observation role through one or more
exact Producer Selections, including any declared multiplicity or composition
relation; it MUST NOT imply scheduling, substitution, or evidence merging.
Evaluation Coordination then forms the Evaluation Purpose as a separate exact
closure over that already identified plan. When posture influenced selection,
the purpose identity transitively binds the exact Resolved Posture Snapshot and
Resolved Assurance Requirements provenance fixed by the companion record. Each
selection's requested reach is chosen to attempt its assigned part of that
proof plan; no request or producer fact may become an input that selects or
weakens the requirements.

Engine receives the exact Evaluation Authorization Request, admits or
re-admits the complete private envelope and its authority, determines whether
the plan identity and complete requested-reach closure exactly match the plan
bound by the Evaluation Purpose and are within that envelope for the exact
purpose, and either:

- issues one exact Evaluation Authorization Binding;
- returns an exact authorization refusal with decisive owner provenance; or
- returns no owner result when the request cannot be verified.

Only Engine can issue the binding. A caller, Evaluation Coordination,
Contracts, SDK, transport, producer, Dock, or host may request, carry, compare,
or consume it but MUST NOT mint, broaden, reinterpret, transfer, or reconstruct
it. The public projection MUST expose only stable evaluation-relevant facts and
exact opaque lineage sufficient for congruence and later Engine resolution. It
MUST NOT expose credentials, authenticator material, private witness objects,
repository paths, raw authority capsules, Engine caches or counters, or the
complete private envelope.

At minimum the immutable binding transitively binds:

- its Engine owner identity and exact binding identity;
- the exact Evaluation Purpose and Evaluation Use Intent;
- the exact Observation Plan and complete Producer Selection/request closure;
- subject or subject snapshot and exact scope;
- objective;
- Resolved Profile and Resolution stack identities and semantic fingerprints;
- active level and the Engine-produced admitted bounds relevant to the request;
- the exact Requested Observation Reach;
- the exact underlying envelope and its root/parent and constraint closure;
- Engine's request-within-envelope conclusion;
- an opaque exact issuance-authority and repository-identity basis;
- issuance semantic time; and
- exact rules naming the checkpoints that require current verification.

Supersession is an immutable Engine-owned relation from an older binding or
request to its successor. It never mutates the older binding. An approved
Context Resolution escalation admits a new envelope and requires a new
Evaluation Authorization Binding; a request for escalation grants nothing.

## Readiness, use intent, and basis formation

Evaluation Coordination creates one Readiness Basis only after Engine has
issued the exact binding and verified it current for pre-launch use. Every
Producer Selection's capability, operating-ceiling, and required host-
prerequisite facts MUST be independently admitted under their named owners and
currentness rules; self-description or a protocol field alone is not an
admitted fact.

One exact Readiness Basis yields exactly one Readiness Assessment or one
Readiness Refusal. The assessment is content-addressed, immutable, and accounts
for every Producer Selection and required observation role. Any change to an
input, role assignment, selection, relevant applicability fact, owner-
currentness basis, or semantic time creates another basis and assessment.

The use-intent rule is closed:

| Use intent and readiness | Attempt Disposition | Required behavior |
| --- | --- | --- |
| `decision_seeking` + `ready` | `attemptable` | A Prospective Evaluation Basis may form. The later result is eligible for operational consideration only under separately owned policy and fresh reliance checks. |
| `decision_seeking` + `known_limited` | `held` | No basis forms and no selection launches. Replace the limited Producer Selection through a new exact Observation Plan, choose a different still-authorized request that can realize the unchanged requirements, request Engine escalation, or create a new explicitly diagnostic purpose. |
| `decision_seeking` + `unknown` | `held` | Valid but insufficient capability knowledge is not treated as ready. Obtain an admitted basis that resolves the unknown, form a new exact Observation Plan, or create a new explicitly diagnostic purpose. |
| `diagnostic` + any valid Readiness Assessment | `attemptable` | A diagnostic basis may form, with every known limitation and unknown retained. Evaluation semantics remain unchanged. |
| Any use + Readiness Refusal or non-current/unverifiable authorization | `refused` | No basis forms and no collection or new truth is permitted at that boundary. |

A diagnostic run may produce canonical evidence and exact claim, metric, and
gate truth under the unchanged requirements when all later closure conditions
are met. Evaluators MUST NOT branch on use intent or apply an easier policy.
Diagnostic provenance cannot be relabeled after formation. A diagnostic result
MUST NOT support positive operational reliance unless a separate exact policy
explicitly permits that provenance; it never authorizes mutation by itself.

Prospective Evaluation Basis formation is idempotent for one exact formation
request: the same complete inputs yield the same identity, and the same identity
with different content refuses. A deliberately new evaluation, changed use
intent, changed authorization, changed readiness fact, changed Observation Plan
or Producer Selection, or changed currentness basis requires a new Prospective
Evaluation Basis.

One basis may have zero or more Producer Attempts for each exact Producer
Selection only when that selection's producer policy allows retries. Delivery
replay of the same attempt identity and bytes is idempotent. A new launch,
rerun, timeout recovery, or changed invocation is a distinct attempt bound to
that one selection and has distinct source-observation lineage even when it
references the same still-current basis. Attempts or observations from
different selections or retries are never merged, substituted, or composed
unless a later exact admission/evaluation policy explicitly declares and
accounts for that relation.

## Currentness checkpoints and historical truth

Currentness is an owner observation at a boundary, not an eternal property of
an immutable artifact. Three checkpoints are mandatory:

1. **Before launch or collection.** Engine MUST verify the exact Evaluation
   Authorization Binding current and unsuperseded. Evaluation Coordination
   MUST re-establish that the Readiness Basis, Assessment, Attempt Disposition,
   complete Observation Plan, and every selected capability, operating ceiling,
   and host prerequisite remain current. Failed authorization or required-
   input currentness produces `refused`. When fresh valid facts instead produce
   `known_limited` or `unknown`, a decision-seeking purpose produces `held`.
   Either outcome stops the affected selection before launch; the plan-level
   disposition remains closed over all selections and cannot be silently
   reduced to the unaffected subset.
2. **Before final Contracts closure and new truth.** After the exact canonical
   evidence-set identity exists, Engine MUST verify the exact binding under the
   final-closure rule, and Contracts MUST exact-check the evidence set,
   Prospective Evaluation Basis, authorization verification, and every other
   context dependency. Missing, mismatched, not-current, superseded, or
   unverifiable authorization refuses new claim, metric, and gate truth.
3. **Before operational reliance.** The downstream owner MUST resolve the
   exact Evaluation Authorization Binding and obtain the Engine historical-
   validity outcome needed to establish that the closed result was validly
   formed. That historical resolution is universal. The admitted Operational
   Reliance Policy MUST then declare exactly one present-use mode for that
   binding: `historical_validity_only` or `continuing_current`. Only
   `continuing_current` requires a fresh positive Engine current verification at
   the reliance semantic time. `historical_validity_only` makes current
   evaluation-authorization status irrelevant to that Determination while every
   other policy-declared result, target, subject/scope, work-structure, and
   currentness dependency remains exact. A later Action Authorization policy may
   independently require a fresh Engine verification; fresh Action
   Authorization Binding and target checks remain separate and mandatory before
   an attempt. A changed dependency requires a new reliance occurrence or fresh
   evaluation exactly when the admitted policy requires one, never a rewrite of
   historical truth.

A source observation, evidence record, closed evaluation context, claim
evaluation, metric result, gate decision, and canonical explanation remain
immutable historical facts for their exact identities. A later revocation,
supersession, stack change, escalation, or posture transition MUST NOT rewrite
them. A current view may say that a historical result was validly issued under
an earlier basis and is no longer eligible for current reliance.

Historical replay, explanation, and any later Operational Reliance require
Engine resolution of exact issuance and the applicable historical-validity
outcome, not a current-authorization fiction. Merely delivering an already
closed immutable result does not relaunch a producer or reacquire authority.
Re-running evaluation follows the launch and final-closure checkpoints above;
present reliance follows its explicit mode, and any effect follows a fresh,
separate action-authorization and attempt chain.

## Capability, operating, and observation invariants

- Assurance requirements state what must be established. They grant no
  capability or authority. Authorization, capability, ceiling, actual reach,
  readiness, diagnostics, retries, escalation, or host limitations MUST NOT
  select an easier policy, remove a requirement, or imply a waiver.
- Engine authorization states what the exact evaluation may request. It does
  not prove producer capability, readiness, actual observation, evidence
  adequacy, gate truth, or host enforcement.
- Producer Semantic Capability states what a producer is admitted to represent
  under exact conditions. It does not grant permission, prove launch, or prove
  actual reach.
- An Observation Plan closes required observation roles over exact Producer
  Selections. It neither schedules attempts nor permits a missing, limited,
  failed, or retried selection to be replaced by another selection implicitly.
- The operating ceiling bounds a Dock or adapter invocation. A producer may
  have broader semantic capability than one operating ceiling, and an operating
  ceiling may be lower than Engine authorization. Neither relationship changes
  the proof plan.
- Host Enforcement Facts exist only when the actual host applies or refuses
  named restrictions. Context Resolution coordinates that lack a faithful host
  primitive remain semantic constraints, not falsely claimed sandbox controls.
- Actual Observed Reach is an observation fact. It MUST be exact and may not be
  clamped upward or downward to match a request, ceiling, or claim minimum.
- Actual reach above Engine authorization, requested reach, or an admitted
  operating ceiling refuses candidate admission. Actual reach below request is
  honest partial collection only under an exact producer policy that permits
  it; later Contracts claim evaluation independently decides per-claim
  eligibility and sufficiency.
- The current first-party Dock rule `request = admitted ceiling = completed
  actual` is one adapter operating contract. It does not prove Engine authority
  or host enforcement and MUST NOT become a universal producer invariant.

## Retry, cache, supersession, and reuse rules

| Event | Required semantic behavior |
| --- | --- |
| Exact request replay before launch | Engine issuance may be idempotent only for the complete same authorization request and lineage; Engine still supplies checkpoint verification. |
| Deliberately new evaluation with apparently equal inputs | Close the exact Observation Plan into a new Evaluation Purpose and create a fresh authorization request and binding. Do not silently transfer the earlier purpose or binding. |
| Changed plan, selection, assignment, multiplicity/composition, or requested reach | Form a fresh Observation Plan, Evaluation Purpose, authorization request and Engine binding, readiness closure, and Prospective Evaluation Basis. Reuse or congruence against an older plan refuses. |
| Attempt retry under unchanged basis | Reuse the Prospective Evaluation Basis only when every bound dependency remains exact and current and that exact Producer Selection's policy permits retry; create a new attempt identity bound to the same selection unless it is exact delivery replay. Other selections and their attempts remain distinct. |
| Authorization becomes non-current before launch | Attempt Disposition is `refused`; do not launch and create no observation. `held` is unavailable because this is a failed authorization boundary, not a valid `known_limited` or `unknown` assessment. |
| Authorization becomes non-current after partial or completed collection but before final closure | Preserve exact attempt and observation provenance, but refuse new Contracts truth. A fresh authorized run cannot retroactively adopt or upgrade those observations. |
| Authorization changes after truth closes | Preserve the closed result. Any present reliance universally resolves the exact Binding and historical validity, then follows its admitted `historical_validity_only` or `continuing_current` mode. Only the latter requires current Engine authorization for the Determination. A later effect still requires fresh separate action authorization and target checks; a fresh evaluation is required only when the selected reliance or authorization policy makes the changed Engine proposition a present dependency. |
| Approved escalation | Engine issues a fresh binding from the newly admitted envelope. Readiness, Prospective Evaluation Basis, attempts, evidence-set identity, and final evaluation context are fresh. Requirements remain unchanged unless separately reselected for an independent reason. |
| Refused escalation | Existing authorization remains unchanged if otherwise current; no requested reach or requirement changes. |
| Superseded request or binding | Follow Engine's exact successor relation. The superseded object remains historical and cannot authorize new collection or new evaluation truth. Present reliance may consume an already closed result only through universal historical Binding resolution/validity and the policy-selected `historical_validity_only` or `continuing_current` mode; the superseded binding never becomes action authority. |
| Partial collection or operational timeout | Record a distinct per-selection attempt terminal and any permitted exact partial observation. Do not fabricate completion, actual reach, evidence sufficiency, plan completion, or gate truth. A retry is another attempt for that selection. |
| Readiness cache reuse | Require exact Readiness Basis identity and fresh owner verification. Changed time/currentness, applicability knowledge, Observation Plan, role allocation, Producer Selection, capability, ceiling, host prerequisite, purpose, use, requirements, or authorization invalidates current reuse. |
| Current-result cache reuse | A cached result may supply only exact immutable historical truth. A new present-use decision requires an exact Operational Reliance occurrence, universal historical Binding resolution/validity, the policy-selected `historical_validity_only` or `continuing_current` checks, every other declared current dependency, and complete accounting. A cached Determination cannot authorize an effect; fresh action authorization and target checks remain separate. |
| Historical-result cache reuse | Permit exact immutable replay or explanation only, with exact historical Binding resolution and owner-attributed resolution outcome. Label any present reliance, current Engine status, disclosure limit, and action authority separately. |

No cache can convert historical validity into current authorization, make a
policy omit its required mode, convert evaluation authorization into action
authorization, replace an owner verification, merge or substitute selections,
attempts, or evidence roots, or make a diagnostic result decision-seeking.

## Advisory and enforceable behavior

Evaluation Coordination produces semantic owner results. A CLI, skill, or
protocol-only consumer may display `attemptable`, `held`, or `refused`, preserve
the exact reasons, and decline an operation routed through itself. Such behavior
is advisory outside that consumer's own call boundary. Agent instructions and
protocol acknowledgements do not prove that filesystem, network, process,
memory, CPU, tool, or other host restrictions were enforced.

That evaluation boundary does not impose a general Handbook identity,
credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
or ABAC platform. Engine Context Resolution still owns semantic admission and
verification of its Evaluation Authorization Binding. Separately, a standalone
CLI/SDK action path may carry user-prompt or configured-agent-role intent,
claimed principal/delegation, and declared **advisory** action authority when an
exact downstream Action Authorization policy permits that mode. Those claims do
not become Engine evaluation authorization, independently authenticated
identity/delegation, or Host Enforcement Facts; ambient operating-system
permissions are real execution conditions but are not fine-grained Handbook
credentials. Advisory authority cannot satisfy a policy that requires stronger
owner-verified or host-enforced facts, and it cannot waive the action target,
currentness, or commit-time checks fixed by the operational-reliance companion.

An enforcing host or future Substrate adopter may launch only from one exact
current Prospective Evaluation Basis, translate only constraints it actually
supports, apply or refuse those constraints, and emit exact Host Enforcement
Facts and attempt provenance. It MUST NOT infer untranslatable enforcement,
change assurance requirements, broaden Engine authorization, reinterpret gate
truth, or acquire posture-transition authority. This record selects no host,
adopter, constraint translation, sandbox, tool protocol, or enforcement
mechanism.

## Responsibility map

| Responsibility | Semantic owner | MUST NOT own or infer |
| --- | --- | --- |
| Assurance requirements | Posture and assurance-policy owners select the immutable proof plan before capability comparison. | Engine authority, readiness, easier requirements from current capability, evidence truth, or host enforcement. |
| Contracts-domain evaluation coordination | The broader Contracts domain's distinct coordination subresponsibility forms Observation Plan and Producer Selections, closes them into Evaluation Purpose, and forms Readiness Basis and Assessment, Attempt Disposition, and Prospective Evaluation Basis; it owns exact identity, cardinality, role accounting, and per-selection retry/reuse rules for those objects. | Engine admission or issuance, capability self-attestation, evidence admission or claim/metric/gate truth, operational enforcement, or mutation. |
| Engine Context Resolution | Engine admits complete envelopes, proves request non-expansion, issues Evaluation Authorization Bindings, preserves private lineage, and owns verification, historical validity, currentness, and supersession. | Assurance selection, producer capability, readiness comparison, evidence adequacy, gate truth, host enforcement, or a general Handbook IAM platform. |
| Producer-capability authority | A named independent owner admits versioned Producer Semantic Capability facts and their currentness. | Engine authorization, actual reach, evidence sufficiency, or permission to launch. |
| Dock or adapter | Owns its admitted operating ceiling, request compatibility, invocation protocol, and honest operational terminals. | Engine authority, universal producer capability, host enforcement, canonical evidence, or semantic truth. |
| Enforcing host | Applies or refuses supported runtime restrictions and records exact Host Enforcement Facts and attempt effects. | Resolution semantics, assurance selection, readiness, evidence admission, gate interpretation, or invented enforcement. |
| Producer | Performs one exact attempt and reports source observations and Actual Observed Reach with provenance. | Canonical evidence admission, readiness reclassification, claim adequacy, gate truth, or authority expansion. |
| Contracts-domain evidence and evaluation subresponsibilities | Sibling subresponsibilities admit candidates, fix the evidence-set identity, close the final evaluation context with the exact Prospective Evaluation Basis and Engine verification, and own claim/metric/gate truth under the companion record. | Engine currentness or authorization issuance; producer capability or host enforcement; recomputing or reinterpreting coordination-owned readiness; or requirement relaxation. |
| Work Progression and other Operational Reliance owners | Resolve exact historical results and Binding validity, select the explicit `historical_validity_only` or `continuing_current` mode, close target/currentness/accounting, and produce one atomic Determination with paired eligibility/disposition facets. Work Progression is the proposed semantic owner for local and parent eligibility. | Rewriting historical truth, relabeling diagnostic provenance, treating readiness as a gate, issuing action authority, or mutating state. |
| Action requester and action-authorization owner | The requester owns one exact Action Intent. The authorization owner independently produces an Action Authorization Binding or Refusal with exact advisory, verified, or host-enforced authority provenance and every declared fresh condition. | Treating Evaluation Authorization as action authorization, fabricating authentication/delegation/enforcement, improving reliance eligibility, or mandating a general Handbook IAM implementation. |
| Action executor and authoritative state/effect owner | The executor owns one exact Action Attempt Identity and Terminal. The state/effect owner performs through-commit expected-state and authorization checks and alone issues the Mutation Receipt for a committed change. | Ordinary retry after uncertain commit, broadening the authorized effect, or reporting mutation success without the authoritative receipt. |
| SDK, CLI, Tauri, skills, and other transports | Preserve and project exact owner results, correlation, presentation, interface-local failures, and prompt- or role-supplied advisory-authority claims with their provenance. | Semantic formation, owner verification, capability inference, authentication by assertion, task-wide enforcement, or truth reinterpretation. |

## Discriminating scenario corpus

Each scenario holds the selected requirements fixed and assumes exact admitted
identities except where the row identifies a failure.

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Ready multi-producer decision-seeking run.** One exact Observation Plan assigns schema conformance to selection A and runtime side effects to selection B. Valid, admitted, current facts positively prove for every role and selection that authorized/requested reach is congruent, capability represents the role/dialect, the operating ceiling permits the attempt, host prerequisites can be met, and the declared two-selection composition can be attempted. | Readiness is `ready` with complete per-selection/role accounting, disposition is `attemptable`, and one Prospective Evaluation Basis may form. Attempts bind A or B exactly; collection success, evidence sufficiency, pass, and all other semantic truth remain unproved. | Readiness requires positive proof across the complete plan without treating producers as interchangeable or predicting an evaluation outcome. |
| **Known-limited decision-seeking run.** Every formation input is valid, admitted, and current, but selection B's exact operating ceiling positively cannot meet its assigned gate-relevant requirement. | Requirements and assignments remain unchanged; readiness is `known_limited`; disposition is `held`; no basis forms and no selection launches. | A valid limiting fact produces `known_limited` and cannot relax assurance. |
| **Known-limited diagnostic run.** The same request is deliberately re-formed with `diagnostic` intent and Engine issues a congruent binding. | A new purpose, binding, readiness basis, assessment, and diagnostic Prospective Evaluation Basis form. Any later canonical truth retains diagnostic provenance and cannot support positive reliance unless an exact downstream policy permits it. | Diagnostics are exact new provenance, not a weaker gate or relabeling. |
| **Valid but inconclusive capability knowledge.** Every required formation input is valid, admitted, and current, but selection B's admitted capability fact explicitly leaves a necessary semantic limit unknown, so the comparison proves neither adequate nor inadequate reach. | Readiness is `unknown`. Decision-seeking is held; a diagnostic purpose may be attemptable. Unknown never defaults to ready. | Valid insufficient knowledge is distinct from missing or invalid input. |
| **Invalid readiness formation.** A required capability record is missing or stale, or a Producer Selection names a role/dialect outside the admitted capability contract. | Readiness Refusal is produced; there is no Readiness Assessment, basis, launch, evidence insufficiency, or gate truth. | Missing, invalid, and unsupported formation inputs never become `unknown` or `known_limited`. |
| **Coordinates without Engine binding.** Stack and coordinate values are valid, and a Dock accepts them, but no binding exists. | Authorization or basis formation refuses before launch. No readiness assessment, observation, or semantic truth is fabricated. | Structural equality and Dock acceptance are not authority. |
| **Same coordinates, different lineage.** A real binding is reused for another subject, objective, parent, constraint closure, use intent, or same-labelled changed stack. | Engine or congruence checking refuses. A fresh purpose and binding are required. | Authorization is purpose-bound and non-transferable. |
| **Plan changed after purpose formation.** The subject and coordinate labels are unchanged, but selection B, one role assignment, declared multiplicity, or one requested reach differs from the Observation Plan identity bound by the Evaluation Purpose. | Engine authorization formation or later congruence refuses the mismatch. A fresh Observation Plan, Evaluation Purpose, authorization request/binding, readiness closure, and Prospective Evaluation Basis are required. | A purpose closes one exact plan; apparent coordinate equality cannot authorize plan substitution. |
| **Current Dock equality.** Requested reach, manifest ceiling, and reported completed actual reach are equal, but Engine authorization is missing. | The Dock operating contract may be internally coherent, but no valid Prospective Evaluation Basis or canonical evaluation truth can result. | Equality proves neither Engine authority nor host enforcement. |
| **Ceiling above request.** An admitted producer/adapter ceiling is broader than the exact authorized request, and separate valid current facts positively establish every other readiness condition for the complete plan. | Readiness is `ready`; the request remains the operative bound and actual reach may not exceed it. No automatic widening occurs. | A permissive ceiling participates in positive readiness proof but is not permission to widen. |
| **Honest partial observation.** Selection B's producer policy permits partial completion and its attempt reports actual reach below that selection's request after a ready launch. | The exact B-bound attempt and partial observation may proceed to admission if otherwise valid. Contracts decides claim-specific eligibility under the declared plan relation and may later establish indeterminacy; readiness is not rewritten retroactively and A's observations are not merged implicitly. | Actual reach, selection identity, and pre-launch readiness are different facts. |
| **Actual exceeds a bound.** A producer reports reach above request, Engine authorization, or admitted operating ceiling. | Candidate admission refuses; the value is not clamped. No claim, metric, or gate truth is produced from it. | Actual-observation honesty is fail closed. |
| **Authorization superseded before launch.** Engine reports the binding superseded at the launch checkpoint. | Disposition refuses; no selection launches. The immutable binding remains historically issued and retains its successor relation. | Issuance, currentness, and supersession are separate facts. |
| **Authorization changes after collection.** Exact observations exist, but Engine reports the binding non-current before final Contracts closure. | Attempt and observation history remain immutable; final closure refuses new claim, metric, and gate truth. A fresh binding cannot retroactively authorize the old collection. | Pre-launch permission does not bypass the pre-truth checkpoint. |
| **Authorization changes after a pass.** A valid `pass` closed before the binding became non-current. One Operational Reliance Policy declares `historical_validity_only`; another declares `continuing_current`. | The historical `pass` and canonical explanation remain unchanged, and both reliance paths must resolve the exact Binding and historical validity. The first may form its Determination without treating current Engine status as an input when every other dependency closes. The second refuses absent a fresh positive Engine current verification and requires a fresh evaluation when its policy requires one. Any effect under either path still requires a distinct fresh Action Authorization Binding, target verification, attempt, and state-owner receipt. | Historical validity, policy-selected continuing-current reliance, and fresh action authorization are separate propositions. |
| **Approved escalation.** The proof plan is unchanged, and Engine approves broader reach after a prior basis exists. | Engine issues a fresh binding; readiness, basis, attempts, evidence set, and final context are fresh. Prior evidence and results are not upgraded. | Escalation is not in-place expansion. |
| **Per-selection retry after timeout.** Selection B's attempt times out after an exact permitted partial observation; its producer policy permits retry under the still-current same basis, while selection A already completed. | A's attempt remains unchanged. B's partial terminal remains immutable and a new B-bound attempt identity is created; it cannot substitute for A or merge implicitly with either prior attempt. Exact delivery replay alone is idempotent. | Retry is per selection, not plan-wide overwrite or duplicate authority. |
| **Stale readiness cache.** Authorization is still current, but producer capability or operating-ceiling currentness changed after assessment. | Cached readiness cannot support launch. A fresh Readiness Basis and Assessment are required. | Readiness cache identity includes all independent capability facts. |
| **Protocol-only restriction.** A CLI passes an exact requested reach and the process acknowledges it, but no host applies a corresponding restriction. | The request and acknowledgement remain advisory/operational facts. No Host Enforcement Fact may claim the restriction was enforced. | Declared intent is not host enforcement. |
| **Host cannot enforce.** A selected attempt requires a translatable restriction the actual host cannot apply. | The host refuses launch or downstream policy holds. It does not silently run, weaken requirements, or claim enforcement. | Only the actual host proves applied restriction. |

## Settled versus deferred

This record settles the canonical vocabulary; complete pre-truth stage order;
the Contracts-domain Evaluation Coordination semantic owner placement;
Observation Plan and Producer Selection closure; purpose, authorization,
readiness, disposition, basis, and per-selection attempt identity/cardinality;
the exact `known_limited`/`unknown`/Readiness Refusal boundary; Engine issuance,
projection privacy, verification, historical-validity, currentness, and
supersession responsibilities; mandatory current verification at collection and
final truth closure; universal historical Binding resolution/validity plus the
explicit `historical_validity_only` / `continuing_current` reliance-time rule;
separate fresh action authorization; readiness states and refusal boundary;
diagnostic versus decision-seeking behavior; capability/ceiling/host/actual-
reach separation; retry, partial collection, cache, escalation, supersession,
and reuse rules; advisory versus enforceable behavior without a general
Handbook IAM mandate; the responsibility map; and every scenario outcome above.

Deliberately deferred to later human-approved task-level planning are concrete
schemas, types, APIs, operations, storage, registries, transports, status-code
encodings, algorithms, cache mechanisms, retry counts, timeouts, producer
selection, capability policies, host constraints, enforcement mechanisms,
crate/module placement, migrations, feature flags, slice or packet
decomposition, acceptance planning, and rollout. Concrete assurance mappings,
requirements, metrics, thresholds, gate policies, concrete explanation-
projection decisions, UI, parent-promotion policy content, posture-transition
policy content, publication, and external adopter or Substrate selection remain
owned elsewhere or deferred. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
owns detailed present-use and action-chain semantics and proposes Work
Progression as local/parent eligibility owner; physical placement, work hierarchy
and state model, concrete policies, and implementation remain deferred.

Human approval of this proposed record would make it semantic input to later
planning only. It would not authorize implementation, change any phase or slice
status, or establish a delivery outcome.
~~~~~~~~~~~~

## evaluation-identity-evidence-closure-semantic-decision.md

SHA-256: `49f98b80d85414d8d8fc6ccdd3cca811cfb74aa759755c8dd62369dca99c7ff1`

~~~~~~~~~~~~markdown
# HCM evaluation identity and evidence-closure semantic decision

**Status: PROPOSED**

This record fixes five coupled greenfield seams in the Handbook Contract
Membrane: evaluation-occurrence identity, stable Producer Selection versus
checkpoint-current facts, prospective applicability before plan closure,
canonical evidence-set formation and closure, and the semantic-time and
historical-resolvability rules needed to replay the resulting truth.

It is a narrow companion to the proposed
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
[posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
Those records retain their object classifications, semantic owners, assurance,
authorization, readiness, authored relationships, claim, metric, gate,
projection, reliance, and action boundaries. This record normatively refines
only identities and closure relations that those records deliberately left
open.

This record is semantic authority for human review only. It is not
implementation authority, a slice or packet specification, a persistence
design, or rollout planning. Current source is ownership and trust-boundary
evidence, not a compatibility obligation. The product is greenfield; no V1 or
legacy identity, migration, alias, bridge, or dual behavior is selected.

## Purpose and scope

The exact lifecycle is:

```text
Resolved Assurance Requirements
  -> Prospective Applicability Basis, when any expected claim may be prospectively not applicable
  -> Observation Plan with exact Producer Selections
  -> Evaluation Purpose containing Evaluation Occurrence Identity and use intent
  -> Evaluation Authorization Request
  -> Engine Evaluation Authorization Binding
  -> Readiness Basis and Readiness Assessment
  -> Attempt Disposition
  -> Prospective Evaluation Basis, when attemptable
  -> Producer Attempts, source observations, and candidate/submission occurrences
  -> Occurrence Collection Frontier
  -> terminal completion of every Contracts admission operation causally attributable to an in-frontier submission
  -> Canonical Evidence Set Formation and Closure
  -> final Contracts evaluation-context closure
  -> final applicability, claims, metrics, semantic gate, and canonical explanation
  -> action-specific Operational Reliance Basis and atomic Determination, if used now
  -> separate Action Intent / authorization / attempt / terminal / receipt, if an effect is requested
```

Allocation of an Evaluation Occurrence Identity is part of Evaluation Purpose
formation, not a separate lifecycle stage. Evidence admission may occur more
than once before or after the collection frontier, but every admission causally
descending from an in-frontier candidate MUST be terminal before set formation.
Final evaluation-context formation may occur only after the one exact set
closure it consumes.

This record settles:

1. how exact replay differs from a deliberately new evaluation;
2. which Producer Selection changes reform the plan and purpose, and which
   current-fact changes reform readiness only;
3. when positive pre-existing applicability facts may establish an expected
   claim prospectively not applicable and thereby omit its observation
   relations before the plan closes;
4. how Contracts closes one complete, non-ambient evidence set before any
   claim evaluator selects from it; and
5. which semantic times and historical dependencies must remain resolvable.

The proposed
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md)
owns detailed authored fields and relationship roles beyond the exact subject,
scope, and snapshot references required by these closures. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
owns present-use reliance, atomic eligibility/disposition, local closeout,
parent eligibility, action authorization, attempts, terminals, receipts, and
mutation boundaries. Those proposal links do not approve either record or
select physical placement, policies, hierarchy/state models, or implementation.

## Canonical vocabulary

Terms in separate rows are not synonyms.

| Term | Fixed meaning |
| --- | --- |
| **Evaluation Occurrence Identity** | The Contracts Evaluation Coordination-owned identity that distinguishes one deliberately intended evaluation occurrence from every other occurrence, even when their semantic inputs are otherwise equal. It is allocated as part of Evaluation Purpose formation, remains bound through every later object in that occurrence, and is neither content-derived nor a transport correlation. |
| **Evaluation Content Fingerprint** | A deterministic fingerprint over the exact semantic formation content of an Evaluation Purpose, excluding Evaluation Occurrence Identity, the Purpose's own owner-issued record identity, formation/delivery metadata, and transport correlation. Referenced semantic-owner identities remain part of the content. Equal fingerprints mean equal declared evaluation content, not the same occurrence, authorization, attempt, evidence set, or result. |
| **Exact formation replay** | Re-presentation or reconstruction of the same owner formation request under the same Evaluation Occurrence Identity and exact semantic content. It must return the same owner record or exact idempotent outcome; it does not create a new evaluation. |
| **Delivery replay** | Re-delivery of the same exact message or bytes under interface-local correlation. It may be deduplicated at that boundary but has no power to create, merge, or distinguish semantic occurrences. |
| **Deliberately new evaluation** | A new Evaluation Occurrence Identity and therefore a new Evaluation Purpose, request, Engine authorization issuance path, readiness closure, Prospective Evaluation Basis, evidence-set closure, and final context, even when its Evaluation Content Fingerprint equals an earlier one. |
| **Stable Producer Selection closure** | The immutable semantic portion of a Producer Selection: exact assigned observation-role relation, producer kind and configuration, Dock or adapter identity when applicable, Requested Observation Reach, exact admitted Producer Semantic Capability definition reference, exact operating-ceiling definition reference, declared host-prerequisite definitions, exact retry-policy and partial-completion-policy definition references when either behavior is permitted, and plan multiplicity/composition relation. It contains no checkpoint-current availability or prerequisite-satisfaction observation. |
| **Checkpoint-current producer fact** | An owner-produced observation at a named Readiness or launch semantic time about the currentness or applicability of a selected capability, ceiling, retry, or partial-completion definition; current producer availability; host-prerequisite satisfaction; or another declared operational condition. It cannot change the stable selection it verifies. |
| **Prospective Applicability Policy** | An immutable, named, versioned Contracts definition that can establish, solely from an exact closed set of pre-existing canonical facts, that one expected claim is positively not applicable to the exact subject/snapshot and scope before that claim's observation obligations are planned. It neither establishes the final Applicability Result nor prunes only part of an otherwise applicable claim. |
| **Eligible prospective fact boundary** | The exact finite, non-ambient semantic-owner, repository or resolver, fact-kind, proposition-scope, subject/snapshot, applicability-scope, causal/semantic-time-cut, currentness, reuse-restriction, and deterministic inclusion/exclusion boundary from which prospective applicability is decided. |
| **Prospective Applicability Basis** | The Contracts-owned immutable closure over one Resolved Assurance Requirements identity, subject/snapshot and scope, one Prospective Applicability Policy, one eligible prospective fact boundary, applicability semantic time, exactly one prospective result for every expected claim, and complete derived observation-relation and fact accounting. |
| **Prospective prune** | Positive policy proof that one exact expected claim is not applicable to the exact subject/snapshot and scope. Every observation-role relation required solely for that claim is omitted for that claim. A shared role remains planned whenever another retained claim requires it. Missing, unknown, ambiguous, conflicting, stale, unsupported, or evaluation-produced information retains the claim and every relation required to evaluate it. |
| **Occurrence Collection Frontier** | The immutable Evaluation Coordination-owned terminal record for one Evaluation Occurrence Identity and one committed Prospective Evaluation Basis. It binds the exact owner-produced launch, refusal, attempt, source, submission, and terminal provenance required by every Producer Selection and plan relation; the exact selection-bound retry and partial-completion policies; and one causally closed frontier after which no new valid attempt, source observation, or candidate/submission occurrence may be created for that occurrence. It establishes no candidate admission, evidence membership, applicability, claim, metric, gate, or operational truth. |
| **Eligible evidence repository boundary** | The exact owner-repository identities, committed Prospective Evaluation Basis, subject/snapshot, scope, admitted evidence kinds, sealed Occurrence Collection Frontier, terminal admission-result population causally attributable to candidate/submission occurrences inside that frontier, and precommitted semantic-time rule from which Canonical Evidence Set Formation must account. It is a semantic membership boundary, not a storage technology or ambient query. |
| **Canonical Evidence Set Formation Policy** | An immutable, named, versioned Contracts definition that fixes the eligible repository boundary, closure preconditions, membership rule, per-selection and plan-relation accounting, retry and duplicate treatment, and set semantics. It cannot select evidence according to a desired claim or gate outcome. |
| **Canonical Evidence Set Formation** | The Contracts-owned deterministic operation that accounts for every required plan relation, attempt, candidate/submission occurrence, admission result, and canonical evidence record in the exact eligible boundary before producing either one closure or a formation refusal. |
| **Canonical Evidence Set Closure** | The immutable Contracts-produced record that binds the exact formation policy, eligible boundary, Occurrence Collection Frontier, unordered canonical evidence membership, complete formation-member/nonmember and terminal accounting, closure semantic time, and closure identity. It establishes no applicability, claim, metric, or gate truth. |
| **Final evaluation context** | The Contracts-owned closure formed only after the Canonical Evidence Set Closure exists. It binds that closure, the exact Prospective Evaluation Basis and occurrence, all required definitions and policies, the final-closure Engine verification, and the exact evaluation semantic time before truth evaluation begins. |
| **Semantic time** | A named owner-attributed temporal coordinate or causal checkpoint used by a semantic rule. It is part of provenance and currentness evaluation; it is not assumed to be a globally comparable wall-clock timestamp. |
| **Historical resolvability** | The ability to resolve an exact historical identity to its owner-recognized semantic content, owner provenance, applicable semantic-time relation, and required verification outcome for explanation or deterministic replay. It does not itself make the object current, grant present authority, or decide eligibility for an action. |
| **Exact historical replay** | Deterministic reconstruction of the same owner results from the same occurrence, exact historical inputs, definitions, policies, membership closures, semantic times, and owner verification facts. It performs no new observation and grants no present authority. |

Evaluation Occurrence Identity, Evaluation Content Fingerprint, Evaluation
Purpose identity, Evaluation Authorization Request identity, Engine binding
identity, Prospective Evaluation Basis identity, Producer Attempt identity,
Canonical Evidence Set Closure identity, and final evaluation-context identity
are distinct. No one of them may substitute for another merely because their
objects transitively bind common inputs.

## Evaluation occurrence and purpose identity

Evaluation Coordination MUST allocate one Evaluation Occurrence Identity only
as part of forming an exact Evaluation Purpose. The purpose identity MUST bind:

- the Evaluation Occurrence Identity;
- the Evaluation Content Fingerprint;
- exact subject/snapshot and scope;
- objective and Evaluation Use Intent;
- Resolved Assurance Requirements;
- the Prospective Applicability Basis when one influenced plan formation;
- the complete Observation Plan and Producer Selection identities; and
- the exact Canonical Evidence Set Formation Policy identity and
  configuration;
- every exact precommitted semantic-time or boundary-selection rule that can
  affect collection closure, evidence membership, evidence currentness, final
  applicability, claim truth, metric truth, or gate truth; and
- purpose-formation semantic time and owner provenance.

The content fingerprint MUST exclude the occurrence identity so two deliberate
evaluations with equal semantic content remain recognizable as equal-content
but different occurrences. The complete purpose identity MUST include both, so
those occurrences can never share authorization, attempts, evidence, closure,
or truth by implication.

The occurrence identity is semantic and owner-produced. It MUST NOT be
inferred from content, timestamps, a process identifier, an idempotency key, a
CLI or Tauri correlation, a Dock invocation, a random transport nonce, file
location, or evidence membership. Concrete allocation mechanics remain
deferred.

For one Evaluation Occurrence Identity:

- the exact same semantic formation content is exact formation replay and MUST
  return the same Evaluation Purpose identity and content;
- different semantic formation content MUST refuse as conflicting reuse;
- an exact delivery replay cannot allocate another occurrence or purpose; and
- a caller requesting a genuinely new evaluation MUST request deliberately new
  formation, and Evaluation Coordination MUST allocate the new occurrence
  identity even when every semantic-content input is equal.

Changing subject/snapshot, scope, objective, use intent, requirements,
Prospective Applicability Basis, Observation Plan, Producer Selection, role
assignment, composition/multiplicity, or Requested Observation Reach changes
purpose content and therefore requires a new occurrence identity, purpose,
authorization request and binding, readiness closure, and prospective basis.
The Formation Policy and every truth-affecting semantic-time or boundary rule
above participate in purpose semantic content and the Evaluation Content
Fingerprint. Changing any one likewise requires the complete new occurrence
chain.

Before occurrence commitment, Evaluation Coordination MAY form a fresh
Readiness Basis, Readiness Assessment or Refusal, Attempt Disposition, and
Prospective Evaluation Basis under the same occurrence only when all of the
following remain exact: subject/snapshot, scope, objective, Evaluation Use
Intent, Resolved Assurance Requirements, any Prospective Applicability Basis
and prune-supporting fact closure, Observation Plan, every Producer Selection,
every stable producer, Dock, prerequisite, retry, and partial-completion
definition, Evaluation Purpose, Evaluation Authorization Binding, and every
precommitted formation or semantic-time rule. Every dependency requiring
current-use verification at the refreshed pre-launch checkpoint MUST have a
positive exact owner verification, including Engine verification of the same
binding. Only checkpoint-current facts that change no stable proposition may
differ.

Every earlier Prospective Evaluation Basis remains immutable and MUST be
recorded as unused and ineligible for launch. It is not overwritten or
relabeled as the later basis.

An occurrence commits to one Prospective Evaluation Basis when any semantic
owner closes durable launch, launch-refusal, Host Enforcement Fact, Producer
Attempt, Dock invocation, source-observation, candidate/submission, or other
collection-effect provenance under that basis. After commitment, the occurrence
cannot adopt another Prospective Evaluation Basis. A change requiring another
basis terminates or abandons that occurrence without new truth; later
collection requires a deliberately new evaluation. An exact permitted retry
remains under the committed basis only while every bound dependency remains
exact and current under its owner rules.

## Stable selection and checkpoint-current facts

Producer Selection MUST bind stable semantic definitions and assignments, not
the transient result of checking them. Its stable closure includes exactly the
role relation, producer/configuration, applicable Dock or adapter, requested
reach, admitted capability-definition reference, operating-ceiling-definition
reference, declared host-prerequisite definitions, exact retry-policy and
partial-completion-policy definition references when applicable, and declared
composition or multiplicity relation.

The Readiness Basis separately binds, at its named semantic time:

- currentness/applicability verification for each exact capability definition;
- currentness/applicability verification for each exact operating-ceiling
  definition;
- currentness/applicability verification for every exact retry-policy and
  partial-completion-policy definition required by the selection;
- producer availability and any exact selected endpoint or instance facts the
  stable selection permits;
- actual host-prerequisite satisfaction and supported enforcement facts needed
  prospectively for launch;
- Engine authorization verification; and
- the owner identity, semantic time, and exact content of every verification or
  observation used.

A checkpoint-current fact may verify, limit, leave unknown, or refuse readiness
for the stable selection. It MUST NOT silently substitute a producer,
configuration, role, adapter, reach, capability definition, ceiling definition,
prerequisite definition, or plan relation.

An endpoint or producer instance may remain a checkpoint-current fact only
when the stable selection explicitly permits that bounded choice and its exact
identity does not change semantic capability, source authority, configuration,
role compatibility, or requested reach. If any of those propositions changes,
the producer/configuration changes and the stable selection MUST reform.

The reformation rule is closed:

| Change | Earliest closure that MUST change | What need not change when still exact and current |
| --- | --- | --- |
| Role assignment, producer kind/configuration, Dock/adapter identity, requested reach, capability, ceiling, prerequisite, retry, or partial-completion semantic definition, or multiplicity/composition relation | Producer Selection, Observation Plan, Evaluation Purpose with a new occurrence identity, request/binding, readiness, and Prospective Evaluation Basis | Nothing downstream may be reused merely because content looks compatible. |
| New currentness verification over the same exact capability, ceiling, retry, or partial-completion definition | Readiness Basis, Assessment, Attempt Disposition, and any later Prospective Evaluation Basis | Observation Plan, Purpose, occurrence identity, and binding need not reform only under the complete unchanged and positively current pre-commit rule above. |
| Availability or prerequisite-satisfaction change within the unchanged stable selection | Readiness Basis, Assessment, Attempt Disposition, and any later Prospective Evaluation Basis | The stable Selection, Plan, Purpose, occurrence, and binding need not reform only under that same pre-commit rule. |
| A verification establishes that the selected definition is no longer applicable and a successor definition must be selected | Producer Selection and the complete downstream chain, with a new occurrence identity | An historical reference may remain resolvable but cannot be swapped in place. |
| A current fact changes after occurrence commitment | The existing occurrence retains its exact history but cannot form new truth from a replacement basis; subsequent collection is a deliberately new evaluation | Permitted retries under the unchanged exact basis remain in the occurrence while every dependency stays exact and current. |
| Transport, delivery correlation, rendering, or projection changes | No semantic closure | Interface-local delivery behavior only. |

Availability is not capability. Capability-definition currentness is not
operating-ceiling currentness. A prerequisite definition is not proof that the
prerequisite is satisfied. None is Engine authorization or a Host Enforcement
Fact.

Every Prospective Evaluation Basis MUST transitively bind, through its exact
Producer Selections and Readiness Basis, each retry and partial-completion
definition plus the checkpoint verification that permitted its use. A copied
boolean such as `retry_allowed` or `partial_allowed` is not a substitute for
those owner identities and semantics.

## Prospective applicability before plan closure

A Prospective Applicability Basis is required whenever an expected claim would
otherwise have its observation obligations omitted because it is considered
not applicable before collection. If no claim is prospectively pruned,
Evaluation Coordination MAY omit this basis only by retaining every expected
claim and every requirement-derived observation relation in the Observation
Plan and recording that no prospective applicability pruning occurred.

The basis MUST bind:

- one exact Resolved Assurance Requirements identity and its complete expected-
  claim and observation-relation inventory;
- exact subject/snapshot and scope identities;
- one exact Prospective Applicability Policy identity, version, semantic
  content, entailment-level compatibility, and currentness verification;
- one exact eligible prospective fact boundary;
- an exact finite membership of pre-existing canonical fact identities, their
  semantic owners, proposition scopes, currentness, reuse restrictions, and
  inclusion or exclusion reasons at the applicability semantic time;
- exactly one result for every expected claim: `retained`,
  `prospectively_not_applicable`, or `unresolved_and_retained`;
- complete derivation and accounting of every resulting retained or omitted
  observation-role relation, without changing the requirements or expected-
  claim inventory; and
- complete decisive reasons, included/excluded facts, and conflict accounting.

The eligible prospective fact boundary MUST identify the exact semantic owners
and repositories or resolvers consulted; admissible fact kinds and proposition
scopes; subject/snapshot and applicability scope; causal or semantic-time cut;
every required owner verification and reuse restriction; and deterministic
inclusion and exclusion rules. Every owner-recognized fact matching that
boundary MUST be included exactly once or excluded with an exact policy-derived
reason. If boundary completeness, fact identity, owner attribution, proposition
scope, currentness, reuse restriction, or conflict accounting cannot be
established, the affected claim remains retained.

The policy MUST declare exact compatibility with the final claim-applicability
semantics bound by the same Resolved Assurance Requirements. For the same
expected claim, subject/snapshot, scope, bound facts, and applicable time
relation, the prospective proof MUST be sufficient to entail final
`not_applicable`. Shape compatibility or equal labels are insufficient.

Only exact canonical facts that existed independently before this evaluation
occurrence may support a prune. Their semantic owners establish exact identity,
content, proposition, scope, required currentness, and any owner-declared reuse
limitation. Contracts alone applies the Prospective Applicability Policy and
establishes the prospective result; an owner fact does not establish its own
prune. Ambient repository state, mutable prose, a projection, a prior result
without an admitted reuse rule, anticipated evidence, producer capability,
availability, authorization, readiness, cost, inconvenience, or missing data
cannot support a prune.

An unresolved, stale, conflicting, unsupported, missing, or ambiguous fact
retains the expected claim and every observation relation required to evaluate
it. It does not prove `not_applicable`. Positive proof for one whole expected
claim omits every relation required solely for that claim. A shared observation
role remains in the plan whenever another retained claim requires it.

The Observation Plan MUST bind the exact Prospective Applicability Basis when
one exists and completely account for every expected claim and every derived
retained or omitted relation. Changing the policy, eligible boundary,
subject/snapshot, scope, fact membership, fact currentness or reuse basis,
semantic time, result, compatibility, or accounting requires a new basis, plan,
and complete new occurrence chain.

Final applicability remains Contracts-owned semantic truth inside the final
evaluation context. Congruence is mandatory:

- every `prospectively_not_applicable` claim may publish only final positive
  `not_applicable` under the exact compatible final semantics; `applicable` or
  `undetermined` produces an evaluation refusal because its required
  observation obligations were omitted;
- every retained claim may later be `applicable`, `not_applicable`, or
  `undetermined` because none of its required observation obligations was
  removed; and
- changed facts or policy may not retroactively justify an earlier prune.

A congruence failure produces no Applicability Result, claim, metric, gate, or
canonical explanation truth for that context. It is not evidence insufficiency,
`not_computable`, `indeterminate`, or an implicit waiver.

Relation-level pruning of an otherwise applicable claim is not selected. Such
behavior requires a separately defined Observation Obligation Applicability
result and matching final relation-level congruence semantics.

## Occurrence collection frontier

After collection activity for the committed Prospective Evaluation Basis is
finished, Evaluation Coordination MUST form exactly one Occurrence Collection
Frontier before evidence-set formation. It may form the frontier only from the
precommitted Plan, stable Producer Selections, composition/multiplicity rules,
retry and partial-completion policies, and exact origin-owner launch, refusal,
attempt, source, submission, and terminal records. It MUST NOT invent a
producer, Dock, host, source, submission, or terminal fact.

The frontier MUST:

- bind the one Evaluation Occurrence Identity and one committed Prospective
  Evaluation Basis;
- bind every Producer Selection and requirement-derived plan relation;
- bind each exact retry and partial-completion definition plus the owner
  verifications that governed collection;
- account for every permitted or refused launch, attempt, source observation,
  candidate/submission occurrence, partial/completed terminal, and no-attempt
  terminal required by the plan;
- prove that the actual attempt/submission topology conforms to the plan and
  every selection-bound policy; and
- close one causal frontier after which no new valid attempt, source
  observation, or candidate/submission occurrence can be added to the
  occurrence.

The frontier closes occurrence-level collection, not Contracts admission.
Admission results bind their earlier candidate/source/attempt, occurrence,
basis, subject/snapshot, scope, selection relation, admission policy, and
admission semantic time. They MUST NOT bind the identity of a future frontier,
evidence closure, or closure cut. After the frontier forms, every Contracts
admission operation causally attributable to an in-frontier candidate MUST
reach an exact terminal owner result before evidence-set formation can close.

One Evaluation Occurrence Identity may have multiple immutable unused
pre-commit Prospective Evaluation Bases, but at most one committed Prospective
Evaluation Basis, one Occurrence Collection Frontier, one Canonical Evidence
Set Closure, and one final evaluation-context/result chain. Exact replay returns
the same identities and content. Different successful frontier, closure, or
final-context content under an already committed or closed occurrence refuses;
it cannot create a second truth for that occurrence.

## Canonical evidence-set formation and closure

Canonical Evidence Set Formation is a Contracts evidence responsibility,
separate from candidate admission and claim evaluation. It begins only from an
exact committed Prospective Evaluation Basis, its sealed Occurrence Collection
Frontier, terminal admission accounting for every in-frontier candidate, and
the precommitted Canonical Evidence Set Formation Policy bound by the Purpose.
It MUST form an exact eligible evidence repository boundary rather than query
ambient current state.

The eligible boundary MUST fix:

- owner repository identities and exact admissible evidence namespaces;
- Evaluation Occurrence Identity and committed Prospective Evaluation Basis;
- subject/snapshot and scope;
- complete Observation Plan, Producer Selection, and plan-relation identities;
- the exact sealed Occurrence Collection Frontier;
- every terminal admission result causally attributable to its in-frontier
  candidates;
- closure semantic time, the precommitted time-selection rule, and every named
  cross-owner time-comparison basis; and
- admitted canonical evidence kinds and deterministic inclusion/exclusion
  rules.

Membership is deterministic and non-discretionary. Every canonical evidence
record whose immutable admission result binds the same occurrence, Prospective
Evaluation Basis, subject/snapshot, scope, and supported Producer Selection or
plan relation, and whose admission is causally attributable to an in-frontier
candidate in the eligible repository boundary, MUST be a member exactly once.
An evidence record or admission result MUST NOT bind the later Canonical
Evidence Set Closure or its causal-cut identity. Claim desirability, finding
direction, metric effect, gate effect, producer preference, arrival order, or
evaluator choice cannot change membership.

Formation MUST account for every Producer Selection and every required plan
relation, including:

- zero or more exact Producer Attempts permitted by the selection-bound retry
  policy;
- every attempt terminal, including refusal, timeout, cancellation, partial,
  and completed states under the exact partial-completion policy;
- every source observation and durable origin-closed candidate/submission
  occurrence inside the frontier;
- every Contracts admission replay, refusal, or idempotent admission outcome;
- every admitted canonical evidence record;
- every durable origin-closed duplicate attempt, source-observation, or
  candidate/submission occurrence inside the frontier;
- every formation nonmember or exclusion, with an exact reason; and
- whether the plan's multiplicity/composition and prospective-pruning
  accounting is complete.

Raw transport delivery replay, interface-local correlation, framing retries,
and adapter-local duplicate handling are not semantic occurrence records and
are not required in formation accounting. They enter that accounting only when
an appropriate origin owner independently closes a bounded durable occurrence
record. Such a record still cannot create a second evidence member from the
same exact canonical evidence identity.

Complete accounting does not assert that every selection produced sufficient
evidence. A selection may close with no attempt because launch was refused, or
with attempts that failed, remained partial, produced refused candidates, or
produced evidence later insufficient for a claim. The closure records those
facts exactly; claim evaluation decides their semantic consequence only after
the final context exists.

The Formation Policy defines deterministic accounting of retries, duplicates,
and partial terminals; the formation responsibility applies it. Neither may
grant a retry, authorize partial completion, redefine a selection's completion
contract, or treat a nonconforming topology as valid merely because it is fully
recorded. An unauthorized extra attempt, impermissible partial result, or
violated multiplicity/composition relation causes formation refusal.

Retry, duplicate, and identity rules are exact:

- raw delivery replay has no semantic member effect;
- same canonical evidence identity and bytes admits idempotently to one member;
- same identity with different content refuses and remains refusal accounting;
- every non-idempotent retry is a distinct attempt and all its durable
  observations, submissions, admission results, and admitted evidence remain
  separately accounted without replacing an earlier attempt; and
- all matching admitted records are members. Later claim policy may resolve
  conflicts or select a compatible subset only within that closed membership.

Canonical evidence membership has set semantics over exact canonical evidence
identity, not byte equality, value equality, or a content fingerprint alone.
Distinct admitted evidence identities remain distinct members even when their
values or encodings are equal. Multiple duplicate submission or admission
occurrences may account to one member only when the canonical evidence identity
itself is the same. Provenance accounting separately preserves the exact
multiplicity and causal graph of attempts, observations, submissions,
admissions, refusals, terminals, and formation exclusions.

Serialization may impose one deterministic canonical order for fingerprinting
and presentation, but arrival, repository, producer, retry, or serialization
order has no semantic precedence. When a claim or sampling policy needs a
sequence, that exact sequence relation must be present in the admitted evidence
or another named canonical input; it is not inferred from set ordering.

A **formation nonmember or exclusion** lies outside set membership under the
Formation Policy because it is outside the boundary, was refused, or otherwise
fails the deterministic membership rule. A **claim-policy nonselection** is an
existing member that one Evidence Evaluation Policy does not select for one
claim-evaluation role. Claim-policy nonselection MUST NOT alter, reopen, or
remove set membership.

Formation MUST refuse rather than close when:

- the frontier is absent, ambiguous, not causally closed, or inconsistent with
  an owner terminal;
- repository, occurrence, basis, subject, scope, selection, frontier, or
  semantic-time identity is absent, ambiguous, mixed, substituted, or ambient;
- any required selection or plan relation lacks terminal accounting;
- any candidate or admission operation causally descended from an in-frontier
  candidate remains open;
- actual attempt/submission topology violates the Plan, stable Selection,
  retry, partial-completion, multiplicity, or composition rules;
- an admitted matching record is omitted or appears more than once;
- a member belongs to another occurrence, Prospective Evaluation Basis,
  subject/snapshot, scope, or unsupported plan relation;
- retry, durable-duplicate, partial, prune, or formation-exclusion accounting
  is incomplete;
- the precommitted Formation Policy or any required owner identity cannot be
  resolved under the exact required owner observations at closure; or
- membership depends on a claim finding, metric, gate, or desired outcome.

Before a successful Closure exists, formation may return only the one Closure
determined by the precommitted policy and exact sealed frontier or a formation
refusal. Once closed, membership is immutable. Re-presentation of the same
exact inputs returns the same Closure; any different policy, boundary, member,
terminal, exclusion, accounting, or semantic-time content under the same
occurrence refuses. A different Closure or final evaluation requires a
deliberately new occurrence.

An attempt, source observation, or candidate/submission occurrence causally
after the sealed frontier is outside that Closure and remains bound to the
occurrence, Prospective Evaluation Basis, attempt, source, and owner under which
it actually occurred; it cannot be rebound to another occurrence. Its origin
owner may preserve it only as exact post-frontier or invalid occurrence
provenance, not as a valid addition to the sealed collection. An admission
result or matching canonical evidence record discovered after Closure but
causally descended from an in-frontier candidate proves that admission work was
open or matching membership was omitted when Closure formed. That is a closure-
integrity violation, not permissible late evidence and not a basis for mutating
membership.

Exact replay of an already accounted occurrence changes nothing. New semantic
truth requires a deliberately new evaluation and new occurrence-bound
collection. No cross-occurrence transfer or automatic reuse of an old-
occurrence record is authorized.

Claim evaluators MUST receive the one exact Canonical Evidence Set Closure from
the final evaluation context. An Evidence Evaluation Policy may select eligible
members from that already closed set and must account for every selected and
nonselected member. It cannot expand the set, query ambient repositories,
reopen formation, import a late record, or define the set whose result it is
intended to evaluate.

## Final evaluation-context closure

The final evaluation context forms only after Canonical Evidence Set Closure.
It MUST bind exactly:

- Evaluation Occurrence Identity and Evaluation Purpose;
- Resolved Assurance Requirements and any Prospective Applicability Basis;
- Observation Plan and every Producer Selection;
- every selection-bound retry and partial-completion definition and its exact
  Readiness/launch checkpoint verification;
- Evaluation Authorization Binding and the exact final-closure Authorization
  Verification Observation;
- Readiness Basis and Assessment, Attempt Disposition, and Prospective
  Evaluation Basis;
- Occurrence Collection Frontier and every terminal admission result in its
  eligible causal population;
- Canonical Evidence Set Formation Policy and Closure;
- contract/claim/applicability/evidence-policy/evaluator/metric/gate
  definitions and exact configurations required by the resolved requirements;
- final evaluation semantic time and every named time/currentness relation; and
- complete cross-owner identity and pre-truth closure accounting.

Context formation MUST refuse before semantic truth when any required identity
cannot be resolved, any closure is mixed or open, the evidence closure is
incomplete, or Engine's required final-closure verification is not positive.
After a valid context exists, Contracts determines final applicability and
checks it against any prospective prune before publishing an Applicability
Result. Incongruence produces an evaluation refusal and no Applicability
Result, claim evaluation, metric result, semantic gate decision, or canonical
explanation; it does not retroactively make the already exact context or
evidence-set membership ambiguous.

One final context consumes one occurrence and one Prospective Evaluation Basis.
It cannot combine occurrences, purposes, prospective bases, or evidence-set
closures even when their content fingerprints, subjects, policies, or members
appear equal. At most one final evaluation-context/result chain may close for
the occurrence; exact replay returns it, while different content refuses.

## Idempotency and replay matrix

| Input or event | Required semantic behavior |
| --- | --- |
| Same occurrence identity and exact purpose-formation content | Exact formation replay returns the same Purpose identity and content. |
| Same occurrence identity with different formation content | Refuse conflicting reuse; do not mutate the earlier Purpose. |
| Request for deliberately new formation with an equal Evaluation Content Fingerprint | Evaluation Coordination allocates a new occurrence identity and forms a new Purpose and downstream chain. No earlier authorization, attempt, evidence, or result transfers. |
| Same semantic occurrence under a different transport correlation | Preserve one semantic occurrence; correlation remains interface-local. |
| Exact authorization-request delivery replay | Engine may return the same issuance outcome for the exact request identity and bytes; currentness still requires a separate checkpoint verification. |
| Same stable semantic chain with only a new checkpoint-current observation before occurrence commitment | Keep the Plan/Purpose/binding only under the complete unchanged and positive-currentness rule; form fresh readiness and any new Prospective Evaluation Basis. Record every earlier basis immutable, unused, and ineligible for launch. |
| Stable selection definition, role, producer/configuration, reach, adapter, prerequisite, retry, partial-completion, or composition change | New Selection, Plan, occurrence, Purpose, request/binding, readiness, and prospective basis. |
| Permitted retry under one unchanged current Prospective Evaluation Basis | New attempt identity in the same occurrence and selection; exact delivery replay alone remains idempotent. |
| Fact change requiring a new prospective basis after occurrence commitment | Preserve and terminate or abandon the occurrence without new truth; subsequent collection is a deliberately new evaluation. |
| Raw candidate delivery replay | Interface-local handling creates no required semantic occurrence. If an origin owner closes a durable duplicate submission, account for that occurrence without creating another evidence member from the same exact evidence identity. |
| Same candidate or evidence identity with different content | Refuse; never overwrite, merge, or select the favorable variant. |
| Re-run of set formation over the exact sealed frontier, closed boundary, and policy | Return the same closure identity, membership, and accounting. |
| Same closure identity with different membership/accounting | Refuse as corruption or substitution. |
| Late activity after closure | Preserve every record under its original occurrence and basis. Post-frontier source activity is outside the Closure; omitted in-frontier admission or evidence demonstrates a closure-integrity violation. Neither changes membership or permits rebinding. New truth requires a new occurrence and new occurrence-bound evidence. |
| Exact historical replay | Resolve the same historical chain and reproduce the same owner results without new current authority or collection. It neither grants nor denies operational reliance. |
| Present re-evaluation of the same subject and definitions | New occurrence, new Purpose and authorization path, new readiness/basis, new evidence closure, and new final context. |

## Semantic times and currentness

Semantic times are named owner facts. A timestamp string alone does not prove
ordering, comparability, freshness, or currentness. Every policy that compares
times across owners MUST name the admitted comparison basis or causal relation.
The lifecycle always preserves exact predecessor identities even when no global
total order is available.

Any rule for selecting a semantic time or causal frontier that can affect
membership, currentness, applicability, or truth MUST be fixed under an exact
named owner definition before the values whose direction it can affect are
available. The actual owner-issued checkpoint may form later at its proper
stage, but it MUST NOT be retrospectively selected from multiple permissible
times after inspecting candidate, evidence, claim, metric, or gate direction.

| Named semantic time or checkpoint | Owner and required binding | Currentness or reformation consequence |
| --- | --- | --- |
| **Requirements selection time** | Assurance Selection binds the snapshot, selector, policies, definitions, exceptions, and their currentness basis. | A different requirements identity starts a new downstream chain. |
| **Prospective applicability time** | Contracts applicability responsibility binds the exact pre-existing fact cut and policy used before plan closure. | Changed facts, policy, scope, subject, or time basis that affects pruning requires a new applicability basis, plan, and occurrence. |
| **Plan and purpose formation time** | Evaluation Coordination binds exact plan content, occurrence identity, use intent, and formation provenance. | Content change requires a new occurrence; exact replay preserves the original time and result. |
| **Authorization issuance time** | Engine binds exact request/purpose/plan and private authority lineage. | Issuance remains historical; it is not a currentness observation. |
| **Authorization verification time** | Engine answers for the named launch, final-closure, or historical checkpoint. | A negative or unverifiable current-use result refuses the governed boundary without rewriting issuance or history. |
| **Readiness assessment time** | Evaluation Coordination binds current capability/ceiling verification, availability, prerequisites, Engine verification, and every other readiness fact. | Fresh facts over stable definitions produce a fresh readiness closure; changed stable semantics reform the plan chain. |
| **Attempt and observation time** | Producer, Dock, and host provenance bind exact occurrence, prospective basis, selection, invocation, terminals, actual reach, and enforcement facts. | Retries remain separate attempts within the same unchanged occurrence and basis; later facts do not rewrite earlier attempts. |
| **Occurrence collection-frontier time** | Evaluation Coordination binds the causally closed collection population and origin-owner terminals for the committed basis. | No later launch, attempt, source, or submission activity is validly added to that occurrence. Admission work for in-frontier candidates must still become terminal. |
| **Evidence admission time** | Contracts admission records the exact source-validity and policy decision for candidate-to-evidence transition. | Admission-time validity remains historical; it is not evaluation-time evidence currentness. |
| **Evidence-set closure time** | Contracts applies the precommitted rule to the sealed frontier, terminal admission population, membership, formation exclusions, and complete accounting. | Later records cannot enter the set. Different closure content under the occurrence refuses. |
| **Final evaluation time** | Contracts binds evidence currentness, final applicability, claim/metric/gate definitions, final Engine verification, and the complete context. | Results are immutable historical truth for this time and context. |
| **Operational reliance time** | The downstream reliance owner universally resolves the exact historical Evaluation Authorization Binding and validity outcome, then applies its admitted `historical_validity_only` or `continuing_current` mode plus every other declared present-use dependency. | Current Engine verification is required only by `continuing_current`; either mode preserves historical truth and cannot inherit evaluation authorization as action authority. A later effect has a distinct fresh Action Authorization Binding and commit-time checks. |

The causal order between named stages is mandatory even when their time values
are not numerically comparable. No owner may substitute its local receipt time
for another owner's semantic time or infer currentness from recency alone.
Every cross-owner comparison MUST bind the exact comparison-policy identity,
each input owner and semantic-time identity, the admitted causal or comparison
relation, the comparison owner, and its exact owner-produced result. Transport
receipt and local wall-clock values may be retained as provenance but cannot
silently substitute for another owner's semantic time.

## Historical resolvability and exact replay

Every retained final evaluation context and result that claims deterministic
explanation, audit, or replay MUST preserve or be able to resolve, for the same
declared retention horizon, the exact historical chain below:

- subject/snapshot and scope; Charter, posture snapshot, selector, policies,
  exceptions, Resolved Assurance Requirements, and their exact revisions when
  applicable;
- Prospective Applicability Policy, fact membership, Basis, results, and
  accounting, eligible fact boundary, and exact final-applicability
  compatibility when pruning occurred;
- Observation Plan, every Producer Selection and stable definition reference,
  including exact retry and partial-completion definitions,
  Evaluation Occurrence Identity, content fingerprint, Purpose, use intent, and
  authorization request occurrence, plus every precommitted formation,
  boundary-selection, and semantic-time rule;
- Engine Authorization Binding, issuance lineage sufficient for owner
  verification, verification observations, and supersession relations;
- Readiness Basis, every checkpoint-current owner fact, Assessment, Attempt
  Disposition, Prospective Evaluation Basis, and retry/partial-policy
  verification result;
- every Producer Attempt, source observation, Actual Observed Reach, Host
  Enforcement Fact, Dock terminal, candidate/submission occurrence, evidence-
  admission result/refusal, and canonical evidence record in the closure's
  accounting boundary;
- Occurrence Collection Frontier; Canonical Evidence Set Formation Policy,
  eligible boundary, exact membership, terminal and formation-exclusion
  accounting, and Closure;
- final evaluation context, applicability and claim results, Evidence
  Evaluation Policies, Metric Definitions/configurations and results, Gate
  Policy and decision, and complete canonical explanation; and
- each semantic owner's identity, exact semantic content/version,
  currentness-at-the-recorded-checkpoint fact, named semantic time, and any
  time-comparison or causal relation required to reproduce the decision.

Historical resolution MAY use retained content, an immutable owner repository,
an exact owner proof, or another later-authorized mechanism. This decision
chooses none. It MUST resolve the same owner-recognized semantics, not a current
replacement, nearest version, projection, rendered summary, cache guess, or
transport copy.

At final evaluation-context closure, Contracts MUST establish that every
required dependency is exactly identified and covered by an owner-declared
historical-resolution obligation compatible with the result's declared
retention horizon. An absent, ambiguous, substituted, or already unresolvable
dependency refuses closure.

A later owner outage may cause an exact owner-produced historical-verification
or resolution observation to report `unverifiable`, `unavailable`, or another
owner-defined non-success outcome. A permanent loss may violate that owner's
recorded retention obligation. Neither event rewrites the immutable historical
result or permits substitution of a similar current object.

A retained historical result may be projected only with the exact owner-
attributed resolution observations and an explicit statement that exact
historical replay is presently unavailable. This record introduces no new
cross-owner aggregate historical-verification status.

One canonical explanation remains source truth. A concise default human view and
requested deeper drill-down are deterministic projections over that same truth,
limited independently to the highest detail both available and authorized for
disclosure. Reduced or denied detail does not trigger reevaluation or create a
new semantic result. Concrete projection decisions, UI/API shape, and any fixed
number of density levels remain deferred.

Those observations and the retained result grant no present authority. Whether
any downstream action may rely on the result is decided exclusively by the
separately owned Operational Reliance Policy and its exact present-use inputs.
That policy universally requires historical resolution/validity and explicitly
selects `historical_validity_only` or `continuing_current` for present Engine
status. This record neither grants nor universally denies operational
eligibility, and any later effect requires separate fresh action authorization.

Exact historical replay uses the recorded semantic times and historical owner
facts. It MUST NOT rerun producers, substitute current definitions, re-evaluate
under a new semantic time, call a current authorization result historical, or
claim present authority. A present evaluation, even of equal content, is a
deliberately new occurrence.

## Semantic responsibility map

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Contracts-domain Evaluation Coordination | Evaluation Occurrence Identity allocation; Evaluation Content Fingerprint and Purpose formation; exact Plan/Purpose congruence; readiness reformation boundary; occurrence commitment to one prospective basis; one exact Occurrence Collection Frontier from origin-owner terminals. | Engine issuance/currentness, prospective or final applicability truth, invention of origin or admission terminals, evidence admission/set membership, claim/metric/gate truth, or operational reliance. |
| Contracts definition authorities | Deterministic Policy Admission Closure, immutable identity, compatibility, currentness, and historical resolution of Prospective Applicability Policies and Canonical Evidence Set Formation Policies, including exact relevant governance-source roles/scope/dependency modes and complete candidate accounting. | Treating authored candidate text or a bare link as admitted policy, recursively letting a candidate admit its own rule, or applying a policy to select a desired plan, set membership, applicability, claim, metric, or gate outcome. |
| Contracts applicability responsibility | Exact resolution and application of an already admitted Prospective Applicability Policy; Prospective Applicability Basis formation; final applicability; prospective/final congruence. | Policy self-admission, ambient or evidence-dependent plan pruning, producer selection, Engine authority, or requirement relaxation. |
| Engine Context Resolution | Evaluation Authorization Binding issuance, private lineage, named verification observations, historical validity, currentness, and supersession. | Occurrence allocation, content fingerprints, applicability, plan adequacy, readiness, evidence membership, or replay reconstruction by Contracts. |
| Producer Capability, Dock/adapter, prerequisite, retry, partial-completion, and availability owners | Their exact semantic definitions and checkpoint-current verification or occurrence facts under independently attributable authority. | Altering Producer Selection, assurance requirements, Engine authorization, readiness outcome, or evidence truth. |
| Contracts evidence admission | Candidate/source/provenance validation and immutable canonical evidence records with exact admission time and basis. | Selecting evidence-set membership by outcome, closing the set, claim truth, or rewriting origin provenance. |
| Contracts evidence-set formation responsibility | Exact resolution and application of the precommitted admitted Formation Policy; eligible boundary over the sealed frontier and terminal admissions; exact set membership; complete plan/attempt/candidate/admission/terminal/formation-exclusion accounting; closure time and identity. | Policy self-admission, candidate admission, claim-specific eligibility or nonselection, findings, metrics, gates, or ambient repository queries. |
| Contracts final-context and evaluation responsibilities | Final context closure, final applicability congruence, claim-policy selection/nonselection within the closed set, claim/metric/gate truth, and canonical explanation. | Reopening evidence membership, importing late evidence, reconstructing Engine authority, or operational mutation. |
| Producer, Dock, adapter, and enforcing host | Exact attempt, source, candidate-submission, terminal, actual-reach, and applied/refused enforcement provenance for their bounded propositions. | Canonical evidence, set closure, applicability, truth evaluation, or current-use authority. |
| Definition and repository owners | Exact historical definition/content resolution, owner provenance, semantic-time/currentness observations, and declared retention obligations for their objects. | Replacing an historical identity with a current equivalent or making a consumer the semantic owner. |
| SDK, CLI, Tauri, Flow, Pipeline, skills, and transports | Bounded composition, rendering, correlation, delivery, and deterministic projections or assemblies of exact owner objects. | Occurrence identity, owner formation replay, evidence membership, semantic-time invention, historical verification, or truth reinterpretation. |
| Work Progression and other Operational Reliance owners | Exact action question, finite result/target closure, universal historical Binding validity, explicit `historical_validity_only` or `continuing_current` mode, Basis, atomic Determination with paired eligibility/disposition facets, and Reliance Refusal. Work Progression is the proposed local/parent eligibility owner. | Rewriting this occurrence's identity, evidence closure, historical truth, issuing action authority, or mutating state. |
| Action requester, authorization, executor, and state/effect owners | Exact Action Intent; Action Authorization Binding or Refusal with declared advisory/verified/enforced provenance; Action Attempt Identity and Terminal; through-commit verification; and authoritative Mutation Receipt. | Treating Engine evaluation authorization as action authorization, fabricating identity/delegation/enforcement, ordinary retry after uncertain commit, or claiming committed mutation without the state-owner receipt. |

Every object introduced by this record is a canonical semantic definition, an
owner-produced canonical record/result, durable provenance, or a bounded value
inside one of those categories under the companion ontology. None is an
Artifact Instance. Physical colocation or shared repository primitives do not
merge semantic owners.

## Discriminating scenario corpus

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Equal content, deliberate rerun.** A user requests deliberately new formation for the same subject, requirements, plan, objective, and use intent after an earlier result. | The Evaluation Content Fingerprint may match, but Evaluation Coordination allocates a new Evaluation Occurrence Identity and creates a new Purpose, authorization path, readiness/basis, frontier, evidence closure, and final context. Nothing transfers automatically. | Semantic occurrence is owner-issued and not content identity. |
| **Exact purpose retry through another transport.** The same occurrence and exact formation content arrive under a different CLI/Tauri correlation. | Exact formation replay returns the same Purpose. The correlation creates no occurrence or authority. | Delivery and semantic idempotency are distinct. |
| **Occurrence key reused with changed plan.** One Producer Selection or requested reach changes while the occurrence identity is reused. | Purpose formation refuses conflicting reuse. A new occurrence and full downstream chain are required. | Occurrence identity cannot mask content substitution. |
| **Availability refresh before commitment.** Every upstream semantic input, precommitted formation/time rule, stable selection definition, prune-supporting fact, Purpose, and Engine binding remains exact and positively current; only permitted producer availability changes, and no durable launch or collection effect exists. | A fresh Readiness Basis/Assessment and prospective basis may form under the same plan, Purpose, and occurrence. The earlier basis remains immutable, unused, and ineligible for launch. | A bounded operational-fact refresh need not reform semantic selection. |
| **Capability version replaced.** Readiness discovers that the selected capability definition is no longer applicable and a successor is required. | Selection, Plan, occurrence, Purpose, Engine request/binding, readiness, and prospective basis reform. The successor is not substituted into the old selection. | Stable definitions and checkpoint facts have different change boundaries. |
| **Readiness changes after partial collection.** One attempt produced a partial observation before a prerequisite fact changed. | The occurrence retains exact partial history but cannot adopt a replacement prospective basis or close new truth. A later collection is a deliberately new evaluation. | One occurrence cannot mix prospective bases. |
| **Planning guess says inapplicable.** Project prose says a claim probably does not apply, but no named policy over a complete eligible canonical-fact boundary proves it. | The claim and every observation relation required to evaluate it remain in the Plan. Missing confidence cannot prune work. | Prospective pruning requires positive owner truth and complete fact accounting. |
| **Selective prospective facts.** One favorable fact would prove a claim not applicable, but another matching current fact inside the declared boundary conflicts with it. | Both facts are accounted. The claim is `unresolved_and_retained`; neither fact can be hidden to prune the claim. | A finite eligible-fact boundary prevents cherry-picking. |
| **Positive whole-claim prune with a shared role.** A named compatible policy over the complete exact fact boundary positively proves claim A not applicable; claim B remains retained and shares one observation role with A. | A is `prospectively_not_applicable`; relations required solely for A are omitted, but the shared role remains planned for B. The full expected-claim inventory remains unchanged. | Claim-level pruning derives complete relation accounting without inventing relation-level applicability. |
| **Prune contradicted at final applicability.** A plan omitted a claim's observation obligations under a bound prospective prune, but final applicability would be `applicable` or `undetermined`. | The already formed final context remains exact, but applicability processing refuses before publishing an Applicability Result, claim, metric, gate, or explanation truth. The omission cannot become evidence insufficiency. | Prospective proof must entail final claim-level non-applicability. |
| **Formation rule chosen after observations.** Two admitted Formation Policies would produce different membership, and the requester tries to choose after seeing the evidence direction. | Purpose formation or later congruence refuses because the exact policy and truth-affecting time/boundary rules were not precommitted in the Purpose. | Evidence-set rules cannot be selected to fit an outcome. |
| **Two retries disagree.** Two distinct permitted attempts under one selection yield admitted canonical evidence with opposing values. | Both evidence records are members of the closed set. The claim's named consistency/evidence policy accounts for the conflict; formation cannot select the favorable retry. | Set closure precedes and constrains claim selection. |
| **Raw and durable duplicate submission.** The same candidate bytes are delivered twice at transport, and the source owner separately closes two durable duplicate submission occurrences under one exact candidate identity. | Raw delivery replay requires no semantic record. Both durable submission occurrences and their admission outcomes are accounted, while the same canonical evidence identity remains one set member. Different content under that identity refuses. | Transport replay, provenance multiplicity, and evidence identity are distinct. |
| **Completed plan with no evidence for one selection.** A durable host refusal terminal exists for one required selection, the frontier accounts for every relation, and all in-frontier admission activity is terminal. | Evidence-set closure may record zero evidence for that selection with complete refusal accounting. Later claim evaluation may be indeterminate; closure itself does not fabricate sufficiency. | Closure completeness is complete accounting, not successful collection. |
| **Unauthorized extra retry.** Every attempt is recorded, but one retry or partial terminal violates the exact selection-bound retry or partial-completion definition. | Frontier or formation refuses despite complete recording. The Formation Policy cannot retroactively grant the attempt or redefine completion. | Complete provenance cannot make a nonconforming topology valid. |
| **Ambient repository query.** A formation request says to use “all current evidence” without exact repositories, a sealed frontier, terminal admissions, or precommitted time rules. | Formation refuses and no set closure or final context forms. | Membership must be finite, exact, terminal, and non-ambient. |
| **Post-frontier favorable source.** A source observation or candidate occurs after the sealed frontier and would change a metric. | It remains provenance of the original occurrence and basis but is outside the Closure and cannot be rebound. New truth requires new occurrence-bound collection. | A later event cannot rewrite closure or provenance. |
| **Omitted in-frontier admission.** A matching canonical evidence record discovered after Closure descends from an in-frontier candidate whose admission was open or omitted. | This proves a closure-integrity violation. The set is not mutated, and the record is not legitimate late evidence. | The frontier must be followed by terminal, complete admission accounting. |
| **Arrival-order policy.** A claim evaluator asks to use the last-arriving member even though no canonical sequence relation exists. | The request refuses; canonical serialization order and arrival order carry no semantic precedence. | Evidence membership is a set unless an exact source relation supplies sequence. |
| **Cross-owner clock labels disagree.** Two local timestamps appear ordered, but their owners supply no admitted comparison policy or causal relation. | No cross-owner ordering or currentness conclusion is drawn. Any later comparison binds its policy, input time identities and owners, causal relation, and result. | Local timestamps do not create a global semantic clock. |
| **Historical policy unavailable.** A retained gate result references a policy version whose owner currently reports an exact unavailable-resolution outcome. | The result is not rewritten or replaced by a similar policy. Its projection carries the owner observation and states that exact replay is presently unavailable. This record makes no operational-reliance decision. | Historical identity, owner resolution, replay availability, and present use are distinct. |
| **Exact historical replay.** Every exact owner dependency and recorded semantic time resolves successfully. | The same closure, claim/metric/gate truth, and explanation are reproduced without producer execution or new authority. | Replay is historical reconstruction, not reevaluation. |
| **Projection offered as history.** A compact gate view is available but its canonical explanation or evidence closure cannot be resolved. | The projection cannot repair the missing owner chain or serve as historical replay input. | Projection identity never replaces source-owner resolvability. |

## Settled versus deferred

This record settles:

- Evaluation Occurrence Identity, Evaluation Content Fingerprint, Purpose
  identity, exact formation replay, delivery replay, and deliberately new
  evaluation as non-substitutable concepts;
- occurrence commitment to one Prospective Evaluation Basis once collection
  effects begin, the complete bounded pre-commit readiness-refresh rule, and
  immutable unused prospective bases;
- stable Producer Selection closure versus checkpoint-current capability,
  ceiling, availability, prerequisite, retry, partial-completion, host, and
  Engine verification facts;
- the exact reformation boundary for definition changes versus current-fact
  refreshes;
- Contracts-owned Prospective Applicability Policy/Basis, positive-proof-only
  whole-claim pruning, exact finite eligible-fact boundary, entailment-level
  final-policy compatibility, complete derived relation accounting, and
  mandatory congruence with final claim applicability, while deferring
  relation-level obligation applicability;
- Evaluation Coordination-owned Occurrence Collection Frontier, terminal
  admission sequencing, one committed basis/frontier/closure/final chain per
  occurrence, and causal-direction separation from earlier admission records;
- Contracts-owned Canonical Evidence Set Formation Policy, eligible boundary,
  deterministic membership, complete attempt/candidate/admission/terminal and
  formation-exclusion accounting, retry and duplicate treatment, evidence-
  identity uniqueness, set semantics, closure time, immutable closure, and
  refusal for ambient, ambiguous, mixed, nonconforming, or open membership;
- claim-policy nonselection only within the already closed set, without
  changing membership, and final context only after closure;
- the named semantic-time/checkpoint vocabulary and prohibition on assuming a
  global clock or substituting receipt time, plus exact precommitment and
  cross-owner comparison rules; and
- the minimum full-chain historical-resolvability and exact-replay invariant,
  preserving exact owner non-success observations without creating an aggregate
  status, rewriting historical truth, or deciding operational eligibility.

The proposed
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md)
owns concrete relationship semantics, references, and requiredness for Charter,
Project Context, Environment Context, Work Specification, Decision Record, and
Risk Record. This record binds only exact subject/snapshot and scope identities
supplied through their proper owners and does not approve that companion.

The proposed
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
owns Current-Use Eligibility and Operational Disposition as facets of one
atomic Determination, Operational Reliance Basis and Refusal, local closeout,
parent eligibility, explicit `historical_validity_only` versus
`continuing_current` present-use modes, action authorization, attempts,
terminals, receipts, and mutation boundaries. Historical resolvability and
replay grant none of them. Work Progression is only the proposed semantic owner
of local/parent eligibility; physical placement, work hierarchy/state model,
concrete policies, and implementation remain deferred.

Also deferred to later human-approved task-level planning are concrete schemas,
Rust types, APIs, identity allocation algorithms, timestamps or logical-clock
technology, storage layouts, repository implementations, retention durations,
registry and resolver mechanics, cache implementation, status-code encoding,
transports, crate/module placement, retry counts, timeouts, policy catalogs,
task slices, acceptance planning, and rollout. No concrete applicability,
evidence-formation, claim, metric, gate, or producer policy is selected.
Cross-occurrence evidence import or reuse and operational repair, quarantine,
or incident response for a later-proven closure-integrity violation are also
deferred and are not implied by complete historical accounting.

Human approval of this proposed record would make it semantic input to later
planning only. It would not authorize implementation, change any phase or
slice status, or establish a delivery outcome.
~~~~~~~~~~~~

## metric-gate-semantic-decision.md

SHA-256: `7c2dc1f4eb43f6569edc547163aa9270cfb6a3461db4b85bdfc5e570abec54c1`

~~~~~~~~~~~~markdown
# HCM metric, evaluation, and gate semantic decision

**Status: PROPOSED**

This record fixes the product vocabulary, responsibility boundaries, numeric
meaning, and discriminating scenarios for the Handbook Contract Membrane
metric/evaluation/gate seam as a greenfield semantic decision. It is not
implementation authority until a human approves it and later task-level
planning separately authorizes delivery.

Current source and the completed [HCM-5.1](slices/HCM-5.1/README.md) and
[HCM-5.2](slices/HCM-5.2/README.md) authorities are evidence only for semantic
ownership, dependency direction, and trust boundaries. Their current public
shapes and computations neither constrain nor supply this model.

This record is a companion to the proposed
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
[posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
and [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
The posture record owns the proposed upstream selection seam; the operational-
reliance record owns the proposed downstream present-use and action seam. When
posture influences assurance selection, the exact snapshot and resolved-
requirement provenance are part of this record's closed evaluation context.
All seven records remain proposals and supply no implementation authority.

## Canonical model and vocabulary

The canonical semantic chain is:

```text
resolved assurance requirements (when applicable)
  -> prospective evaluation purpose and requested observation reach
  -> Engine-owned Evaluation Authorization Binding
  -> separately owned readiness determination
  -> Prospective Evaluation Basis
  -> source observation
  -> evidence candidate
  -> Contracts-owned evidence admission
  -> canonical evidence record
  -> exact canonical evidence-set identity
  -> final Contracts evaluation-context closure
  -> applicability and claim evaluation
  -> zero or more named metric results
  -> gate policy
  -> semantic gate decision and canonical explanation
  -> action-specific Operational Reliance Basis, when used operationally
  -> one atomic Operational Reliance Determination
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> separate Action Intent and Action Authorization Binding or Refusal
  -> Action Attempt, Action Terminal, and state-owner Mutation Receipt only after committed mutation
```

The following terms are normative. Terms in one row are not synonyms for terms
in another row.

| Term | Fixed meaning |
| --- | --- |
| **Source observation** | A source provider's bounded account of what it observed, including source-native value, unit or scale, subject/snapshot identity, collection state, and provenance. It is not canonical evidence and cannot satisfy a claim by itself. |
| **Evidence candidate** | A dock- or adapter-produced, evidence-shaped submission that binds one or more source observations to one exact Prospective Evaluation Basis. It is untrusted input to admission. Do not call it an evidence record or shorten it to “evidence” where that would imply admission. |
| **Canonical evidence record** | The immutable, content-addressed record produced only by Contracts-owned admission after exact identity, provenance, authority, subject, Prospective Evaluation Basis, unit/scale, normalization, admission-time source validity, and duplicate checks. “Evidence record” means this admitted object. |
| **Evidence Evaluation Policy** | An immutable, named, versioned Contracts-owned policy for exactly one declared claim-evaluation role: evidence eligibility/selection, sufficiency, freshness/currentness, consistency/stability, or uncertainty treatment. A selected contract, claim, or evidence requirement declares the roles and compatibility constraints it supports. A resolved-assurance selection may bind exact compatible policies to those roles; no policy is an umbrella override. |
| **Evaluation Authorization Binding** | The immutable, exactly identified, purpose-bounded Engine-owned result issued from one exact currently admitted `ContextResolutionEnvelope`. It transitively and opaquely binds that full envelope and its Engine authority/admission issuance basis while exposing only stable evaluation-relevant provenance: exact subject or scope, objective, intended evaluation use, Resolution stack semantics, requested observation reach, and Engine's determination that the request is admitted for that purpose. Its issuance is historical fact; Engine-owned currentness and supersession verification remain separate owner observations. |
| **Prospective Evaluation Basis** | The exactly identified pre-observation provenance closure for one prospective evaluation. It binds the exact evaluation purpose, subject or scope, objective and intended use, requested reach, Evaluation Authorization Binding, and every required resolved-assurance, readiness, and use-intent input. It contains no evidence-set identity and produces no claim, metric, gate, or other semantic truth. Its physical owner and lifecycle mechanics remain deferred. |
| **Applicability result** | The Contracts-owned determination, made before evidence selection, that a claim is `applicable`, `not_applicable`, or `undetermined`, with an exact basis. `not_applicable` is positive semantic proof, not missing data. |
| **Claim evaluation** | The complete result for exactly one expected claim in one evaluation context: its applicability result and, when applicable, an established finding or an indeterminate determination, plus application of every exact compatible Evidence Evaluation Policy binding, the independent evidence-assessment dimensions, and complete accounting. |
| **Finding** | The semantic satisfaction result established by a claim evaluator: `satisfied`, `unsatisfied`, or a value on a claim-declared named grade/measurement scale. There is no universal partial-satisfaction scalar. |
| **Metric definition** | An immutable, named, versioned, and fully configured algorithm definition over a declared population of compatible claim evaluations, with explicit input contract, scale, unit, aggregation, parameters, and missing/indeterminate-input policy. Applying it in an evaluation context is the invocation that produces a metric result. |
| **Metric result** | The exact output of one metric definition: `computed` with a value on its named scale, `not_applicable`, or `not_computable`, plus included/excluded inputs and a reproducible calculation trace. There is no unqualified “aggregate score.” |
| **Gate policy** | An immutable semantic decision rule over direct claim-evaluation predicates and named metric-result predicates, including condition precedence and treatment of indeterminacy. |
| **Semantic gate decision** | The non-mutating result `pass`, `fail`, `indeterminate`, or `not_applicable`, with every condition result and decisive reason. It answers the policy's bounded semantic question only. |
| **Operational Reliance Determination** | The separately owned, immutable action-domain result produced from one exact Operational Reliance Basis. It contains Current-Use Eligibility and Operational Disposition as two fixed facets for one occurrence, policy, action question, target, semantic time, and decisive accounting closure. It grants no action authority and performs no mutation. |
| **Disposition Reference** | A reference to one exact Operational Reliance Determination plus its exact Operational Disposition facet being displayed or documented. It is not a second result and cannot let the paired eligibility/disposition facets diverge. |

A final **evaluation context** is formed only after its exact canonical
evidence-set identity is fixed. It binds that evidence set; one contract
revision; subject snapshot; evaluation time; claim-evaluator versions; metric
definition identities and configurations; exact Evidence Evaluation Policy
bindings; gate-policy revision; and one exact Prospective Evaluation Basis,
whose identity transitively binds the Evaluation Authorization Binding and
pre-observation provenance. It also binds the Engine-produced authorization-
currentness basis required at final closure. Results that do not share that
closure cannot be combined.

Before observation, the Prospective Evaluation Basis MUST be closed without an
evidence-set identity. Producers and source observations, evidence candidates,
and admitted canonical evidence MUST retain its exact identity and the
authorization provenance it binds so another purpose, subject, scope,
objective, use, stack, request, requirements set, or readiness basis cannot be
substituted or transferred. Canonical evidence MUST NOT depend on the final
evaluation context that is formed only after the evidence set exists.

Engine alone issues the Evaluation Authorization Binding and owns the
underlying envelope admission, request-within-envelope decision, authority
admission, currentness or historical-validity determination, and supersession
semantics. Contracts MAY exact-check the binding's non-substitution and its
congruence with Contracts-owned context facts, including subject or scope,
purpose or intended use, exact stack semantics, and requested reach. Contracts
MUST NOT reconstruct, approximate, broaden, or replace any Engine decision
from coordinate values, rank comparisons, parent or constraint relations,
producer capability, Dock limits, or actual observations.

The Evaluation Authorization Binding identity MUST transitively preserve the
exact full-envelope and Engine authority/admission lineage without making the
full private envelope, credentials, witnesses, or Engine admission machinery
part of the Contracts domain. Sharing a serializable coordinate shape across
an Engine authorization bound, a request, a producer capability, a Dock or
adapter operating ceiling, an actual observation, or a claim requirement does
not make those facts the same semantic type or permit substitution among them.
Viewer disclosure authority, posture strictness, and explanation density are
separate again and do not change evaluation authorization or truth.

Missing, malformed, substituted, stale, superseded, or otherwise unverifiable
authorization before Prospective Evaluation Basis formation MUST prevent or
refuse producer launch and collection as required by the later lifecycle
decision. A change after evidence exists but before final evaluation-context
closure MUST refuse new claim, metric, or gate truth. This record does not
select the readiness or diagnostic state machine used to report those stops.

When posture influenced assurance selection, the evaluation context also fixes
the Resolved Posture Snapshot fingerprint; exact Assurance Policy Selector
identity, version, semantic-content identity, and currentness; exact admitted
policy-resolution basis identity and currentness; exact selected Assurance
Policy identity, version, semantic-content identity, and currentness; and
Resolved Assurance Requirements identity. The requirements identity
transitively binds the admitted exception inputs, every other declared
selection input, and complete selection accounting. Those bindings participate
in evaluation fingerprinting, currentness, and canonical explanation. Metrics
and gates consume the resolved requirements; they never reread or reinterpret
mutable posture, reselect requirements, or change metric meaning. This is the
cross-boundary invariant fixed with the
[companion posture/assurance record](posture-assurance-policy-semantic-decision.md).

A posture-influenced evaluation context is admissible only as the exact and
complete realization of one Resolved Assurance Requirements identity. That
identity selects one exact contract revision and one complete contract-rooted
expected-claim inventory. Any selected claim-set reference MUST be rooted in
that same contract revision. The evaluation subject snapshot MUST satisfy the
exact bound Posture Scope. The evaluation's contract revision, expected-claim
inventory, Evidence Evaluation Policy bindings, Metric Definitions and full
configurations, and Gate Policy MUST be exactly the selected outputs. Every
selected requirement and every metric or gate dependency MUST be closed and
completely accounted. An addition, omission, substitution, wrong-scope
binding, incompatible multiplicity, or unresolved dependency MUST refuse
before producing claim, metric, or semantic-gate truth; it MUST NOT become
ordinary `not_applicable`, `not_computable`, or `indeterminate` truth.

Prospective-basis closure, requirements-to-context conformance, and final
evaluation-context closure are preconditions at their respective stages. A
contract, scope, expected-claim, evaluator-context, selected-policy, Metric
Definition, or Gate Policy closure violation refuses and produces no claim
evaluation, metric result, or semantic gate decision. The same refusal applies
at the applicable provenance boundary when required authorization is missing,
malformed, substituted, bound to the wrong subject, scope, objective, use,
stack, or request, stale, superseded, or otherwise unverifiable by Engine.
These are Prospective Evaluation Basis or final evaluation-context closure
failures, not evidence insufficiency. `not_computable` and `indeterminate`
apply only after a valid final evaluation context has been established and the
declared evidence or claim state prevents computation or decision. Proven
`not_applicable` likewise remains available only inside a valid final context
under the positive-proof rules.

Requirements spanning unrelated contract revisions require separate Resolved
Assurance Requirements and evaluation contexts. Cross-context gate composition
would require a separately defined semantic owner and remains deferred; it MUST
NOT be inferred from plural references.

Admission-time source validity is an immutable historical fact about the
candidate-to-record transition. Evaluation-time currentness is recomputed for
the evaluation time under the claim's freshness policy; admission never makes a
durable claim-currentness decision.

## Independent dimensions

The model must retain these dimensions independently even when one artifact
contains several of them:

| Dimension | Allowed form | Aggregation and compensation rule |
| --- | --- | --- |
| Observed satisfaction / finding | Categorical satisfaction or a claim-declared named grade/measurement scale. An established finding is distinct from an indeterminate determination. | Scalar only when the named scale defines one. It cannot be inferred from evidence count, confidence, criticality, or gate outcome. |
| Applicability | `applicable`, `not_applicable`, or `undetermined`. | Never scalar or averaged. Only proven `not_applicable` removes the claim from an applicable population. |
| Sufficiency / coverage | Categorical sufficiency or an exact ratio over an explicitly named population. | Coverage may aggregate only population-compatible counts. Insufficiency cannot be converted to partial satisfaction or silently removed from a denominator. |
| Currentness | `current`, `stale`, or `unknown` against an explicit evaluation time and policy. | Never compensatory. Stale/unknown input cannot establish a current finding. |
| Consistency / stability | Categorical consistency, conflict, or stability result under a named resolver or sampling model. | Never a generic pass percentage. Conflicts are indeterminate unless the claim evaluator names a deterministic resolver. |
| Uncertainty / confidence | A method-specific exact interval, error bound, calibrated probability, or categorical unknown. | No universal confidence scalar or multiplier. It cannot multiply satisfaction, weight, criticality, or numerator contribution. |
| Criticality / severity | A category in an explicitly named taxonomy: claim impact/criticality or observed finding severity, as applicable. | Not intrinsically ordered, weighted, or a gate effect. It becomes scalar or aggregated only through an explicit compatible named rule. |
| Decision | The categorical semantic gate decision. | Never numeric or averaged. A direct non-compensatory condition cannot be offset by a metric or another condition. |

Repeated observations affect consistency or a named sampling model only when
their lineage and required independence are established. Repetition and
duplicate submission never increase confidence by default.

## Measurement and metric semantics

Every metric definition must state all of the following; omission makes the
definition invalid rather than supplying a default:

1. a stable metric name and version, plus the exact algorithm identity and
   version;
2. its complete configuration, including every parameter, coefficient, and
   constant;
3. the exact eligible-population rule and compatibility contract, including
   the subject, context, evaluator, time, scale, and unit closure required of
   every input;
4. every input and output scale, its direction, permitted operations, and
   bounds;
5. the quantity dimension and canonical unit wherever a value or coefficient
   is dimensioned;
6. the exact aggregation algorithm and semantics;
7. handling for `not_applicable`, indeterminate, insufficient, stale,
   conflicting, uncertain, missing, or incompatible inputs; and
8. complete included/excluded-input and calculation accounting, including
   conversions, intermediate values, bound checks, and the reason for any
   non-computed result.

There is no universal unnamed aggregate, default metric, or concrete metric
algorithm catalog selected by this record. A metric result always identifies
the exact named, versioned, configured definition that produced it.

Externally stable numeric values are exact. Finite decimals retain their exact
coefficient and scale; ratios retain exact numerator and positive denominator;
quantities retain exact value and unit; named categorical or ordinal scales
retain their scale identity and direction. No producer, adapter, metric, or
transport may implicitly coerce an IEEE-754 value, round a boundary, infer a
unit, average an ordinal scale, or collapse a quantity to a unitless number.
Exact conversions must be named, versioned, dimension-compatible, and visible
in the trace.

Every metric algorithm must declare finite public and intermediate numeric
bounds appropriate to its own semantics and representation. An out-of-bound
input, intermediate, or result refuses instead of clamping, rounding, wrapping,
or changing representation silently. Threshold comparison is exact and
declares whether equality passes.

A weight or other coefficient exists only as a parameter inside one named
metric definition. It is not intrinsic to a claim and never means obligation,
criticality, severity, confidence, producer trust, or gate effect. The metric
algorithm must declare each coefficient's meaning, dimension and unit,
permitted sign and treatment of zero, relative or absolute interpretation,
equivalence and normalization behavior, bounds, and role in aggregation. There
is no universal coefficient domain or normalization rule across algorithms.

Only when an algorithm defines dimensionless, strictly positive, relative
weights may exact rational or finite-decimal coefficients be normalized to a
primitive positive-integer vector. That normalization is valid only when it
preserves the algorithm's semantics and every declared public bound. Zero,
negative, absolute, or dimensioned coefficients must never be silently
coerced, discarded, or normalized under this rule.

An Evidence Evaluation Policy may fill only a role declared by the selected
contract, claim, or evidence requirement and only when its exact identity and
semantics satisfy that role's compatibility constraints. It MUST NOT change
canonical-admission authority, provenance, identity, or another non-relaxable
trust invariant; the claim proposition, finding scale, direction, or unit; a
Metric Definition; or Gate Policy meaning. Missing, incompatible, conflicting,
substituted, or unaccounted policy bindings refuse rather than falling back,
overriding claim semantics, or becoming an implicit relaxation.

A user, agent, authored artifact, or tool MAY author an Evidence Evaluation
Policy, Metric Definition, or Gate Policy candidate. It becomes an executable
category-2 definition or policy only after its semantic owner deterministically
produces the category-3 **Policy Admission Closure** defined by the companion
[canonical-object decision](canonical-object-ontology-and-ownership-semantic-decision.md).
That closure binds the exact candidate; typed vocabulary and compatibility;
finite selection/accounting; role and scope completeness; every relevant exact
Charter, Project Context, Decision Record, or Risk Record revision with its
proposition role, scope, and historical/formation-only or continuing-current
dependency mode; and all non-relaxable invariants. When the admission rule
requires an approval or exception, the closure separately binds the exact typed,
authorized source, its authority, scope, identity/version or revision as
applicable, and currentness. It requires only the governance families relevant
to the admission rule. Authored text, a prompt or agent role, artifact promotion,
and a bare Markdown link are governance input, not runtime evidence, a waiver,
or self-admission. “Unsafe” means mechanically prohibited by admitted
constraints, not an evaluator's open-ended organizational judgment; the exact
owner admission rule is a non-recursive root outside the candidate population it
decides.

## Responsibility map

| Layer | Owns | Must not decide |
| --- | --- | --- |
| Engine Context Resolution authorization | Exact envelope admission and non-expansion; issuance of the purpose-bounded Evaluation Authorization Binding; verification of its issuance lineage, underlying envelope, request-within-envelope relation, authority admission, currentness or historical validity, and supersession. | Assurance selection, producer capability or readiness, Contracts context congruence, evidence adequacy, claim truth, metrics, gates, explanation density, or downstream action. |
| Contracts-domain Evaluation Coordination | The proposed Contracts-domain coordination responsibility forms the exact pre-observation binding of purpose, subject/scope, objective/use, requested reach, Engine authorization, and required resolved-assurance/readiness/use-intent provenance. Concrete crate, module, API, persistence, and transport placement remain deferred. | Evidence-set identity, claim or metric truth, gate decision, Engine admission, requirements relaxation, or host enforcement by implication. |
| Validators and source providers | Bounded source-specific execution, source observation, source snapshot identity, native values/units/scales, and collection diagnostics. | Applicability, claim findings, canonical admission, metric weights/values, gate policy, or downstream action. |
| Docks and adapters | Protocol execution/framing, exact translation, preservation of source semantics and lexemes, exact Prospective Evaluation Basis provenance, producer operating-ceiling enforcement, candidate construction, and import diagnostics. | Engine authorization; treating request/ceiling/actual equality as authority proof; canonical evidence; invented units or scale direction; silent normalization; claim meaning; applicability; metrics; or gates. |
| Contracts evaluation-context closure | Final binding of the exact canonical evidence set, Prospective Evaluation Basis, Engine-produced authorization-currentness result, and all contract/policy/evaluator/metric/gate/posture provenance; refusal before semantic truth when closure or required Engine verification is absent or negative. | Forming the pre-observation basis, issuing authorization, or reconstructing Engine rank, parent, constraint, mutation, escalation, authority-admission, currentness, or supersession semantics. |
| Contracts evidence admission | Exact candidate/source/run/subject and Prospective Evaluation Basis binding; authority and provenance validation; canonical normalization; unit/scale identity and permitted exact conversion; idempotent duplicate identity; immutable canonical evidence. | Final evidence-bound evaluation-context closure, claim satisfaction, criticality, cross-claim aggregation, gate outcome, or workflow action. |
| Contracts-owned Evidence Evaluation Policy authority | Canonical establishment, exact re-admission validation, and exact resolution of immutable named/versioned role-specific policies; declared role and contract/claim compatibility; and the policy's bounded evidence-evaluation semantics. | Canonical-admission trust, claim proposition or finding-scale changes, assurance selection, metric or gate meaning, downstream disposition, or mutation. |
| Claim evaluation | Applicability; exact application of the selected compatible Evidence Evaluation Policy bindings; complete evidence selection/accounting; sufficiency, currentness, consistency, uncertainty treatment; semantic interpretation; and one finding or indeterminate determination per expected claim. | Policy invention or fallback, cross-claim metric aggregation, global thresholds, gate precedence, enforcement, posture, promotion, or mutation. |
| Contracts-owned named metric-definition authority | Canonical establishment, exact re-admission validation, and exact resolution of immutable named, versioned, configured metric definitions; validation of the eligible-population and compatibility rules, input/output scales and units, algorithm identity/version, parameters and coefficients under their algorithm-declared semantics and bounds, aggregation semantics, and missing/indeterminate-input policy. | Evidence selection, finding interpretation or establishment, the actual evaluation-run population, metric computation, threshold/comparator application, gate decision, or downstream mutation. |
| Metric computation | Pure exact application of one exactly resolved named metric definition to compatible claim evaluations in one closed evaluation context, including derivation and accounting of the actual included/excluded population and calculation result. | Raw-source/evidence selection, applicability or finding changes, definition changes, gate threshold/comparator application, gate decision, or downstream disposition. |
| Gate policy | Direct non-compensatory claim-evaluation predicates, named metric-result predicates, precedence, indeterminacy treatment, and the semantic gate decision. | Raw evidence interpretation, unit/scale conversion, changing metric semantics, canonical admission, or any workflow/posture/promotion mutation. |
| Work Progression and other Operational Reliance owners | Resolve exact historical results and Evaluation Authorization Binding validity; apply one admitted action-specific Operational Reliance Policy with explicit `historical_validity_only` or `continuing_current` Engine dependency mode; close exact target/currentness/accounting; and produce one atomic Operational Reliance Determination with paired eligibility/disposition facets or a Reliance Refusal. Work Progression is the proposed semantic owner for local-closeout and parent eligibility. | Rewriting the semantic gate or canonical explanation, treating a metric as the decision, issuing action authority, mutating state, or silently selecting work hierarchy/state policy or physical placement. |
| Action requester and action-authorization owner | The requester owns one exact Action Intent after `eligible / allow`. The exact action-authorization owner independently produces an Action Authorization Binding or Action Authorization Refusal with the declared advisory, verified, or host-enforced authority provenance and fresh conditions. | Treating Evaluation Authorization, a gate, `allow`, a prompt, ambient OS permissions, or artifact approval as independently authenticated identity, verified delegation, host-enforced permission, or a mandate for a general Handbook IAM platform. |
| Action executor and authoritative state/effect owner | The executor owns one exact Action Attempt Identity and Action Terminal. The authoritative state/effect owner performs attempt- and commit-bound expected-state, effect, authorization, and declared currentness checks and alone issues the Mutation Receipt for a committed change. | Ordinary retry after uncertain commit, broadening the authorized effect, inferring commit from an executor report, or reporting mutation success without the authoritative receipt. |
| SDK, CLI, Docks, hosts, and other transports | Preserve exact owner results and provenance, including policy-permitted prompt- or configured-role-supplied advisory claims. A real enforcing host may supply stronger independently verified facts. | Becoming a semantic owner by transport; fabricating authentication, delegation, or host enforcement; waiving target/currentness checks; or satisfying a policy that requires enforcement with advisory claims. |

## Gate and explanation invariants

- A direct claim-evaluation predicate is a non-compensatory predicate over
  exactly one claim evaluation, including its applicability, determination,
  finding, sufficiency, currentness, consistency, or uncertainty dimensions. A
  named metric-result predicate is the separate policy construct over one exact
  metric result. A direct predicate is never encoded only as a large weight.
- Metric results do not compensate for one another unless the exact named gate
  policy explicitly declares a compatible composite rule. There is no ambient
  averaging across metrics.
- A gate decision is pure derived semantic truth. It persists or mutates no
  lifecycle, work unit, posture, parent, promotion, release, or deployment
  state and grants no authority to do so.
- Producers, source observations, evidence candidates, and canonical evidence
  MUST retain the exact Prospective Evaluation Basis and its authorization
  provenance without depending on a final evidence-bound evaluation context.
  Claim evaluations, metric results, the semantic gate decision, and canonical
  explanation then bind the exact Evaluation Authorization Binding
  transitively through their one final closed evaluation context and its exact
  Prospective Evaluation Basis. No layer may copy selective coordinate values
  and thereby detach truth from Engine-owned authorization provenance. An
  authorization-related refusal MUST retain any available decisive provenance
  and the exact reason without fabricating claim, metric, or gate truth.
- A later authority change, revocation, supersession, stack change, or approved
  escalation MUST NOT rewrite a closed result or explanation. That result
  remains immutable historical truth for its exact closure. Every present
  Operational Reliance Basis MUST resolve the exact Evaluation Authorization
  Binding and Engine-confirmed historical validity. Its admitted Operational
  Reliance Policy MUST additionally select exactly one continuing-status mode:
  `historical_validity_only` or `continuing_current`. Only the latter requires
  fresh positive Engine current verification at the reliance semantic time and,
  when that policy makes the changed proposition decisive, a fresh evaluation.
  Fresh action-specific authorization and target verification remain separate
  and universal before any later Action Attempt; neither reliance mode weakens
  the strict collection-time or final-evaluation-closure checks above.
- Producer capability, a Dock or adapter operating ceiling, equality among a
  request, operating ceiling, and actual observation, per-claim evidence
  adequacy, viewer disclosure authority, posture strictness, and explanation
  density are independently owned facts. None establishes or replaces Engine
  evaluation authorization.
- The four semantic gate outcomes have fixed meanings: `pass` means the policy
  applies and establishes its bounded proposition; `fail` means it applies and
  establishes its negation or a policy-declared fail-closed violation;
  `indeterminate` means it applies but cannot establish pass or fail under its
  declared treatment; and `not_applicable` means positive semantic proof
  establishes that the policy's governed population or question does not
  apply. Omission, missing input, and inability to compute never produce
  `not_applicable`.
- Complete claim accounting is mandatory. Every expected claim appears exactly
  once with one applicability result. Every `applicable` claim then appears
  exactly once with either one established finding or one indeterminate
  determination. An applicability result of `undetermined` establishes no
  finding. Omission is invalid.
- **Evidence degradation** means a change that, while subject, applicability,
  policy, and all other eligible information remain fixed, removes or weakens
  eligible information supporting the previous decision or adds unresolved
  adverse, conflicting, stale, or uncertain eligible information. Such
  degradation cannot change `fail` or `indeterminate` to `pass`. When it
  destroys the sole valid basis for a prior `pass`, the re-evaluated decision
  cannot remain `pass`; the changed accounting must be visible. Any ordering of
  downstream dispositions belongs to the downstream consumer's policy.
- The full canonical explanation is Contracts-owned canonical decision data,
  not reconstructed prose or a density-specific reevaluation. It preserves
  expected claim inventory; applicability basis; every included, excluded,
  disqualified, stale, conflicting, and duplicate evidence binding; finding and
  assessment dimensions; each metric definition, population, value/state,
  conversion, and calculation; every gate condition and precedence step; and
  the decisive reason/accounting chain. The default human view is concise, and
  a requested deeper drill-down reaches the highest source detail both
  available and independently authorized for that viewer, with omissions made
  explicit. Disclosure authority and source availability are independent from
  truth. No density view may change inputs, rerun evaluation, or become another
  gate. The number, names, transport shapes, and interaction design of those
  projections remain bounded later decisions; this record selects no fixed
  level count, UI layout, or API.

## Discriminating scenario corpus

Each scenario assumes exact identity/provenance and the stated immutable policy
unless the row identifies their failure.

One scenario uses the hypothetical metric definition
`scenario.decimal-coefficient-seam@1` and algorithm
`scenario.exact-relative-weighted-binary-ratio@1`. Its configured population is
exactly three named claims after exclusion of any claim proved
`not_applicable`; compatible inputs must be binary findings from the same
closed evaluation context. Its output is an exact dimensionless rational in
`0..=1`, higher is better; the algorithm divides the sum of coefficients for
satisfied eligible inputs by the sum for all eligible inputs. Its configured
coefficients are exact, dimensionless, strictly positive, relative finite
decimals `0.25`, `1.50`, and `2.25`. For this definition, multiplication by a
common positive factor is equivalent, primitive-positive-integer normalization
is permitted, and the configured normalized components and every intermediate
sum are bounded by `16`. Any eligible missing, indeterminate, stale,
conflicting, uncertain, or incompatible input makes the result
`not_computable`; an empty eligible population is `not_applicable`. Its trace
accounts for every input, exclusion, coefficient, normalized value, sum, and
exact division. This scenario-local definition tests seam semantics; it does
not authorize a product metric catalog entry.

| Scenario and discriminating inputs | Responsible layers | Expected semantic outcome | Invariant proved |
| --- | --- | --- | --- |
| **Hard failure with high aggregate.** One required security claim is established unsatisfied; a named, versioned, configured aggregate metric reports `99/100`. The gate requires that claim satisfied and separately requires that metric at least `95/100`; direct failure is decisive. | Claim evaluator; metric; gate policy. | Metric remains `99/100`; the direct predicate fails; gate decision is `fail`. | A compensatory aggregate cannot override a direct predicate. |
| **Missing versus not applicable.** Claim A is applicable but has no admitted record; Claim B has a proved false applicability rule. The gate requires each applicable required claim determinate and treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate` when no fail is established. | Applicability and claim evaluators; metric; gate. | A has insufficient coverage and no finding; its determination is indeterminate. B is `not_applicable`. The dependent metric is `not_computable`; gate is `indeterminate`, not pass. | Absence, omission, and N/A are distinct. |
| **Stale evidence.** The only otherwise satisfying record exceeds its freshness policy at the explicit evaluation time. The gate requires the claim satisfied and treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate`. | Claim evaluator; metric; gate. | Currentness is stale; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Historical satisfaction is not current satisfaction. |
| **Conflicting evidence.** Two eligible authorized records support opposite findings, no resolver is named, and the gate treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate`. | Claim evaluator; metric; gate. | Consistency is conflicting; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Neither adapter nor metric may cherry-pick. |
| **Duplicate evidence.** The same candidate identity and bytes are submitted twice and the same closed evaluation context is re-evaluated. | Admission; claim evaluator; metric; gate. | Admission is idempotent to one canonical evidence identity; the finding, metric result, semantic gate decision, canonical identities, and accounting are unchanged. A same-identity/different-bytes submission refuses. | Duplication cannot increase weight or confidence. |
| **Flaky observation.** Ten distinct eligible observations contain nine satisfied and one unsatisfied result; the claim declares unanimity as its stability resolver and declares no sampling model. | Claim evaluator. | Consistency is conflicting; no finding is established; claim determination is indeterminate; no generic “90% satisfied” value is produced. | Repetition needs a named sampling model before scalar aggregation. |
| **Quantitative unit normalization.** Authorized observations report `1 s` and `1000 ms` for the same latency quantity, and admission permits one named exact conversion. | Admission; claim evaluator; metric. | Both normalize to the same exact canonical quantity and produce the same finding and metric value with source units retained in provenance. | Units are semantic and exact, not labels. |
| **Graded external scale.** An imported grade is `1` on a versioned ordinal scale where lower is better; a named claim evaluator maps it to a declared grade. | Adapter; admission; claim evaluator; compatible metric. | Direction is honored from the exact scale definition; a binary metric rejects the grade; only a metric naming that scale/mapping may consume it. | External numbers do not imply direction, binary truth, or weights. |
| **Decimal relative coefficients.** The scenario-local `scenario.decimal-coefficient-seam@1` definition receives findings satisfied, unsatisfied, satisfied. A single-condition gate requires that exact metric result to be at least `5/8`, with equality included. | Metric-definition authority; metric computation; gate policy. | The permitted normalization produces `1, 6, 9`; the calculation trace records numerator `10`, denominator `16`, and exact result `10/16 = 5/8`; the gate passes on equality. | Coefficient meaning, conditional normalization, computation, bounds, and threshold remain exact and separately owned without selecting a catalog algorithm. |
| **All not applicable.** Every claim in the gate's governed population is proved `not_applicable`; the named metric's definition declares an empty eligible population `not_applicable`; the gate policy declares that positive proof for the complete population makes its question not applicable; and an admitted action-specific Operational Reliance Policy maps that historical result into one atomic determination. | Applicability evaluator; metric; gate; Operational Reliance owner. | Metric and gate are both `not_applicable`, never `pass`; the Operational Reliance Determination has `not_applicable / no_effect` facets that share one identity and accounting closure. | Empty populations do not pass vacuously, and eligibility/disposition cannot diverge. |
| **Degradation monotonicity.** A prior `pass` depends on one current satisfying record. At a later evaluation time the same subject snapshot, applicability, policy, and other eligible information are unchanged, but that record is now stale; the gate treats the resulting indeterminate direct predicate as `indeterminate`. | Claim evaluator; metric; gate. | No finding is established; the dependent metric is `not_computable`; the re-evaluated gate is `indeterminate`, not `pass`, and changed accounting identifies the stale record. | Loss of the sole eligible basis for pass cannot preserve or improve the gate decision. |
| **Mixed population closure.** Candidate claim evaluations are presented as one metric population but differ in subject snapshot, contract revision, evaluator version, or evaluation time. | Evaluation-context admission; metric; gate. | The mixed closure refuses before metric or semantic-gate truth. It does not produce a `not_computable` metric or `indeterminate` gate. | Context substitution or construction failure is not a semantic result. |
| **Multiple named metrics.** A gate requires both a named functional-metric threshold and a named security-metric threshold. The first predicate passes and the second fails; failure is decisive. | Two metrics; gate policy. | Results remain separate; the security metric predicate is decisive and the gate is `fail`. No hidden cross-metric average exists. | One metric cannot silently compensate for another. |
| **Uncertain boundary.** A latency interval straddles the claim budget, no alternate evaluator is named, and the gate requires that claim satisfied while treating an indeterminate direct predicate as `indeterminate`. | Claim evaluator; dependent metric; gate. | Uncertainty is preserved; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Midpoints or favorable endpoints cannot be selected silently. |
| **Unauthorized source.** An otherwise well-formed candidate binds a source actor that lacks current authority for the stated subject and observation class. | Contracts evidence admission. | Admission refuses before claim evaluation and produces no canonical evidence record, finding, metric result, or gate decision. | Structural validity and adapter output cannot substitute for source authority. |
| **Coordinates without authorization.** A prospective run supplies a valid exact stack and six requested coordinate values but no Evaluation Authorization Binding. Comparison cases reuse a real binding for another subject, scope, objective, use, parent or constraint closure, or pair identical coordinate labels with a changed stack fingerprint. | Engine authorization; prospective evaluation formation. | Prospective Evaluation Basis formation refuses, so no producer is launched and no observation is collected. The comparison cases require a fresh, congruent binding; matching coordinate labels do not establish compatibility or transferability. | Requested coordinates and structural equality are not Engine authorization provenance. |
| **Dock equality without Engine authority.** A proposed Dock request, admitted operating ceiling, and expected actual Resolution are exactly equal, but there is no verifiable Evaluation Authorization Binding. | Dock; Engine authorization; prospective evaluation formation. | Dock equality cannot close a Prospective Evaluation Basis, so the producer MUST NOT be launched under that proposal and no candidate or semantic truth is produced. | A producer operating contract neither grants nor proves Engine authority. |
| **Authorization changes across stages.** In one case Engine reports the binding stale or superseded before launch. In a second, authorization changes after canonical evidence exists but before final evaluation-context closure. In a third, a `pass` was already closed before authorization later became non-current; one Operational Reliance Policy declares `historical_validity_only` and another declares `continuing_current`. | Engine authorization; Evaluation Coordination; Contracts evaluation-context closure; Operational Reliance and action owners. | The first prevents collection. The second preserves historical evidence but refuses new claim, metric, or gate truth and requires a fresh authorized basis/context. The third preserves the immutable historical `pass` and canonical explanation. Both reliance paths resolve the exact Binding and historical validity; only `continuing_current` requires fresh positive Engine current verification and any policy-required fresh evaluation. Any effect under either path still requires a distinct fresh Action Authorization Binding, expected-target verification, exact attempt, Terminal, and state-owner receipt. | Collection authorization, final-truth authorization, historical validity, policy-selected continuing-current reliance, and fresh action authorization are distinct propositions. |
| **Approved Context Resolution escalation.** Engine approves a changed envelope after an earlier Prospective Evaluation Basis or final evaluation context was formed. | Engine authorization; separately owned readiness; prospective evaluation formation; Contracts evaluation-context closure. | The approved envelope requires a fresh Evaluation Authorization Binding, readiness determination, Prospective Evaluation Basis, and final evaluation context. Prior evidence and results remain bound to their earlier bases and contexts and are not upgraded. | Escalation is not in-place expansion or retroactive authorization. |
| **Exact resolved-requirements realization.** One Resolved Assurance Requirements identity for scope `S` selects contract-rooted expected-claim inventory `C`, exact Evidence Evaluation Policy bindings `E`, configured Metric Definitions `M`, and Gate Policy `G`. The valid evaluation uses a subject snapshot inside `S`, one exact policy in each supported required role, and exactly `C/E/M/G`. Comparison cases use a subject outside `S`; substitute an incompatible policy; bind an exact policy to an unsupported role; supply conflicting bindings for one role; change a metric configuration; omit one selected claim; add an unselected claim; or let `G` depend on an unselected claim or metric. | Assurance selection; evaluation-context admission; claim evaluation; metrics; gate. | The exact valid closure may evaluate reproducibly. Every comparison refuses before claim, metric, or semantic-gate truth; none falls back or becomes ordinary `not_applicable`, `not_computable`, or `indeterminate`. | The evaluation context must exactly and completely realize one selected requirements identity. |

## Settled versus deferred

This record settles the vocabulary and relationships, independent dimensions,
exact measurement rules and bounds, weight ownership, Evidence Evaluation
Policy roles and compatibility, the one-contract-rooted
requirements-to-evaluation conformance rule, responsibility map, gate purity
and precedence, exact Engine-owned evaluation-authorization provenance and
context-refusal boundary, the pre-observation Prospective Evaluation Basis and
later evidence-bound evaluation-context closure, universal historical Binding
resolution/validity plus the explicit `historical_validity_only` /
`continuing_current` present-reliance rule, separate fresh action authorization,
Policy Admission Closure requirements, canonical explanation truth and density-
projection principles, complete explanation/accounting requirement, and every
scenario outcome above. Later work may not reopen those decisions merely by
choosing a representation.

Deliberately deferred to human-approved task-level planning are all concrete
metric algorithms and catalog entries; schema, type, API, and operation shapes;
expression language; persistence layout; registry and version-resolution
mechanics; any cutover or temporary internal bridge; implementation packets;
acceptance planning; and rollout. Also out of scope are external adopter
selection, concrete local-closeout or parent-promotion policy, work hierarchy
and state model, posture algorithms, UI, a metric marketplace, and a universal
confidence score.

The proposed
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md)
owns evaluation-purpose formation, authorization issuance, Evaluation
Coordination, Prospective Evaluation Basis lifecycle, currentness checkpoints,
readiness, capability/ceiling separation, diagnostics, and retry/cache/
supersession/escalation semantics. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
owns the common present-use rule, atomic eligibility/disposition determination,
separate action chain, and Work Progression semantic responsibility for local-
closeout and parent eligibility. Concrete physical placement, work hierarchy and
state model, admitted policies, schemas, storage, transport, host/Substrate
mechanism, mutations, implementation, and delivery slices remain deferred. The
proposal does not silently approve any of them or require a general identity,
credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
or ABAC platform.

Human approval of this proposed record would make it semantic input to later
planning only. It would not by itself authorize implementation, change a phase
or slice status, or establish any delivery outcome.
~~~~~~~~~~~~

## operational-reliance-and-work-progression-semantic-decision.md

SHA-256: `e1ad9f340c562a1ec42cf79ad8e2852053fec291823e8bb4aa0d1db411da60ff`

~~~~~~~~~~~~markdown
# HCM operational-reliance and work-progression semantic decision

**Status: PROPOSED**

This record fixes the greenfield semantic seam from immutable evaluation and
gate truth to present operational use, local closeout, parent eligibility, and
any later action that changes work state. It separates a result that was true
for one closed historical context from a present decision to rely on that
result and from authority to perform an action.

It is a narrow companion to the proposed
[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
[posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
and [metric, evaluation, and gate decision](metric-gate-semantic-decision.md).
Those records retain ownership of authored artifacts, posture, assurance,
evaluation authorization, evidence closure, claim and metric truth, and the
semantic gate. This record supplies the downstream rule those records defer.

This record is semantic authority for human review only. It is not
implementation authority, a slice or packet specification, a schema or API
design, or rollout planning. Current HCM-5.2 local-closeout source is evidence
for the purity and owner-separation problem, not a compatibility obligation.
The product is greenfield; no V1 local-closeout shape, gate coupling, parent-
promotion field, migration, alias, bridge, or dual behavior is selected.

## Purpose and scope

The canonical downstream chain is:

```text
immutable historical evaluation / claim / metric / semantic-gate truth
  -> exact action question: kind + target + effect + scope + eligibility-relevant parameters
  -> admitted Operational Reliance Policy
  -> action-specific Operational Reliance Basis at a present semantic time
  -> one Operational Reliance Determination
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> exact Action Intent, only after `allow`
  -> Action Authorization Binding or Action Authorization Refusal
  -> exact Action Attempt Identity and attempt-bound checks, only after a Binding
  -> separately applied effect or mutation
  -> Action Terminal and, only after committed state change, Mutation Receipt
```

For work progression, two distinct reliance chains may exist:

```text
local work result(s)
  -> Local Closeout Operational Reliance Determination
       - Local Closeout Eligibility facet
       - fixed Operational Disposition facet

exact parent + complete child/dependency closure
  + exact child Local Closeout Eligibility results and other declared inputs
  -> parent-action Operational Reliance Determination
       - Parent Eligibility Result facet
       - fixed Operational Disposition facet
```

Neither chain performs a closeout or parent-state mutation. Parent eligibility
is not promotion. An actual local closeout, parent advancement, parent
promotion, release, deployment, posture transition, or other effect follows
its own separately authorized action path.

This record settles the common present-use vocabulary and the exact Work
Progression ownership needed for local closeout and parent eligibility. It does
not select any concrete closeout, parent, release, deployment, or posture
policy; thresholds; work hierarchy; state machine; or mutation.

## Canonical vocabulary

Terms in separate rows are not synonyms.

| Term | Fixed meaning |
| --- | --- |
| **Historical Semantic Result** | An immutable owner-produced evaluation, applicability, claim, metric, semantic-gate, explanation, or other semantic result for one exact historical input closure and semantic time. It remains exact history after its dependencies become non-current. It is not present permission, eligibility, disposition, or action authority. |
| **Operational Reliance** | A present-tense use of one or more exact Historical Semantic Results to decide whether a named action for an exact target may be considered now. Historical display, deterministic explanation, and exact replay that produce no present operational effect are not Operational Reliance. |
| **Action Kind** | An immutable, admitted, named, and versioned semantic definition owned by one downstream action domain. It defines one bounded effect family and the compatibility rules for its exact action question, such as local closeout, parent advancement, release, deployment, posture-observation admission, or posture transition. Equal verbs, labels, or UI actions do not make two Action Kinds interchangeable. It grants no authority. |
| **Action Target Snapshot** | The immutable owner-produced identity of the exact target state, scope, and applicable structural relations against which present reliance and any later action are decided. For parent eligibility it includes the exact parent subject and its complete child/dependency closure. It is neither a Work Specification revision nor an evaluation subject merely because either refers to the same work. |
| **Action Question** | The immutable bounded tuple closed before reliance from one exact Action Kind, one exact Action Target Snapshot and expected state, the exact bounded effect and effect scope, and every action parameter whose value can change eligibility under the Operational Reliance Policy. It is part of the Basis and Determination identities, grants no authority, and is not an Action Intent because it contains no actor or execution request. |
| **Operational Reliance Policy** | An immutable, admitted, named, and versioned action-domain rule that declares one Action Kind; accepted source-result kinds and use intents; exact target and subject-congruence requirements; complete present-use dependency and currentness modes; result-composition and indeterminacy treatment; and the rules that produce one Operational Reliance Determination with fixed Current-Use Eligibility and Operational Disposition facets. It grants no action authority. |
| **Operational Reliance Occurrence Identity** | The action-domain owner-issued identity of one deliberately intended present-use determination. It distinguishes repeated reliance at different times or for different intended actions even when the referenced results and policy are equal. It is not an evaluation occurrence, action authorization, transport correlation, or content fingerprint. |
| **Operational Reliance Basis** | The immutable, action-specific input closure for one Operational Reliance Occurrence Identity. It binds the exact action question—Action Kind, Action Target Snapshot, bounded effect, effect scope, and every eligibility-relevant action parameter—plus the Operational Reliance Policy, finite exact Historical Semantic Result population, accepted evaluation use intent, present reliance semantic time, every required owner resolution/currentness observation, complete included/excluded/missing/conflicting accounting, and exact policy-selection provenance. It contains no Action Authorization Binding and performs no mutation. |
| **Operational Reliance Determination** | The one immutable action-domain result produced from one exact Operational Reliance Basis under one occurrence, policy, action question, target, semantic time, identity, and decisive accounting closure. It contains Current-Use Eligibility and Operational Disposition as two fixed facets, grants no action authority, and performs no mutation. |
| **Current-Use Eligibility** | The semantic facet of one Operational Reliance Determination: `eligible`, `ineligible`, `indeterminate`, or `not_applicable`. It answers only whether the bound historical truth may support consideration of the exact action question for the exact target now. It is not independently formed, current, superseded, or replayed apart from its Determination. |
| **Reliance Refusal** | The owner-produced refusal to form a valid Operational Reliance Basis or Determination because a required identity, policy, target relation, owner resolution, currentness observation, use-intent rule, cardinality, or accounting boundary is missing, malformed, ambiguous, stale where currentness is required, substituted, conflicting, or unverifiable. It is not `ineligible`, `indeterminate`, or `not_applicable`. |
| **Operational Disposition** | The mechanically fixed control-facing facet of one Operational Reliance Determination: `allow`, `block`, `hold`, or `no_effect`. It is not a second policy decision or independently current result. `allow` permits only an Action Intent and request for separate action authorization; it neither performs nor authorizes the action. |
| **Action Intent** | The immutable, exactly identified requester-owned request to perform the action question already decided by one exact `eligible / allow` Operational Reliance Determination. It MUST be exactly congruent with that Action Kind, Action Target Snapshot, expected state, effect, effect scope, and every eligibility-relevant parameter. It may add only the actor or principal, claimed delegation or authority provenance, and execution-local details that the Operational Reliance Policy declares irrelevant to eligibility. Those claims remain claims until the action-authorization owner verifies or explicitly admits them under an advisory-authority rule. The Intent is untrusted until admitted and grants nothing by itself. |
| **Action Authorization Binding** | The immutable authorization-owner semantic result and provenance closure that binds the exact authorization-owner identity; owner-recognized authorization rule; declared authority mode and basis, including whether principal or delegation facts are claimed, owner-verified, or host-enforced; issuance semantic time and owner-verifiable lineage; one exact Action Intent; actor or principal; exact action question; exact `eligible / allow` Operational Reliance Determination; bounded conditions; and the rule governing whether one or more Action Attempts are permitted. It is not itself a credential, capability token, authenticated identity, delegation instrument, or proof of host enforcement. It cannot improve eligibility, broaden the Intent, or substitute for another authorization owner. Evaluation authorization cannot substitute for it. |
| **Action Authorization Refusal** | The immutable authorization-owner result that one exact Action Intent cannot be authorized, with the decisive reason, declared authority mode and basis, semantic time, and owner provenance. It creates no Action Authorization Binding, Action Attempt Identity, Action Terminal, or Mutation Receipt. |
| **Action Attempt Identity** | The exact action-executor-owned identity of one deliberately intended execution attempt under one exact Action Intent and Action Authorization Binding. It distinguishes exact delivery replay from a new attempt. It grants no authority and is not the Intent, Binding, Terminal, or Mutation Receipt. |
| **Action Terminal** | The action-executor-owned immutable durable provenance for one exact Action Attempt Identity under one exact Action Authorization Binding. It records the executor's exact attempt outcome, including a refusal after the attempt boundary was entered, completion without target mutation, execution failure, or effect/commit uncertainty. It may report a committed target-state mutation only by binding the exact authoritative state-owner Mutation Receipt. A pre-attempt Action Authorization Refusal is not an Action Terminal. |
| **Mutation Receipt** | The authoritative state owner's immutable proof that one exact Action Attempt committed the exact authorized mutation against the exact expected predecessor state and produced the exact successor state at the owner's commit semantic time. No terminal, acknowledgement, transport success, or executor assertion substitutes for it. |
| **Local Closeout Eligibility** | The Current-Use Eligibility facet of one atomic Work Progression-owned Operational Reliance Determination for the `local_closeout` Action Kind and one exact local work target. It is produced from its own Local Closeout Policy and Basis. It is not an independent result, the local closeout mutation, or a parent proposition. |
| **Parent Eligibility Result** | The Current-Use Eligibility facet of one atomic Work Progression-owned Operational Reliance Determination for one exact parent-target Action Kind, parent snapshot, and complete declared child/dependency closure. It has its own Parent Eligibility Policy and Basis. It is not an independent result, Local Closeout Eligibility, or parent promotion. |
| **Work Progression** | The downstream semantic responsibility that owns work-target and parent/dependency closure for reliance, Local Closeout and Parent Eligibility policies, Operational Reliance Bases, and atomic Determinations with fixed Current-Use Eligibility and Operational Disposition facets for those action kinds. It does not own Contracts truth, authored Work Specification content, or state mutation by implication. |

`Current` is not a free-standing status. Every currentness observation names its
semantic owner, exact object and proposition, use boundary, semantic time, and
verification rule. A current Historical Semantic Result is still not an action
authorization.

## Ontology placement

The objects introduced by this record retain the six ontology categories fixed
by the canonical-object ontology decision:

- No object introduced by this record is an authored Artifact Instance. A Work
  Specification remains an existing authored artifact when it references a
  result from this seam.
- Action Kind definitions; Operational Reliance Policies; Local Closeout
  Policies; Parent Eligibility Policies; and separately named Action
  Authorization policies are canonical semantic definitions or policies.
- Action Target Snapshots; Operational Reliance Bases and Determinations;
  Reliance Refusals; Action Authorization Bindings and Refusals; Local Closeout
  Eligibility and Parent Eligibility Results as facets of their Determinations;
  and Mutation Receipts are owner-produced canonical records or results.
- A closed Action Intent, Action Attempt Identity and its durable attempt
  occurrence, and Action Terminal are durable request or attempt provenance.
- An unclosed action request, in-progress attempt, or transport-local action
  failure before an owner occurrence closes is transient operational state.
- Renderings, summaries, compact explanations, SDK DTOs, CLI/Tauri payloads,
  and other views are deterministic projections or representations as
  applicable.

None of these objects is an Artifact Instance. Persistence, project-local
storage, human readability, content addressing, or physical colocation MUST
NOT change its category or semantic owner.

Action Question is a bounded value inside its Basis, Determination, Intent, and
authorization closures. It normally participates in those identities rather
than acquiring an independent repository or lifecycle.

## Historical truth and present use

A Historical Semantic Result MUST remain immutable for its exact evaluation
occurrence, evidence closure, definitions, policies, semantic times, and owner
verification facts. Later changes may alter whether an action domain can rely
on it now, but MUST NOT change its decision, explanation, identity, or recorded
historical validity.

Operational Reliance MUST begin from exact owner-recognized result identities.
A summary, projection, rendered document, Work Specification status, SDK DTO,
CLI output, copied gate label, current-head lookup, nearest policy version, or
ambient repository query cannot substitute for the source result. A projection
may help a person understand a decision, but its source result must be resolved
before operational use.

Historical display and exact replay require no Operational Reliance Basis when
they cause no present effect. The moment a result is used to allow, block, hold,
prioritize, close, advance, promote, release, deploy, admit a posture
observation, recommend a transition, or otherwise affect current work, the
responsible action domain MUST form an action-specific basis and result.

Equal Historical Semantic Results may legitimately produce different
Current-Use Eligibility for different Action Kinds, targets, policies, or
reliance times. The difference belongs to the downstream policy and MUST NOT be
described as a changed or reinterpreted gate.

An Operational Reliance Determination MAY be the terminal result of a pure,
non-mutating advisory use. When no effectful Action Intent is formed, no Action
Authorization Binding, Action Attempt, Action Terminal, or Mutation Receipt
exists. `block` does not mutate a work item into a blocked state; `hold` does
not create a queue or waiting state; `no_effect` does not mark an action
successful; and `allow` does not admit an observation, create a priority, or
produce a recommendation.

A separately owned admission, prioritization, or recommendation MAY consume an
exact Operational Reliance Determination only under that later owner's own
exact policy, admission, currentness, and accounting rules. `eligible / allow`
is not the later admission, priority, recommendation, or action authority.

Formation or durable retention of the semantic records defined by this chain
is not recursively another effectful action merely because a repository
changes. The action chain governs the bounded operational effect named by an
exact Action Intent unless another independently admitted Action Kind expressly
targets that record lifecycle.

## Operational Reliance Policy and basis formation

Every Action Kind, Operational Reliance Policy, Action Authorization policy, and
recovery policy consumed by this chain MUST be an exact category-2 definition
admitted by its responsible semantic owner. Candidate content MAY be authored by
a user, an agent, an authored project artifact, or another tool. Authorship
supplies proposed content, provenance, and organizational judgment; it does not
perform executable admission.

The definition owner MUST deterministically produce the exact owner-side
**Policy Admission Closure** defined by the companion
[canonical-object ontology decision](canonical-object-ontology-and-ownership-semantic-decision.md).
That closure binds the candidate's exact content; typed vocabulary and
conformance; exact candidate boundary, selection, and accounting; every relevant
exact Charter, Project Context, Decision Record, or Risk Record revision with its
proposition role, scope, and historical, formation-only, or continuing-current
dependency mode; and, when the candidate's admission rule requires approval or
exception authority, the exact separately typed and authorized source, its
authority, scope, identity/version or revision as applicable, and currentness. It
also establishes compliance with the non-relaxable invariants of this decision.

Admission MUST NOT require every artifact family for every policy, treat a bare
Markdown link as proof, turn authored prose into runtime evidence or a waiver,
or ask an AI to make an open-ended organizational judgment about whether a
candidate is “safe.” Organizational judgment enters through the exact governance
sources declared by the admission rule. A candidate is mechanically prohibited
as unsafe only when an admitted constraint or non-relaxable invariant says so,
and a refusal MUST identify that violated rule. The admission rule is itself an
exact owner-governed version outside the candidate population being decided;
changing it creates a successor rule rather than an unbounded requirement that
each admission recursively admit its own admission process.

An Operational Reliance Policy MUST declare before result direction is used:

- exactly one Action Kind and policy owner;
- supported Action Target Snapshot kind, scope, and congruence relation;
- every accepted Historical Semantic Result kind, semantic owner, cardinality,
  composition relation, and Evaluation Use Intent;
- whether each dependency is historical-identity-only or requires a fresh
  positive currentness verification at reliance time;
- every additional current fact, definition, policy, artifact revision,
  posture snapshot, work-structure relation, or authority-independent
  prerequisite it consumes;
- exact handling of `pass`, `fail`, `indeterminate`, `not_applicable`, result
  direction, accepted exact upstream refusal-record kinds, and mixed semantic
  inputs, while failed historical resolution remains Reliance Refusal;
- the exact predicates that establish `eligible`, `ineligible`,
  `indeterminate`, or `not_applicable` and the fixed disposition mapping; and
- complete accounting, semantic-time, invalidation, and replay rules.

Before forming an Operational Reliance Basis, the responsible downstream
domain MUST close the exact action question whose present use is being decided:
one exact Action Kind, one exact Action Target Snapshot, the exact bounded
effect, effect scope, and every action parameter whose value can change
eligibility under the admitted Operational Reliance Policy. The Basis and its
Operational Reliance Determination MUST bind that exact action question.

A policy MUST NOT be selected, replaced, or configured after inspecting a
favorable result direction unless an already admitted higher-order selection
rule predeclares that exact choice and completely accounts for every candidate.
No Work Specification prose, transport branch, actor preference, or action
request may create a policy or select a convenient one ambiently.

For every consumed Historical Semantic Result role, an Operational Reliance
Policy MUST declare one exact finite eligible source-result boundary. The
boundary MUST bind:

- permitted semantic owners and exact repositories, resolvers, or exact
  nominated references;
- accepted result kinds, Evaluation Use Intents, and result roles;
- required subject, target, scope, evaluation-occurrence, context, and
  semantic-time relations;
- historical-only or present-currentness modes;
- cardinality, multiplicity, ordering, and composition semantics;
- deterministic inclusion and exclusion rules; and
- complete candidate, selected, excluded, missing, duplicate, conflicting,
  unavailable, and refused accounting.

Basis formation MUST account exactly once for every owner-recognized result
matching that boundary and every required result role. Caller-selected subsets,
favorable-result selection, current-head or nearest-version lookup, repository
order, open-ended “all current results” queries, or unrecorded omission MUST
refuse.

An exact upstream refusal record MAY be consumed only when the policy declares
that exact refusal kind as an accepted Historical Semantic Result role and the
refusal resolves successfully from its owner. Failure to resolve a required
identity, semantic content, owner provenance, or historical validity is always
Reliance Refusal. It cannot be mapped by policy to `ineligible`,
`indeterminate`, or `not_applicable`.

Acceptance of diagnostic Evaluation Use Intent MUST be declared per consumed
result role, result direction, and exact Operational Reliance action question.
Permission to use a diagnostic result to block, hold, trigger remediation,
admit a posture observation, or support another negatively directed use does
not permit that result to establish positive `eligible / allow` for a different
Action Kind, target, effect, or scope. No owner may erase, relabel, or infer a
different Evaluation Use Intent.

For one exact Operational Reliance Occurrence Identity:

- exact same basis-formation content is replay and returns the same Basis,
  Operational Reliance Determination identity and both facets;
- exact same refused formation content returns the same Reliance Refusal
  identity and decisive owner provenance;
- changed content under that identity refuses;
- a deliberately new present-use determination receives a new occurrence
  identity even when every semantic input is otherwise equal; and
- transport retry or correlation cannot create, merge, or distinguish
  reliance occurrences.

Exact replay of one Action Authorization request or Action Attempt likewise
returns the same owner outcome for the same exact occurrence and content;
changed content under the same semantic identity refuses.

Basis formation MUST resolve and completely account for every policy-declared
input. It refuses before a Determination when any required owner, identity,
subject/target relation, cardinality, semantic time, source-result integrity,
currentness observation, or policy-selection fact is absent, ambiguous,
substituted, conflicting, or unverifiable. A valid but expressly uncertain
policy input may produce `indeterminate` only when the admitted policy declares
that uncertainty as a semantic input; missing formation truth is never
`indeterminate`.

## Exact current-use and Engine-verification rule

The common present-use floor is mandatory for every action domain:

1. close the exact action question and its eligibility-relevant parameters;
2. resolve the exact Historical Semantic Result and verify its immutable owner
   identity, content, input closure, and recorded historical validity;
3. resolve the exact current Operational Reliance Policy and its complete
   selection basis;
4. resolve an exact Action Target Snapshot and obtain a fresh positive owner
   verification that it is the current expected target for this reliance
   occurrence;
5. establish exact result-to-target subject and scope congruence;
6. obtain every fresh owner verification the policy declares for present use,
   without reconstructing one owner's proposition in another domain; and
7. complete all inclusion, exclusion, missing, conflict, use-intent, and
   currentness accounting before producing eligibility or disposition.

Fresh current verification of the historical evaluation's Engine-issued
**Evaluation Authorization Binding is policy-conditional, not universal**.
Every admitted Operational Reliance Policy that accepts a Historical Semantic
Result whose exact closure binds such a Binding MUST explicitly declare exactly
one reliance-time dependency mode for it:

- **`historical_validity_only`**; or
- **`continuing_current`**.

An omitted, inherited, caller-selected, ambient, or defaulted mode makes the
policy invalid and refuses policy admission or resolution.

Under both modes, the common exact-result floor requires owner-recognized
historical resolution of the exact Evaluation Authorization Binding, its
issuance lineage, and the historical-verification outcome required to establish
that the Historical Semantic Result was validly closed. Any Engine-declared
historical-resolution mechanism may satisfy this requirement; it is not a
current-authorization query. An outcome of `historically_invalid`,
`unverifiable`, unavailable, ambiguous, or substituted causes Reliance Refusal.
It does not rewrite the Historical Semantic Result and MUST NOT become
`ineligible`, `indeterminate`, or `not_applicable`.

Under `continuing_current`, a fresh Engine-produced positive current
verification at the Operational Reliance semantic time is additionally
mandatory. `superseded`, `not_current`, `unverifiable`, unavailable, ambiguous,
or substituted current verification causes Reliance Refusal.

Under `historical_validity_only`, current Evaluation Authorization status is
not an Operational Reliance input. The action domain MUST NOT infer a current
Engine conclusion from the historical Binding or allow an independently
obtained current observation to alter the Operational Reliance Determination.

A later Action Authorization policy MAY independently declare the same
Evaluation Authorization Binding or another Engine-owned proposition as an
authorization-time or commit-time condition. When it does, only Engine's exact
fresh verification satisfies that condition. The Operational Reliance mode
neither supplies nor forbids that distinct authorization-stage dependency.
Any other Engine-owned target, artifact, posture, authority, or Context
Resolution fact declared by either policy likewise requires Engine's exact
verification. Policy-conditional means policy-mandatory when declared, never
caller-optional.

This is the focused downstream rule to which the companion decisions defer. It
refines broader language suggesting fresh evaluation-authorization verification
for every operational reliance. Evaluation authorization is permission to
perform one historical evaluation under one purpose and reach. It is neither
permission to act now nor a universal statement that every possible action
depends on continuing evaluation reach. Universal continuing-current status
would conflate evaluation and action authority; allowing an omitted mode would
make accidental policy silence authoritative. The explicit two-mode rule avoids
both.

Fresh **Action Authorization Binding** and expected-target verification are
universal before every Action Attempt and cannot be policy-disabled. A state-
changing action MUST additionally enforce the same Binding, target, exact
authorized effect, and every declared current-through-commit dependency at its
commit or linearization boundary. Missing, non-current, superseded, mismatched,
or unverifiable verification refuses the governed attempt or commit without
changing the already closed Operational Reliance Determination.

## Eligibility, disposition, and refusal vocabulary

One valid Operational Reliance Basis produces exactly one immutable Operational
Reliance Determination. That Determination contains one Current-Use Eligibility
facet and one mechanically paired Operational Disposition facet:

| Current-Use Eligibility | Fixed meaning | Operational Disposition |
| --- | --- | --- |
| `eligible` | The exact policy positively establishes that the bound Historical Semantic Results may support consideration of this exact Action Kind for this exact target now. | `allow` |
| `ineligible` | The exact policy positively establishes that the bound results may not support this exact action for this target now. | `block` |
| `indeterminate` | The basis is valid, but an expressly admitted uncertain or unresolved semantic input prevents the policy from establishing eligible or ineligible. | `hold` |
| `not_applicable` | Positive policy proof establishes that the action question does not apply to this exact target and scope. | `no_effect` |

The mapping is fixed so interfaces cannot turn `indeterminate` into `allow`,
turn `not_applicable` into success, or reinterpret a positive blocker as a
temporary hold. An action-specific policy controls the bounded question. Thus a
failed security gate may make local closeout `ineligible` while making an exact
remediation action `eligible`; the gate truth is unchanged in both cases.

The two facets share one Operational Reliance Occurrence Identity, policy,
action question, target, semantic time, determination identity, and decisive
accounting closure. Operational Disposition is not a second policy decision,
separately current object, independently supersedable record, or independently
replayable result. No implementation may issue, persist, supersede, reference,
or replay one facet as though the other could differ. A reference MAY identify
the shared Operational Reliance Determination and the exact facet displayed or
documented.

Reliance Refusal produces no Operational Reliance Determination and therefore
no Eligibility or Disposition facet. Adapters MAY render it operationally as
“no action permitted,” but MUST preserve the refusal kind and decisive owner
reason rather than fabricate `block` or `hold`. `allow` alone never permits an
executor or mutation owner to act without a current exact Action Authorization
Binding.

## Work Progression ownership

Work Progression is the exact downstream semantic owner for local closeout and
parent eligibility. It owns:

- exact work-target and parent/child/dependency relationship closure needed by
  those decisions;
- admission and resolution of Local Closeout Policies and Parent Eligibility
  Policies;
- action-specific policy selection and complete candidate accounting;
- Operational Reliance Occurrence Identity and Basis formation;
- Local Closeout and Parent Eligibility Operational Reliance Determinations,
  including their Current-Use Eligibility and Operational Disposition facets;
  and
- exact historical replay and currentness/invalidation observations for those
  work-progression results.

The authoritative work-structure and work-state owner supplies exact work
identities, current state snapshots, and parent/child/dependency relations.
Work Progression resolves and completely closes those owner facts into its
Action Target Snapshots; it MUST NOT invent or rewrite them. This target model
does not retain the current Contracts-owned `LocalCloseoutResultV1` as the
greenfield local-closeout authority. A future local result is the distinct
Work Progression-owned Local Closeout Eligibility defined here.

Work Progression MUST NOT own or infer Contracts evidence, claim, metric, or
gate truth; Engine evaluation authorization; artifact promotion; Action
Authorization Binding issuance merely because it produced `allow`; or an actual
work-state mutation. One future component may host Work Progression and an
action-authority or state-owner responsibility, but their policies, identities,
decisions, refusals, currentness, and receipts MUST remain separately
attributable.

Release, deployment, environment change, posture observation, posture
recommendation, posture transition, and other action kinds remain owned by
their separately named downstream domains. They MAY reuse the common reliance
vocabulary and floor fixed here but MUST form their own policies, bases,
atomic Determinations, authorizations, and effects. This record creates no
universal enforcement owner.

## Local closeout

One Local Closeout Operational Reliance Determination MUST be formed
independently from one exact Local Closeout Policy and one exact local-work
Operational Reliance Basis. Its Local Closeout Eligibility facet and the Basis
MUST bind:

- the exact local Action Target Snapshot and currentness verification;
- every exact evaluation, claim, metric, gate, or other owner result the policy
  declares, including Evaluation Use Intent;
- exact subject/scope congruence between those results and the local target;
- every policy-declared artifact, dependency, completion, or currentness fact;
- exact policy identity, selection provenance, and reliance semantic time; and
- complete included, excluded, missing, conflicting, stale, and indeterminate
  accounting.

A Local Closeout Policy MAY declare that one exact gate `pass` is sufficient
only when its complete predeclared input and currentness closure makes that
true. No universal `gate passed -> local closeout eligible` rule exists.
`fail`, `indeterminate`, or `not_applicable` retain their Contracts meanings;
the Local Closeout Policy maps them only for its bounded action question.

Local Closeout Eligibility is pure. It MUST NOT mutate a Work Specification,
work state, Pipeline state, parent, release, deployment, posture, or any other
object. A local closeout action may proceed only after `eligible / allow`, a
congruent Action Intent, a separate exact Action Authorization Binding, and an
exact Action Attempt. The authoritative work-state owner alone enforces the
target and every declared through-commit currentness condition, commits the
mutation, and issues its Mutation Receipt.

## Parent eligibility and parent action

Parent Eligibility is a separate Work Progression decision, not a field inside
Local Closeout Eligibility, a gate, or a score. Its exact Parent Eligibility
Policy and Operational Reliance Basis MUST bind:

- one exact parent Action Target Snapshot and currentness verification;
- the exact owner-established parent-to-child and dependency closure at the
  reliance semantic time;
- every child, dependency, role, cardinality, ordering or composition relation
  the policy declares;
- each exact Local Closeout Eligibility or other owner result consumed for each
  required relation, with its subject/scope congruence and current-use basis;
- every required parent-level gate, policy, artifact, authority-independent
  prerequisite, and present-use owner verification; and
- complete included, excluded, missing, duplicate, conflicting, stale,
  superseded, and unresolved accounting.

Every Parent Eligibility Policy MUST declare one exact finite authoritative
parent-relationship boundary. That boundary MUST bind:

- the authoritative work-structure owner and exact resolver or snapshot
  identity;
- the exact parent Action Target Snapshot;
- supported child, dependency, role, ordering, and composition relation kinds
  and directions;
- the exact direct, transitive, or otherwise declared relation reach;
- the causal or semantic-time cut and currentness rule;
- the policy's cycle treatment;
- deterministic relation inclusion and exclusion rules; and
- complete included, excluded, missing, duplicate, conflicting, cyclic,
  superseded, stale, and unresolved accounting.

Work Progression MUST account exactly once for every owner-recognized relation
matching that boundary and every policy-required result role. Open-ended
traversal, unsupported cycles, mixed relation cuts, ambiguous direction, or
inability to establish boundary completeness causes Reliance Refusal before
Parent Eligibility.

For every relation role, the Parent Eligibility Policy MUST declare the exact
proposition it consumes, including whether it requires a child Local Closeout
Eligibility, Action Terminal, Mutation Receipt, current child work-state
snapshot, or another exact owner result. Those propositions are non-
substitutable. Consuming Local Closeout Eligibility establishes only the
child's eligibility proposition. A parent policy that requires the child to
have actually closed MUST consume the authoritative current closed-state
proposition and/or exact Mutation Receipt declared by that policy; it MUST NOT
infer committed closeout from eligibility.

A parent policy MUST NOT treat an absent child, unaccounted dependency,
ambiguous hierarchy, historical local result without required current-use
proof, or copied `eligible` label as satisfied. A changed parent snapshot,
child/dependency membership, relation, child target, Local Closeout result, or
declared currentness fact requires a new parent basis and result.

`Parent Eligibility Result = eligible` means only that the exact policy
positively permits consideration of the named parent Action Kind for the exact
parent snapshot. It does not:

- close or mutate the parent;
- promote, merge, release, deploy, or advance anything;
- inherit any child's Action Authorization Binding;
- authorize a parent actor or state owner;
- prove that an action attempt occurred; or
- imply that a later target-state recheck will succeed.

Any parent advancement or promotion is a separately named action. It requires
its own current exact Action Authorization Binding, expected parent snapshot,
an exact Action Attempt Identity, through-commit dependency/currentness
enforcement, an atomic state-owner decision, and Action Terminal. At the
mutation commit boundary, the state owner MUST establish that the exact parent
snapshot and every parent/child/dependency relation declared to remain current
through commit still match the Parent Eligibility and Action Authorization
closures. A committed change additionally requires a Mutation Receipt. Parent
eligibility remains immutable history if that later action refuses or the
target changes.

## Action authorization and mutation

Eligibility and authority are deliberately ordered and non-substitutable:

1. Work Progression or another action domain produces one exact Operational
   Reliance Determination with `eligible / allow` facets for one closed action
   question.
2. The requester forms an exact Action Intent congruent with that action
   question and adds only the actor or principal, claimed delegation or authority
   provenance, and policy-permitted execution-local details.
3. The exact action-authorization owner resolves the Action Intent and either
   issues one exact Action Authorization Binding or one exact Action
   Authorization Refusal. A refusal creates no Binding or Action Attempt.
   Evaluation Authorization, a gate, `allow`, artifact approval, transport
   authentication, or a prior authorization cannot substitute.
4. Before an Action Attempt begins, the executor allocates or resolves one
   exact Action Attempt Identity and obtains exact owner-produced positive
   verification of the Binding, Intent, Operational Reliance Determination,
   expected Action Target Snapshot, and every condition declared for attempt
   admission. The executor MUST NOT reconstruct another owner's authorization,
   target-currentness, or dependency proposition.
5. For a state-changing action, the authoritative state owner enforces at the
   mutation commit or linearization boundary that the same exact Binding
   remains current, the same expected target snapshot remains current, the
   effect is exactly the authorized effect, and every dependency declared to
   remain current through commit still verifies under its owner. A prior
   recheck separated from commit by an unguarded concurrency gap is
   insufficient. If a required cross-owner causal or compare condition cannot
   be established, no state change may commit and no Mutation Receipt may
   issue.
6. The executor produces exactly one Action Terminal for the Action Attempt.
   The authoritative state owner produces a Mutation Receipt only after durable
   commit of the exact authorized state change. The Terminal may report
   committed mutation only by binding that Receipt.

### Authority provenance without a Handbook IAM platform

This semantic action chain does **not** require Handbook to build a general
identity, credential, authentication, delegation, capability-token, secrets,
PKI, RBAC, or ABAC platform. An Action Authorization Binding is the action-
authorization owner's exact semantic result and provenance contract. Concrete
authentication, credential, host-permission, or enforcement mechanisms remain
owned by the systems that actually provide them. Engine Context Resolution
retains its separate semantic admission and verification responsibility for
evaluation reach; neither action authorization nor this clarification replaces
or weakens it.

A standalone CLI or SDK workflow MAY use explicitly permitted **advisory
authority**. Under such an admitted action-authorization rule, a user prompt or
configured agent role may supply action intent, a claimed principal or
delegation, and declared authority. The Binding MUST preserve that exact source
and label those propositions as advisory claims rather than independently
authenticated identity, verified delegation, host-enforced permission, or a
fine-grained credential. Ambient operating-system permissions are real execution
capability, but they are not by themselves Handbook proof of the claimed
principal, delegation, scope, or policy authorization.

Advisory authority may support an effectful Action Attempt only when the exact
action-authorization policy explicitly permits that authority mode for the
Action Kind and target. It cannot fabricate verified facts, satisfy a policy
that requires independently authenticated, owner-verified, or host-enforced
authority, waive the exact `eligible / allow` Determination, or bypass target,
currentness, attempt-admission, and through-commit checks. A real enforcing host
may supply stronger exact facts under its own authority; Substrate is only a
possible future example and is not selected here. Changing advisory source,
claimed principal or delegation, declared authority, assurance mode, or any
verified authority fact requires a new authorization decision and cannot be
hidden inside delivery replay.

An action authorization does not improve Current-Use Eligibility. An
ineligible, indeterminate, not-applicable, or refused reliance result cannot be
overridden by broader action authority. Conversely, an eligible result cannot
manufacture authority. An Action Authorization Binding MAY impose additional
restrictions or execution conditions but MUST NOT authorize a broader or
different action question.

An Action Intent MUST NOT broaden, substitute, or change the Action Kind,
Action Target Snapshot, expected state, effect, effect scope, or any
eligibility-relevant action parameter already decided by the Operational
Reliance Determination. A change to any of those values requires a deliberately
new Operational Reliance Occurrence Identity, Basis, Determination, Action
Intent, and authorization path. A change only to the actor or principal,
claimed delegation, declared authority, or its source requires a new Action
Intent and authorization. A change only to the authorization owner's rule,
basis, or conditions may require a new authorization decision over the
unchanged Intent; it does not rewrite that Intent.

Exact delivery replay under the same Action Attempt Identity and exact attempt
content MUST return the same Action Terminal and any same Mutation Receipt
without re-executing the effect. Changed attempt content under the same Action
Attempt Identity MUST refuse.

A non-idempotent retry is a deliberately new Action Attempt. It is permitted
only when the exact Action Authorization Binding or a fresh successor Binding
permits another attempt, all required dependencies remain exact and current,
and the authoritative effect or state owner has positively established that
the prior attempt did not apply or commit.

When a prior attempt has an uncertain effect or commit status, an ordinary non-
idempotent retry MUST NOT proceed. A later attempt is permissible only under a
separately admitted recovery rule and fresh authorization that explicitly
account for possible prior application and establish duplicate-safe recovery.
Recomputed eligibility, unchanged target appearance, timeout, lost
acknowledgement, or transport failure does not establish non-commit.

## Work Specification relationship

A Work Specification remains the authored plan for one feature, slice,
delivery unit, or other bounded body of work. It may:

- nominate an exact already owner-recognized Operational Reliance Policy or
  other input through a Declared Input Reference when the consuming Work
  Progression definition permits that role;
- be bound as one exact authored input when the policy explicitly declares its
  proposition, scope, revision, and currentness mode; and
- in a later revision, add exact Outcome or Disposition References to already
  closed evaluation, Operational Reliance Determination, Action Authorization
  Refusal, Action Terminal, or Mutation Receipt identities. A Disposition
  Reference to a reliance outcome names the shared Determination and identifies
  its Operational Disposition facet; it does not make the two facets
  independent records.

It MUST NOT create an executable reliance policy, Current-Use Eligibility,
Operational Disposition, Operational Reliance Determination, Action
Authorization Binding or Refusal, Action Attempt, Action Terminal, or Mutation
Receipt merely through acceptance language, a `completed` field, approval
prose, a checkbox, or a reference. Work Progression independently resolves and
closes every declared input. The action authority and state owner independently
authorize and apply any effect.

An operational mutation MUST NOT silently rewrite the Work Specification
revision that preceded it. A later revision may record the exact outcome. That
reference preserves causal order and does not make the artifact the action
owner or the later revision an input to the earlier reliance occurrence.

## Posture-feedback boundary

A semantic gate, Local Closeout Eligibility, Parent Eligibility Result,
Operational Disposition, Action Terminal, or Mutation Receipt does not change
posture. A gate or work-progression result may enter posture reasoning only
through this forward path:

```text
exact Historical Semantic Result
  -> posture-observation-specific Operational Reliance Basis and Determination
  -> separately owner-admitted Posture Observation
  -> recommendation-specific exact policy, basis, and owner result
  -> Posture Recommendation
  -> transition-specific Operational Reliance Basis and Determination
  -> exact Posture Transition Action Intent
  -> Engine-issued current Action Authorization Binding
  -> final authority, approval, and expected-posture-state verification
  -> Engine Action Terminal and, after committed state change, Mutation Receipt
  -> new Resolved Posture Snapshot for a later evaluation
```

The posture-observation owner applies its own exact policy, target/scope
congruence, current-use rule, and admission boundary. The recommendation owner
cannot admit its own observations. A Posture Recommendation does not establish
transition eligibility or authorization. The transition-specific Operational
Reliance Basis MUST independently close the exact recommendation, current
posture target, transition policy, approvals, and every other declared present
dependency. The Engine transition owner then requires its own fresh authority,
current expected posture state, approvals, through-commit verification, and
transition validity.

No observation, recommendation, transition basis, authorization, terminal,
receipt, or successor posture snapshot may alter the posture, requirements,
Evaluation Purpose, evidence closure, Metric Definitions, Gate Policy, or
semantic result of the evaluation occurrence that produced the source result.

## Currentness, invalidation, and replay

The following changes preserve all historical objects but prevent reuse of an
existing Operational Reliance Basis for a new present decision:

| Change or condition | Required consequence |
| --- | --- |
| Action Kind, policy identity/content, policy-selection basis, accepted-result rule, bounded effect, effect scope, or eligibility-relevant parameter changes | Form a deliberately new reliance occurrence, Basis, Determination, Intent, and authorization path under the exact changed action question. |
| Action Target Snapshot, expected state, or result-to-target congruence changes | The prior basis remains history but cannot govern the changed target. Form a new basis and Determination; no rebinding is permitted. |
| A Historical Semantic Result changes | Results are immutable, so this is substitution or corruption and refuses. A genuinely new evaluation result requires a new reliance occurrence. |
| A policy-declared historical-only source receives a newer revision | The exact old source remains usable only as the policy explicitly permits; the newer revision is not substituted and no present authority is implied. |
| A dependency declared current-at-reliance becomes non-current, superseded, stale, or unverifiable | No Operational Reliance Determination may be formed from the old observation. Obtain a fresh positive owner verification or form a new upstream result as the policy requires. |
| Evaluation Authorization Binding becomes non-current under explicit `historical_validity_only`, while its exact historical resolution remains valid | The immutable result retains its historical authorization provenance. No Engine current-use conclusion is inferred, and this change alone does not alter the Determination. All other declared present-use dependencies and separate action authority still apply. |
| Engine reports the exact historical Binding `historically_invalid`, unverifiable, unavailable, ambiguous, or substituted under either mode | Reliance formation refuses without rewriting the Historical Semantic Result. No Eligibility or Disposition facet is produced. |
| The action policy declares `continuing_current` and Engine reports non-current, superseded, unverifiable, unavailable, ambiguous, or substituted | Reliance formation refuses because the required positive owner verification is absent. It never treats the authorization as current or reconstructs Engine truth. A fresh evaluation is required when the policy requires one. |
| Work parent/child/dependency membership or relation changes | A prior Parent Eligibility Result remains history but cannot govern the changed closure. Form a new exact parent basis with complete accounting. |
| Local work state or a required local result changes after Parent Eligibility closes | The parent result remains history. Present parent action stops at final recheck and requires a new parent reliance occurrence. |
| Action Authorization Binding expires, is superseded, no longer matches the target/effect, or its advisory/verified authority source, claimed principal/delegation, authority mode, or required verification facts change before attempt or commit | The already closed Determination remains history; the attempt or mutation refuses. A new authorization decision is required, and new authority never retroactively changes the Determination. |
| Target state or a dependency declared current-through-commit changes after `allow` but before mutation | Commit-boundary enforcement refuses the mutation and no Mutation Receipt issues. Recompute reliance against the new target or dependency closure before requesting new authorization. |
| Exact historical dependency is temporarily unavailable for resolution | Historical results are not rewritten. A display may report the owner-attributed non-success, but operational reliance refuses because the common exact-result floor cannot be established. |

Exact reliance replay resolves the same occurrence, policy, action question,
target snapshot, historical results, reliance-time observations, Determination,
and both facets, or returns the same exact Reliance Refusal for the same refused
formation content.
It MUST NOT substitute current definitions, reacquire evidence, rerun producers,
create new owner verification facts, or claim current action authority. A
present determination, even over equal source content, is a deliberately new
reliance occurrence.

Exact replay of the same Action Authorization request returns the same Binding
or Action Authorization Refusal. Exact replay of the same Action Attempt returns
the same Action Terminal and any bound Mutation Receipt without re-execution.
Changed content under any one of those semantic identities refuses.

An exact historical Action Terminal or Mutation Receipt remains immutable.
Current state is established from the state owner, never by replaying the
receipt or inferring that an earlier successful action still governs the target.

## Semantic responsibility map

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Contracts evaluation responsibilities | Historical applicability, claim, metric, semantic-gate, and canonical-explanation truth for one closed evaluation context. | Operational Reliance Policy, local closeout, parent eligibility, action authorization, work-state mutation, or posture transition. |
| Engine Context Resolution | Historical and current verification of Evaluation Authorization Bindings and every other declared Engine-owned Context Resolution proposition. | Current-Use Eligibility, action-policy selection, Work Progression truth, action authorization merely because an evaluation was authorized, or a general Handbook IAM platform. |
| Engine artifact/profile domain | Exact Work Specification and other artifact revisions, Artifact Subject Binding, artifact currentness, promotion, and lineage under the authored-artifact lifecycle. | Operational work completion, local/parent eligibility, action disposition, or action authority from artifact status or prose. |
| Authoritative work-structure and work-state owner | Exact work identities, current state snapshots, parent/child/dependency relations, state currentness, concurrency, and committed successor state. | Contracts truth, Work Progression policy or eligibility, action authorization merely from state ownership, or mutation without exact authorization. |
| Policy-definition and admission owners | Exact candidate population, deterministic Policy Admission Closure, typed vocabulary and conformance, relevant governance-source roles/scope/currentness, required separately authorized approvals or exceptions, non-relaxable invariant checks, admission/refusal, currentness, and supersession for each Action Kind, reliance, authorization, or recovery policy. | Treating candidate authorship, a bare link, rendered prose, an AI's unconstrained judgment, or a policy's own desired outcome as admission; converting authored text into runtime evidence or a waiver; or recursively letting a candidate admit its own admission rule. |
| Work Progression | Work target and finite parent/dependency closure for its decisions; Local Closeout and Parent Eligibility policies; reliance occurrence, basis, atomic Determination with eligibility/disposition facets, currentness, and replay. | Contracts truth, evaluation authorization, Work Specification authorship, release/deployment policy, action authorization by implication, or mutation. |
| Other downstream action domains | Their exact action-specific reliance policies, bases, atomic Determinations, currentness, and replay under the common floor. | A universal enforcement policy, another action domain's policy, Contracts truth, or Engine authority. |
| Action requester or origin owner | Exact Action Intent request content and durable request occurrence provenance. | Action Kind or target truth, eligibility, authorization, attempt, or effect merely because it requested one. |
| Action authority owner | Exact authorization rule; declared advisory, owner-verified, or host-enforced authority mode and basis; exact provenance of claimed versus verified principal/delegation facts; Action Kind, target, expected state, bounded effect, Operational Reliance Determination binding, issuance time and lineage, refusal, Binding, currentness, supersession, and permitted-attempt rule. | Improving eligibility, rewriting a gate, treating Evaluation Authorization as action authorization, claiming authentication/delegation/enforcement not established by its exact inputs, mandating a general Handbook IAM implementation, or proving an attempt or effect. |
| Action executor | Action Attempt Identity, attempt-admission checks from exact owner facts, exact authorized execution, one Action Terminal per attempt, delivery replay, failure, refusal after the attempt boundary, and effect/commit uncertainty. | Self-producing another owner's authorization/currentness, broadening the effect, or asserting committed state change without the state-owner receipt. |
| Authoritative state or effect owner | Exact expected predecessor, through-commit verification, atomic state change or effect decision, durability, concurrency, exact successor, commit semantic time, and Mutation Receipt. | Inventing eligibility, broadening authority, changing policy, or reporting an uncertain commit as success. |
| Posture observation owner | Exact policy-driven admission of permitted historical results as immutable Posture Observations. | Gate reinterpretation, recommendation by implication, or Engine posture mutation. |
| Posture recommendation owner | Exact policy-driven recommendation from admitted observations. | Observation self-admission, approval, or transition. |
| Engine posture-transition owner | Fresh authority/currentness/approval checks and atomic transition from one exact posture authority state to its successor. | Rewriting the source gate, eligibility, disposition, or recommendation. |
| SDK, CLI, Tauri, Flow, Pipeline, skills, and transports | Bounded composition, presentation, deterministic projections/assemblies, correlation, scheduling, transport-local errors, and exact carriage of prompt- or role-supplied advisory-authority claims around owner objects. | Authenticating a claimed principal/delegation, fabricating host enforcement, satisfying an owner policy that requires stronger facts, policy selection, current-use formation, disposition reinterpretation, action authorization, or mutation merely because they carry or invoke an operation. |

Physical colocation does not collapse these owners. In particular, a future
Engine-hosted Work Progression implementation would still consume rather than
reconstruct Contracts truth, and its eligibility responsibility would remain
separate from any Engine action-authority or mutation responsibility.

## Discriminating scenario corpus

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Historical pass, current local closeout.** Gate `G1` passed for exact work snapshot `W1`. The current Local Closeout Policy accepts its decision-seeking provenance, `W1` remains the current target, all declared dependencies verify, and no other closeout predicate is missing. | Work Progression forms a new reliance occurrence and basis, produces one Determination with `eligible / allow` facets, and stops before action authority. `G1` remains an unchanged Historical Semantic Result. | Historical truth may support current consideration without becoming the action. |
| **Pass is insufficient under another policy.** The same `G1` is supplied to a Local Closeout Policy that also requires an exact unresolved dependency disposition. | If the dependency is a missing formation input, Reliance Refusal produces no Determination. If the policy admits an exact uncertain dependency state, the Determination has `indeterminate / hold` facets. `G1` does not fill the gap. | Gate pass is neither a universal closeout rule nor a missing-input default. |
| **Failed gate enables remediation but blocks closeout.** Exact gate `G2` is `fail`. The Local Closeout Policy makes closeout ineligible, while a separately admitted remediation policy positively requires that failure. | The closeout Determination contains `ineligible / block`; the remediation Determination may contain `eligible / allow`. Neither changes `G2`. | Disposition is action-question-specific, not gate reinterpretation. |
| **Pure advisory prioritization.** A policy forms an exact `eligible / allow` Determination whose action question is limited to a non-mutating prioritization recommendation, and no effectful Action Intent is requested. | The Determination is the terminal result. No priority state is mutated and no Action Authorization Binding, Action Attempt, Action Terminal, or Mutation Receipt exists. Any later authoritative priority change requires its own action chain. | A pure disposition or advisory result is non-mutating and need not recurse into action authorization. |
| **Diagnostic result.** A diagnostic evaluation closes a valid failed gate. One policy permits that exact diagnostic role and direction to trigger remediation; another does not permit it to support positive closeout. | The remediation basis may form under its exact policy. Closeout reliance refuses for incompatible use intent. Permission for remediation, blocking, holding, or observation admission does not make the diagnostic result decision-seeking or permit a different positive action question. | Evaluation Use Intent acceptance is role-, direction-, and action-question-specific. |
| **Ambient favorable-result selection.** A caller asks Work Progression to use “the latest passing gate” for a target without an exact nominated-reference or owner/repository/time-cut boundary. | Reliance formation refuses. Repository order, current-head lookup, result direction, or caller-selected subset cannot establish the finite source-result population. | Complete result accounting requires an exact eligible boundary before selection. |
| **Historical-validity-only mode with later Engine changes.** A valid historical gate binds Evaluation Authorization `A1`. Engine later reports `A1` non-current while historical resolution remains `historically_valid`. In a comparison case, Engine reports historical resolution `historically_invalid` or `unverifiable`. | Under `historical_validity_only`, non-currentness alone does not alter the Determination. The historical-invalid or unverifiable comparison causes Reliance Refusal without rewriting the gate. | Historical validity and continuing-current authorization are separate Engine propositions. |
| **Continuing Engine dependency declared.** A high-risk deployment policy expressly requires the evaluation binding to remain current at reliance time, and Engine reports it superseded. | The action domain preserves the historical result but cannot treat the binding as current. Reliance refuses, no Determination is produced, and a fresh evaluation is required under the policy. | Policy-conditional Engine verification is mandatory when declared and owner-produced. |
| **Eligible but authorization refused.** Local closeout has an `eligible / allow` Determination, but the action-authorization owner refuses the exact actor, authority basis, effect, or scope. | The authorization owner produces one exact Action Authorization Refusal. No Action Authorization Binding, Action Attempt Identity, Action Terminal, mutation, or Mutation Receipt exists. The Determination remains immutable history. | Eligibility, authorization refusal, attempt provenance, and mutation truth are distinct propositions. |
| **Authorized but ineligible.** A broad action credential exists, but the current reliance result is `ineligible / block`. | No action-intent authorization may validly bind the ineligible result, and no mutation occurs. Broader credentials cannot override the action-domain decision. | Authority cannot improve semantic eligibility. |
| **Same kind and target, changed effect.** A Determination establishes `eligible / allow` for exact effect `E1` against target snapshot `T1`. A later Action Intent uses the same Action Kind and target label but requests materially different effect or effect scope `E2`. | Intent formation or authorization refuses the incongruence. A deliberately new reliance occurrence, Basis, Determination, Intent, and authorization path are required for `E2`. | An allow cannot be broadened through later Action Intent parameters. |
| **Target changes between allow and commit.** Local closeout is allowed and an exact Action Authorization Binding is issued for target snapshot `W1`, but the work-state owner now reports `W2`. | Final expected-state verification refuses. The prior Basis, Determination, and authorization remain history; a new reliance occurrence against `W2` is required. | Present-use closure does not bypass mutation-time concurrency. |
| **Uncertain commit followed by retry.** An authorized non-idempotent Action Attempt loses its definitive state-owner outcome and has an uncertain Terminal with no Mutation Receipt. The target presentation appears unchanged and the caller requests another attempt. | The ordinary retry refuses. It may proceed only after the authoritative effect/state owner positively establishes non-application or under a separately admitted duplicate-safe recovery rule and fresh authorization that explicitly account for possible prior commit. | Timeout, unchanged appearance, and recomputed eligibility do not prove non-commit. |
| **One child is locally eligible.** Child `C1` has exact Local Closeout Eligibility `eligible`; parent `P1` has another required child `C2`. | No Parent Eligibility Result can be inferred from `C1`. Work Progression must form the exact parent/child closure and account for `C2` under a Parent Eligibility Policy. | Local eligibility never implies parent eligibility. |
| **Complete parent closure.** Parent snapshot `P1` has exact required children `C1` and `C2`; the policy consumes current exact local results for both plus one parent-level gate, and all positively satisfy it. | Work Progression may produce one parent-action Operational Reliance Determination with `eligible / allow` facets. The parent remains unchanged and no promotion has occurred. | Parent eligibility is a separate pure composition facet. |
| **Parent closure omits a required relation.** The finite authoritative boundary contains child `C3`, but the proposed parent Basis accounts only for `C1` and `C2`. In a comparison case, the policy requires actual `C3` closeout but supplies only `C3` Local Closeout Eligibility. | Both cases refuse before Parent Eligibility. The first lacks complete relation membership; the second supplies the wrong child proposition and requires the declared current-state or Mutation Receipt proof. | Complete relation closure and proposition-role congruence precede parent eligibility. |
| **Dependency added after parent eligibility.** `P1` was eligible, then the work-structure owner adds required child `C3` before parent action. | The historical parent result remains exact for its old closure. Final currentness recheck stops the action; a new parent basis must account for `C3`. | Parent membership currentness is part of present use and cannot be patched in place. |
| **Parent eligible, promotion refused.** Parent eligibility is `eligible / allow`, but action authorization is superseded or the compare-and-write expected parent state no longer matches. | No promotion commits and no Mutation Receipt exists. The parent Determination is not rewritten to `ineligible`. | Parent eligibility, promotion authority, and mutation success are three propositions. |
| **Work Specification says completed.** A current Work Specification revision contains `completed` prose and cites a gate summary, but no exact owner result is resolved. | No Local Closeout or Parent Eligibility forms. The artifact remains authored project truth only. | Authored planning state and rendered references grant no operational authority. |
| **Later Work Specification records outcome.** After a valid local mutation receipt `M1`, a new Work Specification revision adds an Outcome Reference to `M1`. | The new artifact revision records the relationship. It neither caused `M1` nor changes the earlier reliance, authorization, action, or receipt. | Artifact references preserve causal direction and owner boundaries. |
| **Compact gate projection offered for action.** A UI supplies a concise gate view but the canonical gate result cannot be resolved. | Operational Reliance refuses. The projection remains useful presentation but cannot satisfy the exact-result floor. | Presentation density does not create action truth. |
| **Gate feeds posture through later owner stages.** A gate failure forms a valid posture-observation-specific Operational Reliance Determination and is separately admitted as a Posture Observation. A recommendation policy produces an exact Posture Recommendation. A distinct transition-specific Basis and Determination then close over the recommendation, exact current posture target, approvals, and other declared facts before Engine issues transition Action Authorization. | Only the final Engine-authorized compare-and-write may create the successor posture snapshot and Mutation Receipt. The originating gate and evaluation remain unchanged and the successor posture affects only a later evaluation. | Observation, recommendation, transition eligibility, transition authority, and mutation are separate and cannot form a same-occurrence cycle. |
| **Historical replay after policy supersession.** An old Basis, Determination, and Action Terminal are replayed after the Operational Reliance Policy has a successor. | Exact replay returns the old historical results and policy identity. It grants no current disposition or authority under the successor; present use requires a new occurrence. | Historical reconstruction and present reliance are distinct. |
| **Exact attempt replay.** The same Action Attempt Identity and exact attempt content are delivered again after its Terminal and Mutation Receipt closed. A comparison case changes the effect under that attempt identity. | Exact replay returns the same Terminal and Receipt without re-executing. The comparison refuses conflicting identity reuse. | Attempt replay and a deliberately new execution are distinct. |
| **Transport reports success without receipt.** A CLI or host reports a successful call, but the state owner cannot establish whether the write committed and emits no Mutation Receipt. | The effect is uncertain and MUST NOT be presented as a successful mutation. Eligibility and authorization cannot repair the missing state-owner truth. | Transport success and action intent are not durable mutation truth. |
| **Standalone advisory authority is explicitly permitted.** A local CLI action policy permits advisory authority for one exact Action Kind. A user prompt supplies the intent, claimed principal, claimed delegation, and declared authority; the process also has ambient operating-system permission to write the target. | The action-authorization owner may issue a Binding that preserves the prompt provenance and labels the authority facts advisory. The attempt still requires the exact `eligible / allow` Determination, target-currentness and attempt checks, and through-commit state-owner verification and Receipt. Neither the Binding nor ambient OS permission is represented as independent authentication, verified delegation, or fine-grained Handbook credential. | Advisory authority can participate in the complete effectful chain without fabricating stronger assurance or bypassing mutation safeguards. |
| **Prompt offered where enforced authority is required.** Another Action Authorization policy requires an independently authenticated principal and host-enforced permission. A prompt or configured agent role declares both but supplies no owner-verifiable fact. | Authorization refuses. The declared claims cannot satisfy the stronger policy, and advisory mode cannot waive the requirement or target/currentness checks. | Policy-required enforcement cannot be fabricated by advisory inputs. |
| **Agent-authored policy candidate.** An agent proposes an Operational Reliance Policy and cites one Decision Record by Markdown link, omits the exact scope and candidate population, and asks the admission owner to judge whether it is generally safe. | Policy admission refuses. A valid Policy Admission Closure requires exact candidate content, typed roles and scope, relevant exact governance-source revisions and dependency modes, complete selection/accounting, and mechanical invariant checks. It neither requires every artifact family nor turns the Decision Record into runtime evidence or a waiver. | Candidate authorship and organizational rationale are permitted, but deterministic owner admission is separate and bounded. |

## Settled versus deferred

This record settles:

- Historical Semantic Result, Operational Reliance, Action Kind, Action Target
  Snapshot, Action Question, Operational Reliance Policy and Occurrence Identity, Operational
  Reliance Basis and Determination, Current-Use Eligibility, Reliance Refusal,
  Operational Disposition, Action Intent, Action Authorization Binding and
  Refusal, Action Attempt Identity, Action Terminal, and Mutation Receipt as
  non-substitutable concepts;
- exact action-question closure over kind, target, effect, effect scope, and
  every eligibility-relevant parameter before reliance, plus mandatory Intent
  and authorization congruence;
- the common exact-result, current-policy, current-target, congruence,
  declared-currentness, and complete-accounting floor for present use;
- finite exact eligible historical-result boundaries, diagnostic provenance by
  role/direction/action question, and refusal/replay behavior;
- universal historical resolution and validity of the exact Evaluation
  Authorization Binding, mandatory explicit `historical_validity_only` or
  `continuing_current` policy mode, and fresh Engine verification exactly when
  a reliance or later authorization policy declares a present Engine-owned
  dependency;
- fresh action authorization, exact Action Attempt identity, and commit-boundary
  authorization/target/through-commit verification that no policy may disable;
- the exact eligibility states and fixed disposition mapping as facets of one
  atomic Operational Reliance Determination, plus the formation-refusal
  boundary;
- Work Progression as semantic owner of local closeout and parent eligibility,
  including target/dependency closure, policies, bases, results, currentness,
  accounting, and replay;
- Local Closeout Eligibility and Parent Eligibility Result as separate pure
  decisions with separate policies and currentness closures;
- finite authoritative parent-relation boundaries, explicit relation reach and
  cycle treatment, complete membership accounting, and non-substitutable child
  proposition roles;
- the rule that local eligibility never establishes parent eligibility, parent
  eligibility never establishes promotion, and every state effect requires
  separately authorized action and state-owner receipt;
- prohibition of ordinary non-idempotent retry after uncertain effect or
  commit absent authoritative non-application or admitted duplicate-safe
  recovery with fresh authorization;
- explicit placement of every introduced object in the canonical six-category
  ontology without creating another Artifact Instance family;
- deterministic owner-side Policy Admission Closure for user-, agent-, artifact-,
  or tool-authored policy candidates, with exact relevant governance inputs,
  selection/accounting, mechanical non-relaxable checks, and no bare-link proof,
  authored-text waiver, open-ended AI safety judgment, or recursive self-admission;
- Action Authorization Binding as a semantic result/provenance contract rather
  than a mandate for a general Handbook IAM platform, including explicitly
  policy-permitted advisory authority and its non-substitution for authenticated,
  owner-verified, or host-enforced facts;
- Work Specification reference and causal-order boundaries;
- the gate-to-posture observation/recommendation/transition boundary; and
- currentness, invalidation, retry, exact replay, and scenario outcomes above.

Deliberately deferred are all concrete Local Closeout, Parent Eligibility,
release, deployment, remediation, posture-observation, or other action-policy
content; thresholds; selected result populations; work hierarchy and dependency
model; work-state or parent-state machine; actor and approver rules; concrete
authorization mechanism; mutation effects; persistence and retention policy;
and the physical placement of Work Progression, action authority, or state
ownership. No concrete parent promotion, release, deployment, or posture
transition is selected.

Also deferred to later human-approved task-level planning are schemas, fields,
Rust types, APIs, operations, repositories, registry/resolver mechanics,
identity allocation, cache implementation, status-code encoding, crate/module
placement, SDK/CLI/Tauri/Flow/Pipeline/skill integration, UI and explanation
projection, task slices, acceptance criteria, implementation sequence, and
rollout. This record does not authorize changes to completed HCM-5.2 source or
claim that the proposed Work Progression owner exists today.

Human approval of this proposed record would make it semantic input to later
planning only. It would not authorize implementation, change any phase or
slice status, close or promote any work, or establish a delivery outcome.
~~~~~~~~~~~~
