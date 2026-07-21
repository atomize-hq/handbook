# HCM-2.2 Atomic-Stage Implementation Proof Wall

> **Status (2026-07-21): exact implementation subject awaiting fresh review.**
> This wall records P0-P8 implementation and proof. It is not a `CLEAN` review,
> primary commit, closeout, or HCM-2.3 authorization claim.

## Authority and immutable comparison evidence

- phase/slice: `HCM-2` / `HCM-2.2`;
- branch: `feat/hcm-2-2-atomic-stage-authority-repair`;
- clean implementation entry:
  `486458acfe8373a977e595a4854ac786166e3e76`;
- selected handoff:
  `20260720T225541Z--HCM-2-2--orchestration--atomic-stage-authority-repair-approved`;
- planning primary: `df5acae4`, Review 7 `CLEAN`;
- mechanical planning closeout descendant: `486458ac`; and
- later-slice authority: none.

Entry reproduced the exact HEAD, clean worktree, direct primary/closeout
lineage, handoff/schema/vector/link/ledger wall, and every named authority
reference. The original dirty implementation worktree remained read-only. Its
external archive was reverified at SHA-256
`29a7863787dc78902a2eb1dd948a2fbc9830b94a85fead40283a8c54d603cb9e` before
being used only for hunk-by-hunk comparison. No original documentation,
checklist, research, handoff, authority, ledger, or proof byte was imported.

## Reproduced impact boundaries

GitNexus was refreshed before implementation. Upstream impact was reproduced
before every existing function, class, or method edit. The accepted bounded
high-risk boundaries included:

| Existing symbol/boundary | Reproduced risk and bounded blast radius |
|---|---|
| `evaluate_charter_intake` | HIGH; seven direct callers, eleven total impacts across compiler/CLI intake |
| generic lineage `validate_record` | CRITICAL; ten direct callers, 73 total impacts, twenty processes; changed only by candidate-specific historical-version refusal |
| `CharterLifecycleStoreV1::current_promotion_anchor` | HIGH; retained lifecycle observation only |
| promotion transaction read/recovery/currentness group | HIGH/CRITICAL graph reachability; contained to the reviewed transaction, lifecycle, approval, and selected-reader flows |
| transaction `read_bounded_regular` | CRITICAL; four direct callers, thirteen total impacts, six processes |
| lifecycle `read_bounded_regular` | HIGH; six direct callers, fourteen total impacts, three processes |

The two retained readers now open once with native no-follow semantics, reject
Windows reparse handles, validate the same handle as bounded regular content,
and consume at most `limit + 1` bytes. No format, caller, module, or execution-
flow expansion was admitted. New atomic-stage helpers were unindexed/UNKNOWN
with zero known upstream expansion and remained private to the reviewed
transaction module.

## Implemented authority chain

The bounded implementation establishes one acyclic lineage:

1. candidate `1.2` subject bytes and subject fingerprint;
2. engine-owned lifecycle-validation result `1.0`, persisted JCS+LF by exact
   no-follow/create-new/rename-no-replace-or-exact-equal authority;
3. final content-addressed candidate `1.2` with exactly one result ref;
4. human approval over that final candidate; and
5. promotion only after complete subject/result/content/intake/profile/policy/
   lifecycle/active-observation/reopened-coverage/registry/approval currentness.

Candidate `1.0`/`1.1` remains historical-only. The generic lineage validator
was not widened to own the lifecycle result. Compiler and CLI authoring persist
and project the exact four immutable paths: normalized content, intake record,
lifecycle-validation result, and final candidate.

Promotion intent `1.2` is a closed JCS+LF record with engine-random transaction
identity distinct from promotion identity. Lifecycle observation reads only the
nested intent-1.2 authority. Precommit and W9 recovery validate the bound prior
canonical/lifecycle state, current retained registry head, selected approval
journals, candidate/result bindings, and the exact intended lifecycle
successor. Historical committed journals remain valid through a later committed
successor chain without requiring an old head to stay current forever.

## Atomic stage and recovery proof

Intent and all three outputs use non-authoritative same-filesystem sibling
scratch. Each output receives an independent engine-random 128-bit purpose-
typed create-new name. The writer performs bounded write, file fsync, close,
native no-follow reopen, exact byte/hash/length/type/binding verification,
scratch-directory fsync, atomic rename-no-replace to the exact pending stage,
ordered pending/scratch/transaction-parent fsyncs, and exact pending-stage
revalidation.

`canonical.new`, `promotion-record.new`, and `lifecycle-transition.new` have
only `{absent, exact, mismatch}`. Partial pending stages are always preserved
evidence and refusal. `canonical.old` retains only its separately authentic
exact-prefix completion rule; marker temporaries retain only their derivable
72-byte exact-prefix rule. Scratch is ignored and preserved by selected readers
and recovery.

The executable proof covers:

- the exact three-purpose by `S0`-`S11` Cartesian product, with missing or
  duplicate pairs rejected;
- every zero-through-complete bounded scratch prefix for all three purposes;
- `W0`-`W15` and `R0`-`R9` from every admitted old-authority origin;
- all 27 pending new-output stage combinations, admitting only the four
  monotone writer prefixes;
- all zero-through-72-byte marker prefixes and every amendment old snapshot
  prefix;
- both terminal markers at all four pending-fsync replay boundaries;
- both terminal destinations crossed with absent, exact-pre-existing,
  mismatching, and unsafe states;
- intent/output scratch collisions, simultaneous suffixes, unsafe destinations,
  orphan scratch, replay, concurrent readers/writers, and exact evidence
  preservation; and
- real create-then-amend author/registry/ES256 approval/promotion/lifecycle
  product authority.

On Windows, no-replace file and directory publication uses
`TempPath::persist_noclobber`; collision tests prove that the source and
destination evidence are retained. Unix production builds retain native
`renameat(..., RENAME_NOREPLACE)` and passed both WSL release installs.

## RED regressions and repairs

Focused RED tests preceded production increments. The complete workspace replay
later found two stale product-proof contracts:

1. CLI flow fixtures attempted to append historical candidate `1.1` under the
   new current authority. The shared fixture now constructs authentic candidate
   `1.2`, lifecycle-validation result `1.0`, approvals, promotion/lifecycle
   records, and strict intent `1.2`; all 98 CLI surface tests pass.
2. The live installed-skill smoke expected three authoring paths. It now requires
   the exact four immutable partitions and verifies candidate/result subject
   identity and the single result ref. The full live smoke then completed `OK`.

Neither repair relaxed production validation.

## Proof replay

| Command or proof | Result |
|---|---|
| `cargo test -p handbook-engine --lib` | PASS, 125 / 125 |
| `cargo test --workspace --all-targets --all-features` | PASS on native Windows across every target and feature |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --doc` | PASS, two compiler plus seven engine compile-fail doctests |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` | PASS |
| `cargo fmt --all -- --check` and `git diff --check` | PASS |
| WSL `tools/ci/install-smoke.sh` | PASS, terminal `OK` for install/reinstall/dev/public-wrapper |
| WSL `tools/ci/codex-skill-live-smoke.sh` | RED on stale three-path assertion; strengthened four-path replay PASS, terminal `OK` |
| engine package/member replay | PASS: 171 regular members, 168 source-identical, 62 exact definitions, zero mismatches |
| packaged engine archive | SHA-256 `b1424db426af9d1a8ac72f064c6098354f79eb5a419f31ab0e4790605bb3f199` |
| HCM-2.2 JSON/JSONL and Draft 2020-12 wall | PASS: 36 parse units with duplicate rejection; 13 schemas accepted |
| frozen intent `1.2` identity | PASS: 16,019 schema bytes, SHA-256 `c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa` |
| amendment intent/marker vector | PASS: fingerprint `d7f3c3a9...`, 7,607 bytes, document SHA-256 `ad48fe8c...`, exact 72-byte marker |
| handoff validator | PASS: 49 records, 245 current dispatches, eight admitted legacy dispatches, 49 ledger entries |
| handoff negative self-tests | PASS: historical-v1 admission and orchestration contract |
| archive boundary normal and self-test | PASS |
| HCM-2.2 relative Markdown links | PASS, zero missing targets |

The first handoff command attempted Windows Python, which lacks the external
`jsonschema` module. WSL Python supplied Draft 2020-12 validation. Normal corpus
validation used WSL-visible `GIT_DIR`/`GIT_WORK_TREE` for this Windows linked
worktree; the orchestration self-test ran without that override so its own
temporary Git repositories remained isolated.

## Review and landing gate

The next step is an immutable, sorted path/SHA-256 exact-subject dispatch to one
fresh isolated read-only reviewer. Every finding is accepted without waiver.
Any remediation changes the subject, requires relevant/full proof replay, and
uses a different fresh reviewer. Only exact `CLEAN` bytes may receive GitNexus
change detection, the scoped primary implementation commit, and the separate
parent handoff/ledger closeout commit. HCM-2.3 remains unauthorized.
