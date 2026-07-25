# HCM-2.3 Review 10 contract-correction implementation proof wall

Status: **BLOCKED — explicit selector stop condition reached**

Recorded at: `2026-07-25T04:11:00Z`

This evidence preserves every predecessor and records the bounded implementation
attempt authorized by the Review 10 contract-correction successor. It does not
rewrite, supersede, or weaken a prior proof, selector, review, or hash.

## Immutable authority

- Frozen Review 10 artifact:
  `sha256:8a593688c1573cc62b4ee21b8fa279edf03b74b2b65a08f2f9eded060f355921`.
- Frozen Review 10 historical subject:
  `sha256:8dac11d067a4fba0efea04a3ae1435ae1fa9c8a82307171676380e9aa8f1413a`.
- Frozen Review 10 proof:
  `sha256:d8d183f0f121123326eeefa38e51821c7e14b8d3e5fea66d37a7a422fee3015f`.
- CLEAN 99-path contract-correction successor review:
  `sha256:9e3e5966ca0ec3ced877d12b2fcec71bdf5c3a11c0c55f9d81ca7dd7ad5cb975`,
  subject
  `sha256:a71018c79c777d3b3875c83e437ec5378d872184efef379da3eedd31db8b96e9`.
- Corrected successor proof:
  `sha256:f9142c2e3cf62907c0a096678800cd3987ea690e88762648c3d74cf97ec9d4b6`.
- Sixth superseding implementation selector:
  `sha256:85f3bc83dc95cf722199f0930c2e4cbb0a999c34d087b641f57baf37d44aaa0a`.
- Fresh sixth-selector closure review:
  `sha256:0822a3da7e6966619252bc16b29235f2cdc455572a4d2f076ddfd976535ab224`,
  verdict `CLEAN`, zero findings.
- Fresh sixth-selector API-compatibility review:
  `sha256:5e34a9d2dc59b1e025233e1f8b6768865c792453281eb6a2de0c7f2d52a8da6c`,
  verdict `CLEAN`, zero findings.

Branch remained `codex/hcm-2-3-planning`; HEAD remained
`3c49fa2c6d653f4b1a703d1d5c5196147b003533`; the staged set remained empty.

## Authorized implementation delta

Only the two authorized existing functions were intentionally changed:

1. `query_object_id_absence_nt` now accepts exactly:
   - immediate raw `STATUS_OBJECTID_NOT_FOUND` with the complete initialized
     `IO_STATUS_BLOCK` sentinel unchanged;
   - raw and final `STATUS_OBJECTID_NOT_FOUND` with `Information == 0`; or
   - raw `STATUS_PENDING` followed by final
     `STATUS_OBJECTID_NOT_FOUND` with `Information == 0`.
   Every other combination remains fail-closed.
2. `publish_replacement` now declares and calls local
   `NtSetInformationFile(FileRenameInformation)` with its local
   `IO_STATUS_BLOCK`, checked `FileName` offset plus exact counted UTF-16 leaf
   bytes, retained parent handle, and `ReplaceIfExists == FALSE`. The former
   `SetFileInformationByHandle(FileRenameInfo)` declaration and last-error
   conversion are gone.

All native declarations, layouts, constants, buffer construction, and unsafe
operations for the replacement remain inside `publish_replacement`. No helper,
dependency, Cargo edit, global, fallback primitive, fifth unsafe item, public
API, persisted-label migration, or package-version change was added. The final
implementation file hash at this wall is
`sha256:98cbf055ce888ef65da5f44a0ac540b28457ea13eee8d36e692551d6c0017628`.

## GitNexus impact gate

The index was rebuilt from the current uncommitted worktree before editing.

- `query_object_id_absence_nt`: exact LOW risk; one direct caller and three
  upstream nodes, with or without tests.
- `publish_replacement`: exact CRITICAL risk; three direct callers, eight
  production upstream nodes, ten with tests, six affected execution processes,
  and one affected module.

The CRITICAL publication impact was reported before the edit and was already
inside the authorized selector.

## Passing gates

- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- `cargo check -p handbook-engine --all-targets`: PASS with one warning.
- Corrected executable contract checker: PASS — 81 rows, 162 platform cells,
  four chains, 20 semantic/schema mutations, 39 `NtCreateFile` mutations, and
  36 `NtFsControlFile` mutations.
- Complete handoff validation: PASS — three record schemas, two
  internal-dispatch schemas, two templates, 52 records, 312 current internal
  dispatches, eight admitted legacy dispatches, and 52 ledger entries.
- `handbook-engine` remains `0.1.1`.
- Staged paths: zero.

## Blocking gates

The focused command

```text
cargo test -p handbook-engine --test hcm_2_3_generic_lineage
```

completed 54 tests with 33 passing and 21 failing. The deterministic minimized
native-publication loop

```text
cargo test -p handbook-engine --test hcm_2_3_generic_lineage promotion_persists_the_schema_exact_native_result_chain -- --exact --nocapture
```

fails before `NtSetInformationFile`, while reopening the retained candidate,
with raw `STATUS_SHARING_VIOLATION (0xC0000043)` and the initialized
`IO_STATUS_BLOCK` sentinel unchanged. Other preserved failures include existing
promotion-record exactness, recovery admission, candidate suffix, and
publication continuity paths. Temporary diagnostic output and experimental
changes were removed.

Strict lint also fails:

```text
cargo clippy -p handbook-engine --all-targets -- -D warnings
```

The decisive out-of-scope finding is dead code at
`create_replacement_scratch`. That production symbol is not in the immutable
42-symbol selector. Deleting it, annotating it, or restoring a caller would edit
an additional symbol; suppressing the warning would waive a required gate.
Clippy also reports `clippy::cmp_owned` inside the already-selected
`native_metadata_subjects`, but no repair was attempted after the first
out-of-scope requirement reached the operator's mandatory stop condition.

## Disposition

Implementation is not represented as CLEAN or merge-ready. The two bounded
contract corrections are present and compile, but completing the inherited
proof wall requires authority beyond the sixth selector. No additional symbol
was edited to repair the failures. Nothing was staged, committed, merged, or
pushed, and no package version changed.
