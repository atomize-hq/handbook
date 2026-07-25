# HCM-2.3 implementation Review 6 remediation

## Authority

This remediation is bounded to the five actionable findings returned by fresh
complete-subject Review 6 and the operator's explicit additional authorization
for the CRITICAL shared-budget compatibility repair. It does not authorize
HCM-2.4, Phase 3+, Phase 4 SDK/transport work, a public budget API, widened byte
limits, frozen-stage reordering, or changes to
`SchemaRegistry::load_admitted_deferred_fingerprints`.

The shared-budget repair must preserve both public compatibility entries,
transfer an owned `SourceByteBudget` through the internal path, charge every
selected source exactly once, and restrict the budget-aware resolver caller to
the HCM-2.3 artifact-repository path.

## Review 6 findings accepted

1. CRITICAL canonical-publication lost-update race.
2. Durable replay identity incorrectly includes derived current context.
3. Null-intake operations erase typed `not_applicable` and can persist refusal.
4. Fixed selection uses separate profile/schema and intake 8 MiB budgets.
5. Ordinary artifact validation reports unevaluated intake as `pass`.

No finding was waived. No implementation commit may be created until a later
fresh complete-subject review is CLEAN.

## Authorized CRITICAL GitNexus impact evidence

### `resolve_profile_selection`

Exact symbol UID:

`Function:crates/engine/src/profile_selection.rs:resolve_profile_selection`

Exact command:

~~~powershell
npx gitnexus impact --uid "Function:crates/engine/src/profile_selection.rs:resolve_profile_selection" --direction upstream --repo "C:\hcm22ar-doc-repair" --branch codex/hcm-2-3-planning --depth 4 --include-tests --limit 500
~~~

Result:

- epistemic status: `exact`;
- risk: `CRITICAL`;
- affected symbols: 445;
- depth 1 / direct callers: 30;
- depth 2: 99;
- depth 3: 126;
- depth 4: 190;
- affected processes: 26;
- affected modules: 20.

Direct callers:

1. `Function:crates/compiler/tests/setup.rs:ready_custom_decisions`
2. `Function:crates/compiler/tests/setup.rs:shipped_selection`
3. `Function:crates/engine/src/artifact_repository.rs:ArtifactRepositoryV1.open_under_authority#2`
4. `Function:crates/engine/src/profile_decision.rs:resolve_shipped_profile_decisions`
5. `Function:crates/engine/src/profile_selection.rs:shipped_profile`
6. `Function:crates/engine/tests/hcm_1_3_artifact_registry.rs:shipped_profile`
7. `Function:crates/engine/tests/hcm_1_3_artifact_registry.rs:custom_profile`
8. `Function:crates/engine/tests/hcm_1_4_profile_decisions.rs:repository_selected_profile_and_source_permutations_produce_identical_decisions`
9. `Function:crates/engine/tests/hcm_1_4_profile_inspection.rs:custom_decisions_inner`
10. `Function:crates/engine/tests/hcm_1_4_profile_inspection.rs:shipped_selection`
11. `Function:crates/engine/tests/hcm_2_3_registration_kernel.rs:complete_fixture_selection_resolves_with_only_its_repository_schema_root`
12. `Function:crates/engine/tests/hcm_2_3_registration_kernel.rs:generic_operation_service_routes_only_by_stable_operation_and_registered_target`
13. `Function:crates/engine/tests/profile_selection.rs:exact_selection_recomputes_the_complete_typed_closure`
14. `Function:crates/engine/tests/profile_selection.rs:shipped_root_and_repository_replace_whole_fixture_are_exact`
15. `Function:crates/engine/tests/profile_selection.rs:built_in_selection_uses_immutable_package_bytes_and_ignores_repo_shadows`
16. `Function:crates/engine/tests/profile_selection.rs:package_owned_refs_and_every_shipped_profile_drift_refuse_repository_bytes`
17. `Function:crates/engine/tests/profile_selection.rs:subset_child_request`
18. `Function:crates/engine/tests/profile_selection.rs:layered_later_owned_request`
19. `Function:crates/engine/tests/profile_selection.rs:shadowed_later_owned_ancestry_fields_refuse_in_both_source_orders`
20. `Function:crates/engine/tests/profile_selection.rs:empty_and_inherited_later_owned_ancestry_fields_remain_supported`
21. `Function:crates/engine/tests/profile_selection.rs:replace_whole_schema_and_kind_fields_return_only_the_winning_literal_sets`
22. `Function:crates/engine/tests/profile_selection.rs:root_and_child_replace_whole_fields_refuse_duplicate_schema_and_kind_refs`
23. `Function:crates/engine/tests/profile_selection.rs:condition_sources_are_derived_only_from_selected_ancestry_descriptors`
24. `Function:crates/engine/tests/profile_selection.rs:vocabulary_role_mismatch_request`
25. `Function:crates/engine/tests/profile_selection.rs:compound_invalid_fixtures_preserve_all_ten_fail_fast_stages`
26. `Function:crates/engine/tests/profile_selection.rs:stage_five_nested_profile_field_decode_precedes_stage_six_cycle`
27. `Function:crates/engine/tests/profile_selection.rs:non_object_profile_records_refuse_at_stage_five_before_unreferenced_source_checks`
28. `Function:crates/engine/tests/profile_selection.rs:stage_five_nested_kind_capability_decode_precedes_stage_six_cycle`
29. `Function:crates/engine/tests/profile_selection.rs:stage_eight_stable_then_schema_and_structural_closure_precedence_are_exact`
30. `Function:crates/engine/tests/profile_selection.rs:stage_eight_compound_fixtures_cover_each_remaining_producer_boundary`

Affected processes:

`doctor::run`, `setup::run`, `preflight_author_environment_inventory`,
`promote_at`, `evaluate_intake_document`,
`every_purpose_boundary_fault_leaves_the_authoritative_stage_absent_or_exact`,
`scan_inventory_store`, `commit_new_locked`, `recover_one_pending`,
`recover_pending`, `record_event_inner`,
`recover_one_approval_authority_journal_locked`, `author::run`, `preflight`,
`evaluate_committed_read`, `load_selected_charter_with_limit`,
`context_resolution_registry::load`, `finalize`, `mutate_inner`,
`recover_approval_authority`, `promote_prepared_locked`,
`recover_establishing`, `expire_retained_result_at_for_testing`,
`read_committed_closure`, `read_committed_authoritative`, and
`observe_committed_approver_registry`.

Affected modules:

`Tests` (343 direct hits), `Cluster_292` (2 direct hits), `Cluster_237` (19
indirect hits), `Author` (12 indirect hits), `Cluster_221` (8 indirect hits),
`Cluster_184` (6 indirect hits), `Cluster_159` (6 indirect hits), `Cluster_1`
(4 indirect hits), `Cluster_216` (3 indirect hits), and the 11 two-hit indirect
clusters `86`, `259`, `269`, `169`, `7`, `215`, `105`, `389`, `408`, `405`,
and `213`.

### `admit_selection_request`

Exact symbol UID:

`Function:crates/engine/src/instance_profile.rs:admit_selection_request`

Exact command:

~~~powershell
npx gitnexus impact --uid "Function:crates/engine/src/instance_profile.rs:admit_selection_request" --direction upstream --repo "C:\hcm22ar-doc-repair" --branch codex/hcm-2-3-planning --depth 4 --include-tests --limit 500
~~~

Result reproduced immediately before the first production edit:

- epistemic status: `exact`;
- risk: `CRITICAL`;
- affected symbols: 259;
- depth 1 / direct callers: 4;
- depth 2: 30;
- depth 3: 99;
- depth 4: 126;
- affected processes: 14;
- affected modules: 20.

Direct callers:

1. `Function:crates/engine/src/instance_profile.rs:request_counts_and_declared_source_identity_fail_before_reads`
2. `Function:crates/engine/src/instance_profile.rs:repository_sources_stop_at_exact_per_source_and_aggregate_sentinels`
3. `Function:crates/engine/src/instance_profile.rs:repository_source_bytes_are_retained_after_the_admission_read`
4. `Function:crates/engine/src/profile_selection.rs:resolve_profile_selection`

Affected processes:

`doctor::run`, `setup::run`, `preflight_author_environment_inventory`,
`promote_at`, `evaluate_intake_document`,
`every_purpose_boundary_fault_leaves_the_authoritative_stage_absent_or_exact`,
`scan_inventory_store`, `preflight`, `evaluate_committed_read`,
`commit_new_locked`, `load_selected_charter_with_limit`,
`context_resolution_registry::load`, `promote_prepared_locked`, and
`recover_one_approval_authority_journal_locked`.

Affected modules:

`Tests` (221 direct hits), `Cluster_281` (3 direct hits), `Author` (8 indirect
hits), `Cluster_237` (3 indirect hits), `Cluster_1` (3 indirect hits),
`Cluster_292`, `Cluster_86`, `Cluster_269`, `Cluster_169`, and `Cluster_7` (2
indirect hits each), plus the one-hit indirect clusters `240`, `259`, `238`,
`184`, `215`, `216`, `196`, `105`, `389`, and `405`.

## Stop guard

Before changing any other existing HIGH or CRITICAL symbol, a public signature,
either byte limit, or frozen resolution-stage order, stop and request explicit
operator authorization. All other edited production symbols still require
their own upstream GitNexus impact analysis before edit.
