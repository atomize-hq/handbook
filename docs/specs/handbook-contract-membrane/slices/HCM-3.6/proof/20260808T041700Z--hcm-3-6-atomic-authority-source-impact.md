# HCM-3.6 atomic-authority source and impact proof

## Result

This increment changes documentation only. No Rust symbol is edited. It uses a
fresh local GitNexus index of the exact assigned base to identify every
existing symbol in the minimum future implementation ceiling and to bind the
required HIGH/CRITICAL acceptance posture. FTS/BM25 is unavailable under the
offline load-only policy; concept search returned no results and is recorded
as unavailable, never GREEN. Exact symbol `context` and upstream `impact`
remain available and were used.

The local index was built with `npx --no-install gitnexus analyze --index-only`
and reported 22,139 nodes, 47,181 edges, 475 clusters, and 300 flows. Index-only
mode injected no AGENTS, CLAUDE, or skill-file changes.

## Physical source evidence

`CanonicalCharter` stores `posture.baseline_level` and a fixed ordered nine-item
`engineering_posture.dimensions` array. Each
`CanonicalCharterDimension` stores `dimension_id` and
`level_override: Option<u8>`. Current rendering resolves the effective level as
`level_override.unwrap_or(baseline_level)`. The canonical schema and semantic
validator require the exact ordered dimension IDs. Therefore the prior
pseudo-path `/engineering_posture/dimensions/testing_rigor/level` is not a
physical Charter path; the real testing-rigor leaf is
`/engineering_posture/dimensions/2/level_override`.

The promotion transaction currently writes canonical bytes plus promotion and
lifecycle-transition records, then validates a promotion-only chain. Its
current read rejects any canonical head not equal to the final promotion
journal output. The lifecycle store obtains its anchor through
`current_promotion_anchor` and validates lifecycle state against that exact
canonical fingerprint. These facts prove that a posture-only side journal or
unrecorded Charter rewrite would split current authority and is inadmissible.

## Exact upstream impact

| Existing future-edit symbol | Direct / total impact | Processes / modules | Risk and required acceptance |
| --- | ---: | ---: | --- |
| `CharterAuthorityTransactionServiceV1::read_committed_charter` | 5 / 7 | 0 / 2 | **MEDIUM**. Preserve public result shape and Flow-compatible most-recent-promotion ancestry while returning current bytes/fingerprint/lifecycle head. |
| `read_committed_charter_locked` | 0 / 0 | 0 / 0 | LOW. Equality with the public locked path and retained-authority callers is mandatory. |
| `recover_pending_locked` | 3 / 38 | 2 / 5 | **CRITICAL**. Affects approval `approve_inner` and lifecycle-validation `finalize`; require exhaustive promotion+posture recovery, approval, and validation replay. |
| `validate_terminal_history` | 4 / 25 | 2 / 4 | **HIGH**. Replace promotion-only head selection with one merged edge chain without weakening promotion-terminal validation. |
| `validate_terminal_inventory` | 2 / 26 | 2 / 4 | **HIGH**. Continue exact promotion journal/final-record validation, then contribute edges to the merged chain; forks/unreachable history still refuse. |
| `current_committed_intent_locked` | 0 / 0 | 0 / 0 | LOW. Retire promotion-head semantics from lifecycle use; retain a narrowly named promotion lookup only where exact promotion history is required. |
| `CharterLifecycleStoreV1::load_current_locked` | 4 / 19 | 3 / 6 | **CRITICAL**. Affects lifecycle event, retained observation, and recovery paths; recover canonical journals before selecting bytes. |
| `load_canonical_bytes_locked` | 2 / 17 | 3 / 6 | **CRITICAL**. Must accept v1.0 promotion anchors and private v1.1 posture anchors while deriving one exact state. |
| `current_promotion_anchor` | 1 / 13 | 3 / 6 | **CRITICAL**. Replace with heterogeneous current-authority anchor; never synthesize a promotion for a posture head. |
| `retain_loaded_authority` | 2 / 2 | 1 / 1 | LOW. Retain exact v1.0 or v1.1 current lifecycle bytes without public record-class expansion. |
| `validate_transition_shape` | 1 / 13 | 3 / 6 | **CRITICAL if edited**. The selected design leaves the v1.0 validator unchanged and adds a separate private v1.1 validator; any implementation edit to this symbol requires the same CRITICAL wall. |

GitNexus context identifies `record_event_inner`, `observe_retained_locked`,
`recover_one_pending`, approval, and lifecycle-validation finalization as the
principal compatibility processes. HIGH/CRITICAL work is not accepted by a
focused unit test alone. It requires all named process regressions, the full
engine/workspace wall, exact staged change detection, and fresh independent
review.

## Read-only dependencies

The later implementation may call but must not edit:

- `parse_canonical_charter` and `serialize_canonical_charter` for typed exact
  bytes and deterministic whole-document replay;
- `resolve_profile_selection` for the exact selected-profile fingerprint; its
  prior P0 CRITICAL result remains a read-only warning;
- `charter_lifecycle_state_fingerprint` for exact result-state identity;
- `CharterPromotionWorkflowServiceV1::promote` as the unchanged later-promotion
  compatibility consumer; and
- HCM-3.4/HCM-3.5 exact redacted evidence boundaries.

Any discovered need to edit one of those dependencies is outside the minimum
allowlist and requires a new reviewed authority decision plus fresh impact.

## Future exact path ceiling

The only prospective source paths are those enumerated in the atomic-authority
selector. New files have no pre-existing symbol impact. `lib.rs` may receive
private `mod` declarations only; no `pub mod`, `pub use`, or existing public
type change is permitted. Crate-local `#[cfg(test)]` child modules are the only
selected new test seam. Existing integration, Flow, approval, promotion,
lifecycle, and workspace tests are regression commands, not edit targets.

The compare-to-main graph is expected to reflect large historical repository
divergence and cannot be used as a focused HCM-3.6 green claim. The future
implementation must record it honestly alongside exact scoped/staged detection.

