"""Finite Windows correction checks; Driver retains normal resource watchdogs."""
import argparse,json,subprocess,sys,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent;root=scratch/'accounting-after-v2';root.mkdir(exist_ok=False);end=time.monotonic()+1800;rows=[]

def command(name,args,cap,phase_end=None):
 start=time.monotonic();stop=min(end,start+cap,phase_end if phase_end else end);censored=False
 with (root/(name+'.stdout')).open('wb') as out,(root/(name+'.stderr')).open('wb') as err:
  p=subprocess.Popen([str(x) for x in args],cwd=source,stdout=out,stderr=err)
  try:p.wait(timeout=max(0,stop-time.monotonic()))
  except subprocess.TimeoutExpired:
   censored=True;subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30);p.wait(timeout=30)
 row=dict(name=name,argv=[str(x) for x in args],exit=p.returncode,censored=censored,seconds=time.monotonic()-start);rows.append(row)
 (root/'steps.json').write_text(json.dumps(rows,indent=2));return not censored and p.returncode==0

target=scratch/'build-accounting';exe=target/'release/file-native-incremental.exe'
if command('tests',['cargo','test','--locked','--workspace','--target-dir',target],600):
 phase_end=min(end,time.monotonic()+900)
 for script,name in [('paired_intents.py','paired-intents'),('supplement.py','supplement'),('replay_lifetime.py','replay-lifetime')]:
  if not command(name,[sys.executable,source/script,'--exe',exe,'--base',root/name],900,phase_end):break
(root/'completed.json').write_text(json.dumps(dict(rows=rows,compile_source_binding='separate post-build receipt; initial build and stopped warm-build observation retained',case_phase_seconds=900,cap_seconds=1800),indent=2))
