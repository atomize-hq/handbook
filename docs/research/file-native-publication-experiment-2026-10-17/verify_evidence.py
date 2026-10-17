"""Check archived bytes, pins, declared counts and positive growth evidence."""
import collections,gzip,hashlib,json,pathlib,subprocess
HERE=pathlib.Path(__file__).resolve().parent
def sha(b):return hashlib.sha256(b).hexdigest()
def load(p):return json.loads(p.read_bytes())
def main():
    e=HERE/'evidence';pins=load(e/'pins.json');raw=load(e/'raw-index.json');decoded=0
    for item in raw:
        b=gzip.decompress((e/item['artifact']).read_bytes())
        assert sha(b)==item['uncompressed_sha256'] and len(b)==item['uncompressed_bytes']
        lines=b.decode('utf-8-sig').splitlines();assert len(lines)==item['lines']
        for line in lines:json.loads(line)
        decoded+=len(lines)
    for name,digest in pins['source_sha256'].items():assert sha((HERE/name).read_bytes())==digest,name
    assert sha(pathlib.Path(pins['binary']).read_bytes())==pins['binary_sha256']
    assert sha((e/'execution-proposal-input.md').read_bytes())==pins['controlling_proposal_sha256']
    faults=load(e/'run-3-fault-observations.json');assert len(faults)==233
    phases=collections.Counter(f.get('phase_oracle','partial-retry') for f in faults)
    assert phases=={'must-not-select':174,'must-retain':51,'visible-uncertain':6,'partial-retry':2}
    assert all(f['selected_at_reopen'] for f in faults if f.get('phase_oracle')=='must-retain')
    assert all(not f['selected_at_reopen'] for f in faults if f.get('phase_oracle')=='must-not-select')
    assert len(load(e/'run-3-race-observations.json'))==120
    assert len(load(e/'run-3-semantic-race-observations.json'))==480
    assert len(load(e/'run-3-slow-sequence-observations.json'))==4
    assert len(load(e/'run-3-import-move-observations.json'))==2
    rows=load(e/'growth-summary.json');assert len(rows)==12
    complete=[r for r in rows if r.get('retained_selected_publications')=={1000:20,10000:10,100000:3}[r['tier']]]
    assert len(complete)==10
    for r in complete:assert len(r['accepted_request_wall_ms'])+1==r['retained_selected_publications']
    large=[r for r in rows if r['tier']==100000]
    assert len(large)==2 and all(r['initialization_timeout'] and 'incomplete' in r for r in large)
    inv=load(e/'inventory.json');assert inv['total_allocated_file_bytes']<inv['cap_bytes'] and inv['disk_free_at_inventory']>inv['reserve_bytes']
    staged=subprocess.check_output(['git','diff','--cached','--name-only'],cwd=HERE).decode().splitlines()
    prefix='docs/research/file-native-publication-experiment-2026-10-17/'
    assert staged and all(path.startswith(prefix) for path in staged)
    result={'status':'verified','raw_artifacts':len(raw),'raw_json_lines':decoded,'fault_phase_counts':dict(phases),'completed_growth_configurations':len(complete),'censored_large_configurations':len(large),'allocated_file_bytes':inv['total_allocated_file_bytes'],'free_bytes':inv['disk_free_at_inventory'],'native_binary_sha256':pins['binary_sha256'],'verifier_sha256':sha(pathlib.Path(__file__).read_bytes()),'scope':'new experiment directory only; no indexed GitNexus coverage inferred','limits':'artifact consistency and recorded logical assertions; not native Linux, UI, OS crash, GC or a new performance rerun'}
    (e/'verification.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8');print(json.dumps(result))
if __name__=='__main__':main()
