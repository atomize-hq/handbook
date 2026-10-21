"""Stdlib-only bootstrap; parent must verify THIS file before executing it.

Guard command/claim binds parent-sealed manifest SHA and bootstrap/loader SHA.
No owned implementation import occurs until the complete closure is verified.
No sandbox claim: standard library/runtime are separately bound native tools.
"""
import argparse
import hashlib
import importlib.abc
import importlib.machinery
import json
from pathlib import Path
import runpy
import sys

ROOT = Path('/home/spenser/file-native-online-linux-20261020/source')
IMPLEMENTATION = frozenset({'native_guard.py', 'native_driver.py', 'native_resources_v3.py',
                          'native_driver_v3.py', 'native_adapter_v3.py', 'native_historical_v3.py',
                          'native_capsule_v3.py', 'native_bootstrap_v3.py'})
ACCEPTED = {'native_guard.py': '4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9',
            'native_driver.py': 'dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce'}
SHARED_REQUIRED = {'focused_protocol.py': '58928f6bf82f82d13fb5d809c8d6095e331ec4feeac48c389d72744a3ff32859',
                   'focused_schedules.py': 'ee7d2aef5b7ecd492602c49f9e78646465bc2cd04a1e597e1d46e86cd67ff756',
                   'case_ledger.py': 'ce7fb4983b5f31e0f529a69431a169574e7ba5cb55e225b17d42d10354be931b',
                   'source_receipt.json': 'ae78d32a4a14d8327c3b4090fea3ea05f4f69b8542c0e3c281692ec7acc7b63a'}


def need(value, message):
    if not value:
        raise RuntimeError(message)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def owned(path):
    p = Path(path)
    need(p.is_absolute() and '..' not in p.parts and p.is_relative_to(ROOT) and p.resolve() == p,
         'bootstrap-owned-canonical-path')
    current = Path(p.anchor)
    for part in p.parts[1:]:
        current /= part
        need(not current.is_symlink(), 'bootstrap-symlink-component')
    return p


def parse(raw):
    need(len(raw) <= 262144, 'bootstrap-manifest-bound')
    def unique(items):
        result = {}
        for key, value in items:
            need(key not in result, 'bootstrap-duplicate-key')
            result[key] = value
        return result
    def invalid(value):
        raise ValueError('nonfinite-json:'+value)
    return json.loads(raw, object_pairs_hook=unique, parse_constant=invalid)


def verify_closure(manifest_path, expected_sha256, bootstrap_sha256, loader_sha256):
    """Read-only verifier also used by reviewed control variants before imports."""
    path = owned(manifest_path)
    need(sha(path) == expected_sha256, 'parent-manifest-SHA')
    manifest = parse(path.read_bytes())
    return verify_bound_manifest(manifest, bootstrap_sha256, loader_sha256)


def verify_bound_manifest(manifest, bootstrap_sha256, loader_sha256):
    """Same read-only closure gate; controls vary data in memory, not source."""
    folder = owned(Path(manifest['implementation_directory']))
    need(folder == Path(__file__).resolve().parent, 'actual-bootstrap-directory')
    source_files = manifest['source_files']
    need(type(source_files) is dict and 1 <= len(source_files) <= 512, 'bootstrap-source-count')
    need(set(manifest['implementation']) == IMPLEMENTATION, 'closed-implementation-set')
    all_paths = {}
    for relative, record in source_files.items():
        p = owned(ROOT/relative)
        need(relative == p.relative_to(ROOT).as_posix() and p not in all_paths, 'canonical-unique-source-entry')
        need(set(record) == {'bytes', 'sha256'} and p.stat().st_size == record['bytes'] and
             sha(p) == record['sha256'], 'bootstrap-source-binding:'+relative)
        all_paths[p] = record
    for filename in IMPLEMENTATION:
        p = owned(folder/filename)
        need(p in all_paths and manifest['implementation'][filename] == p.relative_to(ROOT).as_posix(),
             'missing-actual-implementation:'+filename)
        if filename in ACCEPTED:
            need(all_paths[p]['sha256'] == ACCEPTED[filename], 'accepted-implementation-changed')
    need(all_paths[folder/'native_bootstrap_v3.py']['sha256'] == bootstrap_sha256 and
         all_paths[folder/'native_capsule_v3.py']['sha256'] == loader_sha256, 'parent-bootstrap-loader-binding')
    shared = owned(ROOT/manifest['shared_directory'])
    for name, expected in SHARED_REQUIRED.items():
        p = shared/name
        need(p in all_paths and all_paths[p]['sha256'] == expected, 'missing-bound-shared-source:'+name)
    documents = manifest['source_documents']
    mandatory_docs = {manifest['candidate']['build_receipt'], manifest['admission'], manifest['version_map'],
                      (shared/'source_receipt.json').relative_to(ROOT).as_posix()}
    if manifest['historical_grant'] is not None:
        mandatory_docs.add(manifest['historical_grant'])
    need(mandatory_docs <= set(documents), 'missing-mandatory-source-documents')
    # Every hook's transitive owned code and review/source documents must be
    # declared. The import finder below checks actual imported bytes too.
    for spec in manifest['extensions'].values():
        for relative in spec['dependencies']+spec['source_documents']:
            need(owned(ROOT/relative) in all_paths, 'missing-hook-dependency-or-document')
        need(owned(Path(spec['path'])) in all_paths and
             all_paths[Path(spec['path'])]['sha256'] == spec['sha256'], 'missing-hook-source')
    for relative in manifest['source_documents']:
        need(owned(ROOT/relative) in all_paths, 'missing-source-document')
    entrypoint = owned(ROOT/manifest['entrypoint'])
    need(entrypoint in all_paths, 'unbound-entrypoint')
    return manifest, all_paths


class BoundOwnedImports(importlib.abc.MetaPathFinder):
    def __init__(self, paths):
        self.paths = paths

    def find_spec(self, fullname, path=None, target=None):
        spec = importlib.machinery.PathFinder.find_spec(fullname, path)
        if spec is not None and spec.origin and spec.origin not in ('built-in', 'frozen'):
            actual = Path(spec.origin).absolute()
            if actual.is_relative_to(ROOT):
                need(actual in self.paths and actual.resolve() == actual and
                     actual.stat().st_size == self.paths[actual]['bytes'] and
                     sha(actual) == self.paths[actual]['sha256'], 'unbound-owned-import:'+fullname)
        return spec


def main(args=None):
    parser = argparse.ArgumentParser()
    parser.add_argument('--manifest', required=True)
    parser.add_argument('--manifest-sha256', required=True)
    parser.add_argument('--bootstrap-sha256', required=True)
    parser.add_argument('--loader-sha256', required=True)
    parser.add_argument('--claim-id', required=True)
    parser.add_argument('--ledger-sha256', required=True)
    ns = parser.parse_args(args)
    need(sys.platform == 'linux' and sys.flags.isolated and sys.flags.no_site and __debug__,
         'isolated-native-bootstrap-required')
    manifest, paths = verify_closure(ns.manifest, ns.manifest_sha256, ns.bootstrap_sha256, ns.loader_sha256)
    need(manifest['launch_binding'] == dict(claim_id=ns.claim_id, ledger_sha256=ns.ledger_sha256),
         'parent-launch-claim-binding')
    for module in sys.modules.values():
        origin = getattr(module, '__file__', None)
        need(origin is None or not Path(origin).absolute().is_relative_to(ROOT), 'owned-import-before-bootstrap')
    sys.meta_path.insert(0, BoundOwnedImports(paths))
    sys.path.insert(0, manifest['implementation_directory'])
    from native_capsule_v3 import Capsule
    capsule = Capsule(ns.manifest, ns.manifest_sha256, bootstrap_verified=True)
    capsule.verify_runtime_implementation()
    # The bound entrypoint chooses only source-released controls/cases. No
    # independent unbound runner is injected via Python globals or sys.argv.
    runpy.run_path(str(ROOT/manifest['entrypoint']), run_name='__main__',
                   init_globals={'BOUND_CAPSULE': capsule})


if __name__ == '__main__':
    main()
