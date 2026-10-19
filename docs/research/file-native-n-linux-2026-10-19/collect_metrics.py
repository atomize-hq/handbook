"""Readable derived growth counters from retained exact-candidate raw rows."""
import json
from pathlib import Path
base=Path(__file__).resolve().parent
rows=[]
for path in sorted((base/'matrix').glob('*/measurements.json')):
    for r in json.loads(path.read_bytes()):
        m=r['prepare_metrics']
        rows.append({k:r[k] for k in ['n','mode','profile','case','prepare_seconds','select_seconds','refresh_seconds','replay_objects','replay_files','oracle_seconds','changed_paths','navigation_changed_paths']}|dict(document_visits=m['documents_visited'],edge_visits=m['edges_visited'],range_rows=m['range_rows'],range_leaf_pairs=m['range_leaf_pairs'],range_page_visits=m['range_page_visits'],parts_emitted=m['navigation_parts'],navigation_bytes_emitted=m['navigation_bytes'],B_generation_ns=m['ns'].get('B-generation-inclusive'),B_generation_verification_ns=r['select_metrics']['ns'].get('B-generation-verification-inclusive')))
(base/'derived-growth.json').write_text(json.dumps(rows,indent=2))
print(json.dumps([r for r in rows if r['case'] in ['single-payload','single-title','high-degree-title']],indent=2))
