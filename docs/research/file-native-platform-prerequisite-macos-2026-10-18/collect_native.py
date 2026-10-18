"""Lossless platform-specific export; run only after serial engine lanes exit."""
import base64
import gzip
import hashlib
import json
import pathlib
import shutil
import sys

HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE.parents[2]
allocation=json.loads((HERE/'allocation.json').read_bytes())
runtime=pathlib.Path(allocation['runtime'])
data=runtime/'data'
output=HERE/'evidence'

def sha(value): return hashlib.sha256(value).hexdigest()
def write(path,value): path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n')

def collect():
    output.mkdir(exist_ok=False)
    index=[]
    for path in sorted(data.rglob('*')):
        if not path.is_file():continue
        relative=path.relative_to(data)
        if path.name.endswith('-raw.jsonl'):
            raw=path.read_bytes();rows=[json.loads(line) for line in raw.splitlines()]
            target=output/(str(relative).replace('/','--')+'.gz')
            target.write_bytes(gzip.compress(raw,mtime=0))
            index.append(dict(source=str(relative),file=target.name,records=len(rows),
                bytes=len(raw),sha256=sha(raw),compressed_sha256=sha(target.read_bytes())))
        elif len(relative.parts)<6 and (path.name.endswith('-observations.json') or
                path.name.endswith('-completed.json') or path.name.endswith('-stopped.json') or
                path.name.startswith(('harness-session-','native-session-')) or
                path.name=='navigation-paths.json'):
            shutil.copyfile(path,output/str(relative).replace('/','--'))
    write(output/'raw-index.json',index)
    return verify()

def verify():
    pins=json.loads((HERE/'source-pins.json').read_bytes())
    for name,pin in pins.items():
        assert sha((ROOT/name).read_bytes())==pin['sha256'],('source-drift',name)
    binary=json.loads((HERE/'binary-pin.json').read_bytes())
    assert sha(pathlib.Path(binary['path']).read_bytes())==binary['sha256']
    wrapper=sha((HERE/'native_runner.py').read_bytes())
    allocation_sha=sha((HERE/'allocation.json').read_bytes())
    source_sha=sha((HERE/'source-pins.json').read_bytes())
    native=synthetic=resource_available=censored=0
    phase_counts={}
    for item in json.loads((output/'raw-index.json').read_bytes()):
        compressed=(output/item['file']).read_bytes()
        assert sha(compressed)==item['compressed_sha256']
        raw=gzip.decompress(compressed)
        assert sha(raw)==item['sha256'] and len(raw)==item['bytes']
        rows=[json.loads(line) for line in raw.splitlines()]
        assert len(rows)==item['records']
        parent=pathlib.PurePosixPath(item['source']).parent
        is_synthetic='controller-capture' in parent.parts
        for row in rows:
            prefix=str(parent).replace('/','--')+'--'
            manifest=json.loads((output/(prefix+f"harness-session-{row['session']}.json")).read_bytes())
            n=json.loads((output/(prefix+f"native-session-{row['session']}.json")).read_bytes())
            assert sha(json.dumps(manifest['files'],sort_keys=True).encode())==manifest['sha256']==row['harness_sha256']==n['original_harness_sha256']
            for name,digest in manifest['files'].items():
                assert sha((HERE.parent/'file-native-engine-followup-2026-10-17'/name).read_bytes())==digest
            for name,digest in n['source_files'].items():
                assert sha((HERE.parent/'file-native-engine-followup-2026-10-17'/name).read_bytes())==digest
            assert row['native_wrapper_sha256']==n['wrapper_sha256']==wrapper
            assert n['allocation_sha256']==allocation_sha and n['source_pin_sha256']==source_sha
            assert n['baseline']==allocation['baseline']
            assert row['binary_sha256']==n['binary_sha256']
            if not is_synthetic:assert row['binary_sha256']==binary['sha256']
            stdout=base64.b64decode(row['stdout_base64'],validate=True)
            assert sha(stdout)==row['stdout_sha256'] and len(stdout)==row['stdout_bytes']
            parsed=None;error=None
            if stdout.strip():
                try:parsed=json.loads(stdout)
                except (ValueError,UnicodeError) as exc:error=f'{type(exc).__name__}: {exc}'
            elif row['exit']==0:error='empty stdout from a successful process'
            assert json.dumps(parsed,sort_keys=True,allow_nan=False)==json.dumps(row['output'],sort_keys=True,allow_nan=False)
            assert error==row['stdout_parse_error']
            assert row['platform']=='native-macos-arm64'
            if is_synthetic:synthetic+=1
            else:
                native+=1
                resource_available+=row['native_process_usage'] is not None
                censored+=bool(row['censored'])
            phase_counts[str(parent)]=phase_counts.get(str(parent),0)+1
    # Original process oracle assertions are also checked on exported observations.
    observations=[]
    for path in output.glob('*fault-observations.json'):
        for row in json.loads(path.read_bytes()):
            if row.get('phase_oracle')=='must-retain':assert row['selected_at_reopen']
            if row.get('phase_oracle')=='must-not-select':assert not row['selected_at_reopen']
        observations.append(path.name)
    result=dict(status='passed',native_events=native,synthetic_events=synthetic,
        native_censored_events=censored,native_process_rusage_available=resource_available,
        phase_event_counts=phase_counts,phase_oracle_files=observations,
        qualification='Export integrity/source/session/response interpretation checks; not independent engine test cases or hardware persistence proof')
    write(HERE/'evidence-verification.json',result)
    print(json.dumps(result,indent=2))
    return result

if __name__=='__main__':
    if sys.argv[1:] == ['--verify']:verify()
    else:collect()
