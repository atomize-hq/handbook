# Frozen quota V2 source review

**Source-only disposition: REMEDIATE. Runtime HOLD; full V6/experiment qualification INCOMPLETE.** Two bounded oracle defects remain. No imports, controls, oracle, engine, native test or Git operation were executed. The proposed10positive/57negative cases are source definitions, not passing results. FINAL_SOURCE_PIN=None correctly prevents full V6 verification; no paced/final pin is inferred.

Five frozen sources total44,814bytes within45,056. First/end observations match the exact supplied hashes:

The full five-file and eight-dependency byte/SHA manifest is in evidence-quota-v2.json.

All five full sources and their transitive code were reviewed, including the previously read unchanged v5/R1/page code and the completed legacy decoder/tree/navigation source reads. The five new plus eight transitive size/hash observations all match freeze receipt80ceaace78e0a6afd6143f3eba03ce46dd7f13c21fe8801b1cf7824eb5c4d6ee. The declared Rust format owner lifetime.rs matches3172ba1b1e78d048c2eb168b02ca524b4d3979abaa6528aaebb149a8a2c3b824; its registry/receipt/stage logic was compared directly as source. No claim of a new engine compile or final-engine equivalence is made. Last source hashes were checked before static-complete/source-freeze-release notice; no source was read after release. Old R1 and REMEDIATE artifacts remain intact.

Paths below are relative to the matrix execution/harness directory unless explicitly Rust. Manual impact remains HIGH/graph UNKNOWN: quota.verify -> v6.verify and controls; typed semantic V2 -> v6 full audit. No symbol edits or graph/index/bootstrap were performed.

## Q1 [P1] Control-stage checkpoint names disagree with the actual emitter

`quota_v2_oracle.py:176-198` derives allowed checkpoint names from the filename suffix. For terminal-control-stage it accepts terminal-control-stage-created, terminal-control-stage-before-write, terminal-control-stage-partial, terminal-control-stage-write and terminal-control-stage-staged; phase extraction also removes that filename prefix.

The frozen Rust lifetime.rs:265-266 calls write_stage with phase **terminal-control**. Its lines249-257 emit **terminal-control-created/before-write/partial/write/staged**, with no extra -stage. Receipt-stage uses terminal-receipt-stage and already agrees.

Concrete causal failure: a real process dies at terminal-control-partial with its owned replacement stage containing the first half of the exact expected registry. All owner/registry/candidate/identity/hash/schedule evidence may be valid, but a proof carrying that actual native label fails stage-checkpoint. Conversely, controls.py:47-51 generates the oracle's invented suffix-created name, and line82's control-prefix positive therefore masks the mismatch. This prevents truthful qualification of the mandatory control-write crash windows.

Fix: use an explicit filename-slot to native emitted-phase mapping; derive recognition and phase extraction from the emitted phase. Keep the existing owner/source/schedule qualification and exact prefix/length checks. Do not change Rust event names solely to match the test helper.

Required source controls: independently spell the frozen native label for each control phase, with empty/half/full bytes at the appropriate phase; retain a legitimate reused-stage-created prefix because Rust opens an owned prior stage before truncation. Reject the invented terminal-control-stage-* labels. Keep receipt-stage positives and malformed-prefix/phase negatives. No new native run is implied by requesting these source definitions.

## Q2 [P2] A terminal receipt can attest an impossible original allowance

`quota_v2_oracle.py:157-175` checks u64 fields and used_bytes+128KiB == charge_bytes <= original_allowance, but places no admission ceiling on original_allowance when the owner has already left active. The optional before-owner proof may be absent.

The same oracle's registry:41-43 and frozen Rust Registry.validate:25-35 require active allowances + terminal bytes +128KiB control margin <=32GiB. A new V2 owner that can later become terminal therefore necessarily had original_allowance <= **LIMIT-MARGIN**. Imported V1 owners are marked legacy and cannot acquire this new terminal preselection proof, so they do not supply an exception.

Concrete static counterexample: take the committed receipt fixture and change only original_allowance to33GiB, serialize the exact receipt bytes again, retain the small used/charge/candidate and exact current terminal aggregate, and omit optional preowner evidence. Every present relation/private-byte/member check still passes, despite a prestate that could never have admitted that owner. This is not an executed test. It does not claim disagreement with Rust TerminalReceipt.validate, which currently also checks the relational receipt fields only; it identifies missing independent historical-admission closure.

Fix: enforce original_allowance <= LIMIT-MARGIN (<=32GiB alone is a weaker necessary bound), retaining exact charge=used+128KiB and charge<=original. Continue comparing all before-owner fields when authenticated preowner evidence is supplied. No historical I/O reconstruction or mandatory full historical registry archive is needed to enforce this unavoidable bound. Used remains explicitly a durable attestation, not measured historical writes.

Required source controls: a self-consistent oversized-original receipt negative with unchanged valid small charge and no preowner proof, a LIMIT-MARGIN+1 boundary negative, and a feasible exact-ceiling positive. Add a supplied-before-owner mismatch negative. Do not turn the 33GiB number into a real allocation or fixture.

## Remaining source assessment

- Registry dispatch mirrors the numeric-version discriminator and keeps the legacy semantic path separate. V2 controls have closed shapes, scalar/bool/u64 checks,8active/4096terminal caps, legacy subset/phase checks and32GiB aggregate margin. V1 gets no new terminal-ownership proof; its legacy semantic verifier remains controlling.
- Terminal receipt count/bytes are recomputed exactly from immutable final receipts. Published receipts whose identical preselection owner is still active are tracked separately, avoiding double credit/count. Candidate raw digest, request/intent/base, historical selected membership, terminal kind, accepted-attempt exclusion and no postselection allocation are checked. A fresh other attempt accepting the request does not transfer this failed owner's private bytes/charge.
- Candidates bind replay object/file hashes and consumer; terminal and sealed active roots are passed as normalized sealed semantic owners. The V2 semantic branch retains full accepted membership/outcomes/results, retired controls, graphs/indexes/projections/sources and request-intent checking, then adds private root budget/count/cost constraints and B generation/projection equality plus exact generation namespace. V1 invokes the old verifier. No omission of those existing semantic checks was established.
- NativeSnapshot is an explicit full read-only census: rejects ancestor/tree symlinks/reparse points, records native identities/link counts and exact bytes, checks opened identity/length/mtime and bounded rereads, and checks the complete before/after tree. Private bytes are charged against used/current allowance/terminal charge without calling shared replay resources new private allocation. Hardlink aliases are inventoried, critical receipt/stage aliases refuse, and no allocated-byte/deletion credit is invented. Native correctness remains untested here.
- Stage ownership needs the exact active/terminal owner and separately supplied qualified schedule/source/hash/phase evidence; a filename alone is insufficient. The supplied qualified schedule digest list is a trusted caller input that the parent must separately validate, not proof synthesized by this parser. Q1 currently prevents actual control-phase evidence from passing.
- V6 checks companion hashes before/after and preserves v5's reverse mapped-role closure, full pack/tree audit, explicit inventory option and semantic/navigation/intent adapters. It records a compressed native identity manifest and demands unchanged input state. One root per subprocess remains required because callbacks are process globals. Full source pin None blocks verification before store reads; assigning a final pin later requires a fresh exact binding/review, not an implied waiver.

## Proposed control meaningfulness, not runtime results

Source contains10positive case labels and57negative labels after loop expansion. They target useful independent invariants: V1 dispatch, active/selection/published-pending/committed states, stage prefixes, B private files, fresh winner and mock alias census; invalid registry caps/types/legacy/phases, receipt fields/charge/kind, candidate binding, selected terminal owner, postselection phase, pending double count, missing/foreign/private files, undercharge, hardlinks/IDs, schedule/source/prefix/phase, B missing/hash/extra and receipt framing.

Their scope is only the quota parser with a supplied MemorySnapshot/Authority. They do not exercise NativeSnapshot file handles, the full typed semantic V2 walker, v6 composition, actual request acceptance, native IDs, qualified native schedules or any final engine. The V1 positive is empty-registry dispatch, not broad legacy compatibility. Mock prefix schedule evidence is intentionally fabricated parser data. Current tests accept any listed exception class without asserting a specific refusal reason, so future receipt review must verify that changed negatives reach the intended invariant rather than an unrelated earlier error. Q1's positive helper currently shares the wrong name construction; Q2 is absent. None of the67case definitions has been run under this authorization.

## Budget, preservation and disposition

Fresh independent META9 closed-membership census uses P/meta-membership-v4.json SHA024a23d8304de4dec44332b1c6d686031ffc1dbc9f5a30e678f2bae03d5dfe0e: all P recursive files; Landing root/execution-root regular files; seven named execution subtrees with stream-snapshot-controls excluded; all20named promoted harness files. Each pathname is counted once, including physical copies. Actual8,068,802bytes/957files; adding16,384review reserve and358,400future reserve gives8,443,586 <9,437,184cap. P alone and the withdrawn8MiB runtime forecast were not used.

Only this review and evidence-quota-v2.json are written, together <=16KiB. Preserve80cea/89685 and prior source drafts/receipts. Source disposition REMEDIATE Q1/Q2; the parent may prepare a new frozen source for review. Runtime remains HOLD pending independent hard-bounded guard/output disposition, fresh complete census and explicit finite GO. No AST import, controls run, full V6/native/real-store qualification or clock reset is granted by this report.


