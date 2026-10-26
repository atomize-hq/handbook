# Bounded Mac blocker diagnosis — 2026-10-24

No new Cargo invocation, build, test, dependency retrieval, process kill, source/lock edit or supervisor construction occurred. Earlier attempt/report/failures are retained.

## Dependency root cause and correction

I chose `CARGO_NET_OFFLINE=true` as a cache-first attempt. “Installed caches first” did not require offline-only execution; ordinary retrieval was explicitly authorized. I then overstated this resolver error as a host dependency blocker without checking the existing P cache. That was a workflow error, not an engine failure.

The invoked Cargo/rustc1.89.0 toolchain is `/Users/spensermcconnell/.rustup/toolchains/1.89.0-aarch64-apple-darwin`; `.rustc_info.json` shows successful compiler version/target probes, not workspace compilation. Other installed toolchain directories include1.95.0 and1.97.0; no toolchain switch/upgrade was attempted and no version incompatibility was demonstrated.

The lock requires the crates.io registry versions `zstd0.14.3`, `zstd-safe8.2.0`, `zstd-sys2.1.2+zstd.1.5.7`. Default `/Users/spensermcconnell/.cargo` lacks those exact archives/sources and exact index entries. Its cached zstd index extends through0.14.0; Cargo's reported offline candidate0.11.2+zstd.1.5.2 does not mean that is the only index version (offline availability filtering matters).

All three exact archives, extracted sources and index records exist under `/private/tmp/file-native-p-macos-20261020/cargo-private-proposed/registry/{cache,src,index}/index.crates.io-1949cf8c6b5b557f`. Fresh archive SHA256 matches the frozen Cargo.lock:

- zstd0.14.3: `db3ae9bf90af6e40197bef767eeefc46ad6397420fe873830bd85e5fbbfb3c70` (52,615bytes).
- zstd-safe8.2.0: `ded520f16b29e1a9b771c722221b2b0a0398c2316d9bb3c42b458a5414d7434b` (46,799bytes).
- zstd-sys2.1.2+zstd.1.5.7: `481ada9c543b2323cbb7bbdbb5249177fbf0188ed959ed2c15e4b22421c84ed3` (785,774bytes).

**All45 registry package archives in the final lock exist with matching checksums across the two existing caches.** P cache alone covers30; default cache coverage is recorded below. This proves local input availability across roots, not that either Cargo home alone resolves the whole locked workspace or that every crate source/index is ready. No cache was merged/copied, no dependency version substituted, and no private Cargo clone created.

Current runtime environment has `CODEX_SANDBOX_NETWORK_DISABLED=1`. Two bounded ordinary HTTP HEAD probes to `https://index.crates.io/zs/td/zstd` and `https://static.crates.io/crates/zstd/zstd-0.14.3.crate` each exited6 (`Could not resolve host`). Thus **ordinary network resolution is unavailable from this execution runtime now**; this is not evidence the real Mac host has no internet or the registry lacks those versions. Current enforced permission policy is workspace-write, network-restricted, approval=never. The human's full-access preference does not remove those effective tool constraints in this session. No escalation/bypass attempted.

## Process inspection root cause and alternatives

Failed command inside supervisor: `/bin/ps -axo pid=,ppid=,pgid=,rss=`. Python `Popen` raised `PermissionError: [Errno1] Operation not permitted: '/bin/ps'`, so ps never ran. The later direct `ps -axo pid,ppid,pgid,rss,comm` returned shell “operation not permitted: ps” and exit127. This is an executable-launch denial in this runtime, not a negative RSS observation. `/bin/ps` exists, root-owned mode04755; `/usr/bin/top` is also setuid. This is consistent with confined setuid execution, but no sandbox policy/audit log was available to prove the exact deny rule. A general macOS host process-inspection prohibition is **not established**.

Small direct native read-only probes (no experiment imports/child workload) succeeded for the probe process and its existing parent:

- `/usr/lib/libproc.dylib` `proc_pidinfo(pid,3,...,136)` returned136, errno0.
- `proc_pid_rusage(pid,2,...)` returned0, errno0, populated buffer.
- `os.kill(self,0)` succeeded as an existence/permission check; it sent no stop signal.
- `os.wait4` is available for a supervisor that retains its own child handle.

These show existing native identity/rusage APIs are usable without executing setuid ps; they do **not** prove aggregate descendant enrollment, monitoring cadence, forced stop or cleanup under a real build. The previous statement that native RSS supervision as a whole is unavailable was too broad.

Existing reusable implementations already reside at `/private/tmp/file-native-online-macos-20261020/execution/readiness-20261021/native_probe.py` (identity/usage/group API definitions) and `monitor_v20.py` (OwnedTree child traversal using proc_listpids, native snapshot/monitor). They were inspected as text, not imported or run. Stop primitives `Popen.wait`, POSIX process-group signals and wait4 are ordinary existing APIs; successful stop enforcement was not newly tested. There is no reason to start another qualification campaign merely to replace ps with the already existing native API route.

## Original Cargo exit and closure reconciliation

The task-owned supervisor launched Cargo and then failed at its first ps call. It did not save `p.pid`/PGID, birth identity, original wait status or a finally-block cleanup receipt. Its Python handle is gone; the exec tool returned exit1 for Python with no live session ID, not an original Cargo exit receipt. Original Cargo's stdout is empty; stderr is the complete497-byte resolution diagnostic. Its only target output is a1,699-byte `.rustc_info.json` reflecting successful compiler probes. No compiled engine/test binary or fixture appeared.

Read-only `/usr/sbin/lsof -F pcfn -- <build.stdout> <build.stderr>` returned exit1 with no matching rows and no stderr in the current runtime. This is evidence that no matching open log holder was visible to lsof then, **not** recovery of the original Cargo PID, birth or exit. No positively identified original process is available to signal. No process was killed. Original Cargo exact exit and authoritative owned-tree closure remain **UNVERIFIED**; a conventional expected Cargo101 exit is not fabricated. There is no demonstrated resource-limit violation: resource enforcement/closure proof was missing, rather than memory/free-space limits observed exceeded.

## Smallest concrete remedy (not executed)

For the next separately authorized execution, use ordinary Cargo with the locked source and network-capable host runtime, letting its existing default cache be reused and retrieving only missing exact dependencies. Exact matching local archives already exist if normal cache provisioning is preferred; choosing that route must preserve prior cache/evidence and avoid a new private clone. No source, lockfile or toolchain-version repair is justified by this failure.

Use the existing libproc identity/rusage/child-tree APIs instead of setuid ps, retain PID/birth/PGID and original child handle immediately on launch, and keep cleanup/wait in a finally path. Keep finite timeout/RSS/disk limits intact. This is a narrow launch-method correction, not engine work or a new supervisor campaign. The alternative is running the unchanged ps-based method from a host execution context where ps launch is permitted. In either case, preserve the failed attempt and do not rescore it as a build/test result.

No renewed build/test was authorized in this diagnostic. Final31/boundary statuses remain UNRUN. Supporting raw probe records: `cache-probes.json`, `all-locked-cache-check.json`, `combined-cache-check.json`, `index-network.json`, `crate-network.json`, `native-api-probes.json`, `original-log-handles.json`.

Exact-cache archive coverage by root: {"/Users/spensermcconnell/.cargo/": 40, "/private/tmp/file-native-p-macos-20261020/": 30}.

Coordinator delivery failed once: `MCP tool call requires approval, but approval policy is never`. No retry made.
