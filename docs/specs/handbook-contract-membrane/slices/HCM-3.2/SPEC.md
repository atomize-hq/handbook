# HCM-3.2 Context Resolution kernel — quarantine identity-ordering finalization

Status: the recovered implementation's prior proof wall was GREEN, but fresh
finding `HCM32-QF-IMPL-DISC-001` is evidence-disputed and not self-closed. Its
dispatched mixed valid-plus-invalid matching state was not shown reachable past
generic lineage, so no production repair was made. The already-authorized
different-fresh closure must independently adjudicate whether that invariant
evidence falsifies the claimed reachable P2. `CLEAN` closes the finding;
`FINDINGS` or `BLOCKED` leaves HCM-3.2 incomplete and only then requires a
classifier-reachable RED or new operator authority.

## Fresh-parent authority

- phase: `HCM-3`
- slice: `HCM-3.2`
- parent orchestration:
  `20260805T101500Z--HCM-3-2--context-resolution-quarantine-finalization`
- integrated outcome:
  `hcm-3.2-context-resolution-quarantine-finalization`
- causal budget:
  `sha256:a50c9dbfec9d454fe4300a5e88ba20cc76e839ed7c85c90b2e92768a6c03a237`
- bound operator prompt:
  `sha256:c4966d679e5bca3e01058c5b4ecbf04bb607db8ab0297ac4088f646b527a5db0`
- checkpoint: `eb4b7ba55c6f3db40799e67275f3bbbb8610c87a`
- checkpoint tree: `d51327c442190edcf1bf56423db54fd2522c5e6f`
- publication: local-only expected-old compare-and-swap; never push

This is an explicitly authorized fresh HCM-3.2 parent. It does not continue,
supersede, rename, or reset a stopped HCM-3.2 parent. The HCM-3.1 completion,
prior HCM-3.2 handoffs, findings, selectors, dispatches, proof, and protected
33-path WIP are immutable source evidence only. The additive authority is
`decision/2026-08-05-context-resolution-quarantine-finalization-selector.md`.

## Objective

Complete the engine-owned Context Resolution kernel with:

1. one configurable ordered stack and six independent Resolution dimensions;
2. deterministic root/child envelopes, monotonic narrowing, deny-on-overlap,
   and indeterminate refusal;
3. typed mutation, memory, validation, escalation, disposition, and semantic
   memory boundaries;
4. one exceptional private HCM-3.2 JCS capsule admitted only through the real
   generic intake/candidate/promotion/committed-lineage owner path;
5. authenticated publisher authority, committed provenance, and fresh
   authority revalidation at every operational consumption;
6. replacement history and crash recovery meeting the minimum four-part
   recovery floor; and
7. focused and proportional regression proof followed by independent review,
   two-commit v1.4 closeout, and local-only publication; and
8. identity-before-mutation refusal when a valid anchored quarantine record
   for candidate A is evaluated with a different otherwise-valid candidate B.

`PG-RES-01` may close only for this bounded kernel. HCM-3.3 Projection,
HCM-3.4 Snapshot Memory, HCM-3.5 adoption, HCM-3.6 posture, Phase 3 exit,
release, and remote publication remain open.

## Selected authority artifact

The compatibility tuple is exact:

| Domain | Exact identity |
|---|---|
| artifact kind | `handbook.artifact-kind.context-resolution-authority-binding@1.0.0` |
| artifact instance | `context_resolution_authority` |
| canonical path | `.handbook/project/context-resolution-authority.yaml` |
| wrapper schema | `handbook.schemas.context-resolution-authority-capsule@1.0.0` |
| intake definition | `handbook.intake.context-resolution-authority-capsule@1.0.0` |
| wrapper member | `/declaration_jcs` |
| declaration schema | `handbook.context-resolution-authority-declaration@1.0` |
| codec | `handbook.hcm-3.2.context-resolution-authority-binding-jcs@1.0` |
| payload schema | `handbook.context-resolution-authority-binding@1.0` |

The generic owner sees only the closed object
`{"declaration_jcs": <string>}`. The string content is the byte-exact RFC
8785/JCS serialization of one closed nine-member declaration. The declaration
contains an exact JCS string for the complete 53-leaf binding. Generic
validation establishes wrapper shape and `/declaration_jcs` provenance only;
it never represents the declaration or binding as semantically valid.

The HCM product gate alone parses and validates the declaration and binding.
It is mandatory before candidate eligibility, authoritative persistence,
promotion, pending or installed recovery, current reads, resolver admission,
and every authority consumption. Direct seeding, generic
`eligible_without_approval`, an ungated committed read, cache reuse, or clone
reuse never creates Context Resolution authority.

## Canonical and fingerprint contract

The selector freezes the exact nine declaration members, literal-null root
predecessor quartet, 53 JSON-pointer payload manifest, layer-specific limits,
and six non-substitutable fingerprint domains. Generic domains preserve their
existing algorithms: canonical JSON of the terminal value, canonical JSON of
the normalized wrapper, and exact canonical YAML artifact bytes. Only the
three HCM-owned declaration/payload/semantic domains add literal tags and
unsigned 64-bit big-endian lengths. Both declaration and payload parse
duplicate-safely as strict UTF-8/I-JSON, re-emit byte-identically as JCS, and
refuse BOM, trailing bytes, alternate encodings, unknown versions, extra or
missing members, mixed-null predecessor values, and bound excess before
expensive semantic work. The two nested carrier strings have explicit 96 KiB
and 80 KiB decoded UTF-8 exceptions; every other string uses 8 KiB. Depth,
members, and decoded escape attribution are exact in the selector.

Generic lineage supplies atomic wrapper authorship and replacement only. It
does not supply inner-field partial updates, queries, provenance, or schema
completion. Authorship and replacement are atomic over the whole binding,
while all 53 leaves still receive complete HCM validation. A second artifact
family, consumer, public interchange use, or generic atomic-document feature
requires new product authority.

## Authority and publication proof

The declaration embeds the independently committed prior approver-registry
quartet. An external, closed nine-member `publisher_authority` object is
accepted only on the HCM promotion request and retained in the generic
promotion intent. It binds exact result-state and transition ref/fingerprint
pairs, publisher credential hash, padded RFC 4648 base64 challenge JCS, and
padded raw assertion response. It is forbidden for every other kind. The
publisher update must be the authentic direct `update_mapping` successor of
the embedded prior head, signed by the same active administrator/publisher
credential. That credential must already cover registry administration and
Context Resolution publication ownership; the result preserves every prior
unrelated mapping/state field, advances only that credential's exact existing
authenticated-use counter/sequence/head fields, and adds only
`context_resolution_authority_publication` /
`context-resolution-authority/<outer64hex>`. The retained transition,
challenge, credential, and mapping cardinality all reverify. Response proof is
the exact existing two-hop chain: transition authorization pair -> retained
assertion -> retained decoded-response pair; the publisher raw response equals
the response record's strictly decoded and canonically re-encoded raw bytes.

Construction is acyclic:

1. validate committed prior registry H0;
2. build exact payload, declaration, wrapper, and outer bytes;
3. compute all fingerprint domains and durably commit the semantically gated,
   non-authoritative generic candidate;
4. commit authenticated H0-to-H1 registry publication authority; and
5. commit generic promotion T1 carrying the external H1 witness.

Replacement repeats H1-to-H2 and T2. The outer bytes contain only the prior
registry quartet, never the resulting registry head, assertion, or mapping.

## Private proof states and currentness

The implementation must keep these private and non-interchangeable:

- candidate-validated capsule: non-capability;
- publication-authorized pending commit: non-capability;
- historical committed proof: recovery/audit only, non-cloneable and unable to
  construct or satisfy an admission;
- current authority witness: operational only while the exact current
  registry/publisher/repository/profile/stack/predecessor/lineage closure
  remains live; and
- corrupt, ambiguous, or quarantined publication: non-capability.

There is no caller-selected historical/current mode flag. Phase derives from
the committed journal and exact installed state. Resolver construction,
cache hit, admission, clone consumption, envelope resolution, mutation,
memory, validation, escalation, transition, and semantic-memory boundaries
revalidate current authority. Head advance, revocation, mapping loss, restored
old bytes, predecessor mismatch, repository/profile/stack drift, or broken
lineage makes old operational values unusable.

## Replacement and recovery floor

Replacement support is complete only when all four properties hold:

1. a superseded promotion remains historically verifiable from its retained
   exact displaced bytes and one unique committed direct successor; only the
   latest committed canonical output can be operational;
2. every crash boundary, including canonical installation before the durable
   publication result or commit marker, converges by exact recovery or enters
   an explicit bounded fail-closed quarantine; installed-but-uncommitted bytes
   never satisfy a current read;
3. staged and installed work remains a non-capability
   `PublicationAuthorizedPendingCommit` equivalent until the generic journal
   is durably committed; historical/current proof cannot be borrowed; and
4. the HCM promotion key is exactly `hcm32crpub_<outer64hex>`; the committed
   candidate and H2 registry witness are durable before the predecessor is
   disabled; HCM entry acquires promotion/registry locks before the generic
   lineage lock; and irrecoverable work receives an exact durable quarantine
   diagnostic rather than stopping unrelated recovery.

The availability trade-off is deliberate. After H2 authorizes O2 and before
T2 commits, neither O1 nor O2 is operational authority. O1 remains authentic
history only and never reactivates. Exact retry may finish T2; incompatible or
irrecoverable state quarantines fail closed.

Cold recovery of an H2-to-T2 no-journal crash is registry-head-driven. The
exact H2 operation ID and publication mapping identify the outer fingerprint;
the already committed candidate is found only by replaying its exact HCM
kind/instance, closures, normalized wrapper, canonical YAML, and predecessor.
One candidate permits the existing `promote` retry. Missing, multiple, invalid,
or conflicting candidates create one bounded transition-keyed quarantine
record under `.handbook/state/context-resolution-authority/quarantine/`.
Resolver load without a supplied retry still discovers and diagnoses the gap;
non-HCM recovery remains available. The selector freezes record schema,
phases/reasons, cardinality, lock order, exact retry, and committed resolution.

## Existing kernel behavior preserved

The shipped stack identity and HCM-3.1 vocabulary/profile closure remain
byte-identical. One stack has one exact fingerprint, one linear order, six
complete non-empty ranked domains, and adjacent defaults that do not widen
toward narrower levels. Envelopes materialize all dimensions, use at most one
parent, and children preserve or narrow each rank and mutation authority.
Unknown dimensions, malformed selectors, stale authority, overlap that is not
deterministically deny, and indeterminate matching refuse.

Escalation requests and dispositions, and memory-promotion requests and
dispositions, are separate immutable append-only records with exact
cardinality, authority, evidence, compare-and-write, and target-horizon rules.
They never self-authorize or promote Projection, Snapshot, artifact, contract,
posture, or gate truth.

## Production and test boundary

The absolute production ceiling is exactly:

- `crates/engine/src/artifact_lineage_store.rs`;
- `crates/engine/src/artifact_mutation.rs` only for retained recovered work;
- `crates/engine/src/context_resolution.rs` only for retained recovered work.

The new identity-ordering repair remains in `artifact_lineage_store.rs` unless
fresh impact analysis proves otherwise. No fourth production path is
permitted.

Test edits are limited to:

- `crates/engine/tests/context_resolution_kernel.rs`;
- `crates/engine/tests/hcm_2_3_generic_lineage.rs`; and
- `crates/engine/tests/fixtures/hcm_3_2_context_resolution/**`.

The selector freezes exact existing/new symbol, path, declaration, line, and
test ceilings. No public API, dependency, crate, unsafe/native/platform or
transport surface, generic framework, shipped identity mutation, or registry
semantic change is authorized.

## Required proof

Focused RED/GREEN must cover:

- the ordered stack, six independent dimensions, deterministic fingerprints,
  root/child narrowing, deny overlap, indeterminate refusal, memory,
  validation, escalation, terminal cardinality, and semantic-memory limits;
- empty-target intake through candidate, authenticated publisher update,
  promotion, committed recovery, resolver admission, and O1-to-O2-to-O3 cold
  recovery without direct canonical seeding;
- exact wrapper/declaration/payload canonicalization, 53-pointer validation,
  all fingerprint substitution pairs, unknown tuple/version, duplicate keys,
  mixed-null predecessor, golden generic/HCM fingerprint vectors, equality to
  committed generic records, and limit-1/limit/limit+1 plus jointly satisfiable
  size/depth/member/string/escape bounds;
- direct seed, forged/missing/ambiguous journal or publisher, cross-repository
  replay, wrong predecessor, stale cache/clone, revocation, mapping loss,
  rollback, restored old bytes, and non-current head;
- exact publisher object/challenge/response/mapping syntax and substitution;
  exact retry conflict; cold restart immediately before/after H2 and before
  generic-intent creation/fsync; missing/ambiguous candidate/witness;
  concurrent retry; durable quarantine/non-HCM coexistence; and faults after
  every registry, staging, canonical-install, publication-result, marker,
  evidence, result, ledger, and committed-rename boundary; and
- unchanged generic non-HCM intake/candidate/promotion/recovery/currentness,
  HCM-3.1 vocabulary, stack identity, profile replay, and useful L0-L3
  precursor behavior; and
- a valid anchored open quarantine record for candidate A plus partial
  completion scratch refusing a different otherwise-valid candidate B before
  any durable byte changes.

The proportional wall includes focused tests, affected engine and lineage
tests, full workspace all-target/all-feature tests, strict Clippy, format,
diff/whitespace, exact definition/profile replay, ordinary handoff validation,
both orchestration self-tests, GitNexus scoped and compare-to-main detection,
remote-baseline verification, and protected-path proof. Unavailable FTS or
comparison remains unavailable, never GREEN.

The 2026-08-05 fresh-parent replay passed that wall. It includes the exact
valid anchored quarantine candidate A plus partial completion scratch refusing
a different otherwise-valid candidate B before any durable-byte change, 32/32
Context Resolution kernel tests, 55/55 generic-lineage tests, complete engine
and workspace all-target/all-feature tests, workspace check, strict Clippy,
format, whitespace, ordinary handoff validation, and both self-tests. GitNexus
scope-all and compare-to-`main` completed at `CRITICAL`; the scoped symbols and
flows remain inside the authorized currentness/quarantine/generic-lineage
seams. FTS is unavailable and is not classified GREEN. Exact evidence is in
[`proof/20260805T125727Z--quarantine-finalization-proof-wall.md`](proof/20260805T125727Z--quarantine-finalization-proof-wall.md).

## Deferred continuous availability

The staged H2 installation-authorization / H3 operational-activation design
is documented in
`decision/2026-08-04-deferred-continuous-availability.md` as future work and
non-authority. HCM-3.2 does not implement it. Its trigger is a future explicit
product requirement or SLO for uninterrupted Context Resolution through every
replacement crash boundary.

## Completion and stops

Planning is CLEAN and the recovered implementation/proof subject is completed
locally at reviewed primary commit `9fd54c2f746eddd4df5dff37ba3828626395000d`.
The completed v1.4 handoff
[`20260805T153602Z--HCM-3-2--orchestration--context-resolution-quarantine-finalization-completed.json`](../../handoffs/records/20260805T153602Z--HCM-3-2--orchestration--context-resolution-quarantine-finalization-completed.json)
records the fresh-parent complete-subject `CLEAN` closure with no unresolved
P1/P2, completing HCM-3.2 and closing `PG-RES-01` only for this bounded kernel.
The mechanical P4 closeout is recorded separately after the handoff validator
and both self-tests; it adds no continuing authority.

No automatic continuation is authorized. Stop before HCM-0.11, HCM-3.3+, any
fourth production path, tooling/schema/template work, public-API or dependency
expansion, generic ownership change, unsafe/native/platform/transport work,
shipped identity change, release, push, publication, unrepresentable v1.4
cadence, exhausted review budget, unavailable mandatory delegation, or
unresolved P1/P2. If the minimum replacement floor cannot fit these bounds,
return a durable same-slice true stop; bootstrap-only scope is not inferred.
