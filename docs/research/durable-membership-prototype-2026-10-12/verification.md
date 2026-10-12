# Evidence and verification provenance

All results are from the isolated experiment; none qualifies Handbook production
behavior. Main raw responses/assertions are retained losslessly as
`samples.jsonl.gz` / `checks.jsonl.gz`. `scripts/pack_evidence.py` uses deterministic
gzip and verifies byte-for-byte decompression against the original. The original
uncompressed logs and generated stores/builds remain local and ignored. Small
concurrent-memory logs remain plain JSONL, with per-PID timestamps and units.

| Run | Native storage | Passing assertions | Failures | Skips |
| --- | --- | ---: | ---: | ---: |
| `evidence/windows` main | Windows/NTFS | 5,898 | 0 | 0 |
| `evidence/linux` main | Linux/WSL2 ext4 | 5,912 | 0 | 0 |
| `evidence/windows-supplement` | Windows/NTFS | 984 | 0 | 0 |
| `evidence/windows-agents` 100k-history | Windows/NTFS | 428 | 0 | 0 |
| `evidence/linux-agents` 100k-history | Linux/WSL2 ext4 | 428 | 0 | 0 |

Each `run-status.json` is authoritative for its run. Supplemental and agent
statuses intentionally have `full_harness_complete=false`; only main runs execute
the full measurement/concurrency/fault/validation suite. Summary's
`nonpassing_checks` also contains `status=observation` rows such as storage and
cohort observations; those rows are not failures. The counts are assertions,
including repetitive membership checks, not independent test cases.

## Repaired-code verification

Final common-layer SHA-256:
`40d601b4bc47a452edcfaf625e3493e2e0d9bc9bfae3b290c1e58899df60cb0d`.
`evidence/windows-repaired-run` is a complete new 30-sample 1k/10k/100k main
run with 5,924 passing assertions, zero failure/skip. Its Windows binary hash is
`400db2cd1547779e1dc671ed9e491b4e968500aad2edbf26fbe43f790513b0b1`.
`evidence/linux-repaired-run` is the matching full native Linux/ext4 rerun with
5,912 passing assertions, zero failure/skip. Its binary hash is
`6a638566267a9fd423ac92659e8bcc5788354df2912d534979f06ef06bb1c6c2`.
Together the final full runs and exact final focused repair runs have **12,068
passing assertions**, zero failure/skip. Each operating system also passes all
nine final unit tests. Combined retained initial/final runs contain 25,950 passing
assertions and 132 observation rows; the regenerated summary has 1,808 groups.

`evidence/{windows,linux}-repair-final` each passes 116 focused assertions with
zero failure/skip, using the exact final `scripts/review_regression.py`. These
checks reach object-install interruption/error, reuse an actual orphan, verify
name boundary work, and force a named replay-repair error after root_replace
before allowing the exact original outcome to be acknowledged. Earlier focused
runs are retained as `windows-repair` / `linux-repair`; final script additionally
requires exact checkpoint names for injected errors. The Windows repaired main
environment snapshot includes the earlier auxiliary repair-script hash, but that
script was not executed by `run.py`; final focused-run provenance is authoritative
for the strengthened checks.

Both native operating systems reran all nine unit tests after the repair and
rebuilt release binaries with the isolated lockfile. Chunk/disk algorithms and
their source hashes are unchanged. Final source formatting and Python syntax
checks passed. All initial measurements and their exact common source remain
retained, including the pre-repair overlapping-agent cohorts.

## Initial measurements and repair provenance

The first five run rows above used initial common-layer source, retained exactly
at `evidence/measured-main.rs`, SHA-256
`1ed8e9cd48b17e29e726143f4ee23706d6a026a265c24b80edcd5a29bf7ddc67`.
Independent review found two P2 publication sequencing gaps; the final source
repairs installed-orphan shard persistence and selected-root replay flushing.
The initial source/evidence is retained rather than relabeled as final code.

The Windows main process loaded an earlier harness revision before later focused
checks were added. Its exact source is retained at
`evidence/windows-run-harness.py`, SHA-256
`06de33a35f41588a524fa2862267629643c554a46e2278d06a46abe8311e4204`.
The final `scripts/run.py`, hash
`94d42fc4dc9a0e0cf5cf67732d21bef6c94051270437c7a4edc4bb318bb0966f`,
adds changed-proof/item conflict and missing root/owner/object namespace checks.
The Windows supplement ran final concurrency/validation; Linux main ran the final
entire harness. Measurement and fault paths were unchanged between those harness
versions. Native Rust binaries used the same initial Rust source/lockfile
throughout those initial runs.

Windows main binary SHA-256:
`959eabfe2cdc2d422d57e40d161d052565df105187fccadca9713be4a3f5ec2a`.
Linux main binary SHA-256:
`0651b6907789198ea850d177ffb56f2fa151954cb15c0edbbbda02c4d6fb0a6c`.
Environment files retain the exact source, Cargo and binary hashes, OS/filesystem
observations, arguments, cache policy and unavailable metrics. Builds used the
isolated lockfile with Rust/Cargo 1.89.0. Binaries are not committed.

`summarize.py` later gained gzip-reading support; most initial and Linux runtime snapshots
record its earlier hash `73fd19751529582b6285f65756ecb76043198981cea9e104cd60cc9bbbeb7d6f`.
Final summarizer hash is
`b73b0b90617b09979ad9104151bbf75c37597498fda91a30beedafca4e90d59b`.
Statistics and aggregation logic did not change; gzip-only reads match original
rows, and the reviewer reproduced all original 924 groups from raw evidence.
Linux runtime harnesses did not execute that older summarizer; final aggregation
used the retained final version above, and all 1,808 groups reproduce from raw logs.

During the experiment, `cargo test --locked` passed all nine module tests on
Windows and all nine on native Linux/ext4. The four chunk and five disk tests
cover randomized reference-map equality, reopen/snapshot/retained roots,
incremental encoding or path mutation and malformed bounded schemas/structure.
These session-observed unit results are recorded here; raw terminal output is not
retained. Native CLI integration/fault assertions have raw retained evidence.
Final `cargo fmt -- --check` passed. Gzip-only summarizer reads were verified
against the original 5,922 Windows assertion/observation rows.

All 590 bounded input paths in `evidence/inputs.json` were rehashed after execution
and matched their baseline hashes. This includes the prior research and applicable
contracts. Git showed no modification of pre-existing tracked files. The manifest
also records starting HEAD/branch and product phase-refactor reference. Unrelated
untracked research remains outside this unit and is excluded from its commit.

GitNexus's registered index predates the starting HEAD by three commits. The
original index's registered Node/CLI runner was used; the generic launcher chose
an incompatible runtime. Scoped impact calls for this workspace's new symbols
returned `Target not found` / `UNKNOWN`, so they provide no verified blast-radius
claim. The experiment has no production imports or indexed production-symbol
edits. GitNexus 1.6.12 `detect-changes --scope staged --limit 12` examined the
72 staged files and reported: “Diff touched 72 file(s) but no indexed symbols
overlap those hunks — not a clean tree.” This matches the isolated new-file scope;
it is not a successful indexed analysis of the new symbols. Unindexed new-file
coverage remains a limitation.

Hardware power loss, guest/host reboot, bare-metal Linux and native macOS were not
tested. Concurrent RSS is OS-reported process residency, includes shared pages per
process and is not unique physical RAM, PSS, allocations or a simultaneous peak.
The five-second preparation pause ensures overlap; cohort elapsed time includes
that deliberate pause and cannot be interpreted as isolated commit throughput.
