# HCM-2.2 Atomic-Stage Implementation Review 2 Remediation

> **Status (2026-07-21): all Review 2 findings repaired; awaiting a new fresh review.**
> This record does not claim `CLEAN`, a primary commit, closeout, or authority
> for HCM-2.3.

## Review admission and disposition

Dispatch
`20260721T035019Z--HCM-2-2--fresh-atomic-stage-implementation-review-2`
bound 197 unique sorted paths and aggregate
`sha256:95da29f080ae05bed82b46bb1c03ce59af12f3feba4c8a25cba1f4a2b25f3a55`.
The first assigned Review 2 agent produced no verdict because its response was
stopped by an automated content filter. No subject byte changed. A different
fresh isolated reviewer, `/root/hcm_2_2_atomic_review_2b`, independently
admitted all 197 paths and the exact aggregate, remained read-only, and returned
`CHANGES_REQUIRED` with three Required findings. The subject was rejected and
none of its bytes were committed.

All three findings were accepted without waiver:

1. witness-only lifecycle-result bytes could be changed and restored because
   result identity deliberately excludes `validated_at_utc`; changing the
   result and witness equally also replayed;
2. promotion transaction-root entries with unsupported suffixes were silently
   ignored; and
3. pending recovery could mutate or terminalize a journal before malformed
   committed/rolled-back history was rejected.

## RED regressions

The following tests failed against the admitted Review 2 bytes before
production remediation:

- `mutated_witness_only_state_refuses_without_restoring_result`;
- `equal_result_and_witness_mutation_refuses_replay_and_promotion`;
- `unsupported_transaction_root_suffix_is_preserved_mismatch`; and
- `malformed_history_blocks_pending_recovery_before_mutation`.

The observed failures reproduced each review report exactly: forged witness
bytes restored a missing result, equal result/witness mutation replayed,
`.other` returned no transaction, and a recoverable pending journal changed
before malformed history refusal.

## Bounded repairs

Lifecycle-validation persistence now publishes one independently retained,
closed, self-fingerprinted exact-byte binding before its raw witness and result.
The binding includes the complete result JCS, result document SHA-256, byte
length, semantic result fingerprint, and a binding fingerprint whose preimage
includes the audit timestamp bytes. Bounded no-follow replay verifies the
binding first. Binding-only or binding-plus-witness writer crash states can
restore the exact original result; missing binding, a result without its
witness, changed witness bytes, changed result bytes, or equal result/witness
mutation refuses and preserves all observed evidence. Promotion uses the same
three-way exact authority check before approval resolution.

Promotion recovery now performs one closed transaction-root inventory. Only
the two exact ignored scratch roots and grammar-valid `.pending`,
`.committed`, and `.rolled-back` entries are admitted. Unknown/crossed
names, unsupported suffixes, simultaneous suffixes, non-UTF-8 names, and unsafe
transaction types refuse without cleanup. The inventory validates every
terminal payload, final binding, duplicate, and sole create-to-current chain
before any pending journal executes. After pending recovery it inventories and
validates the resulting selected head again.

GitNexus remained current at entry commit `486458ac`. The new
lifecycle-binding and terminal-inventory helpers were not present in the entry
index and reported `UNKNOWN` with no discovered callers. Existing
`recover_pending_locked` remained HIGH with three direct callers, thirteen
impacted symbols, and three processes. Existing `transaction_directories`
remained HIGH with three direct callers, fifteen impacted symbols, and four
processes. The affected processes stayed inside reviewed promotion,
retained-authority, selected-reader, lifecycle, and test paths; no additional
module or execution-flow expansion was admitted.

## Exact-byte proof replay

| Gate | Result |
|---|---|
| focused Review 2 RED regressions | PASS: all four are GREEN |
| atomic-stage module | PASS: 19 / 19, including every purpose × `S0`-`S11`, `W0`-`W15`, `R0`-`R9`, suffix, terminal, and history-before-mutation case |
| engine unit suite | PASS: 130 / 130 |
| lifecycle-validation authority integration | PASS: 8 / 8 |
| complete engine all-target/all-feature suite | PASS |
| native Windows workspace all-target/all-feature suite | PASS |
| strict workspace Clippy | PASS with `-D warnings` |
| workspace doctests and warning-free rustdoc | PASS: two compiler plus seven engine compile-fail doctests |
| formatting and whitespace | PASS: `cargo fmt --all -- --check` and `git diff --check` |
| WSL install/reinstall/dev/public-wrapper smoke | PASS, terminal `OK` |
| WSL installed live-skill smoke | PASS, terminal `OK` |
| engine package/member replay | PASS: 172 regular members, 169 source-identical inputs, all 62 definition assets exact |
| packaged engine archive | SHA-256 `edf841294da67bc54c2df17c74493c2af9eb9dfc31bb45675f1ddd700efe44d7` |
| HCM-2.2 JSON/JSONL and Draft 2020-12 wall | PASS: 36 duplicate-safe parse units and 13 schemas |
| frozen promotion intent `1.2` | PASS: unchanged 16,019-byte schema; amendment fingerprint `sha256:d7f3c3a9ee860288f0ed52c83254044d67828d2499909633584290d8ec265830`; 7,607-byte document SHA-256 `sha256:ad48fe8c0be1d3ee72888a6409d7e9ef3278623bf2c68c5cb199a3db18a08654`; 72-byte marker |
| handoffs and negative self-tests | PASS: 49 records, 247 current dispatches, eight admitted legacy dispatches, 49 ledger entries; both self-tests fail closed |
| archive boundary normal/self-test | PASS |
| membrane relative links | PASS: 174 checked, zero missing |
| external comparison archive | PASS: SHA-256 `29a7863787dc78902a2eb1dd948a2fbc9830b94a85fead40283a8c54d603cb9e`; all 13 live evidence paths byte-exact |

The original dirty worktree still has its exact 13-path inventory and remains
read-only. WSL-generated `.implemented/` proof logs were removed after their
terminal results were captured; they are reproducible output and are excluded
from the implementation subject.

## Next gate

Freeze a new exact manifest that includes this remediation record, dispatch it
to another different fresh isolated read-only reviewer, and accept every
finding without waiver. Primary implementation and mechanical closeout commits
remain prohibited until a complete exact subject is `CLEAN`.
