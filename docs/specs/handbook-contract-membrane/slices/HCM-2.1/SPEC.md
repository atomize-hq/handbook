# HCM-2.1 Specification: Project Context Canonical-YAML Pilot

## Status and authority

This is the complete planning-only implementation packet for `HCM-2.1`. It is
authority for a future, separately selected implementation session only after
the exact planning subject receives a fresh `CLEAN` review and the parent
planning closeout records that result. It authorizes no Rust, product, fixture,
template, or runtime implementation in the session that creates or closes this
packet.

The packet consumes, without reopening, the reviewed HCM-1 kind/schema,
profile/descriptor, selected-registry, and setup/doctor-decision boundaries.
Entry evidence is:

- branch `feat/handbook-contract-membrane`;
- planning entry HEAD `78171f5024a20d23919e41dc2d30f32fa9df3b6e`;
- HCM-1.4 reviewed implementation commit
  `353e3bf0e0e8e75e470c19487ee2783c8c1a2aaa`;
- selected HCM-1.4 closeout
  `20260718T191425Z--HCM-1-4--orchestration--profile-aware-setup-doctor-landed`;
- exact shipped profile ref
  `handbook.profile.shipped-root@1.0.0` with reviewed fingerprint
  `sha256:ee8f327c5a5a68faf3184bad268118b3014e96beea142b1f5959b701e857cdf6`;
- exact `project_context` descriptor path `.handbook/project/context.yaml`;
- exact kind ref `handbook.artifact-kind.project-context@1.0.0` and content
  schema ref `handbook.schemas.artifacts.project-context@1.0.0`; and
- the HCM-0.6 approved Project Context responsibility, lifecycle posture, and
  first-party fixed-renderer requirement.

The selected HCM-1.4 handoff is completed dependency and transition evidence.
It cannot widen this slice, authorize another artifact family, or select a
future implementation session. Live source, tests, definition bytes, control-
pack authority, and Git history at the entry HEAD override stale narrative
wording.

## Objective

Cut exactly the shipped `project_context` artifact instance over to one
profile-selected canonical YAML authority while retaining one fixed,
deterministic, first-party Markdown renderer as an in-memory human-review view.
The future implementation must make the selected descriptor path and bound
content schema drive Project Context load, validation, write, authoring,
doctor, and flow behavior; produce exact source-byte and rendered-output
fingerprints; and stop every selected product path from reading the retired
Project Context Markdown authority.

The pilot must:

1. use the exact HCM-1.2 `handbook.artifact.project-context` / `1.0` content
   shape without editing or version-substituting the admitted schema, kind,
   profile, or instance descriptor;
2. parse duplicate-safe YAML into the JSON data model, validate it through the
   selected instance registry, and closed-decode it into one typed engine-owned
   canonical record;
3. serialize one deterministic canonical YAML byte form for authoring writes;
4. render one deterministic Markdown review view from the typed canonical
   record with no wall clock, environment, repository, Resolution, vocabulary,
   or ambient-state input;
5. fingerprint the exact observed/written canonical YAML bytes and the exact
   rendered UTF-8 Markdown bytes independently with lowercase SHA-256;
6. keep setup non-authoring: missing required Project Context remains
   `author_required`, valid canonical YAML remains `preserve`, and setup writes
   neither content nor a Markdown starter;
7. extend doctor's machine report additively to expose the pilot's source and
   rendered-output fingerprints only when the same selected artifact row is
   structurally valid and typed-decodable;
8. make `handbook-flow` assemble the Project Context packet section only from
   the selected canonical YAML record and its in-memory rendered view;
9. cut Project Context authoring inputs, focused tests, and live-skill smoke
   fixtures directly to the canonical `1.0` record without a legacy mapper,
   dual-read mode, importer, or compatibility profile; and
10. cut Environment Inventory's reference-only Project Context dependency from
    the retired Markdown path to the exact selected YAML descriptor path,
    without converting Environment Inventory content authority or authoring
    model.

The output is a vertical Project Context pilot, not a generic canonical-
artifact framework. It starts no HCM-2.2+ work.

## Pilot selection and fixed authority

The explicitly selected HCM-2.1 row names Project Context as the current
leading candidate and the HCM-0.6 decision selects one always-required root
instance. This packet fixes that named candidate as the pilot. It does not add,
remove, rename, re-version, or change requiredness for any kind or instance.

The canonical record is exactly:

| Field | Exact contract |
|---|---|
| `schema_id` | literal `handbook.artifact.project-context` |
| `schema_version` | literal `1.0` |
| `record_id` | HCM-1.2 `recordId` grammar |
| `summary` | non-empty bounded long text |
| `system_boundaries` | 1-64 unique bounded strings |
| `ownership` | 1-64 unique bounded strings |
| `authoritative_references` | 0-128 unique stable refs |
| `known_unknowns` | 0-64 unique bounded strings |

Unknown fields, duplicate YAML keys at any nesting level, multiple YAML
documents, aliases/tags that do not normalize to the admitted JSON data model,
non-object roots, wrong constants, schema failures, typed-decode disagreement,
oversized input, unsafe paths, symlinks, and non-regular files fail closed.
There is no application-level defaulting and no field is inferred from the
repository, old Markdown, the Charter, a template, or an agent.

The HCM-1.2 schema is already fingerprint-bound admitted authority. HCM-2.1
must not expand it to preserve the richer pre-membrane
`ProjectContextStructuredInput` shape. That `0.1.0` authoring model and its
timestamped Markdown format are precedent only and are retired from selected
Project Context product paths by this direct cutover.

## Canonical bytes, rendering, and fingerprints

### Canonical YAML

The engine owns a typed `CanonicalProjectContext` whose fields match the exact
table above. Parsing follows one ordered boundary:

1. enforce the existing selected-artifact byte limit;
2. parse exactly one duplicate-safe YAML document into the JSON data model;
3. require an object root;
4. locate the selected artifact decision whose typed `SymbolicId::as_str()` is
   `project_context`, then call
   `decisions.registry().validate_json(decision.instance_id(), &value)`;
5. closed-decode the validated value into `CanonicalProjectContext`; and
6. require the decoded constants and field values to re-encode to the same
   JSON data model before accepting typed truth.

Authoring serialization does not delegate fingerprint-significant choices to a
general YAML serializer. It uses this closed emitter:

1. emit the eight mapping keys in the exact table order above; scalar-valued
   keys and empty-list keys use the bare key plus `: `, while a non-empty-list
   key uses the bare key plus `:` and an immediate LF with no intervening
   space;
2. emit every string scalar with enclosing double quotes; escape `"`, `\`,
   backspace, form feed, LF, CR, and tab as `\"`, `\\`, `\b`, `\f`, `\n`,
   `\r`, and `\t`; escape every other U+0000-U+001F code point as lowercase
   `\u00xx`; do not escape `/`; emit every other Unicode scalar directly as
   UTF-8; never emit plain, single-quoted, literal, folded, anchor, alias, or
   tagged scalars;
3. emit a non-empty sequence as a newline followed by one `  - ` plus JSON
   string per item in record order; emit an empty sequence as `[]` on the key
   line;
4. emit no document marker, comment, blank line, trailing space, tab, or CR;
   and
5. emit UTF-8 with LF separators and exactly one final LF.

The implementation may use `serde_json::to_string` for each already validated
string scalar only while byte-golden tests prove it implements the exact rule
above; it must assemble the YAML mapping/list framing itself. Therefore the byte
contract is not coupled to a YAML crate's version or configuration.
Parsing may accept any structurally equivalent input admitted by the earlier
duplicate-safe boundary; the source fingerprint remains the exact observed
bytes. A successful authoring write uses the closed emitted bytes and returns
their fingerprint.

The future implementation must add a literal boundary fixture whose canonical
output bytes are exactly:

```yaml
schema_id: "handbook.artifact.project-context"
schema_version: "1.0"
record_id: "handbook.project-context"
summary: "Coordinates \"alpha\"\nboundary #1."
system_boundaries:
  - "API -> worker"
  - "No <external> writes"
ownership:
  - "Platform/Ops"
authoritative_references:
  - "handbook.project-context@1.0.0"
known_unknowns:
  - "Which region?\tOwner TBD"
```

The code fence has one final LF after its last data line and no blank data line;
the golden fixture bytes must match the displayed data lines exactly.

### Fixed renderer-derived Markdown view

The existing engine-owned Project Context renderer is replaced in place as the
fixed first-party renderer for the canonical `1.0` record. It remains a pure
engine transformation but its input becomes `&CanonicalProjectContext` and it
has no timestamp parameter or environment lookup. Its exact v1 headings are:

```text
# Project Context
## Summary
## System Boundaries
## Ownership
## Authoritative References
## Known Unknowns
```

The title is literal. Every rendered record string first passes the same closed
`plain_text` transform:

1. replace each CRLF pair, remaining CR, LF, or tab, together with any adjacent
   ASCII spaces, by one ASCII space; collapse all remaining runs of ASCII spaces
   and trim leading/trailing ASCII spaces;
2. reject U+0001-U+0008, U+000B-U+000C, U+000E-U+001F, and U+007F; and
3. prefix `\` before every ASCII punctuation character in the exact set below,
   including a source backslash; preserve all other Unicode scalar values
   byte-for-byte in UTF-8.

```text
! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
```

An empty result is refused. `summary` is emitted as one transformed paragraph;
list values are emitted in canonical record order as controlled `- ` bullets;
an empty `authoritative_references` or `known_unknowns` list emits the literal
`- None recorded.`. Only renderer-owned headings, blank lines, and bullet
prefixes can create Markdown structure.

The normalization/refusal table is exhaustive:

| Input condition | Exact result |
|---|---|
| CRLF, CR, LF, tab, or runs of ASCII spaces | one ASCII space after collapse |
| any listed ASCII punctuation | same character preceded by one backslash |
| source backslash | two output backslashes |
| U+0001-U+0008, U+000B-U+000C, U+000E-U+001F, U+007F | render refusal |
| leading/trailing ASCII spaces | removed |
| zero characters after normalization | render refusal |
| any other Unicode scalar | preserved exactly; no Unicode normalization |

For the canonical boundary fixture above, the exact Markdown fixture is:

```text
# Project Context

## Summary

Coordinates \"alpha\" boundary \#1\.

## System Boundaries

- API \-\> worker
- No \<external\> writes

## Ownership

- Platform\/Ops

## Authoritative References

- handbook\.project\-context\@1\.0\.0

## Known Unknowns

- Which region\? Owner TBD
```

The golden fixture has UTF-8/LF bytes and exactly one final LF after its last
data line. It has no blank data line after that LF.

The rendered view is not written as a repository file in HCM-2.1. Authoring,
doctor, and flow render it in memory from accepted canonical YAML. This avoids
a two-file transaction and prevents a persistent Markdown peer authority or
stale derived file. The view has no capitalized `Projection` definition,
request, Resolution, vocabulary, lossiness, or provenance claim.

### Fingerprints

- `source_fingerprint` is `sha256:` plus the lowercase SHA-256 of the exact
  accepted canonical YAML bytes.
- `rendered_output_fingerprint` is `sha256:` plus the lowercase SHA-256 of the
  exact rendered Markdown bytes.
- The two domains are not interchangeable and neither is a definition,
  semantic-approval, Resolution, Projection, lifecycle, freshness, or evidence
  fingerprint.
- Repeated load/render of identical bytes must reproduce both fingerprints.
- Semantically equivalent but byte-different accepted YAML may reproduce the
  rendered fingerprint while producing a different source fingerprint; the
  API and proof must preserve that distinction.

## Selected-path integration contract

### Engine owner

`handbook-engine` owns canonical parsing, selected-schema validation, typed
decode, deterministic serialization, rendering, and both fingerprint
derivations. It also owns a bounded no-follow load operation that accepts an
already resolved `&ResolvedProfileDecisions`, finds only instance ID
`project_context`, verifies the exact kind/schema binding, and reads only the
descriptor's canonical path.

No HCM-2.1 public API accepts an arbitrary repository path, a
`CanonicalArtifactKind` selector, legacy Markdown bytes, an unverified schema
ref, or an alternate instance ID. The compiler and flow may project the typed
result but may not reparse, revalidate, render, or fingerprint independently.

#### One retained observation for inspection and doctor

Engine inspection must not discard validated Project Context bytes and then
perform a second semantic load. For only the selected `project_context` row,
the existing strict descriptor-relative open retains one private
`CanonicalProjectContextObservation` containing the initial file identity, the
exact bounded source bytes, typed record, rendered bytes, and both fingerprints.
Compiler code receives read-only accessors for the path, record, rendered bytes,
lengths, and fingerprints; the raw retained bytes and file identity stay engine-
private. All Project Context structural validation, typed decode, render, and
fingerprinting use that one retained byte vector.

The existing artifact row remains the readiness authority. Add exactly three
inspection reasons without changing any existing reason's meaning:

| Closure failure | Artifact status | Artifact reason | Project Context row |
|---|---|---|---|
| schema-valid JSON cannot closed-decode or roundtrip | `structurally_invalid` | `typed_decode_failed` | null |
| typed record cannot pass the fixed renderer | `structurally_invalid` | `rendered_view_refused` | null |
| final descriptor stability check differs or fails | `unreadable` | `observation_changed_during_inspection` | null |

Precedence is exact: aggregate/document limit; path/open/read; duplicate or YAML
syntax; object root; selected schema; typed decode/roundtrip; renderer; final
stability. The first failing stage supplies the row status/reason. Unchanged
non-Project-Context rows retain their HCM-1.4 path and reasons byte-for-byte.

On Unix, the final stability check strict-reopens the same descriptor path
immediately before report projection and requires the same regular-file device
and inode plus exact byte equality with the retained observation. A replacement
with identical bytes is invalid because its file identity differs; a same-file
content change is invalid because its bytes differ. No report path ever reads a
second file's bytes for typed/rendered truth. Test hooks must exercise different-
byte substitution and identical-byte inode-replacement ABA. A failed final
check forces overall readiness `INVALID` through the existing row and emits no
fingerprints.

### Authoring and CLI

`handbook author project-context --validate --from-inputs <path|->` validates
the supplied canonical `1.0` YAML without mutation. The non-validate command:

1. resolves the shipped profile decisions;
2. parses/validates/renders the complete candidate before mutation;
3. acquires the retained no-follow Project Context authoring lock;
4. rechecks root, selected descriptor, target safety, and existing truth;
5. refuses overwrite of structurally valid typed canonical YAML;
6. atomically writes only `.handbook/project/context.yaml`; and
7. returns canonical path, byte length, source fingerprint, rendered-output
   fingerprint, and rendered media type `text/markdown`.

Invalid or unreadable existing selected truth requires repair; authoring may
replace a present structurally invalid regular file only after the same
preflight used by the current command and only with a fully accepted candidate.
The old `.handbook/project_context/PROJECT_CONTEXT.md` path is never read,
written, renamed, copied, or used as overwrite eligibility. The command grammar
and file/stdin modes remain fixed, while the old `0.1.0` input shape and dynamic
timestamp test hook are deleted.

The installed Project Context input template and the live-skill smoke fixture
switch to the exact canonical record. The command does not synthesize missing
fields, import old Markdown, or preserve the prior Markdown headings.

#### Platform boundary

HCM-2.1 chooses typed non-Unix refusal rather than weakening the no-follow and
atomicity contract. Validate-only parsing/rendering of supplied input remains
platform-independent and non-mutating. On `cfg(not(unix))`, non-validate Project
Context authoring returns a distinct `unsupported_platform_strict_mutation`
refusal before lock acquisition, directory creation, target inspection, temp
creation, or write; it must not call the existing path-based fallback writer.
The broken subject is the selected descriptor path and the next safe action is
to run the mutation on a supported Unix host. CLI maps it through the existing
author-refused outcome/category and exit class with exact human-text/stderr
golden proof; no author JSON surface is added or implied.

The selected strict loader likewise maps non-Unix unavailability to the
existing artifact status `unsafe_path` and reason
`unsupported_platform_strict_read`; doctor reports `INVALID` with a null
Project Context row, and flow blocks the required selected source with the same
reason. No non-Unix path-based fallback read is allowed. Cross-compilation is
required but is not runtime proof: closeout also requires a native Windows test
job covering validate-only success, pre-mutation author refusal/no filesystem
delta, doctor invalid/null, and flow refusal. If that job is unavailable, the
future implementation stops rather than claims platform proof.

### Environment Inventory reference-only cutover

Environment Inventory remains fixed Markdown authority and keeps its existing
`0.1.0` input/render/write behavior except for one dependency reference. Because
the shipped Project Context instance is always required, Environment Inventory
preflight now resolves the same shipped decisions and requires a successful
engine-owned selected Project Context observation. Its input field
`project_context_ref` is required to be exactly the descriptor-owned
`.handbook/project/context.yaml`; `null`, the retired Markdown path, any fixed
substitute, missing/invalid selected YAML, or a reference unequal to the
selected decision refuses before the Environment Inventory lock or write.

The Environment Inventory renderer emits exactly:

```text
> **Project Context Ref:** `.handbook/project/context.yaml`
```

This is a reference-only consumer change, not an Environment Inventory
canonical-content conversion. It may edit only the named engine/compiler
Environment Inventory input validation, preflight, renderer constant, focused
tests, input template, runtime-smoke fixture, and all-three smoke assertions.
It may not change Environment Inventory's canonical path, schema version,
headings, timestamp behavior, write policy, or flow source. Positive proof must
show selected canonical Project Context allows Environment Inventory authoring;
refusal proof must cover missing/invalid/unsafe selected YAML, null/legacy/
mismatched input refs, and no Environment Inventory mutation. The installed
all-three smoke must assert the selected YAML exists, legacy Project Context
Markdown does not exist, and Environment Inventory contains only the selected
YAML reference.

### Setup

HCM-1.4 setup already consumes the selected descriptor and performs structural
inspection through the bound schema. HCM-2.1 changes no setup production
decision unless a focused test proves a real mismatch. Its required
integration is evidence:

- missing `.handbook/project/context.yaml` is always `author_required`;
- valid canonical Project Context is `preserve`;
- malformed/unsafe/unreadable Project Context is `invalid`;
- `--rewrite` remains a typed no-materializer refusal; and
- setup never writes Project Context or legacy Markdown.

Any implementation need to make setup render, author, migrate, or infer
content is a scope stop.

### Doctor

Doctor report schema version becomes `1.1.0`. Existing HCM-1.4 fields retain
their meaning. One additive top-level field is required:

```text
project_context: null | {
  instance_id: "project_context",
  kind_ref: "handbook.artifact-kind.project-context@1.0.0",
  canonical_path: ".handbook/project/context.yaml",
  source_fingerprint: "sha256:<64 lowercase hex>",
  rendered_output_fingerprint: "sha256:<64 lowercase hex>",
  rendered_media_type: "text/markdown"
}
```

It is non-null only when the selected artifact inspection row is
`structurally_valid` / `present_and_structurally_valid` and contains the one
retained, final-stability-checked engine observation defined above. Doctor must
project that observation; it may not call a second loader. Otherwise it is null
and the exact existing-or-new artifact status/reason is the complete readiness
explanation. Typed-decode disagreement maps to `structurally_invalid` /
`typed_decode_failed`; renderer refusal maps to `structurally_invalid` /
`rendered_view_refused`; substitution or identical-byte inode ABA maps to
`unreadable` / `observation_changed_during_inspection`. Each yields overall
`INVALID` and no fingerprints. Human wording remains CLI-owned, names those
reasons, and prints fingerprints only for the non-null row.

Doctor does not claim semantic approval, currentness, lifecycle state,
freshness, or Projection provenance.

### Flow

The default and injected-contract `handbook-flow` resolver paths use one mixed-
family cutover adapter named `BR-HCM-2-PILOT-FLOW-01`:

- Charter, Environment Inventory content, and Feature Spec remain on their
  unchanged fixed pre-membrane source paths; Environment Inventory's embedded
  Project Context reference changes only as specified above;
- Project Context comes only from the exact selected descriptor, typed
  canonical loader, and in-memory fixed renderer;
- the legacy `CanonicalArtifactKind::ProjectContext` value may remain only as
  a packet output tag during the bridge; it cannot select membership, path,
  parser, validator, or renderer;
- the Project Context packet source path is an owned string from the selected
  descriptor and the exact DTO/domain table below applies;
- missing/invalid/unsafe Project Context blocks selection because the shipped
  descriptor is always required; and
- legacy Project Context Markdown bytes, custom legacy layout overrides, and
  pipeline-produced Project Context Markdown have no effect on the selected
  Project Context source, blocker, section, or fingerprints.

The bridge exists only because HCM-2.1 converts one family while sibling
families remain fixed. It is internal, not a user compatibility promise. Its
entry and deletion gate are added to `06`; HCM-2.4 must remove it when remaining
shipped families cut over. HCM-2.1 does not change `handbook-pipeline`, its
frozen external-consumer proof corpus, or historical stage outputs and makes no
claim that those sibling workflow artifacts are canonical Project Context
truth.

The bridge makes these exact public DTO changes:

- every `canonical_repo_relative_path` in `PacketSourceSummary`,
  `PacketSection`, `BudgetTarget`, flow `ResolverSubjectRef::CanonicalArtifact`,
  and compiler `SubjectRef::CanonicalArtifact` becomes `String`;
- every `canonical_repo_relative_path` in flow `BudgetNextSafeAction`, flow
  `ResolverNextSafeAction`, and compiler `NextSafeAction` becomes `String` for
  the exact variants `CreateSystemRoot`, `EnsureSystemRootIsDirectory`,
  `RemoveSystemRootSymlink`, `CreateCanonicalArtifact`,
  `FillCanonicalArtifact`, and `ReduceCanonicalArtifactSize`; policy IDs and
  `RunGenerate.packet_id` remain `&'static str`;
- `PacketSourceSummary` keeps `byte_len` and `content_sha256` as the exact source
  byte domain and adds nullable `rendered_output_byte_len`,
  `rendered_output_sha256`, and `rendered_media_type`;
- `PacketSectionMode` adds `Rendered`; and
- `PacketSection` adds nullable `source_content_sha256` and
  `rendered_output_sha256`.

The exact value/domain contract is:

| Surface | Project Context | Fixed siblings |
|---|---|---|
| manifest path/length/hash | selected YAML path, source byte length, source SHA-256 | unchanged fixed source values |
| freshness inputs | unchanged `reduced-v1-m8` encoding: selected path, presence, source SHA-256, and existing starter-match flag; source length and rendered values excluded | unchanged |
| source summary | source fields plus rendered length/hash and `text/markdown` | source fields; all rendered fields null |
| section | `mode=Rendered`, rendered Markdown contents, source and rendered hashes both non-null | existing mode/contents; source hash non-null, rendered hash null |
| fixture lineage | same source-summary rules as live selection | unchanged plus nullable fields |
| budget effective bytes | rendered Markdown length | source byte length |
| budget target | selected YAML path, rendered length, `byte_domain=rendered_output` | fixed path, source length, `byte_domain=source` |
| blocker/refusal subject | selected YAML path and selected inspection reason | unchanged |
| ordering | second in existing Charter, Project Context, Environment Inventory, Feature Spec order | unchanged |

`BudgetByteDomain { Source, RenderedOutput }` is added to each `BudgetTarget`;
per-artifact and total thresholds sum the effective byte column above. Budget
summary/exclusion notes name the selected YAML path and exact effective byte
length/domain. Tests must cover source-smaller/render-larger and source-larger/
render-smaller cases immediately below, at, and above both thresholds.

Ownership conversion is closed: fixed engine/layout `&'static str` paths call
`.to_owned()` once when entering a changed flow DTO; the selected descriptor
path is cloned or moved as an owned `String`; flow budget-to-resolver and flow-
to-compiler projections move or clone that `String`. No `Box::leak`, string
interning, hard-coded Project Context path adapter, ambient lookup, or lifetime
coercion is permitted. Exact refusal/blocker/output tests cover selected and
fixed sibling paths through every named carrier.

The Project Context author result is part of the same selected-path closure:
`AuthorProjectContextResult.canonical_repo_relative_path` becomes `String` and
is populated only by clone/move from the selected artifact decision. The legacy
public `CANONICAL_PROJECT_CONTEXT_REPO_PATH` constant and its author/module/
crate re-exports are removed; they are not renamed to the YAML path or retained
as compatibility aliases. Author core/shell, compiler public API, CLI text, and
file/stdin tests prove the owned selected path survives result projection and
that neither the removed constant nor legacy layout bytes select or affect it.

The upstream engine identity chain is also exact. Static
`CanonicalArtifactDescriptor` and `CanonicalLayout` path constants remain
unchanged fixed-source inputs for siblings. At identity construction, those
paths become owned once, while the selected Project Context decision contributes
its already owned/cloned descriptor path. The exact owned fields are:

- `CanonicalArtifactIdentity.relative_path: String`;
- `ArtifactIngestIssue.canonical_repo_relative_path: String`;
- `BaselineArtifactValidation.canonical_repo_relative_path: String`; and
- the existing `ArtifactManifest.artifacts` vector of owned identities.

`freshness.rs` receives borrowed `&str` views from those owned identities for
sorting and encoding. Its only permitted changes are ownership/borrowing
adaptations (`as_str()`/references) required by the type transition. The encoder
field order, values, length prefixes, sort key, issue ordering, SHA-256, schema
version, and generation remain byte-for-byte unchanged. Golden proof uses a
fixed-sibling manifest before/after ownership conversion and an exact selected-
path manifest to show independently reconstructed `reduced-v1-m8` preimages and
fingerprints are unchanged except for the intentionally different selected path
and source hash values.

C03 artifact and freshness log lines keep their existing format but contain the
selected Project Context source values. Immediately after the Project Context
C03 artifact line, emit exactly one additional line:

```text
hcm2.project_context rendered_byte_len=<decimal> rendered_sha256=sha256:<64-lowercase-hex> media_type=text/markdown
```

Neither source length nor any rendered value enters the C03 freshness hash.
`C03_SCHEMA_VERSION=reduced-v1-m8` and manifest generation `1` remain unchanged,
and `crates/engine/src/freshness.rs` receives only the ownership/borrowing
adaptation above. The packet decision summary count includes the added rendered
line.
Compiler JSON field order is append-only and exact:
source objects keep their six existing fields then append
`rendered_output_byte_len`, `rendered_output_sha256`,
`rendered_media_type`; section objects keep their five existing fields then
append `source_content_sha256`, `rendered_output_sha256`; budget targets keep
path and length then append `byte_domain` as `Source` or `RenderedOutput`.

Shared and CLI source-summary text remains byte-identical for siblings. For
Project Context its parenthetical is exactly:

```text
present, <source-decimal> source bytes, source_sha256=sha256:<64-lowercase-hex>, <rendered-decimal> rendered bytes, rendered_sha256=sha256:<64-lowercase-hex>, media_type=text/markdown
```

A rendered Project Context section emits `MODE: rendered from selected
canonical YAML`, then `SOURCE SHA256: <value>`, then `RENDERED SHA256: <value>`
between its heading and content fence. Budget target human/inspect text appends
`[source]` or `[rendered_output]` after the decimal byte count. Compiler JSON,
shared/inspect rendering, and CLI rendering may not drop, relabel, or recompute
either fingerprint. Golden proof freezes manifest, freshness, included-source,
fixture-lineage, section, budget, notes, blocker, decision-log, compiler JSON/
shared/inspect, and CLI bytes for valid selected truth and for conflicting
legacy Markdown/layout inputs.

Because the enclosing C04 public DTO and serialized output shape change, both
flow and compiler `C04_RESULT_VERSION` advance exactly from `reduced-v1-m8.1`
to `reduced-v1-m8.2`. Compiler rendering accepts only `reduced-v1-m8.2` and
rejects an otherwise valid `reduced-v1-m8.1` result as
`UnsupportedResultVersion`; flow construction, compiler constant, shared/JSON/
inspect output goldens, CLI goldens, and fixed-sibling regressions all assert
the new exact value. This is a C04 result-envelope transition only; it does not
change the frozen C03 schema/generation.

## Required skill chain

The future implementation parent must apply, in order and with durable proof:

1. `using-agent-skills`;
2. `context-engineering`;
3. `source-driven-development`;
4. `spec-driven-development`;
5. `planning-and-task-breakdown`;
6. `api-and-interface-design`;
7. `security-and-hardening`;
8. `incremental-implementation`;
9. `test-driven-development`;
10. `debugging-and-error-recovery` when any check fails;
11. `documentation-and-adrs`;
12. `code-review-and-quality`; and
13. `git-workflow-and-versioning`.

The parent must reread the live skill files rather than rely on this summary.

## Live baseline and blast radius

Planning refreshed GitNexus with `npx gitnexus analyze --index-only`. The
incremental index completed successfully at the planning entry. Free-form
query remained degraded because the FTS index was unavailable, so planning
used graph context, live source/tests, and Git history without claiming
semantic-search completeness.

Planning-time upstream impact results were:

| Surface | Risk | Observed blast radius | HCM-2.1 disposition |
|---|---:|---:|---|
| `ProjectContextStructuredInput` | LOW | 14 impacted; 2 direct; author/tests | retire only through direct Project Context cutover and exact export/test cleanup |
| engine `render_project_context_markdown` | LOW | 2 direct callers | replace input and output contract; delete time dependence and golden-test exact bytes |
| compiler `author_project_context_from_input` | MEDIUM | 6 impacted; 5 direct; CLI author flow | change test-first; keep command grammar, lock, and safe-write boundaries |
| compiler `run_setup` | LOW | 2 direct; setup CLI flow | expect no production edit; prove selected YAML behavior through tests |
| compiler `doctor` | LOW | 2 direct; doctor CLI flow | additive `1.1.0` report only; preserve HCM-1.4 decision fields |
| flow `resolve_with_contract` | MEDIUM | 5 direct; flow/compiler tests | install the bounded mixed-family bridge and preserve non-Project-Context behavior |

The future implementation must rerun upstream impact for every existing
function, type, or method it edits, report any HIGH/CRITICAL result before the
edit, and stop if the required mitigation widens outside this packet.

## Exact allowed scope

### Engine

The future implementation may change:

- `crates/engine/src/lib.rs` and `crates/engine/src/author/mod.rs`, only for the
  Project Context export cutover;
- `crates/engine/src/author/project_context_core.rs`, by replacing the old rich
  input/timestamped renderer surface with the exact canonical record,
  parse/serialize/render/fingerprint contract above, or deleting it after the
  same functionality moves to one new
  `crates/engine/src/project_context_artifact.rs` owner module;
- `crates/engine/src/canonical_artifacts.rs`, `artifact_manifest.rs`,
  `baseline_validation.rs`, and `canonical_paths.rs` only as required to let
  flow carry the selected Project Context path/representation without changing
  fixed sibling behavior beyond the enumerated Environment Inventory reference;
  and
- `crates/engine/src/profile_inspection.rs` for the one retained Project
  Context observation plus the three exact status reasons;
- `crates/engine/src/author/environment_inventory_core.rs` only for the exact
  reference-only selected YAML path validation/render cutover; and
- `crates/engine/src/canonical_repo_support.rs` only if the selected loader
  cannot use its existing bounded no-follow primitives and file-identity
  stability check unchanged.
- `crates/engine/src/freshness.rs` only for the exact ownership/borrowing
  adaptation above; its encoder bytes, constants, sort/order, and semantics may
  not change.

No definition asset, schema JSON, schema entry, kind definition, profile,
descriptor, stable-role registry, condition definition, semantic-capability
definition, Cargo manifest, or lockfile may change.

### Compiler and CLI

The future implementation may change only the Project Context portions of:

- `crates/compiler/src/author/project_context.rs`;
- `crates/compiler/src/author/project_context_core.rs`;
- `crates/compiler/src/author/project_context_shell.rs`;
- `crates/compiler/src/author/environment_inventory.rs` and
  `environment_inventory_shell.rs`, only for selected Project Context
  observation preflight and exact reference equality;
- `crates/compiler/src/author/mod.rs` and `crates/compiler/src/lib.rs`;
- `crates/compiler/src/doctor.rs` and `profile_readiness.rs`;
- `crates/compiler/src/blocker.rs` only for `C04_RESULT_VERSION` transition and
  the compiler-owned path carriers reached through its public blocker type;
- `crates/compiler/src/refusal.rs` only for the exact owned path fields in
  `SubjectRef` and `NextSafeAction`;
- `crates/compiler/src/baseline_validation.rs`, `canonical_artifacts.rs`,
  `layout.rs`, and `resolver.rs` only when compilation or the selected flow
  projection requires the engine-owned cutover;
- `crates/compiler/src/rendering/shared.rs`, `rendering/json.rs`, and
  `rendering/inspect.rs` only to project the exact bridge DTO additions and C04
  result version; and
- `crates/cli/src/author.rs`, `doctor.rs`, `doctor_rendering.rs`, `rendering.rs`,
  `main.rs`, and the reason-name-only `setup.rs` change defined below.

`crates/compiler/src/repo_file_access.rs` is not an allowed production edit;
Project Context must guard before its non-Unix fallback writer as specified.

`crates/compiler/src/setup.rs` remains a test subject, not a planned production
edit. `crates/cli/src/setup.rs` may change only its exhaustive reason-name match
to add exact snake-case strings `typed_decode_failed`, `rendered_view_refused`,
and `observation_changed_during_inspection`; all other setup rendering, command,
decision, output, and exit behavior remains byte-for-byte unchanged. Any other
setup production need is allowed only if focused RED proof shows current HCM-1.4
behavior violates the exact setup contract and fresh impact remains in slice.

### Flow, templates, tests, and proof

The future implementation may change:

- `crates/flow/src/resolver.rs`, `packet_result.rs`, `budget.rs`, and `lib.rs`
  for the exact selected Project Context bridge, byte domains, owned paths, and
  fingerprint projection;
- Project Context-focused tests under `crates/engine/tests/`,
  `crates/compiler/tests/`, `crates/flow/tests/`, and `crates/cli/tests/`;
- existing compiler/CLI rendering or resolver tests only where their public
  Project Context expectation directly changes;
- `core/library/project_context/PROJECT_CONTEXT_INPUTS.yaml.tmpl`;
- `core/library/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl`,
  only for its exact Project Context reference;
- `tools/fixtures/project_context_inputs/runtime_smoke_valid.yaml`;
- `tools/fixtures/environment_inventory_inputs/runtime_smoke_valid.yaml`, only
  for its exact Project Context reference;
- `tools/ci/codex-skill-live-smoke.sh`, solely for Project Context canonical-
  path/content/fingerprint and Environment Inventory reference assertions; and
- HCM-2.1 proof, internal dispatches, affected control-pack rows, and one
  parent closeout under the existing orchestration paths.

The implementation must not change Charter authoring, Environment Inventory
content authority beyond the enumerated reference-only cutover,
`core/pipelines/**`, `core/stages/**`, `crates/pipeline/**`, external pipeline
consumer fixtures, SDK/Tauri/Substrate/adapters, contract/dock code, installed
generated projections, HCM-2.2+, or unrelated cleanup. A discovered need
outside the list is a scope stop requiring a separately reviewed packet repair.

## Test-first proof contract

Before production edits, add RED proof for:

1. exact canonical Project Context positive parse and closed typed decode;
2. duplicate keys at top-level and nested values, multiple documents,
   non-object roots, unknown/missing fields, wrong constants, wrong types,
   schema bounds, invalid stable refs, and over-limit bytes;
3. the literal boundary-rich closed-emitter YAML fixture, every empty/non-empty
   list shape, JSON string escaping, and exact one-LF output;
4. the literal Markdown fixture, every row of the exhaustive transform table,
   renderer-owned structure only, exact one-LF, no clock/environment input, and
   distinct source/render fingerprints;
5. descriptor/kind/schema/path mismatch refusal and no arbitrary-path API;
6. no-follow missing/symlink/non-regular/oversize/read-error behavior, retained-
   observation reuse, typed-decode/render reasons, different-byte substitution,
   and identical-byte inode-replacement ABA invalidation;
7. owned engine identity/ingest/baseline/manifest paths with independently
   reconstructed byte-identical C03 preimages before/after ownership adaptation;
8. validate-only file/stdin non-mutation;
9. author file/stdin success, exact atomic YAML write, existing valid refusal,
   invalid regular-file repair, lock behavior, owned selected result path,
   removed legacy path constant/re-exports, and no legacy Markdown access;
10. setup missing/valid/invalid/rewrite/no-write behavior for the selected path
    plus exact CLI names for only the three new inspection reasons;
11. doctor `1.1.0` null/non-null Project Context row, exact fingerprints, the
    three exact failure reasons, observation/ABA invalidation, stable HCM-1.4
    fields, JSON/text output, and exit mapping;
12. Environment Inventory positive selected-reference authoring plus missing,
    invalid, unsafe, null, retired-path, mismatched-path, and zero-mutation
    refusal tests without changing any other Environment Inventory byte;
13. flow requiredness, exact DTO fields/modes, selected path, rendered section,
    both fingerprints, source-versus-rendered budget thresholds, manifest/
    freshness/log/fixture bytes, legacy-Markdown and layout irrelevance, and
    unchanged sibling source behavior; full owned-path compile/API/output proof
    through budget/resolver/compiler/CLI; and C04 `.2` acceptance plus `.1`
    rejection;
14. native non-Unix validate-only success, pre-mutation author refusal/no
    filesystem delta, doctor invalid/null, and flow refusal plus Windows-target
    compilation; and
15. end-to-end installed all-three smoke from canonical input template through
    Project Context author, Environment Inventory selected YAML reference,
    doctor JSON, and flow-visible Project Context truth with no legacy Markdown.

The implementation must also replay all HCM-1.1 through HCM-1.4 engine,
compiler setup/doctor, complete author, flow resolver, CLI author/doctor/setup,
workspace, Clippy, package-definition, Windows-target, install-smoke,
handoff-validator, scope, secret, and whitespace baselines affected by the
changed files. Platform proof that cannot run locally is a genuine stop when
the packet's exact owner code is platform-sensitive; cross-compilation is not
runtime proof.

## Classification and gate ceiling

The future implementation may promote only:

- `Canonical artifact identities`, `Structured authoring`, `Markdown
  validation`, and `Flow packet assembly` to `BoundaryLanded` for the exact
  shipped `project_context` pilot path;
- `PG-YAML-01` to closed for one exact structurally validated and
  deterministically rendered family;
- the Project Context subset of `PG-ARTIFACT-01` through selected content-
  authority participation in setup, doctor, and flow; and
- the product-path subset of `PG-YAML-02` proven by author/setup/doctor/flow
  bypass of legacy Project Context Markdown.

`PG-ARTIFACT-01` and `PG-YAML-02` remain open program-wide while fixed sibling
and frozen pipeline surfaces remain. `PG-KIND-01` remains open for semantic
validation, supplied intake, lifecycle, and Projection coverage. HCM-2.1 does
not promote intake, semantic approval, lifecycle, freshness, condition
evaluation, vocabulary, Resolution, capitalized Projection, SDK/transport,
publication, downstream, contract, or dock proof.

## Explicit non-goals

- changing the approved six-kind catalog or three-instance root profile;
- changing the exact Project Context schema, kind, descriptor, path, role, or
  requiredness;
- preserving the pre-membrane `0.1.0` Project Context input shape or Markdown;
- user migration/import/alias tooling or Markdown/YAML dual-read/write;
- persisted Markdown output;
- a generic renderer-definition schema or custom-kind renderer engine;
- semantic validators, lifecycle policies, review triggers, intake records,
  candidates, approval/promotion, or Project Authority work;
- materializing content during setup or supporting `--rewrite`;
- condition evidence/evaluation;
- capitalized Projection or Context Resolution input/provenance;
- pipeline/stage/output migration;
- SDK, Tauri, Substrate, adapters, contracts, docks, publishing, or releases;
- HCM-2.2, HCM-2.3, HCM-2.4, or adjacent cleanup.

## Exit gate

The future implementation may close only when:

1. every exact contract, path, API, refusal, serialization, rendering,
   fingerprint, and bridge rule in this packet is implemented without schema or
   descriptor drift;
2. targeted positive, negative, no-follow, retained-observation/ABA, direct-
   cutover, deterministic, setup, doctor, Environment Inventory reference,
   flow DTO/owned-path/C04-version/budget/log, CLI, native non-Unix, skill-smoke,
   and package proof passes;
3. all applicable HCM-1 and frozen pipeline/workspace regressions pass;
4. the literal 29-member package-definition tree remains exact by set, size,
   SHA-256, and bytes in the repository and engine package;
5. no legacy Project Context Markdown byte can influence Project Context or
   Environment Inventory authoring, setup, doctor, or flow; Environment
   Inventory emits only the selected YAML reference; and no persistent derived
   Project Context Markdown file is written;
6. `BR-HCM-2-PILOT-FLOW-01` is recorded with its bounded lifetime and deletion
   proof while no other compatibility bridge is introduced;
7. affected `00`, `03`, `04`, and `06` rows report only evidence-supported
   Project Context promotion and preserve every sibling gate;
8. formatting, Clippy, tests, docs, package, Windows-target plus the required
   native Windows runtime job, handoff validation, `git diff --check`, exact
   path/scope, secret, whitespace, and repository-required GitNexus detection
   pass;
9. one fresh isolated read-only built-in `default` reviewer returns `CLEAN`
   over the exact complete implementation subject and proof wall; every valid
   finding receives bounded remediation, full proof replay, a new immutable
   dispatch, and a different fresh reviewer; and
10. the clean implementation subject is committed first, followed only by one
    separate parent-owned completed v1.2 handoff and deterministic ledger
    closeout commit. HCM-2.2 is not started.

## Stop conditions

Stop instead of improvising when:

- the exact HCM-1.2 Project Context schema cannot support the pilot without a
  schema/kind/profile/descriptor version change;
- the selected path cannot drive authoring and flow without an unreviewed fixed
  path or legacy translation;
- setup must infer or materialize content;
- a persistent Markdown file or two-file transaction becomes necessary;
- the flow bridge cannot isolate Project Context from frozen pipeline/sibling
  behavior;
- required proof needs pipeline, HCM-2.2+, SDK, transport, publication, or
  downstream scope;
- a HIGH/CRITICAL impact cannot be mitigated inside the exact allowed files;
- required platform/runtime proof is unavailable; or
- fresh built-in independent review cannot complete.

The stop must record the exact missing authority or proof and preserve safe
reviewed work without claiming HCM-2.1 implementation completion.
