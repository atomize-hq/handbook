# Proof and Regression Ledger

## Purpose

This ledger records what is actually proven, what remains only architectural intent, and which current behaviors must survive target-architecture work.

It is not a task checklist. Slice-local `tasks/todo.md` files own execution status.

## Proof levels

| Level | Question answered |
|---|---|
| `Exists` | Does an artifact, type, command, crate, or test exist? |
| `SemanticallyCorrect` | Does it encode the approved target meaning? |
| `BoundaryLanded` | Does the correct owner expose/enforce it? |
| `RealPathAdopted` | Does a real product path use that boundary? |
| `RuntimeProven` | Does required runtime/e2e/negative evidence exercise the path? |
| `ReviewClean` | Has an independent review found no remaining actionable issue? |

No lower proof level implies a higher one.

## Current proven baselines

### `PR-001` — Published owner crates

**Current evidence:**

- `handbook-engine = 0.1.1` is published;
- `handbook-flow = 0.1.1` is published;
- `handbook-pipeline = 0.1.2` is published;
- the released pipeline proof rejects path dependencies and checks registry provenance;
- prior dedicated Substrate proofs showed real published engine/flow and pipeline consumption in bounded seams.

**Classification:** `ContractCorrectAndProven` only for the exact published APIs and proof seams exercised.

**Must preserve:**

- registry-only proof;
- exact version assertion;
- real downstream seam;
- distinction between engine/flow proof and pipeline proof;
- no claim that every future membrane API is already importable.

**Current adoption boundary:** the current inspected Substrate checkout pins exact Handbook crate versions but contains no live Handbook API call, and its pipeline pin is behind the current Handbook workspace version. The prior dedicated worktree evidence remains valid feasibility/real-seam proof only for the exact published APIs and throwaway seam it exercised; it does not make the future membrane SDK or current Substrate checkout adopted.

### `PR-002` — Structured baseline input parsing

**Current evidence:** engine exposes typed YAML parse/validate models for Charter, Project Context, and Environment Inventory.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic typed parsing/validation value, not Markdown authority.

**Must not overclaim:** the existing per-artifact Rust models are not a generic artifact-kind/schema registry, and the current templates/directives are not a versioned adaptive intake coverage contract.

### `PR-003` — Deterministic Markdown rendering

**Current evidence:** engine exposes deterministic Markdown renderers for the three baseline authoring families.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic renderer-derived human-review views where still valuable.

**Must not preserve:** independently editable Markdown as canonical truth.

### `PR-004` — Trusted repo-relative artifact access

**Current evidence:** canonical loading/path contracts enforce bounded repo-relative access and reject unsafe states such as disallowed symlinks.

**Classification:** `BoundaryLanded` for current fixed artifacts.

**Must preserve:** trusted path normalization and no-follow behavior when descriptors become dynamic.

### `PR-005` — Work-level scoped rule filtering

**Current evidence:** pipeline stages carry work levels and compiler inclusion filters honor scoped blocks.

**Classification:** `UsefulPrecursor`.

**Must preserve:** the ability to select relevant rule/context sections for a declared working scope.

**Must not preserve:** L0-L3 as the final mixed taxonomy if Context Resolution replaces it.

### `PR-006` — Doctor JSON baseline

**Current evidence:** `handbook doctor --json` emits a typed serialized report.

**Classification:** `UsefulPrecursor`.

**Must preserve:** machine-readable baseline/refusal/next-action semantics.

**Gap:** JSON parity and common envelope do not yet cover all commands.

### `PR-007` — Flow resolver typed decisions

**Current evidence:** flow exposes `resolve_with_contract`, typed selection, refusal, blockers, budget outcome, and next actions.

**Classification:** `BoundaryLanded` for the current reduced request model.

**Must preserve:** typed semantic decisions and consumer-owned rendering.

**Gap:** no profile or Context Resolution input; byte budgets are not semantic projections.

### `PR-008` — Narrow snapshot and fingerprint primitives

**Current evidence:** engine freshness/manifest records compute deterministic fingerprints; pipeline route basis records revision/fingerprint state; capture logic uses rollback snapshots for write safety.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic normalization/fingerprint value, revision-bound route provenance, and safe capture rollback behavior.

**Must not overclaim:** these primitives do not implement general Snapshot Memory, strategic capture hooks, project/world state records, snapshot deltas, drift analysis, or Resolution-aware snapshot projection.

### `PR-009` — Charter questionnaire-shaped coverage and posture validation

**Current evidence:** the Charter structured-input template and engine types retain project shape, constraints, operational reality, posture, domains, nine engineering dimensions, exceptions, debt, and decision-record fields; validation refuses incomplete/placeholder required content and rendering emits deterministic posture sections.

**Classification:** `UsefulPrecursor`.

**Must preserve:** the semantic coverage and deterministic validation value unless a Phase 0 decision explicitly removes or replaces an item.

**Must not preserve:** a rigid terminal questionnaire, prompt-owned authority, Markdown as canonical truth, or one artifact-specific implementation as the generic intake architecture.

## Open program proof gates

| Gate | Required proof | Current state |
|---|---|---|
| `PG-PROFILE-01` | selected profile resolves complete artifact/vocabulary/Resolution truth with deterministic fingerprint | open |
| `PG-DEFAULT-01` | focused research plus a user brainstorming/decision session explicitly approve the shipped kind set, default instances, and requiredness; examples/current enums do not count | closed for the HCM-0.6 documentation decision by its exact approved record, final proof wall, clean independent review, and two-commit closeout; no runtime/profile publication proof |
| `PG-KIND-01` | a versioned `ArtifactKindDefinition` resolves a safe canonical schema, validation, optional intake, lifecycle, and projections independently from repository instance state | open; HCM-1.1 proves the exact capability-free kind/schema resolution and structural-validation subset only, while lifecycle and Projection coverage remain unimplemented |
| `PG-KIND-02` | repository-defined custom kind registers, passes meta-schema/structural validation, and exercises supplied intake coverage without a new Rust enum variant, executable hook, remote schema fetch, generated CLI command, or pre-Phase-3 generic projection engine | open; HCM-1.1 proves registration, local meta-schema/structural validation, and enum/remote-hook/CLI/Projection non-adoption for a two-kind custom fixture, while supplied intake coverage remains unimplemented |
| `PG-ARTIFACT-01` | a profile-selected `ArtifactInstanceDescriptor` binds a kind to path/label/requiredness/dependencies and participates in validation/doctor/flow | open |
| `PG-INTAKE-01` | guided-adaptive, express, and agent-assisted acquisition use one intake definition and produce the same candidate schema while exposing missing coverage | open |
| `PG-INTAKE-02` | intake provenance distinguishes user declarations, evidenced inference, defaults, unknowns, contradictions, waivers, and approvals; normative fields cannot be silently inferred into authority | open |
| `PG-CHARTER-01` | `CharterIntakeDefinition` covers approved questionnaire domains, promotes only an approved schema-valid candidate to canonical Charter YAML, and deterministically renders Markdown as a renderer-derived human-review view | open |
| `PG-YAML-01` | one artifact family is canonically YAML, structurally validated, and deterministically rendered | open |
| `PG-YAML-02` | no dual editable Markdown/YAML truth remains for converted families | open |
| `PG-VOCAB-01` | lexical and structural conflation render correctly without losing stable role resolution | open |
| `PG-RES-01` | six-dimension envelope validates inheritance, authority, memory, and validation horizons | open |
| `PG-PROJ-01` | same source truth yields multiple deterministic Resolution projections with provenance | open |
| `PG-PROJ-02` | omitted required claims remain visible and cannot false-pass | open |
| `PG-SNAP-01` | same selected stable state and capture policy produce the same normalized state fingerprint with deterministic ordering | open |
| `PG-SNAP-02` | capture detects concurrent revision changes and returns stable, bounded, or non-promotable unstable truth honestly | open |
| `PG-SNAP-03` | prior top-level closeout and current parent-resume snapshots produce a deterministic delta that detects stale handoff and unexplained drift | open |
| `PG-SNAP-04` | Resolution-aware snapshot projection includes only authorized/relevant fields and enumerates omissions | open |
| `PG-SNAP-05` | redaction tests prove secrets, unsafe environment values, and unrestricted diff/command contents are excluded | open |
| `PG-SNAP-06` | planned-versus-actual signals distinguish durable justified divergence from unexplained scope/proof/semantic drift | open |
| `PG-POSTURE-01` | identical Charter/override/condition/evidence state resolves the same `ProjectPostureKernel` fingerprint without creating a second editable authority | open |
| `PG-POSTURE-02` | policy-defined hard and accumulated triggers produce evidence-linked advisory recommendations with typed notification/acknowledgment; only authorized transitions change policy, and lowering honors sustained-evidence/floor/red-line rules | open |
| `PG-SDK-01` | CLI and direct Rust consumer call the same use case and receive equivalent typed results | open |
| `PG-JSON-01` | every supported nontrivial CLI operation emits one schema-valid JSON envelope | open |
| `PG-TAURI-01` | thin Tauri command adapter serializes the same SDK DTO without CLI subprocess | open |
| `PG-CONTRACT-01` | one exact independently locked then active contract definition drives claim evaluation while lifecycle/evaluation state remain separate | open |
| `PG-DOCK-01` | one exact content-addressed external process validator runs under the frozen protocol/isolation/Resolution contract and its candidate is admitted as canonical evidence by the membrane | open |
| `PG-GATE-01` | complete claim partition composes deterministically; hard failure and required not-observed block regardless of weighted score | open |
| `PG-SUB-CLI-01` | Substrate uses exact bundled CLI/schema in a real replaceable seam | open |
| `PG-PUBLISH-01` | new downstream-intended API passes exact crates.io external consumer proof | open |
| `PG-SUB-RUST-01` | current-tip Substrate worktree uses exact new crates.io API in a real seam | open |
| `PG-HANDOFF-01` | version-routed schemas validate parent-owned true-stop handoffs; the ledger rebuild is byte-identical; immutable history/supersession, scoped stop/resume, and repository-relative refs work without internal subagents writing global records | HCM-0.8 validation and two-commit negative proof complete; closes only when the HCM-0.8 v1.2 parent record and rebuilt ledger validate |
| `PG-HANDOFF-02` | once snapshots land, handoffs reference start/end snapshots and delta; orchestration rechecks current state before dispatch | open |
| `PG-ORCH-01` | an explicitly selected phase/slice remains owned by one active parent that executes built-in `default` subagents, captures identity/status, collects results, and completes review -> valid-finding remediation -> different fresh review without an ordinary user-mediated task hop | proven by the HCM-0.8 one-parent multi-round review/remediation loop; final lineage capture belongs in the v1.2 parent record |
| `PG-ORCH-02` | every current internal dispatch declares execution target, parent ID, role, replayable subject manifest/fingerprint, fresh-context requirement, closeout owner, ordered required-skills chain beginning with `using-agent-skills`, and complete structured return contract; unavailable mandatory delegation fails closed | proven by HCM-0.8 current-schema dispatch validation and fail-closed orchestration self-tests |
| `PG-CATALOG-01` | proposed proof that the frozen HCM-0.4 `05` payload was mechanically decomposed without semantic delta | retired without proof; HCM-0.9 was abandoned after terminal Redesign Review 2 was not CLEAN, no decomposition occurred, and the monolith remains canonical |

## Greenfield deletion gates

Temporary scaffolding may be introduced only when a row is added here first.

| Bridge ID | Architectural purpose | Allowed lifetime | Deletion proof |
|---|---|---|---|
| `BR-SUB-CLI-01` | let Substrate consume the versioned Handbook JSON protocol before the permanent published-Rust boundary is available | may enter the normal path only in HCM-6.1 after `PG-JSON-01`; remains isolated and replaceable until HCM-6.3 | `PG-PUBLISH-01` and `PG-SUB-RUST-01` pass for the replacing exact API/seam; the normal Substrate path no longer spawns/parses the Handbook CLI; bridge-specific dependencies/config/tests are removed; standalone Handbook CLI remains unaffected |

There is no approved user migration tool, legacy importer, dual-read mode, or compatibility profile.

## Regression rules

Every implementation slice must preserve applicable baselines:

1. trusted repo-relative/no-follow filesystem behavior;
2. deterministic structured parsing and rendering where retained;
3. typed refusal/blocker/next-action semantics;
4. published owner-crate boundaries not explicitly replaced;
5. registry-only released proof for public APIs;
6. consumer-owned product wording;
7. strict separation of docs/artifacts/evidence from contract authority;
8. no human-output parsing by machine consumers;
9. no promotion beyond evidence Resolution;
10. no implicit legacy compatibility commitment;
11. snapshot records remain immutable, descriptive, redacted, and separate from canonical/transition authority;
12. comprehensive snapshots are never injected wholesale into a narrower agent context;
13. artifact kinds remain distinct from profile-selected repository instances;
14. custom schemas/kinds do not create executable hooks, Rust enum requirements, or dynamic CLI commands;
15. all intake modes converge on one canonical schema and expose missing coverage/provenance;
16. agent inference cannot promote constitutional or normative decisions without required approval;
17. posture recommendations remain advisory and cannot auto-mutate Charter policy.
18. internal delegated agents do not write canonical handoffs or append the global ledger;
19. writing an internal dispatch is not orchestration completion and does not create an ordinary user task hop;
20. implementation/documentation agents cannot self-review; valid findings require remediation and a different fresh reviewer;
21. mandatory built-in delegation cannot be replaced by shell-launched agents, external Codex processes, temporary-file transport, or filesystem polling.
22. completed orchestration cannot use a queue-shaped `review_required` stop or a stop/status/resume combination inconsistent with the true boundary;
23. findings close only through typed successful parent/delegated remediation followed by a different fresh review of the remediation result fingerprint;
24. final review evidence binds a replayable sorted repository-path/SHA-256 subject manifest, not a free-form fingerprint string.
25. completed two-commit closeout replays the final review manifest against the primary reviewed commit while validating the mechanically rebuilt post-closeout ledger separately.
26. unavailable mandatory delegation maps only to `capability_unavailable`/`blocked`/`top_level_resume`, and that mapping is enforced in both directions.
27. a completed closeout records every findings -> remediation -> completed fresh re-review edge, permits a re-review to discover another remediated round, and requires the final completed review to be clean.
28. for completed closeout, `reviewed_state.baseline_head` equals `repo_state.head`; a valid manifest from another commit cannot authorize an unreviewed primary slice commit.
29. schema, kind, capability, capability-dependency contract, intake, renderer, profile-parent, vocabulary, posture, and other HCM-0.2 definition refs derive mechanically as `identity@version`; every referenced definition has a recomputable uniform fingerprint producer, the graph is acyclic with intake compatibility owned only intake -> kind, and range/latest/ambient/bare-ref fallback is refused.
30. a resolved artifact instance has one concrete safe repo-relative canonical path; templates and absolute paths do not survive resolution.
31. fixed renderer-definition refs and generic Projection-definition refs remain separate; declaring a future compatible Projection definition never authorizes HCM-0.2/Phase 2 to produce a capitalized Projection or populate a precursor engine.
32. stable roles and semantic capabilities are separate registries/fields; profiles, kinds, and vocabulary pin one exact stable-role registry ref/fingerprint pair, while versioned capability contracts bind required semantic fields to schema pointers and pass semantic validation; dependencies cite exact capability contracts and use only the frozen `exactly_one`/`at_least_one` semantics without source-order provider selection.
33. every valid resolved profile has exactly one `always`-required instance selecting the `constitutional_root` capability, and role/vocabulary/conditions/overlays cannot erase or multiply it.
34. intake source kinds and coverage results remain typed; defaults, inference, waivers, contradictions, unknowns, declarations, and approvals do not collapse into one value authority.
35. promotion re-resolves current semantic definitions and uses compare-and-write fingerprints; stale candidates or targets leave canonical truth unchanged.
36. vocabulary applies only to registered stable roles; it may change labels/aliases and declare acyclic structural absorption, but cannot mention capability IDs, rename machine identifiers/commands, absorb `constitutional_authority`, or erase authority/evidence boundaries.
37. posture input/kernel fingerprints include the exact profile ref plus `resolved_profile_fingerprint`, an exact ref/fingerprint pair for every other semantic input, and an immutable freshness-evaluation basis whenever time affects applicability, with explicit normalization inclusions/exclusions; recommendations remain advisory and only authorized compare-and-write transitions mutate canonical policy.
38. profile inheritance is single-parent replace-whole: child omission inherits, field presence replaces completely, and explicit empty/null clears; v1 has no append/key merge, tombstone, multi-parent, or invocation-time field override.
39. immutable intake, candidate, approval, and promotion records link only downstream-to-upstream; later transitions never add forward refs or change prior bytes/fingerprints.
40. every reassessment trigger maps a non-empty exact set of coverage IDs; unknown, empty, or unrelated mappings fail closed and never reopen the whole artifact implicitly.
41. hard-trigger contracts and accumulated posture rules have replayable fingerprints and typed bounded evidence; each v1 recommendation proposes exactly one global-dimension transition with causal scope metadata, its `from` matches the kernel, its notification matches policy, and its constitutional-root-only compare-and-write `replace` replays resulting authority/kernel fingerprints while stale/unauthorized/unmapped/override-target changes fail closed.
42. a Context Resolution stack has one exact definition fingerprint, one linear level order, six complete non-empty ranked domains, and adjacent complete defaults that never increase toward narrower levels; envelopes materialize all six dimensions, use at most one parent, and children can only preserve or narrow ranks/mutation authority, with valid selector overlap resolving deny and indeterminate matching refusing.
43. Projection definitions, requests, and results bind paired exact source/profile/vocabulary/definition/envelope refs/fingerprints; every selected definition belongs to the exact resolved-profile catalog; every definition declares mandatory base currentness requirements (`none`/null/empty or exact snapshot family-selector-adapter-slot closure with captured-revision basis), one exact fingerprinted metadata-only disclosure policy, one exact versioned built-in metadata-only support evaluator whose schema/pointer/derivation dependency/input/reason-order definition has a recomputable fingerprint producer and an exact input allowlist excluding source-definition/capability identity, and complete per-rule six-dimension minimum/registered-classification closure; exact profile/definition/source validation is the sole owner of source-definition and semantic-capability contract/binding compatibility, and invalid definition/capability semantics refuse before per-rule evaluation; unregistered definition classes and invalid policy/registry/evaluator state refuse before result construction; exact request values equal the bound snapshot's captured composite/slots and result observations equal those same values; every v1 source selector is `exactly_one`; deterministic reveal/derive rules are acyclic and non-executable; applicable rule evaluation compares Resolution then maps exact upstream redaction then applies fail-closed disclosure/support before payload read; one definition-ordered disclosure evaluation per applicable rule records the exact support pair plus first-precedence source-kind/schema/pointer/derivation unsupported reason or null, satisfies the exact short-circuit/nullability matrix, and closes evaluator identity/semantics into definition/evaluation/result fingerprints; every rule is included, typed-omitted, or exactly not-applicable; fixed redacted > partial > collapsed > lossless precedence applies; the requiredness/claim table fixes target absence and `not_observed`/`none`; and results have `authority_effect: none`.
44. snapshot capture policies bind exact source/comparison/drift/predecessor/redaction/retention definitions and static window rules while live revision/cursor stays capture-local; selected families equal observed-plus-excluded families; each observation records pre/captured/post revision, bound evaluation, window inputs, and payload fingerprint; top-level stable/bounded requires no exclusions and derives exactly from family classifications while any exclusion/unstable family forces diagnostic-only unstable or refusal; state versus record fingerprint inclusions/exclusions are deterministic; and predecessor links are immediate, ordered, acyclic, and boundary-compatible.
45. snapshot deltas fail closed on incompatible/unstable inputs, compare or type-exclude every selected family exactly once, bind every normalized change to stable before/after fingerprints, evaluate every exact catalog rule once, map every match bijectively to one signal, and preserve durable justification refs.
46. snapshot Projections extend rather than weaken the generic Projection contract, retain all generic result fields and complete rule accounting, never disclose beyond per-rule minimum Resolution or the exact disclosure/upstream-redaction policies, map the exact upstream disposition `original_pointer` subtree to `redacted` rather than `unavailable` without rereading hidden bytes, evaluate an action-typed retained pointer independently outside that subtree even when path segments are shared, enumerate omissions/proof effects, require fresh capture to finish before exact request construction, preserve identical request/result sources, and fingerprint exact per-family revision/disclosure checks before grounding; comprehensive capture is never comprehensive disclosure.
47. snapshot redaction remains fail-closed with fingerprinted unmatched-action `omit`, explicit deny floors for secrets, unsafe environment/secret-file/command/diff content, and deterministic overlap refusal; retention resolves an exact horizon/trigger/record-class tuple; deduplication/compaction never rewrites an immutable record or removes a referenced/held/unexpired record.
48. Resolution escalation/promotion requests and terminal dispositions are separate append-only uniquely fingerprinted records; no request self-authorizes or mutates in place, and promotion writes only new reviewed semantic memory through target-horizon validation and compare-and-write; neither path promotes a snapshot/Projection into artifact, contract, posture, or gate authority.
49. the target crate graph is acyclic: semantic owners never depend on SDK/transports/Substrate, `handbook-contracts` never depends on pipeline, no Handbook crate depends on Substrate, and `handbook-compiler` gains no new permanent owner or downstream API while retiring in HCM-4.1.
50. every ordinary SDK use case has one stable dot-separated machine operation ID/version/definition fingerprint plus exact request/result/blocker/refusal/error schema pairs; custom kind/profile/vocabulary/pipeline IDs remain request data and never generate operations or transport commands.
51. intake/candidate/approval/promotion, posture transition, and Resolution-escalation/memory-promotion lifecycles are mechanically reachable through separate typed operations that preserve immutable upstream lineage, exactly-one terminal disposition, approval authority, and compare-and-write semantics; generic typed record list/read operations make promotion/transition/semantic-memory records and every other governed pending state rediscoverable by restarted/separate actors.
52. public SDK Rust methods are typed; unbounded `Value`, unknown-field flattening, field-presence variant inference, range/latest schema fallback, and transport-owned domain branching are prohibited.
53. generic request/response schemas are closed and fingerprinted; exact negotiated API/operation/definition/schema/capability pins validate before body access and enter the request fingerprint; the response binds that exact request; outcome arrays obey the one-status truth table; bare semantic/schema/evidence/artifact refs refuse; v1 transport extensions are exactly empty.
54. `read_only`, `append_only`, and `compare_and_write` have closed idempotency/write-mechanic combinations while exact conditional write sets and realized write receipts distinguish canonical truth, semantic memory, semantic records, observation evidence, and operational state; compare-and-write atomic groups may include their required immutable semantic lineage record; promotion writes canonical artifact plus promotion record, posture transition writes constitutional-root update plus transition record without granting authority to its derived kernel, and memory-promotion disposition always writes a terminal record plus conditional semantic memory.
55. mutation keys bind repository, negotiated API/bootstrap context, operation definition, and request fingerprint; the retained-result/tombstone by same/different-fingerprint matrix is total, same-fingerprint races replay one winner, and different-fingerprint races conflict. The ordinary request uses a two-level RFC 8785/SHA-256 closure: a fixed-size `request_payload_fingerprint` covers the exact repository-root/Resolution/snapshot/body/extensions remainder, and `request_fingerprint` covers the descriptor-pinned generic request schema, operation, definition, expected response schema, negotiated API/bootstrap, repository identity, scoped idempotency-key fingerprint (null for read-only), and payload fingerprint. The original-result fingerprint is RFC 8785/SHA-256 over the exact committed semantic response closure and realized receipts while excluding correlation, idempotency, and outer response fingerprint; retained replay and tombstone compaction preserve that identity. A descriptor-pinned bridge recovery hold's complete tagged/schema-bound open frame carries the non-secret key-scope preimage, generic-request/response-schema bindings, request-payload/request fingerprints, and one descriptor/operation-schema-bounded raw key only over private stdin so the handler can recompute every nested fingerprint before persisting a redacted identity; it is fsynced before transmission/spawn, remains non-authoritative/outside domain writes and receipts, and blocks only exact-pair result compaction while unresolved. Ordinary-request admission independently recomputes both request fingerprints and the exact key tuple before any hold association or mutation; a bridge path proceeds only when that tuple matches the acknowledged open. One atomic active-key index permits at most one unresolved hold per scope/key pair, binds its immutable request identity, and distinguishes same-request `key_held` from different-request conflict while allowing a new hold only after release. Control-open admission atomically includes the domain key: absent and retained-same may open, reserved keys create no hold, and retained-different or any tombstoned key returns a deterministic state-level refusal before control mutation; retained-same open acquires the result-compaction guard in that same transaction. Before first establishment, an unresolved hold participates in the same atomic ordinary-request admission: the exact held request may establish/replay one winner, while a different request on that scope/key receives only the typed `idempotency.recovery_hold_conflict` refusal before mutation; an adapter-local pre-spawn tuple mismatch remains a bounded host failure with no Handbook response. Release binds the exact `open_fingerprint` and only follows a fsynced durable-capture or discriminated child-not-spawned/operation-aborted proof plus complete release frame. Private-control byte/schema admission precedes state lookup; the total schema-valid absent/unresolved/released event/output matrix fixes every acknowledgment field, replays identical unresolved open and released release, rejects conflicting/unknown events and transplantation, rejects every open after release, and never lets released state reactivate or re-block compaction. Each released tombstone is fixed-size and non-expiring; aggregate storage is intentionally monotonic until irreversible namespace retirement, and exhaustion fails closed without eviction. Pre-definition parseable selection failures use only the non-null-fingerprint descriptor-bound admission envelope with null trusted operation fields; its reason-discriminated variants distinguish missing, malformed, unknown/incompatible, client-pin-missing, registry-missing, mismatch, and stale state; its exact four-leaf requested-API DTO and fixed path order classify absent/null/partial/malformed/mismatched input; attempted-value digests bind one named raw JSON value; the total operation/API/definition precedence emits only winning-field evidence; the request-object digest deletes exactly top-level correlation `request_id` and client-supplied derived `request_fingerprint` before canonicalization but preserves every other compound-invalid field; admission correlation safely echoes only allowlisted IDs and otherwise emits deterministic null; exact Problem details include stage/reason and mirror that evidence under an injective code/rule/details-schema mapping; and the manifest closes over descriptor-owned schemas only. Duplicate JSON member names at any nesting level, like other pre-object normalization failures, yield no Handbook response. After exact definition selection, request identity has a total normalized-valid versus request-validation-refusal preimage: the refusal branch binds the descriptor/definition/API context, correlation/client-metadata/client-derived-fingerprint-excluded request-object digest, and one closed missing/null/malformed/mismatch reason/evidence variant without requiring the invalid response-schema/repository/key inputs; it is non-null response correlation only and can never reach ledger lookup, hold, reservation, replay, or mutation. Accepted read-only responses use only not-applicable, accepted mutating pre-commit failures only not-established, and committed mutations only established. Memory-promotion disposition alone establishes before its authorized domain compare so a new-key stale basis writes/replays the required terminal stale disposition instead of leaving the request pending; the crash-recoverable idempotency journal is explicitly non-authoritative protocol-control metadata outside governed write sets/receipts.
56. each API major has a distinct immutable mechanically derived bootstrap-descriptor ref; the descriptor or exact compile-time Rust types are the only discovery roots; snapshot-bound paged operation/profile/schema/governed-record catalogs, exact schema/record/vocabulary/Resolution-stack/Projection-definition reads, and deterministic applicable-snapshot selection provide complete restart-safe machine inputs without inferred defaults or repository/prose reads.
57. transport JSON Schema/operation/API identities use explicit full SemVer domains with mechanically derived refs; the bootstrap descriptor pins the generic ordinary-request schema and its identity enters every accepted request fingerprint, while frozen two-component domain-record routing tags are non-schema identifiers that never alias a public schema ref; deprecation names an exact replacement operation and migration artifact, and stale/unsupported/cross-major/tampered state fails closed.
58. shared problem/problem-binding, diagnostic, next-action, versioned artifact, closed locator, source, omission, provenance, schema-manifest, catalog-page, write-receipt, and idempotency DTOs have exact fields, unique instance identity, total category/nullability/stage partitions, enums, bounds, sensitivity/no-follow rules, duplicate policy, canonical ordering, and independently recomputable non-circular instance fingerprints.
59. posture recommendation evaluation is read-only and total over recommendation/no-recommendation; only the separate append operation persists an immutable recommendation.
60. direct SDK, CLI JSON, and Tauri paths preserve the same DTO semantics, negotiated API/request binding, status/data/outcome/provenance/schema/idempotency/write-receipt fields, operation fingerprints, and safe string-or-null correlation matrix; after exact definition acceptance invalid correlation is an ordinary request-validation refusal with invariant semantic/Problem fingerprints. The `established` ordinary-response DTO exposes only the scoped `idempotency_key_fingerprint`, never a raw key; direct SDK, CLI JSON, Tauri, first-execution, replay, tombstone, diagnostics, capture, and ledger fixtures scan every serialized or durable surface for raw-key absence. Direct SDK, CLI JSON, and Tauri postselection fixtures cover missing/null/wrong-type/malformed/mismatched expected-response binding and repository identity, missing/null/wrong-type/malformed definition-pinned mutation keys, unsupported/wrong-type extensions, and unrelated generic failures with deterministic non-null request fingerprints, injective exact Problems, correct not-applicable/not-established variants, inert client-supplied fingerprints, and no ledger access or raw-key disclosure. A schema-valid bounded idempotency key has no ordinary mismatch comparator and is accepted as the request value. Human output renders from typed results and no transport creates a second decision path; a post-commit serialization/delivery failure emits no false ordinary error and recovers the committed result/receipts only through established-key replay.
61. JSON stdout is one atomic schema-valid response followed by zero or one LF for every selected nontrivial JSON operation; the bridge accepts the identical framing and rejects all other trailing bytes; stderr is non-authoritative; exit/status mappings agree; domain-negative results inside successful data do not become transport errors.
62. the Tauri adapter uses SDK calls without a normal-operation CLI subprocess or frontend semantic authority; adapter errors cannot replace expected typed blocked/refused outcomes with prose, pre-commit versus post-commit failures remain distinct, and post-commit transport uncertainty cannot fabricate an `error` domain result.
63. the transitional Substrate bridge pins the binary/bootstrap/operation/schema/capability fingerprints, bounds process resources, parses exactly one validated JSON response, never parses stderr/human text, transports sensitive bodies only through bounded stdin or trusted exact refs, proves no argv/env/stderr/process/temp leakage, and remains isolated under `BR-SUB-CLI-01`. Its only non-ordinary invocation is descriptor-pinned `handbook --bridge-control-json`, whose closed stdin/stdout/exit protocol reaches only the SDK-private recovery-hold ledger interface and cannot make a semantic decision or become a public operation/SDK/Tauri surface. The tagged/schema-bound open carries its complete key-scope preimage, generic request/response schema bindings, bounded request-payload/request fingerprints that include the scoped raw-key digest, and one explicitly bounded raw key only through private stdin; the descriptor pins `bridge_recovery_record_max_bytes` across the variable-length initial and `release_pending` private record variants, pre-open worst-case proof prevents later overflow, exact-ceiling records pass, and one-byte-over records fail before open; the handler recomputes all identities, rejects disagreement among the top-level, key-scope, descriptor/catalog-selected operation-definition, generic-request/response-schema, and outer request bindings even when every enclosing fingerprint was recomputed, and persists only the redacted open. Ordinary request admission then recomputes the same normalized-valid generic-request-schema/payload/request/key tuple from the actual accepted request and must match the acknowledged hold before bridge mutation; the request-validation-refusal fingerprint variant is never eligible to open or associate a hold. The bridge performs normalized-valid preflight before control/open, so postselection-invalid mutating input is an adapter-local pre-spawn failure with no Handbook response, hold, ledger lookup, or mutation; direct SDK/CLI/Tauri ordinary refusal fixtures do not claim bridge response equivalence for that case. Concurrent bridge-open versus direct SDK, CLI JSON, and Tauri fixtures prove that the exact held request establishes/replays one result, a different held request receives the typed ordinary recovery-hold-conflict response before mutation, and an adapter-local pre-spawn mismatch emits no Handbook response rather than masquerading as that refusal. Pre-state byte/schema/fingerprint failure is disjoint from state-level refusal, and a complete state/event/output matrix fixes every acknowledgment field; an identical lost-open or lost-release acknowledgment retry reproduces the original RFC 8785-plus-LF bytes and `control_ack_fingerprint` without a replay-only code. Its enforced order is fsync complete private request/open frame, send/retry open, execute/recover exact request, fsync the closed terminal outcome/evidence and exact open-bound release frame as `release_pending`, send/retry only that persisted release, fsync released acknowledgment, then delete private state. The exact non-authoritative hold opens before mutating spawn; any commit-uncertain timeout/truncation/invalid-output path preserves the exact original envelope/key/fingerprint/hold and blocks compaction until same-key established replay is durably captured with the original-result identity and receipts, while the discriminated child-not-spawned or operation-aborted proof is equally durable before release. Either terminal branch persists the complete release before transmission; release then creates durable idempotent fixed-size control tombstone memory and permits ordinary result compaction. Aggregate terminal memory is deliberately monotonic for an active repository namespace and storage exhaustion fails closed. Restart resumes the recorded adapter phase and can neither reconstruct a release from ambient state nor reopen/re-execute after `release_pending`. It never reopens a released hold, fabricates an error, chooses a new recovery key, or duplicates a write; private-channel framing/pin/exit failures, postselection-invalid mutating preflight with no control/spawn/response, initial/release-pending private recovery records at the exact serialized ceiling and one byte over, request transplantation independently across operation/definition/API/bootstrap/repository/raw-key with all enclosing hashes recomputed, exact-pair association/compaction, concurrent distinct-hold same-key same/different-request admission, bridge-open versus direct-transport exact/different-request admission, control-open admission against absent/reserved-same/reserved-different/retained-same/retained-different/tombstoned-same/tombstoned-different domain states, generic-request-schema substitution, key-scope/raw-key/release-transplantation tampering, pre-spawn/aborted-journal proof variants, the full acknowledgment matrix, lost-initial-open/release-ack byte-and-fingerprint replay, pre-send/post-send/pre-spawn crash windows, every boundary around release persistence/send/ledger transition/acknowledgment fsync, restart, delayed recovery beyond nominal retention, repeated lost delivery, durable capture, lost release acknowledgment, storage exhaustion/namespace retirement, released-tombstone retention, and post-release compaction are failure-injection proofs.
64. bridge proof, Tauri proof, packaged-crate proof, registry proof, and direct Substrate adoption are separate subjects/classifications; none substitutes for another.
65. each changed crate is proved, packaged, isolated, published, and checksum-resolved from crates.io before the next dependent DAG node is packaged; a dependent is never proved against a workspace/path build and later treated as registry-equivalent.
66. downstream-intended Rust APIs are complete only after exact crates.io publication, registry-only external resolution, a current-tip Substrate worktree with a named real seam, positive/negative/no-fallback proof, and preserved review evidence; public symbols, path builds, manifest pins, or toy consumers do not count.
67. HCM-0.5 may append contract/dock ordinary operations after semantic freeze but cannot change the HCM-0.4 owner graph, typed DTO/outcome rules, transport parity, bridge boundary, or published-proof plan.
68. contract definitions use exact `contract_id@full-SemVer` refs and RFC 8785/SHA-256 fingerprints; same-ref/different-bytes conflicts, every semantic change requires the minimum closed compatibility-table version bump, and ranges/latest/automatic compatible substitution refuse.
69. contract definition records remain immutable in every lifecycle state and fingerprint-bind the authenticated definition-author/admission-authority identity; each definition derives one exact timestamp-free draft-genesis lifecycle fingerprint, the first transition alone uses null prior transition plus that fingerprint, every transition/resulting-lifecycle fingerprint has an exact non-circular RFC 8785 preimage and derivation order, every later transition requires the exact non-null immediate prior ref/resulting fingerprint, lifecycle transitions are append-only and limited to the closed adjacency/authority table, non-author draft transition, author substitution, self-lock, stale/wrong-genesis/fake-prior/null-later transition refuses, `closed` is terminal, and evaluation/verdict/gate outcomes never become lifecycle states.
70. claim applicability is declarative and matcher-bound, evaluates before evidence, and distinguishes proven false `not_applicable` from indeterminate `blocked`; evidence requirements are non-empty all-of clauses with exact kind/case/cardinality/stability/freshness/minimum-six-dimension-Resolution bindings and no cross-kind substitution or dock-selected applicability.
71. canonical evidence binds exact parent evaluation-run/request-ID/request-fingerprint/dock-run plus contract/claim/subject/case/dock implementation/execution/source/schema/freshness/Resolution identities; result, candidate, execution record, and evidence equality-check that closure, while one evaluation may own multiple distinct runs; effective Resolution is the dimension-by-dimension minimum of request, capability ceiling, and actual grant/observation; per-tuple cardinality and repeated-observation consistency use the fixed `blocked` > `flaky` > `fail`/`warning` > `not_observed` > `pass` precedence.
72. every claim appears in exactly one closed verdict; `warning` is advisory-only, `not_applicable` requires proven false applicability, malformed/stale/refused/failed/insufficient-Resolution evidence cannot pass, and validators/runners/transports cannot emit canonical verdict truth.
73. gates completely partition claims and decide only `passed` or `blocked`; hard/required non-pass, missing accounting, and stale/invalid bindings outrank score, while local closeout and parent promotion are independently fingerprinted policy decisions that default false on indeterminate state.
74. dock identity binds the exact manifest, content-addressed implementation bundle, normalized no-extra/no-missing/no-symlink file manifest, entrypoint digest, closed typed native-or-bundled-interpreter launch vector/fingerprint, and canonical runtime/dependency-closure descriptor containing exact member roles, dependency edges, bundle-only resolution policy, platform ABI, ref, and fingerprint; the host traverses/recomputes every executable/interpreter/application/library/package dependency before body access/spawn, and its allowlist mapping confers no semantic authority.
75. v1 process docks accept one bounded UTF-8 JSON request and produce one bounded JSON result, each optionally followed by one LF; duplicate members, trailing bytes, extra documents, prose/ANSI, invalid UTF-8/JSON, schema/fingerprint mismatch, and stderr-only results are protocol errors with no candidates.
76. v1 dock execution is default-deny and network-denied with safe staged refs, sanitized environment, exact launch-vector direct spawn, bounded resources/output, host-monotonic timeout, idempotent cancellation, process-tree termination, typed refusal, and ordered first-match crash/protocol/output/isolation/cleanup outcome behavior; no non-completed outcome yields partial evidence.
77. a dock emits only untrusted candidates; each candidate is independently admitted by `handbook-contracts` into exactly one immutable evidence record or rejected with no write, while the executor appends only one operational execution record per admitted run and never gains contract/evidence/verdict/gate authority.
78. `handbook.dock.json-schema@1.0.0` remains the bounded future first proof target for one exact offline Draft 2020-12 schema/ref closure; remote refs, executable hooks, ambient schemas, unsupported dialects, and fingerprint mismatch refuse, and the design selection does not prove `PG-DOCK-01`.
79. HCM-0.5 ordinary operations preserve frozen HCM-0.4 owners, DTOs, transports, idempotency, write-set/receipt semantics, and publication rules; `dock.run` writes exactly one operational record for an admitted run, candidate admission is a separate exactly-one evidence append, and verdict/gate evaluation remains read-only until later approved persistence authority exists.
80. the HCM-0.5 documentation freeze keeps canonical `05` monolithic, adds no catalog leaves/schema/runtime/Rust/Cargo/CLI/Tauri/Substrate/HCM-0.6 work, and leaves `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` open.

## Slice closeout evidence record

When a slice closes, update only the affected rows and cite:

- commit/tree state;
- exact source boundary;
- exact tests and commands;
- real-path proof when required;
- negative/fail-closed proof;
- published/downstream evidence when required;
- independent review result;
- handoff record ID.

Do not replace evidence refs with “all tests passed.”

## HCM-0.2 semantic-freeze proof gate

HCM-0.2 is a documentation/design freeze. Its closeout does not promote any runtime implementation gate in the open-program table. At its completed closeout, the following conditions had to hold; later approved slices may resolve the items explicitly left open here without rewriting this historical gate:

- `02-semantic-model.md` identifies the exact frozen HCM-0.2 boundary while leaving HCM-0.3+ contracts and the HCM-0.6 shipped-default decision unresolved;
- the HCM-0.2 sections in `05-contracts-schemas-and-gates.md` define every in-scope authored or derived field with its owner/authority, default or omission behavior, required validation, and explicit non-goal;
- profile layering is explicit single-parent replace-whole with omitted-versus-empty/null semantics, no ambient/invocation-time field overrides, per-field source decisions, one exact stable-role registry ref/fingerprint closure, fail-closed conflicts, and a complete normalized `ResolvedInstanceProfile` fingerprint;
- uniform exact-definition identity derives every ref from declared identity/version fields and recomputes RFC 8785/SHA-256 definition closure fingerprints, while the local schema registry adds Draft 2020-12 meta-validation, safe repo-relative/no-follow resolution, transitive local-ref closure, and fail-closed conflict/remote/hook handling;
- kind definitions remain reusable schema/behavior authority, while instance descriptors alone own repository ID/path/label/requiredness/dependency/selection state;
- exact-version refs, concrete resolved paths, exact capability-dependency contracts, frozen `exactly_one`/`at_least_one` provider semantics, and renderer-versus-Projection separation are unambiguous;
- stable-role refs and semantic-capability refs are separate throughout kind, instance, profile, vocabulary, and dependency contracts; the stable-role registry is exact and replayable while versioned capability contracts/schema bindings, not roles/labels/filenames, determine capability conformance;
- every valid resolved profile contains exactly one `always`-required instance selecting `constitutional_root`, with policy, decision/exception authority, exact posture-dimension/red-line bindings, and reassessment bindings whose mapped coverage path equals or is an ancestor of each bound pointer;
- intake definition with non-empty v1 candidate-field mappings, typed source/result provenance, immutable downstream lineage, candidate, approval, compare-and-write promotion, Charter authority, and non-empty trigger-to-coverage reassessment mappings are explicit and fail closed;
- Charter intake definition/result/candidate coverage sets are identical and retain project shape, delivery constraints, all five live default-delivery implications, operational reality, risk, posture, governance, debt, and decision-record domains with exact candidate-schema paths;
- vocabulary defaulting, lexical ambiguity, stable-role-only structural absorption, adapter-loss refusal, and stable machine-identifier/capability boundaries are explicit;
- project-posture kernel typed ref/fingerprint inputs bind `resolved_profile_fingerprint`, explicit freshness basis, versioned hard triggers, fingerprinted bounded accumulated-signal rules/policies, exactly-one global-dimension recommendations with causal scope metadata, constitutional-root-only typed bound transition changes, authorized reassessment, resulting-kernel fingerprint, hysteresis, normalization, and non-second-authority rules are explicit;
- examples do not select the shipped artifact kinds/default instances/requiredness reserved for HCM-0.6;
- live Rust remains accurately classified as fixed-layout/Markdown-authority precursors, and all affected runtime program gates stay open;
- no Rust files change;
- control-pack links, fixed-renderer-versus-Projection terminology assertions, archive boundary, handoff validation/self-tests, scoped diff, and `git diff --check` pass;
- a fresh isolated built-in `default` reviewer reports no unresolved actionable finding over the complete HCM-0.2 subject and proof wall;
- the primary reviewed commit and second mechanical v1.2 parent-owned handoff/ledger closeout both validate.

## HCM-0.3 Resolution/Snapshot/Projection freeze proof gate

HCM-0.3 is a documentation/design freeze. It does not implement the HCM-3.2–HCM-3.5 kernels or promote any `PG-RES`, `PG-PROJ`, `PG-SNAP`, `PG-HANDOFF-02`, profile-runtime, or consumer-runtime gate. Before HCM-0.3 may close:

- `02-semantic-model.md` freezes the ordered-stack, six-dimension comparison, fully materialized envelope, single-parent constraint, typed mutation, escalation, memory-promotion, deterministic reveal/derive, omission/proof, snapshot consistency/fingerprint, delta/drift, redaction/retention, and specialized snapshot-Projection semantics;
- exact HCM-0.3 definitions/records in `05-contracts-schemas-and-gates.md` give every in-scope field an owner/authority, default or omission rule, required validation, and explicit non-goal;
- profile `context_resolution_ref` and `projection_catalog_refs` plus kind/instance `projection_definition_refs` resolve exact definition/source/schema fingerprints; every request definition belongs to its resolved profile catalog while catalog truth remains separate from fixed renderer refs and creates no Phase-2 Projection engine;
- the stack is one versioned linear definition with six complete rank-zero-to-broader domains and complete adjacent broad-to-narrow non-increasing level defaults; level names are configurable but cannot reverse privilege meaning or become one aggregate score;
- every envelope pins one exact resolved profile/stack, materializes all six dimensions, has at most one exact parent, narrows dimension ranks and mutation authority monotonically, resolves valid selector overlap with deny while refusing indeterminate matching, and fingerprints the complete resolved constraint closure;
- escalation/promotion requests identify exact missing condition/source evidence, requested authority, and compare-and-write basis; separate append-only terminal dispositions preserve prior bytes, enforce exactly-one disposition, prohibit self-approval, and require target-horizon validation plus a new semantic-memory result only for applied promotion;
- Projection definitions are exact profile-catalog members, declarative, acyclic, target-schema bound, declare mandatory base currentness requirements with exact captured-revision family/selector/adapter/slot closure, bind one exact fingerprinted metadata-only disclosure policy and one exact versioned/fingerprinted built-in support-evaluator definition with an exact source-kind/schema/pointer/derivation input/reason allowlist, validate source-definition and semantic-capability contracts/bindings exclusively in exact profile/definition/source validation before per-rule evaluation, give every rule complete six-dimension minimum Resolution plus registered classification, use exactly-one v1 selectors, and are limited to `reveal`/`derive` through allowlisted deterministic derivations; prompts, models, commands, executable hooks, remote code, extension-supplied required behavior, content-sniffing disclosure/support, and transport-owned domain rules fail closed;
- Projection requests/results use paired exact provenance refs/fingerprints; reject unregistered rule classifications or stale/missing policy/registry/support-evaluator state before result construction; deterministically compare rule minimum Resolution, map upstream redaction, evaluate fail-closed disclosure/support before protected payload access; record one definition-ordered disclosure evaluation per applicable rule under the complete outcome/nullability/fingerprint matrix with the exact support pair; partition every definition rule exactly once across included/typed-omitted/operation-mismatch-not-applicable; apply the complete requiredness/claim/output/proof table plus deterministic redacted-over-partial-over-collapsed-over-lossless precedence; bind output/result fingerprints; and set `authority_effect: none`;
- capture policies bind allowed memory horizons, exact source adapters, multi-source slot/composite rules, static bounded-window rules, comparison/drift/predecessor catalogs, an exact bounded-skew rule, consistency behavior, redaction, retention, and policy fingerprint while live source revisions/cursors remain capture-local; no invocation widens them;
- snapshots bind exact profile/envelope/policy/source refs, family-composite and per-slot pre/captured/post revisions, the policy-selected bound evaluation, window inputs, and payload fingerprints; type every selected family as observed or excluded; detect active-plan-only drift; derive top-level stable/bounded only from a complete non-excluded family set and force any unstable family/exclusion to diagnostic-only unstable or refusal; enforce unique strictly increasing boundary sequences plus exact acyclic immediate predecessor applicability; exclude boundary identity from state fingerprints while retaining it in record fingerprints;
- deltas require exact compatible stable/bounded snapshot inputs, compare or type-exclude every family, normalize before/after changes, bind an exact endpoint-selected/comparison-admitted drift catalog, account every catalog rule exactly once with a matched-signal bijection, and refuse incompatible/unstable/uncataloged/incompletely-evaluated inputs instead of returning empty/green truth;
- snapshot grounding extends the generic capitalized Projection contract without renaming or dropping `sources`, `resolution_envelope`, disclosure, or rule-accounting fields, copies exact request values from the bound snapshot's captured composite/per-slot state, binds result observations to those same values, covers every family behind unfiltered delta signals, maps exact upstream disposition original-pointer coverage to `redacted` without rereading or misclassifying it unavailable, independently evaluates the action-typed retained pointer outside that subtree, requires any fresh capture before request construction, preserves identical exact request/result snapshot/delta sources, completely accounts included/omitted/not-applicable rules and lossiness, and fingerprints currentness/disclosure evidence before action;
- redaction remains fail-closed with fingerprinted unmatched-action `omit`, explicit known sensitive-surface floors, identical-action overlap, `omit` precedence, refusal for distinct non-omit overlap, immutable dispositions, and non-overridable secret/environment/command/diff floors; retention covers the complete allowed-horizon/trigger/record-class cross-product; content-addressed payload deduplication and reviewed compaction preserve immutable record identity and references/holds/floors;
- shipped Resolution level names/default product policy and the HCM-0.6 artifact default set are not selected by illustrative examples;
- live Rust remains accurately classified as byte-budget/work-level/capture/fingerprint precursors only; all affected runtime/open-program gates remain open and no Rust files change;
- targeted terminology/contract assertions, control-pack links, archive boundary, handoff validation/self-tests, scoped diff, and `git diff --check` pass;
- a fresh isolated built-in `default` reviewer reports no unresolved actionable finding over the complete HCM-0.3 subject and proof wall;
- the primary reviewed commit and second mechanical v1.2 parent-owned handoff/ledger closeout both validate.

## HCM-0.4 ownership/SDK/transport freeze proof gate

HCM-0.4 is a documentation/design freeze. It does not create crates, implement SDK/CLI/Tauri/Substrate behavior, publish packages, select shipped artifact defaults, freeze HCM-0.5 contract/dock semantics, or promote `PG-SDK-01`, `PG-JSON-01`, `PG-TAURI-01`, `PG-SUB-CLI-01`, `PG-PUBLISH-01`, or `PG-SUB-RUST-01`. At its completed closeout, the following conditions had to hold; later approved HCM-0.5/HCM-0.6 decisions resolve their reserved design questions without changing this historical gate:

- `01-target-architecture.md` freezes the semantic-owner/SDK/transport/Substrate split, `handbook-contracts` owner, acyclic dependency direction, `handbook-compiler` retirement, and four-tier integration ladder;
- `02-semantic-model.md` requires adapters to preserve exact operation/schema/semantic/Resolution/provenance/omission truth and separates the transitional bridge from permanent Rust adoption;
- `03-seam-crosswalk.md` accurately records that the SDK/Tauri/bridge are absent, current CLI JSON is partial, compiler composition remains, prior published consumption proves only its exact spike, and the currently inspected Substrate checkout has manifest pins without a live Handbook API call;
- `04-phase-slice-map.md` makes HCM-0.2/HCM-0.3/HCM-0.8 the proven inputs while keeping HCM-0.5 contract/dock operations and HCM-0.6 shipped defaults unresolved;
- `05-contracts-schemas-and-gates.md` gives every target crate/surface one owner, allowed dependency direction, forbidden authority, and explicit compiler transition with no cycle or SDK back-edge;
- the SDK ordinary-use-case inventory is complete for approved HCM-0.2/HCM-0.3/repository/flow/pipeline capabilities, including candidate/approval persistence, separate posture evaluation/persistence, separate escalation/memory-promotion request/disposition operations, generic typed governed-record rediscovery, deterministic applicable-snapshot selection, and exact vocabulary/Resolution-stack/Projection-definition reads; it is data-parameterized rather than command-generated and explicitly defers contract/dock operation IDs until HCM-0.5;
- every operation definition binds exact ID/version/owner, request/result/problem schemas and fingerprints, one legal mutability/idempotency/write-mechanic combination plus an exact conditional write set whose realized writes appear as response receipts, capability pins, transport targets, durable result/tombstone retention, exact deprecation replacement/migration bindings, and a recomputable definition fingerprint;
- transport requests bind exact negotiated API/bootstrap plus operation/definition/response-schema context before body access, make cwd discovery explicit, carry operation-owned mutation preconditions, require exactly-empty v1 extensions, and define deterministic fingerprint inclusions/exclusions;
- transport responses bind the exact request fingerprint and enforce the complete `ok`/`blocked`/`refused`/`error` data/outcome truth table, exact ref/fingerprint bindings, a total Problem category/nullability matrix, exact write receipts, closed bounded problem/diagnostic/next-action/versioned-artifact/tagged-locator/source/omission/provenance/schema/catalog/idempotency DTOs, no partial success serialization, and a recomputable response fingerprint;
- Rust DTO/generated Draft 2020-12 JSON Schema parity, closed/discriminated types, explicit transport-schema-versus-record-routing version domains, compatibility negotiation, a distinct-ref-per-major non-circular bootstrap descriptor, snapshot-bound paged catalogs, and exact schema/vocabulary/Resolution/Projection discovery are explicit and prohibit unbounded public JSON values, inferred defaults, repository-file discovery, or prose/help discovery;
- CLI JSON has exact one-document stdout, stderr, ANSI/prompt, atomicity, human-rendering, and exit/status rules without collapsing domain-negative data into process failure;
- Tauri maps commands to the same SDK operations/DTOs without normal-operation CLI subprocesses, frontend authority, or prose-only expected outcomes, and has a cross-transport equivalence proof;
- the Tier-2 Substrate bridge pins binary/checksum/bootstrap/definition/schema/capability state, bounds process resources, validates exactly one response, keeps sensitive request bodies out of argv/environment/stderr/process titles/ambient temp files, proves redaction and leakage resistance, fails closed without fabricated domain results, proves only `PG-SUB-CLI-01`, and carries deletion gate `BR-SUB-CLI-01`;
- the Tier-4 boundary prefers exact published SDK APIs, preserves Substrate orchestration/wording, prohibits CLI/path/patch fallback in the proved seam, and closes the bridge only after exact publication/direct-adoption proof;
- the published proof plan proves, packages, isolates, publishes, and checksum-resolves each changed crate from crates.io before packaging the next dependent DAG node, then requires registry-only external consumption, current-tip Substrate real-seam adoption, negative/no-fallback proof, and durable evidence; prior proofs remain version/API/seam bounded;
- all new YAML/JSON fenced examples parse, targeted field/inventory/outcome/owner/dependency/bridge/proof assertions pass, relative links and archive boundaries remain clean, and there are no absolute machine paths in changed durable content;
- live Rust and every runtime/open-program gate remain accurately unpromoted; no Rust, Cargo, CLI, Tauri, or Substrate files change;
- handoff validation/self-tests, scoped diff, `git diff --check`, and repository-required staged change detection pass;
- a fresh isolated built-in `default` reviewer reports no unresolved actionable finding over the complete HCM-0.4 subject and proof wall;
- the primary reviewed commit and second mechanical v1.2 parent-owned handoff/ledger closeout both validate.

## HCM-0.5 contract-membrane/dock documentation-freeze proof gate

HCM-0.5 is a documentation/design freeze. It does not implement `handbook-contracts`, an SDK/CLI/Tauri/Substrate operation, a schema, a dock manifest/bundle/runner, or a validator and cannot promote `PG-CONTRACT-01`, `PG-DOCK-01`, or `PG-GATE-01`. Before the parent may close HCM-0.5:

- the exact review-clean HCM-0.5 packet commit/path/fingerprint and selected v1.2 entry handoff/ledger parity validate before canonical edits;
- canonical `00`-`06` are the only semantic files changed, and the final intended `00-README.md` status bytes are present before proof/review without claiming runtime proof or gate promotion;
- `01-target-architecture.md` preserves `handbook-contracts` as semantic owner, a separable process executor as operational owner, validator-as-witness boundaries, process-first/future-Rust semantic parity, implementation-bundle/runtime-closure identity, default-deny isolation, unconditional v1 network denial, and explicit runtime non-goals;
- `02-semantic-model.md` adds contract/claim/applicability/evidence/evaluation/gate semantics without modifying the HCM-0.3 Context Resolution/Snapshot/Projection authority and applies all six Resolution dimensions independently;
- only the Contract membrane, External docks, and directly coupled Contract-catalog decomposition/monolith crosswalk rows plus directly coupled high-risk notes change in `03-seam-crosswalk.md`; both runtime seam rows remain `TargetOnly`, the catalog row only removes volatile line-count wording, and the HCM-0.9-abandoned monolithic-catalog boundary remains intact;
- `04-phase-slice-map.md` closes the HCM-0.5 dependency/output/exit/non-goal design and aligns only the directly coupled Phase-5 implementation wording; HCM-0.6 is not started;
- canonical `05-contracts-schemas-and-gates.md` remains one monolith and replaces the preliminary contract/dock material with exact field/rule/state/protocol tables while preserving HCM-0.2/HCM-0.3/HCM-0.4 semantics except for the packet-approved additive ordinary operation rows;
- contract identity derives exactly from stable ID/full SemVer, same-ref/different-bytes conflicts, exact pins never auto-substitute, and the patch/minor/major compatibility table is mechanically exhaustive for changed semantics;
- lifecycle prose, state list, diagram, and adjacency/authority table agree exactly; definitions and transitions are immutable; definition identity binds the authenticated author/admission authority; draft-author equality and lock-author distinctness are enforced; draft genesis plus transition and resulting-lifecycle fingerprints have exact non-circular preimages and derivation order; null prior is legal only for the first transition; and non-author/author-substituted/wrong-genesis/fake-first/null-later plus every unlisted edge, stale basis, self-lock, skip, rollback, reactivation, `active -> closed`, and evaluation-state conflation refuses;
- applicability and the complete gate-effect x verdict matrix are mechanically exhaustive; every claim is partitioned exactly once; hard/required `warning` and advisory `fail` refuse as evaluator defects; hard/required non-pass, required not-observed, incomplete accounting, and stale bindings outrank score;
- all-of per-kind evidence requirements, exact case/cardinality variants, duplicate-ref handling, repeated consistency, freshness, provenance, subject/case/run/source bindings, invalid score weights, and dimension-by-dimension Resolution qualification are mechanically asserted under the fixed precedence;
- evaluation, manifest/request/result/execution/evidence/verdict/gate records bind and current-head-revalidate the exact active transition/resulting lifecycle fingerprint plus its immediately prior independent-lock transition/fingerprint, as well as the exact manifest plus implementation bundle, normalized bundle manifest, entrypoint digest, closed typed launch vector/fingerprint, canonical runtime-closure descriptor/ref/fingerprint, schema/protocol, parent evaluation-run/request-ID/request-fingerprint/dock-run, contract/claim/subject/case, Resolution, grant, source, and fingerprint identities; bare-lock, later deprecation/closure, stale lifecycle head, and lock/activation substitution refuse or block; every nested candidate binds a non-empty no-duplicate claim-ID subset of the request and result-observed partition and is equality-checked during completed-result validation, so result or candidate transplantation across any lifecycle/evaluation/request/run/claim binding is priority-5 `protocol_error`, appends exactly one operational record, and exposes no candidates/evidence;
- pre-spawn substitution/overclaim/network/ref mismatch refuses before body access or spawn; native executable and bundled interpreter/application/argv path, digest, kind, order, addition, removal, and substitution fixtures refuse; runtime-closure member add/remove/substitute/role/mode/digest, dependency-edge/order/kind/provider, resolution-root/policy, platform-ABI, stale-fingerprint, and deterministic-reproduction fixtures are exact; request limits intersect rather than widen manifest/policy ceilings; shell/PATH/shebang/ambient interpreter/package-manager/dynamic dependency resolution is forbidden;
- one-document stdin/stdout framing, duplicate-key rejection, typed completed/refused/cancelled results, ordered first-match total host outcome mapping, stderr diagnostics-only, safe artifacts, output quotas, host-monotonic timeout, idempotent cancellation, process-tree cleanup, and no-partial-evidence behavior are mechanically covered; an outcome-discriminated execution record binds expected request/selection identity and is schema-valid for `not_created`/`created` process plus `absent`/`invalid`/`valid` result observations, including spawn failure, malformed output, and identity transplantation; valid completion atomically retains fingerprinted normalized request and accepted-result admission bases plus the bounded candidate bundle, and after the original request/full result/process/workspace/caller state is discarded the original retained candidate remains admissible while requested-but-unobserved claim, changed case/Resolution, reordered/substituted claim, or post-hoc nonmember rejects; the fact cross-product proves cleanup uncertainty, deadline, host cancellation, exit/signal, framing/schema/output, refused/completed result, unsolicited cancellation, and catch-all precedence map every admitted vector to exactly one outcome;
- the selected `handbook.dock.json-schema@1.0.0` target is justified from live local Draft 2020-12 validator evidence, remains bounded to one exact offline schema/ref closure, refuses remote/executable/ambient/unsupported/tampered inputs, and is not described as implemented or runtime-proven;
- every appended `contract.*`/`dock.*` operation has one exact owner/mutability/idempotency/authority/write-set binding conforming to HCM-0.4; the combined base-plus-HCM-0.5 inventory classifies every non-read-only operation exactly once with condition restricted mechanically to `always` or one declared exact `data.*` result discriminant plus exact cardinality/atomic-group/receipt behavior; admitted `dock.run` and valid `contract.evidence.append` each produce exactly one separate receipt, while pre-admission, zero-candidate, invalid-candidate, refused, failed, timed-out, cancelled, and protocol-error cases follow their exact no-write/operational-only matrix;
- HCM-0.2/HCM-0.3/HCM-0.4 frozen sections regress cleanly, the canonical-`05` no-leaf rule holds, historical handoffs/dispatches remain immutable, and no absolute machine path enters changed durable content;
- every new JSON/YAML fenced example parses with duplicate-key rejection where applicable; Markdown relative links, anchors, and fences, archive boundary/self-test, and all three handoff validator modes pass;
- `git diff --check`, exact scoped diff/path inspection, no-Rust/Cargo/runtime/schema/HCM-0.6 assertions, and repository-required GitNexus change detection pass for the complete subject;
- the final independent review manifest binds all three packet files, all seven intended canonical files including final `00` status bytes, and the complete proof; a fresh isolated built-in `default` reviewer reports no unresolved actionable finding;
- any accepted in-scope blocker is remediated only within the authorized canonical subject, the complete proof wall reruns, and a different fresh reviewer returns `CLEAN`; no subject byte changes after `CLEAN`;
- proof and staging replay the clean manifest byte-identically before the primary subject commit, followed only by the parent-owned separate v1.2 handoff/ledger closeout commit; and
- HCM-0.6 is not auto-started.

## HCM-0.6 shipped-default documentation-decision proof gate

HCM-0.6 closes `PG-DEFAULT-01` only as a reviewed target-data decision. It does
not promote `PG-PROFILE-01`, `PG-KIND-01`, `PG-ARTIFACT-01`, `PG-INTAKE-01`,
`PG-INTAKE-02`, `PG-CHARTER-01`, `PG-YAML-01`, `PG-YAML-02`, or any runtime,
transport, contract, dock, or gate proof. Before HCM-0.6 may close:

- the review-clean research dossier separates primary-source facts, current
  repository precedent, local inference, and user authority;
- Minimal, Standard, and Full candidates use the same rubric and keep shipped
  kinds distinct from selected instances and requiredness;
- the explicit user decision record fixes exactly six first-party kind refs,
  exactly three root-profile instance descriptors, always/always/conditional
  requiredness, the exact managed-operational-surface condition ref/policy,
  and one unique `constitutional_root` provider;
- the machine tables pin full exact namespaced kind refs, the
  `handbook.capabilities.constitutional-root@1.0.0` capability contract, and
  `handbook.roles.core@1.1.0` with
  `sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029`;
- kind identity, instance identity, stable role, semantic capability, label,
  path, requiredness, materialization, and authority remain separate typed
  decisions even where names match;
- `project_authority` is always required and uniquely selects
  `constitutional_root` through a conforming Project Authority kind;
  `project_context` is always required factual orientation/reference truth;
  `environment_context` is required only on positive independently
  authoritative evidence and no indeterminate outcome becomes false;
- the root profile selects no Work Specification, Decision Record, or Risk
  Record and setup/doctor receive no authority to scaffold empty artifacts for
  them;
- lifecycle/reassessment responsibilities do not invent policy IDs, states,
  freshness thresholds, retention, mutation, or automation;
- all six kinds receive a future first-party schema-backed intake and fixed
  deterministic human-review renderer while canonical YAML remains authority,
  fixed renderers remain outside capitalized Projection, and the shipped root
  profile initially selects no Projection definitions;
- deferred runbook, quality-strategy, catalog, and other artifact roles remain
  outside the six-kind catalog unless separately amended with evidence;
- affected `00`-`06` authority, the slice decision/SPEC/plan/todo, and final
  proof agree without modifying Rust, Cargo, runtime, schemas, profile assets,
  setup/doctor, CLI, SDK, Tauri, Substrate, or HCM-0.7;
- `git diff --check`, archive-boundary checks, Markdown/link/identity/refusal
  assertions, handoff validation/self-tests, and staged GitNexus change
  detection pass;
- a different-fresh isolated built-in `default` reviewer reports `CLEAN` over
  the complete exact subject and proof wall; and
- the primary reviewed documentation commit plus a second mechanical v1.2
  completed handoff/ledger closeout bind and replay the exact subject.

## HCM-0.7 implementation-program and first-packet proof gate

HCM-0.7 approves planning authority only. It does not execute HCM-1.1, change
Rust/Cargo/runtime/assets, promote the Artifact kind/schema registry seam, or
close `PG-KIND-01`/`PG-KIND-02`. Before HCM-0.7 may close:

- completed v1.2 records for HCM-0.2, HCM-0.3, HCM-0.4, HCM-0.5, HCM-0.6,
  and HCM-0.8 validate as dependency evidence, while HCM-0.9 remains abandoned
  and the monolithic `05` remains canonical;
- `00-README.md` and `04-phase-slice-map.md` agree that Phase 0 is closed,
  phases execute in numeric order, slices inside a phase execute in listed
  order, phase exits gate the next phase, and only a review-clean slice-local
  packet authorizes implementation;
- only HCM-1.1 is authorized, packet approval is not execution, later slices
  remain unauthorized, and no parallel cross-slice execution or sibling cleanup
  is implied;
- `slices/HCM-1.1/SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` exist and agree
  on exact objective, owner, live baseline, public boundary, source/asset
  topology, dependency configuration, task ordering, proof wall, non-goals,
  classification ceiling, and stop conditions;
- the HCM-1.1 packet preserves the current four-variant
  `CanonicalArtifactKind`, layout/setup/doctor/flow paths, and HCM-1.2 through
  HCM-1.4 ownership while authorizing only an additive `handbook-engine`
  kind/schema-registry boundary;
- the packet requires a bounded lowercase-ASCII definition-identity grammar,
  canonical full-SemVer roundtrip, exact ref delimiter, RFC 8785/SHA-256
  closure fingerprints, closed typed definition decoding with duplicate/unknown/
  wrong-record/non-empty-extension refusal before fingerprinting, Draft 2020-12
  meta/instance validation, an exact root Draft 2020-12 declaration plus closed
  schema-position keyword allowlist, explicit in-memory local-ref registration
  with all resolver features disabled, a frozen v1 identifier/resource policy
  that refuses authored base rebinding, anchors, and dynamic references,
  repo-relative no-follow access, bounded inputs, typed fail-closed errors, and
  no custom executable keywords, remote refs, ambient discovery, or
  source-order winner;
- the packet's positive proof includes one capability-free repository-defined
  custom kind loaded and structurally exercised without a new Rust enum variant
  or CLI command, while non-empty semantic-capability, renderer, lifecycle,
  intake, Projection, instance, and profile publication stays outside the
  HCM-1.1 loader and refuses until an owning later packet supplies type-specific
  source bytes, normalization/fingerprint producers, compatibility rules, and
  non-vacuous proof;
- the custom proof corpus contains at least two distinct valid schema entries
  and two distinct valid kinds so both accepted source permutations must yield
  identical schema/kind registry fingerprints, exact lookup sets, closures, and
  validation behavior rather than a vacuous one-element reversal;
- the packet permits at most `BoundaryLanded` for the Artifact kind/schema
  registry and bounded kind/schema structural evidence for `PG-KIND-01` and
  `PG-KIND-02`; both gates remain open until later slices prove the missing
  non-vacuous lifecycle/Projection and supplied-intake coverage; it forbids
  product-path, shipped-profile, release, or downstream proof claims;
- Markdown relative links, headings, packet cross-references, prohibited-path
  assertions, archive boundary/self-test, all handoff validator modes, exact
  scoped diff, `git diff --check`, and repository-required change detection
  pass;
- a fresh isolated built-in `default` reviewer reports no valid Critical or
  Required finding over the complete phase-map/packet/proof subject; any valid
  finding is remediated and checked by a different fresh reviewer; and
- proof and staging replay the clean subject byte-identically before the primary
  documentation/planning commit, followed only by the separate parent-owned
  v1.2 handoff/ledger closeout commit.

## HCM-1.1 additive kind/schema registry boundary proof gate

HCM-1.1 lands only the additive `handbook-engine` owner-library boundary. It
does not adopt the registry in current canonical artifact, layout, setup,
doctor, flow, compiler, CLI, SDK, Tauri, Substrate, profile, instance, intake,
renderer, lifecycle, Projection, publication, or downstream paths. Before the
parent may close HCM-1.1:

- exact definition refs enforce the bounded lowercase-ASCII namespaced identity
  grammar plus byte-canonical full SemVer, and definition fingerprints replay
  lowercase SHA-256 over RFC 8785 JSON with duplicate-key and fixed source/
  aggregate-limit refusal before typed decoding;
- the package owns exact `handbook.roles.core@1.0.0` and `@1.1.0` bytes whose
  frozen fingerprints replay, whose typed records reject unknown fields,
  invalid categories, duplicates, and substitutions, and whose assets appear in
  `cargo package` contents;
- schema-registry entries are explicit repository-relative YAML sources, schema
  documents stay under explicit allowed roots, every component/final file is
  opened descriptor-relatively with safe `rustix::fs::openat` plus no-follow
  flags and a retained final handle, bounded reads stop after one sentinel byte,
  and the bounded closure binds exact
  document, entry, closure, and registry fingerprints;
- the Draft 2020-12 schema profile requires the exact root dialect, rejects
  nested dialects, authored identifiers/anchors/dynamic or recursive refs,
  unknown schema-position keywords, ambient/remote/file/data/query/backslash/
  encoded-traversal refs, unsafe pointers, cycles, missing targets, drift, and
  prewalk/validator-target mismatches, while object-valued instance data under
  `const`/`enum`/`default`/`examples` remains data;
- JSON Schema resolver features remain disabled, the validator registry contains
  only admitted in-memory `handbook+repo:///` resources, regex compilation uses
  the linear engine, and the engine dependency graph contains no HTTP/TLS,
  async, file-resolver, custom-keyword, custom-format, or executable-hook path;
- capability-free `ArtifactKindDefinition` records select one exact built-in
  stable-role registry, one exact schema entry/closure, known stable roles, and
  the exact structural profile, while every non-empty semantic capability,
  required capability, semantic validator, renderer, lifecycle policy, review
  trigger, Projection ref, extension, opaque dependency producer, wrong-record,
  instance, setup, or intake field refuses before entering the fingerprint;
- artifact-kind definition fingerprints bind only the normalized capability-free
  record plus the exact selected stable-role and schema-entry/closure
  fingerprints, and kind/schema registry fingerprints are sorted exact-member
  closures without source-order winners;
- the committed proof corpus supplies two distinct schema entries and two
  distinct kinds, a real local `$ref` closure, a valid YAML instance, stable
  invalid-instance locations, deterministic repeated fingerprints, and equal
  forward/reverse source-permutation lookup sets and behavior without adding a
  `CanonicalArtifactKind` variant or product-path command;
- focused positive, negative, security, format, clippy, engine, full-workspace,
  dependency-feature, package-content, archive-boundary, handoff-validator, and
  scoped-diff proof passes, and repository-required staged GitNexus detection
  reports only the expected additive owner-library flows;
- `Artifact kind/schema registry` promotes only to `BoundaryLanded`;
  `PG-KIND-01` remains open for lifecycle/Projection coverage and `PG-KIND-02`
  remains open for supplied-intake coverage, while every sibling gate and
  product-path classification stays unchanged;
- one fresh isolated read-only built-in `default` reviewer reports no actionable
  Critical or Required finding over every implementation, asset, fixture,
  packet-status, crosswalk, gate-evidence, proof, and immutable dispatch byte;
  any accepted finding receives bounded remediation, a complete proof replay,
  and a different fresh reviewer; and
- the final clean manifest and proof replay byte-identically before the primary
  reviewed-slice commit, followed only by a separate completed v1.2 parent
  handoff/ledger closeout commit; HCM-1.2 is not started.

## Control-pack orchestration-repair proof gate

The immutable HCM-0.1 history remains evidence of the prior workflow and is not rewritten. At corrective slice HCM-0.8's completed closeout, the following conditions had to hold; the later HCM-0.6 decision resolves the shipped-default item that was intentionally open at that boundary:

- all control-pack files exist and link correctly;
- all versioned handoff/internal-dispatch schemas, current templates, every immutable handoff record, current JSON dispatch, and every ledger entry pass Draft 2020-12 validation with exact record/index parity and a byte-identical deterministic ledger rebuild; the validator hash-admits all exact pre-correction v1.0/v1.1 records and eight legacy Markdown dispatches, and deterministic negative proof rejects unknown/modified/deleted historical records or dispatches;
- completed-closeout negative proof preserves a final manifest that includes the pre-closeout ledger by replaying it against the primary reviewed commit, then validates the mechanically changed post-closeout ledger through exact record/index parity;
- README selective-loading and authority rules are complete;
- orchestration prompt requires explicit phase/slice selection, can select latest-for-slice or a specified resume handoff, and treats the handoff as context rather than work-selection authority;
- Snapshot Memory semantics are threaded through architecture, contracts, phase sequencing, proof, orchestration, and optional handoff refs;
- artifact-kind/instance separation, repository-defined schemas, adaptive intake, Charter authority, and posture recommendation semantics are threaded through architecture, contracts, sequencing, proof, and skill/orchestration guidance;
- the pack consistently classifies fixed deterministic pre-Phase-3 human-review outputs as renderer-derived views with no Context Resolution input and outside the capitalized Phase-3 Projection request/result/provenance contract;
- the shipped default artifact set is explicitly unresolved pending `HCM-0.6` research and user brainstorming/decision;
- escalation protocol distinguishes parent-local remediation, internally dispatched decomposition/docs/proof work, broader design/authority, external/human blockers, context boundaries, and delegation-capability failure;
- internal dispatches declare execution target, parent orchestration, fresh `default` agent, role, replayable subject manifest, required skills, complete return contract, and parent-owned closeout;
- the top-level runner waits for built-in subagent results and enforces review -> remediation -> different fresh review without ordinary user-mediated task hops or subagent-authored global handoffs;
- active docs point to this pack without treating archived docs as authority;
- no Rust files changed;
- `git diff --check` passes;
- a fresh built-in independent reviewer completes review inside the active top-level orchestration and reports no unresolved actionable findings before the corrected pack is treated as frozen implementation authority;
- if an independent review reports an actionable finding, the parent remediates it directly or through a fresh internal remediation agent, then a different fresh built-in reviewer completes another review before `HCM-0.8` may close;
- the final v1.2 parent-owned handoff records proof-relevant delegated runs and a genuine stop reason; no internal review/remediation round writes its own canonical handoff or ledger entry.

## HCM-0.9 abandoned contract-catalog decomposition gate

`PG-CATALOG-01` did not close and is retired with the abandoned slice. The
terminal simplified planning subject is preserved at checkpoint `f3a33ddb55443d37f3a51ffb58f1c85b74a28b23`
and fingerprint
`sha256:c0a719f7d35f7eff0ce73cb008baf2593b73ef43001fbf5068b902f46492451c`.
Redesign Review 2 retained one Required finding: the planned negative proof
permitted a forbidden runtime **or** HCM-0.5 fixture instead of requiring both
distinct categories. The authorized review budget ended there.

No `05` payload byte moved, no leaf/index/verifier was created, no runtime or
HCM-0.5 work occurred, and no topology proof is promoted. The monolithic `05`
file at the frozen HCM-0.4 authority remains canonical. Any future decomposition
requires a new explicit human decision and a newly reviewed slice packet; it
must not resume or silently repair HCM-0.9.
