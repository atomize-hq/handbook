# R2 frozen shared-runtime static review

Verdict: **REMEDIATE. Runtime NO_GO.** The fixed source cutoff of 2026-10-21T04:10:14Z has elapsed. No post-cutoff source repair or runtime action is authorized by this review.

The complete final runtime source was read: runtime_contracts.py, 81,001 bytes, SHA256 de7702855dbb17b5cd00832aff457215ddcc4419bd77c3490faa1764c3eb932d. Claim042 permits 81,920 bytes / one file. Exact companion hashes are in evidence-R2.json. Review used source text, hashes and searches only; no private imports or compilation.

## R2-A — P2: phase-profile ABI deterministically refuses

runtime_contracts.py:474-480 requires history_phase_profile to return case_index_scope equal to closed-case-all-engine-and-checker-calls and an integer case_index_max_calls. Frozen long_source_profiles.py:1077-1100 now returns the count but still omits case_index_scope. Its get therefore yields None and preflight raises source-derived-complete-case-index-geometry for every phase, even with otherwise valid ports/bindings/admission. This surviving portion of R1-A is independent of missing Windows platform code. A caller constant or asserted GO cannot replace the source-owned producer contract.

## R2-B — P2: six initial indices and complete trace are never sealed

Frozen history_lane.py run_history:445 onward closes two lookup and four bootstrap cases, then seals sixteen phase bodies with seal_final_provisional. It never calls runner.seal_case after initial closure or runner.seal_trace after all phase seals. Its return contains comparison/lookups/phases/phase_seals and no complete trace.

Runtime case_indices starts empty and trace_ref starts None (399-400). seal_case:655-672 is the initial-index path; seal_trace:674-693 requires all22 indices and sixteen sealed bodies before writing history-expanded-case-index-v2. A search of all existing Python sources under the research root found these names only at their runtime definitions. No existing dispatcher caller proves the missing steps. Native ports alone do not invoke them. Frozen finalizer reconstruct_whole_schedule:697-705 requires all22 actual case indices. Even if R2-A were separately resolved, the supplied coordinator cannot furnish complete whole-history evidence. An invented manifest or newly authored post-cutoff algorithm is not a substitute.

## Repaired items and retained boundaries

Earlier lookup metadata/per-format oracle, paced family/clock/physical tuple, history wrapper ABI, selected18-field final body and source-derived index width/count findings are repaired, except R2-A's missing scope. R1-B is resolved: control boundary_cleanup_calls is0; backlog remains None/UNMEASURED. The current history validator requires measured backlog only online, where two actual boundary passes and measured zero remain mandatory. This supersedes the old report's suggestion that control backlog must be proven zero.

The current contract requires21 ports including case_call_pairs, actual parent case_id separated from helper scope_id and fresh32hex case_scope, all engine/checker call pairs, four ready_refs and origin_ref, distinct source-file/model hashes, completed case attachments, actual buffer SHA/length/reread and external own-body hashes. The initial-case and global-trace profile functions exist; invocation remains absent.

The fixed16-phase/1600-accept schedule, finite native reader streams, immutable origin,44s publisher/48s reader/23.6s old-pin profile, actual mapped metrics and point terminal-charge checks are static observations. Concrete reviewed Windows prefix/ports, positive source-bound admission, actual native closure/raw evidence, all mandatory cells and post-host-exit receipt join remain unproved dependencies. In-host summaries remain PROVISIONAL_UNQUALIFIED; no final PASS is claimed.

## Scope and accounting

The structured-data standard remains PROPOSED; no default/format/migration approval follows. No source edit, private import, compile, control/native/engine/Cargo run, Git/indexing, delegation or new algorithm occurred. Prior review.md/evidence.json are preserved unchanged. Only this new R2 pair is written, together at most12,288 bytes, with a fresh META10v5 whole-membership census before each write and256,000 bytes future reserve retained. All copies are counted once per pathname; no deletion, expiry or relabel credit is used. Reservation handling and exact census are recorded in evidence. Final four source hashes are rechecked before handoff; sources stay frozen. Verdict remains REMEDIATE / runtime NO_GO.
