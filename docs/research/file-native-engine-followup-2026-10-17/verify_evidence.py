"""Assert archived logs/pins are internally consistent; never manufactures passes."""
import base64,gzip,hashlib,json,pathlib,zipfile
H=pathlib.Path(__file__).resolve().parent
def sha(b):return hashlib.sha256(b).hexdigest()
pins=json.loads((H/'evidence/pins.json').read_bytes())
linux_source_hashes={line.split()[1]:line.split()[0] for line in pins['linux']['stdout'].splitlines() if len(line.split())==2 and len(line.split()[0])==64}
for name,v in pins['sources'].items():assert linux_source_hashes.get(name)==v['sha256'],('native-source-association',name)
for name,v in pins['sources'].items():assert sha((H/name).read_bytes())==v['sha256'],name
for name,v in pins['harness'].items():assert sha((H/name).read_bytes())==v['sha256'],name
for name,v in pins.get('terminal_helpers',{}).items():assert sha((H/name).read_bytes())==v['sha256'],name
allowed_harness={name:{v['sha256']} for name,v in pins['harness'].items()}
capture_path=H/'evidence/capture-amendment.json'
if capture_path.exists():
    capture=json.loads(capture_path.read_bytes())
    assert capture['new_controller_sha256']==pins['harness']['experiment.py']['sha256'],'capture-current-controller'
amendment_path=H/'evidence/harness-before-wsl-large-budget.json'
if amendment_path.exists():
    amendment=json.loads(amendment_path.read_bytes());archived=H/'evidence'/amendment['archive'];assert sha(archived.read_bytes())==amendment['archive_sha256']
    if capture_path.exists():assert capture['old_controller_sha256']==amendment['source_hashes']['experiment.py'] and capture['old_controller_archive']==amendment['archive'],'capture-archived-controller'
    with zipfile.ZipFile(archived) as z:
        for name,digest in amendment['source_hashes'].items():
            assert sha(z.read(name))==digest,('archived-harness',name);allowed_harness.setdefault(name,set()).add(digest)
    profile=H/'evidence/wsl/manifest--growth-raw.jsonl.gz'
    if profile.exists():assert sha(gzip.decompress(profile.read_bytes())[:amendment['profile_raw_prefix_bytes']])==amendment['profile_raw_prefix_sha256'],'pre-amendment-profile-prefix'
old=H/'evidence/pins-development-v3.json';archive=H/'evidence/source-development-v3.zip'
if old.exists() and archive.exists():
    with zipfile.ZipFile(archive) as z:
        for name,v in json.loads(old.read_bytes())['sources'].items():assert sha(z.read(name))==v['sha256'],('archived-source',name)
real_lanes=['windows','wsl','windows-supplement','wsl-supplement']
probe_lanes=['controller-capture-windows','controller-capture-wsl']
sessions={};session_files={};probes={};probe_scenarios={}
for lane in real_lanes+probe_lanes:
    root=H/'evidence'/lane
    for path in root.glob('*harness-session-*.json'):
        manifest=json.loads(path.read_bytes())
        assert set(manifest['files'])=={'experiment.py','baseline_harness.py','workloads.py','regressions.py','terminal_copy.py'},('harness-file-set',lane,path.name)
        assert sha(json.dumps(manifest['files'],sort_keys=True).encode())==manifest['sha256'],('harness-manifest-aggregate',lane,path.name)
        for name,digest in manifest['files'].items():assert digest in allowed_harness[name],(lane,path.name,name)
        sessions[lane,path.name]=manifest['sha256']
        session_files[lane,path.name]=manifest['files']
    if lane in probe_lanes and root.exists():
        probe=json.loads((root/'base--capture-observations.json').read_bytes())
        assert probe['helper_sha256']==pins['harness']['capture_checks.py']['sha256'],('capture-helper-source',lane)
        assert sha((root/'synthetic-probe.rs').read_bytes())==probe['probe_source_sha256'],('capture-probe-source',lane)
        assert [v['scenario'] for v in probe['checks']]==['valid-json','partial-json','invalid-utf8','empty-success'],('capture-scenarios',lane)
        probes[lane]=probe
        probe_scenarios[lane]=[]
verified=[];distinct_raw={};final_events=0;synthetic_events=0;lossless_events=0
for indexpath in sorted((H/'evidence').glob('*/raw-index.json')):
    for item in json.loads(indexpath.read_bytes()):
        data=(indexpath.parent/item['file']).read_bytes();assert sha(data)==item['compressed_sha256'];raw=gzip.decompress(data);assert sha(raw)==item['sha256'];rows=[json.loads(s) for s in raw.splitlines() if s.strip()];assert len(rows)==item['records'];verified.append(dict(file=str(indexpath.parent.name+'/'+item['file']),records=len(rows)))
        distinct_raw[item['sha256']]=len(rows)
        lane=indexpath.parent.name
        if lane in real_lanes+probe_lanes:
            expected=probes[lane]['probe_binary_sha256'] if lane in probe_lanes else pins['windows_binary' if lane.startswith('windows') else 'linux_binary']['sha256']
            for r in rows:
                if 'binary_sha256' in r or item['source'].endswith('-raw.jsonl'):
                    assert r.get('binary_sha256')==expected,('native-event-binary-association',lane,item['file'])
                    parent=pathlib.PurePosixPath(item['source']).parent.as_posix()
                    prefix='base' if parent=='.' else parent.replace('/','--')
                    session_key=(lane,f"{prefix}--harness-session-{r['session']}.json")
                    assert session_key in sessions and sessions[session_key]==r.get('harness_sha256'),('native-event-session-association',session_key,item['file'],r.get('seq'))
                    if (H/'evidence/capture-amendment.json').exists() and session_files[session_key]['experiment.py']==pins['harness']['experiment.py']['sha256']:
                        assert 'stdout_base64' in r,('lossless-stdout-required',lane,item['file'],r.get('seq'))
                    if lane in probe_lanes:synthetic_events+=1
                    else:final_events+=1
                    if 'stdout_base64' in r:
                        stdout=base64.b64decode(r['stdout_base64'],validate=True)
                        assert len(stdout)==r['stdout_bytes'] and sha(stdout)==r['stdout_sha256'],('lossless-stdout-integrity',lane,item['file'],r.get('seq'))
                        parsed=None;error=None
                        if stdout.strip():
                            try:parsed=json.loads(stdout)
                            except (ValueError,UnicodeError) as exception:error=f'{type(exception).__name__}: {exception}'
                        elif r['exit']==0:error='empty stdout from a successful process'
                        assert json.dumps(parsed,sort_keys=True,allow_nan=False)==json.dumps(r['output'],sort_keys=True,allow_nan=False) and error==r['stdout_parse_error'],('lossless-stdout-interpretation',lane,item['file'],r.get('seq'))
                        lossless_events+=1
                    if lane in probe_lanes:
                        assert r['session']==probes[lane]['session'],('capture-probe-session',lane)
                        scenario=r['command']['scenario'];expected_stdout={'partial-json':b'{"status":','invalid-utf8':b'\xff\xc3','empty-success':b''}
                        probe_scenarios[lane].append(scenario)
                        assert 'stdout_base64' in r,('capture-missing-stdout',lane,scenario)
                        if scenario in expected_stdout:
                            assert stdout==expected_stdout[scenario] and r['output'] is None and r['stdout_parse_error'],('capture-refusal-evidence',lane,scenario)
                            assert r['censored']==(scenario=='partial-json'),('capture-censor',lane,scenario)
                        else:assert scenario=='valid-json' and r['output']==dict(status='ok',result=None) and not r['censored'] and r['exit']==0
for lane,scenarios in probe_scenarios.items():assert sorted(scenarios)==sorted(v['scenario'] for v in probes[lane]['checks']),('capture-exact-event-coverage',lane)
helper_observations=0
for lane in real_lanes:
    root=H/'evidence'/lane
    if not root.exists():continue
    for path in root.glob('*fault-observations.json'):
        for r in json.loads(path.read_bytes()):
            if r.get('phase_oracle')=='must-retain':assert r['selected_at_reopen']
            if r.get('phase_oracle')=='must-not-select':assert not r['selected_at_reopen']
    for label,helper in [('cap-diagnosis','cap_diagnosis.py'),('retained-probe','retained_probe.py'),('continuation','continue_selected.py'),('backend','backend_checks.py'),('restoration','restoration.py'),('indirect-gc','indirect_gc.py')]:
        for path in root.glob(f'*--{label}-observations.json'):
            data=json.loads(path.read_bytes())
            for row in data if isinstance(data,list) else [data]:
                if 'helper_sha256' in row:
                    assert row['helper_sha256']==pins['harness'][helper]['sha256'],('helper-source-association',lane,path.name,helper)
                    helper_observations+=1
budget_path=H/'evidence/contention-budget-observations.json'
if budget_path.exists():
    budget=json.loads(budget_path.read_bytes());within=[]
    for row in budget['lanes']:
        path=H/'evidence'/row['platform']/(row['backend']+'--contention-observations.json');raw=path.read_bytes()
        assert sha(raw)==row['source_sha256'],('contention-budget-source',row['platform'],row['backend'])
        requests=[v for v in json.loads(raw) if 'width' in v]
        assert len(requests)==row['requests'] and max(v['complete_controller_ns'] for v in requests)/1e6==row['max_request_ms']
        assert max(v['attempts'] for v in requests)==row['max_attempts'] and sum(v['retries'] for v in requests)==row['total_retries']
        within.append(row['max_request_ms']<budget['observation_budget_ms'])
    assert all(within)==budget['all_observed_within_budget'],'contention-observed-budget'
result=dict(status='archive-and-pin-consistency-passed',logs=len(verified),records=sum(v['records'] for v in verified),distinct_raw_contents=len(distinct_raw),records_in_distinct_raw_contents=sum(distinct_raw.values()),count_qualification='Archive record totals include duplicate archives; distinct-content totals deduplicate whole identical files only, not cross-file events or test cases. Synthetic transport events are counted separately from real-engine native events.',final_native_events_associated=final_events,synthetic_transport_events_associated=synthetic_events,lossless_stdout_events_checked=lossless_events,helper_observations_associated=helper_observations,verified=verified)
(H/'evidence/verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='verified'}))
