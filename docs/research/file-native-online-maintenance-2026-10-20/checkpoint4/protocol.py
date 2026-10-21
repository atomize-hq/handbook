"""Prepared controls only. Run solely inside the admitted existing Windows Job."""
import concurrent.futures as cf
import hashlib
import json
import os
from pathlib import Path
import sys
import threading
import time

assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1', 'guarded-stage-required'
sys.path.insert(0, 'C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness')
from driver import Driver

BASE = Path('C:/file-native-online-cp1-20261020')
NAME = sys.argv[1]
assert NAME.startswith('cp4-') and all(c.isalnum() or c == '-' for c in NAME)
EVIDENCE = BASE / 'evidence'
D = Driver(BASE / 'build/release/file-native-incremental.exe', EVIDENCE / f'{NAME}-events.jsonl')
ROWS = []

def note(case, **fields):
    ROWS.append(dict(case=case, **fields))
    (EVIDENCE / f'{NAME}-results.json').write_text(json.dumps(dict(status='RUNNING', binary=D.binary_sha256, cases=ROWS), indent=2) + '\n')

def ctl(root, operation, **fields):
    return D.call(root, 'lifecycle', operation=operation, **fields)

def registry(root):
    return json.loads((root / 'RESERVATIONS.json').read_bytes())

def tree(root):
    return {str(p.relative_to(root)): (p.stat().st_size, hashlib.sha256(p.read_bytes()).hexdigest())
            for p in root.rglob('*') if p.is_file()}

def request(name, base):
    return dict(request=name, base=base, ops=[], packet=False)

def prepare(root, name, base=None):
    out = D.call(root, 'prepare', request=request(name, D.head(root) if base is None else base))
    return out['result']['candidate']

def attempt(path):
    return path.split('/')[1]

def seed(mode, tag, enable=True):
    root = BASE / f'fixtures/cp3/{NAME}-{tag}/disposable/store'
    D.call(root, 'init', mode=mode, consumer='project', count=10)
    ctl(root, 'setup')
    if enable:
        ctl(root, 'enable-terminal-v2', explicit_disposable_opt_in=True)
        assert registry(root)['version'] == 2
    return root

def stale(root, name):
    candidate = prepare(root, name)
    D.publish(root, name + '-advance')
    out = D.call(root, 'select', candidate=candidate, code=2)
    assert out['error'] == 'conflict:expected-publication', out
    return candidate

def release(root, candidate, **fields):
    return ctl(root, 'release-definitive-conflict', attempt=attempt(candidate), **fields)

def unchanged_refusal(root, operation, **fields):
    before = tree(root)
    out = ctl(root, operation, code=2, **fields)
    assert tree(root) == before, out
    return out

def wait(path):
    deadline = time.monotonic() + 12
    while not path.exists():
        assert time.monotonic() < deadline, path
        time.sleep(.005)

def held(root, tag, checkpoint, action, **fields):
    ready, resume = root.parent / f'{tag}-ready', root.parent / f'{tag}-resume'
    outputs, errors = [], []
    def target():
        try:
            outputs.append(D.call(root, action, env=dict(EXP_BARRIER=checkpoint, EXP_READY=str(ready), EXP_RELEASE=str(resume)), **fields))
        except BaseException as error:
            errors.append(error)
    thread = threading.Thread(target=target)
    thread.start()
    wait(ready)
    def finish():
        resume.write_text('release')
        thread.join(15)
        assert not thread.is_alive() and not errors and outputs, errors
        return outputs[0]
    return finish

# A v1 prepare already owns its reservation before conversion, including when
# it computes/seals AFTER opt-in. No new preselection provenance may be inferred.
legacy = seed('A', 'legacy-race', enable=False)
raw_v1 = (legacy / 'RESERVATIONS.json').read_bytes()
unchanged_refusal(legacy, 'enable-terminal-v2')
assert (legacy / 'RESERVATIONS.json').read_bytes() == raw_v1
finish = held(legacy, 'old-compute', 'semantic-root', 'prepare', request=request('legacy-computing', D.head(legacy)))
old_ids = set(registry(legacy))
ctl(legacy, 'enable-terminal-v2', explicit_disposable_opt_in=True)
candidate = finish()['result']['candidate']
state = registry(legacy)
assert old_ids == set(state['legacy']) and state['active'][attempt(candidate)]['state'] == 'sealed'
D.publish(legacy, 'legacy-advance')
unchanged_refusal(legacy, 'release-definitive-conflict', attempt=attempt(candidate))
note('v1-compute-opt-in-seal-stays-legacy', legacy=list(old_ids))

for mode in ['A', 'B']:
    root = seed(mode, 'rounds-' + mode)
    protected = prepare(root, mode + '-protected-preparation')
    packet = D.call(root, 'packet')['result']
    packet_bytes = tree(root / packet['path'])
    protected_bytes = (root / protected).read_bytes()
    initial = D.call(root, 'resolve', request_id='initial')['result']
    finish_reader = held(root, 'held-reader', 'reader-opened', 'point', id='d000000')
    started = time.monotonic()
    accepted = {'initial': initial}
    previous_charge = 0
    try:
        for round_no in range(4):
            base = D.head(root)
            names = [f'{mode}-round-{round_no}-{n}' for n in range(4)]
            with cf.ThreadPoolExecutor(max_workers=4) as pool:
                candidates = list(pool.map(lambda name: prepare(root, name, base), names))
            # The winner owns W while all three other real selectors contend.
            winner = held(root, f'winner-{round_no}', 'lock-acquired', 'select', candidate=candidates[0])
            begun = [threading.Event() for _ in range(3)]
            with cf.ThreadPoolExecutor(max_workers=3) as pool:
                futures = [pool.submit(D.call, root, 'select', candidate=c, code=2, on_started=lambda event, signal=begun[n]: signal.set()) for n, c in enumerate(candidates[1:])]
                assert all(signal.wait(10) for signal in begun)
                time.sleep(.1)
                selected = winner()
                assert selected['result']['status'] == 'published', selected
                accepted[names[0]] = selected['result']['result']
                for future in futures:
                    outcome = future.result()
                    assert outcome['error'] == 'conflict:expected-publication'
                    assert outcome['metrics']['ns']['lock-wait'] > 1000000
            for c in candidates[1:]:
                before_owner = registry(root)['active'][attempt(c)]
                generation = tree(root / 'generations' / attempt(c))
                out = release(root, c)['result']
                assert out['status'] == 'terminal-charged' and out['reclaimed_bytes'] == 0
                receipt_raw = (root / 'abandoned' / attempt(c)).read_bytes()
                receipt = json.loads(receipt_raw)
                assert receipt['used_bytes'] == before_owner['used']
                assert receipt['charge_bytes'] == before_owner['used'] + 131072 <= before_owner['bytes']
                assert receipt['control_stage'] == f'attempts/{attempt(c)}/terminal-control-stage'
                assert tree(root / 'generations' / attempt(c)) == generation
                assert hashlib.sha256(receipt_raw).hexdigest() == out['receipt_sha256']
                after = registry(root)
                assert after['terminal_bytes'] > previous_charge
                previous_charge = after['terminal_bytes']
                raw = (root / 'RESERVATIONS.json').read_bytes()
                assert release(root, c)['result']['status'] == 'terminal-already-charged'
                assert (root / 'RESERVATIONS.json').read_bytes() == raw
            assert len(registry(root)['active']) == 2
            assert registry(root)['terminal_count'] == (round_no + 1) * 3
            assert (root / protected).read_bytes() == protected_bytes
            assert tree(root / packet['path']) == packet_bytes
            for name, expected in accepted.items():
                assert D.call(root, 'resolve', request_id=name)['result'] == expected
            unchanged_refusal(root, 'collect')
        # Reuse the logical request after a definitive old-attempt release.
        retry = D.publish(root, names[1])
        assert retry['result']['status'] == 'published' and retry['result']['result']['request'] == names[1]
        assert D.call(root, 'resolve', request_id=names[1])['result'] == retry['result']['result']
        raw = (root / 'RESERVATIONS.json').read_bytes()
        assert release(root, candidates[1])['result']['status'] == 'terminal-already-charged'
        assert (root / 'RESERVATIONS.json').read_bytes() == raw
        elapsed = time.monotonic() - started
        if elapsed < 15.1:
            time.sleep(15.1 - elapsed)
    finally:
        assert finish_reader()['status'] == 'ok'
    assert (root / protected).read_bytes() == protected_bytes
    assert tree(root / packet['path']) == packet_bytes
    note('four-publisher-rounds-held-reader', mode=mode, real_losers=12, active_owners=2, terminal_bytes=registry(root)['terminal_bytes'], held_seconds=time.monotonic()-started, reclaimed_bytes=0)

root = seed('A', 'faults')
windows = ['terminal-before-receipt', 'terminal-receipt-stage-created', 'terminal-receipt-stage-before-write', 'terminal-receipt-stage-partial', 'terminal-receipt-stage-write', 'file-write', 'terminal-receipt-stage-staged', 'terminal-receipt-before-publish', 'terminal-receipt-published', 'terminal-receipt-durable', 'terminal-control-created', 'terminal-control-before-write', 'terminal-control-partial', 'terminal-control-write', 'terminal-control-staged', 'terminal-control-visible', 'terminal-control-durable', 'terminal-reply']
for checkpoint in windows:
    c = stale(root, 'crash-' + checkpoint)
    before = registry(root)
    release(root, c, code=86, env={'EXP_EXIT': checkpoint})
    middle = registry(root)
    committed = checkpoint in ['terminal-control-visible', 'terminal-control-durable', 'terminal-reply']
    assert (attempt(c) not in middle['active']) == committed
    assert middle['terminal_count'] == before['terminal_count'] + int(committed)
    fence = root / 'abandoned' / attempt(c)
    prepublish = windows.index(checkpoint) < windows.index('terminal-receipt-published')
    if prepublish:
        assert not fence.exists(), (checkpoint, 'partial final fence must never exist')
    if fence.exists() and not committed:
        unchanged_refusal(root, 'abandon', attempt=attempt(c))
        # A partial owned stage is never authority; retry rewrites the SAME file.
        if checkpoint in ['terminal-control-write', 'terminal-control-staged']:
            (root / f'attempts/{attempt(c)}/terminal-control-stage').write_bytes(b'partial owned control')
    out = release(root, c)['result']
    assert out['status'] == ('terminal-already-charged' if committed else 'terminal-charged')
    assert registry(root)['terminal_count'] == before['terminal_count'] + 1
    assert not (root / f'attempts/{attempt(c)}/terminal-control-stage').exists()
    assert not (root / f'attempts/{attempt(c)}/terminal-receipt-stage').exists()
    note('handoff-crash-window', checkpoint=checkpoint, committed_before_retry=committed, reclaimed_bytes=0)

# More than eight DISTINCT genuine partial-stage deaths must not consume the
# eight live admission slots permanently. Keep every raw failed prefix.
for n in range(10):
    c = stale(root, f'partial-owner-{n}')
    before = registry(root)
    point = 'terminal-receipt-stage-created' if n % 2 == 0 else 'terminal-receipt-stage-partial'
    release(root, c, code=86, env={'EXP_EXIT': point})
    assert not (root / 'abandoned' / attempt(c)).exists()
    assert registry(root)['active'][attempt(c)] == before['active'][attempt(c)]
    assert release(root, c)['result']['status'] == 'terminal-charged'
    assert registry(root)['terminal_count'] == before['terminal_count'] + 1
note('ten-distinct-partial-stage-owners-recover', reclaimed_bytes=0)

# Two preproofs see the same tokens and no final receipt. First writer dies;
# second compares actual final bytes or defers an unexpectedly appeared stage.
for point in ['terminal-receipt-stage-created', 'terminal-receipt-stage-partial', 'terminal-receipt-published', 'terminal-control-partial']:
    c = stale(root, 'same-preproof-' + point)
    before = registry(root)
    done = held(root, 'same-preproof-' + point, 'terminal-proved', 'lifecycle', operation='release-definitive-conflict', attempt=attempt(c))
    release(root, c, code=86, env={'EXP_EXIT': point})
    out = done()['result']
    assert out['status'] in ['deferred', 'terminal-charged'], out
    if out['status'] == 'deferred':
        assert release(root, c)['result']['status'] == 'terminal-charged'
    assert registry(root)['terminal_count'] == before['terminal_count'] + 1
    raw = (root / 'RESERVATIONS.json').read_bytes()
    assert release(root, c)['result']['status'] == 'terminal-already-charged'
    assert (root / 'RESERVATIONS.json').read_bytes() == raw
    note('concurrent-preproof-death-recovery', checkpoint=point, second=out['status'], reclaimed_bytes=0)

# Hardlinks are fault injection ONLY in new tiny disposable fixtures. Both
# names stay charged/retained; none is normal representation/backup/transport.
for endpoint in ['receipt-stage', 'control-stage', 'final-receipt']:
    fault = seed('A', 'link-' + endpoint)
    c = stale(fault, 'link-' + endpoint)
    if endpoint == 'control-stage':
        release(fault, c, code=86, env={'EXP_EXIT': 'terminal-receipt-published'})
    name = ('abandoned/' + attempt(c)) if endpoint == 'final-receipt' else f'attempts/{attempt(c)}/terminal-{endpoint}'
    alias = fault / name
    alias.parent.mkdir(parents=True, exist_ok=True)
    os.link(fault / 'SELECTED.json', alias)
    selected = (fault / 'SELECTED.json').read_bytes()
    raw = (fault / 'RESERVATIONS.json').read_bytes()
    out = unchanged_refusal(fault, 'release-definitive-conflict', attempt=attempt(c))
    assert 'terminal-file-links' in out['error'], out
    assert (fault / 'SELECTED.json').read_bytes() == selected
    assert (fault / 'RESERVATIONS.json').read_bytes() == raw
    note('hardlink-endpoint-refusal', endpoint=endpoint, both_aliases_fully_charged=True, selected_unchanged=True)

unknown = seed('A', 'unknown-slot')
done = held(unknown, 'unknown-slot', 'semantic-root', 'prepare', code=2, request=request('unknown-slot', D.head(unknown)))
active = registry(unknown)['active']
assert len(active) == 1
owner = next(iter(active))
slot = unknown / f'attempts/{owner}/terminal-receipt-stage'
slot.parent.mkdir(parents=True, exist_ok=True)
slot.write_bytes(b'unknown before preselection claim')
out = done()
assert out['error'] == 'refused:preexisting-terminal-stage-at-seal'
assert registry(unknown)['active'][owner]['state'] == 'preparing'
assert slot.read_bytes() == b'unknown before preselection claim'
note('unknown-slot-refuses-before-active-claim', reclaimed_bytes=0)

# Structural partial-stage injection after a REAL durable preselection claim;
# an actual successful selection must retain it under accepted allocation owner.
owned = seed('A', 'prefence-selection')
c = prepare(owned, 'prefence-selection')
reservation = registry(owned)['active'][attempt(c)]
slot = owned / f'attempts/{attempt(c)}/terminal-receipt-stage'
slot.write_bytes(b'owned structural partial stage')
out = D.call(owned, 'select', candidate=c)['result']
assert out['status'] == 'published'
publication = json.loads((owned / f"publications/{out['selected']['publication']}.json").read_bytes())
owner = publication['allocation_owner']
assert owner['attempt'] == attempt(c) and owner['conservative_bytes'] == reservation['bytes']
assert owner['includes_intermediates_and_stages'] is True
assert slot.read_bytes() == b'owned structural partial stage'
assert attempt(c) not in registry(owned)['active']
assert D.call(owned, 'resolve', request_id='prefence-selection')['result'] == out['result']
unchanged_refusal(owned, 'release-definitive-conflict', attempt=attempt(c))
note('prefence-stage-selected-ownership', structural_stage_injection=True, real_native_selection=True, reclaimed_bytes=0)

c = stale(root, 'uncertain-reply')
out = release(root, c, code=2, env={'EXP_ERROR': 'terminal-control-visible'})
assert 'injected-io:terminal-control-visible' in out['error']
assert release(root, c)['result']['status'] == 'terminal-already-charged'

# Every grant path shares selection_grant: the durable marker must precede all
# acceptance allocation/result/publication/hint/selector installs in trace order.
for action in ['select', 'publish', 'checkpoint']:
    env = dict(EXP_TRACE='1', EXP_EXIT='selection-started-durable')
    if action == 'select':
        c = prepare(root, 'grant-file')
        out = D.call(root, 'select', candidate=c, code=86, env=env)
    elif action == 'publish':
        out = D.publish(root, 'grant-prepared', code=86, env=env)
    else:
        out = D.call(root, 'checkpoint', request_id='grant-checkpoint', code=86, env=env)
    trace = D.last['stderr']
    assert 'checkpoint:selection-started-durable' in trace
    assert all('checkpoint:' + x not in trace for x in ['membership-root', 'outcome-root', 'result-installed', 'publication-installed', 'attempt-hint', 'selector-staged'])
    started = [a for a, r in registry(root)['active'].items() if r['state'] == 'selection-started']
    assert started
    for a in started:
        unchanged_refusal(root, 'release-definitive-conflict', attempt=a)
    note('selection-grant-before-installs', action=action, active_started=started)

# A genuine lost semantic reply is accepted, not a definitive loser.
c = prepare(root, 'accepted-lost-reply')
D.call(root, 'select', candidate=c, code=86, env={'EXP_EXIT': 'selector-visible'})
accepted = D.call(root, 'resolve', request_id='accepted-lost-reply')['result']
unchanged_refusal(root, 'release-definitive-conflict', attempt=attempt(c))
assert D.call(root, 'select', candidate=c)['result'] == accepted
# Another attempt accepting the same request (including another intent) leaves
# the first owner's bytes charged; intent mismatch is never absence.
for differing_intent in [False, True]:
    name = 'other-accepted-' + str(differing_intent)
    first = D.call(root, 'prepare', request=request(name, None))['result']['candidate']
    second = D.call(root, 'prepare', request=request(name, D.head(root) if differing_intent else None))['result']['candidate']
    D.call(root, 'select', candidate=second)
    unchanged_refusal(root, 'release-definitive-conflict', attempt=attempt(first))
note('accepted-other-intent-and-semantic-lost-reply-refuse', reclaimed_bytes=0)

race = seed('A', 'races')
for changed in ['head', 'registry']:
    c = stale(race, 'race-' + changed)
    done = held(race, changed, 'terminal-proved', 'lifecycle', operation='release-definitive-conflict', attempt=attempt(c))
    if changed == 'head':
        D.publish(race, 'head-racer')
    else:
        another = prepare(race, 'registry-racer')
    before = tree(race)
    assert done()['result']['status'] == 'deferred'
    assert tree(race) == before
    release(race, c)
    if changed == 'registry':
        D.call(race, 'select', candidate=another)
    note('proof-token-race-defers', token=changed)

D.call(race, 'packing', operation='build', bootstrap=True, force=True, codec='raw', cleanup=False)
c = stale(race, 'race-physical')
done = held(race, 'physical', 'terminal-proved', 'lifecycle', operation='release-definitive-conflict', attempt=attempt(c))
D.call(race, 'packing', operation='build', force=True, codec='raw', cleanup=False)
before = tree(race)
assert done()['result']['status'] == 'deferred' and tree(race) == before
release(race, c)
note('proof-token-race-defers', token='physical')

c = stale(race, 'invalid-controls')
path = race / 'RESERVATIONS.json'
original = path.read_bytes()
for case in ['used-margin', 'count-cap', 'overflow']:
    value = json.loads(original)
    if case == 'used-margin':
        row = value['active'][attempt(c)]
        row['used'] = row['bytes'] - 131072 + 1
    elif case == 'count-cap':
        value['terminal_count'] = 4096
        value['terminal_bytes'] = 4096 * 196608
    else:
        value['terminal_bytes'] = 18446744073709551615
    path.write_text(json.dumps(value, indent=2))
    unchanged_refusal(race, 'release-definitive-conflict', attempt=attempt(c))
    path.write_bytes(original)
    note('structural-quota-refusal', reason=case, authentic_4096_attempts=False)

candidate = race / c
data = candidate.read_bytes()
value = json.loads(data)
value['request']['request'] += '-corrupt'
candidate.write_text(json.dumps(value, indent=2))
unchanged_refusal(race, 'release-definitive-conflict', attempt=attempt(c))
candidate.write_bytes(data)
release(race, c, code=86, env={'EXP_EXIT': 'terminal-receipt-durable'})
receipt = race / 'abandoned' / attempt(c)
raw = receipt.read_bytes()
receipt.write_bytes(raw + b' ')
unchanged_refusal(race, 'release-definitive-conflict', attempt=attempt(c))
receipt.write_bytes(raw)
release(race, c)

# Touched mapped-index corruption must fail before a terminal owner is written.
c = stale(race, 'bad-index')
expected = json.loads((race / c).read_bytes())['expected']['publication']
config = json.loads((race / 'STORE.json').read_bytes())
catalog = json.loads((race / f"pack-catalogs/{config['catalog']}.json").read_bytes())
key = f'publications/{expected}.json'
mapped = [d for d in catalog['packs'] if d['role'] == 'publication' and d['first'] <= key <= d['last']]
if not mapped:
    # Advance maintenance so that this candidate's base becomes mapped.
    D.call(race, 'packing', operation='build', force=True, codec='raw', cleanup=False)
    config = json.loads((race / 'STORE.json').read_bytes())
    catalog = json.loads((race / f"pack-catalogs/{config['catalog']}.json").read_bytes())
    mapped = [d for d in catalog['packs'] if d['role'] == 'publication' and d['first'] <= key <= d['last']]
assert mapped
desc = mapped[0]
index = race / f"packs/{desc['instance']}/{desc['index']}.idx"
raw = index.read_bytes()
index.write_bytes(bytes([raw[0] ^ 1]) + raw[1:])
unchanged_refusal(race, 'release-definitive-conflict', attempt=attempt(c))
index.write_bytes(raw)
release(race, c)
unchanged_refusal(race, 'collect')
note('candidate-receipt-index-corruption-refuse', reclaimed_bytes=0)


# Genuine old journal/output survives process death; a new invocation cannot
# adopt its namespace. No injected birth ID or unrelated fixture mutation.
birth = seed('A', 'birth-restart')
D.call(birth, 'packing', operation='build', bootstrap=True, force=True, codec='raw', cleanup=False, code=86, env={'EXP_EXIT':'pack-index'})
old = json.loads((birth/'PACKING.json').read_bytes())
assert old['status'] == 'building'
old_files = {p: p.read_bytes() for p in (birth/'packs'/old['id']).iterdir() if p.is_file()}
assert old_files
before = tree(birth)
out = D.call(birth, 'packing', operation='build', bootstrap=True, force=True, codec='raw', cleanup=False, code=2)
assert out['error'] == 'refused:unfinished-maintenance-resume-or-abort' and tree(birth) == before
D.call(birth, 'packing', operation='abort')
D.call(birth, 'packing', operation='build', bootstrap=True, force=True, codec='raw', cleanup=False)
fresh = json.loads((birth/'PACKING.json').read_bytes())
assert fresh['id'] != old['id'] and all(p.read_bytes() == b for p,b in old_files.items())
note('actual-stale-journal-fresh-birth-no-readoption', old=old['id'], fresh=fresh['id'], old_files=len(old_files))

(EVIDENCE / f'{NAME}-results.json').write_text(json.dumps(dict(status='PASS', binary=D.binary_sha256, cases=ROWS), indent=2) + '\n')
print(json.dumps(dict(status='PASS', cases=len(ROWS), qualification='bounded source-checkpoint controls only; no native/long/full-matrix claim')))
