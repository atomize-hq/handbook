"""Matrix suballocation checks layered over the unchanged shared guard budget."""
import os
from pathlib import Path
from job_guard import Budget, census


def excluding(root, omitted):
    root, omitted = Path(root).resolve(), Path(omitted).resolve()
    answer = dict(bytes=0,files=0)
    for child in root.iterdir():
        if child.resolve() == omitted:continue
        assert not child.is_symlink(), 'receipt-symlink'
        row = census(child) if child.is_dir() else dict(bytes=child.stat().st_size,files=1)
        for key in answer:answer[key] += row[key]
    return answer


def check_matrix(reserves=None, path=None):
    reserves = reserves or {}
    budget = Budget(path or os.environ['FILE_NATIVE_BUDGET'])
    measured = budget.check(reserves)
    notes = budget.config.get('allocation_notes',{})
    if 'matrix_nonstream_receipts_max_bytes' in notes:
        receipts = budget.categories['receipts']['root']
        stream = budget.categories['stream_receipts']['root']
        other = excluding(receipts,stream)
        reserve = reserves.get('receipts',{}).get('bytes',0)
        cap = notes['matrix_nonstream_receipts_max_bytes']
        assert other['bytes']+reserve <= cap, 'matrix-nonstream-receipts-prospective-cap'
        measured['matrix_nonstream_receipts'] = dict(actual=other,reserve_bytes=reserve,cap_bytes=cap,
             basis='one traversal excluding stream-control-main; equivalent to receipts minus stream census without racing the stream writer')
    if 'final_protocol_source' in budget.categories:
        rows = measured['categories']
        other = rows['source']['absolute_actual']['bytes']-rows['final_protocol_source']['absolute_actual']['bytes']
        cap = rows['source']['cap_bytes']-rows['final_protocol_source']['cap_bytes']
        assert other+reserves.get('source',{}).get('bytes',0) <= cap, 'reserved-final-protocol-source-cap'
        measured['matrix_source_excluding_reserved_focus'] = dict(actual_bytes=other,cap_bytes=cap)
    return measured
