# HCM-2.4 P1A Const-String Coverage Selector Approval

Status: accepted after different-fresh selector closure review; implementation
proof and complete-subject review remain parent-owned.

Date: 2026-07-26

Supersedes: no prior decision. This record extends, and does not rewrite,
`20260726-p1a-typed-closure-selector-correction.md`.

## Context

The P1A typed-closure correction is green, but the real
`ArtifactRepositoryV1::open` proof rejects each of the five new package
intakes. Every released candidate schema has type-absent `schema_id` and
`schema_version` leaves whose `const` value is a JSON string.
`ResolvedSchema::collect_coverage_leaf_shapes` currently refuses those leaves
as indeterminate before the intake coverage comparison.

The released schema files are immutable. The operator explicitly approved one
additional production symbol for this branch-local case and prohibited a sixth
production symbol or broader schema inference.

Fresh GitNexus upstream impact at the resumed baseline resolves
`ResolvedSchema.collect_coverage_leaf_shapes#5` with LOW risk: one direct
caller (`coverage_leaf_shapes`), no indexed process, and one module.

## Decision

P1A may edit only `ResolvedSchema::collect_coverage_leaf_shapes` in
`crates/engine/src/schema_registry.rs` beyond the four production symbols
already frozen by the typed-closure selector correction.

After reference resolution and the existing ambiguity check, the method may
classify a coverage leaf as `ResolvedBindingJsonType::String` only when both
conditions hold:

1. the schema node has no `type` member; and
2. the schema node has a `const` member whose JSON value is a string.

An explicit `"type": "string"` retains its existing behavior. The new branch
does not change structural validation or the released schema bytes.

Test-only assertions may be added inside the existing
`schema_registry.rs` test module. The already-selected
`crates/engine/tests/hcm_2_4_definition_runtime.rs` remains the real-path
acceptance proof.

## Preserved refusal boundary

The method must continue to refuse every other unsupported or indeterminate
coverage leaf, including:

- a type-absent `const` whose value is null, Boolean, number, array, or object;
- type-absent `enum`, `default`, `examples`, or annotation-only nodes,
  including a one-element string enum;
- ambiguous/composite nodes and semantic `$ref` siblings;
- unsupported explicit types and every existing open-object, cycle, pointer,
  and reference refusal.

No object shape, array shape, scalar type, or cardinality may be inferred from
instance data or any keyword other than the exact string-valued `const` case.

## Proof

Before implementation, a focused test must demonstrate that the exact
type-absent string-valued `const` leaf is RED. Negative cases must demonstrate
that non-string `const` values, a type-absent string `enum`, and a string
`const` paired with an unsupported explicit type remain refused.
After implementation:

- those focused tests must be GREEN;
- all six cases in `hcm_2_4_definition_runtime` must be GREEN;
- the existing P1A typed-closure target and immutable HCM-2.3 registration
  target must remain GREEN; and
- formatting, `git diff --check`, scoped diff inspection, fresh independent
  review, and the packet regression wall must pass.

## Stop conditions

Stop P1A if the proof requires:

- any sixth production symbol;
- any schema or released-definition byte change;
- inference beyond the exact string-valued `const` predicate;
- an edit to the compatibility loader, public API, dependency, Cargo metadata,
  unsafe policy, package boundary, or sibling slice; or
- a wider GitNexus impact or changed execution-flow contract.
