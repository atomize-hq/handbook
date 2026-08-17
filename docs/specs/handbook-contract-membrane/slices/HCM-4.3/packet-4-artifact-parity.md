# HCM-4.3 Packet 4 — artifact parity decisions

This compact checkpoint records the slice-delegated Packet-4 decisions selected
from the HCM-4.3 authority, the completed HCM-4.2 operation/admission boundary,
and the current Engine, SDK, and CLI artifact paths before product edits. The
slice README remains controlling for scope and later packets.

## Exact ordinary-operation boundary

The ten authority-fixed `@1.0.0` rows remain ordinary Engine-owned operations
in invocation order: `artifact.kind.list`, `artifact.instance.list`,
`artifact.read`, `artifact.validate`, `intake.definition.read`,
`intake.coverage.evaluate`, `intake.record.append`,
`artifact.candidate.validate`, `artifact.candidate.append`, and
`artifact.candidate.promote`. No artifact use case or additional inventory row
is introduced.

Each row gains one exact definition, request root, result root, Engine-native
exact admission claim, and `cli_json` evidence. Repository-root and input-file
locators remain CLI transport facts and do not enter public operation bodies.
Read/list bodies contain only their semantic selection fields; mutation bodies
contain the existing owner request fields plus the definition-pinned raw
`/body/idempotency_key`, which is removed from admitted public state and passed
to the Engine only through the sealed owner-key bridge.

The existing Engine artifact repository and mutation services remain the only
semantic execution paths. SDK operation dispatch admits the request and the
exact owner definition before calling one service once, then exhaustively maps
owner-native result and error enums by cause. It does not classify owner text.
Operation success data and problem details use closed typed artifact
projections; arbitrary `serde_json::Value` assembly is removed from active CLI
execution and rendering.

The exact per-row closure is frozen as follows. Every request/result root uses
`handbook.operation.<operation-id-with-hyphens>-{request,result}@1.0.0`.
Every problem uses `handbook.artifact-operation-details@1.0.0`; blocked
problems additionally bind exactly one of
`handbook.artifact-recheck.repository-authority@1.0.0` or
`handbook.artifact-recheck.supported-platform@1.0.0`. Read-only rows use
class `read_only`, idempotency `safe`, and no declared writes.

| Operation | Semantic request body | Consequential result facts | Mutation closure |
|---|---|---|---|
| `artifact.kind.list` | `{}` | operation ID, selection fingerprint, ordered kind ref/definition fingerprint/schema ref rows | read only |
| `artifact.instance.list` | `{}` | operation ID, selection fingerprint, ordered instance ID/kind ref/canonical path/intake-definition ref rows | read only |
| `artifact.read` | kind ref, instance ID | operation/context IDs, selected target, canonical path, artifact fingerprint, closed content | read only |
| `artifact.validate` | kind ref, instance ID | operation/context IDs, artifact fingerprint, `valid`, closed content, all five layer results | read only |
| `intake.definition.read` | kind ref, instance ID | operation ID, target, intake definition ref/fingerprint, candidate schema ref, ordered coverage rows | read only |
| `intake.coverage.evaluate` | target, acquisition mode, expected current artifact fingerprint, coverage submissions | schema/version, context/mode/basis, coverage results, normalized content, field sources, outcome, evaluation fingerprint | read only |
| `intake.record.append` | target, acquisition mode, expected current artifact fingerprint, coverage submissions, raw key | semantic mutation result facts below | append-only/key required; `intake_record`/`semantic_record`/record `handbook.artifact-record.intake-record@1.0.0`/group `intake_record_append`/condition `handbook.artifact-write-condition.semantic-record@1.0.0` |
| `artifact.candidate.validate` | target, intake record ref/fingerprint, expected current artifact fingerprint | operation/outcome/context, intake and basis refs, normalized content/ref/fingerprint, validation result/ref/fingerprint, candidate/ref/fingerprint | read only |
| `artifact.candidate.append` | target, intake record ref/fingerprint, expected candidate fingerprint, raw key | semantic mutation result facts below | append-only/key required; `artifact_candidate`/`semantic_record`/record `handbook.artifact-record.candidate@1.0.0`/group `artifact_candidate_append`/condition `handbook.artifact-write-condition.semantic-record@1.0.0` |
| `artifact.candidate.promote` | target, candidate ref/fingerprint, expected current artifact fingerprint, optional publisher authority, raw key | semantic mutation result facts below | compare-and-write/key required; expected basis is target, candidate ref/fingerprint, expected current artifact fingerprint, and publisher authority; `canonical_artifact`/`canonical_truth`/record `handbook.artifact-record.canonical-artifact@1.0.0` plus `promotion_record`/`semantic_record`/record `handbook.artifact-record.promotion-record@1.0.0`, both group `artifact_candidate_promote`, using `handbook.artifact-write-condition.canonical-truth@1.0.0` and `handbook.artifact-write-condition.semantic-record@1.0.0` respectively |

The semantic mutation result contains schema ID/version, operation and
transaction IDs, owner request fingerprint, outcome, optional exact refusal,
and ordered authoritative public outputs plus the owner result fingerprint.
`execution_disposition` is renderer state derived from the response's
idempotency state and is not semantic result data. The bounded
transaction-evidence ref/fingerprint remain the exact public compatibility
binding needed to reconstruct the deprecated document. They are carried in
typed mutation success data and in typed refusal problem details; they are not
receipts. The referenced evidence bytes, journal contents, and
subordinate-output inventory remain private and never cross the membrane.

## Result and receipt closure

Read, list, validation, intake-definition, coverage-evaluation, and
candidate-validation results retain every current public field and ordering.
The recursive content members remain bounded contract values inside exact
closed containing DTOs; they do not expose repository-private records or host
paths.

The three mutation results retain the current semantic legacy fields, including
refusal cause, authoritative output references, and fingerprints.
For each realized output, the Engine maps its exact operation and authority
class to the fixed runtime record binding above; the SDK may validate and
project that owner-native binding but may not synthesize a binding from output
ordinal or prose.
The generic owner store proves exactly one public authoritative output for each
successful intake or candidate append and exactly two for successful promotion;
installed subordinate-closure outputs are private closure, while scratch,
staging, journals, markers, and evidence are private transaction machinery.
None is a membrane receipt. The fixed public write sets above become
definition-bound receipts. Refusal has no public write receipt. A failure before
public commit is not promoted into a receipt; no receipt is inferred from file
presence or fabricated after an unclassified error. The HCM promotion closeout
that follows a committed generic-store result must recover/project that exact
established committed or replayed result if closeout initially fails; it may not
return non-ok after canonical truth and the promotion semantic record are
public.

Replay preserves the originally established terminal, problem/data, receipts,
and semantic-result fingerprint. Thus replay of a commit remains `ok`, while
replay of a retained refusal remains `refused`; only the legacy renderer derives
`execution_disposition: replayed` from the idempotency state.

The Engine adds only the closed causes needed to avoid prose classification.
The exact cause mapping is:

| Owner cause | Terminal / problem / retry |
|---|---|
| selection read, profile/registry/intake-source unavailable, repository identity unavailable, artifact read unavailable | `blocked` / `prerequisite` / `after_recheck` with repository-authority recheck |
| unsupported platform | `blocked` / `prerequisite` / `after_recheck` with supported-platform recheck |
| selection or intake admission | `refused` / `schema` / `after_authority_change` |
| canonical syntax, structural validation, invalid request syntax | `refused` / `schema` / `after_request_change` |
| target mismatch or target resolution | `refused` / `resolution` / `after_request_change` |
| operation not applicable | `refused` / `capability` / `never` |
| stale operation context, current artifact, or basis | `refused` / `precondition` / `after_request_change` |
| idempotency conflict or expiry | `refused` / `idempotency` / `after_request_change` |
| unsafe caller reference | `refused` / `safety` / `never` |
| request/document bound exceeded | `refused` / `safety` / `after_request_change` |
| unsafe owner filesystem | `error` / `implementation` / `never` with deterministic correlation |
| retained owner-state bound exceeded | `error` / `implementation` / `never` with deterministic correlation |
| owner I/O | `error` / `implementation` / `transient` with deterministic correlation |
| invalid plan/refusal/intent/marker/staging, conflicting retained state, retained-result mismatch | `error` / `implementation` / `never` with deterministic correlation |

Candidate validation exposes its closed target, authority, intake-record,
currentness, validation, unsafe-reference, unsafe-filesystem, retained-bound,
and owner-I/O causes instead of collapsing them under `Evaluation`. Repository
artifact-operation errors retain the underlying operation cause. Mutation
request bounds remain distinct from retained-store bounds, and mutation store
errors retain the underlying store cause.
The SDK exhaustively matches these enums; owner detail text may be rendered but
never selects status, category, retry, receipt, or exit.

## CLI projections and compatibility

The CLI acquires transport-local input, constructs one admitted request, and
receives one `OperationResponseV1` before selecting a renderer. `--json`
serializes one `InvocationResponseV1` plus LF through the shared single-write
emitter. Existing command-specific JSON remains only under `--legacy-json`.
Default human output and legacy JSON are reconstructed from the ordinary
response and keep the current pretty/compact bytes, field names, order, LF,
and terminal exit parity. A semantic artifact refusal is rendered once on
stdout and the duplicate `artifact operation refused: ...` stderr line is
removed. Transport-operational input or physical emission failure remains
bounded by the shared failure rules.

Only the ten fixed inventory rows become admitted, discoverable, and have the
exact current transport vector `[cli_json]`; posture keeps exactly `[rust_sdk]`
and every unrelated row keeps its current admission, transport, and discovery
state.
