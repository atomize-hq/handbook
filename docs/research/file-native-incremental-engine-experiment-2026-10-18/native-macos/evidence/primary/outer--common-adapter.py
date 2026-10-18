import runpy,sys,time
from controller import Driver
original_init=Driver.__init__
original_call=Driver.call
def init(self,*args,**kw):
 original_init(self,*args,**kw);self.end=min(self.end,3046876.729177291)
def call(self,v,cap=180,env=None,allow=False):
 return original_call(self,v,min(cap,600 if "smoke" in str(self.base) else 180),env,allow)
Driver.__init__=init
Driver.call=call
runpy.run_path('/private/tmp/file-native-incremental-macos-20261018/source-final/docs/research/file-native-incremental-engine-experiment-2026-10-18/native.py',run_name="__main__")
