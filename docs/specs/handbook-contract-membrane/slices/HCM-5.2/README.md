# HCM-5.2 Evidence, Verdict, and Gate Engine

This document owns the outcome and packet authority for HCM-5.2. It defines
exactly one slice outcome: immutable canonical evidence admission, closed claim
verdict computation, complete accounting and freshness, deterministic gate
composition, and a separate local-closeout result. It does not implement a
dock protocol or executor, validator proof, parent promotion, posture mutation,
or another Phase-5 slice.

## Current admission state

**HCM-5.2 Packets 1–5 are implemented. The slice remains open; Packets 6–7 are
unselected.**

The hard predecessor is satisfied. The completed
[`HCM-5.1` authority](../HCM-5.1/README.md) establishes the inward
`handbook-contracts` owner, exact contract/claim/applicability/lifecycle
semantics, the active-after-independent-lock evaluation basis, the durable
owner repository, and the exact four-operation outward membrane. Its six
packets and all 16 acceptance conditions are complete; the final integrated
proof and deterministic schema checks are green, its controlling aggregate
review is `APPROVE` with no unresolved P1/P2, and its single slice closeout is
complete through commits `b37a307`, `dd51ed8`, `cfd3d35`, `7925652`,
`3bd2c41`, `149c7fd`, and `cc0588d`.

Further HCM-5.2 product execution remains inadmissible until a fresh packet task
proves its declared predecessor and scope immediately before edits. Packets
1–5 add the owner-native identity/policy kernel, atomic read-only evaluation-run
preparation, the sealed read-only candidate-source normalization boundary, the
retained-root immutable-all canonical-evidence repository, pure closed claim-
verdict evaluator, and pure content-addressed gate plus sibling local-closeout
composition fixed by their rows below. They add no derived-result persistence,
SDK surface, parent-promotion/posture behavior, or executor.

## Outcome

Extend `handbook-contracts` as the single semantic owner that:

1. defines one protocol-neutral but untrusted evidence-candidate boundary and
   admits an exact candidate into one immutable canonical `EvidenceRecordV1`
   only after independent owner-side source resolution and validation;
2. retains canonical evidence append-only with deterministic list/read,
   idempotent replay, concurrency, recovery, and immutable retention;
3. derives one complete `ClaimVerdictV1` for every claim in an exact active
   contract from all evidence bound to one exact evaluation run;
4. enforces applicability, identity, currentness, freshness, Resolution,
   cardinality, consistency, and complete accounting in a fixed closed order;
5. composes a content-addressed `GateResultV1` with exact integer/rational score
   arithmetic only after hard, required, integrity, and accounting checks;
6. derives a sibling `LocalCloseoutResultV1` without persistence or mutation;
   and
7. closes exactly the five current HCM-5.2 reserved ordinary operations through
   the existing SDK/schema/manifest/catalog and generic consumer membrane.

The selected direction is:

```text
handbook-engine current authority / Resolution / currentness observations
    <- handbook-contracts contract + evidence + verdict + gate owner
        <- handbook-sdk outward DTOs and five ordinary-operation mappings
            <- existing generic CLI and Tauri capability transports

future HCM-5.3 executor -> immutable operational record + untrusted candidate
                       -> owner-side candidate source boundary
                       -> handbook-contracts admission -> EvidenceRecordV1

GateResultV1 -> LocalCloseoutResultV1
LocalCloseoutResultV1 -> later parent-promotion policy owner (not HCM-5.2)
```

Arrows point from consumer to dependency. An executor, source provider, SDK,
transport, installed skill, or caller never becomes a semantic owner.

## Current and bounded adopted evidence

Current source and tests remain product truth. This authority is grounded in:

- the current program [`README`](../../README.md),
  [`phase-and-slice map`](../../phase-and-slice-map.md),
  [`implemented baseline`](../../implemented-baseline.md),
  [`architecture context`](../../architecture-context.md), and
  [`seam crosswalk`](../../seam-crosswalk.md);
- the completed [`HCM-5.1` authority](../HCM-5.1/README.md), its owner crate
  under `crates/contracts/`, and its final owner/SDK/generic-consumer tests;
- the current exact 62-row inventory, deterministic 135-root manifest, and
  immutable 23-capability production witness under
  `crates/sdk/src/contract_membrane/` and `crates/sdk/schemas/`; and
- current Engine Context Resolution, currentness, authority-admission, and
  repository-durability precedents.

The archive ref `archive/handbook-contract-membrane-pre-clean-20260812` is
adopted only for these bounded product decisions:

- the `Evidence record`, `Verdict contract`, and `Gate contract` sections of
  `05-contracts-schemas-and-gates.md` for candidate-versus-canonical evidence,
  cardinality, consistency, closed verdicts, fail-closed gates, and exact score
  precedence;
- the purpose-named owner, acyclic dependencies, and dock-witness separation
  in `01-target-architecture.md`, `02-semantic-model.md`, and
  `03-seam-crosswalk.md`; and
- HCM-0.5 decisions D1 and D2 from
  `handoffs/records/20260716T015949Z--HCM-0-5--orchestration--contract-dock-design-frozen.json`:
  `handbook-contracts` owns canonical evidence/verdict/gate truth, and every
  downstream identity revalidates the current active-after-independent-lock
  basis.

No historical status, proof wall, ledger, orchestration record, packet count,
runtime choice, implementation-completion claim, or dock protocol is adopted.
Where the historical design computed local closeout and parent promotion
together, current authority narrows the result: HCM-5.2 computes local closeout
only; parent promotion is a later independent owner and result.

## Selected owner and dependency placement

No new semantic-owner crate is selected. Extend the existing
`handbook-contracts` package at version `0.1.0` and owner-source identity
`handbook.owner.contracts@0.1.0`.

- `handbook-contracts` owns `EvidenceCandidateV1` structural meaning,
  `EvidenceRecordV1`, evaluation-run identity, freshness/stability/gate/local
  policies, claim verdicts, gate composition, local-closeout derivation, and
  the durable canonical-evidence repository.
- `handbook-contracts` may depend inward on narrow owner-neutral Engine
  facilities for exact Context Resolution rank/authority comparison, generic
  source-currentness and committed-authority observations, semantic time,
  repository durability, and one lifetime-sealed credential-backed evidence-
  source admission. Engine gains no contract, lifecycle, evidence, verdict,
  gate, or closeout decision.
- `handbook-sdk` depends outward on `handbook-contracts`, supplies bounded DTOs
  and repository-bound use-case composition, and must not recompute an
  eligibility, verdict, score, gate, or closeout branch.
- Engine, Flow, pipeline, and `handbook-contracts` must not depend on SDK or a
  transport. Flow and pipeline gain no new dependency in this slice.
- The future HCM-5.3 executor may depend on protocol-neutral contracts types
  and supply exact operational records/candidate bytes through the selected
  source boundary. It cannot call a constructor that mints canonical evidence.
- Parent promotion is not owned by `handbook-contracts` in this slice. A later
  promotion/posture owner may consume `LocalCloseoutResultV1` but must apply its
  own exact policy, authority, currentness, and mutation boundary.

This placement preserves the current acyclic graph and keeps contract, dock,
executor, transport, posture, and promotion authority separate.

## Exact identities and canonical bytes

All V1 owner records use closed `deny_unknown_fields` data, normalized bounded
collections, RFC 8785 canonical JSON as the sole fingerprint preimage, and
lowercase `sha256:<64-hex>` fingerprints. Display JSON, checked-in schema bytes,
transport framing, filenames, and adapter correlations are separate identity
domains and never authorize an owner record.

The required independently retrievable schema roots are exactly:

| Type | Exact schema ref |
| --- | --- |
| untrusted candidate | `handbook.contract-evidence-candidate@1.0.0` |
| canonical evidence | `handbook.contract-evidence-record@1.0.0` |
| admission policy | `handbook.contract-evidence-admission-policy@1.0.0` |
| freshness policy | `handbook.contract-evidence-freshness-policy@1.0.0` |
| stability policy | `handbook.contract-evidence-stability-policy@1.0.0` |
| evaluation run | `handbook.contract-evaluation-run@1.0.0` |
| claim verdict | `handbook.contract-claim-verdict@1.0.0` |
| gate policy | `handbook.contract-gate-policy@1.0.0` |
| gate result | `handbook.contract-gate-result@1.0.0` |
| local-closeout policy | `handbook.contract-local-closeout-policy@1.0.0` |
| local-closeout result | `handbook.contract-local-closeout-result@1.0.0` |
| operation problem details | `handbook.contract-evidence-operation-details@1.0.0` |

The slice ships these exact owner-engine policy identities and canonical
checked-in policy bytes:

- `handbook.contract-evidence-admission-policy.closed@1.0.0`;
- `handbook.contract-evidence-stability-policy.outcome-consensus@1.0.0`;
- `handbook.contract-local-closeout-policy.gate-passed@1.0.0`.

Freshness and gate policy **instances** are contract-selected immutable records,
not global defaults. `EvidenceFreshnessPolicyV1` binds a stable policy ID, full
SemVer/ref, positive JSON-safe maximum age, exact-revision requirement, future-
skew rule, schema binding, and fingerprint. `GatePolicyV1` binds a stable policy
ID, full SemVer/ref, the fixed closed-accounting algorithm, optional exact score
threshold, exact local-closeout policy, schema binding, and fingerprint. This
slice implements their schemas, registries, and evaluation semantics but ships
no production freshness/gate instance because a product contract instance is a
non-goal. Exact test-only fixture policies exercise the engine without entering
the production witness or becoming defaults.

Any same-ref changed policy bytes, unregistered or unknown policy, range,
`latest`, compatible substitution, or mismatched schema/policy fingerprint
refuses. A new semantic rule requires a new exact policy version and, where
contract behavior changes, a contract version allowed by HCM-5.1.

### Evaluation-run identity

`ContractEvaluationRunV1` binds the exact contract ref/definition fingerprint,
the complete HCM-5.1 evaluation-lifecycle basis, subject ref/fingerprint, the
ordered duplicate-free exact case set, the definition's complete ordered claim
set, the requested six-dimension Resolution envelope, exact matcher,
freshness/stability/gate/local-policy refs/fingerprints, a bounded run nonce,
and one owner-established semantic-time/currentness observation. The owner
rederives the active-after-independent-lock chain and current head before
returning the run.

```text
evaluation_run_fingerprint = sha256(RFC8785(run fields except run_ref/fingerprint))
evaluation_run_ref = contract_ref + "#evaluation/" + hex(evaluation_run_fingerprint)
```

The nonce distinguishes intentionally separate observations but grants no
authority. A caller-assembled run is untrusted until owner re-admission. The
owner-native read-only `prepare_evaluation_run_v1` use case constructs this
basis after the first current-head recheck; it is not a sixth ordinary
operation. HCM-5.3 must call that owner use case before any future spawn, and
every later use case independently re-admits the complete run object.

### Candidate and evidence identity

`EvidenceCandidateV1` is protocol-neutral untrusted data. It binds the exact
evaluation run, lifecycle basis, contract/definition, subject/case, a non-empty
ordered duplicate-free observed-claim subset, evidence kind, exact fact schema
and bounded fact payload, `satisfied` or `violated` observation, producer and
request/execution record refs/fingerprints, non-empty ordered source
ref/fingerprint/revision/collection-time bindings, the complete disjoint
`observed`/`unobserved`/typed-`excluded` claim partition for its source request,
request and actual six-dimension Resolution envelopes, observed time, and its
own candidate ref/fingerprint. It contains no verdict, gate, applicability,
closeout, promotion, receipt, or lifecycle truth.

The candidate ref is its source-owned content-addressed identity. Merely
matching its schema or fingerprint does not admit it.

`EvidenceRecordV1` contains the same exact semantic observation closure plus the
owner-derived effective Resolution, admission/currentness observations, the
public source-actor/authority/registry-head binding proven by the sealed
admission, exact admission/freshness/stability policies, and the admitted
candidate ref and fingerprint. The credential and sealed witness are never
persisted. Its identity is:

```text
evidence_fingerprint = sha256(RFC8785(record fields except evidence_ref/fingerprint))
evidence_ref = contract_ref + "#evidence/" + hex(evidence_fingerprint)
```

Changed candidate, contract, lifecycle basis, source, subject, case, claim,
fact, producer/execution, Resolution, time, revision, or policy bytes therefore
produce a different evidence identity. An old fingerprint cannot be retained.

### Verdict, gate, and local-closeout identity

Each `ClaimVerdictV1` binds the exact run, current lifecycle basis, definition
and claim, applicability result, evidence-repository snapshot root, all
supporting and disqualifying evidence ref/fingerprint pairs, freshness and
effective-Resolution accounting, the revalidated current evidence-source
authority-registry heads for every considered record, one closed verdict,
ordered typed reasons, and its content-addressed identity:

```text
verdict_ref = evaluation_run_ref + "#verdict/" + claim_id + "/" + hex(fingerprint)
```

`GateResultV1` binds the same run/basis/current evidence root, exact gate
policy, current source-authority registry observations, the complete
definition-order claim-to-verdict partition, exact score inputs/result, ordered
blockers/actions, decision `passed` or `blocked`, and:

```text
gate_ref = evaluation_run_ref + "#gate/" + hex(gate_fingerprint)
```

`LocalCloseoutResultV1` is a sibling record, not a field that conflates gate
and promotion. It binds the exact gate ref/fingerprint, local policy, final
contracts-owned lifecycle-head, source-currentness, and evidence-source
authority-registry observations, decision `eligible` or `blocked`,
blockers/actions, and:

```text
local_closeout_ref = evaluation_run_ref + "#local-closeout/" + hex(fingerprint)
```

None of these derived records is persisted in V1. Re-evaluation against a new
evidence root, source revision, semantic time, or lifecycle head produces new
content-addressed results. No `parent_promotion_eligible` field exists.

## Evidence admission and trust boundary

`contract.evidence.append` accepts an exact candidate ref/fingerprint and raw
idempotency key through the existing sealed SDK bridge; it does not accept a
caller-authored candidate body as trusted input.

1. `handbook-contracts` resolves the exact candidate bytes and immutable
   producer/request/execution observation from an owner-selected
   `EvidenceCandidateSourceV1` boundary.
2. The source also supplies one lifetime-sealed Engine
   `EvidenceSourceAdmissionV1`. Its exact current approval class is
   `contract_evidence_source`; the credential proof binds the retained physical
   repository root, candidate ref/fingerprint, producer ref/fingerprint,
   request/execution ref/fingerprint, authenticated source actor, authority
   ref/fingerprint, committed registry state/head, and owner-established
   semantic time. It is never an SDK DTO or persisted candidate field.
3. The source boundary supplies bytes and operational provenance only. The
   sealed admission proves who attested the exact operational observation, not
   that its fact satisfies a claim. Neither can construct `EvidenceRecordV1`, a
   verdict, a gate, or closeout truth.
4. The owner parses and canonicalizes the bytes, recomputes every ref and
   fingerprint, resolves the exact contract/claim/policies, rederives the
   evaluation run and current lifecycle head, and independently validates
   subject/case/kind/source/currentness/freshness/Resolution/partition closure.
5. Only the owner-private admitted witness may construct `EvidenceRecordV1`.
   Structural validity, process completion, host allowlisting, executor
   selection, a signature, schema validation, or source availability alone is
   insufficient.
6. `handbook-contracts` rederives its own current lifecycle head and rechecks
   the Engine-supplied current source and evidence-source authority-registry
   heads immediately before the append commit. A change invalidates the
   candidate and writes nothing.

HCM-5.2 defines this protocol-neutral source interface, the sealed Engine
admission, and a repository-backed read-only resolver for already committed
candidate observations; it defines no public candidate writer or executor.
HCM-5.3 later selects the dock protocol, operational-record writer,
executor/runtime, and how an admitted host produces those records. Until such a
candidate exists, the ordinary append operation returns typed absence; stale or
unavailable source authority returns refusal/unavailability. It never accepts
caller substitution. The focused and generic integration tests may seed exact
signed source fixtures through test support, allowing the semantic operation,
schemas, dispatch, and consumer path to close without implementing a dock.

## Currentness, freshness, Resolution, and retention

Every admitted exact freshness policy has closed V1 fields: full SemVer identity
and fingerprint, positive JSON-safe `max_observation_age_seconds`,
`require_exact_source_revision` fixed to `true`, and `future_skew_seconds` fixed
to `0`. Every source must still resolve to the admitted exact
ref/fingerprint/revision both at append and at evaluation. `observed_at` must
not be after owner-established evaluation time, and age must not exceed the
policy maximum. Missing time, unknown source, changed revision, clock
indeterminacy, or arithmetic overflow is stale/blocked, never fresh by default.
Every considered record's public evidence-source actor/authority/registry-head
binding must also resolve to the current committed registry at evaluation,
gate, and local-closeout time. Revoked, missing, multiply resolved, stale, or
substituted source authority is ineligible/blocked even when the historical
credential was valid at append.

Effective Resolution is the dimension-by-dimension minimum of the evaluation
request, producer capability ceiling when one exists, and actual observation
envelope. The owner uses the exact registered Engine stack/ranks for all six
dimensions. Unknown stack/level, substituted authority, incomparable values,
missing dimension, or a value below the claim minimum makes the record
ineligible and visible in accounting. Resolution never grants mutation,
closeout, or promotion authority.

Canonical evidence retention is exactly `immutable_all` in V1. HCM-5.2 has no
delete, prune, overwrite, compaction, newest-wins, or retention-expiry mutation.
Stale evidence remains readable history but is ineligible for a current
verdict. List roots and cursors bind the complete immutable selected record set
and retention mode; later retention or archival requires new authority.

## Cardinality, consistency, and accounting

Evaluation never accepts a caller-selected evidence subset. From one stable
repository snapshot root it reads every canonical record whose exact
`evaluation_run_ref` matches, validates the complete physical/logical inventory,
then partitions each record as eligible or disqualified for each definition
claim and required case/kind tuple.

- `exactly_one`: zero eligible records is `not_observed`; one proceeds; more
  than one distinct eligible record is `blocked` and every surplus ref is
  reported.
- `at_least_one`: zero is `not_observed`; one or more proceed to consistency.
- `all_declared_cases`: every exact declared case must have at least one
  eligible record. A missing case is `not_observed`; unknown or duplicate case
  identity is `blocked`.
- All evidence-requirement clauses remain ordered all-of. No cross-kind,
  cross-case, newest-wins, source-preference, or compatible-version
  substitution exists.
- Under
  `handbook.contract-evidence-stability-policy.outcome-consensus@1.0.0`, all
  eligible observations for one `(claim_id, case_id, evidence_kind)` tuple
  agree only when their closed `satisfied`/`violated` observation is identical.
  Mixed outcomes are `flaky`; differing bounded fact detail with the same
  observation remains consistent and all refs stay visible.
- Exact idempotent append replay returns the one existing record and never
  creates a duplicate. Duplicate physical records, duplicate accounting
  membership, duplicate requested claims/cases, or the same ref with changed
  bytes is corruption and blocks; evaluation never silently deduplicates it.

Every definition claim appears exactly once in the verdict set and GateResult.
Unknown claims, records outside the run, malformed stored bytes, missing
inventory members, surplus physical members, or an incomplete claim/verdict
partition blocks the complete evaluation. Disqualified evidence is not a vote
and cannot disappear from accounting.

## Closed evaluation order and verdicts

The owner evaluates in this exact order from one immutable snapshot:

1. resolve the exact definition in `handbook-contracts`, rederive the
   independent-lock/active chain, require current-head equality, re-admit the
   evaluation run and policies, and resolve current committed evidence-source
   authority for every considered record;
2. evaluate each claim's HCM-5.1 applicability exactly once;
3. for applicable claims, resolve all candidate/evidence identities and apply
   contract/run/subject/claim/kind/case/source/currentness/freshness/Resolution
   eligibility;
4. enforce physical/logical inventory and complete accounting;
5. enforce per-clause cardinality;
6. enforce repeated-observation stability;
7. map unanimous `satisfied` or `violated` observation to the claim's gate
   effect; and
8. combine all-of clauses using
   `blocked > flaky > fail/warning > not_observed > pass`.

Each claim receives exactly one verdict:

| Verdict | Exact meaning |
| --- | --- |
| `pass` | Applicable; every all-of clause has sufficient fresh eligible consistent `satisfied` evidence. |
| `fail` | Applicable hard/required claim has sufficient fresh eligible consistent `violated` evidence. |
| `blocked` | Authority, lifecycle, applicability, identity, policy, inventory, accounting, currentness, Resolution, or evaluator integrity prevents a valid observation result. |
| `warning` | Applicable advisory claim has sufficient fresh eligible consistent `violated` evidence; legal only for advisory. |
| `not_observed` | Applicable claim lacks fresh eligible sufficient-cardinality evidence for at least one clause/case. |
| `not_applicable` | The exact registered matcher deterministically proves false before evidence selection. |
| `flaky` | Eligible repeated observations conflict under the exact stability policy. |

Malformed, stale, substituted, duplicate, unknown, or over-limit input cannot
be downgraded to `not_observed` or `warning`. A hard/required `warning`, an
advisory `fail`, dock-supplied applicability, duplicate verdict, or omitted
claim is an evaluator defect and blocks the gate.

## Gate composition and score arithmetic

`contract.verdict.evaluate` and `contract.gate.evaluate` take the complete
bounded `ContractEvaluationRunV1`, rederive its ref/fingerprint and current
basis, and never trust caller-supplied verdict DTOs. `handbook-contracts`
rechecks its lifecycle head plus every source-currentness and evidence-source
authority-registry head after verdict derivation and before result construction.

The closed gate matrix is:

| Gate effect | `pass` | `not_applicable` | `fail` | `blocked` | `warning` | `not_observed` | `flaky` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hard_fail` | continue | continue | block | block | invalid/block | block | block |
| `required` | continue | continue | block | block | invalid/block | block | block |
| `advisory` | continue | continue | invalid/block | score-visible deficit unless a global integrity blocker applies | score-visible concern | score-visible deficit | score-visible instability |

Before score, the gate blocks on any hard/required non-pass other than proven
`not_applicable`, any global lifecycle/currentness/policy/identity/inventory or
accounting defect, any invalid verdict/effect pair, or incomplete partition.
Only a clean subject reaches weighted scoring.

Weights are the HCM-5.1 JSON-safe positive integers. Omitted weights contribute
to neither sum; `not_applicable` is excluded; every applicable `pass` contributes
its weight to both sums; every other applicable weighted verdict contributes
zero to the numerator and its weight to the denominator. Checked `u128` sums
and products are required; overflow blocks.

An optional threshold is a normalized exact rational
`1 <= numerator <= denominator <= 9_007_199_254_740_991`. With a threshold:

```text
passed iff passed_weight * threshold_denominator
       >= total_weight * threshold_numerator
```

There is no float, percentage rounding, tolerance, NaN, infinity, or adapter
default. If no applicable weighted claim exists, the score is `not_computed`;
a declared threshold with zero denominator blocks. Score never overrides a
hard/required, integrity, freshness, accounting, or lifecycle blocker.

The GateResult decision is only `passed` or `blocked` and includes exact ordered
blockers/next actions. Extensions, candidates, manifests, transports, or docks
cannot add waivers.

## Local closeout and parent-promotion boundary

After a `passed` GateResult, the owner independently evaluates
`handbook.contract-local-closeout-policy.gate-passed@1.0.0`. Local closeout is
`eligible` only when the exact gate passed, the complete accounting and score
bases remain current, and a final contracts-owned lifecycle-head,
source-currentness, and evidence-source authority-registry recheck succeeds.
Every missing, revoked, stale, substituted, malformed, unknown, duplicate,
over-limit, or indeterminate input yields `blocked`.

`LocalCloseoutResultV1` is a pure derived observation. It does not close a work
unit, write lifecycle/posture state, issue a receipt, mutate a parent, or grant
promotion. `contract.gate.evaluate` returns `GateResultV1` and
`LocalCloseoutResultV1` as sibling fields in its operation result so the exact
five reserved operations remain sufficient.

Parent promotion remains entirely later. HCM-5.2 defines no parent policy,
promotion verdict, eligibility boolean, operation, receipt, mutation, SDK
method, schema, or transport branch. A later owner must bind the exact local
result and independently recheck its own authority/currentness/policy; local
eligibility alone can never imply promotion.

## Durable append, replay, recovery, and concurrency

Canonical evidence extends the HCM-5.1 physical owner root
`.handbook/state/contracts/v1` under a separate versioned evidence subtree. The
same retained physical repository root used for contract/lifecycle resolution
must be used for candidate resolution and evidence mutation.

- Evidence records are immutable canonical bytes addressed by ref/fingerprint.
- Append is linearized under one repository-root lock, rederives the contracts-
  owned lifecycle head and rechecks source-currentness and evidence-source
  authority-registry heads inside that boundary, writes atomically, and returns
  a receipt only after durable commit.
- The raw idempotency key remains sealed/redacted. Exact key/request replay
  returns the original record/receipt; conflicting key or candidate reuse
  writes nothing.
- A distinct candidate may yield a distinct evidence record even when its
  observation matches another record; cardinality and stability see both.
- Readers/evaluators bind one stable evidence inventory root. A concurrent
  append is wholly before or after that root and can never produce a mixed
  result.
- Startup/recovery validates an exact transaction inventory, canonical bytes,
  refs/fingerprints, idempotency records, and indexes. Missing, extra, partial,
  duplicated, tampered, or ambiguous state fails closed.
- Durability or cleanup uncertainty returns `unavailable` and no false receipt,
  verdict, gate, or closeout truth.

V1 persists no verdict, gate, score, or local-closeout record. Persistence of
derived evaluation results requires a separately authorized write operation.

## Ordinary operations and outward SDK closure

Current source already reserves these five rows in the exact 62-row inventory.
HCM-5.2 changes their owner from deferred Phase 5 to `handbook-contracts` and
closes them at version `1.0.0` without adding or renaming a row:

| Operation ref | Class / idempotency | Exact effect |
| --- | --- | --- |
| `contract.evidence.list@1.0.0` | `read_only` / `safe` | Return one deterministic immutable-all evidence page under an exact query/root/cursor; no write. |
| `contract.evidence.read@1.0.0` | `read_only` / `safe` | Return one exact ref/fingerprint-bound canonical evidence record or typed absence; no write. |
| `contract.evidence.append@1.0.0` | `append_only` / `idempotency_key_required` | Resolve and admit one exact untrusted candidate, append exactly one canonical evidence record, or write nothing. |
| `contract.verdict.evaluate@1.0.0` | `read_only` / `safe` | Recompute the complete ordered claim-verdict set from one exact run/current evidence root; no persistence. |
| `contract.gate.evaluate@1.0.0` | `read_only` / `safe` | Recompute verdicts, one GateResult, and the sibling LocalCloseoutResult; no persistence or promotion. |

Each operation receives mechanically named request/result roots under the
current convention:

```text
handbook.operation.contract-evidence-list-request@1.0.0
handbook.operation.contract-evidence-list-result@1.0.0
handbook.operation.contract-evidence-read-request@1.0.0
handbook.operation.contract-evidence-read-result@1.0.0
handbook.operation.contract-evidence-append-request@1.0.0
handbook.operation.contract-evidence-append-result@1.0.0
handbook.operation.contract-verdict-evaluate-request@1.0.0
handbook.operation.contract-verdict-evaluate-result@1.0.0
handbook.operation.contract-gate-evaluate-request@1.0.0
handbook.operation.contract-gate-evaluate-result@1.0.0
```

Together with the 12 domain/problem roots above, this slice adds exactly 22
independently retrievable roots. On the admitted final state the exact product
truth is therefore **5 HCM-5.2 operations / 62 inventory rows / 28 capabilities
/ 157 schema roots**, with exact `[cli_json, rust_sdk, tauri]` transport evidence
on each HCM-5.2 operation. A packet may use `$defs` for nested shapes but may not
add another root; a genuine independently retrievable root need requires
authority remediation before implementation, not a silent count change.

Every row requires complete definitions, request/result/problem schemas,
independent owner admission, write/idempotency closure, dispatch, exact checked-
in schema bytes, deterministic manifest entries, production-witness evidence,
and generic SDK/CLI/Tauri proof before discovery. SDK DTOs preserve owner
terminal, evidence, replay, receipt, verdict, blocker, score, gate, and local-
closeout truth. The existing generic CLI/Tauri/installed-skill consumers gain no
operation hardcoding or contract-specific branch.

Typed problem causes are exhaustive across malformed, over-limit, missing,
unknown, stale, substituted, duplicate, conflict, ineligible candidate,
insufficient Resolution, repository/candidate-source unavailable, durability
uncertain, and internal-invariant conditions. Read absence is a typed successful
result; admission refusal/conflict writes nothing; unavailable infrastructure or
uncertain durability maps to the shared unavailable/error terminal without a
fabricated semantic result.

## Trust and abuse boundary

Untrusted inputs include candidate/source/subject/fact bytes, refs and claimed
fingerprints, evaluation nonce, requested Resolution, semantic times, repository
bytes, cursors, raw idempotency keys, SDK DTOs, transport framing, and future
executor records. Protected assets are contract/lifecycle authority, source
currentness, exact semantic time, canonical evidence, repository integrity, raw
keys, receipts, verdict/gate/local-closeout truth, and parent mutation authority.

The packets must prove that:

- caller, executor, dock, validator, transport, or source-provider data cannot
  construct canonical evidence or inject a verdict/applicability/gate result;
- changed contract/basis/candidate/source/policy/schema/fact/repository bytes
  cannot retain an old ref/fingerprint or current status;
- cherry-picked evidence, missing/surplus/duplicate/unknown inventory, stale
  source/head, insufficient Resolution, and inconsistent observation cannot
  become green through scoring or omission;
- exact replay is observable and single-record, conflicting key/request reuse
  writes nothing, and concurrent readers see one stable root;
- public DTOs, errors, receipts, logs, fixtures, and schemas reveal no raw key,
  credential, private authority record, host path, unbounded rejected bytes, or
  quarantined source content; and
- bounded candidates, facts, sources, claims, cases, records, pages, diagnostics,
  recovery inventories, weights, and arithmetic prevent allocation/traversal or
  overflow abuse.

## Scope and allowed implementation boundary

Slice implementation may change only:

- `crates/contracts/**` owner-native semantics, policies, evidence repository,
  source port, and focused tests;
- directly necessary narrow read-only Engine exports for generic committed
  authority, source-currentness, Resolution, semantic-time observations, and
  existing generic durability support, plus focused Engine tests; contract
  lifecycle derivation/read remains entirely in `handbook-contracts`;
- directly necessary SDK contract-membrane DTO, operation, admission, dispatch,
  schema-generation, production-witness, consumer, export, and focused-test
  paths under `crates/sdk/**`;
- workspace/directly affected manifests only when a selected dependency requires
  them;
- generic CLI/Tauri regression tests needed to prove unchanged capability-
  driven discovery/schema/invocation; and
- this authority plus directly necessary current baseline, architecture, seam,
  roadmap, and navigation truth after executable state changes.

CLI/Tauri parser or command-specific product changes, installed-skill workflow
changes, Flow/pipeline integration, posture/promotion mutation, and any dock
runtime/executor source are out of scope. If the five operations cannot close
through the existing generic consumers, the affected packet returns `STOPPED`
rather than widening scope.

## Cross-packet invariants

Every packet must preserve:

1. `handbook-contracts` is the sole evidence/verdict/gate/local-closeout owner;
   Engine supplies generic facts and SDK/transports project owner truth.
2. Candidate bytes remain untrusted; only the owner-private admitted witness
   constructs canonical evidence after exact source resolution and validation.
3. Definition, active-after-independent-lock basis, evaluation run, candidate,
   evidence, policies, verdict, gate, and local-closeout identities are exact and
   content addressed; substitution refuses.
4. `handbook-contracts` rederives its current lifecycle head, and current source
   plus evidence-source authority-registry heads are rechecked at run creation
   where applicable, candidate admission, verdict, gate, and local-closeout
   boundaries. Revocation or substitution fails closed downstream.
5. Applicability precedes evidence and is only `applicable`, `not_applicable`, or
   `blocked`; a source/dock cannot decide it.
6. Evaluation uses every record at one exact repository root, complete claim
   accounting, the closed cardinalities, and fixed precedence.
7. Missing eligible evidence is `not_observed`; malformed/stale/substituted/
   duplicate/unknown/over-limit state is refusal or `blocked`, never a pass.
8. Gate decision is only `passed` or `blocked`; exact integer/rational scoring
   cannot override hard, required, integrity, freshness, or accounting truth.
9. LocalCloseoutResult is a separate pure result. Parent promotion and posture
   state are absent and local eligibility implies neither.
10. Evidence storage is immutable-all, append-only, idempotent, recoverable, and
    snapshot-consistent; verdict/gate/closeout remain unpersisted.
11. The five reserved rows retain exact operation identities/classes and close
    through the existing operation/DTO/schema/owner-admission/transport rules.
12. Generic CLI, Tauri, and installed-skill consumers gain no hardcoded HCM-5.2
    identity, command, or workflow branch.
13. HCM-5.3 dock protocol/runtime/executor, HCM-5.4 validator proof, product
    contract instances, publication, and Phase 6 remain absent.

## Ordered packet decomposition

| Packet | Bounded outcome | Hard dependency | Verification and allowed boundary |
| --- | --- | --- | --- |
| **1 — evidence/evaluation identity and policy kernel** | Extend `handbook-contracts` with the exact candidate, evaluation-run, admission/freshness/stability/gate/local-policy, evidence, verdict, gate, and local-closeout owner-native types and canonical identities fixed here, plus the read-only owner-native `prepare_evaluation_run_v1` use case. Add no repository writes or SDK changes. | HCM-5.1 closeout and this authority. | Focused contracts tests prove canonical bytes/refs/fingerprints, bounded run preparation and current-head refusal, policy identities, unknown-field/ref/version substitution refusal, no parent-promotion field, and no SDK dependency. |
| **2 — candidate-source admission boundary** | Add the owner-side exact candidate/source observation port, repository-backed read-only resolver, sealed `contract_evidence_source` Engine admission, owner-private evidence witness, contracts-owned lifecycle plus generic source/authority-currentness/Resolution/freshness rechecks, and candidate-to-evidence normalization. Test support may seed signed fixtures; add no public candidate writer, executor, dock protocol, or canonical-evidence persistence. | Packet 1. | Focused contracts/Engine tests prove caller-body/source/credential substitution cannot mint evidence, every exact binding and registry head is rederived, stale/malformed/missing/unknown/over-limit candidates refuse, and operational attestation grants no verdict/gate authority. |
| **3 — durable canonical-evidence repository** | Extend the retained-root owner repository with immutable-all list/read, idempotent candidate admission append, stable roots/cursors, replay, linearized concurrency, atomic durability, exact inventory recovery, and no derived-result persistence. | Packet 2. | Focused owner tests cover paging/retention, exact read, append/replay/conflict, duplicate/surplus/missing inventory, concurrent append/read snapshots, material crash boundaries, contracts-lifecycle/source-currentness/evidence-source-authority head change in the final pre-commit window with no write or false receipt, raw-key secrecy, and fail-closed recovery. |
| **4 — closed claim-verdict evaluator** | Re-admit one evaluation run, scan one complete evidence root, revalidate current source-authority registry bindings, evaluate applicability, eligibility/currentness/freshness/Resolution, accounting, cardinality, stability, observation mapping, and all-of precedence into one verdict per definition claim. | Packet 3. | Focused contracts tests exhaust every verdict, reason precedence, claim/evidence/case/kind matrices, missing/surplus/duplicate/unknown/corrupt state, revoked/stale/substituted source authority, stale basis/source, exact replay non-duplication, and deterministic re-evaluation. |
| **5 — gate and local-closeout composition** | Compose the complete GateResult, checked integer/rational score, blockers/actions, and sibling LocalCloseoutResult with final contracts-owned lifecycle, source-currentness, and evidence-source authority-registry rechecks. Add no persistence, parent promotion, posture input, or mutation. | Packet 4. | Exhaustive gate-effect/verdict matrix, score boundary/overflow/zero-denominator tests, hard/required precedence, complete partition enforcement, authority revocation and stale-between-stage refusal, local-closeout separation, and absence of promotion fields/behavior. |
| **6 — five-operation SDK/schema/witness closure** | Map owner use cases outward; close the exact five reserved rows, 12 domain/problem plus 10 request/result roots, owner admission, dispatch, deterministic 157-root manifest, 28-capability witness, and exact transports. No adapter-specific route. | Packet 5. | Contracts/SDK tests prove exact definitions/classes/owner/version/source, structural-before-owner refusal, schema regeneration, request/result re-admission, receipts/replay, manifest/catalog/count closure, dependency direction, and unchanged predecessor rows. |
| **7 — generic consumer and integrated slice proof** | Prove all five operations through Rust SDK and existing generic CLI/Tauri consumers; exercise absence, append/refusal/replay, verdict/gate/local-closeout matrices, stale/substituted/corrupt roots, and final deterministic counts; update current product-state/navigation truth only. | Packet 6. | Focused owner/SDK/CLI/Tauri suites, schema regeneration, formatting, workspace check, strict Clippy/rustdoc, dependency/source/link gates, generic-consumer source gates, final integrated proof, and required GitNexus change detection. |

The order is strictly sequential:

```text
HCM-5.1 closeout -> P1 -> P2 -> P3 -> P4 -> P5 -> P6 -> P7
```

Each packet returns control to the meta task. No packet dispatches the next,
claims slice completion, prepares HCM-5.3/HCM-5.4, or performs aggregate
closeout.

## Delegated just-in-time preparation

Separate packet files are optional. Each fresh packet task must reconcile this
authority with current source/tests and its completed predecessor before product
edits.

- Packet 1 may inspect only the bounded evidence/verdict/gate sections and D1/D2
  adopted above. It may choose private modules/helpers/fixture factoring, not
  public identities, policy semantics, arithmetic, or owner placement.
- Packet 2 may additionally use current HCM-5.1 authority-admission and Engine
  currentness/Resolution precedents. It may choose private port/helper names,
  record filenames, and test-fixture mechanics, not the
  `contract_evidence_source` class/bindings, a public candidate writer,
  dock/runtime, caller-trusted candidate body, or source semantic authority.
- Packet 3 may use current HCM-5.1 owner-repository/durability precedents. It may
  choose private record/index/transaction filenames and lock factoring, not the
  retained root, immutable-all policy, replay, snapshot, or recovery semantics.
- Packet 4 may use the bounded historical verdict section and current HCM-5.1
  claim types. It may choose internal evaluator factoring and fixtures, not the
  closed vocabulary/order/cardinality/precedence/accounting rules.
- Packet 5 may use the bounded historical gate section and current gate/posture
  separation. It may choose helper factoring, not gate matrix, score formula,
  local-closeout identity, or parent-promotion exclusion.
- Packet 6 may use only current HCM-4.2/HCM-4.5/HCM-5.1 membrane precedents. It
  may choose internal DTO/schema `$defs` factoring, not root refs/counts,
  operation identities/classes, transports, or generic-consumer rule.
- Packet 7 uses final current product and completed packet evidence only; it may
  choose test/fixture factoring, not widen implementation or historical inputs.

If permitted sources leave more than one consequential answer about source
trust, canonical identity, currentness, evidence accounting, score arithmetic,
local-closeout ownership, persistence, operation closure, or transport
admission, the packet returns `STOPPED` before the affected edit. Private names,
file factoring, or fixture mechanics are recoverable preparation.

## Slice acceptance

HCM-5.2 completes only when:

1. `handbook-contracts` owns all selected semantic types and depends only
   inward; no SDK/transport/executor becomes an alternate authority.
2. Exact canonical identity and policy rules above are implemented with no
   same-ref substitution or identity-domain conflation.
3. Candidate source resolution is independent of caller DTOs and only the
   owner-private admission witness can construct canonical evidence.
4. Evidence append rederives the exact active lifecycle in `handbook-contracts`
   and rechecks source currentness plus current evidence-source authority before
   durable commit; revoked/stale/substituted/malformed input writes nothing.
5. Canonical evidence is immutable-all, list/read are deterministic and bounded,
   append is exactly-once/replay-visible, and recovery/concurrency never produce
   a partial record, duplicate, mixed root, or false receipt.
6. Applicability, evidence eligibility, accounting, cardinality, consistency,
   observation mapping, and all-of combination follow the fixed order.
7. Every exact definition claim receives exactly one legal closed verdict with
   complete supporting/disqualifying accounting and deterministic reasons.
8. Missing evidence is distinguishable from malformed/stale/substituted/
   duplicate/unknown/over-limit state; none can be scored into green.
9. The contracts-owned lifecycle head and generic source-currentness/evidence-
   source authority-registry heads are rechecked at every declared boundary;
   revoked, substituted, or stale-between-stage results fail closed.
10. Gate composition enforces the exact matrix and global blockers before exact
    checked integer/rational scoring; score never overrides hard/required or
    integrity truth.
11. LocalCloseoutResult is a separate content-addressed pure result; no parent-
    promotion/posture field, decision, operation, receipt, or mutation exists.
12. Exactly the five named HCM-5.2 operations have complete owner admission,
    schemas, dispatch, witness, and `[cli_json, rust_sdk, tauri]` evidence.
13. Exact final counts are 5 HCM-5.2 operations, 62 inventory rows, 28
    capabilities, and 157 schema roots; existing rows/capabilities/schemas and
    six fixed use cases remain regression-protected.
14. SDK, generic CLI, generic Tauri, and installed-skill source gates prove no
    HCM-5.2-specific consumer branch and exact terminal/receipt/replay truth.
15. HCM-5.3/HCM-5.4, Flow/pipeline execution, product contract instances,
    promotion/posture mutation, publication, and Phase 6 remain absent.
16. Focused tests, deterministic schema regeneration, formatting, workspace
    check, strict Clippy/rustdoc, dependency/source/Markdown-link gates, and
    required GitNexus change detection pass or are reported honestly.
17. One aggregate verification and focused causal review of the final slice has
    no unresolved P1 or P2.

## Aggregate verification and closeout

Each packet performs only its focused verification and returns control. Packet
completion never closes HCM-5.2.

After Packet 7, the meta task rechecks all 17 acceptance conditions on the final
integrated state, reuses still-valid packet evidence, reruns invalidated or
integration checks, and conducts one focused causal review under the P1-P4
policy. Blocking P1/P2 findings are remediated and reviewed again within current
authority or produce a genuine workflow stop; P3/P4 remain advisory.

Only then may the meta task record one HCM-5.2 slice closeout and separately
evaluate HCM-5.3 admission. That closeout does not promote a parent, exit Phase
5, start a dock, or prove a validator.

## Non-goals

HCM-5.2 does not authorize:

- HCM-5.3 dock manifest/request/result protocol, operational repository,
  runtime, executor, isolation, timeout, cancellation, cleanup, or `dock.*`
  operation closure;
- HCM-5.4 validator implementation, bundled interpreter, JSON Schema proof dock,
  real process execution, or validator fixtures;
- parent-promotion policy/result/eligibility, posture aggregation/policy/
  recommendation/acknowledgement/hysteresis, or any promotion/posture mutation;
- a product contract instance, shipped evidence policy selection, waiver, or
  automatic lifecycle transition;
- Flow/pipeline contract execution or grounding-adoption changes;
- command-specific CLI/Tauri routes, parser changes, or installed-skill workflow
  branches;
- persisted verdict/gate/local-closeout records or a new write operation;
- publication, bridge, Substrate/external adoption, GUI work, or Phase 6;
- restoration of historical ledgers/proof walls/control-plane material; or
- adjacent-slice preparation, Packet 1 dispatch, or product implementation by
  this authority-preparation task.

## Use and drift

Use this document as the current HCM-5.2 outcome and packet authority. Source
and tests remain product truth. If authorized implementation changes an admitted
surface, update the current document that owns that product truth in the same
bounded change.

Do not use this authority to infer HCM-5.2 implementation or completion, begin
HCM-5.3/HCM-5.4, promote a parent, or grant semantic authority to a candidate
producer, executor, SDK, transport, or installed skill.
