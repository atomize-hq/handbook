# HCM-2.3 59-surface successor implementation proof wall

Created at: 2026-07-25T15:21:35Z

Status: **PASS — complete successor proof wall**

This proof wall succeeds, and does not modify or replace, the immutable Review
10 evidence, the Review 10 contract-correction successor, the earlier
implementation proof wall, or any predecessor selector or review.

## Frozen authority

- Review 10 remains byte-exact at
  `sha256:8a593688c1573cc62b4ee21b8fa279edf03b74b2b65a08f2f9eded060f355921`.
- The Review 10 contract-correction successor remains byte-exact at
  `sha256:9e3e5966ca0ec3ced877d12b2fcec71bdf5c3a11c0c55f9d81ca7dd7ad5cb975`.
- The immutable 59-surface successor selector is
  `20260725T144741Z--HCM-2-3--atomic-publication-implementation-59-surface-successor-selector`
  at
  `sha256:61b04e6452797ba88ea87106376c9e55ea3629bfef98f8cb7681e05d41b378bd`.
- Its fresh closure review is CLEAN with zero findings at
  `sha256:ff126d5d4b9bfb4c30a9e841b581be6a095e9424c11ea8c86ea5efa52db4ccda`.
- Its fresh API-compatibility review is CLEAN with zero findings at
  `sha256:14c7902642a0851b2dcf6cf19cd694b567a19718d687ae75fa2b7549122cd26c`.
- The selector preserves the prior 58-surface selector and reviews as
  immutable evidence. Only the prior closure conclusion is superseded by the
  59-surface selector and its fresh CLEAN reviews.

## Repository identity and invariants

- Branch: `codex/hcm-2-3-planning`.
- HEAD:
  `3c49fa2c6d653f4b1a703d1d5c5196147b003533`.
- Staged paths: zero.
- `handbook-engine`: `0.1.1`.
- `handbook-engine` 0.2.0 consolidation remains deferred.
- Temporary strict-read diagnostic tags: absent.
- `read_bounded_regular` and
  `open_repo_relative_regular_file_strict` remained outside the selector and
  were not edited after their restored pre-implementation source checkpoint.
  The checkpointed complete
  `crates/engine/src/artifact_lineage_store.rs` SHA-256 was
  `98cbf055ce888ef65da5f44a0ac540b28457ea13eee8d36e692551d6c0017628`.
- No public API, dependency, Cargo manifest, lockfile, fallback primitive,
  strict-reader rule, broader completion rule, or additional unsafe item was
  added.

## GitNexus gates

Before the fifty-ninth surface was edited, upstream impact analysis for
`every_installed_subset_recovers_the_exact_missing_complement` returned exact
LOW risk with zero direct callers, zero affected symbols, zero processes, and
zero modules. The eleven previously authorized test functions were replayed
before edit and each returned the same exact LOW / zero-caller / zero-process /
zero-module result.

Final change detection:

```text
npx gitnexus detect-changes --scope all --repo C:\hcm22ar-doc-repair --limit 500
```

Result: PASS with the expected CRITICAL aggregate warning — 27 indexed files,
112 changed indexed symbols, and 32 affected execution flows. All 112 symbols
and all 32 flows are the previously classified HCM-2.3 implementation subject;
none is an unexpected scope escape. The exact 59-code-surface selector plus its
existing crate-policy surface closes the authorized implementation delta. The
aggregate graph includes pre-existing HCM-2.3 registration, selection,
repository, CLI, strict-reader, and schema-resolution changes already present
in the accepted complete subject; it does not imply a new edit to
`read_bounded_regular`, strict-reader admission, sharing security, or a public
API.

## Implemented successor delta

The fifty-ninth surface changes only
`every_installed_subset_recovers_the_exact_missing_complement`:

- intake and candidate subsets retain their existing recovery expectations;
- promotion subsets 0 and 2 retain their successful recovery expectations;
- promotion subsets 1 and 3 now require mutation-free
  `ConflictingTransactionState`, because the moved canonical candidate lacks
  durable `native-publication-result.json`;
- `install_subset` is unchanged for this case and no continuity evidence is
  synthesized.

The eleven prior-selector tests now carry the exact promotion basis authority
and use the transaction-specific candidate reference persisted in
`native-publication.json`. The two fresh-path hook cases discover that
deterministic reference from an isolated faulted probe transaction before
exercising the actual fresh path. The only retained
`.registry-brief.yaml.generic-replace` fixture is the deliberately
pre-establishment residue tested by
`fresh_promotion_refuses_preexisting_replacement_scratch_without_mutation`.

Current implementation hashes:

| Path | SHA-256 |
|---|---|
| `crates/engine/src/artifact_lineage_store.rs` | `12e4dff7be28a4e3ba51f295509d070aaf27bb0f07aa3117aeb234941fe09b22` |
| `crates/engine/src/artifact_operations.rs` | `09f84564bc1646b443614e3815fcf46ad2611dfe5a28422b552ba03e64659470` |
| `crates/engine/src/lib.rs` | `c14a73d723f5c85a67067f160c92a5e92e05db7804b80048be9274c79c2a9fb2` |
| `crates/engine/tests/hcm_2_3_generic_lineage.rs` | `afb7f4937041e7b301cfef3de135cd3d4b85ab71f0fdf68a66471751d9fb5350` |

## Validation

- Focused successor test:
  `cargo test -p handbook-engine --test hcm_2_3_generic_lineage every_installed_subset_recovers_the_exact_missing_complement -- --exact`
  — PASS, 1/1.
- Complete lineage target:
  `cargo test -p handbook-engine --test hcm_2_3_generic_lineage` — PASS,
  54/54 in 276.47 seconds.
- Complete workspace:
  `cargo test --workspace --quiet` — PASS, exit 0 in 761.9 seconds,
  including engine 152/152, CLI 98/98, actual-binary HCM-2.3 8/8, and
  lineage 54/54.
- Strict lint:
  `cargo clippy -p handbook-engine -p handbook-cli --all-targets -- -D warnings`
  — PASS.
- Engine all-target check:
  `cargo check -p handbook-engine --all-targets` — PASS.
- Formatting:
  `cargo fmt --all -- --check` — PASS.
- Whitespace/error-marker hygiene: `git diff --check` — PASS.
- Dependency feature tree:
  `cargo tree -p handbook-engine -e features` — PASS with no manifest,
  lockfile, dependency, or feature addition.
- Corrected frozen executable contract checker — PASS: 81 rows, 162 platform
  cells, four chains, 20 semantic/schema mutations, 39 `NtCreateFile`
  mutations, and 36 `NtFsControlFile` mutations.
- Duplicate-safe contract and document validation — PASS: six JSON artifacts,
  three Draft 2020-12 schemas, 33 Markdown files, 54 local links, and balanced
  fences.
- Normal handoff validation — PASS: three record schemas, two
  internal-dispatch schemas, two templates, 52 records, 318 current internal
  dispatches, eight admitted legacy dispatches, and 52 ledger entries.
- Historical v1.0 admission self-test — PASS.
- Orchestration-contract self-test — PASS.
- Preservation hashes remain exact:
  `profile_builtins.rs`
  `e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1`,
  HCM-1.1 schema entry
  `ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536`,
  and HCM-1.1 root schema
  `f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510`.

## Disposition

The implementation and proof wall are complete and green. Strict mutation and
share denial are preserved; promotion recovery refuses rather than inventing
missing durable native-publication continuity. No staging, commit, merge,
push, ledger write, version change, or handbook-engine 0.2.0 consolidation was
performed. A fresh complete-subject review remains the final merge-readiness
gate after this proof wall is frozen.
