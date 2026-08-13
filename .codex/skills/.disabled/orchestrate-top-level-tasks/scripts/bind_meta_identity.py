#!/usr/bin/env python3
"""Atomically bind a Handbook meta task identity without authorizing dispatch."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

from protocol_json import load_json


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("state", type=Path)
    parser.add_argument("--thread-id", required=True)
    parser.add_argument("--host-id", required=True)
    return parser.parse_args()


def validate_state_file(path: Path) -> None:
    result = subprocess.run(
        [sys.executable, str(Path(__file__).with_name("validate_state.py")), str(path)],
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        output = (result.stdout + result.stderr).strip()
        raise SystemExit(f"state validation failed: {output}")


def main() -> int:
    args = parse_args()
    if not args.thread_id or not args.host_id:
        raise SystemExit("thread and host IDs must be non-empty")
    validate_state_file(args.state)
    try:
        state = load_json(args.state)
    except ValueError as exc:
        raise SystemExit(f"invalid state JSON: {exc}") from exc
    if not isinstance(state, dict):
        raise SystemExit("state must be a JSON object")
    if state.get("protocol") != "codex.top-level-orchestration-state.v1":
        raise SystemExit("unsupported state protocol")
    if state.get("completion_profile") != "handbook_v1_4":
        raise SystemExit("unsupported completion profile")
    if state.get("publication_mode") != "local_only":
        raise SystemExit("Handbook meta state must use local_only publication")
    if state.get("status") != "INITIALIZING":
        raise SystemExit("identity may bind only from INITIALIZING")
    if state.get("meta") != {"thread_id": None, "host_id": None}:
        raise SystemExit("meta identity is already partially or fully bound")
    if state.get("active_dispatch") is not None:
        raise SystemExit("cannot bind identity with an active dispatch")

    state["meta"] = {"thread_id": args.thread_id, "host_id": args.host_id}
    state["status"] = "READY"
    state["dispatch_authorized"] = False

    payload = json.dumps(state, indent=2, sort_keys=False) + "\n"
    args.state.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        prefix=args.state.name + ".",
        dir=args.state.parent,
        text=True,
    )
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as handle:
            handle.write(payload)
            handle.flush()
            os.fsync(handle.fileno())
        validate_state_file(Path(temporary_name))
        os.replace(temporary_name, args.state)
    except BaseException:
        try:
            os.unlink(temporary_name)
        except FileNotFoundError:
            pass
        raise
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
