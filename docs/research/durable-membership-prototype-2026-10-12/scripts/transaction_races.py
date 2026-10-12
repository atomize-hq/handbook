#!/usr/bin/env python3
"""Native replay repair dispatch and prepared authority/reply race evidence.

--baseline records the reviewed candidate's read_only dispatch bypass using
frozen candidate binaries. Final mode requires the repair checkpoint even when
read_only is supplied to a semantic run command. All evidence/work IDs are new;
process exit/IO injection retains OS caches and is not hardware power loss.
"""
import argparse
import json
import os
from pathlib import Path

from run import environment, json_line, request
from transaction_tests import Runner


def replay_dispatch(runner, variant, baseline):
    store = runner.new_store(variant, "replay-readonly-dispatch", 64)
    original = request("replay-original", content="retained original survives revocation")
    result = runner.success(runner.call(store, variant, original, "replay-dispatch", "committed-original"), "replay:original")
    runner.check(result["status"] == "committed", "replay:original-committed")
    runner.success(runner.call(store, variant, {"op": "authority", "epoch": 2, "scope": False},
                              "replay-dispatch", "revoke"), "replay:revoke")
    before = (store / "ROOT").read_bytes()
    # Positive checkpoint control: the ordinary semantic replay already repairs.
    control = runner.call(store, variant, original, "replay-dispatch", "ordinary-repair-checkpoint", {"PROTO_FAIL": "replay_sync"})
    response = control["response"] or {}
    runner.check(response.get("ok") is False and "replay_sync" in response.get("error", "")
                 and response.get("publication") == "indeterminate"
                 and response.get("metrics", {}).get("replay_repairs") == 1,
                 "replay:ordinary-repair-checkpoint-reached", variant=variant, response=response)
    flagged = {**original, "read_only": True}
    stopped = runner.call(store, variant, flagged, "replay-dispatch", "readonly-repair-checkpoint", {"PROTO_FAIL": "replay_sync"})
    response = stopped["response"] or {}
    metrics = response.get("metrics") or {}
    if baseline:
        result = runner.success(stopped, "baseline:readonly-replay-bypass")
        runner.check(result["status"] == "replay" and metrics.get("replay_repairs") == 0
                     and metrics.get("syncs") == 0 and metrics.get("lock_hold_ns") == 0,
                     "baseline:P2-semantic-readonly-bypasses-repair", variant=variant, response=response)
        runner.note("baseline:reviewed-P2", variant=variant,
                    interpretation="Exact terminal outcome returned despite injected replay_sync failure; read_only semantic dispatch bypasses lock/publication repair. Selected membership remains retained.")
    else:
        runner.check(response.get("ok") is False and "replay_sync" in response.get("error", "")
                     and response.get("publication") == "indeterminate" and metrics.get("replay_repairs") == 1
                     and metrics.get("syncs", 0) >= 2 and metrics.get("lock_hold_ns", 0) > 0,
                     "replay:readonly-semantic-cannot-bypass-repair", variant=variant, response=response)
        replay = runner.call(store, variant, flagged, "replay-dispatch", "readonly-exact-successful-replay")
        result = runner.success(replay, "replay:flagged-success")
        metrics = replay["response"]["metrics"]
        runner.check(result["status"] == "replay" and result["outcome"] == "outcome:replay-original:retained original survives revocation"
                     and metrics["replay_repairs"] == 1 and metrics["syncs"] >= 2 and metrics["lock_hold_ns"] > 0,
                     "replay:exact-outcome-after-required-boundary", variant=variant, response=replay["response"])
    runner.check((store / "ROOT").read_bytes() == before, "replay:repair-does-not-create-new-membership-root", variant=variant)
    runner.membership(store, variant, original["request"], original["item"], True, "replay:retained-original")


def authority_races(runner, variant, resumed, pause_ms):
    for count in (1, 2, 4, 8):
        label = "resumed" if resumed else "fresh"
        store = runner.new_store(variant, "authority-%s-%s" % (label, count), 64)
        originals = [request("%s-%s" % (label, index), content="prepared exact original") for index in range(count)]
        if resumed:
            for original in originals:
                for stage, expected in (("reserve", "reserved"), ("accept", "accepted")):
                    record = runner.call(store, variant, {**original, "stage": stage}, "authority-races", "setup-" + stage,
                                         processes=count, request=original["request"])
                    runner.check(runner.success(record, "authority:setup")["status"] == expected, "authority:setup-distinct-state")
        markers = [store / ("agent-%s.prepared" % index) for index in range(count)]
        children = [runner.launch(store, variant, {**original, "ready_file": str(marker), "pause_ms": pause_ms},
                    "authority-races", label, processes=count, agent=index)
                    for index, (original, marker) in enumerate(zip(originals, markers))]
        for child, marker in zip(children, markers):
            runner.wait_ready(child, marker, "authority:actual-prepared")
        runner.check(all(child.process.poll() is None for child in children), "authority:all-prepared-live", variant=variant, processes=count)
        revoked = runner.call(store, variant, {"op": "authority", "epoch": 2, "scope": False},
                              "authority-races", "revoke-during-preparation", processes=count, resumed=resumed)
        runner.success(revoked, "authority:revocation-committed")
        runner.check(all(child.process.poll() is None for child in children), "authority:revocation-outside-agent-lock", variant=variant, processes=count)
        for index, child in enumerate(children):
            record = child.finish()
            runner.check(runner.status(record) == "refused", "authority:stale-original-refused", variant=variant,
                         processes=count, resumed=resumed, response=record["response"])
            metrics = record["response"]["metrics"]
            runner.check(metrics["buffered_pairs"] >= 2 and metrics["logical_bytes"] > 0
                         and metrics["preparation_ns"] > 0 and metrics["retries"] >= 1 and metrics["nodes_written"] == 0,
                         "authority:real-logical-preparation-then-revalidation", variant=variant, processes=count, metrics=metrics)
            original = originals[index]
            values = runner.membership(store, variant, original["request"], original["item"], resumed,
                                       "authority:retained-prefix")
            if resumed:
                runner.check(values[0]["state"] == "accepted" and values[0]["outcome"] is None,
                             "authority:resumed-refusal-preserves-accepted-prefix", retained=values[0])


def lost_reply(runner, variant, pause_ms):
    for count in (1, 2, 4, 8):
        for mode in ("PROTO_FAULT", "PROTO_FAIL"):
            store = runner.new_store(variant, "lostreply-%s-%s" % (count, mode), 64)
            originals = [request("lost-%s" % index, content="retained lost reply original") for index in range(count)]
            markers = [store / ("agent-%s.prepared" % index) for index in range(count)]
            children = [runner.launch(store, variant, {**original, "ready_file": str(marker), "pause_ms": pause_ms},
                        "lost-reply-cohorts", "before-reply", {mode: "before_reply"}, processes=count, agent=index, mode=mode)
                        for index, (original, marker) in enumerate(zip(originals, markers))]
            for child, marker in zip(children, markers):
                runner.wait_ready(child, marker, "lostreply:actual-prepared")
            for child in children:
                record = child.finish()
                if mode == "PROTO_FAULT":
                    runner.check(record["returncode"] == 86 and record["response"] is None,
                                 "lostreply:process-died-at-reply-boundary", variant=variant, processes=count)
                else:
                    response = record["response"] or {}
                    runner.check(response.get("ok") is False and "before_reply" in response.get("error", "")
                                 and response.get("publication") == "indeterminate"
                                 and response.get("metrics", {}).get("buffered_pairs", 0) >= 2,
                                 "lostreply:io-error-after-prepared-publication", variant=variant, processes=count, response=response)
            for original in originals:
                values = runner.membership(store, variant, original["request"], original["item"], True, "lostreply:selected-terminal")
                runner.check(values[0]["state"] == "terminal" and values[0]["outcome"] ==
                             "outcome:%s:retained lost reply original" % original["request"], "lostreply:exact-terminal-retained", retained=values[0])
            runner.success(runner.call(store, variant, {"op": "authority", "epoch": 2, "scope": False},
                                      "lost-reply-cohorts", "revoke-before-replay", processes=count, mode=mode), "lostreply:revoke")
            before = (store / "ROOT").read_bytes()
            replays = [runner.launch(store, variant, {**original, "read_only": True, "pause_ms": 400},
                       "lost-reply-cohorts", "exact-replay-after-revocation", processes=count, agent=index, mode=mode)
                       for index, original in enumerate(originals)]
            for original, child in zip(originals, replays):
                record = child.finish()
                result = runner.success(record, "lostreply:replay")
                metrics = record["response"]["metrics"]
                runner.check(result["status"] == "replay" and result["outcome"] ==
                             "outcome:%s:retained lost reply original" % original["request"]
                             and metrics["replay_repairs"] == 1 and metrics["syncs"] >= 2 and metrics["lock_hold_ns"] > 0,
                             "lostreply:exact-replay-after-required-boundary", variant=variant, processes=count, response=record["response"])
            runner.check((store / "ROOT").read_bytes() == before, "lostreply:replay-keeps-terminal-membership-root", variant=variant, processes=count)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("binary", "work", "output"):
        parser.add_argument("--" + name, required=True)
    parser.add_argument("--baseline", action="store_true")
    parser.add_argument("--pause-ms", type=int, default=2500)
    parser.add_argument("--variants", default="chunk,disk")
    parser.add_argument("--seed", type=int, default=20261012)
    parser.add_argument("--timeout", type=float, default=180)
    args = parser.parse_args()
    binary, work, output = (Path(value).resolve() for value in (args.binary, args.work, args.output))
    variants = args.variants.split(",")
    if not binary.is_file() or work.exists() or output.exists() or set(variants) - {"chunk", "disk"}:
        parser.error("existing binary, new work/output and chunk/disk variants required")
    if not 1000 <= args.pause_ms <= 60000:
        parser.error("pause must be1..60 seconds")
    work.mkdir(parents=True)
    output.mkdir(parents=True)
    env = environment(binary, work, args)
    env["evidence_run_id"] = output.name
    env["child_stdio_encoding"] = "utf-8, explicit on stdin/stdout/stderr independent of native locale"
    env["experiment"] = "reviewed candidate replay dispatch bypass reproduction" if args.baseline else "final replay repair and prepared authority/lost-reply width cohorts"
    if args.baseline:
        relative = "evidence/windows-tx-final-20261012c/environment.json" if os.name == "nt" else "evidence/linux-tx-final-20261012b/environment.json"
        manifest = Path(__file__).resolve().parents[1] / relative
        source_provenance = json.loads(manifest.read_text(encoding="utf-8"))
        if source_provenance["binary_sha256"] != env["binary_sha256"]:
            raise ValueError("candidate binary hash does not match candidate source manifest")
        env["binary_source_provenance_manifest"] = relative
        env["binary_source_provenance_sha256"] = source_provenance["source_sha256_at_run_start"]
        env["source_at_run_start_note"] = "Current source can already contain the fix; candidate binary provenance is the separately hash-matched previous environment manifest."
    (output / "environment.json").write_text(json.dumps(env, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    runner = Runner(binary, output, work, args.timeout)
    try:
        for variant in variants:
            print("replay-dispatch", variant, flush=True)
            try:
                replay_dispatch(runner, variant, args.baseline)
                if not args.baseline:
                    print("fresh-authority", variant, flush=True)
                    authority_races(runner, variant, False, args.pause_ms)
                    print("resumed-authority", variant, flush=True)
                    authority_races(runner, variant, True, args.pause_ms)
                    print("lost-reply", variant, flush=True)
                    lost_reply(runner, variant, args.pause_ms)
            except Exception as error:
                runner.note("transaction-races:variant-aborted", status="fail", variant=variant,
                            error=type(error).__name__ + ": " + str(error))
                print("FAILED", variant, error, flush=True)
        status = {"complete": runner.failures == 0, "full_harness_complete": False, "passes": runner.passes,
                  "failures": runner.failures, "skips": runner.skips, "observations": runner.observations,
                  "variants": variants, "baseline": args.baseline, "processes": [1, 2, 4, 8], "evidence_run_id": output.name}
        (output / "run-status.json").write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json_line(status), flush=True)
        return int(runner.failures != 0)
    finally:
        runner.close()


if __name__ == "__main__":
    raise SystemExit(main())
