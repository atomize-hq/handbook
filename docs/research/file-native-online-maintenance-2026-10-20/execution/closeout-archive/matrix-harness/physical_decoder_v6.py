"""Quota-v2; pending."""
import gzip,hashlib,json,re,sys,time
from pathlib import Path
import physical_decoder_v5 as v5
import quota_v2_oracle as quota
from outcome_oracle import strict
FINAL_SOURCE_PIN='0628322e4b48e0efad19cdc32238a622d4aaa7a8'
COMPANIONS={"physical_decoder_v5.py":"1347778df8839da571603b215a8f3aecac70b40eff571ddbad0cb19e27bf5be9","quota_v2_oracle.py":"bf4e892298760c79ce5a164c3294e069e5188b8c2c260c8477e9c93e64c468d3","verify_operational_v2.py":"e517d07c5f345e9fb3a90535d5185828147b45a27896e658dbf4164ea7a76caf","physical_decoder.py":"eb00a0b49ce4754b23bb4e4a340ec99070805cf43b6f2e1c1125d029f5453001","paged_index.py":"788286c46af4c53b045e0c029ae7ef22f83ecd92e31de18147c7e23fc44660ea","outcome_oracle.py":"a5721ab4102ab742efe66f546c9016b051cd3914e07700cc3de7c0f55a3ced02"}
def verify(root,output,inventory=None,baseline=None,*,quiescent=False,known_requests=None,request_provenance=None,quota_evidence=None,artifact_limits=None):
 assert quiescent and type(FINAL_SOURCE_PIN) is str and re.fullmatch(r'[0-9a-f]{40}',FINAL_SOURCE_PIN),'final-source-not-frozen'
 source=Path(__file__).resolve().parent
 for name,digest in COMPANIONS.items():assert hashlib.sha256((source/name).read_bytes()).hexdigest()==digest,'companion-source-changed'
 artifacts=_Artifacts(artifact_limits,output) if artifact_limits is not None else None
 started=time.monotonic();d=v5.Decoder(root,quiescent=True);rows=d.verify_packs()
 if inventory:assert rows==v5.prior.legacy.expected_inventory(inventory),'raw-inventory'
 with (Path(root)/'RESERVATIONS.json').open('rb') as f:reg=quota.registry(f.read(65537),d.config['store_id'])
 snap=quota.NativeSnapshot(root) if reg['version']==2 else None
 try:
  ownership=quota.verify(d,snap,quota_evidence)
  ownership['proof']['final_source_pin']=FINAL_SOURCE_PIN
  sys.path.insert(0,str(v5.prior.PRIOR/'file-native-recovery-reclamation-2026-10-19'))
  if ownership['version']==2:
   import verify_operational_v2 as semantic_module
   semantic_module.checked=d.checked
   semantic=semantic_module.verify(root,baseline,quota=ownership)
  else:
   import verify_operational as semantic_module
   semantic_module.checked=d.checked;semantic=semantic_module.verify(root,baseline)
  sys.path.insert(0,str(v5.prior.PRIOR/'file-native-navigation-2026-10-19'))
  import oracle
  oracle.checked=d.checked;navigation=oracle.verify(root)
  from outcome_oracle import verify as intents
  if request_provenance is not None:known_requests=v5.resolve_provenance(d.accepted_chain(),request_provenance,known_requests)
  request_intents=intents(root,decoder=d,known_requests=known_requests)
  identity_output=None
  if snap:
   identity_output=Path(output).with_suffix('.quota-identities.jsonl.gz')
   assert not identity_output.exists()
   if artifacts:bounded_v6_identities(snap,identity_output,artifacts)
   else:
    with gzip.open(identity_output,'xb',compresslevel=1) as f:
     for name,fact in sorted(snap.files.items()):f.write((json.dumps(dict(path=name,**fact),separators=(',',':'))+'\n').encode())
     f.write((json.dumps(dict(directories=snap.directories),separators=(',',':'))+'\n').encode())
   with gzip.open(identity_output,'rb') as f:
    for line in f:strict(line)
 finally:
  if snap:snap.unchanged()
 digest=hashlib.sha256()
 for row in rows:digest.update((json.dumps(row,separators=(',',':'))+'\n').encode())
 result=dict(schema_pin=FINAL_SOURCE_PIN,quota_format_sha256=quota.FORMAT_SHA256,companion_sha256=COMPANIONS,quota=ownership['proof'],mapped_cohort_closure=d.mapped_cohort_proof,paged_source_pin=v5.paged_index.SOURCE_PIN,paged_full_audits=getattr(d,'paged_audits',[]),raw_records=len(rows),raw_bytes=sum(x['raw_length'] for x in rows),inventory_digest=digest.hexdigest(),physical_metadata=d.physical_metadata,semantic=semantic,navigation=navigation,request_intents=request_intents,mapped_occurrences=sum(map(len,getattr(d,'variants',{}).values())),seconds=time.monotonic()-started,quota_identity_manifest=str(identity_output) if identity_output else None,qualification='full state audit')
 for name,digest in COMPANIONS.items():assert hashlib.sha256((source/name).read_bytes()).hexdigest()==digest,'companion-source-changed'
 if artifacts:bounded_v6_result(result,output,artifacts)
 else:
  Path(output).write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
 return result

class ArtifactCensor(RuntimeError):
    pass

class _ArtifactSink:
    def __init__(self,budget,file,identity):
        self.budget=budget;self.file=file;self.identity=identity
    def write(self,data):
        cap=self.budget.maximum-(self.budget.reserve if self.identity else 0)
        if self.budget.written+len(data)>cap:raise ArtifactCensor('v6-artifact-byte-censor')
        n=self.file.write(data)
        self.budget.written+=n
        if n!=len(data):raise OSError('v6-short-artifact-write')
        return n
    def flush(self):self.file.flush()
    def tell(self):return self.file.tell()

class _Artifacts:
    def __init__(self,value,output):
        assert type(value) is dict and set(value)=={'version','result_path','identity_path','max_bytes','result_reserve_bytes'},'artifact-limits-shape'
        assert type(value['version']) is int and value['version']==1
        for k in ['max_bytes','result_reserve_bytes']:
            assert type(value[k]) is int and 0<=value[k]<2**64
        assert 0<value['result_reserve_bytes']<=value['max_bytes']
        self.result=Path(output).absolute();self.identity=self.result.with_suffix('.quota-identities.jsonl.gz')
        assert value['result_path']==str(self.result) and value['identity_path']==str(self.identity),'artifact-path-binding'
        self.maximum=value['max_bytes'];self.reserve=value['result_reserve_bytes'];self.written=0
    def emit(self,path,rows,identity=False):
        path=Path(path).absolute()
        assert path==(self.identity if identity else self.result)
        stage=path.with_name(path.name+'.partial')
        assert not path.exists() and not stage.exists(),'artifact-preexists'
        with stage.open('xb') as file:
            sink=_ArtifactSink(self,file,identity)
            if identity:
                with gzip.GzipFile(fileobj=sink,mode='wb',filename='',mtime=0,compresslevel=1) as stream:
                    for row in rows:stream.write((json.dumps(row,separators=(',',':'))+'\n').encode())
            else:
                encoder=json.JSONEncoder(indent=2,ensure_ascii=False)
                for chunk in encoder.iterencode(rows):sink.write(chunk.encode())
                sink.write(b'\n')
            sink.flush()
            import os
            os.fsync(file.fileno())
        # Only a complete closed stream acquires its final name. Partials remain.
        assert not path.exists(),'artifact-final-race'
        stage.rename(path)

def bounded_v6_identities(snapshot,path,budget):
    assert type(budget) is _Artifacts
    def rows():
        for name,fact in sorted(snapshot.files.items()):yield dict(path=name,**fact)
        yield dict(directories=snapshot.directories)
    budget.emit(path,rows(),True)

def bounded_v6_result(result,path,budget):
    assert type(budget) is _Artifacts
    budget.emit(path,result)
