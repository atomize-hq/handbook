"""Keep instrumentation-only patches against each matched pristine source."""
import difflib,hashlib,json,pathlib,shutil,sys
HERE=pathlib.Path(__file__).resolve().parent
plain=pathlib.Path(sys.argv[1]);profile=pathlib.Path(sys.argv[2]);variant=sys.argv[3]
meta=json.loads((profile/'research-instrumentation.json').read_text())
patch=[]
paths=['crates/engine/src/'+x['file'] for x in meta['sources']]+['crates/engine/src/lib.rs','crates/engine/src/research_profile.rs']
for path in paths:
    left_bytes=(plain/path).read_bytes() if (plain/path).exists() else b''
    right_bytes=(profile/path).read_bytes()
    left=left_bytes.decode('utf-8');right=right_bytes.decode('utf-8')
    patch.extend(difflib.unified_diff(left.splitlines(keepends=True),right.splitlines(keepends=True),fromfile='a/'+path,tofile='b/'+path))
    meta.setdefault('exact_matched_sources',{})[path]={'pristine_sha256':hashlib.sha256(left_bytes).hexdigest(),'instrumented_sha256':hashlib.sha256(right_bytes).hexdigest()}
for source in meta['sources']:
    matched=meta['exact_matched_sources']['crates/engine/src/'+source['file']]
    source['before_sha256']=matched['pristine_sha256'];source['after_sha256']=matched['instrumented_sha256']
(HERE/f'instrumentation-{variant}.patch').write_text(''.join(patch),encoding='utf-8')
(HERE/f'instrumentation-{variant}.json').write_text(json.dumps(meta,indent=2)+'\n',encoding='utf-8')
if (profile/'research-source-snapshot.json').exists():shutil.copyfile(profile/'research-source-snapshot.json',HERE/f'source-snapshot-{variant}.json')
print(json.dumps({'variant':variant,'instrumented_files':len(paths)}))
