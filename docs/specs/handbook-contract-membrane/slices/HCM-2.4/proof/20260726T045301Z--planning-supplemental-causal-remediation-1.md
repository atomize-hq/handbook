# HCM-2.4 supplemental causal remediation 1

Captured: `2026-07-26T04:53:01Z`

Closure run: `/root/hcm24_planning_closure_review`

Reviewed fingerprint:
`sha256:15e618577d9b784de4b6ff23e6170c9a8b6ceff0a7ebef06aa4de3754ad38d59`

Verdict: FINDINGS, one P2 directly unmasked by the preceding remediation, no
P1/P3/P4.

## Causal finding

The discovery remediation froze an exact Work Specification descriptor with a
non-empty renderer ref. It updated the planned `artifact_instance.rs` admission
guard only for Project Context and Environment Context. The repository-reachable
existing guard rejects every non-Charter descriptor with a renderer ref, so the
new Work Specification fixture would fail before Stage 10 real-path proof.

This is directly caused/unmasked by the descriptor remediation and therefore
qualifies for supplemental causal cycle 1. It is not unrelated discovery.

## Bounded remediation

P1A now admits exactly three frozen non-Charter descriptor rows:

- Project Context;
- Environment Context; and
- the repository-profile Work Specification instance.

For Work Specification, the exact admitted closure includes its kind, intake,
singleton renderer, `always` requiredness, role/path, and empty lifecycle,
Projection, overlay, condition, and extension fields. Inline/unit admission
proof precedes P3's real repository-path exercise. Every missing, extra,
mismatched, or unrelated later-owned dependency remains refused.

The correction does not add Work Specification to the shipped-root profile,
create a command, infer a path, change a public API, or widen into generic
configured renderers/Projections. It modifies only planning documents and
retains the selected scope, authority, and risk ceiling.

Supplemental causal cycle 1 is consumed. One supplemental causal cycle remains
available only for a valid P1/P2 directly caused or unmasked by this
remediation.
