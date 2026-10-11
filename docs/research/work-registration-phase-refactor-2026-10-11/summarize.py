"""Verify paired native evidence and retain reproducible descriptive summaries."""
import collections,hashlib,itertools,json,pathlib,random,re,statistics,sys
HERE=pathlib.Path(__file__).resolve().parent
def read(name):return [json.loads(line) for line in (HERE/name).read_text(encoding='utf-8').splitlines() if line.strip()]
def describe(values):
    return {'n':len(values),'min':min(values),'median':statistics.median(values),'mean':statistics.mean(values),'max':max(values),'sample_sd':statistics.stdev(values) if len(values)>1 else 0}
def paired(rows):
    table={}
    for row in rows:
        key=(row['cohort'],row['history'],row['sample']);assert row['variant'] not in table.setdefault(key,{})
        table[key][row['variant']]=row
    for key,pair in table.items():assert set(pair)=={'baseline','refactor'},key
    return table
def logical_input(path):
    value=json.loads((path/'fixture.input.json').read_text(encoding='utf-8'))
    if value['composition']['form']=='kind':value['composition']['binding']['project_identity']='independent-physical-root'
    return value
def parity(group):
    rows=read(group+'-population.jsonl');table={}
    for row in rows:table.setdefault((row['cohort'],row['history'],row['sample']),{})[row['root_variant']]=row
    checks=[]
    for key,p in sorted(table.items()):
        if set(p)!={'baseline','refactor'}:raise ValueError('Incomplete population '+str(key))
        b,r=p['baseline'],p['refactor']
        for field in ['work_bytes','native_bytes','basis','grant_lineage','role','kind_versions','large_native','near_bound_utf8']:assert b[field]==r[field],(key,field,b[field],r[field])
        left=pathlib.Path(b['root']);right=pathlib.Path(r['root'])
        assert logical_input(left)==logical_input(right),(key,'logical_input')
        lf=json.loads((left/'fixture.grant.json').read_text());rf=json.loads((right/'fixture.grant.json').read_text())
        assert lf['terms']==rf['terms'],(key,'grant_terms')
        files_l={p.relative_to(left).as_posix():p.stat().st_size for p in left.rglob('*') if p.is_file() and p.relative_to(left).as_posix().startswith('.handbook')}
        files_r={p.relative_to(right).as_posix():p.stat().st_size for p in right.rglob('*') if p.is_file() and p.relative_to(right).as_posix().startswith('.handbook')}
        # These owner paths contain independent physical-root fingerprints.
        # Normalize only their documented positions, preserving multiplicity
        # and each leaf size. Work transaction/slot paths remain exact.
        def shape(path):
            path=re.sub(r'^(\.handbook/local-charter/v1/records/(?:candidate|intake|validation|approval|promotion|lifecycle)_)[0-9a-f]{64}(\.json)$',r'\1<independent-fingerprint>\2',path)
            return re.sub(r'^(\.handbook/admission-governance/v2/requests/)[0-9a-f]{64}(/(?:intent|instruction|rule|rules|decision|deferral|outcome|relation|consumed|terminal|configuration|current-displaced|predecessor|request|terms|stage-[0-6])\.json)$',r'\1<independent-fingerprint>\2',path)
        assert collections.Counter((shape(path),size) for path,size in files_l.items())==collections.Counter((shape(path),size) for path,size in files_r.items()),(key,'complete-native/work-file-shape-byte-sizes')
        checks.append({'cohort':key[0],'history':key[1],'sample':key[2],'native_bytes':b['native_bytes'],'work_bytes':b['work_bytes'],'native_and_work_files':len(files_l),'logical_input_sha256':hashlib.sha256(json.dumps(logical_input(left),sort_keys=True,separators=(',',':')).encode()).hexdigest(),
                       'baseline_raw_file_sizes':files_l,'refactor_raw_file_sizes':files_r,'fingerprint_path_normalization_only':True,
                       'baseline_normalization_mapping':{path:shape(path) for path in files_l if path!=shape(path)},
                       'refactor_normalization_mapping':{path:shape(path) for path in files_r if path!=shape(path)}})
    result={'group':group,'pairs':len(checks),'checks':checks}
    (HERE/(group+'-byte-parity.json')).write_text(json.dumps(result,indent=2)+'\n');return result
def summarize_rows(rows,required):
    pairs=paired(rows);assert len(pairs)==required
    result=[]
    for (cohort,history),items in itertools.groupby(sorted(pairs.items()),key=lambda x:x[0][:2]):
        p=[v for _,v in items];out={'cohort':cohort,'history':history,'paired_trials':len(p)}
        for metric in ['preparation_wall_ms','invocation_wall_ms','saving_wall_ms','pair_wall_ms','preparation_cpu_ms','invocation_cpu_ms','pair_cpu_ms']:
            b=[x['baseline'][metric] for x in p];r=[x['refactor'][metric] for x in p];d=[a-z for a,z in zip(b,r)]
            ratios=[100*(a-z)/a for a,z in zip(b,r)] if all(x>0 for x in b) else []
            data={'baseline':describe(b),'refactor':describe(r),'paired_saved':describe(d),'paired_refactor_faster':sum(x>0 for x in d)}
            if ratios:data['paired_reduction_percent']=describe(ratios)
            out[metric]=data
        # Exact sign-flip enumeration provides a bounded paired statistic, not
        # a production SLA or a claim that the diverse profiles are identical.
        d=[x['baseline']['pair_wall_ms']-x['refactor']['pair_wall_ms'] for x in p]
        observed=abs(sum(d));sums=[abs(sum(v*s for v,s in zip(d,signs))) for signs in itertools.product([-1,1],repeat=len(d))]
        out['pair_sign_flip_two_sided_p']=sum(x>=observed-1e-9 for x in sums)/len(sums)
        rnd=random.Random(20261011+history);medians=[]
        for _ in range(10000):medians.append(statistics.median([d[rnd.randrange(len(d))] for _ in d]))
        medians.sort();out['paired_median_saved_ms_bootstrap_95_percentile']=[medians[249],medians[9749]]
        result.append(out)
    return result
def count(profile,label):return sum(x['calls'] for x in profile['spans'] if x['symbol']==label)
def attribution():
    rows=read('attribution-measurements.jsonl');on=[x for x in rows if x['instrumented']];off=[x for x in rows if not x['instrumented']]
    assert len(on)==len(off)==12
    counts=[]
    for row in on:
        prep=row['preparation_profile'];inv=row['invocation_profile'];v=row['variant'];c=row['cohort']
        p=[count(prep,x) for x in ['work_scan','work_state/store.rs:inventory','work_state/store.rs:manifest']]
        expected=[2,2,2] if v=='baseline' else ([0,0,0] if c=='explicit' else [1,1,1]);assert p==expected,(c,v,'prep',p)
        n=[count(inv,x) for x in ['work_scan','work_state/store.rs:inventory','work_state/store.rs:manifest']];assert n==[4,5,6],(c,v,'invoke',n)
        assert count(inv,'local_authority/work_registration.rs:validate_current')==8
        if v=='refactor':assert count(prep,'local_authority/work_registration.rs:validate_current')==0 and count(prep,'local_authority/local_check.rs:work_registration_current')==0
        assert prep['retained_locks_after_phase']==inv['retained_locks_after_phase']==0
        counts.append({'cohort':c,'history':row['history'],'variant':v,'preparation_scans_inventory_manifest':p,'invocation_scans_inventory_manifest':n,
            'invocation_validate_current':count(inv,'local_authority/work_registration.rs:validate_current'),'by_observation':inv,'preparation_profile':prep})
    controls=[]
    for row in on:
        mate=next(x for x in off if (x['cohort'],x['history'],x['variant'])==(row['cohort'],row['history'],row['variant']))
        assert row['work_bytes']==mate['work_bytes'] and row['locator_bytes']==mate['locator_bytes']
        controls.append({'cohort':row['cohort'],'history':row['history'],'variant':row['variant'],'enabled_invocation_wall_ms':row['invocation_wall_ms'],'disabled_invocation_wall_ms':mate['invocation_wall_ms'],
            'enabled_minus_disabled_invocation_ms':row['invocation_wall_ms']-mate['invocation_wall_ms'],'pair_includes_preparation_profile_materialization':True})
    return {'structural_checks':counts,'enabled_disabled_controls':controls}
def main():
    if len(sys.argv)>1 and sys.argv[1]=='parity':
        for group in sys.argv[2:]:print(json.dumps({k:v for k,v in parity(group).items() if k!='checks'}))
        return
    rows=read('pristine-measurements.jsonl');assert len(rows)==72 and all(not x['instrumented'] for x in rows)
    for key,p in paired(rows).items():
        assert p['baseline']['work_bytes']==p['refactor']['work_bytes'] and p['baseline']['locator_bytes']==p['refactor']['locator_bytes'],key
        assert {x['within_pair_order'] for x in p.values()}=={0,1}
        assert all(x['handles_after']==x['handles_before'] for x in p.values())
    result={'pristine':summarize_rows(rows,36),'controls':summarize_rows(read('controls-measurements.jsonl'),6),'attribution':attribution()}
    for cell in result['pristine']:
        assert cell['pair_wall_ms']['paired_refactor_faster']==6,('unresolved paired benefit',cell['cohort'],cell['history'])
        assert cell['paired_median_saved_ms_bootstrap_95_percentile'][0]>0
    (HERE/'summary.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
    lines=['| Input / history | Baseline preparation / invocation / actual pair (ms) | Refactor preparation / invocation / actual pair (ms) | Median paired saving (ms) | Median paired reduction | Faster pairs |','| --- | --- | --- | --- | --- | --- |']
    for c in result['pristine']:
        b=[c[m]['baseline']['median'] for m in ['preparation_wall_ms','invocation_wall_ms','pair_wall_ms']];r=[c[m]['refactor']['median'] for m in ['preparation_wall_ms','invocation_wall_ms','pair_wall_ms']]
        lines.append(f"| {c['cohort']} / {c['history']} | {' / '.join(f'{x:.2f}' for x in b)} | {' / '.join(f'{x:.2f}' for x in r)} | {c['pair_wall_ms']['paired_saved']['median']:.2f} | {c['pair_wall_ms']['paired_reduction_percent']['median']:.2f}% | 6/6 |")
    (HERE/'performance-tables.md').write_text('\n'.join(lines)+'\n',encoding='utf-8');print(json.dumps({'main_observations':72,'paired_cells':len(result['pristine']),'accepted_positive_pairs':36}))
if __name__=='__main__':main()
