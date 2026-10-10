# Native optimization evidence

Research only at `8013948ebd3fbef6725f5d72f3d29120f7617009`. Product changes
exist only as an isolated worktree and reproducible patch. Production limits,
formats, exact fingerprints and detection policy remain unchanged in the working
branch. The [report](../work-state-native-optimization-investigation-2026-10-10.md)
owns interpretation and the proposed successor's human review boundary.

## Reproduction

Use a fresh detached checkout outside the primary repository; all stores are new
native scratch roots. A physical authority store cannot be cloned between roots.
Do not apply this experiment to a retained user store. The complete patch is the
final measured Rust source, including its 17 causal spans and new research module.
`prepare.py` records how the parent-handle/owner/read experiments and spans were
constructed after the separate semantic experiment. Reproduction uses the final
patch directly, not `prepare.py` on an already patched source.

```powershell
$experiment = Join-Path $env:TEMP ('hcm-native-opt-source-' + [guid]::NewGuid())
$target = Join-Path $env:TEMP ('hcm-native-opt-target-' + [guid]::NewGuid())
$evidence = Join-Path $env:TEMP ('hcm-native-opt-results-' + [guid]::NewGuid())
$inputs = (Resolve-Path docs/research/work-state-native-optimization-2026-10-10).Path
git worktree add --detach $experiment 8013948ebd3fbef6725f5d72f3d29120f7617009
git -C $experiment apply --check "$inputs/experiment.patch"
git -C $experiment apply "$inputs/experiment.patch"
$native = Join-Path $experiment 'docs/research/work-state-native-optimization-2026-10-10/native'
New-Item -ItemType Directory -Path "$native/src",$evidence | Out-Null
Copy-Item docs/research/work-state-capacity-and-performance-2026-10-10/harness/Cargo.toml "$native/Cargo.toml"
Copy-Item docs/research/work-state-capacity-and-performance-2026-10-10/harness/Cargo.lock "$native/Cargo.lock"
Copy-Item "$inputs/native_main.rs" "$native/src/main.rs"
cargo build --locked --offline --release --jobs 2 --manifest-path "$native/Cargo.toml" --target-dir $target
& "$inputs/verify.ps1" -Experiment $experiment -Target $target -Evidence $evidence
foreach ($mode in @('probes','reads','effects','pending')) {
    python "$inputs/run_native.py" "$target/release/work-state-capacity-research.exe" $mode $evidence
    if ($LASTEXITCODE -ne 0) { throw 'incomplete native run' }
}
python "$inputs/summarize.py" $evidence
python "$inputs/report_tables.py" $evidence
```

To check this retained result set, run:

```powershell
python "$inputs/check_evidence.py" $inputs
```

Remove `--offline` only to retrieve the same locked dependencies if absent.
Rust/Cargo 1.89.0 MSVC, release defaults, two jobs, Engine test-support, 32 MiB
worker stack; Python 3.11.9. The baseline product source and Cargo.lock equal the
prior deeper native profile. Native benchmark/build/test workloads are serialized.
The host is Windows 11 build 26200, i9-12900KF (16 cores/24 logical processors),
about 128 GiB RAM and NTFS; `environment.json` records the actual queries.
`build-final.txt` records the measured binary's final successful build; the earlier
`build.txt` is a preliminary build, superseded before all retained trial modes.

## Controls and limits

`base` is the unoptimized path in the experimental binary, including disabled
span/counter/variant branches; it is not a pristine binary. Enabled and disabled
current reads alternate, while variant order rotates. Current reads have four
enabled and four disabled observations per exact-size group; preparation/read/
committed recovery controls have six observations. SDK forwards invocation
directly to the same native owner; the invocation harness adds only a boundary
handle-count sample and no injected failure. Fresh effects use separate physical
stores at matching entry sizes 25–30, rotating/reversing variant order: six
unprofiled observations per variant, one diagnostic profiled effect at 31. Actual
preparation plus invocation is paired before computing workflow medians.

Every timed owner call reopens its native owner. OS caches are warm after
population; fresh owner invocation is not a cold-disk test. No cache eviction,
new-process/cold-disk comparison, hardware power-loss proof, ETW syscall trace,
parallel throughput benchmark or production p95 follows from this evidence.
Pending recovery at full 32 is one profiled observation per variant, accompanied
by relevant integrity/durability tests; its time is illustrative.

Inclusive spans overlap. Exclusive wall subtracts measured children and includes
unmeasured work; it is not disk or syscall attribution. CPU is Windows process
user+kernel CPU, with approximately 15.625 ms quantization. Returned bounded-read
bytes exclude retained-file fingerprint/checkpoint reads. Directory units count
successful listed entries, not failed-list partial observations. Boundary handles
are sampled counts, not an exact handle peak. Working set is the largest
OS-reported peak observed during polling; a peak in the final polling interval
can be missed. Private bytes are sampled every 200 ms across population and trials,
not per-operation allocation. These counts cannot certify physical immutability.

`owner`, `binding`, `semantic`, and `combined` retain original physical inventory,
content, current-control and publication checks. `direct` and `combined-direct`
remove required-read ancestor inventory observations and are separately marked
semantic departures; the hard-link probe demonstrates why they cannot be promoted
as unchanged-policy optimizations.

`gitnexus-impact-final.json` retains actual upstream results and graph caveats.
The original stale index/missing-table results were not successful impact proof.
Bundled Node 24.19 rebuilt the baseline index; FTS remained degraded, graph queries
then worked. The rebuild's tracked instruction changes were restored. Six newly
generated untracked skill folders remain outside the research commit because
automatic approval review blocked their cleanup.

Closeout whitespace checks exclude the preserved `experiment.patch`, three raw
test logs and the two captured Pro answers. Their patch context, terminal blank
lines and Markdown hard breaks are intentional source bytes; the authored
research/document files are checked separately.
