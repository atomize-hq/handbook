"""External ordinary subprocess120s correction, explicitly separate from baseline."""
import pathlib,sys,subprocess
F=pathlib.Path(__file__).parent/'source/docs/research/file-native-recovery-reclamation-2026-10-19';sys.path.insert(0,str(F))
original=subprocess.run
def bounded(*a,**kw):
    kw['timeout']=min(120,kw.get('timeout',120));return original(*a,**kw)
subprocess.run=bounded
from capsule import run
run(pathlib.Path(sys.argv[1]),sys.argv[2])
