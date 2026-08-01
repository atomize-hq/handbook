# HCM-3.2 Planning Discovery Authority Stop

## Reviewed subject

Fresh run `/root/hcm32_planning_review` reviewed the initial four-file HCM-3.2
planning subject at aggregate fingerprint
`sha256:184c9746e080fa09498ce3700e9354a9895f8eaf00abd4bfd587c3664b4f915d`.
The immutable dispatch is
`20260801T202751Z--HCM-3-2--context-resolution-planning-review`.

## Findings and consolidated disposition

| Finding | Severity | Parent disposition |
|---|---|---|
| `HCM32-PLN-DISC-001` | P1 | Confirmed authority boundary. The live contracts provide semantic authority refs but no admitted bytes/authentication resolver. A generic replacement is outside the slice. Stop before implementation. |
| `HCM32-PLN-DISC-002` | P2 | Confirmed. Resumption must freeze candidate class to exact trigger, missing condition, requested authority, and evidence mapping. Core trigger-class strings are insufficient. |
| `HCM32-PLN-DISC-003` | P2 | Closed in planning text: containment compares positive parent allow language; inherited denies accumulate and subtract separately. |
| `HCM32-PLN-DISC-004` | P2 | Confirmed. The public-name list is not callable authority. Resumption must freeze exact signatures and symbol counting. |

GitNexus independently reports HIGH impact for
`ContextResolutionStackDefinition` and CRITICAL impact for
`AuthoredStack::resolve` and `ContextResolutionStackDefinition::load_bytes`.
The initial LOW/MEDIUM risk ceiling cannot authorize the configurable-stack
edit required by the slice.

## Why implementation cannot safely continue

Shape validation, exact fingerprints, and a one-terminal registry can prove
internal coherence, but they cannot prove that a caller-supplied authority,
decision, evidence, trigger, constraint, source, or target pair is admitted or
authorized. Accepting such pairs would permit fabricated root creation and
apparently approved escalation/promotion. Adding a generic authority schema,
catalog, authentication token, or framework would exceed the explicit
no-new-framework/schema authority boundary.

No Rust file, test, fixture, Cargo manifest, shipped definition, profile, or
pipeline file was edited. No implementation check was run.

## Exact resumption decision

An exact reviewed authority ref must choose an admitted authority model (or a
narrowed proof claim), name its owner and bytes/fingerprint/authentication
rules, enumerate exact callable signatures and negative tests, map all
candidate conditions, and explicitly accept or avoid the named CRITICAL stack
symbols. The decision remains HCM-3.2-scoped and grants no HCM-3.3+ authority.

## Closure gate

A different-fresh closure reviewer must verify all four predecessor findings
are honestly closed or converted into the exact durable authority stop above.
The reviewer must not implement, invent the missing authority, or relabel the
stop as completed proof.
