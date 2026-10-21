# Frozen R4: full source review

**REMEDIATE — native/control/runtime/engine GO remains FALSE.**

Full read:1541 guard lines,985 unlocked-capture lines,296 controls lines, README, receipt and052 budget. The five originals total152747B/5 under155648B/5; each fits its current individual cap. Source hashes, dependency context and accounting are recorded in evidence.json. All five hashes were rechecked unchanged at07:32:25Z after reading, then the full read freeze was released. This verdict applies only to that R4 freeze, not a successor.

Only source text, file metadata/hashes, prior evidence and official native API documentation were read. No reviewed Python import/AST/compile, native/control/Job/engine/build/Cargo/Git action, source edit, subagent or new chat occurred. Private executable closure within the five files is stdlib/native DLLs plus the exact-buffer guard loader. The fixed bootstrap's complete previously read bytes were rechecked unchanged; current finalizer grant/lifetime/bounds paths and Context parent/argv/local-lease interface were inspected. No blanket approval of either changing external source tree is implied.

## Blocking findings

### R4-1 [P1] Persistent limit-denial accounting is not checked at Job termination

guard1457-1465 queries both extended limits and basic accounting. It correctly interprets ActiveProcesses and TotalTerminatedProcesses, and censors a nonzero limit-termination counter. Its callers are setup, pre-resume and periodic sampling: inner1042/1150/1188; outer386/486/580. Neither inner root wait/drain/final close1210-1313 nor outer finish600-679 queries accounting again.

A child denied/terminated for a limit violation can occur after the last sampler poll, and the root can finish before another poll. The persistent counter then remains nonzero but unseen; <40ms coverage and empty membership alone do not detect it. The current source can report qualified=true without the claimed complete denial accounting. This is a source acceptance gap, not an observed native failure.

Take and retain a mandatory final snapshot from the SAME still-held Job after owned activity/drain, before closing its final handle, for BOTH Jobs. Recheck actual configured limits and cumulative denial accounting. A nonzero/unknown result must censor qualification; independent physical cleanup must still be attempted/proved. Include a finite between-poll denial negative under separate admission, preserving the existing27 intended faults.

[Microsoft's basic accounting contract](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_accounting_information) defines TotalTerminatedProcesses as the cumulative limit-violation termination count. Empty membership is a separate observation.

### R4-2 [P1] Inner acquisition-pending state does not fence closure or the next run

guard1156 sets _ROOT_PENDING=true before Popen;1164 clears it only after positive Job assignment/membership. This intentionally recognizes an acquisition interval whose native result may be unknown. But finally1250 checks state.uncertain, _UNCLOSED and process.poll without checking _ROOT_PENDING. If acquisition raises while process remains None, the code can close an empty inner Job, set proven=true1312 and OWNER closed, and omit an acquisition fence. A later run checks only _UNCLOSED922, so it is not prevented by that unresolved pending state.

The watchdog consults worker_guard._ROOT_PENDING275-285, but this does not repair the guard's exported physical predicate or its later-admission gate. A Popen exception or a Python assignment that did not complete is not itself proof that no native child exists. The outer Capture already retains creating_root uncertainty; mirror that protection for the inner path.

Retain a source-owned acquisition context, make unresolved pending state prevent physical closure and later admission, and clear it only after positive proof of no native child, containment or actual reap/close. Do not fabricate a no-child witness from an empty Job. Known assigned/reaped failure branches may clear the flag only from their actual native proof. No acquisition fault was executed by this review.

## Clock/model compatibility requiring prospective alignment

Frozen R4 is honestly capped at900s: WholeCallWatchdog248 initializes min(now+900,FUNCTIONAL), configuration104 can only tighten, controls115 also caps900, and worker948-950 independently passes min(900,remaining) to the inner guard. It cannot admit a2400s stage forecast as though that were the enforced clock.

The parent's proposed3600s initial ceiling is a future change, not approval here. Changing the initial watchdog alone leaves the inner worker's min900 clamp. The shared singleton watchdog only tightens, so an old-capture configuration narrowed to2400 cannot later extend to3600 for a finalizer. Admit the true old+new orchestration endpoint at the start, with explicitly bounded phase endpoints and controls<=900 before writers/launch, or provide separately admitted source-owned watchdog scopes. Preserve the hard functional cutoff and all elapsed time; never reset or retroactively extend a phase.

The known16 fixed48s windows already consume768s before other setup/oracle/closure work. This review does not claim every actual900s attempt must time out; it finds an actual900 versus selected2400 forecast/admission mismatch that cannot be concealed by relabelling the same run. A revised clock/model and source hash need fresh review.

## Recheck of all four R3 P1 branches

R3-1 outer pre-assignment cleanup: Capture tracks creating_root/assigned/root-reaped and closes only the retained Popen native handle. terminate_unassigned_root463-472 checks that handle's state and uses TerminateProcess on it. Preparation failures call fail/finish. Closure600-687 handles missing PipeCapture and checks actual native reap/root/Job close. The outer branch now supplies the required mechanism; R4-2 above identifies the remaining inner acquisition-fence defect.

R3-2 whole-call watchdog: load_config303 obtains the source-owned watchdog before configuration/source/census reads, main966 arms before dispatch, and worker/CLI finish only after final writes and stdout flush. Native finish no longer cancels it. Expiry marks failure; a bounded12s cleanup grace cannot qualify. Acquisition uncertainty suppresses a forced hard exit while native ownership is unresolved. Completion is serialized against expiry. These source mechanisms address the old early-cancellation branch, subject to the actual clock compatibility above and native validation.

R3-3 outer terminal coverage: sample483-528 now checks start, completed-collection and recorded intervals; terminal_coverage473-481 emits bounded root/Job witnesses and censors missing/late/negative gaps. finish614/621 records both terminal events; qualified705 requires both, no health failure, no expiry, complete raw/stdio and census. This addresses the old timing-tail gap. R4-1 separately identifies the missing final kernel-accounting snapshot.

R3-4 grant API health: publish_parent_records754-755 and _issue_postexit_grant811 now require the actual old outer qualified result, no global fence and no watchdog expiry in addition to physically closed old and native suspended prepared-new capabilities. That repairs the old closed-only gate. Exact native identity checks, source-owned watchdog and actual consumer binding remain mandatory; JSON declarations do not create these capabilities.

## Native limits, supervisors and27 controls

set_job_limits1450-1455 enables flags8968, ActiveProcessLimit12 or16 and unchanged process/job commit caps2/3GiB. check_job_limits reads actual native values and accounting. Source setup/pre-resume/poll checks and explicit member-count checks now support12 inner/16 outer upper bounds;2048 remains query capacity. Native excess-denial proof is incomplete until R4-1 is fixed.

supervisors402-415 checks actual creation identities and writes at most4 live registrations. supervisor_rss451-465 counts actual live rows, refuses excess, and _super_memory725-742 refuses unreadable/zero LIVE samples while distinguishing actual departure. Registry16 is storage capacity, not the live limit. Required roles/actual scope still must be admitted; do not discard a legitimate supervisor to force fit.

The fixed control process tree remains at most2 inner/3 outer processes; sequential128 is cumulative identity churn, and its >=129 identity evidence assertion remains. These native limits bound memory/evidence breadth by refusal, but do not prove every benchmark's complete semantic roster fits. Include bootstrap/coordinator, four reader actors, publishers/maintenance/oracles, guard worker and nested children, with overlap handled explicitly. No workload, denominator or required native operation may be truncated to fit12.

All27 names/order and intended injections remain. The guard's physical proven1312 is exported independently of health errors1337; qualification1366 remains strict. Thus faithful missing-live-rss/census-error/error-storm controls may retain errors while proving resource closure, prospectively repairing the R2 contradiction without changing its failed history. Summary persistence, exact source/budget checks, held-buffer fence/reconciliation, fixed deadlines and global FAILED/NOT_STARTED handling remain. Native applicability, exact budgets/censors, all-sink fit and all27 behavior are UNRUN; no error-cap masking, skipped case, retry/cherrypick or status reinterpretation is allowed.

## Actual v2-r3 grant references and dependencies

The producer adds exactly authority_ref and profile_catalogue_ref to postexit-finalizer-grant-v2-r3 (capture815-867). Authority is loaded through a hash/size reference, names the actual whole immutable inner budget in guard_bounds_ref, and must bind the original returned budget_sha256. evidence_bounds is decoded from that same verified buffer; its exact compact ASCII insertion-order JSON+LF hash is compared with returned bounds_sha256. The emergency .bounds.json is not accepted as that budget.

The inspected actual phase_finalizer023738de paths1262-1321 consume the same two references, check authority root/cap/hold/output identity against config, validate the profile catalogue, load the whole guard budget and require actual parser bounds. guard_encoding130-135 preserves the matching byte domain; it does not sort the insertion-order bounds. Lifetime/parent observation1149-1219 links the reaped worker, inner and outer Jobs, source-owner audit, returned completion and stored receipts. Source-bound plan/consumer consistency is supported on these cited paths; full finalizer semantics and native closure are separate reviews.

The producer still requires a real held new-parent native handle, creation, membership, image and deadline in the same publisher, and closed-old native lifetimes. Only guarded-bootstrap mode can issue phase-finalizer grants. Controls/Context JSON cannot substitute for those objects. Later new-finalizer native closure and actual inclusive census remain required.

Bootstrap32f01f6f remains the previously fully read fixed entry/source-closure/startup-gate implementation. Context660f3285's actual parent/native argv/image/local lease interface was checked; R4 has no C31-specific caller feature. No new Context/all38/real-archive/RETIRE authority is supplied. These external dependencies can change under their owners; pins and separate full reviews are required before runtime.

## Preservation, scope and reporting

All five exact R4 hashes and current052 source budget were verified. Existing R2 withdrawal/R3 reviews and both negative-source packets remain preserved; this review creates no new source packet or archive. Structured-format standard remains proposed; no migration/public-default approval. GitNexus remains UNKNOWN/unregistered with manual HIGH impact; no symbols were edited or index bootstrapped.

The full read freeze was released after the unchanged07:32:25Z check for prospective retargeted fixes only. No successor is reviewed or approved while WIP. Intermediate source-watchdog misses remain historical; fixed functional2026-10-21T11:10:14Z/closeout13:10:14Z and engine062 stay unchanged. Native0 is preserved.

Exactly two new R4 reports, combined<=24576B, are written after precomputing UTF8 bytes in memory and a fresh full closed META-v8 census before each write. v8 cap10878976 and hash42fe5298f372601c0696017f16ec0e0281a00f223d5a6aabe54ed19836edb9ba retain the same24-harness/245974B physical membership and explicit Context-original C3 exclusion. Resource's current160957 pending consists of136381 other holds plus this24576 report allowance once. Actual report bytes consume that allowance; unused balance is reported for parent release once. No stale256K/472893 or archive reservation is revived.

