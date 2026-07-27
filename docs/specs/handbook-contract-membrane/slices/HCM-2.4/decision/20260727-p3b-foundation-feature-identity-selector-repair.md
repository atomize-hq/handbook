# HCM-2.4 P3B foundation feature-identity selector repair

Status: **proposed exact two-fixture amendment; implementation paused for fresh
review**

Date: 2026-07-27

## Trigger

After the repository-identity prerequisite review was CLEAN and implemented,
standalone Stage 10 preview/apply and raw-payload refusal passed. The three
foundation-flow journey tests then reached the reviewed P3 runtime and exposed
two remaining stale proof expectations:

- the two M4 journey assertions still named the compile header “Feature
  Specification” rather than “Work Specification”; and
- the M5 handoff correctly derived feature ID `example-record-work` from the
  foundation-flow Work Specification `record_id: "example.record.work"`, while
  the committed M4 proof contract, expected slice plan, scorecard, bundle root,
  and test constant all retain stable feature ID
  `fs-m4-foundation-journey-2026-04`.

The compile-header assertions are already inside the first reviewed P3B
selector. The feature-identity mismatch is an input-fixture omission.

## Exact additive selector

Add exactly these two existing test-fixture inputs to P3B:

- `tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_10_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_10_feature_spec.md`

In each file, change only:

```text
record_id: "example.record.work"
```

to:

```text
record_id: "fs.m4.foundation.journey-2026-04"
```

All other Work Specification fields and bytes remain unchanged. The normalized
handoff feature ID therefore remains
`fs-m4-foundation-journey-2026-04`, preserving the existing test constant,
expected slice plan, scorecard, bundle root, consumer path, and proof identity.

## Review finding remediation

Discovery finding
`p3b-foundation-feature-identity-selector-repair-discovery-1-P2-1` correctly
identified that the originally proposed dotted numeric segments violated the
selected Work Specification `record_id` pattern. The corrected final segment
`journey-2026-04` begins with a lowercase letter, remains inside the same
one-field/two-file selector, and slugifies to the unchanged required feature
ID.

## Boundary and exit proof

This is test/evidence input only. It does not edit the shared pipeline proof
corpus, expected generated Markdown views, M5 scorecard, expected slice plan,
production/runtime, renderer, handoff logic, profile, definition, schema,
public API, dependency, Cargo/version, unsafe policy, or sibling slice.

Entry requires fresh built-in review accepting these exact two one-line changes.
Exit remains the eight focused tests, all 98 `cli_surface` tests, P3 packet
wall, workspace wall, formatting, clippy, archive boundary, diff checks, and
different-fresh implementation review.
