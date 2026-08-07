# HCM-3.5 P3 namespaced shared-grounding inclusion selector

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P3

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** the direct user-authorized P1-P4 recovery grant,
preserving the same continuation parent and P1 causal identity.

## Selected path

`handbook_pipeline::include_grounded_shared_resolution(SharedResolutionInclusion)
-> GroundedSharedResolutionInclusion` is the P3 path. It accepts exactly one
opaque engine-created shared-inclusion value and returns it under the fixed,
deterministic `handbook.hcm-3-5.shared-resolution` namespace.

There is no string selector, repository input, stage ID, scoped document,
`work_level`, L0-L3 level, raw snapshot/delta/signal, fallback, or refusal
recovery branch. The engine value is already exact/current/refused before P3
can be called. P3 only names its namespace and retains the opaque value for a
later selected consumer; it cannot recreate engine semantics.

## HIGH-risk regression boundary

Local GitNexus impact marked
`compile_pipeline_stage_with_runtime` **HIGH**: 8 upstream impacts across the
CLI, capture, and handoff paths. `compile_pipeline_stage` has two LOW-risk
upstream impacts, while `filter_scoped_blocks` is an existing work-level
implementation seam. P3 does not edit, call, wrap, or retrofit any of them.

The P3 allowlist is only the new pipeline module, pipeline crate-root
module/export list, focused type/namespace test, and its selector/proof
records. It makes no Cargo/dependency, engine, Flow, CLI, compiler, SDK,
Substrate, schema/configuration, handoff-schema, gate-runtime, protected
checkout, or remote change.
