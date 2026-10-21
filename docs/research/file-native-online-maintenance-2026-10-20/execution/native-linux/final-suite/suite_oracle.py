"""Read-only bridge to the reviewed shared v6/v5/paged oracle; no native parser."""
import copy, fcntl, gzip, hashlib, json, os, sys, uuid
from pathlib import Path
from suite_bootstrap import need, encoded, digest, owned
import native_guard as guard
from native_resources_v3 import strict

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
    def refuse(self,root,checkpoint,require_before_index=False):
        self.host.quiesce(root);before=self.host.inventory(root)
        import physical_decoder_v5 as v5
        opened=[];original=v5.prior.legacy.bounded
        def observed(path,*args,**kwargs):
            opened.append(str(path));return original(path,*args,**kwargs)
        v5.prior.legacy.bounded=observed
        local_reason=None
        try:
            try:
                decoder=v5.Decoder(root,quiescent=True);decoder.verify_packs()
            except (AssertionError,ValueError,OSError,KeyError,IndexError) as error:local_reason=repr(error)
        finally:v5.prior.legacy.bounded=original
        if require_before_index and 'oversize-index' not in checkpoint:
            need(not any(n.endswith('.idx') for n in opened),'index-opened-before-metadata-refusal')
        full=self(root=root,checkpoint=checkpoint,final_pin=self.host.m['final_pin'],
                  known_requests=self.host.h.ledgers[Path(root)].oracle_inputs(),required_mapped_keys=[],expected_refusal=True)
        reason=full['reason']
        self.host.poisoned_roots.add(Path(root))
        need(self.host.inventory(root)==before,'refusal-oracle-mutated-source')
        row=dict(status='expected-independent-refusal',final_pin=self.host.m['final_pin'],engine_code_imported=False,
                 engine_audit_used=False,source_unchanged=True,checkpoint=checkpoint,reason=reason,
                 opened_paths=opened,local_physical_reason=local_reason,full_v6_refusal=full,complete_expected_refusal=True,
                 receipt_sha256=hashlib.sha256(encoded(dict(root=str(root),checkpoint=checkpoint,reason=reason,before=before))).hexdigest())
        self.host.emit(dict(kind='independent-malformed-refusal',**row));return row
    def __call__(self,*,root,checkpoint,final_pin,known_requests,required_mapped_keys,expected_refusal=False):
        need(final_pin==self.host.m['final_pin'],'oracle-final-pin')
        self.host.quiesce(root);before=self.host.inventory(root)
        output=self.host.output_root/('oracle-'+uuid.uuid4().hex+'.json')
        source=self.host.m['oracle']
        need(source['reviewed'] is True and source['final_pin']==final_pin,'independent-v6-reviewed-source')
        config=dict(root=str(root),output=str(output),pin=final_pin,known_requests=known_requests,
                    quota_evidence=self.host.quota_evidence(root),source=source,
                    manifest_sha=self.host.source_sha,manifest_path=self.host.m['manifest_path'],expected_refusal=expected_refusal,
                    bootstrap_path=str(Path(__file__).with_name('suite_bootstrap.py')),
                    bootstrap_sha=self.host.m['source_files'][str(Path(__file__).with_name('suite_bootstrap.py'))]['sha256'])
        # A separate2GiB checker process; fixed script prefix, JSON data only.
        script="""import hashlib,json,sys,types
from pathlib import Path
CFG=CONFIG
bootstrap=Path(CFG['bootstrap_path']);bb=bootstrap.read_bytes()
if hashlib.sha256(bb).hexdigest()!=CFG['bootstrap_sha']:raise SystemExit('checker-bootstrap-before-import')
module=types.ModuleType('suite_bootstrap');module.__file__=str(bootstrap);module.__package__=None
exec(compile(bb,str(bootstrap),'exec'),module.__dict__)
m=module.load(CFG['manifest_path'],CFG['manifest_sha'])
sys.modules['suite_bootstrap']=module
sys.meta_path.insert(0,module.ClosedImports(m['source_files']))
for item in reversed(CFG['source']['import_directories']):sys.path.insert(0,item)
digest=module.digest
import physical_decoder_v6 as oracle
oracle.FINAL_SOURCE_PIN=CFG['pin']
try:
    answer=oracle.verify(CFG['root'],CFG['output'],quiescent=True,known_requests=CFG['known_requests'],quota_evidence=CFG['quota_evidence'])
except (AssertionError,ValueError,OSError,KeyError,IndexError) as error:
    if not CFG['expected_refusal']:raise
    answer={'status':'expected-independent-refusal','reason':repr(error),'schema_pin':CFG['pin']}
    with Path(CFG['output']).open('xb') as stream:stream.write((json.dumps(answer,sort_keys=True)+'\\n').encode())
else:
    if CFG['expected_refusal']:raise RuntimeError('full-independent-oracle-accepted-malformed')
print(json.dumps({'written':CFG['output'],'sha256':digest(CFG['output'])}))
""".replace('CONFIG',repr(config))
        needed=self.host.profile(self.host._current)['oracle_output_bytes']
        lease='oracle-artifacts-'+uuid.uuid4().hex
        self.host.budget.reserve_evidence(lease,needed,2)
        result=self.host.commands.run(root,'oracle',[self.host.m['python'],'-I','-S','-B','-'],
                                      script.encode(),seconds=120)
        raw=output.read_bytes();proof=strict(raw,needed)
        self.host.budget.settle(lease)
        after=self.host.inventory(root)
        need(before==after,'oracle-mutated-source-inventory')
        if expected_refusal:
            need(proof['status']=='expected-independent-refusal' and proof['schema_pin']==final_pin,'actual-full-refusal-receipt')
            return dict(proof,receipt=str(output),receipt_sha256=hashlib.sha256(raw).hexdigest(),command_token=result['token'])
        d=self.decoder(root)
        for key in required_mapped_keys:need(key in d.entries,'required-mapped-key-missing')
        mapped=sorted(required_mapped_keys)
        row=dict(status='passed',final_pin=final_pin,engine_code_imported=False,engine_audit_used=False,
                 actual_H=len(d.accepted_chain()),mapped_keys=mapped,source_unchanged=True,
                 checks=sorted(__import__('focused_protocol').ORACLE_CHECKS),
                 receipt_sha256=hashlib.sha256(raw).hexdigest(),receipt=str(output),underlying=proof,
                 command_token=result['token'],before_after_inventory_sha256=hashlib.sha256(encoded(before)).hexdigest())
        # No claim from a fake callback: v6 process must complete and create its
        # exact source-bound receipt, including all accepted request intentions.
        need(proof['schema_pin']==final_pin and proof['request_intents'],'complete-independent-intent-proof')
        return row
