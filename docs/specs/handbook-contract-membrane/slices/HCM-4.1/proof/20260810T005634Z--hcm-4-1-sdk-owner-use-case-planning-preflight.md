# HCM-4.1 planning preflight and proof wall

## Bound state

| Check | Required value | Observation | Status |
|---|---|---|---|
| Assigned root | `C:/Users/spmcc/.codex/worktrees/ebc1/handbook` | exact `git rev-parse --show-toplevel` value | passed |
| Base / tree | `cc44a84c0f5b75f336301dac0c502cfae909c17e` / `54309db1e88e6cfeb1611cf18dfab2d07c10d487` | detached HEAD and tree match | passed |
| Integration ref | local-only expected-old base | `refs/heads/orchestration/handbook-hcm-4-1-planning-20260810T004658Z` resolves to base | passed |
| Remote observation | `origin/feat/handbook-contract-membrane` at `1256e724a2b7da6b6250f57d6f63fced1e2cf949` | unchanged tracking ref | passed |
| Clean starting worktree | empty porcelain | empty | passed |
| Protected checkouts | assigned root differs from every listed protected path | no protected path resolves to assigned root; missing listed worktree paths are recorded as absent, not mutated | passed |
| HCM-3.6 / Phase-3 status | Phase 3 and residual P2 remain open | latest `20260809T233300Z` authority boundary says exactly that | passed |
| HCM-3.5 P5/P6 | later dependencies | Packet 5 gate runtime and Packet 6 ordinary composition remain deferred | passed |

## Planning assertions

1. No Rust, test, fixture, Cargo, dependency, version, feature, schema,
   configuration, generated code, lint, public symbol, or transport file may
   appear in the reviewed subject manifest.
2. No HCM-3.6 implementation or Phase-3 exit claim may be made. The only
   exception is planning HCM-4.1's SDK/ordinary-consumer boundary.
3. The future posture ingress must bind recommendation, policy, approval,
   canonical head, reassessment, exact canonical bytes, and compare-and-write;
   all rejected synthetic routes remain absent.
4. `handbook-compiler` gains no new permanent owner/public API; the target
   dependency graph remains acyclic.
5. The inventory exactly covers the operation IDs in `05` and does not turn
   custom data into generated operations.

## Review/closeout proof

Before primary commit: freeze and replay a sorted SHA-256 manifest, check every
text entry for trailing whitespace, run `git diff --check`, validate each new
v1.4 dispatch, obtain discovery/review/remediation/closure evidence according
to the causal budget, and run scoped plus compare-to-main GitNexus change
detection when available. In this environment GitNexus comparison is
**unavailable**, never GREEN.

Before closeout: run ordinary handoff validation, both v1.4 self-tests,
deterministic ledger rebuild/parity, Markdown/link/identity checks, protected
path re-observation, and local/remote ref rechecks. The permanent ordinary
validator's known historical HCM-3.5 contradiction must be recorded exactly as
failed/not GREEN if it remains the sole failure; any additional failure blocks
this slice.
