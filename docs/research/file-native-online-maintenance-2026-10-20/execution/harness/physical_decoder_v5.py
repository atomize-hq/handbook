"""Independent CP3 STORE3/catalog4/FNPIDX02 full quiescent audit.

CP2 metadata/intent logic derives from the independently implemented v4 oracle.
Page bytes are decoded by independent paged_index.py, never Rust engine code.
Legacy FNPIDX01 and catalog2/3 remain explicit, separately decoded alternatives.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import sys
import time
import struct
import physical_decoder as prior
import paged_index

SCHEMA_PIN = 'f9b762a1a0d352564542bb4850ad77ee14a7c6e4'
FORMAT_INTRODUCED = 'f9b762a1a0d352564542bb4850ad77ee14a7c6e4'
INSPECTED_PARENT = 'bd6dd9367acddaddcd7873c8043548502a170c3b'
WORK_LIMIT = 128*1024


def descriptor4(d, require_instance=False):
    assert type(d) is dict
    assert prior.DESC <= set(d) <= prior.DESC|{'instance','segment','page'}
    prior.descriptor({k:v for k,v in d.items() if k not in ['segment','page']}, require_instance)
    prior.unsigned(d.get('segment',0))
    if d.get('page') is not None:paged_index.root_descriptor(d)
    return d


def generation4(g):
    prior.shape(g,prior.GEN)
    assert type(g['id']) is str and prior.ID.fullmatch(g['id'])
    assert prior.unsigned(g['ordinal']) > 0 and type(g['catalog']) is str and prior.HASH.fullmatch(g['catalog'])
    assert g['plan'] is None or type(g['plan']) is str and prior.HASH.fullmatch(g['plan'])
    assert type(g['packs']) is list and len(g['packs']) <= 64
    for d in g['packs']:descriptor4(d)


def selector4(s):
    prior.shape(s,{'publication','digest'})
    assert type(s['publication']) is str and prior.ID.fullmatch(s['publication'])
    assert type(s['digest']) is str and prior.HASH.fullmatch(s['digest'])


def cutoff4(c):
    prior.shape(c,{'selected','sequence'})
    selector4(c['selected']);prior.unsigned(c['sequence'])


def discovery4(d):
    prior.shape(d,{'version','completed','interval','deferred','include_attempts'})
    assert type(d['version']) is int and d['version'] == 1 and type(d['include_attempts']) is bool
    if d['completed'] is not None:cutoff4(d['completed'])
    if d['interval'] is not None:
        i=d['interval'];prior.shape(i,{'target','next','expected_sequence'});cutoff4(i['target'])
        if i['next'] is not None:selector4(i['next'])
        assert prior.unsigned(i['expected_sequence']) <= i['target']['sequence']
        assert d['completed'] is None or d['completed']['sequence'] <= i['target']['sequence']
    assert type(d['deferred']) is list and len(d['deferred']) <= 128
    seen=set()
    for item in d['deferred']:
        prior.shape(item,{'publication','roles','reason'});cutoff4(item['publication'])
        assert 0 < prior.unsigned(item['roles'],31)
        assert item['reason'] in ['reservation','retired','oversize','partial']
        name=item['publication']['selected']['publication'];assert name not in seen;seen.add(name)
        assert d['include_attempts'] or item['roles'] & 24 == 0
    assert len(json.dumps(d,indent=2,ensure_ascii=False).encode()) <= 32*1024


def work2(value):
    assert type(value) is dict and type(value.get('version')) is int and value['version'] == 2
    required=prior.WORK|{'physical_parent','transition_parent','birth_claims'}
    assert required <= set(value) <= required|{'discovery'}
    assert type(value['store_id']) is str
    assert type(value['physical_parent']) is str and prior.HASH.fullmatch(value['physical_parent'])
    parent=value['transition_parent'];assert parent is None or type(parent) is str and prior.HASH.fullmatch(parent)
    claims=value['birth_claims'];assert type(claims) is list and len(claims) <= 64
    assert all(type(c) is str and prior.ID.fullmatch(c) for c in claims) and len(set(claims)) == len(claims)
    generation4(value['generation'])
    assert type(value['older']) is list and len(value['older']) < 64
    seen=set()
    for g in value['older']:
        generation4(g);assert g['ordinal'] < value['generation']['ordinal'] and g['id'] not in seen and g['plan'] is not None;seen.add(g['id'])
    for field in ['jobs','retired','garbage']:assert type(value[field]) is list and len(value[field]) <= 4096
    for j in value['jobs']:
        prior.shape(j,{'descriptor','frontier','position'});descriptor4(j['descriptor'])
        assert 0 < prior.unsigned(j['frontier']) <= value['generation']['ordinal']
        assert prior.unsigned(j['position'],2**32-1) <= j['descriptor']['records']
    for r in value['retired']:
        prior.shape(r,{'descriptor','pack_done','index_done'});descriptor4(r['descriptor'])
        assert type(r['pack_done']) is bool and type(r['index_done']) is bool
    for name in value['garbage']:metadata2(name)
    for field in ['removed_bytes','removed_files','reconciled_absent']:prior.unsigned(value[field])
    if value.get('discovery') is not None:discovery4(value['discovery'])


def resolve_provenance(chain, rows, known=None):
    from outcome_oracle import intent, normalized
    by_pair = {}
    for row in rows:
        request = row['request'];digest = intent(request)
        assert digest == row['intent']
        pair = (request['request'],digest)
        if pair in by_pair:assert normalized(by_pair[pair]) == normalized(request)
        else:by_pair[pair] = request
    resolved = dict(known or {})
    for _, publication in chain.values():
        pair = (publication['request'],publication['intent'])
        if pair in by_pair:resolved[publication['request']] = by_pair[pair]
    return resolved


def metadata2(name):
    if type(name) is str and name.startswith('packs/') and name.count('/') == 1:
        assert prior.ID.fullmatch(name.split('/')[1]), 'invalid-owned-pack-directory-task'
    elif type(name) is str and name.startswith('physical-progress/'):
        parts = name.split('/')
        assert len(parts) == 2
        assert any(parts[1].endswith(suffix) and prior.ID.fullmatch(parts[1][:-len(suffix)])
                   for suffix in ['.json', '.work-stage', '.selector-stage'])
    else:
        prior.metadata_path(name)


def config3(c):
    allowed = {'version', 'store_id', 'mode', 'consumer', 'fallback', 'navigation_version', 'catalog', 'physical'}
    assert type(c) is dict and set(c) <= allowed
    assert {'version', 'store_id', 'mode', 'consumer', 'fallback', 'catalog', 'physical'} <= set(c)
    assert type(c['version']) is int and c['version'] == 3 and type(c['store_id']) is str
    assert c['mode'] in ['A', 'B'] and c['consumer'] in ['project', 'research'] and type(c['fallback']) is bool
    if 'navigation_version' in c:
        assert type(c['navigation_version']) is int and c['navigation_version'] in [1, 2, 3, 4]
    prior.shape(c['physical'], {'id', 'ordinal', 'plan'})
    b = c['physical']
    assert type(b['id']) is str and prior.ID.fullmatch(b['id']) and prior.unsigned(b['ordinal']) > 0
    assert type(b['plan']) is str and prior.HASH.fullmatch(b['plan'])
    assert type(c['catalog']) is str and prior.HASH.fullmatch(c['catalog'])


class Decoder(prior.Decoder):
    def __init__(self, root, *, quiescent=False):
        self.candidate_context = False
        self.authority = self
        super().__init__(root, quiescent=quiescent)
        if self.config['version'] != 3:
            self.required = {'STORE.json'}
            self.physical_metadata = dict(legacy=True, schema_pin=SCHEMA_PIN,
                                          journal=self.journal(), required_paths=sorted(self.required))

    @classmethod
    def candidate(cls, root, c, authority=None):
        config3(c)
        assert authority is not None and not authority.candidate_context, 'captured-selected-authority-required'
        assert authority.authority is authority and Path(root).resolve() == authority.root.resolve(), 'foreign-candidate-authority'
        assert authority.config['store_id'] == c['store_id'], 'candidate-authority-store'
        reader = cls.__new__(cls)
        reader.candidate_context = True
        reader.authority = authority
        reader.root = Path(root)
        reader.config = c
        reader.entries = {}
        reader.descriptors = []
        reader.zstd = None
        reader.required = set()
        reader.pin(c['physical'], c['catalog'])
        cat = reader.catalog(c['catalog'])
        reader.descriptors = cat['packs']
        for d in reader.descriptors:
            reader.load_index(d)
        reader.physical_metadata = reader.selected_work(cat)
        return reader

    def catalog(self, h):
        cat = self.read_metadata('pack-catalogs/'+h+'.json',59*1024,h)
        prior.shape(cat,{'version','store_id','packs'})
        assert type(cat['version']) is int and cat['version'] in [2,3,4]
        assert cat['store_id'] == self.config['store_id']
        assert type(cat['packs']) is list and len(cat['packs']) <= 64
        previous = None
        indexes = set()
        segments = set()
        for d in cat['packs']:
            descriptor4(d,require_instance=cat['version'] == 2)
            segment = d.get('segment',0)
            assert cat['version'] >= 3 or segment == 0
            assert d.get('page') is None or cat['version'] == 4, 'paged-catalog-version'
            name = prior.container_path(d,'idx')
            assert name not in indexes,'duplicate-physical-index-instance'
            indexes.add(name);segments.add(segment)
            assert len(segments) <= 4,'catalog-segment-cap'
            order = (segment,d['role'],d['first'])
            if previous is not None:
                assert previous[0] < order,'catalog-segment-role-order'
                p = previous[1]
                assert p.get('segment',0) != segment or p['role'] != d['role'] or p['last'] < d['first'],'within-segment-overlap'
            previous = (order,d)
            for ext,size in [('pack',d['pack_bytes']),('idx',d['index_bytes'])]:
                name = prior.container_path(d,ext)
                self.required.add(name)
                assert (self.root/name).stat().st_size == size
        return cat

    def load_index(self, d):
        if d.get('page') is not None:
            data = prior.legacy.bounded(self.root/prior.container_path(d,'idx'),d['index_bytes'],d['index_bytes'])
            audited = paged_index.full(data,d)
            if not hasattr(self,'variants'):self.variants = {}
            if not hasattr(self,'paged_audits'):self.paged_audits = []
            self.paged_audits.append(dict(path=prior.container_path(d,'idx'),index_sha256=d['index'],
                root_page_sha256=d['page']['hash'],height=d['page']['height'],pages=audited['pages'],
                index_bytes=audited['index_bytes'],records=len(audited['entries']),
                exact_whole_index_preorder_tree_authenticated=True))
            for key,offset,encoded,raw,digest in audited['entries']:
                entry = (d,offset,encoded,raw,digest)
                if key in self.entries:
                    assert self.entries[key][3:] == entry[3:], 'segment-conflicting-identity'
                else:self.entries[key] = entry
                self.variants.setdefault(key,[]).append(entry)
                assert len(self.variants[key]) <= 4, 'per-key-segment-cap'
            return
        data = prior.legacy.bounded(self.root/prior.container_path(d,'idx'),d['index_bytes'],d['index_bytes'])
        assert prior.digest(data) == d['index'] and data[:8] == b'FNPIDX01'
        assert data[8:40].hex() == d['pack']
        size,codec,count = struct.unpack_from('<QBI',data,40)
        assert (size,codec,count) == (d['pack_bytes'],d['codec'],d['records'])
        if not hasattr(self,'variants'):self.variants = {}
        pos,end,previous,first = 53,0,'',None
        for _ in range(count):
            assert pos+2 <= len(data)
            n = struct.unpack_from('<H',data,pos)[0];pos += 2
            assert n <= 512 and pos+n+48 <= len(data)
            key = data[pos:pos+n].decode();pos += n
            offset,encoded,raw = struct.unpack_from('<QII',data,pos);pos += 16
            digest = data[pos:pos+32].hex();pos += 32
            assert key > previous and prior.legacy.role(key) == d['role']
            assert offset == end and raw <= 8*1024**2 and encoded <= 8*1024**2+65536 and offset+encoded <= size
            assert codec != 0 or encoded == raw
            entry = (d,offset,encoded,raw,digest)
            if key in self.entries:
                assert self.entries[key][3:] == entry[3:],'segment-conflicting-identity'
            else:self.entries[key] = entry
            self.variants.setdefault(key,[]).append(entry)
            assert len(self.variants[key]) <= 4,'per-key-segment-cap'
            first = key if first is None else first
            previous,end = key,offset+encoded
        assert pos == len(data) and end == size and first == d['first'] and previous == d['last']

    def raw(self, key):
        if self.config['version'] != 3:return super().raw(key)
        if key not in self.entries:return prior.legacy.bounded(self.root/key,128*1024**2)
        result = None
        for d,offset,encoded,length,digest in self.variants[key]:
            with (self.root/prior.container_path(d,'pack')).open('rb') as source:
                assert source.seek(0,2) == d['pack_bytes']
                source.seek(offset);data = source.read(encoded)
            assert len(data) == encoded
            if d['codec']:
                if self.zstd is None:self.zstd = prior.legacy.Zstd()
                data = self.zstd.decode(data,length)
            assert len(data) == length and prior.digest(data) == digest,'mapped-record-corruption'
            assert result is None or result == data,'duplicate-mapped-bytes-disagree'
            result = data
        return result

    def verify_packs(self):
        for d in self.descriptors:
            digest = hashlib.sha256()
            with (self.root/prior.container_path(d,'pack')).open('rb') as source:
                for block in iter(lambda:source.read(1024**2),b''):digest.update(block)
            assert digest.hexdigest() == d['pack']
        owners = self.cohort_owners()
        rows, counts, retry = [],{},hashlib.sha256()
        retry_count = 0
        for key in sorted(self.entries):
            raw = self.raw(key)
            kind = self.cohort_record(key,raw,owners)
            counts[kind] = counts.get(kind,0)+1
            if kind == 'legacy-accepted-owner-retry-allocation':
                retry_count += 1
                retry.update((key+'\0'+prior.digest(raw)+'\n').encode())
            rows.append(dict(role=prior.legacy.role(key),key=key,raw_length=len(raw),sha256=prior.digest(raw)))
        self.mapped_cohort_proof = dict(profile='every-mapped-key-to-authenticated-accepted-role-owner-v1',
            accepted_publications=len(owners['publication']),mapped_keys=len(rows),bindings=counts,
            candidate_context=self.candidate_context,captured_selected_authority=True,
            legacy_retry_allocations=retry_count,legacy_retry_evidence_sha256=retry.hexdigest(),
            legacy_retry_policy='d70 cohort accepted attempt + typed allocation; preserved by full conversion; no publication/result acceptance inferred')
        return rows

    def cohort_owners(self):
        authority = self.authority
        assert authority.authority is authority and not authority.candidate_context, 'self-authorizing-physical-candidate'
        assert not self.candidate_context or authority is not self, 'candidate-needs-captured-accepted-authority'
        assert authority.root.resolve() == self.root.resolve() and authority.config['store_id'] == self.config['store_id']
        owners = dict(publication={},result={},attempt={})
        for selected,p in self.accepted_chain().values():
            owner = p['allocation_owner']
            prior.shape(owner,{'attempt','conservative_bytes','includes_intermediates_and_stages'})
            assert owner['attempt'] == p['attempt'] and owner['includes_intermediates_and_stages'] is True
            prior.unsigned(owner['conservative_bytes'])
            for kind,identifier in [('publication',p['id']),('result',p['result_hash']),('attempt',p['attempt'])]:
                assert identifier not in owners[kind], 'accepted-role-owner-alias'
                owners[kind][identifier] = (selected,p)
        return owners

    def cohort_record(self, key, raw, owners):
        role = prior.legacy.role(key)
        parts = key.split('/')
        value = prior.legacy.strict(raw)
        if role == 'publication':
            identity = parts[1][:-5]
            assert identity in owners['publication'], 'mapped-publication-not-accepted'
            selected,p = owners['publication'][identity]
            assert prior.digest(raw) == selected['digest'] and value == p, 'mapped-publication-accepted-binding'
        elif role == 'result':
            identity = parts[1][:-5]
            assert identity in owners['result'], 'mapped-result-not-accepted'
            selected,p = owners['result'][identity]
            prior.shape(value,{'outcome','request','publication','state_digest','changed','removed'})
            graph = {name:p['graph'][name] for name in ['version','consumer','budget','cost','count','tree','fallback']}
            assert prior.digest(raw) == p['result_hash'] and value['outcome'] == 'accepted'
            assert (value['request'],value['publication']) == (p['request'],p['id']), 'mapped-result-owner-binding'
            assert value['state_digest'] == prior.digest(json.dumps(graph,indent=2,ensure_ascii=False).encode()), 'mapped-result-state-binding'
            assert all(type(value[name]) is list and all(type(item) is str for item in value[name]) for name in ['changed','removed'])
        else:
            attempt = parts[1][:-5] if role == 'hint' else parts[1]
            assert attempt in owners['attempt'], 'mapped-attempt-not-accepted'
            selected,p = owners['attempt'][attempt]
            if role == 'hint':
                selector4(value)
                assert value == selected, 'mapped-hint-accepted-binding'
            elif role == 'candidate':
                prior.shape(value,{'purpose','version','store_id','attempt','expected','request','intent',
                    'graph','projection','changes','changed','removed','generation_manifest','checkpoint',
                    'packet_ref','replay_objects','replay_files'})
                from outcome_oracle import intent
                assert type(value['version']) is int and value['version'] == 2 and value['purpose'] == 'publication'
                assert (value['store_id'],value['attempt'],value['request']['request'],value['intent'],value['expected']) == (
                    p['store_id'],p['attempt'],p['request'],p['intent'],p['predecessor']), 'mapped-candidate-accepted-binding'
                assert intent(value['request']) == p['intent'], 'mapped-candidate-request-intent'
                assert value['request']['base'] is None or value['request']['base'] == value['expected']
                assert all(value[name] == p[name] for name in ['graph','projection','changes','generation_manifest','packet_ref'])
                result_raw = self.authority.raw('results/'+p['result_hash']+'.json')
                assert prior.digest(result_raw) == p['result_hash']
                result = prior.legacy.strict(result_raw)
                assert value['changed'] == result['changed'] and value['removed'] == result['removed']
                assert type(value['replay_objects']) is list and all(type(item) is str and prior.HASH.fullmatch(item) for item in value['replay_objects'])
                assert type(value['replay_files']) is dict and all(type(name) is str and type(h) is str and prior.HASH.fullmatch(h) for name,h in value['replay_files'].items())
            else:
                assert role == 'allocation'
                prior.shape(value,{'publication','result'})
                publication = parts[3][:-5]
                assert type(value['publication']) is str and prior.ID.fullmatch(value['publication'])
                assert type(value['result']) is str and prior.HASH.fullmatch(value['result'])
                assert value['publication'] == publication, 'mapped-allocation-path-binding'
                if publication == p['id']:
                    assert value['result'] == p['result_hash'], 'mapped-allocation-accepted-result'
                else:
                    # Old cohort() included typed allocation files belonging to
                    # accepted attempts, including earlier unselected retries.
                    # Rolling discovery adds only p.id; conversion preserves old
                    # keys. Never extend this exception to unaccepted attempts,
                    # another accepted publication, or any other physical role.
                    assert publication not in owners['publication'], 'mapped-allocation-wrong-accepted-owner'
                    return 'legacy-accepted-owner-retry-allocation'
        return 'accepted-'+role

    def accepted_chain(self):
        authority = self.authority
        if hasattr(authority,'_accepted_chain'):return authority._accepted_chain
        selected = prior.legacy.strict((self.root/'SELECTED.json').read_bytes())
        chain = {}
        seen = set()
        expected = None
        while selected is not None:
            selector4(selected)
            assert len(seen) < 200001 and selected['publication'] not in seen
            seen.add(selected['publication'])
            p = prior.legacy.strict(authority.checked(self.root/f"publications/{selected['publication']}.json",selected['digest']))
            assert p['id'] == selected['publication'] and p['store_id'] == self.config['store_id']
            sequence = prior.unsigned(p['sequence'])
            assert expected is None or sequence == expected
            assert sequence not in chain
            chain[sequence] = (dict(selected),p)
            assert (sequence == 0) == (p['predecessor'] is None)
            expected = sequence-1 if sequence else None
            selected = p['predecessor']
        assert chain and min(chain) == 0 and len(chain) == max(chain)+1
        authority._accepted_chain = chain
        return chain

    def terminal_exclusion(self, p, selected, role_number, key):
        if role_number >= 3:
            marker = self.root/f"retired/{p['id']}.json"
            if marker.exists():
                assert prior.legacy.strict(prior.legacy.bounded(marker,4096)) == selected,'retirement-marker-selector-mismatch'
                return dict(reason='permanent-retirement',key=key)
        path = self.root/key
        if path.is_file() and path.stat().st_size > 8*1024**2:
            raw = prior.legacy.bounded(path,128*1024**2)
            value = prior.legacy.strict(raw)
            if role_number == 0:assert prior.digest(raw) == selected['digest']
            elif role_number == 1:assert prior.digest(raw) == p['result_hash']
            elif role_number == 2:assert value == selected
            elif role_number == 3:
                assert value['version'] == 2 and value['purpose'] == 'publication'
                assert value['store_id'] == p['store_id'] and value['attempt'] == p['attempt']
                assert value['request']['request'] == p['request'] and value['intent'] == p['intent']
                assert value['expected'] == p['predecessor']
            else:
                prior.shape(value,{'publication','result'})
                assert value == dict(publication=p['id'],result=p['result_hash'])
            return dict(reason='immutable-oversize',key=key,bytes=len(raw),sha256=prior.digest(raw))
        return None

    def discovery_proof(self, discovery):
        if discovery is None:return dict(rolling_adopted=False,semantic_acceptance_authority=False)
        discovery4(discovery)
        chain = self.accepted_chain()
        def reference(c):
            cutoff4(c)
            assert c['sequence'] in chain and chain[c['sequence']][0] == c['selected'],'cursor-reference-not-accepted-ancestor'
            return c['sequence']
        completed = reference(discovery['completed']) if discovery['completed'] is not None else -1
        interval = discovery['interval']
        target,next_sequence = completed,completed
        if interval is not None:
            target = reference(interval['target'])
            assert target > completed,'unfinished-interval-does-not-advance-cutoff'
            assert interval['next'] is not None,'unfinished-interval-missing-next'
            next_sequence = interval['expected_sequence']
            assert completed < next_sequence <= target and chain[next_sequence][0] == interval['next'],'partial-next-sequence-lineage'
        else:assert completed >= 0,'empty-unprocessed-discovery-state'
        def processed(sequence):return sequence <= completed or interval is not None and next_sequence < sequence <= target
        deferred = {}
        holes = []
        for item in discovery['deferred']:
            sequence = reference(item['publication'])
            assert processed(sequence),'deferred-reference-outside-processed-interval'
            deferred[sequence] = item['roles']
            holes.append(dict(sequence=sequence,selected=item['publication']['selected'],roles=item['roles'],
                              reason_at_last_selection=item['reason']))
        visited,covered,deferred_roles = 0,0,0
        terminal = []
        ranges = [range(completed+1)]
        if interval is not None:ranges.append(range(next_sequence+1,target+1))
        for region in ranges:
            for sequence in region:
                selected,p = chain[sequence]
                assert p['allocation_owner']['attempt'] == p['attempt'] and p['allocation_owner']['includes_intermediates_and_stages'] is True
                keys = [f"publications/{p['id']}.json",f"results/{p['result_hash']}.json",f"attempt-hints/{p['attempt']}.json",
                        f"attempts/{p['attempt']}/candidate.json",f"attempts/{p['attempt']}/allocations/{p['id']}.json"]
                for n,key in enumerate(keys[:5 if discovery['include_attempts'] else 3]):
                    bit = 1 << n
                    if key not in self.entries:
                        exclusion = self.terminal_exclusion(p,selected,n,key)
                        if exclusion is not None:
                            terminal.append(dict(sequence=sequence,**exclusion))
                            continue
                        assert deferred.get(sequence,0) & bit,'covered-key-without-mapping-or-explicit-hole:'+key
                        deferred_roles += 1
                        continue
                    raw = self.raw(key);value = prior.legacy.strict(raw)
                    if n == 0:assert prior.digest(raw) == selected['digest']
                    elif n == 1:assert prior.digest(raw) == p['result_hash']
                    elif n == 2:assert value == selected
                    elif n == 3:
                        assert value['version'] == 2 and value['purpose'] == 'publication'
                        assert value['store_id'] == p['store_id'] and value['attempt'] == p['attempt']
                        assert value['request']['request'] == p['request'] and value['intent'] == p['intent']
                        assert value['expected'] == p['predecessor']
                    else:
                        prior.shape(value,{'publication','result'})
                        assert value == dict(publication=p['id'],result=p['result_hash'])
                    covered += 1
                visited += 1
        return dict(rolling_adopted=True,version=1,include_attempts=discovery['include_attempts'],
                    completed_sequence=completed if completed >= 0 else None,
                    fixed_target_sequence=target,next_sequence=next_sequence if interval is not None else None,
                    checked_processed_publications=visited,checked_mapped_keys=covered,
                    explicit_unmapped_roles=deferred_roles,deferred=holes,
                    terminal_exclusions=terminal,
                    publications_after_fixed_target=max(chain)-target,
                    semantic_acceptance_authority=False,
                    proof='selected accepted predecessor lineage plus per-role mapping or explicit deferred hole; full audit only')

    def selected_work(self, catalog):
        c = self.config
        b = c['physical']
        mutable = self.root/('physical-progress/'+b['id']+'.json')
        assert not mutable.exists(), 'unexpected-mutable-progress'
        w = self.read_metadata('physical-plans/'+b['plan']+'.json', WORK_LIMIT, b['plan'])
        work2(w)
        g = w['generation']
        assert w['store_id'] == c['store_id'] and (g['id'], g['ordinal'], g['catalog']) == (b['id'], b['ordinal'], c['catalog'])
        assert g['packs'] == catalog['packs']
        if self.candidate_context:
            assert w['transition_parent'] is None, 'physical-build-cannot-claim-metadata-only-parent'
        for old in w['older']:
            self.pin(old, old['catalog'])
            retained = self.catalog(old['catalog'])
            assert retained['packs'] == old['packs']
            old_root = self.read_metadata('physical-plans/'+old['plan']+'.json', WORK_LIMIT, old['plan'])
            work2(old_root)
            assert old_root['store_id'] == c['store_id']
            assert (old_root['generation']['id'], old_root['generation']['ordinal'], old_root['generation']['catalog']) == (old['id'], old['ordinal'], old['catalog'])
        for job in w['jobs']:
            for ext, size in [('pack', job['descriptor']['pack_bytes']), ('idx', job['descriptor']['index_bytes'])]:
                name = prior.container_path(job['descriptor'], ext)
                self.required.add(name)
                assert (self.root/name).stat().st_size == size
        stage = self.selector_stage(w) if not self.candidate_context else None
        journal = self.journal() if not self.candidate_context else None
        work_stage = 'physical-progress/'+b['id']+'.work-stage'
        owned_work_stage = None
        if (self.root/work_stage).exists():
            self.required.add(work_stage)
            data = prior.legacy.bounded(self.root/work_stage, WORK_LIMIT)
            owned_work_stage = dict(path=work_stage, bytes=len(data), sha256=prior.digest(data),
                                    accepted_as_immutable_plan=False)
        def present(name):
            path = self.root/name
            return path.exists() or name.startswith('physical-stages/') and path.parent.exists()
        garbage_present = [name for name in w['garbage'] if present(name)]
        owned_directories = []
        for name in w['garbage']:
            if name.startswith('packs/') and name.count('/') == 1:
                path = self.root/name
                children = []
                if path.exists():
                    assert path.is_dir() and not path.is_symlink(), 'owned-pack-directory-type'
                    children = list(path.iterdir())
                    assert len(children) <= 128, 'owned-pack-directory-bound'
                    for child in children:
                        child_name = child.relative_to(self.root).as_posix()
                        prior.metadata_path(child_name)
                        assert child.is_file() and not child.is_symlink()
                        assert child_name in w['garbage'], 'unidentified-child-in-owned-aborted-directory'
                owned_directories.append(dict(path=name,present=path.exists(),
                                              children=len(children),empty_only_removal=True))
        rolling = self.discovery_proof(w.get('discovery'))
        return dict(schema_pin=SCHEMA_PIN, work_version=2, selected_generation=b,
                    rolling_discovery=rolling,
                    physical_parent=w['physical_parent'], physical_parent_is_initial_physical_CAS_token=True,
                    transition_parent=w['transition_parent'], transition_parent_is_comparison_token_not_retained_chain=True,
                    birth_claims=w['birth_claims'], birth_claim_count=len(w['birth_claims']),
                    retained_generations=len(w['older']), pending_loose_jobs=len(w['jobs']),
                    retired_container_jobs=len(w['retired']), garbage_entries=len(w['garbage']),
                    pending_garbage=len(garbage_present), absent_garbage_tombstones=len(w['garbage'])-len(garbage_present),
                    owned_aborted_pack_directories=owned_directories,
                    observed_removed_bytes=w['removed_bytes'], observed_removed_files=w['removed_files'],
                    observed_reconciled_absent=w['reconciled_absent'], counters_are_not_admission_credit=True,
                    metadata_work_root_selection_is_not_physical_activation=True,
                    mutable_progress_accepted=False, pending_selector_stage=stage, journal=journal,
                    owned_work_stage=owned_work_stage,
                    required_paths=sorted(self.required), quiescent_native_lock_state_unmeasured=True)

    def selector_stage(self, w):
        c = self.config
        b = c['physical']
        name = 'physical-progress/'+b['id']+'.selector-stage'
        path = self.root/name
        if not path.exists():
            return None
        self.required.add(name)
        raw = prior.legacy.bounded(path, 4096)
        try:
            target = prior.legacy.strict(raw)
            config3(target)
        except (ValueError, AssertionError, KeyError, TypeError) as error:
            raise AssertionError('unidentified-selector-stage-ownership-must-refuse-and-preserve') from error
        fixed = copy.deepcopy(target)
        next_plan = fixed['physical']['plan']
        fixed['physical']['plan'] = b['plan']
        assert fixed == c, 'foreign-work-selector-stage'
        result = dict(path=name, plan=next_plan, state='owned-candidate-selector', establishes_selection=False)
        if next_plan == b['plan']:
            result['state'] = 'already-selected-stale-stage'
        elif any(old['plan'] == next_plan for old in w['older']):
            result['state'] = 'protected-retained-plan-reference'
        else:
            plan_name = 'physical-plans/'+next_plan+'.json'
            if (self.root/plan_name).exists():
                candidate = self.read_metadata(plan_name, WORK_LIMIT, next_plan)
                work2(candidate)
                assert candidate['store_id'] == c['store_id']
                assert candidate['generation'] == w['generation']
                assert candidate['physical_parent'] == w['physical_parent'], 'same-generation-physical-parent-rewrite'
                assert candidate['transition_parent'] == b['plan'], 'candidate-stage-not-owned-by-exact-selected-root'
                assert set(w['birth_claims']) <= set(candidate['birth_claims']), 'same-generation-birth-claim-loss'
                result['candidate_plan_present_and_authenticated'] = True
            else:
                result['state'] = 'selector-written-before-candidate-plan'
        return result

    def journal(self):
        path = self.root/'PACKING.json'
        if not path.exists():
            return None
        j = self.read_metadata('PACKING.json', 65536)
        assert type(j) is dict and type(j.get('version')) is int
        if j['version'] != 2:
            return dict(version=j['version'], legacy_journal=True, establishes_selection=False,
                        qualification='legacy journal is not a Work2 ownership/activation proof')
        required = {'version', 'id', 'status', 'expected_config', 'catalog', 'old',
                    'reserved_bytes', 'used_bytes', 'deleted_bytes', 'deferred_bytes'}
        assert required <= set(j) <= required|{'target', 'abandoned', 'ownership_transferred'}
        assert type(j['id']) is str and prior.ID.fullmatch(j['id'])
        assert j['status'] in ['building', 'installed', 'registered', 'complete', 'aborted-owned']
        assert type(j['expected_config']) is str and prior.HASH.fullmatch(j['expected_config'])
        assert j['catalog'] is None or type(j['catalog']) is str and prior.HASH.fullmatch(j['catalog'])
        assert type(j['old']) is list and len(j['old']) <= 64
        for d in j['old']:
            descriptor4(d)
        for field in ['reserved_bytes', 'used_bytes', 'deleted_bytes', 'deferred_bytes']:
            prior.unsigned(j[field])
        transferred = j.get('ownership_transferred', False)
        assert type(transferred) is bool
        owners = j.get('abandoned', [])
        assert type(owners) is list and len(owners) <= 64
        seen = set()
        for owner in owners:
            prior.shape(owner, {'id', 'catalog', 'plan'})
            assert type(owner['id']) is str and prior.ID.fullmatch(owner['id']) and owner['id'] not in seen
            seen.add(owner['id'])
            assert all(owner[k] is None or type(owner[k]) is str and prior.HASH.fullmatch(owner[k]) for k in ['catalog', 'plan'])
        current_id = self.config.get('physical', {}).get('id')
        selected_owner = current_id == j['id']
        target = j.get('target')
        if target is not None:
            config3(target)
            assert target['store_id'] == self.config['store_id'] and target['physical']['id'] == j['id']
            assert target['catalog'] == j['catalog']
            if not selected_owner and not transferred:
                assert all(target[field] == self.config[field] for field in ['store_id', 'mode', 'consumer', 'fallback']), 'unselected-target-changes-frozen-semantic-config'
                assert target.get('navigation_version', 1) == self.config.get('navigation_version', 1), 'unselected-target-changes-navigation-config'
        refs = list(owners)
        if not transferred and not selected_owner:
            refs.append(dict(id=j['id'], catalog=j['catalog'], plan=target['physical']['plan'] if target else None))
        ownership = []
        for owner in refs:
            row = dict(owner)
            if not transferred and not selected_owner:
                directory = self.root/('packs/'+owner['id'])
                files = list(directory.iterdir()) if directory.exists() else []
                assert len(files) <= 128
                for item in files:
                    assert item.is_file() and not item.is_symlink()
                    prior.metadata_path(item.relative_to(self.root).as_posix())
                    self.required.add(item.relative_to(self.root).as_posix())
                row['owned_container_files'] = len(files)
                if owner['plan']:
                    name = 'physical-plans/'+owner['plan']+'.json'
                    if (self.root/name).exists():
                        plan = self.read_metadata(name, WORK_LIMIT, owner['plan'])
                        work2(plan)
                        assert plan['store_id'] == self.config['store_id'] and plan['generation']['id'] == owner['id']
                        assert plan['generation']['catalog'] == owner['catalog']
                        row['authenticated_flattened_garbage_paths'] = len(plan['garbage'])
                for name, limit in [('physical-pins/'+owner['id']+'.lock', 1024),
                                    ('physical-progress/'+owner['id']+'.work-stage', WORK_LIMIT),
                                    ('physical-progress/'+owner['id']+'.selector-stage', 4096),
                                    ('physical-stages/'+owner['id']+'/STORE.json', 4096)]:
                    if (self.root/name).exists():
                        self.required.add(name)
                        prior.legacy.bounded(self.root/name, limit)
            ownership.append(row)
        result = dict(version=2, id=j['id'], status=j['status'], abandoned_owner_count=len(owners),
                      ownership_transferred=transferred, selected_by_STORE=selected_owner,
                      pending_owned_refs=ownership, establishes_acceptance=False)
        if not selected_owner and not transferred and j['status'] == 'installed':
            assert target is not None
            candidate = Decoder.candidate(self.root, target, authority=self)
            candidate_work = candidate.read_metadata('physical-plans/'+target['physical']['plan']+'.json', WORK_LIMIT, target['physical']['plan'])
            assert candidate_work['physical_parent'] == j['expected_config'], 'unselected-candidate-physical-parent-journal-mismatch'
            current_binding = prior.legacy.bounded(self.root/'STORE.json', 4096)
            parent_matches = prior.digest(current_binding) == candidate_work['physical_parent']
            if parent_matches:
                assert target['physical']['ordinal'] == self.config.get('physical', {}).get('ordinal', 0)+1, 'candidate-ordinal-not-successor'
            result['activation_parent_matches_current_STORE'] = parent_matches
            result['stale_candidate_is_not_selected'] = not parent_matches
            candidate_rows = candidate.verify_packs()
            self.required.update(candidate.required)
            result['complete_unselected_candidate_records'] = len(candidate_rows)
            result['candidate_is_not_semantic_acceptance'] = True
        return result


def verify(root, output, inventory=None, baseline=None, *, quiescent=False, known_requests=None, request_provenance=None):
    assert re.fullmatch(r'[0-9a-f]{40}', SCHEMA_PIN), 'replacement-source-pin-not-yet-bound'
    assert quiescent, 'explicit-quiescent-verifier-contract-required'
    started = time.monotonic()
    d = Decoder(root, quiescent=True)
    rows = d.verify_packs()
    if inventory:
        assert rows == prior.legacy.expected_inventory(inventory), 'complete-raw-inventory-mismatch'
    sys.path.insert(0, str(prior.PRIOR/'file-native-recovery-reclamation-2026-10-19'))
    import verify_operational
    verify_operational.checked = d.checked
    semantic = verify_operational.verify(root, baseline)
    sys.path.insert(0, str(prior.PRIOR/'file-native-navigation-2026-10-19'))
    import oracle
    oracle.checked = d.checked
    navigation = oracle.verify(root)
    from outcome_oracle import verify as intents
    if request_provenance is not None:
        known_requests = resolve_provenance(d.accepted_chain(),request_provenance,known_requests)
    request_intents = intents(root, decoder=d, known_requests=known_requests)
    inventory_hash = hashlib.sha256()
    for row in rows:
        inventory_hash.update((json.dumps(row, separators=(',', ':'))+'\n').encode())
    result = dict(schema_pin=SCHEMA_PIN, mapped_cohort_closure=d.mapped_cohort_proof,
                  paged_source_pin=paged_index.SOURCE_PIN,
                  paged_full_audits=getattr(d,'paged_audits',[]),
                  legacy_index_descriptors=sum(item.get('page') is None for item in d.descriptors),
                  raw_records=len(rows), raw_bytes=sum(r['raw_length'] for r in rows),
                  inventory_digest=inventory_hash.hexdigest(), physical_metadata=d.physical_metadata,
                  semantic=semantic, navigation=navigation, request_intents=request_intents,
                  mapped_occurrences=sum(len(v) for v in getattr(d,'variants',{}).values()),
                  duplicate_occurrences=sum(max(0,len(v)-1) for v in getattr(d,'variants',{}).values()),
                  independent_codec=d.zstd.binding if d.zstd else None, seconds=time.monotonic()-started,
                  qualification='quiescent full selected representation and bounded pending ownership; catalog2/3/4 and full paged byte/tree audits; segmented mapping and rolling cursor closure; Python memory is not a Rust reader allocation claim; native liveness/crash schedules separately required')
    Path(output).write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--inventory')
    parser.add_argument('--baseline')
    parser.add_argument('--known-requests')
    parser.add_argument('--request-provenance')
    parser.add_argument('--quiescent', action='store_true')
    args = parser.parse_args()
    known = json.loads(Path(args.known_requests).read_bytes()) if args.known_requests else None
    provenance = json.loads(Path(args.request_provenance).read_bytes()) if args.request_provenance else None
    answer = verify(args.root, args.output, args.inventory, args.baseline,
                    quiescent=args.quiescent, known_requests=known,request_provenance=provenance)
    print(json.dumps(dict(status='passed', actual_H=answer['semantic']['publications'],
                          records=answer['raw_records'], schema_pin=SCHEMA_PIN)))
