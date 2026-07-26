# HCM-2.4 Charter compatibility planning amendment proof

Recorded: 2026-07-26T12:26:26Z
Mode: planning amendment only
Selected predecessor handoff:
`20260726T050447Z--HCM-2-4--orchestration--planning-completed`

## Baseline

The live baseline reproduced the selected handoff exactly:

- branch: `feat/handbook-contract-membrane`;
- HEAD: `fa31f65fddd678b33ea89d6b7e41201240254986`;
- worktree: clean;
- origin ahead/behind: `0/0`; and
- GitNexus index: current at the same commit, 1,451 files, 18,593 symbols,
  40,260 edges, 440 clusters, and 300 processes.

The GitNexus repository alias reported by the installed CLI is `handbook`.
Full-text query is unavailable because the local FTS extension cannot load.
Exact context, upstream impact, and direct source inspection remained available
and are the evidence used below. No impact result was UNKNOWN.

## Independently reproduced P0 stop

The live call path is:

```text
resolve_shipped_profile_decisions
  -> resolve_profile_selection(repo_root, shipped_profile_request())
  -> ResolvedProfileDecisions::from_profile
  -> load_shipped_charter_definition_registry()
  -> CharterDefinitionRegistry::validate_selected_decisions
```

Exact live evidence:

- `resolve_shipped_profile_decisions`, lines 160–173 of
  `crates/engine/src/profile_decision.rs`, resolves the request and immediately
  validates the result through the shipped Charter registry;
- `shipped_profile_request`, lines 309–370 of that file, selects only
  `handbook.profile.shipped-root@1.1.0`;
- `CharterDefinitionRegistry::validate_selected_decisions`, lines 118–162 of
  `crates/engine/src/charter_definition_registry.rs`, rejects every selected
  profile ref other than `handbook.profile.shipped-root@1.1.0`; and
- the existing P1B selector allowed only `shipped_profile_request`, so changing
  that request to 1.2 would be rejected by the next live call.

The previous P1B single-function ceiling is therefore false. The stop was
correct and no implementation edit was authorized.

## HCM-2.2 authority boundary

The released 1.1 Project Authority descriptor is frozen as:

| Field | Exact value |
| --- | --- |
| descriptor schema | `handbook.artifact-instance-descriptor@1.0` |
| id | `project_authority` |
| kind | `handbook.artifact-kind.project-authority@1.1.0` |
| role | `constitutional_authority` |
| capabilities | exactly [`constitutional_root`] |
| label | `Charter` |
| canonical path | `.handbook/project/charter.yaml` |
| requiredness | `always`; `condition_ref: null` |
| dependencies | empty |
| lifecycle | `handbook.lifecycle.constitutional-review-lock@1.0.0` |
| intake | `handbook.intake.charter@1.0.0` |
| renderers | exactly [`handbook.renderer.charter-review-markdown@1.0.0`] |
| Projections | empty |
| validation overlays | empty |
| extensions | empty |

The selected HCM-2.2 Charter record identity remains:

- profile ref: `handbook.profile.shipped-root@1.1.0`; and
- profile definition fingerprint:
  `sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74`.

The compatibility amendment does not create a new Charter record identity.
Candidate, intake, lifecycle-result, promotion, promotion-intent, approval,
lineage, transaction, and committed-authority records continue to carry that
exact pair and retain their existing schemas, fingerprints, validation rules,
and negative cases.

## Selected compatibility design

The smallest fail-closed design uses the existing
`CharterDefinitionRegistry::validate_selected_decisions` boundary and keeps its
signature and public API unchanged.

The boundary must:

1. accept the exact 1.1 ref/fingerprint pair exactly as before;
2. accept the exact 1.2 ref only with the literal authored profile fingerprint
   produced and frozen by P1A;
3. reject every other ref/fingerprint tuple, including 1.3, a higher compatible
   semver, a same-ref wrong fingerprint, and any range or prefix match;
4. for both accepted tuples, compare every resolved `project_authority` field
   in the table above, relying on the closed descriptor loader for the exact
   descriptor schema id/version and unknown-field refusal;
5. require the exact subordinate Charter definitions to be present in the
   frozen shipped Charter registry; and
6. return the existing `UnsupportedDependency` refusal for any mismatch.

P1A must record the literal 1.2 profile fingerprint in its immutable definition
vector before compatibility work begins. A placeholder, runtime “newer than”
comparison, unbounded semver check, loose string exception, repository-selected
substitute, fallback resolution, second profile read, or dual-read path is RED.

The generic selected decisions remain 1.2 for HCM-2.4 Project Context and
Environment Context work. Charter record-producing and currentness checks use
the already-frozen HCM-2.2 record identity constants after the registry boundary
passes. The existing lifecycle constants may become `pub(crate)` without value
changes so the exact pair is shared internally. This is an internal
compatibility membrane, not a migration, alias, or public compatibility promise.

The necessary order is:

```text
P1A exact 1.2 definition + literal fingerprint
  -> P1B Charter compatibility foundation while shipped selection remains 1.1
  -> P1C shipped-root 1.2 adoption through shipped_profile_request
```

## Exact live production symbols and impacts

All impacts are fresh upstream results with tests, depth 3, at the baseline
commit.

| Exact GitNexus UID | Impact | Amendment disposition |
| --- | --- | --- |
| `Function:crates/engine/src/profile_decision.rs:resolve_shipped_profile_decisions` | CRITICAL: 219 impacted, 74 direct, 8 processes, 13 modules | Read/proof anchor only; signature and immediate registry validation remain unchanged |
| `Function:crates/engine/src/profile_decision.rs:shipped_profile_request` | CRITICAL: 155 impacted, 1 direct, 7 processes, 10 modules | P1C exact source/ref adoption only |
| `Function:crates/engine/src/charter_definition_registry.rs:CharterDefinitionRegistry.validate_selected_decisions#1` | LOW graph result: 0 impacted | P1B exact tuple/full-descriptor boundary; graph edge is incomplete because live source has six current production callers, and P1B adds exactly one seventh caller from `evaluate_charter_intake` |
| `Const:crates/engine/src/charter_lifecycle_validation.rs:SELECTED_PROFILE_REF` | LOW: 0 impacted | Value immutable; `pub(crate)` visibility only |
| `Const:crates/engine/src/charter_lifecycle_validation.rs:SELECTED_PROFILE_FINGERPRINT` | LOW: 0 impacted | Value immutable; `pub(crate)` visibility only |
| `Function:crates/engine/src/charter_intake.rs:evaluate_charter_intake` | HIGH: 42 impacted, 6 direct, 1 process, 4 modules | Validate compatibility and continue emitting HCM-2.2 profile identity |
| `Function:crates/engine/src/charter_approval_workflow.rs:validate_candidate_currentness` | HIGH: 17 impacted, 1 direct, 1 process, 3 modules | Compare candidate with frozen HCM-2.2 identity after caller boundary validation |
| `Function:crates/engine/src/charter_promotion_workflow.rs:validate_candidate_contract` | LOW: 2 impacted, 1 direct, 1 process, 1 module | Compare retained fingerprint with frozen HCM-2.2 identity |
| `Function:crates/engine/src/charter_promotion_workflow.rs:CharterPromotionWorkflowServiceV1.promote_at#2` | LOW: 1 impacted, 1 direct, 0 processes, 1 module | Remove the false selected-profile-1.1 gate, retain registry validation, emit frozen HCM-2.2 identity |
| `Function:crates/engine/src/charter_lifecycle_validation.rs:build_result` | LOW: 1 impacted, 1 direct, 1 process, 1 module | Emit frozen HCM-2.2 identity |
| `Function:crates/engine/src/charter_lifecycle_validation.rs:validate_definition_authority` | LOW: 2 impacted, 2 direct, 1 process, 1 module | Replace the direct selected-profile-1.1 gate with the exact registry boundary |
| `Function:crates/engine/src/charter_authority_transaction.rs:CharterAuthorityTransactionServiceV1.preflight#1` | HIGH: 27 impacted, 2 direct, 1 process, 3 modules | Compare promotion authority with frozen HCM-2.2 identity after registry validation |
| `Function:crates/compiler/src/doctor.rs:doctor_report_from_inspection` | HIGH: 12 impacted, 2 direct, 1 process, 3 modules | Existing sixth caller; read/proof-only, preserve exact Charter definition-closure reporting |

The six live `validate_selected_decisions` callers are
`resolve_shipped_profile_decisions`,
`CharterApprovalServiceV1::approve_inner`,
`CharterPromotionWorkflowServiceV1::promote_at`,
`validate_definition_authority`, and
`CharterAuthorityTransactionServiceV1::preflight`, plus compiler
`doctor_report_from_inspection`. The graph's zero upstream count does not
override those source-proved edges. P1B explicitly adds a seventh caller from
`evaluate_charter_intake` so the public intake function rejects
invalid decisions before producing intake/candidate evaluations. The exact new
integration target
`crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs` must include
`invalid_compatible_profile_decisions_cannot_produce_charter_intake`.

Fresh preservation anchors are:

- `Function:crates/engine/src/charter_lineage_store.rs:validate_candidate_v13`:
  CRITICAL, 43 impacted, 14 processes; and
- `Function:crates/engine/src/charter_promotion_intent_v12.rs:validate_promotion_intent_v12`:
  HIGH, 18 impacted, 3 processes.

Those two symbols are no-edit anchors. Their exact 1.1 identity checks and every
existing negative remain unchanged. A proposed edit to either is RED_STOP.

Current impact ceilings are the exact process/module counts above. P1B may add
the one planned seventh intake caller and P0 may add only explicitly manifested
new test callers. An eighth production caller, new production process/module,
authority class, public surface, or materially wider HIGH/CRITICAL result
requires a parent-owned same-scope planning correction and stops the active
packet.

## Required proof

P1B/P1C must add and preserve:

- direct 1.1 acceptance with the released bytes and fingerprint unchanged;
- exact 1.2 compatibility acceptance only after the P1A literal fingerprint is
  frozen;
- per-field mutation rejection for id, kind, role, capabilities, label, path,
  requiredness, condition, dependencies, lifecycle, intake, renderer
  cardinality/ref, Projections, validation overlays, extensions, and descriptor
  schema closure;
- wrong 1.2 fingerprint, unlisted 1.3, range/prefix, missing subordinate
  definition, fallback, and second-profile-read rejection;
- byte/fingerprint equality for HCM-2.2 intake, candidate, lifecycle result,
  approval, promotion, intent, lineage, transaction, recovery, replay, and
  committed authority under direct 1.1 and compatible selected 1.2 decisions;
- unchanged `validate_candidate_v13` and `validate_promotion_intent_v12`
  negatives;
- a real shipped path proving `resolve_shipped_profile_decisions` selects 1.2,
  generic consumers observe 1.2, and Charter records still contain the frozen
  1.1 pair; and
- the complete HCM-2.2 engine/compiler/CLI focused wall plus HCM-2.1 and
  HCM-2.3 preservation.

The exact compatibility/preservation command wall is:

```text
cargo test -p handbook-engine --test hcm_2_4_charter_profile_compatibility
cargo test -p handbook-engine --test hcm_2_2_approval_use
cargo test -p handbook-engine --test hcm_2_2_authenticator_security
cargo test -p handbook-engine --test hcm_2_2_authority_repair
cargo test -p handbook-engine --test hcm_2_2_authority_workflow
cargo test -p handbook-engine --test hcm_2_2_charter_observation
cargo test -p handbook-engine --test hcm_2_2_charter
cargo test -p handbook-engine --test hcm_2_2_definition_profile
cargo test -p handbook-engine --test hcm_2_2_definition_registry
cargo test -p handbook-engine --test hcm_2_2_intake
cargo test -p handbook-engine --test hcm_2_2_lifecycle_store
cargo test -p handbook-engine --test hcm_2_2_lifecycle
cargo test -p handbook-engine --test hcm_2_2_lineage_store
cargo test -p handbook-engine --test hcm_2_2_promotion_workflow
cargo test -p handbook-engine --test hcm_2_2_runtime_vectors
cargo test -p handbook-engine --test hcm_2_2_test_surface
cargo test -p handbook-engine --test hcm_2_2_transaction_promotion
cargo test -p handbook-engine --lib --all-features
cargo test -p handbook-compiler --test hcm_2_2_c04_version
cargo test -p handbook-compiler --test hcm_2_2_product_cutover
cargo test -p handbook-cli --test hcm_2_2_product_cutover
cargo test -p handbook-cli --test hcm_2_2_skill_assets
cargo test -p handbook-compiler doctor::tests::doctor_nulls_project_context_and_is_invalid_after_substitution_or_inode_aba -- --exact
cargo test -p handbook-cli --test cli_surface profile_setup_and_doctor_use_typed_rows_json_and_exit_policy -- --exact
cargo test -p handbook-engine --test hcm_1_2_selected_kinds
cargo test -p handbook-engine --test hcm_1_2_unselected_kinds
cargo test -p handbook-engine --test hcm_1_4_profile_decisions
cargo test -p handbook-engine --test hcm_1_4_profile_inspection
cargo test -p handbook-engine --test hcm_2_1_project_context
cargo test -p handbook-compiler --test hcm_2_1_c04_version
cargo test -p handbook-engine --test hcm_2_3_generic_lineage
cargo test -p handbook-engine --test hcm_2_3_registration_kernel
cargo test -p handbook-engine --test hcm_2_3_selection_request
cargo test -p handbook-cli --test hcm_2_3_artifact_cli
```

P1B and P1C run the applicable focused commands after RED/GREEN, then the whole
wall before packet closeout. Any renamed, missing, ignored, filtered, weakened,
or deleted HCM-2.2 negative is RED.

## Bounded P2 selector correction

Live CLI help still embeds the superseded Environment Inventory Markdown path
at `crates/cli/src/main.rs:124`. The exact live symbol is
`Enum:crates/cli/src/main.rs:AuthorCommand`; fresh upstream impact is LOW with
zero graph edges, and direct help/snapshot evidence makes that graph result
incomplete.

The P2 manifest must add only:

- production: `crates/cli/src/main.rs`, exact
  `AuthorCommand::EnvironmentInventory` help text;
- test: `crates/cli/tests/cli_surface.rs`, exact inline
  `author_help_matches_snapshot` and snapshot-consuming
  `author_environment_inventory_help_matches_snapshot`; and
- snapshot:
  `crates/cli/tests/snapshots/handbook-author-environment-inventory-help.txt`.

These are direct consumers of the path change already authorized by P2. This
correction does not rename the command, change its arguments, widen setup or
doctor scope, or alter any other P2–P7 decision.

## True stop

This proof authorizes planning amendment and review only. Implementation still
requires a new explicit top-level selection of the completed amended planning
handoff. That later selection enters read-only P0 first and authorizes no edit
until the amended inventory/impact gate is frozen and passes.
