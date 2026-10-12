#!/usr/bin/env python3
"""Measure buffered transaction coalescing, cancellation, and overlapping RSS.

Independent native stores receive identical ordered updates in sequential and
coalesced modes. Each timed command is a fresh process; initialization and
verification are separate. OS caches are uncontrolled, not cold. Counters are
application helpers/logical bytes, never syscalls, allocation counts, or device
IO. Overlap memory samples are a common live window, never summed peak RSS.
"""
import argparse
import hashlib
import json
from pathlib import Path
import random
import statistics
import time

from agent_growth import current_rss
from run import environment, json_line, storage_size
from transaction_tests import Runner, get, put, tx


def measured_fields(record):
    response = record["response"]
    result = response.get("result") or {}
    metrics = response.get("metrics") or {}
    return {"wall_ns": record["wall_ns"], "command_ns": response.get("elapsed_ns"),
            "open_ns": result.get("open_ns"), "core_ns": result.get("core_ns"),
            "peak_rss_kib": response.get("peak_rss_kib"), **metrics}


def buffer_measurements(runner, variant, sizes, samples, seed, rows):
    rng = random.Random(seed)
    stores = {}
    for size in sizes:
        print("transaction-measure", variant, size, flush=True)
        modes = {mode: runner.new_store(variant, "tx-%s-%s" % (size, mode), size)
                 for mode in ("sequential", "coalesced")}
        stores[size] = modes["coalesced"]
        for mode, store in modes.items():
            runner.note("tx-measure:initial-storage", variant=variant, size=size, mode=mode, **storage_size(store))
        # Every A/B pair has exactly the same state history and values. Alternate
        # A/B command order; separate native identity prevents cross-mode reuse.
        for sample in range(samples):
            for count in (1, 64):
                start = rng.randrange(size)
                pairs = [["u:%09d" % ((start + offset) % size),
                          ("value:%s:%s:%s" % (sample, count, offset)).ljust(96, "v")]
                         for offset in range(count)]
                actions = [put(key, value) for key, value in pairs]
                logical_bytes = sum(len(key.encode("utf-8")) + len(value.encode("utf-8")) for key, value in pairs)
                order = ("sequential", "coalesced") if sample % 2 == 0 else ("coalesced", "sequential")
                for mode in order:
                    store = modes[mode]
                    before = storage_size(store)
                    record = runner.call(store, variant, tx(actions, coalesce=(mode == "coalesced")),
                        "transaction-measure", "commit-%s" % count, size=size, sample=sample, mode=mode,
                        pairs=count, logical_bytes=logical_bytes)
                    result = runner.success(record, "tx-measure:commit")
                    runner.check(result["status"] == "committed", "tx-measure:committed", variant=variant, mode=mode)
                    after = storage_size(store)
                    row = {"variant": variant, "size": size, "sample": sample, "mode": mode,
                           "case": "commit-%s" % count, "pairs": count, "logical_input_bytes": logical_bytes,
                           **measured_fields(record),
                           **{field + "_delta": after[field] - before[field]
                              for field in ("storage_files", "storage_bytes", "storage_objects")}}
                    rows.append(row)
                    runner.note("tx-measure:command", **row,
                        interpretation="One atomic selected-root transition; sequential mode persists each pair's intermediate tree inside commit, coalesced mode persists bounded final changes. Retained logical file deltas include proposals/staging, not allocated blocks.")
                # Identical bounded overlay cancellation performs genuine logical
                # puts, then ends before acquiring writer lock/object installation.
                for ending in ("cancel", "drop"):
                    store = modes["coalesced"]
                    before_root = (store / "ROOT").read_bytes()
                    before = storage_size(store)
                    cancelled_actions = [put(key, (ending + ":" + value)[:96]) for key, value in pairs]
                    record = runner.call(store, variant, tx(cancelled_actions, end=ending), "transaction-measure", ending,
                                         size=size, sample=sample, mode="coalesced", pairs=count, logical_bytes=logical_bytes)
                    result = runner.success(record, "tx-measure:" + ending)
                    runner.check(result["status"] == ("cancelled" if ending == "cancel" else "dropped"),
                                 "tx-measure:" + ending + "-status", result=result)
                    after = storage_size(store)
                    row = {"variant": variant, "size": size, "sample": sample, "mode": "coalesced",
                           "case": ending, "pairs": count, "logical_input_bytes": logical_bytes,
                           **measured_fields(record),
                           **{field + "_delta": after[field] - before[field]
                              for field in ("storage_files", "storage_bytes", "storage_objects")}}
                    rows.append(row)
                    runner.note("tx-measure:cancellation", **row)
                    runner.check((store / "ROOT").read_bytes() == before_root and row["writes"] == 0
                                 and row["nodes_written"] == 0 and row["storage_bytes_delta"] == 0
                                 and row["lock_hold_ns"] == 0, "tx-measure:cancelled-buffer-zero-disk-waste", row=row)
            for mode, store in modes.items():
                for case, command in (
                    ("positive-fresh", {"op": "lookup", "keys": ["u:%09d" % (size - 1)]}),
                    ("negative-fresh", {"op": "lookup", "keys": ["u:999999999"]}),
                    ("warm-repeat", {"op": "lookup", "keys": ["u:%09d" % (size - 1)], "repeat": 100}),
                    ("lookup-batch-64", {"op": "lookup", "keys": ["u:%09d" % (index % size) for index in range(64)]}),
                ):
                    record = runner.call(store, variant, command, "transaction-measure", case, size=size, sample=sample, mode=mode)
                    result = runner.success(record, "tx-measure:" + case)
                    runner.check(result["values"][0] is None if case == "negative-fresh" else result["values"][0] is not None,
                                 "tx-measure:lookup-membership", case=case, mode=mode, variant=variant)
                    row = {"variant": variant, "size": size, "sample": sample, "mode": mode,
                           "case": case, "pairs": 0, **measured_fields(record)}
                    rows.append(row)
        for mode, store in modes.items():
            # Reopen and selected audit verify the measured history independently.
            runner.success(runner.call(store, variant, {"op": "audit"}, "transaction-measure", "final-audit", size=size, mode=mode),
                           "tx-measure:final-audit")
            runner.note("tx-measure:final-storage", variant=variant, size=size, mode=mode, **storage_size(store))
    return stores


def cohort(runner, memory_log, store, variant, processes, trial, args, prefix):
    print("transaction-cohort", variant, processes, trial, flush=True)
    markers, commands, children = [], [], []
    started = time.perf_counter_ns()
    completed = False
    try:
        for index in range(processes):
            key = "u:%s:%s:%s:%s" % (prefix, processes, trial, index)
            marker = store / (key.replace(":", "-") + ".prepared")
            command = tx([get("u:000099999"), put(key, "independent-buffer")], ready_file=str(marker), pause_ms=args.pause_ms)
            markers.append(marker)
            commands.append(command)
            children.append(runner.launch(store, variant, command, "transaction-cohort", "prepared-independent",
                                           size=100000, processes=processes, trial=trial, agent=index))
        deadline = time.monotonic() + min(args.timeout, 60)
        while not all(marker.exists() for marker in markers) and time.monotonic() < deadline:
            if any(child.process.poll() is not None for child in children):
                break
            time.sleep(0.005)
        runner.check(all(marker.exists() for marker in markers), "tx-cohort:all-prepared", variant=variant,
                     processes=processes, trial=trial, markers=[marker.exists() for marker in markers])
        live_before = [child.process.poll() is None for child in children]
        memory_started = time.perf_counter_ns()
        observed = [current_rss(child.process.pid) for child in children]
        memory_finished = time.perf_counter_ns()
        live_after = [child.process.poll() is None for child in children]
        marker_ages = [(time.time_ns() - marker.stat().st_mtime_ns) / 1000000 for marker in markers]
        available = [item["rss_bytes"] for item in observed if item["available"]]
        valid = all(live_before) and all(live_after) and all(age < args.pause_ms for age in marker_ages)
        memory = {"format": "durable-membership-transaction-overlap-memory-v1", "variant": variant,
                  "size": 100000, "processes": processes, "trial": trial, "agents": observed,
                  "sampling_started_mono_ns": memory_started, "sampling_finished_mono_ns": memory_finished,
                  "sampling_window_ns": memory_finished - memory_started, "all_live_before": all(live_before),
                  "all_live_after": all(live_after), "marker_age_after_ms": marker_ages,
                  "all_within_preparation_pause": valid, "rss_agents_available": len(available),
                  "observed_rss_sum_bytes": sum(available) if valid and len(available) == processes else None,
                  "interpretation": "Actual overlapping live snapshot+buffer processes, sequential OS RSS reads within a bounded common preparation window. Shared pages can count in every process; no PSS/unique RAM claim and no sum-of-peaks estimate."}
        memory_log.write(json_line(memory) + "\n")
        memory_log.flush()
        runner.check(valid, "tx-cohort:valid-overlap-memory-window", variant=variant, processes=processes, trial=trial, memory=memory)
        if len(available) != processes:
            runner.note("tx-cohort:rss-unavailable", status="skip", variant=variant, processes=processes, trial=trial,
                        agents=[item for item in observed if not item["available"]])
        records = [child.finish() for child in children]
        statuses = [runner.success(record, "tx-cohort:child-result")["status"] for record in records]
        runner.check(statuses.count("committed") == 1 and statuses.count("conflict") == processes - 1,
                     "tx-cohort:one-root-winner-explicit-conflicts", variant=variant, statuses=statuses)
        for index, (command, status) in enumerate(zip(commands, statuses)):
            if status == "conflict":
                retry = {key: value for key, value in command.items() if key not in ("ready_file", "pause_ms")}
                result = runner.success(runner.call(store, variant, retry, "transaction-cohort", "manual-rebegin",
                    size=100000, processes=processes, trial=trial, agent=index), "tx-cohort:manual-retry")
                runner.check(result["status"] == "committed", "tx-cohort:retry-committed", result=result)
        keys = [command["actions"][1]["key"] for command in commands]
        values = runner.lookup(store, variant, keys, "transaction-cohort", "retained-independent-updates")
        runner.check(values == ["independent-buffer"] * processes, "tx-cohort:all-independent-updates-survive")
        finished = time.perf_counter_ns()
        runner.note("tx-cohort:cohort", variant=variant, size=100000, processes=processes, trial=trial,
                    cohort_started_mono_ns=started, cohort_finished_mono_ns=finished, cohort_wall_ns=finished - started,
                    committed=statuses.count("committed"), explicit_conflicts=statuses.count("conflict"),
                    manual_retries=statuses.count("conflict"), rss_observed_sum_bytes=memory["observed_rss_sum_bytes"],
                    lock_wait_ns=[record["response"]["metrics"]["lock_wait_ns"] for record in records],
                    lock_hold_ns=[record["response"]["metrics"]["lock_hold_ns"] for record in records],
                    child_peak_rss_kib=[record["response"].get("peak_rss_kib") for record in records],
                    interpretation="Includes startup, preparation pause, commits/conflicts, explicit manual rebegin/revalidation, verification, and shutdown; child peaks are individually reported, never summed as concurrent memory.")
        completed = True
    finally:
        if not completed:
            for child in children:
                if child.process.poll() is None:
                    child.process.kill()
            for child in children:
                child.finish()


def summarize(rows):
    groups = {}
    for row in rows:
        key = (row["variant"], row["size"], row["mode"], row["case"], row["pairs"])
        groups.setdefault(key, []).append(row)
    summary = []
    for key, samples in sorted(groups.items()):
        entry = dict(zip(("variant", "size", "mode", "case", "pairs"), key))
        entry["samples"] = len(samples)
        entry["metrics"] = {}
        for field in ("wall_ns", "command_ns", "open_ns", "core_ns", "preparation_ns", "lock_wait_ns", "lock_hold_ns",
                      "nodes_loaded", "nodes_written", "bytes_read", "bytes_written", "writes", "reused_objects",
                      "buffered_pairs", "logical_bytes", "logical_input_bytes", "storage_bytes_delta", "storage_objects_delta"):
            values = sorted(row[field] for row in samples if isinstance(row.get(field), (int, float)))
            if values:
                entry["metrics"][field] = {"n": len(values), "median": statistics.median(values),
                                           "min": values[0], "max": values[-1],
                                           "p95": values[min(len(values) - 1, int(0.95 * (len(values) - 1)))]}
        summary.append(entry)
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for argument in ("binary", "output", "work"):
        parser.add_argument("--" + argument, required=True)
    parser.add_argument("--sizes", default="1000,10000,100000")
    parser.add_argument("--samples", type=int, default=30)
    parser.add_argument("--cohort-trials", type=int, default=3)
    parser.add_argument("--pause-ms", type=int, default=5000)
    parser.add_argument("--processes", default="1,2,4,8")
    parser.add_argument("--variants", default="chunk,disk")
    parser.add_argument("--seed", type=int, default=20261012)
    parser.add_argument("--timeout", type=float, default=180)
    args = parser.parse_args()
    binary, output, work = (Path(value).resolve() for value in (args.binary, args.output, args.work))
    variants = args.variants.split(",")
    processes = [int(value) for value in args.processes.split(",")]
    sizes = [int(value) for value in args.sizes.split(",")]
    if not binary.is_file() or output.exists() or work.exists():
        parser.error("existing binary and new output/work required")
    if set(variants) - {"chunk", "disk"} or set(processes) - {1, 2, 4, 8}:
        parser.error("unknown variant/process count")
    if args.samples < 1 or args.cohort_trials < 0 or not 1000 <= args.pause_ms <= 60000 or any(size < 64 or size > 1000000 for size in sizes):
        parser.error("positive samples, cohort trials >=0, pause1..60s and sizes64..1,000,000 required")
    if args.cohort_trials and 100000 not in sizes:
        parser.error("100000 size required for cohort measurements; use --cohort-trials0 for bounded smoke")
    output.mkdir(parents=True)
    work.mkdir(parents=True)
    env = environment(binary, work, args)
    env["evidence_run_id"] = output.name
    env["child_stdio_encoding"] = "utf-8, explicit on stdin/stdout/stderr independent of native locale"
    env["experiment"] = "buffered transaction sequential/coalesced native-store comparison and overlapping100k snapshot+overlay processes"
    (output / "environment.json").write_text(json.dumps(env, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    runner, rows = Runner(binary, output, work, args.timeout), []
    prefix = hashlib.sha256(str(output).encode("utf-8")).hexdigest()[:12]
    try:
        with (output / "simultaneous-memory.jsonl").open("w", encoding="utf-8", newline="\n") as memory:
            for variant in variants:
                try:
                    stores = buffer_measurements(runner, variant, sizes, args.samples, args.seed, rows)
                    if args.cohort_trials:
                        for count in processes:
                            for trial in range(args.cohort_trials):
                                cohort(runner, memory, stores[100000], variant, count, trial, args, prefix)
                except Exception as error:
                    runner.note("tx-measure:suite-aborted", status="fail", variant=variant,
                                error=type(error).__name__ + ": " + str(error))
                    print("FAILED", variant, error, flush=True)
        (output / "summary.json").write_text(json.dumps(summarize(rows), indent=2, sort_keys=True) + "\n", encoding="utf-8")
        status = {"complete": runner.failures == 0, "full_harness_complete": False, "passes": runner.passes,
                  "failures": runner.failures, "skips": runner.skips, "observations": runner.observations,
                  "variants": variants, "sizes": sizes, "samples": args.samples, "cohort_trials": args.cohort_trials,
                  "evidence_run_id": output.name, "summary_groups": len(summarize(rows))}
        (output / "run-status.json").write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json_line(status), flush=True)
        return int(runner.failures != 0)
    finally:
        runner.close()


if __name__ == "__main__":
    raise SystemExit(main())
