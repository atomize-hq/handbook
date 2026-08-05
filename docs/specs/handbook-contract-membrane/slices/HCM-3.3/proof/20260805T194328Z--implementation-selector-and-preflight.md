# HCM-3.3 deterministic Projection implementation selector and preflight

## Dispatch identity and authority

- Parent orchestration: `20260805T194328Z--HCM-3-3--deterministic-projection-implementation`
- Increment task / host: `019fd36d-248b-7312-9675-93885308ba3d` / `local`
- Meta workflow / thread / host: `handbook-hcm-3-3-selector-plan-20260805` / `019fd2d6-dc58-7562-90a9-37e6bbf711ee` / `local`
- Dispatch nonce: `204e7dc06f5b43fc9131fe9e4b995859`
- Phase / slice / packet: `HCM-3` / `HCM-3.3` / `HCM-3.3-I1-implementation-causal-review`
- Selected handoff: `20260805T185300Z--HCM-3-3--orchestration--pg-proj-01-proof-obligation-remediation-completed`
- Exact base / tree: `5542371afef828fd83282aa3971262e6ad1b62ed` / `999ab75512caba5a4b23c47df931206f01c3f138`
- Dedicated local target: `refs/heads/codex/hcm-3-3-selector-planning-20260805`

The explicit implementation dispatch is the new user authority. The selected
planning and corrective handoffs are immutable context and are not resumed,
superseded, amended, or treated as implementation authority.

## Preflight evidence

The assigned checkout resolves to
`C:/Users/spmcc/.codex/worktrees/9550/handbook`, is detached at the exact base
and tree above, is clean, and owns its own registered worktree entry. The
dedicated local target equals the base and tree, and
`12c203f605c26ce7d1911e4ee60c0de205822000` is an ancestor. The configured
remote is `origin`; its baseline `12c203f605c26ce7d1911e4ee60c0de205822000`
is administrative scheduling evidence and is not queried in local-only mode.

Protected observations were frozen before edits. The product checkout is at
HEAD `1a4b10841d51803432bccd35348962e9e859c3b6`, tree
`8280353ac41a1431a32bee9ba6164226c0c17577`, status SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, and
index SHA-256
`60c00adbea6fce2383b7183129c93de628061a77aa08a174ada84b45b526608e`.
The meta checkout is at HEAD
`12c203f605c26ce7d1911e4ee60c0de205822000`, tree
`d1cca0b342a8f671a4b9cef3f330cab42c0c32ef`, status SHA-256
`4fcd533c178666e5ad60625373598dbc4b199b4c58c0a907db043188c3cd4de9`, and
index SHA-256
`ed3f648a99b3c703e94f26af0416bdbc83210902df2974bbeb515af397353220`.
Its pre-existing untracked orchestration state is user-owned and untouched.

GitNexus was bootstrapped through the repository-authorized CLI path and is
current for the exact base. FTS/BM25 concept search is unavailable and is not
reported GREEN. Exact call-graph impact is available. The existing
`ContextResolutionEnvelope` impl was assessed as LOW risk with zero upstream
dependants, affected processes, or modules. This selector admits only one new
crate-private method on that impl; it changes no existing method or public
surface. The other existing Rust edit is one private module declaration in
`crates/engine/src/lib.rs`.

## Selective context capsule

SLICE / OBJECTIVE: implement the private generic deterministic Projection
engine selected by HCM-3.3 and prove only its selected `PG-PROJ-01` and
`PG-PROJ-02` portions.

ACTIVE PACKET: `docs/specs/handbook-contract-membrane/slices/HCM-3.3/SPEC.md`.

DEPENDENCY / AUTHORIZATION PROOF: the completed HCM-3.2 kernel handoff supplies
immutable Context Resolution precedent; the explicit nonce-bound increment
dispatch authorizes this implementation and no sibling work.

SELECTED HANDOFF / VALIDITY: the exact completed corrective handoff and ledger
entry match and require a fixed source/profile/vocabulary/definition closure
under at least two independently authorized envelope pairs.

ACTIVE RESOLUTION ENVELOPE: private engine, private configuration, unit proof,
and parent-local proof/control artifacts only.

GROUNDING SNAPSHOT / START DELTA: Snapshot Memory is not available; refs remain
null and no snapshot claim is made.

TARGET AUTHORITY BOUNDARY: internal `handbook-engine` semantics with no public
export, dependency/version change, shipped default, adopter, or transport.

CURRENT REPO-TRUTH STATUS: profile descriptors already carry projection
catalog references, and HCM-3.2 supplies exact envelope bindings, but no
capitalized generic Projection engine exists. Existing renderer-derived views
remain distinct and untouched.

MUST-READ PACK SECTIONS: HCM-3.3 `SPEC.md` and `tasks/plan.md`; HCM-3.3 row in
`04`; Projection semantics/collapse/provenance in `02`; Projection definition,
request, result, and field authority in `05`; `PG-PROJ-01` and `PG-PROJ-02` in
`06`; live `07`, `08`, and `09`.

LIVE SOURCE / TESTS / PRECEDENT: `crates/engine/src/lib.rs`,
`definition_identity.rs`, `context_resolution.rs`, profile/catalog data types,
and HCM-3.1/HCM-3.2 exact-binding and canonical-fingerprint tests.

SIBLING SEAMS IN CONTEXT: HCM-3.2 envelopes are consumed without mutation;
HCM-3.4 Snapshot Memory, HCM-3.5 adoption, and HCM-3.6 posture remain outside.

ALLOWED AREAS: the exact owner/configuration/test/proof paths below and the
parent's v1.4 dispatch/handoff/ledger closeout surfaces.

EXPLICIT NON-GOALS: public API, Cargo/dependency/version change, unsafe policy,
shipped profile/schema/registry defaults, renderer/transport/SDK/CLI/pipeline
adoption, source mutation, executable hooks, remote code, model prompts,
synthesis, release, remote publication, or another slice.

APPLICABLE CONTRACTS / PROOF GATES: complete fail-closed definition/request/
result field authority plus the exact selected `PG-PROJ-01` and `PG-PROJ-02`
boundary; no Phase-3 exit.

REQUIRED SKILL CHAIN: using-agent-skills, context engineering, planning/task
breakdown, documentation/ADRs, incremental implementation, test-driven
development, debugging/error recovery on failure, code review/quality, and git
workflow/versioning.

KNOWN CORRECTIONS OR CONFLICTS: the future proof wall must isolate Resolution
as the sole varying input. The repository-local `.agents/skills` path named by
`07` is absent; installed workflow-equivalent skills are used and the absence
is recorded.

KNOWN P3/P4 ADVISORIES: `HCM-RF-0001` and `HCM-RF-0002` do not intersect this
engine boundary.

MAXIMUM PERMITTED CLASSIFICATION / PROOF CHANGE: close only the actually
proved internal-engine portions of `PG-PROJ-01` and `PG-PROJ-02`.

EXIT PROOF: deterministic positive and negative tests, exact multi-envelope
replay/provenance/source-immutability proof, complete workspace wall,
GitNexus scoped and compare-to-main change detection, independent discovery
and closure review, and two-commit v1.4 local closeout.

STOP CONDITIONS: any public/dependency/unsafe/default/adopter/transport scope,
HIGH/CRITICAL unexpected impact, unavailable required GitNexus code-symbol
proof, source-authority promotion, unresolved P1/P2 beyond the causal budget,
target-ref drift, or required remote interaction.

## Frozen integrated outcome and causal budget

The sole integrated outcome is
`hcm-3.3-deterministic-projection-engine-implementation`. Its sole packet is
`HCM-3.3-I1-implementation-causal-review`, and this selector is its authority
reference. The registry is frozen before any review. Planning selector review,
implementation, proof, and final-closeout stages advance monotonically under
one parent/outcome-derived budget. No packet, cycle, subject, or task rename
creates another allowance.

## Exact owner, symbol, configuration, and path envelope

Primary implementation may touch only:

1. `crates/engine/src/lib.rs` — add one private `mod projection;` declaration;
   no `pub use`, public module, version, feature, or unsafe-policy change.
2. `crates/engine/src/context_resolution.rs` — add one crate-private
   `projection_authority_view` method to the existing
   `ContextResolutionEnvelope` impl. It must call the existing private
   currentness guard first, then return an opaque private view containing only
   the exact envelope, resolved-profile, Resolution-stack pairs, active level,
   six dimension values/ranks, and no authority payload or mutation handle.
   No existing method, field, visibility, public export, or behavior changes.
3. `crates/engine/src/projection.rs` — new private production module owning
   definition/profile/source validation, exact binding checks, metadata-only
   disclosure/support evaluation, reveal/derive execution, rule accounting,
   lossiness, provenance, and deterministic fingerprints.
4. `crates/engine/src/projection/tests.rs` — new private unit proof module.
5. `crates/engine/src/projection/fixtures/source.json` — fixed canonical source
   bytes for the replay wall.
6. `crates/engine/src/projection/fixtures/resolved-profile.json` — fixed private
   configured-custom-kind/profile catalog closure.
7. `crates/engine/src/projection/fixtures/vocabulary.json` — fixed vocabulary
   closure.
8. `crates/engine/src/projection/fixtures/definition.json` — fixed declarative
   reveal/derive definition closure.

New private symbols are limited to the minimum typed domain needed by the
contract: exact bindings, Projection operation/currentness/rule/omission/proof
effect/lossiness/error types, validated private profile/definition/source/
request/result types, a fixed built-in metadata-only support evaluator, and one
private deterministic execution entrypoint. The new opaque Resolution view
type may be constructed in production only by the admitted currentness-guarded
`ContextResolutionEnvelope` method. Configuration is data-only JSON parsed by
the private engine; it cannot contain executable hooks, remote references,
prompts, synthesis, or transport rules.

The allowed call path is:

```text
private test or future engine-owned caller
  -> obtain current opaque Projection authority view from ContextResolutionEnvelope
  -> parse and validate exact private profile/definition configuration
  -> bind one exact source and the returned exact Resolution envelope/profile/stack pairs
  -> validate profile/vocabulary/catalog/cardinality/surface/operation/currentness
  -> evaluate six dimensions, upstream redaction, disclosure, and support from metadata
  -> only then parse/read permitted source payload fields
  -> execute reveal or allowlisted deterministic derive
  -> canonicalize output and complete accounting/provenance
  -> compute output/evaluation/result fingerprints with authority_effect none
```

No source-order fallback or alternative production call path is admitted.

## Positive, negative, and future-proof wall

The tests must prove:

- declarative configured custom-kind selection through exact source/profile/
  vocabulary/definition/envelope pairs and the same generic engine;
- reveal and allowlisted acyclic derive replay deterministically;
- fixed source bytes plus fixed profile/vocabulary/definition bytes and
  fingerprints under at least two independently authorized envelope pairs.
  Source/profile/vocabulary/definition pairs and bytes, operation, surface,
  purpose, and the complete currentness mode/basis/family/adapter/slot/value
  closure remain byte-identical; only the exact envelope pair and its six
  authorized dimension ranks may vary. Each envelope executes two identical
  replays with complete per-envelope and cross-envelope accounting;
- cross-envelope comparison traces every changed rule disposition only to the
  definition's six-dimension minima, requires distinct output bytes and
  fingerprints when the changed dispositions change output, explicitly
  accounts for correctly equal output bytes/fingerprints when they do not, and
  always requires distinct deterministic result fingerprints bound to the two
  distinct envelope pairs;
- exact source immutability, output/evaluation/result fingerprints, complete
  provenance, computed lossiness, and `authority_effect: none`;
- zero/multiple source, stale/mismatched pairs, unlisted definition, invalid
  custom kind/configuration, unsupported operation/surface, invalid evaluator,
  and forbidden executable/remote/prompt/synthesis content all refuse closed;
- currentness `none` requires null basis and empty request/result checks.
  Exact currentness is snapshot-selector-only with the fixed
  `captured_revision` family/adapter/slot closure; missing/extra/duplicate or
  substituted tuples, captured-value mismatch, stale adapter, and an equal
  unrelated live revision that does not equal the bound captured value all
  refuse before a result;
- support evaluator substitution, missing/stale/incompatible evaluator or
  schema/pointer/derivation registry closure, forbidden evaluator input,
  deterministic first-unsupported-reason ordering, and evaluator semantic
  drift that fails to change the evaluator/definition/evaluation/result
  fingerprint closure all refuse;
- the Resolution bridge refuses stale authority before producing its opaque
  view, and the engine refuses request/profile/stack/envelope pair mismatches
  before payload access;
- all six dimensions are evaluated for every applicable rule; each rule is
  included, typed-omitted, or operation-mismatch `not_applicable`; omitted
  required/claimed rules are `not_observed` and cannot false-pass;
- protected payload is not parsed when Resolution, upstream redaction,
  disclosure, or support denies every rule;
- lossiness precedence is exactly redacted, then partial, then collapsed, then
  lossless; and
- collapse never widens, while expansion refuses and requires a separately
  broader authorized request or typed Resolution escalation outside the
  successful Projection result.

## Review and closeout envelope

Before code, a fresh isolated `gpt-5.6-sol` / `high` reviewer must return CLEAN
for this selector through a schema-valid v1.4 planning discovery dispatch. The
implementation/proof child and all later discovery/closure reviewers use the
same model/effort with `fork_turns=none`, distinct fresh identities, and durable
parent-handoff evidence. Review cadence follows live `07`/`08`/`09`; no cycle
follows CLEAN, no closure reopens discovery, and mechanical closeout consumes
no review cycle.
