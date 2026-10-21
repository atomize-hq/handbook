"""R1 memory-only accepted-role controls. No engine or protocol/store writes."""
import argparse
import base64
import copy
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
from types import SimpleNamespace
import sys
import physical_decoder_v5 as physical
from outcome_oracle import intent
from job_guard import sha_file

parser=argparse.ArgumentParser()
parser.add_argument('--decoder-sha256',required=True)
args=parser.parse_args()
assert os.environ.get('FILE_NATIVE_STAGE_JOB')=='1' and sys.dont_write_bytecode
assert sha_file(physical.__file__)==args.decoder_sha256
assert sha_file(physical.paged_index.__file__)=='788286c46af4c53b045e0c029ae7ef22f83ecd92e31de18147c7e23fc44660ea'


def encoded(value):return json.dumps(value,indent=2,ensure_ascii=False).encode()
def digest(data):return hashlib.sha256(data).hexdigest()


class MemoryPath:
    files={}
    def __init__(self,value):self.value=PurePosixPath(str(value))
    def __str__(self):return self.value.as_posix()
    def __fspath__(self):return str(self)
    def __truediv__(self,value):return MemoryPath(self.value/str(value))
    def __eq__(self,other):return isinstance(other,MemoryPath) and self.value==other.value
    def __hash__(self):return hash(self.value)
    @property
    def parent(self):return MemoryPath(self.value.parent)
    def resolve(self):return self
    def relative_to(self,root):return self.value.relative_to(str(root))
    def read_bytes(self):return self.files[str(self)]
    def exists(self):return str(self) in self.files or any(k.startswith(str(self)+'/') for k in self.files)
    def is_file(self):return str(self) in self.files
    def is_dir(self):return self.exists() and not self.is_file()
    def is_symlink(self):return False
    def stat(self):return SimpleNamespace(st_size=len(self.read_bytes()))
    def open(self,mode):
        assert mode=='rb','memory-files-are-read-only'
        return io.BytesIO(self.read_bytes())
    def iterdir(self):
        prefix=str(self)+'/'
        return iter(MemoryPath(prefix+name) for name in sorted({k[len(prefix):].split('/')[0] for k in self.files if k.startswith(prefix)}))


ROOT=MemoryPath('/r1-memory-store')
def put(name,raw):MemoryPath.files[str(ROOT/name)]=raw
def bounded(path,limit,exact=None):
    raw=MemoryPath(path).read_bytes()
    assert len(raw)<=limit and (exact is None or len(raw)==exact)
    return raw


def semantic_records():
    records={};selected=None;publications=[]
    graph=dict(version=1,consumer='project',budget=1000,cost=0,count=1,tree=None,fallback=False)
    for n in range(2):
        name,attempt='publication-'+str(n),'attempt-'+str(n)
        request=dict(request='initial' if n==0 else 'second',base=selected,ops=[],packet=False)
        result=dict(outcome='accepted',request=request['request'],publication=name,
            state_digest=digest(encoded(graph)),changed=[],removed=[])
        result_raw=encoded(result);result_hash=digest(result_raw)
        p=dict(version=1,store_id='store-memory',id=name,sequence=n,predecessor=selected,
            graph=graph,projection=None,changes=None,membership=None,outcomes=None,
            request=request['request'],intent=intent(request),result_hash=result_hash,attempt=attempt,
            generation_manifest=None,allocation_owner=dict(attempt=attempt,conservative_bytes=67108864,
            includes_intermediates_and_stages=True),packet_ref=None)
        candidate=dict(purpose='publication',version=2,store_id=p['store_id'],attempt=attempt,
            expected=selected,request=request,intent=p['intent'],graph=graph,projection=None,changes=None,
            changed=[],removed=[],generation_manifest=None,checkpoint=None,packet_ref=None,replay_objects=[],replay_files={})
        publication_raw=encoded(p);selected=dict(publication=name,digest=digest(publication_raw))
        records.update({f'publications/{name}.json':publication_raw,f'results/{result_hash}.json':result_raw,
            f'attempt-hints/{attempt}.json':encoded(selected),f'attempts/{attempt}/candidate.json':encoded(candidate),
            f'attempts/{attempt}/allocations/{name}.json':encoded(dict(publication=name,result=result_hash))})
        publications.append((selected,p))
    return records,publications


BASE,PUBLICATIONS=semantic_records()


def pair(role,records,instance,segment=1,legacy=False):
    import struct
    rows=sorted((k,v) for k,v in records.items() if physical.prior.legacy.role(k)==role)
    data=b''.join(raw for _,raw in rows);pack_hash=digest(data)
    first,last=rows[0][0],rows[-1][0]
    if legacy:index=bytearray(b'FNPIDX01'+bytes.fromhex(pack_hash)+struct.pack('<QBI',len(data),0,len(rows)))
    else:
        index=bytearray(b'FNPIDX02'+bytes([0,physical.paged_index.ROLES.index(role),0,0]))
        index+=struct.pack('<IHQQ',len(rows),len(rows),0,len(data))+bytes.fromhex(pack_hash)+struct.pack('<Q',len(data))
        for key in [first,last]:index+=struct.pack('<H',len(key))+key.encode()
    offset=0
    for key,raw in rows:
        index+=struct.pack('<H',len(key))+key.encode()+struct.pack('<QII',offset,len(raw),len(raw))+bytes.fromhex(digest(raw));offset+=len(raw)
    d=dict(pack=pack_hash,index=digest(index),pack_bytes=len(data),index_bytes=len(index),records=len(rows),
        codec=0,role=role,first=first,last=last,instance=instance,segment=segment)
    if not legacy:d['page']=dict(version=2,hash=digest(index),bytes=len(index),height=0)
    put(f'packs/{instance}/{pack_hash}.pack',data);put(f"packs/{instance}/{d['index']}.idx",bytes(index))
    return d


def generation(records,instance,ordinal,older=None,parent='a'*64,sharing=False):
    if sharing:
        packs=[pair('result',records,instance,1,True),pair('result',records,instance,2,False)]
    else:packs=[pair(role,records,instance) for role in sorted({physical.prior.legacy.role(k) for k in records})]
    cat=dict(version=4,store_id='store-memory',packs=packs);cat_raw=encoded(cat);cat_hash=digest(cat_raw)
    put('pack-catalogs/'+cat_hash+'.json',cat_raw)
    gen=dict(id=instance,ordinal=ordinal,catalog=cat_hash,plan=None,packs=packs)
    work=dict(version=2,store_id='store-memory',generation=gen,older=older or [],jobs=[],retired=[],garbage=[],
        removed_bytes=0,removed_files=0,reconciled_absent=0,physical_parent=parent,transition_parent=None,birth_claims=[],discovery=None)
    work_raw=encoded(work);plan_hash=digest(work_raw);put('physical-plans/'+plan_hash+'.json',work_raw)
    put('physical-pins/'+instance+'.lock',encoded(dict(version=1,store_id='store-memory',id=instance,ordinal=ordinal,catalog=cat_hash)))
    config=dict(version=3,store_id='store-memory',mode='A',consumer='project',fallback=False,navigation_version=4,
        catalog=cat_hash,physical=dict(id=instance,ordinal=ordinal,plan=plan_hash))
    return config,{**gen,'plan':plan_hash}


def fixture(changes=None,pending=None,sharing=False,retired=False):
    MemoryPath.files={}
    for key,raw in BASE.items():put(key,raw)
    records={**BASE,**(changes or {})}
    config,gen=generation(records,'physical-current',1,sharing=sharing)
    current=encoded(config);put('STORE.json',current);put('SELECTED.json',encoded(PUBLICATIONS[-1][0]))
    if retired:put('retired/publication-0.json',encoded(PUBLICATIONS[0][0]))
    target=None
    if pending is not None:
        target,_=generation({**BASE,**pending},'physical-pending',2,older=[gen],parent=digest(current))
        put('PACKING.json',encoded(dict(version=2,id='physical-pending',status='installed',expected_config=digest(current),
            catalog=target['catalog'],old=gen['packs'],reserved_bytes=1000000,used_bytes=50000,deleted_bytes=0,deferred_bytes=0,
            target=target,abandoned=[],ownership_transferred=False)))
    return target


def byte_only(reader):
    for d in reader.descriptors:
        assert digest((reader.root/physical.prior.container_path(d,'pack')).read_bytes())==d['pack']
    for key in reader.entries:reader.raw(key)
    return dict(mapped_keys=len(reader.entries),self_hashed_pack_index_and_all_raw_valid=True)


negative=[];positive=[];vectors=[]
def refuses(name,callback,raw_records=None):
    before=dict(MemoryPath.files)
    try:callback()
    except (AssertionError,ValueError,KeyError,TypeError) as error:
        negative.append(dict(case=name,refused=True,error_type=type(error).__name__,reason=str(error)))
    else:raise AssertionError('negative-was-accepted:'+name)
    assert MemoryPath.files==before,'read-only-memory-check-mutated-files'
    if raw_records:vectors.append(dict(case=name,records={k:base64.b64encode(v).decode() for k,v in raw_records.items()}))


saved=(physical.Path,physical.prior.Path,physical.prior.legacy.Path,physical.prior.legacy.bounded)
physical.Path=physical.prior.Path=physical.prior.legacy.Path=MemoryPath
physical.prior.legacy.bounded=bounded
try:
    fixture();d=physical.Decoder(ROOT,quiescent=True);assert len(d.verify_packs())==10
    assert d.mapped_cohort_proof['accepted_publications']==2
    positive.append('authenticated-accepted-five-role-cohort')
    phantom=copy.deepcopy(PUBLICATIONS[-1][1]);phantom.update(id='unselected-publication',attempt='unselected-attempt',request='unselected-request',sequence=2,predecessor=PUBLICATIONS[-1][0])
    phantom_result=dict(outcome='accepted',request=phantom['request'],publication=phantom['id'],state_digest=digest(encoded(phantom['graph'])),changed=[],removed=[])
    phantom['result_hash']=digest(encoded(phantom_result));phantom['allocation_owner']['attempt']=phantom['attempt']
    request=dict(request=phantom['request'],base=phantom['predecessor'],ops=[],packet=False);phantom['intent']=intent(request)
    c=copy.deepcopy(json.loads(BASE['attempts/attempt-1/candidate.json']));c.update(attempt=phantom['attempt'],request=request,intent=phantom['intent'],expected=phantom['predecessor'])
    ph_sel=dict(publication=phantom['id'],digest=digest(encoded(phantom)))
    extras={f"publications/{phantom['id']}.json":encoded(phantom),f"results/{phantom['result_hash']}.json":encoded(phantom_result),
        f"attempt-hints/{phantom['attempt']}.json":encoded(ph_sel),f"attempts/{phantom['attempt']}/candidate.json":encoded(c),
        f"attempts/{phantom['attempt']}/allocations/{phantom['id']}.json":encoded(dict(publication=phantom['id'],result=phantom['result_hash']))}
    for key,raw in extras.items():
        change={key:raw};fixture(change);d=physical.Decoder(ROOT,quiescent=True);byte_only(d)
        refuses('self-hashed-extra-'+physical.prior.legacy.role(key),d.verify_packs,change)
    wrong={
        'publication':{'publications/publication-0.json':BASE['publications/publication-1.json']},
        'result':{f"results/{PUBLICATIONS[0][1]['result_hash']}.json":BASE[f"results/{PUBLICATIONS[1][1]['result_hash']}.json"]},
        'hint':{'attempt-hints/attempt-0.json':BASE['attempt-hints/attempt-1.json']},
        'candidate':{'attempts/attempt-0/candidate.json':BASE['attempts/attempt-1/candidate.json']},
        'allocation':{'attempts/attempt-0/allocations/publication-1.json':encoded(dict(publication='publication-1',result=PUBLICATIONS[1][1]['result_hash']))}}
    for role,change in wrong.items():
        fixture(change);d=physical.Decoder(ROOT,quiescent=True);byte_only(d)
        refuses('wrong-accepted-'+role+'-binding',d.verify_packs,change)
    wrong_allocation={'attempts/attempt-0/allocations/publication-0.json':encoded(dict(publication='publication-0',result=PUBLICATIONS[1][1]['result_hash']))}
    fixture(wrong_allocation);d=physical.Decoder(ROOT,quiescent=True);byte_only(d)
    refuses('canonical-allocation-wrong-result',d.verify_packs,wrong_allocation)
    bad_request=copy.deepcopy(json.loads(BASE['attempts/attempt-0/candidate.json']))
    bad_request['request']['packet']=True
    changed={'attempts/attempt-0/candidate.json':encoded(bad_request)}
    fixture(changed);d=physical.Decoder(ROOT,quiescent=True);byte_only(d)
    refuses('accepted-candidate-wrong-normalized-intent',d.verify_packs,changed)
    retry={'attempts/attempt-0/allocations/unselected-retry.json':encoded(dict(publication='unselected-retry',result='f'*64))}
    fixture(retry,retired=True);d=physical.Decoder(ROOT,quiescent=True);d.verify_packs()
    assert d.mapped_cohort_proof['legacy_retry_allocations']==1 and d.mapped_cohort_proof['accepted_publications']==2
    positive.append('retired-accepted-metadata-and-legacy-accepted-owner-retry-allocation')
    malformed={'attempts/attempt-0/allocations/unselected-retry.json':encoded(dict(publication='different-retry',result='f'*64))}
    fixture(malformed);d=physical.Decoder(ROOT,quiescent=True);byte_only(d)
    refuses('legacy-retry-path-body-mismatch',d.verify_packs,malformed)
    malformed={'attempts/attempt-0/allocations/unselected-retry.json':encoded(dict(publication='unselected-retry',result='f'*64,unknown=True))}
    fixture(malformed);d=physical.Decoder(ROOT,quiescent=True);byte_only(d)
    refuses('legacy-retry-unknown-field',d.verify_packs,malformed)
    fixture(sharing=True);d=physical.Decoder(ROOT,quiescent=True);rows=d.verify_packs()
    assert len(rows)==2 and sum(len(v) for v in d.variants.values())==4 and d.mapped_cohort_proof['accepted_publications']==2
    assert len({physical.prior.container_path(x,'pack') for x in d.descriptors})==1
    assert len({physical.prior.container_path(x,'idx') for x in d.descriptors})==2
    positive.append('same-birth-same-pack-distinct-legacy-and-paged-indexes')
    target=fixture(pending={});before=dict(MemoryPath.files);d=physical.Decoder(ROOT,quiescent=True);d.verify_packs()
    assert d.physical_metadata['journal']['complete_unselected_candidate_records']==10
    assert not d.physical_metadata['journal']['selected_by_STORE'] and MemoryPath.files==before
    positive.append('genuine-owned-installed-pending-context-captured-selected-authority')
    refuses('candidate-missing-captured-authority',lambda:physical.Decoder.candidate(ROOT,target))
    candidate=physical.Decoder.candidate(ROOT,target,authority=d);byte_only(candidate);candidate.authority=candidate
    refuses('self-authorizing-candidate-context',candidate.verify_packs)
    bad={f"attempts/{phantom['attempt']}/candidate.json":encoded(c)}
    target=fixture(pending=bad);journal=MemoryPath.files.pop(str(ROOT/'PACKING.json'))
    authority=physical.Decoder(ROOT,quiescent=True)
    candidate=physical.Decoder.candidate(ROOT,target,authority=authority);byte_only(candidate)
    refuses('pending-output-unselected-attempt',candidate.verify_packs,bad)
    put('PACKING.json',journal)
    refuses('journal-owned-output-is-not-accepted-authority',authority.journal,bad)
finally:
    physical.Path,physical.prior.Path,physical.prior.legacy.Path,physical.prior.legacy.bounded=saved
assert sha_file(physical.__file__)==args.decoder_sha256
print(json.dumps(dict(status='passed',decoder_sha256=args.decoder_sha256,byte_core_sha256='788286c46af4c53b045e0c029ae7ef22f83ecd92e31de18147c7e23fc44660ea',
    positives=positive,negative_cases=negative,base_records={k:base64.b64encode(v).decode() for k,v in BASE.items()},negative_raw_vectors=vectors,
    no_engine=True,no_protocol_or_store_filesystem_writes=True,virtual_files=len(MemoryPath.files),
    qualification='pure memory mapped-role/authority/metadata controls; no complete graph/source/navigation or real-store/engine qualification')))
