"""Separately admitted stages over the ONE verified H101006-derived clone.

This file is an execution harness, not a stage GO. Each invocation has its own
external fixed-cutoff watchdog; no SELECTED/control rewind is supported.
"""
import argparse,hashlib,json,sys,time
from pathlib import Path
import driver
from driver import Driver
from independent import Decoder,verify,role,sha
R=Path('C:/file-native-packing-20261020');ROOT=R/'history-restored/disposable/store';BASE=R/'long-matrix';OLD=Path('C:/file-native-navigation-20261019/bin/final.exe');NEW=R/'bin/candidate1.exe';HELPER=R/'build/release/qualified_lookup.exe'
def write(path,value):Path(path).write_text(json.dumps(value,indent=2),encoding='utf-8')
def read(path):return json.loads(Path(path).read_bytes())
def prep():
    BASE.mkdir(exist_ok=False);d=Driver(BASE/'prep-events');assert sha(OLD.read_bytes())=='d4b7c495abfc029e8fabd6fcfc75501c5de04ce72dd38847a69bc8824022303f';assert sha(NEW.read_bytes())=='478dd9ba9f687d4de81c60ce55f9ddf9e4e2fafcc493d581b3c63e12e42f7fe5'
    before=read(ROOT/'SELECTED.json');old=read(ROOT/f"publications/{before['publication']}.json");assert old['sequence']==101005
    # Separately timed navigation upgrade, using the qualified N binary before
    # physical activation. Its new publication is explicitly outside original keys.
    start=time.monotonic();upgrade=d.call(ROOT,'publish',exe_override=OLD,request=dict(request='P-navigation-upgrade',base=before,ops=[],packet=False));elapsed=time.monotonic()-start
    after=upgrade['result']['selected'];p=read(ROOT/f"publications/{after['publication']}.json");assert p['graph']==old['graph'] and p['sequence']==101006
    d.call(ROOT,'cancel',exe_override=OLD,request_id='P-benchmark-cancelled')
    added=[]
    for key in [f"publications/{p['id']}.json",f"results/{p['result_hash']}.json",f"attempt-hints/{p['attempt']}.json",f"attempts/{p['attempt']}/candidate.json",f"attempts/{p['attempt']}/allocations/{p['id']}.json"]:
        raw=(ROOT/key).read_bytes();added.append(dict(role=role(key),key=key,raw_length=len(raw),sha256=sha(raw)))
    capture=read(R/'post-gc-operational-backup/manifest.json')
    manifest=dict(profile='authenticated-operational-capture-role-inventory-plus-added-records',source_inventory=str(R/'post-gc-operational-backup/files.jsonl'),source_inventory_sha256=capture['inventory_sha256'],source_capture_transport_sha256=capture['transport_sha256'],original_selected=before,frozen_selected=after,added_records=added,record_identity='role, stable logical key, raw length, raw SHA-256; exact original bytes')
    write(BASE/'L-inventory.manifest.json',manifest)
    requests=[dict(category='early',id='history-000001'),dict(category='middle',id='history-050000'),dict(category='recent',id=old['request']),dict(category='missing',id='P-benchmark-missing'),dict(category='cancelled',id='P-benchmark-cancelled'),dict(category='lost-reply',id='deep-lost-reply')]
    expected={x['id']:d.call(ROOT,'resolve',exe_override=OLD,request_id=x['id'])['result'] for x in requests}
    write(BASE/'frozen.json',dict(source_H=101006,measurement_H=101007,selected=after,original_lookup_requests=requests,expected_outcomes=expected,original_graph=old['graph'],new_loose_tail_lookup_keys=[],navigation_upgrade_excluded_from_packing_timing=dict(seconds=elapsed,response=upgrade),original_N_exe_sha256=sha(OLD.read_bytes()),new_P_exe_sha256=sha(NEW.read_bytes())))
    d.call(ROOT,'audit',exe_override=OLD);print(json.dumps(dict(status='prepared',source_H=101006,measurement_H=101007,requests=requests)))
def measure(cell):
    frozen=read(BASE/'frozen.json');d=Driver(BASE/(cell+'-events'));assert read(ROOT/'SELECTED.json')==frozen['selected'];requests=frozen['original_lookup_requests']
    warm_exe=HELPER if cell=='L-old' else NEW;fresh_exe=OLD if cell=='L-old' else NEW
    warm=d.call(ROOT,'lookup-bench',exe_override=warm_exe,count=500,requests=requests)
    fresh=[]
    for request in requests:
        for i in range(20):
            start=time.perf_counter_ns();out=d.call(ROOT,'resolve',exe_override=fresh_exe,request_id=request['id']);elapsed=time.perf_counter_ns()-start;assert out['result']==frozen['expected_outcomes'][request['id']]
            fresh.append(dict(category=request['category'],sample=i,external_process_ns=elapsed,engine_wall_ns=out['wall_ns'],rss=out.get('peak_memory_bytes'),metrics=out['metrics']))
    if cell!='L-old':
        old=read(BASE/'L-old-measurement.json');assert [x['result_digest'] for x in warm['result']['rows']]==[x['result_digest'] for x in old['warm']['result']['rows']]
    result=dict(cell=cell,source_pin='4cb6b9fba9ffabc294a6006ec54fe75366968db4',warm_binary=str(warm_exe),warm_binary_sha256=sha(warm_exe.read_bytes()),fresh_binary=str(fresh_exe),fresh_binary_sha256=sha(fresh_exe.read_bytes()),warm=warm,fresh=fresh,process_starts=len(fresh),cache_qualification='same-process warm after first lookup; fresh process is not OS/device cold; cache eviction and physical writes unmeasured')
    write(BASE/(cell+'-measurement.json'),result);print(json.dumps(dict(cell=cell,process_starts=len(fresh),warm_samples_per_category=500)))
def build(cell):
    d=Driver(BASE/(cell+'-build-events'));frozen=read(BASE/'frozen.json');assert read(ROOT/'SELECTED.json')==frozen['selected']
    if cell=='P0':
        from guard import current
        admission=current();admission.update(total_cap_bytes=2684354560,total_cap_files=560000,P0_new_bound=644081109,measurement_margin=33554432)
        admission['prospective_bytes']=admission['total_new_bytes']+644081109+33554432;admission['prospective_files']=admission['total_new_files']+2000
        write(R/'receipts/p0-fresh-admission.json',admission)
        assert admission['prospective_bytes']<=2684354560 and admission['prospective_files']<=560000 and admission['free_bytes']>=140*1024**3
    out=d.call(ROOT,'packing',timeout=1800,exe_override=NEW,operation='build',codec='raw' if cell=='P0' else 'zstd',include_attempts=cell=='P0',reserve_bytes=644081109 if cell=='P0' else 350000000,cleanup=False)
    c=read(ROOT/'STORE.json');cat=read(ROOT/f"pack-catalogs/{c['catalog']}.json");grouping=[{k:x[k] for k in ['role','first','last','records']} for x in cat['packs']]
    if cell=='PZ':assert grouping==read(BASE/'P0-build.json')['grouping']
    result=dict(cell=cell,response=out,catalog=cat,grouping=grouping,pack_plus_index_bytes=sum(x['pack_bytes']+x['index_bytes'] for x in cat['packs']),physical_cleanup_complete=False,footprint_qualification='additive candidates and loose duplicates coexist; no compacted-store claim')
    write(BASE/(cell+'-build.json'),result);print(json.dumps(dict(cell=cell,pack_plus_index_bytes=result['pack_plus_index_bytes'],packs=len(cat['packs']))))
def independent(cell):
    result=verify(ROOT,BASE/(cell+'-independent.json'),BASE/'L-inventory.manifest.json',R/'preservation-inputs/post-gc-reference.json')
    assert result['semantic']['publications']==101007 and result['semantic']['ledger_rows']==100000 and result['semantic']['retired_publications']==101
    if cell=='PZ':assert result['inventory_digest']==read(BASE/'P0-independent.json')['inventory_digest']
def cleanup():
    d=Driver(BASE/'chosen-cleanup-events');assert read(BASE/'PZ-independent.json')['inventory_digest']==read(BASE/'P0-independent.json')['inventory_digest']
    out=d.call(ROOT,'packing',timeout=1800,exe_override=NEW,operation='cleanup');write(BASE/'chosen-cleanup.json',out);assert out['result']['status'] in ['complete','cleaning'];print(json.dumps(out['result']))
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('stage');n=a.parse_args()
    if n.stage=='prep':prep()
    elif n.stage.endswith('-measure'):measure(n.stage[:-8])
    elif n.stage.endswith('-build'):build(n.stage[:-6])
    elif n.stage.endswith('-verify'):independent(n.stage[:-7])
    elif n.stage=='cleanup':cleanup()
    else:raise ValueError(n.stage)
