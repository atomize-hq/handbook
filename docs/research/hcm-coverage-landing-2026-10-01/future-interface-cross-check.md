# Future obligations and public interface cross-check

Baseline: `b0e73fc54f28d80c3f8f58d37b939db41bbbf0e4`, 2026-10-01.
This is focused research reconciliation, not the controlling final review,
implementation authority, or runtime certification. Only this report was written.
The [research brief](README.md) governs scope. Read the future inventory and all
five area reports; checked the primary passages and current sources cited below.
Detailed subsection/scenario coverage remains in those reports rather than being
duplicated here.

## Stable compatibility conclusions

**V-01 — The approved model fits the inward owner architecture; delivered shapes
do not establish its missing behaviors.** **Already covered narrowly:** current
[Contracts exports](../../../crates/contracts/src/lib.rs#L7),
[SDK exports](../../../crates/sdk/src/contract_membrane/mod.rs#L59), and
[SDK dependencies](../../../crates/sdk/Cargo.toml#L37) put semantic owners inward
of consumer composition. The [current run](../../../crates/contracts/src/evaluation_run.rs#L75)
contains exact contract/lifecycle/subject/case/Resolution/policy/nonce bindings;
it does not contain the new complete requirements/source/use/intent, Purpose,
authorization/readiness, basis/frontier/set/final-context chain. Those gaps are
**missing in the inspected route**, corroborating S-10–S-25/C-01–C-16/T-02–T-03.
[Current gate/local composition](../../../crates/contracts/src/gate_composition.rs#L175)
rederives current verdicts and Contracts-owned local eligibility rather than
resolving the approved Work Progression determination. **Change needed** for
selected target behavior; future consumers cannot manufacture that truth.
[The approved guide](../../specs/handbook-contract-membrane/semantic-model/README.md#current-product-versus-proposed-model)
imposes no V1 alias, migration bridge, dual-write, or representation compatibility.

**V-02 — Generic consumers preserve admitted current responses, with operation-
specific transport coverage.** **Already covered narrowly:**
[CLI invocation/framing](../../../crates/cli/src/capability.rs#L249) checks exact
capability and transport, invokes the SDK consumer, then emits canonical JSON;
[SDK consumer](../../../crates/sdk/src/contract_membrane/consumer.rs#L146) selects
the exact capability and readmits the response;
[Tauri registry](../../../crates/tauri/src/lib.rs#L126) contains three generic
commands and [invocation](../../../crates/tauri/src/lib.rs#L281) calls the SDK.
Neither path computes Contracts truth. This is a real non-UI facade, not proof of
HCM-6.4 GUI delivery. [Inventory selection](../../../crates/sdk/src/contract_membrane/operation.rs#L780)
is asymmetric: posture apply selects Rust SDK/Tauri; artifact, Flow, and Pipeline
ordinary rows select CLI/Tauri; contract, Dock, and snapshot rows select all
three. Consequently, require the selected behavior's actual exposed consumers,
not universal Rust/CLI/Tauri parity or one API per semantic object.
[CLI evidence tests](../../../crates/cli/tests/hcm_5_2_packet7.rs#L139) assert
discovery/schema/framing and repository-unavailable verdict/gate responses;
those assertions are narrower than successful approved-model evaluation/replay.
Live consumers, installed binaries, and external use remain **unverified** here.

**V-03 — Public meaning must close with each selected owner behavior.**
[Preparation responsibility](../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#evaluation-preparation),
[evidence responsibility](../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md#semantic-responsibility-map),
and [action responsibility](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#semantic-responsibility-map)
agree: SDK/transports carry exact owner outcomes; they do not select policy,
mint semantic occurrence identity, resolve historical authority themselves, or
reinterpret refusals. R-25/S-29/C-17/T-21/U-16 correctly make interface closure
cross-cutting. For every selected exposed increment, close the needed owner
exports, SDK mapping, exact schemas/operation admission/catalog manifests and
bounded consumer verification together with identity, refusal, currentness and
required history. Concrete names/count/physical types are **decision-dependent**;
neither an omnibus final integration phase nor 62-row full closure is required.

**V-04 — Numeric and explanation contracts require explicit owner semantics
before GUI or adapter promises.** **Change needed/missing:** the current
[ContractValue numeric variants](../../../crates/sdk/src/contract_membrane/bounds.rs#L496)
are safe integers, with [floating-number rejection](../../../crates/sdk/src/contract_membrane/bounds.rs#L675).
That does not itself prevent exact finite-decimal/ratio/quantity representations,
but generic JSON framing does not supply their definition, unit/scale, bounds,
conversion, or trace. Preserve T-09–T-11's scoped gap without selecting a new wire
encoding here. [Canonical explanation and its views](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#projection-and-representation-rules)
are separate: Contracts owns full truth; density/disclosure/source availability
govern views, never another evaluation or gate. T-15–T-19's missing owner closure
therefore precedes a consumer claiming that feature. GUI layouts, view inventory,
level names/count, disclosure mechanism, and transport contracts remain choices.
The [all-N/A scenario](../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md#discriminating-scenario-corpus-1)
uses its exact named policies; no universal N/A-to-pass, block, completion, or
numeric-value rule is selected. A view must not label empty collection “tests
passed.”

**V-05 — Grounding is delivered locally; its external placement and four
reservations are separate choices.** **Already covered narrowly:** current
[four snapshot operations](../../../crates/sdk/src/contract_membrane/snapshot_operation.rs#L23),
[public snapshot/session exports](../../../crates/sdk/src/contract_membrane/mod.rs#L201),
and [inventory test assertions](../../../crates/sdk/tests/hcm_4_6_packet_5.rs#L58)
support real local seams. The same test identifies truthful omission of
`projection.definition.read`, `projection.create`, `snapshot.verify_current`,
and `snapshot.resolve_applicable`; these are not a mandatory completion quota.
[HCM-4.6's handoff](../../specs/handbook-contract-membrane/slices/HCM-4.6/README.md#phase-6-adopter-handoff-and-proof-boundary)
ends at explicit repository consumer invocation and separately requires a future
host's trusted workspace/session binding, start/end events, cancellation,
delivery/recovery, UX/operations ownership and actual adoption proof. A GUI's
planned projection workflow does not alone justify exposing those reservations.
Host/adopter selection and runtime external use are **decision-dependent/unverified**.

**V-06 — Dock and workflow adapters retain distinct integration boundaries.**
The current [validator evaluator](../../../crates/dock-json-schema/src/evaluator.rs#L244)
uses an admitted local resource registry and Rust `jsonschema` Draft 2020-12;
[its manifest](../../../crates/dock-json-schema/Cargo.toml#L21) pins the dependency.
This corroborates C-18's established-tool integration foundation, not a sandbox
program. [The corrected threat boundary](../../specs/handbook-contract-membrane/slices/HCM-5.3/README.md#threat-model-and-enforcement-boundary)
explicitly excludes arbitrary executable admission and general hostile-code
confinement as inferred successor requirements while retaining bounded actual
host controls. Further Dock producers need individually selected scope and
independent observation admission; tooling examples in history are not a
committed next-tool roster. [HCM-6.5](../../specs/handbook-contract-membrane/phase-and-slice-map.md#phase-6--consumer-adoption)
instead selects bounded profile/vocabulary/Resolution translation after an
adapter-owner decision. Neither adapter class may invent Engine authority,
claim/metric/gate truth, eligibility, or mutation receipts.

**V-07 — Future work/posture consumers must preserve separately attributable
responsibilities and the causal order.**
[Work Progression](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership)
owns local/parent reliance, while the authoritative work-structure/state owner
supplies exact state and relations. Physical placement/work model is unselected;
Contracts' existing local result and Pipeline routing writes do not prove those
propositions. [Governed completion](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#governed-completion)
requires actual governing target/coverage selection before collection or
no-collection commitment, without requiring a future Determination/Intent.
[Parent action](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action)
requires its own finite relation boundary, qualifying assessment and separately
authorized guarded commit. [Posture feedback](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#posture-feedback-boundary)
passes through independent observation/recommendation/reliance/action owners to
a later evaluation. These are **missing/change needed** in U-04/U-06/U-08/U-14's
inspected target paths; current Charter transition remains a substantive narrower
foundation. Algorithms, actor rules, models and hosts remain **decision-dependent**.
Where a consumed result role binds an Evaluation Authorization Binding,
[historical validity and selected continuing status](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#exact-current-use-and-engine-verification-rule)
must resolve; do not fabricate one for accepted upstream refusal records or other
roles with no such binding. Target/currentness and declared dependencies must
remain guarded through selected commit, not merely checked earlier.

## Complete future-inventory reconciliation

Future IDs below refer to [the inventory](future-obligations.md). Each relationship
uses the primary sources above or below; delegated implementation unknowns in the
inventory are replaced only by the area reports' bounded source evidence, never
by a blanket conformance claim.

| Future ID | Fits / change needed / decision-dependent conclusion |
| --- | --- |
| F-01 | Fits approved documentary authority; V-01 and [phase ordering](../../specs/handbook-contract-membrane/phase-and-slice-map.md#ordering-and-authority-rules) preserve current hard predecessors. No research result admits a slice. |
| F-02 | Fits V-02/V-03; exact version-bound CLI bridge/adopter/seam remains decision-dependent. Local CLI generic source is not an operating external bridge. |
| F-03 | Fits V-03; publication mechanism/exact affected versions/current external proof are decision-dependent and unverified. Cargo versions/documentation URLs are not registry or consumer proof. |
| F-04 | Fits V-03; direct Substrate participation against published versions and bridge replacement require real external evidence, unverified here. |
| F-05 | Fits V-02–V-05; current non-UI facade is foundation. Selected new requirements/results/explanation/reliance workflows need owner closure; a GUI does not supply it or automatically admit reserved projection operations. |
| F-06 | Fits V-06; workflow-adapter owner/mapping boundary remains decision-dependent, preserving exact source and Resolution meaning. |
| F-07 | Local grounding covered narrowly V-05; host/automatic trigger/adopter separately decision-dependent and unverified. Five evidence-source slots are not an inert future seam. |
| F-08 | Truthful four-row omission covered narrowly V-05; ordinary closure requires independently selected consumer and semantics, not reservation presence. |
| F-09 | Fits forward-only chain V-07; S-02–S-05/S-28/U-14 show scoped target gaps. Algorithms/notifications/acknowledgement/hysteresis/host remain deferred. |
| F-10 | Change needed V-07: current Contracts local authority is not target Work Progression. Work/state/relations/parent policy placement remains decision-dependent. |
| F-11 | Change needed V-07/U-01–U-12: historical closure, governing selection, atomic facets and distinct action/receipt chain are not supplied by generic transport terminals. No general IAM selected. |
| F-12 | Change needed V-01/R-02–R-14: [governance/permitted use](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#policy-admission) supplies upstream admitted definitions/qualification; exact current references alone do not. Criteria/realization remain decisions. |
| F-13 | Change needed V-01/V-03: S report scopes missing complete requirements, Purpose/Engine authorization/Coordination readiness. Existing profile readiness and coordinate equality are distinct propositions. |
| F-14 | Change needed V-01/V-03/V-04: C/T reports scope occurrence closure/history and independent metrics/gates/explanation gaps. Current independent candidate admission and exact arithmetic remain foundations. |
| F-15 | Fits bounded established-tool integration V-06/C-18; each additional tool scope/admission selected separately. Confinement is not an assumed future obligation. |
| F-16 | Fits current inward dependencies; each selected public meaning closes with owner behavior V-02–V-04. Exact versions/operations/publication remain decisions; no universal transport matrix or V1 bridge imposed. |
| F-17 | [Explicit deferrals](../../specs/handbook-contract-membrane/semantic-model/README.md#explicitly-deferred) remain choices constrained by semantics. Select only those needed for the next bounded outcome; do not turn every listed option/expansion into a feature. |
| F-18 | Change needed before product dispatch: [lifecycle mode](../../specs/handbook-contract-membrane/work-unit-lifecycle.md#execution-modes) and [completion transition](../../specs/handbook-contract-membrane/work-unit-lifecycle.md#state-transitions) automatically continue, contrary to the research brief's human review before each increment. Research does not edit the router or dispatch successors. |
| F-19 | Preserve completed status and qualify stale summaries using V-01/V-05/V-06 and [approved current-product boundary](../../specs/handbook-contract-membrane/semantic-model/README.md#current-product-versus-proposed-model). No historical non-goal becomes a mandatory successor feature or permanent ban. |

All five HCM-6.1–6.5 outcomes and their declared predecessor/external-owner
conditions remain in the [current phase table](../../specs/handbook-contract-membrane/phase-and-slice-map.md#phase-6--consumer-adoption).
This research creates no blanket prerequisite that all five semantic areas finish
before any Phase-6 work. A selected consumer may use already delivered bounded
capabilities, respecting current hard predecessors and actual selection authority;
only the new behavior it promises requires its corresponding semantic dependencies.
Local source/dependency compatibility is not successful runtime invocation or
current publication/adopter proof.

## Corrections and checks

- Synthesis should avoid promising universal transport parity (V-02), treating
  future GUI projection as compulsory closure of four reservations (V-05), or
  deferring all public/history/refusal work to a final integration stage (V-03).
- A historical preparation summary says every present reliance “universally”
  resolves an Evaluation Binding; the [action owner's precise consumed-role
  rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#exact-current-use-and-engine-verification-rule)
  and U-03 qualify this by whether the role's closure actually binds one. Use the
  qualified proposition in the landing map; no approved chapter was edited here.
- Read the five primary reports/future inventory and the cited primary plan,
  semantic, source/export/manifest and test bodies. No builds/tests, schema
  regeneration, production package execution, UI interaction, live publication,
  adopter check, product edit, commit or reindex occurred. Test claims mean source
  assertions inspected. This pass reconciles the material interface boundaries;
  it does not independently recertify every area-report scenario.
- `node .gitnexus/run.cjs query consumer --repo handbook --limit 3` exited 0 with
  empty results, missing Protocol/Category FTS tables, an index 20 commits behind,
  skipped foreign-worktree registry entry and Node-version warning. No graph
  result supplied evidence. Direct finite owner paths/exports were used instead.
- No indexed symbol changed; impact analysis is inapplicable. Coordinator owns
  the required pre-commit change-detection gate and controlling final review.
- Checked all 42 local Markdown references: targets, heading/explicit anchors
  and cited code-line bounds resolve; zero trailing-whitespace lines. Also read
  the proposed landing-map draft: its selected-consumer and reservation wording
  incorporates V-02/V-05's qualifications. No map edit was made by this agent.
