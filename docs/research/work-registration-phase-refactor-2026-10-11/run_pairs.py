"""Untimed independent-root population and serial paired native measurements."""
import concurrent.futures, ctypes, hashlib, json, pathlib, subprocess, sys, tempfile, time
HERE=pathlib.Path(__file__).resolve().parent
SCRATCH=pathlib.Path('C:/Users/spmcc/AppData/Local/Temp/hcm-wr-phase-roots-20261011')
class Memory(ctypes.Structure):
    _fields_=[('cb',ctypes.c_ulong),('faults',ctypes.c_ulong),('peak_working',ctypes.c_size_t),('working',ctypes.c_size_t),('peak_paged',ctypes.c_size_t),('paged',ctypes.c_size_t),('peak_nonpaged',ctypes.c_size_t),('nonpaged',ctypes.c_size_t),('pagefile',ctypes.c_size_t),('peak_pagefile',ctypes.c_size_t),('private',ctypes.c_size_t)]
PSAPI=ctypes.WinDLL('psapi');PSAPI.GetProcessMemoryInfo.argtypes=[ctypes.c_void_p,ctypes.POINTER(Memory),ctypes.c_ulong]
KERNEL=ctypes.WinDLL('kernel32');KERNEL.GetProcessHandleCount.argtypes=[ctypes.c_void_p,ctypes.POINTER(ctypes.c_ulong)]
def run(command):
    start=time.perf_counter();peak_working=peak_private=peak_handles=handle_samples=0
    # Files keep the child from blocking on a finite Windows pipe buffer while
    # the parent polls resources. Nothing is emitted inside operation timers.
    with tempfile.TemporaryFile(mode='w+t',encoding='utf-8') as stdout_file,tempfile.TemporaryFile(mode='w+t',encoding='utf-8') as stderr_file:
        p=subprocess.Popen([str(x) for x in command],stdout=stdout_file,stderr=stderr_file)
        while p.poll() is None:
            m=Memory();m.cb=ctypes.sizeof(m)
            if PSAPI.GetProcessMemoryInfo(int(p._handle),ctypes.byref(m),m.cb):
                peak_working=max(peak_working,m.peak_working);peak_private=max(peak_private,m.private)
            handles=ctypes.c_ulong()
            if KERNEL.GetProcessHandleCount(int(p._handle),ctypes.byref(handles)):
                peak_handles=max(peak_handles,handles.value);handle_samples+=1
            time.sleep(.1)
        process_wall=time.perf_counter()-start
        stdout_file.seek(0);stderr_file.seek(0);stdout=stdout_file.read();stderr=stderr_file.read()
    raw=HERE/'raw';raw.mkdir(exist_ok=True);label=command[1]+'-'+pathlib.Path(command[2]).name+'-'+str(command[-1])
    (raw/(label+'.stdout.jsonl')).write_text(stdout,encoding='utf-8')
    (raw/(label+'.stderr.txt')).write_text(stderr,encoding='utf-8')
    if p.returncode:raise RuntimeError(json.dumps({'command':[str(x) for x in command],'exit':p.returncode,'stdout':stdout,'stderr':stderr}))
    rows=[json.loads(x) for x in stdout.splitlines() if x.strip()]
    if rows and command[1] in ['measure','resources'] and not handle_samples:
        raise RuntimeError('No successful process handle samples for '+str(command[2]))
    exe_sha256=hashlib.sha256(pathlib.Path(command[0]).read_bytes()).hexdigest()
    for row in rows:row.update(process_wall_s=process_wall,sampled_peak_private_bytes=peak_private,os_peak_working_set_bytes=peak_working,sampled_peak_process_handles=peak_handles,successful_handle_samples=handle_samples,resource_sample_interval_ms=100,exe_sha256=exe_sha256,raw_stdout=(raw/(label+'.stdout.jsonl')).relative_to(HERE).as_posix(),raw_stderr=(raw/(label+'.stderr.txt')).relative_to(HERE).as_posix())
    return rows
def root(group,cohort,history,sample,variant):return SCRATCH/f'{group}-{cohort}-{history:02}-{sample:02}-{variant}'
def cells(group='pristine'):
    if group in ['dimensions','controls']:
        for sample in [6,7,8,9]:
            for cohort in (['explicit','kind'] if sample in [6,7] else ['kind']):yield cohort,16,sample
        return
    # Alternate cells as well as within-pair order; fixtures remain independently built.
    for sample in (range(1) if group.startswith('attribution') else range(6)):
        for history in ([8,16,31] if sample%2==0 else [31,16,8]):
            for cohort in (['explicit','kind'] if sample%2==0 else ['kind','explicit']):yield cohort,history,sample
def populate(exe,group):
    jobs=[(c,h,s,v) for c,h,s in cells(group) for v in ['baseline','refactor']]
    target=HERE/f'{group}-population.jsonl';target.parent.mkdir(parents=True,exist_ok=True)
    existing=set()
    if target.exists():
        existing={(x['cohort'],x['history'],x['sample'],x['root_variant']) for x in map(json.loads,target.read_text().splitlines())}
    def job(args):
        c,h,s,v=args;p=root(group,c,h,s,v)
        if p.exists():
            if s!=0 or not (p/'fixture.first.json').exists():raise RuntimeError(f'Incomplete or unrecognized preexisting fixture {p}; preserve for inspection')
            rows=run([exe,'inspect',p,c,h,s])
            for row in rows:row['original_population_exe_sha256']='b2ffbabbbe38a4cafb0c999adecc3a3f33b736b1489b396faf8486a8a32c6b6e'
        else:rows=run([exe,'populate',p,c,h,s])
        for row in rows:row['root_variant']=v
        return rows
    with target.open('a',encoding='utf-8') as out,concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        futures={pool.submit(job,j):j for j in jobs if j not in existing}
        for f in concurrent.futures.as_completed(futures):
            for row in f.result():out.write(json.dumps(row)+'\n');out.flush();print(json.dumps({'populated':futures[f],'seconds':round(row['process_wall_s'],1)}),flush=True)
def measure(base,ref,group,on):
    output=HERE/f'{group}-measurements.jsonl'
    with output.open('w',encoding='utf-8') as out:
        for c,h,s in cells(group):
            for order,v in enumerate(['baseline','refactor'] if s%2==0 else ['refactor','baseline']):
                exe=base if v=='baseline' else ref
                for row in run([exe,'measure',root(group,c,h,s,v),c,h,s,v,'on' if on else 'off']):
                    row['within_pair_order']=order;out.write(json.dumps(row)+'\n');out.flush()
                    print(json.dumps({'measured':[c,h,s,v],'pair_ms':round(row['pair_wall_ms'],2)}),flush=True)
def attribution(base,ref):
    # onx/off have identical root-name byte lengths. Alternate enabled/disabled
    # pair order and reverse baseline/refactor order in the second pair.
    with (HERE/'attribution-measurements.jsonl').open('w',encoding='utf-8') as out:
        for cell_order,(c,h,s) in enumerate(cells('attribution-onx')):
            modes=['onx','off'] if cell_order%2==0 else ['off','onx']
            for mode_order,mode in enumerate(modes):
                group='attribution-'+mode
                variants=['baseline','refactor'] if (cell_order+mode_order)%2==0 else ['refactor','baseline']
                for variant_order,v in enumerate(variants):
                    exe=base if v=='baseline' else ref
                    for row in run([exe,'measure',root(group,c,h,s,v),c,h,s,v,'on' if mode=='onx' else 'off']):
                        row.update(mode_order=mode_order,within_pair_order=variant_order);out.write(json.dumps(row)+'\n');out.flush()
                        print(json.dumps({'attributed':[c,h,s,v,mode],'pair_ms':round(row['pair_wall_ms'],2)}),flush=True)
if __name__=='__main__':
    if sys.argv[1]=='populate':populate(pathlib.Path(sys.argv[2]),sys.argv[3])
    elif sys.argv[1]=='measure':measure(pathlib.Path(sys.argv[2]),pathlib.Path(sys.argv[3]),sys.argv[4],len(sys.argv)>5 and sys.argv[5]=='on')
    elif sys.argv[1]=='attribution':attribution(pathlib.Path(sys.argv[2]),pathlib.Path(sys.argv[3]))
    else:raise ValueError('populate|measure')
