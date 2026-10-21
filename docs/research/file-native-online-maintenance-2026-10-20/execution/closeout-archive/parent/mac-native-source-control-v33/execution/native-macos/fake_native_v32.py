"""Reviewed-control candidate only: fast child has no artificial post-input sleep."""
import hashlib,json,os,sys,time
from pathlib import Path
if '--backpressure' in sys.argv:
    parts=[]
    while True:
        block=sys.stdin.buffer.read(4096)
        if not block:break
        parts.append(block);time.sleep(.002)
    raw=b''.join(parts)
else:raw=sys.stdin.buffer.read()
v=json.loads(raw)
if v['action']=='wait-owned':
    p=Path(v['ready']);p.write_text(json.dumps({'case_id':v['case_id'],'input_sha256':hashlib.sha256(raw).hexdigest()}))
    time.sleep(10)  # Deliberate native lifetime barrier, not a startup proof delay.
if os.environ.get('EXP_EXIT')=='control-ready-exit':raise SystemExit(86)
if v['action']=='publish' and os.environ.get('EXP_CONTROL_CONFLICT')=='1':
    print(json.dumps({'status':'error','error':'conflict:expected-publication','result':None}));raise SystemExit(2)
print(json.dumps({'status':'ok','error':None,'result':{'request_id':v.get('request',{}).get('request'),'input_bytes':len(raw),'input_sha256':hashlib.sha256(raw).hexdigest(),'environment':os.environ.get('EXP_CONTROL_CONFLICT')}}))
