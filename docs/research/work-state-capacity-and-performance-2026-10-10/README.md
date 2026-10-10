# Work-state capacity research harness

This is a bounded research executable, not a product benchmark platform. It
references the current Engine and SDK by path and the existing supplied test
setup helpers. Every store is a newly created task-owned temporary directory;
the executable accepts no retained project path. It never changes product
sources, capacity constants, the workspace lockfile, or existing stored truth.

The [investigation report](../work-state-capacity-and-performance-investigation-2026-10-10.md)
interprets the evidence and leaves design selection to the human. The format
standard remains proposed; this machine-readable research output adopts no
product storage policy.

## Reproduce

From the repository root in PowerShell, with Rust 1.89.0 and Python 3.11:

```powershell
$benchTarget = Join-Path ([System.IO.Path]::GetTempPath()) 'hcm-work-capacity-research-target'
$benchOutput = Join-Path ([System.IO.Path]::GetTempPath()) ('hcm-work-capacity-results-' + [guid]::NewGuid().ToString())
New-Item -ItemType Directory -Path $benchOutput | Out-Null
cargo build --locked --offline --release --jobs 2 --manifest-path docs/research/work-state-capacity-and-performance-2026-10-10/harness/Cargo.toml --target-dir $benchTarget
cargo run --locked --offline --release --jobs 2 --manifest-path docs/research/work-state-capacity-and-performance-2026-10-10/harness/Cargo.toml --target-dir $benchTarget -- all > (Join-Path $benchOutput 'results.jsonl')
python docs/research/work-state-capacity-and-performance-2026-10-10/summarize.py (Join-Path $benchOutput 'results.jsonl') (Join-Path $benchOutput 'summary.json')
```

`--offline` requires the lockfile's dependencies to be cached. Remove that flag
to fetch the same locked versions when reproducing in a clean environment. The
harness lock was seeded from the workspace lock and checked to contain no new
dependency versions; only the research package was added. Build output goes to
a task-owned temporary target directory, outside the checkout. Each reproduction
uses a fresh results directory, preserving the accepted evidence in this checkout.

Modes `baseline`, `administration`, and `pending` can run separately for a focused
rerun. Mode `all` executes them sequentially. The accepted run is one `all` run;
untimed pilot output with different dependency resolution is excluded.

## What is timed

`std::time::Instant` measures wall time within the process. Setup, population,
canonical approval construction, recursive byte accounting, and scratch-store
cleanup are outside each operation timer. There is no process launch in an
operation timer. Prepared values use small supplied ordinary-leaf inputs: label
`Leaf` (4 UTF-8 bytes), question `Exact bounded work question` (27 bytes). The
authority closure retained in intents is much larger than these input fields;
the component metadata records its actual size.

The [environment](environment.json) records platform, toolchain, profile,
storage, stack size, feature selection, cache conditions and sampling. Engine
`test-support` is enabled for the entire executable. Ordinary SDK methods use
their public route, but these are **feature-enabled** measurements, not a claim
about a separately compiled default-feature consumer. Native effects at 16 and
31 slots use the existing boundary callback to timestamp real durable stages;
the other effects use the SDK's ordinary method. Callback timestamps entail
small instrumentation overhead, which has not been separately calibrated.

Repeated read groups use one untimed warmup and five timed calls on the same
store. Mutation/preparation groups have one observation at each exact history
size. Their values are individual observations, not statistically established
medians or tail guarantees. Only repeated-read/component groups have a useful
same-store observed median/range. The summarizer also computes descriptive
arithmetic for initial activation across eight fresh fixtures and full32
revocation across three distinct cohorts; those pooled mutation medians are
not repeated fixed-cohort estimates. The report uses individual observations
or their range for these pooled groups. No p95 estimate is made. Histories are populated with
committed items unless a mode explicitly creates pending or noncommitted truth.

- **Baseline:** one explicit cohort and one kind cohort. Registration prepare
  is timed separately through native and SDK routes at 2, 3, 8, 16, 31 and 32
  consumed slots; the effect follows. Exact item read, current item read,
  committed original recovery and current terms use both native and SDK routes.
  A kind cohort at 2 slots contains a grant and a kind, so it has no committed
  item yet; item-read measurements first appear at 3. At 32, the new effect
  must refuse with `work_capacity`, while old reads/recovery continue.
- **Administration:** one successive scope chain at 2, 16 and 31 slots, followed
  by fresh scope and revocation refusal at 32 on a valid, unrevoked latest grant.
  Scope labels alternate finite bounds with separately supplied approvals.
  Separate cohorts measure successful revocation at 2 and 31, then original
  revocation recovery at 3 and 32. One small cohort measures unchanged-scope
  D-to-G re-establishment; governance establishment itself is outside its timer.
- **Pending:** one kind cohort interrupts registration at the existing `Intent`
  boundary to leave an accepted original pending request at 31, resolves it,
  recovers it to committed, and repeats at 32. Another reaches pending at 32,
  revokes the independent local-check authority outside the timer, and measures
  positive original noncommit closure and later exact noncommit reads.
- **Contention:** three pairs of simultaneous native current-item reads at a
  full kind store, with a barrier and two 32 MiB worker stacks. Each outcome
  and duration is recorded; refusal and success are grouped separately.

Initial work-grant activation is individually timed in each fresh fixture,
after initial/native-check setup and before kind establishment. That includes
owner bootstrap; current reads and new work requests have an established owner.

Recovery observations use the original retained locator and ordinary owner
reopening after a real injected durable interruption, within the same process.
They exclude new-process startup and cold-cache restart. The current public
methods reopen and revalidate their retained owner; this harness introduces no
owner cache. Existing restart tests remain semantic precedent, not timings for
this experiment.

## Components and beyond-cap diagnostic

At a real, full kind store, five observations after one warmup measure ordinary
regular-file reads, SHA256 over in-memory JSON bytes, `serde_json::Value` decode,
and canonical serialization of already parsed Values, separately. Files,
parsed Values and input bytes are prepared outside those component timers as
appropriate. These probes do not reproduce duplicate-safe typed decoding,
native retained-root file guards, owner/manifest validation, authority proof,
dependency closure, locking, tree/checkpoint validation or durability.
They delimit raw I/O/codec costs rather than subtracting into a precise profile.

The beyond-cap diagnostic streams the same actual kind-item intent through
Value decode, canonical serialization and SHA256 32, 128, 512 and 2048 times.
Only one decoded/canonical record exists per iteration; it does not create a
large retained store or raise a product constant. Its count is
`synthetic_record_count`, and its bytes are `processed_input_bytes`. It establishes
the cost of that repeated CPU traversal in this process, not native support,
completeness/absence proof, authority or recovery guarantees beyond 32. It
excludes request uniqueness, root/lock identity, semantic proof, filesystem
guards, live currentness, writes and concurrency. A cap-only native variant
would encounter other unchanged proof bounds, including the 256-path exact-file
checkpoint; no such product-symbol experiment is included.

## Output and checks

`results.jsonl` contains each timed observation plus component metadata and
native boundary timestamps. Native `history_slots_at_entry` is the actual
consumed transaction-slot count before the call. `retained_work_bytes_at_entry`
means regular-file content beneath `.handbook/work-state`; it excludes external
anchor/bootstrap files, allocation overhead and the owner's exact byte budget
accounting. It must not be called disk allocation or available capacity.
Read outcome `verified` means the expected typed result was asserted; the
operation name identifies committed, pending or noncommit state. Mutation rows
preserve the actual returned disposition. Capacity refusal is distinct from a
successful effect. Component bytes exclude the zero-byte lock file; metadata
gives the exact surrogate record count/bytes.

`summary.json` is regenerated by the standard-library Python summarizer and
retains every group, sample count, median and observed extrema. Synthetic counts
and byte basis remain distinct from native store sizes. The executable asserts
the expected public outcomes, capacity refusal code, and original recovery
results. A nonzero exit or incomplete JSONL file is not a completed measurement
run. The whole product suite is not required for this read-only investigation.

GitNexus impact was attempted for the new harness `run` symbol and returned
target not found / risk `UNKNOWN`. Direct-source fallback found only the local
research `main -> run -> probes` callers, with no product callers. No experimental
product symbol was edited. Existing supplied test helpers are read-only context.
