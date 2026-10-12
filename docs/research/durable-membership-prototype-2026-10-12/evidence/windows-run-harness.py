#!/usr/bin/env python3
"""Bounded prototype runner. Python stdlib only; no cache-drop or power-cut claim.

Each command is a fresh OS process unless its repeat field says otherwise. All
child responses and harness assertions are retained, including failures. Work
directories must be new: this runner never deletes an existing experiment.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import shutil
import subprocess
import sys
import threading
import time


CHECKPOINTS = (
    "object_write", "object_sync", "objects_dir_sync", "root_stage",
    "root_sync", "root_replace", "publication_sync", "before_reply",
)
PREPUBLICATION = set(CHECKPOINTS[:5]) | {"short_write"}


def json_line(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def file_hash(path):
    digest = hashlib.sha256()
    with open(path, "rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def capture(command):
    try:
        response = subprocess.run(command, capture_output=True, text=True, timeout=20)
        return {"command": command, "returncode": response.returncode,
                "stdout": response.stdout.strip(), "stderr": response.stderr.strip()}
    except (OSError, subprocess.TimeoutExpired) as error:
        return {"command": command, "unavailable": str(error)}


def environment(binary, work, arguments):
    result = {
        "format": "durable-membership-experiment-environment-v1",
        "utc_started": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "platform": platform.platform(), "system": platform.system(),
        "release": platform.release(), "version": platform.version(),
        "machine": platform.machine(), "processor": platform.processor(),
        "logical_cpus": os.cpu_count(), "python": sys.version,
        "binary": str(binary), "binary_sha256": file_hash(binary),
        "arguments": vars(arguments), "work": str(work),
        "disk_usage_before": shutil.disk_usage(work)._asdict(),
        "cache_control": "No OS cache eviction. Fresh processes reopen; OS cache is uncontrolled and usually warmed by initialization/prior samples.",
        "fault_model": "Injected IO/flush failures and exit(86) checkpoints; external process termination during preparation. These preserve OS cache and are not hardware power loss or a persistence simulator.",
        "unmeasured": ["physical IO", "OS syscall counts", "allocation counts", "power-cut durability"],
        "seed": arguments.seed,
        "source_sha256_at_run_start": {
            str(path.relative_to(Path(__file__).resolve().parents[1])): file_hash(path)
            for pattern in ("src/*.rs", "Cargo.*", "scripts/*.py")
            for path in sorted(Path(__file__).resolve().parents[1].glob(pattern)) if path.is_file()
        },
    }
    if os.name == "nt":
        drive = work.drive.rstrip(":")
        result["volume"] = capture([
            "powershell", "-NoProfile", "-Command",
            "Get-Volume -DriveLetter '" + drive + "' | Select-Object DriveLetter,FileSystem,FileSystemLabel,DriveType,AllocationUnitSize | ConvertTo-Json -Compress",
        ])
        result["storage"] = capture([
            "powershell", "-NoProfile", "-Command",
            "Get-PhysicalDisk | Select-Object FriendlyName,MediaType,BusType,Size | ConvertTo-Json -Compress",
        ])
    else:
        result["filesystem"] = capture(["stat", "-f", "-c", "%T", str(work)])
        result["mount"] = capture(["findmnt", "-T", str(work), "-o", "SOURCE,FSTYPE,TARGET,OPTIONS", "-J"])
        for filename in ("/proc/version", "/sys/class/dmi/id/product_name", "/sys/class/dmi/id/sys_vendor"):
            try:
                result[filename] = Path(filename).read_text().strip()
            except OSError as error:
                result[filename] = {"unavailable": str(error)}
        result["virtualization"] = capture(["systemd-detect-virt"])
    result["rustc"] = capture(["rustc", "-Vv"])
    result["cargo"] = capture(["cargo", "-V"])
    return result


class Child:
    def __init__(self, runner, store, variant, command, tags, injection=None):
        self.runner = runner
        self.store = store
        self.variant = variant
        self.command = command
        self.tags = tags
        self.injection = injection or {}
        env = os.environ.copy()
        env.pop("PROTO_FAULT", None)
        env.pop("PROTO_FAIL", None)
        env.update(self.injection)
        self.started = time.perf_counter_ns()
        self.process = subprocess.Popen(
            [str(runner.binary), str(store), variant], stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env,
        )
        self.process.stdin.write(json_line(command) + "\n")
        self.process.stdin.close()
        self.process.stdin = None
        self.record = None
        self.thread = threading.Thread(target=self.collect, daemon=True)
        self.thread.start()

    def collect(self):
        timed_out = False
        try:
            stdout, stderr = self.process.communicate(timeout=self.runner.timeout)
        except subprocess.TimeoutExpired:
            timed_out = True
            self.process.kill()
            stdout, stderr = self.process.communicate()
        finished = time.perf_counter_ns()
        response = None
        decode_error = None
        try:
            response = json.loads(stdout)
            if not isinstance(response, dict) or not isinstance(response.get("ok"), bool):
                raise ValueError("response must be an object with boolean ok")
        except (ValueError, json.JSONDecodeError) as error:
            decode_error = str(error)
        self.record = {
            "format": "durable-membership-experiment-sample-v1",
            "variant": self.variant, "store": str(self.store.relative_to(self.runner.work)),
            "command": self.command, "tags": self.tags, "injection": self.injection,
            "pid": self.process.pid, "returncode": self.process.returncode,
            "wall_ns": finished - self.started, "timed_out": timed_out,
            "started_mono_ns": self.started, "finished_mono_ns": finished,
            "response": response, "stderr": stderr.strip(),
        }
        if decode_error:
            self.record["decode_error"] = decode_error
            self.record["stdout"] = stdout
        self.runner.emit_sample(self.record)

    def finish(self):
        self.thread.join(self.runner.timeout + 5)
        if self.record is None:
            raise RuntimeError("child collector did not finish")
        return self.record


class Runner:
    def __init__(self, binary, output, work, timeout):
        self.binary, self.output, self.work, self.timeout = binary, output, work, timeout
        self.sample_lock = threading.Lock()
        self.samples = output.joinpath("samples.jsonl").open("w", encoding="utf-8", newline="\n")
        self.checks = output.joinpath("checks.jsonl").open("w", encoding="utf-8", newline="\n")
        self.failures, self.passes, self.skips, self.observations = 0, 0, 0, 0
        self.counter = 0

    def emit_sample(self, value):
        with self.sample_lock:
            self.samples.write(json_line(value) + "\n")
            self.samples.flush()

    def check(self, condition, name, **details):
        status = "pass" if condition else "fail"
        self.checks.write(json_line({"name": name, "status": status, **details}) + "\n")
        self.checks.flush()
        if condition:
            self.passes += 1
        else:
            self.failures += 1
            raise AssertionError(name + ": " + json_line(details))

    def note(self, name, status="observation", **details):
        self.checks.write(json_line({"name": name, "status": status, **details}) + "\n")
        self.checks.flush()
        if status == "skip":
            self.skips += 1
        elif status == "fail":
            self.failures += 1
        else:
            self.observations += 1

    def call(self, store, variant, command, suite, case, injection=None, **tags):
        return self.launch(store, variant, command, suite, case, injection, **tags).finish()

    def launch(self, store, variant, command, suite, case, injection=None, **tags):
        return Child(self, store, variant, command, {"suite": suite, "case": case, **tags}, injection)

    def success(self, record, name):
        response = record.get("response") or {}
        self.check(record["returncode"] == 0 and not record["timed_out"] and response.get("ok") is True,
                   name, variant=record["variant"], response=response,
                   returncode=record["returncode"], stderr=record["stderr"])
        return response.get("result")

    def status(self, record):
        response = record.get("response") or {}
        self.check(record["returncode"] in (0, 1) and not record["timed_out"] and
                   isinstance(response.get("ok"), bool), "structured-result", variant=record["variant"],
                   returncode=record["returncode"], response=response, stderr=record["stderr"])
        if response.get("ok"):
            return (response.get("result") or {}).get("status")
        self.check(isinstance(response.get("error"), str) and bool(response["error"]), "explicit-refusal-error", response=response)
        return "refused"

    def new_store(self, variant, label, n=1000):
        self.counter += 1
        store = self.work / (variant + "-" + label + "-" + str(self.counter))
        store.mkdir()
        self.success(self.call(store, variant, {"op": "init", "n": n}, "setup", label, size=n), "init:" + label)
        return store

    def lookup(self, store, variant, keys, suite, case):
        result = self.success(self.call(store, variant, {"op": "lookup", "keys": keys}, suite, case), case)
        self.check(isinstance(result, dict) and isinstance(result.get("values"), list) and len(result["values"]) == len(keys),
                   case + ":shape", result=result)
        return result["values"]

    def membership(self, store, variant, request, item, exists, case):
        values = self.lookup(store, variant, ["r:" + request, "i:" + item], "correctness", case)
        self.check(all(value is not None for value in values) if exists else all(value is None for value in values),
                   case + ":atomic-membership", values=values, expected_exists=exists)
        if exists:
            self.check(isinstance(values[0], dict) and values[0].get("original", {}).get("request") == request
                       and values[0].get("original", {}).get("item") == item and values[1] == request,
                       case + ":request-item-binding", values=values)
        return values

    def wait_ready(self, child, marker, name):
        deadline = time.monotonic() + min(self.timeout, 20)
        while time.monotonic() < deadline and not marker.exists() and child.process.poll() is None:
            time.sleep(0.005)
        self.check(marker.exists(), name + ":prepared-marker", stderr=(child.record or {}).get("stderr"))

    def close(self):
        self.samples.close()
        self.checks.close()


def request(name, item=None, content="content", stage="terminal", **kwargs):
    return {"op": "run", "request": name, "item": item or name, "content": content,
            "proof": 1, "stage": stage, **kwargs}


def measurement(runner, variant, sizes, samples, seed):
    rng = random.Random(seed)
    for size in sizes:
        print("measure", variant, size, flush=True)
        store = runner.new_store(variant, "growth-" + str(size), size)
        initial_storage = storage_size(store)
        runner.note("growth-storage-initial", variant=variant, size=size, **initial_storage)
        replay = request("replay-" + str(size), content="retained exact original")
        committed = runner.call(store, variant, replay, "measure", "replay-setup", size=size)
        runner.success(committed, "replay-setup")
        runner.check(runner.status(committed) == "committed", "replay-setup:committed", variant=variant)
        for sample in range(samples):
            index = rng.randrange(size)
            commands = [
                ("positive-fresh", {"op": "lookup", "keys": ["u:%09d" % index], "instrument": True}),
                ("negative-fresh", {"op": "lookup", "keys": ["u:999999999"], "instrument": True}),
                ("replay-fresh", replay),
                ("insert-fresh", request("insert-%s-%s" % (size, sample))),
            ]
            rng.shuffle(commands)
            for case, command in commands:
                record = runner.call(store, variant, command, "measure", case, size=size, sample=sample)
                result = runner.success(record, case)
                if case.startswith("positive"):
                    runner.check(result["values"][0] == "x" * 96, case + ":exact-value", variant=variant)
                elif case.startswith("negative"):
                    runner.check(result["values"][0] is None, case + ":absent", variant=variant)
                else:
                    expected = "replay" if case.startswith("replay") else "committed"
                    runner.check(runner.status(record) == expected, case + ":status", expected=expected, result=result)
        keys = ["u:%09d" % rng.randrange(size) for _ in range(64)]
        for sample in range(samples):
            for case, command in (
                ("warm-repeat", {"op": "lookup", "keys": [keys[0]], "repeat": 100, "instrument": True}),
                ("lookup-batch-64", {"op": "lookup", "keys": keys, "instrument": True}),
            ):
                runner.success(runner.call(store, variant, command, "measure", case, size=size, sample=sample), case)
            # Alternate ordering to avoid assigning all first-in-pair cache cost to one mode.
            for instrument in ((True, False) if sample % 2 else (False, True)):
                runner.success(runner.call(store, variant, {"op": "lookup", "keys": [keys[sample % 64]], "instrument": instrument},
                                           "measure", "instrumentation", size=size, sample=sample, instrument=instrument), "instrumentation")
        for sample in range(samples):
            for count, case in ((1, "update-1"), (64, "write-batch-64")):
                pairs = [["u:%09d" % ((sample * 64 + offset) % size), "v" * 95 + str(sample % 10)] for offset in range(count)]
                record = runner.call(store, variant, {"op": "put", "pairs": pairs}, "measure", case, size=size, sample=sample)
                runner.success(record, case)
                verified = runner.lookup(store, variant, [pairs[0][0], pairs[-1][0]], "correctness", "verify-" + case)
                runner.check(verified == [pairs[0][1], pairs[-1][1]], "put:durable-values", variant=variant, case=case, values=verified)
        before = storage_size(store)
        runner.note("growth-storage", variant=variant, size=size, **before)
        runner.note("growth-storage-delta", variant=variant, size=size,
                    workload="replay setup, repeated fresh insert, update1 and write64; all proposals and old objects retained",
                    **{name + "_delta": before[name] - initial_storage[name]
                       for name in ("storage_files", "storage_bytes", "storage_objects")})
        if size != min(sizes):
            runner.success(runner.call(store, variant, {"op": "audit"}, "measure", "broad-audit", size=size), "broad-audit")
            continue
        runner.note("sibling-stress-scope", variant=variant, size=size,
                    scope="Only smallest measured history; 0/1k/10k unrelated sibling files. Core growth uses exact guards at every size.")
        siblings = store / "siblings"
        siblings.mkdir(exist_ok=True)
        count = 0
        for sibling_count in (0, 1000, 10000):
            for number in range(count, sibling_count):
                (siblings / ("sibling-%09d" % number)).write_bytes(b"unrelated\n")
            count = sibling_count
            for sample in range(samples):
                for guard in ("exact", "list"):
                    record = runner.call(store, variant, {"op": "lookup", "keys": [keys[0]], "guard": guard, "instrument": True},
                                         "measure", "sibling-guard", size=size, sample=sample, siblings=sibling_count, guard=guard)
                    runner.success(record, "sibling-guard")
        runner.success(runner.call(store, variant, {"op": "audit"}, "measure", "broad-audit", size=size), "broad-audit")


def storage_size(store):
    files, byte_count, objects = 0, 0, 0
    for base, _, names in os.walk(store):
        for name in names:
            path = Path(base) / name
            files += 1
            byte_count += path.stat().st_size
            if "objects" in path.relative_to(store).parts:
                objects += 1
    return {"storage_files": files, "storage_bytes": byte_count, "storage_objects": objects,
            "storage_metric": "logical file bytes; no allocation/compression/dedup estimate"}


def concurrency(runner, variant):
    for processes in (1, 2, 4, 8):
        for trial in range(3):
            store = runner.new_store(variant, "independent-%s-%s" % (processes, trial))
            children = [runner.launch(store, variant, request("agent-%s" % number, pause_ms=300),
                                      "concurrency", "independent", processes=processes, trial=trial, agent=number)
                        for number in range(processes)]
            records = [child.finish() for child in children]
            for number, record in enumerate(records):
                runner.success(record, "independent")
                runner.check(runner.status(record) == "committed", "independent:all-committed", variant=variant, processes=processes)
                runner.membership(store, variant, "agent-%s" % number, "agent-%s" % number, True, "independent:retained")
    for case in ("same-request-same-content", "same-request-different-content", "same-item"):
        store = runner.new_store(variant, case)
        commands = [request("same" if case != "same-item" else "request-%s" % number,
                            item="shared", content=("content-%s" % number if case == "same-request-different-content" else "content"),
                            pause_ms=300) for number in range(4)]
        records = [child.finish() for child in [runner.launch(store, variant, command, "concurrency", case, processes=4, agent=number)
                                              for number, command in enumerate(commands)]]
        statuses = [runner.status(record) for record in records]
        runner.check(statuses.count("committed") == 1, case + ":one-commit", variant=variant, statuses=statuses)
        if case == "same-request-same-content":
            runner.check(statuses.count("replay") == 3, case + ":exact-replay", statuses=statuses)
        else:
            runner.check(statuses.count("refused") == 3, case + ":conflicts-refused", statuses=statuses)
        winner = statuses.index("committed")
        values = runner.membership(store, variant, commands[winner]["request"], "shared", True, case + ":winner-retained")
        runner.check(values[0].get("original", {}).get("content") == commands[winner]["content"]
                     and values[0].get("original", {}).get("proof") == 1,
                     case + ":exact-original", record=values[0])
        if case == "same-item":
            memberships = runner.lookup(store, variant, ["r:" + command["request"] for command in commands],
                                        "concurrency", "same-item-request-memberships")
            runner.check(sum(value is not None for value in memberships) == 1 and memberships[winner] is not None,
                         "same-item:losers-not-members", values=memberships)
    store = runner.new_store(variant, "stale-root")
    marker = store / "prepared"
    slow = runner.launch(store, variant, request("stale", pause_ms=1200, ready_file=str(marker)), "concurrency", "stale-base")
    runner.wait_ready(slow, marker, "stale-base")
    runner.success(runner.call(store, variant, request("intervening"), "concurrency", "stale-intervening"), "intervening")
    result = slow.finish()
    runner.success(result, "stale-base")
    runner.check((result["response"].get("metrics") or {}).get("retries", 0) >= 1, "stale-base:revalidated", result=result["response"])
    runner.membership(store, variant, "stale", "stale", True, "stale:retained")
    runner.membership(store, variant, "intervening", "intervening", True, "intervening:retained")
    store = runner.new_store(variant, "revoke-race")
    marker = store / "prepared"
    slow = runner.launch(store, variant, request("revoked", pause_ms=1200, ready_file=str(marker)), "concurrency", "revoke-race")
    runner.wait_ready(slow, marker, "revoke-race")
    runner.success(runner.call(store, variant, {"op": "authority", "epoch": 2, "scope": False}, "concurrency", "revoke"), "revoke")
    result = slow.finish()
    runner.check(runner.status(result) == "refused", "revoke-race:refused", response=result["response"])
    runner.membership(store, variant, "revoked", "revoked", False, "revoke-race:absent")
    replay_after_revoke(runner, variant)
    held_reader(runner, variant)
    held_publication_handle(runner, variant)


def replay_after_revoke(runner, variant):
    store = runner.new_store(variant, "semantic")
    original_command = request("original", content="exact retained content")
    original = runner.call(store, variant, original_command, "correctness", "original")
    original_result = runner.success(original, "original")
    payloads = store / "payloads"
    payloads.mkdir(exist_ok=True)
    sidecar = payloads / "original"
    sidecar.write_bytes(b"optional payload deleted below")
    sidecar.unlink()
    runner.membership(store, variant, "original", "original", True, "payload-loss:membership-independent")
    reserve = runner.call(store, variant, request("pending", stage="reserve"), "correctness", "reserve")
    runner.success(reserve, "reserve")
    runner.check(runner.status(reserve) == "reserved", "reservation:retained")
    accepted = runner.call(store, variant, request("pending", stage="accept"), "correctness", "accept")
    runner.success(accepted, "accept")
    runner.check(runner.status(accepted) == "accepted", "acceptance:distinct")
    attempt = runner.call(store, variant, request("pending", stage="attempt"), "correctness", "attempt")
    runner.success(attempt, "attempt")
    runner.check(runner.status(attempt) == "uncertain", "attempt:uncertain", response=attempt["response"])
    reconciliation = runner.call(store, variant, request("pending"), "correctness", "terminal-after-attempt")
    runner.success(reconciliation, "terminal-after-attempt")
    runner.check(runner.status(reconciliation) == "uncertain", "attempt:no-auto-terminal", response=reconciliation["response"])
    runner.success(runner.call(store, variant, {"op": "authority", "epoch": 2, "scope": False}, "correctness", "revoke"), "revoke")
    replay = runner.call(store, variant, original_command, "correctness", "replay-after-revoke")
    replay_result = runner.success(replay, "replay-after-revoke")
    runner.check(runner.status(replay) == "replay" and replay_result.get("outcome") == original_result.get("outcome")
                 and "outcome" in original_result, "replay:original-outcome", original=original_result, replay=replay_result)
    changed = runner.call(store, variant, request("original", content="changed"), "correctness", "changed-content")
    runner.check(runner.status(changed) == "refused", "replay:changed-refused", response=changed["response"])
    resumed = runner.call(store, variant, request("pending"), "correctness", "resume-after-revoke")
    runner.check(runner.status(resumed) == "refused", "resume:current-proof-required", response=resumed["response"])
    values = runner.membership(store, variant, "pending", "pending", True, "pending:retained")
    runner.check(values[0].get("state") == "attempted", "pending:uncertainty-retained", record=values[0])


def held_reader(runner, variant):
    store = runner.new_store(variant, "held-reader")
    marker = store / "reader-prepared"
    held = runner.launch(store, variant, {"op": "hold", "keys": ["r:new"], "pause_ms": 1200, "ready_file": str(marker)},
                         "concurrency", "held-reader")
    runner.wait_ready(held, marker, "held-reader")
    runner.success(runner.call(store, variant, request("new"), "concurrency", "held-reader-publication"), "held-reader-publication")
    result = runner.success(held.finish(), "held-reader")
    runner.check(result["values"] == [None], "held-reader:old-snapshot", result=result)
    runner.membership(store, variant, "new", "new", True, "held-reader:new-root")
    runner.note("held-reader-model", variant=variant, limitation="Retained root and immutable object reachability; no mmap is used by these prototypes and no mapping-lifetime claim is made.")


def held_publication_handle(runner, variant):
    store = runner.new_store(variant, "held-root-handle")
    old_bytes = (store / "ROOT").read_bytes()
    if os.name == "nt":
        import ctypes
        from ctypes import wintypes
        kernel = ctypes.WinDLL("kernel32", use_last_error=True)
        kernel.CreateFileW.argtypes = [wintypes.LPCWSTR, wintypes.DWORD, wintypes.DWORD, wintypes.LPVOID,
                                      wintypes.DWORD, wintypes.DWORD, wintypes.HANDLE]
        kernel.CreateFileW.restype = wintypes.HANDLE
        kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        kernel.CloseHandle.restype = wintypes.BOOL
        handle = kernel.CreateFileW(str(store / "ROOT"), 0x80000000, 1, None, 3, 0, None)
        if handle == ctypes.c_void_p(-1).value:
            raise OSError(ctypes.get_last_error(), "CreateFileW FILE_SHARE_READ only")
        try:
            record = runner.call(store, variant, request("held-root"), "concurrency", "held-root-no-delete-share")
            runner.check(runner.status(record) == "refused", "held-root:explicit-replacement-failure", response=record["response"])
            runner.check((store / "ROOT").read_bytes() == old_bytes, "held-root:old-root-retained")
            runner.membership(store, variant, "held-root", "held-root", False, "held-root:not-published")
        finally:
            if not kernel.CloseHandle(handle):
                raise OSError(ctypes.get_last_error(), "CloseHandle")
        retry = runner.call(store, variant, request("held-root"), "concurrency", "held-root-after-release")
        runner.success(retry, "held-root-after-release")
        runner.check(runner.status(retry) == "committed", "held-root:retry-after-release")
    else:
        with (store / "ROOT").open("rb") as held:
            record = runner.call(store, variant, request("held-root"), "concurrency", "held-root-open-descriptor")
            runner.success(record, "held-root-open-descriptor")
            runner.check(runner.status(record) == "committed", "held-root:posix-publication")
            runner.check(held.read() == old_bytes and (store / "ROOT").read_bytes() != old_bytes,
                         "held-root:old-descriptor-new-path")
    runner.membership(store, variant, "held-root", "held-root", True, "held-root:retained")


def faults(runner, variant):
    acknowledged = request("acknowledged", content="never lose this acknowledged outcome")
    for mode in ("PROTO_FAULT", "PROTO_FAIL"):
        for checkpoint in CHECKPOINTS + (("short_write",) if mode == "PROTO_FAIL" else ()):
            store = runner.new_store(variant, mode + "-" + checkpoint)
            acknowledged_result = runner.success(runner.call(store, variant, acknowledged, "faults", "acknowledged-base"), "acknowledged-base")
            target = request("fault-target")
            record = runner.call(store, variant, target, "faults", checkpoint, {mode: checkpoint}, mode=mode)
            if mode == "PROTO_FAULT":
                runner.check(record["returncode"] == 86 and record["response"] is None, "fault:checkpoint-reached",
                             variant=variant, checkpoint=checkpoint, mode=mode, record=record)
            else:
                runner.check(record["response"] is not None and record["response"].get("ok") is False,
                             "io-fault:refused", variant=variant, checkpoint=checkpoint, mode=mode, record=record)
            old = runner.call(store, variant, acknowledged, "faults", "acknowledged-replay", checkpoint=checkpoint, mode=mode)
            runner.success(old, "acknowledged-replay")
            runner.check(runner.status(old) == "replay" and old["response"]["result"].get("outcome") == acknowledged_result.get("outcome"),
                         "fault:acknowledged-survives", checkpoint=checkpoint, mode=mode)
            values = runner.lookup(store, variant, ["r:fault-target", "i:fault-target"], "faults", "recover-selected-root")
            exists = values[0] is not None
            runner.check((values[1] is not None) == exists, "fault:whole-old-or-new", checkpoint=checkpoint, mode=mode, values=values)
            if checkpoint in PREPUBLICATION:
                runner.check(not exists, "fault:prepublication-old", checkpoint=checkpoint, mode=mode)
            if checkpoint == "before_reply":
                runner.check(exists, "fault:commit-before-reply-new", mode=mode)
            retry = runner.call(store, variant, target, "faults", "recovery-retry", checkpoint=checkpoint, mode=mode)
            runner.success(retry, "recovery-retry")
            runner.check(runner.status(retry) == ("replay" if exists else "committed"), "fault:retry-original", checkpoint=checkpoint, mode=mode)
            runner.membership(store, variant, "fault-target", "fault-target", True, "fault:retained-after-retry")
    # External termination is separate from instrumented abrupt exits.
    store = runner.new_store(variant, "external-kill")
    runner.success(runner.call(store, variant, acknowledged, "faults", "acknowledged-base"), "acknowledged-base")
    marker = store / "prepared"
    child = runner.launch(store, variant, request("killed", pause_ms=10000, ready_file=str(marker)), "faults", "external-kill")
    runner.wait_ready(child, marker, "external-kill")
    child.process.kill()
    record = child.finish()
    runner.check(record["returncode"] != 0, "external-kill:terminated", returncode=record["returncode"])
    runner.membership(store, variant, "killed", "killed", False, "external-kill:not-published")
    replay = runner.call(store, variant, acknowledged, "faults", "external-kill-replay")
    runner.success(replay, "external-kill-replay")
    runner.check(runner.status(replay) == "replay", "external-kill:acknowledged-survives")


def validation(runner, variant):
    def fresh(label):
        store = runner.new_store(variant, label)
        runner.success(runner.call(store, variant, request("known"), "validation", "known"), "known")
        return store

    for kind in ("torn-root", "missing-selected-node", "corrupt-selected-node", "missing-lock", "replaced-lock", "missing-owner", "corrupt-owner"):
        store = fresh(kind)
        root_bytes = (store / "ROOT").read_bytes()
        root = json.loads(root_bytes)["root"]
        node = root["node"]
        selected = store / "objects" / node[:2] / node
        if kind == "torn-root":
            (store / "ROOT").write_bytes(root_bytes[:max(1, len(root_bytes) // 2)])
        elif kind == "missing-selected-node":
            selected.unlink()
        elif kind == "corrupt-selected-node":
            data = bytearray(selected.read_bytes())
            data[len(data) // 2] ^= 1
            selected.write_bytes(data)
        elif kind == "missing-lock":
            (store / "LOCK").unlink()
        elif kind == "replaced-lock":
            (store / "LOCK").rename(store / "LOCK.old")
            (store / "LOCK").write_bytes(b"new inode, same cooperative lock pathname")
        elif kind == "missing-owner":
            (store / "OWNER").unlink()
        elif kind == "corrupt-owner":
            (store / "OWNER").write_bytes(b"different native owner identity")
        record = runner.call(store, variant, {"op": "lookup", "keys": ["r:known"]}, "validation", kind)
        runner.check(record["response"] is not None and record["response"].get("ok") is False,
                     kind + ":explicit-corrupt-refusal", variant=variant, record=record)
    store = fresh("unrelated-damage")
    unrelated = store / "objects" / "ff" / ("f" * 64)
    unrelated.parent.mkdir(exist_ok=True)
    unrelated.write_bytes(b"malformed unrelated immutable object")
    runner.membership(store, variant, "known", "known", True, "exact:unrelated-damage")
    audit = runner.call(store, variant, {"op": "audit"}, "validation", "audit-unrelated-damage")
    runner.check(audit["response"] is not None and audit["response"].get("ok") is False,
                 "audit:unrelated-damage-detected", variant=variant, record=audit)
    # Deliberate coherent local rollback illustrates the absence of an external witness.
    store = fresh("coherent-rollback")
    root_bytes = (store / "ROOT").read_bytes()
    runner.success(runner.call(store, variant, request("later"), "validation", "rollback-acknowledged"), "rollback-acknowledged")
    (store / "ROOT").write_bytes(root_bytes)
    values = runner.lookup(store, variant, ["r:known", "r:later"], "validation", "coherent-rollback")
    runner.check(values[0] is not None and values[1] is None, "rollback:local-witness-limitation", values=values)
    runner.note("coherent-local-rollback", variant=variant, limitation="Restoring a valid prior root hides a later acknowledged record; local checks cannot detect this excluded rollback threat.")
    link = runner.work / (variant + "-symlink-boundary")
    base = fresh("symlink-target")
    try:
        link.symlink_to(base, target_is_directory=True)
    except OSError as error:
        runner.note("symlink-store-boundary", status="skip", variant=variant, reason=str(error))
    else:
        result = runner.call(link, variant, {"op": "lookup", "keys": ["r:known"]}, "validation", "symlink-store-boundary")
        runner.check(result["response"] is not None and result["response"].get("ok") is False,
                     "symlink:boundary-refused", variant=variant, record=result)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--work", required=True)
    parser.add_argument("--sizes", default="1000,10000,100000")
    parser.add_argument("--samples", type=int, default=30)
    parser.add_argument("--seed", type=int, default=20261012)
    parser.add_argument("--timeout", type=float, default=180)
    parser.add_argument("--sections", default="measure,concurrency,faults,validation")
    parser.add_argument("--variants", default="chunk,disk")
    args = parser.parse_args()
    binary, output, work = Path(args.binary).resolve(), Path(args.output).resolve(), Path(args.work).resolve()
    if not binary.is_file():
        parser.error("binary does not exist")
    sizes = [int(value) for value in args.sizes.split(",")]
    if args.samples < 1 or any(size < 1 or size > 1000000 for size in sizes):
        parser.error("positive samples and sizes <= 1,000,000 required")
    variants = args.variants.split(",")
    sections = args.sections.split(",")
    if set(variants) - {"chunk", "disk"} or set(sections) - {"measure", "concurrency", "faults", "validation"}:
        parser.error("unknown variant or section")
    if work.exists() or output.exists():
        parser.error("work and output must both be new directories; retained results are never overwritten")
    work.mkdir(parents=True)
    output.mkdir(parents=True)
    output.joinpath("environment.json").write_text(json.dumps(environment(binary, work, args), indent=2, sort_keys=True) + "\n", encoding="utf-8")
    runner = Runner(binary, output, work, args.timeout)
    try:
        for variant in variants:
            for section in sections:
                print(section, variant, flush=True)
                try:
                    if section == "measure":
                        measurement(runner, variant, sizes, args.samples, args.seed)
                    else:
                        {"concurrency": concurrency, "faults": faults, "validation": validation}[section](runner, variant)
                except Exception as error:
                    # Continue independent suites, preserving the first causal failure.
                    runner.note("suite-aborted", status="fail", variant=variant, suite=section,
                                error=type(error).__name__ + ": " + str(error))
                    print("FAILED", variant, section, error, flush=True)
        status = {"passes": runner.passes, "failures": runner.failures, "skips": runner.skips,
                  "observations": runner.observations, "complete": runner.failures == 0,
                  "sections": sections, "variants": variants,
                  "full_harness_complete": runner.failures == 0 and set(sections) == {"measure", "concurrency", "faults", "validation"}
                                           and set(variants) == {"chunk", "disk"},
                  "disk_usage_after": shutil.disk_usage(work)._asdict()}
        output.joinpath("run-status.json").write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json_line(status), flush=True)
        return 1 if runner.failures else 0
    finally:
        runner.close()


if __name__ == "__main__":
    raise SystemExit(main())
