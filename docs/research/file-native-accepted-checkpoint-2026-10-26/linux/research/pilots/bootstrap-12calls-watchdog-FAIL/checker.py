import json,os,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parent
PACKET=ROOT/'source/mac-sdk-20261025'
os.environ['FILE_NATIVE_ORACLE_RESEARCH']=str(PACKET/'research')
sys.path.insert(0,str(PACKET/'sdk'))
import physical_decoder_v6 as oracle
config=json.loads(Path(sys.argv[1]).read_text())
oracle.FINAL_SOURCE_PIN=config['baseline_pin']
answer=oracle.verify(config['root'],config['output'],quiescent=True,
                     known_requests=config['known_requests'],quota_evidence={})
print(json.dumps({'output':config['output'],'actual_H':answer.get('semantic',{}).get('publications'),
                  'schema_pin':answer['schema_pin'],'request_intents_present':bool(answer['request_intents'])}))
