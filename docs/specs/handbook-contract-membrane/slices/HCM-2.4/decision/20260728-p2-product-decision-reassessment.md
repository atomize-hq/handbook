# P2 Environment Context product-decision reassessment

Status: recommendation only; explicit human product decision required

Date: 2026-07-28

## Decision boundary

This analysis is planning-only. It does not select a new applicability
authority, change a released definition or shipped profile, authorize runtime,
or make P2 GREEN. Option C containment remains active until a later exact human
selection says otherwise.

The product objective released for P2 is deterministic, fail-closed
Environment Context applicability from authoritative condition evidence.
Native WebAuthn/FIDO/CTAP ceremony was a later HCM-2.4 planning choice, not part
of that released objective.

## Current product experience

The default CLI selects exact
`handbook.profile.shipped-root@1.2.0`. That profile conditionally selects
Environment Context, but the live resolver has no released evaluator authority
and returns:

- condition outcome: `unresolved`;
- reason: `EvidenceContractUnavailable`;
- applicability: `indeterminate`; and
- closure fingerprint: none.

For the missing or otherwise structurally valid baseline, profile readiness
therefore becomes `INDETERMINATE`. Setup selects
`condition_indeterminate`, writes no selected artifact, renders
`OUTCOME: INDETERMINATE`, and exits 1. Doctor renders the same repository
outcome and exits 1. Invalid, unsafe, or unreadable selected artifacts take the
higher-precedence `INVALID` readiness path and still exit nonzero.

If `.handbook/project/environment.yaml` is absent, its row is
`not_inspected / conditional_evidence_unavailable_path_missing`. If a
structurally valid file exists, its row becomes
`structurally_valid / conditional_evidence_unavailable_path_present`, but
applicability and repository readiness remain indeterminate. Users therefore
cannot resolve the state by creating Environment Context, editing
`applicability_basis`, or toggling profile state. A malformed or unsafe
artifact can make the repository INVALID, but no current shipped-root 1.2 state
can make it READY.

## Comparison

| Option | Product result | Can a repository reach usable READY? | Product direction |
| --- | --- | --- | --- |
| A. Indefinite fail-closed | Every non-invalid shipped-root 1.2 repository remains indeterminate; invalid input remains invalid | No | Baseline/non-solution |
| B. Default-profile successor without Environment Context | Charter and Project Context determine readiness; Environment Context is deferred | Yes | Simplest recommended direction, subject to human selection |
| C. Project-owner declaration | A complete, current owner declaration determines true or false | Yes, if an exact owner authority and verifier contract are approved and implemented | Viable only after explicit authority amendment |
| D. Named existing external/already-owned authority | A versioned verifier consumes exact positive and negative evidence | Yes, if a real authority with complete coverage is named | Viable only when such an authority already exists |
| E. Existing Charter operational-reality evidence | Current Charter evidence may support true; absence remains unproved | Only for positive-evidence repositories | Supporting input, not a complete solution |

## A. Keep indefinite fail-closed indeterminate behavior

- **Product experience:** The current behavior continues. Setup and doctor
  report `INDETERMINATE` and exit nonzero for shipped-root 1.2 when Environment
  Context is missing or structurally valid. Invalid, unsafe, or unreadable
  selected artifacts instead report the higher-precedence `INVALID` state and
  also exit nonzero.
- **Security/fail-closed posture:** Strongest conservative posture. No
  applicability is guessed, self-declared, or derived from silence.
- **Implementation and operational burden:** No immediate implementation
  burden, but permanent operator burden is high: every default repository has
  a readiness state it cannot repair.
- **Released-contract changes:** None.
- **Migration/versioning:** None. Immutable shipped-root 1.1 and 1.2 remain as
  they are.
- **Program effect:** P2 stays unresolved and not GREEN. P6 remains blocked
  until P2 and P4 are GREEN; P7 and Phase 2 remain blocked behind P6.
- **Usable READY:** No for any repository selected through the current default
  profile.
- **Smallest valid next packet:** A bounded product-decision packet choosing a
  different option. Repeating runtime diagnostics or native feasibility work
  cannot close the authority gap.

Option A is the safety baseline and a valid temporary containment state. It is
not a recommended release state or product solution.

## B. Postpone Environment Context in a new default-profile successor

- **Product experience:** A new shipped default selects the existing Charter
  and Project Context instances but does not select Environment Context.
  Repositories with valid required selected artifacts can reach READY. Existing
  Environment Context files are not deleted or migrated; they are simply not
  selected by the successor.
- **Security/fail-closed posture:** The product does not guess Environment
  Context applicability. It removes the unresolved conditional requirement
  from the new default until an evaluator authority exists. Users of immutable
  1.1 or 1.2 retain their exact fail-closed behavior.
- **Implementation and operational burden:** Lowest burden among usable
  alternatives. A later implementation would add and admit one exact profile
  definition/fingerprint, cut the hardcoded default selector to that exact
  ref, and replay profile/readiness/CLI compatibility proof. It requires no
  evidence producer, trust system, native ceremony, or device operation.
- **Released-contract changes:** This changes the approved HCM-0.6
  three-instance shipped-default product decision. It therefore requires an
  explicit human authority amendment; this analysis cannot select it.
- **Migration/versioning:** Shipped-root 1.1 and 1.2 must remain byte-immutable.
  The change must be an additive exact profile successor with its own version
  and fingerprint; exact-ref selection means there is no implicit latest or
  fallback. The profile contract supports a complete replacement
  `artifact_instances` list and requires exactly one always-required
  constitutional root, which the retained Charter instance supplies. Existing
  Charter and Project Context kind/intake/renderer authority from 1.2 can be
  preserved unchanged. Removing a selected instance is product-incompatible
  with the approved three-instance default even though the profile schema can
  represent it. Current policy does not itself decide whether the successor
  label is `1.3.0` or `2.0.0`; a later packet must freeze the exact ref,
  fingerprint, compatibility statement, and CLI cutover.
- **Program effect:** B makes the user-facing default usable but does not make
  the current evaluator objective GREEN. A selecting decision must explicitly
  rebaseline P2 as deferred/superseded for the new default and amend the P6
  entry/Phase 2 exit gates; otherwise the current P2→P6 block remains. P4 still
  needs its own GREEN proof, P7 remains behind P6, and no Phase 2 completion is
  implied.
- **Usable READY:** Yes, for the new successor when its retained Charter and
  Project Context requirements are satisfied.
- **Smallest valid next packet:** A planning-only human-decision packet that
  selects postponement, freezes the exact successor profile ref and two-instance
  descriptor list, states the 1.1/1.2 compatibility boundary, and explicitly
  rebaselines P2/P6/P7/Phase 2 gates. Runtime publication/cutover would require a
  later separately selected high-risk implementation packet.

## C. Accept a project-owner declaration as authoritative evidence

- **Product experience:** A project owner submits an affirmative current
  declaration that qualifying managed operational responsibility either exists
  (`true`) or does not exist (`false`). A valid declaration plus the applicable
  artifact state could allow deterministic readiness.
- **Security/fail-closed posture:** Potentially fail-closed only if the product
  expressly treats a named owner identity as authoritative and verification is
  complete. Missing, expired, revoked, malformed, conflicting, or
  unverifiable declarations must remain indeterminate; silence must never mean
  false.
- **Identity and ownership:** A later authority amendment must name the exact
  already-approved project-owner identity or credential that owns or signs the
  declaration and the verifier that establishes control of it. The existing
  repository identity is useful subject data but is not, by itself, a released
  cryptographic signer authority. If no existing owner credential/key/issuer is
  named, C would invent a trust system and is not implementable under this
  option.
- **Subject binding:** Canonical declaration bytes must bind the repository
  identity, project identity, exact current Charter authority/result
  fingerprint, condition definition ref, selected profile ref, and declaration
  sequence or current-head identity. Cross-repository and cross-project replay
  must fail.
- **True/false semantics:** `true` is an affirmative owner statement that at
  least one qualifying responsibility exists. `false` is a separate
  affirmative owner statement that no qualifying responsibility exists after
  the required review. Omission, unchecked flags, or artifact absence are
  neither value.
- **Freshness/reconfirmation:** The authority amendment must freeze issuance
  time, expiry or maximum age, and reconfirmation triggers. At minimum, a change
  to the bound repository/project identity, current Charter result/head,
  relevant operational facts, owner identity, or condition definition makes
  the declaration stale until reissued.
- **Revocation/change:** A later declaration must supersede the earlier
  sequence/current head atomically. Revoked credentials, withdrawn
  declarations, rollback, forks, or missing maximal current heads produce
  refused or indeterminate results, never fallback to an earlier favorable
  declaration.
- **Contradiction handling:** Concurrent true/false declarations, conflict with
  stronger admitted evidence, identity mismatch, or unverifiable revocation
  state must have explicit precedence. Without a single valid maximal result,
  the evaluator returns refused/indeterminate.
- **Audit and fingerprint:** Retain canonical declaration bytes, signer/owner
  identity, subject bindings, timestamps, sequence/current-head reference,
  verification result, revocation/currentness observations, outcome reason,
  and a deterministic closure fingerprint.
- **Hardware/native ceremony:** Hardware can strengthen proof that a credential
  was controlled during signing; it cannot prove the factual existence or
  global absence of managed operational responsibility. A native ceremony is
  neither necessary nor sufficient if an already-owned verifiable declaration
  authority exists. Without such an authority, hardware feasibility does not
  close the product trust decision.
- **Implementation and operational burden:** Medium to high. It needs
  declaration lifecycle, verification, currentness, revocation, contradiction,
  audit, and recovery behavior, plus owner operational procedures.
- **Released-contract changes:** C explicitly amends released authority by
  making an owner declaration an admitted authoritative source. It requires a
  later human selection and versioned evaluator/contract successor.
- **Migration/versioning:** Existing 1.1/1.2 profile bytes stay immutable.
  Exact successor definition/evaluator refs, canonical record formats,
  compatibility behavior, and current-head rules are required. Existing
  declarations cannot be inferred or backfilled.
- **Program effect:** After authority selection, implementation, complete
  positive/negative proof, and independent review, P2 could become GREEN and
  unblock its side of P6. P4 must also become GREEN; P7 and Phase 2 remain
  behind P6.
- **Usable READY:** Yes only after an exact owner authority is named and the
  complete lifecycle is implemented. The current repository contains no
  selected authority that makes C usable today.
- **Smallest valid next packet:** A planning-only authority decision naming the
  exact owner identity/credential and verifier, then freezing the fields and
  lifecycle above. If no existing identity authority can be named, stop rather
  than design a new trust system.

## D. Use a named existing external or already-owned evidence authority

- **Product experience:** The product verifies evidence already issued by a
  named authority and deterministically derives true or false. Users do not
  perform a Handbook-specific native ceremony, but must keep the external
  evidence current.
- **Security/fail-closed posture:** Strong if the issuer and complete closure
  are real and exact. Unnamed issuers, partial evidence, stale state, replay,
  ambiguity, or contradiction remain indeterminate.
- **Required authority detail:** A valid successor must name exact issuer and
  verification key or key-discovery authority; repository/project subject
  mapping; complete positive and negative evidence; canonicalization and
  signature rules; issuance, expiry, revocation, and freshness; replay and
  current-head rules; contradiction precedence; and deterministic closure/audit
  fingerprints. An API name or general enterprise system is not enough.
- **Implementation and operational burden:** Medium. A verifier-only path can
  be smaller than a producer plus native adapter, but issuer integration,
  key/currentness operations, negative coverage, outages, and rotation remain
  operational dependencies.
- **Released-contract changes:** A versioned authority/evaluator successor must
  admit the exact evidence source. It may not silently weaken HCM-0.6.
- **Migration/versioning:** Shipped profiles 1.1/1.2 remain immutable. The
  evidence and evaluator refs, key policy, canonical formats, compatibility,
  and failure behavior need exact new versions. No existing evidence is
  admitted until it passes the successor contract.
- **Program effect:** If complete true/false authority is selected and proven,
  P2 can become GREEN and later unblock P6 together with P4. P7 and Phase 2
  remain behind P6. If the authority covers only positive cases, the result is
  equivalent to E and cannot complete P2 alone.
- **Usable READY:** Yes after a real complete authority is named and integrated.
  No such exact issuer/key and full-coverage source is named in current
  authority, so D is not presently actionable.
- **Smallest valid next packet:** An evidence-authority nomination and
  compatibility audit that proves the exact existing source already supplies
  all required fields and both outcomes. Only after that proof may a later
  human decision select a bounded verifier-only implementation.

This option must reuse a real external or already-owned authority. It must not
invent a Handbook trust system under the label “verifier-only.”

## E. Narrow use of existing Charter operational-reality evidence

- **Product experience:** A repository whose current approved Charter chain
  affirmatively records qualifying operational responsibility could receive
  positive supporting evidence for condition true. Other repositories remain
  indeterminate.
- **Security/fail-closed posture:** Strong if the evidence is consumed only
  through the complete released Charter promotion/result/currentness chain and
  exact current canonical bytes. It must never treat absence, silence, or an
  omitted Charter field as global proof of false.
- **Implementation and operational burden:** Lower than C or D because it
  reuses existing owned evidence and currentness behavior. Burden remains for
  exact semantic mapping, current-result binding, contradiction precedence,
  closure fingerprinting, and preservation tests.
- **Released-contract changes:** The current released condition does not name
  the exact admitted Charter mapping/evaluator. A versioned authority amendment
  is still needed to admit it as positive evidence.
- **Migration/versioning:** Existing Charter authority and shipped profile bytes
  remain unchanged. Any evaluator successor must bind exact Charter
  schema/result/profile refs and preserve the released promotion and
  currentness chain. No new meaning may be assigned to historical silence.
- **Program effect:** E alone cannot make P2 genuinely GREEN because it lacks
  complete negative evidence. It cannot generally unblock P6, P7, or Phase 2.
  A later product decision could accept partial positive readiness, but that
  would itself rebaseline the released objective and exit gates.
- **Usable READY:** Yes only for repositories with complete current positive
  Charter evidence and a valid Environment Context artifact after a selected
  evaluator exists. Repositories without that evidence remain indeterminate.
- **Smallest valid next packet:** A planning/proof packet mapping exact existing
  Charter operational-reality fields through promotion, result, and currentness
  to condition true, while proving no valid path to condition false. Return to
  a human decision on whether partial coverage is acceptable.

The established boundary remains: existing Charter operational-reality
evidence can support presence, but it cannot prove global absence by itself.

## Recommendation

Recommend **B: an additive default-profile successor that temporarily
postpones Environment Context**.

It is the smallest direction that restores a usable READY state without
guessing applicability or creating a new trust, evidence-production, native,
device, or operational system. It can preserve the existing Charter and
Project Context authority exactly and leave shipped-root 1.1/1.2 immutable.

This is a recommendation, not a selection. The next human decision must
explicitly authorize the HCM-0.6 default-profile change, freeze the exact
successor version/fingerprint and two-instance list, state how old exact
profiles remain supported, and rebaseline P2/P6/P7/Phase 2 gates. Until that
decision is made and later implementation is separately authorized, Option C
containment remains in force. The missing/structurally-valid baseline remains
`INDETERMINATE`; invalid selected artifacts remain `INVALID`; neither can reach
READY.
