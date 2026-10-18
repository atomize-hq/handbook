# Native macOS paired-intent accounting correction — October 18, 2026

**Tested source pin: `5a65a2155f9eb297d2cbb1b7c900fa749629605c`.** Locked release build, both Rust tests and all three focused case groups passed in fresh owned paths. Counts were derived from the actual Driver records: **181 responses, zero censors**, with exit counts `{'0': 149, '2': 19, '86': 13}`. No requested check was unperformed.

Executable SHA-256: `051c6dd72b7ebf6d7309a121cb374942fe71dc2ea526c0e3c05f8fb38c1447f6`. All **85 executed capsule files** match exact Git bytes. Source/session ZIPs, source TAR/Git-blob/SHA-256 pins, raw stdout/stderr bytes, executable hashes and finite wrapper logs are retained in the distinct [accounting-5a65a21 lane](evidence/accounting-5a65a21/verification.json). No Rust or experiment source was edited on Mac.

## Reached focused proof

| Group | Actual responses | Result |
| --- | ---: | --- |
| Paired intents, default strict A/B | 32 | Unrelated preparation and selection succeed; public losing-intent select/publication remain refused; losing attempt is abandoned/collected; original acceptance remains exact; final reservations empty |
| Causal supplement | 109 | Recovery, reader overlap, actual reclamation, restoration, quota and fallback regressions passed |
| Replay lifetime, A/B N512 | 40 | Eligible preparation and intermediate import/packet reopen after GC; tampered dependency lists refused; abandoned intermediate sources reclaimed |
| Total | **181** | **Zero censors** |

The paired suite records four expected public different-intent refusals across A/B. In its collection result, A reclaims four files (9,916 logical bytes) and B reclaims 40 files (172,529 logical bytes), while accepted evidence is preserved. Both final quota maps are empty. [accounting-verification.json](accounting-verification.json) retains exact paired results, causal summary, replay flags and all timings. All **168 JSON responses** conserve total read/written bytes across payload/control/metadata/result roles. Injected exit responses remain unparsed raw bytes; they are not counted as native censors. Response counts are not independent statistical trials or assertion counts.

The defect involved two eligible preparations with the same request ID but different intents: internal acceptance lookup could poison unrelated recovery and prevent abandoning the loser. The supplied minimal lifetime correction uses request-ID acceptance for internal recovery, transfers only the exact accepted attempt with matching intent/owner, and fences only the losing attempt. Public different-intent retry/selection refusal remains observable. No format change is claimed. Mac did not run the old-core `--expect-poisoned` reproduction; the independent review/Windows reproduction remains separate provenance.

## Actual clock and caps

This is one new explicitly authorized **1,800-second total followup**, including fetch/source capture, locked build/tests, focused cases, export, report and push. No previous measurement or correction window was reopened/reset.

- Start: **2026-10-18 23:21:10 UTC**, epoch `1792365670`.
- Absolute execution cutoff passed to the native wrapper: **23:46:10 UTC**, epoch `1792367170` (start + 1,500 seconds).
- Total/report deadline: **23:51:10 UTC**, epoch `1792367470` (start + 1,800 seconds). Reporting reserve: **300 seconds**.
- Build/tests: <=300 seconds each. Actual build 13.156 seconds; tests 9.354 seconds.
- All three case groups share one <=900-second phase. Actual combined phase: **123.160 seconds**, including transitions; paired 6.098, supplement 9.186, replay 105.771 seconds. The phase deadline is never restarted between groups.
- Every ordinary Driver call retains its <=180-second cap. The paired helper also receives the stricter shared case deadline. All subprocess groups and descendants remain under the native execution cutoff.
- At postrun verification: 339.326 seconds elapsed, 1160.674 execution seconds and 1460.674 total seconds remained. Final transport receipt records elapsed time through export/commit/push verification.

[allocation-accounting-correction.json](allocation-accounting-correction.json) was frozen prospectively, with authority allocation Git blob and unchanged original allocation hash. The source selection was narrowed before build to omit prior report/evidence transport; its earlier full 279-file snapshot is preserved, never executed, and its clock/deadlines were unchanged. The native launcher is invoked with `--deadline-epoch 1792367170`; actual argv, effective deadlines, case-phase timestamps, exits and log hashes are in evidence/accounting-5a65a21/outer--runs.jsonl.

## Resource and scope boundaries

The globally owned incremental allocation remains <=6 GiB with >=20 GiB free reserve. The wrapper inventories the entire incremental root plus the conservative complete isolated transport Git database and all matching bounds fixtures, keeps a 512 MiB margin, and stops only its own launched process groups on a guard/cap. All old and current source/worktrees/build/runtime/evidence/report are included. The six tiny unit fixtures remain preserved; no cleanup occurred.

- Whole root: `/private/tmp/file-native-incremental-macos-20261018`
- Exact source worktree: `/private/tmp/file-native-incremental-macos-20261018/accounting-checkout`
- Executed source: `/private/tmp/file-native-incremental-macos-20261018/source-accounting-capsule/docs/research/file-native-incremental-engine-experiment-2026-10-18`
- Build: `/private/tmp/file-native-incremental-macos-20261018/accounting-lane/build`
- Fresh runtimes: `accounting-lane/paired-intents`, `supplement`, `replay-lifetime` beneath the owned root
- Local logs/pins: `/private/tmp/file-native-incremental-macos-20261018/evidence-accounting`
- Initial full snapshot/transport: `source-accounting-core` and `accounting-full-transport` beneath the owned root, never executed
- Result checkout: `/private/tmp/file-native-incremental-macos-20261018/corrected-checkout`

The shared Mac checkout stays read-only and clean at 379af4eda9b95ee82be9d3afcfdcdca2d6d132f9. No 1k/large matrix, history rerun, production file, running-binary swap or Windows-history interaction occurred. The current Windows long history remains separately qualified on 7ac; this Mac followup neither changes nor reruns it. No successor was dispatched.

## Preserved version distinctions

| Source/lane | Retained responses | Qualification |
| --- | ---: | --- |
| 5a65a21 current accounting | 181 | Only the three focused groups above |
| 7e00d22 audit | 42 | Strict N128 A/B negative evidence; [prior report](7e00d22-audit-report.md) |
| 7ac70f9 replay | 610 | N512 replay plus affected suites; [prior report](7ac70f9-replay-report.md) |
| a1108c6 affected | 570 | Ancestor-name collection/counter correction |
| F3 main/companions | 726 | Earlier smoke, consumer/copy and interruption evidence |
| fb73e75 development | 117 | Partial pre-smoke run stopped on common-pin hold |

Eight evidence lanes retain **2246 responses**, zero censors, each with its own exact source/session/executable binding. The aggregate is an inventory, not one current-core benchmark. All predecessor raw evidence remains byte-identical. Earlier measurements are not relabeled as 5a observations.

F3 primary payload/control/result role bins remain invalid under ancestor `results`; its raw values, total/metadata bytes and times retain their prior qualifications. F3's real ancestor-`objects` GC defect remains correctly identified. performance.md and smoke JSON remain F3-only. Earlier 7ac positive audits do not substitute for 7e negative-evidence proof. No later-core speedup/history/p95/device-cold/cross-host claim follows. Peak-memory telemetry remains null on Mac. Experimental private Candidate v2 and existing store/publication formats are unchanged by this correction; the structured format standard remains proposed, and no production migration is authorized.

## Export and report transport

This report-only followup is parented to `c8d1195ee406deedee1f32649004ec3779b27256` on the existing Mac result branch. It changes only `native-macos/` artifacts; the tested newer core remains in its independently pinned capsule, not transplanted into the report branch. Exact source/session archives and binary hashes preserve that distinction.

[helpers/launch_accounting.py](helpers/launch_accounting.py) is the equally bounded native wrapper; [helpers/verify_accounting_run.py](helpers/verify_accounting_run.py) derives actual counts and validates reached summaries without another engine call. Reproduction uses a fresh exact capsule/target/base, the frozen absolute execution deadline and one shared case phase. Default paired mode is strict (`--expect-poisoned` is omitted). Never rerun into retained roots.

Portable verification after exact-byte Git materialization:

```bash
python3 helpers/verify_export.py .
```

The portable verifier checks raw gzip hashes/counts, stdout/stderr byte bindings, JSON type/parse association, session ZIP and source TAR bytes against pins, and executable-hash associations. Exact summary copies are additionally hash-bound in focused-summary-bindings.json. Physical executables remain on Mac and were rehashed locally; no reproducible-build claim is made. Local `.gitattributes` preserves exact bytes, including earlier source-pinned CRLF and raw logs. New names are portable/collision-checked. GitNexus does not map this unregistered isolated report checkout; coverage is unknown, with manual staged path/index/source verification recorded. No PR, force push, source edit or shared-checkout mutation occurred.
