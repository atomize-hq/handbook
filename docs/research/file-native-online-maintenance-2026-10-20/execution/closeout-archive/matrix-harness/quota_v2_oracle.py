import copy,hashlib,os,re,stat
from pathlib import Path
from outcome_oracle import strict,intent
from verify_operational_v2 import typed_pretty as pretty
FORMAT_SHA256='3172ba1b1e78d048c2eb168b02ca524b4d3979abaa6528aaebb149a8a2c3b824'
LIMIT=32*1024**3
MARGIN=128*1024
HASH=re.compile(r'[0-9a-f]{64}')
ID=re.compile(r'[A-Za-z0-9_-]{1,99}')
RKEYS=['request','intent','base','bytes','state','used','candidate_digest']
CKEYS={'purpose','version','store_id','attempt','expected','request','intent','graph','projection','changes','changed','removed','generation_manifest','checkpoint','packet_ref','replay_objects','replay_files'}
TKEYS=['version','store_id','attempt','candidate_digest','request','intent','expected','reason','used_bytes','original_allowance','charge_bytes','handoff_margin','control_stage']
VKEYS=['version','store_id','active','legacy','terminal_count','terminal_bytes']
def uint(x):
    assert type(x) is int and 0<=x<2**64,'quota-u64'
    return x
def closed(x,keys):
    assert type(x) is dict and set(x)==set(keys),'quota-closed-shape'
def selector(s):
    if s is not None:
        closed(s,{'publication','digest'})
        assert type(s['publication']) is str and ID.fullmatch(s['publication'])
        assert type(s['digest']) is str and HASH.fullmatch(s['digest'])
    return s
def registry(raw,store_id):
    assert len(raw)<=65536,'registry-byte-cap'
    v=strict(raw);assert type(v) is dict
    if type(v.get('version')) is not int or not 0<=v['version']<2**64:return dict(version=1,active=v)
    closed(v,VKEYS)
    assert type(v['version']) is int and v['version']==2 and v['store_id']==store_id,'registry-version'
    a=v['active'];assert type(a) is dict and len(a)<=8,'active-cap'
    legacy=v['legacy'];assert type(legacy) is list and len(legacy)<=8 and len(set(legacy))==len(legacy) and set(legacy)<=set(a),'legacy-subset'
    for name,r in a.items():
        assert ID.fullmatch(name);closed(r,RKEYS)
        assert type(r['request']) is str and len(r['request'].encode())<=256 and HASH.fullmatch(r['intent'])
        selector(r['base']);uint(r['bytes']);uint(r['used']);assert r['used']<=r['bytes']
        assert r['state'] in {'preparing','sealed','sealed-preselection','selection-started','abandoned'}
        assert not(name in legacy and r['state'] in {'sealed-preselection','selection-started'}),'legacy-phase'
        assert r['candidate_digest'] is None or HASH.fullmatch(r['candidate_digest'])
    n=uint(v['terminal_count']);z=uint(v['terminal_bytes'])
    assert n<=4096 and ((n==0)==(z==0)) and z>=n*MARGIN,'terminal-aggregate-cap'
    assert sum(r['bytes'] for r in a.values())+z+MARGIN<=LIMIT,'aggregate-byte-cap'
    return v
def candidate(raw,attempt,store_id):
    c=strict(raw);closed(c,CKEYS)
    assert type(c['version']) is int and c['version']==2 and c['purpose'] in {'publication','packet'}
    assert c['store_id']==store_id and c['attempt']==attempt and ID.fullmatch(attempt),'candidate-owner'
    selector(c['expected']);assert intent(c['request'])==c['intent'] and HASH.fullmatch(c['intent']),'candidate-intent'
    g=c['graph'];closed(g,{'version','consumer','budget','cost','count','tree','fallback'})
    assert type(g['version']) is int and g['version']==1 and type(g['fallback']) is bool
    for k in ['budget','cost','count']:uint(g[k])
    assert g['tree'] is None or HASH.fullmatch(g['tree'])

    if c['request']['base'] is not None:assert c['request']['base']==c['expected'],'explicit-base'
    assert type(c['changed']) is list and type(c['removed']) is list
    assert type(c['replay_objects']) is list and len(set(c['replay_objects']))==len(c['replay_objects'])
    assert all(type(x) is str and HASH.fullmatch(x) for x in c['replay_objects'])
    assert type(c['replay_files']) is dict
    for name,digest in c['replay_files'].items():
        assert type(name) is str and name.startswith(('originals/','derived/')) and not any(x in {'','.','..'} for x in name.split('/')) and '\\' not in name and ':' not in name
        assert type(digest) is str and HASH.fullmatch(digest)
    return c
def receipt(attempt,r,store_id):
    return dict(version=1,store_id=store_id,attempt=attempt,candidate_digest=r['candidate_digest'],request=r['request'],intent=r['intent'],expected=r['base'],reason='definitive-expected-publication-conflict',used_bytes=r['used'],original_allowance=r['bytes'],charge_bytes=r['used']+MARGIN,handoff_margin=MARGIN,control_stage='attempts/'+attempt+'/terminal-control-stage')
class NativeSnapshot:
    def __init__(self,root):
        self.root=Path(root).absolute();self.files={};self.directories=[];self.native=True
        for p in [self.root,*self.root.parents]:
            s=p.lstat();assert not stat.S_ISLNK(s.st_mode) and not getattr(s,'st_file_attributes',0)&1024,'ancestor-reparse'
        for base,dirs,files in os.walk(self.root,followlinks=False):
            d=Path(base);self.directories.append(d.relative_to(self.root).as_posix())
            for name in dirs+files:
                p=d/name;s=p.lstat();assert not stat.S_ISLNK(s.st_mode) and not getattr(s,'st_file_attributes',0)&1024,'tree-reparse'
                if name not in files:assert stat.S_ISDIR(s.st_mode);continue
                assert stat.S_ISREG(s.st_mode);digest=hashlib.sha256()
                with p.open('rb') as f:
                    opened=os.fstat(f.fileno());assert (opened.st_dev,opened.st_ino,opened.st_size,opened.st_mtime_ns)==(s.st_dev,s.st_ino,s.st_size,s.st_mtime_ns),'identity-race'
                    for block in iter(lambda:f.read(1024*1024),b''):digest.update(block)
                    after=os.fstat(f.fileno());assert (after.st_size,after.st_mtime_ns)==(opened.st_size,opened.st_mtime_ns),'file-changed'
                assert s.st_ino and s.st_nlink>=1,'missing-native-ID'
                self.files[p.relative_to(self.root).as_posix()]=dict(bytes=s.st_size,sha256=digest.hexdigest(),identity=[s.st_dev,s.st_ino],links=s.st_nlink)
        self.directories.sort()
    def read(self,name,limit):
        assert name in self.files,'missing-private-file'
        assert self.files[name]['bytes']<=limit,'read-cap'
        with (self.root/name).open('rb') as f:b=f.read(limit+1);s=os.fstat(f.fileno())
        assert len(b)==self.files[name]['bytes'] and len(b)<=limit and hashlib.sha256(b).hexdigest()==self.files[name]['sha256'] and [s.st_dev,s.st_ino]==self.files[name]['identity'],'snapshot-read-changed'
        return b
    def unchanged(self):
        other=NativeSnapshot(self.root)
        assert self.files==other.files and self.directories==other.directories,'quota-tree-mutated'
def verify(decoder,snapshot=None,evidence=None):
    if snapshot is None:
        with (Path(decoder.root)/'RESERVATIONS.json').open('rb') as f:raw=f.read(65537)
    else:raw=snapshot.read('RESERVATIONS.json',65536)
    v=registry(raw,decoder.config['store_id'])
    if v['version']==1:return dict(version=1,active=v['active'],semantic_owners=None,proof=dict(version=1,new_terminal_claim=False))
    snap=snapshot or NativeSnapshot(decoder.root);proofs=evidence or {};files=snap.files
    assert type(proofs) is dict and set(proofs)<={'owners','stages','qualified_schedule_sha256'},'evidence-shape'
    assert all(type(proofs.get(k,{})) is dict for k in ['owners','stages'])
    qualified=proofs.get('qualified_schedule_sha256',[])
    assert type(qualified) is list and len(qualified)<=8192 and len(set(qualified))==len(qualified) and all(type(x) is str and HASH.fullmatch(x) for x in qualified)
    assert snap.read('RESERVATIONS.json',65536)==raw,'registry-census-race'
    selected=strict(snap.read('SELECTED.json',65536));selector(selected)
    chain={p['id']:(s,p) for s,p in decoder.accepted_chain().values()}
    assert selected['publication'] in chain and chain[selected['publication']][0]==selected
    accepted={p['attempt']:(s,p) for s,p in chain.values()};assert len(accepted)==len(chain),'accepted-attempt-alias'
    active=v['active'];legacy=set(v['legacy']);receipts={};pending={};owners={};stages={};totals={}
    def read(name,limit):return snap.read(name,limit)
    def member(s):
        selector(s);assert s is not None and s['publication'] in chain and chain[s['publication']][0]==s,'base-membership'
        return chain[s['publication']][1]
    def check_owner(attempt,r,terminal=False):
        b=read('attempts/'+attempt+'/candidate.json',128*1024*1024)
        assert hashlib.sha256(b).hexdigest()==r['candidate_digest'],'candidate-digest'
        c=candidate(b,attempt,v['store_id'])
        assert c['graph']['consumer']==decoder.config.get('consumer','project'),'private-consumer'
        assert (c['request']['request'],c['intent'],c['expected'])==(r['request'],r['intent'],r['base']),'candidate-binding'
        if c['expected'] is not None:member(c['expected'])
        for d in c['replay_objects']:assert hashlib.sha256(read('objects/'+d[:2]+'/'+d,128*1024*1024)).hexdigest()==d,'replay-object'
        for n,d in c['replay_files'].items():assert hashlib.sha256(read(n,128*1024*1024)).hexdigest()==d,'replay-file'
        if c['generation_manifest'] is not None:
            assert decoder.config['mode']=='B' and HASH.fullmatch(c['generation_manifest']),'generation-mode-hash'
            prefix='generations/'+attempt+'/'
            raw=read(prefix+'MANIFEST.json',128*1024*1024)
            assert hashlib.sha256(raw).hexdigest()==c['generation_manifest'],'generation-manifest-digest'
            manifest=strict(raw);assert type(manifest) is dict,'generation-manifest-shape'
            for path,f in manifest.items():
                assert type(path) is str and path and not path.startswith('/') and '\\' not in path and ':' not in path and all(x not in {'','.','..'} for x in path.split('/')),'generation-path'
                assert type(f) is dict and {'source','digest'}<=set(f)<={'source','digest','projection_profile'},'generation-spec'
                source=f['source'];assert type(source) is str and source.startswith(('originals/','derived/')) and '\\' not in source and ':' not in source and all(x not in {'','.','..'} for x in source.split('/')),'generation-source-path'
                assert type(f['digest']) is str and HASH.fullmatch(f['digest'])
                if f.get('projection_profile') is not None:assert uint(f['projection_profile'])<2**32
                for name in [prefix+path,source]:assert hashlib.sha256(read(name,128*1024*1024)).hexdigest()==f['digest'],'generation-file'
            actual={n[len(prefix):] for n in files if n.startswith(prefix)}
            assert actual==set(manifest)|{'MANIFEST.json'},'generation-complete-namespace'
        elif decoder.config['mode']=='B' and c['purpose']=='publication':raise AssertionError('B-owner-generation-missing')
        if terminal:
            assert c['purpose']=='publication' and c['request']['packet'] is False and c['checkpoint'] is None and c['packet_ref'] is None,'terminal-kind'
            assert attempt not in accepted,'terminal-selected-owner'
            p=member(c['expected']);assert c['expected']!=selected and p['sequence']<chain[selected['publication']][1]['sequence'],'historical-base'
        owners[attempt]=copy.deepcopy(r);owners[attempt]['state']='sealed'
        return c
    for attempt,r in active.items():
        if attempt in accepted:
            p=accepted[attempt][1];o=p['allocation_owner']
            assert o['attempt']==attempt and o['includes_intermediates_and_stages'] is True and o['conservative_bytes']>=r['bytes'],'accepted-allowance'
            assert (p['intent'],p['request'])==(r['intent'],r['request']) and (r['state']=='sealed' if attempt in legacy else r['state']=='selection-started'),'accepted-phase'
        if r['state'] in {'sealed','sealed-preselection','selection-started'}:check_owner(attempt,r)
    for name in sorted(files):
        if not name.startswith('abandoned/'):continue
        a=name[10:];assert ID.fullmatch(a),'receipt-path';b=read(name,8192)
        if b==b'fenced':
            assert a in active and active[a]['state']=='abandoned','uncharged-fence'
            continue
        t=strict(b);closed(t,TKEYS);assert pretty({k:t[k] for k in TKEYS})==b,'receipt-exact-bytes'
        assert type(t['version']) is int and t['version']==1 and t['store_id']==v['store_id'] and t['attempt']==a,'receipt-owner'
        for k in ['candidate_digest','intent']:assert type(t[k]) is str and HASH.fullmatch(t[k])
        assert type(t['request']) is str and len(t['request'].encode())<=256
        for k in ['used_bytes','original_allowance','charge_bytes','handoff_margin']:uint(t[k])
        assert t['handoff_margin']==MARGIN and t['used_bytes']+MARGIN==t['charge_bytes']<=t['original_allowance']<=LIMIT-MARGIN,'terminal-charge'
        assert t['reason']=='definitive-expected-publication-conflict' and t['control_stage']=='attempts/'+a+'/terminal-control-stage','reason-stage'
        r=dict(request=t['request'],intent=t['intent'],base=t['expected'],bytes=t['original_allowance'],state='sealed-preselection',used=t['used_bytes'],candidate_digest=t['candidate_digest'])
        check_owner(a,r,True)
        if a in active:
            assert a not in legacy and active[a]==r,'terminal-phase'
            pending[a]=t
        else:receipts[a]=t
        assert files[name]['links']==1,'receipt-hardlink'
    assert len(receipts)==v['terminal_count'] and sum(t['charge_bytes'] for t in receipts.values())==v['terminal_bytes'],'aggregate-exact'
    for a in set(active)|set(receipts):
        r=active.get(a);t=receipts.get(a) or pending.get(a)
        before=proofs.get('owners',{}).get(a)
        if before is not None:assert t and before=={k:t[k] for k in ['used_bytes','original_allowance','candidate_digest','request','intent','expected']},'before-owner'
        for suffix,prefix,limit in [('terminal-receipt-stage','terminal-receipt-stage',8192),('terminal-control-stage','terminal-control',65536)]:
            name='attempts/'+a+'/'+suffix
            if name not in files:continue
            assert (r and a not in legacy and r['state']=='sealed-preselection') or t,'unowned-stage'
            f=files[name];assert f['links']==1 and f['bytes']<=limit,'stage-alias-size';stage=read(name,limit)
            p=proofs.get('stages',{}).get(name);assert type(p) is dict,'stage-schedule-required'
            closed(p,{'attempt','candidate_digest','sha256','checkpoint','source_sha256','censored_schedule_sha256'})
            expected_t=t or receipt(a,r,v['store_id'])
            assert p['attempt']==a and p['candidate_digest']==expected_t['candidate_digest'] and p['sha256']==f['sha256'] and p['source_sha256']==FORMAT_SHA256,'stage-binding'
            assert p['checkpoint'] in {prefix+'-created',prefix+'-before-write',prefix+'-partial',prefix+'-write',prefix+'-staged'} and HASH.fullmatch(p['censored_schedule_sha256']),'stage-checkpoint'
            assert p['censored_schedule_sha256'] in proofs.get('qualified_schedule_sha256',[]),'stage-schedule-unqualified'
            if suffix=='terminal-receipt-stage':expected=pretty({k:expected_t[k] for k in TKEYS})
            else:
                assert r is not None and t is not None,'control-stage-phase'
                nextv=copy.deepcopy(v);nextv['active'].pop(a);nextv['terminal_count']+=1;nextv['terminal_bytes']+=t['charge_bytes']
                nextv={k:nextv[k] for k in VKEYS}
                nextv['active']={a:{k:z[k] for k in RKEYS} for a,z in sorted(nextv['active'].items())}
                nextv['legacy']=sorted(nextv['legacy']);expected=pretty(nextv)
            assert expected.startswith(stage),'stage-prefix'
            phase=p['checkpoint'].removeprefix(prefix)
            if phase=='-before-write':assert not stage
            if phase=='-partial':assert len(stage)==max(1,len(expected)//2),'stage-partial-length'
            if phase in {'-write','-staged'}:assert stage==expected,'stage-complete-length'
            stages[name]=dict(attempt=a,bytes=f['bytes'],sha256=f['sha256'],schedule=p['censored_schedule_sha256'])
        private=[n for n in files if n.startswith(('attempts/'+a+'/','generations/'+a+'/'))]
        assert t is None or not any(n.startswith('attempts/'+a+'/allocations/') for n in private),'postselection-allocation'
        size=sum(files[n]['bytes'] for n in private)+files.get('abandoned/'+a,{}).get('bytes',0)
        allowance=t['charge_bytes'] if t else r['bytes'] if r['state'] in {'preparing','abandoned'} else r['used']
        assert size<=allowance,'private-undercharged'
        totals[a]=dict(namespace_files=len(private),logical_bytes=size,charged_bytes=allowance)
    assert set(proofs.get('owners',{}))<=set(receipts)|set(pending),'unbound-owner-proof'
    assert set(proofs.get('stages',{}))==set(stages),'unbound-stage-proof'
    allowners=set(active)|set(receipts)|set(accepted)
    for n in files:
        if n.endswith(('/terminal-control-stage','/terminal-receipt-stage')):assert n in stages,'unattributed-stage'
        if n.startswith(('attempts/','generations/')):assert n.split('/')[1] in allowners,'unattributed-owner'
    identities={};aliases=[]
    for n,f in files.items():
        key=tuple(f['identity']);old=identities.get(key)
        if old:
            assert (old[1]['bytes'],old[1]['sha256'])==(f['bytes'],f['sha256']),'alias-disagreement'
            aliases.append([old[0],n])
        else:identities[key]=(n,f)
    critical=set(stages)|{'abandoned/'+a for a in set(receipts)|set(pending)}
    assert all(not(critical&set(pair)) for pair in aliases),'control-alias'
    census=dict(namespace_files=len(files),logical_bytes=sum(f['bytes'] for f in files.values()),native_identities=len(identities),unique_identity_logical_bytes=sum(f['bytes'] for _,f in identities.values()),alias_pairs=aliases,native_identity_observed=snap.native,allocated_bytes=None,directory_device_writes=None)
    digest=hashlib.sha256(pretty(dict(files=dict(sorted(files.items())),directories=snap.directories))).hexdigest()
    return dict(version=2,active=active,semantic_owners=owners,proof=dict(profile='quota-v2-retention',format_sha256=FORMAT_SHA256,active=len(active),legacy=len(legacy),terminal_count=len(receipts),terminal_bytes=v['terminal_bytes'],published_receipts_still_active=len(pending),owned_stages=stages,private_totals=totals,physical_census=census,tree_manifest_sha256=digest,used_counter='attested used; IO history unknown',before_owner_proofs=len(proofs.get('owners',{})),reclaimed_bytes=0))
