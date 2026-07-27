# HCM-2.4 P3B negative-fixture independence selector repair

Status: **selector closure-review CLEAN; exact test-only implementation
authorized**

Date: 2026-07-27

## Trigger

After all transient fixture-repository outputs were removed, the P3 packet wall
failed only here:

```text
cargo test -p handbook-cli --test pipeline_handoff_refusals

pipeline_handoff_emit_refuses_when_feature_spec_artifact_is_missing
FAILED before product execution:
remove canonical Work Specification fixture: NotFound
```

`foundation_inputs_repo` copies the committed fixture repository. That
repository intentionally contains no post-capture
`artifacts/work-specification/work-specification.yaml`. The test nevertheless
calls `remove_file` on that path and therefore relied on an untracked generated
output being present in the source fixture. This is a test-setup defect, not a
runtime or canonical-authority defect.

## Exact selector amendment

Editable:

- `crates/cli/tests/pipeline_handoff_refusals.rs`, only
  `pipeline_handoff_emit_refuses_when_feature_spec_artifact_is_missing`.

Replace the fallible deletion of
`artifacts/work-specification/work-specification.yaml` with one assertion that
the freshly copied committed fixture does not contain that canonical output.
Keep the command, refusal assertions, exact missing-input reason, and next-safe
action unchanged.

The P3B packet manifest, plan, todo, and proof may record this selector,
review, implementation, and verification result.

## Reviewability and non-goals

This amendment changes one setup assertion in one negative test. The target
test has no callers; GitNexus cannot resolve the test symbol and reports risk
`UNKNOWN`, while live source shows no production call path. The amendment:

- makes the test independent of ignored or untracked generated fixture output;
- preserves the negative case and strengthens the fixture-cleanliness
  precondition;
- does not create or seed canonical YAML before testing its absence; and
- requires the complete four-test refusal suite and P3/P3B walls to pass.

No production/library symbol, support helper, fixture authority input,
generated output, schema, public API, dependency, Cargo/version, unsafe-policy,
P2, P4, P5, P6, or sibling-slice change is authorized.

## Entry and exit

Entry requires fresh built-in review accepting the exact selector.

Exit requires:

- the exact refusal test reaches product execution and preserves the current
  missing-canonical-input refusal;
- all four `pipeline_handoff_refusals` tests pass with transient outputs absent;
- the complete P3/P3B proof wall remains green; and
- the final P3B implementation reviewer covers this delta.
