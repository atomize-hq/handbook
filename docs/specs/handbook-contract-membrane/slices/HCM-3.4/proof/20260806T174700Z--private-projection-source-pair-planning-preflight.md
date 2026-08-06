# HCM-3.4 private Projection source-pair planning preflight

## Identity and exact local admission

- Meta workflow: `handbook-hcm-3-4-source-pair-planning-20260806`.
- Dispatch nonce: `f77bcdc4a7e644cd84c89b2d4cba9cb99a18c8875b344e75ba123cc5d9fabc9c`.
- Bound increment task/host: `019fd829-752d-7d93-bfd8-ecb0a30d03a3` / `local`.
- Assigned checkout: `C:/Users/spmcc/.codex/worktrees/8f7d/handbook`.
- Expected and observed HEAD: `7f269887c1715263cd2ea370427ca2cc76e4e1a3`.
- Expected and observed tree: `d4d0eea44c2b3ba985e87674844e5b9713376db1`.
- Required ancestor `d7877f7843afbcda78d65e0fa2bf7093d3c71e6a` is an ancestor.
- Dedicated local planning ref
  `refs/heads/orchestration/handbook-hcm-3-4-source-pair-planning-20260806`
  observed the expected base. The assigned worktree is detached and clean.
- Observed `origin/main`: `a3babd20329027afacdcee9d8b7b9d638d15af5b`.

The provided protected-checkout baseline manifests remain external control
evidence. This task does not issue a Git command in, write to, stage from, or
commit any protected checkout; every reviewed-primary path is inside the
assigned worktree.

## Source-pair discovery facts

The immutable `20260806T064500Z` HCM-3.4 record is a valid blocked source
handoff: its private Snapshot Memory work is not reopened, and it reports that
PG-SNAP-04 lacks a generic Projection representation for the required pair and
five-family currentness closure. The relevant live private code facts are:

- `ProjectionRequest.sources` and `bind_sources` already enforce exact
  configured-selector identity and reject unbound/duplicate sources;
- `SourceDocument.captured_revisions` has one currentness family tuple;
- `validate_request_currentness` requires exact expected/captured/live tuple
  equality but cannot read five tuples from the one current source; and
- source semantic validation has no check that the selected delta's
  `to_snapshot` dependency equals the selected current snapshot.

The canonical Snapshot-grounding Projection contract already declares exactly
one `snapshot_current`, exactly one `snapshot_delta`, the `to_snapshot`
relationship, five currentness families, typed omission, pre-access redaction,
and non-authority. This planning packet proposes no contract revision.

## Planning-only proof wall

Before discovery review, the parent will replay the sorted path/hash manifest
for this selector, specification amendment, plan, ledger, and preflight;
inspect the live Projection and Snapshot Memory seams as context only; check
UTF-8, Markdown fences, trailing whitespace, and `git diff --check`; and
validate the v1.4 dispatch with `validate_handoffs.py --verify-dispatch`.

No Rust, test, fixture, runtime configuration, product proof, consumer, or
remote operation has been run or claimed. GitNexus MCP tools and the local
runner are unavailable, so scoped and compare-to-main change detection must be
recorded as unavailable rather than GREEN.
