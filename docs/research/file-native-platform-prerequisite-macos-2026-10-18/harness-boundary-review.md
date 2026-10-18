# Adapter boundary review

The exact engine baseline is `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`.
No engine, dependency manifest/lock, copied original helper, or owner file is edited.
The isolated workspace imports serde/serde_json/sha2/fs2 and its local modules;
it does not import production Handbook crates. The format standard is proposed;
Contracts JSON and existing owner/versioned identities remain controlling.

At start GitNexus on this Mac reported its Handbook index at `e6513bfb...`, 160
commits behind the then-active checkout. The pinned engine query returned no relevant process.
The pre-adapter impact request for `Driver` returned target-not-found and UNKNOWN
risk. This is unknown coverage, not zero impact. Reindexing a shared checkout is
outside this prerequisite. Explicit inspection covers experiment.Driver inherited
call/start/finish/record, baseline helpers, workloads, regression entrypoints and
the new adapter. No core-symbol edit is needed.

`native_runner.py` subclasses the pinned Driver, substitutes it only in that
Python interpreter, and dispatches the original correctness functions unchanged.
It replaces obsolete paths/160 GiB guards, applies declared 120/300-second caps,
and records native source/session identity and process resource usage. Original
assertions, checkpoint hooks, positive history proof, request outcomes, packet
checks, closure work and engine synchronization remain unchanged. A fresh run
directory is required. Root commands must resolve strictly beneath the newly
owned runtime root. Native wait4 records the exact waited child's resource
usage; Darwin ru_maxrss is bytes. Missing resource data remains null.

The allocation guard checks free space on every engine launch, recomputes owned
file allocation at most every five seconds, and reserves an additional 512 MiB
inside the 6 GiB cap for the bounded interval and at most eight small correctness
workers. Phase completion records a fresh inventory. Only original disposable
lifecycle fixtures permit reclamation. No unrelated checkout/runtime is cleaned.
The new scheduler serializes engine phases, preserves logs/statuses on failure,
and kills only its own process group if its declared outer cap expires.

The smoke reuses original request/batch/packet helpers with four 1k project
stores, six updates (1/8/64 both packet flags), initial/current requested packets,
and three fresh-process reads per binding/key. Each store must retain seven
publications. No old growth matrix/fill/history or 100k initialization is invoked.
The optional 10k pair is not allocated initially.

The Bash adaptation replaces only `mapfile` with an equivalent Bash 3.2 array
read in the current shell. It still positively binds one publication, exact
closure path and manifest digest before extraction; it invokes no engine.
Python standard-library checks do not constitute unfamiliar-human usability.

Native API inspection found Rust 1.89's Apple File::sync_all calls
fcntl(F_FULLFSYNC), confirmed by the native binary's fcntl argument 0x33 and
successful direct file/directory probes. Source reference:
https://github.com/rust-lang/rust/blob/1.89.0/library/std/src/sys/fs/unix.rs#L1129-L1139
The hard-link implementation uses linkat on this build; selector replacement
uses rename and fs2 coordination uses flock. Success is process/API evidence;
there is no reboot/power-cut persistence qualification.

Publication/outcome lineage is retained indefinitely in this bounded proof.
Complete markers do not assert continuing integrity, COW point reads do not
audit unrelated pages, and both browsing contracts remain open.
