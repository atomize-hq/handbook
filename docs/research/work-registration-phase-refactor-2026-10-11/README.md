# Work-registration phase refactor native evidence

Packet 1 measurement support and results for the
[controlling plan](../../specs/handbook-contract-membrane/slices/work-registration-phase-refactor/README.md).
The exact current owner/versioned JSON contracts and 32 retained transactions /
128 MiB store inventory bound remain unchanged. This evidence selects no general format default,
index/backend, capacity increase, parent-handle reuse or new freshness witness.

## Results

All **36 accepted pristine pairs** improved the actual prepare/save/invoke
workflow, with six faster pairs in every required cell. Median paired reductions
range from **33.51% to 39.80%**. Preparation, invocation and actual workflow values
below are separately computed wall medians in milliseconds; the workflow includes
the complete saved original locator and durable saving. Every phase's wall/CPU
observations and dispersion are retained in [summary.json](summary.json), with
[workflow dispersion and CPU tables](performance-detail-tables.md). This is the
selected Windows warm native comparison, not a latency SLA.

| Input / history | Baseline preparation / invocation / actual pair (ms) | Refactor preparation / invocation / actual pair (ms) | Median paired saving (ms) | Median paired reduction | Faster pairs |
| --- | --- | --- | --- | --- | --- |
| explicit / 8 | 1621.14 / 3435.12 / 5065.97 | 44.43 / 3046.75 / 3095.84 | 1959.05 | 38.62% | 6/6 |
| explicit / 16 | 2289.04 / 4914.41 / 7181.77 | 44.51 / 4272.22 / 4322.07 | 2856.46 | 39.73% | 6/6 |
| explicit / 31 | 3641.64 / 8119.45 / 11766.85 | 44.01 / 7026.99 / 7075.81 | 4650.80 | 39.80% | 6/6 |
| kind / 8 | 1853.61 / 3681.47 / 5629.08 | 494.37 / 3151.85 / 3760.39 | 1821.53 | 33.51% | 6/6 |
| kind / 16 | 2581.20 / 5509.04 / 8070.04 | 819.62 / 4422.54 / 5250.38 | 2856.63 | 35.18% | 6/6 |
| kind / 31 | 4312.84 / 9429.00 / 13766.62 | 1567.35 / 7291.56 / 8895.76 | 4802.26 | 34.66% | 6/6 |

Within-cell paired-saving sample SDs are 190.14–259.20 ms, while the smallest
individual workflow saving is 1,601.68 ms. Each cell's descriptive seeded
10,000-resample median bootstrap interval excludes zero, and the exact two-sided
paired sign-flip result is 0.03125 for six positive differences. These small,
diverse matched-fixture samples do not establish a production latency distribution.
The [six isolated dimension controls](control-tables.md) also improve preparation,
invocation and workflow individually; each has only one matched pair.

Corrected enabled attribution checks both sources at every history/cohort:

| Phase | Baseline scans / inventories / manifests | Refactor scans / inventories / manifests | Refactor live `validate_current` calls |
| --- | --- | --- | --- |
| Explicit preparation | 2 / 2 / 2 | 0 / 0 / 0 | 0 |
| Kind preparation | 2 / 2 / 2 | 1 / 1 / 1 | 0 |
| Fresh Item invocation, both forms | 4 / 5 / 6 | 4 / 5 / 6 | 8, also 8 in baseline |

The [observation tables](profile-tables.md) and
[raw observation data](profile-observations.json) keep I1 separate from I2–I4,
including all reconstruction/encoding bytes, native proof hit/miss bytes, listings,
entries and successful bounded leaf reads. I1 reconstructs every admitted row in
its own temporary artifact scope. At history 31, I1 derives 31 rows in each source;
I2 likewise derives 31. The refactor derives the new candidate once outside scans,
then I3 and I4 each hit all 32 exact intent keys without rebuilding outputs.
Baseline I3 and I4 each reconstruct 32 outputs. Thus held I2–I4 output/item-output
derivations fall **95→31**, with **64 artifact hits**, for either form. The held map
contains 32 entries including the outside-scan candidate. Baseline already had a
per-scan historical-proof set; its new hit/miss counters are absent, so zeros are
not evidence of missing baseline proof reuse. Successful artifact hits also skip
the already derived historical proof; fresh context/status/physical reads remain.

At history 31, held I2–I4 canonical encodes fall 3,355→1,162 for explicit and
6,963→1,758 for kind. Returned bounded leaf reads remain identical within those
held observations: 720 calls and 23.48 MiB explicit, or 720 calls and 27.93 MiB
kind. Listings remain 2,245 calls/55,950 entries in either form/source. The
[held-history table](profile-held-history-tables.md) reports every history,
including encoded byte totals. This isolates pure reconstruction savings while
preserving the physical observation structure; it does not measure disk traffic.

The [nested owner spans](profile-owner-spans.json) include native current-check/
capture/historical validation, grant selection, scope, canonical encoding and
strict native durability operations with inclusive/exclusive wall and CPU.
At history 31 the four refactor scan spans still total 4,462.85 ms explicit and
4,852.16 ms kind inclusive, within the 6,579.63/7,023.25 ms instrumented invocations.
`validate_current` remains eight calls, 740.01/755.71 ms inclusive. Native proof
capture remains one call, 275.19/278.79 ms inclusive. These nested costs overlap
their owners and cannot be added as separately removable time. Exact per-path
guard-held durations are in [profile-lock-held.json](profile-lock-held.json).

The 24 matching attribution observations compare enabled/disabled telemetry in
independent roots. Their 12 invocation differences have median **78.37 ms**, range
**−10.06 to 208.65 ms**; see [all overhead controls](instrumentation-overhead-tables.md).
One control per cell/source includes timing variation, including the negative
observation. No overhead is subtracted from pristine latency. Attribution pair
timing includes preparation profile materialization and is not acceptance latency.
The observer correction and excluded/replaced pristine pair are recorded below,
with their diagnostics and original observations retained.

### Memory and native resources

Absolute pristine memory peaks increase at larger histories. All cells are shown
in [memory-tables.md](memory-tables.md), with raw per-process samples and release
checks in [memory-summary.json](memory-summary.json). At history 31, explicit
sampled private-peak median/max changes **90.84/108.75→104.14/128.09 MiB**;
kind changes **113.78/116.69→135.29/139.61 MiB**. Median increases are 13.31 and
21.52 MiB. OS working-set peak maxima are 118.82→137.55 and 126.31→149.07 MiB.
This is a visible memory tradeoff, not unchanged resource use. The 128 MiB
constant limits the store inventory; it is not a process memory cap.

The invocation-local raw intent key plus five expected record buffers reach
9/17/32 entries at histories 8/16/31. Their explicit byte gauges are
2,238,034/4,231,666/7,969,726 and kind gauges
2,600,869/5,014,861/9,541,096; see [cache gauges](profile-cache-gauges.json).
At 32 entries that is 7.60/9.10 MiB. The gauges exclude decoded intent/output,
current `Parsed` clones, Rc bookkeeping, allocator and stack/temporary buffers;
they cannot explain the complete process-memory difference quantitatively.
Artifacts retain no handles, are pruned to currently admitted exact intent bytes,
and live only within the consuming invocation or separate temporary scan scope.
They remain bounded by current record/history limits. Allocator residency may
outlive their Rust values.

All 72 accepted pristine observations return handles to their pre-workflow count.
Sampled handle peaks are close in each cell; history-31 maxima are 650/650 explicit
and 649/650 kind for baseline/refactor. Both same-process resource runs then pass
all 48 calls with per-call and total handles **57→57**, no tracked lock left after
any profiled phase, exact original locators, and supported recovery meanings:
BeforeIntent→Unaccepted; Intent/Snapshot→Committed; PublicationAttempt→Uncertain;
Reply→Replayed. Six freshly bound originals refuse after revocation while the
six committed-original recoveries replay. Assertions also check joined race
threads return handles to baseline. [Resource summary](resource-summary.json)
retains all phase dispositions and repeated private/working memory sequences.

| Source | 48-call process handles | Sampled private / OS working-set peak (MiB) | Sampled handle peak | Initial history-9 replay private-after range (MiB) | Final history-23 refusal/replay private-after range (MiB) |
| --- | --- | --- | --- | --- | --- |
| baseline | 57→57 | 58.57 / 68.81 | 509 | 13.73–15.12 | 16.29–18.66 |
| refactor | 57→57 | 63.88 / 78.78 | 508 | 14.20–15.40 | 17.25–43.47 |

History deliberately grows 9→23 between these fixed-history tails. The final
refactor tail is not monotonic and its maximum is below an earlier 47.71 MiB
after-call value during injected Reply, but its last value is still 43.37 MiB.
This supports bounded observed residency and no monotonic accumulation in this
sequence, not full memory release or a long-run leak proof. Resource artifact
gauges reach at most 23 entries/5,721,696 buffer bytes. Handle assertions alone
are not memory evidence.

The separate current-read race blocks on `.handbook/work-state/work.lock` through
`WorkLock::open`. Its one child-thread `lock_wait` span is **3,404.9798 ms baseline /
3,528.2715 ms refactor**. The injected 200 ms delay occurs in the parent's
BeforeIntent callback; the parent then continues registration under its work
guard. It is not the complete guard lifetime. After acquisition, the child's
work-lock held durations are **943.954/1,023.392 ms**. Full child current-read wall
is 4,356.1925/4,561.0163 ms, with process CPU 4,218.75/4,437.50 ms including both
threads. Wall spans are child-thread-local; the parent guard lifetime is not
separately profiled. The [resource tables](resource-tables.md) and raw race profiles
retain these distinct measurements. Nested spans and overlapping guard lifetimes
are not additive removable cost.

The common driver, four actual binaries and 18 primary source/dependency files
passed [post-run source verification](verified-measurement-sources.json).
[Timing-end inventory](timing-window-end.json) completed with no known native
build/test/population/measurement workers. The separate
[focused verification](verification.md) reconciles 80 passed tests. Its three
ignored child helpers ran through passing parents; two physical-root cases with
unavailable historical originals remain unavailable, not passed evidence.

## Method

The pristine baseline is detached `8e5c6f21077a508d40cd0661bd1946d04a132281`.
The refactor source is a detached copy of the authorized live product deltas;
`build-*.json`, source snapshots and attribution patches retain exact provenance.
`native_main.rs` is the same driver in all builds. Pristine binaries invoke
`sdk.register_work_item` with no injected boundary callback. Attribution builds
insert only research spans/counters in isolated sources and use the same
handle-sampling boundary callback in enabled and disabled controls. No
instrumentation source is installed in the product.

Main trials use six independent-root baseline/refactor pairs for each
explicit/kind cell at fixed pre-invocation work histories 8, 16 and 31. The kind
total includes its grant and committed kind versions. Both physical roots of
every pair are constructed independently through public native owners; no root,
proof or authority bytes are copied. Population is untimed and uses one documented
public-owner driver for both controls. Logical inputs and native/work byte sizes
are checked within pairs before timing. Trial order reverses baseline/refactor
between samples and rotates histories/cohorts. OS caches are warm after population.
The parity evidence retains both complete raw leaf-path/size manifests. Comparison
normalizes only native Charter record and admission-request fingerprint path
positions, with multiplicity preserved; all work transaction/slot paths stay
exact. Independent physical fingerprint values are never copied or rewritten.
The only accepted grammars are
`.handbook/local-charter/v1/records/{candidate|intake|validation|approval|promotion|lifecycle}_{[0-9a-f]{64}}.json`
and `.handbook/admission-governance/v2/requests/{[0-9a-f]{64}}/{leaf}.json`, where
`leaf` is `intent`, `instruction`, `rule`, `rules`, `decision`, `deferral`,
`outcome`, `relation`, `consumed`, `terminal`, `configuration`,
`current-displaced`, `predecessor`, `request`, `terms` or `stage-0` through
`stage-6`. These positions
come from [Charter record construction](../../../crates/engine/src/local_charter.rs#L549)
and [native deferral request storage](../../../crates/contracts/src/project_admission_governance/deferral.rs#L913)
/ [governed transition storage](../../../crates/contracts/src/project_admission_governance/transition.rs#L776),
with their retained inventory allowlists.
Every actually normalized original path and comparison path is retained in the
parity JSON. Unknown path differences fail equality.

The actual pair timer starts before preparation and ends after invocation. It
includes construction of the returned complete original locator, cloning and
canonical serialization for saving, and a file write followed by `sync_all`.
Preparation and invocation are also timed separately; their independently
computed medians are never added to invent a workflow median. Supplied input/
initial/grant fixture loading occurs before the workflow in both controls.

| Main sample | Native basis | Committed grant lineage | Role | Kind versions | Byte dimension |
| --- | --- | --- | --- | --- | --- |
| 0 | D | 1 | Ordinary | 1 | Small |
| 1 | G | 2 | Ordinary | 1 | Small |
| 2 | D | 3 | Ordinary | 1 | Small |
| 3 | G | 4 | Planning output | 2 | Small |
| 4 | D | 1 | Requirements input | 2 | Native initial/check instructions each 4,000 bytes; small work fields |
| 5 | G | 4 | Ordinary | 2 | Native small; UTF-8 label/question each exactly 4,096 bytes |

Explicit cohorts have zero kind versions and preserve the same supplied role.
The focused `controls` cohort at history 16 varies one dimension relative to
sample 0: sample 6 only native instruction bytes; 7 only work field bounds; 8
only a second ordinary kind version; 9 only planning role. Samples 6/7 cover
explicit and kind; 8/9 cover kind. This is a bounded diversity/control design,
not a complete factorial study.

Attribution uses separately constructed fixed-history sample-0 roots, with
enabled/disabled mode and baseline/refactor order alternated. The `onx` and `off`
root names have equal byte lengths. Its pair timer includes preparation profile
materialization, whereas invocation materialization follows the pair timer;
these attribution pair latencies are not pristine performance evidence. Span
inclusive/exclusive wall and CPU are nested attribution, not additive removable
cost or OS syscall/disk-I/O proof. Observation 1 is I1; 2–4 are I2–I4 for successful
Item invocation. Outside-scan observations are 0. The separate preparation phase
resets observation numbering.

Resource checks repeat 48 replay, injected-error/recovery, and revoked fresh
refusal/original replay calls in each same process, with per-call and overall
handle checks and private/working-set snapshots. Three repetitions exercise
BeforeIntent, Intent, Snapshot, PublicationAttempt and Reply interruption prefixes.
Those injected originals deliberately grow retained history. The six initial
committed replays and the final six revoked fresh refusals/original replays each
hold history fixed; repeated phase memory values and absolute process peaks are
reported separately. Handle release assertions alone do not prove memory reuse.
All outcomes retain their supported refusal, noncommit or original-recovery
meaning. A separate work-lock race injects a 200 ms parent delay and measures
the complete concurrent current-read wait; its CPU is process user+kernel CPU and therefore
includes both threads. This race is outside the uncontended latency study.
Research cache gauges count admitted raw-key plus expected-record buffer bytes;
they exclude typed/Rc/allocator overhead, and are not process memory measurements.
Lock-held samples cover successful returned guards until their native unlock;
strict acquisition spans additionally include path/metadata checks, while
`lock_wait` isolates the blocking native `file.lock()` call.

## Reproduction

Use new detached source checkouts and fresh independent roots. Never run this
against a retained user authority store. `prepare_driver.py` installs the same
standalone manifest/lock and driver in each isolated source. `snapshot_source.py`
copies authorized live crate deltas only. `instrument.py` requires retained exact
primary-index impact evidence and adds attribution-only hooks. Its new artifact
helpers are covered by the existing owning parser/scan impact and direct source.

Build with `build_driver.py SOURCE TARGET NAME [profile]`; it records the locked
offline release command, toolchain, source and executable SHA-256 and full output.
Use `run_pairs.py populate EXE pristine`, then after all population/build/test
workloads finish, `run_pairs.py measure BASELINE REFACTOR pristine`. `controls`
uses the focused dimension fixtures. Populate `attribution-onx` and
`attribution-off`, then run `run_pairs.py attribution PROFILE_BASE PROFILE_REFACTOR`.
All timed processes are serial. The runner uses files while polling 100 ms resource
samples, so large attribution output cannot block on a Windows pipe buffer.
The same polling pass records process handle count for pristine, attribution
enabled/disabled and same-process resource runs. It retains the sampled maximum
alongside boundary callback maxima and before/after release checks. The sampler
addition was made before any timings; `sampler-revision.json` records its scope.
Full native stdout/stderr and collected rows are retained.

Run `summarize.py parity pristine controls attribution-onx attribution-off` before
any timed fresh effect. After the serial runs, `summarize.py`, `profile_tables.py`
and `summarize_memory.py` verify and summarize the complete data. Finally run
`run_resources.py PROFILE_BASE PROFILE_REFACTOR` against the already measured
sample-0/history-8 explicit roots, then `summarize_resources.py`. The resource
program deliberately consumes extra slots and revokes its original grant, so
those roots are no longer pristine pre-invocation fixtures afterwards.
`final_tables.py` renders dispersion/CPU, isolated dimension, overhead and held
history tables from retained data only; it performs no native effects.
[evidence-sha256.json](evidence-sha256.json) records exact raw-file hashes for
this comparative evidence and support. Its explicit exclusions cover itself and
the separately finalized focused-test verification/logs.

`retain_build.py` saves each completed executable before its Cargo dependency
target is reused. `retained_exe` and the recorded executable hash identify the
binary actually used. `retain_profile.py` retains instrumentation-only patches
and exact raw-file hashes against the matched pristine source.

The actual pristine latency binaries are `pristine-baseline-v2` and
`pristine-refactor-v2`; attribution uses the matched `profile-baseline-v3` and
`profile-refactor-v3`. All four use the same driver bytes. The first profile attempt
and a diagnostic retry exposed a research observer defect: explicit lock release
takes the file from its guard, bypassing the Drop-only observer. Preparation and
locator saving finished, then the second phase's observer lock-map assertion
failed before invocation. The initial stack/before-saving diagnosis was incorrect.
Both disposable profile sources now observe explicit release after the existing
unlock/error mapping. The product is unchanged. Old binaries, diagnostics and
patches are retained. `attribution-correction.json` proves retained history 8 and
the complete raw path/size manifest stayed unchanged, with a pristine native
current-item inspection and current leaf hashes; the saved locator is outside the
authority store. Corrected attribution retries consume the first fresh Item
effect on that root.

The first pristine explicit/history-8 sample-0 pair overlapped a still-finishing
environment/process snapshot. Both observations are retained as diagnostic and
excluded. After the original series, all timing paused while a fresh independent
replacement pair was populated and parity/source/environment/quiescence checks
completed. It ran baseline first and refactor second; actual late sequence and
UTC launch/collection timestamps are retained in `replaced-measurements.jsonl`.
`main-selection.json` links the complete original 72 observations, the excluded
pair and the accepted 72-observation series (74 actual pristine observations).
Resource controls use the original already-measured sample-0 roots.

The first four sample-0/history-8 roots were populated by the initial pristine
baseline driver. Four sample-0/history-16 workers were suspended while release
suites ran; the old queued scheduler was stopped to prevent later jobs using an
outdated coupled fixture profile. Those workers finish their original native
population before a read-only inspect reconciliation. Their roots and source/
binary provenance are retained. No measured effect is recycled or pre-run.

## Limits

This is warm Windows 11 / NTFS / MSVC native evidence on the recorded host and
toolchain. It does not establish cold-cache latency, production p95, Unix,
hardware power-loss behavior, large-population throughput or physical disk-I/O
counts. CPU has Windows process-time quantization. Polling private-memory and
handle maxima can miss brief transients or the final interval; polling and
boundary handle maxima are sampled, not exact peaks.
Bounded leaf-byte counters count successful returned `read_regular_bounded`
payloads; they exclude separate checkpoint/fingerprint raw reads and are not
kernel read-byte or physical disk-I/O counters. The OS reports working-set peak;
private-byte peak is sampled every 100 ms. Absolute values and within-process
repeated samples include decoded intent/output and allocator costs beyond the
cache-buffer gauge.
Allocator residency can persist without retained guards or authority cache state.
Existing physical-root cases requiring unavailable originals remain unavailable;
an ignored case is not passed evidence. Four invocation history scans remain
substantial staged cost. Adjacent I1/I2 choreography and administrative headroom
are separate follow-ups, not part of this packet.

The comparison supports Packet 1's performance boundary: all required cells
improve beyond their observed variation, successful pure reconstructions fall,
and invocation physical/current-check counts and original outcomes stay intact.
Its invocation-local artifact bound and measured memory cost are explicit above.
Four scans remain substantial staged cost. Reassess repeated-history cost and
possible I1/I2 choreography separately, proving original conflict/replay fallback,
native-before-work ordering, races and error priority. Dependency-closure/index/
control publication, audit selection, retained-original treatment and protected
administrative headroom remain with the capacity follow-up. Existing root/initial
anchors, current-authority witness and physical fingerprints need no new primitive;
today's coherent-administrator restore trust limit remains.
