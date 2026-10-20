"""Admitted XZ preservation of exact NEW targets; no deletion operation."""
import argparse,hashlib,json,lzma,os,struct,time
from pathlib import Path,PurePosixPath
from guard import current

class CappedOutput:
    def __init__(self,file,cap):self.file=file;self.cap=cap
    def write(self,data):
        if self.file.tell()+len(data)>self.cap:raise ValueError('compressed-cap-before-write-prefix-retained')
        count=self.file.write(data);assert count==len(data);return count
    def flush(self):return self.file.flush()
    def tell(self):return self.file.tell()

def archive_artifacts(output):
    runtime=Path('C:/file-native-packing-20261020');assert runtime.resolve()==runtime
    work=Path(__file__).resolve().parents[3];assert str(work).replace('\\','/')=='C:/Users/spmcc/.codex/worktrees/file-native-packing-20261020/handbook'
    targets=[runtime/'build',work/'docs/research/file-native-incremental-engine-experiment-2026-10-18/target']
    for root in targets:
        assert root.resolve()==root
        for component in [root,*root.parents]:assert not (os.lstat(component).st_file_attributes&0x400),'reparse-or-junction-ancestor'
    output=Path(output).resolve();assert output.is_relative_to(runtime) and not any(output.is_relative_to(p) for p in targets)
    admission=current();admission.update(compressed_cap=56*1024**2,inventory_report_margin=2*1024**2,retained_executables='already in current actual bytes; no additional copies',old_failed_gzip_preserved=True)
    assert admission['total_new_bytes']+58*1024**2<=2791728742 and admission['total_new_files']+16<=560000 and admission['free_bytes']>=140*1024**3
    units=json.loads((runtime/'fix2/unit-tests/summary.json').read_bytes());assert units['core_pin']=='d70f3864f2888d46d9112213b4a03f8d82745a8e' and units['meaningful_workspace_tests']==5 and units['status']=='passed'
    output.mkdir(parents=True,exist_ok=False);(output/'fresh-admission.json').write_text(json.dumps(admission,indent=2));(output/'archive-harness.py').write_bytes(Path(__file__).read_bytes());start=time.monotonic();rows=[];total=0
    keep={str(targets[0]/'release/qualified_lookup.exe'),str(targets[0]/'release/packing-sample.exe')};transport=output/'artifacts.bin.xz'
    with transport.open('xb') as physical,lzma.LZMAFile(CappedOutput(physical,56*1024**2),mode='wb',format=lzma.FORMAT_XZ,check=lzma.CHECK_CRC64,preset=6) as archive:
        archive.write(b'FNAR1\n')
        for index,root in enumerate(targets):
            for directory,dirs,files in os.walk(root,followlinks=False):
                dirs.sort();files.sort();assert not (os.lstat(directory).st_file_attributes&0x400),'reparse-directory'
                for name in dirs:assert not (os.lstat(Path(directory)/name).st_file_attributes&0x400),'reparse-or-junction-child'
                for name in files:
                    path=Path(directory)/name;assert not (os.lstat(path).st_file_attributes&0x400) and path.resolve().is_relative_to(root)
                    key=f'{index}/{path.relative_to(root).as_posix()}';encoded=key.encode();assert len(encoded)<=4096
                    with path.open('rb') as source:
                        size=os.fstat(source.fileno()).st_size;total+=size;assert size<=512*1024**2 and total<=512*1024**2 and len(rows)<2000
                        archive.write(struct.pack('<IQ',len(encoded),size));archive.write(encoded);remaining=size;digest=hashlib.sha256()
                        while remaining:
                            block=source.read(min(65536,remaining));assert block;remaining-=len(block);digest.update(block);archive.write(block)
                        assert not source.read(1) and path.stat().st_size==size
                    archive.write(digest.digest());rows.append(dict(path=key,absolute=str(path),bytes=size,sha256=digest.hexdigest(),retain_in_place=str(path) in keep))
    assert transport.stat().st_size<=56*1024**2
    raw_inventory=''.join(json.dumps(row,separators=(',',':'))+'\n' for row in rows).encode();assert len(raw_inventory)<=1024**2;(output/'files.jsonl').write_bytes(raw_inventory)
    digest=hashlib.sha256()
    with transport.open('rb') as f:
        for block in iter(lambda:f.read(65536),b''):digest.update(block)
    with lzma.open(transport,'rb') as archive:
        assert archive.read(6)==b'FNAR1\n'
        for row in rows:
            header=archive.read(12);assert len(header)==12;n,size=struct.unpack('<IQ',header);assert n<=4096 and size==row['bytes']
            key=archive.read(n).decode();p=PurePosixPath(key);assert key==row['path'] and not p.is_absolute() and '..' not in p.parts and '\\' not in key and ':' not in key
            remaining=size;raw=hashlib.sha256()
            while remaining:
                block=archive.read(min(65536,remaining));assert block;remaining-=len(block);raw.update(block)
            assert archive.read(32)==raw.digest() and raw.hexdigest()==row['sha256']
        assert not archive.read(1)
    assert keep=={row['absolute'] for row in rows if row['retain_in_place']};kept=sum(row['bytes'] for row in rows if row['retain_in_place']);assert kept<=16*1024**2
    result=dict(profile='new-build-and-cache-fixture-byte-archive',status='complete-round-trip-verified-no-deletion',transport_codec='stdlib LZMA XZ preset6 CRC64; FNAR1 raw framing unchanged',targets=[str(p) for p in targets],files=len(rows),logical_bytes=total,inventory_sha256=hashlib.sha256(raw_inventory).hexdigest(),transport_sha256=digest.hexdigest(),transport_bytes=transport.stat().st_size,retained_executable_bytes=kept,old_failed_gzip_bytes=(runtime/'artifact-preservation/artifacts.bin.gz').stat().st_size,all_target_bytes_preserved=True,previous_failures_and_censors_remain_separate=True,delete_authority=False,core_pin=units['core_pin'],five_workspace_tests_before_snapshot=True,seconds=time.monotonic()-start)
    (output/'manifest.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2));return result

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--output',required=True);args=parser.parse_args();archive_artifacts(args.output)
