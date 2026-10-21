"""Pure fake lossless append controls; no Driver/engine integration is executed."""
import gzip
import hashlib
import json
import os
from pathlib import Path
import sys
from event_log import append_event
from job_guard import write_json,sha_file
from matrix_budget import check_matrix

assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1' and sys.dont_write_bytecode
root,output = Path(sys.argv[1]).resolve(),Path(sys.argv[2]).resolve()
receipts = Path('C:/file-native-online-matrix-20261020/receipts').resolve()
assert root.is_relative_to(receipts) and output.is_relative_to(receipts)
assert not any(p.is_relative_to(receipts/'stream-control-main') for p in [root,output])
assert not root.exists() and not output.exists()
check_matrix({'receipts':dict(bytes=256*1024,files=10)})
root.mkdir()
plain,packed,legacy = root/'events.jsonl',root/'events.jsonl.gz',root/'legacy-reference.jsonl'
rows = [dict(input=dict(action='fake',request=dict(request='name-'+str(i),ops=[],text='é\\n雪')),
    executable_sha256='a'*64,exit=0,stdout=json.dumps(dict(result='ok',samples=[j*17+i for j in range(100)])),
    stderr='',process_started_ns=i*1000,process_done_ns=i*1000+999,observed_peak_rss=12345,
    violations=[],raw_bytes_marker='\u0000') for i in range(17)]
for row in rows:
    append_event(plain,row)
    append_event(packed,row)
    with legacy.open('a',encoding='utf-8') as stream:
        stream.write(json.dumps(row,separators=(',',':'))+'\n')
        stream.flush()
expected=legacy.read_bytes()
assert plain.read_bytes() == expected
with gzip.open(packed,'rb') as stream:
    decoded=stream.read()
assert decoded == expected and [json.loads(line) for line in decoded.splitlines()] == rows
truncated=root/'truncated.jsonl.gz';truncated.write_bytes(packed.read_bytes()[:-3])
try:
    with gzip.open(truncated,'rb') as stream:stream.read()
except (EOFError,OSError) as error:
    truncation=dict(refused=True,error_type=type(error).__name__,reason=str(error))
else:
    raise AssertionError('incomplete-gzip-tail-accepted')
assert not (root/'unsupported.txt').exists()
try:
    append_event(root/'unsupported.txt',rows[0])
except AssertionError:
    pass
else:
    raise AssertionError('unsupported-event-format-accepted')
assert not (root/'unsupported.txt').exists()
write_json(output,dict(status='passed',module_sha256=sha_file(Path(__file__).with_name('event_log.py')),
    rows=len(rows),all_fields_and_line_bytes_preserved=True,legacy_plain_bytes_identical=True,
    decoded_sha256=hashlib.sha256(decoded).hexdigest(),gzip_sha256=sha_file(packed),
    compressed_bytes=packed.stat().st_size,decoded_bytes=len(decoded),truncated_tail=truncation,
    no_engine=True,no_Driver_changes=True,
    qualification='synthetic lossless codec proof only; synthetic compression ratio is not a final workload forecast'))
print(json.dumps(dict(status='passed',rows=len(rows))))
