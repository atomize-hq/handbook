# HCM-3.6 strict-Clippy partial-corrective planning selector remediation

**Status:** active correction to the planning selector. This is
documentation/control authority only; it does not admit a source edit, resolve
strict Clippy, or close Phase 3.

## Exact supersession boundary

This document corrects only the diagnostic-count and `grounding.rs` candidate
details in
[`20260809T231500Z--hcm-3-6-strict-clippy-partial-corrective-planning-selector.md`](20260809T231500Z--hcm-3-6-strict-clippy-partial-corrective-planning-selector.md).
The original selector and preflight remain byte-immutable discovery evidence.
All of their unchanged prohibitions, proof gates, P2 ownership, Phase-3-open
status, Phase-4 responsibility, HCM-3.5 Packet 5/6 deferral, local-only
boundary, and true-stop conditions continue to apply.

The fresh discovery reviewer found that all-target Clippy contains 30 unique
location/code pairs: the 29 product-code diagnostics counted in the consumed
authority stop plus the test-target `unnecessary_sort_by` diagnostic. A later
partial candidate must remove all 10 pairs below and then prove that exactly
the unchanged 20 private-unreachable diagnostics remain. No unaccounted lint
diagnostic may be omitted from that before/after proof.

## Corrected six-path candidate set

| Path | Exact later-only change | Invariant |
| --- | --- | --- |
| `crates/engine/src/charter_authority_transaction.rs` | Elide the explicit lifetime in `posture_output_for` only. | Same borrowing, error, posture, and visibility behavior. |
| `crates/engine/src/snapshot_memory/policy.rs` | Change `normalize_windows` only from `&mut Vec<AuthoredStaticWindow>` to `&mut [AuthoredStaticWindow]`. | Same caller-owned collection, ordering, normalization, and bytes. |
| `crates/engine/src/snapshot_memory/record.rs` | Replace the three specified `map_or(true, is_safe_text)` forms only with `is_none_or(is_safe_text)`. | Same `None` acceptance, predicate, and refusal behavior. |
| `crates/engine/src/snapshot_memory/redaction.rs` | Pass `valid_pointer_segment` directly to `Iterator::all`. | Same pointer validation and redaction/refusal behavior. |
| `crates/engine/src/grounding.rs` | Add private aliases for the two current complex signatures **and** move only the private fields of `GroundedResolution` behind one private boxed backing representation, forwarding the current accessors. | `GroundingOutcome::Grounded(GroundedResolution)`, public `GroundedResolution` name, public accessor signatures and values, refusal behavior, bytes, and external construction boundary are unchanged. No public variant payload, export, caller, reachability, semantic route, or API change. |
| `crates/engine/src/snapshot_memory/tests.rs` | Replace the pure canonical-string sort comparator only with `sort_by_key(canonical_json)`. | Same input ordering and test assertion behavior; no fixture, expectation, or coverage change. |

This is 10 diagnostic pairs: lifetime (1), pointer argument (1), Option
predicate (3), redundant closure (1), `GroundingOutcome` large variant (1),
type complexity (2), and test sort idiom (1). The private backing
representation is the sole permitted route for `large_enum_variant`; boxing
the public enum payload itself is not authorized.

## Retained authority stop

`HCM3EXIT-P2-CLIPPY-001` remains an open P2 with the same owner and resumption
condition recorded in the immutable 15:20 and 17:30 handoffs. This corrected
partial package may reduce the diagnostic count only. It cannot make the
remaining private posture surface reachable, hide it with `#[cfg(test)]`,
suppress it, baseline it, list it as accepted debt, change `lib.rs` or public
visibility, add a synthetic production caller, wire legacy CLI/compiler or
SDK/transport behavior, misuse candidate approval/promotion/lifecycle/recovery,
or start Phase 4. Any need for those routes is a separate authority stop.

Before a future source edit, a new implementation parent must run fresh
GitNexus impact/context for every changed existing symbol and record FTS as
unavailable if it remains unavailable. It must use a fresh code review cadence
and close only at the same P2 authority boundary; neither a partial source
commit nor this selector amendment is a strict-Clippy or Phase-3 exit claim.
