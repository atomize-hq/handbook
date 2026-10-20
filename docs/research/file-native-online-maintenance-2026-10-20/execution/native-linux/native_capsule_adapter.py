"""Explicit source-bound capsule dispatch; no decoder/oracle substitutions."""
import argparse
import json
from pathlib import Path
import runpy
import sys
import time
import types

import native_guard as guard
from native_driver import Driver, legacy_driver_module, owner, ACCEPTED_GUARD


def strict(data):
    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise ValueError('duplicate-manifest-key')
            result[key] = value
        return result
    assert len(data) <= 65536, 'bounded-adapter-manifest'
    return json.loads(data, object_pairs_hook=pairs)


def run(manifest_path, stage):
    manifest_path = Path(manifest_path).resolve()
    assert manifest_path.is_relative_to(guard.ROOT/'evidence'), 'owned-manifest'
    manifest = strict(manifest_path.read_bytes())
    assert set(manifest) == {'suite_pin', 'guard_sha256', 'executable', 'source_files', 'stages'}
    claim = owner()
    assert manifest['suite_pin'] == claim['source_pin']
    assert manifest['guard_sha256'] == ACCEPTED_GUARD
    executable = manifest['executable']
    assert set(executable) == {'path', 'sha256', 'source_pin'}
    assert executable['source_pin'] == manifest['suite_pin'], 'candidate-source-pin-equality'
    path = Path(executable['path']).resolve()
    assert path.is_relative_to(guard.ROOT/'target') or path.is_relative_to(guard.ROOT/'source')
    assert guard.digest(path) == executable['sha256']
    assert 1 <= len(manifest['source_files']) <= 256

    def verify_sources():
        for relative, expected in manifest['source_files'].items():
            source = (guard.ROOT/'source'/relative).resolve()
            assert source.is_relative_to(guard.ROOT/'source'), 'source-containment'
            assert set(expected) == {'bytes', 'sha256'}
            assert source.stat().st_size == expected['bytes'] and guard.digest(source) == expected['sha256'], relative

    verify_sources()
    spec = manifest['stages'][stage]
    assert set(spec) == {'script', 'argv', 'driver_interface', 'receipt', 'case_mapping'}
    script = (guard.ROOT/'source'/spec['script']).resolve()
    assert spec['script'] in manifest['source_files'], 'bound-capsule-script'
    assert script.is_relative_to(guard.ROOT/'source')
    assert spec['driver_interface'] in ('legacy', 'explicit', 'none')
    assert isinstance(spec['argv'], list) and all(isinstance(x, str) for x in spec['argv'])
    for argument in spec['argv']:
        assert '\0' not in argument and '..' not in Path(argument).parts
        if Path(argument).is_absolute():
            assert Path(argument).resolve().is_relative_to(guard.ROOT), 'owned-capsule-argument'
    assert spec['case_mapping'], 'explicit-applicable-case-map-required'
    receipt = Path(spec['receipt']).resolve()
    assert receipt.is_relative_to(guard.ROOT/'evidence') and not receipt.exists()
    receipt.parent.mkdir(parents=True, exist_ok=True)
    previous_driver = sys.modules.get('driver')
    previous_path, previous_argv = list(sys.path), sys.argv
    if spec['driver_interface'] == 'legacy':
        module = legacy_driver_module(path, expected_sha256=executable['sha256'], suite_pin=manifest['suite_pin'])
        sys.modules['driver'] = module
    elif spec['driver_interface'] == 'explicit':
        module = types.ModuleType('driver')

        class BoundDriver(Driver):
            def __init__(self, executable_path, output):
                assert Path(executable_path).resolve() == path, 'explicit-capsule-binary'
                super().__init__(path, output, expected_sha256=executable['sha256'],
                                 suite_pin=manifest['suite_pin'],
                                 binary_source_pin=executable['source_pin'])

        module.Driver = BoundDriver
        sys.modules['driver'] = module
    started = time.time()
    failure = None
    try:
        sys.path.insert(0, str(script.parent))
        sys.argv = [str(script), *spec['argv']]
        try:
            runpy.run_path(str(script), run_name='__main__')
        except SystemExit as e:
            if e.code not in (None, 0):
                raise
    except BaseException as e:
        failure = repr(e)
        raise
    finally:
        sys.path[:], sys.argv = previous_path, previous_argv
        if previous_driver is None:
            sys.modules.pop('driver', None)
        else:
            sys.modules['driver'] = previous_driver
        try:
            verify_sources()
            assert guard.digest(path) == executable['sha256']
        except BaseException as e:
            failure = failure or 'source-or-binary-after:'+repr(e)
            raise
        finally:
            row = dict(status='failed' if failure else 'passed', suite_pin=manifest['suite_pin'],
                       stage=stage, script=str(script), script_sha256=guard.digest(script),
                       manifest_sha256=guard.digest(manifest_path), executable=executable,
                       guard_sha256=ACCEPTED_GUARD, guard_nonce=claim['nonce'],
                       case_mapping=spec['case_mapping'], seconds=time.time()-started,
                       failure=failure, source_and_binary_verified_before_after=failure is None,
                       semantic_decoder_substituted=False, windows_controls_stubbed=False,
                       closure_authority='separate-outer-native-guard-receipt')
            receipt.write_text(json.dumps(row, indent=2)+'\n')
    return row


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--manifest', required=True)
    parser.add_argument('--stage', required=True)
    options = parser.parse_args()
    print(json.dumps(run(options.manifest, options.stage), indent=2))
