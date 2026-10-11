"""Retain exact build command, output, toolchain, source and binary provenance."""
import hashlib,json,pathlib,subprocess,sys,time
HERE=pathlib.Path(__file__).resolve().parent
source=pathlib.Path(sys.argv[1]).resolve();target=pathlib.Path(sys.argv[2]).resolve();name=sys.argv[3]
subprocess.run([sys.executable,str(HERE/'prepare_driver.py'),str(source)],check=True)
manifest=source/'docs/research/work-registration-phase-refactor-2026-10-11/native/Cargo.toml'
command=['cargo','build','--locked','--offline','--release','--jobs','2','--manifest-path',str(manifest),'--target-dir',str(target)]
if len(sys.argv)>4 and sys.argv[4]=='profile':command+=['--features','profile']
start=time.perf_counter()
with (HERE/f'build-{name}.txt').open('w',encoding='utf-8') as log:
    p=subprocess.run(command,cwd=source,stdout=log,stderr=subprocess.STDOUT)
exe=target/'release/work-state-capacity-research.exe'
files=list(sorted((source/'crates/engine/src/work_state').glob('*.rs')))+[source/'crates/engine/src'/f for f in ['lib.rs','local_authority.rs','local_authority/work_registration.rs','local_authority/local_check.rs','repository_durability.rs','definition_identity.rs']]+[source/'crates/sdk/src/work_state.rs',source/'crates/sdk/tests/work_kind_support/mod.rs',source/'crates/sdk/tests/work_state_support/mod.rs',source/'Cargo.toml',source/'Cargo.lock',manifest,manifest.parent/'Cargo.lock',manifest.parent/'src/main.rs']
if (source/'crates/engine/src/research_profile.rs').exists():files.append(source/'crates/engine/src/research_profile.rs')
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
record={'command':command,'exit':p.returncode,'wall_s':time.perf_counter()-start,'source':str(source),'target':str(target),'source_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip(),
        'exe':str(exe),'exe_sha256':sha(exe) if exe.exists() else None,'rustc':subprocess.check_output(['rustc','-Vv'],text=True).strip(),'sources':{str(f.relative_to(source)).replace('\\','/'):sha(f) for f in files}}
(HERE/f'build-{name}.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8');print(json.dumps({k:v for k,v in record.items() if k!='sources'}),flush=True)
if p.returncode:sys.exit(p.returncode)
