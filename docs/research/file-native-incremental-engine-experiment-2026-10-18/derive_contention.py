"""Derive wave/request observations without treating them as throughput trials."""
import argparse,hashlib,json,zipfile
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
base=Path(ns.scratch)/'contention-final';path=base/'contention.json';raw=path.read_bytes();records=json.loads(raw);waves=[]
for workers in [1,2,4,8]:
 group=[row for row in records if row['workers']==workers];assert len(group)==workers
 pending=group[0]['pending'];assert all(row['pending']==pending for row in group)
 archive=base/pending['snapshot_archive'];archive_raw=archive.read_bytes();assert hashlib.sha256(archive_raw).hexdigest()==pending['snapshot_sha256']
 with zipfile.ZipFile(archive) as snapshot:
  assert snapshot.testzip() is None,'Pending snapshot CRC failure'
  files=[entry for entry in snapshot.infolist() if entry.filename!='OBSERVED-RESERVATIONS.json']
  assert len(files)==pending['new_unique_files'] and sum(entry.file_size for entry in files)==pending['new_unique_file_bytes']
  assert json.loads(snapshot.read('OBSERVED-RESERVATIONS.json'))==pending['observed_reservations']
 requests=[]
 for row in group:
  attempts=row['all_attempts'];failed=[out for out in attempts if out['status']!='ok']
  assert attempts[-1]['result']['status']=='published' and attempts[-1]['result']['result']['outcome']=='accepted'
  elapsed=row['request_all_attempt_ns']/1e9
  writes=[out['metrics'].get('bytes_written') for out in failed];reads=[out['metrics'].get('page_reads') for out in failed]
  waits=[out['metrics'].get('ns',{}).get('lock-wait') for out in attempts];holds=[out['metrics'].get('ns',{}).get('lock-hold') for out in attempts]
  requests.append(dict(request=row['request'],attempts=len(attempts),failed_attempts=len(failed),errors=[out.get('error') for out in failed],all_attempt_seconds=elapsed,within_declared_aggregate_180_seconds=elapsed<=180,failed_attempt_logical_write_bytes=sum(writes) if all(v is not None for v in writes) else None,failed_attempt_object_reads=sum(reads) if all(v is not None for v in reads) else None,sum_lock_wait_ms=sum(waits)/1e6 if all(v is not None for v in waits) else None,sum_lock_hold_ms=sum(holds)/1e6 if all(v is not None for v in holds) else None,post_request_reservations=row['post_request_reservations']))
 waves.append(dict(workers=workers,requests=requests,total_attempts=sum(row['attempts'] for row in requests),accepted_requests=len(requests),failed_attempts=sum(row['failed_attempts'] for row in requests),min_request_seconds=min(row['all_attempt_seconds'] for row in requests),max_request_seconds=max(row['all_attempt_seconds'] for row in requests),all_observed_requests_within_aggregate_180_seconds=all(row['within_declared_aggregate_180_seconds'] for row in requests),observed_reservations=len(pending['observed_reservations']),reserved_bytes=pending['reserved_bytes'],new_unique_files=pending['new_unique_files'],new_unique_file_bytes=pending['new_unique_file_bytes'],existing_file_growth_bytes=pending['existing_file_growth_bytes'],census_and_archive_pause_seconds=pending['census_and_archive_pause_seconds'],snapshot_archive=pending['snapshot_archive'],snapshot_sha256=pending['snapshot_sha256']))
result=dict(source='contention-final/contention.json',source_sha256=hashlib.sha256(raw).hexdigest(),waves=waves,qualification='Controlled preparation barrier, file census/archive, cleanup and pure retries are included in request wall time. One request per wave edits a5002-edge adjacency; others change scattered labels. Failed-attempt logical API writes are not unique discarded physical bytes. Code caps each call at180s and at most4 attempts; observed whole-request180s compliance is reported separately, not asserted as an implemented aggregate watchdog.')
Path(ns.output).write_text(json.dumps(result,indent=2));print(json.dumps([{k:v for k,v in wave.items() if k!='requests'} for wave in waves],indent=2))
