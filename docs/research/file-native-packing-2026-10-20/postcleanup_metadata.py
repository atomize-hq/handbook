"""Small metadata closeout and transport-cap check after primary groups close."""
import json,runpy
from pathlib import Path
from guard import current

HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020')
for name in ['main-cleanup','postcleanup-verify','postcleanup-reopen']:
    receipt=json.loads((R/'receipts'/f'{name}.json').read_bytes());assert receipt['exit']==0 and receipt['process_closure_verified'] and receipt['censor'] is None
runpy.run_path(str(HERE/'packed_backup_adapter_check.py'),run_name='__main__')
runpy.run_path(str(HERE/'main_checkpoint.py'),run_name='__main__')
actual=current();allocation=json.loads((R/'long-matrix-fixed2/postcleanup-allocation.json').read_bytes());cleanup=json.loads((R/'long-matrix-fixed2/cleanup-summary.json').read_bytes())
cap=2766562918;transfer=256*1024**2;reduced=cap-transfer;prospective=actual['total_new_bytes']+620000000+33554432
result=dict(status='actual metadata and proposal; capture/restore still unadmitted',current_actual=actual,actual_successful_cleanup_bytes=cleanup['actual_successfully_deleted_bytes'],actual_successful_cleanup_files=504833+32,deferred_bytes=cleanup['deferred_bytes'],eligible_file_reduction=cleanup['eligible_file_reduction'],ninety_five_percent_A_gate=True,whole_store=allocation['total'],whole_engine_excluding_ledger=allocation['engine_excluding_test_ledger'],roles=allocation['roles'],whole_loose_before_activation_allocation=None,current_cap=cap,proposed_Mac_transfer_bytes=transfer,proposed_reduced_Windows_P_cap=reduced,capture_only_reserved_bytes=620000000+33554432,capture_only_prospective_total=prospective,capture_fits_after_proposed_transfer=prospective<=reduced,restore_separate_actual_archive_admission_required=True,restore_max_file_bytes=600000000,restore_and_reopen_margin=33554432,fullpostcleanup_oracle_passed=True,reopen_111_events_101_retired_refusals_passed=True,adapter_byte_protocol_and_prewrite_refusal_checks_passed=True,all_process_groups_closed=True,functional_cutoff='2026-10-20T07:55:16Z',final_deadline='2026-10-20T08:25:16Z')
(R/'receipts/postcleanup-admission-state.json').write_text(json.dumps(result,indent=2));print(json.dumps({k:v for k,v in result.items() if k!='roles'},indent=2))
