# Causal Review Budget and Lineage Hardening

Status: implemented control decision and deterministic regression record

Control scope: HCM-0.8 orchestration protocol

Task: `causal-review-budget-and-lineage-hardening`

Date: 2026-07-28

## Authority boundary

This record repairs orchestration control only. HCM-2.4 runtime, product
definitions, product schemas, Cargo files, product tests, SPEC, plan, todo, and
historical dispatch/handoff bytes are evidence and remain unchanged. In
particular, commit `6734e525342139456c8c1c195697c0ffa4b34049` and handoff
`20260728T011713Z--HCM-2-4--orchestration--p2-course-correction-decision-required`
are immutable HCM-2.4 truth. This control repair neither selects nor supersedes
that handoff.

## Evidence and reproduction

The observed P3B population is the 13 immutable `*p3b*.json` dispatches from
`20260727T021420Z` through `20260727T061322Z`, under parent orchestration
`20260726T195633Z--HCM-2-4--implementation-resume-orchestration`. The raw
closeout evidence is:

- `records/20260727T070426Z--HCM-2-4--orchestration--authority-prerequisites-required.json`
- `records/20260727T165925Z--HCM-2-4--orchestration--p2-review-budget-exhausted.json`

Replay against the predecessor validator established all five failure modes:

1. changing packet/selector and cycle subjects created another discovery under
   the same parent and integrated P3B outcome;
2. reconciliation considered only `delegated_runs` declared by the handoff;
3. proof-relevant dispatches omitted from that array were invisible;
4. related failures exposed by remediation could be reintroduced as discovery;
5. untyped packet subdivision was indistinguishable from a legitimate
   planning-to-implementation transition.

The P3B closeout declares one delegated run although the immutable parent
population contains the 13 P3B review dispatches above: nine discovery cycles
and four closures. By contrast, the
bounded P2 handoff declares its discovery, closure, and two supplemental
cycles, then stops when the allowance is exhausted.

## Root causes

- Budget identity was local to mutable packet/cycle names instead of the
  parent orchestration and integrated outcome.
- Integrated outcome identity itself was initially self-asserted, so renaming
  it could manufacture another derived budget.
- Review cycles had no explicit monotonic stage dimension.
- Failure provenance was not typed, so a remediation-unmasked failure could be
  relabeled as discovery.
- The handoff array was treated as the dispatch universe rather than being
  reconciled with immutable dispatch files.
- The first discovery review had no complete convergence gate for packet,
  recursive fixture/consumer, manifest, formatting, and whitespace scope.
- Fixture and golden repair authority had no small typed ceiling.
- RFC 3339 timestamp offsets permitted lexical population-order ambiguity, and
  v1.3 predecessor bytes lacked corpus-level freezing.

## Decision

Adopt additive `internal-dispatch` and `handoff-record` v1.4 successors.
V1.1, v1.2, and v1.3 remain byte-frozen validation-only evidence. A v1.4
parent cannot mix predecessor dispatch versions.

The successor contract provides:

- a parent-level authorized outcome/packet/authority registry frozen before
  review and fingerprint-bound into every current dispatch;
- a deterministic `causal_budget_id` derived from parent orchestration ID and
  registry-authorized integrated outcome ID;
- explicit monotonic `planning`, `implementation`, `proof`, and
  `final_closeout` stages, with one discovery lineage per stage;
- typed causal reasons for reviewer findings, remediation-unmasked failures,
  proof gaps, manifest/scope omissions, authority expansion, and external
  blockers;
- executable-dispatch prefix validation, so renamed discovery subdivision
  fails before another run is launched;
- exact parent-population enumeration and a deterministic aggregate in the
  parent handoff, including failed, blocked, abandoned, and deliberately
  non-executed runs;
- a five-part pre-review convergence gate;
- a bounded ancillary allowance restricted to exact test fixtures, copied
  fixture authority, deterministic goldens, and assertions, with path, count,
  changed-line, and risk ceilings mechanically replayed from a Git baseline
  and again at primary-commit closeout;
- canonical UTC-second `Z` timestamps with parsed-instant ordering and exact
  aggregate freezing of the 66-dispatch/12-record v1.3 corpora.

Separate packets remain legitimate when they represent separate integrated
outcomes, or when they advance an existing outcome through an explicit
monotonic stage transition. Mechanical closeout remains outside the review
cycle count and cannot create or reset a discovery lineage.

## Deterministic proof

`validate_handoffs.py --self-test-orchestration-contract` contains named
successor-contract cases for every mandatory regression:

- renamed discovery and post-CLEAN discovery fail;
- remediation-unmasked failure consumes the next causal cycle;
- a third supplemental fails;
- omitted dispatches fail and explicitly abandoned dispatches reconcile;
- separate packets and planning-to-implementation stage advance pass;
- mechanical closeout cannot reset the budget;
- the exact observed nine-discovery/four-closure P3B shape fails, including an
  attempted packet-as-outcome budget reset;
- the bounded P2 discovery/closure/two-supplemental lineage passes.

The same self-test validates the v1.4 templates while retaining the full v1.3
orchestration suite. `--self-test-v1-admission` and ordinary validation retain
the frozen historical corpora. Raw command results are referenced by the
review dispatch and final parent handoff rather than copied into this record.

## Independent review remediation

Fresh discovery review
`20260728T024913Z--HCM-0-8--causal-review-budget-lineage-hardening-review`
returned four valid P2 findings and one P3 evidence-shape correction:

- outcome IDs could be renamed into new budgets;
- v1.3 dispatch and handoff bytes were not aggregate-frozen;
- offset timestamps could evade lexical population cutoffs;
- ancillary changed-line counts were self-declared;
- the synthetic P3B regression represented eight discoveries instead of the
  immutable nine-discovery/four-closure population.

The repairs above freeze and bind the outcome registry, hash-admit both v1.3
corpora, require canonical UTC and parsed ordering, recompute ancillary diffs,
and replay the exact evidence shape. The original discovery dispatch remains
immutable and is exact-hash admitted only as the pre-registry review that
identified these defects; all later executable dispatches require the registry.

Different-fresh closure review
`20260728T032400Z--HCM-0-8--causal-review-budget-lineage-hardening-closure-review`
returned one further P2: closeout recomputed the primary-commit ancillary diff
but did not compare it with the path, per-path changed-line, aggregate
changed-line, and path-count ceilings approved before review. The remediation
preserves those reviewed ceilings by baseline and path, rejects any final
observation above them, and adds an exact post-review-growth regression.

The first supplemental closure review
`20260728T034100Z--HCM-0-8--causal-review-budget-lineage-hardening-supplemental-review`
then returned `HCM-CRBL-P2-04-C1-S1`: closeout took maximum ancillary ceilings
from every parent dispatch, so a later non-review mechanical dispatch could
widen the clean review's frozen allowance. The bounded remediation cuts the
applicable allowance population off at the final clean-review dispatch,
rejects every later non-zero allowance, and replays the exact mechanical
widening attempt as a deterministic negative regression.

## Rejected alternatives

Increasing supplemental counts, weakening independent review, authorizing
open-ended paths, retroactively rewriting predecessor records, and building a
separate review framework were rejected. They either preserve the bypass,
invalidate immutable evidence, or exceed the smallest versioned repair.
