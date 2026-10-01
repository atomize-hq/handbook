# HCM Artifact-System Seam — ChatGPT Pro Consultation

- Date: 2026-09-02
- Conversation: https://chatgpt.com/c/6a98c6c9-3730-83e9-b7c0-8dd22a43fa24
- Configuration: Chat surface; GPT-5.6 Sol; Pro power 5 of 5
- Task/spec: Review how the three proposed HCM semantic decisions compose with
  Handbook's authored artifacts, owner records, projections, runtime
  coordination, and crate boundaries before implementation planning.
- Current approach: Preserve the posture → assurance → authorization/readiness →
  evidence → claim/metric/gate → downstream-action chain; keep generic artifacts
  distinct from derived canonical records; keep projections non-authoritative;
  use greenfield semantics without V1 compatibility obligations.
- Source packet:
  `docs/research/hcm-artifact-system-seam-chatgpt-pro-source-packet-2026-09-02.md`
- Conversation source hashes:
  - source packet: `D30D770668FE0DB340629D93461E8BEF4A061427030367E1A4FEC51C51642B50`
  - posture decision: `AC71F03BD3C15CC73B4BAA8365EF0A8F4A286B35C55C889F1CE3B06F79A82AA4`
  - evaluation authorization/readiness decision: `815ACCDC2B7A476286F333E7626DCF5A664FF0740AC40EC137AD52E80AB69F74`
  - metric/gate decision: `457F8A3BF0A9151588669618FBB2382451F99962D0A19DA76BAD962752C3E2D2`
  - architecture context (independently confirmed by Pro):
    `A8208183AC102B27FD7211845CADF635D1931B59CDCCBEB7A98A83ECD7DAAA88`

> Advisory only; verify against local project truth and authoritative docs; do
> not reduce scope without user approval.

## Prompt (verbatim)

We are already working on this project task.

Task/spec:
Perform a fresh, independent architecture consultation on how three proposed greenfield Handbook Contract Membrane semantic decisions compose with Handbook's broader artifact and runtime system. The goal is to resolve the remaining semantic integration questions before task-level implementation planning—not to produce code, schemas, APIs, task slices, or migration work.

Read all five attached project sources completely. Use this order:
1. hcm-artifact-system-seam-chatgpt-pro-source-packet-2026-09-02.md
2. posture-assurance-policy-semantic-decision.md
3. evaluation-authorization-capability-and-readiness-semantic-decision.md
4. metric-gate-semantic-decision.md
5. architecture-context.md

Treat the source packet as the bounded repository-truth dossier. The excluded repository is unavailable and must not be inferred. Current implementation is evidence of present boundaries, not a compatibility obligation: this project is greenfield, so do not invent V1 compatibility, migration, legacy bridges, or staged preservation unless an architectural invariant genuinely requires one.

Current approach:
- The three decisions form one semantic chain: scoped posture selects assurance requirements; requirements close the complete observation/evaluation plan and authorization/readiness basis; admitted evidence closes claims, exact metrics, and a pure semantic gate; separate downstream policy decides operational action and any posture feedback.
- Handbook currently has six declared authored artifact families: Project Authority/Charter, Project Context, Environment Context, Work Specification, Decision Record, and Risk Record. Only the first three are instantiated in the shipped profile.
- We are testing a four-category model:
  1. authored canonical project artifacts;
  2. owner-derived canonical records;
  3. user-facing deterministic projections;
  4. transient operational objects.
- We suspect the new posture/evaluation/gate records should not automatically be forced into the generic authored-artifact registry, but this ownership boundary is not yet settled.
- Projections never own truth. Engine should remain inward of Contracts, owner crates should remain inward of SDK/transport/CLI/Tauri, and Flow/Pipeline should orchestrate or package rather than redefine semantics.
- Project Context and Environment Context likely provide declared inputs to planning and policy selection; Work Specification likely consumes gate/disposition outputs; Decision and Risk Records likely relate to policy/version/change/risk provenance without becoming runtime evaluation objects. These relationships need validation.
- There are vocabulary risks around old FeatureSpec versus current Work Specification, repository/profile readiness versus evaluation readiness, Context Resolution capability versus explanation density, and several distinct forms of currentness/freshness.

Evidence, constraints, and validation:
- All three decision records are PROPOSED and are planning candidates, not implementation authority.
- No implementation or compatibility commitment is authorized.
- Exact SHA-256 hashes:
  - source packet: D30D770668FE0DB340629D93461E8BEF4A061427030367E1A4FEC51C51642B50
  - posture decision: AC71F03BD3C15CC73B4BAA8365EF0A8F4A286B35C55C889F1CE3B06F79A82AA4
  - evaluation authorization/readiness decision: 815ACCDC2B7A476286F333E7626DCF5A664FF0740AC40EC137AD52E80AB69F74
  - metric/gate decision: 457F8A3BF0A9151588669618FBB2382451F99962D0A19DA76BAD962752C3E2D2
- The repository graph index was stale/degraded, so the packet relies on current source inspection and hash verification. Flag any conclusion whose confidence depends on unavailable repository evidence.

Please return:
1. A direct KEEP / ADJUST / PIVOT verdict on the combined approach, with the smallest sufficient causal explanation.
2. A recommended classification table for the important objects named across the three decisions: authored canonical artifact, owner-derived canonical record, deterministic projection, transient operational object, or another category you can justify.
3. Which objects, if any, belong in the generic Artifact Kind / Artifact Instance machinery versus dedicated owner repositories, and why. Do not equate persistence with authored-artifact status.
4. The exact semantic impact on each of the six authored artifact families, including whether each is an input, policy-bearing authority, planning consumer, result consumer, provenance record, or unaffected.
5. A crate/namespace responsibility map for Engine, Contracts, Dock, dock schema, SDK, CLI, Tauri, Flow, and Pipeline, including any missing bounded-context owner.
6. A clean currentness/freshness taxonomy: artifact lifecycle/readiness, posture snapshot currentness, authorization/currentness, readiness, evidence currentness, evaluation-result currentness, and operational-reliance currentness. Identify terms that must be renamed or kept explicitly distinct.
7. A projection taxonomy and ownership rule that covers artifact rendering, grounding projections, Flow packets, gate/explanation views, and adapter/transport DTOs without letting projections become authority.
8. The remaining semantic decisions, if any, that must be settled before full task-level implementation planning. Rank them as blocking or non-blocking and keep each boundary narrow.
9. Any contradictions, false equivalences, circular dependencies, missing identities, replay/audit gaps, UX/workflow hazards, or future dock/adapter/skill-family constraints that the packet has not surfaced.
10. A concise recommended next sequence for us to review together. Do not decompose implementation.

Be especially skeptical of:
- making every semantic record an artifact;
- allowing authored artifacts to absorb runtime truth;
- allowing projections, SDK types, or orchestration packets to become authorities;
- confusing policy selection with metric/gate meaning;
- confusing evaluation readiness with artifact/profile readiness;
- confusing Context Resolution capability with explanation density;
- circular identity between prospective bases, evidence sets, and final evaluation contexts.

If a recommendation conflicts with the supplied repository truth, name the conflict and ask for the smallest additional source needed rather than guessing.

Do not reduce the requested scope, change the product direction, or convert this into implementation planning without explicit approval. Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Response (verbatim)

Source integrity

I read all five files in the requested order and independently recomputed their SHA-256 digests. The four supplied decision/packet hashes match exactly. The architecture context also matches the packet-listed digest A8208183AC102B27FD7211845CADF635D1931B59CDCCBEB7A98A83ECD7DAAA88.

I have treated the packet as bounded repository truth, the three decisions as proposed semantic authorities, and current implementation only as boundary evidence. That matches the packet’s explicit greenfield and no-compatibility premise. 

hcm-artifact-system-seam-chatgp… +1

1. Verdict
ADJUST

Keep the substantive semantic chain and dependency direction. They are strong:

Posture selects exact proof obligations without weakening trust or changing metric/gate meaning.

Evaluation Coordination compares those obligations with Engine authorization and independently admitted producer facts.

Engine alone owns Context Resolution admission and Evaluation Authorization.

Contracts closes admitted evidence into claim, metric, and semantic-gate truth.

Operational disposition and mutation remain downstream and separately authorized.

Evidence binds the Prospective Evaluation Basis and does not depend circularly on the later final evaluation context. 

evaluation-authorization-capabi…

 

metric-gate-semantic-decision

The smallest sufficient adjustment is architectural, not directional:

Expand the proposed four-category object model. It lacks a category for immutable semantic definitions and policies, and it wrongly leaves durable source/attempt provenance inside “transient operations.”

Keep the generic Artifact Kind / Instance machinery restricted to authored project truth rather than making it a universal canonical-object repository.

Settle a few exact identity and closure seams before implementation planning: deliberate new-evaluation identity, Producer Selection versus currentness granularity, canonical evidence-set closure, planning-time applicability, and action-specific operational reliance.

Harmonize the three proposed records so that the evaluation record’s exact plan-first order and Contracts-domain owner placement are not contradicted by stale “deferred” wording in the older records.

That is an ADJUST, not a pivot.

2. Recommended object classification

The decisions distinguish immutable named definitions, owner-issued results, source observations, untrusted candidates, and projections too sharply for one generic “record” category. The most stable model has six classes.

Important object	Recommended class	Semantic treatment
Charter / Project Authority; Project Context; Environment Context; Work Specification; Decision Record; Risk Record	Authored canonical project artifact	Human- or governance-authored project truth with project lifecycle, revision, promotion, mutation, and lineage semantics.
Artifact Kind definitions, Artifact Instance Descriptors, profiles, renderer definitions, projection definitions	Canonical system definition	Engine-owned definitions configuring authored artifacts and their available views; not themselves runtime evaluation results.
Contract revisions, expected-claim definitions and inventories, claim-set definitions	Canonical semantic definition	Contracts-owned immutable definitions resolved exactly by identity and version.
Assurance Policy Selector, Assurance Policy, Evidence Evaluation Policies, Metric Definitions, Gate Policies, Enforcement Policies, posture-observation and recommendation policies	Canonical semantic definition or policy authority	Named, versioned rules. They may be authored somewhere, but their executable identity belongs to their semantic owner’s definition registry, not to the project-artifact repository.
Producer Semantic Capability definition; Dock/adapter operating-ceiling definition	Canonical owner definition or attestation	Independently admitted capability or operating-bound semantics. Their present currentness or availability is a separate owner observation.
Typed Exception Policy	Canonical semantic definition	Defines permitted issuers, scope, effects, record requirements, and currentness.
Human-authored exception request or record source	Potential authored governance artifact	It may use an explicit artifact kind if Handbook chooses an authored lifecycle. Artifact status alone never makes it an admitted exception.
Admitted exception input used by assurance selection	Owner-produced canonical admission record	A separate exact result proving authorization, policy conformance, scope, identity, and currentness.
Private ContextResolutionEnvelope, Resolved Profile, exact stack and authority-admission lineage	Engine canonical authority record	Private Engine truth. Consumers receive bounded owner projections or opaque references, not reconstructed copies.
Posture Scope and Resolved Posture Snapshot	Engine-produced canonical record	Immutable scoped derivation from exact Charter authority and other admitted resolution inputs.
Policy-resolution basis and Resolved Assurance Requirements	Contracts-produced canonical record	Exact selection outcome binding the posture snapshot, selector, selected policies, definitions, exceptions, accounting, and explanation.
Observation Plan and Producer Selections	Contracts Evaluation Coordination canonical record	Exact pre-authorization closure of roles, selections, multiplicity/composition, requested reach, and stable capability/ceiling references.
Evaluation Purpose and Evaluation Use Intent	Contracts Evaluation Coordination canonical record	Purpose is formed over the exact Observation Plan. Use intent is an exact bound value within that purpose.
Evaluation Authorization Request	Untrusted operational request	A request to Engine. It is not canonical authorization and may be retained for audit without gaining authority.
Evaluation Authorization Binding, authorization refusal, Authorization Verification Observation, supersession relation	Engine-produced canonical record	Engine issuance and verification truth. Currentness remains a separate observation rather than a mutable field on the binding.
Readiness Basis, Readiness Assessment, Readiness Refusal, Attempt Disposition, Prospective Evaluation Basis	Contracts Evaluation Coordination canonical record	Immutable, content-addressed coordination results. None is evidence or gate truth.
Producer Attempt, source observation, Actual Observed Reach, Dock terminal	Bounded source/operational provenance record	Durable occurrence history. It is authoritative only for the origin owner’s bounded assertion and still requires downstream admission.
Host Enforcement Fact	Bounded host provenance record	Authoritative for what the host actually applied or refused, not for Engine authorization, evidence adequacy, or gate truth.
Evidence candidate	Untrusted boundary submission	Evidence-shaped input retaining source and Prospective Evaluation Basis provenance. It is not canonical evidence.
Canonical evidence record	Contracts-produced canonical record	Immutable admitted evidence after authority, identity, provenance, subject, unit/scale, normalization, validity, and duplicate checks.
Canonical evidence-set closure and final evaluation context	Contracts-produced canonical closure record	Exact membership and final evaluation provenance boundary. It must exist before claim, metric, or gate truth.
Applicability result, claim evaluation, metric result, semantic gate decision	Contracts-produced canonical result	Immutable truth for one exact final context.
Canonical explanation	Contracts-produced canonical decision data	Part of the semantic result, not merely prose and not a projection.
Enforcement Disposition or other downstream disposition	Downstream-owner canonical result	Exact application of one domain-specific reliance/enforcement policy to preserved gate truth and its required currentness basis.
Operational Reliance Basis, action decision, mutation receipt	Downstream-owner canonical record	Present-tense action eligibility and any separately authorized effect. The action itself is not gate truth.
Posture Observation and Posture Recommendation	Owner-produced canonical record	Non-mutating observation and proposal.
Posture Transition receipt and resulting authority state	Engine mutation result and authority state	Transition is a separately authorized Engine mutation; the receipt is its immutable history.
Artifact Markdown rendering; owner public record projection; grounding projection; explanation-density view	Deterministic projection	A bounded view of exact source truth. It may own its projection identity and omission accounting, never the source semantics.
Flow packet	Deterministic assembly product	Combines exact artifact renders, grounding projections, references, omissions, and budget decisions. It is not a new authority over the included content.
SDK DTO; CLI JSON or text; Tauri IPC payload	Representation of an owner projection	Transport or interface representation. Correlation and framing are adapter-local; semantic meaning stays with the owner.
Queued launch, process handle, in-progress scheduler state, transport-local error	Transient operational object	Live execution state, not durable domain truth merely because it may be logged.

The key correction is that canonical semantic definitions are not owner-derived records, while source observations and attempts are often durable and therefore are not transient. The records themselves already distinguish policies, source observations, evidence candidates, admitted evidence, owner results, and operational state. 

posture-assurance-policy-semant…

 

evaluation-authorization-capabi…

 

metric-gate-semantic-decision

Project Classification, Posture Levels, Posture Dimensions, requested reach, and use intent are generally bound values inside the relevant artifact or canonical record. They do not each need independent artifact or repository status.

3. Generic Artifact Kind / Instance machinery versus owner repositories
Objects that belong in the generic artifact system

The generic machinery should continue to own:

The six declared Artifact Kind definitions and Artifact Instance Descriptors.

Canonical instances of Charter, Project Context, Environment Context, Work Specification, Decision Record, and Risk Record.

Their authored lifecycle: intake, candidate formation, validation, promotion, project-authorized mutation, exact current revision, and lineage.

References from those artifacts to exact semantic definitions or owner records.

That matches the machinery’s current emphasis on canonical paths, requiredness, dependencies, lifecycle and intake definitions, renderers, projections, validation, candidate promotion, mutation outcomes, and lineage. 

hcm-artifact-system-seam-chatgp…

Objects that should use dedicated owner repositories
Owner repository or registry	Objects
Engine definition repositories	Profiles, artifact definitions, posture-resolution definitions, Context Resolution definitions, projection/grounding definitions.
Engine authority/result repositories	Resolved Posture Snapshots, Evaluation Authorization Bindings, verification observations, supersession relations, posture transitions and receipts.
Contracts definition registries	Contract and claim definitions, Assurance Policy Selector and Assurance Policies, Evidence Evaluation Policies, Metric Definitions, Gate Policies, and any Contracts-owned exception or capability definitions.
Contracts evaluation repositories	Resolved Assurance Requirements, Observation Plans, Purposes, Readiness records, Prospective Evaluation Bases, canonical evidence, evidence-set closures, final contexts, claims, metrics, gates, and canonical explanations.
Dock/producer/host provenance repositories	Manifests, attempts, source observations, actual reach, operating terminals, candidates, and Host Enforcement Facts.
Downstream-domain repositories	Enforcement or reliance dispositions, workflow eligibility, parent-readiness results, and mutation receipts.
Projection caches or indexes	Renderings, Flow packets, explanation views, transport DTOs, and cross-owner audit views. These remain non-authoritative and reproducible from owner records.
Do not add a generic “derived artifact” mode

A first-class derived-record category inside ArtifactInstanceDescriptor would collapse incompatible lifecycles:

Authored artifacts can be candidates, promoted, edited, and project-mutated.

Definitions are immutable named/versioned authorities.

Evaluation records are immutable owner outcomes.

Authorization and currentness use issuance, verification, and supersession.

Source records are occurrence provenance subject to later admission.

Projections are purpose- and audience-bound views.

Sharing low-level fingerprint, canonicalization, or lineage primitives is reasonable. Sharing the same semantic repository and lifecycle is not.

For unified browsing, use a non-authoritative cross-owner catalog or lineage projection. It can list heterogeneous resources by owner, kind, exact identity, and relationship without making them Artifact Instances.

A policy or exception may have a two-stage life:

An authored project artifact is its human-editable source.

Its semantic owner independently admits an immutable executable definition or admitted exception record.

Promotion in the artifact system must never substitute for the second step.

4. Semantic impact on the six authored artifact families

The current registry declares all six families, while the shipped profile instantiates only Charter, Project Context, and optional Environment Context. Decision and Risk kinds currently have no declared stable roles. 

hcm-artifact-system-seam-chatgp…

Authored artifact	Role in the combined model	Exact permitted impact	Must not become
Project Authority / Charter	Input; policy-bearing governance authority; provenance authority	Supplies exact persisted posture baseline and dimension values, Project Classification, governance nominations, approval requirements, and descriptive project constraints. One exact revision is bound into the Resolved Posture Snapshot. It may nominate policy or exception authorities and processes.	An executable Assurance Policy, scope resolver, Evaluation Authorization, admitted exception, evidence record, readiness result, gate, or automatic posture mutation. Free-form stance, shortcut, override, and red-line prose remains non-executable.
Project Context	Declared input; planning input	May provide exact project/domain/component facts used by Posture Scope resolution, assurance selection, subject binding, Evaluation Purpose, or Observation Plan only when the consuming policy explicitly declares that input and binds its exact identity, revision, and currentness basis.	Ambient policy selection, a substitute for Engine scope resolution, producer capability, authorization, evidence, or current host truth.
Environment Context	Declared input; planning input	May state target environments, expected constraints, environment classifications, or declared prerequisites used by scope, planning, or policy selection. It can inform a request to Engine or a Producer Selection.	Proof that an actual producer is capable, a Dock ceiling is current, a host applied restrictions, an environment presently exists, or evidence is current. Those require owner observations at the relevant checkpoint.
Work Specification	Subject/scope input; planning consumer; result and disposition consumer	May identify the exact work subject or snapshot, declare evaluation objectives and acceptance intent, reference selected requirements, and later reference exact semantic gate and downstream disposition records. A work-domain or Pipeline policy may use those records for local closeout or parent eligibility.	A container that copies or recomputes claim/metric/gate truth; a target directly mutated by Contracts or the semantic gate; authority for parent promotion merely because it references a pass.
Decision Record	Provenance record; approval narrative; optional declared input	May record why a policy family/version was adopted, why an exception or reliance policy was approved, or why a posture transition was authorized. It should reference exact semantic identities and approval evidence.	The executable policy, exception authority, Evaluation Authorization, readiness result, or action authorization simply because its prose says so.
Risk Record	Risk provenance; optional declared input; result consumer	May capture project risk judgments that inform later Charter changes or, under an explicit typed selector contract, a future assurance-selection input. It may reference gate failures, unresolved findings, or downstream actions through a separately authorized update.	An ambient severity score, evidence record, claim finding, gate predicate, or automatic posture input. A same-run gate result cannot update risk and then reselect its own assurance requirements.
Validation of the suspected relationships

Project Context and Environment Context: yes, they are suitable declared inputs, but never ambient selectors or authorization facts.

Work Specification: yes, it is the natural authored subject/planning and result-consumer seam, but the actual gate-to-work mapping belongs to downstream policy and a separately authorized work mutation.

Decision Record and Risk Record: they remain principally authored provenance artifacts. The three decisions do not require them to become runtime evaluation objects or universal required inputs.

The core Decision Record and Risk Record artifact semantics can remain unaffected. Typed relationships may be added later, but the packet does not establish current schema fields that should be assumed.

5. Crate and namespace responsibility map

The stable dependency principle is owner crates inward of SDK/transports, with Engine inward of Contracts. Public projection does not transfer owner authority. 

architecture-context

Crate/domain	Recommended semantic responsibilities	Must not own
Engine	Artifact/profile definitions and authored artifact lifecycle; canonical project-artifact reads; exact Posture Scope resolution; Resolved Posture Snapshot; snapshot-currentness verification; Context Resolution envelope admission; Evaluation Authorization issuance, verification, historical validity, and supersession; posture transitions; Engine-owned public projections and grounding/disclosure semantics.	Assurance selection, contract requirements, Producer Semantic Capability, readiness, evidence admission, claim/metric/gate truth, explanation-density choice, or downstream disposition.
Contracts	Contract/claim definitions; recommended Assurance Selection namespace owning selectors, Assurance Policies, Resolved Assurance Requirements, and bounded exception application; settled Evaluation Coordination namespace owning plan, purpose, readiness, disposition, and Prospective Evaluation Basis; evidence admission; evidence-set closure; final evaluation-context closure; claim evaluation; metric-definition and evidence-policy authorities; metric computation; Gate Policy and semantic gate; canonical explanation.	Engine authorization issuance or reconstruction, artifact mutation, host enforcement, Dock ceilings, Pipeline/workflow mutation, or posture transition.
Dock	Exact manifest and protocol; adapter configuration; admitted operating ceiling; invocation framing; exact consumption of one Producer Selection and Prospective Evaluation Basis; per-selection attempt terminal; preservation of source values, units, scales, requested and actual reach; candidate construction and operational diagnostics.	Assurance or producer selection, Engine authority, universal Producer Semantic Capability, readiness, canonical evidence, claim meaning, metrics, gates, or host enforcement claims.
Dock JSON Schema	Structural schema generation and validation for Dock manifests and messages; exact schema asset identity where applicable.	Semantic admission, capability authority, Engine authorization, evidence trust, or proof that runtime constraints were enforced.
SDK	Transport-free composition of owner operations; owner-defined public projections; correlation and bounded convenience sequencing.	New semantic branches, alternate currentness checks, readiness recomputation, owner-result reinterpretation, or authoritative DTO meaning independent of the owner.
CLI	Input acquisition, lexical validation, rendering, streams, exit behavior, correlation, and advisory refusal within its own call boundary.	Task-wide enforcement, proof of host restrictions, semantic readiness, gate reinterpretation, or mutation authority.
Tauri	IPC framing, host context, scheduling/off-main-thread concerns, transport-local errors, and exact mapping of owner results.	Semantic ownership. It may claim Host Enforcement Facts only if Tauri is separately selected and authorized as the actual enforcing host for a named restriction—not merely because it launched a command.
Flow	Deterministic packet assembly from exact artifact renders, grounding projections, public owner views, references, omissions, and budget decisions. It owns packet-assembly accounting.	Charter, posture, assurance, authorization, evidence, metric, gate, or downstream policy truth.
Pipeline	Pipeline capture, compile, handoff, lifecycle, and separately authorized Pipeline mutations. It may own a Pipeline-specific operational-reliance policy and disposition if explicitly selected.	Claim/metric/gate semantics, generic parent readiness, posture policy, or mutation solely inferred from a semantic gate.

Evaluation Coordination’s placement in the broader Contracts domain is already settled by the authorization/readiness record. 

evaluation-authorization-capabi…

 Engine remains the sole issuer and verifier of the authorization binding. 

evaluation-authorization-capabi…

Missing bounded-context owners
1. Assurance Selection — blocking

Recommend a distinct Contracts-domain Assurance Selection responsibility rather than Engine or a new outward crate.

Causal reason: it consumes Engine’s Resolved Posture Snapshot but must resolve and bind Contracts-owned contract revisions, claims, evidence policies, metric definitions, and Gate Policies. Placing it in Engine would reverse the dependency. Placing it in a new component that both consumes Contracts definitions and must be consumed back by Contracts risks a cycle or duplicate definition authority.

2. Producer Capability Authority — blocking

A named independent owner is still needed for Producer Semantic Capability. It must be separate from the producer and Dock’s operating-ceiling authority.

The default recommendation is a sibling authority within the broader Contracts domain because capability compatibility is defined against Contracts-owned observation roles, scales, dialects, and proof requirements. Physical crate placement should remain deferred until other consumers are known.

3. Exception policy and admission — blocking when exceptions are in scope

The clean default is:

Charter/Decision governance nominates and approves.

A Contracts assurance-governance authority owns typed exception-policy semantics.

A dedicated admission responsibility proves each exact exception record.

Assurance Selection applies only the admitted bounded effect.

4. Operational Reliance / disposition — blocking for end-to-end workflow planning

There should not be one universal central enforcement owner. Each action domain owns its mapping:

Pipeline owns Pipeline action policy.

A future dependency owner owns parent eligibility.

Engine owns posture transition authority.

A release/deployment owner owns release or deployment disposition.

A shared vocabulary and exact disposition contract are useful; a shared semantic decision engine is not automatically warranted.

5. Posture observation and recommendation — conditionally blocking

These remain separate from Engine posture mutation. Their owner can remain deferred if initial planning explicitly ends at gate/disposition and excludes posture feedback.

6. Currentness and freshness taxonomy

Currentness is an owner observation at a named boundary, not a mutable property permanently attached to an immutable object. The evaluation record correctly establishes launch, final-closure, and operational-reliance checkpoints. 

evaluation-authorization-capabi…

Recommended term	Owner and meaning	Explicit non-equivalences
Profile Conformance Assessment	Engine artifact/profile owner assesses whether the selected profile, required instances, definitions, schemas, and lifecycle states form a valid project-artifact basis.	Not Evaluation Readiness, authorization, evidence sufficiency, or current gate eligibility.
Artifact Revision Currentness	Engine determines whether an exact artifact revision is the applicable current revision for a named use and semantic time. A valid historical revision can remain readable without being current.	Not artifact validity, promotion state, or evidence freshness.
Definition/Policy Resolution Currentness	Each definition owner verifies that the exact selector, policy, contract, claim, metric, gate, capability, or ceiling version is admitted and applicable at the relevant selection or evaluation time.	Not artifact lifecycle and not a reason to rewrite historical results when a new version appears.
Posture Snapshot Currentness Verification	Engine verifies whether the snapshot’s exact Charter revision, Posture Scope, resolution inputs, and authority basis remain current for a named checkpoint. The snapshot itself stays immutable.	Not Posture Level, readiness, or authorization.
Authorization Verification Observation	Engine’s exact checkpoint result: current-use current, superseded, not_current, or unverifiable; historical historically_valid, historically_invalid, or unverifiable.	Issuance is not currentness. Historical validity cannot authorize a new collection or action.
Evaluation Readiness Assessment	Contracts Evaluation Coordination compares the complete plan with current authorization, capability, ceiling, prerequisites, and composition facts. Result is ready, known_limited, or unknown; invalid formation produces Readiness Refusal.	Not artifact/profile readiness, evidence sufficiency, gate state, permission, or predicted pass.
Admission-Time Source Validity	Contracts records whether a candidate’s source and provenance were valid when the canonical evidence record was admitted.	Does not establish durable claim currentness.
Evidence Evaluation Currentness	Claim evaluation applies the exact freshness/currentness policy at the explicit evaluation time, yielding current, stale, or unknown for the relevant evidence relationship.	Not the evidence record’s immutable admission validity and not a universal evidence-set age.
Historical Evaluation Result	A closed claim, metric, gate, and canonical explanation remain exact historical truth for their final context.	The result should not acquire a mutable current/stale field that rewrites its original meaning.
Current-Use Eligibility	A derived view of whether a historical result remains suitable for a named present use under an exact policy.	Not the semantic gate decision itself and not action authorization.
Operational Reliance Verification	The downstream action-policy owner closes the exact result, action, subject, use intent, and all current owner verifications required for that effect.	Not Evaluation Readiness, not Engine evaluation authorization, and not mutation authority.

The metric record already distinguishes admission-time source validity from evaluation-time evidence currentness. 

metric-gate-semantic-decision

Required vocabulary corrections

Rename existing SDK profile_readiness to Profile Conformance Assessment or similarly artifact-specific language. The packet confirms that its present meaning is repository/profile artifact readiness, not evaluation-attempt readiness. 

hcm-artifact-system-seam-chatgp…

Reserve unqualified Readiness for Evaluation Readiness Assessment only.

Reserve freshness for a named age/time policy, most notably evidence-evaluation freshness. Use currentness for an owner’s present relationship to authority, head, scope, or applicability.

Do not use evaluation-result currentness as an intrinsic property. Use Historical Evaluation Result plus Current-Use Eligibility or Operational Reliance Verification.

Avoid Context Resolution capability. Engine Context Resolution is authorization and bounded operating reach; Producer Semantic Capability is representational ability.

Ban unqualified resolution level in cross-domain prose. Use the exact term: Posture Level, Context Resolution active level/stack, Requested Observation Reach, Actual Observed Reach, Grounding Projection Resolution, Viewer Disclosure, or Explanation Density.

Operational-reliance issue requiring explicit ratification

The proposed evaluation record currently makes fresh Engine authorization verification universally mandatory before operational reliance. 

evaluation-authorization-capabi…

That is internally coherent but stronger than a mere currentness taxonomy. Before approval, explicitly decide between:

Universal rule: every positive operational reliance requires the original evaluation authorization basis still to be current; or

Policy-declared rule: the exact reliance policy decides whether present Engine evaluation-authorization currentness is a dependency, while separately verifying subject, evidence, policy, and action authority.

Whichever is chosen, Engine’s evaluation binding must never be described as authorization for the downstream mutation. If an action needs authorization, its own domain issues that authority.

7. Projection taxonomy and ownership rule

The architecture already establishes that a projection bounds owner truth without transferring ownership, while adapters own only interface-specific acquisition, framing, rendering, correlation, and transport failures. 

architecture-context

Projection or representation	Projection owner	What remains authoritative
Artifact rendering	Engine artifact/renderer owner	Exact artifact instance and revision. Markdown or another rendering owns only deterministic presentation and omission/lossiness accounting.
Owner public record projection	The semantic owner: Engine, Contracts, Dock, host, or downstream domain	Private canonical record. For Evaluation Authorization, Engine exposes stable evaluation-relevant facts plus opaque resolvable lineage without exposing the private envelope or credentials.
Grounding projection	Engine grounding/projection owner	Exact admitted sources and their owners. Grounding resolution and disclosure may select or summarize but do not change source authority.
Canonical explanation	Contracts result owner	It is canonical semantic decision data, not a UI projection.
Explanation-density view	Owner-approved projection definition, rendered by SDK/UI	Canonical explanation. Density may condense but cannot omit decisive status, change truth, or exceed disclosure authority.
Flow packet	Flow	Flow owns packet identity, selection, omission, ordering, and budget accounting. Each included artifact or result remains attributable to its source owner and exact projection.
SDK DTO	Owner-defined outward projection composed by SDK	Owner result and field meaning. SDK may provide stable transport-free representation but cannot invent semantic branches.
CLI/Tauri payload	Adapter	Owner projection. Adapter-added correlation and framing are not semantic instance identity.
Audit or lineage view	A deterministic cross-owner assembler	The contributing Engine, Contracts, Dock, host, artifact, and downstream records. The audit view is an index/assembly, not a new root authority.

The source packet’s tentative classification places “canonical explanations at selected density” under projections, but the metric record says the explanation itself is canonical decision data. The correction is:

Canonical explanation = owner record. Explanation at selected density = projection.

The canonical explanation must retain the complete claim/evidence/metric/gate accounting and decisive chain. 

metric-gate-semantic-decision

Universal projection ownership rule

Every semantically meaningful projection should bind:

exact source owner and source identity;

exact source revision, snapshot, or semantic time;

projection definition identity and version;

purpose and audience;

disclosure authority;

grounding resolution or explanation density, when applicable;

explicit omissions, lossiness, compaction, and budget decisions;

output identity or fingerprint.

A projection owner may be authoritative about what it selected, omitted, rendered, or assembled. It is never authoritative about the underlying Charter, authorization, evidence, claim, metric, or gate truth.

A projection must not be fed back as owner truth. A consumer either:

resolves and verifies the referenced source record with its owner; or

treats the projection as a source observation/evidence candidate and submits it through a separately declared admission policy.

Flow packets and CLI output must never become shortcut evaluation inputs.

8. Remaining semantic decisions before full task-level planning
Blocking
Rank	Narrow decision to settle	Why it blocks
B1	Joint-record normative harmonization	The evaluation record establishes Observation Plan → Purpose → authorization and settles Contracts-domain Evaluation Coordination, while the other two still use older shorthand and describe ownership as deferred. Approved records must have one exact stage order and owner map.
B2	Approve the six-class object ontology and repository boundary	Planning cannot safely assign storage, operations, discovery, or lifecycle until project artifacts, definitions, owner records, origin provenance, projections, and transient state are distinct.
B3	Assurance Selection, exception, and Producer Capability owner placement	These determine dependency direction and whether Engine, Contracts, Dock, or a new component would acquire inappropriate authority. Recommend Contracts assurance and coordination siblings, with a distinct capability authority.
B4	Deliberate new-evaluation identity and idempotency	The record says identical complete formation inputs yield the same identity, yet a deliberately new evaluation with apparently equal inputs requires a new Purpose and binding. A separate occurrence/request identity versus semantic-content fingerprint is required.
B5	Producer Selection versus currentness granularity	Producer Selection currently binds capability and operating-ceiling bases, while Readiness also binds their current facts. Settle which stable definition references belong in the Selection and which checkpoint observations belong only in Readiness.
B6	Canonical evidence-set formation and closure	Final context depends on an exact evidence-set identity, but the set’s membership, closure trigger, completeness accounting, and relationship to attempts, partial observations, duplicates, and retries are not yet semantically defined.
B7	Planning-time applicability	Readiness may consume “applicability information available before observation,” while final applicability is Contracts semantic truth. Define what pre-observation applicability fact is authoritative, whether it may remove plan roles, and how final closure confirms it.
B8	Work Specification subject and downstream-reliance seam	Settle exact Work Specification identity/snapshot binding, local closeout, parent eligibility, action-specific disposition ownership, and separate mutation authority.
B9	FeatureSpec versus Work Specification	New identities and scope bindings cannot use two overlapping names. Greenfield semantics should select Work Specification as canonical unless unique FeatureSpec semantics are separately proven.
B10	Semantic-time and historical resolvability invariant	Formation, authorization, readiness, launch, observation, admission, evaluation, and reliance occur at distinct semantic times. Owner records and exact historical definitions must remain resolvable enough for promised replay and audit, without yet choosing storage mechanics.
B11	Operational-reliance authorization rule	Ratify whether fresh Engine evaluation-authorization verification is universally required or declared per reliance policy, while keeping downstream action authorization separate.

The repeated-evaluation identity problem follows directly from the combination of content-addressed idempotency and the requirement for a deliberately new evaluation with equal-looking inputs. 

evaluation-authorization-capabi… +1

Non-blocking, provided they remain outside the first implementation-planning scope
Rank	Decision	Deferral condition
N1	Concrete explanation-density levels and UI wording	Canonical explanation and decisive status remain complete and owner-controlled.
N2	Exact typed relationships for Decision and Risk Records	No runtime path treats their prose as executable policy or authority.
N3	Posture Observation, Recommendation, acknowledgement, and hysteresis algorithms	Initial planning stops before automated posture feedback.
N4	Cross-context gate composition	Each evaluation remains one exact contract-rooted context.
N5	Metric catalogs, thresholds, assurance mappings, and actual producer selection policies	The semantic definition and owner boundaries are approved first.
N6	Physical persistence, APIs, schemas, transports, cache implementation, and crate extraction	These are task-level choices after the semantic blockers are settled.
N7	External host or Substrate adoption	The first plan is advisory only. It becomes blocking before claiming real host enforcement.
9. Contradictions, false equivalences, identity gaps, and hazards
A. Cross-record authority drift

The evaluation record settles Evaluation Coordination in Contracts and gives the exact plan-first order. 

evaluation-authorization-capabi…

The posture record still says evaluation coordination/readiness owner placement is deferred and points to the later decision. 

posture-assurance-policy-semant…

 The metric record similarly describes prospective evaluation placement as deferred and delegates it to the later record. 

metric-gate-semantic-decision

This is not a conceptual contradiction once the later record is treated as resolving the deferral, but it is unacceptable ambiguity in a jointly approved authority set. Amend the older records to say that the companion evaluation decision owns and has settled the exact seam.

Their chain diagrams should also be normalized to:

Resolved Assurance Requirements
  -> Observation Plan
  -> Producer Selections
  -> Evaluation Purpose
  -> Evaluation Authorization Request
  -> Engine Evaluation Authorization Binding
  -> Readiness Basis / Assessment
  -> Attempt Disposition
  -> Prospective Evaluation Basis
B. Canonical explanation versus projection

The packet’s tentative category table risks treating the explanation itself as a projection. The metric decision makes the full explanation canonical decision data. Only a bounded explanation-density rendering is a projection.

Without this distinction, a summary renderer could accidentally become the only retained decision basis or could omit decisive evidence accounting.

C. Deliberately new evaluation versus content identity

A content-addressed Purpose or Basis cannot distinguish:

exact replay of the same formation request; and

a deliberately new evaluation occurrence with otherwise identical semantic content.

The records require the distinction but do not name the occurrence identity that establishes it. The solution is not a random transport correlation ID. It is a semantic evaluation-occurrence or request identity, separately bound from the content fingerprint.

D. Producer Selection may overbind mutable capability facts

A Producer Selection binds admitted capability and ceiling “bases,” while Readiness binds current capability, ceiling, prerequisite, currentness, and semantic time. 

evaluation-authorization-capabi… +1

Without a clearer split:

every capability-currentness refresh might appear to change the Selection and force a new Purpose and Engine binding; or

the Selection might retain a stale capability observation and let Readiness appear current.

Recommended split:

Producer Selection: stable producer/configuration identity, role assignment, requested reach, and exact capability/ceiling definition references.

Readiness Basis: current owner verification observations for those definitions, actual availability/prerequisite facts, and semantic time.

A semantic change to capability or ceiling requires a new Selection. A new currentness observation over the same semantic definition requires a new Readiness Basis, not necessarily a new Observation Plan.

E. Evidence-set closure is not sufficiently defined

The records correctly prevent canonical evidence from depending on the final evaluation context. That avoids the main prospective-basis/evidence/final-context cycle. 

metric-gate-semantic-decision

The remaining gap is how admitted records become the exact evidence set for a context. Contracts needs a named closure responsibility that defines:

eligible source repository boundary;

exact included membership;

handling of partial and terminal attempts;

duplicate and retry accounting;

whether every admitted record under the Prospective Evaluation Basis is included or a separate closure policy selects a subset;

completeness against every Producer Selection and plan relation;

ordered versus set semantics;

closure semantic time;

refusal when membership is ambient, ambiguous, or still open.

Claim evaluators may select eligible evidence inside the already closed evidence set. They must not define the set itself, or evidence-set identity becomes circular with claim truth and permits cherry-picking.

F. Planning-time applicability is underspecified

Readiness includes “applicability information available before observation,” while the final Applicability Result is a Contracts semantic decision.

Potential failures include:

unauthoritative Project Context prose pruning an observation role;

a planning guess becoming final not_applicable;

the plan omitting a role that final applicability later declares applicable;

final applicability depending on evidence that was never collected because readiness assumed it away.

A narrow decision should define an exact prospective applicability basis or state that no role may be omitted unless a Contracts-owned applicability determination over pre-existing canonical facts positively proves the role unnecessary. Final context must either rebind the same basis or refuse a mismatch.

G. Operational reliance may overcouple observation authority and action currency

Engine’s Evaluation Authorization proves that a particular observation plan was admitted. It does not authorize deployment, promotion, mutation, or release.

Requiring its currentness for every operational use may be an intentional conservative reuse policy, but it must not imply:

current Engine authorization grants the action;

non-current observation authorization automatically negates historical truth; or

a downstream domain can omit its own action authority.

An action-specific Operational Reliance Basis is needed either way.

H. Authored context can be mistaken for current runtime fact

Environment Context is especially hazardous. An authored statement such as “network disabled” or “runs in a sandbox” is a declared project input, not proof that a particular attempt had a network restriction. Only an exact Host Enforcement Fact can establish what the host applied.

The same applies to producer capability and Dock ceilings: manifests and protocol fields cannot self-admit capability or prove enforcement.

I. Work Specification can absorb runtime truth

A Work Specification should reference exact evaluation and disposition records, not copy their statuses into mutable prose and become a second evaluation repository.

A permitted project update might record that a specific disposition was accepted, but that is a new project-authorized mutation with exact provenance—not a gate side effect.

J. Flow packet and transport authority leakage

Flow currently consumes artifacts and projections and owns packet omissions and budget decisions, while Pipeline may later consume dispositions; neither owns the underlying semantics. 

hcm-artifact-system-seam-chatgp…

Hazards include:

including a private Engine envelope rather than its public projection;

treating a packet summary as canonical explanation;

allowing a CLI exit code to replace an exact gate decision;

accepting an SDK or Tauri DTO back as source authority;

losing the owner identity during transport mapping.

The packet or DTO must retain exact owner identities and projection provenance.

K. Future Dock, adapter, and skill-family constraints

Every future producer family must preserve these boundaries:

Producer Semantic Capability independently admitted; not self-attested by manifest.

Requested reach, Engine authorization, capability, operating ceiling, actual reach, and host enforcement remain distinct.

Attempts, retries, observations, and limitations bind one exact Producer Selection.

No implicit merge or substitution across producer selections or retries.

Source-native units, scales, direction, exact decimal or rational values, and lexemes remain preserved.

No floating-point coercion, silent rounding, inferred units, or scale-direction guesses. 

metric-gate-semantic-decision

Partial completion is reported honestly and only under the exact producer policy.

Dock JSON Schema proves structure, not semantic capability or authority.

A skill, CLI instruction, protocol acknowledgment, or equality among request/ceiling/actual remains advisory. Only the actual host can produce Host Enforcement Facts. 

evaluation-authorization-capabi…

L. Replay and audit retention gap

The decisions require exact replay while allowing Engine verification to return unverifiable. They do not yet state the minimum historical resolvability invariant across owners.

Before implementation planning, settle that a closed result expected to be auditable must retain or be able to resolve:

exact policy and definition versions;

Charter and subject revisions;

posture snapshot and requirements;

Purpose, plan, binding, and verification observations;

attempts, source observations, evidence records, and evidence-set closure;

final context, results, and canonical explanation;

owner identities and supersession relations.

This does not require choosing a database or persistence layout. It is a semantic retention/reconstructability requirement.

M. UX and workflow state collisions

The combined system has several deliberately distinct negative or non-positive states:

held

refused

known_limited

unknown

stale

superseded

indeterminate

not_computable

not_applicable

historical pass not eligible for current reliance

A lower-density view must not collapse these into “failed,” “not ready,” or “blocked.” Every user-facing view should preserve at least:

stage + semantic owner + exact status + decisive reason + next permitted transition

Examples:

“Evaluation held: producer capability is unknown” is not a failed gate.

“Evaluation refused: authorization superseded” means no new gate truth exists.

“Gate indeterminate: current evidence is insufficient” is not a readiness result.

“Historical pass; current reliance not established” is not a rewritten fail.

“Diagnostic pass” must retain diagnostic provenance and cannot silently appear decision-seeking.

10. Recommended next review sequence

Jointly approve the object ontology and owner map. Confirm the six classes, reject a generic derived-artifact repository, and place Assurance Selection in Contracts.

Resolve the identity and closure memorandum. Cover deliberate new-evaluation occurrence identity, Selection-versus-currentness granularity, semantic times, historical resolvability, evidence-set closure, and planning-time applicability.

Resolve the authored-artifact relationship memorandum. Select Work Specification as the canonical work artifact, retire FeatureSpec from new semantics, and fix exact roles for Project Context, Environment Context, Decision Record, and Risk Record.

Resolve the operational-reliance memorandum. Select action-domain owners, Work Specification/Pipeline closeout and parent-eligibility boundaries, mutation authority, and the universal-versus-policy-declared Engine-verification rule.

Perform one final joint semantic consistency review of all three decision records. Normalize their stage diagrams, ownership tables, settled/deferred sections, projection vocabulary, and cross-references. Only after that should full task-level implementation planning begin.

Repository-bounded confidence is high for the ownership, dependency, artifact, projection, and current implementation boundaries stated in the packet. It is intentionally lower for the unseen detailed schemas of Work Specification, Decision Record, and Risk Record and for any external enforcing host. The packet itself reports the degraded repository index and relies on current-source inspection, while external publication and host adoption remain outside repository proof. 

hcm-artifact-system-seam-chatgp…

 

architecture-context

## Local reconciliation

### Conclusion

The `ADJUST` verdict is supported by repository truth. The consultation validates
the product direction and the three-decision semantic chain. It does not identify
a reason to reintroduce V1 compatibility or to make derived evaluation records
generic authored artifacts.

### Guidance accepted as repository-consistent

- Expand the tentative four-category model to distinguish at least:
  authored canonical project artifacts; canonical semantic definitions/policies;
  owner-produced canonical records/results; durable origin/attempt provenance;
  deterministic projections/representations; and transient operational state.
- Keep the generic Artifact Kind / Artifact Instance machinery scoped to the six
  authored project-artifact families. Persistence, canonical identity, and audit
  value do not by themselves make a record an authored artifact.
- Treat the full canonical explanation as Contracts-owned decision data. Only a
  density-bounded rendering of that explanation is a projection.
- Keep Assurance Selection and Evaluation Coordination as distinct
  responsibilities in the broader Contracts domain, with Engine remaining the
  sole Context Resolution and Evaluation Authorization owner.
- Keep Producer Semantic Capability independently admitted from both a
  producer's self-description and a Dock's operating ceiling. Exact physical
  placement remains a planning decision.
- Distinguish stable Producer Selection definition references from
  checkpoint-current capability, ceiling, availability, prerequisite, and
  semantic-time observations in the Readiness Basis.
- Add an explicit semantic evaluation-occurrence/request identity, separate from
  content fingerprints and transport correlation, so exact replay is
  distinguishable from a deliberately new evaluation with equal semantic
  inputs.
- Define Contracts-owned canonical evidence-set formation and closure before
  claim evaluation. Claim evaluators may choose eligible evidence only within
  that already closed set.
- Resolve prospective applicability before permitting it to prune an
  Observation Plan, and require final applicability to remain congruent with
  that basis.
- Keep historical evaluation truth immutable and distinguish it from
  Current-Use Eligibility and action-specific Operational Reliance
  Verification.
- Use Work Specification as the greenfield canonical work-artifact vocabulary
  unless a genuinely distinct Feature Spec semantic role is established.
- Preserve the distinction between artifact/profile conformance and evaluation
  readiness; the current SDK `profile_readiness` name is a future namespace
  collision, not evidence that the two concepts should merge.
- Normalize the two older proposed decisions after the new semantics are
  approved so their deferred wording and abbreviated stage diagrams defer to
  the later Evaluation Authorization/Readiness decision.

### Guidance requiring an explicit product decision

- Whether every positive operational reliance universally requires current
  Engine evaluation-authorization verification, or whether each exact
  action-domain reliance policy declares that dependency. Either choice must
  keep downstream action authorization separate from Engine evaluation
  authorization.
- The exact owner and authority path for Assurance Policy selection, exception
  admission, and Producer Semantic Capability. The Contracts-domain placements
  are directionally sound, but physical crate/repository placement is not
  established by current source.
- The Work Specification/Pipeline relationship for local closeout, parent
  eligibility, and separately authorized mutation.
- The minimum cross-owner semantic-time and historical-resolvability invariant
  required for replay and audit.

### Advice not promoted to a current requirement

- The eleven blocking rows should not automatically become eleven decision
  documents. They can be grouped into the four narrow semantic packages in
  Pro's recommended sequence if each remains complete.
- Concrete Decision Record and Risk Record schema relationships remain
  non-blocking because the bounded source does not establish their detailed
  schemas. Their prose must not become executable runtime authority.
- Posture feedback algorithms, explanation-density UI levels, cross-context
  gates, concrete catalogs/mappings, physical persistence, and external host
  adoption remain outside the immediate semantic closure unless the next
  planning scope explicitly includes them.

No product decision document was edited during this consultation.

