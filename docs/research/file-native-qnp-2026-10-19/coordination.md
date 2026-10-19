# Native qualification, navigation and packing increment

Human authorization on October 19, 2026 permits this standalone engine experiment
through separate fresh top-level task sessions, including review, remediation,
integration and landing. Earlier proposal wording in the consultation predates
that authorization. This is not an HCM product phase or a production migration.

Integration starts at `f0904a63cb2500b9bea3a4034d19cbe38bfac729`, with exact
Rust/Cargo core `07a8574e581e66ba58d7caac87238aa5f93918be`, on isolated
`codex/file-native-qnp-20261019`. The shared checkout and prior recovery worktree
are preserved. Existing experimental precedent lands reviewed commits on a pushed
isolated research branch. No merge into an unrelated product branch is inferred.

## Tasks and gates

| Task | Fresh top-level chat | Host | Initial state |
| --- | --- | --- | --- |
| Q Linux | `01a155e3-4af4-79a3-bc8b-77e1c54bd961` | native Linux via local coordinator | read-only admission |
| Q Mac | `01a155e3-b55b-7bc1-8cfc-453c5779a800` | native macOS | read-only admission |
| N | `01a155e3-52dc-7a53-8b44-4c1c99ba8d10` | Windows | read-only admission and design |
| P | not dispatched | primary Windows plus small native lanes | requires reviewed Q/N and qualified frozen integrated pin |

Q has two hours including closeout and a 1,800-second capsule cap per host.
N has four hours including closeout. P has six hours with the last 30 minutes
reserved for closeout. Original failed/censored attempts and deadlines remain
visible. Workers return results to this coordinator; they do not create successors.

N native deltas require separately coordinated Linux and Mac lanes after its
candidate source pin exists. Q fixes, if any, must be integrated and the actual
resulting source qualified before navigation freezes and P starts.

## Preservation and resources

The untouched H101003 original and original operational archive remain protected.
The existing final post-GC H101006 clone is read-only during Q/N. Before P performs
destructive packing, its exact baseline must be captured, independently restored,
verified and reopened, with fresh disposable authorization at the new destination.
The older H101003 archive is not an exact rollback for that state.

The human subsequently approved a 40 GiB combined owned logical-byte ceiling
after the complete cross-host census exceeded the original 32 GiB. The
5,000,000-file ceiling is unchanged. The scope includes
preserved datasets, archives, builds, stages and evidence. Reserve 1 GiB and
100,000 files inside those limits. The approved change is recorded in
`evidence/q-n-execution-admission-40gib.json`. Current census, not historical closeout totals,
controls admission. No child receives an independent allowance. Keep one active
long-history mutation lane. Free-space reserves are Windows 140 GiB, native Linux
20 GiB, WSL 40 GiB and Mac 20 GiB. Initial aggregate new-work RSS limits are 4 GiB
Windows and 3 GiB each Linux/Mac, with build concurrency two; checker/process
2 GiB and point reader 256 MiB including 64 MiB index/cache remain stricter bounds.

## Frozen research decision rules

Before any P measurement: exact byte/semantic equivalence and recovery must pass;
packing must reduce eligible-class file count by at least 95%; compression needs
at least 20% smaller pack-plus-index bytes than P0. Report both P0 versus L and PZ
versus P0, using a 2x matched warm-lookup limit with absolute distributions.
Ordinary-publication median regression attributable to packing is limited to 20%,
with a 1 ms absolute noise allowance. These are research gates, not production
SLAs. Uncompressed packs can win. Resource/deadline refusals are censors.

## Controlling sources

The [complete reconciled brief](../file-native-packing-guidance-pro-consult-2026-10-19/next-experiment-brief.md)
defines the full Q/N/P functional, measurement, compatibility and fault matrix.
The [reconciliation](../file-native-packing-guidance-pro-consult-2026-10-19/reconciliation.md)
qualifies the advisory [Pro answer](../file-native-packing-guidance-pro-consult-2026-10-19/answer.md).
The [independent recovery review](../file-native-recovery-reclamation-review-2026-10-19.md)
records existing verification and limitations. The proposed structured-data
standard does not override existing owner/versioned contracts or Contracts JSON.

Every substantive change receives coordinator review and appropriate checks.
GitNexus impact precedes symbol edits; detect_changes precedes commits. Missing
experimental symbols or stale graph evidence remains UNKNOWN with retained
attempts and explicit manual call tracing, never a clean graph assertion.
