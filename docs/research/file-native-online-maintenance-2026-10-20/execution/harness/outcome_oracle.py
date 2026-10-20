"""Independent exact-request intent proof; no engine decoder/serializer imports.

The existing full semantic walker separately proves selected membership/outcomes,
all retained graph indexes, sources and navigation. This closes the N16-only
history recipe helper's gap for the new authentic N128 fixtures.
"""
import hashlib
import json
from pathlib import Path
import re
import sys

PRIOR = Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def normalized(request):
    assert set(request) <= {'request', 'base', 'ops', 'packet'}
    assert {'request', 'base', 'ops'} <= set(request)
    base = request['base']
    if base is not None:
        assert set(base) == {'publication', 'digest'}
        base = dict(publication=base['publication'], digest=base['digest'])
    ops = []
    for op in request['ops']:
        assert set(op) <= {'kind', 'id', 'text', 'target', 'value'}
        assert {'kind', 'id'} <= set(op)
        value = op.get('value', 0)
        assert type(value) is int and 0 <= value < 2**64
        ops.append(dict(kind=op['kind'], id=op['id'], text=op.get('text', ''),
                        target=op.get('target', ''), value=value))
    packet = request.get('packet', True)
    assert type(packet) is bool
    return dict(request=request['request'], base=base, ops=ops, packet=packet)


def intent(request):
    return sha(json.dumps(normalized(request), indent=2, ensure_ascii=False).encode())


def verify(root, decoder=None, known_requests=None):
    root = Path(root)
    if decoder is None:
        sys.path.insert(0, str(PRIOR/'file-native-packing-2026-10-20'))
        from independent import Decoder
        decoder = Decoder(root)
    known_requests = known_requests or {}
    selector = json.loads((root/'SELECTED.json').read_bytes())
    initial = selector
    expected_sequence = None
    count = history = candidate_proofs = 0
    seen = set()
    origin = None
    evidence = hashlib.sha256()
    while selector is not None:
        assert count < 200001 and selector['publication'] not in seen
        seen.add(selector['publication'])
        p = json.loads(decoder.checked(root/f"publications/{selector['publication']}.json", selector['digest']))
        assert p['id'] == selector['publication']
        if expected_sequence is not None:
            assert p['sequence'] == expected_sequence
        request = known_requests.get(p['request'])
        if p['sequence'] == 0:
            assert p['request'] == 'initial' and p['predecessor'] is None
            request = dict(request='initial', base=None, ops=[], packet=False)
        match = re.fullmatch(r'history-([0-9]{6,})', p['request'])
        if match:
            i = int(match[1])
            assert i == p['sequence'] and i >= 1
            if origin is None:
                source = json.loads((root/'HISTORY-SOURCE.json').read_bytes())
                origin = decoder.checked(root/source['original'], source['digest'])
                assert origin
            variant = origin[:-1]+(b'\n' if origin[-1:] == b' ' else b' ')
            ops = []
            if i % 5:
                kind = {1: 'import', 2: 'label', 3: 'cost', 4: 'import'}[i % 5]
                text = ((variant if i % 5 == 1 else origin).decode() if i % 5 in [1, 4]
                        else f'history-{i%16}-{(i//80)%2}' if i % 5 == 2 else '')
                ops = [dict(kind=kind, id='d000000' if i % 5 in [1, 4] else f'd{i%16:06}',
                            text=text, target='', value=0)]
            request = dict(request=p['request'], base=p['predecessor'], ops=ops, packet=False)
            history += 1
        # Accepted candidate bytes are optional after legitimate retirement.
        # If unavailable, exact retained caller input or the explicit history
        # recipe above must establish the intent; never guess another request.
        try:
            candidate = json.loads(decoder.raw(f"attempts/{p['attempt']}/candidate.json"))
        except FileNotFoundError:
            candidate = None
        if candidate is not None:
            assert candidate['attempt'] == p['attempt'] and candidate['store_id'] == p['store_id']
            assert candidate['request']['request'] == p['request']
            assert candidate['graph'] == p['graph'] and candidate['projection'] == p['projection']
            if request is None:
                request = candidate['request']
            else:
                assert normalized(candidate['request']) == normalized(request)
            candidate_proofs += 1
        assert request is not None, 'missing-independent-request-intent-evidence:'+p['request']
        assert normalized(request)['request'] == p['request'] and intent(request) == p['intent'], 'request-intent-mismatch'
        result = json.loads(decoder.checked(root/f"results/{p['result_hash']}.json", p['result_hash']))
        assert result['request'] == p['request'] and result['publication'] == p['id'] and result['outcome'] == 'accepted'
        evidence.update(json.dumps([selector, p['intent'], p['result_hash']], separators=(',', ':')).encode()+b'\n')
        count += 1
        expected_sequence = p['sequence']-1
        selector = p['predecessor']
    assert expected_sequence == -1
    return dict(profile='independent-complete-exact-request-intent-and-accepted-result',
                selected=initial, publications=count, actual_H=count,
                authentic_history_recipe_intents=history, retained_candidate_proofs=candidate_proofs,
                evidence_sha256=evidence.hexdigest(), engine_code_imported=False)
