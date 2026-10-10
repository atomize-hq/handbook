"""Capture real GitNexus attempts; never reinterpret a miss as successful impact."""
import json, pathlib, subprocess, sys
repo = pathlib.Path(sys.argv[1]).resolve()
out = pathlib.Path(sys.argv[2])
targets = [
 ('required','crates/engine/src/work_state/store.rs'),
 ('inventory','crates/engine/src/work_state/store.rs'),
 ('manifest','crates/engine/src/work_state/store.rs'),
 ('open','crates/engine/src/work_state/store.rs'),
 ('WorkLock.validate#0','crates/engine/src/work_state/store.rs'),
 ('pin_windows_parent','crates/engine/src/repository_durability.rs'),
 ('open_exact','crates/engine/src/repository_durability.rs'),
 ('list_directory_bounded','crates/engine/src/repository_durability.rs'),
 ('read_regular_bounded','crates/engine/src/repository_durability.rs'),
 ('canonical_json_bytes','crates/engine/src/definition_identity.rs'),
 ('scan','crates/engine/src/work_state/transaction.rs'),
 ('parsed','crates/engine/src/work_state/transaction.rs'),
 ('execute','crates/engine/src/work_state/transaction.rs'),
 ('validate_authority_history','crates/engine/src/work_state/transaction.rs'),
 ('RetainedRepositoryTreeGuardV1.validate#0','crates/engine/src/repository_durability.rs'),
 ('windows_handle_info','crates/engine/src/repository_durability.rs'),
 ('open_windows_directory_guard','crates/engine/src/repository_durability.rs'),
 ('with_work_registration_establishment','crates/engine/src/local_authority/work_registration.rs'),
 ('WorkRegistrationCurrentV1.validate_current#0','crates/engine/src/local_authority/work_registration.rs'),
 ('RetainedRepositoryRootV1','crates/engine/src/repository_durability.rs'),
 ('WorkLock','crates/engine/src/work_state/store.rs'),
]
records = []
for name, file in targets:
 args = ['node','.gitnexus/run.cjs','impact',name.split('.')[-1].split('#')[0], '--file',file,'--direction','upstream','--summary-only']
 if '.' in name and '#' in name:
  args += ['--uid',f'Function:{file}:{name}']
 if name in ('WorkLock','RetainedRepositoryRootV1'):
  args += ['--kind','Struct']
 p = subprocess.run(args,cwd=repo,capture_output=True,text=True,timeout=120)
 raw = p.stdout + p.stderr
 try:
  result = json.JSONDecoder().raw_decode(p.stdout[p.stdout.index('{'):])[0]
 except (ValueError, json.JSONDecodeError):
  result = {'error':'unparsed tool output'}
 records.append({'command':args,'exit':p.returncode,'result':result,'raw':raw})
 out.write_text(json.dumps(records,indent=2)+'\n',encoding='utf-8')
 print(json.dumps({'name':name,'risk':result.get('risk'),'impacted':result.get('impactedCount'),'summary':result.get('summary'),'error':result.get('error'),'status':result.get('status')}),flush=True)
