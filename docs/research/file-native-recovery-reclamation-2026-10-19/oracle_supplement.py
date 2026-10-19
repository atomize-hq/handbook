"""Supplement within the original native lane's deadline; same core binary."""
import hashlib,json,sys
from pathlib import Path
from bounded import run
source=Path(__file__).resolve().parent;root=Path(sys.argv[1]);prior=json.loads((root/'finished.json').read_bytes());deadline=prior['rows'][0]['start_epoch']+1800;label=sys.argv[2] if len(sys.argv)>2 else 'oracle-small-v2'
row=run([sys.executable,source/'capsule.py','--exe',root/'build/release/file-native-incremental','--scratch',root/label],root/(label+'-supplement'),300,deadline)
row['source_sha256']={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [source/'capsule.py',source/'oracle.py',source/'operational_backup.py',source/'oracle_supplement.py']}
(root/(label+'-supplement-binding.json')).write_text(json.dumps(row,indent=2));print(json.dumps(row,indent=2))
