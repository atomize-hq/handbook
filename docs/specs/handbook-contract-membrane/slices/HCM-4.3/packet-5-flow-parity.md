# HCM-4.3 Packet 5 — Flow parity decisions

This compact checkpoint records the slice-delegated Packet-5 decisions selected
from the HCM-4.3 authority, the completed HCM-4.2 Flow admission boundary, and
the current Flow, SDK, and CLI generate/inspect paths before product edits. The
slice README remains controlling for scope and later packets.

## Exact ordinary-operation boundary

Both `generate` and `inspect` map to the existing exact inventory row
`flow.resolve@1.0.0`. The row remains an ordinary Flow-owned, read-only, safe
operation. It does not become an SDK use case, split into presentation-specific
operations, or create another inventory row.

The semantic request body is the exact closed object `{ packet_id }`, where
`packet_id` is exactly one of `planning.packet`, `execution.demo.packet`, or
`execution.live.packet`. Unknown packet IDs fail request-schema admission before
Flow executes. Repository
discovery, fixture-root acquisition, and fixture-set decoding remain CLI
transport concerns; selected fixture identity and lineage are returned as owner
result truth. The exact request and result roots are
`handbook.operation.flow-resolve-request@1.0.0` and
`handbook.operation.flow-resolve-result@1.0.0`. Non-ok operation problems bind
to `handbook.flow-operation-details@1.0.0`; blocked owner outcomes additionally
bind `handbook.flow-recheck@1.0.0`.

The SDK admits the exact definition and request, independently obtains Flow
owner admission, calls the existing resolver exactly once, and exhaustively
projects the owner-native result and error causes. It does not reimplement
packet selection, fixture policy, blocker/refusal choice, budget behavior, or
message-based classification.

The Flow admission file remains the stable owner-source evidence preimage. Its
dispatch hook selects the exact `flow.resolve` claim from a separate Flow-owned
registry module, following the established Engine pattern; generated claim
fingerprints never live inside the source bytes they fingerprint.

## Result and terminal closure

The exact result preserves the complete current public Flow projection:

- contract/schema versions and canonical C03 fingerprint;
- packet identity, status, variant, ordered sources, omissions, body notes,
  sections, and fixture context/lineage;
- ordered decision-log facts and the current next safe action;
- the full budget disposition, reason, byte domain, per-target actual values,
  and budget next action;
- the selected packet identity and status;
- any owner refusal category, subject, detail, and next actions; and
- every owner blocker category, subject, detail, and next actions.

The projection exposes only portable owner facts. Host repository roots, raw
private fixture data, private Flow state, and raw keys do not cross the
membrane. The operation declares no writes, uses not-applicable idempotency, and
returns no receipts because the current Flow resolver realizes no public write.

`ready` is `ok`. Non-ready results map by the closed owner cause table below;
container presence and rendered text never select the terminal. When Flow
currently carries both a legacy refusal summary and blocker evidence for the
same prerequisite, the typed terminal follows this table while the complete
nested result preserves both facts for established human rendering.

| Owner cause | Terminal / problem / retry |
|---|---|
| system root missing, not a directory, or symlink; required artifact missing, empty, starter-owned, invalid, or unreadable; freshness invalid; budget refused | `blocked` / `prerequisite` / `after_recheck`, with exact typed recheck cause and the owner next safe actions retained in the result |
| non-canonical input attempt | `refused` / `safety` / `never` |
| unsupported request | `refused` / `capability` / `never` |
| pre-result owner ingest reports system-root or required-artifact absence/shape | `blocked` / `prerequisite` / `after_recheck`, selected from the closed ingest variant |
| owner ingest read failure | `error` / `implementation` / `transient`, with host path removed |
| owner manifest profile-decision failure | `error` / `implementation` / `never`, with portable sanitized detail |

The full resolved owner result is success data for `ok` and typed problem
details for `refused` or `blocked`, so non-ok rendering does not discard
composition, fixture, blocker, diagnostic, action, or budget truth. Owner errors
use their closed cause and sanitized portable detail. No terminal, retry,
receipt, or exit decision depends on rendered text.

## CLI projections and transport

Each command performs repository/fixture acquisition, constructs one exact
ordinary request, and receives one `OperationResponseV1` before choosing a
renderer. The established generate Markdown and inspect inspection text are
reconstructed from that same typed response, preserving their current wording,
ordering, streams, and exit behavior. Renderer selection cannot rerun Flow.

`--json` wraps the ordinary response in one `InvocationResponseV1` and emits
exactly one schema-valid document plus LF through the shared single-write
emitter. Semantic non-ok output remains on stdout without semantic stderr prose;
physical stdout failure is exit 1 with no retry or second document. Transport
acquisition, decoding, adapter, serialization, framing, and emission failures
remain distinct typed stages. Flow has no established command-specific JSON
mode, so `--legacy-json` remains unsupported and keeps its existing Clap exit.

After complete proof, exactly `flow.resolve@1.0.0` becomes admitted,
discoverable, and current with the transport vector `[cli_json]`. The exact
62-row inventory, operation identity, posture Rust-SDK evidence, earlier CLI
closures, and every unrelated row remain unchanged.
