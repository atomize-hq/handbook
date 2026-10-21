# Private platform039 interfaces — SOURCE ONLY

No interface is runtime GO. Parent source/binary/schema/review/guard/budget
bindings and actual complete import closure must precede activation.

LaneHost exposes authorize(config,lane), FAST poll(), checkpoint(label,reserves),
invoke(spec), record(label,value,category,max_bytes), drain(label), and run_case.
history_authorization is None until actual authorize(config,"history") stores
the full positive receipt. Shared runner public make_runner(host,bindings) is
pure before that authority; preflight may then touch native ports/harness.
harness is exact reviewed shared7ce; its Driver/AssertionError(row) contract stays.

lane-invoke-v2 closed common fields:
kind,api,label,executable,role,timeout_seconds,max_stdout_bytes,max_stderr_bytes,
growth,allowed_exit_codes,profile,transport_profile,max_receipt_bytes.
engine-json adds final_pin,root,action,fields,stdin_release,max_input_bytes,
quarantine_release. fields MUST exclude root/action; publish wraps request once.
python-argv adds script,argv,transitive_bindings,startup_gate,required_gate,
max_stdin_bytes,output. Scripts require unique reserved startup gate before any
SELECTED/corpus IO. Only one exact expected exit; profile is parent-reviewed
typed data, never a formula callback. The actual invocation policy and profile
must agree before launch. Return includes response in memory and raw single-ref,
exit/source/image/budget/closure, engine_ns/spawn_total_ns/native peak.
No duplicate standalone stdout JSON file. Helper must consume response directly.
record(bytes) preserves exact bytes and hash; projection happens before encoding.

Single raw stream: canonical UTF8 sorted compact JSON plus LF, RFC4648 base64.
One stream header binds run/source manifest/guard/claim. Each call has seq:u64
starting0, strictly+1 on every BEGIN/plannedIN/START/RSS/out/err/END.
BEGIN pid/creation nullable, input_scope="planned", input_n/input_sha and exact
planned input frames; this is forensic request capture, not stdin delivery.
START closed {v,kind,call,seq,pid,creation,qpc,rss}, one positive owned native
identity/RSS before actual stdin operation. DATA closed
{v,kind,call,seq,stream,offset,n,sha,b64}, <=16384 raw bytes, stream in/out/err.
RSS retains actual QPC/native PID/creation/rss/peak/role/phase.
END retains actual closure/violations/source/image/input/output clocks and counts,
in_n/in_sha actual completed written PREFIX and first_write_qpc.
first_write_qpc is sampled immediately before first WriteFile eventually returning
positive completion; pending candidate retained until completion. Zero written
means null. Positive means >=START.qpc. PlannedIN may precede START; successful
out/err and actual stdin cannot. Constructor failure without START is censored.

Reference closed {schema:"single-raw-ref-v1",path,run,call,begin,end,frames,bytes,
sha256}. begin/end bound physical span, possibly interleaved other calls. SHA
and bytes cover ONLY exact canonical frames for this call in physical order,
including END; no self-hash field. Finalizer also hashes closed whole stream.
Extractor checks sequence, stream continuity, exactSHA/call/run/source/image,
positive start, expected actual native identity, full stdin/EOF/reap/IO/callback
closure and strict stdout JSON. Unknown/nonclosed data/start/begin refuses.

Projection closed {schema:"native-response-ref-v1",raw_ref:<exact ref>}.
Only exact persisted decoded native engine envelope is replaced; expanded size,
depth and nodes are bounded. Actual response bytes exist once in raw capture.
No caller-supplied response flag creates PASS. NativeCallFailure args[0] remains
the unchanged in-memory row; bounded repr points to this call's raw evidence.

transport_envelope_v2 derives per16KiB base64+256-byte metadata ceiling, actual
input, stdout/stderr caps,1024-byte RSS rows, BEGIN/END max and all sink membership.
Each active call reserves success raw bound plus failure escrow. Pending escrow
is released only after owner/writer completion; retained stream count increases
monotonically. Profile keys include api_version=2,capture_schema,chunk_bytes=16384,
profile_sha256,stdout_bytes,stderr_bytes,rss_samples,callback_seconds,
failure_tail_bytes,begin_max_bytes,end_max_bytes,reference_max_bytes,
projection_max_bytes,all_sink_components v2. All sink components are mandatory:
input_frames,stdout_frames,stderr_frames,rss_frames,call_header_footer,
reference_rows,harness_projection,helper_records,oracle_metadata,outer_guard,
finalization. Whole-phase aggregate counts live escrow once per maximum live
writers, not per602 sequential calls. No averages/gzip/deletion credit.

closed-writer-settlement-v1 requires source_manifest/profile/budget/whole-phase
SHA and positive process_reaped,io_closed,callbacks_joined,raw_writer_released.
Actual retained category bytes/files/dirs must not exceed prior upper lease.
Only unwritten potential is released; original upper and closure proof remain.
Corpus upper leases stay until a source-proven actual delta/quiescent census.
Parent grant supplies profile_sha256/whole_phase_forecast_sha256.
checkpoint emits forecast_sha256 and raw_ref. Final outer census follows all
host output, gzip/finalJSON closure and two actual empty Job observations.

HistoryPorts source owns bindings/build/catalog/namespace/ownedread/write,
verify_ready/clock_sample/enter/leave/set_origin/observe_started/current and phase
observations/abort/guard_prefix/donor_authority/seal_context/index/read_reference.
Shared036 owns semantics, exact16 phases,1600 accepts/4000 offers/640 paced calls
and640000 warm+640 first, matched initial unpaced samples and final provisional
seals. Exact finite selected policy is publisher44s/reader48s/oldpin23.6s with
unchanged45s pacing; profile owner035 must bind actual budgets/serializer widths.
031 prefix observation is provisional and cannot pretend final closed gzip/Job.
Final16 body refs exclude own ref and are joined only after host exit.

Windows Native uses the same lifetime lock as Driver for identity/memory/resume/
terminate/close. Retired or uncertain numeric handles are not retried. Driver
owns Popen/all3pipes/reap. W5 drain handles unstarted threads, isolates everyjoin
and retains errors. finalize independently attempts evidence/census/native close.
Reader end proofs are idempotent and require actual owner/EOF/native/IO/callback
closure. Ordinary injected exit2 differs from intentional138/137.

Clock ABI counter(),frequency(),bridge(),process_proof(pid) uses actual lazy
kernel32 QPC and bound retained stage/child identity, never arbitrary PID reopen.
No native API has been invoked. Bootstrap explicitly opts into a new031
source/schema via exact parent release; old50ecc is immutable.

165 case source entries are named but source/Windows ABI review and complete
dependencies remain pending; the remaining document-matrix body is explicit.
No case, build, unit, native control or runtime qualification is implied.

KNOWN CAPTURE BLOCKER: frozen7ce request-before-dispatch and call-result retain
input payload copies. Response projection is implemented; a source-owned planned
input predispatch handshake is still required to eliminate input duplication and
retain failures before Driver._execute/admit. Do not claim one-input capture or
whole-sink fit. Proposed opaque predispatch token/reservation + plannedIN ownership
needs exact source/schema/profile agreement. This packet is a first v2 source
review draft with explicit remediation still required, never a runnable release.
