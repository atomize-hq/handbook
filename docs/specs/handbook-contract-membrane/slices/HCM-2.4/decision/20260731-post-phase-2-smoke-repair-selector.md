# HCM-2.4 post-Phase-2 smoke-repair selector

Status: planning findings remediated; production edits remain gated on a
different-fresh CLEAN closure review.

## Selection identity

- phase: `HCM-2`
- slice: `HCM-2.4`
- packet ID: `20260731-post-phase-2-smoke-repair-selector`
- integrated outcome ID: `hcm-2.4-post-phase-2-smoke-repair`
- parent orchestration ID:
  `20260731T182849Z--HCM-2-4--post-phase-2-smoke-repair`
- causal budget ID:
  `sha256:27a404e0d9edf04c02a1aa0ac67180c2c341c6e05701267c29d2c19635243e47`
- outcome-registry fingerprint:
  `sha256:1ed5542bcc08a954c32421633ae8d1667e2db5b6ddd7351c169ac97d734e447c`
- source handoff:
  `20260731T135635Z--HCM-2-4--orchestration--administrative-root-status-synchronized`

The registry is frozen as the canonical-JSON-newline encoding of this single
outcome:

```json
[{"authority_ref":"docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260731-post-phase-2-smoke-repair-selector.md","integrated_outcome_id":"hcm-2.4-post-phase-2-smoke-repair","packet_ids":["20260731-post-phase-2-smoke-repair-selector"]}]
```

This is a new, bounded post-exit product-smoke correction. The completed P7
handoff, administrative status synchronization, and formal Phase 2 exit remain
immutable evidence. No P7 result is invalidated or rewritten. `HCM-3.x`,
published JSON protocol work, SDK/transport work, release, push, and automatic
continuation remain unauthorized.

## Live boundary and reproduced observations

Live preflight verified branch `feat/handbook-contract-membrane` at
`55d220a9ec41cc34e7b42273f23ff90c529f7339`. The GitNexus index was refreshed
to that HEAD; graph analysis is available while its FTS/BM25 search backend is
not. The nine operator-owned paths named in the request were hashed and remain
outside this packet.

Fresh disposable Git repositories reproduced the following current-HEAD
behavior:

1. `handbook setup` creates `.handbook/repository-identity.v1` and runtime lock
   state, returns `ACTION_REQUIRED`, and authors no canonical content or profile
   selection.
2. Both `handbook inspect` and `handbook generate` then return
   `SystemRootMissing` with `RunSetup`; repeating setup preserves the same loop.
3. A truly absent root produces the same typed missing-root refusal.
4. `artifact list-kinds --json` and `artifact list-instances --json` use empty
   stdout and plain stderr before safe identity/selection establishment.
5. The current generic custom-kind fixture succeeds for both list operations
   after its explicit profile selection is combined with `setup` identity.
6. Project Context structured input validation succeeds on Windows, while
   strict mutation refuses with `UnsupportedPlatformStrictMutation` and points
   to a supported Unix host.

The root-cause trace is narrow: setup safely establishes repository identity,
but canonical root detection currently recognizes only selected canonical
artifact paths. Flow refusal ordering therefore selects the missing-root/setup
action before it can report the two missing required artifacts.

## A. Setup and packet decision

The selected state contract is:

| State | Required result |
|---|---|
| No `.handbook` root | `SystemRootMissing`; setup is the next safe action. |
| Setup-created operational root with required canonical artifacts absent | Report missing required Project Authority and Project Context with author/fill actions; never direct the operator back to setup. |
| Valid required canonical artifacts | Continue normal packet-readiness evaluation. |
| Legacy Markdown-only tree | Do not establish the canonical root and do not influence canonical truth. |
| Unsafe, symlink, or non-directory root | Preserve existing fail-closed behavior. |

Three mechanisms were compared:

1. **Recognize a safely established operational root — selected.** Treat a
   strictly validated `.handbook/repository-identity.v1` created by setup as
   sufficient operational-root evidence, while retaining selected canonical
   artifact paths as the other establishment route. Missing, malformed,
   symlinked, non-regular, or otherwise unsafe identity evidence does not
   establish the root. This reuses the existing no-follow identity boundary and
   changes neither content truth nor setup mutations.
2. **Create a non-content canonical namespace during setup — rejected.** An
   empty `.handbook/project` directory would happen to satisfy current
   scaffold detection, but would make setup mutate a content-owned namespace
   solely to signal operational state and would introduce a new observable
   setup artifact.
3. **Introduce an empty-root status/action — rejected.** A new status or next
   action would spread through engine, flow, compiler, CLI rendering, and
   potentially public result surfaces when existing missing-artifact refusals
   already express the correct operator action.

Setup remains content-non-authoring. It may not create Charter, Project
Context, Environment Context, `.handbook/profile-selection.json`, legacy
Markdown truth, or an empty canonical content namespace. Explicitly selected
non-Git setup retains its current exit code `1`/`ACTION_REQUIRED` behavior.

## B. Artifact `--json` adjudication

The active HCM-2.3 contract freezes machine-readable success results and typed
engine/domain mutation results as a local CLI fixture contract. Its tests
assert prose stderr for pre-establishment list failures and do not freeze a
CLI-local JSON error envelope for those failures. Creating an exact envelope
now would invent schema identity, versioning, fields, and compatibility
semantics adjacent to the Phase 4 public protocol/DTO boundary.

Decision: defer the plain-stderr/empty-stdout `--json` pre-establishment gap.
Make no artifact runtime change. The runbook must state the current behavior;
focused tests must retain established successful JSON listing and the existing
missing/unsafe selection and identity refusals. No profile selection is ever
generated by setup.

## C. Minimal manual smoke journey

Add one concise disposable-directory runbook. It must use repository-relative
source references and must not mutate the source checkout. It freezes these
steps and expectations:

1. initialize a fresh Git repository; run setup and doctor; confirm required
   canonical artifacts remain action-required;
2. validate
   `tools/fixtures/project_context_inputs/runtime_smoke_valid.yaml`, then show
   the expected Windows-only mutation refusal without widening platform
   authority;
3. run the exact current closed-intake proof
   `cargo test -p handbook-cli --test hcm_2_2_product_cutover
   author_intent_is_evaluated_but_fails_closed_without_lineage_persistence --
   --exact`, then run approver bootstrap against the setup repository and show
   the expected native authenticator-unavailable boundary with no
   current-operation content mutation;
4. copy
   `crates/engine/tests/fixtures/hcm_2_3_generic_custom_kind`, run setup to
   establish identity, then run both generic listing commands with
   `--repository-root` and `--json`;
5. show incomplete packet behavior in the setup repository, then reproduce
   prepared packet behavior with the two exact disposable integration proofs
   `cargo test -p handbook-cli --test cli_surface
   inspect_reports_ready_when_required_artifacts_present -- --exact` and
   `cargo test -p handbook-cli --test cli_surface
   generate_emits_real_packet_body_when_ready -- --exact`; their preparation
   source is `planning_ready_repo` in `crates/cli/tests/cli_surface.rs` and the
   test-only committed-Charter authority helper in
   `crates/engine/tests/support/hcm_2_2_committed_charter.rs`;
6. copy `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo`, then run
   `pipeline list`, `pipeline show --id pipeline.foundation_inputs`, and
   `pipeline resolve --id pipeline.foundation_inputs`, documenting that
   compile/capture require the persisted route basis and declared stage inputs.

Named failing observations are `SMOKE-ROOT-LOOP` (setup cannot escape
`RunSetup`) and `SMOKE-CONSUMABILITY` (no one current disposable journey binds
the shipped fixture, native-boundary, generic, packet, and pipeline
prerequisites). The JSON observation is `SMOKE-JSON-PREESTABLISHMENT` and is
explicitly deferred above, not silently accepted.

## D. Exact scope ceiling

Production Rust paths, exactly 4 of the permitted 5:

1. `crates/engine/src/canonical_artifacts.rs`
2. `crates/engine/src/repository_invocation_identity.rs`
3. `crates/flow/src/resolver.rs`
4. `crates/compiler/src/blocker.rs`

Non-control test, fixture, and help paths, exactly 6 of the permitted 12 and at
most 600 changed lines in aggregate:

1. `crates/engine/tests/canonical_artifacts_ingest.rs`
2. `crates/cli/tests/setup_cli.rs`
3. `crates/flow/tests/resolver_core.rs`
4. `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/post-phase-2-smoke-runbook.md`
5. `crates/cli/tests/cli_surface.rs`
6. `crates/compiler/tests/author.rs`

Control/proof paths may be limited to this selector, new-parent v1.4 review
dispatches, the exact post-exit status/evidence rows in `00-README.md`,
`03-seam-crosswalk.md`, `04-phase-slice-map.md`, and
`06-proof-and-regression-ledger.md`, and one compact smoke-repair proof record
if raw proof cannot fit clearly in the ledger row. The closeout commit may add
only the one parent-owned v1.4 handoff record, deterministic ledger rebuild,
and exact P3/P4 inventory registration if a review actually requires it.

No Cargo, dependency, public API, published schema, SDK, transport, native
adapter, historical proof/dispatch/handoff/transcript, or HCM-3.x path is in
scope. If implementation requires another production path, another
non-control path, more than 600 ancillary changed lines, or a new protocol
surface, stop with an authority-required handoff.

### Authority-resumption amendment

The explicit resumption from handoff
`20260731T194418Z--HCM-2-4--orchestration--post-phase-2-smoke-repair-authority-blocked`
adds only `crates/cli/tests/cli_surface.rs`. The existing
`generate_blocks_invalid_required_charter_with_required_artifact_invalid`
fixture must place its unchanged invalid bytes at canonical
`.handbook/project/charter.yaml`, preserving its `RequiredArtifactInvalid`
assertions. It must not establish repository identity as a substitute because
that would exercise a missing canonical Charter rather than an invalid one.

The explicit resumption from handoff
`20260731T221742Z--HCM-2-4--orchestration--post-phase-2-smoke-repair-second-authority-stop`
adds only `crates/compiler/tests/author.rs`. The shared
`legacy_authoring_fixture_repo` must establish valid operational repository
identity through
`RepositoryInvocationIdentityServiceV1::initialize_for_setup`, without
handcrafting identity bytes or changing the existing legacy starter-template
bytes, authoring assertions, output paths, independent missing-root tests, or
invalid-root tests.

## Acceptance contract

RED must first prove current HEAD's post-setup `SystemRootMissing`/`RunSetup`
loop. GREEN must prove all of the following:

1. a safely setup-created root reports missing Project Authority and Project
   Context with author/fill actions for both inspect and generate, never
   `RunSetup`;
2. a truly absent root remains `SystemRootMissing`;
3. valid required canonical artifacts retain normal readiness evaluation;
4. legacy Charter/Project Context Markdown neither establishes root nor
   influences output;
5. setup authors no canonical content or profile selection;
6. explicit non-Git setup behavior remains unchanged;
7. unsafe, symlink, and non-directory roots and identity evidence remain
   fail-closed;
8. generic list operations succeed with explicit selection and safe identity;
9. generic operations still refuse missing or unsafe selection/identity;
10. pre-establishment artifact `--json` behavior remains unchanged and is
    recorded as deferred, while established JSON success remains valid; and
11. the manual runbook reproduces every selected journey from disposable
    directories without source-checkout mutation.

Before editing either production function or equivalent symbol, run upstream
GitNexus impact analysis and report direct callers, affected processes, total
impact, and risk. Warn before any HIGH/CRITICAL edit. Verification must include
focused engine, flow, compiler, CLI, setup/doctor, artifact, packet, and
fixture tests; proportional workspace tests; strict Clippy; formatting;
`git diff --check`; legacy-influence scans; and an installed-binary smoke in
fresh disposable directories.

## Planning-finding remediation

Discovery run `/root/hcm_2_4_smoke_planning_review` returned two P2 findings
against the original selector fingerprint. Their ownership remains immutable:

- `HCM24-PSRPR-0001` is accepted. Operational-root recognition alone would
  expose stale missing-artifact `RunSetupRefresh` actions. The production
  ceiling now includes the exact flow and compiler blocker paths. Missing
  required artifacts must use the existing instance-aware author/fill action
  helper in all selected blocker/refusal branches; true root absence and unsafe
  root status retain `RunSetup`.
- `HCM24-PSRPR-0002` is accepted. The vague prepared-packet phrase is replaced
  by two exact, existing disposable CLI integration proofs and their named
  preparation/helper sources. The Charter intake step likewise names the exact
  current closed-envelope proof. This removes the proposed new Charter fixture
  and its test edit, reducing the changed ancillary surface while keeping the
  installed-binary setup, native-boundary, generic, incomplete-packet, and
  pipeline smoke steps disposable.

The remediation is consolidated in this selector. Its different fingerprint
must receive a different-fresh `planning` closure review under the unchanged
parent, outcome registry, and causal budget before RED tests or production
edits.

## Review and stop rules

The first v1.4 dispatch enters the `planning` stage and reviews this selector
read-only. A CLEAN verdict authorizes RED tests and the bounded implementation;
FINDINGS require consolidated selector remediation and a different-fresh
closure under the same parent and causal budget. Later reviews enter
`implementation` and then `final_closeout`, each with one discovery lineage and
only protocol-permitted causal supplements. Every material remediation changes
the subject fingerprint and receives a different-fresh review.

The parent true-stops only after a complete CLEAN subject, a reviewed primary
local commit, one completed or blocked parent-owned handoff, deterministic
ledger rebuild, and separate mechanical closeout commit. Exhausted causal
allowance, scope-ceiling breach, unavailable mandatory proof, or broader
authority produces a truthful blocked handoff. Nothing is pushed.
