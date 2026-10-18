"""Native/WSL outer watchdog, including build and all subprocess descendants."""
import argparse, hashlib, json, os, shutil, signal, subprocess, sys, time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--root',required=True);a.add_argument('--source',required=True);a.add_argument('--pin',required=True)
a.add_argument('--reserve-gib',type=int,default=20);a.add_argument('--cap-gib',type=int,default=6);ns=a.parse_args()
root=Path(ns.root).resolve();root.mkdir(parents=True,exist_ok=False);source=Path(ns.source).resolve()
started=time.monotonic();end=started+105*60;rows=[]
env=os.environ.copy();env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+env.get('PATH','')
def size():
    return sum((Path(d)/n).stat().st_size for d,_,names in os.walk(root) for n in names if (Path(d)/n).exists())
def command(name,cmd,cap):
    begin=time.monotonic();censor=None
    with (root/(name+'.stdout')).open('wb') as out,(root/(name+'.stderr')).open('wb') as err:
        p=subprocess.Popen(cmd,cwd=source,env=env,stdout=out,stderr=err,start_new_session=True)
        while p.poll() is None:
            if time.monotonic()>min(end,begin+cap):censor='deadline'
            elif shutil.disk_usage(root).free<ns.reserve_gib*1024**3:censor='free-reserve'
            elif size()>ns.cap_gib*1024**3:censor='allocation-cap'
            if censor:
                os.killpg(p.pid,signal.SIGKILL);p.wait();break
            time.sleep(2)
    row={'name':name,'argv':cmd,'exit':p.returncode,'censored':bool(censor),'reason':censor,'seconds':time.monotonic()-begin}
    rows.append(row);(root/'outer.json').write_text(json.dumps({'pin':ns.pin,'prospective_work_seconds':6300,'report_reserve_seconds':900,'rows':rows},indent=2))
    return p.returncode==0 and not censor

target=str(root/'build')
if command('test',['cargo','test','--locked','--workspace','--target-dir',target],600) and command('build',['cargo','build','--release','--locked','--workspace','--target-dir',target],600):
    # Preserve capsule source bytes; apply tighter host caps through a distinct
    # adapter, whose bytes are themselves retained alongside the raw observations.
    adapter='''import runpy,sys,time\nfrom controller import Driver\noriginal_init=Driver.__init__\noriginal_call=Driver.call\ndef init(self,*args,**kw):\n original_init(self,*args,**kw);self.end=min(self.end,DEADLINE)\ndef call(self,v,cap=180,env=None,allow=False):\n return original_call(self,v,min(cap,600 if "smoke" in str(self.base) else 180),env,allow)\nDriver.__init__=init\nDriver.call=call\nrunpy.run_path(NATIVE,run_name="__main__")\n'''.replace('DEADLINE',repr(end)).replace('NATIVE',repr(str(source/'native.py')))
    adapter_path=root/'adapter.py';adapter_path.write_text(adapter)
    env['PYTHONPATH']=str(source)
    command('native',[sys.executable,str(adapter_path),'--exe',str(root/'build/release/file-native-incremental'),'--base',str(root/'results'),'--pin',ns.pin],max(1,end-time.monotonic()))
(root/'finished.json').write_text(json.dumps({'pin':ns.pin,'elapsed_seconds':time.monotonic()-started,'logical_bytes':size(),'rows':rows},indent=2))
