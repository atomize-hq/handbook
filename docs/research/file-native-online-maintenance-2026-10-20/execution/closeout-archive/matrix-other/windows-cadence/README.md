# QPC Windows sampler: source-only review candidate

Parent SOURCE ONLY GO registered this new directory at <=65,536 bytes /8 files,
inside existing Matrix F01 source1MiB. Admission budget-final-source-v2.json SHA256
0b26efdba8af4ab41a939697244e02b1e7da1ec55ab857d05bd6d74de89ca781; parent claim017
SHA256990a696b96b7dfbd589a5fd6d134cc900e65c1c61b1a614d10556019a803050d.
No new global allowance. No fixtures, native Jobs, controls, engine, builds,
Git operations, indexing, installs, system settings or process priorities changed.
All live controls require a separate parent review, allocation and GO.

The exact Python3.11.9 executable reports monotonic=GetTickCount64(), resolution
0.015625s; perf_counter=QueryPerformanceCounter(), resolution1e-7s. Clock metadata
is observed locally, not a precision/actual-cadence guarantee. Historical raw
47/63ms gaps and qualification are unchanged. Quantization is a possible cause,
not proof that all historical gaps were artifacts.

job_guard_qpc.py is a copy of frozen harness/job_guard.py (SHA2561e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae).
Only run changes: sampler state/clock metadata, serialized raw timing writes,
memory_watch, first-full-sample readiness/freshness before resume, root-end
coverage evidence/censor and final provenance fields. Budget, rss, all other
functions/classes, signatures and imports are AST-identical. The diff is complete.
Existing callers continue using the frozen original. Future callers must explicitly
bind the new executed controller SHA; this source-only worker changes no callers.

The sampler schedules on perf_counter_ns at20ms, records QPC starts/completions
and paired coarse monotonic values, and preserves interval_seconds as QPC seconds
with an explicit clock label. Start gaps over40ms produce an explicit NON-RSS raw
row, flush, then cadence termination. Full-sample collection/completion intervals,
end-of-write sample duration/intervals also fail closed over40ms, including the
first sample. Raw full rows and supplemental late rows precede termination.
Coarse timestamps/intervals remain separate diagnostics; timing rows must not be
counted as RSS samples. samples counts attempts, completed_samples counts full rows.
The root stays suspended until the sampler positively observes its RSS>0 and all
registered supervisor RSS through a complete successful sample. Its freshness is
checked before resume, with a raw row before any stale-coverage censor. A raw
root-end row covers only the root. A separate WHOLE-JOB post-drain row records
the observed empty membership and last recorded sample before sampler shutdown;
an unknown/nonempty Job or >40ms tail censors. This can conservatively censor
a delay after the Job became empty. Exception paths already retain errors and
cannot qualify through later emergency closure. Native two-empty closure proof
and token release remain exact and occur later. Main and
sampler raw writes share a lock to preserve gzip integrity. That wait is
clipped to the unchanged absolute_deadline; failure never resumes or feeds stdin.

All native Job caps/commit flags, token/nonce/start identity, child assignment,
RSS/refusal policy, independent absolute watchdog, budget/free-space checks and
two-empty closure are unchanged. Functional cutoff2026-10-21T11:10:14Z and
closeout13:10:14Z/sourcefreeze04:10:14Z remain fixed. Setup and token waits consume
the global clock. Strict cadence can censor real host stalls; no false >=25Hz
claim is allowed. This is sampled RSS, not a kernel hard-RSS guarantee.

GitNexus cached1.6.9 list/impact against the exact Matrix checkout returned
repository-not-found. Indexed impact UNKNOWN; manually treated HIGH. The new run
and nested memory_watch affect future opted-in guard stages; no existing importer,
Driver, helper, engine or frozen guard is edited. Parent static review is required
before native benign controls. The controlling consultation/gates/admission and
full frozen guard/smoke sources were read; structured-format standard remains
proposed. Existing JSON machine receipt and gzip JSONL formats are retained with
additive timing/provenance fields; no engine serialization change.

Planned separate controls: short normal/nested/absolute timeout, injected slow
first full sample/start gap/missing owned RSS, prospective cap refusal, complete
gzip EOF+SHA/native PID-start+token/Job closure; then only if admitted a5-15s tiny
CPU/IO stress. Retain every failure/censor prefix. No synthetic passing evidence
may qualify actual engine/native runs. Not executed in this source-only stage.

Parent tail-gap remediation: the normal root-exit/drain flow is unchanged; the
new sampling boundary follows its final positive empty observation, BEFORE stop,
stream closure, native Job close and token release. Static corner: last full
sample999ms, root exit1000ms, descendant empty1060ms with sampler starved. Root
gap1ms would pass, WHOLE-JOB gap61ms now writes a raw boundary and censors. No live
control was executed. Frozen reference actual and expected SHA remain identical.
