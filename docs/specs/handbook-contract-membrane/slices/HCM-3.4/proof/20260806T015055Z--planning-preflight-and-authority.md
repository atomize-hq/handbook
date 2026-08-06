# HCM-3.4 planning preflight and authority proof

## Identity and local baseline

- Meta workflow: handbook-hcm-3-4-planning-20260805.
- Dispatch nonce: fec4262142809e7f9af12781714f4c4a89e409efa7958ad30e50d4cd64109fb9.
- Meta task/host: 019fd4ae-f9ee-7e82-bd09-499df96f9aa5 / local.
- Increment task/host: 019fd4b2-f71b-7e81-8c24-52bcd6fb96b5 / local.
- Assigned mutable checkout: C:/Users/spmcc/.codex/worktrees/d19f/handbook.
- HEAD: 0ca422c6a0150c9f79348eafcaba37b97435b76d.
- HEAD tree: 2106c81c51aebee9d1b171b51e82a11405c072ca.
- Dedicated local integration ref: refs/heads/orchestration/handbook-hcm-3-4-planning-20260806 at the same commit and tree; required-ancestor check passed.
- Assigned worktree is detached, clean, and listed by git worktree list.

## Local-only remote posture

origin is recorded only as the configured remote name. Its local tracking HEAD is refs/remotes/origin/main at a3babd20329027afacdcee9d8b7b9d638d15af5b, matching the dispatch baseline. No fetch, remote query, push, publication, merge, rebase, reset, clean, or force-update occurred. The tracking observation is not remote authority.

## Protected-path baseline

The following Git-visible byte manifests include every tracked or non-ignored file, raw file-object hash, deterministic path ordering, clean staged/unstaged state, and no non-ignored untracked paths:

| Protected path | HEAD/tree | Files | Manifest SHA-256 |
|---|---|---:|---|
| C:/Users/spmcc/Documents/__Project_Code/handbook | 0ca422c6a0150c9f79348eafcaba37b97435b76d / 2106c81c51aebee9d1b171b51e82a11405c072ca | 2054 | 8247ce80ba5380da6f6107bbb6946053bbd8e83fa0cba01d807bd26ace310e0b |
| C:/Users/spmcc/.codex/orchestration/handbook-hcm-3-4-planning | cba8dc7a4889184fe9ddfc8d0ba3dfc0bd78a474 / 545b69efee5f15144f18314b1f5308f22ac110c6 | 2059 | e8f61b516731a7353d0ff43a874d7219c6a255a932b0337bfd01e24924a75661 |
| C:/Users/spmcc/.codex/worktrees/958a/handbook | 0ca422c6a0150c9f79348eafcaba37b97435b76d / 2106c81c51aebee9d1b171b51e82a11405c072ca | 2054 | 8247ce80ba5380da6f6107bbb6946053bbd8e83fa0cba01d807bd26ace310e0b |

## Predecessor and authority admission

The selected HCM-3.3 v1.4 final handoff is completed with stop_reason completed, records a final CLEAN review and no unresolved P1/P2, and names no automatic continuation. It is immutable dependency context, not implementation authority. The scoped HCM-3.4 planning-only contract selects this new planning outcome and does not modify HCM-3.3.

The phase-map HCM-3.4 row and exact Snapshot Memory semantics/contracts/proof rows are loaded. The slice is documentation-and-planning-only: no production symbol is selected or edited, therefore upstream code-symbol impact analysis is not applicable. GitNexus MCP tools are absent from this session; scoped and compare-to-main change detection must be reported as unavailable, never GREEN.

## Planning proof wall

Before each review dispatch, replay the full HCM-3.4 planning manifest, inventory every allowed planning path, check UTF-8 whitespace and git diff --check, and validate the v1.4 dispatch with validate_handoffs.py --verify-dispatch. At closeout, validate the handoff/ledger, run both required self-tests, inspect exact paths, recheck these protected manifests, and update only the dedicated local integration ref by expected-old compare-and-swap.

No product behavior test was run or claimed: this packet plans future evidence only.
