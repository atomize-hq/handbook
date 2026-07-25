# HCM-2.3 Generic Artifact Runtime Contract 1.0

## Authority and version boundary

This contract is planning authority for the exact HCM-2.3 `registry-brief`
proof only. It is bound by the reviewed planning subject and does not enter the
shipped definition catalog. Its logical identity is
`handbook.hcm-2-3.generic-artifact-owner-contract@1.0.0`.

The machine schemas are:

- [`generic-artifact-runtime-records-1.0.0.schema.json`](generic-artifact-runtime-records-1.0.0.schema.json);
- [`generic-artifact-control-records-1.0.0.schema.json`](generic-artifact-control-records-1.0.0.schema.json);
- [`repository-profile-selection-1.0.0.schema.json`](repository-profile-selection-1.0.0.schema.json); and
- the exact positive/negative vectors beside them.

HCM-2.3 allocates these additive legacy record-routing versions:

| Record | HCM-2.3 version | Prior versions preserved |
|---|---|---|
| `handbook.artifact-intake-record` | `1.2` | `1.0`, `1.1` |
| `handbook.artifact-validation-result` | `1.0` (new generic structural result) | HCM-2.2 lifecycle result is a different schema ID and stays unchanged |
| `handbook.artifact-candidate` | `1.4` | `1.0`, `1.1`, `1.2`, `1.3` |
| `handbook.artifact-promotion-record` | `1.2` | `1.0`, `1.1` |

These tags are record-routing versions, not public JSON Schema SemVer refs.
No decoder aliases, widens, upgrades, rewrites, or falls back across versions.
The HCM-2.2 Charter path continues to require its exact record versions and
closed values. A generic `1.2`/`1.4`/`1.2` record in a Charter store, or a
Charter record in the generic store, refuses `record_family_mismatch` before
lineage or mutation.

## Exact executable operation sequence

The only valid acquisition-to-promotion order is:

1. `intake.coverage.evaluate` evaluates supplied submissions and returns an
   ephemeral `CoverageEvaluationV1`; it has no timestamp, record/candidate ID,
   ref, or write;
2. `intake.record.append` re-evaluates the exact request under current context,
   establishes its journal, samples `finalized_at_utc` once, and atomically
   appends intake record `1.2` plus its subordinate value closure;
3. `artifact.candidate.validate` loads that committed intake ref/fingerprint,
   reconstructs normalized content and field sources, runs every applicable
   validation layer, and returns the exact candidate preview, validation result
   preview, normalized content, and their derived refs/fingerprints without a
   write;
4. `artifact.candidate.append` accepts the committed intake ref/fingerprint and
   `expected_candidate_fingerprint`, independently repeats step 3, requires
   exact preview equality, and atomically appends candidate `1.4` plus its
   subordinate validation/content closure; and
5. `artifact.candidate.promote` loads the committed candidate closure, reruns
   all current validation, and compare-and-writes canonical YAML plus promotion
   record `1.2`.

No operation can be skipped by supplying derived bytes. Validation input is
the committed intake ref/fingerprint plus expected basis/context, never an
unpersisted `candidate_ref`. The CLI `candidate-validate` therefore uses
`--intake-record-ref`, `--intake-record-fingerprint`, and optional exact basis;
`candidate-append` receives the same pair plus
`expected_candidate_fingerprint` inside its bounded request document. Append
revalidation makes calling validate first observable and testable but never
turns a caller-returned preview into authority.

`CoverageEvaluationV1` is a closed result containing operation context, mode,
basis, ordered typed coverage results, normalized content, field sources, and
`evaluation_fingerprint`. That fingerprint is RFC 8785/SHA-256 over the entire
result except itself. It is advisory and is not retained by append. The append
request repeats the raw normalized submissions; the engine recomputes the
evaluation and refuses any optional caller-supplied evaluation fingerprint that
does not match. This prevents a pre-finalization evaluation from predicting an
intake ID whose timestamp does not yet exist.

## Record identity and exact closure

Every record is a closed object accepted by the runtime-record schema. Array
order is semantic. Object member order is not. Record bytes are RFC 8785 JCS
UTF-8 plus one LF.

For each record, compute the subject by removing exactly its ID and terminal
fingerprint fields:

| Record | Removed fields | ID derivation |
|---|---|---|
| intake `1.2` | `intake_record_id`, `record_fingerprint` | `intake_<fingerprint hex>` |
| validation `1.0` | `validation_result_id`, `validation_result_fingerprint` | `validation_<fingerprint hex>` |
| candidate `1.4` | `candidate_id`, `candidate_fingerprint` | `candidate_<fingerprint hex>` |
| promotion `1.2` | `promotion_id`, `promotion_fingerprint` | `promotion_<fingerprint hex>` |

The fingerprint is lowercase `sha256:` plus SHA-256 of the subject's RFC 8785
bytes without LF. The complete persisted record inserts the derived ID and
fingerprint, is serialized again as JCS plus one LF, and must reproduce both on
reload. No audit-only field is excluded. `finalized_at_utc` therefore
participates in intake identity and is sampled once when the transaction intent
is durably established; replay uses the recorded value.

The exact record fields, enum values, nullability, and grammar are the JSON
Schema, not illustrative prose. In particular:

- every exact definition ref uses the inherited HCM-1.1 3–255-byte identity,
  at-least-two 1–63-byte dot segments, lowercase segment grammar, single `@`,
  and full byte-canonical SemVer contract, with no whitespace or line
  terminator; and
- every safe/repository ref is an ASCII `/`-separated normalized relative path
  of at most 1,024 bytes and 64 nonempty components, with no leading/trailing
  slash, empty or dot component, backslash, colon/URI/drive prefix, whitespace,
  line terminator, or alias;

- intake `1.2` adds exact intake/operation-context/value fingerprints and basis
  without changing the immutable HCM-2.2 `1.1` or earlier `1.0` lineage;
- validation `1.0` is deterministic and timestamp-free, binds intake/content,
  current profile/context/schema closure, exact layer results, and `valid`;
- candidate `1.4` adds exact intake/context/content/result bindings, requires
  one validation ref, and permits only `eligible_without_approval` plus null
  approval policy; and
- promotion `1.2` requires no approval refs, one validation ref,
  `decision: not_required`, and null `authorized_by_ref`.

The two null-policy closed values are legal only in those new versions and only
when the selected intake policy is null and the kind has no approval-requiring
capability. They are not accepted by Charter record schemas. `approved` remains
impossible without required approval refs.

## Validation-result authority

`artifact.candidate.validate` is read-only. It constructs the complete
timestamp-free validation `1.0` result in memory, derives its fingerprint and
future content-addressed ref, and returns it as a preview. It writes no result,
content, candidate, receipt, journal, key, or hidden cache.

`artifact.candidate.append` reloads the committed intake/value closure,
reconstructs the normalized content, repeats validation, and requires its
derived candidate fingerprint to equal `expected_candidate_fingerprint`. It
then persists the exact validation-result bytes under:

```text
.handbook/evidence/artifacts/<instance-id>/validation-results/validation_<fingerprint>.json
```

The candidate's sole `validation_result_refs` member is that exact path.
Validation identity binds the operation-context fingerprint, so profile/schema/
kind/intake/descriptor drift derives a different preview and makes the expected
candidate stale. Promotion does not trust the retained result as a waiver: it
recomputes the current result from retained intake/content and current resolved
definitions, requires byte/fingerprint equality with the retained result, then
uses the same ref in promotion `1.2`. Missing, duplicate, malformed, rewritten,
wrong-context, wrong-schema, stale, or uncommitted result bytes refuse before
canonical mutation.

The promotion's ordered `resolved_definitions` is not a separately authored
summary: it must equal the selected operation context's UTF-8 exact-ref-sorted
`resolved_definitions` array byte-for-byte. Any ref, fingerprint, cardinality,
or order difference refuses even if the promotion identity was recomputed.

## Supporting closure and canonical write sets

Content-addressed intake values, normalized content, and validation-result
bytes are subordinate closure objects, not independent semantic records,
canonical truth, observations, or write receipts. They are unreachable through
`record.list`, `record.read`, definition discovery, or a standalone mutation.
An owner resolver may return one only while resolving a committed semantic
record that cites its exact ref/fingerprint. An orphan is invisible and blocks
recovery/admission; it is never adopted or deleted automatically.

The realized write sets remain exactly canonical:

| Operation | Authoritative write set | Subordinate closure | Receipts |
|---|---|---|---|
| `intake.record.append` | exactly one `semantic_record` intake `1.2` | exactly the ordered distinct value objects cited by its coverage results | exactly one future semantic-record receipt; closure refs remain internal evidence |
| `artifact.candidate.append` | exactly one `semantic_record` candidate `1.4` | exactly one normalized-content object and one validation-result object cited by the candidate | exactly one future semantic-record receipt; subordinate refs remain internal evidence |
| `artifact.candidate.promote` | exactly one `canonical_truth` artifact and exactly one `semantic_record` promotion `1.2` in one atomic group | none | exactly two receipts, one per authoritative write-set item |

HCM-2.3 does not invent a `WriteReceipt` lookalike. Its closed local mutation
result reports authoritative output refs/fingerprints plus one separately typed
internal-transaction-evidence ref/fingerprint. Subordinate refs occur only in
the semantic record, marker, and internal evidence; they never occur in a
canonical receipt. Phase 4 alone constructs canonical `WriteReceipt` bytes,
whose exact fields remain `{record_kind, record, authority_class, condition,
atomic_group, receipt_fingerprint}`. A future adapter maps one authoritative
output to one receipt and supplies its Phase-4-owned condition/atomic-group
context; it does not copy transaction IDs, replay state, or subordinate closure
into that receipt. Thus intake/candidate will yield one canonical receipt and
promotion two only after Phase 4, without claiming byte identity between an
HCM-2.3 local result and a future receipt. Tests compare actual filesystem,
semantic-record, internal-evidence, and authoritative-output deltas to this
table and reject every undeclared path or pseudo-receipt.

## Direct CLI domain replay and Phase 4

HCM-2.3 does not publish or simulate the Phase 4 bootstrap descriptor, operation
catalog, transport DTOs, or SDK idempotency ledger. Its direct CLI uses an
engine-owned **domain mutation ledger**, not the later ordinary-transport
ledger. The domain key scope is exactly:

```text
repository_identity_fingerprint
owner_contract_ref = handbook.hcm-2-3.generic-artifact-owner-contract@1.0.0
owner_contract_subject_fingerprint = <planning-handoff exact subject>
stable_operation_id
domain_mutation_key_fingerprint
```

The bounded request field remains named `idempotency_key` for compatibility;
the raw value is 16-128 ASCII `[A-Za-z0-9_-]`, input-only, and its fingerprint
is `sha256(RFC8785({"key": <raw key>}))`. The domain request fingerprint is
over the closed owner contract/repository/key fingerprint and exact caller
operation subject in the control schema; derived operation context is excluded
and is instead frozen in the transaction intent. Lookup/reservation/replay
therefore occurs before resolving current context. A same-key exact caller
request replays its original result after profile/intake/canonical drift, while
a same-key different request conflicts; drift cannot establish a second
mutation.

The establishment boundary is exact. Lookup of an existing key occurs first.
An exact `retained_result` replays its retained result; an exact `active_hold`
resumes its one transaction; a different request conflicts; and a tombstone
returns `idempotency_expired` for the exact request or `idempotency_conflict`
for a different request. For a new key, repository identity, the complete exact
operation context, and deterministic operation evaluation must succeed before
establishment. Only an evaluation that produces either a complete commit plan
or one closed `establishedRefusal` may establish. Establishment atomically
publishes the complete intent at its recognized establishing path; the ledger's
`active_hold` is the exact logical projection of that valid intent before or
after its byte-preserving move to pending `intent.json`, not a second persisted
file. Therefore hold and intent cannot tear or disagree.

The pre-establishment/established matrix is closed:

| Class | Exact cases | Durable effect |
|---|---|---|
| pre-establishment control refusal | invalid/oversized/unknown request; invalid key; missing, unsafe, changing, or mismatched repository identity; lock failure; unresolved/unsafe operation context; corrupt or ambiguous ledger/journal inventory | return the typed control refusal; create no hold, journal, evidence, result, or artifact byte |
| existing-key control outcome | exact retained result; exact active hold; same-key different request; exact/different tombstone request | replay, resume, `idempotency_conflict`, or `idempotency_expired` as fixed above; create no second transaction |
| new established commit | exact context plus a complete deterministic output plan | atomically publish the intent with `planned_outcome: commit` at the establishing path; its exact `active_hold` projection now exists; move the same bytes to pending `intent.json` |
| new established refusal | exact context plus exactly one closed refusal decision | atomically publish the outputless intent with `planned_outcome: refuse` at the establishing path; its exact `active_hold` projection now exists; move the same bytes to pending `intent.json` |

The only established-refusal codes and payload are the closed
`establishedRefusal` definition in the control-record schema. The same complete
object, operation ID, transaction ID, request fingerprint, context fingerprint,
and journal fingerprint chain is repeated without reinterpretation through
intent, verified refusal decision, refused marker, evidence, and result.
`stale_basis` contains unequal non-null expected/observed basis fingerprints;
`stale_current_artifact` contains the exact nullable request expectation and
locked nullable observation and they must differ; every other code has both
fingerprint fields null.
Context/source resolution failure is pre-establishment, so a retry may observe a
repaired repository; a recorded established refusal is not re-evaluated and
replays byte-identically despite later context or artifact drift.

The ledger retains successful and refused established results for at least 30
days, then may replace a result only with a non-expiring consumed-key tombstone.
Tombstones retain scope, key fingerprint, and request fingerprint. The active
recovery hold is derived only from the journal intent; exact held requests
resume/serialize, different requests conflict, and no second transaction is
created or persisted.

Phase 4 may later wrap, but never rewrite, this domain ledger. HCM-2.3 proves
only owner-side invariants: its derivation accepts repository/owner/operation,
the raw key, and the closed owner request—no negotiated API, bootstrap,
transport, or operation-definition fields—so equal owner input always derives
the same key/request pair; replay/conflict/tombstone behavior is wholly owned
here. A future outer entry must bind and pass that pair without deriving a
second inner namespace, migration, reinterpretation, deletion, or replacement.
Phase-4-first/direct, direct-first/Phase-4, outer-ledger transition, and
inter-ledger crash execution fixtures are explicitly deferred to Phase 4,
which owns their still-undefined state model. HCM-2.3 tests only that transport
metadata cannot enter the owner API/preimage and that deterministic owner
fixtures replay identically through repeated direct invocations. This boundary
neither invents Phase 4 descriptor/receipt bytes nor weakens its outer contract.

## Journal identities and bounded inventory

All mutations take the one repository-scoped generic-artifact lock after the
existing repository-identity and HCM-2.2 recovery locks. The transaction ID is
`<operation-token>_<64 lowercase hex>`, where the hex is SHA-256 of RFC 8785:

```json
{
  "repository_identity_fingerprint": "sha256:...",
  "owner_contract_ref": "handbook.hcm-2-3.generic-artifact-owner-contract@1.0.0",
  "owner_contract_subject_fingerprint": "sha256:...",
  "operation_id": "...",
  "domain_mutation_key_fingerprint": "sha256:...",
  "request_fingerprint": "sha256:..."
}
```

Operation tokens are exactly `intake_record_append`, `artifact_candidate_append`,
and `artifact_candidate_promote`. Their pending directory names and intent
`transaction_id` must agree.

Establishment uses two same-filesystem atomic file renames under the generic
lock. The exact paths are:

```text
.handbook/state/idempotency/generic-artifact-operations/establishing/<domain-mutation-key-fingerprint>.intent.writing
.handbook/state/idempotency/generic-artifact-operations/establishing/<domain-mutation-key-fingerprint>.intent
```

The owner writes and syncs the complete JCS+LF intent to `.intent.writing`,
then atomically renames it to `.intent` and syncs the parent. Publication of the
closed, fingerprint-valid `.intent` is the one establishment point and creates
the logical `active_hold`. The owner then creates/syncs the deterministic empty
`<transaction-id>.pending` shell, atomically renames the identical `.intent`
bytes to its `intent.json`, and syncs both parents. The active hold projects
from exactly one of the established `.intent` or pending `intent.json` states.

Before the first rename, `.intent.writing` is non-authoritative scratch. On
restart it is removed and deterministically rebuilt only by a new request after
current context/evaluation; this explicit scratch cleanup is not authority
deletion, adoption, or consumed-key release. After the first rename, an exact
`.intent` is established and must move byte-for-byte to its deterministic
pending path without replanning the outcome or resampling time. Before that
move or any later recovery mutation, the owner reopens the current repository
authority and proves that the persisted request subject, operation context,
target instance, schema closure, intake binding, and promotion definition set
still match exactly. Exact same-request lookup resumes it only after this check,
and a different request under the key conflicts. The only legal establishment
crash states are: no file; `.intent.writing` only; established `.intent` with no
shell; established `.intent` plus an exactly zero-entry pending shell; or pending
`intent.json` with neither establishing file. Any dual established/pending
intent, nonempty shell, multiple scratch files, retained result or
tombstone plus establishing state, mismatched path/key/transaction/fingerprint,
or unsafe/extra entry refuses `conflicting_establishment_state` without
mutation. Tests inject before/after each write, sync, rename, shell create, and
parent-sync boundary. No separately persisted `active_hold` record is permitted;
the positive active-hold vector is the required lookup projection of the one
established intent.

A pending transaction directory contains only:

```text
intent.json
staged/<ordinal>-<output-token>.bin
native-publication.json        # promotion commit only
verified.json
native-publication-result.json # promotion commit only, absent before native observation
commit-marker.json            # absent until commit
evidence.json                 # absent until the marker is exact
```

`intent.json` is a closed JCS+LF object containing schema ID/version,
transaction ID, exact ledger scope/key/request fingerprints, operation-context
fingerprint, the complete operation-discriminated `request_subject` from which
the request fingerprint was derived, expected basis, `planned_outcome`,
refusal-or-null, ordered output descriptors, sampled timestamp if the operation
has one, and intent fingerprint. A promotion commit additionally binds
`expected_basis_presence` and the exact nullable fingerprint, byte length,
native identity, and digest-derived pre-call native version of its canonical
basis; all four basis values are null when presence is `absent` and all are
non-null when presence is `present`. The intent token is not independently
trusted: the platform-specific durable observation must reproduce it from its
structured components. Candidate request subjects retain their exact context
fingerprint; promotion request subjects additionally retain the exact canonical
artifact ref. Recovery validates those fields rather than trusting a
self-consistent journal fingerprint chain.

Promotion commit staging writes the closed `native-publication.json` before
`verified.json`. It binds `atomic-displaced-basis-v1`, the selected platform
adapter, transaction and intent, expected basis presence/fingerprint/length/
native identity/pre-call native version/complete platform observation, exact
canonical/candidate/displaced refs, and exact replacement bytes SHA-256/length/
native identity/pre-call native version/complete platform observation.
`verified.json` binds the publication-observation fingerprint. Other operations
may contain neither native record nor binding.

The normative structured component set, canonical digest, stable-pass
algorithm, permitted post-move fields, and metadata/side-stream refusal are
defined under “Structured native observation and digest authority” below.
Immediately before the first move, complete basis and replacement observations
must equal their persisted identity, bytes, length, link count, components, and
derived tokens. `primitive_mutated_same_object` is legal only after the trace
proves a successful native move of that same bound object and all
move-invariant fields remain exact. `unchanged_before_first_move` is legal only
when the full structured observation is exact. `external_change` and
`external_object` are never accepted as the expected basis or replacement.
Every observation handle is closed before the next move.
A commit intent has null refusal and its complete nonempty operation-specific
output set. A refusal intent has the exact closed refusal, null sampled time,
and zero outputs. Each commit output descriptor contains ordinal, token,
authority class (`subordinate_closure` is allowed only here), final ref, exact
bytes SHA-256, byte length, install mode, and receipt class. The intent
fingerprint excludes only itself.

`verified.json` is a closed JCS+LF object binding transaction/intent fingerprint,
planned outcome, and refusal-or-null. A commit uses
`verification_basis: staging | installed_complete` and the exact ordered
ref/digest/length list after close/reopen/no-follow verification. The second
basis is legal only when every installed output already matches intent exactly;
its refs are the final installed refs. A refusal uses only
`verification_basis: refusal_decision`, repeats the exact refusal, and has zero
staged outputs. `commit-marker.json` binds transaction/intent/verified
fingerprints, outcome, refusal-or-null, exact realized authoritative
ref/fingerprint pairs, ordered subordinate refs, and marker fingerprint. A
promotion commit and the special post-boundary publication conflict also bind
the exact native-publication-result fingerprint. A refused marker has no
outputs. Neither record contains wall-clock identity. `evidence.json` is the
closed internal evidence record from the control schema; it repeats the
outcome/refusal, repeats the native-result fingerprint when the marker has one,
and is derived from the exact marker and installed bytes (zero installed
artifact bytes for refusal). It is not a
canonical receipt or a mutable fourth semantic authority source. A refused
domain result must point to this exact evidence; outputless never means
evidence-less.

Every result evidence ref is operation-discriminated and repeats the identical
transaction ID: `intake-records/<intake transaction>.committed/evidence.json`,
`artifact-candidates/<candidate transaction>.committed/evidence.json`, or
`artifact-promotions/<promotion transaction>.committed/evidence.json` beneath
`.handbook/state/transactions/`. No shortened family, alias, or cross-family
lookup is valid for committed or refused results.

After exact installed outputs (the empty set for refusal), marker, evidence,
retained domain result, and ledger entry are durable, the owner atomically renames the transaction directory
from `<transaction-id>.pending` to `<transaction-id>.committed` in the same
family directory and syncs the parent. Pending-with-marker means commit-complete
but closeout-incomplete and recovery finishes evidence/result/ledger/rename.
Committed journals are retained; HCM-2.3 promises no deletion or compaction.
Staged files are non-authoritative after an exact marker and may be absent or
corrupt without changing the committed outcome, but unknown directory entries,
identity disagreement, unsafe types, or installed-output disagreement still
refuse.

Inventory is deterministic lexical order under the retained root and no-follow
at every component. Limits are:

| Inventory | Maximum |
|---|---:|
| pending transactions per operation token | 1,024 |
| committed journals per operation token | 4,096 |
| files per transaction directory, recursively | 16 |
| bytes per transaction directory | 16 MiB |
| all generic pending transaction bytes | 64 MiB |
| all retained committed journal bytes | 256 MiB |
| committed semantic records per family/instance | 4,096 |
| subordinate closure objects per family/instance | 8,192 |
| bound native-publication sibling evidence paths per pending promotion | 2 |
| all bound native-publication sibling evidence paths in the canonical parent | 2,048 |
| bytes per record/closure object | 1 MiB |
| bytes per native-publication sibling evidence file | 1 MiB |
| bytes per engine-owned intake/evaluate/candidate/promotion request document | 1 MiB |
| committed generic store bytes per instance | 256 MiB |
| directory depth below a selected generic store root | 8 |

`native-publication-result.json` is documentation authority, not current
implementation authority. The live
`validate_transaction_inventory` top-level allowlist admits
`intent.json`, `native-publication.json`, `verified.json`,
`commit-marker.json`, and `evidence.json`, but rejects this new result name.
The exact required future change is narrow admission only for an
`artifact_candidate_promote_*` transaction whose intent is a commit, whose
native observation is exact, and whose result schema/fingerprint/trace binds
that same transaction. It must remain forbidden for intake/candidate journals,
planned refusals, intent-free shells, and every state before an exact native
observation.

That edit is not authorized here. Nor are the coupled recovery and committed-
read edits that would consume the new inventory member. Exact depth-three live
GitNexus evidence is:

| Symbol | UID | Direct callers | Affected without / with tests | Processes | Modules |
|---|---|---:|---:|---:|---:|
| `validate_transaction_inventory` | `Function:crates/engine/src/artifact_lineage_store.rs:validate_transaction_inventory` | 2 | 16 / 28 | 7 | 2 |
| `GenericArtifactLineageStoreV1::recover_pending` | `Function:crates/engine/src/artifact_lineage_store.rs:GenericArtifactLineageStoreV1.recover_pending#1` | 1 | 9 / 21 | 5 | 2 |
| `GenericArtifactLineageStoreV1::verify_committed` | `Function:crates/engine/src/artifact_lineage_store.rs:GenericArtifactLineageStoreV1.verify_committed#1` | 3 | 12 / 24 | 5 | 2 |

All three results are `CRITICAL`. The direct caller of `recover_pending` is
`GenericArtifactLineageStoreV1::recover_locked`; the direct callers of
`verify_committed` are `recover_locked`, `require_exact_committed_chain`, and
`validate_recoverable_reachability`. The evidence commands, each also run
without `--include-tests`, are:

```text
npx gitnexus impact validate_transaction_inventory --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact validate_transaction_inventory --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact recover_pending --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact recover_pending --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_committed --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_committed --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
```

Review 5 additionally identified six exact CRITICAL structured-observation
surfaces: `native_bound_tokens`, `native_path_tokens`,
`native_metadata_subjects`, `observe_retained_regular_file`,
`same_native_metadata`, and `validate_native_publication_observation`. Their
exact UIDs, direct callers, affected symbols/processes/modules, and depth-three
commands with and without tests are preserved in the authority-repair proof.
Review 6 additionally identified seven exact CRITICAL retained-handle
publication surfaces:

| Symbol | UID | Direct callers | Affected without / with tests | Processes | Modules |
|---|---|---:|---:|---:|---:|
| `rename_store_path` | `Function:crates/engine/src/artifact_lineage_store.rs:rename_store_path` | 6 | 28 / 59 | 8 | 2 |
| `publish_replacement` | `Function:crates/engine/src/artifact_lineage_store.rs:publish_replacement` | 3 | 17 / 19 | 6 | 2 |
| `verify_compare_and_write_path` | `Function:crates/engine/src/artifact_lineage_store.rs:verify_compare_and_write_path` | 4 | 22 / 52 | 6 | 2 |
| `require_installed_guards_unchanged` | `Function:crates/engine/src/artifact_lineage_store.rs:require_installed_guards_unchanged` | 3 | 22 / 53 | 7 | 2 |
| `observe_installed_output` | `Function:crates/engine/src/artifact_lineage_store.rs:observe_installed_output` | 3 | 21 / 21 | 6 | 2 |
| `observe_prepared_replacement` | `Function:crates/engine/src/artifact_lineage_store.rs:observe_prepared_replacement` | 2 | 13 / 15 | 6 | 2 |
| `require_prepared_replacement_unchanged` | `Function:crates/engine/src/artifact_lineage_store.rs:require_prepared_replacement_unchanged` | 2 | 13 / 15 | 6 | 2 |

Their exact direct callers, process names, module names, and depth-four commands
with and without tests are preserved in the authority-repair proof. Explicit
operator authorization for all sixteen CRITICAL surfaces is a mandatory stop
before Rust, any implementation selector, staging, or commit. Partial
authorization does not authorize the coupled change.

Any over-count/size/depth condition refuses before recovery mutation. Duplicate,
extra, wrong-name, wrong-type, symlink, reparse point, non-regular, hard-link
alias, mutation-between-inventory-and-open, or outside-root
entry refuses. No ignored file or unknown extension is skipped.

The raw journal/store implementation is crate-private and exposes no public
unlocked or authority-free read surface. Public repository methods return only
owned safe DTOs after the repository-authority and generic locks are held,
markerless recovery has completed, and current authority has been revalidated.
Cached or previously decoded bytes never bypass those checks.

## Atomic canonical publication protocol

Promotion alone uses `atomic-displaced-basis-v1`. For transaction
`T = artifact_candidate_promote_<64hex>`, the three same-parent, same-volume
publication paths and one expected-present journal evidence file are:

| Symbol | Exact ref | Authority and creation rule |
|---|---|---|
| `C` | `.handbook/project/registry-brief.yaml` | canonical pathname; a promotion-installed value is not reader-authoritative until the transaction marker is exact |
| `R` | `.handbook/project/.registry-brief.yaml.generic-publish-<T>.candidate` | transaction evidence containing the exact replacement; fresh execution must create-new it as one regular, single-link file |
| `D` | `.handbook/project/.registry-brief.yaml.generic-publish-<T>.displaced` | transaction evidence for the displaced basis or retained replacement after rollback; fresh execution requires it absent |
| `B` | `.handbook/state/transactions/artifact-promotions/<T>.pending/expected-basis.backup` | expected-present evidence copied from the retained `C` source handle before the native boundary; expected-absent requires null |

`R`, `D`, and `B` are neither canonical truth, a semantic record, subordinate
closure, nor general-purpose scratch. They are journal-bound native-publication
evidence. `B` preserves exact expected bytes and the structured metadata
fingerprint before `C -> D`; it is never automatically restored, canonicalized,
read as current state, deleted, or cleaned. A same-name file without the exact
journal binding, an extra matching sibling, a non-regular type, link count other
than one, cross-volume target, reparse point/symlink, or identity/path
disagreement is `conflicting_transaction_state`.

Fresh execution creates `R` exclusively, writes the exact canonical bytes,
flushes the file, closes and reopens it no-follow, verifies bytes/length/native
identity/structured native observation/derived version, and makes its directory
entry durable. Expected-present execution then copies `C` from its retained
source handle into create-new `B`, flushes and durably installs `B`, and binds
its bytes, length, and `sha256` of the exact structured basis observation.
Recovery never creates, rewrites, restores from, or deletes an existing `R`,
`D`, or `B`; it accepts each only through the exact persisted binding. `D` must
be absent before the first native call. `native-publication.json` and
`verified.json` are durable before publication.

Immediately before the native call, the owner:

1. retains and rechecks `R` no-follow and requires the exact
   replacement bytes, length, identity, link count, structured platform
   observation, and digest-derived pre-call version;
2. retains `C` when present and requires
   exact expected presence, bytes, length, identity, link count, structured
   platform observation, and digest-derived pre-call version;
3. proves `D` absent and retains the exact source and destination parent
   directories across the call and post-observation; and
4. invokes only the selected no-replace, parent-anchored adapter while the source
   handle remains live.

Every successful move must open the destination from the retained destination
parent and prove it is the same live kernel object as the retained source before
constructing `native-publication-result.json` and `commit-marker.json`. Numeric
identity, bytes, and permitted ctime/change-time/USN deltas are corroboration,
not live continuity. The exact native-call step records this retained-source,
retained-parent, destination-relative-open proof. A failure preserves the live
source binding and invokes no fallback. Any lost handle, parent/ancestor
substitution, same-ID ABA, or unavailable proof is markerless
`refused_ambiguous`.

### Platform adapters

On supported Unix/Linux:

- resolve the canonical hierarchy with retained directory descriptors and
  `openat2` beneath/no-symlink constraints;
- expected absent calls
  `renameat2(source-parent,R,destination-parent,C,RENAME_NOREPLACE)`;
- expected present calls the same parent-anchored form for `C -> D`, proves
  `D` opened from the retained destination parent is the same live object as
  the still-open source descriptor, then does the equivalent retained-handle
  `R -> C`;
- retain the source descriptor plus both source and destination parent
  descriptors until the destination observation and continuity record are
  complete; and
- after every successful rename, `fsync` the destination parent before the
  completed result is durable.

Lack of filesystem support for `RENAME_NOREPLACE`, retained-fd `statx`, or
complete retained-fd extended-attribute observation is an
unsupported-platform refusal, not permission to fall back to ordinary rename.

On Windows:

Microsoft defines `UNICODE_STRING.Length`/`MaximumLength` in bytes and makes a
terminator optional; this contract selects the stricter unterminated exact-count
representation below. Microsoft also lists `FILE_DIRECTORY_FILE` as compatible
only with synchronous, write-through, backup-intent, and open-by-ID options, not
`FILE_OPEN_REPARSE_POINT`; directory role is therefore proved from the opened
handle rather than requested with that incompatible flag.

- keep only initial already-authorized root/volume acquisition, for which no
  retained parent exists, on `CreateFileW`;
- use one private `#[cfg(windows)]` `ntdll!NtCreateFile` `extern "system"` FFI
  helper for every subsequent component/child open and the post-rename
  destination rebound. It is local to `artifact_lineage_store.rs`, adds no
  dependency, public signature, static/global/thread-local state, or public API,
  and is a future unauthorized CRITICAL child of the already stopped
  retained-handle publication boundary rather than a seventeenth independent
  production surface;
- declare pointer-size-correct `repr(C)` `UNICODE_STRING` (`USHORT Length`,
  `USHORT MaximumLength`, UTF-16 `Buffer`), `OBJECT_ATTRIBUTES` (`ULONG Length`,
  `HANDLE RootDirectory`, `PUNICODE_STRING ObjectName`, `ULONG Attributes`,
  security pointers), and `IO_STATUS_BLOCK` (status/pointer union plus
  pointer-size `Information`) layouts, with `NTSTATUS` as signed 32-bit,
  `ACCESS_MASK`/`ULONG` as unsigned 32-bit, and the ABI calling convention
  `system`;
- pass the non-null role-specific retained parent in
  `OBJECT_ATTRIBUTES.RootDirectory`, a validated single-child
  `UNICODE_STRING` whose non-null buffer is exactly one unterminated counted
  UTF-16 child: even nonzero `Length <= 65534`, `MaximumLength == Length`, and
  exactly `Length` buffer bytes with no terminator;
- pass `OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE`, `FILE_OPEN`, null
  allocation/EA, zero EA length and file attributes. Retained-directory opens
  use exactly
  `FILE_OPEN_REPARSE_POINT | FILE_SYNCHRONOUS_IO_NONALERT` (`0x00200020`)
  without `FILE_DIRECTORY_FILE`; post-open handle observations prove directory
  type and reject any reparse point or tag before authority. Retained regular
  file roles additionally use `FILE_NON_DIRECTORY_FILE` (`0x00200060`);
- use `FILE_GENERIC_READ` for retained directories, enumerated children, and
  destination rebound observations; use `FILE_GENERIC_READ | DELETE` for the
  retained source; use only `FILE_SHARE_READ | FILE_SHARE_WRITE`, never
  `FILE_SHARE_DELETE`, for retained ancestors/sources/parents;
- reject before authority every null/wrong `RootDirectory`, multi-component or
  invalid name, wrong attributes/disposition/options/access/share, non-success
  `NTSTATUS`, non-success `IO_STATUS_BLOCK.Status`, or
  `IO_STATUS_BLOCK.Information != FILE_OPENED`; never fall back to a
  `CreateFileW` child open;
- enumerate each next component from its retained parent, compare enumerated
  and opened `FILE_ID_INFORMATION`, prove the retained-directory post-open
  type/reparse observations, and reject any reparse/junction substitution;
- retain the DELETE-capable source through the move and destination
  observation;
- call local
  `ntdll!NtSetInformationFile(source, IO_STATUS_BLOCK, FILE_RENAME_INFORMATION{ReplaceIfExists=FALSE, RootDirectory=retained_destination_parent, FileName=simple_child_name}, exact_counted_buffer_length, FileRenameInformation)`;
- keep the `NtSetInformationFile` declaration, `IO_STATUS_BLOCK` and
  `FILE_RENAME_INFORMATION` layouts, counted leaf-name buffer, and unsafe call
  inside `publish_replacement`; add no helper, dependency, Cargo edit, global,
  fallback primitive, or public API;
- reject an existing destination without overwrite and do not fall back to
  path-based `MoveFileExW`, ordinary rename, copy/delete, or `ReplaceFileW`; and
- open the destination from the same retained destination parent and prove it is
  the same live object as the retained source before recording completion.

The local rename FFI is the five-parameter `system` ABI
`NtSetInformationFile(HANDLE, PIO_STATUS_BLOCK, PVOID, ULONG,
FileRenameInformation)` returning signed 32-bit `NTSTATUS`.
`FILE_RENAME_INFORMATION` is `repr(C)` `BOOLEAN ReplaceIfExists`, `HANDLE
RootDirectory`, `ULONG FileNameLength`, then the counted UTF-16 `FileName`.
`ReplaceIfExists` is false, `RootDirectory` is the retained target parent, and
the passed `Length` is the checked `FileName` field offset plus the exact
unterminated leaf-name bytes. The DELETE-capable source and parent handles,
I/O-status storage, information buffer, and leaf storage remain live for the
synchronous call.

`ReplaceFileW` is explicitly forbidden for both forward and rollback
publication. Microsoft documents that it opens the replacement with no sharing
mode, merges creation time, DACLs, encryption/compression, object identifiers,
and named streams from the replaced object, and may leave those merges applied
after error 1177. HCM-2.3 does not define those merged objects as canonical
output and cannot authenticate an exact filesystem-independent merge. Errors
1175/1176/1177 are therefore not states in this adapter and have no live
vectors.

`GetFileInformationByHandleEx(FileIdInfo)` must return a complete 128-bit
identifier that is neither `0x00000000000000000000000000000000` nor
`0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF`. Microsoft documents that all zero means
the filesystem does not support a 128-bit file ID and all one means no unique
128-bit ID exists; both values must be ignored. HCM-2.3 therefore rejects both
at retained-source, enumerated/opened-child, destination-observation, and
same-live-object continuity roles and has no weaker fallback.

The same synchronous retained regular-file handle must query object-ID absence
through a second private `#[cfg(windows)]` `ntdll!NtFsControlFile`
`extern "system"` helper local to `artifact_lineage_store.rs`. It adds no
dependency, public signature/API, static/global/thread-local state, or cleanup
authority. Like the future private `NtCreateFile` helper, it is an unauthorized
CRITICAL child of the already stopped structured-observation/publication
boundaries, not a seventeenth independent surface.

The exact ten parameters are `FileHandle`, null `Event`, null `ApcRoutine`, null
`ApcContext`, mutable `IO_STATUS_BLOCK`, `FsControlCode`, null `InputBuffer`,
zero `InputBufferLength`, non-null mutable `OutputBuffer`, and
`OutputBufferLength`. The call uses `FSCTL_GET_OBJECT_ID (0x0009009C)`, an exact
64-byte `FILE_OBJECTID_BUFFER` (`ObjectId[16]`, `ExtendedInfo[48]`), and
initializes the I/O status block to `Status = 0xC0000001` and
`Information = ULONG_PTR_MAX`.

Microsoft's `NtFsControlFile` contract reports the immediate native call status
as its return value and the completed operation status/information through
`IO_STATUS_BLOCK`; synchronous handles avoid an asynchronous APC/Event path.
The protocol therefore records and validates both statuses rather than
converting the call to a Boolean/last-error boundary.

Object-ID absence has exactly three accepted completion traces:

1. immediate raw `STATUS_OBJECTID_NOT_FOUND (0xC00002F0)` while the complete
   `IO_STATUS_BLOCK` remains unchanged at its initialized
   `Status = STATUS_UNSUCCESSFUL` and `Information = ULONG_PTR_MAX` sentinel;
2. raw and final `IO_STATUS_BLOCK.Status` both
   `STATUS_OBJECTID_NOT_FOUND` with `Information == 0`; or
3. raw `STATUS_PENDING (0x00000103)` followed by completed
   `IO_STATUS_BLOCK.Status == STATUS_OBJECTID_NOT_FOUND` and
   `Information == 0`.

The immediate raw status is the completion authority only in the first exact
sentinel-preserving trace. `STATUS_PENDING` is never itself absence authority;
it must complete through the I/O status block. A pending call with either
sentinel unchanged, a partial sentinel change, or any other completed status or
length fails closed. Dual `STATUS_SUCCESS` with `Information == 64` means the
object ID is present and refuses. Every other sentinel, inconsistent raw/final
status, or information length fails closed before a move and becomes retained
ambiguity after a move. `DeviceIoControl` exposes a Boolean/`GetLastError`
boundary rather than the required raw NTSTATUS; it is not object-ID absence
authority and no fallback is allowed. Mutation between the two complete
observation passes likewise fails closed.

### Structured native observation and digest authority

`native-version-v1:<64hex>` is not writer-chosen or opaque. It is SHA-256 over
the RFC 8785 canonical UTF-8 encoding, with no trailing line feed, of the exact
`nativePlatformObservation` admitted by the control schema. The
`native-id-v1:<64hex>` token is likewise SHA-256 over the canonical identity
projection:

- Unix: `platform_family`, `file_type`, `device_id`, and `inode`;
- Windows: `platform_family`, `file_type`, `volume_serial_number_hex`, and
  complete nonsentinel 128-bit `file_id_hex`.

The durable pre-boundary observation record contains the complete expected
basis observation or null, the complete replacement observation, and both
derived tokens. Every regular `C/R/D` entry in the native-call trace and final
result contains the complete reopened observation and derived tokens. An absent
path contains null components. Token equality without successful recomputation
from those persisted components has no authority.

Unix observes the retained, no-follow file descriptor with
[`statx(fd, "", AT_EMPTY_PATH | AT_NO_AUTOMOUNT, ...)`](https://man7.org/linux/man-pages/man2/statx.2.html).
The canonical component set is device, inode, regular-file type, byte length,
link count exactly one, full mode, uid, gid, mtime seconds/nanoseconds, ctime
seconds/nanoseconds, and the complete extended-attribute set. The adapter uses
retained-fd [`flistxattr`](https://man7.org/linux/man-pages/man2/listxattr.2.html)
and [`fgetxattr`](https://man7.org/linux/man-pages/man2/getxattr.2.html), encodes
each raw attribute name as RFC 4648 Base64, records value byte length and
SHA-256, sorts by raw name bytes, and derives `entries_digest` over that exact
array. It performs two complete enumerate-and-read passes and accepts only
byte-identical passes. `ERANGE`, an unreadable value, a changed name/value set,
unsupported xattrs, incomplete `statx` fields, overflow, or any other
observation failure refuses; it never silently omits a namespace or attribute.

Windows obtains the complete component set from retained-handle APIs:

- [`GetFileInformationByHandleEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandleex)
  with `FileIdInfo`, `FileBasicInfo`, `FileStandardInfo`,
  `FileAttributeTagInfo`, and `FileStreamInfo`;
- [`GetSecurityInfo`](https://learn.microsoft.com/en-us/windows/win32/api/aclapi/nf-aclapi-getsecurityinfo)
  for owner, group, and DACL only; and
- [`FSCTL_READ_FILE_USN_DATA`](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_read_file_usn_data)
  for the last file USN only.

The persisted observation is volume serial, complete nonsentinel file ID,
`object_id_query: absent_status_objectid_not_found`, regular/reparse
status and null reparse tag, byte length, link count exactly one, creation time,
last-write time, change time, complete file-attribute bits, canonical owner/
group/DACL components and digest, exact UTF-16-code-unit-sorted
[`FILE_STREAM_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_stream_info)
inventory, and USN. Exactly the default `::$DATA` stream is permitted;
non-default streams refuse. SACL is deliberately
`excluded_not_observed_not_trusted`: it is never requested, never hashed, and
never treated as canonical-byte authority. Two complete handle observations
must agree before the pre-call record is accepted. Access denial, unsupported
information class, unstable results, malformed security data, a reparse point,
or an incomplete stream/security observation refuses. Object-ID presence,
unsupported query, access denial, or any status other than the exact absent
status also refuses.

For the same authenticated object, every structured field is a move invariant
except:

- Unix may change only `ctime`;
- Windows may change only `change_time_100ns` and `usn`.

Mode, uid, gid, mtime, xattrs, Windows creation/last-write time, attributes,
owner/group/DACL, stream inventory, file type, identity, size, and link count
must remain exact. The Windows
[`FILE_BASIC_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_basic_info)
change time is distinct from last-write time. The USN read operation does not
provide causal proof: Microsoft specifies that `Reason`, `TimeStamp`, and
`SourceInfo` in the returned record are invalid for this control code. A
permitted change-time/USN/ctime delta is therefore authorized only by the exact
successful no-replace trace, retained live-source-to-destination continuity,
and equality of every move-invariant component. Reusable numeric identity,
same-ID ABA, an arbitrary token, or USN alone never proves causality.

A metadata, xattr, DACL, attribute, last-write, or named-stream delta is
outputless and mutation-free. If detected before the first move, the adapter
invokes no native move and refuses. If detected after any move, it writes no
result or marker, performs no further move (including restoration), retains all
pre/post observations and `C/R/D` evidence, withholds every reader, and returns
`refused_ambiguous`. Required observation unavailable before a move is an
unsupported-platform refusal; unavailable after a move is retained ambiguous
evidence. Neither case authorizes cleanup or a weaker opaque token fallback.

Before the native boundary, only the write-capable producer handle that created
`R` calls `FlushFileBuffers`; it then reopens as the DELETE-capable retained
source. Observation handles never flush. Windows claims no directory-fsync or
write-through rename guarantee for `NtSetInformationFile`; publication
authority instead requires the completed live-handle proof, then durable native
result and marker. If a crash loses those handles before a completed continuity
record is durable, recovery must not infer success from the `C/R/D` tuple,
numeric identity, bytes, or version deltas. It retains `C/R/D/B`, writes no
result or marker, grants no cleanup, and returns outputless
`refused_ambiguous`. If the exact completed native result is durable, recovery
may finish only its already-bound non-native suffix.

### Exhaustive `C/R/D` and continuity classifier

In this table `Ø` is absent; `E0` and `N0` are the exact expected-basis and
replacement pre-call observations; `E'` and `N'` are the same live objects
proved while their retained source handles are still open; `X` is an
unauthenticated regular observation; and `U` is unsafe, unknown, inconsistent,
or supported only by a reopened numeric identity. `L` is the exact durable
completed continuity record; `¬L` is absent or incomplete. `B` is retained
expected-basis evidence and never changes the classifier.

| Expected basis | Observable `C / R / D` | Continuity | Authorized action or refusal | Cleanup |
|---|---|---|---|---|
| absent | `Ø / N0 / Ø` | live source and parents retained | invoke the parent-anchored no-replace adapter once | forbidden |
| absent | `N' / Ø / Ø` | `L`, destination proved from retained source before result construction | persist authorized result | forbidden |
| absent | `X / N0 / Ø` | destination existed; source handle proves no move | stable outputless conflict; retain both | forbidden |
| absent | any post-call tuple | `¬L`, including crash or same-ID reopen | markerless `refused_ambiguous`; no retry or inference | forbidden |
| present | `E0 / N0 / Ø` | both sources and parents retained; `B` exact | invoke parent-anchored `C -> D` once | forbidden |
| present | `Ø / N0 / E'` | first-step continuity exact and second source still retained | invoke parent-anchored `R -> C` once | forbidden |
| present | `N' / Ø / E'` | `L` covers both moves | persist authorized result; retain `D` and `B` | forbidden |
| present | `X / N0 / Ø` | claim did not move the retained source | stable outputless competing-basis conflict | forbidden |
| present | any `D=X`, parent mismatch, zero ID, object-ID failure, metadata drift, same-ID ABA, duplicate, or `U` | invalid or incomplete | markerless `refused_ambiguous`; no move, result, marker, restoration, or inference | forbidden |
| present | any post-call tuple after handle loss or crash | `¬L`, regardless of bytes/identity/version equality | markerless `refused_ambiguous`; retain `C/R/D/B` and do nothing | forbidden |

`native-publication-result.json` binds the pre-boundary observation and verified
fingerprints, platform adapter, expected/replacement stable bindings and
complete pre-call platform observations and derived versions, the exact ordered
native-call trace, per-step retained-source/parent/destination continuity proof,
exact before/after `C/R/D` observations including complete platform components,
post-move version transitions, exact `B` binding or null, basis/publication
disposition, withheld reader authority, and `cleanup_authority: forbidden`. A
promotion commit marker and its evidence repeat the result fingerprint.
`authorized` requires:

- expected absent: `C=N'`, `R=Ø`, and `D=Ø`; or
- expected present: `C=N'`, `R=Ø`, and `D=E'`.

`refused_basis_conflict` is permitted only when a live retained source proves no
native move occurred and a stable pre-call destination/source conflict is
exact. Once a move may have occurred, absent completed live continuity cannot
be reduced to a basis conflict. `D`, `B`, and every unexpected object can never
be restored, adopted, deleted, or cleaned by recovery. Only a completed native
result permits the special outputless
`commit-marker.json` with `outcome: refused` after a commit-planned promotion
intent; a committed-outcome marker is forbidden. `refused_ambiguous` permits
neither marker nor further mutation. Candidate, displaced, and backup evidence
has no
automatic cleanup authority in fresh execution, recovery, successful commit,
conflict refusal, or later reads.

### Authorized-reader boundary

Every public canonical read holds repository authority and the generic lock,
inventories every pending/committed promotion journal that names `C`, and runs
this classifier before opening `C`. A pending transaction, markerless
replacement, incomplete rollback/forward state, invalid native record, or
ambiguous evidence withholds canonical bytes. For a promotion-installed
canonical value, the reader requires the exact intent →
`native-publication.json` → verified → `native-publication-result.json` →
commit-marker → evidence → domain-result → retained-ledger chain, then reopens
`C` no-follow and proves its replacement bytes/identity plus current
path-to-handle binding before returning. A later external canonical change is a
new current basis only after no pending protocol state remains; it is never
mistaken for completion of the older promotion.

The expected-present claim deliberately creates the intermediate
`C=Ø,R=N0,D=E'|X` state. That state is never transient authorized truth:
repository/generic authority is held across the sequence, and every authorized
reader inventories and resolves the naming transaction before attempting to
open `C`. A direct pathname reader outside this owner boundary has no Handbook
authority. Crash recovery applies the same table before any canonical bytes are
returned.

The deterministic suite injects changed bytes, deletion, different-file
replacement, Unix/Windows same-ID ABA, Windows ancestor and destination-parent
rename, junction substitution, zero/all-ones file ID, invalid `NtCreateFile`
retained-parent child/rebound traces, object-ID presence/query failure and
between-pass mutation, backup loss/mismatch, and all prior structured
metadata changes. The exact 81-row matrix treats every crash without a durable
completed continuity record as markerless, outputless, non-retryable
`refused_ambiguous`, regardless of rebound `C/R/D`. No recovery move or result
is inferred from a reusable numeric ID.

### Executable matrix and chain gate

`generic-artifact-control-vectors-v1.0.json` is the machine authority for this
table. Its `atomic_publication_matrix_checker` requires exactly 81 named rows,
each with separate `unix` and `windows` cells (162 platform cells total), and
forbids wildcard platform keys. Thirty exact source-race rows cover both
expected-basis states. Its four `publication_chain_contracts` require the
complete expected-present Unix success, expected-present Windows success,
expected-present Unix outputless-conflict, and expected-absent Unix
source-deletion outputless-conflict chains from intent through retained ledger.

The deterministic checker embedded in the authority-repair proof is a release
gate. It fails on a missing, duplicate, or extra row/chain ID; absent Unix or
Windows coverage; `all`, `any`, or `*` platform keys; an incomplete action,
tuple, diagnostic, durability, classifier, result, retry, reader, or cleanup
cell; a missing record reference; a broken transaction/fingerprint edge; a
non-outputless conflict suffix; any fingerprint replay mismatch; or any
declared schema/semantic rejection vector that was not actually mutated and
rejected. It also rejects any `D=X` restoration/adoption, missing or weakened
exact replacement-source race row, unauthenticated `C` or `R` authority,
missing/mismatched `B`, crash inference, numeric-only continuity, zero/all-ones
Windows file ID, a non-absent object-ID status, a retained-parent child/rebound
open not using the exact `NtCreateFile` contract, a retained directory whose
post-open type/reparse state is not exact, an object-ID query not using the exact
  synchronous `NtFsControlFile` immediate-or-completed status contract, or a Windows move
not using the exact retained-handle
  `NtSetInformationFile(FileRenameInformation)` form. The checker executes all 39
declared `NtCreateFile` mutations and all 33 declared `NtFsControlFile`
mutations. Schema admission alone cannot satisfy this gate.

Primary native authority:

- Linux [`renameat2(2)`](https://man7.org/linux/man-pages/man2/renameat2.2.html)
  for same-filesystem no-replace moves and
  [`fsync(2)`](https://man7.org/linux/man-pages/man2/fsync.2.html) for the
  separate parent-directory durability requirement;
- Linux [`openat2(2)`](https://man7.org/linux/man-pages/man2/openat2.2.html) and
  [`statx(2)`](https://man7.org/linux/man-pages/man2/statx.2.html) for bounded
  no-follow resolution and native identity/version observations;
- Microsoft [`NtSetInformationFile`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntsetinformationfile)
  with [`FILE_RENAME_INFORMATION`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_file_rename_information);
  the live page expressly requires the retained target-directory
  `RootDirectory` for a relative simple `FileName`;
- Microsoft [`NtCreateFile`](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntcreatefile),
  [`OBJECT_ATTRIBUTES`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_object_attributes),
  [`UNICODE_STRING`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_unicode_string),
  and [`IO_STATUS_BLOCK`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ns-wdm-_io_status_block)
  for retained-parent-relative child/rebound opens;
- Microsoft [`128-bit file ID`](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/98860416-1caf-4c80-a9ab-8d61e1ccf5a5)
  sentinel authority and initial-root-only
  [`CreateFileW`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew);
- Microsoft
  [`GetFileInformationByHandleEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandleex),
  [`FILE_ID_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info),
  [`NtFsControlFile`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntfscontrolfile),
  [`FSCTL_GET_OBJECT_ID`](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_get_object_id),
  [`FILE_OBJECTID_BUFFER`](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ns-winioctl-file_objectid_buffer),
  [`DeviceIoControl`](https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-deviceiocontrol)
  as evidence that its Boolean/`GetLastError` boundary is not raw-NTSTATUS
  authority,
  [`FSCTL_READ_FILE_USN_DATA`](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_read_file_usn_data),
  and [`FlushFileBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers); and
- Microsoft [`ReplaceFileW`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
  only as primary evidence for rejecting its no-sharing replacement open,
  metadata/stream merge, and partial 1177 behavior from this protocol.

## Total recovery table

The exact output sets are:

- intake: `V = {every distinct cited value object}` plus `I = {intake record}`;
- candidate: `N = {normalized content}`, `R = {validation result}`, and
  `C = {candidate record}`; and
- promotion: `Y = {canonical YAML}` and `P = {promotion record}`.

An established refusal has the exact output set `F = {}`. It never enters the
commit-output subset table below; it follows the separate refusal table after
it. Discovery of any staged, semantic-record, subordinate-closure, or canonical
artifact output for a refusal is `conflicting_refusal_state` and preserves all
bytes without mutation.

For intake, every member of the finite power set of `V ∪ I` is an installed
subset; for candidate the eight subsets of `{N,R,C}` are explicit test cases;
for promotion the four subsets of `{Y,P}` are explicit test cases. Recovery is
total over each set:

| Intent | Verified staging | Installed authoritative/closure set | Marker | Outcome |
|---|---|---|---|---|
| absent | any | any | any | refuse `orphan_transaction_state`; write nothing |
| valid | incomplete/invalid | empty or any proper subset | absent | refuse `incomplete_staging`; preserve all bytes |
| valid | incomplete/invalid | complete exact byte-identical set | absent | create `verified.json` with `installed_complete`, write marker/evidence/result/ledger, then rename committed |
| valid | exact complete | empty | absent | install every output in ordinal order, sync parents, write marker |
| valid | exact complete | any proper exact byte-identical subset | absent | verify installed subset, install exact complement, sync parents, write marker |
| valid | exact complete | complete exact byte-identical set | absent | write marker/evidence/result/ledger, then rename committed |
| valid | any | complete exact byte-identical set | exact in `.pending` | verify intent/marker/installed bytes, finish any missing evidence/result/ledger/rename, then replay original result |
| valid | any | empty or any proper subset | exact | refuse `invalid_commit_marker`; write nothing |
| valid | any | complete exact byte-identical set | exact in `.committed` | verify intent/marker/installed bytes and retained result, then replay original result |
| valid | any | any | present but invalid/mismatched | refuse `invalid_commit_marker`; write nothing |
| invalid/mismatched | any | any | any | refuse `invalid_transaction_intent`; write nothing |
| valid | exact complete | any set with mismatched or excess output | any | refuse `conflicting_transaction_state`; write nothing |

Pre-boundary established-refusal recovery is total over the exact durable
prefixes below. Every named record must repeat the identical closed refusal and
its exact operation/transaction/request/context/fingerprint bindings:

| Durable refusal prefix | Recovery outcome |
|---|---|
| `active_hold + intent` | write exact `verified.json` with `refusal_decision`, then continue the suffix |
| previous + `verified` | write the exact refused marker, then continue the suffix |
| previous + `marker` | write exact outputless evidence, refused result, retained-result ledger, then rename committed |
| previous + `evidence` | write exact refused result, retained-result ledger, then rename committed |
| previous + `result` | write exact retained-result ledger, then rename committed |
| previous + `retained_result` ledger | rename the journal committed, then replay the exact refused result |
| previous + committed rename | verify the complete prefix and replay the exact refused result |
| any missing-middle, mismatched, duplicate pending/committed, or extra/output-bearing state | refuse `conflicting_refusal_state`; preserve all bytes and write nothing |

The final-boundary publication conflict is the only exception to the
planned-refusal prefix above. Its intent and verified stage remain the original
commit plan with null refusal and complete staged outputs. After the classifier
has reached one exact stable conflict row (including an authenticated expected
`D` plus a later competing `C`, or the exact expected basis restored after
replacement-source loss/substitution), recovery follows only:

| Durable publication-conflict prefix | Recovery outcome |
|---|---|
| exact verified commit plan + stable conflict `C/R/D` and exact call trace, no native result | persist the exact `refused_basis_conflict` native result |
| previous + native result | write outputless refused marker with `publication_basis_conflict` and native-result fingerprint |
| previous + marker | write matching outputless evidence/result/retained ledger, then rename committed |
| any later exact ordered prefix | resume only its missing suffix and replay the conflict |
| native result/marker mismatch, any artifact output, missing/interleaved trace step, or unstable/ambiguous tuple | refuse without mutation or reader authority |

The active hold is the intent projection created at the single `.intent`
publication boundary. Crashes before it leave no hold; crashes after it preserve
and move the same intent bytes, then resume only the missing row suffix.
The refused result becomes externally replayable only with its exact evidence
and retained-result ledger durable. Replacing retained refusal with its exact
tombstone is a separate locked compare-and-write after the retention deadline;
it never changes or removes the committed journal and never reruns evaluation.

Canonical YAML follows only the atomic canonical publication protocol above
after every other output is staged and verified. The generic `{Y,P}` subset
table is refined for promotion: `{Y}` is complete only with an exact authorized
native-publication result, while a stable final-boundary basis conflict follows
the protocol's outputless terminal-refusal suffix. Any other canonical-only,
native-result-free, ambiguous, or native-binding-mismatched subset refuses
without mutation. Intake and candidate continue to use create-new/no-replace
and the ordinary complement rule.
Every transition reopens and verifies installed bytes before mutation, syncs
each installed file and parent, and is restart-idempotent. Marker creation is
followed only by the deterministic evidence/result/ledger/rename suffix; a
crash after any step resumes the missing suffix. An exact retained result with
a pending marker cannot be regenerated differently, and a committed journal
without its exact retained ledger result refuses. Readers acquire the lock, run
the atomic publication classifier and this table across both suffixes, validate
every staged or installed runtime
record against the current selected repository context, and expose only
marker-backed semantic records/canonical truth. A schema-valid, fully
refingerprinted journal or output that names stale or forged context,
definitions, schema closure, intake binding, or canonical authority refuses
without state delta. One committed intake without a candidate is valid history,
not a partial transaction.

All four engine-owned byte-document entry points share the exact 1 MiB bound.
The bound is checked before YAML/JSON parsing; intake evaluation checks it
before repository recovery can create lock state. Exactly 1 MiB continues to
the duplicate-safe parser, while 1 MiB + 1 byte refuses before any repository,
journal, evidence, receipt, or canonical mutation. Intake semantic records
stamp `consumer.version` from the engine's compile-time repository `VERSION`;
the public intake API and request subject have no caller-controlled version
field, and persisted intake validation requires that exact engine release.

The post-marker suffix is itself total for both outcomes over evidence `E`, retained result `D`,
ledger entry `L`, and committed rename `Q`: the only admissible installed sets
are the ordered prefixes `{}`, `{E}`, `{E,D}`, `{E,D,L}`, and `{E,D,L,Q}` with
every present byte exact. Recovery installs the missing suffix in that order.
Any non-prefix subset, mismatch, duplicate pending/committed directory, result
without evidence, ledger without result, committed rename without ledger, or
changed installed authoritative output refuses without mutation. Tests inject
a crash before and after every `E/D/L/Q` durability boundary.

## Mandatory negative and compatibility proof

Before GREEN, tests must prove:

- every prior record-version fixture remains byte-identical and each new/old
  cross-version or store-family substitution refuses;
- every new closed enum/null combination rejects unknown, Charter, governed,
  approval-injected, and result-count variants;
- exact positive fingerprints/IDs/refs reproduce the normative vectors and
  one-byte/key/order/time mutations change identity or refuse as specified;
- evaluate and candidate validate perform zero writes, and append cannot retain
  an invalid or preview-mismatched candidate;
- all recovery-table rows, all candidate/promotion subsets, every intake subset
  for two values, every established-refusal prefix, all inventory boundaries,
  repeated restarts, retained committed/refused replay, tombstones, and
  concurrent same/different requests pass;
- no closure object has an independent receipt, list/read visibility, authority,
  adoption, cleanup, or deletion path; and
- direct CLI replay/currentness/tombstones pass, the owner identity API has no
  transport metadata input, and Phase 4 execution proof remains explicitly
  deferred without claiming that Phase 4 is delivered.
