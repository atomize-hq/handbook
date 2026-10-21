# Full qualification bodies source review — frozen Q10

Verdict: **REMEDIATE — source only. Runtime NO_GO.**

One required P2 finding remains in the source-owned all-sink forecast. No additional required change was identified in the reviewed LONG, HISTORY or legacy decoder bodies. Separate Windows integration observations below do not qualify or replace the parallel whole-platform review.

## Exact scope and evidence

The ten regular files total309,273 bytes. The self-excluding manifest is1,780 bytes, SHA2569a1ac35baa177b6f7e0057c30fb3e63a87834d6ff75ec120e3e64ab472873173. Every listed member's actual bytes/hash was verified; the actual directory has exactly those nine members plus the manifest.

Read in full: long_lane.py32,413 bytes/563 lines, SHA33ebdba457002c2b1e2b5b3d9ff6e64985e570b1621769179f4e59fa1106fa60; long_source_profiles.py84,940 bytes/1,519 lines, SHAdff8f89773e0f3b93e907253f9b217b3b532f0a760c3686ea3d45cc2f97e1f71; legacy_archive_restore.py14,204 bytes, SHA79e7ccac897d4c38aa53bbecbb049c7e6d327e834cf806b77c1a22c4d7f826bc; HISTORY56,996 bytes, SHAdb1541a81e35932a5cd1edb5a5c38d1b12b7cac50c0424e2954182f98cf7922a. fake_controls.py, fake_cases.json, the disabled example and README were also read completely.

The exact104,749-byte finalizer3fd82367193bbe9e5849e2ada66e70ed42797827e1ad7125e2eef405b10dabc2 is covered by its separate full independent INTEGRATE review; that report was checked. This review does not claim a new full read of that finalizer. The90,716-byte Runtime9a64752efacb6ae2912e5f3cce81f746868618971d637fca04acb47e0d059924 retains this reviewer's prior full R3 source proof plus the exact affected069 admission review. Neither acceptance grants runtime authority.

## Q1 — P2 Required: account for the live checker-output file reservation

long_source_profiles.py:397-399 allocates output_files=7/8/5/6 for legacy_full/full/mapping/backlog. LONG's sink at944 budgets five times the legacy retained-file count. HISTORY's checker sinks at1296 multiply the respective retained-mode counts. control_allocations:823-842 validates the actual CHECKER_WRITER_CONTRACT and models gate holds, but never consumes its output_reserved_names or adds an active/unsettled output-file term.

The inspected Windows producer windows_capsule.py:614-616 reserves2*(len(CONTRACT.artifacts)-1)+1=17 output names before dispatch. Its CHECKER_WRITER_CONTRACT:644 declares output_reserved_names17, max_live1 and failure retain-full-unsettled-and-stop. windows_oracle_bridge.py:250-285 settles that reservation only after actual same-call process/IO/callback closure and verification of every retained final/partial output. A failure retains the full unresolved hold.

Consequently the frozen forecast's all-sink proof omits a real intermediate/failure file reservation. At the mode ceilings, five legacy retained prefixes account for35 files, whereas four retained prefixes plus the fifth17-name live hold require45 within that component. Even six actual successful files per prior legacy prefix give41 at that point. This is a component/escrow completeness finding, not a claim that every supplied global allocation must fail: unrelated spare future artifact entries are not a source-derived accounting term for this writer.

Add an explicit evidence-category file escrow derived from the actual source contract's maximum live checker writers and reserved output-name count, or prove a tighter peak from the exact source schedule and retained members. Keep the existing per-call total_output_bytes allowance; do not count the same output bytes twice merely to repair the file count. Preserve full unresolved holds until positive writer settlement. The two stream-helper gate holds and checker/control gate terms are already present and concern different physical names; they do not supply the missing output reservation. Apply the term to both LONG and HISTORY models and update the source-owned forecast identity.

No source repair was performed by this reviewer. The reviewed snapshot remains REMEDIATE.

## Body, admission and provider checks

LONG validates its exact source roots, bound inputs, phase forecasts and allocation identity, authorizes before private module loading, and loads the exact bounded raw buffer it hashes. The legacy adapter reuses the fixed reviewed stream decoder, hashes the same buffer before compilation, authenticates original archive metadata, enforces original DFS order and FNOP1 framing, checks file/compressed digests plus strict single-member EOF, writes only a fresh quarantine and does not claim semantic or engine release. Failure retains the quarantine.

LONG requires a complete independent legacy oracle before granting engine access. The five oracle stages retain actual request/index/native and companion output references. Matched fresh/warm lookup identities, semantic head and independent inventory comparisons remain enforced. Conversion, cleanup, stream restore and accepted destination successor have explicit separate phases. The final result remains provisional pending the post-host-exit join.

The profile model consumes actual native wire/guard/control/checker/provider literals. It rejects caller schema substitution, authenticates source geometry, distinguishes source-file/action/global-catalogue/model hashes, and requires an authenticated global catalogue before execution. Unbound previews do not get a usable global hash. Both actual inner and outer numeric guard_bounds must be supplied with the closed source-owned shape and minima before the model returns. No actual14-CLI/two-Job raw-bound values are released by this review; source-bound numeric admission remains required before GO.

The paired stream contract is source-owned: two fixed producer/consumer helper IDs, maximum two live helpers, control stdin0/stdout4096/stderr8192, dedicated inherited anonymous payload pipe, no retained payload copy, twelve bounded metadata names, two native raw references and actual native metadata/closure/gate refs. The explicitly parent-approved fresh private module instance's fixed Runtime/subprocess-Popen adapter was inspected as an existing allowed design; it is not treated as a forbidden global standard-library mutation or caller-selected callback algorithm.

LONG counts613+k actual native operations:601 resolves, two warm calls, setup, conversion, publication, k cleanup calls, five independent oracles and two stream helpers. Source-note accounting incorporates seven startup gates and full two-helper unsettled gate holds, with four callback-wait observations as a normal-plus-failure upper bound. Q1 concerns output-file escrow, rather than these repaired helper terms.

HISTORY retains the two exact donors,22 cases,16 phases,26 complete oracles,1188 mapping checks,24 backlog checks,4000 offers,2400 terminal losers and802 quota checks. Actual phase profiles export the closed case scope/count; initial cases are sealed after closure and the full22-case trace after sixteen phase bodies. Control maintenance remains0 with backlogNone/UNMEASURED; online requires measured zero after two actual drain passes. The immutable finite reader schedule and44/48/23.6-second policy remain source-bound. No final outer-Job PASS is asserted in a live host.

## Separate Windows integration observations

These observations were sent to META for the separately assigned Windows reviewer. They are pinned to the inspected companion hashes and are not additional required changes to this frozen Q10 source. Later adjacent-path rereads found the first two repaired; this does not accept the full changed Windows source:

- windows_oracle_bridge.py20,202 bytes/SHA8aea1d3d02fd2c025d8051cb32c30a4dc7729793318b6922087346849a71f2ec: host.checker_results population is after an unconditional return in checker_settlement; release_long_root indexes that registry. At later20,183-byte/SHA6a69098d878a66f68e1dcc83045e5e4c52879168cf9b01bccac7237f9ae2a192 the publication is before return, repairing this observation.
- windows_capsule.py42,379 bytes/SHA12fe1f61cf7360ca6e808d02e81418e0262b8e7c3e64ab1b86f03d40972576f0: run_lane stores the RangeRef returned by run_long in host.long_result, while long_stream_dependency expects a proof body with transport/successor fields. At later43,338-byte/SHA2f1125dcd8ef5de6160562ad2ccb875348ad4d8dafbadfb14079eebe41b5b707 the body is read and hashed from the original result reference before dependency use, repairing this observation.
- windows_hooks.legacy_forecast compares original archive remaining bytes/files against independently selected stream limits. The legacy adapter uses the authenticated original592,458,971-byte/5,807-file archive limits. The consumer must join the correct allocation rather than assume they equal the later stream envelope. This mismatch remains in the inspected52,951-byte hooks source, SHA688fb3e761f4930c619ef7c27880b6807d4f6c1353b92527e14e7b376837784f.

The Windows sources were WIP during adjacent-path inspection. No whole Windows acceptance or current integrated runtime qualification follows from these observations. The independent finalizer and Runtime source acceptances remain scoped to their exact hashes.

## Review limits, controls and accounting

The prepared Q pure suite has48 small units. It explicitly excludes full forecasts, mixed file replay, native lifecycle, archive/corpus behavior and post-exit joins; none were run. This does not rescore any separate matrix unit family or prior negative result. No private-module imports, AST/compile, private execution, controls/tests, native/DLL/Job/engine/Cargo/bootstrap actions, Git/indexing, source edits or delegation were performed.

Correctness has Q1 above. Architecture/security preserve source-owned formulas, actual native records, source grant equality, byte quarantine and provisional-only authority. Readability is dense in the large profile module but does not create an additional required change under the fixed source allocation. Performance and actual timing are unmeasured; bounded formulas are not measured native results. The structured-data standard remains PROPOSED, with no format/default migration approved.

Only this fresh071 review.md/evidence.json pair is written, total at most24,576 bytes. Before each write and after the pair, META v15 counts all P regular files, both Landing root levels, seven selected subtrees with the declared exclusion and24 promoted harness files, once per pathname. Dynamic future retains base112617 plus unused allowances for R4.4, Windows, Qualification, v38 return and v38 review. There is no old-pair revival, duplicate pool, deletion/expiry/relabel credit or new source copy. Parent alone closes unused allowances. All ten source hashes are rechecked at handoff; old R2 and Runtime reports are preserved.
