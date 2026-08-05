# HCM-3.2 quarantine finalization proof wall

Status: the recorded mandatory implementation/proof wall was GREEN for its
bound subject. The later `HCM32-QF-IMPL-DISC-001` remediation audit below
supersedes any inference that the current complete subject is closed. The new
finding is evidence-disputed and not self-closed, no production repair was
authorized, and this record does not close HCM-3.2 or `PG-RES-01`. The
already-authorized different-fresh closure owns independent adjudication.

## Bound authority and subject

- proof dispatch:
  `handoffs/dispatches/20260805T112506Z--HCM-3-2--quarantine-finalization-proof.json`
- dispatch SHA-256:
  `b64e9cf35ab0ca65e4209373c2fc88a29d6ef24402352c5df978488141b73678`
- pre-control-doc implementation/proof subject:
  `sha256:54f2fdfb072500da361aca7fd6ae6c6dfc2b68fd46f62605520a4527e9d13833`
- checkpoint: `eb4b7ba55c6f3db40799e67275f3bbbb8610c87a`
- checkpoint tree: `d51327c442190edcf1bf56423db54fd2522c5e6f`
- required HCM-3.1 ancestor:
  `144fad13c14a64ae36f5bdf5554d9adcac10d20e`
- unchanged authoritative remote baseline:
  `7becde555bcc9742cd0ada4cff27542f521e6abb`
- protected worktree head:
  `eb4b7ba55c6f3db40799e67275f3bbbb8610c87a`
- protected status/content aggregate:
  `3493cd4963697e8c9032a92f8753b041aa18cf5e9bc7cbf4f7a522e0cee2a896`
- immutable 23-file prior source-evidence aggregate:
  `df63d7c489af87257aae38a9ef2026787d5f5908ac5898aef69cec479bc4124d`

The stopped predecessor parent, its dispatches, and its proof records are
byte-preserved source evidence only. They are not borrowed as current review or
completion authority.

## `HCM32-QF-IMPL-DISC-001` remediation reachability audit

The exact remediation dispatch is
`handoffs/dispatches/20260805T134628Z--HCM-3-2--quarantine-finalization-implementation-remediation.json`
with SHA-256
`c7708ef4cb6d1c8107c0a55e516327dc4c960a5e5b31932f127cd48b7720bca5`.
GitNexus reported `HIGH` upstream risk for
`detect_authorized_publication_gap`: one direct caller,
`load_authority_snapshot`, eight upstream dependents, two affected process
groups (`authorize_memory` and `authorize_validation`), and four affected
modules. Production therefore remained on hold until a genuine classifier RED
could be demonstrated.

No attempted mixed valid-plus-invalid matching inventory both preserved exact
generic authority and reached the quarantine reason assertion:

1. An uncited matching candidate made the otherwise-valid committed candidate
   unreadable with `artifact inventory contains a record uncited by any journal
   chain`.
2. A second honest candidate-append transaction for the same normalized outer
   was refused with `fresh immutable output must be absent before intent
   establishment`, because the content-addressed normalized-content output
   already existed.
3. Rebinding a committed historical candidate to the authorized outer made the
   committed chain fail with `installed or staged output bytes disagree with
   intent` before classification.
4. The final retained-read-hook reachability attempt produced no classifier
   assertion before the explicit 1,204-second command timeout; its exact stale
   cargo/test process tree was terminated before validation resumed.

The causal invariant is closed at the generic-lineage boundary: equal
normalized content has one content-addressed ref; a second honest append cannot
re-establish that immutable output; uncited inventory is rejected globally; and
changing a cited committed output breaks its exact intent. Consequently the
dispatched mixed state was not shown reachable at
`detect_authorized_publication_gap`. Editing that `HIGH`-risk production seam
without genuine RED would weaken generic truth, so no production symbol was
changed.

The focused regression
`matching_invalid_recovery_candidate_refuses_in_both_lexical_orders` now locks
in the observed fail-closed boundary. It places matching invalid candidate
evidence lexically before and after the valid candidate, proves both are
rejected as uncited before classification, removes each orphan, and proves the
same committed valid candidate is immediately authoritative again. Exact run:
exit 0; 1 passed, 0 failed, 32 filtered; harness 81.26s; wall 85.9s.
The proportional complete-kernel replay also passed: 33 passed, 0 failed,
ignored, or filtered; harness 593.47s; wall 598.2s. Prior engine/workspace wall
evidence is preserved exactly rather than misreported as a post-production
replay, because no production byte changed.

Final audit gates passed: `cargo fmt --all -- --check`, `git diff --check`, and
ordinary handoff validation (5 record schemas, 5 internal-dispatch schemas, 2
templates, 85 records, 508 current internal dispatches, 8 admitted legacy
dispatches, and 85 ledger entries). GitNexus scope-all reported 18 files, 192
symbols, 50 affected processes, and `CRITICAL`; compare-to-`main` reported
1,355 files, 9,929 symbols, 259 processes, and `CRITICAL`. Those are the
expected complete dirty feature and branch comparison scopes, not six-path
remediation-only risk claims. FTS remained explicitly unavailable. HEAD and
tree remained `eb4b7ba55c6f3db40799e67275f3bbbb8610c87a` and
`d51327c442190edcf1bf56423db54fd2522c5e6f`; the checkout remained detached;
the staged diff remained empty.

Disposition: `HCM32-QF-IMPL-DISC-001` is evidence-disputed and **not
self-closed**. The already-authorized different-fresh closure must
independently adjudicate whether the two-order generic-lineage invariant
evidence falsifies the claimed reachable P2. `CLEAN` closes the finding;
`FINDINGS` or `BLOCKED` leaves HCM-3.2 incomplete and only then requires a
classifier-reachable RED or new operator authority. This truth correction is
part of the same consolidated remediation packet, not another review or repair
cycle. It does not authorize commit, ref movement, push, or publication.

## Mandatory sequential proof wall

| Gate | Exact result | Measured wall |
|---|---|---:|
| `cargo test -p handbook-engine --test context_resolution_kernel different_valid_quarantine_candidate_refuses_before_publication_mutation -- --nocapture` | exit 0; 1 passed, 0 failed, 31 filtered; harness 89.37s | 89.859s |
| `cargo test -p handbook-engine --test context_resolution_kernel -- --nocapture` | exit 0; 32 passed, 0 failed/ignored/filtered; harness 631.65s | 631.881s |
| `cargo test -p handbook-engine --test hcm_2_3_generic_lineage -- --nocapture` | exit 0; 55 passed, 0 failed/ignored/filtered; harness 283.88s | 284.159s |
| `cargo test -p handbook-engine --all-targets --all-features` | exit 0; every engine unit/integration target passed | 1424.742s |
| `cargo test --workspace --all-targets --all-features` | exit 0; every workspace target passed | 1983.706s |
| `cargo check --workspace --all-targets --all-features` | exit 0 | 19.347s |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0; zero warnings | 16.236s |
| `cargo fmt --all -- --check` | exit 0 | 1.698s |
| `git diff --check` | exit 0 | 0.052s |
| ordinary handoff validation | exit 0; 5 record schemas, 5 internal-dispatch schemas, 2 templates, 85 records, 506 current dispatches, 8 admitted legacy dispatches, 85 ledger entries | 29.587s |
| v1 admission self-test | exit 0; every historical admission/ledger scenario passed | 48.652s |
| orchestration-contract self-test | exit 0; v1.4 causal and orchestration contracts passed | 7.858s |

The focused negative uses a valid anchored open quarantine record for candidate
A and partial completion scratch, supplies a different otherwise-valid
candidate B, and proves identity refusal occurs before cleanup or publication
mutation. Scratch, open record, anchor, completion state, canonical authority,
transaction state, and surrounding inventory remain byte-identical.

## Stable finding disposition

| Finding | Fresh proof disposition |
|---|---|
| `HCM32-JCS-IMPL-DISC-001` | Cryptographic substitution coverage remains closed by hash-consistent signature, RP-ID hash, flags, counter, and challenge negatives that reach the authentic verifier. |
| `HCM32-JCS-IMPL-DISC-002` | Currentness and clone/cache admission remain closed by full committed-lineage, registry, assertion/response, cryptographic, repository, profile, stack, predecessor, and byte/fingerprint revalidation. |
| `HCM32-JCS-IMPL-DISC-003` | Retry classification retains the authentic live transition and exact one-mapping registry delta, credential/use-head, response/challenge, predecessor, and committed-chain checks. |
| `HCM32-JCS-IMPL-DISC-004` | Recovery phases and the selected zero/one/multiple-candidate plus generic non-HCM matrix remain GREEN in the 32-test kernel and 55-test lineage walls. |
| `HCM32-JCS-IMPL-DISC-005` | The fresh valid-A/different-valid-B/partial-scratch negative proves anchor/open/committed identity is validated before scratch cleanup, absent-open success, or durable mutation. |
| `HCM32-JCS-IMPL-DISC-006` | Control truth now reports the fresh-parent proof candidate conditionally, distinguishes source evidence from authority, and records unavailable FTS honestly. |

These are proof dispositions for the current candidate, not a substitute for
the required fresh-parent complete-subject review.

## GitNexus and control-plane observations

The corrected explicit-repository scope-all replay exited 0 in 2.813s with 14
files, 177 symbols, 50 affected processes, and `CRITICAL` risk. The eight
symbol-bearing paths and all 50 processes stayed within the authorized
Context Resolution, currentness, quarantine, generic-lineage, focused-test,
SPEC, plan, and checklist seams. Compare-to-`main` exited 0 in 10.328s with
1,355 files, 9,924 symbols, 259 processes, and `CRITICAL` risk, representing the
expected complete feature lineage. The first scope-all invocation omitted the
required repository selector and exited 1 before analysis; it was an invocation
targeting error, not a subject result. FTS emitted `extension unavailable` and
is recorded unavailable, never GREEN.

Each existing authorized control-document top-level section had exact upstream
impact of zero, zero affected processes/modules, and `LOW` risk before edit.
The complete product/test subject retained the previously reviewed selected
`CRITICAL` seams; no unexpected path, symbol, or process appeared.

## Post-edit replay

Post-edit validator, whitespace, allowlist, manifest, final GitNexus, remote,
base/ref, and protected observations are recorded by the fresh-parent proof
worker's structured return and must all remain GREEN or exactly unavailable as
declared before review dispatch. No file was staged or committed, no ref was
updated, and nothing was pushed or published.
