"""Bind caller inputs by root, request ID and independently normalized intent."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
from outcome_oracle import intent, normalized, strict
from job_guard import sha_file, write_json
from matrix_budget import check_matrix

parser = argparse.ArgumentParser()
parser.add_argument('--root', required=True)
parser.add_argument('--events', action='append', required=True)
parser.add_argument('--producer-sha256', required=True)
parser.add_argument('--output', required=True)
args = parser.parse_args()
check_matrix({'receipts':dict(bytes=1024**2,files=3)})
root = Path(args.root).resolve()
by_pair = {}
sources = []
skipped = []
for name in args.events:
    path = Path(name);before = sha_file(path)
    opener = gzip.open if path.suffix == '.gz' else open
    with opener(path,'rb') as stream:
        for number, raw in enumerate(stream,1):
            row = strict(raw);value = row.get('input',{})
            if value.get('action') != 'publish' or Path(value.get('root','')).resolve() != root:continue
            assert row['executable_sha256'] == args.producer_sha256, 'mixed-producer-root-requires-explicit-binding'
            request = value['request']
            evidence = dict(path=str(path.resolve()),line=number,row_sha256=hashlib.sha256(raw).hexdigest(),
                            producer_sha256=row['executable_sha256'],exit=row['exit'])
            try:
                digest = intent(request)
                canonical = normalized(request)
                assert type(canonical['request']) is str
                assert all(type(op[k]) is str for op in canonical['ops'] for k in ['kind','id','text','target'])
            except (AssertionError,KeyError,TypeError,ValueError) as error:
                assert row['exit'] != 0
                skipped.append(dict(**evidence,reason='invalid-caller-request-not-an-accepted-input',error=str(error)))
                continue
            pair = (request['request'],digest)
            if pair in by_pair:
                assert normalized(by_pair[pair]['request']) == canonical
                by_pair[pair]['sources'].append(evidence)
            else:by_pair[pair] = dict(request=request,intent=digest,sources=[evidence])
    assert sha_file(path) == before,'producer-events-changed-during-provenance-read'
    sources.append(dict(path=str(path.resolve()),sha256=before,bytes=path.stat().st_size))
output = Path(args.output)
write_json(output,list(by_pair.values()))
write_json(output.with_suffix('.binding.json'),dict(root=str(root),source_events=sources,
           provenance_sha256=sha_file(output),request_intent_pairs=len(by_pair),skipped_invalid_attempts=skipped,
           establishes_acceptance=False,qualification='caller input evidence only; selected semantic lineage and publication intent choose the matching pair'))
print(json.dumps(dict(status='bound',request_intent_pairs=len(by_pair),invalid_attempts_retained=len(skipped))))
