"""Matrix suballocation checks layered over the unchanged shared guard budget."""
import os
from pathlib import Path
from job_guard import Budget, census


def excluding(root, omitted):
    root = Path(root).resolve()
    paths = omitted if isinstance(omitted,(list,tuple)) else [omitted]
    omitted = {Path(path).resolve() for path in paths}
    answer = dict(bytes=0,files=0)
    for child in root.iterdir():
        if child.resolve() in omitted:continue
        assert not child.is_symlink(), 'receipt-symlink'
        row = census(child) if child.is_dir() else dict(bytes=child.stat().st_size,files=1)
        for key in answer:answer[key] += row[key]
    return answer


def check_matrix(reserves=None, path=None):
    reserves = reserves or {}
    budget = Budget(path or os.environ['FILE_NATIVE_BUDGET'])
    measured = budget.check(reserves)
    notes = budget.config.get('allocation_notes',{})
    receipt_reserved = budget.config.get('receipt_reservation_categories')
    if receipt_reserved is not None:
        assert type(receipt_reserved) is list and len(set(receipt_reserved)) == len(receipt_reserved)
        assert all(budget.categories[name].get('nonadditive_subset_of') == 'receipts' for name in receipt_reserved)
        actual = excluding(budget.categories['receipts']['root'],
            [budget.categories[name]['root'] for name in receipt_reserved])
        caps = {key:budget.categories['receipts'][key]-sum(budget.categories[name][key] for name in receipt_reserved)
                for key in ['bytes','files']}
        # The Landing stream receipts live outside receipts/ but share F12.
        # Reserve their complete caps rather than spending their unused space.
        for group in budget.config.get('groups',{}).values():
            if 'receipts' in group['categories']:
                external = [name for name in group['categories'] if name != 'receipts']
                for key in caps:
                    caps[key] = min(caps[key],group[key]-sum(budget.categories[name][key] for name in external+receipt_reserved))
        caps['bytes'] = min(caps['bytes'],notes.get('matrix_excluding_reserved_receipts_max_bytes',caps['bytes']))
        pending = reserves.get('receipts',{})
        assert all(actual[key]+pending.get(key,0) <= caps[key] for key in caps), 'matrix-other-receipts-prospective-cap'
        measured['matrix_nonstream_receipts'] = dict(actual=actual,reserve_bytes=pending.get('bytes',0),
            reserve_files=pending.get('files',0),cap_bytes=caps['bytes'],cap_files=caps['files'],
            reservation_categories=receipt_reserved,basis='single traversal excluding every reserved receipt child; full external group caps reserved')
    elif 'matrix_nonstream_receipts_max_bytes' in notes:
        receipts = budget.categories['receipts']['root']
        stream = budget.categories['stream_receipts']['root']
        other = excluding(receipts,stream)
        reserve = reserves.get('receipts',{}).get('bytes',0)
        cap = notes['matrix_nonstream_receipts_max_bytes']
        assert other['bytes']+reserve <= cap, 'matrix-nonstream-receipts-prospective-cap'
        measured['matrix_nonstream_receipts'] = dict(actual=other,reserve_bytes=reserve,cap_bytes=cap,
             basis='one traversal excluding stream-control-main; equivalent to receipts minus stream census without racing the stream writer')
    reserved = budget.config.get('source_reservation_categories',
        ['final_protocol_source'] if 'final_protocol_source' in budget.categories else [])
    assert type(reserved) is list and len(set(reserved)) == len(reserved)
    if reserved:
        rows = measured['categories']
        assert all(budget.categories[name].get('nonadditive_subset_of') == 'source' for name in reserved)
        other = rows['source']['absolute_actual']['bytes']-sum(rows[name]['absolute_actual']['bytes'] for name in reserved)
        cap = rows['source']['cap_bytes']-sum(rows[name]['cap_bytes'] for name in reserved)
        assert other+reserves.get('source',{}).get('bytes',0) <= cap, 'reserved-final-protocol-source-cap'
        other_files = rows['source']['absolute_actual']['files']-sum(rows[name]['absolute_actual']['files'] for name in reserved)
        file_cap = budget.categories['source']['files']-sum(budget.categories[name]['files'] for name in reserved)
        assert other_files+reserves.get('source',{}).get('files',0) <= file_cap, 'reserved-sibling-source-file-cap'
        measured['matrix_source_excluding_reserved_focus'] = dict(actual_bytes=other,cap_bytes=cap,
            actual_files=other_files,cap_files=file_cap,reservation_categories=reserved)
    return measured
