# HCM-3.3 post-remediation proof wall

## Consolidated remediation result

Fresh isolated remediator `/root/hcm33_i1_remediation` ran with
`gpt-5.6-sol`, `high` reasoning effort, and `fork_turns=none`. It changed only
five admitted implementation paths: `projection.rs`, `projection/tests.rs`,
and the definition, resolved-profile, and source fixtures. The private
Resolution bridge, module registration, vocabulary fixture, parent proof and
control paths remained outside the remediation edit.

The test-first RED run executed 13 tests: the original nine passed and exactly
four new tests failed, one for each discovery P2. The consolidated repair then:

- restricts exact currentness to snapshot selectors and requires an
  independently identity-bound live family/selector/adapter/slot observation
  before any protected payload read;
- makes validated target ranks the effective rule-evaluation boundary and
  binds the collapse target into result provenance and fingerprints;
- binds the byte-fingerprinted authored profile/configuration pair into
  request/result validation and identity separately from the underlying
  Resolution-profile pair;
- resolves selected capability, source/target-schema, derivation,
  classification, matcher, schema-registry, pointer, compatibility, policy,
  and evaluator dependencies against bounded trusted private semantic
  definitions, rejecting repaired fingerprint substitution and semantic drift
  before payload access.

The final focused suite passed 13/13, including stale/missing/extra/duplicate/
substituted currentness tuples, equal-but-unbound live revision refusal,
no-read refusal, broad-authority-to-narrow-target deterministic collapse,
authored-profile-only drift, dependency-family substitution, policy-rule
drift, evaluator supported-kind drift, and source-schema pointer drift.

## Final validation

- `cargo test -p handbook-engine --lib projection::tests -- --nocapture`:
  passed, 13/13.
- complete `handbook-engine` library binary: passed, 168/168 in 153.76s.
- `cargo fmt --all -- --check`: passed.
- `cargo check -p handbook-engine --all-targets --all-features`: passed.
- `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings`:
  passed.
- staged and unstaged `git diff --check`: passed.
- bounded complete-crate command: `TIMEOUT` at 308.1s with no failure output;
  it is not represented as GREEN and no owned process remains.
- independent parent `--lib --test-threads=1` replay: `TIMEOUT` at 304s with
  no failure output; the exact owned Cargo/test process tree was terminated.
  The remediator's completed 168/168 library transcript remains the available
  full-library pass and the parent timeout is not represented as GREEN.
- exact path audit: passed; eight selected implementation paths total and only
  five changed by remediation.

Final byte hashes for the remediated paths are:

- `projection.rs`: `ebf6bf4284ba36cc09a23eca07165a14f6647ff5b15a0b694c1ef3fd44e802b9`;
- `projection/tests.rs`: `8a1727613997dafc70de73920de50d94f733a0c00f76433782ef57b30a159767`;
- `definition.json`: `e8106932e7016697196f7b89af86344f025c51594dded5b42f1393d86bd6158f`;
- `resolved-profile.json`: `a1c39546db52eeb357981f5b287c6746fe57c0d137b06a979a76f0f4e4f1aac0`;
- `source.json`: `5349066cd3e3040ec3bd0a4f02a224e4dbb52f67fe04f5145a4c5a12c226b113`.

## Final GitNexus boundary

The refreshed exact staged comparison is `MEDIUM`: eight paths, 102 complete
indexed symbol identities, and two private currentness-test flows. Its full
symbol/process population and raw structured-output fingerprint are in
`20260805T221615Z--gitnexus-post-remediation-change-detection.json`.

The required final compare-to-main invocation exited `1` after the refresh and
emitted no comparison payload beyond the FTS-unavailable diagnostic. It is
recorded as unavailable, never GREEN. The earlier discovery artifact preserves
the last available branch-wide compare-to-main observation as `CRITICAL`
(1,395 files, 10,029 symbols, 259 flows); that result described accepted-base
divergence from local `main`, not the selected code delta. FTS remains
unavailable.

## Earned boundary pending closure

This proof supports different-fresh delta-focused closure of
`HCM33-I1-IMPL-DISC-001` through `-004`. The exact private/internal portions of
`PG-PROJ-01` and `PG-PROJ-02` remain unearned until that closure returns CLEAN.
No later slice, public API/default/adopter, Snapshot Memory, release,
publication, or Phase-3 exit is claimed.
