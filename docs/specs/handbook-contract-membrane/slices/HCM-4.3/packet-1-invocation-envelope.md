# HCM-4.3 Packet 1 — invocation-envelope decisions

This compact checkpoint records the slice-delegated Packet-1 decisions selected
from the HCM-4.3 authority and current SDK/CLI source before product edits. The
slice README remains controlling for scope, semantics, and later packets.

## Public SDK factoring and identities

The new public family lives in one additive
`contract_membrane::invocation` module. It owns the closed
`InvocationResponseV1`, `UseCaseResponseV1`, and `TransportFailureV1` family;
the CLI owns only selection, framing, physical emission, and rendering.

The checked-in roots use the authority-fixed identities:

- `handbook.invocation-response@1.0.0`;
- `handbook.use-case-response@1.0.0`;
- `handbook.transport-failure@1.0.0`;
- the six authority-fixed `handbook.use-case.*-result@1.0.0` roots; and
- the initial closed detail roots
  `handbook.use-case.repository-doctor-details@1.0.0` and
  `handbook.transport-failure-details@1.0.0`.

The doctor detail root is a tagged union over a complete readiness report and
the two current typed doctor failures. The transport detail root is a tagged
record over the authority's repository, acquisition, decoding, framing,
serialization, emission, and adapter failure kinds. No detail root accepts an
untyped arbitrary success value.

Each use-case success variant owns a distinct closed Rust projection and exact
result-schema binding. The setup and doctor projections share bounded readiness
row types without sharing use-case identity. Charter,
approver-administration, and route-resolve projections mirror only their
current public SDK result fields.

The Project Context success projection is the one required additive exception.
It is a closed tagged union with `author` and `validate` variants because the
current SDK returns mutation data only for authoring while validation returns
unit success. Both variants carry the sanitized CLI-owned acquisition facts
`input_mode` and `input_source` needed to preserve the current human rendering;
the author variant additionally carries the current SDK mutation result. The
CLI supplies those bounded facts to the SDK constructor, but cannot change the
variant, semantic terminal, or mutation result. No renderer side state is
permitted.

Unknown discriminants, binding mismatches, terminal-shape violations,
noncanonical ordering, invalid detail refs, and fingerprint mismatches fail
during construction or typed admission.

## CLI factoring and compatibility

One executable 36-row coverage table is the classification source for every
authority-listed leaf/mode. `--json` is parsed uniformly in the currently valid
leaf-local positions. Existing command-specific JSON moves behind mutually
exclusive `--legacy-json`; only doctor is routed through the new invocation
contract in this packet. Other families retain their current semantic path
until their assigned packets and do not gain discovery evidence here.

One CLI output-mode type, one typed-terminal exit projection, and one prepared
single-write emission helper are shared. The helper serializes before writing,
appends one LF, performs no retry after a rejected or partial write, and makes a
physical output failure override the semantic exit to 1. Clap keeps ownership
of help/version exit 0 and pre-dispatch usage exit 2.

Doctor executes once to a typed SDK result, then derives human, invocation
JSON, or byte-compatible legacy JSON. `Ready` maps to `ok`; every other current
readiness status maps to `blocked` with the authority-fixed prerequisite code.
Doctor remains a use case, transport-empty, and absent from ordinary operation
admission and discovery.
