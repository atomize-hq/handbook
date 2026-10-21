# Independent focused harness v3.1 source review

**Bounded SOURCE disposition: INTEGRATE exact frozen v3.1. Full qualification: INCOMPLETE. All real cases: PENDING. Execution: HOLD.**

F5 is addressed in source. No additional required source defect was established in the complete v3-to-v3.1 diff and affected actor/closure paths. This accepts a source handoff only; it grants no runtime GO, resource allocation, final engine/schema acceptance, native qualification or clock reset. The original v3 REMEDIATE remains unchanged.

## F5 correction

`focused_protocol.py:667-680` now requires the complete post-join STORE to equal the installed target captured before release, and selected Work to equal the captured hash-bound candidate. This includes catalog and physical id/ordinal/plan; no arbitrary normalization exception is allowed. The comparisons occur before the generic transition check, mapped-tail assertion and successful joined-overlap evidence. That evidence additionally records selected_target, captured_work_sha256 and exact_candidate_selected.

The candidate is independently parsed from its hash-checked plan before either release. The native state reader rechecks selected plan bytes against the selected binding. Exact selected STORE plus selected Work equality therefore ties the completed-cutoff/parent proof to the actual selection rather than merely another changed mapping. The sole physical maintainer uses cleanup=false; the paired ordinary no-op publication supplies no permitted physical-plan normalization. The two assertions implement the requested causal identity without relying on a generic full oracle.

Actor joins already precede these comparisons. A target mismatch consequently fails after both pair waits and remains subject to outer held-reader cleanup and native/Python registry closure. The accepted publication input/result may remain retained in the input ledger on this failure, correctly preserving an actual publication rather than hiding it; successful overlap/schedule evidence is not recorded.

## Static control and closure inspection

The new overlap controls retain deep independent copies of target and selected Work, avoiding an alias that would make the equality test meaningless. Two explicit commit orderings are modeled: publisher waits on maintenance_done for maintenance-first, and maintenance waits on publisher_done for publisher-first. Events are cleared before the maintenance ready barrier, and publication sets its completion event after establishing the accepted result. These controls do not alter the production release/join implementation.

For each ordering, one exact-target positive expects15successful joined pairs. Six independent post-release mutations change selected id, catalog, plan, ordinal, Work physical_parent or Work completed cutoff. The negative assertions require the specific STORE/Work mismatch error and absence of both successful joined and schedule-complete rows. Existing trace checks still require every returned actor's wait observation, closed threads and independent case registry closure, including held readers. Fourteen checks are added: two positives and twelve negatives.

No test, control suite, compile check or native process was executed by this reviewer in this retarget, as requested. The receipt's reported50overlap checks,36original controls and33remediation checks/53exception scenarios are worker-reported memory results, not independently rerun evidence. Static inspection supports the new assertions and control structure; real final-pin candidate-to-selected correspondence and native actor lifetime still require qualification.

## Complete diff and source identity

Reviewed the complete diff against the preserved v3 negative archive: the core adds only the two comparisons, explanation and success-evidence fields; overlap_controls adds ordering synchronization/selected Work modeling and exact-target controls; README, forecast and receipt describe those changes. case_ledger, fake_controls, focused_schedules, remediation_controls, remediation.md and the preserved v2 receipt are byte-unchanged. The prior F1-F4 fixes, perf_counter_ns observations, transport-only on_started behavior, one seed plus15pairs,16accepted/cycle,1/4/1 configurations, four held readers and exact final two-call small-tail drainage are not weakened. Terminal-v2 remains an honest fail-closed pending interface and cannot qualify the four-publisher runtime.

All11files/225383bytes match the entry and completion hashes. Receipt SHA256 `4719748175f99b3f00a281f526c0deabcac0977122c07bb6d7c517080fb45381`; independently recomputed sorted compact ten-file manifest SHA256 `06194b44624ff43a9f9f966016fa4799875b04e1d8fdededb6b002ce6c1e5a2f`. Core SHA256 `7ce0a4a06a35fa9e7f4f4080a40315a418bb45e47a8946ca06d78c2117d855e8`; overlap controls SHA256 `ed1374d0ebde93ff4e064fd0cfeb4ce87810178ff08862e8bbb1bebd3c370bd8`.

Preserved v3 archive SHA256 `11b134c1ca04cb17925fbeaccdbf1aeb4239ec0a1ff030f3e23f45332d0c6b15` was independently hashed and read in memory without extraction; all archived member sizes/hashes match its original478f receipt. Existing review-v3.md and evidence-v3.json retain their original dcaf0000 and7c443d2f hashes. No source/raw copies or fixture reopening occurred.

Only this report and evidence-v3.1.json are written. No supplied harness/test code, engine, native control, build, oracle, Git/index/bootstrap, source edit, subagent or new chat was run/created. Standard-library source reads/diffs/hash calculations do not execute the reviewed modules. Prior controlling-document/contract review remains inherited; proposed structured-format status and UNKNOWN/manualHIGH GitNexus limitations are unchanged.

## Remaining boundary

Source fits the existing224KiB/16file envelope. Runtime remains NO_GO; parent-owned a3 sufficient forecasts and every final-source/adapter/schema/donor/guard binding still require review and prospective admission. Terminal-v2, native sharing/death/cancel/path reuse, protected reader/preparation quiescence, all independent full oracles, native/Matrix/long representation and restore obligations remain mandatory. No hidden COW reset, missing case or planned deletion credit can substitute. Final functional2026-10-21T11:10:14Z and protected closeout13:10:14Z remain fixed.

**INTEGRATE is limited to the exact reviewed v3.1 source handoff; full qualification remains INCOMPLETE and real execution HOLD.**
