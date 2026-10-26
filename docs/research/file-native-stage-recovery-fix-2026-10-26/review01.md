# First fresh focused recovery review

Reviewer: `/root/stage_recovery_safety_review`, fresh `reviewer_v1`, read-only. Available role capability GPT-6.1 High/default was disclosed; no test/build/index/mutation was performed by the reviewer.

The complete original returned observation follows.

Focused recovery review found **one P2 blocker**. No other qualifying blocker was found.

**SR-01 — Legitimate aborted-owner adoption becomes unrecoverable after deferral.** Priority P2; high confidence from decisive source evidence.

The new equality check at physical_lifetime.rs:294 rejects any pending checkpoint whose `birth_claims` differ from the selected Work. However, cleanup legitimately changes this field:

1. Cleanup calls `adopt_aborted` at packing.rs:855.
2. That function adds the aborted owner's birth claim at packing.rs:618, inherits its garbage, and saves the checkpoint.
3. Budget exhaustion or process termination can leave the complete checkpoint unselected, either before or after immutable installation.
4. The next cleanup calls `physical::load` during adoption and rejects that checkpoint with `integrity:work-stage-frozen-generation`.

Repeated cleanup therefore fails instead of resuming legitimate owned progress. The installed-checkpoint case previously accepted this transition: the baseline validator did not compare `birth_claims`. This regression is directly caused by the repair's shared-validator change. The new test categorizes every birth-claim addition as foreign, leaving the actual adoption path uncovered.

Resolution needs to distinguish authorized aborted-owner adoption from forged birth claims while preserving ownership validation.

The assessed subject was branch `codex/file-native-accepted-checkpoint-20261026`, baseline `528de30ffdca065e024f5c7de142d24f6755ec55`, with:

- Frozen-subject SHA256 `f86310ae3dd2dd15efb852da8fd37f22a629b68e64cfafe9536ce03862218caa`.
- Source-manifest SHA256 `081af3911043dba998a122c4709861a2bef00bb4250b21b71436d415ae2b26cf`.

All 27 source files and 78 evidence files matched the freeze before and after observation. All 74 decoded ORIGINS entries matched their recorded hashes.

I inspected the exact delta, changed assertions, persistence ordering, native identity checks, selection gates, reader protection, cleanup callers and deletion accounting. Retained raw evidence supports the unchanged discriminator's fail-to-pass result, 20 checkpoint entries passing, the Work-bound pass, Linux production check and seven native child cuts. Final test-binary bindings match the frozen source and retained executable. No tests, builds, indexing or mutations were performed during this review.

Limitations remain explicit: no performance, full-workspace or other-host qualification; fake-clock tests do not establish native timing. GitNexus rejected the unregistered repository, so callers were inspected directly. The structured-format standard linked by live AGENTS.md is absent from this checkout; its contents could not be reviewed. Existing strict JSON and versioned contracts were inspected, and no format migration was introduced.

**Observation result: SR-01 remains an implementation blocker.** This report does not establish strategic disposition or task acceptance.

## Owner assessment

SR-01 is validated. The first candidate's assertion that cleanup never changes birth claims was incorrect. `adopt_aborted` is a real cleanup caller, and the extra equality check was an unintended strengthening beyond the baseline checkpoint validator. The frozen first candidate and evidence remain unchanged; its three changed source files are retained under `review01-source/`. Remediation restores the baseline treatment of mutable birth claims while retaining strict bounds/identifier validation, actual current-owner/selector/digest/generation validation, and the construction identity/persistence checks. It does not add a new journal authorization mechanism or change the serialized contract. Real adoption-generated payloads are exercised at both complete construction and installed frontiers, including durable journal ownership handoff after successful selection. A different fresh focused reviewer will inspect the repair and immediate consequences.
