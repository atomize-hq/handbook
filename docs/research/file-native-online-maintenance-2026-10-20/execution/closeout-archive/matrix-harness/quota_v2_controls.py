import argparse,base64,copy,hashlib,json,os,sys
import quota_v2_oracle as q
R='terminal-receipt-stage'
C='terminal-control-stage'
class MemorySnapshot:
 def __init__(self,records,overrides):
  self.records=copy.deepcopy(records);self.native=False;self.directories=['.']
  self.files={name:dict(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),identity=[1,n+1],links=1) for n,(name,raw) in enumerate(sorted(records.items()))}
  for name,facts in overrides.items():self.files[name].update(facts)
 def read(self,name,limit):
  assert name in self.records,'missing-file'
  raw=self.records[name];assert len(raw)<=limit
  assert hashlib.sha256(raw).hexdigest()==self.files[name]['sha256']
  return raw
class Authority:
 def __init__(self,chain,mode):
  self.root=q.Path('V');self.chain=chain
  self.config=dict(store_id='s',mode=mode)
 def accepted_chain(self):return self.chain
def fixture(mode='committed'):
 s0=dict(publication='p0',digest='0'*64);s1=dict(publication='p1',digest='1'*64)
 chain={n:(s,dict(id=s['publication'],sequence=n,attempt='a'+str(n),request='a'+str(n),intent='a'*64,allocation_owner=dict(attempt='a'+str(n),conservative_bytes=67108864,includes_intermediates_and_stages=True))) for n,s in enumerate([s0,s1])}
 c=dict(purpose='publication',version=2,store_id='s',attempt='x',expected=s0,request=dict(request='r',base=s0,ops=[],packet=False),intent='',graph=dict(version=1,consumer='project',budget=1000,cost=0,count=0,tree=None,fallback=False),projection=None,changes=None,changed=[],removed=[],generation_manifest=None,checkpoint=None,packet_ref=None,replay_objects=[],replay_files={})
 c['intent']=q.intent(c['request']);raw=q.pretty(c)
 r=dict(request=c['request']['request'],intent=c['intent'],base=s0,bytes=67108864,state='sealed-preselection',used=len(raw)+4096+65536,candidate_digest=hashlib.sha256(raw).hexdigest())
 v=dict(version=2,store_id='s',active={},legacy=[],terminal_count=0,terminal_bytes=0)
 t=q.receipt('x',r,'s');records={'SELECTED.json':q.pretty(s1),'attempts/x/candidate.json':raw}
 if mode in {'active','pending'}:v['active']['x']=copy.deepcopy(r)
 if mode in {'committed','pending'}:records['abandoned/x']=q.pretty(t)
 if mode=='committed':v['terminal_count']=1;v['terminal_bytes']=t['charge_bytes']
 return dict(registry=v,records=records,chain=chain,candidate=c,reservation=r,receipt=t,overrides={},evidence={})
def snapshot(f):
 records=dict(f['records']);records['RESERVATIONS.json']=q.pretty(f['registry'])
 return MemorySnapshot(records,f['overrides'])
def invoke(f):
 snap=snapshot(f);before=copy.deepcopy((snap.records,snap.files))
 answer=q.verify(Authority(f['chain'],f.get('mode','A')),snap,f['evidence'])
 assert before==(snap.records,snap.files),'fixture-mutated'
 return answer
def set_receipt(f,key,value):
 t=q.strict(f['records']['abandoned/x']);t[key]=value;f['records']['abandoned/x']=q.pretty(t)
def set_candidate(f,key,value):
 c=q.strict(f['records']['attempts/x/candidate.json']);c[key]=value
 raw=q.pretty(c);f['records']['attempts/x/candidate.json']=raw
 digest=hashlib.sha256(raw).hexdigest()
 if 'x' in f['registry']['active']:f['registry']['active']['x']['candidate_digest']=digest
 if 'abandoned/x' in f['records']:set_receipt(f,'candidate_digest',digest)
def stage(f,suffix,raw=b'',proof=True):
 name='attempts/x/'+suffix;f['records'][name]=raw
 if proof:
  p=dict(attempt='x',candidate_digest=f['reservation']['candidate_digest'],sha256=hashlib.sha256(raw).hexdigest(),checkpoint={R:R,C:'terminal-control'}[suffix]+('-partial' if suffix==C and raw else '-created'),source_sha256=q.FORMAT_SHA256,censored_schedule_sha256='d'*64)
  f['evidence']['stages']={name:p};f['evidence']['qualified_schedule_sha256']=['d'*64]
def generation(f):
 f['mode']='B'
 body=b'b';manifest={'f.md':dict(source='originals/f',digest=hashlib.sha256(body).hexdigest())}
 f['records']['originals/f']=body
 f['records']['generations/x/f.md']=body;raw=q.pretty(manifest)
 f['records']['generations/x/MANIFEST.json']=raw
 set_candidate(f,'generation_manifest',hashlib.sha256(raw).hexdigest())
 return f
def vectors(f,base):
 snap=snapshot(f);old=snapshot(base)
 return dict(changed={name:base64.b64encode(raw).decode() for name,raw in snap.records.items() if old.records.get(name)!=raw},deleted=sorted(set(old.records)-set(snap.records)),facts=f['overrides'],authority=f['chain'],evidence=f['evidence'])
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--oracle-sha256',required=True);args=parser.parse_args()
 assert sys.dont_write_bytecode and os.environ.get('FILE_NATIVE_STAGE_JOB')=='1'
 assert hashlib.sha256(q.Path(q.__file__).read_bytes()).hexdigest()==args.oracle_sha256
 base=fixture();positive=[];negative=[];rawvectors=[]
 def yes(label,f):
  answer=invoke(f);positive.append(dict(case=label,proof=answer['proof'],vector=vectors(f,base)))
 def no(label,f):
  before=copy.deepcopy(f)
  try:invoke(f)
  except (AssertionError,KeyError,ValueError,TypeError) as error:
   assert before==f;negative.append(dict(case=label,error=type(error).__name__,reason=str(error)));rawvectors.append(dict(case=label,**vectors(f,base)));return
  raise AssertionError('negative:'+label)
 f=fixture();f['registry']={};yes('legacy-V1',f)
 f=fixture('active');yes('preselection',f)
 f=fixture('active');f['registry']['active']['x']['state']='selection-started';yes('selection-started',f)
 yes('published-pending',fixture('pending'))
 yes('committed',fixture())
 f=fixture('active');stage(f,R,q.pretty(f['receipt'])[:20]);yes('receipt-prefix',f)
 f=fixture('pending');replacement=copy.deepcopy(f['registry']);replacement['active']={};replacement['terminal_count']=1;replacement['terminal_bytes']=f['receipt']['charge_bytes'];raw=q.pretty(replacement);stage(f,C,raw[:len(raw)//2]);yes('control-prefix',f)
 f=generation(fixture());yes('B-private',f)
 f=fixture();f['chain'][1][1].update(request='r',intent=f['candidate']['intent']);answer=invoke(f);again=invoke(f);assert answer==again;yes('fresh-winner',f)
 f=fixture('active');f['records']['derived/one']=b'shared';f['records']['derived/two']=b'shared';snap=snapshot(f);identity=snap.files['derived/one']['identity'];f['overrides']={'derived/one':dict(links=2),'derived/two':dict(identity=identity,links=2)};yes('mock-alias-census',f)
 for key,value in [('version',3),('store_id','foreign'),('unknown',1),('terminal_count',2),('terminal_bytes',q.MARGIN),('terminal_count',4097),('terminal_count',True),('terminal_bytes',2**64),('legacy',['missing']),('legacy',['x','x'])]:
  f=fixture();f['registry'][key]=value;no('registry-'+key+'-'+str(value)[:20],f)
 f=fixture('active')
 for n in range(9):f['registry']['active']['owner-'+str(n)]=copy.deepcopy(f['reservation'])
 no('ninth',f)
 for key,value in [('state','unknown'),('used',2**64),('used',67108865),('bytes',True),('candidate_digest','bad')]:
  f=fixture('active');f['registry']['active']['x'][key]=value;no('active-'+key+'-'+str(value),f)
 f=fixture('active');f['registry']['legacy']=['x'];no('legacy',f)
 for key,value in [('version',2),('store_id','foreign'),('attempt','foreign'),('request','different'),('intent','b'*64),('used_bytes',0),('original_allowance',1),('charge_bytes',1),('handoff_margin',0),('control_stage','outside'),('reason','other'),('unknown',1)]:
  f=fixture();set_receipt(f,key,value);no('receipt-'+key,f)
 for key,value in [('purpose','packet'),('store_id','foreign'),('attempt','foreign'),('intent','f'*64),('packet_ref',{}),('checkpoint',{}),('expected',None),('request',dict(request='z',base=None,ops=[],packet=False))]:
  f=fixture();set_candidate(f,key,value);no('candidate-'+key,f)
 f=fixture();f['chain'][1][1]['attempt']='x';no('selected-terminal',f)
 f=fixture('pending');f['registry']['active']['x']['state']='selection-started';no('grant-terminal',f)
 f=fixture('pending');f['registry']['terminal_count']=1;f['registry']['terminal_bytes']=f['receipt']['charge_bytes'];no('pending-doublecount',f)
 f=fixture();del f['records']['attempts/x/candidate.json'];no('missing-candidate',f)
 f=fixture();f['records']['attempts/x/allocations/unselected.json']=b'{}';no('allocation',f)
 f=fixture();f['records']['attempts/foreign/private']=b'x';no('unknown-owner',f)
 f=fixture();f['records']['attempts/x/private']=b'x'*(f['receipt']['charge_bytes']+1);no('undercharged',f)
 f=fixture();f['overrides']={'abandoned/x':dict(links=2)};no('receipt-hardlink',f)
 f=fixture();f['records']['control-alias']=f['records']['abandoned/x'];identity=snapshot(f).files['abandoned/x']['identity'];f['overrides']={'control-alias':dict(identity=identity)};no('ID-alias',f)
 f=fixture('active');stage(f,R,proof=False);no('no-schedule',f)
 f=fixture('active');stage(f,R,b'!');no('stage-prefix',f)
 f=fixture('active');stage(f,R);f['evidence']['qualified_schedule_sha256']=[];no('unqualified-stage',f)
 f=fixture('active');stage(f,R);name=next(iter(f['evidence']['stages']));f['evidence']['stages'][name]['source_sha256']='b'*64;no('stage-source',f)
 f=fixture('pending');stage(f,C,b'!');no('bad-replacement',f)
 f=fixture();stage(f,C);no('stage-after',f)
 f=fixture('active');stage(f,R);f['registry']['active']['x']['state']='selection-started';no('stage-owner-phase',f)
 f=generation(fixture());del f['records']['generations/x/f.md'];no('B-missing',f)
 f=generation(fixture());f['records']['generations/x/MANIFEST.json']=b'{}';no('B-hash',f)
 f=generation(fixture());f['records']['generations/x/extra']=b'x';no('B-extra',f)
 f=fixture('pending');f['registry']['active']['x']['bytes']=q.LIMIT-q.MARGIN;set_receipt(f,'original_allowance',q.LIMIT-q.MARGIN);yes('max-allowance',f)
 f=fixture();set_receipt(f,'original_allowance',33*1024**3);no('33GiB',f)
 f=fixture('pending');stage(f,C);name=next(iter(f['evidence']['stages']));f['evidence']['stages'][name]['checkpoint']='terminal-control-stage-created';no('bad-control-label',f)
 f=fixture();f['records']['abandoned/x']+=b'\n';no('receipt-newline',f)
 assert len(positive)==11 and len(negative)==59
 print(json.dumps(dict(status='passed',oracle_sha256=args.oracle_sha256,positives=positive,negative_cases=negative,base_records={n:base64.b64encode(b).decode() for n,b in snapshot(base).records.items()},negative_vectors=rawvectors,no_engine=True,no_writes=True,native_qualified=False)))
if __name__=='__main__':main()
