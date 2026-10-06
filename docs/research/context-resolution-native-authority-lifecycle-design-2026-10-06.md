# Context Resolution native authority lifecycle

Status: **proposed discovery/design for human selection**, 2026-10-06. This brief
completes bounded research; it supplies no approved implementation, migration,
runtime approval, semantic-policy change, or independent-local-check Packet 5.
The proposed slice is separate from that active slice and its planning files.

## Evidence and governing boundaries

Product source and test bodies were inspected at
`2da2760233fa1c1744664ccf0bf92e09c18b4033` on
`feat/handbook-contract-membrane-clean`. Context Resolution, artifact mutation,
lineage, and the authority-kernel tests were unchanged from that pin. Concurrent
Packet-3 edits in local authority, Contracts, Dock and SDK, including the new
`local_authority/local_evaluation.rs`, were observed as **working changes**, not
committed delivery evidence. No installed-project census or external-consumer
proof was obtained.

The [authority entry point](../specs/handbook-contract-membrane/README.md),
[architecture](../specs/handbook-contract-membrane/architecture-context.md#semantic-ownership-and-dependency-direction),
[coverage responsibility](../specs/handbook-contract-membrane/coverage-and-landing-map.md#coverage-and-responsibility),
[current-source constraints](../specs/handbook-contract-membrane/coverage-and-landing-map.md#current-source-constraints-on-the-next-outcome),
and [delivery sequencing](../specs/handbook-contract-membrane/phase-and-slice-map.md#current-delivery-sequencing)
govern interpretation. Roadmap status supplies no implementation permission.
The [primary unresolved decisions](../specs/handbook-contract-membrane/phase-and-slice-map.md#primary-unresolved-decisions)
remain with their named later owners.

**Approved semantics:** [object classification](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#object-classification),
[generic exclusion](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#generic-artifact-inclusion-and-exclusion),
[Engine responsibility](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#semantic-responsibility-map),
and [identity/currentness](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#identity-lifecycle-and-currentness-by-category)
require native owner lifecycles for non-artifact truth. Context Resolution
definitions are category 2; private envelopes, resolved profiles, exact stacks and
authority-admission lineage are category 3. The entire capsule is not one
definition. Shared persistence and identity primitives may be reused without
sharing artifact-promotion authority.

[HR-007 G1/G2](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#approved-rules)
distinguish replaceable choices from exact mechanisms, initial human decisions
from subsequently delegated operation, and recommendations from admission.
The [approval/assurance clarification](../specs/handbook-contract-membrane/semantic-model/amendments/approval-authority-and-assurance.md#independent-dimensions)
separates approval requirement, decision authority, delivery, assurance and
composition. Its selected local-advisory local-check journey does not downgrade
the existing credential-backed publication contract.
The [format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
remains **PROPOSED** except the fixed Contracts JSON choice. Neither reading it
nor this proposal authorizes re-encoding or a migration.

## Implemented behavior and the concrete mismatch

[Context Resolution](../../crates/engine/src/context_resolution.rs),
`ContextResolutionAuthorityAdmissionResolver::new` → `load_authority_snapshot`,
requires this exact private compatibility tuple:

| Component | Current selection |
| --- | --- |
| Generic kind / instance | `handbook.artifact-kind.context-resolution-authority-binding@1.0.0` / `context_resolution_authority` |
| Schema / intake | `handbook.schemas.context-resolution-authority-capsule@1.0.0` / `handbook.intake.context-resolution-authority-capsule@1.0.0` |
| Installed publication | `.handbook/project/context-resolution-authority.yaml` |
| Declaration / codec | `handbook.context-resolution-authority-declaration`, schema `1.0`; `handbook.hcm-3.2.context-resolution-authority-binding-jcs`, codec `1.0` |
| Payload | `handbook.context-resolution-authority-binding`, schema `1.0`, closed 53-leaf binding |

The meaningful [kernel test](../../crates/engine/tests/context_resolution_kernel.rs)
`seed_real_authority_repository_with_credentials` constructs repository,
resolved-profile/stack, prior registry, eleven authority-use mappings and four
candidate mappings. It wraps strict `payload_jcs` in `declaration_jcs`, then
uses generic intake, candidate validation/append and promotion. The selected
[fixture profile](../../crates/engine/tests/fixtures/hcm_3_2_context_resolution/.handbook/definitions/profiles/context-resolution-authority-1.0.0.yaml)
adds the non-family descriptor; its
[kind](../../crates/engine/tests/fixtures/hcm_3_2_context_resolution/.handbook/definitions/artifact-kinds/context-resolution-authority-1.0.0.yaml),
[intake](../../crates/engine/tests/fixtures/hcm_3_2_context_resolution/.handbook/definitions/intakes/context-resolution-authority-1.0.0.yaml),
[schema entry](../../crates/engine/tests/fixtures/hcm_3_2_context_resolution/.handbook/definitions/schemas/context-resolution-authority-1.0.0.entry.yaml),
[schema](../../crates/engine/tests/fixtures/hcm_3_2_context_resolution/.handbook/definitions/schemas/context-resolution-authority-1.0.0.schema.json)
and [selection](../../crates/engine/tests/fixtures/hcm_3_2_context_resolution/.handbook/profile-selection.json)
make that path executable. This proves a supported selected path, not that every
shipped profile includes it or that any external installation uses it.

[Artifact mutation](../../crates/engine/src/artifact_mutation.rs) specially checks
this tuple in candidate append, `promote`, `build_promotion_plan` and persisted
intent/output validation. Promotion checks exact predecessor, publication-bound
idempotency and publisher proof; replay uses historical publisher validation.
`load_authority_snapshot` nevertheless resolves through
`ArtifactRepositoryV1` and requires exact generic committed promotion bytes and
intent from `GenericArtifactLineageStoreV1`. Thus non-authored Engine authority
uses authored-artifact lifecycle semantics. The dated
[E5](hcm-coverage-landing-2026-10-01/rules-and-definitions.md#e5) and R-02 finding
is revalidated by these current bodies, not adopted as old sequencing authority.

Existing safety is valuable. `validate_capsule_value` checks duplicate-safe,
byte-identical JCS, closed shapes/bounds, predecessor quartet and prior registry.
Outer canonical YAML bytes, tagged declaration bytes, tagged payload bytes and
tagged semantic content excluding `binding_fingerprint` have **distinct**
fingerprints. `verify_publisher_authority` verifies the exact direct registry
successor, retained assertion/credential chain, challenge, signature and current
use head. `admit` authenticates a mapped use/subject and rejects changed-subject or
counter replay. `CurrentAuthorityWitness` rechecks exact committed output,
repository/profile/stack and current registry; even cached/cloned admissions can
stale. `detect_authorized_publication_gap` and lineage quarantine prevent uncertain
publication from becoming permission. These checks must survive owner separation.

## Focused owner and consumer map

| Actual seam | Established responsibility and proposed impact |
| --- | --- |
| [Definition registry](../../crates/engine/src/context_resolution_registry.rs), [profile selection](../../crates/engine/src/profile_selection.rs) | Admit exact stack/policies and resolve profiles; remain Engine-owned category-2 inputs and category-3 resolved truth. No vocabulary, rank, matcher or profile replacement is proposed. |
| Resolver/admissions/envelopes in `context_resolution.rs` | Sole authority interpretation and non-expansion kernel. Replace publication/currentness source behind sealed admissions; preserve eleven uses, exact subjects and consuming checks. |
| [Artifact repository](../../crates/engine/src/artifact_repository.rs), [generic lineage](../../crates/engine/src/artifact_lineage_store.rs), artifact mutation | Currently carry installation, transaction, displaced-history and quarantine duties. Reuse fitting lock/retained-file/atomic primitives; separate native lifecycle meaning and reserve its targets against generic writes. Keep legacy evidence readable. |
| [Approver observation](../../crates/engine/src/approver_registry_observation.rs), [authenticator](../../crates/engine/src/charter_authenticator.rs), [trusted lineage](../../crates/engine/src/charter_lineage_store.rs) | Establish credential, registry and signed publisher evidence actually consumed by the legacy path. Native lifecycle cannot mint those proofs or treat role/conversation claims as equivalent. |
| [Private projection](../../crates/engine/src/projection.rs), `ProjectionAuthorityAccess` / `validate_request`; [grounding](../../crates/engine/src/grounding.rs), `GroundingRequest` / `resolve` | Consume `ProjectionAuthorityView` from an admitted envelope, exact profile/stack/envelope pairs and ranks. Preserve source fidelity, disclosure/non-expansion and stale-view refusal. This is a direct consumer seam, not a claim that every production snapshot operation loads this legacy capsule. |
| [Native local authority](../../crates/engine/src/local_authority.rs), [local check](../../crates/engine/src/local_authority/local_check.rs), [typed setup SDK](../../crates/sdk/src/local_setup.rs) | Additive local witness and `admit_local_envelope` route already share the envelope kernel. They do not migrate generic publication. Local authority owns its declared terms/revocation; do not reinterpret it as a general legacy publisher. |
| Packet-3 working `local_authority/local_evaluation.rs` | Calls `local_path_within_selector` and `local_owner_subtree_within_selector`, as does working local-check code. Those actual shared read/write containment semantics require reconciliation and regression; implementation independence is not complete. |

Constructor callers inspected are the kernel tests and the real-authority fixture
builder in [projection tests](../../crates/engine/src/projection/tests.rs),
`admitted_envelopes::Fixture`. Engine publicly re-exports the resolver in
[lib.rs](../../crates/engine/src/lib.rs), so this bounded local map cannot establish
external API non-use. SDK setup directly calls the local owner instead. These
positive traces delimit compatibility evidence; keyword absence is not a consumer
census or proof of external independence.

## Proposed slice outcome and constituent lifecycle

**Proposal:** an Engine-owned authority publication stream, durable exact history,
and native currentness witness which can supply the existing envelope kernel
without artifact intake/candidate/promotion establishing authority. Candidate
content remains untrusted; admission, activation and present-use permission remain
distinct. Only this authority path is in scope. Complete evaluation Purpose/
occurrence authorization, Contracts qualification, memory-promotion redesign and
global six-family cleanup are separate outcomes.

| Boundary | Proposed owner rule |
| --- | --- |
| Candidate / declaration | Caller supplies exact content, definitions, expected predecessor and request identity. A transient declaration is not an admitted definition or active authority. Any retained submission provenance is separately attributable to its origin. Engine may persist a validated declaration, explicitly inactive. |
| Authorization / admission | Engine resolves applicable prior terms and independently verifies exact repository, profile/stack, mappings, scope and real approval evidence. Definition owners retain each category-2 proposition; the binding/admission is category 3. Unsupported or insufficient assurance refuses. Validation alone grants no use. |
| Initial activation | Supported explicit selection establishes one native stream only when absence of prior authority is positively proven under locks, including legacy/control/quarantine inventories. Missing files alone cannot bootstrap. Commit exact admitted content, authorization closure and current head together; report a receipt only for a proven effect. |
| Replacement / revocation | Bind the full expected predecessor and prior-authorized decision, then recheck through commit. New terms cannot authorize their own activation. Replacement retains immutable predecessors; revocation creates a terminal relationship rather than deletion. Restoring permission after revocation requires a separately authorized successor. |
| Exact current resolution | Resolve the positively selected route and exact committed head, then repository/profile/stack/registry basis and relevant use checks. Native proof must feed the same sealed admission/non-expansion checks. Stale predecessor, revoked state, conflicting head or damaged route refuses; no latest-file or permissive fallback. |
| Uncertain recovery | Distinguish validated, authorized but uncommitted, installed but unproven, committed and quarantined states. Recovery binds original request, exact bytes/predecessor/authority and effect evidence; unknown or multiple candidates refuse. A retry is not a new activation. Preserve gap quarantine even when credential authorization and native commit cannot be one atomic effect. |
| Historical resolution | Return exact retained bytes, input/definition closure, authority provenance and transition/cutover at its recorded time. Label historical truth separately from present permission. Replacement/revocation cannot rewrite old results; historical resolution never returns a usable current admission. |

Native currentness initially preserves the exercised contract's exact dependencies,
including registry/use-head sensitivity. Broader tolerance for unrelated registry
changes would be a separately selected semantic change. Existing historical
publication validation still calls live repository/profile/stack validation;
the same-profile replacement test proves displaced bytes and unique witnesses,
not arbitrary replay after profile changes. Native history must retain formation
dependencies and distinguish historical verification from current-source checks.

Reuse candidates include exact references/fingerprints, strict parsing,
[retained repository file support](../../crates/engine/src/canonical_repo_support.rs),
existing authority locks and transaction installation/recovery mechanics. Their
private visibility, lock order, ownership and recovery side effects need checking
before extraction. A native store must not call generic promotion as its authority
decision or make every generic read recover a second lifecycle.

## Compatibility alternatives and recommendation

| Alternative | Benefits | Cost / refusal boundary |
| --- | --- | --- |
| A. Explicit per-binding native selection and later adoption, retaining legacy reads/writes for unadopted roots | Limits churn; preserves installed bindings and evidence; makes cutover inspectable. A compatibility adapter can retain the old resolver entry point while resolving a positively selected native stream. | Temporarily two supported routes across different roots, one writer per stream. Native selection cannot be inferred from lookup failure. Global six-family confinement remains incomplete while the unadopted legacy writer exists. |
| B. Replace the lifecycle in place for all compatible legacy roots | Earlier removal of this generic special case and fewer ongoing write routes. Existing bytes/path can remain. | Requires a verified installed/history inventory, supported adoption authority, atomic cutover and recovery for every eligible root before release. Ambiguous roots cannot be converted automatically. Larger compatibility/reconciliation risk than the first increment justifies. |
| C. Fresh native roots only; retire legacy write support after explicit re-establishment | Simplest initial native path. | Abrupt retirement strands installed writers; re-establishment changes authority and cannot silently transfer old approvals or history. Not recommended as the complete compatibility posture. |

**Recommend A**, beginning with fresh explicit native establishment, then a
separately reviewed exact adoption/replacement outcome. Keep legacy outer YAML,
declaration/payload JCS, tags/preimages, named tuple and retained historical bytes
unchanged. Proposed native owner-managed lifecycle records use versioned JSON;
that is a boundary-specific recommendation awaiting selection, not adoption of
the general format defaults. Do not remove/rewrite installed profile descriptors,
schema/intake assets or old journal/registry records to make the model look clean.

Proposed publication keeps `.handbook/project/context-resolution-authority.yaml`
as an exact installed compatibility output, backed by native immutable records
and a native head/receipt. The new owner checks its closed codec directly rather
than requiring an Artifact Kind to grant authority. A distinct native namespace
under the existing authority state root must avoid its legacy quarantine files.
This is one controlled installation and native lifecycle, not parallel generic
and native commits or independently editable YAML/JSON masters.

For later adoption, retain the exact legacy current output, unique committed
intent/receipt and predecessor witnesses, displaced bytes, registry/assertion
evidence and selected definition closure. An explicit adoption relation identifies
that legacy history and the native starting head without renaming old identities.
Historical lookup may use a read-only legacy evidence adapter. Adoption also needs
**new, supported authorization for the cutover** under prior terms: the old signed
publication alone authorizes no new adoption action or authority upgrade.
Under a common guard, compare the expected legacy head and all required authority
basis, reserve the path, commit native cutover, and disable generic mutations of
the adopted tuple/targets, including replay/recovery writes. Recovery must account
for already authorized legacy transactions before declaring exclusive ownership.

After cutover, old resolver calls may delegate through the compatibility adapter
only for the selected exact native binding. Old generic mutation calls should
return a typed native-owner-required refusal; preserving kernel behavior does not
promise continuing generic writes on adopted roots. Any promised legacy mutation
adapter would need separate review and must call only the native lifecycle.
Pre-cutover legacy writes remain unchanged under A. Mixed-null predecessor,
missing/duplicate witness, stale registry/profile, open quarantine, unknown schema,
marker-only install, inconsistent selection or native read failure refuses
adoption/current use. Never choose one by directory order, auto-upgrade its mode,
delete ambiguity or fall back after a damaged native selection. No destructive
migration, universal dual-write or retirement deadline is selected.

## Native and SDK surface direction

Keep signatures tentative until the first increment's source/lock review. Engine
should expose bounded declaration preparation/admission, establishment,
`resolve_current(expected_head)`, exact historical read and recovery observation;
later replacement/revocation/adoption remain separate operations. A returned
opaque locator binds owner/stream/record and content identities. Closed outcomes
distinguish committed/replayed, refused/stale/conflicting and recovery-required;
historical records cannot manufacture `ContextResolutionAuthorityAdmission`.
Private authority mappings, credential evidence and envelope internals need no
new public serialization contract.

SDK composes that owner into usable transport-free requests/results and example
usage, preserves refusal and commit uncertainty, and passes sealed native authority
to the existing kernel. It cannot verify or relax authority itself. Preserve the
existing resolver's external signature where a truthful compatibility adapter is
possible; explicitly version any unavoidable new boundary. Do not select ordinary
operation rows/counts or universal DTOs prematurely. No CLI, installed-skill, GUI,
registry/IAM, credential-provider replacement, broad policy/profile replacement,
schema migration or universal lifecycle refactor belongs to this slice.

## Acceptance examples and verification direction

These are proposed product acceptance, **not tests run by this research task**.

| Example | Required observable result |
| --- | --- |
| Registered fresh root, exact profile/stack and supplied valid publisher proof; explicit native selection; activate O1 and admit a root envelope | Real Engine publication/current witness and sealed mapped-use admission without generic intake/append/promote records. Reopen and a normal-dependency SDK caller obtain the same exact binding and usable envelope. File/schema validity alone is insufficient. |
| Same request identity/exact bytes replayed; changed content or same subject ref with changed fingerprint | Exact replay has one outcome/head; changed identity/content refuses without replacing current authority. |
| Unknown/duplicate/noncanonical/oversized declaration, uncovered mapping, forged signature, bad challenge/RP/flags, credential counter or cross-resolver replay | Typed refusal preserves head/history and actual authorization accounting; no local-claim fallback for credential-required rules. |
| Replace O1→O2→O3 under authorized terms; race two O1 successors; revoke O3 | Only exact committed winner is current. Cached/cloned O1 admissions stale; revoked current use refuses. No overwrite or history loss; revocation does not erase historical permission at formation. |
| Exact historical O1/O2 lookup after replacement/revocation or changed current definition selection | Retain original bytes/fingerprints and input/authority closure. Report historical verification separately; never rebind to current definitions or return live permission from history. |
| Authorization advances registry but native commit is interrupted; installed bytes without receipt; conflicting/partial head/quarantine | Refuse operational use until exact recovery proves committed/replayed or a bounded noncommit. Unknown/multiple retry candidates remain quarantined; failure/retry never supplies a fresh bootstrap. |
| Valid legacy O1 adopted with separately authorized cutover; concurrent old publication; ambiguous legacy or damaged native selection | Preserve exact old identities/history, compare predecessor through commit, account for pending old effects and establish one writer. Refuse ambiguity and stale cutover; no fallback or assurance upgrade. |
| Generic artifact call against a selected native authority target, including profile aliases/unknown non-family targets and persisted replay | Refuse before new generic mutation; selected new publication uses only native lifecycle. Existing six authored families keep their behavior. Legacy unadopted exception remains explicitly outside any global confinement claim. |
| Narrow/broad envelope projections; local literal input read versus complete owner-subtree mutation reach | Preserve exact source bytes, omissions/ranks/non-expansion and stale-envelope refusal. Literal authorization cannot grant arbitrary descendants; exercise shared Packet-3/P4 read/write/currentness checkpoints after reconciliation. |

Use the existing [kernel test bodies](../../crates/engine/tests/context_resolution_kernel.rs)
for real-authority, replacement-chain, stale/malformed/uncovered, forged publisher,
counter/replay, cached witness and quarantine expectations. Use
[projection tests](../../crates/engine/src/projection/tests.rs), particularly
`corrected_two_envelope_matrix_varies_only_resolution_and_preserves_source` and
`stale_envelope_stack_custom_kind_policy_and_evaluator_dependencies_refuse`, for
consumer invariants. Add native owner tests for the selected path, exclusive
cutover/history/recovery and an ordinary SDK consumer; mocks cannot substitute
for owner-established publication. Future focused execution may use
`cargo test -p handbook-engine --test context_resolution_kernel` and
`cargo test -p handbook-engine projection::tests`; run selected local-authority
and SDK regression targets from the reconciled source. Use isolated execution
when needed, not shared builds against concurrent edits. No whole-workspace suite
or external publication proof is necessary to finish this brief.

## Increment boundary, dependencies and human decisions

**First proposed increment:** on an explicitly selected, registered fresh root,
establish one credential-preserving native authority stream, reopen it, admit a
root envelope through the real kernel, and expose current/exact historical read
and bounded recovery through a usable typed SDK example. Scope includes native
declaration/activation/head/receipt, retained identities, authorization gap
accounting and exclusion of its targets from generic mutations. It includes no
legacy adoption or legacy writer retirement. Prove positive absence across the
relevant legacy/native state, exact supplied publication authority, actual commit,
restart/replay, interrupted-commit refusal and shared envelope/path behavior.
Keep the existing legacy route available only for unselected roots.

This is an independently observable authority/read/refusal outcome, so it ends
at a **packet human-review boundary** after proportionate verification and one
focused causal review. Preparatory internal work has no separate delivery claim.
Its exact storage contract, lock order, supported selection/authority rule and
public signatures need bounded preparation after the decisions below; no packet
dispatch or implementation plan is authorized by this document.

Later increments stay broad: native replacement/revocation and complete historical
verification; explicit adoption/exclusive legacy cutover if selected; then focused
consumer and confinement closure for the chosen scope. Their dependencies follow
the native stream/history and supported prior authority, not arbitrary packet
numbers. Each independently meaningful outcome returns for human review. Do not
turn this outline into detailed successor packets.

Discovery has no blanket P3/P4 predecessor. Product execution must use a stable
selected source and reconcile the actual shared kernel/path helpers, local witness
currentness/revocation, registry lock ordering and consumer behavior with the
then-landed local-check work. Packet 3/P4 do not supply authority for this stream;
this stream does not replace their local operating/source/evaluation authority.
Unknown external users and installed legacy inventories constrain any release or
retirement claim, not completion of this design proposal.

The human still needs to select:

1. **Compatibility:** A (recommended), B, or a narrower fresh-only outcome, and
   whether installed-binding adoption belongs in this slice's later outcome.
   No adoption or destructive conversion is implicitly selected.
2. **First authority route:** preserve the existing credential-backed contract
   for the native first path (recommended for this mismatch), or separately
   extend a supported local-advisory rule. The latter needs exact scope/rule and
   prior decision authority; local-check's selected intent is not a universal
   successor. Credential acquisition is not imposed on the unrelated local-check
   journey, and unavailable credentials cannot downgrade this legacy contract.
3. **Representation/publication:** retain the exact capsule/preimages and installed
   YAML path with separate versioned native JSON lifecycle records (recommended),
   or select a versioned native codec/path with an explicit compatibility contract.
   New framing and fingerprint rules require selection, not generic pretty-printing.

These choices can be discussed together. Human selection then permits preparation
of the concrete first increment and its authorization boundary; this completed
research is not implementation readiness or permission to dispatch it.

Research verification used authority/source/test-body inspection and local
reference/whitespace/scope checks; no product suites or external host work ran.
GitNexus query/context were attempted with the cached 1.6.12 runner: foreign
storage warnings, missing `Protocol`/`Category` tables and unresolved resolver
prevented reliable graph evidence. Local metadata pins the index to `a35297d`,
46 commits behind the inspected source. No index rebuild was attempted. Direct
source/caller inspection supplements this limitation; no product symbols changed.
