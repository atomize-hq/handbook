"""38 future controls; native RUN GO required. Toy reviews never qualify engines."""
import copy
import gzip
import hashlib
import json
import os
from pathlib import Path
import struct
import sys
import time

CASES = [
 ('C01-empty', 'positive', 'empty'),
 ('C02-chunk-minus', 'positive', 65535),
 ('C03-chunk', 'positive', 65536),
 ('C04-chunk-plus', 'positive', 65537),
 ('C05-magic', 'format', 'magic'),
 ('C06-version', 'format', 'version'),
 ('C07-type', 'format', 'type'),
 ('C08-duplicate-json', 'format', 'duplicate-json'),
 ('C09-file-sha', 'format', 'file-sha'),
 ('C10-truncated-gzip', 'format', 'truncated'),
 ('C11-trailing', 'format', 'trailing'),
 ('C12-concatenated', 'format', 'concatenated'),
 ('C13-hardlinks', 'topology', 'hardlinks'),
 ('C14-external-alias', 'topology', 'external-alias'),
 ('C15-dangling-link', 'topology', 'dangling-link'),
 ('C16-external-link', 'topology', 'external-link'),
 ('C17-fifo', 'topology', 'fifo'),
 ('C18-native-reparse-fork', 'native', 'reparse-fork'),
 ('C19-new-source-entry', 'mutation', 'new-entry'),
 ('C20-source-replaced', 'mutation', 'replace'),
 ('C21-content-changed', 'mutation', 'bytes'),
 ('C22-linked-ancestor', 'native', 'ancestor'),
 ('C23-inflight-mutation', 'native', 'inflight-mutation'),
 ('C24-missing-context', 'admission', 'context'),
 ('C25-stale-review', 'admission', 'review'),
 ('C26-protected-root', 'admission', 'protected'),
 ('C27-active-io', 'native', 'active-io'),
 ('C28-archive-cap', 'retirement', 'cap'),
 ('C29-exact-retirement', 'retirement', 'success'),
 ('C30-midway-stop', 'native', 'midway-stop'),
 ('C31-crash-journal', 'native', 'crash-journal'),
 ('C32-sharing', 'native', 'sharing'),
 ('C33-protected-bind', 'native', 'protected-bind-alias'),
 ('C34-special-hardlinks', 'native', 'special-alias-closure'),
 ('C35-negative-admission', 'native', 'negative-admission-before-import'),
 ('C36-review-substitution', 'native', 'same-buffer-review-substitution'),
 ('C37-foreign-posix', 'codec', 'posix-ff'),
 ('C38-foreign-utf16', 'codec', 'utf16-surrogate'),
]
CONTROL_POLICY_VERSION = 2
FORECAST = dict(policy_version=CONTROL_POLICY_VERSION, cells=38, control_only=True, engine_qualification=False,
    seconds_per_host=300, fixture_archive_restore_partial_bytes=12582912,
    fixture_archive_restore_partial_files=384, evidence_bytes=1048576, evidence_files=40,
    full_final_archive_allowance_admitted=False, source_execution_admitted=False)


def admitted(proof, expected):
    if not isinstance(proof, dict) or set(proof) != set(expected) | {'go', 'epoch'}:
        raise RuntimeError('NO_GO:control-admission-shape')
    if proof['go'] is not True or any(proof[k] != v for k, v in expected.items()):
        raise RuntimeError('NO_GO:control-admission-binding')
    if type(proof['epoch']) not in [int, float] or not 0 <= time.time() - proof['epoch'] <= 1:
        raise RuntimeError('NO_GO:control-admission-stale')


def load_archive(expected):
    import importlib.util
    path = Path(__file__).with_name('archive.py')
    with path.open('rb') as stream:
        raw = stream.read(57345)
    if len(raw) > 57344 or hashlib.sha256(raw).hexdigest() != expected:
        raise RuntimeError('control-source-binding')
    spec = importlib.util.spec_from_file_location('fixture_archive_' + expected[:12], path)
    module = importlib.util.module_from_spec(spec)
    exec(compile(raw, str(path), 'exec'), module.__dict__)  # Same hashed buffer; no reread/cache.
    return module


def file_sha(path):
    digest = hashlib.sha256()
    with open(path, 'rb') as source:
        for block in iter(lambda: source.read(65536), b''):
            digest.update(block)
    return digest.hexdigest()


def write_json(path, value):
    data = json.dumps(value, sort_keys=True, ensure_ascii=True, separators=(',', ':')).encode() + b'\n'
    with open(path, 'xb') as output:
        output.write(data)
        output.flush()
        os.fsync(output.fileno())
    return hashlib.sha256(data).hexdigest()


def inventory(root):
    result = {}
    for directory, dirs, files in os.walk(root, followlinks=False):
        for name in dirs + files:
            path = Path(directory) / name
            info = os.lstat(path)
            value = [info.st_mode, info.st_size, info.st_dev, info.st_ino, info.st_nlink]
            if path.is_symlink():
                value.append(os.readlink(path))
            elif path.is_file():
                value.append(file_sha(path))
            result[str(path.relative_to(root))] = value
    return result


def prepared(A, case, source_pin, guard_sha, deadline, scope_sha, context):
    root = case / 'source'
    root.mkdir()
    (root / 'empty-directory').mkdir()
    (root / 'sample.bin').write_bytes(b'forensic-control-bytes\n')
    review = dict(version=1, host=sys.platform, source_pin=source_pin,
        required_case_ids=['tiny-fixture-prepared'], completed_case_ids=['tiny-fixture-prepared'],
        complete=True, full_oracles=True, negative_receipts_complete=True,
        positive_native_closure=True, new_final_fixture=True, control_only=True)
    review_path = case / 'toy-control-review.json'
    review_sha = write_json(review_path, review)
    ref = dict(path=str(review_path), sha256=review_sha)
    with A.Node(root) as node:
        identity = node.meta['identity']
    plan = dict(version=1, operation='ARCHIVE', host=sys.platform, source_pin=source_pin,
        tool_sha256=file_sha(Path(A.__file__)), guard_sha256=guard_sha, native_scope_sha256=scope_sha,
        roots=[dict(path=str(root), identity=identity)], protected_roots=[str(case / 'protected')],
        artifact_root=str(case / 'archive'), limits=dict(logical_bytes=1048576,
            entries=128, stream_bytes=2097152, depth=16,
            deadline_epoch=deadline, archive_bytes=1048576),
        evidence={key: [ref] for key in ['host_review', 'case_ledger', 'oracles', 'negatives', 'quiescence']},
        new_final_fixture=True, namespace_freeze_nonce=case.name, control_only=True,
        archive_sha256=None)
    return plan


def refuse(A, callback, expected=None):
    try:
        callback()
    except A.NoGo as error:
        if expected is not None and expected not in str(error):
            raise AssertionError(('wrong-refusal', expected, str(error)))
        return str(error)
    raise AssertionError('missing-refusal')


def frames(A, raw):
    result, position = [], len(A.MAGIC)
    while position < len(raw):
        kind, length, payload = A.FRAME.unpack_from(raw, position)
        end = position + A.FRAME.size + length + payload
        result.append([kind, raw[position + A.FRAME.size:position + A.FRAME.size + length],
                       raw[position + A.FRAME.size + length:end]])
        position = end
    return result


def malformed(A, data, mode):
    if mode == 'truncated':
        return data[:-4]
    if mode == 'trailing':
        return data + b'!'
    if mode == 'concatenated':
        return data + gzip.compress(b'extra member', mtime=0)
    raw = gzip.decompress(data)
    if mode == 'magic':
        return gzip.compress(b'BADMAGIC' + raw[8:], mtime=0)
    parts = frames(A, raw)
    if mode == 'version':
        header = A.decode(parts[0][1])
        header['version'] = 99
        parts[0][1] = A.encode(header)
    elif mode == 'type':
        parts[1][0] = 255
    elif mode == 'duplicate-json':
        parts[0][1] = b'{"version":1,' + parts[0][1][1:]
    elif mode == 'file-sha':
        part = next(x for x in parts if x[0] == A.FILE and x[2])
        part[2] = bytes([part[2][0] ^ 1]) + part[2][1:]
    return gzip.compress(A.MAGIC + b''.join(A.FRAME.pack(k, len(m), len(p)) + m + p
                                          for k, m, p in parts), mtime=0)


def native_case(A, context, case_id, mode, plan_path, plan_sha, plan):
    if not hasattr(context, 'exercise_native_control'):
        return dict(status='PENDING_NATIVE_HOOK', passed=False, mode=mode)
    proof = context.exercise_native_control(case_id=case_id, mode=mode,
        archive_module=A, plan_path=str(plan_path), plan_sha256=plan_sha, plan=plan)
    needed = {'status', 'passed', 'mode', 'source_sha256', 'raw_receipts',
              'native_closure_proven', 'protected_unchanged', 'archive_retained'}
    if not isinstance(proof, dict) or set(proof) != needed:
        raise AssertionError('native-control-proof-shape')
    if proof['source_sha256'] != plan['tool_sha256'] or proof['mode'] != mode:
        raise AssertionError('native-control-source-binding')
    for ref in proof['raw_receipts']:
        if set(ref) != {'path', 'sha256'} or file_sha(ref['path']) != ref['sha256']:
            raise AssertionError('native-control-raw-sha')
    if type(proof['passed']) is not bool:
        raise AssertionError('native-pass-must-be-boolean')
    if not isinstance(proof['raw_receipts'], list) or len(proof['raw_receipts']) > 3:
        raise AssertionError('native-control-evidence-cap')
    if proof['passed'] and (proof['status'] != 'NATIVE_CONTROL_PASSED' or not proof['raw_receipts'] or not all(proof[k] is True for k in
        ['native_closure_proven', 'protected_unchanged', 'archive_retained'])):
        raise AssertionError('unsupported-native-pass')
    if mode == 'windows-fifo-applicability-v1':
        if os.name != 'nt' or hasattr(os, 'mkfifo') or case_id != 'C17-fifo':
            raise AssertionError('unsupported-platform')
        if (proof['status'] != 'WINDOWS_UNSUPPORTED_PRIMITIVE' or proof['passed'] is not False
            or len(proof['raw_receipts']) != 1 or not all(proof[k] is True for k in
            ['native_closure_proven', 'protected_unchanged', 'archive_retained'])):
            raise AssertionError('unsupported-proof')
        loaded = A.load_plan(plan_path, plan_sha, 'ARCHIVE')
        poll = lambda: A.check(loaded, context, 'controls-c17')
        poll()
        ref = proof['raw_receipts'][0]
        raw, sha = A.reference_read(ref['path'], 65536, poll)
        actual = A.decode(raw)
        before = actual.pop('before_sha256')
        after = actual.pop('after_sha256')
        expected = dict(policy_version=2, case_id=case_id, mode=mode, host=sys.platform,
            archive_sha256=plan['tool_sha256'], controls_sha256=file_sha(__file__),
            context_sha256=plan['native_scope_sha256'], guard_sha256=plan['guard_sha256'],
            source_pin=plan['source_pin'], plan_sha256=plan_sha, root=plan['roots'][0],
            nonce=plan['namespace_freeze_nonce'], scope_ref=plan['evidence']['quiescence'][0],
            scope='cpython-mkfifo-win32-disk-tree-v1', api='CreateNamedPipeW',
            path=str(Path(plan['roots'][0]['path']) / 'negative-fifo'),
            handle='INVALID_HANDLE_VALUE', winerror=123, os_mkfifo_absent=True)
        if (sha != ref['sha256'] or A.encode(actual) != A.encode(expected) or not A.is_sha(before) or before != after
            or type(actual['winerror']) is not int or type(actual['policy_version']) is not int
            or actual['os_mkfifo_absent'] is not True):
            raise AssertionError('unsupported-raw-proof')
        poll()
        proof = dict(proof, applicable=False, applicability_verified=True)
    return proof


def run(root, context, archive_sha256, source_pin, guard_sha256, deadline_epoch, scope_sha256):
    """Admitted native control batch."""
    start = time.time()
    if context is None or not hasattr(context, 'admit_controls'):
        raise RuntimeError('NO_GO:control-runtime-admission-required')
    deadline = min(deadline_epoch, start + 300, 1792581014)
    root = Path(os.path.abspath(root))
    if root.exists():
        raise RuntimeError('new-control-prefix-required')
    expected = dict(phase='batch', root=str(root), case_id='BATCH', deadline_epoch=deadline,
        archive_sha256=archive_sha256, source_pin=source_pin, guard_sha256=guard_sha256, scope_sha256=scope_sha256,
        forecast_sha256=hashlib.sha256(json.dumps(FORECAST, sort_keys=True).encode()).hexdigest(),
        control_only=True)
    proof = context.admit_controls(expected=dict(expected), forecast=dict(FORECAST))
    admitted(proof, expected)  # Before import.
    A = load_archive(archive_sha256)
    admitted(proof, expected)  # Fresh before first write.
    root.mkdir()
    rows = []
    for case_id, group, mode in CASES:
        if time.time() >= deadline:
            rows.append(dict(case=case_id, status='NOT_STARTED_DEADLINE', passed=False))
            continue
        case = root / case_id
        case_expected = dict(expected, phase='case', root=str(case), case_id=case_id,
            prospective_bytes=262144, prospective_files={'positive': 7, 'format': 7,
                'topology': 9, 'mutation': 8, 'admission': 5, 'retirement': 11,
                'native': 13, 'codec': 5}[group])
        admitted(context.before_control_write(expected=dict(case_expected)), case_expected)
        case.mkdir()
        try:
            original = prepared(A, case, source_pin, guard_sha256, deadline, scope_sha256, context)
            plan = context.prepare_control_plan(copy.deepcopy(original))
            refs = plan['evidence']['quiescence']
            original['evidence']['quiescence'] = refs
            if (A.encode(plan) != A.encode(original) or not isinstance(refs, list) or len(refs) != 1
                or set(refs[0]) != {'path', 'sha256'} or not A.is_sha(refs[0]['sha256'])):
                raise AssertionError('control-constructor-scope-only')
            source = case / 'source'
            if group == 'positive':
                if mode == 'empty':
                    (source / 'zero.bin').write_bytes(b'')
                    (source / 'sample.bin').write_bytes(b'')
                else:
                    (source / 'sample.bin').write_bytes(b'x' * mode)
            if group == 'topology':
                if mode == 'hardlinks':
                    os.link(source / 'sample.bin', source / 'alias.bin')
                elif mode == 'external-alias':
                    os.link(source / 'sample.bin', case / 'outside-alias.bin')
                elif mode in ['dangling-link', 'external-link']:
                    target = case / 'missing'
                    if mode == 'external-link':
                        target = case / 'outside-marker'
                        target.write_bytes(b'NEVER FOLLOW OR DELETE')
                    os.symlink(target, source / 'negative-link')
                elif mode == 'fifo':
                    if hasattr(os, 'mkfifo'):
                        os.mkfifo(source / 'negative-fifo')
                    else:
                        group, mode = 'native', 'windows-fifo-applicability-v1'
            if group == 'admission' and mode == 'protected':
                plan['protected_roots'] = [str(source)]
            if group == 'retirement' and mode == 'cap':
                plan['limits']['archive_bytes'] = 24
            plan_path = case / 'capture-plan.json'
            plan_sha = write_json(plan_path, plan)
            before = inventory(source)
            if group == 'codec':
                token, win = ('/w==', False) if mode == 'posix-ff' else ('ANg=', True)
                want = b'\xff' if not win else b'\x00\xd8'
                if A.name_bytes(token, win) != want:
                    raise AssertionError('foreign-name-bytes')
                display = A.display_name(token, win)
                codec, errors = ('utf-16-le', 'surrogatepass') if win else ('utf-8', 'surrogateescape')
                if display.encode(codec, errors) != want:
                    raise AssertionError('foreign-display-not-lossless')
                if win != (os.name == 'nt'):
                    refuse(A, lambda: A.unpack_name(token, win), 'foreign-native-path')
                rows.append(dict(case=case_id, status='CONTROL_PASSED', passed=True))
                continue
            if group == 'native':
                proof = native_case(A, context, case_id, mode, plan_path, plan_sha, plan)
                rows.append(dict(case=case_id, **proof))
                continue
            if group == 'admission':
                if mode == 'context':
                    message = refuse(A, lambda: A.archive(plan_path, plan_sha, None), 'no-reviewed-context')
                elif mode == 'protected':
                    message = refuse(A, lambda: A.archive(plan_path, plan_sha, context), 'protected-root')
                else:
                    review = Path(plan['evidence']['host_review'][0]['path'])
                    review.write_bytes(review.read_bytes() + b' ')
                    message = refuse(A, lambda: A.archive(plan_path, plan_sha, context), 'evidence-sha')
                if inventory(source) != before:
                    raise AssertionError('refused-source-changed')
                rows.append(dict(case=case_id, status='EXPECTED_REFUSAL', passed=True, refusal=message))
                continue
            if group == 'topology' and mode == 'external-alias' or group == 'retirement' and mode == 'cap':
                message = refuse(A, lambda: A.archive(plan_path, plan_sha, context))
                if inventory(source) != before:
                    raise AssertionError('failed-capture-source-changed')
                rows.append(dict(case=case_id, status='EXPECTED_REFUSAL', passed=True, refusal=message))
                continue
            proof = A.archive(plan_path, plan_sha, context)
            archive = case / 'archive/archive.gz'
            expected = file_sha(archive)
            if inventory(source) != before or proof['status'] != 'VERIFIED':
                raise AssertionError('capture-source-proof')
            if group == 'format':
                changed = malformed(A, archive.read_bytes(), mode)
                bad = case / 'bad.gz'
                bad.write_bytes(changed)
                loaded = A.load_plan(plan_path, plan_sha, 'ARCHIVE')
                message = refuse(A, lambda: list(A.archive_events(bad, file_sha(bad), loaded)))
                rows.append(dict(case=case_id, status='EXPECTED_REFUSAL', passed=True, refusal=message))
                continue
            if group == 'mutation':
                if mode == 'new-entry':
                    (source / 'new.bin').write_bytes(b'new')
                elif mode == 'replace':
                    os.rename(source / 'sample.bin', case / 'retained-original.bin')
                    (source / 'sample.bin').write_bytes(b'replacement')
                else:
                    (source / 'sample.bin').write_bytes(b'changed same inode')
                message = refuse(A, lambda: A.verify(plan_path, plan_sha, expected, context))
                rows.append(dict(case=case_id, status='EXPECTED_REFUSAL', passed=True, refusal=message))
                continue
            if group == 'retirement':
                permit = copy.deepcopy(plan)
                permit['operation'], permit['archive_sha256'] = 'RETIRE', expected
                permit_path = case / 'parent-retirement-permit.json'
                permit_sha = write_json(permit_path, permit)
                result = A.retire(permit_path, permit_sha, context)
                if result['status'] != 'RETIRED' or source.exists() or not archive.is_file():
                    raise AssertionError('authorized-tiny-retirement-failed')
            else:
                decoded = list(A.archive_events(archive, expected, A.load_plan(plan_path, plan_sha, 'ARCHIVE')))
                if proof['census']['logical_bytes'] and sum(len(meta) for kind, meta, _ in decoded if kind == A.DATA) < 1:
                    raise AssertionError('payload-not-retained')
            rows.append(dict(case=case_id, status='CONTROL_PASSED', passed=True,
                             archive_sha256=expected, source_only_engine_qualification=False))
        except BaseException as error:
            rows.append(dict(case=case_id, status='CONTROL_FAILED', passed=False, error=repr(error)[:1024]))
        finally:
            context.after_control(case_id=case_id, root=str(root), deadline_epoch=deadline)
    exact = sorted(r['case'] for r in rows) == sorted(c[0] for c in CASES)
    passed = sum(r['passed'] is True for r in rows)
    unsupported = [r for r in rows if r.get('applicability_verified') is True
        and r.get('applicable') is False and r['case'] == 'C17-fifo'
        and r['status'] == 'WINDOWS_UNSUPPORTED_PRIMITIVE' and r['passed'] is False]
    status = 'CONTROL_BATCH_PASSED' if exact and passed == 38 else 'CONTROL_BATCH_INCOMPLETE'
    if exact and os.name == 'nt' and passed == 37 and len(unsupported) == 1:
        status = 'CONTROL_BATCH_APPLICABLE_QUALIFIED'
    result = dict(status=status, policy_version=CONTROL_POLICY_VERSION,
                  passed_count=passed, unsupported_count=len(unsupported),
                  control_only=True, engine_qualification=False,
                  elapsed_seconds=time.time() - start, cells=rows, source_sha256=archive_sha256,
                  all_failure_prefixes_retained=True, no_automatic_cleanup=True)
    context.finalize_controls(result=result, root=str(root), deadline_epoch=deadline)
    return result
