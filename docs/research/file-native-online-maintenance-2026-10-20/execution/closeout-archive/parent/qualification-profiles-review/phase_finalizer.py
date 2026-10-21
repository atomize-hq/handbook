"""Bounded post-exit join for exactly sixteen FINAL provisional history bodies.

No native/process/engine work on import. This program never spawns a child,
decodes semantic storage, or accepts a summary PASS in place of closed raw data.
An admitted finalizer process must be outside the ORIGINAL owned Job. Its own
outer closure remains the parent's separate responsibility.
"""
import base64
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import zlib
from long_source_profiles import (canonical, closed, encode_private_record, hex64,
                                  integer, need, no_links, read, sha, strict)

API='history-postexit-finalizer-v1'
QPC_V2='qpc-ns-v2; whole-Job tail coverage; strict >40ms; coarse monotonic retained; no retrospective requalification'
CALL_FIELDS={
 'stream':{'v','kind','run','source_pin','binary_sha','guard_sha','driver_sha','profile_sha'},
 'begin':{'v','kind','call','seq','root','action','role','env','input_n','input_sha','input_scope','pid','creation','qpc_start'},
 'start':{'v','kind','call','seq','pid','creation','qpc','rss'},
 'data':{'v','kind','call','seq','stream','offset','n','sha','b64'},
 'rss':{'v','kind','call','seq','pid','creation','qpc','rss','peak','role','phase'},
 'end':{'v','kind','call','seq','exit','in_n','out_n','err_n','in_sha','out_sha','err_sha',
        'input_complete','out_eof','err_eof','rss_n','peak','qpc_end','first_write_qpc','native_closed',
        'io_closed','callbacks_joined','intent','violations'}}
PROVISIONAL='PROVISIONAL_UNQUALIFIED'
PHASE_BODY_KEYS={'schema','status','phase_id','plan_sha256','source_pin','binary_sha256',
    'budget_sha256','outer_run_id','source_profile_sha256','evidence','summary',
    'owned_closure_ref','checkpoint_ref','case_closure_ref'}


def source(row, name):
    raw=read(row['path'],integer(row['bytes'],0,2*1024**2,'source-bytes'))
    need(len(raw)==row['bytes'] and sha(raw)==row['sha256'] and row.get('reviewed') is True,
         'source-bytes:'+name)
    spec=importlib.util.spec_from_file_location('finalizer_'+name,row['path'])
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def file_binding(row, maximum, label):
    need(type(row) is dict and {'path','bytes','sha256'}<=set(row) and hex64(row['sha256']),
         'file-ref:'+label)
    size=integer(row['bytes'],0,maximum,'file-size:'+label)
    path=no_links(row['path'])
    h=hashlib.sha256();total=0
    with path.open('rb') as f:
        a=os.fstat(f.fileno())
        need(a.st_size==size,'file-size-binding:'+label)
        for block in iter(lambda:f.read(65536),b''):
            total+=len(block);need(total<=size,'file-grew:'+label);h.update(block)
        b=os.fstat(f.fileno())
    need(total==size and h.hexdigest()==row['sha256'] and
         (a.st_dev,a.st_ino,a.st_size,a.st_mtime_ns)==(b.st_dev,b.st_ino,b.st_size,b.st_mtime_ns),
         'closed-file-content:'+label)
    return path


def body_reference(row, limit, label):
    need(type(row) is dict and {'path','bytes','sha256'}<=set(row) and
         set(row)<= {'path','offset','bytes','sha256','source_pin','run','schema'},
         'closed-body-reference:'+label)
    path=no_links(row['path']);offset=integer(row.get('offset',0),0,2**63-1,'body-offset')
    count=integer(row['bytes'],1,limit,'body-bytes')
    with path.open('rb') as f:
        before=os.fstat(f.fileno())
        need(offset+count<=before.st_size,'body-span')
        f.seek(offset);raw=f.read(count)
        after=os.fstat(f.fileno())
    need(len(raw)==count and sha(raw)==row['sha256'] and
         (before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns)==
         (after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns),'body-ref-content')
    value=strict(raw,limit)
    need(raw==encode_private_record(value),'canonical-private-record')
    return value


def _lines(path, maximum_bytes, maximum_line):
    with no_links(path).open('rb') as f:
        total=offset=0
        while True:
            line=f.readline(maximum_line+1)
            if not line:break
            need(line.endswith(b'\n') and len(line)<=maximum_line,'complete-bounded-line')
            total+=len(line);need(total<=maximum_bytes,'stream-byte-bound')
            yield offset,line
            offset=total


def gzip_lines(row, limits):
    """Single member, bounded expansion/rows/line, CRC and exact compressed EOF."""
    path=file_binding(row,limits['qpc_compressed_bytes'],'QPC-gzip')
    decoder=zlib.decompressobj(16+zlib.MAX_WBITS)
    pending=b'';expanded=rows=offset=0;compressed=hashlib.sha256()
    with path.open('rb') as f:
        while True:
            block=f.read(65536)
            if not block:break
            compressed.update(block)
            need(not decoder.eof,'gzip-concatenation-or-trailing')
            while block:
                out=decoder.decompress(block,65536)
                block=decoder.unconsumed_tail
                expanded+=len(out)
                need(expanded<=limits['qpc_expanded_bytes'],'QPC-expansion-bound')
                pending+=out
                while b'\n' in pending:
                    line,pending=pending.split(b'\n',1);line+=b'\n'
                    rows+=1
                    need(rows<=limits['qpc_rows'] and len(line)<=limits['qpc_line_bytes'],
                         'QPC-line-row-bound')
                    yield offset,line
                    offset+=len(line)
                need(len(pending)<=limits['qpc_line_bytes'],'QPC-pending-line-bound')
                need(not decoder.unused_data,'gzip-extra-member-or-padding')
                if decoder.eof:
                    need(not block and f.read(1)==b'','gzip-exact-EOF')
                    break
    need(decoder.eof and not pending and compressed.hexdigest()==row['sha256'],
         'gzip-truncated-or-incomplete-line')


def raw_capture(config):
    """Parse exactly ONE shared lossless v2 capture, including all interleaving."""
    limit=config['limits'];ref=config['native_capture']
    path=file_binding(ref,limit['native_bytes'],'native-capture')
    calls={};header=None;line_count=0;total_frames=0
    for offset,line in _lines(path,ref['bytes'],limit['native_line_bytes']):
        line_count+=1
        need(line_count<=limit['native_rows'],'native-row-bound')
        row=strict(line,limit['native_line_bytes'])
        kind=row.get('kind')
        need(kind in CALL_FIELDS and set(row)==CALL_FIELDS[kind] and row['v']==2,
             'closed-v2-frame')
        need(line==encode_private_record(row),'native-canonical-frame')
        if kind=='stream':
            need(header is None and offset==0,'one-first-capture-header')
            header=row
            expected=config['identity']
            need(row['run']==expected['run'] and row['source_pin']==expected['source_pin'] and
                 row['binary_sha']==expected['binary_sha256'] and
                 row['guard_sha']==expected['guard_sha256'] and
                 row['driver_sha']==expected['driver_sha256'] and
                 row['profile_sha']==expected['profile_sha256'],'capture-header-bindings')
            continue
        need(header is not None and re.fullmatch('[0-9a-f]{32}',row['call']),
             'header-before-owned-call')
        token=row['call']
        if kind=='begin':
            need(token not in calls and len(calls)<limit['native_calls'],'unique-native-call')
            need(row['role'] in ('reader','checker','long-engine','publisher','maintenance'),
                 'known-native-role')
            need(row['input_scope']=='planned' and row['seq']==0,'planned-BEGIN-seq0')
            if row['pid'] is not None:integer(row['pid'],1,2**32-1,'call-pid')
            if row['creation'] is not None:integer(row['creation'],1,2**64-1,'call-creation')
            integer(row['qpc_start'],1,2**64-1,'call-start')
            need(type(row['env']) is dict and len(canonical(row['env']))<=limit['env_bytes'],
                 'bounded-exact-environment')
            calls[token]=dict(begin=row,start=None,seq_next=0,planned=bytearray(),
                begin_offset=offset,end_offset=None,frames=0,bytes=0,
                digest=hashlib.sha256(),streams={k:hashlib.sha256() for k in ('in','out','err')},out_buffer=bytearray(),out_frames=[],
                sizes={k:0 for k in ('in','out','err')},rss=[],end=None)
        need(token in calls and calls[token]['end'] is None,'open-owned-call')
        c=calls[token]
        need(type(row['seq']) is int and row['seq']==c['seq_next'],'call-sequence-no-gap-or-duplicate')
        c['seq_next']+=1;c['digest'].update(line);c['frames']+=1;c['bytes']+=len(line)
        total_frames+=1
        if kind=='start':
            need(c['start'] is None,'exactly-one-START')
            integer(row['pid'],1,2**32-1,'START-pid');integer(row['creation'],1,2**64-1,'START-creation')
            integer(row['qpc'],c['begin']['qpc_start'],2**64-1,'START-QPC')
            cap=config['caps']['reader_bytes'] if c['begin']['role']=='reader' else config['caps']['checker_bytes']
            integer(row['rss'],1,cap,'positive-before-input-RSS')
            need(c['begin']['pid'] in (None,row['pid']) and c['begin']['creation'] in (None,row['creation']),
                 'START-matches-constructor-identity')
            c['start']=row
        elif kind=='data':
            need(row['stream'] in c['streams'] and hex64(row['sha']) and type(row['b64']) is str,
                 'typed-data-frame')
            need(row['stream']=='in' or c['start'] is not None,'START-before-OUT-ERR')
            raw=base64.b64decode(row['b64'],validate=True)
            need(base64.b64encode(raw).decode()==row['b64'] and
                 len(raw)==row['n'] and 0<len(raw)<=16384 and sha(raw)==row['sha'] and
                 row['offset']==c['sizes'][row['stream']],'exact-lossless-data-frame')
            c['streams'][row['stream']].update(raw);c['sizes'][row['stream']]+=len(raw)
            if row['stream']=='in':c['planned'].extend(raw)
            if row['stream']=='out':
                c['out_buffer'].extend(raw);c['out_frames'].append((offset,len(line),sha(line)))
            need(c['sizes'][row['stream']]<=limit['per_call_stream_bytes'],'native-stream-bound')
        elif kind=='rss':
            need(c['start'] is not None and row['pid']==c['start']['pid'] and row['creation']==c['start']['creation'] and
                 row['role']==c['begin']['role'] and row['phase'] in ('suspended','start','sample'),
                 'RSS-native-identity')
            for key in ('qpc','rss','peak'):integer(row[key],1,2**64-1,'RSS-'+key)
            role_limit=config['caps']['reader_bytes'] if row['role']=='reader' else config['caps']['checker_bytes']
            need(row['rss']<=role_limit and row['peak']<=role_limit,'role-RSS-cap')
            need(not c['rss'] or row['qpc']>=c['rss'][-1]['qpc'],'ordered-RSS')
            c['rss'].append(dict(qpc=row['qpc'],rss=row['rss'],peak=row['peak']))
            need(len(c['rss'])<=limit['rss_per_call'],'finite-native-RSS')
        elif kind=='end':
            need(c['start'] is not None,'no-START-constructor-prefix-is-not-qualified')
            need(row['exit'] in (0,2) and row['violations']==[] and row['intent'] is None,
                 'no-hidden-native-censor')
            need(all(row[k] is True for k in ('input_complete','out_eof','err_eof',
                 'native_closed','io_closed','callbacks_joined')),'native-actual-closure')
            need(row['qpc_end']>=c['begin']['qpc_start'] and row['rss_n']==len(c['rss']) and
                 c['rss'],'native-complete-lifetime')
            for stream,field in (('in','in'),('out','out'),('err','err')):
                need(row[field+'_n']==c['sizes'][stream] and
                     row[field+'_sha']==c['streams'][stream].hexdigest(),'native-final-stream-digest')
            need(len(c['planned'])==c['begin']['input_n'] and
                 sha(bytes(c['planned']))==c['begin']['input_sha'],'planned-input-artifact')
            actual=integer(row['in_n'],0,len(c['planned']),'actual-completed-prefix')
            need(row['in_sha']==sha(bytes(c['planned'][:actual])) and
                 row['input_complete']==(actual==len(c['planned'])),'actual-completion-prefix')
            if actual:
                integer(row['first_write_qpc'],c['start']['qpc'],row['qpc_end'],
                        'first-positive-completion-operation-after-START')
            else:
                need(row['first_write_qpc'] is None,'no-write-counter-without-completed-byte')
            need(c['sizes']['in']==c['begin']['input_n'] and
                 c['streams']['in'].hexdigest()==c['begin']['input_sha'],'complete-input-source')
            response=strict(bytes(c['out_buffer']),limit['per_call_stream_bytes'])
            need(type(response) is dict,'decoded-native-response')
            if row['exit']==2:
                need(c['begin']['action']=='publish' and response.get('status')=='error' and
                     response.get('error') in ('conflict:expected-publication','conflict:captured-base'),
                     'authentic-declared-semantic-conflict-only')
            if response.get('peak_memory_bytes') is not None:
                cap=config['caps']['reader_bytes'] if c['begin']['role']=='reader' else config['caps']['checker_bytes']
                integer(response['peak_memory_bytes'],1,cap,'reported-native-peak')
            c['response_sha256']=sha(canonical(response))
            c['out_buffer'].clear();c['planned'].clear()
            c['end']=row;c['end_offset']=offset+len(line)
    need(header is not None and calls and all(c['end'] is not None for c in calls.values()),
         'complete-closed-native-file')
    result={}
    for token,c in calls.items():
        effective=dict(c['begin'],pid=c['start']['pid'],creation=c['start']['creation'])
        result[token]=dict(begin=effective,start=c['start'],end=c['end'],rss=c['rss'],
            out_frames=c['out_frames'],response_sha256=c['response_sha256'],
            ref=dict(schema='single-raw-ref-v1',path=str(path),run=header['run'],call=token,
                begin=c['begin_offset'],end=c['end_offset'],frames=c['frames'],
                bytes=c['bytes'],sha256=c['digest'].hexdigest()))
    return dict(header=header,calls=result,frames=total_frames,rows=line_count,
                file_sha256=ref['sha256'],file_bytes=ref['bytes'])


def rehydrate(value, native, config, *, depth=0, state=None):
    """Resolve a closed response reference, with bounded RAM and no duplicate sink."""
    if state is None:state={'nodes':0,'responses':{}}
    state['nodes']+=1
    need(depth<=64 and state['nodes']<=250000,'phase-rehydration-bound')
    if type(value) is dict and value.get('schema')=='native-response-ref-v1':
        closed(value,('schema','raw_ref'),'native-response-projection')
        ref=value['raw_ref'];call=native['calls'].get(ref['call'])
        need(call is not None and ref==call['ref'],'projection-authentic-call-span')
        token=ref['call']
        if token not in state['responses']:
            out=bytearray();path=no_links(ref['path'])
            with path.open('rb') as stream:
                for offset,size,want in call['out_frames']:
                    stream.seek(offset);line=stream.read(size)
                    need(len(line)==size and sha(line)==want,'reopened-authentic-output-frame')
                    frame=strict(line,config['limits']['native_line_bytes'])
                    raw=base64.b64decode(frame['b64'],validate=True)
                    need(frame['call']==token and frame['stream']=='out' and
                         len(out)==frame['offset'] and sha(raw)==frame['sha'],'exact-output-projection')
                    out.extend(raw)
                    need(len(out)<=config['limits']['per_call_stream_bytes'],'response-RAM-bound')
            response=strict(bytes(out),config['limits']['per_call_stream_bytes'])
            need(sha(canonical(response))==call['response_sha256'],'parsed-native-response-hash')
            state['responses'][token]=response
        return state['responses'][token]
    if type(value) is dict:
        return {k:rehydrate(v,native,config,depth=depth+1,state=state) for k,v in value.items()}
    if type(value) is list:
        return [rehydrate(v,native,config,depth=depth+1,state=state) for v in value]
    return value


def clock_interval(bridge, qpc):
    """Integer rational mapping; uncertainty is preserved as an interval."""
    closed(bridge,('clock_kind','frequency','start','end','max_bracket_ns','sha256'),'clock-bridge')
    need(bridge['clock_kind']=='windows-qpc' and hex64(bridge['sha256']),'native-clock-domain')
    copy=dict(bridge);want=copy.pop('sha256')
    need(sha(canonical(copy))==want,'actual-clock-receipt-bytes')
    f=integer(bridge['frequency'],1,2**63-1,'QPC-frequency')
    intervals=[]
    for point in (bridge['start'],bridge['end']):
        closed(point,('qpc_before','perf_ns','qpc_after'),'clock-bracket')
        before=integer(point['qpc_before'],0,2**64-1,'bracket-before')
        after=integer(point['qpc_after'],before,2**64-1,'bracket-after')
        perf=integer(point['perf_ns'],1,2**63-1,'bracket-perf')
        need((after-before)*10**9<=bridge['max_bracket_ns']*f,'clock-uncertainty-cap')
        lower=perf+((qpc-after)*10**9)//f
        upper=perf+((qpc-before)*10**9+f-1)//f
        intervals.append((lower,upper))
    lo=max(x[0] for x in intervals);hi=min(x[1] for x in intervals)
    need(lo<=hi,'inconsistent-clock-handshakes')
    return lo,hi


def _guard_file(config):
    row=config['guard_receipt']
    path=file_binding(row,config['limits']['guard_json_bytes'],'closed-QPC-receipt')
    value=strict(read(path,config['limits']['guard_json_bytes']),config['limits']['guard_json_bytes'])
    identity=config['identity'];owner=value['owner'];timing=value['memory_sample_timing']
    need(value['guard_sha256']==identity['guard_sha256'] and
         value['budget_sha256']==identity['budget_sha256'] and
         owner['nonce']==identity['run'] and owner['job_root_pid']==identity['host_pid'] and
         owner['job_root_creation_time']==identity['host_creation'] and
         owner['state']=='closed','actual-original-guard-owner')
    need(value['exit']==0 and value['censor'] is None and value['errors']==[] and
         value['terminations']==[] and value['remaining_job_pids']==[] and
         value['missing_memory_samples']==0 and value['process_closure_verified'] is True,
         'no-original-guard-censor')
    need(timing['clock']=='perf_counter_ns' and timing['target_interval_ns']==20000000 and
         timing['censor_interval_ns']==40000000,'exact-original-QPC-program-clock')
    for key in ('intervals_over_40ms','slow_samples'):
        need(timing[key]==0,'guard-'+key)
    for key in ('max_interval_seconds','max_completion_interval_seconds',
                'max_collection_duration_seconds','max_sample_duration_seconds',
                'max_recorded_interval_seconds','root_end_coverage_gap_seconds',
                'job_end_coverage_gap_seconds'):
        need(type(timing[key]) in (int,float) and 0<=timing[key]<=.04,'guard-window-'+key)
    need(value['sampling_revision']==QPC_V2 or value['sampling_revision'].startswith('qpc-ns-v3;'),
         'supported-guard-source-shape')
    need(type(value['closure_checks']) is list and len(value['closure_checks'])==2,
         'two-original-Job-observations')
    previous=None
    for check in value['closure_checks']:
        need(check['members']==[] and type(check['epoch']) in (int,float) and
             math.isfinite(check['epoch']) and
             owner['resumed_epoch']<=check['epoch']<=owner['closed_epoch'],
             'numeric-two-empty-observations')
        need(previous is None or check['epoch']>=previous,'ordered-empty-observations')
        previous=check['epoch']
    # The old exact guard source queries both empty observations only after
    # Popen.wait/root coverage and finally-drain. New shapes must give qpc_ns.
    if value['sampling_revision'].startswith('qpc-ns-v3;'):
        need(all(type(x.get('perf_counter_ns')) is int and
                 x['perf_counter_ns']>timing['root_wait_returned_ns']
                 for x in value['closure_checks']),'new-guard-numeric-postexit-empties')
    return value


def scan_guard(config, native):
    guard=_guard_file(config);timing=guard['memory_sample_timing'];identity=config['identity']
    need(Path(guard['raw_rss'])==Path(config['qpc_capture']['path']),'exact-QPC-stream-path')
    source_ref=config['bindings']['guard'];file_binding(source_ref,2*1024**2,'guard-program')
    old=guard['sampling_revision']==QPC_V2
    if old:
        # Source-level control-flow obligations are checked on the exact pinned
        # producer. No fabricated QPC fields are added to historical receipts.
        text=read(source_ref['path'],2*1024**2).decode()
        wait=text.index('process.wait(timeout=max(.001, absolute_deadline-time.time()) + 10)')
        root_end=text.index("event='root-end-sampling-coverage'",wait)
        finally_close=text.index('for _ in range(2):',root_end)
        empty=text.index("closure_checks.append(dict(epoch=time.time(), members=remaining))",finally_close)
        need(wait<root_end<finally_close<empty,'old-exact-closure-control-flow')
    last_tick=last_complete=None;count=0;first=None;last=None;root_end=None;job_end=None
    peaks={};observations=[];identity_rows={}
    for token,c in native['calls'].items():
        b,e=c['begin'],c['end']
        lo,hi=clock_interval(config['clock_bridge'],b['qpc_start'])
        endlo,endhi=clock_interval(config['clock_bridge'],e['qpc_end'])
        identity_rows.setdefault(b['pid'],[]).append(dict(creation=b['creation'],start=lo,
            end=endhi,role=b['role'],call=token,rss=c['rss']))
    for offset,line in gzip_lines(config['qpc_capture'],config['limits']):
        row=strict(line,config['limits']['qpc_line_bytes']);event=row.get('event')
        if event=='rss-sample':
            need(row.get('sample_complete') is True,'positive-full-QPC-sample')
            tick=integer(row['perf_counter_ns'],1,2**63-1,'QPC-tick')
            done=integer(row['completed_perf_counter_ns'],tick,2**63-1,'QPC-completion')
            need(done-tick<=40000000 and (last_tick is None or 0<tick-last_tick<=40000000) and
                 (last_complete is None or 0<done-last_complete<=40000000),'raw-QPC-cadence')
            need(row['collection_duration_seconds']>=0 and row['collection_duration_seconds']<=.04,
                 'raw-QPC-collection')
            need(type(row['processes']) is list and len(row['processes'])<=config['limits']['job_members'],
                 'finite-QPC-members')
            total=0;seen=set();host=False
            for process in row['processes']:
                closed(process,('pid','rss','peak'),'QPC-process')
                pid=integer(process['pid'],1,2**32-1,'sampled-pid')
                need(pid not in seen,'duplicate-QPC-PID');seen.add(pid)
                rss=integer(process['rss'],1,2**64-1,'sampled-RSS')
                peak=integer(process['peak'],rss,2**64-1,'sampled-peak');total+=rss
                if pid==identity['host_pid']:
                    host=True;limit=config['caps']['checker_bytes']
                else:
                    matches=[x for x in identity_rows.get(pid,[]) if x['start']<=done and x['end']>=tick]
                    need(len(matches)==1,'PID-creation-lifetime-unambiguous')
                    item=matches[0]
                    # The capture binds creation on begin and each native RSS
                    # row; old QPC rows can therefore be joined without a new ABI.
                    need(item['rss'] and all(x['qpc']>=native['calls'][item['call']]['begin']['qpc_start']
                                            for x in item['rss']),'native-identity-linked-RSS')
                    limit=config['caps']['reader_bytes'] if item['role']=='reader' else config['caps']['checker_bytes']
                need(rss<=limit and peak<=limit,'QPC-role-RSS-cap')
                peaks[pid]=max(peaks.get(pid,0),peak)
            need(total==row['job_rss'] and total<=config['caps']['aggregate_bytes'],
                 'QPC-aggregate-RSS')
            need(0<=row['supervisor_rss']<=config['caps']['supervisor_bytes'],
                 'QPC-supervisor-cap')
            if first is None:first=tick
            last=done;last_tick=tick;last_complete=done;count+=1
            observations.append((tick,done,host))
            need(len(observations)<=config['limits']['qpc_rows'],'bounded-QPC-index')
        elif event=='root-end-sampling-coverage':
            need(root_end is None and row['interval_seconds']<=.04,'root-exit-tail')
            root_end=integer(row['perf_counter_ns'],1,2**63-1,'root-exit-perf')
        elif event=='job-drain-sampling-coverage':
            need(job_end is None and row['empty_observed'] is True and row['members']==[] and
                 0<=row['interval_seconds']<=.04,'actual-Job-tail')
            job_end=integer(row['perf_counter_ns'],1,2**63-1,'Job-empty-perf')
        else:
            raise RuntimeError('late/missing/unsupported-QPC-event:'+str(event))
    need(first and last and root_end and job_end and first<=root_end<=job_end and
         count==timing['completed_samples']==timing['samples'] and
         first<=timing['first_positive_full_sample_ns']<=first+40000000 and
         root_end==timing['root_wait_returned_ns'] and job_end==timing['job_empty_observed_ns'] and
         0<=root_end-last<=40000000 and 0<=job_end-last<=40000000,
         'whole-original-run-QPC-coverage')
    # Old recorded_ns isn't in each frame: rely only on the EXACT producer's
    # after-write checks, zero late events and matching completed counts. New
    # guard revisions may expose every recorded counter for direct checking.
    return dict(raw_sha256=config['qpc_capture']['sha256'],samples=count,
        first_ns=first,last_ns=last,host_exit_ns=root_end,job_empty_ns=job_end,
        observations=observations,peaks=peaks,owner=guard['owner'],guard=guard)


def _phase_windows(plan, refs, config, native, guard, history_module):
    need(type(refs) is list and len(refs)==16,'exact-sixteen-final-phase-references')
    phases={p['id']:p for p in plan['phases']}
    need(len(phases)==16 and {r['id'] for r in refs}==set(phases),'exact16-phase-membership')
    results=[];seen_refs=set();previous_end=None;actual_counts={}
    for ref in refs:
        closed(ref,('id','body_ref'),'phase-index-entry')
        body=body_reference(ref['body_ref'],config['limits']['phase_bytes'],ref['id'])
        need(set(body)==PHASE_BODY_KEYS and body['schema']=='history-final-provisional-v1' and
             body['status']==PROVISIONAL and body['phase_id']==ref['id'] and
             body['plan_sha256']==plan['plan_sha256'],'FINAL-sealed-phase-body')
        key=(str(Path(ref['body_ref']['path'])),ref['body_ref'].get('offset',0),ref['body_ref']['sha256'])
        need(key not in seen_refs,'duplicate-phase-body-span');seen_refs.add(key)
        identity=config['identity']
        need(all(body[k]==identity[k] for k in ('source_pin','binary_sha256','budget_sha256')) and
             body['outer_run_id']==identity['run'] and
             body['source_profile_sha256']==identity['profile_sha256'],'phase-final-binding')
        evidence=rehydrate(body['evidence'],native,config);observation=evidence['guard_observation']
        need(observation['status']==PROVISIONAL and observation['final_receipt'] is None and
             observation['outer_run_id']==identity['run'] and
             observation['host_pid']==identity['host_pid'] and
             observation['host_creation_id']==str(identity['host_creation']),
             'provisional-original-host-identity')
        start=integer(observation['phase_start_ns'],guard['first_ns'],guard['host_exit_ns'],'phase-start')
        end=integer(observation['phase_end_ns'],start,guard['host_exit_ns'],'phase-end')
        need(previous_end is None or start>=previous_end,'ordered-nonoverlapping-source-phases')
        previous_end=end
        before=[x for x in guard['observations'] if x[0]<=start]
        within=[x for x in guard['observations'] if start<x[0]<=end]
        need(before,'sample-before-phase-start')
        covered=[before[-1]]+within
        need(0<=start-covered[0][0]<=40000000 and
             0<=end-covered[-1][0]<=40000000 and all(x[2] for x in covered),
             'host-present-sample-coverage-through-phase')
        summary=history_module.validate_phase_evidence(phases[ref['id']],evidence)
        need(summary==body['summary'],'phase-source-validator-result')
        for name in ('owned_closure_ref','checkpoint_ref','case_closure_ref'):
            proof=body_reference(body[name],config['limits']['metadata_bytes'],name)
            need(type(proof) is dict,'complete-attachment')
            if name=='owned_closure_ref':
                need(proof.get('children_empty') is True and proof.get('callbacks_joined') is True and
                     proof.get('scope')=='owned-children-IO-handles','actual-owned-phase-drain')
            elif name=='checkpoint_ref':
                need(proof.get('go') is True and proof.get('no_other_semantic_writer') is True,
                     'actual-closed-phase-census')
            else:
                need(proof.get('processes_closed') is True and proof.get('handles_closed') is True and
                     proof.get('python_callbacks_joined') is True,'case-closure-before-final-seal')
        # Every referenced invocation is resolved against actual canonical frames.
        tokens=evidence.get('native_call_refs')
        need(type(tokens) is list and tokens,'all-phase-native-refs')
        used=set()
        for r in tokens:
            token=r['call'];need(token in native['calls'] and token not in used,'phase-owned-native-call')
            need(r==native['calls'][token]['ref'],'actual-native-span-digest')
            used.add(token);call=native['calls'][token]
            lo,hi=clock_interval(config['clock_bridge'],call['begin']['qpc_start'])
            elo,ehi=clock_interval(config['clock_bridge'],call['end']['qpc_end'])
            need(lo>=start and ehi<=end,'native-call-bracketed-by-phase')
            action=call['begin']['action'];actual_counts[action]=actual_counts.get(action,0)+1
        results.append(dict(id=ref['id'],body_ref=ref['body_ref'],summary=summary,
                            start_ns=start,end_ns=end,native_calls=len(used)))
    return results,actual_counts


def census(config, prospective=True):
    """Fresh all named local roots; no planned deletion/reassignment credit."""
    rows=config['census']['roots']
    need(type(rows) is dict and rows,'explicit-whole-root-census')
    paths=[Path(r['root']) for r in rows.values()]
    need(all(p.is_absolute() for p in paths) and
         all(not a.is_relative_to(b) and not b.is_relative_to(a)
             for i,a in enumerate(paths) for b in paths[i+1:]),'disjoint-census-roots')
    result={};all_bytes=all_files=all_dirs=0
    for name,spec in rows.items():
        need({'root','bytes','files','directories','remaining_reservation'}==set(spec),'census-root-spec')
        root=no_links(spec['root']);used=count=dirs=0;h=hashlib.sha256()
        if root.exists():
            for folder,children,names in os.walk(root,followlinks=False):
                children.sort();names.sort()
                for n in children:
                    p=no_links(Path(folder)/n);need(p.is_dir(),'census-directory')
                    dirs+=1
                    h.update(encode_private_record(dict(path=p.relative_to(root).as_posix(),directory=True)))
                for n in names:
                    p=no_links(Path(folder)/n);need(p.is_file(),'census-file')
                    size=p.stat().st_size;used+=size;count+=1
                    h.update(encode_private_record(dict(path=p.relative_to(root).as_posix(),bytes=size)))
                    need(count<=spec['files'] and dirs<=spec['directories'] and used<=spec['bytes'],
                         'actual-census-cap:'+name)
        reserve=spec['remaining_reservation']
        closed(reserve,('bytes','files','directories'),'remaining-assigned-reservation')
        if prospective:
            need(used+reserve['bytes']<=spec['bytes'] and count+reserve['files']<=spec['files'] and
                 dirs+reserve['directories']<=spec['directories'],'actual-plus-valid-reservations:'+name)
        result[name]=dict(bytes=used,files=count,directories=dirs,
                         membership_sha256=h.hexdigest(),remaining_reservation=reserve)
        all_bytes+=used+reserve['bytes'];all_files+=count+reserve['files'];all_dirs+=dirs+reserve['directories']
    carry=config['census']['foreign_carried_reservations']
    closed(carry,('bytes','files','directories','receipt_ref'),'foreign-reserved-footprint')
    body_reference(carry['receipt_ref'],config['limits']['metadata_bytes'],'foreign-ledger')
    all_bytes+=carry['bytes'];all_files+=carry['files'];all_dirs+=carry['directories']
    caps=config['census']['combined_caps']
    need(all_bytes<=caps['bytes'] and all_files<=caps['files'] and all_dirs<=caps['directories'],
         'whole-combined-resource-cap')
    return dict(roots=result,combined=dict(bytes=all_bytes,files=all_files,directories=all_dirs),
                foreign_reservations=carry,planned_deletion_credit=0)


def validate_join_inputs(config):
    closed(config,('api','go','runtime_go','identity','bindings','limits','caps','plan',
        'phase_refs','native_capture','qpc_capture','guard_receipt','clock_bridge',
        'expanded_trace','census','output','current_finalizer_identity'),'finalizer-config')
    need(config['api']==API and config['go'] is True and config['runtime_go'] is True,
         'separate-postexit-finalizer-GO')
    i=closed(config['identity'],('run','source_pin','binary_sha256','budget_sha256','guard_sha256',
        'driver_sha256','profile_sha256','host_pid','host_creation'),'original-run-identity')
    need(re.fullmatch('[0-9a-f]{32}',i['run']) and re.fullmatch('[0-9a-f]{40}',i['source_pin']) and
         all(hex64(i[k]) for k in ('binary_sha256','budget_sha256','guard_sha256',
             'driver_sha256','profile_sha256')),'complete-final-source-binding')
    integer(i['host_pid'],1,2**32-1,'original-host-pid')
    integer(i['host_creation'],1,2**64-1,'original-host-creation')
    own=config['current_finalizer_identity']
    need(own['pid']==os.getpid() and hex64(own['parent_postexit_grant_sha256']),
         'separately-admitted-finalizer')
    owner_path=Path('C:/file-native-online-control-20261020/current-owner.json')
    current=strict(read(owner_path,8192),8192)
    need(current['state']=='running' and current['nonce']!=i['run'] and
         current['job_root_pid']==os.getpid(),'actual-new-guard-owner')
    module=source(config['bindings']['native'],'native_identity')
    held=module.Native(current)
    try:
        actual=held.creation(held.stage)
        need(held.stage_live() and held.members()=={os.getpid()} and
             actual==current['job_root_creation_time']==own['creation'] and
             not(os.getpid()==i['host_pid'] and actual==i['host_creation']),
             'live-current-Job-creation-distinct-from-closed-original')
    finally:
        held.close()
    limit_keys=('native_bytes','native_line_bytes','native_rows','native_calls','per_call_stream_bytes',
        'env_bytes','rss_per_call','qpc_compressed_bytes','qpc_expanded_bytes','qpc_line_bytes',
        'qpc_rows','job_members','guard_json_bytes','phase_bytes','metadata_bytes')
    closed(config['limits'],limit_keys,'all-bounded-read-limits')
    for k,v in config['limits'].items():integer(v,1,2**40,'limit:'+k)
    need(config['limits']['native_calls']<=20000 and config['limits']['native_rows']<=2000000 and
         config['limits']['qpc_rows']<=200000 and
         config['limits']['per_call_stream_bytes']<=16*1024**2 and
         config['limits']['native_line_bytes']<=65536 and
         config['limits']['qpc_line_bytes']<=65536 and config['limits']['phase_bytes']<=4*1024**2 and
         config['limits']['job_members']<=12,'bounded-parser-memory')
    closed(config['caps'],('reader_bytes','checker_bytes','aggregate_bytes','supervisor_bytes'),'role-caps')
    need(config['caps']['reader_bytes']==256*1024**2 and config['caps']['checker_bytes']==2*1024**3 and
         config['caps']['aggregate_bytes']<=4*1024**3 and
         config['caps']['supervisor_bytes']<=768*1024**2,'unchanged-role-resource-limits')
    for name,row in config['bindings'].items():
        file_binding(row,2*1024**2,'transitive-source:'+name)
    need(config['bindings']['guard']['sha256']==i['guard_sha256'],'actual-guard-source')
    need(no_links(__file__)==Path(config['bindings']['finalizer']['path']) and
         sha(read(__file__,65536))==config['bindings']['finalizer']['sha256'],'own-finalizer-source')
    return config


def join_history(config):
    validate_join_inputs(config)
    history=source(config['bindings']['history'],'history')
    plan=config['plan']
    need(plan['total_accepted']==1600 and plan['max_offers']==4000 and
         plan['max_sealed_losers']==2400 and len(plan['phases'])==16,'whole-history-plan')
    native=raw_capture(config)
    guard=scan_guard(config,native)
    phase_results,counts=_phase_windows(plan,config['phase_refs'],config,native,guard,history)
    trace=body_reference(config['expanded_trace'],config['limits']['metadata_bytes'],'source-expanded-trace')
    need(trace['profile_sha256']==config['identity']['profile_sha256'] and
         trace['source_pin']==config['identity']['source_pin'],'same-source-operation-schedule')
    # All initial lookup/bootstrap/checker/native calls must be declared, even
    # when outside the sixteen phase windows.
    expected=trace['native_invocations']
    need(type(expected) is list and len(expected)==len(native['calls']),'all-native-calls-source-counted')
    for item in expected:
        closed(item,('call','root','action','role','input_sha','source_operation'),'source-native-operation')
        need(item['call'] in native['calls'],'source-operation-call-present')
        begin=native['calls'][item['call']]['begin']
        need(all(item[k]==begin[k] for k in ('root','action','role','input_sha')),
             'actual-call-matches-expanded-source')
    need(len({x['call'] for x in expected})==len(expected),'no-reused-source-call-token')
    before=census(config)
    return dict(api=API,status='QUALIFIED_ORIGINAL_HISTORY_JOIN',source_pin=config['identity']['source_pin'],
        original_run=config['identity'],source_bindings=config['bindings'],
        original_guard_receipt=config['guard_receipt'],native_capture=config['native_capture'],
        qpc_capture=config['qpc_capture'],phase_refs=config['phase_refs'],phases=phase_results,
        actual_phase_action_counts=counts,source_schedule=config['expanded_trace'],
        QPC=dict(samples=guard['samples'],first_ns=guard['first_ns'],last_ns=guard['last_ns'],
                 host_exit_ns=guard['host_exit_ns'],job_empty_ns=guard['job_empty_ns']),
        two_empty_original_Job_observations=guard['guard']['closure_checks'],
        census_before_output=before,
        remaining_finalization_reservation=config['output'],
        qualification='original16-phasehistoryonly; parent must close this separately admitted finalizer run; other matrix cells separate')


def write_result(config):
    result=join_history(config)
    output=closed(config['output'],('path','category','max_bytes','reserved_files'),'bounded-finalizer-output')
    need(output['reserved_files']==1 and 0<output['max_bytes']<=1024*1024,'one-final-output-reservation')
    raw=encode_private_record(result)
    need(len(raw)<=output['max_bytes'],'result-prewrite-bound')
    root=Path(config['census']['roots'][output['category']]['root'])
    path=no_links(output['path'])
    need(path.is_relative_to(root) and path.parent.is_dir() and not path.exists(),
         'fresh-owned-finalizer-output')
    reserve=config['census']['roots'][output['category']]['remaining_reservation']
    need(reserve['bytes']>=output['max_bytes'] and reserve['files']>=1,'preheld-output-reserve')
    with path.open('xb') as f:
        need(f.write(raw)==len(raw),'finalizer-short-write');f.flush();os.fsync(f.fileno())
    after=census(config,prospective=False)
    return dict(output=dict(path=str(path),bytes=len(raw),sha256=sha(raw)),
                after_output_census=after,original_history_qualified=True,
                finalizer_outer_closure_pending=True)


if __name__=='__main__':
    import argparse
    parser=argparse.ArgumentParser()
    parser.add_argument('config')
    args=parser.parse_args()
    try:
        config=strict(read(args.config,1024*1024),1024*1024)
        print(json.dumps(write_result(config),sort_keys=True,separators=(',',':'),ensure_ascii=True))
    except BaseException as error:
        import sys
        sys.stderr.buffer.write(ascii(error).encode('ascii')[:2000]+b'\n')
        sys.stderr.buffer.flush()
        sys.exit(1)

