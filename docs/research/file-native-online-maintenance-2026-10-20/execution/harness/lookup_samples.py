"""Matched six-category fresh/warm samples; no device-cold or fresh p99 claim."""
import argparse
import hashlib
import json
from pathlib import Path
import statistics
from driver import Driver
from job_guard import sha_file, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--exe', required=True)
parser.add_argument('--root', required=True)
parser.add_argument('--baseline', required=True)
parser.add_argument('--output', required=True)
parser.add_argument('--fresh', type=int, choices=[20, 50], default=20)
args = parser.parse_args()
base = json.loads(Path(args.baseline).read_bytes())
out = Path(args.output)
assert not out.exists()
out.mkdir(parents=True)
d = Driver(args.exe, out/'events.jsonl')
root = Path(args.root)
before = (root/'SELECTED.json').read_bytes()
warm = d.call(root, 'lookup-bench', count=500, requests=base['requests'])
assert len(warm['result']['rows']) == 6
for row in warm['result']['rows']:
    expected = base['expected_outcomes'][row['category']]
    expected_digest = hashlib.sha256(json.dumps(expected, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()).hexdigest()
    assert row['result_digest'] == expected_digest and len(row['warm_ns']) == 500
fresh = []
for request in base['requests']:
    for i in range(args.fresh):
        response = d.call(root, 'resolve', request_id=request['id'])
        assert response['result'] == base['expected_outcomes'][request['category']]
        fresh.append(dict(category=request['category'], sample=i,
                          engine_ns=response['wall_ns'], spawn_total_ns=d.last['spawn_total_ns'],
                          result=response['result'], metrics=response['metrics'],
                          observed_peak_rss=d.last['observed_peak_rss'],
                          reported_peak_rss=response.get('peak_memory_bytes')))
assert before == (root/'SELECTED.json').read_bytes()
summary = {}
for request in base['requests']:
    category = request['category']
    rows = [r for r in fresh if r['category'] == category]
    summary[category] = dict(fresh_count=len(rows),
                            engine_median_ns=statistics.median(r['engine_ns'] for r in rows),
                            engine_max_ns=max(r['engine_ns'] for r in rows),
                            spawn_median_ns=statistics.median(r['spawn_total_ns'] for r in rows),
                            spawn_max_ns=max(r['spawn_total_ns'] for r in rows))
write_json(out/'samples.json', dict(executable_sha256=sha_file(args.exe),
           actual_H=base['actual_H'], categories=base['requests'], fresh=fresh,
           warm=warm, summary=summary, selected_unchanged=True,
           qualification='new process; OS/device cache uncontrolled; all raw samples/maxima; engine/spawn clocks separate; no fresh p99 qualification'))
print(json.dumps(dict(status='passed', actual_H=base['actual_H'], fresh=len(fresh), warm_per_category=500)))
