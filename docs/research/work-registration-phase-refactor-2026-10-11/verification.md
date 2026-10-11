# Focused Packet 1 verification

Baseline: `8e5c6f21077a508d40cd0661bd1946d04a132281` on
`feat/handbook-contract-membrane-clean`. This records checks for the authorized
work-registration phase refactor; it does not close the slice.

The normal-dependency SDK example check passed:

```powershell
cargo check --locked -p handbook-sdk --example work_state
```

The Engine release command passed all 14 tests, with no ignored cases: six
repository durability, two work-kind and six work-state tests. The work-state
target includes the new supplied-basis, missing-root pure-binding test and checks
the original retained V1 request bytes.

```powershell
$env:RUST_MIN_STACK='33554432'
cargo test --locked --release -p handbook-engine --features test-support --test work_state --test work_kind --test repository_durability -- --test-threads=1
```

Both new private unit fixtures passed under their actual `work_state::` module
filter (two passed, zero ignored; 285 unrelated unit tests filtered out). The
binding fixture uses the original request bytes and the derivation fixture
compares the resulting receipt against the untouched historical byte oracle.
Raw output is in `tests-engine-units.txt`.

```powershell
$env:RUST_MIN_STACK='33554432'
cargo test --locked --release -p handbook-engine --features test-support --lib work_state:: -- --test-threads=1
```

The isolated baseline comparison compiled an archive of `8e5c6f2` with only the
new Engine integration test copied in. The exact test failed at missing-root
preparation (`retained_state`), as expected; compilation succeeded, exit 101.
The same test passed in the refactor's Engine command above. This comparison was
run after implementation, so it corroborates the regression fixture rather than
claiming a test-first execution chronology. `baseline-red.ps1` retains the runner;
`tests-baseline-red.txt` retains source/test hashes and the result;
`tests-baseline-red-full.txt` retains complete build/test output. Its separate
archive and target did not modify the benchmark baseline.

The focused SDK command was continued through its required targets after three
test-fixture corrections. The failed outputs remain available; they are not
counted as passed checks. No product change was needed for these corrections:

- `work_kind` had an old eager-preparation assertion. It now requires successful
  binding followed by invocation refusal for an unactivated grant. The initial
  failure is in `tests-sdk-initial.txt`; all seven corrected tests passed in
  `tests-sdk.txt`.
- The kind process helper still treated deferred invocation refusal as a failed
  child process. It now saves the successfully bound original and reports the
  actual committed/refused disposition; its parent still enforces the expected
  outcome in each serialization order. The analogous role helper was corrected
  too. Nine kind-durability tests passed in `tests-sdk.txt`; the affected kind race
  passed its exact rerun in `tests-sdk-kind-race.txt`.
- The new in-place kind snapshot mutation used an incorrect JSON pointer. The
  tagged snapshot's pointer is `/value/definition/template/label_max_bytes`.
  Nine kind-integrity tests passed in `tests-sdk-remaining.txt`; the corrected
  mutation test passed in `tests-sdk-kind-mutation.txt`, covering eight scenarios.

The remaining four SDK targets completed successfully in
`tests-sdk-final-targets.txt`. The unique final SDK outcomes are:

| Target | Passed | Ignored entries | Evidence |
| --- | ---: | ---: | --- |
| `work_kind` | 7 | 0 | `tests-sdk.txt` |
| `work_kind_durability` | 10 | 1 | Nine earlier passes plus the corrected exact race rerun |
| `work_kind_integrity` | 10 | 0 | Nine earlier passes plus the corrected exact mutation rerun |
| `work_state` | 13 | 0 | `tests-sdk-final-targets.txt` |
| `work_state_durability` | 11 | 2 | `tests-sdk-final-targets.txt` |
| `work_state_integrity` | 11 | 0 | `tests-sdk-final-targets.txt` |
| `work_state_retained` | 2 | 2 | `tests-sdk-final-targets.txt` |

This is **64 unique SDK passes**, together with 14 Engine integration passes and
two private unit passes: **80 focused passes**. It is a reconciled result across
the continued commands, not a claim that the original seven-target command
finished in one uninterrupted run. Only the affected failures were rechecked;
unchanged earlier passes were retained. All runtime commands use single-threaded
execution and the same 32 MiB Windows test-thread stack.

```powershell
$env:RUST_MIN_STACK='33554432'
cargo test --locked --release -p handbook-sdk --features handbook-engine/test-support --test work_state --test work_state_retained --test work_state_integrity --test work_state_durability --test work_kind --test work_kind_integrity --test work_kind_durability -- --test-threads=1
```

The affected exact reruns and final continued command were:

```powershell
cargo test --locked --release -p handbook-sdk --features handbook-engine/test-support --test work_kind_durability real_process_kind_item_serializes_scope_change_revocation_and_governance_in_both_orders -- --exact --test-threads=1
cargo test --locked --release -p handbook-sdk --features handbook-engine/test-support --test work_kind_integrity repeated_kind_history_reads_reject_in_place_and_substituted_original_definition_records -- --exact --test-threads=1
cargo test --locked --release -p handbook-sdk --features handbook-engine/test-support --test work_state --test work_state_retained --test work_state_integrity --test work_state_durability -- --test-threads=1
```

New integration fixtures cover deferred live refusals, exact original replay,
same-item independent binding, exact K1 provenance after K2, pre-reservation
complete-record bounds, changed history bytes between invocation observations,
and fresh native/work/manifest/lock checks. The complete derived-record oversize
test passed before positive reservation. The repeated work-record mutation test
passed 12 scenarios, kind mutation passed eight, and Item-seam physical checks
passed 11. On Windows, held handles can deny a mutation attempt; those branches
check unchanged commit/original replay, while successfully applied damage must
fail closed. These results do not claim every substitution was permitted.

The five ignored entries are three process helpers (`native_kind_child`,
`native_role_child`, `native_work_child`) that were executed by their passing
parent tests, plus two original physical-root tests requiring unavailable
historical roots. The two unavailable cases are not passed evidence. Untouched
historical JSON request/receipt byte fixtures passed independently.

Rustfmt checks on all 13 changed Rust files and `git diff --check` passed.
Shared durability code, dependencies, capacity constants and historical fixture
files are unchanged. Required process-death, both-order serialization, early
acceptance, administrative transition, exhaustion and zero-free-slot original
recovery checks passed. No unrelated workspace suite was run.

Serial comparative measurement began only after the final SDK command exited
successfully and process inventory showed no test, build or population worker.
The [native evidence](README.md) records its method, raw observations, comparative
results and platform/resource limits separately.

The completed measurement contains 72 accepted pristine observations (36 paired
fresh effects), 12 focused dimension-control observations, 24 matched attribution
observations and 96 same-process resource calls. Every accepted main pair and all
six dimension-control pairs improved the actual preparation/save/invocation
workflow. The main cells each contain six pairs at a fixed pre-invocation history;
their median paired reductions are 33.51–39.80%. Attribution verifies explicit
preparation 0 / kind preparation 1 full scans, no live preparation checks, fewer
successful pure derivations, and preserved invocation 4 scans / 5 inventories /
6 manifest observations / 8 native current checks.

Both resource variants completed 48 calls with every per-call and overall handle
count 57→57 and no observer-tracked retained locks. Six initial fixed-history
replays and twelve final fixed-history refusal/replay calls expose the private
memory ranges separately from intentional history growth. Refactor allocator
residency can remain higher after a call; this evidence does not claim immediate
absolute heap release. The main pristine peak-memory tradeoff and the observed
controlled work-lock waits are reported without subtracting research overhead
or adding nested span costs.

The first overlapping pristine pair is excluded and replaced with fresh roots;
the failed research lock-observer attempts and their correction are retained.
The corrected observer changed only disposable attribution sources. The final
source check verifies the common driver, 18 primary source/dependency files and
four retained binaries. `timing-window-end.json` records zero native test, build,
population or measurement workers after the serial runs. Product sources remain
the same sources used by the focused passing checks.

The independent focused causal review approved all nine Packet-1 acceptance
conditions within the disclosed limits, with no unresolved P1/P2 and no P3/P4
advisories. It inspected the bounded source/test/docs deltas, relevant owner/store
dependencies and measurement/test evidence. The Engine integration and example
outcomes above are summarized from successful root tool outputs; the review
policy permits summarized verification evidence. No unchanged check was rerun
solely to create another log.

Exact staged GitNexus change detection reports 574 files (18 reviewed tracked
paths plus 556 packet evidence files), 118 indexed symbols and six affected
execution flows, with HIGH risk. The displayed symbol list is capped at 15 despite
the requested larger limit; the index also lacks newly introduced helpers. Direct
source reconciliation covers those helpers and shifted/unchanged neighboring
symbol ranges. All production edits are the four selected Engine/SDK source
files; the remaining tracked paths are focused tests, SDK documentation/example
and four HCM status documents. Shared durability, capacity, dependencies and
historical fixtures remain unchanged. The earlier default-`main` comparison
reported the cumulative branch's 1,292 files / 27,712 symbols / 724 processes as
CRITICAL; it includes preceding HCM work and is not the packet-only scope. Both
risk results were reported before the local commit.

The broad staged whitespace check flags the four exact captured instrumentation
patch files' line endings/context bytes and four raw test logs' final blank lines.
Their original bytes are retained rather than rewritten. The staged check passes
for all product/tests/docs/support code with raw `.patch` and `.txt` evidence
excluded; independent SHA-256/length checks pass for all 544 measurement files.
Git has `core.autocrlf=false` and no evidence-file filters, preserving those bytes
in the commit. These checks do not alter the reviewed implementation or measured
sources.
