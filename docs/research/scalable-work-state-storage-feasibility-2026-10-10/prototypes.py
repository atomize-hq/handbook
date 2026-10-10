"""Storage-only feasibility probes; never a production owner or native verifier.

All stores retain the same opaque native original bytes once, referenced by digest.
The synthetic envelope and its checksum are NEW research data, not native identities.
Durability sources and explicit omissions are in prototypes-README.md.
"""
from __future__ import annotations

import argparse
import contextlib
import ctypes
import hashlib
import json
import os
from pathlib import Path
import random
import sqlite3
import struct
import subprocess
import sys
import tempfile
import time

ZERO = "0" * 64
FRAME = struct.Struct("<II")
SEGMENT_RECORDS = 1000


def encoded(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def identity(path):
    stat = Path(path).stat()
    return stat.st_dev, stat.st_ino


def atomic_file(path, data):
    """Sync bytes before publishing; same-volume rename is not a power-loss test."""
    path = Path(path)
    stage = path.with_name(path.name + ".new")
    with stage.open("wb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())
    if os.name == "nt":
        move = ctypes.WinDLL("kernel32", use_last_error=True).MoveFileExW
        move.argtypes = [ctypes.c_wchar_p, ctypes.c_wchar_p, ctypes.c_uint32]
        move.restype = ctypes.c_int
        if not move(str(stage), str(path), 1 | 8):
            raise ctypes.WinError(ctypes.get_last_error())
    else:
        os.replace(stage, path)
        descriptor = os.open(path.parent, os.O_RDONLY)
        try:
            os.fsync(descriptor)
        finally:
            os.close(descriptor)


def crash_at(stage, selected):
    # os._exit does not close streams/rollback SQLite. Real abrupt process exit,
    # not an OS crash, controller kill, power cut, or proof of device durability.
    if stage == selected:
        os._exit(91)


class Refusal(Exception):
    pass


class BoundStore:
    def __init__(self, root, blob, dependency, ordinary_limit=200000, admin_limit=10000,
                 create=False):
        self.root = Path(root)
        self.blob = blob
        self.blob_id = digest(blob)
        self.dep = dependency
        self.dep_id = digest(dependency)
        self.ordinary_limit = ordinary_limit
        self.admin_limit = admin_limit
        if create:
            self.root.mkdir(parents=True, exist_ok=False)
            atomic_file(self.root / "owner.lock", b"L")
        if not (self.root / "owner.lock").is_file():
            raise Refusal("missing lock")
        self.root_id = identity(self.root)
        self.lock_id = identity(self.root / "owner.lock")
        self.lock_wait_ms = []

    @contextlib.contextmanager
    def locked(self):
        if identity(self.root) != self.root_id:
            raise Refusal("substituted root")
        if identity(self.root / "owner.lock") != self.lock_id:
            raise Refusal("substituted lock")
        with (self.root / "owner.lock").open("r+b") as stream:
            before = time.perf_counter_ns()
            if os.name == "nt":
                import msvcrt
                while True:
                    try:
                        stream.seek(0)
                        msvcrt.locking(stream.fileno(), msvcrt.LK_NBLCK, 1)
                        break
                    except OSError:
                        if (time.perf_counter_ns() - before) > 30_000_000_000:
                            raise Refusal("lock timeout")
                        time.sleep(.001)
            else:
                import fcntl
                fcntl.flock(stream.fileno(), fcntl.LOCK_EX)
            if identity(self.root / "owner.lock") != self.lock_id:
                raise Refusal("lock replaced while waiting")
            self.lock_wait_ms.append((time.perf_counter_ns() - before) / 1e6)
            try:
                yield
            finally:
                if os.name == "nt":
                    stream.seek(0)
                    msvcrt.locking(stream.fileno(), msvcrt.LK_UNLCK, 1)
                else:
                    fcntl.flock(stream.fileno(), fcntl.LOCK_UN)

    def make_record(self, head, key, kind, target, original):
        if kind not in ("accepted", "terminal", "admin"):
            raise Refusal("unknown kind")
        if kind == "accepted" and head["ordinary"] >= self.ordinary_limit:
            raise Refusal("ordinary saturation")
        if kind == "admin" and head["admin"] >= self.admin_limit:
            raise Refusal("administrative exhaustion")
        header = dict(seq=head["count"] + 1, key=key, kind=kind, target=target,
                      original=original, dep=self.dep_id, blob=self.blob_id,
                      prev=head["chain"])
        body = encoded(header)
        chain = digest(bytes.fromhex(head["chain"]) + body)
        frame = FRAME.pack(len(body), 0) + body + bytes.fromhex(chain)
        return header, frame, chain

    def verified(self, header, blob, chain, dependency):
        if digest(blob) != header["blob"] or digest(dependency) != header["dep"]:
            raise Refusal("selected original/dependency checksum mismatch")
        actual = digest(bytes.fromhex(header["prev"]) + encoded(header))
        if actual != chain:
            raise Refusal("selected envelope checksum mismatch")
        return header

    def close(self):
        pass


class FileStore(BoundStore):
    """A: sequence-named individual records plus checksummed rebuildable log index.

    B subclasses only physical framing: immutable closed segments and active tail.
    Index checks local head/mapping agreement on reopen, not external authenticity.
    """
    segmented = False
    stream_audit = False

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.head_path = self.root / "head.json"
        self.index_path = self.root / "index.jsonl"
        self.records = {}
        self.keys = {}
        self.current = {}
        self.cached_ordinary, self.cached_admin = 0, 0
        self.cache_end = 0
        self.cache_hash = ZERO
        if kwargs.get("create"):
            (self.root / "records").mkdir()
            (self.root / "deps").mkdir()
            (self.root / "originals").mkdir()
            atomic_file(self.root / "deps" / self.dep_id, self.dep)
            atomic_file(self.root / "originals" / digest(self.blob), self.blob)
            atomic_file(self.index_path, b"")
            atomic_file(self.head_path, encoded(dict(count=0, chain=ZERO, cache=ZERO,
                                                     ordinary=0, admin=0)))
        with self.locked():
            self.refresh()

    def refresh(self):
        try:
            head = json.loads(self.head_path.read_bytes())
        except (OSError, ValueError) as error:
            raise Refusal("missing/malformed authoritative head") from error
        previous_count = getattr(self, "head", {"count": 0})["count"]
        if head["count"] < previous_count:
            raise Refusal("head rolled back within open session")
        if getattr(self, "head", None) == head:
            return
        self.head = head
        try:
            with self.index_path.open("rb") as stream:
                stream.seek(self.cache_end)
                for line in stream:
                    row = json.loads(line)
                    if row["seq"] > head["count"]:
                        break
                    if row["seq"] != len(self.records) + 1:
                        raise ValueError("index sequence")
                    self.cache_hash = digest(bytes.fromhex(self.cache_hash) + line)
                    self.install(row)
                    self.cache_end = stream.tell()
            last_chain = self.records[head["count"]]["chain"] if head["count"] in self.records else ZERO
            if (len(self.records), self.cache_hash, last_chain, self.cached_ordinary, self.cached_admin) != (
                    head["count"], head["cache"], head["chain"], head["ordinary"], head["admin"]):
                raise ValueError("incomplete/stale/modified index")
        except (OSError, ValueError, KeyError):
            self.rebuild()

    def install(self, row):
        self.records[row["seq"]] = row
        self.keys[row["key"]] = row["seq"]
        self.current[row["target"]] = row["seq"]
        self.cached_ordinary += row["kind"] == "accepted"
        self.cached_admin += row["kind"] == "admin"

    def read_frame(self, locator, objects=None):
        try:
            record_path = self.root / "records" / locator[0]
            if self.segmented and self.stream_audit and objects is not None:
                if record_path not in objects["frames"]:
                    objects["frames"][record_path] = objects["stack"].enter_context(record_path.open("rb"))
                context = contextlib.nullcontext(objects["frames"][record_path])
            else:
                context = record_path.open("rb")
            with context as stream:
                stream.seek(locator[1])
                raw = stream.read(FRAME.size)
                header_size, blob_size = FRAME.unpack(raw)
                if header_size > 4096 or blob_size > 1048576:
                    raise Refusal("oversize research frame")
                header = json.loads(stream.read(header_size))
                if blob_size != 0:
                    raise Refusal("unexpected inline original")
                blob_path = self.root / "originals" / header["blob"]
                if objects is not None:
                    if blob_path not in objects:
                        objects[blob_path] = blob_path.read_bytes()
                    blob = objects[blob_path]
                else:
                    blob = blob_path.read_bytes()
                chain = stream.read(32).hex()
                if len(chain) != 64:
                    raise Refusal("truncated frame")
                return header, blob, chain, FRAME.size + header_size + blob_size + 32
        except (OSError, ValueError, struct.error) as error:
            raise Refusal("missing/damaged selected record") from error

    def locator(self, seq, offset=0):
        file_no = (seq - 1) // SEGMENT_RECORDS if self.segmented else seq
        suffix = "seg" if self.segmented else "rec"
        return [f"{file_no:09}.{suffix}", offset]

    def rebuild(self):
        self.records, self.keys, self.current = {}, {}, {}
        self.cached_ordinary, self.cached_admin = 0, 0
        chain, cache_hash = ZERO, ZERO
        offset = 0
        ordinary, admin = 0, 0
        objects, audited_original_ids = {"frames": {}}, set()
        with contextlib.ExitStack() as stack, self.index_path.with_suffix(".rebuilt").open("wb") as index:
            objects["stack"] = stack
            for seq in range(1, self.head["count"] + 1):
                if not self.segmented or (seq - 1) % SEGMENT_RECORDS == 0:
                    offset = 0
                locator = self.locator(seq, offset)
                header, blob, next_chain, size = self.read_frame(locator, objects)
                dependency_path = self.root / "deps" / header["dep"]
                if dependency_path not in objects:
                    dependency = dependency_path.read_bytes()
                    if digest(dependency) != header["dep"]:
                        raise Refusal("dependency checksum mismatch")
                    objects[dependency_path] = dependency
                if digest(bytes.fromhex(header["prev"]) + encoded(header)) != next_chain:
                    raise Refusal("envelope checksum mismatch")
                if header["blob"] not in audited_original_ids:
                    if digest(blob) != header["blob"]:
                        raise Refusal("original checksum mismatch")
                    audited_original_ids.add(header["blob"])
                if header["seq"] != seq or header["prev"] != chain:
                    raise Refusal("history sequence/chain mismatch")
                chain = next_chain
                row = dict(header, locator=locator, size=size, chain=chain)
                line = encoded(row) + b"\n"
                cache_hash = digest(bytes.fromhex(cache_hash) + line)
                index.write(line)
                self.install(row)
                ordinary += header["kind"] == "accepted"
                admin += header["kind"] == "admin"
                offset += size
            index.flush()
            os.fsync(index.fileno())
        if (chain, cache_hash, ordinary, admin) != (self.head["chain"], self.head["cache"],
                                                   self.head["ordinary"], self.head["admin"]):
            raise Refusal("history does not match authoritative head")
        # Cache publication need not be durable; reopening rebuilds if absent.
        os.replace(self.index_path.with_suffix(".rebuilt"), self.index_path)
        self.cache_hash = cache_hash
        self.cache_end = self.index_path.stat().st_size

    def selected(self, seq):
        row = self.records[seq]
        header, blob, chain, _ = self.read_frame(row["locator"])
        if header != {key: row[key] for key in header} or chain != row["chain"]:
            raise Refusal("selected record/index disagreement")
        try:
            dep = (self.root / "deps" / header["dep"]).read_bytes()
        except OSError as error:
            raise Refusal("missing selected dependency") from error
        return self.verified(header, blob, chain, dep)

    def lookup(self, target):
        with self.locked():
            self.refresh()
            if target not in self.current:
                raise Refusal("missing target")
            record = self.selected(self.current[target])
            if record["target"] != target:
                raise Refusal("selected target disagreement")
            if record["kind"] == "terminal":
                original = self.selected(record["original"])
                if original["kind"] != "accepted" or original["target"] != target:
                    raise Refusal("wrong original")
            return record

    def execute(self, key, kind, target, fail=None):
        with self.locked():
            self.refresh()
            if key in self.keys:
                result = self.selected(self.keys[key])
                if (result["kind"], result["target"], result["blob"], result["dep"]) != (kind, target, self.blob_id, self.dep_id):
                    raise Refusal("replay request changed")
                return "replayed", result["seq"]
            original = 0
            if kind == "accepted" and target in self.current:
                raise Refusal("one registration lineage")
            if kind == "terminal":
                if target not in self.current:
                    raise Refusal("missing accepted original")
                result = self.selected(self.current[target])
                if result["kind"] != "accepted":
                    raise Refusal("already terminal")
                original = result["seq"]
            header, frame, chain = self.make_record(self.head, key, kind, target, original)
            try:
                stored_blob = (self.root / "originals" / self.blob_id).read_bytes()
                stored_dep = (self.root / "deps" / self.dep_id).read_bytes()
            except OSError as error:
                raise Refusal("missing selected retained original/dependency") from error
            self.verified(header, stored_blob, chain, stored_dep)
            locator = self.locator(header["seq"])
            path = self.root / "records" / locator[0]
            if self.segmented:
                if path.exists():
                    locator[1] = path.stat().st_size
                    last = self.records.get(self.head["count"])
                    if not last or last["locator"][0] != locator[0] or locator[1] != last["locator"][1] + last["size"]:
                        raise Refusal("uncommitted tail requires original recovery")
                mode = "ab"
            else:
                if path.exists():
                    raise Refusal("orphan record requires original recovery")
                mode = "xb"
            with path.open(mode) as stream:
                stream.write(frame)
                stream.flush()
                os.fsync(stream.fileno())
            crash_at("after_data", fail)
            row = dict(header, locator=locator, size=len(frame), chain=chain)
            line = encoded(row) + b"\n"
            cache_hash = digest(bytes.fromhex(self.cache_hash) + line)
            head = dict(count=header["seq"], chain=chain, cache=cache_hash,
                        ordinary=self.head["ordinary"] + (kind == "accepted"),
                        admin=self.head["admin"] + (kind == "admin"))
            atomic_file(self.head_path, encoded(head))
            crash_at("after_commit", fail)
            with self.index_path.open("ab") as index:
                index.write(line)
            self.install(row)
            self.head = head
            self.cache_hash = cache_hash
            self.cache_end += len(line)
            crash_at("after_index", fail)
            return "committed", header["seq"]

    def audit(self):
        with self.locked():
            self.refresh()
            self.rebuild()
            return self.head["count"]

    def seed(self, count):
        """Population only: files individually fsynced, head/cache once per 1000."""
        with self.locked():
            self.refresh()
            with self.index_path.open("ab") as index:
                segment = None
                try:
                    for position in range(count):
                        kind, target = workload(position)
                        original = self.current.get(target, 0) if kind == "terminal" else 0
                        header, frame, chain = self.make_record(self.head, f"seed-{position}", kind, target, original)
                        locator = self.locator(header["seq"])
                        path = self.root / "records" / locator[0]
                        if self.segmented:
                            if (header["seq"] - 1) % SEGMENT_RECORDS == 0:
                                if segment:
                                    segment.flush()
                                    os.fsync(segment.fileno())
                                    segment.close()
                                segment = path.open("xb")
                            locator[1] = segment.tell()
                            segment.write(frame)
                        else:
                            with path.open("xb") as stream:
                                stream.write(frame)
                                stream.flush()
                                os.fsync(stream.fileno())
                        row = dict(header, locator=locator, size=len(frame), chain=chain)
                        line = encoded(row) + b"\n"
                        index.write(line)
                        self.cache_hash = digest(bytes.fromhex(self.cache_hash) + line)
                        self.install(row)
                        self.cache_end += len(line)
                        self.head = dict(count=header["seq"], chain=chain, cache=self.cache_hash,
                                         ordinary=self.head["ordinary"] + (kind == "accepted"),
                                         admin=self.head["admin"] + (kind == "admin"))
                        if (position + 1) % SEGMENT_RECORDS == 0 or position == count - 1:
                            if segment:
                                segment.flush()
                                os.fsync(segment.fileno())
                            index.flush()
                            atomic_file(self.head_path, encoded(self.head))
                finally:
                    if segment:
                        segment.close()


class SegmentedStore(FileStore):
    segmented = True


class SqliteStore(BoundStore):
    def __init__(self, *args, **kwargs):
        if sqlite3.sqlite_version_info < (3, 51, 3):
            raise Refusal("SQLite lacks 2026 WAL-reset fix; use documented bundled runtime")
        super().__init__(*args, **kwargs)
        path = self.root / "owner.sqlite"
        if not kwargs.get("create") and not path.is_file():
            raise Refusal("missing owner database")
        # Python isolation_level=None means explicit SQL controls transactions.
        self.db = sqlite3.connect(path, isolation_level=None, timeout=30)
        assert self.db.execute("PRAGMA journal_mode=WAL").fetchone()[0] == "wal"
        self.db.execute("PRAGMA synchronous=FULL")
        self.db.execute("PRAGMA foreign_keys=ON")
        self.db.execute("PRAGMA wal_autocheckpoint=1000")
        self.db.execute("PRAGMA cache_size=-8192")
        if kwargs.get("create"):
            self.db.executescript("""
                CREATE TABLE dependencies(id TEXT PRIMARY KEY, bytes BLOB NOT NULL);
                CREATE TABLE originals(id TEXT PRIMARY KEY, bytes BLOB NOT NULL);
                CREATE TABLE records(seq INTEGER PRIMARY KEY, key TEXT UNIQUE NOT NULL,
                    target TEXT NOT NULL, kind TEXT NOT NULL, original INTEGER,
                    header BLOB NOT NULL, blob TEXT NOT NULL REFERENCES originals(id), chain TEXT NOT NULL,
                    dep TEXT NOT NULL REFERENCES dependencies(id),
                    FOREIGN KEY(original) REFERENCES records(seq));
                CREATE TABLE current(target TEXT PRIMARY KEY, seq INTEGER NOT NULL REFERENCES records(seq), proof TEXT NOT NULL);
                CREATE TABLE head(id INTEGER PRIMARY KEY CHECK(id=1), count INTEGER NOT NULL,
                    chain TEXT NOT NULL, ordinary INTEGER NOT NULL, admin INTEGER NOT NULL, proof TEXT NOT NULL);
            """)
            self.db.execute("BEGIN IMMEDIATE")
            self.db.execute("INSERT INTO dependencies VALUES(?,?)", (self.dep_id, self.dep))
            self.db.execute("INSERT INTO originals VALUES(?,?)", (digest(self.blob), self.blob))
            self.db.execute("INSERT INTO head VALUES(1,0,?,0,0,?)", (ZERO, digest(encoded(["research-head", 0, ZERO, 0, 0]))))
            self.db.execute("COMMIT")
        if self.db.execute("PRAGMA synchronous").fetchone()[0] != 2:
            raise Refusal("unexpected durability setting")

    def read_head(self):
        result = self.db.execute("SELECT count,chain,ordinary,admin,proof FROM head WHERE id=1").fetchone()
        if result is None:
            raise Refusal("missing authoritative head row")
        if digest(encoded(["research-head", *result[:4]])) != result[4]:
            raise Refusal("head checksum disagreement")
        return dict(zip(("count", "chain", "ordinary", "admin"), result[:4]))

    def selected(self, seq):
        result = self.db.execute("SELECT r.header,o.bytes,r.chain,d.bytes FROM records r "
                                 "LEFT JOIN dependencies d ON d.id=r.dep "
                                 "LEFT JOIN originals o ON o.id=r.blob WHERE r.seq=?", (seq,)).fetchone()
        if result is None or result[3] is None or result[1] is None:
            raise Refusal("missing selected row/dependency")
        header = self.verified(json.loads(result[0]), result[1], result[2], result[3])
        if header["seq"] != seq:
            raise Refusal("selected row sequence disagreement")
        return header

    def lookup(self, target):
        with self.locked():
            self.db.execute("BEGIN")
            try:
                head = self.read_head()
                current = self.db.execute("SELECT seq,proof FROM current WHERE target=?", (target,)).fetchone()
                if current is None or current[0] > head["count"]:
                    raise Refusal("missing target/currentness")
                if digest(encoded(["research-current", target, current[0]])) != current[1]:
                    raise Refusal("current pointer checksum disagreement")
                record = self.selected(current[0])
                if record["target"] != target:
                    raise Refusal("selected target disagreement")
                if record["kind"] == "terminal":
                    original = self.selected(record["original"])
                    if original["kind"] != "accepted" or original["target"] != target:
                        raise Refusal("wrong original")
                return record
            finally:
                self.db.execute("ROLLBACK")

    def append_in_transaction(self, key, kind, target, fail=None):
        head = self.read_head()
        replay = self.db.execute("SELECT seq FROM records WHERE key=?", (key,)).fetchone()
        if replay:
            result = self.selected(replay[0])
            if (result["kind"], result["target"], result["blob"], result["dep"]) != (kind, target, self.blob_id, self.dep_id):
                raise Refusal("replay request changed")
            return "replayed", replay[0]
        current = self.db.execute("SELECT seq,proof FROM current WHERE target=?", (target,)).fetchone()
        if current and digest(encoded(["research-current", target, current[0]])) != current[1]:
            raise Refusal("current pointer checksum disagreement")
        if kind == "accepted" and current:
            raise Refusal("one registration lineage")
        original = 0
        if kind == "terminal":
            if not current or self.selected(current[0])["kind"] != "accepted":
                raise Refusal("missing accepted original/already terminal")
            original = current[0]
        header, _, chain = self.make_record(head, key, kind, target, original)
        self.db.execute("INSERT INTO records VALUES(?,?,?,?,?,?,?,?,?)",
                        (header["seq"], key, target, kind, original or None,
                         encoded(header), self.blob_id, chain, self.dep_id))
        self.db.execute("INSERT INTO current VALUES(?,?,?) ON CONFLICT(target) DO UPDATE SET seq=excluded.seq,proof=excluded.proof",
                        (target, header["seq"], digest(encoded(["research-current", target, header["seq"]]))))
        ordinary = head["ordinary"] + (kind == "accepted")
        admin = head["admin"] + (kind == "admin")
        self.db.execute("UPDATE head SET count=?,chain=?,ordinary=?,admin=?,proof=? WHERE id=1",
                        (header["seq"], chain, head["ordinary"] + (kind == "accepted"),
                         head["admin"] + (kind == "admin"),
                         digest(encoded(["research-head", header["seq"], chain, ordinary, admin]))))
        crash_at("after_data", fail)
        return "committed", header["seq"]

    def execute(self, key, kind, target, fail=None):
        with self.locked():
            self.db.execute("BEGIN IMMEDIATE")
            try:
                result = self.append_in_transaction(key, kind, target, fail)
                # Reused exact objects remain selected dependencies; every timed
                # mutation checks their retained bytes before committing.
                self.selected(result[1])
                self.db.execute("COMMIT")
                crash_at("after_commit", fail)
                return result
            except BaseException:
                if self.db.in_transaction:
                    self.db.execute("ROLLBACK")
                raise

    def audit(self):
        with self.locked():
            self.db.execute("BEGIN")
            try:
                if self.db.execute("PRAGMA integrity_check").fetchall() != [("ok",)]:
                    raise Refusal("database formatting/index integrity")
                if self.db.execute("PRAGMA foreign_key_check").fetchall():
                    raise Refusal("foreign key integrity")
                head = self.read_head()
                chain, count, ordinary, admin = ZERO, 0, 0, 0
                derived_current = {}
                object_ids = {}
                for table in ("originals", "dependencies"):
                    ids = set()
                    for key, value in self.db.execute("SELECT id,bytes FROM " + table):
                        if digest(value) != key:
                            raise Refusal("shared object checksum mismatch")
                        ids.add(key)
                    object_ids[table] = ids
                for raw_header, next_chain in self.db.execute("SELECT header,chain FROM records ORDER BY seq"):
                    header = json.loads(raw_header)
                    if header["blob"] not in object_ids["originals"] or header["dep"] not in object_ids["dependencies"]:
                        raise Refusal("missing retained original/dependency")
                    if digest(bytes.fromhex(header["prev"]) + encoded(header)) != next_chain:
                        raise Refusal("envelope checksum mismatch")
                    count += 1
                    if header["seq"] != count or header["prev"] != chain:
                        raise Refusal("history sequence/chain mismatch")
                    chain = next_chain
                    ordinary += header["kind"] == "accepted"
                    admin += header["kind"] == "admin"
                    derived_current[header["target"]] = count
                if (count, chain, ordinary, admin) != (head["count"], head["chain"], head["ordinary"], head["admin"]):
                    raise Refusal("history/head disagreement")
                current = {}
                for target, seq, proof in self.db.execute("SELECT target,seq,proof FROM current"):
                    if digest(encoded(["research-current", target, seq])) != proof:
                        raise Refusal("current pointer checksum disagreement")
                    current[target] = seq
                if current != derived_current:
                    raise Refusal("current map/history disagreement")
                return count
            finally:
                self.db.execute("ROLLBACK")

    def seed(self, count):
        with self.locked():
            for start in range(0, count, SEGMENT_RECORDS):
                self.db.execute("BEGIN IMMEDIATE")
                for position in range(start, min(start + SEGMENT_RECORDS, count)):
                    kind, target = workload(position)
                    self.append_in_transaction(f"seed-{position}", kind, target)
                self.db.execute("COMMIT")

    def close(self):
        self.db.close()


STORES = {"A_files": FileStore, "B_segments": SegmentedStore, "C_sqlite": SqliteStore}


def workload(position):
    # 60% accepted, 30% terminal, 10% administrative. Terminal cites prior accept.
    group, slot = divmod(position, 10)
    if slot < 6:
        return "accepted", f"item-{group * 6 + slot}"
    if slot < 9:
        return "terminal", f"item-{group * 6 + slot - 6}"
    return "admin", f"admin-{group}"


def timed(action, repetitions=1):
    values = []
    for _ in range(repetitions):
        before = time.perf_counter_ns()
        action()
        values.append((time.perf_counter_ns() - before) / 1e6)
    ordered = sorted(values)
    return dict(n=len(values), median_ms=ordered[len(values) // 2],
                p95_ms=ordered[max(0, (95 * len(values) + 99) // 100 - 1)],
                min_ms=ordered[0], max_ms=ordered[-1], samples_ms=values)


def working_set():
    if os.name != "nt":
        return None
    class Counters(ctypes.Structure):
        _fields_ = [("cb", ctypes.c_uint32), ("faults", ctypes.c_uint32)] + [
            (name, ctypes.c_size_t) for name in ("peak", "working", "paged_peak", "paged",
                                               "nonpaged_peak", "nonpaged", "pagefile", "pagefile_peak")]
    counters = Counters()
    counters.cb = ctypes.sizeof(counters)
    psapi = ctypes.WinDLL("psapi")
    kernel = ctypes.WinDLL("kernel32")
    kernel.GetCurrentProcess.restype = ctypes.c_void_p
    psapi.GetProcessMemoryInfo.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_uint32]
    if not psapi.GetProcessMemoryInfo(kernel.GetCurrentProcess(), ctypes.byref(counters), counters.cb):
        return None
    return dict(working_bytes=counters.working, peak_bytes=counters.peak)


def benchmark(args, blob, dependency):
    factory = STORES[args.backend]
    root = Path(args.scratch) / f"{args.backend}-{args.count}-{args.label}"
    store = factory(root, blob, dependency, admin_limit=200000, create=True)
    measurements = {}
    measurements["population_batched"] = timed(lambda: store.seed(args.count))
    targets = [f"item-{random.Random(42 + i).randrange(max(1, args.count * 6 // 10))}" for i in range(40)]
    cursor = iter(targets)
    measurements["selected_lookup_warm"] = timed(lambda: store.lookup(next(cursor)), len(targets))
    measurements["full_history_audit"] = timed(store.audit, 3)
    measurements["durable_accepted"] = timed(lambda: store.execute(f"accept-{len(store.lock_wait_ms)}", "accepted",
                                                                    f"new-{len(store.lock_wait_ms)}"), 16)
    measurements["durable_terminal"] = timed(lambda: store.execute(f"terminal-{len(store.lock_wait_ms)}", "terminal",
                                                                    f"item-{(len(store.lock_wait_ms) % max(1, args.count // 10)) * 6 + 3}"), 16)
    measurements["durable_admin"] = timed(lambda: store.execute(f"admin-{len(store.lock_wait_ms)}", "admin",
                                                                 f"new-admin-{len(store.lock_wait_ms)}"), 16)
    memory = working_set()
    store.close()
    reopened = [None]
    def reopen():
        if reopened[0]:
            reopened[0].close()
        reopened[0] = factory(root, blob, dependency, admin_limit=200000)
        reopened[0].lookup("item-0")
    measurements["reopen_selected_warm_os_cache"] = timed(reopen, 3)
    reopened[0].close()
    if args.backend != "C_sqlite":
        (root / "index.jsonl").unlink()
        measurements["missing_index_rebuild"] = timed(reopen)
        reopened[0].close()
    else:
        # REINDEX exercises reconstruction of SQL secondary/unique indexes; owner
        # current-map reconstruction remains a separate semantic audit obligation.
        rebuilt = factory(root, blob, dependency, admin_limit=200000)
        measurements["sql_reindex"] = timed(lambda: rebuilt.db.execute("REINDEX"))
        rebuilt.close()
    file_sizes = [path.stat().st_size for path in root.rglob("*") if path.is_file()]
    report = dict(backend=args.backend, count_initial=args.count, label=args.label,
                  original_bytes=len(blob), dependency_bytes=len(dependency), original_sha256=digest(blob),
                  dependency_sha256=digest(dependency), measurements=measurements,
                  memory_after_warm=memory, logical_file_bytes=sum(file_sizes), files=len(file_sizes),
                  python=sys.version, sqlite=sqlite3.sqlite_version,
                  sqlite_source_id=sqlite3.connect(":memory:").execute("SELECT sqlite_source_id()").fetchone()[0],
                  scratch=str(root), cold_claim=False)
    report["script_sha256"] = digest(Path(__file__).read_bytes())
    Path(args.output).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({key: report[key] for key in ("backend", "count_initial", "logical_file_bytes", "files")}))


def existing_audit(args, blob, dependency):
    root = Path(args.root).resolve()
    scratch = Path(args.scratch).resolve()
    if args.backend != "B_segments" or root.parent != scratch or root.name != f"B_segments-{args.count}-base":
        raise Refusal("follow-up restricted to this task's named B scratch cohort")
    store = SegmentedStore(root, blob, dependency, admin_limit=200000)
    count = store.head["count"]
    measurements = {"streamed_full_history_audit": timed(store.audit, 3)}
    store.close()
    (root / "index.jsonl").unlink()
    rebuilt = [None]
    measurements["streamed_missing_index_rebuild"] = timed(
        lambda: rebuilt.__setitem__(0, SegmentedStore(root, blob, dependency, admin_limit=200000)))
    assert rebuilt[0].head["count"] == count
    assert rebuilt[0].lookup("item-0")["kind"] == "terminal"
    rebuilt[0].close()
    report = dict(backend="B_segments", variant="one-open-per-segment-audit",
                  initial_count=args.count, actual_count=count, stream_audit=SegmentedStore.stream_audit,
                  measurements=measurements, script_sha256=digest(Path(__file__).read_bytes()),
                  original_sha256=digest(blob), dependency_sha256=digest(dependency),
                  changes="Only audit/rebuild frame-handle reuse; existing cohort has 48 timed effects +24 contended effects")
    Path(args.output).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print("streamed audit verified", args.count, count)


def faults(args, blob, dependency):
    results = []
    for name, factory in STORES.items():
        def fresh(label, ordinary=2, admin=2):
            path = Path(args.scratch) / f"fault-{name}-{label}"
            return factory(path, blob, dependency, ordinary_limit=ordinary, admin_limit=admin, create=True)
        store = fresh("capacity")
        assert store.execute("a", "accepted", "a") == ("committed", 1)
        assert store.lookup("a")["kind"] == "accepted"
        assert store.execute("a", "accepted", "a") == ("replayed", 1)
        changed_request = factory(store.root, blob + b" ", dependency)
        try:
            changed_request.execute("a", "accepted", "a")
            raise AssertionError("changed exact request replayed")
        except Refusal as error:
            assert "replay request changed" in str(error)
        changed_request.close()
        assert store.execute("b", "accepted", "b") == ("committed", 2)
        checks = ["accepted-original-before-terminal", "same-key-missing-reply-replay", "changed-exact-request-replay-refused"]
        for key, kind, target, expected in (("c", "accepted", "c", "ordinary saturation"),
                                           ("x", "terminal", "missing", "missing accepted")):
            try:
                store.execute(key, kind, target)
                raise AssertionError("unexpected admission")
            except Refusal as error:
                assert expected in str(error)
                checks.append(expected)
        store.execute("admin-a", "admin", "admin-a")
        store.execute("admin-b", "admin", "admin-b")
        try:
            store.execute("admin-c", "admin", "admin-c")
            raise AssertionError("unexpected admin admission")
        except Refusal as error:
            assert "administrative exhaustion" in str(error)
        checks.append("protected-admin-at-ordinary-saturation-and-explicit-admin-exhaustion")
        store.execute("close-a", "terminal", "a")
        assert store.lookup("a")["kind"] == "terminal"
        try:
            store.execute("new-a", "accepted", "a")
            raise AssertionError("reinstatement")
        except Refusal:
            checks.append("closure-after-both-logical-budgets-full-no-reinstatement")
        assert store.audit() == 5
        root = store.root
        store.close()
        if name != "C_sqlite":
            stale = (root / "index.jsonl").read_bytes()
            (root / "index.jsonl").write_bytes(stale.splitlines(keepends=True)[0])
            store = factory(root, blob, dependency)
            assert store.lookup("a")["kind"] == "terminal"
            store.close()
            (root / "index.jsonl").unlink()
            store = factory(root, blob, dependency)
            assert store.lookup("a")["kind"] == "terminal"
            store.close()
            checks += ["stale-index-rebuilt", "missing-index-rebuilt"]
            (root / "head.json").unlink()
            try:
                factory(root, blob, dependency)
                raise AssertionError("missing head admitted")
            except Refusal:
                checks.append("missing-head-fails-closed")
        else:
            store = factory(root, blob, dependency)
            store.db.execute("DELETE FROM head")
            try:
                store.lookup("a")
                raise AssertionError("missing head admitted")
            except Refusal:
                checks.append("missing-head-fails-closed")
            store.close()
            for coherent in (False, True):
                store = fresh("pointer-" + str(coherent))
                store.execute("a", "accepted", "a")
                store.execute("closed-a", "terminal", "a")
                if coherent:
                    store.db.execute("UPDATE current SET seq=1,proof=? WHERE target='a'",
                                     (digest(encoded(["research-current", "a", 1])),))
                    assert store.lookup("a")["kind"] == "accepted"
                    checks.append("coherent-pointer-plus-local-checksum-rewrite-not-detected-by-point-lookup")
                    action = store.audit
                else:
                    store.db.execute("UPDATE current SET seq=1 WHERE target='a'")
                    action = lambda: store.lookup("a")
                try:
                    action()
                    raise AssertionError("current pointer damage admitted")
                except Refusal:
                    checks.append("coherent-pointer-rewrite-detected-by-history-audit" if coherent else
                                  "pointer-only-rollback-detected-by-selected-checksum")
                store.close()
        # Damaged selected original/dependency; coherent deliberate rewriting is
        # not authenticated by locally rewritable hash/head. Separate fresh roots.
        for damage in ("original", "dependency", "unrelated", "truncation"):
            store = fresh(damage)
            store.execute("a", "accepted", "a")
            store.execute("b", "accepted", "b")
            if name == "C_sqlite":
                store.db.execute("PRAGMA foreign_keys=OFF")
                if damage == "dependency":
                    store.db.execute("DELETE FROM dependencies")
                elif damage == "truncation":
                    store.db.execute("UPDATE originals SET bytes=substr(bytes,1,length(bytes)-1)")
                else:
                    store.db.execute("DELETE FROM records WHERE seq=?", (2 if damage == "unrelated" else 1,))
            elif damage == "dependency":
                (store.root / "deps" / store.dep_id).unlink()
            else:
                row = store.records[2 if damage == "unrelated" else 1]
                path = store.root / "records" / row["locator"][0]
                if name == "A_files":
                    if damage == "truncation":
                        with path.open("r+b") as stream:
                            stream.truncate(row["size"] - 12)
                    else:
                        path.unlink()
                else:
                    with path.open("r+b") as stream:
                        if damage == "truncation":
                            stream.truncate(row["locator"][1] + row["size"] - 12)
                        else:
                            stream.seek(row["locator"][1])
                            stream.write(b"\xff" * 8)
            if damage == "unrelated":
                assert store.lookup("a")["kind"] == "accepted"
                checks.append("unrelated-damage-not-detected-by-selected-lookup")
                action = store.audit
            else:
                action = lambda: store.lookup("a")
            try:
                action()
                raise AssertionError("damaged data admitted")
            except (Refusal, OSError):
                checks.append(damage + "-damage-detected-by-" + ("audit" if damage == "unrelated" else "selected-lookup"))
            store.close()
        # Crash a child at exact publication stages; parent reopens actual disk.
        if name == "B_segments":
            store = fresh("missing-segment")
            store.execute("a", "accepted", "a")
            (store.root / "records" / store.records[1]["locator"][0]).unlink()
            try:
                store.lookup("a")
                raise AssertionError("missing segment admitted")
            except Refusal:
                checks.append("missing-segment-selected-refusal")
            store.close()
        for kind, stage in ((kind, stage) for kind in ("accepted", "terminal")
                            for stage in ("after_data", "after_commit")):
            store = fresh(kind + "-" + stage)
            if kind == "terminal":
                store.execute("prior-accepted", "accepted", "crash-item")
            root = store.root
            store.close()
            command = [sys.executable, str(Path(__file__).resolve()), "crash", "--backend", name,
                       "--root", str(root), "--stage", stage, "--kind", kind, "--fixture", args.fixture]
            if args.dependency:
                command += ["--dependency", args.dependency]
            completed = subprocess.run(command, capture_output=True)
            assert completed.returncode == 91, completed.stderr.decode(errors="replace")
            store = factory(root, blob, dependency)
            expected_seq = 2 if kind == "terminal" else 1
            if stage == "after_commit":
                assert store.execute("crash-key", kind, "crash-item") == ("replayed", expected_seq)
                outcome = "committed; original-key replay after absent reply"
            elif name == "C_sqlite":
                assert store.execute("crash-key", kind, "crash-item") == ("committed", expected_seq)
                outcome = "noncommitted SQL transaction rolled back on reopen"
            else:
                assert store.head["count"] == expected_seq - 1
                if kind == "terminal":
                    assert store.lookup("crash-item")["kind"] == "accepted"
                try:
                    store.execute("crash-key", kind, "crash-item")
                    raise AssertionError("orphan silently replaced")
                except Refusal:
                    outcome = "noncommitted head; orphan/tail refuses fresh publication; original recovery omitted"
            results.append(dict(backend=name, experiment="real abrupt child os._exit", kind=kind, stage=stage,
                                caller_result="missing reply; uncertain until original request recovery", outcome=outcome))
            store.close()
        results.append(dict(backend=name, experiment="modeled deliberate file/SQL damage and logical limits",
                            checks=checks, limits=dict(ordinary=2, admin=2), uncertainty="No injected physical I/O failure or power loss"))
    Path(args.output).write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
    print("fault checks passed", len(results))


def contention(args, blob, dependency):
    factory = STORES[args.backend]
    root = Path(args.root)
    if args.worker:
        store = factory(root, blob, dependency, admin_limit=200000)
        samples = []
        for position in range(12):
            samples.append(timed(lambda: store.execute(f"writer-{args.worker}-{position}", "accepted",
                                                       f"contended-{args.worker}-{position}"))["median_ms"])
        result = dict(worker=args.worker, samples_ms=samples, lock_wait_ms=store.lock_wait_ms)
        store.close()
        Path(args.output).write_text(json.dumps(result) + "\n", encoding="utf-8")
        return
    commands = []
    outputs = []
    for worker in ("one", "two"):
        output = Path(args.scratch) / f"contention-{args.backend}-{args.count}-{worker}.json"
        outputs.append(output)
        command = [sys.executable, str(Path(__file__).resolve()), "contention", "--backend", args.backend,
                   "--root", str(root), "--worker", worker, "--fixture", args.fixture,
                   "--output", str(output)]
        if args.dependency:
            command += ["--dependency", args.dependency]
        commands.append(command)
    before = time.perf_counter_ns()
    processes = [subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE) for command in commands]
    for process in processes:
        stdout, stderr = process.communicate(timeout=120)
        assert process.returncode == 0, stderr.decode(errors="replace")
    total_ms = (time.perf_counter_ns() - before) / 1e6
    store = factory(root, blob, dependency, admin_limit=200000)
    for worker in ("one", "two"):
        for position in range(12):
            assert store.lookup(f"contended-{worker}-{position}")["kind"] == "accepted"
    store.close()
    result = dict(backend=args.backend, initial_count=args.count, total_process_wall_ms=total_ms,
                  writers=2, committed=24, all_results_visible=True,
                  workers=[json.loads(path.read_text()) for path in outputs],
                  includes_process_startup_and_reopen=True, common_exclusive_owner_lock=True)
    Path(args.output).write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print("contention coherent", args.backend, total_ms)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("benchmark", "faults", "crash", "contention", "audit-existing"))
    parser.add_argument("--backend", choices=STORES)
    parser.add_argument("--fixture", required=True, help="opaque exact native accepted record bytes")
    parser.add_argument("--dependency", help="opaque exact native dependency proof bytes; default same fixture")
    parser.add_argument("--scratch", default=tempfile.gettempdir())
    parser.add_argument("--root")
    parser.add_argument("--output")
    parser.add_argument("--count", type=int, default=10000)
    parser.add_argument("--label", default="base")
    parser.add_argument("--stage")
    parser.add_argument("--kind", choices=("accepted", "terminal"), default="accepted")
    parser.add_argument("--worker")
    parser.add_argument("--stream-audit", action="store_true")
    args = parser.parse_args()
    blob = Path(args.fixture).read_bytes()
    dependency = Path(args.dependency).read_bytes() if args.dependency else blob
    if args.stream_audit:
        SegmentedStore.stream_audit = True
    if not 0 < len(blob) <= 1048576 or not 0 < len(dependency) <= 1048576:
        raise Refusal("fixture outside research byte bound")
    if args.mode == "benchmark":
        if not 10 <= args.count <= 100000:
            raise Refusal("population outside bounded research range")
        benchmark(args, blob, dependency)
    elif args.mode == "faults":
        faults(args, blob, dependency)
    elif args.mode == "contention":
        contention(args, blob, dependency)
    elif args.mode == "audit-existing":
        existing_audit(args, blob, dependency)
    else:
        store = STORES[args.backend](args.root, blob, dependency)
        store.execute("crash-key", args.kind, "crash-item", args.stage)
        raise AssertionError("child did not exit")


if __name__ == "__main__":
    main()
