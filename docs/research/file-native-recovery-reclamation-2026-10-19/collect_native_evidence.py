"""Bundle only bounded native receipts/logs, never runtime stores."""
import argparse,tarfile
from pathlib import Path
a=argparse.ArgumentParser();a.add_argument('root');a.add_argument('output');ns=a.parse_args();root=Path(ns.root);files=list(root.glob('*.json'))+list(root.glob('*.stdout'))+list(root.glob('*.stderr'))
for directory,names in [('small',['summary.json','events.json']),('oracle-small-v2',['summary.json','events.json']),('extra-data',['extra.json','events.jsonl','session.json','session-source.zip']),('paired-data',['paired-intents.json','events.jsonl','session.json','session-source.zip']),('replay-data',['replay-lifetime.json','events.jsonl','session.json','session-source.zip']),('checks-data',['checks.json','events.jsonl','session.json','session-source.zip'])]:
    files.extend(root/directory/name for name in names)
files.extend(root/'backup-pipeline'/name for name in ['summary.json','events.json'])
files.extend(root/'final-core-small'/name for name in ['summary.json','events.json'])
files.extend(root/'final-core-extra'/name for name in ['extra.json','events.jsonl','session.json','session-source.zip'])
files.extend(root/'compatible-legacy'/name for name in ['legacy-compatibility.json'])
files.extend(root/'compatible-extra'/name for name in ['extra.json','events.jsonl','session.json','session-source.zip'])
files.extend(root/'compatible-small'/name for name in ['summary.json','events.json'])
with tarfile.open(ns.output,'w:gz',compresslevel=1) as archive:
    for p in files:
        if p.is_file():archive.add(p,arcname=str(p.relative_to(root)))
print(len(files))
