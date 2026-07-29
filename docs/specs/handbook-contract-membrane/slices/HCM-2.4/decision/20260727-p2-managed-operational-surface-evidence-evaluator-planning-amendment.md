# P2 managed-operational-surface evidence/evaluator planning amendment

> Superseded historical design. The accepted greenfield decision removes this
> evaluator/evidence system from the active product.

Status: **six-row transaction-store matrix with checked 4096-entry and
131072-byte canonical-envelope ceilings frozen for one fresh independent review
and later distinct P2S/P2A human approval; no runtime implementation authority**

Date: 2026-07-27

## Decision boundary

This amendment freezes the smallest complete versioned design needed to replace
the shipped resolver's current `unresolved` / `EvidenceContractUnavailable`
result for
`handbook.condition.project.managed-operational-surface@1.0.0`. It changes no
released definition, fingerprint, production code, schema, CLI, compiler,
flow, setup, doctor, or P6 surface.

The released condition remains authoritative:

- accepted input classes are `authoritative_fact_ref` and
  `admitted_evidence_ref`;
- minimum independent current bases is one;
- Environment Context is a self-reference exclusion;
- outcome precedence is refused, unresolved, stale, unknown, false, true; and
- no outcome may be coerced to a Boolean.

The proposed evaluator definition admits only `admitted_evidence_ref` in V1.
`authoritative_fact_ref` remains released future vocabulary but has no V1
transport, request, record branch, or runtime path. Bytes claiming that class
fail the V1 closed schema as `refused` / `evidence_refused`; the evaluator does
not advertise an unreachable `input_adapter_unavailable` branch.

Human approval is required before creating or editing any runtime, schema, or
definition file named below. A production native-authenticator implementation
is not proven locally. It therefore remains a separate feasibility and
authority gate before any producer runtime packet.

## Closed schema set and common scalar grammar

The proposal adds exactly these new refs, all JSON Schema draft 2020-12,
`additionalProperties: false`, `unevaluatedProperties: false`, and with every
listed key required even when its value is nullable:

1. `handbook.schemas.project-condition-admitted-evidence-source@1.0.0`;
2. `handbook.schemas.project-condition-evidence@1.0.0`;
3. `handbook.schemas.project-condition-evidence-head@1.0.0`;
4. `handbook.schemas.project-condition-evidence-transaction@1.0.0`;
5. `handbook.schemas.project-condition-evidence-challenge@1.0.0`;
6. `handbook.schemas.project-condition-evidence-assertion@1.0.0`;
7. `handbook.schemas.project-condition-evidence-use-transition@1.0.0`; and
8. `handbook.schemas.project-condition-evidence-evaluation-closure@1.0.0`.

Every record uses UTF-8 RFC 8785 canonical JSON plus one LF. Unless a field has
a narrower constant below:

- `Fingerprint` is `^sha256:[0-9a-f]{64}$`;
- `UtcSecond` is a valid UTC RFC 3339 instant with exact
  `YYYY-MM-DDTHH:MM:SSZ` form and no fraction or offset;
- `ExactRef` is
  `^[a-z][a-z0-9.-]{2,255}@[0-9]+\.[0-9]+\.[0-9]+$`;
- `Identifier` is `^[a-z0-9](?:[a-z0-9-]{0,62}[a-z0-9])?$`;
- `SemanticRef` is NFC, 1 through 256 UTF-8 bytes, contains no slash,
  backslash, dot segment, drive prefix, NUL, control character, or whitespace;
- a sequence is an integer from 1 through 4096; and
- a byte count is an integer from 0 through 131072.

No string is trimmed, case-folded, path-normalized, or otherwise repaired.

## Admitted source record

The producer accepts a typed `AdmittedEvidenceSourceCandidateV1`; it never
accepts a source fingerprint or another authority identity. The candidate
fields are exactly `source_id`, `observed_at_utc`, `valid_until_utc`,
`claim_kind`, `surface_kind`, and `surface_ref`. The producer supplies the
condition and retained repository identities.

The producer validates that candidate, then materializes this exact immutable
source record:

| Field | Exact V1 contract |
|---|---|
| `$schema` | `handbook.schemas.project-condition-admitted-evidence-source@1.0.0` |
| `schema_id` | `handbook.project-condition-admitted-evidence-source` |
| `schema_version` | `1.0` |
| `source_id` | `Identifier` |
| `source_kind` | `operator_responsibility_attestation` |
| `condition_ref` | exact managed-operational-surface condition ref |
| `repository_identity_fingerprint` | `Fingerprint`, exact retained repository identity |
| `observed_at_utc` | `UtcSecond` |
| `valid_until_utc` | `UtcSecond` |
| `claim_kind` | `qualifying_responsibility_present`, `no_qualifying_responsibility`, or `unknown` |
| `surface_kind` | `runtime`, `deployment`, `operated_automation`, or `operational_integration` only for a qualifying-positive claim; otherwise null |
| `surface_ref` | `SemanticRef` only for a qualifying-positive claim; otherwise null |
| `source_fingerprint` | computed `Fingerprint` |

`source_fingerprint` is SHA-256 over RFC 8785 canonical JSON containing every
source-record key and value except `$schema` and `source_fingerprint`. Its
immutable relative ref is exactly
`sources/admitted-evidence_<64 lowercase source-fingerprint hex>.json`.
The producer derives both values; a public or private producer call has no
parameter for either. An existing derived path is accepted only when its
retained canonical bytes are byte-identical.

`observed_at_utc < valid_until_utc` and their difference is at most 2592000
seconds. A source cannot name Environment Context, the evidence/head records,
a profile flag, an `applicability_basis`, or an artifact as its `source_id` or
`surface_ref`.

## Evidence record and monotonic head

Evidence records are immutable. The maximum serialized record size is 131072
bytes, the maximum source record size is 16384 bytes, and `inputs` contains 1
through 16 rows sorted by `input_id`. Duplicate `input_id`, `source_ref`,
`source_fingerprint`, or `(claim_kind, surface_ref)` after source resolution is
refused.

The exact evidence-record keys and values are:

| Field | Exact V1 contract |
|---|---|
| `$schema` | `handbook.schemas.project-condition-evidence@1.0.0` |
| `schema_id` | `handbook.project-condition-evidence` |
| `schema_version` | `1.0` |
| `condition_ref` | exact managed-operational-surface condition ref |
| `condition_definition_fingerprint` | recomputed released definition fingerprint |
| `repository_identity_fingerprint` | exact retained repository identity |
| `evidence_sequence` | sequence |
| `previous_record_ref` | null at sequence 1; otherwise exact evidence-record relative-ref grammar |
| `previous_record_fingerprint` | null at sequence 1; otherwise `Fingerprint` |
| `inputs` | sorted closed input rows below |
| `producer_verification` | closed producer-verification object below |
| `record_subject_fingerprint` | computed `Fingerprint` |
| `record_fingerprint` | computed `Fingerprint` |

Each input row has exactly:

| Field | Exact V1 contract |
|---|---|
| `input_id` | `Identifier` |
| `input_class` | `admitted_evidence_ref` |
| `source_ref` | `^sources/admitted-evidence_[0-9a-f]{64}[.]json$` |
| `source_fingerprint` | `Fingerprint`; its hex must equal the source-ref suffix |

All source, record, challenge, and assertion relative refs below resolve only
under `.handbook/state/project-condition-evidence/`. The record relative ref is
exactly
`records/managed-operational-surface_<64 lowercase record-fingerprint hex>.json`.
Sequence 1 has both previous fields null. Sequence N greater than 1 has both
non-null, points to the retained immutable sequence N-1 record, and matches its
recomputed fingerprint. The evaluator replays the chain to sequence 1, refuses
a fork, gap, duplicate sequence, cycle, or more than 4096 records, and never
discovers a filename.

`record_subject_fingerprint` is SHA-256 over RFC 8785 canonical JSON with
exactly:

```text
{
  schema_id,
  schema_version,
  condition_ref,
  condition_definition_fingerprint,
  repository_identity_fingerprint,
  evidence_sequence,
  previous_record_ref,
  previous_record_fingerprint,
  inputs
}
```

`record_fingerprint` is SHA-256 over RFC 8785 canonical JSON containing every
evidence-record key and value except `record_fingerprint`. It therefore binds
the schema ref, subject fingerprint, and complete producer verification.

The only mutable selector is
`.handbook/state/project-condition-evidence/managed-operational-surface-head.json`.
Its maximum size is 8192 bytes and its exact fields are:

| Field | Exact V1 contract |
|---|---|
| `$schema` | `handbook.schemas.project-condition-evidence-head@1.0.0` |
| `schema_id` | `handbook.project-condition-evidence-head` |
| `schema_version` | `1.0` |
| `condition_ref` | exact managed-operational-surface condition ref |
| `repository_identity_fingerprint` | exact retained repository identity |
| `evidence_sequence` | sequence |
| `current_record_ref` | exact evidence-record relative-ref grammar |
| `current_record_fingerprint` | `Fingerprint`, matching the ref suffix |
| `credential_id_hash` | `Fingerprint` |
| `authenticator_use_head_ref` | `^authenticator-use-heads/[0-9a-f]{64}[.]json$` |
| `authenticator_use_head_fingerprint` | `Fingerprint` |
| `previous_evidence_head_fingerprint` | null at sequence 1; otherwise `Fingerprint` |
| `head_fingerprint` | computed `Fingerprint` |

`head_fingerprint` covers every head key and value except `$schema` and
`head_fingerprint`. At evaluation, the selected head must match the record and
the credential's current retained authenticator-use head exactly. A later use
of that credential intentionally makes this evidence unresolved until it is
republished.

The rollback-resistant live-store selector is an immutable transaction chain
under
`.handbook/state/project-condition-evidence/transactions/`. Each committed
transaction has a maximum size of 8192 bytes and exactly:

| Field | Exact V1 contract |
|---|---|
| `$schema` | `handbook.schemas.project-condition-evidence-transaction@1.0.0` |
| `schema_id` / `schema_version` | `handbook.project-condition-evidence-transaction` / `1.0` |
| `evidence_sequence` | sequence |
| `previous_transaction_ref` | null at sequence 1; otherwise exact transaction-ref grammar |
| `previous_transaction_fingerprint` | null at sequence 1; otherwise `Fingerprint` |
| `record_ref` / `record_fingerprint` | exact current immutable record ref/fingerprint |
| `evidence_head_fingerprint` | exact committed result evidence head |
| `credential_id_hash` | `Fingerprint` |
| `authenticator_use_head_ref` / `authenticator_use_head_fingerprint` | exact committed result use head |
| `producer_verification_fingerprint` | exact current record verification fingerprint |
| `committed_at_utc` | exact producer `verified_at_utc` |
| `transaction_fingerprint` | computed `Fingerprint` |

`transaction_fingerprint` covers every key except `$schema` and itself. Its ref
is exactly
`transactions/transaction_<four-digit zero-padded sequence>_<64 lowercase
transaction-fingerprint hex>.json`.

The evaluator safely enumerates only that dedicated transactions directory.
Every entry must be a no-follow regular file with the exact filename grammar;
an unknown entry, alias, duplicate sequence, duplicate fingerprint, invalid
record, fork, gap, cycle, or more than 4096 entries is refused. It replays the
unique chain from sequence 1 and selects the unique maximal committed
transaction. The mutable evidence head, immutable record, and current
authenticator-use head must all equal that maximal transaction. Restoring both
mutable heads to an older mutually consistent pair while a newer immutable
transaction remains is therefore refused. This bounded transaction-store
enumeration is the only filename discovery in V1; all source and dependency
refs remain exact and non-discovered. A full external repository snapshot
rollback that also removes every newer immutable transaction is outside local
detectability and is not claimed as proof.

Restoring an older record/evidence head against a newer authenticator-use head,
restoring both mutable heads while a newer transaction remains, or forking or
truncating either immutable chain is refused.

All reads and writes are repository-relative regular-file-only, no-follow at
every component, use retained-handle identity checks, and reject substitution,
inode ABA, links, special files, path escapes, case aliases, and non-canonical
bytes.

## Challenge, assertion, and producer verification

The exact challenge fields are:

| Field | Exact V1 contract |
|---|---|
| `$schema` | `handbook.schemas.project-condition-evidence-challenge@1.0.0` |
| `schema_id` / `schema_version` | `handbook.project-condition-evidence-challenge` / `1.0` |
| `operation_id` | `Identifier` |
| `operation` | `publish_managed_operational_surface_evidence` |
| `repository_identity_fingerprint` | `Fingerprint` |
| `condition_ref` / `condition_definition_fingerprint` | exact condition / definition pair |
| `approval_class` / `authority_ref` | `project_condition_evidence` / exact condition ref |
| `evidence_sequence` | sequence |
| `previous_record_fingerprint` | null at sequence 1; otherwise `Fingerprint` |
| `previous_evidence_head_fingerprint` | null at sequence 1; otherwise `Fingerprint` |
| `previous_transaction_fingerprint` | null at sequence 1; otherwise `Fingerprint` |
| `record_subject_fingerprint` | `Fingerprint` |
| `approver_registry_state_fingerprint` | `Fingerprint` |
| `registry_head_transition_fingerprint` | `Fingerprint` |
| `nonce_base64` | exactly 32 bytes encoded as `^[A-Za-z0-9+/]{43}=$` |
| `challenge_fingerprint` | computed `Fingerprint` |

`challenge_fingerprint` covers every challenge key and value except `$schema`
and itself. The immutable challenge ref is
`project-condition-evidence/challenges/challenge_<64 lowercase challenge hex>.json`.

The exact assertion record fields are `schema_id`, `schema_version`,
`assertion_id`, `protocol_id`, `challenge_schema_ref`,
`challenge_jcs_base64`, `challenge_fingerprint`,
`decoded_response_schema_ref`, `decoded_response_ref`,
`decoded_response_fingerprint`, `rp_id`, `client_data_hash`,
`signed_preimage_sha256`, `credential_id_hash`, `algorithm`,
`signature_base64`, `authenticator_data_base64`, `flags_byte`,
`attested_credential_data_included`, `extensions_included`, `user_present`,
`user_verified`, `sign_count`, `verified_at_utc`, and
`assertion_fingerprint`.

Their constants are respectively
`handbook.project-condition-evidence-assertion`, `1.0`,
`fido2-ctap2.1-native`,
`handbook.schemas.project-condition-evidence-challenge@1.0.0`,
`handbook.schemas.security.authenticator-get-assertion-response@1.0.0`,
`handbook.local`, `ES256`, flags byte 5, false, false, true, and true.
`assertion_id` is `Identifier`; every fingerprint field is `Fingerprint`;
the decoded response ref is
`^authenticator-get-assertion-responses/authenticator-get-assertion-response_[0-9a-f]{64}[.]json$`;
base64 fields use the released authenticator bounds; `sign_count` is an
integer from 1 through 4294967295; and `verified_at_utc` is `UtcSecond`.
`assertion_fingerprint` covers every assertion key except itself. Its immutable
ref is
`project-condition-evidence/assertions/assertion_<64 lowercase assertion hex>.json`.

The exact evidence-use transition fields are `$schema`, `schema_version`,
`consumer_kind`, `consumer_id`, `credential_id_hash`, `sequence`,
`prior_head_fingerprint`, `challenge_fingerprint`, `assertion_ref`,
`assertion_fingerprint`, `nonce_sha256`, `prior_sign_count`, `sign_count`,
`result_head_fingerprint`, and `transition_fingerprint`.
Their constants are the use-transition schema ref, `1.0`, and
`project_condition_evidence`; `consumer_id` equals the challenge
`operation_id`; integer sequences and sign counts advance exactly once; and
all fingerprint/ref values replay against the challenge, assertion, and
released authenticator-use head. `transition_fingerprint` covers every key
except `$schema` and itself. Its immutable ref is
`authenticator-use-transitions/authenticator-use-transition_<64 lowercase
transition hex>.json`.

The evidence record's `producer_verification` has exactly:

| Field | Exact V1 contract |
|---|---|
| `verification_kind` | `native_approver_registry_assertion_v1` |
| `approval_class` | `project_condition_evidence` |
| `authority_ref` | exact condition ref |
| `approver_registry_state_ref` | `^registry-states/registry-state_[0-9a-f]{64}[.]json$` |
| `approver_registry_state_fingerprint` | `Fingerprint`, matching ref suffix |
| `registry_head_transition_ref` | `^registry-transitions/registry-transition_[0-9a-f]{64}[.]json$` |
| `registry_head_transition_fingerprint` | `Fingerprint`, matching ref suffix |
| `credential_id_hash` | `Fingerprint` |
| `challenge_ref` | exact challenge-ref grammar |
| `challenge_fingerprint` | `Fingerprint`, matching ref suffix |
| `assertion_ref` | exact evidence-assertion-ref grammar |
| `assertion_fingerprint` | `Fingerprint`, matching ref suffix |
| `authenticator_use_transition_ref` | exact evidence-use-transition-ref grammar |
| `authenticator_use_transition_fingerprint` | `Fingerprint`, matching ref suffix |
| `result_authenticator_use_head_ref` | `^authenticator-use-heads/[0-9a-f]{64}[.]json$` |
| `result_authenticator_use_head_fingerprint` | `Fingerprint` |
| `verified_at_utc` | `UtcSecond`, equal to the assertion value |
| `producer_verification_fingerprint` | computed `Fingerprint` |

`producer_verification_fingerprint` covers every producer-verification field
except itself. The credential must be active, unexhausted, and mapped to the
exact approval-class/authority pair in the current committed registry. Registry
state and transition must equal the current committed registry head. The native
CTAP2.1 ES256 assertion must verify the exact challenge with user presence and
verification, no extensions or attested credential data, the retained public
key, and an advancing sign count.

## Producer API and transaction boundary

The selected producer owner is a future new engine module
`crates/engine/src/project_condition_evidence.rs`. Its typed API is:

```text
ProjectConditionEvidenceServiceV1<P: NativeAuthenticatorPortV1>::publish(
  repo_root: &Path,
  operation_id: EvidenceOperationId,
  candidate_sources: Vec<AdmittedEvidenceSourceCandidateV1>
) -> Result<PublishedProjectConditionEvidenceV1, EvidencePublishRefusalV1>
```

It accepts no bytes, refs, fingerprints, verification closure, sequence,
previous-record value, profile value, Environment Context value, or
applicability flag. It resolves and retains the current evidence head,
repository identity, condition definition, registry, credential/use head, and
all derived refs itself. Supplying a fingerprint is impossible at the typed
boundary; an attempted JSON/file bypass is not an admitted API.

One dedicated bounded journal transaction durably converges, or recovers, to
exactly one state:

1. zero new durable files and the unchanged prior heads; or
2. canonical source records, challenge, decoded response, assertion,
   evidence-use transition, result authenticator-use head, immutable evidence
   record, current evidence head, and immutable transaction record, all
   mutually replayable.

The journal stages prior and result bytes for both mutable heads, fsyncs every
new immutable record and parent, compare-and-replaces both retained heads, then
writes and fsyncs the immutable transaction record as the final commit receipt.
A reader that observes new heads without that receipt refuses until recovery.
Recovery replays the intent fingerprint; it completes a fully staged
transaction or restores both retained prior heads. It refuses a missing stage,
mismatched fingerprint, already-diverged head, fork, sequence reuse, partial
immutable set, or cross-family use transition. No source, assertion, record,
head, or transaction receipt is published outside that journal contract.

This contract deliberately does not select a CLI/compiler caller. A future
production adapter may call `publish` only after a separate non-production
platform probe and human-approved P2A selector prove the exact native primitive
and ownership boundary.

## Freshness basis

Every source independently satisfies:

```text
observed_at_utc < valid_until_utc
valid_until_utc - observed_at_utc <= 2592000 seconds
observed_at_utc <= producer_verification.verified_at_utc
```

`resolve_shipped_profile_decisions` samples one `evaluated_at_utc` before any
condition evaluation and passes it unchanged. Each ordered input contributes
this exact closure row:

```text
{
  input_id,
  source_fingerprint,
  evaluated_at_utc,
  observed_at_utc,
  verified_at_utc,
  valid_until_utc,
  maximum_validity_seconds: 2592000,
  classification
}
```

Classification is `verification_not_yet_valid` when evaluated before
`verified_at_utc`, `observation_not_yet_valid` when evaluated before
`observed_at_utc`, `current` only when both lower bounds are satisfied and
`evaluated_at_utc <= valid_until_utc`, otherwise `expired`. Boundaries are
inclusive at `verified_at_utc`, `observed_at_utc`, and `valid_until_utc`.
Mixed current and non-current inputs are representable and resolve `stale`.
File mtimes, presentation timestamps, and local clock repair do not
participate.

## Total outcome and reason precedence

The evaluator applies this first-match order without dropping an admitted row:

1. `refused` / `evidence_refused` for unsafe identity, non-canonical or
   oversized bytes, schema/field/order/duplicate error, any fingerprint,
   condition, repository, chain, mapping, signature, counter, or current-head
   mismatch, a disallowed/circular input, rollback/fork/replay, or a current
   qualifying-positive combined with a current global negative;
2. `unresolved` / `evidence_record_missing` when the fixed evidence head is
   absent;
3. `unresolved` / `evidence_dependency_unresolved` when an otherwise
   well-formed exact definition, current head, registry record, credential,
   source, challenge, response, assertion, or transition cannot be retained;
4. `stale` / `evidence_not_current` when at least one otherwise trusted input
   is not current;
5. `unknown` / `insufficient_current_proof` when all current inputs are
   `unknown`;
6. `false` / `affirmative_no_qualifying_responsibility` when at least one
   current global negative exists and no current qualifying-positive exists;
7. `true` / `qualifying_continuing_responsibility` when at least one current
   qualifying-positive exists and no current global negative exists.

All seven branches are reachable in V1; the six released outcome values remain
unchanged. Bare silence never produces false. Invalid or contradictory
evidence wins over missing dependencies, missing dependencies win over stale,
and stale wins over current claims. True maps only to `Required`, false only to
`Optional`, and the other outcomes only to `Indeterminate`.

## Fingerprinted evaluation closure

The evaluator emits the closed
`handbook.schemas.project-condition-evidence-evaluation-closure@1.0.0` shape:

```text
{
  schema_id,
  schema_version,
  evaluator_ref,
  evaluator_definition_fingerprint,
  condition_ref,
  condition_definition_fingerprint,
  repository_identity_fingerprint,
  evaluated_at_utc,
  dependency_observations,
  evidence_head_fingerprint,
  evidence_record_fingerprint,
  producer_verification_fingerprint,
  freshness_basis,
  ordered_inputs,
  outcome,
  reason,
  closure_fingerprint
}
```

The first seven identities are exact constants or retained fingerprints;
`evaluated_at_utc` is `UtcSecond`. `freshness_basis` is the ordered per-input
array above. `ordered_inputs` repeats each resolved input row plus the complete
resolved source record in `input_id` order.

`dependency_observations` is an ordered array of closed rows with exactly
`dependency_kind`, `logical_id`, `relative_ref`, `observation_kind`,
`byte_count`, `observed_bytes_fingerprint`, `required_fingerprint`,
`claimed_fingerprint`, `computed_fingerprint`, and `error_code`.

`dependency_kind` is one of `evidence_transaction_store`, `evidence_head`,
`evidence_transaction`, `evidence_record`, `admitted_source`,
`condition_definition`, `evaluator_definition`, `repository_identity`,
`registry_head_transition`, `registry_state`, `credential_registration`,
`authenticator_use_head`, `challenge`, `decoded_assertion_response`,
`assertion`, or `evidence_use_transition`.

The singleton `evidence_transaction_store` row is always first. Its
`logical_id` is exactly `managed-operational-surface-transactions` and its
`relative_ref` is exactly
`.handbook/state/project-condition-evidence/transactions`. Other
`logical_id` values are the condition ref for singleton dependencies, the
zero-padded sequence for chain rows, `input_id` for sources, and the retained
credential hash for credential/use-head rows.

Rows are ordered by the enum order above, then ascending sequence, `input_id`,
or logical ID. Every attempted dependency produces exactly one row, including
the transaction store and each transaction/record chain member. `relative_ref`
is the exact attempted ref or fixed selector. `observation_kind` is `absent`,
`unsafe`, `read_error`, `directory_limit_exceeded`, `retained_directory`, or
`retained_bytes`.

For `retained_directory`, `byte_count` is the byte length of an RFC 8785
canonical directory-observation envelope and `observed_bytes_fingerprint` is
SHA-256 of that envelope. The envelope is exactly
`{filename_encoding, entries}`: `filename_encoding` is `utf8` on Unix and
`windows_utf16le` on Windows; `entries` is the raw filename byte sequence of
every entry encoded as base64 and sorted lexicographically by raw bytes.
Required, claimed, and computed fingerprints are null for the store row. This
binds invalid encoding, unknown names, case aliases, and duplicate-normalized
names before any sequence-bearing transaction row exists. An over-limit store
never enters the retained-directory envelope path.

For absent/unsafe/read-error/directory-limit-exceeded rows, byte count,
observed, claimed, and computed fingerprints are null; `required_fingerprint`
is populated only when a retained parent supplied one. For the singleton store
row it is therefore null in all four cases. For retained bytes, byte count and
SHA-256 of the exact raw bytes are always non-null; claimed/computed
fingerprints are nullable only until parsing reaches the corresponding
field/preimage. An accepted row has a null error; every rejected row has one
exact error.

The closed error-code enum is `evidence_record_missing`, `dependency_missing`,
`transaction_store_missing`, `transaction_store_unsafe`,
`transaction_store_read_failed`, `transaction_entry_invalid`,
`transaction_entry_alias`, `transaction_store_over_limit`, `unsafe_path`,
`identity_changed`, `read_failed`, `oversized`, `non_canonical`,
`schema_invalid`, `fingerprint_mismatch`, `wrong_authority`, `chain_invalid`,
`transaction_tip_mismatch`, `producer_invalid`, `dependency_unresolved`,
`not_current`, `contradiction`, or null. Distinct retained transaction-store
listings and retained corrupt bytes for the head, any
transaction/record/source, challenge, response, assertion, use transition,
registry object, credential, definition, or identity therefore bind distinct
closure fingerprints. Over-limit stores are the deliberate exception: their
fixed failure observation does not bind names, encodings, which ceiling fired,
entry counts, projected sizes, or directory contents.

### Normative transaction-store failure matrix

Transaction-store inspection uses this exact first-match order before any
sequence-bearing transaction row is emitted:

1. a missing selector is `absent`;
2. an extant selector that cannot be retained as a no-follow directory is
   `unsafe`;
3. a retained directory whose complete entry set cannot be read is
   `read_error`;
4. enumeration initializes `entry_count` to zero and
   `projected_envelope_bytes` to the exact RFC 8785 empty-envelope length: 41
   for `{"entries":[],"filename_encoding":"utf8"}` or 52 for
   `{"entries":[],"filename_encoding":"windows_utf16le"}`;
5. for each yielded entry, `candidate_count` is
   `entry_count.checked_add(1)`. Arithmetic failure or a value greater than
   4096 immediately emits `directory_limit_exceeded` /
   `transaction_store_over_limit` before the yielded name is copied, retained,
   measured, validated, sorted, encoded, serialized, or fingerprinted;
6. while `candidate_count` is admissible, the implementation borrows only the
   yielded raw name long enough to obtain its exact encoded byte length `n`:
   the Unix filename byte sequence for `utf8`, or its Windows UTF-16LE byte
   sequence for `windows_utf16le`. It computes the base64 text length exactly as
   `4 * floor((n + 2) / 3)` with checked add and multiply, then computes
   `candidate_envelope_bytes` by checked addition of the current projection,
   one comma when `entry_count` is nonzero, two JSON quote bytes, and that
   base64 length. Any arithmetic failure or value greater than 131072 emits the
   same `directory_limit_exceeded` / `transaction_store_over_limit` row before
   that offending name is copied, retained, encoded, or serialized;
7. only an entry that passes both ceilings is copied into retained raw-name
   storage and advances both counters. No envelope byte and no complete or
   partial envelope fingerprint is produced during enumeration;
8. only after end-of-directory may the complete retained set be sorted by raw
   bytes, base64-encoded, and serialized once. Its exact serialized length must
   equal `projected_envelope_bytes`, and its complete bytes produce the store
   fingerprint; and
9. only after that complete bounded envelope exists, a raw name that is not
   byte-equal to its canonical lowercase transaction filename but
   ASCII-case-folds to that valid filename, or two raw names that
   ASCII-case-fold to the same valid filename, is
   `transaction_entry_alias` before the general
   `transaction_entry_invalid` rule.

No later store rule replaces an earlier one. The envelope path is reachable
only after the enumerator observes end-of-directory with at most 4096 retained
entries and at most 131072 projected bytes. Count and byte-limit failures,
including checked-arithmetic failure, intentionally collapse to the same fixed
row and closure. In particular, over-limit fixtures deliberately contain
otherwise invalid or aliasing names: either ceiling selects
`transaction_store_over_limit` before all per-name validation. Previously
retained names are discarded without serialization or fingerprinting, the
offending entry is not retained, and no entry after the offender is requested.

The fingerprint/nullability tuple below is ordered as
`(byte_count, observed_bytes_fingerprint, required_fingerprint,
claimed_fingerprint, computed_fingerprint)`. `unix` means
`filename_encoding:"utf8"`; `windows` means
`filename_encoding:"windows_utf16le"`. A directory envelope is RFC 8785 JSON
without a trailing LF and therefore remains distinct from the persisted
record encodings above.

| Store case | Canonical first-row fixture | `observation_kind` | Exact fingerprint/nullability tuple | Exact `error_code` | Closure-fingerprint rule |
|---|---|---|---|---|---|
| absent | `TS-ABSENT-V1` | `absent` | `(null, null, null, null, null)` | `transaction_store_missing` | With all other closure fields equal, the observation kind and error code alone distinguish this closure from every other store failure. |
| unsafe | `TS-UNSAFE-V1` | `unsafe` | `(null, null, null, null, null)` | `transaction_store_unsafe` | With all other closure fields equal, the observation kind and error code alone distinguish this closure from absent and unreadable selectors. |
| unreadable | `TS-UNREADABLE-V1` | `read_error` | `(null, null, null, null, null)` | `transaction_store_read_failed` | No partial entry set is fingerprinted; the exact observation kind and error code bind the failure into the closure. |
| invalid-entry | `TS-INVALID-ENTRY-V1` | `retained_directory` | `unix=(63, sha256:afe7684b58ea472b8cee85911c6f77053de0b628a1339fc0714bae6b0093ae29, null, null, null)`; `windows=(94, sha256:73e13e5aee402f17754e78a71ba17c751501ef721cc7fcf32f3ab2ed35282f50, null, null, null)` | `transaction_entry_invalid` | The complete platform-tagged directory envelope and error code are inside the first row; changing any raw name, encoding tag, or platform changes the closure fingerprint. |
| alias | `TS-ALIAS-V1` | `retained_directory` | `unix=(159, sha256:deb72da42ca648f3fe5abf6cbdb375a4a82eb7b448f795735a2435ad37194c53, null, null, null)`; `windows=(286, sha256:08a402f611c7a7c56c30ee425689ccd5e07b45d57f9a04aa7d7a520d20863141, null, null, null)` | `transaction_entry_alias` | The complete platform-tagged alias spelling and error code are inside the first row; a case-spelling change cannot reuse another alias closure fingerprint. |
| over-limit | `TS-OVER-LIMIT-V1` | `directory_limit_exceeded` | `(null, null, null, null, null)` | `transaction_store_over_limit` | With all other closure fields equal, the observation kind and error code bind one fixed failure closure. No name, platform tag, count, projected size, selected ceiling, complete/partial envelope, sentinel hash, or listing fingerprint enters the row or closure. Count, byte, and checked-arithmetic failures intentionally share this row and closure. |

All six rows have the common exact identities
`dependency_kind:"evidence_transaction_store"`,
`logical_id:"managed-operational-surface-transactions"`,
`relative_ref:".handbook/state/project-condition-evidence/transactions"`, and
no additional keys. Their canonical RFC 8785 first-row fixtures are:

```text
TS-ABSENT-V1={"byte_count":null,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_store_missing","logical_id":"managed-operational-surface-transactions","observation_kind":"absent","observed_bytes_fingerprint":null,"relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
TS-UNSAFE-V1={"byte_count":null,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_store_unsafe","logical_id":"managed-operational-surface-transactions","observation_kind":"unsafe","observed_bytes_fingerprint":null,"relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
TS-UNREADABLE-V1={"byte_count":null,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_store_read_failed","logical_id":"managed-operational-surface-transactions","observation_kind":"read_error","observed_bytes_fingerprint":null,"relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
TS-INVALID-ENTRY-V1[unix]={"byte_count":63,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_entry_invalid","logical_id":"managed-operational-surface-transactions","observation_kind":"retained_directory","observed_bytes_fingerprint":"sha256:afe7684b58ea472b8cee85911c6f77053de0b628a1339fc0714bae6b0093ae29","relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
TS-INVALID-ENTRY-V1[windows]={"byte_count":94,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_entry_invalid","logical_id":"managed-operational-surface-transactions","observation_kind":"retained_directory","observed_bytes_fingerprint":"sha256:73e13e5aee402f17754e78a71ba17c751501ef721cc7fcf32f3ab2ed35282f50","relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
TS-ALIAS-V1[unix]={"byte_count":159,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_entry_alias","logical_id":"managed-operational-surface-transactions","observation_kind":"retained_directory","observed_bytes_fingerprint":"sha256:deb72da42ca648f3fe5abf6cbdb375a4a82eb7b448f795735a2435ad37194c53","relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
TS-ALIAS-V1[windows]={"byte_count":286,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_entry_alias","logical_id":"managed-operational-surface-transactions","observation_kind":"retained_directory","observed_bytes_fingerprint":"sha256:08a402f611c7a7c56c30ee425689ccd5e07b45d57f9a04aa7d7a520d20863141","relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
TS-OVER-LIMIT-V1={"byte_count":null,"claimed_fingerprint":null,"computed_fingerprint":null,"dependency_kind":"evidence_transaction_store","error_code":"transaction_store_over_limit","logical_id":"managed-operational-surface-transactions","observation_kind":"directory_limit_exceeded","observed_bytes_fingerprint":null,"relative_ref":".handbook/state/project-condition-evidence/transactions","required_fingerprint":null}
```

The retained-directory fixture envelopes are exact:

```text
TS-INVALID-ENTRY-V1[unix]={"entries":["dW5leHBlY3RlZC50eHQ="],"filename_encoding":"utf8"}
TS-INVALID-ENTRY-V1[windows]={"entries":["dQBuAGUAeABwAGUAYwB0AGUAZAAuAHQAeAB0AA=="],"filename_encoding":"windows_utf16le"}
TS-ALIAS-V1[unix]={"entries":["VFJBTlNBQ1RJT05fMDAwMV8wMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwLkpTT04="],"filename_encoding":"utf8"}
TS-ALIAS-V1[windows]={"entries":["VABSAEEATgBTAEEAQwBUAEkATwBOAF8AMAAwADAAMQBfADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAAMAAwADAALgBKAFMATwBOAA=="],"filename_encoding":"windows_utf16le"}
```

`TS-OVER-LIMIT-V1` is the exact platform-independent first row above, not a
directory envelope. Once either candidate ceiling or any checked operation
fails, the enumerator returns that row without retaining the offending entry or
requesting another one. Previously retained names are dropped. It performs no
sort, base64 encoding, envelope serialization, or complete or partial envelope
fingerprint. Count, projection, and platform state select the fixed row but are
not serialized into it.

The proof-name function
`transaction(i) = "transaction_" || decimal4(i) || "_" || lowercase_hex64(i) ||
".json"` produces one exact 86-ASCII-byte valid transaction filename. The
boundary vectors are:

- `TS-COUNT-4096-V1` enumerates ASCII `c0000` through `c4095`. End-of-directory
  at 4096 retains the complete envelope before invalid-name classification:
  Unix is `(45096,
  sha256:fe286f6ba67aa4f2af1036eb99e75df54fc72957c9c19a2e494c46fab77c8dfc)`
  and Windows is `(77875,
  sha256:8e1215a1d217e2e8654da5d057e69dca9a481cf9b11270230a2b4e9dae8f697d)`.
  `TS-COUNT-4097-V1` appends `c4096`; candidate count 4097 emits the fixed
  over-limit row before that name is measured or retained, despite the earlier
  invalid names.
- `TS-UNIX-ENVELOPE-1101-V1` enumerates `transaction(1)` through
  `transaction(1101)`. Its complete envelope is `(131059,
  sha256:3c37bbe60138407f496a24f7a61f58f44175c81e4038e7d5de2549c2c1327a0e)`.
  Adding `transaction(1102)` projects 131178 bytes, so
  `TS-UNIX-ENVELOPE-1102-V1` emits the fixed row before retaining entry 1102.
- `TS-WINDOWS-ENVELOPE-557-V1` enumerates `transaction(1)` through
  `transaction(557)` as UTF-16LE. Its complete envelope is `(130946,
  sha256:87881b12bfa5937094c20f030f6a818d5e826c13644f0aeb69b28d98c8e46dea)`.
  Adding `transaction(558)` projects 131181 bytes, so
  `TS-WINDOWS-ENVELOPE-558-V1` emits the fixed row before retaining entry 558.

For the exact-byte vectors, `p(P,i,L)` is ASCII prefix `P`, four-digit decimal
`i`, one `-`, and enough ASCII `a` bytes to make total length `L`; enumeration
uses the group and index order written below, which is also raw-byte sort order.

- `TS-EXACT-131072-V1[unix]` contains 86 `p("u84-",0..85,84)` names and 1018
  `p("u86-",0..1017,86)` names. Its complete envelope is exactly `(131072,
  sha256:8e7ecc78200f2835ed60bb652de75728e3d59e12754c09ff630bad28094c5ea6)`.
  `TS-EXACT-131073-V1[unix]` instead contains 56
  `p("u84-",0..55,84)` names followed by 1047
  `p("u86-",0..1046,86)` names. Before the last entry the projection is 130954;
  the ordinal-1103 candidate is exactly 131073 and is not retained.
- `TS-EXACT-131072-V1[windows]` contains 86
  `p("w85-",0..85,85)` names and 473 `p("w86-",0..472,86)` names encoded as
  UTF-16LE. Its complete envelope is exactly `(131072,
  sha256:d0677bb6103e70003e5c7ae1eaa15be84a06034cd6401403bb772ec3cd2dba30)`.
  `TS-EXACT-131073-V1[windows]` instead contains 27
  `p("w85-",0..26,85)` names followed by 531
  `p("w86-",0..530,86)` names. Before the last entry the projection is 130838;
  the ordinal-558 candidate is exactly 131073 and is not retained.

Two adversarial families freeze early termination:

- `TS-OVER-LIMIT-LONG-NAME-V1[unix]` starts with 1099 transaction names
  projecting 130821 bytes; a 255-ASCII-`z` name at ordinal 1100 projects 131164
  and is not retained. The Windows case starts with 555 transaction names
  projecting 130476 bytes; the same 255-character name is 510 UTF-16LE bytes
  and projects 131159 at ordinal 556. Both emit the fixed row before
  invalid-name validation.
- `TS-OVER-LIMIT-ARBITRARY-SUFFIX-V1[unix,N]` is defined for every
  `N >= 1102`: it presents 1101 transaction names, an ordinal-1102 transaction
  name that projects 131178, and traps if ordinal 1103 is requested. The Windows
  form is defined for every `N >= 558`, presents 557 transaction names, an
  ordinal-558 transaction name that projects 131181, and traps if ordinal 559
  is requested. Each form emits byte-identical `TS-OVER-LIMIT-V1` and never
  touches its arbitrary suffix.

These vectors prove both ceilings and checked projection without entering an
offending name, partial envelope, or partial fingerprint into the closure.
Every bounded directory retains and fingerprints its complete envelope. Every
count, byte, or arithmetic over-limit store intentionally shares the same fixed
failure closure when every non-store closure field is equal. This loss of
per-listing distinction is intentional fail-closed behavior, not an incomplete
observation.

For every row, `closure_fingerprint` remains SHA-256 over the complete
evaluation closure's RFC 8785 JSON except `closure_fingerprint` itself. The
canonical first row above is the first element of `dependency_observations`.
Thus null byte fingerprints do not collapse absent, unsafe, unreadable, and
over-limit stores because their observation/error pairs differ;
retained-directory cases bind the exact platform envelope; every over-limit
store deliberately maps to the same fixed store row; and byte-identical
complete closures reproduce the same fingerprint.

`evidence_head_fingerprint`, `evidence_record_fingerprint`, and
`producer_verification_fingerprint` are non-null only after the corresponding
record has parsed and replayed far enough to establish that value; otherwise
they are null. `freshness_basis` and `ordered_inputs` contain only rows resolved
before the winning branch and are empty before any input resolves.
`dependency_observations` contains every dependency attempted through the
winning branch and never drops a failed observation.

`closure_fingerprint` is SHA-256 over RFC 8785 canonical JSON containing every
closure key and value except itself. The different evaluation instant,
malformed bytes, unsafe observation, missing dependency, and every successful
outcome therefore produce replayably distinct closure fingerprints.

## Evaluator owner, compatibility API, and transport

The semantic owner is the same future new module,
`crates/engine/src/project_condition_evidence.rs`. Its selected evaluator API
returns an evaluator-owned type rather than constructing a sibling's private
fields:

```text
pub(crate) fn evaluate_managed_operational_surface(
  repo_root: &Path,
  condition_ref: &ExactDefinitionRef,
  condition_definition_fingerprint: &DefinitionFingerprint,
  evaluated_at_utc: &str
) -> ManagedOperationalSurfaceEvaluation
```

It performs no network, subprocess, environment-variable, CLI-argument, or
Markdown read. Its transport is only the bounded exact transaction directory,
the fixed evidence head, and exact retained relative refs reachable from the
unique maximal transaction/head.

`profile_decision.rs` adds exactly this crate-private constructor:

```text
pub(crate) fn ProjectConditionEvaluation::evaluated(
  condition_ref: ExactDefinitionRef,
  condition_definition_fingerprint: DefinitionFingerprint,
  outcome: ProjectConditionOutcome,
  reason: ProjectConditionDecisionReason,
  evidence_closure_fingerprint: DefinitionFingerprint
) -> Self
```

`ProjectConditionDecisionReason` retains
`EvidenceContractUnavailable` and adds exactly
`EvidenceRefused`, `EvidenceRecordMissing`, `EvidenceDependencyUnresolved`,
`EvidenceNotCurrent`, `InsufficientCurrentProof`,
`AffirmativeNoQualifyingResponsibility`, and
`QualifyingContinuingResponsibility`. These public enum additions are
source-breaking for external exhaustive matches; human runtime approval must
explicitly accept that compatibility cost. The serialized existing variant and
the public `ResolvedProfileDecisions::from_profile` signature, return type, and
fallback behavior remain byte-for-byte/behaviorally unchanged.

`resolve_shipped_profile_decisions` remains the only shipped integration entry.
It samples the instant, evaluates only the exact selected condition, and passes
the result to:

```text
fn ResolvedProfileDecisions::from_profile_with_condition_evaluations(
  profile: &ResolvedInstanceProfile,
  condition_evaluations: BTreeMap<
    ExactDefinitionRef,
    ProjectConditionEvaluation
  >
) -> Result<Self, ProfileDecisionError>
```

This new helper is private to `profile_decision.rs`. The direct public
`from_profile` calls it with an empty map and continues to produce
`unresolved` / `EvidenceContractUnavailable`, no closure fingerprint, and
`Indeterminate`. `artifact_decision` consumes only an exact matching supplied
evaluation.

## Evaluator definition

The eight schema paths are exactly
`crates/engine/definitions/schemas/<schema-ref-without-version>/1.0.0.schema.json`,
where each directory name is the complete schema ID listed under the closed
schema set. The new definition path is
`crates/engine/definitions/project-condition-evaluators/handbook.condition-evaluator.managed-operational-surface/1.0.0.yaml`
under ref
`handbook.condition-evaluator.managed-operational-surface@1.0.0`. Its exact
closed keys are `schema_id`, `schema_version`, `evaluator_id`,
`evaluator_version`, `condition_ref`, `condition_definition_fingerprint`,
`source_schema_ref`, `evidence_schema_ref`, `head_schema_ref`,
`transaction_schema_ref`, `challenge_schema_ref`, `assertion_schema_ref`,
`use_transition_schema_ref`, `closure_schema_ref`, `admitted_input_classes`,
`maximum_validity_seconds`, `outcome_precedence`, `owner_module`, `transport`,
`extensions`, and `definition_fingerprint`.

V1 admits exactly `[admitted_evidence_ref]`, maximum validity is `2592000`,
precedence is exactly `[refused, unresolved, stale, unknown, false, true]`,
owner is `handbook_engine::project_condition_evidence`, transport is
`bounded_transaction_tip_fixed_head_and_retained_refs`, extensions is `{}`,
and the ordinary uniform definition fingerprint covers every key except itself.

No released authenticator, condition, profile, artifact, intake, renderer, or
schema record changes behind an existing ref.

## Reviewable runtime packets after human approval

No packet below is authorized by this amendment.

| Packet | Exact future files/symbols | Acceptance and stop boundary |
|---|---|---|
| P2S schema/semantic freeze | eight new schema files, one new evaluator definition, schema/definition vector tests only | Closed-schema vectors and all preimage/nullability checks pass; no Rust production edit |
| P2A native feasibility | non-production probe first; proposed new `PlatformNativeAuthenticatorPortV1` only after a separate exact compiler/platform selector | Prove the local platform primitive and cancellation/error mapping; stop and return for authority on unavailable primitive, dependency/Cargo change, unsafe code, or another process/module |
| P2P producer transaction | new `project_condition_evidence.rs`, `lib.rs` module exposure, producer-only tests | Sources/challenge/assertion/use-chain/record/two-head/immutable-transaction transaction, recovery, rollback, fork, replay, cross-family refusal, and unavailable-port zero-write pass; no profile or Environment Context edit |
| P2R evaluator | evaluator functions in the same new module and evaluator-only tests | All seven precedence branches, raw-observation closure replay, mixed freshness, and fixed-path safety pass; no shipped resolver edit |
| P2I resolver integration | exact reason enum variants, `ProjectConditionEvaluation::evaluated`, `resolve_shipped_profile_decisions`, new private `from_profile_with_condition_evaluations`, and `artifact_decision` only | Direct public fallback unchanged; shipped exact-condition integration and downstream compile wall pass; no Environment Context writer/consumer edit |
| P2V Environment Context | only the separately frozen existing P2 author/consumer selector | True writes canonical YAML; false writes nothing/complete; other outcomes write nothing/fail closed; no P6 deletion |

Each packet has its own RED/GREEN wall, immutable dispatch, discovery review,
remediation if required, different-fresh closure, and reviewed commit. P2A must
complete before P2P; P2S precedes P2P/P2R; P2P and P2R precede P2I; all precede
P2V. Creating a packet does not complete P2.

## Future selectors and impact ceilings

The GitNexus index was fully rebuilt at
`00dde0162fcb15576c83b6ed40ab7286488d0890`. Fresh depth-3 upstream impact is:

| Existing symbol | Risk | Hard ceiling |
|---|---|---|
| `resolve_shipped_profile_decisions` | CRITICAL | 220 impacted, 75 direct, 8 processes, 13 modules |
| `ResolvedProfileDecisions::from_profile` | CRITICAL | 156 impacted, 1 direct, 7 processes, 10 modules |
| `artifact_decision` | CRITICAL | 77 impacted, 1 direct, 6 processes, 10 modules |
| `ProjectConditionDecisionReason` | LOW graph result with incomplete type-edge coverage | zero indexed; compiler, downstream, serialization, and workspace walls are authoritative |

P2I may edit only the four existing selectors above in
`profile_decision.rs`; the public `from_profile` body may be mechanically
refactored only by extracting its unchanged behavior into the selected private
helper. It may add only the two named new symbols in that file. P2P/P2R may add
the named new module and its private/crate-private symbols; after their first
minimal introduction, fresh impact must show no caller outside the selected
packet until P2I and no process outside the table's existing integrated ceiling.

Stop before implementation if refreshed impact is wider, another existing
production symbol is needed, the direct fallback changes, a public signature
other than the explicitly approved reason-enum additions changes, or P2A
requires a dependency, Cargo/version change, unsafe policy change, new process,
or unselected platform module. The released condition definition, shipped
profile, Environment Context definition/schema/kind/intake/renderer,
authenticator/registry schemas, and every existing fingerprint remain
byte-identical.

## Exact proof selectors

P2S adds schema-vector tests proving every constant, bound, enum, regex,
cross-field rule, preimage, closed-key refusal, and closure nullability branch.

P2P adds `crates/engine/tests/hcm_2_4_project_condition_evidence_producer.rs`
and must prove:

- producer-computed source fingerprints/refs and rejection of any byte/file
  bypass or supplied-fingerprint surface;
- source tamper, input/source mismatch, wrong authority/repository, and circular
  evidence refusal;
- exact challenge/assertion/use-transition/verification replay;
- unavailable native port makes zero writes;
- atomic success, every injected interruption, two-head/commit-receipt recovery,
  stale-head conflict, old-record restoration, simultaneous two-head rollback
  with a newer immutable transaction, transaction/record-chain fork/gap/cycle,
  assertion replay, sequence reuse, and cross-family use-chain refusal; and
- no CLI/compiler/Environment Context effect.

P2R adds `crates/engine/tests/hcm_2_4_project_condition_evidence_evaluator.rs`
and must prove:

- fixed safe head/read boundaries, byte ceilings, no-follow, retained identity,
  substitution and inode ABA refusal;
- absent, unsafe, malformed, partially parsed, semantically refused, stale,
  unknown, false, and true closure fingerprints replay from exact ordered
  dependency observations;
- one-byte mutation, missing, unsafe, and read-failure observations for every
  dependency class produce replayably distinct closure fingerprints;
- absent, unsafe, unreadable, invalid-entry, alias, and over-limit transaction
  stores produce distinct first-row observations before transaction replay;
- count 4096/4097, Unix 1101/1102, Windows 557/558, exact
  131072/131073-byte, long-name, and arbitrary-suffix vectors prove checked
  dual-ceiling projection, preserve complete-envelope fingerprints for bounded
  directories, retain no offending entry, create no complete or partial
  over-limit envelope or fingerprint, and reproduce the common fixed failure
  closure;
- each lower/upper freshness boundary, evaluator time before verification,
  evaluator clock rollback, mixed current/stale sources, and one sampled instant;
- every precedence branch is reachable and ordered; and
- Environment Context bytes, Markdown, `applicability_basis`, artifact
  presence, profile flags, and ambient environment have zero influence.

P2I may change only condition-specific cases in
`crates/engine/tests/hcm_1_4_profile_decisions.rs` and
`crates/engine/tests/hcm_1_4_profile_inspection.rs`. It proves all exact
outcome/reason/applicability mappings, closure propagation, wrong-condition
isolation, direct `from_profile` fallback compatibility, serialized existing
reason stability, exhaustive workspace/downstream compilation, and the
recorded CRITICAL flow wall.

P2V separately proves true writes Environment Context, false writes nothing
and treats absence complete, the other outcomes write nothing and fail closed,
and setup/doctor/compiler/flow share the exact evaluated closure. P6 remains
forbidden.

## Feasibility evidence and human gate

The non-production probe

```text
cargo test -p handbook-engine --test hcm_2_2_authenticator_security
```

passed 9/9. It proves existing CTAP2.1 request encoding, ES256 assertion
verification, deterministic credential selection, malformed/status refusal,
and unavailable-port refusal. It does not prove a production native adapter or
the new two-head plus immutable-transaction transaction. Live production source
still exposes only
`UnavailableNativeAuthenticatorPortV1` in the compiler path.

Review-clean planning therefore does not authorize runtime. Human approval
must first approve P2S and the non-production P2A feasibility selector. After
P2A reports observed local behavior, the human must separately approve its
exact production adapter or name a completed producer dependency before P2P.
P2R/P2I/P2V remain closed until their dependency packets are review-clean.

## Discovery-review remediation

The first independent planning review returned seven Required findings. This
revision addresses them without runtime work:

1. binds the current evidence head to the current authenticator-use head and a
   safely discovered unique maximal immutable transaction chain, adds immutable
   sequence/previous-record replay, and selects rollback/fork/recovery proofs;
2. defines a retained admitted-source record and producer-computed source
   fingerprint/ref while removing every caller-supplied fingerprint surface;
3. closes every challenge, assertion, use-transition, verification, and
   evaluation-closure field/preimage/nullability rule and binds ordered raw
   observations for every dependency class;
4. makes freshness per source and includes `verified_at_utc` in the current
   predicate;
5. removes the unreachable V1 authoritative-fact adapter branch;
6. selects an evaluator-owned return plus exact crate-private constructor,
   reason variants, and explicit public-enum compatibility gate; and
7. decomposes schema, native feasibility, producer, evaluator, integration, and
   Environment Context work into separately reviewable packets and proof walls.

The first closure review closed items 2 and 4 through 7 but found the original
rollback and raw-observation remedies incomplete. The first supplemental
revision added the immutable maximal transaction selector and complete ordered
dependency observations. Supplemental review 1 closed rollback but found that
transaction-directory failures precede sequence-bearing rows. Supplemental
review 2 accepted the singleton store observation direction but returned one
Required finding: its six failure cases lacked a normative one-to-one matrix.

The later matrix-only discovery review accepted five rows but returned one
Required finding: the over-limit row still required a complete directory
envelope that is unbounded against the 131072-byte ceiling. That run stopped
without remediation.

The prior typed and matrix-only lineages are exhausted and remain immutable.
The current explicit human selector authorizes only the dual count/canonical-
envelope ceiling repair above and exactly one fresh complete-subject review. A
valid P1/P2 from that review stops the top-level run without remediation. A
CLEAN result permits only the reviewed planning commit, separate mechanical
closeout, and presentation for distinct P2S/P2A human approval; it grants no
runtime, schema, definition, P4, P6, or sibling authority.
