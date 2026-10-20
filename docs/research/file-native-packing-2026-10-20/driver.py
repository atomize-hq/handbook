"""Engine child controls inside the independently watchdog-enforced outer Job."""
import json,os,subprocess,threading,time,sys,hashlib
from pathlib import Path
import guard
EXE=Path(os.environ.get('FILE_NATIVE_P_EXE',str(guard.ROOT/'build/release/file-native-incremental.exe')))
def rss(pid):
    if hasattr(guard,'rss'):return guard.rss(pid)
    try:
        if sys.platform=='darwin':
            p=subprocess.run(['ps','-o','rss=','-p',str(pid)],capture_output=True,timeout=2);n=int(p.stdout.strip())*1024;return n,n
        data=Path(f'/proc/{pid}/status').read_text();values={line.split(':')[0]:int(line.split()[1])*1024 for line in data.splitlines() if line.startswith(('VmRSS:','VmHWM:'))};return values['VmRSS'],values['VmHWM']
    except (OSError,ValueError,KeyError,subprocess.TimeoutExpired):return None
class Driver:
    def __init__(self,base):
        self.base=Path(base);self.base.mkdir(parents=True,exist_ok=False);self.events=[];self.lock=threading.Lock();self.bindings={}
    def call(self,root,action,code=0,env=None,timeout=120,exe_override=None,**fields):
        payload=dict(root=str(root),action=action,**fields)
        environ={k:v for k,v in os.environ.items() if not k.startswith('EXP_')};environ['EXP_NAVIGATION']='4'
        if env:environ.update(env)
        executable=Path(exe_override) if exe_override else EXE
        with self.lock:
            if str(executable) not in self.bindings:self.bindings[str(executable)]=hashlib.sha256(executable.read_bytes()).hexdigest()
        p=subprocess.Popen([str(executable)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=environ)
        start=time.monotonic();stop=threading.Event();peak=None;missing=0;violations=[]
        limit=256*1024**2 if action in ['resolve','point','inspect','lookup-bench'] else 2*1024**3
        def sample():
            nonlocal peak,missing
            while not stop.is_set():
                n=rss(p.pid)
                if n is None:missing+=1
                else:
                    peak=max(peak or 0,n[1])
                    if n[0]>limit:violations.append('rss');p.kill();return
                stop.wait(.01)
        t=threading.Thread(target=sample,daemon=True);t.start()
        try:out,err=p.communicate(json.dumps(payload).encode(),timeout=min(timeout,max(.001,guard.CUTOFF-time.time())))
        except subprocess.TimeoutExpired:p.kill();out,err=p.communicate();violations.append('time-censor')
        finally:stop.set();t.join(timeout=2)
        row=dict(input=payload,exe=str(executable),exe_sha256=self.bindings[str(executable)],env=env,exit=p.returncode,seconds=time.monotonic()-start,stdout=out.decode(),stderr=err.decode(),sampled_peak_rss=peak,missing_samples=missing,rss_limit=limit,violations=violations)
        with self.lock:
            self.events.append(row);(self.base/'events.json').write_text(json.dumps(self.events,indent=2),encoding='utf-8')
        assert p.returncode==code and not violations,row
        return json.loads(out) if out else None
    def head(self,root):return json.loads((Path(root)/'SELECTED.json').read_bytes())
    def publish(self,root,id,ops=None,packet=False,env=None,code=0):
        return self.call(root,'publish',code=code,env=env,request=dict(request=id,base=self.head(root),ops=ops or [],packet=packet))
