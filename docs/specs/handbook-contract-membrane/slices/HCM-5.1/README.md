# HCM-5.1 Contract Lifecycle and Claim Model

This document owns the outcome and packet authority for HCM-5.1. It defines
exactly one slice outcome: a purpose-named contract owner with exact versioned
definitions, immutable authority transitions, typed claims and applicability,
and gate-effect declarations. It does not implement evidence admission,
verdict or gate evaluation, a dock executor, or another Phase-5 slice.

## Current admission state

**HCM-5.1 authority is prepared; no HCM-5.1 packet is implemented by this
document.**

Phase 4 is exited. The completed
[`HCM-4.5` authority](../HCM-4.5/README.md) records Packets 1–6, aggregate
verification and review, and its single slice closeout; the
[`phase-and-slice map`](../../phase-and-slice-map.md) independently records the
Phase-4 exit. Those facts satisfy the hard predecessor for HCM-5.1.

Contract-owner placement is resolved here. Product execution remains
inadmissible until the meta task reloads this authority, confirms the
predecessor evidence is still current, and dispatches Packet 1 separately.

## Outcome

Establish `handbook-contracts` as the semantic owner of contract definition and
lifecycle truth. The owner must:

1. admit immutable exact-version contract definitions and deterministic
   definition fingerprints;
2. model claims, declarative applicability, evidence requirements, and gate
   effects as closed owner-native types;
3. append only authorized lifecycle transitions through a closed graph and
   derive the current lifecycle head from the immutable chain;
4. create one exact active-after-independent-lock admission basis for later
   evaluation without performing evaluation;
5. expose list, read, append, and lifecycle-transition use cases through
   outward SDK mappings and the existing contract membrane; and
6. leave evidence normalization, claim verdicts, gate results, local closeout,
   parent promotion, and dock execution with their later slices.

The selected direction is:

```text
handbook-engine authority and exact semantic primitives
    <- handbook-contracts definition / claim / lifecycle owner
        <- handbook-sdk outward DTO and ordinary-operation mapping
            <- existing generic CLI and Tauri capability transports

separate future dock executor -> untrusted candidate only
                            -> later handbook-contracts evidence admission
```

Arrows point from consumer to dependency. Neither `handbook-contracts` nor a
dock executor may depend on `handbook-sdk`, a transport, or installed-skill
prose.

## Current and adopted evidence

Current source and tests remain product truth. This authority is grounded in:

- the current program [`README`](../../README.md),
  [`phase-and-slice map`](../../phase-and-slice-map.md),
  [`implemented baseline`](../../implemented-baseline.md),
  [`architecture context`](../../architecture-context.md), and
  [`seam crosswalk`](../../seam-crosswalk.md);
- the completed [`HCM-4.2`](../HCM-4.2/README.md) and
  [`HCM-4.5`](../HCM-4.5/README.md) authorities;
- the current workspace and crate manifests;
- exact-ref, fingerprint, structural-verification, operation-admission,
  production-witness, schema-generation, and consumer precedents under
  `crates/sdk/src/contract_membrane/` and focused `crates/sdk/tests/contract_*`
  suites; and
- Engine owner-native admission, committed approver authority, immutable
  lifecycle/persistence, project-condition applicability, grounding, and
  posture precedents under `crates/engine/src/` and their focused tests.

The archive ref `archive/handbook-contract-membrane-pre-clean-20260812` is
adopted only for these bounded product decisions:

- the HCM-0.5 contract identity, lifecycle, claim/applicability,
  active-after-independent-lock basis, and four HCM-5.1 operation semantics
  from `05-contracts-schemas-and-gates.md`;
- the purpose-named owner placement and acyclic dependency direction from
  `01-target-architecture.md`, the contract semantics from
  `02-semantic-model.md`, and the contract/dock separation from
  `03-seam-crosswalk.md`; and
- decisions HCM-0.5-D1 and HCM-0.5-D2 from the named closeout record.

Historical status, proof walls, ledgers, orchestration records, and authority
claims are not adopted. The current product contains no `handbook-contracts`
crate, contract-definition owner, canonical contract evidence, verdict/gate
engine, or dock executor.

## Selected owner and dependency placement

Create the workspace crate at `crates/contracts` with initial package identity
`handbook-contracts` version `0.1.0` and owner-source identity
`handbook.owner.contracts@0.1.0`. It is a semantic owner, peer to Engine, Flow,
and pipeline, not an SDK submodule and not a neutral DTO extraction.

- `handbook-contracts` may depend inward on exact, owner-neutral semantic and
  repository-authority primitives exposed by `handbook-engine`.
- `handbook-sdk` depends outward on `handbook-contracts`, maps owner results
  into the existing bounded consumer membrane, and must not reimplement a
  definition, transition, applicability, or authority decision.
- `handbook-engine`, `handbook-flow`, `handbook-pipeline`, and
  `handbook-contracts` must not depend on `handbook-sdk` or a transport.
- Pipeline gains no `handbook-contracts` dependency in this slice; that edge is
  reserved for later contract execution authority.
- SDK Packet-0 primitives remain at their current consumer-facing placement.
  The owner uses owner-native types and an explicit outward mapping rather than
  depending back on those SDK types.
- A future executor may consume protocol-neutral request types only under its
  own slice. It remains operational and cannot author definitions, lifecycle
  state, applicability, evidence, verdicts, or gate truth.

This is the selected resolution of the former outward-mapping-versus-neutral-
extraction decision. A later extraction requires new explicit authority.

## V1 lifecycle authority policy

HCM-5.1 ships exactly one lifecycle policy identity,
`handbook.contract-lifecycle-policy.independent-lock@1.0.0`. Its checked-in
canonical definition and derived fingerprint bind the lifecycle graph and these
four approval classes:

| Approval class | Authorized use |
| --- | --- |
| `contract_definition_author` | Append one immutable draft and, as the same authenticated author, advance it to `review_ready` or withdraw it. |
| `contract_independent_lock` | Lock or reject a `review_ready` definition; the authenticated lock actor must differ from the definition author. |
| `contract_activation` | Activate one exact immediately prior locked definition after required definition/matcher/policy/currentness closure. |
| `contract_lifecycle` | Deprecate, close, or perform the lifecycle-authority withdrawal branch allowed by the graph. |

The approval-class IDs are semantic policy data. Exact authority refs and
authenticated credential holders are resolved from the current committed
Engine approver registry and its exact state ref/fingerprint; they are not
hardcoded by this document, authored by a contract definition, or supplied as
trusted SDK input. Missing or multiply resolved required mappings block the
operation. A future lifecycle-policy version may select different classes only
under a new exact ref/fingerprint and the versioning rules in this authority.

## Contract definition identity

An admitted `ContractDefinitionV1` is immutable exact-definition data. Its
closed semantic fields are:

- an exact definition-schema ref/fingerprint;
- a stable lowercase dot-separated `contract_id`, full SemVer
  `contract_version`, and mechanically derived
  `contract_ref = contract_id + "@" + contract_version`;
- one closed `contract_kind` discriminator;
- a non-empty, ordered, duplicate-free claim list;
- authenticated `definition_author_ref` plus the exact current authority
  ref/fingerprint under which that author appended the definition;
- exact lifecycle-policy, matcher, gate-policy, and extensions-policy
  bindings; and
- a deterministic lowercase SHA-256 fingerprint over the RFC-8785 canonical
  semantic object excluding only the fingerprint itself.

The owner refuses a range, `latest`, filename, display label, CLI/Tauri command,
dock identity, caller-authored fingerprint, unknown required field, or changed
bytes behind an existing exact ref. A correction creates a new exact version;
even a draft is never edited in place.

Version compatibility is descriptive and never authorizes substitution:

| Change | Minimum version | Boundary |
| --- | --- | --- |
| Presentation-only annotation with identical claim, lifecycle, applicability, evidence, and gate closure | patch | Consumers still pin the exact ref/fingerprint. |
| New advisory claim with no score weight and no membership in a hard/required set | minor | It cannot change existing claim or policy behavior. |
| Any existing-claim change, hard/required addition, selector/applicability/evidence/freshness/Resolution/matcher/lifecycle/gate/schema/subject/case/authority change, or unproved compatibility | major | The old exact version remains immutable history. |

Every admitted definition deterministically derives one draft-genesis
lifecycle fingerprint from its exact definition fingerprint, exact lifecycle
policy, `state: draft`, and `prior_transition_ref: null`. Genesis is a derived
basis, not a separately mutable record or receipt.

## Claim and applicability model

Each contract-local `ContractClaimV1` has this complete semantic shape:

| Field | Rule |
| --- | --- |
| `claim_id` | Stable and unique within the exact ordered definition. |
| `statement` | Immutable normative statement; display prose cannot contradict machine fields. |
| `subject_selector` / `case_selector` | Closed declarative selectors interpreted only by exact registered matcher bindings. |
| `applicability_rule` | Closed declarative condition evaluated once before evidence binding. |
| `gate_effect` | Exactly `hard_fail`, `required`, or `advisory`. This is a declaration, not a gate result. |
| `evidence_requirements` | Non-empty ordered all-of clauses. Each binds one evidence kind, one cardinality, an exact case set or the claim case selector, and an exact stability policy. |
| `freshness_policy` | Exact deterministic source/time/revision currentness binding. |
| `minimum_resolution` | Complete six-dimension minimum Context Resolution envelope. |
| `score_weight` | Omitted or a positive integer from 1 through 9,007,199,254,740,991. Zero, negative, fraction, null, string, or transport default refuses. |

The only evidence cardinalities are `exactly_one`, `at_least_one`, and
`all_declared_cases`. V1 has no any-of group, newest-wins rule, dock-preference
order, cross-kind substitution, or source-order selection.

The integer-only score weight is the exact HCM-5.1 v1 selection from the
historical positive-finite design range. It preserves the current JSON-safe
integer and canonical membrane domain; fractional scoring would require a new
contract-model version and cannot be introduced by an SDK or transport.

Applicability has exactly three typed outcomes:

- `applicable` — the exact matcher and current inputs deterministically prove
  the rule true;
- `not_applicable` — they deterministically prove it false; and
- `blocked` — the rule, matcher, identity, currentness, or required input is
  malformed, stale, unresolved, unavailable, substituted, or indeterminate.

Executable predicates, prompts, remote code, content sniffing, ambient matcher
selection, and dock-supplied applicability refuse. HCM-5.1 may evaluate this
closed applicability result, but it does not bind evidence or create a
`ClaimVerdict` or `GateResult`.

An exact matcher binding is usable only when an owner-native, checked-in
matcher implementation and its input-schema fingerprint are registered in
`handbook-contracts`. Unknown or merely well-formed bindings block. HCM-5.1
does not ship a real product contract or infer a matcher from labels, schemas,
artifact kinds, filenames, or transports.

## Immutable definition lifecycle

Lifecycle describes definition authority only. It never contains claim
evaluation, evidence, verdict, score, gate, dock-run, local-closeout, parent-
promotion, or posture state.

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

The graph is exhaustive. `closed` is terminal; state skipping, rollback,
reactivation, `active -> closed`, and every unlisted edge refuse.

Each append-only `ContractLifecycleTransitionV1` binds the exact definition,
immediately prior transition and lifecycle fingerprint (or the exact derived
genesis basis for the first transition), one allowed from/to edge, authenticated
actor and authority admission, exact transition policy, ordered supporting
evidence, typed reason and semantic decision time, its own deterministic
content-addressed identity, and the resulting lifecycle fingerprint. The
resulting fingerprint transitively closes over the ordered chain without
hashing itself or relying on ambient history serialization.

Transition `supporting_evidence` is authority/decision evidence for admitting
that lifecycle edge. It is an ordered exact ref/fingerprint set and is not a
canonical claim `EvidenceRecord`, claim observation, verdict input, or dock
candidate. Those later evidence semantics remain with HCM-5.2.

Authority is fail closed and owner resolved:

- definition append and lifecycle transition consume a sealed, current,
  credential-backed Engine authority admission; an SDK or transport cannot
  author actor, authority, registry head, or authentication truth;
- the definition author alone may advance `draft -> review_ready`; the author
  or lifecycle authority may withdraw `draft -> closed`;
- `review_ready -> locked` requires a named lock authority whose authenticated
  actor is byte-distinct from the immutable definition author;
- `review_ready -> closed` requires named lock authority;
- `locked -> active` requires activation authority and exact applicability-rule,
  matcher-definition, policy, and currentness closure; it does not evaluate a
  claim against a subject or case; and
- deprecation and closure require lifecycle authority plus exact replacement
  or retention/reference-safety facts required by policy.

The owner rechecks the current committed authority source and the exact current
lifecycle head immediately before each write. Missing authority, stale registry
or lifecycle basis, actor substitution, self-lock, duplicate/conflicting
idempotency reuse, or durability uncertainty produces no false committed
transition.

## Evaluation lifecycle admission basis

HCM-5.1 exposes one immutable, content-addressed
`EvaluationLifecycleAdmissionBasisV1` for later slices. It binds:

- the exact contract ref and definition fingerprint;
- the exact lifecycle policy;
- the exact `review_ready -> locked` transition, its fingerprint, and resulting
  lifecycle fingerprint, with an actor distinct from the definition author;
- the immediately following `locked -> active` transition, its fingerprint,
  and resulting lifecycle fingerprint; and
- a deterministic basis ref/fingerprint over that complete closure.

The lifecycle owner rederives the chain and requires its current authoritative
head to equal the bound active transition. Bare lock, draft, review-ready,
deprecated, closed, stale, substituted, non-adjacent, or self-locked input
refuses basis creation. HCM-5.2 must recheck current-head equality before
evaluation, evidence admission, verdict, and gate composition; HCM-5.1 does not
perform those later actions.

## Ordinary operation and SDK boundary

HCM-5.1 adds exactly four version-`1.0.0` ordinary operations owned and admitted
by `handbook-contracts`:

| Operation | Class / idempotency | Exact effect |
| --- | --- | --- |
| `contract.definition.list` | `read_only` / `safe` | Return one bounded deterministic page under an exact root/cursor/retention contract; no write. |
| `contract.definition.read` | `read_only` / `safe` | Return one exact ref/fingerprint-bound definition or typed absence/refusal; no write. |
| `contract.definition.append` | `append_only` / `idempotency_key_required` | Append exactly one admitted immutable draft definition and return its derived genesis basis. |
| `contract.lifecycle.transition` | `compare_and_write` / `compare_and_write_required` | Append exactly one transition against the exact current lifecycle fingerprint. |

These rows extend, rather than reinterpret, the current operation inventory.
They require exact request/result/problem schemas, independent owner admission,
write-set and idempotency closure, dispatcher coverage, checked-in deterministic
schema bytes, and production-witness evidence before discovery. Once complete,
they expose the existing `rust_sdk`, `cli_json`, and `tauri` transports through
the unchanged generic capability surfaces; no command-specific CLI/Tauri route
or installed-skill branch is added.

SDK DTOs are bounded outward projections. Owner-native definition, claim,
applicability, lifecycle, authority, and persistence types do not move into the
SDK. Consumer re-admission must preserve exact owner terminal, mutation,
receipt, idempotency, and replay truth. Transport names, schema filenames, or
consumer DTO fingerprints never become owner identity.

The minimum new exact schema-root identities are:

- `handbook.contract-definition@1.0.0`;
- `handbook.contract-claim@1.0.0`;
- `handbook.contract-lifecycle-policy@1.0.0`;
- `handbook.contract-lifecycle-transition@1.0.0`;
- `handbook.contract-evaluation-lifecycle-basis@1.0.0`;
- `handbook.contract-applicability-outcome@1.0.0`; and
- request and result roots for each of the four operations, mechanically named
  `handbook.operation.<hyphenated-operation-id>-request@1.0.0` and
  `handbook.operation.<hyphenated-operation-id>-result@1.0.0` under the current
  operation-schema convention.

Nested types may remain definitions within those roots when exact generation
and consumer validation do not require an independently addressable root. A
packet may add another root only when a named request/result binding or
independent manifest retrieval requires it; it must record the exact reason and
update the deterministic manifest/count truth. Existing shared blocker,
refusal, error, invocation-response, receipt, and idempotency roots are reused,
not forked.

## Trust and abuse boundary

The definition body, selector/applicability inputs, transition request,
authentication material, raw idempotency key, expected lifecycle basis,
repository bytes, SDK request, and transport framing are untrusted until their
own boundary admits them. The protected assets are definition and lifecycle
authority, authenticated actor identity, the append-only record chain, raw
credentials/keys, trusted semantic decision time, current-head truth, and
mutation receipts.

The implementation packets must prove these abuse cases fail closed:

- spoofed actor or authority input cannot manufacture the sealed Engine
  admission or self-lock an authored definition;
- changed definition, transition, policy, matcher, schema, or lifecycle bytes
  cannot retain an old ref/fingerprint or current-head status;
- exact replay is observable and idempotent, while conflicting key or expected-
  basis reuse writes nothing;
- public errors, DTOs, receipts, fixtures, logs, and schemas reveal no raw
  credential, idempotency key, private authority record, host path, or
  unbounded rejected content;
- bounded strings, collections, definitions, claims, transitions, list pages,
  source bytes, and recovery work prevent unbounded allocation or traversal;
  and
- an SDK, transport, skill, executor, validator, or allowlist cannot elevate
  structural validity or process completion into owner authority.

## Scope and allowed implementation boundary

Slice implementation may change only:

- the new `crates/contracts/**` owner crate and its focused tests;
- directly necessary Engine exports or a narrow generic authority-admission
  seam, plus focused Engine tests, without moving contract semantics into
  Engine;
- directly necessary SDK contract-membrane DTO, operation, owner-admission,
  dispatch, schema-generation, production-witness, consumer, export, and
  focused-test paths under `crates/sdk/**`;
- workspace and directly affected crate manifests;
- generic CLI/Tauri/install regression tests only when needed to prove the
  unchanged capability-driven consumers admit the four new operations; and
- this authority plus directly necessary current baseline, architecture,
  seam, roadmap, and navigation truth after executable state changes.

CLI/Tauri parser or command-specific product changes, installed-skill workflow
changes, pipeline integration, and Flow changes are out of scope. If one of the
four operations cannot use the existing generic transports without a new
consumer branch, the affected packet returns `STOPPED` rather than widening
scope.

## Cross-packet invariants

Every packet must preserve these invariants:

1. `handbook-contracts` owns definition, claim, applicability, lifecycle, and
   evaluation-basis semantics; Engine supplies generic authority/semantic
   facts, and SDK/transports project owner truth only.
2. Definitions, transitions, and evaluation bases are immutable and exactly
   versioned or content addressed. Same-ref changed bytes, stale bases, and
   compatible-version substitution refuse.
3. Lifecycle state is derived only from the closed append-only transition
   chain and never encodes an evaluation or gate outcome.
4. Definition author/admission authority is fingerprint-bound. Independent
   lock authority is authenticated, current, and actor-distinct from the
   author.
5. Applicability is decided before evidence binding and is only
   `applicable`, `not_applicable`, or `blocked`; a dock cannot decide it.
6. Gate effect is only `hard_fail`, `required`, or `advisory`. HCM-5.1 creates
   no verdict or passed/blocked gate result.
7. Evidence requirements remain ordered all-of clauses with closed
   cardinality; absence, surplus, stale identity, and indeterminacy cannot be
   defaulted into satisfaction.
8. Caller input never supplies trusted actor, authority, semantic decision
   time, current lifecycle head, derived fingerprint, receipt, idempotency
   result, or replay truth.
9. Contract owners and future executors do not depend on SDK/transports;
   executor allowlisting or completion grants no semantic authority.
10. The four new operations preserve the existing operation/DTO/schema,
    structural-versus-owner-trust, transport, receipt, and raw-key boundaries.
11. The current generic CLI, Tauri, and installed-skill consumers gain no
    hardcoded operation identity or contract-specific workflow branch.
12. HCM-5.2, HCM-5.3, HCM-5.4, publication, and external adoption remain
    absent unless their own later authority and proof establish them.

## Ordered packet decomposition

| Packet | Bounded outcome | Dependency | Verification and allowed boundary |
| --- | --- | --- | --- |
| **1 — contract owner and claim kernel** | Add `handbook-contracts` at the selected inward seam. Implement owner-native exact definition identity, version compatibility checks, immutable definition construction, typed claims, closed evidence-requirement/cardinality and gate-effect declarations, declarative applicability outcomes, canonical fingerprints, and mutation matrices. No repository writes or SDK changes. | Phase-4 exit and this authority. | Focused contracts tests prove exact refs/fingerprints, version rules, ordered uniqueness, closed selectors/matchers, applicability tri-state, all-of clauses, numeric bounds, unknown-field refusal, and no SDK dependency. |
| **2 — authority admission and lifecycle chain** | Reuse the current committed Engine authority root through one narrow sealed credential-backed admission seam; implement the exact v1 lifecycle policy and approval classes, draft genesis, exhaustive lifecycle graph, actor/authority rules, append-only transition identities, current-head derivation, and active-after-independent-lock basis. No owner repository or SDK operations yet. | Packet 1. | Focused Engine/contracts tests prove exact policy identity/fingerprint, missing/duplicate/stale/substituted authority refusal, self-lock refusal, every legal/illegal edge, exact prior chaining, definition immutability, and no evaluation fields in lifecycle state. |
| **3 — durable owner use cases** | Add the repository-bound deep owner module for deterministic paged definition list/read, idempotent append, compare-and-write transition, recovery, and replay. Persist only immutable owner records and an exact derived head; use safe repository-relative access, bounded inputs, lock ordering, atomic durability, and total recovery. | Packet 2. | Focused owner tests prove exact page root/cursor/retention behavior, replay, conflicting-key refusal, stale-head refusal, crash recovery at material write boundaries, no partial/duplicate record, definition ordering, current authority/time recheck, and no executor/evidence/verdict/gate state. |
| **4 — SDK membrane and operation closure** | Map the owner use cases outward through SDK DTOs; add the four exact operation rows, requests/results/problems, owner admission and dispatch, deterministic schema roots/manifest entries, production-witness closure, and current transport evidence. Do not add a command-specific adapter route. | Packet 3. | Focused contracts/SDK tests prove schema regeneration, exact owner/version/source evidence, request/result re-admission, pre-owner structural refusal, mutation receipts/replay, no raw-key leakage, inventory/catalog closure, and dependency direction. |
| **5 — generic consumer proof** | Prove all four operations through the existing SDK, generic CLI capability actions, and generic Tauri facade. Exercise read-only, append, compare-and-write, replay, stale/substituted identity, unavailable authority, and lifecycle-basis refusal while proving no new parser, command, or installed-skill branch. | Packet 4. | Focused SDK/CLI/Tauri integration and source gates prove discovery/schema/invocation parity, singular machine framing, unchanged existing operations/use cases, and transport non-authority. Product corrections stay within Packets 1–4 boundaries and invalidate affected evidence. |
| **6 — integrated slice proof and current documentation** | Prove the complete owner-to-SDK path, deterministic final inventory/schema/capability counts, all lifecycle and claim invariants, fail-closed authority/substitution boundaries, unchanged Phase-4 regressions, and update only current implementation/navigation truth. This is integration, not HCM-5.2 or slice closeout. | Packets 1–5. | Run focused owner/SDK/CLI/Tauri suites, schema regeneration, formatting, workspace check, strict Clippy/rustdoc, dependency/source/link gates, and required GitNexus change detection. |

The order is strictly sequential:

```text
Phase-4 exit -> P1 -> P2 -> P3 -> P4 -> P5 -> P6
```

Each packet returns control to the meta task. A packet may not dispatch the
next packet, claim HCM-5.1 or Phase-5 completion, prepare HCM-5.2, or perform
aggregate closeout.

## Delegated just-in-time preparation

The user's packet-by-packet Phase-5 execution request delegates bounded
just-in-time preparation for Packets 1–6 inside the fresh top-level task that
implements each packet. Separate packet files are optional.

Before product edits, each packet must reconcile this authority with live
source/tests, the current program and architecture sources, and completed
predecessor checkpoints. Packet 1 may additionally consult only the bounded
HCM-0.5 definition/claim sections adopted above; Packet 2 the bounded lifecycle
and D2 sections; Packet 3 current Engine durability/recovery precedents; Packet
4 the completed HCM-4.2/HCM-4.5 and bounded HCM-0.5 operation sections; and
Packets 5–6 only the final current product and completed predecessor evidence.

Within its packet, a task may select private module/helper names, internal file
factoring, the bounded repository-relative owner-record layout, fixture
factoring, and derived schema filenames/counts. It may not change the selected
crate owner, dependency direction, four operation identities/classes, exact
definition/version rules, lifecycle graph, independent-lock basis, authority
non-forgeability, applicability/gate-effect vocabulary, generic consumer rule,
or later-slice exclusions.

If permitted sources leave more than one consequential answer about authority
authentication, lifecycle compare basis, durable recovery, public owner/SDK
type ownership, operation write effects, or transport admission, the packet
returns `STOPPED` before the affected edit with the exact missing decision.
Private names, storage filenames, or test fixture mechanics are recoverable
preparation, not a stop.

## Slice acceptance

HCM-5.1 completes only when all of the following hold:

1. `handbook-contracts` exists as the purpose-named semantic owner and depends
   only inward; no owner crate depends on SDK or a transport.
2. Contract definitions use stable lowercase identity plus full SemVer, exact
   refs/fingerprints, immutable normalized bytes, and no automatic compatible
   substitution.
3. Definition fingerprints bind author/admission authority, ordered claims,
   selectors/matchers, lifecycle, applicability/evidence, gate, and extensions
   closure; same-ref changed bytes refuse.
4. Claims are closed, ordered, unique, and typed exactly as specified; evidence
   requirements are non-empty all-of clauses with only the three declared
   cardinalities.
5. Applicability yields only `applicable`, `not_applicable`, or `blocked` from
   exact declarative current inputs. Executable/ambient matchers and dock-
   supplied applicability refuse.
6. Gate effect is exactly `hard_fail`, `required`, or `advisory`; no HCM-5.1
   type or persisted state represents a verdict or gate decision.
7. Draft genesis is deterministic, the transition graph is exhaustive, every
   transition is immutable and immediately prior-bound, and current state is
   derived from the chain.
8. Current credential-backed Engine authority is re-admitted before mutation;
   definition-author substitution, stale authority, self-lock, stale lifecycle
   basis, state skipping, rollback, and reactivation fail before a write.
   The exact v1 policy uses only the four approval classes fixed here, with
   authority refs and holders resolved from one current committed registry.
9. The evaluation lifecycle basis proves one independent lock immediately
   followed by the current active transition. Bare lock, deprecated/closed,
   non-adjacent, stale, or substituted bases refuse.
10. Durable owner use cases provide deterministic bounded page/root/cursor/
    retention list behavior, exact read, exactly-once draft append, compare-and-
    write transition, replay, and crash recovery without partial records or
    false receipts.
11. Exactly the four named HCM-5.1 operations have complete owner admission,
    requests/results/problems, schemas, write/idempotency closure, dispatch,
    production-witness evidence, and current `rust_sdk`, `cli_json`, and
    `tauri` transport evidence.
12. SDK, generic CLI, generic Tauri, and installed-skill source gates prove
    outward mapping and operation-independent consumption with no new
    contract-specific adapter or prompt branch.
13. Invalid, missing, stale, substituted, unknown, over-limit, or noncanonical
    identity/authority/schema/request/current-head input produces no owner
    mutation and no fabricated terminal, receipt, replay, verdict, or gate
    truth.
14. Existing Phase-4 operations, fixed use cases, terminal semantics,
    idempotency/raw-key boundaries, schema identity domains, and generic
    consumer behavior remain regression-protected.
15. Focused tests, deterministic schema regeneration, formatting, workspace
    check, strict Clippy/rustdoc, dependency/source/Markdown-link gates, and
    required GitNexus change detection pass or are reported honestly.
16. One aggregate causal review of the final slice has no unresolved P1 or P2.

## Aggregate verification and closeout

Each packet performs only its focused verification and returns control to the
meta task. Packet review applies when a material packet decision or measured
risk warrants it under the current review policy. Packet completion never
closes the slice.

After Packet 6, and only after every acceptance condition is established on
the final product state, the meta task performs one aggregate verification and
one focused causal review using the P1–P4 rubric. Valid packet evidence may be
reused; later-invalidated evidence must be rerun. Blocking P1/P2 findings are
remediated and reviewed again within current authority or produce a genuine
workflow stop. P3/P4 advisories remain non-blocking.

The meta task then records one HCM-5.1 aggregate closeout and separately
evaluates HCM-5.2 admission. No packet creates a slice closeout, phase-exit
claim, ledger, selector, proof wall, handoff, durable capsule, or mechanical
closeout commit.

## Non-goals

HCM-5.1 does not authorize:

- canonical `EvidenceRecord`, `ClaimVerdict`, or `GateResult` implementation;
- evidence eligibility/cardinality/consistency evaluation, scoring, gate
  composition, local closeout, or parent promotion;
- a dock manifest, request/result protocol, executor, runtime, validator, or
  any HCM-5.3/HCM-5.4 work;
- pipeline or Flow contract execution, posture aggregation or mutation, or a
  gate-to-posture policy;
- command-specific CLI/Tauri routes or installed-skill workflow branches;
- restoration of an archived control plane or a monolithic historical catalog;
- publication, bridge, Substrate/external adoption, Phase 6, or GUI work; or
- adjacent slice preparation, Packet 1 dispatch, or implementation by this
  authority-preparation task.

## Use and drift

Use this document as the current HCM-5.1 outcome and packet authority. Source
and tests remain product truth. If authorized implementation changes the owner
surface, operation inventory, schema/capability witness, or current transport
evidence, update the current document that owns that truth in the same bounded
change.

Do not use this authority to infer HCM-5.1 completion, begin HCM-5.2, or grant a
dock, adapter, SDK, or transport semantic authority.
