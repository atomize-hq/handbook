"""Separately admitted packed operational capture/restore, with prewrite caps.

Keeps FNOP1 framing and the existing independent transport/inventory contract.
Only local stream plumbing and tighter resource bounds adapt the old harness.
Presence of this script grants no capture or restore admission.
"""
import argparse,contextlib,gzip,hashlib,importlib.util,json,threading,types
from pathlib import Path
from guard import current

HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020')
ROOT=R/'history-restored/disposable/store';CAPSULE=R/'packed-operational-backup';RESTORED=R/'packed-restored/disposable/store'
SOURCE=HERE.parent/'file-native-recovery-reclamation-2026-10-19/operational_backup.py'
CAP=2684354560;FILES=560000;CAPTURE_BYTES=620000000;RESTORE_BYTES=600000000
read=lambda p:json.loads(Path(p).read_bytes())
def sha_file(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()

class Quota:
    def __init__(self,limit):self.limit=limit;self.used=0;self.lock=threading.Lock()
    def open(self,path,mode):return CappedFile(Path(path).open(mode),self)
class CappedFile:
    def __init__(self,file,quota):self.file=file;self.quota=quota
    def write(self,data):
        with self.quota.lock:
            if self.quota.used+len(data)>self.quota.limit:raise ValueError('prewrite-byte-cap')
            self.quota.used+=len(data)
            written=self.file.write(data)
            if written!=len(data):raise ValueError('short-write')
            return written
    def __getattr__(self,name):return getattr(self.file,name)
    def __enter__(self):return self
    def __exit__(self,*args):self.file.close()

def adapted(action):
    original=SOURCE.read_text();text=original
    replacements=[('MAX_FILES = 5_000_000','MAX_FILES = 12000'),('MAX_BYTES = 32 * 1024**3','MAX_BYTES = 600000000'),("gzip.open(destination/'operational.bin.gz','wb',compresslevel=1)","capped_gzip(destination/'operational.bin.gz','wb',compresslevel=1)"),("(destination/'files.jsonl').open('wb')","capture_open(destination/'files.jsonl','wb')"),("target.open('xb')","restore_open(target,'xb')")]
    for before,after in replacements:
        assert text.count(before)==(2 if before=="target.open('xb')" else 1),before
        text=text.replace(before,after)
    module=types.ModuleType('bounded_packed_operational_backup');module.__dict__['__name__']='bounded_packed_operational_backup'
    capture_quota=Quota(CAPTURE_BYTES);restore_quota=Quota(RESTORE_BYTES)
    @contextlib.contextmanager
    def capped_gzip(path,mode,compresslevel):
        with capture_quota.open(path,'wb') as raw:
            with gzip.GzipFile(filename='',mode=mode,compresslevel=compresslevel,fileobj=raw) as stream:yield stream
    module.__dict__.update(capped_gzip=capped_gzip,capture_open=capture_quota.open,restore_open=restore_quota.open)
    exec(compile(text,str(SOURCE)+'[bounded-adapter]','exec'),module.__dict__)
    module.EXCLUDED|={'pack-readers.lock','pack-maintenance.lock'}
    folder=R/'packed-backup-inputs';folder.mkdir(exist_ok=True)
    destination=folder/(action+'-operational_backup.py');assert not destination.exists();destination.write_text(text,encoding='utf-8')
    binding=dict(original_source=str(SOURCE),original_sha256=sha_file(SOURCE),adapted_source=str(destination),adapted_sha256=sha_file(destination),wrapper_sha256=sha_file(__file__),replacement_counts=[dict(before=a,after=b,count=original.count(a)) for a,b in replacements],profile_unchanged='FNOP1 quiescent-full-operational',capture_prewrite_combined_transport_inventory_cap=CAPTURE_BYTES,restore_prewrite_all_file_bytes_cap=RESTORE_BYTES,excluded=sorted(module.EXCLUDED),no_engine_core_change=True)
    (folder/(action+'-binding.json')).write_text(json.dumps(binding,indent=2))
    return module,capture_quota,restore_quota

def admission(action,reserve):
    actual=current();actual.update(action=action,reserve_bytes=reserve,prospective_bytes=actual['total_new_bytes']+reserve,prospective_files=actual['total_new_files']+12010,cap=CAP)
    (R/'receipts'/('packed-'+action+'-fresh-admission.json')).write_text(json.dumps(actual,indent=2))
    assert actual['prospective_bytes']<=CAP and actual['prospective_files']<=FILES and actual['free_bytes']>=140*1024**3
    return actual

def run(action):
    cleanup=read(R/'long-matrix-fixed2/cleanup-summary.json');assert cleanup['deferred_bytes']==0 and cleanup['remaining_mapped_loose_files']==[] and cleanup['actual_successfully_deleted_bytes']==1221001465
    verified=read(R/'long-matrix-fixed2/postcleanup-independent.json');assert verified['raw_records']==504833 and verified['semantic']['publications']==101007 and verified['semantic']['permanent_digest']=='f398ab10c2020b988d46530c6e93e0c6c6e0188d6e66e9b8e4e07e96c107f63e'
    admission(action,(CAPTURE_BYTES if action=='capture' else RESTORE_BYTES)+33554432)
    backup,capture_quota,restore_quota=adapted(action)
    if action=='capture':
        assert read(ROOT/'SELECTED.json')==read(R/'long-matrix/frozen.json')['selected']
        result=backup.capture(ROOT,CAPSULE,'d70f3864f2888d46d9112213b4a03f8d82745a8e',True)
        assert result['files']<=12000 and result['logical_bytes']<=RESTORE_BYTES and capture_quota.used<=CAPTURE_BYTES
        result['transport_inventory_actual_bytes']=capture_quota.used
    else:
        manifest=read(CAPSULE/'manifest.json');assert manifest['logical_bytes']<=RESTORE_BYTES and manifest['files']<=12000
        assert sha_file(CAPSULE/'files.jsonl')==manifest['inventory_sha256']
        count=total=0
        with (CAPSULE/'files.jsonl').open('rb') as f:
            for line in f:
                row=json.loads(line);assert type(row['bytes']) is int and 0<=row['bytes']<=512*1024**2;total+=row['bytes'];count+=1
        assert (count,total)==(manifest['files'],manifest['logical_bytes'])
        result=backup.restore(CAPSULE,RESTORED);assert restore_quota.used==total;result['prewrite_charged_bytes']=restore_quota.used
        assert not (RESTORED/'DISPOSABLE.json').exists()
    (R/'receipts'/('packed-'+action+'-result.json')).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('action',choices=['capture','restore']);run(parser.parse_args().action)
