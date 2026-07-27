# HCM-2.4 P7 partial control-pack closeout

Status: **partial true-stop subject; fresh aggregate review CLEAN**

Recorded at `2026-07-27T06:37:22Z`.

## Reviewed implementation boundary

The independently reviewed implementation is split across two incremental
commits:

1. `5cf41d2f64d68cb7f78abb5eccab9acf307089d3` contains the review-clean P0,
   P1A, P1B, P1C, and P3 packets plus truthful P2 and P4/P5 stop evidence.
2. `5a2ccf6939be0e549e7e7355a245c7511c271eef` contains the review-clean P3B
   test/evidence integration and its deterministic post-review whitespace
   record.

The checkpoint staging inventory excluded all unrelated `.claude/**`,
`AGENTS.md`, and `CLAUDE.md` changes and all transient generated
fixture-repository outputs. After the P3B commit, exactly those eight
unrelated/pre-existing paths remained changed.

## Earned truth

- Five exact successor kind/intake/fixed-renderer closures are package-admitted
  without changing released schemas, public APIs, dependencies, Cargo
  metadata, commands, or package boundaries.
- `ResolvedSchema::collect_coverage_leaf_shapes` classifies only a type-absent,
  string-valued `const` leaf as String. Non-string `const`, `enum`, default,
  examples, annotations, composites, unsupported explicit types, and every
  other indeterminate shape retain refusal. No sixth P1A production symbol was
  added.
- Shipped-root `1.2` is the live selection with exactly Project Context,
  Project Authority, and conditional Environment Context. Released HCM-2.2
  Charter authority retains its exact `1.1` identity pair.
- Stage 10 Work Specification YAML is the capture, provenance,
  feature-identity, and handoff authority. Feature Spec Markdown is a
  deterministic human-review view and is not an authority input.
- P3B independently closed its only discovery P2 and P4, passed its 273-test
  packet wall and complete workspace wall, and ended with a different-fresh
  CLEAN closure review.

## Historical blocked and unearned truth at that closeout

- P2 is blocked on a separately approved, fingerprinted condition-evidence/
  evaluator contract and exact implementation selector. `indeterminate`
  applicability cannot be coerced.
- P4 and P5 are blocked on a separately approved generic coverage-token
  derivation selector. Their exact fixture selection, read/validate, renderer,
  and negative evidence do not prove positive mutation or promotion.
- P6 is ineligible. Both `BR-HCM-2-PILOT-FLOW-01` and
  `BR-HCM-2-CHARTER-FLOW-01`, fixed-family/path selection, and their deletion
  gates remain live.
- The Phase 2 exit gate, program-wide `PG-YAML-02`, HCM-2.4 completion, and all
  HCM-3.x work remain unearned.

The later operator-approved P4/P5 selector superseded only that historical
token blocker. Commit `00dde0162fcb15576c83b6ed40ab7286488d0890` now proves
the shared token prerequisite and exact positive Decision/Risk mutation paths;
P4's separate negative-surface/path gate remains open. P2 now has a remediated
exact planning amendment with a six-row matrix, count-only entry-4097
termination, one intentionally shared bounded over-limit closure, and one
fresh review gate, but remains runtime-blocked. CLEAN permits only distinct
human approval of P2S/P2A feasibility; a later explicit production adapter or
completed-producer decision is still required. P6, Phase 2 exit, and HCM-3.x
remain unearned.

## Control-pack delta

The aggregate review subject updates only:

- the exact HCM-2.4 partial boundary in `03-seam-crosswalk.md`;
- the HCM-2.4 slice row and still-open Phase 2 exit in
  `04-phase-slice-map.md`;
- exact earned `PG-KIND-01`, `PG-ARTIFACT-01`, `PG-YAML-01`, and
  `PG-YAML-02` evidence plus an explicit partial-boundary section in
  `06-proof-and-regression-ledger.md`;
- HCM-2.4 SPEC/plan/todo/proof status supported by the two reviewed commits;
  and
- this partial closeout proof.

No bridge row is deleted and no broad or program-wide promotion is claimed.

## Verification carried forward

- P1A focused wall: 244 passed; closure review CLEAN.
- P1B focused/preservation wall: review CLEAN after remediation.
- P1C engine wall: 343 passed; compiler 9/9; CLI 19/19; exact WSL proof
  passed; closure review CLEAN.
- P3 implementation closure review CLEAN.
- P3/P3B packet wall: 273 passed.
- P3B semantic proof: 1/1; CLI surface: 98/98.
- Complete workspace wall after P3B remediation: exit 0 with two
  platform-marked ignores.
- Strict affected-crate clippy, formatting, archive boundary/self-test, and
  diff hygiene passed.
- Pre-P3B-commit rerun: semantic 1/1, CLI surface 98/98, formatting and archive
  boundary passed; staged GitNexus detection reported LOW risk, 50 files, two
  symbols, and zero affected processes.

The aggregate control-pack reviewer must verify the live commit subjects,
status consistency, exact maximum promotion, preserved blockers and bridge
rows, and absence of authority widening. Documentation-only closeout does not
invent a code-symbol blast radius.

## Advisory disposition

The P3 closure review left one valid P3 advisory at
`crates/pipeline/tests/pipeline_handoff.rs:406`: the Markdown-view
mutation/deletion test compares canonical bytes, canonical input SHA, view
absence, and validated manifest equality, but does not explicitly compare
`manifest.feature_id` or the emitted bundle root across the two cases. Runtime
behavior is correct and the added assertions are useful coverage beyond the
selected proof wall. The parent will register this exact advisory as
`HCM-RF-0001` in the mechanical handoff/ledger closeout commit; it is not part
of the material control-pack review subject.

No unresolved P1/P2 intersected the reviewed P0/P1/P3 subjects at that
closeout. The then-current authority prerequisites blocking P2 and P4/P5 were
top-level resume conditions, not waived review findings.

## Fresh aggregate review

Dispatch
`20260727T064246Z--HCM-2-4--partial-control-pack-complete-subject-review`
bound the complete 168-path pre-closeout subject at
`sha256:8e2f246b9739e1d0bafe125d5408d44e2f0eb3d7e6708dbdba7a9f8f71acbf43`.
Fresh reviewer `/root/hcm_2_4_partial_closeout_review` returned CLEAN with no
Critical or Required finding, independently corroborated planned
`HCM-RF-0001`, and reported one P4 malformed-status nit in
`decision/20260726-p1c-project-context-kind-compatibility.md`. The parent
validated and corrected only that status sentence; no implementation,
classification, proof, or authority byte changed, so no re-review is required
under the P3/P4 disposition rule.

The reviewer replayed all 168 hashes, the aggregate fingerprint, ancestry, and
scope; passed 319 focused tests with two expected ignores and the 273-test
P3/P3B wall; and passed formatting and scoped diff hygiene. Its workspace
rerun exceeded the output window, while the manifested fingerprint-bound
complete workspace result remains exit 0.
