#!/usr/bin/env python3
"""Native repair checks for installed-orphan reuse and selected-root replay."""
import argparse
import json
from pathlib import Path
from run import Runner, environment, request


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--work", required=True)
    parser.add_argument("--seed", type=int, default=20261012)
    parser.add_argument("--timeout", type=float, default=180)
    args = parser.parse_args()
    binary, output, work = (Path(x).resolve() for x in (args.binary, args.output, args.work))
    if not binary.is_file() or output.exists() or work.exists():
        parser.error("existing binary and new output/work required")
    output.mkdir(parents=True)
    work.mkdir(parents=True)
    (output / "environment.json").write_text(json.dumps(environment(binary, work, args), indent=2, sort_keys=True) + "\n", encoding="utf-8")
    runner = Runner(binary, output, work, args.timeout)
    try:
        for variant in ("chunk", "disk"):
            for mode in ("PROTO_FAULT", "PROTO_FAIL"):
                store = runner.new_store(variant, "orphan-" + mode)
                target = request("orphan-original")
                runner.success(runner.call(store, variant, request("acknowledged"), "repair", "acknowledged"), "repair:acknowledged")
                before = runner.success(runner.call(store, variant, {"op": "snapshot"}, "repair", "before-orphan"), "repair:before")
                old_objects = {p for p in (store / "objects").glob("*/*") if len(p.name) == 64}
                fault = runner.call(store, variant, target, "repair", "object_install", {mode: "object_install"}, mode=mode)
                runner.check(fault["returncode"] == 86 if mode == "PROTO_FAULT" else ((fault["response"] or {}).get("ok") is False
                             and "object_install" in (fault["response"] or {}).get("error", "")),
                             "repair:install-interrupted", variant=variant, mode=mode)
                objects = {p for p in (store / "objects").glob("*/*") if len(p.name) == 64}
                runner.check(bool(objects - old_objects), "repair:installed-orphan-exists", variant=variant, mode=mode)
                after = runner.success(runner.call(store, variant, {"op": "snapshot"}, "repair", "after-orphan"), "repair:after")
                runner.check(after["seq"] == before["seq"], "repair:orphan-not-selected", variant=variant, mode=mode)
                runner.membership(store, variant, target["request"], target["item"], False, "repair:absent-before-retry")
                retry = runner.call(store, variant, target, "repair", "orphan-retry", mode=mode)
                runner.success(retry, "repair:orphan-retry")
                metrics = retry["response"]["metrics"]
                runner.check(runner.status(retry) == "committed" and metrics["reused_objects"] >= 1 and metrics["name_boundaries"] >= 2,
                             "repair:reused-orphan-name-boundaries", variant=variant, mode=mode, metrics=metrics)
                runner.membership(store, variant, target["request"], target["item"], True, "repair:orphan-retained")
                runner.check(runner.status(runner.call(store, variant, request("acknowledged"), "repair", "acknowledged-replay")) == "replay",
                             "repair:previous-ack-survives", variant=variant, mode=mode)

                store = runner.new_store(variant, "root-repair-" + mode)
                target = request("selected-original")
                fault = runner.call(store, variant, target, "repair", "root_replace", {mode: "root_replace"}, mode=mode)
                runner.check(fault["returncode"] == 86 if mode == "PROTO_FAULT" else ((fault["response"] or {}).get("ok") is False
                             and "root_replace" in (fault["response"] or {}).get("error", "")),
                             "repair:root-replace-interrupted", variant=variant, mode=mode)
                runner.membership(store, variant, target["request"], target["item"], True, "repair:selected-before-replay")
                # The injected error is after the replay publication repair, so
                # reaching it proves that exact retry did not skip that route.
                stopped = runner.call(store, variant, target, "repair", "replay_sync", {"PROTO_FAIL": "replay_sync"}, mode=mode)
                response = stopped["response"] or {}
                runner.check(response.get("ok") is False and "replay_sync" in response.get("error", "")
                             and response.get("metrics", {}).get("replay_repairs") == 1,
                             "repair:replay-boundary-reached-no-ack", variant=variant, mode=mode, response=response)
                replay = runner.call(store, variant, target, "repair", "replay-after-repair", mode=mode)
                result = runner.success(replay, "repair:replay-after-repair")
                runner.check(runner.status(replay) == "replay" and replay["response"]["metrics"]["replay_repairs"] == 1
                             and replay["response"]["metrics"]["syncs"] >= 2
                             and result["outcome"] == "outcome:selected-original:content",
                             "repair:ack-after-root-boundary", variant=variant, mode=mode, response=replay["response"])
        status = {"complete": runner.failures == 0, "full_harness_complete": False, "passes": runner.passes,
                  "failures": runner.failures, "skips": runner.skips, "observations": runner.observations,
                  "sections": ["review-publication-repair"], "variants": ["chunk", "disk"]}
        (output / "run-status.json").write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json.dumps(status, sort_keys=True))
    finally:
        runner.close()
    return int(runner.failures != 0)


if __name__ == "__main__":
    raise SystemExit(main())
