"""Small end-to-end adversarial checks of exported evidence interpretation."""
import base64,gzip,hashlib,json,pathlib,shutil,subprocess,sys,tempfile
H=pathlib.Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix='file-native-reporting-') as tmp:
    root=pathlib.Path(tmp);e=root/'evidence';lane=e/'windows';lane.mkdir(parents=True)
    for name in ['verify_evidence.py','tables.py']:shutil.copyfile(H/name,root/name)
    files={}
    for name in ['experiment.py','baseline_harness.py','workloads.py','regressions.py','terminal_copy.py']:
        source=('fixture harness '+name).encode();(root/name).write_bytes(source);files[name]=hashlib.sha256(source).hexdigest()
    aggregate=hashlib.sha256(json.dumps(files,sort_keys=True).encode()).hexdigest()
    (e/'pins.json').write_text(json.dumps(dict(sources={},harness={name:dict(sha256=digest) for name,digest in files.items()},linux=dict(stdout=''),windows_binary=dict(sha256='native-fixture'),linux_binary=dict(sha256='linux-fixture'))))
    (e/'capture-amendment.json').write_text(json.dumps(dict(new_controller_sha256=files['experiment.py'])))
    (lane/'base--harness-session-22.json').write_text(json.dumps(dict(files=files,sha256=aggregate)))
    stdout=b'{"status":"ok","flag":true}\n'
    event=dict(binary_sha256='native-fixture',session=22,harness_sha256=aggregate,seq=1,exit=0,output=dict(status='ok',flag=True),stdout_base64=base64.b64encode(stdout).decode(),stdout_bytes=len(stdout),stdout_sha256=hashlib.sha256(stdout).hexdigest(),stdout_parse_error=None)
    for mutation in ['valid','different-harness','unknown-session','unknown-session-missing-harness','missing-native-pins','incomplete-manifest','stdout-digest','stdout-length','stdout-interpretation','stdout-bool-number','stdout-missing']:
        row=dict(event)
        manifest=dict(files=files,sha256=aggregate)
        if mutation=='incomplete-manifest':
            subset={'experiment.py':files['experiment.py']};subset_hash=hashlib.sha256(json.dumps(subset,sort_keys=True).encode()).hexdigest()
            manifest=dict(files=subset,sha256=subset_hash);row['harness_sha256']=subset_hash
        (lane/'base--harness-session-22.json').write_text(json.dumps(manifest))
        if mutation=='different-harness':row['harness_sha256']='unknown'
        if mutation=='unknown-session':row['session']=23
        if mutation=='unknown-session-missing-harness':row['session']=23;del row['harness_sha256']
        if mutation=='missing-native-pins':del row['binary_sha256'];del row['harness_sha256']
        if mutation=='stdout-digest':row['stdout_sha256']='bad-digest'
        if mutation=='stdout-length':row['stdout_bytes']+=1
        if mutation=='stdout-interpretation':row['output']=None
        if mutation=='stdout-bool-number':row['output']=dict(status='ok',flag=1)
        if mutation=='stdout-missing':del row['stdout_base64']
        raw=(json.dumps(row)+'\n').encode();compressed=gzip.compress(raw,mtime=0);(lane/'growth-raw.jsonl.gz').write_bytes(compressed)
        (lane/'raw-index.json').write_text(json.dumps([dict(file='growth-raw.jsonl.gz',source='growth-raw.jsonl',records=1,sha256=hashlib.sha256(raw).hexdigest(),compressed_sha256=hashlib.sha256(compressed).hexdigest())]))
        result=subprocess.run([sys.executable,str(root/'verify_evidence.py')],capture_output=True,text=True)
        assert (result.returncode==0)==(mutation=='valid'),(mutation,result.stdout,result.stderr)
        if mutation!='valid':assert ('lossless-stdout-integrity' if mutation in ['stdout-digest','stdout-length'] else 'lossless-stdout-required' if mutation=='stdout-missing' else 'lossless-stdout-interpretation' if mutation in ['stdout-interpretation','stdout-bool-number'] else 'harness-file-set' if mutation=='incomplete-manifest' else 'native-event-binary-association' if mutation=='missing-native-pins' else 'native-event-session-association') in result.stderr
    common=dict(backend='manifest',root='/fixture/growth/1000-A-project',censored=False,exit=0,status='ok',engine_ms=10,external_ms=12,bytes_read=1,bytes_written=1,metadata_read=1,metadata_written=1,peak_memory_bytes=1)
    operations=[dict(common,action='init',delivery_status='published',publication_outcome='accepted'),dict(common,action='publish',request='commit-1-packet-True',packet_requested=True,delivery_status='selection-uncertain',publication_outcome=None)]
    points=[]
    for key in ['d000001','not-present']:
        for i in range(3):points.append(dict(common,action='point',publication='bound-publication',binding_role='current',key=key,status='error' if i==2 else 'ok'))
    (lane/'operations.json').write_text(json.dumps(operations));(lane/'points.json').write_text(json.dumps(points))
    result=subprocess.run([sys.executable,str(root/'tables.py')],capture_output=True,text=True);assert result.returncode==0,result.stderr
    assert '| windows | manifest | 1,000 | A | 10.00 | — | — | — |' in (e/'operation-table.md').read_text(encoding='utf8')
    assert '| windows |' not in (e/'lookup-table.md').read_text(encoding='utf8')
    assert '| windows |' not in (e/'batch-table.md').read_text(encoding='utf8')
    # A completely failed current cohort must not relabel a good retained cohort.
    points=[]
    for key in ['d000001','not-present']:
        for i in range(3):
            points.append(dict(common,action='point',publication='current-failed',binding_role='current',key=key,exit=2))
            points.append(dict(common,action='point',publication='initial-good',binding_role='initial retained',key=key))
    (lane/'points.json').write_text(json.dumps(points))
    result=subprocess.run([sys.executable,str(root/'tables.py')],capture_output=True,text=True);assert result.returncode==0,result.stderr
    rendered=(e/'lookup-table.md').read_text(encoding='utf8')
    assert rendered.count('| windows |')==1 and '| initial retained |' in rendered and '| current |' not in rendered
print('PASS: exact native session association; unknown harness/session refuse; failed points and uncertain publication excluded from completed tables')
