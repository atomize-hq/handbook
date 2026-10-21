"""Generate STATIC source inventory/case crosswalk; no native runtime or controls.

Only AST compilation and case-ledger data construction occur. Imports from the
ledger's shared core are removed and replaced with a local invariant check. No
Driver/guard module is imported or invoked. All case observations remain PENDING.
"""
import ast
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
SHARED = Path('C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/'
              'docs/research/file-native-online-maintenance-2026-10-20/execution/final-protocol')
EXPECTED = {'focused_protocol.py': '58928f6bf82f82d13fb5d809c8d6095e331ec4feeac48c389d72744a3ff32859',
            'focused_schedules.py': 'ee7d2aef5b7ecd492602c49f9e78646465bc2cd04a1e597e1d46e86cd67ff756',
            'case_ledger.py': 'ce7fb4983b5f31e0f529a69431a169574e7ba5cb55e225b17d42d10354be931b',
            'source_receipt.json': 'ae78d32a4a14d8327c3b4090fea3ea05f4f69b8542c0e3c281692ec7acc7b63a'}
OLD = {'native_guard.py': '4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9',
       'native_driver.py': 'dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce',
       'native_capsule_adapter.py': '3bafbc8ac45de6ea2e4060d1a04c6fa5fc1ac278080e3ef1ffcececbd52481b7'}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def need(condition, message):
    if not condition:
        raise RuntimeError(message)


def write_new(name, value):
    path = HERE/name
    need(not (HERE/'native-source-receipt-v2.json').exists(), 'source-packet-already-sealed')
    need(not path.exists() or name in ('native-case-map-v2.json', 'native-static-validation-v2.json'),
         'immutable-source-packet-already-exists:'+name)
    path.write_bytes(encoded(name, value))


def encoded(name, value):
    # Compact derived case data preserves every row/obligation. Source/docs stay
    # readable. Compute ALL generated sizes before any write or transfer.
    return (json.dumps(value, indent=None if name == 'native-case-map-v2.json' else 2,
                       ensure_ascii=False, allow_nan=False)+'\n').encode()


def generate():
    for filename, expected in EXPECTED.items():
        need(sha(SHARED/filename) == expected, 'frozen-shared-source:'+filename)
    for filename, expected in OLD.items():
        need(sha(HERE/filename) == expected, 'accepted-revision-changed:'+filename)
    tree = ast.parse((SHARED/'case_ledger.py').read_text())
    tree.body = [node for node in tree.body if not isinstance(node, ast.ImportFrom)]
    namespace = {'require': need}
    exec(compile(tree, '<static-case-ledger-data>', 'exec'), namespace)
    rows = namespace['required_cases']('Linux')
    groups = {}
    for case_id, row in rows.items():
        groups[row['group']] = groups.get(row['group'], 0)+1
        row.update(final_pin=None, source_sha256=None,
                   status='PENDING', runtime_observations=[],
                   required_checks=[row['obligation'], 'exact final source/binary/guard/case mapping',
                                    'raw native event counts measured; no fixed267/282 assertion',
                                    'full independent final-version oracle at required boundary'])
        if row['group'] == 'legacy-packing':
            row['version_mapping'] = dict(status='PENDING reviewed final schema/checkpoint correspondence',
                malformed_offsets='shared reviewed v5 decoder/constructor only',
                build='explicit force=true for forced correctness; bootstrap=true only initial discovery',
                readers='legacy conversion gate separately; generation pins + two successors for final store',
                backup='ALL durable physical-pins/*.lock retained; only finite named global gates excluded')
        elif row['group'] == 'legacy-recovery':
            row['version_mapping'] = dict(status='PENDING final recovery/view/namespace correspondence',
                events='actual measured events, original282 historical only',
                restore='include durable pin locks; complete read-only oracle + accepted successor')
        elif row['group'] in ('strict', 'focused'):
            row['version_mapping'] = dict(status='PENDING final physical decoder and exact refusal mapping',
                parser='NO native parser fork; reviewed shared decoder required',
                controls='B loose publication controls remain permanent')
        else:
            row['version_mapping'] = dict(status='PENDING final engine source and native capability review',
                shared_core_sha256=EXPECTED['focused_protocol.py'],
                shared_schedules_sha256=EXPECTED['focused_schedules.py'])
    case_map = dict(status='SOURCE-ONLY ALL RUNTIME CASES PENDING', final_pin=None,
        shared=EXPECTED, groups=groups, required=len(rows), cases=rows,
        workspace='Five baseline identity slots PLUS ALL final discovered workspace tests; never stop at five',
        ordinary=dict(original_routes=17, immediate=['A/N16','A/N128','B/N16','B/N128'],
                      shell='native Bash, selected-once B; exact final route manifest required'),
        windows_only_skips=['loose-no-delete-sharing/A', 'loose-no-delete-sharing/B',
                            'B-control-no-delete-sharing', 'pack/index no-delete-sharing'],
        one_publisher_overlap='Parent confirmed58928f source is superseded for this gap; preserve strong proof')
    static = []
    for p in sorted(HERE.glob('*_v2.py')):
        parsed = ast.parse(p.read_text())
        compile(parsed, str(p), 'exec')  # In memory; no imports/pycache/execution.
        static.append(dict(file=p.name, ast_compiled=True,
                           classes=[n.name for n in parsed.body if isinstance(n, ast.ClassDef)],
                           imported_modules=[n.module for n in parsed.body if isinstance(n, ast.ImportFrom)]))
    validation = dict(kind='STATIC ONLY; not a runtime control',
        ast=static, accepted_sources_unchanged=OLD, shared_source_bindings=EXPECTED,
        no_native_copy=True, no_compiler=True, no_process_control=True, no_engine=True,
        gitnexus='UNAVAILABLE; index impact UNKNOWN/manual HIGH, new-only files/import trace',
        native_census=dict(source=dict(bytes=631232,files=61,directories=12),
                           target=dict(bytes=126797363,files=408,directories=90),
                           fixtures=dict(bytes=109,files=1,directories=5),
                           evidence=dict(bytes=4076854,files=177,directories=10)),
        unsealed_draft_format_failure=dict(bytes=273253, files=11, cap=262144,
             reason='Pretty derived crosswalk exceeded envelope before seal; compacted without removing rows',
             native_transfer=False, engine_or_control_run=False))
    names = ['native_resources_v2.py','native_driver_v2.py','native_historical_v2.py',
             'native_adapter_v2.py','native_capsule_v2.py','native_peer_v2.c','native_packet_v2.py',
             'native-case-map-v2.json','native-static-validation-v2.json',
             'native-source-review-v2.md','native-control-plan-v2.md',
             'native-source-receipt-v2-pre-review.json']
    generated = {'native-case-map-v2.json': encoded('native-case-map-v2.json', case_map),
                 'native-static-validation-v2.json': encoded('native-static-validation-v2.json', validation)}
    files = {}
    for name in names:
        raw = generated[name] if name in generated else (HERE/name).read_bytes()
        files[name] = dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
    size = sum(row['bytes'] for row in files.values())
    need(size+8192 <= 256*1024 and len(files)+1 <= 16, 'source-only-authorized-envelope')
    write_new('native-case-map-v2.json', case_map)
    write_new('native-static-validation-v2.json', validation)
    write_new('native-source-receipt-v2.json', dict(status='STATIC SOURCE PACKET FOR REVIEW; NO CONTROL GO',
        files=files, payload_bytes=size, payload_files=len(files), receipt_excluded_from_own_manifest=True,
        source_envelope=dict(bytes=262144,files=16), accepted_immutable=OLD,
        shared_api=EXPECTED, final_pin=None, live_controls_run=0, engine_calls_run=0, native_transfers=0))
    print(json.dumps(dict(bytes=size, files=len(files)+1, groups=groups, required_cases=len(rows))))


if __name__ == '__main__':
    generate()
