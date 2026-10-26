"""Verbatim retained mapping/backlog methods with task-owned import wiring."""
import fcntl,hashlib,json
from pathlib import Path
from focused_protocol import require as need
def strict(raw, limit=262144):
    need(type(raw) is bytes and len(raw) <= limit, 'bounded-json-bytes')
    def pairs(items):
        result = {}
        for key, value in items:
            need(key not in result, 'duplicate-json-key')
            result[key] = value
        return result
    def invalid(value):
        raise ValueError('nonfinite-json:'+value)
    return json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid)
def encoded(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode()
def digest(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
class Oracle:
    def __init__(self,host):self.host=host
    def decoder(self,root):
        import physical_decoder_v5 as v5
        d=v5.Decoder(root,quiescent=True);d.verify_packs()
        return d
    def mapping(self,root,key):
        self.host.quiesce(root);d=self.decoder(root)
        present=key in d.entries
        raw=d.raw(key) if present else None
        row=dict(kind='independent-mapping',root=str(root),key=key,mapped=present,independent=True,
                 raw_sha256=hashlib.sha256(raw).hexdigest() if raw is not None else None,
                 decoder_sha256=digest(__import__('physical_decoder_v5').__file__))
        raw_receipt=encoded(row);sha=hashlib.sha256(raw_receipt).hexdigest()
        self.host.emit(dict(row,receipt_sha256=sha))
        return dict(mapped=present,raw_sha256=row['raw_sha256'],independent=True,receipt_sha256=sha)
    def backlog(self,root):
        """Independent physical candidates, pins and owner census, not journal status."""
        self.host.quiesce(root);d=self.decoder(root);root=Path(root)
        config=strict((root/'STORE.json').read_bytes(),65536)
        work_path=root/'physical-plans'/(config['physical']['plan']+'.json')
        work=strict(work_path.read_bytes(),131072)
        # v5 has already fully validated this work and all generation identities.
        protected=set();unprotected=set();excluded=set()
        held_generations=[]
        for gen in work.get('older',[]):
            # Durable lock file presence is NOT a live lease. Probe the native
            # lock without truncation; only an actual conflicting holder pins it.
            with (root/'physical-pins'/(gen['id']+'.lock')).open('rb') as stream:
                try:fcntl.flock(stream,fcntl.LOCK_EX|fcntl.LOCK_NB)
                except BlockingIOError:held_generations.append(gen)
                else:fcntl.flock(stream,fcntl.LOCK_UN)
        for gen in [work['generation'],*held_generations]:
            for desc in gen['packs']:
                prefix='packs/'+(desc['instance']+'/' if desc.get('instance') else '')
                protected.update([prefix+desc['pack']+'.pack',prefix+desc['index']+'.idx'])
        journal=root/'PACKING.json'
        if journal.exists():
            j=strict(journal.read_bytes(),65536)
            if j.get('status') not in {'complete','aborted-owned'}:
                if j.get('id'):
                    protected.update(p.relative_to(root).as_posix() for p in (root/'packs'/j['id']).rglob('*') if p.is_file())
        garbage=set(work.get('garbage',[]))
        for path in garbage:
            if self.host.exists(root,path) and path not in protected:unprotected.add(path)
        for job in work.get('jobs',[]):
            # A job's immutable candidate keys are the independently checked
            # retirement set. Unknown job shape refuses instead of crediting0.
            need(type(job) is dict and set(job)=={'descriptor','frontier','position'},'source-mapped-cleanup-job-shape')
            desc=job['descriptor'];prefix='packs/'+(desc['instance']+'/' if desc.get('instance') else '')
            import paged_index
            raw=(root/(prefix+desc['index']+'.idx')).read_bytes()
            if desc.get('page') is not None:entries=paged_index.full(raw,desc)['entries']
            else:
                # Reuse the authenticated shared decoder, never duplicate its
                # flat-index parser or invoke a nonexistent convenience API.
                probe=object.__new__(type(d));probe.root=root;probe.entries={}
                probe.load_index(desc)
                entries=[(key,*value[1:]) for key,value in sorted(probe.entries.items())]
            for row in entries[job['position']:]:
                name=row[0]
                if self.host.exists(root,name) and name not in protected:
                    if config.get('mode')=='B' and name.startswith('publications/'):
                        protected.add(name)
                    else:unprotected.add(name)
        for hole in (work.get('discovery') or {}).get('deferred',[]):
            need(type(hole) is dict,'deferred-owner-shape')
            if hole.get('reason')=='oversize':
                excluded.add(encoded(hole).decode())
        all_packs={p.relative_to(root).as_posix() for p in (root/'packs').rglob('*') if p.is_file()}
        # Every otherwise-unclassified physical file is backlog. Unselected
        # abandoned births cannot disappear from a summary counter.
        unprotected.update(all_packs-protected)
        row=dict(unprotected=len(unprotected),protected=len(protected),excluded=len(excluded),
                 unprotected_paths=sorted(unprotected),protected_paths=sorted(protected),
                 excluded_facts=sorted(excluded),independent=True)
        self.host.emit(dict(kind='independent-backlog',root=str(root),**row))
        return row
