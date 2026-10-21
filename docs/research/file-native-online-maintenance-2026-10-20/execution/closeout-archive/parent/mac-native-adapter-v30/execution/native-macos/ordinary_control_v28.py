import json,sys
from pathlib import Path
import capsule_adapter_v28 as a
r=a.ROOT;h=Path(__file__).resolve().parent;base=r/'fixtures/ordinary-tiny-v28';store=base/'disposable/store';generation=store/'generations/tiny-0';generation.mkdir(parents=True,exist_ok=False)
a.report(generation/'MANIFEST.json',{'fake':'tiny-control-only'});(generation/'doc.txt').write_bytes(b'tiny ordinary data\n')
pub={'attempt':'tiny-0','generation_manifest':a.roles.sha256(generation/'MANIFEST.json')};a.report(store/'publications/p0.json',pub)
s={'publication':'p0','digest':a.roles.sha256(store/'publications/p0.json')};a.report(store/'SELECTED.json',s);a.report(store/'STORE.json',{'mode':'B'})
b=a.Bridge({'control_only':True,'control_argv':[sys.executable],'phase':'tiny','role_identity_budget':16,'run_name':'ordinary-tiny-v28','stdout_cap':65536})
wrapper='import subprocess,sys,time;time.sleep(.12);p=subprocess.run(sys.argv[1:]);time.sleep(.08);raise SystemExit(p.returncode)'
def copy(label,captured='',save='',code=0):
 argv=[sys.executable,'-B','-c',wrapper,'/bin/bash',str(h/'ordinary_copy_v27.sh'),sys.executable,str(h),str(store),str(base/label),str(captured),str(save)]
 p,res=b.invoke('reader',argv,code=code,timeout=5)
 return json.loads(p.stdout) if not code else p.stderr.decode()
try:
 first=copy('first',save=base/'captured.json');assert first['selection_read_count']==1 and first['engine_or_pack_decoder_used'] is False
 a.report(store/'SELECTED.json',{'publication':'missing-new-current','digest':'0'*64})
 second=copy('second',captured=base/'captured.json');assert first['files']==second['files'] and first['selected']==second['selected']
 (generation/'doc.txt').unlink();(generation/'MANIFEST.json').unlink();generation.rmdir()
 error=copy('retired',captured=base/'captured.json',code=1);assert 'selected-generation-missing' in error
finally:b.close()
a.report(r/'evidence/control/ordinary-tiny-v28-proof.json',{'kind':'synthetic-selected-once-shell-controls','calls':len(b.calls),'selection_read_count':1,'frozen_selector_after_current_advance_exact_copy':True,'missing_generation_refused':True,'no_engine_or_pack_decoder':True,'real_engine_immediate_capsule_executed':False})
print('ORDINARY_TINY_CONTROL_PASS')
