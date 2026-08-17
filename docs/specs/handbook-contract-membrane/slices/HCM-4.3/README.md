# HCM-4.3 Complete CLI JSON Parity

This document owns the outcome authority for HCM-4.3. It defines the complete
current CLI command boundary, the machine/human/diagnostic/exit contract, the
compatibility posture, and the ordered implementation packets. It does not
implement a packet, authorize an adjacent slice, or make CLI command names the
source of semantic operation identity.

## Current admission state

**Packets 1 through 8, the one aggregate causal review, and aggregate slice
closeout are complete. HCM-4.3 is `SLICE_COMPLETE`.**

The hard predecessor, HCM-4.2, is complete. Its packet commits are
`f54dcbd`, `4d70701`, `241b998`, `caf0ef4`, and `c1f5d33`, following the
Packet-0 foundation at `8605d65`. Aggregate verification on the current product
state passed the Packet-5 integrated proof, SDK package-content inspection, and
strict SDK rustdoc. One aggregate causal review returned `APPROVE/PROCEED` with
no P1 or P2. The prior P3 status-prose drift is corrected by the current
authority-preparation change; the separate P3 bootstrap defense-in-depth note
remains optional and does not block this slice.

HCM-4.3 packet implementation completed through `f901231`. Aggregate review
then found three stale Engine test contracts caused by the Packet-3 and
Packet-4 compatibility changes; test-only remediations `920d217` and
`45cad19` aligned those assertions without changing product behavior. The
formerly failing Engine binaries pass 21/21, the packet-focused Engine, SDK,
and CLI surfaces pass, and the fresh aggregate causal review returned
`APPROVE/PROCEED` with no P1–P4 findings. All ten slice acceptance conditions
are satisfied: 36 supported leaves/modes route through 18 admitted `cli_json`
ordinary operations or six fixed SDK use cases, the operation inventory
remains exactly 62 rows with its pre-existing Rust-SDK posture capability, the
schema manifest contains 115 roots, and pipeline route-resolve remains
correctly omitted from ordinary transport.

This closeout satisfies HCM-4.4's hard predecessor. It does not authorize or
start HCM-4.4 or any later slice; those remain subject to their own current
authority and explicit execution request.

## Outcome

Every supported nontrivial invocation on the current `handbook` CLI surface
must cross one shared typed execution boundary before either human or machine
rendering. For machine mode on a writable stdout transport, one invocation
emits exactly one versioned JSON response document. Human output is a
projection of the same typed result. Semantic diagnostics remain in that
result, operational progress and logging use stderr, and one documented exit
policy derives from terminal state.

The completed slice preserves this direction:

```text
semantic owner result -> SDK-owned invocation response
                      -> CLI human renderer | machine serializer | exit policy
```

The CLI continues to own parsing, repository discovery, input acquisition,
transport-local failures, rendering, stream selection, and process exit. It
does not own operation identity, request/result DTO meaning, owner admission,
semantic terminal status, mutation rules, or write receipts.

## Supported command boundary

The parity set is the current nontrivial leaf behavior below. Modes that select
one semantic action are included even when they share a Clap command path.

- setup: implicit auto setup, `setup init`, and `setup refresh`;
- author: Charter author, approve, promote, and validate; Project Context
  author and validate;
- approvers: bootstrap, add credential, revoke credential, and update mapping;
- artifact: list kinds, list instances, read, validate, intake-definition read,
  intake evaluate, intake append, candidate validate, candidate append, and
  promote;
- pipeline: list, show, resolve, compile payload, compile explain, capture
  preview, direct capture, capture apply, handoff emit, and state set;
- Flow packet composition: `generate` and `inspect`; and
- repository diagnosis: `doctor`.

Help, version output, parent commands that only print subcommand help, and Clap
syntax failures before a leaf invocation is recognized are shell control
behavior, not Handbook result documents. They keep Clap's established stdout,
stderr, and exit behavior. HCM-4.3 adds no new top-level product command and
does not turn examples, paths, or command spellings into semantic selectors.

Packet 1 must encode an executable coverage table that assigns every listed
leaf or mode exactly once to one of two result classes:

- an **ordinary operation response** when an exact HCM-4.2 inventory row
  matches the owner, mutability, request, result, and write semantics; or
- a **typed use-case response** only when the invocation appears in the closed
  use-case registry below.

Current admission is not a classification escape hatch. An exact matching row
classified as ordinary must gain its complete definition, request/result and
problem schemas, independent owner admission, receipts where applicable, and
current `cli_json` transport before its assigned packet completes. It may not
be downgraded to a use case merely because HCM-4.2 default-denied it.

The mandatory ordinary mappings are below; every operation ID denotes the exact
HCM-4.2 `@1.0.0` reference.

- the ten artifact leaves map respectively to `artifact.kind.list`,
  `artifact.instance.list`, `artifact.read`, `artifact.validate`,
  `intake.definition.read`, `intake.coverage.evaluate`,
  `intake.record.append`, `artifact.candidate.validate`,
  `artifact.candidate.append`, and `artifact.candidate.promote`;
- both `generate` and `inspect` map to `flow.resolve`, with distinct renderers
  over the same owner result; and
- pipeline list, show, both compile modes, capture preview, direct capture,
  capture apply, handoff emit, and state set map respectively to
  `pipeline.catalog.list`, `pipeline.catalog.read`, `pipeline.compile`,
  `pipeline.capture.plan`, `pipeline.capture.apply`,
  `pipeline.capture.apply`, `pipeline.handoff.emit`, and
  `pipeline.state.apply`. Both compile modes share one mapping.

The closed SDK use-case registry is limited to these current compositions:

| Exact use-case ref | Exact success-result schema | Covered invocations | Reason it is not an ordinary operation |
| --- | --- | --- | --- |
| `handbook.use-case.repository-setup@1.0.0` | `handbook.use-case.repository-setup-result@1.0.0` | implicit auto setup, `setup init`, `setup refresh` | HCM-4.2 setup rows remain `sdk_composition` with no independent owner. |
| `handbook.use-case.repository-doctor@1.0.0` | `handbook.use-case.repository-doctor-result@1.0.0` | `doctor` | The doctor row remains `sdk_composition` with no independent owner. |
| `handbook.use-case.author-charter@1.0.0` | `handbook.use-case.author-charter-result@1.0.0` | Charter author, approve, promote, validate | The current closed SDK route composes Charter behavior and is not one exact inventory operation. |
| `handbook.use-case.author-project-context@1.0.0` | `handbook.use-case.author-project-context-result@1.0.0` | Project Context author and validate | The current closed SDK route has no exact HCM-4.2 operation identity. |
| `handbook.use-case.approver-administration@1.0.0` | `handbook.use-case.approver-administration-result@1.0.0` | bootstrap, add credential, revoke credential, update mapping | No exact HCM-4.2 approver-administration operation exists. |
| `handbook.use-case.pipeline-route-resolve-and-persist@1.0.0` | `handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0` | pipeline resolve | Live resolution persists route basis, so the read-only inventory row is a semantic mismatch. |

These are SDK use-case identities, not operation identities or CLI paths. They
select typed projections but never appear in operation admission or discovery.
The use-case class closes transport parity without manufacturing semantic
authority. HCM-4.3 does not enlarge the 62-row HCM-4.2 inventory merely to make
CLI coverage uniform. A future slice may add an operation only with separately
authorized semantics, complete schemas, independent owner admission, and
transport proof.

This authority resolves the known predecessor exceptions:

- setup and doctor remain `sdk_composition`, `independent_owner_absent`,
  transport-empty, omitted rows; they use typed use-case responses over
  their existing typed SDK results and do not self-admit as operations; and
- `pipeline.route.resolve@1.0.0` remains `semantic_mismatch`, transport-empty,
  and omitted because live route resolution persists the route basis; the
  current CLI resolve invocation uses a typed use-case response and must
  not invent a read-only operation or hide that mutation.

No other leaf may enter the use-case registry during this slice. A mismatch in
one of the mandatory ordinary mappings returns `STOPPED` with the exact
semantic conflict rather than silently reclassifying it.

## Stable transport contract

### Typed result boundary

The SDK contract membrane owns one public outer machine root,
`handbook.invocation-response@1.0.0`. Its Rust projection is
`InvocationResponseV1`. It has an exact `schema` binding, a closed
`response_kind` discriminator with only `operation`, `use_case`, and
`transport_failure`, exactly one matching `response` body, and an outer
fingerprint over the binding, discriminator, and body. Unknown variants or a
body/discriminator mismatch fail closed.

The three bodies are:

1. an HCM-4.2 `OperationResponseV1` or `OperationAdmissionRefusalV1` for an
   ordinary mapping;
2. the SDK-owned `handbook.use-case-response@1.0.0` projection, a closed tagged
   union over the six registry rows above, carrying the exact use-case and
   success-result-schema bindings plus shared terminal fields, receipts,
   diagnostics, next actions, and fingerprints; or
3. the SDK-owned `handbook.transport-failure@1.0.0` projection for repository,
   acquisition, decoding, framing, serialization, or emission failure after
   leaf recognition.

All three public DTO shapes, their constructors and validation, and their
schemas live in `handbook_sdk::contract_membrane`. The existing feature-gated
SDK generator writes their self-contained Draft 2020-12 roots into
`crates/sdk/schemas/2020-12/` and adds their exact bytes to the single
`crates/sdk/schemas/manifest.json`. The SDK gains no dependency on the CLI.
The CLI supplies bounded adapter-failure facts to SDK constructors and owns
framing and presentation, but it cannot define or mutate DTO shape.

Use-case and transport-failure bodies are not semantic operation definitions:
neither may impersonate an owner response, claim operation admission, or
become discoverable in the HCM-4.2 catalog. Bootstrap/catalog documents retain
their distinct HCM-4.2 types if a command consumes them in the future; this
slice does not create such a command.

Each use-case variant owns a concrete bounded Rust success-result projection.
On `ok`, `data` must validate against the exact success-result schema in the
registry row; on any other terminal, `data` is null and every `Problem.details`
binding must belong to that use case's closed checked-in detail-schema set.
The generated `handbook.use-case-response@1.0.0` root expresses the same
six-way discriminator/result-binding closure, so an arbitrary
`ContractValue` cannot satisfy a success variant. Use-case refs, result refs,
and allowed detail refs are validated together by SDK constructors and typed
deserialization.

Every `InvocationResponseV1` carries enough closed state for all three
consumers:

- the machine serializer emits the exact outer invocation document;
- the human renderer derives display text without rerunning semantic logic;
  and
- the exit policy reads the same terminal classification without inspecting
  rendered text, except for the physical output-failure override below.

No renderer may manufacture a success, refusal, blocker, error, receipt,
diagnostic, or next action. No command handler may emit response fragments
before its complete typed result and selected rendering are ready. Human input
prompts remain CLI-owned acquisition behavior; machine mode never prompts.

### Machine mode and framing

Under this authority, **machine mode means `--json`**. It is the uniform
selector for every supported nontrivial invocation and remains accepted in the
existing command-local positions. The implementation may normalize it to one
global parsed option, but must not require scripts to move an already valid
flag. `--legacy-json` is the compatibility projection defined below, not the
new machine protocol.

Machine mode obeys all of these rules:

1. stdout contains one complete UTF-8 JSON document followed by exactly one LF;
2. the document validates against the checked-in, versioned, closed
   `handbook.invocation-response@1.0.0` root and passes the same schema-binding
   and fingerprint checks as the in-memory result, plus owner admission checks
   when it carries an ordinary operation body;
3. no heading, prompt, progress message, log line, warning, fallback prose,
   payload prelude, or second JSON value appears on stdout;
4. semantic diagnostics and next actions remain fields in the response;
   operational progress and logs, when present, go only to stderr; and
5. secrets, raw idempotency keys, private owner records, host paths not selected
   for public projection, and debug representations never enter either stream.

Serialization is completed before the first stdout byte. For a writable stdout
transport, every recognized machine invocation therefore emits exactly one
complete response document. A bounded serialization or presentation failure
produces an invocation response with a typed transport-failure body. If the
operating system rejects the stdout write, including a broken pipe or a failure
after accepting only a prefix, the process cannot retract accepted bytes: it
writes no second document, may write one non-secret operational diagnostic to
stderr, and exits 1. That physical emission failure is the sole exception to
the one-complete-document guarantee and is tested as transport behavior, not
represented as a false semantic response.

### Human rendering

Human mode remains the default. Existing human wording, ordering, payload-only
compile behavior, and stream placement are compatibility anchors wherever
current tests freeze them. Renderers consume only `InvocationResponseV1`.
Raw `pipeline compile` payload mode remains a human projection; JSON mode places
that payload in typed result data and never writes a second payload document.

Human output may be improved only when the assigned packet records the exact
compatibility reason and updates focused snapshots. One required stream
migration is already authorized: artifact semantic refusals remain typed human
output on stdout, while the duplicate `artifact operation refused: ...`
stderr line is removed. Existing tests that freeze that duplication must be
updated to assert the clean split. HCM-4.3 does not authorize unrelated UX
redesign.

### Diagnostics and stderr

Semantic diagnostics are data and stay inside the typed result in both modes.
Human rendering may display them; machine serialization preserves them as
fields. Stderr is reserved for transport-operational progress/logging and the
last-resort emission failure above. A semantic `blocked`, `refused`, or `error`
outcome is not duplicated as ad hoc stderr prose.

### Terminal projection

All response bodies preserve HCM-4.2 terminal closure: `ok` carries non-null
data and no terminal problems; `blocked`, `refused`, and `error` carry null data
and populate only their matching problem array. A non-`ok` use-case response
places the complete typed owner result needed by the human renderer in the
terminal problem's schema-bound `details`, so the renderer never reruns or
loses diagnosis. Use-case receipts are separate from terminal data and record
every write already realized by a composite SDK use case, including a write
that precedes a non-`ok` final readiness status. This does not relax the
ordinary-operation receipt rules.

Owner-native results map by cause, not wording:

- a completed semantic result is `ok`;
- an unmet external prerequisite with a defined recheck is `blocked` and uses
  a `prerequisite` problem;
- caller-controlled schema, compatibility, capability, resolution, authority,
  precondition, safety, or idempotency rejection is `refused`; and
- owner implementation failure or adapter execution failure is `error`.

Repository readiness is frozen more specifically because setup and doctor
share it:

| `RepositoryReadinessStatus` | Terminal | Typed projection |
| --- | --- | --- |
| `Ready` | `ok` | The complete readiness report is `data`. |
| `ActionRequired` | `blocked` | One `prerequisite` blocker with code `repository.action_required`; its details carry the complete report and its recheck/next actions identify the required authoring or setup action. |
| `Indeterminate` | `blocked` | One `prerequisite` blocker with code `repository.readiness_indeterminate`; its details carry the complete report and its recheck identifies the unavailable evidence. |
| `Invalid` | `blocked` | One `prerequisite` blocker with code `repository.readiness_invalid`; its details carry the complete report and its recheck/next actions identify repository repair. |

This table preserves the current `0` for `Ready` and `1` for all other
readiness statuses while selecting one stable terminal meaning. Closed setup or
doctor errors outside the readiness report follow the cause table above.

The transport-failure body uses the same cause rule: unavailable repository
prerequisites are `blocked`, caller-controlled acquisition or decoding is
`refused`, and I/O, framing, serialization, or adapter-internal failure is
`error`. It never reports `ok` and never carries write receipts.

### Exit policy

The stable process table is:

| Condition | Exit |
| --- | ---: |
| typed terminal `ok` | `0` |
| typed terminal `blocked`, `refused`, or `error` | `1` |
| recognized invocation with CLI transport failure | `1` |
| physical failure while emitting the selected output | `1`, overriding a previously derived typed-terminal exit |
| Clap help or version | `0` |
| Clap syntax/usage failure before leaf dispatch | `2` |

Other owner-specific statuses must map exhaustively into the shared terminal
set under the cause rule above; renderers and command paths may not add a
second exit table. Signal termination and operating-system launch failure
remain outside the Handbook result contract.

## Compatibility and migration

Default human behavior is preserved during the migration. Existing
command-specific JSON for Charter authoring, approver administration,
artifacts, and doctor is a real compatibility surface, but it is not the final
shared protocol.

- `--json` selects the new versioned shared response contract.
- A separately named `--legacy-json` compatibility selector preserves each
  currently tested legacy document byte/field behavior while its replacement
  is introduced. It is accepted only on commands that already had JSON before
  this slice and is mutually exclusive with `--json`.
- Legacy projection must consume the same typed result as the new renderer; it
  may not retain a parallel semantic execution path.
- On a writable stdout transport, `--legacy-json` emits exactly one clean
  legacy JSON document with its currently tested terminal LF and no prose or
  second value. It preserves the legacy field/byte contract and derives the
  same typed-terminal exit as human and `--json` modes, but it does not validate
  against `handbook.invocation-response@1.0.0`. Physical output failure uses
  the same exit-1 override and no-retry rule.
- Help and migration documentation identify the replacement schema and field
  mapping. Legacy mode is advisory-deprecated, is not capability evidence, and
  has no removal date in this slice.
- Removing legacy mode, renaming existing commands, or changing default human
  output requires later explicit authority and consumer evidence.

This is the bounded compatibility cost of making `--json` mean one protocol
across the whole CLI rather than perpetuating command-specific shapes.

## Cross-packet invariants

Every packet must preserve these invariants:

1. Engine, Flow, and pipeline retain their semantic decisions; SDK composition
   and CLI presentation cannot self-admit owner truth.
2. Stable exact operation refs, not CLI paths, select ordinary operation
   semantics; SDK-owned use-case refs never masquerade as operation refs.
3. Every supported leaf has one result-class mapping, one typed owner/SDK
   source mapping, and one terminal mapping; unclassified fallthrough is a
   failure.
4. Ordinary machine responses preserve HCM-4.2 terminal closure, schema
   binding, fingerprints, idempotency state, and realized-write receipts;
   SDK-owned use-case and transport responses preserve the same bounded public
   primitives without claiming operation admission, and use-case receipts do
   not conceal writes that precede a non-`ok` composite outcome.
5. Instance bytes, checked-in schema bytes, and CLI LF framing remain separate
   identity domains.
6. Machine stdout is prepared as one singular document; a rejected or partial
   operating-system write never triggers a second document, and
   progress/logging never contaminates stdout.
7. Human and machine renderers never rerun owner logic or derive different
   terminal outcomes.
8. On equivalent writable output transports, exit status is derived from typed
   terminal state before rendering and is identical for human, `--json`, and
   legacy modes. A physical output failure overrides that derived status to 1
   and never triggers a second document.
9. Legacy JSON is a projection-only migration surface and never current
   transport or capability evidence.
10. Raw keys and private owner state remain absent from responses, diagnostics,
    logs, snapshots, fixtures, and failure fallbacks.

## Ordered packet decomposition

| Packet | Bounded outcome | Dependency |
| --- | --- | --- |
| **1 — invocation envelope, coverage, and doctor pilot** | Encode the authority's complete command/mode coverage table; add the SDK-owned invocation, use-case, transport-failure, six success-result, and initial detail roots to the existing generator/tree/manifest; implement the closed outer and use-case discriminators, uniform `--json` selection, one exit table with its physical-output override, and doctor as the first end-to-end use-case/new-JSON/legacy-JSON proof. Preserve doctor as `independent_owner_absent`, transport-empty, and omitted from operation discovery. | HCM-4.2 closeout, satisfied. |
| **2 — setup parity** | Move implicit setup, init, and refresh through the SDK-owned repository-setup use-case response; preserve setup human behavior; implement the frozen readiness terminal table, full problem-detail projection, and realized-write receipts without self-admitting setup as an operation. | Packet 1. |
| **3 — author and approver parity** | Close Charter, Project Context, and approver administration through their three fixed SDK use-case refs and shared terminal results; retain current human and explicit legacy JSON projections without retaining parallel execution logic. | Packet 2. |
| **4 — artifact parity** | Close all ten exact artifact/intake ordinary mappings with complete schemas, independent Engine admission, `cli_json` evidence, and receipts for mutations; replace ad hoc `serde_json::Value` assembly, emit singular stdout, and remove duplicate artifact-refusal stderr prose. | Packet 3. |
| **5 — Flow generate and inspect parity** | Close `flow.resolve@1.0.0` with its exact schemas, independent Flow admission, and `cli_json` evidence; make generate and inspect render human Markdown/inspection text and machine JSON from that one ordinary response while preserving fixture/refusal/blocker/budget truth and current human behavior. | Packet 4. |
| **6 — pipeline read and resolve parity** | Close catalog list/read and compile as the three fixed ordinary mappings with exact schemas, independent pipeline admission, and `cli_json` evidence. Preserve `pipeline.route.resolve@1.0.0` as `semantic_mismatch`, transport-empty, and omitted because live resolution persists route basis; project resolve through its fixed SDK use case. Preserve raw human payload mode while machine output remains one invocation document. | Packet 5. |
| **7 — pipeline mutation parity** | Close capture plan/apply, handoff emit, and state apply as the fixed ordinary mappings with independent pipeline admission, exact idempotency/compare-and-write truth, `cli_json` evidence, and receipts wherever realized writes are represented. Direct capture maps to capture apply, emits one invocation envelope with the final ordinary body, and cannot conceal a partial write. | Packet 6. |
| **8 — integrated parity and migration proof** | Prove complete leaf/result-class coverage, deterministic schema regeneration for every public response root, human/machine terminal parity, one-document stdout on writable transports, write-failure behavior, stderr separation, exit codes, raw-key absence, legacy projection compatibility, and catalog/discovery updates exactly for the newly closed ordinary operations while all use cases remain absent from operation discovery. Update user-facing CLI protocol documentation without creating another closeout layer. | Packets 1–7. |

The dependency order is strictly sequential:

```text
HCM-4.2 -> P1 -> P2 -> P3 -> P4 -> P5 -> P6 -> P7 -> P8
```

Each packet returns control to the meta task. A packet may not dispatch the next
packet or claim slice completion. One aggregate review and closeout occurs only
after Packet 8 and all slice acceptance conditions are satisfied.

## Delegated just-in-time preparation

The user's packet-by-packet execution request delegates bounded preparation for
Packets 1 through 8 in the same fresh top-level task that implements each
packet. Before product edits, the task must reconcile its packet against this
authority, all HCM-4.2 checkpoints, current source/tests, the implemented
baseline, architecture context, seam crosswalk, and phase map.

The only adopted historical input is the HCM-4.3 subsection of the archived
`04-phase-slice-map.md` and the CLI transport/CLI JSON rows of the archived
`03-seam-crosswalk.md` at ref
`archive/handbook-contract-membrane-pre-clean-20260812`. They supply the bounded
intent for singular stdout JSON, typed human rendering, stderr logging, stable
exit behavior, operation mapping, and machine-clean output. Historical status,
selectors, proof walls, handoffs, and closeout procedures remain inactive.

The packet task may select private helper/module names, schema filenames,
test-fixture factoring, and non-semantic renderer factoring. It may not add an
operation merely because a CLI leaf lacks an HCM-4.2 match: only the closed
use-case registry above may use a use-case response. It may not guess a
semantic owner, change an existing operation's meaning, weaken owner admission,
add a new product command, or broaden the slice. Any operation-inventory
expansion requires new
explicit semantic authority outside this packet sequence. If current permitted
sources leave more than one consequential result-class, owner, mutability,
write-set, or compatibility answer after applying the resolutions in this
document, return `STOPPED` before product edits with that exact decision.

## Slice acceptance

HCM-4.3 completes only when all of the following hold:

1. Every supported nontrivial leaf/mode in this document appears exactly once
   in an executable coverage table and has the exact ordinary-operation or
   SDK-use-case mapping fixed by this authority.
2. Every mapped ordinary CLI JSON operation has exact request/result/outcome
   schemas, independent owner admission, current `cli_json` transport evidence,
   and discovery state consistent with the production witness. The fixed SDK
   use-case and transport-failure bodies have exact public schemas but do not
   appear in operation admission or discovery.
3. On a writable stdout transport, every `--json` success, blocked, refusal,
   error, and transport failure emits exactly one schema-valid
   `handbook.invocation-response@1.0.0` document plus one LF with no other
   stdout bytes. Broken-pipe and partial-write tests prove exit 1, no retry or
   second document, and only a bounded non-secret stderr diagnostic.
4. Human, `--json`, and legacy outputs for the same invocation derive from one
   typed result and agree on result class, ordinary operation identity when
   present, terminal state, receipts, diagnostics, next actions, and exit code
   under equivalent writable output conditions. Physical output failure is the
   sole exit override.
5. The documented `0/1/2` exit table is exhaustively tested across all command
   families; no renderer or command-local branch owns a competing table.
6. Progress/logging uses stderr only, semantic diagnostics remain in the result,
   and semantic refusals do not produce duplicate stderr prose.
7. Current human behavior and payload-only compile output remain compatible;
   all previously supported JSON documents remain available through tested
   `--legacy-json` projections, have a documented migration mapping, and retain
   singular clean legacy framing without being treated as invocation-envelope
   documents.
8. The existing SDK schema generator and exact-byte manifest cover the outer
   invocation root, its use-case and transport bodies, all six use-case success
   result roots, all use-case detail schemas, and every new ordinary
   request/result/problem root, and regenerate deterministically without
   silently expanding the operation inventory.
9. Raw idempotency keys, private owner records, and debug-only values remain
   absent from machine/human output, stderr, fixtures, and checked-in schemas.
10. Focused packet evidence passes, Packet 8 proves cross-command integration,
    required GitNexus change detection reports only expected symbols/flows, and
    one aggregate causal review has no unresolved P1 or P2.

## Non-goals

HCM-4.3 does not authorize:

- Tauri commands or UI work;
- the HCM-4.5 Handbook skill or prompt-owned semantics;
- a new top-level product command, command renaming, or unrelated UX redesign;
- Phase-5 contract, evidence, gate, or dock owners;
- publication, release, a Substrate bridge, or external adoption;
- broader posture policy, grounding adoption, or deferred Phase-3 work;
- removal of legacy JSON compatibility; or
- orchestration ledgers, selectors, proof walls, handoffs, durable context
  capsules, or another HCM-4.2 closeout.

## Use and drift

Use this document as the completed HCM-4.3 outcome and packet boundary. Source
and tests remain product truth. If later authorized work changes the supported
CLI surface, operation inventory, transport evidence, or schema manifest,
update the implemented baseline, phase map, architecture context, and seam
crosswalk only where their owned current truth changes. Do not use this
closeout as authority to execute HCM-4.4 or later work.
