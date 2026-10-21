# Independent Windows final-platform source review

**Bounded source disposition: REMEDIATE. No memory/native-control GO. Full qualification: INCOMPLETE. All166real cases: PENDING. Runtime: NO_GO/HOLD.**

Four required source findings remain. These are static causal findings and proposed proofs, not executed controls or native reproductions. Missing final engine/schema/hooks/all-body admission remain honest pending dependencies; they are not mislabeled source defects or waived qualification.

## Required findings, severity first

### W1 — P1: an uninitialized tail deadline escapes finally before raw retention and resource closure

`windows_driver.py:288-305`, specifically292-293; downstream cleanup/raw completion/done at306-407.

The variable `until` is initialized only in the exception handler for `p.wait(timeout=5)`. After a successful reap, the following while condition nevertheless reads it whenever stdout/stderr pipes are not already EOF/closed. A start-callback error/timeout before the I/O loop is a direct causal path: the outer except terminates the owned child; the finally reap succeeds; both output pipes still need draining; evaluating the condition raises UnboundLocalError inside finally. Pipe/event/native/Popen handle closure, timer/callback joins, raw-complete/native-call retention and done signaling below that line are skipped. The original failure is masked, and the registry entry remains unfinished. The accepted outer Job's eventual kill does not reconstruct those private pipe buffers or produce the missing per-call evidence.

**Fix:** establish a finite fallback-tail deadline unconditionally before using it. Structure finalization so tail/reap/sink failures are recorded independently and cannot bypass later resource closures or final registry completion/uncertain-closure publication. Preserve the first error and every secondary error. Do not turn unobserved tail/EOF into success.

**Required proof after source remediation/review:** callback exception and timeout before stdin; read/write/RSS/ready failure with live pipes; child successfully reaped but unread stdout/stderr; reap timeout. Every path must attempt complete raw retention and close/join every owned resource, retaining uncertain closure explicitly. Existing43prepared memory checks do not execute _execute and cannot discharge this defect.

### W2 — P1: HANDLE termination and close do not share a lifetime lock; setup fallback can reuse a closed numeric HANDLE

`windows_native.py:147-160,279-293`; caller `windows_driver.py:28-37,66-76,332-346`.

Native.terminate holds native.lock while validating the retained HANDLE, creation time and current Job, then calling TerminateProcess. Native.close_identity does not take that lock. The call-owner cleanup also closes the Popen HANDLE and publishes resources_closed without the Driver lock used by interrupt/_terminate. A concurrently authorized close_owned/interrupt path can therefore validate a HANDLE while the owner closes it. In the natural-exit race, closure can occur after the terminator's final identity check and before TerminateProcess; numeric HANDLE reuse by another concurrent call would defeat the otherwise sound retained-identity proof. Timer cancellation before close does not serialize external registry-control callers.

There is a separate concrete stale fallback: if capture fails, native identity is absent, the finally block can reap and close Popen._handle, but resources_closed remains false because it requires a native identity. Later close_owned selects that entry, and DriverSupport._terminate's fallback calls TerminateProcess on int(p._handle) without checking that the HANDLE is still open or that the process is already reaped. An invalid or reused numeric value is not an owned live-process proof.

**Fix:** serialize termination, native identity closure, Popen-HANDLE closure and terminal ownership state under a consistent lifetime/lock protocol. Retire each termination capability before releasing its HANDLE; never use a closed/reaped Popen fallback. Preserve the ability to close an owned suspended child after partial capture failure without acquiring new authority by PID/name. Native process/I/O/callback closure remain distinct facts.

**Required proof:** controlled close-vs-interrupt and close-vs-close_owned interleavings, including natural exit between identity verification and termination, plus capture failure followed by repeated drain. A closed or reused handle must never reach a native terminate call. Future admitted benign native controls must exercise actual retained HANDLE semantics; fake identity equality alone is insufficient. No such control was run here.

### W3 — P2: partial setup acquisitions escape the owner's registry/cleanup boundary

`windows_driver.py:122-135,176-179,306-346`.

The three pipes are constructed on the RHS of one list assignment. If the second or third construction raises, assignment never happens: the already-created Pipe objects are absent from `pipes`, so finally cannot close their server/client/event handles. Pipe has no destructor/registration that rescues these successfully acquired earlier objects. The failed constructor's own cleanup does not close the preceding constructors' resources.

Also, the call's evidence reservation and registry entry are installed before the actual image hash at135, but that hash/need is outside the try/finally. A hash read/refusal leaves a pending evidence reservation and a done-unset registry entry even though no child started. That entry cannot be reconciled by the normal owner finalization and prevents evidence close/drain.

**Fix:** enter the call-owner failure/finalization boundary before registration/reservation and all subsequent fallible setup. Acquire/register pipes one at a time (or use equivalent explicit resource ownership) so every completed acquisition is visible to cleanup. Record a no-child setup failure with settled evidence/registry state instead of inventing native closure. Combine with W1's cleanup isolation; simply initializing until does not fix these acquisitions.

**Required proof:** fail first/second/third pipe acquisition, event/client creation, executable hash verification and Popen/capture/resume. Check every earlier acquired handle/fd/event, every registry/evidence reservation and the preserved primary/secondary rows. No leaked resource may disappear merely because its enclosing Python list assignment failed.

### W4 — P2: native reader-end methods do not implement the existing shared harness contract

`windows_adapter.py:250-265`; shared `focused_schedules.py:436-469`, particularly460-466, from the reviewed focused-protocol handoff.

The shared cancel/death schedule calls adapter.interrupt, then immediately requires identity_verified=true and closed=true. NativeAdapter.interrupt returns WindowsDriver.interrupt's termination-request-only object, which has neither field and does not wait for closure. These cases therefore fail even when termination succeeds. The later finish_interrupted hook cannot repair the already-failed assertion.

The error case is also incompatible: the shared schedule injects an ordinary exit2 error, waits for it, then always invokes finish_interrupted(handle,'error') in finally. That adapter method requires row.intentional_interrupt=='error', while the ordinary injected-error row has intentional_interrupt=None and Driver.interrupt supports only cancel/death. Thus the error schedule also always fails this adapter check.

**Fix:** implement the unchanged shared boundary explicitly: cancel/death interrupt must return a source-bound identity/positive-closure proof only after the single call owner has drained/reaped/closed; finish must be safe/idempotent when invoked afterward. Handle the ordinary injected-error completion separately using its exact exit/error/raw identity, without fabricating an intentional interrupt or semantic noncommit. Retain technical violations as failures. Alternatively, a deliberately reviewed shared-interface change would need its own bounded source revision; silently changing the frozen caller contract is not selected here.

**Required proof:** source-signature contract traces for error/cancel/death, including callback/raw/EOF/closure errors and repeated finish. Verify no false commit fence and no early positive closure. Real native pin release/protected-owner proof remains separately required.

## Complete inspection and supported boundaries

Read all11Python modules (121436bytes), all8case-data maps (59371bytes), and all8binding/document/receipt files (41839bytes):27flat files/222646bytes. Read the entire current accepted QPC guard50ecc source and its child-assignment, owner, deadline/sampling, Job-drain/two-empty and post-output receipt paths. Rechecked the known7ce shared Harness closure/transport contract and reader-end schedule. No target module was imported or compiled; no prepared control/native/engine process was executed.

The intended safeguards are concrete but remain unqualified:

- CREATE_SUSPENDED without breakaway; positive child HANDLE/creation/image/RSS proof before resume/stdin; current-Job PID membership includes the retained creation-bound stage and caller, rather than IsProcessInJob(anyJob). No new Job is created by the platform. W2 concerns retaining that proof through close/termination races.
- One call owner submits/completes/cancels the three overlapped pipes and reaps its Popen. Completed native writes and actual stdin closure are tracked separately from child consumption; env/on_started are explicit transport parameters. Positive-start callback work is bounded and separately joined. W1/W3 prevent accepting current exceptional-path ownership as complete.
- Lossless raw chunks and raw-complete metadata precede strict UTF8/JSON parsing on paths that reach finalization. Per-call AssertionError rows bind exact invocation data, and RSS/time/closure violations cannot be recovered as semantic conflicts. Raw output beyond bounds is a technical censor, not a fake reply. The outer QPC is a separate final authority and cannot repair missing private transport bytes.
- Growth anchors nonoverlapping roots once, retains concurrent/settled increments and failure output, and performs full censuses at declared quiescent checkpoints. All-sink input/env/output/tail/RSS/helper/oracle/guard/finalization components are required. No planned deletion, clone/link, gzip ratio or forecast expiry credit is introduced. Full-body E_total and Windows topology remain NULL/NO_GO.
- Bootstrap requires python -I -S -B, a byte-bound complete source/release manifest and a reserved nonce/PID startup gate before executing the corpus-argument script. Import/hook sources are hash-bound; absent final hooks refuse. Dynamic final-source policy implementations and exact source/schema maps remain pending, not assumed accepted by this review.
- OracleBridge invokes a separate guarded checker through LaneHost.invoke; complete before/after file/directory hashes must match. The ordinary engine-audit path cannot provide its required output-file/full-independent proof. Required physical/outcome/intent/membership/graph/source/navigation/provenance/retirement/B/read-only checks and reverse cohort closure are retained. No parser fork or wildcard pin-lock exclusion is selected.
- Historical grants bind distinct actual executable/source, exact case/root/action/input/env/role/exit and finite count on an owned derivative. Protected operational sources refuse. W2 must hold equally for those shared-registry calls.
- All166case IDs are unique and PENDING. Constructors/version/checkpoint/donor/oracle bindings and actual final unit catalog remain NULL. Five legacy unit slots do not cap19+actual final units; no filtered/Unsupported case is promoted to pass. The explicitly disclosed HISTORY40-call adequacy and inside-host final-QPC-receipt issues remain separate pending helper integration; this adapter does not falsely relax them.

The existing43literal memory-control sites exercise strict-data helpers, lease math, envelopes, fake token ownership and signatures/pending hooks. They do not cover native overlapped setup/drain/EOF or the reader-end caller/adapter contract. No finite memory/native-control approval follows from their presence or from the owner's prior syntax-only result. Remediate and freeze replacement source, review the full diff/ownership proof, then obtain the separately bounded parent decision for any controls.

## Identity, scope, accounting and clocks

All27actual file hashes/sizes match frozen source-receipt.json and were rechecked unchanged at static completion. Receipt SHA256 `b4671f65fe56b6cf964aa850c843fac49fa36c0fe4050891a5a302dc8ab76ec6`. Independently recomputed canonical26file manifest SHA256 `3e3505e2cba61387327f802deae11d6885ba0996410f4cdcc97dc25cee8fd3c3`. Accepted QPC guard SHA256 `50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378`; shared core SHA256 `7ce0a4a06a35fa9e7f4f4080a40315a418bb45e47a8946ca06d78c2117d855e8` independently match actual source. No changed final engine/schema pin is inferred.

Prior controlling-document and proposed structured-data-standard review remains inherited. The new private JSON maps do not adopt a format migration/default or product authority. GitNexus remains exact-checkout UNKNOWN/unregistered/manualHIGH; no index/bootstrap or graph claim. No engine/build/native/Job/oracle/import/control/compile/Git/source edit/subagent/chat/fixture action occurred. Only this report and evidence.json are written in the authorized parent destination; old platform source stays preserved. Meta alone receives notices, including explicit static-completion/source-freeze release.

Before report writes, independently recounted the full canonical META8 membership from P/meta-membership.json SHA256 `a3d26618a7d0e1facaef914831be1be8f87eab7747d56292e8e8afe26fa8b106`: all P recursive files including archives/extracts; regular files at Landing research/execution roots; listed recursive execution subtrees excluding stream-snapshot-controls; and all20promoted harness files/203215bytes. Original helper/windows/native-linux sources are excluded as specified, not charged twice into META. At2026-10-21T01:19:39Z the complete total is7881876bytes/929files (P5512881bytes/683files). Adding the full32768byte report allowance and preserving262144bytes future reserve gives8176788bytes,211820below8388608. No planned deletion/reservation expiry is credited.

Original claim025 source registration and cumulative44GiB/5m-file/internal reserve remain unchanged. Functional2026-10-21T11:10:14Z, source-review target04:10:14Z and protected closeout13:10:14Z do not reset. **REMEDIATE is the bounded source decision; the real platform/full experiment remains unqualified and runtime NO_GO.**
