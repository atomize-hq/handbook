"""Bounded, verified archive of two exact NEW artifact targets; never deletes.

Execution still needs stage admission. No dataset, old Q/N path or Cargo cache is
eligible. The archive preserves all target bytes, including earlier build and
cache-fixture prefixes; successful later qualification does not erase censors.
"""
import argparse,gzip,hashlib,json,os,struct,time
from pathlib import Path,PurePosixPath

def archive_artifacts(output):
    runtime=Path('C:/file-native-packing-20261020');assert runtime.resolve()==runtime
    work=Path(__file__).resolve().parents[3]
    targets=[runtime/'build',work/'docs/research/file-native-incremental-engine-experiment-2026-10-18/target']
    assert str(work).replace('\\','/')=='C:/Users/spmcc/.codex/worktrees/file-native-packing-20261020/handbook'
    assert all(p.resolve()==p and not p.is_symlink() for p in targets)
    for root in targets:
        for component in [root,*root.parents]:
            assert not (os.lstat(component).st_file_attributes&0x400),'reparse-or-junction-ancestor'
    output=Path(output).resolve();assert output.is_relative_to(runtime) and not any(output.is_relative_to(p) for p in targets)
    from guard import current
    admission=current();admission.update(archive_cap_bytes=64*1024**2,retained_executable_forecast=16*1024**2,inventory_return_margin=1024**2)
    assert admission['total_new_bytes']+81*1024**2<=2684354560 and admission['total_new_files']+16<=560000 and admission['free_bytes']>=140*1024**3
    output.mkdir(parents=True,exist_ok=False);(output/'fresh-admission.json').write_text(json.dumps(admission,indent=2));start=time.monotonic();rows=[];total=0
    # These executable paths remain usable after any separately reviewed
    # reclamation. Candidate1 and all historical binaries lie outside targets.
    keep={str(targets[0]/'release/qualified_lookup.exe'),str(targets[0]/'release/packing-sample.exe')}
    with (output/'artifacts.bin.gz').open('xb') as physical,gzip.GzipFile(fileobj=physical,mode='wb',compresslevel=1,mtime=0) as archive:
        archive.write(b'FNAR1\n')
        for index,root in enumerate(targets):
            for directory,dirs,files in os.walk(root,followlinks=False):
                dirs.sort();files.sort()
                assert not (os.lstat(directory).st_file_attributes&0x400),'reparse-directory'
                for name in dirs:assert not (os.lstat(Path(directory)/name).st_file_attributes&0x400),'reparse-or-junction-child'
                for name in files:
                    path=Path(directory)/name;assert not (os.lstat(path).st_file_attributes&0x400) and not path.is_symlink() and path.resolve().is_relative_to(root)
                    relative=path.relative_to(root).as_posix();key=f'{index}/{relative}';encoded=key.encode();assert len(encoded)<=4096
                    with path.open('rb') as source:
                        size=os.fstat(source.fileno()).st_size;total+=size;assert size<=512*1024**2 and total<=400*1024**2 and len(rows)<2000
                        archive.write(struct.pack('<IQ',len(encoded),size));archive.write(encoded);remaining=size;digest=hashlib.sha256()
                        while remaining:
                            block=source.read(min(65536,remaining));assert block;remaining-=len(block);digest.update(block);archive.write(block)
                            # One <=64 KiB compressor input plus final framing
                            # fits within the 1 MiB margin below the hard cap.
                            assert physical.tell()<=63*1024**2,'archive-soft-cap-prefix-retained'
                        assert not source.read(1) and path.stat().st_size==size
                    archive.write(digest.digest());rows.append(dict(path=key,absolute=str(path),bytes=size,sha256=digest.hexdigest(),retain_in_place=str(path) in keep))
    transport=output/'artifacts.bin.gz';assert transport.stat().st_size<=64*1024**2
    raw_inventory=''.join(json.dumps(row,separators=(',',':'))+'\n' for row in rows).encode();(output/'files.jsonl').write_bytes(raw_inventory)
    digest=hashlib.sha256()
    with transport.open('rb') as f:
        for block in iter(lambda:f.read(65536),b''):digest.update(block)
    # Independent transport traversal uses only the complete retained inventory,
    # verifies every raw byte, and creates no restored duplicate target.
    with gzip.open(transport,'rb') as archive:
        assert archive.read(6)==b'FNAR1\n'
        for row in rows:
            header=archive.read(12);assert len(header)==12;n,size=struct.unpack('<IQ',header);assert n<=4096 and size==row['bytes']
            key=archive.read(n).decode();p=PurePosixPath(key);assert key==row['path'] and not p.is_absolute() and '..' not in p.parts and '\\' not in key and ':' not in key
            remaining=size;raw=hashlib.sha256()
            while remaining:
                block=archive.read(min(65536,remaining));assert block;remaining-=len(block);raw.update(block)
            assert archive.read(32)==raw.digest() and raw.hexdigest()==row['sha256']
        assert not archive.read(1)
    assert keep=={row['absolute'] for row in rows if row['retain_in_place']}
    kept=sum(row['bytes'] for row in rows if row['retain_in_place']);assert kept<=16*1024**2
    result=dict(profile='new-build-and-cache-fixture-byte-archive',status='complete-round-trip-verified-no-deletion',targets=[str(p) for p in targets],files=len(rows),logical_bytes=total,inventory_sha256=hashlib.sha256(raw_inventory).hexdigest(),transport_sha256=digest.hexdigest(),transport_bytes=transport.stat().st_size,retained_executable_bytes=kept,all_target_bytes_preserved=True,previous_failures_and_censors_remain_separate=True,delete_authority=False,seconds=time.monotonic()-start)
    (output/'manifest.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2));return result

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--output',required=True);args=parser.parse_args();archive_artifacts(args.output)
