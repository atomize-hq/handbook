# HCM-2.4 P3B — CLI surface proof integration

Status: **implemented and closure-review CLEAN**

Baseline: reviewed checkpoint `5cf41d2f64d68cb7f78abb5eccab9acf307089d3`.

Closure review:
`20260727T061322Z--HCM-2-4--p3b-cli-surface-closure-review`;
reviewer `/root/hcm_2_4_p3b_final_implementation_closure`; subject
`sha256:e65e411a8e21399601af790b2d6228ac14427a6b36d8c0ad68fc90b7e27a78a1`;
verdict CLEAN with no new finding or advisory.

## Scope and preserved authority

P3B contains only test, fixture, deterministic evidence, slice-local selector,
dispatch, and proof changes. It changes no production/library symbol, released
schema or definition, public API, dependency, Cargo/version, unsafe policy, or
sibling slice.

The P3 authority switch is preserved:

- `artifacts/work-specification/work-specification.yaml` is the canonical Stage
  10 artifact and handoff input;
- `artifacts/feature_spec/FEATURE_SPEC.md` is a deterministic generated view;
- the two fixture repositories contain authority inputs but no committed
  repository identity or post-capture generated output; and
- every indeterminate admission/refusal outside the reviewed P3B test
  integrations remains unchanged.

## RED and selector sequence

The first post-P3 workspace wall isolated eight `cli_surface` failures. Focused
tests then exposed, in order:

1. stale Stage 10 fixture-authority mirrors;
2. missing setup-owned identity in fresh temporary repositories;
3. generic foundation-flow record identity that would replace the M4 feature
   identity;
4. one private M5 consumer that requested the generated Markdown view as
   authority;
5. paired shared compile payload/explain goldens with eleven direct test
   consumers;
6. one negative test that depended on an untracked generated YAML file solely
   so setup could delete it; and
7. one contract test that compared a rich M4 view with a distinct minimal
   renderer fixture.

Each selector was frozen before implementation and passed the required
built-in review. Three discovery findings were remediated and closed by
different-fresh reviewers:

- `p3b-foundation-feature-identity-selector-repair-discovery-1-P2-1` corrected
  the record ID to schema-valid `fs.m4.foundation.journey-2026-04`;
- `p3b-shared-compile-golden-selector-repair-discovery-1-P2-1` added the
  pipeline compile suite as the omitted read/proof-only consumer; and
- `p3b-negative-fixture-independence-selector-repair-discovery-1-P2-1`
  synchronized the SPEC P3B row with the exact test path and selector count.

The feature-spec-contract selector reviewer returned CLEAN with one P4 wording
advisory. The local wording now attributes minimal input/view binding to Stage
10 capture tests and renderer-vector freezing to the definition-support suite.

The complete-subject discovery reviewer
`/root/hcm_2_4_p3b_final_implementation_review` returned one P2 and one P4:

- `p3b-final-implementation-review-P2-1` found that the rich M4 objective did
  not state a credible alternative and trade-off as required by the frozen
  architect directive. Both canonical inputs, their deterministic views, the
  downstream plan, and the selector record now use the same explicit approach,
  Markdown-authority alternative, and validation/identity/provenance trade-off.
  `feature_spec_contract` now asserts all three semantic clauses.
- `p3b-final-implementation-review-P4-1` found the discovery-finding count typo
  corrected above.

## Implementation result

- Both fixture roots now contain byte-identical P3 Stage 10 stage, directive,
  template, profile-selection, and Work Specification profile inputs.
- Fresh temporary repositories initialize identity through the existing engine
  setup service; no identity is committed.
- Happy and skip model outputs use the same rich, schema-valid M4 Work
  Specification and preserve feature ID
  `fs-m4-foundation-journey-2026-04`.
- Private M5 repo and bundle consumers read duplicate-safe canonical YAML, not
  the generated Markdown view.
- Happy/skip views, plan, scorecard, transcripts, and the paired compile
  goldens were regenerated deterministically.
- The missing-input refusal test now asserts that the committed fixture starts
  without post-capture YAML and preserves every product refusal assertion.
- The contract test preserves directive/template/schema/canonical assertions
  while requiring happy/skip canonical inputs and generated views to be
  byte-identical.

The four known transient fixture-repository outputs were removed before proof
and remained absent after the complete wall.

## Verification

All Rust commands used `CARGO_TARGET_DIR=target/hcm-2-4-p3b-proof`.

Focused negative and packet proof:

```text
cargo test -p handbook-cli --test pipeline_handoff_refusals \
  pipeline_handoff_emit_refuses_when_feature_spec_artifact_is_missing -- --exact --nocapture
1 passed

cargo test -p handbook-cli --test pipeline_handoff_refusals
4 passed

cargo test -p handbook-cli --test feature_spec_contract
1 passed

cargo test -p handbook-pipeline --lib
26 passed
cargo test -p handbook-pipeline --test pipeline_capture
47 passed
cargo test -p handbook-pipeline --test pipeline_handoff
10 passed
cargo test -p handbook-compiler --test pipeline_capture
39 passed
cargo test -p handbook-compiler --test pipeline_handoff
8 passed
cargo test -p handbook-pipeline --test pipeline_compile
20 passed
cargo test -p handbook-compiler --test pipeline_compile
20 passed
cargo test -p handbook-cli --test cli_surface
98 passed
```

The complete P3/P3B packet wall passed 273 tests with no failure. The full
workspace wall also passed:

```text
cargo test --workspace --quiet
post-remediation exit 0; 717 seconds; no failures; two platform-marked tests ignored
```

Additional convergence checks:

```text
cargo fmt --all -- --check
PASS

cargo clippy -p handbook-pipeline -p handbook-compiler -p handbook-cli \
  --all-targets -- -D warnings
PASS

python tools/check_archive_boundary.py
PASS

python tools/check_archive_boundary.py --self-test
PASS

git diff --check
PASS
```

No transient generated fixture output or unrelated `.claude/**`, `AGENTS.md`,
or `CLAUDE.md` path is part of the P3B subject.

## Review closeout

The complete-subject discovery P2 and P4 are closed. The different-fresh
closure reviewer replayed the 48-file post-remediation manifest, verified the
semantic objective/input/view/plan path, independently passed the 273-test
packet wall, and found no remediation-caused regression.
