# HCM-3.6 strict-Clippy corrective impact analysis

## Scope and method

This record belongs to the fresh `HCM-3.6` strict-Clippy corrective parent
`handbook-hcm-3-6-strict-clippy-corrective-20260809`. It is not a Phase-3 exit
claim. The authority boundary is
`20260809T152000Z--HCM-3-6--orchestration--phase-3-exit-strict-lint-authority-boundary`;
the active selector is
`20260809T163659Z--phase-3-strict-clippy-corrective-selector.md`.

Before implementation, upstream callgraph impact was run with GitNexus 1.6.9
in the assigned worktree for each existing Rust item that the correction may
change. The command shape was:

```text
npx --no-install gitnexus impact --repo C:\Users\spmcc\.codex\worktrees\a05e\handbook --direction upstream --include-tests --summary-only --file <admitted-path> <symbol>
```

GitNexus's FTS extension is unavailable in this environment. That limitation
does not affect the callgraph impact results, but no FTS-derived query result is
claimed as GREEN.

## High-risk results and restraint

| Symbol | Direct / affected | Affected processes | Risk | Bounded correction |
| --- | ---: | ---: | --- | --- |
| `CommittedAuthorityHeadV1` | 1 / 17 | 3 | HIGH | Visibility-only lint reachability correction; preserve fields, construction, and values. |
| `PostureTransitionDraftV1` | 2 / 12 | 0 | HIGH | Visibility-only lint reachability correction; preserve draft data and validation semantics. |
| `encode_jcs_lf_v1` | 4 / 16 | 0 | HIGH | Visibility-only lint reachability correction; preserve encoding bytes and callers. |
| `normalize_observation` | 1 / 14 | 2 | HIGH | Replace only equivalent `Option` predicate spelling; preserve validation and normalization result. |

The `CommittedAuthorityHeadV1` chain reaches the posture-preflight process and,
indirectly, `finalize` and `approve_inner`; the proposed edit cannot change its
data layout or runtime behavior. `PostureTransitionDraftV1` and
`encode_jcs_lf_v1` are exercised by the private posture transaction tests and
the lifecycle-store rebase reader; no record grammar or byte encoding may
change. `normalize_observation` reaches snapshot derivation and grounding
consumers; the replacement is the standard equivalent of
`map_or(true, is_safe_text)`.

All other warned-item impacts were LOW except two unresolved private helper
lookups (`validate_canonical_jcs_lf_v1` and `allocate_posture_transaction_id_v1`),
which GitNexus reported as UNKNOWN with no resolved callers. No result was
CRITICAL. `GroundedResolution` is public and used across the Flow crate, so the
correction must preserve the public `GroundingOutcome::Grounded(GroundedResolution)`
shape and its accessor signatures; only a private field representation may be
changed.

## Required follow-through

An independently fresh complete-subject code review must inspect every HIGH
symbol's delta, including whether the visibility adjustments remain confined to
private parent modules and whether the `GroundingOutcome` public surface is
unchanged. Any semantic, public, schema, dependency, configuration, or
non-admitted-path need is an authority stop. The exact strict Clippy gate and
the required engine/workspace tests remain mandatory; no lint suppression,
configuration change, disabled test, or waiver is permitted.
