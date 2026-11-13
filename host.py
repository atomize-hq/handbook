"""Emit selected public-safe hosted environment metadata."""
import json
import os
import platform
import subprocess
from pathlib import Path

def command(args):
    return subprocess.check_output(args, text=True, stderr=subprocess.DEVNULL).strip()

if platform.machine().lower() not in ('amd64', 'x86_64'):
    raise RuntimeError('x64 host required')
info = {'os': platform.platform(), 'machine': platform.machine(),
        'image_os': os.getenv('ImageOS'), 'image_version': os.getenv('ImageVersion')}
if os.name == 'nt':
    query = """$ErrorActionPreference='Stop'; $c=Get-CimInstance Win32_ComputerSystem; $p=Get-CimInstance Win32_Processor | Select-Object -First 1; $o=Get-CimInstance Win32_OperatingSystem; $v=Get-Volume -DriveLetter C; $d=Get-MpComputerStatus; $u=Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System'; @{uac_enabled=$u.EnableLUA; cpu=$p.Name; logical_processors=$c.NumberOfLogicalProcessors; build=$o.BuildNumber; os_version=$o.Version; filesystem=$v.FileSystem; root='C:'; defender=@{antivirus_enabled=$d.AntivirusEnabled; real_time_enabled=$d.RealTimeProtectionEnabled; tamper_protected=$d.IsTamperProtected; engine_version=$d.AMEngineVersion; signatures=$d.AntivirusSignatureVersion}} | ConvertTo-Json -Compress -Depth 4"""
    info.update(json.loads(command(['powershell', '-NoProfile', '-Command', query])))
else:
    cpu = Path('/proc/cpuinfo').read_text()
    info['cpu'] = next((line.split(':',1)[1].strip() for line in cpu.splitlines() if line.startswith('model name')), None)
    info['logical_processors'] = os.cpu_count()
    info['root_filesystem'] = command(['stat', '-f', '-c', '%T', str(Path(os.environ['FIXTURE_ROOT']).parent)])
    info['root'] = 'fixture parent'
    info['mount'] = command(['findmnt', '-n', '-o', 'FSTYPE,OPTIONS', '-T', str(Path(os.environ['FIXTURE_ROOT']).parent)])
    info['effective_uid_zero'] = os.geteuid() == 0
fixture_root = Path(r'C:\fnbench\work\fixtures') if os.name == 'nt' else Path(os.environ['FIXTURE_ROOT'])
project_path = fixture_root / 'attempt-01' / 'work' / 'project'
info['fixture_root_path'] = str(fixture_root)
info['project_path_depth'] = len(project_path.parts)
info['project_path_characters'] = len(str(project_path))
(Path(os.environ['PUBLIC_DIR']) / 'host.json').write_text(json.dumps(info, indent=2) + '\n')
