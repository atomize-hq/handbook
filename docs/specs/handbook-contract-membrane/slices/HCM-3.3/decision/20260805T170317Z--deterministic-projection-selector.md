# HCM-3.3 Deterministic Projection selector decision

**Decision status:** frozen planning candidate pending the required fresh
discovery and different-fresh closure reviews.

## Authority and scope

This decision selects exactly one documentation-and-planning outcome:
`HCM-3.3-P1-selector-plan-causal-review`. It consumes the completed HCM-3.2
handoff as immutable prerequisite evidence and the HCM-3.3 Phase-map row plus
the canonical Projection sections as source authority. It neither resumes nor
changes HCM-3.2.

Allowed primary-state paths are exactly:

- `docs/specs/handbook-contract-membrane/slices/HCM-3.3/SPEC.md`;
- `docs/specs/handbook-contract-membrane/slices/HCM-3.3/tasks/plan.md`;
- `docs/specs/handbook-contract-membrane/slices/HCM-3.3/tasks/todo.md`;
- this decision, HCM-3.3-local review dispatches, and HCM-3.3-local proof
  records strictly needed for the planning lineage.

The second, mechanical closeout may add only one parent-owned v1.4 handoff and
the deterministic ledger rebuild. No other path is admitted.

## Selector decision

Plan a single generic deterministic Projection engine that accepts exact
fingerprinted sources and an exact fingerprinted ProjectionDefinition, then
produces only deterministic `reveal` or `derive` views under a supplied
Context Resolution envelope. A configured registered custom kind must be
selected declaratively with the same exact ref/fingerprint closure and execute
through this engine without a first-party branch, new operation, or source-order
fallback. The plan fixes the following future boundary:

- exact profile/definition/source validation owns source/capability
  compatibility and refuses before rule evaluation; the exact resolved-profile
  catalog admits the definition;
- the definition owns source selectors, surfaces, reveal/derive eligibility,
  target schema, disclosure/support/currentness closure, field rules, and
  allowlisted acyclic derivations; `none` currentness is null/empty while exact
  checking is snapshot-selector-only and uses captured family/adapter/slot
  revisions;
- its support evaluator is exact, versioned, built-in, metadata-only, and bound
  to the canonical input allowlist and deterministic first-unsupported-reason
  order; evaluator identity and semantic drift close into definition,
  evaluation, and result fingerprints;
- the engine performs full rule accounting and returns typed omissions,
  lossiness, provenance, output/result fingerprints, and `authority_effect:
  none`;
- a collapse is narrower-or-equal Projection; expansion is a broader valid
  request or `ResolutionEscalationRequest`, never self-widening;
- the core performs no synthesis, source mutation, authority promotion,
  transport rendering, or Snapshot Memory capture/adoption.

The decision is a future implementation plan, not an implementation packet.
It deliberately does not select code symbols or paths. Any future code edit
requires a new explicit selector followed by fresh GitNexus impact analysis.

## Dependency proof

The assigned checkout and dedicated local integration ref both resolve to
`12c203f605c26ce7d1911e4ee60c0de205822000` and tree
`d1cca0b342a8f671a4b9cef3f330cab42c0c32ef`. The completed HCM-3.2 handoff
records `PG-RES-01` as complete only for its bounded kernel and expressly
prohibits automatic HCM-3.3 continuation. The current user authorization is
the new, narrower planning-only grant that supersedes that prohibition only
for this selector/plan work.

`PG-PROJ-01` and `PG-PROJ-02` remain open. This packet may define the proof
needed to close them later, but claims neither gate, Phase 3 exit, a Projection
engine, a Snapshot capability, nor a real-path adoption result.

## Causal outcome registry

| Integrated outcome ID | Packet IDs | Authority reference |
|---|---|---|
| `hcm-3.3-deterministic-projection-selector-plan` | `HCM-3.3-P1-selector-plan-causal-review` | this decision |

The registry is frozen before discovery. Its v1.4 canonical JSON fingerprint
and derived causal budget are carried unchanged by every planning dispatch and
the completed parent handoff.

## Review and proof wall

The first reviewer receives only bounded authority, the candidate subject,
proof wall, and non-goals. It performs one complete-subject discovery review
and returns findings first. The parent validates every P1/P2, remediates them
once in a consolidated pass, and sends the delta to a different fresh closure
reviewer. Closure may consider only known repairs and immediately
repair-caused/unmasked P1/P2. A clean closure ends review.

Planning proof is: exact base/ref/tree and protected-path observations;
authority and predecessor validation; replayable subject manifests; strict
UTF-8/whitespace and `git diff --check`; v1.4 dispatch and handoff validation
with both self-tests; ordinary scoped-diff inspection; and local-only
compare-and-swap publication evidence. GitNexus impact is not applicable to a
code-symbol-free change; GitNexus change detection is recorded unavailable if
the capability is absent.

Future implementation proof also includes a positive configured-custom-kind
replay and invalid-custom-configuration refusal; profile-unlisted definition
refusal; null/empty and captured-revision currentness negatives (including an
equal-live-revision stale source); evaluator substitution/registry/input/reason
ordering negatives; and evaluator-semantic-drift fingerprint/refusal replay.

## Explicit non-goals

Do not change production Rust, tests, fixtures, profiles, schemas,
registries, validators, templates, skills, tools, dependencies, public APIs,
versions, snapshots, pipelines, adopters, consumers, transports, HCM-0.11,
HCM-3.2, HCM-3.4+, or any remote state. Do not create an HCM-3.3
implementation packet, push, fetch, query a remote, merge, rebase, reset,
clean, force-update, or move the integration ref before final local
compare-and-swap publication.
