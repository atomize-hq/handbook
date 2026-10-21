# Mac native v35 independent full-source review

**NO-INTEGRATE as the full native facade; INCOMPLETE; NO runtime GO.** Three concrete source blockers remain in addition to the author's acknowledged missing implementations. Source clock is closed; this review authorizes no repair, import or execution.

All four Python files and the review JSON were read in full. The five files independently matched their frozen sizes/hashes twice (68,940 bytes total). The four old v34 code files were also read for interface/lease comparison. All 17 runtime-closure members matched the v34 manifest across retained P packets and Landing/native-macos. Relevant OwnedCall/invoke, producer binding, native-exec proof, identity, Events and admission paths were traced. The shared Harness barrier was checked against its actual Landing source.

## P1 — Constrain native control-environment paths before launch

native_facade_v35.py:254–257 accepts any EXP_* string map. It omits v34 NativeAdapter.start:111–112 checks for EXP_READY, EXP_RELEASE and EXP_PACK_CANCEL. Candidate producer.bind authenticates the candidate executable but does not constrain that environment.

An admitted candidate call can therefore set a real EXP_BARRIER and an EXP_READY pathname outside its owned F08 case controls. Rust's checkpoint writes that pathname in the native child. Python's patched open/file lease does not intercept child filesystem writes; the phase-root census cannot detect an external overwrite. Root admission and valid executable identity do not close this path.

Before any future release, validate source-recognized file-bearing environment variables against exact declared case-control paths and source checkpoint mappings before input allocation/launch. No native overwrite was attempted in this review.

## P1 — Use the actual shared barrier contract

native_lifecycle_v35.py:67 selects phase from EXP_PAUSE; native_case_extensions_v35.py:55 uses the same key for paused-reader quiescence. Actual focused_protocol.Harness.barrier:497–499 supplies EXP_BARRIER, EXP_READY and EXP_RELEASE. Final Rust checkpoint handling recognizes EXP_BARRIER.

A normal shared barrier therefore yields phase=None and fails source-bound-ready-phase. The quiescence path likewise rejects an otherwise authentic paused reader. Missing recipe dictionaries do not reconcile the existing shared-schedule interface. Future wiring must use and verify the real barrier key/paths; inventing a second optional label is not interface qualification.

## P1 — Bind each unit/route to its own authenticated invocation

native_case_extensions_v35.py:125–159 checks receipt hashes, current nonce, binary hash and numeric closure, then accepts provider-supplied names and booleans. It neither rejects reused receipts/identities nor checks raw invocation/test filter/route parameters against the corresponding expected unit/route.

For units sharing a Cargo test executable, one genuine one-test receipt can be relabeled under multiple expected names, with test_count=1 claimed on each row. All 17 ordinary routes share /bin/bash; one genuine receipt can likewise be reused with 17 distinct route_id labels. Ordinary source_sha256 is only checked for truthiness, not equality with its approved route specification. Distinct row labels and frozen provider source do not independently prove distinct tests/routes ran.

Require unique owned invocation bindings and authenticate expected unit/filter/source or route/source/selection/copy parameters from raw evidence. Parse and bind actual per-unit/route results; do not treat provider assertion booleans as their own proof. This is a static acceptance-path counterexample, not an executed replay test.

## Remaining scope and evidence

The 158-ID crosswalk names handlers, constructor parameters and required assertion selectors. It does not implement the 158 recipes. Workspace/ordinary provider implementations, latest shared/decoder/quota/CP4 reconciliation, exhaustive final unit/route manifests, final Rust pin/binary and exact runtime release remain absent or unbound as the author states. Missing providers remain distinct from the validator defects above. No broad workspace label, handler count, raw v34 finite-control result or outer guard closure substitutes for engine qualification.

Useful implemented primitives include pre-activation owner registration, withheld-input native proof, monotonic leases, bounded file slots/retained prefixes, explicit clock6 intervals, source-bound fixed-extent patching, bounded copies and oracle inventory comparisons. They remain unqualified as an assembled facade. Recipes must also fit Events' 262,144-byte row bound and the finite phase/RSS budgets; no full-inventory/158-case sizing proof was supplied.

No candidate imports, native probes/controls, engine/build/test actions, source/fixture edits or subagents were used. Gzip/source bytes were decoded and hashed read-only. Structured-data guidance remains PROPOSED; no migration/default authority is inferred.

Review artifacts are restricted to this report and evidence.json, combined <=12,288 bytes. Fresh whole META10 membership censuses precede both writes with >=256,000 bytes future reserve, without deletion/relabel/expiry credit. Prior v34/v35 packets and negative evidence remain retained. The final five-file hash check preceded source-read release; no source rereads followed.
