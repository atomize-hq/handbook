# HCM-3.4 private Projection source-pair — implementation-ready planning packet

## Status and authority

This is a planning-only packet for the fresh integrated outcome
`hcm-3.4-private-projection-source-pair-planning` and packet
`HCM-3.4-P2-private-projection-source-pair-planning`. Its only authority is
the frozen selector at
`../decision/20260806T174700Z--private-projection-source-pair-planning-selector.md`.
It consumes, but does not alter, the immutable blocked HCM-3.4 handoff
`20260806T064500Z--HCM-3-4--orchestration--snapshot-memory-projection-cardinality-stop`.

This plan is not an implementation grant. Do not edit Rust, product or test
code, fixtures, generic Projection runtime configuration, public APIs,
schemas, dependencies, workflow/runtime configuration, or a consumer. Do not
claim `PG-SNAP-04`, HCM-3.5 adoption, `PG-HANDOFF-02`, or Phase-3 exit from
this packet.

## Grounded problem statement

The immutable true stop and the live private engine agree on the failure mode:

| Live seam | Current behavior | Why PG-SNAP-04 cannot be proven |
|---|---|---|
| `ProjectionRequest.sources` and `bind_sources` in `crates/engine/src/projection.rs` | Multiple exact sources and exactly-one selector binding already exist. | This is sufficient to name two sources but not to prove their semantic relationship. |
| `SourceDocument.captured_revisions` and `ProjectionCurrentnessRequest::captured` | One source exposes one `{family, adapter, family_revision, slots}` tuple. | The snapshot-grounding definition requires five distinct tuples from one `snapshot_current` source. |
| `validate_request_currentness` | It compares one captured tuple per requirement with an independent live observation. | Reusing the singleton for five required families fails exact family/adapter closure; it cannot validate the complete set. |
| `validate_selected_source_semantics` | It validates each selected source independently. | It does not assert that `snapshot_delta.to_snapshot` is the selected current snapshot. |

The target is therefore a bounded private generic source-pair capability—not a
Snapshot-specific Projection engine, not a consumer bridge, and not a revised
proof obligation. The pre-HCM-3.4 generic Projection baseline at
`d7877f7843afbcda78d65e0fa2bf7093d3c71e6a` remains binding: deterministic
non-authority, exact source binding, independent-live currentness, typed
omissions, no hidden-data read, and fail-closed refusal.

## Frozen pair identity and compatibility contract

The only planned pair is ordered and complete. Neither member can be inferred
from source order, label, timestamp, filename, or a latest lookup.

| Role | Exact identity | Required relation | Refuse when |
|---|---|---|---|
| `snapshot_current` | One `source_kind: snapshot` document; exact record ref/fingerprint and its separate state fingerprint. | Its immutable record is stable or policy-admissible bounded, has the selected policy/schema/adapter closure, and supplies all five captured family tuples. | absent, duplicate, bare-ref, malformed, unstable/excluded, wrong kind/schema/capability, stale record/state identity, or incomplete/duplicate family closure. |
| `snapshot_delta` | One `source_kind: snapshot_delta` document; exact delta ref/fingerprint. | Its immutable endpoint metadata has one `to_snapshot` dependency whose exact record pair and state fingerprint equal `snapshot_current`; its own contract already proves ordered compatible previous-to-current endpoints, same repository/workspace/stream, compatible policy/schema/adapters, and complete comparison coverage. | absent, duplicate, bare-ref, wrong kind/schema/capability, missing/duplicate/malformed dependency, substituted current record or state fingerprint, reversed/incompatible endpoints, stale/uncataloged delta, or incomplete comparison coverage. |

The current source is the sole currentness authority. Its canonical set is
exactly these entries in stable family order, with no duplicate, omitted, or
extra family, adapter, or source-slot identity:

| Family | Selector | Required slots |
|---|---|---|
| `git` | `snapshot_current` | none |
| `handbook` | `snapshot_current` | none |
| `work` | `snapshot_current` | `work_ledger`, `active_plan` |
| `session` | `snapshot_current` | none |
| `evidence` | `snapshot_current` | none |

For each tuple the request value, the source's captured value, and the
independent live observation must all match exactly. A currentness check cannot
be supplied by the delta, a caller-provided replacement, a plan/session-only
observation, or another selector. Since `/signals` is unfiltered delta output,
the five checks include `session`; filtering it would require a separately
declared rule and typed omission, not a silent family subset.

## Minimum private engine/configuration seam

The later implementation must stay inside the private `handbook-engine`
Projection module. It may modify no public export and may not create a shipped
definition, CLI/SDK adapter, packet, Handoff, pipeline, or runtime adoption.

1. In `crates/engine/src/projection.rs`, split decode from the normalized
   private model. `RawSourceDocument` has
   `captured_revisions: Option<CapturedRevision>` and
   `captured_family_revisions: Option<Vec<CapturedRevision>>`; it requires
   exactly one. Legacy singleton source bytes decode through the first field;
   pair-capable source bytes use the non-empty, family-sorted,
   family-unique array. Missing, both-present, empty, unsorted, and duplicate
   forms refuse before payload access. The private normalized `SourceDocument`
   exposes only one canonical captured-family closure. This is compatibility
   preservation, not a second currentness mode.
2. Add one private, definition-declared `source_pair_requirements` collection
   of `SourcePairRequirement { current_selector_id, derived_selector_id,
   dependency_role, require_state_identity }` values. An empty collection is
   the legacy/default behavior. For this packet the sole requirement names
   `snapshot_current`, `snapshot_delta`, `to_snapshot`, and `true`. The generic
   engine validates identity closure; immutable snapshot and delta records
   remain the owners of snapshot/delta compatibility.
3. Add private optional `state_fingerprint` to the normalized source document
   and to `ProjectionSourceSelection`; require it only for a current selector
   whose pair requirement sets `require_state_identity`. The generic request
   binds it to the source document, the delta's `to_snapshot` dependency, and
   the copied generic result provenance. A legacy selection has no state
   fingerprint. Add private `source_dependencies` only to the source envelope:
   each `SourceDependency` carries a stable role, exact source pair, and its
   required state fingerprint. It is not an ambient link, resolver, reverse
   lookup, or payload injection path.
4. Change private currentness construction and validation to look up a
   required family/adapter/slot tuple in the selected current source's
   canonical closure. `CurrentnessFamilyRequirement`, `CapturedRevision`,
   expected request values, and independent live observations each use the
   same exact adapter ref/fingerprint pair. Definition-declared slot sets may
   be empty; when non-empty they are sorted, unique, and must match exactly.
   The result has exactly one canonical `CurrentnessCheck` per required family;
   its work record contains the expected/captured/observed slot maps rather
   than emitting a second record per slot. It rejects missing, duplicate,
   extra, selector/adapter/slot-substituted, or live-mismatched tuples before
   any result. Existing one-family definitions keep their exact singleton
   behavior through an explicit compatibility replay.
5. Run the pair relation check after exact selector binding and before
   currentness/disclosure/payload access. A result keeps both generic source
   selections in its existing `sources` provenance and always retains
   `authority_effect: none`; neither the relation nor the result can mutate
   its sources.

This is the smallest bounded seam because the engine already has a vector of
exact source selections and generic typed omission machinery. The new pieces
only represent plural captured tuples and one declared exact source relation;
they do not add discovery, a Snapshot-specific output model, a second
Projection call path, a caller-selected family subset, or a consumer.

## Required implementation packets (future authority only)

### Packet A — private source-envelope normalisation and pair validator

**Likely owner and paths:** private types and validators in
`crates/engine/src/projection.rs`; private assertions in
`crates/engine/src/projection/tests.rs` only.

**Acceptance criteria:** legacy single-family source bytes preserve existing
success/refusal behavior; raw legacy-only and multi-family-only forms normalize
to exact canonical closure; missing/both-present/empty/unsorted/duplicate raw
closures refuse; a delta can name exactly one current dependency; and the
engine refuses all record/state substitution, malformed, incompatible, or
unbound pair cases before payload access.

**Proof:** focused private source-envelope/parser and relation tests with no
Snapshot Memory runtime adapter or consumer. A future code selector must run
upstream GitNexus impact analysis for every edited function and stop before a
HIGH/CRITICAL expansion unless the new authority explicitly admits it.

### Packet B — currentness closure and generic-result preservation

**Likely owner and paths:** the private currentness request/validation helpers
and exact currentness tests in `crates/engine/src/projection.rs` and
`crates/engine/src/projection/tests.rs`.

**Acceptance criteria:** exactly the five listed tuples, including exact
adapter pairs and empty declared slot sets, are copied from `snapshot_current`,
independently observed live, and represented as exactly five generic result
checks; work's one check contains the three slot maps. Request/result source
provenance retains the current state fingerprint unchanged. All tuple,
adapter-fingerprint, state-identity, and value closure failures refuse;
ordinary one-family Projection currentness remains unchanged; pair identity
passes no authority to the result.

**Proof:** positive five-family replay and the negative matrix below. The
later selector must include an old one-source fixture replay as an explicit
compatibility wall.

### Packet C — private fixed-input PG-SNAP-04 integration proof

**Likely owner and paths:** one new private test-only integration fixture and
one exact test in `crates/engine/src/projection/tests.rs`, with fixed canonical
source bytes defined or included only for that test.

**Acceptance criteria:** the test passes one fixed compatible
`snapshot_current`/`snapshot_delta` pair through unchanged generic execution;
it proves authorized disclosure, typed omission, all-family accounting,
captured-versus-live equality, original/retained-pointer behavior without a
hidden payload read, stale/insufficient refusal, provenance of both sources,
and `authority_effect: none`.

**Proof boundary:** no CLI, public adapter, Handoff, packet, pipeline,
consumer, public schema, or fabricated pre-implementation smoke path. The
fixture is proof-only and cannot become a configured runtime definition.

## Fixed canonical integration matrix

| Case | Required result |
|---|---|
| compatible pair / all five live tuples equal | generic result has both exact sources, every applicable field is disclosed or typed-omitted once, five-family checks pass, and authority effect is none |
| omitted `snapshot_current` or `snapshot_delta` | typed cardinality refusal before result/payload read |
| duplicate role or substituted exact pair/current state fingerprint | typed cardinality or stale-binding refusal before result/payload read; result provenance cannot retain a substituted state identity |
| delta missing, duplicating, or mispointing its `to_snapshot` relation | typed pair-compatibility refusal before currentness/disclosure |
| unstable/excluded current or incompatible/reversed delta endpoints | typed source/pair refusal; no empty/green projection |
| legacy-only, multi-family-only, missing, both-present, empty, unsorted, or duplicate captured closure | only the first two valid forms normalize; every other raw form refuses before payload access |
| omitted, duplicate, extra, or selector/adapter/slot-substituted currentness tuple | typed currentness refusal before output; adapter ref or fingerprint substitution also refuses |
| stale family or slot live observation, including `session` | typed stale refusal; no partial result; successful result has exactly five family checks with work slot maps in one check |
| authorized current field | exact generic disclosure and included accounting |
| resolution-insufficient, redacted, unavailable, or unsupported rule | one generic typed omission with its existing proof effect; no field silently disappears |
| original pointer covered by omit/fingerprint/artifact-ref/redacted-summary disposition | `redacted` omission and zero original-payload reads |
| exact retained pointer outside original subtree | independently evaluated; never treated as covered by prefix coincidence |

## Future verification and review sequence

1. A later explicit implementation selector names the exact code symbols and
   runs upstream impact analysis before any edit. It must warn on HIGH or
   CRITICAL impact before proceeding.
2. Implement Packets A–C sequentially, replaying the unchanged one-source
   currentness fixture at each material boundary.
3. Run focused positive and fail-closed tests, formatting, source diff checks,
   and private source-byte immutability assertions. Then run the applicable
   engine regression wall once the subject converges.
4. Before review, produce a complete manifest that includes the private test
   fixture and every consumer of the changed private model. Do not call this
   packet GREEN if platform, GitNexus comparison, or regression proof is
   unavailable; record it honestly.
5. Use one discovery review, one consolidated remediation for valid P1/P2,
   and one different-fresh closure review under the later selector's frozen
   outcome registry. Mechanical closeout comes only after CLEAN.

## Non-goals and future stop conditions

Stop rather than broaden if the minimum seam needs public visibility,
dependency/Cargo changes, a generic shipped configuration, a schema/API
version, a model-specific Projection result, a Snapshot Memory runtime
serializer, any consumer adoption, HCM-3.5 work, or a revision of PG-SNAP-04.
The next authority must explicitly decide such an expansion. This packet does
not resolve, run, or claim any of those items.
