"""Enforce the final read-only census's outer600-second process-tree bound."""
import argparse,json,shutil,subprocess,sys,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent
end=json.loads((scratch/'final-proofs-v3/reserve-bound.json').read_bytes())['conservative_deadline_epoch']
assert time.time()+600<end,'Full cap must fit original reporting reserve'
log=scratch/'final-resource-census';log.mkdir(exist_ok=False)
began=time.time();censored=False
argv=[sys.executable,str(source/'final_resources.py'),'--scratch',str(scratch),'--output',str(source/'evidence/final-resources.json')]
with (log/'census.stdout').open('wb') as out,(log/'census.stderr').open('wb') as err:
 p=subprocess.Popen(argv,stdout=out,stderr=err)
 try:p.wait(timeout=600)
 except subprocess.TimeoutExpired:
  censored=True;subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30);p.wait(timeout=30)
row=dict(argv=argv,started_epoch=began,deadline_epoch=end,cap_seconds=600,exit=p.returncode,censored=censored,seconds=time.time()-began,
 qualification='Read-only census. A timeout is a censored resource observation, not proof of quiescence. Guest has its own40-second alarm. Snapshot precedes its own small receipts and final report/inventory/Git writes.')
(log/'completed.json').write_text(json.dumps(row,indent=2))
destination=source/'evidence/final-resource-controller';destination.mkdir(exist_ok=False)
for name in ['census.stdout','census.stderr','completed.json']:shutil.copyfile(log/name,destination/name)
print(json.dumps(row))
assert p.returncode==0 and not censored,row
