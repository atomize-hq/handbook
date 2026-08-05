# HCM-3.3 PG-PROJ-01 discovery remediation

## Causal binding

- parent orchestration:
  `20260805T184000Z--HCM-3-3--pg-proj-01-proof-obligation-remediation`;
- integrated outcome:
  `hcm-3.3-pg-proj-01-multi-envelope-proof-remediation`;
- discovery run: `/root/hcm33_proj01_discovery`;
- discovery dispatch:
  `20260805T184200Z--HCM-3-3--pg-proj-01-proof-obligation-discovery`;
- finding: `HCM33-PROJ01-DISC-001` (`P2`, `major`,
  `local_remediation`).

## Finding validation

The finding is valid. The pre-repair `SPEC.md` required repeated deterministic
results for identical exact inputs, but did not require one fixed source,
resolved profile, vocabulary, and `ProjectionDefinition` to be evaluated under
two authorized envelopes. The pre-repair plan likewise allowed different
source or definition inputs to masquerade as Resolution effects and omitted a
cross-envelope fingerprint/accounting comparison.

No canonical Projection semantic change is needed. The defect is limited to
the future implementation proof obligation and is repairable inside the exact
planning subject.

## Consolidated parent remediation

`SPEC.md` and `tasks/plan.md` now require one explicit multi-envelope matrix:

1. freeze byte-identical source truth and exact source/profile/vocabulary/
   definition pairs, operation, surface, purpose, and currentness inputs;
2. vary only at least two exact, independently authorized envelope pairs;
3. execute two identical deterministic replays for each envelope;
4. require complete included/omitted/`not_applicable`, proof-effect,
   derivation, disclosure-evaluation, lossiness, output, and result accounting;
5. compare envelope-caused differences and explicitly account for output
   fingerprint equality or distinction according to the fixed rules;
6. require complete provenance/fingerprints and before/after equality of the
   source bytes and source fingerprint with `authority_effect: none`; and
7. preserve reveal/derive-only, no-synthesis, collapse, and authorized
   broader-request-or-escalation boundaries.

Changing any non-envelope input invalidates the proof case. Both envelopes
must be authorized before request construction; the engine may not widen its
own authority.

## Scope and proof disposition

The remediation changes only `SPEC.md`, `tasks/plan.md`, and this parent-owned
proof record. It does not alter the completed parent, historical decision or
todo, code, tests, fixtures, profiles, schemas, registries, validators,
templates, tools, dependencies, APIs, versions, Snapshot Memory, HCM-3.2,
HCM-3.4+, adopters, consumers, transports, gates, protected checkouts, or
remote state.

`PG-PROJ-01` and `PG-PROJ-02` remain open. This record proves only that the
planning obligation was repaired and must receive a different-fresh,
delta-focused closure before the corrective parent can close.
