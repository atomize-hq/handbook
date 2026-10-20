"""Quiescent full independent request-intent qualification at a real H boundary."""
import argparse
import json
from pathlib import Path
from outcome_oracle import verify
from job_guard import write_json

parser = argparse.ArgumentParser()
parser.add_argument('--root', required=True)
parser.add_argument('--output', required=True)
args = parser.parse_args()
row = verify(args.root)
write_json(args.output, row)
print(json.dumps(row))
