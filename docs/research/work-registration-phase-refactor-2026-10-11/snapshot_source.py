"""Copy only live authorized source deltas into a detached source checkpoint."""
import hashlib,json,pathlib,shutil,subprocess,sys
HERE=pathlib.Path(__file__).resolve().parent;PRIMARY=HERE.parents[2]
target=pathlib.Path(sys.argv[1]).resolve()
if target==PRIMARY or PRIMARY in target.parents:raise ValueError('detached external source required')
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=PRIMARY,text=True).strip()
target_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=target,text=True).strip()
if target_head!=head:raise ValueError('source baseline mismatch')
paths=subprocess.check_output(['git','diff','--name-only','--','crates','Cargo.toml','Cargo.lock'],cwd=PRIMARY,text=True).splitlines()
record={'baseline':head,'primary':str(PRIMARY),'target':str(target),'copies':{}}
for path in paths:
    original=PRIMARY/path;copy=target/path
    if not original.is_file():raise ValueError('unexpected deleted source '+path)
    copy.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(original,copy)
    record['copies'][path]=hashlib.sha256(original.read_bytes()).hexdigest()
(target/'research-source-snapshot.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'target':str(target),'copied_files':len(paths)}))
