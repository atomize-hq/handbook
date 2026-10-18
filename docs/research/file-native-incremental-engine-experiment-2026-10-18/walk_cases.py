"""Ordinary selected-once copy under a concurrent successor; shell controls."""
import argparse,json,os,shutil,subprocess,sys,time
from pathlib import Path
from controller import Driver
from checks import req,op,result

def run(d,source,shell):
    rows=[]
    for mode in ['A','B']:
        root=d.base/mode/'store';r=str(root);initial=result(d.call(dict(action='init',root=r,mode=mode,consumer='project',count=16)))['selected']
        if mode=='A':initial=result(d.call(dict(action='publish',root=r,request=req('packet-copy',[],packet=True))))['selected']
        dest=d.base/mode/'copied';ready=d.base/(mode+'-ready');release=d.base/(mode+'-release')
        env=os.environ.copy()
        if mode=='B':env.update(WALK_READY=str(ready),WALK_RELEASE=str(release))
        cmd=([shutil.which('pwsh') or 'powershell','-NoProfile','-ExecutionPolicy','Bypass','-File',str(source/'walkthrough.ps1'),'-Root',r,'-Destination',str(dest),'-Python',sys.executable]
             if shell=='powershell' else ['bash',str(source/'walkthrough.sh'),r,str(dest)])
        with (d.base/(mode+'.stdout')).open('wb') as stdout,(d.base/(mode+'.stderr')).open('wb') as stderr:
            p=subprocess.Popen(cmd,env=env,stdout=stdout,stderr=stderr)
            if mode=='B':
                deadline=time.monotonic()+30
                while not ready.exists():assert p.poll() is None and time.monotonic()<deadline;time.sleep(.01)
                retained=ready.read_text().strip()
                newer=result(d.call(dict(action='publish',root=r,request=req('while-copy-paused',[op('import',text='new selected source\r\n')],base=initial))))['selected']
                assert newer!=initial and retained.endswith(json.loads((root/f"publications/{initial['publication']}.json").read_bytes())['attempt'])
                release.write_text('release')
            assert p.wait(timeout=180)==0
        rows.append({'mode':mode,'shell':shell,'argv':cmd,'copied_manifest':__import__('hashlib').sha256((dest/'MANIFEST.json').read_bytes()).hexdigest(),'paused_successor':mode=='B','bound_selected':initial})
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--source',required=True);a.add_argument('--shell',choices=['powershell','bash'],required=True);ns=a.parse_args()
    d=Driver(ns.exe,ns.base,900);(d.base/'walk-cases.json').write_text(json.dumps(run(d,Path(ns.source),ns.shell),indent=2))
