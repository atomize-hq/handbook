# Phase-3 exit selector preflight

**Selector candidate:**
[`../decision/20260809T143000Z--phase-3-exit-audit-selector.md`](../decision/20260809T143000Z--phase-3-exit-audit-selector.md)

## Bound local state

| Check | Required | Observed | Result |
|---|---|---|---|
| Task worktree | `C:\\Users\\spmcc\\.codex\\worktrees\\92a7\\handbook` | that exact root | pass |
| HEAD / integration ref | `bfedb7131a66bbed6a574ee2a90336439161b117` | same | pass |
| HEAD / integration tree | `9e51ebdde2ed57ed7fa17c607e34d15dbba785da` | same | pass |
| Required ancestor | `1256e724a2b7da6b6250f57d6f63fced1e2cf949` | ancestor of HEAD | pass |
| Remote tracking baseline | `origin/feat/handbook-contract-membrane` at `1256e724a2b7da6b6250f57d6f63fced1e2cf949` | same local observation; no fetch | pass |
| Task worktree status | clean and detached from protected branch | clean | pass |
| Protected checkout | clean at the same base/tree, feature branch not integration ref | pass | pass |
| Protected index manifest SHA-256 | `c5e7811645a52bdc861a85822432eb835acb2f2fc2c9b197166ca14fba5059d8` | same | pass |
| Protected status SHA-256 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | same | pass |
| GitNexus CLI | `1.6.9` | `npx --no-install gitnexus --version` returned `1.6.9` | pass |

## Selector admission conclusion

The exact local base, tree, ancestor, integration-ref ownership, remote-
tracking observation, clean task worktree, and protected checkout satisfy the
bound entry conditions. This is pre-review evidence only: it does not make the
candidate selector effective, change Phase-3 status, interpret a validator
failure as GREEN, or authorize any product change.
