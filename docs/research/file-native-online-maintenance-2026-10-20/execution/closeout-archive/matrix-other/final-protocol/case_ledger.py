"""Required-case/reuse ledger. Creating a row never marks a case passed."""
import copy
from focused_protocol import require

MALFORMED = (
    'missing-catalog', 'corrupt-catalog', 'missing-index', 'corrupt-index',
    'missing-pack', 'truncated-pack', 'corrupt-record', 'offset-overflow',
    'raw-length-bound', 'duplicate-key', 'omitted-entry', 'wrong-pack-binding',
    'raw-hash-mismatch', 'bad-codec', 'too-many-packs', 'duplicate-json-key',
)
PACKING = ['operational-A', 'operational-B', 'orphan-unregistered-backup-restore',
           'unselected-installed-backup-restore', *MALFORMED]
for mode in ('A', 'B'):
    for phase, nth in [(p, 1) for p in ('pack-reservation', 'pack-build', 'pack-index',
            'pack-install', 'pack-before-catalog', 'pack-catalog', 'pack-accounting')]+[
            ('pack-loose-delete', 1), ('pack-loose-delete', 3)]:
        PACKING.append(f'interruption-{mode}-{phase}-{nth}')
    for phase in ('pack-read-lease', 'pack-catalog-captured', 'resolve-history-captured', 'pack-record-located'):
        PACKING.append(f'protected-read-{mode}-{phase}')
    PACKING += [f'writer-and-competing-maintenance-{mode}',
                f'cancelled-drain-releases-barriers-{mode}',
                f'repack-old-pack-deletion-interruption-{mode}']
require(len(PACKING) == 52 and len(set(PACKING)) == 52, 'packing-ID-crosswalk-count')

RECOVERY = [f'navigation-and-operational-backup/{m}/N{n}' for m in ('A', 'B') for n in (16, 128)]
RECOVERY += [f'namespace-interruption-correcting-successor/{direction}/{phase}'
             for direction, phases in (
                 ('up', ('refresh-plan', 'refresh-preserved', 'refresh-file', 'refresh-namespace',
                         'refresh-before-complete', 'refresh-complete', 'refresh-namespace-preserved')),
                 ('down', ('refresh-plan', 'refresh-preserved', 'refresh-file', 'refresh-namespace',
                           'refresh-before-complete', 'refresh-complete')))
             for phase in phases]
RECOVERY += [f'{name}/{m}' for m in ('A', 'B') for name in (
    'checkpoint-transfer-retire-repeat-gc', 'half-preparation-backup-recovery')]
RECOVERY += ['impossible-final-namespace', 'skipped-or-selected-once/A', 'skipped-or-selected-once/B']
require(len(RECOVERY) == 24, 'recovery-ID-crosswalk-count')
STRICT = ['independent-'+name for name in MALFORMED]
STRICT += ['strict-numeric-type-'+name for name in ('version', 'records', 'pack_bytes', 'index_bytes', 'codec')]
STRICT += ['independent-oversize-catalog', 'independent-oversize-index',
           'physical-catalog-and-hint-cannot-grant-membership']
require(len(STRICT) == 24, 'strict-ID-crosswalk-count')
FOCUSED = [
    'historical-selector-captured-before-publish-and-cleanup-exact-copy',
    'retained-controls-have-no-deletion-credit', 'historical-generation-discovery-after-repack',
    'concurrent-semantic-writer-preserves-frozen-and-new-current-discovery',
    'retirement-removes-content-but-keeps-permanent-B-publication-control',
    'missing-control-fails-closed', 'conflicting-control-fails-closed',
    'A-cleanup-retains-no-publication-control-duplicates',
]

# implementation means source exists; status always initially PENDING. Reuse is
# an extension of the identical adversarial obligation, never a skipped case.
NEW = [
    ('S-A-zstd', 'Harness.small', 'one-new-A-N64-H114', 'three cycles+coupled capture; full default payload'),
    ('S-B-zstd', 'Harness.small', 'ONE-new-B-N64-H114', 'one deep B lineage; no deep clone/restore'),
    ('C-P0-small', 'Harness.pack + oracle', 'existing-shallow-donor-A/B', 'small matched P0; do not rerun deep B'),
    ('D-partial-fixed-cutoff', 'partial_interval', 'new-shallow-A-N10-interval', 'cancel/unselected crash/fixed target/holes'),
    ('D-reservation-late-eligible', 'reservation_late', 'new-shallow-A-N10-reservation', 'genuine transfer; mask24; late retry'),
    ('D-cutoff-Q-stale-CAS', 'cutoff_and_stale_cas', 'new-shallow-A-N10-cutoff', 'semantic cutoff vs metadata physical CAS; quota'),
    ('F1-terminal-retired-bootstrap-delta', 'terminal_retirement', 'new-shallow-A-N10-retired', '>128 terminal retired bootstrap and delta'),
    ('F2-auto-decline-later-admit', 'automatic_policy', 'new-shallow-A-N10-auto', 'early/owned/H151 decline -> boundedH551 ordinary auto success or explicit incomplete; no repair'),
    ('D-stable-base-segment-pressure', 'stable_base', 'new-shallow-A-N10-stable', 'three deltas; reduced segment3; stable base retained'),
    ('S-persistent-owners', 'persistent_owners', 'operational-A/B-donor', 'preparation, packet, retained snapshot; retired refusal'),
    ('S-two-maintainers-semantic-crossing', 'contenders', 'writer-and-competing-maintenance-A/B', 'source-bound barrier; semantic advance'),
    ('S-reader-error', 'reader_end', 'protected-read-A/B-donor', 'EXP_ERROR after capture; positive native closure'),
    ('S-reader-cancel', 'native adapter required', 'protected-read-A/B-donor', 'cancel dependent I/O before pin release'),
    ('S-reader-death', 'native adapter required', 'protected-read-A/B-donor', 'exact process identity; durable owners persist'),
    ('S-maintainer-death', 'native adapter required', 'interruption-A/B-donor', 'native death around selection and journal'),
    ('S-path-reuse-fresh-birth', 'native adapter required', 'repack-old-pack-deletion-A/B', 'same bytes fresh instance; stale old journal cannot delete live birth'),
    ('D-terminal-oversize', 'source-bound constructor required', 'operational-A-oversize-donor', 'immutable terminal exclusion; no mutation to later eligibility'),
    ('D-deferred128-and-state32KiB', 'source-bound constructor required', 'pressure-A-donor', '128/129 and32768/32769 bytes; genuine reservation holes; later release'),
    ('D-segment4-and-collateral64MiB', 'source-bound constructor required', 'pressure-A-donor', '4 relevant segments, 5th consolidate/backpressure; 64MiB bound'),
    ('Q-raw8MiB', 'source-bound constructor required', 'oversize-A-donor', 'exact below/at/above raw boundary; no compact payload'),
    ('Q-pack64MiB-index16MiB-record65536', 'source-bound constructor required', 'limit-A-donor', 'authentic reduced-limit plus exact admission boundaries'),
    ('Q-catalog64-Work128KiB-cache64MiB', 'source-bound constructor required', 'strict-and-workspace-donor', '64/65 descriptors;128KiB Work; cache incl transients'),
    ('CP3-six-outcome-categories', 'paged adapter required', 'existing-packed-A/B-donor', 'early/mid/recent/missing/cancelled/lost reply'),
    ('CP3-authenticated-negative-all-segments', 'paged adapter required', 'malformed-seed-A-donor', 'route and leaf boundary/order absence across EVERY relevant segment'),
    ('CP3-touched-route-leaf-raw', 'paged adapter required', 'malformed-A-donor', 'required route/leaf/raw corruption refuses; loose duplicate cannot mask'),
    ('CP3-untouched-full-audit', 'paged adapter required', 'malformed-A-donor', 'point lookup may defer untouched damage; full audit finds it'),
    ('CP3-duplicates-segments', 'paged adapter required', 'malformed-A-donor', 'equal raw agrees; conflicting identity in any relevant segment refuses'),
    ('CP3-candidate-full-verification', 'paged adapter required', 'unselected-installed-donor', 'all ordering/coverage/raw bytes verified before selection'),
    ('Compat-STORE1-STORE2-to-new', 'versioned adapter required', 'shallow-legacy-donor', 'one-time old-operation quiescence; no mixed safety inference'),
    ('Compat-STORE3-to-final', 'versioned adapter required', 'shallow-v3-donor', 'catalog2/3->final exact version; oldclient refusal; restore successor'),
    ('Backup-durable-pin-locks', 'backup adapter required', 'operational/recovery-donor', 'include ALL physical-pins/*.lock; finite named global-gate exclusions'),
    ('Backup-shallow-final-restore-successor', 'backup adapter required', 'operational/recovery-donor', 'extend existing shallow restore; full oracle; accepted successor'),
    ('Final-two-pass-drain', 'Harness.drain_two_passes', 'each-new-or-donor-lineage', 'exactly TWO cleanup calls after producers/blockers end; no hidden loop'),
]
for phase in ('pack-reservation', 'pack-install', 'pack-before-catalog', 'physical-before-selection',
              'physical-after-selection', 'pack-accounting', 'pack-loose-delete', 'pack-old-delete',
              'physical-after-owned-file-delete', 'physical-progress-plan-installed'):
    NEW.append(('S-crash-'+phase, 'crash_extension', 'existing-matching-interruption-A/B',
                'critical crash full oracle; actual deletion proof if deletion phase'))

EXTERNAL = [
    ('H1k-H10k-three-cycles-1and4', 'Matrix owner', 'F05', '1600 actual added pubs; no-maintenance controls+3 maintained cycles'),
    ('D-N64-N1k-AandB-payload-title', 'Matrix owner', 'F07', 'matched shallow maintenance/no-maintenance; default payloads'),
    ('L-H1k-H10k-exact-long', 'Matrix owner', 'F09/F10', 'six categories;20fresh/500warm minimum; complete long verification'),
    ('R-long-stream-restore', 'Matrix/stream owner', 'F11', 'complete durable pins, full independent restore, accepted successor'),
]


def required_cases(host):
    require(host in ('Windows', 'Linux', 'Mac'), 'native-host-required')
    rows = {}
    def add(case_id, implementation, reuse, obligation, group):
        require(case_id not in rows, 'duplicate-case-ID')
        rows[case_id] = dict(case_id=case_id, group=group, status='PENDING',
                             implementation=implementation, reuse=reuse, obligation=obligation)
    for name in PACKING:
        add('P/'+name, 'existing capsule + final version map', 'same shallow donor obligation',
            'retain exact legacy assertion; version/checkpoint crosswalk required', 'legacy-packing')
    for name in RECOVERY:
        add('R/'+name, 'existing recovery capsule', 'one existing shallow source/restore set',
            'complete final-pin recovery; actual events measured separately from old282', 'legacy-recovery')
    for name in STRICT:
        add('I/'+name, 'existing strict supplement + version map', 'one malformed donor lineage',
            'exact refusal/allocation/acceptance assertion; independent decoder', 'strict')
    for name in FOCUSED:
        add('F/'+name, 'existing focused capsule + version map', 'one shallow B/A donor',
            'ordinary B selects once, loose controls remain; no native parser substitution', 'focused')
    for name, implementation, reuse, obligation in NEW:
        add(name, implementation, reuse, obligation, 'new-focused')
    for index in range(5):
        add(f'Workspace/unit-slot-{index+1}', 'exact final cargo test manifest', 'one external cache unit root',
            'bind actual test name/binary; all five units required; no fabricated test names', 'workspace')
    add('Ordinary/all-routes', 'exact shell route manifest', 'existing immediate/ordinary roots',
        'all applicable original17 routes; immediate A/B N16/N128; retained B once', 'ordinary')
    if host == 'Windows':
        for mode in ('A', 'B'):
            add('P/windows-loose-no-delete-sharing-'+mode, 'native sharing adapter required',
                'matching shallow packing donor', 'hold/release/retry; actual removal credit only', 'legacy-packing')
        add('F/B-control-no-delete-sharing-remains-retained', 'native sharing adapter required',
            'existing shallow B donor', 'required publication control never removed', 'focused')
        add('Windows-pack-index-no-delete-sharing', 'native sharing adapter required',
            'operational-A/B oldcontainers', 'pack AND index sharing deferral/release/retry', 'new-focused')
        add('X-Windows1000', 'Harness.mapping_stress', 'ONE-new-A-N10-stress',
            '1000 actual new catalog/descriptor mappings+accepted tail; separate work CAS', 'activation')
        for row in EXTERNAL:
            add(*row, group='external-full-matrix')
    else:
        add('X-native-pressure', 'Harness.mapping_stress(native_focused=True,count<=64)',
            'existing small pressure donor or one A10 lineage',
            'modest focused pressure only; cannot claim Windows p99 qualification', 'activation')
    return rows


class CaseLedger:
    def __init__(self, host, final_pin):
        self.host, self.final_pin = host, final_pin
        self.rows = required_cases(host)
        self.observations = []

    def record(self, case_id, observation):
        require(case_id in self.rows, 'unknown-case-ID')
        require(observation.get('case_id') == case_id, 'observation-case-binding')
        require(observation.get('status') in {'PENDING', 'UNSUPPORTED', 'FAILED', 'CENSORED', 'PASSED'},
                'explicit-evidence-status')
        if observation['status'] == 'PASSED':
            require(observation.get('final_pin') == self.final_pin and observation.get('host') == self.host,
                    'final-pin-native-binding')
            require(observation.get('raw_sha256') and observation.get('binary_sha256')
                    and observation.get('case_source_sha256') and observation.get('oracle_receipts')
                    and observation.get('guard_receipt_sha256'), 'missing-authentic-evidence-bindings')
            require(observation.get('full_oracle') is True and observation.get('native') is True
                    and observation.get('processes_closed') is True and observation.get('source_verified') is True,
                    'not-native-complete-qualification')
            require(observation.get('version_mapping') and observation.get('required_checks_complete') is True,
                    'no-version-map-or-obligations-missing')
        else:
            require(observation.get('reason'), 'pending-failure-censor-reason')
        self.observations.append(copy.deepcopy(observation))
        self.rows[case_id]['status'] = observation['status']
        self.rows[case_id]['latest_observation'] = len(self.observations)-1

    def result(self):
        missing = [name for name, row in self.rows.items() if row['status'] != 'PASSED']
        return dict(status='INCOMPLETE' if missing else 'ALL_REQUIRED_CASE_EVIDENCE_PRESENT',
                    host=self.host, final_pin=self.final_pin, required=len(self.rows),
                    missing=missing, rows=copy.deepcopy(self.rows),
                    observations=copy.deepcopy(self.observations),
                    experiment_verdict='parent must separately assess gates, source review and all hosts')
