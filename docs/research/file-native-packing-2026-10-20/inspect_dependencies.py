import hashlib,json,urllib.request
from pathlib import Path
out=Path('C:/file-native-packing-20261020/donors');out.mkdir(exist_ok=True)
urls={
 'dolt-table.go':'https://raw.githubusercontent.com/dolthub/dolt/48829716caa01d5c958b328cc6a93cb5ff1e50ce/go/store/nbs/table.go',
 'dolt-table-index.go':'https://raw.githubusercontent.com/dolthub/dolt/48829716caa01d5c958b328cc6a93cb5ff1e50ce/go/store/nbs/table_index.go',
 'dolt-file-manifest.go':'https://raw.githubusercontent.com/dolthub/dolt/48829716caa01d5c958b328cc6a93cb5ff1e50ce/go/store/nbs/file_manifest.go',
 'dolt-read-drain.diff':'https://github.com/dolthub/dolt/commit/06b4d9bbbf783f102d820f0880b46454b31a1c32.diff',
 'dolt-relocation.diff':'https://github.com/dolthub/dolt/commit/c5dab4af005bfdf36c68f2ea1361f4418c543205.diff',
 'zstd-0.14.3.json':'https://crates.io/api/v1/crates/zstd/0.14.3',
 'zstd-0.14.3.crate':'https://static.crates.io/crates/zstd/zstd-0.14.3.crate'
}
rows=[]
for name,url in urls.items():
    with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'bounded-file-native-research/1'}),timeout=30) as r:b=r.read(2*1024**2+1)
    assert len(b)<=2*1024**2
    (out/name).write_bytes(b);rows.append(dict(name=name,url=url,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()))
assert next(x for x in rows if x['name'].endswith('.crate'))['sha256']=='db3ae9bf90af6e40197bef767eeefc46ad6397420fe873830bd85e5fbbfb3c70'
(out/'manifest.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows,indent=2))
