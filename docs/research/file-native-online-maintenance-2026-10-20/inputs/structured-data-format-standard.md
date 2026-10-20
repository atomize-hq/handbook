# Structured data formats: JSON, YAML, and human views

Status: **proposed standard for human review**, 2026-10-02. This document does
not authorize product changes or convert existing files. The user's explicit
selection that Contracts remain JSON is fixed input; the general defaults below
are proposed for adoption. The [audit](../../research/json-yaml-format-audit-2026-10-02.md)
records historical intent, current behavior, reusable tools, and verification.

## Decision in ordinary English

Choose a format for a particular object and interface, not from the word
"configuration", "policy", or "state" alone. Keep one authoritative structured
meaning under its semantic owner, and derive human views from it.

- Use readable YAML for structured documents whose supported workflow includes
  direct human authoring, especially the six authored project-artifact families.
- Use JSON for owner-managed configuration/state, activation history, receipts,
  protocol messages, and schemas when the supported interface is an operation
  rather than direct file authoring.
- Keep Contracts JSON, including their established exact identities and stored
  representation. The first production Dock integration remains native Rust
  JSON Schema validation over exact inline JSON and an offline local schema closure.
- Supply Markdown or other human views where the workflow needs them. A human
  view does not become a second editable authority or change the source format.

These are defaults plus explicit selections, not a retrospective requirement to
convert every YAML state file to JSON or every JSON policy to YAML. Existing
owner-specific contracts remain controlling until a bounded change selects otherwise.

## Format selection

Apply the first applicable rule in this table. For a new boundary, record the
selection and its reason in that boundary's ordinary design/authority document;
do not create a new per-object registry or approval system.

| Object or interface | Selection | Boundary |
| --- | --- | --- |
| Existing versioned owner, protocol, or file contract | Preserve its selected format and identity rules | Re-encoding is a deliberate contract change, not formatting cleanup. |
| Contracts definitions and established Contracts policy sources | JSON | Preserve the user's Contracts JSON choice and current domain-specific policy selections; this does not decide every future policy domain. |
| JSON Schema documents, current Dock request/result protocol, selected JSON Schema adapter input | JSON | Schema validation is not policy admission or proof of authority. The JSON adapter does not require every upstream authored artifact to be stored as JSON. |
| The six authored artifact families: Charter, Project Context, Environment Context, Work Specification, Decision Record, Risk Record | Canonical YAML, with derived human views where selected | Preserve the native authoring/admission/promotion lifecycle. A file format alone does not authorize direct edits to activated truth. |
| New directly authored structured domain document, including a policy outside an already selected JSON contract | Prefer readable YAML | Its semantic owner must select the schema, vocabulary, admission lifecycle, supported edit/import route, and canonical stored representation. It does not become a seventh generic artifact family. |
| New machine-managed project configuration, selection state, owner result, receipt, journal, index or cache | Prefer JSON | A CLI/SDK/GUI operation may accept parameters and produce human views. No YAML authoring surface is implied. Existing YAML state remains a documented exception. |
| New user-authored configuration or declarative definition | Prefer readable YAML unless an existing format contract controls | "Configuration" is not sufficient to select JSON: authoring profile/definition content differs from recording which profile is selected. |
| Human explanation, review, or assembled document | Markdown, text, HTML, or GUI projection as selected | Preserve exact source ownership/identity and any required omissions/disclosure meaning. No editing the view back into authority implicitly. |

Before choosing a format, answer: who authors this content, through which
supported interface, which owner admits or activates it, what exact identity is
retained, and what human view is needed? Those answers are part of the bounded
feature design, not a universal public API requirement.

## Separate five responsibilities

1. **Authoring input:** a proposed YAML or JSON document, or typed operation
   parameters. Readability can matter here; syntactic validity supplies no authority.
2. **Owner-established source:** the exact admitted definition or retained record,
   with one controlling lifecycle and a declared physical representation.
3. **Identity canonicalization:** the exact normalization/preimage used for a
   fingerprint. It can be canonical JSON even when the source is YAML.
4. **Transport:** DTO, JSON message and framing carrying an object. Framing and
   correlation do not define the source object's semantic identity.
5. **Human projection:** a view derived from the source. Editing it neither alters
   the source nor activates a new configuration.

An owner may explicitly support YAML candidate input with JSON activated records.
It must preserve the exact semantic correspondence and required provenance. Do
not create independently editable YAML and JSON masters. There is no generic
bidirectional synchronization promise, comment-preserving editor, or round-trip
through Markdown. A canonical YAML source may itself be owner-managed after
promotion; readability and permission to edit are different properties.

## Parsing, canonicalization, and persistence

Use the existing [tool inventory](../../research/json-yaml-format-audit-2026-10-02.md#reusable-tooling)
before adding serialization infrastructure. Reuse must preserve dependency
direction, public/private visibility, bounds, number rules, error behavior and
owner lifecycle. In particular, Engine helpers may be reused inward of SDK;
Contracts must not depend on SDK simply to reuse its serializer. The standalone
Dock validator retains its intentionally separate dependency boundary.

For new structured boundaries:

- Select an explicit JSON-compatible data model when interoperating between YAML,
  JSON and JSON Schema. Use string mapping keys and finite representable values;
  enforce the owner's numeric and schema constraints separately. Do not silently
  truncate, coerce, or discard fields to cross a boundary.
- Require one complete document, reject duplicate keys before typed decoding,
  enforce the selected resource limits, and validate the owner's closed shape.
  Do not assume every existing YAML/JSON caller already supplies these properties.
- Use deterministic block YAML for a selected canonical-YAML output, with
  parse/emit/parse equality and the owner's declared ordering. Comments, original
  key order, scalar style and formatting are not preserved by the current generic
  emitter. If they matter to authoring, preserve that input separately and define
  its role; do not claim it is the activated record automatically.
- Do not equate a `.yaml` suffix with human-readable block YAML. JSON-shaped
  content in a YAML file is syntactically supported in current assets; converting
  its presentation is separate, bounded work.
- Name the exact byte/semantic domain of every fingerprint. Current artifact
  publication hashes deterministic YAML bytes; many definition identities hash
  canonical JSON values. Preserve both where their contracts require them.
  New formatting must not silently replace old identities or historical bytes.
- Preserve each boundary's framing: some canonical JSON records have no trailing
  newline, other records add one LF, and Dock frames require one line. Do not
  choose a universal pretty-printer or newline rule for all JSON.
- Reuse retained-root file and atomic-write mechanisms where applicable. Ordinary
  files are the selected starting storage; file format does not replace validation,
  conflict detection, exact replay, or truthful commit/recovery. A new database,
  login platform, tamper-proof store or broader hardening program is not selected.

YAML parser acceptance does not establish portable semantics for YAML tags,
aliases, merge keys, streams or special scalars. Each supported input must either
specify and verify its normalization or reject unsupported features before owner
admission. No universal rejection capability is claimed for the current helpers.
JSON Schema can validate a YAML-derived data value; this does not grant the current
JSON-only Dock a YAML file reader or a new adapter interface.

## Existing exceptions and incomplete work

The audit identifies deliberate Pipeline YAML state/caches, JSON-shaped Engine
definition assets with `.yaml` extensions, current JSON Contracts policies,
and older Python tooling with different parsing guarantees. They are evidence to
classify, not automatic migration tasks. Format conformity does not repair the
separate Context Resolution generic-artifact lifecycle mismatch.

Human-facing projection is a product direction, not a claim that a renderer exists
for every owner object today. General YAML merge/update/reconstruction and
comment-preserving authoring are not proven complete by the existing parser/emitter.
Only selected domain operations supply those semantics today.

New work should check parsing refusals, deterministic output, source-to-view
fidelity, exact identities, and the actual consumer route proportionately. Where
content or activation is immutable, it must also check conflicting identities,
replay and interrupted writes. No full-workspace suite is implied for a format-only
documentation decision, and no universal JSON/YAML conversion test replaces owner tests.

## Effect on the pending governance plan

The [establishment plan](slices/admission-governance-establishment/README.md) now
records human authorization for Packet 1 after a clean causal review of its bounded
planning reconciliation. Its selected typed SDK setup path with owner-managed JSON
configuration/activation records fits the proposed machine-managed default.
That slice-specific authorization does not adopt this standard's other proposed
defaults. Human projections remain compatible future work.

If direct YAML authoring of a governance candidate is wanted, select that input
and its validation/normalization in a bounded plan revision; do not change the
activated storage format solely because the object is also a semantic policy.
No file importer or renderer is added by adopting a format convention.

The 24-hour ceiling discussed with the human is an example/test value for a
required caller-supplied parameter, with no implicit default. The plan now carries
that clarification and its verification requirements; no authoring/projection
expansion is selected. This standard neither activates governance nor
authorizes successor planning, migration, or product execution.
