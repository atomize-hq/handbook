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
| `PG-DEFAULT-01` | focused research plus a user brainstorming/decision session explicitly approve the shipped kind set, default instances, and requiredness; examples/current enums do not count | open |
| `PG-KIND-01` | a versioned `ArtifactKindDefinition` resolves a safe canonical schema, validation, optional intake, lifecycle, and projections independently from repository instance state | open |
| `PG-KIND-02` | repository-defined custom kind registers, passes meta-schema/structural validation, and exercises supplied intake coverage without a new Rust enum variant, executable hook, remote schema fetch, generated CLI command, or pre-Phase-3 generic projection engine | open |
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
| `PG-CONTRACT-01` | locked contract drives claim evaluation and lifecycle-aware gate | open |
| `PG-DOCK-01` | real external process validator emits normalized evidence under declared protocol/Resolution | open |
| `PG-GATE-01` | hard failure blocks regardless of weighted score; required not-observed cannot green | open |
| `PG-SUB-CLI-01` | Substrate uses exact bundled CLI/schema in a real replaceable seam | open |
| `PG-PUBLISH-01` | new downstream-intended API passes exact crates.io external consumer proof | open |
| `PG-SUB-RUST-01` | current-tip Substrate worktree uses exact new crates.io API in a real seam | open |
| `PG-HANDOFF-01` | version-routed schemas validate parent-owned true-stop handoffs; the ledger rebuild is byte-identical; immutable history/supersession, scoped stop/resume, and repository-relative refs work without internal subagents writing global records | HCM-0.8 validation and two-commit negative proof complete; closes only when the HCM-0.8 v1.2 parent record and rebuilt ledger validate |
| `PG-HANDOFF-02` | once snapshots land, handoffs reference start/end snapshots and delta; orchestration rechecks current state before dispatch | open |
| `PG-ORCH-01` | an explicitly selected phase/slice remains owned by one active parent that executes built-in `default` subagents, captures identity/status, collects results, and completes review -> valid-finding remediation -> different fresh review without an ordinary user-mediated task hop | proven by the HCM-0.8 one-parent multi-round review/remediation loop; final lineage capture belongs in the v1.2 parent record |
| `PG-ORCH-02` | every current internal dispatch declares execution target, parent ID, role, replayable subject manifest/fingerprint, fresh-context requirement, closeout owner, ordered required-skills chain beginning with `using-agent-skills`, and complete structured return contract; unavailable mandatory delegation fails closed | proven by HCM-0.8 current-schema dispatch validation and fail-closed orchestration self-tests |

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
55. mutation keys bind repository, negotiated API/bootstrap context, operation definition, and request fingerprint; the retained-result/tombstone by same/different-fingerprint matrix is total, same-fingerprint races replay one winner, and different-fingerprint races conflict. The original-result fingerprint is RFC 8785/SHA-256 over the exact committed semantic response closure and realized receipts while excluding correlation, idempotency, and outer response fingerprint; retained replay and tombstone compaction preserve that identity. Pre-definition parseable selection failures use only the non-null-fingerprint descriptor-bound admission envelope with null trusted operation fields; its reason-discriminated variants distinguish missing, malformed, unknown/incompatible, client-pin-missing, registry-missing, mismatch, and stale state; its exact four-leaf requested-API DTO and fixed path order classify absent/null/partial/malformed/mismatched input; attempted-value digests bind one named raw JSON value; the total operation/API/definition precedence emits only winning-field evidence; the request-object digest deletes exactly top-level correlation `request_id` and client-supplied derived `request_fingerprint` before canonicalization but preserves every other compound-invalid field; admission correlation safely echoes only allowlisted IDs and otherwise emits deterministic null; exact Problem details include stage/reason and mirror that evidence under an injective code/rule/details-schema mapping; and the manifest closes over descriptor-owned schemas only. Duplicate JSON member names at any nesting level, like other pre-object normalization failures, yield no Handbook response. Accepted read-only responses use only not-applicable, accepted mutating pre-commit failures only not-established, and committed mutations only established. Memory-promotion disposition alone establishes before its authorized domain compare so a new-key stale basis writes/replays the required terminal stale disposition instead of leaving the request pending; the crash-recoverable idempotency journal is explicitly non-authoritative protocol-control metadata outside governed write sets/receipts.
56. each API major has a distinct immutable mechanically derived bootstrap-descriptor ref; the descriptor or exact compile-time Rust types are the only discovery roots; snapshot-bound paged operation/profile/schema/governed-record catalogs, exact schema/record/vocabulary/Resolution-stack/Projection-definition reads, and deterministic applicable-snapshot selection provide complete restart-safe machine inputs without inferred defaults or repository/prose reads.
57. transport JSON Schema/operation/API identities use explicit full SemVer domains with mechanically derived refs, while frozen two-component domain-record routing tags are non-schema identifiers that never alias a public schema ref; deprecation names an exact replacement operation and migration artifact, and stale/unsupported/cross-major/tampered state fails closed.
58. shared problem/problem-binding, diagnostic, next-action, versioned artifact, closed locator, source, omission, provenance, schema-manifest, catalog-page, write-receipt, and idempotency DTOs have exact fields, unique instance identity, total category/nullability/stage partitions, enums, bounds, sensitivity/no-follow rules, duplicate policy, canonical ordering, and independently recomputable non-circular instance fingerprints.
59. posture recommendation evaluation is read-only and total over recommendation/no-recommendation; only the separate append operation persists an immutable recommendation.
60. direct SDK, CLI JSON, and Tauri paths preserve the same DTO semantics, negotiated API/request binding, status/data/outcome/provenance/schema/idempotency/write-receipt fields, operation fingerprints, and safe string-or-null correlation matrix; after exact definition acceptance invalid correlation is an ordinary request-validation refusal with invariant semantic/Problem fingerprints. Human output renders from typed results and no transport creates a second decision path; a post-commit serialization/delivery failure emits no false ordinary error and recovers the committed result/receipts only through established-key replay.
61. JSON stdout is one atomic schema-valid response followed by zero or one LF for every selected nontrivial JSON operation; the bridge accepts the identical framing and rejects all other trailing bytes; stderr is non-authoritative; exit/status mappings agree; domain-negative results inside successful data do not become transport errors.
62. the Tauri adapter uses SDK calls without a normal-operation CLI subprocess or frontend semantic authority; adapter errors cannot replace expected typed blocked/refused outcomes with prose, pre-commit versus post-commit failures remain distinct, and post-commit transport uncertainty cannot fabricate an `error` domain result.
63. the transitional Substrate bridge pins the binary/bootstrap/operation/schema/capability fingerprints, bounds process/resources, parses exactly one validated JSON response, never parses stderr/human text, transports sensitive bodies only through bounded stdin or trusted exact refs, proves no argv/env/stderr/process/temp leakage, and remains isolated under `BR-SUB-CLI-01`. Any commit-uncertain timeout/truncation/invalid-output path preserves the exact original envelope/key/fingerprint and recovers only through same-key established replay with the original-result identity and receipts, never a fabricated error, new key, or duplicate write.
64. bridge proof, Tauri proof, packaged-crate proof, registry proof, and direct Substrate adoption are separate subjects/classifications; none substitutes for another.
65. each changed crate is proved, packaged, isolated, published, and checksum-resolved from crates.io before the next dependent DAG node is packaged; a dependent is never proved against a workspace/path build and later treated as registry-equivalent.
66. downstream-intended Rust APIs are complete only after exact crates.io publication, registry-only external resolution, a current-tip Substrate worktree with a named real seam, positive/negative/no-fallback proof, and preserved review evidence; public symbols, path builds, manifest pins, or toy consumers do not count.
67. HCM-0.5 may append contract/dock ordinary operations after semantic freeze but cannot change the HCM-0.4 owner graph, typed DTO/outcome rules, transport parity, bridge boundary, or published-proof plan.

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

HCM-0.2 is a documentation/design freeze. Its closeout does not promote any runtime implementation gate in the open-program table. Before HCM-0.2 may close:

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

HCM-0.4 is a documentation/design freeze. It does not create crates, implement SDK/CLI/Tauri/Substrate behavior, publish packages, select shipped artifact defaults, freeze HCM-0.5 contract/dock semantics, or promote `PG-SDK-01`, `PG-JSON-01`, `PG-TAURI-01`, `PG-SUB-CLI-01`, `PG-PUBLISH-01`, or `PG-SUB-RUST-01`. Before HCM-0.4 may close:

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

## Control-pack orchestration-repair proof gate

The immutable HCM-0.1 history remains evidence of the prior workflow and is not rewritten. Before corrective slice `HCM-0.8` may close:

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
