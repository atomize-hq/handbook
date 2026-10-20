"""Actual prior recovery capsule, final P engine, N oracle and strict child caps."""
import argparse,importlib.util,json,os,subprocess,sys,threading,time
from pathlib import Path
from types import SimpleNamespace
from driver import rss
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'file-native-navigation-2026-10-19'));import oracle
sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'))
spec=importlib.util.spec_from_file_location('prior_recovery',HERE.parent/'file-native-recovery-reclamation-2026-10-19/capsule.py');prior=importlib.util.module_from_spec(spec);spec.loader.exec_module(prior)
def run(exe,base):
    base=Path(base);rows=[]
    def bounded(argv,input=None,env=None,**kw):
        action=json.loads(input)['action'];limit=256*1024**2 if action in ['resolve','point','inspect'] else 2*1024**3
        p=subprocess.Popen(argv,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env);peak=None;missing=0;stop=threading.Event();bad=[]
        def sample():
            nonlocal peak,missing
            while not stop.is_set():
                n=rss(p.pid)
                if n is None:missing+=1
                else:
                    peak=max(peak or 0,n[1])
                    if n[0]>limit:bad.append('rss');p.kill();return
                stop.wait(.01)
        t=threading.Thread(target=sample,daemon=True);t.start();start=time.monotonic()
        try:out,err=p.communicate(input,timeout=120)
        except subprocess.TimeoutExpired:p.kill();out,err=p.communicate();bad.append('120-second-censor')
        finally:stop.set();t.join(timeout=2)
        rows.append(dict(action=action,exit=p.returncode,seconds=time.monotonic()-start,peak_rss=peak,missing_samples=missing,limit=limit,violations=bad));base.parent.mkdir(parents=True,exist_ok=True);(base.parent/'loose-regression-resources.json').write_text(json.dumps(rows,indent=2));assert not bad
        return subprocess.CompletedProcess(argv,p.returncode,out,err)
    prior.subprocess=SimpleNamespace(run=bounded);return prior.run(Path(exe),base)
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);n=a.parse_args();run(n.exe,n.base)
