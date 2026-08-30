# HCM-4.6 Production Snapshot Grounding Owner and Ordinary SDK Seam

Status: **implemented checkpoint; Packets 1–6 and the single aggregate
closeout are complete**.

This follow-up slice productionizes the existing HCM-3.3 through HCM-3.5
projection, Snapshot Memory, grounding, Flow, and pipeline precursor boundary.
It establishes one production Engine owner, a durable snapshot repository, four
ordinary operations, and one fixed SDK session-boundary composition without
selecting an external adopter or reopening the completed Phase-4 or Phase-5
owners.

## Current product state and predecessor

Current source contains the completed production owner and its bounded outward
seams:

- Engine owns bounded redacted capture, immutable durable records, exact
  predecessor/sequence selection, compatible delta, Resolution-bounded
  projection, currentness, retention, recovery, and refusal semantics;
- five private adapters account the exact 15 source slots, while the HCM-3.5
  fixture is test-only and cannot supply production identity or fallback truth;
- Flow and pipeline preserve the typed Engine outcome without recomputation;
- the SDK exposes exactly four admitted ordinary snapshot operations and one
  fixed non-discoverable session-boundary composition; and
- direct SDK plus unchanged generic CLI/Tauri proof closes deterministic truth
  at `62 operations / 35 capabilities / 180 roots / 1 production manifest / 7
  fixed use cases`.

No automatic session trigger or external adopter is selected. That remaining
consumer decision belongs to a separately authorized Phase-6 slice.

HCM-4.6 follows completed Phase 5 because its source adapters may observe the
current Contracts, evidence, gate, local-closeout, Dock, and validator owners.
That chronology does not revise the historical Phase-4 exit or imply that its
selected HCM-4.2 through HCM-4.5 outcome failed. Phase-5 semantic owners remain
closed predecessors, not work owned by this slice.

## Outcome

HCM-4.6 establishes the following production topology:

```text
explicit session-boundary trigger host (future adopter)
    -> fixed transport-free SDK composition
        -> purpose-named Engine snapshot-grounding owner
            -> admitted current source adapters
            -> durable immutable snapshot repository
            -> predecessor / delta / bounded projection / grounding
        -> typed Flow adoption -> typed pipeline inclusion
    -> unchanged generic SDK / CLI / Tauri operation transports
```

The trigger host owns only when to request a start or end boundary and the
trusted repository/workspace/session context supplied by its eventual product
integration. It cannot provide a predecessor, delta, source-currentness verdict,
grounding truth, or success receipt.

The purpose-named `SnapshotGroundingOwnerV1` in `handbook-engine` is the sole V1
semantic and persistence owner. It owns repository identity, policy admission,
source adapters, redaction-before-read, currentness, immutable capture,
predecessor selection, sequencing, delta, projection, grounding, repository
recovery, retention, and owner receipts. The SDK composes that owner with Flow
and pipeline and projects bounded public DTOs. Flow owns adoption of the typed
grounding outcome; pipeline owns whether its typed shared-resolution packet
includes that adopted value. Neither recomputes Engine truth.

CLI and Tauri remain generic transports over admitted SDK operations. No new
CLI command, Tauri command, installed-skill branch, adapter-specific semantic
path, or transport-owned session lifecycle is authorized.

## Repository identity, stream, and boundary sequence

The production repository is rooted below the already trusted physical
repository root at `.handbook/state/snapshot-memory/v1/`. It is not the
demonstrative `.handbook/grounding/hcm-3.5/v1` tree. A repository binding closes
over:

- the normalized retained physical repository root;
- the admitted workspace identity;
- `handbook.snapshot-memory@1.0.0` repository format identity;
- the exact admitted
  `handbook.snapshot-policy.production-session-boundary@1.0.0` ref and derived
  fingerprint; and
- the exact Engine authority/currentness observation used at open.

A stream identity closes over that repository binding, workspace, policy
fingerprint, and the fixed `session` stream class. A boundary identity adds an
opaque host-provided session ref, `start` or `end`, and the Engine-assigned
strictly increasing stream sequence. Caller timestamps, filenames, directory
order, or model text never determine order.

The production policy retains the precursor's five canonical family identities
exactly: `evidence`, `git`, `handbook`, `session`, and `work`. It is a new
versioned production identity, not the demonstrative
`handbook.snapshot-policy.session-boundary@1.0.0`. Its grounding definition is
`handbook.grounding.summary.production-session-boundary@1.0.0`, and its
disclosure policy is
`handbook.grounding.disclosure.production-session-boundary@1.0.0`; each exact
fingerprint is derived from the separately admitted canonical production bytes.
The family names and source-slot names below are closed V1 vocabulary.

For V1, a `start` boundary has exactly one of these predecessor outcomes:

- `applicable_prior`: the immediately preceding committed boundary in the same
  stream is an eligible `end`, and its exact ref/fingerprint is bound; or
- `no_applicable_prior`: the stream has no earlier committed boundary.

An earlier but incompatible, corrupt, stale, held-in-recovery, wrong-policy,
wrong-stream, skipped, or durability-uncertain record is not
`no_applicable_prior`; capture refuses or reports infrastructure unavailable.
An `end` boundary must name the exact open `start` for the same session and must
be the next stream boundary. Starts and ends alternate. A second open start,
end without its start, duplicate sequence, skipped predecessor, or caller-made
predecessor writes nothing.

## Capture lifecycle and idempotency

The fixed SDK use case accepts one explicit `start` or `end` request. It does
not monitor a process, infer a session, or schedule another boundary.

1. The SDK re-admits bounded repository/workspace/session inputs and calls the
   Engine owner once with an opaque idempotency key.
2. Engine opens and recovers the repository, locks the stream, resolves its
   committed head, and rechecks repository binding, source policy, source
   authorities, and currentness before reading an admitted payload.
3. Engine captures every required source family through the fixed adapter
   matrix, applying redaction or omission before payload bytes enter an owner
   record.
4. Before any commit, Engine validates complete source/rule accounting and
   computes the boundary's required compatible delta, Resolution-bounded
   projection, and grounding result. A start binds prior-end to current-start,
   or returns the valid first-start `no_applicable_prior` result without
   inventing an empty predecessor. An end binds its exact open start and derives
   the start-to-end final session delta and bounded handoff grounding.
5. Engine atomically commits the immutable boundary, accounting, required delta
   and grounding/projection records, replay entry, receipt, and derived stream
   head. The receipt is returned only after that whole owner transaction is
   durably committed; failure in any pre-commit semantic or durability check
   writes none of it.
6. SDK passes the committed typed owner result through Flow adoption and
   pipeline inclusion without changing Engine semantics. If this downstream
   composition fails after the owner commit, the use case returns
   `capture_committed_composition_failed` with the exact receipt and replay
   recovery action; it cannot claim write-nothing or automatically recapture.

Exact replay of the same idempotency key and canonical request returns the same
boundary, delta/grounding result, and receipt with `replayed: true`; it performs
no second capture. The SDK may then retry only the pure Flow/pipeline
composition over that committed owner result. Reuse with a different request
conflicts and writes nothing. The raw key is never persisted; only its domain-
separated digest is stored. Concurrent requests for one stream serialize under
the owner lock and observe one committed head.

## Durability, recovery, retention, and compaction

Snapshots, boundary records, source accounting, deltas when retained, and
owner receipts are immutable content-addressed records. Mutable heads and
indexes are derived, bounded, and rebuildable from committed records. Writes use
a repository-local transaction journal and atomic replace within one filesystem
boundary. Recovery completes or rolls back an interrupted prepared transaction,
rebuilds derived indexes, and refuses any conflicting or unverifiable record.

Success is forbidden when record, replay-index, head, directory, or journal
durability is uncertain. A post-commit delivery failure is reported as delivery
uncertain with the idempotency recovery action; it does not authorize automatic
retry. A caller can resolve the original key through the same owner operation.

Retention is selected from the admitted policy's horizon, trigger, and record
class. V1 supports only owner-derived implicit holds with exact identities:
`open-session:<start-ref>`, `applicable-predecessor:<boundary-ref>`, and
`retained-derived:<delta-or-grounding-ref>`. They are deterministically derived
from immutable committed records and the current stream head; a caller cannot
create, release, or override one. A hold ceases only when the owner, under the
stream lock, proves from the next committed head and retention graph that its
condition no longer applies. That derivation and any release are recorded in
the immutable maintenance audit. Unexpired retention windows independently
prevent deletion. Receipts alone do not extend retention.

Pruning and compaction have no public operation in V1. The Engine owner runs one
bounded maintenance pass on repository open, before admitting a new read or
capture. Pruning removes only independently deletable immutable records in a
separate atomic maintenance transaction. Compaction may deduplicate verified
payload blobs or rebuild derived indexes, but cannot rewrite a record's
canonical bytes, ref, fingerprint, accounting, or receipt. A budget-exhausted
pass retains all unprocessed bytes and reports `maintenance_deferred`; a
conflicting, corrupt, or durability-uncertain maintenance journal makes the
repository unavailable until recovery. If safe reachability or hold state
cannot be proved, the owner retains the bytes and never guesses that deletion
succeeded. Caller-created administrative/legal holds are not supported in V1;
any requirement for one is an authority STOP, not a reason to delete or invent
an administrative seam.

## Production source and currentness matrix

The five private Engine adapters are exactly `EvidenceSnapshotSourceV1`,
`GitSnapshotSourceV1`, `HandbookSnapshotSourceV1`,
`SessionBoundarySourceV1`, and `WorkSnapshotSourceV1`. Adapter bytes are
untrusted observations until the owner re-admits identity, authority,
currentness, policy, and Resolution. A stable double-read or owner-native
current-head comparison is performed immediately around capture; a changed or
unavailable currentness basis fails closed.

| Canonical family | Exact production slots and bounded observation | Currentness basis | Failure outcome |
| --- | --- | --- | --- |
| `evidence` | `contracts_definition_lifecycle`: current definition/lifecycle/evaluation basis; `contracts_evidence_root`: immutable evidence root; `contracts_decision`: current verdict/gate/local-closeout refs; `dock_manifest_bundle`: exact production manifest/bundle identity; `dock_execution_candidate`: durable completed execution/candidate refs. All use existing read-only owner ports; no raw evidence, candidate, validator body, or second derived verdict. | Contracts independently recomputes lifecycle/source/authority/freshness/Resolution; Dock rechecks package/execution currentness; Contracts re-admits any candidate source; Engine rechecks every returned exact binding before commit. | absent/inapplicable slot: accounted omission; stale, incomplete, non-completed, untrusted, unavailable, or durability-uncertain slot: refusal/unavailable, never green |
| `git` | `head`, `index`, `worktree`: exact repository `HEAD`, index identity, bounded tracked/untracked status, and policy-selected diff fingerprints; no unrestricted diff body | same normalized repository and all three observations stable across capture | required observation changed/unavailable: capture unavailable; policy-excluded material: accounted omission |
| `handbook` | `engine_authority`, `profile`, `schema_artifact_resolution`: current admitted Engine authority, profile, schema/artifact, Resolution, and policy-selected Handbook heads | exact refs/fingerprints and Engine authority/currentness head stable before commit | stale/substituted/unadmitted authority: refusal; optional absent item: accounted omission |
| `session` | `boundary_context`: trusted host's opaque session ref, boundary kind, and bounded declared context; no transcript, prompt, environment dump, or inferred lifecycle | exact request binding plus owner stream head under the capture lock | wrong/open/closed/replayed-conflict sequence: refusal; first start: valid `no_applicable_prior` |
| `work` | `flow_state`, `pipeline_state`, `handoff`: current bounded Flow/pipeline work, route, capture, and handoff refs already committed by their owners; no restored orchestration ledger | exact owner refs/fingerprints and current heads re-admitted immediately before commit | required stale/unavailable slot: unavailable; policy-inapplicable slot: accounted omission |

The historical `evidence` family is therefore retained, not replaced by new
`contracts` or `dock` families. Completed HCM-5 ownership is represented by the
five exact slots inside that family, preserving compatible five-family delta and
grounding vocabulary while making the post-Phase-5 sources explicit.

The completed Contracts, evidence, gate, local-closeout, Dock, and validator
owners remain authoritative for their own facts. These adapters observe their
bounded public/read-only truths and do not reconstruct, persist, promote, or
reinterpret them. No HCM-5 schema, verdict, gate, local-closeout, manifest,
execution, or candidate semantic is reopened.

## Projection, disclosure, and public-data boundary

Production capture is policy-complete: each required or observed source item is
classified exactly once as retained observation, derived bounded fact,
redacted, omitted, inapplicable, unavailable, or refused. Unknown family,
unknown rule, adapter mismatch, matcher failure, source drift, and incomplete
accounting fail closed.

Secrets, environment values, secret-file bodies, raw command output,
unrestricted diffs, raw evidence/candidate bodies, transcripts, and unknown
bytes are omitted or replaced with an admitted fingerprint/artifact ref before
read into the snapshot payload. No raw comprehensive snapshot is stored or
returned by default. The public `snapshot.read` operation returns a descriptor,
source accounting, and bounded retained fact summaries, never repository
record bytes or the complete source aggregate.

Delta is computed only over an exact compatible predecessor/current pair and
accounts for every selected family and rule. Projection is bounded by the exact
admitted definition, disclosure policy, Resolution, vocabulary, and currentness
observations. It may reveal or derive only admitted retained facts; it cannot
synthesize missing authority. Its result includes complete omissions,
refusals, lossiness, provenance, and required rechecks.

## Fixture disposition

The checked-in `.handbook/grounding/hcm-3.5/v1` material and its repeated
placeholder fingerprints remain demonstrative HCM-3.5 lineage only. They are
not promoted, copied into the production repository, admitted as production
source identities, or used as fallback truth.

The implementation packet that removes the fixed production lookup must move
any still-needed examples behind explicit test support and label them
demonstrative. Production policy/source identities use the new V1 owner and
repository bindings above. An implementation that preserves the old fixture as
a production default or silently replaces a placeholder with locally observed
authority must stop.

## Exact outward seam and inventory delta

HCM-4.6 closes exactly four existing reserved ordinary rows. It does not add or
remove an inventory row.

| Operation | V1 behavior | Owner | Mode | Final transports |
| --- | --- | --- | --- | --- |
| `snapshot.capture` | capture one explicit start/end boundary with idempotent durable receipt | Engine | append | `[cli_json, rust_sdk, tauri]` |
| `snapshot.read` | read one exact bounded descriptor/accounting result | Engine | read-only | `[cli_json, rust_sdk, tauri]` |
| `snapshot.delta` | compute/read one exact compatible bounded delta | Engine | read-only | `[cli_json, rust_sdk, tauri]` |
| `snapshot.project` | compute one Resolution/disclosure-bounded grounding projection | Engine | read-only | `[cli_json, rust_sdk, tauri]` |

The owner designation on `snapshot.capture` changes from the absent
SDK-composition placeholder to Engine; `snapshot.project` changes from the
absent Flow placeholder to Engine. This is not an implemented inventory change
until the rows pass definition, schema, independent owner admission, dispatch,
transport, and production-witness proof.

`projection.definition.read`, `projection.create`,
`snapshot.verify_current`, and `snapshot.resolve_applicable` remain absent.
Their required semantics are owner-private parts of policy admission,
currentness, and predecessor resolution in this slice; there is no independent
ordinary consumer justifying closure. They cannot be closed by reservation
inertia.

The SDK also adds exactly one fixed, non-discoverable use case:
`handbook.use-case.session-grounding-boundary@1.0.0`. Its closed request
discriminator is `start` or `end`; a start composes the prior-end to start delta
when applicable, while an end composes the exact open-start to end final delta
and bounded handoff refs. Both variants include projection, Flow adoption, and
pipeline inclusion. Like the six existing fixed use cases, it does not enter
the 62-row inventory, owner
admission, capability catalog, or generic capability-driven installed skill.
It is callable through the transport-free Rust SDK and the existing generic
Tauri use-case discriminator. It gains no special CLI command or Tauri command.

## DTO, schema, manifest, and owner-admission closure

Public DTOs are purpose-bounded projections. They carry exact refs,
fingerprints, terminal classification, predecessor outcome, sequence,
Resolution, currentness, complete source/omission/refusal accounting, replay,
receipt, and recovery action; they carry no raw key, credential, physical host
path, private authority record, unrestricted source bytes, raw snapshot, or raw
HCM-5 evidence/candidate body.

The four ordinary operations add eight operation roots, and the fixed use case
adds two roots:

```text
handbook.operation.snapshot-capture-request@1.0.0
handbook.operation.snapshot-capture-result@1.0.0
handbook.operation.snapshot-read-request@1.0.0
handbook.operation.snapshot-read-result@1.0.0
handbook.operation.snapshot-delta-request@1.0.0
handbook.operation.snapshot-delta-result@1.0.0
handbook.operation.snapshot-project-request@1.0.0
handbook.operation.snapshot-project-result@1.0.0
handbook.use-case.session-grounding-boundary-details@1.0.0
handbook.use-case.session-grounding-boundary-result@1.0.0
```

Nested repository binding, boundary, source-accounting, descriptor, delta,
grounding, omission/refusal, recheck, receipt, and problem shapes remain `$defs`
within those ten independently retrievable roots. A genuine need for another
independently retrievable root stops for authority remediation; a packet cannot
silently change the count.

The exact admitted final product truth is therefore:

- **62 operation rows** — unchanged;
- **35 capabilities** — exactly four newly admitted ordinary rows;
- **180 independently retrievable schema roots** — exactly ten new roots;
- **one production Dock manifest** — unchanged; and
- **seven fixed SDK use cases** — exactly one new non-discoverable use case.

Every ordinary row requires a complete definition, request/result schemas,
independent Engine owner evidence, dispatch, checked-in exact schema bytes,
deterministic manifest entry, production-witness capability evidence, and
generic Rust SDK/CLI/Tauri proof. The use case requires SDK composition and
generic Tauri proof but remains outside owner admission and capability
discovery. Count guards and the immutable production witness change only after
all corresponding evidence is current.

## Failure and trust boundary

Malformed or over-limit input, unknown/substituted refs, repository mismatch,
policy mismatch, source drift, incompatible predecessor, sequence conflict,
incomplete accounting, insufficient Resolution, redaction uncertainty,
retention conflict, corrupt store, and idempotency conflict detected before the
owner transaction are typed refusals/conflicts and write nothing. Required
source unavailability, lock/recovery failure, and uncertain owner-transaction
durability are typed unavailable/error outcomes and cannot manufacture a
semantic result. After a durable owner commit, Flow/pipeline composition or
adapter delivery failure reports the exact committed receipt and recovery state
rather than claiming write-nothing. Read absence and the first-start
`no_applicable_prior` case are valid typed results, not failures.

Public logs, diagnostics, receipts, DTOs, schemas, fixtures, CLI output, and
Tauri failures are bounded and secret-free. Adapter framing, schema validity,
instance canonical fingerprints, checked-in schema-byte fingerprints, owner
admission, and repository durability remain distinct proof domains.

## Scope and allowed implementation boundary

A later implementation task under this authority may change only:

- directly necessary Engine projection, Snapshot Memory, grounding, production
  adapter, repository, recovery, retention, and focused-test paths;
- directly necessary Flow and pipeline typed adoption/inclusion paths and
  focused tests, without moving Engine semantics;
- directly necessary SDK operation, DTO, fixed-use-case, schema generation,
  admission, production witness, generic dispatch, and focused-test paths;
- generic CLI/Tauri regression and parity tests plus the existing generic Tauri
  use-case discriminator; and
- directly necessary current navigation and protocol documentation.

The old demonstrative fixture may be moved or relabeled only as part of focused
test support after production lookup is removed. Existing Contracts, evidence,
gate, local-closeout, Dock, validator, posture, CLI-command, Tauri-command, and
installed-skill product semantics are read-only.

## Cross-packet invariants

1. Engine is the sole snapshot-grounding semantic and persistence owner; SDK,
   Flow, pipeline, transports, and a future host cannot self-admit that truth.
2. Capture is always explicit. No watcher, hook, process monitor, or model text
   infers start/end or automatically adopts the use case.
3. Redaction/omission occurs before source payload read or persistence, and no
   raw comprehensive snapshot is a default product or public DTO.
4. Every required source and rule is accounted exactly once; incomplete or
   unknown accounting fails closed.
5. The stream head, applicable predecessor, sequence, delta pair, projection
   policy, Resolution, and currentness are owner-derived and exact.
6. Immutable record bytes, refs, fingerprints, receipts, and replay outcomes
   survive recovery; compaction never rewrites them.
7. No success is returned under durability uncertainty. No adapter retries an
   uncertain mutation automatically.
8. Completed HCM-5 owners remain authoritative. Their observations are inputs,
   not copied semantic ownership or permission to persist a second verdict.
9. Exactly four reserved rows close, exactly one fixed use case is added, and
   final truth is `62 / 35 / 180 / 1 / 7`.
10. Generic CLI/Tauri routes remain generic. Command names, fixture paths, and
    the fixed use-case discriminator do not become semantic identities.
11. `no_applicable_prior` is valid only when the same stream has no prior
    committed boundary; every other predecessor uncertainty fails closed.
12. Every valid end atomically binds its open start and commits the compatible
    start-to-end final delta, bounded handoff grounding, and complete accounting.
13. Only owner-derived implicit V1 holds exist; maintenance is owner-triggered
    on repository open and cannot add an administrative operation by accident.
14. Phase-6 adopter selection and automatic session lifecycle integration are
    absent from every packet and from HCM-4.6 closeout.

## Ordered implementation packets

Packets are strict and sequential. Packet implementation authority begins only
after this planning commit and a separately dispatched execution task.

| Packet | Outcome | Hard dependency | Completion proof |
| --- | --- | --- | --- |
| **1 — owner, repository binding, and source-policy kernel** | Establish the purpose-named Engine owner, exact repository/stream/boundary identities, the exact production policy/definition/disclosure refs, five canonical families, closed source slots, five private adapters/currentness matrix, and closed error/accounting vocabulary. Remove production reliance on the demonstrative fixture without capturing or persisting a production record yet. | HCM-4.6 authority and Phase-5 exit | Focused Engine tests prove canonical identities, five-family/slot policy admission, exact adapter ownership/currentness contracts, compatibility with the precursor family vocabulary, redaction-before-read floor, complete accounting vocabulary, no old-fixture production fallback, and dependency/source/security gates. |
| **2 — durable immutable repository and lifecycle** | Implement repository open/recovery, immutable record append/read, derived heads/indexes, stream locking, strict start/end sequencing, applicable-predecessor selection, valid first start, idempotency/conflict, atomic all-owner-result commit, receipts, implicit hold derivation/release audit, retention, owner-open maintenance, pruning, and non-rewriting compaction. | Packet 1 | Focused crash/reopen/concurrency/replay/retention tests prove atomic recovery, exact alternation and no-skipping, key conflict write-nothing, all-or-nothing boundary/delta/grounding/head commit, uncertain-durability no-success, exact implicit holds/releases, maintenance-deferred retention, no administrative seam, and stable record bytes/fingerprints. |
| **3 — production capture, delta, projection, and grounding** | Implement all five source adapters and exact slots, stable currentness rechecks, bounded redacted capture, prior-end-to-start and start-to-end compatible deltas, Resolution/disclosure-bounded projection, complete omission/refusal accounting, and typed start/final grounding over the durable repository. | Packet 2 | Engine tests cover every family/slot/currentness/failure cell, first-start and applicable-prior start paths, exact open-start-to-end final delta/handoff, end replay/recovery, drift and substitution, redaction-before-read, no raw aggregate, complete accounting before commit, deterministic vectors, and all HCM-3.3–3.5 precursor regressions. |
| **4 — Flow, pipeline, and fixed SDK composition** | Adopt the typed start or final grounding outcome through Flow/pipeline and add the exact fixed session-boundary SDK use case with start/end variants, committed-owner/composition-failed recovery, replay/delivery recovery, and generic Tauri use-case dispatch. | Packet 3 | Cross-crate tests prove one owner execution, exact Flow/pipeline status preservation, no recomputation, end final-delta/handoff preservation, pure post-commit recomposition, no automatic lifecycle trigger/recapture, no CLI/Tauri special command, and use-case absence from ordinary inventory/discovery. |
| **5 — four ordinary operations and schema/admission closure** | Close exactly `snapshot.capture`, `snapshot.read`, `snapshot.delta`, and `snapshot.project`; generate exactly the ten roots; establish Engine owner evidence, dispatch, manifest entries, and immutable production-witness capability evidence at exact `62 / 35 / 180 / 1 / 7`. Leave the other four reserved rows absent. | Packet 4 | SDK/schema/owner tests prove DTO bounds, strict validation, exact roots/counts/fingerprints, independent owner admission, mutation receipt/replay, no raw/private data, and discovery refusal until every row's definition/schema/owner/transport evidence is complete. |
| **6 — generic consumers, Phase-6 handoff, and integrated proof** | Prove the four operations through direct SDK and unchanged generic CLI/Tauri consumers, the fixed use case through direct SDK/generic Tauri, all failure/recovery paths, deterministic schema regeneration, source/dependency/security gates, documentation consistency, and the Phase-6 handoff record below. | Packet 5 | Focused Engine/Flow/pipeline/SDK/CLI/Tauri suites, existing precursor and HCM-4/HCM-5 regressions, formatting/check/test/strict Clippy/rustdoc, schema regeneration/meta-validation, exact count guards, Markdown links, secret/source/dependency gates, and required GitNexus change detection all pass in one aggregate run. |

## Delegated just-in-time preparation

Each packet task must reread this authority, the immediately completed packet,
current affected source/tests, and only the architecture/skill material needed
for that packet. It may choose private file/module/test factoring, bounded
fixture names, and equivalent internal type decomposition.

It may not change owner placement, repository/stream identity, the five
families/closed source slots or adapter identities,
currentness/failure rules, lifecycle sequencing, fixture disposition, public
operation/use-case identity, public schema-root inventory, counts, transport
shape, Phase-6 boundary, or any cross-packet invariant. If permitted current
sources leave more than one consequential answer on those subjects, the packet
must stop for authority remediation rather than invent a local decision.

## Slice acceptance

HCM-4.6 is complete only when all of the following are simultaneously true:

1. A purpose-named Engine owner implements the selected semantic/persistence
   topology with no SDK, Flow, pipeline, transport, or host back-edge.
2. The production repository/stream/boundary identities and strict start/end
   sequence are exact, and first start has the valid `no_applicable_prior`
   outcome only when no same-stream boundary exists.
3. Immutable persistence, idempotency, concurrency, crash recovery, receipts,
   retention, holds, pruning, compaction, and durability-uncertain behavior pass
   focused proof.
4. All five source families, exact slots, and currentness bases are production-
   backed, bounded, and fail closed without reopening their semantic owners.
5. Every selected source/rule is accounted exactly once and redaction/omission
   occurs before source payload read or persistence.
6. Compatible delta and admitted Resolution/disclosure-bounded projection are
   deterministic and expose no raw comprehensive snapshot by default.
7. The HCM-3.5 fixture is demonstrative/test-only and cannot supply production
   policy, source identity, predecessor, fingerprint, or fallback truth.
8. Flow and pipeline preserve the typed owner outcome without recomputing it.
9. The exact fixed SDK use case composes one explicit boundary and remains
   outside operation inventory, owner admission, capability discovery, and the
   generic installed skill.
10. A valid end commits the exact start-to-end final delta and bounded handoff;
    exact replay or downstream recomposition cannot create another capture.
11. Exactly the four named rows have complete Engine definition/schema/owner/
    dispatch/transport/witness closure; the other four reserved rows stay
    absent.
12. Exactly ten roots are added and the final deterministic truth is
    `62 operations / 35 capabilities / 180 roots / 1 production manifest / 7
    fixed use cases`.
13. Direct SDK and unchanged generic CLI/Tauri paths preserve terminal,
    currentness, omission/refusal, Resolution, replay, receipt, and recovery
    truth; no special semantic branch exists.
14. Malformed, stale, substituted, incompatible, incomplete, conflicting,
    corrupt, unavailable, or durability-uncertain inputs cannot produce a
    fabricated success or partial write.
15. Public DTOs, logs, diagnostics, schemas, receipts, fixtures, and transport
    errors contain no raw keys, credentials, private authority, physical paths,
    source aggregates, unrestricted diffs, transcripts, or raw HCM-5 bodies.
16. The focused and aggregate verification matrix passes with deterministic
    identities/counts and required GitNexus change detection.
17. Current documentation reports both the completed implementation and the
    still-unselected Phase-6 adopter boundary without changing historical
    Phase-4/Phase-5 closeout claims.

## Phase-6 adopter handoff and proof boundary

The implemented Packet 6 boundary proves deterministic explicit invocation of
the four admitted operations through the direct Rust SDK and unchanged generic
CLI/Tauri consumers, and proves the fixed session-boundary use case through the
direct Rust SDK and unchanged generic Tauri discriminator. The proof preserves
terminal, currentness, omission/refusal, Resolution, replay, receipt,
final-delta, handoff, and recovery truth. It does not select or implement an
external adopter, automatic trigger, watcher, or session lifecycle host.

HCM-4.6 hands Phase 6 one production-capable, transport-free SDK use case and
four admitted ordinary operations. Its proof ends at deterministic explicit
invocation through current generic repository consumers. It does not prove that
an IDE, agent host, background service, publication bridge, Substrate, or any
other external product calls the seam at session start/end.

A later Phase-6 authority must name the actual trigger host, its trusted
repository/workspace/session binding, exact start/end events, cancellation and
delivery behavior, product UX/operations owner, deployment boundary, and
adoption evidence. It must consume the HCM-4.6 use case without supplying
predecessor/currentness/grounding truth or adding an SDK semantic branch. It
must preserve the start result and the end result's final-delta/handoff refs and
must recover a committed owner receipt without recapturing. No repository-only
test may substitute for that external adoption proof.

## Aggregate closeout rule

After Packets 1–6 have individual current checkpoints, one aggregate closeout
task reruns the integrated matrix, verifies exact `62 / 35 / 180 / 1 / 7`
truth, confirms all four remaining reserved rows are absent, reviews the full
causal implementation diff once under the current review policy, updates only
current truth that changed, and makes one coherent implementation closeout
commit. It does not create a ledger, proof wall, dispatch archive, selector,
durable handoff capsule, or Phase-6 authority.

## Genuine stop conditions

Preparation or implementation must stop if:

- a selected source lacks one production owner/currentness basis, or observing
  it requires reopening HCM-5 semantics;
- safe repository identity, atomic recovery, stream order, retention reachability,
  or no-success-under-uncertain-durability cannot be proved;
- a caller-created administrative/legal hold is required before a separately
  authorized hold owner and bounded administrative seam exist;
- a required adapter must read/store raw comprehensive material before
  redaction or cannot account every selected source/rule;
- current source requires a different public operation, use-case, schema-root,
  transport, or final count than this authority;
- closing one of the four intentionally absent reserved rows becomes necessary;
- an implementation relies on the old fixture for production identity/truth;
- the only viable composition adds a special CLI/Tauri/skill semantic route;
- work requires automatic session triggering or a real adopter before a
  separately authorized Phase-6 slice; or
- required impact/change detection reports an unreviewed high or critical
  cross-owner blast radius.

## Explicit non-goals

- product implementation, schemas, fixtures, adapters, operations, tests, or a
  consumer in this authority-preparation task;
- automatic session lifecycle integration or selection/invention of an
  external Phase-6 adopter;
- posture, recommendation, notification, promotion, parent-readiness, or parent
  mutation authority;
- reopening or replacing HCM-5 Contracts, evidence, verdict, gate,
  local-closeout, Dock, validator, manifest, or candidate semantics;
- raw comprehensive snapshot retrieval, transcripts, unrestricted diffs,
  secrets, environment dumps, or model-owned source interpretation;
- a special CLI command, Tauri command, installed-skill branch, transport-owned
  semantic path, or new publication/bridge/Substrate claim;
- closing every reserved projection/snapshot row merely because it exists; or
- restoration of old orchestration ledgers, proof walls, dispatches, selectors,
  control-plane status, or historical ownership conclusions.

## Use and drift

This file is the single current HCM-4.6 slice authority and completed checkpoint.
Current product source and tests remain truth about what is implemented. If a
later task finds a consequential conflict with this authority, it stops and
remediates this file before changing product semantics. Phase-6 work still
requires separate authority and a named external adopter.
