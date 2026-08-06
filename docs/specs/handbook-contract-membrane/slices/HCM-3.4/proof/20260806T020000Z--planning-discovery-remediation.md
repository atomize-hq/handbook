# HCM-3.4 planning discovery remediation

**Timestamp:** 2026-08-06T02:00:00Z
**Outcome:** hcm-3.4-snapshot-memory-planning
**Review packet:** HCM-3.4-P1-snapshot-memory-planning-causal-review
**Discovery reviewer:** /root/hcm34_planning_discovery (fresh built-in subagent)
**Review result:** findings — no P1; two valid P2 findings; no P3/P4 advisories.

## Valid findings and consolidated in-scope repair

| Finding | Material planning gap | Consolidated repair | Result |
|---|---|---|---|
| HCM34-PLAN-DISC-001 (P2) | PG-SNAP-04 deferred generic Projection proof to HCM-3.5 even though HCM-3.4 must establish its own Snapshot Projection capability boundary. | SPEC.md and tasks/plan.md now require a private HCM-3.4 integration proof through the existing generic Projection: exact snapshot/delta sources, Resolution minima, typed disclosures/omissions, all-family accounting, captured-revision currentness, original/retained-pointer source identity, hidden-data and stale/insufficient-input refusal, and authority_effect: none. The plan expressly prohibits Handoff, packet, pipeline, or other HCM-3.5 consumer adoption. | repaired within planning authority |
| HCM34-PLAN-DISC-002 (P2) | The redaction proof matrix covered known unmatched and matcher failure but not an unknown or unclassifiable capture surface. | PG-SNAP-05 and packet 5 now require deterministic unknown/unclassifiable-surface omission before persistence and prove that it cannot downgrade into a non-omit action or retained data. | repaired within planning authority |

## Boundary check

The repair changes planning documents only. It selects no implementation symbol, source path, schema, public API, dependency, workflow, runtime configuration, product test, HCM-3.5 adoption, or remote action. No Snapshot Memory behavior has been implemented or validated.

## Local documentary checks

- UTF-8 decoding, trailing-whitespace scan, Markdown fence parity, and `git diff --check` are required before closure review and primary commit.
- The discovery dispatch remains immutable. The different-fresh closure dispatch will bind the repaired complete subject and these two finding identifiers.
- GitNexus impact analysis is not applicable because no function, class, or method is edited. GitNexus change detection remains unavailable in this environment and must be recorded as unavailable, never green; selected-path Git review is still required before commit.

## Next causal action

Run exactly one different-fresh, delta-focused closure review of HCM34-PLAN-DISC-001 and HCM34-PLAN-DISC-002. It may report only P1/P2 issues directly caused or unmasked by this repair; it must not reopen general discovery.
