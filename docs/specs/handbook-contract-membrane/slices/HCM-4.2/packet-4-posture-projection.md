# HCM-4.2 Packet 4 — posture projection and direct-Rust admission

This checkpoint records the decisions selected under the slice's delegated
Packet-4 preparation boundary. It controls only Packet 4. The slice README,
Packet-2 and Packet-3 checkpoints, current program sources, and live source and
tests remain controlling where this checkpoint is silent.

## Current decision

Packet 4 adds one definition-bound Rust SDK operation seam for
`posture.transition.apply@1.0.0`. The SDK admits the generic request envelope,
projects its bounded posture body, passes the raw idempotency key through the
sealed `/body/idempotency_key` bridge, invokes the existing engine facade, and
mechanically projects the closed owner outcome into `OperationResponseV1`.
The existing `HandbookSdkV1::apply_posture_transition` method remains the
HCM-4.1 precursor and compatibility anchor; it is not by itself current
transport evidence.

The engine remains the semantic owner. It continues to resolve repository
identity, current canonical and lifecycle heads, profile and policy, approvals,
source/result kernels, lifecycle validation, persistence, recovery, and replay.
No public request accepts any of those owner-resolved values.

## Exact request and correlation

The operation request schema contains exactly these seven semantic fields:

1. `idempotency_key`: visible ASCII, `1..=256` bytes, present only in the wire
   body at `/body/idempotency_key`;
2. `repository_identity_fingerprint`: an exact SHA-256 fingerprint;
3. `expected_canonical`: reference, fingerprint, document SHA-256, and positive
   byte length;
4. `recommendation`: bounded reference and exact fingerprint;
5. `reassessment_coverage_ids`: `1..=64` unique, strictly byte-sorted visible
   ASCII IDs of `1..=256` bytes;
6. `change`: dimension ID and index, authority path, required-nullable expected
   stored value, expected effective level, and proposed effective level; and
7. `effective_at_utc`: canonical UTC-second text.

The admitted public body is sanitized and exposes only fields 2–7. The raw key
has no public accessor. Its only production borrow uses Packet 2's sealed owner
consumer at the definition-pinned pointer. The engine's raw SHA-256 digest is
checked transiently against the owner success receipt and is discarded. The
only fingerprint projected into the public result and idempotency state is the
operation-scoped fingerprint
`sha256("handbook.operation-idempotency-key-v1\\0" || operation_ref ||
"\\0/body/idempotency_key\\0" || raw_key)`.

`expected_canonical.reference` is exactly
`.handbook/project/charter.yaml`; its fingerprint and document SHA-256 are
equal exact fingerprints, and its byte length is positive.

`request_id` follows the implemented Packet-2 required-nullable contract rather
than the stale archived suggestion: explicit `null` is accepted as no
correlation and a valid ID is accepted and echoed. An absent or invalid member
returns `ContractError` during `OperationRequestV1::parse_and_admit`, before any
owner call; it produces neither `OperationResponseV1` nor a normalized
`OperationAdmissionRefusalV1`. The field is excluded from the request and
semantic-result fingerprints. Changing one valid ID to another changes only
the echo and outer response fingerprint.

## Result, replay, and realized writes

`Applied` and `Replayed` data is the bounded seven-field projection of the live
receipt: scoped idempotency-key fingerprint, repository identity fingerprint,
prior canonical, resulting canonical, posture transition, lifecycle transition,
and resulting kernel. No private record, raw canonical bytes, journal path,
recovery frame, owner text, or unscoped key digest is public.

The exact compare-and-write expected basis is the byte-sorted set
`/body/change`, `/body/effective_at_utc`, `/body/expected_canonical`,
`/body/reassessment_coverage_ids`, `/body/recommendation`, and
`/body/repository_identity_fingerprint`. The atomic group is
`posture_transition_apply`, with exactly two declared and realized writes:

| Record kind | Authority class | Runtime record projection | Condition |
| --- | --- | --- | --- |
| `constitutional_root` | `canonical_truth` | `handbook.posture-record.constitutional-root@1.0.0` plus the resulting-canonical fingerprint | `handbook.posture-write-condition.canonical-truth@1.0.0`, exactly `{prior_canonical,resulting_canonical}` |
| `posture_transition` | `semantic_record` | `handbook.posture-record.transition@1.0.0` plus the posture-transition fingerprint | `handbook.posture-write-condition.semantic-record@1.0.0`, exactly `{lifecycle_transition,posture_transition,resulting_kernel}` |

Non-`ok` outcomes have no receipts. Replay returns the same data and receipt
bytes and claims no new writes. The semantic result fingerprint is therefore
identical to the original. The replay state sets
`original_result_fingerprint` to that recomputed semantic fingerprint. The
engine's committed posture journal has no expiry or cleanup path, so the owner
projection exposes the maximum canonical timestamp as its retention bound.
Each declared condition-schema fingerprint hashes its exact checked-in schema
bytes including the terminal LF. Each realized receipt fingerprint is computed
by the existing `WriteReceipt` preimage over record kind, exact runtime record
binding, authority class, condition, and atomic group; definition-bound
validation checks the condition against the corresponding declared schema. No
private record bytes enter either preimage.

## Exhaustive owner mapping

Each non-success result has one Problem, empty evidence, diagnostics, and next
actions, and no receipts. `details` is exactly
`{owner_variant: <original Rust variant>}` under
`handbook.posture-details.<lower-kebab-variant>@1.0.0`. Refusal rules are
`handbook.posture-rule.<lower-kebab-variant>@1.0.0` bound to the exact admitted
engine owner-source fingerprint; blocker and error rules are null. The Problem
subject is the exact selected operation-definition binding.

| Owner result | Terminal / stage | Category, code, retry, and extra binding |
| --- | --- | --- |
| `Applied` | `ok` / established | seven-field data; original-result null; two receipts |
| `Replayed` | `ok` / established | identical data and receipts; original-result equals semantic result |
| `Blocked(UnsupportedPlatform)` | `blocked` / `capability_validation` | `prerequisite`; `posture.unsupported_platform`; `after_recheck`; exact unsupported-platform recheck |
| `Refused(InvalidIdempotencyKey)` | `refused` / `request_validation` | `schema`; `posture.invalid_idempotency_key`; `after_request_change` |
| `Refused(RepositoryIdentityMismatch)` | `refused` / `authority_validation` | `authority`; `posture.repository_identity_mismatch`; `after_request_change` |
| `Refused(InvalidFixedDimension)` | `refused` / `precondition_validation` | `precondition`; `posture.invalid_fixed_dimension`; `after_request_change` |
| `Refused(StaleCanonical)` | `refused` / `precondition_validation` | `precondition`; `posture.stale_canonical`; `after_request_change` |
| `Refused(MissingLifecycleAuthority)` | `refused` / `authority_validation` | `authority`; `posture.missing_lifecycle_authority`; `after_authority_change` |
| `Refused(LifecycleNotCurrent)` | `refused` / `precondition_validation` | `precondition`; `posture.lifecycle_not_current`; `after_authority_change` |
| `Refused(InvalidSemanticInput)` | `refused` / `precondition_validation` | `precondition`; `posture.invalid_semantic_input`; `after_request_change` |
| `Refused(AuthorityConflict)` | `refused` / `authority_validation` | `authority`; `posture.authority_conflict`; `after_request_change` |
| `Error(UnsafeFilesystem)` | `error` / `execution_start` | `implementation`; `posture.unsafe_filesystem`; `never`; deterministic correlation |
| `Error(IntegrityOrDurability)` | `error` / `execution_start` | `implementation`; `posture.integrity_or_durability`; `never`; deterministic correlation |
| `Error(Io)` | `error` / `execution_start` | `implementation`; `posture.io`; `transient`; deterministic correlation |

The unsupported-platform recheck schema is
`handbook.posture-recheck.unsupported-platform@1.0.0` and its condition is the
same one-field owner-variant projection. Error correlations are domain-separated
hashes of the request fingerprint and exact owner variant; they contain no raw
key or owner text.

## Schemas, owner admission, and current transport

The checked-in manifest adds the exact operation request and result schemas,
twelve owner-variant detail schemas, the unsupported-platform recheck schema,
and the two declared-write condition schemas. Production construction requires
all of those exact bytes. The engine independently selects the complete exact
definition claim, including every generic and operation schema binding,
idempotency and write-set fingerprints, and empty required-capability set,
before its private admission token can exist.

The exact inventory state model adds `AdmissionReason::Admitted` and
`DiscoveryState::Discoverable`. `OperationInventoryRow` stores its current
transport slice and discovery state explicitly; all other rows retain their
Packet-2 reason, `[]`, and `Omit`. Only after full closure does M62-027 change
to `Admitted`, `[RustSdk]`, and `Discoverable`. At that point the Packet-2
canonical tuple digest is mechanically recomputed with its one changed current
admission value and replaces the old digest in the checkpoint and executable
guard; transport and discovery are additionally checked as exact independent
fields. The resulting digest is
`sha256:6993971ab59c0b6d98485b7a6eff7c06b87f8dcc350837a2cdf2ccc1cd5d6d1f`.
After this closure, that value supersedes Packet 2's historical current-state
digest; before closure, the Packet-2 digest remains authoritative.
If closure fails, neither enums/state nor digest are promoted.

`rust_sdk` becomes current only if the same bounded implementation proves all
of the following together: exact definition and checked-in schemas, engine
admission, raw-key bridge confinement, exhaustive owner mapping, declared and
realized write equality, applied/replayed semantic identity, request-ID
correlation behavior, unknown-operation default denial, bootstrap/catalog
binding, and runtime success/refusal/error evidence. Only then may the frozen
62-row inventory change the posture row to admitted/discoverable with current
transport `[rust_sdk]`, and only that capability may enter production
bootstrap. Any failed closure leaves the row `incomplete_closure`, transport
`none`, and discovery omitted; declared targets never substitute for proof.

This packet adds no CLI or Tauri adapter, other operation admission, broader
posture policy, future owner, publication, bridge, runtime, or dock decision.
