"""Bounded A packet-output preservation and retained-corruption boundary checks."""
import pathlib
import sys
import native_runner as n

backend=sys.argv[1]
base=n.RUNTIME/'data/v1'/backend/'supplement'
base.mkdir(parents=True,exist_ok=False)
d=n.NativeDriver(base,n.EXE,'supplement',backend)
rows=[]
root=base/'edited-packet'
d.init(root,'A','research',16)
selected=n.b.selected_state(root)[0]
edits={'packet/context.md':b'UNADMITTED PACKET OUTPUT\r\n',
       'packet/provenance.json':b'{"unadmitted":"caf\xc3\xa9"}\r\n'}
for name,value in edits.items():(root/'view'/name).write_bytes(value)
(root/'view/NAVIGATION.md').unlink()
audit=d.call(n.e.cmd(root,'view-audit'))['result']
assert set(audit['edited'])==set(edits) and audit['missing']==['NAVIGATION.md']
d.call(n.e.cmd(root,'resume'),env={'EXP_EXIT':'view-divergence-preserved'},allowed=(86,))
d.call(n.e.cmd(root,'resume'))
preserved=[p.read_bytes() for p in (root/'divergence').rglob('*') if p.is_file()]
assert all(value in preserved for value in edits.values())
assert n.b.selected_state(root)[0]==selected
audit=d.call(n.e.cmd(root,'view-audit'))['result']
assert not audit['missing'] and not audit['edited']
rows.append(dict(case='edited-packet-outputs',backend=backend,selection_unchanged=True,
    interruption_after_preservation=True,missing_navigation_rebuilt=True,
    preserved_sha256={k:n.b.sha(v) for k,v in edits.items()}))

root=base/'corrupt-nonretired-prior'
d.init(root,'A','research',16)
old,oldpub,_=n.b.selected_state(root)
d.publish(root,'A',n.b.request('accepted-before-view',[n.b.edit()]),
          env={'EXP_EXIT':'selection-boundary'},allowed=(86,))
current=n.b.selected_state(root)[0]
assert current!=old
assert n.b.load(root/'VIEW-STATUS.json')['target_publication']==old['publication']
prior=root/oldpub['candidate']['state']
saved=prior.read_bytes();(base/'prior-state-original.bin').write_bytes(saved)
prior.write_bytes(saved[:-1]+b'!')
view_before={str(p.relative_to(root/'view')):n.b.sha(p.read_bytes()) for p in (root/'view').rglob('*') if p.is_file()}
result=d.call(n.e.cmd(root,'resolve',request_id='accepted-before-view'))['result']
assert result['outcome']=='accepted'
resume=d.call(n.e.cmd(root,'resume'),allowed=(2,))
assert 'integrity-digest' in resume['error']
assert n.b.load(root/'SELECTED.json')==current
assert prior.read_bytes()==saved[:-1]+b'!'
assert view_before=={str(p.relative_to(root/'view')):n.b.sha(p.read_bytes()) for p in (root/'view').rglob('*') if p.is_file()}
assert d.call(n.e.cmd(root,'resolve',request_id='accepted-before-view'))['result']==result
rows.append(dict(case='corrupt-nonretired-prior',backend=backend,old=old,current=current,
    error=resume['error'],authority_not_repaired=True,view_unchanged=True,original_outcome=result))
n.b.write(base/'supplement-observations.json',dict(helper_sha256=n.b.sha(pathlib.Path(__file__).read_bytes()),cases=rows,status='passed'))
