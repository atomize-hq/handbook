# Separate final-pin Linux qualification: 4bde1563

**Passed** on native Linux/ext4 for exact source
`4bde156323812ef18c0dea6798cfc2108aaad933`. This appends a new qualification;
the [original 6e66 results](../README.md), measurements, cancellation and raw
archive remain bound to their original source. No matrix was repeated, no old
evidence or build was deleted, and no engine source was edited in this lane.
The parent owns integration and the decision to admit P.

The only changed core file relative to 6e66 is `src/store.rs`: `Store::init`
omits the obsolete root README when the new store uses profile 4. Profiles 2/3
retain their existing genesis boilerplate; authenticated profile-1 fixtures keep
their original boilerplate. The pinned `focused.py` additionally asserts README
absence for profile-4 A/B genesis. No schema, source representation, production
API or owner contract changed. The structured-data standard remains proposed.

## Exact source, executable and harness

| Binding | Value |
| --- | --- |
| Candidate | `4bde156323812ef18c0dea6798cfc2108aaad933` |
| Core `src/` Git tree | `d22e997c7c75940a6b4077368549d28c1780ef49` |
| Native release executable SHA-256 | `da75d34e5fe162447e356aead2fba0f2a1ac21bc1055ebb3812bce80fda79846` |
| Exact source/harness manifest SHA-256 | `4e9a39306385256489df87dca084870843cf1c75fdaccc3008fc5abfe0a7f1d2` |
| Authenticated final raw archive SHA-256 | `07ec4c40e36489424f23b05d58732c0baa511f2e28d4fa578254846b6e5e7548` |

[The manifest](source-manifest.json) binds all **16 core Rust/Cargo files**
individually, plus consumers, comparison sources and controlling harnesses in the
163-file committed-blob export. Native source was verified before compilation
and again at export. Comparing all 16 core hashes to the prior manifest confirms
only `store.rs` changed. [Transfer verification](transfer-verification.json).

Fresh native root:
`/home/spenser/file-native-n-linux-20261019/final-4bde`.
Both the original 6e66 and final 4bde source/build trees coexist. The final
executable is 2,324,504 bytes, built with Rust/Cargo 1.89.0 on native x86_64
Linux/ext4, Python 3.13.7, and existing locked offline dependencies. Dev/test
debug information and incremental compilation were disabled; the executable
was built in release mode with `-j2`. [Environment](raw/environment.json).
The report branch's source checkout was not silently replaced: the actual
compiler inputs came from the authenticated 4bde blob export.

## Actual checks

| Check | Result / receipt |
| --- | --- |
| `cargo test --locked --offline --workspace -j2 --target-dir <fresh>` | Two bounds/codec tests passed; workspace consumer binaries linked. 5.15 s. [Output](raw/tests.stdout). |
| `cargo build --locked --offline --release -j2 --target-dir <fresh>` | Passed in 11.26 s. [Build output](raw/build.stderr). |
| Actual final-pin `focused.py --exe … --base … --capsule` | Passed **39 cases / 153 calls**, including explicit A/B N128 README absence. Complete outer stage 140.25 s. [Summary](raw/focused-data/summary.json), [events](raw/focused-data/events.json). |
| Full adapted recovery capsule on that executable | Passed all **24 cases / 282 calls**, 71.37 s within the enclosing focused stage. [Summary](raw/focused-data/recovery-capsule/summary.json), [resource receipts](raw/focused-data/capsule-resources.json). |
| Explicit A/B N16 profile-4 genesis | README absent and complete independent graph/projection/source verification passed. [Smoke summary](raw/readme-data/summary.json). |
| Stored profile-1/2/3 boilerplate, both modes | Six cases preserved exact README and old publication bytes through a profile-4 upgrade and checkpoint. Legacy profile 1 came from the authenticated packaged fixture; 2/3 were explicitly initialized under those profiles. [Smoke events](raw/readme-data/events.json). |

The full focused run includes dirtiness, grouped split/merge, typed ordered
duplicate adjacency, target location, deletion, external navigation divergence,
interrupted A refresh, mixed/all-complete self-describing JSON, whitespace/key
order stability, navigation absence, packets, checkpoints, and frozen legacy
ordinary/packet candidates. Its independent oracle decodes the complete checked
graph indexes and YAML and compares exact authored bytes and current A / retained
B materializations. The adapted recovery capsule covers actual restored stores,
pending A plans, recreated divergence, fences, pins, eligible candidates, packet
ownership, half-preparation recovery, checkpoint/transfer/retire/repeated GC and
valid restored successors. Its original sources were not edited.

The explicit README suite took 11.56 s. Profile-1's original README mentions
`NAVIGATION.md`; profiles 2/3 mention `collection.yaml`. The test preserves those
actual historical bytes, rather than normalizing them to the new dialect.
All eight smoke cases passed. These are additional final-pin receipts, not a
relabeling of the earlier 601-edge supplement or N128/N1k measurement matrix.

## Fixed clock, capacity and preservation

The original lane start remains **2026-10-19T22:54:45Z**. Execution cutoff remains
**23:44:45Z**, final deadline **23:54:45Z**. The original N overall deadline was
not extended. Final-pin checks completed at **23:21:46Z**. Each ordinary child
retained its 120-second cap; the enclosing focused group had an effective hard
deadline equal to the earlier original cutoff or 1,800-second cap. All group
descendants, including checker and engine, inherited that group and would be
killed on censor. [Stage records](raw/stages.json).

Admission counted the preserved parent native root, added source/build/fixture/
return forecast, and a 64 MiB local/Git allowance: **691,047,792 bytes** against
the original **805,306,368-byte / 100,000-file** lane allowance.
[Admission](raw/admission.json). This remains inside the parent's already
approved combined 40 GiB / 5,000,000-file envelope, including its internal reserve;
no additional allowance was inferred.

The new sampler censused the **entire continuing native root**, including all
6e66 artifacts and the cancelled N10k prefix, every half second. Its largest
in-stage sample was 531,876,761 bytes / 24,267 files. Archive-end native census
was 532,426,140 bytes / 24,301 files. Minimum observed free space was
41,743,826,944 bytes, above 20 GiB. Build/test/focused/checker groups include the
supervisor's own high-water RSS: maximum aggregate sample 664,694,784 bytes;
focused group 60,121,088. All are below the 3 GiB host aggregate and applicable
2 GiB checker limits.

All 461 child resource rows have samples; maximum sampled engine-child RSS was
5,771,264 bytes and longest ordinary child 7.48 s. Point readers retain their
256 MiB limit. These are sampled observations, not power-loss or physical-I/O
qualification. [Review summary](raw/review-summary.json).

Export rehashed the original 6e66 source, executable and authenticated archive,
and the prior Q executable; all matched. The complete shared Cargo census had
zero logical growth. The [closing host census](closing-native.json) found no
changed prior native scopes and no matching task Python/engine processes under
its stated filter. [Combined final accounting](closing-combined.json) includes
both native generations, source/build/fixture/return copies, the old temporary
test root, all local artifacts and **all shared Git growth since dispatch**:
about **550 MB / 24,828 files**, plus a further 16 MiB / 1,000-file allowance for
closing receipts and Git writes inside the same lane cap.

One initial **local export inventory assertion failed before native transfer**:
its core selector mistakenly included the consumer/comparison `src/` paths in
the 16-file core count. The original script and
[failure record](prepare-attempt1.json) are preserved. The corrected selector
binds exactly 14 core Rust files and two Cargo files; all 163 transport files
remain included. There was no failed/censored engine case in this final-pin run.
No old cancellation or failed admission was erased or relabeled.

## Return and review boundary

[The final raw archive](final-raw.tar.gz) contains **30 raw files /
3,050,382 uncompressed bytes**, 350,686 compressed bytes. Every member hash,
length and archive hash was checked before local extraction. Returned native
external harness bytes equal the local scripts. All final cases, exact arguments,
source/core/executable bindings and original outputs remain separate under this
directory. Large engine fixtures and build outputs remain on the native host.

The existing GitNexus graph does not register this worktree, so its failed cached
precommit query remains UNKNOWN; no package-manager bootstrap, cache deletion or
index rebuild was attempted. Manual tracing and staged diff review constrain
this change to the existing qualification directory and this separate final-pin
subdirectory. Only external harness/report/evidence changed in this lane. No
production adoption, API freeze, physical power-loss guarantee or new dependency
is implied by these results.
