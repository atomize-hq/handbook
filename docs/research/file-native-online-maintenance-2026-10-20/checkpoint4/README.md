# Checkpoint 4: bounded failed-attempt ownership and constructor CPU

Status: F1/F2 SOURCE REMEDIATION ONLY, not compiled or executed. The accepted a3 commit and ce8b
binary remain retained. This directory is a private research contract selected
for the authorized disposable experiment. The structured data format standard
remains PROPOSED; no public format/default/migration is approved by this change.

## Exact scope

Four engine source files: pack.rs, packing.rs, lifetime.rs, store.rs. Cargo files,
dependencies, source/package path, release profile and target directory are
unchanged. No graph/projection/semantic index implementation changes. Candidate,
publication, accepted result, packet, pack, catalog and page serialization stays
unchanged. GitNexus impact on the exact unregistered checkout is UNKNOWN; the
explicit manual risk is HIGH (memory accounting and crash ownership).

Builder caches only the checked sum of capacities of owned entry key/hash
strings. The counter advances after entries.push succeeds. Vec capacity and
old+new Vec reallocation storage remain in the pre-allocation reservation.
Production per-push metadata accounting becomes constant time.

## Explicit v2 quota owner contract

Existing stores continue to read/write the v1 RESERVATIONS.json map. Only
`lifecycle operation=enable-terminal-v2, explicit_disposable_opt_in=true`
establishes the CLOSED JSON v2 envelope after disposable setup/identity checks.
It contains version=2, store_id, active (at most 8 entries), legacy (at most 8
active ids), terminal_count (at most 4096), and terminal_bytes. Control size is
at most 64 KiB. Active original allowances + terminal_bytes + fixed 128 KiB
control margin must not exceed 32 GiB. Admission never enumerates terminal
receipts. The 4096 cap is a private experiment limit, not a changed active cap.

All owners present at opt-in enter legacy, including preparing owners that seal
later. Their state never becomes new preselection proof. Only a new v2 ordinary
publication (packet=false, packet_ref=None, checkpoint=None) can seal as
sealed-preselection. A new v2 selection grant durably changes the state to
selection-started before select_locked performs acceptance allocations,
membership/outcome writes, result/publication/hint/selector installation. It
never reverts. File selection, in-process preparation selection and checkpoint
selection share this grant. Initial store creation remains v1. Packets and
nonordinary candidates stay outside the terminal-release subset. Before stamping
NEW sealed-preselection, both fixed attempts/<attempt>/terminal-{receipt,control}-stage
slots must be absent. That durable state reserves those private slots before
either can be created; legacy owners never gain this namespace claim.

The former v1 map parser rejects the v2 envelope at reservation boundaries.
Store::open itself is NOT claimed to reject v2. Old accepted reads/fast paths
can still work. All v2 writers preserve the WHOLE envelope and legacy set;
selected transfer cannot remove a fenced owner, and abandon cannot overwrite
a JSON terminal receipt with `fenced`. Rolling sees the bounded active map and
does not serialize it back over the envelope. Reused attempts are rejected by
point checks of active/attempt/generation/fence ownership, with no scan.

## Release operation and proof boundary

`lifecycle operation=release-definitive-conflict, attempt=<safe id>` is explicit.
It does not cancel or fence the logical request. It requires the exact sealed
candidate digest, request/intent/expected/store/attempt bindings and the new
preselection state. A noninitial expected selector must authenticate as selected
history older than the captured current head. Accepted lookup uses intent=None
FIRST. Same-attempt acceptance refuses to the normal resolution/recovery path;
another accepted attempt refuses for an active owner. A completed terminal
receipt stays idempotent after a fresh same-request attempt is accepted.
Missing, corrupt, ambiguous, legacy, packet, current-base and selection-started
owners refuse. A missing reply is never evidence of a definitive loser.

Shared P and the captured physical handle remain live throughout. All ancestor
checks, candidate hashes, selected/history/indexed outcome proofs and receipt
bytes/hash preparation occur OUTSIDE W. Exact raw RESERVATIONS/SELECTED/STORE
bytes are captured. Under W bounded control bytes are reread/compared, including
the actual current final receipt, plus native opened-handle checks. Mismatch
returns deferred or refuses, releasing W so fresh proofs can run outside W.
There is no Q acquisition, index traversal, candidate hash, ancestor/namespace
walk or garbage deletion under this handler's W interval.

Each immutable CLOSED <=8 KiB abandoned/<attempt> receipt binds version=1,
store/attempt, candidate digest, request/intent/expected, definitive-conflict
reason, original allowance, used bytes, fixed 128 KiB handoff margin, charge
and the exact private relative control-stage path. used+margin must fit the
original allowance BEFORE a receipt/fence can be written. Exact existing final
bytes/hash are required; a final receipt is NEVER repaired or overwritten.

The exact active preselection owner reserves two deterministic private stages.
The receipt stage is fully written/synced before atomic rename publishes the
immutable final fence. Death after create, before first write or during native
prefix writing leaves NO final fence and retains the full active charge. Retry
authenticates the active claim and may rewrite its partial private stage. A
stage existing before the durable claim refuses at seal. A stage unexpectedly
appearing after preproof defers rather than being adopted inside W. If another
preproof saw no fence before a first writer published and died, the second
rereads actual final bytes under W: exact bytes resume; mismatch refuses.

Both stage writers open WITHOUT truncation. Existing handles are checked outside
W and retained with nonblocking exclusive file locks. Immediately before
set_len/write under W, the SAME handle must still be regular, single-link and
have the proved native identity. Unix uses dev/ino/nlink; Windows uses volume/
file-index/nNumberOfLinks through existing GetFileInformationByHandle APIs,
OPEN_REPARSE_POINT and sharing that denies other writers/deleters. Reparse,
symlink, identity and hardlink aliases refuse before truncation. New stages use
create_new and the same handle proof. Complete final receipt reads also reject
multilink/reparse handles. No dependency or Cargo feature changes.

W spans receipt durability through ONE atomic v2-envelope replacement removing
the active owner and adding terminal charge/count. Before replacement, including
receipt-before-control death, the active full allowance remains charged. After
replacement, active absence plus the exact receipt yields an idempotent reply
without increment. Both scratch files are synced before rename; a checkpoint after
rename and BEFORE root sync exposes uncertain reply recovery. Windows directory
sync remains reported unsupported: this does not add a power-loss claim.

Unused allowance release is not reclaimed storage. Every reply reports
reclaimed_bytes=0. Terminal garbage, failed B generations and receipts remain
on disk and in the actual whole-store census. No planned deletion credit.

## Collect and lifetime preservation

V2 collect refuses at lifecycle entry BEFORE W, head loading, semantic scans or
deletion. Terminal GC is UNIMPLEMENTED and needs a separate future contract.
The first registry check happens before trying exclusive P, so even a held P
reader gets the explicit v2 refusal. V1 collect takes try-exclusive P before W
and rechecks mode while holding it: opt-in holds shared P, preventing conversion
across this check. It only tries W once: contention returns busy:collect-writer
and drops P, avoiding a W-holder needing shared P. The V1 collector keeps
its prior disposable/root/ownership checks. No v2 owner can reach that collector.
Other preparations, snapshots, packets, accepted evidence and physical generation
ownership are preserved. No full GC is called inside online qualification.

## Prepared controls and honest remaining qualification

There were 20 source tests at the accepted compile-only freeze: packing7, pack3, paged4, disk1, bounds1, lifetime4.
All remain UNRUN on this source. The former a3 final run was 11/14, not 13.
New accounting checks compare the cached counter with an independent allocation
scan through growth and failure paths. Production push_builder is exercised at
the maximum-key index limit and compared with independent direct construction.

The existing heavy cache test now also holds a REAL independent borrowed source
and REAL cache, pushes source entries through production builder reservations,
reserves finish output before allocating its Vec, proves eviction, and exercises
checked-overflow refusal before allocation. Appended assertions add ZERO fixture
files. Existing setup still creates 80,976,318 bytes (five source indexes plus
one changed-index copy); it cannot be run under the old 96 MiB shared fixture
cap. No authentic preexisting large fixture was found for read-only reuse.

protocol.py contains the native A/B concurrency, ownership, crash, token-race,
corruption and byte-unchanged refusal controls. Quota-cap fixtures are explicitly
structural, not 4096 authentic attempts. New controls include ten distinct
zero/partial-stage deaths, all create/write/publish/handoff crash windows,
concurrent preproof deaths, and actual handle/hardlink refusal. Hardlinks are
FAULT INJECTION ONLY within NEW tiny disposable fixtures; all names/bytes are
retained and charged, with no dedup credit. No hardlinks are used for normal
representation, transport, donor reuse or space workarounds. None have run.
An explicitly structural partial-stage injection also checks real successful
selection retains attempt scratch under its accepted allocation owner.

control.py binds reviewed QPC guard SHA50ecc7..., requires the new budget SHA,
captures Rust/Cargo/control inputs, and runs the complete workspace release
build/unit suite serially with NO filters. Build scratch stays in target/tmp;
unit temp files go into admitted fixtures. Actual monitor gaps >40 ms censor.
JSON/gzip closure, census and fresh bindings remain mandatory after any repair.

forecast.json is a finite proposal, NOT runtime admission. The new 192 MiB
fixture cap has been sealed; a NEW exact phase budget and parent runtime GO are
still required after this remediation's static review. The previous frozen
manifest/reports and unused runtime budget remain preserved, unexecuted.
Current source-only
work does not establish the memory boundary, v2 protocol, native, long-history,
full matrix, backup/restore or performance qualification.


## Paced source candidate: no execution admission

The accepted compile-only binary remains f154d60f02d86582206cbdf5291da5a4e74db205c714ff1485c2a7ded4ff14af
(20 compiled tests, none run). This amended source has31 tests; only Rust
syntax/Python AST/JSON parsing occurred. All prior raw is retained. Superseded
Windows-only draft hashes/limitations are in forecast; no86KiB duplicate.

Only packing.rs adds pacing, at most16384 bytes above the accepted stage.
pack.rs/lifetime.rs/store.rs retain reviewed hashes. No Cargo/dependency/feature/
profile/package/default-read changes. The original unpaced sampling loop uses
a shared row calculation/cache string; counts/timing boundaries/fields/values
stay unchanged when concurrent_pacing is absent. Its six categories retain
50 fresh processes/category and500 warm queries/category in both LEGACY and
PAGED representations at the exact H cutoff before semantic writes. The
coordinator owns explicit reviewed physical conversion/oracles. Root admission
NEVER converts. All16 paced phases follow conversion and require v3 capture;
there is no paced STORE2/legacy mode or inferred migration.

Private closed concurrent_pacing v1 input (deny_unknown_fields):
version:u8=1; phase_id:safe ASCII1..99bytes;
family:enum "cp4-control"|"f05-selected";
reader:u8=0..3; batch:u8=0..9; control_dir:canonical absolute PathBuf;
anchor_sha256 and ready_sha256:optional64lowerhex, absent/null for batch0,
both required later. Exactly one request row and count1000. id is1..256 UTF8
bytes; category1..64 ASCII bytes. No approved public interface/format migration.

Root admission is a SOURCE-DEFINED per-platform allowlist in pace_allowed.
A request/env cannot add a root. CP4 admits only registered fixture prefix/
<safe cp4-name>/disposable/store; F05 only the two exact registered donor roots.
Windows F05 suffixes are history/H1000-baseline/disposable/store (H1002) and
history/authentic-N128/disposable/store (H10002), beneath the fixed matrix
runtime. Same inherited store_id is irrelevant. Mac/Linux approved prefixes are respectively /private/tmp/file-native-online-
macos-20261020/fixtures and /home/spenser/file-native-online-linux-20261020/
fixtures. Only {native-paced-cp4-<40lowerhex>|native-final-<40lowerhex>}/
<safe-cell-or-lineage>/<safe-attempt>/disposable/store matches. No Unix F05
history. Native owners confirmed these patterns; unsupported roots fail closed.
FILE_NATIVE_STAGE_JOB=1 and FILE_NATIVE_PACED_ROOT bind the exact canonical
root, including native Windows extended prefix. DISPOSABLE.json must match.
control_dir is exactly root.parent()/lookup-pace/phase, within the canonical
disposable family and OUTSIDE the engine Store. Supplied/canonical ancestors
reject links/reparse/escape. No unrecognized Store all-path-oracle bookkeeping.
All ready/stage/anchor files are retained F05 family fixture bytes; proof copies
are separately charged F06 evidence. Every failed prefix/stage remains charged.

Schedule: ordinal=batch*1000+j, j0..999; ordinary readers1..3 due_us=ordinal*4500;
held reader0 due_us=16000000+ordinal*2900. Deadline=origin+
floor(due_us*frequency/1000000), checked. Last ordinary44,995,500us; held
44,997,100us; held batch0 last18,897,100us. No reset/skip/extra/adaptive queries.
Sleep is BEFORE Instant starts around resolve. Every warm_ns remains, resolving
only. Catchup/late counts and maximum start lag are retained. deadline_ok
requires lag<=100ms and physical successors<=60. The finite proof separately
requires child join by45.1s, publisher end by45s. Overruns fail qualification.

clock_kind is "windows-qpc", "linux-clock-monotonic" or
"macos-clock-monotonic". Windows raw QPC/Frequency uses existing locked ABI.
64-bit Linux/macOS use native extern C clock_gettime(CLOCK_MONOTONIC), ID1/6,
reprC timespec{c_long seconds,c_long nanoseconds}, Linux clockid c_int,
Darwin c_uint. Seconds>=0/nanoseconds0..999999999 and checked sec*1e9+nsec
yield u64 ticks at frequency1e9. Other systems/32-bit Unix explicitly refuse.
Primary headers: [Apple _time.h](https://raw.githubusercontent.com/apple-oss-distributions/Libc/main/include/_time.h),
[Linux UAPI time.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/time.h).
Cached libc0.2.191 agrees; installed native header/type/constant and same-clock
Python bridge proof remain prerequisites before selection. No Rust Instant is
added to Python perf epoch. Legacy *_qpc field names mean declared clock ticks
on Unix; clock_kind/frequency are authoritative.

Four ready files reader-0.ready.json..reader-3.ready.json, each<=4096 bytes,
create_new only. Complete exact marker schema:
clock_kind,family,version=1,phase_id,reader,batch=0,pid,request,category,
selector{publication,digest},physical{id,ordinal,plan},catalog,
capture_qpc,first_done_qpc,ready_qpc,frequency.
Actual Snapshot and physical pin precede the separately timed first resolve;
that resolve completes before marker creation. Native PID is actual process ID.
Python reads at most4097 bytes and rejects oversize before parsing; partial
JSON is not readiness. Windows retained open handles deny writes/replacement.
Later calls verify original marker/anchor digests. No existing marker adoption.

Four actual readies/live platform owners precede one immutable anchor.json
(strict<=4096 bytes), exactly:
{clock_kind,version:1,phase_id,frequency:u64,origin:u64}.
Controller creates anchor-stage exclusively, flushes+fsyncs then atomically
renames once. Native wait<=5s, initial origin future with lead<=2s; proof uses1s.
No origin reset or extra binding sidecar. All files survive and are charged.

Paced rows retain existing warm arrays and add concurrent_pacing header,
serialized<=2048 bytes, exactly:
clock_kind,family,version,phase_id,pid,reader,batch,frequency,origin,period_us,
hold_us,completed,first_read,last_read,io_done,pins_dropped,reply_ready,wait_ticks,
max_lag_ticks,late,catchup,deadline_ok,successors,physical,catalog,selector,
cache_current_bytes,cache_peak_bytes,cache_after_drop_bytes,anchor_sha256,
ready_sha256,milestones.
completed1000; <=10 tuples[global_ordinal,start_tick,end_tick], j0,111,...999.
Later selected/physical/catalog and query/category are their OWN captures/
schedule, may differ from batch0. Original ready SHA binds stream identity.
io_done follows all dependent Store I/O/result hashing. Snapshot is dropped;
Metrics.finish folds lock accounting, checked independent sum of Index.memory
must equal cache_bytes, borrowed/constructor/aux are zero, Rc strong_count1.
Peak includes current residency, bounded MAX_CACHE. Consume/destructure sole
nonClone PhysicalRead, explicitly drop cache then sole File lease, sample
pins_dropped and cache_after_drop_bytes0. No Store I/O follows. Snapshot owns
one private nonClone preparation File; resolve locals return Values without
files/Rcs; Index/page caches hold bytes without generation leases. Capture moves
one noninheritable File; no lease Clone/accessor/try_clone path exists.
reply_ready is native preparation, NOT exit; driver child wait/join is separate.

Platform-owned SHA-bound clock module interface:
frequency()->positive int; counter()->same declared native ticks;
bridge()->{qpc_before:int,perf_ns:int,qpc_after:int} bracketing SAME clock;
OwnedDriver.process_proof(pid)->{owned_token:stable nonempty registry token,
pid:int,creation_time:stable native token,
image_path:absolute str,image_sha256:64hex,alive:bool}.
Extra observations may exist; stable fields compare, including owned_token.
C.bind_driver(MatrixDriverType, executable:Path, raw_output:Path)->OwnedDriver
is REQUIRED, with proof_contract exactly native-owned-v1. Its call/publish/
head/binary_sha256/executable interface preserves Driver raw/return behavior.
The factory must bind the SAME platform Native clock context to its actual
retained OwnedProcessRegistry. It must register the real newly created child
Popen/native HANDLE before on_started/resume/input and retain the record
through native exit/wait, with creation/image/input/exit raw identity bindings.
process_proof(pid) is a registry lookup joined to that Native record, never
arbitrary OpenProcess PID inspection or a fresh unowned PID object. Merely
wrapping the old PID callback is insufficient. Native4 adapter implementation/
ABI/path/SHA/registry raw review is still REQUIRED BEFORE PACE RUN; no adapter
is implemented or implicitly selected here. Factory/contract checks occur
BEFORE fixture init; absent adapter fails closed, with no PID-only fallback.
Native clocks/bridge must use that same context; never assume perf epoch.
onstarted, actualready and all4live-before-anchor proofs recheck PID/creation/
image/path/SHA. MarkerSHA joins stable process-proof SHA explicitly in ready
result; each result retains stream_proof_sha256 and its OWN started proof/
native PID/header match. Driver waits on that actual child handle, then joins
QPC; raw input/positive-start PID/exit/process_done_ns remain bound and retained.
Exact platform module/adapter path/SHA/Native4 ABI proof must join future phase
input/review. Linux installed proof SHA12b17dee6221966770692d3cbe52ed5852dd371dc61c774f768dd25c6fd61afe
confirms c_long8/timespec16 offsets0/8 and clock1; Mac SDK confirms same
layout and clock6 (raw4 is prohibited). These proofs permit source freeze,
without granting fixture allocation or runtime admission.

pace_controls.py is the sole new filename, <=8192bytes, UNRUN. This finite
Windows control creates a NEW tiny A fixture, genuine cancellation and lost
reply at selector-visible, three accepted categories plus missing/cancelled/
lost; all300 fresh +six500-warm baseline. Four genuine readers, ten calls each,
40,000 warm+40 first queries; all raw arrays retained in Driver JSONL. One
publisher,100 fixed400ms slots, exactly10 packing calls (<=60), no retries/
extra queries. Native I/O/hash/drop/exit ordering and actual held18.9s are
checked. Three pure source tests cover closed input, deadline/conversion
boundaries/overflow, header refusal; all31 tests remain unrun. Full-matrix qualification remains pending.

control.py requires reviewed QPC guard SHA and exact budget. Pace proposal:
180s,6MiB fixtures,4MiB evidence. Exact pre-new-phase evidence split:
actual44,701,439 +full-build/unit393,216 +full-protocol2,621,440 +
input/census131,072 +unassigned2,484,481 = hard50,331,648.
Pace reserves ZERO;4,194,304 requested remains deferred by1,709,823.
No compression estimate/deletion/borrow from protected margins. Future prefix
GO may run full-build/unit/protocol then use actual census to release unused
prospective allowance; otherwise explicit internal F12 transfer is required.
No reduced baseline/protocol. Fixture proposal176,197,844/201,326,592; controls80KiB/source1MiB. Full30-file freeze,
independent review, exact reconciled budget and runtime GO are still required.


Reviewed source amendment (budget2305B/3efc4dd4...): six TEST-ONLY production
constructor cases bring actual unit count to29. Discovery128 valid structural
rows reach byte refusal,129 count refusal; this is not128 actual reservations
(current active cap8). A separate110-row constructor uses production io::json
and safe ASCII padding for exact32768accept/32769reject. Raw Builder::push
and finish_reserved accept8388608bytes, reject8388609 without mutation.
Catalog calls production catalog on64 real tiny Builder pack/index pairs,
accepts64 descriptors and rejects65. Work uses legal metadata path padding,
production validate_work/io::json/persist_plan, persists131072 and refuses
131073 before filesystem mutation. install invokes real create-new/refuse/
same-live-owner verification. Each emits observed boundary receipts. Genuine
CLI pack-index death, unfinished-journal refusal, abort and fresh distinct
birth preserve old owned bytes. No128 owner/1100 real garbage-file claim.
Extra units reserve2MiB/200 files and32MiB RAM, serial; heavy80,976,318-byte
fixture remains unchanged and may run ONLY ONCE on final reviewed Rust.

Pace keeps all10 physical builds and adds exactly4 cleanup calls after each,
40 total, without retries or an unbounded drain loop. Production cleanup
checks every retained generation before loose work; a busy first old pin
does not stop testing/pruning unprotected middle generations. Raw immutable
selected Work hash/byte length, membership and owned-process proofs accompany
each cleanup. Success requires actual absent middle plan while first old
owner and3 ordinary owners are alive, before native first pin_drop. Native
headers/milestones retain ordinary progress. No protected fixture is targeted:
cleanup is confined to this newly created tiny disposable proof Store.
Actual Work must be<=96KiB before each build/after each cleanup group; otherwise
proof fails without skipping/retrying queries/builds or loosening128KiB.
The40 actual cleanup calls increase requested pace evidence to4MiB. Exact
retained/build/protocol/census/unassigned split is unchanged; pace still has
ZERO reservation and now1,709,823-byte deficit. New80KiB/five individual
control caps are checked before every write. Runtime budget/source review
and the real NativeOwnedDriver registry adapter remain prerequisites.

## Actual telemetry (31 source units UNRUN)

Authoritative v3 budget/hash are in forecast. The10-file telemetry/two-unit
patch is7817B<=8192. Native workspace execution remains required.

rolling_publication_body_opens counts successful logical publication-body
reads through bounded packed/loose I/O, including eligible's second read.
Head-target, suffix(expected sequence/completed cutoff) and explicit-deferred
contexts differ. unchanged_historical_body_opens counts suffix accesses at/
below completed cutoff; bounded head reads are allowed. Explicit deferred
reads increment rolling_deferred_body_opens. rolling_deferred_rechecked
snapshots actual rolling_deferred_visited. Safe Rc/Cell RAII restores context
on success/error/unwind.

rolling_discovery_metadata_read_bytes/written_bytes count completed native
WHOLE immutable Work-plan/deterministic work-stage I/O. Their sum is
rolling_discovery_metadata_bytes: a conservative Discovery-containing-Work
envelope upper bound, NOT a literal JSON subspan.
rolling_discovery_serialized_bytes counts canonical Discovery buffers actually
created by production validate's io::json size check, separately from I/O.

forbidden_global_operations counts bulk events while THIS process owns actual
W/Q-exclusive. Source-global depths cover nested Metrics; sibling readers are
permitted. Native File closes before depth decrements. W wraps both store
acquisitions; Q wraps activation/save_inner. Failed acquisitions set no depth.
Global forbidden/envelope/serialization totals start at process creation;
finish snapshots without clearing, including repeated calls. Logical body
counters are per Metrics/discovery context.

Hooks cover native directory enumeration, Disk full/range/reachable/bulk/audit,
legacy full-index read/hash/parse even below64KiB, paged full-file audit/hash,
cohort hash updates, full copy, representation file/directory deletion and
large writes. Generic whole-buffer hash>64KiB is supplemental. Bounded
selected-head/control CAS/paged point routes are allowed. No blanket context
skip/hash exemption: audited Work hashes occur before Q/W. Unix immutable-
install stage unlink is bounded installation, not bulk representation deletion.
Raw std directory/hash/delete sites were audited.

Two positive units cover actual W/native enumeration/small legacy index,
nested Metrics, after-drop nonincrement, actual accepted old publication
suffix/deferred reads and context restoration. Extra tiny unit runtime
forecast256KiB/100files is required before tests. No execution yet.
