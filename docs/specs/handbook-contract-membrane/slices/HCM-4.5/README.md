# HCM-4.5 Capability-driven Handbook Skill

This document owns the outcome authority and current implementation status for
HCM-4.5. It defines exactly one slice outcome: a deliberately thin installed
Handbook skill that consumes ordinary capabilities through the Phase-4
bootstrap, catalog, schema, and invocation membrane. It does not authorize
Phase 5 or Phase 6, convert fixed SDK use cases into ordinary capabilities, or
make a CLI or Tauri command name a source of semantic identity.

## Current admission state

**Packets 1–6, the one aggregate verification, the aggregate causal review,
and aggregate slice closeout are complete. HCM-4.5 is `SLICE_COMPLETE`.**

The hard predecessor, HCM-4.4, is complete at `b145ce2`. Its Packets 1–6
completed sequentially through `3ae846c`, `1fc369a`, `d12cae8`, `2042b6f`,
`083123f`, and `11d8d3a`. The final product state proves the immutable
bootstrap/catalog witness, 120 exact schema roots, 19 discoverable ordinary
capabilities, the exact current transport vectors, five transport-neutral
facade roots, three Tauri transport commands, six fixed non-discoverable SDK
use cases, and owner-preserving read-only and mutation invocation.

That evidence satisfied HCM-4.5's predecessor. Authority was prepared at
`3df62da`; Packets 1–6 were implemented sequentially at `81ddfd7`, `23e0a7c`,
`8dd1627`, `7215fee`, `d507143`, and `c52b1b7`.

Aggregate verification on the final product state passed the SDK Packet-1
suite (4/4), CLI Packet-2–6 suites (29/29), repaired feature-gated integrated
schema/catalog proof (3/3), and Tauri integrated facade proof (1/1). Packet-6
evidence also passed HCM-4.3/HCM-4.4 parity, deterministic 120-root schema
regeneration, temp-HOME source/release/public installation smoke, formatting,
workspace check, strict Clippy and rustdoc, dependency/source gates, and
Markdown links.

The aggregate causal review found one authority/evidence mismatch: the sealed
production witness has no truthful test-only seam for fabricating a newly
admitted capability, while the original criterion 11 required exactly that
construction. The accepted Packet-6 authority already required the strongest
truthful alternative when injection would change production inventory or
semantics. Criterion 11 below now states that bounded proof directly: dynamic
structural selection from the admitted catalog plus source gates, together with
the invariant that any real later capability first completes product-owned
definition, schema, owner-route, admission, and transport closure. No product
behavior was weakened or changed by this correction. The controlling aggregate
review has no unresolved P1 or P2.

The separate Phase-4 exit criteria are satisfied by the completed Phase-4
product evidence. This closeout does not authorize Phase 5, resolve its owner
placement decision, or start any later slice.

## Outcome

Replace the root installed Handbook skill's current command-oriented ordinary
workflow with one capability-driven workflow over a small generic machine
adapter. The workflow:

1. negotiates the current bootstrap contract;
2. pages the admitted capability catalog rather than carrying an internal
   operation inventory;
3. selects semantic identity only from an exact discovered operation binding
   and pins the selected capability entry fingerprint;
4. obtains the exact request, result, outcome, and shared response schemas by
   their manifest-bound fingerprints;
5. gathers one structured operation body from those contracts and admits it
   before an owner call;
6. invokes only when the selected capability currently exposes the adapter's
   `cli_json` transport; and
7. consumes the returned typed terminal, data or problems, diagnostics, next
   actions, idempotency state, receipts, and replay truth without deriving or
   reclassifying owner semantics.

The selected direction is:

```text
user intent
    -> installed Handbook skill
    -> generic CLI membrane actions
    -> SDK bootstrap / catalog / exact schema assets
    -> discovered exact operation binding + entry fingerprint
    -> structured body admission
    -> current cli_json transport evidence
    -> existing SDK dispatcher -> Engine / Flow / pipeline owner
    -> exact InvocationResponseV1
    -> faithful skill presentation
```

The skill and generic CLI adapter contain no table or branch over the current
19 operation refs. A later capability still requires its own definition,
schemas, owner admission, dispatcher route, and current transport evidence,
but once those product-owned facts exist, ordinary skill uptake must occur
through discovery and contracts without capability-specific skill prose or a
new CLI command.

## Current source boundary

Current source and tests are product truth. This authority is selected from:

- the current program [`README`](../../README.md),
  [`phase-and-slice map`](../../phase-and-slice-map.md),
  [`implemented baseline`](../../implemented-baseline.md),
  [`architecture context`](../../architecture-context.md), and
  [`seam crosswalk`](../../seam-crosswalk.md);
- the completed [`HCM-4.2`](../HCM-4.2/README.md),
  [`HCM-4.3`](../HCM-4.3/README.md), and
  [`HCM-4.4`](../HCM-4.4/README.md) outcomes and their linked focused packet
  checkpoints where needed;
- `install/handbook-home/SKILL.md.tmpl`,
  `install/handbook-home/charter-intake/SKILL.md.tmpl`, the projection and
  install scripts under `tools/codex/`, and their current CLI/install tests;
- the SDK bootstrap, typed catalog, production witness, operation,
  invocation, facade, schema-generation, and response modules under
  `crates/sdk/src/contract_membrane/`;
- the focused SDK contract, schema, integrated-proof, and HCM-4.4 parity tests;
  and
- the current CLI and Tauri adapters plus their focused parity tests.

No historical material is adopted. If a packet later needs lineage for one
bounded current decision, it must return `STOPPED` unless the meta task grants
that historical input explicitly; packet work may not silently reactivate a
pre-clean skill or control package.

The current root skill is the generic capability algorithm and the installed
projection contains its `SKILL.md` plus `agents/openai.yaml`; the separate
Charter leaf remains available without entering ordinary discovery. The CLI
exposes only `bootstrap`, `catalog-read`, `schema-read`, and `invoke` below the
machine-only capability group. The SDK consumer and existing Tauri facade
retain their transport-neutral contracts and owner routes.

The production witness is deliberately closed over the current manifest,
definitions, owner evidence, inventory, and transport evidence; it has no
truthful test-only injection seam for a synthetic twentieth capability. Packet
6 therefore uses the authority-permitted strongest proof: it dynamically
selects an already admitted read-only `cli_json` capability by structural
properties and exact schema acceptance, without naming that operation in the
selection test, generic adapter, or installed skill. This proves routing
independence for an admitted capability, not literal admission of a fabricated
future capability. A real later capability must first gain complete
product-owned definition, schemas, owner route, admission, and current
`cli_json` evidence; the unchanged generic skill and four CLI actions then
consume it through discovery.

## Selected consumer and adapter shape

### Deep SDK consumer seam

The SDK remains the deep module. Add one small consumer interface over the
existing production witness and invocation dispatcher that can:

- negotiate the current bootstrap request and return the existing
  `BootstrapFacadeResponseV1` family;
- read the existing capability, schema, or profile catalogs with their typed
  cursor/retention behavior and return `CatalogReadResponseV1`;
- resolve exact schema bytes only when the requested exact ref and fingerprint
  match the current manifest;
- locate one discovered capability by its exact operation binding and
  discovery-time capability-entry fingerprint, compare both against the current
  witness, require the requested transport in both its declared and current
  transport evidence, validate the structured body against its exact request
  schema, and construct the existing definition-bound operation and invocation
  requests; and
- dispatch once through the current owner route and re-admit the exact
  `InvocationResponseV1` against the request, definition, schema, and owner
  outcome.

Private helper names and module factoring remain packet decisions. The
interface must not expose or accept owner-admission tokens, private authority
records, trusted time, realized receipts, terminal status, or fingerprints as
caller-authored truth. It may accept transport-local request correlation and a
raw operation body because those are adapter inputs; it must derive every
semantic binding from the current witness.

The interface does not admit the six fixed use cases. They remain outside the
operation catalog and generic capability workflow. The existing separately
installed Charter intake skill remains a fixed workflow and is not rewritten
as an ordinary capability. Other fixed use cases retain their existing SDK and
CLI surfaces but gain no generic discovery identity in this slice.

### Generic CLI machine actions

The installed binary gains one machine-only `handbook capability` group with
exactly four generic actions:

| Action | Semantic input | Exact success output |
| --- | --- | --- |
| `bootstrap` | adapter-supported client version, bounded page size, and optional correlation | existing `BootstrapFacadeResponseV1` plus one CLI LF |
| `catalog-read` | catalog kind, bounded page size, and optional exact typed cursor | existing `CatalogReadResponseV1` plus one CLI LF |
| `schema-read` | exact schema ref and expected manifest fingerprint | the exact checked-in Draft 2020-12 schema bytes, including their one tracked LF |
| `invoke` | exact discovered operation ref, definition fingerprint, capability-entry fingerprint, optional correlation, and one structured JSON body from a file or stdin | existing `InvocationResponseV1` plus one CLI LF |

These names identify transport actions only. `invoke` derives operation
identity from the discovery-time exact operation binding and capability-entry
fingerprint, then requires an exact match in the current witness before body
admission or owner execution. It never derives identity from the command path,
a friendly label, the skill text, or a Tauri command ID. It constructs and
re-admits the existing `OperationRequestV1` and `InvocationRequestV1`
internally; the skill copies discovered fingerprints and never authors them.

The group is additive and machine-only. It does not rename or reinterpret an
existing human or JSON command, become an ordinary operation, add a capability
row, expose the six fixed use cases, or add another machine DTO family. The
bootstrap, catalog, operation request, invocation request/response, and schema
roots remain the existing 120-root Phase-4 set.

On success each action writes exactly its declared single value and no prose to
stdout. Bootstrap refusal, catalog blocking or cursor mismatch, and invocation
non-success preserve their existing typed result family and exit nonzero.
Syntax, acquisition, unavailable binding, manifest mismatch, incompatible
version, unavailable transport, or invalid body fails before owner execution;
it uses an existing typed membrane failure when that family can truthfully
represent the condition, otherwise it leaves stdout empty, emits one bounded
non-secret adapter diagnostic to stderr, and exits nonzero. It must never
manufacture an owner refusal, blocker, error, success, receipt, or replay state.

### Skill workflow

The installed root `handbook` skill owns only the generic ordinary-capability
algorithm:

1. Resolve a real repository root and the installed runtime without accepting
   a repository path from untrusted operation input.
2. Run bootstrap and admit its schema binding, version negotiation, catalog
   roots, owner versions, manifest, first capability page, and fingerprints.
3. Page capabilities with the returned exact cursor until one unambiguous
   operation binding matches the user's requested effect. Retain that exact
   binding and its capability-entry fingerprint as the invocation selection.
   Do not infer an operation from CLI/Tauri names and do not guess when no
   single capability matches.
4. Require `cli_json` in both declared and current transports. Treat an absent,
   deprecated-incompatible, schema-incomplete, version-incompatible, or
   transport-unavailable capability as unavailable; report its typed facts and
   do not fall back to a legacy command or use case.
5. Retrieve the exact request, result, blocker, refusal, error, shared
   invocation-response, and any required-capability schemas from manifest-bound
   refs. Ref/fingerprint mismatch, a changed capability-entry fingerprint, or
   missing schema stops before invocation.
6. Gather a closed JSON body according to the exact request schema. Preserve
   unknown user facts as unresolved rather than inventing values; keep raw
   idempotency keys or other sensitive request fields in bounded temporary
   input and out of logs, responses, fixtures, and skill summaries.
7. For a non-read-only class, require explicit authority for that exact
   mutation and respect the discovered idempotency kind and key/basis pointers.
   Never retry automatically after delivery uncertainty or an unknown
   post-commit result.
8. Invoke once through the generic action and treat the typed response as
   controlling. Present the exact status; matching data or problem collection;
   diagnostics; next actions; idempotency state; receipts; result and response
   fingerprints; and replay indicator without translating one terminal into
   another.

The skill may summarize typed details for the user, but it cannot replace them
with prompt-owned diagnosis or claim a mutation absent receipts and owner
truth. Fixed non-discoverable use cases are not enumerated, synthesized, or
invoked through this algorithm. The separate `handbook-charter-intake` skill
remains available for its existing fixed Charter workflow.

## Trust, mutation, and replay boundary

Discovery is evidence, not authority. A capability entry proves current
definition/schema/owner/transport closure; it does not let the skill supply
owner tokens, policy results, trusted time, repository identity, mutation
receipts, terminal state, or replay truth. The SDK and semantic owner re-admit
the constructed request and resolve all private authority.

Read-only operations may execute after exact input admission when the user's
request selects one unambiguous effect. Append-only or compare-and-write
operations require explicit authorization for the exact discovered operation
and input. The skill must surface the discovered class and idempotency contract
before execution, preserve the raw key only in request acquisition, and never
invent or rewrite expected-basis values.

An owner `ok`, `blocked`, `refused`, or `error` remains that terminal. The skill
does not infer success from exit code, prose, changed files, empty problems, or
receipts. Receipts and replay fields are copied from the typed response only.
Same-key replay may be requested only after a complete prior response or as the
owner-defined recovery path for delivery uncertainty; the adapter never retries
automatically. A replay must preserve the original semantic result fingerprint,
terminal data or problems, receipts, and idempotency truth without another
owner write.

## Scope and allowed implementation boundary

Slice implementation may change only:

- directly necessary SDK contract-membrane consumer, production-witness,
  invocation-dispatch, export, schema-asset access, and focused-test paths under
  `crates/sdk/**`;
- directly necessary generic machine-adapter parsing/emission and focused tests
  under `crates/cli/**`;
- `install/handbook-home/**`, generated `.agents/skills/handbook/**`, and the
  directly necessary projection, install, release-package, and install/live
  smoke paths under `tools/codex/**`, `tools/release/**`, and `tools/ci/**`;
- directly necessary workspace or crate manifests only if the selected
  implementation cannot use current dependencies; and
- current HCM-4.5 and directly necessary program/navigation prose after
  executable truth changes.

Engine, Flow, pipeline, Tauri product source, the operation inventory, current
operation definitions, checked-in schema roots, and the six fixed use-case
contracts are read-only. Existing CLI product commands and their human,
`--json`, and `--legacy-json` behavior are compatibility anchors. If the generic
consumer cannot be closed without changing an owner semantic, adding a
contract root, granting a new transport to an operation, or converting a use
case, the responsible packet returns `STOPPED` rather than widening scope.

## Cross-packet invariants

Every packet must preserve these invariants:

1. Engine, Flow, and pipeline remain the only semantic and mutation owners;
   the SDK consumer seam, CLI adapter, and skill cannot self-admit owner truth.
2. The operation catalog remains the sole ordinary capability inventory. The
   skill and generic CLI adapter contain no hardcoded current operation list or
   capability-specific workflow branch.
3. The discovery-time exact operation binding and capability-entry fingerprint
   jointly pin the selected semantics and current closure. CLI/Tauri command
   names, help labels, schema filenames, and prompt wording never do.
4. A generic invocation requires the selected transport in both declared and
   current capability evidence. HCM-4.5 adds no `cli_json` evidence to posture
   or any other operation.
5. The six fixed SDK use cases remain outside owner admission and capability
   discovery and cannot enter the generic capability path.
6. Bootstrap/catalog/invocation values retain their existing distinct DTO
   families, closed discriminators, canonical fingerprints, typed cursors, and
   120-root schema/manifest identity. No new root or parallel DTO is created.
7. Exact schema bytes, instance canonical bytes, and CLI framing remain
   separate identity domains. Schema lookup requires both current ref and
   manifest fingerprint.
8. Current-witness comparison and structural body admission occur before an
   owner call, but semantic trust remains owner-controlled. Invalid or stale
   identity/entry fingerprint, version, schema, transport, or input produces no
   write and no fabricated owner terminal.
9. The typed owner terminal, data/problems, diagnostics, next actions,
   idempotency state, receipts, fingerprints, and replay flag cross SDK, CLI,
   and skill unchanged except for declared CLI framing.
10. Non-read-only invocation requires exact user authority; delivery
    uncertainty causes no automatic retry; same-key replay preserves the
    original result and receipts without duplicate mutation.
11. Raw idempotency keys, secrets, private owner records, host paths, debug
    values, and unbounded errors remain absent from outputs, logs, fixtures,
    checked-in skill prose, and schema assets.
12. Later admitted capabilities become usable after their owner route,
    definition, schemas, admission, and `cli_json` evidence exist, without a
    new skill branch or CLI action.

## Ordered packet decomposition

| Packet | Bounded outcome | Dependency | Allowed paths or boundary |
| --- | --- | --- | --- |
| **1 — discovered SDK consumer seam** | Add the small SDK interface that resolves current bootstrap/catalog/schema facts, requires the discovery-time exact operation binding and capability-entry fingerprint to match the current witness, enforces requested declared/current transport evidence, validates its raw body, constructs existing requests, dispatches once, and re-admits the response. Prove absent/incompatible/stale identity or entry fingerprint, schema, version, transport, and input fail before owner execution. No CLI or skill changes. | HCM-4.4 closeout, satisfied. | Directly necessary `crates/sdk/src/contract_membrane/**`, SDK exports, and focused SDK tests only. No owner, inventory, definition, schema-root, or use-case changes. |
| **2 — generic discovery and schema CLI actions** | Add the machine-only `handbook capability` group and close `bootstrap`, `catalog-read`, and `schema-read` over Packet 1. Prove exact typed pagination/cursor/retention outcomes, exact manifest-bound schema bytes, singular stdout framing, bounded failures, and no impact on existing command behavior. `invoke` remains default-denied. | Packet 1. | Directly necessary CLI parser/adapter/emission paths and focused CLI/SDK tests. Existing command handlers remain read-only. |
| **3 — generic read-only invocation** | Close `capability invoke` for discovered `cli_json` capabilities through the SDK consumer seam. Prove one representative read-only operation is selected from live discovery, its exact binding and entry fingerprint remain current, its request schema is retrieved, its structured body is admitted, its owner executes once, and the exact schema-valid `InvocationResponseV1` reaches stdout. Prove posture, fixed use cases, unknown refs, stale definition/entry fingerprints, invalid bodies, and command-name substitution cannot execute. | Packet 2. | Directly necessary CLI adapter and SDK consumer paths plus focused read-only and regression tests. No mutation route is exercised by this packet. |
| **4 — capability-driven skill and installed projection** | Rewrite the root `handbook` skill as the generic discovery/schema/input/invocation algorithm, retain the separate Charter intake projection without promoting it, and refresh repo-local/global/release packaging. Prove generated and installed file sets, discovery links, runtime prerequisites, no embedded current operation inventory, no product-specific ordinary workflow branches, and no raw sensitive request material in the asset. | Packet 3. | `install/handbook-home/**`, `.agents/skills/handbook/**`, directly necessary `tools/codex/**`, `tools/release/**`, `tools/ci/**`, and focused asset/install tests. The Charter leaf may receive only directly necessary routing clarification. |
| **5 — mutation, replay, and fail-closed skill proof** | Drive one representative discovered mutation (the current `intake.record.append@1.0.0` is the selected proof) through bootstrap, catalog, exact schema retrieval, structured input admission, explicit mutation authority, generic invocation, and typed output. Prove receipts and non-replayed truth on first execution, exact same-key replay without another write, conflicting/invalid input refusal, transport-unavailable denial, no automatic retry, and raw-key absence from output/log/fixture surfaces. | Packet 4. | Focused SDK/CLI/installed-skill harnesses and only corrections within Packets 1–4 boundaries. The representative ref is test evidence, not a skill or product routing branch. |
| **6 — integrated consumer proof and protocol documentation** | Prove the four generic actions, complete capability pagination, all exact schema retrievals needed by the representative flows, unchanged 120-root manifest and 19-capability witness, unchanged transport vectors and six use-case omissions, existing CLI/Tauri regressions, read-only and mutation end-to-end use, and future-capability uptake without skill/CLI routing changes. Update only current user-facing and HCM truth. This is an integration packet, not a second implementation authority or closeout layer. | Packets 1–5. | Focused SDK/CLI/install/Tauri-regression integration tests and directly necessary current documentation. Product corrections stay within an earlier packet boundary and invalidate affected evidence. |

The order is strictly sequential:

```text
HCM-4.4 -> P1 -> P2 -> P3 -> P4 -> P5 -> P6
```

Each packet returns control to the meta task. A packet may not dispatch the next
packet, claim HCM-4.5 or Phase-4 completion, prepare Phase 5, or perform
aggregate closeout.

## Delegated just-in-time preparation

The user's packet-by-packet execution request delegates bounded just-in-time
preparation for Packets 1 through 6 inside the fresh top-level task that
implements each packet. Separate packet files are optional. Before product
edits, a packet must reconcile its bounded outcome with this authority, live
source/tests, current program and architecture sources, the HCM-4.4 closeout,
and the completed predecessor checkpoints identified here.

Within its packet, the task may select private module/helper names, exact Clap
argument spelling below the four fixed action names, test fixture factoring,
temporary-file mechanics, and whether schema-byte access is exposed through
the production witness or a narrower SDK consumer helper. It may not change
the four action identities, existing DTO/schema families, exact transport
admission rule, capability-only workflow, mutation/replay contract, fixed
use-case exclusion, dependency direction, or allowed product boundary.

If current sources leave more than one consequential answer about schema-byte
authority, transport evidence, pre-owner validation, mutation authorization,
post-commit uncertainty, or typed failure representation, the packet returns
`STOPPED` before the affected edit with the exact missing decision. Missing
packet documentation or a private helper name is recoverable preparation, not
a stop.

## Slice acceptance

HCM-4.5 completes only when all of the following hold:

1. The installed root Handbook skill discovers ordinary capabilities from the
   current bootstrap/catalog surfaces and contains no hardcoded current
   capability inventory or capability-specific ordinary workflow branch.
2. One small SDK consumer interface derives definition/schema/owner bindings
   from the production witness, validates structured input, requires declared
   and current transport evidence, calls one existing owner route, and
   re-admits the exact response without semantic reimplementation.
3. The CLI exposes only the four generic machine actions fixed here. They are
   transport labels, not operation identities, and no existing command's human,
   `--json`, legacy, or exit behavior changes.
4. Bootstrap and catalog actions preserve version negotiation, owner/schema/
   profile roots, pagination, cursor, retention, refusal/blocking, exact
   fingerprints, and singular CLI framing in their existing response families.
5. Schema retrieval returns bytes only for an exact current manifest ref and
   expected fingerprint; missing, stale, substituted, or incompatible schema
   identity fails closed without an owner call.
6. Generic invocation accepts semantic identity only as one discovery-time
   exact operation binding plus capability-entry fingerprint, requires both to
   match the current witness, validates the body against the exact request
   schema, and emits the existing schema-valid
   `InvocationResponseV1` with no second value or prose on stdout.
7. The selected read-only proof discovers and invokes a real current operation
   end to end. Unknown/unavailable refs, invalid version/schema/input,
   transport absence, posture's lack of `cli_json`, fixed use cases, and
   CLI/Tauri name substitution all fail before owner execution.
8. The selected mutation proof discovers and invokes
   `intake.record.append@1.0.0` only after explicit mutation authority, preserves
   first-execution receipts and idempotency truth, and proves same-key replay
   returns the original semantic result and receipts without another write.
9. Owner `ok`, `blocked`, `refused`, and `error` fixtures cross the SDK, CLI,
   and skill with their matching data/problems, diagnostics, next actions,
   receipts, fingerprints, and replay truth unchanged; the skill never
   reclassifies from exit code, prose, or local observation.
10. The operation inventory remains exactly 62 rows, the schema tree exactly
    120 roots, the capability witness exactly 19 entries, current transport
    vectors unchanged, and all six fixed use cases absent from discovery and
    generic invocation.
11. A future-uptake proof dynamically selects an admitted read-only `cli_json`
    capability by structural catalog and schema properties without naming it in
    the generic adapter, installed skill, or selection test. Source gates show
    no current operation ref occurs in the installed root skill or generic
    capability adapter/parser; test evidence and pre-existing owner,
    definition, and typed-parity mappings remain outside that generic route.
    The proof records that the sealed production witness has no truthful
    synthetic-admission seam: a real later capability must first gain complete
    product-owned definition, schemas, owner route, admission, and current
    transport evidence, after which the unchanged skill and four generic
    actions consume it through discovery.
12. Root/leaf projection generation, source and release installation, discovery
    links, and live-skill smoke tests prove the capability-driven asset is what
    users receive while the fixed Charter skill remains separately available.
13. Raw idempotency keys, secrets, private owner records, host paths, debug
    values, and unbounded adapter errors are absent from machine responses,
    stderr fallbacks, fixtures, snapshots, installed skill prose, and checked-in
    schemas.
14. Focused SDK/CLI/install tests, representative end-to-end proofs, existing
    HCM-4.3/HCM-4.4 regressions, deterministic schema regeneration, formatting,
    workspace check, strict Clippy/rustdoc, dependency inspection, Markdown
    links, and required GitNexus change detection pass or are reported
    honestly.
15. One aggregate causal review of the complete slice has no unresolved P1 or
    P2 after all packet evidence remains valid on the final product state.

## Aggregate verification and closeout

Each packet runs focused verification for its own boundary and returns its
commit and evidence to the meta task. Packet review applies only when a
material packet decision or measured risk warrants it under the current review
policy. Packet completion never closes the slice.

After Packet 6, and only after every acceptance condition is proven, the meta
task performs one aggregate verification over the final SDK, CLI, skill,
projection/install, and representative operation paths and one focused causal
review using the P1–P4 rubric. Valid packet evidence may be reused; checks
invalidated by later packets must be rerun. Blocking P1/P2 findings are
remediated and reviewed again within current authority or produce a genuine
workflow stop. P3/P4 advisories remain non-blocking.

The meta task then records one aggregate HCM-4.5 closeout and evaluates Phase-4
exit separately. No packet creates a slice closeout, phase-exit claim, ledger,
selector, proof wall, handoff, durable capsule, or mechanical closeout commit.

## Non-goals

HCM-4.5 does not authorize:

- Phase-5 contract, evidence, verdict, gate, dock, or validator semantics;
- a hardcoded current capability list, capability-specific prompt branches, or
  one CLI command per operation;
- conversion, enumeration, or generic invocation of the six fixed
  non-discoverable SDK use cases;
- new operation rows, definitions, owner claims, schema roots, transport
  evidence, semantic routes, or a CLI posture transport;
- SDK or skill reimplementation of Engine, Flow, or pipeline decisions;
- Tauri product changes, UI, publication, bridge, Substrate adoption, plugin
  platform, or speculative workflow-engine scope;
- a CLI subprocess behind Tauri, parsing existing human/legacy command output,
  or fallback from the generic membrane to a legacy command;
- automatic mutation retries, prompt-invented authority or inputs, or terminal
  reclassification;
- adjacent slice preparation, Packet 1 dispatch, or orchestration artifacts.

## Use and drift

Use this document as the current HCM-4.5 outcome and packet authority. Source
and tests remain product truth. If later authorized implementation changes the
consumer interface, installed skill, capability evidence, schema count,
transport vectors, or fixed-use-case boundary, update only the current HCM
document that owns that truth. Do not use this authority to execute Phase 5 or
any adjacent work.
