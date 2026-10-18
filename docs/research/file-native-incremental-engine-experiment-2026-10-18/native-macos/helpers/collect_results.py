"""Lossless portable native exports, independently checking response/session bytes."""
import argparse
import base64
import gzip
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import unicodedata
import zipfile

ROOT=Path(__file__).resolve().parent
NS='docs/research/file-native-incremental-engine-experiment-2026-10-18/'

def sha(data):return hashlib.sha256(data).hexdigest()
def load(p):return json.loads(p.read_bytes())
def write(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n')
def typed(v):return json.dumps(v,sort_keys=True,ensure_ascii=False,allow_nan=False)

def safe_members(names):
    seen=set()
    for name in names:
        path=PurePosixPath(name)
        assert not path.is_absolute() and '..' not in path.parts and '\\' not in name
        assert all(part and not part.endswith((' ','.')) and not any(c in part for c in '<>:"|?*') for part in path.parts)
        key=unicodedata.normalize('NFC',name).casefold()
        assert key not in seen,('normalization-collision',name)
        seen.add(key)

def collect(label,runtime,evidence,output,adapter=None,allocation=None):
    output.mkdir(parents=True,exist_ok=False)
    pins=load(evidence/'source-pins.json')
    expected={name.removeprefix(NS):v['sha256'] for name,v in pins['files'].items()}
    binaries=load(evidence/'binary-pins.json')
    expected_binary={v['sha256'] for v in binaries.values()}
    index=[];bindings=[];sessions=[];event_count=0;censors=0;exits={};actions={}
    for events in sorted(runtime.rglob('events.jsonl')):
        parent=events.parent;relative=events.relative_to(runtime).as_posix();prefix='base' if relative=='events.jsonl' else relative.removesuffix('/events.jsonl').replace('/','--')
        session=load(parent/'session.json')
        if (parent/'outer-binding.json').exists():
            outer=load(parent/'outer-binding.json')
        else:
            assert adapter is not None and allocation is not None
            outer=dict(executable_sha256=session['sha256'],source_archive_sha256=session['source_archive_sha256'],
                adapter_sha256=sha(adapter.read_bytes()),allocation_sha256=sha(allocation.read_bytes()),
                cap_seconds=600 if 'smoke' in str(parent) else 180,
                binding_origin='Derived from retained unmodified common run_lane adapter and prospective outer allocation; original session JSON is unchanged')
        if adapter is not None and not (parent/'outer-binding.json').exists():
            shutil.copyfile(adapter,output/'common-adapter.py')
            shutil.copyfile(allocation,output/'prospective-allocation.json')
        assert session['sha256'] in expected_binary and outer['executable_sha256']==session['sha256']
        assert session['harness_sha256']==expected['controller.py']
        assert session['source_hashes']==expected
        archive=parent/'session-source.zip'
        assert sha(archive.read_bytes())==session['source_archive_sha256']==outer['source_archive_sha256']
        with zipfile.ZipFile(archive) as z:
            safe_members(z.namelist());assert set(z.namelist())==set(expected)
            for name,digest in expected.items():assert sha(z.read(name))==digest
        for name in ['session.json','session-source.zip']:
            shutil.copyfile(parent/name,output/(prefix+'--'+name))
        write(output/(prefix+'--outer-binding.json'),outer)
        sessions.append(dict(path=relative,source_archive_sha256=session['source_archive_sha256'],exe_sha256=session['sha256'],cap_seconds=outer['cap_seconds']))
        raw=events.read_bytes();rows=[json.loads(line) for line in raw.splitlines()]
        compressed=gzip.compress(raw,mtime=0);dest=output/(prefix+'--events.jsonl.gz');dest.write_bytes(compressed)
        assert gzip.decompress(dest.read_bytes())==raw
        index.append(dict(file=dest.name,source=relative,records=len(rows),bytes=len(raw),sha256=sha(raw),compressed_sha256=sha(compressed)))
        for line,row in enumerate(rows,1):
            out=base64.b64decode(row['stdout_b64'],validate=True);err=base64.b64decode(row['stderr_b64'],validate=True)
            assert len(out)==row['stdout_bytes'] and sha(out)==row['stdout_sha256']
            parsed=None;error=None
            try:parsed=json.loads(out)
            except Exception as exc:error=str(exc)
            assert typed(parsed)==typed(row['output']) and error==row['parse_error']
            assert row['exe_sha256']==session['sha256']
            bindings.append(dict(file=dest.name,line=line,stdout_sha256=sha(out),stdout_bytes=len(out),stderr_sha256=sha(err),stderr_bytes=len(err),exe_sha256=row['exe_sha256']))
            event_count+=1;censors+=bool(row['censored']);exits[str(row['exit'])]=exits.get(str(row['exit']),0)+1
            action=row['input'].get('action','unknown');actions[action]=actions.get(action,0)+1
    for p in sorted(runtime.rglob('*')):
        if not p.is_file():continue
        rel=p.relative_to(runtime)
        if len(rel.parts)>2:continue
        if p.name in ['environment.json','summary.json','checks.json','supplement.json','adversarial.json','namespace-cases.json','matrix.json','completed.json','deep-cases.json','walk-cases.json','A.stdout','A.stderr','B.stdout','B.stderr','walk.stdout','walk.stderr','uname.stdout','uname.stderr','df.stdout','df.stderr','rust.stdout','rust.stderr','cargo.stdout','cargo.stderr']:
            shutil.copyfile(p,output/rel.as_posix().replace('/','--'))
    # Original setup/build/test/watchdog artifacts and exact wrapper sources.
    copied=[]
    for p in sorted(evidence.iterdir()):
        if p.is_file():
            shutil.copyfile(p,output/('outer--'+p.name));copied.append(dict(file='outer--'+p.name,sha256=sha(p.read_bytes()),bytes=p.stat().st_size))
    write(output/'raw-index.json',index);write(output/'response-bindings.json',bindings)
    result=dict(lane=label,pin=pins['pin'],events=event_count,censors=censors,exit_counts=exits,action_counts=actions,sessions=sessions,
        raw_archives=len(index),source_files=len(expected),outer_artifacts=copied,status='lossless-response-source-session-association-passed',
        qualification='Events are process responses, not independent test trials. Expected injected exits/refusals are not automatically failures. Native completion and each required result are evaluated separately.')
    write(output/'verification.json',result)
    safe_members([str(p.relative_to(output)) for p in output.rglob('*') if p.is_file()])
    return result

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--label',required=True);p.add_argument('--runtime',type=Path,required=True);p.add_argument('--evidence',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--adapter',type=Path);p.add_argument('--allocation',type=Path);a=p.parse_args()
    print(json.dumps(collect(a.label,a.runtime,a.evidence,a.output,a.adapter,a.allocation),indent=2))
