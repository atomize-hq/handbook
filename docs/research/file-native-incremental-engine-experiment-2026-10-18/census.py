"""Post-run actual logical file census; allocated blocks only where exposed."""
import argparse,json,os,shutil,time
from pathlib import Path

def census(root,owners=False):
    root=Path(root);start=time.monotonic();groups={};allocated=True;owner_bytes=0;owner_count=0
    for directory,_,names in os.walk(root):
        for name in names:
            p=Path(directory)/name;relative=p.relative_to(root);category=relative.parts[0] if len(relative.parts)>1 else '(root-files)';s=p.stat();row=groups.setdefault(category,{'files':0,'logical_bytes':0,'allocated_file_bytes':0})
            row['files']+=1;row['logical_bytes']+=s.st_size
            if hasattr(s,'st_blocks'):row['allocated_file_bytes']+=s.st_blocks*512
            else:allocated=False
            if owners and relative.parts[0]=='publications' and p.suffix=='.json':
                value=json.loads(p.read_bytes());owner_bytes+=value['allocation_owner']['conservative_bytes'];owner_count+=1
    if not allocated:
        for row in groups.values():row['allocated_file_bytes']=None
    return {'root':str(root),'epoch':time.time(),'seconds':time.monotonic()-start,'categories':groups,'files':sum(x['files'] for x in groups.values()),'logical_bytes':sum(x['logical_bytes'] for x in groups.values()),'allocated_file_bytes':sum(x['allocated_file_bytes'] for x in groups.values()) if allocated else None,'free_bytes':shutil.disk_usage(root).free,'conservative_publication_owner_bytes':owner_bytes if owners else None,'owner_records':owner_count if owners else None,'qualification':'Actual file-length sum, not content-hash deduplication or filesystem metadata/physical write amplification. Conservative ownership includes intermediates/stages and overcounts shared or collected storage; it is separate from active candidate admission and actual bytes.'}

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--roots',nargs='+',required=True);a.add_argument('--output',required=True);a.add_argument('--owners',action='store_true');ns=a.parse_args();rows=[]
    for root in ns.roots:
        rows.append(census(root,ns.owners));Path(ns.output).write_text(json.dumps(rows,indent=2))
