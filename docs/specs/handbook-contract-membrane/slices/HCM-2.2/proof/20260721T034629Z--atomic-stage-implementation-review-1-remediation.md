# HCM-2.2 Atomic-Stage Implementation Review 1 Remediation

> **Status (2026-07-21): remediated complete subject awaiting a different fresh
> isolated Review 2.** This record does not claim `CLEAN`, a primary commit,
> closeout, or HCM-2.3 authority.

## Review admission and verdict

Fresh isolated Review 1 admitted all 194 paths and the exact aggregate subject
from
[`20260721T022507Z--HCM-2-2--fresh-atomic-stage-implementation-review-1.json`](../../../handoffs/dispatches/20260721T022507Z--HCM-2-2--fresh-atomic-stage-implementation-review-1.json),
aggregate `sha256:ad6287c46021f4fb7cd8c2f1fd1cc91a0f6d680051dde88913c864f8a5b6254f`.
It returned `NOT CLEAN` with five Required findings. Every finding was accepted
without waiver; the immutable Review 1 dispatch remains unchanged.

## Required findings and bounded repairs

1. **W9 recovery retained authority.** Roll-forward recovery revalidated the
   current canonical/lifecycle closure but did not reread the retained candidate
   intake and normalized-content authorities. One shared candidate-validation
   authority check now serves both normal preflight and W9 recovery. The RED
   regression mutates each retained authority after W9, proves refusal before
   commit, and proves the complete pending journal remains evidence.
2. **Historical terminal authority.** Selected-head reads could skip malformed
   older committed or rolled-back journals. Recovery and every selected reader
   now validate every exact terminal payload, transaction-name binding, marker,
   final output, duplicate suffix, and one reachable create-to-current successor
   chain. The real author/approve/promote create-then-amend product path proves
   an intact successor chain; malformed historical marker, name, final, and
   rollback cases all preserve and refuse.
3. **Lifecycle observation order.** The state fingerprint previously sorted
   active observations and erased committed-head order. It now preserves the
   exact retained sequence while separately rejecting duplicates. Forward and
   reversed sequences produce different fingerprints, and reordered result
   authority refuses.
4. **Production test controls.** Lifecycle and promotion fault selection plus a
   caller clock hook leaked into the production feature graph/API. Fault enums,
   injected-error variants, thread-local selectors, and injection helpers are
   now module-private `#[cfg(test)]`; ordinary writer/recovery methods carry no
   fault arguments; the caller-selected promotion clock is absent. A dedicated
   production-source guard and exhaustive module-local S/W/R/lifecycle matrices
   prove both halves of the boundary.
5. **Lifecycle-result audit-byte immutability.** The content identity correctly
   excludes `validated_at_utc`, but an existing ID therefore needed a separate
   exact-byte authority. The engine now create-new publishes an immutable
   same-filesystem witness outside the semantic identity cycle. Persistence and
   promotion require result/witness byte equality. Witness-only crash state
   restores the exact authored result; result-only, unequal, forged, and
   timestamp-only states preserve evidence and refuse.

GitNexus reproduced the planned HIGH/CRITICAL promotion and lifecycle blast
radii before existing-symbol edits. The review repairs stayed inside the
authorized transaction recovery, lifecycle validation/store, selected product
reader, and test surfaces. Newly introduced uncommitted helpers remained absent
from the clean-HEAD index and therefore reported UNKNOWN with zero known
upstream expansion.

## Remediated proof replay

| Proof | Result |
|---|---|
| focused Review 1 RED regressions | PASS: W9 retained intake/content; historical terminals; observation order; production surface; result timestamp/witness cases |
| engine unit suite | PASS, 128 / 128 |
| lifecycle-validation authority integration | PASS, 6 / 6 |
| lifecycle-store integration | PASS, 5 / 5 |
| atomic-stage module | PASS, 17 / 17, including every purpose × `S0`-`S11`, `W0`-`W15`, and `R0`-`R9` replay pair |
| `cargo test --workspace --all-targets --all-features` | PASS on native Windows for the remediated exact bytes |
| strict workspace Clippy | PASS after correcting two test-only style findings |
| workspace doctests and warning-free rustdoc | PASS: two compiler plus seven engine compile-fail doctests |
| formatting and whitespace | PASS: `cargo fmt --all -- --check` and `git diff --check` |
| WSL install/reinstall/dev/public-wrapper smoke | PASS, terminal `OK` |
| WSL installed live-skill smoke | PASS, terminal `OK` |
| engine package/member replay | PASS: 172 regular members, 169 source-identical, all 62 definition assets exact |
| packaged engine archive | SHA-256 `546ee43a0da76c9c497ac72256d5faded8aeedecb5bb14b5ab58335828f4c074` |
| HCM-2.2 JSON/JSONL and Draft 2020-12 wall | PASS: 36 duplicate-safe parse units and 13 schemas |
| frozen promotion intent `1.2` | PASS: unchanged 16,019-byte schema, amendment fingerprint, 7,607-byte document hash, and 72-byte marker |
| handoffs and negative self-tests | PASS: 49 records, 246 current dispatches, eight admitted legacy dispatches, 49 ledger entries; both self-tests fail closed |
| archive boundary normal/self-test | PASS |
| external comparison archive | PASS: SHA-256 `29a7863787dc78902a2eb1dd948a2fbc9830b94a85fead40283a8c54d603cb9e` |

The original dirty implementation worktree remains an unmodified comparison
checkpoint with its original 13-path dirty inventory. WSL-generated
`.implemented/` logs were removed from the authority worktree after their
terminal results were captured; they are reproducible output and are not part
of the review subject.

## Next gate

Freeze a new exact sorted path/SHA-256 manifest and dispatch the complete
remediated subject to a **different fresh isolated reviewer**. Any finding again
requires acceptance without waiver, a new RED remediation, full proof replay,
and another different fresh reviewer. Only a complete-subject `CLEAN` verdict
may advance to final byte replay, GitNexus change detection, the scoped primary
implementation commit, and the separate handoff/ledger closeout commit.
