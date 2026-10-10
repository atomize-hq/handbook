"""Check the report's local references and the bounded tracked product scope."""
import hashlib,json,pathlib,re,subprocess,sys,unicodedata
repo=pathlib.Path(sys.argv[1]).resolve(); here=repo/'docs/research/work-state-native-optimization-2026-10-10'
report=repo/'docs/research/work-state-native-optimization-investigation-2026-10-10.md'
text=report.read_text(encoding='utf-8'); assert '_PENDING' not in text, 'unfinished report'
checked=[]
for target in re.findall(r'\]\(([^)]+)\)',text):
    if '://' in target: continue
    filename,_,fragment=target.partition('#'); path=(report.parent/filename).resolve()
    assert path.is_file(), f'missing reference: {target}'
    if fragment:
        headings=re.findall(r'^#{1,6} (.+)$',path.read_text(encoding='utf-8'),re.M)
        slugs={re.sub(r'[^\w\- ]','',h.strip().lower()).replace(' ','-') for h in headings}
        assert fragment in slugs, f'missing heading: {target}'
    checked.append(target)
diff=subprocess.run(['git','diff','8013948ebd3fbef6725f5d72f3d29120f7617009','--','crates','Cargo.lock','Cargo.toml'],
    cwd=repo,capture_output=True,check=True,text=True).stdout
assert not diff, 'primary product differs from experiment baseline'
subprocess.run(['git','apply','--check',str(here/'experiment.patch')],cwd=repo,check=True)
subprocess.run(['git','diff','--check'],cwd=repo,check=True)
for path in here.glob('*.py'): compile(path.read_text(encoding='utf-8'),str(path),'exec')
for path in here.glob('*.json'): json.loads(path.read_text(encoding='utf-8'))
result={'status':'passed','report_local_links':checked,'primary_product_diff':'empty',
        'patch_applies_to_primary_baseline':True,'python_sources_compile':True,'json_artifacts_parse':True,
        'report_sha256_at_check':hashlib.sha256(report.read_bytes()).hexdigest()}
(here/'document-checks.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps(result,indent=2))
