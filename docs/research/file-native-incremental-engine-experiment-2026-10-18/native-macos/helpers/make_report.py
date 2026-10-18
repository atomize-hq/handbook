"""Assemble only owned native result artifacts; never alter the common capsule."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import time

ROOT=Path(__file__).resolve().parent
REPO=ROOT/'final-checkout'
PIN='f3e942f6a2e84ff33fd6ea860caed49ad650f86f'
NS='docs/research/file-native-incremental-engine-experiment-2026-10-18'
OUT=REPO/NS/'native-macos'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=REPO,text=True).strip()==PIN
OUT.mkdir(exist_ok=False)
def load(p):return json.loads(p.read_bytes())
def sha(data):return hashlib.sha256(data).hexdigest()
def write(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n')

for folder,source in [('development','export-development'),('primary','export-final-primary'),('deep','export-final-deep'),('walk','export-final-walk')]:
    shutil.copytree(ROOT/source,OUT/'evidence'/folder)
helpers=OUT/'helpers';helpers.mkdir()
for name in ['outer.py','capsule_adapter.py','launch_final.py','launch_companions.py','collect_results.py','verify_export.py','verify_completion.py','make_report.py']:
    shutil.copyfile(ROOT/name,helpers/name)
for name in ['deep_cases.py','walk_cases.py','helper-pins.json']:
    shutil.copyfile(ROOT/'companions-f2c45c1'/name,helpers/name)
for name in ['allocation.json','allocation-final.json','execution-plan.md','execution-plan-development.md']:
    shutil.copyfile(ROOT/name,OUT/name)
for name in ['environment.json','development-hold.json','development-summary.json','development-transport-verification.json','development-wrapper-pins.json']:
    shutil.copyfile(ROOT/'evidence'/name,OUT/('development-'+name))
for name in ['counter-path-classification-issue.json','counter-path-comparison.json','native-completion-verification.json','companion-verification.json','native-api-disassembly.txt','global-finished.json','launch-bindings.json','companion-runs.jsonl']:
    shutil.copyfile(ROOT/'evidence-final'/name,OUT/name)
for name in ['development-export.log','final-primary-export.log','final-deep-export.log','final-walk-export.log']:
    shutil.copyfile(ROOT/name,OUT/name)
(OUT/'.gitattributes').write_text('# Preserve exact native evidence, source archive and helper bytes.\n* -text\n')

pins=load(ROOT/'evidence-final/source-pins.json')
assert pins['pin']==PIN
for name,v in pins['files'].items():
    assert sha((REPO/name).read_bytes())==v['sha256']
    assert subprocess.check_output(['git','show',PIN+':'+name],cwd=REPO)==(REPO/name).read_bytes()
main=load(ROOT/'evidence-final/native-completion-verification.json')
exports={name:load(OUT/'evidence'/name/'verification.json') for name in ['development','primary','deep','walk']}
assert exports['primary']['events']==602 and exports['deep']['events']==120 and exports['walk']['events']==4
assert not any(exports[name]['censors'] for name in exports)

raw=[json.loads(line) for line in (ROOT/'final-lane/results/smoke/events.jsonl').read_bytes().splitlines()]
operations=[]
for line,row in enumerate(raw,1):
    c=row['input'];v=row['output'] or {};m=v.get('metrics',{})
    if c['action'] not in ['init','publish','refresh','validation-trace']:continue
    item=dict(action=c['action'],request=c.get('request',{}).get('request'),mode=c.get('mode'),root=c['root'],
        raw_line=line,controller_ms=row['elapsed_ns']/1e6,engine_ms=v.get('wall_ns',0)/1e6,
        exit=row['exit'],censored=row['censored'],result=v.get('result'),metrics=m,
        role_split_status='invalid: ancestor results path shadows payload/control categories; raw values retained',
        valid_byte_counters=['bytes_read','bytes_written','bytes_metadata_read','bytes_metadata_written'],
        peak_memory_bytes=v.get('peak_memory_bytes'))
    operations.append(item)
write(OUT/'smoke-operations.json',operations)
matrix=load(ROOT/'final-lane/results/smoke/matrix.json')
trace=[];publication=[]
for row in matrix:
    if row.get('stage')=='validation':
        value=row['observation']['result'];f=value['full_metrics'];i=value['incremental_metrics']
        trace.append(dict(requested=row['k'],full_reference_ms=value['full_load_semantic_validation_ns']/1e6,
            complete_incremental_preparation_ms=value['incremental_complete_preparation_ns']/1e6,
            full_documents_visited=f['documents_visited'],incremental_documents_visited=i['documents_visited'],
            incremental_edges_visited=i['edges_visited'],metadata_read=i['bytes_metadata_read'],metadata_written=i['bytes_metadata_written'],
            scope=value['scope']))
    if row.get('stage')=='publication-and-A':
        p=row['publication'];f=row['refresh'];publication.append(dict(requested=row['k'],actual_changed=len(p['result']['result']['changed']),
            accepted_publication_ms=p['wall_ns']/1e6,explicit_refresh_ms=f['wall_ns']/1e6,touched_paths=f['metrics']['touched_paths'],copies=f['metrics']['copies']))
write(OUT/'smoke-traces.json',dict(validation=trace,publication_and_refresh=publication,storage=[r for r in matrix if 'storage_trees' in r]))
lines=['# Native 1k observations','',
    'Each row is one observation. Full-reference and incremental-preparation clocks have different scopes; do not interpret their ratio as end-to-end speedup. Inclusive engine timers overlap. Fresh process is not device-cold. Primary role bins are invalid; totals and objects metadata counters remain valid.','',
    '| Requested IDs | Full load + semantic validation ms | Complete incremental preparation ms | Documents visited full / incremental | Incremental edges visited | Metadata read / written B |',
    '| ---: | ---: | ---: | ---: | ---: | ---: |']
for v in trace:lines.append(f"| {v['requested']} | {v['full_reference_ms']:.3f} | {v['complete_incremental_preparation_ms']:.3f} | {v['full_documents_visited']} / {v['incremental_documents_visited']} | {v['incremental_edges_visited']} | {v['metadata_read']} / {v['metadata_written']} |")
lines+=['','Incremental preparation includes graph/projection persistence, reservation and sealing. Full reference includes full load plus semantic validation. Explicit comparison audit is excluded from both clocks.','',
    '| Requested IDs | Actually changed | Accepted publication ms | Explicit A refresh ms | Touched paths / copies |','| ---: | ---: | ---: | ---: | ---: |']
for v in publication:lines.append(f"| {v['requested']} | {v['actual_changed']} | {v['accepted_publication_ms']:.3f} | {v['explicit_refresh_ms']:.3f} | {v['touched_paths']} / {v['copies']} |")
lines+=['','The imports reuse exactly the same per-ID source bytes across overlapping batches, producing actual changed sets 1/7/56. Validation traces use label operations and remain separately scoped.','',
    '| Operation | Engine ms | Copies | Total read / written B | Metadata read / written B |','| --- | ---: | ---: | ---: | ---: |']
for row in operations:
    if row['action']=='init' or row.get('request')=='B-one':
        m=row['metrics'];name='initial '+str(row['mode']) if row['action']=='init' else 'B one-label successor'
        lines.append(f"| {name} | {row['engine_ms']:.3f} | {m['copies']} | {m['bytes_read']} / {m['bytes_written']} | {m['bytes_metadata_read']} / {m['bytes_metadata_written']} |")
lines+=['','B copied 2,003 files per complete generation in this corpus, including per-document navigation/derived files. Manifest creation is additional. Both B initialization and its successor were accepted/published and explicitly verified.','',
    'Raw `metrics`, storage tree occupancy/height/physical/tombstone keys, quotas, sync/namespace/page counts and all stage clocks remain in smoke-operations.json and the source-bound JSONL archives. Peak-memory telemetry is null on Mac because the unchanged helper reads /proc. No p95 or causal OS ranking is inferred.']
(OUT/'performance.md').write_text('\n'.join(lines)+'\n')

binary=load(ROOT/'evidence-final/binary-pins.json')
rust={name:v['sha256'] for name,v in pins['files'].items() if name.endswith('.rs')}
write(OUT/'pins.json',dict(core_pin=PIN,companion_pin='f2c45c16886ae3c1c89d1084e43b05133383c087',
    capsule_files=len(pins['files']),capsule_pin_file_sha256=sha((ROOT/'evidence-final/source-pins.json').read_bytes()),
    rust_source_set_sha256=sha(json.dumps(rust,sort_keys=True).encode()),rust_files=rust,binaries=binary,
    helper_sources=load(ROOT/'companions-f2c45c1/helper-pins.json')))

report=f'''# Native macOS incremental engine result — October 18, 2026

Core correctness/native coverage passed on **{PIN}**, with an open byte-role instrumentation limitation described below. No engine source was changed on Mac. The final worktree is detached from the exact common pin until this report-only commit; source/capsule files remain byte-identical.

## Reached evidence

| Area | Actual result |
| --- | --- |
| Locked workspace build/tests | Passed; two Rust unit tests passed; separately compiled project/research consumers built |
| Main capsule | 602 uncensored engine events; four A/B x project/research named cases; 23 publication exit boundaries |
| Phase oracle | 14 must-not-select, one visible/incomplete uncertain, eight completed-boundary must-retain cases; original result checked beneath successors |
| Causal supplement | Reader/resolve leases, actual reclamation and original outcomes, restoration, b/crash/a reversal, divergence interruption/tampering, packet redirect/historical packet, sealed-root protection, orphan allocation cleanup, true eight-candidate quota, checkpoint/pin/cancel/abandon/GC boundaries passed; 13 injected exits in this dataset |
| Adversarial | Nine grouped checks passed, including forged candidate summaries, real >4 KiB indirection, unrelated point versus affected point/audit/GC, missing/corrupt graph/history/result evidence |
| Native smoke | One A/B 1k pair; three 1/8/64 validation traces; three accepted A publications with explicit refresh, and one complete B successor |
| Library consumers/copies | Two actual separately linked consumers passed init/publish/resolve/audit; three primary Bash 3.2 copies verified |
| Companion controls | 120 events for 16 checkpoint-specific exits plus waiting cancellation and holder-exit recovery; four events for A/B ordinary copies, including positively bound old B path copied while a real successor selects |
| Final total | **726 source/session/executable-bound native events**, zero native censors: 604 exit 0, 69 expected refusal exit 2, 53 injected process exits 86. These are process-response counts, not assertion counts or independent trials |
| Larger cases | No native 10k/100k growth, large-history, broad matrix or additional backend comparison allocated |

Primary and companion groups remain distinct. There are five successful ordinary shell copies overall. The companion source is separately pinned at `f2c45c16886ae3c1c89d1084e43b05133383c087`; the executable and library source remain f3e942f. Holder interruption uses the helper's actual `EXP_EXIT=lock-acquired` / exit 86, not a claimed SIGKILL or reboot.

## Open instrumentation defect

`src/io.rs: Metrics::record_bytes` searches **all ancestor components**. The common runner passes `--base <root>/results`; its ancestor `results` therefore captures every non-objects access before payload/control classification. Main capsule payload/control zeros and result totals are **invalid role classifications**, not true zero work. Raw events are unchanged. Total bytes and objects metadata counters remain valid, as do the separately asserted correctness outcomes and raw timers.

The first 16-document A init wrote 340,085 total bytes and 14,862 metadata bytes, but reports payload/control written as zero and result written as 325,223. A path-neutral companion on the **same binary** reports 309,226 payload, 15,523 control and 474 result bytes, with the same total and metadata written. See [counter-path-comparison.json](counter-path-comparison.json) and [the original trigger](counter-path-classification-issue.json).

The coordinator confirmed the issue and directed qualification with **no engine patch or completed-capsule rerun**. Only path-neutral companion role observations are interpretable. This remains open for consolidation; no primary split is silently reconstructed or relabeled.

## Measurements and interpretation

[performance.md](performance.md) contains every reached native 1k sample, requested versus actual changed sets, separate publication/A-refresh clocks and storage tree measurements. Full-reference load+semantic validation differs from complete incremental preparation with persistence/reservation/sealing; their ratio is not an end-to-end speedup. B retains complete independent readable generations. A may mix/lag until its explicit refresh. Both contracts stay open.

Requested publication batches 1/8/64 actually change 1/7/56 IDs because overlapping imports reuse identical bytes. The reported refresh counts agree. No p95, device-cold claim, causal cross-host ranking or production storage decision is made. Inclusive timers are not summed as disjoint work. Application byte/sync/page counters are not device I/O or physical write amplification.

## Native environment and durability

Native Darwin 25.6.0 / macOS 26.6.1 arm64, Rust/Cargo 1.89.0, Python 3.11.9 and system Bash 3.2; APFS data volume. Actual directory/file fsync and Darwin F_FULLFSYNC probes succeeded, same-filesystem hard links share an inode, and this volume is case- and Unicode-normalization-insensitive. Environment/probes are retained in development-environment.json; final native environment/source/binary snapshots are in evidence/primary/environment.json and session files.

The final executable disassembly retains the native fcntl(F_FULLFSYNC), linkat, rename and flock calls. Rust 1.89 Apple File::sync_all uses F_FULLFSYNC. This is API/process-interruption qualification only; kernel/device caches survive and no power-cut/reboot persistence is established. The unchanged /proc peak-memory helper is unavailable on Mac, represented as null rather than zero. Exact source/schema semantics remain limited to the experimental fixture contract; the structured format standard remains proposed, existing owner/versioned contracts control, and no production migration is authorized.

## Pins and preserved roots

Main executable SHA-256: `{binary['file-native-incremental']['sha256']}`.
All 49 capsule file SHA-256/Git-blob identities, Rust source set, three native binary hashes and both companion helper hashes are in [pins.json](pins.json), source tar and session archives. All actual core bytes match the common Git pin. Companion controls are extracted independently without switching the common worktree or copying newer core source.

- Whole owned root: `/private/tmp/file-native-incremental-macos-20261018`
- Final source snapshot: `/private/tmp/file-native-incremental-macos-20261018/source-final/{NS}`
- Final worktree/report: `/private/tmp/file-native-incremental-macos-20261018/final-checkout/{NS}/native-macos`
- Final binary/build: `/private/tmp/file-native-incremental-macos-20261018/final-lane/build`
- Final main runtime: `/private/tmp/file-native-incremental-macos-20261018/final-lane/results`
- Companion runtimes: `final-lane/deep-cases` and `final-lane/walk-cases` beneath that owned root

Every predecessor prerequisite root remains preserved. The shared Mac checkout was only read. Its HEAD was 379af4e at this task's opening; no switch/reset/cleanup or source write was performed there. The final result commit is parented directly to f3e942f and contains only this `native-macos/` namespace.

## Development separation and finite budget

Superseded pin fb73e75bb9893a4d897ecd28eb5615f6cc2a45d2 built and passed two Rust unit tests; its partial native run has 117 uncensored events. It was stopped by the coordinator's common-pin hold before its 1k smoke. Its controlled stop/traceback, sources, binaries and raw events are retained under development; they do not count as final measurements.

Before final launch the coordinator explicitly authorized one corrected final lane: 105 minutes work plus 15 minutes reporting. allocation-final.json preserves that prospective clock; no running benchmark deadline was reset. The common runner is used unchanged with a stricter 5 GiB run-root limit inside an additional whole-owned-root <=6 GiB guard, including development, source worktree, final source/build/runtime/evidence. Volume reserve is >=20 GiB. Workspace test/build caps are 600 seconds, stricter than the permitted 900; ordinary engine calls 180 seconds; smoke 600 seconds. All final steps finished within caps. No competing benchmark was launched by this chat.

## Reproduction and export verification

Use a fresh isolated worktree at the exact common pin, archive only the capsule, and choose fresh owned paths. The common source was executed with:

```bash
python3 /private/tmp/file-native-incremental-macos-20261018/source-final/{NS}/run_lane.py \\
  --root /private/tmp/file-native-incremental-macos-20261018/final-lane \\
  --source /private/tmp/file-native-incremental-macos-20261018/source-final/{NS} \\
  --pin {PIN} --cap-gib 5 --reserve-gib 20
```

Do not rerun into retained roots. `helpers/launch_final.py` is the actual whole-root envelope. It preserves the unmodified common runner and its retained cap adapter. The above common runner's results-container role limitation still applies; do not treat its role bins as valid.

Companion commands use PYTHONPATH pointing at that f3e experiment, the same native executable, fresh bases `final-lane/deep-cases` / `final-lane/walk-cases`, and the exact retained f2 helper files. Their actual argv/exit/source hashes are in companion-runs.jsonl. No full corpus or build is included in Git transport.

Portable read-only verification after exact-byte Git materialization:

```bash
python3 helpers/verify_export.py .
```

This verifies every stdout hash/length and JSON type association, stderr bindings, gzip bytes/counts, session ZIP names/bytes, Git-source TAR names/bytes, and source/session/binary-hash associations. Physical executable bytes remain on Mac; hash association is not a reproducible-build claim. Local `.gitattributes` disables text conversion. New artifact names are checked for NFC/casefold collisions and portable characters. No intentional native-case probe filenames are unpacked into Git artifacts.

GitNexus does not map this isolated snapshot/helpers; unknown graph coverage is reported rather than zero impact. Manual caller review is limited to owned watchdog/export CLIs, and precommit exact staged paths/source bytes provide the scope proof. No Rust/library change, force push, PR, production merge/deployment, successor dispatch or Pro consultation occurs.
'''
(OUT/'README.md').write_text(report)
write(OUT/'assembly-summary.json',dict(core_pin=PIN,primary_events=602,deep_events=120,walk_events=4,final_native_events=726,
    final_native_censors=0,development_events=117,role_split='invalid for primary, valid for path-neutral companions',
    epoch=time.time(),source_files_verified=49))
print(OUT)
