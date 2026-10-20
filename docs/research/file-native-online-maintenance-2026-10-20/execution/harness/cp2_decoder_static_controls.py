"""Synthetic schema/lineage/hole negatives; no engine or real-store reads."""
import copy
import hashlib
import json
from pathlib import Path
from job_guard import write_json, sha_file
from matrix_budget import check_matrix
from physical_decoder_v4 import Decoder, descriptor4, discovery4, SCHEMA_PIN

check_matrix({'receipts':dict(bytes=1024**2,files=10)})
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
out = Path('C:/file-native-online-matrix-20261020/receipts/cp2-decoder-static-controls.json')
write_json(out,dict(status='passed',schema_pin=SCHEMA_PIN,negative_cases=rows,
           completed_and_partial_lineage_controls=True,terminal_exclusion_control=True,
           decoder_sha256=sha_file(Path(__file__).with_name('physical_decoder_v4.py')),
           no_engine=True,no_real_store_reads=True,qualification='synthetic parser controls only; full independent quiescent fixtures remain pending'))
print(json.dumps(dict(status='passed',negative_cases=len(rows),synthetic_only=True)))
