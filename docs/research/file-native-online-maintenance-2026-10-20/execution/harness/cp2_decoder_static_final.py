"""Synthetic schema/lineage/hole negatives; no engine or real-store reads."""
import copy
import hashlib
import json
from pathlib import Path
from job_guard import write_json, sha_file
from matrix_budget import check_matrix
from physical_decoder_v4 import Decoder, descriptor4, discovery4, SCHEMA_PIN

check_matrix({'receipts':dict(bytes=1024**2,files=10),'protocol':dict(bytes=1024**2,files=30)})
payloads = {}
chain = {}
prior_selected = None
for n in range(2):
    result = json.dumps(dict(outcome='accepted'),indent=2).encode()
    result_hash = hashlib.sha256(result).hexdigest()
    p = dict(id=f'publication-{n}',sequence=n,store_id='store-static',attempt=f'attempt-{n}',
             request=f'request-{n}',intent='b'*64,result_hash=result_hash,predecessor=prior_selected,
             allocation_owner=dict(attempt=f'attempt-{n}',includes_intermediates_and_stages=True))
    pub = json.dumps(p,indent=2).encode()
    selected = dict(publication=p['id'],digest=hashlib.sha256(pub).hexdigest())
    candidate = dict(version=2,purpose='publication',store_id=p['store_id'],attempt=p['attempt'],
                     request=dict(request=p['request']),intent=p['intent'],expected=p['predecessor'])
    payloads.update({f"publications/{p['id']}.json":pub,f"results/{result_hash}.json":result,
        f"attempt-hints/{p['attempt']}.json":json.dumps(selected).encode(),
        f"attempts/{p['attempt']}/candidate.json":json.dumps(candidate).encode(),
        f"attempts/{p['attempt']}/allocations/{p['id']}.json":json.dumps(dict(publication=p['id'],result=result_hash)).encode()})
    chain[n] = (selected,p)
    prior_selected = selected

class Synthetic(Decoder):
    def __init__(self):
        self.entries = dict.fromkeys(payloads)
        self.terminal = set()
    def accepted_chain(self):return chain
    def raw(self,key):return payloads[key]
    def terminal_exclusion(self,p,selected,role_number,key):
        return dict(reason='synthetic-terminal-retirement',key=key) if key in self.terminal else None

d = Synthetic()
complete = dict(version=1,completed=dict(selected=chain[1][0],sequence=1),interval=None,deferred=[],include_attempts=True)
assert d.discovery_proof(complete)['checked_processed_publications'] == 2
partial = dict(version=1,completed=None,interval=dict(target=complete['completed'],next=chain[0][0],expected_sequence=0),deferred=[],include_attempts=True)
assert d.discovery_proof(partial)['checked_processed_publications'] == 1
rows = []
def refuses(name,call):
    try:call()
    except (AssertionError,ValueError,KeyError) as error:rows.append(dict(case=name,refused=True,reason=str(error)))
    else:raise AssertionError('negative-was-accepted:'+name)

bad = copy.deepcopy(complete);bad['completed']['sequence'] = True
refuses('bool-sequence',lambda:discovery4(bad))
bad = copy.deepcopy(complete);bad['completed']['selected'] = chain[0][0]
refuses('forged-completed-selector',lambda:d.discovery_proof(bad))
bad = copy.deepcopy(partial);bad['interval']['next'] = chain[1][0]
refuses('forged-next-sequence',lambda:d.discovery_proof(bad))
bad = copy.deepcopy(partial);bad['deferred'] = [dict(publication=dict(selected=chain[0][0],sequence=0),roles=8,reason='reservation')]
refuses('hole-outside-processed-interval',lambda:d.discovery_proof(bad))
bad = copy.deepcopy(complete);item=dict(publication=complete['completed'],roles=8,reason='reservation');bad['deferred']=[item,item]
refuses('duplicate-deferred-reference',lambda:discovery4(bad))
bad = copy.deepcopy(complete);bad['include_attempts']=False;bad['deferred']=[item]
refuses('attempt-hole-when-attempts-excluded',lambda:discovery4(bad))
key = 'publications/publication-1.json';d.entries.pop(key)
refuses('missing-permanent-mapping',lambda:d.discovery_proof(complete));d.entries[key]=None
key = 'attempts/attempt-0/candidate.json';d.entries.pop(key)
refuses('missing-candidate-without-hole-or-terminal-policy',lambda:d.discovery_proof(complete))
d.terminal.add(key)
assert len(d.discovery_proof(complete)['terminal_exclusions']) == 1
example = dict(pack='a'*64,index='b'*64,pack_bytes=1,index_bytes=53,records=1,codec=0,
               role='publication',first='publications/p.json',last='publications/p.json',segment=True)
refuses('bool-segment',lambda:descriptor4(example))
# Caller provenance selects the publication's exact pair, never last write by ID.
from physical_decoder_v4 import resolve_provenance
from outcome_oracle import intent
first=dict(request='collision',base=None,ops=[],packet=False)
last=dict(request='collision',base=None,ops=[],packet=True)
provenance=[dict(request=first,intent=intent(first)),dict(request=last,intent=intent(last))]
selected_input={0:(chain[0][0],dict(request='collision',intent=intent(first)))}
assert resolve_provenance(selected_input,provenance)['collision']==first
omitted=dict(request='equivalent',base=None,ops=[])
explicit={**omitted,'packet':True}
assert intent(omitted)==intent(explicit)
resolve_provenance({0:(chain[0][0],dict(request='equivalent',intent=intent(omitted)))},
                   [dict(request=omitted,intent=intent(omitted)),dict(request=explicit,intent=intent(explicit))])

# Bounded synthetic index bytes exercise overlapping segments and duplicate data.
import struct
root=Path('C:/file-native-online-matrix-20261020/protocol/cp2-static-indices')
assert not root.exists()
key='publications/synthetic.json'
def index_fixture(instance,segment,data):
    pack_hash=hashlib.sha256(data).hexdigest()
    index=(b'FNPIDX01'+bytes.fromhex(pack_hash)+struct.pack('<QBI',len(data),0,1)
           +struct.pack('<H',len(key))+key.encode()+struct.pack('<QII',0,len(data),len(data))
           +hashlib.sha256(data).digest())
    index_hash=hashlib.sha256(index).hexdigest()
    folder=root/'packs'/instance;folder.mkdir(parents=True)
    (folder/(pack_hash+'.pack')).write_bytes(data)
    (folder/(index_hash+'.idx')).write_bytes(index)
    return dict(instance=instance,segment=segment,pack=pack_hash,index=index_hash,
                pack_bytes=len(data),index_bytes=len(index),records=1,codec=0,
                role='publication',first=key,last=key)
class ByteControl(Decoder):
    def __init__(self):
        self.root=root;self.config=dict(version=3,store_id='static-store')
        self.required=set();self.entries={};self.descriptors=[];self.zstd=None
    def read_metadata(self,*unused):return self.cat
reader=ByteControl()
descriptors=[index_fixture('physical-static-'+str(n),n,b'x') for n in range(1,6)]
reader.cat=dict(version=3,store_id='static-store',packs=descriptors[:4]);reader.catalog('unused')
reader.cat=dict(version=3,store_id='static-store',packs=descriptors)
refuses('fifth-segment',lambda:reader.catalog('unused'))
reader.cat=dict(version=3,store_id='static-store',packs=[descriptors[0],{**descriptors[1],'segment':1}])
refuses('overlap-inside-one-segment',lambda:reader.catalog('unused'))
reader.cat=dict(version=3,store_id='static-store',packs=[descriptors[0],{**descriptors[0],'segment':2}])
refuses('same-index-instance-reused',lambda:reader.catalog('unused'))
reader.load_index(descriptors[0]);reader.load_index(descriptors[1])
assert reader.raw(key)==b'x' and len(reader.variants[key])==2
loose=root/key;loose.parent.mkdir();loose.write_bytes(b'x')
second=root/'packs'/descriptors[1]['instance']/(descriptors[1]['pack']+'.pack')
second.write_bytes(b'y')
refuses('corrupt-second-mapped-variant-despite-good-first-and-loose',lambda:reader.raw(key))
second.write_bytes(b'x')
conflict=index_fixture('physical-static-conflict',6,b'y')
refuses('conflicting-cross-segment-identity',lambda:reader.load_index(conflict))
assert reader.raw(key)==b'x'

out = Path('C:/file-native-online-matrix-20261020/receipts/cp2-decoder-static-final.json')
write_json(out,dict(status='passed',schema_pin=SCHEMA_PIN,negative_cases=rows,
           completed_and_partial_lineage_controls=True,terminal_exclusion_control=True,
           decoder_sha256=sha_file(Path(__file__).with_name('physical_decoder_v4.py')),
           no_engine=True,no_real_store_reads=True,request_intent_pair_controls=True,duplicate_variant_controls=True,qualification='synthetic parser controls only; full independent quiescent fixtures remain pending'))
print(json.dumps(dict(status='passed',negative_cases=len(rows),synthetic_only=True)))
