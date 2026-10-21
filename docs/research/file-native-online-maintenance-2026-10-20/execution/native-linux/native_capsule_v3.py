"""NEW closed capsule loader. Validation constructs no store and launches nothing.

An eventual parent-released manifest must be supplied by its exact SHA. No sample
manifest in this source packet has GO. Existing candidate dispatcher is immutable.
"""
import importlib.util
from pathlib import Path
import sys

import native_guard as guard
from native_driver import ACCEPTED_GUARD, owner
from native_driver_v3 import Driver
from native_adapter_v3 import NativeAdapter
from native_historical_v3 import HistoricalFactory
from native_resources_v3 import Reservations, confined, digest_bytes, need, regular_read, strict
from native_bootstrap_v3 import IMPLEMENTATION, ACCEPTED, SHARED_REQUIRED

SHARED = {'focused_protocol.py': '58928f6bf82f82d13fb5d809c8d6095e331ec4feeac48c389d72744a3ff32859',
          'focused_schedules.py': 'ee7d2aef5b7ecd492602c49f9e78646465bc2cd04a1e597e1d46e86cd67ff756',
          'case_ledger.py': 'ce7fb4983b5f31e0f529a69431a169574e7ba5cb55e225b17d42d10354be931b',
          'source_receipt.json': 'ae78d32a4a14d8327c3b4090fea3ea05f4f69b8542c0e3c281692ec7acc7b63a'}
need(SHARED == SHARED_REQUIRED, 'bootstrap-loader-shared-closure-equality')


class Capsule:
    def __init__(self, manifest_path, expected_sha256, *, bootstrap_verified=False):
        need(bootstrap_verified is True, 'stdlib-bootstrap-required-before-owned-imports')
        self.path = confined(guard.ROOT/'source', Path(manifest_path))
        raw = regular_read(self.path, 262144)
        need(digest_bytes(raw) == expected_sha256, 'parent-released-manifest-hash')
        self.manifest = strict(raw)
        m = self.manifest
        need(set(m) == {'suite_pin', 'guard_sha256', 'candidate', 'source_files', 'shared_directory',
                        'admission', 'version_map', 'historical_grant', 'extensions', 'implementation',
                        'implementation_directory', 'launch_binding', 'source_documents', 'entrypoint'}, 'closed-v3-manifest')
        need(isinstance(m['suite_pin'], str) and len(m['suite_pin']) == 40 and
             all(c in '0123456789abcdef' for c in m['suite_pin']), 'exact-final-pin')
        need(m['guard_sha256'] == ACCEPTED_GUARD, 'unchanged-accepted-guard')
        self.verify_sources()
        candidate = m['candidate']
        need(set(candidate) == {'path', 'sha256', 'source_pin', 'build_receipt'} and
             candidate['source_pin'] == m['suite_pin'], 'strict-final-candidate-source-binding')
        p = Path(candidate['path'])
        need(p.is_absolute() and p.resolve() == p and
             (p.is_relative_to(guard.ROOT/'source') or p.is_relative_to(guard.ROOT/'target')),
             'owned-candidate-path')
        need(guard.digest(p) == candidate['sha256'], 'candidate-binary-sha')
        build = self.document(candidate['build_receipt'])
        need(build['source_pin'] == candidate['source_pin'] and build['binary_sha256'] == candidate['sha256'] and
             build['parent_reviewed'] is True, 'reviewed-final-build-receipt')
        self.shared = confined(guard.ROOT/'source', guard.ROOT/'source'/m['shared_directory'])
        for filename, sha in SHARED.items():
            relative = (self.shared/filename).relative_to(guard.ROOT/'source').as_posix()
            need(relative in m['source_files'] and m['source_files'][relative]['sha256'] == sha,
                 'frozen-shared-source:'+filename)
        self.admission = self.document(m['admission'])
        self.versions = self.document(m['version_map'])
        need(set(self.versions) == {'final_pin', 'cases', 'checkpoints', 'crash_constructors', 'capabilities',
                                    'capabilities_reviewed'}, 'version-map-shape')
        need(self.versions['final_pin'] == m['suite_pin'], 'version-map-final-pin')
        self.expected_sha256 = expected_sha256

    def verify_sources(self):
        files = self.manifest['source_files']
        need(type(files) is dict and 1 <= len(files) <= 512, 'bounded-source-manifest')
        for relative, expected in files.items():
            need(set(expected) == {'bytes', 'sha256'}, 'source-entry-shape')
            p = confined(guard.ROOT/'source', guard.ROOT/'source'/relative)
            need(relative == p.relative_to(guard.ROOT/'source').as_posix(), 'canonical-source-entry')
            need(p.stat().st_size == expected['bytes'] and guard.digest(p) == expected['sha256'],
                 'source-file-changed:'+relative)
        folder = confined(guard.ROOT/'source', Path(self.manifest['implementation_directory']))
        need(folder == Path(__file__).resolve().parent and set(self.manifest['implementation']) == IMPLEMENTATION,
             'closed-actual-implementation-set')
        for filename in IMPLEMENTATION:
            p = confined(guard.ROOT/'source', folder/filename)
            relative = p.relative_to(guard.ROOT/'source').as_posix()
            need(self.manifest['implementation'][filename] == relative and relative in files,
                 'missing-actual-implementation:'+filename)
            if filename in ACCEPTED:
                need(files[relative]['sha256'] == ACCEPTED[filename], 'accepted-source-identity')
        for relative in self.manifest['source_documents']+[self.manifest['entrypoint']]:
            need(relative in files, 'missing-bound-document-or-entrypoint')
        for spec in self.manifest['extensions'].values():
            path = confined(guard.ROOT/'source', Path(spec['path']))
            relative = path.relative_to(guard.ROOT/'source').as_posix()
            need(relative in files and files[relative]['sha256'] == spec['sha256'], 'hook-in-source-set')
            need(all(name in files for name in spec['dependencies']+spec['source_documents']),
                 'hook-closure-and-documents')

    def verify_runtime_implementation(self):
        self.verify_sources()
        folder = Path(self.manifest['implementation_directory'])
        for filename in IMPLEMENTATION-{'native_bootstrap_v3.py'}:
            module = sys.modules.get(filename[:-3])
            need(module is not None and Path(module.__file__) == folder/filename,
                 'imported-module-canonical-path:'+filename)
        adapter_relative = (folder/'native_adapter_v3.py').relative_to(guard.ROOT/'source').as_posix()
        need(guard.digest(folder/'native_adapter_v3.py') == self.manifest['source_files'][adapter_relative]['sha256'],
             'adapter-sha-must-be-parent-bound')

    def document(self, relative):
        need(isinstance(relative, str) and relative in self.manifest['source_files'], 'manifest-bound-document')
        return strict(regular_read(confined(guard.ROOT/'source', guard.ROOT/'source'/relative), 262144))

    def construct(self, output, *, hooks=None):
        """Only under a live guard and separately released real/control admission."""
        need(guard.digest(self.path) == self.expected_sha256, 'capsule-changed')
        self.verify_sources()
        self.verify_runtime_implementation()
        m, candidate = self.manifest, self.manifest['candidate']
        claim = owner()
        budget = Reservations(claim, self.admission, expected_ledger_sha256=self.admission['ledger_sha256'])
        need(claim['source_pin'] == m['suite_pin'], 'capsule-versus-live-guard')
        need(m['launch_binding'] == dict(claim_id=claim['claim_id'], ledger_sha256=claim['ledger_sha256']),
             'bootstrap-manifest-versus-live-claim')
        binding = dict(final_pin=m['suite_pin'], case_versions=self.versions['cases'],
                       checkpoints=self.versions['checkpoints'], crash_constructors=self.versions['crash_constructors'],
                       capabilities=self.versions['capabilities'],
                       capabilities_reviewed=self.versions['capabilities_reviewed'],
                       source_manifest_sha256=self.expected_sha256,
                       source_hashes=[v['sha256'] for v in m['source_files'].values()],
                       adapter_sha256=guard.digest(Path(__file__).with_name('native_adapter_v3.py')),
                       hooks=m['extensions'])
        driver = Driver(candidate['path'], output, expected_sha256=candidate['sha256'], suite_pin=m['suite_pin'],
                        binary_source_pin=candidate['source_pin'], reservations=budget)
        adapter = NativeAdapter(driver, binding, hooks=hooks)
        historical = None
        if m['historical_grant'] is not None:
            name = m['historical_grant']
            need(name in m['source_files'], 'historical-companion-source-entry')
            historical = HistoricalFactory(guard.ROOT/'source'/name, expected_sha256=m['source_files'][name]['sha256'],
                                           suite_pin=m['suite_pin'], source_files=m['source_files'],
                                           reservations=budget, start_observer=adapter.started,
                                           register_driver=adapter.register_historical,
                                           validate_context=adapter.validate_historical_context)
        return driver, adapter, historical

    def shared_modules(self):
        """Load exact shared data/schedules, never alter a parser or oracle."""
        self.verify_sources()
        modules = {}
        for name in ('focused_protocol', 'focused_schedules', 'case_ledger'):
            need(name not in sys.modules, 'no-shared-module-substitution')
            spec = importlib.util.spec_from_file_location(name, self.shared/(name+'.py'))
            module = importlib.util.module_from_spec(spec)
            sys.modules[name] = module
            spec.loader.exec_module(module)
            modules[name] = module
        required = modules['case_ledger'].required_cases('Linux')
        need(set(self.versions['cases']) == set(required), 'ALL-required-case-IDs-must-have-version-rows')
        return modules
