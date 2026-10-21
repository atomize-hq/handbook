"""Concrete native Host for shared Harness3.1; no engine or parser implementation."""
import copy, hashlib, json, os, re, shutil, threading, time, uuid
from contextlib import contextmanager
from pathlib import Path
import native_guard as guard
from native_driver import owner
from native_resources_v3 import Reservations, canonical, confined, inventory, regular_read, strict
from native_adapter_v3 import NativeAdapter
from focused_protocol import Harness, Inputs, intent, normalize, ORACLE_CHECKS
from suite_bootstrap import need, encoded, digest, owned
from suite_driver import Commands,JsonDriver


def suite_limits(m):
    policy=m['native_guard_policy']
    need(policy['reviewed'] is True and policy['version']==5,'explicit-new-guard4-source-review')
    need(policy['source_sha256']==m['source_files'][policy['source_path']]['sha256'] and
         digest(policy['source_path'])==policy['source_sha256'],'actual-new-guard4-source')
    cap=policy['fixture_cap_bytes'];need(type(cap) is int and cap in {96*1024**2,512*1024**2,640*1024**2},'closed-parent-selected-fixture-cap')
    limits={'source':(4*1024**2,1500),'target':(256*1024**2,5000),'fixtures':(cap,30000),'evidence':(24*1024**2,2000)}
    c=m.get('private_cache',{'enabled':False})
    if c['enabled']:
        need(c['reviewed'] is True and c['claim_id']==m['claim']['claim_id'] and c['ledger_sha256']==m['claim']['ledger_sha256'] and c['root']==str(guard.ROOT/'cargo-private'),'explicit-Linux-C1-allocation')
        need(type(c['cap_bytes']) is int and 0<c['cap_bytes']<=64*1024**2 and type(c['cap_files']) is int and 0<c['cap_files']<=4096,'closed-Linux-C1-limits')
        limits['cargo-private']=(c['cap_bytes'],c['cap_files'])
    return limits

def suite_owner(m):
    """New owner protocol; never relabel the frozen4dd ancestor convention."""
    limits=suite_limits(m);ancestors={};pid=os.getppid()
    while pid>1:
        row=guard.process(pid);need(row is not None,'new-owner-ancestor-observation')
        ancestors[pid]=row;pid=row['ppid']
    candidates=[]
    for path in (guard.ROOT/'source').glob('guard-owner-*.json'):
        data=strict(regular_read(path,65536),65536)
        if data.get('guard_version')!=5:continue
        row=ancestors.get(data['pid'])
        if row is None or row['start']!=data['process_start']:continue
        need(data['runtime_scope']==m.get('runtime_scope','full-body') and data['guard_sha256']==m['native_guard_policy']['source_sha256'] and
             data['policy_manifest_sha256']==m['manifest_sha256'] and
             data['policy_manifest_path']==m['manifest_path'],'new-owner-source-and-manifest')
        need(digest(Path('/proc',str(row['pid']),'cmdline'))==data['owner_cmdline_sha256'],'same-live-ancestor-commandline')
        need(data['category_limits']=={k:list(v) for k,v in limits.items()},'exact-new-owner-category-limits')
        candidates.append(data)
    need(len(candidates)==1,'one-live-new-guard4-ancestor')
    owner=candidates[0]
    need(time.time()<min(owner['absolute_deadline'],guard.CUTOFF) and os.getpgrp()!=owner['pid'],'new-guard-owned-child-group')
    need(owner['claim_id']==m['claim']['claim_id'] and owner['ledger_sha256']==m['claim']['ledger_sha256'] and
         owner['source_pin']==m['final_pin'],'new-guard-parent-authority')
    return owner

class SuiteReservations(Reservations):
    def __init__(self,m,*args,**kwargs):
        self.limits=suite_limits(m)
        super().__init__(*args,**kwargs)
    def check(self,extra):
        # Same monotone floors, all pending leases, native census retries and
        # prospective claim ceilings; only the reviewed fixture cap differs.
        for category in ('fixtures','evidence'):
            current=self.observation(guard.ROOT/category)
            reservations=[v[category] for v in self.pending.values()]
            add=extra.get(category,dict(bytes=0,files=0))
            byte_cap=min(self.limits[category][0],self.claim['before'][category]['bytes']+self.claim['categories'][category]['bytes'])
            file_cap=min(self.limits[category][1],self.claim['before'][category]['files']+self.claim['categories'][category]['files'])
            byte_floor=max(current['bytes'],self.anchor[category]['bytes']+self.charged[category]['bytes'])
            file_floor=max(current['file_count'],self.anchor[category]['files']+self.charged[category]['files'])
            need(byte_floor+sum(v['bytes'] for v in reservations)+add['bytes']<=byte_cap,'prospective-byte-limit:'+category)
            need(file_floor+sum(v['files'] for v in reservations)+add['files']<=file_cap,'prospective-file-limit:'+category)
        need(shutil.disk_usage(guard.ROOT).free>=20*1024**3,'free-floor')

class FinalHarness(Harness):
    @contextmanager
    def case(self,case_id,root,forecast):
        if root in self.active:
            need(self.a.case_roots.get(str(root))==case_id and forecast==self.a.forecast(case_id),'borrow-identical-active-case')
            yield
        else:
            with super().case(case_id,root,forecast):yield
    def small(self,root,mode,forecast,codec='zstd'):
        # Only these NEW roots opt in. No legacy capsule registry is converted.
        with self.case('S-'+mode+'-'+codec,root,forecast):
            self.initialize(root,mode,64)
            self.call(root,'lifecycle',operation='enable-terminal-v2',explicit_disposable_opt_in=True)
            for n in range(63):self.publish(root,'S-'+mode+'-'+codec+'-initial-'+str(n))
            self.pack(root,codec,bootstrap=True);self.cleanup(root,max_batches=160)
            self.verify(root,'N64-H64-bootstrap')
            for cycle,publishers in enumerate((1,4,1)):
                self.concurrent_cycle(root,'S-'+mode+'-'+codec+'-c'+str(cycle+1),publishers,
                                      codec=codec,collect_conflicts=publishers==4)
            need(len(self.ledgers[root].accepted)==112,'three-real-16-publication-cycles')
            self.coupled_capture(root,'S-'+mode+'-'+codec,codec)
            self.check_outcomes(root);self.verify(root,'small-final')
    def note(self,kind,**fields):
        if fields.get('failed_corpus_policy')=='engine-abandon-collect-each-round':
            fields['shared_original_policy_label']=fields['failed_corpus_policy']
            fields['failed_corpus_policy']='terminal-v2-definitive-conflict-retain-all'
        return super().note(kind,**fields)
    def collect_failed_round(self,root,label,requests,outcomes,candidates):
        need(self.a.case_roots[str(root)] in {'S-A-zstd','S-B-zstd'},'terminal-v2-only-new-concurrent-roots')
        need(candidates and set(candidates)=={r['request'] for r in requests},'four-source-bound-candidates')
        failed=[]
        for request,response in zip(requests,outcomes):
            row=candidates[request['request']];c=strict(row['bytes'],4*1024**2)
            need(c['request']==request and c['intent']==intent(request) and c['expected']==request['base'] and
                 request['packet'] is False and row['path']=='attempts/'+c['attempt']+'/candidate.json','ordinary-preselection-candidate')
            self.note('prepared-prefix-native-bytes',path=row['path'],raw_utf8=row['bytes'].decode(),sha256=hashlib.sha256(row['bytes']).hexdigest())
            if response.get('status')=='error':
                need(response.get('error') in {'conflict:captured-base','conflict:expected-publication'},'definitive-conflict-only')
                failed.append((c['attempt'],row['path'],row['bytes']))
        need(len(failed)==3,'exact-three-real-conflicts')
        self.verify(root,label+'-before-terminal-release')
        before=self.a.inventory(root)
        for attempt,path,raw in failed:
            self.call(root,'lifecycle',operation='release-definitive-conflict',attempt=attempt)
            need(self.read(root,path,4*1024**2)==raw,'terminal-release-destroyed-candidate')
        after=self.a.inventory(root)
        need(set(before['files'])<=set(after['files']),'terminal-release-deleted-retained-prefix')
        for name,fact in before['files'].items():
            if name!='RESERVATIONS.json':
                need(after['files'][name]['sha256']==fact['sha256'],'terminal-release-mutated-prefix:'+name)
        registry=self.document(root,'RESERVATIONS.json',65536)
        need(registry['version']==2 and registry['terminal_count']>=len(failed),'terminal-v2-registry-charge')
        self.note('terminal-v2-round',attempts=[x[0] for x in failed],retained_all_failed_files=True,reclaimed_bytes=0,
                  before_sha256=hashlib.sha256(canonical(before)).hexdigest(),after_sha256=hashlib.sha256(canonical(after)).hexdigest())
        self.verify(root,label+'-after-terminal-release')

class Host(NativeAdapter):
    def __init__(self,m,source_sha):
        self.m=m;self.source_sha=source_sha;claim=suite_owner(m)
        need(claim['source_pin']==m['final_pin'] and claim['claim_id']==m['claim']['claim_id'] and
             claim['ledger_sha256']==m['claim']['ledger_sha256'],'suite-native-live-claim')
        self.growth=m['growth'];self.output_root=owned(m['evidence_root'],guard.ROOT/'evidence')
        need(self.output_root.is_dir(),'pre-admitted-evidence-parent')
        admission=dict(go=True,final_pin=m['final_pin'],claim_id=claim['claim_id'],ledger_sha256=claim['ledger_sha256'],cases={})
        budget=SuiteReservations(m,claim,admission,expected_ledger_sha256=claim['ledger_sha256'])
        self.budget=budget;self._sink_lock=threading.RLock();self._sink=self.output_root/'suite-events.jsonl'
        key='suite-sink-'+uuid.uuid4().hex;budget.reserve_evidence(key,4096,1)
        with self._sink.open('xb'):pass
        budget.settle(key)
        self._wire_lock=threading.RLock();self.wire_output=self.output_root/'native-wires.jsonl'
        key='wire-sink-'+uuid.uuid4().hex;budget.reserve_evidence(key,4096,1)
        with self.wire_output.open('xb'):pass
        budget.settle(key)
        out=self.output_root/'engine-events.jsonl'
        driver=JsonDriver(m['candidate']['path'],out,expected_sha256=m['candidate']['sha256'],suite_pin=m['final_pin'],
                      binary_source_pin=m['candidate']['source_pin'],reservations=budget,wire_sink=self.wire_emit,suite_manifest=m)
        versions=m['case_versions']
        binding=dict(final_pin=m['final_pin'],case_versions=versions,checkpoints=m['checkpoints'],
                     crash_constructors=m['crash_constructors'],capabilities=m['capabilities'],
                     capabilities_reviewed=m['capabilities_reviewed'],source_manifest_sha256=source_sha,
                     source_hashes=[r['sha256'] for r in m['source_files'].values()]+m['workspace']['source_sha256s'],adapter_sha256=digest(__file__),hooks={})
        super().__init__(driver,binding)
        self.binding['guard_sha256']=m['native_guard_policy']['source_sha256']
        self.commands=Commands(self);self._current=None;self._aliases={};self._command_starts=[]
        self._known_candidates={};self._case_requests={};self._command_rows=[];self.poisoned_roots=set();self.case_results={};self.final_registry=[]
        from suite_oracle import Oracle
        self.oracle=Oracle(self)
        self.h=FinalHarness(driver,self,self.oracle,self.emit,final_pin=m['final_pin'])
    def emit(self,row):
        row=dict(row,qualification_pin=self.m['final_pin'],suite_manifest_sha256=self.source_sha,
                 guard_nonce=self.driver.owner['nonce'] if hasattr(self,'driver') else None)
        raw=encoded(row)+b'\n';key='emit-'+uuid.uuid4().hex
        with self._sink_lock:
            self.budget.reserve_evidence(key,len(raw),0)
            with self._sink.open('ab') as f:f.write(raw);f.flush()
            self.budget.settle(key)
    def wire_emit(self,row):
        # Its exact lease was admitted before Popen. Do not add whole-tree walks
        # to the fixed500ms positive-start callback; retain the lease until end.
        need(row['kind']=='suite-native-input-wire','pre-reserved-wire-row-only')
        row=dict(row,qualification_pin=self.m['final_pin'],suite_manifest_sha256=self.source_sha,
                 guard_nonce=self.driver.owner['nonce'])
        raw=encoded(row)+b'\n'
        with self._wire_lock:
            # This owner cannot retire the entry before this writer finishes.
            # A read-only dict lookup never waits behind an unrelated census.
            lease=self.budget.pending.get(row['reservation'])
            need(lease and lease['root'] is None and lease['evidence']['bytes']>=len(raw),'exact-held-wire-reservation')
            with self.wire_output.open('ab') as stream:stream.write(raw);stream.flush()
    def profile(self,case_id):
        name=self.m['case_families'][case_id]
        need(name in self.growth['families'],'registered-case-growth-family')
        return copy.deepcopy(self.growth['families'][name])
    def root(self,case_id,label='store'):
        need(case_id in self.m['case_versions'] and re.fullmatch(r'[A-Za-z0-9_-]+',label),'known-case-root-label')
        slug=case_id.replace('/','_')
        root=owned(Path(self.m['fixture_root'])/slug/'attempt-1'/'disposable'/label,guard.ROOT/'fixtures')
        return root
    def forecast(self,case_id):
        value=self.profile(case_id)['forecast']
        return dict(value)
    def register(self,case_id,root):
        root=owned(root,guard.ROOT/'fixtures');slot=case_id+'@'+str(root.relative_to(Path(self.m['fixture_root'])))
        spec=self.profile(case_id)
        # Expansion is source-defined and parent-reviewed: namespace, templates,
        # per-case counts and all SHA values are sealed in the suite manifest.
        need(self.m['templates_reviewed'] is True,'parent-root-template-release')
        self.budget.admission['cases'][slot]=dict(root=str(root),
            forecast_sha256=hashlib.sha256(canonical(spec['forecast'])).hexdigest(),
            ceiling=spec['ceiling'],calls=spec['calls'],controls=spec['controls'])
        self.budget.preflight(slot,root,spec['forecast'])
        self.case_roots[str(root)]=case_id
        return spec['forecast']
    def preflight(self,case_id,root,forecast,final_pin):
        need(case_id in self.m['case_versions'],'declared-case-ID')
        version=self.m['case_versions'][case_id]
        need(version['status']=='reviewed-source-map' and version['final_pin']==final_pin==self.m['final_pin'],'released-version-map-before-write')
        need(version['source_sha256'] in self.binding['source_hashes'],'actual-case-code-SHA')
        self._current=case_id;self.register(case_id,root)
        self._case_start_tokens={id(d):set(d.registry) for d in self.drivers}
        need(forecast==self.forecast(case_id),'exact-case-forecast')
        return dict(go=True,case_id=case_id,root=str(root),final_pin=final_pin,reader_limit=256*1024**2,
            checker_limit=2*1024**3,default_payloads=True,heavy_token=self.driver.owner['nonce'],
            host_guard=self.driver.owner['claim_id'],binary_sha256=self.driver.binary_sha256,
            binary_source_pin=final_pin,source_manifest_sha256=self.source_sha,adapter_sha256=digest(__file__),
            guard_sha256=self.binding['guard_sha256'])
    def mkdir(self,path):
        path=owned(path,guard.ROOT/'fixtures');need(not path.exists(),'fresh-fixture-directory')
        missing=[];p=path
        while not p.exists():missing.append(p);p=p.parent
        key='mkdir-'+uuid.uuid4().hex;self.budget.reserve_control(key,0,directories=len(missing))
        path.mkdir(parents=True,exist_ok=False);self.budget.settle(key)
    def alias(self,case_id,label,source=None):
        root=self.root(case_id,label);need(case_id==self._current,'alias-only-current-case')
        count=len(self._aliases.get(case_id,[]))
        need(count<self.profile(case_id)['max_roots'],'finite-case-root-count')
        self.register(case_id,root);self._aliases.setdefault(case_id,[]).append(root)
        self.h.active.add(root);self.h.ledgers.setdefault(root,Inputs())
        if source is not None:
            self.quiesce(source);facts=self.inventory(source)
            key='clone-'+uuid.uuid4().hex
            self.budget.reserve_control(key,facts['bytes'],files=facts['file_count'],directories=facts['directory_count']+1)
            root.parent.mkdir(parents=True,exist_ok=True)
            shutil.copytree(source,root,symlinks=False)
            self.budget.settle(key)
            for row in self.h.ledgers[source].accepted.values():self.h.ledgers[root].accept(row['request'],row['result'],row['selected'])
            self.h.call(root,'lifecycle',operation='setup')
        return root
    @contextmanager
    def case(self,case_id,label='store'):
        source_case=self.m.get('donor_cases',{}).get(case_id)
        root=self.root(source_case or case_id,label)
        if source_case:
            need(source_case in self.case_results and root in self.h.ledgers and
                 root not in self.poisoned_roots and self.h.ledgers[root].accepted,'completed-authentic-donor-inputs')
        with self.h.case(case_id,root,self.forecast(case_id)):
            yield Case(self,case_id,root)
    @contextmanager
    def existing_lineage(self,case_id,root):
        need(case_id==self._current and root in self.h.ledgers and root not in self.poisoned_roots,'closed-existing-lineage')
        need(len(self._aliases.get(case_id,[]))<self.profile(case_id)['max_roots'],'finite-final-lineage-count')
        self.register(case_id,root);self.h.active.add(root);self._aliases.setdefault(case_id,[]).append(root)
        try:yield Case(self,case_id,root)
        finally:self.h.active.discard(root)
    def write(self,root,name,raw):
        need(type(raw) is bytes,'fixture-byte-mutation-only')
        path=confined(root,Path(root)/name);key='write-'+uuid.uuid4().hex
        parents=[];p=path.parent
        while not p.exists():parents.append(p);p=p.parent
        self.budget.reserve_control(key,len(raw),files=0 if path.exists() else 1,directories=len(parents))
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_bytes(raw);self.budget.settle(key)
    def preserve_move(self,root,name,suffix='.withheld'):
        path=confined(root,Path(root)/name);target=path.with_name(path.name+suffix)
        need(path.is_file() and not target.exists(),'fresh-preserved-mutation-prefix');path.rename(target)
    def prepared_candidates(self,root,requests):
        answer={}
        for p in (Path(root)/'attempts').glob('*/candidate.json'):
            raw=self.read(root,p.relative_to(root).as_posix(),4*1024**2);c=strict(raw,4*1024**2)
            for request in requests:
                if c.get('request')==request:
                    need(request['request'] not in answer,'unique-prepared-attempt')
                    answer[request['request']]=dict(path=p.relative_to(root).as_posix(),bytes=raw)
        need(set(answer)=={r['request'] for r in requests},'all-four-real-candidates')
        return answer
    def mapping(self,root,key):return self.oracle.mapping(root,key)
    def interrupt(self,handle,kind):
        native=handle.get('native')
        if native and native['input']['action']=='packing':
            need(kind=='death' and native['ready_ns'] is not None and native['env']['EXP_BARRIER'] in
                 {'physical-before-selection','physical-after-selection'},'source-bound-ready-maintainer-death')
        return super().interrupt(handle,kind)
    def backlog(self,root):return self.oracle.backlog(root)
    def independent_oracle(self,**fields):return self.oracle(**fields)
    def quota_evidence(self,root):
        evidence=copy.deepcopy(self.m['quota_evidence'].get(str(root),{}))
        evidence.setdefault('owners',{});evidence.setdefault('stages',{})
        evidence.setdefault('qualified_schedule_sha256',[])
        return evidence
    def backup_restore(self,c,label):
        import operational_backup as backup
        self.quiesce(c.root);before=self.inventory(c.root)
        excluded=set(self.m['backup_ephemeral_names'])
        need(excluded and all('/' not in n and '*' not in n for n in excluded),'finite-backup-exclusions')
        need(not any(n.startswith('physical-pins') for n in excluded),'durable-pin-locks-are-data')
        archive=self.root(c.id,'archive-'+label);restored_root=self.root(c.id,label)
        need(not archive.exists() and not restored_root.exists(),'fresh-backup-prefixes')
        key='backup-'+uuid.uuid4().hex
        # Exactly TWO new destinations: stream archive and restored full tree.
        allowance=2*before['bytes']+sum(2*len(n.encode())+512 for n in before['files'])+65536
        self.budget.reserve_control(key,allowance,files=before['file_count']+8,directories=before['directory_count']+4)
        previous=set(backup.EXCLUDED)
        try:
            backup.EXCLUDED=excluded;archive.parent.mkdir(parents=True,exist_ok=True)
            backup.capture(c.root,archive,self.m['final_pin'],True);backup.restore(archive,restored_root)
        finally:backup.EXCLUDED=previous;self.budget.settle(key)
        for name,fact in before['files'].items():
            if name.startswith('physical-pins/'):
                p=restored_root/name;need(p.is_file() and digest(p)==fact['sha256'],'omitted-durable-pin-lock')
        self.register(c.id,restored_root);self.h.active.add(restored_root)
        self._aliases.setdefault(c.id,[]).append(restored_root);self.h.ledgers[restored_root]=Inputs()
        for row in self.h.ledgers[c.root].accepted.values():
            self.h.ledgers[restored_root].accept(row['request'],row['result'],row['selected'])
        for (root,path),request in list(self._known_candidates.items()):
            if root==str(c.root):self._known_candidates[(str(restored_root),path)]=copy.deepcopy(request)
        restored=Case(self,c.id,restored_root);restored.call('lifecycle',operation='setup')
        need(self.inventory(c.root)==before,'backup-mutated-original')
        return restored
    def extend(self,h,root,spec):
        from suite_extensions import EXTENSIONS
        need(spec['case_id'] in EXTENSIONS and spec['source_sha256']==digest(__import__('suite_extensions').__file__),'closed-extension-implementation')
        return EXTENSIONS[spec['case_id']](Case(self,spec['case_id'],root),spec)
    def command_binding(self,action,argv,stdin,env):
        need(action in {'ordinary','workspace','oracle'} and argv and Path(argv[0]).is_absolute(),'registered-native-command-kind')
        row=self.m['tools'][argv[0]];need(digest(argv[0])==row['sha256'],'native-tool-binding')
        need(not any(x in argv for x in ['-c','--command']) or action=='oracle','no-unbound-shell-eval')
        if action=='ordinary':
            from suite_ordinary import SCRIPT
            need(argv[1:5]==['--noprofile','--norc','-s','--'] and stdin==SCRIPT.encode(),'exact-source-bound-Bash-script')
            need(argv[0]==self.m['bash'] and self.m['python']=='/usr/bin/python3','ordinary-native-interpreters')
        need(not env or set(env)<=set(self.m['command_env_keys']),'command-environment-allowlist')
    def environment(self,extra):
        e={k:v for k,v in os.environ.items() if not k.startswith('EXP_')}
        if extra:
            for k,v in extra.items():
                if v is None:e.pop(k,None)
                else:e[k]=v
        return e
    def command_started(self,token,pid,start,action,ns):
        self._command_starts.append(dict(token=token,pid=pid,start=start,action=action,observed_ns=ns))
    def close_case(self,case_id,root):
        command_error=None
        try:self.commands.close()
        except BaseException as e:command_error=repr(e)
        closure=super().close_case(case_id,root)
        complete=closure['registries'];self.final_registry=copy.deepcopy(complete)
        closure['global_registry_sha256']=hashlib.sha256(canonical(complete)).hexdigest()
        closure['global_registry_entries']=sum(len(r) for r in complete)
        closure['registries']=[{k:v for k,v in rows.items() if k not in self._case_start_tokens.get(id(driver),set())}
                               for driver,rows in zip(self.drivers,complete)]
        closure['registry_encoding']='lossless-append-deltas; prior closed rows immutable; final full snapshot retained'
        aliases=self._aliases.pop(case_id,[])
        for other in aliases:
            self.h.active.discard(other);self.case_roots.pop(str(other),None)
            self.budget.case_specs.pop(str(other),None)
        if command_error:
            closure['processes_closed']=False;closure['handles_closed']=False;closure['errors'].append(command_error)
        self._current=None
        return closure

class Case:
    def __init__(self,host,case_id,root):self.a=host;self.h=host.h;self.id=case_id;self.root=Path(root)
    def init(self,mode='A',n=16):
        self.h.initialize(self.root,mode,n);return self
    def call(self,action,**fields):
        request=fields.get('request')
        if request is not None and action in {'publish','prepare'}:self.h.ledgers[self.root].record(request)
        if action=='checkpoint':
            # Exact Request construction is source-mapped in the final manifest.
            need(self.a.m['checkpoint_request_shape']=='checkpoint-op-v1-base-none-packet-false','source-mapped-checkpoint-request')
            request=dict(request=fields['request_id'],base=None,
                         ops=[dict(kind='checkpoint',id='',text='',target='',value=1)],packet=False)
            self.h.ledgers[self.root].record(request)
        response=self.h.call(self.root,action,**fields)
        if response and response.get('status')=='ok':
            r=response.get('result')
            if action=='prepare' and isinstance(r,dict) and r.get('candidate'):
                raw=self.h.read(self.root,r['candidate'],4*1024**2)
                self.a._known_candidates[(str(self.root),r['candidate'])]=strict(raw,4*1024**2)['request']
            if action in {'publish','checkpoint','select'} and isinstance(r,dict):
                accepted=r.get('result',r)
                if isinstance(accepted,dict) and accepted.get('outcome')=='accepted':
                    if action=='select':
                        request=self.a._known_candidates.get((str(self.root),fields['candidate']))
                        if request is None:
                            request=self.document(fields['candidate'],4*1024**2)['request']
                            need((request['request'],intent(request)) in self.h.ledgers[self.root].attempts,'captured-restored-candidate')
                    need(request is not None,'exact-captured-accepted-input')
                    self.h.ledgers[self.root].accept(request,accepted,self.h.selected(self.root))
        return response
    def publish(self,name,ops=None,**fields):
        request=dict(request=name,base=self.h.selected(self.root),ops=ops or [],packet=fields.pop('packet',False))
        return self.call('publish',request=request,**fields)
    def lost(self,name,phase,ops=None):
        request=dict(request=name,base=self.h.selected(self.root),ops=ops or [],packet=False)
        self.h.ledgers[self.root].record(request);self.h.call(self.root,'publish',request=request,code=86,env={'EXP_EXIT':phase})
        result=self.h.call(self.root,'resolve',request_id=name)['result']
        if result.get('outcome')=='accepted':self.h.ledgers[self.root].accept(request,result,self.h.selected(self.root))
        return result
    def pack(self,codec='raw',bootstrap=False,**fields):
        return self.h.pack(self.root,codec,bootstrap=bootstrap,**fields)
    def drain(self):return self.h.drain_two_passes(self.root)
    def verify(self,name='boundary'):return self.h.verify(self.root,self.id+'-'+name)
    def read(self,name,limit=128*1024):return self.h.read(self.root,name,limit)
    def document(self,name,limit=128*1024):return strict(self.read(name,limit),limit)
    def write(self,name,raw):return self.a.write(self.root,name,raw)
    def clone(self,label):
        root=self.a.alias(self.id,label,self.root);return Case(self.a,self.id,root)
    def selected(self):return self.h.selected(self.root)


# V5 phase completion owns its receipt and caller, not merely its workload.
def checked_snapshot(m,before,reserve):
    limits=suite_limits(m);answer={}
    for category,(cap,files) in limits.items():
        row=guard.inventory(guard.ROOT/category)
        ceiling=reserve[category]
        need(row['bytes']<=min(cap,before[category]['bytes']+ceiling['bytes']) and
             row['files']<=min(files,before[category]['files']+ceiling['files']),'inclusive-phase-cap:'+category)
        answer[category]=row
    phase=m['phase']
    need(answer['source']['bytes']+phase['external_source_upper_bytes']<=phase['combined_source_cap_bytes'],'whole-local-native-future-source')
    need(answer['evidence']['bytes']+phase['external_evidence_upper_bytes']<=phase['combined_evidence_cap_bytes'],'whole-local-native-return-evidence')
    need(shutil.disk_usage(guard.ROOT).free>=20*1024**3,'phase-free-floor')
    return answer


def inclusive_receipt(m,path,value,before,reserve):
    """Fixed point counts this exclusive output before writing and checks afterward.
    No self hash or recursive receipt. The caller accepts only this function's
    successful return AND the phase process's clean exit under its live watchdog.
    """
    with phase_io(m,True):
        import os
        path=owned(path,guard.ROOT/'evidence');need(not path.exists(),'fresh-inclusive-receipt')
        current=checked_snapshot(m,before,reserve);predicted=copy.deepcopy(current)
        value=dict(value);value['inclusive_final']=predicted
        if 'final' in value:value['final']=predicted
        size=-1
        for attempt in range(16):
            raw=encoded(value)+b'\n'
            if len(raw)==size:break
            size=len(raw);predicted['evidence']=dict(current['evidence'])
            predicted['evidence']['bytes']+=size;predicted['evidence']['files']+=1
            # Allocated blocks are observed afterward, not guessed from logical bytes.
            predicted['evidence'].pop('allocated_bytes',None)
            value['receipt_bytes']=size
        else:raise RuntimeError('receipt-byte-fixed-point')
        raw=encoded(value)+b'\n';need(len(raw)==size and size<=(m['phase']['guard_receipt_max_bytes'] if str(path)==m['phase']['guard_receipt_path'] else m['phase']['receipt_max_bytes']),'receipt-byte-exact-and-bounded')
        cap=min(suite_limits(m)['evidence'][0],before['evidence']['bytes']+reserve['evidence']['bytes'])
        files=min(suite_limits(m)['evidence'][1],before['evidence']['files']+reserve['evidence']['files'])
        need(current['evidence']['bytes']+size<=cap and current['evidence']['files']+1<=files,'own-receipt-prospective-allocation')
        need(current['evidence']['bytes']+size+m['phase']['external_evidence_upper_bytes']<=m['phase']['combined_evidence_cap_bytes'],'own-receipt-all-copy-cap')
        with path.open('xb') as out:out.write(raw);out.flush();os.fsync(out.fileno())
        actual=checked_snapshot(m,before,reserve)
        for key in actual:
            need(actual[key]['bytes']==predicted[key]['bytes'] and actual[key]['files']==predicted[key]['files'],'quiescent-inclusive-receipt-census:'+key)
        need(path.stat().st_size==size,'own-receipt-final-size')
        return value


def cache_identity(root,hashes=True):
    """Protected global cache identity includes removals, changes and aliases.
    Never copy configuration or credentials. Identities, not their contents, are
    retained in evidence. Regular-file SHA is checked before and after a phase.
    """
    rows=[];root=Path(root)
    for folder,dirs,names in os.walk(root,followlinks=False):
        for name in dirs+names:
            p=Path(folder)/name;s=p.lstat();rel=p.relative_to(root).as_posix()
            if p.is_symlink():rows.append([rel,'link',os.readlink(p)])
            elif p.is_file():rows.append([rel,s.st_size,s.st_mtime_ns,s.st_dev,s.st_ino,digest(p) if hashes else None])
    return hashlib.sha256(encoded(sorted(rows))).hexdigest()


def phase_launch(m,manifest_sha,deadline):
    """Concrete same-file launcher. The bootstrap's OS watchdog is already armed.
    Source/claim/config are sealed before this function. No absent shell wrapper.
    Parent-owned post-exit checks remain inside this phase and its absolute clock.
    """
    import ctypes,gzip,signal,subprocess,zlib
    from suite_driver import descendants
    from suite_bootstrap import strict
    phase=m['phase'];need(phase['deadline_epoch']==deadline and time.time()<deadline,'one-absolute-phase-deadline')
    need(phase['combined_source_cap_bytes'] in {4*1024**2,4608*1024} and
         phase['combined_evidence_cap_bytes']==24*1024**2,'closed-combined-source-evidence-policy')
    need(phase['mode'] in {'controls','full-body','workspace','build','cache'} and phase['seconds']>0,'closed-concrete-phase-mode')
    scopes={'controls':'interface-controls-v5','full-body':'full-body','workspace':'workspace-v5','build':'native-build-v5','cache':'private-cache-v5'}
    need(m['runtime_scope']==scopes[phase['mode']],'one-literal-phase-scope')
    if phase['mode'] in {'build','workspace'}:
        from suite_oracle import native_stage_command
        native_stage_command(m)

    from suite_bootstrap import PHASE_STARTED
    need(deadline<=guard.CUTOFF and 0<deadline-PHASE_STARTED<=phase['seconds'],'finite-phase-duration')
    need(digest(m['python'])==m['tools'][m['python']]['sha256'],'phase-exact-native-Python')
    limits=suite_limits(m);reserve={k:dict(bytes=0,files=0) for k in limits}
    for entry in phase['reserve']:
        k,amount=entry.split('=',1);b,f=map(int,amount.split(':'));need(k in reserve and b>=0 and f>=0,'phase-reservation');reserve[k]=dict(bytes=b,files=f)
    before={k:guard.inventory(guard.ROOT/k) for k in limits}
    for k,(b,f) in limits.items():need(before[k]['bytes']+reserve[k]['bytes']<=b and before[k]['files']+reserve[k]['files']<=f,'phase-prospective-total:'+k)
    checked_snapshot(m,before,reserve)
    if phase['mode'] in {'build','workspace'}:
        target=owned(m['workspace']['target_dir'],guard.ROOT/'target')
        temporary=guard.ROOT/'fixtures'/('native-final31-tmp-'+m['final_pin']) if phase['native_stage']=='test' else target/'owned-tmp'
        owned(temporary,guard.ROOT).mkdir(parents=True,exist_ok=True)
    output=owned(m['evidence_root'],guard.ROOT/'evidence');need(not output.exists(),'fresh-admitted-phase-output-parent');output.mkdir(parents=True)
    phase_memory=owned(phase['memory_path'],output);final=owned(phase['receipt_path'],output)
    need(not phase_memory.exists() and not final.exists(),'fresh-phase-artifacts')
    own=guard.process(os.getpid());need(own and own['rss']>0,'positive-phase-caller-RSS')
    need(ctypes.CDLL(None,use_errno=True).prctl(36,1,0,0,0)==0,'phase-subreaper')
    global_cache=Path('/home/spenser/.cargo');protected=None
    with owned(phase['io_lock_path'],output).open('xb'):pass
    inclusive_receipt(m,phase['owner_path'],dict(kind='live-V5-phase-owner',pid=own['pid'],start=own['start'],claim=m['claim'],final_pin=m['final_pin'],manifest_sha256=manifest_sha,deadline_epoch=deadline,cmdline_sha256=digest(Path('/proc',str(os.getpid()),'cmdline'))),before,reserve)
    bootstrap=owned(Path(__file__).parent/'suite_bootstrap.py',guard.ROOT/'source')
    command=[m['python'],'-I','-S','-B',str(bootstrap),'--phase-body','--manifest',m['manifest_path'],'--sha256',manifest_sha]
    expected=[x.replace('@MANIFEST_PATH@',m['manifest_path']).replace('@MANIFEST_SHA256@',manifest_sha) for x in phase['guard_launch_template']]
    need(command==expected,'exact-same-file-phase-child')
    streams={};tracked={};handles={};registry_lock=threading.RLock();errors=[];samples=0;gaps=[];last=None;stop=threading.Event();root=None;worker=None;closure=[]
    compressed=zlib.compressobj(level=6,wbits=31);memory=phase_memory.open('xb');written=0
    def compressed_write(data):
        nonlocal written
        need(written+len(data)<=phase['memory_max_bytes'],'phase-memory-reservation')
        if data:
            with phase_io(m,False):memory.write(data);memory.flush()
        written+=len(data)
    def members():
        with registry_lock:
            snapshot=guard.processes();owned_pids={pid for pid,start in tracked.items() if pid in snapshot and snapshot[pid]['start']==start}
            owned_pids.update(pid for pid,row in snapshot.items() if row['ppid']==os.getpid())
            while True:
                extra={pid for pid,row in snapshot.items() if row['ppid'] in owned_pids}-owned_pids
                if not extra:break
                owned_pids.update(extra)
            result=[]
            for pid in owned_pids:
                row=guard.process(pid)
                if row is None:continue
                if pid in tracked:need(tracked[pid]==row['start'],'phase-PID-reuse')
                else:
                    fd=os.pidfd_open(pid)
                    again=guard.process(pid)
                    if again is None or again['start']!=row['start']:
                        os.close(fd);raise RuntimeError('phase-pidfd-birth-race')
                    tracked[pid]=row['start'];handles[pid]=fd
                result.append(row)
            return result
    def kill_owned():
        try:members()
        except BaseException as e:errors.append('phase-kill-observation:'+repr(e))
        for pid,fd in list(handles.items()):
            try:signal.pidfd_send_signal(fd,signal.SIGKILL)
            except ProcessLookupError:pass
            except BaseException as e:errors.append('phase-owned-kill:'+repr(e))
    def observe():
        nonlocal samples,last
        due=time.monotonic()
        while not stop.is_set():
            try:
                now=time.time();rows=members();caller=guard.process(os.getpid());need(caller and caller['rss']>0,'live-phase-caller')
                live=[r for r in rows if r['state']!='Z'];need(all(type(r['rss']) is int and r['rss']>0 for r in live),'phase-owned-positive-memory')
                heavy=sum(r['rss'] for r in live);need(heavy<=2*1024**3 and all(r['rss']<=2*1024**3 for r in live),'phase-owned-memory')
                supervisors=caller['rss']+sum(r['rss'] for r in live if (root is not None and r['pid']==root.pid) or str(bootstrap) in r['cmd'])
                need(supervisors<=768*1024**2 and heavy+caller['rss']<3*1024**3,'phase-caller-and-controller-supervisor-plus-owned-memory')
                if last is not None:gaps.append(now-last);need(0<=now-last<=.04,'phase-50Hz-cadence')
                last=now;samples+=1
                row=dict(epoch=now,caller_pid=own['pid'],caller_start=own['start'],caller_rss=caller['rss'],caller_hwm=caller['hwm'],members=rows,heavy_rss=heavy,supervisor_rss=supervisors,zombie_pids=[r['pid'] for r in rows if r['state']=='Z'])
                compressed_write(compressed.compress(encoded(row)+b'\n'))
            except BaseException as e:errors.append(repr(e));kill_owned();return
            due+=.02;stop.wait(max(0,due-time.monotonic()))
    failure=None;result=None;guard_receipt=None;readers=[];raw_outputs={};raw_counts={}
    from suite_bootstrap import EARLY_ROWS,EARLY_STOP,EARLY_THREAD
    if EARLY_STOP is not None:
        EARLY_STOP.set();EARLY_THREAD.join(2);need(not EARLY_THREAD.is_alive(),'early-phase-observer-joined')
        for row in EARLY_ROWS:
            samples+=1
            if last is not None:gaps.append(row['epoch']-last)
            last=row['epoch'];compressed_write(compressed.compress(encoded(row)+b'\n'))
    def drain(stream,name):
        count=0
        try:
            with streams[name] as out:
                while True:
                    block=os.read(stream.fileno(),65536)
                    if not block:break
                    need(count+len(block)<=phase['caller_stream_max_bytes'],'phase-caller-output-bound')
                    with phase_io(m,False):out.write(block);out.flush()
                    count+=len(block)
                os.fsync(out.fileno())
            raw_counts[name]=count
        except BaseException as e:errors.append('phase-output:'+repr(e));kill_owned()
        finally:stream.close()
    try:
        worker=threading.Thread(target=observe,daemon=False);worker.start()
        protected=cache_identity(global_cache)
        need(protected==phase['global_cache_identity_sha256'],'parent-exact-global-cache-file-baseline')
        inner={k:dict(bytes=0,files=0) for k in limits}
        for entry in m['native_guard_policy']['reserve']:
            k,amount=entry.split('=',1);b,f=map(int,amount.split(':'));inner[k]=dict(bytes=b,files=f)
        now=checked_snapshot(m,before,reserve)
        for k in limits:
            extra=phase['memory_max_bytes']+phase['receipt_max_bytes']+2*phase['caller_stream_max_bytes'] if k=='evidence' else 0
            need(now[k]['bytes']+inner[k]['bytes']+extra<=before[k]['bytes']+reserve[k]['bytes'] and now[k]['files']+inner[k]['files']+(3 if k=='evidence' else 0)<=before[k]['files']+reserve[k]['files'],'outer-plus-inner-retained-reservations:'+k)
        for name in ['stdout','stderr']:
            raw_outputs[name]=owned(phase[name+'_path'],output);streams[name]=raw_outputs[name].open('xb')
        root=subprocess.Popen(command,cwd=str(guard.ROOT),stdin=subprocess.DEVNULL,start_new_session=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        for name,stream in [('stdout',root.stdout),('stderr',root.stderr)]:
            t=threading.Thread(target=drain,args=(stream,name),daemon=False);readers.append(t);t.start()
        while root.poll() is None:
            if errors:kill_owned();raise RuntimeError('phase-memory-censor:'+repr(errors))
            need(time.time()<deadline,'absolute-phase-timeout');time.sleep(.005)
        for t in readers:t.join(3)
        need(all(not t.is_alive() for t in readers) and len(raw_counts)==2,'phase-caller-output-EOF-and-joined')
        need(root.returncode==0 and not errors,'phase-child-failure')
        from suite_driver import verify_guard_policy
        guard_receipt=owned(phase['guard_receipt_path'],output);g=strict(guard_receipt.read_bytes(),phase['guard_receipt_max_bytes'])
        verify_guard_policy(m,dict(manifest_path=m['manifest_path'],manifest_sha256=manifest_sha),g)
        need(g['source_pin']==m['final_pin'] and g['claim_id']==m['claim']['claim_id'] and g['ledger_sha256']==m['claim']['ledger_sha256'],'same-phase-guard-source-claim')
        need(g['exit']==0 and not g['censors'] and not g['observation_errors'] and g['monitors_joined'],'accepted-inner-V5-guard')
        need(len(g['closure_observations'])>=2 and all(not r['pids'] and not r['states'] for r in g['closure_observations'][-2:]),'terminal-two-numeric-empty-inner-observations')
        from suite_bootstrap import verify_guard_memory
        memory_proof=verify_guard_memory(g)
        packet_path=owned(phase['packet_path'],output)
        if phase['mode'] in {'build','workspace'}:
            from suite_oracle import native_stage_finalize
            result=native_stage_finalize(m,g)
            packet_path=guard_receipt
        else:
            packet=strict(packet_path.read_bytes(),phase['packet_max_bytes'])
            if phase['mode']=='cache':
                need(packet['kind']=='V5_PRIVATE_C1_COPY_ONLY' and packet['final_pin']==m['final_pin'] and packet['claim']==m['claim'] and packet['manifest_sha256']==m['private_cache']['manifest_sha256'],'actual-C1-copy-report')
                result=packet
            elif phase['mode']=='controls':
                from suite_extensions import finalize_controls
                result=finalize_controls(m,packet,g)
            else:
                from suite_main import finalize
                result=finalize(m,packet,str(guard_receipt))
        result=dict(result,inner_memory=memory_proof)
        need(cache_identity(global_cache)==protected,'protected-global-cache-file-change')
    except BaseException as e:failure=repr(e);kill_owned()
    finally:
        # Alarm expiry is irrevocably FAILED. Disable only to permit bounded
        # process reaping; this path can never produce a successful receipt.
        if time.time()>=deadline:signal.setitimer(signal.ITIMER_REAL,0);failure=failure or 'phase-deadline-expired'
        until=time.monotonic()+10
        while time.monotonic()<until:
            rows=members()
            if any(r['state']!='Z' for r in rows):kill_owned()
            if root is not None:
                try:root.wait(timeout=.1)
                except subprocess.TimeoutExpired:pass
            for pid in list(tracked):
                try:os.waitpid(pid,os.WNOHANG)
                except ChildProcessError:pass
            rows=members();closure.append(dict(epoch=time.time(),pids=[r['pid'] for r in rows],states=[r['state'] for r in rows]))
            if len(closure)>=2 and not closure[-1]['pids'] and not closure[-2]['pids']:break
            time.sleep(.02)
        for t in readers:t.join(3)
        need(all(not t.is_alive() for t in readers),'phase-caller-output-readers-joined')
        stop.set()
        if worker is not None:worker.join(5)
        need(worker is None or not worker.is_alive(),'phase-caller-memory-observer-joined')
        compressed_write(compressed.flush());memory.flush();os.fsync(memory.fileno());memory.close()
        for fd in handles.values():os.close(fd)
    need(len(closure)>=2 and all(not r['pids'] and not r['states'] for r in closure[-2:]),'phase-terminal-two-empty')
    need(not failure and not errors and samples>=2 and time.time()<deadline,'phase-complete-within-deadline:'+str(failure))
    from suite_bootstrap import verify_guard_memory
    phase_memory_proof=verify_guard_memory(dict(memory_samples_path=str(phase_memory),memory_samples_sha256=digest(phase_memory),tracked_identity={str(pid):start for pid,start in tracked.items()},memory_sample_count=samples,heavy_rss_limit=2*1024**3,process_rss_limit=2*1024**3,supervisor_rss_limit=768*1024**2,native_host_rss_limit=3*1024**3))
    value=dict(kind='V5_PHASE_FINAL',status='PASSED',scope=m['runtime_scope'],engine_qualification=phase['mode']=='full-body',unit_qualification=phase['mode']=='workspace' and phase['native_stage']=='test',
        manifest_sha256=manifest_sha,final_pin=m['final_pin'],claim=m['claim'],phase_caller=own,phase_memory_path=str(phase_memory),phase_memory_sha256=digest(phase_memory),
        phase_memory_samples=samples,phase_memory_proof=phase_memory_proof,phase_caller_outputs={k:dict(path=str(p),bytes=p.stat().st_size,sha256=digest(p)) for k,p in raw_outputs.items()},phase_max_interval=max(gaps),phase_memory_observer_joined=True,phase_owned_closure=closure,guard_receipt_sha256=digest(guard_receipt),packet_sha256=digest(packet_path),
        deadline_epoch=deadline,completed_epoch=time.time(),global_cache_identity_sha256=protected,result=result)
    result=inclusive_receipt(m,final,value,before,reserve)
    need(time.time()<deadline,'deadline-through-own-receipt-and-inclusive-census')
    # No output or census follows cancellation. Exit is the external completion
    # witness; failure before this point leaves the prefix, never a qualified run.
    signal.setitimer(signal.ITIMER_REAL,0)
    return result

@contextmanager
def phase_io(m,exclusive):
    import fcntl
    p=owned(m['phase']['io_lock_path'],guard.ROOT/'evidence')
    fd=os.open(p,os.O_RDWR|os.O_NOFOLLOW)
    try:
        fcntl.flock(fd,fcntl.LOCK_EX if exclusive else fcntl.LOCK_SH)
        yield
    finally:os.close(fd)
