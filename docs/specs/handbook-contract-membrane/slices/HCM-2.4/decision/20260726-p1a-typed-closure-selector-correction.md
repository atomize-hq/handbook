# P1A typed-closure and intake-admission selector correction

Status: accepted only when the different-fresh selector closure review is clean;
implementation remains paused until then

## Context

P1A discovery review over subject
`sha256:4edbc4fe580a3c819fcdac64bd83623d09ebc2337f0d3c170d8f7d0ecd927865`
found two Required gaps:

1. the new intake, renderer, kind, and profile fingerprints covered authored
   content or partial schema closure rather than the uniform exact typed
   dependency closure required by `05`; and
2. `ArtifactRepositoryV1::open` routed every package intake through the legacy
   Charter compatibility parser, which trusted its fingerprint and discarded
   modes and coverage.

The P1A semantic contract already requires complete typed closure and actual
first-party intake resolution. Its original selector made
`artifact_intake_registry.rs` and `artifact_repository.rs` read-only and omitted
the profile fingerprint producer. The contract and selector therefore could not
both be satisfied.

## Decision

Correct P1A without widening its behavior or family scope:

- add the exact P1A typed-closure branch in
  `ArtifactIntakeDefinitionV1::parse`;
- add the exact successor typed-closure branch in
  `AuthoredArtifactKindDefinition::validate`;
- add the exact shipped-root `1.2` branch in
  `validate_authored_profile_fingerprints`;
- change only `load_repository_intakes` compatibility selection in
  `artifact_repository.rs`, retaining the released Charter intake on the
  identity-only path and sending exactly the five P1A package intakes through
  full semantic admission;
- retain the already authorized built-in mapping and descriptor guard changes;
- add one exact `hcm_2_4_definition_runtime.rs` target for actual repository
  admission and fail-closed dependency/mode/coverage cases; and
- replay read/proof-only `hcm_2_3_registration_kernel.rs` unchanged at
  `sha256:86e6e98fe8fad63578be3d157d3a836c4bfe3a4dc7aa49f1ff41d87e33b61629`
  to prove generic repository-defined intake behavior did not move.

No public signature, schema, Cargo/package/dependency, command, module,
Resolution, Projection, migration, dual-read, shipped-default, or sibling-slice
change is authorized.
`ArtifactIntakeRegistry::load_with_builtin_compatibility` and
`resolve_profile_selection` are explicitly excluded.

## Atomicity and reviewability

The correction is atomic because a typed intake fingerprint, its runtime
admission producer, the successor kind/renderer closure, and profile `1.2`
literal must converge on one dependency graph. Landing only one component would
either preserve caller-trusted fingerprints, make the profile unresolvable, or
temporarily accept definitions whose runtime semantics are discarded.

The implementation boundary is still exact and reviewable: three newly
admitted private runtime functions/branches plus the already selected P1A
guards, one new focused runtime test, and one unchanged regression target.
Fresh GitNexus impact is required for every newly admitted symbol before edit.

The selector-closure baseline reports:

- `ArtifactIntakeDefinitionV1::parse`: LOW, 11 impacted, 3 direct, 1 process,
  2 modules;
- `AuthoredArtifactKindDefinition::validate`: LOW, 0 graph edges, with the
  exact-source/type-edge caveat;
- `validate_authored_profile_fingerprints`: CRITICAL, 130 impacted, 1 direct,
  8 processes, 10 modules; and
- `load_repository_intakes`: CRITICAL, 44 impacted, 1 direct, 7 processes,
  11 modules.

The CRITICAL results are accepted only for the exact P1A/1.2 branch changes.
Any broader changed symbol, public caller, process/module contract, or generic
behavior is a stop.

## Consequences

- P1A implementation remains blocked until this correction is independently
  reviewed.
- The two discovery P2s remain open until implementation remediation and a
  different-fresh closure review.
- P1B remains blocked until the corrected literal profile `1.2` fingerprint and
  actual intake-admission proof are review-clean.
