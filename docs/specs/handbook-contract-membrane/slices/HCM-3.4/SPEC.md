# HCM-3.4 Snapshot Memory and deterministic delta engine — planning selector and specification

**Status:** planning-only selector candidate. This packet authorizes documentation and planning artifacts only. It does not authorize Rust, tests, fixtures, schemas, public APIs, dependencies, runtime configuration, or Snapshot Memory implementation.

## Objective

Select and describe the future HCM-3.4 Snapshot Memory boundary so a later explicitly authorized implementation increment can build it without inventing meaning, widening authority, or exposing sensitive state. The eventual capability captures policy-selected state as immutable normalized ContextMemorySnapshot records, derives deterministic compatible SnapshotDelta artifacts, and supplies Resolution-aware grounding only through the existing generic Projection contract.

This planning increment finishes when this selector, specification, plan, task ledger, and planning proof are independently review-clean and have a v1.4 two-commit local-only closeout. It deliberately stops before implementation.

## Authority, dependencies, and predecessor

The HCM-3.4 row in 04-phase-slice-map.md, the Snapshot Memory sections in 01-target-architecture.md, 02-semantic-model.md, and 05-contracts-schemas-and-gates.md, and the open PG-SNAP-01 through PG-SNAP-06 rows in 06-proof-and-regression-ledger.md define the target.

The completed HCM-3.3 handoff at docs/specs/handbook-contract-membrane/handoffs/records/20260805T230330Z--HCM-3-3--orchestration--protected-index-digest-contradiction.json is the immediate predecessor context. It is correct predecessor evidence because it closes the selected private/internal deterministic Projection increment, preserves the generic exact-source/fingerprint/omission/non-authority boundary that HCM-3.5 will use for snapshot grounding, and is the immediately preceding completed Phase-3 slice. It is not resumed or amended, and it grants no automatic continuation.

HCM-3.2 remains the completed Context Resolution prerequisite beneath both slices. HCM-3.4 may plan references to exact Resolution envelope pairs, but it does not change the completed HCM-3.2 kernel. The present operator grant supersedes the control pack's general HCM-3.4 deferral only for this documentation-and-planning outcome; it grants no product implementation.

## Frozen planning decision

The future implementation is one deterministic, policy-defined observation and comparison capability. It must preserve these decisions:

1. SnapshotCapturePolicy is an exact versioned definition. It binds allowed memory horizons and triggers; exact source adapters and multi-source composite rules; static bounded windows; comparison, drift, predecessor, redaction, retention, and consistency policies; and its deterministic closure fingerprint. Live revisions and cursors are capture inputs, never mutable policy fields.
2. ContextMemorySnapshot is an immutable provenance-bearing record of policy-selected Git, Handbook, work, session, and evidence state. Each selected family is observed exactly once with adapter identity, pre/captured/post revision values, normalized payload fingerprint, and applicable per-slot/bound evidence. Each policy-selected family is observed or represented once by a typed exclusion; absence is never unchanged state.
3. Consistency is derived, not caller asserted. An all-observed stable capture is stable; a complete policy-compliant mixture of stable and bounded families is bounded; any unstable family or whole-family exclusion yields diagnostic-only unstable or an explicit policy refusal. Unstable records cannot ground closeout, promotion, a hard gate, or a stable delta.
4. State identity and record identity remain separate. The state fingerprint covers normalized selected state and semantic capture inputs while excluding boundary identity and volatile capture metadata. The record fingerprint covers the complete immutable record except itself. Canonical ordering is defined for maps, paths, semantic sets, windows, evidence refs, and rule evaluations.
5. SnapshotDelta is a deterministic derived record over ordered compatible stable/bounded snapshots. It compares or type-excludes every selected family exactly once, records normalized stable-key changes, evaluates every bound drift rule exactly once in catalog order, and maps every matched rule bijectively to one typed signal. It refuses incompatible, unstable, incomplete, stale, duplicate, or uncataloged input rather than emitting an empty or green delta.
6. Drift signals are deterministic classification, not model interpretation. The permitted vocabulary is expected_progress, justified_divergence, unexplained_drift, scope_expansion, execution_inefficiency_signal, planning_inaccuracy_signal, proof_drift, semantic_drift, and stale_handoff. A justified divergence requires a durable rule-admitted justification reference; prose cannot reclassify a signal.
7. Capture redaction is fail-closed. V1 preserves unmatched_action: omit, explicit secret/environment/secret-file/command/diff deny floors, exact original-pointer subtree coverage, and refusal for incomparable overlapping non-omit actions. Retention is selected by the complete memory-horizon/trigger/record-class tuple. Content-addressed payload deduplication and reviewed compaction never rewrite record bytes, merge record identity, or remove held, referenced, or unexpired records.
8. The paired workflow is policy-defined: a prior eligible top-level end snapshot links to a new session-start snapshot in the same boundary stream, then an ordered compatible delta exposes stale handoffs and drift before later work acts. Unique increasing boundary sequences and immediate acyclic predecessor selection fail closed. HCM-3.5 alone owns adoption by handoffs, packets, and pipeline consumers.

## API and data-model boundary

The names below are future private conceptual model boundaries, not selected Rust symbols, public DTOs, JSON schemas, or implementation paths:

| Future boundary | Owns | Must not own |
|---|---|---|
| SnapshotCapturePolicy | selection, static window, consistency, comparison, drift, predecessor, redaction, and retention closure | ambient discovery, invocation-selected authority, model classification |
| ContextMemorySnapshot | immutable normalized family observations, consistency/admissibility, previous-link, state and record fingerprints | contract/artifact truth, handoff intent, claim verdicts, queue mutation |
| SnapshotDelta | compatible ordered comparison, normalized changes, catalog-complete drift evaluations and signals | source mutation, free-form causal explanation, policy replacement |
| Redaction disposition and retention policy | disclosure/storage outcome and immutable optimization bounds | invocation override, secret retention, record rewrite/deletion under holds |
| Generic Projection integration | HCM-3.4 private proof through the existing Resolution-aware generic Projection contract, using exact snapshot/delta sources, ordinary complete accounting, and authority_effect: none | a second snapshot-specific projection model, consumer adoption, or comprehensive disclosure |

The future owner split remains: engine-side pure definitions, normalization, validation, fingerprinting, capture consistency, and delta computation; eventual SDK capture orchestration; a private HCM-3.4 integration proof through the existing generic Projection; and HCM-3.5 flow/packet/pipeline grounding adoption. A later selector must use fresh impact analysis to name exact code paths and must stop on public API, dependency, schema, or owner-boundary expansion not explicitly authorized there.

## Determinism, normalization, and privacy rules

- Every reference is an exact ref/fingerprint pair. No latest, range, ambient adapter, source-order, wall-clock, random, or model-derived fallback participates in a semantic decision.
- Family composite revisions cover every declared source slot in stable order. Static windows bind their live source revision and cursor at capture, while reusable policy identity stays unchanged.
- Snapshot and delta IDs are unique record identities, not state identity. Input order matters for deltas; reversing endpoints changes the result.
- Family observations, exclusions, changes, catalog evaluations, signals, dispositions, and manifest-like collections have complete deterministic ordering and cardinality rules.
- Secrets, credentials, unrestricted environment values, secret files, raw command arguments/output, and unrestricted full diffs are excluded by default. Large diffs remain fingerprinted or separately referenced only when policy allows.
- A snapshot is descriptive evidence only. It cannot promote itself into canonical artifact, contract, posture, gate, or handoff authority.

## Integration and compatibility posture

HCM-3.4 provides a capability boundary for later integration; it does not adopt it in existing runtime paths. Its implementation proof must exercise only the existing generic Projection contract with exact snapshot/delta sources and typed disclosures or omissions: all selected families must be accounted for, currentness must use captured revisions, retained-pointer handling must not read hidden data, stale or insufficient inputs must refuse, and authority_effect remains none. That private proof creates no Handoff, packet, pipeline, or consumer adoption. The existing nullable handoff snapshot_refs posture remains honest as not_available until snapshot capture actually lands and a separately authorized adoption slice supplies real refs. HCM-3.5 alone may update handoff, packet, and pipeline grounding. HCM-3.6 may consume signals as evidence for advisory posture recommendations, never automatic policy mutation.

No legacy migration, dual-readable truth, public compatibility promise, transport surface, default catalog, or schema version is selected here. A future implementation may add an additive internal representation only after its own selected compatibility/migration decision and proof. This plan claims no existing persisted data requires conversion.

## Private Projection source-pair planning amendment

The immutable HCM-3.4 true stop
`20260806T064500Z--HCM-3-4--orchestration--snapshot-memory-projection-cardinality-stop`
established one narrow missing capability: the private generic Projection
implementation can bind a vector of exact sources, but each source currently
has only one captured-revision tuple and no declarative source-pair relation.
It therefore cannot prove the already-required `PG-SNAP-04` source pair and
five-family currentness closure. The true stop, its blocked implementation
state, and every earlier planning/implementation review remain immutable
evidence; this amendment neither reopens nor waives their causal lineage.

The separately selected source-pair planning outcome may plan only the minimum
private generic Projection capability. Its detailed selector and
implementation-ready work package are
`decision/20260806T174700Z--private-projection-source-pair-planning-selector.md`
and `tasks/plan.md`. It authorizes no Rust, test, fixture, Projection runtime
configuration, consumer, HCM-3.5 adoption, or proof-gate closure.

The planned boundary is deliberately additive:

1. one immutable `snapshot_current` source carries an exact record pair,
   state fingerprint, and a complete canonical captured-family closure for
   `git`, `handbook`, `work` (`work_ledger` and `active_plan` slots),
   `session`, and `evidence`;
2. one immutable `snapshot_delta` source carries its exact delta pair and an
   exact `to_snapshot` dependency that names the selected current record and
   state identity; the delta's own immutable contract establishes its ordered,
   compatible previous-to-current endpoints; and
3. a definition-declared private source-pair requirement validates that one
   selected delta satisfies that exact dependency on one selected current
   source before currentness, disclosure, or payload access.

The future private source grammar must make that representation unambiguous:
its raw source document has exactly one of legacy singleton
`captured_revisions` or canonical multi-family `captured_family_revisions`,
then normalizes either to one family closure. A pair-required current snapshot
also has one exact `state_fingerprint` copied into its private generic source
selection and result provenance; the selected delta's `to_snapshot` dependency
must equal both identities. The five required result currentness records are
one canonical record per family (with work's slot maps nested in its one
record), and adapters remain exact ref/fingerprint pairs. These additions stay
private and opt-in: a legacy source selection has no state fingerprint and a
definition without a pair requirement keeps the established one-source shape.

Missing halves, duplicate selectors, substitutions, malformed or incompatible
endpoint bindings, stale family/slot observations, omitted or extra family
tuples, and missing pair metadata must all refuse without a Projection result.
Definitions that declare no source-pair requirement, and legacy one-family
source documents, retain their existing exact binding/currentness behavior.
The planning outcome does not select a runtime configuration or implementation
fixture; a later explicit implementation selector must create its private,
fixed-input test-only integration proof.

## Future implementation proof and regression strategy

A later implementation selector must bind exact code paths, owners, tests, fixtures, source manifests, impact analysis, and platform evidence. At minimum it must prove:

| Gate | Required future evidence |
|---|---|
| PG-SNAP-01 | identical selected stable state/policy replay has byte-identical normalized payload, state fingerprint, ordering, and distinct record identity where boundary metadata differs |
| PG-SNAP-02 | stable, policy-valid bounded, active-plan-only drift, out-of-bound, exclusion, retry, and refusal cases derive the exact admissibility outcome |
| PG-SNAP-03 | immediate valid prior-end/new-start links and compatible deltas detect stale handoff and unexplained drift; self/future/cyclic/skipped/wrong-stream links and unstable/incompatible endpoints refuse |
| PG-SNAP-04 | private HCM-3.4 integration through the existing generic Projection uses exact snapshot/delta source pairs, Resolution minima, typed disclosures or omissions, complete all-family accounting, captured-revision checks, retained-pointer handling without hidden reads, stale/insufficient-input refusal, and authority_effect: none; it creates no HCM-3.5 consumer adoption |
| PG-SNAP-05 | secret, environment, secret-file, command, diff, known-unmatched, matcher-failed, unknown/unclassifiable-surface, identical-overlap, omit-overlap, and incompatible-overlap cases prove fail-closed redaction and safe retained-pointer behavior |
| PG-SNAP-06 | catalog-complete rule evaluation, one-to-one signal mapping, durable justification, scope/proof/semantic drift, ordering, and no-free-form-reclassification scenarios replay deterministically |

Every future packet must run positive replay and explicit fail-closed negatives, then a full scoped proof wall, fresh independent review, remediation/closure lineage, and the repository-required change detection. Unit proof alone cannot claim HCM-3.5 adoption, PG-HANDOFF-02, public API, Phase-3 exit, or a model interpretation capability.

## Frozen outcome registry and review cadence

The single integrated planning outcome is hcm-3.4-snapshot-memory-planning; its only packet is HCM-3.4-P1-snapshot-memory-planning-causal-review; its authority reference is the HCM-3.4 planning-selector decision. One complete-subject discovery review, one consolidated remediation for all valid P1/P2 findings, and one different-fresh delta-focused closure review are permitted. At most two immediately causal supplemental cycles may address P1/P2 findings directly caused or unmasked by the preceding repair. No review follows CLEAN, and P3/P4 advisories follow 09-review-finding-inventory.md.

## Explicit non-goals and stop conditions

Do not edit product source, tests, fixtures, schemas, public APIs, dependencies, workflows, or runtime configuration. Do not implement or test Snapshot Memory behavior, run product behavior tests, select HCM-3.5 or later, change completed HCM-3.3 evidence, query or mutate a remote, push, create a PR, or touch protected checkouts.

Stop at human review after planning closeout. A future request for implementation needs a new explicit top-level authorization, an implementation selector, fresh impact analysis before each code-symbol edit, and its own proof/review loop.
