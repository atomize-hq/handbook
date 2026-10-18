"""Complete retained-read/assembly evidence after a bounded growth interruption.

Never resumes initialization, selects a candidate, or changes publication history.
Run only after the measured scheduler has exited. Successful existing samples are
not repeated. Events append to growth raw evidence with this invocation's session.
"""
import argparse,json,pathlib
from experiment import Driver,cmd
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--exe',type=pathlib.Path,required=True);a=p.parse_args()
assert (a.base/'execution-completed.json').exists(),'wait for the measured scheduler to exit'
helper_sha256=b.sha(pathlib.Path(__file__).read_bytes())
rows=[]
for backend in ['manifest','cow']:
    base=a.base/backend;raw=base/'growth-raw.jsonl'
    if not raw.exists():continue
    prior=[json.loads(s) for s in raw.read_bytes().splitlines() if s]
    d=None
    for root in sorted((base/'growth').glob('*-project')):
        if not (root/'SELECTED.json').exists():
            rows.append(dict(root=str(root),status='unselected initialization; no retained probe'))
            continue
        existing=[r for r in prior if r['command']['root']==str(root) and not r['censored'] and (r.get('output') or {}).get('status')=='ok']
        # The normal workload writes all twelve lookup samples last.
        if len([r for r in existing if r['command']['action']=='point'])==12 and any(r['command']['action']=='packet' for r in existing):continue
        if d is None:d=Driver(base,a.exe,'growth',backend)
        info=d.inspect(root);selected=info['selected'];initial=info['history'][-1]['publication']
        data=(root/'publications'/(initial+'.json')).read_bytes();old=dict(publication=initial,digest=b.sha(data))
        if not any(r['command']['action']=='packet' for r in existing):
            packet=d.call(cmd(root,'packet',selected=selected),timeout=600);b.check_packet(root/packet['result']['path'])
        seen=set();samples=0
        for binding in [selected,old]:
            if binding['publication'] in seen:continue
            seen.add(binding['publication'])
            for key in ['d000001','not-present']:
                count=sum(r['command']['action']=='point' and r['command'].get('selected',{}).get('publication')==binding['publication'] and r['command'].get('id')==key for r in existing)
                assert count<=3
                for _ in range(3-count):
                    point=d.call(cmd(root,'point',selected=binding,id=key));assert (point['result']['value'] is not None)==(key=='d000001');samples+=1
        rows.append(dict(root=str(root),backend=backend,session=d.session,helper_sha256=helper_sha256,status='retained assembly/lookup probes completed',selected_publications=len(info['history']),bindings=len(seen),new_lookup_samples=samples,qualification='separate post-cap probes; original censored publication remains censored; no candidate promoted'))
b.write(a.base/'retained-probe-observations.json',rows)
print(json.dumps(rows,indent=2))
