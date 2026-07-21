# HCM-2.2 Exact-Result Authority-Repair Implementation Proof Wall

> **Status (2026-07-21): proof-complete exact implementation subject awaiting
> fresh isolated review.** This record does not claim `CLEAN`, a primary commit,
> parent closeout, or HCM-2.3 authority.

## Authority and preservation

- phase/slice: `HCM-2` / `HCM-2.2`;
- branch: `codex/hcm-2-2-exact-result-authority-repair`;
- reviewed clean implementation baseline:
  `f676ea59bbbbcdd0ecc3475bbf2dabd80f5c9d4d`;
- selected handoff:
  `20260721T143023Z--HCM-2-2--orchestration--exact-result-authority-repair-approved`;
- required ancestor commits:
  `f40ac326d31f0bd33a707243681ace4ce9f463fb` and
  `f676ea59bbbbcdd0ecc3475bbf2dabd80f5c9d4d`; and
- later-slice authority: none.

Entry reproduced the exact branch, HEAD, clean worktree, ancestors, selected
handoff, decision, SPEC, plan, checklist, final Review 3 dispatch, archive
manifest, and extracted-byte evidence. The rejected implementation worktree
and preservation root remained read-only. The preservation manifest remained
`cbcb29fe7ef50184cbc2efcaa9489d07c99d0d4b51c7e92836e7e2cc3d1cf44a`;
the 30,789,857,280-byte archive remained SHA-256
`46dbed286dba9b3b6cedf7ef13340c43dcd56e068a5efa9af1deba0cabadd33d`,
with all 26,712 source entries previously extracted byte-equal. Only bounded
code/test/tool hunks were selectively reapplied; no rejected authority,
handoff, ledger, plan, checklist, status, or proof document was copied.

## Reproduced impact boundary

GitNexus was current before implementation. Upstream impact was reproduced
before every existing symbol edit. Across the 213 analyzed existing symbols,
the reproduced distribution was 18 CRITICAL, 20 HIGH, 5 MEDIUM, and 170 LOW.
No new or expanded elevated process/module boundary appeared.

| Required elevated symbol | Reproduced impact |
|---|---|
| `evaluate_charter_intake` | HIGH; 11 total impacts, 7 direct callers, 2 processes, 3 modules |
| generic lineage `validate_record` | CRITICAL; 73 total impacts, 10 direct callers, 20 processes, 16 modules |
| `CharterLifecycleStoreV1::current_promotion_anchor` | HIGH; 11 total impacts, 1 direct caller, 3 processes, 1 module |
| `recover_one_pending` | HIGH; 10 total impacts, 1 direct caller, 3 processes, 3 modules |

The affected elevated paths remained confined to reviewed Charter intake,
lineage, author/approval, promotion transaction/recovery, lifecycle, retained
observation, compiler, and CLI flows. New private helpers and tests had no
indexed upstream blast radius.

## Test-first exact-result repair

The Review 3 coherent-rewrite attack was preserved as RED before production
repair:

```text
cargo test -p handbook-engine --test hcm_2_2_authority_repair \
  coherent_result_witness_and_binding_rewrite_refuses_without_mutation \
  -- --exact --nocapture
RED: candidate 1.2 mutable-companion design accepted the coherent rewrite
```

The repaired subject introduces candidate `1.3` with exactly one closed
`validation_result_binding`: semantic result ref/fingerprint, SHA-256 over the
exact persisted result `1.0` JCS+LF document, and LF-inclusive byte length.
Candidate-subject identity remains result-independent; final candidate identity
includes the binding; approval binds that final candidate; promotion and frozen
intent `1.2` bind the approved candidate. Result `1.0` remains byte-unchanged
and binds only the candidate subject.

Authoring now holds the author lock while recomputing subject and semantic
result authority, performing two complete bounded no-follow inventories of the
candidate and result stores, and classifying zero, exactly one, and multiple
matches. Root/file native identity, raw name, length, and SHA-256 are stable
across both scans. Missing, orphan, crossed, ambiguous, malformed, unsafe, or
mutating stores refuse before adoption or cleanup; evidence is never deleted,
rewritten, adopted, or automatically completed. A test-only post-result/
pre-candidate hook proves preserved orphan refusal.

Approval and promotion independently validate semantic result identity, exact
result document bytes, candidate identity, current definitions/policy,
lifecycle/observation/coverage authority, and retained registry state before
canonical mutation. Candidate `1.0`, `1.1`, and `1.2` remain historical-only
and require no fallback, dual read, upgrade, copied reference, or approval
carry-forward.

## Atomicity, recovery, and negative closure

The retained scratch-publication design publishes exact verified whole files
through purpose-typed create-new sibling scratch and atomic no-replace rename.
Scratch is never recovery authority. The executable matrices cover every
`W0`-`W15`, `R0`-`R9`, and three-purpose `S0`-`S11` case; all pending-output
state combinations; zero-through-complete scratch and marker prefixes;
collision, unsafe, simultaneous, crossed, replay, concurrent, mismatch, and
evidence-preservation states; retained lifecycle anchors; and journal recovery.

The candidate/approval suites cover coherent result/witness/binding rewrite,
timestamp-only rewrite, wrong exact digest, wrong LF-inclusive length,
semantic/raw crossing, forged second candidate, zero/one/multiple discovery,
missing result, orphan result/candidate, unsafe names/links/files/scan changes,
historical candidate approval/promotion, stale canonical/policy/lifecycle/
observation/coverage authority, and new-approval enforcement. The exact-result
repair suite passes 15 tests; the approval-use suite passes 14 tests; engine
library tests pass 133 tests.

## Product and platform proof

Compiler and CLI authoring persist the exact content, intake, result, and
evidence-store candidate paths. Runtime vectors validate the closed candidate
binding and top-level cross-record contracts. The selected product path retains
setup/doctor `1.2.0`, Environment Inventory's canonical Charter reference,
C04 `reduced-v1-m8.3`, and separate source/render fingerprint domains while
C03 remains unchanged.

The Linux installed-runtime skill smoke was first RED on its stale candidate
`1.2` state-store assertion. The assertion was tightened to require candidate
`1.3` in the evidence store and independently recompute the exact persisted
result SHA-256 and LF-inclusive length. The rerun completed installed author/
replay/refusal flows with terminal `OK`. Independent Linux install/reinstall,
dev-setup, mode-crossover, and public-wrapper smoke also completed `OK`.

## Complete proof replay

| Proof | Result |
|---|---|
| `cargo test --workspace --all-targets --all-features` | PASS on native Windows, 205.4 seconds, every workspace target and feature |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --doc` | PASS, 2 compiler and 7 engine compile-fail doctests |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` | PASS |
| `cargo fmt --all -- --check`, `cargo check --workspace --all-targets --all-features`, `git diff --check` | PASS |
| WSL `tools/ci/codex-skill-live-smoke.sh` | PASS, terminal `OK` |
| WSL `tools/ci/install-smoke.sh` | PASS, terminal `OK` |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS, 172 regular members; 168 same-path source-identical plus source-identical `Cargo.toml.orig`; all 62 definition members exact |
| packaged engine archive | 439,998 bytes; SHA-256 `ba083426474491973e7a19a3ec42bb3ad8ac66f8fffddb474854a503e113ac04` |
| released definition closure | PASS, all 62 baseline definition files byte-unchanged |
| lifecycle-result `1.0` schema | 15,430 bytes; SHA-256 `1d7d8733b19599804fcda32fe119766b1b910e2223aa981fcd9eb92cc4d5c03f`; unchanged |
| promotion-intent `1.2` schema | 16,019 bytes; SHA-256 `c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`; unchanged |
| HCM-2.2 JSON/JSONL and Draft 2020-12 wall | PASS, 26 files, 36 duplicate-rejecting parse units, 13 valid schemas |
| archive boundary normal and negative self-test | PASS |
| handoff validator | PASS, 50 records, 251 current dispatches, 8 admitted legacy dispatches, 50 ledger entries |
| handoff v1-admission and orchestration negative self-tests | PASS |

## Review and landing boundary

The next record is an immutable, sorted path/SHA-256 review dispatch bound to
the complete implementation/control/proof subject. One fresh isolated read-only
reviewer must return Critical, Required, Optional, and Nit findings first. Any
finding changes the subject, requires proof replay and a new dispatch, and goes
to a different fresh reviewer. Only unchanged exact bytes receiving `CLEAN` may
advance through GitNexus change detection, the primary implementation commit,
and the separate parent handoff/ledger closeout commit. HCM-2.3 remains
unauthorized.
