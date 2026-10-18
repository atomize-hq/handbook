"""Bounded lossless subprocess controller; engine events remain distinct from synthetic probes."""
import argparse,base64,gzip,hashlib,json,os,shutil,subprocess,time,threading,zipfile
from pathlib import Path
PYTHON=__import__('sys').executable
def sha(b):return hashlib.sha256(b).hexdigest()
class Driver:
    def __init__(self,exe,base,seconds=14400):
        self.exe=str(Path(exe).resolve());self.base=Path(base).resolve()
        self.base.mkdir(parents=True,exist_ok=True)
        self.end=time.monotonic()+seconds;self.events=self.base/'events.jsonl'
        self.event_lock=threading.Lock()
        self.session={'exe':self.exe,'sha256':sha(Path(exe).read_bytes()),'started':time.time(),
          'harness_sha256':sha(Path(__file__).read_bytes()),'platform':__import__('platform').platform()}
        source=Path(__file__).parent;hashes={}
        with zipfile.ZipFile(self.base/'session-source.zip','w',compression=zipfile.ZIP_DEFLATED) as archive:
            for path in sorted(source.rglob('*')):
                relative=path.relative_to(source)
                if not path.is_file() or any(part in ['evidence','__pycache__','target','.git'] for part in relative.parts):continue
                data=path.read_bytes();name=relative.as_posix();hashes[name]=sha(data)
                archive.writestr(zipfile.ZipInfo(name,(1980,1,1,0,0,0)),data,compress_type=zipfile.ZIP_DEFLATED)
        self.session['source_hashes']=hashes;self.session['source_archive_sha256']=sha((self.base/'session-source.zip').read_bytes())
        (self.base/'session.json').write_text(json.dumps(self.session,indent=2))
    def guard(self):
        if time.monotonic()>self.end:raise TimeoutError('total controller deadline')
        free=shutil.disk_usage(self.base).free
        reserve=(140 if os.name=='nt' else 20)*1024**3
        if free<reserve:raise RuntimeError('free-space reserve')
    def call(self,v,cap=180,env=None,allow=False):
        self.guard();start=time.perf_counter_ns()
        e=os.environ.copy();e.update(env or {})
        proc=subprocess.Popen([self.exe],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=e)
        censored=False;watch_reason=[];stop=threading.Event()
        def watch():
            while not stop.wait(30):
                try:
                    self.guard()
                    owned=[Path('C:/file-native-incremental-20261018'),Path('C:/file-native-incremental-20261018-build'),Path('C:/file-native-incremental-20261018-build-control')] if os.name=='nt' else [self.base]
                    total=0
                    for root in owned:
                        for directory,_,names in os.walk(root):
                            for name in names:
                                try:total+=(Path(directory)/name).stat().st_size
                                except FileNotFoundError:pass
                    if total>(32 if os.name=='nt' else 6)*1024**3:raise RuntimeError('allocation cap')
                except Exception as ex:
                    watch_reason.append(str(ex));proc.kill();return
        watcher=threading.Thread(target=watch,daemon=True);watcher.start()
        try:out,err=proc.communicate(json.dumps(v,ensure_ascii=False).encode(),timeout=min(cap,self.end-time.monotonic()))
        except subprocess.TimeoutExpired:
            censored=True;proc.kill();out,err=proc.communicate()
        finally:stop.set();watcher.join()
        censored=censored or bool(watch_reason)
        elapsed=time.perf_counter_ns()-start
        parsed=None;error=None
        try:parsed=json.loads(out)
        except Exception as ex:error=str(ex)
        row={'input':v,'environment':env or {},'elapsed_ns':elapsed,'exit':proc.returncode,'censored':censored,
          'controller_start_ns':start,'controller_end_ns':start+elapsed,
          'stdout_b64':base64.b64encode(out).decode(),'stdout_sha256':sha(out),'stdout_bytes':len(out),
          'stderr_b64':base64.b64encode(err).decode(),'parse_error':error,'output':parsed,
          'exe_sha256':self.session['sha256'],'watchdog_reason':watch_reason}
        with self.event_lock:
            with self.events.open('ab') as f:f.write(json.dumps(row,ensure_ascii=False,separators=(',',':')).encode()+b'\n')
        if not allow and (censored or proc.returncode or not parsed or parsed['status']!='ok'):
            raise RuntimeError(str(row)[:2000])
        return parsed
def stage1(d):
    rows=[]
    for n in [1000,10000]:
        for codec in ['json','compact']:
            root=str(d.base/f'{codec}-{n}')
            env={'EXP_CODEC':codec,'EXP_BACKEND':'cow','EXP_LAYOUT':'sharded'}
            init=d.call({'action':'init','root':root,'mode':'A','consumer':'project','count':n,'packet':False},cap=2400,env=env)
            for k in [1,8,64]:
                ops=[{'kind':'cost','id':f'd{(i*n//k+17)%n:06}','value':0} for i in range(k)]
                # labels force actual metadata updates without growing source bytes.
                for i,o in enumerate(ops):o.update(kind='label',text=f'{k}-{i}')
                v=d.call({'action':'publish','root':root,'mode':'A','policy':'slot','request':
                    {'request':f'change-{k}','base':None,'ops':ops,'packet':False}},cap=1200,env=env)
                rows.append({'n':n,'codec':codec,'k':k,'output':v})
            d.call({'action':'inspect','root':root,'audit':True},cap=1200,env=env)
    (d.base/'stage1.json').write_text(json.dumps(rows,indent=2))
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True)
    a.add_argument('--stage',choices=['codec'],required=True);ns=a.parse_args()
    stage1(Driver(ns.exe,ns.base,7200))
