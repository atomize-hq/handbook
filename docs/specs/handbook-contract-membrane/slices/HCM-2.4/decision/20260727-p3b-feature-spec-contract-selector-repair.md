# HCM-2.4 P3B feature-spec contract selector repair

Status: **selector review CLEAN; exact test-only implementation authorized**

Date: 2026-07-27

## Trigger

After the negative-fixture repair, the complete P3 packet wall passed 174 tests
before this single-test suite failed:

```text
cargo test -p handbook-cli --test feature_spec_contract

foundation_flow_demo_feature_specs_match_directive_and_template_contract
FAILED: rich M4 expected view != minimal "Ship safely" renderer fixture
```

P3B intentionally changed the two foundation-flow Work Specification model
outputs from the minimal definition-support sample to the same rich M4 journey
record. Their generated Markdown views changed accordingly. The test still
compares both rich views to
`crates/engine/tests/fixtures/hcm_2_4_work_specification/artifacts/feature_spec/FEATURE_SPEC.md`,
which is the unchanged minimal renderer fixture for the distinct
`example.record.work` / `Ship safely` canonical input.

The already-green `cli_surface` happy and skip paths prove byte-exact rendering
of each rich canonical input through the real Stage 10 capture path. This
failure is a stale cross-fixture equality assertion, not permission to alter
the released renderer definition, minimal renderer fixture, or production
renderer.

## Exact selector amendment

Editable:

- `crates/cli/tests/feature_spec_contract.rs`, only
  `foundation_flow_demo_feature_specs_match_directive_and_template_contract`.

Keep all directive, template-field, duplicate-safe canonical-YAML,
canonical-byte, exact-eight-field, schema, version, non-empty scope, and
non-empty acceptance assertions.

Replace only the cross-fixture comparison against the minimal renderer golden
with local first-case baselines that require:

1. happy and skip canonical Work Specification model-output bytes are
   byte-identical; and
2. happy and skip generated Markdown-view bytes are byte-identical.

The test must continue to read both journey cases. It may not rewrite or stop
validating either expected view. Exact rich renderer bytes remain proved by the
already-selected `cli_surface` real capture paths. The unchanged Stage 10
capture tests continue to bind the minimal canonical input to its minimal
renderer fixture, while the definition-support suite freezes the released
renderer vector.

The P3B SPEC row, packet manifest, plan, todo, and proof may record this
selector, review, implementation, and verification result.

## Reviewability and non-goals

This amendment changes one stale assertion family inside one test and
introduces no function, class, method, or production symbol. GitNexus cannot
resolve the test symbol and reports risk `UNKNOWN`; live source shows no
caller or production path.

No production/library symbol, renderer, renderer vector, engine fixture,
foundation-flow model output or expected view, support helper, schema, public
API, dependency, Cargo/version, unsafe-policy, P2, P4, P5, P6, or sibling-slice
change is authorized.

## Entry and exit

Entry requires fresh built-in review accepting this exact selector.

Exit requires:

- the one-test suite passes without changing either compared fixture family;
- both journey model outputs and both expected views remain pairwise identical;
- the complete 98-test CLI surface and P3/P3B walls pass; and
- the final P3B implementation reviewer covers this delta.
