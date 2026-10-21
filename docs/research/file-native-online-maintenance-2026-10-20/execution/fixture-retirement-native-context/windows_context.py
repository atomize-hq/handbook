"""Windows cooperating-writer Context. SOURCE ONLY until reviewed finite RUN GO.

CLI: exactPython -I -S -B -u windows_context.py MANIFEST_PATH MANIFEST_SHA256.
The unlocked outer capture owns the Job/RSS/IO watchdog; THIS process owns the
heavy-token byte lock. No IPC, owner-JSON authority, hostile-user isolation,
implicit extraction, retry, deletion permission, or missing-case PASS exists.
Archive037's native DELETE handle remains its content/disposition endpoint.
"""
import ast
import ctypes as c
from ctypes import wintypes as w
import hashlib
import json
import math
import ntpath
import os
from pathlib import Path
import struct
import sys
import threading
import time

ENGINE = "0628322e4b48e0efad19cdc32238a622d4aaa7a8"
FUNCTIONAL = 1792581014
TOKEN = "C:\\file-native-online-control-20261020\\windows-heavy.lock"
CHUNK = 65536
KEEP = []                         # Uncertain native ownership is never retried.
ENVELOPE = {"version", "runtime_go", "action", "parent_capture_sha256",
            "context_source_sha256", "deadline_perf_ns", "deadline_epoch",
            "real_retire_go", "authority", "parent"}
AUTHORITY = {"version", "engine_commit", "archive_source", "controls_source",
             "plan", "token", "protected_scope", "writer_catalog", "prior_jobs",
             "qualification", "budget", "limits", "native_controls"}
LIMITS = {"max_refs", "max_ref_bytes", "max_aliases", "max_members",
          "archive_bytes", "artifact_files", "body_bytes", "body_files",
          "evidence_bytes", "evidence_files", "control_root", "evidence_root"}
NATIVE_CASES = {"C18-native-reparse-fork", "C22-linked-ancestor",
                "C23-inflight-mutation", "C27-active-io", "C30-midway-stop",
                "C31-crash-journal", "C32-sharing", "C33-protected-bind",
                "C34-special-hardlinks", "C35-negative-admission",
                "C36-review-substitution"}


class Refusal(RuntimeError):
    pass


def need(value, code):
    if not value:
        raise Refusal(code)


def closed(value, keys, code):
    need(type(value) is dict and set(value) == set(keys), code)


def integer(value, low, high):
    return type(value) is int and low <= value <= high


def is_sha(value):
    return type(value) is str and len(value) == 64 and all(x in "0123456789abcdef" for x in value)


def encode(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=True,
                      separators=(",", ":"), allow_nan=False).encode("ascii")


def decode(raw):
    def pairs(items):
        result = {}
        for key, value in items:
            need(key not in result, "duplicate-key")
            result[key] = value
        return result
    def invalid(value):
        raise Refusal("nonfinite-json:" + value)
    return json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid)


def absolute(value):
    need(type(value) is str and len(value) <= 2048, "bounded-native-path")
    drive, tail = ntpath.splitdrive(value)
    need(len(drive) == 2 and drive[1] == ":" and drive[0].isalpha()
         and tail.startswith("\\") and ntpath.abspath(value) == value,
         "canonical-drive-path-required")
    need(":" not in tail and "\x00" not in tail, "ADS-or-device-path")
    need(all(x not in [".", ".."] and not x.endswith((".", " "))
             for x in tail.split("\\") if x), "ambiguous-component")
    return value


class Overlapped(c.Structure):
    _fields_ = [("Internal", c.c_size_t), ("InternalHigh", c.c_size_t),
                ("Offset", w.DWORD), ("OffsetHigh", w.DWORD), ("event", w.HANDLE)]


class BasicProcess(c.Structure):
    _fields_ = [("status", c.c_long), ("peb", c.c_void_p),
                ("affinity", c.c_size_t), ("priority", c.c_long),
                ("pid", c.c_size_t), ("parent", c.c_size_t)]


class UnicodeString(c.Structure):
    _fields_ = [("length", w.USHORT), ("maximum", w.USHORT), ("buffer", c.c_void_p)]


class Native:
    """Lazy actual Win32 boundary. Unknown/truncated queries always refuse."""
    def __init__(self):
        need(os.name == "nt" and c.sizeof(c.c_void_p) == 8, "Windows64-only")
        self.k = c.WinDLL("kernel32", use_last_error=True)
        self.nt = c.WinDLL("ntdll", use_last_error=True)
        self.shell = c.WinDLL("shell32", use_last_error=True)
        self.uncertain = []
        declarations = {
            "CreateFileW": ([w.LPCWSTR, w.DWORD, w.DWORD, c.c_void_p,
                              w.DWORD, w.DWORD, w.HANDLE], w.HANDLE),
            "CloseHandle": ([w.HANDLE], w.BOOL),
            "GetFileInformationByHandleEx": ([w.HANDLE, c.c_int, c.c_void_p, w.DWORD], w.BOOL),
            "GetFinalPathNameByHandleW": ([w.HANDLE, w.LPWSTR, w.DWORD, w.DWORD], w.DWORD),
            "GetVolumeInformationByHandleW": ([w.HANDLE, w.LPWSTR, w.DWORD,
                c.POINTER(w.DWORD), c.POINTER(w.DWORD), c.POINTER(w.DWORD), w.LPWSTR, w.DWORD], w.BOOL),
            "ReadFile": ([w.HANDLE, c.c_void_p, w.DWORD, c.POINTER(w.DWORD), c.c_void_p], w.BOOL),
            "LockFileEx": ([w.HANDLE, w.DWORD, w.DWORD, w.DWORD, w.DWORD,
                             c.POINTER(Overlapped)], w.BOOL),
            "UnlockFileEx": ([w.HANDLE, w.DWORD, w.DWORD, w.DWORD,
                               c.POINTER(Overlapped)], w.BOOL),
            "GetCurrentProcess": ([], w.HANDLE),
            "GetProcessId": ([w.HANDLE], w.DWORD),
            "OpenProcess": ([w.DWORD, w.BOOL, w.DWORD], w.HANDLE),
            "GetProcessTimes": ([w.HANDLE] + [c.POINTER(w.FILETIME)] * 4, w.BOOL),
            "QueryFullProcessImageNameW": ([w.HANDLE, w.DWORD, w.LPWSTR, c.POINTER(w.DWORD)], w.BOOL),
            "WaitForSingleObject": ([w.HANDLE, w.DWORD], w.DWORD),
            "QueryInformationJobObject": ([w.HANDLE, c.c_int, c.c_void_p,
                                            w.DWORD, c.POINTER(w.DWORD)], w.BOOL),
            "GetProcessHandleCount": ([w.HANDLE, c.POINTER(w.DWORD)], w.BOOL),
            "FindFirstFileNameW": ([w.LPCWSTR, w.DWORD, c.POINTER(w.DWORD), w.LPWSTR], w.HANDLE),
            "FindNextFileNameW": ([w.HANDLE, c.POINTER(w.DWORD), w.LPWSTR], w.BOOL),
            "FindClose": ([w.HANDLE], w.BOOL),
        }
        for name, (args, result) in declarations.items():
            fn = getattr(self.k, name)
            fn.argtypes, fn.restype = args, result
        self.nt.NtQueryInformationProcess.argtypes = [w.HANDLE, c.c_int,
            c.c_void_p, w.ULONG, c.POINTER(w.ULONG)]
        self.nt.NtQueryInformationProcess.restype = c.c_long
        self.shell.CommandLineToArgvW.argtypes = [w.LPCWSTR, c.POINTER(c.c_int)]
        self.shell.CommandLineToArgvW.restype = c.POINTER(w.LPWSTR)
        self.k.LocalFree.argtypes, self.k.LocalFree.restype = [c.c_void_p], c.c_void_p

    def query(self, handle, kind, size):
        raw = c.create_string_buffer(size)
        need(self.k.GetFileInformationByHandleEx(handle, kind, raw, size), "native-file-query:" + str(kind))
        return raw.raw

    def open(self, path, access=0x80000080, share=1):
        handle = self.k.CreateFileW(absolute(str(path)), access, share, None, 3, 0x02200000, None)
        need(handle not in (None, c.c_void_p(-1).value), "native-open:" + str(c.get_last_error()))
        return handle

    def close(self, handle):
        if not self.k.CloseHandle(handle):
            self.uncertain.append(handle)
            raise Refusal("native-close-uncertain")

    def meta(self, handle):
        identity = self.query(handle, 18, 24)
        basic, standard = self.query(handle, 0, 40), self.query(handle, 1, 24)
        need(not standard[20], "delete-pending-object")
        text = c.create_unicode_buffer(32768)
        length = self.k.GetFinalPathNameByHandleW(handle, text, len(text), 1)
        need(0 < length < len(text), "native-normalized-volume-path")
        path = text.value
        end = path.find("}\\")
        need(path.startswith("\\\\?\\Volume{") and end > 10, "local-volume-GUID-required")
        volume = path[:end + 2].lower()
        serial, maximum, flags = w.DWORD(), w.DWORD(), w.DWORD()
        filesystem = c.create_unicode_buffer(32)
        need(self.k.GetVolumeInformationByHandleW(handle, None, 0, c.byref(serial),
             c.byref(maximum), c.byref(flags), filesystem, len(filesystem)), "native-volume-query")
        need(filesystem.value == "NTFS", "unqualified-filesystem")
        return dict(identity="win:" + identity.hex(), mount=volume + identity[:8].hex(),
                    path=path, attrs=struct.unpack_from("<I", basic, 32)[0],
                    directory=bool(standard[21]), links=struct.unpack_from("<I", standard, 16)[0],
                    size=struct.unpack_from("<q", standard, 8)[0],
                    stamp=basic[16:32].hex())

    def streams(self, handle):
        raw, offset = self.query(handle, 7, CHUNK), 0
        for unused in range(2048):
            need(offset + 24 <= len(raw), "stream-overflow")
            nxt, length = struct.unpack_from("<II", raw, offset)
            need(length % 2 == 0 and offset + 24 + length <= len(raw), "stream-shape")
            name = raw[offset + 24:offset + 24 + length].decode("utf-16-le", "strict")
            need(name in ("", "::$DATA"), "nondefault-ADS")
            if not nxt:
                return
            need(nxt >= 24 + length, "stream-offset")
            offset += nxt
        raise Refusal("stream-count")

    def read(self, handle, count):
        raw, used = c.create_string_buffer(count), w.DWORD()
        need(self.k.ReadFile(handle, raw, count, c.byref(used), None), "native-read")
        need(used.value <= count, "native-read-count")
        return raw.raw[:used.value]

    def creation(self, handle):
        values = [w.FILETIME() for unused in range(4)]
        need(self.k.GetProcessTimes(handle, *(c.byref(x) for x in values)), "native-process-times")
        return values[0].dwLowDateTime | (values[0].dwHighDateTime << 32)

    def handle_count(self):
        count = w.DWORD()
        need(self.k.GetProcessHandleCount(self.k.GetCurrentProcess(), c.byref(count)), "native-handle-count")
        return count.value

    def members(self, maximum):
        raw = c.create_string_buffer(8 + maximum * c.sizeof(c.c_size_t))
        returned = w.DWORD()
        need(self.k.QueryInformationJobObject(None, 3, raw, len(raw), c.byref(returned)), "current-Job-query")
        assigned, listed = struct.unpack_from("<II", raw)
        need(0 < assigned == listed <= maximum, "current-Job-list-incomplete")
        ids = set((c.c_size_t * listed).from_buffer(raw, 8))
        need(len(ids) == listed and os.getpid() in ids, "current-Job-native-members")
        return ids

    def parent(self):
        value, used = BasicProcess(), w.ULONG()
        need(self.nt.NtQueryInformationProcess(self.k.GetCurrentProcess(), 0,
            c.byref(value), c.sizeof(value), c.byref(used)) == 0, "native-parent-query")
        need(value.pid == os.getpid() and value.parent > 0, "native-parent-identity")
        return value.parent

    def argv(self, handle):
        used = w.ULONG()
        self.nt.NtQueryInformationProcess(handle, 60, None, 0, c.byref(used))
        need(c.sizeof(UnicodeString) <= used.value <= 65536, "native-commandline-bound")
        raw = c.create_string_buffer(used.value)
        need(self.nt.NtQueryInformationProcess(handle, 60, raw, len(raw), c.byref(used)) == 0,
             "native-commandline-unavailable")
        string = UnicodeString.from_buffer(raw)
        start, end = c.addressof(raw), c.addressof(raw) + len(raw)
        need(string.length % 2 == 0 and start <= string.buffer <= end
             and string.buffer + string.length <= end, "native-commandline-pointer")
        text = c.string_at(string.buffer, string.length).decode("utf-16-le", "strict")
        count = c.c_int()
        pointer = self.shell.CommandLineToArgvW(text, c.byref(count))
        need(pointer and 1 <= count.value <= 64, "native-argv-count")
        try:
            return [pointer[i] for i in range(count.value)]
        finally:
            need(not self.k.LocalFree(pointer), "native-argv-free")


class Pin:
    """Own no-follow ancestry and leaf handles until explicitly retired."""
    def __init__(self, native, path, access=0x80000080, share=1, allow_reparse=False):
        self.n, self.path, self.handles = native, absolute(str(path)), []
        self.closed = False
        try:
            item = Path(self.path)
            for ancestor in reversed(item.parents):
                handle = native.open(str(ancestor))
                self.handles.append(handle)
                meta = native.meta(handle)
                need(meta["directory"] and not meta["attrs"] & 0x400, "reparse-ancestor")
            self.handle = native.open(self.path, access, share)
            self.handles.append(self.handle)
            self.meta = native.meta(self.handle)
            need(allow_reparse or not self.meta["attrs"] & 0x400, "reparse-reference")
        except BaseException:
            self.close()
            raise

    def close(self):
        if self.closed:
            return
        errors = []
        while self.handles:
            handle = self.handles.pop()   # Retire before attempting a close.
            try:
                self.n.close(handle)
            except BaseException as error:
                errors.append(error)
        self.closed = True
        need(not errors, "pinned-handle-close-uncertain")


class GuardianLease:
    """Actual local byte-lock acquisition; never a caller-provided held flag."""
    def __init__(self, native, token, poll):
        closed(token, {"path", "identity"}, "token-binding")
        need(token["path"] == TOKEN, "wrong-global-token")
        self.n, self.poll, self.locked = native, poll, False
        self.pin = Pin(native, TOKEN, 0xC0000080, 3)
        self.overlapped = Overlapped()
        try:
            need(self.pin.meta["identity"] == token["identity"] and not self.pin.meta["directory"]
                 and self.pin.meta["links"] == 1 and self.pin.meta["size"] >= 1, "token-native-identity")
            while True:
                poll()
                if native.k.LockFileEx(self.pin.handle, 3, 0, 1, 0, c.byref(self.overlapped)):
                    self.locked = True
                    self.acquired_perf_ns = time.perf_counter_ns()
                    break
                need(c.get_last_error() == 33, "native-token-lock-error")
                time.sleep(0.02)
        except BaseException:
            self.pin.close()
            raise

    def check(self):
        self.poll()
        need(self.locked and not self.pin.closed and not self.n.uncertain, "local-lease-revoked")
        need(self.n.meta(self.pin.handle) == self.pin.meta, "token-object-mutated")
        return self.acquired_perf_ns

    def release(self):
        self.check()
        self.locked = False               # Never retry uncertain unlock/close.
        if not self.n.k.UnlockFileEx(self.pin.handle, 0, 1, 0, c.byref(self.overlapped)):
            KEEP.append(self)
            raise Refusal("native-token-unlock-uncertain")
        self.pin.close()


class Context:
    """Closed parent admission plus local native proof, scoped to one process."""
    def __init__(self, manifest_path, manifest_sha):
        self.n = Native()
        self.baseline_handles = self.n.handle_count()
        self.baseline_threads = {x.ident for x in threading.enumerate()}
        self.refs, self.plans, self.actors = {}, {}, []
        self.buffered_bytes, self.control_scopes = 0, {}
        self.anchor_parents = []
        self.control_case, self.batch_expected = None, None
        self.lease, self.parent_handle = None, None
        self.archive_module = None
        self.completed = False
        self.max_refs, self.max_ref_bytes = 512, 64 * 1024**2
        manifest = self.read_ref({"path": manifest_path, "sha256": manifest_sha,
                                  "bytes": Path(manifest_path).stat().st_size})
        self.m = decode(manifest)
        closed(self.m, ENVELOPE, "context-envelope")
        need(self.m["version"] == 1 and type(self.m["version"]) is int
             and self.m["runtime_go"] is True, "no-context-runtime-admission")
        need(self.m["action"] in {"CONTROL", "ARCHIVE_VERIFY", "RETIRE"}
             and type(self.m["real_retire_go"]) is bool, "closed-action")
        need(self.m["action"] == "RETIRE" or self.m["real_retire_go"] is False,
             "retire-grant-on-wrong-action")
        for key in ("parent_capture_sha256", "context_source_sha256"):
            need(is_sha(self.m[key]), "source-SHA")
        need(integer(self.m["deadline_perf_ns"], 1, 2**63 - 1), "perf-deadline")
        need(type(self.m["deadline_epoch"]) in (int, float)
             and math.isfinite(self.m["deadline_epoch"])
             and time.time() < self.m["deadline_epoch"] <= FUNCTIONAL, "functional-deadline")
        self.poll()
        own = Path(__file__).absolute()
        need(hashlib.sha256(self.read_ref({"path": str(own), "bytes": own.stat().st_size,
             "sha256": self.m["context_source_sha256"]})).hexdigest() == self.m["context_source_sha256"],
             "executed-Context-source")
        self.a = decode(self.read_ref(self.m["authority"]))
        closed(self.a, AUTHORITY, "Context-authority")
        need(self.a["version"] == 1 and type(self.a["version"]) is int
             and self.a["engine_commit"] == ENGINE, "fixed-engine-authority")
        closed(self.a["limits"], LIMITS, "Context-limits")
        self.limits = self.a["limits"]
        for key in LIMITS - {"control_root", "evidence_root"}:
            need(integer(self.limits[key], 1, 2**34), "positive-limit:" + key)
        need(self.limits["max_refs"] <= 4096 and self.limits["max_ref_bytes"] <= 64 * 1024**2
             and self.limits["max_aliases"] <= 4096 and self.limits["max_members"] <= 4096,
             "native-collection-limit")
        self.max_refs, self.max_ref_bytes = self.limits["max_refs"], self.limits["max_ref_bytes"]
        need(len(self.refs) <= self.max_refs, "initial-reference-count")
        for path in sorted({ntpath.dirname(absolute(self.limits[k])) for k in ("control_root", "evidence_root")}):
            pin = Pin(self.n, path)
            self.anchor_parents.append(pin)
            need(pin.meta["directory"], "output-existing-native-parent")
        self.verify_parent()
        self.lease = GuardianLease(self.n, self.a["token"], self.poll)
        self.protected = decode(self.read_ref(self.a["protected_scope"]))
        self.validate_protected()
        self.verify_writers()
        self.archive_raw = self.read_ref(self.a["archive_source"])
        need(len(self.archive_raw) <= 55296, "archive-source-cap")
        self.budget = decode(self.read_ref(self.a["budget"]))
        self.charged_archive = 0
        self.artifact_anchor = None
        self.check_budget(0, 0)

    def poll(self):
        need(time.time() < min(self.m["deadline_epoch"], getattr(self, "control_deadline", self.m["deadline_epoch"]))
             and time.perf_counter_ns() < self.m["deadline_perf_ns"], "Context-deadline")

    def read_ref(self, ref, load=True):
        closed(ref, {"path", "bytes", "sha256"}, "reference-shape")
        need(is_sha(ref["sha256"]) and integer(ref["bytes"], 0, self.max_ref_bytes), "reference-bound")
        if load:
            need(ref["bytes"] <= 2 * 1024**2, "decoded-reference-bound")
        path = absolute(ref["path"])
        key = (path, ref["bytes"], ref["sha256"])
        if key in self.refs:
            pin, previous = self.refs[key]
            need(self.n.meta(pin.handle) == pin.meta, "reference-lease-mutated")
            need(not load or previous is not None, "reference-not-buffered")
            return previous
        need(len(self.refs) < self.max_refs and self.n.handle_count() < 4096, "reference-count")
        need(not load or self.buffered_bytes + ref["bytes"] <= 32 * 1024**2, "reference-RAM-cap")
        for scope in self.control_scopes.values():
            base = ntpath.normcase(scope["root"]["path"]).rstrip("\\")
            need(not ntpath.normcase(path).startswith(base + "\\"), "reference-inside-retirement-root")
        need(not any(k[0] == path for k in self.refs), "reference-rebinding")
        pin = Pin(self.n, path)
        try:
            need(not pin.meta["directory"] and pin.meta["links"] == 1
                 and pin.meta["size"] == ref["bytes"], "reference-native-shape")
            self.n.streams(pin.handle)
            digest, blocks, size = hashlib.sha256(), [], 0
            while True:
                if hasattr(self, "m"):
                    self.poll()
                block = self.n.read(pin.handle, CHUNK)
                if not block:
                    break
                size += len(block)
                need(size <= ref["bytes"], "reference-growth")
                digest.update(block)
                if load:
                    blocks.append(block)
            need(size == ref["bytes"] and digest.hexdigest() == ref["sha256"]
                 and self.n.meta(pin.handle) == pin.meta, "same-handle-reference-SHA")
            raw = b"".join(blocks) if load else None
            self.refs[key] = (pin, raw)
            self.buffered_bytes += len(raw) if raw is not None else 0
            return raw
        except BaseException:
            pin.close()
            raise

    def verify_parent(self):
        parent = self.m["parent"]
        closed(parent, {"pid", "creation", "image", "source", "argv"}, "outer-parent-shape")
        need(integer(parent["pid"], 1, 2**32 - 1) and self.n.parent() == parent["pid"],
             "actual-direct-parent")
        self.parent_handle = self.n.k.OpenProcess(0x101000, False, parent["pid"])
        need(self.parent_handle and self.n.k.GetProcessId(self.parent_handle) == parent["pid"]
             and self.n.creation(self.parent_handle) == parent["creation"]
             and self.n.k.WaitForSingleObject(self.parent_handle, 0) == 258, "live-native-parent")
        text, length = c.create_unicode_buffer(32768), w.DWORD(32768)
        need(self.n.k.QueryFullProcessImageNameW(self.parent_handle, 0, text, c.byref(length)),
             "native-parent-image")
        need(ntpath.normcase(text.value) == ntpath.normcase(parent["image"]["path"]), "parent-image-binding")
        self.read_ref(parent["image"], False)
        self.read_ref(parent["source"])
        need(parent["source"]["sha256"] == self.m["parent_capture_sha256"], "capture-source-binding")
        need(type(parent["argv"]) is list and self.n.argv(self.parent_handle) == parent["argv"]
             and parent["source"]["path"] in parent["argv"], "native-parent-source-argv")
        members = self.n.members(self.limits["max_members"])
        need(parent["pid"] not in members, "guardian-must-be-outside-current-Job")

    def verify_writers(self):
        catalog = decode(self.read_ref(self.a["writer_catalog"]))
        closed(catalog, {"version", "root", "freeze_nonce", "engine_commit", "ports", "review"}, "writer-catalog")
        need(catalog["version"] == 1 and catalog["engine_commit"] == ENGINE
             and type(catalog["ports"]) is list and len(catalog["ports"]) <= 256
             and (catalog["ports"] or self.m["action"] == "CONTROL"),
             "source-closed-writer-universe")
        self.read_ref(catalog["review"])
        runs = set()
        if type(self.a["native_controls"]) is dict and self.a["native_controls"].get("phase") == "C31_RECOVERY":
            role, q = self.c31_port()
            checkpoint = decode(self.read_ref(q["checkpoint_ref"]))
            self.c31_binding(q, checkpoint)
            runs.add(self.c31_closure(q, "fault", checkpoint))
        for prior in self.a["prior_jobs"]:
            closed(prior, {"raw", "completion", "raw_files"}, "prior-job-shape")
            raw_bytes = self.read_ref(prior["raw"])
            raw, completion = decode(raw_bytes), decode(self.read_ref(prior["completion"]))
            need(completion["receipt_sha256"] == hashlib.sha256(raw_bytes).hexdigest()
                 and completion["qualified"] is True and completion["exit"] == 0
                 and completion["censor"] is None and completion["process_closure_verified"] is True
                 and completion["postwrite_census_complete"] is True, "prior-guard-returned-completion")
            need(raw["errors"] == [] and raw["remaining_job_pids"] == []
                 and raw["rss_evidence"]["gzip_complete"] is True, "prior-raw-native-closure")
            creator, job_close = raw["creator_lifetime"], raw["job_close"]
            need(creator["exact_job_query_succeeded"] is True and creator["in_exact_job"] is False
                 and not creator["handle_flags"] & 1 and creator["source_known_duplicate_handles"] == 0
                 and creator["source_known_inherited_handles"] == 0 and job_close["checked"] is True,
                 "prior-exact-Job-lifetime")
            checks = raw["closure_checks"]
            need(len(checks) == 2 and checks[0]["perf_counter_ns"] < checks[1]["perf_counter_ns"]
                 and all(x["members"] == [] and x["run_nonce"] == raw["run_nonce"] for x in checks),
                 "prior-two-positive-empty-queries")
            need(raw["stdio"] and all(x["complete"] and x["eof"] for x in raw["stdio"].values()),
                 "prior-stdio-complete")
            for ref in prior["raw_files"]:
                self.read_ref(ref, False)
            runs.add(raw["run_nonce"])
        need(type(self.a["prior_jobs"]) is list and len(self.a["prior_jobs"]) <= 64, "prior-job-count")
        for port in catalog["ports"]:
            closed(port, {"name", "source", "function", "ast_sha256", "last_run", "closed_grant"}, "writer-port")
            source = self.read_ref(port["source"])
            tree = ast.parse(source)
            nodes = [n for n in ast.walk(tree) if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))
                     and n.name == port["function"]]
            need(len(nodes) == 1 and hashlib.sha256(ast.dump(nodes[0], include_attributes=False).encode()).hexdigest()
                 == port["ast_sha256"] and port["last_run"] in runs, "writer-source-run-binding")
            grant = decode(self.read_ref(port["closed_grant"]))
            need(grant["root"] == catalog["root"] and grant["run_nonce"] == port["last_run"]
                 and grant["freeze_nonce"] == catalog["freeze_nonce"] and grant["state"] == "revoked",
                 "writer-grant-not-revoked")
        self.writers = catalog

    def validate_protected(self):
        closed(self.protected, {"version", "namespace", "roots", "review"}, "protected-scope")
        need(type(self.protected["version"]) is int and self.protected["version"] == 1
             and type(self.protected["namespace"]) is str and 0 < len(self.protected["namespace"]) <= 256
             and type(self.protected["roots"]) is list
             and 0 < len(self.protected["roots"]) <= 256, "protected-root-count")
        self.read_ref(self.protected["review"])
        self.protected_pins = []
        for row in self.protected["roots"]:
            closed(row, {"path", "identity", "mount"}, "protected-root")
            pin = Pin(self.n, row["path"])
            self.protected_pins.append(pin)
            need(pin.meta["identity"] == row["identity"] and pin.meta["mount"] == row["mount"],
                 "protected-native-identity")

    def native_scope(self, *, path, handle, identity, parent_path, parent_handle,
                     root, protected_roots, plan_sha256):
        self.check(phase="capture", plan_sha256=plan_sha256, prospective_bytes=0, roots=[root])
        node = self.n.meta(handle)
        need(node["identity"] == identity, "node-ID")
        self.n.streams(handle)
        owned_parent, top = None, None
        if parent_handle is None:
            owned_parent = Pin(self.n, parent_path)
            parent_handle = owned_parent.handle
        try:
            parent = self.n.meta(parent_handle)
            top = self.root_pin(root)
            need(parent["directory"] and not parent["attrs"] & 0x400
                 and node["mount"] == parent["mount"] == top.meta["mount"], "mounted-node-or-parent")
            canonical = node["path"].casefold()
            base = top.meta["path"].rstrip("\\").casefold()
            need(canonical == base or canonical.startswith(base + "\\"), "outside-root")
            for pin in self.protected_pins:
                protected = pin.meta["path"].rstrip("\\").casefold()
                need(not (canonical == protected or canonical.startswith(protected + "\\")
                     or protected.startswith(canonical.rstrip("\\") + "\\")), "protected-alias")
                need(node["identity"] != pin.meta["identity"], "protected-native-ID")
            if (not node["directory"] or node["attrs"] & 0x400) and node["links"] > 1:
                self.alias_root = root["path"]
                self.aliases(path, node, base)
            return dict(go=True, epoch=time.time(), backend="win32-pinned-volume-reparse",
                namespace=self.protected["namespace"], mount=node["mount"], parent_mount=parent["mount"],
                root_mount=top.meta["mount"], root_identity=root["identity"], node_identity=identity,
                parent_identity=parent["identity"], protected_roots_sha256=hashlib.sha256(encode(protected_roots)).hexdigest(),
                protected_scope_sha256=self.scope_sha(self.plan(plan_sha256)), source_pin=ENGINE,
                guard_sha256=self.m["parent_capture_sha256"], freeze_nonce=self.plan(plan_sha256)["namespace_freeze_nonce"],
                no_protected_alias=True, provider_sha256=self.m["context_source_sha256"])
        finally:
            if top:
                top.close()
            if owned_parent:
                owned_parent.close()

    def aliases(self, path, node, base):
        size, text = w.DWORD(32768), c.create_unicode_buffer(32768)
        search = self.n.k.FindFirstFileNameW(str(path), 0, c.byref(size), text)
        need(search not in (None, c.c_void_p(-1).value), "native-alias-enumeration")
        seen = set()
        try:
            while True:
                self.poll()
                need(len(seen) < self.limits["max_aliases"], "alias-count")
                alias = node["path"][:node["path"].find("}\\") + 1] + text.value
                need(alias.casefold() not in seen and alias.casefold().startswith(base + "\\"), "external-alias")
                seen.add(alias.casefold())
                # Open via the admitted DOS root, not an unchecked volume/device path.
                root = self.writers["root"]
                suffix = alias[len(base):].lstrip("\\")
                pin = Pin(self.n, ntpath.join(self.alias_root, suffix), share=5, allow_reparse=True)
                try:
                    need(pin.meta["identity"] == node["identity"] and pin.meta["mount"] == node["mount"], "alias-ID")
                finally:
                    pin.close()
                size.value = len(text)
                if not self.n.k.FindNextFileNameW(search, c.byref(size), text):
                    need(c.get_last_error() == 38, "incomplete-alias-enumeration")
                    break
            need(len(seen) == node["links"], "unresolved-alias-closure")
        finally:
            if not self.n.k.FindClose(search):
                self.n.uncertain.append(("alias-search", search))
                raise Refusal("alias-search-close")


    def root_pin(self, root):
        pin = Pin(self.n, root["path"], share=5)
        try:
            need(pin.meta["directory"] and pin.meta["identity"] == root["identity"],
                 "root-native-ID")
            return pin
        except BaseException:
            pin.close()
            raise


    def plan(self, digest):
        if digest not in self.plans:
            if self.m["action"] == "CONTROL":
                need(self.control_case is not None, "no-active-control")
                found = []
                for name in ("capture-plan.json", "parent-retirement-permit.json"):
                    path = ntpath.join(self.control_case["root"], name)
                    if os.path.isfile(path):
                        need(os.stat(path).st_size <= 131072, "control-plan-cap")
                        with open(path, "rb") as stream:
                            candidate = stream.read(131073)
                        raw = self.read_ref({"path": path, "bytes": len(candidate), "sha256": digest}) if hashlib.sha256(candidate).hexdigest() == digest else None
                        if raw is not None:
                            found.append(decode(raw))
                need(len(found) == 1, "unregistered-control-plan")
                plan = found[0]
                scope = self.control_scopes[self.control_case["case_id"]]
                template = decode(encode(scope["template"]))
                if plan["operation"] == "RETIRE":
                    need(self.control_case["case_id"] in {"C29-exact-retirement"} | NATIVE_CASES,
                         "control-retirement-case")
                    template["operation"], template["archive_sha256"] = "RETIRE", plan["archive_sha256"]
                    need(is_sha(plan["archive_sha256"]), "control-archive-SHA")
                if self.control_case["case_id"] == "C28-archive-cap":
                    template["limits"]["archive_bytes"] = 24
                if self.control_case["case_id"] == "C26-protected-root":
                    template["protected_roots"] = [template["roots"][0]["path"]]
                need(encode(plan) == encode(template), "control-plan-substitution")
                self.plans[digest] = plan
            else:
                ref = self.a["plan"]
                need(ref["sha256"] == digest, "unregistered-plan")
                self.plans[digest] = decode(self.read_ref(ref))
        return self.plans[digest]

    def scope_sha(self, plan):
        if self.m["action"] == "CONTROL":
            scope = self.control_scopes[self.control_case["case_id"]]
            need(plan["roots"] == [scope["root"]], "control-native-scope-root")
            return scope["ref"]["sha256"]
        return self.a["protected_scope"]["sha256"]

    def check(self, *, phase, plan_sha256, prospective_bytes, roots):
        self.lease.check()
        if self.actors and getattr(self, "active_native_actor", {}).get("kind") == "writer":
            limit = time.perf_counter() + 1
            while self.active_native_actor.get("last_write_perf_ns", 0) < self.active_verify_begin:
                need(time.perf_counter() < limit, "native-mutator-overlap-deadline")
                self.poll(); time.sleep(0.001)
        need(not self.actors, "active-native-writer-or-io")
        fault = getattr(self, "native_fault", None)
        if fault and phase == "retire" and not os.path.lexists(fault["absent"]):
            handle = self.n.k.CreateFileW(fault["absent"], 0, 7, None, 3, 0x02200000, None)
            error = c.get_last_error()
            if handle not in (None, c.c_void_p(-1).value):
                self.n.close(handle)
                raise Refusal("midway-negative-absence-not-observed")
            need(error == 2, "midway-absence-not-positive-Win32-error2")
            fault["observed"] = True
            if fault["case_id"] == "C31-crash-journal":
                self.c31_checkpoint(fault)
            raise Refusal("source-owned-midway-stop-after-native-absence")
        need(self.n.k.WaitForSingleObject(self.parent_handle, 0) == 258, "outer-guardian-dead")
        members = self.n.members(self.limits["max_members"])
        need(members == {os.getpid()}, "other-live-Job-member")
        plan = self.plan(plan_sha256)
        need(plan["source_pin"] == ENGINE and plan["guard_sha256"] == self.m["parent_capture_sha256"]
             and (self.m["action"] == "CONTROL" or plan["namespace_freeze_nonce"] == self.writers["freeze_nonce"])
             and roots == plan["roots"] and len(roots) == 1
             and (self.m["action"] == "CONTROL" or roots[0]["path"] == self.writers["root"]),
             "exact-plan-writer-scope")
        need(plan["native_scope_sha256"] == self.m["context_source_sha256"], "scope-source-binding")
        if self.m["action"] != "CONTROL":
            need(plan["control_only"] is False, "toy-review-cannot-qualify-REAL")
            self.qualification()
        else:
            need(plan["control_only"] is True, "CONTROL-only-plan")
        need(integer(prospective_bytes, 0, plan["limits"]["archive_bytes"] + 1048576), "prospective-bound")
        self.charged_archive = max(self.charged_archive, prospective_bytes)
        retiring = phase == "retire"
        if retiring:
            need((self.m["action"] == "RETIRE" and self.m["real_retire_go"] is True)
                 or (self.m["action"] == "CONTROL" and self.control_case["case_id"]
                     in {"C29-exact-retirement"} | NATIVE_CASES and plan["control_only"] is True),
                 "no-separate-retirement-GO")
            self.archive_proof(plan)
        return dict(go=True, epoch=time.time(), phase=phase, source_pin=ENGINE, host="win32",
            freeze_nonce=plan["namespace_freeze_nonce"], guard_sha256=self.m["parent_capture_sha256"],
            prior_groups_empty=True, io_closed=True, handles_closed=True, namespace_frozen=True,
            references_frozen=True, archive_protected=retiring, native_delete_reviewed=retiring)


    def qualification(self):
        # Actual Windows producer emits native-case-index-v1/ranged raw frames;
        # case_ledger.required_cases("Windows") defines the exact166 catalog.
        # The former arbitrary>=166 aggregate had no matching producer and could
        # not authenticate units/oracles/censors. It grants no REAL authority.
        raise Refusal("PENDING:literal-Windows166-unit-oracle-negative-raw-qualification-adapter")

    def archive_proof(self, plan):
        need(plan["operation"] == "RETIRE" and is_sha(plan["archive_sha256"]), "retire-permit-binding")
        path = ntpath.join(plan["artifact_root"], "archive.gz")
        info = os.stat(path, follow_symlinks=False)
        self.read_ref({"path": path, "bytes": info.st_size, "sha256": plan["archive_sha256"]}, False)
        need(info.st_size <= self.limits["archive_bytes"], "archive-owner-cap")
        # The helper itself strictly re-reads the full stream and source before any unlink.


    def census(self, root):
        root = absolute(root)
        total = files = 0
        if not os.path.lexists(root):
            parent = Pin(self.n, str(Path(root).parent))
            parent.close()
            return 0, 0
        root_info = os.lstat(root)
        need(not getattr(root_info, "st_file_attributes", 0) & 0x400, "budget-root-reparse")
        for directory, dirs, names in os.walk(root, followlinks=False):
            self.poll()
            for name in list(dirs) + names:
                path = ntpath.join(directory, name)
                info = os.lstat(path)
                if getattr(info, "st_file_attributes", 0) & 0x400:
                    base = ntpath.normcase(self.limits["control_root"]).rstrip("\\")
                    need(self.m["action"] == "CONTROL" and ntpath.normcase(path).startswith(base + "\\"),
                         "budget-reparse-outside-disposable-controls")
                    if name in dirs:
                        dirs.remove(name)
                    total += max(info.st_size, 16384)
                    files += 1
                elif name in names:
                    total += info.st_size
                    files += 1
                    if self.m["action"] == "CONTROL" and path == ntpath.join(self.limits["control_root"], "C18-native-reparse-fork", "source", "sample.bin"):
                        try:
                            extra = os.stat(path + ":native-fork", follow_symlinks=False)
                        except FileNotFoundError:
                            pass
                        else:
                            need(extra.st_size <= 65536, "control-ADS-budget-bound")
                            total += extra.st_size
                            files += 1
        return total, files


    def check_budget(self, prospective_bytes, prospective_files):
        closed(self.budget, {"version", "root_caps", "review"}, "Context-budget")
        need(self.budget["version"] == 1 and type(self.budget["root_caps"]) is list
             and len(self.budget["root_caps"]) <= 32, "Context-budget-version")
        self.read_ref(self.budget["review"])
        if not hasattr(self, "budget_tickets"):
            self.budget_tickets = {}
        for rule in self.budget["root_caps"]:
            closed(rule, {"root", "bytes", "files", "reserve_bytes", "reserve_files"}, "root-budget")
            need(all(integer(rule[k], 0, 2**46) for k in
                 ("bytes", "files", "reserve_bytes", "reserve_files")), "budget-bound")
            actual, files = self.census(rule["root"])
            key = rule["root"]
            if key not in self.budget_tickets:
                need(actual + rule["reserve_bytes"] <= rule["bytes"]
                     and files + rule["reserve_files"] <= rule["files"], "initial-coexistence-budget")
                self.budget_tickets[key] = [actual, files, actual, files]
            ticket = self.budget_tickets[key]
            ticket[2], ticket[3] = max(ticket[2], actual), max(ticket[3], files)
            need(ticket[2] <= ticket[0] + rule["reserve_bytes"] <= rule["bytes"]
                 and ticket[3] <= ticket[1] + rule["reserve_files"] <= rule["files"], "phase-ticket-exceeded")
        need(integer(prospective_bytes, 0, self.limits["body_bytes"])
             and integer(prospective_files, 0, self.limits["body_files"]), "control-prospective-bound")
        free = c.c_ulonglong()
        self.n.k.GetDiskFreeSpaceExW.argtypes = [w.LPCWSTR, c.c_void_p, c.c_void_p, c.c_void_p]
        self.n.k.GetDiskFreeSpaceExW.restype = w.BOOL
        need(self.n.k.GetDiskFreeSpaceExW("C:\\", c.byref(free), None, None)
             and free.value >= 140 * 1024**3, "Windows-free-floor")

    def archive(self):
        if self.archive_module is None:
            import types
            module = types.ModuleType("owned_archive_" + self.a["archive_source"]["sha256"][:12])
            module.__file__ = self.a["archive_source"]["path"]
            exec(compile(self.archive_raw, module.__file__, "exec"), module.__dict__)
            self.archive_module = module
        return self.archive_module


    def prepare_control_plan(self, prepared):
        self.lease.check()
        case = self.control_case
        need(self.m["action"] == "CONTROL" and case is not None, "control-constructor-authority")
        path = ntpath.join(case["root"], "source")
        need(prepared["roots"][0]["path"] == path and len(prepared["roots"]) == 1
             and prepared["artifact_root"] == ntpath.join(case["root"], "archive")
             and prepared["protected_roots"] == [ntpath.join(case["root"], "protected")]
             and prepared["namespace_freeze_nonce"] == case["case_id"]
             and prepared["control_only"] is True and prepared["source_pin"] == ENGINE
             and prepared["native_scope_sha256"] == self.m["context_source_sha256"]
             and prepared["guard_sha256"] == self.m["parent_capture_sha256"]
             and prepared["tool_sha256"] == self.a["archive_source"]["sha256"],
             "control-constructor-binding")
        pin, parent = self.root_pin(prepared["roots"][0]), None
        try:
            parent = Pin(self.n, case["root"])
            need(pin.meta["mount"] == parent.meta["mount"], "control-mounted-root")
            absent = prepared["protected_roots"][0]
            handle = self.n.k.CreateFileW(absent, 0, 7, None, 3, 0x02200000, None)
            error = c.get_last_error()
            if handle not in (None, c.c_void_p(-1).value):
                self.n.close(handle)
                raise Refusal("control-protected-prefix-exists")
            need(error == 2, "control-protected-absence-unproved")
            raw = encode(dict(version=1, control_only=True, root=prepared["roots"][0],
                root_native=pin.meta, parent_native=parent.meta, protected_absent=absent,
                absent_winerror=error, namespace=self.protected["namespace"],
                prior_jobs=self.a["prior_jobs"], writer_catalog=self.a["writer_catalog"],
                context_source_sha256=self.m["context_source_sha256"],
                guard_sha256=self.m["parent_capture_sha256"], source_pin=ENGINE,
                lease_acquired_perf_ns=self.lease.check(), pid=os.getpid(),
                job_members=sorted(self.n.members(self.limits["max_members"])),
                nonce=prepared["namespace_freeze_nonce"]))
            fact = self.control_write("native-scope.json", raw)
            ref = {k: fact[k] for k in ("path", "sha256")}
            prepared["evidence"]["quiescence"] = [ref]
            self.control_scopes[case["case_id"]] = dict(root=prepared["roots"][0], ref=ref,
                                                        template=decode(encode(prepared)))
            self.read_ref(dict(ref, bytes=len(raw)))
            return prepared
        finally:
            pin.close()
            if parent:
                parent.close()

    def admit_controls(self, *, expected, forecast):
        closed(expected, {"phase", "root", "case_id", "deadline_epoch", "archive_sha256",
                          "source_pin", "guard_sha256", "scope_sha256", "forecast_sha256", "control_only"},
               "control-batch-expectation")
        need(expected["phase"] == "batch" and expected["case_id"] == "BATCH",
             "control-batch-phase")
        need(self.m["action"] == "CONTROL", "no-CONTROL-authority")
        self.lease.check()
        need(expected["control_only"] is True and expected["source_pin"] == ENGINE
             and expected["scope_sha256"] == self.m["context_source_sha256"]
             and expected["archive_sha256"] == self.a["archive_source"]["sha256"]
             and absolute(expected["root"]) == self.limits["control_root"], "control-batch-binding")
        need(forecast["cells"] == 38 and forecast["engine_qualification"] is False
             and expected["guard_sha256"] == self.m["parent_capture_sha256"]
             and expected["forecast_sha256"] == hashlib.sha256(json.dumps(forecast, sort_keys=True).encode()).hexdigest()
             and time.time() < expected["deadline_epoch"] <= self.m["deadline_epoch"], "control-catalog")
        self.check_budget(forecast["fixture_archive_restore_partial_bytes"],
                          forecast["fixture_archive_restore_partial_files"])
        self.control_deadline = expected["deadline_epoch"]
        self.batch_expected = dict(expected)
        self.case_reserve_bytes, self.case_reserve_files = 0, 0
        return dict(expected, go=True, epoch=time.time())

    def before_control_write(self, *, expected):
        self.lease.check()
        need(self.batch_expected is not None, "no-control-batch")
        closed(expected, set(self.batch_expected) | {"prospective_bytes", "prospective_files"},
               "control-case-expectation")
        need(all(expected[k] == self.batch_expected[k] for k in
             set(self.batch_expected) - {"phase", "root", "case_id"})
             and expected["case_id"] in self.control_case_ids
             and expected["root"] == ntpath.join(self.limits["control_root"], expected["case_id"]),
             "control-case-exact-batch")
        need(self.m["action"] == "CONTROL" and expected["phase"] == "case"
             and ntpath.dirname(expected["root"]) == self.limits["control_root"], "control-case-root")
        need(self.batch_expected is not None and expected["case_id"] not in self.control_scopes, "case-once")
        self.case_reserve_bytes += expected["prospective_bytes"]
        self.case_reserve_files += expected["prospective_files"]
        need(self.case_reserve_bytes <= self.limits["body_bytes"]
             and self.case_reserve_files <= self.limits["body_files"], "control-catalog-reservation")
        self.check_budget(expected["prospective_bytes"], expected["prospective_files"])
        self.control_case = dict(expected)
        return dict(expected, go=True, epoch=time.time())



    def control_write(self, name, raw, cap=16384):
        need(self.m["action"] == "CONTROL" and self.control_case is not None
             and name in {"native-scope.json", "capture-plan.json", "parent-retirement-permit.json",
                          "native-control-receipt.json", "negative-archive.py", "negative-authority.json",
                          "negative-manifest.json", "fault-checkpoint.json", "recovery-proof.json"}
             and type(raw) is bytes and len(raw) <= cap <= 131072, "bounded-control-sink")
        self.check_budget(0, 0)
        path = ntpath.join(self.control_case["root"], name)
        owners = 0
        for rule in self.budget["root_caps"]:
            prefix = ntpath.normcase(rule["root"]).rstrip("\\") + "\\"
            if ntpath.normcase(path).startswith(prefix):
                ticket = self.budget_tickets[rule["root"]]
                need(ticket[2] + len(raw) <= ticket[0] + rule["reserve_bytes"]
                     and ticket[3] + 1 <= ticket[1] + rule["reserve_files"], "control-sink-ticket")
                owners += 1
        need(owners, "control-sink-no-owner")
        with open(path, "xb") as stream:
            stream.write(raw); stream.flush(); os.fsync(stream.fileno())
        return dict(path=path, bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())

    def control_snapshot(self, path):
        """Bounded no-follow forensic snapshot of this disposable control only."""
        root = Path(path)
        need(self.control_case is not None and str(root) == ntpath.join(self.control_case["root"], "source"),
             "snapshot-outside-disposable-case")
        rows, count, total = [], 0, 0
        for directory, dirs, files in os.walk(root, followlinks=False):
            for name in sorted(dirs + files):
                self.poll()
                item = Path(directory) / name
                info = item.lstat()
                count += 1
                need(count <= 128, "control-snapshot-entries")
                row = [str(item.relative_to(root)), info.st_mode, info.st_size, info.st_ino, info.st_nlink]
                if getattr(info, "st_file_attributes", 0) & 0x400:
                    row.append(os.readlink(item))
                    if name in dirs:
                        dirs.remove(name)
                elif name in files:
                    total += info.st_size
                    need(total <= 2 * 1024**2, "control-snapshot-bytes")
                    digest = hashlib.sha256()
                    with open(item, "rb") as stream:
                        for block in iter(lambda: stream.read(CHUNK), b""):
                            self.poll(); digest.update(block)
                    row.append(digest.hexdigest())
                rows.append(row)
        return hashlib.sha256(encode(sorted(rows))).hexdigest()

    def native_negative(self, callback, allowed):
        try:
            callback()
        except (Refusal, self.native_module.NoGo) as error:
            reason = str(error)
            need(any(x in reason for x in allowed), "native-unexpected-refusal:" + reason[:128])
            return dict(type=type(error).__name__, reason=reason[:512])
        raise Refusal("native-fault-did-not-refuse")

    def control_permit(self, plan, digest):
        import copy
        permit = copy.deepcopy(plan)
        permit["operation"], permit["archive_sha256"] = "RETIRE", digest
        raw = encode(permit) + b"\n"
        ref = self.control_write("parent-retirement-permit.json", raw, 131072)
        return ref["path"], ref["sha256"]

    def native_actor(self, kind, source):
        """Source-owned finite actor. No callback, executable, argv or plugin port."""
        ready, stop = threading.Event(), threading.Event()
        result = {"kind": kind, "closed": False, "bytes": 0, "error": None}
        pipe = [w.HANDLE(), w.HANDLE()]
        thread_handle = None
        if kind == "pipe":
            self.n.k.CreatePipe.argtypes = [c.POINTER(w.HANDLE), c.POINTER(w.HANDLE), c.c_void_p, w.DWORD]
            self.n.k.CreatePipe.restype = w.BOOL
            need(self.n.k.CreatePipe(c.byref(pipe[0]), c.byref(pipe[1]), None, 0), "native-pipe-create")
        else:
            need(kind == "writer", "closed-native-actor-kind")
        def body():
            handle = None
            try:
                if kind == "writer":
                    handle = self.n.open(str(source / "sample.bin"), 0x40000080, 7)
                    self.n.k.WriteFile.argtypes = [w.HANDLE, c.c_void_p, w.DWORD,
                                                   c.POINTER(w.DWORD), c.c_void_p]
                    self.n.k.WriteFile.restype = w.BOOL
                    data, used = c.create_string_buffer(b"native-inflight-mutator"), w.DWORD()
                    need(self.n.k.WriteFile(handle, data, len(data) - 1, c.byref(used), None),
                         "native-mutator-write")
                    result["bytes"] = used.value
                    need(used.value == len(data) - 1, "native-mutator-short-write")
                    ready.set()
                    result["first_write_perf_ns"] = time.perf_counter_ns()
                    while not stop.wait(0.001):
                        need(time.time() < self.m["deadline_epoch"], "native-actor-deadline")
                        self.n.k.SetFilePointerEx.argtypes = [w.HANDLE, c.c_longlong, c.c_void_p, w.DWORD]
                        self.n.k.SetFilePointerEx.restype = w.BOOL
                        need(self.n.k.SetFilePointerEx(handle, 0, None, 0), "native-mutator-seek")
                        need(self.n.k.WriteFile(handle, data, len(data) - 1, c.byref(used), None),
                             "native-mutator-inflight-write")
                        result["bytes"] += used.value
                        result["last_write_perf_ns"] = time.perf_counter_ns()
                else:
                    ready.set()
                    data, used = c.create_string_buffer(1), w.DWORD()
                    ok = self.n.k.ReadFile(pipe[0], data, 1, c.byref(used), None)
                    result["read_ok"], result["read_error"] = bool(ok), c.get_last_error() if not ok else 0
                    need(not ok and result["read_error"] == 995, "native-read-not-cancelled")
            except BaseException as error:
                result["error"] = repr(error)[:512]
            finally:
                if handle is not None:
                    try:
                        self.n.close(handle)
                    except BaseException as error:
                        result["error"] = repr(error)[:512]
                result["closed"] = not self.n.uncertain
                ready.set()
        thread = threading.Thread(target=body, daemon=False, name="owned-native-" + kind)
        self.actors.append(thread)
        thread.start()
        need(ready.wait(1) and thread.is_alive() and result["error"] is None, "native-actor-not-ready")
        if kind == "pipe":
            self.n.k.OpenThread.argtypes, self.n.k.OpenThread.restype = [w.DWORD, w.BOOL, w.DWORD], w.HANDLE
            self.n.k.GetThreadIOPendingFlag.argtypes = [w.HANDLE, c.POINTER(w.BOOL)]
            self.n.k.GetThreadIOPendingFlag.restype = w.BOOL
            self.n.k.CancelSynchronousIo.argtypes, self.n.k.CancelSynchronousIo.restype = [w.HANDLE], w.BOOL
            thread_handle = self.n.k.OpenThread(0x100041, False, thread.native_id)
            need(thread_handle, "native-IO-thread-open")
            pending, end = w.BOOL(), time.perf_counter() + 1
            while time.perf_counter() < end:
                need(self.n.k.GetThreadIOPendingFlag(thread_handle, c.byref(pending)), "native-IO-pending-query")
                if pending.value:
                    break
                time.sleep(0.005)
            need(pending.value, "native-blocked-IO-not-observed")
            result["native_pending_observed"] = True
        return thread, stop, pipe, thread_handle, result

    def close_native_actor(self, actor):
        thread, stop, pipe, thread_handle, result = actor
        stop.set()
        if thread_handle:
            need(self.n.k.CancelSynchronousIo(thread_handle), "native-cancel-sync-IO")
        thread.join(1)
        need(not thread.is_alive(), "native-actor-did-not-close")
        for handle in [thread_handle] + [x.value for x in pipe]:
            if handle:
                self.n.close(handle)
        need(result["closed"] is True and result["error"] is None, "native-actor-closure-failed")
        self.actors.remove(thread)
        return result

    def exercise_native_control(self, **request):
        """Actual fixed native faults; no evidence-only PASS or external executor."""
        closed(request, {"case_id", "mode", "archive_module", "plan_path", "plan_sha256", "plan"},
               "native-request")
        A, plan = request["archive_module"], request["plan"]
        case_id, mode = request["case_id"], request["mode"]
        expected_modes = dict(zip(sorted(NATIVE_CASES), [
            "reparse-fork", "ancestor", "inflight-mutation", "active-io", "midway-stop",
            "crash-journal", "sharing", "protected-bind-alias", "special-alias-closure",
            "negative-admission-before-import", "same-buffer-review-substitution"]))
        expected_modes["C17-fifo"] = "windows-fifo-applicability-v1"
        need(self.m["action"] == "CONTROL" and self.control_case is not None
             and case_id == self.control_case["case_id"] and expected_modes.get(case_id) == mode,
             "native-case-binding")
        need(A.__file__ == self.a["archive_source"]["path"]
             and plan == self.plan(request["plan_sha256"]), "native-source-plan-binding")
        source, case = Path(plan["roots"][0]["path"]), Path(self.control_case["root"])
        need(source == case / "source", "native-disposable-prefix")
        self.lease.check()
        self.native_module = A
        handles = self.n.handle_count()
        owned = sum(len(pin.handles) for pin, raw in self.refs.values())
        protected = [self.n.meta(pin.handle) for pin in self.protected_pins]
        before = self.control_snapshot(source)
        events, unsupported, actor = [], False, None
        if mode == "windows-fifo-applicability-v1":
            need(os.name == "nt" and not hasattr(os, "mkfifo"), "Windows-fifo-applicability")
            baseline = A.archive(request["plan_path"], request["plan_sha256"], self)
            backup = case / "archive/archive.gz"
            backup_sha = baseline["compressed_sha256"]
            self.n.k.CreateNamedPipeW.argtypes = [w.LPCWSTR, w.DWORD, w.DWORD, w.DWORD,
                                                 w.DWORD, w.DWORD, w.DWORD, c.c_void_p]
            self.n.k.CreateNamedPipeW.restype = w.HANDLE
            path = str(source / "negative-fifo")
            handle = self.n.k.CreateNamedPipeW(path, 3, 0, 1, 4096, 4096, 100, None)
            error = c.get_last_error()
            if handle not in (None, c.c_void_p(-1).value):
                self.n.close(handle)
                raise Refusal("disk-path-unexpectedly-created-native-pipe")
            need(error == 123, "disk-pipe-refusal-not-invalid-name")
            after = self.control_snapshot(source)
            need(before == after and hashlib.sha256(backup.read_bytes()).hexdigest() == backup_sha,
                 "unsupported-primitive-mutated-source-or-backup")
            raw = dict(policy_version=2, case_id=case_id, mode=mode, host=sys.platform,
                archive_sha256=plan["tool_sha256"], controls_sha256=self.a["controls_source"]["sha256"],
                context_sha256=plan["native_scope_sha256"], guard_sha256=plan["guard_sha256"],
                source_pin=plan["source_pin"], plan_sha256=request["plan_sha256"], root=plan["roots"][0],
                nonce=plan["namespace_freeze_nonce"], scope_ref=plan["evidence"]["quiescence"][0],
                scope="cpython-mkfifo-win32-disk-tree-v1", api="CreateNamedPipeW", path=path,
                handle="INVALID_HANDLE_VALUE", winerror=error, os_mkfifo_absent=True,
                before_sha256=before, after_sha256=after)
            unsupported = True
        elif mode == "crash-journal":
            raw = dict(version=1, case_id=case_id, mode=mode,
                batch_plan_sha256=request["plan_sha256"], actual_proofs=self.c31_consume())
        else:
            proof = A.archive(request["plan_path"], request["plan_sha256"], self)
            archive = case / "archive" / "archive.gz"
            archive_sha = proof["compressed_sha256"]
            need(hashlib.sha256(archive.read_bytes()).hexdigest() == archive_sha, "native-baseline-archive")
            loaded = A.load_plan(request["plan_path"], request["plan_sha256"], "ARCHIVE")
            scan = lambda: list(A.source_events(loaded, self))
            verify = lambda: A.verify(request["plan_path"], request["plan_sha256"], archive_sha, self)
            if mode == "reparse-fork":
                ads = str(source / "sample.bin") + ":native-fork"
                payload = b"native ADS fault"
                with open(ads, "xb") as stream:
                    need(stream.write(payload) == len(payload), "native-ADS-short-write")
                    stream.flush(); os.fsync(stream.fileno())
                need(os.stat(ads, follow_symlinks=False).st_size == len(payload), "native-ADS-size")
                events.append(dict(api="named-NTFS-data-stream", path=ads, bytes=len(payload)))
                events.append(self.native_negative(verify, ["unsupported-ADS", "nondefault-ADS"]))
            elif mode == "ancestor":
                retained = case / "retained-source"
                os.rename(source, retained)
                os.symlink(retained, source, target_is_directory=True)
                opened = self.n.open(str(source))
                try:
                    actual = self.n.meta(opened)
                    need(actual["attrs"] & 0x400, "actual-linked-root-not-reparse")
                    events.append(dict(api="CreateSymbolicLinkW", root_native=actual))
                finally:
                    self.n.close(opened)
                events.append(self.native_negative(verify, ["root", "reparse", "identity"]))
            elif mode in {"inflight-mutation", "active-io"}:
                actor = self.native_actor("writer" if mode == "inflight-mutation" else "pipe", source)
                try:
                    begin = time.perf_counter_ns()
                    self.active_native_actor = actor[4]
                    self.active_verify_begin = begin
                    events.append(dict(verify_begin_perf_ns=begin))
                    events.append(self.native_negative(verify, ["active-native-writer-or-io"]))
                    end = time.perf_counter_ns()
                    if mode == "inflight-mutation":
                        need(begin <= actor[4].get("last_write_perf_ns", 0) <= end,
                             "native-mutator-no-overlap-with-verification")
                    events.append(dict(verify_end_perf_ns=end))
                finally:
                    events.append(self.close_native_actor(actor)); actor = None
            elif mode in {"midway-stop", "sharing"}:
                permit_path, permit_sha = self.control_permit(plan, archive_sha)
                blocker = None
                try:
                    if mode == "sharing":
                        blocker = Pin(self.n, str(source / "sample.bin"))
                        handle = self.n.k.CreateFileW(str(source / "sample.bin"), 0x80010080, 1,
                                                       None, 3, 0x02200000, None)
                        error = c.get_last_error()
                        if handle not in (None, c.c_void_p(-1).value):
                            self.n.close(handle)
                            raise Refusal("sharing-blocker-did-not-deny-DELETE")
                        need(error == 32, "native-sharing-error-not32")
                        events.append(dict(api="CreateFileW", delete_access=True, winerror=error))
                    else:
                        self.native_fault = dict(case_id=case_id, absent=str(source / "empty-directory"),
                                                 observed=False)
                    retired = A.retire(permit_path, permit_sha, self)
                    need(retired["status"] == "PARTIAL" and source.exists()
                         and retired["archive_retained"] is True, "native-partial-retirement-not-observed")
                    if mode == "midway-stop":
                        need(retired["removed_entries_observed"] >= 1 and self.native_fault["observed"],
                             "midway-stop-before-observed-native-removal")
                    journal = case / "archive" / "retirement.journal"
                    raw_journal = journal.read_bytes()
                    need(0 < len(raw_journal) <= 1048576, "native-journal-bound")
                    position, kinds = 0, []
                    while position < len(raw_journal):
                        need(position + 4 <= len(raw_journal), "native-journal-frame")
                        size = struct.unpack_from("<I", raw_journal, position)[0]; position += 4
                        need(0 < size <= 512 and position + size <= len(raw_journal), "native-journal-length")
                        kinds.append(decode(raw_journal[position:position+size])["kind"]); position += size
                    need(kinds[0] == "intent" and kinds[-1] == "stop", "native-partial-journal")
                    events.append(dict(result=retired, journal_sha256=hashlib.sha256(raw_journal).hexdigest(),
                                       journal_kinds=kinds))
                finally:
                    self.native_fault = None
                    if blocker:
                        blocker.close()
            elif mode == "protected-bind-alias":
                target = case / "protected"
                os.link(source / "sample.bin", target)
                digest = hashlib.sha256(target.read_bytes()).hexdigest()
                events.append(self.native_negative(scan, ["external-alias", "protected"]))
                need(hashlib.sha256(target.read_bytes()).hexdigest() == digest, "protected-alias-changed")
                events.append(dict(protected_sha256=digest))
            elif mode == "special-alias-closure":
                link, outside = source / "native-special", case / "outside-special"
                os.symlink(source / "sample.bin", link)
                os.link(link, outside, follow_symlinks=False)
                handle = self.n.open(str(link))
                try:
                    meta = self.n.meta(handle)
                    need(meta["attrs"] & 0x400 and meta["links"] >= 2, "native-hardlink-to-reparse-not-created")
                    events.append(dict(native_reparse=meta))
                finally:
                    self.n.close(handle)
                events.append(self.native_negative(scan, ["external-alias", "hardlink"]))
            elif mode == "negative-admission-before-import":
                marker = case / "MUST-NOT-EXECUTE"
                canary = case / "negative-archive.py"
                self.control_write("negative-archive.py", ("open(" + repr(str(marker)) + ", 'xb').close()\n").encode())
                authority = decode(encode(self.a))
                body = canary.read_bytes()
                authority["archive_source"] = dict(path=str(canary), bytes=len(body),
                                                     sha256=hashlib.sha256(body).hexdigest())
                authority_path = case / "negative-authority.json"
                body = encode(authority); self.control_write("negative-authority.json", body, 131072)
                manifest = decode(encode(self.m))
                manifest["runtime_go"] = False
                manifest["authority"] = dict(path=str(authority_path), bytes=len(body),
                                               sha256=hashlib.sha256(body).hexdigest())
                manifest_path = case / "negative-manifest.json"
                body = encode(manifest); self.control_write("negative-manifest.json", body, 65536)
                negative = Context.__new__(Context)
                try:
                    events.append(self.native_negative(
                        lambda: negative.__init__(str(manifest_path), hashlib.sha256(body).hexdigest()),
                        ["no-context-runtime-admission"]))
                finally:
                    negative.finish()
                need(not marker.exists(), "negative-admission-executed-provider")
                events.append(dict(marker_absent=True, negative_manifest_sha256=hashlib.sha256(body).hexdigest()))
            elif mode == "same-buffer-review-substitution":
                ref = plan["evidence"]["host_review"][0]
                raw_review, review_sha = A.reference_read(ref["path"], 131072, lambda: self.lease.check())
                need(review_sha == ref["sha256"], "native-review-initial-SHA")
                changed = decode(raw_review); changed["complete"] = False
                Path(ref["path"]).write_bytes(encode(changed))
                need(hashlib.sha256(raw_review).hexdigest() == review_sha
                     and decode(raw_review)["complete"] is True
                     and hashlib.sha256(Path(ref["path"]).read_bytes()).hexdigest() != review_sha,
                     "native-same-buffer-substitution-not-observed")
                events.append(self.native_negative(verify, ["evidence-sha"]))
                events.append(dict(original_buffer_sha256=review_sha,
                    substituted_path_sha256=hashlib.sha256(Path(ref["path"]).read_bytes()).hexdigest()))
            else:
                raise Refusal("unimplemented-native-case")
            need(hashlib.sha256(archive.read_bytes()).hexdigest() == archive_sha, "native-archive-changed")
            raw = dict(version=1, case_id=case_id, mode=mode, plan_sha256=request["plan_sha256"],
                context_sha256=self.m["context_source_sha256"], archive_sha256=archive_sha,
                before_source_sha256=before, events=events, archive_retained_sha256=archive_sha)
        self.lease.check()
        need(self.n.members(self.limits["max_members"]) == {os.getpid()}
             and not self.actors and not self.n.uncertain
             and self.n.handle_count() == handles + sum(len(pin.handles) for pin, body in self.refs.values()) - owned
             and [self.n.meta(pin.handle) for pin in self.protected_pins] == protected,
             "native-control-closure-or-protected-change")
        raw_bytes = encode(raw)
        need(len(raw_bytes) <= 32768, "native-control-receipt-cap")
        receipt = self.control_write("native-control-receipt.json", raw_bytes, 32768)
        return dict(status="WINDOWS_UNSUPPORTED_PRIMITIVE" if unsupported else "NATIVE_CONTROL_PASSED",
            passed=not unsupported, mode=mode, source_sha256=plan["tool_sha256"],
            raw_receipts=[{k: receipt[k] for k in ("path", "sha256")}],
            native_closure_proven=True, protected_unchanged=True, archive_retained=True)


    def c31_port(self):
        port = self.a["native_controls"]
        closed(port, {"version", "phase", "c31"}, "native-controls-port")
        need(type(port["version"]) is int and port["version"] == 1
             and port["phase"] in {"BATCH", "C31_FAULT", "C31_RECOVERY"}, "fixed-native-phase")
        q = port["c31"]
        closed(q, {"batch_root", "fixture_root", "nonce", "plan_ref", "scope_ref", "archive_ref",
                   "checkpoint_ref", "fault_capture_ref", "fault_lifetime_ref", "fault_completion_ref",
                   "recovery_ref", "recovery_capture_ref", "recovery_lifetime_ref",
                   "recovery_completion_ref", "review_ref"}, "C31-closed-port")
        self.read_ref(q["review_ref"])
        for key in ("batch_root", "fixture_root"):
            absolute(q[key])
        need(ntpath.basename(q["fixture_root"]) == "C31-crash-journal"
             and q["nonce"] == "C31-crash-journal"
             and not ntpath.normcase(q["fixture_root"]).startswith(ntpath.normcase(q["batch_root"]) + "\\"),
             "C31-independent-fixed-prefix")
        return port["phase"], q

    def control_fact(self, path, cap):
        pin = Pin(self.n, str(path), share=3)
        try:
            need(not pin.meta["directory"] and pin.meta["links"] == 1
                 and pin.meta["size"] <= cap, "control-fact-bound")
            self.n.streams(pin.handle)
            blocks, size = [], 0
            while True:
                block = self.n.read(pin.handle, CHUNK)
                if not block:
                    break
                size += len(block); need(size <= cap, "control-fact-growth")
                blocks.append(block)
            need(self.n.meta(pin.handle) == pin.meta, "control-fact-mutated")
            raw = b"".join(blocks)
            return dict(path=str(path), bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()), raw
        finally:
            pin.close()

    def c31_closure(self, q, which, checkpoint):
        raw_bytes = self.read_ref(q[which + "_capture_ref"])
        raw = decode(raw_bytes)
        life = decode(self.read_ref(q[which + "_lifetime_ref"]))
        completion = decode(self.read_ref(q[which + "_completion_ref"]))
        need(raw["guard_sha256"] == self.m["parent_capture_sha256"]
             and raw["process_closure_verified"] is True
             and completion["schema"] == "bounded-outer-caller-completion-v1"
             and completion["process_closure_verified"] is True
             and completion["receipt_ref"] == q[which + "_capture_ref"]
             and completion["outer_lifetime_ref"] == q[which + "_lifetime_ref"],
             "C31-actual-capture-return")
        need(life["schema"] == "outer-controller-native-lifetime-v1"
             and life["run_nonce"] == raw["run_nonce"]
             and life["guard_sha256"] == raw["guard_sha256"]
             and life["empty_observations"] == raw["closure_checks"]
             and life["job_close"] == raw["job_close"], "C31-outer-lifetime-binding")
        empties, job, host = raw["closure_checks"], raw["job_close"], raw["host"]
        need(len(empties) == 2 and empties[0]["perf_ns"] < empties[1]["perf_ns"]
             < job["completed_perf_ns"] and job["success"] is True
             and all(x["members"] == [] for x in empties),
             "C31-two-positive-empty-queries-and-last-Job-close")
        creator = raw["creator_lifetime"]
        need(creator == life["creator"] and creator["in_exact_job"] is False
             and creator["exact_job_query_succeeded"] is True
             and not creator["handle_flags"] & 1
             and creator["source_known_duplicate_handles"] == 0
             and creator["source_known_inherited_handles"] == 0, "C31-no-other-Job-owner")
        need(host["io_closed"] is True and host["writer_closed"] is True
             and host["callbacks_joined"] is True
             and completion["native_root_exit"]["wait_result"] == 0
             and completion["native_root_exit"]["handle_close_result"] is True
             and raw["rss_evidence"]["gzip_complete"] is True
             and all(x["complete"] is True and x["eof"] is True for x in raw["stdio"].values()),
             "C31-IO-handles-callbacks-closed")
        receipt = Path(q[which + "_capture_ref"]["path"])
        for name, ref in raw["stdio"].items():
            need(name in {"stdout", "stderr"}, "C31-stdio-name")
            self.read_ref(dict(path=str(receipt.with_suffix("." + name)),
                bytes=ref["retained_bytes"], sha256=ref["retained_sha256"]), False)
        ref = raw["rss_evidence"]
        self.read_ref(dict(path=str(receipt.with_suffix(".rss.jsonl.gz")),
            bytes=ref["bytes"], sha256=ref["gzip_sha256"]), False)
        if which == "fault":
            need("c31_native_kill" in raw, "PENDING:C31-fixed-G-native-kill-producer")
            kill = raw["c31_native_kill"]
            closed(kill, {"api", "success", "pid", "creation", "checkpoint_ref", "perf_ns", "exit_code", "wait_before", "job_members_before"},
                   "C31-native-kill-observation")
            need(kill["api"] == "TerminateJobObject" and kill["success"] is True
                 and kill["pid"] == checkpoint["pid"] and kill["creation"] == checkpoint["creation"]
                 and kill["checkpoint_ref"] == q["checkpoint_ref"]
                 and kill["exit_code"] == 3121 and host["exit_code"] == 3121
                 and kill["wait_before"] == 258 and kill["job_members_before"] == [checkpoint["pid"]]
                 and checkpoint["perf_counter_ns"] < kill["perf_ns"] < host["exit_observed_perf_ns"],
                 "C31-actual-outer-kill-after-checkpoint")
            need(raw["qualified"] is False and completion["qualified"] is False
                 and host["exit_code"] != 0 and host["pid"] == checkpoint["pid"]
                 and host["creation"] == checkpoint["creation"]
                 and checkpoint["perf_counter_ns"] < host["exit_observed_perf_ns"],
                 "C31-actual-expected-fault-not-healthy-PASS")
        else:
            need(host["pid"] == checkpoint["pid"] and host["creation"] == checkpoint["creation"],
                 "C31-recovery-native-process")
            need(completion["qualified"] is True and host["exit_code"] == 0,
                 "C31-recovery-guard-completed")
        need(self.lease.acquired_perf_ns > job["completed_perf_ns"], "C31-local-lease-before-old-closure")
        return raw["run_nonce"]

    def c31_checkpoint(self, fault):
        phase, q = self.c31_port()
        need(phase == "C31_FAULT" and self.control_case["root"] == q["fixture_root"],
             "C31-fault-phase-only")
        plan_ref, _ = self.control_fact(Path(q["fixture_root"]) / "capture-plan.json", 131072)
        permit_ref, _ = self.control_fact(Path(q["fixture_root"]) / "parent-retirement-permit.json", 131072)
        archive_ref, _ = self.control_fact(Path(q["fixture_root"]) / "archive/archive.gz", 1048576)
        journal_ref, journal = self.control_fact(Path(q["fixture_root"]) / "archive/retirement.journal", 1048576)
        need(journal and fault["observed"], "C31-fsynced-intent-and-native-removal")
        scope = self.control_scopes["C31-crash-journal"]
        scope_ref, _ = self.control_fact(scope["ref"]["path"], 16384)
        row = dict(version=1, status="C31_FAULT_CHECKPOINT_AWAITING_OUTER_NATIVE_KILL",
            source_pin=ENGINE, context_sha256=self.m["context_source_sha256"],
            guard_sha256=self.m["parent_capture_sha256"], fixture_root=q["fixture_root"],
            batch_root=q["batch_root"], nonce=q["nonce"], root=scope["root"],
            plan_ref=plan_ref, permit_ref=permit_ref, archive_ref=archive_ref,
            scope_ref=scope_ref, journal_ref=journal_ref, observed_absent=fault["absent"],
            native_absence_error=2, pid=os.getpid(), creation=self.n.creation(self.n.k.GetCurrentProcess()),
            lease_acquired_perf_ns=self.lease.check(), perf_counter_ns=time.perf_counter_ns(),
            ambiguous_uncommitted_credit=0, engine_qualification=False)
        raw = encode(row)
        need(len(raw) <= 16384, "C31-checkpoint-cap")
        self.control_write("fault-checkpoint.json", raw)
        # The G must actually kill this process. Timeout/self-return cannot qualify.
        while True:
            self.poll()
            time.sleep(0.02)

    def c31_phase(self):
        phase, q = self.c31_port()
        need(phase in {"C31_FAULT", "C31_RECOVERY"}, "fixed-C31-worker-phase")
        case = Path(q["fixture_root"])
        need(str(case.parent) == self.limits["control_root"], "C31-finite-owner-prefix")
        self.control_case = dict(root=str(case), case_id="C31-crash-journal")
        A, controls = self.archive(), self.controls_module
        self.native_module = A
        if phase == "C31_FAULT":
            need(all(q[k] is None for k in ("plan_ref", "scope_ref", "archive_ref", "checkpoint_ref",
                 "fault_capture_ref", "fault_lifetime_ref", "fault_completion_ref", "recovery_ref",
                 "recovery_capture_ref", "recovery_lifetime_ref", "recovery_completion_ref")),
                 "C31-fault-must-be-new")
            need(not os.path.lexists(case) and not os.path.lexists(case.parent), "C31-new-fixture-prefix")
            self.check_budget(2097152, 16)
            case.parent.mkdir(); case.mkdir()
            plan = self.prepare_control_plan(controls.prepared(A, case, ENGINE,
                self.m["parent_capture_sha256"], self.m["deadline_epoch"],
                self.m["context_source_sha256"], self))
            path = case / "capture-plan.json"; raw = encode(plan) + b"\n"
            self.control_write("capture-plan.json", raw, 131072)
            digest = hashlib.sha256(raw).hexdigest()
            proof = A.archive(str(path), digest, self)
            permit_path, permit_sha = self.control_permit(plan, proof["compressed_sha256"])
            self.native_fault = dict(case_id="C31-crash-journal",
                                     absent=str(case / "source/empty-directory"), observed=False)
            A.retire(permit_path, permit_sha, self)
            raise Refusal("C31-not-killed-by-outer-guard")
        checkpoint = decode(self.read_ref(q["checkpoint_ref"]))
        self.c31_binding(q, checkpoint)
        self.c31_closure(q, "fault", checkpoint)
        plan = decode(self.read_ref(q["plan_ref"]))
        self.control_scopes["C31-crash-journal"] = dict(root=plan["roots"][0],
            ref={k: q["scope_ref"][k] for k in ("path", "sha256")}, template=plan)
        source = case / "source"; before = self.control_snapshot(source)
        archive_ref, _ = self.control_fact(case / "archive/archive.gz", 1048576)
        journal_ref, _ = self.control_fact(case / "archive/retirement.journal", 1048576)
        need(archive_ref == q["archive_ref"] and journal_ref == checkpoint["journal_ref"],
             "C31-surviving-backup-and-partial-journal")
        permit = checkpoint["permit_ref"]
        message = self.native_negative(lambda: A.retire(permit["path"], permit["sha256"], self),
                                       ["journal", "census", "source", "changed", "mismatch"])
        need(self.control_snapshot(source) == before
             and self.control_fact(case / "archive/archive.gz", 1048576)[0] == archive_ref
             and self.control_fact(case / "archive/retirement.journal", 1048576)[0] == journal_ref,
             "C31-recovery-mutated-partial-or-backup")
        row = dict(version=1, status="C31_RECOVERY_EXPECTED_REFUSAL", checkpoint_ref=q["checkpoint_ref"],
            fixture_root=q["fixture_root"], batch_root=q["batch_root"], nonce=q["nonce"],
            source_pin=ENGINE, context_sha256=self.m["context_source_sha256"],
            guard_sha256=self.m["parent_capture_sha256"], plan_ref=q["plan_ref"],
            scope_ref=q["scope_ref"], archive_ref=archive_ref, journal_ref=journal_ref,
            partial_before_sha256=before, partial_after_sha256=before, refusal=message,
            pid=os.getpid(), creation=self.n.creation(self.n.k.GetCurrentProcess()),
            lease_acquired_perf_ns=self.lease.check(), resume_authorized=False,
            retirement_authorized=False, engine_qualification=False, expiry_credit=0)
        raw = encode(row)
        self.control_write("recovery-proof.json", raw)
        return row

    def c31_binding(self, q, checkpoint):
        need(checkpoint["status"] == "C31_FAULT_CHECKPOINT_AWAITING_OUTER_NATIVE_KILL"
             and checkpoint["fixture_root"] == q["fixture_root"]
             and checkpoint["batch_root"] == q["batch_root"] and checkpoint["nonce"] == q["nonce"]
             and checkpoint["source_pin"] == ENGINE
             and checkpoint["context_sha256"] == self.m["context_source_sha256"]
             and checkpoint["guard_sha256"] == self.m["parent_capture_sha256"]
             and all(checkpoint[k] == q[k] for k in ("plan_ref", "scope_ref", "archive_ref"))
             and integer(checkpoint["lease_acquired_perf_ns"], 1, checkpoint["perf_counter_ns"] - 1)
             and checkpoint["native_absence_error"] == 2
             and checkpoint["observed_absent"] == ntpath.join(q["fixture_root"], "source", "empty-directory")
             and checkpoint["ambiguous_uncommitted_credit"] == 0
             and checkpoint["engine_qualification"] is False, "C31-checkpoint-exact-source-prefix")

    def c31_consume(self):
        phase, q = self.c31_port()
        need(phase == "BATCH" and q["batch_root"] == self.limits["control_root"], "C31-batch-binding")
        checkpoint = decode(self.read_ref(q["checkpoint_ref"]))
        self.c31_binding(q, checkpoint)
        self.c31_closure(q, "fault", checkpoint)
        recovery = decode(self.read_ref(q["recovery_ref"]))
        self.c31_closure(q, "recovery", recovery)
        fault_capture = decode(self.read_ref(q["fault_capture_ref"]))
        recovery_capture = decode(self.read_ref(q["recovery_capture_ref"]))
        need(fault_capture["job_close"]["completed_perf_ns"] < recovery["lease_acquired_perf_ns"]
             < recovery_capture["host"]["exit_observed_perf_ns"], "C31-recovery-local-lease-after-fault-closure")
        need(recovery["status"] == "C31_RECOVERY_EXPECTED_REFUSAL"
             and all(recovery[k] == q[k] for k in ("fixture_root", "batch_root", "nonce", "plan_ref", "scope_ref", "archive_ref"))
             and recovery["checkpoint_ref"] == q["checkpoint_ref"]
             and recovery["source_pin"] == ENGINE
             and recovery["context_sha256"] == self.m["context_source_sha256"]
             and recovery["guard_sha256"] == self.m["parent_capture_sha256"]
             and recovery["partial_before_sha256"] == recovery["partial_after_sha256"]
             and is_sha(recovery["partial_before_sha256"])
             and recovery["resume_authorized"] is False and recovery["retirement_authorized"] is False
             and recovery["engine_qualification"] is False and recovery["expiry_credit"] == 0,
             "C31-actual-recovery-proof")
        for ref in (q["plan_ref"], q["scope_ref"], q["archive_ref"], recovery["journal_ref"]):
            self.read_ref(ref, ref is not q["archive_ref"])
        return dict(checkpoint_ref=q["checkpoint_ref"], recovery_ref=q["recovery_ref"],
                    fault_capture_ref=q["fault_capture_ref"], recovery_capture_ref=q["recovery_capture_ref"])

    def after_control(self, *, case_id, root, deadline_epoch):
        self.lease.check()
        need(self.control_case is not None and case_id == self.control_case["case_id"]
             and root == self.limits["control_root"] and deadline_epoch == self.control_deadline,
             "control-finally-case-binding")
        need(not self.n.uncertain and not self.actors and not getattr(self, "helper_uncertain", False)
             and {x.ident for x in threading.enumerate()} == self.baseline_threads, "control-IO-uncertain")
        result_path = Path(self.control_case["root"]) / "archive/retirement-result.json"
        if result_path.is_file():
            fact, raw = self.control_fact(result_path, 131072)
            self.helper_uncertain = getattr(self, "helper_uncertain", False) or (
                "native-close-handle" in str(decode(raw).get("error", "")))
        self.check_budget(0, 0)
        self.control_case = None

    def finalize_controls(self, *, result, root, deadline_epoch):
        need(result["control_only"] is True and result["engine_qualification"] is False,
             "control-result-scope")
        self.check_budget(0, 0)

    def finish(self):
        """Release only after reviewed synchronous helper and actual handle closure."""
        need(not self.completed, "Context-finish-twice")
        pins = [pin for pin, raw in self.refs.values()]
        pins += getattr(self, "protected_pins", []) + self.anchor_parents
        for pin in reversed(pins):
            pin.close()
        if self.parent_handle:
            handle, self.parent_handle = self.parent_handle, None
            self.n.close(handle)
        need(not self.n.uncertain and not self.actors and not getattr(self, "helper_uncertain", False)
             and {x.ident for x in threading.enumerate()} <= self.baseline_threads, "owned-IO-callbacks-unclosed")
        expected = self.baseline_handles + (len(self.lease.pin.handles) if self.lease else 0)
        need(self.n.handle_count() == expected, "archive-or-other-native-handle-leak")
        if self.lease:
            self.lease.release()
        self.completed = True


def main(argv):
    need(len(argv) == 3 and is_sha(argv[2]), "closed-Context-CLI")
    context = None
    try:
        context = Context.__new__(Context)
        context.__init__(absolute(argv[1]), argv[2])
        plan = context.a["plan"]
        if context.m["action"] == "CONTROL":
            need(context.a["controls_source"] is not None, "missing-R3-controls-source")
            import types
            controls = types.ModuleType("owned_controls")
            controls.__file__ = context.a["controls_source"]["path"]
            raw = context.read_ref(context.a["controls_source"])
            exec(compile(raw, controls.__file__, "exec"), controls.__dict__)
            context.controls_module = controls
            context.control_case_ids = {row[0] for row in controls.CASES}
            need(len(controls.CASES) == len(context.control_case_ids) == 38, "actual-control-catalog")
            if type(context.a["native_controls"]) is dict and context.a["native_controls"].get("phase") in {"C31_FAULT", "C31_RECOVERY"}:
                result = context.c31_phase()
            else:
                result = controls.run(context.limits["control_root"], context,
                    context.a["archive_source"]["sha256"], ENGINE, context.m["parent_capture_sha256"],
                    context.m["deadline_epoch"], context.m["context_source_sha256"])
        elif context.m["action"] == "ARCHIVE_VERIFY":
            archive = context.archive()
            result = archive.archive(plan["path"], plan["sha256"], context)
            archive.verify(plan["path"], plan["sha256"], result["compressed_sha256"], context)
        else:
            need(context.m["real_retire_go"] is True, "no-real-retire-GO")
            result = context.archive().retire(plan["path"], plan["sha256"], context)
        failures = [result.get("error", "")] + [row.get("error", "") for row in result.get("cells", [])]
        context.helper_uncertain = getattr(context, "helper_uncertain", False) or any(
            "native-close-handle" in str(value) for value in failures)
        context.finish()
        passed = (context.m["action"] != "CONTROL"
            or result.get("status") in {"CONTROL_BATCH_PASSED", "CONTROL_BATCH_APPLICABLE_QUALIFIED",
                                       "C31_RECOVERY_EXPECTED_REFUSAL"})
        sys.stdout.buffer.write(encode(dict(status="CONTEXT_COMPLETE" if passed else "CONTEXT_CONTROL_BATCH_INCOMPLETE", action=context.m["action"],
                                           result=result, engine_qualification=False)) + b"\n")
        return 0 if passed else 2
    except BaseException as error:
        if context is not None and not getattr(context, "completed", False):
            if "native-close-handle" in str(error):
                context.helper_uncertain = True
            try:
                context.finish()
            except BaseException:
                KEEP.append(context)  # Actual token capability stays owned until process exit.
        sys.stderr.buffer.write(encode(dict(status="CONTEXT_REFUSED", error=type(error).__name__,
                                           reason=str(error)[:512], expiry_credit=0)) + b"\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
