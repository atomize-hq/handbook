"""Closed historical companion grants; the suite pin never relabels an old ELF."""
import threading
from pathlib import Path

import native_guard as guard
from native_driver_v3 import Driver
from native_resources_v3 import canonical, confined, digest_bytes, need, regular_read, strict

# The ONLY historical native producer observed and authenticated in this lane.
# Adding a STORE3 producer requires a new parent-reviewed source revision/grant.
D70 = dict(path='/home/spenser/file-native-p-linux-20261020/bin/candidate-linux-final',
           sha256='a8501737ca4fdd5cfc177620860c1ae103d56f2e9b2151104a922df511a3d84b',
           source_pin='d70f3864f2888d46d9112213b4a03f8d82745a8e',
           source_authority_sha256='0c636670f89bd11086b139b966452d4f852e537d06997aa1f6649af5ad01d29a')


class HistoricalFactory:
    def __init__(self, grant_path, *, expected_sha256, suite_pin, source_files, reservations,
                 start_observer=None, register_driver=None, validate_context=None):
        path = confined(guard.ROOT/'source', Path(grant_path))
        relative = path.relative_to(guard.ROOT/'source').as_posix()
        need(relative in source_files and source_files[relative]['sha256'] == expected_sha256,
             'companion-grant-not-in-candidate-source-manifest')
        self.raw = regular_read(path, 65536)
        need(digest_bytes(self.raw) == expected_sha256, 'companion-grant-hash')
        self.grant = strict(self.raw, 65536)
        need(set(self.grant) == {'grant_id', 'suite_pin', 'producer', 'producer_receipt', 'invocations'},
             'closed-historical-grant')
        need(self.grant['suite_pin'] == suite_pin and self.grant['producer'] == D70,
             'explicit-d70-versus-final-source-binding')
        receipt = self.grant['producer_receipt']
        need(set(receipt) == {'path', 'sha256'}, 'producer-receipt-shape')
        proof_path = confined(guard.ROOT/'source', guard.ROOT/'source'/receipt['path'])
        need(receipt['path'] in source_files and source_files[receipt['path']]['sha256'] == receipt['sha256'],
             'historical-producer-receipt-source-manifest')
        proof = strict(regular_read(proof_path, 65536), 65536)
        need(digest_bytes(regular_read(proof_path, 65536)) == receipt['sha256'] and
             proof['producer'] == D70 and proof['reviewed_by_meta'] is True,
             'historical-producer-review-receipt')
        need(isinstance(self.grant['grant_id'], str) and self.grant['grant_id'] and
             1 <= len(self.grant['invocations']) <= 128, 'bounded-historical-invocations')
        self.path, self.expected, self.suite_pin = path, expected_sha256, suite_pin
        self.reservations, self.start_observer = reservations, start_observer
        self.register_driver = register_driver
        self.validate_context = validate_context
        self.lock, self.used = threading.Lock(), set()

    def create(self, case_id, phase, root, output):
        root = confined(guard.ROOT/'fixtures', Path(root))
        need(case_id in ('Compat-STORE1-STORE2-to-new', 'Compat-STORE3-to-final'), 'compat-case-only')
        need(self.validate_context is not None, 'historical-phase-context-required')
        self.validate_context(case_id, phase, root)
        entries = [row for row in self.grant['invocations'] if row['case_id'] == case_id and
                   row['phase'] == phase and row['root'] == str(root)]
        need(entries, 'unlisted-historical-case-phase-root')
        for row in entries:
            need(set(row) == {'invocation_id', 'case_id', 'phase', 'root', 'input_sha256', 'env',
                             'action', 'operation', 'expected_exit', 'role', 'expected_error'},
                 'closed-historical-invocation')
            expected_role = 'reader' if row['action'] in {'resolve', 'point', 'inspect', 'lookup-bench'} else 'checker'
            need(row['role'] == expected_role and row['expected_exit'] in (0, 2, 86), 'historical-action-role-exit')
            need(row['expected_exit'] != 2 or isinstance(row['expected_error'], str) and row['expected_error'],
                 'historical-refusal-must-be-exact')

        def policy(payload, env, code):
            self.validate_context(case_id, phase, root)
            need(guard.digest(self.path) == self.expected and guard.digest(Path(D70['path'])) == D70['sha256'],
                 'historical-grant-or-binary-changed')
            matches = [row for row in entries if row['input_sha256'] == digest_bytes(canonical(payload)) and
                       row['env'] == env and row['action'] == payload['action'] and
                       row['operation'] == payload.get('operation', '') and row['expected_exit'] == code]
            need(len(matches) == 1, 'unlisted-historical-input-env-action-exit')
            with self.lock:
                key = matches[0]['invocation_id']
                need(key not in self.used, 'historical-invocation-count-exhausted')
                self.used.add(key)
            return matches[0]

        driver = HistoricalDriver(D70['path'], output, expected_sha256=D70['sha256'], suite_pin=self.suite_pin,
                                binary_source_pin=D70['source_pin'], reservations=self.reservations,
                                historical_policy=policy, start_observer=self.start_observer,
                                grant_id=self.grant['grant_id'], case_id=case_id, phase=phase)
        need(self.register_driver is not None, 'historical-driver-must-join-suite-registry')
        self.register_driver(driver, case_id, phase, root)
        return driver


class HistoricalDriver(Driver):
    def __init__(self, *args, grant_id, case_id, phase, **kwargs):
        self.grant_id, self.case_id, self.phase = grant_id, case_id, phase
        super().__init__(*args, **kwargs)

    def append(self, row, state):
        super().append(dict(row, historical_grant_id=self.grant_id, historical_case_id=self.case_id,
                            historical_phase=self.phase), state)
