# HCM-3.3 Deterministic Projection engine — selector and implementation plan

**Status:** planning-only selector candidate. This packet is the sole HCM-3.3
authority in the current increment; it authorizes no Rust, test, fixture,
schema, profile, registry, public API, dependency, or transport change.

## Objective

Freeze one reviewable implementation decision for the Phase-3 Deterministic
Projection engine. The future engine must derive Resolution-aware views from
exact fingerprinted structured sources without becoming a second editable
authority or synthesizing content. This packet delivers the bounded selector,
implementation sequence, future proof obligations, and review lineage only.

## Dependency and authority boundary

HCM-3.2 is completed by
`docs/specs/handbook-contract-membrane/handoffs/records/20260805T153602Z--HCM-3-2--orchestration--context-resolution-quarantine-finalization-completed.json`.
Its `PG-RES-01` kernel boundary is consumed as immutable context, not resumed
or amended. The authoritative semantic source is the generic Projection
contract in `02-semantic-model.md` and `05-contracts-schemas-and-gates.md`.
The HCM-3.3 row in `04-phase-slice-map.md` supplies sequencing and scope.

This packet is deliberately documentation-and-planning-only. A future
implementation increment must receive a new explicit user authorization and
its own selected implementation packet; it cannot infer authority from this
plan, this review, or the HCM-3.2 handoff.

## Frozen planning decision

The future implementation is one deterministic generic Projection boundary,
not a renderer migration, Snapshot Memory implementation, pipeline adoption,
or user-visible transport feature.

1. A request binds exact paired source, resolved-profile, vocabulary,
   Projection-definition, and Resolution-envelope references and fingerprints.
   The selected definition must belong to that exact resolved-profile catalog,
   and each source selector has v1 `exactly_one` cardinality. A registered
   configured custom kind is selected declaratively through the same exact
   ref/fingerprint-bound configuration and executes through this same engine;
   it gains no hard-coded first-party branch, new core operation, or
   source-order fallback. Missing, ambiguous, stale, incompatible, unregistered,
   or profile-unlisted source/capability/definition state refuses before a
   result.
2. A `ProjectionDefinition` is the only mapping authority. It admits only
   deterministic `reveal` and `derive` operations, exact source selectors,
   target schema, allowed surfaces, complete six-dimension minimum Resolution
   rules, mandatory currentness requirements (`none` is null/empty, or exact
   checking is snapshot-selector-only with the fixed `captured_revision`
   family/adapter/slot closure), one exact fingerprinted metadata-only
   disclosure policy, and one exact versioned built-in metadata-only support
   evaluator with its canonical input allowlist and deterministic
   first-unsupported-reason order. Request expected revisions and result
   observations must equal the bound snapshot captured values, not arbitrary
   or merely equal live values. Evaluator identity and semantic closure enter
   the definition, disclosure-evaluation, and result fingerprints. The
   definition also owns field/claim rules and allowlisted acyclic derivations.
   It admits no executable hook, remote code, model prompt, content-sensitive
   policy, transport-owned semantic rule, or source-order fallback.
3. `reveal` exposes already-bound source fields. `derive` computes an
   allowlisted deterministic aggregate, selection, normalization, or
   presentation from exact inputs. Neither operation changes source authority;
   every result has `authority_effect: none`. Synthesis is explicitly outside
   the engine and remains candidate-only future work.
4. Collapse is a Projection to an equal-or-narrower envelope on every
   dimension. Expansion is never a Projection result: an already-authorized
   broader request or a typed `ResolutionEscalationRequest` is required. The
   engine must never widen its own authority, invent absent source detail, or
   promote an observation, snapshot, delta, or result to canonical truth.
5. Every applicable rule is exactly included or given a typed omission
   (`out_of_resolution`, `redacted`, `unavailable`, or `unsupported`); only an
   operation mismatch is `not_applicable`. Required or claimed omissions retain
   `not_observed` proof effect and cannot false-pass. Result lossiness is
   computed, not requested: `redacted` outranks `partial`, which outranks
   `collapsed`, which outranks `lossless`.
6. Result provenance includes exact source/profile/vocabulary/definition/
   envelope pairs, requester purpose and surface, operation and target schema,
   currentness basis and validation, complete rule accounting, typed omissions
   and proof effects, derivation input/output fingerprints, output/result
   fingerprints, computed lossiness, and `authority_effect: none`.

## Planned owner and non-adoption boundary

The future plan preserves the target split: pure definition, validation,
fingerprint, disclosure, support, rule-accounting, and reveal/derive semantics
belong to the engine-side semantic owner; request-scoped assembly and a typed
Projection result belong to flow. No actual source path, Rust symbol, public
DTO, transport, pipeline, SDK, Snapshot Memory, or renderer is selected by
this planning packet. The exact owner paths and any cross-crate surface are a
future implementation selector decision, after fresh impact analysis.

Existing fixed deterministic renderer-derived views remain outside the
capitalized Projection contract. HCM-3.4 Snapshot Memory, HCM-3.5
snapshot/pipeline adoption, HCM-3.6 posture, HCM-0.11, and all consumer or
transport migration remain separate work.

## Future implementation proof obligations

A separately authorized implementation selector must require at least:

- one `PG-PROJ-01` multi-envelope matrix that starts from one
  byte-identical source truth and holds its exact source ref/fingerprint,
  resolved-profile ref/fingerprint, vocabulary ref/fingerprint,
  `ProjectionDefinition` ref/fingerprint, operation, surface, purpose, and
  currentness closure constant while varying only the exact paired
  ref/fingerprint of at least two independently authorized Context Resolution
  envelopes;
- two byte-identical deterministic replays under each envelope. Within each
  envelope, output bytes/fingerprint, ordered disclosure-evaluation
  fingerprints, complete result fingerprint, and all inclusion, omission,
  `not_applicable`, proof-effect, derivation, and lossiness entries must replay
  exactly;
- an explicit cross-envelope comparison derived from the unchanged definition
  rules. It must identify every envelope-caused rule-disposition difference,
  require distinct output bytes/fingerprints whenever those rules change the
  output, and explicitly justify and account for any equal output
  bytes/fingerprint when the rules leave output unchanged. The result
  fingerprints must remain deterministic and bind their distinct envelope
  pairs; changing any non-envelope source/profile/vocabulary/definition input
  invalidates the case rather than counting as a Resolution projection;
- complete provenance for every replay, including exact source, profile,
  vocabulary, definition, envelope, derivation input/output, output, and result
  fingerprints, plus before/after equality of source bytes and source
  fingerprint and exact `authority_effect: none`;
- deterministic repeated reveal and derive results from identical exact inputs;
- positive replay/provenance for a declaratively configured registered custom
  kind using the same engine, plus refusal for invalid custom configuration;
- refusal for zero/multiple source matches, stale pairs, a fingerprint-valid
  but profile-unlisted definition, unsupported surface or operation, invalid
  capability/disclosure/support/currentness closure, and forbidden
  executable/remote/synthesis definition content;
- `none` currentness null/empty validation and exact snapshot-only
  captured-revision family/adapter/slot tuple checks, including mismatch and an
  equal live revision that cannot green a stale captured source;
- evaluator substitution, missing/stale/incompatible evaluator or registry,
  forbidden evaluator input, deterministic first unsupported reason, and
  evaluator-semantic drift that changes the required fingerprints or refuses;
- typed per-rule inclusion, omission, `not_applicable`, `not_observed`, target
  absence, and fixed lossiness-precedence cases;
- no protected-payload read before the Resolution, upstream-redaction,
  disclosure, and support decisions allow it;
- collapse without authority widening and expansion as a request or escalation,
  never implicit result growth;
- provenance and output/result fingerprint replay with `authority_effect: none`;
- proof of both open gates `PG-PROJ-01` and `PG-PROJ-02` only to the exact
  selected implementation boundary, with no Phase-3 exit or Snapshot claim.

The multi-envelope case is not an expansion operation. Both envelopes must be
valid before their requests are constructed; the engine may neither widen the
narrower request nor manufacture absent detail. The case must preserve the
collapse boundary and must treat any need for broader unauthorized authority
as a separate typed escalation rather than as a successful Projection.

## Frozen outcome registry and review cadence

The only integrated outcome is
`hcm-3.3-deterministic-projection-selector-plan`; the only packet ID is
`HCM-3.3-P1-selector-plan-causal-review`; and its authority reference is this
selector decision:
`docs/specs/handbook-contract-membrane/slices/HCM-3.3/decision/20260805T170317Z--deterministic-projection-selector.md`.

One planning discovery review, one consolidated remediation for every valid
P1/P2, and one different-fresh delta-focused closure review are authorized.
Only a P1/P2 directly caused or unmasked by the immediately preceding repair
may use a supplemental causal cycle, with at most two supplements. No review
may follow CLEAN, and P3/P4 remain non-blocking only when handled under `09`.

## Completion and stop conditions

This planning slice completes only when the selector, this specification, and
the plan/task ledger are independently review-clean, ordinary and self-test
handoff validation succeeds, and the parent-owned two-commit local-only
closeout is complete. Completion does not implement or promote a Projection
engine.

Stop with a true authority boundary for an implementation request, any need to
alter the canonical Projection semantics, HCM-3.2, HCM-3.4+, code/tests,
schemas/templates/tooling, dependencies/public APIs, a consumer/transport,
remote publication, or an unresolved P1/P2 beyond the causal allowance.
