# HCM-3.5 P3 namespaced shared-grounding inclusion — source and impact record

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P3

**Status:** CLEAN

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** the direct user-authorized P1-P4 recovery grant,
preserving the same continuation parent and P1 causal identity.

## Discovery and HIGH-risk boundary

The local-only GitNexus query attempted first could not produce keyword results
because the local FTS feature is absent. Exact symbol impact was then run:

| Symbol | Direct callers | Affected processes | Risk | P3 disposition |
| --- | ---: | ---: | --- | --- |
| `compile_pipeline_stage` | 1 | 1 | LOW | Not edited or called. |
| `compile_pipeline_stage_with_runtime` | 2 | 3 | HIGH | Not edited, called, wrapped, or retrofitted. |
| `filter_scoped_blocks` | existing compile callers | read-only context | Existing work-level seam | Not edited or called. |

The HIGH seam reaches CLI execution, capture application, and pipeline-handoff
emission. That warning is resolved by isolation rather than a modification: P3
is a new module with no dependency on any compile or filter path.

## One remediation

`include_grounded_shared_resolution` accepts only an engine-created
`SharedResolutionInclusion` and returns `GroundedSharedResolutionInclusion`
under the static namespace `handbook.hcm-3-5.shared-resolution`. The namespace
is not caller-controlled and the opaque inclusion is retained unchanged.

The new source contains no `work_level`, L0-L3, document selector, repository
read, snapshot/delta/signal read, JSON, filesystem, or legacy compile fallback.
It makes no inclusion decision from raw level data. A typed-engine refusal
cannot reach P3 because P3 has no input other than the successful opaque engine
inclusion.

## Closure subject

```text
crates/pipeline/src/lib.rs
crates/pipeline/src/shared_grounding_inclusion.rs
crates/pipeline/tests/hcm_3_5_shared_grounding_inclusion.rs
docs/specs/handbook-contract-membrane/slices/HCM-3.5/decision/20260807T170000Z--p3-namespaced-shared-grounding-inclusion-selector.md
docs/specs/handbook-contract-membrane/slices/HCM-3.5/proof/20260807T171000Z--p3-namespaced-shared-grounding-inclusion-*.md
```

The generated `AGENTS.md` and `CLAUDE.md` count refreshes remain unstaged,
uncommitted, undiscarded, and excluded. The staged local GitNexus detection
reported six files, one container symbol (`pipeline_contract_version`), zero
affected processes, and LOW risk; exact upstream impact for that untouched
function is also LOW with zero callers/processes. P3 is CLEAN. P4 has not
begun.
