"""Count owned logical lengths without following links/junctions."""
import argparse,json,os,time
from pathlib import Path
def census(root):
    root=Path(root).resolve();files=total=links=0;stack=[root];start=time.monotonic()
    while stack:
        with os.scandir(stack.pop()) as entries:
            for e in entries:
                stat=e.stat(follow_symlinks=False)
                if e.is_symlink() or getattr(stat,'st_file_attributes',0)&1024:links+=1
                elif e.is_dir(follow_symlinks=False):stack.append(Path(e.path))
                elif e.is_file(follow_symlinks=False):files+=1;total+=stat.st_size
    return dict(root=str(root),files=files,logical_bytes=total,links_excluded=links,seconds=time.monotonic()-start)
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('roots',nargs='+');a=p.parse_args();rows=[census(r) for r in a.roots];print(json.dumps(dict(roots=rows,files=sum(r['files'] for r in rows),logical_bytes=sum(r['logical_bytes'] for r in rows)),indent=2))
