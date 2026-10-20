#!/usr/bin/python3
"""CONTROL ONLY: fake JSON peer; never reads/writes an engine store."""
import json
import resource
import sys
import time

request = json.load(sys.stdin)
time.sleep(request.get('control_sleep', .04))
result = dict(control_only=True, action=request['action'],
              request_id=request.get('request_id'), request=request.get('request'))
print(json.dumps(dict(result=result,
                     peak_memory_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024)))
raise SystemExit(2 if request['action'] == 'control-error' else 0)
