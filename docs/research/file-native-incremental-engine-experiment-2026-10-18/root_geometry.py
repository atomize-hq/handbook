"""Separate current semantic/history/journal tree footprints from whole-store bytes."""
import argparse,json,time
from pathlib import Path
from verify_store import Tree,checked

class CountedTree(Tree):
    def __init__(self,root):super().__init__(root);self.object_sizes={}
    def obj(self,h):
        raw=super().obj(h);self.object_sizes[h]=len(raw);return raw

def geometry(root):
    root=Path(root);start=time.monotonic();selected=json.loads((root/'SELECTED.json').read_bytes())
    pub=json.loads(checked(root/f"publications/{selected['publication']}.json",selected['digest']));roles={};union={}
    for role,h in [('graph-and-constraints',pub['graph']['tree']),('projection',pub['projection']),('membership',pub['membership']),('outcomes',pub['outcomes']),('change-journal',pub['changes'])]:
        t=CountedTree(root);entries=t.entries(h);union.update(t.object_sizes)
        roles[role]=dict(root=h,entries=len(entries),stats=t.stats(),reachable_objects=len(t.object_sizes),reachable_object_file_bytes=sum(t.object_sizes.values()))
    return dict(root=str(root),selected=selected,selected_publications=pub['sequence']+1,roles=roles,union_reachable_objects=len(union),union_reachable_object_file_bytes=sum(union.values()),seconds=time.monotonic()-start,qualification='Current selected roots only, including indirect values; role totals may share objects. Excludes older COW versions/intermediates, source/derived blobs, publication/result files and filesystem overhead. Whole-root actual census is separate.')

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--roots',nargs='+',required=True);a.add_argument('--output',required=True);ns=a.parse_args();rows=[]
    for root in ns.roots:rows.append(geometry(root));Path(ns.output).write_text(json.dumps(rows,indent=2))
