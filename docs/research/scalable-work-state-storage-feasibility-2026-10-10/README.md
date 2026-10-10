# Scalable work-state storage feasibility evidence

Research only, source baseline `889d9b24f0d3be33169c14951d4336dd86ee03e2`.
The [report](../scalable-work-state-storage-feasibility-2026-10-10.md) interprets
these experiments. Product source, limits, configurations, lockfile and retained
user stores are unchanged. The format standard's general defaults remain
proposed; no storage architecture or integrity-policy change is selected.

## Native profile reproduction

Use a fresh external archive of the baseline, not a retained worktree/store.
Rust/Cargo 1.89.0 MSVC, release defaults, two build jobs, Engine `test-support`,
32 MiB worker stack; Python 3.11.9 for this preparation and summary. Product
dependencies and Cargo.lock are reused exactly from the preceding accepted
research harness. Build and store output stay outside tracked sources.

```powershell
$profileCopy = Join-Path $env:TEMP ('hcm-scalable-source-' + [guid]::NewGuid())
$profileTar = $profileCopy + '.tar'
$profileTarget = Join-Path $env:TEMP 'hcm-scalable-profile-target'
$profileOutput = Join-Path $env:TEMP ('hcm-scalable-output-' + [guid]::NewGuid())
New-Item -ItemType Directory -Path $profileCopy,$profileOutput | Out-Null
git archive --format=tar --output=$profileTar 889d9b24f0d3be33169c14951d4336dd86ee03e2
tar -xf $profileTar -C $profileCopy
python docs/research/scalable-work-state-storage-feasibility-2026-10-10/prepare_profile.py $profileCopy
cargo build --locked --offline --release --jobs 2 --manifest-path "$profileCopy/docs/research/scalable-work-state-storage-feasibility-2026-10-10/native/Cargo.toml" --target-dir $profileTarget
& "$profileTarget/release/work-state-capacity-research.exe" $profileOutput > "$profileOutput/native-profile.jsonl"
python docs/research/scalable-work-state-storage-feasibility-2026-10-10/summarize_native.py "$profileOutput/native-profile.jsonl" "$profileOutput/native-summary.json"
```

Remove `--offline` only to fetch the same locked dependencies if not cached.
The preparation script requires an archive outside this checkout and rejects a
previously instrumented copy. It adds 96 RAII span entries and one research
module; the generated `profile-insertions.json` records the exact original file,
line and symbol of each insertion. The three checked-in source files plus this
recipe specify the instrumentation diff. `#[allow(unsafe_code)]` applies only
to the external research module calling Windows GetProcessTimes; it does not
change the product's lint configuration. WPR CPU recording was attempted and
failed with `0xc5585011`; WPR status confirmed no recording afterward.

Wall spans nest: inclusive times overlap and must not be added. Exclusive times
subtract instrumented children; they include all work not further instrumented.
Process CPU combines user and kernel time; its observed resolution was 15.625 ms,
so small individual/exclusive CPU spans are noisy and are not a syscall-level
profile. Counter calls, span construction and aggregation add overhead. Three
instrumentation-enabled reads alternate with three disabled-span controls at
each size; the disabled control still has a cheap conditional at the inserted
sites and is not a pristine unpatched binary. Effect timing has one instrumented
sample with no matched control. No disk-busy or scheduler-wait attribution is
inferred from elapsed time minus CPU time.

Every native call reopens the owner. These runs have warm OS caches after
population, no cache eviction, no process restart measurement and no hardware
power-loss simulation. Shared histories mix 27/256-byte questions within supplied
native scope. The distinct cohort captures original D then G native histories
using supported governance establishment and fresh unchanged-scope work activation.
It is two proof populations, not 16 distinct native authority epochs. Setup and
population are excluded. Native outcomes are asserted; an incomplete/failed
process is never accepted as a complete profile.

The profile exports one exact native intent, its exact canonical authority object,
and its exact terminal bytes from a real committed scratch item. Their physical
identities belong to that scratch root; they are opaque comparison data, not
portable authorization. [Prototype instructions](prototypes-README.md) state
the different synthetic meanings and durable settings used for the storage matrix.
Serial native/prototype runs avoid competing benchmark workloads. Raw large
stores, source copies, binaries/build trees and temporary dumps remain outside
tracked sources. Compact raw observations, summaries and original comparison
fixtures are retained here. New physical identities can change fixture bytes on
reproduction; every backend in a run must receive the same fixture bytes.
