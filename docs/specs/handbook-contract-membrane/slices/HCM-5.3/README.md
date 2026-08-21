# HCM-5.3 Process Dock Protocol

This document owns the outcome and packet authority for HCM-5.3. It defines
exactly one slice outcome: a portable, language-neutral one-shot process
protocol plus a first-party Rust host/executor that can later run the selected
HCM-5.4 JSON Schema validator without moving contract, evidence, verdict, gate,
lifecycle, closeout, or promotion truth out of `handbook-contracts`.

It does not authorize the JSON Schema validator, a production dock manifest or
bundle, a product contract, or another Phase-5 slice.

## Current admission state

**Packets 1–5 and the aggregate HCM-5.3 closeout are complete.**

The hard predecessors are satisfied. The completed
[`HCM-5.1`](../HCM-5.1/README.md) and
[`HCM-5.2`](../HCM-5.2/README.md) authorities establish the inward
`handbook-contracts` owner, exact active-after-independent-lock evaluation-run
basis, protocol-neutral untrusted candidate boundary, immutable canonical
evidence, closed verdict/gate/local-closeout truth, and the current exact
predecessor product state of 62 operation rows, 28 capabilities, and 157 schema
roots. Packets 1–5 preserve those predecessor identities while implementing the
exact three Dock capabilities and 11 roots for current `62 / 31 / 168` truth.
Production manifest inventory remains exactly empty; process completion proof
uses test-only native helpers.

The five packet commits returned to the meta task, which completed the aggregate
acceptance audit, verification/review, and single slice closeout recorded below.
HCM-5.4 authority is not prepared, and its validator is not implemented.

## Outcome and ownership

Create a purpose-named operational crate at `crates/dock` with package identity
`handbook-dock` version `0.1.0` and owner-source identity
`handbook.owner.dock@0.1.0`. It owns only:

1. exact dock-manifest and process-protocol identities;
2. an immutable manifest registry, initially empty in production;
3. a repository-bound operational request/result/execution-record and
   untrusted-candidate observation store;
4. direct first-party process launch, bounded I/O/time handling, cancellation,
   cleanup, and operational outcome classification; and
5. the candidate-source adapter that lets `handbook-contracts` independently
   resolve and admit a completed candidate.

The selected dependency direction is:

```text
handbook-engine generic authority / durability facts
    <- handbook-contracts semantic contract / evidence / verdict / gate owner
        <- handbook-dock operational protocol / executor / candidate source
            <- handbook-sdk outward DTOs and three ordinary operations
                <- existing generic CLI and Tauri capability transports
```

Arrows point from consumer to dependency. `handbook-dock` may depend inward on
`handbook-contracts` and narrow generic Engine facilities. `handbook-sdk`
depends outward on the dock. Engine, Contracts, Flow, and pipeline never depend
on SDK or a transport; Contracts never depends on the dock. The dock may
implement or adapt the existing Contracts candidate-source port, but it cannot
construct `EvidenceRecordV1`, a claim verdict, gate result, local-closeout
result, lifecycle transition, receipt for a Contracts mutation, or posture or
promotion truth.

## Current and bounded adopted evidence

Current source and tests remain product truth. This authority is grounded in:

- the current program [`README`](../../README.md),
  [`phase-and-slice map`](../../phase-and-slice-map.md),
  [`implemented baseline`](../../implemented-baseline.md),
  [`architecture context`](../../architecture-context.md), and
  [`seam crosswalk`](../../seam-crosswalk.md);
- the completed HCM-5.1/HCM-5.2 owner authorities and current source under
  `crates/contracts/`;
- the exact three reserved `dock.manifest.list`, `dock.manifest.read`, and
  `dock.run` rows in `crates/sdk/src/contract_membrane/operation.rs`;
- the current SDK operation-admission, dispatch, deterministic schema,
  production-witness, and generic-consumer precedents;
- the current `jsonschema` Draft 2020-12 practices in
  `crates/engine/src/schema_registry.rs` and
  `crates/sdk/tests/contract_schema_generation.rs`, including exact dialect,
  bounded local-ref closure, in-memory registry construction, and refusal of
  external retrieval; and
- the current workspace and directly affected crate manifests.

The archive ref `archive/handbook-contract-membrane-pre-clean-20260812` is
adopted only for the language-neutral one-request/one-result process framing,
exact manifest/bundle/launch identity, completed-only candidate rule, executor
non-authority, and selected `handbook.dock.json-schema@1.0.0` proof target in
the bounded sections named by the HCM-5.3 request. Current request authority
replaces the archive's interpreter, broad isolation, and first-implementation
choices with the narrower portable Rust path below. Historical status,
orchestration state, packet counts, acceptance counts, schemas, host profiles,
and completion claims are not adopted.

Retained design lineage additionally supports JSON Schema as the portable shape
layer, Rust as the supported product runtime, and the legacy Python harness as
reference material only. None of that lineage is executable authority by
itself.

## Threat model and enforcement boundary

The first protected path is a repository-owned, first-party, offline,
deterministic, read-only JSON Schema validator witness. It is not a marketplace,
an arbitrary-code service, or a general hostile-plugin sandbox.

### Trust boundaries and protected assets

Untrusted inputs are manifest and bundle bytes, claimed fingerprints, SDK DTOs,
evaluation inputs, protocol JSON, instance/schema payloads, process stdout and
stderr, exit status, timing, candidate observations, repository bytes, and raw
idempotency keys. Protected assets are exact contract/lifecycle/currentness
truth, semantic time, raw credentials and keys, the retained repository root,
operational-record integrity, canonical evidence, verdict/gate/local-closeout
truth, and mutation authority.

The portable v1 host must enforce what it claims:

- verify exact manifest, bundle-file inventory, entrypoint, fixed argument
  vector, and executable-byte fingerprints before body access or launch;
- launch the exact resolved executable directly, never through a shell,
  command string, shebang lookup, package manager, ambient executable search,
  or inherited search path;
- clear the inherited environment, set only protocol-defined constant values,
  use an explicit private working directory, and provide only bounded inline
  JSON inputs over stdin; no repository path, home path, credential, proxy,
  package state, or ambient schema location enters the request;
- enforce request, result, stdout, stderr, diagnostic, candidate, count,
  wall-time, and cancellation-grace bounds with checked arithmetic;
- treat zero exit plus one exact valid completed result and confirmed cleanup as
  the only candidate-producing host outcome; and
- persist no partial candidate after refusal, cancellation, timeout, failure,
  protocol error, durability uncertainty, or cleanup uncertainty.

V1 does **not** claim portable operating-system enforcement of filesystem,
network, system-call, child-process-tree, CPU, or memory isolation. The protocol
requires `network: denied`, the host clears proxy and ambient environment, and
only a current exact first-party implementation bundle may be admitted, but
those controls are not a hostile-code confinement boundary. The first
validator's implementation and dependency/source gates must prove it contains
no network, arbitrary-file, process-spawn, executable-hook, or dynamic-loading
path. Independently admitted third-party code and stronger host confinement
require later explicit authority and may not weaken or silently reinterpret
this v1 protocol.

Abuse cases that must fail closed include manifest/bundle/entrypoint
substitution; ambient lookup; shell interpolation; inherited-secret exposure;
wrong request/result/candidate/run binding; duplicate JSON members; extra
stdout; invalid UTF-8 or JSON; output overrun; nonzero exit; late result after a
deadline or cancellation; idempotency conflict; candidate transplantation;
record tampering; and uncertain durable commit or cleanup.

## Manifest, protocol, and runtime boundary

### Exact manifest and implementation identity

`DockManifestV1` is immutable exact-definition data identified by a stable
lowercase dot-separated `dock_id`, full SemVer, derived
`dock_ref = dock_id + "@" + dock_version`, and a deterministic lowercase
SHA-256 fingerprint over RFC 8785 canonical semantic bytes excluding only the
fingerprint itself. V1 binds:

- `handbook.dock.process-json@1.0.0`, the exact request/result schema refs, and
  owner-native protocol-shape fingerprints; the SDK schema manifest separately
  binds checked-in schema bytes and never authorizes the dock owner;
- one native first-party implementation bundle ref/fingerprint;
- one ordered, duplicate-free normalized safe-relative bundle-file inventory
  with file length and SHA-256, one relative executable entrypoint and matching
  digest, one exact fixed argument vector and its fingerprint, and one exact
  supported target tuple;
- supported evidence kinds, input roles/media, actual Resolution ceiling, and
  deterministic/offline/read-only declarations; and
- positive protocol ceilings for request/result/diagnostic/candidate bytes and
  counts, wall timeout, and cancellation grace.

Only `native_process` is admitted by this v1 first-party implementation path.
The protocol is language-neutral: another implementation may conform later,
but implementation language, private host mechanics, filenames, and target
layout are not semantic authority. Ranges, `latest`, changed bytes behind an
exact ref, unknown required fields, extra/missing bundle members, unsafe paths,
entrypoint or argument substitution, unsupported target, or an unregistered
manifest refuse before launch.

Production manifest inventory is exactly empty after HCM-5.3. Test-only helper
manifests and native helper executables may prove the executor, but they do not
enter checked-in production discovery. HCM-5.4 alone installs the already
selected first production validator manifest and bundle.

V1 hard ceilings are part of the protocol, not caller-selected hints:

| Surface | Exact hard ceiling |
| --- | --- |
| Bundle inventory | 256 regular files; safe-relative UTF-8 path at most 4,096 bytes; each file at most 64 MiB; aggregate at most 256 MiB |
| Fixed launch arguments | 16 arguments; each at most 4,096 UTF-8 bytes; no empty executable or argument lookup |
| Inline request inputs | 128 role-distinct JSON documents; each at most 1 MiB; aggregate source bytes at most 8 MiB |
| Process request / stdout result | 8 MiB canonical JSON before the required LF for each document |
| Stderr | 65,536 bytes total; truncation is recorded and any excess is a failed output bound, never a candidate side channel |
| Candidate observations | 128 entries; each canonical candidate payload at most 512 KiB; all entries remain inside the 8 MiB result ceiling |
| Diagnostics | 128 entries; each normalized public diagnostic at most 4,096 UTF-8 bytes and inside the result ceiling |
| Time | positive wall timeout no greater than 300,000 ms; cancellation grace no greater than 5,000 ms |

Manifest and request limits may be lower. The effective value is the checked
minimum of the two and these hard ceilings; zero, overflow, or a larger value
refuses before launch. HCM-5.4 may select lower validator limits but may not
raise this protocol version's ceilings.

### One-shot process JSON

The exact protocol is:

1. The SDK asks `handbook-contracts` to prepare one current exact evaluation run
   before dock selection or launch. The dock re-admits the complete run binding;
   bare lock, stale/deprecated/closed head, changed policy/currentness, or a
   caller-assembled substitute refuses.
2. The host resolves one exact manifest and local first-party bundle mapping,
   re-verifies the complete bundle inventory, executable digest, target, and
   fixed arguments, creates an empty private working directory, and computes
   the effective limits as the minimum of manifest, host policy, and request.
3. The host launches the exact executable directly with piped standard streams,
   writes exactly one bounded UTF-8 RFC-8785-canonical JSON
   `DockProcessRequestV1` followed by one LF, then closes stdin.
4. The child may emit bounded stderr diagnostics only. Stdout must be exactly
   one bounded UTF-8 RFC-8785-canonical JSON `DockProcessResultV1` followed by
   one LF. Any missing or extra byte, duplicate member, second document, prose,
   prompt, invalid UTF-8/JSON, noncanonical bytes, unknown field, schema or
   fingerprint mismatch, or wrong request/run/manifest binding is a host
   `protocol_error`.
5. The host owns monotonic deadline and cancellation. It attempts termination,
   waits no longer than the exact grace, then performs the portable child kill
   available to this first-party path. A deadline remains `timed_out`, and an
   accepted cancellation remains `cancelled`, regardless of later output.
6. After launch admission, the host attempts to commit exactly one operational
   execution record for every completed, refused, timed-out, cancelled, failed,
   or protocol-error outcome, including spawn failure. Record durability
   uncertainty produces no false receipt or candidate. Only a zero-exit
   `completed` result with confirmed cleanup and durable record commit may then
   materialize completed candidate observations.

`DockProcessRequestV1` binds its schema/protocol refs and fingerprints, request
ref/fingerprint, exact manifest and implementation bundle, evaluation run and
lifecycle basis, contract/claim/subject/case partition, requested evidence
kinds and Resolution, ordered inline JSON input roles with exact media/schema/
content fingerprints, effective limits, `network: denied`, and the exact
owner-established semantic-time/currentness observations. It contains no path, credential, raw
idempotency key, verdict, gate, or mutation authority.

`DockProcessResultV1` has only `completed` or `refused` status. A completed
result binds the exact request/run/manifest/implementation, a complete
observed/unobserved/typed-excluded claim partition, bounded candidate
observations, bounded normalized diagnostics, actual observation Resolution,
and its result fingerprint. A refused result has one closed reason plus bounded
diagnostics and no candidate observation. The child cannot report host timeout,
cancellation, failure, protocol error, cleanup, canonical evidence, verdict,
gate, applicability, local closeout, or promotion truth.

The host outcome vocabulary is exactly `completed`, `refused`, `timed_out`,
`cancelled`, `failed`, or `protocol_error`. The first matching host fact wins:
cleanup/durability uncertainty; deadline; accepted cancellation; spawn/crash/
nonzero exit; framing/binding/output failure; valid refusal; valid completion;
otherwise protocol error. Every outcome except valid completion has an empty
candidate disposition.

### Operational records and candidate handoff

`DockExecutionRecordV1` is immutable content-addressed operational truth. It
binds exact request/result/manifest/bundle identities, host outcome, bounded
exit/timing/resource/diagnostic facts, cleanup disposition, and the exact
completed-result candidate-observation bundle fingerprint and count. It does
not embed final candidate refs, preventing a fingerprint cycle.

After committing a completed execution record, the dock materializes each
untrusted `EvidenceCandidateV1` by binding the retained request, result,
execution record, manifest producer, evaluation run, claim partition,
subject/case, sources, observation, and requested/actual Resolution. The exact
candidate records live in the dock operational store and are resolved through
an adapter to the existing Contracts candidate-source boundary. Contracts
recomputes every identity, rechecks current source authority and lifecycle, and
is the only owner that may append canonical evidence through
`contract.evidence.append@1.0.0`. A completed run with zero candidates retains
one execution record and creates no evidence.

The store is repository-bound, append-only for execution and candidate records,
idempotent for `dock.run`, bounded, atomically committed, snapshot-consistent,
and fail-closed on missing, extra, partial, duplicated, tampered, or ambiguous
inventory. Exact request/key replay returns the original record; conflicting
key/request reuse writes nothing. Cleanup uncertainty retains only the bounded
quarantine facts needed for recovery and exposes no candidate.

## Ordinary operations and schema closure

HCM-5.3 closes exactly the three already-reserved version-`1.0.0` rows without
adding or renaming an inventory row:

| Operation ref | Class / idempotency | Exact effect |
| --- | --- | --- |
| `dock.manifest.list@1.0.0` | `read_only` / `safe` | Return one bounded deterministic page from the exact immutable production manifest registry; HCM-5.3 returns the valid empty registry. |
| `dock.manifest.read@1.0.0` | `read_only` / `safe` | Return one exact ref/fingerprint-bound manifest or typed absence; no write or host mapping disclosure. |
| `dock.run@1.0.0` | `append_only` / `idempotency_key_required` | Prepare/re-admit the exact evaluation run, resolve one exact production manifest, execute once, and append one operational record after launch admission; manifest absence or pre-launch admission refusal writes nothing, and only completed/clean/durably recorded output exposes untrusted candidates. |

The exact independently retrievable new schema roots are:

1. `handbook.dock-manifest@1.0.0`;
2. `handbook.dock-process-request@1.0.0`;
3. `handbook.dock-process-result@1.0.0`;
4. `handbook.dock-execution-record@1.0.0`;
5. `handbook.dock-operation-details@1.0.0`;
6. `handbook.operation.dock-manifest-list-request@1.0.0`;
7. `handbook.operation.dock-manifest-list-result@1.0.0`;
8. `handbook.operation.dock-manifest-read-request@1.0.0`;
9. `handbook.operation.dock-manifest-read-result@1.0.0`;
10. `handbook.operation.dock-run-request@1.0.0`; and
11. `handbook.operation.dock-run-result@1.0.0`.

Implementation bindings, limits, inline inputs, candidate observations,
partitions, diagnostics, and host outcomes remain `$defs` within those roots.
Existing contract evaluation-run, evidence-candidate, shared problem, receipt,
idempotency, invocation, artifact, and binding roots are reused. An additional
independently retrievable root requires authority remediation; it may not appear
as an incidental implementation choice.

`handbook.dock-operation-details@1.0.0` uses the closed causes `malformed`,
`over_limit`, `missing`, `unknown`, `stale`, `substituted`, `conflict`,
`unsupported_target`, `manifest_unavailable`, `launch_refused`, `timed_out`,
`cancelled`, `process_failed`, `protocol_error`, `cleanup_uncertain`,
`durability_uncertain`, and `internal_invariant`. Pre-launch refusal writes
nothing; a post-admission host outcome is returned through the exact execution
record when its durable commit is known. No problem cause fabricates a semantic
result.

After implementation, exact product truth must be **3 HCM-5.3 operations / 62
inventory rows / 31 capabilities / 168 schema roots**, with exact
`[cli_json, rust_sdk, tauri]` evidence on all three operations and an empty
production dock-manifest registry. Current pre-implementation truth remains
62 / 28 / 157.

SDK DTOs are bounded outward projections. The three operations must close
through independent dock-owner admission, exact definitions, request/result/
problem schemas, write/idempotency declarations, dispatch, checked-in canonical
schema bytes, deterministic manifest and production witness, and the unchanged
generic Rust SDK/CLI/Tauri consumers. No dock-specific CLI command, Tauri
command, parser branch, installed-skill workflow branch, or transport-owned
semantic branch is authorized.

## Scope and non-goals

Slice implementation may change only:

- a new `crates/dock/**` operational crate, test-only native protocol helpers,
  and focused tests;
- the narrowest directly necessary `crates/contracts/**` candidate-source
  construction/adapter seam and focused tests, without changing canonical
  evidence, verdict, gate, lifecycle, or local-closeout semantics;
- directly necessary SDK DTO, operation, dock-owner admission, dispatch,
  schema-generation, production-witness, consumer, export, and focused-test
  paths under `crates/sdk/**`;
- workspace and directly affected crate manifests;
- generic CLI/Tauri regression tests only when needed to prove unchanged
  capability-driven consumption; and
- this authority plus directly necessary current navigation, roadmap,
  architecture, baseline, and seam truth after executable state changes.

HCM-5.3 does not authorize:

- the `handbook.dock.json-schema@1.0.0` validator executable, its production
  manifest/bundle, validator-specific facts, fixtures, or real contract proof;
- any Python interpreter, Python package, Python harness, package installer, or
  Python runtime dependency in the first-party path;
- arbitrary third-party bundle admission, a plugin marketplace, hostile-code
  confinement, stronger host sandboxing, remote refs, network use, executable
  hooks, dynamic loading, or ambient schema/runtime discovery;
- canonical evidence construction, claim/applicability evaluation, verdict,
  gate, local closeout, promotion, posture, or contract lifecycle mutation in
  the dock;
- persisted verdict/gate/local-closeout state, Flow/pipeline integration,
  command-specific adapters, installed-skill workflow changes, publication,
  Phase 6, or GUI work; or
- HCM-5.4 authority preparation, Packet 1 dispatch, product implementation, or
  slice closeout by this authority-preparation task.

## Cross-packet invariants

Every packet must preserve:

1. Contracts owns canonical evidence/verdict/gate/lifecycle/local-closeout
   truth; Dock owns operational records and untrusted completed candidates only.
2. The process protocol is language-neutral and exactly versioned; Rust is the
   supported first-party host and first validator implementation, not semantic
   authority.
3. Every manifest, bundle member, launch vector, request, result, execution
   record, and candidate is exact-ref/fingerprint bound; substitution refuses.
4. Production manifest inventory remains empty until HCM-5.4 installs the
   selected validator; test helpers never enter production discovery.
5. Contracts prepares and every later boundary re-admits the current exact
   active-after-independent-lock evaluation run before launch or evidence.
6. Launch is direct and exact with cleared ambient inputs; protocol requests
   contain bounded inline JSON and no host/repository path or credential.
7. Host outcome, timeout, cancellation, result validation, cleanup, and durable
   commit are closed and deterministic; no non-completed or uncertain path
   exposes a candidate.
8. Execution-record identity closes over the candidate-observation bundle, and
   final candidate identity closes over the execution record, without a cycle.
9. The dock/source/SDK/transport cannot construct canonical evidence or emit a
   verdict, applicability, gate, local-closeout, promotion, or lifecycle result.
10. Claims distinguish enforced portable controls from protocol obligations and
    audited first-party restrictions; no unimplemented isolation guarantee is
    asserted.
11. The three reserved rows and 11 new roots close through the existing exact
    membrane and unchanged generic consumers with final 62 / 31 / 168 truth.
12. HCM-5.4 validator behavior and stronger future third-party confinement stay
    outside this slice.

## Ordered minimal packet decomposition

| Packet | Bounded outcome | Hard dependency | Verification and allowed boundary |
| --- | --- | --- | --- |
| **1 — dock owner and protocol kernel** | Add `handbook-dock` at the selected operational seam. Implement exact manifest/bundle/launch identities, immutable empty production registry, process request/result/execution-record types, closed outcomes, bounds, canonical fingerprints, and list/read owner use cases. Add no process launch, candidate persistence, SDK operation, production manifest, or validator. | HCM-5.1/HCM-5.2 closeout and this authority. | Focused dock tests prove exact identities, canonical bytes, unknown-field/version/substitution refusal, safe-relative bundle closure, empty registry paging/read absence, closed protocol/outcome vocabularies, bounds, dependency direction, and no semantic-owner constructors. |
| **2 — operational store and Contracts candidate-source seam** | Add repository-bound append-only request/result/execution/candidate observation storage, exact replay/conflict/concurrency/recovery behavior, completed-candidate materialization without identity cycles, and the narrow adapter/constructor seam required to satisfy the existing Contracts candidate-source port. Add no process launch or canonical evidence writer. | Packet 1. | Focused Dock/Contracts tests prove retained-root equality, atomic inventory/recovery, stable roots, exact replay, conflicting-key refusal, tamper/partial/surplus refusal, execution-to-candidate binding, current source-authority re-admission, raw-key/path secrecy, and that only Contracts can create canonical evidence. |
| **3 — portable first-party process executor** | Implement exact bundle re-verification, direct native launch, cleared environment, private cwd, one-request/one-result framing, bounded concurrent stream capture, monotonic timeout, host cancellation, portable child termination, output admission, cleanup, outcome classification, and completed-only store commit. Use test-only native helper processes; production manifest inventory stays empty. | Packet 2. | Focused executor tests exhaust valid completion/refusal, absent/substituted bundle, direct-argv source gates, invalid/duplicate/noncanonical/extra/oversized output, stderr bounds/redaction, nonzero exit, timeout, cancellation/replay races, cleanup/durability uncertainty, zero-candidate completion, and no partial candidate. Platform tests may prove private mechanics, but public claims remain only the portable guarantees above. |
| **4 — three-operation SDK/schema/witness closure** | Map Dock use cases outward; change exactly the three reserved rows from deferred Phase 5 to the dock owner; add independent owner admission, DTOs, dispatch, five domain/problem plus six operation roots, deterministic 168-root schema manifest, 31-capability witness, and exact current transports. `dock.run` composes Contracts run preparation before executor admission. Add no adapter-specific route or production dock manifest. | Packet 3. | Dock/Contracts/SDK tests prove definition/class/idempotency/owner/source evidence, structural-before-owner refusal, run-preparation order, schema regeneration/meta-validation/external-retrieval refusal, request/result re-admission, receipts/replay, empty manifest catalog, final counts, dependency direction, and unchanged predecessor rows. |
| **5 — generic consumer and integrated slice proof** | Prove manifest list/read and run through the Rust SDK and existing generic CLI/Tauri consumers, using only test fixtures for process completion; prove the full failure/no-partial/source-admission boundary and exact final counts; update current product/navigation truth only. This is integration, not HCM-5.4 or slice closeout. | Packet 4. | Focused Dock/Contracts/SDK/CLI/Tauri suites, schema regeneration, formatting, workspace check, strict Clippy/rustdoc, dependency/source/no-Python/generic-consumer/Markdown-link gates, final integrated proof, and required GitNexus change detection. |

The order is strictly sequential:

```text
HCM-5.2 closeout -> P1 -> P2 -> P3 -> P4 -> P5
```

Each packet returns control to the meta task. No packet dispatches the next,
claims HCM-5.3 completion, prepares or implements HCM-5.4, or performs aggregate
closeout.

## Delegated just-in-time preparation

Separate packet files are optional. Each fresh packet task must reconcile this
authority with current source/tests and its completed predecessor before product
edits.

- Packet 1 may select private module/helper names and internal `$defs`, but not
  owner placement, public identities, protocol framing, outcome vocabulary, or
  the empty production registry.
- Packet 2 may select private storage filenames, lock factoring, and adapter
  helper names, but not retained-root equality, append-only/replay/recovery
  semantics, identity ordering, or Contracts-only canonical admission.
- Packet 3 may select private asynchronous/process-wait mechanics and test-helper
  factoring, but not direct launch, exact bundle verification, cleared ambient
  inputs, framing/bounds/outcomes, completed-only candidates, or the stated
  limits on isolation claims.
- Packet 4 may select internal DTO/schema `$defs` factoring, but not the three
  operation identities/classes, 11 root refs, final counts, run-preparation
  order, transports, or generic-consumer rule.
- Packet 5 uses final current product and completed packet evidence only; it may
  choose test/fixture factoring, not widen implementation or historical inputs.

If permitted sources leave more than one consequential answer about identity,
launch, source trust, durable ordering, portable enforcement, operation closure,
or transport admission, the packet returns `STOPPED` before the affected edit.
Private names and fixture mechanics are recoverable preparation, not a stop.

## Slice acceptance

HCM-5.3 completes only when all 16 conditions hold:

1. `handbook-dock` is the purpose-named operational owner and all dependency
   edges preserve Contracts/Engine semantic ownership and transport
   independence.
2. `handbook.dock.process-json@1.0.0` has exact versioned Draft 2020-12 request
   and result contracts, one canonical UTF-8 JSON object plus LF per stream, and
   no stdout side channel.
3. Manifest, bundle inventory, executable, target, fixed arguments, protocol,
   schemas, ceilings, and fingerprints are exact and substitution-resistant.
4. Production manifest inventory is empty and deterministic; list/read are
   bounded, exact, and reveal no local executable mapping.
5. `dock.run` obtains and re-admits one current exact Contracts evaluation run
   before any body access or launch; stale, bare-lock, absent-manifest, or
   substituted input spawns nothing and writes nothing.
6. The host re-verifies exact executable bytes and launches directly with
   cleared ambient environment, explicit private cwd, fixed arguments, inline
   bounded input, no shell/search/package/runtime discovery, and no exposed
   repository path or credential.
7. Checked request/result/stream/candidate/count/time/grace bounds, host-owned
   monotonic timeout, idempotent cancellation, and portable child termination
   are implemented and deterministically tested.
8. The six host outcomes and two process-result statuses follow the fixed
   precedence; invalid UTF-8/JSON/canonical form/schema/fingerprint/binding,
   extra output, crash, nonzero exit, late result, or cleanup uncertainty cannot
   be reclassified as completion.
9. Only zero-exit, exact completed, fully admitted, durably recorded, clean runs
   expose candidates; every other outcome and every uncertain boundary exposes
   none, with no partial evidence.
10. Operational records and candidates are append-only, exact, idempotent,
    recoverable, snapshot-consistent, noncyclically bound, and resolvable through
    the current Contracts candidate-source admission without granting the dock
    canonical authority.
11. Replay, conflicting key/request, concurrency, tamper, partial inventory,
    durability uncertainty, and cleanup failure are fail closed and leak no raw
    key, credential, host path, proxy, environment, or unbounded diagnostic.
12. Exactly the three named operations have complete dock-owner admission,
    DTOs, definitions, schemas, dispatch, witness, and
    `[cli_json, rust_sdk, tauri]` evidence with no command-specific consumer.
13. Exactly 11 new roots close the final 62-row / 31-capability / 168-root tree;
    all 157 predecessor roots, 28 predecessor capabilities, and six fixed use
    cases remain regression-protected.
14. Rust SDK, generic CLI, generic Tauri, and installed-skill source gates prove
    operation-independent consumption and preserve owner terminal, operational,
    receipt, replay, and no-partial truth.
15. The production JSON Schema validator, its manifest/bundle, validator fact
    schema, real contract/evidence/gate proof, Python/runtime dependencies,
    third-party confinement, publication, and Phase 6 remain absent.
16. Focused tests, deterministic schema regeneration, formatting, workspace
    check, strict Clippy/rustdoc, dependency/source/security/Markdown-link gates,
    required GitNexus change detection, and one aggregate P1-P4 causal review
    pass with no unresolved P1 or P2, or limitations are reported honestly.

## Aggregate verification and closeout rule

Each packet performs only its focused verification and returns control. Packet
completion never closes HCM-5.3.

After Packet 5, the meta task must recheck all 16 acceptance conditions on the
integrated product, reuse still-valid packet evidence, rerun invalidated or
cross-layer checks, and conduct one focused causal review under the current
P1-P4 policy. It remediates and re-reviews blocking P1/P2 findings within
authority or records a genuine workflow stop; P3/P4 remain advisory.

Only then may the meta task record one HCM-5.3 slice closeout and separately
admit HCM-5.4 authority preparation. HCM-5.3 closeout does not install a
validator, create canonical evidence, prove a real gate, or exit Phase 5.

## Slice closeout

HCM-5.3 closed on 2026-08-20 after all 16 acceptance conditions were rechecked
against the integrated Packet-5 product. The aggregate verification reran the
cross-packet owner, replay/currentness, process/store, schema/count, generic
consumer, dependency/source/security, portability, documentation-link, and
workspace gates. Stable outward replay identity and receipt projection preserve
the original execution and evaluation-run receipt while Contracts prepares the
current run before body, manifest, or spawn access. Only completed, clean,
durably recorded runs expose candidates; Contracts remains the sole semantic
authority; and the production manifest registry remains exactly empty.

The single focused aggregate P1–P4 review found one P2 currentness defect:
predecessor integration guards still asserted pre-HCM-5.3 capability and schema
totals. The test-only repair updated those exact aggregate guards to current
`62 / 31 / 168` truth and included the three Dock operations in generic
transport expectations. Focused reruns and the required delta review found no
remaining P1 or P2. No production behavior, operation, schema, dependency,
manifest, or capability was added by closeout.

This closeout does not prepare or implement HCM-5.4 and is not Phase-5 exit.

## Exact HCM-5.4 handoff

HCM-5.4 receives, without reopening selection:

- validator identity `handbook.dock.json-schema@1.0.0`;
- the exact process protocol, manifest/bundle binding, request/result/execution
  records, candidate-source adapter, host-enforced bounds, six host outcomes,
  completed-only rule, three ordinary operations, and empty
  production manifest registry established by HCM-5.3;
- a required native Rust first-party validator executable using the repository's
  existing pinned `jsonschema` crate and current exact Draft 2020-12 in-memory
  local-closure practices; and
- one narrow responsibility: validate one JSON-compatible instance against one
  exact local Draft 2020-12 schema/ref closure and emit bounded normalized
  schema-conformance candidate observations.

HCM-5.4 must add the first production manifest and native bundle, validator-
specific fact schema, positive/negative/refusal/timeout/fingerprint/Resolution
fixtures, and one exact active-contract path through completed execution,
Contracts admission, immutable evidence, verdict, gate, and local closeout. It
must refuse remote refs, executable hooks, ambient schemas, unsupported
dialects, changed closure or instance fingerprints, and any path that would let
the validator or host emit semantic truth. It must not add Python or depend on
ambient interpreters/packages, expand the process protocol by implementation
convenience, or treat first-party admission as general third-party confinement.

Preparation or execution of HCM-5.4 requires its own current slice authority.
This document selects the handoff only.

## Use and drift

Use this document as the current HCM-5.3 outcome and packet authority. Source
and tests remain product truth. Packets 1–5 establish exact current counts of
62 / 31 / 168 while the production dock registry remains exactly empty. The
aggregate HCM-5.3 acceptance audit and single slice closeout are complete;
HCM-5.4 is unprepared and unimplemented. If later authorized implementation
changes an admitted surface, update the current document that owns that truth
in the same bounded change.

Do not use HCM-5.3 closeout to infer validator implementation, HCM-5.4
authority, or Phase-5 completion.
