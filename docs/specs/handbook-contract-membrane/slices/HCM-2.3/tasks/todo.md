# HCM-2.3 Implementation Todo

This checklist remains unchecked during planning. A later implementation parent
may check an item only after its acceptance and verification evidence exists.

## Entry and context

- [ ] Verify the exact implementation selector, branch, planning closeout HEAD,
      clean status, ancestry, both HCM-2.2 commits, and both HCM-2.3 planning
      commits.
  - Acceptance: no staged, unstaged, or untracked path; the selector names this
    exact packet and remains within HCM-2.3.
  - Verify: record `git status`, `git rev-parse`, `git merge-base --is-ancestor`,
    handoff parse, and all three handoff validator results.

- [ ] Apply the required live skill chain and record a bounded implementation
      capsule.
  - Acceptance: context, source, spec, plan, API, security, TDD, documentation,
    review, and Git workflows have explicit phase evidence.
  - Verify: proof log names the live skill paths, relevant authority sections,
    allowed paths, non-goals, gates, and stop conditions.

- [ ] Refresh GitNexus and inventory live registry/profile/intake/CLI surfaces.
  - Acceptance: every existing symbol proposed for edit has fresh upstream
    impact; all HIGH/CRITICAL results are warned before edits.
  - Verify: record contexts, impacts, processes/modules, index status, and exact
    decision to consume or edit each symbol.

- [ ] Replay preservation, HCM-1.3 fixture, HCM-2.2 authority, package member,
      and existing CLI baselines.
  - Acceptance: all bytes/identities/fingerprints match planning closeout.
  - Verify: literal hash/manifests and preservation metadata/sentinels compare.

## Registration and instance closure

- [ ] Add RED tests for the closed fixed-path repository selection record and
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

- [ ] Implement the intake registry and additive artifact operation context.
  - Acceptance: exact refs/fingerprints/compatibility resolve deterministically;
    source permutations agree; old resolved-profile fingerprints do not change.
  - Verify: focused GREEN tests, engine tests, feature/dependency inspection.
  - Files: new intake-registry/context modules, narrow `lib.rs`, tests.

- [ ] Add RED tests and the narrow descriptor intake-selection change.
  - Acceptance: generic `intake_definition_ref` is carried, all other later-
    owned refs still refuse, and the operation context closes intake before use.
  - Verify: fresh CRITICAL impact warning, focused tests, all descriptor/profile/
    HCM-2.2 dependency-closure regressions.
  - Files: narrow `artifact_instance.rs`, tests, remaining profile/intake fixture.

- [ ] Complete Checkpoint A.
  - Acceptance: registration closure is green, package defaults exclude the
    custom kind, HCM-1.3 bytes match, and scope/graph/diff checks pass.
  - Verify: full engine tests plus captured manifests.

## Generic read, validation, and intake

- [ ] Add RED tests for descriptor-selected generic canonical read/validation.
  - Acceptance: kind/instance mismatch, YAML/path/size/mutation/structural
    failures, layer precedence, and the runtime exact-ref/safe-ref/digest/record-
    ID schema rejection vectors, including final line terminators, are frozen
    before behavior.
  - Verify: focused tests fail for the missing generic operation service.

- [ ] Implement read-only generic artifact operations.
  - Acceptance: list/read/validate/intake-definition-read consume existing
    registries, return typed layers, and contain no kind/filename dispatch.
  - Verify: focused GREEN, full engine regressions, source inspection guard.
  - Files: new artifact-operations module, narrow exports, tests.

- [ ] Add RED tests and implement pure intake evaluation and finalized-record
      candidate previews.
  - Acceptance: all three modes converge on equal content without record
    identity or writes; a fixed committed intake reproduces intake `1.2`,
    validation result `1.0`, candidate `1.4`, and promotion `1.2` vectors; old
    versions remain exact; semantic/approval are explicitly not-applicable.
  - Verify: schema/vector/fingerprint replay plus focused positive/negative/
    cross-version/Charter-substitution tests.
  - Files: new artifact-intake module, tests, narrow service integration.

- [ ] Add RED tests and implement deterministic generic canonical YAML.
  - Acceptance: one stable JSON-data-model encoding, parse/emit/parse equality,
    no tags/anchors/comments, no per-kind transformation.
  - Verify: golden/permutation/Unicode/ambiguous-scalar/output-limit tests.
  - Files: new canonical-yaml module and focused tests.

- [ ] Complete Checkpoint B.
  - Acceptance: pure kernel and all earlier membrane regressions pass with no
    persistent delta.
  - Verify: engine tests, format, Clippy, graph, scope, diff-check.

## Persistence and concurrency

- [ ] Add the complete RED intake/candidate transaction and recovery matrix.
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

- [ ] Implement the separate generic lineage store.
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

- [ ] Add RED tests and implement non-governed compare-and-write promotion.
  - Acceptance: current/absent basis, ABA/stale context, cross-kind/approval
    injection, exact ordered equality with operation-context resolved
    definitions, mismatch/reorder refusal, crash/recovery, same/different races,
    and fresh-process validation are deterministic and fail closed.
  - Verify: promotion matrix plus emitted-byte/schema revalidation.
  - Files: generic store/service and focused tests only.

- [ ] Complete Checkpoint C.
  - Acceptance: persistent operations are serializable and recoverable; no
    orphan/partial authority is visible; all HCM-2.2 regressions remain green.
  - Verify: native-platform proof, engine tests, format, Clippy, graph and diff.

## Stable CLI and real path

- [ ] Add RED tests and implement generic CLI list/read/validate commands.
  - Acceptance: fixed grammar, explicit repository root, exact kind/instance
    data, typed JSON/human results, stable exits, no ambient/dynamic dispatch.
  - Verify: actual binary integration tests.
  - Files: narrow CLI command registration, new generic adapter, CLI tests.

- [ ] Add RED tests and implement the separate generic CLI intake/candidate/
      promote commands.
  - Acceptance: evaluate is read-only; intake append finalizes first; candidate
    validate reads committed intake and writes nothing; candidate append accepts
    only the committed intake pair plus expected candidate fingerprint; promote
    maps to one exact compare-and-write; bounded path/stdin requests carry raw
    idempotency keys without argv/output disclosure; all modes, basis, restart,
    retained refused-result replay, exact/different tombstone outcomes,
    pre-establishment zero writes, and refusal results pass with no direct CLI
    writes.
  - Verify: actual binary integration tests and source-level adapter boundary.
  - Files: generic CLI adapter/tests/fixture inputs.

- [ ] Run the complete actual-binary real-path scenario.
  - Acceptance: registration, instance selection, canonical validation, all
    intake modes, finalized intake, zero-write candidate validation, candidate
    append, promotion, per-operation/fault restart, replay, and identical kind/
    instance data through every layer are proven.
  - Verify: captured structured requests/contexts/records/results; unit-only
    proof is rejected.

- [ ] Close all remaining attack, source-order, mutation, restart, and process-
      concurrency vectors.
  - Acceptance: the full SPEC matrix has one named passing test/evidence item,
    including every established-refusal crash prefix and binding attack.
  - Verify: matrix audit; any behavior gap returns to its owning RED increment.

## Regression, review, and commits

- [ ] Run the full workspace regression wall and all documentation/security/
      archive/handoff/scope/fingerprint/package checks.
  - Acceptance: every command in `SPEC.md` passes; no dependency, shipped
    definition, HCM-2.2, SDK/transport, Phase 3+, or unauthorized path changed.
  - Verify: complete proof wall with exact command/result evidence.

- [ ] Update only evidence-earned packet/control-pack status and classification.
  - Acceptance: one atomic maximum set moves exactly the registry-brief subset
    of `Artifact kind/schema registry` and registry-brief subset of `Charter
    intake coverage` to `RealPathAdopted`; the exact Charter cell and broader
    generic/custom intake do not move; PG-KIND-02 closes only for the proof
    lineage; broader gates remain open.
  - Verify: exact crosswalk-cell diff assertion plus reference/link checks.

- [ ] Run GitNexus change detection before review and inspect the full subject.
  - Acceptance: affected symbols/flows/modules equal expected implementation
    scope and every existing edited symbol had prior impact evidence.
  - Verify: recorded `detect-changes`, changed-path manifest, `git diff --check`.

- [ ] Dispatch one fresh isolated read-only complete-subject review.
  - Acceptance: immutable schema-valid dispatch binds the exact manifest/
    fingerprint and authority while remaining outside its own manifest; findings
    are ordered Critical, Required, Optional, Nit; reviewer receives no success
    assertion.
  - Verify: active-dispatch exclusion proof, dispatch validation, unchanged
    subject hashes after creation, and structured reviewer return.

- [ ] Remediate every valid finding and obtain a different-fresh CLEAN review.
  - Acceptance: no waiver; behavior remedies return to RED; full proof reruns;
    final reviewer is distinct and exact final subject is CLEAN.
  - Verify: immutable remediation proof/dispatch chain and final fingerprint.

- [ ] Create the primary implementation commit from unchanged CLEAN bytes.
  - Acceptance: staged subject exactly replays final manifest; all gates and
    preservation checks pass; handoff/ledger closeout is excluded.
  - Verify: staged diff/manifest, GitNexus detect-changes, commit inspection.

- [ ] Create the parent-owned completed handoff and deterministic ledger-only
      closeout commit.
  - Acceptance: handoff binds the primary commit/CLEAN subject, preserves open
    gates, gives one exact next selector, and does not start later work; only
    handoff/ledger/index closeout files enter commit two.
  - Verify: all three handoff modes, ledger rebuild equality, ancestry, final
    clean status, preservation equality, and exact handoff parse command.
