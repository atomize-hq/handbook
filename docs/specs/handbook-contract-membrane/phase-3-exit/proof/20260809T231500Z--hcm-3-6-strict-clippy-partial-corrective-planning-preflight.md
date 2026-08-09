# HCM-3.6 strict-Clippy partial-corrective planning preflight

**Parent:** `handbook-hcm-3-6-strict-clippy-partial-corrective-planning-20260809`

**Purpose:** record the read-only planning baseline for the bounded partial
candidate. This proof does not admit a source edit or promote Phase 3.

## Control and resume inputs read

The live `AGENTS.md`, the full `00` through `09` Handbook control pack, the
HCM-3.6 SPEC/plan/todo/implementation selector/source-impact/proof wall, both
HCM-3.6 immutable authority-boundary records, the Phase-3 exit selector,
true-stop, strict-Clippy impact and authority-stop records, and the HCM-3.5
future dependency plan were read before this selector was drafted.

Relevant constraints retained in this planning package are: v1.4 parent-owned
dispatch/review/closeout lineage; no P1/P2 waiver; immutable predecessor
records; full fresh review for material control changes; local-only workflow;
and HCM-3.5 Packet 5/6 deferral behind HCM-4 SDK/CLI/machine transport and
later runtime/consumer authority.

## Baseline state

| Check | Result |
| --- | --- |
| Working checkout | clean, detached at `8c2e31b518007b4a2b14f36f128d35b04c6e632e` / tree `dfe7fac4370e9f6ca20fba0fa22e9b4dd4098f4d` |
| Protected checkout | not touched; reserved read-only at `C:/Users/spmcc/Documents/__Project_Code/handbook` |
| Remote/ref/branch action | none; no fetch, push, CAS, branch change, or publication |
| Strict gate | `cargo clippy --workspace --all-targets --all-features -- -D warnings` exited `101` with 29 diagnostics |
| Candidate reduction | the consumed authority-stop evidence proves a previously reverted six-file subset reduces the 29 diagnostics to 20 without reaching the remaining P2 |
| GitNexus FTS | unavailable; no FTS/query result is claimed GREEN |
| Current planning symbol edits | none; no fresh symbol impact is required or claimed |

## Exact live diagnostic partition

The live strict-gate replay shows 20 private-reachability diagnostics that are
outside this plan: 7 in `charter_authority_transaction.rs`, 3 in
`charter_lifecycle_transition_v11.rs`, 8 in
`charter_posture_transaction_intent_v1.rs`, and 2 in `project_posture.rs`.
The independently safe partial partition is nine diagnostics in six paths:
one lifetime elision, one mutable-vector-to-slice signature, three Option
predicate spellings, one redundant closure, two private type aliases, and one
test-only sort idiom. The selector names every location and invariant.

The 20 diagnostic P2 is neither accepted debt nor an inventory candidate. Its
owner remains `user / authorized product authority`; its exact resumption
condition is a new explicit reachability or semantic-adoption authority.

## GitNexus truth

The current session exposes no GitNexus MCP `impact`/`context` tool. This is a
planning-only run with no source-symbol edit. The consumed
[`20260809T164900Z--hcm-3-6-strict-clippy-impact-analysis.md`](20260809T164900Z--hcm-3-6-strict-clippy-impact-analysis.md)
records the prior source attempt's callgraph impacts and its unavailable FTS
status; it is retained only as history. A later implementation must run fresh
impact/context for each edited existing symbol and treat unavailable FTS as
unavailable, not as GREEN.
