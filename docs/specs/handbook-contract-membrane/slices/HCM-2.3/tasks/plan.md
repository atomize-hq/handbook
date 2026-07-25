# HCM-2.3 Test-First Implementation Plan

## Planning authority

This plan implements only the reviewed HCM-2.3 specification after a separate
explicit implementation selector. It does not authorize itself and it does not
authorize work during the planning session that created it. The later parent
must re-read the live packet, control pack, selected planning-completed handoff,
AGENTS instructions, and required skills before touching code.

HCM-2.2 primary commit `6766d3ed4894aad6598faaa7c4a54b493f92c1f6`
and closeout commit `5c31eeefb5adf71d75ec3059b1b6947025d2fd6b`
are immutable dependencies. The implementation branch must begin from the
exact planning closeout commit named by the future handoff, not from this
document's preparation baseline.

## Outcome and dependency graph

The implementation delivers one repository-defined `registry-brief` custom
kind through one generic CLI/engine path:

```text
safe explicit repository source bindings
  -> exact schema/kind/profile/instance/intake registries
    -> artifact operation context
      -> descriptor-selected canonical YAML read/validation
      -> supplied intake evaluation
        -> immutable intake + candidate transaction
          -> non-governed compare-and-write promotion
            -> actual CLI restart/replay/concurrency proof
```

The order is intentionally bottom-up. Source/identity and descriptor/intake
closure must exist before generic operations; pure evaluation must exist before
persistence; persistence/recovery must exist before CLI mutation; the binary
real path comes only after owner behavior is independently proven.

## Required skill chain

Apply the live skill files as their phases begin:

1. `using-agent-skills`;
2. `context-engineering`;
3. `source-driven-development`;
4. `spec-driven-development`;
5. `planning-and-task-breakdown`;
6. `api-and-interface-design` before public request/result types;
7. `security-and-hardening` before source and persistence changes;
8. `test-driven-development` and `incremental-implementation` for every RED;
9. `documentation-and-adrs` before changing canonical status/classification;
10. `code-review-and-quality` before the proof wall; and
11. `git-workflow-and-versioning` before staging or commits.

If an applicable skill adds a stronger stop or verification gate, the stronger
gate wins. The implementation parent records durable skill evidence rather
than merely naming the skills.

## Entry gate and live inventory

Before any edit:

1. verify the exact branch, planning closeout HEAD, clean staged/unstaged/
   untracked state, ancestry through both HCM-2.2 commits and both HCM-2.3
   planning commits, and the planning handoff selector;
2. run all three handoff validation modes and archive-boundary self-test/normal
   mode;
3. verify all preservation locations named by the planning handoff have the
   same metadata and sentinel hashes recorded at planning closeout;
4. refresh GitNexus if stale, query the live artifact/profile/intake/CLI call
   paths, and capture `context` plus upstream `impact` for every existing symbol
   that may change;
5. enumerate current registry/profile/inspection/Charter/generic CLI tests and
   exact package definition members; and
6. record a bounded implementation capsule with allowed paths, proof gates,
   non-goals, known CRITICAL surfaces, and stop conditions.

Planning-time graph results are evidence, not a substitute for fresh analysis:

| Existing symbol | Planning result | Expected disposition |
|---|---:|---|
| `load_artifact_kind_registry` | CRITICAL, 385 impacted | consume unchanged |
| `load_artifact_kind_registry_admitted` | CRITICAL, 357 impacted | consume unchanged |
| `SchemaRegistry::load_with_request_budget` | CRITICAL, 7 impacted | consume unchanged |
| `ArtifactInstanceRegistry::resolve` | CRITICAL, 427 impacted | one narrow intake-ref admission change after warning |
| `resolve_profile_selection` | CRITICAL, 458 impacted | consume unchanged |
| `ResolvedArtifactRegistry::validate_json` | CRITICAL, 351 impacted | consume unchanged |

Warn before the anticipated `ArtifactInstanceRegistry::resolve` edit. If live
impact adds another HIGH/CRITICAL edit, touches new processes/modules, or makes
the narrow change unsafe, stop for explicit review. Do not list or authorize a
HIGH/CRITICAL symbol that was not live-analyzed.

## Standing increment rule

Every behavior increment follows:

1. add the smallest focused RED test and capture the intended failure;
2. implement only enough owner behavior for GREEN;
3. refactor without broadening scope;
4. run the focused test, affected crate tests, and format check;
5. inspect changed paths and `git diff --check`; and
6. update the durable proof log before starting the next increment.

No task is complete on compile-only proof. Existing definitions, tests, and
product paths must stay green at every checkpoint. Do not stage or commit an
intermediate increment.

## Increment 0 — Freeze baseline and exact fixture manifest

**Purpose:** fail fast on a wrong worktree, stale graph, or changed dependency.

**RED/proof first:** add no behavior. Capture literal HCM-1.3 fixture inputs,
HCM-2.2 definition/record identities, package definition members, existing CLI
grammar, and planning-time preservation baselines. Assert no HCM-2.3 example
kind is present in package-owned `crates/engine/definitions`.

**Expected file areas:** proof log only initially; no production file.

**Acceptance and proof:** exact hashes/manifests replay; GitNexus contexts and
fresh impacts are recorded; worktree is still clean.

**Stop:** any dependency byte/identity/branch mismatch, dirty unknown path,
stale graph that cannot be refreshed, or preservation drift.

## Increment 1 — RED repository selection and intake-source admission

**Purpose:** freeze the new effect-binding record and all source attacks before
adding a loader.

**RED tests:** closed `.handbook/profile-selection.json`; missing/unknown/
duplicate fields; exact-ref/path mismatch; duplicate/conflicting intake source;
absolute/escape/backslash/symlink/non-regular/oversize/over-total/over-count/
mutation; every required source-class field including empty arrays; built-in and
repository-path typed mapping; reordered-array equivalence; no cwd/ancestor/Git/
env scan; missing fixed record. Validate every positive record against
`repository-profile-selection-1.0.0.schema.json` and replay its normative
fingerprint/vector before production work. Require the exact shipped-plus-
custom source counts and selected-profile replacement members from that vector;
same-`exact_ref`/different-source duplicates, cross-definition-class rebinding,
and schema-valid but semantically incomplete selections refuse. Execute the
complete typed resolver closure: JSON-Schema acceptance alone is not admission.
Execute the normative schema mutations for one-segment, underscore, repeated-
hyphen, noncanonical-SemVer, dot-component, trailing-slash, 65-component,
1,025-byte, and final-line-terminator exact refs/paths and require schema
rejection before owner code.

**Expected file areas:**

- new focused engine integration test;
- first subset of `tests/fixtures/hcm_2_3_generic_custom_kind/` selection/profile
  inputs; and
- no production changes.

**Acceptance and proof:** every test is demonstrably RED for the intended
missing owner behavior, not because fixture JSON is malformed.

**Stop:** the only viable source path requires ambient discovery, arbitrary
schema bytes, an invocation-time profile override, or a new shipped definition.

## Increment 2 — GREEN intake registry and operation selection context

**Purpose:** admit exact repository `ArtifactIntakeDefinition` sources and bind
them to an unchanged resolved profile/kind registry.

**Implementation:** add focused engine modules for the intake registry and
artifact operation selection/context. Reuse the uniform definition identity,
safe source reader, byte budget, typed exact refs, and existing schema/kind
registries. Do not duplicate source or schema validation. Keep the existing
resolved-profile fingerprint algorithm unchanged; derive the additive operation-
context fingerprint specified by `SPEC.md`.

**Expected file areas:**

- new `crates/engine/src/artifact_intake_registry.rs`;
- new `crates/engine/src/artifact_operation_context.rs`;
- narrow `crates/engine/src/lib.rs` declarations/re-exports; and
- Increment 1 test/fixture additions.

**Acceptance and proof:** source permutations produce identical intake registry
and operation-context fingerprints; all negative source tests return typed
stable refusals; no network/resolver feature or dependency enters the graph.

**Stop:** implementation requires modifying kind/schema loaders,
`resolve_profile_selection`, Cargo files, fingerprint algorithms, or accepting
an unresolved intake before content use.

## Increment 3 — RED/GREEN descriptor-selected optional intake

**Purpose:** permit exactly a generic intake ref on non-Charter descriptors
while preserving every previously frozen dependency refusal.

**RED tests:** new `example.profile.registry-root@1.1.0` selects
`example.intake.registry-brief@1.0.0`; null intake remains accepted/not-
applicable; missing/wrong-kind/wrong-schema intake refuses at operation-context
closure; non-Charter lifecycle/renderer/Projection/overlay refs still refuse;
the exact Project Authority `1.1` closure stays byte- and behavior-identical.

**Impact gate:** rerun upstream impact on
`ArtifactInstanceRegistry::resolve`, warn with the live result, and inspect all
affected processes before editing.

**Expected file areas:**

- narrow `crates/engine/src/artifact_instance.rs` change;
- focused unit/integration tests;
- remaining profile/kind/intake fixture sources; and
- no HCM-2.2 source or asset edit.

**Acceptance and proof:** the only relaxed later-owned field is
`intake_definition_ref`; the new operation context closes it before use; all
HCM-1.2/HCM-1.3/HCM-2.2 descriptor tests pass.

**Stop:** the change must relax any other dependency, special-case the example
kind, or alter the frozen Charter branch.

### Checkpoint A — Registration closure

- focused selection/intake/descriptor tests pass;
- full `handbook-engine` tests pass;
- HCM-1.1/HCM-1.3 fixture equality and package-default non-membership pass;
- format, diff-check, scope, and graph inspection are clean.

## Increment 4 — RED layered generic canonical validation

**Purpose:** freeze one generic read/validate path before exposing operations.

**RED tests:** select canonical bytes only from the descriptor path; assert kind
and instance request equality; YAML duplicate/multi-document/syntax/object/size
failures; safe-path and retained-observation mutation; exact structural errors;
semantic layer is explicitly not-applicable; intake/approval/external layers do
not waive earlier failure. Replay the runtime-schema exact-ref, safe-ref,
digest/record-ID, and final-line-terminator rejection vectors at the same
identity/path boundaries before service behavior.

**Expected file areas:** focused engine test plus HCM-2.3 valid/invalid canonical
fixtures. No production change yet.

**Acceptance and proof:** failures demonstrate the current absence of a generic
operation service rather than bypassing the registry.

## Increment 5 — GREEN read-only generic artifact operations

**Purpose:** implement `artifact.kind.list`, `artifact.instance.list`,
`artifact.read`, `artifact.validate`, and `intake.definition.read` in one typed
engine owner service.

**Implementation:** consume `ResolvedArtifactRegistry` and its validator
unchanged; retain no-follow observations; return closed typed layered results;
require exact request kind/instance match; branch only on stable operation, not
kind or filename.

**Expected file areas:**

- new `crates/engine/src/artifact_operations.rs`;
- narrow `lib.rs` export;
- Increment 4 tests; and
- no CLI file.

**Acceptance and proof:** valid `registry_brief` canonical YAML passes through
the exact registered schema; invalid bytes return deterministic structural
locations; null intake returns typed not-applicable; a test scans/guards against
kind-specific dispatch.

**Stop:** the service needs a per-kind adapter, generated operation, enum
variant, renderer, generic Projection, or duplicate validator.

## Increment 6 — RED/GREEN pure generic intake and candidate construction

**Purpose:** implement the exact two-row intake definition and deterministic
record previews without persistence.

**RED first:** all three modes; equal candidate content; ordered provenance;
missing/duplicate/unknown/contradicted/default/inferred/evidence/waiver/wrong-
source/low-confidence/low-specificity submissions; coverage target overlap or
schema mismatch; semantic absent; approval injection.

**Implementation:** add a pure evaluator behind `intake.coverage.evaluate`. It
returns coverage/content/field-source preview only: no timestamp, intake ID,
candidate ID, ref, key, journal, or I/O. Add a pure candidate-preview builder
that accepts an already finalized committed intake `1.2` value in tests,
constructs timestamp-free validation result `1.0`, candidate `1.4`, exact refs/
fingerprints, and performs no I/O. The live `artifact.candidate.validate`
operation is wired only after the committed store exists.

**Expected file areas:**

- new `crates/engine/src/artifact_intake.rs`;
- focused engine tests;
- narrow operation-service integration; and
- the mandatory runtime schema/vector fixtures from the planning packet.

**Acceptance and proof:** identical normalized submissions across modes produce
identical content preview but no record identity; the fixed finalized intake
vector reproduces exact intake `1.2`, validation `1.0`, candidate `1.4`, and
promotion `1.2` identities; every populated leaf has exactly one source; every
old version stays byte-identical and cross-version/store substitutions refuse;
Charter candidate/result/approval code is not called or imported.

**Stop:** generic behavior needs Charter constants, a hidden prompt/model call,
implicit default/approval, or a kind-specific branch.

## Increment 7 — RED/GREEN deterministic generic canonical YAML

**Purpose:** freeze generic JSON-to-canonical-YAML bytes before any write path.

**RED first:** mapping order, nested mappings/sequences, scalar ambiguity,
Unicode/control characters, LF/final-LF, no tags/anchors/directives/comments,
parse/emit/parse equality, unsupported/non-finite values, output size, and
source-order permutations.

**Implementation:** add one engine-owned deterministic JSON-data-model emitter.
It has no schema/kind awareness and never parses caller-supplied YAML as a
template.

**Expected file areas:**

- new `crates/engine/src/canonical_yaml.rs`;
- focused unit tests; and
- narrow export/use from artifact operations.

**Acceptance and proof:** golden bytes replay independently; emitted candidate
content revalidates through the selected registry schema.

**Stop:** emitter correctness requires a renderer, per-kind code, YAML tags/
anchors, a new dependency, or a semantic schema transformation.

### Checkpoint B — Pure operation kernel

- read/validate/intake/candidate/emitter focused tests pass;
- affected engine tests and all earlier membrane regressions pass;
- no persistent path has changed;
- graph and scope remain within the packet.

## Increment 8 — RED generic lineage transaction and recovery

**Purpose:** freeze separate intake-record append and atomic content/validation-
result/candidate visibility before store code.

**RED tests:** every write/fsync/close/reopen/install/marker crash point; every
installed subset of two intake values plus intake, all eight content/result/
candidate subsets, and all four canonical/promotion subsets; missing/malformed/
extra/duplicate/symlink/reparse/non-regular/hard-link-alias/mismatched/mutated
intent/staging/output/marker; exact inventory count/byte/depth boundaries; a
valid intake with no candidate; missing-intake candidate refusal; subordinate-
closure invisibility; same/different key/request races; retained-result expiry,
tombstones, recovery holds, currentness drift, repeated restart, and process
concurrency. Separately freeze the pre-establishment zero-write matrix and every
established-refusal prefix: atomic intent publication with its derived hold,
verified refusal decision,
refused marker, evidence, refused result, retained-result ledger, committed
rename, exact replay, and tombstone. Reject output-bearing refusal, null/missing
evidence, code/layer/nullability mismatch, cross-operation/transaction/request/
context binding, non-prefix crash state, and same-key different request.

**Expected file areas:** focused engine integration/fault-injection tests only.
Fault injection remains private to tests.

**Acceptance and proof:** the RED suite freezes the total recovery matrix and
does not pass through HCM-2.2 Charter transaction code accidentally.

## Increment 9 — GREEN generic lineage store

**Purpose:** implement transaction-owned immutable intake and content/validation-
result/candidate persistence.

**Implementation:** add a separate generic store implementing the exact runtime
contract: one coarse ordered lock, bounded lexical no-follow inventory, JCS+LF
records, subordinate value/content/validation closure, create-new immutable
writes, same-filesystem staging, closed intent/verified/marker records, recovery-
first visibility, the total complement table, and no authority deletion. Intake
and candidate append each realize exactly one authoritative semantic-record
output; closure refs live only in semantic records and internal transaction
evidence and never become independent write-set items or canonical receipts.
Wire `artifact.candidate.validate` to committed intake with zero writes, then
make candidate append independently reproduce the expected preview. Implement
the direct-CLI domain ledger with derived context outside key/request identity, lookup-before-
currentness, 30-day results, non-expiring tombstones, and recovery holds. It is
separate from the future canonical Phase 4 outer ledger, which must pass the
same domain key/request pair rather than derive a second inner namespace.
HCM-2.3 proves only that transport metadata cannot enter the owner derivation
API/preimage. Implement the exact establishment boundary: control failures
before exact context/evaluation write nothing; a deterministic commit or closed
refusal atomically publishes its one intent and derives the non-persisted hold.
Exercise every writing/established/pending intent crash transition and reject
dual or mismatched state. Persist established refusal through
the outputless verified/marker/evidence/result/ledger/rename chain, require exact
non-null evidence, resume every legal prefix, and retain/replay/tombstone it
without re-evaluation. Phase-4-first/direct, direct-first/Phase-4, outer-ledger state, and
inter-ledger crash execution fixtures are deferred to the Phase 4 owner.
Store fingerprints, never raw keys. Do not widen Charter schemas or paths.

**Expected file areas:**

- new `crates/engine/src/artifact_lineage_store.rs`;
- Increment 8 tests;
- narrow operation-service integration; and
- `lib.rs` export only if required by the public owner API.

**Acceptance and proof:** every recovery/inventory/race/key test is GREEN on the
supported native platform; validate has zero filesystem delta; a fresh process
may see a committed intake alone but sees content/result/candidate only through
one committed candidate; realized filesystem, evidence, authoritative-output,
and future-receipt cardinalities equal the exact write-set table; a candidate
with missing intake refuses; Charter lineage paths,
versions, and vectors are byte-identical. Every pre-establishment refusal has
zero durable delta; every established refusal has only its exact control/journal
lineage and zero artifact output delta.

**Stop:** strong no-follow/durability/serializability cannot be provided, lock
order would invert HCM-2.2, recovery needs deletion/adoption, or state becomes
ambiguous.

## Increment 10 — RED/GREEN non-governed compare-and-write promotion

**Purpose:** define the only future generic canonical mutation, without
authorizing its implementation.

**Authority repair gate:** Review 6 invalidated the prior verify-then-rename
publication authority, and Review 9 identified documentation-only ABI and
authority defects. The current work may repair only the seven frozen
documentation surfaces; production, selectors, staging, and commit remain
frozen. Before Rust resumes, a different-fresh complete-subject review must
accept `atomic-displaced-basis-v1` across the SPEC, runtime contract, control
schema/vector, tasks, and authority-repair proof, after which the operator must
explicitly authorize all sixteen CRITICAL surfaces and a later implementation
selector must bind that authority. This documentation gate authorizes no
implementation change by itself and Checkpoint C remains reopened. The
executable vector gate requires exactly 81
named matrix rows with 162 distinct Unix/Windows cells and four complete
fingerprinted success/conflict chains, including the expected-absent
source-deletion outputless conflict; missing IDs, wildcard coverage,
incomplete cells, broken record/fingerprint edges, or an unexecuted declared
semantic rejection fail the gate. It also fails if any unauthenticated `C`,
`R`, or `D=X` object is restored/adopted/returned/cleaned, any exact source-race
outcome is absent or weakened, a crash without completed live continuity
authorizes retry/result/marker/mutation, or any Windows move uses a pathname
primitive instead of retained-handle
`NtSetInformationFile(FileRenameInformation)` with retained `RootDirectory`, a
simple child name, and `ReplaceIfExists=FALSE`. Every native version must be
recomputed from its
persisted structured observation. A writer-chosen token, Unix mode/xattr delta,
or Windows attribute/last-write/DACL/named-stream delta must be executed and
rejected. All-zero and all-ones Windows file IDs, every object-ID outcome except
the exact immediate-sentinel or completed-`IO_STATUS_BLOCK`
`NtFsControlFile` absence traces, numeric-only continuity,
ancestor/parent/junction races, same-ID ABA, and basis-backup mismatch must also
be rejected.

**RED first:** exact current/absent basis, stale basis, ABA, changed operation
context, wrong candidate/kind/instance/schema/intake, approval injection,
structural invalidity, emitted-byte drift, resolved-definition mismatch/reorder,
wrong or shortened transaction evidence family, every promotion crash point,
proper-subset recovery, same/different candidate races, restart and replay.
At the final native boundary, inject changed bytes, deletion, different-object
replacement, same-ID ABA, and competing publication on Unix and Windows. Cover
retained-source and retained-parent no-replace `C -> D` and `R -> C`, plus every
crash boundary. A crash lacking a durable completed continuity record must be
markerless non-retryable ambiguity regardless of the reopened tuple. Inject
Windows ancestor/source-parent/destination-parent rename, junction substitution,
zero/all-ones `FILE_ID_INFORMATION`, Object-ID presence/unsupported/access-denied/other/
between-pass mutation, and expected-basis-backup loss or mismatch. Cover
destination interference, sharing errors, other errors, and reader attempts
before marker/refusal terminalization. Inject Unix mode and
xattr mutations and Windows attribute, last-write, owner/group/DACL, and
non-default named-stream mutations after close without changing file bytes.
Each must invoke no move when found before the boundary or perform no further
move when found afterward, retain evidence, write no result/marker, and
withhold readers. Reject any `D=X`
restore/adopt/delete/cleanup path, any `ReplaceFileW` path, and any
1175/1176/1177 vector as outside the selected adapter.

**Implementation:** extend the generic store/service with the exact promotion
`1.2` intent/output set, staged canonical and promotion bytes, and the
`atomic-displaced-basis-v1` protocol. Use the exact transaction-unique candidate
and displaced refs plus transaction-local `expected-basis.backup` copied from
the retained basis handle and preserved as evidence only; durable
`native-publication.json` before the boundary; parent-anchored no-replace
claim/publish adapters; retained source and parent handles through destination
observation; stable identity/content plus complete structured pre-call/post-move
observations, digest-derived versions, and a completed live-object continuity
record;
durable `native-publication-result.json` with exact native-call trace; result
fingerprint repeated by marker and evidence; exact forward/authenticated-basis
restoration actions; mutation-free ambiguous retention for every `D=X`;
outputless
`publication_basis_conflict` terminal refusal; and recovery-first reader
authority. Native return codes and reopened numeric identity are diagnostic
only. Candidate/displaced/backup evidence has no cleanup path. Successful
promotion still exposes exactly two
authoritative outputs and no Phase-4 receipt.
On Windows, every ancestor is retained without `FILE_SHARE_DELETE`; each next
child and the post-rename destination rebound is opened by a private local
`ntdll!NtCreateFile` `extern "system"` helper. The helper uses a non-null
role-specific retained `OBJECT_ATTRIBUTES.RootDirectory`, one validated child
`UNICODE_STRING` backed by an unterminated counted UTF-16 buffer with even
nonzero `Length <= 65534`, `MaximumLength == Length`, exactly `Length` buffer
bytes, and no terminator. It uses
`OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE`, `FILE_OPEN`, null allocation/EA,
exact retained-directory
`FILE_OPEN_REPARSE_POINT | FILE_SYNCHRONOUS_IO_NONALERT` options
(`0x00200020`) without `FILE_DIRECTORY_FILE`, and regular-file options
(`0x00200060`) that add `FILE_NON_DIRECTORY_FILE`,
role-appropriate read/attribute/control access plus `DELETE` for the source,
and read/write sharing without `FILE_SHARE_DELETE`. Every call requires
`STATUS_SUCCESS`, matching `IO_STATUS_BLOCK.Status`, and
`IO_STATUS_BLOCK.Information == FILE_OPENED`; null/wrong parents,
multi-component names, wrong flags/access/share, and `CreateFileW` child-open
fallback fail before authority. `CreateFileW` remains only for initial
root/volume acquisition where no retained parent exists. Post-open retained
directory observations must prove directory type and reject any reparse point
or tag. Enumerated/opened children have matching nonsentinel file IDs. The
DELETE-capable source handle calls local `ntdll!NtSetInformationFile` with
`FileRenameInformation`, the retained destination parent as
`FILE_RENAME_INFORMATION.RootDirectory`, a simple counted leaf name, and
`ReplaceIfExists=FALSE`, and remains live through relative destination
observation. All FFI declarations, native layouts, counted buffer construction,
and unsafe code remain inside `publish_replacement`; no helper, dependency,
Cargo edit, global, fallback primitive, or public API is added. Complete
nonsentinel file ID
and exact absent object-ID status are mandatory. A second private local
synchronous `ntdll!NtFsControlFile` `extern "system"` helper freezes the exact
ten-parameter ABI: the retained regular-file synchronous handle; null
Event/APC routine/APC context; initialized mutable `IO_STATUS_BLOCK`;
`FSCTL_GET_OBJECT_ID`; null/zero input; and non-null exact 64-byte
`FILE_OBJECTID_BUFFER` output. Immediate raw
`STATUS_OBJECTID_NOT_FOUND` is authoritative when the I/O status block remains
exactly at its initialized status/information sentinel; the prior matching
raw/final `STATUS_OBJECTID_NOT_FOUND` plus zero `Information` completion remains
valid. `STATUS_PENDING` must complete through the I/O status block to
`STATUS_OBJECTID_NOT_FOUND` with zero `Information`; pending with either
sentinel unchanged fails closed. Dual success with 64-byte `Information` means
present and refuses; every other sentinel, disagreement, or unexpected trace
fails closed. `DeviceIoControl` is not raw-NTSTATUS authority and has no
fallback. Both private helpers are
unauthorized CRITICAL children of the already stopped sixteen surfaces, not
independent seventeenth surfaces, and add no dependency, public API, global,
static, or thread-local state. There is no `MoveFileExW` fallback or invented
directory/write-through durability claim.
On Unix, retained-fd `statx` plus two identical complete
`flistxattr`/`fgetxattr` passes record device/inode/type/length/link/mode/uid/
gid/mtime/ctime and every raw-name-sorted xattr value digest. On Windows,
retained-handle information classes plus owner/group/DACL security observation,
exact default-only stream enumeration, and last file USN record every required
component; SACL is excluded and never trusted. Every component except Unix
ctime or Windows change-time/USN is move invariant. Unavailable, unreadable, or
unstable required metadata refuses without fallback.
Do not construct Phase-4 `WriteReceipt` bytes; prove only the exact future one-
receipt-per-authoritative-output mapping. Re-read the
candidate closure and independently reproduce the timestamp-free validation
result under every current authority input while holding the generic lock. Null
approval is not implicit approval.

**Expected file areas:**

- `artifact_lineage_store.rs`;
- `artifact_operations.rs`;
- focused promotion integration tests; and
- no Charter workflow file.

**Acceptance and proof:** one winner, deterministic equivalent replay, typed
different-request and final-boundary basis conflict, all-or-neither visibility,
and fresh-process canonical validation all pass. No authorized reader returns a
markerless replacement. Fresh execution authorizes only a completed
retained-live-handle proof. Recovery without its durable completed record is
markerless non-retryable ambiguity even when `C/R/D` bytes and numeric IDs look
exact. Unexpected `D`, transaction replacement/substitution, and
`expected-basis.backup` remain evidence; every ambiguous tuple is mutation-free,
and crate-native proof passes on Unix and Windows without fallback.

**Authority stops before GREEN:** the required
`native-publication-result.json` is rejected by the live inventory allowlist,
and the recovery and committed-read consumers must understand the same exact
chain. Exact depth-three GitNexus results are all `CRITICAL`:

| Symbol | Direct | Affected without / with tests | Processes | Modules |
|---|---:|---:|---:|---:|
| `validate_transaction_inventory` | 2 | 16 / 28 | 7 | 2 |
| `GenericArtifactLineageStoreV1::recover_pending` | 1 | 9 / 21 | 5 | 2 |
| `GenericArtifactLineageStoreV1::verify_committed` | 3 | 12 / 24 | 5 | 2 |
| `native_bound_tokens` | 5 | 23 / 23 | 6 | 2 |
| `native_path_tokens` | 1 | 17 / 17 | 5 | 2 |
| `native_metadata_subjects` | 1 | 17 / 17 | 5 | 2 |
| `observe_retained_regular_file` | 5 | 18 / 20 | 6 | 2 |
| `same_native_metadata` | 5 | 22 / 22 | 6 | 2 |
| `validate_native_publication_observation` | 2 | 13 / 13 | 5 | 2 |
| `rename_store_path` | 6 | 28 / 59 | 8 | 2 |
| `publish_replacement` | 3 | 17 / 19 | 6 | 2 |
| `verify_compare_and_write_path` | 4 | 22 / 52 | 6 | 2 |
| `require_installed_guards_unchanged` | 3 | 22 / 53 | 7 | 2 |
| `observe_installed_output` | 3 | 21 / 21 | 6 | 2 |
| `observe_prepared_replacement` | 2 | 13 / 15 | 6 | 2 |
| `require_prepared_replacement_unchanged` | 2 | 13 / 15 | 6 | 2 |

The exact include-tests commands, also run without `--include-tests`, are:

```text
npx gitnexus impact validate_transaction_inventory --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact validate_transaction_inventory --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact recover_pending --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact recover_pending --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_committed --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_committed --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_bound_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_bound_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_path_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_path_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_metadata_subjects --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_metadata_subjects --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_retained_regular_file --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_retained_regular_file --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact same_native_metadata --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact same_native_metadata --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact validate_native_publication_observation --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact validate_native_publication_observation --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact rename_store_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact rename_store_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact publish_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact publish_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_compare_and_write_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_compare_and_write_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_installed_guards_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_installed_guards_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_installed_output --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_installed_output --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_prepared_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_prepared_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_prepared_replacement_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_prepared_replacement_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
```

No Rust, implementation selector, staging, or commit is permitted until the
operator explicitly authorizes all sixteen CRITICAL surfaces: the three
record-chain surfaces, six structured-observation surfaces, and seven
retained-handle publication surfaces. Partial approval is insufficient.

**Stop:** promotion needs HCM-2.2 record versions/authority, last-writer-wins,
ordinary replace-after-check, unjournaled replacement, unsupported native
fallback, automatic migration, cleanup/adoption of candidate or displaced
evidence, cooperating-lock-holder threat narrowing, or cross-store reverse
locking.

### Checkpoint C — Persistence and concurrency

- complete intake/candidate and promotion fault matrices pass;
- native Windows and any repository-supported strict platform proof pass;
- all engine tests, Charter regressions, format, Clippy, and diff checks pass;
- no unjournaled/orphan state is product-visible; and
- fresh complete-subject review has accepted the atomic-publication repair and
  the implementation proves its exact schema/vector authority.

## Increment 11 — RED/GREEN generic CLI read path

**Purpose:** expose stable list/read/validate commands through a thin adapter.

**RED first:** Clap grammar; required `--repository-root`; exact kind/instance
data; missing fixed selection record; JSON/human success/refusal; no cwd scan;
no generated kind command; stable exit mapping.

**Implementation:** add the fixed `artifact` command family and a focused CLI
adapter that maps arguments to engine types and renders returned typed results.

**Expected file areas:**

- narrow `crates/cli/src/main.rs` command registration;
- new `crates/cli/src/artifact.rs`;
- CLI tests; and
- no compiler/flow/setup/doctor changes.

**Acceptance and proof:** actual binary tests prove list/read/validate by custom
kind/instance; CLI contains no kind-specific semantics or source discovery.

**Stop:** command routing must be dynamic/generated, or normal behavior requires
compiler, flow, setup, doctor, Tauri, Substrate, or transport-catalog changes.

## Increment 12 — RED/GREEN generic CLI intake and promotion path

**Purpose:** complete the bounded real product path.

**RED first:** separate `intake-evaluate`, `intake-append`, `candidate-validate`,
`candidate-append`, and `promote` grammar; bounded `--from-inputs path|-` and
`--from-request path|-`; exact expected-current grammar including `absent`;
candidate/intake refs; mutation idempotency key inside the request document and
absent from argv/output; JSON/human outcomes, approval-option absence, restart/
replay, established-refusal exact result/evidence binding, pre-establishment
zero-write behavior, tombstone outcomes, and refusal exit codes.

**Implementation:** extend only the generic CLI adapter to call the owner
service. It never writes files directly or reproduces coverage/currentness.

**Expected file areas:**

- `crates/cli/src/artifact.rs`;
- CLI integration tests;
- fixture input envelopes; and
- main help/golden only if existing test convention requires it.

**Acceptance and proof:** the actual binary resolves registration, validates
canonical YAML, evaluates all modes without mutation, appends one finalized
intake, validates a candidate from that committed intake without mutation,
appends only the independently reproduced expected candidate, promotes it,
restarts after every operation/fault boundary, replays, and retains identical
kind/instance IDs through every captured layer. Filesystem-delta assertions
prove evaluate/validate write nothing and each mutation realizes only its exact
write set.

**Stop:** stdin becomes interactive/prompt authority, JSON is claimed as full
Phase 4 transport, or any per-kind CLI adapter appears.

## Increment 13 — Attack, real-path, restart, and concurrency closure

**Purpose:** run the complete HCM-2.3 matrix as one coherent product proof.

Add only missing tests discovered by matrix review: cross-kind substitution,
definition mutation between phases, source order, unsafe filesystem states,
compound-invalid precedence, concurrent process races, crash/restart at every
commit and refusal durability boundary, retained refusal replay and tombstone
conflict/expiry, installed-default non-membership, and guards against enum/
filename/dynamic-command dispatch.

**Expected file areas:** HCM-2.3 engine/CLI tests and proof log only; production
changes require returning to the owning prior increment with a new RED.

**Acceptance and proof:** one actual-binary scenario and the full negative/
attack matrix pass; unit-only evidence is explicitly insufficient.

**Stop:** a missing proof needs scope beyond the exact modules/commands or
changes the specification.

## Increment 14 — Full regression wall and evidence-backed status

**Purpose:** prove compatibility and update only the classifications earned.

Run the complete command wall in `SPEC.md`, focused HCM-1.1/HCM-1.3/HCM-2.1/
HCM-2.2 suites, package-member equality, feature/dependency inspection,
definition/vector/fingerprint replay, links/references, archive boundary,
handoff modes, whitespace, scope, and GitNexus detection.

After all proof passes, update only:

- HCM-2.3 packet/todo status and one implementation proof wall;
- `03-seam-crosswalk.md`, solely for the atomic maximum change set: the exact
  registry-brief subset of `Artifact kind/schema registry` and the exact
  registry-brief subset of `Charter intake coverage` each move `TargetOnly ->
  RealPathAdopted`;
- `04-phase-slice-map.md`, solely for factual HCM-2.3 completion/next gate; and
- `06-proof-and-regression-ledger.md`, solely for exact PG-KIND-01 subset,
  PG-KIND-02 proof-kind closure, and PG-ARTIFACT-01 subset evidence.

No other classification moves. The exact Charter intake subset remains
`ContractCorrectAndProven`, broader generic/custom intake remains `TargetOnly`,
and every other crosswalk cell must compare byte-identical. `00-README.md` changes only if its current status
would otherwise be factually stale.

**Acceptance and proof:** exact changed-path manifest and fingerprint are stable;
no shipped definition/package member, HCM-2.2 byte, Cargo file, Phase 3+, SDK,
transport, setup, doctor, flow, compiler, or production asset changed outside
the explicit implementation scope.

**Stop:** any regression, archive/preservation drift, unexpected graph flow, or
classification would exceed the ceiling.

## Increment 15 — Fresh complete-subject review loop

**Purpose:** obtain independent quality evidence, not self-approval.

1. freeze the exact complete subject manifest/fingerprint, then build an
   immutable schema-valid review dispatch that binds it and remains transport
   outside its own manifest; an active dispatch never hashes itself;
2. send it to a fresh isolated read-only built-in `default` reviewer with no
   parent reasoning or success conclusion;
3. require findings ordered Critical, Required, Optional, Nit and exact return
   metadata;
4. accept and repair every valid finding without waiver, returning to the
   owning RED increment when behavior changes;
5. rerun the complete proof wall and create a new immutable dispatch; and
6. use a different fresh reviewer. Repeat until the complete exact subject is
   CLEAN.

**Acceptance and proof:** the final CLEAN manifest includes every implementation,
fixture, test, packet, control-pack, proof, and prior immutable review/remediation
artifact selected for review, while excluding only the active dispatch itself.
The active dispatch validates independently, binds the subject exactly, enters
the later commit as an audit artifact, and no reviewed subject byte changes
after its creation.

**Stop:** mandatory delegation unavailable, review cannot bind the exact
subject, a valid finding needs new authority, or the subject cannot reach CLEAN.

## Increment 16 — Primary implementation commit

Apply `git-workflow-and-versioning`. Before staging:

- rerun the full proof wall against the CLEAN bytes;
- run fresh GitNexus `detect-changes` and inspect symbols/flows/modules;
- replay the exact final subject manifest/fingerprint;
- verify preservation locations and no unauthorized paths;
- run `git diff --check`; and
- confirm handoff validators pass before the ledger changes.

Stage only the reviewed HCM-2.3 implementation/control-pack subject, inspect the
staged diff and manifest byte-for-byte, and create one primary implementation
commit. Do not include the parent closeout handoff or rebuilt ledger.

**Stop:** staged bytes differ from CLEAN, GitNexus scope is unexpected, any gate
fails, or the index is already changed.

## Increment 17 — Parent closeout and separate commit

The active top-level parent creates one HCM-2.3 implementation-completed v1.2
handoff. It consumes the planning closeout as dependency context, binds the
primary implementation commit and final CLEAN subject, states the exact proof
classification/open gates, and provides one exact next selector without
starting HCM-2.4 or another phase.

Rebuild `handoffs/ledger.jsonl` deterministically, validate all three modes, and
commit only the new handoff plus ledger/index closeout artifacts in a second
commit. Verify final ancestry, two-commit split, clean worktree, preservation
baselines, and exact handoff read command.

## Risks and mitigations

| Risk | Impact | Mitigation/stop |
|---|---|---|
| descriptor resolver is CRITICAL | broad profile/CLI regressions | RED all old dependency closures; narrow one-field change; stop on widened live impact |
| intake registry accidentally changes old profile fingerprints | invalidates HCM-2.2 authority | additive operation-context fingerprint; old fingerprint bytes are golden invariants |
| generic path imports Charter semantics | false generic proof and authority leak | separate modules/paths/record `1.0`; null approval; cross-import and substitution tests |
| source binding becomes ambient/caller authority | unsafe extension boundary | fixed closed selection record, explicit root, exact refs/fingerprints, no scan/override |
| multi-file append/promotion exposes partial truth | corrupted canonical authority | coarse lock, recovery-first readers, staged verified bytes, intent/commit visibility |
| Phase 4 SDK/transport claim is pulled forward | ownership conflict | engine owner APIs + local CLI only; no SDK crate/bootstrap/public catalog/gate movement |
| generic emitter widens into Projection/renderer | scope escape | JSON-data-model serialization only; no kind/schema semantics; stop on transformation need |
| tasks become horizontal or oversized | unreviewable implementation | checkpoints after registry, pure kernel, persistence, CLI; return changes to owning RED |

## Definition of done

Each increment meets its acceptance criteria, is runtime-tested, handles its
edge/refusal paths, keeps existing tests green, stays focused and formatted,
documents public owner behavior, and records proof. The feature is done only
after complete integration/security/documentation proof, different-fresh CLEAN
review, the primary implementation commit, separate parent closeout commit, and
a clean worktree. No deployment or publication is part of this slice.
