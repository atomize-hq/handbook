# HCM-2.3 Implementation Todo

This historical implementation checklist includes completed planning and
documentation-remediation items, while every incomplete implementation,
different-fresh review, primary-commit, and closeout item remains unchecked. A
later implementation parent may check an incomplete item only after its
acceptance and verification evidence exists.

## Entry and context

- [x] Verify the exact implementation selector, branch, planning closeout HEAD,
      clean status, ancestry, both HCM-2.2 commits, and both HCM-2.3 planning
      commits.
  - Acceptance: no staged, unstaged, or untracked path; the selector names this
    exact packet and remains within HCM-2.3.
  - Verify: record `git status`, `git rev-parse`, `git merge-base --is-ancestor`,
    handoff parse, and all three handoff validator results.

- [x] Apply the required live skill chain and record a bounded implementation
      capsule.
  - Acceptance: context, source, spec, plan, API, security, TDD, documentation,
    review, and Git workflows have explicit phase evidence.
  - Verify: proof log names the live skill paths, relevant authority sections,
    allowed paths, non-goals, gates, and stop conditions.

- [x] Refresh GitNexus and inventory live registry/profile/intake/CLI surfaces.
  - Acceptance: every existing symbol proposed for edit has fresh upstream
    impact; all HIGH/CRITICAL results are warned before edits.
  - Verify: record contexts, impacts, processes/modules, index status, and exact
    decision to consume or edit each symbol.

- [x] Replay preservation, HCM-1.3 fixture, HCM-2.2 authority, package member,
      and existing CLI baselines.
  - Acceptance: all bytes/identities/fingerprints match planning closeout.
  - Verify: literal hash/manifests and preservation metadata/sentinels compare.

## Registration and instance closure

- [x] Add RED tests for the closed fixed-path repository selection record and
      safe intake source admission.
  - Acceptance: malformed, duplicate, missing, conflict, unsafe path, symlink,
    size/count/depth, mutation, every source class/effect mapping, reorder, and
    ambient-discovery cases match the exact selection schema/vector; exact
    shipped-plus-custom counts/replacements resolve; same-ref/different-source,
    cross-class, and schema-valid incomplete selections refuse; executable
    schema mutations reject one-segment/underscore/repeated-hyphen/noncanonical-
    SemVer refs plus dot/trailing/65-component/1,025-byte paths and final line
    terminators in either grammar.
  - Verify: schema validation, normalized fingerprint replay, typed round-trip,
    and focused tests fail with named assertions before production edits.
  - Files: focused engine test and first HCM-2.3 fixture subset.

- [x] Implement the intake registry and additive artifact operation context.
  - Acceptance: exact refs/fingerprints/compatibility resolve deterministically;
    source permutations agree; old resolved-profile fingerprints do not change.
  - Verify: focused GREEN tests, engine tests, feature/dependency inspection.
  - Files: new intake-registry/context modules, narrow `lib.rs`, tests.

- [x] Add RED tests and the narrow descriptor intake-selection change.
  - Acceptance: generic `intake_definition_ref` is carried, all other later-
    owned refs still refuse, and the operation context closes intake before use.
  - Verify: fresh CRITICAL impact warning, focused tests, all descriptor/profile/
    HCM-2.2 dependency-closure regressions.
  - Files: narrow `artifact_instance.rs`, tests, remaining profile/intake fixture.

- [x] Complete Checkpoint A.
  - Acceptance: registration closure is green, package defaults exclude the
    custom kind, HCM-1.3 bytes match, and scope/graph/diff checks pass.
  - Verify: full engine tests plus captured manifests.

## Generic read, validation, and intake

- [x] Add RED tests for descriptor-selected generic canonical read/validation.
  - Acceptance: kind/instance mismatch, YAML/path/size/mutation/structural
    failures, layer precedence, and the runtime exact-ref/safe-ref/digest/record-
    ID schema rejection vectors, including final line terminators, are frozen
    before behavior.
  - Verify: focused tests fail for the missing generic operation service.

- [x] Implement read-only generic artifact operations.
  - Acceptance: list/read/validate/intake-definition-read consume existing
    registries, return typed layers, and contain no kind/filename dispatch.
  - Verify: focused GREEN, full engine regressions, source inspection guard.
  - Files: new artifact-operations module, narrow exports, tests.

- [x] Add RED tests and implement pure intake evaluation and finalized-record
      candidate previews.
  - Acceptance: all three modes converge on equal content without record
    identity or writes; a fixed committed intake reproduces intake `1.2`,
    validation result `1.0`, candidate `1.4`, and promotion `1.2` vectors; old
    versions remain exact; semantic/approval are explicitly not-applicable.
  - Verify: schema/vector/fingerprint replay plus focused positive/negative/
    cross-version/Charter-substitution tests.
  - Files: new artifact-intake module, tests, narrow service integration.

- [x] Add RED tests and implement deterministic generic canonical YAML.
  - Acceptance: one stable JSON-data-model encoding, parse/emit/parse equality,
    no tags/anchors/comments, no per-kind transformation.
  - Verify: golden/permutation/Unicode/ambiguous-scalar/output-limit tests.
  - Files: new canonical-yaml module and focused tests.

- [x] Complete Checkpoint B.
  - Acceptance: pure kernel and all earlier membrane regressions pass with no
    persistent delta.
  - Verify: engine tests, format, Clippy, graph, scope, diff-check.

## Persistence and concurrency

- [x] Add the complete RED intake/candidate transaction and recovery matrix.
  - Acceptance: every crash point; every two-value intake subset; all eight
    content/result/candidate and four canonical/promotion subsets; malformed/
    extra/unsafe/mismatched state; exact inventory limits; valid intake-without-
    candidate; zero-write validation; subordinate invisibility; key retention/
    tombstone/hold/currentness; race, restart, replay, and conflict are asserted;
    pre-establishment refusals prove zero durable writes; and every legal
    established-refusal prefix from atomic intent publication/derived hold
    through evidence, refused
    result, retained ledger, committed rename, exact replay, and tombstone is
    covered. Output-bearing, missing-evidence, mismatched-binding, non-prefix,
    same-key/different-request, and code/layer/nullability cases refuse exactly.
  - Verify: focused fault tests fail before store implementation; injection is
    test-private.

- [x] Implement the separate generic lineage store.
  - Acceptance: exact paths, bounded no-follow inventory, coarse ordered lock,
    JCS+LF immutable records and subordinate value/content/validation closure,
    closed journal records, total complement recovery, direct-CLI domain replay,
    one authoritative semantic-record output per append, exact internal
    evidence, future receipt cardinality, and per-operation all-or-neither
    visibility pass. Only exact context/evaluation may establish; active hold is
    solely the exact intent projection; every writing/established/pending intent
    crash state is total; and an established refusal persists/restarts/replays as
    the outputless evidence-backed control lineage while a pre-establishment
    failure persists nothing.
  - Verify: filesystem/evidence/output delta fixtures, full recovery/key/race/restart
    suite, and unchanged Charter versions/lineage bytes.
  - Files: new generic lineage-store module, tests, narrow service/export wiring.

- [x] Add RED tests and implement non-governed compare-and-write promotion.
  - Acceptance: current/absent basis, ABA/stale context, cross-kind/approval
    injection, exact ordered equality with operation-context resolved
    definitions, mismatch/reorder refusal, crash/recovery, same/different races,
    and fresh-process validation are deterministic and fail closed.
  - Verify: promotion matrix plus emitted-byte/schema revalidation.
  - Files: generic store/service and focused tests only.

- [x] Repair atomic canonical-publication documentation authority after Review 6.
  - Acceptance: SPEC, runtime contract, control schema/vector, and plan/todo
    define one implementation-grade `atomic-displaced-basis-v1` state machine
    for Unix and Windows, exact `C/R/D` names and authority, durable pre/post
    native records, exhaustive fresh/recovery classifier, rollback/refusal,
    reader boundary, inventory, cleanup prohibition, and final-boundary vectors.
  - Verify: duplicate-safe JSON, Draft 2020-12 meta-validation/vector admission,
    fingerprint replay, Markdown links/fences, diff whitespace, dispatch replay,
    and exact zero `crates/**` change.
  - Files: HCM-2.3 authority documents and one repair proof only; no Rust,
    fixtures, handoff/ledger, archive, staging, commit, or push.

- [x] Remediate atomic-publication documentation Review 1.
  - Acceptance: replace `ReplaceFileW` and reverse replacement with a shared
    no-replace claim/publish protocol; separate stable identity/content from
    primitive-mutated post-version evidence; define exact Windows
    access/share/close and `GetFileInformationByHandleEx(FileIdInfo)` authority;
    close expected-present, expected-absent, outputless-conflict, crash,
    interference, and semantic-binding vectors; and record the unapproved
    CRITICAL inventory admission stop.
  - Verify: fingerprinted Unix/Windows success and conflict chains; exact
    `C/R/D` action vectors; schema plus semantic rejection replay; primary
    native references; duplicate-safe JSON; links/fences; handoff modes; diff;
    and exact 65-entry subject/crates replay.
  - Files: HCM-2.3 SPEC/runtime/schema/vector/tasks and the existing atomic
    authority proof only.

- [x] Remediate atomic-publication documentation Review 2.
  - Acceptance: preserve `validate_transaction_inventory`, `recover_pending`,
    and `verify_committed` as three explicit CRITICAL operator stops before any
    selector; replace prose-only coverage with an initial exact
    50-row/100-platform-cell matrix (subsequently extended by Review 3) and
    three complete fingerprinted chains; execute every declared
    schema/semantic rejection; and correct Windows authority so only the
    write-capable `R` producer flushes before close while read-only observation
    handles never flush.
  - Verify: deterministic checker rejects missing IDs, wildcard platforms,
    incomplete rows, broken refs/fingerprints, non-outputless conflict, and
    unexecuted rejections; schema/fingerprint/Rust identity-chain replay;
    duplicate-safe JSON; links/fences; handoff modes; dispatch/crate replay;
    diff; and exact zero `crates/**` change.
  - Files: the same seven HCM-2.3 documentation-authority paths only.

- [x] Remediate atomic-publication documentation Review 3.
  - Historical, superseded acceptance: unauthenticated `D=X` always returns
    mutation-free
    `refused_ambiguous` and retains all evidence; only authenticated `D=E'` may
    restore the expected basis after `R` loss/substitution. The exact
    66-row/132-cell matrix covers post-handle-close `R` deletion plus same-byte
    and different-byte substitution through no-call, failed-call,
    successful-call, crash-without-return, and distinct rebound
    classifications. Review 3 then required `MoveFileExW` plus
    `MOVEFILE_WRITE_THROUGH`, but Review 6 superseded that path-only primitive
    with the retained-parent protocol; it is not current acceptance authority.
  - Historical verification: the then-current checker asserted the exact
    source-race row set and refused every `D=X`
    restore/adopt/delete/cleanup path. The former `MoveFileExW` flag scan is
    superseded; the current checker rejects every pathname primitive and requires
    retained-handle `SetFileInformationByHandle(FileRenameInfo)` authority.
  - Files: the same seven HCM-2.3 documentation-authority paths only.

- [x] Remediate atomic-publication documentation Review 4.
  - Acceptance: the expected-absent adapter has the same exact
    post-handle-close `R` deletion, same-byte substitution, and different-byte
    substitution cross-product as expected-present, with distinct no-call,
    failed-call, successful-call, crash-unmoved, and crash-moved rebounds on
    Unix and Windows. Stable safe rows end only in outputless
    `publication_basis_conflict`; unsafe, duplicate, or displaced-unbound rows
    remain mutation-free `refused_ambiguous`. No unauthenticated `C` or `R`
    becomes canonical or readable, and all objects/evidence remain retained
    pending separately authorized cleanup.
  - Verify: proof-embedded checker pins 81 rows/162 platform cells, all 30 source
    races, four complete chains, fresh/recovery equality, the exact no-call
    empty trace, every Windows move flag, and deterministic failures for row
    removal or disposition weakening; schema/fingerprint/frozen-Rust identity
    replay, duplicate-safe JSON, links/fences, handoff modes, exact dispatch
    manifest delta, diff, and zero `crates/**` change all pass.
  - Files: the same seven HCM-2.3 documentation-authority paths only.

- [x] Remediate atomic-publication documentation Review 5.
  - Acceptance: replace opaque native-version authority with persisted
    platform components and deterministic RFC 8785/SHA-256 derivation. Unix
    binds device/inode/type/length/link/mode/uid/gid/mtime/ctime and a complete
    raw-name-sorted retained-fd xattr inventory. Windows binds volume/file
    identity, type/reparse/length/link/times/attributes, owner/group/DACL
    digest, default-only exact named-stream inventory, and USN while explicitly
    excluding SACL. Only Unix ctime or Windows change-time/USN may differ after
    a proven move; every other component is exact.
  - Acceptance: Unix mode/xattr and Windows attribute/last-write/DACL/named-
    stream races are outputless and mutation-free, retain pre/post observations
    and `C/R/D`, withhold readers, and never authorize an opaque token fallback.
    Unsupported, unreadable, incomplete, or unstable required observation
    refuses.
  - Verify: 67 positive fingerprints, 51 atomic vectors, unchanged exact
    81-row/162-cell matrix, four chains, and all 17 declared mutations pass the
    proof-embedded checker; arbitrary Unix/Windows tokens and every named
    component delta are actually mutated and rejected.
  - Files: the same seven HCM-2.3 documentation-authority paths only. The six
    newly discovered exact CRITICAL production seams are recorded as future
    implementation stops; this item authorizes none of them.

- [x] Remediate atomic-publication documentation Review 6.
  - Acceptance: replace every Windows pathname move with the DELETE-capable
    retained-source `SetFileInformationByHandle(FileRenameInfo)` form using the
    retained destination parent as `RootDirectory`, one simple child name, and
    `ReplaceIfExists=FALSE`; retain Windows ancestors without
    `FILE_SHARE_DELETE`, enumerate/open each child from its retained parent, and
    reject reparse/junction substitution.
  - Acceptance: every successful Unix/Windows move proves the destination is the
    same live kernel object as the still-open source before result/marker
    construction. Numeric identity is never sufficient. All-zero or all-ones
    Windows file ID, Object-ID presence/unsupported/access-denied/other/mutation, same-ID ABA,
    and lost parent/source handles fail closed.
  - Acceptance: expected-present creates transaction-local
    `expected-basis.backup` from the retained basis handle and preserves its
    bytes/structured-metadata fingerprint as evidence only. Recovery without a
    durable completed continuity record is outputless, markerless, non-retryable
    `refused_ambiguous` and never infers, restores, adopts, deletes, or cleans.
  - Verify: 67 positive fingerprints, 61 atomic vectors, 81 rows/162 cells, four
    chains, 20 semantic mutations, 35 schema rejections, and four schema
    acceptances pass the proof checker. The seven newly identified CRITICAL
    publication seams join the prior nine as independent future stops.
  - Files: the same seven HCM-2.3 documentation-authority paths only; no Rust,
    selector, staging, commit, or push.

- [x] Remediate atomic-publication documentation Review 7.
  - Acceptance: all five status/proof-authority documents state that the
    documentation authority is reviewable while the current production
    implementation is known non-authoritative; the exact registry-brief subset
    remains at its current `TargetOnly` baseline and cannot move to
    `RealPathAdopted`, be committed, or close `PG-KIND-02`; every document
    preserves the exact ordered post-documentation gates.
  - Acceptance: the active blocker names Reviews 1-7 and all sixteen grouped
    `CRITICAL` production surfaces, including expressly unauthorized
    `publish_replacement`.
  - Verify: exact five-document delta, unchanged protocol/schema/vector/proof/
    plan and `crates/**` bytes, embedded checker, frozen Rust identity,
    handoffs, JSON, Markdown, diff, and GitNexus change detection.
  - Files: the five status/proof-authority documents only; no production,
    selector, dispatch, handoff/ledger, archive, staging, commit, or push.

- [x] Remediate atomic-publication documentation Review 8.
  - Acceptance: the schema, semantic authority, and executed negatives reject
    both zero and all-ones 128-bit Windows file-ID sentinels at retained-source,
    enumerated/opened-child, destination-observation, and same-live continuity
    roles.
  - Acceptance: every retained-parent child and post-rename rebound open is
    frozen to the exact private local `ntdll!NtCreateFile` ABI, layouts,
    retained `RootDirectory`, single child name, no-reparse attributes,
    disposition/options/access/share, and success/`IO_STATUS_BLOCK` contract.
    `CreateFileW` remains initial-root/volume-only and cannot prove a child.
  - Acceptance: the exact registry-brief subset retains the current
    `TargetOnly` baseline, zero landed evidence, open `PG-KIND-02`, and the
    frozen future `TargetOnly -> RealPathAdopted` diff.
  - Verify: checker/schema/fingerprint replay, exact nine-document delta,
    frozen Rust, handoff/JSON/Markdown checks, diff check, GitNexus impacts and
    change detection. The future private helper is CRITICAL but remains within
    the already stopped sixteen-surface implementation boundary.
  - Files: the exact nine Review 8 documentation-authority paths only; no
    production, selector, dispatch, handoff/ledger, archive, staging, commit, or
    push.

- [x] Remediate atomic-publication documentation Review 9.
  - Acceptance: implementation-authority prose is current; all twelve omitted
    structured-observation GitNexus commands are explicit; Review 3
    `MoveFileExW` acceptance is historical; child `UNICODE_STRING` buffers are
    exact unterminated counted UTF-16; retained-directory `NtCreateFile` options
    and post-open type/reparse proof are compatible; and object-ID absence uses
    the exact private synchronous `NtFsControlFile` raw-plus-final status
    contract. Both future private helpers remain unauthorized CRITICAL children
    of the existing sixteen stopped surfaces.
  - Verify: executable checker runs 39 `NtCreateFile` and 33
    `NtFsControlFile` mutations; frozen Rust identity, all handoff modes,
    duplicate-safe JSON, Markdown links/fences, exact seven-document delta, zero
    crate delta, diff check, and GitNexus change detection pass.
  - Files: the exact seven Review 9 documentation-authority paths only; no
    production, test, fixture, top-level phase, prior proof/dispatch, selector,
    preservation/archive, staging, commit, or push.

- [x] Replace verify-then-unconditional-rename with the reviewed atomic protocol.
  - Acceptance: Unix uses retained source and parent dirfds with
    `renameat2(..., RENAME_NOREPLACE)`; Windows uses only the retained-handle
    `FileRenameInfo` form above. Expected present claims `C -> D`, proves live
    source-to-destination continuity, then publishes retained `R -> C` with the
    same proof. Expected absent directly publishes retained `R -> C`.
    `expected-basis.backup` is evidence only. Any missing completed continuity,
    same-ID ABA, object-ID/file-ID sentinel failure, parent race, `D=X`, or crash state is
    markerless ambiguity without recovery mutation.
    Unix mode/xattr and Windows attribute/last-write/DACL/named-stream deltas
    produce evidence-retaining mutation-free ambiguity. Native returns are
    diagnostic; exact retained-handle continuity controls publication; no
    destination is overwritten; ambiguous state mutates nothing; public reads
    recover before opening canonical and
    never return the transient `C`-absent or markerless states. `ReplaceFileW`
    is forbidden.
  - Verify: RED-first changed/deleted/replaced/same-byte-ABA/competing vectors on
    both platforms; post-handle-close `R` deletion and same-byte/different-byte
    substitution; Windows ancestor/parent/junction, zero/all-ones ID,
    `NtCreateFile` retained-parent rejection, Object-ID,
    share/destination/other-error/crash cases; every claim/publish crash row;
    backup preservation; stable conflict terminal refusal; exact record/
    fingerprint chain; restart; concurrent process; and actual-binary proof.
  - Files: generic lineage store/service plus focused tests and fixtures only
    after a fresh review accepts the documentation repair and the operator
    authorizes all sixteen CRITICAL record-chain, structured-observation, and
    retained-handle publication surfaces named in the SPEC and authority proof.
  - Result: landed in implementation commit
    `628b672ef33326e87e4fb30be13489e8af04b38c`; the final implementation proof
    records the atomic publication, recovery, refusal, crash, restart, and
    native-platform evidence.

- [x] Re-complete Checkpoint C after atomic-publication implementation.
  - Acceptance: persistent operations are serializable and recoverable; no
    orphan/partial/markerless authority is visible; all HCM-2.2 regressions
    remain green.
  - Verify: native Unix/Windows proof, engine tests, format, Clippy, graph and
    diff after the replacement implementation.
  - Result: the final implementation proof wall records Checkpoint C complete
    with workspace tests, strict Clippy, formatting, schemas, handoffs,
    platform proof, and GitNexus detection passing.

## Stable CLI and real path

- [x] Add RED tests and implement generic CLI list/read/validate commands.
  - Acceptance: fixed grammar, explicit repository root, exact kind/instance
    data, typed JSON/human results, stable exits, no ambient/dynamic dispatch.
  - Verify: actual binary integration tests.
  - Files: narrow CLI command registration, new generic adapter, CLI tests.

- [x] Add RED tests and implement the separate generic CLI intake/candidate/
      promote commands.
  - Acceptance: evaluate is read-only; intake append finalizes first; candidate
    validate reads committed intake and writes nothing; candidate append accepts
    only the committed intake pair plus expected candidate fingerprint; promote
    maps to one exact compare-and-write; bounded path/stdin requests carry raw
    idempotency keys without argv/output disclosure; all modes, basis, restart,
    retained refused-result replay, exact/different tombstone outcomes,
    pre-establishment zero writes, and refusal results pass with no direct CLI
    writes.
  - Acceptance: the engine checks the shared 1 MiB byte-document limit before
    parsing for evaluate/intake/candidate/promote (exact limit accepted, +1
    refused with zero mutation), and intake `consumer.version` is engine-owned
    compile-time release identity rather than a public caller argument.
  - Verify: actual binary integration tests and source-level adapter boundary.
  - Files: generic CLI adapter/tests/fixture inputs.

- [x] Run the complete actual-binary real-path scenario.
  - Acceptance: registration, instance selection, canonical validation, all
    intake modes, finalized intake, zero-write candidate validation, candidate
    append, promotion, per-operation/fault restart, replay, and identical kind/
    instance data through every layer are proven.
  - Verify: captured structured requests/contexts/records/results; unit-only
    proof is rejected.

- [x] Close all remaining attack, source-order, mutation, restart, and process-
      concurrency vectors.
  - Acceptance: the full SPEC matrix has one named passing test/evidence item,
    including every established-refusal crash prefix and binding attack.
  - Verify: matrix audit; any behavior gap returns to its owning RED increment.

## Regression, review, and commits

- [x] Run the full workspace regression wall and all documentation/security/
      archive/handoff/scope/fingerprint/package checks.
  - Acceptance: every applicable command in `SPEC.md` passes; an unchanged
    baseline packaging limitation is recorded rather than repaired outside
    scope; no dependency, shipped definition, HCM-2.2, SDK/transport, Phase 3+,
    or unauthorized path changed.
  - Verify: complete proof wall with exact command/result evidence.

- [x] Record the pre-Review 7 registry-brief candidate status without landing it.
  - Acceptance: the exact Charter cell and broader generic/custom intake do not
    move; the dirty registry-brief subject remains at the current exact
    `TargetOnly` baseline and does not move to `RealPathAdopted` or close
    `PG-KIND-02` while its publication boundary is non-authoritative.
  - Verify: exact crosswalk-cell diff assertion plus reference/link checks.

- [x] Run GitNexus change detection before review and inspect the full subject.
  - Acceptance: affected symbols/flows/modules equal expected implementation
    scope and every existing edited symbol had prior impact evidence.
  - Verify: recorded `detect-changes`, changed-path manifest, `git diff --check`.

- [x] Dispatch one fresh isolated read-only complete-subject review.
  - Acceptance: immutable schema-valid dispatch binds the exact manifest/
    fingerprint and authority while remaining outside its own manifest; findings
    are ordered Critical, Required, Optional, Nit; reviewer receives no success
    assertion.
  - Verify: active-dispatch exclusion proof, dispatch validation, unchanged
    subject hashes after creation, and structured reviewer return.
  - Result: Review 1 admitted the 43-path subject and returned three Critical
    plus ten Required findings; no finding was waived.

- [x] Remediate every valid finding and obtain a different-fresh CLEAN review.
  - Acceptance: no waiver; behavior remedies return to RED; full proof reruns;
    final reviewer is distinct and exact final subject is CLEAN.
  - Verify: immutable remediation proof/dispatch chain and final fingerprint.
  - Result: all review findings were remediated without waiver. The fresh
    built-in reviewer returned `CLEAN` over the exact 111-path subject
    `sha256:d510e94e5b47db209022927bc4afa74b8820e27cd645123cc1cdc3aa5f72fdef`
    bound by the immutable dispatch
    `handoffs/dispatches/20260725T152312Z--HCM-2-3--fresh-59-surface-complete-subject-review.json`.
    Commit `746fff667f6fbe0270182a467285d60394362530` preserves the
    dispatch artifact; the parent closeout records delegated reviewer identity,
    status, and result separately.

- [x] Create the primary implementation commit from unchanged CLEAN bytes.
  - Acceptance: staged subject exactly replays final manifest; all gates and
    preservation checks pass; handoff/ledger closeout is excluded.
  - Verify: staged diff/manifest, GitNexus detect-changes, commit inspection.
  - Result: implementation commit
    `628b672ef33326e87e4fb30be13489e8af04b38c` is an ancestor of the final
    review artifact commit and the current canonical branch head; the final
    111-path manifest replays byte-identically.

- [ ] Create the parent-owned completed handoff and deterministic ledger-only
      closeout commit.
  - Acceptance: handoff binds the primary commit/CLEAN subject, preserves open
    gates, gives one exact next selector, and does not start later work; only
    handoff/ledger/index closeout files enter commit two.
  - Verify: all three handoff modes, ledger rebuild equality, ancestry, final
    clean status, preservation equality, and exact handoff parse command.
  - Current status: intentionally pending in this reviewed checklist. Protocol
    `08` requires the reviewed reconciliation commit to exist before its hash
    can become both `repo_state.head` and `reviewed_state.baseline_head` in the
    parent-owned v1.2 handoff. The subsequent mechanical commit records
    completion through that immutable handoff and rebuilt ledger; it does not
    rewrite this reviewed subject.
