"""Companions share run_lane's original monotonic deadline; capsule code is copied intact."""
import json,os,re,runpy,shutil,signal,subprocess,sys,time
from pathlib import Path

def owned_bytes():
    total=0
    for owned in Path.home().glob('file-native-incremental-*'):
        if not owned.is_dir():continue
        for directory,_,names in os.walk(owned):
            for name in names:
                try:total+=(Path(directory)/name).stat().st_size
                except FileNotFoundError:pass
    return total

def inner(script,args):
    from controller import Driver
    original=Driver.__init__;deadline=float(os.environ['EXP_NATIVE_DEADLINE'])
    def init(self,*a,**kw):original(self,*a,**kw);self.end=min(self.end,deadline)
    Driver.__init__=init;sys.argv=[str(script),*args];runpy.run_path(str(script),run_name='__main__')

def main(script,args):
    base=Path(args[args.index('--base')+1]).resolve();root=base.parent
    assert root.name=='file-native-incremental-wsl-final-20261018','owned lane only'
    record=root/(base.name+'-budget.json');adapter=root/'adapter.py'
    row=dict(script=str(script),argv=args,work_window_seconds=6300,case_cap_seconds=890,started_epoch=time.time())
    matches=re.findall(r'self\.end=min\(self\.end,([0-9.]+)\)',adapter.read_text()) if adapter.exists() else []
    if len(matches)!=1:
        row['unperformed']='original native adapter/deadline unavailable';record.write_text(json.dumps(row,indent=2));return 0
    deadline=float(matches[0]);row['original_monotonic_deadline']=deadline;remaining=deadline-time.monotonic()
    if remaining<=0:
        row['unperformed']='original native work window expired';record.write_text(json.dumps(row,indent=2));return 0
    env=os.environ.copy();env['EXP_NATIVE_DEADLINE']=str(deadline)
    p=subprocess.Popen([sys.executable,str(Path(__file__).resolve()),'--inner',str(script),*args],env=env,start_new_session=True)
    end=min(deadline,time.monotonic()+890);reason=None
    while p.poll() is None:
        if time.monotonic()>=end:reason='original native deadline or companion cap'
        elif shutil.disk_usage(root).free<40*1024**3:reason='guest free reserve'
        else:
            size=owned_bytes()
            if size>6*1024**3:reason='combined guest allocation'
        if reason:os.killpg(p.pid,signal.SIGKILL);p.wait();break
        try:p.wait(timeout=min(2,max(.01,end-time.monotonic())))
        except subprocess.TimeoutExpired:pass
    row.update(exit=p.returncode,censored=bool(reason),reason=reason,finished_epoch=time.time());record.write_text(json.dumps(row,indent=2));return 124 if reason else p.returncode

if __name__=='__main__':
    assert sys.argv[1]=='--inner';inner(Path(sys.argv[2]),sys.argv[3:])
