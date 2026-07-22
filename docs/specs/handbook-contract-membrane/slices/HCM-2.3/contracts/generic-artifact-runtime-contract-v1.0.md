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
pending path without re-resolving context or resampling time. Exact same-request
lookup resumes it and a different request under the key conflicts. The only
legal establishment crash states are: no file; `.intent.writing` only;
established `.intent` with no shell; established `.intent` plus empty shell; or
pending `intent.json` with neither establishing file. Any dual established/
pending intent, nonempty shell, multiple scratch files, retained result or
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
verified.json
commit-marker.json            # absent until commit
evidence.json                 # absent until the marker is exact
```

`intent.json` is a closed JCS+LF object containing schema ID/version,
transaction ID, exact ledger scope/key/request fingerprints, operation-context
fingerprint, expected basis, `planned_outcome`, refusal-or-null, ordered output
descriptors, sampled timestamp if the operation has one, and intent fingerprint.
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
refused marker has no outputs. Neither record contains wall-clock identity.
`evidence.json` is the closed internal evidence record from the control schema;
it repeats the outcome/refusal and is derived from the exact marker and
installed bytes (zero installed artifact bytes for refusal). It is not a
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
| bytes per record/closure object | 1 MiB |
| committed generic store bytes per instance | 256 MiB |
| directory depth below a selected generic store root | 8 |

Any over-count/size/depth condition refuses before recovery mutation. Duplicate,
extra, wrong-name, wrong-type, symlink, reparse point, non-regular, hard-link
alias where detectable, mutation-between-inventory-and-open, or outside-root
entry refuses. No ignored file or unknown extension is skipped.

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

Established-refusal recovery is total over the exact durable prefixes below.
Every named record must repeat the identical closed refusal and its exact
operation/transaction/request/context/fingerprint bindings:

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

The active hold is the intent projection created at the single `.intent`
publication boundary. Crashes before it leave no hold; crashes after it preserve
and move the same intent bytes, then resume only the missing row suffix.
The refused result becomes externally replayable only with its exact evidence
and retained-result ledger durable. Replacing retained refusal with its exact
tombstone is a separate locked compare-and-write after the retention deadline;
it never changes or removes the committed journal and never reruns evaluation.

Canonical YAML uses replace-if-current only after every other output is staged
and verified. If canonical installation succeeds and promotion record is absent,
the exact `{Y}` row completes `P`; if either byte differs, recovery refuses.
Intake and candidate use create-new/no-replace and the same complement rule.
Every transition reopens and verifies installed bytes before mutation, syncs
each installed file and parent, and is restart-idempotent. Marker creation is
followed only by the deterministic evidence/result/ledger/rename suffix; a
crash after any step resumes the missing suffix. An exact retained result with
a pending marker cannot be regenerated differently, and a committed journal
without its exact retained ledger result refuses. Readers acquire the lock, run
this table across both suffixes, and expose only marker-backed semantic records/
canonical truth. One committed intake without a candidate is valid history,
not a partial transaction.

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
