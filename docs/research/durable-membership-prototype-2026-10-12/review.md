# Independent causal review

This records the predecessor's review at `c7f54e63`. Follow-up transaction code
and evidence have a separate causal review in
[transaction-comparison.md](transaction-comparison.md); this verdict is not reused
as approval of that changed subject.

One independent read-only reviewer inspected the bounded experiment, native raw
evidence, source/reuse accounting and causal comparison. Initial verdict was
BLOCKED on two P2 publication sequencing defects; no other P1/P2 finding was made.

1. Existing checksum-object reuse could skip fsync of its containing shard after
   a previous interrupted installation. Final `Objects::write` registers that
   shard before either the existing-object or newly installed branch. A new
   `object_install` checkpoint exercises the interruption gap.
2. Exact terminal replay after interrupted root replacement could acknowledge
   without completing the selected-root file/store publication boundary. Final
   replay invokes the same `sync_publication` helper as publication while holding
   the retained writer lock, then reaches `replay_sync` before reply.

The same reviewer conducted a fresh delta review. Source inspection confirmed both
causes were removed; focused raw Windows/Linux repair evidence confirms actual
orphan installation/reuse and reached replay repair before acknowledgement.
There was no new P1/P2 finding in that delta. Final verdict: **READY FOR HUMAN
REVIEW**, with both P2 findings resolved and no remaining P1/P2 finding.

The reviewer independently verified all 12,068 final full/focused passing
assertions, zero failure/skip, all 25,950 retained passes and 132 observations,
all 1,808 regenerated summary groups (including metric/amortized-core statistics),
the refreshed report tables, final Rust/native binary hashes and all 590 unchanged
input hashes. Original overlap measurements keep their explicit source/version
limits. The verdict covers this bounded experiment and report only.

The reviewer also reproduced all 924 original summary groups, 13,650 passing
assertions and 84 observations, with zero failure/skip; all 590 bounded input
hashes; positive lookup counters, timings and p95; overlapping RSS, retries and
lock holds; and upstream package/revision/source/license hashes. The source
snapshot/measurement versions are explicitly retained, including the original
common layer and Windows harness.

P3 advisory about later gzip-reader support in `summarize.py` was addressed with
provenance in `verification.md`. One remaining P3 test-strength advisory: focused
`name_boundaries >= 2` proves boundary work, while source inspection establishes
that the particular reused object's shard is included. It does not independently
trace that shard's native fsync. Adding a tracing layer is outside this bounded
unit; the limitation is explicit and does not qualify hardware persistence.

Production selection, migration, native macOS, hardware power cuts, reclamation,
arbitrary external effects and full Handbook native guard/grant proof closure
remain outside the review verdict.
