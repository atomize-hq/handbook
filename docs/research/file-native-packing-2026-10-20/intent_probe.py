"""Original accepted request/intent checks, outside warm timing samples."""
import argparse,copy,json,sys
from pathlib import Path
from driver import Driver
from independent import sha
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'file-native-incremental-engine-experiment-2026-10-18'))
from verify_store import CachedPoints,checked
R=Path('C:/file-native-packing-20261020');ROOT=R/'history-restored/disposable/store'
def run(cell,publish=False):
    d=Driver(R/'long-matrix'/(cell+'-intent-events'));selected=(ROOT/'SELECTED.json').read_bytes();s=json.loads(selected)
    old=cell=='L-old';exe=Path('C:/file-native-navigation-20261019/bin/final.exe') if old else R/'bin/candidate1.exe'
    if old:assert json.loads((ROOT/'STORE.json').read_bytes())['version']==1,'L-intent-probe-must-precede-activation'
    p=json.loads(checked(ROOT/f"publications/{s['publication']}.json",s['digest']));request='history-050000';ref=CachedPoints(ROOT).point(p['outcomes'],sha(request.encode()));assert ref['request']==request
    publication=json.loads(checked(ROOT/f"publications/{ref['selected']['publication']}.json",ref['selected']['digest']));candidate=json.loads((ROOT/f"attempts/{publication['attempt']}/candidate.json").read_bytes());expected=json.loads(checked(ROOT/f"results/{ref['result_hash']}.json",ref['result_hash']))
    assert candidate['intent']==ref['intent'] and sha(json.dumps(candidate['request'],indent=2,ensure_ascii=False).encode())==ref['intent']
    same=d.call(ROOT,'resolve',exe_override=exe,request_id=request,intent=ref['intent']);assert same['result']==expected
    wrong_intent=sha(b'P-explicit-different-original-intent');assert wrong_intent!=ref['intent'];different=d.call(ROOT,'resolve',exe_override=exe,request_id=request,intent=wrong_intent,code=2);assert different['error']=='refused:request-id-intent-mismatch'
    if publish:
        same_request=d.call(ROOT,'publish',exe_override=exe,request=candidate['request']);assert same_request['result']==dict(status='deduplicated',result=expected)
        changed=copy.deepcopy(candidate['request']);changed['packet']=not changed['packet'];different_request=d.call(ROOT,'publish',exe_override=exe,request=changed,code=2);assert different_request['error']=='refused:request-id-intent-mismatch'
    assert (ROOT/'SELECTED.json').read_bytes()==selected
    if old:assert json.loads((ROOT/'STORE.json').read_bytes())['version']==1,'old-processes-must-close-before-activation'
    result=dict(status='passed',cell=cell,request=request,original_publication=ref['selected'],original_intent=ref['intent'],same_intent_exact_original_outcome=True,different_intent_refusal=different['error'],original_request_publish_replay_checked=publish,selected_unchanged=True,events=len(d.events))
    (R/'long-matrix'/(cell+'-intent.json')).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('cell');a.add_argument('--publish',action='store_true');n=a.parse_args();run(n.cell,n.publish)
