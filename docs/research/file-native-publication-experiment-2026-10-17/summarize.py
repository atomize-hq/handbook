"""Preserve raw samples and derive literal tables; no benchmark fabrication."""
import collections, gzip, hashlib, json, math, pathlib, shutil, subprocess, time
from run import Driver, DEFAULT_BASE, DEFAULT_EXE, HERE, inventory, load, sha, write

def percentile(values, q):
    values=sorted(values)
    return values[max(0,math.ceil(len(values)*q)-1)] if values else None

def main():
    evidence=HERE/'evidence';evidence.mkdir(exist_ok=True)
    roots=[pathlib.Path('C:/file-native-experiment-20261017'),pathlib.Path('C:/file-native-experiment-20261017-v2'),DEFAULT_BASE]
    raw_index=[]
    for i,root in enumerate(roots,1):
        for source in root.glob('*.jsonl'):
            dest=evidence/f'run-{i}-{source.name}.gz'
            h=hashlib.sha256();size=0;lines=0
            with source.open('rb') as src,dest.open('wb') as output,gzip.GzipFile(fileobj=output,mode='wb',filename='',mtime=0) as gz:
                while block:=src.read(1024*1024):h.update(block);size+=len(block);lines+=block.count(b'\n');gz.write(block)
            raw_index.append({'source':str(source),'artifact':dest.name,'uncompressed_sha256':h.hexdigest(),'uncompressed_bytes':size,'lines':lines,'compressed_bytes':dest.stat().st_size,'qualification':'current corrected run' if i==3 else 'superseded / failed development run'})
        for source in root.glob('*.json'):
            shutil.copyfile(source,evidence/f'run-{i}-{source.name}')
    write(evidence/'raw-index.json',raw_index)
    pins={'source_base_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=HERE).decode().strip(),
          'rustc':subprocess.check_output(['rustc','-Vv']).decode(),
          'cargo':subprocess.check_output(['cargo','-V']).decode(),
          'binary':str(DEFAULT_EXE),'binary_sha256':sha(DEFAULT_EXE.read_bytes()),
          'source_sha256':{str(p.relative_to(HERE)):sha(p.read_bytes()) for p in sorted(HERE.rglob('*')) if p.is_file() and ('src' in p.relative_to(HERE).parts or p.name in ['Cargo.toml','Cargo.lock','run.py','supplement.py','summarize.py','additional.py','cap-guard.ps1'])},
          'controlling_proposal_sha256':sha((HERE.parent/'dolt-gitbutler-file-engine-2026-10-16/experiment.md').read_bytes()),
          'format_standard_sha256':sha((HERE.parents[1]/'specs/handbook-contract-membrane/structured-data-format-standard.md').read_bytes()),
          'recorded_epoch':time.time()}
    write(evidence/'pins.json',pins)
    shutil.copyfile(HERE.parent/'dolt-gitbutler-file-engine-2026-10-16/experiment.md',evidence/'execution-proposal-input.md')
    inv=[]
    for root in roots:
        data=inventory(root);inv.append({'root':str(root),**data})
    build=inventory(DEFAULT_EXE.parents[1]);total=sum(i['allocated_file_bytes'] for i in inv)+build['allocated_file_bytes']
    write(evidence/'inventory.json',{'runs':inv,'build':build,'total_allocated_file_bytes':total,'file_allocation_method':'Windows FileStandardInfo.AllocationSize; excludes MFT and directory index allocation','disk_free_at_inventory':shutil.disk_usage(DEFAULT_BASE).free,'cap_bytes':64*1024**3,'reserve_bytes':160*1024**3})
    assert total<64*1024**3
    assert shutil.disk_usage(DEFAULT_BASE).free>160*1024**3
    rows=[];growth_file=DEFAULT_BASE/'growth-raw.jsonl'
    if growth_file.exists():
        events=[json.loads(line) for line in growth_file.read_text(encoding='utf-8').splitlines()]
        groups=collections.defaultdict(list)
        for event in events:
            command=event['command'];key=pathlib.Path(command['root']).name
            groups[key].append(event)
        for path in (DEFAULT_BASE/'growth').iterdir():
            if path.is_dir():groups.setdefault(path.name,[])
        for key,events in groups.items():
            tier,mode,consumer,policy=key.split('-');success=[];refusals=[]
            for e in events:
                if e['command']['action']!='publish':continue
                if e.get('output',{}).get('status')=='ok' and e['output']['result'].get('status') in ['published','published/view-pending']:success.append(e)
                else:refusals.append(e)
            ns=[e['output']['wall_ns']/1e6 for e in success]
            r={'tier':int(tier),'mode':mode,'consumer':consumer,'policy':policy,'accepted_publications':len(success),'refused_attempts':len(refusals),
               'accepted_request_wall_ms':ns,'median_ms':percentile(ns,.5),'p95_ms':percentile(ns,.95) if len(ns)>=20 else None,
               'estimator':'nearest-rank; p95 suppressed below 20 samples; heterogeneous batches are not one latency distribution',
               'max_lock_wait_ms':max((e.get('output',{}).get('metrics',{}).get('ns',{}).get('lock-wait',0)/1e6 for e in events),default=0),
               'max_lock_hold_ms':max((e.get('output',{}).get('metrics',{}).get('ns',{}).get('lock-hold',0)/1e6 for e in events),default=0),
               'attempt_bytes_read':sum(e.get('output',{}).get('metrics',{}).get('bytes_read',0) for e in events),
               'attempt_bytes_written':sum(e.get('output',{}).get('metrics',{}).get('bytes_written',0) for e in events),
               'peak_process_memory_bytes':max([e['output']['peak_memory_bytes'] for e in events if e.get('output') and e['output'].get('peak_memory_bytes') is not None],default=None),
               'busy_attempt_wall_ms':[e['output']['wall_ns']/1e6 for e in refusals if str((e.get('output') or {}).get('error','')).startswith('busy:')],
               'initialization_ms':[e['output']['wall_ns']/1e6 for e in events if e['command']['action']=='init' and e.get('output')],
               'initialization_timeout':any(e.get('timeout') and e['command']['action']=='init' for e in events),
               'attempt_log_incomplete':not events,
               'serial':[{'request':e['command']['request']['request'],'wall_ms':e['output']['wall_ns']/1e6,'metrics':e['output']['metrics']} for e in success if e['command']['request']['request'].startswith('serial-')],
               'refusals':[{'request':e['command']['request']['request'],'error':e.get('output',{}).get('error'),'metrics':e.get('output',{}).get('metrics')} for e in refusals]}
            path=DEFAULT_BASE/'growth'/key
            r['inventory']=inventory(path)
            r['installed_generations']=len(list((path/'generations').iterdir())) if (path/'generations').exists() else 0
            try:
                sel=load(path/'SELECTED.json');pub_bytes=(path/'publications'/(sel['publication']+'.json')).read_bytes();assert sha(pub_bytes)==sel['digest'];pub=json.loads(pub_bytes)
                state_bytes=(path/pub['candidate']['state']).read_bytes();assert sha(state_bytes)==pub['candidate']['state_digest']
                selected=set();gen=set();candidate=pub['candidate'];head=sel
                while head:
                    selected.add(head['publication']);p_bytes=(path/'publications'/(head['publication']+'.json')).read_bytes();assert sha(p_bytes)==head['digest'];p=json.loads(p_bytes);gen.add(p['candidate']['generation']);head=p['predecessor']
                r['retained_selected_publications']=len(selected)
                r['unselected_generation_count']=r['installed_generations']-len(gen) if mode=='B' else 0
                r['current_readable']=candidate['closure'] if mode=='B' else 'view'
                if mode=='B':
                    unselected=[p for p in (path/'generations').iterdir() if p.name not in gen]
                    r['unselected_generation_logical_bytes']=sum(p.stat().st_size for directory in unselected for p in directory.rglob('*') if p.is_file())
            except (FileNotFoundError,KeyError) as error:
                r['incomplete']=str(error);r['pending_installed_generations']=r['installed_generations']
            rows.append(r)
        write(evidence/'growth-summary.json',rows)
        table=['| Docs | Option | Store | Policy | Selected | Refused attempts | Init ms | Serial 1 / 8 / 64 ms | Logical MiB | Allocated files MiB | Orphan generations |','|---:|:---:|---|---|---:|---:|---:|---|---:|---:|---:|']
        for r in rows:
            timings=' / '.join(f"{e['wall_ms']:.2f}" for e in r['serial'])
            initial=f"{r['initialization_ms'][0]:.1f}" if r['initialization_ms'] else 'timeout 600s' if r['initialization_timeout'] else 'interrupted / unlogged'
            table.append(f"| {r['tier']} | {r['mode']} | {r['consumer']} | {r['policy']} | {r.get('retained_selected_publications','incomplete')} | {r['refused_attempts']} | {initial} | {timings} | {r['inventory']['logical_bytes']/1024**2:.1f} | {r['inventory']['allocated_file_bytes']/1024**2:.1f} | {r.get('unselected_generation_count',0)} |")
        (evidence/'growth-table.md').write_text('\n'.join(table)+'\n',encoding='utf-8')
    write(evidence/'summary.json',{'current_race_count':len(load(DEFAULT_BASE/'race-observations.json')),'current_fault_count':len(load(DEFAULT_BASE/'fault-observations.json')),
                                  'supplement':load(DEFAULT_BASE/'supplement-completed.json') if (DEFAULT_BASE/'supplement-completed.json').exists() else None,
                                  'additional':load(DEFAULT_BASE/'additional-completed.json') if (DEFAULT_BASE/'additional-completed.json').exists() else None,
                                  'growth_configurations_observed':len(rows),'growth_configurations_complete':sum(r.get('retained_selected_publications')=={1000:20,10000:10,100000:3}[r['tier']] for r in rows),'native_linux':'unmet: configured host refused authentication; no WSL substitution',
                                  'explorer':'unmet: UI helper observed Explorer but input failed to navigate to fixture; not counted as successful exercise',
                                  'gate3':'not entered: required native/platform evidence incomplete; no integrated external backend, no COW/DB comparison claimed'})
    print(json.dumps({'configurations':len(rows),'allocated_files_GiB':total/1024**3,'raw_artifacts':len(raw_index)}))

if __name__=='__main__':main()
