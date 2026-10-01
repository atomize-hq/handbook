# HCM canonical-object ontology — ChatGPT Pro review

- Date: 2026-09-03
- Conversation: https://chatgpt.com/c/6a98f18f-da0c-83ea-9367-c8bded598285
- Configuration: Chat surface; GPT-5.6 Sol; Pro power 5 of 5
- Review target:
  `docs/specs/handbook-contract-membrane/canonical-object-ontology-and-ownership-semantic-decision.md`
- Reviewed target SHA-256:
  `AB42B094949F9D0FAB671E03E861A50AF7B071F805D2CBB20A46A6C5A9B6E71D`
- Task/spec: Independent bounded review of the first proposed companion decision,
  covering only canonical-object ontology, the generic artifact boundary, and
  semantic-owner placement.
- Current approach: Six native object categories; exactly six authored artifact
  families in the generic lifecycle; dedicated owners for definitions, results,
  provenance, projections, and operations; Engine inward of Contracts and all
  owners inward of SDK/transports.

> Advisory only; verify against local project truth and authoritative docs; do
> not reduce scope without user approval.

## Prompt (verbatim)

We are already working on this project task.

Task/spec: Review the proposed canonical-object ontology and ownership semantic decision as the first of four narrow greenfield HCM companion decisions. Determine whether it completely and correctly settles only the object categories, generic Artifact Kind / Artifact Instance boundary, and semantic-owner map needed before implementation planning, without absorbing the later evaluation-closure, authored-artifact-relationship, or operational-reliance decisions.

Current approach: The repository-derived approach is to distinguish six categories: authored canonical project artifacts; canonical semantic definitions/policies; owner-produced canonical records/results; durable origin/attempt/source provenance; deterministic projections/representations/assemblies; and transient operational state. Only the six authored families—Project Authority/Charter, Project Context, Environment Context, Work Specification, Decision Record, and Risk Record—belong in the generic artifact lifecycle. All other canonical objects retain dedicated semantic owners. Assurance governance/exception admission, Assurance Selection, Producer Capability Authority, and Evaluation Coordination are distinct responsibilities in the broader Contracts bounded context; Engine remains inward of Contracts, and owners remain inward of SDK/transports. Physical crates, stores, APIs, schemas, and implementation planning remain deferred.

Project sources: Read all six attached files completely. Read canonical-object-ontology-and-ownership-semantic-decision.md first as the review target; then read hcm-artifact-system-seam-chatgpt-pro-source-packet-2026-09-02.md as the bounded repository-truth dossier; then the posture, evaluation-authorization/readiness, metric/gate, and architecture-context records. The excluded repository is unavailable and must not be inferred. Current source is boundary evidence, not a compatibility obligation.

Evidence, blockers, constraints, and validation: The target is PROPOSED human-review semantic authority only. It is 442 lines, has no unresolved placeholders or trailing whitespace, all local links resolve, and its SHA-256 is AB42B094949F9D0FAB671E03E861A50AF7B071F805D2CBB20A46A6C5A9B6E71D. The three companion decisions remain proposed and preserve their existing detailed semantics. This is a greenfield product: do not invent V1 compatibility, migration, legacy aliases, dual writes, schemas, APIs, storage, or task decomposition. The later decisions must retain ownership of evaluation-occurrence/evidence closure/applicability/history, exact artifact relationships, and operational reliance/closeout/promotion/mutation. A blocking finding must demonstrate semantic contradiction, authority collapse, missing required refusal/identity boundary, circular dependency, or scope leakage—not merely a wording preference or possible implementation choice.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Is the current approach the best way forward? Give a direct keep, adjust, or pivot recommendation; tradeoffs; what to keep/change first; inspections; validation; and pitfalls.

Return findings using exactly P1 Critical, P2 Required, P3 Optional/Consider, or P4 Nit. For each P1/P2, identify the exact target section, violated invariant, causal consequence, and smallest semantic correction. Specifically test:
1. whether the six top-level categories are mutually discriminating without pretending every object is persisted;
2. whether requests/candidates can be classified without confusing untrusted origin assertions, durable provenance, and transient state;
3. whether canonical semantic definitions/policies are cleanly separated from owner-produced results;
4. whether the generic artifact inclusion test is neither too broad nor circular;
5. whether Assurance Selection and Producer Capability Authority placement preserves Engine → Contracts dependency direction and avoids selector self-authorization;
6. whether projection, deterministic assembly, representation, canonical explanation, and explanation-density view are correctly separated;
7. whether the record accidentally decides any of the later three companion subjects;
8. whether any named object from the three companion decisions is misclassified or left without an owner;
9. whether the scenario corpus discriminates the high-risk false equivalences;
10. whether the settled/deferred boundary is sufficient for this record to be approved as input to the next loop.

Do not propose implementation work. If there are no P1/P2 findings, state that explicitly and say whether this candidate may proceed to the next semantic-decision loop. Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Response (verbatim)

P1 Critical

None.

I found no semantic contradiction, authority collapse, reversed dependency, or circular ownership that would require a pivot.

P2 Required

None.

I found no missing refusal or identity boundary, no misclassification that would place a non-artifact object into the generic Artifact Kind / Artifact Instance lifecycle, and no leakage that substantively decides evaluation closure, authored-artifact relationships, or operational reliance.

P3 Optional/Consider
P3-1 — Make the request/candidate-to-provenance object boundary even more explicit

Target sections: “Canonical vocabulary,” “Normative ontology invariants,” and “Object classification.”

The current semantics are sound: an in-flight request or candidate is transient; an origin owner may later close a durable occurrence record; downstream admission creates a separate owner result rather than upgrading the source assertion. The logged-process scenario also makes clear that the closed fact and terminal are new durable objects, not semantic promotion by logging. 

canonical-object-ontology-and-o… +2

The optional improvement is to make the two object identities explicit in the classification rows:

the live or unclosed Evaluation Authorization Request / evidence candidate is category 6;

an origin-closed occurrence record referring to it is a distinct category-4 provenance object;

an Engine authorization result or Contracts admission result is a further distinct category-3 object.

That would prevent a reader from interpreting “when retained” as a storage operation changing the ontology category of the same object. The record already supplies the necessary semantics, so this is clarification rather than correction.

P3-2 — Tighten the wording around ownership of the policy-resolution basis

Target sections: “Object classification” and “Semantic responsibility map.”

The candidate passes the selector-self-authorization test. Selector and policy definitions are owned by assurance-governance definition authorities, while Assurance Selection is expressly prohibited from admitting or authorizing its inputs. It consumes admitted inputs and produces Resolved Assurance Requirements with complete accounting. 

canonical-object-ontology-and-o… +2

There is nevertheless a possible double reading because the table says Assurance Selection owns the “policy-resolution basis,” while the posture decision calls that basis an exact admitted basis that the selector consults. 

posture-assurance-policy-semant…

A small wording improvement would distinguish:

admission/currentness of selector, policy, mapping, and candidate-definition inputs, owned by the appropriate assurance-governance definition authority; from

the exact selection-basis closure and accounting record produced by Assurance Selection over those already admitted inputs.

No new responsibility is required. This would merely make the already-selected anti-self-authorization boundary impossible to misread.

P3-3 — Keep projection-versus-representation classification explicitly operation-based, not medium-based

Target sections: “Canonical vocabulary,” “Object classification,” and “Projection and representation rules.”

The substantive separation is correct:

a projection owns bounded selection, disclosure, omission, lossiness, derivation, and output identity;

a deterministic assembly additionally owns multi-source ordering and budget accounting;

a representation owns encoding and framing;

the complete canonical explanation remains Contracts decision data;

an explanation-density view is a projection of that complete explanation. 

canonical-object-ontology-and-o… +1

The optional clarification concerns “Markdown review rendering,” which the classification table places unconditionally under deterministic projection, while the repository dossier distinguishes currently configured renderer references from projection-definition references. 

canonical-object-ontology-and-o…

 

hcm-artifact-system-seam-chatgp… +1

A medium-neutral statement would be stronger:

a Markdown review view is a projection when it selects, reorganizes, bounds, redacts, summarizes, or otherwise derives source content;

the Markdown encoding of an artifact or projection is its representation;

where both occur, the projection identity and Markdown representation identity remain distinct.

The existing rules already imply this layering, so approval need not wait for the wording change.

P3-4 — Add the profile-readiness versus evaluation-readiness collision to the scenario corpus

The bounded source packet explicitly identifies the existing SDK profile_readiness vocabulary as repository/profile artifact readiness and warns that it must not be conflated with the proposed evaluation-attempt Readiness Assessment. 

hcm-artifact-system-seam-chatgp…

The responsibility map already prevents the conflation: Engine owns profile conformance, while Contracts-domain Evaluation Coordination owns evaluation Readiness Basis and Assessment. 

canonical-object-ontology-and-o…

A small scenario would make that distinction executable as review guidance:

A profile and its required Artifact Instances are structurally present and valid, but the selected Observation Plan has unknown producer capability. Artifact/profile readiness may succeed; evaluation readiness remains unknown and a decision-seeking attempt is held.

This is optional scenario hardening, not a missing owner decision.

P4 Nit
P4-1 — Label the identity table as categories and subordinate forms

The ontology fixes six top-level categories and expressly makes representation a subordinate form within category 5, but the “Identity, lifecycle, and currentness by category” table presents Representation as its own peer row. 

canonical-object-ontology-and-o… +1

Changing the first-column heading to “Category or subordinate form”, or visually nesting Representation beneath category 5, would avoid a superficial seven-category reading.

A second tiny cleanup would change “Evaluation Purpose and Use Intent” in the classification table to “Evaluation Purpose, including its bound Evaluation Use Intent,” matching the later rule that such typed values ordinarily participate in a containing record rather than requiring independent repository status. 

canonical-object-ontology-and-o… +1

Recommendation: KEEP

The current approach is the best way forward.

The candidate completely and correctly settles the ontology, generic artifact boundary, and semantic-owner map required before implementation planning. It does so without reopening the three companion decisions and without taking ownership of the three later focused subjects.

There are no P1 or P2 findings. The candidate may proceed to the next semantic-decision loop.

Results of the ten requested tests
1. Six mutually discriminating categories without universal persistence

Pass.

The categories discriminate by native semantic role:

authored project truth;

admitted semantic rules;

owner determinations;

occurrence/source provenance;

views, assemblies, and encodings;

live operational coordination.

Persistence, authority, canonicality, authored-artifact status, projection identity, and representation identity are explicitly orthogonal. The record also says category alone does not impose retention on category-2, category-3, or category-5 objects. 

canonical-object-ontology-and-o… +1

That is a better model than the source packet’s preliminary four-way grouping because it separately exposes executable definitions, origin provenance, and representations—the three areas most likely to be swallowed by an overly broad “derived object” category. The source packet itself stated that the four-way grouping was a review question, not approved authority. 

hcm-artifact-system-seam-chatgp…

2. Requests and candidates versus source provenance and transient state

Pass, with P3-1 clarification available.

An untrusted request or candidate is not authorization, evidence, or another downstream owner result. Origin closure may establish only the occurrence proposition; downstream admission creates a separate category-3 result. The object table correctly keeps the Evaluation Authorization Request distinct from Engine’s Binding or refusal and the evidence candidate distinct from canonical evidence. 

canonical-object-ontology-and-o… +1

This agrees with the authorization decision, where the request remains untrusted until Engine acts, and with the metric decision, where an evidence candidate remains untrusted until Contracts admission produces a canonical evidence record. 

evaluation-authorization-capabi…

 

metric-gate-semantic-decision

3. Definitions and policies versus owner-produced results

Pass.

The record consistently separates:

policy, vocabulary, capability, contract, metric, and gate definitions;

admission, resolution, authorization, readiness, evidence, evaluation, gate, reliance, and mutation results.

It also fixes the decisive rule that a human-authored artifact may be source material for an executable policy, but artifact promotion cannot perform the semantic owner’s admission. 

canonical-object-ontology-and-o… +1

That preserves the companion metric decision’s distinction between a Metric Definition and its metric result, and between Gate Policy and semantic gate decision. 

metric-gate-semantic-decision

4. Generic Artifact Kind / Artifact Instance inclusion test

Pass.

The four-part inclusion test is restrictive without being improperly circular. Membership is a closed semantic declaration: an object must be project-authored truth, must naturally use the authoring/promotion/revision/mutation lifecycle, must belong to one of the six selected families, and must not use artifact promotion as a substitute for another owner’s admission. 

canonical-object-ontology-and-o…

The explicit enumeration is a feature, not a defect. It means a seventh family requires a later semantic decision rather than entering through incidental durability, a new schema, or repository convenience. That matches the repository evidence: Engine already has generic kind/instance registries and lifecycle operations, but the three companion decisions never said every new semantic record should become an Artifact Instance. 

hcm-artifact-system-seam-chatgp… +1

Rejecting a generic derived artifact category is especially important. It prevents immutable policies, semantic results, occurrence records, projections, caches, and transport values from acquiring authored candidate/promotion/mutation semantics merely because common repository primitives are convenient. 

canonical-object-ontology-and-o…

5. Assurance Selection, Producer Capability Authority, and dependency direction

Pass.

The target preserves the required direction:

Engine <- Contracts-domain responsibilities <- SDK / transports

Engine owns posture and Context Resolution authority. Contracts consumes those exact owner results; Engine does not depend outward on Contracts to establish its own authority. SDK, CLI, Tauri, Flow, Pipeline, Docks, and hosts remain consumers, projections, or operational owners rather than semantic substitutes. 

canonical-object-ontology-and-o…

 

hcm-artifact-system-seam-chatgp…

 

architecture-context

Selector self-authorization is prevented through three separate boundaries:

governance/definition authorities admit the selector, policies, and typed exception policy;

exception admission establishes the exact authorized record;

Assurance Selection applies only admitted inputs and bounded effects.

Producer Capability Authority is also independently attributable from producer self-description and Dock operating ceiling. That prevents current capability from feeding backward into requirements selection. 

canonical-object-ontology-and-o…

This composes correctly with the evaluation decision’s placement of Evaluation Coordination inside the broader Contracts domain while keeping Engine authorization and producer capability separately owned. 

evaluation-authorization-capabi…

6. Projection, assembly, representation, explanation, and density view

Pass.

The record correctly establishes five different propositions:

source owners establish source truth;

a projection owner establishes what was selected, omitted, redacted, derived, or exposed;

an assembly owner establishes multi-source composition and budget accounting;

a representation/schema owner establishes encoding and framing properties;

Contracts establishes the complete canonical explanation accompanying its decision truth.

An explanation-density view is then a projection of the canonical explanation, not a second gate explanation and not a lower-resolution truth branch. 

canonical-object-ontology-and-o…

This matches the metric decision, which defines the explanation as canonical decision data containing complete evidence, claim, metric, condition, precedence, and decisive-reason accounting. 

metric-gate-semantic-decision

It also preserves Flow’s narrower assembly authority: Flow may select, order, omit, summarize, and budget packet contents without acquiring authority over the included Charter, grounding, posture, or gate propositions. 

hcm-artifact-system-seam-chatgp…

7. Leakage into the later three decisions

Pass.

The target repeatedly and concretely defers:

evaluation-occurrence identity, evidence-set formation and closure, prospective applicability, semantic times, and historical resolvability;

exact fields and relationships among the six authored artifact families and evaluation records;

Current-Use Eligibility, Operational Reliance Basis, closeout, parent eligibility, promotion, release/deployment effects, verification policy, and mutation authority. 

canonical-object-ontology-and-o… +2

The category-level statements about immutable historical identity and separate currentness are necessary anti-collapse invariants. They do not choose the later occurrence algorithm, evidence-set rules, relationship cardinalities, retention minimums, reliance policy, or mutation path.

8. Classification and ownership of companion-decision objects

Pass.

The classification table covers the material objects from all three companion decisions:

posture scope, snapshot, selection basis, requirements, exceptions, observations, recommendations, and transition results;

Observation Plan, Producer Selections, Evaluation Purpose, authorization request/binding/refusal/verification, readiness objects, disposition, basis, attempts, reach, terminals, and host facts;

contract/claim/evidence-policy/metric/gate definitions, evidence records and closure, applicability, claim evaluations, metric results, gate decisions, and canonical explanation. 

canonical-object-ontology-and-o…

Typed coordinates and bounded values are also correctly prevented from becoming independent repository objects merely because they have semantic types. 

canonical-object-ontology-and-o…

No companion object is placed under SDK or transport ownership, and no semantic result is incorrectly promoted into the six-family artifact lifecycle.

9. Scenario corpus discrimination

Pass.

The scenarios test the principal false equivalences:

artifact versus resolved owner record;

authored policy prose versus executable policy;

exception authorization versus admission versus application;

durable attempt history versus evidence or gate truth;

self-described capability versus independently admitted semantic capability;

canonical explanation versus density projection;

assembly versus included-source ownership;

representation integrity versus semantic authority;

unified catalog versus universal repository;

logs versus closed provenance;

historical identity versus current applicability;

projection reuse versus source-owner truth;

physical colocation versus semantic ownership. 

canonical-object-ontology-and-o…

The scenario set is sufficient for approval. P3-4 would add one particularly useful vocabulary-collision case, but its absence does not leave the owner map unresolved.

10. Settled/deferred sufficiency

Pass.

The settled list exactly matches this decision’s proper job: ontology, canonicality scope, six artifact families, inclusion/exclusion, owner boundaries, dependency direction, capability independence, and projection distinctions. The deferred list leaves concrete relationships, evaluation closure, operational reliance, implementation shapes, physical placement, and delivery planning to their proper later authorities. 

canonical-object-ontology-and-o…

That is sufficient for this record to become input to the next semantic-decision loop.

Tradeoffs

The design deliberately trades universal repository simplicity for semantic integrity. Implementations will eventually need to resolve and navigate several owner-native object families rather than treating everything as one artifact type. That increases explicit identity, resolution, currentness, and catalog work, but it avoids much more damaging lifecycle and authority ambiguity.

The closed six-family artifact enumeration introduces intentional governance friction. A genuinely new authored project-truth role cannot appear merely because a developer adds a schema or store entry. It requires a semantic decision. In a greenfield authority system, that is the correct bias.

The model also creates several objects around one user-visible operation—a request, origin provenance, owner admission result, projection, and representation. That is more conceptually demanding than one polymorphic “record,” but each object answers a different proposition and can therefore fail, remain historical, or be projected without contaminating the others.

Finally, keeping Assurance Selection, Producer Capability Authority, Evaluation Coordination, and evidence/evaluation responsibilities semantically separate may produce more internal boundaries even when they are physically colocated. That cost is justified because it prevents capability-driven requirement relaxation, selector self-authorization, and readiness recomputation by evaluators.

What to keep unchanged

Keep the following as written:

the six top-level categories and subordinate representation distinction;

exactly six generic authored-artifact families;

Work Specification as the greenfield name, with no FeatureSpec compatibility fiction;

owner- and proposition-scoped canonicality;

the four-condition artifact inclusion test;

rejection of a universal derived artifact;

separate definition, admission, selection, coordination, evaluation, and action owners;

Engine inward of Contracts and all owners inward of SDK/transports;

canonical explanation versus explanation-density projection;

immutable historical result versus separately established currentness;

the three explicit later-decision deferrals.

No category count, owner placement, or settled/deferred boundary should be reopened before the next loop.

The optional edits should be taken, if desired, in this order: clarify request/candidate occurrence identity, tighten policy-resolution-basis wording, make Markdown projection/representation language medium-neutral, and add the profile-readiness scenario.

Inspections and validation

I read the six supplied files completely in the requested order and checked the target against the full companion semantics and bounded architecture dossier.

The exact target bytes produce SHA-256:

AB42B094949F9D0FAB671E03E861A50AF7B071F805D2CBB20A46A6C5A9B6E71D

The file has 442 logical lines, no trailing whitespace, and no unresolved TODO/TBD/FIXME/placeholder markers. The posture, evaluation-authorization/readiness, metric/gate, and architecture-context files also match the hashes recorded in the bounded source packet.

The target’s three local links name the canonical companion basenames and correspond to the supplied records. Because the excluded repository tree is unavailable and the uploaded copies have attachment suffixes, repository-relative link resolution itself remains supported by the supplied validation statement rather than independently reconstructed from an unavailable tree.

No excluded repository behavior, migration requirement, compatibility alias, physical crate plan, schema, API, persistence layout, or implementation task was inferred.

Pitfalls for the next three loops

The evaluation-identity/closure decision should define occurrence identity, evidence-set closure, semantic-time bindings, and historical resolvability without turning those records into Artifact Instances or reopening their owners.

The authored-artifact-relationships decision should add exact references and cardinalities without making a Decision Record, Risk Record, Work Specification, or Charter the executable owner of a Contracts policy, Engine authorization, gate result, or operational action.

The operational-reliance decision must consume immutable historical truth through fresh owner checks. It must not place current-use state inside the gate decision, use a projection as the reliance basis, or let closeout, promotion, release, or mutation flow directly from pass.

Across all three, avoid generic labels such as derived artifact, readiness, current, canonical, or approved unless their exact owner, proposition, category, and semantic time are explicit.

Final disposition: KEEP. No P1 or P2 findings. The candidate may proceed to the next semantic-decision loop as proposed human-review authority; approval would not authorize implementation.

## Local reconciliation

The controlling verdict is `KEEP`, with no P1 or P2 findings. Repository
inspection supports that verdict.

The four P3 clarifications and one P4 cleanup are accepted because they sharpen
existing semantics without changing category count, ownership, or the
settled/deferred boundary:

- distinguish the live request/candidate from a separately closed durable
  provenance occurrence rather than implying that storage changes one object's
  category;
- distinguish admission/currentness of policy inputs from the
  Assurance-Selection-produced selection-basis closure and accounting;
- classify Markdown by operation rather than medium: bounded/derived view as
  projection, Markdown encoding as representation;
- add a profile-conformance versus Evaluation Readiness scenario; and
- label the lifecycle table as categories and subordinate forms and keep
  Evaluation Use Intent within Evaluation Purpose.

These are non-material clarifications under the review policy and do not require
another causal Pro round. No implementation, schema, API, persistence, or later
companion-decision scope is accepted by this review.

