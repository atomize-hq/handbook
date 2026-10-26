# Accepted file-native source and research checkpoint

This is source/research preservation, not production adoption or a completed database experiment. The accepted source and decisive positive/negative observations are now in one isolated Git line; no new engine work or runtime qualification was performed to create this checkpoint.

| Commit / source identity | What it establishes |
|---|---|
| `0628322e4b48e0efad19cdc32238a622d4aaa7a8` | Frozen baseline; original31 test catalog was unrun when frozen. |
| `123afcae984d40ab47bac4091e5d58794c3c17d9` | Only telemetry fixture `n=1 -> 10`; io.rs SHA `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`. Retained complete31 tests and real80,976,318-byte fixture passed on all three native hosts. |
| `a1b743d81bb12fbd538181a16fb8fa2da35368b7` | Accepted Linux checkpoint-resume repair and nine focused tests. Exact26-file manifest `43a298bb575b724187eea89de6d55176d67992c2ed3f025e78f8e46d90c981cb`; physical_lifetime.rs SHA `36603ef499aa29234441f28b2095627ba8c93e3a1657a6f35aaa3a3198c4d8e0`. Focused recovery improvement only. |
| The commit containing this document | Curated research/evidence checkpoint. It does not change the accepted crate. |

The accepted source is [the research crate](../file-native-incremental-engine-experiment-2026-10-18/). A common source commit on Mac/Linux/Windows does **not** establish all-host runtime qualification of the new repair: the three complete31 runs and Windows/Mac successful pilots used the earlier fixture10 source. The accepted repair has Linux nine-test/work-bound/non-test-compile and independent-review evidence. The latest Linux repaired-source pilot still failed drainage.

## Current product truth

| Native observation | Source and disposition |
|---|---|
| Windows | Fixture10 source:31/31 PASS, real boundary PASS, functional A/N64/H3 pilot PASS;16engine+4checker calls,16.1077172s old reader, actual H2 deletion, backlog0, full independent oracle. Four cleanup batches <=25.9851ms. Earlier30/31 fixture failure and failed native transport/PID-identity gap remain retained. |
| Mac | Fixture10 source:31/31 PASS and real boundary PASS. Same-store functional pilot completed after retained adapter NameError and proof-only continuation;19engine+1checker,16.103821917s old reader, actual H2 deletion/backlog0/full oracle. Cleanup51.227250/52.667042ms target misses remain. |
| Linux accepted repair | Exact26 source: nine focused recovery tests, Work-bound and non-test compile PASS; independent review accepted. Latest native pilot17engine calls: real H3 and16.113507960s old reader, but two drainage calls left backlog6->8 and H2 loose file present; final complete oracle UNRUN. Seven of eight cleanup batches exceeded50ms. |
| Rejected optimization | Isolated27-source candidate `01bc6bc93a827c0490e45b712647d70eebd35cc1facbb5f76dbb64ee57c804c3` is **NOT ACCEPTED**. Its patch/review/negative results are research only; CWR-01/CWR-02 remain P2 open. Removed root syncs measured5.489-5.967 microseconds versus mandatory barriers8.7-9.25 milliseconds. No rejected source entered the crate. |

Windows reports directory sync as unsupported; Unix performs sync_all. Cross-host timing therefore is not an equal-durability proof. No complete operating envelope, full matrix,40ms monitoring or hard all-instants memory qualification is established here.

## Read the evidence

- [Windows corrected tests](windows/tests-pass/report.md); [Windows pilot](windows/pilot-pass/report.md).
- [Mac corrected tests](macos/tests-pass/report.md); [Mac pilot and limitations](macos/pilot-pass-with-preserved-failure/report.md).
- [Accepted Linux repair](linux/accepted-repair/REPORT.md); [independent repair review](linux/accepted-repair/independent-review.md).
- [Latest Linux drainage assessment](linux/research/drainage-assessment/REPORT.md); [Linux export status](linux/EXPORT-STATUS.json).
- [Rejected optimization research](linux/rejected-research/) and [stored-path/original-path/hash map](provenance.json).

Some exact Markdown/raw logs are stored as `.gz` to preserve original whitespace. Decompress them to obtain the exact source bytes/hash in provenance.json; they were not edited to pass whitespace checks. Host reports contain historical absolute origin paths. These local links and provenance map identify durable copied evidence; the old paths are not promises that temporary originals survive.

## Preservation boundary

The original Linux/Windows export manifests are preserved with their original meaning: they describe the received bundles, not a claim that every export member was copied under the same name here. `provenance.json` maps every imported stored file to exact original bytes and origin, and discloses omissions. Current accepted26 source is in commit `a1b743d8`; duplicate fixture10 preimage is in `123afcae`, so those source copies were not duplicated as alternative authorities. Test-scoped red preimages and the immutable original accepted repair patch are retained. Its original `git apply --check` error (`dev/null: No such file or directory`) is preserved; exact reviewed source was imported and checked instead of pretending that check passed.

No targets, Cargo caches, store payload corpora, pack/index datasets, executable binaries or whole chats are included. Small state/control observations and source/binary hash bindings are included. This is **not a complete archive**, not a safe-to-delete declaration, and not proof that omitted originals are recoverable. See [Linux original-copy gaps](linux/EXCLUSIONS-AND-SOLE-ORIGINAL-GAPS.md), `provenance.json`, and [reproduction/source closure](REPRODUCTION.md). Older consolidated inventory is historical; the accepted26 and latest drainage status above control current interpretation.

The original Mac `/private/tmp` source is mostly missing; no recovery was written over it. The new persistent sparse worktree recovered exact source from Git plus the accepted exports. Primary tracked/index state and N/Q worktrees were left alone; missing old worktree registrations were not pruned. No new tests/builds/pilots/indexing, cleanup repair, dependency change, main merge, PR or origin push occurred during this checkpoint.
