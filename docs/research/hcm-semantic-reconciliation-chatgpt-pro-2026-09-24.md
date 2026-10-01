# HCM semantic reconciliation — first Pro exchange and local causal review

Date: 2026-09-24 (America/Indianapolis).

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Scope and provenance

The user requested a first reconciliation through ChatGPT Pro change exchange, a local causal review, and an independent review in a fresh Pro conversation. This pass reconciles seven proposed semantic decisions in place and updates four navigation/context files. It does not consolidate files, approve semantics, authorize implementation, or change phase/slice status.

- First Pro conversation: https://chatgpt.com/c/6ab5b358-8354-83e9-ab69-73d635026ee7
- Model verified in the UI: GPT-5.6 Sol, Pro (5 of 5).
- Repository HEAD: `18d7137db593fb76460094c7d3aaa422e65400eb`.
- Exact starting source bundle fingerprint: `c51bec7b25b672a14ac87a2585c54fb79ce9bbdfb6b0d15937eccf918425e950`.
- The 28-source bundle includes the pre-existing dirty/untracked drafts. HEAD alone is not their text baseline. A disposable alternate Git index exposed only the selected files to the tracked-file bundle builder; the real Git index remained unchanged.
- Applied Pro patch SHA-256: `7fc2c1e8fe52adb9a03ebd3d87bde058ee56a504828afc7d1246f42667c37ebc`.
- Local correction SHA-256: `aa095b991c906ccfac7ee7acead12bc35a9cbb3783968c3822bc7a9bd4a78a6f`.
- Complete delta sent for independent review: `0122ca97e8d52a67151f98ea4e6264f67aee1efb59fda524c0eaa5cd7dca2624`.
- Fresh-review ZIP SHA-256: `66bcc64e1db2a35b0850351f196e150cce343c3740d941f6da1d8715af05e0ab`.

## Local disposition

The returned patch passed exact baseline, allowed-path, and git-apply checks and changed the seven semantic proposals plus README, architecture context, phase/slice map, and seam crosswalk. The Pro patch was applied, then the single local correction below was applied. All seven decisions remain PROPOSED and all work remains uncommitted. The independent review later found one additional contradictory authorization disclaimer; its exact recommended deletion was applied and verified locally. No outstanding qualifying P1/P2 remains from the sequence; see the companion review record for the final disposition.

The return processor's initial fenced-diff extraction truncated the patch at a Markdown code fence inside the diff. That truncated artifact was not applied. Extracting the complete outer diff through its final closing fence produced the exact hash declared by Pro and passed the processor's validation and application steps. No patch semantics were edited during extraction.

The first Pro response's suggested future consolidation grouping is advisory only; no document-count decision was adopted. The fresh independent review and final local disposition are recorded in [the companion review record](hcm-semantic-reconciliation-fresh-pro-review-2026-09-24.md).

## Local causal review

# Local causal review — 2026-09-24

Scope: first Pro reconciliation across the seven PROPOSED semantic decisions and four navigation/context files. Reviewed the complete 11-file delta against the exact pre-task dirty snapshot, with companion definitions and scenarios. This is a documentation semantic review, not implementation verification or human approval.

One P2 Required finding corrected: authored-artifact-evaluation-relationships-semantic-decision.md, Decision Record to definitions, policies, and exceptions. The Pro delta required every referenced target owner to produce Policy Admission Closure, although supported targets include an individual action or exception record. That generalized policy-candidate admission to category-3/4 action/exception objects and could require the wrong authorization result. The local correction confines Policy Admission Closure to definition/policy/selector candidates and explicitly retains separately typed action/exception authorization and admission. See local-remediation.patch. Focused re-read against ontology and operational-reliance definitions: the offending generalization is removed; no new authority shortcut.

No other blocking finding identified locally in the bounded delta. Review traced: strict launch/final-closure Engine verification versus policy-selected reliance-time currentness; atomic eligibility/disposition; transition reliance before intent/authorization/mutation; advisory claims versus verified/enforced facts without a general IAM mandate; deterministic policy admission and exact governance-source roles; occurrence/evidence closure and immutable historical truth; proposed versus implemented ownership; canonical explanation versus disclosure/density. This local conclusion is evidence, not an instruction to agree.

Validation: original Pro patch SHA-256 7fc2c1e8fe52adb9a03ebd3d87bde058ee56a504828afc7d1246f42667c37ebc matched its declaration and passed baseline/path/git-apply validation. Exactly 11 permitted Markdown paths changed. Local links, PROPOSED markers, fences, whitespace, and table shapes checked. No code, completed slice authority, implemented-baseline.md, source research, real Git index, HEAD, or phase status changed. No build/tests run for prose-only work.

Remaining product approval: all seven records stay PROPOSED; no consolidation file count, schema/API, physical placement, concrete policy contents, or implementation authority selected.

## Exact local correction

~~~~~~~~~~~~diff
--- a/docs/specs/handbook-contract-membrane/authored-artifact-evaluation-relationships-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/authored-artifact-evaluation-relationships-semantic-decision.md
@@ -345,10 +345,14 @@
 selector, exception, or action was proposed, admitted, selected, or changed. It
 may also carry policy-candidate content or organizational judgment and may bind
 an already owner-recognized target through a Governance Source Reference. The
-target owner MUST independently establish exact executable semantic content and
-produce its Policy Admission Closure with exact source roles, scope,
-dependency/currentness modes, compatibility, required separately authorized
-approvals or exceptions, and supersession.
+target owner MUST independently establish the target's exact semantics and
+applicable admission or authorization. For a definition, policy, or selector
+candidate, that includes its Policy Admission Closure with exact source roles,
+scope, dependency/currentness modes, compatibility, required separately
+authorized approvals or exceptions, and supersession. A particular exception
+record or action instead follows its own separately typed authorization and
+admission boundaries; referencing it does not turn it into a policy candidate
+or substitute Policy Admission Closure for those owner results.
 
 Decision Record acceptance establishes only the human-decision proposition
 declared by that record. It cannot turn copied prose or a bare link into

~~~~~~~~~~~~

## Submitted reconciliation prompt

~~~~~~~~~~~~text
Perform the FIRST IN-PLACE RECONCILIATION of the seven Handbook Contract Membrane semantic decisions. The attached codex-pro-repo-input ZIP has a UI-added timestamp. Read bundle-manifest.json, worktree.patch, then all seven full current *semantic-decision.md files under repo/docs/specs/handbook-contract-membrane/. Read relevant architecture/current-source evidence and prior reviews, especially their Local reconciliation sections and the operational-reliance review's Companion-document normalization required section.

BASELINE: HEAD 18d7137db593fb76460094c7d3aaa422e65400eb; bundle fingerprint c51bec7b25b672a14ac87a2585c54fb79ce9bbdfb6b0d15937eccf918425e950. Patch the exact CURRENT repo/ bytes, including existing dirty candidates, NOT HEAD. worktree.patch is historical context, not your return patch. Five normally untracked candidates were included through a temporary index and are existing baseline files here. Unbundled paths are unavailable, not grounds to infer contents.

EDIT SCOPE: Only the seven *semantic-decision.md records and, if needed, README.md, architecture-context.md, phase-and-slice-map.md, seam-crosswalk.md in docs/specs/handbook-contract-membrane/. No new/deleted/renamed files. No code, schemas, APIs, implementation plan, or changes to research transcripts, implemented-baseline.md or completed slices. Keep all seven PROPOSED; do not imply human approval or implementation. Greenfield design: current source is evidence, not a compatibility obligation.

RECONCILE:
1. Align authorization and metric/gate rules with operational reliance: historical authorization validity/resolution is universal; continuing-current Engine evaluation authorization is explicitly policy-selected (historical_validity_only / continuing_current). Fresh action-specific authorization remains separate. Update invariants, tables, scenarios, cache/retry rules, diagrams consistently. Do not weaken authorization at collection or final evaluation closure.
2. Eligibility and disposition are facets of ONE atomic Operational Reliance Determination. Disposition References identify that object/facet. Complete ontology and owner maps for reliance/action bindings, refusals, attempts, terminals and state-owner receipts.
3. Add transition-specific Operational Reliance before posture-transition intent, authorization and mutation. Preserve forward-only P_n -> evaluation -> observation/recommendation -> separate authorized transition -> P_n+1.
4. Replace stale 'future decision' deferrals with exact companion references. Work Progression is the PROPOSED semantic owner of local/parent eligibility; physical placement, work hierarchy/state model, policies and implementation remain deferred. Architecture/roadmap updates must describe proposals, never silently approve them.
5. Preserve exact occurrence/frontier/evidence identities, finite population accounting, no substitution, uncertain-commit retry prohibitions, commit-time checks, Engine inward dependency direction and truthful HCM-5.2 history. Reconcile real contradictions; do not redesign implementation.

LATE USER CLARIFICATIONS (not yet explicitly recorded):
A. Handbook must NOT build a general identity/credential/authentication/delegation/capability-token/secrets/PKI/RBAC/ABAC platform. Action Authorization Binding is a semantic result/provenance contract, not a mandate for an internal IAM system. Preserve Engine Context Resolution's semantic admission/verification responsibility.
B. Standalone CLI/SDK workflows may use explicitly permitted ADVISORY authority from user prompts or configured agent roles. Such inputs supply intent, claimed principal/delegation and declared authority; they must not be represented as independently authenticated identity/delegation or host-enforced permissions. Ambient OS permissions are real but not fine-grained Handbook credentials. A real enforcing host may supply stronger facts; Substrate is only a future example. Reconcile this across the action chain, not just pure non-mutating recommendations. Advisory mode cannot fabricate verified facts, satisfy policy-required enforcement, or waive required target/currentness checks. The absolute claim 'prompts cannot provide authority' is wrong for advisory authority.
C. Users/agents can author policy candidates; responsible owners deterministically admit against typed vocabulary, exact references, scope/role completeness, selection/accounting and non-relaxable invariants. 'Unsafe' means mechanically prohibited by admitted constraints, not an AI's open-ended organizational judgment. Governance inputs supply organizational judgment; do not invent a human prompt for every policy.
D. Policy admission must bind relevant exact Charter/Project Context/Decision/Risk revisions, proposition roles, scope, currentness and separately typed/authorized approval/exception sources when required. A bare Markdown link is insufficient proof. Policy Admission Basis/Result were SUGGESTED names, not settled new runtime objects: choose a minimal coherent owner-produced closure within the six-category ontology, explaining the naming. Do not require every artifact family for every policy, turn authored text into runtime evidence/waivers, or create unbounded policy-admission recursion. Preserve historical/formation-only versus continuing-current dependencies.
E. Preserve one canonical evaluation truth, concise default human explanation and requested deeper drill-down to highest available/authorized detail, independent disclosure authority, no reevaluation from display density. Canonical explanation is Contracts truth; density views are projections. Explicitly distinguish settled principles from remaining bounded explanation-projection decisions/planning. No dedicated decision exists yet; do not drop this UX requirement or invent UI layouts, APIs or a fixed four-level schema.

Keep seven records IN PLACE in this pass. Recommend eventual responsibility-based consolidation separately (five was only a hypothesis); no merges now. Do not reduce the task or replace it with an easier alternative. Produce the smallest COMPLETE reconciliation without removing safeguards. Check patch application, status, link/table/scenario consistency and whitespace; distinguish unbundled from broken links and state validation limitations honestly.

RETURN: summarize changes, remaining deferred decisions and consolidation recommendation, then ONE COMPLETE text-only standard unified Git patch in ONE FINAL diff code block. Use a/docs/... and b/docs/... paths, never repo/docs/.... No binary patch, new files, replacement ZIP/files, ellipses or abbreviated hunks. If also attaching the patch, name it codex-pro-return.patch and make it identical to the mandatory full code block. Actually check it against extracted current sources if your tools permit. Returned changes will be validated/applied and causally reviewed locally, then reviewed in a FRESH independent Pro session. Final human semantic approval remains pending.
~~~~~~~~~~~~

## Complete Pro return

~~~~~~~~~~~~markdown
## Reconciliation result

Completed the **first in-place reconciliation** against the exact bundled current bytes, not against repository HEAD and not by replaying `worktree.patch`.

The reconciliation modifies exactly the seven `*semantic-decision.md` records plus `README.md`, `architecture-context.md`, `phase-and-slice-map.md`, and `seam-crosswalk.md`. No files were created, deleted, renamed, or moved. All seven semantic decisions remain explicitly **PROPOSED**, and no language silently authorizes implementation or claims human approval.

The reconciled records now consistently establish:

- Universal resolution and historical-validity verification of the exact Evaluation Authorization Binding, followed by the policy-selected `historical_validity_only` or `continuing_current` mode for present Operational Reliance. Collection-time and final-evaluation authorization remain strict, and fresh action-specific authorization remains separate.
- Current-Use Eligibility and Operational Disposition as inseparable facets of one atomic Operational Reliance Determination. Disposition References identify that Determination and its exact displayed facet.
- A complete reliance/action provenance chain: Operational Reliance Basis, Determination or Reliance Refusal, Action Intent, Action Authorization Binding or Refusal, Action Attempt Identity, Action Terminal, and authoritative state-owner Mutation Receipt.
- Transition-specific Operational Reliance before posture-transition intent, authorization, attempt, and mutation, preserving the forward-only `P_n → evaluation → observation/recommendation → reliance → separately authorized transition → P_n+1` sequence.
- Work Progression as the **proposed semantic owner** of local closeout and parent eligibility, while physical placement, work hierarchy/state model, concrete policies, consumers, mutation mechanisms, and implementation remain deferred.
- Truthful HCM-5.2 history: the current implementation still has Contracts-owned local closeout and no parent-eligibility result or consumer. The proposals do not retroactively change that implementation history.
- A minimal category-3 **Policy Admission Closure**, rather than mandatory new Basis/Result object types. It deterministically closes admission or refusal against exact candidate content, typed vocabulary, finite accounting, relevant governance revisions and proposition roles, scope, dependency/currentness modes, required separately authorized approvals or exceptions, and non-relaxable invariants.
- Policy candidates authored by users, agents, artifacts, or tools as permitted governance inputs, without treating authored text, prompts, agent roles, approval prose, or Markdown links as admitted policy, runtime evidence, waivers, or authenticated authority.
- Action Authorization Binding as a semantic result and provenance contract—not a mandate for a general Handbook identity, credential, authentication, delegation, capability-token, secrets, PKI, RBAC, or ABAC platform.
- Policy-permitted advisory authority for standalone CLI/SDK action paths, including effectful paths, while preserving truthful claimed-versus-verified provenance and every target, currentness, authorization, attempt, and commit-time safeguard.
- One Contracts-owned canonical explanation truth, a concise default human projection, and requested drill-down to the highest available and independently authorized detail, with no density-driven reevaluation or invented fixed UI/API level scheme.
- Exact occurrence, frontier, evidence, population-accounting, retry, commit-time, and inward-dependency invariants without redesigning the implementation.

## Remaining deferred decisions

The records deliberately leave concrete policy contents and mappings, physical component and crate/module placement, work hierarchy and state model, schemas and APIs, persistence and registry mechanics, identity-allocation algorithms, transports, host integration, Substrate adoption, mutation implementations, task slicing, rollout, and external publication unresolved.

Explanation projection also retains bounded future decisions: the exact projection inventory, names, omission contracts, transport shapes, and interaction design remain open. The settled principles are the single canonical truth, concise default explanation, authorized drill-down, independently governed disclosure, and no reevaluation from display density.

## Consolidation recommendation

No records should be merged during this pass. After the fresh independent Pro review and final human semantic approval, consolidation should follow stable responsibility boundaries rather than a predetermined file count; “five” should remain only a discarded hypothesis.

A plausible later structure would group:

1. Ontology, authored-artifact relationships, and deterministic policy admission.
2. Posture/assurance selection plus evaluation authorization, capability, readiness, and prospective formation.
3. Evaluation occurrence, evidence closure, metric/gate truth, and canonical explanation.
4. Operational Reliance, Work Progression, action authorization/execution, receipts, and posture-transition feedback.

The metric/gate material could remain separate if its truth-owner review boundary remains clearer that way. The final count should emerge from responsibility cohesion, independent reviewability, and avoidance of circular ownership—not from a target number.

## Validation

- All 28 manifested bundle entries were checked against the extracted bytes; hashes and byte counts match.
- Manifest HEAD is `18d7137db593fb76460094c7d3aaa422e65400eb`, and the bundle fingerprint matches `c51bec7b25b672a14ac87a2585c54fb79ce9bbdfb6b0d15937eccf918425e950`.
- `git diff --check` passes.
- Exactly 11 permitted files are modified.
- All seven semantic-decision records contain exactly one `Status: PROPOSED`.
- All 44 Markdown tables pass column-consistency checking.
- The complete patch applies cleanly to a fresh reconstruction of the exact bundled current baseline.
- After applying it there, all 11 resulting file byte sequences match the reconciled worktree.
- The patch contains no binary data, new/deleted/renamed files, abbreviated hunks, or `repo/docs/...` paths.
- There are 44 pre-existing relative references whose targets were not bundled. They remain unavailable for inspection rather than being classified as broken. The reconciliation introduces zero new unresolved relative targets.
- Validation was necessarily limited to bundled sources and documentation-level checks; unbundled repository content and code/test execution were unavailable.
- Patch SHA-256: `7fc2c1e8fe52adb9a03ebd3d87bde058ee56a504828afc7d1246f42667c37ebc`.

Final human semantic approval remains pending.

:chatgpt-content-reference{index="0"}[Download codex-pro-return.patch](sandbox:/mnt/data/codex-pro-return.patch)

## Complete unified patch

```diff
diff --git a/docs/specs/handbook-contract-membrane/README.md b/docs/specs/handbook-contract-membrane/README.md
index aa01196..42d5976 100644
--- a/docs/specs/handbook-contract-membrane/README.md
+++ b/docs/specs/handbook-contract-membrane/README.md
@@ -86,8 +86,13 @@ Use the source that owns the question:
 | What is the current program-level authority boundary? | This file. It owns only post-reset authority classification and navigation. |
 | What are the execution units and their lifecycle boundaries? | [`work-unit-lifecycle.md`](work-unit-lifecycle.md) defines slice and packet identity, ownership, completion, and genuine workflow stops. |
 | What verification, review, and closeout apply at those boundaries? | [`review-and-closeout-policy.md`](review-and-closeout-policy.md) defines proportionate evidence, causal review rounds, escalation, and minimal closeout information. |
-| What proposed greenfield vocabulary and responsibility boundary governs metric/evaluation/gate semantics? | [`metric-gate-semantic-decision.md`](metric-gate-semantic-decision.md) records the proposed semantic decision for later human approval and task-level planning; it is not implementation authority. |
-| What proposed greenfield vocabulary and responsibility boundary governs the posture-to-assurance-policy seam? | [`posture-assurance-policy-semantic-decision.md`](posture-assurance-policy-semantic-decision.md) records the proposed semantic decision for later human review; it is not implementation authority, slice authority, or rollout planning. |
+| What proposed greenfield ontology and semantic-owner rules classify definitions, owner results, authored artifacts, occurrences, references, and projections? | [`canonical-object-ontology-and-ownership-semantic-decision.md`](canonical-object-ontology-and-ownership-semantic-decision.md) records the proposed six-category ontology, owner map, policy-admission closure, action provenance, and explanation-projection principles; it is not implementation authority. |
+| What proposed greenfield vocabulary and responsibility boundary governs the posture-to-assurance-policy seam? | [`posture-assurance-policy-semantic-decision.md`](posture-assurance-policy-semantic-decision.md) records the proposed posture, assurance selection, transition-specific reliance, and separate action chain; it is not implementation authority, slice authority, or rollout planning. |
+| What proposed greenfield responsibility boundary governs evaluation authorization, capability, readiness, and Prospective Evaluation Basis formation? | [`evaluation-authorization-capability-and-readiness-semantic-decision.md`](evaluation-authorization-capability-and-readiness-semantic-decision.md) records the proposed Engine/Contracts-domain Evaluation Coordination boundary and currentness rules; it is not implementation authority. |
+| What proposed greenfield identity and closure rules govern evaluation occurrence, evidence frontier, finite accounting, retry, and canonical truth? | [`evaluation-identity-evidence-closure-semantic-decision.md`](evaluation-identity-evidence-closure-semantic-decision.md) records the proposed exact identity and evidence-closure decision; it is not implementation authority. |
+| What proposed greenfield relationships connect authored Charters, contexts, decisions, risks, work specifications, evaluations, reliance, and action provenance? | [`authored-artifact-evaluation-relationships-semantic-decision.md`](authored-artifact-evaluation-relationships-semantic-decision.md) records the proposed relationship and reference rules; it is not implementation authority. |
+| What proposed greenfield vocabulary and responsibility boundary governs metric/evaluation/gate semantics and canonical explanation? | [`metric-gate-semantic-decision.md`](metric-gate-semantic-decision.md) records the proposed semantic decision for later human approval and task-level planning; it is not implementation authority. |
+| What proposed greenfield boundary governs present operational reliance, atomic eligibility/disposition, local closeout, parent eligibility, and the separate action path? | [`operational-reliance-and-work-progression-semantic-decision.md`](operational-reliance-and-work-progression-semantic-decision.md) records the proposed Work Progression and common action-domain semantics; it is not implementation authority and selects no concrete work hierarchy, policy, mutation, or placement. |
 | How is missing phase or slice authority prepared at the correct level? | [`authority-preparation-prompt.md`](authority-preparation-prompt.md) defines separate phase-to-slice and slice-to-packet top-level planning tasks. Each returns to the meta task before the next level. |
 | How is one authorized slice or packet assigned to a top-level task session? | [`task-session-prompt.md`](task-session-prompt.md) provides the copy/paste-ready assignment prompt and compact result presentation. |
 | How does a meta task prepare and continue requested HCM work? | [`meta-task-prompt.md`](meta-task-prompt.md) routes approved phase preparation, requested-slice preparation, whole-slice or sequential packet execution, and one aggregate closeout. |
diff --git a/docs/specs/handbook-contract-membrane/architecture-context.md b/docs/specs/handbook-contract-membrane/architecture-context.md
index 0355d5d..7d28ad1 100644
--- a/docs/specs/handbook-contract-membrane/architecture-context.md
+++ b/docs/specs/handbook-contract-membrane/architecture-context.md
@@ -35,6 +35,7 @@ The labels below describe architecture state, not lifecycle or authorization:
 | Contract definition, claim, applicability, lifecycle, durable use-case owner, outward SDK membrane, and generic consumers | Current | HCM-5.1 Packets 1–6 establish and integrate [`handbook-contracts`](../../../crates/contracts/) inward of SDK with owner-native exact definition, claim, applicability, fingerprint, compatibility, lifecycle-chain, current-head, evaluation-basis, and repository-bound durability semantics. Its fixed repository-relative store owns bounded current-only paging, exact immutable reads, admitted append/transition, replay, and fail-closed recovery. A bounded outward SDK projection closes exactly four operations within the 62-row inventory and the 23-capability/135-root witness while independently re-admitting owner evidence and preserving the sealed Engine authority seam; the existing generic Rust SDK, CLI, and Tauri consumers preserve that owner truth without a contract-specific branch, and the owner has no SDK or transport dependency. |
 | Evidence/gate owner | Current | HCM-5.2 Packets 1–7 implement the `handbook-contracts` identity/policy/run kernel, exact read-only candidate-source normalization through one sealed Engine `contract_evidence_source` admission plus generic source-currentness/Resolution observations, retained-root immutable-all canonical-evidence list/read/idempotent append with bounded atomic recovery, pure closed claim-verdict evaluation, gate/local-closeout composition, the exact five-operation outward SDK membrane, and integrated proof through the unchanged generic SDK/CLI/Tauri consumers. Verdicts and gate results remain non-persisted derived truth. |
 | Process dock | Current | HCM-5.3 Packets 1–5 implement the purpose-named operational dock owner, language-neutral one-shot JSON protocol, portable first-party Rust executor, completed-only candidate source, exact three-operation/11-root closure, and unchanged generic consumers. HCM-5.4 Packets 1–6 add the one reproducibly packaged native-Rust JSON Schema validator manifest, exact offline local-closure observation, two checked-in schema roots, and the real Contracts-owned semantic proof while preserving the same generic consumers and owner direction. Aggregate HCM-5.4 closeout is complete and Phase 5 is exited. |
+| Handbook Contract Membrane greenfield semantic decisions | Target/future | The seven `*semantic-decision.md` records in this directory remain **PROPOSED**. Together they propose the six-category ontology, posture/assurance selection, Contracts-domain Evaluation Coordination, exact evaluation/evidence closure, authored-artifact relationships, metric/gate truth and explanation, and Work Progression/Operational Reliance/action boundaries. They do not alter current source ownership, approve implementation, select physical placement or concrete policies, or change a phase/slice status. |
 | Publication, bridge, and Substrate adoption | External-unknown | No repository-only claim can prove their current external existence, deployment, or conformance. |
 
 The workspace membership and current crate edges are visible in
@@ -100,14 +101,18 @@ The current [`HCM-5.2 authority`](slices/HCM-5.2/README.md) preserves that
 placement: `handbook-contracts` remains the sole canonical evidence/verdict/gate
 and local-closeout owner, SDK maps five reserved operations outward, and a
 future executor supplies operational records plus untrusted candidates through
-an owner-side source boundary. Local closeout is a sibling derived result. A
-future parent-readiness evaluation, if authorized, may consume that exact result
-but must remain a separate fail-closed, non-mutating derived result; local
-closeout cannot imply parent eligibility. The parent domain, subject-to-parent
-binding, policy, owner, and consumer remain unselected, and any actual parent
-mutation is another later authority boundary. This is implemented owner, SDK,
-and generic-consumer architecture through Packet 7; aggregate
-verification/review and the single HCM-5.2 closeout are complete.
+an owner-side source boundary. Local closeout is a sibling derived result. No
+parent-eligibility result, parent consumer, or parent mutation exists in HCM-5.2;
+local closeout cannot imply any of them. The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+assigns the greenfield semantic responsibility for local-closeout and parent
+eligibility to Work Progression, with each as the eligibility facet of its own
+atomic Operational Reliance Determination. That proposal does not change HCM-5.2
+history or select the work hierarchy/state model, parent binding, concrete
+policy, physical component placement, consumer, action authorization, mutation,
+or implementation slice. This is implemented owner, SDK, and generic-consumer
+architecture through Packet 7; aggregate verification/review and the single
+HCM-5.2 closeout are complete.
 
 The current [`HCM-5.3 authority`](slices/HCM-5.3/README.md) remains the closed
 empty-production-registry checkpoint. HCM-5.4 extends it with one exact
@@ -209,22 +214,32 @@ prepared first-path authority.
 ## Gate, dependency composition, and posture ownership
 
 The current HCM-5.2 implementation provides a gate result and separate local-
-closeout result without a parent-promotion field. Local closeout may become an
-input to a later dependency or workflow-composition owner that evaluates whether
-a selected parent is ready to advance. Such eligibility must be a separate,
-fail-closed, non-mutating derived result governed by its own parent binding,
-policy, and currentness checks; local closeout alone can never imply it. Current
-authority has not selected that parent domain, binding, owner, consumer, or
-policy, and it authorizes no parent mutation.
-
-Gate and local-closeout results may also become inputs to later posture policy,
-but gate ownership must not absorb dependency aggregation, posture aggregation,
-policy, recommendation, acknowledgement, hysteresis, promotion, or transition
-authority. Gates answer their bounded evidence question; later dependency,
-posture, or promotion owners decide what those answers mean. Any eventual
-parent-state mutation is separately authorized from eligibility evaluation and
-must retain its own authority, currentness, concurrency, durability, and replay
-boundary.
+closeout result without a parent-promotion field, parent-eligibility result, or
+consumer. The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+places greenfield semantic responsibility for local-closeout and parent
+eligibility in Work Progression. It requires separate action-specific bases and
+atomic Operational Reliance Determinations, with eligibility and disposition as
+inseparable facets; local closeout alone can never imply parent eligibility.
+The proposal deliberately leaves physical placement, the work hierarchy/state
+model, exact parent/child/dependency binding, concrete policies/currentness
+inputs, consumers, and implementation unselected. Current HCM-5.2 authority is
+unchanged and authorizes no parent mutation.
+
+Gate and local-closeout results may also become exact historical inputs to an
+admitted downstream Operational Reliance Policy, including a posture-transition
+policy, but gate ownership must not absorb dependency aggregation, posture
+aggregation, policy, recommendation, acknowledgement, hysteresis, promotion, or
+transition authority. Gates answer their bounded evidence question. One atomic
+Operational Reliance Determination decides present eligibility/disposition for
+an exact action question; a separately owned Action Intent, Action Authorization
+Binding or Refusal, Action Attempt, Action Terminal, and authoritative state-
+owner Mutation Receipt govern any effect. For posture specifically, the proposed
+[posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md)
+keeps the path forward-only: `P_n` to evaluation, observation/recommendation,
+transition-specific reliance, separate authorized mutation, and `P_{n+1}`.
+Neither proposal approves a concrete policy, host, schema, API, or mutation
+mechanism.
 
 This separation prevents an evidence producer from silently becoming the policy
 engine and lets posture policy evolve without changing each gate implementation.
diff --git a/docs/specs/handbook-contract-membrane/authored-artifact-evaluation-relationships-semantic-decision.md b/docs/specs/handbook-contract-membrane/authored-artifact-evaluation-relationships-semantic-decision.md
index fe5c190..7b7bdcd 100644
--- a/docs/specs/handbook-contract-membrane/authored-artifact-evaluation-relationships-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/authored-artifact-evaluation-relationships-semantic-decision.md
@@ -14,9 +14,11 @@ It is a narrow companion to the proposed
 [posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
 [evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
 [evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
-and [metric, evaluation, and gate decision](metric-gate-semantic-decision.md).
+[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
 Those records retain their object classifications, semantic owners, lifecycle
-stage order, authorization, readiness, evidence, truth, and action boundaries.
+stage order, authorization, readiness, evidence, truth, reliance, and action
+boundaries.
 
 This record is semantic authority for human review only. It is not
 implementation authority, a slice or packet specification, a schema or API
@@ -79,7 +81,7 @@ Terms in separate rows are not synonyms.
 | **Declared Input Reference** | A Typed Semantic Reference by which an authored revision nominates an exact artifact, canonical definition or policy, owner-produced record or result, durable provenance object, owner-recognized candidate, or other exact owner object permitted by the consuming definition as relevant to a later named operation. It expresses authored intent only. An unresolved, unadmitted, non-current, incompatible, or refused target does not become usable because it is named. The consuming owner must independently resolve the exact target, obtain every required owner admission and currentness observation, and close it under the consumer's exact role, subject or snapshot, scope, compatibility, multiplicity, and accounting rules. Admission or derivation that creates a different owner identity does not retarget the reference. |
 | **Bound Evaluation Input** | A closure-specific immutable relation by which a named consuming semantic owner accepts one exact Artifact Revision Identity, canonical semantic definition or policy, owner-produced record or result, durable provenance object, or other exact owner object permitted by that closure, together with its exact role, subject or snapshot, scope, compatibility, required admission and currentness observations, and complete accounting, into one named requirements, applicability, plan, Purpose, readiness, final-context, or other evaluation closure. It may be formed from a Declared Input Reference or directly from a consuming definition's declared input rule. It is not a mutable state of the target, does not alter the source artifact, and does not transfer target ownership. |
 | **Outcome Reference** | A Typed Semantic Reference from a later artifact revision to an already closed owner-produced evaluation, metric, gate, explanation, current-use, eligibility, or action result. It records a relationship; it neither reproduces the result nor grants its authority. |
-| **Disposition Reference** | An Outcome Reference specifically to a separately owned operational disposition. It may document how work was treated, but it does not compute the disposition or perform its mutation. |
+| **Disposition Reference** | An Outcome Reference to one exact Operational Reliance Determination and the exact Operational Disposition facet being documented. Eligibility and disposition are facets of that one atomic owner result, not independently formed or current records. The reference may document how work was treated, but it neither computes the Determination nor authorizes or performs a mutation. |
 | **Governance Source Reference** | A Typed Semantic Reference in an artifact revision to an already owner-recognized target, declaring that the source revision supplies rationale, provenance, or source material for that target. A target owner's separately formed input or provenance binding from an owner-produced object back to a pre-existing artifact revision is not a Governance Source Reference authored into the earlier artifact. If the target did not yet have an owner-recognized identity when the artifact revision formed, only that later target-owned binding—or a later artifact revision that names the issued target—can establish the relationship. |
 | **Authored declaration** | Project intent, desired state, rationale, classification, plan, risk proposition, treatment, or other content for which one of the six artifact families is authoritative. It is not an observation merely because it is precise or current. |
 | **Owner-produced state** | A resolved, admitted, observed, verified, evaluated, or disposition proposition established only by its named semantic owner from exact inputs. It may cite an artifact revision without becoming authored artifact content. |
@@ -174,7 +176,10 @@ These uses are distinct. In particular:
   Evaluation Policy, Metric Definition, or Gate Policy unless the responsible
   owner separately admits an exact definition and the Work Specification refers
   to it.
-- An accepted Decision Record does not admit executable policy.
+- An accepted Decision Record does not admit executable policy. A user or agent
+  MAY author policy-candidate content or organizational rationale in a Decision
+  Record, prompt, configured role, or another permitted source, but the
+  responsible semantic owner must independently close admission.
 - An accepted Risk Record does not admit an exception, waive a requirement, or
   authorize operational reliance.
 - A separately named admission policy MAY accept an exact artifact revision as
@@ -182,6 +187,36 @@ These uses are distinct. In particular:
   different identity, owner, proposition, and lifecycle; the artifact does not
   become evidence or runtime truth by implication.
 
+### Authored policy candidates require deterministic owner admission
+
+A user, agent, authored artifact, or tool MAY propose policy content. Candidate
+authorship supplies content, provenance, and organizational judgment; it does
+not produce a canonical semantic definition or policy. The responsible policy
+owner MUST form the exact **Policy Admission Closure** defined by the
+[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md)
+before executable use.
+
+That closure MUST bind the exact candidate; typed vocabulary and conformance;
+finite candidate selection and accounting; every relevant exact Charter,
+Project Context, Decision Record, or Risk Record revision with its proposition
+role, scope, and historical, formation-only, or continuing-current dependency
+mode; and every non-relaxable invariant required by the owner's exact admission
+rule. When that rule requires an approval or exception, the closure MUST
+separately bind the exact typed, authorized source, its authority, scope,
+identity/version or revision as applicable, and currentness. It requires only
+governance sources the rule declares relevant, not every artifact family for
+every policy. A bare Markdown link, accepted prose, candidate author's authority
+claim, or artifact promotion is insufficient proof. Authored text remains
+governance input and MUST NOT be reclassified as runtime evidence, a waiver, or
+an independently authenticated authority fact.
+
+Admission is deterministic rather than an AI's open-ended judgment about what is
+organizationally “safe.” A candidate is mechanically prohibited only when an
+admitted constraint or non-relaxable invariant says so, and the owner refusal
+identifies that rule. The exact admission-rule version is governed outside the
+candidate population it decides, so a policy candidate cannot recursively admit
+its own admission process.
+
 ## Profile instantiation and requiredness
 
 Artifact Kind registration, profile instantiation, and semantic use are three
@@ -241,7 +276,7 @@ Work Specification, Decision Record, or Risk Record are semantically absent.
 | **Project Authority / Charter** | Constitutional project authority, Project Classification, authored posture baseline and dimension values, governance nominations, descriptive constraints, and authorized Charter change provenance. | Exact governance definitions; source material for posture resolution; exact Decision/Risk revisions; separately admitted policy or exception records; other exact authority references declared by its kind. | Resolved Posture Snapshot; Posture Scope resolution; Assurance Policy selection; Resolved Assurance Requirements; Engine evaluation authorization; evidence, metric, gate, disposition, or mutation truth. |
 | **Project Context** | Authored project/domain/product description, system boundaries, ownership, authoritative source references, and known unknowns for its exact project scope. | Exact context inputs to subject/scope resolution, policy selection, plan or purpose formation, grounding, or explanation only when the consuming definition explicitly declares that role and binds the exact revision. | Runtime/system observation; current external state; producer capability or availability; authorization; readiness; evidence; applicability; claim, metric, gate, or action truth. |
 | **Environment Context** | Authored descriptions of named intended or known environments, declared constraints, expected capabilities, authoritative references, and known unknowns for exact environment scopes. | Exact declared environment input to Producer Selection, host-prerequisite definition selection, plan/purpose formation, or explanation when explicitly declared and independently resolved. | Producer Semantic Capability; current availability; actual prerequisite satisfaction; Dock ceiling; Requested or Actual Observed Reach; Host Enforcement Fact; readiness; canonical evidence; or proof that an environment exists now. |
-| **Work Specification** | The objective, bounded scope, non-goals, planned work, acceptance intent, dependencies, and authored work-planning state for one feature, slice, delivery unit, or other exact body of work. Authored work-planning state is distinct from both the Engine-owned Artifact Instance lifecycle and every separately owned operational workflow, closeout, eligibility, promotion, or mutation state. | Exact Contract, claim, requirements, policy, Evaluation Purpose, evaluation result, semantic gate, Current-Use/eligibility result, and action-domain Disposition references in the causally valid revision appropriate to each relation. | Evaluation Purpose or requirements merely by prose; evidence; claim/metric/gate computation; local closeout; parent eligibility; action disposition; completion authority; promotion, deployment, or any mutation outside its authored-artifact lifecycle. |
+| **Work Specification** | The objective, bounded scope, non-goals, planned work, acceptance intent, dependencies, and authored work-planning state for one feature, slice, delivery unit, or other exact body of work. Authored work-planning state is distinct from both the Engine-owned Artifact Instance lifecycle and every separately owned operational workflow, closeout, eligibility, promotion, or mutation state. | Exact Contract, claim, requirements, policy, Evaluation Purpose, evaluation result, semantic gate, Operational Reliance Determination and displayed eligibility/disposition facet, Action Authorization Refusal, Action Terminal, and Mutation Receipt references in the causally valid revision appropriate to each relation. | Evaluation Purpose or requirements merely by prose; evidence; claim/metric/gate computation; local closeout; parent eligibility; action disposition; action authorization; completion authority; promotion, deployment, or any mutation outside its authored-artifact lifecycle. |
 | **Decision Record** | A governed human decision, status, context, rationale, consequences, supersession, participants or authority provenance, and the exact scope of that human decision. | Governance Source References to exact definitions, policies, selectors, exceptions, approvals, artifact revisions, owner results, or action records; exact supersession references to other Decision Record revisions. | Executable policy, exception admission, owner currentness, assurance selection, evaluation truth, action authorization, or mutation merely because its prose is accepted or resembles a rule. |
 | **Risk Record** | A bounded authored risk or uncertainty proposition, owner, treatment intent, review basis, risk-governance status, and acceptance or monitoring rationale for an exact scope. | Exact source observations, canonical evidence, evaluations, gates, Decision Records, typed exception records, policies, treatments, and action results as provenance-bearing references. | Canonical evidence membership; finding or risk measurement; proof of mitigation; prospective applicability; Assurance Policy relaxation; exception/waiver authority; gate reinterpretation; or operational permission. |
 
@@ -293,27 +328,37 @@ revision and every exact definition or owner result it consumes; it cannot bind
 only a record id, path, current head, or summary.
 
 A later Work Specification revision may add Outcome or Disposition References
-to an already closed evaluation, gate, current-use, eligibility, or action
-result. The reference means “this revision records that exact owner outcome.” It
-does not mean the Work Specification computed the result, that a gate mutated
-work, or that an authored `completed` label proves local closeout, parent
-eligibility, release, or deployment. The downstream action owner retains its
-result and mutation authority.
+to an already closed evaluation, gate, Operational Reliance Determination,
+Action Authorization Refusal, Action Terminal, or Mutation Receipt. A
+Disposition Reference identifies the shared Determination and the exact
+Operational Disposition facet being documented; it does not make that facet or
+its paired Current-Use Eligibility facet an independent result. The reference
+means “this revision records that exact owner outcome.” It does not mean the
+Work Specification computed the result, that a gate mutated work, or that an
+authored `completed` label proves local closeout, parent eligibility, release,
+or deployment. The downstream reliance, authorization, executor, and state
+owners retain their respective results and authority.
 
 ### Decision Record to definitions, policies, and exceptions
 
 A Decision Record may explain why a separately owned definition, policy,
 selector, exception, or action was proposed, admitted, selected, or changed. It
-may bind the exact target using a Governance Source Reference. The target owner
-MUST independently establish exact executable semantic content, admission,
-scope, compatibility, currentness, and supersession.
+may also carry policy-candidate content or organizational judgment and may bind
+an already owner-recognized target through a Governance Source Reference. The
+target owner MUST independently establish exact executable semantic content and
+produce its Policy Admission Closure with exact source roles, scope,
+dependency/currentness modes, compatibility, required separately authorized
+approvals or exceptions, and supersession.
 
 Decision Record acceptance establishes only the human-decision proposition
-declared by that record. It cannot turn copied prose into executable policy,
-make an unadmitted policy current, authorize a particular exception, or replace
-an owner result. If the human decision changes executable semantics, a separate
-new definition/policy identity and owner admission are required; neither target
-is mutated through the Decision Record reference.
+declared by that record. It cannot turn copied prose or a bare link into
+executable policy, make an unadmitted policy current, authorize a particular
+exception, replace a required approval source, or substitute for an owner
+result. A policy admission rule may consume exact Decision Record revisions as
+organizational-judgment inputs without requiring a new human prompt for every
+candidate. If the decision changes executable semantics, a separate new
+candidate, definition/policy identity, and owner admission are required;
+neither target is mutated through the Decision Record reference.
 
 ### Risk Record to evidence, assurance, and exceptions
 
@@ -454,7 +499,8 @@ is never rewritten into an earlier input.
 | Contracts Evaluation Coordination | Exact artifact/snapshot input bindings used by applicability, plan, Purpose, readiness, and prospective-basis closure, while preserving occurrence and anti-cycle rules. | Artifact mutation, Engine authorization, evidence or gate truth, or downstream action. |
 | Contracts evidence/evaluation responsibilities | Candidate admission, evidence closure, applicability, claims, metrics, semantic gate, and canonical explanation under exact closed inputs. | Ambient artifact rereads, artifact mutation, risk acceptance, Decision Record execution, or action disposition. |
 | Producer Capability, Dock, producer, and host owners | Their separate capability definitions, ceilings, attempts, source observations, actual reach, prerequisites, terminals, and Host Enforcement Facts. | Promoting Environment Context declarations into runtime facts or altering artifact content by implication. |
-| Downstream action domains | Their exact current-use, closeout, eligibility, reliance, action, and mutation results under separately named policies and authority. | Rewriting artifacts or gate truth by implication, or making an artifact reference the action's authority. |
+| Work Progression and other Operational Reliance owners | Exact finite source-result and target closure, action-specific Policy Admission Closure and policy selection, Operational Reliance Bases, atomic Determinations with paired eligibility/disposition facets, Reliance Refusals, and present-use currentness/replay. Work Progression is the proposed owner for local closeout and parent eligibility. | Rewriting artifacts or gate truth, treating a Disposition Reference as a separate result, issuing action authority, or mutating state. |
+| Action requester, authorization, executor, and state/effect owners | Exact Action Intent; Action Authorization Binding or Refusal with declared advisory, verified, or enforced authority provenance; Action Attempt Identity and Terminal; and authoritative Mutation Receipt for committed state change. | Treating artifact prose or a reference as independently authenticated identity/delegation, fabricating host enforcement, improving reliance eligibility, or reporting mutation success without the state-owner receipt. |
 | SDK, CLI, Tauri, Flow, Pipeline, skills, projections, and transports | Bounded composition, presentation, deterministic views/assemblies, correlation, and transport of exact source and relationship identities. | Resolving ambient references, inventing owner truth, changing relation direction, or treating rendered prose as an exact target. |
 
 Physical colocation does not collapse these responsibilities. A cross-owner
@@ -477,7 +523,7 @@ relation.
 | **Gate pass recorded by a later Work Specification.** Gate result `G1` closes for an occurrence whose Purpose bound `W1`. A later revision `W2` references `G1` and an action-domain closeout disposition. | `W2` records exact owner outcomes. It neither computed `G1` nor proves closeout from `pass`; `W2` cannot be rebound as an input to the completed occurrence. | Work Specification is a work plan and provenance consumer, not a result or action owner. |
 | **Authored completed label without disposition.** A Work Specification author marks an authored planning state `completed`, but no separately owned current-use or closeout result exists. | The artifact revision may be valid authored project truth, but no local closeout, parent eligibility, promotion, release, deployment, or other operational proposition is established. | Authored lifecycle language is not operational authority. |
 | **Decision predates target identity.** Decision Record revision `D1` supplies rationale for a proposed policy, but the policy owner has issued no policy or pre-admission candidate identity. Contracts later admits policy `Q1` and binds `D1` as source provenance. In a comparison case, `D1` names an exact owner-recognized candidate `Qc`, and admission later creates distinct policy identity `Q1`. | `D1` contains no Typed Semantic Reference to the not-yet-issued `Q1` and is not mutated when `Q1` forms. `Q1` may bind `D1` under its owner-produced source closure, and a later Decision Record revision may reference `Q1`. In the comparison case, the original reference remains to `Qc`; the owner's candidate-to-policy relation, not silent retargeting, connects `Qc` and `Q1`. | An authored artifact cannot mint a future owner result; target-owned provenance, candidate identity, and later artifact-authored reference are distinct relations. |
-| **Decision prose resembles policy.** An accepted Decision Record says all security failures block release, but no exact Gate or Enforcement Policy was admitted. | The human decision and rationale remain canonical authored truth. No executable predicate or disposition exists until the responsible owner admits a separate definition, which may cite the Decision Record as provenance. | Governance acceptance does not perform semantic-policy admission. |
+| **Decision prose resembles policy.** An accepted Decision Record says all security failures block release, but the candidate supplies only a Markdown link, no exact target scope or finite candidate accounting, and no exact admission-rule closure. | The human decision and rationale remain canonical authored truth. No executable Gate or Operational Reliance Policy exists until the responsible owner deterministically produces a Policy Admission Closure that binds the exact candidate, relevant governance-source revision and role, scope, dependency modes, required approvals or exceptions, complete accounting, and non-relaxable checks. | Governance acceptance and a bare link do not perform semantic-policy admission or turn authored text into runtime evidence or a waiver. |
 | **Decision references admitted policy.** A later Decision Record revision cites exact admitted policy `Q1` and explains why it was chosen. `Q1` is later superseded by its owner. | The record's historical reference remains exact. It does not keep `Q1` current, supersede it itself, or update to the successor automatically. | A Governance Source Reference preserves rather than transfers target ownership/currentness. |
 | **Risk acceptance offered as waiver.** A Risk Record marks a security risk accepted and is submitted directly to Assurance Selection to omit the related claim. | Selection refuses the shortcut. Only a declared admitted policy input, or an independently authorized and admitted typed exception record with bounded effect, can influence requirements. | Risk governance status is not exception authority. |
 | **Authored Risk Record versus owner-produced risk fact.** Selector `S1` declares exact Risk Record revision `R1` in an authored-risk-classification role. Selector `S2` instead declares exact owner-produced risk fact `F1`, whose source closure cites `R1`. A caller supplies `F1` to `S1`, supplies `R1` to `S2`, or asks either selector to treat `R1`'s accepted status as a waiver. | Each substitution refuses. `S1` may interpret only the authored proposition declared for its role; `S2` may interpret only the independently owner-produced proposition of `F1`. `R1` does not become the owner of `F1`, and neither input creates an exception or prospective applicability proof. | Authored risk governance, owner-produced risk fact, assurance selection, typed exception authority, and prospective applicability are non-substitutable. |
@@ -509,8 +555,16 @@ This record settles:
 - explicit, exact use of Project Context and Environment Context without ambient
   selection or promotion into runtime facts;
 - Work Specification as a feature/slice/work plan that may reference, but cannot
-  compute or own, evaluation and downstream results;
-- Decision Record as human-decision provenance rather than executable policy;
+  compute or own, evaluation, atomic reliance, authorization, terminal, receipt,
+  or downstream mutation results;
+- user-, agent-, artifact-, and tool-authored policy candidates as permitted
+  governance inputs that require deterministic owner-side Policy Admission
+  Closure, exact relevant source roles/scope/dependency modes, complete
+  selection/accounting, and mechanical invariant checks without bare-link proof,
+  authored-text evidence/waiver conversion, open-ended AI judgment, or recursive
+  self-admission;
+- Decision Record as human-decision and organizational-judgment provenance rather
+  than executable policy or per-policy prompt ceremony;
 - Risk Record as authored risk governance rather than evidence, assurance
   relaxation, exception, or operational authority, including non-substitutable
   direct-artifact and owner-produced-risk input modes;
@@ -519,15 +573,24 @@ This record settles:
   current-head substitution; and
 - the within-occurrence acyclic graph and scenario outcomes above.
 
-Deliberately deferred to the focused operational-reliance decision are
-Current-Use Eligibility, Operational Reliance Basis, local closeout, parent
-eligibility or promotion, release/deployment effects, action-specific policy and
-authorization, universal-versus-policy-declared Engine verification for present
-use, and every operational mutation. This record permits exact references to
-those later owner results but decides none of their semantics.
-
-Explanation-density projection and UI presentation are deferred. Also deferred
-to later human-approved task-level planning are concrete artifact fields,
+The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+owns the detailed semantics of Operational Reliance Basis and Determination,
+atomic eligibility/disposition facets, local closeout, parent eligibility,
+policy-selected Engine continuing-current verification, action authorization,
+attempts, terminals, receipts, and mutation boundaries. It proposes Work
+Progression as the semantic owner of local/parent eligibility. This record only
+fixes exact artifact relationships to those objects; it does not approve that
+proposal, select physical placement, define the work hierarchy or state model,
+choose policy content, or authorize implementation or mutation.
+
+The settled explanation principles are one canonical owner truth, a concise
+default human explanation, requested drill-down to the highest available and
+authorized detail, independent disclosure authority, and no reevaluation from
+presentation density. Concrete explanation projections, density choices, UI,
+and API representation remain bounded future decisions; no fixed level count or
+layout is selected. Also deferred to later human-approved task-level planning
+are concrete artifact fields,
 schemas, Rust types, APIs, relationship encodings, registry/resolver mechanics,
 repository and storage layouts, paths, profile changes, cardinality numbers,
 operation names, crate/module placement, transport shapes, indexes, cache
diff --git a/docs/specs/handbook-contract-membrane/canonical-object-ontology-and-ownership-semantic-decision.md b/docs/specs/handbook-contract-membrane/canonical-object-ontology-and-ownership-semantic-decision.md
index 841c289..c75f2d1 100644
--- a/docs/specs/handbook-contract-membrane/canonical-object-ontology-and-ownership-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/canonical-object-ontology-and-ownership-semantic-decision.md
@@ -11,7 +11,10 @@ lifecycle and which remain under dedicated semantic owners.
 It is a narrow companion to the proposed
 [posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
 [evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
-and [metric, evaluation, and gate decision](metric-gate-semantic-decision.md).
+[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
+[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
 Those records retain ownership of their detailed semantics. This record places
 their important objects into one ontology and responsibility map without
 redefining their stage order, algorithms, state machines, or truth rules.
@@ -49,17 +52,25 @@ does not name a seventh family and creates no compatibility requirement.
 Whether a future model discovers a genuinely different artifact role remains a
 new semantic decision rather than an alias inferred here.
 
-This record deliberately does not settle the next three companion subjects:
-
-- evaluation-occurrence identity, evidence-set closure, prospective
-  applicability, or historical resolvability;
-- the exact relationships and fields among the six authored artifact families
-  and evaluation records; or
-- operational reliance, local closeout, parent eligibility, promotion, or
-  mutation authority.
-
-Only the cross-boundary invariants needed to keep those later decisions from
-collapsing the ontology are fixed here.
+This record classifies but does not redefine the detailed semantics fixed by
+three exact companion proposals:
+
+- the proposed
+  [evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md)
+  owns evaluation-occurrence identity, evidence-set formation and closure,
+  prospective applicability, semantic times, and historical resolvability;
+- the proposed
+  [authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md)
+  owns exact relationships among the six authored artifact families and owner
+  definitions, records, results, and provenance; and
+- the proposed
+  [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+  owns present-use reliance, atomic eligibility/disposition, local closeout,
+  parent eligibility, action authorization, attempts, terminals, and mutation
+  receipts.
+
+All remain `PROPOSED`. Their presence supplies exact companion references, not
+human approval, implementation authority, physical placement, or rollout.
 
 ## Canonical vocabulary
 
@@ -71,6 +82,7 @@ The following terms are normative. Terms in separate rows are not synonyms.
 | **Canonical object** | An exact object whose semantic owner recognizes its identity and content as authoritative for one bounded proposition. Canonicality is owner- and proposition-scoped; it does not mean “stored in the generic artifact repository,” “currently applicable,” “globally trusted,” or “safe for every use.” |
 | **Authored canonical project artifact** | Human- or governance-authored project truth governed through the generic Artifact Kind / Artifact Instance lifecycle, including candidate formation, validation, promotion, exact revision, authorized mutation, and lineage. Only the six named families in this record have this status. |
 | **Canonical semantic definition or policy** | An immutable, named, versioned rule or vocabulary definition admitted by the owner whose semantics it controls. It defines how later inputs are resolved, admitted, evaluated, or interpreted; it is not a result of applying itself. |
+| **Policy Admission Closure** | The one immutable owner-produced canonical record that closes admission or refusal of one exact policy candidate under one exact owner admission-rule version. It binds exact candidate content; typed vocabulary and conformance; finite candidate selection and accounting; every relevant exact governance-source revision with proposition role, scope, and historical, formation-only, or continuing-current dependency mode; any separately typed and authorized approval or exception source required by that rule; and all non-relaxable invariant checks. The name **Closure** is selected because one exact category-3 owner result closes both basis and outcome without mandating separate `Policy Admission Basis` and `Policy Admission Result` runtime object types. |
 | **Owner-produced canonical record or result** | An immutable determination, closure, verification, admission, disposition, or receipt produced by applying one semantic owner's rules to exact inputs. It remains owned by that domain even when another domain consumes it. |
 | **Durable origin, attempt, or source provenance** | An immutable occurrence record of what an origin owner requested, attempted, observed, reported, applied, refused, or completed. It may be canonical for that bounded occurrence assertion while remaining untrusted or insufficient for a downstream semantic proposition. |
 | **Deterministic projection** | A purpose-, audience-, disclosure-, and definition-bound view of exact source truth. It may select, redact, collapse, render, or assemble while recording its source identities, omissions, lossiness, and output identity. It is authoritative only about that projection operation, not the source proposition. |
@@ -158,10 +170,28 @@ current private representation.
 - Shared canonicalization, fingerprint, exact-reference, repository-support,
   lineage, or pagination primitives MAY be reused across owners. Reusing a
   primitive MUST NOT merge repositories, lifecycle meanings, or authority.
-- An authored artifact MAY be the human-editable source for a policy,
-  exception, or other owner definition. The semantic owner MUST independently
-  admit an immutable executable definition or record. Artifact promotion alone
-  never performs that admission.
+- A user, agent, authored artifact, or another tool MAY author policy-candidate
+  content. Candidate authorship supplies proposed content, provenance, and
+  organizational judgment only. The semantic owner MUST independently and
+  deterministically produce a Policy Admission Closure before the candidate is a
+  category-2 definition or policy. Artifact promotion, prompt authority, agent
+  role, a bare Markdown link, and human-readable approval prose alone never
+  perform that admission.
+- A Policy Admission Closure MUST bind only the exact governance sources relevant
+  to the candidate's declared admission rule, not every artifact family by
+  default. It MUST bind each consumed Charter, Project Context, Decision Record,
+  or Risk Record revision by exact owner identity and revision, proposition role,
+  scope, and currentness/dependency mode. When the admission rule requires an
+  approval or exception, the closure MUST separately bind the exact typed,
+  authorized source, its authority, scope, identity/version or revision as
+  applicable, and currentness. Authored text remains governance input and MUST
+  NOT be reclassified as runtime evidence or a waiver.
+- Policy admission MUST apply typed vocabulary, complete finite candidate
+  selection/accounting, scope and role completeness, and non-relaxable invariants
+  mechanically. “Unsafe” means prohibited by one of those admitted constraints;
+  it MUST NOT mean an AI's unbounded organizational judgment. The owner admission
+  rule is an exact governance root outside the candidate population it decides,
+  so a candidate cannot select or recursively admit its own admission rule.
 - An owner-produced record MUST remain immutable historical truth for its exact
   input closure. Currentness, supersession, or current-use eligibility is a
   separate owner observation or relationship; it MUST NOT rewrite the record.
@@ -192,7 +222,7 @@ current private representation.
 
 ## Object classification
 
-The table classifies the important objects already named by the three companion
+The table classifies the important objects already named by the companion
 decisions. It does not redefine their detailed fields or behavior.
 
 | Objects | Ontology category | Semantic owner and boundary |
@@ -203,7 +233,10 @@ decisions. It does not redefine their detailed fields or behavior.
 | Assurance Policy Selector; Assurance Policies; policy mappings; typed exception policies | Canonical semantic definitions or policies | Contracts-domain assurance-governance definition authorities own admission and currentness of their executable definitions, mappings, and candidate inputs. Assurance Selection resolves and applies only those admitted inputs; it cannot admit or authorize its own inputs. |
 | Producer Semantic Capability definitions | Canonical semantic definitions or policies | A distinct Producer Capability Authority within the broader Contracts bounded context independently admits the semantic representational capability used for proof-plan compatibility. A producer or Dock cannot self-admit it. |
 | Dock or adapter operating-ceiling definitions and producer retry/partial-completion policies | Canonical semantic definitions or policies | The Dock or adapter owner establishes its bounded operating contract. That contract is not Engine authorization, Producer Semantic Capability, or host enforcement. |
-| Enforcement, reliance, posture-observation, posture-recommendation, workflow, parent-eligibility, release, or deployment policies | Canonical semantic definitions or policies | The downstream action domain owns each named policy. This record selects no universal enforcement owner. |
+| Action Kind definitions; Operational Reliance, Local Closeout, Parent Eligibility, Action Authorization, posture-observation, posture-recommendation, workflow, recovery, release, or deployment policies | Canonical semantic definitions or policies | The responsible definition or downstream action domain owns each exact admitted definition. This record selects no universal enforcement owner and no policy becomes admitted merely because a user, agent, artifact, or transport authored it. |
+| Live, unclosed policy-candidate submission | Transient operational state | It is proposed content at an admission boundary. Candidate authorship or transport does not create a category-2 definition. |
+| Origin-closed policy-candidate submission occurrence | Durable origin, attempt, or source provenance | The user, agent, artifact, tool, or adapter origin owns the exact historical assertion that the candidate was submitted. It is not policy admission. |
+| Policy Admission Closure | Owner-produced canonical record or result | The exact definition/admission owner deterministically closes admission or refusal under one exact owner rule version, with typed conformance, finite candidate accounting, relevant exact governance-source roles/scope/currentness, required typed approvals or exceptions, and non-relaxable invariant checks. It is one category-3 closure rather than a required pair of new runtime object types. |
 | Private `ContextResolutionEnvelope`, Resolved Profile, exact stack, and Engine authority-admission lineage | Owner-produced canonical records | Engine owns private authority truth and exposes only bounded public projections or opaque resolvable references. |
 | Posture Scope; Resolved Posture Snapshot; posture-transition result and resulting authority state | Owner-produced canonical records or results | Engine posture responsibilities own scoped posture resolution and separately authorized mutation truth. |
 | Exact selection-basis closure; policy-resolution basis; Resolved Assurance Requirements | Owner-produced canonical records or results | Contracts-domain Assurance Selection closes the exact admitted candidate set and selection inputs, owns complete accounting, and produces the exact selection outcome. It does not establish governance admission or currentness for the definitions and mappings it consumes. |
@@ -211,7 +244,11 @@ decisions. It does not redefine their detailed fields or behavior.
 | Observation Plan; Producer Selections; Evaluation Purpose, including its bound Evaluation Use Intent; Readiness Basis and Assessment; Readiness Refusal; Attempt Disposition; Prospective Evaluation Basis | Owner-produced canonical records or results | Contracts-domain Evaluation Coordination owns these pre-truth closures and results. |
 | Evaluation Authorization Binding; authorization refusal; Authorization Verification Observation; supersession relation | Owner-produced canonical records or results | Engine alone owns issuance, refusal, verification, historical validity, currentness, and supersession truth. |
 | Canonical evidence record; canonical evidence-set closure; final evaluation context; applicability result; claim evaluation; metric result; semantic gate decision; canonical explanation | Owner-produced canonical records or results | Contracts evidence and evaluation responsibilities own admission, closure, evaluation truth, and complete canonical decision data. |
-| Enforcement or other downstream disposition; current-use or Operational Reliance Basis; action decision; mutation receipt | Owner-produced canonical records or results | The exact downstream action domain owns its present-use determination and effect. A gate remains non-mutating input. |
+| Action Target Snapshot; Operational Reliance Basis; atomic Operational Reliance Determination with Current-Use Eligibility and Operational Disposition facets; Reliance Refusal | Owner-produced canonical records or results | The authoritative target/state owner establishes target truth; Work Progression or another exact downstream action domain owns reliance basis, determination, refusal, currentness, and replay. Eligibility and disposition are facets of one Determination, not independently current records. |
+| Action Authorization Binding; Action Authorization Refusal | Owner-produced canonical records or results | The exact action-authority owner produces the semantic authorization result and provenance closure. A Binding may preserve advisory claims when policy permits, but it is not by itself a credential, authenticated identity, verified delegation, capability token, or host-enforcement proof. |
+| Mutation Receipt | Owner-produced canonical record or result | The authoritative state or effect owner alone proves that one exact authorized Action Attempt committed the exact predecessor-to-successor mutation. |
+| Closed Action Intent occurrence; Action Attempt Identity and durable attempt occurrence; Action Terminal | Durable origin, attempt, or source provenance | The requester owns the exact Intent occurrence; the executor owns attempt identity, attempt occurrence, and Terminal. None substitutes for reliance, authorization, target truth, or a state-owner Mutation Receipt. |
+| Live, unclosed Action Intent request or in-progress Action Attempt | Transient operational state | Requesters, executors, and transports coordinate live work only. Closing durable provenance or an owner result creates a distinct object rather than reclassifying the live state. |
 | Posture Observation and Posture Recommendation | Owner-produced canonical records or results | Their separately named owners establish non-mutating admitted observation and recommendation propositions; neither owns Engine transition authority. |
 | Live, unclosed Evaluation Authorization Request | Transient operational state | The requester states what it asks Engine to admit. An origin-closed request-occurrence record is separate durable provenance, while Engine's Binding or refusal is a separate owner-produced result. Neither reclassifies the live request or makes it authorization. |
 | Origin-closed request- or submission-occurrence record | Durable origin, attempt, or source provenance | The requester, producer, or adapter owns the exact historical assertion that a particular request or submission occurred. It refers to but does not reclassify the live submission, and it is not the receiving owner's admission result. |
@@ -277,14 +314,19 @@ responsibilities, not physical crates, modules, storage engines, or APIs.
 | Engine artifact and profile domain | The six Artifact Kind families, Artifact Instance Descriptors, profile conformance, authored artifact lifecycle and repositories, canonical project-artifact reads, and Engine-owned artifact render/projection definitions. | Contract definitions, assurance selection, evaluation readiness, evidence or gate truth, or generic authority over non-artifact owner records. |
 | Engine posture domain | Exact Posture Scope resolution, Resolved Posture Snapshot, snapshot verification, and separately authorized posture transition. | Assurance selection, evidence evaluation, gate truth, or transition authority inferred from a recommendation. |
 | Engine Context Resolution domain | Complete envelope admission, non-expansion, Evaluation Authorization issuance/refusal, private lineage, verification, historical validity, currentness, and supersession. | Requirements, producer capability, readiness, evidence adequacy, gate truth, host enforcement, or downstream action authority. |
-| Contracts-domain assurance governance and exception admission | Admission and currentness of Assurance Policy Selector, Assurance Policy, policy-mapping, candidate-policy, and typed exception-policy definitions; independently attributable admission of exact authorized exception records. | Posture resolution, closing the selection basis, selecting or applying requirements, allowing a selector to authorize its own inputs, evidence admission, or Engine authority. |
+| Definition and policy-admission owners | Exact candidate populations; owner admission-rule versions; deterministic Policy Admission Closures; typed vocabulary/conformance; relevant exact governance-source proposition roles, scopes, and dependency/currentness modes; required typed/authorized approvals or exceptions; non-relaxable invariant checks; admission/refusal, currentness, and supersession. | Treating authorship, prompt/agent role, artifact promotion, bare links, open-ended AI safety judgment, or policy-favorable outcomes as admission; requiring every artifact family; turning authored text into runtime evidence/waiver; or letting a candidate recursively admit its own admission rule. |
+| Contracts-domain assurance governance and exception admission | Admission and currentness of Assurance Policy Selector, Assurance Policy, policy-mapping, candidate-policy, and typed exception-policy definitions through exact Policy Admission Closures; independently attributable admission of exact authorized exception records. | Posture resolution, closing the selection basis, selecting or applying requirements, allowing a selector to authorize its own inputs, evidence admission, or Engine authority. |
 | Contracts-domain Assurance Selection | Exact resolution of one selector and policy over an Engine-owned Resolved Posture Snapshot and other declared admitted inputs; closure of the exact selection basis; application of only admitted exception effects; policy-resolution basis; Resolved Assurance Requirements; complete candidate and selection accounting. | Definition, mapping, candidate-input, policy, or exception admission/currentness; policy or exception invention; exception authorization; posture resolution or mutation; Engine authorization; capability-driven relaxation; evidence admission; or gate reinterpretation. |
 | Contracts-domain Producer Capability Authority | Independent admission and currentness of named/versioned Producer Semantic Capability definitions against Contracts-owned observation roles, scales, dialects, and proof requirements. | Producer self-attestation, Dock ceiling, operational availability, Engine permission, actual reach, evidence sufficiency, or host enforcement. |
 | Contracts-domain Evaluation Coordination | Observation Plan and Producer Selection closure, Evaluation Purpose, Readiness Basis and Assessment, Readiness Refusal, Attempt Disposition, and Prospective Evaluation Basis. | Engine authorization issuance, capability self-attestation, evidence admission, claim/metric/gate truth, or host launch by implication. |
 | Contracts definition, evidence, and evaluation responsibilities | Contract/claim/evidence-policy/metric/gate definitions; candidate admission; canonical evidence; evidence-set and final-context closure; applicability, claim, metric, gate, and canonical-explanation truth. | Authored artifact mutation, Engine authorization, Dock operation, host enforcement, or downstream action. |
 | Dock and adapter domains | Their manifests, operating-ceiling definitions, protocols, exact invocation, attempt terminals, source-value preservation, and candidate construction. | Producer Semantic Capability admission, Engine authorization, readiness, canonical evidence, or semantic gate truth. |
 | Producer and enforcing host domains | Producers own exact attempt observations and Actual Observed Reach; the actual host owns exact applied/refused Host Enforcement Facts. | Canonical evidence, Engine authority, proof requirements, readiness reclassification, or invented enforcement. |
-| Downstream action domains | Their named policies, reliance/disposition records, eligibility decisions, fresh action authority, and separately authorized mutations. | Rewriting gate truth, inheriting Engine evaluation authority, or a universal enforcement policy by default. |
+| Work Progression and other Operational Reliance domains | Their exact Action Kinds and admitted policies; action-target closure for their decisions; Operational Reliance Occurrences, Bases, atomic Determinations with eligibility/disposition facets, Reliance Refusals, currentness, and replay. | Rewriting gate truth, inheriting Engine evaluation authority, independently authorizing an action, mutating state, or creating a universal enforcement policy by default. |
+| Action requester or origin owner | Exact Action Intent content, claimed principal/delegation/authority provenance, and durable request occurrence. | Target truth, Operational Reliance, authorization, attempt, or effect merely because it requested one. |
+| Action-authority owner | Exact authorization rule and declared advisory/verified/enforced authority basis; principal/delegation provenance; Action Authorization Binding or Refusal; currentness, supersession, and permitted-attempt rule. | Improving eligibility, fabricating authentication/delegation/enforcement, requiring a general Handbook IAM platform, proving an attempt, or claiming a committed effect. |
+| Action executor | Exact Action Attempt Identity, attempt-admission checks, execution, Action Terminal, delivery replay, failure, and effect/commit uncertainty. | Reconstructing another owner's authority/currentness, broadening the effect, retrying an uncertain non-idempotent effect without the admitted recovery conditions, or asserting commit without a state-owner receipt. |
+| Authoritative state or effect owner | Exact target snapshots and expected predecessor; through-commit verification; atomic effect or state transition; durability, concurrency, exact successor, commit semantic time, and Mutation Receipt. | Inventing eligibility, broadening authorization, changing policy, or reporting an uncertain commit as success. |
 | Flow | Deterministic packet assembly and its selection, omission, ordering, compaction, and budget accounting. | Any included artifact, posture, authorization, evidence, metric, gate, or disposition truth. |
 | Pipeline | Pipeline capture, compile, handoff, lifecycle, and separately authorized Pipeline actions. | Contracts truth, generic parent eligibility, posture policy, or mutation merely because a gate passed. |
 | SDK | Transport-free composition and owner-defined public projections. | Alternate semantic branches, owner verification, readiness recomputation, or independent DTO meaning. |
@@ -325,10 +367,13 @@ Only the distinctions necessary to preserve the ontology are settled here.
 | Representation | Exact encoding/schema version and interface-local framing when needed; it carries rather than replaces the semantic identity. | Transport availability or schema conformance is independent of source currentness and semantic admissibility. |
 | Transient operational state | Operation-local identity only as needed for coordination. | It expires with the operation unless an owner closes a separate durable provenance record. |
 
-This record does not choose the exact occurrence identity for deliberately new
-evaluations, evidence-set membership or closure, prospective applicability
-basis, semantic-time retention contract, or reliance verification basis. Those
-belong to the later focused decisions.
+The proposed
+[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md)
+owns exact occurrence identity, evidence-set membership and closure, prospective
+applicability, and semantic-time/historical-resolvability rules. The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+owns the present-use verification basis. This ontology only classifies their
+objects, and neither proposal is human approval or implementation authority.
 
 ## Projection and representation rules
 
@@ -356,12 +401,20 @@ and representation identity MUST remain distinct and MUST both preserve the
 exact source-owner attribution.
 
 The full **canonical explanation** produced with Contracts claim, metric, and
-gate truth is owner-produced canonical decision data. It is not a projection.
-An **explanation-density view** is a deterministic projection of that complete
-explanation. It may make the same result more ingestible, but it MUST preserve
-the exact result identity, semantic status, decisive reason, owner, and any
-mandatory next-state distinctions. It MUST NOT change evaluation inputs,
-truth, authority, or current-use eligibility.
+gate truth is owner-produced canonical decision data and one canonical source of
+evaluation truth. It is not a projection. A concise default human explanation
+and any requested deeper drill-down are deterministic explanation-density views
+of that same source. Each MUST preserve the exact result identity, semantic
+status, decisive reason, owner, and mandatory next-state distinctions; bind its
+own disclosure authority and omissions; and expose no more than the highest
+source detail both available and authorized for that viewer. Denied or
+unavailable detail is a projection/disclosure outcome, not reevaluation.
+
+Display density MUST NOT change inputs, rerun evaluation, create another gate,
+or alter truth, authority, or current-use eligibility. These principles are
+settled here. The exact number, names, omission contracts, transport shapes, and
+interaction design of explanation projections remain bounded later decisions;
+this record selects no fixed level count, UI layout, or API.
 
 Context Resolution authorization, Grounding Projection Resolution, viewer
 disclosure authority, posture strictness, producer capability, and Explanation
@@ -383,12 +436,12 @@ source and projection identities unless an owner explicitly binds it.
 | Scenario and inputs | Expected semantic outcome | Invariant proved |
 | --- | --- | --- |
 | **Promoted Charter and resolved posture.** A Charter revision is promoted through the project-artifact lifecycle. Engine resolves a Posture Scope and produces a Resolved Posture Snapshot. | The Charter remains an authored canonical artifact. The snapshot is a distinct Engine-produced canonical record binding that revision; it is not another Artifact Instance and cannot be edited or promoted as one. | Authored source and owner derivation have different identities and lifecycles. |
-| **Human-authored policy source.** A Decision Record contains approved prose and a policy document is stored beside project artifacts. Contracts has not admitted an exact executable policy definition. | The prose and file may be authored project provenance, but no Assurance, Metric, Gate, or other semantic policy exists for execution until its owner admits an exact definition. | Artifact promotion and human approval do not substitute for executable owner admission. |
+| **User- or agent-authored policy candidate.** A Decision Record contains approved rationale and a user or agent submits candidate policy text stored beside project artifacts. The submission cites the record only by a Markdown link, omits exact proposition role/scope/currentness, and has no Policy Admission Closure. | The artifact and candidate may be governance provenance, but no Assurance, Metric, Gate, Operational Reliance, or Action Authorization policy exists for execution. The responsible owner must deterministically close admission or refusal against exact candidate content, typed rules, relevant exact governance revisions and dependency modes, required typed approvals/exceptions, finite candidate accounting, and non-relaxable invariants. | Candidate authorship and organizational judgment are permitted; artifact promotion, bare links, prompts, and agent roles do not substitute for executable owner admission. |
 | **Selector receives an unadmitted exception.** An authorized actor authored an exact exception source, but the distinct exception-admission responsibility has not established policy conformance, scope, identity, and currentness. | Assurance Selection refuses the input and cannot admit, broaden, or self-authorize it. Only a separate admission record may supply a bounded effect. | Record authorization, admission, and selection-time application remain distinct. |
 | **Durable failed attempt.** A Dock retains an exact timeout terminal and the producer retains a partial source observation. | Both may be durable occurrence provenance and canonical for what their origin owners reported. Neither is an authored artifact, canonical evidence, readiness result, or gate truth. | Durability and bounded canonicality do not imply artifact status or downstream admission. |
 | **Self-described capability.** A producer manifest claims support for a role and a Dock ceiling permits the request, but no independent Producer Semantic Capability definition has been admitted. | Evaluation Coordination cannot treat the producer as semantically capable. The manifest and ceiling remain Dock/producer facts and cannot create the missing Contracts-domain capability authority. | Producer self-description and operating ceiling cannot substitute for admitted semantic capability. |
 | **Conformant artifacts but unknown evaluation readiness.** Engine produces a successful Profile Conformance Assessment for the selected profile and its required project-artifact instances, but a required Producer Selection has a valid current capability fact that leaves a necessary semantic limit unknown. The Evaluation Purpose is decision-seeking. | Artifact/profile conformance remains successful. Evaluation Readiness is separately `unknown`, the Attempt Disposition is `held`, and no Prospective Evaluation Basis or launch follows. | Artifact/profile conformance does not imply Evaluation Readiness or attemptability. |
-| **Canonical explanation at lower density.** Contracts closes a gate result with complete canonical explanation. A UI requests a concise view. | The complete explanation remains canonical decision data. The concise output is a projection that binds its source, density, disclosure, decisive status and reason, and omissions; it cannot alter the gate or become the only retained basis. | Explanation truth and presentation density are separate. |
+| **Concise explanation and authorized drill-down.** Contracts closes one gate result with complete canonical explanation. The default surface requests a concise human explanation; the user then requests deeper detail, while one source section is unavailable or outside the viewer's disclosure authority. | Both responses are projections of the same canonical explanation. Each binds source, density, disclosure, decisive status/reason, and omissions; the drill-down reaches the highest available and authorized detail and reports the bounded omission without reevaluating. Neither view alters the gate or becomes the only retained basis. | Canonical explanation truth, projection density, source availability, and disclosure authority are independent. |
 | **Flow packet under a budget.** Flow includes a Charter rendering, a grounding projection, and a gate-summary projection, then omits lower-priority detail to fit a budget. | Flow owns the packet identity and omission/budget accounting. Engine and Contracts retain authority for each included source; the packet is not a replacement artifact or evaluation record. | Assembly authority is limited to assembly decisions. |
 | **Transport round trip.** A valid Tauri payload containing a projected gate result is captured and later submitted to another consumer. | Schema validity and byte equality prove only representation integrity. The consumer must resolve and verify the referenced Contracts result or admit the payload through a declared source policy; the payload itself is not Contracts truth. | Representation does not transfer owner authority. |
 | **Cross-owner search.** A user searches one catalog and sees a Work Specification, Evaluation Authorization Binding, Producer Attempt, gate result, and Flow packet. | The catalog preserves each object's category, owner, identity, and relationship. Search inclusion does not make the four non-artifact objects Artifact Instances or the catalog their owner. | Unified discovery does not require a universal repository. |
@@ -410,8 +463,9 @@ This record settles:
 - dedicated semantic-owner repositories with optional shared low-level
   primitives when durability is required, without requiring every canonical
   result to be persisted, plus a non-authoritative cross-owner catalog;
-- the classifications of the important objects named across the three
-  companion decisions;
+- the classifications of the important objects named across all companion
+  decisions, including Policy Admission Closure and the complete Operational
+  Reliance/action chain;
 - Engine ownership inward of Contracts and all semantic owners inward of SDK,
   CLI, Tauri, skills, and other transports;
 - assurance governance and exception admission, Assurance Selection, Producer
@@ -422,34 +476,39 @@ This record settles:
 - independent admission of Producer Semantic Capability from producer
   self-description, Dock ceiling, availability, actual reach, and host
   enforcement;
-- projection, assembly, representation, and canonical-explanation boundaries;
-  and
+- deterministic policy-candidate admission as one owner-produced Policy
+  Admission Closure without a mandatory Basis/Result pair, every-artifact
+  requirement, authored-text waiver, open-ended AI safety judgment, or recursive
+  self-admission;
+- projection, assembly, representation, and canonical-explanation boundaries,
+  including one canonical truth, concise default explanation, requested
+  authorized drill-down, independent disclosure, and no density-driven
+  reevaluation; and
 - the category-specific identity, lifecycle, persistence, and currentness
   distinctions required to prevent authority collapse.
 
-Deliberately deferred to a later focused **evaluation identity and closure**
-decision are evaluation-occurrence identity versus content fingerprints,
-evidence-set formation and closure, prospective applicability, exact semantic
-times, and minimum historical resolvability. This record does not decide those
-subjects by implication.
-
-Deliberately deferred to a later focused **authored-artifact relationships**
-decision are exact fields, references, requiredness, profile instances,
-subject/snapshot binding, and input/result relationships for Charter, Project
-Context, Environment Context, Work Specification, Decision Record, and Risk
-Record. This record classifies those artifacts but does not redesign them.
-
-Deliberately deferred to a later focused **operational reliance** decision are
-Current-Use Eligibility, action-specific Operational Reliance Basis, local
-closeout, parent eligibility, promotion, release/deployment effects, the
-universal-versus-policy-declared Engine verification rule, and all mutation
-authority. A semantic gate remains pure and non-mutating.
+Detailed evaluation-occurrence identity, evidence closure, prospective
+applicability, semantic time, and historical resolvability remain owned by the
+proposed
+[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md).
+Detailed authored-artifact fields, references, requiredness, profile instances,
+subject/snapshot binding, and input/result relations remain owned by the proposed
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md).
+Detailed present-use reliance, local and parent eligibility, explicit
+`historical_validity_only` versus `continuing_current` Engine verification,
+action authorization, attempts, terminals, and receipts remain owned by the
+proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
+This record classifies those objects without redesigning them, approving those
+proposals, selecting physical placement, or authorizing implementation. A
+semantic gate remains pure and non-mutating.
 
 Also deferred to later human-approved task-level planning are concrete schemas,
 types, APIs, operation names, storage layouts, repository implementations,
 catalog mechanics, crate/module placement, transports, cache implementations,
 schema generation, task slices, acceptance planning, and rollout. Concrete
-policy mappings, metrics, thresholds, producer selections, UI layout,
+policy mappings, metrics, thresholds, producer selections, policy-admission
+encodings, explanation-projection levels and interaction design, UI layout,
 publication, and external host or Substrate adoption remain owned elsewhere or
 deferred.
 
diff --git a/docs/specs/handbook-contract-membrane/evaluation-authorization-capability-and-readiness-semantic-decision.md b/docs/specs/handbook-contract-membrane/evaluation-authorization-capability-and-readiness-semantic-decision.md
index 17146cb..a182bb6 100644
--- a/docs/specs/handbook-contract-membrane/evaluation-authorization-capability-and-readiness-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/evaluation-authorization-capability-and-readiness-semantic-decision.md
@@ -9,11 +9,17 @@ evaluation formation, and the currentness boundaries for collection, new
 evaluation truth, and operational reliance.
 
 It is a narrow companion to the proposed
-[metric/evaluation/gate decision](metric-gate-semantic-decision.md) and
-[posture/assurance-policy decision](posture-assurance-policy-semantic-decision.md).
-Those records own assurance selection and evaluation truth. This record owns
-neither, and it is not implementation authority until a human approves it and
-later task-level planning separately authorizes delivery.
+[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
+[posture/assurance-policy decision](posture-assurance-policy-semantic-decision.md),
+[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
+[metric/evaluation/gate decision](metric-gate-semantic-decision.md), and
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
+Those records own object classification, assurance selection, exact occurrence
+and evidence closure, authored relationships, evaluation truth, and downstream
+reliance/action semantics. This record owns none of those detailed domains, and
+it is not implementation authority until a human approves it and later task-
+level planning separately authorizes delivery.
 
 Current source and completed HCM authorities are evidence for owner direction
 and trust boundaries only. Current DTOs, crate placement, the first-party Dock
@@ -40,7 +46,8 @@ resolved assurance requirements
   -> exact canonical evidence-set identity
   -> final Contracts evaluation-context closure
   -> claim / metric / semantic-gate truth
-  -> separately owned operational reliance
+  -> action-specific Operational Reliance Basis and atomic Determination, if used now
+  -> separate Action Intent / authorization / attempt / terminal / receipt, if an effect is requested
 ```
 
 The following terms are normative and non-substitutable:
@@ -65,7 +72,7 @@ The following terms are normative and non-substitutable:
 | **Attempt Disposition** | The evaluation-coordination result `attemptable`, `held`, or `refused`. A valid Readiness Assessment plus Evaluation Use Intent produces `attemptable` or `held` under the closed table below. A failed authorization or readiness-formation boundary always produces `refused`. It remains advisory until an enforcing host actually honors it. |
 | **Prospective Evaluation Basis** | The immutable, exactly identified pre-observation closure formed only for an `attemptable` disposition. It binds the Evaluation Purpose, complete Observation Plan, Evaluation Authorization Binding, launch-check verification, Resolved Assurance Requirements, Readiness Basis and Assessment, Attempt Disposition, every Producer Selection and operating fact, use intent, and required currentness provenance. It contains no evidence-set identity and establishes no claim, metric, gate, or downstream truth. |
 | **Producer Attempt** | One exactly identified launch or collection attempt under one Prospective Evaluation Basis and exactly one Producer Selection. A non-idempotent retry is another attempt for that same selection even if it uses the same basis. |
-| **Operational reliance** | A separately owned present-tense use of a closed result to allow, block, hold, promote, mutate, release, deploy, or otherwise affect work. Historical display and exact replay are not operational reliance. |
+| **Operational reliance** | A separately owned present-tense use of a closed result under one action-specific Operational Reliance Basis to produce one atomic Operational Reliance Determination with paired Current-Use Eligibility and Operational Disposition facets. Historical display and exact replay are not operational reliance; the Determination grants no action authority or mutation. |
 
 `ready` requires positive proof from valid, admitted, current facts that, for
 every required role and exact Producer Selection: the Engine-authorized and
@@ -135,6 +142,16 @@ Authorization Request and Engine binding, fresh readiness closure, and a fresh
 Prospective Evaluation Basis. Neither an older purpose nor an older binding can
 authorize a changed plan that happens to target the same subject or coordinates.
 
+Every canonical definition or policy consumed by this record remains subject to
+its semantic owner's deterministic **Policy Admission Closure** under the
+[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md).
+A user, agent, artifact, or tool may author a candidate, but candidate text,
+prompt authority, a bare link, or owner-unverified organizational judgment is
+not an admitted capability, retry, partial-completion, readiness, or other
+policy. Admission binds only the exact relevant governance revisions and roles,
+complete candidate accounting, and non-relaxable invariant checks; it does not
+turn authored text into runtime evidence or require recursive policy admission.
+
 ## Observation-plan, evaluation-purpose, and authorization lifecycle
 
 Evaluation Coordination first forms the exact Observation Plan after exact
@@ -260,11 +277,22 @@ an immutable artifact. Three checkpoints are mandatory:
    Prospective Evaluation Basis, authorization verification, and every other
    context dependency. Missing, mismatched, not-current, superseded, or
    unverifiable authorization refuses new claim, metric, and gate truth.
-3. **Before operational reliance.** The downstream policy owner MUST obtain the
-   exact current owner verifications it declares, including fresh Engine
-   authorization verification. If authorization, requirements, subject,
-   evidence, or another reliance dependency changed, the consumer requires a
-   fresh evaluation rather than treating historical truth as current authority.
+3. **Before operational reliance.** The downstream owner MUST resolve the
+   exact Evaluation Authorization Binding and obtain the Engine historical-
+   validity outcome needed to establish that the closed result was validly
+   formed. That historical resolution is universal. The admitted Operational
+   Reliance Policy MUST then declare exactly one present-use mode for that
+   binding: `historical_validity_only` or `continuing_current`. Only
+   `continuing_current` requires a fresh positive Engine current verification at
+   the reliance semantic time. `historical_validity_only` makes current
+   evaluation-authorization status irrelevant to that Determination while every
+   other policy-declared result, target, subject/scope, work-structure, and
+   currentness dependency remains exact. A later Action Authorization policy may
+   independently require a fresh Engine verification; fresh Action
+   Authorization Binding and target checks remain separate and mandatory before
+   an attempt. A changed dependency requires a new reliance occurrence or fresh
+   evaluation exactly when the admitted policy requires one, never a rewrite of
+   historical truth.
 
 A source observation, evidence record, closed evaluation context, claim
 evaluation, metric result, gate decision, and canonical explanation remain
@@ -273,11 +301,13 @@ supersession, stack change, escalation, or posture transition MUST NOT rewrite
 them. A current view may say that a historical result was validly issued under
 an earlier basis and is no longer eligible for current reliance.
 
-Historical replay and explanation require Engine verification of exact issuance
-and historical validity, not a current authorization fiction. Merely delivering
-an already closed immutable result does not relaunch a producer or reacquire
-authority. Re-running evaluation or using the result for a new effect follows
-the current checkpoints above.
+Historical replay, explanation, and any later Operational Reliance require
+Engine resolution of exact issuance and the applicable historical-validity
+outcome, not a current-authorization fiction. Merely delivering an already
+closed immutable result does not relaunch a producer or reacquire authority.
+Re-running evaluation follows the launch and final-closure checkpoints above;
+present reliance follows its explicit mode, and any effect follows a fresh,
+separate action-authorization and attempt chain.
 
 ## Capability, operating, and observation invariants
 
@@ -322,18 +352,19 @@ the current checkpoints above.
 | Attempt retry under unchanged basis | Reuse the Prospective Evaluation Basis only when every bound dependency remains exact and current and that exact Producer Selection's policy permits retry; create a new attempt identity bound to the same selection unless it is exact delivery replay. Other selections and their attempts remain distinct. |
 | Authorization becomes non-current before launch | Attempt Disposition is `refused`; do not launch and create no observation. `held` is unavailable because this is a failed authorization boundary, not a valid `known_limited` or `unknown` assessment. |
 | Authorization becomes non-current after partial or completed collection but before final closure | Preserve exact attempt and observation provenance, but refuse new Contracts truth. A fresh authorized run cannot retroactively adopt or upgrade those observations. |
-| Authorization changes after truth closes | Preserve the closed result. Current operational reliance requires fresh owner verification and, when the basis changed, a fresh evaluation. |
+| Authorization changes after truth closes | Preserve the closed result. Any present reliance universally resolves the exact Binding and historical validity, then follows its admitted `historical_validity_only` or `continuing_current` mode. Only the latter requires current Engine authorization for the Determination. A later effect still requires fresh separate action authorization and target checks; a fresh evaluation is required only when the selected reliance or authorization policy makes the changed Engine proposition a present dependency. |
 | Approved escalation | Engine issues a fresh binding from the newly admitted envelope. Readiness, Prospective Evaluation Basis, attempts, evidence-set identity, and final evaluation context are fresh. Requirements remain unchanged unless separately reselected for an independent reason. |
 | Refused escalation | Existing authorization remains unchanged if otherwise current; no requested reach or requirement changes. |
-| Superseded request or binding | Follow Engine's exact successor relation. The superseded object remains historical and cannot authorize new collection, truth, or reliance. |
+| Superseded request or binding | Follow Engine's exact successor relation. The superseded object remains historical and cannot authorize new collection or new evaluation truth. Present reliance may consume an already closed result only through universal historical Binding resolution/validity and the policy-selected `historical_validity_only` or `continuing_current` mode; the superseded binding never becomes action authority. |
 | Partial collection or operational timeout | Record a distinct per-selection attempt terminal and any permitted exact partial observation. Do not fabricate completion, actual reach, evidence sufficiency, plan completion, or gate truth. A retry is another attempt for that selection. |
 | Readiness cache reuse | Require exact Readiness Basis identity and fresh owner verification. Changed time/currentness, applicability knowledge, Observation Plan, role allocation, Producer Selection, capability, ceiling, host prerequisite, purpose, use, requirements, or authorization invalidates current reuse. |
-| Current-result cache reuse | Require the complete final evaluation closure plus every current reliance verification. Authorization change prevents current-use reuse even though historical identity remains valid. |
-| Historical-result cache reuse | Permit exact immutable replay or explanation only; label current authorization and reliance separately. |
+| Current-result cache reuse | A cached result may supply only exact immutable historical truth. A new present-use decision requires an exact Operational Reliance occurrence, universal historical Binding resolution/validity, the policy-selected `historical_validity_only` or `continuing_current` checks, every other declared current dependency, and complete accounting. A cached Determination cannot authorize an effect; fresh action authorization and target checks remain separate. |
+| Historical-result cache reuse | Permit exact immutable replay or explanation only, with exact historical Binding resolution and owner-attributed resolution outcome. Label any present reliance, current Engine status, disclosure limit, and action authority separately. |
 
-No cache can convert historical validity into current authorization, replace an
-owner verification, merge or substitute selections, attempts, or evidence
-roots, or make a diagnostic result decision-seeking.
+No cache can convert historical validity into current authorization, make a
+policy omit its required mode, convert evaluation authorization into action
+authorization, replace an owner verification, merge or substitute selections,
+attempts, or evidence roots, or make a diagnostic result decision-seeking.
 
 ## Advisory and enforceable behavior
 
@@ -344,6 +375,20 @@ is advisory outside that consumer's own call boundary. Agent instructions and
 protocol acknowledgements do not prove that filesystem, network, process,
 memory, CPU, tool, or other host restrictions were enforced.
 
+That evaluation boundary does not impose a general Handbook identity,
+credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
+or ABAC platform. Engine Context Resolution still owns semantic admission and
+verification of its Evaluation Authorization Binding. Separately, a standalone
+CLI/SDK action path may carry user-prompt or configured-agent-role intent,
+claimed principal/delegation, and declared **advisory** action authority when an
+exact downstream Action Authorization policy permits that mode. Those claims do
+not become Engine evaluation authorization, independently authenticated
+identity/delegation, or Host Enforcement Facts; ambient operating-system
+permissions are real execution conditions but are not fine-grained Handbook
+credentials. Advisory authority cannot satisfy a policy that requires stronger
+owner-verified or host-enforced facts, and it cannot waive the action target,
+currentness, or commit-time checks fixed by the operational-reliance companion.
+
 An enforcing host or future Substrate adopter may launch only from one exact
 current Prospective Evaluation Basis, translate only constraints it actually
 supports, apply or refuse those constraints, and emit exact Host Enforcement
@@ -359,14 +404,16 @@ mechanism.
 | --- | --- | --- |
 | Assurance requirements | Posture and assurance-policy owners select the immutable proof plan before capability comparison. | Engine authority, readiness, easier requirements from current capability, evidence truth, or host enforcement. |
 | Contracts-domain evaluation coordination | The broader Contracts domain's distinct coordination subresponsibility forms Observation Plan and Producer Selections, closes them into Evaluation Purpose, and forms Readiness Basis and Assessment, Attempt Disposition, and Prospective Evaluation Basis; it owns exact identity, cardinality, role accounting, and per-selection retry/reuse rules for those objects. | Engine admission or issuance, capability self-attestation, evidence admission or claim/metric/gate truth, operational enforcement, or mutation. |
-| Engine Context Resolution | Engine admits complete envelopes, proves request non-expansion, issues Evaluation Authorization Bindings, preserves private lineage, and owns verification, historical validity, currentness, and supersession. | Assurance selection, producer capability, readiness comparison, evidence adequacy, gate truth, or host enforcement. |
+| Engine Context Resolution | Engine admits complete envelopes, proves request non-expansion, issues Evaluation Authorization Bindings, preserves private lineage, and owns verification, historical validity, currentness, and supersession. | Assurance selection, producer capability, readiness comparison, evidence adequacy, gate truth, host enforcement, or a general Handbook IAM platform. |
 | Producer-capability authority | A named independent owner admits versioned Producer Semantic Capability facts and their currentness. | Engine authorization, actual reach, evidence sufficiency, or permission to launch. |
 | Dock or adapter | Owns its admitted operating ceiling, request compatibility, invocation protocol, and honest operational terminals. | Engine authority, universal producer capability, host enforcement, canonical evidence, or semantic truth. |
 | Enforcing host | Applies or refuses supported runtime restrictions and records exact Host Enforcement Facts and attempt effects. | Resolution semantics, assurance selection, readiness, evidence admission, gate interpretation, or invented enforcement. |
 | Producer | Performs one exact attempt and reports source observations and Actual Observed Reach with provenance. | Canonical evidence admission, readiness reclassification, claim adequacy, gate truth, or authority expansion. |
 | Contracts-domain evidence and evaluation subresponsibilities | Sibling subresponsibilities admit candidates, fix the evidence-set identity, close the final evaluation context with the exact Prospective Evaluation Basis and Engine verification, and own claim/metric/gate truth under the companion record. | Engine currentness or authorization issuance; producer capability or host enforcement; recomputing or reinterpreting coordination-owned readiness; or requirement relaxation. |
-| Downstream policy and mutation owners | Decide whether a closed result with exact use intent and current reliance verifications may affect work, then perform any separately authorized mutation. | Rewriting historical truth, relabeling diagnostic provenance, treating readiness as a gate, or inheriting Engine/posture authority. |
-| SDK, CLI, Tauri, skills, and other transports | Preserve and project exact owner results, correlation, presentation, and interface-local failures. | Semantic formation, owner verification, capability inference, task-wide enforcement, or truth reinterpretation. |
+| Work Progression and other Operational Reliance owners | Resolve exact historical results and Binding validity, select the explicit `historical_validity_only` or `continuing_current` mode, close target/currentness/accounting, and produce one atomic Determination with paired eligibility/disposition facets. Work Progression is the proposed semantic owner for local and parent eligibility. | Rewriting historical truth, relabeling diagnostic provenance, treating readiness as a gate, issuing action authority, or mutating state. |
+| Action requester and action-authorization owner | The requester owns one exact Action Intent. The authorization owner independently produces an Action Authorization Binding or Refusal with exact advisory, verified, or host-enforced authority provenance and every declared fresh condition. | Treating Evaluation Authorization as action authorization, fabricating authentication/delegation/enforcement, improving reliance eligibility, or mandating a general Handbook IAM implementation. |
+| Action executor and authoritative state/effect owner | The executor owns one exact Action Attempt Identity and Terminal. The state/effect owner performs through-commit expected-state and authorization checks and alone issues the Mutation Receipt for a committed change. | Ordinary retry after uncertain commit, broadening the authorized effect, or reporting mutation success without the authoritative receipt. |
+| SDK, CLI, Tauri, skills, and other transports | Preserve and project exact owner results, correlation, presentation, interface-local failures, and prompt- or role-supplied advisory-authority claims with their provenance. | Semantic formation, owner verification, capability inference, authentication by assertion, task-wide enforcement, or truth reinterpretation. |
 
 ## Discriminating scenario corpus
 
@@ -389,7 +436,7 @@ identities except where the row identifies a failure.
 | **Actual exceeds a bound.** A producer reports reach above request, Engine authorization, or admitted operating ceiling. | Candidate admission refuses; the value is not clamped. No claim, metric, or gate truth is produced from it. | Actual-observation honesty is fail closed. |
 | **Authorization superseded before launch.** Engine reports the binding superseded at the launch checkpoint. | Disposition refuses; no selection launches. The immutable binding remains historically issued and retains its successor relation. | Issuance, currentness, and supersession are separate facts. |
 | **Authorization changes after collection.** Exact observations exist, but Engine reports the binding non-current before final Contracts closure. | Attempt and observation history remain immutable; final closure refuses new claim, metric, and gate truth. A fresh binding cannot retroactively authorize the old collection. | Pre-launch permission does not bypass the pre-truth checkpoint. |
-| **Authorization changes after a pass.** A valid `pass` closed before the binding became non-current. | The historical `pass` and explanation remain unchanged. Current operational reliance requires fresh verification and, when the basis changed, a fresh evaluation. | Current use does not rewrite historical truth. |
+| **Authorization changes after a pass.** A valid `pass` closed before the binding became non-current. One Operational Reliance Policy declares `historical_validity_only`; another declares `continuing_current`. | The historical `pass` and canonical explanation remain unchanged, and both reliance paths must resolve the exact Binding and historical validity. The first may form its Determination without treating current Engine status as an input when every other dependency closes. The second refuses absent a fresh positive Engine current verification and requires a fresh evaluation when its policy requires one. Any effect under either path still requires a distinct fresh Action Authorization Binding, target verification, attempt, and state-owner receipt. | Historical validity, policy-selected continuing-current reliance, and fresh action authorization are separate propositions. |
 | **Approved escalation.** The proof plan is unchanged, and Engine approves broader reach after a prior basis exists. | Engine issues a fresh binding; readiness, basis, attempts, evidence set, and final context are fresh. Prior evidence and results are not upgraded. | Escalation is not in-place expansion. |
 | **Per-selection retry after timeout.** Selection B's attempt times out after an exact permitted partial observation; its producer policy permits retry under the still-current same basis, while selection A already completed. | A's attempt remains unchanged. B's partial terminal remains immutable and a new B-bound attempt identity is created; it cannot substitute for A or merge implicitly with either prior attempt. Exact delivery replay alone is idempotent. | Retry is per selection, not plan-wide overwrite or duplicate authority. |
 | **Stale readiness cache.** Authorization is still current, but producer capability or operating-ceiling currentness changed after assessment. | Cached readiness cannot support launch. A fresh Readiness Basis and Assessment are required. | Readiness cache identity includes all independent capability facts. |
@@ -404,11 +451,14 @@ Observation Plan and Producer Selection closure; purpose, authorization,
 readiness, disposition, basis, and per-selection attempt identity/cardinality;
 the exact `known_limited`/`unknown`/Readiness Refusal boundary; Engine issuance,
 projection privacy, verification, historical-validity, currentness, and
-supersession responsibilities; the three currentness checkpoints; readiness
-states and refusal boundary; diagnostic versus decision-seeking behavior;
-capability/ceiling/host/actual-reach separation; retry, partial collection,
-cache, escalation, supersession, and reuse rules; advisory versus enforceable
-behavior; the responsibility map; and every scenario outcome above.
+supersession responsibilities; mandatory current verification at collection and
+final truth closure; universal historical Binding resolution/validity plus the
+explicit `historical_validity_only` / `continuing_current` reliance-time rule;
+separate fresh action authorization; readiness states and refusal boundary;
+diagnostic versus decision-seeking behavior; capability/ceiling/host/actual-
+reach separation; retry, partial collection, cache, escalation, supersession,
+and reuse rules; advisory versus enforceable behavior without a general
+Handbook IAM mandate; the responsibility map; and every scenario outcome above.
 
 Deliberately deferred to later human-approved task-level planning are concrete
 schemas, types, APIs, operations, storage, registries, transports, status-code
@@ -416,9 +466,14 @@ encodings, algorithms, cache mechanisms, retry counts, timeouts, producer
 selection, capability policies, host constraints, enforcement mechanisms,
 crate/module placement, migrations, feature flags, slice or packet
 decomposition, acceptance planning, and rollout. Concrete assurance mappings,
-requirements, metrics, thresholds, gate policies, UI, explanation-density
-projection, parent-promotion policy, posture transitions, publication, and
-external adopter or Substrate selection remain owned elsewhere or deferred.
+requirements, metrics, thresholds, gate policies, concrete explanation-
+projection decisions, UI, parent-promotion policy content, posture-transition
+policy content, publication, and external adopter or Substrate selection remain
+owned elsewhere or deferred. The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+owns detailed present-use and action-chain semantics and proposes Work
+Progression as local/parent eligibility owner; physical placement, work hierarchy
+and state model, concrete policies, and implementation remain deferred.
 
 Human approval of this proposed record would make it semantic input to later
 planning only. It would not authorize implementation, change any phase or slice
diff --git a/docs/specs/handbook-contract-membrane/evaluation-identity-evidence-closure-semantic-decision.md b/docs/specs/handbook-contract-membrane/evaluation-identity-evidence-closure-semantic-decision.md
index 5c64842..dbd8cd5 100644
--- a/docs/specs/handbook-contract-membrane/evaluation-identity-evidence-closure-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/evaluation-identity-evidence-closure-semantic-decision.md
@@ -12,11 +12,14 @@ It is a narrow companion to the proposed
 [canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
 [posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
 [evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
-and [metric, evaluation, and gate decision](metric-gate-semantic-decision.md).
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
+[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
 Those records retain their object classifications, semantic owners, assurance,
-authorization, readiness, claim, metric, gate, projection, and downstream
-action boundaries. This record normatively refines only identities and closure
-relations that those records deliberately left open.
+authorization, readiness, authored relationships, claim, metric, gate,
+projection, reliance, and action boundaries. This record normatively refines
+only identities and closure relations that those records deliberately left
+open.
 
 This record is semantic authority for human review only. It is not
 implementation authority, a slice or packet specification, a persistence
@@ -43,8 +46,9 @@ Resolved Assurance Requirements
   -> terminal completion of every Contracts admission operation causally attributable to an in-frontier submission
   -> Canonical Evidence Set Formation and Closure
   -> final Contracts evaluation-context closure
-  -> final applicability, claims, metrics, semantic gate, and explanation
-  -> separately owned present operational reliance, if any
+  -> final applicability, claims, metrics, semantic gate, and canonical explanation
+  -> action-specific Operational Reliance Basis and atomic Determination, if used now
+  -> separate Action Intent / authorization / attempt / terminal / receipt, if an effect is requested
 ```
 
 Allocation of an Evaluation Occurrence Identity is part of Evaluation Purpose
@@ -66,9 +70,15 @@ This record settles:
    claim evaluator selects from it; and
 5. which semantic times and historical dependencies must remain resolvable.
 
-It does not decide authored-artifact fields beyond exact subject, scope, and
-snapshot references required by these closures. It does not decide current-use
-eligibility, local closeout, parent promotion, action policy, or mutation.
+The proposed
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md)
+owns detailed authored fields and relationship roles beyond the exact subject,
+scope, and snapshot references required by these closures. The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+owns present-use reliance, atomic eligibility/disposition, local closeout,
+parent eligibility, action authorization, attempts, terminals, receipts, and
+mutation boundaries. Those proposal links do not approve either record or
+select physical placement, policies, hierarchy/state models, or implementation.
 
 ## Canonical vocabulary
 
@@ -613,7 +623,7 @@ times after inspecting candidate, evidence, claim, metric, or gate direction.
 | **Evidence admission time** | Contracts admission records the exact source-validity and policy decision for candidate-to-evidence transition. | Admission-time validity remains historical; it is not evaluation-time evidence currentness. |
 | **Evidence-set closure time** | Contracts applies the precommitted rule to the sealed frontier, terminal admission population, membership, formation exclusions, and complete accounting. | Later records cannot enter the set. Different closure content under the occurrence refuses. |
 | **Final evaluation time** | Contracts binds evidence currentness, final applicability, claim/metric/gate definitions, final Engine verification, and the complete context. | Results are immutable historical truth for this time and context. |
-| **Operational reliance time** | The downstream action owner, outside this decision, verifies the exact present-use dependencies its policy requires. | It cannot rewrite historical truth or inherit evaluation authorization as action authority. |
+| **Operational reliance time** | The downstream reliance owner universally resolves the exact historical Evaluation Authorization Binding and validity outcome, then applies its admitted `historical_validity_only` or `continuing_current` mode plus every other declared present-use dependency. | Current Engine verification is required only by `continuing_current`; either mode preserves historical truth and cannot inherit evaluation authorization as action authority. A later effect has a distinct fresh Action Authorization Binding and commit-time checks. |
 
 The causal order between named stages is mandatory even when their time values
 are not numerically comparable. No owner may substitute its local receipt time
@@ -683,10 +693,20 @@ attributed resolution observations and an explicit statement that exact
 historical replay is presently unavailable. This record introduces no new
 cross-owner aggregate historical-verification status.
 
+One canonical explanation remains source truth. A concise default human view and
+requested deeper drill-down are deterministic projections over that same truth,
+limited independently to the highest detail both available and authorized for
+disclosure. Reduced or denied detail does not trigger reevaluation or create a
+new semantic result. Concrete projection decisions, UI/API shape, and any fixed
+number of density levels remain deferred.
+
 Those observations and the retained result grant no present authority. Whether
 any downstream action may rely on the result is decided exclusively by the
-separately owned Operational Reliance policy over its exact present-use inputs.
-This record neither grants nor universally denies operational eligibility.
+separately owned Operational Reliance Policy and its exact present-use inputs.
+That policy universally requires historical resolution/validity and explicitly
+selects `historical_validity_only` or `continuing_current` for present Engine
+status. This record neither grants nor universally denies operational
+eligibility, and any later effect requires separate fresh action authorization.
 
 Exact historical replay uses the recorded semantic times and historical owner
 facts. It MUST NOT rerun producers, substitute current definitions, re-evaluate
@@ -699,7 +719,7 @@ deliberately new occurrence.
 | Responsibility | Owns | MUST NOT own or infer |
 | --- | --- | --- |
 | Contracts-domain Evaluation Coordination | Evaluation Occurrence Identity allocation; Evaluation Content Fingerprint and Purpose formation; exact Plan/Purpose congruence; readiness reformation boundary; occurrence commitment to one prospective basis; one exact Occurrence Collection Frontier from origin-owner terminals. | Engine issuance/currentness, prospective or final applicability truth, invention of origin or admission terminals, evidence admission/set membership, claim/metric/gate truth, or operational reliance. |
-| Contracts definition authorities | Independent admission, immutable identity, compatibility, currentness, and historical resolution of Prospective Applicability Policies and Canonical Evidence Set Formation Policies. | Applying a policy to select a desired plan, set membership, applicability, claim, metric, or gate outcome. |
+| Contracts definition authorities | Deterministic Policy Admission Closure, immutable identity, compatibility, currentness, and historical resolution of Prospective Applicability Policies and Canonical Evidence Set Formation Policies, including exact relevant governance-source roles/scope/dependency modes and complete candidate accounting. | Treating authored candidate text or a bare link as admitted policy, recursively letting a candidate admit its own rule, or applying a policy to select a desired plan, set membership, applicability, claim, metric, or gate outcome. |
 | Contracts applicability responsibility | Exact resolution and application of an already admitted Prospective Applicability Policy; Prospective Applicability Basis formation; final applicability; prospective/final congruence. | Policy self-admission, ambient or evidence-dependent plan pruning, producer selection, Engine authority, or requirement relaxation. |
 | Engine Context Resolution | Evaluation Authorization Binding issuance, private lineage, named verification observations, historical validity, currentness, and supersession. | Occurrence allocation, content fingerprints, applicability, plan adequacy, readiness, evidence membership, or replay reconstruction by Contracts. |
 | Producer Capability, Dock/adapter, prerequisite, retry, partial-completion, and availability owners | Their exact semantic definitions and checkpoint-current verification or occurrence facts under independently attributable authority. | Altering Producer Selection, assurance requirements, Engine authorization, readiness outcome, or evidence truth. |
@@ -709,7 +729,8 @@ deliberately new occurrence.
 | Producer, Dock, adapter, and enforcing host | Exact attempt, source, candidate-submission, terminal, actual-reach, and applied/refused enforcement provenance for their bounded propositions. | Canonical evidence, set closure, applicability, truth evaluation, or current-use authority. |
 | Definition and repository owners | Exact historical definition/content resolution, owner provenance, semantic-time/currentness observations, and declared retention obligations for their objects. | Replacing an historical identity with a current equivalent or making a consumer the semantic owner. |
 | SDK, CLI, Tauri, Flow, Pipeline, skills, and transports | Bounded composition, rendering, correlation, delivery, and deterministic projections or assemblies of exact owner objects. | Occurrence identity, owner formation replay, evidence membership, semantic-time invention, historical verification, or truth reinterpretation. |
-| Downstream action domain | Present-use policy, reliance checks, action authority, and any mutation at a later semantic time. | Rewriting this occurrence's identity, evidence closure, historical truth, or treating Engine evaluation authorization as action authority. |
+| Work Progression and other Operational Reliance owners | Exact action question, finite result/target closure, universal historical Binding validity, explicit `historical_validity_only` or `continuing_current` mode, Basis, atomic Determination with paired eligibility/disposition facets, and Reliance Refusal. Work Progression is the proposed local/parent eligibility owner. | Rewriting this occurrence's identity, evidence closure, historical truth, issuing action authority, or mutating state. |
+| Action requester, authorization, executor, and state/effect owners | Exact Action Intent; Action Authorization Binding or Refusal with declared advisory/verified/enforced provenance; Action Attempt Identity and Terminal; through-commit verification; and authoritative Mutation Receipt. | Treating Engine evaluation authorization as action authorization, fabricating identity/delegation/enforcement, ordinary retry after uncertain commit, or claiming committed mutation without the state-owner receipt. |
 
 Every object introduced by this record is a canonical semantic definition, an
 owner-produced canonical record/result, durable provenance, or a bounded value
@@ -782,18 +803,23 @@ This record settles:
   preserving exact owner non-success observations without creating an aggregate
   status, rewriting historical truth, or deciding operational eligibility.
 
-Deliberately deferred to the focused authored-artifact relationships decision
-are concrete fields, references, requiredness, and mutation relationships for
-Charter, Project Context, Environment Context, Work Specification, Decision
-Record, and Risk Record. This record binds only exact subject/snapshot and
-scope identities supplied through their proper owners.
-
-Deliberately deferred to the focused operational-reliance decision are
-Current-Use Eligibility, Operational Reliance Basis, local closeout, parent
-eligibility, promotion, release/deployment effects, action-specific policies,
-the universal-versus-policy-declared Engine verification rule for present use,
-and every mutation authority. Historical resolvability and replay grant none
-of them.
+The proposed
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md)
+owns concrete relationship semantics, references, and requiredness for Charter,
+Project Context, Environment Context, Work Specification, Decision Record, and
+Risk Record. This record binds only exact subject/snapshot and scope identities
+supplied through their proper owners and does not approve that companion.
+
+The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+owns Current-Use Eligibility and Operational Disposition as facets of one
+atomic Determination, Operational Reliance Basis and Refusal, local closeout,
+parent eligibility, explicit `historical_validity_only` versus
+`continuing_current` present-use modes, action authorization, attempts,
+terminals, receipts, and mutation boundaries. Historical resolvability and
+replay grant none of them. Work Progression is only the proposed semantic owner
+of local/parent eligibility; physical placement, work hierarchy/state model,
+concrete policies, and implementation remain deferred.
 
 Also deferred to later human-approved task-level planning are concrete schemas,
 Rust types, APIs, identity allocation algorithms, timestamps or logical-clock
diff --git a/docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md b/docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md
index 731c601..6bebe63 100644
--- a/docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md
@@ -13,12 +13,18 @@ Current source and the completed [HCM-5.1](slices/HCM-5.1/README.md) and
 ownership, dependency direction, and trust boundaries. Their current public
 shapes and computations neither constrain nor supply this model.
 
-The companion
-[posture and assurance-policy semantic decision](posture-assurance-policy-semantic-decision.md)
-owns the proposed upstream selection seam. When posture influences assurance
-selection, that record's exact snapshot and resolved-requirement provenance are
-part of this record's closed evaluation context; neither record is
-implementation authority.
+This record is a companion to the proposed
+[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
+[posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md),
+[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
+[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
+and [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
+The posture record owns the proposed upstream selection seam; the operational-
+reliance record owns the proposed downstream present-use and action seam. When
+posture influences assurance selection, the exact snapshot and resolved-
+requirement provenance are part of this record's closed evaluation context.
+All seven records remain proposals and supply no implementation authority.
 
 ## Canonical model and vocabulary
 
@@ -39,8 +45,13 @@ resolved assurance requirements (when applicable)
   -> applicability and claim evaluation
   -> zero or more named metric results
   -> gate policy
-  -> semantic gate decision
-  -> separately owned downstream disposition or mutation
+  -> semantic gate decision and canonical explanation
+  -> action-specific Operational Reliance Basis, when used operationally
+  -> one atomic Operational Reliance Determination
+       - Current-Use Eligibility facet
+       - fixed Operational Disposition facet
+  -> separate Action Intent and Action Authorization Binding or Refusal
+  -> Action Attempt, Action Terminal, and state-owner Mutation Receipt only after committed mutation
 ```
 
 The following terms are normative. Terms in one row are not synonyms for terms
@@ -61,7 +72,8 @@ in another row.
 | **Metric result** | The exact output of one metric definition: `computed` with a value on its named scale, `not_applicable`, or `not_computable`, plus included/excluded inputs and a reproducible calculation trace. There is no unqualified “aggregate score.” |
 | **Gate policy** | An immutable semantic decision rule over direct claim-evaluation predicates and named metric-result predicates, including condition precedence and treatment of indeterminacy. |
 | **Semantic gate decision** | The non-mutating result `pass`, `fail`, `indeterminate`, or `not_applicable`, with every condition result and decisive reason. It answers the policy's bounded semantic question only. |
-| **Downstream disposition** | A separately owned enforcement, workflow, posture, or promotion interpretation of a semantic gate decision, such as allow, block, hold, or no effect. A mutation, if any, remains another authority-controlled operation. |
+| **Operational Reliance Determination** | The separately owned, immutable action-domain result produced from one exact Operational Reliance Basis. It contains Current-Use Eligibility and Operational Disposition as two fixed facets for one occurrence, policy, action question, target, semantic time, and decisive accounting closure. It grants no action authority and performs no mutation. |
+| **Disposition Reference** | A reference to one exact Operational Reliance Determination plus its exact Operational Disposition facet being displayed or documented. It is not a second result and cannot let the paired eligibility/disposition facets diverge. |
 
 A final **evaluation context** is formed only after its exact canonical
 evidence-set identity is fixed. It binds that evidence set; one contract
@@ -240,19 +252,39 @@ coerced, discarded, or normalized under this rule.
 
 An Evidence Evaluation Policy may fill only a role declared by the selected
 contract, claim, or evidence requirement and only when its exact identity and
-semantics satisfy that role's compatibility constraints. It MUST NOT change canonical-admission
-authority, provenance, identity, or another non-relaxable trust invariant; the
-claim proposition, finding scale, direction, or unit; a Metric Definition; or
-Gate Policy meaning. Missing, incompatible, conflicting, substituted, or
-unaccounted policy bindings refuse rather than falling back, overriding claim
-semantics, or becoming an implicit relaxation.
+semantics satisfy that role's compatibility constraints. It MUST NOT change
+canonical-admission authority, provenance, identity, or another non-relaxable
+trust invariant; the claim proposition, finding scale, direction, or unit; a
+Metric Definition; or Gate Policy meaning. Missing, incompatible, conflicting,
+substituted, or unaccounted policy bindings refuse rather than falling back,
+overriding claim semantics, or becoming an implicit relaxation.
+
+A user, agent, authored artifact, or tool MAY author an Evidence Evaluation
+Policy, Metric Definition, or Gate Policy candidate. It becomes an executable
+category-2 definition or policy only after its semantic owner deterministically
+produces the category-3 **Policy Admission Closure** defined by the companion
+[canonical-object decision](canonical-object-ontology-and-ownership-semantic-decision.md).
+That closure binds the exact candidate; typed vocabulary and compatibility;
+finite selection/accounting; role and scope completeness; every relevant exact
+Charter, Project Context, Decision Record, or Risk Record revision with its
+proposition role, scope, and historical/formation-only or continuing-current
+dependency mode; and all non-relaxable invariants. When the admission rule
+requires an approval or exception, the closure separately binds the exact typed,
+authorized source, its authority, scope, identity/version or revision as
+applicable, and currentness. It requires only the governance families relevant
+to the admission rule. Authored text, a prompt or agent role, artifact promotion,
+and a bare Markdown link are governance input, not runtime evidence, a waiver,
+or self-admission. “Unsafe” means mechanically prohibited by admitted
+constraints, not an evaluator's open-ended organizational judgment; the exact
+owner admission rule is a non-recursive root outside the candidate population it
+decides.
 
 ## Responsibility map
 
 | Layer | Owns | Must not decide |
 | --- | --- | --- |
 | Engine Context Resolution authorization | Exact envelope admission and non-expansion; issuance of the purpose-bounded Evaluation Authorization Binding; verification of its issuance lineage, underlying envelope, request-within-envelope relation, authority admission, currentness or historical validity, and supersession. | Assurance selection, producer capability or readiness, Contracts context congruence, evidence adequacy, claim truth, metrics, gates, explanation density, or downstream action. |
-| Prospective evaluation formation | Exact pre-observation binding of purpose, subject/scope, objective/use, requested reach, Engine authorization, and required resolved-assurance/readiness/use-intent provenance. Physical owner placement remains deferred. | Evidence-set identity, claim or metric truth, gate decision, Engine admission, requirements relaxation, or host enforcement by implication. |
+| Contracts-domain Evaluation Coordination | The proposed Contracts-domain coordination responsibility forms the exact pre-observation binding of purpose, subject/scope, objective/use, requested reach, Engine authorization, and required resolved-assurance/readiness/use-intent provenance. Concrete crate, module, API, persistence, and transport placement remain deferred. | Evidence-set identity, claim or metric truth, gate decision, Engine admission, requirements relaxation, or host enforcement by implication. |
 | Validators and source providers | Bounded source-specific execution, source observation, source snapshot identity, native values/units/scales, and collection diagnostics. | Applicability, claim findings, canonical admission, metric weights/values, gate policy, or downstream action. |
 | Docks and adapters | Protocol execution/framing, exact translation, preservation of source semantics and lexemes, exact Prospective Evaluation Basis provenance, producer operating-ceiling enforcement, candidate construction, and import diagnostics. | Engine authorization; treating request/ceiling/actual equality as authority proof; canonical evidence; invented units or scale direction; silent normalization; claim meaning; applicability; metrics; or gates. |
 | Contracts evaluation-context closure | Final binding of the exact canonical evidence set, Prospective Evaluation Basis, Engine-produced authorization-currentness result, and all contract/policy/evaluator/metric/gate/posture provenance; refusal before semantic truth when closure or required Engine verification is absent or negative. | Forming the pre-observation basis, issuing authorization, or reconstructing Engine rank, parent, constraint, mutation, escalation, authority-admission, currentness, or supersession semantics. |
@@ -262,7 +294,10 @@ semantics, or becoming an implicit relaxation.
 | Contracts-owned named metric-definition authority | Canonical establishment, exact re-admission validation, and exact resolution of immutable named, versioned, configured metric definitions; validation of the eligible-population and compatibility rules, input/output scales and units, algorithm identity/version, parameters and coefficients under their algorithm-declared semantics and bounds, aggregation semantics, and missing/indeterminate-input policy. | Evidence selection, finding interpretation or establishment, the actual evaluation-run population, metric computation, threshold/comparator application, gate decision, or downstream mutation. |
 | Metric computation | Pure exact application of one exactly resolved named metric definition to compatible claim evaluations in one closed evaluation context, including derivation and accounting of the actual included/excluded population and calculation result. | Raw-source/evidence selection, applicability or finding changes, definition changes, gate threshold/comparator application, gate decision, or downstream disposition. |
 | Gate policy | Direct non-compensatory claim-evaluation predicates, named metric-result predicates, precedence, indeterminacy treatment, and the semantic gate decision. | Raw evidence interpretation, unit/scale conversion, changing metric semantics, canonical admission, or any workflow/posture/promotion mutation. |
-| Downstream consumers | Their own versioned mapping from the exact semantic decision to enforcement, posture, workflow, or promotion disposition; any separately authorized mutation with fresh authority/currentness checks. | Rewriting the gate decision, treating a metric as the decision, bypassing decisive provenance, or transferring mutation authority into Contracts, a dock, SDK, or transport. |
+| Work Progression and other Operational Reliance owners | Resolve exact historical results and Evaluation Authorization Binding validity; apply one admitted action-specific Operational Reliance Policy with explicit `historical_validity_only` or `continuing_current` Engine dependency mode; close exact target/currentness/accounting; and produce one atomic Operational Reliance Determination with paired eligibility/disposition facets or a Reliance Refusal. Work Progression is the proposed semantic owner for local-closeout and parent eligibility. | Rewriting the semantic gate or canonical explanation, treating a metric as the decision, issuing action authority, mutating state, or silently selecting work hierarchy/state policy or physical placement. |
+| Action requester and action-authorization owner | The requester owns one exact Action Intent after `eligible / allow`. The exact action-authorization owner independently produces an Action Authorization Binding or Action Authorization Refusal with the declared advisory, verified, or host-enforced authority provenance and fresh conditions. | Treating Evaluation Authorization, a gate, `allow`, a prompt, ambient OS permissions, or artifact approval as independently authenticated identity, verified delegation, host-enforced permission, or a mandate for a general Handbook IAM platform. |
+| Action executor and authoritative state/effect owner | The executor owns one exact Action Attempt Identity and Action Terminal. The authoritative state/effect owner performs attempt- and commit-bound expected-state, effect, authorization, and declared currentness checks and alone issues the Mutation Receipt for a committed change. | Ordinary retry after uncertain commit, broadening the authorized effect, inferring commit from an executor report, or reporting mutation success without the authoritative receipt. |
+| SDK, CLI, Docks, hosts, and other transports | Preserve exact owner results and provenance, including policy-permitted prompt- or configured-role-supplied advisory claims. A real enforcing host may supply stronger independently verified facts. | Becoming a semantic owner by transport; fabricating authentication, delegation, or host enforcement; waiving target/currentness checks; or satisfying a policy that requires enforcement with advisory claims. |
 
 ## Gate and explanation invariants
 
@@ -289,17 +324,22 @@ semantics, or becoming an implicit relaxation.
   and the exact reason without fabricating claim, metric, or gate truth.
 - A later authority change, revocation, supersession, stack change, or approved
   escalation MUST NOT rewrite a closed result or explanation. That result
-  remains immutable historical truth for its exact closure. Any current
-  operational reliance requires fresh owner verification; whenever the
-  authorization purpose or basis changed, it requires a fresh Evaluation
-  Authorization Binding, Prospective Evaluation Basis, and final evaluation
-  context.
+  remains immutable historical truth for its exact closure. Every present
+  Operational Reliance Basis MUST resolve the exact Evaluation Authorization
+  Binding and Engine-confirmed historical validity. Its admitted Operational
+  Reliance Policy MUST additionally select exactly one continuing-status mode:
+  `historical_validity_only` or `continuing_current`. Only the latter requires
+  fresh positive Engine current verification at the reliance semantic time and,
+  when that policy makes the changed proposition decisive, a fresh evaluation.
+  Fresh action-specific authorization and target verification remain separate
+  and universal before any later Action Attempt; neither reliance mode weakens
+  the strict collection-time or final-evaluation-closure checks above.
 - Producer capability, a Dock or adapter operating ceiling, equality among a
   request, operating ceiling, and actual observation, per-claim evidence
   adequacy, viewer disclosure authority, posture strictness, and explanation
   density are independently owned facts. None establishes or replaces Engine
   evaluation authorization.
-- The four semantic decisions have fixed meanings: `pass` means the policy
+- The four semantic gate outcomes have fixed meanings: `pass` means the policy
   applies and establishes its bounded proposition; `fail` means it applies and
   establishes its negation or a policy-declared fail-closed violation;
   `indeterminate` means it applies but cannot establish pass or fail under its
@@ -320,12 +360,20 @@ semantics, or becoming an implicit relaxation.
   destroys the sole valid basis for a prior `pass`, the re-evaluated decision
   cannot remain `pass`; the changed accounting must be visible. Any ordering of
   downstream dispositions belongs to the downstream consumer's policy.
-- The explanation is canonical decision data, not reconstructed prose. It
-  preserves expected claim inventory; applicability basis; every included,
-  excluded, disqualified, stale, conflicting, and duplicate evidence binding;
-  finding and assessment dimensions; each metric definition, population,
-  value/state, conversion, and calculation; every gate condition and
-  precedence step; and the decisive reason/accounting chain.
+- The full canonical explanation is Contracts-owned canonical decision data,
+  not reconstructed prose or a density-specific reevaluation. It preserves
+  expected claim inventory; applicability basis; every included, excluded,
+  disqualified, stale, conflicting, and duplicate evidence binding; finding and
+  assessment dimensions; each metric definition, population, value/state,
+  conversion, and calculation; every gate condition and precedence step; and
+  the decisive reason/accounting chain. The default human view is concise, and
+  a requested deeper drill-down reaches the highest source detail both
+  available and independently authorized for that viewer, with omissions made
+  explicit. Disclosure authority and source availability are independent from
+  truth. No density view may change inputs, rerun evaluation, or become another
+  gate. The number, names, transport shapes, and interaction design of those
+  projections remain bounded later decisions; this record selects no fixed
+  level count, UI layout, or API.
 
 ## Discriminating scenario corpus
 
@@ -362,7 +410,7 @@ not authorize a product metric catalog entry.
 | **Quantitative unit normalization.** Authorized observations report `1 s` and `1000 ms` for the same latency quantity, and admission permits one named exact conversion. | Admission; claim evaluator; metric. | Both normalize to the same exact canonical quantity and produce the same finding and metric value with source units retained in provenance. | Units are semantic and exact, not labels. |
 | **Graded external scale.** An imported grade is `1` on a versioned ordinal scale where lower is better; a named claim evaluator maps it to a declared grade. | Adapter; admission; claim evaluator; compatible metric. | Direction is honored from the exact scale definition; a binary metric rejects the grade; only a metric naming that scale/mapping may consume it. | External numbers do not imply direction, binary truth, or weights. |
 | **Decimal relative coefficients.** The scenario-local `scenario.decimal-coefficient-seam@1` definition receives findings satisfied, unsatisfied, satisfied. A single-condition gate requires that exact metric result to be at least `5/8`, with equality included. | Metric-definition authority; metric computation; gate policy. | The permitted normalization produces `1, 6, 9`; the calculation trace records numerator `10`, denominator `16`, and exact result `10/16 = 5/8`; the gate passes on equality. | Coefficient meaning, conditional normalization, computation, bounds, and threshold remain exact and separately owned without selecting a catalog algorithm. |
-| **All not applicable.** Every claim in the gate's governed population is proved `not_applicable`; the named metric's definition declares an empty eligible population `not_applicable`; the gate policy declares that positive proof for the complete population makes its question not applicable; and the downstream policy maps that decision to `no_effect`. | Applicability evaluator; metric; gate; downstream consumer. | Metric and gate are both `not_applicable`, never `pass`; downstream disposition is `no_effect`. | Empty populations do not pass vacuously. |
+| **All not applicable.** Every claim in the gate's governed population is proved `not_applicable`; the named metric's definition declares an empty eligible population `not_applicable`; the gate policy declares that positive proof for the complete population makes its question not applicable; and an admitted action-specific Operational Reliance Policy maps that historical result into one atomic determination. | Applicability evaluator; metric; gate; Operational Reliance owner. | Metric and gate are both `not_applicable`, never `pass`; the Operational Reliance Determination has `not_applicable / no_effect` facets that share one identity and accounting closure. | Empty populations do not pass vacuously, and eligibility/disposition cannot diverge. |
 | **Degradation monotonicity.** A prior `pass` depends on one current satisfying record. At a later evaluation time the same subject snapshot, applicability, policy, and other eligible information are unchanged, but that record is now stale; the gate treats the resulting indeterminate direct predicate as `indeterminate`. | Claim evaluator; metric; gate. | No finding is established; the dependent metric is `not_computable`; the re-evaluated gate is `indeterminate`, not `pass`, and changed accounting identifies the stale record. | Loss of the sole eligible basis for pass cannot preserve or improve the gate decision. |
 | **Mixed population closure.** Candidate claim evaluations are presented as one metric population but differ in subject snapshot, contract revision, evaluator version, or evaluation time. | Evaluation-context admission; metric; gate. | The mixed closure refuses before metric or semantic-gate truth. It does not produce a `not_computable` metric or `indeterminate` gate. | Context substitution or construction failure is not a semantic result. |
 | **Multiple named metrics.** A gate requires both a named functional-metric threshold and a named security-metric threshold. The first predicate passes and the second fails; failure is decisive. | Two metrics; gate policy. | Results remain separate; the security metric predicate is decisive and the gate is `fail`. No hidden cross-metric average exists. | One metric cannot silently compensate for another. |
@@ -370,7 +418,7 @@ not authorize a product metric catalog entry.
 | **Unauthorized source.** An otherwise well-formed candidate binds a source actor that lacks current authority for the stated subject and observation class. | Contracts evidence admission. | Admission refuses before claim evaluation and produces no canonical evidence record, finding, metric result, or gate decision. | Structural validity and adapter output cannot substitute for source authority. |
 | **Coordinates without authorization.** A prospective run supplies a valid exact stack and six requested coordinate values but no Evaluation Authorization Binding. Comparison cases reuse a real binding for another subject, scope, objective, use, parent or constraint closure, or pair identical coordinate labels with a changed stack fingerprint. | Engine authorization; prospective evaluation formation. | Prospective Evaluation Basis formation refuses, so no producer is launched and no observation is collected. The comparison cases require a fresh, congruent binding; matching coordinate labels do not establish compatibility or transferability. | Requested coordinates and structural equality are not Engine authorization provenance. |
 | **Dock equality without Engine authority.** A proposed Dock request, admitted operating ceiling, and expected actual Resolution are exactly equal, but there is no verifiable Evaluation Authorization Binding. | Dock; Engine authorization; prospective evaluation formation. | Dock equality cannot close a Prospective Evaluation Basis, so the producer MUST NOT be launched under that proposal and no candidate or semantic truth is produced. | A producer operating contract neither grants nor proves Engine authority. |
-| **Authorization changes across stages.** In one case Engine reports the binding stale or superseded before launch. In a second, authorization changes after canonical evidence exists but before final evaluation-context closure. In a third, a `pass` was already closed before authorization later became non-current. | Engine authorization; prospective evaluation formation; Contracts evaluation-context closure; result/explanation projection. | The first prevents collection. The second preserves historical evidence but refuses new claim, metric, or gate truth and requires a fresh authorized basis/context. The third preserves the immutable historical `pass` and provenance, while current operational reliance requires fresh owner verification and, when the basis changed, a fresh authorization, prospective basis, and evaluation. | Current authority gates collection, new truth, and use at distinct boundaries without retroactively rewriting historical records. |
+| **Authorization changes across stages.** In one case Engine reports the binding stale or superseded before launch. In a second, authorization changes after canonical evidence exists but before final evaluation-context closure. In a third, a `pass` was already closed before authorization later became non-current; one Operational Reliance Policy declares `historical_validity_only` and another declares `continuing_current`. | Engine authorization; Evaluation Coordination; Contracts evaluation-context closure; Operational Reliance and action owners. | The first prevents collection. The second preserves historical evidence but refuses new claim, metric, or gate truth and requires a fresh authorized basis/context. The third preserves the immutable historical `pass` and canonical explanation. Both reliance paths resolve the exact Binding and historical validity; only `continuing_current` requires fresh positive Engine current verification and any policy-required fresh evaluation. Any effect under either path still requires a distinct fresh Action Authorization Binding, expected-target verification, exact attempt, Terminal, and state-owner receipt. | Collection authorization, final-truth authorization, historical validity, policy-selected continuing-current reliance, and fresh action authorization are distinct propositions. |
 | **Approved Context Resolution escalation.** Engine approves a changed envelope after an earlier Prospective Evaluation Basis or final evaluation context was formed. | Engine authorization; separately owned readiness; prospective evaluation formation; Contracts evaluation-context closure. | The approved envelope requires a fresh Evaluation Authorization Binding, readiness determination, Prospective Evaluation Basis, and final evaluation context. Prior evidence and results remain bound to their earlier bases and contexts and are not upgraded. | Escalation is not in-place expansion or retroactive authorization. |
 | **Exact resolved-requirements realization.** One Resolved Assurance Requirements identity for scope `S` selects contract-rooted expected-claim inventory `C`, exact Evidence Evaluation Policy bindings `E`, configured Metric Definitions `M`, and Gate Policy `G`. The valid evaluation uses a subject snapshot inside `S`, one exact policy in each supported required role, and exactly `C/E/M/G`. Comparison cases use a subject outside `S`; substitute an incompatible policy; bind an exact policy to an unsupported role; supply conflicting bindings for one role; change a metric configuration; omit one selected claim; add an unselected claim; or let `G` depend on an unselected claim or metric. | Assurance selection; evaluation-context admission; claim evaluation; metrics; gate. | The exact valid closure may evaluate reproducibly. Every comparison refuses before claim, metric, or semantic-gate truth; none falls back or becomes ordinary `not_applicable`, `not_computable`, or `indeterminate`. | The evaluation context must exactly and completely realize one selected requirements identity. |
 
@@ -382,28 +430,38 @@ Policy roles and compatibility, the one-contract-rooted
 requirements-to-evaluation conformance rule, responsibility map, gate purity
 and precedence, exact Engine-owned evaluation-authorization provenance and
 context-refusal boundary, the pre-observation Prospective Evaluation Basis and
-later evidence-bound evaluation-context closure, historical-result/current-
-operational distinction, complete explanation/accounting requirement, and
-every scenario outcome above. Later work may not reopen those decisions merely
-by choosing a representation.
+later evidence-bound evaluation-context closure, universal historical Binding
+resolution/validity plus the explicit `historical_validity_only` /
+`continuing_current` present-reliance rule, separate fresh action authorization,
+Policy Admission Closure requirements, canonical explanation truth and density-
+projection principles, complete explanation/accounting requirement, and every
+scenario outcome above. Later work may not reopen those decisions merely by
+choosing a representation.
 
 Deliberately deferred to human-approved task-level planning are all concrete
 metric algorithms and catalog entries; schema, type, API, and operation shapes;
 expression language; persistence layout; registry and version-resolution
 mechanics; any cutover or temporary internal bridge; implementation packets;
 acceptance planning; and rollout. Also out of scope are external adopter
-selection, parent-promotion policy, posture algorithms, UI, a metric
-marketplace, and a universal confidence score.
-
-A later human-approved **Evaluation Authorization, Capability, and Readiness**
-semantic decision owns evaluation-purpose formation, authorization issuance
-workflow, Prospective Evaluation Basis lifecycle and physical owner placement,
-the binding's public projection and opaque-lineage detail, currentness
-checkpoints, readiness states, producer capability versus operating ceiling,
-diagnostic intent, retry/cache/supersession/escalation lifecycle, and the
-advisory-CLI versus real host-enforcement boundary. This record selects no API,
-schema, storage, transport, or host/Substrate mechanism, and no separate fifth
-authorization decision is required.
+selection, concrete local-closeout or parent-promotion policy, work hierarchy
+and state model, posture algorithms, UI, a metric marketplace, and a universal
+confidence score.
+
+The proposed
+[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md)
+owns evaluation-purpose formation, authorization issuance, Evaluation
+Coordination, Prospective Evaluation Basis lifecycle, currentness checkpoints,
+readiness, capability/ceiling separation, diagnostics, and retry/cache/
+supersession/escalation semantics. The proposed
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+owns the common present-use rule, atomic eligibility/disposition determination,
+separate action chain, and Work Progression semantic responsibility for local-
+closeout and parent eligibility. Concrete physical placement, work hierarchy and
+state model, admitted policies, schemas, storage, transport, host/Substrate
+mechanism, mutations, implementation, and delivery slices remain deferred. The
+proposal does not silently approve any of them or require a general identity,
+credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
+or ABAC platform.
 
 Human approval of this proposed record would make it semantic input to later
 planning only. It would not by itself authorize implementation, change a phase
diff --git a/docs/specs/handbook-contract-membrane/operational-reliance-and-work-progression-semantic-decision.md b/docs/specs/handbook-contract-membrane/operational-reliance-and-work-progression-semantic-decision.md
index 9c87a60..100b7cb 100644
--- a/docs/specs/handbook-contract-membrane/operational-reliance-and-work-progression-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/operational-reliance-and-work-progression-semantic-decision.md
@@ -88,9 +88,9 @@ Terms in separate rows are not synonyms.
 | **Current-Use Eligibility** | The semantic facet of one Operational Reliance Determination: `eligible`, `ineligible`, `indeterminate`, or `not_applicable`. It answers only whether the bound historical truth may support consideration of the exact action question for the exact target now. It is not independently formed, current, superseded, or replayed apart from its Determination. |
 | **Reliance Refusal** | The owner-produced refusal to form a valid Operational Reliance Basis or Determination because a required identity, policy, target relation, owner resolution, currentness observation, use-intent rule, cardinality, or accounting boundary is missing, malformed, ambiguous, stale where currentness is required, substituted, conflicting, or unverifiable. It is not `ineligible`, `indeterminate`, or `not_applicable`. |
 | **Operational Disposition** | The mechanically fixed control-facing facet of one Operational Reliance Determination: `allow`, `block`, `hold`, or `no_effect`. It is not a second policy decision or independently current result. `allow` permits only an Action Intent and request for separate action authorization; it neither performs nor authorizes the action. |
-| **Action Intent** | The immutable, exactly identified requester-owned request to perform the action question already decided by one exact `eligible / allow` Operational Reliance Determination. It MUST be exactly congruent with that Action Kind, Action Target Snapshot, expected state, effect, effect scope, and every eligibility-relevant parameter. It may add only the actor or principal and execution-local details that the Operational Reliance Policy declares irrelevant to eligibility. It is untrusted until admitted and grants nothing by itself. |
-| **Action Authorization Binding** | The immutable authorization-owner result that binds the exact authorization-owner identity; owner-recognized authorization rule and authority basis; issuance semantic time and owner-verifiable lineage; one exact Action Intent; actor or principal; exact action question; exact `eligible / allow` Operational Reliance Determination; bounded conditions; and the rule governing whether one or more Action Attempts are permitted. It cannot improve eligibility, broaden the Intent, or substitute for another authorization owner. Evaluation authorization cannot substitute for it. |
-| **Action Authorization Refusal** | The immutable authorization-owner result that one exact Action Intent cannot be authorized, with the decisive reason, authority basis, semantic time, and owner provenance. It creates no Action Authorization Binding, Action Attempt Identity, Action Terminal, or Mutation Receipt. |
+| **Action Intent** | The immutable, exactly identified requester-owned request to perform the action question already decided by one exact `eligible / allow` Operational Reliance Determination. It MUST be exactly congruent with that Action Kind, Action Target Snapshot, expected state, effect, effect scope, and every eligibility-relevant parameter. It may add only the actor or principal, claimed delegation or authority provenance, and execution-local details that the Operational Reliance Policy declares irrelevant to eligibility. Those claims remain claims until the action-authorization owner verifies or explicitly admits them under an advisory-authority rule. The Intent is untrusted until admitted and grants nothing by itself. |
+| **Action Authorization Binding** | The immutable authorization-owner semantic result and provenance closure that binds the exact authorization-owner identity; owner-recognized authorization rule; declared authority mode and basis, including whether principal or delegation facts are claimed, owner-verified, or host-enforced; issuance semantic time and owner-verifiable lineage; one exact Action Intent; actor or principal; exact action question; exact `eligible / allow` Operational Reliance Determination; bounded conditions; and the rule governing whether one or more Action Attempts are permitted. It is not itself a credential, capability token, authenticated identity, delegation instrument, or proof of host enforcement. It cannot improve eligibility, broaden the Intent, or substitute for another authorization owner. Evaluation authorization cannot substitute for it. |
+| **Action Authorization Refusal** | The immutable authorization-owner result that one exact Action Intent cannot be authorized, with the decisive reason, declared authority mode and basis, semantic time, and owner provenance. It creates no Action Authorization Binding, Action Attempt Identity, Action Terminal, or Mutation Receipt. |
 | **Action Attempt Identity** | The exact action-executor-owned identity of one deliberately intended execution attempt under one exact Action Intent and Action Authorization Binding. It distinguishes exact delivery replay from a new attempt. It grants no authority and is not the Intent, Binding, Terminal, or Mutation Receipt. |
 | **Action Terminal** | The action-executor-owned immutable durable provenance for one exact Action Attempt Identity under one exact Action Authorization Binding. It records the executor's exact attempt outcome, including a refusal after the attempt boundary was entered, completion without target mutation, execution failure, or effect/commit uncertainty. It may report a committed target-state mutation only by binding the exact authoritative state-owner Mutation Receipt. A pre-attempt Action Authorization Refusal is not an Action Terminal. |
 | **Mutation Receipt** | The authoritative state owner's immutable proof that one exact Action Attempt committed the exact authorized mutation against the exact expected predecessor state and produced the exact successor state at the owner's commit semantic time. No terminal, acknowledgement, transport success, or executor assertion substitutes for it. |
@@ -181,6 +181,36 @@ targets that record lifecycle.
 
 ## Operational Reliance Policy and basis formation
 
+Every Action Kind, Operational Reliance Policy, Action Authorization policy, and
+recovery policy consumed by this chain MUST be an exact category-2 definition
+admitted by its responsible semantic owner. Candidate content MAY be authored by
+a user, an agent, an authored project artifact, or another tool. Authorship
+supplies proposed content, provenance, and organizational judgment; it does not
+perform executable admission.
+
+The definition owner MUST deterministically produce the exact owner-side
+**Policy Admission Closure** defined by the companion
+[canonical-object ontology decision](canonical-object-ontology-and-ownership-semantic-decision.md).
+That closure binds the candidate's exact content; typed vocabulary and
+conformance; exact candidate boundary, selection, and accounting; every relevant
+exact Charter, Project Context, Decision Record, or Risk Record revision with its
+proposition role, scope, and historical, formation-only, or continuing-current
+dependency mode; and, when the candidate's admission rule requires approval or
+exception authority, the exact separately typed and authorized source, its
+authority, scope, identity/version or revision as applicable, and currentness. It
+also establishes compliance with the non-relaxable invariants of this decision.
+
+Admission MUST NOT require every artifact family for every policy, treat a bare
+Markdown link as proof, turn authored prose into runtime evidence or a waiver,
+or ask an AI to make an open-ended organizational judgment about whether a
+candidate is “safe.” Organizational judgment enters through the exact governance
+sources declared by the admission rule. A candidate is mechanically prohibited
+as unsafe only when an admitted constraint or non-relaxable invariant says so,
+and a refusal MUST identify that violated rule. The admission rule is itself an
+exact owner-governed version outside the candidate population being decided;
+changing it creates a successor rule rather than an unbounded requirement that
+each admission recursively admit its own admission process.
+
 An Operational Reliance Policy MUST declare before result direction is used:
 
 - exactly one Action Kind and policy owner;
@@ -541,8 +571,8 @@ Eligibility and authority are deliberately ordered and non-substitutable:
    Reliance Determination with `eligible / allow` facets for one closed action
    question.
 2. The requester forms an exact Action Intent congruent with that action
-   question and adds only the actor or principal and policy-permitted execution-
-   local details.
+   question and adds only the actor or principal, claimed delegation or authority
+   provenance, and policy-permitted execution-local details.
 3. The exact action-authorization owner resolves the Action Intent and either
    issues one exact Action Authorization Binding or one exact Action
    Authorization Refusal. A refusal creates no Binding or Action Attempt.
@@ -568,6 +598,40 @@ Eligibility and authority are deliberately ordered and non-substitutable:
    commit of the exact authorized state change. The Terminal may report
    committed mutation only by binding that Receipt.
 
+### Authority provenance without a Handbook IAM platform
+
+This semantic action chain does **not** require Handbook to build a general
+identity, credential, authentication, delegation, capability-token, secrets,
+PKI, RBAC, or ABAC platform. An Action Authorization Binding is the action-
+authorization owner's exact semantic result and provenance contract. Concrete
+authentication, credential, host-permission, or enforcement mechanisms remain
+owned by the systems that actually provide them. Engine Context Resolution
+retains its separate semantic admission and verification responsibility for
+evaluation reach; neither action authorization nor this clarification replaces
+or weakens it.
+
+A standalone CLI or SDK workflow MAY use explicitly permitted **advisory
+authority**. Under such an admitted action-authorization rule, a user prompt or
+configured agent role may supply action intent, a claimed principal or
+delegation, and declared authority. The Binding MUST preserve that exact source
+and label those propositions as advisory claims rather than independently
+authenticated identity, verified delegation, host-enforced permission, or a
+fine-grained credential. Ambient operating-system permissions are real execution
+capability, but they are not by themselves Handbook proof of the claimed
+principal, delegation, scope, or policy authorization.
+
+Advisory authority may support an effectful Action Attempt only when the exact
+action-authorization policy explicitly permits that authority mode for the
+Action Kind and target. It cannot fabricate verified facts, satisfy a policy
+that requires independently authenticated, owner-verified, or host-enforced
+authority, waive the exact `eligible / allow` Determination, or bypass target,
+currentness, attempt-admission, and through-commit checks. A real enforcing host
+may supply stronger exact facts under its own authority; Substrate is only a
+possible future example and is not selected here. Changing advisory source,
+claimed principal or delegation, declared authority, assurance mode, or any
+verified authority fact requires a new authorization decision and cannot be
+hidden inside delivery replay.
+
 An action authorization does not improve Current-Use Eligibility. An
 ineligible, indeterminate, not-applicable, or refused reliance result cannot be
 overridden by broader action authority. Conversely, an eligible result cannot
@@ -580,11 +644,11 @@ Action Target Snapshot, expected state, effect, effect scope, or any
 eligibility-relevant action parameter already decided by the Operational
 Reliance Determination. A change to any of those values requires a deliberately
 new Operational Reliance Occurrence Identity, Basis, Determination, Action
-Intent, and authorization path. A change only to the actor or principal
-requires a new Action Intent and authorization. A change only to the
-authorization owner's rule, basis, or conditions may require a new
-authorization decision over the unchanged Intent; it does not rewrite that
-Intent.
+Intent, and authorization path. A change only to the actor or principal,
+claimed delegation, declared authority, or its source requires a new Action
+Intent and authorization. A change only to the authorization owner's rule,
+basis, or conditions may require a new authorization decision over the
+unchanged Intent; it does not rewrite that Intent.
 
 Exact delivery replay under the same Action Attempt Identity and exact attempt
 content MUST return the same Action Terminal and any same Mutation Receipt
@@ -687,7 +751,7 @@ existing Operational Reliance Basis for a new present decision:
 | The action policy declares `continuing_current` and Engine reports non-current, superseded, unverifiable, unavailable, ambiguous, or substituted | Reliance formation refuses because the required positive owner verification is absent. It never treats the authorization as current or reconstructs Engine truth. A fresh evaluation is required when the policy requires one. |
 | Work parent/child/dependency membership or relation changes | A prior Parent Eligibility Result remains history but cannot govern the changed closure. Form a new exact parent basis with complete accounting. |
 | Local work state or a required local result changes after Parent Eligibility closes | The parent result remains history. Present parent action stops at final recheck and requires a new parent reliance occurrence. |
-| Action Authorization Binding expires, is superseded, or no longer matches the target/effect before attempt or commit | The already closed Determination remains history; the attempt or mutation refuses. New authority never retroactively changes it. |
+| Action Authorization Binding expires, is superseded, no longer matches the target/effect, or its advisory/verified authority source, claimed principal/delegation, authority mode, or required verification facts change before attempt or commit | The already closed Determination remains history; the attempt or mutation refuses. A new authorization decision is required, and new authority never retroactively changes the Determination. |
 | Target state or a dependency declared current-through-commit changes after `allow` but before mutation | Commit-boundary enforcement refuses the mutation and no Mutation Receipt issues. Recompute reliance against the new target or dependency closure before requesting new authorization. |
 | Exact historical dependency is temporarily unavailable for resolution | Historical results are not rewritten. A display may report the owner-attributed non-success, but operational reliance refuses because the common exact-result floor cannot be established. |
 
@@ -714,19 +778,20 @@ receipt or inferring that an earlier successful action still governs the target.
 | Responsibility | Owns | MUST NOT own or infer |
 | --- | --- | --- |
 | Contracts evaluation responsibilities | Historical applicability, claim, metric, semantic-gate, and canonical-explanation truth for one closed evaluation context. | Operational Reliance Policy, local closeout, parent eligibility, action authorization, work-state mutation, or posture transition. |
-| Engine Context Resolution | Historical and current verification of Evaluation Authorization Bindings and every other declared Engine-owned Context Resolution proposition. | Current-Use Eligibility, action-policy selection, Work Progression truth, or action authorization merely because an evaluation was authorized. |
+| Engine Context Resolution | Historical and current verification of Evaluation Authorization Bindings and every other declared Engine-owned Context Resolution proposition. | Current-Use Eligibility, action-policy selection, Work Progression truth, action authorization merely because an evaluation was authorized, or a general Handbook IAM platform. |
 | Engine artifact/profile domain | Exact Work Specification and other artifact revisions, Artifact Subject Binding, artifact currentness, promotion, and lineage under the authored-artifact lifecycle. | Operational work completion, local/parent eligibility, action disposition, or action authority from artifact status or prose. |
 | Authoritative work-structure and work-state owner | Exact work identities, current state snapshots, parent/child/dependency relations, state currentness, concurrency, and committed successor state. | Contracts truth, Work Progression policy or eligibility, action authorization merely from state ownership, or mutation without exact authorization. |
+| Policy-definition and admission owners | Exact candidate population, deterministic Policy Admission Closure, typed vocabulary and conformance, relevant governance-source roles/scope/currentness, required separately authorized approvals or exceptions, non-relaxable invariant checks, admission/refusal, currentness, and supersession for each Action Kind, reliance, authorization, or recovery policy. | Treating candidate authorship, a bare link, rendered prose, an AI's unconstrained judgment, or a policy's own desired outcome as admission; converting authored text into runtime evidence or a waiver; or recursively letting a candidate admit its own admission rule. |
 | Work Progression | Work target and finite parent/dependency closure for its decisions; Local Closeout and Parent Eligibility policies; reliance occurrence, basis, atomic Determination with eligibility/disposition facets, currentness, and replay. | Contracts truth, evaluation authorization, Work Specification authorship, release/deployment policy, action authorization by implication, or mutation. |
 | Other downstream action domains | Their exact action-specific reliance policies, bases, atomic Determinations, currentness, and replay under the common floor. | A universal enforcement policy, another action domain's policy, Contracts truth, or Engine authority. |
 | Action requester or origin owner | Exact Action Intent request content and durable request occurrence provenance. | Action Kind or target truth, eligibility, authorization, attempt, or effect merely because it requested one. |
-| Action authority owner | Exact authorization rule and authority basis, actor/principal, Action Kind, target, expected state, bounded effect, Operational Reliance Determination binding, issuance time and lineage, refusal, Binding, currentness, supersession, and permitted-attempt rule. | Improving eligibility, rewriting a gate, treating Evaluation Authorization as action authorization, or proving an attempt or effect. |
+| Action authority owner | Exact authorization rule; declared advisory, owner-verified, or host-enforced authority mode and basis; exact provenance of claimed versus verified principal/delegation facts; Action Kind, target, expected state, bounded effect, Operational Reliance Determination binding, issuance time and lineage, refusal, Binding, currentness, supersession, and permitted-attempt rule. | Improving eligibility, rewriting a gate, treating Evaluation Authorization as action authorization, claiming authentication/delegation/enforcement not established by its exact inputs, mandating a general Handbook IAM implementation, or proving an attempt or effect. |
 | Action executor | Action Attempt Identity, attempt-admission checks from exact owner facts, exact authorized execution, one Action Terminal per attempt, delivery replay, failure, refusal after the attempt boundary, and effect/commit uncertainty. | Self-producing another owner's authorization/currentness, broadening the effect, or asserting committed state change without the state-owner receipt. |
 | Authoritative state or effect owner | Exact expected predecessor, through-commit verification, atomic state change or effect decision, durability, concurrency, exact successor, commit semantic time, and Mutation Receipt. | Inventing eligibility, broadening authority, changing policy, or reporting an uncertain commit as success. |
 | Posture observation owner | Exact policy-driven admission of permitted historical results as immutable Posture Observations. | Gate reinterpretation, recommendation by implication, or Engine posture mutation. |
 | Posture recommendation owner | Exact policy-driven recommendation from admitted observations. | Observation self-admission, approval, or transition. |
 | Engine posture-transition owner | Fresh authority/currentness/approval checks and atomic transition from one exact posture authority state to its successor. | Rewriting the source gate, eligibility, disposition, or recommendation. |
-| SDK, CLI, Tauri, Flow, Pipeline, skills, and transports | Bounded composition, presentation, deterministic projections/assemblies, correlation, scheduling, and transport-local errors around exact owner objects. | Policy selection, current-use formation, disposition reinterpretation, action authorization, or mutation merely because they carry or invoke an operation. |
+| SDK, CLI, Tauri, Flow, Pipeline, skills, and transports | Bounded composition, presentation, deterministic projections/assemblies, correlation, scheduling, transport-local errors, and exact carriage of prompt- or role-supplied advisory-authority claims around owner objects. | Authenticating a claimed principal/delegation, fabricating host enforcement, satisfying an owner policy that requires stronger facts, policy selection, current-use formation, disposition reinterpretation, action authorization, or mutation merely because they carry or invoke an operation. |
 
 Physical colocation does not collapse these owners. In particular, a future
 Engine-hosted Work Progression implementation would still consume rather than
@@ -762,6 +827,9 @@ separate from any Engine action-authority or mutation responsibility.
 | **Historical replay after policy supersession.** An old Basis, Determination, and Action Terminal are replayed after the Operational Reliance Policy has a successor. | Exact replay returns the old historical results and policy identity. It grants no current disposition or authority under the successor; present use requires a new occurrence. | Historical reconstruction and present reliance are distinct. |
 | **Exact attempt replay.** The same Action Attempt Identity and exact attempt content are delivered again after its Terminal and Mutation Receipt closed. A comparison case changes the effect under that attempt identity. | Exact replay returns the same Terminal and Receipt without re-executing. The comparison refuses conflicting identity reuse. | Attempt replay and a deliberately new execution are distinct. |
 | **Transport reports success without receipt.** A CLI or host reports a successful call, but the state owner cannot establish whether the write committed and emits no Mutation Receipt. | The effect is uncertain and MUST NOT be presented as a successful mutation. Eligibility and authorization cannot repair the missing state-owner truth. | Transport success and action intent are not durable mutation truth. |
+| **Standalone advisory authority is explicitly permitted.** A local CLI action policy permits advisory authority for one exact Action Kind. A user prompt supplies the intent, claimed principal, claimed delegation, and declared authority; the process also has ambient operating-system permission to write the target. | The action-authorization owner may issue a Binding that preserves the prompt provenance and labels the authority facts advisory. The attempt still requires the exact `eligible / allow` Determination, target-currentness and attempt checks, and through-commit state-owner verification and Receipt. Neither the Binding nor ambient OS permission is represented as independent authentication, verified delegation, or fine-grained Handbook credential. | Advisory authority can participate in the complete effectful chain without fabricating stronger assurance or bypassing mutation safeguards. |
+| **Prompt offered where enforced authority is required.** Another Action Authorization policy requires an independently authenticated principal and host-enforced permission. A prompt or configured agent role declares both but supplies no owner-verifiable fact. | Authorization refuses. The declared claims cannot satisfy the stronger policy, and advisory mode cannot waive the requirement or target/currentness checks. | Policy-required enforcement cannot be fabricated by advisory inputs. |
+| **Agent-authored policy candidate.** An agent proposes an Operational Reliance Policy and cites one Decision Record by Markdown link, omits the exact scope and candidate population, and asks the admission owner to judge whether it is generally safe. | Policy admission refuses. A valid Policy Admission Closure requires exact candidate content, typed roles and scope, relevant exact governance-source revisions and dependency modes, complete selection/accounting, and mechanical invariant checks. It neither requires every artifact family nor turns the Decision Record into runtime evidence or a waiver. | Candidate authorship and organizational rationale are permitted, but deterministic owner admission is separate and bounded. |
 
 ## Settled versus deferred
 
@@ -806,6 +874,14 @@ This record settles:
   recovery with fresh authorization;
 - explicit placement of every introduced object in the canonical six-category
   ontology without creating another Artifact Instance family;
+- deterministic owner-side Policy Admission Closure for user-, agent-, artifact-,
+  or tool-authored policy candidates, with exact relevant governance inputs,
+  selection/accounting, mechanical non-relaxable checks, and no bare-link proof,
+  authored-text waiver, open-ended AI safety judgment, or recursive self-admission;
+- Action Authorization Binding as a semantic result/provenance contract rather
+  than a mandate for a general Handbook IAM platform, including explicitly
+  policy-permitted advisory authority and its non-substitution for authenticated,
+  owner-verified, or host-enforced facts;
 - Work Specification reference and causal-order boundaries;
 - the gate-to-posture observation/recommendation/transition boundary; and
 - currentness, invalidation, retry, exact replay, and scenario outcomes above.
diff --git a/docs/specs/handbook-contract-membrane/phase-and-slice-map.md b/docs/specs/handbook-contract-membrane/phase-and-slice-map.md
index 3f62ea8..6802ecc 100644
--- a/docs/specs/handbook-contract-membrane/phase-and-slice-map.md
+++ b/docs/specs/handbook-contract-membrane/phase-and-slice-map.md
@@ -90,9 +90,12 @@ atomic posture transition while rehoming the residual work to later owners.
 
 - Gate-owned local-closeout evaluation formerly associated with the grounding
   slice moved to the later evidence/verdict/gate owner and is now implemented
-  by HCM-5.2. Historical parent-promotion eligibility remains a separate future
-  dependency/workflow-composition boundary with no selected parent domain,
-  binding, policy, owner, or consumer.
+  by HCM-5.2. No parent-eligibility result or consumer is implemented there. The
+  proposed [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+  assigns greenfield semantic responsibility for local-closeout and parent
+  eligibility to Work Progression, without approving or scheduling physical
+  placement, a work hierarchy/state model, exact parent binding, concrete policy,
+  consumer, action/mutation path, or implementation slice.
 - Ordinary SDK/JSON exposure, publication, bridging, and Substrate adoption for
   grounding move to the appropriate Phase-4 and Phase-6 owners.
 - Multi-source posture evaluation, recommendations, notifications,
@@ -199,8 +202,8 @@ actual owner boundary. Unselected external consumers are not an exit failure.
 | External grounding trigger/adopter placement | HCM-4.6 implements the bounded owner, repository, four-operation seam, fixed SDK composition, and unchanged generic repository consumers. Future Phase-6 authority must separately choose any publication, bridge, automatic trigger host, and real external adopter. |
 | Broader posture evaluation, recommendation, notification, acknowledgement, and hysteresis | Future posture authority must place this policy without transferring engine-owned mutation authority to a gate or adapter |
 | CLI operation-inventory expansion beyond the completed 62-row predecessor | HCM-4.3 does not expand the inventory for transport parity: its authority fixes the exact ordinary mappings and a closed six-entry SDK use-case registry. Any new semantic operation or use case requires later explicit authority, complete schemas, and independent owner admission where applicable. |
-| Parent-promotion eligibility and dependency-composition boundary | HCM-5.2 fixes gate and local-closeout truth without a parent-promotion field or consumer. Future authority must first select the parent model, subject-to-parent binding, dependency/workflow owner, policy and currentness inputs, and consumer. Any selected eligibility result must be separate, fail closed, and remain non-mutating; local closeout alone cannot imply it, and actual parent mutation requires still-separate authority. |
-| Gate-to-posture input and policy boundary | HCM-5.2 fixes gate and local-closeout truth without posture fields; future posture authority must separately decide whether and how to consume those results without transferring posture policy or mutation authority to Contracts, SDK, or an adapter. |
+| Parent eligibility, work hierarchy/state, and dependency-composition realization | HCM-5.2 truthfully fixes gate and local-closeout results without a parent field or consumer. The proposed [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md) selects Work Progression as the greenfield semantic owner of local/parent eligibility and requires separate atomic Operational Reliance Determinations. Later human-approved authority must still select the physical component, work hierarchy/state model, exact parent/child/dependency binding, concrete policies/currentness inputs, consumers, action authorization/mutation mechanisms, implementation slice, and rollout. Local closeout cannot imply parent eligibility or mutation. |
+| Gate-to-posture observation, recommendation, reliance, and transition realization | HCM-5.2 fixes gate/local-closeout truth without posture fields. The proposed [posture and assurance-policy decision](posture-assurance-policy-semantic-decision.md) fixes the forward-only semantic order through admitted observation/recommendation, transition-specific Operational Reliance, separate Action Intent/Authorization, and atomic Engine mutation. Later authority must still select concrete policies, algorithms, approvals, physical placement, schemas/APIs, host integration, implementation, and rollout without transferring gate or mutation authority to Contracts, SDK, or an adapter. |
 | Third-party dock admission and stronger host confinement | Later explicit authority after the HCM-5.3/HCM-5.4 repository-owned first-party path; neither prepared slice authorizes arbitrary-code execution or claims a general hostile-code boundary |
 | CLI/Tauri membrane routes and framing | HCM-4.3 owns current CLI parity and framing; HCM-4.4 separately owns the Tauri facade after HCM-4.3 closeout. |
 | Publication mechanism, exact versions, bridge, and external adopter | Future publication and consumer authorities with current external proof |
diff --git a/docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md b/docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md
index 0846b97..928d40b 100644
--- a/docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md
+++ b/docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md
@@ -9,11 +9,18 @@ requirements. It is semantic authority for human review only. It is not
 implementation authority, a slice or packet specification, or rollout
 planning.
 
-The companion
-[metric, evaluation, and gate semantic decision](metric-gate-semantic-decision.md)
-owns metric and gate meaning. This record owns the earlier selection seam: how
-an exact resolved posture may select exact assurance requirements that metrics
-and gates later consume without reinterpreting posture.
+This record is a companion to the proposed
+[canonical-object ontology and ownership decision](canonical-object-ontology-and-ownership-semantic-decision.md),
+[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md),
+[evaluation identity and evidence-closure decision](evaluation-identity-evidence-closure-semantic-decision.md),
+[authored-artifact and evaluation-relationship decision](authored-artifact-evaluation-relationships-semantic-decision.md),
+[metric, evaluation, and gate decision](metric-gate-semantic-decision.md), and
+[operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md).
+The metric record owns metric and gate meaning. This record owns the earlier
+selection seam: how an exact resolved posture may select exact assurance
+requirements that metrics and gates later consume without reinterpreting
+posture. The operational-reliance record owns the later present-use and action
+seam. All seven records remain proposals and supply no implementation authority.
 
 ## Canonical model and vocabulary
 
@@ -32,8 +39,13 @@ explicit Charter posture + exact scope
   -> final Contracts evaluation-context closure
   -> claim evaluations
   -> named metrics
-  -> semantic gate decision
-  -> separately owned enforcement disposition
+  -> semantic gate decision and canonical explanation
+  -> action-specific Operational Reliance Basis, when used operationally
+  -> one atomic Operational Reliance Determination
+       - Current-Use Eligibility facet
+       - fixed Operational Disposition facet
+  -> separate Action Intent and Action Authorization Binding or Refusal
+  -> Action Attempt, Action Terminal, and state-owner Mutation Receipt only after committed mutation
 ```
 
 The following terms are normative. Terms in different rows are not synonyms.
@@ -56,11 +68,11 @@ Classification already names a different concept.
 | **Evidence Evaluation Policy** | The immutable, named, versioned Contracts-owned role-specific policy defined by the companion record. Assurance selection may bind an exact policy only to an evidence-evaluation role and compatibility constraint declared by the selected contract, claim, or evidence requirement. It cannot alter canonical-admission trust, the claim proposition or finding scale, Metric Definition meaning, or Gate Policy meaning. |
 | **Metric Definition** | The immutable, named, versioned, fully configured algorithm definition owned by the companion record. Assurance selection may choose an exact definition reference; it MUST NOT change the definition's scale, unit, population, algorithm, parameters, missing-input policy, or numeric meaning. |
 | **Gate Policy** | The immutable semantic decision rule owned by the companion record. Assurance selection may choose an exact policy reference; gate evaluation remains a pure application of that resolved policy to compatible claim evaluations and metric results. |
-| **Enforcement Policy** | A separately owned, named, versioned rule that consumes an exact semantic gate decision, its Resolved Posture Snapshot, and the required evaluation provenance to determine an operational disposition. It cannot alter semantic gate truth or authorize posture mutation by implication. |
-| **Enforcement Disposition** | The non-semantic operational interpretation produced by an Enforcement Policy, such as whether the separately owned workflow proceeds, blocks, or holds. Different enforcement policies may produce different dispositions from the same gate truth without changing that truth. |
+| **Enforcement Policy** | An action-domain Operational Reliance Policy for an enforcement, workflow, or posture action question. It consumes the exact historical gate result and its declared target/currentness inputs to produce one atomic Operational Reliance Determination. It cannot alter gate truth, grant action authority, or authorize posture mutation by implication. |
+| **Enforcement Disposition** | The Operational Disposition facet of that one Operational Reliance Determination: `allow`, `block`, `hold`, or `no_effect`, mechanically paired with its Current-Use Eligibility facet. It is not an independently issued, current, supersedable, or replayable result. A Disposition Reference identifies the shared Determination and its exact Operational Disposition facet. |
 | **Posture Observation** | An immutable, provenance-bound, admitted fact that may inform later posture reasoning, including an exact gate result when a named observation policy permits it. An observation is neither a recommendation nor authority to mutate posture. |
 | **Posture Recommendation** | A separately owned, explainable proposal derived from admitted observations and a named recommendation policy. It identifies a proposed transition but cannot apply it, and its production does not imply approval. |
-| **Posture Transition** | The separately authorized, atomic Engine-owned mutation from one exact current posture authority state to the next. It performs fresh authority and currentness checks and creates the next posture state; neither an Assurance Policy, gate, observation, recommendation, nor enforcement disposition may perform it. |
+| **Posture Transition** | The separately authorized, atomic Engine-owned mutation from one exact current posture authority state to the next. Before transition intent or authorization, one transition-specific Operational Reliance Determination MUST establish `eligible / allow` for the exact expected posture target, bounded transition, and scope. A congruent Action Intent, separate Action Authorization Binding, exact Action Attempt Identity, commit-time checks, Action Terminal, and Engine state-owner Mutation Receipt then govern the mutation. Neither an Assurance Policy, gate, observation, recommendation, nor reliance disposition performs or authorizes it. |
 
 ## Current product facts and semantic gap
 
@@ -213,6 +225,34 @@ mapping.
   Resolved Assurance Requirements and evaluation contexts. This record does
   not authorize cross-context gate composition.
 
+### Policy candidates require deterministic owner admission
+
+A user, agent, authored artifact, or tool MAY author an Assurance Policy
+Selector, Assurance Policy, Evidence Evaluation Policy, Metric Definition, Gate
+Policy, Enforcement/Operational Reliance Policy, posture-observation policy,
+recommendation policy, or exception-policy candidate. Candidate authorship may
+supply organizational judgment, rationale, claimed provenance, and proposed
+content. It does not make the candidate executable, independently authenticate
+an author, or waive an invariant.
+
+Each responsible semantic owner MUST deterministically produce the category-3
+**Policy Admission Closure** defined by the companion
+[canonical-object decision](canonical-object-ontology-and-ownership-semantic-decision.md)
+before a candidate becomes an admitted category-2 definition or policy. The
+closure binds the exact candidate; typed vocabulary and conformance; finite
+candidate selection/accounting; proposition roles; exact scope; and every
+relevant exact Charter, Project Context, Decision Record, or Risk Record revision
+with its historical, formation-only, or continuing-current dependency mode.
+Where an admission rule requires approval or exception authority, the closure
+separately binds the exact typed, authorized source, its authority, scope,
+identity/version or revision as applicable, and currentness; a bare Markdown link
+or approval prose is insufficient proof. The admission rule requires only
+relevant governance families, does not turn authored text into runtime evidence
+or a waiver, and is a non-recursive root outside the candidate population it
+decides. “Unsafe” means mechanically prohibited by admitted constraints, not an
+AI's open-ended organizational judgment. Governance inputs supply organizational
+judgment without requiring a fresh human prompt for every policy.
+
 This is the cross-boundary invariant shared with the
 [metric/gate record](metric-gate-semantic-decision.md): **posture may select
 which exact assurance requirements govern a run, but it may never alter the
@@ -318,10 +358,17 @@ P_n Resolved Posture Snapshot
   -> evidence collection and admission
   -> final Contracts evaluation-context closure
   -> metrics / gate
-  -> semantic gate decision
+  -> semantic gate decision and canonical explanation
   -> optional admitted Posture Observation
   -> separately owned Posture Recommendation
-  -> separately authorized Engine Posture Transition
+  -> transition-specific Operational Reliance Basis
+  -> one Operational Reliance Determination for the exact posture transition
+       - Current-Use Eligibility facet
+       - fixed Operational Disposition facet
+  -> exact Posture Transition Action Intent, only after `eligible / allow`
+  -> Action Authorization Binding or Action Authorization Refusal
+  -> exact Action Attempt Identity and commit-bound Engine mutation checks
+  -> Action Terminal + Engine state-owner Mutation Receipt after committed change
   -> P_{n+1}
 ```
 
@@ -346,20 +393,52 @@ P_n Resolved Posture Snapshot
 - Gate `pass`, `fail`, `indeterminate`, or `not_applicable` MUST NOT
   automatically raise or lower posture. A permitted gate result may become an
   admitted Posture Observation only through its separately owned policy.
-- A Posture Recommendation remains non-mutating. The Engine transition owner
-  MUST independently establish current authority, exact expected posture,
-  approvals, and transition validity.
-
-## Enforcement separation
-
-Enforcement is downstream of semantic gate truth. An Enforcement Policy MAY
-consume an exact gate decision, the exact Resolved Posture Snapshot, the exact
-Resolved Assurance Requirements, and other inputs it declares. It MUST NOT
-change a gate decision, replace `indeterminate` with `pass`, treat a disposition
-as semantic evidence, or acquire posture-transition authority. The same gate
-truth may legitimately yield different dispositions under different exact
-Enforcement Policies; the explanation must preserve both identities and the
-unmodified gate result.
+- A Posture Recommendation remains non-mutating. Before any transition intent,
+  one transition-specific Operational Reliance Basis and Determination MUST bind
+  the exact expected posture target, proposed transition, current semantic time,
+  historical results, target/currentness facts, policy, and complete accounting.
+  Only `eligible / allow` permits formation of a congruent Action Intent.
+- The Engine transition-authorization owner MUST then independently produce an
+  Action Authorization Binding or Refusal for that exact Intent. Policy-permitted
+  standalone CLI/SDK flows MAY carry advisory authority from a prompt or
+  configured agent role as claimed principal, delegation, and declared authority
+  provenance. Those claims are not independently authenticated identity,
+  verified delegation, or host-enforced permission; ambient OS permissions are
+  not fine-grained Handbook credentials. Advisory authority cannot satisfy a
+  policy that requires stronger enforcement or waive expected-target,
+  currentness, approval, transition-validity, attempt, or commit-time checks. A
+  real enforcing host may supply stronger facts. No general Handbook identity,
+  credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
+  or ABAC platform is implied.
+
+## Operational reliance and enforcement separation
+
+Enforcement is downstream of immutable semantic gate truth. Each admitted
+Enforcement Policy is an Operational Reliance Policy for one exact action
+question and target. It MAY consume the exact gate decision, Resolved Posture
+Snapshot, Resolved Assurance Requirements, and other declared inputs, but it
+MUST form one exact Operational Reliance Basis and one atomic Operational
+Reliance Determination with paired Current-Use Eligibility and Operational
+Disposition facets. It MUST NOT change the gate, replace `indeterminate` with
+`pass`, treat either facet as semantic evidence, or acquire action or posture-
+transition authority.
+
+Every present reliance universally resolves the exact Evaluation Authorization
+Binding and Engine-confirmed historical validity. The admitted policy MUST then
+select exactly one Engine continuing-status mode: `historical_validity_only` or
+`continuing_current`. Only `continuing_current` requires fresh positive Engine
+current verification at the reliance semantic time and any resulting policy-
+required fresh evaluation. Neither mode weakens mandatory current authorization
+at collection or final evaluation closure. A later effect always requires a
+fresh separate Action Authorization Binding and exact target/currentness checks.
+
+The same gate truth may legitimately yield different atomic Determinations
+under different exact policies or action questions. Each Determination preserves
+the shared historical gate identity, its own policy/action/target/currentness
+closure, both inseparable facets, and a Disposition Reference to its exact
+Operational Disposition facet. The canonical gate explanation remains unchanged; operational reliance
+adds downstream accounting and does not reevaluate truth or choose an
+explanation-density view.
 
 ## Responsibility map
 
@@ -367,16 +446,19 @@ unmodified gate result.
 | --- | --- | --- |
 | Charter authoring and governance | Project authority authors and approvers own Project Classification, persisted baseline and dimension values, descriptive guidance, and authorized Charter changes. | Executable scope matching, assurance selection by prose, metric meaning, gate truth, or automatic posture mutation. |
 | Posture resolution | The posture-resolution owner resolves exact scope and produces the immutable Resolved Posture Snapshot from current admitted Charter authority. | Classification-based override, assurance-policy invention, evidence interpretation, gate truth, or transition approval. |
-| Assurance-policy authority | Establishes and resolves named/versioned selectors and Assurance Policies and produces Resolved Assurance Requirements with complete explanation. | Posture mutation, canonical evidence admission, metric reinterpretation, gate evaluation, or enforcement. |
+| Policy-definition and admission owners | Accept user-, agent-, artifact-, and tool-authored candidates and deterministically produce a Policy Admission Closure or refusal against exact typed vocabulary, relevant governance revisions/proposition roles/scopes/dependency modes, finite accounting, required separately authorized approvals/exceptions, and non-relaxable invariants. The assurance-policy authority then resolves admitted selectors/policies and produces Resolved Assurance Requirements with complete explanation. | Treating authorship, prompts, agent roles, artifact promotion, bare links, approval prose, or favorable outcomes as admission; demanding every artifact family; turning governance text into runtime evidence/waiver; recursive self-admission; posture mutation; or gate evaluation. |
 | Engine Context Resolution authorization | Engine alone admits the exact full envelope, establishes request-within-envelope and non-expansion, issues the purpose-bounded Evaluation Authorization Binding, and verifies its lineage, authority/currentness, historical validity, and supersession. | Assurance-policy selection or relaxation, producer capability or readiness, evidence adequacy, gate truth, explanation density, enforcement, or posture transition. |
-| Evaluation authorization, capability, and readiness | Owner placement remains deferred. The later semantic decision must assign comparison of the already resolved requirements, exact Engine authorization, requested reach, and admitted producer/operating facts, plus Prospective Evaluation Basis formation, to determine whether the exact proof plan can proceed. | Granting Engine authority, selecting an easier policy, weakening requirements, establishing evidence or gate truth, or host enforcement by implication. |
+| Contracts-domain Evaluation Coordination | The proposed Contracts-domain coordination responsibility compares already resolved requirements, exact Engine authorization, requested reach, and admitted producer/operating facts; owns readiness and Prospective Evaluation Basis formation; and determines whether the exact proof plan can proceed. Concrete crate, module, API, persistence, and transport placement remain deferred. | Granting Engine authority, selecting an easier policy, weakening requirements, establishing evidence or gate truth, or host enforcement by implication. |
 | Exception governance, authority, and admission | Charter governance owns non-executable nominations and process declarations. The designated exception-policy authority owns admitted exception semantics, permitted issuers, scope, effects, record requirements, and currentness; the exact record authority authorizes each record; and the exception-admission owner establishes authority, identity, scope, currentness, and policy conformance before selection may apply the bounded effect. | Treating Charter prose, approver names, process metadata, record location, or minimum fields as executable authority; allowing assurance selection to mint, approve, broaden, infer, or self-authorize an exception. |
 | Contract, evidence-policy, evidence, and claim evaluation | The respective semantic owners admit exact contract and evidence authority, own compatible role-specific Evidence Evaluation Policy semantics, and establish applicability and claim truth under the resolved requirements. | Rereading posture, inventing or substituting policy bindings, changing the selected requirements or claim meaning, gate disposition, recommendation, or posture mutation. |
 | Metric computation and gate evaluation | The owners defined by the companion record purely apply exact Metric Definitions and Gate Policy inside the closed evaluation context. | Assurance reselection, mutable-posture interpretation, enforcement, recommendation, or mutation. |
-| Enforcement | A separately named Enforcement Policy owner maps preserved gate truth and exact context to an Enforcement Disposition. | Rewriting semantic gate truth or directly changing posture. |
-| Posture observation | A separately owned admission boundary establishes immutable, provenance-bound observations from permitted sources. | Recommendation, approval, or mutation merely because an observation exists. |
-| Posture recommendation | A separately owned recommendation policy derives an explainable proposal from admitted observations. | Applying, approving, or implying a transition. |
-| Posture transition | The Engine posture-transition owner performs the separately authorized atomic mutation and creates the next authority state. | Recomputing prior gate truth or treating a gate result, observation, or recommendation as self-authorizing. |
+| Work Progression and other Operational Reliance owners | Apply one admitted action-specific policy to exact historical results, target/currentness observations, and complete accounting; universally resolve historical Evaluation Authorization Binding validity; select `historical_validity_only` or `continuing_current`; and produce one atomic Determination with paired eligibility/disposition facets or a Reliance Refusal. Work Progression is the proposed semantic owner for local-closeout and parent eligibility. | Rewriting gate truth or canonical explanation, issuing action authority, mutating state, or silently selecting work hierarchy/state policy or physical placement. |
+| Posture observation | A separately owned admission boundary establishes immutable, provenance-bound observations from permitted sources. | Recommendation, reliance, approval, or mutation merely because an observation exists. |
+| Posture recommendation | A separately owned recommendation policy derives an explainable proposal from admitted observations. | Applying, approving, forming transition eligibility by itself, or implying a transition. |
+| Posture-transition reliance owner | Forms the exact transition-specific Operational Reliance Basis and one atomic Determination before transition intent, with exact expected posture target, bounded transition, currentness, policy, and accounting. | Treating recommendation as eligibility, issuing action authority, mutating posture, or changing historical gate truth. |
+| Transition requester and action-authorization owner | The requester forms one congruent Posture Transition Action Intent only after `eligible / allow`. The authorization owner independently produces an Action Authorization Binding or Refusal with exact advisory, verified, or host-enforced authority provenance and fresh conditions. | Treating reliance, Evaluation Authorization, a prompt, ambient OS permission, or a recommendation as authenticated identity, verified delegation, host enforcement, or a reason to mandate a general Handbook IAM platform. |
+| Engine posture-transition executor and state owner | Owns one exact Action Attempt Identity and Action Terminal; enforces the same authorization, expected posture, exact transition, approvals, and declared currentness at the atomic mutation boundary; and alone issues the Mutation Receipt for committed `P_{n+1}`. | Ordinary retry after uncertain commit, broadening the authorized transition, recomputing prior gate truth, or reporting committed posture without the authoritative receipt. |
+| SDK, CLI, host, and other transports | Preserve exact owner results and provenance, including policy-permitted prompt- or configured-role-supplied advisory claims; a real enforcing host may supply stronger independently verified facts. | Becoming a semantic owner by transport, fabricating authentication/delegation/enforcement, waiving target/currentness checks, or satisfying enforcement-required policy with advisory claims. |
 
 ## Discriminating scenario corpus
 
@@ -392,12 +474,14 @@ the row names a failure.
 | **Selected proof plan exceeds current reach.** A higher posture selects a proof plan requiring observation reach that the current Engine authorization, producer capability, or Dock operating ceiling cannot support. | The exact requirements remain selected and unchanged. Readiness or Prospective Evaluation Basis formation reports or refuses the inability to proceed before collection; no lighter Assurance Policy, omitted requirement, or implied waiver is selected. | Current authorization and capability cannot weaken the engineering standard. |
 | **Posture changes mid-run.** A run closes on snapshot `P_n`; an authorized transition creates `P_{n+1}` before the gate finishes. | The existing Prospective Evaluation Basis and final context remain bound to `P_n` and its resolved requirements. They do not reselect. A fresh run under `P_{n+1}` resolves a new snapshot, requirements, authorization, basis, and final context; the two contexts cannot be combined. | Runs bind one immutable posture snapshot. |
 | **Approved Context Resolution escalation.** The proof plan remains fixed, but Engine approves a broader or otherwise changed envelope after the first authorization binding was issued. | The approval produces a new authorization basis and requires a fresh Evaluation Authorization Binding, readiness determination, Prospective Evaluation Basis, and final evaluation context. The Resolved Assurance Requirements and all prior evidence, result truth, and explanation remain unchanged and bound to their original bases and contexts. | Escalation changes granted reach, not the proof plan or historical truth. |
-| **Failed gate feeds posture reasoning.** A gate returns `fail`, and a named observation policy admits that exact result. A recommendation policy proposes raising a relevant dimension. | The gate truth remains `fail`; the admitted observation and recommendation are non-mutating. Only a separately authorized Engine transition can create the next posture. | Feedback does not create a causal mutation shortcut. |
+| **Failed gate feeds posture reasoning.** A gate returns `fail`, and a named observation policy admits that exact result. A recommendation policy proposes raising a relevant dimension. | The gate truth remains `fail`; the admitted observation and recommendation are non-mutating. A transition-specific Operational Reliance Basis and atomic Determination must first establish `eligible / allow` for the exact expected posture and transition. Only a congruent Action Intent, separate Action Authorization Binding, exact attempt, commit-time Engine checks, Terminal, and state-owner Mutation Receipt can create `P_{n+1}`. | Feedback does not create a causal mutation or authority shortcut. |
 | **Relevant-dimension monotonicity.** Within the same policy family/version and exact scope, all non-posture inputs and other dimensions are fixed while one declared relevant dimension increases. | The newly resolved requirements preserve or strengthen every prior requirement under the policy's declared strength relation; none is removed or weakened. | Monotonicity is bounded and dimension-relevant. |
 | **Incomparable posture vectors.** For the same policy and scope, vector A is higher in security while vector B is higher in operability, and both dimensions are relevant. | Neither vector is declared globally stricter. The exact policy resolves each independently without scalar collapse; any unsupported combination refuses. | A dimension vector has no global total order. |
 | **Missing policy mapping.** A valid snapshot and exact scope have no supported Assurance Policy mapping, or two selectors tie without deterministic precedence. | Selection refuses and produces no Resolved Assurance Requirements. It does not choose a default, nearest, lowest, or highest policy. | Missing or ambiguous mapping fails closed. |
-| **Different enforcement, same truth.** The same exact semantic gate decision and context are consumed by two different named/versioned Enforcement Policies. | The exact semantic gate decision and canonical explanation remain unchanged. The policies may produce different dispositions, each attributed to its own identity, without rewriting the gate. | Enforcement is separate from gate semantics. |
+| **Different enforcement, same truth.** The same exact semantic gate decision and context are consumed by two different admitted named/versioned Enforcement Policies for distinct action questions or targets. | The exact semantic gate decision and canonical explanation remain unchanged. Each policy closes its own Operational Reliance Basis and atomic Determination; different paired eligibility/disposition facets are possible and each Disposition Reference identifies its shared Determination and exact Operational Disposition facet. Neither result authorizes an effect. | Operational reliance is separate from gate semantics, and eligibility/disposition are inseparable facets rather than independent results. |
 | **Free-form shortcut and red line.** Charter text says a shortcut is allowed for a named area and a red line applies to another, but no admitted typed exception policy and record binds the exact scope. A second input is typed but was issued by an unauthorized actor; a third was synthesized or approved by the selector itself. | The prose and both invalid records refuse and do not change requirement selection. Only an exact record that was separately authorized and admitted under the exact exception policy may have its bounded effect applied. | Governance prose, typing alone, and selector self-authorization are not exception authority. |
+| **User- or agent-authored policy candidate.** A Decision Record contains organizational rationale and a user or configured agent supplies candidate Assurance Policy text. The submission cites the record by a bare Markdown link, omits exact proposition role/scope/currentness, and has no owner-produced Policy Admission Closure. | The candidate remains governance input and no executable Assurance Policy exists. The semantic owner must deterministically admit or refuse the exact candidate against typed vocabulary, finite candidate accounting, relevant exact governance revisions and dependency modes, required separately authorized approvals/exceptions, and non-relaxable invariants. | Authorship and organizational judgment are permitted; artifact promotion, links, prompts, and agent roles do not perform policy admission or become runtime evidence/waivers. |
+| **Advisory CLI posture-transition request.** After an exact transition-specific Determination yields `eligible / allow`, a standalone CLI request carries a prompt-supplied claimed principal and delegation under an action-authorization policy that permits advisory authority. A comparison policy requires host-enforced authority. | Under the advisory policy, the authorization owner may issue a Binding that truthfully records the claims and advisory basis, but it does not assert authenticated identity, verified delegation, or host enforcement; all expected-target, currentness, approval, attempt, and commit checks still apply. Under the enforcing policy, the same advisory claims cannot satisfy authorization and the request refuses absent the required stronger facts. | Advisory authority may participate in the full action chain without fabricating verified facts, waiving safeguards, or creating a general Handbook IAM mandate. |
 | **Deterministic requirement replay.** The exact Charter identity/revision, scope, snapshot fingerprint, selector identity/version/content/currentness, admitted policy-resolution basis/currentness, selected-policy identity/version/content/currentness, admitted exception inputs, and other declared inputs are replayed. A comparison case uses a different policy or resolution basis that yields apparently equivalent requirement references. | Exact replay produces the identical Resolved Assurance Requirements identity, references, accounting, and explanation. The comparison case has a distinct provenance-bound requirements identity or refuses; equivalent-looking references do not collapse different authority. Any changed or missing binding likewise changes identity or refuses. | Resolved requirements are exact, transitively closed, and reproducible. |
 | **Exact resolved-requirements realization.** One Resolved Assurance Requirements identity for scope `S` selects contract-rooted expected-claim inventory `C`, exact Evidence Evaluation Policy bindings `E`, configured Metric Definitions `M`, and Gate Policy `G`. Before observation, the valid path forms one exact Prospective Evaluation Basis that binds those requirements and a separately issued congruent Evaluation Authorization Binding. After admission fixes the canonical evidence-set identity, the final context binds that evidence set, basis, and exactly `C/E/M/G`. Comparison cases alter either closure or omit, substitute, or transfer the Engine binding or basis. | The exact valid two-stage closure may evaluate reproducibly. Every comparison refuses at the applicable boundary before claim, metric, or semantic-gate truth; none falls back, reselects assurance, or becomes ordinary `not_applicable`, `not_computable`, or `indeterminate`. | Exact pre-observation provenance and later evidence-bound closure are both necessary and neither substitutes for the other. |
 
@@ -411,28 +495,38 @@ provenance, assurance/Context Resolution orthogonality, prohibition on
 capability-driven relaxation, separate exact Engine authorization conformance,
 the pre-observation Prospective Evaluation Basis and later evidence-bound
 evaluation-context ordering, fresh-context escalation, non-relaxable trust
-invariants, bounded monotonicity, causal ordering, enforcement separation,
-typed-exception boundary, and scenario outcomes above.
+invariants, bounded monotonicity, forward-only causal ordering, transition-
+specific Operational Reliance before posture-transition intent, the atomic
+eligibility/disposition rule, universal historical Binding validity plus the
+explicit `historical_validity_only` / `continuing_current` reliance modes,
+separate action authorization and mutation receipts, advisory-authority truth
+constraints without a general IAM mandate, deterministic Policy Admission
+Closure, operational-reliance separation, typed-exception boundary, and scenario
+outcomes above.
 
 Deliberately deferred to later human-approved task-level authority are concrete
 policy mappings and requirement sets; thresholds; schema shapes; Rust types or
 APIs; storage and registry mechanics; expression languages; migrations;
 feature flags; slice or packet decomposition; rollout; and implementation
-sequence. This record also does not select an assurance-policy or evaluation-
-coordination/readiness owner placement, observation or recommendation
-algorithm, acknowledgement or hysteresis model, UI, publication, or external
-adopter.
-
-A later human-approved **Evaluation Authorization, Capability, and Readiness**
-semantic decision owns evaluation-purpose formation, Engine authorization-
-issuance workflow, Prospective Evaluation Basis lifecycle and physical owner
-placement, the binding's public projection and opaque-lineage detail,
-currentness checkpoints, readiness states, producer capability versus
-operating ceiling, diagnostic intent, retry/cache/supersession/escalation
-lifecycle, and the advisory-CLI versus real host-enforcement boundary. This
-record selects no concrete API, schema, storage, transport, readiness
-algorithm, or host/Substrate mechanism, and it does not require a separate
-fifth authorization decision.
+sequence. This record also does not select concrete policy candidates or
+mappings, a work hierarchy or state model, physical component placement,
+observation or recommendation algorithms, acknowledgement or hysteresis model,
+UI, publication, or external adopter.
+
+The proposed
+[evaluation authorization, capability, and readiness decision](evaluation-authorization-capability-and-readiness-semantic-decision.md)
+owns evaluation-purpose formation, Engine authorization issuance, the Contracts-
+domain Evaluation Coordination semantic responsibility, Prospective Evaluation
+Basis lifecycle, currentness checkpoints, readiness, capability/ceiling
+separation, diagnostics, and retry/cache/supersession/escalation semantics. The
+proposed [operational-reliance and work-progression decision](operational-reliance-and-work-progression-semantic-decision.md)
+owns the common present-use rule, action chain, and Work Progression semantic
+responsibility for local-closeout and parent eligibility. Concrete physical
+placement, work hierarchy/state model, policy contents, transition mechanisms,
+APIs, schemas, storage, transport, host/Substrate integration, implementation,
+and delivery slices remain deferred. These proposals do not silently approve
+any such choice or require a separate authorization decision or a general IAM
+platform.
 
 Human approval of this proposed record would make it semantic input to later
 planning only. It would not authorize implementation, change any phase or slice
diff --git a/docs/specs/handbook-contract-membrane/seam-crosswalk.md b/docs/specs/handbook-contract-membrane/seam-crosswalk.md
index 12a208f..b671ac2 100644
--- a/docs/specs/handbook-contract-membrane/seam-crosswalk.md
+++ b/docs/specs/handbook-contract-membrane/seam-crosswalk.md
@@ -74,7 +74,7 @@ Read each row as a routing constraint, never as a work queue.
 | Operation and admission | Semantic owner crates plus HCM-4.2 operation/envelope primitives | Implemented exact 62-row inventory and independent owner-admission boundary; the four snapshot rows close with exact Engine evidence and generic `[cli_json, rust_sdk, tauri]` transport proof, while the other four reserved projection/snapshot rows remain absent and seven fixed use cases remain outside discovery | Future transports consume admitted definitions without adding command-derived operations or converting use cases into ordinary capabilities | HCM-4.6 Packet 5 for snapshot closure | Adapters invent operations, bypass owner admission, or disagree about terminal outcomes |
 | Bootstrap and catalog witness | HCM-4.2 production witness over checked-in schemas and owner evidence | Implemented immutable descriptor plus capability/schema/profile roots, typed pages/cursors, and exactly 35 capabilities with current SDK/CLI/Tauri transport vectors; the SDK consumer, generic CLI actions, installed skill, and Tauri facade consume this witness without another inventory or snapshot-specific route | Later transport evidence changes current transports only after complete definition/schema/owner/transport closure is current | HCM-4.6 Packet 5 for current snapshot witness proof | Discovery claims unsupported operations or treats structural closure as owner trust |
 | Contract-owner placement and dock execution | HCM-5.1 Packets 1–6 establish and integrate `handbook-contracts` definition/claim/applicability/lifecycle ownership inward of SDK; HCM-5.2 Packets 1–7 add evidence/verdict/gate/local-closeout ownership; HCM-5.3 Packets 1–5 add `handbook-dock`, the operational protocol/store/executor, three outward SDK operations, and integrated generic SDK/CLI/Tauri proof; HCM-5.4 Packets 1–6 install the first production validator and prove its real path | Contracts has no SDK/transport dependency and remains canonical semantic owner. Dock depends inward, owns the one exact production manifest, package admission, operational records, and untrusted completed candidates only. Those Phase-5 owners remain unchanged within current overall truth of 62 inventory rows, 35 capabilities, 180 schema roots, one production manifest, and seven fixed use cases. | Preserve the implemented owner direction, one-manifest identity, fixed host-relative package resolution, generic adapters, and independent Contracts candidate resolution | HCM-5.4 Packets 1–6, aggregate closeout, and Phase-5 exit complete | Circular dependencies make presentation concerns authoritative, callers manufacture lifecycle authority, durability reports false receipts, or an executor becomes the semantic owner |
-| Evidence, verdict, gate, local closeout, dependency composition, and posture boundary | HCM-5.2 Packets 1–7 implement owner identities/run preparation, sealed exact candidate-source normalization, retained-root immutable-all canonical-evidence list/read/idempotent append with atomic recovery, pure closed claim-verdict evaluation, gate/local-closeout composition, the exact five-operation outward SDK membrane, and integrated proof through unchanged generic consumers; verdict persistence and dock execution remain absent; Engine retains only generic authority/currentness/Resolution facts and posture authority | Candidate bytes and immutable provenance are read from already committed bounded observations, authenticated under the exact `contract_evidence_source` class, independently rechecked by `handbook-contracts`, durably retained only after final source, lifecycle, and evidence-authority re-admission, and accounted into deterministic verdict, gate, and sibling local-closeout results without persisting them; no parent-promotion eligibility result or consumer exists, and no posture or promotion authority follows | Preserve `handbook-contracts` as canonical evidence/verdict/gate/local-closeout owner; future executors provide untrusted candidates only. A separately authorized dependency/workflow owner may consume exact local-closeout results plus its own parent binding, policy, and currentness facts to produce a fail-closed, non-mutating parent-readiness result; any parent mutation and future posture policy remain separately owned | HCM-5.2 aggregate closeout complete; future dependency/workflow, posture, and promotion owners remain unselected and separate | A candidate producer becomes semantic authority, local closeout implies parent promotion, eligibility mutates parent state, or the gate silently becomes dependency/posture policy or mutation authority |
+| Evidence, verdict, gate, local closeout, dependency composition, and posture boundary | HCM-5.2 Packets 1–7 implement owner identities/run preparation, sealed exact candidate-source normalization, retained-root immutable-all canonical-evidence list/read/idempotent append with atomic recovery, pure closed claim-verdict evaluation, gate/local-closeout composition, the exact five-operation outward SDK membrane, and integrated proof through unchanged generic consumers; verdict persistence and dock execution remain absent; Engine retains only generic authority/currentness/Resolution facts and posture authority | Candidate bytes and immutable provenance are read from already committed bounded observations, authenticated under the exact `contract_evidence_source` class, independently rechecked by `handbook-contracts`, durably retained only after final source, lifecycle, and evidence-authority re-admission, and accounted into deterministic verdict, gate, and sibling local-closeout results without persisting them; no parent-eligibility result or consumer exists, and no posture or promotion authority follows | Preserve `handbook-contracts` as canonical evidence/verdict/gate/local-closeout owner and the inward dependency direction; future executors provide untrusted candidates only. The **PROPOSED** [operational-reliance/work-progression decision](operational-reliance-and-work-progression-semantic-decision.md) assigns greenfield semantic responsibility for local/parent eligibility to Work Progression through separate atomic Operational Reliance Determinations, and the **PROPOSED** [posture/assurance decision](posture-assurance-policy-semantic-decision.md) requires transition-specific reliance before separate posture action authorization/mutation. Physical placement, work hierarchy/state model, concrete policies, consumers, schemas/APIs, mutation mechanisms, implementation, and slices remain unselected. | HCM-5.2 aggregate closeout complete; proposed Work Progression semantic responsibility only; future human authority must select physical owner/slice and concrete realization | A candidate producer becomes semantic authority, local closeout implies parent eligibility/promotion, eligibility/disposition split into divergent results, eligibility mutates state, or the gate silently becomes dependency/posture policy or mutation authority |
 | CLI transport | SDK owns transport-free composition; CLI owns parsing, repository discovery, rendering, streams, and exit mapping | Implemented exact 36-leaf coverage plus exactly four generic capability actions over one typed owner/SDK source per route; human, new-JSON, established legacy, and generic membrane renderers preserve the same typed result and exit truth | Preserve the adapter-only boundary as commands evolve under separate authority | HCM-4.5 for the generic membrane; future command-specific authority otherwise | CLI wording or local branches become a second domain implementation |
 | CLI JSON | SDK owns the invocation/use-case/transport types and schemas; CLI owns serialization selection and physical emission | Implemented global `--json`: one schema-valid `handbook.invocation-response@1.0.0` document plus LF on writable stdout, semantic diagnostics in the result, operational stderr separation, and bounded explicit legacy migration | Tauri consumes the same typed outcomes while keeping CLI LF framing distinct from IPC framing and discovery truth | HCM-4.4 for Tauri | Scripts parse prose, receive mixed/multiple stdout values, or observe status/exit disagreement |
 | Installed skill consumer | SDK owns the consumer contracts and owner dispatch; CLI owns the four generic machine actions | Implemented generated root/leaf projections and source/release/public installation; the root skill pages live discovery, retrieves exact schemas, invokes once, and preserves typed owner and replay truth through CLI only | Later admitted ordinary capabilities require complete product-owned definition/schema/owner/transport closure, then become available without skill prose or another generic action | HCM-4.5 | Prompt text becomes a product inventory, bypasses mutation authority, retries uncertain writes, or reclassifies typed terminals |
```
~~~~~~~~~~~~
