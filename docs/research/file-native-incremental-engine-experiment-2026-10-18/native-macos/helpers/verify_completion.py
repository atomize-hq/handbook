"""Check actual reached native results independently of the final marker."""
import argparse
import json
from pathlib import Path

p=argparse.ArgumentParser();p.add_argument('--runtime',type=Path,required=True);p.add_argument('--pin',required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
root=a.runtime
def load(path):return json.loads(path.read_bytes())
def events(name):return [json.loads(line) for line in (root/name/'events.jsonl').read_bytes().splitlines()]

complete=load(root/'completed.json');assert complete['pin']==a.pin
assert complete['affected_checks'] is True and complete['smoke_n']==1000 and complete['consumers']==2 and complete['shell_copies']==3
checks=load(root/'checks/summary.json')
named=[r for r in checks if 'consumer' in r]
faults=[r for r in checks if 'boundary' in r]
assert len(named)==4 and {(r['mode'],r['consumer']) for r in named}=={(m,c) for m in ['A','B'] for c in ['project','research']}
assert all(r['status']=='passed' for r in named)
assert len(faults)==23 and len({r['boundary'] for r in faults})==23
for row in faults:
    assert row['original_exit']==86
    if row['before_selection']:assert row['recovered_outcome']=='absent-as-of'
    if row['completed_boundary']:assert row['recovered_outcome']=='accepted'
supplement=load(root/'supplement/summary.json')
assert supplement['review_findings']=='regressions passed'
assert supplement['actual_retired_source_reclaimed'] is True and supplement['reader_overlap'] is True
assert supplement['stable_divergence_backups']==1 and supplement['quota_slots']==8 and supplement['fallback'] is True
adversarial=load(root/'adversarial/summary.json')
assert adversarial['large_header_indirection'] is True and len(adversarial['passed'])==9
matrix=load(root/'smoke/matrix.json')
assert len([r for r in matrix if r.get('stage')=='validation'])==3
assert len([r for r in matrix if r.get('stage')=='publication-and-A'])==3
assert len([r for r in matrix if r.get('stage')=='complete-B'])==1
assert len([r for r in matrix if r.get('stage')=='complete-B-successor'])==1
smoke=events('smoke')
initial=[r for r in smoke if r['input']['action']=='init']
updates=[r for r in smoke if r['input']['action']=='publish']
assert len(initial)==2 and {r['input']['mode'] for r in initial}=={'A','B'}
assert all(r['input']['count']==1000 for r in initial)
assert {r['input']['request']['request'] for r in updates}=={'update-1','update-8','update-64','B-one'}
for row in initial+updates:
    assert row['exit']==0 and row['censored'] is False and row['output']['status']=='ok'
    delivered=row['output']['result']
    assert delivered['status']=='published' and delivered['result']['outcome']=='accepted'
refresh=[r for r in smoke if r['input']['action']=='refresh']
assert len(refresh)==3 and all(r['exit']==0 and not r['censored'] and r['output']['status']=='ok' for r in refresh)
for consumer in ['project','research']:
    rows=events('consumer-'+consumer)
    assert [r['input']['action'] for r in rows]==['init','publish','resolve','audit']
    assert all(r['exit']==0 and not r['censored'] and r['output']['status']=='ok' for r in rows)
for name in ['consumer-project','consumer-research','B-copy']:
    assert 'verified ' in (root/name/'walk.stdout').read_text()
all_events=[r for f in root.rglob('events.jsonl') for r in [json.loads(line) for line in f.read_bytes().splitlines()]]
assert not any(r['censored'] for r in all_events)
result=dict(status='passed',pin=a.pin,named_cases=len(named),publication_fault_boundaries=len(faults),
    must_not_select=sum(r['before_selection'] for r in faults),completed_boundary_must_retain=sum(r['completed_boundary'] for r in faults),
    visible_uncertain=sum(not r['before_selection'] and not r['completed_boundary'] for r in faults),
    supplement=supplement,supplement_injected_process_exits=sum('EXP_EXIT' in r['environment'] for r in events('supplement')),
    adversarial_cases=adversarial['passed'],
    smoke_initializations=len(initial),smoke_updates=len(updates),smoke_refreshes=len(refresh),
    independent_consumer_cases=2,ordinary_copies=3,native_events=len(all_events),native_censors=0,
    qualification='Named/group counts are not assertion counts or independent statistical trials. No large growth/history measurements are allocated.')
a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
