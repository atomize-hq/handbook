# HCM-2.4 P3B fixture-contract selector repair

Status: **proposed exact fixture-proof expansion; implementation paused for
fresh review**

Date: 2026-07-27

## Trigger

The first P3B selector review was CLEAN for the four declared paths, but the
first implementation test still refused:

```text
REASON: invalid_capture_input: Stage 10 outputs must declare the
descriptor-selected Work Specification YAML and its deterministic Markdown view
```

Root-cause tracing established that `install_foundation_inputs_repo` and
`install_foundation_flow_demo_repo` recursively copy their committed fixture
repositories. Both fixture repositories still contain the pre-P3 Stage 10
stage, directive, and template. They also lack committed repository-profile
selection for the required Work Specification descriptor. The untracked
profile files observed in the live worktree are exact candidate fixture inputs,
not proof that a clean checkout contains them.

The prior four-path selector is therefore insufficient. No production defect
or broader runtime authority is implied.

## Exact additive fixture selector

In addition to the original P3B selector, editable paths are exactly:

- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/stages/10_feature_spec.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/library/feature_spec/feature_spec_architect_directive.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/library/feature_spec/FEATURE_SPEC.md.tmpl`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/profile-selection.json`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `tests/fixtures/foundation_flow_demo/repo/core/stages/10_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/repo/core/library/feature_spec/feature_spec_architect_directive.md`
- `tests/fixtures/foundation_flow_demo/repo/core/library/feature_spec/FEATURE_SPEC.md.tmpl`
- `tests/fixtures/foundation_flow_demo/repo/.handbook/profile-selection.json`
- `tests/fixtures/foundation_flow_demo/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`

The three `core/` assets in each fixture must be byte-identical copies of their
reviewed P3 canonical counterparts. The two `.handbook/` files in each fixture
must be byte-identical across the two fixture roots and may select only the
repository-scoped Work Specification descriptor over shipped-root `1.2`.

Generated capture outputs under either fixture repository remain transient and
must not be committed.

## Reviewability and non-goals

This repair adds no production symbol, released profile, schema, public API,
dependency, Cargo/version, unsafe-policy, or sibling-slice surface. The ten
paths are immutable test-fixture inputs consumed by the same eight CLI tests.
Their only semantic purpose is to make a clean checkout reproduce the already
reviewed P3 Stage 10 contract.

No support helper, pipeline runtime, engine runtime, compiler runtime, main
`core/` authority, output fixture, or non-Stage-10 CLI test is selected.

## Entry and exit

Entry requires a fresh built-in review accepting this exact additive selector.

Exit retains the original P3B proof wall and additionally requires:

- both fixture copies are byte-identical where declared;
- both fixture roots work after transient generated outputs are absent; and
- `git status --short` shows no unselected fixture-repo debris staged.
