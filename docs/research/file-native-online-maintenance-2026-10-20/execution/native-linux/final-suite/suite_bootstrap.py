"""Closed Linux final-suite source loader. No workload is authorized by this file."""
import argparse, hashlib, importlib.abc, importlib.machinery, json, os, sys
from pathlib import Path

ROOT=Path('/home/spenser/file-native-online-linux-20261020')
GUARD='4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9'
OWNER='dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce'

def need(test,message):
    if not test: raise RuntimeError(message)

def encoded(value):
    return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode()

def digest(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1048576),b''): h.update(b)
    return h.hexdigest()

def strict(raw,limit=262144):
    need(len(raw)<=limit,'json-byte-bound')
    def pairs(items):
        answer={}
        for k,v in items:
            need(k not in answer,'duplicate-json-key');answer[k]=v
        return answer
    def bad(v): raise ValueError('nonfinite:'+v)
    return json.loads(raw,object_pairs_hook=pairs,parse_constant=bad)

def owned(path,base=ROOT):
    p=Path(path);need(p.is_absolute() and p.is_relative_to(base) and '..' not in p.parts,'owned-absolute-path')
    need(p.resolve()==p,'canonical-owned-path')
    current=Path(p.anchor)
    for part in p.parts[1:]:
        current/=part;need(not current.is_symlink(),'no-symlink-component')
    return p

class ClosedImports(importlib.abc.MetaPathFinder):
    def __init__(self,files): self.files=files
    def find_spec(self,fullname,path=None,target=None):
        spec=importlib.machinery.PathFinder.find_spec(fullname,path)
        if spec is not None and spec.origin and spec.origin not in ('frozen','built-in'):
            p=Path(spec.origin).absolute()
            if p.is_relative_to(ROOT):
                need(str(p) in self.files and digest(owned(p))==self.files[str(p)]['sha256'],'unbound-owned-import:'+fullname)
        return spec

def verify_sources(m):
    files=m['source_files'];need(len(files)==38 and hashlib.sha256(json.dumps(sorted(files),separators=(',',':')).encode()).hexdigest()=='4afeef6f8b3891bfe869732b8d9beb83d096491cc8eafde6f6b36d29334e0b7d','closed38-source-paths')
    if m.get('source_roots'):
        normalized={}
        for name,row in files.items():
            if type(row) is list:
                need(len(row)==2,'compact-source-record');row=dict(bytes=row[0],sha256=row[1])
            alias,relative=name.split('/',1)
            base=owned(m['source_roots'][alias],ROOT/'source')
            p=owned(base/relative,base)
            need(str(p) not in normalized,'unique-source-closure-path')
            normalized[str(p)]=row
        files=normalized;m['source_files']=files
    for name,row in files.items():
        p=owned(name,ROOT/'source');need(set(row)=={'bytes','sha256'},'source-record')
        need(p.stat().st_size==row['bytes'] and digest(p)==row['sha256'],'source-changed:'+name)
    for n,h in [('native_guard.py',GUARD),('native_driver.py',OWNER)]:
        p=ROOT/'source'/n;need(str(p) in files and files[str(p)]['sha256']==h,'accepted-source:'+n)
    required={'suite_bootstrap.py','suite_driver.py','suite_adapter.py','suite_oracle.py','suite_packing.py',
              'suite_recovery.py','suite_extensions.py','suite_ordinary.py','suite_units.py','suite_main.py'}
    folder=Path(__file__).resolve().parent
    need(all(str(folder/n) in files for n in required),'all-actual-suite-implementation-required')
    for folder_name in m['import_directories']: owned(folder_name,ROOT/'source')
    return files

def load(path,expected):
    path=owned(path,ROOT/'source');need(digest(path)==expected,'parent-manifest-binding')
    m=strict(path.read_bytes());need(m['version']==1 and m['platform']=='Linux','suite-format-platform')
    files=verify_sources(m)
    # Runtime remains NO_GO until the parent seals pin, body, all hooks and oracle.
    need(m['go'] is True and m['final_pin'] and len(m['final_pin'])==40,'parent-runtime-GO-required')
    scope=m.get('runtime_scope','full-body');need(scope in {'full-body','interface-controls-v4'},'closed-runtime-scope')
    if scope=='interface-controls-v4':need(m['engine_case_qualification'] is False,'controls-never-engine-qualification')
    else:
        need(m['candidate']['source_pin']==m['final_pin'],'same-final-candidate-pin')
        need(digest(owned(m['candidate']['path'],ROOT/'target'))==m['candidate']['sha256'],'candidate-binary-binding')
        need(m['oracle']['reviewed'] is True and m['oracle']['final_pin']==m['final_pin'],'final-independent-oracle-release')
    need(m['claim']['ledger_sha256'] and m['claim']['claim_id'],'sealed-runtime-claim')
    m['manifest_path']=str(path)
    m['manifest_sha256']=expected
    growth=owned(m['growth_path'],ROOT/'source')
    need(str(growth) in files and digest(growth)==files[str(growth)]['sha256'],'bound-growth-data')
    m['growth']=strict(growth.read_bytes(),65536)
    return m

def main():
    p=argparse.ArgumentParser();p.add_argument('--manifest',required=True);p.add_argument('--sha256',required=True)
    ns=p.parse_args();need(sys.platform=='linux' and __debug__ and sys.flags.isolated and sys.flags.no_site,'isolated-native-python')
    m=load(ns.manifest,ns.sha256)
    sys.meta_path.insert(0,ClosedImports(m['source_files']))
    for item in reversed(m['import_directories']):sys.path.insert(0,item)
    os.environ['FILE_NATIVE_ORACLE_RESEARCH']=str(owned(m['oracle']['research_root'],ROOT/'source'))
    os.environ['PYTHONDONTWRITEBYTECODE']='1'
    if m.get('runtime_scope')=='interface-controls-v4':
        from suite_extensions import interface_controls_v4
        interface_controls_v4(m,ns.sha256)
    else:
        from suite_main import run
        run(m,ns.sha256)

def verify_raw_packet(m,packet):
    """Read every retained native call, chunk and direct closure; no subprocess."""
    import base64
    starts={};calls={};chunks={};wires={};commands=[];closures={};registry=[]
    for name,row in packet['raw_files'].items():
        p=owned(name,ROOT/'evidence')
        need(p.stat().st_size==row['bytes'] and digest(p)==row['sha256'],'full-raw-artifact')
        with p.open('rb') as stream:
            for line in stream:
                event=strict(line,16*1024**2);kind=event.get('kind')
                if kind=='native-positive-start':
                    token=event['control_token'];need(token not in starts,'duplicate-native-start');starts[token]=event
                elif kind=='native-call':
                    token=event['call_token'];need(token not in calls,'duplicate-native-call');calls[token]=event
                elif kind=='native-raw-chunk':
                    key=(event['call'],event['stream']);need(event['stream'] in {'stdout','stderr'},'raw-stream-name')
                    data=base64.b64decode(event['bytes_b64'],validate=True)
                    prior=chunks.setdefault(key,bytearray());need(event['offset']==len(prior),'contiguous-full-native-stream')
                    prior.extend(data)
                elif kind=='native-command':commands.append(event)
                elif kind=='suite-native-input-wire':
                    token=event['call'];need(token not in wires,'one-native-input-wire');wires[token]=event
                elif kind=='case-closure':
                    case_id=event['case_id'];need(case_id not in closures,'duplicate-case-closure');closures[case_id]=event
                    c=event['closure']
                    while len(registry)<len(c['registries']):registry.append({})
                    need(len(registry)==len(c['registries']),'stable-driver-registry-order')
                    for old,delta in zip(registry,c['registries']):
                        need(not(set(old)&set(delta)),'immutable-closed-registry-row');old.update(delta)
                    need(hashlib.sha256(encoded(registry)).hexdigest()==c['global_registry_sha256'] and
                         sum(len(r) for r in registry)==c['global_registry_entries'],'lossless-exact-global-closure-reconstruction')
    need(starts and set(starts)==set(calls),'every-native-start-has-one-final-row')
    need(set(wires)==set(calls),'exact-wire-artifact-for-every-native-call')
    need(registry==packet['final_native_registry'] and all(r['closed'] and r['resources_closed'] and r['callback_joined']
         for rows in registry for r in rows.values()),'retained-full-final-registry')
    need(all(key[0] in calls for key in chunks),'no-orphan-raw-chunks')
    for token,row in calls.items():
        start=starts[token]
        need(row['guard_nonce']==packet['guard_nonce'] and row['source_pin']==m['final_pin'] and
             row['suite_pin']==m['final_pin'] and row['claim_id']==m['claim']['claim_id'],'native-call-claim-source')
        need(start['pid']==row['pid'] and start['process_start_ticks']==row['process_start_ticks'] and
             start['input']==row['input'] and start['env']==row['env'],'same-retained-native-call-identity')
        need(start['before_stdin_delivery'] is True and row['observed_ns']<=row['stdin_ns']<=row['done_ns'],'positive-native-start-before-input')
        need(row['initial_blocked_rss']>0 and row['actual_exec_rss']>0 and
             row['observed_peak_rss']>0 and row['observed_peak_rss']<=row['rss_limit'],'real-positive-role-RSS')
        need(row['engine_root_reaped'] and row['engine_monitor_joined'] and row['python_callback_joined'] and
             not row['violations'],'uncensored-native-call-closure')
        if row['binary_role']=='candidate':
            need(row['executable_sha256']==m['candidate']['sha256'] and row['binary_source_pin']==m['final_pin'],'same-final-ELF')
        else:
            binding=m['compatibility'][row['historical_case_id']]
            need(row['binary_role']=='historical' and row['executable_sha256']==binding['producer_binary_sha256'] and
                 row['binary_source_pin']==binding['producer_source_pin']!=m['final_pin'],'distinct-granted-historical-ELF')
        wire=base64.b64decode(wires[token]['bytes_b64'],validate=True)
        need(wires[token]['before_stdin_delivery'] and len(wire)==wires[token]['bytes'] and
             hashlib.sha256(wire).hexdigest()==wires[token]['sha256'] and strict(wire,4*1024**2)==row['input'],'native-raw-wire-json-binding')
        need(row['input_fully_delivered'] and row['stdin_closed'] and not row['input_broken_pipe'] and
             len(wire)==row['input_bytes_expected']==row['input_bytes_sent'],'complete-engine-input')
        need(hashlib.sha256(wire).hexdigest()==row['input_wire_sha256']==row['input_written_prefix_sha256'],'exact-native-input-wire')
        for stream in ['stdout','stderr']:
            raw=bytes(chunks.get((token,stream),b''))
            need(hashlib.sha256(raw).hexdigest()==row[stream+'_sha256'] and
                 raw.decode(errors='replace')==row[stream],'complete-native-raw-stream')
        if row['intentional_interrupt']:
            need(row['intentional_interrupt'] in {'cancel','death'} and row['exit'] in {-9,-15},'exact-intentional-native-termination')
        elif row['exit']==86:need(row['env'] and row['env'].get('EXP_EXIT'),'source-declared-exit86')
        else:need(row['exit'] in {0,2},'unexpected-native-exit')
    for row in commands:
        need(row['guard_nonce']==packet['guard_nonce'] and row['qualification_pin']==m['final_pin'],'same-native-command-stage')
        need(row['root_reaped'] and row['watcher_joined'] and not row['errors'],'native-command-closure')
        proof=row['actual_exec']
        need(proof and proof['before_stdin_delivery'] and proof['rss']>0 and proof['inode']>0 and
             proof['sha256']==row['binary_sha256']==m['tools'][row['argv'][0]]['sha256'],'actual-command-ELF-inode-RSS-proof')
        need(row['initial_blocked_rss']>0 and row['peak_rss']>0 and row['peak_rss']<=row['rss_limit'],'native-command-role-memory')
        raw=base64.b64decode(row['stdin_bytes_b64'],validate=True)
        need(row['input_fully_delivered'] and row['stdin_closed'] and
             len(raw)==row['stdin_bytes']==row['input_bytes_sent'] and hashlib.sha256(raw).hexdigest()==row['stdin_sha256'],'full-command-input')
        for stream in ['stdout','stderr']:
            raw=base64.b64decode(row[stream+'_bytes_b64'],validate=True)
            need(hashlib.sha256(raw).hexdigest()==row[stream+'_sha256'] and raw.decode(errors='replace')==row[stream],'full-command-raw-stream')
    for row in packet['observations']:
        case_id=row['case_id'];need(case_id in closures,'actual-direct-case-closure')
        end=closures[case_id];c=end['closure']
        need(end['failure'] is None and c['processes_closed'] and c['handles_closed'] and
             c['python_callbacks_joined'] and not c['errors'],'case-not-positively-closed')
    return dict(engine_calls=len(calls),generic_native_commands=len(commands),direct_case_closures=len(closures),
                all_wire_hashes_and_raw_streams_verified=True)


def verify_guard_memory(g):
    import zlib
    path=owned(g['memory_samples_path'],ROOT/'evidence')
    need(digest(path)==g['memory_samples_sha256'],'guard-memory-file-hash')
    decoder=zlib.decompressobj(16+zlib.MAX_WBITS)
    pending=b'';count=0;prior=None;maximum=0.;positive=0;zombies=0
    def sample(raw):
        nonlocal count,prior,maximum,positive,zombies
        row=strict(raw,1024**2);count+=1
        need(count<=g['memory_sample_count'],'unexpected-extra-memory-row')
        epoch=row['epoch']
        if prior is not None:
            gap=epoch-prior;need(0<=gap<=.04,'raw-memory-cadence-not-qualified');maximum=max(maximum,gap)
        prior=epoch
        live=[];z=set(row['zombie_pids']);zombies+=len(z)
        for member in row['members']:
            need(g['tracked_identity'].get(str(member['pid']))==member['start'],'retained-guard-PID-start')
            if member['pid'] in z:
                need(member['rss'] is None or member['rss']>=0,'explicit-zombie-RSS')
            else:
                need(type(member['rss']) is int and member['rss']>0 and
                     type(member['hwm']) is int and member['hwm']>=member['rss'],'positive-live-native-RSS')
                live.append(member['rss']);positive+=1
        need(sum(live)==row['heavy_rss'] and row['heavy_rss']<=g['heavy_rss_limit'] and
             all(r<=g['process_rss_limit'] for r in live),'actual-heavy-and-process-memory')
        need(0<row['supervisor_rss']<=g['supervisor_rss_limit'] and
             row['heavy_rss']+row['supervisor_rss']<g['native_host_rss_limit'],'actual-supervisor-host-memory')
    with path.open('rb') as stream:
        for block in iter(lambda:stream.read(65536),b''):
            need(not decoder.eof,'gzip-trailing-member-or-bytes')
            incoming=block
            while incoming:
                piece=decoder.decompress(incoming,1024**2);incoming=decoder.unconsumed_tail
                pending+=piece
                while b'\n' in pending:
                    line,pending=pending.split(b'\n',1);need(bool(line),'blank-memory-row');sample(line)
                need(len(pending)<=1024**2,'memory-row-byte-bound')
                need(not decoder.unused_data,'gzip-trailing-member-or-bytes')
    need(decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail and not pending,'complete-single-gzip-member')
    need(count==g['memory_sample_count'] and count>=2 and positive>0,'complete-positive-native-memory-series')
    return dict(samples=count,raw_max_interval_seconds=maximum,positive_live_member_observations=positive,
                explicit_zombie_observations=zombies,single_gzip_member=True)




if __name__=='__main__':main()
