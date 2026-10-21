import gzip
import copy
import os
from pathlib import Path
import stat
import threading
from windows_deadlines import canonical, digest, integer, need, strict


def confined(base, path):
    base, path = Path(base), Path(path)
    need(base.is_absolute() and path.is_absolute() and ".." not in path.parts
         and path.is_relative_to(base), "absolute-root-containment")
    current = Path(path.anchor)
    for part in path.parts[1:]:
        current /= part
        try:
            s = current.lstat()
        except FileNotFoundError:
            continue
        need(not stat.S_ISLNK(s.st_mode) and not(getattr(s, "st_file_attributes", 0) & 1024),
             "reparse-path-component:"+str(current))
        if stat.S_ISREG(s.st_mode):
            need(s.st_nlink == 1, "hardlinked-file:"+str(current))
    return path


def regular_read(path, limit):
    path = Path(path)
    confined(Path(path.anchor), path)
    need(integer(limit), "integer-read-limit")
    with path.open("rb") as stream:
        before = os.fstat(stream.fileno())
        need(stat.S_ISREG(before.st_mode) and before.st_nlink == 1
             and before.st_size <= limit, "bounded-regular-read")
        raw = stream.read(limit+1)
        after = os.fstat(stream.fileno())
    need(len(raw) <= limit and (before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns)
         == (after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns), "read-changed")
    confined(Path(path.anchor), path)
    return raw


def file_hash(path, limit=None, poll=None):
    path = Path(path)
    confined(Path(path.anchor), path)
    import hashlib
    value = hashlib.sha256()
    with path.open("rb") as stream:
        before = os.fstat(stream.fileno())
        need(stat.S_ISREG(before.st_mode) and before.st_nlink == 1
             and (limit is None or before.st_size <= limit), "hash-file-bound")
        total = 0
        while True:
            if poll:
                poll()
            block = stream.read(65536)
            if not block:
                break
            total += len(block)
            need(limit is None or total <= limit, "hash-growth-bound")
            value.update(block)
        after = os.fstat(stream.fileno())
    need((before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns)
         == (after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns), "hash-changed")
    return value.hexdigest()


def inventory(root, poll=None, absent=False):
    root = Path(root)
    confined(Path(root.anchor), root)
    if not root.exists():
        need(absent, "missing-inventory-root")
        return dict(files={}, directories=[], bytes=0, file_count=0, directory_count=0)
    files, directories = {}, []
    for folder, dirs, names in os.walk(root, followlinks=False):
        if poll:
            poll()
        for name in dirs+names:
            confined(root, Path(folder)/name)
        for name in sorted(dirs):
            directories.append((Path(folder)/name).relative_to(root).as_posix())
        for name in sorted(names):
            path = Path(folder)/name
            s = path.stat()
            files[path.relative_to(root).as_posix()] = dict(bytes=s.st_size,
                sha256=file_hash(path, s.st_size, poll), native_allocated_bytes=None)
    return dict(files=files, directories=sorted(directories),
                bytes=sum(r["bytes"] for r in files.values()), file_count=len(files),
                directory_count=len(directories))


class GrowthLedger:
    def __init__(self, roots, poll):
        self.roots = copy.deepcopy(roots)
        self.poll = poll
        self.lock = threading.RLock()
        self.anchors, self.leases = {}, {}
        paths = [Path(x["root"]) for x in roots.values()]
        need(all(p.is_absolute() for p in paths), "absolute-budget-roots")
        need(all(not a.is_relative_to(b) and not b.is_relative_to(a)
                 for i,a in enumerate(paths) for b in paths[i+1:]), "nonoverlapping-budget-roots")
        for name, spec in self.roots.items():
            need(set(spec) >= {"root","bytes","files","directories"}
                 and all(integer(spec[k]) for k in ("bytes","files","directories")), "closed-root-cap")
            self.anchors[name] = inventory(spec["root"], poll, absent=True)
        self.check()

    def root_name(self, root):
        root = Path(root)
        found = [n for n,s in self.roots.items() if root.is_relative_to(Path(s["root"]))]
        need(len(found) == 1, "exact-budget-category")
        confined(Path(self.roots[found[0]]["root"]), root)
        return found[0]

    def check(self):
        for name,spec in self.roots.items():
            anchor = self.anchors[name]
            pending = [r for lease in self.leases.values() for n,r in lease["growth"].items() if n == name]
            for field,base in (("bytes",anchor["bytes"]),("files",anchor["file_count"]),
                               ("directories",anchor["directory_count"])):
                need(base+sum(r[field] for r in pending) <= spec[field],
                     "prospective-root-cap:"+name+":"+field)

    def reserve(self, token, growth):
        with self.lock:
            self.poll()
            need(token not in self.leases and type(growth) is dict, "unique-growth-lease")
            for name,row in growth.items():
                need(name in self.roots and set(row) == {"bytes","files","directories"}
                     and all(integer(v) for v in row.values()), "incremental-growth-shape")
            self.leases[token] = dict(growth=copy.deepcopy(growth), settled=False)
            try:
                self.check()
            except BaseException:
                del self.leases[token]
                raise
            return copy.deepcopy(self.leases[token])

    def settle(self, token, actual=None, proof=None):
        with self.lock:
            need(token in self.leases,"known-growth-lease")
            lease=self.leases[token]
            if actual is not None:
                need(type(proof) is dict and proof.get("schema")=="closed-writer-settlement-v1"
                     and all(proof.get(k) is True for k in
                         ("process_reaped","io_closed","callbacks_joined","raw_writer_released")),
                     "positive-writer-closure-before-unwritten-release")
                need(all(type(proof.get(k)) is str and len(proof[k])==64 for k in
                    ("source_pin_sha256","profile_sha256","budget_sha256","whole_phase_forecast_sha256")),
                     "source-budget-profile-whole-phase-settlement-bindings")
                need(set(actual)==set(lease["growth"]),"same-settlement-categories")
                for n,row in actual.items():
                    need(set(row)=={"bytes","files","directories"} and all(integer(v) for v in row.values()),
                         "actual-retained-settlement-shape")
                    need(all(row[k]<=lease["growth"][n][k] for k in row),"actual-within-prereserved-potential")
                lease["original_upper"]=copy.deepcopy(lease["growth"])
                lease["growth"]=copy.deepcopy(actual)
                lease["closure"]=copy.deepcopy(proof)
                lease["released_unwritten_only"]=True
            lease["settled"]=True
            self.check()

    def checkpoint(self, label, reserves, *, quiescent):
        with self.lock:
            need(quiescent, "quiescent-census-no-pending-writers")
            measured = {n:inventory(s["root"], self.poll, absent=True) for n,s in self.roots.items()}
            for name,row in measured.items():
                a=self.anchors[name]
                pending=[r for x in self.leases.values() for n,r in x["growth"].items() if n==name]
                for field,key in (("bytes","bytes"),("files","file_count"),("directories","directory_count")):
                    need(row[key]<=a[key]+sum(r[field] for r in pending),
                         "observed-unforecast-growth:"+name+":"+field)
            marker="forecast-only:"+label
            if reserves:
                self.reserve(marker,reserves)
                del self.leases[marker]  # Never spent: shared outer allocation remains held.
            self.check()
            return dict(label=label, actual=copy.deepcopy(measured),
                        prospective=copy.deepcopy(self.leases), planned_reclaim_credit=0)


def per_call_envelope(parts):
    fields={"input_bytes","env_bytes","stdout_bytes","stderr_bytes","rss_samples",
            "harness_bytes","lane_record_bytes","oracle_metadata_bytes",
            "outer_guard_bytes","finalization_bytes"}
    need(type(parts) is dict and set(parts)==fields
         and all(integer(value) for value in parts.values()),"closed-all-sink-call-components")
    i,e,s,t,r=(parts[k] for k in ("input_bytes","env_bytes","stdout_bytes","stderr_bytes","rss_samples"))
    need(i<=4*1024**2 and e<=8192 and s+t<=16*1024**2,"source-call-input-output-ceilings")
    u=s+t+131072
    n=(s+65536+16383)//16384+(t+65536+16383)//16384
    native=4*((u+2)//3)+1024*n+1024*r+6*u+2*i+2*e+98304
    other=sum(parts[k] for k in fields-{"input_bytes","env_bytes","stdout_bytes","stderr_bytes","rss_samples"})
    return dict(native_bytes=native,total_bytes=native+other,pipe_tail_bytes=131072,
                raw_chunk_records=n,components=dict(parts),gzip_credit=0,
                qualification="prospective-source-envelope-not-measured-or-runtime-GO")


def native_call_bound(profile,payload,env):
    if profile.get("api_version")==2:
        return transport_envelope_v2(profile,payload,env)
    parts=profile["all_sink_components"]
    need(parts["input_bytes"]>=len(canonical(payload)) and parts["env_bytes"]>=len(canonical(env))
         and parts["stdout_bytes"]>=profile["stdout_bytes"]
         and parts["stderr_bytes"]>=profile["stderr_bytes"]
         and parts["rss_samples"]>=profile["rss_samples"],"honest-per-call-component-bounds")
    return per_call_envelope(parts)



def template_globals(host,extra):
    spec=host.grant["case_template_sources"]["packing"]
    host.case_templates.load(spec,extra)
    namespace=host.case_templates.loaded[spec["sha256"]]
    result=dict(extra)
    for name,value in namespace.items():
        code=getattr(value,"__code__",None)
        if code is not None and code.co_filename==spec["path"]:
            result[name]=value
    return result

def gzip_copy(source, destination, ledger, category, max_bytes, poll):
    """Retain source; reserve worst case gzip+trailer BEFORE opening output."""
    source, destination = Path(source), Path(destination)
    confined(Path(ledger.roots[category]["root"]), destination)
    need(destination.parent.is_dir() and not destination.exists(), "fresh-gzip-output")
    size = source.stat().st_size
    need(max_bytes >= size+(size//16384+1)*32+4096, "gzip-incompressible-and-footer-bound")
    token = "gzip:"+str(destination)
    ledger.reserve(token, {category:dict(bytes=max_bytes, files=1, directories=0)})
    with source.open("rb") as inp, destination.open("xb") as raw:
        with gzip.GzipFile(filename="", fileobj=raw, mode="wb", compresslevel=1, mtime=0) as out:
            while True:
                poll()
                block = inp.read(65536)
                if not block:
                    break
                out.write(block)
                need(raw.tell() <= max_bytes-4096, "gzip-runtime-bound")
        raw.flush()
    need(destination.stat().st_size <= max_bytes, "closed-gzip-final-bound")
    ledger.settle(token)
    # Caller must checkpoint after its final JSON as well; this is not that census.
    return dict(source_sha256=file_hash(source), gzip_sha256=file_hash(destination),
                bytes=destination.stat().st_size, source_retained=True)


def profile_reservation_v2(p):
    parts=p["all_sink_components"]
    fields={"v","input_frames","stdout_frames","stderr_frames","rss_frames","call_header_footer",
        "reference_rows","harness_projection","helper_records","oracle_metadata","outer_guard","finalization"}
    need(p.get("api_version")==2 and p.get("capture_schema")=="single-raw-ref-v1"
         and p.get("chunk_bytes")==16384 and set(parts)==fields and parts["v"]==2
         and all(integer(v) for v in parts.values()),"authenticated-typed-v2-profile")
    need(type(p.get("profile_sha256")) is str and len(p["profile_sha256"])==64
         and integer(p["failure_tail_bytes"]),"source-profile-digest-and-failure-escrow")
    raw=sum(parts[k] for k in ("input_frames","stdout_frames","stderr_frames","rss_frames",
        "call_header_footer","harness_projection","reference_rows"))
    return raw+p["failure_tail_bytes"]+max(0,16384-p["end_max_bytes"])

def transport_envelope_v2(profile,payload,env):
    maximum=profile_reservation_v2(profile)
    n=len(canonical(payload));need(n<=4*1024**2 and len(canonical(env))<=8192,"actual-input-env-ceilings")
    limit=2**64-1
    overhead=len(canonical(dict(v=2,kind="data",call="f"*32,seq=limit,stream="err",
        offset=limit,n=limit,sha="f"*64,b64="")))+1
    def frames(size):
        full,tail=divmod(size,16384)
        return full*(overhead+4*((16384+2)//3))+(overhead+4*((tail+2)//3) if tail else 0)
    parts=profile["all_sink_components"]
    need(parts["input_frames"]>=frames(n)
         and parts["stdout_frames"]>=frames(profile["stdout_bytes"])
         and parts["stderr_frames"]>=frames(profile["stderr_bytes"]),"coalesced-actual-source-frame-bounds")
    return dict(native_bytes=maximum,total_bytes=sum(v for k,v in parts.items() if k!="v")+profile["failure_tail_bytes"],
        active_failure_escrow=profile["failure_tail_bytes"],coalescing_memory_bytes=3*16384,
        source_derived=True,actual_byte_reclaim=0,gzip_credit=0)
def abort_pending(self,token,payload,env,binary_sha,source_pin,error):
    c=self.calls[token]
    if c["ended"]:return self.call_reference(token)
    if c["encoded"] is None:self._intent(token,payload,canonical(payload),env)
    row=dict(input=payload,env=env,call_token=token,exit=None,stdout="",stderr="",
        stdout_bytes=0,stderr_bytes=0,input_bytes_sent=0,input_written_prefix_sha256=digest(b""),
        stdout_sha256=digest(b""),stderr_sha256=digest(b""),input_fully_delivered=False,
        stdout_eof=True,stderr_eof=True,memory_sample_count=0,observed_peak_rss=0,
        first_write_qpc=None,resources_closed=True,python_callback_joined=True,
        intentional_interrupt=None,violations=["prelaunch:"+type(error).__name__],
        executable_sha256=binary_sha,source_pin=source_pin,native_identity=None,
        positive_start_event=None,no_child_started=True,popen_reaped=False,
        process_started_ns=time.perf_counter_ns(),process_done_ns=time.perf_counter_ns(),spawn_total_ns=0)
    self.append(dict(kind="native-call",**row));return row

