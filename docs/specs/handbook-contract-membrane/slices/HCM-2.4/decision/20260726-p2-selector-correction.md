# HCM-2.4 P2 legacy-authority selector correction

Status: accepted after independent discovery and different-fresh closure review

Date: 2026-07-26

## Context

The HCM-2.4 specification and plan require P2 to make selected Environment
Context YAML the sole editable authority before P6 begins. P2's RED and GREEN
conditions explicitly cover compiler, setup, doctor, manifest, and flow
behavior and require mutation or deletion of legacy Environment Inventory
Markdown to have no outcome effect.

The frozen P2 production selector named the Environment Inventory authoring
files and library templates but omitted the existing fixed canonical,
baseline-validation, template-library, and flow consumers. P6 named most of
those consumers, yet P6 cannot begin until P2 is independently green. The
selector therefore could not authorize the behavior already required by the
same contract.

## Decision

Add only these existing Environment Inventory branches to the P2 selector:

- `crates/engine/src/baseline_validation.rs`;
- `crates/engine/src/canonical_artifacts.rs`;
- `crates/engine/src/canonical_paths.rs`;
- `crates/engine/src/lib.rs`;
- `crates/compiler/src/baseline_validation.rs`;
- `crates/compiler/src/template_library.rs`;
- `crates/compiler/src/lib.rs`;
- `crates/flow/src/resolver.rs`; and
- `crates/cli/src/rendering.rs` only if its Environment Inventory labels must
  change to preserve the existing result shape.

The selector is branch-limited inside each file. It does not authorize a
blanket rewrite. P6 still owns deletion of aggregate bridge types and fixed
selectors after P1A-P1C and P2-P5 are green.

The exact test expansion is limited to existing baseline, canonical-ingest,
freshness, rendering, resolver, setup, doctor, native mutation/refusal, and
both engine and compiler `artifact_manifest_interface.rs` tests named in the
amended P2 selector. The manifest suites directly prove selected Environment
Context identity/fingerprint, conditional absence, and zero influence from
legacy Markdown mutation or deletion.

## Impact evidence

GitNexus was force-rebuilt at
`0b177ef9415e942cde763fcb086cec546a9a24f3`. Exact upstream results for the
newly named anchors are:

| Anchor | Risk | Impact |
| --- | --- | --- |
| `resolve_environment_inventory_selection` | LOW | 4 impacted, 1 direct, 0 processes, 1 module |
| compiler `validate_artifact_markdown` | LOW | 0 impacted |
| `CanonicalArtifacts.load_fixed_siblings_with_contract#2` | LOW | 10 impacted, 2 direct, 1 process, 2 modules |
| engine `baseline_artifact_validations` | LOW | 3 impacted, 3 direct, 0 processes, 1 module |
| flow `validate_artifact_markdown` | LOW | 0 impacted |
| flow `resolve_with_contract` | MEDIUM | 5 impacted, all direct tests, 0 processes, 1 module |
| flow `baseline_artifact_validations` | LOW | 6 impacted, 1 direct, 0 processes, 1 module |
| compiler `baseline_artifact_validations` | LOW | 0 impacted |

No newly admitted anchor is HIGH or CRITICAL. Existing P6 impact ceilings and
the HCM-2.4 public-API, dependency, package, and sibling-scope stops remain
unchanged.

## Consequences

- P2 can prove its existing zero-legacy-influence GREEN condition without
  beginning P6 early.
- P6 can delete the common bridge types and fixed selectors only after every
  replacement vertical is green.
- No semantic contract, proof classification, public API, dependency, package
  boundary, or sibling slice changes.

Review lineage:

- discovery:
  `20260726T162140Z--HCM-2-4--p0-selector-repair-review`;
- closed finding: `p0-selector-repair-discovery-P2-1`; and
- CLEAN closure:
  `20260726T162911Z--HCM-2-4--p0-selector-repair-closure-review`.

## Rejected alternatives

- Deferring the Environment Context flow cutover to P6 would weaken P2's
  existing GREEN condition and permit P6 to start with a knowingly
  non-authoritative vertical.
- Treating the missing paths as implicit would violate the specification's
  exact-selector and no-blanket-rewrite rules.
