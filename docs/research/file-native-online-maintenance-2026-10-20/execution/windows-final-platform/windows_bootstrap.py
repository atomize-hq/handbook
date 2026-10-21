import argparse
import ast
import hashlib
import importlib.abc
import importlib.machinery
import importlib.util
import json
import os
from pathlib import Path
import runpy
import stat
import sys
import time
GUARD_SHA="50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378"
CUTOFF=1792581014
def need(value,message):
    if not value:raise RuntimeError(message)
def read(path,limit):
    path=Path(path)
    need(path.is_absolute() and ".." not in path.parts,"absolute-boot-path")
    part=Path(path.anchor)
    for name in path.parts[1:]:
        part/=name
        s=part.lstat()
        need(not stat.S_ISLNK(s.st_mode) and not(getattr(s,"st_file_attributes",0)&1024),
             "boot-reparse")
    with path.open("rb") as stream:
        a=os.fstat(stream.fileno())
        need(stat.S_ISREG(a.st_mode) and a.st_nlink==1 and a.st_size<=limit,"boot-file-bound")
        raw=stream.read(limit+1);b=os.fstat(stream.fileno())
    need(len(raw)<=limit and (a.st_dev,a.st_ino,a.st_size,a.st_mtime_ns)
         ==(b.st_dev,b.st_ino,b.st_size,b.st_mtime_ns),"boot-read-changed")
    return raw
def strict(raw):
    def pairs(items):
        value={}
        for key,item in items:
            need(key not in value,"boot-duplicate-JSON")
            value[key]=item
        return value
    def invalid(value):
        raise RuntimeError("boot-nonfinite:"+value)
    return json.loads(raw.decode("utf-8"),object_pairs_hook=pairs,parse_constant=invalid)
def sha(raw):
    return hashlib.sha256(raw).hexdigest()
def verify_manifest(path,expected):
    raw=read(path,2*1024**2)
    need(sha(raw)==expected,"parent-manifest-hash")
    m=strict(raw)
    fields={"version","runtime_go","final_pin","guard","source_files","release",
            "roots","engine","hooks","cases","entrypoint","invocations","historical_grants",
 "absolute_deadline","history_release","platform_directory","imports","capture"}
    need(set(m)==fields and m["version"]==1 and type(m["version"]) is int
         and m["runtime_go"] is True,"closed-reviewed-runtime-manifest")
    need(type(m["final_pin"]) is str and len(m["final_pin"])==40
         and all(c in "0123456789abcdef" for c in m["final_pin"]),"final-reviewed-engine-pin")
    g=m["guard"];gs=g["sha256"]
    need(sha(read(g["path"],131072))==gs,"exact-guard-source")
    if gs!=GUARD_SHA:
        need(g["schema_path"] in m["source_files"],"guard-schema-closure")
        schema=next(ast.literal_eval(n.value) for n in ast.parse(read(g["schema_path"],131072)).body
            if isinstance(n,ast.Assign) and any(getattr(t,"id",0)=="GUARD_WIRE_CONTRACT" for t in n.targets))
        need(schema["api"]=="bounded-windows-guard-wire-r3","R3-guard-wire")
    release=m["release"]
    need(set(release)>={"path","sha256","claim_id","reviewed_source_manifest_sha256"},
         "reviewed-release")
    grant=strict(read(release["path"],1048576))
    need(sha(read(release["path"],1048576))==release["sha256"]
         and grant["runtime_go"] is True and grant["final_pin"]==m["final_pin"]
         and grant["claim_id"]==release["claim_id"]
         and grant["source_reviewed"] is True and grant["transport_reviewed"] is True and grant["guard_sha256"]==gs,
         "parent-runtime-release")
    need(type(m["absolute_deadline"]) in (int,float)
         and time.time()<min(m["absolute_deadline"],CUTOFF),"fixed-runtime-deadline")
    paths={}
    for name,row in m["source_files"].items():
        p=Path(name)
        need(p.is_absolute() and p.resolve()==p and os.path.normcase(str(p)) not in paths,
             "canonical-source-entry")
        need(set(row)=={"bytes","sha256"} and type(row["bytes"]) is int and 0<row["bytes"]<=2*1024**2,
             "source-file-bound")
        source=read(p,row["bytes"])
        need(len(source)==row["bytes"] and sha(source)==row["sha256"],"source-closure:"+name)
        paths[os.path.normcase(str(p))]=row
    self_path=os.path.normcase(str(Path(__file__).resolve()))
    need(self_path in paths,"boot-self-binding")
    need(set(m["imports"].values())<=set(m["source_files"]),"module-closure")
    need(m["guard"]["path"] in m["source_files"],"guard-in-closure")
    need(grant["source_manifest_sha256"]==sha(json.dumps(m["source_files"],sort_keys=True,
         separators=(",",":")).encode()),"parent-source-set-review-binding")
    return m,paths,grant
class BoundImports(importlib.abc.MetaPathFinder):
    def __init__(self,paths,modules):
        self.paths,self.modules=paths,modules
        self.standard=[Path(sys.base_prefix)/x for x in ("Lib","DLLs")]
    def find_spec(self,fullname,path=None,target=None):
        if fullname in self.modules:
            p=Path(self.modules[fullname]);key=os.path.normcase(str(p))
            need(key in self.paths and sha(read(p,self.paths[key]["bytes"]))==self.paths[key]["sha256"],
                 "bound-owned-import-bytes")
            return importlib.util.spec_from_file_location(fullname,p)
        spec=importlib.machinery.PathFinder.find_spec(fullname,path)
        if spec and spec.origin and spec.origin not in ("built-in","frozen"):
            p=Path(spec.origin).resolve();key=os.path.normcase(str(p))
            standard=any(p.is_relative_to(base) for base in self.standard) and "site-packages" not in p.parts
            need(standard or key in self.paths,"unbound-transitive-import:"+fullname)
            if key in self.paths:
                need(sha(read(p,self.paths[key]["bytes"]))==self.paths[key]["sha256"],"import-source-changed")
        return spec
def gate(spec,manifest_sha,deadline):
    gate=spec["startup_gate"]
    need(set(gate)>={"ready","release","nonce","root","reserved","max_bytes"}
         and gate["reserved"] is True and gate["max_bytes"]==8192
         ,"prospectively-reserved-owned-startup-gate")
    root=Path(gate["root"])
    ready,release=Path(gate["ready"]),Path(gate["release"])
    need(root.is_absolute() and root.resolve()==root and ready.resolve()==ready
         and release.resolve()==release and ready.is_relative_to(root) and release.is_relative_to(root)
         and ready.parent.is_dir() and not ready.exists() and not release.exists(),
         "fresh-owned-gate-paths")
    data=json.dumps(dict(pid=os.getpid(),nonce=gate["nonce"],manifest_sha256=manifest_sha),
                    separators=(",",":")).encode()
    need(len(data)<=8192,"startup-control-cap")
    with ready.open("xb") as stream:
        stream.write(data);stream.flush()
    while not release.exists():
        need(time.time()<deadline,"startup-gate-total-deadline")
        time.sleep(.005)
    row=strict(read(release,8192))
    need(row==dict(nonce=gate["nonce"],pid=os.getpid()),"owned-startup-release")
    return dict(pid=os.getpid(),nonce=gate["nonce"],gate_sha256=sha(data))
def main():
    p=argparse.ArgumentParser()
    p.add_argument("--manifest",required=True);p.add_argument("--manifest-sha256",required=True)
    p.add_argument("--invocation")
    a=p.parse_args()
    need(os.name=="nt" and sys.flags.isolated and sys.flags.no_site and sys.flags.dont_write_bytecode
         and __debug__,"isolated-unoptimized-Windows-bootstrap")
    m,paths,grant=verify_manifest(a.manifest,a.manifest_sha256)
    spec=m["entrypoint"] if a.invocation is None else m["invocations"][a.invocation]
    need(spec["script_path"] in m["source_files"],"pinned-script-entrypoint")
    gate_receipt=gate(spec,a.manifest_sha256,min(m["absolute_deadline"],CUTOFF))
    sys.meta_path.insert(0,BoundImports(paths,m["imports"]))
    if a.invocation is None:
        from windows_capsule import LaneHost
        host=LaneHost(m,a.manifest_sha256,grant,gate_receipt)
        entry=runpy.run_path(spec["script_path"],run_name="bound_lane_entry")
        try:
            result=entry[spec["function"]](host,spec["config"])
            host.record("entry-result",result,category="evidence",max_bytes=spec["result_max_bytes"])
        finally:
            try:host.drain("entry-finally")
            finally:print(json.dumps(host.finalize(),separators=(",",":")))
    else:
        sys.argv=[spec["script_path"]]+spec["script_argv"]
        runpy.run_path(spec["script_path"],run_name="__main__")
if __name__=="__main__":
    main()
