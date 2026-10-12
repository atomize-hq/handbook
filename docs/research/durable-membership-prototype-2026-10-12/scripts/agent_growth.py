#!/usr/bin/env python3
"""Observe simultaneous prepared AGENT processes against retained growth stores.

This runner adds independent membership records to established native stores.
It never clones stores or changes their native ownership/lock identities.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import time

from run import Runner, environment, json_line, request


def current_rss(pid):
    """Return observed OS process counters; unavailable remains explicit."""
    started = time.perf_counter_ns()
    try:
        if os.name == "nt":
            import ctypes
            from ctypes import wintypes

            class Counters(ctypes.Structure):
                _fields_ = [("cb", wintypes.DWORD), ("PageFaultCount", wintypes.DWORD)] + [
                    (name, ctypes.c_size_t) for name in (
                        "PeakWorkingSetSize", "WorkingSetSize", "QuotaPeakPagedPoolUsage",
                        "QuotaPagedPoolUsage", "QuotaPeakNonPagedPoolUsage", "QuotaNonPagedPoolUsage",
                        "PagefileUsage", "PeakPagefileUsage", "PrivateUsage",
                    )]

            kernel = ctypes.WinDLL("kernel32", use_last_error=True)
            psapi = ctypes.WinDLL("psapi", use_last_error=True)
            kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
            kernel.OpenProcess.restype = wintypes.HANDLE
            kernel.CloseHandle.argtypes = [wintypes.HANDLE]
            kernel.CloseHandle.restype = wintypes.BOOL
            psapi.GetProcessMemoryInfo.argtypes = [wintypes.HANDLE, ctypes.POINTER(Counters), wintypes.DWORD]
            psapi.GetProcessMemoryInfo.restype = wintypes.BOOL
            # PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, own child processes.
            handle = kernel.OpenProcess(0x0400 | 0x0010, False, pid)
            if not handle:
                raise OSError(ctypes.get_last_error(), "OpenProcess")
            try:
                counters = Counters()
                counters.cb = ctypes.sizeof(counters)
                if not psapi.GetProcessMemoryInfo(handle, ctypes.byref(counters), counters.cb):
                    raise OSError(ctypes.get_last_error(), "GetProcessMemoryInfo")
                result = {"source": "GetProcessMemoryInfo WorkingSetSize", "rss_bytes": counters.WorkingSetSize,
                          "private_commit_bytes": counters.PrivateUsage, "available": True}
            finally:
                if not kernel.CloseHandle(handle):
                    raise OSError(ctypes.get_last_error(), "CloseHandle")
        else:
            lines = Path("/proc/%s/status" % pid).read_text().splitlines()
            values = {line.split(":", 1)[0]: line.split(":", 1)[1].strip() for line in lines if ":" in line}
            fields = values.get("VmRSS", "").split()
            if len(fields) != 2 or fields[1] != "kB":
                raise ValueError("VmRSS unavailable or unexpected unit")
            result = {"source": "/proc/PID/status VmRSS", "rss_bytes": int(fields[0]) * 1024,
                      "available": True}
    except (OSError, ValueError) as error:
        result = {"available": False, "rss_bytes": None, "unavailable": str(error)}
    result.update(pid=pid, observed_started_mono_ns=started, observed_finished_mono_ns=time.perf_counter_ns())
    return result


def cohort(runner, memory_log, store, variant, processes, trial, args, prefix):
    print("agent-growth", variant, "history", args.size, "processes", processes, "trial", trial, flush=True)
    before = runner.success(runner.call(store, variant, {"op": "snapshot"}, "agent-growth", "before-cohort",
                                        size=args.size, processes=processes, trial=trial), "agent-growth:before-snapshot")
    commands = []
    markers = []
    children = []
    started = time.perf_counter_ns()
    completed = False
    try:
        for agent in range(processes):
            name = "%s-%s-%s-%s-%s" % (prefix, variant, processes, trial, agent)
            marker = store / (name + ".prepared")
            runner.check(not marker.exists(), "agent-growth:fresh-marker", marker=str(marker))
            command = request(name, content="agent-growth exact original", pause_ms=args.pause_ms, ready_file=str(marker))
            commands.append(command)
            markers.append(marker)
            children.append(runner.launch(store, variant, command, "agent-growth", "independent", size=args.size,
                                           processes=processes, trial=trial, agent=agent))
        deadline = time.monotonic() + min(args.timeout, 90)
        while not all(marker.exists() for marker in markers) and time.monotonic() < deadline:
            if any(child.process.poll() is not None for child in children):
                break
            time.sleep(0.005)
        runner.check(all(marker.exists() for marker in markers), "agent-growth:all-prepared",
                     variant=variant, processes=processes, trial=trial, ready=[marker.exists() for marker in markers])
        alive_before = [child.process.poll() is None for child in children]
        runner.check(all(alive_before), "agent-growth:all-live-before-sample", variant=variant, processes=processes,
                     trial=trial, alive=alive_before)
        memory_started = time.perf_counter_ns()
        ages_ms = [(time.time_ns() - marker.stat().st_mtime_ns) / 1000000 for marker in markers]
        observed = [current_rss(child.process.pid) for child in children]
        memory_finished = time.perf_counter_ns()
        alive_after = [child.process.poll() is None for child in children]
        # A common sampling window must fit within every post-marker preparation pause.
        ages_after_ms = [(time.time_ns() - marker.stat().st_mtime_ns) / 1000000 for marker in markers]
        paused_window = all(age < args.pause_ms for age in ages_after_ms)
        measured = [entry["rss_bytes"] for entry in observed if entry["available"]]
        memory = {
            "format": "durable-membership-agent-memory-v1", "variant": variant, "size": args.size,
            "processes": processes, "trial": trial, "pause_ms": args.pause_ms,
            "sampling_started_mono_ns": memory_started, "sampling_finished_mono_ns": memory_finished,
            "sampling_window_ns": memory_finished - memory_started, "marker_age_before_ms": ages_ms,
            "marker_age_after_ms": ages_after_ms, "all_live_before": all(alive_before), "all_live_after": all(alive_after),
            "all_within_preparation_pause": paused_window, "agents": observed,
            "observed_rss_sum_bytes": sum(measured) if len(measured) == processes else None,
            "observed_available_rss_sum_bytes": sum(measured), "rss_agents_available": len(measured),
            "interpretation": "Timestamped OS-reported resident counters of overlapping prepared children, sampled sequentially in one short window. Shared pages may be counted once per process; this is not unique physical RAM or PSS. Linux VmRSS may lag. Individual child peaks are reported separately and their sum is not concurrent RSS.",
        }
        memory_log.write(json_line(memory) + "\n")
        memory_log.flush()
        runner.check(all(alive_after) and paused_window, "agent-growth:overlapping-preparation-sample",
                     variant=variant, processes=processes, trial=trial, alive_after=alive_after,
                     marker_ages_ms=ages_after_ms, pause_ms=args.pause_ms)
        if len(measured) != processes:
            runner.note("agent-growth:rss-unavailable", status="skip", variant=variant, processes=processes, trial=trial,
                        agents=[entry for entry in observed if not entry["available"]])
        records = [child.finish() for child in children]
        finished = time.perf_counter_ns()
        for record in records:
            runner.success(record, "agent-growth:child-success")
            runner.check(runner.status(record) == "committed", "agent-growth:all-committed",
                         variant=variant, processes=processes, trial=trial, response=record["response"])
        keys = [key for command in commands for key in ("r:" + command["request"], "i:" + command["item"])]
        # Boundary probes and semantic bindings share one reopened batched lookup.
        keys += ["u:%09d" % (args.size - 1), "u:%09d" % args.size]
        result = runner.success(runner.call(store, variant, {"op": "lookup", "keys": keys}, "agent-growth", "verify-cohort",
                                           size=args.size, processes=processes, trial=trial), "agent-growth:batched-membership")
        runner.check(result["seq"] - before["seq"] == processes, "agent-growth:exact-root-delta",
                     variant=variant, processes=processes, trial=trial, before_seq=before["seq"], after_seq=result["seq"])
        values = result["values"]
        runner.check(len(values) == len(keys), "agent-growth:lookup-shape", count=len(values))
        for agent, command in enumerate(commands):
            retained, item = values[2 * agent:2 * agent + 2]
            original = {name: command[name] for name in ("request", "item", "content", "proof")}
            runner.check(isinstance(retained, dict) and retained.get("original") == original
                         and retained.get("state") == "terminal" and item == command["request"],
                         "agent-growth:retained-exact-binding", variant=variant, processes=processes, trial=trial,
                         agent=agent, retained=retained, item=item)
        runner.check(values[-2] is not None and values[-1] is None, "agent-growth:history-boundaries", size=args.size)
        runner.note("agent-growth:cohort", variant=variant, size=args.size, processes=processes, trial=trial,
                    cohort_started_mono_ns=started, cohort_finished_mono_ns=finished, cohort_wall_ns=finished - started,
                    before_seq=before["seq"], after_seq=result["seq"], rss_observed_sum_bytes=memory["observed_rss_sum_bytes"],
                    child_peak_rss_kib=[record["response"].get("peak_rss_kib") for record in records],
                    interpretation="Cohort includes process startup, eager/lazy preparation, explicit pause, serialized publication and shutdown; excludes before-snapshot and after-verification commands. Peak values are individual, not simultaneous memory.")
        completed = True
    finally:
        if not completed:
            for child in children:
                if child.process.poll() is None:
                    child.process.kill()
            for child in children:
                child.finish()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--chunk-store")
    parser.add_argument("--disk-store")
    parser.add_argument("--work", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--trials", type=int, default=2)
    parser.add_argument("--size", type=int, default=100000)
    parser.add_argument("--pause-ms", type=int, default=5000)
    parser.add_argument("--processes", default="1,2,4,8")
    parser.add_argument("--variants", default="chunk,disk")
    parser.add_argument("--seed", type=int, default=20261012)
    parser.add_argument("--timeout", type=float, default=180)
    args = parser.parse_args()
    binary, work, output = Path(args.binary).resolve(), Path(args.work).resolve(), Path(args.output).resolve()
    variants = args.variants.split(",")
    processes = [int(value) for value in args.processes.split(",")]
    if set(variants) - {"chunk", "disk"} or set(processes) - {1, 2, 4, 8}:
        parser.error("variants chunk/disk and processes 1/2/4/8 required")
    if args.trials < 1 or args.size < 1 or args.size > 1000000 or not 1000 <= args.pause_ms <= 60000:
        parser.error("positive trials and bounded size/pause required (pause 1..60s)")
    if not binary.is_file() or not work.is_dir() or output.exists():
        parser.error("binary/work must exist and output must be new")
    stores = {}
    for variant in variants:
        argument = getattr(args, variant + "_store")
        if not argument:
            parser.error("--%s-store required for selected variant" % variant)
        store = Path(argument).resolve()
        try:
            store.relative_to(work)
        except ValueError:
            parser.error("store must lie within work ancestor")
        if not store.is_dir():
            parser.error("selected store does not exist")
        stores[variant] = store
    output.mkdir(parents=True)
    env = environment(binary, work, args)
    env["experiment"] = "Overlapping prepared AGENT process memory and publication cost against existing native growth stores"
    env["stores"] = {variant: str(store) for variant, store in stores.items()}
    output.joinpath("environment.json").write_text(json.dumps(env, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    prefix = "ag-" + hashlib.sha256(str(output).encode("utf-8")).hexdigest()[:12]
    runner = Runner(binary, output, work, args.timeout)
    try:
        with output.joinpath("simultaneous-memory.jsonl").open("w", encoding="utf-8", newline="\n") as memory_log:
            for variant in variants:
                for count in processes:
                    for trial in range(args.trials):
                        try:
                            cohort(runner, memory_log, stores[variant], variant, count, trial, args, prefix)
                        except Exception as error:
                            runner.note("agent-growth:cohort-aborted", status="fail", variant=variant, processes=count, trial=trial,
                                        error=type(error).__name__ + ": " + str(error))
                            print("FAILED", variant, count, trial, error, flush=True)
        status = {"complete": runner.failures == 0, "full_harness_complete": False, "passes": runner.passes,
                  "failures": runner.failures, "skips": runner.skips, "observations": runner.observations,
                  "variants": variants, "processes": processes, "trials": args.trials, "size": args.size,
                  "all_concurrent_rss_available": runner.skips == 0}
        output.joinpath("run-status.json").write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json_line(status), flush=True)
        return 1 if runner.failures else 0
    finally:
        runner.close()


if __name__ == "__main__":
    raise SystemExit(main())
