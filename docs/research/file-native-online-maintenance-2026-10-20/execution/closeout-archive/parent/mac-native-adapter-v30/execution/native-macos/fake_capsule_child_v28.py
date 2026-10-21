"""Tiny Python protocol control; never imports or executes an engine."""
import json,os,sys,time
v=json.loads(sys.stdin.buffer.read());time.sleep(.15)
if v['action']=='interrupt':raise SystemExit(86)
if v['action']=='duplicate':print('{"duplicate":1,"duplicate":2}');raise SystemExit(0)
if v['action']=='overflow':sys.stdout.buffer.write(b'x'*100000);sys.stdout.buffer.flush();time.sleep(.1);raise SystemExit(0)
r={'status':'error' if v['action']=='refuse' else 'ok','error':'expected-refusal' if v['action']=='refuse' else None,'result':{'action':v['action'],'body':'R'*50000,'environment':os.environ.get('EXP_CONTROL')}}
sys.stderr.write('fake-child-stderr\n');print(json.dumps(r));raise SystemExit(2 if v['action']=='refuse' else 0)
