# HCM-3.2 Fresh Kernel Pre-Review Proof

Parent: `20260802T232751Z--HCM-3-2--context-resolution-kernel-fresh`

## Planning authority

The fresh selector subject ended terminal CLEAN at
`sha256:dea09f62e2572ebf4aeaf53d787e9d462d602129d85cc29e3aa5081c7fe4bd2a`
after discovery, one consolidated remediation/closure, and two immediately
causal supplementals. No test or Rust edit preceded CLEAN.

## Live impact gate

Immediately before Rust edits, exact-UID upstream impact reported:

- `ContextResolutionStackDefinition`: HIGH, 4 impacted, 4 processes;
- `AuthoredStack::resolve`: CRITICAL, 6 impacted, 5 processes;
- `ContextResolutionStackDefinition::load_bytes`: CRITICAL, 30 impacted,
  8 processes (authorized ceiling, byte unchanged);
- `GenericArtifactLineageStoreV1::validate_inventory`: CRITICAL, 30 impacted,
  6 processes;
- `GenericArtifactLineageStoreV1::require_journal_authority`: LOW, 9 impacted,
  1 process, and the reviewed conditional routing need was confirmed.

FTS remained unavailable/degraded and is not reported GREEN.

## RED/GREEN

RED was captured before production edits:

- configurable non-shipped stack load refused as exact-shipped-only;
- the selected kernel imports were unresolved;
- the dedicated real-authority fixture initially lacked repository profile
  selection.

GREEN commands/results:

- `cargo test -p handbook-engine --test context_resolution_stack`: 4/4;
- `cargo test -p handbook-engine --test context_resolution_kernel`: 17/17;
- real authority positive: all eleven uses through repository guard, generic
  recovery/currentness, committed registry, and native assertion verification;
- real authority negatives: stale raw bytes, malformed and uncovered binding,
  authenticator refusal, invalid signature, non-increasing counter, exact cache
  replay, changed-subject replay, and captured replay across fresh resolvers;
- `cargo test -p handbook-engine --all-targets --all-features`: GREEN in
  702.7 seconds after an earlier 604-second incomplete timeout;
- `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings`:
  GREEN;
- `cargo fmt --all -- --check` and `git diff --check`: GREEN.

## Exact ceilings

- production paths: 4/4;
- test families: 3/3;
- named production declarations: conservative 79/80;
- inserted production lines: 1,570/2,000;
- `artifact_lineage_store.rs`: 17 non-test inserted lines, below 60;
- dependencies, Cargo, unsafe, transport, native/platform, shipped definition,
  profile, vocabulary, pipeline, and HCM-3.3+ deltas: none.

The dedicated test fixture seeds the closed canonical binding deterministically
because the existing generic intake output ceiling (15 distinct values) cannot
encode its 53-leaf closed schema. The proof still reads through the real guard,
generic lineage inventory/recovery/currentness, committed registry, schema
operation service, and authenticator. Existing generic mutation regression is
part of the complete engine wall; no production bypass or ceiling change was
added.

## Pending gates

Independent implementation discovery review, any causal remediation/closure,
workspace proof wall, GitNexus changed-symbol compare evidence, proof/final
reviews, two-commit closeout, and local CAS publication remain pending.
