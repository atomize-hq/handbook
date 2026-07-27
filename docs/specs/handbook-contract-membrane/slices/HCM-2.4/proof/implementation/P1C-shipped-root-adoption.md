# HCM-2.4 P1C shipped-root adoption proof

Status: discovery remediation and complete packet proof are green;
different-fresh implementation closure review is pending.

## Entry and authority

P1B closed with no P1/P2. P1C initially exposed a contradiction between the
successor Project Context kind `1.1` and the HCM-2.1 fixed kind `1.0` guard.
The parent paused production work, repaired SPEC/plan/todo/path authority, and
obtained a clean different-fresh review at
`sha256:ef12a4559ed7272c911bc81be7a445927e007ffc7e54ee98ddb4d2392cb5a5b0`.

The reviewed correction authorizes only:

- `shipped_profile_request` selecting exact shipped-root `1.2` and its exact
  P1A kind source closure; and
- the existing private Project Context `selected_contract_matches` predicate
  plus its existing unit test, admitting exact kind `1.0` or exact kind `1.1`
  over the unchanged schema `1.0` and canonical path.

`resolve_shipped_profile_decisions` and
`crates/engine/tests/hcm_2_1_project_context.rs` remain zero-byte read/proof
anchors.

## Impact

The frozen exact impact record remains conservative:

- `shipped_profile_request`: CRITICAL, 155 upstream impacts, 1 direct,
  7 processes, 10 modules; and
- `resolve_shipped_profile_decisions`: CRITICAL, 219 upstream impacts,
  74 direct, 8 processes, 13 modules, read/proof-only.

GitNexus 1.6.9 cannot resolve the private compatibility predicate exactly.
Its owning `project_context_artifact.rs` file reports exact LOW impact:
8 upstream files, 3 direct, zero processes and zero modules.

## TDD and implementation

The live shipped-selection test first failed because the resolver returned
`handbook.profile.shipped-root@1.1.0`. After the exact request edit it passed
with profile `1.2`, three root descriptors, exact Project/Environment Context
intake and singleton renderer refs, and released Charter record identity.

The first preservation wall then failed HCM-2.1 9/12 at
`SelectedContractMismatch`. Before editing the predicate, its existing unit
test was expanded and failed on the exact kind `1.1` positive while retaining
the kind `1.0` positive. The predicate now requires decision/instance byte
equality and admits only exact kind `1.0` or exact kind `1.1`; schema and path
checks are unchanged. Its table refuses both crossed directions, unrelated
identity, unlisted version, prefix, range, `latest`, bare/fallback, wrong
schema, and wrong path.

No production symbol, public signature/type, schema, profile definition,
dependency, Cargo surface, fallback, or generic version inference was added.
Work Specification, Decision Record, and Risk Record definitions remain
source-available but unselected.

## Discovery finding and bounded remediation

The complete-subject discovery reviewer found that two Unix-only doctor
projections remained pinned to Project Context kind `1.0`. The parent
validated the finding in WSL before editing. Direct RED proof additionally
showed:

- compiler doctor expected report schema `1.1.0` while the selected report is
  `1.2.0`; and
- the CLI test installed an obsolete inline Charter schema `1.0`, causing
  `OUTCOME: INVALID` before its Project Context assertion.

The first five-document selector repair was rejected with two P2 findings.
After a changed-fingerprint repair, different-fresh authority closure was
CLEAN at
`sha256:6910cdfab15a33c627c7b8f48116dcdce84bebca2bafd6622cc7270d563db080`.
Implementation then changed only three assertion literals and replaced the
obsolete inline setup with the already-existing
`write_valid_selected_charter` helper. No helper implementation, fixture
asset, other assertion/setup, or production code changed.

Exact Linux proof used Ubuntu 24.04 WSL with
`CARGO_TARGET_DIR=/tmp/handbook-p1c-target`:

```text
cargo test -p handbook-compiler --test doctor \
  doctor_api_projects_the_exact_stable_project_context_row -- --exact

cargo test -p handbook-cli --test cli_surface \
  doctor_reports_ready_when_required_artifacts_present -- --exact

cargo test -p handbook-compiler --lib \
  doctor::tests::doctor_nulls_project_context_and_is_invalid_after_substitution_or_inode_aba \
  -- --exact
```

Result: exit 0; 1/1, 1/1, and 1/1 passed. The compiler projection reports
schema `1.2.0` and exact Project Context kind `1.1.0`; the CLI projection uses
released valid Charter authority, remains `INDETERMINATE` only for conditional
Environment Context evidence, and exposes exact kind `1.1.0`; substitution
and inode-ABA refusals remain green.

## Packet proof wall

The converged engine wall was split only because the unchanged HCM-2.3 generic
lineage stress target takes longer than the shorter command budget:

```text
cargo test -p handbook-engine --test hcm_2_3_generic_lineage -- --nocapture
```

Post-remediation result: exit 0; 54 passed in 309.44 seconds.

```text
cargo test -q -p handbook-engine --all-features --lib \
  --test hcm_1_2_selected_kinds \
  --test hcm_1_2_unselected_kinds \
  --test hcm_1_4_profile_decisions \
  --test hcm_1_4_profile_inspection \
  --test hcm_2_1_project_context \
  --test hcm_2_2_approval_use \
  --test hcm_2_2_authenticator_security \
  --test hcm_2_2_authority_repair \
  --test hcm_2_2_authority_workflow \
  --test hcm_2_2_charter \
  --test hcm_2_2_charter_observation \
  --test hcm_2_2_definition_profile \
  --test hcm_2_2_definition_registry \
  --test hcm_2_2_intake \
  --test hcm_2_2_lifecycle \
  --test hcm_2_2_lifecycle_store \
  --test hcm_2_2_lineage_store \
  --test hcm_2_2_promotion_workflow \
  --test hcm_2_2_runtime_vectors \
  --test hcm_2_2_test_surface \
  --test hcm_2_2_transaction_promotion \
  --test hcm_2_3_registration_kernel \
  --test hcm_2_3_selection_request \
  --test hcm_2_4_charter_profile_compatibility
```

Post-remediation result: exit 0; 289 passed, 0 failed, including library 154/154,
HCM-2.1 12/12, P1B compatibility 9/9, and every selected HCM-2.2 target.
Combined engine wall: 343 passed, 0 failed.

Compiler and CLI preservation:

```text
cargo test -q -p handbook-compiler \
  --test hcm_2_1_c04_version \
  --test hcm_2_2_c04_version \
  --test hcm_2_2_product_cutover

cargo test -q -p handbook-cli \
  --test hcm_2_2_product_cutover \
  --test hcm_2_2_skill_assets \
  --test hcm_2_3_artifact_cli

cargo test -q -p handbook-cli --test cli_surface \
  profile_setup_and_doctor_use_typed_rows_json_and_exit_policy -- --exact
```

Post-remediation result: compiler 9/9; CLI 19/19. The separately listed WSL
wall supplies the exact Unix-only compiler projection, CLI projection, and
substitution/ABA proof.

Hygiene and scope:

```text
cargo fmt --all -- --check
git diff --check
git diff --exit-code -- \
  crates/engine/tests/hcm_2_1_project_context.rs \
  crates/engine/src/charter_lineage_store.rs \
  crates/engine/src/charter_promotion_intent_v12.rs \
  docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts
npx gitnexus detect-changes --scope unstaged --repo handbook --limit 200
```

Post-remediation result: all exit 0. The exact production diff remains
confined to `shipped_profile_request` and `selected_contract_matches`. P1C
test deltas are the predicate matrix, selected-profile assertions, three
Unix-only assertion literals, and one existing-helper setup substitution.
GitNexus reproducibly reports LOW, 22 tracked files, and zero affected
processes for the aggregate open slice.
Its changed-symbol enumeration is unstable and is not used as an exact count.

## Boundary

P1C promotes only generic selected-profile truth to shipped-root `1.2`.
Every Charter record, currentness check, promotion, and transaction preflight
retains the released HCM-2.2 profile `1.1` pair. P1C does not implement an
Environment Context writer, Work Specification pipeline, Decision/Risk support
proof, aggregate flow conversion, or any P2-P7 behavior.
