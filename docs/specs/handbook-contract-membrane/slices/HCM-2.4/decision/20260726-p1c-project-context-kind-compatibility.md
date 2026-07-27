# P1C Project Context kind compatibility correction

Status: accepted predicate authority repair; Unix proof correction pending
fresh review.

## Conflict

P1C must select `handbook.profile.shipped-root@1.2.0`, whose exact
`project_context` descriptor selects
`handbook.artifact-kind.project-context@1.1.0` over
`handbook.schemas.artifacts.project-context@1.0.0` at
`.handbook/project/context.yaml`.

The same packet requires the complete HCM-2.1 preservation wall, but
`project_context_artifact::selected_contract_matches` accepts only kind
`handbook.artifact-kind.project-context@1.0.0`. After the otherwise exact
`shipped_profile_request` adoption, 9 of 12 HCM-2.1 tests refuse with
`SelectedContractMismatch`. The P1C production selector did not include that
predicate, so the contract was not implementable without an authority repair.

## Decision

Authorize only the existing private `selected_contract_matches` predicate and
its existing unit test to treat Project Context kind `1.0` and kind `1.1` as
an exact compatibility pair when:

- decision and instance kind refs are byte-equal;
- the selected schema remains exact Project Context schema `1.0`; and
- the canonical path remains `.handbook/project/context.yaml`.

Every other kind, schema, path, mismatch, prefix, range, or fallback continues
to refuse. No public signature/type, schema, profile, definition, dependency,
Cargo surface, new production symbol, or sibling behavior is authorized.

The later P1C discovery review found that two Unix-only tests project the live
successor decisions but still assert kind `1.0`. Direct WSL RED proof also
showed the compiler test still expects doctor report schema `1.1.0` and the CLI
test installs an obsolete inline Charter schema `1.0`. Authorize only:

- compiler test
  `crates/compiler/tests/doctor.rs::doctor_api_projects_the_exact_stable_project_context_row`
  report schema expectation `1.1.0` to `1.2.0` and Project Context kind
  expectation `1.0.0` to selected `1.1.0`; and
- CLI test
  `crates/cli/tests/cli_surface.rs::doctor_reports_ready_when_required_artifacts_present`
  obsolete inline Charter schema `1.0` setup to the existing
  `write_valid_selected_charter` helper and Project Context kind expectation
  `1.0.0` to selected `1.1.0`.

No helper implementation, fixture asset, other setup, other assertion, or
production edit is included.

## Evidence and risk

The live failure is deterministic: engine library proof passes 154/154, then
the HCM-2.1 target fails 9/12 solely at the fixed kind guard. GitNexus 1.6.9
cannot resolve the private predicate exactly, but its owning file has exact
LOW upstream impact: 8 impacted files, 3 direct, zero processes and zero
modules. The CRITICAL `shipped_profile_request` and
`resolve_shipped_profile_decisions` ceilings remain unchanged.

## Proof

Before P1C can close:

- before the predicate edit, its existing unit test must retain the exact
  `1.0` positive and fail on the new exact `1.1` positive;
- after the predicate edit, that unit test must accept both exact versions and
  reject both crossed directions, unrelated identity, unlisted version,
  prefix, range, `latest`, bare/fallback, wrong schema, and wrong path values;
- unchanged `hcm_2_1_project_context` bytes must return to 12/12, enforced by
  a scoped zero-diff/hash check;
- the P1C selected-profile, P1B Charter, complete HCM-2.2, HCM-2.1, and
  HCM-2.3 preservation walls must pass; and
- a fresh independent review must accept this authority repair before the
  predicate is edited;
- both exact Unix projection tests must pass with kind `1.1`, using WSL and an
  isolated Linux target directory; and
- the Unix
  `doctor::tests::doctor_nulls_project_context_and_is_invalid_after_substitution_or_inode_aba`
  refusal must remain green before P1C closure review.
