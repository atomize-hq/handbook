# R3 bounded Windows guard and unlocked caller: full source review

**REMEDIATE. Runtime/control/engine GO remains FALSE.** Four blocking findings apply to the exact five-file freeze supplied at2026-10-21T05:42:24Z. All1454 guard lines,296 control lines and939 unlocked-capture lines were independently read, together with README, receipt,050 source budget, structured-format standard and the relevant bootstrap/Context/API integration paths. No reviewed module was imported, parsed as AST, compiled or executed. No native API/control/Job/engine/Cargo/Git action or source edit occurred.

The five originals total144,563B/5 under147,456B/5; each fits its050 cap. Exact hashes and byte sizes are in evidence.json. Donor/reference identities and prior negative reviews remain historical inputs, not retrospectively rescored. Private guard imports are stdlib/Windows DLLs. controls compiles the same bounded hash-verified buffer; unlocked_capture likewise loads the exact guard buffer. Its fixed bootstrap and Context routes depend on separately reviewed sources/releases; this review does not approve those entire changing source trees.

## Blocking findings

### R3-1 [P1] A pre-assignment failure can leave an uncontained suspended root

unlocked_capture399 creates the suspended Popen process. Creation/image checks401-403 precede Job assignment404-405. If either image validation or assignment fails,417-419 calls fail()/finish(). Both terminate()448-456 and finish578-581 terminate only the new Job, which does not contain that child when assignment failed. finish then waits for a suspended process that has no termination path. The hard watchdog may exit the publisher and close an empty Job while the unassigned root remains alive.

This is particularly relevant to the required nested-Job-assignment negative. Track successful assignment and provide checked termination/reap/close of the source-owned retained root on every failure before containment, or use an atomic containment mechanism. Do not reopen an arbitrary PID or declare closure from an empty unrelated Job. Retain uncertainty when cleanup cannot be positively established. The inner guard's assignment-failure path1166-1168 already explicitly kills and waits for its owned root; the outer caller lacks that protection.

### R3-2 [P1] Whole-call deadline protection ends before final output

The Capture watchdog starts at309 and its termination timer at394, but finish588-591 cancels both before receipt/fsync653-656, census659, OWNER674, publisher records697-779, grant writes783-831, and final census/stdout930-934. These synchronous operations have no remaining watchdog/deadline enforcement. Even the closeout cutoff is not checked on those paths. Thus a delayed native file operation or census can continue beyond the admitted final deadline without the promised hard refusal.

load_config106-190 is also evaluated before Capture's watchdog in main924; it validates then reads/hashes several inputs before that protection exists. Keep an admitted absolute whole-call watchdog or actual bounded native-parent supervision from initial dispatch through final durable output/census and stdout completion. Any separate closure grace must be explicit, bounded and nonqualifying on expiry. Do not restart the same phase clock after setup/native close. A hard124 expiry remains failure, never successful resource closure.

### R3-3 [P1] Outer RSS coverage has no root/Job terminal check

sample458-498 bounds each row's collection/write duration and recorded-to-recorded gap. The main loop stops when the root exits. finish569-651 never checks the final RSS gap to actual root exit or whole-Job drain, unlike the inner guard's coverage1007-1029/1216-1225.

For example, a bootstrap gate read/write/fsync at562 can delay the outer sampler after its last row while the worker completes. The next poll sees exit; finish can set qualified at660 without detecting a >40ms terminal coverage gap. Descendants are also drained after the main sampler stops. max_interval_ns measures only gaps that had another sample.

Record and enforce source-owned root-end and whole-Job-tail coverage before raw closure, with actual QPC observations and bounded raw markers. Preserve the same40ms requirement across start/completion/recorded timing domains; the outer currently checks only the recorded interval. A late or missing terminal witness must censor qualification while preserving independent physical closure.

### R3-4 [P1] Held-capability grant API accepts a failed outer capture

publish_parent_records726-735 requires a physically closed guarded-bootstrap Capture and an inner qualified completion/exit0. It does not require the outer cap.result['qualified']. _issue_postexit_grant783-787 similarly checks previous.status/parent_records, not the actual outer qualification or unresolved publisher fences.

A late outer sample can fail its40ms check after the inner worker has already completed successfully. Outer cleanup may then prove physical closure with worker exit0 and a valid inner report while outer qualified remains false. The public API can nevertheless publish parent records and issue a grant. CLI main926 guards its one-shot publication, but the held-old/prepared-new grant workflow uses these API functions.

Require the actual old outer capture's strict health/evidence/census qualification and absence of unresolved ownership/publication fences at both production and grant gates. Keep checked physical old/new native capabilities as an additional requirement. A closed resource state must not stand in for a healthy qualified phase.

## R2 contradiction and intended controls

The R2 qualification withdrawal stays valid negative history. R3 guard1343 now exports process_closure_verified=proven1318 independently of errors, while qualified1372 still requires exit0, no censor/errors, physical closure, complete gzip/stdio and successful postwrite census. This is the necessary prospective separation: a faithful missing-live-rss/census-error/error-storm run can now retain its health errors while independently proving resource closure for controls243. No undersized error cap, omitted case, changed fault, retry, cherrypick or global-PASS reinterpretation is required by that repair.

All27 intended names remain ordered. Controls now checks an absolute<=900s/fixed-functional deadline and starts its own watchdog before private import/case census. The nonempty private-buffer Event, later-admission refusal and separate native reconciliation remain present. The old CENSOR is preserved. Failed/NOT_STARTED cases still prevent CONTROLS_PASSED. Native behavior, actual expected censors, injection applicability, all-sink forecast and retained negative proof remain UNRUN and need separate finite admission. This source review supplies no native PASS.

## Native ownership, Context and producer observations

Source positively supplies exact retained creation/Job checks, suspended positive RSS, native image inspection for the outer root, checked64-bit ABI, stored-DEFLATE accounting, bounded stdio, two actual numeric empty observations and checked native root/Job closes. The outer lane is a different byte lock; it never takes the inner heavy token. Context receives a freshly written immutable manifest containing the actual publisher PID/creation/native image/argv; its worker owns the local heavy lease in that same process.

The inspected Context4350d9cd source still explicitly refuses CONTROL pending constructor/native fault schedules. That is known pending work, not proof of38 controls or archive/RETIRE admission. RETIRE requires both explicit real_retire_go and the complete independent Context permit. No such GO follows here.

The new worker-return, native_lifetime, parent-completion and held-new-parent grant functions are concrete producers, not just schema declarations. They bind inner receipt/return SHA, actual reaped guard-worker identity and positive native handle close, inner and outer Job lifetime, source-owner audits and the separately prepared suspended parent. The four findings above must be fixed before these producers are accepted. Their actual consumer/source-schema compatibility and finalizer's later native closure remain separately pending.

GUARD_WIRE_CONTRACT and CAPTURE_WIRE_CONTRACT are distinct import-free literals. The inner literal includes source-owned dimensions, nanosecond/FILETIME widths, required RSS creation identity, markers, stored-block costs and publication/lifetime schemas. This review read those literals as text; it did not execute extraction. New producer fields do not authorize omission of old identity, exact byte-domain or scope checks in consumers.

## Separate assessment: proposed live limits and semantic rosters

Frozen R3 does not enforce12 inner or16 outer live processes. Both Jobs set flags8192|256|512 and leave Basic.Active unset (guard1154-1159; capture386-391). The2048 member buffer/overflow check is a query capacity; it is not a12/16 admission proof. The registry still accepts16 generic supervisor rows. Forecasters cannot simply replace2048/16 with12/4.

For a prospective revision, set and positively establish JOB_OBJECT_LIMIT_ACTIVE_PROCESS with the intended limit before assigning/resuming children in both Jobs. Preserve no-breakaway/noninheritance, handle ownership and explicit excess/assignment failure cleanup. Bind actual enforced live limits separately from buffer capacity and cumulative identity allowances in the source literal, runtime receipts and forecast. [Microsoft documents the active-process flag and limit](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information).

The controls' static process tree peaks at two inner processes (test root plus one nested child), and three outer processes (controls worker plus those two). sequential-128 is cumulative identity churn, not128 simultaneous children; shrinking its cumulative identity allowance would change the required control.

Guarded-bootstrap adds its guard-worker outside the inner Job. A benchmark roster must include the coordinator/bootstrap, all four concurrent reader actors, publishing/maintenance/native-oracle processes and any nested children. The inspected driver still spawns calls through Popen; no full semantic12-process roster/enforced publisher concurrency bound was established by these five files. An active cap can bound resources by refusing excess children, but does not prove that it accommodates every required workload without changing the experiment.

Outer counts include nested descendants; do not add the same process twice as two different live slots. [Microsoft's nested-Job contract](https://learn.microsoft.com/en-us/windows/win32/procthread/nested-jobs) describes subset membership and descendant accounting. Native qualification must verify the intended nested behavior on this host.

A proposed four-supervisor limit must count actual creation-checked registered identities and explicitly account for publisher, native parent, current guard worker and any admitted fourth role. Current generic16 rows/extra PIDs do not prove that roster. Respect live foreign owners; an inability to fit must refuse, not discard a supervisor or mask RSS. The all-scope peak must include outer-owned processes and the separately measured publisher/supervisor set, with documented overlap, actual stage evidence and no deletion/compression credit. The current source checks outer RSS<=3GiB and outer RSS+registered supervisors<=4GiB; shrinking a count forecast does not strengthen that physical proof.

## Authority, preservation, accounting and handoff

The structured-format standard was reread and remains proposed. No format migration/default, engine062 change, old receipt requalification or retirement authority is granted. GitNexus remains UNKNOWN/unregistered from the source receipt; manual impact HIGH. No source symbols were edited or index bootstrapped.

All five exact hashes were checked unchanged after complete source reading, then the read freeze was released to the coordinator for prospective R4. This verdict applies to those frozen R3 hashes, not any successor. Preserve R2 withdrawal/R3 failures and native0; review a successor only after a new freeze. Functional2026-10-21T11:10:14Z and closeout13:10:14Z stay fixed; intermediate missed source targets are not retrospective approvals or clock resets.

Only review.md and evidence.json under the new R3 report directory are written, combined<=24576B. Their exact UTF8 bytes are precomputed in memory. A fresh whole closed META10.25-v7 census precedes each write; the same physical membership includes all P, roots, seven subtrees and24 promoted harness files/245974B. The original Context/source allocation is C3-excluded; distinct copies in P count. Allocation046 establishes10747904B; v7 documents rolling pending136381B plus the new R3 report allowance, counted once. No obsolete generic256000/472893 reserve or deletion/expiry/relabel credit is revived.

