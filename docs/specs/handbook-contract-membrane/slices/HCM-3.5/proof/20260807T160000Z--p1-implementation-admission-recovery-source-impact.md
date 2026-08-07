# HCM-3.5 P1 implementation-admission recovery — source and impact record

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P1

**Status:** CLEAN

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** the direct user-authorized
`p1-definition-and-implementation-admission-recovery` grant bound to task
`019fdcd7-31ae-7881-8502-1b173d751c93`, host `local`, and nonce
`f5fb20977fd6dad67ece2a8fd47f685de067b8cbf25cee9edd9c772e8a242d2e`.

## Checkout and scope

- The assigned source checkout was verified at
  `925262b062cb6e7c3898db74f20d84772e46af1e`.
- The independently checked final integration ref remained at
  `7e0836a1c60f3992a80719b597f3fc18fabe807e`; it is not a P1 write target.
- The required ancestor is `1a10bfd59e8433fa344dd2baa4880b4b87c7f8f6`.
- The protected checkout and every remote remained untouched.

P1 changes only the admitted engine module, narrow private Snapshot Memory
adapter, P1-owned test/fixture material, and P1 decision/proof records. It
does not alter Flow, pipeline, CLI, compiler, SDK, Substrate, Cargo,
dependencies, schemas, configuration, gate runtime, or the HCM-3.4 fixture.

## Local GitNexus discovery evidence

The mandated local-only refresh completed with exactly:

```text
npx --no-install gitnexus analyze
```

It rebuilt the cached local index (21,669 nodes, 46,452 edges, 474 clusters,
and 300 flows) without install, fetch, or remote access. The refresh updated
only generated counts in `AGENTS.md` and `CLAUDE.md`; those files remain
unstaged, uncommitted, undiscarded, and outside this P1 subject.

The exact upstream impact review established:

| Existing symbol | Direct callers | Processes | Risk | P1 disposition |
| --- | ---: | ---: | --- | --- |
| `ContextMemorySnapshot` | 1 | 1 | LOW | Read only; not changed. |
| `SnapshotDelta` | 1 | 1 | LOW | Read only; not changed. |
| `build_snapshot` | 6 | 1 | MEDIUM | Read through the private adapter; not changed. |
| `derive_snapshot_delta` | 3 | 1 | LOW | Read through the private adapter; not changed. |
| `projection_authority_view` | 0 | 0 | LOW | Read through its existing crate-private interface; not changed. |
| `execute_projection_with_live_observer` | 4 | 4 | HIGH | Not edited and not used. |

The HIGH seam reaches
`snapshot_source_pair_requires_exact_dependency_state_and_five_family_currentness`,
`stale_envelope_stack_custom_kind_policy_and_evaluator_dependencies_refuse`,
`currentness_none_and_exact_captured_revision_closures_refuse_substitution`,
and
`exact_currentness_requires_snapshot_selection_and_independent_live_observation`.
P1 leaves that seam intact instead of retrofitting it as a shortcut.

## Discovery findings and consolidated remediation

The initial complete-subject review found the same three concrete gaps that
the recovery grant assigned to P1: no immutable summary/disclosure definition,
no independently provable persisted current snapshot-to-compatible-delta route,
and no typed bounded disclosure partition.

One consolidated P1 remediation adds:

- `grounding` as the engine-owned public module, with opaque exact-reference
  constructors, opaque Flow/Pipeline values, typed refusal shapes, typed
  omissions, provenance, and non-promoting evidence;
- the frozen definition/disclosure artifacts selected in the paired decision,
  bound to `sha256:e553ba87b1182df8d4bc587259bcbe227e208b2e1ef670a8b6dbc35f4ed75e1d`
  and `sha256:ec46c655d094a69f7d46aa1788f337d2feb0acb72398526f4570d470d0e66948`;
- maximum-two, kind-and-signal-ID ordered summary selection with ineligible,
  redacted, and overflow typed omissions, and no raw snapshot/delta payload
  disclosure;
- a narrow crate-private adapter that validates policy/captures/records,
  derives the compatible delta from exact persisted endpoints, and verifies its
  route fingerprints before exposing only private normalized values; and
- a P1-owned temporary-repository source fixture that proves the route rather
  than changing the HCM-3.4 fixture.

## Closure inputs

The independent closure subject is the complete P1 allowlist:

```text
crates/engine/src/lib.rs
crates/engine/src/grounding.rs
crates/engine/src/snapshot_memory/mod.rs
crates/engine/tests/hcm_3_5_grounding.rs
crates/engine/tests/fixtures/hcm_3_5_grounding/**
docs/specs/handbook-contract-membrane/slices/HCM-3.5/decision/20260807T153500Z--p1-implementation-admission-recovery-selector.md
docs/specs/handbook-contract-membrane/slices/HCM-3.5/proof/20260807T160000Z--p1-implementation-admission-recovery-*.md
```

The separate, generated `AGENTS.md` and `CLAUDE.md` count refreshes are
explicitly excluded. The paired proof matrix records the test and formatter
evidence. `npx --no-install gitnexus detect-changes --scope staged` was run
against this checkout before closeout; its local FTS-disabled output reported
`No changes detected`, so the command supplied no per-symbol delta rows. The
staged Git path manifest and `git diff --cached --check` independently confirm
the allowlist above and no whitespace defect. P1 is therefore CLEAN; no P2 work
has begun.
