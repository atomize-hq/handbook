# HCM-3.6 P0 source implementation selector impact

## Admission snapshot

| Check | Observation | Result |
| --- | --- | --- |
| Task HEAD/tree | \`c38042434a66b083d57fedc5e99017a86d7ba7c6\` / \`4a8a91d666019a704d7cc1cc039e60dd3ed31fa0\` | passed |
| Local integration ref | \`refs/heads/orchestration/handbook-hcm-3-6-implementation-20260808\` at the same commit | passed |
| Required ancestor | \`1256e724a2b7da6b6250f57d6f63fced1e2cf949\` is an ancestor | passed |
| Protected checkout | clean at \`1256e724a2b7da6b6250f57d6f63fced1e2cf949\` / tree \`ad5bac4904912e9a52b23ac862414deb79613154\` | passed |
| Remote tracking observation | local \`origin/feat/handbook-contract-membrane\` remains \`1256e724a2b7da6b6250f57d6f63fced1e2cf949\`; no network command ran | passed |
| Ordinary validator | \`handoff validation failed: 20260806T202700Z--HCM-3-5--resolution-aware-adoption-planning: continuation writes or advances before selector CLEAN\` | non-passing inherited exception only |

\`npx --no-install gitnexus analyze\` rebuilt the local graph at the exact base:
22,173 nodes, 47,216 edges, 475 clusters, and 300 processes. Full-text/BM25 is
unavailable under the offline load-only policy. Concept query returned the
explicit unavailable warning and is not GREEN. The analyzer changed only the
generated symbol/edge count lines in \`AGENTS.md\` and \`CLAUDE.md\`; those exact
hunks were restored to expected-base bytes and remain outside every allowlist.

## Exact upstream impact

| Selected existing function | Direct / total | Processes | Risk and required acceptance |
| --- | ---: | --- | --- |
| \`read_committed_charter\` | 4 / 6 | no named process | LOW; preserve existing public shape and promotion ancestor projection |
| \`read_committed_charter_locked\` | 0 / 0 | no named process | LOW; preserve the lock-local terminal projection for the unchanged public read shape |
| authority \`recover_pending_locked\` | 3 / 21 | \`approve_inner\`, \`finalize\` | HIGH; promotion and posture recovery share current-head selection |
| \`validate_terminal_history\` | 4 / 18 | \`approve_inner\`, \`finalize\` | HIGH; preserve promotion validation while merging posture edges |
| \`validate_terminal_inventory\` | 2 / 17 | \`approve_inner\`, \`finalize\` | HIGH; retain exact inventory validation and no-fork/no-cycle proof |
| \`current_committed_intent_locked\` | 0 / 0 | none | LOW; retain promotion-only lookup only where history needs it |
| lifecycle \`recover_pending_locked\` | 4 / 10 | \`record_event_inner\` | HIGH; recovery cannot choose a separate Charter |
| lifecycle \`recover_one_pending\` | 1 / 8 | \`record_event_inner\` | HIGH; recovery cannot choose a separate Charter |
| \`load_current_locked\` | 4 / 13 | event, observation, recovery | CRITICAL; all lifecycle current reads use the merged head |
| \`load_canonical_bytes_locked\` | 2 / 10 | event, observation, recovery | CRITICAL; accept private 1.1 and retained 1.0 authority |
| \`current_promotion_anchor\` | 1 / 7 | event, observation, recovery | HIGH; remove promotion-only selection without synthesis |
| \`retain_loaded_authority\` | 2 / 2 | \`observe_retained_locked\` | LOW; retain exact 1.0 or 1.1 bytes privately |
| \`record_event_inner\` | 2 / 2 | none | LOW; events chain from the 1.1 posture head |
| \`observe_retained_locked\` | 0 / 0 | none | LOW; retained callers observe the same terminal Charter |

The prior planning record classified recovery/lifecycle compatibility as
CRITICAL using its wider process inventory. This selector retains that stricter
acceptance posture whenever an edit reaches either recovery path, even where
the fresh call graph reports HIGH. These HIGH/CRITICAL risks were surfaced
before code modification; focused tests alone cannot admit them.

## Read-only dependencies and required comparison

\`parse_canonical_charter\`, \`serialize_canonical_charter\`,
\`resolve_profile_selection\`, \`compute_freshness\`,
\`charter_lifecycle_state_fingerprint\`, and
\`CharterPromotionWorkflowServiceV1::promote\` are read-only dependencies.
\`validate_transition_shape\` is also read-only: lifecycle 1.1 uses a new private
validator. Any required edit to one of these dependencies, a public surface,
Cargo/configuration, an external test, or a consumer is outside admission.

Before the reviewed primary commit, record staged change detection and a
byte-hashed compare artifact over the final code-bearing delta. The required
comparison to \`main\` is expected to contain historical divergence and must be
marked as such, never as an HCM-3.6 green signal.
