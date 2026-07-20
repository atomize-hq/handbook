# HCM-2.2 Specification: Constitutional-Root Canonical Charter

## Status and authority

`HCM-2.2` is reopened at escalation `HCM-2.2-ESC-001`. The prior planning
selection and the implementation present at checkpoint commits
`684029d4dda0c800fe840a19b54bf7f87fb41123` and
`db503f7a96479775fe25dbb864b9379fd3e61ac8` are non-authoritative checkpoint
evidence: fresh implementation Review 2 found a lifecycle-validation identity
cycle and three additional durability/API findings. They do not establish a
completed slice or close any Charter proof gate. The later implementation
selected from the review-clean Option 1 handoff at entry HEAD
`4e164061e18da17cc24d576a26800ab1ecce69c2` stopped when the `W5`-`W7`
exact-prefix grammar proved unrecoverable after process memory loss.

User-selected escalation `HCM-2.2-ESC-002` chooses atomic whole-file scratch
publication for the three new-output stages. This document is now the mutable
documentation-only authority-repair subject. Intent `1.2` remains byte-for-byte
unchanged. No Rust or implementation work may resume until a fresh isolated
reviewer returns `CLEAN` over the complete repaired persistence/recovery
subject and a new parent handoff records that reviewed state. The retained dirty
implementation and its verified recoverable snapshot are checkpoint evidence
only. No `HCM-2.3` authority exists.

The packet consumes, without reopening, the reviewed HCM-1 registry/profile
boundaries and the HCM-2.1 Project Context canonical-YAML pilot. Entry evidence
is:

- branch `feat/handbook-contract-membrane`;
- planning entry HEAD `e6513bfb1d2c1f21eddeba5f1eda7f3114769f36`;
- HCM-2.1 reviewed implementation commit
  `d61670eb2906c6725d0c268c8f63245297808b6f`;
- selected HCM-2.1 closeout
  `20260719T104915Z--HCM-2-1--orchestration--project-context-canonical-yaml-landed`;
- exact shipped profile `handbook.profile.shipped-root@1.0.0`;
- exact `project_authority` descriptor path
  `.handbook/project/charter.yaml`, role `constitutional_authority`, required
  capability `constitutional_root`, and `always` requiredness; and
- immutable released Project Authority schema/kind/profile `1.0.0` bytes.

The selected historical handoffs remain dependency and transition evidence.
They do not override this reopened status, select resumed implementation,
widen the slice, or authorize HCM-2.3. Live source, tests, definition bytes,
and Git history at the entry HEAD override stale narrative wording.

## HCM-2.2-ESC-001 Option 1 authority repair

This section is additive and controlling for every later repaired HCM-2.2
candidate, lifecycle-validation result, promotion transaction, recovery path,
and fault-injection surface. Where the older text below describes candidate
`1.1`, a validation result that binds the final candidate fingerprint, a
promotion journal without the fields or terminal states below, destructive
mismatch recovery, or production-visible fault hooks, that text describes only
the non-authoritative checkpoint. Released records and the reviewed `1.1`
fingerprint semantics are not changed.

The normative machine-readable planning contracts are:

- [`contracts/lifecycle-validation-result-1.0.0.schema.json`](contracts/lifecycle-validation-result-1.0.0.schema.json), the closed Draft 2020-12 result schema;
- [`contracts/authority-repair-runtime-vectors-v1.0.json`](contracts/authority-repair-runtime-vectors-v1.0.json), the exact create/amend identity vectors, definition order, cross-record matrix, and negative matrix;
- [`contracts/promotion-transaction-intent-1.2.0.schema.json`](contracts/promotion-transaction-intent-1.2.0.schema.json), the closed Draft 2020-12 promotion-intent schema; and
- [`contracts/promotion-transaction-intent-vectors-v1.0.json`](contracts/promotion-transaction-intent-vectors-v1.0.json), the exact amendment intent/fingerprint/document/marker vector, owned-name grammar, terminal sets, bindings, and negative matrix.

### Additive candidate 1.2 and acyclic identity

The repaired selected path accepts only `handbook.artifact-candidate` version
`1.2`. Its complete candidate semantic subject is the RFC 8785 JCS object with
exactly these fields and no others:

1. `schema_id`;
2. `schema_version`;
3. `intake_record_ref`;
4. `target_kind_ref`;
5. `target_instance_id`;
6. `target_schema_ref`;
7. `profile_ref`;
8. `resolved_profile_fingerprint`;
9. `normalized_content_ref`;
10. `field_sources` in candidate order;
11. `unresolved_coverage_ids` in intake-definition order;
12. `promotion_eligibility`;
13. `required_approval_policy_ref`; and
14. `basis_artifact_fingerprint`.

The `candidate_subject_fingerprint` preimage is that complete object. It
excludes **exactly** `candidate_id`, `candidate_fingerprint`,
`candidate_subject_fingerprint`, and `validation_result_refs`; no timestamp,
implicit engine state, or omitted candidate semantic field exists. The value is
`sha256:` plus lowercase SHA-256 of its RFC 8785 JCS UTF-8 bytes. It is
validation-binding identity only. It is not candidate identity, canonical
authority, an approval target, or a promotion decision.

After lifecycle validation, the final candidate-fingerprint preimage is the
same fourteen subject fields plus `candidate_subject_fingerprint` and exactly
one `validation_result_refs` member. It excludes exactly `candidate_id` and
`candidate_fingerprint`. `candidate_fingerprint` is `sha256:` plus lowercase
SHA-256 of the RFC 8785 JCS UTF-8 preimage, and `candidate_id` is
`candidate_<fingerprint-hex>`. The construction order is therefore strictly:

```text
complete candidate semantic subject
  -> candidate_subject_fingerprint
  -> lifecycle-validation result and content-addressed ref
  -> final candidate_fingerprint and candidate_id
  -> human approval over the final candidate fingerprint
  -> promotion
```

No edge points from the validation result back to the final candidate identity.
If any implementation requires that edge, the graph is cyclic and work stops.

Candidate versions `1.0` and `1.1`, their approvals, and their content-addressed
records remain immutable historical/checkpoint evidence. They are not admitted
by the repaired selected product path. A candidate must be re-authored and
re-evaluated as `1.2`; it cannot be upgraded by copying an old result ref.
Approvals must be newly obtained because every approval binds the new final
candidate fingerprint. There is no implicit `1.1`/`1.2` dual read, fallback,
automatic migration, checkpoint rewrite, or approval carry-forward.

### Engine-owned lifecycle-validation result 1.0

Only a dedicated engine lifecycle-validation service may construct
`handbook.lifecycle-validation-result` version `1.0`. A caller supplies the
candidate subject inputs, never a validation result, validation result ref,
validation status, definition set, lifecycle binding, currentness observation,
or validation timestamp. The service resolves and validates all authority,
constructs the closed record, persists it, and returns its ref. Prefer a
dedicated result validator/store rather than widening generic lineage
`validate_record`.

The validation-result fingerprint preimage contains exactly the schema ID and
version, candidate subject fingerprint, intake ref and fingerprint, normalized
content ref and fingerprint, target instance and canonical ref, create-only or
current canonical basis, separately observed current canonical fingerprint,
profile ref and resolved fingerprint, complete ordered resolved-definition
bindings, lifecycle policy ref and fingerprint, lifecycle head ref and
fingerprint, lifecycle state and state fingerprint, complete ordered active
observations, complete ordered reopened-coverage IDs, and literal status
`passed`. It excludes exactly `validation_result_id`,
`validation_result_fingerprint`, and `validated_at_utc`. The ID is
`lifecycle-validation-result_<fingerprint-hex>`.

`validated_at_utc` is engine-authored audit metadata only and is excluded from
identity. Before allocating a timestamp, the engine computes the preimage,
fingerprint, ID, and final ref and checks for an existing record. An existing
exact schema-valid record whose recomputed preimage, fingerprint, ID, and bytes
match is returned with its original timestamp. An existing ID with any byte,
timestamp, preimage, or identity difference refuses; replay never fabricates a
second timestamp for the same ID.

The only result-ref grammar is
`lifecycle-validation-results/lifecycle-validation-result_<64 lowercase
hex>.json`; it resolves beneath
`.handbook/state/lifecycle-validation-results/` to the same basename. The
persisted bytes are RFC 8785 JCS of the complete record followed by one LF and
may not exceed 262,144 bytes including that LF. Every directory component and
file is opened no-follow and must be the expected directory or regular-file
type on the same filesystem. For a new ID, the engine writes and fsyncs one
create-new owned temporary file, installs it with atomic rename-no-replace, and
fsyncs the store directory. A concurrent/pre-existing destination is success
only after a bounded no-follow reload proves exact byte equality; otherwise it
refuses and retains the existing file. The engine never follows a link,
overwrites, truncates, appends to, or deletes a result. Failed owned temporaries
may be removed only after the final destination has been classified.

The ordered resolved-definition list has cardinality thirteen and is exactly
the vector order: approval policy, capability contract, intake definition,
lifecycle policy, production-posture reassessment trigger, trust-boundary
reassessment trigger, renderer, amendment review trigger, semantic validator
`1.0.0`, semantic validator `1.1.0`, target kind, target schema, and waiver
policy. Active observations are the complete current set, unique by ref and
fingerprint and ordered exactly as `active_observation_refs` in the current
committed lifecycle-transition head. That retained head order is the lifecycle
writer's event-precedence then event-fingerprint order; it is not re-sorted by
observation ref during validation. Reopened coverage is the complete unique set selected by all active observations, ordered by the
sixteen-item intake-definition coverage order. Empty arrays are authoritative
empty sets, not unknown or omitted data. The candidate has exactly one result
ref; zero, two, duplicate, excess, or reordered refs refuse.

The engine proves these cross-record equalities before it persists the result:

- the recomputed candidate subject fingerprint equals the supplied subject
  binding;
- the intake ref equals the candidate ref and the loaded intake ID,
  fingerprint, target, profile, basis, and exact bytes agree;
- the normalized-content ref equals the candidate ref and its basename,
  loaded bytes, content fingerprint, target schema, and semantic validation
  agree;
- the canonical basis is literal null only when the selected target is absent;
  otherwise candidate basis, observed current fingerprint, and current
  no-follow target bytes/fingerprint are all non-null and equal;
- profile ref/fingerprint and all thirteen definition ref/fingerprint pairs
  equal the currently selected exact producers;
- lifecycle policy, current head, state, state fingerprint, complete active
  observation set, and complete reopened-coverage set equal the recovered
  lifecycle authority under its lock; and
- every reopened coverage ID is present in the new intake evaluation and is
  satisfied or covered by its exact allowed waiver semantics.

### Promotion-time currentness and refusal boundary

Promotion acquires promotion, registry, and lifecycle locks in that order,
recovers each domain, and then—before creating a journal, allocating a
transaction-owned file, invoking an authenticator, or mutating any path—reloads
the candidate and its sole result and repeats every result-schema, fingerprint,
ID/ref/basename, subject, intake/content lineage, canonical basis, profile,
definition, lifecycle-policy, head/state, active-observation, reopened-coverage,
and approval equality. It recomputes the current complete ordered observation
and coverage sets rather than checking membership. Approvals must bind the
exact final `1.2` candidate fingerprint and remain current under the retained
registry pair.

Missing, caller-authored, forged, stale, duplicate, reordered, incomplete, or
excess validation authority refuses before mutation. A schema-valid result is
not sufficient if any current producer or authority byte has drifted. The same
closed checks run again before a recovery roll-forward; inability to prove
them preserves the journal and blocks selected reads/writes rather than
guessing.

### Promotion transaction intent 1.2 and recovery

The repaired writer uses the linked closed schema for
`handbook.charter-promotion-transaction-intent` version `1.2`. The exact
top-level keys are `schema_id`, `schema_version`, `transaction_id`,
`promotion_id`, `mutation_mode`, `target`, `candidate_lineage`,
`selected_contract`, `human_authority`, `outputs`, `recovery`, and
`intent_fingerprint`. The closed nested keys are:

| Object | Exact keys |
|---|---|
| `target` | `target_instance_id`, `canonical_artifact_ref`, `basis_artifact_fingerprint`, `observed_current_artifact_fingerprint` |
| `candidate_lineage` | `candidate_ref`, `candidate_fingerprint`, `candidate_subject_fingerprint`, `intake_record_ref`, `intake_record_fingerprint`, `normalized_content_ref`, `normalized_content_fingerprint`, `validation_result_ref`, `validation_result_fingerprint` |
| `selected_contract` | `profile_ref`, `resolved_profile_fingerprint`, complete ordered `resolved_definitions`, `lifecycle_policy_ref`, `lifecycle_policy_fingerprint`, `prior_lifecycle_head_ref`, `prior_lifecycle_head_fingerprint`, `prior_lifecycle_state`, `prior_lifecycle_state_fingerprint`, complete ordered `active_observations`, complete ordered `reopened_coverage_ids` |
| each definition | `definition_role`, `definition_ref`, `definition_fingerprint` |
| each observation | `observation_ref`, `observation_fingerprint` |
| `human_authority` | complete ordered `approval_bindings`, `approver_registry_state_ref`, `approver_registry_state_fingerprint`, `registry_head_transition_ref`, `registry_head_transition_fingerprint` |
| each approval | `approval_ref`, `approval_fingerprint`, `approval_class`, `authority_ref` |
| `outputs` | `new_canonical_fingerprint`, `new_canonical_document_sha256`, `new_canonical_byte_length`, `promotion_record`, `lifecycle_transition` |
| each output record | `record_ref`, `record_fingerprint`, `document_sha256`, `byte_length` |
| `recovery` | `old_canonical_status`, `old_canonical_fingerprint`, `old_canonical_document_sha256`, `old_canonical_byte_length` |

Create requires null target basis/current, absent old canonical with three null
old fields, absent lifecycle head/state fingerprint, and empty observation and
reopened sets. Amendment requires non-null equal target basis/current, present
old canonical with non-null fingerprint/document hash/length, and a non-absent
head/state closure. Every intent binding equals the retained candidate result,
approval/registry observation, exact output record, and raw output bytes.
`promotion_id` equals the promotion-record ref basename identity and is derived
from its record fingerprint. Output semantic fingerprints and raw document
SHA-256 values are distinct checks; both plus byte length must match.

The `intent_fingerprint` preimage is the complete closed intent object excluding
**exactly** `intent_fingerprint`; it includes the engine-allocated
`transaction_id` and contains no timestamp. Its value is RFC 8785
JCS/SHA-256. `intent.json` is exactly RFC 8785 JCS of the complete record plus
one LF, bounded to 262,144 bytes. The pending path is exactly
`.handbook/state/transactions/promotions/<transaction_id>.pending/`. The
published vector independently freezes the amendment preimage, intent
fingerprint, complete-document byte length/SHA-256, pending path, and marker
payload.

The only owned pending names are the vector's fifteen names: `intent.json`,
`canonical.old`, `canonical.new`, `promotion-record.new`,
`lifecycle-transition.new`, and the `.tmp`/published pairs for `prepared`,
`canonical-installed`, `records-installed`, `committed`, and `rolled-back`.
Unknown names refuse and preserve the directory. Each marker is published by
create-new `<marker>.tmp`, exact-prefix write, fsync, rename-no-replace to the
unsuffixed name, and directory fsync. Its complete payload is exactly 72 ASCII
bytes `sha256:<64 lowercase hex>\n`, hashing the exact persisted
`intent.json` bytes including LF. A published marker forbids its `.tmp`; a
partial `.tmp` is permitted only as an exact prefix while the published marker
is absent. Published forward markers form the prefix `prepared` ->
`canonical-installed` -> `records-installed` -> `committed`; a gap or optimistic
marker is mismatch. `rolled-back` never coexists with a forward marker.
Because recovery cannot observe whether a pending-directory fsync completed
after a terminal-marker rename, every exact published `committed` or
`rolled-back` observed under `.pending` first repeats the pending-directory
fsync and then exact-revalidates the complete terminal payload before any
terminal directory rename. A later transaction-parent fsync cannot substitute
for this child-directory durability replay.

`intent.json` is never written partially inside `.pending`. Before that
directory exists, the writer create-news a scratch file beneath the
non-authoritative sibling `.intent-staging/` directory using an engine-random
128-bit lowercase-hex name unrelated to semantic IDs; writes the complete
self-fingerprinted JCS+LF intent; fsyncs, closes, boundedly reopens no-follow,
and verifies schema, fingerprint, document SHA-256, byte length, and exact
bytes, then fsyncs `.intent-staging/`. A crash may orphan that scratch file, but scratch is outside the journal
scan and can never authorize cleanup, recovery, or a selected read. Optional
scratch garbage collection is separately authorized and out of scope.

After scratch verification, the writer create-news and fsyncs the empty
`.pending` directory, then atomically rename-no-replace moves the complete
scratch file directly to `intent.json` on the same filesystem and fsyncs the
pending directory, `.intent-staging/`, and transaction parent. A crash exposes either an empty
pending directory or a complete self-verifying `intent.json`; `intent.tmp`, a
partial `intent.json`, and any other pending name are mismatch. This removes the
need to authenticate lost in-process intent bytes during recovery.

The three new outputs use a second non-authoritative sibling scratch root,
exactly `.handbook/state/transactions/promotions/.output-staging/`, on the same
filesystem as `.pending`. The engine allocates one independent random 128-bit
lowercase-hex token for each output and create-news exactly one of
`<32hex>.canonical`, `<32hex>.promotion-record`, or
`<32hex>.lifecycle-transition`. Tokens and scratch names are absent from intent
identity and unrelated to semantic IDs. A create-new collision refuses that
writer attempt without replacing either file; the already-published journal is
then handled only by ordinary recovery.

For each output in canonical, promotion-record, lifecycle-transition order, the
writer writes only to its scratch file, fsyncs and closes it, boundedly reopens
it no-follow, and proves byte-for-byte equality to the retained engine bytes.
It also proves the intent-bound document SHA-256 and byte length; canonical
fingerprint for `canonical`; and closed record schema, semantic fingerprint,
content-addressed ref, and complete intent/currentness bindings for both record
outputs. It then fsyncs `.output-staging/`. Any failed check refuses without
publishing or mutating the named pending stage.

The exact per-output scratch/publication boundaries are:

| Boundary | Required state transition |
|---|---|
| `S0` | engine retains the complete expected bytes and all intent/type-specific bindings |
| `S1` | create-new the independent random purpose-typed scratch file |
| `S2` | write only scratch; bounded fault fixtures cover every zero-through-complete prefix |
| `S3` | fsync and close the complete scratch file |
| `S4` | bounded/no-follow reopen scratch |
| `S5` | verify exact bytes, document hash/length, and all type-specific bindings |
| `S6` | fsync `.output-staging/` |
| `S7` | atomic rename-no-replace scratch to the exact pending stage |
| `S8` | fsync the pending directory |
| `S9` | fsync `.output-staging/` after the rename |
| `S10` | fsync the promotion-transaction parent |
| `S11` | bounded/no-follow reopen and exact-reverify the pending stage |

No boundary may be collapsed, reordered, or treated as implied by a later
fsync. The writer starts the next output only after the current output reaches
`S11`.

Only a closed, complete, verified scratch file may be atomically
rename-no-replace moved to its exact pending name: `canonical.new`,
`promotion-record.new`, or `lifecycle-transition.new`. The destination must be
absent and the move must remain on the same filesystem. After the rename, the
writer fsyncs the pending directory, `.output-staging/`, and the transaction
parent in that order, then boundedly reopens the pending stage no-follow and
reverifies its exact bytes/hash/length and type-specific bindings. A crash
before the rename exposes the pending stage as absent; a crash at or after the
rename exposes it only as absent or exact complete bytes. A partial, excess,
wrong, unsafe, or substituted pending new-output stage is always mismatch even
when its bytes happen to be a prefix of the intended document.

Partial output writes exist only in `.output-staging/`. Recovery and selected
readers never scan, classify, compare, delete, or infer authority from either
scratch sibling. A scratch orphan cannot authorize journal cleanup or
roll-forward, cannot make an absent pending stage present, and never adds a
recovery axis. Optional scratch garbage collection remains separately
authorized and out of scope. `canonical.old` is deliberately different:
create forbids it, while amendment may retain an absent/exact-prefix/exact
snapshot because the retained exact old canonical target is its authentic
completion source. Amendment requires `canonical.old` exact before any
new-output scratch publication begins.

This repair changes only persistence and recovery grammar. The closed intent
`1.2` schema remains exactly 16,019 bytes with SHA-256
`c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`;
the published amendment intent fingerprint, 7,607-byte document hash, and
72-byte marker remain unchanged. Any proposed intent field or additive version
requires a separate planning finding and fresh review; it is not inferred by
this repair.

The writer's ordered transitions are exact and cumulative:

| Boundary | Canonical target | Pending state after directory fsync | Final record state | Published forward markers | Recovery class if the process stops |
|---|---|---|---|---|---|
| `W0 admitted` | exact old or absent | directory absent | each output final absent or pre-existing exact-equal | none | no transaction exists |
| `W1 intent scratch` | old/absent | `.pending` absent; complete verified scratch exists only outside the journal namespace | unchanged | none | no transaction exists; scratch is a non-authoritative orphan |
| `W2 directory` | old/absent | empty `.pending`; complete verified scratch still outside | unchanged | none | pre-intent cleanup of empty pending only; scratch remains outside authority |
| `W3 intent published` | old/absent | exact complete `intent.json`; no pending intent temp exists | unchanged | none | rollback terminalization |
| `W4 old snapshot` | old for amend / absent for create | amend: `canonical.old` exact-prefix then exact+fsynced; create: forbidden | unchanged | none | rollback terminalization |
| `W5 canonical staged` | old/absent | while scratch is written/verified, `canonical.new` is absent; atomic rename then all three directory fsyncs and stage reverify make it exact | unchanged | none | absent stage selects the preceding rollback origin; exact stage selects W5 rollback terminalization; scratch is ignored |
| `W6 promotion staged` | old/absent | while scratch is written/verified, `promotion-record.new` is absent; atomic rename then all three directory fsyncs and stage reverify make it exact | unchanged | none | absent stage selects the preceding rollback origin; exact stage selects W6 rollback terminalization; scratch is ignored |
| `W7 lifecycle staged` | old/absent | while scratch is written/verified, `lifecycle-transition.new` is absent; atomic rename then all three directory fsyncs and stage reverify make it exact; all required pending stages are exact | unchanged | none | absent stage selects the preceding rollback origin; exact stage selects W7 rollback terminalization; scratch is ignored |
| `W8 prepared` | old/absent | exact staged set; marker temp may be exact-prefix, then `prepared` published | unchanged | `prepared` | rollback terminalization |
| `W9 canonical installed` | rename may be observed exact old/absent or exact new; after target-parent fsync it is new | `canonical.new` absent when rename is observed new; both record stages exact | unchanged | `prepared` | old/absent rolls back; new rolls forward |
| `W10 canonical marked` | new | marker temp may be exact-prefix, then marker published | unchanged | through `canonical-installed` | roll forward |
| `W11 promotion installed` | new | promotion stage is absent after rename, or may coexist exact only while a pre-existing exact final is being classified/deleted; lifecycle stage exact | promotion final exact | through `canonical-installed` | roll forward |
| `W12 lifecycle installed` | new | each record stage absent, except an exact redundant stage may remain only beside a pre-existing exact final before owned-stage deletion | both finals exact | through `canonical-installed` | roll forward |
| `W13 records marked` | new | no record stage remains; marker temp may be exact-prefix, then marker published | both finals exact | through `records-installed` | roll forward |
| `W14 committed` | new | no staged output; commit temp may be exact-prefix, then `committed` published; fsync the pending directory and exact-revalidate the complete pending terminal payload while suffix remains `.pending`; recovery repeats both steps before terminal rename | both finals exact | all four | execute exact committed terminalization only after the replayed pending-directory fsync and payload revalidation |
| `W15 terminal` | new at commit; later canonical successors are allowed | rename-no-replace `<id>.pending` to `<id>.committed`, fsync the transaction parent, and reverify the exact create/amend terminal name set from the vector | both finals exact | all four | every exact `.committed` observation repeats the transaction-parent fsync, then verifies the immutable committed link; require target-new equality only if this is the selected current promotion head |

At `W5`-`W7`, every partial file and every complete-but-not-yet-published file
is outside the journal in `.output-staging/`; the named pending stage remains
absent. Atomic rename may be observed only as absent or exact complete stage.
At every marker boundary, a crash may still leave only the next `.tmp` as an
exact prefix because its complete expected payload is recoverable from
`intent.json`. No partial pending new-output stage is writer-reachable. A
pre-existing exact final is never owned; rename-no-replace failure triggers
bounded no-follow exact equality, then deletion of only the redundant owned
stage. Different final bytes refuse.

Rollback recovery is itself an ordered, cumulative, crash-recoverable writer;
it is not an indivisible action hidden behind a recovery-table row. From an
admitted `W3`-`W9` state whose target is still exact old/absent, it executes
these boundaries while holding the same locks:

| Boundary | Exact action and durable postcondition |
|---|---|
| `R0 admitted` | Revalidate one exact `W3`-`W9` rollback origin, including intent, target, allowed final orphans, absent-or-exact new-output stages, the independently allowed `canonical.old`/marker prefixes, types, paths, and marker causality. Scratch is not inspected and no byte is changed. |
| `R1 snapshot complete` | For amendment, create/complete `canonical.old` only from the retained exact old target, fsync it, and reverify exact fingerprint/document hash/length. Create continues to forbid `canonical.old`. |
| `R2 prepared temp removed` | If `prepared.tmp` exists as the allowed exact prefix, no-follow reverify and unlink it, then fsync the pending directory; otherwise prove it absent. |
| `R3 prepared removed` | If `prepared` exists with the exact marker payload, no-follow reverify and unlink it, then fsync the pending directory; otherwise prove it absent. |
| `R4 lifecycle stage removed` | If `lifecycle-transition.new` exists, no-follow reverify its exact complete intent-bound bytes/hash/length/schema/ref/fingerprint/bindings, unlink it, then fsync the pending directory; otherwise prove it absent. Any partial or mismatch is preserved and refuses. |
| `R5 promotion stage removed` | Apply the same exact-complete-or-absent rule to `promotion-record.new`, then fsync the pending directory. |
| `R6 canonical stage removed` | Apply the same exact-complete-or-absent rule to `canonical.new`, then fsync the pending directory. The directory now contains exactly `intent.json` plus exact `canonical.old` for amend, and no other name. |
| `R7 rollback marker temp` | Create-new `rolled-back.tmp`, write the exact 72-byte marker; any crash may leave an exact prefix from zero through 72 bytes. Append only the missing suffix, fsync, and reverify exact bytes. |
| `R8 rollback marker published` | Rename-no-replace `rolled-back.tmp` to `rolled-back`, fsync the pending directory, and reverify the exact create/amend rollback terminal payload. Every recovery observation of published `rolled-back` under `.pending` repeats that pending-directory fsync and payload revalidation before `R9`. |
| `R9 rollback terminal` | Rename-no-replace `<id>.pending` to `<id>.rolled-back`, fsync the transaction parent, reverify the exact terminal name set, and return non-authority. Every observation of an exact `.rolled-back` terminal repeats the parent-directory fsync before return because a crash cannot reveal whether the prior fsync completed. |

The rollback cleanup list is exactly `prepared.tmp`, `prepared`,
`lifecycle-transition.new`, `promotion-record.new`, `canonical.new`; no other
name may be removed. Each unlink is its own reverify/unlink/directory-fsync
boundary. The admitted rollback-progress family is the union obtained by
starting from every exact `W3`-`W9` rollback origin, completing `R1`, and
applying any prefix of that five-name cleanup list; an already-absent listed
name is an idempotent no-op. This construction preserves forward-marker
causality because `prepared.tmp` and `prepared` are removed before any staged
file. A crash before an unlink is durable exposes the preceding member; a crash
after it exposes the following member. States outside that constructed union
are not inferred as recovery work and remain evidence-preserving mismatch.

Recovery first validates the complete state grammar above. For each output,
“accounted” means an exact stage with final absent, an exact final with stage
absent, or an exact pre-existing final plus redundant exact stage awaiting
owned-stage deletion. It then applies this exhaustive partition in order:

| Predicate | Required action |
|---|---|
| transaction directory suffix is absent | no transaction exists; ignore every non-authoritative `.intent-staging/` or `.output-staging/` scratch orphan and perform no journal mutation |
| suffix is exactly `.pending`; no `intent.json`; target old/absent; directory is empty | remove only the empty pending directory; no partial intent is an admitted pending state |
| suffix is exactly `.pending`; valid intent; target old/absent; neither rollback marker name nor `committed` exists; final observations are allowed by the exact origin; and the pending names/bytes belong to the constructed `R0`-`R6` rollback-progress family | resume at the first incomplete `R1`-`R6` boundary, retaining exact final orphans and removing only the fixed cleanup-list names; then begin exact rollback-marker publication |
| suffix is exactly `.pending`; valid intent; target old/absent; exact `R6` terminal payload plus `rolled-back.tmp` as an exact marker prefix; `rolled-back` and every forward marker/stage are absent | append only the missing marker suffix, fsync/reverify, execute `R8`, then execute `R9` |
| suffix is exactly `.pending`; valid intent; target old/absent; exact `R8` rollback terminal payload with published matching `rolled-back`, no marker temp, forward marker, or stage | repeat the pending-directory fsync, exact-revalidate the rollback terminal payload, then execute `R9`; a destination collision, unsafe destination, or any byte/name mismatch preserves the pending evidence and refuses |
| suffix is exactly `.pending`; valid intent; target new; no `committed`; both outputs accounted; marker causality valid; and the bound pre-promotion authority is still the authoritative invisible prior state | normalize redundant exact stages, install any staged output create-new-or-equal, publish any missing causal markers in order, publish `committed`, and terminalize `.committed` |
| suffix is exactly `.pending`; valid intent; target new; matching `committed`; both finals exact; no staged output; and the intended lifecycle transition is the exact committed successor of the bound prior state | repeat the pending-directory fsync, exact-revalidate the complete pending terminal payload, rename-no-replace to `<id>.committed`, fsync the transaction parent, reverify the exact terminal set, and return committed authority; any pre-existing destination, unsafe destination, simultaneous suffix, or byte/name mismatch preserves the complete pending journal and refuses without mutation. Do not demand that the prior state remain current after its intended transition commits |
| suffix is exactly `.committed`; terminal directory has exactly its create/amend vector name set and internally matching intent/finals/transition edge | fsync the transaction parent, then return immutable committed history without content mutation; if it is the selected current promotion head require current target equals its new canonical bytes, otherwise require a later committed successor chain whose basis binds this output |
| suffix is exactly `.rolled-back`; terminal directory has exactly its create/amend vector name set, no forward marker, and matching retained intent/rollback payload | fsync the transaction parent, then return non-authority without content mutation; do not compare it to a target that a later committed transaction may legitimately change |
| every other Cartesian-product state | mismatch: preserve the complete pending/terminal journal and every final record; perform no mutation and block selected reads/writes |

Before `committed`, “currentness” means the bound pre-promotion profile,
definitions, registry pair, lifecycle prior head/state/observations, and target
basis remain authoritative because the new transition is physically possible
but invisible. At and after `committed`, the intended lifecycle transition is
the exact successor and the prior head must **not** be required to remain
current. Later lifecycle events and promotions may advance the chain;
historical committed/rolled-back journals remain valid evidence through
successor/basis links rather than permanent equality to the latest target/head.

The conformance test enumerates the finite Cartesian product of directory
suffix `{absent, pending, committed, rolled_back, other}` and directory kind;
pending intent `{absent, exact, mismatch}` (partial is always mismatch); target
`{absent, old, new, other}`; `canonical.old`
`{absent, exact_prefix, exact, mismatch}`; each of `canonical.new`,
`promotion-record.new`, and `lifecycle-transition.new` independently
`{absent, exact, mismatch}`; promotion and lifecycle finals each `{absent,
exact, mismatch}`; and every marker independently
`{absent, tmp_exact_prefix, published, mismatch}`. Scratch is deliberately not
an authority/recovery axis because it is outside the scanned pending namespace.
The test additionally crosses terminal-marker-published pending kind
`{committed, rolled_back}` with its corresponding destination independently in
`{absent, exact_pre_existing, mismatching, unsafe}`, plus terminal rename and
parent-fsync replay. Only an absent destination may rename for either terminal
kind; `exact_pre_existing`, `mismatching`, `unsafe`, or simultaneous-suffix
destination state preserves the
complete pending evidence and refuses without replacement or cleanup. It
enumerates rollback origin `{W3..W9}`, snapshot prefix length, cleanup cursor
`{R1..R6}`, rollback-marker prefix length `{0..72}`, published rollback marker,
all four `.rolled-back` destination states, terminal rename, and parent-fsync
replay. It asserts
predicate disjointness for the nine non-catch-all rows before
applying mismatch, exactly one total row per combination, the declared result
for every `W0`-`W15` and `R0`-`R9` crash state, and mismatch for every
non-writer state.
Amendment proof separately enumerates `canonical.old` absence and every prefix
length from zero through `old_canonical_byte_length`; each recovery attempt must
end either with another exact prefix after a crash or the exact fsynced snapshot
and exact `.rolled-back` terminal set. Fault injection stops after every
output scratch create-new, each scratch write prefix in the bounded fault
fixture, file fsync, close, no-follow reopen, exact byte/hash/length/type/binding
verification, scratch-directory fsync, atomic stage rename, pending-directory
fsync, post-rename scratch-directory fsync, transaction-parent fsync, and exact
pending-stage reverify. Every pre-rename fault leaves the authoritative stage
absent; every post-rename observation is absent or exact; scratch contents never
select a recovery row. Injection also stops after every rollback reverify,
unlink, directory fsync, marker write/fsync/rename, terminal rename, and parent
fsync, plus the corresponding committed terminal rename and parent fsync;
repeated recovery must reach the same exact terminal set.
Simultaneous `.pending`/terminal destinations and crossed/malformed `.pending`,
`.committed`, and `.rolled-back` suffix/name states always select mismatch.

`committed` remains the sole authority-visibility point. Selected readers expose
a promotion only from an exact `.committed` journal. No mismatch branch deletes
or renames the journal, removes a final record, restores canonical bytes, or
starts a new transaction. Repair of preserved mismatch evidence requires a
separately authorized forensic procedure outside this packet.

### Review 2 implementation obligations

A later implementation packet must close all four Review 2 findings together:

1. implement the candidate `1.2`/validation-result `1.0` acyclic identity and
   all cross-record/currentness rules above;
2. replace the checkpoint promotion intent/recovery behavior with the complete
   intent `1.2`, marker grammar, persistence rules, and total recovery matrix;
3. preserve journal evidence and refuse every final-canonical/final-record/
   intent mismatch exactly as the matrix requires; and
4. remove public production exposure of fault injection. Injection traits,
   constructors, fail-point enums, controls, and setters must be module-private
   and compiled only under `#[cfg(test)]`; no production feature, exported
   symbol, compiler/CLI option, environment variable, or public DTO may enable
   them. Unit tests exercise every boundary through that test-only harness,
   while public-API/rustdoc/all-features proof shows no hook in production.

Before eventual code edits, refresh GitNexus impact for
`evaluate_charter_intake` (known HIGH), generic lineage `validate_record`
(known CRITICAL), and the concrete promotion-transaction recovery symbols
(known HIGH), and warn before any HIGH/CRITICAL edit. The preferred design is a
dedicated engine-owned lifecycle-validation result/service/store so the repair
does not unnecessarily expand the generic validator's CRITICAL blast radius.
Implementation stops if any preimage, persistence/currentness rule, migration,
recovery row, or production/test API boundary remains implicit.

## Objective

Cut exactly the shipped `project_authority` instance to approved canonical
Charter YAML through the first rich, versioned artifact-intake contract. All
three acquisition modes must evaluate one `CharterIntakeDefinition`, produce
the same schema-bound candidate, preserve immutable provenance and explicit
unknowns, require an authorized human approval, and promote with compare-and-
write semantics. A fixed deterministic first-party renderer must produce the
human-review Markdown view only from approved canonical YAML. No nested CLI
questionnaire, hidden model call, prompt-owned policy, self-approval, or
persistent Markdown authority is permitted.

The implementation must:

1. preserve every released `1.0.0` schema, kind, profile, capability, validator,
   and registry byte and add a separately fingerprinted Charter definition
   closure;
2. advance the selected root profile additively so only the Project Authority
   kind/schema/intake/renderer/lifecycle selections change;
3. implement the complete sixteen-item `CharterIntakeDefinition` coverage
   contract and its two targeted reassessment triggers;
4. make `guided_adaptive`, `express`, and `agent_assisted` differ only in
   acquisition interaction, never in candidate schema, required coverage,
   semantic validation, approval, or promotion quality;
5. keep the skill-directed agent responsible for conversation and evidence
   collection while Handbook owns definition resolution, evaluation,
   normalization, validation, approval checks, promotion, and rendering;
6. append immutable intake, candidate, approval, and promotion records without
   rewriting earlier lineage;
7. require explicit approval by current Charter authority over one exact
   candidate fingerprint and reject stale candidate/profile/definition/target
   state before mutation;
8. make canonical YAML, promotion, and lifecycle transition authority-visible
   only at one durable commit point; pre-commit physical files are hidden from
   selected readers and are deterministically removed or completed by recovery;
9. render a clock-, environment-, repository-, Resolution-, and model-free
   Markdown review view in memory from the retained canonical observation;
10. cut author, setup, doctor, Environment Inventory reference checks, and flow
    to the selected Charter YAML while deleting selected-path influence from
    `.handbook/charter/CHARTER.md`; and
11. prove lifecycle review/reassessment behavior reopens only mapped intake
    coverage and cannot silently regenerate or demote constitutional truth.

This is the first-party constitutional-root vertical slice, not the generic
custom-kind proof. It starts no HCM-2.3 work.

## Versioned definition closure

Released exact-definition refs are immutable. HCM-2.2 must add, not mutate, the
following refs and bind their uniform fingerprints into every consumer:

| Definition | Exact HCM-2.2 ref |
|---|---|
| canonical content schema | `handbook.schemas.artifacts.project-authority@1.1.0` |
| artifact kind | `handbook.artifact-kind.project-authority@1.1.0` |
| semantic validator | `handbook.semantic-validation.constitutional-root@1.1.0` |
| trigger-evidence schema | `handbook.schemas.lifecycle.trigger-evidence@1.0.0` |
| approver-registry schema | `handbook.schemas.security.approver-registry@1.0.0` |
| approver-registry-transition schema | `handbook.schemas.security.approver-registry-transition@1.0.0` |
| authenticator-registration-request schema | `handbook.schemas.security.authenticator-registration-request@1.0.0` |
| authenticator-challenge schema | `handbook.schemas.security.authenticator-challenge@1.0.0` |
| authenticator-make-credential-response schema | `handbook.schemas.security.authenticator-make-credential-response@1.0.0` |
| authenticator-registration schema | `handbook.schemas.security.authenticator-registration@1.0.0` |
| authenticator-get-assertion-response schema | `handbook.schemas.security.authenticator-get-assertion-response@1.0.0` |
| authenticator-assertion schema | `handbook.schemas.security.authenticator-assertion@1.0.0` |
| approver-admin API schema | `handbook.schemas.security.approver-admin-api@1.0.0` |
| intake | `handbook.intake.charter@1.0.0` |
| Markdown renderer | `handbook.renderer.charter-review-markdown@1.0.0` |
| lifecycle | `handbook.lifecycle.constitutional-review-lock@1.0.0` |
| approval policy | `handbook.approval.constitutional-candidate@1.0.0` |
| waiver policy | `handbook.waiver.constitutional-intake@1.0.0` |
| reassessment trigger | `handbook.intake-trigger.production-posture-changed@1.0.0` |
| reassessment trigger | `handbook.intake-trigger.trust-boundary-changed@1.0.0` |
| review trigger | `handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0` |
| selected shipped profile | `handbook.profile.shipped-root@1.1.0` |

The `1.1.0` kind retains stable role `constitutional_authority`, capability
`constitutional_root`, and the exact `1.0.0` capability contract. Its bindings
remain `/policy`,
`/policy/revision`, `/governance/decision_authority`,
`/governance/required_approvals`, `/governance/exception_policy`,
`/engineering_posture/dimensions`, `/engineering_posture/red_lines`,
`/governance/review_triggers`, and `/governance/reassessment_triggers`.
Because capability contract `handbook.capabilities.constitutional-root@1.0.0`
requires semantic validator `1.0.0`, the kind selects both that exact retained
validator and the additive `1.1.0` Charter-schema validator. It also selects the
exact renderer, lifecycle, and all three trigger refs above. It adds no intake
field and no Projection ref. Intake compatibility remains intentionally one-way:
`handbook.intake.charter@1.0.0` targets the kind, and only the instance
descriptor selects that intake. The kind fingerprint therefore excludes every
intake definition and cannot participate in a kind↔intake cycle.

The `1.1.0` shipped profile is a complete standalone exact profile, not a
mutable alias. Its complete ordered source lists are frozen, without a
conditional historical-source rule, as:

```yaml
schema_registry_sources:
- handbook.schemas.artifacts.project-authority@1.0.0
- handbook.schemas.artifacts.project-authority@1.1.0
- handbook.schemas.artifacts.project-context@1.0.0
- handbook.schemas.artifacts.environment-context@1.0.0
- handbook.schemas.artifacts.work-specification@1.0.0
- handbook.schemas.artifacts.decision-record@1.0.0
- handbook.schemas.artifacts.risk-record@1.0.0
- handbook.schemas.lifecycle.trigger-evidence@1.0.0
- handbook.schemas.security.approver-registry@1.0.0
- handbook.schemas.security.approver-registry-transition@1.0.0
- handbook.schemas.security.authenticator-registration-request@1.0.0
- handbook.schemas.security.authenticator-challenge@1.0.0
- handbook.schemas.security.authenticator-make-credential-response@1.0.0
- handbook.schemas.security.authenticator-registration@1.0.0
- handbook.schemas.security.authenticator-get-assertion-response@1.0.0
- handbook.schemas.security.authenticator-assertion@1.0.0
- handbook.schemas.security.approver-admin-api@1.0.0
artifact_kind_sources:
- handbook.artifact-kind.project-authority@1.0.0
- handbook.artifact-kind.project-authority@1.1.0
- handbook.artifact-kind.project-context@1.0.0
- handbook.artifact-kind.environment-context@1.0.0
- handbook.artifact-kind.work-specification@1.0.0
- handbook.artifact-kind.decision-record@1.0.0
- handbook.artifact-kind.risk-record@1.0.0
```

Relative to `1.0.0`, it:

- adds the `1.1.0` Project Authority schema/kind plus the exact trigger-evidence
  and nine security schema sources named in the complete list;
- selects `project_authority` kind `1.1.0` at the unchanged canonical path;
- binds that descriptor to the exact intake, renderer, and lifecycle refs;
- preserves all other instance descriptors, sources, requiredness, roles,
  paths, conditions, vocabulary, Context Resolution, and null/deferred fields
  byte-for-semantic-byte; and
- retains the old Project Authority schema/kind sources for historical exact-ref
  resolution exactly as the lists above require. Selection is unambiguously the
  new kind and no operation may silently fall back to `1.0.0`.

Definition loaders must validate fingerprint closure, duplicate refs, exact
compatibility, safe package-local documents, cross-ref compatibility, intake
target kind/schema, renderer input schema/media type, lifecycle trigger refs,
approval/waiver refs, and constitutional capability requirements before
profile selection. HCM-1 package-tree and installed-package equality tests must
prove every admitted old byte remains unchanged.

### Closed subordinate definition records

HCM-2.2 adds exact built-in record schemas for the new subordinate definition
categories. Each schema is closed, rejects unknown fields, requires exact
SemVer identity, uses empty `extensions`, and derives its `sha256:` fingerprint
over normalized semantic fields plus every resolved dependency fingerprint,
excluding only its own fingerprint field. The exact records are:

| Ref | Required semantic fields | Resolved fingerprint closure |
|---|---|---|
| `handbook.semantic-validation.constitutional-root@1.1.0` | exact capability-contract ref; exact candidate-schema ref; exact nine ordered dimension IDs; level range 1-5; non-empty stance/trigger/shortcut/red-line requirements; baseline and governance cross-field rules | capability contract `1.0.0`, retained required validator `1.0.0`, target schema `1.1.0` |
| `handbook.renderer.charter-review-markdown@1.0.0` | exact input-schema and required capability-contract refs, output `text/markdown`, built-in implementation ID `charter-review-markdown-v1`, deterministic profile `closed-charter-render-v1`, no Resolution input | schema/capability and the renderer boundary bytes named below; never the kind fingerprint or an intake ref |
| `handbook.lifecycle.constitutional-review-lock@1.0.0` | exact target capability-contract and approval-policy refs; states `current`, `review_required`, `reassessment_required`; initial `current`; exact trigger refs/mappings; clearance `new_approved_promotion` | capability, approval policy, review trigger, and both reassessment-trigger fingerprints |
| `handbook.approval.constitutional-candidate@1.0.0` | exact target capability-contract plus registry, transition, registration-request, challenge, make-credential-response, registration, get-assertion-response, assertion, and approver-admin API schema refs; decisions `approved`/`rejected`; fixed initial-create quorum and amendment authority/registry-anchor rules; required candidate/basis fingerprints; exact approval-pair and conditional-waiver rules | capability contract and all nine security schema fingerprints; never intake, candidate, or mutable Charter state |
| `handbook.waiver.constitutional-intake@1.0.0` | exact approval-policy ref; exact allowed coverage IDs and only `minimum_specificity` as the waivable quality obligation; required independent value-source/rationale/risk/expiry/review fields; candidate approval must explicitly accept every waiver | approval-policy fingerprint |
| each trigger `@1.0.0` | exact trigger ID/version, event kind, exact evidence-schema ref, required prior/current evidence fingerprints, change rule, sensitivity/freshness class | trigger-evidence schema only; never lifecycle state or Charter bytes |
| `handbook.intake.charter@1.0.0` | target kind/schema, three modes, sixteen exact non-overlapping coverage rows, approval policy, two trigger mappings | kind/schema, approval, waiver, and trigger fingerprints; the kind never points back |

The record field sets are exact:

- semantic validator: `schema_id`, `schema_version`, `profile_id`,
  `profile_version`, `capability_contract_ref`,
  `required_semantic_validator_refs`, `candidate_schema_ref`, `binding_rules`,
  `ordered_dimension_ids`, `cross_field_rules`, `extensions`,
  `profile_fingerprint`;
- renderer: `schema_id`, `schema_version`, `renderer_id`,
  `renderer_version`, `input_schema_ref`, `required_capability_contract_ref`,
  `output_media_type`, `implementation_id`, `determinism_profile`,
  `resolution_input` (literal `null`), `boundary_fixture` containing repo-
  relative path/size/raw SHA-256, `extensions`, `renderer_fingerprint`;
- lifecycle: `schema_id`, `schema_version`, `policy_id`, `policy_version`,
  `target_capability_contract_ref`, `states`, `initial_state`, `review_trigger_refs`,
  `reassessment_mappings`, `clearance`, `approval_policy_ref`, `extensions`,
  `lifecycle_fingerprint`;
- approval policy: `schema_id`, `schema_version`, `policy_id`,
  `policy_version`, `target_capability_contract_ref`, `decisions`,
  `candidate_fingerprint_required`, `basis_artifact_fingerprint_required`,
  `approver_registry_schema_ref`,
  `authenticator_registration_request_schema_ref`,
  `authenticator_challenge_schema_ref`,
  `authenticator_make_credential_response_schema_ref`,
  `authenticator_registration_schema_ref`,
  `authenticator_get_assertion_response_schema_ref`,
  `authenticator_assertion_schema_ref`,
  `approver_admin_api_schema_ref`,
  `approver_registry_transition_schema_ref`,
  `authenticator_requirement`, `initial_creation_authority`,
  `amendment_authority`, `approval_class_mapping`, `waiver_acceptance`,
  `registry_anchor`, `identity_assurance`,
  `extensions`, `approval_policy_fingerprint`;
- waiver policy: `schema_id`, `schema_version`, `policy_id`,
  `policy_version`, `allowed_coverage_ids`, `allowed_quality_obligations`, `required_fields`,
  `expiry_required`, `candidate_binding_required`, `approval_policy_ref`,
  `extensions`, `waiver_policy_fingerprint`;
- trigger: `schema_id`, `schema_version`, `trigger_id`, `trigger_version`,
  `event_kind`, `evidence_schema_ref`, `required_evidence_fields`,
  `change_rule`, `freshness`,
  `sensitivity`, `extensions`, `trigger_fingerprint`; and
- intake: exactly the field set and row shapes in `05`, with the concrete refs,
  rows, and mappings in this packet.

Record-schema identity is frozen, rather than inferred from a live `1.0`
loader. The additive semantic-validator record uses
`handbook.semantic-validation-profile-definition` / `1.1`; its new exact-ref,
candidate-schema, required-validator, ordered-dimension, and cross-field-rule
fields are invalid under retained record schema `1.0`. Artifact kind and profile
retain their existing record schema `1.0` while advancing definition versions.
The intake, renderer, lifecycle, approval, waiver, and trigger categories are
new closed record schema `1.0` types. The implementation must add explicit
typed loaders for those new schema IDs; it must not widen an unrelated retained
record with unknown fields.

The exact value, nested-shape, array-order, dependency-order, record-schema,
normalized-byte, and expected-fingerprint authority is the machine-readable
[`definition-closure-vectors-v1.0.json`](contracts/definition-closure-vectors-v1.0.json).
Every `fingerprint_preimage.definition` is the complete admitted record with
only its own fingerprint field absent. Therefore each object's exact keys,
nested keys, scalar types, enums/literals, ordered arrays, nulls, and empty
objects/lists are normative; the corresponding meta-schema requires all of
them and `additionalProperties: false`/`unevaluatedProperties: false` at every
object depth. Arrays are order-significant exactly as shown except
`resolved_dependencies`, which sorts by dependency role then exact ref before
serialization. No implementer-selected value or field remains.

The literal RFC 8785 UTF-8 preimage bytes are published, one line per vector,
in [`definition-normalized-jcs-v1.jsonl`](contracts/definition-normalized-jcs-v1.jsonl).
`normalized_jcs_line` is one-based; the line's terminal JSONL LF is not part of
the preimage. Each line length and SHA-256 must equal its vector's
`normalized_jcs_byte_length` and `expected_fingerprint`. The eleven complete
closed schema documents are separately frozen as exact minified-JSON+LF bytes:

- [`project-authority-1.1.0.schema.json`](contracts/project-authority-1.1.0.schema.json);
- [`trigger-evidence-1.0.0.schema.json`](contracts/trigger-evidence-1.0.0.schema.json);
- [`approver-registry-1.0.0.schema.json`](contracts/approver-registry-1.0.0.schema.json);
- [`approver-registry-transition-1.0.0.schema.json`](contracts/approver-registry-transition-1.0.0.schema.json);
- [`authenticator-registration-request-1.0.0.schema.json`](contracts/authenticator-registration-request-1.0.0.schema.json);
- [`authenticator-challenge-1.0.0.schema.json`](contracts/authenticator-challenge-1.0.0.schema.json);
- [`authenticator-make-credential-response-1.0.0.schema.json`](contracts/authenticator-make-credential-response-1.0.0.schema.json);
- [`authenticator-registration-1.0.0.schema.json`](contracts/authenticator-registration-1.0.0.schema.json);
- [`authenticator-get-assertion-response-1.0.0.schema.json`](contracts/authenticator-get-assertion-response-1.0.0.schema.json);
- [`authenticator-assertion-1.0.0.schema.json`](contracts/authenticator-assertion-1.0.0.schema.json); and
- [`approver-admin-api-1.0.0.schema.json`](contracts/approver-admin-api-1.0.0.schema.json).

Their raw lengths/document fingerprints, complete schema-registry entries,
closure fingerprints, and entry fingerprints are fixed by the vector file.
Validation reads every `document_contract_path` as literal bytes and compares
its length and SHA-256 directly; parse/reserialization is not an admissible
substitute for exact-byte equality.
The exact expected closure heads are:

| Exact ref | Expected fingerprint |
|---|---|
| `handbook.schemas.artifacts.project-authority@1.1.0` | `sha256:7420efe464c45e17319c56a233f9a54960c52f81b502ed4ffb59a479474f9836` |
| `handbook.semantic-validation.constitutional-root@1.1.0` | `sha256:10703119fbb0a4cfcab3fcdc4c618df269933591caf653786fdd8fee8cfc3c10` |
| `handbook.approval.constitutional-candidate@1.0.0` | `sha256:67466e0c26e48e7e25705b6a601e487b74923371d886d759fcd9face2e793c29` |
| `handbook.waiver.constitutional-intake@1.0.0` | `sha256:704acdfe61b4d76ece2ceac72eb99df65dff2d2998b876990f24c3410fc317e3` |
| `handbook.renderer.charter-review-markdown@1.0.0` | `sha256:68f6fceedaab6133364d18a2b474694d1b44b91d73c05d0af65db1aa58e80fa7` |
| `handbook.lifecycle.constitutional-review-lock@1.0.0` | `sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3` |
| `handbook.artifact-kind.project-authority@1.1.0` | `sha256:3b3d0b353d9c45c20781c3f4b79e30cc27847fb168d1860115f2f941b8bbef0e` |
| `handbook.intake.charter@1.0.0` | `sha256:a92229722f25119c7d91137e1feef4ce51b88ae766ce308b585d37f39eb52d1c` |
| `handbook.profile.shipped-root@1.1.0` | `sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74` |

Every record uses the uniform producer in `05` without substitution: parse to
the JSON data model, remove only its own fingerprint, resolve each exact ref,
and construct the exact two-key preimage envelope
`{definition, resolved_dependencies}`. Add typed closure entries
`{dependency_role, definition_ref, definition_fingerprint}`. The complete
augmented record is one JSON value; object keys are sorted, semantically ordered
arrays retain order, unordered closure entries sort by dependency role then
exact ref, and the one value is serialized as UTF-8 RFC 8785 JCS. Lowercase
SHA-256 over those bytes receives the `sha256:` prefix. There is no field-order
JSON producer, NUL/LF trailer, caller-trusted fingerprint, or second type-
specific hash. Type-specific rules add typed closure entries only. Independent
JCS encoders and literal normalized-record fixtures must reproduce every
fingerprint. Every resolved dependency is also compared directly to the
current producer fingerprint before hashing; reproducing a stale dependency
value is refusal, not closure. Tests refuse missing producers, producer/
dependency mismatches, cycles, duplicate identities, ref substitution, and
changed released bytes.

Only allowlisted built-in semantic-validator and renderer implementation IDs are
valid. A definition cannot name a file, URL, command, plugin, prompt, model, or
dynamic callback. Missing/extra/duplicate/substituted dimension IDs, wrong
order, changed dependency fingerprints, unsupported fields, and any definition
cycle refuse before selection. Exact meta-schema, normalized-record, dependency
order, and reproduced-fingerprint fixtures are part of the implementation wall.

## Canonical Charter schema

The `1.1.0` content schema uses `schema_id`
`handbook.artifact.project-authority`, literal `schema_version: "1.1"`, the
existing record-ID grammar, closed objects, and no application defaults. It
retains every validated semantic carrier in `CharterStructuredInput@0.1.0`
while retaining the complete `1.0` capability-bound fields:

| Field | Exact semantic content |
|---|---|
| `project` | `name`; exact classification enum `greenfield`, `brownfield`, `integration`, `modernization`, `hardening`; positive `team_size`; exact users enum `internal`, `external`, `mixed`; exact lifetime enum `days`, `weeks`, `months`, `years`; unique non-empty surface/runtime enum lists; and the three closed records below |
| `project.constraints` | explicit bounded `deadline`, `budget`, and concrete `experience_notes`; unique `must_use_tech` list, including explicit empty when none |
| `project.operational_reality` | boolean `in_production_today`; bounded `prod_users_or_data` and `uptime_expectations`; unique `external_contracts_to_preserve` |
| `project.default_implications` | exact current enums for `backward_compatibility`, `migration_planning`, `rollout_controls`, `deprecation_policy`, and `observability_threshold` |
| `posture` | literal `rubric_scale: "1-5"`, `baseline_level` 1-5, and 1-64 concrete `baseline_rationale` entries |
| `domains` | 0-64 unique closed records containing non-empty `name`, concrete `blast_radius`, unique `touches`, and unique `constraints` |
| `policy` | retained required `revision` and `authority_statement` |
| `governance` | retained non-empty `decision_authority`, `required_approvals`, `exception_policy`, `review_triggers`, and `reassessment_triggers`, plus `exception_process` with non-empty `approvers`, safe `record_location`, and non-empty unique `minimum_fields` |
| `engineering_posture` | exactly nine ordered closed dimension records plus non-empty unique cross-cutting `red_lines` |
| each dimension | exact `dimension_id`; `level_override` null or 1-5; concrete `default_stance`; non-empty unique `raise_the_bar_triggers`, `allowed_shortcuts`, and per-dimension `red_lines`; unique `domain_overrides` |
| `debt` | retained concrete `system`, unique `labels`, and concrete `review_cadence` |
| `decision_records` | retained boolean `enabled` plus `path` and `format`; when enabled both are concrete/safe, while disabled permits bounded render-safe strings including the retained empty-string pair and the renderer ignores both |

The required dimension order is:

```text
speed_vs_quality
type_safety_static_analysis
testing_rigor
scalability_performance
reliability_operability
security_privacy
observability
dx_tooling_automation
ux_polish_api_usability
```

The new semantic validator enforces the exact set and order, effective level
(`level_override` or baseline), concrete rationale/stance/trigger/shortcut/red-
line values, non-empty governance capability bindings, unique domain names,
safe record locations, and the retained current vague/placeholder and Markdown-
control refusal rules. The released `1.0.0` validator remains byte-identical and
is not claimed to enforce those rules. Tests cover missing, extra, duplicate,
substituted, and reordered dimension IDs independently.

All strings use the existing short/long text bounds and NUL refusal unless the
new schema fixes a stricter field bound. Arrays have deterministic source order,
unique semantic identities, and explicit size bounds. Unknown fields, duplicate
YAML keys at any depth, aliases/tags outside the admitted JSON data model,
multiple documents, non-object roots, invalid Unicode, unsafe paths, symlinks,
non-regular files, oversized input, wrong constants, structural failures, and
semantic-binding failures refuse closed.

The old `CharterStructuredInput@0.1.0` is reusable semantic precedent only. The
new candidate targets the `1.1` canonical schema directly. There is no mapper,
importer, dual read, compatibility profile, automatic migration, or
best-effort conversion from the old input or Markdown.

The required field-by-field preservation proof maps every live `0.1.0` field to
the identically named/new canonical field above, maps `exceptions` to
`governance.exception_process`, and maps current dimensions to
`engineering_posture.dimensions`. Deleting the old selected carrier is blocked
until that crosswalk has no omission and positive/negative tests reproduce its
classification, bounds, placeholder, required-dimension, render-safety,
default-implication, domain, exception, debt, and decision-record validation
value.

Decision-record preservation explicitly includes the live disabled case:
`enabled: false`, `path: ""`, `format: ""` is valid and renders only the fixed
disabled wording. When enabled, either empty field refuses. Disabled non-empty
values remain admitted but have no rendered or authority effect.

## Exact Charter intake coverage

`handbook.intake.charter@1.0.0` targets the `1.1.0` kind/schema and carries the
sixteen coverage IDs frozen below:

The complete literal semantic record is
[`charter-intake-definition-1.0.0.yaml`](contracts/charter-intake-definition-1.0.0.yaml)
(10,009 raw bytes; raw SHA-256
`c845cd7012764645965bc54cabc66e65c91dac468b2f80123f4c62be31565f6a`).
It is the row-level authority where this explanatory table is abbreviated. It
contains, for every row and in the exact coverage order, applicability,
authority class, inferability, declaration requirement, evidence kinds,
freshness, sensitivity, null default, requiredness, specificity, confidence,
unknown/contradiction policy, waiver ref, and prompt-guidance refs. It also
contains the exact mode order, approval ref, reassessment mappings, empty
extensions, and reproduced uniform fingerprint. No row inherits an omitted
field from `05`.

| Coverage ID | Canonical target | Authority/source rule |
|---|---|---|
| `project_shape.definition` | `/record_id`, `/project/name`, `/project/classification`, `/project/team_size`, `/project/users`, `/project/expected_lifetime`, `/project/surfaces`, `/project/runtime_environments` | observational; evidenced inference or user declaration; required |
| `delivery.constraints` | `/project/constraints` | observational; evidenced inference or user declaration; required |
| `delivery.default_implications` | `/project/default_implications` | normative; user declaration required |
| `operational_reality.production_state` | `/project/operational_reality` | observational; evidenced inference or user declaration; required |
| `risk.domains` | `/domains` | observational; evidenced inference or user declaration; required; explicit empty means baseline applies everywhere |
| `engineering_posture.baseline` | `/posture/baseline_level`, `/posture/baseline_rationale` | normative; user declaration required |
| `policy.authority_and_revision` | `/policy` | normative; user declaration required |
| `governance.decision_authority` | `/governance/decision_authority` | normative; user declaration required |
| `governance.required_approvals` | `/governance/required_approvals` | normative; user declaration required |
| `governance.exception_policy` | `/governance/exception_policy`, `/governance/exception_process` | normative; user declaration required |
| `engineering_posture.dimensions` | `/engineering_posture/dimensions` | normative; user declaration required |
| `engineering_posture.red_lines` | `/engineering_posture/red_lines` | normative; user declaration required |
| `governance.review_triggers` | `/governance/review_triggers` | normative; user declaration required |
| `governance.reassessment_triggers` | `/governance/reassessment_triggers` | normative; user declaration required |
| `debt.register` | `/debt` | observational; evidenced inference or user declaration; declaration not required; required |
| `decisions.records` | `/decision_records` | normative; user declaration required |

This packet resolves the illustrative `/governance/exceptions` target in `05`
to the capability-bound canonical `/governance/exception_policy` plus its
retained operational detail `/governance/exception_process`. The coverage ID
remains stable and no separate exception authority is created.

Coverage target paths must be pairwise non-overlapping: no path may equal or be
an ancestor/descendant of another coverage row's path. One row has one authority
class and one allowed value-source rule. Candidate field sources form a
bijection from every populated semantic leaf to exactly one coverage row/value
source. `record_id` is a project-shape value source. The schema constants
`schema_id`, `schema_version`, and `posture.rubric_scale` are the only excluded
leaves: the candidate builder contract declares their exact literal derivation,
so they have no `field_sources` row and are not defaults or intake values.
Meta-validation rejects any other missing source, mixed-authority row, broad-
parent overlap, duplicate leaf source, or mode-specific precedence.

Every item is required, uses `minimum_confidence: high`, blocks unknown or
contradicted coverage, has no deterministic default, and uses the specificity
floor in `05`. Normative items cannot use evidenced inference as their value
source. `known_unknown` remains a visible immutable intake result and candidate
gap; it never populates a normative canonical field or counts as satisfied.
Waiver is permitted only where the frozen coverage row names
`handbook.waiver.constitutional-intake@1.0.0`; a valid waiver remains `waived`,
not `satisfied`, and promotion additionally requires the approval policy to
permit that exact waiver for the exact candidate. Charter uses a waiver only
over the `minimum_specificity` quality obligation after a structurally and
semantically valid value exists from the row's normal authoritative source.
Waiver cannot supply a value or waive requiredness, user declaration, source
kind, structural/semantic validation, unknown, contradiction, approval, or
authority rules.

The waiver definition allows exactly these ten IDs and no others:
`delivery.default_implications`, `policy.authority_and_revision`,
`governance.decision_authority`, `governance.required_approvals`,
`governance.exception_policy`, `engineering_posture.dimensions`,
`engineering_posture.red_lines`, `governance.review_triggers`,
`governance.reassessment_triggers`, and `decisions.records`. Every waiver has
non-empty rationale and accepted-risk text, a bounded UTC expiry, a review UTC,
the candidate fingerprint, the exact coverage ID, literal quality obligation
`minimum_specificity`, independent underlying `source_kind: user_declaration`,
value ref/fingerprint, and an engine-derived actor/authority binding. The
coverage result retains `source_kind: user_declaration`, names the value ref,
sets `evaluation: waived`, and carries the waiver ref; `source_kind: waiver`
refuses for these Charter rows. An expired, wrong-coverage/obligation/source,
changed-candidate/value, or unaccepted waiver refuses. A promotion approval must
list every accepted waiver fingerprint as a condition.

The intake binds approval policy
`handbook.approval.constitutional-candidate@1.0.0` and exactly two targeted
reassessment mappings:

- `production-posture-changed` reopens only
  `operational_reality.production_state`; and
- `trust-boundary-changed` reopens only `governance.exception_policy`.

Unmapped/unknown triggers refuse. Reassessment appends a new intake/candidate
lineage; it does not edit an old record, mutate canonical truth, revoke an
approval, or regenerate the whole Charter automatically.

## Acquisition and immutable lineage

The engine exposes stable typed use cases for intake definition inspection,
intake evaluation/finalization, candidate construction, approval recording,
promotion, canonical validation, rendering, and registry bootstrap/add/revoke/
mapping update. Compiler and CLI are thin
adapters; generated skill assets call those operations. Exact CLI command names
must remain under the existing `handbook author charter` family and are frozen
as:

```text
handbook author charter --mode <guided-adaptive|express|agent-assisted> --from-inputs <path|-> [--expected-current-fingerprint <sha256:...>] [--json]
handbook author charter --approve-candidate <candidate-ref> --approval-class <class> --authority-ref <authority-ref> [--accept-waiver-ref <waiver-ref>]... [--json]
handbook author charter --promote-candidate <candidate-ref> --approval-ref <approval-ref> [--expected-current-fingerprint <sha256:...>] [--json]
handbook author charter --validate [--json]
handbook approvers bootstrap --initial-charter-quorum <class>=<authority-ref> [--initial-charter-quorum <class>=<authority-ref>]... [--json]
handbook approvers add-credential --approval-mapping <class>=<authority-ref> [--approval-mapping <class>=<authority-ref>]... [--json]
handbook approvers revoke-credential --credential-id-hash <sha256:...> [--json]
handbook approvers update-mapping --credential-id-hash <sha256:...> --approval-mapping <class>=<authority-ref> [--approval-mapping <class>=<authority-ref>]... [--json]
```

The engine operation identities and typed signatures are exact:

```text
bootstrap_approver_registry(request: ApproverAdminRequestV1) -> ApproverAdminResultV1
add_approver_credential(request: ApproverAdminRequestV1) -> ApproverAdminResultV1
revoke_approver_credential(request: ApproverAdminRequestV1) -> ApproverAdminResultV1
update_approver_mapping(request: ApproverAdminRequestV1) -> ApproverAdminResultV1
```

These are synchronous methods on an engine-owned `ApproverRegistryServiceV1`
constructed with exactly one injected `NativeAuthenticatorPortV1`. The port
is an effect boundary, not an authority or serializer, and its exact methods and
opaque CTAP2 transport contract is:

```text
make_credential(request_cbor: bytes) -> response_cbor: bytes
get_assertion(request_cbor: bytes) -> response_cbor: bytes
```

The engine, never the port, builds canonical CBOR. `make_credential` carries
CTAP command `0x01` with an exact map: key `1` is 32-byte `clientDataHash`, key
`2` is RP `{id: "handbook.local", name: "Handbook repository authority"}`,
key `3` is user `{id: <32 user-handle bytes>,
name: "handbook-repository-authority",
displayName: "Handbook repository authority"}`, key `4` is
`[{type: "public-key", alg: -7}]`, and key `7` is
`{rk: false, uv: true}`. `get_assertion` carries command `0x02` with an exact
map: key `1` is RP ID `handbook.local`, key `2` is 32-byte
`clientDataHash`, key `3` is the ordered allow-list of exact
`{type: "public-key", id: <credential bytes>}` descriptors, and key `5` is
`{up: true, uv: true}`. Unknown map keys, duplicate keys, indefinite-length
items, non-shortest integer/length encodings, wrong command, wrong key order,
or any value outside these engine-built maps refuses before the native call.

Before building `get_assertion`, the engine filters the retained committed
registry to credentials that are active, use sequence below 4096, ES256, and
mapped to the exact authority pair required by the current operation. Registry
administration requires the exact
`registry_admin=repository_registry_admin` pair. The engine rejects duplicate
credential IDs, refuses without native I/O when no credential is eligible, caps
the allow-list at 64 entries, and sorts descriptors strictly by raw credential-
ID bytes ascending. The authenticator response must identify exactly one member
of that frozen list; response order or authenticator preference never selects
authority.

`get_assertion` returns an exact CTAP status byte followed, only for success
status `0x00`, by one canonical CBOR map with exactly integer keys `1`, `2`, and
`3`. Key `1` is the required credential descriptor with exact text keys `id`
and `type`, whose values are the selected raw credential ID and literal
`public-key`; key `2` is the complete authenticator-data byte string; key `3`
is one complete strict ASN.1 DER ECDSA signature byte string. User entity,
credential count, extensions, unknown keys, omitted descriptor, duplicate
keys, indefinite lengths, non-shortest encodings, wrong canonical key order,
trailing bytes, an unrequested descriptor, malformed DER, or any optional
field refuses without an assertion record. Key `2` must be exactly 37 bytes:
the 32-byte `SHA-256("handbook.local")`, literal flags byte `0x05` (UP and UV
only), and one four-byte big-endian counter. RFU, backup eligibility/state, AT,
and ED flags are forbidden; attested-credential or extension data and every
trailing byte are forbidden. The decoder publishes literal
`flags_byte: 5`, `attested_credential_data_included: false`, and
`extensions_included: false` and refuses before signature admission if any of
those facts or the exact length differs.

Every received status is mapped without CBOR decode by the total, disjoint
`0x01..0xff` table in
[`ctap-status-mapping-v1.0.json`](contracts/ctap-status-mapping-v1.0.json).
The exact groups are `invalid_request`, `transaction_conflict`,
`authenticator_user_timeout`, `authenticator_resource_exhausted`,
`authorization_refused`, `authenticator_user_verification_retry`,
`authenticator_security_blocked`,
`authenticator_user_cancelled`, and `authenticator_unknown_error`, each with a
frozen retryability and next action. Status `0x2e` is
`authorization_refused`; reserved, extension, vendor, and otherwise unknown
values are `authenticator_unknown_error`. Statuses `0x31`, `0x33`, `0x36`,
`0x3b`, and `0x3f` require a fresh user-verification attempt and are retryable;
the source audit records the official name and exact mapping for every named
CTAP status. No complete received status maps to
`AUTHENTICATOR_UNAVAILABLE`; that code is reserved for an out-of-band port
availability or transport exception before a complete status byte exists.

The engine decodes the success bytes into the closed
`handbook.schemas.security.authenticator-get-assertion-response@1.0.0`
document published by
[`authenticator-get-assertion-response-1.0.0.schema.json`](contracts/authenticator-get-assertion-response-1.0.0.schema.json).
It preserves the exact status, descriptor identity, RP hash, flags, counter,
authenticator data, DER signature, and complete raw transport response. The
decoded response is content-addressed under
`.handbook/state/authenticator-get-assertion-responses/`; the later assertion
binds its exact ref, fingerprint, and schema ref. The five assertion transcript
vectors publish exact request bytes, response bytes, decoded records, and real
matching ES256 signatures. Eighteen malformed/status/descriptor/signature/
authenticator-data negative response vectors plus four credential-selection vectors freeze fake-
port decode, deterministic allow-list, and zero-call refusal behavior.

The port accepts no JCS document, record ref, fingerprint, approval class,
authority ref, registry state, result state, changed path, nonce, quorum,
mapping, or candidate object. It treats the request CBOR as opaque bytes and
returns only raw response CBOR; it may not decode, synthesize, normalize, or
return a semantic record or identity. A compiler/CLI/platform adapter may only
implement these blocking transports; it cannot construct a registration,
assertion, challenge, transition, registry state, refusal, or result. Fake-port
capture tests must assert exact command/request bytes and prove that no
candidate, authority, approval, registry, or record identity crosses the port.

`ApproverAdminRequestV1` is the closed caller-intent-only operation-
discriminated request union and
`ApproverAdminResultV1` is the closed succeeded/refused result union published
by [`approver-admin-api-1.0.0.schema.json`](contracts/approver-admin-api-1.0.0.schema.json)
under exact ref `handbook.schemas.security.approver-admin-api@1.0.0`. The JSON
envelopes use literal schema IDs `handbook.approver-admin-request` or
`handbook.approver-admin-result` and version `1.0`; unknown fields and a request
whose operation does not match its typed engine entry point refuse. No request
branch admits a registration/assertion ref, fingerprint, file, raw response, or
record-shaped value. The exact CLI-to-engine mapping is:

| CLI command | Engine operation | Request operation |
|---|---|---|
| `handbook approvers bootstrap` | `bootstrap_approver_registry` | `bootstrap` |
| `handbook approvers add-credential` | `add_approver_credential` | `add_credential` |
| `handbook approvers revoke-credential` | `revoke_approver_credential` | `revoke_credential` |
| `handbook approvers update-mapping` | `update_approver_mapping` | `update_mapping` |

The product-path operation selector is the closed engine enum
`RepositoryInvocationOperationV1`; compiler/CLI passes a variant, never a raw
token. Its exact mapping is:

| Product operation | Request operation | Engine variant | Allocator token |
|---|---|---|---|
| approver bootstrap | `bootstrap` | `ApproverBootstrap` | `bootstrap` |
| approver add | `add_credential` | `ApproverAddCredential` | `add-credential` |
| approver revoke | `revoke_credential` | `ApproverRevokeCredential` | `revoke-credential` |
| approver mapping update | `update_mapping` | `ApproverUpdateMapping` | `update-mapping` |
| initial Charter approval | `initial_charter_approval` | `CharterApproval` | `charter-approval` |
| amendment Charter approval | `amendment_approval` | `CharterApproval` | `charter-approval` |

The engine-owned `RepositoryInvocationIdentityServiceV1` supplies the two
request leaves that are intentionally absent from this CLI grammar. Setup, and
only setup, may initialize the repository identity at exact path
`.handbook/repository-identity.v1`. The file is exactly 71 lowercase ASCII bytes
with no BOM or line ending: literal `sha256:` followed by 64 lowercase hex
digits. Setup takes the promotion-then-registry locks, performs strict no-follow
inspection, and create-news the absent file through a same-parent temporary
file, file fsync, atomic rename, and parent fsync. Let `S` be 32 bytes obtained
from the OS CSPRNG. The initialized value is exactly
`sha256:` plus the lowercase hex SHA-256 of
`UTF-8("handbook.repository-identity@1.0") || 0x00 || S`. The entropy bytes are
never persisted or returned. A CSPRNG, create, fsync, rename, unsafe-path, or
exact-byte failure refuses setup; an existing valid file is preserved
byte-for-byte.

After registry bootstrap commits, the signed committed registry state is the
authority for repository identity. Setup and every identity read must recover
registry authority first and require the file value to equal that committed
value. Missing identity before bootstrap makes approve and every approver-admin
operation refuse before a native call or delta with a next action to run setup;
missing identity after bootstrap, malformed/unsafe identity state, or any
file/registry disagreement refuses and preserves evidence. No product adapter
may derive repository identity from a path, Git metadata, selected profile,
canonical artifact, fixture value, clock, environment, or caller input.
`--reset-state` does not select, delete, replace, or regenerate
`.handbook/repository-identity.v1`.

For each compiler/CLI approve or approver-admin product invocation the same
engine service allocates one operation ID from a fresh 32-byte OS-CSPRNG value
`N`, after resolving the exact repository identity `R`. The closed allocator
token is one of
`bootstrap`, `add-credential`, `revoke-credential`, `update-mapping`, or
`charter-approval`. The ID is exactly `<operation-token>-<64 lowercase hex>`,
where the hex is SHA-256 of
`UTF-8("handbook.operation-id@1.0") || 0x00 || UTF-8(R) || 0x00 ||
UTF-8(operation-token) || 0x00 || N`. It is 74 through 82 ASCII bytes and remains
unchanged through the closed request, native challenge, raw journal intent,
engine result, and product output; recovery requires equality at every carried
surface. It grants no authority and is deliberately absent from consuming
semantic records. Entropy failure refuses before the native call or any delta.
Compiler/CLI may only request this typed engine value and insert it with the
resolved identity into the closed request; neither adapter accepts, selects a
raw token, synthesizes, hashes, or persists either leaf. The exact derivations,
typed mapping, allocator-token domain, and setup/read refusal cases are frozen by
[`repository-invocation-identity-vectors-v1.0.json`](contracts/repository-invocation-identity-vectors-v1.0.json).

The allocator form is a product-path subset, not a retroactive narrowing of the
closed `ApproverAdminRequestV1` or `CharterApprovalRequestV1` correlation field.
Direct typed engine callers may continue to supply any schema-valid bounded
opaque `operation_id`; those IDs also grant no authority. The compiler/CLI path
always uses the allocator, and tests prove it accepts no raw operation ID or
token. This preserves the published request/schema vectors while making the
omitted CLI leaf total.

Mandatory predecessor recovery can fail before identity resolution, identity
resolution can fail before a repository fingerprint exists, and operation-ID
allocation can fail after the identity exists but before an ID exists. None of
those conditions may be fabricated as an `ApproverAdminResultV1` or an approval
result. Identity and operation-ID failures use the exact closed
`RepositoryInvocationPreflightResultV1` DTO with schema ID
`handbook.repository-invocation-preflight-result`, version `1.0`, and fields
exactly `{schema_id, schema_version, operation, stage, status,
repository_identity_fingerprint, operation_id, changed_paths, refusal,
next_actions}`. `operation` is one of `bootstrap`, `add_credential`,
`revoke_credential`, `update_mapping`, or `charter_approval`; `status` is
literal `refused`; `operation_id` is always null; and `changed_paths` is empty.
The closed branches are:

| Stage | Repository identity | Refusal | Retryable | Exact next action |
|---|---|---|---|---|
| `repository_identity` | null | `repository_identity_unavailable` | false | `run or repair handbook setup, then retry the complete operation` |
| `operation_id` | exact resolved `sha256:` fingerprint | `operation_id_entropy_unavailable` | true | `retry the complete operation with fresh operating-system randomness` |

The first branch requires a null identity; the second requires a non-null valid
identity; crossed nullability, any non-null operation ID, unknown fields, or any
other stage/code pairing refuses serialization. Repository-identity entropy
failure occurs only inside setup and uses the existing typed setup refusal; it
does not enter this operation envelope. Compiler/CLI projects this engine-owned
DTO directly to exact JSON or human text. Once both leaves exist, preflight is
ineligible and the operation returns only its normal exact engine result.
Predecessor-ordered registry/approval recovery remains mandatory before
identity resolution and may perform only the exact roll-forward, rollback,
cleanup, use-head restoration, and finalization effects authorized by the
recovery tables below. If recovery cannot finish safely, its typed recovery
refusal wins and no preflight DTO is emitted. After successful recovery, every
preflight branch performs zero native calls and creates no new operation-owned
filesystem, semantic-record, journal, registry, lifecycle, or canonical delta.
Its empty `changed_paths` therefore describes the refused current operation,
not prior-transaction recovery; recovery effects remain attributable through
their retained intent, marker, record, and use-head evidence.

Unsafe or incomplete mandatory recovery instead returns the distinct exact
closed `RepositoryInvocationRecoveryRefusalV1` envelope with schema ID
`handbook.repository-invocation-recovery-refusal`, version `1.0`, and fields
exactly `{schema_id, schema_version, operation, status,
repository_identity_fingerprint, operation_id, changed_paths, refusal,
next_actions}`. `operation` uses the same five-value product-operation enum as
the preflight DTO; `status` is literal `refused`; both identity fields are null;
and `changed_paths` is empty because it reports current-operation paths only.
`refusal` is exactly `{code:"authority_recovery_blocked", message:"repository
authority recovery could not complete safely", retryable:false}` and
`next_actions` is exactly `["repair retained registry/approval recovery
evidence, then retry the complete operation"]`. Unknown fields, any non-null
identity, any other code/message/retryability/action, or any normal/preflight
result substitution refuses serialization. This envelope has zero native calls
and no new current-operation-owned delta; only already-authorized recovery-table
effects may precede it and remain attributable through retained recovery
evidence. Compiler/CLI projects this engine-owned envelope directly and never
selects recovery detail or wording.

The bootstrap request branch contains exactly schema identity/version,
operation/operation ID, repository identity, null expected state/head, and
`initial_charter_quorum`. It does not admit `approval_mappings`. After validating
and sorting the caller's quorum pairs, the engine alone derives the bootstrap
credential mappings as the union of those exact pairs and the one fixed
`registry_admin=repository_registry_admin` pair, sorts them by class then
authority, and rejects a duplicate before native registration. Caller-supplied
bootstrap mappings, whether arbitrary, fixed-only, or fixed-plus-extra, are
unknown fields and refuse before the native call or any delta.

The closed request and success/refusal examples in
[`approver-admin-api-vectors-v1.0.json`](contracts/approver-admin-api-vectors-v1.0.json)
are thirteen normative positive parser/serializer vectors plus seventeen mandatory
schema-rejection vectors, including three one-over limit cases and explicit
bootstrap mapping injection refusal. The result is
an operation-by-status/refusal
discriminated union, not a status-only envelope. Bootstrap success requires
null prior and assertion identities, non-null registration/result/transition
identities, and exactly make-credential-response/registration/state/transition
changed paths. Add
success requires non-null prior/registration/assertion/result/transition
identities and exactly make-credential-response/registration/get-assertion-
response/assertion/state/transition changed paths. Revoke and update success
require non-null prior/assertion/result/transition identities, null registration
identity, and exactly get-assertion-response/assertion/state/transition changed
paths. Every success
requires literal `refusal: null`.

Every refused result, for every operation and allowed refusal code, requires
all prior/result/transition/registration/assertion refs and fingerprints null
and `changed_paths: []`; no valid ceremony or observed semantic identity is
published on failure. Each operation has its own closed refusal-code set.
Crossed operation/status, null success identity, forbidden non-null identity,
non-null success refusal, and changed-path arity/order examples must fail
schema validation. Adapters may add no transport-owned semantic field. The
total received-status mapping fixture above is authoritative for the refusal
code, retryability, and non-empty exact next action emitted by all four
operations; representative
vectors cover success, every semantic group, and reserved, extension, and
vendor unknowns and carry the exact Admin projection. Every refused Admin
result requires at least one next action; a received-status result must equal
the fixture projection rather than merely choose a schema-valid string. The
exact platform refusal code is
`AUTHENTICATOR_UNAVAILABLE`; its published result vector proves that native
authenticator unavailability returns before registration, assertion, journal,
record, canonical, registry, or lifecycle delta.
`add_credential` additionally admits exact `lockout_refused`; the published
sequence-4095 incomplete-replacement result has every semantic ref null,
`changed_paths: []`, zero native calls, and no registry/use-head delta. A
crossed-code negative proves bootstrap cannot emit that refusal. The complete
[`final-use-add-lockout-vectors-v1.0.json`](contracts/final-use-add-lockout-vectors-v1.0.json)
scenario freezes the exact request, sequence-4095 registry/use-head prestate,
missing required-pair coverage, all-null refused result, empty native-call log,
and byte-identical before/after authority state.

The first form accepts one typed input envelope containing declarations,
evidence refs, explicit unknowns, and contradictions. It writes an immutable
intake record and candidate/content only; it never writes canonical truth. The
approve form treats class and authority as an untrusted exact-pair selector.
The engine re-derives the pair from committed registry state, requires it to be
currently required and mapped to the asserting credential, binds it into the
signed challenge, and refuses an already-satisfied, unmapped, or ambiguous
pair. It records one authorized human decision and never mutates the candidate.
The promote form re-resolves all definitions and current state,
validates lineage and approval, then performs the atomic canonical/promotion
write. Validate is non-mutating and validates the selected canonical truth.
Legacy `--from-inputs` without `--mode`, direct old Markdown authoring, and any
interactive nested wizard refuse with typed next actions rather than guessing.
Approver-admin commands accept no registration/assertion file or raw
authenticator response. Inside the one locked engine call, the state machine is
exactly `intent_validated -> authority_observed -> registration_verified? ->
challenge_frozen -> assertion_verified? -> transaction_prepared -> committed`.
Bootstrap executes the registration step and omits assertion. Add executes both
steps: it first obtains and verifies a new unauthoritative FIDO2 credential,
derives the exact prospective result state and challenge, then obtains and
verifies a fresh current-admin assertion. Revoke and update omit registration
and execute assertion. The engine alone builds JCS requests/challenges, calls
the raw port, decodes/validates the returned bytes, constructs content-addressed
registration/assertion records in memory, and supplies their bytes to the
single registry transaction. No ceremony state, record, ref, or path is
persisted before `transaction_prepared`; any earlier refusal discards it and
returns the all-null refused branch. Every argument is untrusted, re-derived
against committed state, and included only where allowed by the typed result;
command omission, empty/duplicate mappings, unknown/revoked target, stale head,
invalid native bytes/signature, or missing authenticator refuses before any
semantic record or filesystem delta.
For intake/candidate creation and promotion, the bracketed expected-current flag
is create-only optionality: when selected canonical truth is absent, it must be
omitted and `basis_artifact_fingerprint` is literal null. When truth exists, it
is mandatory and non-null; omission, null, or mismatch refuses before intake
finalization, approval, the promotion lock, or any filesystem delta. Candidate
finalization resolves the selected target exactly once and freezes that same
basis. The additive immutable records are exact extensions of the `05` shapes:

| Record | Exact additive schema/version | Added immutable fields |
|---|---|---|
| intake | `handbook.artifact-intake-record` / `1.1` | `basis_artifact_fingerprint` |
| candidate | `handbook.artifact-candidate` / `1.1` | `basis_artifact_fingerprint` |
| approval | `handbook.artifact-approval-record` / `1.1` | `basis_artifact_fingerprint`, `approval_class`, `authority_ref`, `approver_registry_state_ref`, `approver_registry_state_fingerprint`, `registry_head_transition_ref`, `registry_head_transition_fingerprint`, `authenticator_assertion_ref`, `authenticator_assertion_fingerprint`, ordered `accepted_waivers` entries `{waiver_ref, waiver_fingerprint}` |
| promotion | `handbook.artifact-promotion-record` / `1.1` | `basis_artifact_fingerprint`, `approver_registry_state_ref`, `approver_registry_state_fingerprint`, `registry_head_transition_ref`, `registry_head_transition_fingerprint`; retained `expected_current_artifact_fingerprint` must equal the basis |

The fields are required in all four `1.1` schemas: null is valid only for the
same initial-create lineage, and a lowercase `sha256:` value is required for an
amendment. Every downstream record must equal its upstream basis byte-for-byte.
Approval re-observes current canonical truth and requires current equals basis;
promotion requires CLI expected-current, current observation, candidate basis,
every approval basis, `intent.json` basis, promotion basis, and retained
`expected_current_artifact_fingerprint` all equal. Retry and recovery preserve
that equality. The released `1.0` record shapes remain unchanged and are not
valid for HCM-2.2 Charter lineage.

### Approval authority and bootstrap

The approval operation has no file/stdin approval-input, confirmation string,
`--yes`, caller-supplied actor, caller-supplied authority, or agent-attestation
fallback. It succeeds only after a registered FIDO2 authenticator produces a
fresh native CTAP2.1 assertion with user presence and user verification.

The approver registry is not a mutable configuration file. It is an immutable,
content-addressed `handbook.approver-registry-state@1.0` record validated by
`handbook.schemas.security.approver-registry@1.0.0`. State contains exact schema
identity/version, registry ID/version, repository-identity fingerprint, a
non-empty ordered `initial_charter_quorum`, ordered credential records, empty
`extensions`, and `registry_state_fingerprint`. It contains no prior-state or
transition-head field. Credentials contain `credential_id_hash`,
`cose_public_key_base64`, literal `algorithm: ES256`, a
non-empty ordered `approval_mappings` list of closed
`{approval_class, authority_ref}` pairs, `status`, registration ref/fingerprint,
and empty `extensions`. Credential order is ascending `credential_id_hash`;
quorum and mapping pairs sort by class then authority and are unique.

The immutable `initial_charter_quorum` is chosen once by the human bootstrap
command frozen in the CLI grammar above and is never caller-selected by author,
approve, or promote. Repeated quorum arguments are exact UTF-8
`class=authority` pairs, sorted class then authority; empty class/authority and
duplicate pairs refuse. An empty quorum also refuses. The bootstrap credential's exact mappings must
be engine-derived to cover every quorum pair and the fixed authentication-only pair
`registry_admin=repository_registry_admin`. Registry transitions must preserve
the initial quorum byte-for-byte. Initial Charter promotion requires one fresh
approved record for every exact quorum pair, in lexical pair order; a caller
cannot choose a subset, duplicate pairs count once, and any missing pair
refuses. The quorum does not define later constitutional truth: after a Charter
exists, its `governance.decision_authority` and
`governance.required_approvals` determine amendment approvals.

Registry transitions are immutable
`handbook.approver-registry-transition@1.0` records. A transition binds operation,
repository identity, nullable prior transition ref/fingerprint, nullable prior
state ref/fingerprint, result state ref/fingerprint, changed credential,
nullable bootstrap-only admin credential, authorization kind/ref/fingerprint,
audit UTC, and its fingerprint. Bootstrap authorization kind is literal
`registration` and binds the exact verified bootstrap registration; all prior
and admin fields are null. Later `add_credential`, `revoke_credential`, and
`update_mapping` authorization kind is literal `assertion`, every prior/admin
field is non-null, and the authorization is a fresh assertion by an active
`registry_admin` credential. The result state never points back to the
transition, so the identity graph is acyclic. Current registry authority is the
pair `(committed result-state fingerprint, committed head-transition
fingerprint)`. No second bootstrap or unsigned state is admitted.

Every result state must retain at least one usable credential mapped to
`registry_admin=repository_registry_admin`, every immutable initial-quorum pair,
and, after initial promotion, every exact pair currently required by canonical
Charter authority. `usable` means active with committed authenticator-use
sequence below 4096. Add, revoke, update, approval use, or exhaustion that would
remove the last usable covering credential for any such pair refuses before
challenge construction. This prevents
administrative or constitutional lockout without making registry mappings
Charter truth.

### Global authority lock order and retained observations

The global authority lock order is exactly **promotion, then registry, then
lifecycle**. Any operation that needs more than one authority domain acquires
its locks in that order, releases them in reverse order, and runs each domain's
recovery in acquisition order before observing authority. An operation may take
one lock, or a suffix of the order, only when it will never acquire an earlier
lock while that lock is held. No admitted path waits promotion-after-registry,
promotion-after-lifecycle, or registry-after-lifecycle. Recovery uses the same
order; a process that cannot acquire the complete required sequence performs no
platform call and creates no intent or semantic record.

Registry and approval journals that can advance an authenticator-use head form
one recovery domain under the held promotion and registry locks. Approval,
registry administration, and promotion must discover and recover that complete
domain before authority observation, challenge construction, any platform call,
or a new intent. Discovery scans registry journal paths in lexical byte order,
then approval journal paths in lexical byte order, validates every safe path and
raw intent, and builds one predecessor graph over the bound use-head
fingerprints. Recovery advances from the committed use head only through the
unique journal whose bound prior head is current; two successors of one prior,
an unreachable prior, a cycle, duplicate sequence, or a family-local scan is a
durability-contract violation. No family precedence can override predecessor
reachability.

Approval acquires promotion then registry, recovers both lock domains and both
use-journal families, and retains
one no-follow observation containing the exact canonical target/basis bytes,
candidate and resolved-definition fingerprints, and committed registry
state/head refs, fingerprints, and bytes. It holds both locks through challenge
construction, the native authenticator call, assertion verification against
the observed active registry key, the final exact-pair/currentness recheck, and
the approval-journal commit marker. The challenge and approval record
bind the same retained registry pair. Any mismatched observation, missing lock,
or changed byte refuses and requires a new challenge; there is no release-and-
resume approval path.

Registry bootstrap/add/revoke/update acquires promotion then registry, recovers
both lock domains and both use-journal families, retains the selected canonical
authority and registry pair, and holds both
locks through lockout validation and the registry commit marker. Promotion
acquires promotion then registry then lifecycle, recovers all three lock
domains including both use-journal families before lifecycle recovery, retains
the exact registry pair used to validate every approval, and binds that pair
into its
intent and promotion record, rechecks the retained bytes immediately before the
commit marker, and holds all three locks through finalization. Thus no admitted
registry transition can interleave with approval or promotion, and no lifecycle
clearance can fork promotion. Standalone lifecycle events take only lifecycle;
registry-state readers that do not consume authenticator-use authority take
only registry; every reader/consumer of authenticator-use authority takes
promotion then registry; selected Charter readers take only promotion.
Concurrency proof must cover approval-versus-registry mutation,
promotion-versus-registry mutation, promotion-versus-standalone lifecycle,
recovery contention, reverse-order-attempt refusal, starvation bounds, and
deadlock absence.

Every registry writer therefore holds the promotion and registry locks,
recovers every pending registry and approval journal in the shared domain, and
may create no new intent until recovery completes. Bootstrap/add/revoke/update
then:

1. create-news `.handbook/state/transactions/registry/<transaction-id>.pending/`;
2. writes+fsyncs `intent.tmp`, renames it to `intent.json`, and fsyncs the
   directory. Intent binds transaction ID, operation, operation ID, observed prior state/head
   (both null only for bootstrap), authorization kind/ref/fingerprint, exact
   operation-discriminated record set, every final path/ref/fingerprint, and
   every staged-byte fingerprint. The exact set is make-credential response,
   registration, result state, transition, and initial authenticator-use head
   for bootstrap; make-credential response, registration, authorizing
   assertion, result state, transition, authorizing authenticator-use
   transition, and replacement authenticator-use head for add; and authorizing
   assertion, result state, transition, authorizing authenticator-use
   transition, and replacement authenticator-use head for revoke/update;
3. writes+fsyncs one `.new` file for every intent member, verifies every byte,
   and fsyncs the directory;
4. installs every final record create-new or exact-equal and atomically replaces
   the one transaction-owned authenticator-use head only after retaining and
   binding its prior bytes, fsyncs every affected parent,
   writes+fsyncs `records-installed`, then writes exactly 72 ASCII bytes to
   `committed.tmp`: `sha256:<64 lowercase hex>\n`, where the hex is SHA-256 of
   the exact raw `intent.json` bytes. It fsyncs `committed.tmp`, renames it to
   `committed`, and fsyncs the pending directory. That rename is
   the sole authority-visible point; and
5. verifies every bound byte, renames `.pending` to `.committed`, fsyncs the
   transaction parent, and releases the lock.

For this protocol, a matching marker means byte-for-byte equality with that
recomputed 72-byte payload. Missing LF, an extra byte, uppercase hex,
truncation, or a hash of parsed rather than raw intent bytes is mismatching and
refuses.

Readers take the registry lock, recover first, and derive current state/head only
from matching committed journals in one unbroken chain. Recovery ignores
diagnostic-marker optimism, validates safe paths and exact intent/staged/final/
marker bytes, and applies:

| Observed state under registry lock | Commit marker | Required recovery |
|---|---|---|
| empty directory or owned partial `intent.tmp` only | absent | delete owned partial state |
| valid intent; no finals; every operation-required staged file exact | absent | install the complete operation-required set and roll forward commit |
| valid intent; no finals; any operation-required staged file absent/partial | absent | delete owned staged/journal state; caller may resubmit the operation |
| valid intent; a proper subset of exact finals and every remaining required staged file exact | absent | install the complete remaining set and roll forward commit |
| valid intent; a proper subset of exact finals and any remaining required staged file missing/partial | absent | restore the exact prior use-head if it was replaced, leave immutable finals as invisible orphans, and delete owned staged/journal state |
| valid intent; every operation-required final exact | absent | revalidate authorization, result, use-chain, and counter/replay invariants and roll forward commit |
| valid intent; every operation-required final exact | matching | finalize pending to committed |
| valid intent; any operation-required final missing | matching | refuse durability-contract violation and preserve evidence |
| valid non-bootstrap intent whose bound prior state/head is not current, or bootstrap intent when a committed head exists | either | refuse durability-contract violation and preserve evidence; an admitted writer cannot advance the head because it must recover this journal first |
| unsafe/mismatched intent, authorization, staged/final byte, path, or marker | either | refuse and preserve evidence |

Final content-addressed records are never deleted. Cleanup is idempotent and
removes only owned staged/journal bytes. Crash injection covers every create/
write/fsync/rename boundary and table row plus concurrent readers/writers,
bootstrap races, partial ceremony-record installation, initial/replacement
use-head crashes, and add-enrollment orphans. A response, registration,
assertion, use transition, result state, or registry transition outside a
matching committed journal is never authority even if its content-addressed
final file exists.
The first committed Charter promotion anchors the committed bootstrap state and
head. Every later approval/promotion validates an unbroken signature-valid
committed transition chain from the pair anchored by current Charter promotion
to the current pair. Each transition's priors must equal its predecessor's
state/head and repository identity. Replacement, deletion, fork, cycle,
missing link, changed bytes, uncommitted head, or another root refuses. The
next promotion anchors the validated current pair. Direct file replacement can
therefore mint no amendment authority.

The one admitted authenticator protocol is native FIDO2 CTAP2.1 with RP ID
literal `handbook.local`; browser WebAuthn origin is not used and no origin
claim is made. Registration uses `authenticatorMakeCredential`; the request is
RFC 8785 JCS and
`clientDataHash = SHA-256(request JCS UTF-8)`. Let `R` be the exact lowercase
ASCII repository identity fingerprint, including its literal `sha256:` prefix.
The sole admitted derivation is
`user_handle_bytes = SHA-256(UTF-8(R))`; CTAP user `id` is those exact 32 bytes,
and JSON `user_handle_hash` is `sha256:` followed by their 64 lowercase hex
digits. Hashing decoded fingerprint bytes, omitting the prefix, hashing uppercase
text, adding a domain separator, or using the UTF-8 fingerprint text itself as
the user ID refuses.
[`user-handle-derivation-vectors-v1.0.json`](contracts/user-handle-derivation-vectors-v1.0.json)
publishes two normative derivations (including the all-`1` repository-
fingerprint vector) and all five negative transforms.
RP ID, quorum/mappings, algorithms, nonce,
and repository identity are signed inputs. The decoded request must validate
against
`handbook.schemas.security.authenticator-registration-request@1.0.0`; that
closed schema freezes separate bootstrap and add-enrollment shapes, every
key/type, literal RP ID/name, prior-state binding for add, quorum/mapping pair
shapes, algorithm order, attestation/user-verification policy, 32-byte
standard-base64 nonce encoding, and empty extensions. An add-enrollment
registration is an unauthoritative candidate until the active admin's later
assertion commits its result state. The
closed registration record is validated by
`handbook.schemas.security.authenticator-registration@1.0.0` and preserves the
request-schema ref, request bytes/fingerprint, RP/user/client-data bindings, credential ID, COSE
public key, algorithm, authenticator data, attestation object/format/trust,
UP/UV flags, counter, decoded-response ref/fingerprint, audit UTC, and
registration fingerprint.

The raw make-credential response must also decode into
`handbook.schemas.security.authenticator-make-credential-response@1.0.0`.
For admitted `fmt: none`, the attestation object is valid CBOR with exact keys
`fmt`, `attStmt`, and `authData`; `attStmt` is an empty CBOR map. Authenticator
data contains RP-ID hash, flags `UP|UV|AT` (`0x45`), sign count, 16-byte AAGUID,
big-endian credential-ID length, credential ID, and a complete allowlisted COSE
public key. RP hash, flags, counter, credential ID/hash, literal ES256
algorithm/key, raw
authenticator data, and raw attestation object must equal the registration
record and decoded response byte-for-byte. Missing AT, trailing/truncated CBOR,
wrong map arity, key off-curve/unsupported algorithm, mismatched counter/ID/key,
or non-empty `none` attestation statement refuses. Attestation
policy is exactly `none`; the design ships no
manufacturer trust roots and makes no hardware-proven or real-world identity
claim.

For each approval or registry mutation, the engine creates a cryptographically
random 32-byte nonce from the OS CSPRNG and an operation-specific closed RFC
8785 JCS challenge. Approval challenges contain exact keys `schema_id`,
`$schema`, `schema_version`, `operation`, `operation_id`,
`repository_identity_fingerprint`, `candidate_ref`, `candidate_fingerprint`,
`basis_artifact_fingerprint`, `approval_class`, `authority_ref`,
`approver_registry_state_fingerprint`, `registry_head_transition_fingerprint`,
`accepted_waiver_fingerprints`, and `nonce_base64`. Registry mutation challenges
instead contain exact keys `schema_id`, `schema_version`, `operation`,
`$schema`,
`operation_id`, `repository_identity_fingerprint`,
`prior_registry_state_fingerprint`, `prior_transition_fingerprint`,
`result_registry_state_fingerprint`, `changed_credential_id_hash`, and
`nonce_base64`; candidate/basis/class/authority/waiver keys are forbidden.
Decoded challenges must validate against
`handbook.schemas.security.authenticator-challenge@1.0.0`; the assertion record
binds that exact schema ref.

Each `--accept-waiver-ref` resolves to one exact
`{waiver_ref, waiver_fingerprint}` entry before challenge construction. Duplicate
refs, duplicate fingerprints, a ref/fingerprint mismatch, an unaccepted
candidate waiver, or more than ten entries refuses. Approval-record entries
sort strictly ascending by the tuple `(waiver_ref UTF-8 bytes,
waiver_fingerprint lowercase ASCII bytes)`. The challenge carries their unique
fingerprints sorted strictly ascending by lowercase ASCII bytes; an already-
serialized unsorted or duplicate challenge array refuses before
`clientDataHash` or the native call. Thus CLI permutation cannot change signed
or recorded authority bytes.
[`waiver-acceptance-order-vectors-v1.0.json`](contracts/waiver-acceptance-order-vectors-v1.0.json)
freezes two-waiver forward/permuted normalization plus duplicate-ref,
duplicate-fingerprint, unsorted-challenge, and duplicate-challenge refusals.
Native `authenticatorGetAssertion` receives
`clientDataHash = SHA-256(challenge JCS UTF-8)` and signs exactly
`authenticatorData || clientDataHash`. The authenticator-data RP ID hash must
equal `SHA-256(UTF-8("handbook.local"))`; UP and UV flags are required. The
engine first verifies the returned decoded-response ref/fingerprint/raw bytes,
credential descriptor membership, strict DER encoding, and ES256 signature
under that descriptor's committed registry key. The exact assertion bytes,
decoded-response schema/ref/fingerprint, challenge bytes/fingerprint, RP ID,
signed-preimage hash,
credential ID hash, signature, authenticator data, client-data hash, algorithm,
verification flags, counter, audit UTC, and fingerprint are content-addressed
under `.handbook/state/authenticator-assertions/` and validated by
`handbook.schemas.security.authenticator-assertion@1.0.0`. Missing UV/UP,
replayed nonce/challenge, unknown/revoked credential, wrong signature/key/RP,
counter regression when either prior/current counter is nonzero, or any request,
client-data, signed-preimage, repository, or challenge mismatch refuses. There
is no recovery-code, environment-variable, stdin, file, software-signature, or
unattended fallback.

Replay and sign-count state is one shared per-credential authority across
approval and all registry-admin operations, never an in-memory or command-local
cache. The canonical no-follow head path is
`.handbook/state/authenticator-use-heads/<credential-id-hash>.json`. An exact
closed `authenticator-use-head@1.0` object contains `$schema`, `schema_version`,
`credential_id_hash`, `sequence`, `last_assertion_ref`,
`last_assertion_fingerprint`, `last_challenge_fingerprint`,
`last_nonce_sha256`, `sign_count`, and `head_fingerprint`; sequence zero has
literal null in the four `last_*` fields and the registration counter, while a
positive sequence requires all four identities. An exact closed immutable
`authenticator-use-transition@1.0` contains `$schema`, `schema_version`,
`credential_id_hash`, `sequence`, `prior_head_fingerprint`,
`result_head_fingerprint`, `assertion_ref`, `assertion_fingerprint`,
`challenge_fingerprint`, `nonce_sha256`, `prior_sign_count`, `sign_count`,
`consumer_kind` (`approval` or `registry_admin`), `consumer_id`, and
`transition_fingerprint`. Fingerprints use the normal self-fingerprint omission
rule. Sequence is bounded to `0..4096`; after 4096 committed assertions the
credential must be replaced before another use.

Bootstrap installs sequence-zero head authority in its registry journal. Every
accepted approval or later registry-admin assertion traverses the immutable
predecessor chain from the retained current head for at most its sequence,
requires one unbroken fingerprint/sequence chain, and rejects a repeated
challenge fingerprint or nonce SHA-256 anywhere in that bounded chain. It then
installs the assertion, use transition, consuming semantic record(s), and
replacement head in one authority transaction while holding promotion then
registry locks. Registry-admin uses the registry journal defined above.
Approval uses an operation-discriminated
`.handbook/state/transactions/approvals/<approval-id>.pending/` journal with the
same raw-intent-hash marker, complete-set staging/install, retained-prior-head,
proper-subset rollback, committed-marker, and recovery rows as the registry
protocol; its exact set is assertion, use transition, approval record, and
replacement use head. No assertion or semantic record is visible unless that
journal commits, and recovery restores the exact prior head before abandoning
partial immutable finals.

The approval intent is closed and binds `transaction_id`, `operation_id`, `approval_id`, exact
canonical/candidate/class/authority and retained registry-pair identities,
`credential_id_hash`, prior and result use-head paths/fingerprints/sequences and
raw-byte fingerprints, assertion/use-transition/approval-record final paths,
refs, fingerprints and staged-byte fingerprints, and the exact four-member
record set. Registry intents that consume an assertion bind the same use-head
fields in addition to their operation-discriminated registry set. Both
families use the same 72-byte raw-intent-hash marker and the following
cross-family advanced-head rules after complete-domain discovery:

| Committed shared head / pending journal state | Required recovery |
|---|---|
| current equals pending prior; disk head is prior; marker absent | apply that family's complete-set recovery table; either roll forward the unique complete set or abandon without changing the head |
| current equals pending prior; disk head is pending result; marker absent | roll forward only when every required immutable final/staged byte is exact; otherwise restore the intent-bound prior bytes, fsync, and abandon immutable orphans |
| current equals pending prior; disk head is result; matching marker and every final exact | admit the result as current and finalize the pending directory |
| current equals pending result through that same committed journal | finalize/clean the idempotent pending directory; never restore the prior head |
| current is a signature-valid committed descendant of pending result | finalize stale owned state only when the pending transaction is already an ancestor in the committed chain; an uncommitted skipped predecessor is a durability violation and no head is restored |
| current is neither the pending prior, result, nor a committed descendant; disk head differs from both | refuse, preserve evidence, and make no platform call or new intent |
| two valid pending journals from either family bind the same prior head, or their result sequences conflict | refuse fork/ABA durability violation and preserve both |

Because an admitted writer always completes this shared recovery before new
intent, two live successors cannot arise from admitted concurrency. Fault tests
must nevertheless inject approval crash then admin/promotion recovery, admin
crash then approval/promotion recovery, both-family discovery order, replaced-
head-before-marker, marker-before-directory-finalization, stale committed
ancestor cleanup, duplicate-prior fork, and every table row. No recovery may
restore a head older than the current committed chain.

Counter acceptance is exact: prior `0` plus current `0` is allowed for a
counterless authenticator; prior `0` plus current greater than `0` is allowed;
prior greater than `0` requires current strictly greater than prior. Equal,
decreasing, or reset-to-zero values after a nonzero prior refuse. Even the
allowed `0 -> 0` case must advance sequence and consume the unique nonce and
challenge. Sequential and deliberately interleaved approval/admin tests must
prove monotonicity, duplicate-nonce and non-adjacent replay refusal, shared-head
ABA refusal, crash before and after head replacement, orphan exclusion, exact
recovery, and no partially visible use.

Sequence `4095` is final-use-only because one accepted assertion produces the
terminal sequence `4096`; a sequence-4096 credential is active but not usable
and can never assert again. Before constructing a challenge, the engine
simulates the result use head and re-evaluates every registry-admin,
initial-quorum, and current constitutional pair against other usable
credentials in the same authority result. A final use is admitted only when
the committing transaction leaves every pair covered by a distinct usable
credential. For `add_credential`, the newly verified distinct credential and
its engine-validated mappings may satisfy that result-state proof atomically;
it is not counted before the same registry journal commits. An approval,
revoke, update, or ordinary admin use by the sole usable covering credential at
sequence 4095 refuses before native I/O. An already exhausted credential may be
revoked/replaced only through another usable administrator. Tests cover
sequence-4095 ordinary refusal, final-use replacement success, exhaustion with
another administrator, incomplete replacement rollback, crash before/after
use-head replacement and registry marker, and recovery without a window in
which neither old nor new admin is usable.

Every security-facing collection and raw/base64 field is bounded at first
schema admission. The authoritative exact maxima and one-over rejection cases
are published by
[`security-boundary-limit-vectors-v1.0.json`](contracts/security-boundary-limit-vectors-v1.0.json).
The implementation must validate those bounds before base64 decode, CBOR/JCS
parse, allocation proportional to caller data, signature verification, or
filesystem mutation. A schema field listed by that fixture without the same
limit, a newly admitted collection/raw field absent from the fixture, or any
one-over value accepted by schema validation fails the security gate. Exact
proof compares the complete set of bounded security-schema pointers with the
complete fixture-pointer set; both approval-challenge branches must contribute
their waiver and nonce pointers.

[`authenticator-transcript-vectors-v1.0.json`](contracts/authenticator-transcript-vectors-v1.0.json)
publishes literal decoded objects, RFC 8785 JCS bytes, lengths/fingerprints,
client-data hashes, authenticator data, and signed preimage bytes/hashes for
bootstrap and add-enrollment registration with coherent decoded CBOR responses;
add, revoke, and mapping-update assertions; initial Charter approval; and
amendment approval. Independent encoders/CBOR decoders must reproduce all seven
positive ceremonies. The challenge schema has three closed branches: one
registry-mutation branch, one `initial_charter_approval` branch requiring a
literal null basis, and one `amendment_approval` branch requiring a lowercase
SHA-256 basis. The fixture carries both crossed-operation documents as explicit
pre-native-call negatives. Schema validation must reject those two plus extra/
null/cross-operation fields,
wrong algorithm order, non-32-byte nonce, wrong RP, changed client-data bytes,
or a changed signed preimage.

The bootstrap and add-enrollment make-credential responses use distinct
credential bytes, distinct P-256 ES256 COSE public keys generated from distinct
keypairs, response IDs, and response fingerprints. Cross-vector proof must
decode both keys, prove each is on-curve and allowed, and assert byte and
coordinate inequality. The add assertion's
`changed_credential_id_hash` must equal SHA-256 of the exact credential ID bytes
in the add-enrollment response; the bootstrap credential hash must not satisfy
that equality. Cross-vector proof validates those values before any result state
or transition can be admitted.

For initial creation, required `(approval_class, authority_ref)` pairs come only
from the immutable registry-state quorum. For amendment, the engine reads
current canonical truth and forms the Cartesian requirement of every distinct
`governance.required_approvals` class paired with an exact member of
`governance.decision_authority` according to the approval-policy mapping.
A registry credential counts only where its exact class/authority pair is
required and equals both untrusted CLI selectors; one credential may cover multiple pairs only through separate fresh
assertions and records. Duplicate records for one pair count once and never
satisfy another pair. `actor_ref` is derived as
`credential:<credential_id_hash>`; authority, class, basis, registry state/head,
candidate, and accepted-waiver fingerprints are engine-derived challenge
values, never caller record fields. Promotion requires one currently valid
approved `1.1` record per required pair in lexical pair order and exact
acceptance of every candidate waiver.

The assurance claim is exactly `fido2_user_verified_repository_approver`: it
proves a valid CTAP2.1 signature plus user-presence/user-verification for a
repository-registered credential under the trusted-workstation/OS assumption.
It does not prove hardware provenance, legal name, organization membership,
employment, or real-world role. On a platform without the required authenticator
API, approval and registry mutation return `AUTHENTICATOR_UNAVAILABLE` before
semantic-record, journal, registry, lifecycle, or canonical delta; the closed
admin-result union and literal platform-refusal vector carry that exact code.
There is no weaker native fallback. Tests
cover bootstrap races/quorum weakening, unsigned/edited/uncommitted states,
transition/state identity cycles, wrong RP/client-data/signed preimage, forged
mappings, absent/replayed UV/UP assertions, agent-only invocation, stale
basis/authority/classes, multi-pair coverage, rejection, waiver acceptance,
revocation, registry fault recovery, and native unavailability.

Immutable semantic records use the exact `05` v1 base shapes plus only the
additive `1.1` fields frozen above and live below a
repo-local trusted `.handbook/state/` store partitioned by record class and
content-addressed ID. Candidate normalized YAML is content-addressed and not
the canonical path. Append uses no-follow, create-new semantics and byte equality
for idempotent replay. Changed bytes under the same ID, cross-record target or
fingerprint mismatch, reordered/stale lineage, unauthorized/rejected approval,
missing current validation, or changed selected profile/definition/target
fingerprint refuses without canonical mutation.

Runtime-record identity is exact rather than governed by a blanket timestamp
rule. For each class the fingerprint preimage is the complete closed record
after removing its own in-record ID when one exists, own fingerprint, and only
the audit-only fields listed below. The content-addressed ID is
`<record-class>_<64 lowercase SHA-256 hex>`; registry state uses that as its
derived path identity because its exact schema intentionally has no in-record
ID. Every other timestamp is semantic and remains in the preimage:

Every content-addressed cross-record ref uses the exact derived or in-record ID,
never a friendly filename. Registration refs are
`authenticator-registrations/<registration_id>.json`; registry-state refs are
`registry-states/<derived registry-state ID>.json`; transition and assertion
refs follow the same rule in their partitions. Resolving a ref requires all four
equalities: the ref basename equals the target ID, the target record recomputes
that ID, the declared fingerprint equals the target fingerprint, and the exact
target bytes recompute both. The bootstrap runtime vectors normatively bind the
registration ref in registry state and the result-state/authorization refs in
the transition; changing only a fingerprint can never repair a wrong ref. The
runtime fixture publishes a complete 17-edge `cross_record_bindings` table for
candidate/intake, approval, promotion, registry, ceremony, and lifecycle
targets. Every row names the exact source field, partition ref, target ID, and
target fingerprint; independent proof resolves all rows rather than merely
recomputing each record in isolation.

Known external exact refs are not exempt from currentness. The runtime fixture's
17-entry `external_reference_bindings` table covers every definition, profile,
policy, trigger, and published-schema ref in the fourteen admitted records.
Each row resolves to the current closure producer, and every corresponding
in-record fingerprint equals that producer when the retained record shape has a
fingerprint field. Self-consistent records with stale external fingerprints
refuse before append, compare-and-write, challenge construction, or mutation.

| Record class | Audit-only excluded timestamp | Semantic timestamp |
|---|---|---|
| intake | `finalized_at_utc` | none |
| candidate | none | none |
| approval | none | `decided_at_utc` |
| promotion | none | none |
| waiver | none | `expires_at_utc`, `review_at_utc` |
| registry state | none | none |
| registry transition | `occurred_at_utc` | none |
| authenticator make-credential response | none | counter and credential material are semantic |
| authenticator registration | `registered_at_utc` | none |
| authenticator get-assertion response | none | status, descriptor, flags, counter, authenticator data, and signature are semantic |
| authenticator assertion | `verified_at_utc` | counter and nonce/challenge are semantic |
| lifecycle observation | none | `observed_at_utc` |
| lifecycle transition | `transitioned_at_utc` | none |
| trigger evidence | none | `observed_at_utc` |

[`runtime-record-fingerprint-vectors-v1.0.json`](contracts/runtime-record-fingerprint-vectors-v1.0.json)
freezes one complete admitted record, preimage, expected fingerprint, and
expected record/path ID per row. The seven rows governed by published runtime
schemas must validate against those schemas; the remaining rows must equal the
exact retained `05` base plus only the additive HCM-2.2 fields above. The
implementation must reproduce every vector independently and reject an
incomplete/unknown-field record, a producer that excludes a semantic timestamp,
or one that includes an audit-only timestamp. The admitted authenticator-
assertion row uses the exact active bootstrap credential hash present in its
registry-state row and a complete DER-encoded ES256 signature over the frozen
`authenticatorData || clientDataHash` preimage. Independent proof must resolve
that credential from the registry, decode its COSE P-256 public key, and
cryptographically verify the signature; length/shape or fingerprint checks are
not substitutes.

Promotion retains one no-follow target observation, compares the mandatory
amendment fingerprint or explicit create-only null, and uses the exact promotion
journal below.

### Promotion transaction protocol

All selected Charter readers enter the repository-local promotion lock before
reading canonical bytes or promotion lineage and run recovery before returning
data. A promotion writer enters promotion, registry, and lifecycle locks in the
global order above, retains the exact registry pair used by approvals, and
holds all three through the authority commit and finalization. The writer uses
one filesystem beneath
`.handbook`, rejects cross-device paths, symlinks, and non-regular state, and
executes the exact `W0`-`W15` state machine above. Its operational expansion is:

1. retain the complete self-fingerprinted JCS+LF intent bytes, create-new
   `.handbook/state/transactions/promotions/.intent-staging/<32hex>.intent`,
   write only that non-authoritative scratch file, fsync and close it, bounded/
   no-follow reopen and verify its exact bytes/fingerprint/schema/bindings, then
   fsync `.intent-staging/`. The intent binds target path, create-only null or
   mandatory exact old fingerprint, candidate/profile/definition/approval
   fingerprints, new canonical fingerprint, final promotion-record and
   lifecycle-transition paths/fingerprints, and transaction ID;
2. create and fsync the exact empty
   `.handbook/state/transactions/promotions/<transaction_id>.pending/` directory
   with create-new/no-follow semantics, atomically rename-no-replace the
   complete verified intent scratch directly to `intent.json`, fsync the
   pending directory, `.intent-staging/`, and transaction parent in that order,
   then bounded/no-follow reverify exact `intent.json`. The pending namespace
   contains no `intent.tmp` and never exposes partial intent bytes;
3. if replacing truth, copy the retained old bytes to `canonical.old`, fsync,
   and verify the old fingerprint; creation records explicit absence;
4. for each of canonical, promotion record, and lifecycle transition in that
   order, retain the exact complete engine-owned output bytes, create one
   independent purpose-typed 128-bit random create-new file beneath the sibling
   `.output-staging/`, write only that scratch file, fsync and close it, bounded/
   no-follow reopen it, prove exact byte equality plus the intent-bound document
   hash/length and type-specific fingerprint/schema/ref/currentness bindings,
   fsync the scratch directory, atomically rename-no-replace it to
   `canonical.new`, `promotion-record.new`, or `lifecycle-transition.new`, fsync
   the pending, scratch, and transaction-parent directories in that order, then
   bounded/no-follow reverify the pending stage as exact. Before the rename the
   pending stage is absent; after the rename it is exact. No partial pending
   new-output stage is writer-reachable;
5. publish the exact `prepared` marker by its frozen temporary-marker grammar;
6. atomically rename `canonical.new` over the selected target, fsync the target
   parent, then publish exact `canonical-installed` by the same marker grammar;
7. install `promotion-record.new` and `lifecycle-transition.new` at their
   content-addressed final paths with create-new/no-follow semantics and fsync
   both parents. If a final path already exists with exact intended bytes,
   reuse it without assigning transaction ownership; if bytes differ, refuse and
   retain the journal. Then publish exact `records-installed` by the marker
   grammar;
8. create-new `committed.tmp`, write exactly the 72 ASCII bytes
   `sha256:<64 lowercase hex>\n`, where the hex is SHA-256 of the exact raw
   `intent.json` bytes, fsync it, atomically rename it to `committed`, and fsync
   the pending directory; the
   durable `committed` rename is the authority commit point; and
9. verify target, records, markers, and fingerprints, rename the directory from
   `.pending` to `.committed`, fsync its parent, release the lock, and report
   success.

Before the `committed` marker, new canonical bytes and physically installed
records are never product-visible because every selected reader takes the lock,
recovers first, and includes records/index entries only through a matching
committed journal. The authority guarantee is therefore all-or-neither
*visibility*, not impossible multi-file physical atomicity.

Recovery uses only the exact ordered `R0`-`R9` rollback machine and the exact
suffix-disjoint recovery partition above; this section defines no alternate
coarser table. It ignores progress-marker optimism and derives state from the
closed validated `intent.json` plus exact observed authoritative bytes. An
exact empty pending directory is the sole pre-intent state and is deleted; an
`intent.tmp`, partial `intent.json`, other name, or any other state without a
complete valid `intent.json` refuses and preserves evidence. With a valid
intent, recovery validates the transaction ID, all safe paths, intended
fingerprints, old-target snapshot or create-only absence, and commit-marker
payload. It independently classifies `canonical.old`, every pending new-output
stage, every final record, every marker temporary/published pair, and the
target. New-output pending stages are only absent or exact in the writer
closure; any observed partial or otherwise mismatched stage is preserved and
refuses. `.intent-staging/` and `.output-staging/` are never scanned,
classified, deleted, or used to derive authority. Partial diagnostic markers
and `committed.tmp` are not authority and are removed only by the selected
exact rollback/roll-forward state. Any unowned/mismatched byte refuses.

The exact recovery partition covers crashes before and after every atomic
new-output publication, after canonical rename but before
`canonical-installed`, after either record install, and after commit but before
directory rename. Rollback never deletes a final content-addressed record,
whether created by this transaction or already present equal. Authority indexes
make every uncommitted record invisible. Scratch/orphan garbage collection is a
separate non-authoritative operation outside this slice; recovery does not
perform it. Roll-forward never overwrites a final record. Cleanup is idempotent;
missing already-cleaned transaction-owned files are success only when all
remaining observed bytes match the selected predicate. A matching committed
journal must match target and both final records before any reader returns.
Derived promotion/lifecycle indexes include only matching committed journals;
either final record alone is never authority.

Crash injection is required after each `committed` and `rolled-back` marker
rename before its original pending-directory fsync, after that original fsync,
after recovery's replayed pending-directory fsync, and after the following
terminal-payload revalidation. Every retry must replay the pending-directory
fsync before terminal rename and converge to the exact immutable terminal set.
Crash injection is also required at every frozen `S0`-`S11` boundary for each of the
three output purposes, including `S0` before scratch creation, every bounded
scratch-write prefix fixture, and every
remaining create/write/fsync/rename/marker step, plus concurrent readers/
writers and replay from every state. On native Windows,
mutation refuses before creating the transaction directory unless the same
no-follow, atomic-replace, durable-flush, lock, directory-flush, and recovery
contract is proven; read-only validate remains supported. No weaker fallback
writer exists.

## Deterministic canonical bytes, renderer, and lifecycle

### Closed canonical YAML emitter

Canonical mapping order is not inferred from prose or a serializer. These are
the exact ordered key lists and they match the literal fixture:

```text
root = schema_id, schema_version, record_id, project, posture, domains, policy, governance, engineering_posture, debt, decision_records
project = name, classification, team_size, users, expected_lifetime, surfaces, runtime_environments, constraints, operational_reality, default_implications
constraints = deadline, budget, experience_notes, must_use_tech
operational_reality = in_production_today, prod_users_or_data, external_contracts_to_preserve, uptime_expectations
default_implications = backward_compatibility, migration_planning, rollout_controls, deprecation_policy, observability_threshold
posture = rubric_scale, baseline_level, baseline_rationale
domain = name, blast_radius, touches, constraints
policy = revision, authority_statement
governance = decision_authority, required_approvals, exception_policy, exception_process, review_triggers, reassessment_triggers
exception_process = approvers, record_location, minimum_fields
engineering_posture = dimensions, red_lines
dimension = dimension_id, level_override, default_stance, raise_the_bar_triggers, allowed_shortcuts, red_lines, domain_overrides
debt = system, labels, review_cadence
decision_records = enabled, path, format
```

The emitter uses two-space indentation and this recursive grammar:

- an object emits each required key once as `indent + key + ":"`; scalar
  values add one space plus the scalar and LF; object/sequence values add LF
  followed by their children at `indent + 2`;
- a non-empty sequence emits each scalar as `indent + "- " + scalar + LF`;
  each object item emits its first key after `- ` and remaining keys at two
  spaces deeper; an empty sequence emits `[]` on the parent key line;
- strings are always JSON double-quoted; escape `"`, `\`, backspace, form
  feed, LF, CR, and tab as `\"`, `\\`, `\b`, `\f`, `\n`, `\r`, and `\t`;
  encode every other U+0000-U+001F scalar as lowercase `\u00xx`; emit other
  Unicode directly as UTF-8 and never emit plain/single/literal/folded/tagged/
  anchored scalars;
- integers are unsigned base-10 with no sign/leading zero; booleans are
  `true`/`false`; null is literal `null` and occurs only for
  `level_override`;
- mapping keys are the fixed ASCII schema keys only; no key sorting, serializer
  choice, comment, document marker, blank line, tab, CR, trailing space, or
  alias is permitted; and
- output is UTF-8 with LF and exactly one final LF.

[`contracts/canonical-charter-boundary-v1.1.yaml`](contracts/canonical-charter-boundary-v1.1.yaml)
is a literal complete emitted-byte fixture. Its raw bytes and SHA-256 are
planning authority: 5,405 bytes and
`sha256:36d607d2aa8dd5b433ce79fcb106ff2fa2a2fe291509b4ec874d349866226e44`.
Implementation must reproduce them without normalizing the fixture, and a
second independent test emitter must reproduce the production emitter for the
complete empty/non-empty/control/Unicode/integer/boolean/null matrix.

### Closed Markdown renderer

`handbook.renderer.charter-review-markdown@1.0.0` accepts only a structurally
and semantically valid typed `1.1` Charter. Its output media type is
`text/markdown`; it has no Resolution input and is not a capitalized
Projection. It emits exactly these headings in order:

```text
# Engineering Charter — <project.name>
## What this is
## How to use this charter
## Rubric: 1–5 rigor levels
## Project baseline posture
## Domains / areas (optional overrides)
## Posture at a glance (quick scan)
## Dimensions (details + guardrails)
## Cross-cutting red lines (global non-negotiables)
## Constitutional policy and governance
## Exceptions / overrides process
## Debt tracking expectations
## Decision Records (ADRs): how to use this charter
## Review & updates
```

The rubric table is the exact five-level table retained from the current
renderer. Project baseline emits every `project`, `posture`, constraint,
operational, and default-implication field in schema order. Domains preserve
array order and emit name, blast radius, touches, constraints, and the baseline
sentence; exact empty domains emit `None — baseline applies everywhere.`. The
quick-scan table and detail sections iterate the exact nine dimension order;
effective level is override-or-baseline. Each detail emits, in order, stance,
raise triggers, allowed shortcuts, per-dimension red lines, and domain
overrides; exact empty domain overrides emit `- None — baseline applies.`.
Cross-cutting red lines preserve their canonical order. Policy/governance emits
revision, authority statement, decision authority, required approvals,
exception policy, review triggers, and reassessment triggers in that order.
Exceptions, debt, and decision-record sections preserve their schema field
orders. Review emits review cadence and both fixed current change sentences.

All free text passes the retained whitespace normalization and Markdown-control
refusal. Inline lists join with `, `; block lists use `- `; nested lists use
`  - `; empty allowed inline lists use the exact current defaults `none`,
`none declared`, or the domain sentence named above. Table cells additionally
escape backslash, pipe, CR, and LF with the HCM-2.1 exhaustive rule. Headings,
labels, punctuation, blank lines, table separators, and fixed prose are literal,
not locale/config inputs. Output uses LF and exactly one final LF.

[`contracts/charter-review-boundary-v1.0.md`](contracts/charter-review-boundary-v1.0.md)
is the complete literal rendering of the canonical boundary fixture and is
planning authority: 8,079 bytes and
`sha256:4bebff7b716e624a955637f9462e65f4b42b6857b31b0b2add8809034c7b1061`.
Production and independent reference renderers must reproduce it byte-for-byte
and cover exact empty/boundary/control/Unicode/table cases. Source fingerprint
hashes exact observed/emitted canonical YAML bytes; render fingerprint hashes
exact Markdown bytes. Neither includes refs, timestamps, paths, or the other
fingerprint. No rendered file is persisted.

`handbook.lifecycle.constitutional-review-lock@1.0.0` keeps canonical status
stable until an authorized promotion. Before initial promotion there is no
lifecycle state. The first committed promotion establishes `current`.

The exact review trigger
`handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0` observes a
finalized candidate whose basis matches current canonical truth and whose bytes
change any leaf under `/policy`, `/governance`, or `/engineering_posture`; it
produces `review_required`. The two intake triggers accept only exact
`handbook.schemas.lifecycle.trigger-evidence@1.0.0` records with instance ID,
basis canonical fingerprint, event kind, prior/current evidence refs and
fingerprints, non-equal prior/current fingerprints, UTC observation time, and
evidence fingerprint. Production-posture evidence produces the one-row reopen
mapping above; trust-boundary evidence produces its one-row mapping. Unknown,
same-fingerprint, wrong-instance, stale-basis, missing-producer, or unsupported
trigger evidence refuses.

The total state/event table is:

| Prior state | Review event | Reassessment event | Successful addressing promotion |
|---|---|---|---|
| `current` | `review_required` | `reassessment_required` | `current` |
| `review_required` | `review_required` | `reassessment_required` | `current` only if every active review observation is addressed |
| `reassessment_required` | `reassessment_required` with review observation retained | `reassessment_required` | `current` only if every active review/reassessment observation and reopened coverage ID is addressed |

Reassessment has precedence over review; events received together sort first by
that precedence and then event fingerprint. A duplicate event fingerprint is
idempotent and creates no transition. A different event always appends an
observation even when the visible state is unchanged. Replayed/stale prior-state
fingerprints refuse. A promotion that does not address all active observations
may update no truth and cannot clear state.

Lifecycle observation records are closed
`handbook.lifecycle-observation@1.0` values with exact fields
`observation_id`, lifecycle policy ref/fingerprint, instance ID, basis canonical
fingerprint, trigger ref/fingerprint, evidence ref/fingerprint, event kind,
observed UTC, and observation fingerprint. Transition records are closed
`handbook.lifecycle-transition@1.0` values with exact fields `transition_id`,
policy ref/fingerprint, instance ID, prior state/fingerprint, ordered new
observation refs, ordered active observation refs, result state/fingerprint,
nullable clearance promotion ref, transitioned UTC, and transition fingerprint.
State fingerprint input is policy ref/fingerprint, instance ID, current
canonical fingerprint, result state, and active observation fingerprints in
the exact current committed-head order. All record fingerprints use one JCS value and exclude only their
own fingerprint except for the class-specific audit fields frozen in the
runtime fingerprint matrix above.

Standalone lifecycle events use one lifecycle-event lock and a pending journal;
physical observation/transition files are never authority by themselves. Every
lifecycle writer acquires the lock, runs recovery for every pending journal, and
may create no new intent until recovery completes. It then:

1. create-news `.handbook/state/transactions/lifecycle-events/<event-id>.pending/`;
2. writes+fsyncs `intent.tmp`, renames it to `intent.json`, and fsyncs the
   directory. Intent binds transaction/event IDs, current committed lifecycle
   head/state fingerprint, trigger evidence, exact final observation/transition
   paths/fingerprints, and staged-byte fingerprints;
3. writes+fsyncs `observation.new` and `transition.new`, verifies both, and
   fsyncs the directory;
4. installs both final records create-new or exact-equal, fsyncs both parents,
   writes+fsyncs `records-installed`, then writes exactly 72 ASCII bytes to
   `committed.tmp`: `sha256:<64 lowercase hex>\n`, where the hex is SHA-256 of
   the exact raw `intent.json` bytes. It fsyncs `committed.tmp`, renames it to
   `committed`, and fsyncs the pending directory. That rename is
   the sole authority-visible point; and
5. verifies all bound bytes, renames `.pending` to `.committed`, fsyncs the
   transaction parent, and releases the lock.

For this protocol, a matching marker is byte-for-byte equality with that
recomputed 72-byte payload. Missing LF, an extra byte, uppercase hex,
truncation, or a hash of parsed rather than raw intent bytes is mismatching and
refuses.

Readers take the lifecycle lock, recover first, and derive head/state only from
matching committed journals. Recovery ignores diagnostic-marker optimism,
validates safe paths and exact intent/staged/final/marker bytes, and applies this
precedence table:

| Observed state under lifecycle lock | Commit marker | Required recovery |
|---|---|---|
| empty directory or owned partial `intent.tmp` only | absent | delete owned partial state |
| valid intent; no final records; both staged files exact | absent | install both finals and roll forward commit |
| valid intent; no final records; either staged file absent/partial | absent | delete staged/journal state; caller may resubmit the retained trigger evidence |
| valid intent; one exact final and the other exact staged file | absent | install the missing final and roll forward commit |
| valid intent; one exact final and missing/partial other staged file | absent | leave final as invisible orphan and delete staged/journal state |
| valid intent; both exact finals | absent | revalidate trigger/current definitions and roll forward commit |
| valid intent; both exact finals | matching | finalize pending to committed |
| valid intent; either final missing | matching | refuse durability-contract violation and preserve evidence |
| valid intent whose bound prior head/state is not current | either | refuse durability-contract violation and preserve evidence; an admitted writer cannot advance the head because it must recover this journal first |
| unsafe/mismatched path, intent, staged/final byte, or marker | either | refuse and preserve evidence |

Final content-addressed records are never deleted. Cleanup is idempotent and
only owned staged/journal bytes may be removed. Crash injection covers every
create/write/fsync/rename boundary, every table row, concurrent readers, and
competing events.
Promotion clearance advances the same lifecycle head inside the promotion
journal rather than creating a second lifecycle-event journal. All writers use
the global promotion-then-registry-then-lifecycle lock order; a standalone event
takes only the lifecycle-event lock and never acquires registry or promotion.
The promotion intent binds the retained registry state/head pair, the prior
lifecycle head/state and its clearance transition, and its `committed` marker
is the authority-visible point for canonical, promotion, and lifecycle state
together. A standalone event seeing promotion's new head must replay or remain
invisible; it can never overwrite or fork clearance.

Clearance requires the candidate's sole current lifecycle-validation result,
and the promotion repeats the Option 1 currentness checks above. That result
references every active observation, proves the candidate intake lineage
reevaluated every reopened coverage ID, and binds the
`candidate_subject_fingerprint`—never the final candidate fingerprint—plus the
current/policy/state fingerprints. The final candidate and promotion name the
validation result; the transition names the promotion ID; and intent `1.2`
binds their fingerprints without a cyclic record dependency. Recovery installs
both final records before the commit marker. Neither an observation, transition,
renderer output, nor state changes the artifact, posture kernel, approval, or
policy revision by itself.

## Product-path cutover

The implementation cuts one coherent vertical path:

- **engine:** new definition registries, typed canonical Charter, intake and
  lineage records, evaluation, lifecycle, promotion transaction, renderer,
  repository invocation identity, and retained-observation inspection;
- **compiler/CLI:** thin author/approve/promote/validate operations, selected
  profile resolution, typed JSON/human results, setup/doctor inspection, and
  no hidden inference or approval;
- **skill/templates:** guided-adaptive, express, and agent-assisted workflows
  collect inputs/evidence and invoke stable operations; no generated skill
  edits canonical YAML or drives a nested CLI conversation;
- **Environment Inventory:** replace the fixed Markdown Charter reference with
  the exact selected Charter descriptor path and refuse missing/invalid/stale
  constitutional truth, without converting Environment Inventory content;
- **flow:** expand selected structured sourcing to Charter under a separately
  named `BR-HCM-2-CHARTER-FLOW-01`, render it in memory, expose source/rendered
  fingerprints, and remove every legacy Charter Markdown influence;
- **setup/doctor:** setup remains non-authoring while create-new initializing or
  byte-preserving the engine-owned repository identity; doctor advances its
  schema additively and reports identity readiness, definition closure,
  canonical validity, lifecycle state, source/render fingerprints, and typed
  next actions from retained engine observations; and
- **legacy removal:** delete fixed Charter Markdown constants/validators,
  legacy `CharterStructuredInput` selected callers, old author templates, and
  old selected flow/CLI paths only after all named consumers move.

The C03 freshness schema remains `reduced-v1-m8`, generation `1`, only if fixed
goldens prove its source-only preimage contract is byte-identical for unchanged
siblings and its now-owned Charter path/source fingerprint fits the existing
generic encoding. The changed flow/compiler C04 envelope advances together to
`reduced-v1-m8.3`; `.3` is accepted and `.2` rejected at the compiler boundary.
The new Charter bridge is deleted no later than HCM-2.4. HCM-2.1's Project
Context bridge remains separately bounded.

Pipeline/stage/external pipeline consumer code is not a second authoring path.
If a live selected product path still emits or requires Charter Markdown, the
implementation must remove or cut that exact path in this slice; if doing so
requires redesign outside the packet's named engine/compiler/flow/CLI/skill/
template/fixture boundary, stop for scope authority rather than add a dual
truth bridge.

## API, security, and compatibility posture

- Definition, intake, candidate, approval, promotion, lifecycle, render, and
  refusal DTOs are engine-owned typed values; adapters do not reproduce
  semantic checks.
- Public operations return exact schema/version envelopes, including the
  pre-invocation refusal union where a normal request/result identity cannot
  exist and the distinct authority-recovery refusal where mandatory recovery
  cannot complete, stable reason codes,
  current refs/fingerprints, changed paths, and next actions. Human wording is
  consumer-owned.
- No remote schema/definition fetch, executable definition hook, dynamic CLI
  subcommand, prompt callback, or arbitrary renderer process is authorized.
- Reads and writes remain bounded beneath the trusted repository root, reject
  traversal and unsafe file types, and retain observations through
  decode/render/hash/compare-and-write.
- Intake evidence refs are references, not unrestricted content capture;
  secret/path/size controls apply before persistence and JSON output.
- Old `1.0.0` definitions remain resolvable for exact historical validation but
  never act as selected fallback. Old Markdown and `0.1.0` input have no
  compatibility promise.
- On native non-Unix targets, validate/read-only operations remain available;
  every mutating semantic-record or promotion operation must either satisfy an
  equally strong atomic/no-follow contract or refuse before filesystem delta.

## Exact scope

Implementation may change only the packet, affected control-pack rows, exact
engine/compiler/flow/CLI source and tests needed for the boundary above,
definition assets/registries/package manifests, core generated skill/template
inputs and installed-package equality fixtures, and exact Environment Inventory
reference carriers. Every existing symbol requires refreshed GitNexus upstream
impact before edit.

The known `resolve_shipped_profile_decisions` seam is `CRITICAL`: 90 upstream
symbols, 33 direct callers, and five processes at planning time. The future
implementation must warn before editing it, add RED profile-version/old-byte/
consumer tests first, keep the change additive, and replay setup, doctor,
author, flow, environment-inventory preflight, package, and CLI suites before
proceeding. Any wider process impact stops the slice.

Explicit non-goals:

- HCM-2.3 custom-kind intake proof or arbitrary repository renderer execution;
- remaining shipped-family YAML conversion or HCM-2.4 bridge deletion;
- Phase-3 Context Resolution Projections, Snapshot Memory, or posture-kernel
  implementation;
- migration/import of old Charter files or a compatibility/dual-read profile;
- setup authoring, auto-approval, auto-promotion, or silent default policy;
- a nested CLI wizard or an LLM call inside Handbook;
- persistent Markdown, editable renderer output, or a second posture authority;
- public SDK/Tauri/Substrate/dock/publication work; and
- unrelated pipeline, contract, stage, terminology, or documentation cleanup.

## Required skill chain

Every implementation/editing run must load, in order:

1. `using-agent-skills`;
2. `context-engineering`;
3. `source-driven-development`;
4. `spec-driven-development`;
5. `api-and-interface-design`;
6. `security-and-hardening`;
7. `test-driven-development`;
8. `incremental-implementation`;
9. `debugging-and-error-recovery` when a gate fails;
10. `documentation-and-adrs`;
11. `code-review-and-quality`; and
12. `git-workflow-and-versioning`.

Fresh review agents load the same chain except implementation-only mutation
steps and remain read-only.

## Verification and promotion gates

The implementation proof wall includes:

1. exact old/new definition tree, raw bytes, fingerprints, cross-ref closure,
   package archive, and installed-package equality;
2. structural/semantic Charter positives plus duplicate/multi-doc/unknown/
   bounds/capability/ref/fingerprint negative tests;
3. all three modes producing byte-identical candidate content and equivalent
   coverage for identical inputs;
4. provenance/source-kind, known-unknown, contradiction, waiver, confidence,
   specificity, and missing-required-coverage refusal matrices;
5. immutable lineage, wrong actor/rejected approval, stale candidate/profile/
   definition/target, ABA, replay, atomic promotion, and crash fault injection;
6. deterministic full-byte canonical YAML and Markdown render goldens with
   independent exact source/render fingerprints;
7. lifecycle targeted-reopen, unknown-trigger, no-auto-regeneration, and
   authorized-clearing proof;
8. author/approve/promote/validate, setup/doctor, Environment Inventory, flow,
   CLI JSON/human, generated skill, install-smoke, conflicting-legacy-file, and
   repository-identity/operation-ID derivation and refusal real-path proof;
9. native Windows read-only and fail-before-mutation proof;
10. HCM-1.1-HCM-1.4 and HCM-2.1 regressions, full workspace tests, fmt, Clippy,
    docs, package, handoff, archive, link, secret, scope, and diff gates; and
11. fresh isolated review/remediation/different-fresh re-review until `CLEAN`
    over the exact final subject.

Evidence may close `PG-INTAKE-01`, `PG-INTAKE-02`, and `PG-CHARTER-01` only for
the exact first-party Charter. It may extend `PG-YAML-02` only to the converted
Project Context and Charter families. `PG-KIND-01`, `PG-ARTIFACT-01`, and
program-wide `PG-YAML-02` remain open at their named remaining gaps.

## Stop conditions

Stop without claiming completion if:

- old exact-definition bytes must change rather than version;
- canonical schema, capability bindings, intake targets, or profile selection
  cannot form one closed exact-ref graph;
- required normative coverage can enter canonical truth through inference,
  default, waiver, agent output, or unapproved candidate state;
- promotion cannot provide one durable authority-visibility commit across
  canonical truth, promotion record, and lifecycle transition;
- a selected product path still reads/writes legacy Charter Markdown;
- critical resolver impact widens outside the packet or cannot be proven;
- required native-platform, installed-skill, real-flow, or immutable-lineage
  proof is unavailable; or
- mandatory fresh built-in review cannot complete.

The future implementation ends after its reviewed primary commit and separate
parent-owned handoff/ledger closeout. It must not start HCM-2.3.
