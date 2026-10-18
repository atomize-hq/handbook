"""Render reached observations, never estimated or censored samples as results."""
import gzip,json,pathlib,re,statistics
H=pathlib.Path(__file__).resolve().parent
def load(p):return json.loads(p.read_bytes())
def case(root):
    match=re.search(r'/growth/(\d+)-([AB])-(project|research)',root.replace('\\','/'))
    return (int(match[1]),match[2],match[3]) if match else None
def value(r,key):return f"{r[key]:,.2f}" if r else '—'
lines=['# Reached observations','','Times are individual engine observations in ms unless a three-process lookup median is explicitly stated. OS/device caches are uncontrolled. No p95 or product SLA.','','| Platform | Backend | Documents/store | Mode | Init | Commit 1, no packet | Commit 1 + packet | Packet from selected |','| --- | --- | ---: | --- | ---: | ---: | ---: | ---: |']
point_lines=['# Fresh-process lookup observations','','Three process observations per key/binding; medians below. Positive key d000001 is ordinary metadata. Negative key not-present lies outside the existing d* key range; this is not an in-range-miss distribution. Each operation verifies positive selected lineage. Point path verification is not a full audit. End-to-end engine time includes lineage; point-inclusive stage excludes it.','','| Platform | Backend | Documents | Mode | Binding | Positive median ms | Negative median ms | Positive app bytes read | Positive metadata bytes read |','| --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: |']
process_lines=['# Observed process latency','','Controller time includes process creation, request delivery, response serialization/collection and process exit. Engine clock ends before final stdout serialization. These are one-attempt serial observations, separate from retry-inclusive contention request latency.','','| Platform | Backend | Documents | Mode | Operation | Engine ms | Observed process ms |','| --- | --- | ---: | --- | --- | ---: | ---: |']
process_points=['# Observed fresh-process lookup latency','','Three independent process observations per binding/key. Caches uncontrolled; these include process startup/delivery and differ from the engine timer.','','| Platform | Backend | Documents | Mode | Binding | Positive median ms | Negative median ms |','| --- | --- | ---: | --- | --- | ---: | ---: |']
amplification=['# One-document update amplification','','Metadata counters cover state descriptors/tree pages and indirect document metadata, excluding publication/outcome records, readable navigation and packet manifests. Application totals include their instrumented I/O. Work remains fully validated.','','| Platform | Backend | Documents | Mode | Packet | Metadata read | Metadata written | App read | App written | Peak RSS bytes |','| --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: |']
batch_lines=['# Changed-document batches','','Individual reached observations in ms; no percentile or controlled-cache claim. IDs are contiguous from d000000, so every batch includes the overview and 10k+ batches update its large metadata indirection. These are not scattered-key distributions. Every row is a complete commit with its declared packet request. The 100k tier allocates combined 1/64-document commits.','','| Platform | Backend | Documents | Mode | Changed | Packet requested | Engine ms | Observed process ms | App bytes read | App bytes written |','| --- | --- | ---: | --- | ---: | --- | ---: | ---: | ---: | ---: |']
storage_lines=['# Retained storage by store','','Bytes describe actual retained files, with no deletion of growth histories. File allocation excludes directory/MFT metadata. Logical metadata combines candidate state/request records with actual COW objects; application publication/outcome control records are separate. Readable bytes combine current view, retained generation and packet files. Growth roots have no retired snapshots. Stale sealed candidates remain protected until explicit abandonment even when not included in the selected/eligible immutable-liveness buckets.','','| Platform | Backend | Documents | Mode | Consumer | Selected publications | Logical bytes | Allocated file bytes | Files | Directories | Logical state/COW metadata | Logical readable bytes |','| --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
status_lines=['# Growth completion and censoring','','A completed initialization includes its requested readable work. Selected history is independently validated during final accounting; it can exist after a censored operation. Censored counts retain the original observation even if later explicit work succeeds. A dash in a timing table is not a noncommit or capacity result.','','| Platform | Backend | Documents | Mode | Init observation | Complete init/publish observations | Censored init/publish observations | Selected publications / plan |','| --- | --- | ---: | --- | --- | ---: | ---: | --- |']
contention_lines=['# Contention cohorts','','128-document stores; one 64-document request and one-document peers. Each width is one controlled cohort, not independent latency samples or a fairness distribution. Request clocks include explicit pure retries and controller scheduling. Rejected application I/O can include reusable bytes and is not unique orphan storage.','','| Platform | Backend | Mode | Policy | Width | Request median ms | Request max ms | Attempts | Retries | Max attempt wait ms | Max attempt hold ms | Rejected app bytes read | Rejected app bytes written |','| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
amplification[2]+=' Peak native memory is Windows peak working set or Linux maximum RSS, reported by the engine before final stdout serialization; these platform measures are not identical.'
amplification[4]=amplification[4].replace('Peak RSS bytes','Peak native memory bytes')
for table in [point_lines,process_points,process_lines,batch_lines,amplification]:
    table[4]+=' Measurement phase |';table[5]+=' --- |'
lines[4]+=' Phase: commit+packet / selected packet |';lines[5]+=' --- |'
lines[2]+=' Original initialization censors remain dashes; separate post-cap work is labeled here and in the process/batch tables.'
point_lines[2]+=' Phase labels distinguish post-cap probes; original initialization censorship is retained in the growth status table.'
for platform in ['windows','wsl']:
    folder=H/'evidence'/platform
    if not (folder/'operations.json').exists():continue
    ops=load(folder/'operations.json');points=load(folder/'points.json')
    accounts=load(folder/'accounting.json') if (folder/'accounting.json').exists() else []
    groups=sorted({(r['backend'],case(r['root'])) for r in ops if case(r['root']) and case(r['root'])[2]=='project'})
    for backend,c in groups:
        n,mode,consumer=c;rs=[r for r in ops if r['backend']==backend and case(r['root'])==c and not r['censored'] and r['exit']==0 and r['status']=='ok' and (r['action'] not in ['init','publish'] or (r['delivery_status']=='published' and r['publication_outcome']=='accepted'))]
        def get(action,req=None):return next((r for r in rs if r['action']==action and (req is None or r.get('request')==req)),None)
        init=get('init');no=get('publish','commit-1-packet-False');yes=get('publish','commit-1-packet-True');packet=get('packet')
        attempted=[r for r in ops if r['backend']==backend and case(r['root'])==c and r['action'] in ['init','publish']]
        init_attempt=next((r for r in attempted if r['action']=='init'),None)
        init_status='complete' if init else 'censored' if init_attempt and init_attempt['censored'] else 'incomplete/error' if init_attempt else 'not observed'
        selected=next((r['account']['selected_publications'] for r in accounts if r['backend']==backend and case(r['root'])==c),0)
        status_lines.append('| '+' | '.join(map(str,[platform,backend,f'{n:,}',mode,init_status,sum(r['action'] in ['init','publish'] for r in rs),sum(r['censored'] for r in attempted),f'{selected} / '+str({1000:20,10000:10,100000:3}[n])]))+' |')
        for r in rs:
            batch=re.fullmatch(r'commit-(1|8|64)-packet-(True|False)',r.get('request') or '')
            if batch:batch_lines.append('| '+' | '.join(map(str,[platform,backend,f'{n:,}',mode,batch[1],r['packet_requested'],f"{r['engine_ms']:,.2f}",f"{r['external_ms']:,.2f}",r['bytes_read'],r['bytes_written'],r.get('measurement_phase','unclassified')]))+' |')
        no_cell='not allocated' if n==100000 else value(no,'engine_ms')
        phases=' / '.join(r.get('measurement_phase','unclassified') if r else 'not observed' for r in [yes,packet])
        lines.append(f"| {platform} | {backend} | {n:,} | {mode} | {value(init,'engine_ms')} | {no_cell} | {value(yes,'engine_ms')} | {value(packet,'engine_ms')} | {phases} |")
        for r in [no,yes]:
            if r:amplification.append('| '+' | '.join(map(str,[platform,backend,f'{n:,}',mode,r['packet_requested'],r['metadata_read'],r['metadata_written'],r['bytes_read'],r['bytes_written'],r['peak_memory_bytes'],r.get('measurement_phase','unclassified')]))+' |')
        for name,r in [('initialization',init),('commit without packet',no),('commit with packet',yes),('selected packet assembly',packet)]:
            if r:process_lines.append(f"| {platform} | {backend} | {n:,} | {mode} | {name} | {r['engine_ms']:,.2f} | {r['external_ms']:,.2f} | {r.get('measurement_phase','unclassified')} |")
        ps=[r for r in points if r['backend']==backend and case(r['root'])==c and not r['censored'] and r['exit']==0 and r['status']=='ok'];publications=list(dict.fromkeys(r['publication'] for r in ps))
        for pub in publications:
            pos=[r for r in ps if r['publication']==pub and r['key']=='d000001'];neg=[r for r in ps if r['publication']==pub and r['key']=='not-present']
            if len(pos)!=3 or len(neg)!=3:continue
            phase=' / '.join(sorted({r.get('measurement_phase','unclassified') for r in pos+neg}))
            roles={r['binding_role'] for r in pos+neg};assert len(roles)==1;role=next(iter(roles))
            point_lines.append(f"| {platform} | {backend} | {n:,} | {mode} | {role} | {statistics.median(r['engine_ms'] for r in pos):.3f} | {statistics.median(r['engine_ms'] for r in neg):.3f} | {statistics.median(r['bytes_read'] for r in pos):,.0f} | {statistics.median(r['metadata_read'] for r in pos):,.0f} | {phase} |")
            process_points.append(f"| {platform} | {backend} | {n:,} | {mode} | {role} | {statistics.median(r['external_ms'] for r in pos):.3f} | {statistics.median(r['external_ms'] for r in neg):.3f} | {phase} |")
    if (folder/'accounting.json').exists():
        for row in load(folder/'accounting.json'):
            c=case(row['root'])
            if not c:continue
            n,mode,consumer=c;acc=row['account'];total=acc['detailed']['total'];groups=acc['detailed']['groups']
            metadata=sum(v['logical_bytes'] for k,v in groups.items() if k in ['cow-metadata','candidate-state-and-request-metadata'])
            readable=sum(v['logical_bytes'] for k,v in groups.items() if k.startswith(('live-view-','readable-generation-','packet-')))
            storage_lines.append('| '+' | '.join(map(str,[platform,row['backend'],f'{n:,}',mode,consumer,acc['selected_publications'],total['logical_bytes'],total['allocated_file_bytes'],total['files'],acc['detailed']['directories'],metadata,readable]))+' |')
    for backend in ['manifest','cow']:
        observations=folder/f'{backend}--contention-observations.json';rawpath=folder/f'{backend}--contention-raw.jsonl.gz'
        if not observations.exists():continue
        raw=[json.loads(line) for line in gzip.decompress(rawpath.read_bytes()).splitlines() if line]
        cohorts=load(observations)
        for mode,policy,width in sorted({(r['mode'],r['policy'],r['width']) for r in cohorts if 'width' in r}):
            cohort=[r for r in cohorts if (r.get('mode'),r.get('policy'),r.get('width'))==(mode,policy,width)]
            rejected=[r['output']['metrics'] for r in raw if r.get('output') and r['output']['status']=='error' and r['command']['action']=='publish' and r['command']['root'].replace('\\','/').endswith(f'/contention/{mode}-{policy}') and (r['command'].get('request',{}).get('request') or '').startswith(f'wave-{width}-')]
            waits=[v or 0 for r in cohort for v in r['waits']];holds=[v or 0 for r in cohort for v in r['holds']]
            contention_lines.append('| '+' | '.join(map(str,[platform,backend,mode,policy,width,f"{statistics.median(r['complete_controller_ns'] for r in cohort)/1e6:,.2f}",f"{max(r['complete_controller_ns'] for r in cohort)/1e6:,.2f}",sum(r['attempts'] for r in cohort),sum(r['retries'] for r in cohort),f'{max(waits)/1e6:,.2f}',f'{max(holds)/1e6:,.2f}',sum(r['bytes_read'] for r in rejected),sum(r['bytes_written'] for r in rejected)]))+' |')
for file,content in [('operation-table.md',lines),('lookup-table.md',point_lines),('amplification-table.md',amplification),('process-latency-table.md',process_lines),('process-lookup-table.md',process_points),('batch-table.md',batch_lines),('storage-table.md',storage_lines),('growth-status-table.md',status_lines),('contention-table.md',contention_lines)]: (H/'evidence'/file).write_text('\n'.join(content)+'\n',encoding='utf8')
print('Tables rendered from reached archived observations')
