"""Read-only namespace/retention accounting. No authority inference from mtime."""
import pathlib,json
import baseline_harness as b
from storage_account import detailed_inventory
def immutable_paths(root,candidates):
    paths=set();seen=set()
    def checked_object(identity):
        assert len(identity)==64 and all(c in '0123456789abcdef' for c in identity)
        path=f'objects/{identity[:2]}/{identity}';data=(root/path).read_bytes();assert b.sha(data)==identity;paths.add(path);return data
    def page(identity):
        if identity in seen:return
        seen.add(identity);node=json.loads(checked_object(identity))
        if node['kind']=='branch':
            for child in node['children']:page(child['id'])
        else:
            assert node['kind']=='leaf'
            for _,value in node['pairs']:
                data=bytes(value)
                if data:
                    if data.startswith(b'@'):data=checked_object(data[1:].decode())
                    paths.add(json.loads(data)['original'])
    for candidate in candidates:
        data=(root/candidate['state']).read_bytes();assert b.sha(data)==candidate['state_digest'];state=json.loads(data)
        if state.get('format')=='cow-documents-v1':
            if state['root']:page(state['root'])
        else:
            for doc in state['docs'].values():paths.add(doc['original'])
    return paths
def retention_account(root):
    root=pathlib.Path(root);selected=[];next=b.load(root/'SELECTED.json')
    while next:
        data=(root/'publications'/(next['publication']+'.json')).read_bytes();assert b.sha(data)==next['digest'];p=json.loads(data);selected.append(p);next=p['predecessor']
    attempts={p['candidate']['attempt'] for p in selected}
    rows=[];retained=[];pending=[]
    for path in sorted((root/'attempts').glob('*/candidate.json')):
        c=b.load(path);inv=b.inventory(path.parent);closure=root/c['closure']
        if not closure.is_relative_to(path.parent):
            other=b.inventory(closure)
            for key in ['files','directories','logical_bytes','allocated_file_bytes']:inv[key]+=other[key]
        # No active workers are retained at accounting time. A stale expected
        # publication is a known conflict, not a promise of automatic rebase.
        status='selected-retained' if c['attempt'] in attempts else 'unselected-stale' if c['expected']!=b.load(root/'SELECTED.json') else 'unselected-eligible'
        if status=='selected-retained':retained.append(c)
        elif status=='unselected-eligible':pending.append(c)
        rows.append(dict(attempt=c['attempt'],generation=c['generation'],request=c['request']['request'],status=status,inventory=inv))
    return dict(selected_publications=len(selected),candidate_rows=rows,total=b.inventory(root),detailed=detailed_inventory(root,immutable_paths(root,retained),immutable_paths(root,pending)),pending_bytes=sum(r['inventory']['allocated_file_bytes'] for r in rows if r['status']=='unselected-eligible'),orphan_candidate_bytes=sum(r['inventory']['allocated_file_bytes'] for r in rows if r['status']=='unselected-stale'),shared_metadata_allocation='immutable liveness counts unique actual files; shared selected pages counted once; unsealed preparations are not inferred safe to reclaim')
