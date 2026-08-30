# HCM parent-promotion lineage

Date: 2026-08-25

## Bottom line

- The original seed documents do not define a `parent_promotion_eligible` field, a parent entity, a parent-binding rule, or a parent-mutation workflow.
- The old planning stack introduced `parent_promotion_eligible` later as a separate derived result beside local closeout. In that old design, it was explicitly non-lifecycle and non-mutating.
- Current HCM-5.2 correctly removed parent promotion from `handbook-contracts` until a later owner is chosen.
- The narrowest responsible correction now is an authority/planning correction, not a code change: preserve the separate fail-closed eligibility concept, but explicitly defer the parent domain, binding, owner, and any mutation.

## Source hierarchy used

1. Seed intent:
   - `docs/ideas/handbook-contract-membrane-architecture-memo.md`
   - `docs/ideas/substrate_executable_contracts_architecture.md`
   - archival fuller memo at `archive/handbook-contract-membrane-pre-clean-20260812:docs/ideas/handbook-contract-membrane-architecture-memo.md`
2. Current authority and source:
   - `docs/specs/handbook-contract-membrane/*`
   - current Rust source and Cargo graph
3. Historical lineage only:
   - stable historical snapshot `8605d6539355f350c55b921d95fb450bca30ee18`
     (also reachable as `archive/handbook-contract-membrane-pre-clean-20260812`)

## 1. What the seeds do and do not say

The checked-in seed memo is now only a short lineage stub. It says Handbook owns contract meaning, specialized validators contribute bounded evidence, and semantics stay transport-neutral; it does not define parent promotion at all. Source: `docs/ideas/handbook-contract-membrane-architecture-memo.md:7-20`.

The archived fuller memo adds important adjacent intent:

- sessions write durable handoff records, and Snapshot Memory compares prior-end to fresh-start state before grounding a new session; source: `archive/handbook-contract-membrane-pre-clean-20260812:docs/ideas/handbook-contract-membrane-architecture-memo.md:63-67`
- Charter intake and promotion are already their own semantic lane; source: `archive/handbook-contract-membrane-pre-clean-20260812:docs/ideas/handbook-contract-membrane-architecture-memo.md:67-67`
- the membrane should feed real closeout decisions and downstream gates, but Substrate owns how those results are used in its workflows; source: `archive/handbook-contract-membrane-pre-clean-20260812:docs/ideas/handbook-contract-membrane-architecture-memo.md:16-18`, `112-117`, `334-349`

The Substrate executable-contracts seed defines gates broadly enough to block a seam, merge, release, or agent handoff, and it separately discusses stateful workflow and orchestration/session lifecycle contracts. It still does not define any parent-promotion field, parent identity, or mutation rule. Source: `docs/ideas/substrate_executable_contracts_architecture.md:147-149`, `522-530`.

Conclusion from the seeds: parent promotion was never concretely specified there. The seeds support "contract results later inform workflow decisions," but not "the parent is X" or "promotion mutates Y."

## 2. Where `parent_promotion_eligible` came from

Reverse history trace shows the first introduction of `parent_promotion_eligible` in HCM docs at commit `f9d52b8a1421ed63be8b93ea27c53774bf1d26a1` on 2026-07-15 (`docs: checkpoint HCM-0.5 planning review`), via `git log --reverse -S "parent_promotion_eligible" -- docs/specs/handbook-contract-membrane`.

In the old planning stack, the field appears as part of HCM-0.5 design material:

- old target architecture listed "local-closeout, and parent-promotion semantics" alongside contract/gate semantics; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/01-target-architecture.md:77-80`
- old semantic model said evaluation state, claim verdicts, local closeout eligibility, and parent promotion eligibility are derived records, not contract lifecycle state; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/02-semantic-model.md:864-864`
- old semantic model said the two eligibilities are independently computed from exact policies and default false on indeterminate state; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/02-semantic-model.md:898-898`
- old gate/gates doc repeated that local closeout and parent promotion are computed separately and that local pass never implies parent promotion; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md:5050-5050`
- old phase map summarized HCM-5.2 as computing local closeout and parent promotion separately; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/04-phase-slice-map.md:720-726`

This field is therefore a historical planning construct, not a seed-defined product primitive.

## 3. What the old plan actually specified

The old plan did specify one important semantic boundary: `parent_promotion_eligible` was a pure derived eligibility result, not a lifecycle transition and not a mutation.

Evidence:

- "derived records; none is a contract lifecycle state"; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/02-semantic-model.md:864-864`
- Flow and pipeline may return evidence, omissions, and refusal/local-completion status, but "neither consumer evaluates `GateResult`, derives a parent-promotion value, or supplies a default"; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md:5052-5060`

What the old plan did not specify:

- no concrete parent entity
- no binding from an evaluated subject to that parent
- no owner that evaluates parent-promotion policy
- no actual parent mutation API/receipt/transition

That missing specificity is why "implement the old field literally" is still underdetermined.

## 4. What adjacent old concepts imply, and what they do not

The old material gives some adjacency clues:

- orchestration, handoff, Snapshot Memory, and session-grounding were one major nearby theme; source: archival memo `63-67`
- Flow/pipeline evidence and local-completion status were the specific upstream seam mentioned next to future parent-promotion evaluation; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md:5052-5060`
- Charter promotion was already a different lane with its own approval/promotion semantics; source: snapshot `8605d653` at `docs/specs/handbook-contract-membrane/01-target-architecture.md:286-295`

That supports one plausible inference: the historical "parent" was more likely a workflow/orchestration parent than the project Charter. But that is still inference, not textually defined product truth.

## 5. Current authority and source truth

Current HCM authority is explicit that HCM-5.2 ends at a sibling local-closeout result and leaves parent promotion for later:

- HCM README says HCM-5.2 landed separate local closeout and the explicit parent-promotion exclusion/deferred-owner boundary; source: `docs/specs/handbook-contract-membrane/README.md:99`
- architecture context says local closeout is a sibling derived result, any future parent-readiness result must remain separate and non-mutating, and actual parent mutation is a later authority boundary; source: `docs/specs/handbook-contract-membrane/architecture-context.md:99-110`
- architecture context further says later dependency, posture, or promotion owners decide what gate and local-closeout answers mean; source: `docs/specs/handbook-contract-membrane/architecture-context.md:208-226`
- phase map says gate-owned local closeout moved to HCM-5.2 while parent-promotion eligibility remains separately deferred, and future authority must independently select its parent model, binding, owner, policy, currentness inputs, and consumer; source: `docs/specs/handbook-contract-membrane/phase-and-slice-map.md:91-95`, `202-203`
- HCM-5.2 says parent promotion is a later independent owner/result, no `parent_promotion_eligible` field exists, and local eligibility alone can never imply promotion; source: `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md:119-121`, `147-152`, `279-291`, `483-503`

Current lifecycle vocabulary also does not define the needed parent domain. It defines slices, packets, meta tasks, and task sessions, but no "parent work unit" consumed by HCM-5.2 results. Source: `docs/specs/handbook-contract-membrane/work-unit-lifecycle.md:7-14`.

Current grounding remains precursor-only on this exact question:

- implemented baseline says the legacy `GroundingEvidence::local_closeout` and `parent_promotion` accessors are inert placeholders that both report `Unavailable`; source: `docs/specs/handbook-contract-membrane/implemented-baseline.md:69-74`
- current Engine code returns `Unavailable` for both; source: `crates/engine/src/grounding.rs:275-280`
- Flow only carries that placeholder grounding evidence through `GroundedPacket`; source: `crates/flow/src/grounded_packet.rs:14-20`, `72-80`

So current source does not provide an authoritative parent/work-unit model that can safely bind `LocalCloseoutResultV1` to a parent promotion policy today.

## 6. Current promotion code that is real, but separate

Current repo code does contain real promotion semantics elsewhere:

- Flow resolver reads committed Charter authority with `promotion_ref` and `lifecycle_transition_ref`; source: `crates/flow/src/resolver.rs:138-141`, `780-783`
- current posture/SDK tests call `CharterPromotionWorkflowServiceV1::promote(...)`; source: `crates/sdk/tests/direct_typed_posture_api.rs:327-333`

That is evidence against treating HCM-5.2 parent promotion as implicitly "the Charter parent." Charter promotion already has its own subject, approvals, and persistence model.

## 7. Owner and dependency constraints in current code

Current HCM-5.2 already documents the owner separation:

- `handbook-contracts` owns gate/local-closeout truth; a later promotion/posture owner may consume `LocalCloseoutResultV1`; source: `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md:129-149`
- the placement is intentionally acyclic; source: `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md:151-152`

The Cargo graph explains the practical constraint:

- `handbook-contracts` already depends on `handbook-engine`; source: `crates/contracts/Cargo.toml:18-19`
- SDK depends outward on contracts, engine, flow, and pipeline; source: `crates/sdk/Cargo.toml:32-37`

That means putting parent-promotion policy into Engine would create an Engine<->Contracts cycle if Engine had to consume `LocalCloseoutResultV1`. Placing it directly in Flow or pipeline would avoid that cycle mechanically, but would risk moving semantic policy into workflow consumers without current product authority.

So "a separate owner above or beside contracts/engine" is a plausible architecture consequence, not yet a proven product decision.

## 8. Evidence vs inference

| Statement | Status | Basis |
| --- | --- | --- |
| The original intended behavior included a separate parent-promotion eligibility concept | Proven | snapshot `8605d653` semantic model and gate docs explicitly define a separate derived eligibility result; source: `02-semantic-model.md:864`, `898`; `05-contracts-schemas-and-gates.md:5050` |
| That concept was read-only / non-mutating | Proven | snapshot `8605d653` semantic model says derived records are not lifecycle state; its HCM-3.5 boundary says Flow/pipeline do not derive or default parent-promotion values; source: `02-semantic-model.md:864`; `05-contracts-schemas-and-gates.md:5052-5060` |
| Mutation should be later than eligibility | Proven in current authority; compatible with old plan | current HCM-5.2 defines no policy, receipt, mutation, or operation; source: `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md:499-503` |
| The intended parent is a Flow/pipeline work-unit parent | Plausible inference, not proven | snapshot `8605d653` ties future parent-promotion evaluation to Flow/pipeline evidence and local-completion status, but never names a parent entity; source: `05-contracts-schemas-and-gates.md:5052-5060` |
| The intended parent is the Charter | Unsupported and likely wrong | Charter promotion is already a distinct semantic lane in both snapshot `8605d653` and current code; source: `01-target-architecture.md:286-295`; `crates/sdk/tests/direct_typed_posture_api.rs:327-333` |
| A purpose-named owner above/beside Contracts and Engine is the right implementation shape | Plausible architecture inference | current HCM-5.2 requires a later separate owner, and current dependency direction makes Engine ownership cyclic; source: `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md:147-152`; `crates/contracts/Cargo.toml:18-19` |

## 9. Narrowest correction we can responsibly land now

### (a) Product implementation ready now

None.

There is not enough source-backed product truth to implement parent-promotion policy safely today. The missing decisions are the parent domain, the subject-to-parent binding, the owner, and whether any later mutation exists.

### (b) Authority/planning clarification ready now

Yes.

The strongest correction supported by the record is:

- preserve the old semantic intent that parent promotion is a separate, fail-closed, non-mutating eligibility result
- explicitly state that the old plan never defined the parent entity or owner
- explicitly reject reading that field as Charter promotion
- explicitly state that HCM-5.2 was correct to stop at `LocalCloseoutResultV1`

### (c) Exact user/product decision still required

These decisions still need product authority:

- what the parent actually is
- how an evaluated subject binds to that parent
- which owner evaluates parent-promotion policy
- whether the first follow-up lands eligibility only or also a mutation path

## 10. Bounded options

### Option 1: authority-only correction now

Land a small planning/authority correction that says the historical intent was a separate derived eligibility result, but the parent domain and owner are unresolved and remain future work.

Tradeoffs:

- safest and fully source-backed
- resolves the current ambiguity cleanly
- does not yet unlock code implementation

### Option 2: define a new read-only parent-promotion eligibility slice

First add authority that chooses a parent domain and exact binding, then land a pure `ParentPromotionEligibility` result with no mutation. It would consume `LocalCloseoutResultV1` plus its own policy/currentness checks.

Tradeoffs:

- most faithful to the old semantics
- unlocks a real implementation target
- still requires an explicit product decision first

### Option 3: bind it immediately to Charter promotion or another existing promotion lane

Reuse existing promotion machinery by declaring that the "parent" is the Charter or another already-promotable object.

Tradeoffs:

- fastest if a product decision forces that direction
- least supported by the source record
- high risk of changing product semantics rather than correcting them

## Recommended next step

Choose Option 1 immediately, then plan Option 2.

Concretely:

1. Correct the authority/planning language so the historical field is described as "separate derived eligibility, parent still undefined," not as an omitted known-parent implementation.
2. Open a follow-up planning slice whose first job is to define the parent domain and owner.
3. Do not dispatch implementation until that slice answers those product questions.

That path preserves the real old intent without inventing a parent identity that the seeds and current authority never actually chose.
