"""Pause/resume only the recorded disposable native fixture workers on Windows."""
import ctypes,json,pathlib,sys
here=pathlib.Path(__file__).resolve().parent
ids=json.loads((here/'population-pause-pids.json').read_text(encoding='utf-8-sig'))
if isinstance(ids,int):ids=[ids]
kernel=ctypes.WinDLL('kernel32');kernel.OpenProcess.restype=ctypes.c_void_p
nt=ctypes.WinDLL('ntdll')
for pid in ids:
    h=kernel.OpenProcess(0x0800,False,pid)
    if not h:raise RuntimeError(f'Cannot open managed worker {pid}')
    f=nt.NtSuspendProcess if sys.argv[1]=='pause' else nt.NtResumeProcess
    f.argtypes=[ctypes.c_void_p];status=f(h)
    kernel.CloseHandle.argtypes=[ctypes.c_void_p];kernel.CloseHandle(h)
    if status:raise RuntimeError(f'Native worker action failed {pid}: {status}')
print(json.dumps({'action':sys.argv[1],'pids':ids}))
