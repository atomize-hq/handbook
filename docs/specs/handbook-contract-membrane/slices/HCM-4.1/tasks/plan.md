# HCM-4.1 executable SDK ownership plan

## Planning boundary

This is the frozen implementation plan. The operator continuation waiver admits
the ordered HCM-4.1 implementation only under its exact boundaries. The first
typed Rust posture method is callable by direct linkage but is not discoverable,
schema-published, bootstrap-advertised, JSON-serializable, or
transport-supported. Canonical operation discovery remains HCM-4.2.

## Operator-authorized implementation sequence

| Packet | Scope | Compatibility and proof boundary |
|---|---|---|
| 1 | Create `handbook-sdk`, the smallest repository-bound opening surface, direct typed posture request/outcomes, and purpose-named engine facade. | Keep all CLI routes and compiler behavior unchanged; no catalog/descriptor/bootstrap/Serde/schema/JSON/Tauri surface. |
| 2 | Replay HCM-3.6 production reachability from SDK through facade to private writer. | Prove strict Clippy, first/replay/refusal, stale/fork, crash, recovery, restart, exact receipts, and no undeclared write without claiming HCM-3.6 or Phase-3 closure. |
| 3 | Add only typed SDK composition for existing semantic-owner behavior. | No future Phase-5 owner, generic `Value`, raw durable record, or semantic duplication. |
| 4 | Cut over Setup, Author, Approvers, Artifact, Pipeline, Generate, Inspect, and Doctor one route at a time. | Preserve exact grammar, output, exit codes, failures, and owner semantics; do not repurpose any route as posture ingress. |
| 5 | Retire compiler normal-path composition. | Do so only after all eight parity walls pass and cargo/dependency/import/source scans prove removal of normal compiler use. |

## Shared typed-shape and proof vocabulary

Every catalogued ordinary method has an exact operation definition at `1.0.0`,
an opaque typed Rust request/result, and a closed outcome union. Packet 1's
direct typed Rust posture method is the deliberately non-catalogued exception:
it is callable only through Rust linkage and creates no operation definition,
descriptor, bootstrap entry, shared Serde/JSON DTO, schema, JSON transport, or
Tauri surface. The SDK is the normal caller. `CLI` means future HCM-4.3
JSON/human CLI adapter; `Tauri` means future HCM-4.4 adapter; neither may
recalculate semantics.

| Code | Request/result/blocker/refusal/error shape | Mutation and required proof |
|---|---|---|
| `R` | Exact repository/profile/capability/definition refs as applicable -> typed catalog, record, report, or derived value. Blocked when an exact prerequisite is unavailable; refused for malformed, stale, mismatched, unsupported, or insufficient-Resolution input; error only for redacted operation integrity. | Read-only/safe/no receipts. Prove direct SDK result equivalence with its owner and CLI/Tauri adapter parity after those slices. |
| `A` | Typed immutable payload plus required idempotency key -> typed appended record/acknowledgment and one receipt. | Append-only/idempotency-key-required. Prove first execution, same-key replay, changed-key conflict/refusal, exact immutable lineage, restart discovery through `record.list/read`, and no raw key disclosure. |
| `C` | Typed intent plus idempotency key and exact expected authority/head/fingerprint bindings -> typed success with declared atomic receipts, or typed stale/precondition/authority/safety refusal. | Compare-and-write/idempotency-key-required. Prove exact precondition, concurrent/stale refusal, same-key replay after basis movement, no undeclared write, recovery/restart, and exact receipt set. |
| `D` | Exact bootstrap/schema/catalog selector and paging cursor -> bounded typed discovery result. | Read-only/safe. Prove restart-safe complete discovery, no inferred default or process-local state, and exact definition/schema fingerprints. |

All rows have the following error/refusal floor: unknown/stale pair, incompatible
capability or version, malformed typed request, unsupported operation, or
insufficient Resolution returns a structured blocked/refused outcome before an
undeclared read/write; an integrity failure is an error without raw secret or
private durable payload. Every `A`/`C` method has a bounded input-only raw
idempotency key and exposes only its scoped fingerprint.

## Exact ordinary-use-case inventory

`Caller` is the future normal route. A `Phase 5` owner row is reserved in this
HCM-4.1 inventory but cannot be implemented before its Phase-5 authority.

| Operation ID | Semantic owner | SDK coordinator / caller | Shape | Required real-path proof |
|---|---|---|---|---|
| `capabilities.describe` | SDK registry over implemented owners | SDK -> CLI/Tauri; bootstrap root | `D` | Every reported operation has exact schema/definition/capability pins; unavailable operations are absent, not inferred. |
| `profile.list` | engine | SDK -> CLI/Tauri | `R,D` | Direct engine and SDK catalog parity; restart/page proof. |
| `profile.resolve` | engine | SDK -> CLI/Tauri | `R` | Exact selected-profile/refusal parity. |
| `schema.list` | engine | SDK -> CLI/Tauri | `R,D` | Exact schema catalog paging and no range/latest fallback. |
| `schema.read` | engine | SDK -> CLI/Tauri | `R` | Exact schema ref/fingerprint and closed document parity. |
| `vocabulary.read` | engine | SDK -> CLI/Tauri | `R` | Exact vocabulary ref/fingerprint and no presentation-owned semantics. |
| `resolution.stack.read` | engine | SDK -> CLI/Tauri | `R` | Exact six-dimension stack selection/refusal parity. |
| `projection.definition.read` | flow over engine definitions | SDK -> CLI/Tauri | `R` | Exact definition/currentness binding; no CLI-owned projection policy. |
| `artifact.kind.list` | engine | SDK -> CLI/Tauri | `R,D` | Descriptor catalog parity; no generated operation/command. |
| `artifact.instance.list` | engine | SDK -> CLI/Tauri | `R,D` | Profile-selected instance catalog parity and paging. |
| `artifact.read` | engine | SDK -> CLI/Tauri | `R` | Exact typed record/provenance and redaction/refusal wall. |
| `artifact.validate` | engine | SDK -> CLI/Tauri | `R` | Owner validation result and typed validation refusal parity. |
| `artifact.render` | engine | SDK -> CLI/Tauri | `R` | Fixed renderer-derived output only; no capitalized Projection substitution. |
| `intake.definition.read` | engine | SDK -> CLI/Tauri | `R` | Exact definition/fingerprint read. |
| `intake.coverage.evaluate` | engine | SDK -> CLI/Tauri | `R` | Deterministic coverage/gap results; no hidden model authority. |
| `intake.record.append` | engine | SDK repository transaction -> CLI/Tauri | `A` | One immutable record, same-key replay, and record rediscovery. |
| `record.list` | engine | SDK -> CLI/Tauri | `R,D` | Closed family/state selector, snapshot-bound paging, and restart-safe discovery. |
| `record.read` | engine | SDK -> CLI/Tauri | `R` | Family/ref/fingerprint match and typed variant-only result. |
| `artifact.candidate.validate` | engine | SDK -> CLI/Tauri | `R` | Exact candidate/result identity and no write before validation. |
| `artifact.candidate.append` | engine | SDK repository transaction -> CLI/Tauri | `A` | Immutable candidate, same-key replay, and unchanged prior records. |
| `artifact.approval.append` | engine | SDK repository transaction -> CLI/Tauri | `A` | Exact candidate binding, authority refusal, and immutable approval replay. |
| `artifact.candidate.promote` | engine | SDK repository transaction -> CLI/Tauri | `C` | Exact candidate/approval/current-target CAS, atomic canonical+promotion receipts, recovery and stale proof. |
| `posture.resolve` | engine private posture owner through later typed bridge | SDK -> CLI/Tauri | `R` | Same exact inputs produce same kernel identity; private records stay private. |
| `posture.recommendation.evaluate` | engine posture owner | SDK -> CLI/Tauri | `R` | Exactly one recommendation-or-no-recommendation result; advisory/no mutation, trigger/window/floor/red-line proof. |
| `posture.recommendation.append` | engine repository transaction | SDK -> CLI/Tauri | `A` | Persisted exact evaluated recommendation, immutable lineage, replay and rediscovery. |
| `posture.recommendation.acknowledge` | engine repository transaction | SDK -> CLI/Tauri | `A` | Separate immutable acknowledgment; no promotion or canonical mutation. |
| `posture.transition.apply` | engine private posture/Charter authority transaction | SDK coordinator -> CLI/Tauri only after HCM-4.3/4.4 | `C` | Real production SDK ingress; full approved-recommendation/policy/approval/head/reassessment/byte-CAS binding; atomic canonical Charter, PostureTransition, and lifecycle-transition 1.1 outputs; no-op/stale/fork/refusal, crash/recovery/restart, and strict-Clippy reachability proof. |
| `projection.create` | flow over engine definitions | SDK -> CLI/Tauri | `R` | Resolution, omission/lossiness/provenance, no hidden raw read, and owner parity. |
| `resolution.escalation.request.append` | engine | SDK repository transaction -> CLI/Tauri | `A` | Exact pending immutable request and restart discovery. |
| `resolution.escalation.disposition.append` | engine | SDK repository transaction -> CLI/Tauri | `A` | Exactly one approved/refused/superseded disposition; no in-place status. |
| `memory.promotion.request.append` | engine | SDK repository transaction -> CLI/Tauri | `A` | Immutable request and no artifact/contract/posture shortcut. |
| `memory.promotion.disposition.append` | engine | SDK repository transaction -> CLI/Tauri | `C` | Applied/refused/stale disposition; applied atomic semantic-memory receipt, no inferred promotion. |
| `snapshot.capture` | engine normalization/repository readers | SDK orchestration -> CLI/Tauri | `A` | Exact capture identity, redaction, consistency, immutable record, and no authority promotion. |
| `snapshot.read` | engine storage boundary | SDK -> CLI/Tauri | `R` | Exact record/ref/fingerprint and disclosure boundary. |
| `snapshot.delta` | engine | SDK -> CLI/Tauri | `R` | Deterministic compatible-delta/refusal and relation-only signal boundary. |
| `snapshot.project` | flow over engine snapshot semantics | SDK -> CLI/Tauri | `R` | Six-dimension Resolution, redaction-before-read, omissions, and provenance. |
| `snapshot.verify_current` | SDK over engine comparison | SDK -> CLI/Tauri | `R` | Exact currentness result/refusal, no implicit capture. |
| `snapshot.resolve_applicable` | SDK over engine policy/profile/Resolution comparison | SDK -> CLI/Tauri | `R,D` | Exact applicable-or-none deterministic selection and restart-safe catalog proof. |
| `repository.setup.plan` | SDK over engine/profile owners | SDK -> CLI/Tauri | `R` | Typed plan and preserved non-authoring/blocked behavior. |
| `repository.setup.apply` | SDK repository transaction | SDK -> CLI/Tauri | `C` | Exact repository basis, idempotent apply/replay, declared operational receipt, no semantic-authority expansion. |
| `repository.doctor` | SDK over owner reports | SDK -> CLI/Tauri | `R` | Same typed readiness/report data and CLI exit mapping from, not instead of, result. |
| `flow.resolve` | flow | SDK -> CLI/Tauri | `R` | Existing resolver compatibility, no posture/grounding fallback or widened request contract. |
| `pipeline.catalog.list` | pipeline | SDK -> CLI/Tauri | `R,D` | Exact pipeline catalog paging. |
| `pipeline.catalog.read` | pipeline | SDK -> CLI/Tauri | `R` | Exact selected pipeline/read refusal. |
| `pipeline.route.resolve` | pipeline | SDK -> CLI/Tauri | `R` | Deterministic route/refusal parity. |
| `pipeline.compile` | pipeline | SDK -> CLI/Tauri | `R` | In-memory only, deterministic provenance, no content-store write. |
| `pipeline.capture.plan` | pipeline | SDK -> CLI/Tauri | `R` | Read-only capture plan/refusal parity. |
| `pipeline.capture.apply` | pipeline | SDK repository transaction -> CLI/Tauri | `C` | Exact capture CAS, immutable evidence receipt, replay/recovery. |
| `pipeline.handoff.emit` | pipeline | SDK repository transaction -> CLI/Tauri | `A` | One append-only handoff record; no global development-control handoff confusion. |
| `pipeline.state.apply` | pipeline | SDK repository transaction -> CLI/Tauri | `C` | Exact state basis, atomic declared receipt, stale/replay proof. |
| `contract.definition.list` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R,D` | Phase-5 exact catalog proof; no pre-Phase-5 implementation claim. |
| `contract.definition.read` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R` | Exact definition/ref/fingerprint proof. |
| `contract.definition.append` | future contracts | SDK transaction -> CLI/Tauri after Phase 5 | `A` | One exact draft record after admission; same-key replay. |
| `contract.lifecycle.transition` | future contracts | SDK transaction -> CLI/Tauri after Phase 5 | `C` | Locked/current lifecycle authority, exact basis, stale/unauthorized refusal. |
| `contract.evidence.list` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R,D` | Exact canonical evidence catalog. |
| `contract.evidence.read` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R` | Exact evidence/provenance/freshness/Resolution read. |
| `contract.evidence.append` | future contracts | SDK transaction -> CLI/Tauri after Phase 5 | `A` | One membrane-validated evidence record; rejected candidate writes none. |
| `contract.verdict.evaluate` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R` | Deterministic complete claim partition; validators/transports stay witnesses. |
| `contract.gate.evaluate` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R` | Hard/required/advisory precedence and no parent-promotion shortcut. |
| `dock.manifest.list` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R,D` | Exact manifest catalog and closure identity. |
| `dock.manifest.read` | future contracts | SDK -> CLI/Tauri after Phase 5 | `R` | Exact bounded manifest/launch closure. |
| `dock.run` | future contracts plus separable executor | SDK -> CLI/Tauri after Phase 5 | `A` | One admitted operational execution record, no direct verdict/gate/canonical-evidence mutation, failure-closed process proof. |

## Detailed posture transition boundary

The planned SDK method has a purpose-named typed request such as
`ApplyPostureTransitionRequest`; names and fields are not selected as public
Rust API until the HCM-4.1 implementation selector. Its required semantic
fields are non-negotiable: repository identity; exact persisted
recommendation/policy pairs; expected source-kernel pair; sorted immutable
approval inputs and authorized actor; exact current heterogeneous
canonical/lifecycle head; exact Charter reassessment inputs; selected fixed
dimension change; expected canonical ref/fingerprint/document SHA-256/length;
and a bounded idempotency key. It returns typed success with the three realized
durable outputs and resulting kernel pair, or the closed blocked/refused/error
outcomes stated in `SPEC.md`.

The later engine-facing bridge owns conversion from this public typed intent to
private `PostureTransition`, lifecycle v1.1, and journal values. The SDK never
returns raw private durable bytes, raw Charter records, Snapshot payloads, or a
transport-owned policy decision. New public engine facade types and their
versioning are a deliberate later HCM-4.1 implementation choice; they are not
created by this plan.

## Artifact compatibility boundary

The existing `artifact` route is a retained CLI compatibility surface, not an
exception to the SDK boundary. Its SDK methods take operation-specific typed
selectors and bounded document inputs and return closed Rust variants. In
particular, the SDK must not expose `serde_json::Value`, a generic JSON
dispatcher, raw lineage records, or internal transaction evidence records.

Where the owner currently returns a JSON value, the SDK projects it into a
recursive closed `ArtifactDocument` value (`null`, boolean, canonical number
lexeme, string, array, or string-keyed object) and operation-specific result
variants. Mutation success projects the existing lineage evidence only as an
opaque typed receipt reference and fingerprint; it does not expose the durable
record or payload. The CLI may render that typed receipt using its established
legacy `internal_transaction_evidence_ref` and
`internal_transaction_evidence_fingerprint` field names so current output
remains compatible. This rendering compatibility does not make either field a
new SDK JSON, Serde, schema, or transport promise.

## Charter compatibility boundary

`author charter` has a separate retained compatibility contract. The CLI keeps
command-form selection, cwd/repository discovery, file and stdin reads, the
CLI-argument/envelope fingerprint merge and mismatch refusal, human rendering,
legacy `--json` rendering, stdout/stderr, and exit mapping. The SDK accepts a
closed already-parsed Charter command request and returns a closed Rust result;
it exposes no `Serialize`, `serde_json`, `serde_yaml`, `Value`, schema, or
transport API. In particular, the approval identity-preflight result remains a
CLI rendering case even though its established legacy JSON shape differs from
the ordinary Charter-result wrapper.

Before the compiler seam is removed, the Author route needs executable
before/after parity fixtures for exact stdout and stderr bytes, exit status,
and filesystem delta. Each fixture states and exercises its human or `--json`
mode (both when the command supports both) while covering legacy-input refusal,
malformed intake, first author and same-input replay, invalid selected Charter,
validate success and missing Charter, approval identity-preflight refusal, and
promotion invalid-intent refusal. The four current direct compiler behavior tests for author
persistence/collision, approval preflight, and promotion refusal move to SDK
typed-behavior tests without `serde_json::to_value`; CLI tests retain the
legacy JSON and human-output assertions. The route-level source scan must show
no compiler reference. Compiler removal remains last: all eight route parity
walls must pass before the normal CLI dependency/import scan and
`cargo tree -p handbook-cli -e normal -i handbook-compiler` prove its absence.

## Compiler retirement and cutover plan

| Current seam | Future destination | Temporary scaffolding | Required deletion proof |
|---|---|---|---|
| Compiler author/Charter product adapters | SDK composes engine owner operations; CLI parses input and renders result | A one-way internal CLI-to-SDK adapter may exist only during cutover. | CLI normal commands no longer depend on `handbook-compiler`; compiler public root does not gain a replacement facade. |
| Compiler doctor/setup/resolver composition | SDK repository/flow composition; CLI keeps cwd, args, output, exit | No compatibility DTO or semantic wrapper in CLI. | Same SDK typed result powers direct Rust and CLI; legacy compiler code is deleted or remains only for separately named non-SDK support seams. |
| Compiler rendering / CLI rendering | CLI owns human and existing `--json` formatting; SDK never owns prose or a serializer | None that evaluates domain state. | Human and JSON rendering are pure legacy CLI adapters over typed SDK results. They preserve existing bytes without creating a shared SDK JSON/Serde/schema or transport promise; new shared JSON parity remains HCM-4.3. |
| Existing direct CLI owner imports | SDK methods | Transitional imports allowed only when documented by a fresh selector. | No normal CLI owner composition bypass remains; cargo graph shows CLI -> SDK, SDK -> owners, no compiler in target path. |

No temporary route may become public API, become a second semantic owner, or
make posture reachable by bypassing the approved SDK operation. Its deletion
test is a cargo-graph/dependency assertion plus a source/import scan and
real-path behavior/parity tests.

## Implementation-time review and stop matrix

| Risk | Required future proof | Stop condition |
|---|---|---|
| New SDK crate/public types | versioning/compatibility choice, package/archive and external consumer proof | Any changed dependency, public compatibility promise, or transport schema choice not selected by fresh authority. |
| Compiler removal | CLI behavior and error/exit regression, no-cycle graph, source import scan | Any retained compiler behavior proves it remains semantic owner or needs a new public facade. |
| HCM-3.6 posture ingress | GitNexus upstream impact; A01-A24 plus approval/head/CAS/atomic-recovery/real-caller proofs; strict Clippy and Phase-3 regression rerun | Missing approved input, need to expose private records, synthetic caller, HIGH/CRITICAL proof gap, or any P1/P2. |
| DTO/transport work | HCM-4.2/4.3/4.4 selectors and exact schema/JSON/Tauri proof gates | Any attempt to treat planning or direct Rust calls as JSON/Tauri proof. |
| HCM-3.5/P5/P6/Phase 5 | Exact selected later owner and gate/publication proof | Any local/pipeline success is misrepresented as parent promotion or published/consumer adoption. |
