"""Tool-free shell extractions and exact fixture/closure checks. No engine API."""
import hashlib
import json
import pathlib
import subprocess

HERE=pathlib.Path(__file__).resolve().parent
runtime=pathlib.Path(json.loads((HERE/'allocation.json').read_bytes())['runtime'])
data=runtime/'data/v1'
source=HERE.parent/'file-native-engine-followup-2026-10-17'

def sha(value): return hashlib.sha256(value).hexdigest()

rows=[]
for backend in ['manifest','cow']:
    for mode in ['A','B']:
        root=data/backend/'navigation/navigation'/mode
        assert root.is_dir(),root
        destination=runtime/'terminal-extraction'/backend/mode
        destination.parent.mkdir(parents=True,exist_ok=True)
        label=backend+'-'+mode
        if backend=='manifest' and mode=='A':
            # Capture the real native Bash 3.2 incompatibility before adaptation.
            command=['/bin/bash',str(source/'walkthrough.sh'),str(root),str(destination)]
            p=subprocess.run(command,capture_output=True,timeout=120)
            (HERE/'walkthrough-original-bash32.stdout').write_bytes(p.stdout)
            (HERE/'walkthrough-original-bash32.stderr').write_bytes(p.stderr)
            assert p.returncode!=0 and b'mapfile: command not found' in p.stderr
            assert not destination.exists()
            rows.append(dict(case='original-bash32',command=command,exit=p.returncode,
                helper_sha256=sha(pathlib.Path(__file__).read_bytes()),walkthrough_sha256=sha((source/'walkthrough.sh').read_bytes()),
                expected_portability_refusal=True,engine_calls=0))
        command=['/bin/bash',str(HERE/'walkthrough-macos.sh'),str(root),str(destination)]
        p=subprocess.run(command,capture_output=True,timeout=120)
        (HERE/('walkthrough-'+label+'.stdout')).write_bytes(p.stdout)
        (HERE/('walkthrough-'+label+'.stderr')).write_bytes(p.stderr)
        assert p.returncode==0,p.stderr
        assert b'EXTRACTION VERIFIED' in p.stdout
        # Positively bind exactly once. No selected_state/helper/engine import.
        selected=json.loads((root/'SELECTED.json').read_bytes())
        publication_bytes=(root/'publications'/(selected['publication']+'.json')).read_bytes()
        assert sha(publication_bytes)==selected['digest']
        candidate=json.loads(publication_bytes)['candidate']
        manifest_bytes=(destination/'MANIFEST.json').read_bytes()
        assert sha(manifest_bytes)==candidate['manifest_digest']
        manifest=json.loads(manifest_bytes)
        for name,digest in manifest['files'].items():
            path=destination/name
            assert path.resolve().is_relative_to(destination.resolve())
            assert sha(path.read_bytes())==digest
        nav=(destination/'NAVIGATION.md').read_bytes()
        search=subprocess.run(['/usr/bin/grep','-nE','overview|requirement|cites|attachment|d00000',str(destination/'NAVIGATION.md')],capture_output=True)
        assert search.returncode==0
        (HERE/('search-'+label+'.txt')).write_bytes(search.stdout)
        exact=next(destination.glob('docs/*d000002.json')).read_bytes()
        requirement=next(destination.glob('docs/*d000001.yaml')).read_bytes()
        attachment=destination/'docs/item-attachments-d000009/evidence/deep/source.txt'
        assert b'1.00' in exact and b'\r\n' in exact and 'café α'.encode() in exact
        assert b'\r\n' in requirement and attachment.is_file()
        assert b'requires' in nav and b'cites' in nav and b'attachment' in nav
        provenance=json.loads((destination/'packet/provenance.json').read_bytes())
        assembled=b''
        for entry in provenance['inputs']:
            path=(destination/'packet'/entry['path']).resolve()
            assert path.is_relative_to(destination.resolve())
            content=path.read_bytes()
            assert sha(content)==entry['sha256'] and len(content)==entry['bytes']
            assembled+=f"\n\n<!-- source {entry['id']} sha256 {entry['sha256']} -->\n".encode()+content
        assert assembled==(destination/'packet/context.md').read_bytes()
        assert sha(assembled)==provenance['output_sha256']
        # Original B generations use actual independent source files, not hard links.
        independent=None
        if mode=='B':
            generation=root/candidate['closure']
            file=(generation/next(n for n in manifest['files'] if 'd000001.yaml' in n))
            independent=file.stat().st_nlink==1
            assert independent
        rows.append(dict(case='adapted-extraction',backend=backend,mode=mode,command=command,
            helper_sha256=sha(pathlib.Path(__file__).read_bytes()),walkthrough_sha256=sha((HERE/'walkthrough-macos.sh').read_bytes()),
            exit=p.returncode,selected=selected,closure=candidate['closure'],
            manifest_digest=candidate['manifest_digest'],manifest_files=len(manifest['files']),
            copied_path=str(destination),ordered_inputs=len(provenance['inputs']),engine_calls=0,
            source_json_100_crlf_unicode=True,deep_attachment=True,relative_closure=True,
            b_generation_file_link_count_one=independent))
(HERE/'terminal-extraction-observations.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(rows,ensure_ascii=False,indent=2))
