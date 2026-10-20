"""Small P A/B operational, interruption, lifetime and malformed-byte capsule."""
import argparse,ctypes,hashlib,json,os,shutil,struct,sys,threading,time
from pathlib import Path
from driver import Driver,EXE
from independent import Decoder,freeze,verify,sha
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'))
import operational_backup as backup
backup.EXCLUDED|={'pack-readers.lock','pack-maintenance.lock'}
def clone(source,destination):
    shutil.copytree(source,destination);return destination
def head(root):return json.loads((root/'SELECTED.json').read_bytes())
def asynchronous(d,root,action,**kw):
    state={}
    def run():
        try:state['result']=d.call(root,action,**kw)
        except BaseException as e:state['error']=repr(e)
    t=threading.Thread(target=run);t.start();return t,state
def wait(path):
    start=time.monotonic()
    while not path.exists():assert time.monotonic()-start<15,path;time.sleep(.01)
def finished(thread,state):
    thread.join(timeout=120);assert not thread.is_alive() and 'error' not in state,state;return state['result']
def exclusive_delete_handle(path):
    from ctypes import wintypes as w
    k=ctypes.WinDLL('kernel32',use_last_error=True);k.CreateFileW.restype=w.HANDLE;k.CreateFileW.argtypes=[w.LPCWSTR,w.DWORD,w.DWORD,ctypes.c_void_p,w.DWORD,w.DWORD,w.HANDLE];k.CloseHandle.argtypes=[w.HANDLE]
    h=k.CreateFileW(str(path.resolve()),0x80000000,3,None,3,0,None);assert h!=w.HANDLE(-1).value;return lambda:k.CloseHandle(h)
def catalog(root):
    config=json.loads((root/'STORE.json').read_bytes());return json.loads((root/f"pack-catalogs/{config['catalog']}.json").read_bytes())
def rebind(root,cat,index_change=None):
    if index_change:
        d,data=index_change;h=sha(data);(root/f'packs/{h}.idx').write_bytes(data);d['index']=h;d['index_bytes']=len(data)
    raw=json.dumps(cat,indent=2).encode();h=sha(raw);(root/f'pack-catalogs/{h}.json').write_bytes(raw)
    c=json.loads((root/'STORE.json').read_bytes());c['catalog']=h;(root/'STORE.json').write_text(json.dumps(c,indent=2))
def malformed(d,seed,base):
    cases=[]
    variants=['missing-catalog','corrupt-catalog','missing-index','corrupt-index','missing-pack','truncated-pack','corrupt-record','offset-overflow','raw-length-bound','duplicate-key','omitted-entry','wrong-pack-binding','raw-hash-mismatch','bad-codec','too-many-packs','duplicate-json-key']
    for kind in variants:
        root=clone(seed,base/kind/'disposable/store');d.call(root,'lifecycle',operation='setup');cat=catalog(root);desc=next(x for x in cat['packs'] if x['role']=='publication');config=json.loads((root/'STORE.json').read_bytes())
        cp=root/f"pack-catalogs/{config['catalog']}.json";ip=root/f"packs/{desc['index']}.idx";pp=root/f"packs/{desc['pack']}.pack"
        if kind=='missing-catalog':cp.rename(cp.with_suffix('.missing'))
        elif kind=='corrupt-catalog':cp.write_bytes(cp.read_bytes()+b' ')
        elif kind=='missing-index':ip.rename(ip.with_suffix('.missing'))
        elif kind=='corrupt-index':data=bytearray(ip.read_bytes());data[-1]^=1;ip.write_bytes(data)
        elif kind=='missing-pack':pp.rename(pp.with_suffix('.missing'))
        elif kind=='truncated-pack':pp.write_bytes(pp.read_bytes()[:-1])
        elif kind=='corrupt-record':
            data=bytearray(pp.read_bytes());offset=Decoder(root).entries[f"publications/{head(root)['publication']}.json"][1];data[offset]^=1;pp.write_bytes(data)
        elif kind=='too-many-packs':cat['packs']=[cat['packs'][0]]*65;rebind(root,cat)
        elif kind=='duplicate-json-key':
            raw=cp.read_bytes();raw=raw.replace(b'"version": 1',b'"version": 1, "version": 1',1);h=sha(raw);(root/f'pack-catalogs/{h}.json').write_bytes(raw);config['catalog']=h;(root/'STORE.json').write_text(json.dumps(config))
        else:
            data=bytearray(ip.read_bytes());n=struct.unpack_from('<H',data,53)[0];tail=55+n
            if kind=='offset-overflow':struct.pack_into('<Q',data,tail,2**64-1)
            elif kind=='raw-length-bound':struct.pack_into('<I',data,tail+12,8*1024**2+1)
            elif kind=='wrong-pack-binding':data[8]^=1
            elif kind=='raw-hash-mismatch':
                # Alter every record digest so the head's touched record fails.
                pos=53
                for _ in range(desc['records']):
                    length=struct.unpack_from('<H',data,pos)[0];pos+=2+length;data[pos+16]^=1;pos+=48
            elif kind=='bad-codec':data[48]=2;desc['codec']=2
            elif kind in ['duplicate-key','omitted-entry']:
                entries=[];pos=53
                for _ in range(desc['records']):
                    length=struct.unpack_from('<H',data,pos)[0];end=pos+2+length+48;entries.append(bytes(data[pos:end]));pos=end
                assert len(entries)>=2
                if kind=='duplicate-key':
                    length=struct.unpack_from('<H',entries[0])[0];entries[1]=entries[0][:2+length]+entries[1][-48:]
                else:entries.pop();desc['records']-=1;struct.pack_into('<I',data,49,desc['records'])
                data=data[:53]+b''.join(entries)
            rebind(root,cat,(desc,data))
        result=d.call(root,'resolve',request_id='initial',code=2);assert result['error'] and result['result'] is None
        cases.append(dict(case=kind,strict_integrity_refusal=result['error'],loose_duplicate_present=bool(list((root/'publications').glob('*.json')))))
    return cases
def run(base):
    base=Path(base);d=Driver(base);cases=[]
    def note(name,**fields):cases.append(dict(case=name,**fields));(base/'cases.json').write_text(json.dumps(cases,indent=2))
    for mode in ['A','B']:
        root=base/('operational-'+mode)/'disposable/store';d.call(root,'init',mode=mode,consumer='project',count=128);d.call(root,'lifecycle',operation='setup');initial=head(root)
        d.call(root,'lifecycle',operation='pin',name='original',selected=initial)
        oversize=None
        if mode=='A':
            p=json.loads((root/f"publications/{initial['publication']}.json").read_bytes());oversize=root/f"attempts/{p['attempt']}/allocations/oversize-journal.json"
            # Synthetic valid retained journal bytes test eligibility without a
            # new large operational corpus or changing any selected control.
            oversized=(json.dumps(dict(publication='oversize-journal',result=p['result_hash'])).encode()+b' '*(8*1024**2+1));oversize.write_bytes(oversized);oversize_hash=sha(oversized)
        d.publish(root,'packet-owner',packet=True);packet_head=head(root);d.call(root,'checkpoint',request_id='packet-checkpoint')
        eligible=d.call(root,'prepare',request=dict(request='eligible',base=head(root),ops=[],packet=False))['result']['candidate'];eligible_bytes=(root/eligible).read_bytes()
        if mode=='A':
            d.publish(root,'pending-A',[dict(kind='rename',id='d000004',text='pending packed title')],env={'EXP_DEFER_REFRESH':'1'})
            d.call(root,'refresh',code=86,env={'EXP_EXIT':'refresh-plan'});pending=(root/'PENDING.json').read_bytes()
        inventory=base/(mode+'-L-inventory.jsonl');rows=freeze(root,inventory,True)
        packed=d.call(root,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True)
        if oversize:
            assert oversize.exists() and sha(oversize.read_bytes())==oversize_hash and packed['metrics']['pack_excluded_records']==1
        assert (root/eligible).read_bytes()==eligible_bytes
        if mode=='A':
            assert (root/'PENDING.json').read_bytes()==pending
            archive=base/'A-genuinely-pending-packed-backup';backup.capture(root,archive,'final-P-candidate',True)
            pending_restored=base/'A-genuinely-pending-restored'/'disposable/store';backup.restore(archive,pending_restored)
            assert (pending_restored/'PENDING.json').read_bytes()==pending
            d.call(pending_restored,'lifecycle',operation='setup');d.call(pending_restored,'refresh');verify(pending_restored,base/'A-pending-restored-independent.json',inventory)
            assert d.call(root,'select',candidate=eligible,code=2)['error']=='conflict:expected-publication'
            attempt=eligible.split('/')[1];d.call(root,'lifecycle',operation='abandon',attempt=attempt);d.call(root,'refresh')
        else:d.call(root,'select',candidate=eligible)
        verify(root,base/(mode+'-P0-independent.json'),inventory)
        grouping=[(x['role'],x['first'],x['last'],x['records']) for x in catalog(root)['packs']]
        d.publish(root,'loose-tail')
        d.call(root,'packing',operation='build',codec='zstd',cleanup=False)
        assert grouping==[(x['role'],x['first'],x['last'],x['records']) for x in catalog(root)['packs']]
        verify(root,base/(mode+'-PZ-independent.json'),inventory)
        if os.name=='nt':
            journal=json.loads((root/'PACKING.json').read_bytes());old=journal['old'][0];held=root/f"packs/{old['pack']}.pack";release=exclusive_delete_handle(held)
            deferred=d.call(root,'packing',operation='cleanup')['result'];assert deferred['status']=='cleaning' and deferred['deferred_bytes']>=held.stat().st_size
        else:release=lambda:None
        # Backup includes deferred old containers and pending maintenance ownership.
        archive=base/(mode+'-packed-backup');backup.capture(root,archive,'final-P-candidate',True)
        restored=base/(mode+'-packed-restored')/'disposable/store';backup.restore(archive,restored);verify(restored,base/(mode+'-restored-independent.json'),inventory)
        d.call(restored,'inspect');d.call(restored,'lifecycle',operation='setup');d.call(restored,'packing',operation='cleanup');d.publish(restored,'restored-valid-successor')
        release();d.call(root,'packing',operation='cleanup')
        packet_publication=json.loads(Decoder(root).raw(f"publications/{packet_head['publication']}.json"));packet_attempt=packet_publication['packet_ref']['attempt'];packet_manifest=root/f'attempts/{packet_attempt}/packet/MANIFEST.json';packet_bytes=packet_manifest.read_bytes()
        d.call(root,'lifecycle',operation='retire',selected=packet_head)
        d.call(root,'lifecycle',operation='release',name='original');d.call(root,'lifecycle',operation='retire',selected=initial);d.call(root,'lifecycle',operation='collect')
        before=set(Decoder(root).entries);d.call(root,'packing',operation='repack',codec='zstd',include_attempts=True,cleanup=True);after=set(Decoder(root).entries)
        assert len(after)<len(before) and not any(key.startswith('attempts/'+json.loads(Decoder(root).raw(f"publications/{initial['publication']}.json"))['attempt']+'/') for key in after)
        assert packet_manifest.read_bytes()==packet_bytes and f'attempts/{packet_attempt}/candidate.json' in after
        assert d.call(root,'point',code=2,selected=initial,id='d000000')['error']=='retired:snapshot'
        d.call(root,'audit');verify(root,base/(mode+'-repacked-independent.json'))
        note('operational-'+mode,raw_records=len(rows),same_P0_PZ_inventory_and_grouping=True,pins_candidates_packets_pending_A=True,backup_deferred_cleanup_and_successor=True,retired_attempt_repack=True)
    # Small N16 fixtures retain every interrupted prefix and raw return.
    for mode in ['A','B']:
        seed=base/('fault-seed-'+mode)/'disposable/store';d.call(seed,'init',mode=mode,consumer='project',count=16);d.call(seed,'lifecycle',operation='setup');d.publish(seed,'second');d.publish(seed,'third')
        expected=d.call(seed,'resolve',request_id='initial')['result']
        for phase,nth in [('pack-reservation',1),('pack-build',1),('pack-index',1),('pack-install',1),('pack-before-catalog',1),('pack-catalog',1),('pack-accounting',1),('pack-loose-delete',1),('pack-loose-delete',3)]:
            root=clone(seed,base/f'{mode}-{phase}-{nth}'/'disposable/store');d.call(root,'lifecycle',operation='setup')
            inventory=base/f'{mode}-{phase}-{nth}.jsonl';freeze(root,inventory,True)
            d.call(root,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True,code=86,env={'EXP_EXIT':phase,'EXP_NTH':str(nth)})
            assert d.call(root,'resolve',request_id='initial')['result']==expected
            journal=json.loads((root/'PACKING.json').read_bytes());assert journal['reserved_bytes']>0
            if mode=='A' and phase=='pack-install':
                archive=base/'orphan-unregistered-backup';backup.capture(root,archive,'final-P-candidate',True);orphan_restored=base/'orphan-unregistered-restored'/'disposable/store';backup.restore(archive,orphan_restored)
                d.call(orphan_restored,'lifecycle',operation='setup');d.call(orphan_restored,'packing',operation='abort');d.call(orphan_restored,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True);d.call(orphan_restored,'audit');note('orphan-unregistered-backup-restore')
            if journal['status']=='building':d.call(root,'packing',operation='abort');d.call(root,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True)
            else:d.call(root,'packing',operation='resume',cleanup=True)
            verify(root,base/f'{mode}-{phase}-{nth}-independent.json',inventory)
            note('interruption-'+mode+'-'+phase+'-'+str(nth),prefix_preserved=True,reservation_survived=True,original_outcome_exact=True)
        packed=clone(seed,base/('malformed-seed-'+mode)/'disposable/store');d.call(packed,'lifecycle',operation='setup');d.call(packed,'packing',operation='build',codec='raw',include_attempts=True,cleanup=False)
        if mode=='A':
            cases.extend(malformed(d,packed,base/'malformed'));(base/'cases.json').write_text(json.dumps(cases,indent=2))
            # Snapshot taken after completed installation but before catalog activation.
            staging=clone(seed,base/'unselected-staging'/'disposable/store');d.call(staging,'lifecycle',operation='setup');d.call(staging,'packing',operation='build',codec='raw',include_attempts=True,code=86,env={'EXP_EXIT':'pack-before-catalog'})
            archive=base/'unselected-staging-backup';backup.capture(staging,archive,'final-P-candidate',True);restored=base/'unselected-staging-restored'/'disposable/store';backup.restore(archive,restored);d.call(restored,'lifecycle',operation='setup');d.call(restored,'packing',operation='resume',cleanup=True);d.call(restored,'audit');note('unselected-installed-backup-restore')
        # Each read holds its captured catalog from before any physical lookup
        # through the complete accepted-result read, including already active reads.
        for barrier in ['pack-read-lease','pack-catalog-captured','resolve-history-captured','pack-record-located']:
            root=clone(packed,base/(mode+'-reader-'+barrier)/'disposable/store');d.call(root,'lifecycle',operation='setup');ready=root.parent/'ready';release=root.parent/'release'
            t,state=asynchronous(d,root,'resolve',request_id='initial',env={'EXP_BARRIER':barrier,'EXP_READY':str(ready),'EXP_RELEASE':str(release)})
            wait(ready);result=d.call(root,'packing',operation='build',codec='zstd',code=2,env={'EXP_WAIT_MS':'100'});assert result['error']=='busy:physical-readers'
            release.write_bytes(b'go');assert finished(t,state)['result']==expected
            d.call(root,'packing',operation='resume',cleanup=True);d.call(root,'audit');note('protected-read-'+mode+'-'+barrier)
        # Writer can advance semantic head while historical packs build. A competing
        # maintainer refuses. Stale semantic candidate remains an expected conflict.
        root=clone(seed,base/(mode+'-writer-during-build')/'disposable/store');d.call(root,'lifecycle',operation='setup')
        prepared=d.call(root,'prepare',request=dict(request='stale-candidate',base=head(root),ops=[],packet=False))['result']['candidate']
        ready=root.parent/'ready';release=root.parent/'release'
        t,state=asynchronous(d,root,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True,env={'EXP_BARRIER':'pack-before-catalog','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
        wait(ready);d.call(root,'packing',operation='status',code=2);d.publish(root,'semantic-during-build');release.write_bytes(b'go');finished(t,state)
        assert d.call(root,'select',candidate=prepared,code=2)['error']=='conflict:expected-publication';d.call(root,'audit');note('writer-and-competing-maintenance-'+mode)
        # Cancelled draining releases both maintenance and preparation barriers.
        root=clone(packed,base/(mode+'-cancel-drain')/'disposable/store');d.call(root,'lifecycle',operation='setup');ready=root.parent/'reader-ready';release=root.parent/'reader-release'
        reader,reader_state=asynchronous(d,root,'resolve',request_id='initial',env={'EXP_BARRIER':'pack-record-located','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
        wait(ready);cancel=root.parent/'cancel';ready2=root.parent/'packing-ready';release2=root.parent/'packing-release'
        writer,writer_state=asynchronous(d,root,'packing',operation='build',codec='zstd',code=2,env={'EXP_BARRIER':'pack-reader-drain','EXP_READY':str(ready2),'EXP_RELEASE':str(release2),'EXP_PACK_CANCEL':str(cancel)})
        wait(ready2);cancel.write_bytes(b'cancel');release2.write_bytes(b'go');assert finished(writer,writer_state)['error']=='cancelled:pack-barrier'
        release.write_bytes(b'go');finished(reader,reader_state);d.call(root,'resolve',request_id='initial');d.call(root,'packing',operation='resume',cleanup=True);note('cancelled-drain-releases-barriers-'+mode)
        if os.name=='nt':
            root=clone(seed,base/(mode+'-loose-sharing')/'disposable/store');d.call(root,'lifecycle',operation='setup');loose=next((root/'publications').glob('*.json'));size=loose.stat().st_size;release=exclusive_delete_handle(loose)
            result=d.call(root,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True)['result'];assert result['status']=='cleaning' and result['deferred_bytes']>=size and loose.exists()
            release();d.call(root,'packing',operation='cleanup');assert not loose.exists();note('windows-loose-no-delete-sharing-'+mode,deferred_bytes_at_least=size)
        # Repack deletion interrupted after an actual old container removal.
        root=clone(seed,base/(mode+'-old-pack-deletion')/'disposable/store');d.call(root,'lifecycle',operation='setup');initial=head(root);d.call(root,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True);d.publish(root,'later');d.call(root,'lifecycle',operation='retire',selected=initial);d.call(root,'lifecycle',operation='collect')
        d.call(root,'packing',operation='repack',codec='zstd',include_attempts=True,cleanup=True,code=86,env={'EXP_EXIT':'pack-old-delete'});assert d.call(root,'resolve',request_id='initial')['result']==expected;d.call(root,'packing',operation='resume',cleanup=True);d.call(root,'audit');note('repack-old-pack-deletion-interruption-'+mode)
    result=dict(status='passed',cases=cases,events=len(d.events),exe_sha256=hashlib.sha256(EXE.read_bytes()).hexdigest(),operational_backup_exclusions=sorted(backup.EXCLUDED))
    (base/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(dict(status='passed',cases=len(cases),events=len(d.events)),indent=2));return result
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--base',required=True);a=p.parse_args();run(a.base)
