# HCM-5.4 First Real Validator Dock Proof

This document owns the outcome and packet authority for HCM-5.4. It defines
exactly one slice outcome: install the already-selected first-party native Rust
JSON Schema validator as the first production Dock manifest/bundle, validate
one exact inline JSON instance against one exact offline Draft 2020-12 local
schema/ref closure, and prove the completed candidate through Contracts-owned
canonical evidence, verdict, gate, and local closeout.

It does not authorize another operation, capability, process-protocol version,
semantic owner, arbitrary third-party bundle, stronger host sandbox, product
publication, external adoption, parent promotion, posture mutation, or Phase 6.

## Current admission state

**Packets 1–6 are implemented. Packet 4's invalid package checkpoint remains
superseded by Packet 4R, Packet 5 proves the exact contract-owned semantic
path, and Packet 6 finalizes the generic consumers and 170-root public truth.
HCM-5.4 remains active pending aggregate closeout, is not slice-closed, and
Phase 5 is not exited.**

The completed checkpoints are Packet 1
`a66862b267f5e6e72afaed39991bef5ff3d9e743`, Packet 2
`9a620e064fb94e707c69349c586c47704de4e692`, and Packet 3
`5097548ed4b1d5cb2f6e047228e69526de5a2001`. Packet 4 commit
`583785aab141571e6231cf19016fe978cf2e1a65` added the one-entry production
registry and ordinary run path, but its frozen same-target executable could not
be reproduced from an isolated clean build. Packet 4R commit
`00d82346db20418d8c51887023d8e825856cf5bc` supersedes that invalid package
identity, and Packet 5 commit `cc45b11d67e2d3af8b192f64e346a55cce448369`
proves the exact positive, negative, replay, currentness, Resolution, and
non-authority paths through the re-frozen production validator.

The hard predecessor is satisfied. HCM-5.3 is slice-closed at commit
`5f3787706e383714c70978229e1c14dd767b26b6`. Its exact handoff fixes:

- validator identity `handbook.dock.json-schema@1.0.0`;
- `handbook.dock.process-json@1.0.0`, the exact manifest/bundle/request/result/
  execution/candidate bindings, six host outcomes, completed-only rule, and
  host-enforced ceilings;
- `handbook-dock` as operational host/executor and `handbook-contracts` as the
  only canonical evidence/verdict/gate/local-closeout owner;
- native Rust plus the pinned `jsonschema` dependency and current exact offline
  Draft 2020-12 local-closure practices; and
- current product truth of exactly 62 operations, 31 capabilities, 168 schema
  roots, and an empty production manifest registry.

Product execution is admissible only from this amended authority in strict
packet order. Current source truth is exactly 62 operation rows, 31
capabilities, 170 checked-in schema roots, one embedded production manifest,
and six fixed use cases. The deterministic production generator, exact-byte
manifest, schema catalog, Rust SDK, generic CLI, and generic Tauri facade now
include the two selected HCM-5.4 roots without a new route or semantic branch.
Packet completion does not establish aggregate slice closeout or Phase-5 exit.

## Outcome, owner, and dependency direction

Add one purpose-named workspace crate at `crates/dock-json-schema`:

- Cargo package and library name: `handbook-dock-json-schema`, version `0.1.0`;
- native binary name: `handbook-dock-json-schema`;
- manifest/dock identity: `handbook.dock.json-schema@1.0.0`; and
- implementation-bundle identity:
  `handbook.dock.bundle.json-schema-native@1.0.0`, whose exact binding also
  closes over the current target tuple, entrypoint bytes, inventory, and fixed
  argument vector.

The crate owns only the closed validator input/fact types, pure local-closure
admission and schema-conformance evaluation, normalized validator observations,
private wire adapters, and one-shot native process entrypoint. It is not a
semantic owner. The selected dependency direction and process seam are:

```text
handbook-engine generic authority / Resolution / currentness facts
    <- handbook-contracts contract / evidence / verdict / gate owner
        <- handbook-dock manifest / host / operational record / candidate source
            <- handbook-sdk outward DTOs and existing ordinary operations
                <- unchanged generic CLI and Tauri capability consumers

handbook-dock host
    -- exact handbook.dock.process-json@1.0.0 bytes -->
        handbook-dock-json-schema process
```

Arrows point from consumer to dependency. The process arrow is interoperability,
not a Rust crate dependency: the validator must not depend on `handbook-dock`,
`handbook-contracts`, `handbook-engine`, `handbook-sdk`, or another product
owner. Its private Serde wire adapters consume and emit the exact language-
neutral protocol JSON, and cross-crate canonical vectors plus host schema/
fingerprint admission prevent drift. Its normal dependencies are limited to the
pinned offline JSON Schema implementation and narrow serialization/
canonical-hash support. It cannot construct canonical evidence, applicability,
verdict, gate, local-closeout, lifecycle, receipt, promotion, or posture truth.
Dock continues to own only the manifest, bundle admission, execution,
operational record, and completed untrusted candidate. Contracts independently
resolves and admits every candidate.

No Engine, Contracts, Flow, pipeline, SDK, CLI, Tauri, or installed-skill owner
may depend on the validator. No adapter-specific route is added.

## Current and adopted evidence

Current source and tests remain product truth. This authority is grounded in:

- the current program [`README`](../../README.md),
  [`phase-and-slice map`](../../phase-and-slice-map.md),
  [`implemented baseline`](../../implemented-baseline.md),
  [`architecture context`](../../architecture-context.md), and
  [`seam crosswalk`](../../seam-crosswalk.md);
- the completed [`HCM-5.1`](../HCM-5.1/README.md),
  [`HCM-5.2`](../HCM-5.2/README.md), and
  [`HCM-5.3`](../HCM-5.3/README.md) authorities and their current owner tests;
- the exact Dock kernel/executor/store/registry source under `crates/dock/` and
  its completed Packet-1–5 tests;
- Contracts evaluation-run preparation, candidate-source re-admission,
  immutable evidence, verdict, gate, and local-closeout source under
  `crates/contracts/`;
- the current exact operation/schema/production-witness and generic-consumer
  paths under `crates/sdk/`, `crates/cli/`, and `crates/tauri/`; and
- the pinned `jsonschema = 0.47.0` dependency and exact Draft 2020-12 local
  closure practices in `crates/engine/src/schema_registry.rs` and
  `crates/sdk/tests/contract_schema_generation.rs`.

No additional historical control-plane document is adopted. HCM-5.3's bounded
adoption of the selected validator identity and language-neutral framing is
inherited only through its current handoff. Historical Python code, runtime
choices, manifest catalogs, schemas, packet counts, and completion claims remain
non-authoritative.

## Exact product deltas and inventory treatment

HCM-5.4 changes only these admitted product facts:

| Surface | Current HCM-5.3 truth | Exact HCM-5.4 truth |
| --- | --- | --- |
| Operation inventory | 62 rows | 62 rows; no row added, removed, renamed, reclassified, or re-owned |
| Discoverable capabilities | 31 | 31; existing Dock and Contracts capabilities are reused |
| Independently retrievable schema roots | 168 | 170; exactly the two roots below are added |
| Production Dock manifests | 0 | 1; exactly `handbook.dock.json-schema@1.0.0` for the packaged current target |
| Fixed SDK use cases | 6 | 6; unchanged and still outside ordinary discovery |

The exact new roots are:

1. `handbook.dock-json-schema-validation-input@1.0.0`; and
2. `handbook.fact.schema-conformance@1.0.0`.

The first root is required because the implemented Dock protocol admits every
inline role against one manifest-fixed schema binding; an arbitrary contract
schema cannot safely serve as that envelope. The second is the validator-
specific candidate fact selected by the HCM-5.3 handoff and current test
precedent. Nested closure members, normalized violations, paths, and
fingerprint records remain `$defs` inside these two roots. A third new root, a
new ordinary operation, or a 32nd capability requires authority remediation.

Both roots extend the current deterministic Draft 2020-12 generator, checked-in
canonical schema tree, exact-byte manifest, schema catalog, and production
witness. All 168 predecessor roots, 31 predecessor capabilities, 62 inventory
rows, six fixed use cases, and existing transport evidence remain protected.

## Production manifest, native bundle, and launch identity

The first production manifest has these exact semantic values:

| Field | Selected value |
| --- | --- |
| `dock_ref` | `handbook.dock.json-schema@1.0.0` |
| Protocol | exact current `handbook.dock.process-json@1.0.0` binding |
| Request/result schemas | exact current `handbook.dock-process-request@1.0.0` and `handbook.dock-process-result@1.0.0` bindings |
| Implementation | `native_process` |
| Bundle ref | `handbook.dock.bundle.json-schema-native@1.0.0` |
| Entrypoint | `bin/handbook-dock-json-schema` plus the Cargo target's native executable suffix when non-empty |
| Fixed arguments | empty ordered vector |
| Target | exact `host_target_tuple_v1()` value of the packaged host |
| Evidence kinds | exactly `[handbook.evidence.schema-conformance]` |
| Input roles | exactly one `handbook.input.json-schema-validation`, media type `application/json`, bound to the validation-input root |
| Resolution ceiling | shipped-root stack plus `local-observation / identifier-only / current-operation / read-only / operation / observation-only` |
| Declarations | `deterministic: true`, `offline: true`, `read_only: true` |

The target package contains exactly one regular **bundle** member: the native
validator executable. A canonical `manifest.json` sits beside, not inside, the
`bundle/` directory. The package preparation order is fixed: build the
validator; derive the bundle inventory, entrypoint length/digest, empty argument
vector/fingerprint, target, bundle fingerprint, manifest schema/protocol
bindings, and manifest fingerprint through the existing RFC-8785/SHA-256
constructors; freeze those canonical manifest bytes into the same target's host
build; then install those identical bytes and the identical validator executable
in the layout below. Generated bindings contain no placeholder, wildcard,
range, `latest`, or unverified value.

The portable installed layout is fixed relative to the host executable:

```text
<host-executable-directory>/
  handbook-docks/
    handbook.dock.json-schema/
      1.0.0/
        <exact-target-tuple>/
          manifest.json
          bundle/
            bin/
              handbook-dock-json-schema<target-executable-suffix>
```

The production registry is defined by the host-embedded exact manifest, not by
rediscovering or deriving trust from installed files. It therefore lists and
reads exactly one definition even when package material is unavailable. The
owner resolves only that exact current-target location from the host's own
executable location and requires installed `manifest.json` to equal the
embedded canonical bytes before rechecking its one-file bundle. There is no
environment override, PATH search, current-directory search, repository scan,
home-directory lookup, registry scan, package manager, shell, or caller-supplied
bundle root. Production list/read expose the manifest identity but never the
local mapping. A missing, extra, symlinked, non-regular, wrong-target, changed,
or unreadable manifest/member makes the package unavailable or launch-refused
before spawn; it never changes registered truth.

The manifest is target-bound because the existing V1 manifest has exactly one
target and one native bundle. Each supported Cargo target packages and freezes
its own exact manifest fingerprint from its own executable bytes while keeping
the selected dock ref/version. A second build for the same target must reproduce
the frozen package bytes or stop for a version/authority decision; runtime
rederivation may not silently bless changed bytes behind the exact ref. Tests
compare target-local exact bindings and may not normalize one target's digest
into another target's truth.

### Deterministic current-target package rule and Packet 4 supersession

The Packet 4 failure is localized to the current MSVC final link. Two isolated
clean builds of identical source without a reproducibility option had identical
length and all executable bytes except the PE/COFF timestamp, its three debug-
directory timestamp copies, and the RSDS PDB GUID. MSVC `link.exe` stamped wall-
clock time and a fresh PDB identity into each executable. Staging and RFC-8785
manifest derivation were deterministic over a given executable and did not
cause the mismatch.

Packet 4 also froze the 9,545,216-byte `CARGO_BIN_EXE_*` artifact produced for
its integration test. That path is expressly a test-only staging seam and is
compiled under Cargo's test graph/profile with dev dependencies enabled; it is
not an exact normal-dependency production package profile.
This did not create the varying timestamps/GUIDs, but it left the production
compile input ambiguous and explains why a normal clean package build has a
different length before byte comparison. Packet 4R must separate the production
package producer from `CARGO_BIN_EXE_*`; direct native-process tests may retain
that variable only for their existing test-only staging.

For `x86_64-pc-windows-msvc`, the repository-controlled package producer must
pass exact effective Rust linker argument `-C link-arg=/Brepro` to the final
`handbook-dock-json-schema` binary link. The clean package build is the exact
locked/offline package-and-binary selection represented by:

```text
cargo rustc --offline --locked -p handbook-dock-json-schema \
  --bin handbook-dock-json-schema -- -C link-arg=/Brepro
```

The selected profile is the current workspace `dev` profile: unoptimized with
`debug = 1`, with no `--release` or alternate `--profile` selection. Each proof
invocation uses a fresh isolated `CARGO_TARGET_DIR`, the current source and
`Cargo.lock`, no additional `RUSTFLAGS`, and the target reported by the crate
build as `x86_64-pc-windows-msvc`. The build-time mechanism must be repository-
owned and scoped to this native package; an operator's ambient shell flag is not
package authority. A different Cargo profile, target, linker option, source
byte, lockfile, or toolchain is a different build input.

The locally proven toolchain was `rustc 1.89.0` / `cargo 1.89.0`, LLVM 20.1.7,
and MSVC `link.exe` 14.43.34810.0. With `/Brepro`, two clean isolated builds
produced identical 9,548,800-byte executables with diagnostic fingerprint
`sha256:067d9d4386382ed1fc97f312336fc8a148bb135269679d4bb46c5058fe18d411`.
The existing staging implementation then produced identical 2,594-byte
canonical manifests with byte fingerprint
`sha256:655bfc9badf8105dfa6993513fc56619254dad22be920915c4adc49c45c47279`,
bundle fingerprint
`sha256:3f01d34d15a7c3bd7a1887c3dfd4e16f1aa317ea4e50046726026aafa4b1d74f`,
and manifest fingerprint
`sha256:85ca3e4482b2316c654ee117f1ee2e496647734a8e187e36ad5bcae09d6cd893`.
These values are causal experiment evidence, not authority to copy a
placeholder or skip derivation from Packet 4R's final source.

Packet 4R's repeated-build proof selects and freezes one final executable and
manifest byte identity. Every later rebuild of that final same-source/current-
target package must equal the frozen bytes, not merely equal another rebuild.
A reproducible but different result after Packet 4R therefore stops for a new
version/authority decision. No PE normalization, timestamp/GUID exclusion,
selective hashing, signature stripping, post-link rewrite, or fuzzy comparison
may establish equality; comparison covers the complete executable and complete
canonical manifest byte streams.

The crate version remains `0.1.0`; dock ref
`handbook.dock.json-schema@1.0.0` and bundle ref
`handbook.dock.bundle.json-schema-native@1.0.0` also remain exact. HCM-5.4 has
not been released, published, externally adopted, or slice-closed, so this
amendment may invalidate the development checkpoint and re-freeze its package
without representing two released byte identities behind one semantic version.
The old executable fingerprint
`sha256:13b8f78780e540439c30c5e805111e1acce19895c4cce83b4d5563f77e5f09b9`,
bundle fingerprint
`sha256:fd12372df6a2006ee311a43d505ad32e0ecb7c3f5c9999b42a1152c7c6c77855`,
and manifest fingerprint
`sha256:8367b798dd56c2f374b7a5c321f86d2f69461529a254329b4bb46a6fb12a9296`
are invalid superseded checkpoint material and may not admit Packet 5. Packet
4R must atomically derive and freeze the final replacements. Evidence that the
old package was released or externally consumed would invalidate this decision
and require a new version/authority adjudication before product edits.

This is a portable packaging rule, not publication. It does not require a
checked-in executable, PE/import closure, AppContainer, Job Object, platform
sandbox profile, or external installer. Test-only staging may use Cargo's
`CARGO_BIN_EXE_*` path and an explicit temporary package root behind test-only
authority; that path never enters the production registry or request protocol.

## Exact validator input and canonical fingerprints

The process receives exactly one inline `handbook.input.json-schema-validation`
object. Its closed semantic fields are:

- `root_schema`: exact logical schema ref plus the fingerprint of the root
  schema's RFC-8785 canonical JSON value;
- `root_document`: normalized safe-relative UTF-8 path under the sole logical
  `definitions/schemas/` root;
- `root_document_fingerprint`: equal to the matching closure member;
- `closure_fingerprint`: the fingerprint defined below;
- `documents`: a non-empty path-sorted, duplicate-free list of closure members,
  each containing `path`, JSON `schema`, and `document_fingerprint`; and
- `instance`: exactly one JSON-compatible value plus its
  `instance_fingerprint`.

Canonical identities are computed without host paths or transport bytes:

```text
document_fingerprint = sha256(RFC8785(schema JSON value))

closure_members = path-sorted [
  { path, document_fingerprint }
]

closure_fingerprint = sha256(RFC8785(closure_members))
instance_fingerprint = sha256(RFC8785(instance JSON value))
```

The existing Dock inline-input `content_fingerprint` independently closes over
the entire validation input, including the logical root-schema binding, closure
members, closure fingerprint, instance, and instance fingerprint. The process
request fingerprint then closes over that input plus the exact manifest,
bundle, evaluation run, claim/case partition, requested evidence kind,
Resolution, and limits.

Every supplied fingerprint is recomputed before schema construction. Changed
schema, closure, root, or instance bytes behind a claimed fingerprint produce a
child `refused` result and no candidate. The validator never loads a schema or
instance from a path; document paths are logical closure identities only.

The exact custody chain reuses the implemented owner-native record types and
their RFC-8785/SHA-256 constructors:

| Boundary | Exact owning action and identity |
| --- | --- |
| Contract to process request | Contracts prepares the current `EvaluationRunV1`; Dock constructs `DockProcessRequestV1`, whose `request_fingerprint` closes over that run, manifest/bundle, claim/case partition, evidence kind, input content fingerprint, Resolution, and effective limits. |
| Child result | The validator emits one canonical `DockProcessResultV1` line. Its `result_fingerprint` closes over request/manifest/bundle bindings, status, candidate observations or refusal, diagnostics, and actual Resolution when completed. |
| Durable execution | Dock independently re-admits the result and commits `DockExecutionRecordV1`; `record_fingerprint` closes over the request/result/host outcome, cleanup and output facts, and candidate-observation bundle fingerprint without a candidate cycle. |
| Completed candidate | Only after that commit, Dock materializes the existing `EvidenceCandidateV1`; `candidate_fingerprint` closes over the retained producer/request/execution bindings, run, subject/case, sources, observation payload, partition, and actual Resolution. |
| Canonical evidence | `handbook-contracts` resolves that candidate through its existing source port and appends `EvidenceRecordV1`; the evidence record identity closes over the re-admitted candidate plus source-authority, lifecycle, freshness, and currentness observations. |
| Derived semantic results | Contracts recomputes current claim-verdict, gate, and sibling local-closeout records from the exact evaluation run, immutable evidence root, policies, current owner heads, and complete accounting. Dock and validator contribute no semantic field. |

## Offline Draft 2020-12 validation semantics

The validator admits exactly one request only when all of these rules hold:

1. The Dock request has exactly one claim, one case, one requested evidence
   kind (`handbook.evidence.schema-conformance`), one validation input, and the
   exact manifest Resolution ceiling.
2. The root document exists exactly once in the supplied closure, its binding
   and fingerprint match, and every closure path is already normalized under
   `definitions/schemas/`, at most 1,024 UTF-8 bytes and 64 components.
3. There are 1–128 path-sorted unique documents. Each root is an object with
   exact `$schema: https://json-schema.org/draft/2020-12/schema`; a boolean
   document root, missing/nested/other dialect declaration, or meta-schema
   failure refuses.
4. Schema positions are objects or booleans and use this exact frozen Engine
   allowlist: `$schema`, `$ref`, `$defs`, `$comment`, `allOf`, `anyOf`, `oneOf`,
   `not`, `if`, `then`, `else`, `dependentSchemas`, `prefixItems`, `items`,
   `contains`, `properties`, `patternProperties`, `additionalProperties`,
   `propertyNames`, `unevaluatedItems`, `unevaluatedProperties`, `type`, `enum`,
   `const`, `multipleOf`, `maximum`, `exclusiveMaximum`, `minimum`,
   `exclusiveMinimum`, `maxLength`, `minLength`, `pattern`, `maxItems`,
   `minItems`, `uniqueItems`, `maxContains`, `minContains`, `maxProperties`,
   `minProperties`, `required`, `dependentRequired`, `title`, `description`,
   `default`, `deprecated`, `readOnly`, `writeOnly`, and `examples`. `$id`,
   `$anchor`, `$dynamicAnchor`,
   `$recursiveAnchor`, `$recursiveRef`, and `$dynamicRef` refuse. Unknown
   schema-position keywords and executable/custom hooks refuse. `format` is
   admitted only as `date-time` in these four exact logical documents:
   `definitions/schemas/handbook.schemas.lifecycle.trigger-evidence/1.0.0.schema.json`,
   `definitions/schemas/handbook.schemas.security.approver-registry-transition/1.0.0.schema.json`,
   `definitions/schemas/handbook.schemas.security.authenticator-registration/1.0.0.schema.json`,
   and
   `definitions/schemas/handbook.schemas.security.authenticator-assertion/1.0.0.schema.json`.
   The fancy-regex branch is used only when the selected root document is
   `definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json`;
   every other root uses the regular regex branch with format validation off.
   No ambient/custom format is registered.
5. `$ref` is only an empty-document or normalized safe-relative document part
   plus an RFC-6901 JSON Pointer fragment. URI schemes, `//`, absolute paths,
   backslashes, query strings, percent encoding, traversal, plain-name
   fragments, missing targets, targets outside admitted schema positions,
   cycles, and depth over 32 refuse.
6. After the complete closure is prewalked and fingerprint-checked, the
   validator constructs one in-memory `jsonschema::Registry` with exact Draft
   2020-12 internal resource URIs and a base URI for the selected root. No
   retriever, network resolver, filesystem callback, dynamic loader, content
   decoder, or executable format hook exists.
7. The pinned `jsonschema = 0.47.0` dependency is used with
   `default-features = false`. Pattern and exact date-time behavior match the
   existing Engine branches. Validator-construction or evaluation conditions
   that cannot be normalized deterministically refuse; they never become a
   conformance observation.
8. `iter_errors` is exhausted. Zero errors means `conformant`; one or more
   normalized errors means `nonconformant`. Validation never short-circuits to
   a false pass.

The exact manifest ceilings are lower than or equal to HCM-5.3's hard limits:

| Surface | HCM-5.4 manifest value |
| --- | --- |
| Request bytes | 2 MiB |
| Result bytes | 512 KiB |
| Stderr bytes | 4,096 |
| One diagnostic | 1,024 bytes |
| Candidate payload | 256 KiB |
| Inline inputs | 1 |
| Candidate observations | 1 |
| Diagnostics | 8 |
| Wall timeout | 5,000 ms |
| Cancellation grace | 100 ms |

The single inline validation object remains subject to the protocol's 1 MiB
per-input and 8 MiB aggregate hard ceiling. The closure is at most 128
documents and 32 ref edges deep. A nonconformant result may contain at most 128
unique normalized violations and must fit the 256 KiB candidate and 512 KiB
result ceilings. Surplus errors cause `limit_refused`; observations are never
truncated into partial semantic evidence.

## Normalized fact and observation vocabulary

The exact fact schema is `handbook.fact.schema-conformance@1.0.0`. One fact
contains:

- the exact root-schema binding, root-document logical path,
  root-document/closure/instance fingerprints, and closure document count;
- `conformance`, exactly `conformant` or `nonconformant`; and
- ordered `violations`.

`conformant` requires an empty violation list. `nonconformant` requires 1–128
violations. Each violation contains only:

- `code`, from the closed vocabulary below;
- `instance_pointer`, a bounded RFC-6901 pointer;
- `schema_document`, the normalized logical closure path; and
- `schema_pointer`, a bounded RFC-6901 pointer.

The closed codes are:

```text
additional_items
additional_properties
any_of
const
contains
enum
exclusive_maximum
exclusive_minimum
false_schema
format
max_items
maximum
max_length
max_properties
min_items
minimum
min_length
min_properties
multiple_of
not
one_of_multiple_valid
one_of_not_valid
pattern
property_names
required
type
unevaluated_items
unevaluated_properties
unique_items
```

Both pointers are at most 1,024 UTF-8 bytes after RFC-6901 encoding; an
oversized pointer refuses the whole observation as `limit_refused`. No library
display message, rejected instance value, regex, expected value,
host path, internal URI, nested library error object, or source excerpt enters
the fact. `AnyOf`/`oneOf` contexts normalize to their top-level closed code.
Regex-engine/backtrack failures, unresolved references after prewalk, custom
errors, and any unlisted library variant are observation-unavailable refusal,
not a new code selected by convenience.

Violations are sorted bytewise by
`(instance_pointer, schema_document, schema_pointer, code)`. Exact duplicate
tuples are coalesced before the 128-entry check. The normalized fact therefore
does not depend on iterator order or Rust display wording.

The process emits exactly one candidate for the sole claim/case:

- evidence kind `handbook.evidence.schema-conformance`;
- payload schema `handbook.fact.schema-conformance@1.0.0` with its exact
  checked-in schema fingerprint;
- `observation: satisfied` iff the fact is `conformant`;
- `observation: violated` iff the fact is `nonconformant`; and
- the complete claim partition has the sole claim in `observed`, with empty
  `unobserved` and `typed_excluded` lists.

The validator does not decide applicability, evidence eligibility, claim
verdict, gate effect, score, gate decision, or local closeout.

## Refusal taxonomy and completed-only boundary

HCM-5.4 does not extend the two process-result statuses or five child refusal
reasons. It uses them exactly as follows:

| Child refusal reason | Exact validator conditions | Required bounded diagnostic code |
| --- | --- | --- |
| `input_refused` | malformed/duplicate/unordered envelope; wrong claim/case/kind/role; changed root/document/closure/instance fingerprint | `handbook.validator.input-malformed` or `handbook.validator.input-substituted` |
| `unsupported_input` | unsupported dialect/keyword/identifier/hook; remote, ambient, encoded, traversal, dynamic, recursive, missing, cyclic, or outside-root ref | `handbook.validator.schema-unsupported`, `handbook.validator.ref-remote`, or `handbook.validator.ref-missing` |
| `limit_refused` | any checked byte/count/path/depth/violation/result bound is exceeded or arithmetic overflows | `handbook.validator.limit-exceeded` |
| `observation_unavailable` | admitted closure cannot yield a deterministic normalized observation, including regex/backtrack or unlisted library failure | `handbook.validator.observation-unavailable` |
| `internal_refusal` | unreachable serializer/registry/invariant failure after otherwise admitted input | `handbook.validator.internal-refusal` |

Diagnostic messages are stable, generic, control-free, at most 1,024 bytes,
and reveal no rejected bytes or host facts. A child refusal exits zero with one
valid `refused` result and no candidate. Malformed framing, crash, nonzero exit,
timeout, cancellation, extra stdout, output overrun, binding mismatch, cleanup
uncertainty, and durability uncertainty remain host-owned HCM-5.3 outcomes and
never masquerade as a child refusal or conformance fact.

Only a zero-exit, exact canonical `completed` result, confirmed cleanup, and
durable execution-record commit may materialize the one candidate. Every other
path exposes zero candidates and writes zero canonical evidence. Existing Dock
operation problem causes and Contracts candidate/evidence refusal types remain
closed and unchanged.

## Exact contract fixture and end-to-end lifecycle proof

The slice adds one executable, non-production contract fixture family under
`tests/fixtures/hcm_5_4_json_schema_dock/`. Exact
fingerprints are generated from final canonical fixture bytes and checked by
tests; no placeholder is admitted.

The fixture uses:

- contract `handbook.contract.fixture.json-schema-dock-proof@1.0.0`;
- one claim `payload.schema-conformant`, statement “The selected payload
  conforms to the exact local schema closure.”;
- exact-fields subject selector `artifact.kind = json-document`, case selector
  `validation.case = primary`, and `subject_and_case_match` applicability. The
  evaluation subject/case expose those exact matcher inputs, the sole case id is
  `case.schema-conformance`, and their content bindings are derived from the
  fixture's canonical bytes;
- `hard_fail`, exactly-one
  `handbook.evidence.schema-conformance`, current outcome-consensus stability,
  no score weight, and the exact manifest Resolution minimum;
- freshness policy
  `handbook.contract-evidence-freshness-policy.hcm-5-4-proof@1.0.0` with
  300-second maximum age, exact revision, and zero future skew;
- gate policy `handbook.contract-gate-policy.hcm-5-4-proof@1.0.0` using closed
  accounting, no score threshold, and the existing gate-passed local-closeout
  policy;
- four deterministic fixture credentials with pairwise-distinct derived actor
  refs: authenticated definition author, independent lock actor, activation
  actor, and `contract_evidence_source` actor under the current sealed Engine
  authority boundary; and
- one subject, one case, a two-document local schema closure using a relative
  JSON-Pointer `$ref`, one conformant instance, and one nonconformant instance.

The required positive path is exact and owner-driven:

```text
contract.definition.append
  -> contract.lifecycle.transition (draft -> review_ready)
  -> contract.lifecycle.transition (review_ready -> locked, distinct actor)
  -> contract.lifecycle.transition (locked -> active)
  -> Contracts prepare_evaluation_run_v1
  -> dock.run (production manifest and native bundle)
  -> completed Dock result / durable execution record / untrusted candidate
  -> contract.evidence.append (Contracts-owned canonical evidence)
  -> contract.verdict.evaluate (pass)
  -> contract.gate.evaluate (passed + sibling local_closeout eligible)
```

The nonconformant fixture follows a distinct evaluation nonce/run and yields a
completed `violated` candidate, admitted canonical evidence, claim `fail`, gate
`blocked`, and local closeout `blocked`. This is a semantic negative result, not
a validator refusal. Refusal fixtures produce no candidate/evidence/verdict/
gate/local-closeout result for their failed run.

The fixture is product-consumed test evidence, not a production contract
registry, default policy, automatic lifecycle mutation, waiver, or publication
artifact.

## Replay, currentness, Resolution, and evidence rules

1. Every `dock.run` first prepares one exact current active-after-independent-
   lock evaluation run in Contracts and Dock re-admits it before manifest,
   input body, bundle, or spawn access.
2. The run must contain exactly the manifest Resolution ceiling and the
   fixture's one claim/case. Under-resolved, overclaimed, stale-head, bare-lock,
   deprecated, closed, substituted-policy, or caller-assembled run input spawns
   nothing and writes nothing.
3. Exact outward idempotency replay returns the original execution record and
   candidate refs after current run preparation; conflicting key/request reuse
   writes nothing. Replay never launches a second process or creates a second
   candidate.
4. `contract.evidence.append` resolves the committed Dock candidate through
   the existing source port, recomputes every request/result/execution/
   manifest/bundle/fact binding, and rechecks lifecycle, source revision,
   evidence-source authority, freshness, and Resolution immediately before
   the canonical append.
5. Exact evidence replay returns the original evidence/receipt; candidate or
   key conflict writes nothing. Dock completion and source authentication alone
   never satisfy a claim.
6. Verdict and gate/local-closeout are unpersisted and recomputed from one
   complete immutable evidence root plus current lifecycle/source/authority
   heads. Revocation, staleness, changed schema/instance closure, corrupt or
   surplus inventory, insufficient Resolution, timeout, cleanup uncertainty, or
   incomplete accounting cannot become green.
7. Execution-record identity closes over the candidate-observation bundle;
   final Dock candidate identity closes over the execution record; canonical
   evidence identity closes over the candidate; verdict, gate, and local-
   closeout identities close over the current run/evidence/policy bases. No
   identity cycle or transport fingerprint is introduced.

## Scope and allowed implementation boundary

Slice implementation may change only:

- new `crates/dock-json-schema/**` product source, native binary, crate-local
  fixtures/tests, and target-neutral package staging support;
- the narrowest directly necessary `crates/dock/**` production manifest/
  current-target bundle resolver and focused tests, without changing protocol,
  host-outcome, store, candidate, or semantic-authority rules;
- directly necessary SDK schema generation for the two validator input/fact
  roots, checked-in schema
  assets, exact manifest/catalog/production-witness counts, default production
  Dock execution composition, exports, and focused tests under `crates/sdk/**`;
- focused Contracts tests and the exact HCM-5.4 contract fixture family needed
  to prove the existing owner path, without changing HCM-5.1/HCM-5.2 semantics;
- `tests/fixtures/hcm_5_4_json_schema_dock/**` for the exact non-production
  cross-owner fixture family;
- generic CLI/Tauri regression tests needed to prove unchanged discovery,
  schema retrieval, invocation, and typed results; workspace/directly affected
  manifests; and
- this authority plus directly necessary current baseline, architecture, seam,
  roadmap, navigation, and Phase-5 exit truth after executable state changes
  and aggregate closeout.

An implementation packet may make a narrow internal API change needed to admit
the first production registry entry or owner-resolved bundle, but must run
upstream GitNexus impact before each symbol edit and honor HIGH/CRITICAL risk
warnings. It may not add a caller-supplied production path, public arbitrary
manifest constructor, or alternate semantic path.

## Explicit non-goals

HCM-5.4 does not authorize:

- a Python product dependency, Python validator/harness revival, ambient
  interpreter, package installer, virtual environment, or runtime package
  discovery;
- a protocol version/change, second manifest, second validator, new operation,
  new capability, third schema root, command-specific CLI/Tauri route, or
  installed-skill workflow branch;
- remote/network refs, filesystem schema discovery, repository paths in the
  child request, executable/custom schema hooks, content decoders, dynamic
  loading, shell/search-path launch, or inherited environment;
- arbitrary third-party bundle admission, plugin marketplace, hostile-code
  sandbox claim, AppContainer, Job Objects, Windows-only host behavior,
  platform-specific public semantics, PE/import dependency closure, or general
  filesystem/network/syscall/process-tree/CPU/memory isolation;
- canonical evidence, applicability, verdict, gate, local-closeout, lifecycle,
  promotion, posture, or receipt construction by the validator or Dock;
- persisted verdict/gate/local-closeout state, parent promotion, posture
  mutation, Flow/pipeline integration, GUI work, publication, bridge,
  Substrate/external adoption, or Phase 6; or
- adjacent-slice preparation, Packet 1 dispatch, packet concurrency, packet-
  level slice closeout, or product implementation by this preparation task.

## Cross-packet invariants

Every packet must preserve:

1. Contracts alone owns canonical evidence, applicability, verdict, gate,
   local-closeout, lifecycle, and mutation truth.
2. Dock alone owns manifest/bundle admission, execution, host outcomes,
   operational records, and completed untrusted candidates.
3. The validator is a deterministic offline read-only observer under the exact
   language-neutral V1 protocol; Rust is implementation, not semantic authority.
4. Exact manifest, bundle, target, request, input, closure, instance, result,
   execution, candidate, evidence, and derived-result bindings are recomputed at
   every owning boundary; substitution refuses.
5. The production registry contains exactly one current-target manifest and no
   test helper. Missing package material is typed unavailability, never an
   ambient search.
6. Only one validation input, one instance, one local closure, one claim/case,
   one evidence kind, and at most one candidate are admitted per process.
7. Remote/ambient/dynamic/recursive refs, unsupported dialects/keywords/hooks,
   changed fingerprints, and over-limit or indeterminate validation refuse with
   no candidate.
8. Conformant/nonconformant is observational fact only. Candidate
   satisfied/violated mirrors it; Contracts independently decides verdict/gate.
9. Only completed, zero-exit, exact, clean, durably recorded execution exposes a
   candidate. No partial/truncated observation becomes evidence.
10. Replay/currentness/freshness/Resolution/accounting remain exactly the
    HCM-5.2/HCM-5.3 rules; the validator cannot weaken them.
11. Final inventory truth is exactly 62 operations, 31 capabilities, 170 roots,
    one production manifest, and six fixed use cases.
12. The validator has no Handbook product-crate dependency. No Python, ambient
    interpreter/package, network/process/dynamic-loading path exists in it or
    its enabled normal dependency/source closure.
13. Public claims remain portable and evidence-matched; no hostile-code or
    OS-specific confinement claim is added.

## Ordered minimal packet decomposition

| Packet | Bounded outcome | Hard dependency | Verification and allowed boundary |
| --- | --- | --- | --- |
| **1 — validator input/fact kernel and schema identities** | Add the purpose-named crate with closed input, closure-member, fact, violation, refusal, limit, and canonical-fingerprint types; add exactly the two schema identities and focused cross-crate canonical vectors. Add no `jsonschema` evaluation, process binary, production manifest, Dock change, or lifecycle fixture. | HCM-5.3 closeout and this authority. | Focused crate/SDK tests prove exact refs, canonical document/closure/instance/fact fingerprints, path/order/duplicate/count/unknown-field refusal, schema generation/meta-validation, 170-root derivation in test-only generation, no third root, no Handbook product-crate back-edge, and no Python/OS branch. |
| **2 — exact offline Draft 2020-12 evaluator** | Implement the pure pinned-Rust local-closure prewalk, exact current allowlist/dialect/meta-validation/ref rules, in-memory registry, one-instance evaluation, normalized closed violations, ordering/coalescing, and refusal mapping. Add no process entrypoint, manifest, bundle resolver, or Contracts path. | Packet 1. | Exhaustive positive/negative/multi-document/refusal tests cover local pointers/refs, missing/remote/ambient/encoded/traversal/cyclic/deep refs, dialect/keyword/identifier/hook refusal, document/root/closure/instance substitution, meta failure, every reachable normalized code, deterministic order, over-limit no-partial behavior, and source/dependency gates. |
| **3 — native one-shot process and package artifact** | Add the `handbook-dock-json-schema` binary over the exact language-neutral Dock request/result wire contract, one canonical stdin/stdout frame, closed child refusal reasons/diagnostics, empty fixed argv, and target-neutral staging of the single-file native bundle plus canonical manifest artifact. Production registry remains unchanged. | Packet 2. | Direct native-process tests prove exact completed conformant/nonconformant results, one candidate and complete partition, all child refusals, invalid/extra/noncanonical framing, no stdout side channel, cross-crate canonical repeatability, target entrypoint identity, and that normal binary source/dependencies contain no filesystem schema read, env/PATH, network, spawn, dynamic loading, Python, Handbook product crate, or OS-specific branch. |
| **4 — first production manifest and ordinary Dock run** | Freeze the target package's exact manifest into the host, change the production registry from empty to that one current-target definition, add owner-resolved fixed-layout installed-manifest/bundle admission, and make the existing default SDK `dock.run` path resolve/execute it after Contracts run preparation. Preserve the explicit test runtime seam and all three operation identities/schemas. | Packet 3. | Dock/SDK tests prove list/read exact one-entry embedded truth, installed-manifest equality, absent/substituted/wrong-target/extra/symlink bundle refusal, current-run-before-body/manifest/spawn order, direct launch, exact replay, timeout/cancellation/process/protocol/cleanup/durability no-candidate paths, default SDK completion, no runtime rederivation, no local mapping disclosure, no caller path, and unchanged `62 / 31` operation/capability truth. |
| **4R — deterministic package re-freeze** | Supersede only Packet 4's invalid package checkpoint. Make the current-target normal-dependency package build repository-controlled and independent of test-only `CARGO_BIN_EXE_*`, apply the exact `/Brepro` final-link rule, add a clean-build reproducibility guard, regenerate the production manifest from final executable bytes, and re-freeze those identical canonical bytes into the host. Preserve every Packet 1–4 public identity and runtime trust check. | Packets 1–3 and the invalid Packet 4 source checkpoint; Packet 5 remains blocked. | From two or more fresh isolated target directories, build identical current source and `Cargo.lock` with the selected `dev` package profile, require byte-identical whole executables, stage each with the production staging implementation, and require byte-identical canonical manifests. Recompute and check exact executable length/digest, one-file inventory, empty argv and its fingerprint, target, bundle fingerprint, and manifest fingerprint. Prove test-only `CARGO_BIN_EXE_*` cannot supply production frozen bytes. Run Packet 3 staging/process and Packet 4 Dock/SDK trust, admission, launch, replay, and default-path suites; prove unchanged `62 / 31 / 168 / 1 / 6` interim truth, no runtime trust weakening or new public semantics, all affected regressions, formatting/checks, focused P1–P4 review, and GitNexus change detection. Any mismatch stops without re-freezing. |
| **5 — exact contract-to-local-closeout proof** | Add the exact fixture family, establish its definition and independent-lock/active lifecycle, run conformant and nonconformant cases through the production validator, append Contracts-owned evidence, and recompute verdict, gate, and local closeout. Add no production contract/policy default or semantic branch. | Packet 4R. | Focused Contracts/Dock/SDK integration proves the exact positive chain to `pass / passed / eligible`, negative chain to `fail / blocked / blocked`, immutable evidence and replay, current-head/source-authority/freshness/Resolution rechecks, refusal/timeout/fingerprint/under-resolution write-nothing behavior, complete accounting, and validator/Dock non-authority. |
| **6 — generic consumers, integrated slice proof, and current truth** | Prove the existing Rust SDK, generic CLI, and generic Tauri consumers discover/read/invoke the one manifest and real path without new route; finalize the two schema assets, exact counts, package/source/security gates, and current product/navigation documentation. This is integration, not packet-level closeout. | Packet 5. | Run focused validator/Dock/Contracts/SDK/CLI/Tauri suites, deterministic schema regeneration/meta-validation/external-retrieval refusal, real packaged native proof, formatting, workspace check/test, strict Clippy/rustdoc, dependency/source/security/no-Python/no-OS-specific-drift/generic-consumer/Markdown-link gates, final `62 / 31 / 170 / 1` proof, and required GitNexus change detection. |

The order is strictly sequential:

```text
HCM-5.3 closeout -> P1 -> P2 -> P3 -> P4 (invalid) -> P4R -> P5 -> P6
```

Each packet returns control to the meta task. No packet dispatches the next,
claims HCM-5.4 or Phase-5 completion, begins Phase 6, or performs aggregate
closeout.

## Delegated just-in-time preparation

Separate packet files are optional. Each fresh packet task must reconcile this
authority with current source/tests and its completed predecessor before
product edits.

- Packet 1 may choose private module/file names and `$defs` factoring, not the
  crate/dock/schema/evidence/input identities, two-root count, fingerprint
  formulas, or public closed fields.
- Packet 2 may choose private traversal/registry/error-mapping helpers, not the
  dialect/keyword/ref rules, limits, normalized codes/order, or refusal mapping.
- Packet 3 may choose private stdin/stdout and package-staging helpers, not the
  binary/entrypoint identity, empty arguments, one-shot framing, source gates,
  or production/test separation.
- Packet 4 may choose private resolver/helper factoring and exact error
  plumbing, not the fixed layout, one-entry registry, owner-before-launch order,
  target binding, existing operation identities, or no-caller-path rule.
- Packet 4R may choose private repository tooling and test factoring, not the
  normal-dependency `dev` package profile, separation from test-only
  `CARGO_BIN_EXE_*`, `/Brepro` final-link requirement, locked/offline current-
  package selection, two-clean-build byte proof, manifest closure, existing
  semantic versions, or invalidation of Packet 4's old fingerprints.
- Packet 5 may choose fixture file factoring and private setup helpers, not the
  exact contract/policy/claim/evidence identities, lifecycle chain, positive/
  negative outcomes, or Contracts-only semantic path.
- Packet 6 may choose integration-test factoring, not the generic-consumer
  rule, final counts, required gates, or Phase-5 exit evidence.

If permitted current sources leave more than one consequential answer about
public identity, schema/ref semantics, manifest/package trust, launch mapping,
candidate facts, Contracts currentness, operation/capability/schema closure, or
portable security claims, the packet returns `STOPPED` before the affected
edit. Private names, fixture arrangement, and test helper factoring are
recoverable preparation, not a stop.

## Friction and adjudication rules

- A missing generated fingerprint or count before its producing packet is
  recoverable: derive it from final canonical bytes, freeze it, and test it.
  Never invent a placeholder.
- If the existing Dock API cannot install/resolve one production bundle without
  a caller path, Packet 4 may add the narrow owner-resolved capability described
  here. It may not expose general registration, runtime-derived manifest trust,
  or ambient discovery.
- If the two selected roots cannot express a required nested shape, keep the
  shape in `$defs`. A demonstrated need for another independently retrievable
  root stops for authority remediation.
- If a library error cannot map to the closed observation vocabulary without
  implementation-specific text or lost conformance truth, map it to
  `observation_unavailable`. Do not extend the vocabulary in place.
- If deterministic timeout proof cannot use the production validator without a
  flaky timing assertion, use the existing/test-only native host fixture to
  prove the HCM-5.3 timeout/no-candidate boundary and separately prove the
  production validator's normal/refusal paths. Test-only delay controls may not
  enter the production binary, manifest, or request schema.
- If a supported target requires a different public protocol, manifest field,
  semantic observation, or host claim, stop. Private executable suffix and
  package staging mechanics may vary by target while public semantics remain
  exact.
- Review findings and failed checks with an in-authority repair path are
  remediation, not a slice stop. Scope expansion, unresolved identity/trust
  choices, or an unavoidable ambient/third-party execution dependency is a
  genuine stop.

## Slice acceptance

HCM-5.4 completes only when all of these conditions hold:

1. `handbook-dock-json-schema` exists at the selected dependency seam with the
   exact native binary and dock identities, and no semantic-owner back-edge.
2. The validator uses pinned `jsonschema = 0.47.0`, default features off, and
   exact offline Draft 2020-12 local-closure practices with no retriever.
3. Input/document/closure/instance/fact fingerprints use the fixed canonical
   formulas and every claimed binding is recomputed before use.
4. Remote/ambient/dynamic/recursive refs, unsupported dialects/keywords/hooks,
   missing/cyclic/outside-root closure, and changed fingerprints refuse before
   semantic admission with no candidate.
5. Exactly one JSON-compatible instance, one closure, one claim/case, one
   evidence kind, and the exact Resolution ceiling are admitted per request.
6. Conformance facts use the closed vocabulary, deterministic ordering,
   bounded/coalesced locations, and no library text or rejected values.
7. Conformant emits one satisfied candidate; nonconformant emits one violated
   candidate; refusal emits none. No truncated/partial observation is exposed.
8. The production registry contains exactly one exact current-target manifest,
   whose one-file native bundle, entrypoint, empty argv, target, limits, schema
   bindings, and fingerprints are fully reverified before direct launch. Before
   the manifest is frozen, at least two isolated clean builds from identical
   repository-controlled inputs produce byte-identical whole executables and
   byte-identical staged canonical manifests under the exact current-target
   deterministic package rule; any mismatch stops without replacement.
9. Production bundle resolution uses only the fixed host-relative layout, with
   no caller path, environment/PATH/current-directory/home/repository search,
   shell, package manager, or mapping disclosure.
10. Timeout, cancellation, process/protocol failure, cleanup uncertainty, and
    durability uncertainty remain host-owned, durably accounted where
    applicable, and expose no candidate/evidence.
11. The exact contract fixture reaches independent lock then active state under
    distinct admitted actors before evaluation; stale/bare/substituted states
    spawn nothing and write nothing.
12. The positive real path reaches completed execution, Contracts-owned
    immutable evidence, `pass`, gate `passed`, and sibling local closeout
    `eligible` with exact current bases.
13. The negative real path reaches completed violated evidence, `fail`, gate
    `blocked`, and local closeout `blocked`; refusal paths fabricate no semantic
    result.
14. Dock/evidence replay is exact, single-record, and conflict-safe; verdict,
    gate, and local closeout remain unpersisted and currentness-sensitive.
15. Exactly two new roots yield final truth of 62 operations, 31 capabilities,
    170 schema roots, one production manifest, and six fixed use cases, with all
    predecessor rows/roots/capabilities/transport vectors protected.
16. Rust SDK and unchanged generic CLI/Tauri consumers discover/read/invoke the
    path without command-specific or installed-skill branches and preserve
    owner terminal/receipt/replay truth.
17. The validator has no Handbook product-crate dependency, and its enabled
    normal source/dependency closure contains no Python,
    ambient interpreter/package, network, arbitrary filesystem schema read,
    process spawn, executable hook, dynamic loading, or OS-specific semantic
    branch; no stronger confinement or PE-dependency claim is made.
18. Focused tests, deterministic schema regeneration, formatting, workspace
    check/test, strict Clippy/rustdoc, dependency/source/security/no-Python/
    no-OS-specific-drift/generic-consumer/Markdown-link gates, and required
    GitNexus change detection pass or limitations are reported honestly.
19. One aggregate P1–P4 causal review of the final integrated slice has no
    unresolved P1 or P2.

## Aggregate closeout and Phase-5 exit evidence

Each packet performs only focused verification and returns control. Packet
completion never closes HCM-5.4.

After Packet 6, the meta task must recheck all 19 acceptance conditions against
the integrated product, reuse still-valid packet evidence, rerun invalidated
and cross-layer checks, and conduct one focused P1–P4 causal review. It must
remediate/reverify/re-review blocking P1/P2 within authority or record a genuine
workflow stop. P3/P4 remain advisory.

Only then may the meta task record one HCM-5.4 slice closeout. The same aggregate
evidence satisfies the Phase-5 exit test only when it proves:

- one exact active contract drove the real selected production Dock path;
- completed candidates crossed Contracts-owned evidence, verdict, gate, and
  local closeout without moving semantic authority;
- missing, stale, malformed, substituted, under-resolved, timed-out, cancelled,
  failed, protocol-invalid, cleanup-uncertain, and durability-uncertain inputs
  failed closed with no false candidate/evidence/green result; and
- the existing ordinary contract and Dock use cases remained behind SDK types
  and unchanged generic consumers at exact `62 / 31 / 170 / 1` truth.

When those facts and HCM-5.4 closeout are current, the meta task may update the
phase-and-slice map to **Phase 5 exited** in the single aggregate closeout
change. That exit does not authorize publication, a third-party dock, Phase 6,
or an external-adoption claim.

## Use and drift

Use this document as the current HCM-5.4 outcome and packet authority. Source
and tests remain product truth. During implementation, update current product,
architecture, seam, roadmap, and navigation truth only when executable state
actually changes.

Do not infer HCM-5.4 implementation, slice closeout, Phase-5 exit, publication,
or Phase-6 authority from preparation of this document.
