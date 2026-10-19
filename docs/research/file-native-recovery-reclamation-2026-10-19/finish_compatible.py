"""Finish only focused cases whose full caps fit the original native deadline."""
import hashlib,json,sys,time
from pathlib import Path
from bounded import run
s=Path(__file__).resolve().parent;r=Path(sys.argv[1]);old=json.loads((r/'finished.json').read_bytes());end=old['rows'][0]['start_epoch']+1800;exe=r/'build/release/file-native-incremental';rows=[]
for name,script,arguments,cap in [('legacy','legacy_compatibility.py',['--fixtures',str(r.parent/'legacy-fixtures'),'--base',str(r/'compatible-legacy')],10),('extra','focused_extra.py',['--base',str(r/'compatible-extra')],20),('capsule','capsule.py',['--scratch',str(r/'compatible-small')],45 if 'WSL' in __import__('platform').release() or 'microsoft' in __import__('platform').release().lower() else 75)]:
    if time.time()+cap>end:rows.append(dict(name=name,status='unperformed',reason='full cap does not fit original native deadline'));continue
    row=run([sys.executable,s/script,'--exe',exe,*arguments],r/('compatible-focused-'+name),cap,end);rows.append(dict(name=name,**row))
result=dict(source_pin='fe1ee22a82d65dc88a14e38b7e77c921538c4bc4',rows=rows,binary_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),source_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (s.parent/'file-native-incremental-engine-experiment-2026-10-18/src').glob('*.rs')})
(r/'compatible-core-binding.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
