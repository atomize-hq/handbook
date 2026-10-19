"""Additional conservative N10k coexistence admission including 64MiB return/Git share."""
import json,time
import supervise as s
current=s.census(s.BASE)
forecast=3*126976000+64*1024**2
row=dict(epoch=time.time(),current_native=current,largest_next_B_fixture_forecast_bytes=forecast,local_source_return_shared_git_growth_allowance_bytes=64*1024**2,worst_case_combined_additional_bytes=current['logical_bytes']+forecast+64*1024**2,lane_limit_bytes=768*1024**2,hard_observed_native_stop_bytes=700*1024**2,hard_native_plus_local_allowance_bytes=764*1024**2,new_files_forecast=current['files']+50000,global_forecast_including_both_native_lanes_and_reserved_bytes=40828300920,global_limit_bytes=40*1024**3)
(s.BASE/'n10k-conservative-admission.json').write_text(json.dumps(row,indent=2))
assert row['worst_case_combined_additional_bytes']<=row['lane_limit_bytes']
assert row['new_files_forecast']<=100000
print(json.dumps(row,indent=2))
