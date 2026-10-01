# HCM semantic model — Objects and governance

**Status: PROPOSED**

Part of the [combined semantic package](README.md). Its approval boundary and
shared greenfield/current-product distinction apply throughout.

Object categories, semantic ownership, policy admission, and authored-artifact relationships share one home. Grouping them does not merge the artifact owner with any consuming policy, evaluation, or action owner.

## Reading map

- [Shared policy admission](#policy-admission)
- [Contract permitted use and qualification](#contract-permitted-use)
- [Ontology and ownership](#ontology)
- [Authored artifacts and relationships](#authored-relationships)

<a id="policy-admission"></a>
## Shared policy admission

This section owns the common definition/policy-admission rule for every chapter
in this package. Domain sections retain their exact vocabulary, compatibility,
selection, and use constraints. A particular exception record or action follows
its own typed authorization and admission; it is not a policy candidate merely
because an authored artifact references it.

A user, agent, authored artifact, or another tool MAY author policy-candidate
content. Candidate authorship supplies proposed content, provenance, and
organizational judgment only. The responsible semantic owner MUST independently
and deterministically produce a Policy Admission Closure before the candidate
becomes an admitted category-2 definition or policy. The category-3 Closure
closes admission or refusal of one exact candidate under one exact owner
admission-rule version; it does not mandate separate Policy Admission Basis and
Policy Admission Result runtime object types.

The Closure MUST bind:

- exact candidate content, governing project configuration version and semantic-
  content identity, exact applicable owner admission-rule version and semantic-
  content identity, and the active-at-decision basis defined below;
- typed vocabulary, conformance, domain compatibility, proposition roles, and
  exact scope;
- the complete finite candidate boundary, selection, and accounting;
- every relevant exact Charter, Project Context, Decision Record, or Risk Record
  revision, with exact owner identity, proposition role, scope, and historical,
  formation-only, or continuing-current dependency mode;
- each separately typed and authorized approval or exception source required by
  the rule, including its authority, scope, identity/version or revision as
  applicable, and currentness; and
- compliance with every applicable non-relaxable invariant.

Only governance sources declared relevant by the admission rule are required;
no rule here demands every artifact family for every policy. Artifact promotion,
prompt authority, agent roles, bare Markdown links, accepted prose, and
human-readable approval language alone never perform admission. Authored text
remains governance input and MUST NOT be reclassified as runtime evidence, a
waiver, or an independently authenticated authority fact.

Admission applies the typed constraints, complete accounting, scope/role rules,
and non-relaxable invariants mechanically. "Unsafe" means prohibited by an
admitted constraint or invariant, not an AI's open-ended organizational judgment;
a refusal MUST identify the violated rule. Organizational judgment may enter
through exact governance sources without requiring a new human prompt for every
candidate. The owner admission rule is an exact governance root outside the
candidate population it decides. A candidate cannot select or recursively admit
its own admission rule; a changed rule has its own successor rather than an
unbounded self-admission chain.

### Establishment and replacement of governing admission requirements

Handbook MUST define an exact, versioned establishment rule for project admission
governance. This Handbook-defined rule and the applicable non-relaxable semantic
constraints are the starting authority outside the project policy-candidate
population. Neither an ordinary policy candidate nor a proposed governing
configuration may select, admit, weaken, or replace the rule that authorizes its
own establishment. Changing Handbook's establishment rule is not a project
configuration operation.

An independently attributable **project admission-governance responsibility**
MUST establish the governing configuration, exact project and admission scope,
applicable owner-rule bindings, active-version relation, and successor history.
Domain owners retain validation of their own requirements and admission of
ordinary policy candidates. This responsibility does not select a physical
component, service, repository, or new authored artifact family.

In the supported standalone advisory mode, the Handbook establishment rule
permits an explicit local initialization instruction, including a configured
agent acting under that instruction, to supply advisory establishment authority.
Initialization MUST establish that initial setup applies to the exact project,
validate the exact configuration and owner-rule bindings, and record the accepted
instruction and the project's replacement-authority basis. Missing, unreadable,
or unverifiable governance in an established project MUST NOT authorize another
initialization. Adding a scope in an established project MUST follow its existing
governance rather than create another bootstrap route.

Initialization and successor activation follow this bounded governance lifecycle,
not ordinary candidate admission or an action policy that depends on the
configuration being established. Configuration validation, permission for the
exact activation, and confirmation of committed active-state change MUST remain
separately attributable propositions; one owner outcome may carry them without
requiring separate runtime record types. This lifecycle supplies no authority for
evaluation, work progression, posture mutation, or other operational effects.

Configuration versions MUST have exact immutable semantic identities. Every
supported admission MUST resolve one unambiguous governing configuration and
owner-rule basis for its exact project and scope. Missing, conflicting,
unsupported, substituted, or unverifiable governance MUST refuse the affected
operation. Candidate preference, directory proximity, repository order, mutable
file contents, or a "latest" label MUST NOT select governing authority. No implicit
scope inheritance, override priority, or configuration merge is established here.

Replacement MUST deliberately bind the exact current predecessor, proposed
successor, affected scope, and previously established replacement-authority basis.
It MUST validate the successor against Handbook's fixed constraints and applicable
domain constraints. The successor cannot supply missing permission for its own
activation. Where changes to replacement-authority terms are supported, the prior
terms MUST authorize the change; new terms take effect only after activation.
Unsupported changes refuse. The same user or agent may author a candidate and
separately request replacement; these are distinct decisions, not necessarily
different people. Replacement need not satisfy the predecessor's ordinary
candidate-admission requirements or make every configurable requirement stricter;
it cannot relax Handbook's non-relaxable floor.

The authoritative owner MUST establish one activation cutover against the exact
current predecessor, preserving predecessor-to-successor history. A stale
predecessor cannot be overwritten by a replacement request. No activation success
may be reported unless the exact active-state change committed. Authoring,
editing, saving, or validating inactive content does not activate it. Changed
semantic content under an existing immutable identity MUST refuse. Exact replay
returns the earlier outcome without re-executing activation or restoring an old
active version. An uncertain activation outcome MUST remain uncertain until
authoritative state is resolved; timeout is not proof of non-commit or permission
to repeat an effect that might already have committed.

Each new Policy Admission Closure MUST finalize against the exact governing
configuration and owner-rule basis active at its decision boundary. A change
before finalization MUST refuse stale-basis finalization; a fresh decision may be
requested under the newly resolved basis. Completed closures remain exact history
and may be replayed without becoming new admissions. Failure to establish the
governing basis MUST preserve available decisive provenance without inventing an
applicable version or admitting a policy.

Replacement MUST NOT rewrite earlier admission/refusal closures, explanations,
or authority bases. It governs subsequent admissions but does not by itself
re-admit, revoke, or establish successor-rule compliance for existing policies.
Present usability remains subject to definition-owner and consumer currentness,
compatibility, scope, and declared dependency rules. A required assessment under
successor requirements produces a distinct owner determination. Existing exact
requirements, purposes, evidence, and results cannot acquire substituted policy
definitions merely because a configuration changed.

Establishment, activation, and refusal outcomes MUST preserve the exact Handbook
establishment rule, project/scope, configuration identity and semantic content,
owner-rule bindings, predecessor when applicable, accepted instruction and
authority basis, validation, and activation outcome, insofar as established.
Refusal MUST identify the missing or violated condition without fabricating
unresolved inputs. Authority provenance MUST distinguish advisory principal or
delegation claims from independently verified identity, delegation, and enforcement
facts. Other propositions, including configuration identity, validation, current
predecessor, and committed activation, can remain owner-verified. Filesystem
access, an asserted name, a prompt, or a configured role cannot prove stronger
authority than the accepted basis supplies. Advisory authority does not waive
validation, exact scope, currentness, or commit checks.

A Decision Record MAY explain the choice; it is neither mandatory nor an
activation mechanism. No separate service, login, human reconfirmation, or general
identity platform is required. Concrete representation, APIs, owner placement,
storage, activation/concurrency mechanics, and recovery mechanics remain deferred;
the authority, cutover, history, and refusal semantics above do not.

### Admission-governance examples

| Situation | Required outcome |
| --- | --- |
| A local user explicitly initializes project P with configuration C1 and a permitted instruction basis for later replacement. | Handbook applies its fixed establishment rule, validates C1 and its exact scope, and records advisory instruction authority and committed activation. No Decision Record, login, or second approval prompt is inherently required. |
| A candidate says to use its own admission rule; a successor grants permission needed to install itself. | Neither supplies governing authority. The candidate uses existing governance; successor activation needs permission established beforehand. |
| A draft C2 is saved, or C1's immutable identity is reused for different content. | Saving the draft leaves it inactive. Conflicting identity reuse refuses. |
| C2 activates while a candidate is being checked under C1. | A C1 decision completed before cutover remains history; one not yet finalized refuses stale-basis finalization. A fresh C2 decision is distinct. Replaying a completed C1 decision or activation cannot restore C1. |
| A policy admitted under C1 is considered after C2 replaces it. | Its C1 admission remains history, not proof of C2 compliance. Present use follows the exact owner's and consumer's dependency/currentness rules; replacement alone neither revokes nor re-admits it. |
| Established governance is unreadable, or activation's commit is uncertain. | Missing state does not authorize reinitialization. An uncertain operation cannot report success or repeat a possibly committed effect before authoritative resolution. |

<a id="contract-permitted-use"></a>
## Contract permitted use and qualification

Every admitted contract revision MUST have one exact governed **Contract
Permitted Use** value:

| Value | Permitted Evaluation Use Intent |
| --- | --- |
| **Diagnostic-only** | `diagnostic` only, even when all readiness facts establish `ready`. |
| **Decision-seeking eligible** | `diagnostic` or `decision_seeking`, subject to all other requirements. |

This is one semantic property, not an independently editable class plus a second
allowed-intent list. It is part of the contract revision's immutable semantic
content, owned by the Contracts contract-definition/admission responsibility.
It is distinct from Project Classification, posture, admission status, maturity,
deployment environment, readiness, governing assignment, evaluation outcome,
and completion authority. Both values require complete contract semantics for
their own bounded questions. Neither warrants a softer meaning of `pass`.

An author may propose a value; owner admission establishes it under the exact
active project admission configuration and applicable Contracts admission rule
defined by [shared policy admission](#policy-admission). Admission MUST establish
supported explicit permitted use, complete candidate semantics, satisfaction of
the applicable use-qualification requirements and declared governance inputs,
and the authority and predecessor/currentness conditions required by the owner's
native operation. Missing, ambiguous, unsupported, or unverifiable qualification
basis, or unsatisfied required conditions, refuses admission. A candidate cannot
select its own qualification requirements; a name, prompt, previous diagnostic
pass, or requested run intent cannot supply missing qualification authority.

Initial candidates and successor candidates requesting the same permitted use
MUST satisfy the same applicable qualification requirements. Creating a fresh
contract cannot bypass the requirements for broadening an existing contract's
use. Changing permitted use changes semantic content and requires a new exact
admitted revision or successor even when the claims are unchanged. Updating a
current-head or supersession relation additionally follows the owner's exact
authorized lifecycle and expected-predecessor checks. Admission alone neither
updates that relation nor assigns the contract as governing an actual project
obligation. Configuration replacement does not reclassify earlier contracts.

No mandatory promotion sequence, previous diagnostic pass, separate approval
artifact, recursively required validation contract, extra human confirmation,
or identity service is introduced. A contract may start as Decision-seeking
eligible or remain Diagnostic-only indefinitely under its applicable rules.
Concrete qualification criteria remain project-specific and must be selected
before the dependent admission can succeed; deterministic owner admission is
the base mechanism, not an open-ended judgment of organizational safety.

The [requirements and preparation boundary](assurance-and-evaluation-preparation.md#evaluation-requirements)
preserves permitted use through both requirement sources and refuses an
incompatible intent before valid Purpose formation. Policy or exception cannot
widen the same revision's permitted use or relabel an old result. Separately
authorized new revisions remain possible. The [governed-completion boundary](operational-reliance-and-actions.md#governed-completion)
independently establishes which eligible contract and requirements govern the
actual target; eligibility for decision-seeking is never that assignment.

<a id="ontology"></a>
## Ontology and ownership

This chapter classifies canonical objects and assigns semantic responsibilities. It does not redefine the detailed algorithms or state machines owned by the other chapters.

### Purpose and scope

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
  [evaluation identity and evidence-closure decision](evidence-and-evaluation.md#identity-evidence)
  owns evaluation-occurrence identity, evidence-set formation and closure,
  prospective applicability, semantic times, and historical resolvability;
- the proposed
  [authored-artifact and evaluation-relationship decision](objects-and-governance.md#authored-relationships)
  owns exact relationships among the six authored artifact families and owner
  definitions, records, results, and provenance; and
- the proposed
  [operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
  owns present-use reliance, atomic eligibility/disposition, local closeout,
  parent eligibility, action authorization, attempts, terminals, and mutation
  receipts.

All remain `PROPOSED`. Their presence supplies exact companion references, not
human approval, implementation authority, physical placement, or rollout.

### Canonical vocabulary

The following terms are normative. Terms in separate rows are not synonyms.

| Term | Fixed meaning |
| --- | --- |
| **Semantic owner** | The bounded-context responsibility that alone establishes, rejects, verifies, or changes the exact proposition represented by an object. Storage, rendering, transport, or orchestration does not transfer this authority. |
| **Canonical object** | An exact object whose semantic owner recognizes its identity and content as authoritative for one bounded proposition. Canonicality is owner- and proposition-scoped; it does not mean “stored in the generic artifact repository,” “currently applicable,” “globally trusted,” or “safe for every use.” |
| **Authored canonical project artifact** | Human- or governance-authored project truth governed through the generic Artifact Kind / Artifact Instance lifecycle, including candidate formation, validation, promotion, exact revision, authorized mutation, and lineage. Only the six named families in this record have this status. |
| **Canonical semantic definition or policy** | An immutable, named, versioned rule or vocabulary definition established by its semantic owner through its native lifecycle. Ordinary project policy candidates require Policy Admission Closure; governing admission configurations use the bounded establishment lifecycle in Shared policy admission, rooted in a Handbook-defined rule outside that candidate population. It defines how later inputs are resolved, admitted, evaluated, or interpreted; it is not a result of applying itself. |
| **Policy Admission Closure** | The one immutable owner-produced canonical record that closes admission or refusal of one exact policy candidate under one exact owner admission-rule version. It binds exact candidate content; the governing project configuration version, applicable owner admission-rule version, their semantic-content identities, exact scope, and active-at-decision basis; typed vocabulary and conformance; finite candidate selection and accounting; every relevant exact governance-source revision with proposition role, scope, and historical, formation-only, or continuing-current dependency mode; any separately typed and authorized approval or exception source required by that rule; and all non-relaxable invariant checks. The name **Closure** is selected because one exact category-3 owner result closes both basis and outcome without mandating separate `Policy Admission Basis` and `Policy Admission Result` runtime object types. |
| **Owner-produced canonical record or result** | An immutable determination, closure, verification, admission, disposition, or receipt produced by applying one semantic owner's rules to exact inputs. It remains owned by that domain even when another domain consumes it. |
| **Durable origin, attempt, or source provenance** | An immutable occurrence record of what an origin owner requested, attempted, observed, reported, applied, refused, or completed. It may be canonical for that bounded occurrence assertion while remaining untrusted or insufficient for a downstream semantic proposition. |
| **Deterministic projection** | A purpose-, audience-, disclosure-, and definition-bound view of exact source truth. It may select, redact, collapse, render, or assemble while recording its source identities, omissions, lossiness, and output identity. It is authoritative only about that projection operation, not the source proposition. |
| **Representation** | A Rust DTO, canonical JSON value, CLI text, Tauri payload, schema-valid message, or other encoding that carries an owner object or projection. Encoding identity and transport correlation do not create semantic owner identity or authority. |
| **Deterministic assembly** | A projection that combines multiple exact source objects or projections and owns only the selection, ordering, omission, budget, and assembly accounting. A Flow packet is the principal current example. |
| **Transient operational state** | Live execution state that coordinates work but has not been established as a durable owner assertion: for example a queued launch, open process handle, in-memory scheduler state, or transport-local failure. Logging it does not by itself promote it to canonical provenance. |
| **Generic Artifact Kind / Artifact Instance machinery** | Engine-owned definitions and lifecycle behavior for the six authored canonical project-artifact families. It is not a universal registry or repository for every canonical object. |
| **Owner repository** | The semantic owner's durable boundary for definitions, records, provenance, or results under that owner's identity, admission, lifecycle, currentness, and replay rules. Physical storage may be shared without merging semantic ownership. |
| **Cross-owner catalog** | A non-authoritative index or lineage projection that lets consumers discover heterogeneous objects by exact owner, category, identity, and relation. Catalog inclusion does not make an object an Artifact Instance or transfer authority. |

#### Orthogonal properties

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
| **2. Canonical semantic definitions and policies** | Immutable owner-established rules that define later resolution, admission, evaluation, or action semantics, including governing configurations established through the bounded shared-admission lifecycle. |
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

### Current product boundary

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

### Normative ontology invariants

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
- All canonical definition and policy candidates follow the
  [shared policy-admission rule](#policy-admission). Ordinary project candidates
  require Policy Admission Closure; governing configurations instead follow that
  section's bounded establishment/activation lifecycle under the fixed Handbook
  rule. That base case is not an alternative admission path for ordinary policies.
  Authorship and artifact promotion do not establish executable admission.
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

### Object classification

The table classifies the important objects already named by the companion
decisions. It does not redefine their detailed fields or behavior.

| Objects | Ontology category | Semantic owner and boundary |
| --- | --- | --- |
| Handbook admission-governance establishment rule; governing project admission configurations | Canonical semantic definitions or policies | Handbook defines the fixed establishment rule outside project candidate admission. Project admission governance establishes exact configurations, scope, owner-rule bindings, and activation under that rule; configuration content does not admit or activate itself. |
| Admission-governance establishment, activation, and refusal outcomes | Owner-produced canonical records or results | Project admission governance owns exact validation, authority basis, predecessor/cutover, and committed activation or refusal. These outcomes do not create a seventh authored artifact family or grant authority for unrelated effects. |
| Project Authority / Charter; Project Context; Environment Context; Work Specification; Decision Record; Risk Record | Authored canonical project artifacts | Engine's artifact/profile domain owns generic kind, instance, lifecycle, revision, validation, promotion, mutation, and lineage semantics; project authors and authorized governance supply the content. |
| Artifact Kind definitions; Artifact Instance Descriptors; profiles; renderer and projection definitions; Context Resolution definitions | Canonical semantic definitions or policies | Engine admits and resolves its system definitions. The descriptor defines an artifact instance; it is not itself that authored artifact. |
| Contract revisions, including Contract Permitted Use; claim definitions and inventories; claim-set definitions; Evidence Evaluation Policies; Metric Definitions; Gate Policies | Canonical semantic definitions or policies | Contracts definition authorities own executable meaning, permitted-use qualification under the active admission rules, immutable revision identity, and exact resolution. |
| Assurance Policy Selector; Assurance Policies; policy mappings; typed exception policies | Canonical semantic definitions or policies | Contracts-domain assurance-governance definition authorities own admission and currentness of their executable definitions, mappings, and candidate inputs. Assurance Selection resolves and applies only those admitted inputs; it cannot admit or authorize its own inputs. |
| Producer Semantic Capability definitions | Canonical semantic definitions or policies | A distinct Producer Capability Authority within the broader Contracts bounded context independently admits the semantic representational capability used for proof-plan compatibility. A producer or Dock cannot self-admit it. |
| Dock or adapter operating-ceiling definitions and producer retry/partial-completion policies | Canonical semantic definitions or policies | The Dock or adapter owner establishes its bounded operating contract. That contract is not Engine authorization, Producer Semantic Capability, or host enforcement. |
| Action Kind definitions; Operational Reliance, Local Closeout, Parent Eligibility, Action Authorization, posture-observation, posture-recommendation, workflow, recovery, release, or deployment policies | Canonical semantic definitions or policies | The responsible definition or downstream action domain owns each exact admitted definition. This record selects no universal enforcement owner and no policy becomes admitted merely because a user, agent, artifact, or transport authored it. |
| Live, unclosed policy-candidate submission | Transient operational state | It is proposed content at an admission boundary. Candidate authorship or transport does not create a category-2 definition. |
| Origin-closed policy-candidate submission occurrence | Durable origin, attempt, or source provenance | The user, agent, artifact, tool, or adapter origin owns the exact historical assertion that the candidate was submitted. It is not policy admission. |
| Policy Admission Closure | Owner-produced canonical record or result | The exact definition/admission owner deterministically closes admission or refusal under the exact governing configuration and owner rule active at the shared decision boundary, with typed conformance, finite candidate accounting, relevant exact governance-source roles/scope/currentness, required typed approvals or exceptions, and non-relaxable invariant checks. It is one category-3 closure rather than a required pair of new runtime object types. |
| Private `ContextResolutionEnvelope`, Resolved Profile, exact stack, and Engine authority-admission lineage | Owner-produced canonical records | Engine owns private authority truth and exposes only bounded public projections or opaque resolvable references. |
| Posture Scope; Resolved Posture Snapshot; posture-transition result and resulting authority state | Owner-produced canonical records or results | Engine posture responsibilities own scoped posture resolution and separately authorized mutation truth. |
| Exact selection-basis closure; policy-resolution basis; Resolved Assurance Requirements | Owner-produced canonical records or results | Contracts-domain Assurance Selection closes the exact admitted candidate set and selection inputs, owns complete accounting, and produces the exact selection outcome. It does not establish governance admission or currentness for the definitions and mappings it consumes. |
| Direct-contract Evaluation Requirements closure | Owner-produced canonical record or result | Contracts-domain direct requirement resolution closes one exact admitted contract and complete compatible evaluation inputs. Evaluation Requirements is the common semantic role fulfilled by this closure or posture-selected Resolved Assurance Requirements; it requires no additional standalone record. |
| Admitted exception input | Owner-produced canonical record or result | A distinct Contracts-domain exception-admission responsibility establishes exact policy conformance, authority, identity, scope, and currentness. The exact issuer or governance authority authorizes the source record; Assurance Selection only applies an already admitted bounded effect. |
| Observation Plan; Producer Selections; Evaluation Purpose, including its bound Evaluation Use Intent; Readiness Basis and Assessment; Readiness Refusal; Attempt Disposition; Prospective Evaluation Basis | Owner-produced canonical records or results | Contracts-domain Evaluation Coordination owns these pre-truth closures and results. |
| Occurrence Collection Frontier, including the complete-pruning outcome establishing occurrence commitment and empty frontier together | Owner-produced canonical record or result | Evaluation Coordination owns the exact basis commitment and collection boundary, using real origin terminals on the ordinary route or the supported no-collection lifecycle. This is neither fabricated origin provenance nor work completion, and implies no new runtime record family. |
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

### Generic artifact inclusion and exclusion

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

### Semantic responsibility map

The stable dependency direction remains Engine inward of Contracts, and owner
crates inward of SDK and transports. The table assigns semantic
responsibilities, not physical crates, modules, storage engines, or APIs.

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Engine artifact and profile domain | The six Artifact Kind families, Artifact Instance Descriptors, profile conformance, authored artifact lifecycle and repositories, canonical project-artifact reads, and Engine-owned artifact render/projection definitions. | Contract definitions, assurance selection, evaluation readiness, evidence or gate truth, or generic authority over non-artifact owner records. |
| Engine posture domain | Exact Posture Scope resolution, Resolved Posture Snapshot, snapshot verification, and separately authorized posture transition. | Assurance selection, evidence evaluation, gate truth, or transition authority inferred from a recommendation. |
| Engine Context Resolution domain | Complete envelope admission, non-expansion, Evaluation Authorization issuance/refusal, private lineage, verification, historical validity, currentness, and supersession. | Requirements, producer capability, readiness, evidence adequacy, gate truth, host enforcement, or downstream action authority. |
| Project admission governance | Establishment under the fixed Handbook rule; exact configuration/scope and owner-rule bindings; previously authorized successor activation; cutover, active-version relation, history, replay, and truthful authority provenance. Physical placement remains deferred. | Candidate self-authorization, implicit file activation, stale predecessor overwrite, history rewriting, fabricated authentication, or authority for unrelated operational effects. |
| Definition and policy-admission owners | Exact candidate populations; resolving the active governing configuration and applicable owner-rule basis; deterministic Policy Admission Closures at the shared decision boundary; typed vocabulary/conformance; relevant exact governance-source proposition roles, scopes, and dependency/currentness modes; required typed/authorized approvals or exceptions; non-relaxable invariant checks; admission/refusal, currentness, and supersession. | Treating authorship, prompt/agent role, artifact promotion, bare links, open-ended AI safety judgment, or policy-favorable outcomes as admission; requiring every artifact family; turning authored text into runtime evidence/waiver; or letting a candidate recursively admit its own admission rule. |
| Contracts-domain assurance governance and exception admission | Admission and currentness of Assurance Policy Selector, Assurance Policy, policy-mapping, candidate-policy, and typed exception-policy definitions through exact Policy Admission Closures; independently attributable admission of exact authorized exception records. | Posture resolution, closing the selection basis, selecting or applying requirements, allowing a selector to authorize its own inputs, evidence admission, or Engine authority. |
| Contracts-domain Assurance Selection | Exact resolution of one selector and policy over an Engine-owned Resolved Posture Snapshot and other declared admitted inputs; closure of the exact selection basis; application of only admitted exception effects; policy-resolution basis; Resolved Assurance Requirements; complete candidate and selection accounting. | Definition, mapping, candidate-input, policy, or exception admission/currentness; policy or exception invention; exception authorization; posture resolution or mutation; Engine authorization; capability-driven relaxation; evidence admission; or gate reinterpretation. |
| Contracts-domain direct requirement resolution | Complete direct-contract Evaluation Requirements over an exact admitted, usable contract, its permitted use, subject/scope, compatible definitions, dependencies, nomination/resolution provenance, and accounting. | Admitting its own inputs, inventing missing requirements, capability-driven relaxation, assigning the project completion standard, Engine authorization, or evidence truth. |
| Contracts-domain Producer Capability Authority | Independent admission and currentness of named/versioned Producer Semantic Capability definitions against Contracts-owned observation roles, scales, dialects, and proof requirements. | Producer self-attestation, Dock ceiling, operational availability, Engine permission, actual reach, evidence sufficiency, or host enforcement. |
| Contracts-domain Evaluation Coordination | Consumption of exact Evaluation Requirements; contract permitted-use/intent compatibility before valid Purpose formation; Observation Plan and Producer Selection closure, Evaluation Purpose, Readiness Basis and Assessment, Readiness Refusal, Attempt Disposition, Prospective Evaluation Basis, later launch permission/hold/refusal over exact owner checks and recovery accounting, occurrence commitment, and frontier, including the bounded no-collection outcome. | Engine authorization issuance, capability self-attestation, evidence admission, claim/metric/gate truth, or host launch by implication. |
| Contracts definition, evidence, and evaluation responsibilities | Contract/claim/evidence-policy/metric/gate definitions; candidate admission; canonical evidence; evidence-set and final-context closure; applicability, claim, metric, gate, and canonical-explanation truth. | Authored artifact mutation, Engine authorization, Dock operation, host enforcement, or downstream action. |
| Dock and adapter domains | Their manifests, operating-ceiling definitions, protocols, exact invocation, attempt terminals, source-value preservation, and candidate construction. | Producer Semantic Capability admission, Engine authorization, readiness, canonical evidence, or semantic gate truth. |
| Producer and enforcing host domains | Producers own exact attempt observations and Actual Observed Reach; the actual host owns exact applied/refused Host Enforcement Facts. | Canonical evidence, Engine authority, proof requirements, readiness reclassification, or invented enforcement. |
| Work Progression and other Operational Reliance domains | Their exact Action Kinds and admitted policies; independently established governing-completion relationships and required assessment coverage; action-target closure for their decisions; Operational Reliance Occurrences, Bases, atomic Determinations with eligibility/disposition facets, Reliance Refusals, currentness, and replay. | Rewriting gate truth, inheriting Engine evaluation authority, independently authorizing an action, mutating state, or creating a universal enforcement policy by default. |
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

### Identity, lifecycle, and currentness by category

Only the distinctions necessary to preserve the ontology are settled here.

| Category or subordinate form | Identity and lifecycle rule | Currentness rule |
| --- | --- | --- |
| Authored canonical project artifact | Exact kind, instance, project scope, content/revision, and lineage under candidate, validation, promotion, and authorized-mutation rules. | The Engine artifact owner determines which exact revision is current for a named use; historical revisions remain immutable and readable. |
| Canonical semantic definition or policy | Stable owner/name/version plus exact semantic-content identity; immutable after owner establishment through the applicable native lifecycle. A changed meaning is another definition identity or version. | The definition owner separately verifies establishment/admission and applicability at a named semantic time. Governing configurations additionally follow the exact active-version and cutover rules in Shared policy admission. |
| Owner-produced canonical record or result | Exact owner, record kind, bounded proposition, input closure, semantic time, and content identity. Creation is an owner decision, not artifact promotion. | The record remains historical truth. Currentness, verification, supersession, and current-use eligibility are separate relations or observations. |
| Later launch checkpoint and Coordination outcome | Exact owner checks and the governed launch's permission, hold, or refusal remain separate from the immutable formation basis. | Bound definitions declare fixed inputs, allowed operational variation, verifying owners, checkpoints/freshness, and any recovery/continuity conditions before execution. The complete lineage enters frontier provenance; no new service or artifact family is required. |
| No-collection commitment/frontier outcome | One Coordination result establishes both commitment to the exact eligible basis and sealed empty collection; replay preserves the same identities and semantic checkpoint. | Closing readiness/authorization checks belong to the final pre-commit basis; occurrence-state checks belong to the outcome. Final-context verification remains separate. No post-commit producer retry is enabled. |
| Durable origin, attempt, or source provenance | Exact origin owner, occurrence identity, bound purpose/basis/selection where applicable, semantic time, and reported content. Delivery replay and a new occurrence are distinct according to the origin owner's rules. | Later source validity or downstream eligibility is independently observed; retention does not imply continuing authority or freshness. |
| Deterministic projection or assembly | Exact source-owner identities, source revisions or records, projection/assembly definition, purpose, audience, disclosure bound, resolution/density where applicable, omissions/lossiness, and output identity. | A projection remains exact historical output for its source closure. A current view requires current source resolution and a new projection when declared inputs change. |
| Representation | Exact encoding/schema version and interface-local framing when needed; it carries rather than replaces the semantic identity. | Transport availability or schema conformance is independent of source currentness and semantic admissibility. |
| Transient operational state | Operation-local identity only as needed for coordination. | It expires with the operation unless an owner closes a separate durable provenance record. |

The proposed
[evaluation identity and evidence-closure decision](evidence-and-evaluation.md#identity-evidence)
owns exact occurrence identity, evidence-set membership and closure, prospective
applicability, and semantic-time/historical-resolvability rules. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
owns the present-use verification basis. This ontology only classifies their
objects, and neither proposal is human approval or implementation authority.

### Projection and representation rules

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

For a complete-pruning evaluation, projections MUST preserve the distinction
between collection closed without producer activity, final applicability/gate
outcome, and any later operational decision or committed effect. They MUST NOT
render an empty collection as merely "tests passed." Excluded claims and their
decisive proof remain in the canonical explanation even when a bounded projection
omits detail under its declared disclosure rules.

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

### Discriminating scenario corpus

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

### Settled versus deferred

This record settles:

- the two exact Contract Permitted Use values, Contracts-owned qualification
  under established admission rules, immutable revision changes, and distinction
  between a governing contract and a contract inspected as data;

- the six-category ontology plus the subordinate representation distinction;
- Coordination-owned no-collection commitment/frontier as an owner-produced
  lifecycle result, separate fact-input roles, and truthful empty-route projections;
- canonicality as owner- and proposition-scoped rather than repository-wide;
- the fixed Handbook admission-governance establishment rule, standalone advisory
  setup, exact scoped configuration and owner-rule bindings, prior-authority
  successor activation, active-at-decision cutover, and preserved history under
  [shared policy admission](#policy-admission), without selecting storage or IAM;
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
- assurance governance and exception admission, Assurance Selection, direct
  requirement resolution, Producer Capability Authority, and Evaluation
  Coordination as distinct
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
[evaluation identity and evidence-closure decision](evidence-and-evaluation.md#identity-evidence).
Detailed authored-artifact fields, references, requiredness, profile instances,
subject/snapshot binding, and input/result relations remain owned by the proposed
[authored-artifact and evaluation-relationship decision](objects-and-governance.md#authored-relationships).
Detailed present-use reliance, local and parent eligibility, explicit
`historical_validity_only` versus `continuing_current` Engine verification,
action authorization, attempts, terminals, and receipts remain owned by the
proposed
[operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions).
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

<a id="authored-relationships"></a>
## Authored artifacts and relationships

This chapter owns exact relationships from the six authored artifact families to definitions, inputs, and results. Authored project truth is distinct from runtime observation, executable policy, evaluation truth, and action authority.

### Purpose and scope

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

### Canonical vocabulary

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

### Relationship invariants

#### Exact references do not transfer authority

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

#### Reference direction and target-owned lineage

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

#### Authored inputs and owner-produced truth remain distinct

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

#### Development history and contracts inspected as data

Exact diagnostic findings MAY inform later contract authoring, missing-case
analysis, and historical rationale. Such references retain their exact semantic
role; they do not admit a successor, promote permitted use, establish the
governing completion standard, or import a prior occurrence's canonical evidence.
Declared prior-fact roles and their exact reuse restrictions remain distinct
from the deferred cross-occurrence evidence-import mechanism.

Evaluating **under** contract V is different from evaluating candidate D **as
the subject of** V. An already admitted validation contract V MAY inspect an
exact draft or Diagnostic-only D through a declared compatible subject/input
role. V's permitted use governs that run; D's restriction does not forbid a
decision-seeking evaluation of D as data. This does not permit decision-seeking
execution under D or establish D's admission, permitted use, or governing role.
An admission owner may consume V's bounded result only where its applicable
rules declare that exact result role, subject, provenance, and use conditions.
No concrete validation contract, parser, or preview subsystem is selected.

If validation findings are inserted into D's semantic content, the changed D
is a new candidate; the prior result cannot silently validate that changed
content. An owner admission closure may record the validation relationship
without rewriting the inspected candidate. Unadmitted exploratory execution
does not become canonical evaluation under the draft it explores.

#### Policy candidates and governance sources

Authored policy candidates MUST follow [shared policy admission](#policy-admission).
The artifact's exact revision, relationship direction, declared role, scope,
and source dependency mode remain governed by this chapter. A governance
reference supplies neither executable admission nor a runtime authority fact.

### Profile instantiation and requiredness

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

### Per-family responsibility and relationship map

| Authored Artifact Kind | Authoritatively declares | May reference or supply as an explicit input | MUST NOT establish |
| --- | --- | --- | --- |
| **Project Authority / Charter** | Constitutional project authority, Project Classification, authored posture baseline and dimension values, governance nominations, descriptive constraints, and authorized Charter change provenance. | Exact governance definitions; source material for posture resolution; exact Decision/Risk revisions; separately admitted policy or exception records; other exact authority references declared by its kind. | Resolved Posture Snapshot; Posture Scope resolution; Assurance Policy selection; Resolved Assurance Requirements; Engine evaluation authorization; evidence, metric, gate, disposition, or mutation truth. |
| **Project Context** | Authored project/domain/product description, system boundaries, ownership, authoritative source references, and known unknowns for its exact project scope. | Exact context inputs to subject/scope resolution, policy selection, plan or purpose formation, grounding, or explanation only when the consuming definition explicitly declares that role and binds the exact revision. | Runtime/system observation; current external state; producer capability or availability; authorization; readiness; evidence; applicability; claim, metric, gate, or action truth. |
| **Environment Context** | Authored descriptions of named intended or known environments, declared constraints, expected capabilities, authoritative references, and known unknowns for exact environment scopes. | Exact declared environment input to Producer Selection, host-prerequisite definition selection, plan/purpose formation, or explanation when explicitly declared and independently resolved. | Producer Semantic Capability; current availability; actual prerequisite satisfaction; Dock ceiling; Requested or Actual Observed Reach; Host Enforcement Fact; readiness; canonical evidence; or proof that an environment exists now. |
| **Work Specification** | The objective, bounded scope, non-goals, planned work, acceptance intent, dependencies, and authored work-planning state for one feature, slice, delivery unit, or other exact body of work. Authored work-planning state is distinct from both the Engine-owned Artifact Instance lifecycle and every separately owned operational workflow, closeout, eligibility, promotion, or mutation state. | Exact Contract, claim, requirements, policy, Evaluation Purpose, evaluation result, semantic gate, Operational Reliance Determination and displayed eligibility/disposition facet, Action Authorization Refusal, Action Terminal, and Mutation Receipt references in the causally valid revision appropriate to each relation. | Evaluation Purpose or requirements merely by prose; evidence; claim/metric/gate computation; local closeout; parent eligibility; action disposition; action authorization; completion authority; promotion, deployment, or any mutation outside its authored-artifact lifecycle. |
| **Decision Record** | A governed human decision, status, context, rationale, consequences, supersession, participants or authority provenance, and the exact scope of that human decision. | Governance Source References to exact definitions, policies, selectors, exceptions, approvals, artifact revisions, owner results, or action records; exact supersession references to other Decision Record revisions. | Executable policy, exception admission, owner currentness, assurance selection, evaluation truth, action authorization, or mutation merely because its prose is accepted or resembles a rule. |
| **Risk Record** | A bounded authored risk or uncertainty proposition, owner, treatment intent, review basis, risk-governance status, and acceptance or monitoring rationale for an exact scope. | Exact source observations, canonical evidence, evaluations, gates, Decision Records, typed exception records, policies, treatments, and action results as provenance-bearing references. | Canonical evidence membership; finding or risk measurement; proof of mitigation; prospective applicability; Assurance Policy relaxation; exception/waiver authority; gate reinterpretation; or operational permission. |

#### Project Authority / Charter to posture and assurance

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

#### Project Context and Environment Context to evaluation formation

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

#### Work Specification to evaluation and action results

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

#### Decision Record to definitions, policies, and exceptions

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

#### Risk Record to evidence, assurance, and exceptions

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

### Revision, currentness, and invalidation

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
| The same exact direct required-current Artifact Revision receives a fresh positive currentness verification at a later declared checkpoint | The new observation and each new closure explicitly binding it have exact identities; the Artifact Revision does not change. Before commitment, retention of Purpose/occurrence follows the bounded formation-refresh rule. After commitment, a later launch check may confirm the same exact revision only under the [post-commit verification rule](evidence-and-evaluation.md#post-commit-verification), without replacing any formation observation, pruning proof, or basis. A substituted revision or changed fixed fact follows the applicable new occurrence chain. |
| A newer source revision appears after an owner-derived snapshot, admission, fact, record, or result has formed | The derived object remains immutable historical truth for its exact source closure. A historical/provenance-only or formation-only source relation does not silently invalidate it. A continuing-current source relation follows the deriving owner's exact checkpoint rule and may make the derived object unusable at that checkpoint or require a fresh derived object. No owner silently rebinds the old object to the new source revision. If the fresh derived object replaces an input in Purpose semantic content, a deliberately new occurrence is required. |
| The owner-derived target's admission, currentness, applicability, or supersession changes while the source artifact revision remains unchanged | The source artifact and its reference remain unchanged. The target owner's exact observation governs semantic use. Any fresh target or consuming closure follows that owner's rule, and replacement of a Purpose input requires a new Purpose and occurrence. |
| A bound required-current artifact changes after Purpose formation but before launch or no-collection commitment | The earlier Purpose and occurrence retain history but cannot silently rebind. A required artifact-currentness verification failure stops the existing path; using the new revision requires every affected owner-derived closure and a deliberately new evaluation occurrence. |
| A bound required-current artifact changes after collection or no-collection commitment but before final evaluation closure | Commitment/frontier and any attempt/source provenance remain immutable. New semantic truth refuses when the declared currentness rule cannot be met; a replacement revision cannot retroactively authorize or reshape the collection or no-collection basis. |
| An artifact changes after claim/metric/gate truth closes | The historical result remains exact for its bound revision closure. Present reliance is decided later and may require a fresh evaluation; this record makes no reliance decision. |
| A later artifact revision adds a reference to an earlier result or disposition | The new revision records the relationship prospectively from its own formation time. It does not alter the result, its input revision, or the occurrence that produced it. |
| A Decision Record supersedes another Decision Record | Both exact revisions remain historical. The supersession relation affects Decision Record governance only unless another owner explicitly consumes and verifies it. |
| A Risk Record treatment or status changes | A new risk revision forms. No evidence set, assurance requirement, gate, exception, or action truth changes unless its owner forms a fresh result under a declared relationship. |

Any artifact revision, owner snapshot, definition, or result that participates
in an Evaluation Purpose's semantic content participates in its Evaluation
Content Fingerprint through its exact owner identity and revision. Substitution
therefore requires a new occurrence even when rendered text or user-facing
meaning appears equal.

### Causal and anti-cycle rules

The semantic graph is acyclic within one evaluation occurrence:

```text
pre-existing artifact revisions and owner snapshots
  -> requirements / applicability / plan / purpose
  -> authorization / readiness / prospective basis
  -> ordinary attempts and collection frontier OR no-collection commitment/frontier
  -> evidence closure / final evaluation
  -> claim / metric / gate results
  -> later action results
  -> optional later artifact revisions that reference those results
```

On the [complete-pruning route](evidence-and-evaluation.md#complete-pruning),
exact pre-existing applicability facts remain separately owned inputs in their
declared roles through final applicability. They are not imported into the empty
evidence set. A reference alone does not create an input role. The prospective
basis precedes the Coordination commitment/frontier outcome and cannot refer to
that future frontier or be mutated to contain its later occurrence-state facts.

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
  Evaluation Requirements and their exact source, Observation Plan, Formation Policy,
  evidence-set membership, Metric Definition, or Gate Policy that produced it.

Same-occurrence feedback MUST refuse at the earliest formation or congruence
boundary. A valid feedback path creates a later artifact revision or separately
owned observation/recommendation and, if used for evaluation, a deliberately
new occurrence with fresh closures. Exact historical linkage is preserved; it
is never rewritten into an earlier input.

### Semantic responsibility map

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Engine artifact/profile domain | Six Artifact Kinds; profile instantiation and requiredness; Artifact Instance and Revision identity; promotion, current-revision resolution, subject binding, lifecycle, mutation, and lineage. | Posture snapshot, executable policy admission, evaluation truth, operational disposition, or generic authority over owner-produced records. |
| Project authors and artifact governance | Exact authored content and approvals appropriate to each family. | Another owner's admission, currentness, observation, evaluation, or action result. |
| Engine posture domain | Resolution of exact Charter revision and Posture Scope into Resolved Posture Snapshot; snapshot verification and later separately authorized transition. | Charter authoring, assurance selection, or mutation inferred from an artifact/result reference. |
| Contracts assurance and definition responsibilities | Independent admission and resolution of selectors, policies, exceptions, contracts including permitted use, claims, evidence policies, metrics, and gates; complete Evaluation Requirements from posture selection or direct requirement resolution over exact admitted inputs. | Treating Project/Environment Context, Decision prose, Risk acceptance, or Work Specification criteria as executable by implication. |
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

### Discriminating scenario corpus

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Stale Charter revision at posture resolution.** A request names Charter revision `C1` as current, while Engine reports `C2` is the current revision for the exact project and Posture Scope. | Required-current posture resolution refuses or deliberately resolves `C2` and forms a new snapshot basis. It never labels `C1` current or substitutes `C2` under a `C1` identity. | Artifact history, currentness, and owner snapshot formation are distinct. |
| **Charter changes after requirements selection.** Snapshot `P1` and requirements `R1` bind Charter revision `C1`; a new current revision `C2` changes a relevant posture dimension before a run forms. | `P1/R1` remain historical. A current run uses a new resolved snapshot and requirements closure and cannot reread or merge `C2` into `R1`. | Assurance consumes exact Resolved Posture Snapshot, not mutable Charter state. |
| **Different source-currentness modes for derived objects.** Admitted policy `Q1` binds Decision Record revision `D1` as historical rationale with no continuing source-currentness requirement. Resolved Posture Snapshot `P1` binds Charter revision `C1` under a declared continuing-currentness rule. Later revisions `D2` and `C2` become current. | `Q1` is not rewritten or made non-current merely because `D2` exists; the policy owner independently determines `Q1` currentness. `P1` remains immutable historical truth, but it cannot satisfy a checkpoint that requires its bound Charter source to remain current after `C2` replaces `C1`; a fresh snapshot and affected downstream closures are required. Neither target silently rebinds to the newer source revision. | Owner-derived identity, source provenance, source-currentness dependency, and target-owner currentness are distinct. |
| **Ambient Project Context.** A planner forms a Purpose using “the current project context” but binds no exact revision, scope, consumer policy, or resolution accounting. | Purpose formation refuses. The context cannot be discovered or substituted from ambient repository state after closure. | Declared contextual relevance is not a Bound Evaluation Input. |
| **Optional artifact required by one evaluation.** Environment Context is optional for profile conformance, but a selected planning policy explicitly requires exact revision `E1`. The artifact is absent. | Profile conformance retains its own optional result; the policy or evaluation formation refuses for its missing declared input. | Profile requiredness and semantic-input requiredness are independent. |
| **Environment declaration versus host fact.** Environment Context says production supports network denial. A selected host cannot prove it applied that restriction for the attempt. | The declaration remains exact authored context. Readiness, launch, or candidate admission follows its own facts and may refuse; no Host Enforcement Fact is fabricated from the artifact. | Desired/declared environment state is not enforced runtime state. |
| **Work Specification acceptance prose.** A Work Specification says “tests must pass” but names no exact Contract claims, Metric Definition, Gate Policy, or requirements closure. | The statement remains authored acceptance intent. Contracts cannot construct claims or a gate from the prose. | A plan is not an executable evaluation definition. |
| **Complete-pruning presentation.** All expected claims are positively inapplicable under exact declared proofs; Coordination closes collection without a producer. | Preserve the exact lifecycle result and final policy-defined meaning. A projection cannot call this tests passed or committed work completion, and prior facts do not become evidence members. | Owner lifecycle truth, proof inputs, evidence membership, and presentation stay distinct. |
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

### Settled versus deferred

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
[operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
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
