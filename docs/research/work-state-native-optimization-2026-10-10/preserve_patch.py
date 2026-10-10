"""Preserve the measured experimental source without changing the primary product."""
import difflib, hashlib, json, pathlib, subprocess, sys
experiment=pathlib.Path(sys.argv[1]); evidence=pathlib.Path(sys.argv[2])
paths=[
    'crates/engine/src/definition_identity.rs',
    'crates/engine/src/lib.rs',
    'crates/engine/src/local_authority/work_registration.rs',
    'crates/engine/src/repository_durability.rs',
    'crates/engine/src/work_state/store.rs',
    'crates/engine/src/work_state/transaction.rs',
    'crates/engine/src/research_profile.rs',
]
baseline='8013948ebd3fbef6725f5d72f3d29120f7617009'
patch=[]; sources=[]
for path in paths:
    original=subprocess.run(['git','-C',str(experiment),'show',f'{baseline}:{path}'],capture_output=True)
    old=original.stdout.decode('utf-8').replace('\r\n','\n') if original.returncode==0 else ''
    raw=(experiment/path).read_bytes(); new=raw.decode('utf-8').replace('\r\n','\n')
    patch.append(f'diff --git a/{path} b/{path}\n')
    if original.returncode: patch.append('new file mode 100644\n')
    patch.extend(difflib.unified_diff(old.splitlines(keepends=True),new.splitlines(keepends=True),
        fromfile=f'a/{path}' if original.returncode==0 else '/dev/null',tofile=f'b/{path}'))
    sources.append({'path':path,'sha256_raw':hashlib.sha256(raw).hexdigest(),
                    'sha256_lf':hashlib.sha256(new.encode()).hexdigest()})
native=experiment/'docs/research/work-state-native-optimization-2026-10-10/native'
harness=[{'path':p,'sha256':hashlib.sha256((native/p).read_bytes()).hexdigest()}
         for p in ('Cargo.toml','Cargo.lock','src/main.rs')]
output=''.join(patch).encode('utf-8'); (evidence/'experiment.patch').write_bytes(output)
(evidence/'source-provenance.json').write_text(json.dumps({'baseline':baseline,
    'patch_sha256':hashlib.sha256(output).hexdigest(),'sources':sources,'harness':harness},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'patch_bytes':len(output),'files':len(sources),'sha256':hashlib.sha256(output).hexdigest()}))
