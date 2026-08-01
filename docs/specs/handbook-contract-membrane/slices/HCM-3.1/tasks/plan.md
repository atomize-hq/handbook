# HCM-3.1 Delivery Plan

Status: planning discovery findings remediated; fresh closure pending
Authority: `../SPEC.md` and
`../decision/2026-08-01-vocabulary-resolution-selector.md`

## Orchestration identity

- Parent orchestration:
  `20260801T122747Z--HCM-3-1--vocabulary-resolution`
- Integrated outcome: `hcm-3.1-vocabulary-resolution-full-slice`
- Outcome-registry fingerprint:
  `sha256:d0f92e3453edc70c4ed9a185d69dfbbe631fbe84653bad07ecbf060d8ef20ee2`
- Derived causal budget:
  `sha256:f32411b780227c90d7d3146339f933cd19df4911a82043d981958f67663b8758`
- Frozen packets:
  - `HCM-3.1-P1-vocabulary-kernel`
  - `HCM-3.1-P2-profile-renderer-adoption`
  - `HCM-3.1-P3-proof-control-closeout`

No runtime edit begins until a fresh read-only planning discovery returns CLEAN,
or valid P1/P2 findings are remediated and a different-fresh closure returns
CLEAN.

## Step 0 — preserve live state

1. Confirm branch and HEAD from live Git state.
2. Hash and record all nine protected paths, including the untracked ideas file.
3. Keep them unstaged and exclude them from every path manifest and commit.
4. Back them up before any GitNexus refresh and restore byte-for-byte if
   generated counts change `AGENTS.md` or `CLAUDE.md`.
5. Confirm the three dependency handoffs are completed and contain no unresolved
   P1/P2 blocker for HCM-3.1.

## Step 1 — authority and selector freeze

1. Read the HCM `00`–`06` sections selected by the slice prompt.
2. Explore vocabulary concepts through GitNexus before text search. Record FTS,
   index, or comparison limitations honestly.
3. Inspect the live vocabulary, stable-role, profile-selection, instance-profile,
   renderer, and product-consumer paths.
4. Freeze the typed contract, deterministic normalization/collision rule,
   absorption graph, fingerprint preimage/order, profile integration, consumer,
   test matrix, ceilings, and stop conditions in `SPEC.md` and the decision.
5. Declare the one parent registry and derived causal budget before review.
6. Validate the planning dispatch, then send it to a fresh built-in read-only
   reviewer.
7. Remediate valid planning P1/P2 findings inside the authority ceiling and use a
   different-fresh closure. The discovery findings were consolidated in
   `../proof/20260801T124300Z--planning-discovery-remediation.md`. Do not repeat
   discovery after CLEAN in this stage.

## Step 2 — P1 vocabulary kernel

1. Run upstream GitNexus impact analysis immediately before modifying every
   existing function, method, class, or struct implementation. Report callers,
   execution flows, and risk; warn before HIGH/CRITICAL work.
2. Establish focused RED tests for typed authored data, fallback/explicit labels,
   normalization, unique/ambiguous lookup, registry matching, absorption
   constraints, canonical fingerprints, and replay.
3. Replace the shipped-empty-only guard with the reviewed typed validation and
   immutable resolved vocabulary model.
4. Preserve the exact shipped-root bytes and fingerprint vector.
5. Run focused GREEN tests and `cargo fmt --all -- --check`.
6. Verify protected hashes/status, run compare-to-`main` GitNexus change
   detection, and make a reviewable local P1 commit only after its proof is
   converged.

## Step 3 — P2 profile and real renderer adoption

1. Establish RED tests proving exact vocabulary identity survives profile
   resolution and influences only the resolved-profile identity.
2. Add the narrow read-only repository access required by the selected renderer.
3. Establish a real-path RED test using a non-empty repository/test vocabulary;
   do not modify or publish a shipped vocabulary.
4. Apply the resolved vocabulary in the fixed Work Specification renderer:
   preserve empty-vocabulary bytes, change only explicit presentation, and emit
   every absorption without losing role IDs.
5. Add ambiguity, adapter-loss, deterministic replay, and forbidden-influence
   negatives.
6. Run the complete applicable P1+P2 packet wall and focused regression targets.
7. Verify protected hashes/status, run compare-to-`main` GitNexus change
   detection, and make a reviewable local P2 commit.

## Step 4 — implementation review

1. Materialize a complete path-and-hash manifest for the implementation subject.
2. Run the converged packet wall before dispatch.
3. Validate and dispatch one fresh read-only implementation discovery review.
4. Remediate valid findings in the same causal lineage. P1/P2 require a
   different-fresh closure; P3/P4 are repaired when in scope or inventoried with
   exact owner/reason.
5. After CLEAN, do not create another implementation discovery cycle.

## Step 5 — separately typed proof stage

1. Enter `stage_transition=enter` for proof with its own fresh discovery cycle.
2. Prove real-path output, ambiguity, absorption preservation, non-influence,
   fingerprint vectors, and deterministic replay.
3. Run focused engine/flow/compiler/CLI regressions, then the full workspace
   test/format/strict-Clippy/diff wall.
4. Run forbidden-influence and HCM-3.2/HCM-3.3 absence scans.
5. Run GitNexus scoped and compare-to-`main` detection. Unavailable modes are
   recorded as unavailable, never GREEN.
6. Remediate under the permitted closure/supplemental lineage until the proof
   stage is CLEAN.

## Step 6 — earned authority synchronization

Update only evidence-earned rows:

- close `PG-VOCAB-01` only after the real path proves lexical and structural
  conflation with stable-role resolution intact;
- advance only the vocabulary subset of `PG-PROFILE-01`;
- update the Vocabulary seam to its proven classification;
- update directly affected status/evidence text in `00`–`06`;
- leave Context Resolution, Projection, Snapshot, posture, adapters, and the
  Phase 3 exit open.

Historical dispatches and completed handoffs are immutable.

## Step 7 — final-closeout review

1. Converge the aggregate implementation, tests, proof, and earned control-pack
   subject.
2. Enter the separately typed `final-closeout` stage with a different-fresh
   read-only reviewer.
3. Remediate valid findings through exact causal lineage and obtain CLEAN.
4. Make the final reviewed implementation/documentation commit or reviewed
   commit stack. Do not include the global handoff or ledger yet.

## Step 8 — mechanical v1.4 closeout

1. Write the completed v1.4 HCM-3.1 parent handoff using the same parent,
   registry, budget, reviewed commit stack, and final verdicts.
2. Rebuild `handoffs/ledger.jsonl` from immutable records.
3. Add only directly required closeout index artifacts and exact P3/P4 inventory
   registration if a fresh review required it.
4. Run ordinary handoff validation and both self-tests, preserving the corrected
   same-parent cutoff/ancillary-prefix behavior.
5. Re-verify every protected hash/status and run GitNexus compare-to-`main`
   change detection.
6. Commit the mechanical closeout separately. Do not push.
7. Stop. HCM-3.2 remains unselected.

## Commit boundaries

Preferred local stack:

1. contract/selector plus reviewed planning evidence;
2. P1 vocabulary kernel and focused proof;
3. P2 profile/renderer adoption and real-path proof;
4. aggregate proof and earned control-pack synchronization;
5. mechanical v1.4 handoff/ledger closeout only.

If a bounded remediation requires a separate reviewable commit, retain the same
packet and causal lineage. Every commit is preceded by protected-path
verification and GitNexus change detection against `main`.
