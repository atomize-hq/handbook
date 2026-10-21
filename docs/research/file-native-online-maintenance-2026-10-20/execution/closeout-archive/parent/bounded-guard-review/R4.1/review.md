# Independent full R4.1 source review

Verdict: REMEDIATE. No safe source admission is granted for a qualifying native27 run, fixed-SSH finite8 run, or HYST from this exact snapshot. The remaining shared allocation binding is required before qualifying those routes. HYST additionally has an incompatible closed authority schema. This is a source verdict, not a native result or a runtime release.

## Reviewed snapshot and limits

All five frozen source files were read in full as text, including unchanged controls, README and source receipt. Final hash recheck is recorded in evidence.json. No partial read is presented as a full review.

| File | Bytes | SHA256 |
|---|---:|---|
| job_guard_bounded.py | 77503 | 300a71f666c61c8301aa782df2c10412e6afa574891aa071ff897d96f940cfe2 |
| unlocked_capture.py | 58923 | 447f164837ad7a4b46b923f90a72c1568f536d96af29355a48edb421563d44ef |
| controls.py | 14902 | 24b88e544417d909cebdd5dd4e4cb993e35dcfa230b30d3f3059cb3238c2d9c5 |
| README.md | 4078 | dedc5c37b316af68095c00188e1ff5febc761f392e13c74168dcd6aecbbf326a |
| source-receipt.json | 3432 | 2116d8d84f81385ff5da6c26aac4193ed8310f182c7d96308d348d9d01120664 |

Total 158838 bytes / 5 files, within source budget 166912 / 5. Exact source-budget058 is 3834 bytes, SHA256 ea7e3478651f0dc53c8a04dd8d74e0a8d9b7073535eeb8c5d13e289e0ec967d1. R3/R4 negative history remains preserved; no retroactive pass.

Read-only dependency work included the complete current Windows bootstrap and Linux phase bootstrap, current HYST pre-Evidence budget association and postexit finalizer contracts, and verification of the unchanged reference guard digest. Context changed separately; its current 757798dccac67f7dab6c8150f5cf69df87b1c1bbb7623a957a32fef0d8be0136 is not admitted by this review. Current Linux controller V5.3, all38 Context controls/C31, engine, build, retirement and terminal qualification remain separate reviews/admissions.

The structured data format standard remains proposed. No defaults, wire migration or canonicalization changes are authorized by this report. GitNexus Landing registration is UNKNOWN/unavailable; manual lifecycle impact is HIGH. No indexing/bootstrap was attempted. No reviewed module import, AST/compile execution, native call, process workload, SSH, Job, control, engine, Cargo, Git, source edit, subagent or new chat was performed.

## P1 — Bind the actual reopened outer budget before acquiring resources

Location: unlocked_capture.py:379, 414–429, 477–480, 757–764, 964–971.

load_config validates the configured budget reference at171, but Capture constructs a new Budget from that pathname at379 and never compares Budget.raw_hash with c["budget"]["sha256"]. Bounds and preflight then use the independently reopened contents. The actual hash is available in Budget but is omitted from outer-owner.json, the stored outer receipt, and the durable outer caller completion; actual bounds_sha256 and config reference are omitted as well. Source hashes and run nonce do not establish the allocation actually used.

Concrete trigger: replacement/change between the reviewed-reference read and the Budget reopen can leave a still-valid budget with different categories/limits. A run can use those bytes and publish a qualified transport completion without exposing its actual allocation hash. Even without replacement, the current HYST consumer windows_adapter.py:574–592 requires outer["budget_sha256"] to equal the sealed outer reference before Evidence creation; this producer never emits that key. LaneHost invokes that check at61, before Evidence at65.

Required repair: compare the actual Budget.raw_hash against the configured reviewed hash immediately after construction, before any writer/native resource; retain the exact hash/actual bounds association in the outer owner and durable receipt/completion. Reuse the retained parsed budget for checks. Current HYST must join this actual owner to its sealed numeric inner/outer pair. Include the actual config reference if the receipt is to prove which reviewed mode, deadline, manifest and allocation were used. An external filename/hash assertion does not repair this internal producer association.

Scope: shared outer capture; native27 and fixedSSH may have lifecycle containment mechanisms ready for prospective controls, but this exact source cannot establish the requested internal actual-budget association for qualifying admission. HYST deterministically fails closed on its existing consumer. No claim that this gap itself bypasses native Job limits.

## P1 — Align finalizer allocation schema with the current consumer

Location: unlocked_capture.py:900–914; qualification-lanes/phase_finalizer.py:1375–1383.

Capture closes finalizer-allocation-v1 to schema/run/source_pin/budget_sha256/parent_case_id/census/output/limits/caps/guard_bounds_ref. The current finalizer closes the same object to those fields PLUS source_freeze and then validates the freeze. Both use exact closed-key validation.

No allocation object satisfies both contracts: an authority accepted by the finalizer is rejected by Capture before grant creation, and removing source_freeze to satisfy Capture makes the finalizer reject. This blocks actual old/new HYST finalization, not the controls or SSH-only route.

Required repair: agree on the current versioned allocation object, preserve and validate the actual source-freeze association, and review both producer and consumer together. Do not relax closed-key validation or drop the finalizer's source-freeze requirement to force an apparent pass. This incompatibility is from current dependency text, not an executed test.

## Reassessment of previous lifecycle findings

| Prior issue | R4.1 source assessment |
|---|---|
| R4 final denial accounting | Repair mechanism present in BOTH Jobs. Inner1291–1303 and outer738–747 call check_job_limits with the SAME still-held Job before checked last close, retain final snapshot/error, and censor health on denial/query failure while proceeding with cleanup. check_job_limits appends the snapshot before validating cumulative limit-terminated count. Native between-poll denial regression remains unrun and separately admitted. |
| R4 unresolved inner acquisition | Repair mechanism present. _ROOT_PENDING is set before Popen1146, cleared only after positive assignment/member proof1154, blocks next run912, participates in IO-unknown1240 and retained acquisition context1263, and prevents physical closure. Unknown acquisition is conservatively fenced; successful cleanup alone does not clear its unknown state. |
| R3 unassigned outer suspended root | Retained native-handle termination/reap is present. creating_root begins before Popen; unknown acquisition stays fenced. terminate_unassigned_root targets only the retained Popen handle; finish waits/reaps, verifies identity/signalled state and checks handle close. |
| R3 watchdog ends before publication | Whole watchdog starts before load_config and private imports/census; it remains active through publisher outputs, final census and stdout. Expiry makes qualification false; +12s cleanup cannot qualify. Unknown inner/outer acquisition suppresses unsafe hard exit and retains a fence. |
| R3 missing outer root-end/tail coverage | Three clock-domain gaps and raw witnesses are checked at terminal root and empty-Job observations, with <=40ms requirements; qualification requires both coverage records. |
| R3 grant accepts unqualified old outer | publish_parent_records/_issue require actual old outer qualification, no global fences, no expiry, held new Capture capability and actual suspended native child. This mechanism is present, though current authority-schema interoperability above still blocks HYST. |

These are static repair assessments only. A source review cannot turn any earlier negative receipt into a passing native run.

## Clock, resource and evidence audit

Initial whole-call maximum is3600s clipped to functional11:10:14Z. Verified config can only tighten. Controls tighten to<=900s before resource writers and their manifest shares the actual endpoint. The worker now clamps3600 rather than900; actual c.absolute_deadline still wins. Old+new HYST must be admitted with its complete endpoint up front: a subsequent config cannot extend the singleton's earlier endpoint. Closeout13:10:14Z and +12s nonqualifying cleanup remain unchanged.

Native limits remain flags8968 with inner12/outer16 active processes, per-process2GiB and Job3GiB commitment. Source checks/readback occur before resume, during polling, and now finally. The registry's16 and Job table2048 are storage capacities. At most4 creation-checked live supervisors and positive RSS remain required; outer Job+supervisors <=4GiB. Sequential128 remains cumulative and is not reduced to a concurrent128 interpretation.

Stored gzip retains hard prospective row/raw/stored-deflate caps, exact bytes/hash, no compression credit, bounded collections and final/emergency/completion reserves. Physical native closure is independent of health qualification. Stdio remains bounded prefix/tail with109 or actual zero read EOF;233 censors. Unknown native/pipe closure retains references and blocks further controller admission. All27 control IDs, order, injections, checks and separate pipe-fence reconciliation were read. No27th-case result or aggregate pass was executed.

## Fixed SSH route

The local executable bytes were hash verified read-only:1251328 bytes, SHA2568e1212d6b10bbab3dfd85077c1a7d0b60373fde702dfa9ea8c57924793bdb317. The source's nlink2 exception is confined to that exact system image and compares same-buffer identity/size/time/nlink plus hash; generic reference reads still require nlink1. No SSH process was launched.

The route fixes System32/OpenSSH/ssh.exe, alias spenser-linux-codex, -T/BatchMode/StrictHostKeyChecking/ConnectTimeout, remote /usr/bin/python3 -I -S -B, the exact final-suite-v5 bootstrap, safe parameter basename, shlex-quoted phase ABI and one Windows/native endpoint. Actual root image, PID/creation, local Job/RSS/IO/coverage and terminal references are retained. Bootstrap source8ce97085723d2657f9148bfa81bf3d70e67759c86872e30bd87b09805562a135 was read completely; --phase/--manifest/--sha256/--phase-deadline ABI matches.

This encloses local transport only. It does not prove remote process closure, approved native runtime, successful finite8 controls, source deployment, or terminal semantic qualification. Existing Linux V5.3 approval/actual native phase manifest/source closure and verifier remain mandatory. The mode's qualified flag is transport qualification; callers must verify native terminal evidence separately. No HISTORY grant route exists for native-ssh-phase. The shared actual-budget binding finding still prevents positive admission of this snapshot under the requested proof contract.

## Finite remediation and disposition

Repair the actual outer allocation association; align the current HYST authority source_freeze contract; freeze a new exact source snapshot and obtain the bounded review for affected changes. Then admit native27, separate between-poll denial regression and fixedSSH finite8 with all sinks reserved and fresh source/config/budget hashes. Cache/build/body/Context retirement/HYST grants remain separate and unrun.

Only the two allocated review artifacts are written. META uses v9 closed physical membership SHA256842fb0c2f51715ad4af257738df9dbabb6e97f74659aeccad0795f012ee49386, cap10878976. Other pending136381 and this pair's24576 are counted once. Fresh complete physical census is taken before each write; exact consumption, one-time unused release and final source recheck accompany the report delivery.
