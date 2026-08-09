# HCM-3.6 strict-Clippy partial-corrective planning discovery remediation

**Parent:** `handbook-hcm-3-6-strict-clippy-partial-corrective-planning-20260809`

**Discovery dispatch:**
[`20260809T231600Z--HCM-3-6--strict-clippy-partial-corrective-planning-discovery-review`](../../handoffs/dispatches/20260809T231600Z--HCM-3-6--strict-clippy-partial-corrective-planning-discovery-review.json)

## Independent result

Fresh built-in reviewer `/root/partial_clippy_planning_discovery` completed the
schema-validated discovery run with `verdict: findings`. It reported one valid
P2: the selector promised a 20-only residual after nine fixes but omitted
`clippy::large_enum_variant` at `crates/engine/src/grounding.rs:109` from its
partial partition. The all-target strict-gate replay has 30 unique
location/code pairs: the 20 documented private-reachability P2 diagnostics,
nine other previously identified mechanical diagnostics, and this additional
large-variant diagnostic. The discovery result found no Critical, P3, or P4
finding.

The finding does not weaken or replace `HCM3EXIT-P2-CLIPPY-001`. It identifies
a planning error that would have made a later partial proof claim false.

## Parent validation and bounded remediation

The parent independently confirmed the raw Clippy output: `GroundingOutcome`
at lines 109-112 contains `Grounded(GroundedResolution)` and Clippy reports
the large-variant diagnostic. The existing HCM-3.6 strict-Clippy impact record
already records the only compatible route: preserve the public enum payload
shape and accessor signatures while changing only private
`GroundedResolution` representation.

The planning selector was corrected by a new addendum, preserving the
discovery selector/preflight bytes, to:

1. distinguish the 29 product-code diagnostics from the one non-duplicate
   all-target test diagnostic;
2. define the later partial subject as 10 diagnostic pairs in six paths;
3. admit only one private boxed backing representation for
   `GroundedResolution`, with unchanged public type name, variant payload,
   accessors, values, refusal behavior, and canonical-byte semantics; and
4. require residual proof of exactly the same 20 P2 diagnostics, rejecting any
   unaccounted lint pair.

No Rust, test, Cargo, lint-policy, configuration, branch/ref, remote,
protected-checkout, immutable predecessor, or ledger change was made. This is
one consolidated planning remediation. The closure reviewer must be different
and fresh, inspect the selector addendum plus this remediation record, and
confirm the P2 remains an open authority stop.
