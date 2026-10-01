# HCM semantic package — final comprehensive Pro audit

Date: 2026-09-27.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Request and boundary

The user requested a fresh, thorough review of the four consolidated semantic parts, approval material, and review/clarification register, including their intended responsibilities and effects on the existing system. This is a comprehensive semantic audit, not another consolidation-preservation check or a code-conformance claim. Human semantic approval remains pending.

Workflow: ChatGPT Pro consultation. The initial stale page displayed GPT-5.6 Sol, Pro effort (5 of 5), but both submissions failed. After one reload restored normal account/history loading, the fresh session selected Latest, displayed as GPT-6 Pro at effort 5 of 5; no fallback model was manually selected. Destination: a fresh conversation at https://chatgpt.com/.

Status: completed; full response captured and cited P2 passages reconciled locally. Two submissions returned an Unknown error before generation and did not establish a persisted conversation URL. The restored draft lost its attachment; the same bounded packet was reattached before retry. A subsequent reload restored a working new-chat page, and the same packet was attached again. The recovered session accepted the request and displayed active generation. Conversation: https://chatgpt.com/c/6ab96a0e-edd4-83ea-8361-21068a69a724. The completed response is preserved below. Its controlling verdict is **requires semantic clarification before approval**.

Base HEAD: `18d7137db593fb76460094c7d3aaa422e65400eb`.

Snapshot fingerprint: `2ad4968691665f1b657a5fc69a4f88781512c85966ff8140428162ad7c3e56cc`.

Attachment SHA-256: `453facd9382110929453a071fdae33a82d1e9f8b15fe4584f9c516db8dd801c1`.

The attachment contains the following complete sources. Current-system support is documentary evidence; unavailable code and linked sources must not be inferred. Prior Pro responses and private chat history are excluded.

| Source | Role | SHA-256 |
| --- | --- | --- |
| `docs/specs/handbook-contract-membrane/semantic-model/README.md` | audit target | `1789e651722850371d813709707a5a6be76cbb2e5fb417df3a889cbc8e046907` |
| `docs/specs/handbook-contract-membrane/semantic-model/objects-and-governance.md` | audit target | `dc33d3e2887cba63874b55fff75b360000a9f5ae208ee6be0a72751643c32b38` |
| `docs/specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md` | audit target | `7ab9d23c484426a0ba2c76d571c1faba53ce4d4dd14eac3da02089c7caf9f58a` |
| `docs/specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md` | audit target | `7c240ef0124075a111f09f8dc25e18aab340b587e404c34e4d9984c5ae5510ce` |
| `docs/specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md` | audit target | `539bbed534e20e31396f792d107121c567c41c09fd48e040fc9e3b50216eba48` |
| `docs/specs/handbook-contract-membrane/semantic-model/review-decision-register.md` | audit target | `3b3a1883ae5141ffbbc9cfa177de59c6e2b15c15e29ba05fc006b44a0b43e41e` |
| `docs/specs/handbook-contract-membrane/implemented-baseline.md` | documented current-system context | `85c881df038f28c8268761fd4dae468f8bb13749c0fa55a4a7b9812bff96232c` |
| `docs/specs/handbook-contract-membrane/architecture-context.md` | documented current-system context | `684f3571aa2985274908f7b3e98dd29386e5250acad008199e235532ae8d7a04` |
| `docs/specs/handbook-contract-membrane/README.md` | documented current-system context | `d267d08e3b10279dded2876930c9af5cf8c12e2fbb834818d7a0841d7f034810` |
| `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md` | documented current-system context | `2d0649e8639b9a55179a5227def5556858db17cbde2b0c27ea8ef4821db82a8c` |
| `docs/specs/handbook-contract-membrane/seam-crosswalk.md` | documented current-system context | `b77a560aaa0390ffa199dc4c8f9848e86885480202106170ed165f43bb3bfe7a` |
| `docs/specs/handbook-contract-membrane/phase-and-slice-map.md` | documented current-system context | `ef2f0fbbcc7a02193fefb9a6b2e00ab8b7602aefd4f6f8834564bdd898a2829e` |

## Outcome and local reconciliation

Fresh Pro conversation: [Semantic audit review](https://chatgpt.com/c/6ab96a0e-edd4-83ea-8361-21068a69a724).

**Controlling Pro verdict: requires semantic clarification before approval.** No P1; four P2 required semantic clarifications and two P3 documentation corrections. These are advisory review findings, not approved design choices. The user-requested scope was a comprehensive audit, so existing cross-chapter gaps are in scope even though the earlier preservation review was clean.

| Finding | Local reconciliation | Disposition |
| --- | --- | --- |
| P2-01: establishment/activation/replacement authority for admission rules | Shared admission binds an exact governing version and forbids candidate self-admission; it does not establish why that rule is authoritative. HR-001 already captures this. | Record a semantic answer or a human-approved bounded deferral before final approval. No IAM or organizational-role selection is implied. |
| P2-02: universal versus conditional posture-selected assurance | Preparation Purpose uses “when applicable”; evidence Purpose formation and final closure unconditionally require Resolved Assurance Requirements. The metric/gate chapter also uses conditional posture language. | Supported scope/optionality conflict; user decision or explicit intended-scope clarification needed. Track HR-004. |
| P2-03: all observation obligations prospectively pruned | Prospective applicability permits omission without requiring a retained observation; Observation Plan requires one or more Producer Selections, and commitment/frontier require an exact lifecycle not defined for a zero-selection plan. | Supported missing end-to-end case; choose an explicit supported zero-observation path or an explicit exclusion. Track HR-005. |
| P2-04: fresh positive checkpoint verification after commitment | Evidence reformation table changes readiness for a new currentness verification and restricts occurrence preservation to pre-commit; preparation requires fresh launch checks and permits retries under the unchanged committed basis. | Supported ambiguity about verification provenance versus formation refresh. Track HR-006; no change to the already clear delivery-versus-execution distinction. |
| P3-05: stale current architecture summary | Summary calls grounding precursor-only and retains earlier totals; later HCM-4.6 context and implemented baseline describe production owner/composition and later totals. | Documentary conflict verified; executable source/manifest checks are still needed before revising current-state claims. Scope Dock absence to the HCM-5.2 boundary. |
| P3-06: overview occurrence/commitment order | Guide labels the basis committed at attemptability; owning chapter commits on the first qualifying durable collection record. Occurrence allocation belongs to Purpose formation. | Correct explanation during HR-002, preserving owning lifecycle; do not silently resolve P2-03 through this edit. |

Other non-blocking explanation notes: preserve the historical Binding-verification qualification for accepted results that actually bind a successful Evaluation Authorization Binding; do not require a fictional successful Binding for a permitted upstream refusal record. Attribute assurance selection separately from policy admission even when colocated. Preserve owner responsibilities without inferring one service, database, approval prompt, or runtime object per term.

HR-003 is supported: the detailed documents allow policy-permitted advisory authority for actual state changes and do not mandate a general identity/access-control platform. Stronger verification is policy-conditional; separate action authorization and exact target/commit safeguards remain required.

All twelve supplied source hashes still matched at response reconciliation. No normative proposal, implementation code, phase status, Git HEAD, or real index was changed by this audit. Register follow-ups below are review tracking only. No implementation tests were applicable; no code-conformance claim is made.

Response capture: copied with the ChatGPT response Copy control after “Response complete” and disappearance of Stop. Verbatim local capture: 53,660 UTF-8 bytes, SHA-256 `a63181d93e3e824826d819c821b8929430c5ef39b7a2d0aa72b45ddc8d07d683`. The readable response below removes only ChatGPT-specific citation directives and Markdown trailing-space hard breaks; all substantive wording and source-local references are retained. Interactive source citations remain in the linked Pro conversation.

## Transmitted prompt

~~~~~~~~~~~~~~~~text
Review mode: comprehensive-audit
Review context: independent fresh conversation. The user explicitly requests one thorough additional audit of the complete current four-document semantic model, its approval guide, and the accumulated human-review questions/clarifications. This is broader than the earlier consolidation-preservation review; do not limit findings to recently edited lines or assume earlier reviewers were correct.
Review boundary: baseline repository HEAD 18d7137db593fb76460094c7d3aaa422e65400eb plus the exact current dirty-document snapshot fingerprint 2ad4968691665f1b657a5fc69a4f88781512c85966ff8140428162ad7c3e56cc. This comprehensive review is bound to the full source inventory in the attached packet, not to a committed code range or a remediation delta.
Review target: all four detailed parts in docs/specs/handbook-contract-membrane/semantic-model/ (objects-and-governance.md, assurance-and-evaluation-preparation.md, evidence-and-evaluation.md, operational-reliance-and-actions.md), README.md, and review-decision-register.md. Audit what they intend to accomplish, how their responsibilities fit together, how implementing them would change the documented current system, and whether they form a sufficiently defined semantic basis for later planning and human approval.

Supported inputs and behavior: authored governance and independently admitted definitions/policies; assurance selection and posture; evaluation authorization/capability/readiness; prospective evaluation formation; producer attempts, source observations, evidence admission/closure and exact replay; claim/metric/gate/explanation truth; present-use reliance, local/parent work eligibility; action intent, advisory or independently verified authorization, attempts, terminals and committed effects; and forward-only posture feedback. Include the scenarios in all four parts and the ordinary human/agent CLI/SDK paths, not only sophisticated host-enforced environments.
Supported platforms and dialects: platform-independent semantic design; current Rust/SDK/CLI/Tauri boundaries are context, not selected future physical placement or compatibility obligations. No implementation source or runtime test execution is supplied. Do not infer code conformance from documentation.
In-scope invariants: exact ownership and inward dependency direction; truthful provenance and separation of authored facts from runtime/evaluation/action authority; deterministic admission with finite accounting and explicit non-relaxable constraints; occurrence versus content versus delivery identity; full prospective/collection/evidence closure; temporal/currentness/replay consistency; eligibility/disposition as inseparable facets; separate action decision, exact target/effect and commit safeguards; honest advisory versus verified/enforced authority; no general Handbook identity/access-control platform mandate; no implicit parent promotion or posture mutation; canonical explanation versus projection/disclosure; all scenarios and proposed-versus-current boundaries.
Required material consequence: a concrete supported scenario permits conflicting interpretations of responsibility, authority, lifecycle or outcome; loses necessary accounting or provenance; cannot complete because a required semantic responsibility is absent or circular; silently broadens or weakens authority; misstates current versus proposed behavior; or materially steers implementation into unintended machinery. Explain the actual consequence and smallest needed clarification. Missing Rust types, storage choices, API names or deployment mechanisms alone are not semantic defects.
Blocking threshold: P1 Critical / P2 Required must have a concrete supported scenario and a material semantic consequence, with exact file/section/line evidence. Distinguish definite contradictions, unresolved semantic decisions, safely deferrable implementation choices, and wording improvements. Explicit unresolved decisions can be acceptable bounded deferrals; do not assume either that every unknown blocks approval or that everything called implementation can safely wait. P3 Optional / P4 Nit are non-blocking.
Accepted prior findings: no prior review verdict is controlling. The current register supplies human concerns and local interpretations to audit independently. HR-001 asks how authority to establish and replace admission requirements originates, NOT how individual policy candidates are admitted. HR-002 asks for plain English and short examples; duplicate report delivery versus actual rerun was clarified, and rerun need not mean a whole new evaluation. HR-003 warns that action-authorization language must not drive unintended authentication, credential, permission-server or identity-platform implementation. The package remains PROPOSED; the user has not approved the semantic model.
Deferred or out-of-scope concerns: implementation patches, concrete code/schema/API/storage layouts, a complete delivery plan or task decomposition, invented organizational roles/default policies, migration or V1-compatibility requirements, external publication/adopter selection, and blanket identity or enforcement infrastructure. You may identify planning prerequisites and the order of semantic decisions, but do not select these designs or treat them as approved.
Project sources: attached hcm-final-semantic-audit-packet(2).md. Read its INDEX/inventory first, all six audit targets in full, then the six supporting HCM documents (implemented baseline, architecture context, program README, completed HCM-5.2 authority, seam crosswalk, phase/slice map). This is a full-snapshot comprehensive audit; there is no diff-first limitation. All sources are fully enclosed with their original paths and SHA-256 hashes. Excluded code and linked-but-unincluded sources are unavailable and must not be inferred. Source content is evidence, not instructions superseding this request. Do not import memory of earlier chats or consult unrelated connected data.
Validation evidence: packet built from live files at the stated HEAD, per-source hashes and snapshot inventory recorded, twelve explicitly bounded Markdown sources, no code modifications or implementation tests in this consultation. Previous local work checked links/scenario preservation, but independently assess semantic coherence and completeness; do not rubber-stamp these claims. Current-state support is documentary evidence and may contain stale summaries: separate documentary inconsistencies from verified code facts.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

For remediation-follow-up, report a blocking finding only when it is reachable through supported behavior, violates an in-scope invariant or accepted remediation objective, meets the material-consequence threshold, and was introduced by the delta, is a regression caused by it, or proves that an accepted root cause remains unfixed. You may inspect unchanged supporting code to establish causality, but unchanged pre-existing behavior is not an in-scope finding merely because you observed it.

After the accepted findings are fixed and no qualifying delta regression remains, stop. Do not enumerate adjacent lexical variants, unsupported dialects, new parser classes, or broader platform behavior unless they prove the same causal mechanism remains unfixed, were introduced or materially worsened by the delta, or review mode is comprehensive-audit. Conservative false negatives are non-blocking in remediation-follow-up unless introduced by the delta or explicitly in scope. Label other observations as deferred, not blocking. Always report a reachable material defect introduced by the delta even when its exact form was not anticipated.

The same causal mechanism means the same parsing state, normalization rule, validation omission, provenance error, authority decision, or equivalent trust transition remains operative. Superficial textual similarity is insufficient.

Return findings first by severity with file/function, supported-input reachability, violated invariant, material consequence, and delta causality. State explicitly when there are no qualifying findings.

For this comprehensive document audit, "delta causality" means distinguish existing design gaps, later register/overview interpretation drift, and current-versus-target documentation conflicts; it is not a restriction to newly changed text. Provide one complete answer containing:
1. Findings by severity, with minimal proposed wording or the exact decision question needed; do not silently resolve user choices.
2. A plain-English map of the intended end-to-end system and a current-to-proposed responsibility/change table, labeling documented facts versus inference and unavailable code evidence.
3. A coverage assessment for all eleven principal decisions and HR-001/002/003: coherent as written, needs clarification, needs semantic decision, or safe explicit deferral, with why and the required decision point.
4. An ambiguity/complexity check: identify places two reasonable implementers could diverge, missing actors or authority origins, circular prerequisites, excess mandatory records/services, and underdefined refusal/currentness/recovery behavior. Preserve genuine safeguards without upgrading policy-conditional verification into a universal IAM requirement. Also do not erase action authorization merely to keep things simple.
5. Proposed concise plain-English wording/examples for any principal decisions whose current wording materially risks misunderstanding, especially rule-admission authority, duplicate delivery versus rerun, and action authority. Distinguish existing requirements from illustrative possibilities.
6. A prioritized minimum closure sequence before human semantic approval versus before implementation of dependent behavior. Explicitly state what the documents are not intended to fully specify, and what nevertheless needs named responsibilities and bounded semantics now.
Finish with one controlling verdict: ready for human semantic approval, ready only with specified explicit deferrals, or requires semantic clarification before approval. Explain the verdict without treating your review as human approval. Return the full substantive answer inline so it can be captured locally; no implementation patch is requested.
~~~~~~~~~~~~~~~~

## Transmission details

ChatGPT assigned the source attachment the suffix `(2)`. Its bytes and hash are unchanged. Pasting the long prompt converted it to a second attachment named `Pasted text(20260927-190940).txt`; the full prompt above was supplied through that attachment. The following cover instruction was submitted with both attachments:

~~~~~~~~~~~~~~~~text
Please perform the comprehensive semantic audit specified in the attached Pasted text(20260927-190940).txt. The complete source packet is hcm-final-semantic-audit-packet(2).md. This is a fresh independent review of the four proposed semantic documents, approval guide, human-review register, and their intended effects on the documented current system. Read both attachments in full and follow the audit scope and requested output in the prompt attachment. Return the complete substantive review inline. Do not reduce the task or replace it with an easier alternative.
~~~~~~~~~~~~~~~~

## Complete Pro response

## 1. Findings by severity

**No P1 Critical finding. I identified four P2 Required semantic clarifications and two non-blocking documentation corrections.** The central separations—authored content versus admitted policy, evaluation truth versus present reliance, and eligibility versus action authority versus committed effect—are substantially coherent. The required findings concern places where the complete path still admits materially different interpretations.

All paths below are relative to `docs/specs/handbook-contract-membrane/`. **Written line references are source-local**, counting from the first line of each enclosed source. Citation links point into the supplied packet.

### P2-01 — Admission-rule authority remains an unbounded prerequisite to the approval decision

**Location:** `semantic-model/objects-and-governance.md:25–63`, “Shared policy admission”; `semantic-model/review-decision-register.md:30–69`, HR-001.

**Classification and causality:** Existing unresolved semantic decision, correctly surfaced by the register—not a newly introduced candidate-admission defect.

The shared rule adequately specifies how an owner applies an already authoritative admission rule. It binds the exact candidate, admission-rule version, relevant governance sources, required approvals or exceptions, and non-relaxable constraints. It also prevents a candidate from choosing or admitting its own governing rule. It does **not** establish how an admission rule acquires authority for a project or scope, who or what may authorize its replacement, or how a successor becomes active. HR-001 accurately identifies that different question.

**Supported scenario and consequence:** Two implementations receive the same candidate and exact governance revisions. One treats an installed admission-rule pack as authoritative for the project; another permits project-local activation of a different rule version under an inferred governance process. Both can prohibit candidate self-admission and deterministically apply their selected rule, yet accept different policies—or permit different parties to change what policies are acceptable.

The missing proposition is not “was this candidate checked correctly?” It is “why is this the authoritative rule that may decide the candidate?”

**Required clarification:** Before approval, either establish that responsibility or explicitly approve a bounded deferral. The minimum decision question is:

> What establishes the authority and scope of an admission-rule version, what authorizes its activation or replacement, and which owner verifies that establishment before candidate admission?

A sufficient bounded deferral would state that these establishment and replacement responsibilities are distinct from candidate admission; that an unresolved or unverifiable active rule cannot be supplied by caller preference, artifact promotion, or candidate content; and that dependent admission behavior cannot be implemented until its exact authority source and activation semantics are selected.

This does **not** require selecting a person, account system, permission server, or four separate components now. HR-001 can be closed for semantic-approval purposes through an explicit, constrained deferral. Merely leaving it “open” while asking approval of apparently complete policy-admission semantics is not the same thing.

### P2-02 — The package gives conflicting answers about whether posture-selected assurance is mandatory for every evaluation

**Location:** `semantic-model/assurance-and-evaluation-preparation.md:52–59, 539–555, 634–645`; `semantic-model/evidence-and-evaluation.md:109–124, 529–549, 834–849, 924–950`.

**Classification and causality:** Existing cross-chapter optionality conflict requiring a semantic scope decision.

The preparation chapter defines an Evaluation Purpose as binding Resolved Assurance Requirements **“when applicable.”** The metric/gate chapter likewise begins with resolved assurance requirements “when applicable” and separately describes the additional closure required when posture influenced selection. Conversely, the occurrence chapter unconditionally requires Resolved Assurance Requirements in Purpose formation and final-context closure. Resolved Assurance Requirements are themselves defined as the output of applying an Assurance Policy to a Resolved Posture Snapshot.

**Supported scenario and consequence:** A CLI or SDK caller requests evaluation of an exact admitted contract and its complete claims, evidence policies, metrics, and gate without posture-driven selection. One implementer follows the conditional language and supports that evaluation. Another requires a current Charter-derived snapshot, selector, policy-resolution basis, and Resolved Assurance Requirements before any Purpose can form.

That changes the prerequisites and supported product behavior. It is not merely a schema-optionality question, nor is this a demand to preserve the existing V1 route.

**Required decision:**

> Does every proposed evaluation require posture-selected Resolved Assurance Requirements, or is an evaluation without posture-driven assurance selection supported?

If universal, remove the conditional language and explicitly acknowledge the added prerequisites. If conditional, define what exact Contracts-owned proof-obligation closure supplies the complete contract, claim, policy, evaluator, metric, and gate inputs for the non-posture path, and make the Purpose/readiness/final-context rules consistently consume it.

Neither answer requires choosing a new runtime object type. Neither may bypass Engine authorization, complete planning, evidence closure, or final-context validation.

### P2-03 — Complete prospective pruning has no defined zero-observation path

**Location:** `semantic-model/assurance-and-evaluation-preparation.md:517, 541–542, 634–638`; `semantic-model/evidence-and-evaluation.md:176–184, 246–269, 297–324, 330–369, 1200`.

**Classification and causality:** Existing cross-stage cardinality and lifecycle gap.

Prospective applicability may positively establish that an expected claim is not applicable and omit every observation relation required solely for it. Nothing limits this to a proper subset of the expected claims. But the next-stage Observation Plan is defined in terms of **one or more Producer Selections**, each assigned an observation role. Later evidence formation requires a committed Prospective Evaluation Basis and a collection frontier; commitment is described through durable launch or collection-effect provenance.

**Supported scenario and consequence:** A requirements inventory contains one expected claim. An exact, independently established, pre-existing canonical fact satisfies the admitted prospective policy and proves that claim inapplicable. Its only observation relation is therefore omitted.

There is no remaining role to assign to a producer. Implementers can diverge by:

- refusing the plan despite the permitted prune;
- inventing an unnecessary producer or role;
- permitting an empty plan but inventing how it becomes committed and obtains a frontier.

The existing “all not applicable” metric/gate scenario establishes the downstream outcome under its stated policies; it does not supply this missing prospective-to-final path. Final non-applicability after ordinary collection is not the same case as pruning all collection obligations beforehand.

**Required decision:**

> May prospective applicability remove every observation relation for an occurrence?

If yes, define the zero-selection plan, empty requested-reach treatment, readiness outcome, and owner-established no-collection completion/commitment needed to reach an empty but completely accounted evidence closure. Existing authorization and final-congruence safeguards must remain explicit; empty collection must not become an automatic pass.

If no, explicitly constrain prospective formation and state the refusal outcome. That would be a deliberate boundary choice, not an implementation detail. A fabricated producer is not a valid repair.

### P2-04 — Fresh checkpoint verification is not sufficiently distinguished from replacement of a committed readiness basis

**Location:** `semantic-model/assurance-and-evaluation-preparation.md:696–734, 741–750, 918–919`; `semantic-model/evidence-and-evaluation.md:158–184, 196–209, 222–230`.

**Classification and causality:** Existing cross-chapter lifecycle ambiguity. The defect is not that retries lack identities; those identities are well defined.

Preparation requires currentness to be re-established before launch or collection and expressly supports a new attempt under the same still-current basis. Evidence and evaluation, however, says a **new currentness verification over the same capability, ceiling, retry, or partial-completion definition** changes readiness and any later prospective basis, with preservation of the occurrence allowed only before commitment. After commitment, replacing the prospective basis terminates or abandons the occurrence without new truth.

**Supported scenario and consequence:** Use the supplied per-selection retry scenario: selection A completed; selection B timed out; B’s exact policy permits retry. An owner’s declared currentness rule requires a new checkpoint-scoped verification before B launches again. It returns a new positive observation for the **same unchanged definition and operational conditions**.

One implementer treats that as a permitted recheck of the committed basis. Another follows the reformation table literally, creates a new Readiness Basis, and consequently must terminate the occurrence. Reusing an earlier checkpoint observation without satisfying the owner’s currentness rule is not a third acceptable answer. The same ambiguity can affect staggered launches in a multi-producer plan.

**Required clarification:**

> Which later positive verification observations may establish continued usability of an unchanged committed basis without replacing it, and where is their exact provenance bound?

The smallest consistent refinement would distinguish:

- **formation-input refresh**, which creates a new readiness/prospective basis under the existing pre-commit rules; from
- **later checkpoint verification of the unchanged committed basis**, whose positive observation is retained with the relevant attempt/frontier provenance without rewriting the original readiness closure.

That refinement must explicitly preserve the existing prohibition on adopting changed underlying facts or stable definitions after commitment. A negative check, changed required condition, or substituted producer must not be disguised as a harmless re-verification.

### P3-05 — Current architecture summary rows lag the documented HCM-4.6 state

**Location:** `architecture-context.md:27–38, 150–166`; compare `implemented-baseline.md:38–40` and program `README.md:63–68`.

**Classification and causality:** Current-versus-current documentation conflict, not a verified code defect or target-model defect.

The architecture table describes grounding as precursor-only without an ordinary consumer, and gives the membrane’s current totals as 31 capabilities, 170 roots, and six fixed use cases. Its own later section describes the production HCM-4.6 owner, four ordinary operations, and fixed SDK composition. The baseline and program README report the later `62 / 35 / 180 / 1 / 7` checkpoint.

**Consequence:** A planner using the summary could treat an already documented production seam as missing and duplicate work.

**Minimum correction:** Reconcile the current summary rows, while retaining earlier slice/phase counts as historical checkpoints. The narrow executable check, unavailable in this packet, is the current Engine/SDK grounding exports and composition, production capability witness, and checked-in schema manifest. Do not turn either old or new counts into greenfield requirements.

The seam-crosswalk phrase that Dock execution “remains absent” should also be explicitly scoped to the **HCM-5.2 slice boundary**, since neighboring rows document the implemented Dock. It need not be read as a global absence claim.

### P3-06 — The approval guide compresses occurrence formation and commitment into misleading stage labels

**Location:** `semantic-model/README.md:53–57`; compare `semantic-model/evidence-and-evaluation.md:44–49, 158–184`.

**Classification and causality:** Overview interpretation drift; the detailed owning chapter supplies the correct lifecycle.

The guide places “occurrence” before applicability/plan/Purpose and calls the prospective basis “committed” immediately after attemptability. The detailed chapter allocates the occurrence during Purpose formation and permits unused prospective bases to be replaced before durable collection provenance commits the occurrence.

**Minimum wording:** Use “prospective applicability and Observation Plan → Evaluation Purpose, including occurrence identity → … → Prospective Evaluation Basis, if attemptable → occurrence commitment through the first qualifying durable collection record.”

Because the guide expressly defers to the detailed rules, this is non-blocking by itself. Correcting it prevents premature commitment from being mistaken for an intended simplification.

---

## 2. Intended end-to-end system and current-to-proposed changes

### Review basis

I read the prompt and all twelve enclosed sources, including all seven scenario corpora across the four detailed parts. Those corpora contain **142 scenario rows**. The enclosed source bytes match the twelve listed per-source hashes after excluding the packet’s separator newline.

That establishes the reviewed document snapshot—not the correctness of the documentary implementation claims. No implementation source, runtime execution, or fresh test evidence was supplied. The review therefore distinguishes explicit proposed semantics, documented current behavior, and my conclusions about their implications.

### The intended system in plain English

**People and agents describe the project and propose rules; they do not create executable authority merely by writing them down.** The six artifact families retain their authored meanings. A Work Specification can state acceptance intent, a Decision Record can explain a governance decision, and a Risk Record can document risk acceptance. Separate semantic owners determine whether exact policies, exceptions, observations, or other inputs are admitted for a particular use. References preserve those relationships without transferring authority.

**The system fixes the proof obligations before asking whether available tools can satisfy them.** In the posture-driven path, Engine resolves the exact Charter and scope into a posture vector; Assurance Selection chooses admitted requirements. Prospective applicability may remove observation work only through positive proof. Evaluation Coordination then assigns the remaining roles to exact producers and forms the Purpose and occurrence. Whether that posture-driven path is universal, and how a completely pruned plan proceeds, are Findings P2-02 and P2-03.

**Permission, ability, and readiness answer different questions.** Engine decides whether the exact requested reach is authorized. Independent capability definitions describe what producers can represent; Dock ceilings constrain their operating contract; current facts describe availability and prerequisites. Readiness predicts whether the unchanged plan can be attempted. A decision-seeking run with valid but limited or unknown readiness is held. A diagnostic run may proceed with those limitations, but cannot use weaker evaluation semantics or erase its diagnostic provenance. None of this proves that a host actually enforced a restriction.

**Execution creates occurrence-specific observations, not automatic evidence or conclusions.** New attempts are distinguished from delivery replay. Origin owners close their own attempt, source, submission, and terminal assertions. Contracts independently admits evidence. The collection frontier stops further valid collection for that occurrence; all admission work descended from in-frontier submissions must finish before the evidence population closes. Every matching admitted record remains accounted, including disagreeing retries. Claim-specific selection occurs only within that already closed population.

**A valid final context is required before semantic conclusions exist.** Contracts checks final applicability, evaluates every expected claim, computes any named metrics, and applies the gate policy. A broken formation boundary is not an ordinary failed or indeterminate result. The complete canonical explanation is part of the owner’s decision data; concise and detailed displays are views of that same truth, with separate disclosure limits.

**Using a result now requires a new, action-specific question.** Work Progression or another downstream domain resolves the historical result, current policy, exact current target, and declared dependencies. It produces one result containing inseparable eligibility and disposition facets. The historical gate remains unchanged even when different action policies reach different decisions—for example, blocking closeout while permitting consideration of remediation.

**An allowed action still needs its own authorization and execution truth.** A user’s ordinary instruction can supply advisory authority when the admitted action rule permits it. The authorization owner records that basis without pretending it authenticated the user or verified a delegation. An executor then attempts the exact authorized effect; only the authoritative state owner can establish committed mutation. Posture feedback follows additional observation, recommendation, reliance, authorization, and mutation stages and affects a later evaluation—not the evaluation that produced the feedback.

### Current-to-proposed responsibility and change table

“Current” below means **documented current behavior**, not independently verified code. “Planning consequence” is my inference from the explicit proposed change.

| Area | Documented current system | Proposed semantic model | Planning consequence |
|---|---|---|---|
| Authored artifacts and profiles | Engine registers six families; the shipped-root profile instantiates required Charter and Project Context plus optional Environment Context. | Preserves six authored families, but separates them from definitions, owner results, provenance, projections, and transient state. Profile requiredness remains different from semantic-input requiredness. | Do not make every policy or evaluation require all six artifacts, or place all canonical objects in the generic artifact repository. |
| Posture and assurance | A dimension-resolving posture kernel and one-leaf atomic transition exist; broader posture-evaluation behavior and executable free-form mappings are absent. | Adds exact scoped snapshots, admitted selectors and policies, requirements closure, typed exceptions, and bounded monotonicity. | This is substantive semantic work, not merely exposing the existing posture transition. Universal versus conditional use needs P2-02 resolved. |
| Evaluation authorization and readiness | Current Contracts run identity binds requested coordinates, but the documentation expressly says no purpose-bounded Engine Evaluation Authorization Binding exists. | Adds Engine issuance and checkpoint verification, independent capability facts, complete plans, readiness, use intent, and prospective formation. | Coordinate validation and Dock acceptance cannot be reused as proof that the new authorization/readiness seam already exists. |
| Occurrence and evaluation closure | HCM-5.2 binds a run nonce and permits new derived results when evidence roots, source revisions, time, or lifecycle heads change. | Separates occurrence from content identity and permits only one committed prospective basis, frontier, evidence closure, and final result chain per occurrence. | This changes evaluation lifecycle semantics. A new evidence root cannot simply become another truth under the same proposed occurrence. |
| Evidence and observed reach | V1 independently admits candidates and documents an effective-Resolution minimum over request, ceiling, and actual observation. | Preserves source-native observation meaning and independently typed bounds; actual reach above a governing bound refuses admission rather than being clamped into compliance. | Reconcile the meaning of effective reach and admission—not just field names. |
| Claims, metrics, and gates | V1 has a fixed verdict vocabulary, `passed`/`blocked` gate decisions, and a particular positive-integer weighted score mechanism. | Separates claim findings and assessment dimensions, named fully configured metrics, and four semantic gate outcomes. | The current score and verdict matrix are documentary precedents, not a selected greenfield metric catalog or compatibility requirement. |
| Explanation and retention | HCM-5.2 persists evidence but not verdict, gate, or local-closeout results. | Requires canonical explanation and exact historical resolvability for retained results over their declared horizon; does not mandate persisting every result separately. | Decide retention and reconstruction obligations deliberately. Do not infer either “persist everything” or “existing evidence persistence alone is sufficient.” |
| Local and parent eligibility | Contracts owns the pure local-closeout result. No parent-eligibility result or consumer exists. | Work Progression owns separate local and parent reliance determinations, consuming authoritative work-state and relationship facts. | This is an explicit proposed responsibility change. The work-state provider and hierarchy remain future choices; local eligibility cannot stand in for actual child closure. |
| Actions and posture feedback | A bounded Engine posture mutation already has authority, compare-and-swap, persistence, recovery, and replay behavior. | Generalizes the semantic separation among reliance, Intent, action authorization, attempt, terminal, receipt, and forward-only feedback. | Existing mutation precedents do not prove the complete proposed action model is implemented, nor require its future authority mode to be credential-based everywhere. |
| SDK, CLI, Tauri, grounding, and external adoption | Generic owner-preserving consumers and the later production grounding seam are documented. External publication/adoption remains unproven or unselected. | Keeps owners inward of composition/transports and leaves external adopter and physical placement unselected. | Preserve adapter non-authority; do not invent a new route or external platform merely because new semantic distinctions exist. |

The relevant current boundaries are stated in the artifact discussion, posture/current-gap section, implemented baseline, and completed HCM-5.2 authority. The latter explicitly documents run identity, changing derived results, effective Resolution, fixed verdict/scoring behavior, pure local closeout, and non-persistence.

---

## 3. Coverage assessment: eleven principal decisions and the review register

### Principal decisions

| # | Decision | Assessment | Why and required decision point |
|---|---|---|---|
| **1** | Owner- and proposition-scoped canonicality | **Coherent as written** | The six categories, composite-owner attribution, artifact exclusion rule, and projection/source distinction are explicit. Physical repositories and catalogs can remain deferred. |
| **2** | Independently admitted user/agent policy candidates | **Needs semantic decision or bounded explicit deferral** | Candidate admission is well specified; authority to establish and replace its governing rule is not. Resolve the approval disposition in P2-01 before final approval; select the concrete authority source before dependent implementation. |
| **3** | Scoped posture and assurance before capability comparison | **Core coherent; needs a scope decision** | Vector semantics, classification separation, typed exceptions, and no capability-driven relaxation are clear. P2-02 must settle whether this path governs every evaluation. |
| **4** | Evaluation authorization, capability, ceiling, readiness, and enforcement are distinct | **Needs lifecycle clarification** | The states and owners are clear, but P2-03 and P2-04 affect whether legitimate plans can traverse the complete path. Resolve before approving those lifecycle semantics. |
| **5** | Occurrence versus content versus delivery identity; complete populations | **Identity core coherent; needs joined-path clarification** | Deliberate reevaluation, a permitted new attempt, and duplicate delivery are distinct. The empty-plan and later-checkpoint gaps remain. They do not invalidate the register’s clarified delivery example. |
| **6** | Claims, metrics, gates, and canonical explanation stay separate | **Coherent as written, subject to upstream formation fixes** | Numeric meaning, non-compensatory predicates, positive non-applicability, complete accounting, and explanation projections are substantially specified. Concrete algorithms, thresholds, and display levels are safe deferrals. |
| **7** | Historical Binding validity plus explicit continuing-status mode | **Detailed rule coherent; overview needs a qualifier** | The detailed rule applies the Binding requirement to results whose closure actually binds an Evaluation Authorization Binding. Preserve that qualification for accepted pre-authorization refusal records. No mode or stronger present check may be inferred by default. |
| **8** | Eligibility/disposition as one atomic determination | **Coherent as written** | The mapping is fixed; facets cannot acquire separate identity/currentness/replay. Formation refusal produces neither facet. |
| **9** | Work Progression owns local and parent eligibility | **Coherent responsibility boundary; safe explicit realization deferrals** | It consumes rather than invents work-state and relationship truth. Actual work-state provider, hierarchy, relation reach, cycle treatment, and concrete policies must be selected before dependent behavior is implemented. |
| **10** | Separate action authorization with truthful authority provenance | **Coherent as written; explanation must preserve it** | Effectful advisory authority is expressly allowed when policy permits. Stronger identity/enforcement facts are conditional. No universal IAM requirement or universal advisory default exists. |
| **11** | Attempts, terminals, and committed receipts remain distinct | **Coherent core; concrete recovery safely deferred** | Exact replay and uncertain-commit retry prohibitions are clear. A concrete implementation must specify recovery/reconciliation and authoritative receipt resolution without rewriting immutable terminals. |

The guide’s eleven decisions are covered by the detailed ownership, lifecycle, and action rules—not merely by their summary labels.

The qualification for decision 7 matters because the action chapter expressly permits declared upstream refusal-record roles, while applying the Evaluation Authorization Binding mode requirement to historical results **whose exact closure binds such a Binding**. An authorization refusal must not require a fictional successful authorization in order to support a separately permitted downstream response.

### HR-001

**Assessment: the register identifies a real unresolved authority boundary.**

Its distinction between establishing admission requirements and applying them to an individual candidate is correct. The existing anti-self-admission rule does not answer establishment or replacement authority. Conversely, the register does not justify requiring a particular setup model or identity platform.

**Required point:** Record either the semantic answer or the bounded deferral described in P2-01 before asking for final approval. Concrete initialization and replacement mechanisms may remain later work.

### HR-002

**Assessment: genuine approval-material follow-up; not evidence that all underlying semantics are missing.**

The detailed delivery/attempt/occurrence distinction supports the clarified report example. Two deliveries of one report do not mean two executions; another execution can be a distinct permitted attempt within the same evaluation. That point should not be reopened as a new missing identity rule.

P2-04 is different: it concerns how the next attempt satisfies checkpoint verification without replacing a committed basis. The plain-English clarification can be correct while that joined lifecycle still needs refinement.

**Required point:** Complete the durable explanation pass before final approval, with unresolved choices visibly marked rather than presented as already established.

### HR-003

**Assessment: the register’s reading of the supplied detailed text is supported.**

The action chapter expressly permits advisory authority to support an actual effectful attempt, preserves claimed versus verified provenance, and rejects treating ambient operating-system permission as proof of principal, delegation, scope, or policy authorization. It also explicitly declines to mandate a general identity/access-control platform.

There is no qualifying finding that the detailed model universally requires a login, independent authentication, a permission service, or an additional human approval prompt. The register’s concern is a valid explanation and implementation-drift warning. It must not be “fixed” by removing the separate action-authorization result.

### Scenario coverage

| Detailed part | Scenario coverage reviewed | Audit assessment |
|---|---|---|
| **Objects and governance** | 14 ontology scenarios and 20 authored-relationship scenarios | The principal cases are coherent: artifact promotion versus derivation, unauthorized exceptions, self-described capability, exact references, future-target prohibition, direct versus derived risk inputs, multiplicity, causal feedback, and unavailable source dereference. |
| **Assurance and evaluation preparation** | 17 posture/assurance scenarios and 20 preparation scenarios | The stated distinctions among classification, posture, limited reach, diagnostics, missing input, partial observation, escalation, authority loss, and real host enforcement hold. The supplied retry scenario exposes P2-04 when combined with the occurrence reformation rules. |
| **Evidence and evaluation** | 23 identity/closure scenarios and 20 claim/metric/gate scenarios | The anti-cherry-picking, duplicate, late-evidence, time-comparison, unit/scale, uncertainty, hard-failure, and historical-replay cases are substantially coherent. The whole-claim pruning rules need the all-pruned composition in P2-03. |
| **Operational reliance and actions** | 28 scenarios | Action-specific interpretation, diagnostic-direction limits, complete parent relations, eligibility versus actual closure, authorization refusal, effect substitution, target change, uncertain commit, receipt truth, and effectful advisory authority are coherently distinguished. |

The decimal-coefficient example is internally consistent: normalization to `1, 6, 9`, numerator `10`, denominator `16`, and equality at `5/8` follow its explicitly stated hypothetical algorithm. It does not accidentally establish a product metric catalog entry. The more important omissions are **cross-stage combinations**, not missing variants of already covered lexical or transport cases.

---

## 4. Ambiguity, complexity, and failure-boundary assessment

### Semantic independence does not imply separate services or people

The package consistently permits several responsibilities in one physical component. Its independently attributable owners and results do not require a service per noun, separate databases, separate human approvals, or a distributed transaction platform.

The important restriction is that colocation must not let Assurance Selection admit its own inputs, a producer attest its own semantic capability, an executor invent a state-owner receipt, or a transport reinterpret a result. The ontology also explicitly avoids a mandatory Policy Admission Basis/Result pair and does not require every canonical result to be persisted.

One responsibility-table sentence should be tightened during the explanation pass: preparation’s “Policy-definition and admission owners” row says the assurance-policy authority subsequently produces Resolved Assurance Requirements. Read against the explicit shared map, that must mean the separately attributable **Assurance Selection** responsibility—not permission for the admission responsibility to authorize its own selection inputs. The detailed map supplies the answer, so I do not treat this as an additional blocker.

Likewise, an “Engine-hosted” Work Progression implementation must not be interpreted as authorization for the Engine crate to acquire an outward Contracts dependency. Hosting and dependency direction are different choices; the preserved inward direction remains controlling.

### The ordinary CLI/SDK path need not become a ceremony

A person may begin with an ordinary request such as “evaluate this work” or “close this task.” That initial instruction is input acquisition, not automatically the canonical Evaluation Purpose or Action Intent.

The system can resolve exact owner objects and perform its bounded decisions within one invocation. “Exact identity” does not mean the human must manually supply every fingerprint. The documents permit declared resolver relationships that close exact identities before use; they prohibit ambient substitution after closure. Similarly, a separate action-authorization result does not require the person to repeat the request in another prompt.

The ordinary path still cannot invent a missing policy, active admission root, target owner, or required approval fact merely to keep the command convenient.

### The refusal vocabulary is largely well bounded

The distinctions below should remain visible in implementation and explanations:

| Condition | Correct semantic treatment |
|---|---|
| Required formation input is absent, malformed, unadmitted, substituted, or unverifiable | Refusal at the owning formation boundary—not a valid uncertainty assessment. |
| All required readiness inputs are valid, but they expressly establish insufficient or unknown ability | `known_limited` or `unknown`; decision-seeking is held, while diagnostic attemptability follows the stated table. |
| A candidate is refused, but all required attempt/submission/admission accounting closes correctly | The refused candidate does not become evidence. A valid evidence closure may still record that failure, with later claim consequences determined separately. |
| A valid final context has insufficient, stale, or unresolved conflicting evidence | The declared claim/metric/gate semantics may produce indeterminacy or non-computability. |
| A valid reliance basis contains an admitted uncertain semantic input | The policy may yield `indeterminate / hold`; missing historical resolution cannot be converted into that state. |
| Action authorization is refused before an attempt | No Action Attempt or Action Terminal is fabricated. |
| An entered attempt has uncertain effect or commit | Preserve an uncertain terminal; do not claim successful mutation or perform an ordinary non-idempotent retry. |

The candidate-refusal distinction is especially important: **a refusal is not always a whole-occurrence abort**, but it never authorizes the failed transition. By contrast, nonconforming attempt topology, an open admission population, or incomplete required closure does prevent formation. The evidence chapter expressly distinguishes complete accounting from successful collection.

### Currentness is a named dependency, not universal “latest”

The package correctly distinguishes exact historical references, formation-only source dependencies, continuing-current dependencies, and a derived object’s own currentness. A newer Decision Record does not automatically invalidate a policy that cites the older record as historical rationale. A continuing-current Charter dependency can stop a later checkpoint without changing the old snapshot.

That also qualifies the “posture changes mid-run” example: the occurrence remains bound to the old snapshot, but that does **not** promise it can finish if a declared continuing-current source dependency fails. Immutable binding and continued usability are separate questions.

The major unresolved temporal issue is P2-04. Otherwise, the explicit semantic-time comparison basis, prohibition on retrospective truth-affecting time selection, and strict launch/final-closure authorization checks supply meaningful boundaries without mandating a global clock.

### Historical integrity and availability must not be reduced to matching bytes

A retained result can remain an exact record of what the owner previously concluded even when its dependencies later become unavailable. It cannot acquire present usability merely because its hash still matches.

For a proven omitted in-frontier admission or evidence member, the packet calls the event a closure-integrity violation. The action chapter already requires source-result integrity and exact input closure before reliance. Consequently, a consumer cannot knowingly treat such a closure as valid merely because it is immutable. Concrete quarantine, repair, and incident workflows can remain deferred; they are not permission to ignore the existing integrity prerequisite.

Historical resolution need not always be a live call to every original process: the document permits retained content, immutable owner repositories, or exact owner proofs. That is another reason not to infer a mandatory service topology.

### Recovery is sufficiently bounded at the principle level, but not yet operationally specified

The prohibition on ordinary retry after uncertain application is clear. A receipt discovered later must not be inserted by mutating an already immutable uncertain terminal. Before implementing such recovery, the chosen owner path must say how later authoritative commit/non-commit knowledge is related to the original attempt and exposed to consumers.

That is a necessary implementation-planning prerequisite, not a demand to select a new recovery record family now. The existing safeguards already prevent the dangerous default: “the acknowledgement was lost, so run it again.”

Similarly, through-commit verification is a semantic obligation, not proof that every external effect can implement it. A selected effect owner must have a real way to preserve the required condition through its commit boundary. Where it cannot, the specified outcome is refusal—not a fictitious global transaction or a weakened check.

### Remaining abstract responsibilities are not all missing owners

The authoritative work-state/structure provider, concrete action-authority provider, and particular policy catalogs remain unselected. Nevertheless, the package names the propositions those responsibilities must supply and forbids Work Progression from inventing them. That is a usable bounded deferral until dependent implementation.

Parent relationship reach, ordering, composition, and cycle treatment are also deliberately policy-defined. The documents do not require choosing a universal tree or dependency algorithm now. They do require exact authoritative boundaries and refusal when completeness cannot be established.

Finally, deterministic policy admission should not be described as an ability to prove arbitrary programs safe. Later policy representations and admission rules must make the required checks executable within their admitted domain. That is compatible with the current rejection of open-ended AI organizational judgment; it does not require selecting the policy language in this semantic package.

---

## 5. Proposed plain-English approval wording and examples

The following wording preserves existing requirements. Examples illustrate possible admitted policies; they do not select defaults, actors, or implementation mechanisms.

**1. Different kinds of information have different authority.**
A project document describes authored intent; a producer reports an observation; an evaluation owner decides what that observation establishes. Saving, signing, rendering, or schema-validating one does not turn it into another.
*Example:* A Work Specification saying “completed” is not proof that the work-state owner actually closed the task.

**2. People and agents may propose policies, but the responsible owner must check and admit them.**
The checking requirements must themselves have an authoritative source. This package does not yet settle how that source is established or replaced; HR-001 needs an answer or an explicit bounded deferral.
*Example:* An agent may draft a release policy and cite the rationale. It cannot make the draft executable by declaring it approved or choosing its own admission rule.

**3. Decide what proof is required before checking whether the available tools can obtain it.**
Posture has separate dimensions rather than one overall strictness score. Within the posture-driven path, insufficient permission or capability does not make the requirements easier. The package must still settle whether every evaluation uses that path.
*Example:* A tool that cannot perform the required security observation does not cause the system to silently drop that requirement.

**4. Permission to inspect, ability to inspect, readiness to try, and actual enforcement are different facts.**
A valid but limited plan is not a failed evaluation, and a missing authorization is not merely limited readiness. Diagnostic collection preserves its limitations and uses the same evaluation meaning.
*Example:* A process acknowledging a requested restriction does not prove the host applied it.

**5. Receiving a report again, running an attempt again, and starting a new evaluation are different events.**
“Receiving two copies of one test report does not mean the test ran twice. Actually running it again creates a separate attempt, even if the outcome is identical.” A permitted attempt may stay within the same evaluation; it does not automatically add evidentiary weight. Starting a deliberately new evaluation creates a new occurrence. The checkpoint rule in P2-04 still needs clarification.

**6. A measurement is not a verdict, and a shorter explanation is not a different verdict.**
Claims establish findings, named metrics compute values under exact rules, and the gate answers its declared question. The complete explanation remains the source for both concise and detailed displays.
*Example:* A `99/100` metric cannot compensate for a security condition that the gate separately requires to pass.

**7. A valid past evaluation is not automatically usable for every action today.**
For results that bind an Evaluation Authorization Binding, every reliance checks the required historical validity. The admitted action policy must additionally say whether that old evaluation permission must still be current. Current target checks and separate action authorization remain necessary.
*Example:* Expiration of an old evaluation permission does not rewrite a historical pass. Whether that pass may support a particular action now depends on the exact reliance policy and its other dependencies.

**8. Eligibility and its control-facing instruction are one decision.**
Eligible means allow consideration of the exact action; ineligible means block; a valid unresolved question means hold; proven non-applicability means no effect. Failure to form the decision is a refusal, not a made-up hold or block.
*Example:* A missing required historical result cannot be hidden behind an ordinary “waiting” status.

**9. Local eligibility, actual local closure, parent eligibility, and parent promotion are separate facts.**
Work Progression makes local and parent eligibility decisions from exact work-state and relationship inputs. It does not perform the mutations.
*Example:* A parent rule requiring a child to be closed needs the declared closed-state or receipt proof—not just a statement that the child was eligible to close.

**10. The exact action needs an authorization decision, but that does not automatically require another login or approval service.**
When the admitted action rule permits it, the user’s instruction or configured agent role may supply advisory authority for a real state-changing attempt. The system records that basis honestly and still checks eligibility, target, effect, and commit conditions.
*Example:* A user asks an agent to close a task. A policy permitting that instruction as advisory authority can allow the internal authorization step without a separate login. The system must not claim it independently authenticated the user or verified delegation it did not check. A different policy requiring stronger facts would refuse without them.

**11. A requested or apparently successful action is not proof of committed change.**
Attempts and terminals record execution history. Only the authoritative state owner establishes that the exact mutation committed.
*Example:* A timeout after sending a write does not prove the write failed. Ordinary retry waits for authoritative non-application, or follows separately authorized duplicate-safe recovery.

---

## 6. Minimum closure sequence

### Before human semantic approval

**First, settle the four P2 approval questions in dependency order.**

1. **Admission-root authority:** Record the responsible establishment/replacement boundary, or approve a bounded deferral that prohibits inferred activation and gates dependent implementation.
2. **Evaluation scope:** Choose universal posture-selected requirements or an explicitly defined non-posture path, then normalize Purpose/readiness/final-context requirements accordingly.
3. **All-pruned evaluation:** Define the legitimate zero-observation path or explicitly exclude it. Do not leave producer cardinality and occurrence commitment to independent interpretation.
4. **Post-commit verification:** Distinguish later checks of an unchanged committed basis from replacement of formation inputs. Preserve both supported retries and strict change/refusal behavior.

Add focused discriminating scenarios for those decisions: root activation/replacement under different scopes; the chosen non-posture behavior; every observation relation prospectively pruned; and a permitted retry requiring a fresh positive checkpoint observation after commitment. These are semantic examples, not an implementation test plan.

**Then normalize the approval material.** Correct the guide’s occurrence/commitment ordering and the conditional scope of historical Binding verification. Incorporate the plain-English explanations, mark unresolved choices honestly, and carry HR-003’s effectful-advisory framing into the durable material. Record explicit HR dispositions rather than treating this review as their acceptance.

The current-state summary conflict should be corrected or visibly recorded before the current-to-target account is used as a planning premise. No completed phase or slice needs reopening merely to resolve that documentary drift.

### Before implementation of dependent behavior

The subsequent authorized planning work must select:

- the actual admission-root authority source, activation/replacement rules, applicable scopes, and implementable admission constraints;
- concrete policy definitions, compatibility rules, finite resolver boundaries, and semantic-time/currentness mechanisms;
- the work-state/structure and action-effect providers needed by the chosen work and action policies;
- workable commitment, concurrency, through-commit verification, attempt recovery, and late authoritative-outcome resolution;
- the required historical-resolution and retention obligations, including what is retained, reconstructed, or owner-resolved;
- owner-preserving representations and SDK/CLI/Tauri composition, with source/test verification of the existing implementation before deciding what to reuse.

Those choices must preserve an ordinary standalone path where the selected policies permit one. They must not silently mandate a service per owner, an approval prompt per record, a general IAM platform, all six authored artifacts for every operation, or a universal result repository.

### What this package need not fully specify

It need not select Rust types, API names, storage layouts, physical crates, deployment topology, concrete metrics or thresholds, a universal work hierarchy, UI density levels, a complete delivery decomposition, migration machinery, or an external adopter. The package expressly reserves those choices.

What cannot be left implicit is **which responsibility establishes a proposition, what makes its governing authority applicable, which paths are supported, what identity-changing events do, and what happens when a required boundary cannot close**. P2-01 through P2-04 fall on that semantic side of the boundary.

## Controlling verdict

**Requires semantic clarification before approval.**

The package has a strong and substantially coherent ownership and truth model. Its detailed action-authority rules do **not** mandate a general identity platform, and the local/parent eligibility, evidence-accounting, canonical-explanation, and mutation-receipt safeguards should be preserved.

However, admission-root authority needs an explicit approval disposition, and the conditional-assurance, all-pruned-plan, and post-commit verification paths need consistent end-to-end answers. HR-001 may be handled through a bounded explicit deferral; the other three cannot be resolved merely by leaving types, APIs, or storage to later implementation.

Resolving those issues would establish a firmer semantic basis for human approval and subsequent planning. This review itself neither approves the model nor authorizes implementation, mutation, phase changes, or external adoption.
