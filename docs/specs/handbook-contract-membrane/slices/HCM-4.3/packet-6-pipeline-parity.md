# HCM-4.3 Packet 6 — pipeline read and resolve parity decisions

This compact checkpoint records the slice-delegated Packet-6 decisions selected
from the HCM-4.3 authority, the completed HCM-4.2 pipeline inventory boundary,
and the current Pipeline, SDK, and CLI list/show/resolve/compile paths before
product edits. The slice README remains controlling for scope and later packets.

## Exact ordinary-operation boundary

Exactly three Pipeline-owned read-only operations close in this packet:
`pipeline.catalog.list@1.0.0`, `pipeline.catalog.read@1.0.0`, and
`pipeline.compile@1.0.0`. Their exact request/result roots use the established
`handbook.operation.<operation-id-with-hyphens>-{request,result}@1.0.0`
mapping. Their non-ok problems bind one closed
`handbook.pipeline-operation-details@1.0.0` root; recheckable compile problems
also bind `handbook.pipeline-recheck@1.0.0`.

The semantic request bodies are `{}` for catalog list, `{ selector }` for
catalog read, and `{ pipeline_selector, stage_selector }` for compile.
Repository-root discovery remains a CLI transport fact. Catalog results retain
the complete ordered inventory identity, counts, titles, canonical portable
source paths, stage metadata, defaults, activation, and owning-pipeline facts.
Compile retains its target, persisted route basis, variables, documents and
contents, outputs, gating, and optional stage body. Payload and explain are
renderers over that single result and cannot select different executions.

Owner-native catalog I/O or invalid declarative truth maps to a closed
implementation error by typed `PipelineCatalogError` variant. Read selector
rejections map by typed `PipelineLookupError` variant to resolution refusal.
Compile maps `UnsupportedTarget` to capability refusal and all current
definition/state/basis/input prerequisite classifications to blocked with the
owner recovery action. No status, retry, or exit decision examines rendered
messages.

Before admitting a successful compile projection, the SDK applies the frozen
contract string, collection, and document bounds to the complete owner result.
An over-bound owner result becomes the typed blocked
`compile_result_bound_exceeded` ordinary problem through the Pipeline-owned
bound-refusal constructor; it never degrades into an adapter transport failure.
Catalog, compile, and resolve problem projections replace the unselected
repository root with the portable `<repository>` marker before either renderer.

The three operations have no declared writes, use safe idempotency, return no
receipts, gain independent Pipeline admission, and gain exactly `[cli_json]`
current transport evidence. `pipeline.route.resolve@1.0.0` remains
`semantic_mismatch`, transport-empty, and omitted. All Packet-7 rows and every
unrelated inventory row remain unchanged.

## Resolve-and-persist use-case boundary

The live resolve command continues through the SDK-owned
`handbook.use-case.pipeline-route-resolve-and-persist@1.0.0` composition. It
loads one selected definition and route state, resolves one route, builds one
canonical route basis, and performs the existing revision-checked route-basis
write exactly once. It never constructs or admits the read-only inventory row.

The success result remains
`handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0` and retains
the current pipeline id, route-state and effective-run facts, selected route,
and reasons. The additive exact detail root
`handbook.use-case.pipeline-route-resolve-and-persist-details@1.0.0` is a
closed projection of the typed selection, state-read, variable, route,
route-basis-build, compare-and-write conflict, and persistence failure causes.
It carries only portable facts and any complete owner result needed for human
rendering.

A successful persistence realizes one use-case receipt for the Pipeline-owned
route-basis semantic record. Its condition carries the selected pipeline,
expected route-state revision, and exact persisted route-basis fingerprint;
its atomic group is `pipeline_route_resolve_and_persist`. The owner/SDK
execution result carries effects separately from terminal state so any write
established before a later non-ok composition outcome remains visible. A
revision conflict preserves expected and actual revisions and returns no
fabricated receipt. Resolve has no raw idempotency key and does not claim
ordinary-operation idempotency.

## CLI projection and compatibility

Pipeline list, show, resolve, compile payload, and compile explain each acquire
transport facts, execute once to one typed response, then choose a renderer.
`--json` emits one canonical `InvocationResponseV1` plus LF through the shared
single-write helper. Default human wording, ordering, stdout placement, and
payload-only compile output remain reconstructed from the typed response.
Pipeline had no established command-specific JSON document, so
`--legacy-json` remains unsupported.

Semantic non-ok output remains on stdout without duplicate stderr prose.
Physical output failure is exit 1, causes no retry or second document, and may
emit only the shared bounded operational stderr line.
