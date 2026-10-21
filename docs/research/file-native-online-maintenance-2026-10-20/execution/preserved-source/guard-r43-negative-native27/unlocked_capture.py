CAPTURE_WIRE_CONTRACT = {"api":"bounded-unlocked-outer-wire-r3","schema":"bounded-unlocked-outer-qpc-jsonl-r3","version":1,"clock":"checked time.perf_counter_ns nanoseconds","period_ns":20000000,"live_process_limit":16,"live_registered_supervisors":4,"member_table_capacity":2048,"parent_grant_schema":"postexit-finalizer-grant-v2-r3","parent_grant_refs":["authority_ref","profile_catalogue_ref"],"maximum_active_gap_ns":40000000,"rss_row_fields":["event","sample_complete","epoch","perf_counter_ns","completed_perf_counter_ns","interval_seconds","completion_interval_seconds","collection_duration_seconds","job_rss","processes","supervisor_rss","supervisors","suspended","run_nonce"],"process_fields":["pid","creation_time","rss","peak"],"write_complete_fields":["event","sample_complete","sample_id","perf_counter_ns","completed_perf_counter_ns","recorded_perf_counter_ns","duration_seconds","run_nonce"],"row_cap":"budget.evidence_bounds.rss.row_bytes","compression":"bounded gzip-stored-deflate-v1","qualification":"native/QPC health; controls additionally exact27 pass and nestedJob","authority":"fixed roles; no inner heavyTOKEN held","terminal_events":["outer-root-end-coverage","outer-job-tail-coverage"],"native_ssh_profile":"NATIVE_SSH_PROFILE","modes":["controls","guarded-bootstrap","context","native-ssh-phase"],"initial_max_seconds":3600,"controls_max_seconds":900,"cleanup_grace_seconds":12,"deadline_policy":"verified endpoint; tighten only"}
NATIVE_SSH_PROFILE = {'version': 1, 'mode': 'native-ssh-phase', 'ssh_path': 'C:/Windows/System32/OpenSSH/ssh.exe', 'ssh_nlink': 2, 'ssh_bytes': 1251328, 'ssh_sha256': '8e1212d6b10bbab3dfd85077c1a7d0b60373fde702dfa9ea8c57924793bdb317', 'alias': 'spenser-linux-codex', 'flags': ['-T', '-oBatchMode=yes', '-oStrictHostKeyChecking=yes', '-oConnectTimeout=10'], 'remote_python': '/usr/bin/python3', 'remote_python_flags': ['-I', '-S', '-B'], 'native_root': '/home/spenser/file-native-online-linux-20261020', 'bootstrap': '/home/spenser/file-native-online-linux-20261020/source/final-suite-v5/suite_bootstrap.py', 'phase_abi': ['--phase', '--manifest', 'PARAM_PATH', '--sha256', 'PARAM_SHA256', '--phase-deadline', 'FIXED_EPOCH'], 'local_bootstrap': 'native-linux/final-suite/suite_bootstrap.py', 'transport_scope': 'Windows SSH process and inherited local children only'}


"""Bounded Windows outer lane."""
import ast
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path, PurePosixPath
import shlex
import re
import stat
import subprocess
import sys
import threading
import time

API = "bounded-unlocked-capture-v1"
FUNCTIONAL = 1792581014
CLOSEOUT = 1792588214
CONTROL = Path("C:/file-native-online-control-20261020")
LANE = CONTROL / "windows-bounded-outer.lock"
OWNER = CONTROL / "outer-owner.json"
PARENT_ROOT = Path("C:/Users/spmcc/.codex/visualizations/2026/10/20/01a15ab8-d85f-71b1-8d37-c3c5689790be")
_FENCES = []

def need(ok, label):
  if not ok:
    raise RuntimeError(label)

def shape(value, keys, label):
  need(type(value) is dict and set(value) == set(keys.split()), label)
  return value

def number(value, low, high, label):
  need(type(value) is int and low <= value <= high, label)
  return value


def counter():
  value=time.perf_counter_ns()
  number(value,1,2**63-1,"QPC-integer-range")
  return value

def strict(raw):
  def pairs(items):
    result = {}
    for key, value in items:
      need(key not in result, "duplicate-key")
      result[key] = value
    return result
  def bad(value):
    raise ValueError("nonfinite")
  return json.loads(raw, object_pairs_hook=pairs, parse_constant=bad)

def digest(raw):
  return hashlib.sha256(raw).hexdigest()

def path(value, fresh=False):
  need(type(value) is str and len(value) <= 2048, "path-bound")
  p = Path(value)
  need(p.is_absolute() and p.resolve() == p and ".." not in p.parts, "canonical-path")
  cursor = Path(p.anchor)
  for part in p.parts[1:]:
    cursor /= part
    if fresh and not os.path.lexists(cursor):
      continue
    info = cursor.lstat()
    need(not stat.S_ISLNK(info.st_mode) and not getattr(info, "st_file_attributes", 0) & 1024,
      "reparse-path")
  return p

def read(p, cap):
  p = path(str(p))
  with p.open("rb") as stream:
    a = os.fstat(stream.fileno())
    need(stat.S_ISREG(a.st_mode) and a.st_nlink == 1 and a.st_size <= cap, "bounded-input")
    raw = stream.read(cap + 1)
    b = os.fstat(stream.fileno())
  need(len(raw) <= cap and (a.st_dev, a.st_ino, a.st_size, a.st_mtime_ns) ==
    (b.st_dev, b.st_ino, b.st_size, b.st_mtime_ns), "input-changed")
  return raw

def reference(ref, cap, decode=True):
  shape(ref, "path bytes sha256", "closed-reference")
  number(ref["bytes"], 1, cap, "reference-size")
  need(re.fullmatch("[0-9a-f]{64}", ref["sha256"]) is not None, "reference-hash")
  raw = read(ref["path"], cap)
  need(len(raw) == ref["bytes"] and digest(raw) == ref["sha256"], "reference-bytes")
  return strict(raw) if decode else raw

def file_ref(p, cap):
  raw = read(p, cap)
  return dict(path=str(p), bytes=len(raw), sha256=digest(raw))

def native_ssh_config(c,m,review):
  p=NATIVE_SSH_PROFILE
  shape(m,"phase_manifest_ref phase_deadline_epoch source_manifest_ref approved_operator_source_ref","native-ssh-profile")
  need(m["phase_deadline_epoch"]==c["absolute_deadline"],"one-native-Windows-phase-endpoint")
  ssh=dict(path=p["ssh_path"],bytes=p["ssh_bytes"],sha256=p["ssh_sha256"])
  with path(p["ssh_path"]).open("rb") as stream:
    before=os.fstat(stream.fileno())
    need(stat.S_ISREG(before.st_mode) and before.st_size==p["ssh_bytes"] and
         before.st_nlink==p["ssh_nlink"],"fixed-system-SSH-file")
    raw=stream.read(2*1024**2+1)
    after=os.fstat(stream.fileno())
  need((before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns,before.st_nlink)==
       (after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns,after.st_nlink) and
       len(raw)==p["ssh_bytes"] and digest(raw)==p["ssh_sha256"],"same-buffer-system-SSH-image")
  bootstrap=m["approved_operator_source_ref"]
  reference(bootstrap,65536,False)
  reference(m["source_manifest_ref"],131072,False)
  need(path(bootstrap["path"])==Path(__file__).resolve().parent.parent/p["local_bootstrap"] and
       review.get("native_ssh_phase_reviewed") is True and
       review.get("native_bootstrap_sha256")==bootstrap["sha256"] and
       review.get("native_source_manifest_sha256")==m["source_manifest_ref"]["sha256"],
       "reviewed-native-phase-source-closure")
  phase_ref=m["phase_manifest_ref"]
  phase=reference(phase_ref,131072)
  name=path(phase_ref["path"]).name
  need(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]{0,122}\.json",name) is not None,
       "closed-native-parameter-name")
  native=str(PurePosixPath(p["bootstrap"]).parent/name)
  need(phase["go"] is True and phase["final_pin"]=="0628322e4b48e0efad19cdc32238a622d4aaa7a8" and
       phase["phase"]["deadline_epoch"]==m["phase_deadline_epoch"] and
       0<phase["phase"]["seconds"]<=3600 and phase["python"]==p["remote_python"] and
       phase["source_roots"]["suite"]==str(PurePosixPath(p["bootstrap"]).parent) and
       phase["runtime_scope"] in ("interface-controls-v5","private-cache-v5",
                                  "native-build-v5","workspace-v5","full-body"),
       "actual-native-phase-admission")
  need(c["inner"] is None and c["invocation"] is None and c["release"] is None and
       c["sources"]["bootstrap"] is None and c["sources"]["context"] is None and
       c["context_manifest_output"] is None,"native-SSH-only-route")
  remote=[p["remote_python"],*p["remote_python_flags"],p["bootstrap"],"--phase",
          "--manifest",native,"--sha256",phase_ref["sha256"],
          "--phase-deadline",str(m["phase_deadline_epoch"])]
  command=shlex.join(remote)
  need(len(command.encode())<=4096,"bounded-fixed-native-command")
  return dict(argv=[p["ssh_path"],*p["flags"],p["alias"],command],ssh_source=ssh,
              ssh_file_identity=dict(device=before.st_dev,inode=before.st_ino,nlink=before.st_nlink),
              bootstrap_source=bootstrap,source_manifest=m["source_manifest_ref"],
              phase_manifest=phase_ref,native_manifest=native,
              deadline_epoch=m["phase_deadline_epoch"],scope=phase["runtime_scope"])

def _load_config(config_path, expected,wd):
  raw = read(config_path, 131072)
  need(digest(raw) == expected, "configuration-hash")
  c = shape(strict(raw), "api runtime_go mode run_nonce absolute_deadline python sources budget "
       "receipt manifest invocation inner outputs source_review release context_manifest_output", "capture-config")
  need(c["api"] == API and c["runtime_go"] is True and
    c["mode"] in ("controls", "guarded-bootstrap","context","native-ssh-phase"), "explicit-capture-GO")
  if c["mode"]=="controls":wd.tighten(min(wd.deadline,time.time()+900))
  need(type(c["absolute_deadline"]) in (int, float) and math.isfinite(c["absolute_deadline"]) and
    time.time() < c["absolute_deadline"] <= wd.deadline,
    "absolute-capture-deadline")
  need(type(c["run_nonce"]) is str and re.fullmatch("[0-9a-f]{32}", c["run_nonce"]), "run-nonce")
  wd.tighten(c["absolute_deadline"])
  src = shape(c["sources"], "guard controls capture bootstrap context", "source-universe")
  here = Path(__file__).resolve()
  for key, cap, name in (("guard", 77824, "job_guard_bounded.py"),
              ("controls", 15360, "controls.py"),
              ("capture", 65536, "unlocked_capture.py")):
    reference(src[key], cap, False)
    need(path(src[key]["path"]) == here.parent / name, "fixed-cooperating-source")
  need(path(src["capture"]["path"]) == here, "capture-self")
  reference(c["python"], 64 * 1024**2, False)
  need(path(c["python"]["path"]) == Path(sys.executable).resolve(), "exact-Python")
  reference(c["budget"], 131072)
  manifest = reference(c["manifest"], 2 * 1024**2)
  review = reference(c["source_review"], 65536)
  need(review.get("source_reviewed") is True and review.get("capture_sha256") ==
    src["capture"]["sha256"] and review.get("guard_sha256") == src["guard"]["sha256"] and
    review.get("no_job_handle_duplication") is True and
    review.get("no_job_handle_inheritance") is True, "actual-reviewed-source-audit")
  if c["mode"] == "controls":
    need(c["inner"] is None and c["invocation"] is None and c["release"] is None and
      src["bootstrap"] is None and src["context"] is None and c["context_manifest_output"] is None, "controls-only-route")
    shape(manifest, "version runtime_go guard cases summary_limit summary_path absolute_deadline",
       "controls-manifest")
    need(manifest["runtime_go"] is True and manifest["guard"]["sha256"] ==
      src["guard"]["sha256"] and path(manifest["guard"]["path"]) == path(src["guard"]["path"]) and
      manifest["absolute_deadline"] == c["absolute_deadline"], "same-controls-source-deadline")
    tree = ast.parse(reference(src["controls"], 16384, False))
    cases = ast.literal_eval(next(n.value for n in tree.body if isinstance(n, ast.Assign) and
      any(isinstance(t, ast.Name) and t.id == "CASES" for t in n.targets)))
    need(len(cases) == 27 and [x["name"] for x in manifest["cases"]] == list(cases) and
      cases[-1] == "pipe-fence" and manifest["summary_limit"] == 65536, "exact-27-controls")
  elif c["mode"]=="native-ssh-phase":
    c["_native_ssh"]=native_ssh_config(c,manifest,review)
  elif c["mode"]=="context":
    reference(src["context"],98304,False)
    need(path(src["context"]["path"])==here.parent.parent/"fixture-retirement-native-context/windows_context.py","fixed-Context-source")
    shape(manifest,"version runtime_go action parent_capture_sha256 context_source_sha256 deadline_perf_ns deadline_epoch real_retire_go authority parent","Context-template")
    need(manifest["version"]==1 and manifest["runtime_go"] is True and manifest["parent"] is None and
      manifest["action"] in ("CONTROL","ARCHIVE_VERIFY","RETIRE") and
      manifest["context_source_sha256"]==src["context"]["sha256"] and
      manifest["parent_capture_sha256"]==src["capture"]["sha256"] and
      time.time()<manifest["deadline_epoch"]<=c["absolute_deadline"] and
      counter()<manifest["deadline_perf_ns"] and
      manifest["deadline_perf_ns"]-counter()<=int((c["absolute_deadline"]-time.time())*1e9), "actual-Context-template-binding")
    need((manifest["action"]=="RETIRE" and manifest["real_retire_go"] is True) or
      (manifest["action"]!="RETIRE" and manifest["real_retire_go"] is False),"explicit-Context-action")
    need(review.get("context_source_sha256")==src["context"]["sha256"] and review.get("context_reviewed") is True,"full-Context-source-review")
    need(c["inner"] is None and c["invocation"] is None and c["release"] is None and src["bootstrap"] is None,"Context-only-route")
    shape(c["context_manifest_output"],"path max_bytes","generated-Context-manifest")
    need(c["context_manifest_output"]["max_bytes"]==65536,"Context-manifest-cap")
  else:
    need(src["context"] is None and c["context_manifest_output"] is None,"noncontext-route")
    reference(src["bootstrap"], 16384, False)
    need(path(src["bootstrap"]["path"]) ==
      here.parent.parent / "windows-final-platform/windows_bootstrap.py",
      "only-reviewed-Windows-bootstrap")
    need(manifest["runtime_go"] is True and manifest["guard"]["sha256"] ==
      src["guard"]["sha256"] and manifest["absolute_deadline"] <= c["absolute_deadline"],
      "same-guard-bootstrap-release")
    shape(c["inner"], "budget receipt completion", "inner-worker")
    reference(c["inner"]["budget"], 131072)
    shape(c["inner"]["completion"], "path max_bytes", "inner-completion-sink")
    need(c["inner"]["completion"]["max_bytes"] == 65536, "inner-completion-cap")
    inner_budget=reference(c["inner"]["budget"],131072)
    c["_inner_receipt_cap"]=number(inner_budget["evidence_bounds"]["final"]["json_bytes"],8192,32*1024**2,"inner-receipt-read-cap")
    if c["invocation"] is not None:
      need(type(c["invocation"]) is str and c["invocation"] in manifest["invocations"],
        "closed-bootstrap-invocation")
    if c["release"] is not None:
      shape(c["release"], "path max_bytes", "parent-release")
      need(c["release"]["max_bytes"] == 8192, "release-cap")
  shape(c["outputs"], "audit lifetime completion census", "publisher-sinks")
  for sink in c["outputs"].values():
    shape(sink, "path max_bytes", "publisher-sink")
    number(sink["max_bytes"], 8192, 65536, "publisher-cap")
    need(path(sink["path"], True).parent.is_dir(), "publisher-directory")
  c["_config_ref"]=dict(path=str(path(config_path)),bytes=len(raw),sha256=expected)
  c["_watchdog"]=wd
  c["_config_path"], c["_config_sha"], c["_manifest"] = str(path(config_path)), expected, manifest
  return c

def guard_module(c):
  spec = importlib.util.spec_from_file_location("unlocked_guard_dependency", c["sources"]["guard"]["path"])
  module = importlib.util.module_from_spec(spec)
  raw=reference(c["sources"]["guard"],77824,False)
  exec(compile(raw,c["sources"]["guard"]["path"],"exec"),module.__dict__)
  need(module.sha_file(module.__file__) == c["sources"]["guard"]["sha256"], "guard-import-binding")
  return module


def child_forecast(c,g):
  entries = []
  m = c["_manifest"]
  if c["mode"] == "controls":
    for item in m["cases"]:
      budget=g.Budget(item["budget"])
      need(budget.raw_hash==item["budget_sha256"],"case-budget-hash")
      b=budget.config
      try:
        g.Bounds(budget,path(item["receipt"],True),[c["python"]["path"],"-I","-S","-B","-u","-c",""],None,None)
      except ValueError as error:
        need(item["name"]=="final-preflight" and str(error)=="prospective-final-json-envelope","only-intended-admission-negative")
      v = b["evidence_bounds"]
      size = v["rss"]["gzip_bytes"] + v["final"]["json_bytes"] + v["final"]["emergency_bytes"]
      size += sum(v["stdio"][k]["capture_bytes"] + v["stdio"][k]["overflow_tail_bytes"]
            for k in ("stdout", "stderr"))
      size += v["final"]["completion_bytes"]
      need(v["aggregate"]["retained_files"] == 5 and
        v["aggregate"]["retained_bytes"] >= size - v["final"]["completion_bytes"],
        "case-aggregate")
      entries.append((path(item["receipt"], True), size, 5))
    entries.append((path(m["summary_path"], True), 65536, 1))
  elif c["mode"]=="native-ssh-phase":
    return entries
  elif c["mode"]=="context":
    authority=reference(m["authority"],65536)
    body=reference(authority["budget"],65536)
    shape(body,"version root_caps review","Context-budget")
    reference(body["review"],65536)
    need(type(body["root_caps"]) is list and len(body["root_caps"])<=32,"Context-budget-roots")
    for rule in body["root_caps"]:
      shape(rule,"root bytes files reserve_bytes reserve_files","Context-root-reservation")
      for k in ("bytes","files","reserve_bytes","reserve_files"):number(rule[k],0,2**63-1,"Context-budget-number")
      if rule["reserve_bytes"] or rule["reserve_files"]:
        entries.append((path(rule["root"]),rule["reserve_bytes"],rule["reserve_files"]))
    entries.append((path(c["context_manifest_output"]["path"],True),65536,1))
  else:
    b = reference(c["inner"]["budget"], 131072)["evidence_bounds"]
    size = b["rss"]["gzip_bytes"] + b["final"]["json_bytes"] + b["final"]["emergency_bytes"]
    size += sum(b["stdio"][k]["capture_bytes"] + b["stdio"][k]["overflow_tail_bytes"]
          for k in ("stdout", "stderr")) + b["final"]["completion_bytes"]
    need(b["final"]["completion_bytes"] <= 32768, "worker-return-envelope")
    entries.extend([(path(c["inner"]["receipt"], True), size, 5),
      (path(c["inner"]["completion"]["path"], True), 65536, 1)])
    entry = m["entrypoint"] if c["invocation"] is None else m["invocations"][c["invocation"]]
    gate = entry["startup_gate"]
    need(gate["reserved"] is True and gate["max_bytes"] == 8192, "bootstrap-gate-admission")
    entries.extend((path(gate[k], True), 8192, 1) for k in ("ready", "release"))
    if c["release"] is not None:
      entries.append((path(c["release"]["path"], True), 8192, 1))
  return entries



class WholeCallWatchdog:
  def __init__(self):
    self.deadline=min(time.time()+3600,FUNCTIONAL)
    self.expired=False;self.completed=False;self.owner=None
    self.timers=[];self.lock=threading.RLock()
    self.arm(self.deadline)
  def arm(self,end):
    timer=threading.Timer(max(0,end-time.time()),self.expire)
    timer.daemon=False;self.timers.append(timer);timer.start()
  def tighten(self,end):
    need(time.time()<end<=self.deadline,"same-whole-call-deadline")
    if end<self.deadline:self.deadline=end;self.arm(end)
  def check(self,closure=False):
    need(not self.completed,"whole-call-already-finished")
    need((not self.expired and time.time()<self.deadline) if not closure else time.time()<=min(self.deadline+12,CLOSEOUT),
         "whole-call-deadline")
  def expire(self):
    with self.lock:
      if self.expired or self.completed:return
      self.expired=True
      owner=self.owner
      if owner is None and not getattr(getattr(self,"worker_guard",None),"_ROOT_PENDING",False):os._exit(124)
      grace=threading.Timer(12,self.enforce)
      grace.daemon=False;self.timers.append(grace);grace.start()
    if owner is None:return
    owner.failed=True
    if hasattr(owner,"result"):owner.result["qualified"]=False
    if hasattr(owner,"g") and (getattr(owner,"job",None) or getattr(owner,"process",None)):
      owner.terminate("whole-call-deadline")
  def enforce(self):
    with self.lock:
      if self.completed:return
      owner=self.owner
      if getattr(getattr(self,"worker_guard",None),"_ROOT_PENDING",False):
        _FENCES.append(self);return
      if owner is not None and (getattr(owner,"creating_root",False) or
          (getattr(owner,"process",None) is not None and not getattr(owner,"assigned",False)
           and not getattr(owner,"root_reaped",False))):
        if owner not in _FENCES:_FENCES.append(owner)
        return # Unpreemptible acquisition remains fenced; never orphan its suspended root.
    os._exit(124)
  def finish(self):
    with self.lock:
      self.check(closure=self.expired)
      need(not _FENCES and (self.owner.status=="closed" if self.owner else not self.expired),"whole-call-resource-fence")
      self.completed=True
    for timer in self.timers:timer.cancel()
    for timer in self.timers:
      timer.join(1)
      if timer.is_alive():_FENCES.append(self);raise RuntimeError("whole-watchdog-unjoined")

_WHOLE=None
def whole_watchdog():
  global _WHOLE
  if _WHOLE is None:_WHOLE=WholeCallWatchdog()
  return _WHOLE

def load_config(config_path,expected):
  wd=whole_watchdog()
  wd.check()
  return _load_config(config_path,expected,wd)

class Capture:
  """Native capability owned by this publisher, never reconstructed from PID-only JSON."""
  def __init__(self,c):
    self.watchdog=c["_watchdog"]
    need(type(self.watchdog) is WholeCallWatchdog,"source-owned-whole-watchdog")
    self.watchdog.owner=self
    self.creating_root=False;self.assigned=False;self.root_reaped=False
    self.root_close_attempted=False;self.job_close_attempted=False
    self._initialize(c)

  def _initialize(self, c):
    need(not _FENCES, "prior-outer-resource-uncertainty")
    self.c = c
    self.g = g = guard_module(c)
    self.budget = g.Budget(c["budget"]["path"])
    need(self.budget.raw_hash==c["budget"]["sha256"],"actual-reviewed-outer-budget")
    self.config_ref=c["_config_ref"]
    g.check_census_envelope(self.budget)
    self.publisher = dict(pid=os.getpid(), creation=g._stamp(g.K.GetCurrentProcess()),
               image=str(Path(sys.executable).resolve()),
               image_sha256=c["python"]["sha256"])
    self.lock = threading.RLock()
    self.stop = threading.Event()
    self.process = self.job = self.raw = self.pipes = self.js = None
    self.timer = self.disk = None
    self.lane = None
    self.lane_locked = False
    self.status = "preparing"
    self.failed = False
    self.first_error = None
    self.native_calls = 0
    self.empties = []
    self.job_close = self.host = self.creator = self.root_exit = None
    self.gate_released = False
    self.last_sample = self.last_tick = self.last_complete = None
    self.coverage_checks={}
    self.max_gap = self.max_duration = self.samples = 0
    self.saw_nested = False
    self.published = set()
    self.receipt = path(c["receipt"], True)
    self.root_source=c["python"]
    self.argv = [c["python"]["path"], "-I", "-S", "-B", "-u"]
    if c["mode"] == "controls":
      self.argv += [c["sources"]["controls"]["path"], c["manifest"]["path"], c["manifest"]["sha256"]]
    elif c["mode"]=="native-ssh-phase":
      self.argv=c["_native_ssh"]["argv"]
      self.root_source=c["_native_ssh"]["ssh_source"]
    elif c["mode"]=="context":
      self.argv += [c["sources"]["context"]["path"],c["context_manifest_output"]["path"],c["manifest"]["sha256"]]
    else:
      self.argv += [c["sources"]["capture"]["path"], "--worker", c["_config_path"], c["_config_sha"]]
    self.bounds = g.Bounds(self.budget, self.receipt, self.argv, None, None)
    if c["mode"]=="native-ssh-phase":
      need(self.bounds.b["stdio"]["stdout"]["capture_bytes"]==16384,
           "fixed-native-terminal-stdout-cap")
    self.state = g.EvidenceState(self.bounds)
    self.state.run_nonce = c["run_nonce"]
    self.state.terminate = self.terminate
    self.errors = g.BoundedList(self.state, "errors")
    self.pending = [(path(v["path"], True), v["max_bytes"], 1) for v in c["outputs"].values()]
    need(len({str(x[0]) for x in self.pending+[(p,0,0) for p in self.bounds.files]}) == 9 and
      all(not os.path.lexists(x[0]) for x in self.pending), "fresh-distinct-publisher-sinks")
    self.forecast = child_forecast(c,g)
    need(all(not p.is_relative_to(Path(__file__).resolve().parent) for p,n,f in self.pending+self.forecast), "no-runtime-source-writes")
    self.held = self.bounds.reserves(self.g.reserves(self.budget, self.pending + self.forecast +
      [(OWNER, 16384, 2)]))
    self.budget.check(self.held)
    try:
      self.lane = path(str(LANE)).open("r+b")
      info = os.fstat(self.lane.fileno())
      need(info.st_size == 1 and info.st_nlink == 1 and stat.S_ISREG(info.st_mode), "initialized-outer-lane")
      self.lane.seek(0)
      g.msvcrt.locking(self.lane.fileno(), g.msvcrt.LK_NBLCK, 1)
      self.lane_locked = True
      if OWNER.exists():
        previous = g._strict_read(OWNER, 8192)[0]
        need(previous["state"] == "closed" and previous["nonce"] != c["run_nonce"], "unresolved-prior-outer-owner")
      g.supervisors()
      self.budget.check(self.held)
      if c["mode"]=="context":
        self.bind_context_manifest()
        self.bounds=g.Bounds(self.budget,self.receipt,self.argv,None,None)
        self.state.bounds=self.bounds
        self.budget.check(self.held)
      self.job = g.K.CreateJobObjectW(None, None)
      need(bool(self.job), "create-outer-Job")
      self.creator = g._creator(self.job)
      self.job_limit_checks=[g.set_job_limits(self.job,16,self.state)]
      self.raw = g.StoredGzip(self.receipt.with_suffix(".rss.jsonl.gz"), self.state)
      self.state.raw = self.raw
      self.timer = threading.Timer(max(0, c["absolute_deadline"] - time.time()),
                     self.terminate, args=("outer-absolute-deadline",))
      self.timer.daemon = False
      self.timer.start()
      env = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
      self.watchdog.check()
      self.creating_root=True
      self.process = subprocess.Popen(self.argv, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, close_fds=True, bufsize=0, creationflags=4, env=env)
      self.creating_root=False
      self.root_creation=None
      self.watchdog.check()
      self.root_creation=g._stamp(g.HANDLE(self.process._handle))
      self.root_image=g.native_image(g.HANDLE(self.process._handle))
      need(path(self.root_image)==path(self.root_source["path"]),"actual-root-image")
      need(bool(g.K.AssignProcessToJobObject(self.job, g.HANDLE(self.process._handle))),
        "nested-outer-Job-assignment")
      inside=g.w.BOOL()
      need(bool(g.K.IsProcessInJob(g.HANDLE(self.process._handle),self.job,g.ref(inside))) and inside.value and self.process.pid in self.members(),"actual-root-containment")
      self.assigned=True
      self.js = g.JobSampling(self.state, self.job, self.members)
      self.pipes = g.PipeCapture(self.process, self.receipt, self.state)
      self.pipes.start()
      self.sample(suspended=True)
      self.owner = dict(pid=self.publisher["pid"], creation_time=self.publisher["creation"],
        nonce=c["run_nonce"], guard_sha256=c["sources"]["capture"]["sha256"],
        budget_sha256=self.budget.raw_hash,bounds_sha256=self.bounds.sha,config_ref=self.config_ref,
        state="suspended", job_root_pid=self.process.pid,
        job_root_creation_time=self.js.live[self.process.pid][1], receipt=str(self.receipt))
      g.control_json(OWNER, self.owner)
      self.budget.check()
      self.status = "suspended"
    except BaseException as error:
      self.fail(error, "outer-prepare")
      self.finish()
      raise

  def bind_context_manifest(self):
    args=self.g.native_argv()
    source_path=next(x for x in args if Path(x).is_absolute() and Path(x).resolve()==Path(__file__).resolve())
    manifest=dict(self.c["_manifest"])
    manifest["parent"]=dict(pid=self.publisher["pid"],creation=self.publisher["creation"],
      image=dict(self.c["python"],path=str(path(self.g.native_image(self.g.K.GetCurrentProcess())))),
      source=dict(self.c["sources"]["capture"],path=source_path),argv=args)
    sink=self.c["context_manifest_output"]
    target=path(sink["path"],True)
    self.budget.check(self.g.reserves(self.budget,[(target,65536,1)]))
    self.g.write_json(target,manifest,65536)
    actual=file_ref(target,65536)
    self.argv[-1]=actual["sha256"]
    self.c["context_runtime_manifest_ref"]=actual

  def members(self):
    with self.lock:
      return self.g.members(self.job) if self.job is not None else []

  def fail(self, error, label):
    self.failed = True
    if self.first_error is None:
      self.first_error = (label + ":" + self.g._error(error))[:128]
    self.errors.append(self.first_error)
    self.terminate(label)

  def terminate(self, reason):
    self.failed = True
    if self.first_error is None:
      self.first_error = reason[:128]
    with self.lock:
      if self.job and not self.job_close_attempted and self.native_calls < 4:
        self.native_calls += 1
        if not self.g.K.TerminateJobObject(self.job, 137):
          self.state.uncertain = True
      try:self.terminate_unassigned_root()
      except BaseException:self.state.uncertain=True

  def terminate_unassigned_root(self):
    with self.lock:
      if self.process is None or self.assigned or self.root_close_attempted:return
      g=self.g;h=g.HANDLE(self.process._handle)
      state=g.K.WaitForSingleObject(h,0)
      if state==0:return
      need(state==258,"unassigned-root-wait-unknown")
      g.K.TerminateProcess.argtypes=[g.HANDLE,g.w.UINT]
      g.K.TerminateProcess.restype=g.w.BOOL
      need(bool(g.K.TerminateProcess(h,137)),"owned-unassigned-root-termination")
  def terminal_coverage(self,label,observed):
    gaps={k:observed-v if v is not None else None for k,v in
          (("start_gap_ns",self.last_tick),("completion_gap_ns",self.last_complete),
           ("recorded_gap_ns",self.last_sample))}
    proof=dict(perf_ns=observed,**gaps)
    proof["raw_recorded"]=bool(self.raw and self.raw.record(dict(event=label,sample_complete=False,**proof)))
    self.coverage_checks[label]=proof
    if not proof["raw_recorded"] or any(v is None or v>40000000 or v<0 for v in gaps.values()):
      self.fail(RuntimeError("terminal-coverage"),label)

  def sample(self, suspended=False):
    g = self.g
    start = counter()
    g.check_job_limits(self.job,16,self.state)
    pids = self.members()
    need(len(pids)<=16,'outer-live-process-count')
    self.js.sweep(pids)
    rows = []
    total = 0
    for pid in pids:
      value = self.js.memory(pid)
      if value is None:
        need(pid in self.js.departed or pid not in self.members(), "outer-unreadable-live-process")
        need(self.raw.record(dict(event="outer-exit-race",sample_complete=False,pid=pid,
          departed=self.js.departed.get(pid),perf_counter_ns=counter())),"outer-exit-evidence-cap")
        continue
      need(value[0] <= 2 * g.GiB, "outer-process-RSS")
      rows.append(dict(pid=pid, creation_time=self.js.live[pid][1], rss=value[0], peak=value[1]))
      total += value[0]
    supervisor, supervisor_rows = g.supervisor_rss()
    need(total <= 3 * g.GiB and total + supervisor <= 4 * g.GiB, "outer-aggregate-RSS")
    if suspended:
      need(any(x["pid"] == self.process.pid and x["rss"] > 0 for x in rows),
        "positive-suspended-outer-root")
    self.saw_nested |= any(x["pid"] != self.process.pid for x in rows)
    done = counter()
    start_gap=start-self.last_tick if self.last_tick is not None else 0
    complete_gap=done-self.last_complete if self.last_complete is not None else 0
    need(self.raw.record(dict(event="rss-sample", sample_complete=True,
      epoch=time.time(), perf_counter_ns=start, completed_perf_counter_ns=done,
      job_rss=total, processes=rows, supervisor_rss=supervisor, supervisors=supervisor_rows,
      suspended=suspended,interval_seconds=start_gap/1e9,
      completion_interval_seconds=complete_gap/1e9,collection_duration_seconds=(done-start)/1e9)), "outer-RSS-evidence-cap")
    recorded = counter()
    gap = recorded - self.last_sample if self.last_sample is not None else 0
    if not suspended:
      self.max_gap = max(self.max_gap, gap)
    self.max_duration = max(self.max_duration, recorded - start)
    self.samples += 1
    need(self.raw.record(dict(event="rss-write-complete", sample_complete=False,
      sample_id=self.samples, perf_counter_ns=start, completed_perf_counter_ns=done,
      recorded_perf_counter_ns=recorded, duration_seconds=(recorded-start)/1e9)),
      "outer-write-marker-cap")
    self.last_sample,self.last_tick,self.last_complete=recorded,start,done
    need(recorded-start<=40000000 and (suspended or max(gap,start_gap,complete_gap)<=40000000),
      "outer-40ms-coverage")

  def disk_watch(self):
    while not self.stop.is_set() and not self.state.frozen:
      try:
        self.budget.check(stop=self.stop)
      except BaseException as error:
        if not self.stop.is_set():
          self.fail(error, "outer-disk-or-free")
        return
      self.stop.wait(5)

  def release_bootstrap_gate(self):
    if self.c["mode"] != "guarded-bootstrap":
      return
    m = self.c["_manifest"]
    entry = m["entrypoint"] if self.c["invocation"] is None else m["invocations"][self.c["invocation"]]
    gate = entry["startup_gate"]
    target = Path(gate["release"])
    if self.gate_released:
      return
    need(not target.exists(), "unexpected-bootstrap-release")
    if not Path(gate["ready"]).exists():
      return
    ready = self.g._strict_read(gate["ready"], 8192)[0]
    shape(ready, "pid nonce manifest_sha256", "actual-bootstrap-ready")
    need(ready["nonce"] == gate["nonce"] and ready["manifest_sha256"] ==
      self.c["manifest"]["sha256"], "ready-binding")
    pid = ready["pid"]
    need(type(pid) is int and pid in self.members(), "ready-native-outer-member")
    need(self.js.memory(pid) is not None, "ready-positive-native-RSS")
    owner = self.g._strict_read(self.g.OWNER, 8192)[0]
    need(owner["pid"] == self.process.pid and owner["creation_time"] ==
      self.owner["job_root_creation_time"] and owner["job_root_pid"] == pid and
      owner["job_root_creation_time"] == self.js.live[pid][1] and
      owner["guard_sha256"] == self.c["sources"]["guard"]["sha256"] and
      owner["state"] == "running", "actual-inner-worker-and-bootstrap")
    self.budget.check(self.g.reserves(self.budget, [(target, 8192, 1)]))
    self.g.write_json(target, dict(nonce=gate["nonce"], pid=pid), 8192)
    self.gate_released = True

  def start(self):
    try:
      return self._start()
    except BaseException as error:
      self.fail(error,"outer-start")
      return self.finish()

  def _start(self):
    need(self.status == "suspended" and not self.failed, "only-prepared-start")
    self.sample(suspended=True)
    self.watchdog.check()
    self.job_limit_checks.append(self.g.check_job_limits(self.job,16,self.state))
    need(self.g.NT.NtResumeProcess(self.g.HANDLE(self.process._handle)) == 0, "outer-native-resume")
    self.status = "running"
    self.owner.update(state="running", resumed_perf_ns=counter())
    self.g.control_json(OWNER, self.owner)
    self.disk = threading.Thread(target=self.disk_watch, name="outer-disk", daemon=False)
    self.disk.start()
    next_tick = counter() + 20000000
    try:
      while self.process.poll() is None and not self.state.frozen and not self.failed:
        need(time.time() < self.c["absolute_deadline"], "outer-deadline")
        self.stop.wait(max(0, (next_tick - counter()) / 1e9))
        self.sample()
        self.release_bootstrap_gate()
        next_tick = max(next_tick + 20000000, counter())
    except BaseException as error:
      self.fail(error, "outer-monitor")
    finally:
      return self.finish()

  def finish(self):
    if self.status in ("closed", "uncertain"):
      return self.result
    g = self.g
    cleanup = min(self.c["absolute_deadline"] + 12, CLOSEOUT)
    def left():
      return max(0, cleanup - time.time())
    exit_ns = None
    try:
      if self.process:
        if self.process.poll() is None:
          self.terminate("outer-finish-live-root")
        self.process.wait(timeout=left())
        exit_ns = counter();self.root_reaped=True
        if self.status=="running":self.terminal_coverage("outer-root-end-coverage",exit_ns)
      if self.job and self.members():
        self.terminate("outer-descendant-drain")
        while self.members() and left() > 0:
          time.sleep(min(.01, left()))
      if self.status=="running" and self.job:
        need(self.members()==[],"actual-terminal-Job-empty")
        self.terminal_coverage("outer-job-tail-coverage",counter())
      self.stop.set()
      if self.timer:
        self.timer.cancel()
        self.timer.join(left())
      if self.disk:
        self.disk.join(left())
      if self.pipes and self.pipes.thread:
        self.pipes.thread.join(min(5, left()))
        if self.pipes.thread.is_alive():
          self.pipes.stop.set()
          self.pipes.thread.join(min(1, left()))
      workers = (self.timer, self.disk, self.pipes.thread if self.pipes else None)
      need(not any(x and x.is_alive() for x in workers), "outer-unjoined-worker")
      need(not self.creating_root,"root-acquisition-uncertain")
      need(not self.state.uncertain and not g._UNCLOSED, "outer-native-close-uncertain")
      if self.process and self.pipes is None:
        for stream in (self.process.stdin,self.process.stdout,self.process.stderr):
          if stream and not stream.closed:stream.close()
        self.preassign_stdio_closed=True
      if self.pipes:
        need(all(x["closed"] for x in self.pipes.rows.values()), "outer-pipe-close")
      if self.job:
        for unused in range(2):
          need(left() > 0, "outer-cleanup-deadline")
          member = self.members()
          self.empties.append(dict(perf_ns=counter(), members=member))
          need(member == [], "outer-nonempty-Job")
          time.sleep(min(.02, left()))
      if self.js:
        self.js.close()
      if self.raw:
        self.raw.close()
      if self.process:
        self.lock.acquire()
        h = g.HANDLE(self.process._handle)
        need(g.K.GetProcessId(h) == self.process.pid and
          (self.root_creation is None or g._stamp(h) == self.root_creation) and
          g.K.WaitForSingleObject(h, 0) == 0, "actual-outer-root-exit")
        creation = g._stamp(h)
        signalled = g.K.WaitForSingleObject(h, 0)
        need(signalled == 0, "native-guard-worker-signalled")
        self.root_close_attempted=True
        closed = g._checked_close(h, "outer-root-Popen")
        self.root_exit = dict(pid=self.process.pid,creation=creation,wait_result=signalled,
          exit_code=self.process.returncode,handle_close_result=closed,closed_perf_ns=counter())
        self.process._handle.closed = True
        self.lock.release()
        self.host = dict(pid=self.process.pid, creation=creation,
          exit_observed_perf_ns=exit_ns, exit_code=self.process.returncode,
          io_closed=bool(self.pipes and all(x["closed"] for x in self.pipes.rows.values())) or getattr(self,"preassign_stdio_closed",False),
          writer_closed=bool(self.raw and self.raw.closed and self.raw.file.closed),
          callbacks_joined=not any(x and x.is_alive() for x in workers))
      if self.job:
        try:g.check_job_limits(self.job,16,self.state,self.job_limit_checks)
        except BaseException as error:
          self.job_limit_checks.append(dict(final_error=g._error(error)))
          self.fail(error,"outer-final-Job-accounting")
        self.lock.acquire();self.job_close_attempted=True
        closing = self.job
        closed = g._checked_close(closing, "outer-final-Job")
        self.job_close = dict(success=closed, completed_perf_ns=counter())
        self.job = None;self.lock.release()
      self.status = "closed"
    except BaseException as error:
      try:self.lock.release()
      except RuntimeError:pass
      self.failed = True
      self.first_error = self.first_error or ("outer-close:" + g._error(error))[:128]
      self.status = "uncertain"
      _FENCES.append(self)
    physical = self.status == "closed" and ((self.host is not None and len(self.empties) == 2) or self.process is None)
    self.result = dict(schema=API, run_nonce=self.c["run_nonce"],
      budget_sha256=self.budget.raw_hash,bounds_sha256=self.bounds.sha,config_ref=self.config_ref,
      guard_sha256=self.c["sources"]["capture"]["sha256"], dependency_guard_sha256=self.c["sources"]["guard"]["sha256"],
      job_limit_checks=getattr(self,"job_limit_checks",[]),creator_lifetime=self.creator, host=self.host, closure_checks=self.empties, job_close=self.job_close,
      terminal_coverage=self.coverage_checks,process_closure_verified=physical, errors=list(self.errors), first_error=self.first_error,
      qualified=False, stdio=self.pipes.reference() if self.pipes else {},
      rss_evidence=self.raw.reference() if self.raw else None, samples=self.samples,
      max_interval_ns=self.max_gap, max_sample_duration_ns=self.max_duration,
      nested_members_sampled=self.saw_nested)
    try:
      g.write_json(self.receipt, self.result, self.bounds.b["final"]["json_bytes"])
      g.write_json(self.receipt.with_suffix(".bounds.json"),
        dict(resource_state=self.status, failed=self.failed, cap=self.state.cap_reason,
          close_unknown=self.status == "uncertain"), self.bounds.b["final"]["emergency_bytes"])
      self.result["receipt_ref"] = file_ref(self.receipt, self.bounds.b["final"]["json_bytes"])
      need(physical,"no-postclose-census-on-unknown-resources")
      self.result["postclose_census"] = self.budget.check()
      self.result["qualified"] = physical and not self.failed and not self.watchdog.expired and len(self.coverage_checks)==2 and self.process.returncode == 0 and \
        self.raw.complete and all(x["complete"] and x["eof"] for x in self.pipes.reference().values())
      if self.c["mode"] == "controls" and self.result["qualified"]:
        reported = strict(read(self.receipt.with_suffix(".stdout"), self.bounds.b["stdio"]["stdout"]["capture_bytes"]))
        self.result["qualified"] = reported["status"] == "CONTROLS_PASSED" and self.saw_nested
    except BaseException as error:
      self.failed = True
      self.result["qualified"] = False
      self.result["publication_error"] = g._error(error)
    if not physical:
      self.result["qualified"]=False
    if hasattr(self, "owner"):
      self.owner["state"] = "closed" if physical else "closure-unknown"
      try:
        g.control_json(OWNER, self.owner)
      except BaseException:
        self.result["qualified"] = False
        _FENCES.append(self)
    if self.lane_locked and physical and not _FENCES:
      try:
        self.lane.seek(0)
        g.msvcrt.locking(self.lane.fileno(), g.msvcrt.LK_UNLCK, 1)
        self.lane_locked = False
        self.lane.close()
      except BaseException as error:
        self.status="uncertain";self.result["qualified"]=False
        self.result["process_closure_verified"]=False
        self.result["outer_lane_close_error"]=self.g._error(error)
        _FENCES.append(self)
        if hasattr(self,"owner"):
          self.owner["state"]="closure-unknown"
          try:self.g.control_json(OWNER,self.owner)
          except BaseException:pass
    if self.lane is not None and not self.lane_locked and not self.lane.closed:
      self.lane.close()
    return self.result

  def publish(self, key, value):
    self.watchdog.check()
    need(self.status == "closed" and not _FENCES, "publish-only-after-native-close")
    sink = self.c["outputs"][key]
    target = path(sink["path"], True)
    pending = [(p, n, f) for p, n, f in self.pending if str(p) not in self.published]
    self.budget.check(self.g.reserves(self.budget, pending))
    self.g.write_json(target, value, sink["max_bytes"])
    self.published.add(str(target))
    return file_ref(target, sink["max_bytes"])

def publish_parent_records(cap):
  cap.watchdog.check()
  need(cap.status == "closed" and cap.result["qualified"] is True and not _FENCES and not cap.watchdog.expired and cap.c["mode"] == "guarded-bootstrap", "actual-guard-worker-mode")
  worker_report=strict(read(cap.receipt.with_suffix(".stdout"),cap.bounds.b["stdio"]["stdout"]["capture_bytes"]))
  returned=worker_report["worker_completion_ref"]
  need(path(returned["path"])==path(cap.c["inner"]["completion"]["path"]),"actual-worker-return-sink")
  data=reference(returned,65536)
  shape(data, "schema completion receipt_ref owner native_host creator_lifetime closure_checks job_close", "worker-return")
  need(data["schema"] == "bounded-inner-guard-return-v1" and cap.host["exit_code"] == 0 and
    data["completion"]["qualified"] is True and data["completion"]["guard_sha256"] ==
    cap.c["sources"]["guard"]["sha256"], "qualified-exact-original-guard")
  creator = data["creator_lifetime"]
  need(creator["pid"] == cap.host["pid"] and creator["creation_time"] == cap.host["creation"],
    "native-reaped-original-guard-creator")
  original=reference(data["receipt_ref"], cap.c["_inner_receipt_cap"])
  need(data["completion"]["receipt_sha256"]==data["receipt_ref"]["sha256"] and
    original["run_nonce"]==data["completion"]["run_nonce"] and original["owner"]==data["owner"] and
    data["completion"]["budget_sha256"]==cap.c["inner"]["budget"]["sha256"],"same-original-receipt-return")
  audit = dict(guard_sha256=cap.c["sources"]["guard"]["sha256"],
    no_job_handle_duplication=reference(cap.c["source_review"],65536)["no_job_handle_duplication"],
    no_job_handle_inheritance=reference(cap.c["source_review"],65536)["no_job_handle_inheritance"],
    complete_cooperating_owner_universe=[dict(pid=creator["pid"], creation=creator["creation_time"])],
    source_review_ref=cap.c["source_review"],outer_controller_audit=dict(
      guard_sha256=cap.c["sources"]["capture"]["sha256"],
      no_job_handle_duplication=reference(cap.c["source_review"],65536)["no_job_handle_duplication"],
      no_job_handle_inheritance=reference(cap.c["source_review"],65536)["no_job_handle_inheritance"],
      complete_cooperating_owner_universe=[dict(pid=cap.creator["pid"],creation=cap.creator["creation_time"])]))
  audit_ref = cap.publish("audit", audit)
  row = dict(data, process_closure_verified=data["completion"]["process_closure_verified"],
       run_nonce=data["completion"]["run_nonce"])
  lifetime = cap.g.native_lifetime(row, cap.c["sources"]["guard"]["sha256"], audit_ref)
  life_ref = cap.publish("lifetime", lifetime)
  streams = cap.pipes.reference()
  completion = dict(schema="parent-original-guard-completion-v1",
    budget_sha256=cap.budget.raw_hash,bounds_sha256=cap.bounds.sha,config_ref=cap.config_ref, run_nonce=lifetime["run_nonce"],
    guard_sha256=lifetime["guard_sha256"], creator=lifetime["creator"],
    guard_reaped=bool(cap.root_exit and cap.root_exit["wait_result"] == 0 and cap.root_exit["handle_close_result"] is True),
    native_guard_worker_exit=cap.root_exit, outer_controller_lifetime=dict(schema="unlocked-outer-job-native-lifetime-v1",run_nonce=cap.c["run_nonce"],
      creator=dict(pid=cap.creator["pid"],creation=cap.creator["creation_time"]),
      creator_job_query=cap.creator["creator_job_query"],job_handle_flags=cap.creator["job_handle_flags"],
      source_audit_ref=audit_ref,source_audit_selector="outer_controller_audit",
      host=cap.host,empty_observations=cap.empties,job_close=cap.job_close,guard_sha256=cap.c["sources"]["capture"]["sha256"]),
    guard_exit_code=cap.process.returncode, stdout_eof=streams["stdout"]["eof"],
    stderr_eof=streams["stderr"]["eof"], callbacks_joined=cap.host["callbacks_joined"],
    completed_perf_ns=counter(), postclose_census=cap.budget.check(),
    original_returned_completion=data["completion"], outer_receipt_ref=cap.result["receipt_ref"])
  need(completion["guard_reaped"] and completion["guard_exit_code"] == 0 and
    completion["stdout_eof"] and completion["stderr_eof"] and completion["callbacks_joined"],
    "actual-parent-observed-guard-worker-completion")
  completion_ref = cap.publish("completion", completion)
  cap.parent_records = dict(lifetime=lifetime, lifetime_ref=life_ref,
    completion=completion, completion_ref=completion_ref, original_receipt_ref=data["receipt_ref"])
  census_ref = cap.publish("census", dict(actual_after_original_parent_records=cap.budget.check()))
  cap.publisher_refs=dict(audit=audit_ref,lifetime=life_ref,completion=completion_ref,census=census_ref)
  cap.parent_records["census_ref"] = census_ref
  cap.parent_records["inclusive_census"] = cap.budget.check()
  return cap.parent_records

def budget_reference(ref):
  if type(ref) is dict and "offset" in ref:
    shape(ref,"path offset bytes sha256","whole-budget-reference")
    need(type(ref["offset"]) is int and ref["offset"]==0,"whole-budget-zero-offset")
    ref={k:ref[k] for k in ("path","bytes","sha256")}
  return reference(ref,131072)

def validate_grant_source_freeze(authority,previous):
  from datetime import datetime
  freeze=shape(authority["source_freeze"],"engine harness manifest_ref review_ref","source-freeze")
  need(freeze["engine"]=="2026-10-21T04:10:14Z" and type(freeze["harness"]) is str and
       re.fullmatch(r"2026-10-21T\d{2}:\d{2}:\d{2}(?:\.\d{1,6})?Z",freeze["harness"]),
       "fixed-source-freeze-clock")
  instant=datetime.fromisoformat(freeze["harness"][:-1]+"+00:00")
  need(datetime.fromisoformat("2026-10-21T04:10:14+00:00")<=instant<
       datetime.fromisoformat("2026-10-21T11:10:14+00:00"),"source-freeze-before-functional")
  values={};total=0
  for name in ("manifest_ref","review_ref"):
    ref=shape(freeze[name],"path offset bytes sha256","source-freeze-reference")
    need(type(ref["offset"]) is int and ref["offset"]==0 and path(ref["path"]).is_relative_to(PARENT_ROOT),
         "parent-owned-source-freeze")
    total+=number(ref["bytes"],1,65536,"source-freeze-byte-cap")
    need(total<=65536,"source-freeze-pair-cap")
    values[name]=reference({k:ref[k] for k in ("path","bytes","sha256")},65536)
  manifest=shape(values["manifest_ref"],"kind version final_pin frozen_utc source_files","source-manifest")
  review=shape(values["review_ref"],"kind version verdict final_pin source_manifest_sha256 source_files_sha256 review_refs","source-review")
  pin=authority["source_pin"];files=manifest["source_files"]
  need(pin=="0628322e4b48e0efad19cdc32238a622d4aaa7a8" and
       manifest["kind"]=="private-harness-source-manifest-v1" and type(manifest["version"]) is int and
       manifest["version"]==1 and manifest["final_pin"]==pin and manifest["frozen_utc"]==freeze["harness"] and
       type(files) is dict and 0<len(files)<=512,"actual-source-freeze")
  encoded=json.dumps(files,sort_keys=True,separators=(",",":"),ensure_ascii=False,allow_nan=False).encode("utf-8")
  need(review["kind"]=="private-harness-review-acceptance-v1" and type(review["version"]) is int and
       review["version"]==1 and review["verdict"]=="SOURCE_ACCEPTED_FOR_BOUNDED_PHASE_ADMISSION" and
       review["final_pin"]==pin and review["source_manifest_sha256"]==freeze["manifest_ref"]["sha256"] and
       review["source_files_sha256"]==digest(encoded) and type(review["review_refs"]) is list and
       0<len(review["review_refs"])<=64,"reviewed-source-freeze-association")
  indexed={}
  for source,row in files.items():
    shape(row,"bytes sha256","source-manifest-entry")
    key=os.path.normcase(str(path(source)))
    need(key not in indexed,"unique-frozen-source-path");indexed[key]=row
  for name in ("guard","capture","controls"):
    ref=previous.c["sources"][name]
    need(indexed.get(os.path.normcase(str(path(ref["path"]))))==
         {k:ref[k] for k in ("bytes","sha256")},"actual-frozen-capture-source")

def _issue_postexit_grant(previous, prepared, plan):
  need(type(previous) is Capture and type(prepared) is Capture and previous.status == "closed" and previous.result["qualified"] is True and not _FENCES and not previous.watchdog.expired and previous.c["mode"] == "guarded-bootstrap" and
    prepared.status == "suspended" and previous.publisher == prepared.publisher and
    hasattr(previous, "parent_records") and prepared.c["mode"] == "guarded-bootstrap" and
    prepared.c["release"] is not None, "held-native-old-new-parent-capabilities")
  shape(plan, "path max_bytes phase_index_ref expected_source_pin expected_binary_sha256 "
    "expected_runtime_sha256 expected_profile_sha256 expected_finalizer_sha256 "
    "expected_new_guard_sha256 deadline_perf_ns authority_ref profile_catalogue_ref", "closed-postexit-plan")
  prepared.watchdog.check()
  target = path(plan["path"], True)
  for k in ("expected_binary_sha256","expected_runtime_sha256","expected_profile_sha256","expected_finalizer_sha256","expected_new_guard_sha256"):
    need(type(plan[k]) is str and re.fullmatch("[0-9a-f]{64}",plan[k]),"closed-grant-source-hash")
  need(re.fullmatch("[0-9a-f]{40}",plan["expected_source_pin"]) is not None,"grant-source-pin")
  need(target.is_relative_to(PARENT_ROOT) and plan["max_bytes"] == 65536, "parent-owned-grant-sink")
  native = prepared.g
  h = native.HANDLE(prepared.process._handle)
  creation = native._stamp(h)
  need(native.K.GetProcessId(h) == prepared.process.pid and native.K.WaitForSingleObject(h, 0) == 258 and
    creation == prepared.owner["job_root_creation_time"] and prepared.process.pid in prepared.members(),
    "actual-suspended-new-finalizer-parent")
  index = reference(plan["phase_index_ref"], 1024 * 1024)
  records = previous.parent_records
  authority=reference(plan["authority_ref"],4*1024**2)
  shape(authority,"schema run source_pin budget_sha256 parent_case_id census output limits caps guard_bounds_ref outer_guard_bounds_ref source_freeze","finalizer-allocation")
  catalogue=reference(plan["profile_catalogue_ref"],4*1024**2)
  shape(catalogue,"version schedule_sha256 profiles","profile-catalogue")
  validate_grant_source_freeze(authority,previous)
  inner_budget=budget_reference(authority["guard_bounds_ref"])
  outer_budget=budget_reference(authority["outer_guard_bounds_ref"])
  outer_ref=authority["outer_guard_bounds_ref"]
  need(path(outer_ref["path"])==path(previous.c["budget"]["path"]) and
       outer_ref["bytes"]==previous.c["budget"]["bytes"] and
       outer_ref["sha256"]==previous.budget.raw_hash==previous.result["budget_sha256"] and
       digest(native._encode(outer_budget["evidence_bounds"],65536))==
       previous.bounds.sha==previous.result["bounds_sha256"],"actual-old-outer-budget-bounds")
  bounds=inner_budget["evidence_bounds"]
  returned=records["completion"]["original_returned_completion"]
  need(authority["schema"]=="finalizer-allocation-v1" and
       authority["run"]==records["lifetime"]["run_nonce"] and
       authority["source_pin"]==plan["expected_source_pin"] and
       authority["budget_sha256"]==returned["budget_sha256"] and
       authority["guard_bounds_ref"]["sha256"]==returned["budget_sha256"] and
       digest(native._encode(bounds,65536))==returned["bounds_sha256"] and
       catalogue["version"]==3 and type(catalogue["profiles"]) is dict and
       1<=len(catalogue["profiles"])<=4096,"actual-authority-profile-bounds")

  need(index["original_run_nonce"] == records["lifetime"]["run_nonce"] and
    index["source_pin"] == plan["expected_source_pin"] and len(index["phase_refs"]) == 16,
    "actual-complete-final16-index")
  issued = counter()
  need(records["lifetime"]["job_close"]["completed_perf_ns"] < issued <
    plan["deadline_perf_ns"] <= issued + int(max(0, prepared.c["absolute_deadline"]-time.time())*1e9),
    "grant-after-old-lifetime-before-fixed-deadline")
  need(plan["expected_new_guard_sha256"] == prepared.c["sources"]["guard"]["sha256"], "new-guard-binding")
  grant = dict(schema="postexit-finalizer-grant-v2-r3", parent_released=True,
    original_run_nonce=records["lifetime"]["run_nonce"],
    original_guard_sha256=records["lifetime"]["guard_sha256"],
    original_guard_receipt_ref=records["original_receipt_ref"],
    original_completion_ref=records["completion_ref"], original_native_lifetime_ref=records["lifetime_ref"],
    phase_index_ref=plan["phase_index_ref"],authority_ref=plan["authority_ref"],
    profile_catalogue_ref=plan["profile_catalogue_ref"],
    expected_finalizer_parent=dict(pid=prepared.process.pid, creation=creation,
      image=prepared.root_image, image_sha256=prepared.c["python"]["sha256"]),
    issued_perf_ns=issued, deadline_perf_ns=plan["deadline_perf_ns"],
    **{k:plan[k] for k in ("expected_source_pin","expected_binary_sha256","expected_runtime_sha256",
    "expected_profile_sha256","expected_finalizer_sha256","expected_new_guard_sha256")})
  release = path(prepared.c["release"]["path"], True)
  prepared.budget.check(prepared.g.reserves(prepared.budget, [(target, 65536, 1), (release, 8192, 1)]))
  native.write_json(target, grant, 65536)
  ref = file_ref(target, 65536)
  native.write_json(release, dict(schema="bounded-parent-release-v1", nonce=prepared.c["run_nonce"],
    child=dict(pid=prepared.process.pid, creation=creation), publisher=prepared.publisher,
    source_sha256=prepared.c["sources"]["capture"]["sha256"], grant_ref=ref,
    issued_perf_ns=counter()), 8192)
  return ref


def publish_controls_records(cap):
  need(cap.status=="closed","closed-native-capture")
  review=reference(cap.c["source_review"],65536)
  creator=cap.creator
  native_ssh=None
  if cap.c["mode"]=="native-ssh-phase":
    native_ssh=dict(cap.c["_native_ssh"],observed_native_root_image=cap.root_image,
      actual_root_exit=cap.root_exit,windows_transport_only=True,
      native_children_use_separate_Linux_guard=True,verify_phase_terminal_required=True,
      stdout_ref=file_ref(cap.receipt.with_suffix(".stdout"),20480),
      stderr_ref=file_ref(cap.receipt.with_suffix(".stderr"),
        cap.bounds.b["stdio"]["stderr"]["capture_bytes"]+4096))
  audit=cap.publish("audit",dict(guard_sha256=cap.c["sources"]["capture"]["sha256"],
    no_job_handle_duplication=review["no_job_handle_duplication"],
    no_job_handle_inheritance=review["no_job_handle_inheritance"],
    complete_cooperating_owner_universe=[dict(pid=creator["pid"],creation=creator["creation_time"])],
    source_review_ref=cap.c["source_review"]))
  lifetime=dict(schema="outer-controller-native-lifetime-v1",run_nonce=cap.c["run_nonce"],
    guard_sha256=cap.c["sources"]["capture"]["sha256"],creator=creator,
    source_audit_ref=audit,host=cap.host,empty_observations=cap.empties,job_close=cap.job_close)
  life=cap.publish("lifetime",lifetime)
  done=cap.publish("completion",dict(schema="bounded-outer-caller-completion-v1",mode=cap.c["mode"],native_ssh=native_ssh,
    budget_sha256=cap.budget.raw_hash,bounds_sha256=cap.bounds.sha,config_ref=cap.config_ref,
    qualified=cap.result["qualified"],process_closure_verified=cap.result["process_closure_verified"],
    native_root_exit=cap.root_exit,outer_lifetime_ref=life,receipt_ref=cap.result["receipt_ref"],
    actual_postclose_census=cap.budget.check()))
  census=cap.publish("census",dict(observed_before_census_record=cap.budget.check()))
  cap.publisher_refs=dict(audit=audit,lifetime=life,completion=done,census=census)
  return cap.publisher_refs

def publish_postexit_grant(previous,prepared,plan):
  try:
    return _issue_postexit_grant(previous,prepared,plan)
  except BaseException as error:
    if type(prepared) is Capture:
      prepared.fail(error,"postexit-grant-refused")
      prepared.finish()
    raise

def worker(config_path, expected):
  c = load_config(config_path, expected)
  need(c["mode"] == "guarded-bootstrap", "only-fixed-guard-worker")
  g = guard_module(c)
  c["_watchdog"].worker_guard=g
  if c["release"] is not None:
    release = path(c["release"]["path"], True)
    while not release.exists():
      need(time.time() < c["absolute_deadline"], "postexit-parent-release-deadline")
      time.sleep(.005)
    r = strict(read(release, 8192))
    shape(r, "schema nonce child publisher source_sha256 grant_ref issued_perf_ns", "actual-parent-release")
    need(r["schema"] == "bounded-parent-release-v1" and r["nonce"] == c["run_nonce"] and
      r["child"] == dict(pid=os.getpid(), creation=g._stamp(g.K.GetCurrentProcess())) and
      r["source_sha256"] == c["sources"]["capture"]["sha256"], "release-owned-child")
    h = g.K.OpenProcess(0x1000 | 0x100000, False, os.getppid())
    need(bool(h), "actual-release-parent")
    try:
      need(g.K.GetProcessId(h) == r["publisher"]["pid"] == os.getppid() and
        g._stamp(h) == r["publisher"]["creation"] and
        g.K.WaitForSingleObject(h, 0) == 258 and
        g.native_image(h)==r["publisher"]["image"] and
        r["publisher"]["image_sha256"]==c["python"]["sha256"], "same-live-parent-native-identity")
    finally:
      g._checked_close(h, "release-parent")
    grant = reference(r["grant_ref"], 65536)
    need(grant["expected_finalizer_parent"]["pid"] == os.getpid() and
      grant["expected_finalizer_parent"]["creation"] == r["child"]["creation"] and
      grant["expected_new_guard_sha256"] == c["sources"]["guard"]["sha256"],
      "postexit-grant-for-actual-worker")
    os.environ["FILE_NATIVE_POSTEXIT_GRANT_PATH"] = r["grant_ref"]["path"]
    os.environ["FILE_NATIVE_POSTEXIT_GRANT_SHA256"] = r["grant_ref"]["sha256"]
  argv = [c["python"]["path"], "-I", "-S", "-B", "-u", c["sources"]["bootstrap"]["path"],
      "--manifest", c["manifest"]["path"], "--manifest-sha256", c["manifest"]["sha256"]]
  if c["invocation"] is not None:
    argv += ["--invocation", c["invocation"]]
  budget = g.Budget(c["inner"]["budget"]["path"])
  g.check_census_envelope(budget)
  target = path(c["inner"]["completion"]["path"], True)
  row = g.run(argv, c["inner"]["receipt"], c["inner"]["budget"]["path"],
        min(3600, c["absolute_deadline"]-time.time()),
        reserves=g.reserves(budget, [(target, 65536, 1)]))
  body = dict(schema="bounded-inner-guard-return-v1", completion=row["completion"],
    receipt_ref=file_ref(path(c["inner"]["receipt"]), budget.config["evidence_bounds"]["final"]["json_bytes"]),
    owner=row["owner"], native_host=row["native_host"], creator_lifetime=row["creator_lifetime"],
    closure_checks=row["closure_checks"], job_close=row["job_close"])
  budget.check(g.reserves(budget, [(target, 65536, 1)]))
  g.write_json(target, body, 65536)
  final = dict(schema=API, worker_completion_ref=file_ref(target, 65536),
        actual_census=budget.check(), qualified=row["qualified"])
  g._write_all(sys.stdout.buffer,g._encode(final,8192))
  sys.stdout.buffer.flush()
  need(row["process_closure_verified"] is True and not g._UNCLOSED,"worker-resource-fence")
  c["_watchdog"].finish()
  return 0 if row["qualified"] else 2

def main():
  whole_watchdog()
  if len(sys.argv) == 4 and sys.argv[1] == "--worker":
    raise SystemExit(worker(sys.argv[2], sys.argv[3]))
  need(len(sys.argv) == 3, "capture-config-and-hash")
  cap = Capture(load_config(sys.argv[1], sys.argv[2]))
  result = cap.start()
  if cap.c["mode"] == "guarded-bootstrap" and result["qualified"]:
    result["parent_records"] = publish_parent_records(cap)
  elif cap.status=="closed":
    publish_controls_records(cap)
  result["final_inclusive_census"] = cap.budget.check() if cap.status=="closed" else None
  cap.g._write_all(sys.stdout.buffer,cap.g._encode(dict(qualified=result["qualified"],
    process_closure_verified=result["process_closure_verified"],receipt_ref=result["receipt_ref"],
    publisher_refs=getattr(cap,"publisher_refs",{}),final_inclusive_census=result["final_inclusive_census"]),8192))
  sys.stdout.buffer.flush()
  cap.watchdog.finish()
  raise SystemExit(0 if result["qualified"] else 2)

if __name__ == "__main__":
  main()
