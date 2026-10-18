"""Append version-bound replay proof to the existing report-only result branch."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import time

ROOT = Path(__file__).resolve().parent
NS = 'docs/research/file-native-incremental-engine-experiment-2026-10-18'
OUT = ROOT / 'corrected-checkout' / NS / 'native-macos'
PIN = '7ac70f93cce14ecd3d935852801f034c122000d8'
BASE = 'c09a709a9830a8337282bab4980e881574d0074c'
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT / 'corrected-checkout', text=True).strip() == BASE
assert not subprocess.check_output(['git', 'status', '--porcelain=v1'], cwd=ROOT / 'corrected-checkout')
def load(path):
    return json.loads(path.read_bytes())
def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

proof = load(ROOT / 'evidence-replay/replay-verification.json')
assert proof['status'] == 'passed' and proof['events'] == 610 and proof['unperformed'] == []
export = load(ROOT / 'export-replay/verification.json')
assert export['pin'] == PIN and export['events'] == 610 and export['censors'] == 0
shutil.copytree(ROOT / 'export-replay', OUT / 'evidence' / 'replay-7ac70f9')
for name in ['README.md', 'pins.json', 'assembly-summary.json']:
    destination = OUT / ('a1108c6-affected-report.md' if name == 'README.md' else name.replace('.json', '-a1108c6.json'))
    assert not destination.exists()
    shutil.copyfile(OUT / name, destination)
for name in ['launch_replay.py', 'verify_replay_run.py', 'assemble_replay_report.py']:
    shutil.copyfile(ROOT / name, OUT / 'helpers' / name)
shutil.copyfile(ROOT / 'replay-export.log', OUT / 'replay-export.log')
shutil.copyfile(ROOT / 'evidence-replay/replay-verification.json', OUT / 'replay-verification.json')
shutil.copyfile(ROOT / 'evidence-replay/replay-plan.json', OUT / 'replay-plan.json')
shutil.copyfile(ROOT / 'evidence-replay/unit-temporary-roots.json', OUT / 'unit-temporary-roots-replay.json')
binaries = load(ROOT / 'evidence-replay/binary-pins.json')
write(OUT / 'pins.json', dict(
    final_tested_source_pin=PIN, source_files=60,
    source_pins='evidence/replay-7ac70f9/outer--source-pins.json', binaries=binaries,
    prior_affected_pins='pins-a1108c6.json', prior_smoke_pins='pins-f3.json',
    report_only_parent=BASE,
    qualification='Report follow-up contains only native-macos artifacts. Tested core comes from the separate exact 7ac70f9 source capsule, not the older core inherited by this result branch.'))
write(OUT / 'assembly-summary.json', dict(
    final_tested_source_pin=PIN, final_native_events=610, final_native_censors=0,
    final_exit_counts=proof['exit_counts'], source_files=60,
    prior_a1108c6_events=570, prior_f3_events=726, development_events=117,
    all_retained_events=2023, all_retained_censors=0, no_native_1k_rerun=True,
    no_native_history_rerun=True, unperformed_requested_checks=[], original_deadline_preserved=True,
    report_only_parent=BASE, epoch=time.time()))
(OUT / 'execution-plan.md').write_text((OUT / 'execution-plan.md').read_text() + '''

## Replay-retention targeted correction

The coordinator supplied 7ac70f93cce14ecd3d935852801f034c122000d8 after
reproducing loss of eligible candidates' deterministic replay dependencies.
replay-plan.json prospectively binds the same original allocation/deadlines.
A fresh 60-file capsule excludes only committed native-macos report artifacts;
the initial full snapshot is separately preserved. A new replay-lane/build target
and fresh bases run locked tests/build, replay_lifetime.py A/B N512 first,
then checks.py --faults, supplement.py, adversarial.py and namespace_cases.py.
All seven commands passed within the existing window. No N1k/history rerun.
Append report artifacts to the previous Mac result branch by ordinary fast-forward
push; do not transplant newer core into the report-only result commit.
''')
(OUT / 'README.md').write_text(f'''# Native macOS replay-retention correction — October 18, 2026

**Final tested source pin: `{PIN}`.** The targeted A/B N512 replay-lifetime regression, locked workspace tests/release build and all four requested follow-up suites passed. This lane records **610 uncensored engine responses**: 504 exit 0, 70 expected refusals exit 2 and 36 injected process exits 86. Every requested check was performed. Counts describe process responses, not independent trials or assertion counts.

Native executable SHA-256: `{binaries['file-native-incremental']['sha256']}`. All **60 capsule files** are exact Git-pinned bytes. The source snapshot excludes only previously committed `native-macos/` report transport; the initial full 362-file snapshot remains preserved separately and was not executed. No engine or pinned harness source was edited on Mac.

## Reached correction proof

| Suite | Events | Observed result |
| --- | ---: | --- |
| replay_lifetime.py | 40 | A/B, N512; eligible 256-label preparation reopens after collection; intermediate import payloads and packet reopen after collection; resealed tampered replay-object list refused; abandoned intermediate sources reclaimed |
| checks.py --faults | 414 | Four A/B × project/research configurations and 23 publication boundaries passed |
| supplement.py | 109 | Retention, reclamation, restoration, reader overlap, packet/history, divergence, quota, cancellation and other causal regressions passed |
| adversarial.py | 42 | Nine grouped forged/missing/corrupt/indirection cases passed |
| namespace_cases.py, default fixed | 5 | Ancestor `objects/results` root retains all 16 selected originals; original hashes/audit checked |
| Total | **610** | **Zero native censors** |

Both Rust tests passed. The release workspace build also built project/research consumers; no new separately linked consumer execution is claimed on this pin. Four unused-import warnings remain in the exact compiler logs. [replay-verification.json](replay-verification.json) records the assertions, reached summaries, counts and original-clock status. All **574 JSON responses** conserve total read/written bytes across payload/control/metadata/result roles. The other 36 responses are injected process exits without JSON.

## Fault boundary and version distinction

The reproduced defect concerned an eligible, unselected preparation: collection retained final graph/projection roots but could remove intermediate COW objects and overwritten import payloads needed by deterministic replay when the candidate was reopened. The supplied correction records exact replay object/file dependency sets in private Candidate v2, retains them during collection, and recomputes/compares them during read-only selection. The new regression observes successful reopen after GC and refusal of resealed tampered dependency lists in both A and B, then checks abandoned-source reclamation.

Old unselected Candidate v1 fixtures are intentionally refused by this experiment's versioned private contract. Publication and store remain v1; accepted retry still resolves before candidate parsing. Fresh new-pin bases were used; this is not a production migration or a claim that old unselected fixtures can resume. The structured format standard remains proposed, existing owner/versioned contracts control, and no production format migration is authorized.

## Earlier evidence remains intact

| Source version | Retained responses | Interpretation |
| --- | ---: | --- |
| 7ac70f9, current targeted lane | **610** | Replay-retention regression plus requested affected suites; no N1k smoke/history rerun |
| a1108c6 affected lane | 570 | Earlier ancestor-name collection/byte-role correction proof |
| F3 main and companions | 726 | Earlier bounded smoke, consumers, five ordinary copies and deep interruption controls |
| fb73e75 development | 117 | Partial pre-smoke run stopped on the common-pin hold |

All six retained evidence lanes contain **2,023 responses**, with zero censors. These versions are kept separate; their counts are not one new-pin benchmark. Earlier raw evidence, archives, hashes and reports are unchanged. [The a1108c6 report](a1108c6-affected-report.md) and [the F3 snapshot](f3-bounded-path-report.md) preserve their historical interpretation; this README controls the final tested pin.

F3's collector had the real ancestor-`objects` bug and its primary byte-role split is invalid under ancestor `results`. Its raw values remain unchanged. F3 total bytes, objects metadata counters and measured times remain separately qualified. Path-neutral F3 companions provide small valid role observations; they do not replace 1k values. [performance.md](performance.md), smoke-traces.json and smoke-operations.json remain **F3-only**. There is no corrected N1k speedup or history claim. Full-reference and incremental-preparation clocks differ in scope, inclusive clocks overlap, fresh process is not device-cold, and no p95 or causal OS ranking follows. Holder interruption was exit 86, not a claimed SIGKILL/reboot/power cut.

## Original clock, caps and preserved roots

The existing allocation is unchanged: 105 minutes of work plus 15 minutes of reporting from the original final start. At the replay plan snapshot, {load(ROOT / 'evidence-replay/replay-plan.json')['elapsed_original_seconds']/60:.2f} minutes had elapsed, {load(ROOT / 'evidence-replay/replay-plan.json')['remaining_work_seconds']/60:.2f} work minutes remained and {load(ROOT / 'evidence-replay/replay-plan.json')['remaining_total_seconds']/60:.2f} total minutes remained. At postrun verification, {proof['elapsed_original_seconds']/60:.2f} minutes had elapsed, {proof['remaining_original_work_seconds']/60:.2f} work minutes and {proof['remaining_original_total_seconds']/60:.2f} total minutes remained. **No deadline was reset.** Exact epochs and later packaging snapshots are retained.

Locked test/build caps are 600 seconds each, replay suite 900 seconds, checks/supplement 600 seconds each, adversarial 300 seconds and namespace 180 seconds. Every engine call retains the unchanged Driver's 180-second cap. The outer wrapper additionally enforces the original execution cutoff, whole-owned-root <=6 GiB guard with 512 MiB margin and >=20 GiB free-space floor. It kills only its own launched process group on a limit. No cap or deadline fired.

- Preserved whole root: `/private/tmp/file-native-incremental-macos-20261018`
- Exact new source worktree: `/private/tmp/file-native-incremental-macos-20261018/replay-checkout` at 7ac70f9
- Executed source capsule: `/private/tmp/file-native-incremental-macos-20261018/source-replay-core/{NS}`
- Separate build: `/private/tmp/file-native-incremental-macos-20261018/replay-lane/build`
- Fresh runtimes: `replay-lane/replay`, `checks`, `supplement`, `adversarial`, `namespace` beneath the owned root
- Local logs/pins: `/private/tmp/file-native-incremental-macos-20261018/evidence-replay`
- Report checkout: `/private/tmp/file-native-incremental-macos-20261018/corrected-checkout`
- Initial full transport: `source-replay` and `replay-full-transport` beneath the owned root; preserved and never executed

All earlier source/build/runtime/evidence/worktree roots remain preserved. The four tiny bounds-unit temporary roots are enumerated in unit-temporary-roots-replay.json. No cleanup was performed. The shared Mac checkout stays read-only, clean at 379af4eda9b95ee82be9d3afcfdcdca2d6d132f9. Native environment remains Darwin 25.6.0/macOS 26.6.1 arm64, APFS, Rust/Cargo 1.89.0, Python 3.11.9 and Bash 3.2. Earlier fsync/F_FULLFSYNC/link probes and F3 disassembly remain version-labeled evidence, not newly sampled corrected disassembly. Peak-memory telemetry is null on Mac because the unchanged helper reads `/proc`. No power-cut or reboot persistence is established.

## Report transport and reproduction

This is a **report-only follow-up to Mac result `{BASE}`**. Its inherited repository core is not relabeled as the tested 7ac70f9 core. New-pin executable/source bytes come from the separate exact source snapshot and source/session archives. This preserves the existing result branch through an ordinary fast-forward push without introducing engine changes into the report commit. The coordinator can consolidate the scoped report with its newer common core separately.

[helpers/launch_replay.py](helpers/launch_replay.py) preserves the actual bounded launcher. [replay-plan.json](replay-plan.json) binds it prospectively to the original allocation. Actual argv/cwd/caps/start/end/exit/log hashes are in evidence/replay-7ac70f9/outer--runs.jsonl. Use a fresh exact-pin capsule and fresh paths; never rerun into retained roots. Commands are locked workspace test/release build, replay_lifetime.py first, then checks.py --faults, supplement.py, adversarial.py and namespace_cases.py. The unchanged collector exports raw Driver events/session source ZIPs/source TAR, byte-level stdout/stderr associations, source Git/SHA-256 pins, executable hashes and wrapper logs. A separately copied replay summary is bound by its exact hash in replay-summary-binding.json. Full runtime corpora and executable/build bytes stay on Mac.

After exact-byte Git materialization:

```bash
python3 helpers/verify_export.py .
```

The portable verifier checks raw archive hashes/counts, stdout/stderr byte bindings, JSON type/parse association, session source ZIP bytes and source TAR bytes against per-lane pins and executable-hash associations. Physical binaries were rehashed locally; no reproducible-build claim is made. Local `.gitattributes` preserves exact bytes, including the earlier pinned CRLF companion helper. Artifact names remain portable and collision-checked. GitNexus cannot map this unregistered isolated report checkout; its coverage is unknown. Manual scope/source/index checks accompany the report. No PR, force push, production merge/deploy, outward message, new chat, subagent or Pro consultation was performed.
''')
print(OUT)
