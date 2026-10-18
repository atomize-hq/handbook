"""Prove ancestor-name failures, then verify root-relative collection/counters."""
import argparse,hashlib,json
from controller import Driver
from checks import result,expect_error

def run(d,expect_bug=False):
    root=d.base/'objects'/'results'/'disposable'/'store';r=str(root)
    initial=d.call(dict(action='init',root=r,mode='A',consumer='project',count=16))
    docs=result(d.call(dict(action='dump',root=r)))['state']['docs']
    sources={v['original']:v['digest'] for v in docs.values()}
    d.call(dict(action='lifecycle',root=r,operation='setup'))
    d.call(dict(action='lifecycle',root=r,operation='collect'))
    remaining=sum((root/p).exists() for p in sources)
    audit=d.call(dict(action='audit',root=r),allow=expect_bug)
    if expect_bug:
        assert remaining==0;expect_error(audit)
    else:
        assert remaining==len(sources)
        for path,h in sources.items():assert hashlib.sha256((root/path).read_bytes()).hexdigest()==h
        m=initial['metrics'];assert m['bytes_payload_written']>100000 and 0<m['bytes_result_written']<8192 and m['bytes_control_written']>0
        for suffix,total in [('read','bytes_read'),('written','bytes_written')]:
            assert sum(m[f'bytes_{role}_{suffix}'] for role in ['metadata','payload','control','result'])==m[total]
    return {'expected_prior_bug':expect_bug,'selected_sources_before':len(sources),'selected_sources_remaining':remaining,'initial_byte_counters':{k:v for k,v in initial['metrics'].items() if k.startswith('bytes_')}}

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--expect-bug',action='store_true');ns=a.parse_args()
    d=Driver(ns.exe,ns.base,900);(d.base/'namespace-cases.json').write_text(json.dumps(run(d,ns.expect_bug),indent=2))
