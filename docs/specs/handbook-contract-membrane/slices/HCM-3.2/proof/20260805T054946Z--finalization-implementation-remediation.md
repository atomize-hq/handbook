# HCM-3.2 finalization implementation remediation

Parent: `20260805T014559Z--HCM-3-2--context-resolution-finalization`

Outcome: `hcm-3.2-context-resolution-finalization-whole-slice`

Packet: `HCM-3.2-FINAL-P2-product-remediation`

Discovery subject:
`sha256:340fa5249618ec9ef9bbff6a3df63025f2783e60fa8c2097c46c5bd7918f13c1`

Finding runs:

- `/root/hcm32_final_impl_security`, dispatch
  `20260805T040003Z--HCM-3-2--finalization-implementation-security`;
- `/root/hcm32_final_impl_recovery`, dispatch
  `20260805T040004Z--HCM-3-2--finalization-implementation-recovery-state-machine`;
- `/root/hcm32_final_impl_security`, distinct compatibility lens dispatch
  `20260805T040005Z--HCM-3-2--finalization-implementation-compatibility-regression`.

The same-fingerprint burst retained stable finding identities
`HCM32-JCS-IMPL-DISC-004`, `HCM32-JCS-IMPL-DISC-005`, and
`HCM32-JCS-IMPL-DISC-006`. No new identifier replaces or waives them.

## Consolidated repair

- Exact transition, idempotency key, candidate ref, and candidate fingerprint
  admission now occurs under the generic store lock before publication.
- A refused generic result never completes HCM quarantine.
- Completion independently locates and verifies the retained ledger, committed
  promotion intent, request candidate, publisher transition, canonical output,
  domain result, and internal evidence chain.
- The open quarantine diagnostic is immutable. Completion is a separately
  fingerprinted append-only receipt published through a durable scratch rename;
  a partial scratch is safely discarded and reconstructed from committed truth.
- Cold load reconciles a missing completion only when the exact committed T2
  chain verifies; otherwise current use remains refused.
- Generic non-HCM markerless recovery remains fail-closed and now has a direct
  missing-native-result no-mutation test.
- The proof wall and limits were rewritten to report only observed post-repair
  results.

## Observed verification

The public exact-replay, different-candidate, and refused-result tests first
failed on the discovery subject. After repair they passed 1/1 in 192.34,
117.15, and 120.46 seconds. The post-T2 partial-closeout recovery test passed
1/1 in 140.79 seconds. The generic missing-native-result test passed 1/1 in
34.63 seconds.

Complete post-repair results:

- Context Resolution kernel: 31/31 in 532.37 seconds;
- generic lineage: 55/55 in 266.75 seconds;
- all `handbook-engine` tests: exit 0 in 1,187.9 seconds;
- workspace check and strict all-feature Clippy: exit 0;
- format and whitespace: exit 0;
- all workspace tests: exit 0 in 1,546.1 seconds.

Production additions are 2,010/2,400 with per-file allocations 538/600,
173/500, and 1,299/1,300. Selected test/fixture additions are 1,482/2,400 with
eleven/18 material tests. No new production path, public API, dependency,
unsafe/native/platform/transport surface, tooling, H2/H3 implementation, or
later slice was introduced.

Disposition: one different-fresh fingerprint-bound closure review is required.
