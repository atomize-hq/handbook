#!/usr/bin/env python3
"""Summarize retained samples. No extrapolation or physical-IO interpretation."""

import argparse
import csv
import gzip
import json
import math
from pathlib import Path
import statistics


GROUP_FIELDS = ("variant", "suite", "case", "size", "siblings", "guard", "instrument", "processes", "mode")


def read_lines(path):
    if path.exists():
        source = path.open(encoding="utf-8")
    else:
        source = gzip.open(str(path) + ".gz", "rt", encoding="utf-8")
    with source:
        return [json.loads(line) for line in source if line.strip()]


def number(value):
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def stats(values):
    values = sorted(value for value in values if number(value))
    if not values:
        return {"n": 0, "available": False}
    return {"n": len(values), "available": True, "min": values[0],
            "median": statistics.median(values),
            "p95": values[max(0, math.ceil(0.95 * len(values)) - 1)], "max": values[-1]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("evidence", nargs="+", help="Directories containing samples.jsonl")
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    output = Path(args.output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    all_groups, platforms, instrumentation, concurrency = [], [], [], []
    checks = []
    storage = []
    for argument in args.evidence:
        directory = Path(argument).resolve()
        env = json.loads((directory / "environment.json").read_text(encoding="utf-8"))
        run_status = json.loads((directory / "run-status.json").read_text(encoding="utf-8"))
        label = directory.name
        platforms.append({"label": label, "environment": env, "run_status": run_status})
        samples = read_lines(directory / "samples.jsonl")
        evidence_checks = read_lines(directory / "checks.jsonl")
        checks.extend({"platform": label, **check} for check in evidence_checks if check["status"] != "pass")
        storage.extend({"platform": label, **check} for check in evidence_checks if check["name"].startswith("growth-storage"))
        groups = {}
        for sample in samples:
            tags = sample["tags"]
            dimensions = {"variant": sample["variant"], **tags}
            key = tuple(dimensions.get(field) for field in GROUP_FIELDS)
            groups.setdefault(key, []).append(sample)
        for key, members in sorted(groups.items(), key=lambda item: str(item[0])):
            successful = [member for member in members if (member.get("response") or {}).get("ok") is True]
            entry = {"platform": label, **dict(zip(GROUP_FIELDS, key)), "samples": len(members),
                     "successful_responses": len(successful), "structured_refusals": sum((member.get("response") or {}).get("ok") is False for member in members),
                     "abrupt_exits": sum(member.get("response") is None for member in members)}
            entry["wall_ns"] = stats([member.get("wall_ns") for member in members])
            entry["elapsed_ns"] = stats([member["response"].get("elapsed_ns") for member in successful])
            entry["open_ns"] = stats([(member["response"].get("result") or {}).get("open_ns") for member in successful])
            entry["core_ns"] = stats([(member["response"].get("result") or {}).get("core_ns") for member in successful])
            entry["amortized_core_ns_per_key"] = stats([
                (member["response"]["result"]["core_ns"] / (max(1, len(member["command"].get("keys", []))) * member["command"].get("repeat", 1)))
                for member in successful if number((member["response"].get("result") or {}).get("core_ns"))
                and member["command"].get("op") in ("lookup", "hold")
            ])
            entry["peak_rss_kib"] = stats([member["response"].get("peak_rss_kib") for member in successful])
            metric_names = sorted({name for member in successful for name in member["response"].get("metrics", {})})
            entry["metrics"] = {name: stats([member["response"].get("metrics", {}).get(name) for member in successful]) for name in metric_names}
            all_groups.append(entry)
        pairs = {}
        cohorts = {}
        for sample in samples:
            tags = sample["tags"]
            if tags["case"] == "instrumentation" and (sample.get("response") or {}).get("ok") is True:
                pairs.setdefault((sample["variant"], tags["size"], tags["sample"]), {})[tags["instrument"]] = sample
            if tags["suite"] == "concurrency" and tags["case"] == "independent":
                cohorts.setdefault((sample["variant"], tags["processes"], tags["trial"]), []).append(sample)
        paired_groups = {}
        for key, pair in pairs.items():
            if set(pair) == {True, False}:
                enabled = pair[True]["response"].get("elapsed_ns")
                disabled = pair[False]["response"].get("elapsed_ns")
                if number(enabled) and number(disabled) and disabled > 0:
                    paired_groups.setdefault(key[:2], []).append((enabled - disabled, enabled / disabled))
        for (variant, size), values in sorted(paired_groups.items()):
            instrumentation.append({"platform": label, "variant": variant, "size": size,
                                    "paired_elapsed_delta_ns": stats([value[0] for value in values]),
                                    "paired_elapsed_ratio": stats([value[1] for value in values]),
                                    "interpretation": "Paired alternating-order fresh processes; cache/noise not eliminated. Ratio <1 is possible noise, not negative instrumentation work."})
        for (variant, processes, trial), members in sorted(cohorts.items()):
            elapsed = max(member["finished_mono_ns"] for member in members) - min(member["started_mono_ns"] for member in members)
            concurrency.append({"platform": label, "variant": variant, "processes": processes,
                                "trial": trial, "cohort_wall_ns": elapsed,
                                "committed": sum((member.get("response") or {}).get("result", {}).get("status") == "committed" for member in members),
                                "interpretation": "Cohort includes simultaneous preparation pause of 300ms, process startup, publication and shutdown; not isolated commit throughput."})
    summary = {
        "format": "durable-membership-experiment-summary-v1", "platforms": platforms,
        "groups": all_groups, "instrumentation": instrumentation, "concurrency_cohorts": concurrency,
        "storage": storage, "nonpassing_checks": checks,
        "interpretation": [
            "p95 uses nearest-rank observed samples; groups with fewer than 20 samples are descriptive only.",
            "Fresh process means reopen with uncontrolled OS cache; initialization and prior samples may warm cache.",
            "elapsed_ns is child command time; wall_ns also includes process launch/shutdown and collector overhead.",
            "open_ns/core_ns are child-reported phases; amortized core divides aggregate lookup time by actual key count and repeat, not independent tail samples.",
            "Counters describe instrumented helper events and logical bytes, not physical IO or OS syscalls.",
            "Peak RSS is child-reported process peak; allocation counts unavailable. Missing metrics stay unavailable.",
            "All immutable proposals and old objects are retained. Logical file bytes do not measure physical allocation.",
        ],
    }
    (output / "summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    fields = ["platform", *GROUP_FIELDS, "samples", "successful_responses", "wall_median_ms", "wall_p95_ms",
              "command_median_ms", "command_p95_ms", "open_median_ms", "core_median_ms", "peak_rss_max_kib",
              "bytes_read_median", "bytes_written_median", "nodes_loaded_median", "nodes_written_median",
              "listed_entries_median", "lock_wait_median_ms", "lock_hold_median_ms", "retries_median"]
    with (output / "summary.csv").open("w", encoding="utf-8", newline="") as target:
        writer = csv.DictWriter(target, fieldnames=fields)
        writer.writeheader()
        for group in all_groups:
            row = {field: group.get(field) for field in fields if field in group}
            for output_name, source_name, stat_name in (
                ("wall_median_ms", "wall_ns", "median"), ("wall_p95_ms", "wall_ns", "p95"),
                ("command_median_ms", "elapsed_ns", "median"), ("command_p95_ms", "elapsed_ns", "p95"),
                ("open_median_ms", "open_ns", "median"), ("core_median_ms", "core_ns", "median"),
            ):
                row[output_name] = scaled(group[source_name].get(stat_name), 1000000)
            row["peak_rss_max_kib"] = group["peak_rss_kib"].get("max")
            for name in ("bytes_read", "bytes_written", "nodes_loaded", "nodes_written", "listed_entries", "retries"):
                row[name + "_median"] = group["metrics"].get(name, {}).get("median")
            for name in ("lock_wait", "lock_hold"):
                row[name + "_median_ms"] = scaled(group["metrics"].get(name + "_ns", {}).get("median"), 1000000)
            writer.writerow(row)
    lines = ["# Observed prototype measurements", "", "Fresh processes use an uncontrolled OS cache. These are isolated prototype results, not production capacity or power-loss qualification.", "",
             "| Platform | Variant | Case | History size | n | Command median ms | Command p95 ms | Process median ms | Max RSS KiB |",
             "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for group in all_groups:
        if group["suite"] == "measure" and group["case"] != "sibling-guard":
            lines.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
                group["platform"], group["variant"], group["case"] + ("-on" if group["instrument"] else "-off")
                if group["case"] == "instrumentation" else group["case"], group["size"], group["samples"],
                display_ms(group["elapsed_ns"].get("median")), display_ms(group["elapsed_ns"].get("p95")),
                display_ms(group["wall_ns"].get("median")), group["peak_rss_kib"].get("max", "unavailable")))
    lines += ["", "Instrumentation overhead (paired command-time enabled/disabled ratios; alternating order):", "",
              "| Platform | Variant | History size | Pairs | Median ratio | Median delta ms |",
              "| --- | --- | ---: | ---: | ---: | ---: |"]
    for group in instrumentation:
        lines.append("| %s | %s | %s | %s | %.4f | %s |" % (group["platform"], group["variant"], group["size"],
                     group["paired_elapsed_ratio"]["n"], group["paired_elapsed_ratio"]["median"],
                     display_ms(group["paired_elapsed_delta_ns"]["median"])))
    lines += ["", "Sibling enumeration stress (smallest history only):", "",
              "| Platform | Variant | History size | Siblings | Guard | n | Median ms | p95 ms | Listed entries median |",
              "| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | ---: |"]
    for group in all_groups:
        if group["case"] == "sibling-guard":
            lines.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
                group["platform"], group["variant"], group["size"], group["siblings"], group["guard"], group["samples"],
                display_ms(group["elapsed_ns"].get("median")), display_ms(group["elapsed_ns"].get("p95")),
                group["metrics"].get("listed_entries", {}).get("median", "unavailable")))
    lines += ["", "| Platform | Passes | Failures | Skips |", "| --- | ---: | ---: | ---: |"]
    for entry in platforms:
        status = entry["run_status"]
        lines.append("| %s | %s | %s | %s |" % (entry["label"], status["passes"], status["failures"], status["skips"]))
    lines += ["", "Unavailable metrics and fault/cache limits remain explicit in summary.json and environment.json. Raw responses and each assertion are in the input evidence directories.", ""]
    (output / "summary.md").write_text("\n".join(lines), encoding="utf-8")
    print(json.dumps({"groups": len(all_groups), "platforms": len(platforms), "nonpassing_checks": len(checks), "output": str(output)}))


def scaled(value, divisor):
    return value / divisor if number(value) else None


def display_ms(value):
    return "%.4f" % (value / 1000000) if number(value) else "unavailable"


if __name__ == "__main__":
    main()
