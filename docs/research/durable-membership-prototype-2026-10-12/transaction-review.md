# Independent transaction follow-up review

The controlling Round 2 verdict is **approve the bounded experiment for human
review**. No unresolved P1 Critical or P2 Required finding remains. This review
concerns the isolated transaction follow-up to `c7f54e63`, including both adapters,
shared publication/semantic behavior, final native evidence and comparison claims.
It does not approve a production backend, migration, format default or deployment.

## Findings and resolution

The selected predecessor P2 admitted-domain overflow and P3 wrong-shard audit
blind spot are reproducible on both old native binaries. The shared bounded
domain now accommodates maximal admitted escaped/high-byte values, and audit
validates the enumerated object's canonical shard/path before reading it.
Final native suites exercise both repairs, with explicit UTF-8 child transport.

Round 1 found a new P2 semantic replay bypass: generic `read_only:true` leaked
into `run`, allowing an exact terminal replay to return without coordination or
publication repair. Both retained candidate binaries reproduce that behavior;
ordinary replay provides a positive injected-checkpoint control. The CLI now
applies that modifier only to generic `tx`, and read-only API commits reject
replay repair. Final suites require flagged semantic replay to acquire the lock
and reach the durable boundary even after revocation. Injected `replay_sync`
failure returns indeterminate without acknowledgement. That finding is closed.

Review also checked per-attempt typed failure classification: publication state
is reset before an attempt, so earlier success cannot make a later known
prepublication failure indeterminate. It examined real preparation before the
writer lock, final coalesced disk writes, exact root/dependency validation,
explicit conflict/revalidation, retained snapshots, cancellation/drop and
native owner/lock checks. No additional P1/P2 defect was demonstrated.

The final P4 nit concerned documentation: semantic cohort wall timing excludes
the later verification command. The comparison now states that boundary, and
the reviewer confirmed closure. No unresolved review nit remains from this round.

## Evidence checked

The reviewer independently checked final source hashes and both native
correctness/publication/race suites, including the reconstructed superseded
candidate's six exact source hashes. It recomputed all 716 combined growth
groups and all 192 paired groups from raw records with zero discrepancies.
Both cohort families have valid PID-linked current RSS samples, common live
windows, conflict/retry responses and retained bindings. Storage totals,
preparation/open/lock timer labels, median/p95 estimator differences, retry
totals and cumulative child lock maxima agree with raw evidence.

The first candidate's Windows transport failures and all superseded versions
remain visible in [transaction-verification.md](transaction-verification.md).
Assertions, descriptive observations and independent timing trials are kept
distinct. The controlling verdict uses only the final frozen subject; passing
old suites are not substituted for final evidence.

GitNexus's staged check reports MEDIUM risk and three experimental execution
flows (`Execute → Valid_id`, `Execute → Plain`, `Execute → Open`). The index was
refreshed during implementation but does not completely map the newly added
transaction/batch symbols. Earlier bound impacts on indexed common functions
were LOW; missing or ambiguous symbols were explicitly treated as unknown and
checked through isolated caller/source review. These are partial graph results,
not a claim of complete automated coverage.

The required comparison against `main` reports CRITICAL risk over 1,694 files,
28,293 symbols and 296 flows. That range includes existing branch work outside
this unit. The experiment's staged path check excludes that other work, and its
independent Cargo workspace imports no production crate or new production
dependency. Both raw graph reports are retained under
`evidence/tx-final-verification-20261012a/`; they do not turn the broader branch
range into an approved change in this experiment.

## Limits retained for human review

Native Windows NTFS and Linux ext4/WSL2 were executed. Native macOS and hardware
power-loss qualification remain unexecuted; [platforms.md](platforms.md) records
the concrete remaining macOS work. There is no reclamation, resident owner or
decoded disk-page cache. Whole-root validation is deliberately conservative,
and cooperative locking/content checks do not prevent coherent local rollback.
Counters report application file work rather than physical I/O or allocations.
The result is ready for the human review boundary established for this experiment.
