# HCM-4.4 Tauri-ready Command Facade

This document owns the outcome authority for HCM-4.4. It defines exactly one
slice outcome: a bounded, non-UI Tauri command facade over current SDK-owned
contracts and execution seams. It does not implement a packet, authorize
HCM-4.5, create a Tauri application, or make Tauri command names a source of
semantic identity.

## Current implementation state

**Packets 1–6 implementation and packet proof are complete. Aggregate
verification, causal review, and slice closeout remain pending.**

The hard predecessor, HCM-4.3, is complete at `74cec53`. Its packet
implementation completed through `f901231`; test-only remediations `920d217`
and `45cad19` aligned stale Engine assertions before the aggregate closeout.
The current SDK Packet-8 inventory/schema proof passes 1/1 and the current CLI
Packet-8 aggregate suite passes 7/7. The exact 62-row operation inventory, 19
discoverable capabilities, 18 `cli_json` transports, one `rust_sdk` posture
transport, six fixed SDK use cases, and 115-root schema manifest were the
implementation baseline for this slice. Packet 1 added the five facade roots
and extended the deterministic manifest to 120 roots. Packets 2–5 implemented
the command crate and exact read-only, use-case, and mutation routes; Packet 6
supplies the integrated packet proof and current protocol documentation.

This checkpoint does not claim `SLICE_COMPLETE`. The meta task still owns the
single aggregate verification, causal review, and slice closeout required after
Packet 6.

## Outcome

Provide one workspace library crate, package `handbook-tauri`, containing three
versioned Tauri 2 command adapters over a host-managed trusted repository
context. Those commands consume SDK-owned Serde request types and return the
exact SDK-owned bootstrap, catalog, or invocation response types. The facade
supports the current production bootstrap/catalog boundary, all 19 currently
discoverable ordinary operations, and exactly the six HCM-4.3 SDK use cases.
It uses no CLI subprocess in normal operation, imports no CLI product logic,
and gives the frontend no semantic, authority, policy, mutation, receipt, or
terminal-classification ownership.

The selected direction is:

```text
untrusted IPC payload -> SDK-owned typed request admission
                      -> host-owned repository context
                      -> SDK facade -> Engine / Flow / pipeline owner
                      -> exact SDK response DTO
                      -> Tauri serializer and IPC frame
```

## Current and adopted source boundary

Current source and tests are product truth. This authority was selected from:

- the current program [`README`](../../README.md),
  [`phase-and-slice map`](../../phase-and-slice-map.md),
  [`architecture context`](../../architecture-context.md),
  [`seam crosswalk`](../../seam-crosswalk.md), and
  [`implemented baseline`](../../implemented-baseline.md);
- the completed [`HCM-4.3 authority`](../HCM-4.3/README.md) and its compact
  packet-decision precedents;
- `crates/sdk/src/contract_membrane/{mod.rs,invocation.rs,operation.rs,production_witness.rs,serialization.rs,response.rs}`
  and `crates/sdk/src/lib.rs`;
- the focused SDK contract, admission, bootstrap/catalog, integrated-proof,
  and HCM-4.3 Packet-8 tests;
- `crates/cli/src/{coverage.rs,main.rs,request_shared.rs}` and the CLI
  HCM-4.3 Packet-8 proof; and
- the workspace and crate manifests.

The tracked `handbook-tauri` library now provides the exact three-command
adapter over the SDK facade. The operation model and production witness report
`Tauri` only for the 19 proven routes: the 18 predecessor operations retain
`cli_json`, posture retains `rust_sdk`, and the other 43 inventory rows remain
transport-empty and omitted.

Only these historical sections at
`archive/handbook-contract-membrane-pre-clean-20260812` are adopted as bounded
design lineage:

- the HCM-4.4 subsection of `04-phase-slice-map.md`;
- Tier-3 Tauri parity and “Tauri and API posture” in
  `01-target-architecture.md`; and
- “Tauri adapter contract” in `05-contracts-schemas-and-gates.md`.

This authority reselects from that lineage direct SDK/Serde consumption,
transport-owned names/framing/scheduling, LF-free canonical validation before
IPC framing, structured expected outcomes, no normal-operation CLI subprocess,
and explicit pre-commit versus post-commit adapter-failure behavior. It does
not adopt the historical rule that every command ID maps to one operation:
the current product has six non-operation use cases, and current authority
forbids command-name-derived operation identity. Instead, three transport
commands accept SDK request discriminators whose exact refs select behavior.
Historical status, gates, selectors, ledgers, proof walls, handoffs, adjacent
bridge work, Phase-5 material, and closeout procedure remain inactive.

The official Tauri 2
[`command` contract](https://v2.tauri.app/develop/calling-rust/) is a framework
constraint, not product authority: command arguments must implement Serde
`Deserialize`, return values and command errors must implement Serde
`Serialize`, expected `Err` values reject the frontend promise, and
synchronous work must be scheduled off the UI thread. Packet 2 must verify
those rules against the selected exact Tauri 2 dependency before product use;
later framework drift cannot override the facade behavior fixed here.

## Facade and dependency boundary

### Crate placement

- Add workspace member `crates/tauri/`, package `handbook-tauri`, as a library
  only. It contains no executable, `tauri.conf.json`, web assets, window
  construction, frontend code, or UI state.
- `handbook-tauri` depends on `handbook-sdk`, Serde support needed by its
  bounded host error, and Tauri 2 with
  [default features disabled](https://docs.rs/tauri/latest/tauri/#cargo-features)
  so the adapter does not select the default `wry` UI runtime. Packet 2 pins
  the exact compatible Tauri 2 release in `Cargo.lock` after checking current
  official command documentation and compiling the command registration
  surface.
- `handbook-tauri` has no production dependency on `handbook-cli`,
  `handbook-engine`, `handbook-flow`, or `handbook-pipeline`. The SDK and owner
  crates gain no Tauri dependency. Test-only parity harnesses may launch the
  already-built CLI, but they may not import CLI semantics into product code.
- SDK-owned transport-neutral request and response types remain under
  `handbook_sdk::contract_membrane`. The adapter crate owns only host state,
  command functions, Tauri scheduling, framework serialization handoff, and
  bounded host failures.

This is a real seam because the CLI and Tauri adapter are distinct consumers
of the same SDK interface. The SDK facade is the deep module: callers learn
one bootstrap path, one catalog path, and one invocation path rather than the
owner-specific execution topology.

### Exact command surface

The only public Tauri command IDs in this slice are:

| Command ID | SDK input | Resolved SDK result |
| --- | --- | --- |
| `handbook_bootstrap_v1` | existing `BootstrapRequestV1` | a new closed bootstrap-facade outcome over the existing `BootstrapResponseV1` or `BootstrapRefusalV1` |
| `handbook_catalog_read_v1` | new closed `CatalogReadRequestV1` | new closed `CatalogReadResponseV1` over the existing capability, schema, or profile page; cursor mismatch; or typed blocked result |
| `handbook_invoke_v1` | new `InvocationRequestV1` | existing `InvocationResponseV1` |

These IDs are versioned transport routing labels only. `handbook_invoke_v1`
does not infer an operation or use case from its own name. The SDK request
binding and discriminator select exactly one operation or use-case ref, and
unknown, mismatched, duplicate, or non-canonical forms fail closed.

Expected bootstrap refusal, catalog blocking/cursor mismatch, and invocation
`blocked`, `refused`, or `error` outcomes resolve as the typed success value of
the Tauri command. A serialized adapter-host error may reject the Tauri promise
only when no truthful SDK response exists, such as failed command argument
deserialization before request selection, unavailable host-managed state, or
framework-level serialization/delivery failure. Domain non-success and owner
refusal never become rejected prose promises.

### SDK request contracts

Packet 1 adds these exact public roots to the existing deterministic SDK
generator and manifest:

- `handbook.invocation-request@1.0.0`;
- `handbook.use-case-request@1.0.0`;
- `handbook.bootstrap-facade-response@1.0.0`;
- `handbook.catalog-read-request@1.0.0`; and
- `handbook.catalog-read-response@1.0.0`.

`InvocationRequestV1` has an exact schema binding, a closed `request_kind`
discriminator with only `operation` and `use_case`, exactly one matching body,
and a fingerprint over its binding, discriminator, and body. Its operation
body is the existing `OperationRequestV1`. Its use-case body is a closed
`UseCaseRequestV1` union over exactly the six HCM-4.3 refs and the concrete
typed semantic inputs already required by their SDK paths:

1. `handbook.use-case.repository-setup@1.0.0`;
2. `handbook.use-case.repository-doctor@1.0.0`;
3. `handbook.use-case.author-charter@1.0.0`;
4. `handbook.use-case.author-project-context@1.0.0`;
5. `handbook.use-case.approver-administration@1.0.0`; and
6. `handbook.use-case.pipeline-route-resolve-and-persist@1.0.0`.

Repository roots, host paths, stdin/file acquisition choices, CLI flags,
human-rendering choices, exit codes, and raw framework request objects are not
semantic request fields. Where an existing use-case result carries bounded
transport-presentation facts, the adapter supplies one fixed typed Tauri route
value; the frontend cannot choose it. The six use cases remain absent from the
62-row operation inventory, owner admission, and capability discovery.

The bootstrap and catalog responses preserve their distinct HCM-4.2 types and
do not masquerade as invocation responses. New wrapper roots only close their
existing typed outcome alternatives for Serde/Tauri use; they do not redefine
bootstrap negotiation, catalog retention, cursor, owner, schema, profile, or
capability truth.

## Exact supported target

The invocation facade closes exactly these current operations:

- the 18 HCM-4.3 `cli_json` operations fixed in the predecessor: ten artifact
  and intake operations, `flow.resolve`, and seven pipeline operations; and
- the existing `posture.transition.apply@1.0.0` Rust-SDK operation.

No new operation row, use case, definition, owner claim, schema meaning,
discovery entry, or semantic route is created. At slice completion:

- each of the 18 predecessor operations has exactly
  `[OperationTransport::CliJson, OperationTransport::Tauri]`;
- posture has exactly
  `[OperationTransport::RustSdk, OperationTransport::Tauri]`;
- all 19 remain discoverable with the same definitions, owners, schemas,
  mutability, idempotency, write sets, and outcomes; and
- every other inventory row remains transport-empty and omitted.

The read-only packet covers the current 12 read-only rows among that set. The
mutation packet covers the remaining seven ordinary rows: the three artifact
mutations, pipeline capture apply, pipeline handoff emit, pipeline state apply,
and posture transition apply. A use-case may itself realize writes, but it
never becomes an ordinary operation or transport capability.

## Trusted context and delivery boundary

The Tauri host, not the frontend, constructs one managed facade state with the
selected repository root, production witness, retention/clock source, fixed
contract limits, and SDK handle. No IPC request accepts a repository root or
other host path. The adapter normalizes the host-selected root once and never
echoes it in responses or host errors.

IPC payloads remain untrusted. Serde shape admission is followed by the same
SDK canonical, fingerprint, schema, definition, and owner admission already
required by direct SDK and CLI paths. The frontend cannot supply owner tokens,
current transport evidence, trusted time, private authority records, policy
outcomes, realized receipts, or terminal state. Mutation authority and replay
remain with Engine, Flow, or pipeline owners.

The adapter prepares and re-admits LF-free canonical response bytes before
handing the exact DTO to Tauri serialization. Those bytes validate instance
identity; they are not a CLI stdout document and do not acquire the CLI's
terminal LF. Tauri owns the final IPC serializer/frame. Adapter correlation or
framework metadata stays outside SDK fingerprints and semantic equivalence.

After exact request selection but before an owner call, an injectable adapter
failure produces the existing typed transport-failure `error` invocation and
proves that no owner write occurred. Once an owner has committed a mutation,
serialization or IPC delivery uncertainty must not produce an ordinary
Handbook response, retry automatically, or report false failure/success. It
surfaces only as a bounded host-level adapter rejection. A caller retry with
the same established idempotency key must recover the same owner terminal,
semantic fingerprint, idempotency state, and receipts without another write.
Operations without an idempotency key retain their existing compare-and-write
or refusal rules; the adapter invents no retry contract.

Commands execute through Tauri-supported off-main-thread scheduling with owned
request values. Scheduling, cancellation wiring, and window lifecycle remain
adapter concerns and cannot alter owner execution or terminal classification.

## Scope and allowed implementation boundary

Slice implementation may change only:

- `Cargo.toml`, `Cargo.lock`, and the directly involved crate manifests for
  workspace membership and the bounded adapter dependencies;
- a new `crates/tauri/**` library and its focused tests;
- directly necessary SDK facade, contract-membrane, schema-generation,
  checked-in schema, production-witness, and focused-test paths under
  `crates/sdk/**`;
- test-only CLI parity harnesses under `crates/cli/tests/**`, without changing
  CLI product behavior; and
- directly necessary current HCM status, routing, and user-facing protocol
  prose after executable truth changes.

Existing Engine, Flow, pipeline, and CLI product source is read-only for this
slice. If one of the exact target routes cannot be closed without changing an
owner's semantics, adding a new operation/use case, or altering CLI behavior,
the responsible packet returns `STOPPED` with the exact missing seam rather
than widening scope.

## Cross-packet invariants

Every packet must preserve these invariants:

1. Engine, Flow, and pipeline remain the only semantic and mutation owners;
   SDK composition and Tauri framing cannot self-admit owner truth.
2. SDK request refs and discriminators, never Tauri command names or frontend
   labels, select exact operation or use-case identity.
3. The operation inventory remains exactly 62 rows; the facade target remains
   exactly 19 current capabilities and six fixed non-discoverable use cases.
4. The Tauri crate depends toward the SDK only. SDK and owner crates contain no
   Tauri dependency or framework types, and normal operation contains no CLI
   subprocess, CLI-output parsing, or CLI product dependency.
5. Bootstrap/catalog values keep their existing contract family; invocation
   requests and responses keep closed matching discriminators, bindings,
   canonical fingerprints, and exact schemas.
6. The host supplies repository identity, trusted time, retention, limits, and
   SDK state. Untrusted IPC cannot select a host path or manufacture current
   authority, owner admission, transport evidence, receipts, or terminal state.
7. Expected domain non-success resolves as a typed SDK value. Tauri promise
   rejection is reserved for bounded host/framework failure when no truthful
   SDK response exists.
8. Direct SDK, CLI JSON where present, and Tauri paths call one owner execution
   and agree on semantic identity, terminal, data/problems, idempotency state,
   diagnostics, next actions, fingerprints, and receipts. Declared adapter
   correlation and fixed acquisition-presentation facts are the only permitted
   transport differences.
9. Use cases remain outside ordinary admission/discovery and cannot gain an
   operation identity merely because the Tauri facade exposes them.
10. Instance canonical bytes, checked-in schema bytes, CLI LF framing, and
    Tauri IPC framing remain separate identity domains.
11. Pre-commit adapter failure realizes no write; post-commit
    serialization/delivery uncertainty yields no false ordinary response and
    no automatic retry; established-key replay yields the original result
    without duplicate mutation.
12. Raw idempotency keys, private owner records, host paths, secrets, debug
    values, and unbounded framework errors remain absent from SDK responses,
    Tauri rejections, logs, fixtures, and checked-in schemas.

## Ordered packet decomposition

| Packet | Bounded outcome | Dependency | Allowed paths or boundary |
| --- | --- | --- | --- |
| **1 — transport-neutral facade contracts** | Add the exact five SDK request/response roots, closed request/result discriminators, canonical fingerprints, strict Serde admission, concrete six-use-case request union, deterministic schema generation, and focused contract tests. Record the manifest root count established by the packet. No command, owner execution, or transport evidence closes yet. | HCM-4.3 closeout, satisfied. | `crates/sdk/src/contract_membrane/**`, `crates/sdk/schemas/**`, and focused `crates/sdk/tests/**` only. |
| **2 — trusted host, bootstrap/catalog, and command crate** | Add the non-UI `handbook-tauri` workspace library, exact three command IDs, managed trusted repository/witness/clock state, bounded serializable host error, off-main-thread command scheduling, and working bootstrap/catalog commands over SDK types. Pin and compile the exact Tauri 2 dependency with default UI/runtime features disabled. Prove no frontend repository root and no CLI/owner product dependency. Invocation remains default-denied until later packets. | Packet 1. | Root workspace/lockfile, directly involved manifests, new `crates/tauri/**`, and only directly necessary SDK bootstrap/catalog facade tests. |
| **3 — read-only ordinary-operation parity** | Route exactly the 12 current read-only target operations through one SDK-owned invocation dispatcher and `handbook_invoke_v1`; preserve exact owner admission and `InvocationResponseV1`; prove direct SDK/Tauri and CLI JSON equivalence for all 12 predecessor operations; then add `Tauri` evidence only to those proven rows. No mutation or use-case dispatch closes. | Packet 2. | Directly necessary SDK dispatch/operation/witness/schema tests, `crates/tauri/**`, and test-only CLI parity harnesses. Owner and CLI product source remain read-only. |
| **4 — six fixed use-case parity** | Route exactly the six HCM-4.3 use-case request variants through the same invocation command; preserve their existing owner/SDK execution, terminal details, realized-write receipts, and fixed Tauri presentation facts; prove direct SDK/CLI/Tauri semantic equivalence; and prove all six remain absent from operation inventory and capability discovery. | Packet 3. | Directly necessary SDK facade/use-case/contract tests, `crates/tauri/**`, and test-only CLI parity harnesses. No new operation, use case, or CLI behavior. |
| **5 — mutation parity and delivery safety** | Route exactly the seven current mutation target operations, including posture, without changing owner semantics. Prove idempotency/compare-and-write behavior, receipts, pre-commit adapter failure, post-commit serialization/delivery uncertainty, no automatic retry, and established-key replay without duplicate writes. Add `Tauri` transport evidence only after each route is closed, yielding the exact final transport vectors. | Packet 4. | Directly necessary SDK artifact/pipeline/posture dispatch, operation/witness tests, `crates/tauri/**`, schemas only if required by Packet-1 contracts, and test-only parity/failure-injection paths. Owner and CLI product source remain read-only. |
| **6 — integrated facade proof and protocol documentation** | Prove the exact three-command registry, five new contract roots, all 19 capability routes, all six omitted use cases, bootstrap/catalog closure, deterministic schema regeneration, direct/CLI/Tauri equivalence, host-path and secret absence, off-main-thread command compilation, dependency direction, no CLI subprocess, exact transport vectors, and failure/replay rules. Update only user-facing/current HCM truth that changed. This is an integration packet, not a second implementation authority or closeout layer. | Packets 1–5. | Focused SDK/Tauri/CLI integration tests and directly necessary current documentation. Any product correction must remain within an earlier packet's boundary and invalidated evidence must be rerun. |

The order is strictly sequential:

```text
HCM-4.3 -> P1 -> P2 -> P3 -> P4 -> P5 -> P6
```

Each packet returns control to the meta task. A packet may not dispatch the
next packet, prepare HCM-4.5, claim slice completion, or perform aggregate
closeout. The meta task alone admits and dispatches the next eligible packet.

## Delegated just-in-time preparation

The user's packet-by-packet execution request delegates bounded preparation
for Packets 1 through 6 inside the same fresh top-level task that implements
each packet. Separate packet files are optional. Before product edits, the
packet task must reconcile its outcome against this authority, current
source/tests, the HCM-4.3 closeout and packet checkpoints, current program and
architecture sources, the selected exact dependency documentation when
framework behavior matters, and only the bounded historical sections adopted
above.

Within its packet, the task may select private helper/module names, test
fixture factoring, the exact compatible Tauri 2 patch/minor release, and the
documented Tauri scheduling/registration form. It may not change the three
command IDs, five contract refs, exact 19-operation/six-use-case target,
trusted-context rule, transport-vector outcome, expected-error behavior,
delivery/replay contract, crate dependency direction, or allowed product
boundary. It may not add frontend code, an operation, a use case, an owner
claim, an alternative DTO shape, or a CLI fallback.

If permitted current sources leave more than one consequential answer about a
request field, mutation write set, idempotency key, owner route, transport
evidence, trusted host fact, or post-commit result, the packet returns
`STOPPED` before the affected product edit with that exact decision. Missing
packet documentation alone is recoverable preparation and is not a stop.

## Slice acceptance

HCM-4.4 completes only when all of the following hold:

1. `handbook-tauri` is a workspace library with no application/UI assets and
   only the exact three public command IDs; its production dependency graph
   reaches semantic owners only through `handbook-sdk` and contains no CLI.
2. The SDK owns and strictly admits the five exact new contract roots; every
   root is in the deterministic Draft 2020-12 tree and exact-byte manifest,
   and DTO/schema/fingerprint mismatch fails closed.
3. `handbook_invoke_v1` accepts only the exact 19 current operation refs or six
   fixed use-case refs. Command names, frontend labels, and CLI paths cannot
   select or synthesize semantic identity.
4. Bootstrap and all three catalog families preserve existing negotiation,
   pagination, cursor, retention, owner/schema/profile, and refusal/blocking
   truth in their distinct SDK response family.
5. All 19 current capabilities execute once through the existing SDK/owner
   route. The 18 HCM-4.3 operations finish with exactly `[cli_json, tauri]`,
   posture with exactly `[rust_sdk, tauri]`, and every other one of the 62 rows
   remains transport-empty and omitted.
6. The six use cases preserve their exact refs, schemas, terminals, typed
   details, diagnostics, next actions, fingerprints, idempotency state, and
   realized receipts while remaining absent from operation admission and
   discovery.
7. Equivalent direct SDK, CLI JSON where present, and Tauri fixtures are
   schema-valid and semantically equal after excluding only declared adapter
   correlation and fixed acquisition-presentation facts. No renderer,
   command, or frontend branch reruns owner logic or reclassifies the result.
8. Repository identity, trusted time, retention, limits, and SDK state are
   host-managed; IPC cannot supply them. Invalid or substituted request
   bindings fail closed, and host paths/private values are absent from all
   response and error surfaces.
9. Tauri serialization receives a re-admitted LF-free SDK DTO. CLI LF framing
   is absent from DTO/schema identity, and expected domain non-success resolves
   instead of rejecting as prose.
10. Failure injection proves pre-commit adapter failure performs no write and
    returns a typed transport error; post-commit serialization/delivery
    uncertainty returns no ordinary response, performs no automatic retry,
    and same-key retry replays the exact committed result and receipts without
    a duplicate write.
11. Focused SDK and adapter tests, existing HCM-4.3 SDK/CLI regressions, strict
    formatting/Clippy/rustdoc or equivalent applicable compile checks,
    dependency inspection, deterministic schema regeneration, and required
    GitNexus change detection all pass or are reported honestly.
12. One aggregate causal review of the complete slice has no unresolved P1 or
    P2, after all packet evidence remains valid on the final product state.

## Aggregate verification and closeout

Each packet runs focused verification for its own acceptance boundary and
returns its commit and evidence to the meta task. Packet review applies only
when a material packet decision or measured risk warrants it under the current
review policy. Packet completion never closes the slice.

After Packet 6, and only after every acceptance condition is proven, the meta
task performs one aggregate verification over the final SDK/adapter/CLI
integration and one focused causal review using the P1–P4 rubric. Valid packet
evidence may be reused; checks invalidated by later packets must be rerun.
Blocking P1/P2 findings are remediated and reviewed again within current
authority or produce a genuine workflow stop. P3/P4 advisories remain
non-blocking.

The meta task then records one aggregate slice closeout. No packet creates a
slice closeout, ledger, selector, proof wall, handoff, durable capsule, or
mechanical closeout commit.

## Non-goals

HCM-4.4 does not authorize:

- a Tauri application, UI, webview/window configuration, frontend code,
  generated TypeScript client, UI state, or product workflow design;
- HCM-4.5 or prompt/skill-owned product semantics;
- Phase 5 or Phase 6 work, including contract/evidence/gate/dock owners,
  publication, bridge, Substrate adoption, or Tauri product implementation;
- new domain semantics, operation rows, use cases, owner claims, discovery
  meanings, posture policy, grounding adoption, or deferred Phase-3 work;
- command-name-derived operation identity, frontend policy/authority, or
  frontend mutation outside an admitted SDK request;
- a normal-operation CLI subprocess, CLI-output parsing, CLI semantic imports,
  or changes to CLI human/JSON/legacy behavior;
- OpenAPI or another canonical contract family; or
- orchestration artifacts, packet-level slice closeout, or more than one
  aggregate closeout.

## Use and drift

Use this document as the controlling HCM-4.4 outcome and packet boundary until
aggregate slice closeout updates its status. Source and tests remain product
truth. If implementation changes current product classification, capability
transport evidence, dependency topology, or seam routing, update only the
current HCM document that owns that truth. Do not use this authority to execute
HCM-4.5 or any adjacent work.
