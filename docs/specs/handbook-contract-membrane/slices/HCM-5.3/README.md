# HCM-5.3 — Process Dock Protocol

**Lifecycle state:** Authority prepared; implementation not started.

HCM-5.2 slice closeout is the satisfied hard predecessor. No HCM-5.3 packet has
started, no product behavior in this document is claimed current, and Phase 5
has not exited. This document is the complete current slice authority for
HCM-5.3 and is not authority for HCM-5.4 or aggregate closeout.

## Outcome

Implement one repository-owned, one-shot process dock that executes an exact
content-addressed native Rust executable bundle inside a default-deny Windows
AppContainer and Job Object boundary. The dock produces only an immutable
pre-spawn execution record, one append-only terminal operational record, and,
after a completely valid `completed` result, an atomic bundle of untrusted
evidence candidates. `handbook-contracts`
remains the sole semantic owner: it independently admits those candidates and
alone owns evidence, verdict, gate, lifecycle, and local-closeout truth.

The selected V1 runtime profile is
`handbook.dock-host.windows-appcontainer-job@1.0.0` for
`x86_64-pc-windows-msvc` on Windows 10 version 1809 / Windows Server 2019 or
newer. Unsupported hosts, unavailable confinement, substituted executable or
runtime identity, and any admission ambiguity refuse before spawn.

## Authority sources and decision lineage

Current authority and source have priority. This slice adopts:

- the ownership, trust, and dependency rules in the current
  [architecture context](../../architecture-context.md) and
  [seam crosswalk](../../seam-crosswalk.md);
- the implemented `handbook-contracts` evaluation-run and sealed
  `EvidenceCandidateSourceV1` boundary established by
  [HCM-5.1](../HCM-5.1/README.md) and [HCM-5.2](../HCM-5.2/README.md);
- the current 62-row reserved operation inventory and generic
  Rust SDK / CLI JSON / Tauri consumer membrane; and
- only the bounded D3/dock lineage from the pre-clean checkpoint identified by
  the current program README: content-addressed bundle identity, an exact typed
  launch vector and runtime closure, default-deny grants, one-shot JSON framing,
  total host outcomes, and atomic completed-result/candidate retention.

The bounded historical passages consulted were `01-target-architecture.md`
(`Dock posture` and non-negotiable invariants 23–25),
`03-seam-crosswalk.md` (`Contract/dock identity and authority coupling`), and
`05-contracts-schemas-and-gates.md` (`Dock capability manifest`, dock
request/result/framing/isolation/timeout/candidate-admission/operation-binding
sections) at that ref. Their design choices are re-adopted only where this
document states them.

The historical source is the Git ref
`archive/handbook-contract-membrane-pre-clean-20260812`, not restored active
authority. This document re-adopts only the listed protocol decisions. It does
not restore historical proof walls, ledgers, status claims, task prompts,
control-plane artifacts, or the former bundled-Python validator choice.

The selected host boundary is grounded in Microsoft's current
[AppContainer isolation](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation),
[AppContainer launch](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer),
and [Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)
documentation. Those sources define platform mechanics only; this authority
fixes the stricter product policy and protocol.

## Runtime decision

### Selected boundary

The product adds the purpose-named crate and package `crates/dock` /
`handbook-dock` at version `0.1.0`. Its owner-source identity is
`handbook.owner.dock@0.1.0`. It owns bundle and host-policy admission, process
launch and confinement, resource enforcement, termination and cleanup, and the
private operational repository. It does not own contract semantics.

The V1 executable profile is exact:

1. `runtime_kind` is `native_process` and `target` is
   `x86_64-pc-windows-msvc`.
   `handbook-dock` uses target-specific `windows-sys = "=0.61.2"`, already
   present in the current lockfile, for only Foundation, Security/AppContainer,
   FileSystem, JobObjects, Pipes, and Threading APIs. Non-Windows builds expose
   the typed unsupported-host refusal and compile no spawn implementation.
2. The entrypoint is a bundle-relative PE executable with an exact SHA-256
   digest. The application name is its staged absolute path, the Windows command
   line contains only its correctly quoted `argv[0]`, and the working directory
   is the private per-run workspace; no additional argument is admitted. There
   is no shell, `PATH` lookup, shebang, interpreter, package manager, or command
   string.
3. A checked-in canonical host-policy asset identified as
   `handbook.dock-host.windows-appcontainer-job@1.0.0` fixes the minimum OS,
   executable target, allowed platform-ABI imports, literal environment,
   limits, and confinement procedure. Its canonical fingerprint is admitted
   and retained in every request basis and execution record.
4. Every non-platform runtime file is listed in the content-addressed bundle
   closure. The platform-ABI allowlist is algorithmic rather than ambient: each
   transitive PE import must resolve either to a listed bundle file or through
   the host's KnownDLL/API-set mapping to a regular file physically below the
   admitted `%SystemRoot%\System32` root. The runtime closure binds the exact OS
   build plus each admitted platform module's normalized name, physical file
   identity, byte length, version, and SHA-256 digest; all are rechecked
   immediately before spawn. Search-order, current-directory, `PATH`, user,
   registry-selected, and plugin resolution are prohibited.
5. The host copies the already-admitted bundle into the private per-run
   workspace, rehashes it, then derives a unique per-run AppContainer SID/token
   without registering persistent profile storage. It opts out of the broad
   `ALL APPLICATION PACKAGES` grant, uses an empty capability-SID list, and adds
   no network capability. It grants that identity only read/execute access to
   the admitted bundle and read access to staged input. It grants no filesystem
   write, repository root, user profile, credential store, named pipe, device,
   registry, or other project path access. OS files required by the admitted
   platform ABI are the only ambient filesystem exception. If the host cannot
   create this low-privilege profile-free boundary, admission refuses.
6. The process is created suspended, assigned before resume to a new Job Object
   with active-process limit `1`, no breakaway, kill-on-close, CPU and memory
   limits, then resumed. Failure to establish every boundary refuses before
   executable code runs.

All other platforms and execution modes return the typed pre-admission refusal
`dock.host_profile_unsupported`; V1 must not fall back to a less-confined
process.

### Rejected alternatives

| Alternative | Decision |
| --- | --- |
| Bundled or ambient Python, including the historical validator host | Rejected. Current planning explicitly restores no bundled-Python choice, the workspace has no selected interpreter artifact, and interpreter/package closure would introduce a second unresolved runtime boundary. |
| Ambient `std::process::Command` native execution | Rejected. Exact executable identity without OS isolation, process-tree control, and resource/cleanup enforcement is not the required default-deny dock. |
| WASI/WebAssembly runtime | Not selected for V1. No current runtime dependency, component model, filesystem mapping, or production bundle exists; choosing one here would expand the dependency and proof surface beyond current source. |
| OCI/container execution | Not selected for V1. It depends on an external daemon/image platform and does not provide a repository-executable current-host path. |
| In-process Rust validator | Rejected as a dock implementation. It bypasses the process protocol and would prematurely decide HCM-5.4 validator proof. |

## Ownership and dependency direction

The permitted crate direction is:

```text
handbook-engine    <- handbook-contracts
handbook-engine    <- handbook-dock
handbook-contracts <- handbook-dock <- handbook-sdk <- handbook-cli
                                      ^
                                      |
                               handbook-tauri
```

`handbook-dock` depends directly on Engine only for its existing generic
safe-path, retained-root, and atomic-durability primitives; that is an
infrastructure edge, never an authority edge. It depends on
`handbook-contracts` for evaluation-run and candidate-source types. It must not
depend on SDK, CLI, Tauri, Flow, or pipeline.
`handbook-contracts` must not depend on the dock. SDK composition calls the
contracts owner to prepare an evaluation run, then calls the dock, then presents
the committed candidate source back to the contracts owner in a later owner
operation. No adapter may construct an admitted evaluation run or canonical
evidence.

The dock owns operational facts such as admitted identities, timestamps,
resource use, process outcome, bounded diagnostics, and candidate bytes. It
never emits or persists canonical evidence, a verdict, a gate result, a
lifecycle transition, local closeout, posture, or parent promotion.

## Canonical protocol identities

All protocol documents are UTF-8 JSON whose canonical identity is
`sha256:` plus lowercase hexadecimal SHA-256 over RFC 8785 JSON Canonicalization
Scheme bytes. Schema refs use full semantic versions; unknown fields, duplicate
keys, non-finite numbers, invalid Unicode, substituted schema refs, or a
fingerprint that cannot be rederived fail closed.

### New domain schema roots

The slice adds exactly these 13 domain/problem/receipt roots:

1. `handbook.dock-capability-manifest@1.0.0`
2. `handbook.dock-bundle-manifest@1.0.0`
3. `handbook.dock-runtime-closure@1.0.0`
4. `handbook.dock-host-policy@1.0.0`
5. `handbook.dock-process-request@1.0.0`
6. `handbook.dock-process-result@1.0.0`
7. `handbook.dock-request-admission-basis@1.0.0`
8. `handbook.dock-result-admission-basis@1.0.0`
9. `handbook.dock-candidate-bundle@1.0.0`
10. `handbook.dock-execution-record@1.0.0`
11. `handbook.dock-terminal-record@1.0.0`
12. `handbook.dock-run-receipt@1.0.0`
13. `handbook.dock-operation-details@1.0.0`

The process request binds the owner-prepared evaluation-run JCS and fingerprint,
contract/claim/subject/case refs, exact capability-manifest, bundle, executable,
runtime-closure, host-policy refs and fingerprints, ordered logical input refs,
requested resource limits at or below the fixed ceiling, and a 128-bit
cryptographically random host-generated run nonce encoded as exactly 32
lowercase hexadecimal characters. It contains no host path or secret. On first
use of an idempotency key, the host durably binds the logical
request fingerprint, generates the nonce, and finishes the admitted process
request; replay resolves that binding before nonce generation and therefore
reuses the exact request rather than conflicting with a newly generated nonce.

A completed process result contains an ordered list of zero or more closed
observation proposals. Each proposal contains the candidate's producer-observed
fact, source, claim-partition, time, and actual-Resolution fields but no
host-owned request/execution binding, final candidate ref, verdict, gate, or
authority claim. Proposal bytes exist only inside the single bounded stdout
result; the child has no filesystem output. After a valid exit, the host binds
each proposal to the evaluation run/lifecycle/contract/definition/subject/case
fields in the owner-admitted request and to the exact producer, request, and
immutable pre-spawn execution records,
constructs the protocol-neutral `EvidenceCandidateV1`, canonicalizes it,
rederives its candidate ref/fingerprint, and rejects extras, duplicates, or
mismatches before a completed commit. Those candidates remain untrusted to
`handbook-contracts`.

`request_fingerprint` is the canonical fingerprint of the complete process
request preimage excluding only its derived `request_ref` and fingerprint.
`request_ref` is
`<evaluation_run_ref>#dock-request/sha256:<request-fingerprint-hex>`.
The immutable pre-spawn execution record is derived after request admission from
the complete admitted request, producer, executable/runtime/host-policy
bindings, reservation identity, and nonce. It contains no result, candidate, or
terminal forward link. The child never authors it. Result, execution-record,
candidate-bundle, terminal-record, and receipt refs use the same canonical rule
over their complete preimages and are anchored below `request_ref` as
`/result/`, `/execution/`, `/candidates/`, `/terminal/`, and `/receipt/`
respectively. The identity order is acyclic:

```text
request -> pre-spawn execution record -> child result
        -> host-constructed untrusted candidates -> terminal record -> receipt
```

For every host-constructed candidate, `producer` is the admitted capability
manifest binding, `request_record` is the exact process-request binding, and
`execution_record` is the immutable pre-spawn execution-record binding. The
terminal record is downstream and cannot appear in a candidate preimage.

The capability manifest binds one bundle and declares only logical input
classes, candidate classes, the singleton launch vector, and requested limits. The
bundle manifest binds the normalized file inventory, entrypoint, and runtime
closure. The runtime closure binds every bundle runtime file and the exact host
policy. These identities are immutable and recomputable without executing the
bundle.

`handbook.dock-operation-details@1.0.0` is the only new dock-specific problem
details root. It supplies a closed code, phase, and safe redacted facts inside
the existing shared operation blocker/refusal/error envelopes. It must not
contain stderr, environment values, host paths, candidate bytes, or secrets.
The V1 shared-problem code set and envelope placement are exact:

| Shared envelope | Closed dock codes |
| --- | --- |
| blocker | `dock.manifest_not_found`, `dock.host_profile_unsupported`, `dock.confinement_unavailable`, `dock.recovery_required` |
| refusal | `dock.request_invalid`, `dock.evaluation_run_invalid`, `dock.evaluation_run_stale`, `dock.identity_mismatch`, `dock.capability_denied`, `dock.path_unsafe`, `dock.limit_exceeded`, `dock.idempotency_conflict` |
| error | `dock.spawn_failed`, `dock.process_failed`, `dock.resource_exhausted`, `dock.timed_out`, `dock.cancelled`, `dock.protocol_error`, `dock.candidate_invalid`, `dock.cleanup_uncertain`, `dock.durability_uncertain` |

An admitted run whose terminal record commits is an ordinary successful
`dock.run` operation result carrying both execution and terminal refs, even
when the recorded process terminal is non-`completed`; only `completed` carries
a receipt and candidate-bundle ref. A failure before durable reservation uses the
listed blocker/refusal, while failure to persist a truthful terminal uses the
listed error with no execution ref or receipt. Child-declared refusal maps to
the recorded `refused` terminal with the single safe producer code
`dock.producer_refused`; arbitrary producer error text is diagnostic only.

## Bundle and path admission

The private V1 root is `.handbook/state/docks/v1`. It contains exact bounded
`manifests`, `bundles`, `execution-records`, `terminal-records`,
`candidate-bundles`, `receipts`, `transactions`, `idempotency`, and `quarantine`
inventories. Per-run staging is under
`.handbook/runtime/docks/v1/workspaces/<request-fingerprint>` and is never an
operation-supplied path. Physical-root retention and canonical-path checks must
match the existing repository authority model.

Admission requires all of the following:

- relative paths use `/`, are non-empty NFC UTF-8 components, contain no `.`,
  `..`, drive/UNC/device prefix, alternate data stream, reserved Windows name,
  trailing dot/space, separator substitution, or case-fold collision;
- every bundle and input entry is a regular file, listed exactly once with byte
  length and SHA-256 digest; directories are implicit and no unlisted file may
  exist;
- symlinks, junctions, mount points, reparse points, hard links, sparse/device
  files, and any final physical path outside its admitted retained root refuse;
- the staged copy is created without following links and is rehashed after copy
  and before spawn; changed source, destination, root, or manifest identity
  refuses;
- logical inputs are resolved by the SDK/owner boundary into bounded public or
  owner-approved bytes. The child never receives a repository path;
- no secret, authority token, credential, idempotency key, full environment,
  user profile, or source repository metadata is staged; and
- V1 ships no production validator manifest or bundle. `dock.manifest.list` may
  truthfully return an empty page until HCM-5.4 installs an independently
  authorized manifest. Tests may use private fixture bundles that cannot enter
  the production manifest inventory.

Fixed bundle and staging ceilings are: 256 bundle files, 64 MiB per bundle file,
256 MiB aggregate bundle bytes; 64 input files, 16 MiB per input file, and
64 MiB aggregate input bytes. Exceeding any count or byte ceiling refuses or
terminates according to whether execution has begun.

## Capability and process policy

The policy is default deny. V1 admits only:

- read/execute of the exact staged bundle;
- read of the exact staged input files;
- exactly the inherited stdin/stdout/stderr handles required by the protocol;
  no other inheritable handle is admitted.

Network, loopback, DNS, process spawn, shelling out, IPC/named pipes, devices,
registry mutation, repository access, user-profile access, and environment
inheritance are denied. No clock value is treated as trusted protocol evidence;
the host records its own monotonic durations. The child environment is replaced
with exactly `LANG=C`, `LC_ALL=C`, `TZ=UTC`, and `NO_COLOR=1`; the executable
identity, input logical refs, and request values come through the typed request
rather than environment variables. Input refs resolve to normalized
workspace-relative read-only paths; no output path exists.

The child protocol is one-shot:

1. host writes exactly one bounded `handbook.dock-process-request@1.0.0` JSON
   object plus one LF to stdin and closes stdin;
2. child writes exactly one bounded `handbook.dock-process-result@1.0.0` JSON
   object, optionally followed by one LF, to stdout and then exits;
3. leading/trailing non-whitespace, a second value, invalid UTF-8/JSON/schema,
   an identity mismatch, a nonzero exit with a result, or result/exit
   disagreement is `protocol_error`; and
4. stderr is diagnostic only and can never alter a terminal or supply a result.

The stdin request is capped at 1 MiB, stdout result at 16 MiB, and stderr at
256 KiB. The host drains stdout and stderr concurrently to avoid pipe deadlock.
Raw streams are never persisted or returned
through an ordinary operation; the record retains only byte counts, SHA-256
digests, truncation flags, a closed diagnostic code, and an allowlisted redacted
summary. A limit breach terminates the job and can produce no candidates.

The wall-clock ceiling is 300 seconds, CPU ceiling 120 seconds, job memory
ceiling 512 MiB, active-process ceiling 1, cancellation grace 5 seconds, and
cleanup deadline 30 seconds. A manifest may request lower values only.

## Terminal, cancellation, and cleanup semantics

The dock's closed host outcomes are `completed`, `refused`, `timed_out`,
`cancelled`, `failed`, and `protocol_error`. The child result may declare only
`completed`, `refused`, or `cancelled`; the host independently derives the final
outcome in this first-match order:

1. cleanup uncertain or a possible live process/workspace escape -> `failed`;
2. admitted wall/CPU/resource timeout -> `timed_out`;
3. host cancellation observed before a valid completed commit -> `cancelled`;
4. spawn failure, crash, nonzero exit, job violation, I/O failure, atomic-commit
   failure, or durability uncertainty -> `failed`;
5. zero exit with invalid, multiple, oversized, or identity-substituted protocol
   output -> `protocol_error`;
6. valid child `refused` -> `refused`;
7. valid child `cancelled` -> `cancelled`;
8. valid child `completed`, valid candidate list, and successful atomic commit
   -> `completed`;
9. any unclassified state -> `protocol_error`.

Timeouts use a host monotonic clock. Cancellation is an internal idempotent
host token attached to the in-flight request; HCM-5.3 adds no public
`dock.cancel` operation. On timeout, cancellation, protocol error, or failure,
the host closes stdin, requests graceful termination for at most 5 seconds,
closes the kill-on-close job, waits for the entire job to become empty, closes
all handles, removes the workspace access grants, frees the per-run token/SID,
verifies that no AppContainer profile/storage exists, and removes the workspace
within 30 seconds. Cleanup uncertainty
forces `failed`, quarantines the workspace under the private root, withholds a
receipt, and blocks replay until deterministic recovery resolves or retains the
quarantine. No terminal may be upgraded by late child output.

## Atomic records and candidate handoff

Pre-admission refusal writes no execution record and returns no receipt. Once
spawn admission is durably reserved, exactly one immutable pre-spawn execution
record exists and exactly one terminal record must eventually exist. The
terminal record includes the pre-spawn execution binding, admitted request and
host-policy identity, timestamps/durations, resource counters, terminal,
bounded diagnostic metadata, cleanup disposition, and either:

- for `completed`, the request-admission basis, result-admission basis, exact
  process result and candidate bundle in one atomic
  durable transaction; or
- for every other terminal, no candidate bundle and no candidate-source entry.

The completion receipt is exposed only after the transaction, transaction
inventory, terminal record, candidate bundle, and idempotency ledger have all
been durably committed and re-read. There is no partial-evidence receipt and no
receipt for uncertain durability. Candidate limits are 128 candidates,
512 KiB per canonical candidate, and 8 MiB aggregate candidate bytes.

The dock implements the current contracts-owned `EvidenceCandidateSourceV1`
seam as a read-only view over committed completed records. It supplies exact
candidate JCS, evaluation-run JCS, producer/request/execution bindings,
resolution ceiling, and a sealed Engine `contract_evidence_source` admission.
The host obtains that lifetime-sealed admission after the completed atomic
commit using a host-held Engine credential for source actor
`handbook.owner.dock@0.1.0`; neither the credential nor sealed witness reaches
the child or persists.
The contracts owner rederives every identity and independently rechecks source,
lifecycle, freshness, currentness, Resolution, and evidence policy before any
canonical evidence append. The dock never calls a canonical evidence writer and
there remains no public candidate writer.

## Replay, recovery, and concurrency

`dock.run` requires an opaque idempotency key. Only its canonical SHA-256
fingerprint may persist. The same key plus the same admitted request fingerprint
returns the exact committed terminal and, when completed, receipt without
respawn; the same key with a different request is a typed conflict and writes
nothing. An evaluation-run
and request fingerprint is single-flight: concurrent identical callers join or
replay the one reservation and may not spawn twice.

Different admitted requests use disjoint workspaces and may execute concurrently.
Locks protect only reservation, idempotency binding, and atomic commit; no
repository-global lock is held while child code runs. Recovery enumerates exact
bounded inventories for transactions, execution records, terminal records,
candidate bundles, receipts, idempotency ledgers, workspaces, and quarantines.
It rederives every canonical
identity, completes only provably committed transactions, removes provably
unlaunched staging, converts a provably dead admitted run to `failed`, and fails
closed on a live/unknown process, extra/missing entry, substituted root, broken
cross-reference, or uncertain durability. Recovery never reruns a child and
never manufactures candidates or a receipt.

`dock.manifest.list` and `dock.manifest.read` are safe snapshot reads with
deterministic order, bounded pages, snapshot-bound cursors, and no child launch.
They expose public manifest identity and declared capability only, never bundle
bytes, host paths, private policy details, or operational records.

## Exact outward membrane closure

HCM-5.3 closes the three existing reserved inventory rows and adds no row, fixed
use case, transport, command, Tauri command, or installed-skill route:

| Operation | Class | Idempotency | Transport evidence | Write set |
| --- | --- | --- | --- | --- |
| `dock.manifest.list` | `read_only` | `safe` | `[cli_json, rust_sdk, tauri]` | none |
| `dock.manifest.read` | `read_only` | `safe` | `[cli_json, rust_sdk, tauri]` | none |
| `dock.run` | `append_only` | `idempotency_key_required` | `[cli_json, rust_sdk, tauri]` | private dock reservation/transaction/execution/candidate/idempotency records only |

The six new operation roots are mechanically exact:

- `handbook.operation.dock-manifest-list-request@1.0.0`
- `handbook.operation.dock-manifest-list-result@1.0.0`
- `handbook.operation.dock-manifest-read-request@1.0.0`
- `handbook.operation.dock-manifest-read-result@1.0.0`
- `handbook.operation.dock-run-request@1.0.0`
- `handbook.operation.dock-run-result@1.0.0`

Together with the 13 domain roots, this adds exactly 19 schema roots. Final
HCM-5.3 truth is therefore exactly 62 inventory rows, 31 capabilities, and
176 deterministic schema roots. The three rows change from reserved/deferred to
current owner `handbook.owner.dock@0.1.0`; all other rows, capability identities,
schema bytes, fixed-use-case omissions, and transport vectors remain unchanged.

The existing generic SDK consumer, four CLI capability actions, and three Tauri
commands must discover, schema-fetch, invoke, and re-admit all three operations
without a dock-specific consumer branch. CLI preserves one invocation envelope
plus LF and established exit mapping; Tauri preserves its existing IPC framing.
No adapter launches the child or interprets dock semantics.

## Strict packet order

Packets are implementation units and must execute strictly in this order. Each
packet is one independently reviewable commit, returns control to the meta task,
and may start only after its predecessor checkpoint is accepted. No packet may
claim slice completion.

| Packet | Title | Required outcome | Permitted just-in-time sources |
| --- | --- | --- | --- |
| 1 | Protocol identity and operational-owner seam | Add the purpose-named dock crate, dock-owned protocol DTOs/identities, the narrow contracts-owned evaluation/candidate seam, exact dependency edges, closed host/problem terminals, and compile-time owner non-authority tests; no process launch or outward operation yet. | This authority; current HCM-5.1/HCM-5.2 owner types; current operation identity conventions; bounded pre-clean D3 identity/framing sections only. |
| 2 | Manifest, bundle, and host-policy admission | Implement the exact manifests, canonical fingerprints, private inventories, retained-root/path/PE-closure checks, fixture-only bundle assembly, fixed limits, and default-deny host-policy admission; no executable code is run. | Packet 1 checkpoint; current Engine safe-path/durability primitives; bounded D3 manifest/runtime-closure sections; official Microsoft AppContainer isolation and launch documentation. |
| 3 | Native AppContainer process executor | Implement suspended AppContainer launch, Job Object assignment, literal environment, one-shot pipes, concurrent drain, total outcomes, timeout/cancellation/resource enforcement, process-tree termination, and cleanup/quarantine tests. | Packets 1–2 checkpoints; official Microsoft AppContainer, process creation, restricted token, Job Object, and resource-limit documentation; no historical validator or bundled-Python material. |
| 4 | Atomic run repository and candidate handoff | Add reservations, immutable pre-spawn execution records, idempotency/single-flight, atomic terminal records, completed-only host-bound candidate bundle and receipt, bounded recovery, and the exact read-only `EvidenceCandidateSourceV1` adapter with contracts-owned re-admission proof. | Packets 1–3 checkpoints; current HCM-5.1/HCM-5.2 retained-root repository, recovery, evaluation-run, and candidate-source source/tests; bounded D3 total-record/handoff sections only. |
| 5 | Three-operation SDK, schema, and witness closure | Close the three reserved rows, add exact 19 schema roots and three capabilities, map the owner through SDK DTOs/dispatcher, and prove deterministic `62 / 31 / 176` witness truth and unchanged unrelated rows/transports. | Packets 1–4 checkpoints; current HCM-4.2/HCM-4.5 and HCM-5.1/HCM-5.2 membrane/schema/admission conventions; current generated-asset tooling. No historical source is needed. |
| 6 | Generic consumer and integrated slice proof | Prove list/read/run, replay/conflict, refusal/failure/timeout/cancel/protocol/cleanup/recovery, completed-only candidate handoff, secrets/path/process/network denial, and discovery/schema/invocation parity through Rust SDK plus unchanged generic CLI/Tauri consumers; update current product/navigation truth without aggregate closeout. | Packets 1–5 checkpoints and current product tests; no new design or historical source. |

## Cross-packet invariants

Every packet must preserve these invariants:

- contracts owns semantic truth; dock owns operational records and untrusted
  candidates only;
- no partial candidate bundle, false receipt, public candidate writer, verdict
  persistence, or adapter authority can appear;
- bundle, runtime, host policy, request, result, record, and receipt identities
  remain exact, versioned, canonical, and independently rederived;
- unsupported or uncertain confinement, root, durability, cleanup, identity, or
  capability state fails closed without fallback;
- network and process spawn remain denied, secrets and raw diagnostics never
  cross or persist, and all I/O/resource bounds remain enforced;
- the current generic consumer boundary and all unrelated inventory/schema/
  transport truth remain unchanged; and
- HCM-5.4, Flow/pipeline adoption, posture/promotion, publication, GUI, and
  Phase 6 remain outside every packet.

## Acceptance conditions

HCM-5.3 may enter aggregate closeout only when all 19 conditions are current:

1. `handbook-dock` exists at the exact inward dependency position, and crate-edge tests prove no reverse or Flow/pipeline/adapter authority edge.
2. All 19 new schema roots and every manifest/bundle/runtime/request/result/problem/record/receipt ref and fingerprint are exact, versioned, canonical, recomputable, acyclic, and substitution-tested.
3. The only executable profile is the selected native `x86_64-pc-windows-msvc` AppContainer/Job profile; bundled/ambient Python, shell, `PATH`, interpreter, and unconstrained fallback are impossible.
4. Bundle and input admission proves exact inventory, digest, physical root, no traversal/case collision/link/reparse/hardlink/extra file, staged rehash, transitive bundle-or-exact-System32 PE closure, and fixed count/byte ceilings before spawn.
5. Every run binds a current owner-prepared evaluation run and exact contract/claim/subject/case, capability, executable, runtime closure, and host-policy identity; substitution or stale basis refuses.
6. AppContainer capability and DACL tests prove profile-free per-run identity, broad-package opt-out, default-deny filesystem/registry/device/IPC posture, no inherited handle beyond the three pipes, and unconditional network/loopback/DNS denial.
7. Job tests prove assignment-before-resume, active-process limit 1, no breakaway, kill-on-close, fixed memory/CPU/wall limits, and child-spawn denial.
8. One-shot UTF-8 JSON framing, literal environment, concurrent pipe drain, stdout/stderr caps, inline candidate/result limits, and raw-diagnostic non-persistence are adversarially tested.
9. The first-match terminal table is exhaustive and deterministic across refusal, timeout, host cancel, crash/nonzero exit, resource violation, malformed/multiple/substituted result, valid refusal/cancel, completed commit, and fallback.
10. Timeout/cancellation closes input, terminates the entire job, waits for empty, frees all handles/token/SID/grants, proves no persistent AppContainer profile/storage, removes or quarantines the workspace, and cannot be upgraded by late output; cleanup uncertainty is `failed` with no receipt/candidates.
11. An admitted run yields exactly one immutable pre-spawn execution record and one atomic terminal record; only a fully valid completed transaction can atomically retain request basis, result basis, process result, host-bound candidate bundle, idempotency ledger, terminal record, and receipt.
12. Failed/refused/timed-out/cancelled/protocol-error or durability-uncertain runs expose no candidate-source entry, canonical evidence, partial evidence, or receipt.
13. The dock's read-only candidate source provides exact committed provenance and sealed producer admission, while `handbook-contracts` independently re-admits and alone creates canonical evidence/verdict/gate/local-closeout truth.
14. Same-key/same-request replay cannot respawn and returns exact truth; key conflict writes nothing; identical concurrency single-flights; disjoint concurrency uses isolated workspaces and bounded commit locks.
15. Recovery checks exact bounded inventories/cross-references, never reruns or manufactures truth, resolves only provable state, and fails closed on live/unknown processes, extras, substitution, or uncertain durability.
16. Exactly the three reserved dock rows become current with the declared classes/idempotency/write sets and `[cli_json, rust_sdk, tauri]`; no operation, fixed use case, transport, CLI route, or Tauri command is added.
17. Production witness truth is exactly `62 / 31 / 176`, all unrelated rows/schema bytes/capabilities/transports remain unchanged, and production manifest discovery is truthfully empty until a later authorized validator exists.
18. Rust SDK and unchanged generic CLI/Tauri consumers prove discovery, exact schema retrieval, invocation, re-admission, framing, replay/conflict, every terminal, and non-leakage without a dock-specific branch.
19. Focused, crate, workspace, generated-asset drift, adversarial confinement, integrated consumer, and repository-diff checks pass; one aggregate causal review has no unresolved P1/P2 and no out-of-scope authority or behavior is claimed.

## Aggregate closeout rule and HCM-5.4 handoff

After Packet 6 returns, the meta task—not Packet 6—must recheck all 19
acceptance conditions against the aggregate diff, run one focused causal review,
resolve every P1/P2, run required GitNexus change detection, and create the one
HCM-5.3 slice-closeout commit/status update. Packet commits and local packet
checkpoints are evidence, not aggregate closeout.

Only that closeout can satisfy the HCM-5.4 predecessor. The handoff to HCM-5.4
is the exact native process profile, empty production manifest inventory, three
generic dock operations, completed-only candidate-source boundary, and
`62 / 31 / 176` product truth. HCM-5.4 must separately authorize and install the
first real native Rust validator bundle and prove its candidates through current
contracts-owned admission/verdict/gate semantics. It may not infer Python,
change the dock runtime, or treat a dock completion as validator correctness.

## Non-goals

This slice does not:

- implement or select the HCM-5.4 validator or validator correctness proof;
- append canonical evidence, persist verdict/gate/local-closeout, promote a
  parent, change posture, or add lifecycle authority;
- adopt the dock in grounding, Flow, pipeline, or another workflow;
- publish a crate/bundle, establish an external bridge or adopter, add a GUI,
  or start Phase 6;
- add a public cancellation, candidate-write, execution-record-read, bundle
  install, manifest mutation, or administrative operation;
- expose raw stdout/stderr, host paths, bundle bytes, secrets, private records,
  or host-policy internals through the ordinary membrane; or
- restore historical control-plane material, proof ledgers, task prompts,
  statuses, or the bundled-Python design.

## Stopping conditions

Stop the active packet and return to the meta task without widening scope when:

- an exact required identity, limit, terminal, owner boundary, or accepted
  packet checkpoint conflicts with current source;
- the selected AppContainer/Job profile cannot enforce network denial,
  assignment-before-resume, single-process/no-breakaway, exact filesystem
  grants, resource limits, or deterministic cleanup on the supported host;
- implementation would require a bundled/ambient interpreter, shell, external
  container daemon, new runtime profile, new public operation, new transport,
  or changed schema/count truth beyond this authority;
- a repository, transaction, process, cleanup, or idempotency state is uncertain
  and cannot fail closed without producing a false record, receipt, or candidate;
- HCM-5.4 validator semantics, posture/promotion, Flow/pipeline, publication,
  GUI, external adoption, or Phase 6 becomes necessary to pass the packet; or
- a focused review finds an unresolved P1/P2 or GitNexus change detection shows
  an unexpected symbol/execution-flow effect.

Recoverable private helper choices that preserve every fixed outward identity,
limit, dependency, terminal, write set, and acceptance condition may be decided
inside the active packet and recorded in its checkpoint. Any change to those
fixed boundaries requires returning to the meta task for authority repair.
