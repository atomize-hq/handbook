#!/usr/bin/env python3
"""Native transaction/selected-regression evidence; Python stdlib only.

All stores and output directories must be new. --baseline-selected records the
two selected defects in predecessor binaries; those observations are not final
transaction evidence. The final mode exercises both adapters and reuses the
existing semantic, fault, corruption, and native held-ROOT suites unchanged.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import threading
import time

from run import Child, Runner as BaseRunner, concurrency, environment, faults, json_line, request, validation


class Utf8Child(Child):
    """Reuse collection/assertion logic; make native CLI pipes explicitly UTF-8.

    Windows locale text defaults cannot carry arbitrary admitted UTF-8 values.
    Python's documented encoding parameter applies to all three redirected pipes:
    https://docs.python.org/3.11/library/subprocess.html#subprocess.Popen
    """
    def __init__(self, runner, store, variant, command, tags, injection=None):
        self.runner, self.store, self.variant = runner, store, variant
        self.command, self.tags, self.injection = command, tags, injection or {}
        env = os.environ.copy()
        env.pop("PROTO_FAULT", None)
        env.pop("PROTO_FAIL", None)
        env.update(self.injection)
        self.started = time.perf_counter_ns()
        self.process = subprocess.Popen([str(runner.binary), str(store), variant],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
            text=True, encoding="utf-8", env=env)
        self.process.stdin.write(json_line(command) + "\n")
        self.process.stdin.close()
        self.process.stdin = None
        self.record = None
        self.thread = threading.Thread(target=self.collect, daemon=True)
        self.thread.start()


class Runner(BaseRunner):
    def launch(self, store, variant, command, suite, case, injection=None, **tags):
        return Utf8Child(self, store, variant, command, {"suite": suite, "case": case, **tags}, injection)


def tx(actions, **options):
    return {"op": "tx", "actions": actions, **options}


def put(key, value):
    return {"op": "put", "key": key, "value": value}


def get(key):
    return {"op": "get", "key": key}


def selected(runner, variant, baseline=False):
    store = runner.new_store(variant, "selected-domain", 64)
    pairs = [["u:%09d" % index, "x" * 1024] for index in range(64)]
    before = (store / "ROOT").read_bytes()
    record = runner.call(store, variant, {"op": "put", "pairs": pairs}, "selected", "admitted-64x1024")
    if baseline and variant == "chunk":
        response = record["response"] or {}
        runner.check(response.get("ok") is False and "encoded node byte limit exceeded" in response.get("error", ""),
                     "baseline:selected-domain-chunk-defect-reproduced", response=response)
        runner.check((store / "ROOT").read_bytes() == before, "baseline:failure-retains-root")
        values = runner.lookup(store, variant, [key for key, _ in pairs], "selected", "baseline-domain-retained")
        runner.check(values == ["x" * 96] * 64, "baseline:prior-values-retained", variant=variant)
    else:
        runner.success(record, "selected:domain-accepted")
        values = runner.lookup(store, variant, [key for key, _ in pairs], "selected", "domain-reopen")
        runner.check(values == [value for _, value in pairs], "selected:domain-exact-reopen", variant=variant)

    store = runner.new_store(variant, "selected-wrong-shard", 64)
    canonical = next(path for path in sorted((store / "objects").glob("*/*")) if len(path.name) == 64)
    good = runner.call(store, variant, {"op": "audit"}, "selected", "canonical-audit")
    runner.success(good, "selected:canonical-audit")
    wrong_shard = "00" if canonical.parent.name != "00" else "01"
    misplaced = store / "objects" / wrong_shard / canonical.name
    misplaced.parent.mkdir(exist_ok=True)
    misplaced.write_bytes(b"corrupt duplicate at wrong shard, canonical object remains valid")
    record = runner.call(store, variant, {"op": "audit"}, "selected", "wrong-shard-corrupt-duplicate")
    if baseline:
        runner.success(record, "baseline:wrong-shard-audit-defect-reproduced")
        runner.note("baseline:wrong-shard-blind-spot", variant=variant,
                    canonical=str(canonical.relative_to(store)), misplaced=str(misplaced.relative_to(store)),
                    severity="P3: selected canonical membership remains valid; writers do not create this layout")
    else:
        runner.check((record["response"] or {}).get("ok") is False,
                     "selected:wrong-shard-corrupt-duplicate-rejected", variant=variant, response=record["response"])


def transaction_api(runner, variant):
    store = runner.new_store(variant, "transaction-api", 64)
    before = (store / "ROOT").read_bytes()
    result = runner.success(runner.call(store, variant,
        tx([get("u:000000000"), put("u:000000000", "raw:true"), get("u:000000000"),
            put("u:second", "second-value"), get("u:second")]), "transaction", "read-own-writes"), "tx:read-own-writes")
    runner.check(result["status"] == "committed" and result["values"] == ["x" * 96, "raw:true", "second-value"],
                 "tx:raw-point-get-and-read-own-writes", variant=variant, result=result)
    runner.check((store / "ROOT").read_bytes() != before, "tx:published-one-root")
    check = runner.success(runner.call(store, variant, tx([get("u:000000000"), get("u:second")], read_only=True),
                                      "transaction", "reopen-raw-values"), "tx:reopen")
    runner.check(check["status"] == "snapshot" and check["values"] == ["raw:true", "second-value"],
                 "tx:reopened-raw-values", result=check)

    for ending in ("cancel", "drop"):
        before = (store / "ROOT").read_bytes()
        record = runner.call(store, variant, tx([put("u:cancelled", ending), get("u:cancelled")], end=ending),
                             "transaction", ending)
        result = runner.success(record, "tx:" + ending)
        runner.check(result["status"] == ("cancelled" if ending == "cancel" else "dropped")
                     and result["values"] == [ending], "tx:" + ending + "-overlay-visible", result=result)
        runner.check((store / "ROOT").read_bytes() == before, "tx:" + ending + "-root-unchanged")
        metrics = record["response"]["metrics"]
        runner.check(metrics["writes"] == 0 and metrics["nodes_written"] == 0 and metrics["lock_hold_ns"] == 0,
                     "tx:" + ending + "-no-persistence-no-lock", metrics=metrics)
    for actions, case in (([{"op": "cancel"}, get("u:000000000")], "closed-get"),
                          ([{"op": "cancel"}, put("u:bad", "closed")], "closed-put"),
                          ([put("u:bad", "readonly")], "readonly-put"),
                          ([put("r:forbidden", "reserved namespace")], "namespace")):
        before = (store / "ROOT").read_bytes()
        record = runner.call(store, variant, tx(actions, read_only=(case == "readonly-put")), "transaction", case)
        runner.check((record["response"] or {}).get("ok") is False, "tx:" + case + "-refused", response=record["response"])
        runner.check((store / "ROOT").read_bytes() == before, "tx:" + case + "-atomic-refusal")

    result = runner.success(runner.call(store, variant, tx([
        {"op": "put_if", "key": "u:conditional", "value": "one", "expected": None},
        {"op": "put_if", "key": "u:conditional", "value": "two", "expected": "one"}, get("u:conditional")]),
        "transaction", "conditional-read-own-write"), "tx:conditional")
    runner.check(result["status"] == "committed" and result["values"] == ["two"], "tx:conditional-uses-overlay", result=result)
    before = (store / "ROOT").read_bytes()
    record = runner.call(store, variant, tx([put("u:must-not-publish", "value"),
        {"op": "put_if", "key": "u:conditional", "value": "three", "expected": "wrong"}]),
        "transaction", "conditional-failure")
    runner.check((record["response"] or {}).get("ok") is False, "tx:conditional-failure-explicit", response=record["response"])
    runner.check((store / "ROOT").read_bytes() == before, "tx:conditional-failure-all-or-nothing")


def domain(runner, variant):
    store = runner.new_store(variant, "max-domain", 64)
    for case, pairs in (
        ("escaped-control-maximum", [["u:%09d" % index + "\x00" * (1024 - 11), "\x00" * 1024] for index in range(64)]),
        ("escaped-key-high-byte-value", [["u:%09d" % index + "\x01" * (1024 - 11), "\u00ff" * 512] for index in range(64)]),
        ("maximal-escaped-batch-256", [["u:%09d" % index + "\x02" * (1024 - 11), "\x02" * 1024] for index in range(256)]),
        ("batch-256-boundary", [["u:batch:%04d" % index, "batch-boundary-value"] for index in range(256)]),
    ):
        record = runner.call(store, variant, {"op": "put", "pairs": pairs}, "domain", case)
        runner.success(record, "domain:" + case)
        result = runner.success(runner.call(store, variant, tx([get(key) for key, _ in pairs], read_only=True),
                                           "domain", case + "-reopen"), "domain:reopen")
        runner.check(result["values"] == [value for _, value in pairs], "domain:" + case + "-exact-values", variant=variant)
    for case, pairs in (
        ("oversize-key", [["u:atomic-first", "valid"], ["u:" + "k" * 1023, "valid"]]),
        ("oversize-value", [["u:atomic-first", "valid"], ["u:oversize", "v" * 1025]]),
        ("oversize-utf8-value", [["u:atomic-first", "valid"], ["u:oversize", "\u00ff" * 513]]),
        ("batch-257", [["u:over:%04d" % index, "valid"] for index in range(257)]),
    ):
        before = (store / "ROOT").read_bytes()
        record = runner.call(store, variant, {"op": "put", "pairs": pairs}, "domain", case)
        runner.check((record["response"] or {}).get("ok") is False, "domain:" + case + "-explicit-rejection", response=record["response"])
        runner.check((store / "ROOT").read_bytes() == before, "domain:" + case + "-atomic-rejection")


def snapshots_and_conflicts(runner, variant):
    store = runner.new_store(variant, "transaction-snapshot", 64)
    marker = store / "reader-prepared"
    reader = runner.launch(store, variant, tx([], read_only=True, after_keys=["u:000000000", "u:new"],
        ready_file=str(marker), pause_ms=1800), "transaction", "stable-reader-snapshot")
    runner.wait_ready(reader, marker, "tx:reader")
    writer = runner.call(store, variant, tx([put("u:000000000", "changed"), put("u:new", "new-value")]),
                         "transaction", "competing-publication")
    runner.success(writer, "tx:competing-publication")
    result = runner.success(reader.finish(), "tx:stable-reader")
    runner.check(result["status"] == "snapshot" and result["values"] == ["x" * 96, None],
                 "tx:actual-get-after-publication-retains-snapshot", variant=variant, result=result)
    runner.check(runner.lookup(store, variant, ["u:000000000", "u:new"], "transaction", "new-reader") == ["changed", "new-value"],
                 "tx:new-reader-whole-new-state")
    runner.note("tx:no-reclamation-reader-boundary", variant=variant,
                interpretation="Point reads executed after competing process commit against retained transaction snapshot; all immutable objects/old roots remain; no GC or mmap claim.")

    marker = store / "writer-prepared"
    writer = runner.launch(store, variant, tx([get("u:000000000"), put("u:atomic-a", "A"), put("u:atomic-b", "B")],
        ready_file=str(marker), pause_ms=1800), "transaction", "outside-lock-buffer")
    runner.wait_ready(writer, marker, "tx:buffer")
    before = runner.lookup(store, variant, ["u:atomic-a", "u:atomic-b"], "transaction", "before-atomic-publication")
    runner.check(before == [None, None], "tx:buffer-not-selected")
    independent = runner.call(store, variant, tx([put("u:intervening", "survives")]), "transaction", "outside-lock-independent")
    runner.success(independent, "tx:independent-during-agent-work")
    runner.check(not (store / "writer-prepared").is_symlink(), "tx:ordinary-marker")
    record = writer.finish()
    result = runner.success(record, "tx:stale-preparation-result")
    runner.check(result["status"] == "conflict", "tx:stale-root-explicit-conflict", result=result)
    runner.check(record["response"]["metrics"]["nodes_written"] == 0, "tx:conflict-no-object-install", metrics=record["response"]["metrics"])
    runner.check(runner.lookup(store, variant, ["u:atomic-a", "u:atomic-b"], "transaction", "conflict-membership") == [None, None],
                 "tx:conflicting-buffer-not-partially-selected")
    retry = runner.call(store, variant, tx([get("u:000000000"), put("u:atomic-a", "A"), put("u:atomic-b", "B")]),
                        "transaction", "manual-rebegin-and-revalidate")
    runner.check(runner.success(retry, "tx:manual-retry")["status"] == "committed", "tx:manual-retry-committed")
    runner.check(runner.lookup(store, variant, ["u:atomic-a", "u:atomic-b", "u:intervening"], "transaction", "atomic-reopen")
                 == ["A", "B", "survives"], "tx:atomic-new-state-and-independent-survives")

    for count in (1, 2, 4, 8):
        store = runner.new_store(variant, "tx-cohort-%d" % count, 1000)
        children, commands, markers = [], [], []
        for index in range(count):
            marker = store / ("agent-%d.prepared" % index)
            command = tx([put("u:agent:%d:a" % index, "A"), put("u:agent:%d:b" % index, "B")],
                         ready_file=str(marker), pause_ms=2500)
            commands.append(command)
            markers.append(marker)
            children.append(runner.launch(store, variant, command, "transaction", "explicit-conflict-cohort", processes=count, agent=index))
        for child, marker in zip(children, markers):
            runner.wait_ready(child, marker, "tx:cohort")
        records = [child.finish() for child in children]
        statuses = [runner.success(record, "tx:cohort-result")["status"] for record in records]
        runner.check(statuses.count("committed") == 1 and statuses.count("conflict") == count - 1,
                     "tx:cohort-one-winner-explicit-conflicts", variant=variant, processes=count, statuses=statuses)
        for index, (command, status) in enumerate(zip(commands, statuses)):
            if status == "conflict":
                retry_command = {key: value for key, value in command.items() if key not in ("pause_ms", "ready_file")}
                retried = runner.success(runner.call(store, variant, retry_command, "transaction", "manual-cohort-retry", processes=count, agent=index),
                                         "tx:cohort-retry")
                runner.check(retried["status"] == "committed", "tx:cohort-manual-rebegin-committed", result=retried)
        keys = ["u:agent:%d:%s" % (index, suffix) for index in range(count) for suffix in ("a", "b")]
        runner.check(runner.lookup(store, variant, keys, "transaction", "cohort-reopen") == [value for _ in range(count) for value in ("A", "B")],
                     "tx:cohort-no-lost-or-partial-updates", variant=variant, processes=count)


def semantic_cohorts(runner, variant):
    # Existing concurrency() checks the same semantic cases at four processes;
    # this adds every required cohort width without changing that baseline suite.
    for count in (1, 2, 4, 8):
        for case in ("matching-original", "conflicting-original", "same-item"):
            store = runner.new_store(variant, "%s-%s" % (case, count), 64)
            commands = [request("shared-request" if case != "same-item" else "request-%s" % index,
                item="shared-item", content="content-%s" % index if case == "conflicting-original" else "content",
                pause_ms=400) for index in range(count)]
            children = [runner.launch(store, variant, command, "transaction-semantics", case, processes=count, agent=index)
                        for index, command in enumerate(commands)]
            records = [child.finish() for child in children]
            statuses = [runner.status(record) for record in records]
            expected_other = "replay" if case == "matching-original" else "refused"
            runner.check(statuses.count("committed") == 1 and statuses.count(expected_other) == count - 1,
                         "tx-semantics:one-retained-original", variant=variant, processes=count, case=case, statuses=statuses)
            winner = statuses.index("committed")
            retained = runner.membership(store, variant, commands[winner]["request"], "shared-item", True,
                                         "tx-semantics:winner-binding")
            original = {key: commands[winner][key] for key in ("request", "item", "content", "proof")}
            runner.check(retained[0]["original"] == original and retained[0]["state"] == "terminal",
                         "tx-semantics:exact-original-and-terminal", variant=variant, case=case, processes=count, retained=retained[0])
            if case == "same-item":
                values = runner.lookup(store, variant, ["r:" + command["request"] for command in commands],
                                       "transaction-semantics", "competing-request-membership")
                runner.check(sum(value is not None for value in values) == 1, "tx-semantics:one-item-one-request",
                             variant=variant, processes=count, values=values)


def transaction_faults(runner, variant):
    prepublication = {"object_write", "object_sync", "object_install", "objects_dir_sync", "root_stage", "root_sync", "short_write"}
    boundaries = ("object_write", "object_sync", "object_install", "objects_dir_sync", "root_stage", "root_sync",
                  "root_replace", "publication_sync", "before_reply")
    for mode in ("PROTO_FAIL", "PROTO_FAULT"):
        for boundary in boundaries + (("short_write",) if mode == "PROTO_FAIL" else ()):
            store = runner.new_store(variant, "tx-%s-%s" % (mode, boundary), 64)
            runner.success(runner.call(store, variant, tx([put("u:acknowledged", "keep-acknowledged")]),
                                      "transaction-faults", "acknowledged-base"), "tx-fault:base")
            before = (store / "ROOT").read_bytes()
            record = runner.call(store, variant, tx([get("u:acknowledged"), put("u:fault-a", "one"), put("u:fault-b", "two")]),
                                 "transaction-faults", boundary, {mode: boundary}, mode=mode)
            if mode == "PROTO_FAULT":
                runner.check(record["returncode"] == 86 and record["response"] is None,
                             "tx-fault:exit-checkpoint-reached", variant=variant, boundary=boundary, record=record)
            else:
                response = record["response"] or {}
                error_marker = "short write" if boundary == "short_write" else boundary
                runner.check(response.get("ok") is False and error_marker in response.get("error", ""),
                             "tx-fault:io-checkpoint-reached", variant=variant, boundary=boundary, response=response)
                expected = "not_published" if boundary in prepublication else "indeterminate"
                runner.check(response.get("publication") == expected,
                             "tx-fault:known-or-indeterminate-classification", variant=variant, boundary=boundary,
                             expected=expected, response=response)
            result = runner.success(runner.call(store, variant, tx([get("u:acknowledged"), get("u:fault-a"), get("u:fault-b")],
                read_only=True), "transaction-faults", "recover-selected-state", boundary=boundary, mode=mode), "tx-fault:reopen")
            values = result["values"]
            runner.check(values[0] == "keep-acknowledged" and values[1:] in ([None, None], ["one", "two"]),
                         "tx-fault:ack-survives-whole-old-or-new", variant=variant, boundary=boundary, values=values)
            if boundary in prepublication:
                runner.check(values[1:] == [None, None] and (store / "ROOT").read_bytes() == before,
                             "tx-fault:prepublication-old-selected-state", variant=variant, boundary=boundary)
            if boundary == "before_reply":
                runner.check(values[1:] == ["one", "two"], "tx-fault:lost-reply-after-whole-new-state", variant=variant)
            # Generic tx has no request identity and cannot promise exact replay.
            # Rebegin explicitly observes selected values before conditional retry.
            retry = runner.success(runner.call(store, variant, tx([
                {"op": "put_if", "key": "u:fault-a", "value": "one", "expected": values[1]},
                {"op": "put_if", "key": "u:fault-b", "value": "two", "expected": values[2]}]),
                "transaction-faults", "explicit-rebegin-revalidate-recover", boundary=boundary, mode=mode), "tx-fault:conditional-recovery")
            runner.check(retry["status"] == "committed", "tx-fault:recovery-committed", result=retry)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for argument in ("binary", "output", "work"):
        parser.add_argument("--" + argument, required=True)
    parser.add_argument("--baseline-selected", action="store_true")
    parser.add_argument("--sections", default="selected,api,domain,snapshots,semantics,tx-faults,concurrency,faults,validation")
    parser.add_argument("--variants", default="chunk,disk")
    parser.add_argument("--seed", type=int, default=20261012)
    parser.add_argument("--timeout", type=float, default=180)
    args = parser.parse_args()
    binary, output, work = (Path(value).resolve() for value in (args.binary, args.output, args.work))
    suites = {"selected": selected, "api": transaction_api, "domain": domain, "snapshots": snapshots_and_conflicts,
              "semantics": semantic_cohorts, "tx-faults": transaction_faults,
              "concurrency": concurrency, "faults": faults, "validation": validation}
    sections = ["selected"] if args.baseline_selected else args.sections.split(",")
    variants = args.variants.split(",")
    if not binary.is_file() or output.exists() or work.exists():
        parser.error("existing binary and new output/work required")
    if set(sections) - suites.keys() or set(variants) - {"chunk", "disk"}:
        parser.error("unknown section/variant")
    output.mkdir(parents=True)
    work.mkdir(parents=True)
    env = environment(binary, work, args)
    env["experiment"] = "predecessor selected findings reproduction" if args.baseline_selected else "final-source shared transaction interface native checks"
    env["evidence_run_id"] = output.name
    env["child_stdio_encoding"] = "utf-8, explicit on stdin/stdout/stderr independent of native locale"
    (output / "environment.json").write_text(json.dumps(env, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    runner = Runner(binary, output, work, args.timeout)
    try:
        for variant in variants:
            for section in sections:
                print(section, variant, flush=True)
                try:
                    if section == "selected":
                        selected(runner, variant, args.baseline_selected)
                    else:
                        suites[section](runner, variant)
                except Exception as error:
                    runner.note("transaction:suite-aborted", status="fail", variant=variant, suite=section,
                                error=type(error).__name__ + ": " + str(error))
                    print("FAILED", variant, section, error, flush=True)
        status = {"complete": runner.failures == 0, "full_harness_complete": False, "passes": runner.passes,
                  "failures": runner.failures, "skips": runner.skips, "observations": runner.observations,
                  "sections": sections, "variants": variants, "baseline_selected": args.baseline_selected,
                  "evidence_run_id": output.name}
        (output / "run-status.json").write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json_line(status), flush=True)
        return int(runner.failures != 0)
    finally:
        runner.close()


if __name__ == "__main__":
    raise SystemExit(main())
