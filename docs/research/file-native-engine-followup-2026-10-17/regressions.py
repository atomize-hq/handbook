"""Final-byte focused regressions after the full development phase-oracle suite."""
import argparse,pathlib
from experiment import Driver,retention,damaged_view,duplicate_delete,extended_faults,io_checks,orphan_reads
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--exe',type=pathlib.Path,required=True);p.add_argument('--backend',default='manifest');a=p.parse_args()
d=Driver(a.base,a.exe,'regressions',a.backend)
io_checks(d);retention(d);damaged_view(d);duplicate_delete(d);extended_faults(d);orphan_reads(d)
b.write(d.base/'regressions-completed.json',dict(status='passed',backend=a.backend,binary_sha256=d.binary_sha256))
