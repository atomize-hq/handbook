"""Authentic N128 staged histories and shallow matched baseline controls.

All engine calls inherit the reviewed outer stage Job. Existing independent
Python decoders are read-only imports at their preserved integration pin.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import sys
from driver import Driver
from job_guard import Budget, census, sha_file, write_json

RUNTIME = Path('C:/file-native-online-matrix-20261020')
PRIOR = Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook')
RESEARCH = PRIOR/'docs/research'
BASELINE = Path('C:/file-native-packing-20261020/bin/candidate2.exe')
EXPECTED = '568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'


def budget():
    return Budget(os.environ['FILE_NATIVE_BUDGET'])


def driver(name):
    assert sha_file(BASELINE) == EXPECTED, 'baseline-binary-binding'
    return Driver(BASELINE, RUNTIME/'receipts'/(name+'-events.jsonl'))


def oracle(root, output):
    assert sys.dont_write_bytecode, 'read-only-imports-require-python-B'
    sys.path.insert(0, str(RESEARCH/'file-native-packing-2026-10-20'))
    import independent
    result = independent.verify(root, output)
    assert result['navigation']['readable_declared_current']
    assert result['navigation']['discovery_relationships_and_extraction']
    return result


def actual(root):
    sys.path.insert(0, str(RESEARCH/'file-native-packing-2026-10-20'))
    from independent import Decoder
    d = Decoder(root)
    selected = Driver.head(root)
    p = json.loads(d.checked(Path(root)/f"publications/{selected['publication']}.json", selected['digest']))
    return dict(selected=selected, actual_H=p['sequence']+1,
                api_history_non_genesis=p['sequence'], documents=p['graph']['count'],
                includes_genesis=True, corpus=census(root))


def bind():
    assert sha_file(BASELINE) == EXPECTED
    engine = RESEARCH/'file-native-incremental-engine-experiment-2026-10-18'
    paths = sorted((engine/'src').glob('*.rs'))+[engine/'Cargo.toml', engine/'Cargo.lock']
    assert len(paths) == 18
    helpers = [RESEARCH/'file-native-packing-2026-10-20/independent.py',
               RESEARCH/'file-native-recovery-reclamation-2026-10-19/verify_operational.py',
               RESEARCH/'file-native-incremental-engine-experiment-2026-10-18/verify_store.py',
               RESEARCH/'file-native-navigation-2026-10-19/oracle.py']
    row = dict(integration='a0b92fe5dc64269508601dcddf7e2e2ca477e30d',
               core='d70f3864f2888d46d9112213b4a03f8d82745a8e',
               baseline_executable=str(BASELINE), executable_sha256=EXPECTED,
               source_files=[dict(path=str(p), bytes=p.stat().st_size, sha256=sha_file(p)) for p in paths],
               oracle_files=[dict(path=str(p), bytes=p.stat().st_size, sha256=sha_file(p)) for p in helpers],
               no_build=True, protected_sources_read_only=True)
    write_json(RUNTIME/'receipts/baseline-source-binding.json', row)


def history_init():
    root = RUNTIME/'history/authentic-N128/disposable/store'
    assert not root.exists()
    budget().check({'history': dict(bytes=16*1024**2, files=5000)})
    d = driver('history-init')
    d.call(root, 'init', count=128, mode='A', consumer='project')
    d.call(root, 'lifecycle', operation='setup')
    oracle(root, RUNTIME/'receipts/history-genesis-independent.json')
    write_json(RUNTIME/'receipts/history-genesis.json', actual(root))


def history_append(target, chunk=100):
    root = RUNTIME/'history/authentic-N128/disposable/store'
    d = driver('history-'+str(target))
    inspection = d.call(root, 'inspect')['result']
    offset = inspection['publication']['sequence']
    assert target in [1000, 10000] and 0 <= offset < target
    assert 1 <= chunk <= 100, 'bounded-resident-history-chunk'
    count = min(chunk, target-offset)
    end = offset+count
    # Explicit conservative per-new-publication forecast. H1k is a required
    # staging census, never an assumption that the H10k lane will fit.
    reserve = count*64*1024+16*1024**2
    admission = budget().check({'history': dict(bytes=reserve, files=count*32+5000)})
    write_json(RUNTIME/'receipts'/f'history-{offset}-{end}-prospective.json', admission)
    observed = d.call(root, 'history', timeout=120,
                      count=count, start=offset, env={'EXP_DEFER_REFRESH': '1'})
    assert observed['result']['completed'] == count
    record = actual(root)
    assert record['documents'] == 128 and record['actual_H'] == end+1
    record.update(history_profile='authentic resident API mixed content/metadata/no-op; bounded100publicationchunks',
                  durable_history_api_calls=end, readable_A_current_qualified=False,
                  deferred_refresh_pending=True)
    write_json(RUNTIME/'receipts'/f'history-{offset}-{end}-chunk.json', record)
    if end == target:
        d.call(root, 'refresh', timeout=240)
        result = oracle(root, RUNTIME/'receipts'/f'history-{target}-independent.json')
        assert result['semantic']['ledger_rows'] == target
        record.update(full_independent=result, readable_A_current_qualified=True,
                      deferred_refresh_pending=False)
        write_json(RUNTIME/'receipts'/f'history-{target}-summary.json', record)


def history_match(target, in_place=False):
    source = RUNTIME/'history/authentic-N128/disposable/store'
    assert not in_place or target == 10000
    destination = source if in_place else RUNTIME/'history'/f'H{target}-baseline/disposable/store'
    if not in_place:
        assert not destination.exists()
        size = census(source)
        budget().check({'history': dict(bytes=size['bytes']+16*1024**2,
                                        files=size['files']+1000)})
        # Quiescent full copy, no control rewind or imported delete marker.
        shutil.copytree(source, destination, ignore=shutil.ignore_patterns('DISPOSABLE.json'))
        assert not (destination/'DISPOSABLE.json').exists()
    else:
        budget().check({'history': dict(bytes=16*1024**2, files=1000)})
    d = driver('history-match-'+str(target))
    before = d.call(destination, 'inspect')['result']['history_depth']
    assert before == target+1
    lost_request = dict(request='deep-lost-reply', base=d.head(destination), ops=[], packet=False)
    d.call(destination, 'publish', request=lost_request, code=86, env={'EXP_EXIT': 'reply'})
    lost = d.call(destination, 'resolve', request_id='deep-lost-reply')['result']
    assert lost['outcome'] == 'accepted'
    retry = d.call(destination, 'publish', request=lost_request)['result']
    assert retry['status'] == 'deduplicated' and retry['result'] == lost
    assert d.call(destination, 'cancel', request_id='deep-lost-reply')['result'] == lost
    wrong_intent = d.call(destination, 'resolve', request_id='deep-lost-reply', intent='0'*64, code=2)
    assert wrong_intent['error'] == 'refused:request-id-intent-mismatch'
    wrong_retry = d.call(destination, 'publish', request={**lost_request, 'packet': True}, code=2)
    assert wrong_retry['error'] == 'refused:request-id-intent-mismatch'
    d.call(destination, 'cancel', request_id='cancelled')
    d.call(destination, 'refresh', timeout=240)
    categories = [dict(category='early', id='history-000001'),
                  dict(category='middle', id=f'history-{target//2:06}'),
                  dict(category='recent', id=f'history-{target:06}'),
                  dict(category='missing', id='missing'),
                  dict(category='cancelled', id='cancelled'),
                  dict(category='lost-reply', id='deep-lost-reply')]
    outcomes = {r['category']: d.call(destination, 'resolve', request_id=r['id'])['result'] for r in categories}
    result = oracle(destination, RUNTIME/'receipts'/f'H{target}-baseline-independent.json')
    from outcome_oracle import verify as verify_intents
    result['request_intents'] = verify_intents(destination, known_requests={'deep-lost-reply': lost_request})
    row = actual(destination)
    assert row['actual_H'] == target+2
    row.update(requests=categories, expected_outcomes=outcomes,
               fixture=str(destination), matched_representation_plan='same exact frozen accepted cohort; physical-only conversion after baseline samples',
               parent_H=before, lost_reply_exit=86, independent=result,
               exact_retry_deduplicated=True, changed_intent_refused=True,
               operational_history=True, subset_of_physical_records=False)
    write_json(RUNTIME/'receipts'/f'H{target}-baseline.json', row)


def controls(mode, count):
    label = f'N{count}-{mode}'
    root = RUNTIME/'document-controls'/label/'baseline/disposable/store'
    assert not root.exists()
    initial_forecast = max(16*1024**2, (3301302*count+127)//128+4*1024**2)
    budget().check({'document_controls': dict(bytes=initial_forecast, files=8000)})
    d = driver('control-'+label)
    d.call(root, 'init', mode=mode, consumer='project', count=count, timeout=240)
    d.call(root, 'lifecycle', operation='setup')
    initial = root.parents[2]/'matched-initial/disposable/store'
    current = census(root)
    budget().check({'document_controls': dict(bytes=current['bytes']+8*1024**2,
                                             files=current['files']+2000)})
    shutil.copytree(root, initial, ignore=shutil.ignore_patterns('DISPOSABLE.json'))
    observations = []
    # Identical shallow state sequence is used later in matched candidate copies.
    for name, ops in [('payload', [dict(kind='edit', id='d000004', text=' matched payload')]),
                      ('title', [dict(kind='rename', id='d000004', text='Matched title')])]:
        readable = 0
        if mode == 'B':
            selected = d.head(root)
            p = json.loads((root/f"publications/{selected['publication']}.json").read_bytes())
            readable = census(root/'generations'/p['attempt'])['bytes']
        budget().check({'document_controls': dict(bytes=readable+4*1024**2, files=count+1000)})
        response = d.publish(root, 'control-'+name, ops=ops, timeout=240)
        assert response['result']['status'] == 'published'
        observations.append(dict(case=name, engine_ns=response['wall_ns'],
                                 spawn_total_ns=d.last['spawn_total_ns'], response=response))
    result = oracle(root, RUNTIME/'receipts'/(label+'-baseline-independent.json'))
    write_json(RUNTIME/'receipts'/(label+'-baseline.json'), dict(mode=mode, documents=count,
               observations=observations, independent=result, actual_H=3,
               qualification='shallow baseline control; candidate matched initial fixture must reproduce exact requests and payload bytes'))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('stage', choices=['bind', 'history-init', 'history-append', 'history-match', 'controls'])
    parser.add_argument('--target', type=int)
    parser.add_argument('--chunk', type=int, default=100)
    parser.add_argument('--mode', choices=['A', 'B'])
    parser.add_argument('--in-place', action='store_true')
    parser.add_argument('--count', type=int, choices=[64, 1000])
    args = parser.parse_args()
    if args.stage == 'bind': bind()
    elif args.stage == 'history-init': history_init()
    elif args.stage == 'history-append': history_append(args.target, args.chunk)
    elif args.stage == 'history-match': history_match(args.target, args.in_place)
    else: controls(args.mode, args.count)
