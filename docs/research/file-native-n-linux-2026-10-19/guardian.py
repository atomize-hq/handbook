"""Linux descendant RSS and termination guard across nested process groups."""
import os,sys,time,json,signal,resource
from pathlib import Path
BASE=Path(__file__).resolve().parent
root=int(sys.argv[1]);deadline=float(sys.argv[2]);known={};peak=None;peak_process=None;samples=0;reason=None;killed=[];start=time.time()
while True:
    table={}
    for p in Path('/proc').iterdir():
        if not p.name.isdigit():continue
        try:
            fields=(p/'stat').read_text().rsplit(')',1)[1].split();status=(p/'status').read_text().splitlines()
            rss=next((int(x.split()[1])*1024 for x in status if x.startswith('VmRSS:')),None)
            table[int(p.name)]=dict(ppid=int(fields[1]),pgrp=int(fields[2]),birth=fields[19],rss=rss)
        except (OSError,ValueError,IndexError):pass
    owned={root} if root in table else set()
    owned.update(pid for pid,birth in known.items() if pid in table and table[pid]['birth']==birth)
    while True:
        children={pid for pid,t in table.items() if t['ppid'] in owned};new=children-owned
        if not new:break
        owned.update(new)
    for pid in owned:known[pid]=table[pid]['birth']
    measured=[table[pid]['rss'] for pid in owned if table[pid]['rss'] is not None]
    guardian_rss=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024
    # Include the independently running census watcher with a conservative 64MiB
    # allowance. Its actual high-water sample is retained in coexistence.json.
    total=sum(measured)+guardian_rss+64*1024**2
    if measured:samples+=1;peak=max(peak or 0,total);peak_process=max(peak_process or 0,max(measured))
    if total>3*1024**3:reason='aggregate-RSS-censor'
    if any(x>2*1024**3 for x in measured):reason='checker-process-RSS-censor'
    if time.time()>=deadline:reason='original-matrix-outer-time-censor'
    if root not in table and owned:reason='orphaned-descendants-after-controller-exit'
    if reason:
        groups={table[pid]['pgrp'] for pid in owned}
        for group in groups:
            try:os.killpg(group,signal.SIGSTOP)
            except ProcessLookupError:pass
        for group in groups:
            try:os.killpg(group,signal.SIGKILL);killed.append(group)
            except ProcessLookupError:pass
        break
    if root not in table:break
    time.sleep(.05)
(BASE/'descendant-guardian.json').write_text(json.dumps(dict(root_pid=root,start_epoch=start,end_epoch=time.time(),hard_deadline_epoch=deadline,samples=samples,peak_descendant_plus_guardian_and_watcher_allowance_rss=peak,peak_single_descendant_rss=peak_process,watcher_allowance_bytes=64*1024**2,missing_measurement=peak is None,censor=reason,killed_process_groups=killed,tracked_processes=len(known),earlier_unobserved_interval='missing; original per-stage samplers retained'),indent=2))
