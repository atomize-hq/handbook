# HCM-0.8 Post-CLEAN Authority Continuation Control Repair

Status: proposed; implementation forbidden until fresh planning review is CLEAN

## Objective

Add the smallest v1.4-only, fail-closed representation for one bounded
same-parent continuation after a non-completed authority stop whose applicable
review stages already ended CLEAN. The continuation must admit a fresh,
independently CLEAN authority-selector review before edits and retain finite
independent implementation, proof, and final-closeout review capacity without
changing the parent, integrated outcome, packet registry, finding lineage,
causal budget, or immutable dispatch prefix.

The exact regression consumer is HCM-3.2 parent
`20260801T202515Z--HCM-3-2--context-resolution-kernel`, outcome
`hcm-3.2-context-resolution-kernel-full-slice`, budget
`sha256:9e317280367a3668c484802c4cbc0e4e090faad692692e8327a40c11b73346ae`,
and handoff
`20260802T012613Z--HCM-3-2--orchestration--authority-lineage-composition-required`.
HCM-3.2 product implementation is prohibited in this slice.

## Selected mechanism

Current v1.4 dispatches gain one optional `authority_continuation` object. It
contains an immutable `grant`, a per-dispatch `review_slot`, and no new budget
identity. The grant binds:

- one exact non-completed same-parent authority-stop handoff and its dispatch
  population fingerprint;
- the unchanged parent, integrated outcome, outcome-registry fingerprint,
  causal budget, and registered packet IDs;
- one independently issued, repository-reviewed authority artifact plus
  SHA-256. Its typed provenance binds the issuer/owner identity and role,
  source task/thread/host and dispatch nonce, issuance time, predecessor,
  unchanged identities, authorized scope and symbols, and accepted risk. The
  authority artifact must predate the admission dispatch. Its trust anchor is
  not a claimed issuer string or review dispatch alone: the grant names a
  completed v1.4 handoff from a different parent and its completed CLEAN review
  run, whose immutable dispatch manifest contains the exact artifact path/hash.
  The validator replays that handoff, run, dispatch, result subject, and bytes.
  The artifact issuer, attestation reviewer, admission reviewer, and
  continuation executor must satisfy the exact role-separation matrix;
- the authority artifact's parity with an exact acyclic payload projection:
  every stable grant field except `authority_ref.path`,
  `authority_ref.sha256`, and `authority_ref.attestation`. The complete
  authority reference remains inside the fingerprinted grant and is validated
  independently;
- the complete sorted subject-path ceiling, exact symbol delta, risk ceiling,
  prose scope delta, and the immutable direct predecessor handoff's reviewed
  Git baseline commit and tree;
- exactly four ordered review slots: `authority_admission`, `implementation`,
  `proof`, and `final_closeout`;
- an exact maximum of four cycles per slot: one discovery, at most one closure,
  and at most two causal supplementals under the existing P1/P2 rules.

The exact HCM-3.2 authority artifact is added by this HCM-0.8 control increment
under its slice-local authority/proof surface, binding the user/meta task,
thread, host, nonce, predecessor, scope, symbols, and accepted risk without
performing product work. The completed HCM-0.8 closeout handoff and its final
CLEAN run become the trusted different-parent attestation only after closeout;
until then the artifact cannot admit HCM-3.2.

Construction is acyclic: serialize the exact artifact payload projection with
the already-known predecessor reviewed baseline; hash the artifact; create and
complete the different-parent CLEAN review whose dispatch manifest contains
that artifact hash; then place the artifact path/hash and completed attestation
in the full grant and fingerprint it. The artifact never contains its own
path/hash or the later attestation hash.

The first continuation dispatch must be a read-only `authority_admission`
discovery review. Its manifest must include the exact authority and authority-
review dispatch bytes. No later slot or write-role dispatch is admitted until
that review is completed CLEAN. A P1/P2 admission finding terminates that grant:
the artifact and grant are immutable, no admission remediation dispatch is
allowed, and continuation requires a newly independently issued and reviewed
artifact plus a new grant fingerprint. Because only one grant attempt is
permitted per parent, the failed attempt is a true stop rather than a way to
cycle authority.
The first dispatch may use the new typed `authority_extension` stage transition
from the predecessor's last stage; all later transitions are ordinary and
monotonic inside the grant.

The current v1.4 handoff gains an optional `authority_continuations` summary.
For each admitted grant it records the grant fingerprint, predecessor handoff,
selector run, immutable baseline, exact slot/cycle and dispatch membership,
actual changed-path/symbol/risk reconciliation, one exact GitNexus change-
detection evidence artifact and its completed CLEAN review-run attestation for
every code-bearing delta, and `active` or `consumed` status.
Old records omit the field and remain byte-identical. A completed successor
must record the one grant as consumed.

## Fail-closed invariants

1. At most one authority continuation may exist for a parent.
2. The predecessor must be the latest direct same-parent handoff, must have
   `stop_reason=authority_boundary`, and must not be completed.
3. The first extension dispatch must be later than the predecessor cutoff,
   must preserve the exact prefix predecessor, and must carry the same parent,
   outcome, registry, budget, and registered packet identity.
4. Every extension dispatch repeats one byte-equivalent immutable grant and
   fingerprint while `review_slot` varies outside the grant. Its manifest is a
   subset of the grant's sorted subject-path ceiling while including the exact
   authority and authority-review bytes.
5. `authority_admission` is read-only and pre-edit. A later dispatch is invalid
   unless the selector run is independently CLEAN.
6. Existing discovery/closure/supplemental lineage remains mandatory within
   each slot. CLEAN terminates that slot. P1/P2 cannot be waived.
7. Each cycle has exactly one dispatch; a same-cycle burst is deliberately not
   admitted. Slot order and the exact four-cycle ceiling therefore bound both
   cycle and dispatch membership. A second
   grant, a branch from an older cutoff, reuse across a parent/outcome, identity
   drift, or any dispatch after the consumed final CLEAN fails closed.
8. Frozen v1.0-v1.3 corpora and every existing v1.4 record/dispatch remain
   immutable and continue to validate.

## Exact implementation ceiling

Material authority/control paths:

- `docs/specs/handbook-contract-membrane/00-README.md`
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
- `docs/specs/handbook-contract-membrane/07-orchestration-onboarding-prompt.md`
- `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`
- `docs/specs/handbook-contract-membrane/09-review-finding-inventory.md`
- `docs/specs/handbook-contract-membrane/handoffs/internal-dispatch.v1.4.schema.json`
- `docs/specs/handbook-contract-membrane/handoffs/handoff-record.v1.4.schema.json`
- `docs/specs/handbook-contract-membrane/handoffs/internal-dispatch-template.json`
- `docs/specs/handbook-contract-membrane/handoffs/handoff-template.json`
- `docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py`
- this HCM-0.8 SPEC, plan, checklist, selector, and required proof artifacts;
- new HCM-0.8 v1.4 dispatches, final handoff, and deterministic ledger entry.

No Rust, Cargo, dependency, public API, unsafe/native/transport, shipped
identity, product-runtime, HCM-3.2 product, or HCM-3.3+ path may change.

## Validator and schema requirements

- Existing v1.4 objects without the new optional fields remain valid.
- `grant_fingerprint` is `sha256:` plus the lowercase hexadecimal SHA-256 of
  the UTF-8 bytes of RFC 8785/JCS canonical JSON for the immutable `grant`
  object, followed by one LF byte. The fingerprint field and per-dispatch
  `review_slot` are outside that object and therefore excluded. No Unicode or
  path normalization occurs beyond the schema's canonical repository-relative
  path rules. The validator recomputes this exact payload; a golden vector and
  mutation case for every stable field prove it, while slot-only changes retain
  the hash.
- Paths are canonical repository-relative, unique, and Python-ordinal sorted.
- The authority path/hash must match a manifest entry on every continuation
  dispatch; live `--verify-dispatch` replay therefore refuses stale authority.
- Authority admission resolves the named different-parent completed handoff,
  its completed CLEAN run, and that run's immutable dispatch. The run result
  fingerprint must equal the dispatch subject fingerprint, the dispatch
  manifest must contain the exact authority artifact path/hash, and all
  referenced files must replay. A claimed, missing, fabricated, incomplete,
  FINDINGS, mismatched-subject, same-parent, or role-colliding attestation fails.
- Grant `baseline_commit` and `baseline_tree` must equal the referenced direct
  predecessor handoff's reviewed baseline and the repository tree of that
  commit. For the exact HCM-3.2 fixture they are
  `fe62a4b57854830373a57ccdcc7acb9f4b7a9ac0` and
  `671e0fac370d91bb4fe57073fa63ad37f55f4c40`. A future/self-containing or
  merely valid unrelated commit/tree pair fails closed.
- Each continuation/final handoff recomputes the actual Git path delta from the
  immutable baseline and reconciles it with the grant ceiling; recorded
  GitNexus evidence must bind provider/version, exact command/scope,
  baseline/target commit and tree, actual changed paths, complete changed
  symbols and risks, aggregate observed risk, and raw-output fingerprint. The
  evidence path/hash must appear in a completed CLEAN review dispatch manifest
  recorded by the same handoff. Validator replay requires exact evidence-to-
  summary parity, exact actual-path parity, nonempty symbol coverage for a
  code-bearing delta, aggregate risk equality, and grant ceiling containment.
  Unavailable GitNexus is not GREEN. Missing/empty/fabricated/unattested
  evidence, omitted paths or symbols, understated risk, out-of-ceiling symbols,
  and excess risk fail closed.
- The causal sequence validates the immutable predecessor prefix, the one
  typed boundary transition, independent selector CLEAN, ordered slots, exact
  cycle ceilings, no cycle after slot CLEAN, and no dispatch after consumed
  final CLEAN.
- Handoff-chain validation checks direct non-completed predecessor status,
  same-parent/outcome/budget/registry/packet identity, single-use grant,
  summary parity, and completed consumption.

## Testing strategy and commands

TDD begins with failing orchestration-contract fixtures. The exact HCM-3.2
five-dispatch prefix and latest handoff are the positive predecessor fixture.
Positive replay loads a typed authority artifact issued by the recorded
meta/user authority and a separate CLEAN review of that artifact, then adds
selector admission plus later implementation, proof, and final independent
reviews without editing any predecessor byte. It constructs a schema-valid
active successor handoff directly over the committed predecessor and a
completed successor that consumes the grant.

Negative fixtures cover missing/stale authority, reused grant, cross-parent,
cross-outcome/budget/registry/packet drift, completed predecessor, non-direct
or branched cutoff, over-budget cycles, out-of-order slots, write-before-CLEAN,
unrelated actual changed path, in-path/out-of-ceiling symbol, excess observed
risk, self-authored/postdated/issuer-mismatched/ownerless authority, fabricated
or unexecuted/mismatched/FINDINGS authority review, same-parent attestation,
reviewer/executor role collision, authority parity drift, admission finding
followed by reuse, excluded-projection mutation, unrelated or self/future
baseline, missing/unattested GitNexus evidence, empty symbol coverage for code,
omitted or unauthorized symbol, understated risk, same-cycle second dispatch,
active-summary mismatch, second successor, post-consumption dispatch,
post-slot-CLEAN, post-extension-final-CLEAN, and changed immutable prefix.

Required commands:

```text
uv run --with jsonschema==4.25.1 python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
uv run --with jsonschema==4.25.1 python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
uv run --with jsonschema==4.25.1 python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
python -m py_compile docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
git diff --check
```

Run focused positive/negative fixtures first, then ordinary validation, both
self-tests, exact prefix replay, ledger parity, formatting/whitespace, scoped
and compare-to-main GitNexus change detection, and final aggregate review.

## Success criteria

- A schema-valid v1.4 continuation can admit the exact HCM-3.2 prefix, perform
  fresh selector review before edits, and retain later independent material
  review without relabeling any predecessor cycle.
- Every prohibited or identity-changing case fails closed.
- Old v1.4 records validate unchanged; frozen v1.0-v1.3 admission hashes are
  byte-identical.
- No unresolved P1/P2 remains, all proof gates pass, and the two-commit local
  closeout publishes only through expected-old compare-and-swap.

## Stop conditions

Stop on any need for a generic budget reset, historical rewrite, completed
parent continuation, second extension, P1/P2 waiver, public/dependency/runtime
surface, HCM-3.2 product edit, or unexpected HIGH/CRITICAL expansion beyond the
reviewed validator control-plane symbols.
