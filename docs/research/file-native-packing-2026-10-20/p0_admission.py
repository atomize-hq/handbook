import json
from pathlib import Path
from guard import current
r=current();r.update(total_cap_bytes=2684354560,total_cap_files=560000,P0_new_bound=644081109,measurement_margin=33554432)
r['prospective_bytes']=r['total_new_bytes']+r['P0_new_bound']+r['measurement_margin'];r['prospective_files']=r['total_new_files']+2000
Path('C:/file-native-packing-20261020/receipts/p0-fresh-admission.json').write_text(json.dumps(r,indent=2));print(json.dumps(r,indent=2))
assert r['prospective_bytes']<=r['total_cap_bytes'] and r['prospective_files']<=r['total_cap_files'] and r['free_bytes']>=140*1024**3
