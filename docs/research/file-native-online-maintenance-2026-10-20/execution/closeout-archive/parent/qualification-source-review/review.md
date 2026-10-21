# Qualification-lane source review

**REMEDIATE — frozen source proposal; no RUN qualification.**

Reviewed the complete eight-file packet under `L/docs/research/file-native-online-maintenance-2026-10-20/execution/qualification-lanes` (QL), where L is `C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook`. Manifest SHA-256: `f6c57e9a6f07becc21069338ea4c541a73c95ff150096c1971a83983a796f03d`. All seven entries match exact lengths/hashes; total including manifest is 112,267 bytes. Line references below bind these frozen bytes. Proposed later changes described by the parent were assessed conceptually; none is represented as implemented.

This review only read source, hashes and filesystem metadata. It did not import reviewed modules, execute controls, compile, invoke an engine/build/native runner, use Git, edit source, copy a corpus, launch agents or create chats. The controlling brief/reconciliation and proposed structured-format standard had already been read; existing owner/version contracts remain controlling.

## Required findings

### R1 — P2 Required: history accepts a negative authorization and ignores negative live polls

`history_lane.py:635-638` checks that `host.authorize` returns a dict with a digest-shaped receipt, but never requires `go is True` or the authorized final pin. A return such as `{go:false, receipt_sha256:<valid hash>}` passes this check and execution reaches the harness/runner. A later positive runner preflight does not override a negative LaneHost authorization. In contrast, long/legacy code checks positive GO explicitly.

`history_lane.py:689-693` also discards both `host.poll()` results. The documented contract returns `live` and a receipt; it does not require raising on a negative result. These calls can therefore continue to `_publish_round` after `live:false` unless an unreleased runner independently intervenes.

Require positive same-pin host authorization before accessing the harness/runner, and validate every live poll (or release an explicit, reviewed raise-on-negative contract shared by all lanes). Add source-level negative cases that return valid receipt hashes with `go:false` / `live:false`; the current FakeHost raises in authorize and tests long.poll, so it does not cover this history path. These are static counterexamples, not executed tests.

### R2 — P2 Required: the fixed reader budget has no preflight coverage proof

`history_lane.py:98-100,143-148,427-434,646-650,683-703` fixes each phase at 40 native reader calls / 40,000 warm queries, while 100 publisher offers are spaced 400 ms apart. The publisher horizon is at least 39.6 seconds plus the final operation. Nothing in the closed preflight receipt proves that this finite call budget keeps four reader workers servicing that horizon, accounts for the >15-second old-reader hold, or provides a bounded pacing/exhaustion policy.

This does **not** prove every host will exhaust the readers: slow queries or an explicitly admitted schedule could cover the interval. It proves the source does not require that coverage before beginning 100 irreversible acceptances. Validation rejects exhausted/early-ended readers only after the phase. Fast successful queries can consume the finite calls well before publishers finish; hidden extra calls or arbitrary idle padding cannot repair that evidence.

Release a concrete finite reader schedule/capacity contract before RUN. Preflight must bind the per-worker call/query limits, scheduling/hold/release rules, measured or conservatively bounded phase horizon, exhaustion refusal, native identity evidence and corresponding output forecast. Refuse an inadequate plan before writes. If limits change, update all command/evidence forecasts and pure fixtures under a new reviewed freeze; do not silently renew calls or lower the four-reader requirement.

### R3 — P2 Required: in-host phase evidence and final outer-guard evidence need separate schemas and a real join

`history_lane.py:491-511,703-724` requires closure/guard fields and labels each phase complete while the host is still running inside the outer QPC Job. The source additionally says the final outer two empty Job observations are still required. An actual final outer-run cadence/cap/closure receipt is available only after that host exits. A phase-owned child drain is not the outer Job becoming empty. `guard.sha256` is currently the frozen QPC program hash, not by itself the identity of a completed guard run.

The parent's proposed remedy fits the boundary: use an explicitly **PROVISIONAL_UNQUALIFIED** observed-prefix phase schema containing actual outer run identity, host PID plus creation identity, phase clock interval, authenticated raw observation references, owned-child/IO/handle drain scope, and an absent/pending external final-receipt link. Do not fabricate final closure fields to satisfy the current validator.

A separately reviewed platform/parent post-exit finalizer must then join all 16 exact phase receipt hashes and final source/binary/budget bindings to the actual closed raw QPC receipt. It must validate phase-window bracketing in the declared clock domain, full <=40 ms cadence/missing samples, all RSS/cap/censor results, final census, and two numeric empty Job observations after host exit. Missing windows/final receipt/closure means incomplete. No ninth helper is needed in this packet; the platform-owned finalizer still needs concrete source review/binding. This remedy is **proposed, not present in the frozen file**.

### R4 — P2 Required before RUN: whole-lane output/physical forecasts are not quantitatively tied to the source schedule

`long_lane.py:167-198,205-232,276-320,352-394,428-520` checks individual caps, nonempty phase dictionaries and a `reviewed_source_forecast:true` flag, but does not derive or validate a whole-lane artifact reservation from its actual call counts and limits before restoring S. Its fixed schedule contains 602 lookup engine calls, setup/convert/publish/resolve (606 engine calls before cleanup), up to K cleanup calls, four full oracle outputs, two sample artifacts, inventories, known-request maps, transport metadata and failure/finalization records.

For example, the permitted 2 MiB common engine stdout cap times 602 lookups is already 1,262,485,504 bytes, versus at most 52 MiB assigned evidence. Four permitted 2 MiB oracle outputs consume all of M's 8 MiB before the separately reserved stream subset (2 MiB/12 files) or other metadata. These are permitted cap combinations, not claims about actual output sizes. A host that merely enforces the ceiling would refuse mid-lane after expensive irreversible work; that is not a source-derived prospective fit.

History's `forecast_history:169-212` usefully checks closed component sums, zero future/deletion credit and supplemental headroom. It validates supplied components/proof identifiers, however; the released runner must concretely bind those components to the 1,600 accepted / 4,000 offered / 2,400 terminal schedule, readers, lookups, failed prefixes, controls and all raw output sizes. A digest-shaped string or reviewed flag alone is not that proof.

Provide a bound quantitative forecast (in-source pure calculation or a concretely pinned/preflight-validated artifact) before authorize/restore. Count every physical/output/evidence category once, reserve finalization and failures, keep stream metadata inside M8/40, keep F11 through destination successor+oracle, and reject insufficient finite allocations before corpus writes. Preserve parent-only expiry and zero planned deletion/reassignment credit. Do not treat an unspecified LaneHost implementation as an already-implemented enforcement hook.

### R5 — P2 Required: long warm sample validation does not bind the exact query IDs

`long_lane.py:323-348` verifies six warm categories, 500 timings each and result digests, but never checks a warm row's `id` against REQUESTS. `fake_controls.py:112-114` deliberately creates positive warm rows without any id, so the positive fixture also accepts this omission. A wrong missing/cancelled request can have the same outcome bytes and pass the digest check while measuring a different key.

Require the exact original id for each category and validate the final normalized warm shape; add a wrong/missing warm-id negative. History's validator already checks this at `history_lane.py:317-324`. Keep the finite control proposal <=48 cases/16 MiB: replace low-value direct FakeHost tests or otherwise revise the bounded plan explicitly, rather than silently adding cases beyond its limit. No pure cases were run by this reviewer.

## Concrete release gates; current proposals are not implemented native hooks

README, example, manifest and fake-case plan correctly label SOURCE ONLY, false/null RUN values, and pending v5-R1/startup/control releases. Missing native implementations therefore remain **unqualified prerequisites**, not fabricated passes. Retaining these drafts is distinct from approving RUN.

`long_lane.py:205-232,276-320,448-451,484-501` proposes `lane-invoke-v1` and normalized oracle/cleanup responses. The exact released native adapter must be pinned and reviewed before RUN. It must specify actual engine input/output mappings, including whether publish fields are nested under `request`, how publication/resolve results are normalized, where action/source tags originate, real cleanup terminal fields, raw stdout references, separate clocks and positive native containment/closure evidence. The lane currently expects `raw.action` and a direct accepted publish outcome; those are proposed normalized shapes, not proof that the actual final engine emits them. The example's generic `reviewed-v5-R1-cli` label does not implement this mapping.

Likewise bind one concrete LaneHost implementation for authorize/poll/checkpoint/invoke/record/drain, the exact shared Harness/history-native-v1 methods, Python argv startup gate before first corpus IO, terminal-conflict release, and v5-R1 complete-oracle/baseline normalization. Define the overloaded record label/path + dict/bytes behavior with exclusive destinations and prewrite category caps. Source SHA fields do not establish behavior without those implementations and their reviewed schemas. Native oracle/control/guard behavior remains unqualified until separately exercised under finite GO.

## Supported source observations

- Legacy adapter preserves the authenticated inventory's DFS order (`legacy_archive_restore.py:95-140,176-223`); it does not globally sort transport frames. Canonical destination comparison is sorted separately. The exact archive/manifest/inventory and selected identities are checked before engine opening; writes use a fresh quarantine, bounded reads, exclusive files, exact record hashes and strict EOF. Failure retains quarantine without automatic delete/retry.
- Independently rehashed the existing donor `execution/harness/stream_snapshot.py`: exact `f8d223618d869bf85a11ec648254830e643aa1abbb86374233ac9f31fe46cc20`, 34,729 bytes. Read its reused strict gzip/path/inventory/compare interfaces without importing them. Its reader bounds expansion, rejects concatenation/padding/truncation and unconsumed FNOP bytes; the adapter uses those helpers unchanged. This is source compatibility evidence, not a restore/control run.
- Long order is legacy 50 fresh +500 warm per six original categories, same final candidate/H; force convert with cleanup false; full converted oracle; matched paged samples before cleanup; bounded cleanup; full cleaned oracle/census; f8d stream/restore; full restored oracle; accepted successor only on D; exact resolve and post-successor oracle against top-level baseline; S rehashed. Successor/oracle normalization is still the release gate described above.
- History's plan arithmetic is coherent: two authentic N128 donors at H1002/H10002, 1/4-publisher cells, one control +three online phases each, 100 accepted per phase =1,600 accepted; offers total 4,000, losers 2,400. Control validation requires zero concurrent maintenance; online phases require two actual successor mappings and held-reader/new-reader/publisher overlap. Exact acceptances, raw outcomes, sealed loser receipts and independent boundary oracles are required; hidden retries/censors cannot be relabeled as conflicts. These are source checks, not executed workload evidence.
- Whole source/metadata budgets preserve F09 construction and F11 positive successor/coexistence reservations; no automatic F14/F11 release occurs. Pure controls explicitly exclude filesystem archive writes, engines, native Job/startup/RSS, real restore and native scheduling.

## Caps, incidents and review output

Frozen source groups: long 42,384/49,152 bytes (2 files); history 48,321/49,152 (1); controls 21,562/32,768 (5); total 112,267/131,072 (8). History has only 831 bytes of present headroom: remediation must still honor its source cap or obtain an explicit revised allocation. No source edits were made here.

Preserve both earlier incidents as failures: history 52,692 exceeded 49,152 by 3,540 bytes; README 4,310 exceeded 4,096 by 214. The current smaller freeze does not retroactively qualify either write. Manifest reports the owner's in-memory compilation; this reviewer neither ran nor independently qualified that compilation.

The parent updated promoted membership before final handoff: carry 18 promoted harness files/179,468 bytes, superseding the earlier constant. Before revising these two files, whole P measured 5,439,994 bytes/672 files (already including the initial 22,323-byte review). LandingMETA measured 2,106,107/222 with both R-root and execution-root regular files conservatively included; the larger parent census of 2,130,237 bytes is carried, with no credit for the 24,130-byte difference. Carried current total is 7,749,699. Reserving an additional full 32,768 for this revision without subtracting the existing reports, plus 131,072 for the future packet, gives 7,913,539 <=8,388,608, leaving 475,069 bytes. Source originals remain excluded as instructed; no planned deletion/reassignment credit. Only review.md and evidence.json are written by this review, combined <=32 KiB. Companion evidence records roots, inclusion/exclusion rules and hashes.

**REMEDIATE the frozen packet before treating it as an executable qualification lane.** Proposed fail-closed helpers may remain source-only drafts; no import/control/compile/native/archive/engine RUN is approved by this review. Re-freeze bounded source after the fixes, review the concrete native hooks/post-exit finalizer and source-derived forecasts, then obtain the separately scoped finite control and runtime releases. Do not lower the experiment matrix, shift final authority into a fake host, manufacture final closure, reset the clock or count unrun work as passed.
