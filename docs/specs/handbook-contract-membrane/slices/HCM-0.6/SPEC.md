# HCM-0.6 — Shipped Default Artifact Set Decision

> Active product semantics were amended in place by the HCM-2.4 greenfield
> correction. Environment Context is optional advisory context with no
> applicability condition. This file preserves the original decision process;
> the authoritative outcome is the linked decision record.

## Status and authority

This slice is Phase 0 documentation/design work. It is authorized by the
`HCM-0.6` row in `../../04-phase-slice-map.md` and is constrained by the frozen
artifact-kind, artifact-instance, requiredness, and constitutional-root
semantics in `../../02-semantic-model.md` and
`../../05-contracts-schemas-and-gates.md`.

The slice begins with no approved shipped default set. Current enums, templates,
filenames, implementation paths, illustrative control-pack examples, and
historical artifacts are evidence about migration cost or precedent only. None
has decision authority.

The explicit user decision session is now complete. The approved authority is
[`decision/shipped-default-artifact-set-decision.md`](decision/shipped-default-artifact-set-decision.md).
The research and candidate comparison remain decision provenance rather than a
second authority. HCM-0.6 still authorizes documentation/control-pack changes
only; implementation and HCM-0.7 remain out of scope.

## Objective

Produce an evidence-backed, explicitly user-approved decision that identifies:

1. the artifact kinds Handbook ships in its first version;
2. the artifact instances selected by the shipped default profile;
3. the `always`, `conditional`, or `optional` requiredness of each selected
   instance, including an exact condition reference for every conditional
   instance; and
4. the lifecycle, overlap, and future Projection posture needed to keep those
   choices coherent with the frozen semantic contracts.

## Decision method

The decision must be made in this order:

1. inventory the current Handbook artifact surface without treating it as a
   recommendation;
2. research primary-source precedents for durable project authority, context,
   architecture decisions, work specifications, environment/deployment truth,
   risk, and operational knowledge;
3. extract failure modes, especially stale duplication, document sprawl,
   mixed authority, mandatory-empty artifacts, and project/work-scope
   conflation;
4. compare minimal, standard, and full candidates using one explicit rubric;
5. run a user brainstorming/decision session one decision at a time;
6. record the approved kind, instance, and requiredness lists with rationale and
   rejected alternatives; and
7. obtain fresh independent review over the complete decision subject.

## Decision rubric

Every candidate is compared against:

- authority clarity and exactly-one constitutional root;
- default onboarding burden;
- coverage of stable project context versus per-change/work context;
- overlap and drift risk;
- conditionality and empty-document pressure;
- deterministic lifecycle and review triggers;
- repository diversity and custom-kind escape hatches;
- fixed renderer needs before Phase 3 and future Projection needs after it;
- compatibility with the frozen SDK/transport owner boundary; and
- implementation/adoption cost, recorded as a tradeoff rather than decision
  authority.

## Required outputs

- `research/shipped-default-artifact-set-research.md` — primary-source research,
  current-state inventory, failure modes, and source links;
- `research/candidate-comparison.md` — minimal/standard/full comparison and a
  clearly labeled recommendation that is not approval;
- `decision/shipped-default-artifact-set-decision.md` — explicit user-approved
  kind, default-instance, requiredness, rationale, and rejected-alternative
  record;
- affected canonical control-pack rows updated only after approval; and
- a final proof record demonstrating `PG-DEFAULT-01` without claiming runtime
  implementation.

## Human decision boundary

Research and recommendations may be prepared autonomously. Approval may not be
delegated to a subagent or inferred from silence, implementation precedent, or
an assistant recommendation. If the user decision is not available in the
current top-level run, preserve the reviewed research/candidate subject and stop
with `stop_reason=human_input` plus an exact single-question resume boundary.

## Exit gate

The slice closes only when all of the following are true:

- the research dossier uses primary sources and separates sourced facts from
  local inference;
- minimal, standard, and full candidates are compared using the same rubric;
- the user has explicitly approved the shipped kind list, default-instance
  list, and requiredness/condition decision for every selected instance;
- the approved decision is coherent with kind/instance separation and the
  constitutional-root invariant;
- a fresh independent reviewer reports no valid Critical or Required finding;
- the proof wall and repository-required change detection pass; and
- only documentation/control-pack artifacts are committed.

The greenfield-amended decision fixes exactly six kinds and three root-profile
instances: required Project Context, required Project Authority, and optional
Environment Context. Environment Context presence and validity control advisory
context use; a separately selected task gate controls enforcement. No global
applicability condition, evidence policy, or compatibility-only profile is part
of the shipped default. The unique constitutional root, exact role support, and
the lifecycle/intake/renderer/Projection posture remain as decided.

## Non-goals

- Rust, Cargo, runtime, schema, CLI, Tauri, SDK, Substrate, or dock work;
- implementation of the shipped profile;
- HCM-0.7 planning or approval;
- changes to frozen HCM-0.2 through HCM-0.5 contracts except the narrow
  approved-default data references authorized by this slice;
- dynamic CLI commands or command renaming;
- selecting a default from the current four-variant enum;
- requiring every shipped kind as a default instance;
- treating a work-scoped artifact as a project singleton without an explicit
  decision; or
- marking `PG-DEFAULT-01` closed before the user-approved decision and clean
  review exist.
