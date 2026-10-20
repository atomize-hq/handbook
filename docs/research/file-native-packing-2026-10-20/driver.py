"""Engine child controls inside the independently watchdog-enforced outer Job."""
import json,os,subprocess,threading,time
from pathlib import Path
import guard
EXE=guard.ROOT/'build/release/file-native-incremental.exe'
class Driver:
    def __init__(self,base):
        self.base=Path(base);self.base.mkdir(parents=True,exist_ok=False);self.events=[]
    def call(self,root,action,code=0,env=None,timeout=120,**fields):
        payload=dict(root=str(root),action=action,**fields)
        environ={k:v for k,v in os.environ.items() if not k.startswith('EXP_')};environ['EXP_NAVIGATION']='4'
        if env:environ.update(env)
        p=subprocess.Popen([str(EXE)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=environ)
        start=time.monotonic();stop=threading.Event();peak=None;missing=0;violations=[]
        limit=256*1024**2 if action in ['resolve','point','inspect'] else 2*1024**3
        def sample():
            nonlocal peak,missing
            while not stop.is_set():
                n=guard.rss(p.pid)
                if n is None:missing+=1
                else:
                    peak=max(peak or 0,n[1])
                    if n[0]>limit:violations.append('rss');p.kill();return
                stop.wait(.01)
        t=threading.Thread(target=sample,daemon=True);t.start()
        try:out,err=p.communicate(json.dumps(payload).encode(),timeout=min(timeout,max(.001,guard.CUTOFF-time.time())))
        except subprocess.TimeoutExpired:p.kill();out,err=p.communicate();violations.append('time-censor')
        finally:stop.set();t.join(timeout=2)
        row=dict(input=payload,env=env,exit=p.returncode,seconds=time.monotonic()-start,stdout=out.decode(),stderr=err.decode(),sampled_peak_rss=peak,missing_samples=missing,rss_limit=limit,violations=violations)
        self.events.append(row);(self.base/'events.json').write_text(json.dumps(self.events,indent=2),encoding='utf-8')
        assert p.returncode==code and not violations,row
        return json.loads(out) if out else None
    def head(self,root):return json.loads((Path(root)/'SELECTED.json').read_bytes())
    def publish(self,root,id,ops=None,packet=False,env=None,code=0):
        return self.call(root,'publish',code=code,env=env,request=dict(request=id,base=self.head(root),ops=ops or [],packet=packet))
