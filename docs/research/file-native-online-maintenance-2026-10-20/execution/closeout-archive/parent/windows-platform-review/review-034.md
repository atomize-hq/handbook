# Windows platform source review 034

Disposition: **REMEDIATE**. W1-W4 are addressed in their targeted source paths, but one remaining host cleanup defect prevents acceptance of the complete replacement. No control or runtime GO follows from this review.

Reviewed source root: `C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/windows-final-platform`.

The frozen replacement contains exactly 27 flat regular files / 263081 bytes. Receipt SHA256: `7c31340ccbd4509692f46192f7521d5285d049b8b6c73248ddfac8ca695fde4d`; 26-entry self-excluded canonical manifest SHA256: `4c644c15c56f300233552e17702d04bb45a2051bdc6e1e57c221d9b3d90193cf`. All entry sizes/hashes, individual caps, group caps and whole cap were independently checked again at completion.

## Finding W5 — P2: host drain aborts on a retained unstarted callback

Location: `windows_capsule.py:260-264`, especially the unconditional join at line 261. Related acquisition: `windows_adapter.py:164-173`; finalization caller: `windows_bootstrap.py:165-167`.

`NativeAdapter.spawn` registers a callback handle before `thread.start()`. The replacement now correctly keeps that handle, records the start error and sets `joined` when start fails. Such a thread can have `ident is None` and never have started. Adapter `wait` and `close_case` now guard this state, but `LaneHost.drain` still unconditionally joins every registered thread. Joining the unstarted thread raises `RuntimeError`. The loop has no per-handle exception isolation, so later callbacks are not joined, no drain receipt is appended, and the bootstrap's immediately following `host.finalize()` is skipped. Direct `finalize()` also calls the same drain before closing evidence and Native. An existing earlier callback may therefore remain unjoined while the failed callback's handle is safely retained. This is a concrete partial-construction cleanup path, even though the outer Job controller must still contain descendants.

Required remediation: make host drain understand retained unstarted handles, independently attempt every callback join, retain each technical failure and produce an honest closure result. Ensure a failure in one join cannot bypass the remaining owned closeout attempts. Preserve the original start failure; do not manufacture callback success or native/Job closure. Add a prepared source regression with an unstarted registered handle and another outstanding callback, plus a join failure followed by another handle. Check repeated drain/finalize behavior and the failed closure receipt. This review did not execute that regression.

## W1-W4 source reassessment

| Original finding | Source disposition | Basis |
| --- | --- | --- |
| W1 successful reap skips tail cleanup | Addressed in targeted transport path | `DriverSupport._finish_resources` initializes the two-second tail deadline unconditionally. Termination/reap/client close, stream draining, pipe closure, timer/callback joins and HANDLE closure are separately attempted. A read failure records a violation without asserting EOF. `record_call` retries unretained suffixes before strict parsing; row/sink failures no longer skip owner completion and evidence settlement. |
| W2 termination/close HANDLE reuse race | Addressed in targeted ownership paths | Driver and Native share the same retained `RLock`. Identity, memory, resume, termination and native/Popen close serialize through it. Ownership and termination capability retire before release; uncertain numeric handles are not retried. Popen fallback uses only the still-owned open Popen HANDLE, never a new PID lookup. Natural exit is checked before signaling and after a failed signal. |
| W3 partial constructor/reservation loss | Addressed for the original pipe/hash/reservation paths; W5 remains | Registry setup, reservation and actual image hashing now fall inside the owner try/finally. Pipes register before component acquisition and acquire sequentially. Duplicate handles register before verification; stage/evidence/host construction unwinds acquired resources and retains uncertainty. The adapter's new start-failure retention exposes the unchanged host-drain assumption described above. |
| W4 reader-end contract mismatch | Addressed in targeted adapter path | `interrupt(cancel/death)` now waits for `finish_interrupted` and returns a closed proof, cached for repeated finish. Owner done, raw retention, real stdout/stderr EOF, native/I/O closure, callback join, exact input/image/native identity and absence of technical violations are required. Ordinary injected exit 2 requires intentional kind absent and exact EXP_ERROR/EXP_BARRIER plus injected-IO response. Cancel/death require exact 138/137 and the same call's AssertionError row. The frozen shared Harness contract remains unchanged. |

These are source conclusions. The prepared fake traces cannot establish native HANDLE, I/O, callback or Job qualification.

## Clock ABI source review

`windows_native.py:487-542` supplies real lazy kernel32 QueryPerformanceCounter/QueryPerformanceFrequency calls with explicit pointer/BOOL declarations. `counter` and `frequency` reject invalid results; `bridge` brackets `perf_counter_ns` with two native QPC samples and requires ordering. There is no fallback clock.

`LaneHost` binds the retained Native and owned registry. `process_proof(pid)` takes their lifetime lock, supports only the retained stage or exactly one owned live child, checks creation/liveness/image, hashes the image and rechecks identity. It does not reopen an arbitrary PID. Unbound, non-owned, closed/exited or ambiguous ownership refuses. No blocker was found in these four API source bodies within this review's scope. Their recorded AST-method hashes are owner-supplied bindings; this reviewer did not invoke AST tooling or independently certify that hash domain. Complete file SHA was independently verified. Real clock/native behavior and a new controller's explicit source opt-in remain pending; old controller `50ecc7c4...` receives no new qualification.

## Complete review scope and negative comparison

Read the replacement's 11 Python sources, eight case/growth maps and eight binding/document/receipt files, including the complete prepared control source. Compared against the preserved negative packet, including complete changed transport/adapter/host/evidence/native paths and changed binding/document data. Twelve files are byte-identical to the predecessor: six original case/constructor/protocol maps plus workspace ordinary data, and bootstrap, deadlines, growth, hooks and oracle bridge. Their previously reviewed full bodies remain the same. The changed growth map explicitly marks v1 LONG unfit and v2 work pending.

The original negative tar was read without extraction: 53005 bytes, SHA256 `926469b3d65c99f55a6404fc8931ab4b32b70f5551816cc61c07c2a2de9d179f`. Its 27 members total 222646 bytes; all 26 archived manifest entries and the archived receipt SHA `b4671f65fe56b6cf964aa850c843fac49fa36c0fe4050891a5a302dc8ab76ec6` were verified. Original negative review/evidence and archive remain preserved.

The control source prepares 43 original plus 21 regression groups (64 total), with 19 nested fake transport fault scenarios and constructor/close/control/reader-end traces. These cover the targeted W1-W4 paths but omit the host drain/unstarted-thread combination. The 44 literal assertion-site and syntax-compile counts remain owner-reported static metadata. No test result is inferred from a group name or a prepared assertion.

Reviewer activity was limited to source/data reads, byte hashing, standard-library in-memory JSON/tar/diff processing, membership census and these two report files. No supplied module import, AST, compile, control, Win32 provider/API, engine, build, Job, fixture/corpus write, source edit, Git operation, indexing/bootstrap or delegation occurred. Exact-checkout GitNexus remains unavailable/UNKNOWN; the documented manual HIGH blast radius includes adapter/host/history/shared helper callers and transport, reader control, concurrent maintenance, verification and closeout. No graph count is claimed.

## Budget and remaining boundaries

Source budget034 is 2829 bytes / SHA256 `38c9924d8812345700490b9867f7553e915bd344a843f2171c80d309d6bf2293`. Actual groups: source 158648/196608 bytes in 11 files; case data 59764/262144 in eight; bindings including receipt 44669/65536 in eight. Whole 263081/524288, exact 27 names, no subdirectory.

Before this two-file report write, the fresh whole physical META census at 2026-10-21T02:44:30.202823Z was **8772035 bytes / 1050 regular files**. Membership is newly sealed META10 v5, 3503 bytes / SHA256 `ce01f5bc510f5472feaac941b82da16bf78b3c09d737231c4d2f2c9e4d01b3be`, cap 10485760 bytes. Counted every recursive P copy/archive, Landing root-only files and specified recursive subtrees minus the exact exclusion, plus exactly 20 promoted harness files / 203215 bytes. Original source exclusions and the separately allocated future raw subtree were not relabeled into META. Reserving the full 20480-byte two-report cap plus 262144 future bytes projects 9054659, leaving 1431101 bytes. Prior META9 evidence is not retroactively requalified by META10.

All 166 real cases remain PENDING; controls and native qualification remain unexecuted. V1 duplicated raw capture/per-call fatal-tail LONG envelope is explicitly unfit, with v2 lossless references/closed-writer settlement, quantitative LONG, concrete final maps, actual paced HISTORY and outer postexit finalization still pending source work. Final engine/build/schema/terminal-v2/quota oracle and full Windows all-sink admission remain unreleased. No runtime/body GO. Structured-data standard remains PROPOSED, with no default or migration approval. Source target 04:10:14Z, functional cutoff 11:10:14Z and protected closeout 13:10:14Z on 2026-10-21 remain fixed.

Review freeze may be released after this completion notice so the owner can remediate and prepare separately authorized next source work. This negative source verdict grants no control/import/runtime authority.
