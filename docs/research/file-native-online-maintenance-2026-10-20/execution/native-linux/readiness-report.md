# Native Linux preparation receipt

2026-10-20. Status: **provisional CP1 build preserved; revised guard accepted;
thin driver/dispatch review and replacement source pin pending**. This is not
checkpoint or final qualification.

The admitted landing checkout was at
`499619caf9c3663cea467f37a299f7ff1185a835` when inspected during preparation;
it is meta's moving integration branch, not a claim that this worker tested that
engine pin. All worker writes are new files in this `execution/native-linux`
subdirectory. Shared main and prior integration were read only. At the initial
control-preparation checkpoint no engine was built or invoked. A later explicitly
released provisional build is recorded below; no engine source was edited and no
corpus was made or transported.

Parent-reviewed logic `native_guard.py` SHA-256:
`0c1e8b71aa38fd4c02a9fcfd509e66fd4fb1817c81d83a3d7a73cfcc1818a856`.
The `control-smoke-v3/summary.json` hash is
`2b3733641efb63ac255d6daea95b162126ed9cfc47abff0aae5eb5d09315aae9`.

Eight control cases passed: normal exit, nested child, root exit leaving a
new-session descendant, deadline termination, RSS termination, category reservation
overrun, prelaunch free-floor refusal and setup-deadline refusal. Every launched
group has two empty numeric process observations after reaping. Monitors joined
before streams/pidfds closed. Largest sampled interval was 30.9593 ms; none exceeded
40 ms. Raw receipts and compressed per-member RSS samples are retained. These
samples establish control-smoke observations only, not a future scheduling guarantee.

The first reviewed guard's source and receipts remain in `control-smoke/`; the
second revision is retained in `control-smoke-v2/`. The current revision adds the
parent's five requested fixes: absolute setup-inclusive deadline; explicit monitor
shutdown lifetime; positive blocked-launch memory and fail-closed missing live
memory; prospective category byte/file claims; nonce/PID-start/category ownership.

At `1792526982.553802` Unix seconds, before writing that census receipt, the native
runtime held 118,392 logical bytes/108 files (331,776 allocated bytes): source
35,643 bytes/16 files; evidence 82,749 bytes/91 files; target and fixtures zero.
Free space was 87,803,633,664 bytes. See `control-smoke-v3/runtime-census.json`.
The 8 KiB control-only reservation-overrun file was removed after its group closed;
this is not protected corpus reclamation or forecast credit. New reports and
subsequent copies remain separately charged.

Before engine execution: meta must supply the exact CP1 source pin and
source/harness manifest. New source transport is bounded and
source-only; all builds use existing offline locked dependencies, explicit Cargo
PATH and j2. The full old cache-boundary unit is deferred at intermediate CP1;
meta released a 96 MiB final fixture allocation through sealed claim
`44GiB-CENTRAL-001-F08-FINAL-NATIVE` (claim SHA-256
`9b19a7b0eb3fab63f76c53b7b629b18ec1828c3c4346dc4d839380299bd5f984`).
At that configuration checkpoint, the guard SHA-256 was
`e92aa3ffc7d743d2cffac15319fe00b48c0f522d3ff33a36e2480a4983f1062e`.
`final-fixture-config-preflight.json` proves reverting this one constant exactly
reproduces the accepted hash, checks all category limits/free floor and confirms
target/fixtures remain empty. The full cache-boundary unit remains deferred until
the final pin, using `EXP_PACK_TEST_ROOT` support when provided.
Sealed claim 002 subsequently released the C2 Linux target top-up to 256 MiB
(claim SHA-256 `b77352544db52170902dd7b90a76c045943d38eaacb63523db08b71ebd9f2570`).
At that configuration checkpoint the guard SHA-256 was
`528aaa3cf4b3df598f28e037f322cbd79fd9888f9f7edda4c31b1a37e9053a35`.
`target256-config-preflight.json` proves the only additional change is that target
constant, and reversing both configuration changes reproduces the accepted logic
hash. Target and fixtures remain empty; no engine source pin is staged.
That sentence describes the configuration checkpoint only. The later exact-pin
build and current sampler revision are recorded in
`cp1-provisional-62ce9630/report.md`: build passed, sampling cadence gap preserved,
no units or engine invocations, replacement pin awaited. Revised guard SHA-256
`4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9`
passed the v5 controls and was subsequently accepted by parent for replacement-pin
provisional workloads. `adapter-readiness.md` records the prepared thin Linux
driver/capsule dispatcher and their fake-peer-only controls; source review and the
replacement engine/physical-decoder pin remain pending. No old engine pin was rerun.
Full final-pin workspace/strict/packing/recovery and new protocol coverage remain
required. The original 52 applicable packing case IDs are mapped prospectively in
`legacy-case-map.md`; no row is claimed passed.

The historical 40 GiB NO-GO and all old native results remain unchanged. Current
44 GiB admission, inside reserve, protected data, fixed functional/closeout cutoffs,
single long-history lane and same-final-pin qualification obligations still apply.
