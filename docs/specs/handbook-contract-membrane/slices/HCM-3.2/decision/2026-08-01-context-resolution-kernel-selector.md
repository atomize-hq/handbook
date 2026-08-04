# HCM-3.2 Context Resolution Kernel Selector

Status: fresh JCS-capsule and replacement-recovery planning subject; an
independent v1.4 CLEAN review is required before any test, RED, or Rust edit.

Selected slice: `HCM-3.2` only

Parent orchestration: `20260804T124829Z--HCM-3-2--context-resolution-jcs-recovery`

Integrated outcome: `hcm-3.2-context-resolution-jcs-recovery-whole-slice`

## Selection, identity, and historical boundary

The operator selected one genuinely fresh whole-slice parent from commit
`9368b5cee612bdb1ffd25562eef35e8d863ff930`, tree
`08f7810d8316a42daf8aae8bcb3bc5b8cdaf87d8`, with
`HANDOFF_SELECTOR=none`. HCM-3.1 is completed predecessor context; its shipped
vocabulary, profile, and definition closure are immutable. HCM-3.3 and every
later slice remain unselected.

The stopped HCM-3.2 parents, handoff
`20260803T060000Z--HCM-3-2--orchestration--authority-binding-lineage-required`,
blocked codec receipt, off-ref commits `f42ac62c759e47e6c16d37dabc87acab48463cea`,
`01aaae7105bbe02ed403aa4063133b13cde542f7`,
`30cb1eb6baeb475e52d39d80e8629f2c7a899d29`,
`b4efb0f02a67fbd4e3b85ce51951c977a8312642`, and
`a123fed8fe0bd9b546fa7884a0d596e22e6f2fb9` are immutable source evidence.
This parent neither continues nor supersedes them and reuses none of their
parent, outcome, packet, cycle, finding, or budget identities. All findings,
remediations, blockers, security-floor decisions, and three adversarial-review
results are known constraints. They are not waived, renumbered, or presented
as fresh discovery.

## Orchestration registry and cadence

The exact outcome registry preimage is this canonical JSON plus one LF:

```json
[{"authority_ref":"docs/specs/handbook-contract-membrane/slices/HCM-3.2/decision/2026-08-01-context-resolution-kernel-selector.md","integrated_outcome_id":"hcm-3.2-context-resolution-jcs-recovery-whole-slice","packet_ids":["HCM-3.2-JCS-P1-selector-authority","HCM-3.2-JCS-P2-capsule-authority-kernel","HCM-3.2-JCS-P3-replacement-recovery","HCM-3.2-JCS-P4-proof","HCM-3.2-JCS-P5-final-closeout"]}]
```

Registry fingerprint:
`sha256:fedd78206b2b1d1b830bb26b084b90684733f0a71c85e9f6ca5637c697358753`.
Causal-budget fingerprint:
`sha256:34ccbe5a9fb4f6ec07b588e2354166c876ff82dee36b3559097acafde66d82b2`.
The ordinary v1.4 planning, implementation, proof, and final-closeout stages
each use one complete-subject discovery or same-fingerprint burst, one
consolidated remediation, one different-fresh closure, and at most two
immediately causal P1/P2 supplementals. No cycle follows CLEAN.

## Known-finding disposition

Prior planning and selector findings `HCM32-PLN-DISC-001..004`,
`HCM32-SEL-DISC-001..005`, `HCM32-FRESH-SEL-DISC-001..003`, and
`HCM32-FRESH-SEL-CLOS-001` supplied historical constraints already folded into
this complete subject. Prior implementation findings
`HCM32-FRESH-IMPL-DISC-001..004` are represented by their known bounded
remedies. `HCM32-FRESH-IMPL-DISC-005`,
`HCM32-FRESH-IMPL-SUPP-CODEC-001..002`,
`HCM32-FRESH-IMPL-SUPP-CODEC-CLOS-001`,
`HCM32-SEC-SEL-DISC-001..002`, and the open P1
`HCM32-SEC-IMPL-DISC-001` remain carried safety obligations. The JCS capsule,
private proof states, currentness rules, and four-part recovery floor below
are the authorized product response; only fresh implementation and proof can
close the carried open obligations.

Fresh planning discovery returned `HCM32-JCS-PLN-DISC-001..004` as four P2
control findings. This amended subject is their single consolidated
remediation: exact generic/HCM fingerprint split, satisfiable layered limits,
closed publisher syntax/semantics, and registry-head-driven no-journal
recovery. They remain blocking until a different fresh closure returns CLEAN.

## Runtime owner and exact compatibility tuple

`handbook-engine` is the sole runtime owner. It consumes the exact selected
`ResolvedInstanceProfile::context_resolution()` and emits immutable engine
values. No pipeline, compiler, CLI, SDK, Tauri, Substrate, HTTP, dock, or
publication consumer is added.

The private HCM-3.2 v1 capsule has this closed compatibility tuple:

| domain | exact value |
|---|---|
| artifact kind | `handbook.artifact-kind.context-resolution-authority-binding@1.0.0` |
| artifact instance | `context_resolution_authority` |
| canonical path | `.handbook/project/context-resolution-authority.yaml` |
| wrapper schema | `handbook.schemas.context-resolution-authority-capsule@1.0.0` |
| intake definition | `handbook.intake.context-resolution-authority-capsule@1.0.0` |
| sole wrapper terminal | `/declaration_jcs` required string |
| declaration schema | `handbook.context-resolution-authority-declaration@1.0` |
| codec | `handbook.hcm-3.2.context-resolution-authority-binding-jcs@1.0` |
| payload schema | `handbook.context-resolution-authority-binding@1.0` |

Generic intake validates only the closed one-terminal wrapper and provenance
for `/declaration_jcs`. It never claims inner semantic validity or operational
authority. The HCM-owned gate alone parses the exact UTF-8 terminal as RFC
8785/JCS, requires byte-identical re-emission, and validates the declaration
and payload. Historical decoders are immutable and append-only; an unknown or
reinterpreted tuple fails closed.

The declaration has exactly nine members:
`schema_id`, `schema_version`, `codec_id`, `codec_version`, `payload_jcs`,
`payload_byte_fingerprint`, `semantic_binding_fingerprint`, `predecessor`, and
`prior_registry`. `predecessor` has exactly
`outer_artifact_ref`, `outer_artifact_fingerprint`,
`payload_byte_fingerprint`, and `semantic_binding_fingerprint`; a root uses
literal null for all four, while a replacement uses exact non-null strings for
all four. Mixed-null quartets refuse. `prior_registry` has exactly
`state_ref`, `state_fingerprint`, `head_transition_ref`, and
`head_transition_fingerprint` and equals the payload registry quartet.

The inner payload has the exact 53-leaf JSON-pointer manifest:

```text
/schema_id /schema_version /binding_id /binding_version
/repository_identity_fingerprint
/resolved_profile/ref /resolved_profile/fingerprint
/resolution_stack/ref /resolution_stack/fingerprint
/approver_registry/state_ref /approver_registry/state_fingerprint
/approver_registry/head_transition_ref /approver_registry/head_transition_fingerprint
/authority_mappings/{root_creation,parent,requested,approving,decision,evidence,trigger_condition,constraint,source,target,target_memory}/{approval_class,authority_ref}
/candidate_mappings/{dimension_rank_increase,mutation_allow_expansion,missing_context,missing_authority}/trigger/ref
/candidate_mappings/{dimension_rank_increase,mutation_allow_expansion,missing_context,missing_authority}/trigger/fingerprint
/candidate_mappings/{dimension_rank_increase,mutation_allow_expansion,missing_context,missing_authority}/missing_condition
/candidate_mappings/{dimension_rank_increase,mutation_allow_expansion,missing_context,missing_authority}/evidence_requirement
/extensions /binding_fingerprint
```

Every leaf receives complete semantic validation. Generic lineage supplies
atomic wrapper authorship and replacement, not per-leaf provenance, partial
updates, queries, or schema completion. This deliberate trade-off is accepted
for this private capsule. A second artifact family, consumer, public
interchange need, or generic atomic-document facility requires new authority.

Before allocation-heavy validation, reject BOM, invalid UTF-8/I-JSON,
duplicates, trailing bytes, alternate encodings, extra or missing members,
excess depth, member count, string bytes, escapes, or escape expansion. Bounds
are measured at explicit layers:

- the complete canonical YAML artifact is at most 131,072 bytes;
- the decoded UTF-8 content of wrapper `/declaration_jcs` is at most 98,304
  bytes; this is the first carrier exception to the ordinary-string limit;
- the decoded UTF-8 content of declaration `/payload_jcs` is at most 81,920
  bytes; this is the second and final exception;
- every other decoded JSON string in declaration or payload is at most 8,192
  UTF-8 bytes;
- aggregate object-member count across declaration and payload is at most 192;
- maximum JSON container depth is 12, with each root object at depth 1 and
  each nested object/array incrementing depth by one; scalars do not increment;
- across the declaration parse and payload parse together, escaped source
  tokens may produce at most 16,384 decoded UTF-8 bytes. A byte is attributed
  to escape expansion when emitted from a JSON escape token (`\"`, `\\`,
  `\/`, control escape, or one valid `\uXXXX`/surrogate pair), and is counted
  once at the layer that decodes it.

The declaration and payload input lengths are their exact decoded UTF-8 byte
lengths before JSON parsing. Limit-minus-one, exact-limit, and limit-plus-one
vectors cover both exceptions, ordinary strings, aggregate escapes, depth,
members, and one jointly satisfiable maximum artifact.

## Fingerprint domains and preimages

The following domains are distinct and non-substitutable. Domains 1-3 retain
the exact existing generic algorithms without tags or new preimages:

1. generic terminal value = `sha256(canonical_json(JSON string value))`, where
   the canonical JSON bytes include the enclosing quotes and JSON escapes;
2. normalized wrapper = `sha256(canonical_json({"declaration_jcs": value}))`;
3. outer canonical artifact = `sha256(canonical_yaml_bytes(normalized_wrapper))`,
   including exactly the bytes and terminal newline emitted by the existing
   canonical YAML writer.

HCM-owned domains 4-6 use `sha256(tag || u64be(len) || bytes)` with no separator
or terminal newline. `u64be` is one unsigned 64-bit big-endian byte length and
`tag` is the literal ASCII byte sequence shown, including its final NUL:

4. raw declaration tag `handbook.hcm-3.2.declaration-jcs.v1\0`, bytes = exact
   byte-identical JCS declaration input;
5. payload bytes tag `handbook.hcm-3.2.payload-jcs.v1\0`, bytes = exact decoded
   UTF-8 JCS bytes stored in `payload_jcs`;
6. semantic binding tag `handbook.hcm-3.2.semantic-binding.v1\0`, bytes = exact
   RFC 8785/JCS serialization of the payload after removing only the root
   `binding_fingerprint` member.

The declaration's two inner fingerprints and payload binding fingerprint must
match their own domains. The generic value, wrapper, and artifact fingerprints
must match the committed generic promotion. No field may contain or hash its
own final enclosing fingerprint, so no self-reference cycle is admitted.
Focused proof freezes one golden preimage and lowercase `sha256:` digest for
each domain, shows domains 1-3 equal committed generic value/candidate/promotion
records, and substitutes every digest across every other domain.

## Publisher authority and real owner pipeline

The capsule is authoritative only after the real owner pipeline: empty or
current selected target -> generic intake -> candidate -> HCM semantic gate ->
authenticated registry promotion authorizing the exact output -> generic
promotion -> durable committed lineage -> fresh HCM current-authority witness.
Direct fixture seeding, generic `eligible_without_approval`, ungated committed
reads, cache hits, clones, and restored old bytes never create authority.

The generic promotion request retains one HCM-only closed
`publisher_authority` object. It is forbidden for every other artifact tuple
and has exactly these nine members:

```json
{
  "schema_id": "handbook.context-resolution-publisher-authority",
  "schema_version": "1.0",
  "result_registry_state_ref": "registry-states/registry-state_<64hex>.json",
  "result_registry_state_fingerprint": "sha256:<64hex>",
  "result_transition_ref": "registry-transitions/registry-transition_<64hex>.json",
  "result_transition_fingerprint": "sha256:<64hex>",
  "publisher_credential_id_hash": "sha256:<64hex>",
  "challenge_jcs_base64": "<RFC4648 standard base64 with required padding>",
  "raw_assertion_response_base64": "<RFC4648 standard base64 with required padding>"
}
```

Every ref is normalized repository-relative ASCII and at most 512 bytes; every
fingerprint is lowercase SHA-256. Base64 must decode and re-encode byte-
identically using RFC 4648 standard alphabet with padding. Decoded challenge
and raw response are each non-empty and at most 16,384 bytes; the complete
object's canonical JSON is at most 64 KiB.

The challenge is exact retained JCS for the existing closed eleven-member
`handbook.authenticator-challenge@1.0` record: `$schema`,
`changed_credential_id_hash`, `nonce_base64`, `operation`, `operation_id`,
`prior_registry_state_fingerprint`, `prior_transition_fingerprint`,
`repository_identity_fingerprint`, `result_registry_state_fingerprint`,
`schema_id`, and `schema_version`. `operation` is `update_mapping` and
`operation_id` is exactly `hcm32-authorize-<outer64hex>`, where `<outer64hex>`
is domain-3 without `sha256:`. The result transition's exact
`authorization_ref` and `authorization_fingerprint` select one retained
assertion record. That assertion's exact `decoded_response_ref` and
`decoded_response_fingerprint` select one retained assertion-response record.
The publisher object's `raw_assertion_response_base64` must strictly decode and
standard-base64 re-encode byte-identically, and the decoded bytes must equal
the strict decoded and byte-identically re-encoded `raw_response_base64`
member of that response record. The two-hop refs/fingerprints, decoded
credential, client-data hash, signature, RP, presence, verification, and
counter all reverify against the retained challenge and credential.

The transition must be the live direct successor from the declaration's
`prior_registry` quartet and must update exactly the credential named by
`publisher_credential_id_hash`. That active credential is also the exact
asserting administrator and, before H2, covers both
`registry_admin` / `repository_registry_admin` and
`context_resolution_authority_publisher` /
`repository_context_resolution_authority_publisher`. H2 preserves every prior
credential, status, unrelated mapping, and unrelated registry field. For the
same publisher/administrator credential it admits only the exact existing
authenticated-use advancement (`sign_count`, `use_sequence`, and use-head
ref/fingerprint) plus adding one mapping:
`context_resolution_authority_publication` /
`context-resolution-authority/<outer64hex>`. Zero or multiple credentials with
the exact publication mapping, any removed/changed prior mapping, an invalid
counter/use-head advance, or any extra state delta refuses.

Existing registry authentication, repository identity, credential coverage,
response verification, and committed currentness are reused unchanged.
Candidate eligibility runs the complete HCM capsule/semantic gate and may
commit only non-authoritative candidate evidence before H2; it does not require
or convey publisher authority. Promotion planning, persisted-output validation,
recovery, current reads, and admission require the exact publication proof
before the output can become authoritative. Missing/extra/duplicate fields,
wrong base64, credential, challenge, assertion, transition, mapping,
fingerprint, or non-HCM use refuse.

For each authority use, the HCM resolver maps the payload pair to active
non-exhausted ES256 credentials, obtains one fresh authenticator assertion over
a domain-separated challenge binding repository, live registry head,
outer/declaration/payload/semantic fingerprints, use, and exact subject, and
verifies RP, presence, verification, credential selection, signature, and
counter. No candidate, cardinality ambiguity, refusal, stale registry,
revocation, entropy failure, changed subject, or replay mismatch yields an
admission.

## Private typed states, currentness, and replacement

Four private non-interchangeable states are selected:

- `HistoricalCommittedProof`: audit/recovery proof only;
- `PublicationAuthorizedPendingCommit`: exact output is authorized for the
  pending generic journal but conveys no operational capability;
- `CurrentAuthorityWitness`: exact durable committed output is the live direct
  successor registry mapping and may back operational admissions;
- `CorruptOrAmbiguous`: bounded diagnostics and refusal only.

There is no caller-selected historical/current mode flag. Historical or
pending values cannot construct, return, cache, clone, or satisfy
`ContextResolutionAuthorityAdmission`. The types remain private to the HCM
owner and their constructors are phase-specific.

Every resolver load, cache hit, cloned/admission consumption, candidate,
promotion, envelope, mutation, memory, validation, escalation, and transition
operation revalidates exact canonical bytes, committed generic lineage, live
direct-successor registry head/mapping, active publisher credentials, exact
predecessor, repository/profile/stack identity, and all fingerprint domains.
A later head, revocation, mapping loss, profile/stack/repository change,
restored old bytes, or lineage mismatch makes the admission unusable.

For O1 -> O2, H2 authorizes O2 before T2 is durably committed. During that
interval neither O1 nor O2 is operational. O1 remains historical only and is
never reactivated after failure. Once T2 is committed, O2 alone is current.
Each artifact binds its own predecessor and prior-registry quartet; proof
borrowing across versions is impossible.

Superseded O1 remains historically verifiable after O2 owns the canonical path
using retained exact displaced bytes plus exactly one committed direct-
successor lineage witness. Historical verification still proves O1's exact
bytes, H0 -> H1 authenticated publication, publisher, repository/profile/stack,
lineage, and outer/declaration/payload/semantic fingerprints. It creates no
capability.

Every crash boundary converges by exact retry or records a diagnosable
fail-closed HCM quarantine. Installed-but-uncommitted bytes are never readable
as authority. An installed exact output with a missing durable native marker
reconstructs the one truthful publication result from durable pre-observation,
canonical/displaced identities, registry transition, and journal request, then
continues idempotently.

The H2 -> T2 no-journal gap uses registry-head-driven discovery, not a new
registry or generic-intake schema. Before H2, the candidate and all closure
records are already durable generic committed outputs. H2 durably retains the
exact challenge, raw response, assertion, result state, transition, outer
fingerprint mapping, and operation ID. The generic promotion idempotency key is
exactly `hcm32crpub_<outer64hex>` (75 allowed ASCII bytes), so it is
deterministic from H2.

On every HCM candidate, promotion, resolver-load, and authority-consumption
entry, the HCM owner first acquires `ArtifactRepositoryAuthorityGuardV1`
(`promotion.lock`, then `registry.lock`) and then invokes HCM recovery under
`generic-artifact-operations.lock`. No path acquires those locks in reverse.
Under those locks it reads the live transition's exact prior/result states. A
transition whose only state delta is the exact publication mapping and whose
operation ID names its outer fingerprint is an H2 trigger.

For that trigger, recovery searches committed candidates only for the exact
HCM kind/instance and accepts one whose replayed normalized wrapper and
canonical YAML produce the mapped outer fingerprint, whose basis equals the
declaration predecessor, and whose retained intake/validation closures all
verify. It then classifies exactly one of:

- matching committed T2: verify and expose current witness;
- matching pending T2: run normal generic recovery;
- no journal with one candidate: record an open diagnostic, and an exact
  caller retry using that candidate, publisher object, predecessor, and
  deterministic key may create T2;
- no candidate, multiple candidates, invalid candidate, journal/request
  conflict, wrong canonical predecessor, or ambiguous registry delta: record
  fail-closed quarantine and refuse HCM authority.

The durable record path is
`.handbook/state/context-resolution-authority/quarantine/quarantine_<transition64hex>.json`.
It is closed `handbook.context-resolution-publication-quarantine@1.0` with exact
members: `schema_id`, `schema_version`,
`repository_identity_fingerprint`, `result_registry_state_ref`,
`result_registry_state_fingerprint`, `result_transition_ref`,
`result_transition_fingerprint`, `outer_artifact_fingerprint`,
`observed_canonical_artifact_fingerprint` (string or null), `candidate_ref`
(string or null), `candidate_fingerprint` (string or null), `idempotency_key`,
`journal_transaction_id` (string or null), `phase`, `reason`, `resolution`,
`result_transaction_ref` (string or null), `result_transaction_fingerprint`
(string or null), and `record_fingerprint`. `record_fingerprint` is existing
canonical-JSON SHA-256 with itself omitted. `phase` is one of
`authorized_without_generic_intent`, `generic_pending`, or
`installed_without_commit`; `reason` is one of `retry_available`,
`candidate_missing`, `candidate_ambiguous`, `candidate_invalid`,
`predecessor_mismatch`, `journal_conflict`, `installed_result_ambiguous`, or
`registry_delta_ambiguous`; `resolution` is `open` or `committed`.

An open `retry_available` record admits only the exact existing `promote`
retry. After its normal generic T2 commit, recovery replaces the record
durably with `resolution=committed` and the exact retained result pair. Every
other open reason requires explicit future repair authority and remains
non-capability. The quarantine directory contains at most one record for each
transition and at most 64 records total; overflow, unknown records, or changed
same-transition bytes refuse HCM recovery. Cold restart without a retry request
still discovers and records the gap. Concurrent retry serializes under the
same locks. Unrelated generic recovery continues, and no quarantine relaxes
unknown-family refusal or creates authority.

## Definition identity selector

Preserve byte-for-byte and do not version:

- shipped stack ref/fingerprint
  `handbook.context-resolution.shipped-root@1.0.0` /
  `sha256:9e95fdef90b98e28acb60bfe96a72f122418b56b1531eafe8ab6cff3eb7668b4`;
- matcher ref/fingerprint `handbook.mutation-matcher.core@1.0.0` /
  `sha256:be585006043ca46096e85ad76f20a87439e6984bcace997924982f0247aa29f4`;
- escalation ref/fingerprint `handbook.resolution-escalation.core@1.0.0` /
  `sha256:1f7e04ed7d8a68f338b2c421db8ef0c49f16d9b4dcb001145083414efa408121`;
- memory-promotion ref/fingerprint `handbook.memory-promotion.core@1.0.0` /
  `sha256:1f0e07938432a159d2ba9c131ed6dc28536ec8aa30b817f24066e246c06c0bca`.

General stack definitions use the same schema/policies and uniform exact
identity algorithm. They are test/repository definitions, not new shipped
defaults. Same ref with different bytes, stale dependency fingerprints,
range/latest lookup, remote loading, and executable hooks refuse.

## Ordered-stack and six-dimension selector

The list is broad-to-narrow, non-empty, and linear. Stable level IDs are unique;
display labels are presentation only and may repeat. Exactly six non-empty
domains exist. Each has unique value IDs and contiguous ranks from zero, where
zero grants least reach/disclosure/durability/claim authority. Every level has
all six defaults, and each adjacent narrower level may preserve or lower each
rank but never raise one.

The active level selects a named default position only. It is not an authority
score. Envelope dimensions remain complete and independently comparable, so
two envelopes at one level may differ in any subset of dimensions.

## Inheritance selector

Root envelopes have no parent. Children cite exactly one supplied parent by
ref/fingerprint and repeat complete effective state. Parent/profile/stack
identity is equality-checked. Every child rank must be less than or equal to
the corresponding parent rank. Any increase returns no envelope and one typed
`dimension_rank_increase` escalation candidate naming the changed dimensions.

Candidate construction uses only the four exact binding-record mappings above.
The requested authority is the admitted `requested` pair. Dimension and
mutation candidates bind the actual current/proposed envelope inputs; missing
context/authority candidates require caller-supplied exact evidence satisfying
their frozen requirement. No other candidate class or free-form trigger is
admitted.

Constraint inputs are exact sorted ref/fingerprint pairs and cannot become
merge parents. Lower-horizon observations cannot mutate higher authority.

## Mutation selector

Only `repository_path` is admitted. The exact core grammar and byte/segment
limits are enforced before evaluation. Matching is case-sensitive. Root grants
are allow-minus-deny; child grants are parent-intersection-child-minus-all-deny.
Empty allows deny all. Deny always wins a valid overlap.

Containment, if later authorized, compares the positive parent allow language;
inherited denies are accumulated separately and always subtract from the
result. A deny hole does not make an otherwise equal child allow an expansion,
and a child can never remove or weaken that hole. Child allow containment is
proved under the closed grammar. Exact equality,
literal specialization of a parent segment wildcard, and a child path/pattern
beneath a matching parent terminal `/**` prefix are admitted. A child terminal
recursive selector is admitted only beneath an equal or broader parent terminal
recursive selector. Any ambiguous pattern-language containment refuses as
indeterminate; it never guesses or grants. Possible expansion returns no child
and a typed `mutation_allow_expansion` candidate.

Malformed/unresolvable selector or target, unknown target kind, stale matcher,
or evaluation indeterminacy refuses. The matcher reads no filesystem state.

## Memory, validation, and escalation selector

Memory/validation compare requested and envelope ranks in their own domains.
At-or-below is authorized. Higher memory returns promotion-required; higher
validation returns not-authorized. Neither result mutates state or creates
green proof.

Escalation and promotion requests/dispositions are separate immutable records.
The registry admits one terminal disposition per exact request and rejects
duplicate, stale, changed-ID, invalid-outcome, or forbidden-authority records.
Requests have no preapproval effect. Approved escalation cites an exact
replacement envelope; applied promotion cites an exact new semantic-memory
result after compare-and-write validation. Other outcomes cite no result.

The registry is a pure semantic validator only. Durable storage, operation
catalogs, restart discovery, and adapter delivery are not selected.

The exact admission subjects are:

| use | one exact admitted subject |
|---|---|
| `root_creation` | proposed root envelope candidate binding |
| `parent` | proposed child envelope candidate binding, never the reusable parent binding |
| `requested` | exact escalation or promotion request binding |
| `approving` | exact terminal disposition binding |
| `decision` | the exact decision binding cited by that disposition |
| `evidence` | each exact escalation evidence or promotion validation-evidence binding |
| `trigger_condition` | exact trigger binding from the selected candidate mapping |
| `constraint` | each exact envelope constraint binding |
| `source` | each promotion source-input binding and, separately, the exact source-envelope binding |
| `target` | typed semantic-memory target candidate binding |
| `target_memory` | exact promotion request binding at request admission; typed result-record binding for `applied`, otherwise exact request binding |

The transition registry enforces the table and exact cardinality/order. Root
and child envelope resolvers enforce their rows directly. Escalation request
admission takes `requested`, `trigger_condition`, and one `evidence` per record.
Escalation disposition admission takes `approving` and `decision`. Promotion
request admission takes `requested`, ordered `source` admissions for every
source input followed by the source envelope, `target`, and `target_memory`.
Promotion disposition admission takes `approving`, `decision`, one `evidence`
per validation record, and the outcome-specific `target_memory` subject.

No-candidate behavior is exact: successful resolution emits none; a proven
dimension increase or unprovable/expanding child allow emits exactly one of
the mapped candidates; explicit missing context/authority emits its mapped
candidate. Valid allow/deny overlap returns deny and no candidate. Malformed or
unknown input, missing mapping, stale bytes/currentness, ambiguous cardinality,
authenticator failure, replay mismatch, duplicate terminal disposition, and
indeterminate evaluation outside the one containment class refuse with no
candidate. Requests have no authority effect. Exact request/disposition replay
is idempotent; changed-ID replay and a second terminal disposition refuse.

## Compatibility selector

HCM-3.1 stable-role/vocabulary/profile bytes and behavior remain exact. The
shipped empty vocabulary and profile's selected stack closure replay unchanged.

L0-L3 pipeline input/filtering is not a public Context Resolution mapping and
is not edited. Existing positive/negative scoped-filtering tests must pass.
This deliberately preserves the useful precursor until HCM-3.5 selects its
real migration; HCM-3.2 neither removes it nor freezes it into kernel semantics.

## Exact public symbols and signatures

All fields are private. Argument names below are descriptive; the Rust types,
ownership, return types, generic bounds, and method names are exact. `Binding`
means `ContextResolutionExactBinding`, `Admission` means
`ContextResolutionAuthorityAdmission`, and `Error` means
`ContextResolutionKernelError`.

```rust
pub struct ContextResolutionExactBinding;
impl ContextResolutionExactBinding {
    pub fn new(reference: &str, fingerprint: &str) -> Result<Self, Error>;
    pub fn reference(&self) -> &str;
    pub fn fingerprint(&self) -> &str;
}

pub enum ContextResolutionAuthorityUse {
    RootCreation, Parent, Requested, Approving, Decision, Evidence,
    TriggerCondition, Constraint, Source, Target, TargetMemory,
}
impl ContextResolutionAuthorityUse { pub const fn as_str(self) -> &'static str; }

pub struct ContextResolutionAuthorityAdmission;
impl ContextResolutionAuthorityAdmission {
    pub fn authority_use(&self) -> ContextResolutionAuthorityUse;
    pub fn subject(&self) -> &Binding;
    pub fn binding_fingerprint(&self) -> &str;
}

pub struct ContextResolutionAuthorityAdmissionResolver;
impl ContextResolutionAuthorityAdmissionResolver {
    pub fn new(
        repo_root: impl AsRef<std::path::Path>, binding_kind_ref: &str,
        binding_instance_id: &str, expected_artifact_fingerprint: &str,
    ) -> Result<Self, Error>;
    pub fn admit<P: NativeAuthenticatorPortV1>(
        &mut self, authenticator: &mut P,
        authority_use: ContextResolutionAuthorityUse, subject: &Binding,
    ) -> Result<Admission, Error>;
}

pub struct ContextResolutionDimensions;
impl ContextResolutionDimensions {
    pub fn new(values: [&str; 6]) -> Result<Self, Error>;
    pub fn values(&self) -> [&str; 6];
}

pub enum ContextResolutionMutationEffect { Allow, Deny }
pub struct ContextResolutionMutationRule;
impl ContextResolutionMutationRule {
    pub fn new(
        rule_id: &str, effect: ContextResolutionMutationEffect,
        target_kind: &str, selector: &str,
    ) -> Result<Self, Error>;
}
pub enum ContextResolutionMutationDecision { Allowed, Denied, Indeterminate }
pub enum ContextResolutionMemoryDecision { Authorized, PromotionRequired }
pub enum ContextResolutionValidationDecision { Authorized, NotAuthorized }

pub struct ContextResolutionEnvelopeInput;
impl ContextResolutionEnvelopeInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        envelope_id: &str, objective_ref: &str, resolved_profile: Binding,
        resolution_stack: Binding, active_level_id: &str,
        dimensions: ContextResolutionDimensions, parent: Option<Binding>,
        constraint_inputs: Vec<Binding>,
        mutation_rules: Vec<ContextResolutionMutationRule>,
        escalation_triggers: Vec<Binding>,
    ) -> Result<Self, Error>;
    pub fn candidate_binding(&self) -> &Binding;
}

pub struct ContextResolutionEnvelope;
impl ContextResolutionEnvelope {
    pub fn resolve_root(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        input: ContextResolutionEnvelopeInput, creator: &Admission,
        constraints: &[Admission],
    ) -> Result<Self, Error>;
    pub fn resolve_child(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        parent: &Self, input: ContextResolutionEnvelopeInput,
        parent_authority: &Admission, constraints: &[Admission],
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
    pub fn dimensions(&self) -> &ContextResolutionDimensions;
    pub fn evaluate_mutation(
        &self, target_kind: &str, target: &str,
    ) -> ContextResolutionMutationDecision;
    pub fn authorize_memory(
        &self, requested_value: &str,
    ) -> Result<ContextResolutionMemoryDecision, Error>;
    pub fn authorize_validation(
        &self, requested_value: &str,
    ) -> Result<ContextResolutionValidationDecision, Error>;
    pub fn missing_context_candidate(
        &self, proposed: &ContextResolutionEnvelope, evidence: Vec<Binding>,
    ) -> Result<ContextResolutionEscalationCandidate, Error>;
    pub fn missing_authority_candidate(
        &self, proposed: &ContextResolutionEnvelope, evidence: Vec<Binding>,
    ) -> Result<ContextResolutionEscalationCandidate, Error>;
}

pub struct ContextResolutionEscalationCandidate;
impl ContextResolutionEscalationCandidate {
    pub fn class(&self) -> &str;
    pub fn trigger(&self) -> &Binding;
    pub fn missing_condition(&self) -> &str;
    pub fn requested_authority_ref(&self) -> &str;
    pub fn evidence(&self) -> &[Binding];
}

pub struct ContextResolutionEscalationRequest;
impl ContextResolutionEscalationRequest {
    pub fn new(
        request_id: &str, current_envelope: &ContextResolutionEnvelope,
        candidate: ContextResolutionEscalationCandidate,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}
pub enum ContextResolutionEscalationOutcome { Approved, Refused, Superseded }
pub struct ContextResolutionEscalationDisposition;
impl ContextResolutionEscalationDisposition {
    pub fn new(
        disposition_id: &str, request: Binding,
        outcome: ContextResolutionEscalationOutcome, decision: Binding,
        authorized_envelope: Option<Binding>,
        superseding_request: Option<Binding>,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}

pub struct ContextResolutionPromotionRequest;
pub struct ContextResolutionSemanticMemoryTarget;
impl ContextResolutionSemanticMemoryTarget {
    pub fn new(
        target_record_ref: &str, expected_target_fingerprint: Option<&str>,
    ) -> Result<Self, Error>;
    pub fn candidate_binding(&self) -> &Binding;
}
pub struct ContextResolutionSemanticMemoryRecord;
impl ContextResolutionSemanticMemoryRecord {
    pub fn new(record: Binding, target_memory_authority: &Admission) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}
impl ContextResolutionPromotionRequest {
    pub fn new(
        request_id: &str, source_inputs: Vec<Binding>,
        source_envelope: &ContextResolutionEnvelope,
        stack: &ContextResolutionStackDefinition,
        target_memory_horizon: &str,
        target: ContextResolutionSemanticMemoryTarget,
        target_authority: &Admission,
        requested_authority_ref: &str,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}
pub enum ContextResolutionPromotionOutcome { Applied, Refused, Stale }
pub struct ContextResolutionPromotionDisposition;
impl ContextResolutionPromotionDisposition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        disposition_id: &str, request: &ContextResolutionPromotionRequest,
        outcome: ContextResolutionPromotionOutcome, decision: Binding,
        validation_evidence: Vec<Binding>, approving_authority_ref: &str,
        observed_target_fingerprint: Option<&str>,
        result_record: Option<ContextResolutionSemanticMemoryRecord>,
    ) -> Result<Self, Error>;
    pub fn exact_binding(&self) -> &Binding;
}

pub struct ContextResolutionTransitionRegistry;
impl ContextResolutionTransitionRegistry {
    pub fn new() -> Self;
    pub fn admit_escalation_request(
        &mut self, request: ContextResolutionEscalationRequest,
        requested: &Admission, trigger: &Admission, evidence: &[Admission],
    ) -> Result<bool, Error>;
    pub fn admit_escalation_disposition(
        &mut self, disposition: ContextResolutionEscalationDisposition,
        approving: &Admission, decision: &Admission,
    ) -> Result<bool, Error>;
    pub fn admit_promotion_request(
        &mut self, request: ContextResolutionPromotionRequest,
        requested: &Admission, sources: &[Admission], target: &Admission,
        target_memory: &Admission,
    ) -> Result<bool, Error>;
    pub fn admit_promotion_disposition(
        &mut self, disposition: ContextResolutionPromotionDisposition,
        approving: &Admission, decision: &Admission,
        evidence: &[Admission], target_memory: &Admission,
    ) -> Result<bool, Error>;
}

pub enum ContextResolutionKernelErrorKind {
    InvalidInput, AuthorityRefused, StaleAuthority, Indeterminate,
    ExpansionRefused, Cardinality, ReplayMismatch, ForbiddenTarget,
}
pub struct ContextResolutionKernelError;
impl ContextResolutionKernelError {
    pub fn kind(&self) -> ContextResolutionKernelErrorKind;
    pub fn detail(&self) -> &str;
    pub fn candidate(self) -> Option<ContextResolutionEscalationCandidate>;
}
```

The only modified existing public type is
`ContextResolutionStackDefinition`; its existing methods keep their signatures.
Its representation may retain levels, domains, and policy pairs privately.
`ResolvedInstanceProfile` is consumed unchanged. No public fields, serde value
dispatcher, public trait, blanket implementation, generic framework, or symbol
outside this list is authorized.

The signatures in this section are the complete public callable ceiling and
remain unchanged by the JCS/recovery repair. Currentness is carried in private
fields of the opaque admission, resolver, envelope, candidate, request,
disposition, and registry values. Their existing constructors and methods
revalidate internally and may return the already-declared `StaleAuthority` or
`Indeterminate` result; no caller-selected proof mode or new public parameter
is admitted. New capsule codecs, fingerprint-domain values, publisher proofs,
historical/pending/current states, lineage witnesses, and quarantine records
are module-private or `pub(crate)` only where the three selected owner modules
must compose. Existing edited callables keep their current Rust signatures.

An escalation candidate is a sealed value created only by envelope resolution
or the two typed envelope candidate methods. It stores the exact current and
proposed envelope bindings plus complete profile, stack, dimension,
constraint, and mutation metadata. A failed child resolution returns no
envelope but includes this sealed candidate in the kernel error. Request
construction consumes that candidate and the exact current envelope, then
revalidates identical profile/stack, at least one changed bound, and the exact
candidate/current tuple. Cross-profile, cross-stack, unchanged, fabricated,
or mismatched proposals refuse before a request binding exists. No public
constructor can fabricate a candidate.
The existing error accessor consumes the error and returns its owned candidate,
so a failed-resolution candidate can move directly into request construction
without cloning, rebuilding, or exposing private fields.

A semantic-memory target is an unauthoritative candidate whose deterministic
binding is available after construction. Promotion-request construction then
requires a `Target` admission whose subject is that exact candidate binding.
A result record is constructible only with a `TargetMemory` admission whose
subject exactly matches the result binding. Registry admission rechecks both
authority uses and subjects. Because admissions are opaque resolver outputs,
arbitrary artifact, contract, posture, Snapshot, Projection, gate, or durable-
memory bindings cannot enter a request, disposition, or registry through
nominal wrappers.

## Exact ceilings

Production paths:

- `crates/engine/src/context_resolution.rs`;
- `crates/engine/src/context_resolution_registry.rs` (ceiling only; no edit is
  planned);
- `crates/engine/src/artifact_mutation.rs`;
- `crates/engine/src/artifact_lineage_store.rs`;
- `crates/engine/src/lib.rs` (ceiling only; no edit is planned).

Test/fixture paths:

- `crates/engine/tests/context_resolution_kernel.rs`;
- `crates/engine/tests/hcm_2_3_generic_lineage.rs`;
- `crates/engine/tests/fixtures/hcm_3_2_context_resolution/**`.

Control paths are limited to the exact HCM-3.2 SPEC/selector/plan/checklist,
future-work decision, proof and dispatch artifacts, earned 00-06 ledger rows,
one v1.4 handoff, ledger, and a `09` row only when validation requires it.

The five production paths are an absolute ceiling, not a requirement. The
planned code-bearing paths are `context_resolution.rs`, `artifact_mutation.rs`,
and `artifact_lineage_store.rs`. Maximums are 80 changed named production
declarations, 2,400 hand-written production lines (1,300 / 500 / 600 for those
three planned paths), 2,400 hand-written focused test/fixture lines, and 18
material test functions. A declaration is counted once by post-state `(module,
impl owner, name)` for each edited/new struct, enum, function, method, or
constant. Fields, variants, imports, derives, generated implementations, and
tests do not count. No Cargo, dependency, crate, unsafe, schema-catalog,
transport, native/platform, or ancillary production path is authorized.

Live pre-edit GitNexus classified these exact existing seams:

| existing symbol | risk | upstream / processes | selected edit |
|---|---:|---:|---|
| `validate_persisted_output_authority` | CRITICAL | 187 / 2 | require HCM gate and committed publisher proof |
| `validate_request_subject` | CRITICAL | 27 / 6 | retain exact HCM-only publisher authority object |
| `GenericArtifactLineageStoreV1::recover_pending` | CRITICAL | 52 / 5 | exact markerless retry or HCM quarantine |
| `GenericArtifactLineageStoreV1::verify_committed` | CRITICAL | 25 / 5 | distinguish current output from retained historical proof |
| `ArtifactMutationServiceV1::promote` | MEDIUM | 10 / available graph | hold evaluation lock/currentness through effect |
| `candidate_preview` | LOW | 15 / 1 | HCM gate before eligibility |
| `build_promotion_plan` | LOW | 11 / available graph | HCM publication authorization |
| `parse_promotion` | LOW | 12 / available graph | exact retained HCM request field |
| `PromotionDocumentV1` | LOW | available graph | private HCM-only field |
| `GenericArtifactLineageStoreV1::require_journal_authority` | LOW | 10 / 1 | exact HCM quarantine/refusal routing only if necessary |

The authority explicitly anticipated committed-journal verification, recovery,
marker handling, phase-aware validation, retry reconstruction, and these
HCM-specific seams up to CRITICAL. The four observed CRITICAL results were
warned before editing and are admitted only under this reviewed selector and
the full proportional wall. Context Resolution admission/resolver/envelope/
transition edits observed LOW risk. Any additional HIGH/CRITICAL symbol,
sixth production path, public API expansion, or changed owner is an authority
stop before edit. Every existing symbol receives a fresh upstream impact call
immediately before its edit.

## Lineage-store composition boundary

The generic store continues to own only its existing generic transaction
families; the committed registry owner continues to own registry transactions.
Unknown-family refusal, generic intent validation, registry currentness,
replay protection, owner fingerprints, and crash recovery remain intact.
Neither owner parses the other's schema and no transaction is deleted or
broadly allowlisted.

For HCM publication only, evaluation under the existing lineage lock may read
one exact committed registry transition and its authenticated retained witness.
The selected mutation path validates capsule semantics before candidate
eligibility and persistence as non-authoritative evidence. It validates the
publisher proof before promotion planning, authoritative canonical persistence,
immediately before effect, during pending/installed recovery, and before
current reads. The lock spans the final currentness check and effectful
promotion so the registry cannot advance between authorization and publication.

Committed verification has two private results: current exact canonical output
or historical exact displaced output with one unique committed direct
successor. Only the first can support a current witness. Markerless installed
recovery reconstructs an exact native/publication result only from durable
facts and never invents a live-handle claim. If reconstruction is impossible
or conflicts, one HCM-specific quarantine record captures bounded schema,
journal identity, output fingerprint, phase, and reason. It is not generic
authority, and it prevents only HCM authority consumption/publication until
explicit future repair. All other generic recovery remains available.

Required lineage proof includes O1 -> O2 -> O3 cold recovery; historical O1
verification after O2/O3 owns the canonical path; fault injection after every
registry, promotion, installation, native-marker, publication-marker, journal,
and rename boundary; exact retry and retry conflict; forged/missing/ambiguous
journal witness; cross-repository replay; rollback; unknown family; generic
non-HCM transaction validation; registry revocation/currentness loss; and
quarantine diagnostics. Installed-uncommitted output never reads as authority.

## Test selector

RED must fail because configurable stack application and envelopes do not yet
exist. GREEN must prove every SPEC proof obligation, including exact shipped
replay, non-shipped configurability, six independent dimensions, deterministic
fingerprints, child narrowing, deny overlap, indeterminate refusal, typed
memory/validation outcomes, no-effect escalation, transition cardinality, and
unchanged HCM-3.1/L0-L3 behavior. Authority GREEN must exercise the real empty
target -> intake -> candidate -> authenticated registry promotion -> generic
committed lineage -> current resolver path, never direct seeding. It covers all
eleven exact authority subjects and publisher proof.

Capsule negative proof covers duplicate keys, invalid UTF-8/I-JSON, BOM,
trailing bytes, alternate encodings, extra/missing members, mixed-null
predecessor quartet, every size/depth/member/string/escape limit, unknown tuple,
non-byte-identical JCS, all six cross-domain fingerprint substitutions, each of
the exact 53 pointer validations, wrong predecessor, and cross-repository/
profile/stack substitution. Generic structural success alone must remain
non-authoritative.

Currentness negative proof covers direct seed, ungated committed read, generic
`eligible_without_approval`, stale cache and clone, later head, credential
revocation, mapping loss, restored old bytes, rollback, forged/missing/
ambiguous witness, failed promotion, and O1 historical proof structurally
unable to reach every operational API. Positive/negative replacement proof
covers O1 -> O2 -> O3 cold recovery, exact retry conflict, crash injection at
every registry/promotion/publication/journal boundary, markerless installed
recovery, quarantine, and unchanged non-HCM recovery.

Promotion GREEN also uses an actual resolved source envelope and stack to prove
strict target-memory rank, ordered source-envelope admissions, typed semantic-
memory target/result, compare-and-write applied/refused/stale behavior, and
refusal of artifact/contract/posture or durable-memory claims.

No test may claim Projection, Snapshot, flow/pipeline adoption, posture, SDK,
transport, release, publication, or downstream proof.

Escalation negative proof covers cross-profile, cross-stack, unchanged-bound,
fabricated-binding, and candidate/request tuple mismatch for each candidate
class. Semantic-memory proof covers one positively admitted target/result and
construction/registry refusal for artifact, contract, posture, Snapshot,
Projection, gate, and durable-memory subjects.

## Review and proof selector

The fresh parent has no inherited CLEAN stage. This complete selector must
receive an ordinary fresh planning discovery review and, if findings exist,
one consolidated remediation plus different-fresh closure before RED/Rust.
Later implementation review
uses the complete converged subject and exact proof wall. P1/P2 findings receive
one consolidated remediation plus different-fresh closure; only directly
causal unmasked findings may consume the two supplemental cycles. Proof and
final-closeout stages use fresh agents, and no cycle follows CLEAN.

Before the primary commit: focused/full tests, strict Clippy/format/diff,
validator/self-tests, exact definition replay, path ceiling, GitNexus scoped
and compare-to-main detection, remote baseline, and protected-root
non-access/non-mutation all have honest results. An unavailable FTS or
comparison is unavailable, not GREEN.

## Non-goals

- HCM-3.3 Projection or any Resolution-aware view;
- HCM-3.4 Snapshot Memory/delta;
- HCM-3.5 packet/flow/pipeline adoption or L0-L3 migration;
- HCM-3.6 posture;
- SDK, CLI/Tauri/Substrate/HTTP/dock/transport work;
- new dependency, crate, module framework, unsafe/native/platform machinery;
- generic atomic-document/capsule support, a second artifact family or
  consumer, per-inner-field provenance/query/update support;
- staged H2 installation authorization followed by H3 operational activation;
- shipped-definition/profile/vocabulary mutation;
- release, publication, push, or remote reconciliation;
- unrelated cleanup or immutable-history repair.

The H2/H3 continuous-availability design is future work and non-authority.
It would remove the deliberately accepted no-authority interval, but requires
additional registry/state-machine semantics and crash proof. Its sole trigger
is a future explicit product requirement or SLO for uninterrupted Context
Resolution across replacement and every supported crash boundary. This slice
documents it in `decision/2026-08-04-deferred-continuous-availability.md` and
does not implement it.

## Stop conditions

Stop for any required scope beyond the ceilings, new public/dependency/schema/
unsafe/transport authority, HCM-3.3+ behavior, changed shipped identity,
any additional HIGH/CRITICAL symbol, contract contradiction not repairable inside
HCM-3.2, mandatory delegation failure, exhausted causal budget, or unresolved
P1/P2. Local findings, proof gaps, and bounded remediation remain parent-owned.

The JCS capsule, 53-pointer validation, fingerprint domains, external publisher
proof, private states, exact currentness, minimum replacement floor, kernel
semantics, exact public signatures, five-path ceiling, and named risks are now
selected. The fresh slice remains stopped until this complete selector review
is CLEAN. Validator refusal, unsafe bootstrap-only reduction, inability to fit
the recovery floor, P1/P2, or any public/path/dependency/risk expansion stops
fail closed; identities may not be renamed or reset.
