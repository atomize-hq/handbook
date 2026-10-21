import argparse
import contextlib
import ctypes
from ctypes import wintypes as w
import datetime as dt
import hashlib
import binascii
import math
import json
import msvcrt
import os
from pathlib import Path
import shutil
import subprocess
import sys
import threading
import time
import uuid
walk_tree = os.walk
HANDLE = w.HANDLE
DWORD = w.DWORD
qpc = time.perf_counter_ns
epoch = time.time
ref = ctypes.byref
winerr = ctypes.WinError
last_error = ctypes.get_last_error
ptr = ctypes.POINTER
size_of = ctypes.sizeof
sha256 = hashlib.sha256
FUNCTIONAL = dt.datetime.fromisoformat('2026-10-21T11:10:14+00:00').timestamp()
TOKEN = Path('C:/file-native-online-control-20261020/windows-heavy.lock')
REGISTRY = TOKEN.with_name('supervisors.json')
OWNER = TOKEN.with_name('current-owner.json')
REGISTRY_LOCK = TOKEN.with_name('supervisors.lock')
GiB = 1024 ** 3
K = ctypes.WinDLL('kernel32', use_last_error=True)
PS = ctypes.WinDLL('psapi', use_last_error=True)
class Basic(ctypes.Structure):
  _fields_ = [('User',ctypes.c_int64),('Kernel',ctypes.c_int64),('Flags',DWORD),('Min',ctypes.c_size_t),('Max',ctypes.c_size_t),('Active',DWORD),('Affinity',ctypes.c_size_t),('Priority',DWORD),('Scheduling',DWORD)]
class IO(ctypes.Structure):
  _fields_ = [(x, ctypes.c_uint64) for x in ['ro', 'wo', 'oo', 'rb', 'wb', 'ob']]
class Limits(ctypes.Structure):
  _fields_ = [('Basic', Basic), ('IO', IO), ('Process', ctypes.c_size_t), ('Job', ctypes.c_size_t), ('PeakProcess', ctypes.c_size_t), ('PeakJob', ctypes.c_size_t)]
class Memory(ctypes.Structure):
  _fields_ = [('cb', DWORD), ('faults', DWORD)] + [(x, ctypes.c_size_t) for x in ['peak', 'rss', 'peakpage', 'page', 'peaknon', 'non', 'pagefile', 'peakfile']]
K.GetCurrentProcess.restype=HANDLE
K.GetHandleInformation.argtypes=[HANDLE,ptr(DWORD)]
K.CreateJobObjectW.restype = HANDLE
K.CreateJobObjectW.argtypes = [ctypes.c_void_p, w.LPCWSTR]
K.SetInformationJobObject.argtypes = [HANDLE, ctypes.c_int, ctypes.c_void_p, DWORD]
K.AssignProcessToJobObject.argtypes = [HANDLE, HANDLE]
K.TerminateJobObject.argtypes = [HANDLE, w.UINT]
K.QueryInformationJobObject.argtypes = [HANDLE, ctypes.c_int, ctypes.c_void_p, DWORD, ctypes.c_void_p]
K.CloseHandle.argtypes = [HANDLE]
K.OpenProcess.restype = HANDLE
K.OpenProcess.argtypes = [DWORD, w.BOOL, DWORD]
K.GetProcessTimes.argtypes = [HANDLE, ptr(w.FILETIME), ptr(w.FILETIME), ptr(w.FILETIME), ptr(w.FILETIME)]
PS.GetProcessMemoryInfo.argtypes = [HANDLE, ptr(Memory), DWORD]
NT = ctypes.WinDLL('ntdll')
NT.NtResumeProcess.argtypes = [HANDLE]
def sha_file(path):
  digest = sha256()
  with Path(path).open('rb') as source:
    for block in iter(lambda: source.read(1024 * 1024), b''):
      digest.update(block)
  return digest.hexdigest()
def census(root, stop=None):
  root = Path(root)
  count = total = 0
  try:
    info = root.lstat()
  except FileNotFoundError:
    return dict(bytes=0, files=0)
  if not root.is_dir():raise ValueError('root')
  if root.is_symlink() or getattr(info, 'st_file_attributes', 0) & 1024:
    raise ValueError('census-root-link')
  def fail(error):
    raise error
  for directory, dirs, files in walk_tree(root, followlinks=False, onerror=fail):
    for name in dirs + files:
      path = Path(directory) / name
      if path.is_symlink() or getattr(path.lstat(), 'st_file_attributes', 0) & 1024:
        raise ValueError('census-link')
    for name in files:
      if stop is not None and stop.is_set():
        raise InterruptedError('census-stopped')
      try:
        total += (Path(directory) / name).stat().st_size
        count += 1
      except FileNotFoundError:
        pass
  return dict(bytes=total, files=count)
class Budget:
  def __init__(self, path):
    self.path = Path(path)
    self.config, raw = _strict_read(path, 131072)
    self.raw_hash = sha256(raw).hexdigest()
    assert not self.config.get('template_not_execution_admission'), 'unfilled-budget-template'
    assert self.config['admission_ledger_sha256'] == '2ff4e277a44c3dea99f53d5b098250e78a50c40d63539022bf365f4cc048c82c'
    self.categories = self.config['categories']
    if not 1 <= len(self.categories) <= 32 or len(self.config.get('groups', {})) > 16:
      raise ValueError('finite-budget-shape')
    for name, rule in self.categories.items():
      if not isinstance(name, str) or len(name) > 64 or type(rule.get('root')) is not str or (len(rule['root']) > 2048):
        raise ValueError('bounded-budget-path')
      _number(rule['bytes'], 0)
      _number(rule['files'], 0)
    assert all((Path(v['root']).is_absolute() for v in self.categories.values()))
    roots = {name: Path(v['root']).resolve() for name, v in self.categories.items()}
    for name, path in roots.items():
      for other, parent in roots.items():
        if name != other and path.is_relative_to(parent):
          assert self.categories[name].get('nonadditive_subset_of') == other, 'undeclared-overlapping-roots'
    for name, rule in self.categories.items():
      assert rule.get('charge_mode', 'absolute') in ['absolute', 'positive_delta']
      if rule.get('charge_mode') == 'positive_delta':
        assert type(rule['baseline_bytes']) is int and rule['baseline_bytes'] >= 0
        assert type(rule['baseline_files']) is int and rule['baseline_files'] >= 0
  def check(self, reserves=None, stop=None):
    reserves = reserves or {}
    assert set(reserves) <= set(self.categories)
    rows = {}
    for name, rule in self.categories.items():
      actual = census(rule['root'], stop)
      reserve = reserves.get(name, {})
      b, f = (reserve.get('bytes', 0), reserve.get('files', 0))
      assert type(b) is int and b >= 0 and (type(f) is int) and (f >= 0)
      charged = dict(actual)
      if rule.get('charge_mode') == 'positive_delta':
        charged = dict(bytes=max(0, actual['bytes'] - rule['baseline_bytes']), files=max(0, actual['files'] - rule['baseline_files']))
      rows[name] = dict(**charged,absolute_actual=actual,reserve_bytes=b,reserve_files=f,charge_mode=rule.get('charge_mode','absolute'),additive_charge=not bool(rule.get('nonadditive_subset_of')),cap_bytes=rule['bytes'],cap_files=rule['files'])
      if charged['bytes'] + b > rule['bytes'] or charged['files'] + f > rule['files']:
        raise ValueError('prospective-category-cap:' + name + ':' + json.dumps(rows[name]))
    group_rows = {}
    for name, group in self.config.get('groups', {}).items():
      assert set(group['categories']) <= set(rows)
      assert all((rows[k]['additive_charge'] for k in group['categories'])), 'subset-counted-twice'
      total_b = sum((rows[k]['bytes'] + rows[k]['reserve_bytes'] for k in group['categories']))
      total_f = sum((rows[k]['files'] + rows[k]['reserve_files'] for k in group['categories']))
      group_rows[name] = dict(bytes=total_b, files=total_f, cap_bytes=group['bytes'], cap_files=group['files'])
      if total_b > group['bytes'] or total_f > group['files']:
        raise ValueError('prospective-shared-group-cap:' + name + ':' + json.dumps(group_rows[name]))
    free = shutil.disk_usage('C:/').free
    if free < 140 * GiB:
      raise ValueError('windows-free-floor:' + str(free))
    return dict(categories=rows, shared_groups=group_rows, free_bytes=free, epoch=epoch(), qualification='logical bytes; no planned deletion credit; parent owns cross-host ledger')
def creation_time(pid):
  handle = K.OpenProcess(4096, False, pid)
  if not handle:
    error = last_error()
    if error in (87, 1168):
      return None
    raise winerr(error)
  creation = None
  try:
    creation = _stamp(handle)
    return creation
  finally:
    _checked_close(handle, 'creation-time', pid, creation)
def control_json(path, row):
  data = _encode(row, 8192)
  assert len(data) <= 8192, 'control-record-cap'
  temp = path.with_suffix('.tmp')
  with temp.open('wb') as output:
    _write_all(output, data)
    output.flush()
    os.fsync(output.fileno())
  for attempt in range(20):
    try:
      os.replace(temp, path)
      break
    except PermissionError:
      if attempt == 19:
        raise
      time.sleep(0.01)
@contextlib.contextmanager
def registry_lock():
  try:
    with REGISTRY_LOCK.open('xb') as initial:
      initial.write(b'1')
      initial.flush()
      os.fsync(initial.fileno())
  except FileExistsError:
    pass
  handle = REGISTRY_LOCK.open('r+b')
  locked = False
  try:
    while epoch() < FUNCTIONAL:
      if os.fstat(handle.fileno()).st_size != 1:
        time.sleep(0.01)
        continue
      try:
        handle.seek(0)
        msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 1)
        locked = True
        break
      except OSError:
        time.sleep(0.01)
    assert locked, 'registry-lock-cutoff'
    yield
  finally:
    if locked:
      handle.seek(0)
      msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 1)
    handle.close()
def registry_rows():
  if not REGISTRY.exists():
    return []
  with REGISTRY.open('rb') as source:
    raw = source.read(8193)
  assert len(raw) <= 8192, 'supervisor-registry-byte-cap'
  rows = _strict_read(REGISTRY, 8192)[0]
  assert isinstance(rows, list) and len(rows) <= 16, 'supervisor-registry-cap'
  for row in rows:
    assert set(row) == {'pid', 'creation_time', 'registered_by', 'role'}
  return rows
def supervisors(extra_pids=()):
  with registry_lock():
    live = {}
    for row in registry_rows():
      actual = creation_time(row['pid'])
      if actual is not None and actual == row['creation_time']:
        live[row['pid']] = row
    for pid in {os.getpid(), os.getppid(), *extra_pids}:
      started = creation_time(pid)
      assert started is not None, 'supervisor-owner-vanished'
      live[pid] = dict(pid=pid, creation_time=started, registered_by=os.getpid(), role='guard-or-explicit-new-work-supervisor')
    assert len(live) <= 16, 'supervisor-registry-cap'
    control_json(REGISTRY, list(live.values()))
    return list(live.values())
def members(job):
  buffer = ctypes.create_string_buffer(8 + size_of(ctypes.c_size_t) * 2048)
  if not K.QueryInformationJobObject(job, 3, buffer, len(buffer), None):
    raise winerr(last_error())
  assigned = int.from_bytes(buffer.raw[:4], 'little')
  count = int.from_bytes(buffer.raw[4:8], 'little')
  if assigned > 2048 or count > 2048:
    raise ValueError('job-member-table-overflow')
  size = size_of(ctypes.c_size_t)
  return [int.from_bytes(buffer.raw[8 + i * size:8 + (i + 1) * size], 'little') for i in range(count)]
def _strict_read(path, limit):
  with Path(path).open('rb') as stream:
    before = os.fstat(stream.fileno())
    data = stream.read(limit + 1)
    after = os.fstat(stream.fileno())
  if len(data) > limit or before.st_size != len(data) or (before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns) != (after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns):
    raise ValueError('bounded-stable-JSON')
  def pairs(items):
    value = {}
    for key, item in items:
      if key in value:
        raise ValueError('duplicate-JSON-key')
      value[key] = item
    return value
  def invalid(value):
    raise ValueError('nonfinite-JSON')
  return (json.loads(data.decode('utf-8'), object_pairs_hook=pairs, parse_constant=invalid), data)
def write_json(path, row, limit=8192):
  data = _encode(row, limit)
  if not Path(path).parent.is_dir():
    raise ValueError('preexisting-receipt-directory')
  with Path(path).open('xb', buffering=0) as output:
    _write_all(output, data)
    output.flush()
    os.fsync(output.fileno())
def supervisor_rss(rows=None):
  total = 0
  samples = []
  for row in registry_rows() if rows is None else rows:
    value, race = _super_memory(row['pid'], row['creation_time'])
    if race:
      samples.append(dict(pid=row['pid'], creation_time=row['creation_time'], state=race))
      continue
    total += value[0]
    samples.append(dict(pid=row['pid'], creation_time=row['creation_time'], rss=value[0], peak=value[1]))
  if total > 768 * 1024 ** 2:
    raise RuntimeError('combined-supervisor-rss-censor')
  return (total, samples)
class EvidenceCap(RuntimeError):
  pass
def _encode(value, limit):
  parts = []
  size = 1
  for part in json.JSONEncoder(ensure_ascii=True, allow_nan=False, separators=(',', ':')).iterencode(value):
    data = part.encode('utf-8')
    size += len(data)
    if size > limit:
      raise EvidenceCap('encoded-byte-cap')
    parts.append(data)
  return b''.join(parts) + b'\n'
def _error(error):
  name = type(error).__name__[:64]
  number = getattr(error, 'winerror', None) or getattr(error, 'errno', None)
  return name + (':' + str(number) if type(number) is int else '')
def _shape(value, keys):
  if type(value) is not dict or set(value) != set(keys.split()):
    raise ValueError('closed-evidence-bounds')
def _number(value, minimum=1):
  if type(value) is not int or not minimum <= value <= 2 ** 63 - 1:
    raise ValueError('finite-positive-cap')
  return value
class Bounds:
  def __init__(self, budget, receipt, argv, stdin, cwd):
    b = budget.config.get('evidence_bounds')
    _shape(b, 'version receipt_prefix invocation rss collections stdio final aggregate')
    if type(b['version']) is not int or b['version'] != 1:
      raise ValueError('bounds-version')
    prefix = receipt.with_suffix('')
    if receipt.suffix != '.json' or not receipt.is_absolute() or receipt.resolve() != receipt or (str(prefix) != b['receipt_prefix']):
      raise ValueError('exact-canonical-receipt-prefix')
    _shape(b['invocation'], 'argv_bytes argv_items stdin_bytes cwd_bytes')
    _shape(b['rss'], 'complete_rows total_rows row_bytes raw_bytes gzip_bytes finish_reserve_bytes')
    _shape(b['collections'], 'disk supervisor exit identity peaks errors terminations')
    for rule in b['collections'].values():
      _shape(rule, 'rows bytes')
    _shape(b['stdio'], 'stdout stderr read_chunk_bytes')
    for name in ('stdout', 'stderr'):
      _shape(b['stdio'][name], 'capture_bytes overflow_tail_bytes')
    _shape(b['final'], 'json_bytes emergency_bytes completion_bytes')
    _shape(b['aggregate'], 'retained_bytes retained_files')
    def numbers(value):
      for item in value.values():
        if type(item) is dict:
          numbers(item)
        else:
          _number(item)
    for name in ('invocation', 'rss', 'collections', 'stdio', 'final', 'aggregate'):
      numbers(b[name])
    if b['rss']['finish_reserve_bytes'] < 65536 or b['rss']['gzip_bytes'] < 65559 or b['rss']['total_rows'] < b['rss']['complete_rows'] or (b['stdio']['read_chunk_bytes'] > 4096) or any((b['stdio'][k]['overflow_tail_bytes'] < b['stdio']['read_chunk_bytes'] for k in ('stdout','stderr'))) or (b['final']['json_bytes'] < 8192) or (b['final']['emergency_bytes'] < 4096) or (not 8192 <= b['final']['completion_bytes'] <= 65536):
      raise ValueError('bounds-safety-reserves')
    inv = b['invocation']
    if type(argv) not in (list, tuple) or len(argv) > inv['argv_items'] or (not argv) or any((type(x) is not str for x in argv)):
      raise ValueError('bounded-argv')
    self.argv = _encode(list(argv), inv['argv_bytes'])
    if stdin is not None and (type(stdin) is not bytes or len(stdin) > inv['stdin_bytes']):
      raise ValueError('bounded-stdin')
    if len(str(cwd or '').encode('utf-8')) > inv['cwd_bytes']:
      raise ValueError('bounded-cwd')
    self.b = b
    self.raw = _encode(b, 65536)
    self.sha = sha256(self.raw).hexdigest()
    self.files = [receipt.with_suffix(s) for s in ('.json', '.rss.jsonl.gz', '.stdout', '.stderr', '.bounds.json')]
    if not receipt.parent.is_dir() or any((os.path.lexists(p) for p in self.files)):
      raise ValueError('fresh-owned-prefix')
    total = b['rss']['gzip_bytes'] + b['final']['json_bytes'] + b['final']['emergency_bytes'] + sum((b['stdio'][x]['capture_bytes'] + b['stdio'][x]['overflow_tail_bytes'] for x in ('stdout','stderr')))
    if total > b['aggregate']['retained_bytes'] or b['aggregate']['retained_files'] != 5:
      raise ValueError('aggregate-evidence-envelope')
    self.reserved = total + b['final']['completion_bytes']
    self.shared_bytes = 4 * 8192
    self.shared_files = 4
    self.budget = budget
    fixed = len(_encode(budget.check(), 131072)) + len(self.argv) + 131072
    collections = sum((x['bytes'] + 2 for x in b['collections'].values()))
    if fixed + collections + b['collections']['errors']['bytes'] > b['final']['json_bytes']:
      raise ValueError('prospective-final-json-envelope')
  def reserves(self, supplied):
    result = {k: dict(v) for k, v in (supplied or {}).items()}
    covered = shared = False
    for name, rule in self.budget.categories.items():
      root = Path(rule['root']).resolve()
      extra_b = extra_f = 0
      if self.files[0].is_relative_to(root):
        covered = True
        extra_b += self.reserved
        extra_f += 6
      if all((p.is_relative_to(root) for p in (OWNER, REGISTRY))):
        shared = True
        extra_b += self.shared_bytes
        extra_f += self.shared_files
      if extra_b:
        row = result.setdefault(name, {})
        row['bytes'] = row.get('bytes', 0) + extra_b
        row['files'] = row.get('files', 0) + extra_f
    if not covered or not shared:
      raise ValueError('admitted-evidence-and-shared-control-roots')
    return result
class EvidenceState:
  def __init__(self, bounds):
    self.bounds = bounds
    self.lock = threading.RLock()
    self.used = {k: [0, 0] for k in bounds.b['collections']}
    self.suppressed = {k: 0 for k in self.used}
    self.frozen = False
    self.cap_reason = None
    self.terminate = None
    self.raw = None
    self.first_error = None
    self.run_nonce = uuid.uuid4().hex
    self.uncertain = False
  def cap(self, name):
    with self.lock:
      if self.cap_reason is None:
        self.cap_reason = name
      self.frozen = True
    if self.terminate:
      self.terminate('evidence-cap:' + name)
  def charge(self, name, rows, size):
    with self.lock:
      if self.frozen:
        self.suppressed[name] = min(2 ** 63 - 1, self.suppressed[name] + 1)
        return False
      used = self.used[name]
      limit = self.bounds.b['collections'][name]
      if used[0] + rows <= limit['rows'] and used[1] + size <= limit['bytes']:
        used[0] += rows
        used[1] += size
        return True
    self.cap('collection-' + name)
    return False
  def log(self, event, value):
    if self.raw and (not self.frozen):
      self.raw.record(dict(event=event, sample_complete=False, value=value))
class BoundedList(list):
  def __init__(self, state, name):
    super().__init__()
    self.state = state
    self.name = name
  def append(self, value):
    try:
      size = len(_encode(value, self.state.bounds.b['collections'][self.name]['bytes'])) + 1
    except EvidenceCap:
      self.state.cap('collection-' + self.name)
      return
    if self.state.charge(self.name, 1, size):
      super().append(value)
class BoundedMap(dict):
  def __init__(self, state, name):
    super().__init__()
    self.state = state
    self.name = name
    self.sizes = {}
  def __setitem__(self, key, value):
    try:
      size = len(_encode({key: value}, self.state.bounds.b['collections'][self.name]['bytes'])) + 1
    except EvidenceCap:
      self.state.cap('collection-' + self.name)
      return
    delta = max(0, size - self.sizes.get(key, 0))
    if self.state.charge(self.name, int(key not in self), delta):
      self.sizes[key] = max(size, self.sizes.get(key, 0))
      super().__setitem__(key, value)
class StoredGzip:
  def __init__(self, path, state):
    self.state = state
    self.lock = threading.RLock()
    self.file = Path(path).open('xb', buffering=0)
    self.written = 0
    self.rows = 0
    self.complete_rows = 0
    self.raw_bytes = 0
    self.raw_hash = sha256()
    self.gz_hash = sha256()
    self.crc = 0
    self.valid = True
    self.closed = False
    self.complete = False
    self._write(b'\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\xff')
  def _write(self, data):
    view = memoryview(data)
    while view:
      n = self.file.write(view)
      if not n:
        raise OSError('short-owned-evidence-write')
      self.gz_hash.update(view[:n])
      self.written += n
      view = view[n:]
  def record(self, row, flush=False):
    with self.lock:
      if self.closed or self.state.frozen:
        return False
      b = self.state.bounds.b['rss']
      row = dict(row, run_nonce=self.state.run_nonce)
      try:
        data = _encode(row, b['row_bytes'])
      except EvidenceCap:
        self.state.cap('rss-row-bytes')
        return False
      full = row.get('sample_complete') is True
      cost = len(data) + 5 * ((len(data) + 65534) // 65535)
      why = 'rss-complete-rows' if full and self.complete_rows >= b['complete_rows'] else 'rss-total-rows' if self.rows >= b['total_rows'] else 'rss-raw-bytes' if self.raw_bytes + len(data) > b['raw_bytes'] else 'rss-gzip-bytes' if self.written + cost + b['finish_reserve_bytes'] > b['gzip_bytes'] else None
      if why:
        self.state.cap(why)
        return False
      try:
        for offset in range(0, len(data), 65535):
          part = data[offset:offset + 65535]
          n = len(part)
          self._write(b'\x00' + n.to_bytes(2, 'little') + (n ^ 65535).to_bytes(2, 'little'))
          self._write(part)
        self.file.flush()
        self.crc = binascii.crc32(data, self.crc)
        self.raw_hash.update(data)
        self.raw_bytes += len(data)
        self.rows += 1
        self.complete_rows += int(full)
        return True
      except BaseException:
        self.valid = False
        raise
  def close(self):
    with self.lock:
      if self.closed:
        return
      try:
        if self.valid:
          self._write(b'\x01\x00\x00\xff\xff' + (self.crc & 4294967295).to_bytes(4, 'little') + (self.raw_bytes & 4294967295).to_bytes(4, 'little'))
          self.file.flush()
          os.fsync(self.file.fileno())
          self.complete = True
      except BaseException:
        self.valid = False
        raise
      finally:
        self.closed = True
        self.file.close()
  def reference(self):
    return dict(bytes=self.written,raw_bytes=self.raw_bytes,rows=self.rows,complete_rows=self.complete_rows,raw_sha256=self.raw_hash.hexdigest(),gzip_sha256=self.gz_hash.hexdigest(),gzip_complete=self.complete,codec='gzip-stored-deflate-v1',evidence_complete=not self.state.frozen)
K.GetProcessId.argtypes = [HANDLE]
K.GetProcessId.restype = DWORD
K.WaitForSingleObject.argtypes = [HANDLE, DWORD]
K.WaitForSingleObject.restype = DWORD
K.IsProcessInJob.argtypes = [HANDLE, HANDLE, ptr(w.BOOL)]
K.IsProcessInJob.restype = w.BOOL
K.PeekNamedPipe.argtypes = [HANDLE, ctypes.c_void_p, DWORD, ctypes.c_void_p, ptr(DWORD), ctypes.c_void_p]
K.PeekNamedPipe.restype = w.BOOL
_UNCLOSED = []
def _stamp(handle):
  values = [w.FILETIME() for _ in range(4)]
  if not K.GetProcessTimes(handle, *(ref(v) for v in values)):
    raise winerr(last_error())
  return values[0].dwHighDateTime * 2 ** 32 + values[0].dwLowDateTime
def _memory_handle(handle):
  memory = Memory()
  memory.cb = size_of(memory)
  if not PS.GetProcessMemoryInfo(handle, ref(memory), memory.cb):
    return None
  return (memory.rss, memory.peak)
def _super_memory(pid, expected):
  handle = K.OpenProcess(1053712, False, pid)
  if not handle:
    after = creation_time(pid)
    if after is None or after != expected:
      return (None, 'ended-before-open-or-reused')
    raise RuntimeError('unreadable-live-supervisor-handle')
  try:
    if K.GetProcessId(handle) != pid or _stamp(handle) != expected:
      return (None, 'ended-or-reused')
    value = _memory_handle(handle)
    if value is None:
      if K.WaitForSingleObject(handle, 0) == 0:
        return (None, 'exited-during-rss')
      raise RuntimeError('unreadable-live-supervisor-rss')
    return (value, None)
  finally:
    _checked_close(handle, 'supervisor', pid, expected)
class JobSampling:
  def __init__(self, state, job, current_members):
    self.state = state
    self.job = job
    self.current_members = current_members
    self.live = {}
    self.departed = {}
    self.identities = BoundedList(state, 'identity')
    self.identity_peaks = BoundedMap(state, 'peaks')
  def memory(self, pid):
    self.departed.clear()
    entry = self.live.get(pid)
    if entry is None:
      h = K.OpenProcess(1053712, False, pid)
      if not h:
        if pid not in self.current_members():
          self.departed[pid] = 'departed-before-open'
          return None
        raise RuntimeError('unreadable-live-owned-handle')
      creation = None
      try:
        creation = _stamp(h)
        inside = w.BOOL()
        if not K.IsProcessInJob(h, self.job, ref(inside)):
          raise winerr(last_error())
        if not inside.value:
          if pid not in self.current_members():
            self.departed[pid] = 'departed-before-identity-or-reused'
            return None
          raise RuntimeError('owned-member-identity-mismatch')
        if K.GetProcessId(h) != pid:
          raise RuntimeError('owned-HANDLE-PID')
        self.identities.append(dict(pid=pid, creation_time=creation))
        if self.state.frozen:
          return None
        entry = (h, creation)
        self.live[pid] = entry
        h = None
      finally:
        if h:
          _checked_close(h, 'job-transient', pid, creation)
    h, creation = entry
    if K.GetProcessId(h) != pid or _stamp(h) != creation:
      raise RuntimeError('retained-owned-identity')
    inside = w.BOOL()
    if not K.IsProcessInJob(h, self.job, ref(inside)) or not inside.value:
      raise RuntimeError('retained-owned-Job-membership')
    if K.WaitForSingleObject(h, 0) == 0:
      self.departed[pid] = {'state': 'departed-before-rss', 'creation_time': creation}
      self.retire(pid)
      return None
    value = _memory_handle(h)
    if value is None:
      if K.WaitForSingleObject(h, 0) == 0:
        self.departed[pid] = {'state': 'departed-during-rss', 'creation_time': creation}
        self.retire(pid)
        return None
      raise RuntimeError('unreadable-live-owned-rss')
    key = str(pid) + ':' + str(creation)
    self.identity_peaks[key] = max(self.identity_peaks.get(key, 0), value[1])
    return value
  def retire(self, pid):
    entry = self.live.get(pid)
    if entry:
      _checked_close(entry[0], 'owned-identity', pid, entry[1])
      self.live.pop(pid)
  def sweep(self, pids):
    for pid in list(self.live):
      if pid not in pids:
        h = self.live[pid][0]
        if K.WaitForSingleObject(h, 0) != 0:
          raise RuntimeError('live-handle-absent-from-owned-Job')
        self.retire(pid)
  def close(self):
    for pid in list(self.live):
      self.retire(pid)
class PipeCapture:
  def __init__(self, process, receipt, state):
    self.process = process
    self.state = state
    self.stop = threading.Event()
    self.rows = {}
    self.thread = None
    self.closed = False
    try:
      for name in ('stdout', 'stderr'):
        self.rows[name] = dict(bytes=0, sha=sha256(), eof=False, truncated=False, closed=False, pipe=getattr(process, name), out=receipt.with_suffix('.' + name).open('xb', buffering=0))
    except BaseException:
      for row in self.rows.values():
        for obj in (row['out'], row['pipe']):
          try:
            obj.close()
          except BaseException:
            state.uncertain = True
      if state.uncertain:
        _UNCLOSED.append(self)
      raise
  def start(self):
    self.thread = threading.Thread(target=self.drain, name='owned-stdio', daemon=False)
    self.thread.start()
  def drain(self):
    try:
      while not all((v['eof'] or v['truncated'] for v in self.rows.values())):
        progress = False
        for name, row in self.rows.items():
          if row['eof'] or row['truncated']:
            continue
          limit = self.state.bounds.b['stdio'][name]
          available = DWORD()
          handle = HANDLE(msvcrt.get_osfhandle(row['pipe'].fileno()))
          ok = K.PeekNamedPipe(handle, None, 0, None, ref(available), None)
          if not ok:
            if last_error() == 109:
              row['eof'] = True
              continue
            if last_error() == 233:
              row['truncated'] = True
              self.state.cap('stdio-disconnected-233')
              raise RuntimeError('pipe-EOF-unknown-233')
            raise winerr(last_error())
          if not available.value:
            continue
          left = limit['capture_bytes'] + limit['overflow_tail_bytes'] - row['bytes']
          if left <= 0:
            row['truncated'] = True
            self.state.cap(name + '-tail')
            continue
          before = limit['capture_bytes'] - row['bytes']
          amount = min(available.value, self.state.bounds.b['stdio']['read_chunk_bytes'], left, before if before > 0 else 1 if row['bytes'] == limit['capture_bytes'] else left)
          data = os.read(row['pipe'].fileno(), amount)
          if not data:
            row['eof'] = True
            continue
          view = memoryview(data)
          while view:
            n = row['out'].write(view)
            if not n:
              raise OSError('stdio-short-write')
            row['sha'].update(view[:n])
            row['bytes'] += n
            view = view[n:]
          row['out'].flush()
          progress = True
          if row['bytes'] > limit['capture_bytes']:
            self.state.cap(name + '-bytes')
        if self.stop.is_set():
          for row in self.rows.values():
            if not row['eof']:
              row['truncated'] = True
          break
        if not progress:
          self.stop.wait(0.005)
    except BaseException as error:
      self.state.cap('stdio-io-' + _error(error))
    finally:
      for row in self.rows.values():
        good = True
        for obj in (row['out'], row['pipe']):
          try:
            if obj is row['out']:
              obj.flush()
              os.fsync(obj.fileno())
          except BaseException:
            good = False
          finally:
            try:
              obj.close()
            except BaseException:
              good = False
        row['closed'] = good
        if not good:
          row['truncated'] = True
          self.state.uncertain = True
      self.closed = True
  def reference(self):
    return {name: dict(retained_bytes=row['bytes'],retained_sha256=row['sha'].hexdigest(),captured_bytes=min(row['bytes'],self.state.bounds.b['stdio'][name]['capture_bytes']),observed_bytes=row['bytes'],total_output_bytes=row['bytes'] if row['eof'] else None,eof=row['eof'],truncated=row['truncated'] or row['bytes'] > self.state.bounds.b['stdio'][name]['capture_bytes'],complete=row['eof'] and row['closed'] and (row['bytes'] <= self.state.bounds.b['stdio'][name]['capture_bytes'])) for name,row in self.rows.items()}
def run(argv, receipt, budget_path, seconds, reserves=None, stdin=None, cwd=None):
  if _UNCLOSED:
    raise RuntimeError('previous-controller-IO-closure-unknown')
  if type(seconds) not in (int, float) or not math.isfinite(seconds) or seconds <= 0:
    raise ValueError('positive-finite-stage-seconds')
  begin = epoch()
  receipt = Path(receipt)
  budget = Budget(budget_path)
  bounds = Bounds(budget, receipt, argv, stdin, cwd)
  held = bounds.reserves(reserves)
  preflight = budget.check(held)
  supervisors(budget.config.get('supervisor_pids', []))
  supervisor_rss()
  token = TOKEN.open('r+b')
  locked = False
  token_wait_begin = epoch()
  token_contentions = 0
  try:
    if os.fstat(token.fileno()).st_size < 1:
      raise ValueError('uninitialized-heavy-token')
    while epoch() < FUNCTIONAL:
      try:
        token.seek(0)
        msvcrt.locking(token.fileno(), msvcrt.LK_NBLCK, 1)
        acquired_perf_ns=qpc()
        locked = True
        break
      except OSError:
        token_contentions = min(2 ** 63 - 1, token_contentions + 1)
        supervisor_rss()
        time.sleep(0.1)
    if not locked:
      row = dict(exit=None,censor='functional-cutoff-waiting-token',errors=[],no_child_started=True,process_closure_verified=True,qualified=False,token_wait_seconds=epoch() - token_wait_begin,bounds_sha256=bounds.sha)
      write_json(receipt, row, bounds.b['final']['json_bytes'])
      return row
    acquired = epoch()
    absolute_deadline = min(acquired + seconds, FUNCTIONAL)
    lock = threading.RLock()
    stop = threading.Event()
    ready = threading.Event()
    state = EvidenceState(bounds)
    reason = []
    native_calls = [0]
    errors = BoundedList(state, 'errors')
    terminations = BoundedList(state, 'terminations')
    peaks = BoundedMap(state, 'peaks')
    exit_races = BoundedList(state, 'exit')
    samples = BoundedList(state, 'disk')
    supervisor_samples = BoundedList(state, 'supervisor')
    aggregate = [0]
    supervisor_peak = [0]
    missing = [0]
    process = job = owner = raw = pipes = js = None
    timer = sampler = disk_thread = feeder = None
    initial_rss = None
    remaining = None
    closure_checks = []
    creator_lifetime=job_close=None
    io_unknown = False
    def current_members():
      with lock:
        return members(job) if job is not None else []
    def terminate(why):
      with lock:
        if not reason and why != 'root-completed-descendant-cleanup':
          reason.append(why[:128])
        if job and native_calls[0] < 4:
          native_calls[0] += 1
          terminations.append(dict(epoch=epoch(), reason=why[:128]))
          if not K.TerminateJobObject(job, 137):
            if not reason:
              reason.append('owned-Job-termination-failed')
    state.terminate = terminate
    def failed(error, where):
      errors.append(where + ':' + _error(error))
      terminate(where)
    timing = dict(samples=0,completed_samples=0,max_interval_seconds=0.0,intervals_over_40ms=0,max_collection_duration_seconds=0.0,max_completion_interval_seconds=0.0,max_sample_duration_seconds=0.0,max_recorded_interval_seconds=0.0,slow_samples=0,clock='perf_counter_ns',target_interval_ns=20000000,censor_interval_ns=40000000,clock_info={x: vars(time.get_clock_info(x)) for x in ('perf_counter','monotonic')},first_positive_full_sample_ns=None,last_recorded_sample_ns=None,interval_seconds_clock='QPC; coarse diagnostic')
    def record(row, flush=False):
      return raw.record(row, flush) if raw is not None else False
    def coverage(label, empty=None):
      now = qpc()
      previous = timing['last_recorded_sample_ns']
      gap = max(0, now - previous) if previous is not None else None
      row = dict(event=label, sample_complete=False, perf_counter_ns=now, previous_recorded_perf_counter_ns=previous, interval_seconds=gap / 1000000000.0 if gap is not None else None)
      if empty is not None:
        row.update(members=empty, empty_observed=empty == [])
      written = record(row, True)
      timing[label] = dict(perf_counter_ns=now, gap_ns=gap, raw_recorded=written)
      if label == 'root-end-sampling-coverage':
        timing.update(root_wait_returned_ns=now, root_end_coverage_gap_seconds=gap / 1000000000.0 if gap is not None else None)
        if process is not None:
          handle = HANDLE(process._handle)
          values = [w.FILETIME() for _ in range(4)]
          if K.GetProcessId(handle) != process.pid or _stamp(handle) != owner['job_root_creation_time'] or (not K.GetProcessTimes(handle, *(ref(x) for x in values))):
            raise RuntimeError('root-exit-retained-handle-identity')
          timing['root_exit_creation_time'] = _stamp(handle)
          timing['root_exit_observed_ns'] = now
          timing['root_exit_filetime'] = values[1].dwHighDateTime * 2 ** 32 + values[1].dwLowDateTime
      else:
        timing.update(job_empty_observed_ns=now if empty == [] else None, job_end_coverage_gap_seconds=gap / 1000000000.0 if gap is not None else None)
      if not written or gap is None or gap > 40000000 or (empty is not None and empty != []):
        terminate('memory-sample-cadence-censor')
    def memory_watch():
      last_tick = last_coarse = last_complete = last_recorded = None
      next_tick = qpc()
      while not stop.is_set() and (not state.frozen):
        try:
          tick = qpc()
          coarse = time.monotonic()
          interval = tick - last_tick if last_tick is not None else None
          coarse_interval = coarse - last_coarse if last_coarse is not None else None
          if interval is not None:
            timing['max_interval_seconds'] = max(timing['max_interval_seconds'], interval / 1000000000.0)
            if interval > 40000000:
              timing['intervals_over_40ms'] += 1
              record(dict(event='cadence-gap-before-rss',sample_complete=False,epoch=epoch(),perf_counter_ns=tick,previous_perf_counter_ns=last_tick,monotonic=coarse,monotonic_interval_seconds=coarse_interval,interval_seconds=interval / 1000000000.0),True)
              terminate('memory-sample-cadence-censor')
              return
          last_tick, last_coarse = (tick, coarse)
          timing['samples'] += 1
          pids = current_members()
          js.sweep(pids)
          current = 0
          rows = []
          for pid in pids:
            value = js.memory(pid)
            if value is None:
              missing[0] += 1
              if pid in js.departed or pid not in current_members():
                item = dict(pid=pid, epoch=epoch(), **js.departed.get(pid) if isinstance(js.departed.get(pid), dict) else {'state': js.departed.get(pid, 'departed-before-rss'), 'creation_time': None})
                state.log('pid-exit-race', item)
                exit_races.append(item)
                continue
              raise RuntimeError('unreadable-owned-pid-rss')
            peaks[str(pid)] = max(peaks.get(str(pid), 0), value[1])
            rows.append(dict(pid=pid, creation_time=js.live[pid][1], rss=value[0], peak=value[1]))
            current += value[0]
            if value[0] > 2 * GiB:
              terminate('process-rss-censor')
              return
          aggregate[0] = max(aggregate[0], current)
          sup_total, sup_rows = supervisor_rss()
          supervisor_peak[0] = max(supervisor_peak[0], sup_total)
          complete = qpc()
          collection = complete - tick
          completion = complete - last_complete if last_complete is not None else None
          timing['max_collection_duration_seconds'] = max(timing['max_collection_duration_seconds'], collection / 1000000000.0)
          if completion is not None:
            timing['max_completion_interval_seconds'] = max(timing['max_completion_interval_seconds'], completion / 1000000000.0)
          written = record(dict(event='rss-sample',sample_complete=True,epoch=epoch(),perf_counter_ns=tick,completed_perf_counter_ns=complete,collection_duration_seconds=collection / 1000000000.0,completion_interval_seconds=completion / 1000000000.0 if completion is not None else None,monotonic=coarse,monotonic_interval_seconds=coarse_interval,interval_seconds=interval / 1000000000.0 if interval is not None else None,sample_id=timing['completed_samples'] + 1,job_rss=current,processes=rows,supervisor_rss=sup_total,supervisors=sup_rows))
          if not written:
            return
          recorded = qpc()
          duration = recorded - tick
          recorded_interval = recorded - last_recorded if last_recorded is not None else None
          timing['completed_samples'] += 1
          if not record(dict(event='rss-write-complete',sample_complete=False,sample_id=timing['completed_samples'],perf_counter_ns=tick,completed_perf_counter_ns=complete,recorded_perf_counter_ns=recorded,duration_seconds=duration / 1000000000.0)):
            return
          timing['recorded_write_markers'] = timing.get('recorded_write_markers', 0) + 1
          timing['max_sample_duration_seconds'] = max(timing['max_sample_duration_seconds'], duration / 1000000000.0)
          if recorded_interval is not None:
            timing['max_recorded_interval_seconds'] = max(timing['max_recorded_interval_seconds'], recorded_interval / 1000000000.0)
          if duration > 40000000 or (completion is not None and completion > 40000000) or (recorded_interval is not None and recorded_interval > 40000000):
            timing['slow_samples'] += 1
            record(dict(event='full-sample-cadence-late',sample_complete=False,epoch=epoch(),perf_counter_ns=tick,completed_perf_counter_ns=complete,recorded_perf_counter_ns=recorded,previous_recorded_perf_counter_ns=last_recorded,duration_seconds=duration / 1000000000.0,completion_interval_seconds=completion / 1000000000.0 if completion is not None else None,recorded_interval_seconds=recorded_interval / 1000000000.0 if recorded_interval is not None else None),True)
            terminate('memory-sample-cadence-censor')
            return
          last_complete, last_recorded = (complete, recorded)
          timing['last_recorded_sample_ns'] = recorded
          if not supervisor_samples or epoch() - supervisor_samples[-1]['epoch'] >= 1:
            supervisor_samples.append(dict(epoch=epoch(), total=sup_total, processes=sup_rows))
          if current > 3 * GiB:
            terminate('stage-aggregate-rss-censor')
            return
          if not ready.is_set():
            if not any((x['pid'] == process.pid and x['rss'] > 0 for x in rows)):
              raise RuntimeError('first-full-root-rss-not-positive')
            timing['first_positive_full_sample_ns'] = recorded
            ready.set()
        except BaseException as error:
          if not stop.is_set():
            failed(error, 'memory-monitor-error')
          return
        next_tick = max(next_tick + 20000000, qpc())
        stop.wait(max(0, (next_tick - qpc()) / 1000000000.0))
    def disk_watch():
      while not stop.is_set() and (not state.frozen):
        try:
          observation = budget.check(stop=stop)
          if not stop.is_set():
            state.log('disk-budget', observation)
            samples.append(observation)
        except BaseException as error:
          if not stop.is_set():
            failed(error, 'disk-or-free-censor')
          return
        stop.wait(5)
    try:
      if OWNER.exists():
        previous = _strict_read(OWNER, 8192)[0]
        if type(previous.get('pid')) is not int or not previous.get('nonce'):
          raise ValueError('unknown-owner-record')
        if previous.get('state') == 'closure-unknown':
          raise RuntimeError('previous-closure-unknown')
        if previous.get('state') != 'closed':
          old = creation_time(previous['pid'])
          if old is not None and old == previous['creation_time']:
            raise RuntimeError('live-owner-without-token')
      supervisor_rows = supervisors(budget.config.get('supervisor_pids', []))
      total, rows = supervisor_rss(supervisor_rows)
      supervisor_peak[0] = total
      supervisor_samples.append(dict(epoch=epoch(), total=total, processes=rows))
      if reason:
        raise EvidenceCap('prelaunch-collection-cap')
      owner = dict(guard_sha256=sha_file(__file__),acquired_perf_ns=acquired_perf_ns,pid=os.getpid(),creation_time=creation_time(os.getpid()),nonce=state.run_nonce,acquired_epoch=acquired,job_root_pid=None,categories=list(budget.categories),budget_sha256=budget.raw_hash,receipt=str(receipt),state='preparing',authority='advisory; native heavy-token byte lock controls exclusion')
      control_json(OWNER, owner)
      budget.check(held)
      if epoch() >= absolute_deadline:
        raise RuntimeError('functional-cutoff')
      env = dict(os.environ, PYTHONDONTWRITEBYTECODE='1', FILE_NATIVE_STAGE_JOB='1', FILE_NATIVE_BUDGET=str(Path(budget_path).resolve()))
      _encode(env, 65536)
      job = K.CreateJobObjectW(None, None)
      if not job:
        raise winerr(last_error())
      try:creator_lifetime=_creator(job)
      except BaseException:state.uncertain=True;raise
      limits = Limits()
      limits.Basic.Flags = 8192 | 256 | 512
      limits.Process = 2 * GiB
      limits.Job = 3 * GiB
      if not K.SetInformationJobObject(job, 9, ref(limits), size_of(limits)):
        raise winerr(last_error())
      timer = threading.Timer(max(0, absolute_deadline - epoch()), terminate, args=('time-censor',))
      timer.daemon = True
      timer.start()
      raw = StoredGzip(receipt.with_suffix('.rss.jsonl.gz'), state)
      state.raw = raw
      process = subprocess.Popen(argv,cwd=cwd,env=env,close_fds=True,bufsize=0,stdin=subprocess.PIPE if stdin is not None else subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,creationflags=4)
      if not K.AssignProcessToJobObject(job, HANDLE(process._handle)):
        process.kill()
        process.wait(timeout=5)
        raise winerr(last_error())
      if process.pid not in current_members():
        raise RuntimeError('suspended-root-not-in-job')
      js = JobSampling(state, job, current_members)
      initial_rss = js.memory(process.pid)
      if initial_rss is None or initial_rss[0] <= 0:
        raise RuntimeError('suspended-root-rss-unreadable')
      peaks[str(process.pid)] = initial_rss[1]
      owner.update(job_root_pid=process.pid, job_root_creation_time=js.live[process.pid][1], state='assigned-suspended', initial_root_rss=initial_rss[0])
      control_json(OWNER, owner)
      pipes = PipeCapture(process, receipt, state)
      pipes.start()
      sampler = threading.Thread(target=memory_watch, name='owned-memory', daemon=False)
      disk_thread = threading.Thread(target=disk_watch, name='owned-disk', daemon=False)
      sampler.start()
      disk_thread.start()
      while not ready.wait(min(0.005, max(0, absolute_deadline - epoch()))):
        if reason or errors or (not sampler.is_alive()) or (epoch() >= absolute_deadline):
          raise RuntimeError('memory-sampler-not-ready-before-resume')
      check = qpc()
      previous = timing['last_recorded_sample_ns']
      if previous is None or check - previous > 40000000:
        record(dict(event='stale-sample-before-resume',sample_complete=False,perf_counter_ns=check,previous_recorded_perf_counter_ns=previous,interval_seconds=(check - previous) / 1000000000.0 if previous is not None else None),True)
        terminate('memory-sample-cadence-censor')
      if reason or errors:
        raise RuntimeError('memory-sampler-refused-before-resume')
      timing['root_resume_check_ns'] = qpc()
      if epoch() >= absolute_deadline or NT.NtResumeProcess(HANDLE(process._handle)) != 0:
        raise RuntimeError('resume-failed')
      timing['root_resumed_ns'] = qpc()
      owner.update(state='running', resumed_epoch=epoch(), resumed_perf_counter_ns=timing['root_resumed_ns'])
      control_json(OWNER, owner)
      delivered = [0]
      if stdin is not None:
        def feed():
          try:
            while delivered[0] < len(stdin):
              n = process.stdin.write(memoryview(stdin)[delivered[0]:])
              if not n:
                raise OSError('stdin-short-write')
              delivered[0] += n
          except BaseException as error:
            failed(error, 'stdin-delivery-error')
          finally:
            process.stdin.close()
        feeder = threading.Thread(target=feed, name='owned-stdin', daemon=False)
        feeder.start()
      process.wait(timeout=max(0.001, absolute_deadline - epoch()) + 10)
      coverage('root-end-sampling-coverage')
      if current_members():
        terminate('root-completed-descendant-cleanup')
      until = time.monotonic() + 5
      while current_members() and time.monotonic() < until:
        time.sleep(0.01)
      remaining = current_members()
      coverage('job-drain-sampling-coverage', remaining)
    except BaseException as error:
      failed(error, 'stage-deadline-before-launch' if process is None and epoch() >= absolute_deadline else 'guard-error')
      if process:
        try:
          process.wait(timeout=5)
        except subprocess.TimeoutExpired:
          errors.append('termination-wait-failed')
    finally:
      if timer:
        timer.cancel()
        timer.join(timeout=1)
        if timer.is_alive():
          io_unknown = True
          failed(RuntimeError(), 'unjoined-timer')
      stop.set()
      for thread in (sampler, disk_thread, feeder):
        if thread:
          thread.join(timeout=5)
          if thread.is_alive():
            io_unknown = True
            failed(RuntimeError(), 'unjoined-' + thread.name)
      if pipes and pipes.thread:
        pipes.thread.join(timeout=5)
        if pipes.thread.is_alive():
          pipes.stop.set()
          pipes.thread.join(timeout=1)
        if pipes.thread.is_alive():
          io_unknown = True
          failed(RuntimeError(), 'unjoined-owned-stdio')
      if state.uncertain or _UNCLOSED or (process is not None and process.poll() is None):
        io_unknown = True
        failed(RuntimeError(), 'owned-resource-closure-unknown')
      if job:
        try:
          remaining = current_members()
          if remaining:
            terminate('guard-finally')
            until = time.monotonic() + 5
            while current_members() and time.monotonic() < until:
              time.sleep(0.01)
          for unused in range(2):
            remaining = current_members()
            closure_checks.append(dict(epoch=epoch(), perf_counter_ns=qpc(), run_nonce=state.run_nonce, members=remaining))
            time.sleep(0.02)
        except BaseException as error:
          remaining = None
          failed(error, 'Job-closure-error')
      elif process is None:
        closure_checks = [dict(epoch=epoch(), perf_counter_ns=qpc(), run_nonce=state.run_nonce, members=[], proof='no-Job-or-child-created') for unused in range(2)]
        remaining = []
      if io_unknown:
        state.frozen = True
        _UNCLOSED.append(dict(kind='unjoined-io',run_nonce=state.run_nonce,job=job,process=process,raw=raw,pipes=pipes,sampler=sampler,disk=disk_thread,feeder=feeder,timer=timer,identities=js,state=state,owner=owner,creator_lifetime=creator_lifetime))
      else:
        if js:
          try:
            js.close()
          except BaseException as error:
            io_unknown = True
            failed(error, 'identity-close-unknown')
        if raw:
          try:
            raw.close()
          except BaseException as error:
            io_unknown = True
            failed(error, 'gzip-close-error')
        if process:
          for stream in (process.stdin, process.stdout, process.stderr):
            if stream and (not stream.closed):
              try:
                stream.close()
              except BaseException as error:
                io_unknown = True
                failed(error, 'stream-close-unknown')
        if process is not None and process.poll() is not None:
          try:
            process._handle.Close()
          except BaseException as error:
            io_unknown = True
            failed(error, 'root-handle-close-unknown')
        if job:
          with lock:
            closing = job
            job = None
            try:
              _checked_close(closing,'final-Job')
              job_close=dict(checked=True,perf_counter_ns=qpc())
            except BaseException as error:
              job=closing;io_unknown=True;failed(error,'Job-close-unknown')
      if io_unknown and (not _UNCLOSED):
        _UNCLOSED.append(dict(kind='close-unknown', run_nonce=state.run_nonce, job=job, process=process, raw=raw, pipes=pipes, identities=js, state=state, timer=timer))
      proven = len(closure_checks) == 2 and all((x['members'] == [] for x in closure_checks)) and (remaining == []) and (not io_unknown)
      if owner:
        owner.update(state='closed' if proven else 'closure-unknown', closed_epoch=epoch(), remaining_job_pids=remaining, errors=errors)
        try:
          control_json(OWNER, owner)
        except BaseException as error:
          failed(error, 'owner-final-error')
          compact = dict(pid=owner['pid'], creation_time=owner['creation_time'], nonce=owner['nonce'], state='closure-unknown', receipt=str(receipt), errors=['owner-final-error'])
          try:
            control_json(OWNER, compact)
          except BaseException as secondary:
            errors.append('owner-fence:' + _error(secondary))
    streams = pipes.reference() if pipes else {}
    if not streams or not all((x['complete'] for x in streams.values())):
      terminate('stdio-incomplete')
    raw_ref = raw.reference() if raw else dict(gzip_complete=False, evidence_complete=False)
    if not raw_ref['gzip_complete']:
      terminate('gzip-incomplete')
    row = dict(creator_lifetime=creator_lifetime,job_close=job_close,argv=argv,cwd=str(cwd) if cwd else None,started_epoch=begin,token_acquired_epoch=acquired,token_wait_seconds=acquired - token_wait_begin,token_wait_started_epoch=token_wait_begin,token_contentions=token_contentions,before_token_setup_seconds=token_wait_begin - begin,total_seconds=epoch() - begin,stage_seconds=epoch() - acquired,requested_seconds=seconds,functional_cutoff_epoch=FUNCTIONAL,absolute_deadline_epoch=absolute_deadline,memory_sample_timing=timing,raw_rss=str(receipt.with_suffix('.rss.jsonl.gz')),closure_checks=closure_checks,exit=process.returncode if process else None,censor=reason[0] if reason else None,errors=errors,job_commit_limit=3 * GiB,process_commit_limit=2 * GiB,job_aggregate_sampled_peak_rss=aggregate[0] or None,per_process_observed_peak_rss=peaks,missing_memory_samples=missing[0],pid_exit_races=exit_races,initial_suspended_root_rss=initial_rss[0] if initial_rss else None,supervisor_rss_limit=768 * 1024 ** 2,supervisor_peak_rss=supervisor_peak[0],supervisor_samples=supervisor_samples,owner=owner,remaining_job_pids=remaining,process_closure_verified=proven and (not errors),terminations=terminations,disk_samples=samples,prospective=preflight,guard_sha256=sha_file(__file__),budget_sha256=budget.raw_hash,frozen_reference_guard_sha256=sha_file(Path(__file__).resolve().parent.parent / 'harness/job_guard.py'),frozen_reference_expected_sha256='1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae',donor_sha256='50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378',run_nonce=state.run_nonce,bounds_sha256=bounds.sha,collection_usage=state.used,suppressed_collection_rows=state.suppressed,native_identity_rows=js.identities if js else [],identity_peak_rss=js.identity_peaks if js else {},stdio=streams,rss_evidence=raw_ref,cap=state.cap_reason,termination_native_attempts=native_calls[0],qualified=False,postwrite_census_required=True,sampling_revision='bounded-QPC-v3',qualification='caller completion')
    try:
      data = _encode(row, bounds.b['final']['json_bytes'])
    except EvidenceCap:
      state.cap('final-json-bytes')
      row = dict(exit=process.returncode if process else None,censor='evidence-cap:final-json-bytes',errors=['final-json-cap'],process_closure_verified=proven,qualified=False,partial_collections=True,bounds_sha256=bounds.sha,guard_sha256=sha_file(__file__),collection_usage=state.used,stdio=streams,rss_evidence=raw_ref,postwrite_census_required=True)
      data = _encode(row, bounds.b['final']['json_bytes'])
    emergency = dict(censor=row.get('censor'),cap=state.cap_reason,collection_usage=state.used,suppressed=state.suppressed,io_unknown=io_unknown,closure_checks=closure_checks,gzip_complete=raw_ref['gzip_complete'],stdio=streams,completion_pending=True,native_close_uncertainties=[x['public'] for x in _UNCLOSED if isinstance(x,dict) and 'public' in x])
    try:
      write_json(receipt.with_suffix('.bounds.json'), emergency, bounds.b['final']['emergency_bytes'])
    except EvidenceCap:
      state.cap('emergency-record-bytes')
      write_json(receipt.with_suffix('.bounds.json'),dict(censor='emergency-record-cap',io_unknown=io_unknown,completion_pending=True,native_close_uncertainties=[x['public'] for x in _UNCLOSED if isinstance(x,dict) and 'public' in x]),bounds.b['final']['emergency_bytes'])
      row['censor'] = 'evidence-cap:emergency-record-bytes'
      data = _encode(row, bounds.b['final']['json_bytes'])
    with receipt.open('xb') as output:
      _write_all(output, data)
      output.flush()
      os.fsync(output.fileno())
    row['receipt_sha256'] = sha256(data).hexdigest()
    try:
      if io_unknown:
        raise RuntimeError('postwrite-census-IO-unknown')
      row['postwrite_census'] = budget.check()
      row['postwrite_census_complete'] = True
    except BaseException as error:
      row['postwrite_census_complete'] = False
      row['censor'] = 'postwrite-census-error'
      row['completion_error'] = _error(error)
    row['qualified'] = row.get('exit') == 0 and (not row.get('censor')) and (not errors) and proven and raw_ref['gzip_complete'] and all((x['complete'] for x in streams.values())) and (row.get('postwrite_census_complete') is True)
    completion = {k: row.get(k) for k in ('exit','censor','qualified','process_closure_verified','postwrite_census_complete','receipt_sha256','guard_sha256','budget_sha256','bounds_sha256','creator_lifetime','job_close','closure_checks')}
    completion['postwrite_census'] = row.get('postwrite_census')
    try:
      _encode(completion, bounds.b['final']['completion_bytes'])
    except EvidenceCap:
      row['qualified'] = False
      row['censor'] = 'completion-byte-cap'
      completion = dict(qualified=False, censor='completion-byte-cap', receipt_sha256=row['receipt_sha256'])
    row['completion'] = completion
    return row
  finally:
    if locked:
      try:
        token.seek(0)
        msvcrt.locking(token.fileno(), msvcrt.LK_UNLCK, 1)
      finally:
        token.close()
    else:
      token.close()
def _write_all(output, data):
  view = memoryview(data)
  while view:
    n = output.write(view)
    if not n:
      raise OSError('short-control-write')
    view = view[n:]
def _checked_close(handle, origin, pid=None, creation=None):
  try:
    ok = bool(K.CloseHandle(handle))
  except BaseException:
    ok = False
  if not ok:
    public = dict(kind='native-close-uncertain', origin=origin, pid=pid, creation_time=creation, close_attempted=True)
    _UNCLOSED.append(dict(kind='native-close-uncertain', handle=handle, public=public))
    raise RuntimeError('native-close-uncertain:' + origin)
def _finish_io_fence(nonce, deadline):
  if len(_UNCLOSED)!=1:raise RuntimeError('close-unknown')
  c=_UNCLOSED[0]
  if c.get('kind')!='unjoined-io' or c.get('run_nonce')!=nonce or c.get('close_attempted') or c['state'].uncertain:
    raise RuntimeError('fence')
  def left():
    n=min(deadline,FUNCTIONAL)-epoch()
    if n<=0:raise RuntimeError('deadline')
    return n
  for key in ('timer','sampler','disk','feeder'):
    worker=c.get(key)
    if worker:worker.join(left())
    if worker and worker.is_alive():raise RuntimeError('worker-live')
  pipes=c['pipes'];pipes.thread.join(left())
  if pipes.thread.is_alive() or not all(x['closed'] for x in pipes.rows.values()):
    raise RuntimeError('IO-open')
  root=c['process'];root.wait(timeout=left())
  created=_stamp(HANDLE(root._handle))
  if created!=c['owner']['job_root_creation_time'] or K.GetProcessId(HANDLE(root._handle))!=root.pid:
    raise RuntimeError('root-identity')
  empty=[]
  for _ in range(2):
    left();pids=members(c['job'])
    empty.append(dict(run_nonce=nonce,perf_counter_ns=qpc(),members=pids))
    if pids:raise RuntimeError('nonempty-Job')
    time.sleep(min(.02,left()))
  c['close_attempted']=True
  if root._handle.closed:raise RuntimeError('root-close')
  c['identities'].close();c['raw'].close()
  root._handle.Close()
  _checked_close(c['job'],'IO-fence-Job')
  closed=dict(checked=True,perf_counter_ns=qpc())
  return dict(creator_lifetime=c['creator_lifetime'],job_close=closed,run_nonce=nonce,root_creation_time=created,closure_checks=empty,
             IO_closed=True)
def _creator(job):
  h=K.GetCurrentProcess();inside=w.BOOL();flags=DWORD()
  created=_stamp(h)
  ok=K.IsProcessInJob(h,job,ref(inside));query_perf_ns=qpc()
  if not ok or inside.value:
    raise RuntimeError('creator-inside-or-unknown')
  ok=K.GetHandleInformation(job,ref(flags));flags_perf_ns=qpc()
  if not ok or flags.value&1:
    raise RuntimeError('Job-inheritable-or-unknown')
  return dict(creator_job_query=dict(perf_ns=query_perf_ns),job_handle_flags=dict(perf_ns=flags_perf_ns),pid=os.getpid(),creation_time=created,exact_job_query_succeeded=True,
    in_exact_job=False,handle_flags=flags.value,unnamed_job=True,
    source_known_duplicate_handles=0,source_known_inherited_handles=0)
