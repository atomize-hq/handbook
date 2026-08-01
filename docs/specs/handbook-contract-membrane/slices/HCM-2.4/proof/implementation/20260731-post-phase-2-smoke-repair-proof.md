# HCM-2.4 post-Phase-2 smoke-repair proof

Date: 2026-07-31

## Boundary

This is a separately selected post-exit product correction under parent
`20260731T182849Z--HCM-2-4--post-phase-2-smoke-repair`, outcome
`hcm-2.4-post-phase-2-smoke-repair`, packet
`20260731-post-phase-2-smoke-repair-selector`, and causal budget
`sha256:27a404e0d9edf04c02a1aa0ac67180c2c341c6e05701267c29d2c19635243e47`.
It neither rewrites nor invalidates the immutable P7/Phase 2 exit. HCM-3.x,
published JSON protocol, SDK/transport, native-adapter widening, release,
publication, and push remain unauthorized.

## Corrected behavior

- Setup-created strict repository identity establishes only an operational
  root. Missing required Project Authority and Project Context report their
  existing author/fill actions; inspect and generate never request setup again.
- A truly absent root remains `SystemRootMissing` with setup as the next safe
  action. Unsafe roots/identity and legacy Markdown-only trees remain
  fail-closed.
- Setup creates no canonical artifact, profile selection, legacy truth, or
  empty canonical content namespace. Explicit non-Git setup remains
  `ACTION_REQUIRED` with exit code 1.
- Generic listing still requires explicit profile selection plus safe
  repository identity. Pre-establishment `--json` failure remains plain
  stderr with empty stdout because HCM-2.3 defines no CLI-local JSON refusal
  envelope.

## Fixture proof

The CLI regression places the same invalid bytes at
`.handbook/project/charter.yaml` and retains every
`RequiredArtifactInvalid` assertion. The shared compiler authoring fixture
retains all legacy starter-template writes and establishes operational identity
only through
`RepositoryInvocationIdentityServiceV1::initialize_for_setup`. No identity
bytes are handcrafted; authoring assertions, canonical outputs, and the
independent missing-root/invalid-root tests are unchanged.

## Verification wall

| Gate | Result |
|---|---|
| Seven previously failing exact `handbook-compiler --test author` tests | PASS; 7/7 |
| Complete `cargo test -p handbook-compiler --test author` | PASS; 29/29 |
| `cargo test --workspace --all-features` | PASS; exit 0 in 910.1 seconds; independent reviewer replay 874.1 seconds |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo fmt --all -- --check` and `git diff --check` | PASS |
| Fresh `cargo install --locked --force --path crates/cli` | PASS |
| Installed setup/doctor/packet, absent/legacy root, non-Git, JSON boundary, generic list, Windows Project Context, authenticator, and pipeline journeys | PASS |
| Legacy Charter/Project Context production-root scan | PASS; zero matches |
| Retired 12-term bridge/fixed-selector Rust scan | PASS; zero matches |
| Compiler fixture handcrafted identity literal scan | PASS; zero matches |
| Protected path audit | PASS; 9/9 initial SHA-256 values retained and no protected path staged |

GitNexus was refreshed in index-only mode at the live branch HEAD. The named
compiler test helper and CLI test are not indexed, so upstream impact reports
zero indexed symbols/processes and risk UNKNOWN. Exact staged change detection
maps only generic selector Markdown headings and reports a CRITICAL
many-process name collision; the proof reviewer reproduced 286 rather than the
parent's 299 count. Compare-to-`main` terminates under unavailable FTS. Those
diagnostics are recorded honestly and are not called GREEN; exact Git scope,
the full workspace wall, installed smoke, and absence scans provide the usable
subject evidence.

## Review

Proof-stage dispatch
`handoffs/dispatches/20260731T232558Z--HCM-2-4--post-phase-2-smoke-repair-proof-gap-review.json`
returned CLEAN from fresh reviewer
`/root/hcm_2_4_smoke_proof_gap_review` with no P3/P4 advisories. Final
acceptance is conditional on CLEAN from the different-fresh complete-subject
dispatch
`handoffs/dispatches/20260731T235300Z--HCM-2-4--post-phase-2-smoke-repair-final-complete-subject-review.json`.
