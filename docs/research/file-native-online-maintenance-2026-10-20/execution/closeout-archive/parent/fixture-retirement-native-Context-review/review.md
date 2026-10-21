# Windows native Context: full frozen-source review

**REMEDIATE. No native/control/archive/retirement/engine RUN GO.**

Fully read the ENTIRE windows_context.py freeze:92445 bytes, SHA256757798dccac67f7dab6c8150f5cf69df87b1c1bbb7623a957a32fef0d8be0136. This is the new056 source, not the40916-byte WIP or48752-byte first repair. Budget056 is2650 bytes/SHA256cd270e27c6145f049d35b61fe486090f66d9159ef32549ebb0a5c2032e454381, one98304-byte file. Exact hash/membership/cap and report accounting are in evidence.json.

C = C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/fixture-retirement-native-context/windows_context.py.
E is C's execution ancestor. A = E/fixture-retirement/archive.py; T = E/fixture-retirement/controls.py; G = E/windows-bounded-guard/unlocked_capture.py; D = sibling job_guard_bounded.py. Line references below use these actual source files.

A51047/e99cf2ac and T21121/76e5a487 were previously read in full for ArchiveR3 and rehashed unchanged for this review, as expressly allowed. Relevant actual G/D source paths were read for parent/argv generation, heavy token, child reservations and native closure/proof production. This is not a full new G audit. The reviewed G snapshot is53247/bf44e89c; D77521/7b5d5885. No reviewed import, AST/compile, native API/control, engine, archive/restore/delete, Git/index operation or source edit was performed. No agents/chats were created. GitNexus remains UNKNOWN; the structured-data standard remains PROPOSED.

The two source findings below concern affirmative admission BEFORE writes. They do not assert an executed cap breach, successful archive of protected data, false native PASS or real retirement. Keep them separate from the explicitly pending C31 and REAL adapters and the acknowledged G reference-size mismatch.

## C1 [P1] Budget admission does not require owned membership or fund the proposed writes

C:758-785 accepts root_caps=[] and never requires coverage of limits.control_root/evidence_root, unique/disjoint roots, or source-owned mandatory categories. Even with rows present, zero reserve_bytes/reserve_files is allowed. The only use of prospective_bytes/prospective_files is an integer check against limits.body_bytes/body_files; they are not charged to an owning root or checked against that root's remaining reservation.

Consequently admit_controls:843-864 can return go=true for T's whole12MiB/384-file forecast when there are no owning rows, or when an empty control-root row has zero capacity/reservation. before_control_write:866-884 accumulates the per-case numbers against the separate limits, then calls the same incomplete check. In the zero-cap empty-root example, initial actual=0 and ticket high-water=0 satisfy every root inequality before any write.

T.run then creates the batch directory and case prefix, and prepared writes sample.bin and toy-control-review.json. Only the later constructor's control_write(native-scope.json) calls check_budget again and/or detects control-sink-no-owner at C:889-908. Detecting/refusing after those direct writes is not prospective admission. This is a concrete static path through the real closed APIs; no malformed code execution or unreviewed callback is needed.

C.check:655-702 also returns positive per-archive/Sink admission without invoking check_budget. It checks the request against the plan's maximum and updates charged_archive, but that value never finances a root reservation. Thus it does not repair the initial membership/escrow gap. G.child_forecast:216-227 forwards only the supplied nonzero reservation rows; it also does not derive missing Context roots or validate their forecast minima. G's eventual disk census is not a local prewrite escrow.

The high-water implementation itself is useful: for a declared funded root, ticket maxima retain actual peak and prevent later deletion from restoring phase credit. The defect is the absence of required ownership/funding and prospective settlement, not a request to subtract deleted data or add a generic reserve twice.

Required source repair: validate the complete admitted root partition against the exact Context authority before affirmative batch admission; require the selected body/evidence/control sinks and their full finite forecast to be funded. Validate root shapes/uniqueness/overlap and bind per-case/record/partial/archive output to the relevant ticket BEFORE every direct producer write. Either prehold a conservative whole-phase ticket and consume it correctly, or debit bounded suballocations before writes. Keep all external valid holds and physical copies; do not replace explicit funding with a later census or expanded cap.

Proposed finite negatives: empty/missing root membership; a zero-cap/zero-reserve control root with positive limits; enough aggregate capacity under the wrong root; insufficient remaining bytes/files before the first fixture write; missing evidence ownership; overlapping/duplicate roots. These were not executed.

## C2 [P1] Global protected output scope is checked after initial fixture creation

C.__init__:373-379 pins the existing parents of the configured control/evidence outputs; validate_protected:517-530 pins the declared protected native roots. This establishes no-follow ancestry and actual identities, but there is no comparison between those two sets before activation.

admit_controls and before_control_write validate the configured textual paths and current lease, not whether the new batch/case/source prefixes lie inside or alias protected native scope. T.run:256-268 creates root/case directories, then prepared:113-140 writes the source fixture and review. C.prepare_control_plan:797-840 runs only afterward. It checks root identity/parent mount and absence of the case's synthetic protected path, but still does not exclude the global protected native prefixes before those earlier writes.

The global exclusion occurs later in C.native_scope:549-555, when A first traverses the source. A native canonical protected directory can therefore be chosen as the existing output parent, with a fresh nonexistent child as control_root: the initial new directories/sample/review are written inside protected scope before archive traversal rejects protected-alias. This does not require reparse following, a hostile actor, or modifying an existing protected file. It is an ordering gap in the cooperating source's own write path.

Required repair: at batch admission, bind the planned nonexistent output prefixes to retained native parent/ancestor identities and canonical volume paths, and reject containment/alias overlap with protected roots BEFORE any mkdir or fixture/control/evidence write. Retain/check the governing namespace/lease through creation. The deliberate C26 synthetic protected-root negative remains a CONTROL-only test; it must not weaken the global exclusion. Add a fresh-output-under-protected-parent negative that proves zero writes before refusal. C31's independent-prefix test at1269-1275 also rejects fixture-under-batch but allows batch-under-fixture; require bidirectional disjointness before a new batch can write inside a preserved fault/recovery prefix.

## Native ownership and source activation inspected

GuardianLease:292-327 opens the exact windows-heavy.lock through Pin, requires its admitted file identity, single link and nonempty regular-file shape, then calls LockFileEx on byte offset0/length1 using a retained read/write native handle. It loops only on lock contention under the deadline and records acquired_perf_ns. check verifies the same live object and no uncertain close; release marks ownership retired before one checked UnlockFileEx/close attempt, retaining uncertain capabilities in KEEP rather than retrying. D's heavy token uses the same named file, seek0 and one-byte msvcrt lock. G holds its separate outer lane rather than holding the heavy token on the child's behalf.

verify_parent:443-462 obtains actual direct parent PID through the native process query, opens a retained process handle, compares PID/creation/liveness/image, hashes pinned image/source bytes and checks actual native argv containing the bound capture source. G:425-438 constructs those parent fields from its own native identity/argv and publishes the actual child manifest; G:346-350 launches exact Python with -I -S -B -u and the fixed Context path. Assignment to the outer Job and native containment observation precede resume in G. Context requires the parent outside its current immediate Job and later members()=={self}. No owner-JSON held flag supplies the local heavy lease.

Pin retains no-follow native ancestry/leaf handles and refuses reparse ancestors; read_ref hashes and optionally decodes the same retained regular file, verifies its identity/size/streams before and after, bounds reference count/buffered RAM/native handles, and rejects pathname rebinding. verify_writers reads the closed cooperating port catalogue, source-function AST identities and revoked last-run grants against prior native closure records. CONTROL permits an empty external writer-port list because it owns the fresh toy namespace under the heavy lease. These checks are not hostile-user ACL isolation and do not repair C1/C2.

## Native data/mount/alias and DELETE boundary

native_scope:532-569 checks the actual supplied node handle, parent handle and freshly pinned root native identity. Native.meta derives normalized volume-GUID path, FileIdInfo, NTFS volume/mount identity, attributes/link count/size/stamps. Parent ancestry must remain a directory without reparse and all mount IDs agree. Canonical paths and native IDs exclude global protected aliases. Nondirectory/reparse hardlinks require complete native FindFirst/NextFileName enumeration and reopened alias identity/mount agreement; unknown/incomplete enumeration refuses. Temporary top/parent/alias handles have checked closure.

The old blanket leaf-reparse rejection is removed for forensic capture; the source records reparse metadata without following its payload target. Pin still rejects reparse ancestry. FileStreamInfo refuses nondefault ADS, except the controlled C18 fault is specially budgeted until that expected refusal. Core A preserves special metadata/regular alias bytes and refuses unsupported absence/topology; no generic extraction was added.

A remains the same accepted037 implementation: full source/census/strict archive/rewalk and all-alias proof precede any retirement. The exact DELETE-capable file handle is opened before byte verification and retained through disposition/close and observed absence. Context does not replace this with a pathname reopen or data copy. Its protected/mount proof is freshly checked at the deletion handle.

## Namespace-comparison limitation

native_scope/aliases:544-591 uses Python casefold for prefix comparisons and slices an original alias by len(folded_base). Unicode casefold can expand characters, so that index need not select the correct original path suffix. NTFS also supports per-directory case-sensitive lookup; a folded string is not a native ancestry identity proof. See Microsoft's [per-directory case-sensitivity contract](https://learn.microsoft.com/en-us/windows/wsl/case-sensitivity) and [ordinal comparison API](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-comparestringordinal).

The owner declared current CONTROL prefixes ASCII; REAL is explicitly pending. Reopened alias IDs, complete enumeration and duplicate detection are useful additional refusals. I have not established a current CONTROL false acceptance through this seam, and no Unicode/case-sensitive fixture ran. Do not claim Python casefold models all NTFS names or use a folded length to slice original UTF16/path text. Before admitting broader names/case flags, require native component/ancestor binding with the selected lookup semantics or refuse that unsupported scope. An ordinal comparison call alone does not establish containment or override per-directory policy.

## CONTROL constructor, callbacks and real effects

prepare_control_plan now exists. It checks the exact source/guard/provider/pin/root/nonce and emits a bounded actual native-scope record with root/parent identities, protected synthetic-path absence, lease timestamp, process/Job members and authority references. It changes only evidence.quiescence. T's canonical whole-plan equality enforces that restriction, including bool/int distinctions. plan:613-646 hashes the actual closed capture/retirement plan, compares it with the stored template and permits only the declared C26/C28 and tiny-retirement transformations.

CONTROL retirement is explicitly restricted at C:694-698 and plan:626-630 to C29 and the source-owned native cases with control_only=true. REAL still requires action=RETIRE/real_retire_go and immediately encounters the pending qualification adapter. Thus adding toy C29 retirement does not by itself authorize a REAL root.

native_actor uses fixed source-owned writer/pipe bodies; no arbitrary executable/argv/plugin/callback is admitted. C23 observes a native file write during the verification interval while the actor remains live; C27 observes pending synchronous pipe I/O, then CancelSynchronousIo and a joined thread/checked native handle close. check refuses while those actors are active. Closing uncertainty is retained and cannot become a healthy receipt.

The other implemented fixed modes actually create an ADS, linked ancestor, sharing blocker, protected regular alias or external hardlink to a reparse object, require the selected native condition and then the source refusal. C30 observes an actual native absence after at least one toy unlink, records the partial journal and preserves archive/source remainder. C35 supplies a false-admission Context with an archive canary and verifies it never executes; C36 hashes a retained review buffer, changes the pathname content and verifies the later helper refuses the changed SHA. This C36 coverage does not inject a race inside A.evidence's internal hash/decode call; the core's same-buffer implementation was inspected directly.

After each native mode, the producer checks Job members, no active actors/uncertain handles, adjusted native handle-count balance and unchanged protected-root native metadata. Raw receipts are bounded, hash-referenced and returned with the exact eight-field T.native_case envelope. T retains all38 IDs and all eleven native-hook modes; source declarations are not passing runs.

C17 now performs the actual selected CreateNamedPipeW probe on the ordinary local disk path, requires INVALID_HANDLE_VALUE/error123, compares bounded source snapshots and the retained backup hash, and emits the exact closed policy2 raw record consumed by T. It returns WINDOWS_UNSUPPORTED_PRIMITIVE/passed=false. T alone marks applicability_verified after pinned same-buffer validation and fresh checks. A future successful batch is exactly38 passes OR37 plus that one verified unsupported C17, never38PASS for the latter.

## C31 and REAL: honest pending prerequisites

The dedicated C31_FAULT/RECOVERY/BATCH port is closed and source-owned. Fault phase creates the independent toy fixture, captures/verifies a backup, starts retirement, fsyncs an intent journal, observes the first actual native removal, publishes the exact checkpoint and waits for the OUTER guard to terminate the process. It does not substitute an in-process exception or normal return for the crash.

Recovery obtains a fresh local lease after fault closure, binds checkpoint/plan/scope/archive/journal and native source/process identity, tries the old permit against the partial prefix and requires refusal without changing remainder/backup/journal. Batch consumes both runs, checks actual host/Job/IO closure and timing, and emits no resume/expiry/real-retirement credit.

c31_closure:1340-1359 explicitly requires raw.c31_native_kill from the fixed reviewed outer G: successful TerminateJobObject, live target PID/creation and singleton Job members, exact checkpoint ref, exit3121 and ordered native kill/exit timestamps. Fault qualified=false is preserved; recovery must complete healthily in its own native process. The current reviewed G has no c31_native_kill producer. This is EXPLICIT PENDING work, as declared by the owner, not evidence that C31 has passed or that all38 can now qualify. Do not fabricate the missing observation fields or recast timeout/self-return as a crash proof.

qualification:705-710 explicitly refuses pending the literal Windows166/unit/oracle/negative/raw adapter. The former invented >=166 aggregate is gone. REAL archive/retirement is therefore unavailable in this source. This is an intentional gate, not a new false-PASS finding.

## Known G integration blocker

Reviewed G:139 calls reference(context,65536,False), which rejects the actual92445-byte Context before child activation. Parent already identified a separately bounded R4.1 ref-cap98304 repair. Record this as an integration blocker against G53247/bf44e89c, not a Context algorithm bypass. The newly frozen G must be independently reviewed/bound before any run; its repair does not resolve C1/C2 or create the C31 kill producer.

G's current literal outer receipt/lifetime/completion records do supply the parent/host/creator, two empty observations, Job close, native root exit, IO/stdio/RSS references and source/run fields consumed in the scoped Context review. Only those inspected paths are described here; there is no blanket G/controller qualification.

## Verdict and permitted handoff

REMEDIATE C1/C2 before affirmative CONTROL source integration or native admission. Preserve current source/negative history. C31 and REAL remain explicit independent pending adapters; all control/native/runtime observations are UNRUN. No real fixture can be archived or retired from this verdict.

The ArchiveR3 four-file source acceptance remains independent and conditional; its report pair is unchanged. Earlier40916 overcap and48752 first-repair history, old a5/R2 findings and missed04:10 watchdog remain preserved, without retroactive PASS. Engine0628322e4b48e0efad19cdc32238a622d4aaa7a8, functional11:10Z, closeout13:10Z and44GiB cap remain fixed.

Only the newly admitted parent fixture-retirement-native-Context-review/{review.md,evidence.json}, combined<=24576 bytes/2 files, may be written. Before EVERY report write, recount META-v9's exact ALL-P/copy and Landing physical membership, seven subtrees with only stream-snapshot-controls excluded, and24 promoted harness files/245974 bytes. Original Context/Archive/G sources retain their separate C3 allocation. Hold parent-confirmed OTHER pending136381 plus this new24576 once, consuming only this report allowance as it materializes; do not add historical generic256000 or prior reports again. Exact source hashes, scope, prewrite budget GO and report hash are in evidence.json.
