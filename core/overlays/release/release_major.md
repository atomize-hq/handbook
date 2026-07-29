# Overlay: Major Release

Use this overlay when a release represents a significant expansion of scope, a new surface area, or a meaningful shift
in posture (e.g., internal → external users, beta → GA).

## Effects on planning
- Prefer **more sprint slots** than you think you need (avoid cramming).
- Require at least one sprint lane dedicated to integration/hardening.
- Add explicit `research_discovery` tasks early if architectural unknowns exist.
- Strengthen release DoD: require quality gates, docs, and environment context completeness.

## Suggested defaults
- Sprint slots: 3–6
- Observability threshold: standard → high (if user-facing)
- Testing posture: add integration tests for boundary changes
