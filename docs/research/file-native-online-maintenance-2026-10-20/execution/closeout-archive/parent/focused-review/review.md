# Independent frozen focused-harness review

**Engineering disposition: REMEDIATE. Full experiment qualification: INCOMPLETE. Real execution: HOLD.**

## Required findings, severity first

### F1 — P1: authentic semantic conflicts cannot be recovered with the real Driver signature

`focused_protocol.py:248-261`, callers `325-327` and `517-521`; misleading controls `fake_controls.py:139-145,190-205`.

Harness.call receives `env` in `**fields`, incorporates it into its expected `payload`, and passes it to Driver.call. Both actual Windows and native Linux Drivers declare `env=None` as a separate keyword argument and form their raw `row['input']` exclusively from the remaining engine fields. Their environment is recorded separately as `row['env']`. Therefore the exact-input comparison at line260 fails for a legitimate violation-free semantic exit2 whenever the harness supplied env. Harness.publish supplies `env=None` even without an injection, and the four-publisher schedule supplies a real barrier environment. The three losing candidates in each same-base round become unexpected call failures; mandatory 16 accepted /48 conflict/64 attempt completion cannot be reached.

Pure memory proof using the existing FakeAdapter/oracle and an explicit Driver.call signature matching both real Drivers: without env, the exact conflict is recovered; with env=None or the barrier environment, the same valid per-call row raises AssertionError. Existing 36 controls still pass because FakeDriver/RoundDriver incorrectly retain env inside input; they model a different contract.

**Fix:** separate Driver transport options from engine payload construction in Harness.call. Verify engine input against `row['input']` and, where needed, bind the environment independently against `row['env']`. Keep exact invocation ownership, exit2, empty violations and declared semantic error requirements. Update both fake Driver signatures/payload construction to match the real ones.

**Required proof:** exact-signature controls for absent/None/barrier env; mismatched input or environment and RSS/time censors must refuse. Re-run the 16x4 memory trace with real-signature rows; retain eventual admitted native 16accepted/48conflict evidence separately.

### F2 — P2: optional absent journal target is indexed as a mandatory key

`focused_schedules.py:181-183`.

After the owned automatic-benefit decline, the schedule requires `journal['target'] is None`. At exact reviewed CP2 `7a6ee9db`, `packing.rs:147` declares target with `default,skip_serializing_if="Option::is_none"`; the benefit-refusal path at605-609 saves an aborted-owned journal before target exists. A correct journal consequently omits target, so this schedule raises KeyError on valid behavior. The controlling `execution/reviews/cp2/review-7a6ee9db.md` already explicitly preserves this same failed expectation.

**Fix:** accept the documented absent optional target (`journal.get('target') is None`), while retaining the aborted-owned and unchanged-selection checks. Do not change the engine serialization contract to satisfy this helper.

**Required proof:** a contract control with the actual omitted field plus malformed/nonempty target negatives; eventual native owned decline remains separately required.

### F3 — P2: the automatic-policy success workload repeats a disproved H151 assumption

`focused_schedules.py:190-194`; `forecast.json` per_root_case_register entry `auto-A/disposable/store`.

The schedule adds30 publications, observes an owned decline atH31, adds120, then calls an unforced pack with expected exit0 atH151. The reviewed exact7a CP2 report records that this very H151 positive-benefit expectation failed legitimately; adding another400 reachedH551 and demonstrated ordinary automatic admission. The present forecast still assumes final_H151 and a5MiB provisional root. F2's key fix alone therefore cannot make this schedule complete against the actual reviewed implementation.

Independently re-read the retained compressed raw events to check options, rather than assuming the earlier codec/profile matched. The F2 init is A/project/count10. cp2-remediate rows830/862, cp2-finish row122 and cp2-finish2 row402 all use build/zstd/include_attempts=true/cleanup=false/bootstrap=true, with force omitted (the engine treats that as false). Their visitedH/exit pairs are1/2,31/2,151/2,551/0, all bound to binarySHA256 `6f77de3001f488d6dd57317b04f33f216856f11fbfd65d8236e8b45d28ebc4b5`. Thus the H151 negative uses the same codec and N10 default input shape. No new execution occurred; hashes of those retained raw inputs are in evidence.

**Fix:** freeze a source-supported, prospectively admitted workload sufficient for positive charged benefit on the final source, or bound and retain successive genuine automatic declines until a predeclared accepted-history ceiling. Preserve every declined owner, default payload profile, all evidence and no manual abort/force policy. Reforecast root and whole topology before any extra publications; H551 is a demonstrated CP2 point, not an automatic final-pin or resource grant.

**Required proof:** low-count refusal, owned refusal and later ordinary success without repair/force; exact outcomes and selections; actual removal of all declined owners; independent complete oracles. Update declared finalH and owner coexistence forecasts. Do not turn the earlier H151 negative into a pass or hide extra retries/publications.

### F4 — P2: ready/maintenance failures bypass every publisher join

`focused_protocol.py:515-535`.

The publisher join loop follows a try/finally that only releases controls. If spawn/await_ready or the maintenance call at525 raises, Python executes that finally and then propagates the exception past the join loop. Publisher threads can remain live, or retain unobserved exceptions and responses; the outer ExitStack joins only the four held readers. ThreadCalls starts non-daemon threads and maintains no handle registry. A close_case receipt proving native processes/handles closed does not itself prove these Python callbacks joined or that their outcomes were retained. An adapter could add stronger cleanup, but that does not make this shared failure path implement its documented all-joins requirement.

Pure memory proof with the existing RoundAdapter/RoundDriver/RoundHarness, injecting pack failure after all four candidates are ready:8 handles spawned, only4 passed to harness wait; all4 publisher handles omitted. The reviewer then joined every omitted memory thread; no engine/native process was involved.

**Fix:** retain the primary failure, attempt release of every created control independently, and always wait/record every successfully spawned publisher in cleanup before rethrowing. Preserve secondary release/join/Driver failures individually. Native close_case remains mandatory if a bounded join fails; never substitute an unverified positive closure claim.

**Required proof:** failure at each spawn/ready position, maintenance failure, release failure and join failure; every created handle observed and every failure prefix retained; bounded native closure and no non-daemon thread left running.

## Crash-schedule construction dependency

`focused_schedules.py:263-278` does not itself establish retired-container or abandoned-owned-file work before requesting `pack-old-delete` or `physical-after-owned-file-delete`. On a valid drained one-segment donor, ordinary tail build under the default four-segment limit inherits the base, so no old container is retired. It also creates no abandoned owner to trigger the owned-garbage hook. Expected exit86 then fails honestly; actual_removal_proof prevents a false pass. The donor map is explicitly still pending, so this is a concrete qualification/construction dependency rather than a finding that the ledger has falsely passed either case.

Before dispatch, bind each exact donor/precondition to a real eligible old instance or aborted owner and prove that the requested phase occurs after actual removal. A source-supported reduced segment limit can force replacement where appropriate; owned-file deletion needs a legitimate durably abandoned owner. Retain successful-deletion identity and before/after inventory. If the cooperative batch ends before the hook, use only a prospectively bounded, recorded continuation schedule; do not interpret exit0/cleaning or a planned target as a crash pass. Keep namespace mutation/fixture reset and any added peak growth in the reviewed adapter and admission map. `physical-progress-plan-installed` likewise needs actual pending metadata work.

## Reviewed identity and scope

Frozen root: `C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/final-protocol/`.

All7 files/128761bytes were independently hashed before review and rechecked unchanged at the end of static reading. Receipt SHA256 is `a6ebd313ca3f7f8c81b0ac2e2b1bca8b30d30c4861bfae4dd45a4f0f90b8e7b8`; it declares the six-file manifest hash `fc40bab3cf0a022f4d860570dcaa9d592d9b004f22ed13283abae423dcdbe757`. Each actual file hash/size matches that receipt's entries. Core SHA256 `1b10f9121483bb6b5adf1ca20ba31bfad3367161d7d8d77925ce30eb14b17a83`; schedules SHA256 `35ffbbaed3281ca470391f2dbc078805ae5fa5adec725d690b11d54c6ec571d5`. Compact evidence lists all7 hashes. No mutable CP3 Rust source was treated as7a: API/phase/schema contracts were read using Git blobs at exact `7a6ee9db03f5cd2e388129a9019c0d5f274a1844`.

Read the full five landing consultation controls: next-experiment-brief, reconciliation, answer, bounded-review-packet and local-inspection-notes; the proposed structured-data-format standard; all CP1/CP2 review reports in execution/reviews; frozen gates/watchdogs, Matrix checklist, amendment/admission context, native fixture config and Driver/capsule adapter contracts. The proposed standard remains proposed; no owner/versioned format migration is approved. The code-review and GitNexus skill guidance was read; explicit no-subagents/no-bootstrap restrictions governed the local review. GitNexus tools are unavailable, indexed impact/freshness UNKNOWN, manual risk HIGH. No impact graph or detect_changes result is invented; no symbol edit or commit occurred.

Read every new core/schedule/ledger/control/README/forecast/receipt file. Existing 36 pure-memory controls passed (Python tracemalloc peak887202bytes, not native RSS). Additional exact-signature and exceptional-join memory observations are identified above and in evidence. No engine, Cargo/build, real fixture, independent engine oracle, filesystem-backed native control, indexing, source edit, source/raw copy, worktree, subagent, extra chat or Git mutation occurred. Only these review/evidence files are written in the authorized128KiB/12file destination. Meta alone received notices. Static source freeze release was explicitly sent to meta after the unchanged rehash; later worker revisions are outside this review.

## Boundaries that remain sound or explicitly incomplete

- Inputs retains distinct attempted request/intent variants and uniquely accepted inputs. Publish success binds the accepted publication; complete oracle callbacks receive every accepted input. No failed-prefix erasure was found in the append-only core contract. F1 currently prevents authentic conflicts from reaching that intended accounting.
- The default1/4/1 schedule expresses16 new accepted/cycle,64 offered attempts in the four-publisher cycle, all candidates paused on the same base, four held readers and positive native overlap evidence. No hidden frontend retries occur in that schedule. Adapter phase clocks/process intervals still need independent review; a thread/fake interval is not qualification. Its60second barriers must fit the whole admitted phase, including native preparation/maintenance and any optional collection. Do not restart a failed cell silently.
- Coupled capture pauses before the new tail exists, then accepts/maps that tail, activates two successors and holds the old reader beyond15seconds. It joins before requiring A loose deletion; B publication controls remain loose. Shared selected base containers are correctly distinguished from obsolete ones. Actual old-container removal remains separately mandatory.
- Exact two-call drainage is distinct from the many-batch cleanup helper and uses an independent backlog callback. Adapter census must include abandoned owners and all protected/excluded work; no journal-only count qualifies it.
- Windows stress requires new authentic accepted N10-A tails, changing descriptor/catalog plus distinct generation id/ordinal, semantic-selection preservation and separately recorded work-CAS holds. Native<=64 pressure cannot qualify Windows1000. Independent final-pin mapping/raw/source evidence remains adapter-owned and pending.
- Full oracle callbacks explicitly require physical representation, all accepted exact request intents/results, membership, graph/source/navigation/provenance, retired refusal, B loose/ordinary selected-once and unchanged inventory. Engine audit is expressly excluded. Callback strings/booleans still need parent verification of the referenced real artifacts; fake controls are never native qualification.
- Page/version/boundary/death/cancel/sharing/path-reuse/backup/reset constructors and native quiescence/closure are honestly declared pending extensions. No required N/A or skipped-case pass was found. Full quiescence must preserve old-reader/preparation owners; zero hidden COW resets and complete donor provenance remain mandatory. API cancellation is not a commit fence; native interruption adapters must preserve that distinction.
- Reader/checker role selection is action-based256MiB/2GiB, independent of executable basename. The shared core contains no process kill or engine-owned file mutation primitive; adapter implementations must enforce confined observations, owned control paths and guarded native closure. No native adapter implementation is accepted by this review.
- Optional failed-owner abandonment/collection is off by default and requires raw-prefix retention, complete oracle checks and actual native removal before continuation. No accepted B retirement or planned deletion credit may fund it. F4 must also cover its exceptional paths.
- CaseLedger remains PENDING initially for166Windows/158Linux/158Mac rows. Its presence/binding checks are a handoff checklist, not independent proof of referenced native artifacts or performance. Full Matrix/history/long restore/current graph/source/navigation/native obligations remain required after remediation.

## Resources, clocks and next decision

Forecast is correctly NO_GO. B conditional upper149251266bytes excludes physical metadata; the whole diagnostic490682821bytes/54596files exceeds the proposed448MiB469762048byte/50000file envelope by20920773bytes/4596files. These are upper-envelope diagnostics with explicitly unproven other-root allowances, not a proof of fundamental lower-bound impossibility. The current native96MiB cap and unreleased448MiB/Windows480MiB proposals remain HOLD. The existing human cumulative44GiB limit is not a fresh allocation. F3's correction requires a new whole prospective topology, not a smaller payload, omitted case, protected-data deletion or silent budget uplift.

Meta should return F1-F4 for bounded source remediation, freeze/hash the replacement, and obtain a focused re-review with exact-signature and failure-path controls. Complete source-bound adapters/donor/crash constructors, final CP3 source pin, complete independent oracles and prospective GO remain required before any native run. This report authorizes neither integration of this frozen harness nor real execution.

Fixed clocks remain CP3watch2026-10-21T03:10:14Z, source-review/freeze04:10:14Z, functional11:10:14Z and protected closeout13:10:14Z. No reset or reduced full qualification is implied.
