# HCM-3.5 P4 transition materialization selector

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P4

**Status:** SELECTED AND IMPLEMENTED — repository-owned source replay

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** the direct user-authorized P1-P4 recovery grant,
preserving the continuation parent and P1 causal identity. This selector
supersedes only the noncompleted conclusion of
`20260807T173000Z--p4-descriptive-transition-refs-selector.md`; it does not
rewrite that immutable historical record.

## Selected bounded producer

P4 owns a tracked, repository-local source package at
`.handbook/grounding/hcm-3.5/v1`. It contains the complete ordered sequence:

```text
E0 prior end -> S1 session start -> Dg compatible grounding delta
-> G bounded grounding projection -> P4 work -> E2 session end
-> Ds compatible final delta
```

The existing HCM-3.4 snapshot builder and delta derivation remain unmodified.
The sole existing adapter edit is LOW-risk `derive_grounding_source_pair`: it
first retains the legacy empty-history route and then accepts the immediately
preceding E0 only when the source record requires it. The new private
`derive_grounding_transition` replay validates the full E0/S1/E2 chain and
both route descriptors. No Flow, pipeline, compiler, CLI, SDK, Substrate,
schema, template, validator, configuration, gate runtime, remote, or
protected-checkout change is selected.

The `.handbook` source files are force-added to the local index without a
`.gitignore` or configuration change. They are P4-owned deterministic
test/proof source records, distinct from the P1 `TempDir` proof fixture; no
external-live capture, promotion, or parent integration is claimed.

## Frozen exact descriptive refs

```text
prior_end_snapshot_ref
  handbook.hcm-3-5.transition-snapshot.prior-end@1.0.0
  #sha256:5a6d11b725b396fdcc01c4e675fadd46f65508b5cf83d0adafe85d78143666d4

session_start_snapshot_ref
  handbook.hcm-3-5.transition-snapshot.session-start@1.0.0
  #sha256:beac8e254d680875a8b40c4f8aab8c23f440d4ae806b3af0fcd9cf298b465f50

grounding_delta_ref
  handbook.grounding.delta.prior-end-to-session-start@1.0.0
  #sha256:839ba6ea4cc3728f02a4615ab1f2e38381d9f0f1448e919f96a05a85b4ab9438

grounding_projection_ref
  handbook.hcm-3-5.grounding-projection@1.0.0
  #sha256:fa8a0901be95c278cebcee276e7aeba92fd64d1f19afaeb766657d40cd49392b

session_end_snapshot_ref
  handbook.hcm-3-5.transition-snapshot.session-end@1.0.0
  #sha256:c1c20406831464451de0d6dd95bef69099132e15cd7448b54cedc86bb7e25179

session_delta_ref
  handbook.grounding.delta.session-start-to-session-end@1.0.0
  #sha256:049365356ec18ad1525b3621e9b0214e4a0adcdb4b2fc72993b7d548d4d8df27
```

`grounding-projection.json` binds only the exact snapshot/delta,
definition/disclosure identities, the bounded public summary count (2), typed
omission count (3), and non-promoting evidence availability. It contains no
raw snapshot, raw delta signal, copied payload, or promotion assertion.

## Handoff disposition

The final successor may set `snapshot_refs.capture_status` to `captured` and
place these six exact strings in the existing nullable fields only after the
independent closure review is CLEAN. It must describe them as this P4
repository-owned materialization and must not relabel the P1 fixture as a live
current transition.
