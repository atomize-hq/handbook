# HCM-3.2 Selector-Admission Discovery Remediation

Parent orchestration: `20260801T202515Z--HCM-3-2--context-resolution-kernel`
Integrated outcome: `hcm-3.2-context-resolution-kernel-full-slice`
Packet: `HCM-3.2-P2-kernel-implementation`
Stage: implementation
Discovery dispatch:
`20260802T002238Z--HCM-3-2--context-resolution-selector-admission-review`
Discovery subject:
`sha256:62193f0c324bd89d46a286e5af17fb66a311fc340c97a9d9fbecb98fc98c8848`
Reviewer: `/root/hcm32_selector_admission_review`
Verdict: FINDINGS

## Consolidated disposition

The review emitted one P1 and four P2 findings. RED tests, Rust edits, and
implementation commands remained prohibited. This is one consolidated
same-stage remediation; it does not reopen planning, reset the parent/outcome/
packet/budget, or create another discovery cycle.

| Finding | Priority | Disposition |
|---|---:|---|
| `HCM32-SEL-DISC-001` | P1 | Every uncached assertion challenge now binds a fresh 32-byte nonce from the engine's existing `getrandom` dependency. Entropy failure refuses. Cross-resolver captured-response refusal for zero and nonzero counters is mandatory proof. In-resolver exact cached replay remains idempotent. |
| `HCM32-SEL-DISC-002` | P2 | Promotion construction now consumes the actual `ContextResolutionEnvelope` and `ContextResolutionStackDefinition`, requires a strict higher target-memory rank, and uses typed `ContextResolutionSemanticMemoryTarget`/`Record` boundaries. The disposition binds observed current fingerprint and enforces applied/refused/stale compare-and-write shape without durable HCM-3.4 authority. |
| `HCM32-SEL-DISC-003` | P2 | The selector now freezes one exact subject for every authority use. In particular, `parent` authenticates the proposed child candidate, `source` covers every source plus the source envelope, `target` covers the typed target candidate, and `target_memory` is outcome-specific. |
| `HCM32-SEL-DISC-004` | P2 | The SPEC now names the same exact 24 public exports required by the decision signatures, including binding/admission, effect/outcome, and typed semantic-memory names. No unlisted export is permitted. |
| `HCM32-SEL-DISC-005` | P2 | The SPEC and selector now require real-path positive/negative admission proof for artifact/currentness/profile/stack/head/fingerprint/mapping/credential/authenticator/counter/replay/freshness behavior and all eleven exact subjects, plus typed promotion/CAS/forbidden-authority proof. |

## Security boundary after remediation

Untrusted inputs are the selected artifact bytes, caller subject bindings,
mutation selectors/targets, and native authenticator response. Protected assets
are root/parent envelope authority, mutation reach, terminal escalation
authority, and semantic-memory promotion. Exact closed parsing and size bounds
address tampering and denial of service; current repository/profile/stack/
registry bindings address stale authority; signed challenge fields plus fresh
nonce address spoofing and captured replay; opaque exact-subject admissions and
one-terminal registries address elevation and duplicate effects. No sensitive
credential material enters fingerprints or errors.

## Remaining gate

A different fresh reviewer must replay the remediated subject and close all
five finding IDs under the same implementation stage and causal budget. CLEAN
authorizes only the frozen RED/GREEN packet. Any remaining or remediation-
unmasked P1/P2 keeps implementation stopped.
