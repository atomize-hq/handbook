# HCM-3.3 deterministic Projection implementation proof wall

## Bound subject

This proof covers the private `handbook-engine` Projection core selected by
`20260805T194328Z--implementation-selector-and-preflight.md`. The production and
test surface remains exactly the eight admitted paths: the private module
registration, one currentness-guarded `ContextResolutionEnvelope` view, the
private engine and tests, and four fixed JSON fixtures. No public API, Cargo
surface, shipped default, consumer, transport, renderer, Snapshot Memory, or
remote publication is included.

The implementation run was `/root/hcm33_i1_implementation`, using
`gpt-5.6-sol`, `high` reasoning effort, and `fork_turns=none`. Its initial RED
compile failed on 99 unresolved private Projection symbols before the engine
existed. Its final focused replay and the parent replay each passed all nine
private Projection tests.

## Corrected future proof wall

The mandatory two-envelope test uses the existing HCM-3.2 authenticated
repository/resolver fixture to create two distinct admitted
`ContextResolutionEnvelope` values. Each value is converted through the new
opaque, currentness-guarded `projection_authority_view`; no synthetic authority
stands in for this proof. The test fixes source, profile, vocabulary,
definition, operation, surface, purpose, and currentness inputs, then:

- executes two byte-deterministic replays per envelope;
- proves both distinct-output and equal-output cross-envelope accounting;
- proves envelope-bound result fingerprints remain exact even when output
  bytes are equal;
- proves source bytes and source fingerprint remain unchanged;
- corrupts only the temporary committed authority and observes an actual
  `StaleAuthority` refusal when requesting the view again.

The remaining tests cover exact configured custom-kind selection, deterministic
`reveal` and allowlisted acyclic deterministic `derive`, zero/multiple source
refusal, exact closure/currentness validation, disclosure/support short-circuit
behavior before payload reads, all six Resolution dimensions, typed omissions,
proof effects, lossiness precedence, complete provenance and fingerprints,
`authority_effect: none`, collapse without widening, expansion refusal, and
forbidden executable/synthesis/remote/transport authority.

## Validation

- `cargo fmt --all -- --check`: passed.
- `cargo test -p handbook-engine --lib projection::tests -- --nocapture`:
  passed, 9/9, both in the implementation run and independent parent replay.
- `cargo check -p handbook-engine --all-targets --all-features`: passed.
- `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings`:
  passed.
- `git diff --check`: passed.
- exact selected-path audit: passed, eight of eight implementation paths and
  no unexpected implementation path.
- bounded `cargo test -p handbook-engine`: the complete 165-test library binary
  passed, then the command reached its 304-second bound in the pre-existing
  `context_resolution_kernel` integration executable. This is recorded as
  `TIMEOUT`, not GREEN and not a Projection failure.
- GitNexus exact pre-edit impact for
  `Impl:crates/engine/src/context_resolution.rs:ContextResolutionEnvelope`:
  `LOW`, zero direct dependants, zero affected processes/modules.
- GitNexus exact staged change detection after refreshing the local index:
  `HIGH`, eight exact paths, 480 complete indexed symbol identities, and 14
  affected flows. The HIGH result is reported without understatement and is a
  required independent-review input. The earlier stale-index `LOW` result is
  superseded and is not closure evidence.
- GitNexus compare-to-main: `CRITICAL`, 1,395 files, 10,029 symbols, and 259
  flows because the accepted detached base substantially diverges from local
  `main`; this branch-wide result is not attributed to the eight-path slice.
- GitNexus FTS: unavailable and never represented as GREEN.

The byte-hashed raw comparison evidence is
`20260805T211549Z--gitnexus-change-detection.json`.

## Earned boundary

The proof supports only the selected private/internal portions of
`PG-PROJ-01` and `PG-PROJ-02`. It does not claim a public Projection API,
shipped default profile/catalog, consumer adoption, Snapshot Memory,
HCM-3.4+, or Phase-3 exit.
