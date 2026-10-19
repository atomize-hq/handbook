"""Derive bounded review tables without changing raw receipts."""
import json,hashlib,subprocess
from pathlib import Path
BASE=Path(__file__).resolve().parent
R=BASE/'evidence/native-raw'
read=lambda p:json.loads((R/p).read_bytes())
focused=read('focused-data/summary.json');recovery=read('focused-data/recovery-capsule/summary.json')
cells=read('matrix-cells.json');assert len(cells)==4
measurements=[];events=read('focused-data/events.json')+read('supplement-data/events.json')+read('historical2-data/events.json')
for cell in cells:
    label=cell['cell'];rows=read('matrix/'+label+'/measurements.json');assert len(rows)==13
    assert all(r['oracle']['immutable_discovery_semantics_verified'] and r['oracle']['discovery_relationships_and_extraction'] for r in rows)
    measurements.extend(rows);events.extend(read('matrix/'+label+'/events.json'))
assert len(focused['cases'])==39 and len(recovery['cases'])==24 and len(measurements)==52
assert all(not r['violations'] for r in events)
capsule_resources=read('focused-data/capsule-resources.json');assert all(not r['violations'] for r in capsule_resources)
for mode in ['A','B']:
    for n in [16,128]:assert read(f'restored-operational-{mode}-{n}.json')['sources']>0
stage1=read('stage1-data/summary.json');assert len(stage1)==10
all_stages=read('stages.json')+read('matrix-stages.json')
unique={r['stage']:r for r in all_stages};passed=[r for name,r in unique.items() if name!='matrix-controller']
assert all(r['exit']==0 and r['censor'] is None for r in passed)
sampled=[r['sampled_peak_rss'] for r in events+capsule_resources if r['sampled_peak_rss'] is not None]
row=dict(source_pin=read('source-manifest.json')['checkout'],source_src_tree=subprocess.check_output(['git','rev-parse','6e66a7523612b80386cf9cc2b96d5341579b54fd:docs/research/file-native-incremental-engine-experiment-2026-10-18/src']).decode().strip(),focused_cases=39,focused_calls=focused['events'],recovery_cases=24,recovery_calls=recovery['events'],supplement_cases=len(read('supplement-data/summary.json')),matrix_cells=4,matrix_measurements=52,fact_dirtiness_control_rows=10,historical2_cases=len(read('historical2-data/summary.json')),restored_full_operational_oracles=4,point_child_resource_samples_missing=sum(r['sampled_peak_rss'] is None for r in events+capsule_resources),resource_rows=len(events+capsule_resources),maximum_sampled_engine_child_rss=max(sampled),maximum_sampled_stage_group_rss=max(r['peak_aggregate_rss'] for r in passed),maximum_ordinary_driver_child_seconds=max(r['seconds'] for r in events+capsule_resources),all_returned_completed_assertions_passed=True,N10000_A='scope-cancelled before selection; preserved, unqualified',N10000_B='unperformed',extra_overlap_capacity_snapshot='refused, original receipt retained',graph='UNKNOWN; cached tool refused unregistered worktree')
(BASE/'evidence/review-summary.json').write_text(json.dumps(row,indent=2))
print(json.dumps(row,indent=2))
