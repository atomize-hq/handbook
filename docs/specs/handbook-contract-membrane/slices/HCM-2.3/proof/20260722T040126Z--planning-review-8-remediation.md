# HCM-2.3 Planning Review 8 Remediation

## Review authority

- reviewer: `/root/hcm_2_3_planning_review_8`
- verdict: `CHANGES_REQUIRED`
- admitted subject: 30 paths, aggregate
  `sha256:2e0c96f474994a0a0a0f36040708599cf5faa5ead2506595036c6c22d7775cef`
- findings: zero Critical, two Required, zero Optional, zero Nit
- disposition: both Required findings were accepted; neither was waived

## Exact dispositions

| Finding | Accepted defect | Remediation |
|---|---|---|
| Review 8 Required 1 | the HCM-2.3 phase-map row called the selected planning subject a review-clean packet before any complete subject had reached CLEAN | replaced the premature status with the exact live state: planning and fresh review are selected, the subject is not yet CLEAN, and implementation remains unauthorized |
| Review 8 Required 2 | the same phase-map row assigned stable generic CLI/SDK operations to HCM-2.3 even though the packet reserves SDK composition and publication to Phase 4 | limited the HCM-2.3 owner claim to engine-owned typed operations and stable generic CLI commands, and stated explicitly that SDK composition remains Phase 4 |

## Parent schema-boundary hardening

The parent audit also found that JSON-Schema `$` permits a match immediately
before a final line terminator. That behavior could admit an exact ref, safe
path, digest, record ID, transaction ID, instance ID, or output token whose
last byte was not part of the intended grammar. Every one of the 58 `pattern`
constraints across the three HCM-2.3 schemas now combines its terminal anchor
with an actual-end negative lookahead. This is still portable Draft 2020-12
ECMA-262 regex syntax and changes no positive grammar.

Twelve concrete JSON-Pointer attacks freeze that boundary: two selection
exact-ref/path cases, four runtime exact-ref/safe-ref/digest/record-ID cases,
and six control exact-ref/safe-ref/digest/transaction-ID/instance-ID/output-token
cases. SPEC, runtime contract, plan, and todo now name final-line-terminator
rejection explicitly.

## Verification replay

Independent duplicate-safe Draft 2020-12 and semantic replay reported:

- all 355 control-pack JSON documents parse without duplicate members or
  non-finite constants;
- all three HCM-2.3 schemas pass meta-validation;
- all one selection, four runtime, and 39 control positive records validate;
- all 52 schema-boundary mutations replay as declared: selection 12 reject/4
  accept, runtime 14 reject/4 accept, and control 14 reject/4 accept;
- all twelve final-line-terminator attacks reject;
- selection normalization still reproduces
  `sha256:3a5c945e68c29fa765bc4568ff4eae1eacbd84a9f90a71a00a56bb5d04cc82c9`;
- all four runtime identities reproduce, including promotion
  `sha256:d3dfb97fa1e24ac8a16651fa733cf60d83bdae4ecdcaddce1c73c263146ff8ae`
  and its 1,933-byte persisted representation; and
- all 39 identity-bearing control positives reproduce.

## Scope and next review

The remediation changed only the HCM-2.3 phase-map status, HCM-2.3 contracts,
vectors, SPEC, plan, todo, and this proof record. It did not edit Rust, Cargo,
runtime tests, production assets, HCM-2.2 authority, a handoff ledger, or a
preservation location. The repaired complete documentation subject requires a
different fresh isolated read-only reviewer and remains unauthorized for
implementation or commit until that complete subject is CLEAN.
