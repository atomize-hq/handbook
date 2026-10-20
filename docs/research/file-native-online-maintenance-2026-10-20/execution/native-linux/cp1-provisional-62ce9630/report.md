# Provisional CP1 native Linux build

Source pin `62ce963085ac054bcc34eb21c6733b94db160975`. Workspace release build
passed in 23.0773 seconds using Cargo/rustc 1.89.0, `--locked --offline --workspace
--release -j2`, the new owned target and accepted guard
`528aaa3cf4b3df598f28e037f322cbd79fd9888f9f7edda4c31b1a37e9053a35`.

**This is intermediate build evidence, not qualified CP1 or final evidence.**
The guard observed 234 RSS intervals over 40 ms, maximum 58.0982 ms. Its nominal
50 Hz target did not establish the required cadence under compiler load. This
limitation was reported immediately; units and engine calls did not start. Meta
instructed the worker to preserve the result and wait for the corrected source
pin rather than repeat this build solely for cadence.

- Source-only transport: 296,960-byte tar, SHA-256
  `1fb0e80fe29379ab545df9df4e8fc5ca621489fa73e38aba0acda2e95d197102`.
  All 23 engine/Cargo/consumer blobs match the checkpoint binding before and after
  build. The 24th transported file is that checkpoint binding itself. No Git,
  evidence tree, old target or corpus was copied.
- Native binary: 3,545,928 bytes, SHA-256
  `11c306c4fa30d74a6f83274666024591e0e150674419d1e4dbe06e9a251a34f5`.
  Preserved before any future target overwrite at
  `/home/spenser/file-native-online-linux-20261020/evidence/cp1-62ce-native-binary`.
- Sampled heavy RSS peak 459,894,784 bytes; supervisors 49,221,632 bytes.
  No disk/RSS/free-space censor, positive process closure, all monitors joined.
  These remain sampled observations with the cadence limitation above.
- Target actual after build: 126,797,363 logical bytes/408 files. Fixtures zero.
  Binary preservation used a fresh F12 census and explicit 3,545,928-byte forecast;
  native evidence then measured 3,757,784 bytes/156 files before its new receipt.
- Dependency binding independently checks all 45 locked archive SHA-256 values and
  all 1,530 archived files against the 32 actual host-relevant unpacked crates.
  Only Cargo's own non-archive metadata extras are allowed. Compiler/Cargo and
  linked libc/libgcc/loader paths and hashes are retained. No install/bootstrap.

The sampler was subsequently revised to traverse all owned threads' child lists,
including subreaper-adopted escaped children, rather than scan unrelated host
command lines at each tick. Numeric global closure observations remain separate.
Current revised guard SHA-256:
`4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9`.
It marks a cadence gap unqualified and returns nonzero. Parent re-review is pending.

`../control-smoke-v5/summary.json` records nine passing control checks, including
two CPU-bound children spawned from harness threads, max interval 20.1846 ms and
zero over 40 ms. An additional CPU-child + shared-Cargo-census check records 111
samples, max 20.0841 ms, positive closure and joined monitors. The v4 failed subset
is preserved: an exit-to-zombie stat/status race failed closed, then was corrected
by confirming state on missing memory fields. Live missing memory still refuses.

Raw build stdout/stderr, receipt, compressed per-member RSS, source-before/after
binding, dependency binding and binary-preservation receipt are retained here.
No workspace unit, protocol smoke, capsule, corpus, long-history operation or
engine invocation has occurred. Await the replacement pin and guard recheck;
the final full cache unit remains deferred to same-final-pin qualification.
