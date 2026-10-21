"""Fixed clocks and explicit private failures. Imports create no native resources."""
import datetime as dt
import hashlib
import json
import math
import threading
import time

FUNCTIONAL = dt.datetime.fromisoformat("2026-10-21T11:10:14+00:00").timestamp()
FREEZE = dt.datetime.fromisoformat("2026-10-21T04:10:14+00:00").timestamp()
CLOSEOUT = dt.datetime.fromisoformat("2026-10-21T13:10:14+00:00").timestamp()
GUARD_SHA = "50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378"
READER_BYTES, CHECKER_BYTES = 268435456, 2147483648
READERS = frozenset(("resolve", "point", "inspect", "lookup-bench"))


class Refusal(RuntimeError):
    pass


class Unsupported(Refusal):
    """Pending source/capability; never a qualified pass."""


def need(value, message):
    if not value:
        raise Refusal(message)


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def strict(raw, limit):
    need(type(raw) is bytes and len(raw) <= limit, "bounded-json")
    def pairs(items):
        result = {}
        for key, value in items:
            need(key not in result, "duplicate-json-key")
            result[key] = value
        return result
    def invalid(value):
        raise Refusal("nonfinite-json:"+value)
    return json.loads(raw.decode("utf-8"), object_pairs_hook=pairs, parse_constant=invalid)


def integer(value, minimum=0):
    return type(value) is int and value >= minimum


class Deadline:
    def __init__(self, absolute, timeout, cancel=None):
        need(type(absolute) in (float, int) and math.isfinite(absolute), "absolute-deadline")
        need(type(timeout) in (float, int) and math.isfinite(timeout) and timeout > 0,
             "positive-finite-timeout")
        self.end = min(absolute, FUNCTIONAL, time.time()+timeout)
        self.mono_end = time.perf_counter()+max(0, self.end-time.time())
        self.cancel = cancel or threading.Event()
        self.poll()

    def remaining(self):
        return max(0.0, min(self.end-time.time(), self.mono_end-time.perf_counter()))

    def poll(self):
        need(not self.cancel.is_set(), "cancelled-transport")
        need(self.remaining() > 0, "absolute-total-deadline")

    def sleep(self, seconds):
        need(type(seconds) in (float, int) and 0 <= seconds < 60, "bounded-sleep")
        self.poll()
        self.cancel.wait(min(seconds, self.remaining()))
        self.poll()

def project_response(value,reference,response,*,max_bytes,max_depth=32,max_nodes=100000):
    """Only exact persisted decoded engine envelopes become reconstructible refs."""
    nodes=0;expected=canonical(response) if type(response) is dict else None
    def walk(v,depth):
        nonlocal nodes
        nodes+=1
        need(depth<=max_depth and nodes<=max_nodes,"bounded-response-projection-tree")
        if type(v) is dict:
            if str(v.get("schema","")).startswith("native-response-ref"):
                need(v["schema"]=="native-response-ref-v1" and set(v)=={"schema","raw_ref"},
                     "closed-native-response-projection")
                return v
            if set(v)=={"status","result","error","metrics","wall_ns","peak_memory_bytes"}:
                need(expected is not None and canonical(v)==expected,"same-call-response-projection")
                return dict(schema="native-response-ref-v1",raw_ref=reference)
            return {k:walk(x,depth+1) for k,x in v.items()}
        if type(v) is list:return [walk(x,depth+1) for x in v]
        return v
    need(len(canonical(value))<=max_bytes,"bounded-expanded-response-projection")
    return walk(value,0)

