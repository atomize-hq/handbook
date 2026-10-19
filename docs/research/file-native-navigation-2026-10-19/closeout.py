"""Read retained receipts, bind source/binaries and package bounded raw evidence.

Never starts the engine, restores a fixture, changes a selector, or deletes files.
"""
import hashlib,json,platform,shutil,subprocess,tarfile,time
from pathlib import Path
from census import census

HERE=Path(__file__).resolve().parent
WORKTREE=HERE.parents[2]
OWNED=Path('C:/file-native-navigation-20261019')
ENGINE=HERE.parent/'file-native-incremental-engine-experiment-2026-10-18'
OUT=HERE/'results'
def sha(path):
    h=hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
    return h.hexdigest()
def write(path,value):path.write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8')
def git(*args):return subprocess.check_output(['git',*args],cwd=WORKTREE,text=True).strip()
def run():
    OUT.mkdir(exist_ok=True)
    sources={p.name:sha(p) for p in sorted((ENGINE/'src').glob('*.rs'))}
    sources.update({name:sha(ENGINE/name) for name in ['Cargo.toml','Cargo.lock']})
    frozen='4bde156323812ef18c0dea6798cfc2108aaad933'
    assert not git('diff',frozen,'--',str(ENGINE/'src'),str(ENGINE/'Cargo.toml'),str(ENGINE/'Cargo.lock'))
    write(OWNED/'admission/final-source-binding.json',dict(core_commit=frozen,src_git_tree=git('rev-parse',frozen+':docs/research/file-native-incremental-engine-experiment-2026-10-18/src'),source_sha256=sources,binary=str(OWNED/'bin/final.exe'),binary_sha256=sha(OWNED/'bin/final.exe'),measurement_core_commit='6e66a7523612b80386cf9cc2b96d5341579b54fd',measurement_exe_sha256=sha(OWNED/'bin/candidate.exe'),difference='final core only omits inherited Markdown guidance at profile4 genesis; ordinary matrix not relabeled'))
    write(OUT/'source-binding.json',json.loads((OWNED/'admission/final-source-binding.json').read_bytes()))
    write(OUT/'harness-binding.json',dict(note='Closeout harness version; late continuation/admission/reporting changes are not relabeled as the originally measured harness. Raw events and original committed native kit remain bound to their runs.',files={p.name:sha(p) for p in sorted(HERE.iterdir()) if p.suffix in ['.py','.ps1']}))
    bprefix=OWNED/'matrix-final/n10000-B-p4/cell/disposable/store'
    write(OWNED/'admission/B10000-preserved-prefix.json',dict(utc=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),census=census(bprefix),selected_exists=(bprefix/'SELECTED.json').exists(),reservation=json.loads((bprefix/'RESERVATIONS.json').read_bytes()),qualification='Unselected original censored prefix retained in place; no restart or control reconstruction. Conservative reservation is not actual physical storage.'))
    rows=[];events=[];checks=[]
    for base in sorted((OWNED/'matrix-final').iterdir()):
        if not base.is_dir():continue
        p=base/'measurements.json'
        if p.exists():
            for r in json.loads(p.read_bytes()):r['_cell']=base.name;rows.append(r)
        for name in ['events.json','initialization-censor-events.json']:
            p=base/name
            if p.exists():
                for e in json.loads(p.read_bytes()):e['_cell']=base.name;e['_file']=name;events.append(e)
    primary=[r for r in rows if not (r.get('scope') and 'deferred' in r['scope'])]
    for r in primary:
        assert all(r['oracle'].get(k) is True for k in ['immutable_discovery_semantics_verified','discovery_relationships_and_extraction'])
        assert r['oracle']['readable_declared_current'] is True
    unique_events={json.dumps({k:v for k,v in e.items() if not k.startswith('_')},sort_keys=True):e for e in events}
    sampled=[e['sampled_peak_rss'] for e in unique_events.values() if e.get('sampled_peak_rss') is not None]
    measured_owned=[dict(cell=r['_cell'],case=r['case'],bytes=r['prepared_owned_census']['logical_bytes']+r['prepared_worktree_census']['logical_bytes'],files=r['prepared_owned_census']['files']+r['prepared_worktree_census']['files']) for r in rows if r.get('prepared_owned_census') and r.get('prepared_worktree_census')]
    write(OUT/'qualification-summary.json',dict(utc=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),primary_rows=len(primary),extra_deferred_rows=len(rows)-len(primary),all_primary_oracle_flags=True,matrix_child_receipts=len(events),matrix_distinct_child_receipts=len(unique_events),matrix_children_with_rss_samples=len(sampled),matrix_sampled_peak_rss=max(sampled),checker_peak_rss=max(r['checker_peak_rss'] for r in rows),prepared_owned_samples=len(measured_owned),prepared_owned_max_bytes=max(measured_owned,key=lambda r:r['bytes']),prepared_owned_max_files=max(measured_owned,key=lambda r:r['files']),historical_owned_peak='not continuously measured; admission forecasts and available boundary/prepare censuses retained',B10000=dict(status='initialization-censored-before-selection',ordinary_rows=0,profiles2and3='unperformed'),fixture_retention=dict(profile2_full=(OWNED/'matrix-final/n10000-A-p2-full/cell/disposable/store').exists(),profile3_full=(OWNED/'matrix-final/n10000-A-p3-full/cell/disposable/store').exists())))
    # Only small receipt files at explicit levels; never follow store trees.
    files=set(p for p in (OWNED/'admission').iterdir() if p.is_file() and p.suffix in ['.json','.txt'])
    for name in ['stage1-n128','focused-local-final','focused-local-final/recovery-capsule','genesis-final-4bde','ordinary-proof','matrix-final']:
        base=OWNED/name
        files.update(p for p in base.iterdir() if p.is_file() and p.suffix in ['.json','.txt'])
    for base in (OWNED/'matrix-final').iterdir():
        if base.is_dir():files.update(p for p in base.iterdir() if p.is_file() and p.suffix in ['.json','.txt'])
    files.update([OWNED/'initialization-censor-capture/manifest.json',OWNED/'initialization-censor-capture/files.jsonl',OWNED/'n10k-source-corpus.tar.manifest.json',OWNED/'focused-first-preserved.tar.json'])
    items=[dict(path=p.relative_to(OWNED).as_posix(),bytes=p.stat().st_size,sha256=sha(p)) for p in sorted(files)]
    archive=OUT/'raw-receipts.tar.gz'
    with tarfile.open(archive,'w:gz',compresslevel=6) as tar:
        for p in sorted(files):tar.add(p,arcname=p.relative_to(OWNED).as_posix(),recursive=False)
    with tarfile.open(archive,'r:gz') as tar:
        assert len(tar.getmembers())==len(items)
        for item in items:assert hashlib.sha256(tar.extractfile(item['path']).read()).hexdigest()==item['sha256']
    write(OUT/'raw-receipts-index.json',dict(archive=archive.name,bytes=archive.stat().st_size,sha256=sha(archive),files=items,not_embedded=[dict(path=str(p),bytes=p.stat().st_size,sha256=sha(p)) for p in [OWNED/'initialization-censor-capture/operational.bin.gz',OWNED/'n10k-source-corpus.tar.gz',OWNED/'focused-first-preserved.tar.gz']]))
    roots=[census(OWNED),census(WORKTREE)]
    write(OUT/'resource-closeout.json',dict(utc=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),roots=roots,files=sum(r['files'] for r in roots),logical_bytes=sum(r['logical_bytes'] for r in roots),free_bytes=shutil.disk_usage(OWNED).free,owned_ceiling_bytes=1536*1024**2,owned_ceiling_files=150000,physical_allocation_bytes=None,cache_residue_separately_in_global_baseline=dict(logical_bytes=1050883003,files=15042),note='Logical regular file lengths exclude reparse points. Global and other-lane accounting belongs to coordinator. No continuous aggregate RSS or historical disk peak claim. Worktree census includes evidence archive.'))
    print(json.dumps(dict(receipt_files=len(items),receipt_archive_bytes=archive.stat().st_size,primary_rows=len(primary),extra_deferred_rows=len(rows)-len(primary),owned_bytes=sum(r['logical_bytes'] for r in roots),owned_files=sum(r['files'] for r in roots))))
if __name__=='__main__':run()
