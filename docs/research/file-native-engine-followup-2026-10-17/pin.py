"""Capture exact final source/build/environment pins, without generated payloads."""
import hashlib,json,pathlib,platform,subprocess,time,sys
H=pathlib.Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def output(argv):
    p=subprocess.run(argv,capture_output=True);return dict(command=argv,exit=p.returncode,stdout=p.stdout.decode('utf8',errors='replace'),stderr=p.stderr.decode('utf8',errors='replace'))
sources={p.relative_to(H).as_posix():dict(bytes=p.stat().st_size,sha256=sha(p)) for p in [*sorted((H/'src').glob('*.rs')),H/'Cargo.toml',H/'Cargo.lock']}
windows=pathlib.Path('C:/file-native-followup-20261017-build-final-v4/release/file-native-publication-experiment.exe')
linux='/home/spenser/file-native-followup-20261017/build-final-v4/release/file-native-publication-experiment'
data=dict(epoch=time.time(),platform=platform.platform(),sources=sources,windows_binary=dict(path=str(windows),sha256=sha(windows)),windows_rust=output(['rustc','-Vv']),windows_cargo=output(['cargo','-V']),linux=output(['wsl','-d','Ubuntu-24.04','--','bash','-lc','uname -a; cat /etc/os-release; findmnt -T /home/spenser/file-native-followup-20261017; df -B1 /home; rustc -Vv; cargo -V; sha256sum '+linux+'; cd /home/spenser/file-native-followup-20261017/source-final-v4; sha256sum src/*.rs Cargo.*']),cache='OS/device caches uncontrolled; no device-cold claim')
data['harness']={p.name:dict(bytes=p.stat().st_size,sha256=sha(p)) for p in sorted(H.glob('*.py')) if p.name not in ['pin.py','collect.py','verify_evidence.py']}
data['windows_python']=sys.version
data['wsl_python']=output(['wsl','-d','Ubuntu-24.04','--','python3','--version'])
data['terminal_helpers']={p.name:dict(bytes=p.stat().st_size,sha256=sha(p)) for p in [*H.glob('*.ps1'),*H.glob('*.sh')]}
data['linux_binary']={'path':linux,'sha256':next(line.split()[0] for line in data['linux']['stdout'].splitlines() if line.endswith('  '+linux))}
(H/'evidence/pins.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf8')
print('Windows',data['windows_binary']['sha256']);print(data['linux']['stdout'][-1200:])
