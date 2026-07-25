# HCM-2.3 Implementation Review 1 Remediation

> **Status (2026-07-22): remediation implemented and proof wall green; fresh
> Review 2 pending.** This record does not claim `CLEAN`, a primary commit,
> closeout, or authority for HCM-2.4 or any later phase.

## Review admission and verdict

Fresh isolated Review 1 admitted all 43 paths and recomputed the exact aggregate
subject from
[`20260722T155754Z--HCM-2-3--fresh-complete-implementation-review-1.json`](../../../handoffs/dispatches/20260722T155754Z--HCM-2-3--fresh-complete-implementation-review-1.json),
aggregate
`sha256:2027b8b283abc807b0ad2cb3c3eeef87e17cb3fad3aaa907fde48203f4cc350c`.
It returned `CHANGES_REQUIRED` with three Critical and ten Required findings.
Every finding is accepted without waiver; the immutable Review 1 dispatch and
subject hashes remain unchanged.

## Accepted findings

1. Generic operations did not retain recovered repository authority and an
   exact repository identity through generic-lock acquisition and context
   resolution.
2. Ordinary readers could expose canonical bytes installed by a promotion whose
   commit marker had not yet been published.
3. Generic-store path operations did not retain native component identity across
   inventory, open, hash, creation, and rename boundaries.
4. Built-in intake source bindings were silently skipped.
5. Intake registration omitted complete schema-relative coverage
   meta-validation.
6. Pure evaluation did not structurally validate a constructed preview and
   collapsed blocked coverage provenance into a generic error.
7. Generic intake persistence assumed the registry-brief two-field closure.
8. Candidate preview mismatch selected an invalid established-refusal
   code/layer/nullability tuple.
9. Inventory grammar and aggregate/per-instance count and byte ceilings were
   incomplete.
10. `verified.json` did not equal the exact staged-output descriptor projection
    of its intent.
11. Committed tombstone/ledger states were not closed and exactly bound.
12. Timestamp sampling preceded lock acquisition and establishment recovery
    omitted the transaction-family durability sync.
13. The proof/todo/control-pack completion claims exceeded the executable
    evidence and must be rolled back until the repaired subject is independently
    CLEAN.

## Remediation rule

Each behavior repair returns first to a named failing regression. Production
changes remain inside the selected HCM-2.3 engine/CLI owners; the explicitly
authorized schema-root separation remains confined to `schema_registry.rs`.
No package definition, Cargo dependency, HCM-2.2 behavior, HCM-2.4, Phase 3+,
Phase 4 SDK/transport, preservation worktree, or archive is in scope. A new
complete proof wall and a different fresh complete-subject reviewer are required
before commit.

## Implemented remediation evidence

| Finding | Implemented evidence |
|---|---|
| 1 | `ArtifactRepositoryAuthorityGuardV1` retains promotion/registry locks, recovers authority, captures repository identity and registry state, and revalidates before generic replay lookup and intent establishment. |
| 2 | Generic read, validate, evaluate, currentness, and candidate-validation paths enter one recovered generic-store read session before opening fresh repository truth; the markerless installed-promotion regression passes. |
| 3 | Strict reads retain file and Windows directory handles; Windows denies write/delete substitution while retained; Unix uses no-follow descriptor-relative open/create/rename/unlink; bounded inventory and strict handle reads reject links, non-regular files, mutation, traversal, unknown names, and overflow. |
| 4 | BuiltIn intake sources load exclusively through `profile_builtins::definition`; repository intake sources remain trusted repository reads; BuiltIn compatibility definitions are non-generic. |
| 5 | Intake registry meta-validation proves every target is an exact finite schema leaf, all leaves are covered, row targets are type-compatible, and ancestor/descendant overlap is absent. |
| 6 | Known-unknown and contradicted submissions produce bounded auditable `Blocked` evaluations with no content authority; completed previews pass the selected structural validator before use. |
| 7 | Intake plans persist one through fifteen distinct value closures, deduplicate equal fingerprints, retain every coverage row, and prove exact subordinate closure. |
| 8 | Candidate mismatch persists the closed `StaleBasis` / `Currentness` tuple with non-null unequal fingerprints; actual CLI refusal and restart replay are green with zero authoritative outputs. |
| 9 | Exact transaction/family/instance grammar plus transaction, pending, committed, semantic, closure, per-record, per-instance, and aggregate count/byte/depth limits are enforced and tested. |
| 10 | `verified.json` must equal the intent's exact staged-descriptor projection in both staging and installed-complete states; a re-fingerprinted descriptor mutation refuses. |
| 11 | Ledger states, identities, request/transaction binding, retained result, and closed tombstone nullability are validated; unknown and nonclosed states refuse. |
| 12 | Production time is sampled under lock after recovery; establishment flushes the transaction family before intent rename. |
| 13 | Control-pack language now says uncommitted/review-pending, classifications are candidate-only until CLEAN review and primary commit, and the implementation proof records the complete post-remediation executable wall without claiming `CLEAN`. |

## Reproof result

Focused registration 11/11, generic lineage 19/19 on Windows and WSL, schema
source-domain 5/5 on Windows and WSL, direct schema-registry 19/19, and actual
CLI 5/5 pass. The complete HCM-2.2 focused wall, Windows and WSL Clippy, engine,
CLI, and complete workspace tests pass. Package/dependency, archive-boundary,
handoff normal/v1/orchestration validation, preservation hashes, scope, and
`git diff --check` pass; the unchanged CLI packaging metadata limitation is
recorded exactly. The next gate is GitNexus change detection followed by a
different fresh isolated complete-subject review.
