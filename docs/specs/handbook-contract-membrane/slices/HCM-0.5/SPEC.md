# HCM-0.5 Specification: Contract Membrane and Dock Protocol Freeze

## Status and authority

Planning subject for HCM-0.5. This packet becomes slice-local execution authority only after a fresh independent planning review reports `CLEAN`. Until then, the monolithic `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` remains canonical and its contract/dock sections remain preliminary.

This slice is documentation/design-only. It authorizes no Rust, Cargo, CLI, Tauri, Substrate, SDK, dock process, validator, schema file, or runtime implementation.

## Runtime selection

```text
PHASE_ID: HCM-0
SLICE_ID: HCM-0.5
ACTIVE_PACKET: docs/specs/handbook-contract-membrane/slices/HCM-0.5
HANDOFF_SELECTOR: 20260715T202049Z--HCM-0-5--orchestration--planning-review-budget-exhausted
```

HCM-0.4 is completed dependency evidence at `git:214a5b8eb182fce74478df49d4f55d226d65fdf5` and in `handoffs/records/20260715T141656Z--HCM-0-4--orchestration--sdk-transport-contracts-frozen.json`. HCM-0.9 is abandoned evidence only. It supplies no resume context, topology authority, or contract-catalog leaf authority. The selected HCM-0.5 handoff is immutable resume evidence for the exhausted historical planning-review lineage; the human-authorized continuation below replaces its stop recommendation without rewriting that record or its dispatches.

## Selective context capsule

```text
SLICE / OBJECTIVE: HCM-0.5 / freeze contract, evidence, verdict, gate, and dock semantics
ACTIVE PACKET: docs/specs/handbook-contract-membrane/slices/HCM-0.5
DEPENDENCY / AUTHORIZATION PROOF: HCM-0.2, HCM-0.3, HCM-0.4, and HCM-0.8 are closed design dependencies; Phase 0 authorizes docs/design only
SELECTED HANDOFF / VALIDITY: 20260715T202049Z--HCM-0-5--orchestration--planning-review-budget-exhausted; schema/ledger-valid immutable resume evidence; its Reviews 1-4 remain historical and are not renumbered
ACTIVE RESOLUTION ENVELOPE: repository-local documentation scope; no product Resolution label/default is selected; proof comparisons use the frozen six explicit dimensions
GROUNDING SNAPSHOT / START DELTA: Snapshot Memory runtime is not implemented; branch, HEAD, clean status, dependency record, and canonical 05 fingerprint are live preflight evidence
TARGET AUTHORITY BOUNDARY: handbook-contracts owns protocol-neutral semantics; separable docks witness; SDK composes ordinary operations; transports remain thin
CURRENT REPO-TRUTH STATUS: target contract/dock semantics are preliminary and no general contract membrane or external dock runtime exists
MUST-READ PACK SECTIONS: 01 authority/dock/invariants; 02 Resolution/validation/adapter rules; 03 membrane/dock rows; 04 HCM-0.5 and Phase 5; 05 HCM-0.4 operation/transport rules plus preliminary contract/dock sections; 06 open gates/regressions
LIVE SOURCE / TESTS / PRECEDENT: handoff Draft 2020-12 validator and local schema policy are implementation precedents only; ideas memo supplies design lineage, not owner override
SIBLING SEAMS IN CONTEXT: HCM-0.2 semantic identities, HCM-0.3 Resolution/Snapshot/Projection, HCM-0.4 SDK/transport, HCM-0.6 defaults, HCM-5.1 through HCM-5.4 runtime
ALLOWED AREAS: current continuation: this three-file packet, additive continuation-review dispatches, final planning-continuation handoff, and ledger; fresh execution session after approved planning: affected 01-06 sections
EXPLICIT NON-GOALS: canonical 00-06 edits in this continuation; HCM-0.9 repair/decomposition; leaf catalogs; Rust/runtime work; shipped defaults; universal validator; marketplace; HCM-0.6
APPLICABLE CONTRACTS / PROOF GATES: PG-CONTRACT-01, PG-DOCK-01, PG-GATE-01 stay open; HCM-0.5 documentation-freeze gate is added
REQUIRED SKILL CHAIN: using-agent-skills -> context-engineering -> spec-driven-development -> planning-and-task-breakdown -> api-and-interface-design -> security-and-hardening -> incremental-implementation -> documentation-and-adrs -> debugging-and-error-recovery as needed -> code-review-and-quality -> git-workflow-and-versioning
KNOWN CORRECTIONS OR CONFLICTS: preliminary 05 conflates passed/blocked evaluation with contract lifecycle; freeze must separate them without changing HCM-0.4 contracts
MAXIMUM PERMITTED CLASSIFICATION / PROOF CHANGE: documentation/design freeze only; runtime seams remain TargetOnly and runtime proof gates remain open
EXIT PROOF: this continuation exits after a clean Continuation Planning Review, complete packet validation, approved planning commit, and partial/context-boundary v1.2 handoff; canonical freeze and Design Reviews 1-3 belong to the authorized fresh execution session
STOP CONDITIONS: Continuation Planning Review 2 non-clean; a non-bounded or out-of-authority Continuation Planning Review 1 result; product choice absent from authority; broader redesign; runtime proof required; mandatory delegation unavailable
```

## Objective

Freeze one stable, versioned, fail-closed semantic membrane that makes locked Handbook contracts authoritative while every validator remains a bounded witness. Define enough protocol-neutral and process-dock detail that HCM-5.1 through HCM-5.4 can be implemented without re-deciding lifecycle, evidence, verdict, gate, transport, isolation, or first-proof-dock semantics.

## Success criteria

1. Contract definitions, lifecycle transitions, claims, applicability, evidence, verdicts, gates, dock manifests, dock runs, and process results each have exact identity, owner, fingerprint, validation, and authority rules.
2. Evaluation state is not encoded as contract lifecycle state.
3. Missing, stale, refused, failed, out-of-scope, out-of-Resolution, flaky, or malformed evidence cannot produce green truth.
4. Process JSON is the first universal dock transport; future Rust-native docks preserve the same semantic request, evidence, verdict, and gate contract.
5. Docks receive least-privilege grants, bounded resources, no ambient authority, and deterministic cancellation/timeout/failure handling.
6. The first runtime proof target is selected from explicit criteria and repository evidence rather than current naming or implementation inertia.
7. HCM-0.5 adds ordinary SDK operation definitions without modifying any HCM-0.4 owner, operation-definition, DTO, transport, bridge, or publication contract.
8. The control pack stays monolithic, HCM-0.2/0.3/0.4 freezes regress cleanly, and no runtime gate is promoted.

## Exact in-scope semantic questions

The design freeze must answer all of these questions in canonical 01-06 authority:

1. What immutable tuple identifies a contract definition and what changes require a new version/fingerprint?
2. Which lifecycle states and transitions exist, who may authorize each transition, and which transitions are terminal?
3. How are contract lifecycle, evaluation runs, verdicts, and gate decisions kept separate?
4. How does a claim declare subject, case, applicability, severity/gate effect, required evidence kinds, freshness, minimum Resolution, and optional score weight?
5. What evidence identity, provenance, time basis, source closure, dock binding, subject/case binding, and Resolution closure are required?
6. What proves evidence fresh, applicable, visible, and sufficiently authoritative, and what must instead become `not_observed`, `not_applicable`, `blocked`, or `flaky`?
7. What are the exact claim-verdict meanings, allowed evidence cardinality, and precedence rules?
8. How are claim verdicts composed into a gate, and why can no score, waiver-like extension, or adapter override a hard failure or required missing evidence?
9. What does a dock capability manifest promise, and how is its exact identity selected and pinned?
10. What exact JSON request/result framing, status model, and fingerprint bindings does a process dock use?
11. What filesystem, environment, network, process, output, time, cancellation, and resource authority may a dock receive?
12. How do refusal, timeout, cancellation, crash, nonzero exit, malformed output, oversized output, and protocol mismatch fail closed without fabricating evidence?
13. Which actor validates dock candidates into canonical evidence, computes verdicts, composes gates, and performs lifecycle transitions?
14. Which existing validator ecosystem is the first HCM-5.4 proof target, and what bounded claim does it prove?
15. Which ordinary SDK operations are added, with what owners, mutability, idempotency, transport targets, and write sets?

## Owner and authority boundaries

| Surface | Owner | Authority | Forbidden responsibility |
|---|---|---|---|
| Contract definitions, claims, lifecycle rules, canonical evidence, verdicts, gates, protocol-neutral dock DTOs | `handbook-contracts` | canonical semantic owner | process spawning, CLI/Tauri/Substrate wording, universal validation algorithms |
| Exact artifact/profile/Resolution/Snapshot types referenced by contracts | `handbook-engine` | existing HCM-0.2/HCM-0.3 owner | contract or gate decisions |
| Ordinary contract/dock use-case composition | `handbook-sdk` | HCM-0.4 facade only | becoming a second semantic owner |
| Process-dock execution | separable future adapter selected by its HCM-5 packet | operational execution only | canonical evidence, verdict, gate, or contract authority |
| Concrete validator | external/separable dock | witness candidate production | declaring contract truth, final verdicts, gates, waivers, or lifecycle transitions |
| CLI/Tauri/Substrate | existing HCM-0.4 adapters/consumer | invocation and product orchestration | reinterpreting dock or contract semantics |

The membrane validates and normalizes a dock's output before creating a canonical evidence record. A dock result is never canonical evidence merely because the dock emitted it. Only `handbook-contracts` evaluates claims and gates; only the named lifecycle authority may append a transition.

## Contract definition identity and compatibility

Every immutable `ContractDefinition` declares `contract_id`, full-SemVer `contract_version`, and derives `contract_ref` exactly as `contract_id + "@" + contract_version`. `contract_id` is a stable lowercase dot-separated machine identity and is never derived from a filename, title, CLI command, artifact kind, or vocabulary label. The definition binds one exact definition-schema ref/fingerprint, claim order and full claim closure, lifecycle-policy ref/fingerprint, matcher refs/fingerprints, gate policy, extensions policy, and `definition_fingerprint`.

`definition_fingerprint` is lowercase SHA-256 over RFC 8785 canonical JSON of every semantic field except itself. Same `contract_ref` with different bytes/fingerprint is a conflict and refuses; changed normalized semantic bytes always require a new version/ref. Selection is always by exact ref/fingerprint—SemVer ranges, `latest`, ambient fallback, and compatible-version substitution are forbidden in v1.

| Change | Minimum version change | Compatibility meaning |
|---|---|---|
| non-authoritative display annotation only, with identical claim/lifecycle/applicability/evidence/gate closure | patch | no automatic substitution; exact pin still required |
| add an advisory claim whose score weight is omitted, which is absent from every required/hard gate set, and which changes no existing claim or policy | minor | additive authoring signal only; exact pin still required |
| add/remove/change any existing claim; add a hard/required claim; change applicability, evidence kind/cardinality/freshness/Resolution, matcher, lifecycle, score, gate, schema, subject/case, or authority semantics | major | breaking semantic definition |

Any change not proved to meet the patch or minor row is major. Reordering claims is semantic because verdict and diagnostic order are deterministic. Deprecation or closure never reuses an old ref for new meaning.

## Proposed lifecycle model for the freeze

Contract definition lifecycle is distinct from evaluation state:

```text
draft -> review_ready
draft -> closed
review_ready -> locked
review_ready -> closed
locked -> active
locked -> deprecated
active -> deprecated
deprecated -> closed
```

- `draft`: immutable authoring candidate that may be superseded by a new version; cannot drive a gate.
- `review_ready`: frozen candidate awaiting named review authority; cannot drive a gate.
- `locked`: immutable exact definition accepted by the lock authority; may be activated.
- `active`: locked definition eligible for new evaluation within declared applicability.
- `deprecated`: immutable and readable; no new default selection, but exact historical/replay use remains valid until closure policy forbids it.
- `closed`: terminal for new evaluation; immutable historical reads/replay remain available.

Every transition is an append-only `ContractLifecycleTransition` binding prior state/ref/fingerprint, requested state, actor/authority, exact transition-policy ref/fingerprint, supporting evidence refs, decision time, and resulting lifecycle fingerprint. Transition compare-and-write rejects stale current state. A definition is immutable even while `draft`; authoring correction appends a new version and withdraws the old version rather than changing its bytes. `passed` and `blocked` are gate/evaluation outcomes, never lifecycle states.

The transition table is closed; all unlisted transitions refuse:

| From | To | Required authority and evidence | Meaning |
|---|---|---|---|
| `draft` | `review_ready` | definition author; schema/semantic validation evidence | submit exact immutable definition |
| `draft` | `closed` | definition author or lifecycle authority; typed withdrawal reason | withdraw before review |
| `review_ready` | `locked` | named lock authority distinct from the definition author; clean review evidence over exact fingerprint | accept as immutable contract authority |
| `review_ready` | `closed` | named lock authority; typed rejection evidence | reject exact candidate |
| `locked` | `active` | activation authority; applicability/matcher/currentness validation | permit new evaluation |
| `locked` | `deprecated` | lifecycle authority; replacement ref/fingerprint or typed no-replacement rationale | retire before activation |
| `active` | `deprecated` | lifecycle authority; replacement ref/fingerprint or typed no-replacement rationale | stop default/new selection while retaining exact replay |
| `deprecated` | `closed` | closure authority; retention/reference safety evidence | terminally prohibit new evaluation |

Self-lock is refused even if one actor happens to hold both labels; the transition requires distinct author and lock-actor identities. `closed` is terminal. `active -> closed`, reactivation, rollback, state skipping, and transition of a stale/non-current lifecycle basis refuse. A semantic revision creates a new exact definition/version and independently deprecates the former version; lifecycle never changes a definition's bytes.

## Claims and applicability

Each claim has one exact contract-local `claim_id`, statement, typed subject selector, case selector, applicability rule, `gate_effect` (`hard_fail`, `required`, or `advisory`), a non-empty ordered `evidence_requirements` list, freshness policy, complete six-dimension minimum Resolution, and optional `score_weight`. Selectors are closed declarative data evaluated by an exact registered matcher; executable predicates, prompts, remote code, and content-sniffed applicability are refused.

Each evidence-requirement clause declares exactly one evidence kind, one cardinality variant, an exact case set or the claim's case selector, and one stability-policy ref/fingerprint. Clauses are **all-of** in v1: every clause must satisfy independently for the claim to pass. Alternative/any-of groups, dock-preference order, and one kind substituting for another are not supported. Evaluation and cardinality operate on each exact `(claim_id, case_id, evidence_kind)` tuple, then combine clause outcomes with this precedence: `blocked` > `flaky` > `fail`/`warning` > `not_observed` > `pass`. `not_applicable` is decided once at claim applicability before clause evaluation.

Evidence cardinality is one closed variant:

| Variant | Satisfaction set | Missing/surplus behavior |
|---|---|---|
| `exactly_one` | exactly one unique eligible canonical evidence ref for the claim/case | zero -> `not_observed`; more than one after exact-ref deduplication -> `blocked` |
| `at_least_one` | one or more eligible refs for the claim/case | zero -> `not_observed` |
| `all_declared_cases` | one or more eligible refs for every exact case in the claim's non-empty declared case set | any case with zero -> `not_observed`; unknown/duplicate case identity -> `blocked` |

Evidence is never selected by newest timestamp, source order, weight, kind substitution, or dock preference. Exact duplicate refs deduplicate only inside the same requirement tuple; distinct records remain distinct. Within every required tuple, all eligible observations must agree under the exact stability-policy ref/fingerprint. Mixed satisfied and violated observations produce `flaky`; all violated observations produce `fail` for hard/required claims or `warning` for advisory claims; all satisfied observations produce `pass`. A malformed/stale/out-of-scope record is ineligible and remains explicit accounting, not an observation vote. Applicability is evaluated before requirement binding; then kind/case/currentness/Resolution eligibility; then per-tuple cardinality; then consistency; then satisfied/violated mapping; finally all clauses combine with the closed precedence above. No later step can override an earlier higher-precedence outcome.

`score_weight` omission means the claim contributes neither numerator nor denominator to weighted score. When present it must be a positive finite number and enters both denominator and, only for `pass`, numerator. Numeric zero, negative, NaN, infinity, null, strings, and transport-defaulted values refuse the definition. Gate effect remains independent of score presence.

Applicability evaluates before evidence satisfaction:

- proven selector false -> `not_applicable`;
- selector true -> evidence evaluation proceeds;
- malformed, stale, unresolvable, or indeterminate selector/matcher state -> `blocked`, never `not_applicable`;
- a claim cannot be declared `not_applicable` by a dock or transport.

## Evidence identity, provenance, freshness, and Resolution limits

Canonical evidence is an immutable `EvidenceRecord` with an exact record ref/fingerprint and binds:

- exact contract version/fingerprint and claim IDs;
- exact evaluation-run/request identity;
- exact subject and case identities;
- evidence kind and observed fact payload/schema;
- producer dock manifest ref/fingerprint and process execution record ref/fingerprint;
- source/artifact/trace refs with fingerprints and collection time;
- complete request and effective six-dimension Resolution envelopes;
- explicit observed, unobserved, and excluded claim partitions;
- freshness-policy ref/fingerprint, observed-at time, evaluated-at time, source revision, and deterministic freshness outcome;
- normalization-policy ref/fingerprint and canonical record fingerprint.

Evidence satisfies a claim only when every identity and source fingerprint matches, the claim is in the observed partition, the evidence kind/cardinality matches, freshness passes, and the effective envelope meets every minimum Resolution dimension. Effective Resolution is the dimension-by-dimension minimum of request envelope, dock capability ceiling, and actual grant/observation envelope. A broader claim cannot be proved by a narrower observation. Resolution qualifies visibility, observation, authority, and proof; it is never an importance score. Evidence never grants mutation or promotion authority.

Missing or hidden source material remains an explicit omission. Stale evidence, out-of-envelope evidence, incompatible subject/case evidence, and a dock's unsupported claim do not count. They remain visible in evaluation accounting and cannot be weighted into green.

## Verdict vocabulary and semantics

Each applicable claim receives exactly one canonical verdict:

| Verdict | Meaning | Gate relevance |
|---|---|---|
| `pass` | sufficient fresh applicable evidence directly observed and satisfied the claim | may satisfy any gate effect |
| `fail` | sufficient fresh applicable evidence directly observed a violation | blocks `hard_fail` and `required`; advisory remains visible warning input |
| `blocked` | evaluation could not validly reach satisfaction/violation because prerequisite, authority, execution, protocol, or selector state failed | blocks `hard_fail` and `required` |
| `warning` | observed violation of an `advisory` claim | legal only for advisory claims; never satisfies `hard_fail` or `required` |
| `not_observed` | claim applies but required observation/evidence is absent, stale, excluded, insufficient-Resolution, or insufficient-cardinality | blocks `hard_fail` and `required` |
| `not_applicable` | the authoritative applicability rule deterministically evaluated false | excluded from score denominator and never supplied by a dock |
| `flaky` | repeated eligible evidence is inconsistent under the declared stability policy | blocks `hard_fail` and `required`; cannot average into pass |

Verdicts bind all supporting and disqualifying evidence refs and record a deterministic reason code. A validator may report observations or candidate diagnostics, but only `handbook-contracts` creates a claim verdict.

## Gate composition and hard-fail behavior

A `GateResult` binds one contract/evaluation run and completely partitions every claim into exactly one verdict. Its decision is `passed` or `blocked`; there is no partial-green state.

The gate-effect/verdict matrix is closed:

| Gate effect | `pass` | `not_applicable` | `fail` | `blocked` | `warning` | `not_observed` | `flaky` |
|---|---|---|---|---|---|---|---|
| `hard_fail` | continue | continue only with proven false applicability | block | block | invalid combination -> block | block | block |
| `required` | continue | continue only with proven false applicability | block | block | invalid combination -> block | block | block |
| `advisory` | continue | continue | invalid combination -> block as evaluator defect | visible non-blocking deficit unless score policy blocks | visible non-blocking concern unless score policy blocks | visible non-blocking deficit unless score policy blocks | visible non-blocking instability unless score policy blocks |

The gate is `blocked` when any of these is true:

1. any `hard_fail` claim is not `pass` or proven `not_applicable`;
2. any `required` claim is not `pass` or proven `not_applicable`;
3. required evidence or claim accounting is incomplete;
4. any input definition, matcher, evidence, verdict, dock manifest, Resolution binding, freshness binding, or fingerprint is stale/invalid;
5. a declared weighted threshold is not met after the hard/required rules pass.

Weights are positive finite advisory progress metadata. `not_applicable` is omitted from the denominator; every other applicable non-pass verdict contributes zero. A weighted score cannot override rules 1-4. Extensions cannot add waiver semantics. Any future waiver requires its own separately authorized canonical contract, not an extension field or dock assertion.

`local_closeout_eligible` and `parent_promotion_eligible` are computed separately from exact policies and default false on any indeterminate state. A local pass does not imply broader promotion.

## Dock capability manifest

A manifest is immutable exact-definition data identified by `dock_id@dock_version` plus `manifest_fingerprint`. It declares exact protocol versions, execution mode, supported contract kinds, claim/evidence kinds, request/result schema pairs, media types, Resolution ceiling for all six dimensions, required input grant kinds, output kinds/ceilings, deterministic/network posture, timeout/cancellation support, resource ceilings, extensions policy, and one exact `DockImplementationBinding`.

`DockImplementationBinding` contains one content-addressed bundle ref/fingerprint, one normalized bundle-manifest ref/fingerprint enumerating every safe relative file path/mode/SHA-256, one entrypoint path/digest, runtime kind (`native` or `bundled_interpreter`), and an exact runtime/dependency-closure fingerprint. A bundled interpreter and every required library are members of the verified bundle closure; ambient host interpreters, dynamic dependency lookup, shebang/PATH resolution, package-manager install, and mutable external runtime refs are refused. The host allowlist maps the exact manifest plus implementation/bundle/closure fingerprints to one local extracted bundle, re-verifies the complete no-symlink/no-extra/no-missing closure and entrypoint digest before body access or spawn, and executes only that entrypoint. A mapping is operational selection, not new dock identity or authority.

Unknown fields/required extensions, ranges/latest resolution, unsupported protocol major, fingerprint mismatch, stale/missing host mapping, executable/package substitution, unbound runtime closure, or an unsatisfied requested capability refuse before body/artifact access or process spawn. The request, execution record, evidence candidate, and canonical evidence record all bind the exact manifest and implementation/bundle/closure fingerprints.

Capability declaration is a ceiling, not proof of a run. The host intersects requested limits with manifest and policy ceilings; a dock cannot widen them.

## Process JSON request/result protocol

The first universal dock transport is a one-shot process protocol:

1. Resolve the selected exact `DockImplementationBinding` through the host allowlist, verify the complete content-addressed bundle/runtime closure and entrypoint digest, and only then select the local entrypoint; never use a shell, ambient `PATH`, ambient interpreter/library resolution, repository script discovery, or command interpolation.
2. Start it in an isolated workspace with sanitized environment and policy-granted resources.
3. Write exactly one bounded UTF-8 JSON request object to stdin, optionally followed by one LF, then close stdin.
4. Accept exactly one bounded UTF-8 JSON result object on stdout, optionally followed by one LF. Any other stdout byte, duplicate JSON member, unknown required behavior, extra document, prompt, ANSI text, or schema/fingerprint mismatch is `protocol_error`.
5. Treat stderr as bounded redacted diagnostics only; it never carries a result, evidence, or authority.

The request binds protocol/schema refs and fingerprints, unique request/run IDs, selected manifest, contract/claim/subject/case identities, complete Resolution envelope, exact input/artifact refs, requested evidence kinds, workspace grant, resource limits, timeout, cancellation grace, network policy, output policy, and request fingerprint.

The dock result status is `completed`, `refused`, or `cancelled`. A completed result contains a complete observed/unobserved/unsupported claim partition, evidence candidates, diagnostics, artifact candidates, timing, actual resource use, actual observation envelope, and result fingerprint. A refused result contains one closed reason code and no evidence/artifact candidates. A cooperative cancelled result contains the exact request/run/cancellation identity and no evidence/artifact candidates. The host separately records total execution outcome as `completed`, `refused`, `timed_out`, `cancelled`, `failed`, or `protocol_error`; a host deadline remains `timed_out` even if termination elicits a cancelled result.

Only `completed` can enter normalization, and each candidate is independently validated. No partial candidate survives a non-completed outcome.

## Isolation, timeout, cancellation, refusal, and failure behavior

- Default grant: read-only staged inputs, one empty bounded output directory, no network, no repository root, no inherited credentials/config/home, no ambient temp access, sanitized locale/timezone, explicit cwd, closed extra file descriptors, and positive CPU/memory/process/output/time ceilings.
- Logical artifact refs are resolved by the host after no-follow safe-path and fingerprint checks. Docks do not resolve arbitrary repository paths. Output refs must be relative, regular, non-symlink files beneath the output grant and pass count/size/media/fingerprint checks before admission.
- Network is unconditionally denied for every v1 process dock. The v1 manifest and request must both declare `network: denied`; any other value refuses before spawn. A future host-mediated egress design requires a later reviewed protocol version and cannot be introduced by manifest extension, invocation policy, environment, proxy configuration, DNS, or dock behavior.
- Timeout is host monotonic time. At deadline the host requests termination, waits the exact grace, then force-kills the process tree. Timeout yields no evidence candidates.
- Cancellation is host-owned and idempotent. It uses the same terminate/grace/kill sequence. A schema-valid `status: cancelled` result may be captured in the operational execution record during grace but never as claim evidence; absent result remains host outcome `cancelled`.
- Pre-spawn mismatch/refusal creates no process. Dock refusal is typed and creates no evidence. Crash, signal, nonzero exit, oversized output, invalid UTF-8/JSON, schema mismatch, fingerprint mismatch, undeclared artifact, isolation setup failure, or cleanup uncertainty fail closed.
- The host/runner creates only the typed execution outcome/record. During deterministic claim evaluation, `handbook-contracts` may map that trusted execution record to a canonical `blocked` verdict. No runner, dock, transport, or adapter may emit any canonical claim verdict, including `blocked` or `not_applicable`.
- Cleanup failure retains/quarantines the isolated workspace under policy and blocks the run; secrets and raw diagnostic bodies never enter public diagnostics.

## First proof-dock selection

### Selection criteria

The first HCM-5.4 dock must: use an existing mature validator ecosystem; have deterministic bounded input/output; run without network; need read-only low-privilege inputs; map observations to one narrow claim kind; exercise manifest/request/result/refusal/timeout/fingerprint/Resolution/evidence/gate semantics; have positive and negative fixtures already available or cheap to create; and avoid selecting a shipped artifact default, CLI design, or broad product policy.

### Candidate comparison

| Candidate | Evidence in the live repository/pack | Fit | Reason not selected first |
|---|---|---|---|
| Draft 2020-12 JSON Schema data-shape dock | exact local schema policy; live Python `jsonschema` handoff validator; multiple current schemas; original architecture lineage names JSON Schema as MVP adapter | strongest: deterministic, offline, read-only, structured violations, existing positive/negative precedent | selected |
| CLI behavior/test dock | current proof corpus and CLI test helpers exist | useful second dock, but process side effects, filesystem state, exit/status mapping, and HCM-0.4 transport implementation make the first proof wider | defer |
| Documentation/link dock | current Markdown control pack and link/archive checks exist | low privilege, but weakly exercises typed evidence payloads and risks letting documentation appear contract-authoritative | defer |

### Selection

Select `handbook.dock.json-schema@1.0.0` as the first proof dock target. Its sole initial responsibility is to validate one JSON-compatible instance against one exact local Draft 2020-12 schema/ref-closure and emit bounded schema-conformance evidence candidates. It refuses remote refs, executable hooks, ambient schemas, unsupported dialects, and fingerprint mismatch. It does not perform semantic validation, intake approval, lifecycle transition, verdict, gate, or canonical mutation and is not a universal validator.

This selection is design authority for the future HCM-5.4 packet, not implementation authorization or proof that `PG-DOCK-01` passed.

## Ordinary operation additions

HCM-0.5 will append definitions for these stable ordinary operation IDs, using the exact HCM-0.4 operation-definition and transport contracts:

| Operation | Owner | Mutability / idempotency | Authority effect / write set |
|---|---|---|---|
| `contract.definition.list` | `handbook-contracts` | `read_only` / `safe` | none / empty |
| `contract.definition.read` | `handbook-contracts` | `read_only` / `safe` | none / empty |
| `contract.definition.append` | `handbook-contracts` | `append_only` / key required | append one `semantic_record` draft definition |
| `contract.lifecycle.transition` | `handbook-contracts` | `compare_and_write` / compare required | append one `semantic_record` transition against exact current lifecycle fingerprint |
| `contract.evidence.list` | `handbook-contracts` | `read_only` / `safe` | none / empty |
| `contract.evidence.read` | `handbook-contracts` | `read_only` / `safe` | none / empty |
| `contract.evidence.append` | `handbook-contracts` | `append_only` / key required | append exactly one validated `observation_evidence` record from one exact dock execution/candidate binding |
| `contract.verdict.evaluate` | `handbook-contracts` | `read_only` / `safe` | none / empty; returns deterministic claim verdicts without persistence |
| `contract.gate.evaluate` | `handbook-contracts` | `read_only` / `safe` | none / empty; deterministic recomposition over exact records |
| `dock.manifest.list` | `handbook-contracts` | `read_only` / `safe` | none / empty |
| `dock.manifest.read` | `handbook-contracts` | `read_only` / `safe` | none / empty |
| `dock.run` | `handbook-contracts` semantic owner composed with a separable process executor | `append_only` / key required | append exactly one `operational_state` `DockExecutionRecord` for an admitted run; no canonical evidence/verdict/gate/contract mutation |

`dock.run` has one HCM-0.4-legal exactly-one write-set item and receipt for every admitted run. Pre-admission request/manifest refusal writes nothing. The exact execution record retains the closed outcome and fingerprinted candidate bundle, but candidates are untrusted operational data rather than canonical evidence. Each later `contract.evidence.append` validates one exact candidate and appends exactly one evidence record/receipt; a rejected candidate writes nothing. This split is crash-resumable and prevents an executor from acquiring evidence authority. Refused, failed, timed-out, cancelled, protocol-error, and candidate-invalid paths never receive evidence receipts. Verdict/gate persistence is deferred until a future approved operation explicitly defines its authority and write set; read-only evaluation is sufficient for the first proof path.

All operations target `rust_sdk`, `cli_json`, and `tauri` under HCM-0.4. CLI paths and Tauri command names remain adapter decisions. No existing HCM-0.4 operation or schema is changed.

## Negative and refusal cases

The freeze and proof wall must cover at least:

- stale/unsupported contract, claim, manifest, implementation bundle/runtime closure, schema, matcher, Resolution, evidence, or source fingerprints;
- invalid lifecycle transition, stale current-state basis, self-lock, evaluation of draft/review-ready/closed contract;
- claim selector false versus indeterminate; claim missing from every partition; duplicate/multi-verdict claim;
- evidence for wrong contract/claim/subject/case/run, stale evidence, one missing kind from a multi-kind all-of requirement, per-kind surplus/shortage, insufficient Resolution, excluded/hidden source, unsupported kind, cross-kind substitution, contradictory repeated evidence;
- score above threshold with one hard failure; required `not_observed`; flaky hard/required claim; dock-supplied `not_applicable`;
- manifest capability overclaim, unsupported major, unknown required extension, executable/package substitution, checksum mismatch, stale host mapping, unbound interpreter/runtime closure, widened resource/Resolution request, or any network value other than `denied`;
- process spawn through shell/PATH, inherited secret/environment, arbitrary repo path, symlink escape, any network/socket/proxy/DNS attempt, undeclared output, output quota excess;
- duplicate JSON member, extra JSON document/trailing bytes, invalid UTF-8, stdout prose/ANSI, stderr-only result, schema/fingerprint mismatch;
- pre-spawn refusal, dock refusal, timeout, cancellation before/during result, crash, signal, nonzero exit, cleanup uncertainty;
- partial evidence on any non-completed outcome;
- future Rust-native adapter attempting to change semantic evidence shape or authority.

## Affected control-pack sections

| File | Required HCM-0.5 change |
|---|---|
| `00-README.md` | author the final intended HCM-0.5 status during Tasks 3-5, include those bytes in the complete proof wall and every Design Review manifest, and make them effective only through the clean reviewed design commit; no post-`CLEAN` status-byte edit |
| `01-target-architecture.md` | tighten dock posture, validator witness boundary, process-first/future-Rust semantic parity, and non-goals |
| `02-semantic-model.md` | add contract/evidence/Resolution/applicability semantics and adapter implications without changing HCM-0.3 definitions |
| `03-seam-crosswalk.md` | record frozen target semantics/selected first proof target while both runtime seams stay `TargetOnly` |
| `04-phase-slice-map.md` | expand HCM-0.5 dependency/output/exit/non-goal contract; do not begin HCM-0.6 |
| `05-contracts-schemas-and-gates.md` | replace only preliminary HCM-0.5 sections with exact field/rule/state/protocol tables and append HCM-0.5 ordinary operation definitions |
| `06-proof-and-regression-ledger.md` | add HCM-0.5 freeze proof gate and regression rules; keep PG-CONTRACT/DOCK/GATE open |

## Proof wall

1. Parse every new JSON/YAML fenced example with duplicate-key rejection where applicable.
2. Mechanically assert that prose/table/adjacency-list lifecycle edges match exactly, draft records are immutable, and every unlisted transition, stale basis, self-lock, and evaluation-state conflation refuses.
3. Assert the exhaustive gate-effect x verdict matrix, complete claim/evidence/verdict/gate partitioning, required/hard `warning` refusal, and hard-fail/required-not-observed/score precedence.
4. Assert all-of per-kind evidence requirements, freshness, provenance, subject/case/source bindings, per-tuple cardinality/consistency/precedence, score-weight omission/invalid values, and six-dimension Resolution rules.
5. Assert manifest/request/result identity, complete implementation bundle/runtime closure binding, pre-spawn substitution refusal, capability intersection, one-document framing, typed refusal, and no partial evidence.
6. Assert default-deny isolation, safe refs/outputs, timeout/cancellation/process-tree behavior, no-network default, and failure matrix.
7. Assert JSON Schema dock selection criteria and bounded responsibility are present and PG-DOCK-01 remains open.
8. Assert HCM-0.5 operation additions conform to frozen HCM-0.4 owner/mutability/idempotency/write-set/transport rules, including exactly-one `dock.run` and `contract.evidence.append` receipts across pre-admission, completed-zero-candidate, completed-candidate, refused, failed, and invalid-candidate cases.
9. Regression-assert HCM-0.2/0.3/0.4 frozen sections and canonical `05` monolith; assert no contract-catalog leaf files.
10. Check Markdown relative links, anchors, fences, and archive boundary.
11. Run all three handoff validator modes.
12. Run `git diff --check`, scoped diff inspection, no absolute durable paths, no Rust/Cargo changes, no HCM-0.6 changes, and repository-required GitNexus change detection.
13. In the later design-freeze execution session, obtain Design Review 1 over the complete final subject and proof evidence; use Design Review 2 only after the first permitted design remediation and Design Review 3 only after the second. Every Design Review manifest includes the final intended `00-README.md` bytes, and byte-identical staging/commit verification requires that file's hash to equal the clean review manifest entry.
14. Mechanically assert the stage-scoped state machines: historical Planning Reviews 1-4 remain unchanged; Continuation Planning Review 1 may lead to at most one remediation and Continuation Planning Review 2; the future design stage permits at most Design Reviews 1-3 and two remediations; every stage stops immediately on `CLEAN`; every non-clean terminal review transitions byte-identically to its non-completion handoff; no continuation review is named `Review 5`; and no Design Review 4 exists.

## Non-goals

- Rust or any runtime implementation;
- a universal validator, validator DSL, plugin marketplace, third-party adapter marketplace, or remote registry;
- a running dock, process supervisor, Tauri adapter, Substrate integration, CLI command, SDK crate/API, or published schema;
- HCM-0.6 research/default selection or HCM-0.7 implementation-program approval;
- HCM-0.9 repair/resume, catalog decomposition, leaf files, compatibility index, or routing engine;
- changing HCM-0.2 semantic identities, HCM-0.3 Resolution/Snapshot/Projection contracts, or HCM-0.4 owner/DTO/transport/bridge/publication contracts;
- waiver semantics, automatic contract mutation, auto-promotion, model synthesis, or executable predicates in claims/schemas;
- claiming PG-CONTRACT-01, PG-DOCK-01, or PG-GATE-01 passed from documentation.

## Stage-scoped review authority and honest stop behavior

Review identity is stage-scoped. There is no shared counter across planning continuation and design execution:

| Stage | Review identities | Remediation authority | Terminal rule |
|---|---|---|---|
| historical planning | Planning Reviews 1-4 and Remediations 1-3 | exhausted; immutable evidence only | preserve all four dispatches and reviewed fingerprints unchanged |
| current planning continuation | Continuation Planning Review 1; Continuation Planning Review 2 only after one permitted remediation | maximum one bounded planning remediation | stop on `CLEAN`; stop after Continuation Planning Review 2; never call either invocation `Review 5` |
| future canonical design freeze | Design Review 1; Design Review 2 only after design Remediation 1; Design Review 3 only after design Remediation 2 | maximum two design remediations | stop on `CLEAN`; after non-clean Design Review 3, create an honest non-completion handoff; no Design Review 4 |

Every review uses a fresh isolated built-in `default` reviewer with `using-agent-skills` and `code-review-and-quality`; every post-remediation review uses a different fresh identity. A `CLEAN` result ends that stage's review loop immediately. Review budgets are maxima, not targets.

The current planning continuation may change only this three-file subject and additive continuation-review dispatches. After correcting HCM-0.5-R4-001 and HCM-0.5-R4-002, submit the complete subject to Continuation Planning Review 1. If it reports only bounded actionable planning defects classified `local_remediation` and every correction stays inside the authorized three-file subject, permit one remediation and submit the new complete subject to Continuation Planning Review 2. Any other non-clean Continuation Planning Review 1 result is terminal. Every non-clean Continuation Planning Review 2 result is terminal.

A non-clean terminal review permits no local correction. The subject manifest and aggregate fingerprint recorded by its non-completion handoff must equal the terminal review's manifest and fingerprint exactly; no subject byte may change between review and handoff. Budget exhaustion is reported as an honest partial/escalation boundary and never as automatic abandonment.

The review transition set is closed:

| Prior state | Event | Only allowed next state |
|---|---|---|
| human-authorized continuation + R4-001/R4-002 repair | Continuation Planning Review 1 `CLEAN` | validate byte-identical subject, commit approved planning continuation, then partial/context-boundary handoff authorizing a fresh design-freeze session |
| human-authorized continuation + R4-001/R4-002 repair | Continuation Planning Review 1 bounded local planning findings only | one planning remediation, full verification, then different-fresh Continuation Planning Review 2 |
| human-authorized continuation + R4-001/R4-002 repair | Continuation Planning Review 1 any other non-clean result | byte-identical non-completion handoff; no canonical edit |
| Continuation Planning Review 1 bounded findings remediated | Continuation Planning Review 2 `CLEAN` | validate byte-identical subject, commit approved planning continuation, then partial/context-boundary handoff authorizing a fresh design-freeze session |
| Continuation Planning Review 1 bounded findings remediated | Continuation Planning Review 2 any non-clean result | byte-identical non-completion handoff; no further remediation or canonical edit |
| approved planning commit + fresh design-freeze session + complete proof wall | Design Review 1 `CLEAN` | byte-identical proof replay/staging and completion commits |
| approved planning commit + fresh design-freeze session + complete proof wall | Design Review 1 findings | design Remediation 1, full proof wall, then different-fresh Design Review 2 |
| Design Review 1 findings remediated + complete proof wall | Design Review 2 `CLEAN` | byte-identical proof replay/staging and completion commits |
| Design Review 1 findings remediated + complete proof wall | Design Review 2 findings | design Remediation 2, full proof wall, then different-fresh Design Review 3 |
| Design Review 2 findings remediated + complete proof wall | Design Review 3 `CLEAN` | byte-identical proof replay/staging and completion commits |
| Design Review 2 findings remediated + complete proof wall | Design Review 3 any non-clean result | byte-identical non-completion handoff; no Design Review 4 |

Stop with `human_input` if first-proof-dock selection or another decision needs product authority absent from the live pack/evidence. Use `authority_boundary`, `external_blocker`, `context_boundary`, or `capability_unavailable` only when that is the true reason.

## Closeout

This planning-continuation session never edits canonical `00`-`06`. After a clean Continuation Planning Review, run complete packet validation and GitNexus change detection, commit the exact reviewed planning subject plus its additive continuation-review dispatches, then create a `partial` / `context_boundary` v1.2 parent handoff. That handoff records the approved planning commit, historical Planning Reviews 1-4, Continuation Planning Review 1 and optional Continuation Planning Review 2, the reserved Design Review 1-3/two-remediation budget, the exact execution packet path, and the prohibition on treating any planning review as final-design review. Rebuild/validate the ledger and commit only the mechanical handoff/ledger closeout separately.

The fresh design-freeze execution session may use the completed-design path only after canonical design bytes, the complete proof wall, and a `CLEAN` Design Review 1, 2, or 3 over the complete final-design subject. It commits the reviewed packet/control-pack subject first, then creates one completed v1.2 handoff bound to that primary commit and clean manifest and commits the mechanical closeout separately. A non-clean terminal continuation or design review uses the byte-identical non-completion path instead. Do not start HCM-0.6.
