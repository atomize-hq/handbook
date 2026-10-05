# Packet 1 implementation decisions

The human approved the one-packet implementation and Packet-1 review boundary
after the plan at `65a3328972faf08d5cd08d09231a964955f845f4`. This checkpoint
records the slice-delegated encoding choices; it supplies no runtime approval,
scope expansion or successor authority. Contracts JSON remains fixed; the general
structured-data defaults remain proposed. The material choices received one
focused causal APPROVE with no P1/P2 findings before product edits.

## Selected contracts

`ProjectAdmissionEstablishmentV3` owns preparation, establishment, original
request resolution and exact current resolution. `PreparedDeferredGovernanceV3`
is neither cloneable nor serializable. Its complete `DeferredEstablishmentLocatorV3`
contains the full request and request fingerprint; restoration grants resolution
only. No new Engine identity, initialization claim or registration is allocated.

The request retains `ProjectBindingV2`, exact scope and expected D outcome/relation,
the existing configuration and replacement terms, request key and a new explicit
human approval assertion. Its approved-content fingerprint binds P/S/D,
configuration, terms and the fixed establishment/owner rule bindings. Claimed
principal/agent/delegation remain local advisory assertions. Nullable claim keys
are present in the complete new JSON shape. V1/v2 fields and fingerprints do not
change; there is no implicit ceiling.

The shipped establishment descriptor uses `@3.0.0` and binds the original v2
rule. Configuration remains in its original `@1.0.0` domain. Request,
approved-content, instruction, validation, advisory permission, outcome,
current relation, intent/decision/terminal and selection/closure domains use
explicit v3 names in source. The G basis contains its complete locator so bounded
wire validation can verify every identity without claiming native delivery.

The SDK adds the four purpose-named responsibilities in the slice. V3 scope
results distinguish `EstablishedV1`, `DeferredV2` and
`EstablishedFromDeferredV3`; the last retains v2 project registration. Existing
v1/v2 signatures and discriminants retain their meanings. No catalog entry or
ordinary qualification, evaluation, action, transport or onboarding route is added.

## Retained lifecycle and exact replacement

All original D/registration/anchor/claim/receipt/lock records remain unchanged.
New request-addressed history is under the existing v2 `requests/<fingerprint>`
directory. Additive `v2/establishment-v3/<sequence>/selection.json` retains the
complete locator, previous selection and previous terminal closure. Each attempt
has an independent `closure.json` binding its exact selection and terminal bytes,
including noncommits. Earlier attempts must be completely closed noncommits;
there is no successor after G. Contiguous selections, exact inventory, request-key
uniqueness and all required records are checked. The lifetime limit is 4,096
requests including original D; history is never expired or pruned.

Request-directory/request retention precedes complete selection, then seven exact
staged records, intent and one decision precede installation. An interruption
before complete selection leaves unbound partial evidence: resolution is uncertain
and a new request is blocked. A complete selected predecision prefix may close
only that attempt as NotCommitted while retaining current D and initial-choice
consumption. Missing required selected request or closure-linked terminal evidence
cannot fabricate noncommit or retry eligibility.

Execution uses the existing shared Contracts process mutex, root guard and native
governance lock, in that order. Read-only preparation uses record snapshots and
creates no locks. Execution validates original Engine terminal/receipt and native
lock generations under locks and rechecks exact D immediately before decision.
The new path inspects lock presence by metadata instead of reading held Windows
byte-range lock contents. It calls no ordinary setup, registry or generic Context
Resolution authority under governance locks.

Engine's narrow `displace_exact_durable` extracts the existing snapshot-checked
first rename. The existing replacement method preserves its path validation and
uses the extracted step followed by no-replace publication. The owner retains
exact prepared G and displaced D, with a real fault/crash boundary between those
renames. Recovery requires the exact original decision and legal prefix; missing
current by itself chooses nothing. Immutable G installation, current publication,
terminal and selection closure complete durably before success.

Original D and terminal G replay independently validate immutable history and
never repair current. V3 current resolution verifies the whole applicable chain
and exact selected relation; incomplete decided recovery finishes only original G.
The v2 current resolver and the bounded frozen predecessor inventory reader refuse
the durable v3 footprint even after a positive noncommit. Unknown future inventory
continues to refuse. The frozen gate is source-pinned at the approved predecessor
and executes the real retained-root inventory checks, not new enum decoding.

## Verification boundary

The implementation task owns focused native types/repository/process, typed SDK,
actual reset/reopen, selected Windows contention, Engine initialization/identity/
durability, and direct replacement-caller regressions. Final counts, causal review
and local implementation commit are reported by the task and packet evidence below.
The meta task alone performs aggregate slice validation and the human recap.
Unix-specific branches are inspected only unless execution is separately reported.

Final-source verification executed on Windows:

| Focused target | Passed |
| --- | ---: |
| Contracts v3 type/rule fixtures | 13 |
| Contracts v3 native repository | 13 |
| Contracts v3 subprocess | 6 |
| Contracts v3 lifetime-capacity unit | 1 |
| SDK v3 typed consumer | 10 |
| Existing Contracts v1/v2 admission targets | 58 |
| Engine durability / initialization / invocation identity | 6 / 15 / 7 |
| Existing SDK v1/v2 admission | 11 / 9 |
| Actual SDK `setup::behavior_tests` | 12 |
| Direct Contracts exact-replacement caller regressions | 3 |

All listed targets passed with zero failures. The subprocess and SDK targets
include their worker entrypoints. One process test exercises all 26 durable crash
boundaries; the native fault matrix exercises the corresponding error boundaries.
The SDK contention test executes both decision and actual-current-displacement
pauses against current readers, recovery, actual Refresh/reset and direct Engine
identity setup. Ordinary/v1 rejection uses matching self-consistent untrusted
bindings, and original deferral noncommit uses its actual project binding, so these
cases exercise native eligibility rather than a different-root refusal.

The preparation command's `setup_tests` selector selected no tests. The actual
`setup::behavior_tests` selector supplied the 12 executed setup checks above.
Three-crate `cargo check`, exact touched-source `rustfmt --check` and staged Git
whitespace checks passed. The Engine gap extraction's direct caller regressions
cover compare/replay, injected old-or-new head truth, and exact intermediate
recovery with third-state refusal.

Selected failures were reproduced and repaired within this packet. Native
preparation stopped raw-reading held Windows lock contents and instead uses
metadata presence followed by locked generation validation. SDK contention's
snapshot helper likewise treats held empty native locks through checked metadata,
then compares their bytes after workers exit; completed-result markers prevent
partial JSON reads. The Windows reparse fixture normalizes its `mklink` arguments.
The repaired isolated cases and full final targets passed.

GitNexus symbol impact could not resolve the exercised symbols in the stale,
foreign index with missing query tables. Staged change detection reported the
exact 15 packet files with no indexed overlap, explicitly not a clean-tree result.
Comparison with `main` reported critical risk across 1,091 branch files / 300 flows;
the user was warned. This branch-wide result supplies no reliable packet impact
certificate. Direct source/caller review, exact staged inventory and the executed
focused tests provide the bounded evidence. Independent untracked research was
preserved and excluded from staging.

Unix branches were inspected but not executed. The previously recorded broader
Windows owner-suite P3 advisory remains outside these selected passing targets;
no full-workspace, transport or aggregate slice validation is claimed.

The final focused causal review returned **APPROVE**, with no unresolved P1 Critical
or P2 Required findings, covering all eight acceptance conditions against stable
source, test bodies and the reported execution results. Its P3 refusal-test coverage
observation was repaired and reviewed. The local implementation commit is returned
in the task result; the selected human review boundary remains with the meta task.
