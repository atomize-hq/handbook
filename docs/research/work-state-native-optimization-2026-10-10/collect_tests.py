"""Collect the completed bounded suite results without treating ignored tests as passes."""
import json,pathlib,re,sys
here=pathlib.Path(sys.argv[1]); result=[]
expected={'tests-research.txt':1,'tests-engine.txt':3,'tests-sdk.txt':7}
for filename,count in expected.items():
    raw=(here/filename).read_bytes()
    log=raw.decode('utf-16' if raw.startswith(b'\xff\xfe') else 'utf-8-sig')
    matches=list(re.finditer(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;.*?finished in ([0-9.]+)s',log))
    assert len(matches)==count and 'test result: FAILED' not in log, f'incomplete/failing {filename}'
    for m in matches:
        before=log[:m.start()]; heading=next((line.strip() for line in reversed(before.splitlines()) if 'Running ' in line),'focused research library tests')
        passed,failed,ignored,seconds=m.groups();assert int(failed)==0
        result.append({'log':filename,'suite':heading,'passed':int(passed),'failed':int(failed),'ignored':int(ignored),'seconds':float(seconds)})
summary={'status':'passed','passed':sum(r['passed'] for r in result),'failed':0,
         'ignored':sum(r['ignored'] for r in result),'suites':result}
(here/'test-summary.json').write_text(json.dumps(summary,indent=2)+'\n',encoding='utf-8')
print(json.dumps(summary,indent=2))
