"""Strict evidence refusal, packet-owner retention and transport damage checks."""
import argparse,json,sys
from pathlib import Path
from operational_backup import capture,restore
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'file-native-incremental-engine-experiment-2026-10-18'))
from controller import Driver
from checks import req,op,result,expect_error
from verify_store import Tree

def run(exe,base):
    d=Driver(exe,base,900);rows=[]
    for mode in ['A','B']:
        root=d.base/mode/'disposable/store';r=str(root)
        def call(action,allow=False,**kw):return d.call(dict(root=r,action=action,**kw),allow=allow)
        call('init',mode=mode,consumer='project',count=128);call('lifecycle',operation='setup')
        call('publish',request=req('packet',packet=True));head=result(call('inspect'));packet=head['publication']['packet_ref'];accepted=result(call('resolve',request_id='packet'))
        call('checkpoint',request_id='carry-packet');call('lifecycle',operation='retire',selected=head['selected']);call('lifecycle',operation='collect')
        assert (root/f"attempts/{packet['attempt']}/packet/MANIFEST.json").exists();call('audit');assert result(call('resolve',request_id='packet'))==accepted
        p=result(call('inspect'))['publication'];entries=Tree(root).entries(p['projection'])
        for name in ['collection.yaml','packet/context.md','packet/provenance.json']:
            path=root/entries[name]['source'];saved=path.read_bytes()
            for damage in ['missing','corrupt']:
                if damage=='missing':path.unlink()
                else:path.write_bytes(b'corrupt')
                expect_error(call('audit',allow=True));assert not path.exists() or path.read_bytes()!=saved
                path.write_bytes(saved);call('audit')
        node=Tree(root).node(p['projection']);assert node[0]=='branch';h=node[2][0][0];path=root/'objects'/h[:2]/h;saved=path.read_bytes();path.unlink();expect_error(call('audit',allow=True));assert not path.exists();path.write_bytes(saved)
        before=result(call('inspect'))['selected'];expect_error(call('publish',allow=True,request=req('case-ancestor',[op('path',id='d000008',text='docs/item-ATTACHMENTS-d000009/evidence')])), 'ancestor');assert result(call('inspect'))['selected']==before
        # A damaged complete archive is refused before a new root is created.
        archive=d.base/('archive-'+mode);capture(root,archive,'source-manifest',True);transport=archive/'operational.bin.gz';saved=transport.read_bytes();transport.write_bytes(saved[:-1])
        destination=d.base/('damaged-restore-'+mode)/'disposable/store'
        try:restore(archive,destination)
        except ValueError:pass
        else:raise AssertionError('damaged transport accepted')
        assert not destination.exists();transport.write_bytes(saved)
        rows.append(dict(mode=mode,strict_missing_corrupt_evidence_refused=True,packet_owner_survives_retirement=True,case_ancestor_refused=True,damaged_transport_refused=True))
    (d.base/'extra.json').write_text(json.dumps(rows,indent=2),encoding='utf-8');print(json.dumps(rows,indent=2))

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args();run(ns.exe,ns.base)
