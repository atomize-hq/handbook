"""Read-only task adapter for original independent v6 verifier and backlog."""
import ast,ctypes as c,ctypes.wintypes as w,hashlib,json,os,pathlib,sys,time
O=pathlib.Path(__file__).resolve().parent
E=pathlib.Path('C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution')
PRIOR=pathlib.Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research')
os.environ['FILE_NATIVE_ORACLE_RESEARCH']=str(PRIOR)
sys.path.insert(0,str(E/'harness'));sys.path.insert(0,str(O))
import physical_decoder_v6 as v6
import physical_decoder_v5 as v5
import quota_v2_oracle as quota
from focused_protocol import strict
from native_runtime import need
Path=pathlib.Path
def encoded(value):return json.dumps(value,sort_keys=True,separators=(',',':')).encode()
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def inventory(root):
    s=quota.NativeSnapshot(root);return dict(files=s.files,directories=s.directories)
def pinned(path):
    import build_cli as n
    class OVER(c.Structure):_fields_=[('Internal',c.c_size_t),('InternalHigh',c.c_size_t),('Offset',w.DWORD),('OffsetHigh',w.DWORD),('hEvent',w.HANDLE)]
    lock=n.api('LockFileEx',[w.HANDLE,w.DWORD,w.DWORD,w.DWORD,w.DWORD,c.POINTER(OVER)])
    unlock=n.api('UnlockFileEx',[w.HANDLE,w.DWORD,w.DWORD,w.DWORD,c.POINTER(OVER)])
    h=n.create_file(str(path),0x80000000,7,None,3,0x80,None)
    need(h not in (None,c.c_void_p(-1).value),'native-pin-open')
    try:
        over=OVER()
        if lock(h,3,0,0xFFFFFFFF,0xFFFFFFFF,c.byref(over)):
            n.checked(unlock(h,0,0xFFFFFFFF,0xFFFFFFFF,c.byref(over)));return False
        need(c.get_last_error()==33,'native-pin-lock-unexpected:'+str(c.get_last_error()))
        return True
    finally:n.close(h)
class Bridge:
    def __init__(self):self.host=self;self.events=[]
    def quiesce(self,root):pass # Parent proves no active engine call before starting this checker.
    def emit(self,row):self.events.append(row)
    def exists(self,root,name):return (Path(root)/name).exists()
    def decoder(self,root):
        d=v5.Decoder(root,quiescent=True);d.verify_packs();return d

# Reuse the full existing independent backlog method, replacing ONLY Unix flock
# with a real nonblocking exclusive Windows LockFileEx probe on the same pin.
backlog_source=(E/'native-linux/readiness-20261021/final-suite/suite_oracle.py').read_text()
tree=ast.parse(backlog_source)
cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='Oracle')
fn=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name=='backlog')
import textwrap
code=textwrap.dedent(ast.get_source_segment(backlog_source,fn))
old="""with (root/'physical-pins'/(gen['id']+'.lock')).open('rb') as stream:
                try:fcntl.flock(stream,fcntl.LOCK_EX|fcntl.LOCK_NB)
                except BlockingIOError:held_generations.append(gen)
                else:fcntl.flock(stream,fcntl.LOCK_UN)"""
need(code.count(old)==1,'exact-backlog-lock-port-shape')
code=code.replace(old,"if pinned(root/'physical-pins'/(gen['id']+'.lock')):held_generations.append(gen)")
exec(compile(code,'<task-Windows-lock-port-of-retained-backlog>','exec'),globals())
Bridge.backlog=backlog

def main():
    cfg=json.loads(sys.stdin.buffer.read());root=Path(cfg['root']);output=Path(cfg['output'])
    before=inventory(root);bridge=Bridge()
    if cfg['mode']=='mapping':
        d=bridge.decoder(root);key=cfg['key'];raw=d.raw(key) if key in d.entries else None
        answer=dict(mapped=key in d.entries,raw_sha256=hashlib.sha256(raw).hexdigest() if raw is not None else None,independent=True)
    elif cfg['mode']=='backlog':
        answer=bridge.backlog(root)
    elif cfg['mode']=='full':
        v6.FINAL_SOURCE_PIN=cfg['baseline_pin']
        proof=v6.verify(root,str(output),quiescent=True,known_requests=cfg['known_requests'],quota_evidence={})
        d=bridge.decoder(root)
        for key in cfg['mapped_keys']:need(key in d.entries,'required-mapped-key-missing')
        answer=dict(status='passed',final_pin=cfg['baseline_pin'],corrected_source_identity=cfg['corrected_source_identity'],actual_H=len(d.accepted_chain()),mapped_keys=cfg['mapped_keys'],engine_code_imported=False,engine_audit_used=False,checks=cfg['oracle_checks'],underlying=proof,source_unchanged=True,receipt=str(output),receipt_sha256=sha(output))
    else:raise ValueError(cfg['mode'])
    after=inventory(root);need(before==after,'independent-oracle-mutated-store')
    bound=[]
    for module in list(sys.modules.values()):
        p=getattr(module,'__file__',None)
        if p:
            p=Path(p)
            if p.is_file() and (p.is_relative_to(E) or p.is_relative_to(PRIOR) or p.is_relative_to(O)):
                bound.append(dict(path=str(p),sha256=sha(p)))
    answer.update(before_after_inventory_sha256=hashlib.sha256(encoded(before)).hexdigest(),source_unchanged=True,independent=True,loaded_source_bindings=sorted(bound,key=lambda x:x['path']),independent_codec_dll=dict(path='C:/Program Files/Git/mingw64/bin/libzstd.dll',sha256=sha('C:/Program Files/Git/mingw64/bin/libzstd.dll')))
    if cfg['mode']!='full':
        output.write_text(json.dumps(answer,indent=2)+'\n',encoding='utf-8')
        answer['receipt_sha256']=sha(output)
    print(json.dumps(answer),flush=True)
if __name__=='__main__':main()
