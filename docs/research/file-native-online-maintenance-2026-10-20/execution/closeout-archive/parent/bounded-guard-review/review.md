# Bounded Windows guard: independent full source review

Verdict: **REMEDIATE**. Runtime/control/Job/engine GO remains **FALSE**. Five findings block source integration of this exact candidate and validation of its finite control plan. This review supplies no native qualification.

Reviewed the complete 1,249-line guard, complete 181-line controls, README, source receipt, source budget, complete 727-line donor, and the private dependency/reference closure. This is a fresh source review, including donor comparison, rather than adoption of producer AST/compile claims. Reviewed source was neither imported nor executed. No AST, compile, native control, Job, engine, Cargo, cache, corpus or Git command was run by this review.

Frozen inputs: guard `d3e554e0557f8d9939b3a579a3122fc6234c89f6a0414b028d7bc940236c3815`; controls `b2aebd784d822bfe83e3d6d7e2ec328f905b66987b3cd6fcb3e2b51faf6c54cd`. Full hashes, sizes, accounting and review scope are in evidence.json. All four source files total 71,963 bytes, within the registered 73,728-byte/four-file envelope and individual caps.

## Findings

### BG-1 [P1] A disconnected pipe is accepted as proof of drained EOF

`job_guard_bounded.py:721-724` sets eof=true for both ERROR_BROKEN_PIPE (109) and ERROR_PIPE_NOT_CONNECTED (233). `reference()` at 782 then derives complete=true and total_output_bytes from this flag when local closes succeed. The error-233 branch adds no censor or uncertainty, so it can contribute to qualified=true at 1222 without proof that all output was read. The new contract explicitly requires real EOF and exact retained/total counts. Treat 233 as a labelled failure with uncertainty retained/fenced unless a separately justified native proof establishes drained EOF; do not equate disconnection with that proof. Add a focused negative control for this branch, with retained hashes/counts and a nonqualifying result. This is a source inference about the accepted state; no output loss was experimentally observed.

[Microsoft error codes](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-) distinguish the two errors. [PeekNamedPipe](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-peeknamedpipe) reports available data without consuming it. Neither cited contract makes error 233 a positive complete-output witness.

### BG-2 [P1] Failed closes of temporary process handles are silently forgotten

`JobSampling.memory:640-642` discards the return from CloseHandle for a newly opened handle that was not transferred to live. On the departed-before-identity/reused path at 627-630, a failed close can therefore return an allowed exit race, lose the handle value, leave _UNCLOSED empty and eventually permit process_closure_verified/qualified. `creation_time:149-162` has the same unchecked-close pattern while registering/checking supervisors. Unlike retire() and _super_memory(), neither path retains the uncertain handle or reports failure. Check every temporary close; on failure, preserve the handle and propagate a durable closure-unknown fence that prevents subsequent calls. Add fault controls for both temporary-handle paths. Do not merely add an error while losing the resource needed for closure.

### BG-3 [P1] The control runner drops the only completed guard receipt

`controls.py:137-160` validates the returned row, then retains only case/status/receipt/jobs/faults. It drops row['completion'], receipt_sha256 and the actual postwrite census. The next iteration overwrites row, and the final summary at 174 contains no completion. The on-disk .json was intentionally serialized qualified=false/census-pending before the guard computes the final returned values at 1212-1232. Thus a PASSED summary cannot supply the documented completed receipt to an independent finalizer. Persist each bounded completion in an explicitly admitted sink (or a fully pre-reserved bounded summary), bind it to the receipt SHA/source/budget/bounds, and perform caller-level final census after persistence. Preserve failure completions too. Re-admit changed sinks/manifest shape before runtime; no implicit storage allocation.

### BG-4 [P2] The pipe-fence cell does not exercise an unjoined pipe or later-call fence

`controls.py:125-127` injects one 0.2-second sleep into drain(). Guard shutdown waits five seconds and then another second for that thread at 1106-1111. The injected delay does not force either join deadline, an uncertain close, retained buffers or _UNCLOSED. The generic hits assertion at 157-158 proves only that the sleep occurred; there is no assertion of the fence and no subsequent call that must refuse before a new Job. Replace this cell with a bounded deterministic fault that reaches the actual uncertain/unjoined path, verifies retained ownership and failure of a later invocation, and has admitted safe release/closeout. Its expected row/closure assertions must accommodate a deliberately unproven closure rather than demand success at 140. Keep the finite case set and deadlines under parent control.

### BG-5 [P1, inherited] Traversal failures can be labelled a complete inclusive census

`census:77` calls os.walk without onerror. Directory enumeration errors are ignored by default, so an unreadable admitted subtree can be omitted without an exception. Budget.check at 125-147 accepts those partial totals, and run at 1216-1217 labels postwrite_census_complete=true. This issue is inherited from the donor, not introduced by formatting; it is nevertheless material to the new private guard's complete-census and quota qualification. Make enumeration failures abort census and propagate the existing nonqualifying postwrite failure path; distinguish permitted absent paths from unreadable existing paths. Include a bounded enumeration-error negative. [Python's os.walk contract](https://docs.python.org/3/library/os.html#os.walk) documents the default error handling.

## Donor comparison and private closure

The candidate keeps the positional run signature, fixed functional cutoff, shared heavy token/registry, owned Job assignment while suspended, native process/job commit caps (2/3 GiB), 140 GiB free floor, supervisor sampled-RSS limit (768 MiB), and owned-Job termination boundary. It replaces the donor's gzip/evidence handling and PID-only reads with closed Bounds, bounded collections, stored-DEFLATE accounting, retained creation/HANDLE identity, sole-reader pipe capture, explicit closure retention, and returned completion. The old command-line entry point is absent; this is an opt-in module. Existing callers and old bindings receive no approval or silent migration from this review.

The guard's imports are standard-library modules plus Windows DLL bindings (kernel32/psapi/ntdll). Its Budget implementation is embedded; no matrix_budget or other private Python module is imported. controls.py dynamically loads only the manifest-hashed guard after its runtime_go/hash/case/budget prechecks; that loading was not performed here. The frozen harness/job_guard.py is referenced only for a SHA read at receipt construction, not invoked/imported. Its verified hash is 1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae. Donor remains 50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378.

Source inspection found the prospective aggregate formula accounts for all five guard outputs plus a completion allowance and shared control-file coexistence. Stored-DEFLATE row/block overhead and finish reservation are checked before ordinary growth; bounded collection and native-termination attempts have explicit ceilings. These are source observations, not proof that Windows execution conforms. In particular the new EOF, handle, completion and census claims need the remediations above.

The 23 names are present in the required order. Source inspection covered normal nested/sequential identity churn; RSS row/row-byte/raw/gzip caps; disk/supervisor row and byte caps; identity/final preflight; stdout/stderr; timeout; live RSS failure; initial/start/root/tail sampling gaps; exit race; pipe fence; and error storm. No cell has been executed by this reviewer. Per-case budget/manifest/summary admission, native verdicts, independent raw/QPC decoding, nonce/creation/FILETIME consistency and final physical census remain pending.

## Authority and release

GitNexus impact remains UNKNOWN/unregistered from the supplied source receipt; manual blast radius is HIGH for future opted-in sampling, identity, evidence, I/O and qualification. This reviewer made no source or symbol edits and did not bootstrap the index. The structured-data standard remains proposed. No public format default, migration, old receipt requalification, corpus mutation or caller pin was authorized.

Only review.md and evidence.json are written under the authorized bounded-guard-review directory, with their aggregate UTF-8 bytes computed in memory before either write, a fresh whole closed META10-v5 census before each write, and at least 256,000 bytes of future reserve. No files are deleted or relabelled for credit. Original source is outside META by the registered membership; all review copies/artifacts in P are included.

Source freeze 2026-10-21T04:10:14Z; functional cutoff 11:10:14Z; closeout 13:10:14Z remain unchanged. After this complete review and final unchanged-hash check, release the source read freeze to the coordinator for bounded remediation. Require a fresh exact-hash source review of changed material before integration, then separate finite native-control admission/RUN GO. This source verdict does not grant runtime GO.

