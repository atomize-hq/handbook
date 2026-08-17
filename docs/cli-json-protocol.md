# Handbook CLI JSON protocol

`handbook --json` is the stable machine-output selector for every supported
nontrivial CLI invocation. It emits the shared
`handbook.invocation-response@1.0.0` protocol rather than a command-specific
JSON shape.

Help, version, parent help, and command-line syntax handling remain shell
control behavior. They retain Clap's text, stream, and exit behavior and do not
emit Handbook result documents.

## Invocation envelope

On a writable stdout transport, a recognized nontrivial invocation with
`--json` writes exactly one UTF-8 JSON document followed by one LF. The outer
object has this shape:

```json
{
  "schema": {
    "ref": "handbook.invocation-response@1.0.0",
    "fingerprint": "sha256:..."
  },
  "response_kind": "operation",
  "response": {},
  "response_fingerprint": "sha256:..."
}
```

`response_kind` is closed to three values:

| Kind | Meaning | Typed identity in `response` |
| --- | --- | --- |
| `operation` | One independently admitted ordinary operation | Exact operation and request/result schema bindings |
| `use_case` | One SDK composition over owner behavior | One of the six fixed `handbook.use-case.*@1.0.0` bindings |
| `transport_failure` | Input acquisition, repository discovery, serialization, or another adapter failure | Typed failure kind, problem details, diagnostics, and next actions |

The kind-specific body carries the semantic terminal status: `ok`, `blocked`,
`refused`, or `error`. Operation and use-case bodies also carry their applicable
receipts, diagnostics, and next actions. Consumers should select on
`response_kind`, validate against the checked-in Draft 2020-12 root, and then
read the corresponding typed body. Do not infer success from the presence of a
JSON document alone.

The schemas and exact-byte manifest are under
[`crates/sdk/schemas`](../crates/sdk/schemas/). The outer root is
[`handbook.invocation-response@1.0.0`](../crates/sdk/schemas/2020-12/handbook.invocation-response/1.0.0.schema.json).

## Streams and framing

- Stdout contains the selected result projection only. New JSON and legacy
  JSON each use exactly one document plus one LF on writable stdout.
- Semantic blockers, refusals, errors, diagnostics, and safe next actions stay
  in the typed result and its selected renderer. They are not duplicated as
  ad hoc stderr prose.
- Stderr is reserved for operational progress or a bounded physical-output
  failure message. It must not contain raw keys, private owner state, absolute
  host paths, or debug dumps.
- Serialization completes before output begins. If the operating system
  rejects a stdout write, including a broken pipe or partial write, Handbook
  exits 1, does not retry, and does not emit a second document. A partial
  physical write may therefore leave incomplete bytes at the consumer; discard
  them when the process exits nonzero.

Raw idempotency keys, private journals and staging records, absolute host
paths, and unsafe fallback payloads are not part of any public projection.

## Exit codes

| Exit | Meaning |
| --- | --- |
| `0` | Typed terminal `ok`, or shell-control help/version behavior |
| `1` | Typed `blocked`, `refused`, or `error`; transport failure; or physical stdout failure |
| `2` | Clap command-line syntax or selector conflict |

Human, `--json`, and an available `--legacy-json` projection derive this exit
from the same typed terminal. Physical stdout failure is the only transport
override. Renderers do not own a second semantic exit table.

## Human output and the compile exception

Without a JSON selector, each command retains its established human format,
but that format is rendered from the same typed result as the machine envelope.
For `pipeline compile` without `--explain`, the established human projection is
the payload bytes alone. `pipeline compile --json` always emits the invocation
envelope, with the compiled payload represented inside the typed operation
result; it never prefixes or follows the JSON with raw payload bytes.

## Migrating from legacy JSON

`--legacy-json` is an advisory compatibility projection for commands that had a
JSON format before the shared protocol. It is mutually exclusive with
`--json`. No removal deadline is established, but new integrations should use
`--json`; the legacy shapes are command-specific and are not invocation
envelopes.

Legacy JSON remains available only for these established surfaces:

| Family | Supported legacy leaves |
| --- | --- |
| Author Charter | author, approve, promote, validate |
| Approvers | bootstrap, add credential, revoke credential, update mapping |
| Artifact | list kinds, list instances, read, validate, intake-definition read, intake evaluate, intake append, candidate validate, candidate append, promote |
| Doctor | doctor |

Setup, Project Context, Flow (`generate` and `inspect`), and pipeline commands
never had an established legacy JSON protocol, so `--legacy-json` is not added
to them.

For an existing integration:

1. Run the same invocation with `--json` instead of `--legacy-json`.
2. Select the body by `response_kind`.
3. Replace command-specific top-level status or operation fields with the
   typed identity and terminal fields inside `response`.
4. Read receipts, diagnostics, and next actions from the typed body rather
   than reconstructing them from prose.
5. Preserve the process exit check. Writable human, new-JSON, and legacy
   projections agree on terminal and exit.
6. Validate the complete output against
   `handbook.invocation-response@1.0.0` and require exactly one trailing LF.

Legacy byte and field behavior remains available behind the explicit selector;
no legacy mode is synthesized for a command that did not previously have one.

## Ordinary operations and use cases

Ordinary discovery reports only independently owner-admitted capabilities. The
current CLI JSON ordinary set is the frozen artifact/intake, Flow, and pipeline
set: 18 operation identities with `cli_json` transport evidence. Together with
the pre-existing Rust-SDK-only posture transition, the production witness has
19 discoverable capabilities over the unchanged 62-row inventory.

The following six CLI compositions are use cases, not ordinary operations, and
therefore never appear in ordinary admission or discovery:

- `handbook.use-case.repository-setup@1.0.0`
- `handbook.use-case.repository-doctor@1.0.0`
- `handbook.use-case.author-charter@1.0.0`
- `handbook.use-case.author-project-context@1.0.0`
- `handbook.use-case.approver-administration@1.0.0`
- `handbook.use-case.pipeline-route-resolve-and-persist@1.0.0`

In particular, `pipeline.route.resolve@1.0.0` remains a semantic mismatch,
transport-empty, and omitted from ordinary discovery because the live CLI
action persists route basis. The CLI projects that behavior through the fixed
route-resolve-and-persist use case instead.
