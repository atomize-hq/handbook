"""Append one bounded accounting lane to the existing Mac report branch."""
import json
from pathlib import Path
import shutil
import subprocess
import time

ROOT = Path(__file__).resolve().parent
NS = 'docs/research/file-native-incremental-engine-experiment-2026-10-18'
REPO = ROOT / 'corrected-checkout'
OUT = REPO / NS / 'native-macos'
BASE = 'c8d1195ee406deedee1f32649004ec3779b27256'
def load(path):
    return json.loads(path.read_bytes())
def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

A = load(ROOT / 'allocation-accounting-correction.json')
assert time.time() < A['total_deadline_epoch']
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=REPO, text=True).strip() == BASE
assert not subprocess.check_output(['git', 'status', '--porcelain=v1'], cwd=REPO)
proof = load(ROOT / 'evidence-accounting/accounting-verification.json')
export = load(ROOT / 'export-accounting/verification.json')
assert proof['status'] == 'passed' and export['events'] == proof['events'] and export['censors'] == 0
prior = load(OUT / 'assembly-summary.json')
total = prior['all_retained_events'] + proof['events']
shutil.copytree(ROOT / 'export-accounting', OUT / 'evidence' / 'accounting-5a65a21')
for name in ['README.md', 'pins.json', 'assembly-summary.json']:
    destination = OUT / ('7e00d22-audit-report.md' if name == 'README.md' else name.replace('.json', '-7e00d22.json'))
    assert not destination.exists()
    shutil.copyfile(OUT / name, destination)
for name in ['launch_accounting.py', 'verify_accounting_run.py', 'assemble_accounting_report.py']:
    shutil.copyfile(ROOT / name, OUT / 'helpers' / name)
for source, name in [
    (ROOT / 'accounting-export.log', 'accounting-export.log'),
    (ROOT / 'allocation-accounting-correction.json', 'allocation-accounting-correction.json'),
    (ROOT / 'evidence-accounting/accounting-verification.json', 'accounting-verification.json'),
    (ROOT / 'evidence-accounting/unit-temporary-roots.json', 'unit-temporary-roots-accounting.json'),
]:
    shutil.copyfile(source, OUT / name)
binaries = load(ROOT / 'evidence-accounting/binary-pins.json')
write(OUT / 'pins.json', dict(
    final_tested_source_pin=A['pin'], current_lane='Paired-intent accounting + causal supplement + replay lifetime only',
    source_files=proof['source_files'], source_pins='evidence/accounting-5a65a21/outer--source-pins.json', binaries=binaries,
    prior_audit_pins='pins-7e00d22.json', prior_replay_pins='pins-7ac70f9.json',
    prior_affected_pins='pins-a1108c6.json', prior_smoke_pins='pins-f3.json', report_only_parent=BASE,
    qualification='Each observation stays bound to its exact source/session/executable. The report branch does not transplant the independently tested core.'))
write(OUT / 'assembly-summary.json', dict(
    final_tested_source_pin=A['pin'], current_lane_events=proof['events'], current_lane_censors=0,
    current_lane_counts=proof['counts'], current_lane_exit_counts=proof['exit_counts'], counts_derived=True,
    prior_7e00d22_events=42, prior_7ac70f9_events=610, prior_a1108c6_events=570,
    prior_f3_events=726, development_events=117, all_retained_events=total,
    all_retained_censors=0, retained_evidence_lanes=8, unperformed_requested_checks=[],
    explicit_total_cap_seconds=1800, absolute_execution_cutoff_offset_seconds=1500,
    reporting_reserve_seconds=300, combined_case_cap_seconds=900,
    original_windows_reset=False, successor_dispatch=False, report_only_parent=BASE, epoch=time.time()))
(OUT / 'execution-plan.md').write_text((OUT / 'execution-plan.md').read_text() + '''

## One paired-intent accounting followup

The coordinator explicitly authorized only the concrete 5a65a21 affected followup.
The new 1800-second outer window starts at first tool time 23:21:10 UTC and ends
23:51:10 UTC. A native wrapper is passed --deadline-epoch 1792367170, the absolute
start+1500 execution cutoff at 23:46:10 UTC, reserving 300 seconds for reporting.
Locked release build/tests use a distinct target and <=300 seconds each. One
shared case phase <=900 seconds covers paired_intents.py default strict A/B,
supplement.py causal recovery/quota, and replay_lifetime.py A/B N512. The inner
paired helper receives the stricter shared phase deadline. The whole-owned-root
guard includes the incremental root, conservative isolated Git DB and all bounds
temporary fixtures. Source excludes only prior report/evidence transport; the
initial fuller snapshot is preserved and never executed. No source/format edits,
broad matrix, Windows history change, prior-window reset or successor dispatch.
All raw observations and prior reports remain source-qualified and preserved.
''')
counts = proof['counts']
(OUT / 'README.md').write_text(f'''# Native macOS paired-intent accounting correction — October 18, 2026

**Tested source pin: `{A['pin']}`.** Locked release build, both Rust tests and all three focused case groups passed in fresh owned paths. Counts were derived from the actual Driver records: **{proof['events']} responses, zero censors**, with exit counts `{proof['exit_counts']}`. No requested check was unperformed.

Executable SHA-256: `{binaries['file-native-incremental']['sha256']}`. All **{proof['source_files']} executed capsule files** match exact Git bytes. Source/session ZIPs, source TAR/Git-blob/SHA-256 pins, raw stdout/stderr bytes, executable hashes and finite wrapper logs are retained in the distinct [accounting-5a65a21 lane](evidence/accounting-5a65a21/verification.json). No Rust or experiment source was edited on Mac.

## Reached focused proof

| Group | Actual responses | Result |
| --- | ---: | --- |
| Paired intents, default strict A/B | {counts['paired-intents']} | Unrelated preparation and selection succeed; public losing-intent select/publication remain refused; losing attempt is abandoned/collected; original acceptance remains exact; final reservations empty |
| Causal supplement | {counts['supplement']} | Recovery, reader overlap, actual reclamation, restoration, quota and fallback regressions passed |
| Replay lifetime, A/B N512 | {counts['replay-lifetime']} | Eligible preparation and intermediate import/packet reopen after GC; tampered dependency lists refused; abandoned intermediate sources reclaimed |
| Total | **{proof['events']}** | **Zero censors** |

The paired suite records four expected public different-intent refusals across A/B. In its collection result, A reclaims four files (9,916 logical bytes) and B reclaims 40 files (172,529 logical bytes), while accepted evidence is preserved. Both final quota maps are empty. [accounting-verification.json](accounting-verification.json) retains exact paired results, causal summary, replay flags and all timings. All **{proof['byte_role_conservation_json_responses']} JSON responses** conserve total read/written bytes across payload/control/metadata/result roles. Injected exit responses remain unparsed raw bytes; they are not counted as native censors. Response counts are not independent statistical trials or assertion counts.

The defect involved two eligible preparations with the same request ID but different intents: internal acceptance lookup could poison unrelated recovery and prevent abandoning the loser. The supplied minimal lifetime correction uses request-ID acceptance for internal recovery, transfers only the exact accepted attempt with matching intent/owner, and fences only the losing attempt. Public different-intent retry/selection refusal remains observable. No format change is claimed. Mac did not run the old-core `--expect-poisoned` reproduction; the independent review/Windows reproduction remains separate provenance.

## Actual clock and caps

This is one new explicitly authorized **1,800-second total followup**, including fetch/source capture, locked build/tests, focused cases, export, report and push. No previous measurement or correction window was reopened/reset.

- Start: **2026-10-18 23:21:10 UTC**, epoch `{A['start_epoch']:.0f}`.
- Absolute execution cutoff passed to the native wrapper: **23:46:10 UTC**, epoch `{A['execution_deadline_epoch']:.0f}` (start + 1,500 seconds).
- Total/report deadline: **23:51:10 UTC**, epoch `{A['total_deadline_epoch']:.0f}` (start + 1,800 seconds). Reporting reserve: **300 seconds**.
- Build/tests: <=300 seconds each. Actual build {proof['command_seconds']['build']:.3f} seconds; tests {proof['command_seconds']['tests']:.3f} seconds.
- All three case groups share one <=900-second phase. Actual combined phase: **{proof['combined_case_seconds']:.3f} seconds**, including transitions; paired {proof['command_seconds']['paired-intents']:.3f}, supplement {proof['command_seconds']['supplement']:.3f}, replay {proof['command_seconds']['replay-lifetime']:.3f} seconds. The phase deadline is never restarted between groups.
- Every ordinary Driver call retains its <=180-second cap. The paired helper also receives the stricter shared case deadline. All subprocess groups and descendants remain under the native execution cutoff.
- At postrun verification: {proof['elapsed_seconds']:.3f} seconds elapsed, {proof['execution_remaining_seconds']:.3f} execution seconds and {proof['total_remaining_seconds']:.3f} total seconds remained. Final transport receipt records elapsed time through export/commit/push verification.

[allocation-accounting-correction.json](allocation-accounting-correction.json) was frozen prospectively, with authority allocation Git blob and unchanged original allocation hash. The source selection was narrowed before build to omit prior report/evidence transport; its earlier full 279-file snapshot is preserved, never executed, and its clock/deadlines were unchanged. The native launcher is invoked with `--deadline-epoch 1792367170`; actual argv, effective deadlines, case-phase timestamps, exits and log hashes are in evidence/accounting-5a65a21/outer--runs.jsonl.

## Resource and scope boundaries

The globally owned incremental allocation remains <=6 GiB with >=20 GiB free reserve. The wrapper inventories the entire incremental root plus the conservative complete isolated transport Git database and all matching bounds fixtures, keeps a 512 MiB margin, and stops only its own launched process groups on a guard/cap. All old and current source/worktrees/build/runtime/evidence/report are included. The six tiny unit fixtures remain preserved; no cleanup occurred.

- Whole root: `/private/tmp/file-native-incremental-macos-20261018`
- Exact source worktree: `/private/tmp/file-native-incremental-macos-20261018/accounting-checkout`
- Executed source: `/private/tmp/file-native-incremental-macos-20261018/source-accounting-capsule/{NS}`
- Build: `/private/tmp/file-native-incremental-macos-20261018/accounting-lane/build`
- Fresh runtimes: `accounting-lane/paired-intents`, `supplement`, `replay-lifetime` beneath the owned root
- Local logs/pins: `/private/tmp/file-native-incremental-macos-20261018/evidence-accounting`
- Initial full snapshot/transport: `source-accounting-core` and `accounting-full-transport` beneath the owned root, never executed
- Result checkout: `/private/tmp/file-native-incremental-macos-20261018/corrected-checkout`

The shared Mac checkout stays read-only and clean at 379af4eda9b95ee82be9d3afcfdcdca2d6d132f9. No 1k/large matrix, history rerun, production file, running-binary swap or Windows-history interaction occurred. The current Windows long history remains separately qualified on 7ac; this Mac followup neither changes nor reruns it. No successor was dispatched.

## Preserved version distinctions

| Source/lane | Retained responses | Qualification |
| --- | ---: | --- |
| 5a65a21 current accounting | {proof['events']} | Only the three focused groups above |
| 7e00d22 audit | 42 | Strict N128 A/B negative evidence; [prior report](7e00d22-audit-report.md) |
| 7ac70f9 replay | 610 | N512 replay plus affected suites; [prior report](7ac70f9-replay-report.md) |
| a1108c6 affected | 570 | Ancestor-name collection/counter correction |
| F3 main/companions | 726 | Earlier smoke, consumer/copy and interruption evidence |
| fb73e75 development | 117 | Partial pre-smoke run stopped on common-pin hold |

Eight evidence lanes retain **{total} responses**, zero censors, each with its own exact source/session/executable binding. The aggregate is an inventory, not one current-core benchmark. All predecessor raw evidence remains byte-identical. Earlier measurements are not relabeled as 5a observations.

F3 primary payload/control/result role bins remain invalid under ancestor `results`; its raw values, total/metadata bytes and times retain their prior qualifications. F3's real ancestor-`objects` GC defect remains correctly identified. performance.md and smoke JSON remain F3-only. Earlier 7ac positive audits do not substitute for 7e negative-evidence proof. No later-core speedup/history/p95/device-cold/cross-host claim follows. Peak-memory telemetry remains null on Mac. Experimental private Candidate v2 and existing store/publication formats are unchanged by this correction; the structured format standard remains proposed, and no production migration is authorized.

## Export and report transport

This report-only followup is parented to `{BASE}` on the existing Mac result branch. It changes only `native-macos/` artifacts; the tested newer core remains in its independently pinned capsule, not transplanted into the report branch. Exact source/session archives and binary hashes preserve that distinction.

[helpers/launch_accounting.py](helpers/launch_accounting.py) is the equally bounded native wrapper; [helpers/verify_accounting_run.py](helpers/verify_accounting_run.py) derives actual counts and validates reached summaries without another engine call. Reproduction uses a fresh exact capsule/target/base, the frozen absolute execution deadline and one shared case phase. Default paired mode is strict (`--expect-poisoned` is omitted). Never rerun into retained roots.

Portable verification after exact-byte Git materialization:

```bash
python3 helpers/verify_export.py .
```

The portable verifier checks raw gzip hashes/counts, stdout/stderr byte bindings, JSON type/parse association, session ZIP and source TAR bytes against pins, and executable-hash associations. Exact summary copies are additionally hash-bound in focused-summary-bindings.json. Physical executables remain on Mac and were rehashed locally; no reproducible-build claim is made. Local `.gitattributes` preserves exact bytes, including earlier source-pinned CRLF and raw logs. New names are portable/collision-checked. GitNexus does not map this unregistered isolated report checkout; coverage is unknown, with manual staged path/index/source verification recorded. No PR, force push, source edit or shared-checkout mutation occurred.
''')
print(OUT)
