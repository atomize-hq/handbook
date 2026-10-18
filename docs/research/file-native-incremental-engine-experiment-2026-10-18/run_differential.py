import argparse,json
from controller import Driver
a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args()
d=Driver(ns.exe,ns.base,7200);rows=[]
for n in [128,1024]:
    for seed in [11,29,71,113]:
        rows.append(d.call(dict(action='differential',root=str(d.base/f'n{n}-s{seed}'),count=n,seed=seed),cap=900))
(d.base/'differential.json').write_text(json.dumps(rows,indent=2))
