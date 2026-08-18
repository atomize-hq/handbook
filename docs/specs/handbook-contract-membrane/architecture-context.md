# Handbook Contract Membrane Architecture Context

This document owns the stable semantic-ownership, dependency-direction, adapter,
trust, and external-proof boundaries for the Handbook contract membrane. It
explains the enduring architecture that current and future work must preserve.

It is not an implementation inventory, roadmap, ADR, slice authority, packet
specification, or permission to change the product. The
[`implemented baseline`](implemented-baseline.md) owns detailed current-state
classification. Source and tests remain product truth.

## Reading the topology

The labels below describe architecture state, not lifecycle or authorization:

- **Current** means the named ownership or dependency boundary exists in tracked
  source.
- **Precursor** means useful components exist, but an ordinary consumer or
  complete ownership boundary does not.
- **Target/future** names a durable direction or missing owner without claiming
  that it is implemented, scheduled, or authorized.
- **External-unknown** means repository state cannot establish the external
  condition.

| Surface | State | Architecture boundary |
| --- | --- | --- |
| Artifact, profile, registry, Context Resolution, and posture domains | Current | [`handbook-engine`](../../../crates/engine/) owns their semantic rules, authority checks, and durable mutations. |
| Flow and pipeline domains | Current | [`handbook-flow`](../../../crates/flow/) and [`handbook-pipeline`](../../../crates/pipeline/) own their domain behavior while depending on the engine for shared semantic truth. |
| Grounding adoption and deterministic projection/snapshot components | Precursor | Typed Flow and pipeline adapters exist, while projection and snapshot components remain private and no ordinary grounding consumer completes the seam. |
| SDK composition | Current | [`handbook-sdk`](../../../crates/sdk/) composes owner crates into bounded, transport-free public Rust surfaces. |
| Contract-membrane types, schemas, admission, and discovery | Current | HCM-4.2 provides the bounded kernel and exact 62-row inventory; HCM-4.3 and HCM-4.4 extend the deterministic tree to 120 roots and the immutable production witness to 19 capabilities. The 18 predecessor operations expose `cli_json` and `tauri`, posture exposes `rust_sdk` and `tauri`, and the six fixed use cases remain outside ordinary admission and discovery. |
| CLI adapter | Current | [`handbook-cli`](../../../crates/cli/) consumes owner/SDK typed results for all 36 supported nontrivial leaves and exposes exactly four generic capability actions over the SDK consumer seam. It owns only command-line acquisition/rendering/stream/exit concerns, emits the shared invocation envelope for `--json`, and preserves only the 19 established explicit legacy projections. |
| Tauri adapter | Current | [`handbook-tauri`](../../../crates/tauri/) is a non-UI library with three versioned commands over SDK-owned bootstrap, catalog, and invocation DTOs. It owns host context, off-main-thread scheduling, bounded host failures, and IPC framing without CLI or owner-crate product dependencies. |
| Installed Handbook skill | Current | The generated root skill uses only the generic CLI membrane to bootstrap, page discovery, retrieve exact schemas, acquire bounded input, and invoke. It carries no current operation inventory or product-specific ordinary route; the fixed Charter workflow remains a separate leaf skill. |
| Contract/evidence/gate product owner and dock executor | Target/future | These owner and executor surfaces are absent; this topology does not authorize or design them. |
| Publication, bridge, and Substrate adoption | External-unknown | No repository-only claim can prove their current external existence, deployment, or conformance. |

The workspace membership and current crate edges are visible in
[`Cargo.toml`](../../../Cargo.toml) and the crate manifests. The table is a
boundary summary, not a substitute for those executable declarations.

## Semantic ownership and dependency direction

The engine, Flow, and pipeline domains retain authority for their own semantic
decisions. That includes domain invariants, operation outcomes, refusal branches,
authority resolution, and mutation rules. A public projection may describe an
owner result, but it does not transfer ownership of the decision that produced
the result.

The SDK composes those owners for ordinary typed use. It may bound and project
their public inputs and outputs, coordinate repository-bound calls, and expose a
stable Rust-facing surface. It must not reimplement semantic branches or become
an alternate authority source. Otherwise direct owner calls and SDK calls can
produce different truth for the same operation.

CLI and Tauri transport adapters consume SDK-facing surfaces. The installed
root skill consumes only the generic CLI membrane and does not depend on the
SDK or semantic owners directly. These consumers must not add
back-edges from semantic owners into presentation or transport crates. This keeps
domain behavior usable without a specific interface and prevents transport
release concerns from controlling semantic evolution.

The current dependency direction is therefore:

```text
engine <- flow / pipeline <- sdk <- cli <- installed skill
                           <- tauri
```

This is a semantic direction, not a requirement that every future type remain in
its current crate forever. Any deliberately neutral shared layer must be
authorized as such and must preserve owner independence.

### HCM-4.2 placement

The completed HCM-4.2 kernel, envelope, bootstrap, posture projection, and
schema-generation boundaries currently live under
[`handbook_sdk::contract_membrane`](../../../crates/sdk/src/contract_membrane/mod.rs)
with checked-in assets under
[`crates/sdk/schemas`](../../../crates/sdk/schemas/). That is current placement,
not authority for a future contract product owner to depend back on the SDK.

When a future contract owner is authorized, its authority must choose either
outward mapping into SDK types or an explicitly authorized neutral extraction.
This document chooses neither. Preserving that decision prevents a foundational
contract owner from acquiring a circular or presentation-facing dependency.

## Projection and adapter boundary

Adapters may own concerns that exist because of the interface:

- parsing and lexical validation of transport input;
- request framing and response rendering;
- correlation and transport-local metadata;
- presentation, streams, and exit status; and
- transport-specific failures that occur before or after an owner call.

Adapters do not own:

- operation identity or the authoritative operation inventory;
- canonical DTO shape or semantic field meaning;
- owner admission, policy, or mutation decisions; or
- semantic success, omission, refusal, or blocked branches.

Those restrictions prevent a CLI, Tauri surface, or other adapter from becoming
a second product implementation. An adapter may map an owner result into its
medium, but it must preserve the closed owner outcome rather than infer a new
one.

Projection has the same rule. A projection selects and bounds owner truth for a
consumer; it does not synthesize missing authority. The private
[`projection`](../../../crates/engine/src/projection/) and
[`snapshot_memory`](../../../crates/engine/src/snapshot_memory/) components and
the typed grounding adapters are precursors until an authorized ordinary seam
consumes them. Their existence alone does not prove adoption.

## Trust and validation boundaries

Structural admission and semantic trust are different steps. Packet 0 can prove
that a value is bounded, canonical, internally consistent, and exactly bound;
its sealed witness path then permits an owner to establish trusted
`Verified<T>`. Structural validity alone must not be presented as an owner
decision. This distinction prevents well-formed but unauthorized or stale data
from crossing a trust boundary.

The posture seam demonstrates the intended owner relationship. The SDK accepts
both the HCM-4.1 direct typed request and HCM-4.2's exact generic operation
request, then calls the same engine facade. The Engine independently admits the
operation definition and resolves repository authority, policy, approvals,
lifecycle state, persistence, and replay rather than trusting caller-supplied
private records. This prevents a public caller from manufacturing the evidence
that authorizes its own mutation.

Three byte and identity domains must remain separate:

1. **Instance canonicalization and fingerprints** identify concrete values and
   bindings used by product operations.
2. **Checked-in schema-byte fingerprints** identify exact declarative schema
   assets when such an asset authority exists.
3. **Adapter framing** identifies transport messages and correlation, not
   semantic instances or schema authority.

Conflating these domains can make a transport encoding appear to authorize an
instance, or make a schema artifact appear to prove an owner result. Mapping
between domains must be explicit and must preserve each domain's exact binding.

The current engine uses Rust `jsonschema` for existing schema responsibilities.
Historical bundled-Python validator choices are not current architecture.
Future contract runtime or tooling remains a later authority decision; this
document does not select it.

## Gate and posture ownership

A future gate result may be an input to posture evaluation. Gate ownership must
not absorb posture aggregation, policy, recommendation, acknowledgement,
hysteresis, or transition authority. Gates answer their bounded evidence
question; posture owners decide what those answers mean together.

This separation prevents an evidence producer from silently becoming the policy
engine and lets posture policy evolve without changing each gate implementation.
It also keeps mutation authority behind the existing engine-controlled trust
boundary.

## External proof boundary

Repository source can establish local types, implementations, fixtures, tests,
and dependency edges. It cannot, by itself, establish that a package was
published, an external bridge is operating, or Substrate currently consumes a
surface. Such claims remain external-unknown until current evidence from the
external owner or system is explicitly admitted.

Absence of a local adapter is evidence of local absence, not proof of external
nonexistence. Conversely, historical publication or adoption evidence does not
prove current external state. This boundary prevents stale provenance from being
promoted into present operational truth.

## Drift and use

Use this context when defining future slice authority, reviewing dependency
direction, or deciding where a projection or adapter belongs. Do not infer a
schedule, completion claim, or work authorization from it.

If current source changes a stated ownership or dependency boundary, update this
document in the same bounded change or record the discrepancy as documentation
drift. If a proposed change needs to reverse one of these boundaries, authorize
and record that decision separately rather than silently editing the topology.
