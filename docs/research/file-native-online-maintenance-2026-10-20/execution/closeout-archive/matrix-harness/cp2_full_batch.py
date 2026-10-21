"""Read-only CP2 fixtures: historical caller provenance, then full independent oracle."""
import argparse
import base64
import json
import os
from pathlib import Path
import subprocess
import sys
from job_guard import census, sha_file, write_json
from matrix_budget import check_matrix

assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1' and sys.dont_write_bytecode
labels = ['smoke-A','smoke-B','partial','capture','late-eligible','interval-crash',
          'cutoff-cross','owners','F1-retired','F1-oversize','F1-reservation','F2-auto','stable-base']
parser = argparse.ArgumentParser()
parser.add_argument('--output',required=True)
parser.add_argument('--decoder-sha256',required=True)
parser.add_argument('--case',action='append',choices=labels,required=True)
parser.add_argument('--compact-records',action='store_true')
args = parser.parse_args()
assert len(set(args.case)) == len(args.case)
harness = Path(__file__).resolve().parent
output = Path(args.output).resolve()
assert not output.exists()
check_matrix({'receipts':dict(bytes=(4 if args.compact_records else 12)*1024**2,
                             files=100 if args.compact_records else 180)})
output.mkdir()
evidence = Path('C:/file-native-online-cp1-20261020/evidence')
fixtures = Path('C:/file-native-online-cp1-20261020/fixtures/cp2')
decoder = harness/'physical_decoder_v4.py'
decoder_sha = args.decoder_sha256
assert sha_file(decoder) == decoder_sha
old_pin = 'b876df9e01957be29b5c9cbd676d5a169aaa6c67'
new_pin = '7a6ee9db03f5cd2e388129a9019c0d5f274a1844'
old_producer = '8dd3a498289c2e5c438385e4cecd1af2c0cfb84f2769e28048a1a40666383467'
new_producer = '6f77de3001f488d6dd57317b04f33f216856f11fbfd65d8236e8b45d28ebc4b5'
handoff = json.loads((evidence/'cp2-handoff.json').read_bytes())
closed = json.loads((evidence/'cp2-engineering-closeout.json').read_bytes())
assert handoff['fixed_roots_quiescent'] and handoff['old_b876_roots_unchanged_after_source_fix']
assert closed['source_and_CP2_prefixes_read_only'] and closed['CP2_active_Jobs'] == 0
artifact_hashes = {Path(r['path']).name:r['sha256'] for r in handoff['artifacts']}
cases = []
for name in ['smoke-A','smoke-B','partial']:
    cases.append((name,old_pin,old_producer,['smoke']))
for name in ['capture','late-eligible','interval-crash','cutoff-cross','owners']:
    cases.append((name,old_pin,old_producer,['schedules']))
for name in ['F1-retired','F1-oversize','F1-reservation','F2-auto','stable-base']:
    cases.append((name,new_pin,new_producer,['remediate','finish','finish2']))
cases = [row for row in cases if row[0] in args.case]
assert len(cases) == len(args.case)
rows = []
batch_source_sha256=sha_file(__file__)
observations = output/'subprocess-observations.jsonl'
if args.compact_records:
    with observations.open('x',encoding='utf-8'):pass


def record(label,stage,command,result):
    assert sha_file(__file__) == batch_source_sha256
    if args.compact_records:
        case=next(row for row in cases if row[0]==label)
        with observations.open('a',encoding='utf-8') as stream:
            stream.write(json.dumps(dict(label=label,stage=stage,command=command,exit=result.returncode,
                root=str(fixtures/label/'disposable/store'),producer_source_pin=case[1],
                historical_producer_sha256=case[2],decoder_sha256=decoder_sha,
                batch_source_sha256=batch_source_sha256,helper_sha256=sha_file(command[2]),
                stdout_base64=base64.b64encode(result.stdout).decode('ascii'),
                stderr_base64=base64.b64encode(result.stderr).decode('ascii')),separators=(',',':'))+'\n')
            stream.flush()
    else:
        stem=label+'-provenance' if stage=='provenance' else label
        (output/(stem+'.stdout')).write_bytes(result.stdout)
        (output/(stem+'.stderr')).write_bytes(result.stderr)


write_json(output/'batch-binding.json',dict(parser_sha256=decoder_sha,parser_source_pin=new_pin,
    selected_cases=args.case,batch_source_sha256=sha_file(__file__),
    compact_records=args.compact_records,
    compact_raw_format='complete subprocess stdout/stderr bytes in base64 JSONL, one row per stage',
    handoff_sha256=sha_file(evidence/'cp2-handoff.json'),
    closeout_sha256=sha_file(evidence/'cp2-engineering-closeout.json'),
    executable_paths_are_historical_only=True,no_engine_calls=True,no_fixture_copies=True))
for name,pin,producer,events in cases:
    root = fixtures/name/'disposable/store'
    paths = [evidence/('cp2-'+event+'-events.jsonl.gz') for event in events]
    for path in paths:assert sha_file(path) == artifact_hashes[path.name]
    provenance = output/(name+'-requests.json')
    command = [sys.executable,'-B',str(harness/'request_provenance.py'),
        '--root',str(root),'--producer-sha256',producer,'--output',str(provenance)]
    for path in paths:command.extend(['--events',str(path)])
    helper_sha256=sha_file(command[2])
    prepared = subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
    assert sha_file(command[2]) == helper_sha256, 'provenance-helper-changed'
    record(name,'provenance',command,prepared)
    command = [sys.executable,'-B',str(harness/'quiescent_oracle.py'),
        '--root',str(root),'--module',str(decoder),'--module-sha256',decoder_sha,
        '--output',str(output/(name+'.json'))]
    if prepared.returncode == 0:command.extend(['--request-provenance',str(provenance)])
    helper_sha256=sha_file(command[2])
    checked = subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=300)
    assert sha_file(command[2]) == helper_sha256, 'oracle-wrapper-changed'
    record(name,'oracle',command,checked)
    row = dict(label=name,root=str(root),producer_source_pin=pin,historical_producer_sha256=producer,
        provenance_exit=prepared.returncode,oracle_exit=checked.returncode,
        stdout=checked.stdout.decode(errors='backslashreplace' if args.compact_records else 'strict'),
        stderr=checked.stderr.decode(errors='backslashreplace' if args.compact_records else 'strict'),
        provenance_sha256=sha_file(provenance) if provenance.exists() else None,
        events=[dict(path=str(p),sha256=sha_file(p)) for p in paths])
    rows.append(row)
    if not args.compact_records:write_json(output/(name+'-progress.json'),dict(rows=rows))
    print(json.dumps(dict(label=name,provenance_exit=prepared.returncode,oracle_exit=checked.returncode)),flush=True)
    if args.compact_records:
        used=census(output)
        assert used['bytes']+262144 <= 4*1024**2 and used['files']+4 <= 100, 'compact-phase-forecast-exceeded-stop-and-preserve'
    check_matrix({'receipts':dict(bytes=2*1024**2,files=10)})
assert sha_file(decoder) == decoder_sha
raw_binding=None
if args.compact_records:
    raw=observations.read_bytes()
    assert raw.endswith(b'\n'), 'incomplete-observation-EOF'
    captured=[json.loads(line) for line in raw.splitlines()]
    assert len(captured)==2*len(cases)
    for name,*unused in cases:
        assert sorted(row['stage'] for row in captured if row['label']==name)==['oracle','provenance']
    for row in captured:
        for field in ['stdout_base64','stderr_base64']:base64.b64decode(row[field],validate=True)
        assert row['batch_source_sha256']==batch_source_sha256 and row['decoder_sha256']==decoder_sha
    raw_binding=dict(path=str(observations),sha256=sha_file(observations),bytes=len(raw),
        rows=len(captured),complete_line_EOF=True,all_stdout_stderr_bytes_preserved=True)
write_json(output/'batch-results.json',dict(rows=rows,raw_observations=raw_binding,
    no_engine_calls=True,no_fixture_copies=True))
if args.compact_records:
    used=census(output)
    assert used['bytes']+262144<=4*1024**2 and used['files']+4<=100, 'final-compact-phase-forecast-exceeded'
sys.exit(0 if all(r['provenance_exit'] == r['oracle_exit'] == 0 for r in rows) else 1)
