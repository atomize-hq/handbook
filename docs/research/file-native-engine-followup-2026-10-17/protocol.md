# Bounded engine follow-up

This is standalone research. The format standard remains proposed. Exact authored
YAML, JSON (including `1.00`), CRLF and Markdown bytes remain immutable inputs;
the application metadata is experimental JSON. There is no production migration,
new query language, general Markdown/YAML parser, automatic merge or daemon.
Authored fixture sources are retained as UTF-8 text bytes. Growth batches append
whitespace to JSON sources; generic edit requests are not format-aware and may
produce text that a document parser would reject. General document/schema
validity after arbitrary edits is outside this proof. The tested global semantic
constraints live in the experimental graph metadata, independently of source
schema interpretation.
The [existing tool inventory](../json-yaml-format-audit-2026-10-02.md#reusable-tooling)
was reviewed: Engine, SDK, Contracts and Dock canonicalizers have different
dependency, numeric, framing and lifecycle boundaries. This isolated executable
retains its baseline serde/sha2 dependency boundary and local byte/protocol
behavior; it neither imports a private owner codec nor standardizes its
experimental encoding for those owners.

Both browsing contracts remain open. A selects immutable authority, then refreshes
a meaningful mutable tree; ordinary reads and copies can mix, lag or be damaged.
`view-audit` checks actual source and derived bytes; `resume` checks and rebuilds the view,
preserving divergent existing bytes in `divergence/` first. An unchanged-path
optimization during ordinary refresh is explicitly not an integrity assertion.
B prepares and destination-verifies a complete readable generation before
selection. One positively selected exact path binds a reader/copy until explicit
retirement. Re-reading the selector per file does not bind a snapshot.

Publication identity, state equality and content ancestry differ. The sole
authority is SELECTED.json -> positively selected publication -> state descriptor
and retained readable closure. Candidates and COW object roots are prepared first;
there is no backend HEAD. Candidate, request, generation, attempt and publication
IDs are separate. Exact expected publication is rechecked under a per-store lock.
Request identity includes the captured base, operations and packet request.
Same-ID/different-intent refuses; accepted same intent resolves to the original
result even beneath successors, cancellation signals, retirement and reclamation.
Receipts are caches. A duplicate non-idempotent command is rechecked under the
admission lock, or after speculative preparation failure, before returning failure.

An absent request is nonterminal while eligible work may select. `cancel-signal`
only interrupts queued work; `cancel` takes coordination and writes the terminal
fence. It cannot cancel an accepted result. Thinking/capture/external work is
outside coordination; only pure fixture commands are explicitly resubmitted.
Slot preparation versus speculative preparation preserves all rejected attempts.
The lock is not FIFO; bounded successful waves do not prove starvation freedom.

Whole manifest stores one complete JSON state per attempt. The reused local
`DiskIndex` is the October 12 small lazy disk B+ tree, with unchanged path-COW and
coalesced batch algorithms, a fresh 8,192-byte value bound, and actual page objects.
Normal document metadata is in leaves; metadata larger than 4 KiB (the overview's
5,000 edges) is an immutable referenced object. Deletions use empty-value
tombstones. A compact descriptor binds consumer/budget/count/root. This is local
experimental code, not imported EchoDB/Dolt source or a new prolly implementation.
The original unused sequential put path remains for reuse clarity (two compiler
dead-code warnings), while workload updates use put_batch.

COW point reads verify the descriptor and selected path (plus a referenced large
metadata object if needed). They do not audit unrelated pages or full history.
Commit validation and readable rendering still load the complete graph, validate
global constraints, and prepare/verify the promised readable closure. Metadata
read/write amplification, point lookups, audits and end-to-end publication are
reported separately. COW cannot remove B's namespace/copy costs or A's packets.

Three operations are distinct: commit with packet=false; packet assembly from a
selected revision; commit+packet. B always materializes all documents, navigation,
and explicit packet status. A without a requested packet does not capture an
assembly; its exact state remains retained. A requested packet retains the
transitive local closure of eight exact ordered inputs (including attachments),
recipe/renderer, omissions/errors, source hashes and output digest. At 10k+ the
overview requires 5,000 constituents. Same semantics apply to A/B in each workload.
On-demand assembly is a separate operation, not a weakening of B publication.

Unix immutable install uses hard_link of a unique fully written, synced staging
file to an absent same-filesystem final name, then directory fsync, stage unlink,
and stage parent fsync. EEXIST checks exact winning bytes; it never replaces an
inode or repairs corruption. Windows retains no-replace MoveFileExW WRITE_THROUGH
and verified same-content winner handling. Nested mkdir retries sync the existing
parent name. Windows directory sync remains explicitly unsupported. These are
process-interruption tests; kernel cache, Windows host and WSL VHDX survive.
No OS reboot/power-cut or macOS persistence qualification is claimed.

The phase oracle forbids selection before selector replacement; visible but
boundary-incomplete is qualified uncertain; successful selection boundary must
retain original result after death and beneath successors. A refresh/reply error
does not undo accepted publication. A successor re-establishes predecessor
selector/name durability under coordination. Larger fixtures target first/middle/
last copy hooks explicitly. Corrupt selected evidence refuses, never rebuilds
authority from arbitrary survivors. Coherent rollback freshness needs an outside
witness and is not proven by local hashes.

Retention proof owns only `disposable/` fixtures. Registration, release,
retirement and collection take the publication lock. Preparation holds a shared
preparation gate until its sealed candidate exists; collector refuses a live
gate, including a paused process. Collector also refuses unsealed preparations
rather than guessing they are dead. Pending sealed candidates protect actual
originals/pages unless explicitly abandoned. Accepted outcome lineage and fences
are never collected. All selected snapshots remain retained unless explicitly
retired; current head cannot retire. Pins cannot silently rebind names. Packet
registrations protect source snapshot metadata until explicit release; emitted
packet copies themselves contain complete local files and remain independently
usable. No cross-store transaction or invisible tool-free reader registration.

The atomic `retired-attempts` record is the authoritative retirement gate;
publication-side retirement is a discoverability cache. Reopen/audit use the gate
even after interruption between records. Historical read returns retired rather
than missing/corrupt. Mark roots include actual live COW pages, metadata pointers,
document originals and pending preparations. Sweeps delete only verified files
beneath a canonically checked disposable root, leaving directories, outcomes,
retirement records and external copies. Interrupted sweep is idempotent. This
proves a small explicit lifecycle, not a production collector or retention policy.

A rebuild may ignore an explicitly retired previous view source and preserve
unknown divergent bytes conservatively. Missing/corrupt non-retired prior
metadata remains retained corruption and blocks refresh/resume rather than
silently repairing authority. Current-source-only rebuilding is not guaranteed
in that corruption case.

Measurements use independent OS processes, ordinary file copies, application I/O
counters and native peak memory (Windows peak working set; Linux maximum RSS),
reported before final stdout serialization. Timers are inclusive/overlapping.
Engine `Instant` and controller `perf_counter_ns` durations are monotonic;
epoch wall-time labels can adjust and are not used to calculate latency. Each controller
collector records process start/end independently; request completion includes
all rejected attempts, controller scheduling, rebasing and pure retries. Busy wait
is recorded on timeout, successful wait and cancellation. Injected lock-acquired
barrier time precedes normal lock-hold timing and is a correctness control, not a
measured admission workload. No device-cold/cache-control or percentile claim.
At 10k+ a ten-second namespace observer records creation/materialization progress
and its own sampling time. Its directory enumeration is not counted as engine
application payload I/O; latency observations include any shared-host overhead.

`metadata-build` has backend-specific scope: manifest covers serialization;
COW covers diff/tree construction and immutable node/indirection installation.
The caller installs the returned full state or small descriptor after that timer.
It is not a matched complete-metadata-I/O timer. Compare complete preparation or
commit clocks and declared I/O counters, not those stage durations alone.

The post-matrix capture amendment records exact stdout as Base64 with byte count,
SHA-256 and parse error before timeout/malformed-response refusal. Earlier measured
sessions use their exact archived controller source and decoded-output format;
they are not retroactively given raw bytes. Both capped WSL A events retain
`output: null`; exact historical stdout bytes were not captured. No missing event
from a parsing failure is known. Synthetic transport probes are separate controller evidence, not
engine performance/correctness observations. New recovery/probe sessions identify
the amended controller and are labeled separately from original measurements.

Tool-free walkthroughs begin at root README, read one selected publication,
retain its exact path and manifest digest, inspect navigation and sources, copy,
and verify copied bytes using only PowerShell/Bash and standard JSON/hash tools.
These are executed extraction checks, not a study of unfamiliar human usability.
The navigation/publication-copy fixtures use 16 research documents. Large-store
packet and point-read observations do not establish 100k human discovery costs.
