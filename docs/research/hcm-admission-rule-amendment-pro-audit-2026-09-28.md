# HCM admission-rule amendment — Pro completeness audit

Date: 2026-09-28.
Status: Complete — no qualifying findings; HR-001/P2-01 semantic amendment closed.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

Scope: HR-001/P2-01 only. The user accepted the focused decision and requested a
narrow amendment plus a thorough audit of its completeness. No whole-package
approval or implementation authorization.

Conversation: https://chatgpt.com/c/6ab96a0e-edd4-83ea-8361-21068a69a724

Model: 6 Pro, effort 5 of 5, verified in UI before submission. The first submission
failed with a page error; after reloading and restoring the attachment, the page
confirmed the new user message and active response generation. No final review
verdict is inferred from submission.

Packet SHA-256: `1144a990d2a1593ada4fbc0903430dff724e014a39480988d3cff9af7149d248`.
The attached packet contains the index/manifest, exact delta, and full updated
seven sources with source-local line numbers. No executable source or unrelated
working-tree changes were included.

## Source boundary and validation

```json
{
  "head": "18d7137db593fb76460094c7d3aaa422e65400eb",
  "baseline_manifest_sha256": "83005d416306f85204925b2e5beb652c16968f8769d167672db6ea008f016289",
  "delta_sha256": "f0a1bb441c5911060b3ef4f9ab5cd71c6a4a874c73f29ce3d5285814c9dcb8b6",
  "files": [
    {
      "path": "docs/specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md",
      "before_sha256": "7ab9d23c484426a0ba2c76d571c1faba53ce4d4dd14eac3da02089c7caf9f58a",
      "after_sha256": "406e743c1903fc153eab27b2fda19fc154b50ff223e994cf4097f4388e978102"
    },
    {
      "path": "docs/specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md",
      "before_sha256": "7c240ef0124075a111f09f8dc25e18aab340b587e404c34e4d9984c5ae5510ce",
      "after_sha256": "f903a478917874182d78cc72de7392aa9033ff63c01f00ec9068bfebc6590deb"
    },
    {
      "path": "docs/specs/handbook-contract-membrane/semantic-model/objects-and-governance.md",
      "before_sha256": "dc33d3e2887cba63874b55fff75b360000a9f5ae208ee6be0a72751643c32b38",
      "after_sha256": "e47d79e204f68b664eafa438a29c39952c4153a3754e025a2e51b6589ad0f919"
    },
    {
      "path": "docs/specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md",
      "before_sha256": "539bbed534e20e31396f792d107121c567c41c09fd48e040fc9e3b50216eba48",
      "after_sha256": "f4044b020b5c6a51ef58224c815e39887ff60f5cd6f67d762f7da118b9ef8857"
    },
    {
      "path": "docs/specs/handbook-contract-membrane/semantic-model/README.md",
      "before_sha256": "1789e651722850371d813709707a5a6be76cbb2e5fb417df3a889cbc8e046907",
      "after_sha256": "8e302f5648740c6c363768d18da8153325159a5728097965cd79be4b7301e927"
    },
    {
      "path": "docs/specs/handbook-contract-membrane/semantic-model/review-decision-register.md",
      "before_sha256": "206eff93b126129776e080289157caff1a823d042f777e2b86b142dab2e6b7a6",
      "after_sha256": "5d73e64ef7e18335a2acd46b51adcf7d1d42966ba21f409b46e770068bbf7b7f"
    },
    {
      "path": "docs/research/hcm-admission-rule-authority-pro-decision-review-2026-09-28.md",
      "before_sha256": "8c255b29849728582cc586b66da2d66bc89b327b9f0040cc38f92b7aa6d22732",
      "after_sha256": "9f76e507229111491a76a9058e54582228cb343b5f15d6eeaa05029cccf238f3"
    }
  ],
  "validation": [
    "git diff --check passed",
    "UTF-8 reads and replacement-character checks passed",
    "Local Markdown file targets resolve; new governance heading anchor exists",
    "HR-004 and later register content unchanged",
    "Manual complete-delta and cross-document responsibility review; no runtime tests for prose-only change"
  ]
}
```

## Exact submitted prompt

```text
Review mode: remediation-follow-up
Review context: Explicitly requested continuity for this admission-rule decision; continue in this same conversation.
Review boundary: Baseline HEAD 18d7137db593fb76460094c7d3aaa422e65400eb plus previously reviewed full packet SHA-256 453facd9382110929453a071fdae33a82d1e9f8b15fe4584f9c516db8dd801c1. Exact pre-amendment working-copy manifest digest 83005d416306f85204925b2e5beb652c16968f8769d167672db6ea008f016289. Current content-addressed delta SHA-256 f0a1bb441c5911060b3ef4f9ab5cd71c6a4a874c73f29ce3d5285814c9dcb8b6. These are dirty documentation snapshots, not a committed range.
Review target: Thorough completeness and consistency audit of the narrow HR-001/P2-01 amendment across all four normative parts, the approval guide, review-decision register, and subsequent-decision note in the prior consultation record. The user accepted the configuration approach and your closing refinements, requested their incorporation, and explicitly requested this audit.

Supported inputs and behavior: Proposed Markdown semantic requirements, not executable parser behavior. Supported scenarios include standalone advisory initialization for an exact project/scope, configured-agent instruction provenance, independently admitted ordinary policy candidates, explicit successor activation under prior authority, stale predecessor/concurrent candidate finalization, exact replay, unreadable governance, uncertain activation, and present use of historical policy admissions.
Supported platforms and dialects: Handbook-only semantic specification across CLI/SDK/agent usage; no concrete implementation platform, schema, storage, or API selected. Markdown headings, tables, and explicit cross-references are the document forms in scope.
In-scope invariants: Fixed Handbook establishment rule outside project candidate admission; no self-authorized bootstrap or successor; unambiguous configuration/scope/owner-rule basis; native governance lifecycle without recursive dependency on not-yet-admitted project action policies; clear separation from ordinary action authority; exact activation/cutover/current predecessor and active-at-finalization admission; immutable history and exact replay; advisory provenance without fabricated authentication; domain owner constraints retained; replacement is not automatic revocation, re-admission, continued usability, or successor compliance; no new IAM/service/login/approval-prompt requirement; no seventh authored artifact family; no implicit approval or implementation authorization beyond this accepted item.
Required material consequence: Two reasonable implementations could follow the updated normative text and differ materially about governing authority, supported initialization/replacement, applicability/scope, cutover, refusals, immutable history, policy usability, or whether extra identity infrastructure is mandatory.
Blocking threshold: P1 critical or P2 required, with a concrete supported scenario, exact conflicting/missing passages, material consequence, and causal connection to the accepted remediation or its delta. Cosmetic wording is non-blocking.
Accepted prior findings: P2-01/HR-001 missing authority establishment/activation/replacement. User has now accepted the focused resolution: project configuration under a fixed Handbook establishment rule; existing-authority successor activation; exact activation cutover and refusal of stale-basis admission finalization. Your prior proposed closing choices are authorized for this amendment. Prior consultation's earlier "not yet accepted" prose remains historical, with a dated subsequent-decision section explicitly superseding its status.
Deferred or out-of-scope concerns: P2-02 universal/conditional posture selection, P2-03 all-pruned plans, P2-04 post-commit verification, existing guide lifecycle/order issues, implementation grounding, schemas/APIs/storage/crate placement, concrete per-project admission requirements, integrations, and whole-package approval. Do not turn those pre-existing issues into failures of this amendment unless it changes them or creates a causal regression.
Project sources: Attached hcm-admission-amendment-audit-packet.md. Read its index/manifest first, exact diffs second, all current source artifacts third. It contains the complete updated four parts, guide, register, and prior focused consultation with acceptance note. Source-local lines are prefixed. Packet SHA-256 1144a990d2a1593ada4fbc0903430dff724e014a39480988d3cff9af7149d248. Current sources supersede old attachments for this review. Excluded executable source is unavailable and must not be inferred.
Validation evidence: git diff --check passed; seven files decoded as UTF-8 without replacement characters; relative Markdown file targets resolve and new governance heading anchor exists; HR-004 and later register entries are unchanged; manual complete-delta and cross-document responsibility review. No runtime tests were run for prose-only edits. No staging or commit; unrelated dirty work preserved.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

For remediation-follow-up, report a blocking finding only when it is reachable through supported behavior, violates an in-scope invariant or accepted remediation objective, meets the material-consequence threshold, and was introduced by the delta, is a regression caused by it, or proves that an accepted root cause remains unfixed. You may inspect unchanged supporting code to establish causality, but unchanged pre-existing behavior is not an in-scope finding merely because you observed it.

After the accepted findings are fixed and no qualifying delta regression remains, stop. Do not enumerate adjacent lexical variants, unsupported dialects, new parser classes, or broader platform behavior unless they prove the same causal mechanism remains unfixed, were introduced or materially worsened by the delta, or review mode is comprehensive-audit. Conservative false negatives are non-blocking in remediation-follow-up unless introduced by the delta or explicitly in scope. Label other observations as deferred, not blocking. Always report a reachable material defect introduced by the delta even when its exact form was not anticipated.

The same causal mechanism means the same parsing state, normalization rule, validation omission, provenance error, authority decision, or equivalent trust transition remains operative. Superficial textual similarity is insufficient.

Return findings first by severity with file/function, supported-input reachability, violated invariant, material consequence, and delta causality. State explicitly when there are no qualifying findings.

Also provide a compact coverage matrix for the accepted requirements across the four parts, guide, and register. Check the category definitions, universal candidate-admission statements, Policy Admission Closure vocabulary and classification, lifecycle/currentness table, responsibility maps, action-chain universal wording, concrete examples, and settled/deferred statements—not just the new paragraphs. In particular, distinguish failing to resolve a governing basis from fabricating a Policy Admission Closure with a nonexistent version. Identify any remaining required language changes precisely. Conclude whether HR-001's amendment is complete or needs specific corrections. Text findings only; no patch archive. This audit cannot approve the remaining package or authorize implementation.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

```

## Pro response and local reconciliation

Retrieved 2026-09-28. **No qualifying findings:** Pro reported no P1 Critical or
P2 Required defect, no remaining instance of the accepted root cause, and no
material regression introduced by the amendment. Its controlling conclusion is
that HR-001 is complete at the semantic level; no further normative correction
is required.

Pro's coverage review includes all four parts, the guide, register, and prior
consultation acceptance note. It explicitly checked bootstrap and candidate
admission separation, unresolved-governance refusals, replacement permission and
cutover, ordinary action boundaries, truthful advisory authority, and the
distinction between historical admission, present usability, and successor-rule
compliance. It found the six governance examples and settled/deferred boundaries
consistent with the accepted decision.

**Local reconciliation:** Before updating audit bookkeeping, all seven local
source SHA-256 values matched the submitted manifest. The reviewed normative
passages preserve the fixed establishment rule, exact active-at-decision basis,
no fabricated missing version, scoped activation lifecycle, and separate current
usability. No contrary local evidence or required correction was identified.
The four normative parts and guide remain unchanged after this clean review.

Only the register, this audit record, and the prior consultation's current-status
note are updated to record completion. The exact submitted prompt, source hashes,
and delta are preserved below/above as review provenance. Pro's reported reverse-
delta reconstruction and attachment newline-normalization checks are reviewer
validation; local verification independently checked current source hashes.
No runtime tests or executable implementation conformance are claimed.

**Disposition:** HR-001/P2-01 resolved at semantic level, with focused amendment
audit complete. Implementation realization remains deferred. P2-02, P2-03,
P2-04, pre-existing guide follow-ups, whole-package approval, and implementation
authorization remain open. No staging or commit was performed.

## Complete Pro response

Captured from the completed response's visible container. Headings and tables
are restored for readability; attachment-preview labels referring to the submitted
packet are omitted. Response prose is retained.

Raw visible-text capture SHA-256: `0a550d0eb0e8dc4cde2154668826a39f7abb5d7087c5f7abc36867098daf55cf`.

### Findings

No qualifying findings. I found no P1 Critical or P2 Required defect in the submitted HR-001/P2-01 amendment, no remaining instance of the accepted root cause, and no material regression caused by this delta.

The HR-001 amendment is complete at the semantic level. No further normative language change is required to close this amendment. The updated shared section now establishes the source of bootstrap authority, the responsible governance boundary, prior-authority successor activation, exact scope and cutover semantics, and the consequences of replacement. Those rules are carried into the ontology and consuming parts rather than left only in explanatory prose.

This conclusion is limited to the accepted admission-governance decision and its incorporation. It does not approve the remaining package or establish executable implementation.

### Reviewed snapshot and verification

I read the index and manifest, the complete amendment delta, and all seven current source artifacts, including the retained consultation and subsequent-decision note.

The supplied packet identity is reproducible with its LF line-ending convention. The mounted attachment contains CRLF line endings; normalizing them to LF produces the stated packet SHA-256, 1144a990d2a1593ada4fbc0903430dff724e014a39480988d3cff9af7149d248. The extracted delta matches f0a1bb441c5911060b3ef4f9ab5cd71c6a4a874c73f29ce3d5285814c9dcb8b6.

I also verified that:

All seven extracted current sources match their manifest’s after_sha256.
Reversing the exact delta in an isolated working copy reconstructs all seven before_sha256 values.
The reconstructed pre-amendment four parts and guide match their counterparts in the previously reviewed full packet.
HR-004 and everything following it are unchanged, and all seven current sources decode as UTF-8 without replacement characters.

These are document and snapshot checks. No runtime tests were performed, and no executable-source conformance is claimed.

### Compact coverage matrix

All locations below are source-local lines. The six semantic-package paths are relative to docs/specs/handbook-contract-membrane/semantic-model/.

| Source and locations | Accepted requirements covered | Assessment |
| --- | --- | --- |
| objects-and-governance.md — shared admission 25–65, 69–178; vocabulary 240–241; category definition 276; universal candidate rule 329–334; classification 370–381; responsibilities 459–460; lifecycle/currentness 506–507; settled boundary 602–605 | Fixed Handbook root; advisory initialization; exact configuration/scope/owner-rule basis; prior-authority replacement; cutover; stale finalization; replay; uncertainty; history and present use; category-2 configurations versus category-3 outcomes; no seventh artifact family. | Complete. The native establishment exception is explicit, while ordinary candidates still require independent Policy Admission Closure. Configuration establishment is not a general alternative admission path. |
| assurance-and-evaluation-preparation.md — 225–244, 635–644; existing responsibility maps and settled/deferred sections | Assurance admission inherits active governance; domain constraints remain; Assurance Selection cannot establish or replace its governing rules; configuration replacement supplies no Engine authority and does not rewrite requirements, plans, Purpose, readiness, or prospective basis. | Complete. Neither automatic re-admission nor automatic continued usability is inferred. Existing dependency/currentness rules continue to control consumption. |
| evidence-and-evaluation.md — 1063–1079; definition-owner maps 712, 1089–1093 and existing historical-resolution rules | Shared governance covers definition/policy admissions, including Prospective Applicability and Canonical Evidence Set Formation Policies; historical admission, present usability, and successor compliance remain distinct; no substitution into formed purposes or contexts. | Complete. The amendment does not turn replacement into evidence-membership modification, applicability recomputation, or reevaluation of historical truth. |
| operational-reliance-and-actions.md — 166–196, checked against vocabulary, universal action wording, 360–366, 554–583, responsibility map, examples, and settled/deferred sections | Bounded native governance lifecycle; no fabricated historical evaluation or circular project-action-policy prerequisite; ordinary action-policy admission inherits the active basis; ordinary reliance, authorization, target, and commit boundaries remain separate. | Complete. The exception applies to admission-governance initialization and successor activation, not configuration changes generally or unrelated operational effects. |
| README.md — 19–31, 83, 96–109, 113–135 | Principal decision 2 reflects the accepted root, advisory setup, prior permission, explicit activation, historical preservation, and inactive-draft behavior; bounded acceptance is distinguished from remaining package approval and implementation. | Complete. The guide summarizes the owning rule without independently redefining it. |
| review-decision-register.md — 30–86, 167–190 | HR-001 records the accepted semantics and incorporated locations; implementation mechanisms remain deferred; audit outcome is separate from human acceptance; HR-003 retains truthful advisory-authority limits. | Complete. “Resolved at semantic level” and “focused audit pending” describe different events and are not contradictory. |
| docs/research/hcm-admission-rule-authority-pro-decision-review-2026-09-28.md — status 4; subsequent decision 78–92 | Records acceptance of the previously proposed refinements, identifies their incorporation, preserves the earlier consultation as history, and leaves the other findings and package approval open. | Complete. Earlier “not yet accepted” statements are historical, not a competing current disposition. |

### Decisive consistency checks
#### 1. Bootstrap authority is established without recursive self-admission

The amended rule provides the missing starting authority: an exact, versioned Handbook-defined establishment rule outside the project policy-candidate population. Neither an ordinary candidate nor the proposed governing configuration may select, admit, weaken, or replace the rule authorizing its establishment. Changing that Handbook rule is expressly not a project configuration operation.

Initialization then has a bounded responsibility: establish that initial setup applies to the exact project, validate the configuration and owner-rule bindings, and record the instruction and replacement-authority basis. Unreadable or unverifiable governance in an established project cannot authorize another initialization; adding a scope must follow existing governance. These provisions close both the original bootstrap question and the potential reset-through-missing-state shortcut.

The ontology changes are important to this conclusion. The configuration is now explicitly a category-2 owner-established definition under its native governance lifecycle; its establishment, activation, and refusal outcomes are category-3 owner results. The ordinary candidate path remains separate.

I checked the broader candidate-admission statements, including the opening shared rule and the authored-relationship passages. Read with the explicit distinction in the vocabulary and universal ontology invariant, they do not require a governing configuration to recursively obtain an ordinary Policy Admission Closure. Nor does carrying configuration rationale in a Decision Record change the configuration’s native lifecycle. The shared section expressly makes that record optional and non-activating.



There is therefore no required correction to the universal candidate language for this amendment.

#### 2. Failure to resolve governance is not a Policy Admission Closure under a fictional version

This distinction is adequately specified by the combined requirements, even though no new runtime refusal type is named.

A Policy Admission Closure closes an admission or refusal of an exact candidate under an exact governing configuration and owner admission-rule basis. Its vocabulary and classification require those identities, their semantic content, scope, and active-at-decision basis. The new shared rule separately requires refusal when governance cannot be resolved and prohibits inventing an applicable version or unresolved inputs.




Consequently, the supported cases have different truthful outcomes:

| Supported case | What the owner may establish |
| --- | --- |
| Exact active governance resolves, but the candidate violates a declared admission requirement | A refusal decision under that exact basis may be recorded in a Policy Admission Closure, with the decisive rule and accounting. |
| The active governing configuration or applicable owner-rule basis cannot be established | A refusal of the affected operation, retaining available decisive provenance. It cannot manufacture the required basis and call the result a Policy Admission Closure under a nonexistent, guessed, or substituted version. |
| Validation began under C1, but C2 activates before finalization | A stale-basis finalization refusal. It must not assert that C1 remained active or that validation performed under C1 was actually a C2 candidate decision. A fresh decision under C2 is distinct. |
| A completed historical C1 closure is replayed after C2 activates | The same historical closure, not a new admission under either configuration and not a present-currentness assertion. |


The refusal-provenance paragraph reinforces this by requiring preservation only insofar as inputs were established, and by forbidding fabrication of unresolved inputs. That qualification does not make exact governing identities optional inside an otherwise valid Policy Admission Closure; it permits truthful reporting when the operation cannot establish them.

No additional semantic record type, nullable-version convention, or schema decision is needed to complete this distinction. Those would be implementation choices, not missing remediation requirements.

#### 3. Replacement permission and activation are separate from successor content

Replacement binds the exact current predecessor, proposed successor, affected scope, and previously established replacement-authority basis. The successor cannot supply its own missing permission. Supported changes to replacement-authority terms must themselves be authorized by the prior terms and become effective only after activation.

The text also preserves the intended distinction between authority over admission requirements and admission of ordinary candidates: a legitimate replacement need not satisfy the predecessor’s ordinary candidate-admission requirements or make every configurable requirement stricter. It must still satisfy Handbook’s non-relaxable floor and applicable domain constraints. This is the accepted governance boundary, not a candidate bypass.

The cutover provisions cover the material concurrency cases without choosing a mechanism:

A replacement cannot overwrite a stale predecessor.
Activation success requires committed active-state change.
Inactive edits and validation do not activate content.
Conflicting immutable-identity reuse refuses.
Replay cannot re-execute activation or restore an older active version.
An uncertain outcome remains uncertain until authoritative state is resolved; timeout is not permission to repeat a possibly committed effect.

The adjacent active-at-finalization rule and concrete C1/C2 example establish the candidate/replacement ordering. An admission completed before cutover retains its historical basis; an unfinished decision cannot finalize on the displaced basis.


I found no unselected semantic cutover decision hidden behind the deferral of concurrency or recovery mechanics.

#### 4. The native governance lifecycle does not weaken ordinary action authority

The shared section and action chapter both identify the same bounded exception: admission-governance initialization and successor activation do not require ordinary candidate admission, a fabricated historical evaluation, or an action policy whose admission depends on the configuration being established.

That exception retains separately attributable validation, permission for the exact activation, expected-predecessor/currentness, cutover, and committed-effect propositions. It expressly grants no authority for evaluation, work progression, posture mutation, or other operational effects.


I checked this against the action chapter’s universal wording and summary statements. The defined ordinary Action Attempt still requires its Action Authorization Binding and expected-target verification, and the ordinary mutation path retains its through-commit checks. The expressly scoped governance lifecycle does not make those checks optional for those actions. Conversely, those general action requirements do not require inventing a circular prerequisite for the specifically established bootstrap lifecycle.


The existing broad summaries therefore do not create a qualifying contradiction when read with the amendment’s explicit scope. Repeating the exception in every summary or responsibility row is not a required remediation change.

#### 5. Advisory authority remains usable and truthfully limited

The standalone path is not reduced to a dry run. An explicit local initialization instruction, including a configured agent acting under that instruction, may supply advisory establishment authority. The operation still has to validate the exact configuration and scope and establish the actual activation outcome.

The provenance rule distinguishes the strength of individual propositions. A claimed principal or delegation remains advisory unless independently established; configuration identity, validation, predecessor currentness, and committed activation may nevertheless be owner-verified. Filesystem access, an asserted name, a prompt, or a configured role cannot be reported as stronger authority than the accepted basis supplies.


This matches the ordinary action chapter’s existing advisory/verified distinction without inheriting its ordinary action-policy prerequisites into bootstrap. It neither mandates a login, service, additional confirmation, or identity platform nor makes every prompt sufficient for every later replacement. Later replacements remain governed by the previously established permission.

#### 6. Replacement preserves history without inventing either grandfathering or retroactive re-admission

The updated parts consistently preserve three separate propositions:

Historical admission: the exact policy was admitted under C1.

Present usability: the definition owner and consuming owner establish whether that exact policy may be used now, under the applicable scope, compatibility, currentness, and dependency rules.

Successor-rule compliance: the policy has been assessed under C2 and satisfies its requirements.

The first does not prove either of the other two. Replacement alone neither revokes nor re-admits an existing policy, does not establish its compliance with the successor, and does not guarantee continued usability. A required successor assessment is a distinct owner determination. Existing requirements, purposes, evidence, and results cannot silently acquire replacement definitions.

The consuming additions preserve this distinction at assurance/preparation, evidence/evaluation, and operational reliance. In particular, “active at the admission decision” is not turned into “substitute the newest definition into every later consumer.” Present-use consequences remain with the already declared owner and consumer rules.



Different outcomes under different declared dependency rules are intentional policy variation, not an unresolved interpretation of replacement.

### Examples, status, and deferred boundaries

The six new governance examples cover initial advisory setup, candidate and successor self-authorization, inactive content versus conflicting identity reuse, concurrent activation and admission, historical policy use, unreadable governance, and uncertain commit. They agree with the operative requirements and do not silently introduce a stronger authority mode or a concrete implementation mechanism. Configured-agent provenance and false authentication claims are additionally addressed by the normative initialization/provenance rules and the preserved consultation examples.


The acceptance bookkeeping is also consistent:

The guide records the bounded September 28, 2026 acceptance while leaving the package proposed.
HR-001 records semantic resolution and incorporation while reserving the independent audit outcome.
The consultation’s dated subsequent-decision section identifies the accepted refinements and explicitly preserves the earlier pending-acceptance statements as history.
HR-003’s updated reference to HR-001 does not alter ordinary action-authority limits.
HR-004 and later entries remain unchanged.

Those statements do not retroactively turn the earlier consultation into an approval instrument or promote this single decision into package approval.



The remaining deferrals are appropriately bounded: representation, concrete project requirements, APIs, physical placement, storage, identity encoding, and activation/concurrency/recovery mechanics. The owning section expressly excludes authority, cutover, history, and refusal semantics from that deferral. Broader delivery or migration-cutover references in unchanged deferred sections do not unsettle the specifically defined admission-governance cutover.


### Required changes and final disposition

Required normative corrections: none.

The only subsequent text update is ordinary review bookkeeping: replace the pending-audit status with this completed audit outcome and retain its exact review boundary. That is not a defect in a packet submitted while the audit was pending.

A suitable recorded disposition is:

HR-001/P2-01 — Resolved at semantic level. The accepted admission-governance amendment has completed focused remediation-follow-up review against the submitted delta and current source snapshot. No qualifying P1/P2 findings remain. Implementation realization remains deferred. This outcome does not approve the remaining semantic package or authorize implementation.

Controlling conclusion: the HR-001 amendment is complete. The original authority-establishment/activation/replacement gap is fixed, and no qualifying delta regression remains. P2-02, P2-03, P2-04, the pre-existing guide follow-ups, whole-package approval, and implementation authorization remain outside this closure.

## Exact submitted amendment delta

`````diff
--- before/docs/specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md
+++ after/docs/specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md
@@ -229,6 +229,14 @@
 Assurance selection consumes those exact admitted definitions; it cannot admit
 its own inputs or convert organizational rationale into a waiver.
 
+The shared admission-governance lifecycle establishes the exact project
+configuration, owner-rule basis, activation cutover, and authority provenance for
+those admissions. Assurance governance applies its domain constraints under that
+basis; Assurance Selection cannot initialize or replace governing admission rules.
+A successor configuration neither substitutes definitions into already resolved
+requirements nor automatically revokes their inputs. Required currentness and
+dependency checks still apply, with fresh owner results when required.
+
 This is the cross-boundary invariant shared with the
 [metric/gate record](evidence-and-evaluation.md#metric-gate): **posture may select
 which exact assurance requirements govern a run, but it may never alter the
@@ -629,6 +637,12 @@
 particular, authored capability, retry, partial-completion, or readiness policy
 text cannot become admitted policy through prompt authority or a bare link.
 
+Their admission follows the shared active-at-decision rule. Configuration
+replacement does not rewrite an existing plan, Purpose, readiness result, or
+Prospective Evaluation Basis, and grants no Engine evaluation authority. Each
+consumer must apply the exact dependency/currentness rules for its bound inputs;
+neither automatic re-admission nor automatic continued usability is inferred.
+
 ### Observation-plan, evaluation-purpose, and authorization lifecycle
 
 Evaluation Coordination first forms the exact Observation Plan after exact
--- before/docs/specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md
+++ after/docs/specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md
@@ -1067,6 +1067,16 @@
 meaning.
 
 ### Responsibility map
+
+All definition and policy admissions in this part, including Prospective
+Applicability and Canonical Evidence Set Formation Policies, use the governing
+configuration, owner-rule basis, and active-at-decision boundary established by
+[shared policy admission](objects-and-governance.md#policy-admission). A successor
+configuration does not rewrite earlier admissions, evidence membership,
+applicability, or evaluation truth, and does not substitute new policy definitions
+into a formed purpose or context. Historical admission, current usability, and
+compliance with successor admission requirements remain distinct propositions;
+the exact owner and consumer dependency/currentness rules govern present use.
 
 | Layer | Owns | Must not decide |
 | --- | --- | --- |
--- before/docs/specs/handbook-contract-membrane/semantic-model/objects-and-governance.md
+++ after/docs/specs/handbook-contract-membrane/semantic-model/objects-and-governance.md
@@ -33,7 +33,9 @@
 
 The Closure MUST bind:
 
-- exact candidate content and the exact admission-rule version;
+- exact candidate content, governing project configuration version and semantic-
+  content identity, exact applicable owner admission-rule version and semantic-
+  content identity, and the active-at-decision basis defined below;
 - typed vocabulary, conformance, domain compatibility, proposition roles, and
   exact scope;
 - the complete finite candidate boundary, selection, and accounting;
@@ -61,6 +63,119 @@
 candidate population it decides. A candidate cannot select or recursively admit
 its own admission rule; a changed rule has its own successor rather than an
 unbounded self-admission chain.
+
+### Establishment and replacement of governing admission requirements
+
+Handbook MUST define an exact, versioned establishment rule for project admission
+governance. This Handbook-defined rule and the applicable non-relaxable semantic
+constraints are the starting authority outside the project policy-candidate
+population. Neither an ordinary policy candidate nor a proposed governing
+configuration may select, admit, weaken, or replace the rule that authorizes its
+own establishment. Changing Handbook's establishment rule is not a project
+configuration operation.
+
+An independently attributable **project admission-governance responsibility**
+MUST establish the governing configuration, exact project and admission scope,
+applicable owner-rule bindings, active-version relation, and successor history.
+Domain owners retain validation of their own requirements and admission of
+ordinary policy candidates. This responsibility does not select a physical
+component, service, repository, or new authored artifact family.
+
+In the supported standalone advisory mode, the Handbook establishment rule
+permits an explicit local initialization instruction, including a configured
+agent acting under that instruction, to supply advisory establishment authority.
+Initialization MUST establish that initial setup applies to the exact project,
+validate the exact configuration and owner-rule bindings, and record the accepted
+instruction and the project's replacement-authority basis. Missing, unreadable,
+or unverifiable governance in an established project MUST NOT authorize another
+initialization. Adding a scope in an established project MUST follow its existing
+governance rather than create another bootstrap route.
+
+Initialization and successor activation follow this bounded governance lifecycle,
+not ordinary candidate admission or an action policy that depends on the
+configuration being established. Configuration validation, permission for the
+exact activation, and confirmation of committed active-state change MUST remain
+separately attributable propositions; one owner outcome may carry them without
+requiring separate runtime record types. This lifecycle supplies no authority for
+evaluation, work progression, posture mutation, or other operational effects.
+
+Configuration versions MUST have exact immutable semantic identities. Every
+supported admission MUST resolve one unambiguous governing configuration and
+owner-rule basis for its exact project and scope. Missing, conflicting,
+unsupported, substituted, or unverifiable governance MUST refuse the affected
+operation. Candidate preference, directory proximity, repository order, mutable
+file contents, or a "latest" label MUST NOT select governing authority. No implicit
+scope inheritance, override priority, or configuration merge is established here.
+
+Replacement MUST deliberately bind the exact current predecessor, proposed
+successor, affected scope, and previously established replacement-authority basis.
+It MUST validate the successor against Handbook's fixed constraints and applicable
+domain constraints. The successor cannot supply missing permission for its own
+activation. Where changes to replacement-authority terms are supported, the prior
+terms MUST authorize the change; new terms take effect only after activation.
+Unsupported changes refuse. The same user or agent may author a candidate and
+separately request replacement; these are distinct decisions, not necessarily
+different people. Replacement need not satisfy the predecessor's ordinary
+candidate-admission requirements or make every configurable requirement stricter;
+it cannot relax Handbook's non-relaxable floor.
+
+The authoritative owner MUST establish one activation cutover against the exact
+current predecessor, preserving predecessor-to-successor history. A stale
+predecessor cannot be overwritten by a replacement request. No activation success
+may be reported unless the exact active-state change committed. Authoring,
+editing, saving, or validating inactive content does not activate it. Changed
+semantic content under an existing immutable identity MUST refuse. Exact replay
+returns the earlier outcome without re-executing activation or restoring an old
+active version. An uncertain activation outcome MUST remain uncertain until
+authoritative state is resolved; timeout is not proof of non-commit or permission
+to repeat an effect that might already have committed.
+
+Each new Policy Admission Closure MUST finalize against the exact governing
+configuration and owner-rule basis active at its decision boundary. A change
+before finalization MUST refuse stale-basis finalization; a fresh decision may be
+requested under the newly resolved basis. Completed closures remain exact history
+and may be replayed without becoming new admissions. Failure to establish the
+governing basis MUST preserve available decisive provenance without inventing an
+applicable version or admitting a policy.
+
+Replacement MUST NOT rewrite earlier admission/refusal closures, explanations,
+or authority bases. It governs subsequent admissions but does not by itself
+re-admit, revoke, or establish successor-rule compliance for existing policies.
+Present usability remains subject to definition-owner and consumer currentness,
+compatibility, scope, and declared dependency rules. A required assessment under
+successor requirements produces a distinct owner determination. Existing exact
+requirements, purposes, evidence, and results cannot acquire substituted policy
+definitions merely because a configuration changed.
+
+Establishment, activation, and refusal outcomes MUST preserve the exact Handbook
+establishment rule, project/scope, configuration identity and semantic content,
+owner-rule bindings, predecessor when applicable, accepted instruction and
+authority basis, validation, and activation outcome, insofar as established.
+Refusal MUST identify the missing or violated condition without fabricating
+unresolved inputs. Authority provenance MUST distinguish advisory principal or
+delegation claims from independently verified identity, delegation, and enforcement
+facts. Other propositions, including configuration identity, validation, current
+predecessor, and committed activation, can remain owner-verified. Filesystem
+access, an asserted name, a prompt, or a configured role cannot prove stronger
+authority than the accepted basis supplies. Advisory authority does not waive
+validation, exact scope, currentness, or commit checks.
+
+A Decision Record MAY explain the choice; it is neither mandatory nor an
+activation mechanism. No separate service, login, human reconfirmation, or general
+identity platform is required. Concrete representation, APIs, owner placement,
+storage, activation/concurrency mechanics, and recovery mechanics remain deferred;
+the authority, cutover, history, and refusal semantics above do not.
+
+### Admission-governance examples
+
+| Situation | Required outcome |
+| --- | --- |
+| A local user explicitly initializes project P with configuration C1 and a permitted instruction basis for later replacement. | Handbook applies its fixed establishment rule, validates C1 and its exact scope, and records advisory instruction authority and committed activation. No Decision Record, login, or second approval prompt is inherently required. |
+| A candidate says to use its own admission rule; a successor grants permission needed to install itself. | Neither supplies governing authority. The candidate uses existing governance; successor activation needs permission established beforehand. |
+| A draft C2 is saved, or C1's immutable identity is reused for different content. | Saving the draft leaves it inactive. Conflicting identity reuse refuses. |
+| C2 activates while a candidate is being checked under C1. | A C1 decision completed before cutover remains history; one not yet finalized refuses stale-basis finalization. A fresh C2 decision is distinct. Replaying a completed C1 decision or activation cannot restore C1. |
+| A policy admitted under C1 is considered after C2 replaces it. | Its C1 admission remains history, not proof of C2 compliance. Present use follows the exact owner's and consumer's dependency/currentness rules; replacement alone neither revokes nor re-admits it. |
+| Established governance is unreadable, or activation's commit is uncertain. | Missing state does not authorize reinitialization. An uncertain operation cannot report success or repeat a possibly committed effect before authoritative resolution. |
 
 <a id="ontology"></a>
 ## Ontology and ownership
@@ -122,8 +237,8 @@
 | **Semantic owner** | The bounded-context responsibility that alone establishes, rejects, verifies, or changes the exact proposition represented by an object. Storage, rendering, transport, or orchestration does not transfer this authority. |
 | **Canonical object** | An exact object whose semantic owner recognizes its identity and content as authoritative for one bounded proposition. Canonicality is owner- and proposition-scoped; it does not mean “stored in the generic artifact repository,” “currently applicable,” “globally trusted,” or “safe for every use.” |
 | **Authored canonical project artifact** | Human- or governance-authored project truth governed through the generic Artifact Kind / Artifact Instance lifecycle, including candidate formation, validation, promotion, exact revision, authorized mutation, and lineage. Only the six named families in this record have this status. |
-| **Canonical semantic definition or policy** | An immutable, named, versioned rule or vocabulary definition admitted by the owner whose semantics it controls. It defines how later inputs are resolved, admitted, evaluated, or interpreted; it is not a result of applying itself. |
-| **Policy Admission Closure** | The one immutable owner-produced canonical record that closes admission or refusal of one exact policy candidate under one exact owner admission-rule version. It binds exact candidate content; typed vocabulary and conformance; finite candidate selection and accounting; every relevant exact governance-source revision with proposition role, scope, and historical, formation-only, or continuing-current dependency mode; any separately typed and authorized approval or exception source required by that rule; and all non-relaxable invariant checks. The name **Closure** is selected because one exact category-3 owner result closes both basis and outcome without mandating separate `Policy Admission Basis` and `Policy Admission Result` runtime object types. |
+| **Canonical semantic definition or policy** | An immutable, named, versioned rule or vocabulary definition established by its semantic owner through its native lifecycle. Ordinary project policy candidates require Policy Admission Closure; governing admission configurations use the bounded establishment lifecycle in Shared policy admission, rooted in a Handbook-defined rule outside that candidate population. It defines how later inputs are resolved, admitted, evaluated, or interpreted; it is not a result of applying itself. |
+| **Policy Admission Closure** | The one immutable owner-produced canonical record that closes admission or refusal of one exact policy candidate under one exact owner admission-rule version. It binds exact candidate content; the governing project configuration version, applicable owner admission-rule version, their semantic-content identities, exact scope, and active-at-decision basis; typed vocabulary and conformance; finite candidate selection and accounting; every relevant exact governance-source revision with proposition role, scope, and historical, formation-only, or continuing-current dependency mode; any separately typed and authorized approval or exception source required by that rule; and all non-relaxable invariant checks. The name **Closure** is selected because one exact category-3 owner result closes both basis and outcome without mandating separate `Policy Admission Basis` and `Policy Admission Result` runtime object types. |
 | **Owner-produced canonical record or result** | An immutable determination, closure, verification, admission, disposition, or receipt produced by applying one semantic owner's rules to exact inputs. It remains owned by that domain even when another domain consumes it. |
 | **Durable origin, attempt, or source provenance** | An immutable occurrence record of what an origin owner requested, attempted, observed, reported, applied, refused, or completed. It may be canonical for that bounded occurrence assertion while remaining untrusted or insufficient for a downstream semantic proposition. |
 | **Deterministic projection** | A purpose-, audience-, disclosure-, and definition-bound view of exact source truth. It may select, redact, collapse, render, or assemble while recording its source identities, omissions, lossiness, and output identity. It is authoritative only about that projection operation, not the source proposition. |
@@ -158,7 +273,7 @@
 | Category | Native semantic role |
 | --- | --- |
 | **1. Authored canonical project artifacts** | Governed project truth authored through Handbook's project-artifact lifecycle. |
-| **2. Canonical semantic definitions and policies** | Immutable owner-admitted rules that define later resolution, admission, evaluation, or action semantics. |
+| **2. Canonical semantic definitions and policies** | Immutable owner-established rules that define later resolution, admission, evaluation, or action semantics, including governing configurations established through the bounded shared-admission lifecycle. |
 | **3. Owner-produced canonical records and results** | Immutable owner determinations produced from exact input closures. |
 | **4. Durable origin, attempt, and source provenance** | Retained occurrence truth from requesters, producers, Docks, hosts, or other origins, without implied downstream admission. |
 | **5. Deterministic projections, representations, and assemblies** | Purpose-bound views, encodings, and compositions that preserve source ownership without becoming source truth. |
@@ -212,8 +327,11 @@
   lineage, or pagination primitives MAY be reused across owners. Reusing a
   primitive MUST NOT merge repositories, lifecycle meanings, or authority.
 - All canonical definition and policy candidates follow the
-  [shared policy-admission rule](#policy-admission). Authorship and artifact
-  promotion do not establish executable admission.
+  [shared policy-admission rule](#policy-admission). Ordinary project candidates
+  require Policy Admission Closure; governing configurations instead follow that
+  section's bounded establishment/activation lifecycle under the fixed Handbook
+  rule. That base case is not an alternative admission path for ordinary policies.
+  Authorship and artifact promotion do not establish executable admission.
 - An owner-produced record MUST remain immutable historical truth for its exact
   input closure. Currentness, supersession, or current-use eligibility is a
   separate owner observation or relationship; it MUST NOT rewrite the record.
@@ -249,6 +367,8 @@
 
 | Objects | Ontology category | Semantic owner and boundary |
 | --- | --- | --- |
+| Handbook admission-governance establishment rule; governing project admission configurations | Canonical semantic definitions or policies | Handbook defines the fixed establishment rule outside project candidate admission. Project admission governance establishes exact configurations, scope, owner-rule bindings, and activation under that rule; configuration content does not admit or activate itself. |
+| Admission-governance establishment, activation, and refusal outcomes | Owner-produced canonical records or results | Project admission governance owns exact validation, authority basis, predecessor/cutover, and committed activation or refusal. These outcomes do not create a seventh authored artifact family or grant authority for unrelated effects. |
 | Project Authority / Charter; Project Context; Environment Context; Work Specification; Decision Record; Risk Record | Authored canonical project artifacts | Engine's artifact/profile domain owns generic kind, instance, lifecycle, revision, validation, promotion, mutation, and lineage semantics; project authors and authorized governance supply the content. |
 | Artifact Kind definitions; Artifact Instance Descriptors; profiles; renderer and projection definitions; Context Resolution definitions | Canonical semantic definitions or policies | Engine admits and resolves its system definitions. The descriptor defines an artifact instance; it is not itself that authored artifact. |
 | Contract revisions; claim definitions and inventories; claim-set definitions; Evidence Evaluation Policies; Metric Definitions; Gate Policies | Canonical semantic definitions or policies | Contracts definition authorities own executable meaning and exact resolution. |
@@ -258,7 +378,7 @@
 | Action Kind definitions; Operational Reliance, Local Closeout, Parent Eligibility, Action Authorization, posture-observation, posture-recommendation, workflow, recovery, release, or deployment policies | Canonical semantic definitions or policies | The responsible definition or downstream action domain owns each exact admitted definition. This record selects no universal enforcement owner and no policy becomes admitted merely because a user, agent, artifact, or transport authored it. |
 | Live, unclosed policy-candidate submission | Transient operational state | It is proposed content at an admission boundary. Candidate authorship or transport does not create a category-2 definition. |
 | Origin-closed policy-candidate submission occurrence | Durable origin, attempt, or source provenance | The user, agent, artifact, tool, or adapter origin owns the exact historical assertion that the candidate was submitted. It is not policy admission. |
-| Policy Admission Closure | Owner-produced canonical record or result | The exact definition/admission owner deterministically closes admission or refusal under one exact owner rule version, with typed conformance, finite candidate accounting, relevant exact governance-source roles/scope/currentness, required typed approvals or exceptions, and non-relaxable invariant checks. It is one category-3 closure rather than a required pair of new runtime object types. |
+| Policy Admission Closure | Owner-produced canonical record or result | The exact definition/admission owner deterministically closes admission or refusal under the exact governing configuration and owner rule active at the shared decision boundary, with typed conformance, finite candidate accounting, relevant exact governance-source roles/scope/currentness, required typed approvals or exceptions, and non-relaxable invariant checks. It is one category-3 closure rather than a required pair of new runtime object types. |
 | Private `ContextResolutionEnvelope`, Resolved Profile, exact stack, and Engine authority-admission lineage | Owner-produced canonical records | Engine owns private authority truth and exposes only bounded public projections or opaque resolvable references. |
 | Posture Scope; Resolved Posture Snapshot; posture-transition result and resulting authority state | Owner-produced canonical records or results | Engine posture responsibilities own scoped posture resolution and separately authorized mutation truth. |
 | Exact selection-basis closure; policy-resolution basis; Resolved Assurance Requirements | Owner-produced canonical records or results | Contracts-domain Assurance Selection closes the exact admitted candidate set and selection inputs, owns complete accounting, and produces the exact selection outcome. It does not establish governance admission or currentness for the definitions and mappings it consumes. |
@@ -336,7 +456,8 @@
 | Engine artifact and profile domain | The six Artifact Kind families, Artifact Instance Descriptors, profile conformance, authored artifact lifecycle and repositories, canonical project-artifact reads, and Engine-owned artifact render/projection definitions. | Contract definitions, assurance selection, evaluation readiness, evidence or gate truth, or generic authority over non-artifact owner records. |
 | Engine posture domain | Exact Posture Scope resolution, Resolved Posture Snapshot, snapshot verification, and separately authorized posture transition. | Assurance selection, evidence evaluation, gate truth, or transition authority inferred from a recommendation. |
 | Engine Context Resolution domain | Complete envelope admission, non-expansion, Evaluation Authorization issuance/refusal, private lineage, verification, historical validity, currentness, and supersession. | Requirements, producer capability, readiness, evidence adequacy, gate truth, host enforcement, or downstream action authority. |
-| Definition and policy-admission owners | Exact candidate populations; owner admission-rule versions; deterministic Policy Admission Closures; typed vocabulary/conformance; relevant exact governance-source proposition roles, scopes, and dependency/currentness modes; required typed/authorized approvals or exceptions; non-relaxable invariant checks; admission/refusal, currentness, and supersession. | Treating authorship, prompt/agent role, artifact promotion, bare links, open-ended AI safety judgment, or policy-favorable outcomes as admission; requiring every artifact family; turning authored text into runtime evidence/waiver; or letting a candidate recursively admit its own admission rule. |
+| Project admission governance | Establishment under the fixed Handbook rule; exact configuration/scope and owner-rule bindings; previously authorized successor activation; cutover, active-version relation, history, replay, and truthful authority provenance. Physical placement remains deferred. | Candidate self-authorization, implicit file activation, stale predecessor overwrite, history rewriting, fabricated authentication, or authority for unrelated operational effects. |
+| Definition and policy-admission owners | Exact candidate populations; resolving the active governing configuration and applicable owner-rule basis; deterministic Policy Admission Closures at the shared decision boundary; typed vocabulary/conformance; relevant exact governance-source proposition roles, scopes, and dependency/currentness modes; required typed/authorized approvals or exceptions; non-relaxable invariant checks; admission/refusal, currentness, and supersession. | Treating authorship, prompt/agent role, artifact promotion, bare links, open-ended AI safety judgment, or policy-favorable outcomes as admission; requiring every artifact family; turning authored text into runtime evidence/waiver; or letting a candidate recursively admit its own admission rule. |
 | Contracts-domain assurance governance and exception admission | Admission and currentness of Assurance Policy Selector, Assurance Policy, policy-mapping, candidate-policy, and typed exception-policy definitions through exact Policy Admission Closures; independently attributable admission of exact authorized exception records. | Posture resolution, closing the selection basis, selecting or applying requirements, allowing a selector to authorize its own inputs, evidence admission, or Engine authority. |
 | Contracts-domain Assurance Selection | Exact resolution of one selector and policy over an Engine-owned Resolved Posture Snapshot and other declared admitted inputs; closure of the exact selection basis; application of only admitted exception effects; policy-resolution basis; Resolved Assurance Requirements; complete candidate and selection accounting. | Definition, mapping, candidate-input, policy, or exception admission/currentness; policy or exception invention; exception authorization; posture resolution or mutation; Engine authorization; capability-driven relaxation; evidence admission; or gate reinterpretation. |
 | Contracts-domain Producer Capability Authority | Independent admission and currentness of named/versioned Producer Semantic Capability definitions against Contracts-owned observation roles, scales, dialects, and proof requirements. | Producer self-attestation, Dock ceiling, operational availability, Engine permission, actual reach, evidence sufficiency, or host enforcement. |
@@ -382,7 +503,7 @@
 | Category or subordinate form | Identity and lifecycle rule | Currentness rule |
 | --- | --- | --- |
 | Authored canonical project artifact | Exact kind, instance, project scope, content/revision, and lineage under candidate, validation, promotion, and authorized-mutation rules. | The Engine artifact owner determines which exact revision is current for a named use; historical revisions remain immutable and readable. |
-| Canonical semantic definition or policy | Stable owner/name/version plus exact semantic-content identity; immutable after admission. A changed meaning is another definition identity or version. | The definition owner separately verifies whether an exact definition is admitted and applicable at a named semantic time. |
+| Canonical semantic definition or policy | Stable owner/name/version plus exact semantic-content identity; immutable after owner establishment through the applicable native lifecycle. A changed meaning is another definition identity or version. | The definition owner separately verifies establishment/admission and applicability at a named semantic time. Governing configurations additionally follow the exact active-version and cutover rules in Shared policy admission. |
 | Owner-produced canonical record or result | Exact owner, record kind, bounded proposition, input closure, semantic time, and content identity. Creation is an owner decision, not artifact promotion. | The record remains historical truth. Currentness, verification, supersession, and current-use eligibility are separate relations or observations. |
 | Durable origin, attempt, or source provenance | Exact origin owner, occurrence identity, bound purpose/basis/selection where applicable, semantic time, and reported content. Delivery replay and a new occurrence are distinct according to the origin owner's rules. | Later source validity or downstream eligibility is independently observed; retention does not imply continuing authority or freshness. |
 | Deterministic projection or assembly | Exact source-owner identities, source revisions or records, projection/assembly definition, purpose, audience, disclosure bound, resolution/density where applicable, omissions/lossiness, and output identity. | A projection remains exact historical output for its source closure. A current view requires current source resolution and a new projection when declared inputs change. |
@@ -478,6 +599,10 @@
 
 - the six-category ontology plus the subordinate representation distinction;
 - canonicality as owner- and proposition-scoped rather than repository-wide;
+- the fixed Handbook admission-governance establishment rule, standalone advisory
+  setup, exact scoped configuration and owner-rule bindings, prior-authority
+  successor activation, active-at-decision cutover, and preserved history under
+  [shared policy admission](#policy-admission), without selecting storage or IAM;
 - the exact six authored canonical project-artifact families and `Work
   Specification` as the greenfield work-family vocabulary;
 - the inclusion and exclusion rule for generic Artifact Kind / Artifact
--- before/docs/specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md
+++ after/docs/specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md
@@ -169,6 +169,16 @@
 exact Action Intent unless another independently admitted Action Kind expressly
 targets that record lifecycle.
 
+Admission-governance initialization and successor activation follow the bounded
+governance lifecycle in
+[Shared policy admission](objects-and-governance.md#establishment-and-replacement-of-governing-admission-requirements).
+They do not require a fabricated historical evaluation or a project action policy
+whose admission depends on the configuration being established. Validation,
+authority for the exact activation, expected predecessor/currentness, cutover,
+and committed effect remain separately attributable propositions. This is the
+explicit setup/replacement boundary, not a general exemption for configuration
+changes or other effects from the Operational Reliance and action rules.
+
 ### Operational Reliance Policy and basis formation
 
 Every Action Kind, Operational Reliance Policy, Action Authorization policy, and
@@ -177,6 +187,13 @@
 [shared policy admission](objects-and-governance.md#policy-admission). Authorship
 and organizational rationale neither admit a policy nor authorize a particular
 action; the action chain below retains its own separate owner results.
+
+These admissions bind the governing configuration and owner-rule basis active at
+the shared decision boundary. Replacing that configuration neither grants action
+authority nor changes historical results or existing policy identities. Current
+policy usability remains subject to the exact owner's and consumer's declared
+dependency/currentness rules; neither blanket revocation nor automatic successor-
+rule compliance follows from replacement.
 
 An Operational Reliance Policy MUST declare before result direction is used:
 
--- before/docs/specs/handbook-contract-membrane/semantic-model/README.md
+++ after/docs/specs/handbook-contract-membrane/semantic-model/README.md
@@ -20,6 +20,11 @@
 authorize implementation, choose a physical component or crate, change a phase
 or slice status, close or promote work, or establish a delivery outcome.
 No approval is inferred from this consolidation or from a clean review verdict.
+
+The admission-rule establishment/replacement clarification was accepted for
+incorporation on 2026-09-28 under HR-001. That bounded decision is reflected in
+the four parts; the package as a whole remains proposed, and the other open
+review decisions remain open.
 
 Human-review questions and clarification requests are tracked in the
 [review decision register](review-decision-register.md). Open entries are not
@@ -75,7 +80,7 @@
 | Decision | Practical consequence | Detailed boundary |
 | --- | --- | --- |
 | Canonicality is owner- and proposition-scoped. Authored artifacts, executable definitions, owner results, origin provenance, projections, and transient state have separate roles. | A durable file, accepted artifact, schema-valid payload, or prompt does not become evidence, policy, or action authority merely by being present. | [Ontology](objects-and-governance.md#ontology) and [authored relationships](objects-and-governance.md#authored-relationships). |
-| Users and agents may author policy candidates; the responsible owner deterministically admits or refuses them. | Exact candidate content, vocabulary, finite accounting, relevant governance revisions/roles/scopes/dependency modes, required typed approvals or exceptions, and invariants must close. No general AI judgment of organizational safety or requirement for every artifact family is introduced. | [Shared policy admission](objects-and-governance.md#policy-admission). |
+| Project setup establishes versioned admission requirements under a fixed Handbook rule. Users and agents may propose policies; the responsible owner independently admits or refuses each candidate. | An explicit setup instruction can supply advisory authority. Replacement uses previously established permission, becomes active at a defined point, and preserves earlier decisions. Candidate decisions bind the exact active configuration and owner rules, with complete checks and accounting. Saving a draft does not activate it; a Decision Record, login, or extra approval prompt is not inherently required. | [Shared policy admission](objects-and-governance.md#policy-admission). |
 | Assurance selection precedes capability comparison; posture is a scoped dimension vector. | Limited reach or capability cannot silently weaken requirements. Classification and narrative exceptions do not override typed posture or exception authority. | [Posture and assurance](assurance-and-evaluation-preparation.md#posture-assurance). |
 | Evaluation authorization, capability, operating ceiling, readiness, and host enforcement are distinct. | An exact authorized purpose and attemptable prospective basis are required before collection; final truth closure has its own strict checks. | [Evaluation preparation](assurance-and-evaluation-preparation.md#evaluation-preparation). |
 | Deliberately new occurrences differ from content fingerprints and delivery replay; evidence populations close completely. | A retry, late candidate, conflicting identity reuse, or substituted dependency cannot silently alter an already closed occurrence or evidence set. | [Identity and evidence](evidence-and-evaluation.md#identity-evidence). |
--- before/docs/specs/handbook-contract-membrane/semantic-model/review-decision-register.md
+++ after/docs/specs/handbook-contract-membrane/semantic-model/review-decision-register.md
@@ -29,78 +29,61 @@
 
 ## HR-001 — Authority to establish policy-admission requirements
 
-**Status:** Project-configuration direction selected by the user (2026-09-28);
-Pro review complete. Proposed semantic resolution awaiting acceptance and amendment.
-
-The user accepted explicit, versioned project configuration as the first-version
-direction for establishing admission requirements. A Decision Record may explain
-the choice but is not a mandatory activation or approval mechanism. The focused
-[same-session decision review](../../../research/hcm-admission-rule-authority-pro-decision-review-2026-09-28.md)
-recommends retaining that direction with explicit bootstrap, scope, activation,
-replacement, and truthful authority-provenance boundaries. This direction does not mandate an identity
-platform or additional human confirmation, and does not approve the whole package
-or every refinement proposed during consultation.
-
-**Proposed closing choices, not yet accepted:** A fixed Handbook establishment
-rule permits the initial standalone instruction as advisory authority; an
-attributable governance responsibility establishes scope and active version;
-replacement uses previously established authority rather than permissions supplied
-by the successor. Pro also proposes that admissions finalize against the version
-active at the decision boundary, refusing stale-basis finalization after a change.
-That cutover rule and the bounded governance-activation lifecycle are additional
-semantic choices. Earlier admissions/refusals retain their exact history; present
-policy usability remains governed by owner currentness and dependency rules.
-
-The response and local reconciliation are saved in the linked decision-review
-record. P2-01 can be marked resolved at the semantic level after the closing
-choices are accepted and incorporated consistently. Storage, APIs, and concrete
-mechanisms can remain deferred. No normative amendment or implementation approval
-is inferred from this consultation; the other P2 findings remain open.
-
-The [2026-09-27 comprehensive Pro audit](../../../research/hcm-semantic-final-comprehensive-pro-audit-2026-09-27.md)
-supports this question as P2-01. It recommends a semantic answer or an explicitly
-approved bounded deferral before final approval, with concrete authority/activation
-choices required before dependent implementation. This is a review recommendation,
-not acceptance of a particular setup or a new identity platform.
-
-**Question:** How do the requirements for accepting policies first become
-authoritative, and who or what may establish and replace them? This concerns the
-authority over admission requirements, not authorship or admission of an individual
-policy candidate. It affects principal decision 2 in the package guide.
-
-**Already established:** [Shared policy admission](objects-and-governance.md#policy-admission)
-requires an exact owner admission-rule version, deterministic checks, relevant exact
-governance inputs, and preservation of applicable non-relaxable invariants. The
-admission rule exists outside the candidate population it judges; a candidate cannot
-select or admit its own admission rule. Changed rules have successor versions.
-The other three parts use this shared boundary and add domain constraints.
-
-**Not yet established in the normative parts:** How project configuration
-becomes authoritative for a project or scope; who or what can authorize their
-establishment or replacement; and how an authoritative version becomes active.
-Calling the admission rule a "governance root" identifies an assumed starting point
-but does not specify its establishment. Charter or Decision Record references do
-not themselves answer this: the admission rule already determines which sources
-are relevant. No claim is made that the Charter alone contains these requirements.
-
-**Responsibilities to account for in the answer:** Defining admission requirements;
-establishing their authority and applicable scope; applying them to candidates; and
-authorizing their replacement. This is an analysis checklist, not a decision to
-create four components, four human roles, or a new identity-management platform.
-
-**Why it matters:** Implementers could otherwise assume different sources of
-authority or different rights to change acceptance requirements. Existing versioning
-and anti-self-admission constraints do not settle those choices.
-
-**Proposed disposition and timing:** During semantic review, determine which
-responsibilities and constraints must be explicit now, and whether the remaining
-setup choices can be deferred without changing the approved meaning. Present that
-answer or bounded deferral with the affected decision for human approval. Concrete
-initialization and replacement mechanisms must be selected before implementing the
-dependent admission behavior. No specific person, software owner, setup mechanism,
-or resolution deadline has yet been assigned. The project-configuration direction
-above narrows this question; exact activation and replacement semantics still need
-to be reviewed and incorporated.
+**Status:** Resolved at semantic level; accepted for incorporation on 2026-09-28.
+Implementation realization remains deferred. The narrow amendment is applied;
+its focused Pro completeness audit is pending.
+
+**Question resolved:** How do admission requirements first become authoritative,
+and who or what may establish, activate, and replace them? This concerns authority
+over those requirements, not authorship or admission of an individual candidate.
+
+Following the [focused Pro decision review](../../../research/hcm-admission-rule-authority-pro-decision-review-2026-09-28.md),
+the user accepted this item as specified and requested its narrow incorporation
+into the four parts and decision material, followed by a thorough audit of the
+amendment's completeness. This acceptance is limited to HR-001/P2-01; it is not
+whole-package approval or authorization to implement the proposed system.
+
+**Accepted semantics:**
+
+- A fixed, versioned Handbook establishment rule supplies the initial authority
+  outside the project policy-candidate population. In the supported standalone
+  mode, an explicit local instruction or configured agent acting under it may
+  supply honestly advisory authority. Missing established governance does not
+  authorize reinitialization.
+- A separately attributable project admission-governance responsibility establishes
+  the exact configuration, project/scope, applicable owner rules, active version,
+  and history. Domain owners retain validation and ordinary candidate admission.
+  This does not require a separate service, component, or authored artifact family.
+- Deliberate successor activation uses previously established replacement
+  authority and the exact current predecessor. The successor cannot authorize
+  itself. Saving or validating draft content does not activate it.
+- A new admission finalizes against the governing configuration and owner rules
+  active at its decision boundary; changed governance refuses stale-basis
+  finalization. Completed decisions retain their exact historical basis.
+- Replacement alone neither revokes nor re-admits existing policies or establishes
+  successor-rule compliance. Present usability follows exact owner and consumer
+  currentness, compatibility, scope, and dependency rules.
+- Activation follows a bounded governance lifecycle that avoids circular reliance
+  on project policies it is establishing. Validation, permission, and committed
+  effect remain distinct. Other evaluation and operational boundaries still apply.
+- A Decision Record is optional rationale. No identity platform, login, additional
+  human confirmation, or independent identity claim is required by this decision.
+
+**Where incorporated:** [Shared policy admission](objects-and-governance.md#policy-admission)
+owns establishment/replacement, provenance, cutover, refusals, and examples. Its
+ontology and responsibility map classify configurations and owner outcomes. The
+other three parts inherit the governing basis and preserve their domain boundaries;
+the action part explicitly scopes the setup lifecycle. Principal decision 2 in the
+approval guide summarizes this clarification.
+
+**Remaining implementation choices:** Configuration representation and concrete
+project requirements, APIs, physical owner placement, storage, identity encoding,
+activation/concurrency and recovery mechanics. These choices cannot change the
+accepted authority, scope, cutover, history, or refusal semantics.
+
+**Verification:** Local cross-document review and focused Pro amendment audit;
+the audit outcome will be recorded separately from this human decision. P2-02,
+P2-03, P2-04, whole-package approval, and implementation authorization remain open.
 
 ## HR-002 — Plain-English explanations and practical examples
 
@@ -203,8 +186,8 @@
 explanation/drift concern, not approval of a new authority model or removal of the
 separate authorization result. Any proposal for additional identity or permission
 infrastructure needs its own concrete requirement; it cannot be inferred from the
-word "authorization" alone. HR-001 remains open concerning how admission requirements
-become authoritative.
+word "authorization" alone. HR-001 now records the accepted establishment/replacement boundary; it does not
+change these action-authority limits.
 
 ## HR-004 — Is posture-selected assurance required for every evaluation?
 
--- before/docs/research/hcm-admission-rule-authority-pro-decision-review-2026-09-28.md
+++ after/docs/research/hcm-admission-rule-authority-pro-decision-review-2026-09-28.md
@@ -1,7 +1,7 @@
 # HCM admission-rule authority — focused Pro decision review
 
 Date: 2026-09-28.
-Status: Consultation complete; proposed semantic resolution awaiting acceptance. No normative amendment applied.
+Status: Consultation complete; HR-001 clarification accepted and incorporated on 2026-09-28; focused amendment audit pending.
 Scope: Comprehensive audit P2-01 / human-review register HR-001 only.
 
 Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.
@@ -74,6 +74,22 @@
 
 Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.
 ```
+
+## Subsequent human decision and incorporation — 2026-09-28
+
+After reviewing the recommendation, the user accepted this item as specified and
+requested a narrow patch to the four parts and decision material, followed by a
+thorough Pro audit of the change. The accepted clarification now appears in
+Shared policy admission, its ontology and responsibility map, companion references
+in the other three parts, principal decision 2 in the approval guide, and HR-001.
+
+The acceptance includes the fixed Handbook establishment rule, existing-authority
+successor activation, and active-at-finalization cutover with stale-basis refusal.
+The original review and reconciliation below are retained as historical guidance;
+their statements that acceptance or amendment was pending describe that earlier
+point. This update does not approve the whole package or authorize implementation.
+P2-02, P2-03, and P2-04 remain open. A separate amendment-audit record will hold
+the exact submitted delta, source hashes, validation, and review outcome.
 
 ## Local reconciliation and disposition
 

`````
