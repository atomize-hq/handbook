"""Native cap/binding adapter; execute unchanged capsule native.py via runpy."""
import hashlib
import json
import os
from pathlib import Path
import runpy
import sys
import time

ROOT=Path(__file__).resolve().parent
ALLOCATION=json.loads((ROOT/'allocation.json').read_bytes())
SOURCE=ROOT/'source/docs/research/file-native-incremental-engine-experiment-2026-10-18'
sys.path.insert(0,str(SOURCE))
import controller

PinnedDriver=controller.Driver
class BoundedDriver(PinnedDriver):
    def __init__(self,exe,base,seconds=14400):
        super().__init__(exe,base,seconds)
        self.end=min(self.end,time.monotonic()+max(0,ALLOCATION['launch_deadline_epoch']-30-time.time()))
        binding=dict(pin=ALLOCATION['pin'],adapter_sha256=controller.sha(Path(__file__).read_bytes()),
            allocation_sha256=controller.sha((ROOT/'allocation.json').read_bytes()),
            executable_sha256=self.session['sha256'],
            source_archive_sha256=self.session['source_archive_sha256'],
            cap_seconds=600 if self.base.name=='smoke' else 180,
            scope='Caps tightened, source and assertions unchanged; outer root watchdog active')
        (self.base/'outer-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
    def guard(self):
        if (ROOT/'stop.json').exists():raise RuntimeError('outer-stop-requested')
        if time.time()>=ALLOCATION['launch_deadline_epoch']-30:raise TimeoutError('native-report-reserve-cutoff')
        return super().guard()
    def call(self,v,cap=180,env=None,allow=False):
        maximum=600 if self.base.name=='smoke' else 180
        return super().call(v,min(cap,maximum),env,allow)

controller.Driver=BoundedDriver
sys.argv=[str(SOURCE/'native.py'),*sys.argv[1:]]
runpy.run_path(sys.argv[0],run_name='__main__')
