#!/usr/bin/env python3
"""Validate Handbook-local top-level orchestration state."""

from __future__ import annotations

import argparse
import re
from pathlib import Path
from typing import Any

from protocol_json import (
    is_absolute_platform_path,
    is_canonical_branch_ref,
    is_canonical_repo_relative_path,
    load_json,
)

OID_RE = re.compile(r"^[0-9a-f]{40}$")
DIGEST_RE = re.compile(r"^sha256:[0-9a-f]{64}$")
SLICE_ID_RE = re.compile(r"^HCM-[0-9]+\.[0-9]+$")
STATE_STATUSES = {
    "INITIALIZING",
    "READY",
    "DISPATCHING",
    "RUNNING",
    "RECEIPT_RECEIVED",
    "VERIFYING",
    "EVIDENCE_RUNNING",
    "EVIDENCE_RECEIPTS_RECEIVED",
    "EVIDENCE_VERIFYING",
    "BLOCKED",
    "COMPLETE",
}
ACTIVE_MUTATION_STATUSES = {"DISPATCHING", "RUNNING", "RECEIPT_RECEIVED", "VERIFYING"}
ACTIVE_EVIDENCE_STATUSES = {
    "EVIDENCE_RUNNING",
    "EVIDENCE_RECEIPTS_RECEIVED",
    "EVIDENCE_VERIFYING",
}
PLATFORMS = {"linux", "macos", "windows"}


def fail(message: str) -> None:
    raise SystemExit(message)


def require_object(value: Any, name: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        fail(f"{name} must be an object")
    return value


def require_string(value: Any, name: str) -> str:
    if not isinstance(value, str) or not value:
        fail(f"{name} must be a non-empty string")
    return value


def require_oid(value: Any, name: str) -> str:
    text = require_string(value, name)
    if not OID_RE.fullmatch(text):
        fail(f"{name} must be 40 lowercase hexadecimal characters")
    return text


def require_digest(value: Any, name: str) -> str:
    text = require_string(value, name)
    if not DIGEST_RE.fullmatch(text):
        fail(f"{name} must be a canonical SHA-256")
    return text


def require_bool(value: Any, name: str) -> bool:
    if not isinstance(value, bool):
        fail(f"{name} must be a boolean")
    return value


def require_repo_path(value: Any, name: str) -> str:
    text = require_string(value, name)
    if not is_canonical_repo_relative_path(text):
        fail(f"{name} must be a canonical repository-relative POSIX path")
    return text


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("state", type=Path)
    args = parser.parse_args()
    try:
        data = require_object(load_json(args.state), "state")
    except ValueError as exc:
        fail(f"invalid state JSON: {exc}")

    if data.get("protocol") != "codex.top-level-orchestration-state.v1":
        fail("unsupported state protocol")
    if data.get("completion_profile") != "handbook_v1_4":
        fail("completion_profile must be handbook_v1_4")
    if data.get("publication_mode") != "local_only":
        fail("publication_mode must be local_only")
    require_string(data.get("meta_workflow_id"), "meta_workflow_id")

    status = require_string(data.get("status"), "status")
    if status not in STATE_STATUSES:
        fail(f"unsupported status: {status}")
    dispatch_authorized = require_bool(
        data.get("dispatch_authorized"), "dispatch_authorized"
    )
    if status == "INITIALIZING" and dispatch_authorized:
        fail("INITIALIZING must not authorize dispatch")
    if (
        status in ACTIVE_MUTATION_STATUSES | ACTIVE_EVIDENCE_STATUSES
        and not dispatch_authorized
    ):
        fail(f"{status} requires dispatch_authorized=true")

    authority = require_object(data.get("delegated_authority"), "delegated_authority")
    require_bool(
        authority.get("launch_preauthorized_sequence"),
        "delegated_authority.launch_preauthorized_sequence",
    )
    require_bool(
        authority.get("adjudicate_bounded_blockers"),
        "delegated_authority.adjudicate_bounded_blockers",
    )
    if require_bool(
        authority.get("expand_sequence"), "delegated_authority.expand_sequence"
    ):
        fail("delegated_authority.expand_sequence must remain false")

    require_string(data.get("remote"), "remote")
    require_oid(data.get("remote_baseline"), "remote_baseline")
    target_ref = require_string(data.get("target_ref"), "target_ref")
    if not is_canonical_branch_ref(target_ref):
        fail("target_ref must be a canonical refs/heads/ branch ref")
    require_oid(data.get("required_ancestor"), "required_ancestor")

    sequence = data.get("sequence")
    if (
        not isinstance(sequence, list)
        or not sequence
        or not all(
            isinstance(item, str) and SLICE_ID_RE.fullmatch(item) for item in sequence
        )
    ):
        fail("sequence must be a non-empty array of exact HCM slice IDs")
    if len(sequence) != len(set(sequence)):
        fail("sequence entries must be unique")

    required_evidence = data.get("required_evidence")
    if not isinstance(required_evidence, list):
        fail("required_evidence must be an array")
    evidence_requirements: dict[str, dict[str, str]] = {}
    for index, entry in enumerate(required_evidence):
        obj = require_object(entry, f"required_evidence[{index}]")
        evidence_id = require_string(
            obj.get("evidence_id"), f"required_evidence[{index}].evidence_id"
        )
        if evidence_id in evidence_requirements:
            fail("required evidence IDs must be unique")
        platform = require_string(
            obj.get("platform"), f"required_evidence[{index}].platform"
        )
        if platform not in PLATFORMS:
            fail(f"required_evidence[{index}].platform is unsupported")
        after_increment = require_string(
            obj.get("after_increment"),
            f"required_evidence[{index}].after_increment",
        )
        before_increment = require_string(
            obj.get("before_increment"),
            f"required_evidence[{index}].before_increment",
        )
        if after_increment not in sequence or before_increment not in sequence:
            fail(f"required_evidence[{index}] increments must exist in sequence")
        if sequence.index(after_increment) + 1 != sequence.index(before_increment):
            fail(
                f"required_evidence[{index}] must gate the immediately following increment"
            )
        evidence_requirements[evidence_id] = {
            "platform": platform,
            "after_increment": after_increment,
            "before_increment": before_increment,
        }

    cursor = data.get("cursor")
    if type(cursor) is not int or cursor < 0 or cursor > len(sequence):
        fail("cursor must be an integer within sequence bounds")
    base = require_object(data.get("expected_base"), "expected_base")
    base_commit = require_oid(base.get("commit"), "expected_base.commit")
    base_tree = require_oid(base.get("tree"), "expected_base.tree")

    meta = require_object(data.get("meta"), "meta")
    for field in ("thread_id", "host_id"):
        if meta.get(field) is not None and not isinstance(meta[field], str):
            fail(f"meta.{field} must be null or a string")
    if status != "INITIALIZING":
        require_string(meta.get("thread_id"), "meta.thread_id")
        require_string(meta.get("host_id"), "meta.host_id")

    active = data.get("active_dispatch")
    if status in ACTIVE_MUTATION_STATUSES:
        active_obj = require_object(active, "active_dispatch")
        active_increment = require_string(
            active_obj.get("increment"), "active_dispatch.increment"
        )
        if cursor >= len(sequence) or active_increment != sequence[cursor]:
            fail("active_dispatch.increment must equal sequence[cursor]")
        require_string(active_obj.get("thread_id"), "active_dispatch.thread_id")
        require_string(active_obj.get("host_id"), "active_dispatch.host_id")
        nonce = require_string(
            active_obj.get("dispatch_nonce"), "active_dispatch.dispatch_nonce"
        )
        if not re.fullmatch(r"[0-9a-f]{32,}", nonce):
            fail(
                "active_dispatch.dispatch_nonce must be at least 32 lowercase hex characters"
            )
        active_base = require_object(
            active_obj.get("expected_base"), "active_dispatch.expected_base"
        )
        if (
            require_oid(
                active_base.get("commit"), "active_dispatch.expected_base.commit"
            )
            != base_commit
        ):
            fail("active dispatch commit must match expected_base")
        if (
            require_oid(active_base.get("tree"), "active_dispatch.expected_base.tree")
            != base_tree
        ):
            fail("active dispatch tree must match expected_base")
        if active_obj.get("target_ref") != target_ref:
            fail("active_dispatch.target_ref must match state target_ref")
    elif active is not None:
        fail(f"active_dispatch must be null while status is {status}")

    checkpoints = data.get("active_slice_checkpoints")
    if not isinstance(checkpoints, list):
        fail("active_slice_checkpoints must be an array")
    checkpoint_commits: list[str] = []
    for index, entry in enumerate(checkpoints):
        obj = require_object(entry, f"active_slice_checkpoints[{index}]")
        if cursor >= len(sequence) or obj.get("increment") != sequence[cursor]:
            fail(
                f"active_slice_checkpoints[{index}] must belong to the current increment"
            )
        checkpoint_commits.append(
            require_oid(obj.get("commit"), f"active_slice_checkpoints[{index}].commit")
        )
        require_oid(obj.get("tree"), f"active_slice_checkpoints[{index}].tree")
        require_repo_path(
            obj.get("receipt_path"), f"active_slice_checkpoints[{index}].receipt_path"
        )
        require_repo_path(
            obj.get("handoff_record_path"),
            f"active_slice_checkpoints[{index}].handoff_record_path",
        )
    if len(checkpoint_commits) != len(set(checkpoint_commits)):
        fail("active_slice_checkpoints commits must be unique")
    if checkpoints and (
        checkpoints[-1]["commit"] != base_commit or checkpoints[-1]["tree"] != base_tree
    ):
        fail("expected_base must equal the latest active-slice checkpoint")

    completed = data.get("completed")
    if not isinstance(completed, list):
        fail("completed must be an array")
    if cursor != len(completed):
        fail("cursor must equal completed entry count")
    actual_completed: list[str] = []
    for index, entry in enumerate(completed):
        obj = require_object(entry, f"completed[{index}]")
        actual_completed.append(
            require_string(obj.get("increment"), f"completed[{index}].increment")
        )
        require_oid(obj.get("primary_tip"), f"completed[{index}].primary_tip")
        require_oid(obj.get("commit"), f"completed[{index}].commit")
        require_oid(obj.get("tree"), f"completed[{index}].tree")
        require_repo_path(obj.get("receipt_path"), f"completed[{index}].receipt_path")
        require_repo_path(
            obj.get("handoff_record_path"), f"completed[{index}].handoff_record_path"
        )
        require_digest(
            obj.get("handoff_record_sha256"),
            f"completed[{index}].handoff_record_sha256",
        )
    if actual_completed != sequence[:cursor]:
        fail("completed entries must match the sequence prefix")
    completed_by_increment = {entry["increment"]: entry for entry in completed}
    if completed and not checkpoints:
        last = completed[-1]
        if last["commit"] != base_commit or last["tree"] != base_tree:
            fail("expected_base must equal the latest completed closeout")

    evidence_dispatches = data.get("active_evidence_dispatches")
    if not isinstance(evidence_dispatches, list):
        fail("active_evidence_dispatches must be an array")
    active_evidence_ids: list[str] = []
    active_evidence_sources: dict[str, tuple[str, str]] = {}
    for index, entry in enumerate(evidence_dispatches):
        obj = require_object(entry, f"active_evidence_dispatches[{index}]")
        evidence_id = require_string(
            obj.get("evidence_id"), f"active_evidence_dispatches[{index}].evidence_id"
        )
        active_evidence_ids.append(evidence_id)
        if evidence_id not in evidence_requirements:
            fail(f"active_evidence_dispatches[{index}] is not required")
        if obj.get("platform") != evidence_requirements[evidence_id]["platform"]:
            fail(f"active_evidence_dispatches[{index}].platform mismatches requirement")
        require_string(
            obj.get("project_id"), f"active_evidence_dispatches[{index}].project_id"
        )
        project_path = require_string(
            obj.get("project_path"), f"active_evidence_dispatches[{index}].project_path"
        )
        if not is_absolute_platform_path(project_path):
            fail(f"active_evidence_dispatches[{index}].project_path must be absolute")
        require_string(
            obj.get("thread_id"), f"active_evidence_dispatches[{index}].thread_id"
        )
        require_string(
            obj.get("host_id"), f"active_evidence_dispatches[{index}].host_id"
        )
        nonce = require_string(
            obj.get("dispatch_nonce"),
            f"active_evidence_dispatches[{index}].dispatch_nonce",
        )
        if not re.fullmatch(r"[0-9a-f]{32,}", nonce):
            fail(f"active_evidence_dispatches[{index}].dispatch_nonce is invalid")
        source = require_object(
            obj.get("source"), f"active_evidence_dispatches[{index}].source"
        )
        commit = require_oid(
            source.get("commit"), f"active_evidence_dispatches[{index}].source.commit"
        )
        tree = require_oid(
            source.get("tree"), f"active_evidence_dispatches[{index}].source.tree"
        )
        if source.get("target_ref") != target_ref:
            fail(f"active_evidence_dispatches[{index}] source ref must match state")
        if (
            require_oid(
                source.get("live_local"),
                f"active_evidence_dispatches[{index}].source.live_local",
            )
            != commit
        ):
            fail(
                f"active_evidence_dispatches[{index}] live_local must equal source commit"
            )
        active_evidence_sources[evidence_id] = (commit, tree)
    if len(active_evidence_ids) != len(set(active_evidence_ids)):
        fail("active evidence IDs must be unique")
    if status in ACTIVE_EVIDENCE_STATUSES and not evidence_dispatches:
        fail(f"{status} requires active_evidence_dispatches")
    if status not in ACTIVE_EVIDENCE_STATUSES and evidence_dispatches:
        fail(f"active_evidence_dispatches must be empty while status is {status}")

    verified_evidence = data.get("verified_evidence")
    if not isinstance(verified_evidence, list):
        fail("verified_evidence must be an array")
    verified_ids: list[str] = []
    verified_evidence_sources: dict[str, tuple[str, str]] = {}
    for index, entry in enumerate(verified_evidence):
        obj = require_object(entry, f"verified_evidence[{index}]")
        evidence_id = require_string(
            obj.get("evidence_id"), f"verified_evidence[{index}].evidence_id"
        )
        verified_ids.append(evidence_id)
        if evidence_id not in evidence_requirements:
            fail(f"verified_evidence[{index}] is not required")
        if obj.get("platform") != evidence_requirements[evidence_id]["platform"]:
            fail(f"verified_evidence[{index}].platform mismatches requirement")
        require_string(obj.get("project_id"), f"verified_evidence[{index}].project_id")
        project_path = require_string(
            obj.get("project_path"), f"verified_evidence[{index}].project_path"
        )
        if not is_absolute_platform_path(project_path):
            fail(f"verified_evidence[{index}].project_path must be absolute")
        require_repo_path(
            obj.get("receipt_path"), f"verified_evidence[{index}].receipt_path"
        )
        require_digest(
            obj.get("receipt_sha256"), f"verified_evidence[{index}].receipt_sha256"
        )
        require_string(obj.get("thread_id"), f"verified_evidence[{index}].thread_id")
        require_string(obj.get("host_id"), f"verified_evidence[{index}].host_id")
        nonce = require_string(
            obj.get("dispatch_nonce"), f"verified_evidence[{index}].dispatch_nonce"
        )
        if not re.fullmatch(r"[0-9a-f]{32,}", nonce):
            fail(f"verified_evidence[{index}].dispatch_nonce is invalid")
        source = require_object(obj.get("source"), f"verified_evidence[{index}].source")
        commit = require_oid(
            source.get("commit"), f"verified_evidence[{index}].source.commit"
        )
        tree = require_oid(
            source.get("tree"), f"verified_evidence[{index}].source.tree"
        )
        if source.get("target_ref") != target_ref:
            fail(f"verified_evidence[{index}] source ref must match state")
        if (
            require_oid(
                source.get("live_local"),
                f"verified_evidence[{index}].source.live_local",
            )
            != commit
        ):
            fail(f"verified_evidence[{index}] live_local must equal source commit")
        verified_evidence_sources[evidence_id] = (commit, tree)
    if len(verified_ids) != len(set(verified_ids)):
        fail("verified evidence IDs must be unique")

    for evidence_id, source in {
        **active_evidence_sources,
        **verified_evidence_sources,
    }.items():
        checkpoint = evidence_requirements[evidence_id]["after_increment"]
        completed_checkpoint = completed_by_increment.get(checkpoint)
        if completed_checkpoint is None:
            fail(f"{evidence_id} requires completed source checkpoint {checkpoint}")
        if source != (completed_checkpoint["commit"], completed_checkpoint["tree"]):
            fail(f"{evidence_id} source must match completed checkpoint {checkpoint}")

    required_for_current: set[str] = set()
    if cursor < len(sequence):
        required_for_current = {
            evidence_id
            for evidence_id, requirement in evidence_requirements.items()
            if requirement["before_increment"] == sequence[cursor]
        }
    verified_set = set(verified_ids)
    if (
        status in ACTIVE_EVIDENCE_STATUSES
        and set(active_evidence_ids) != required_for_current
    ):
        fail("active evidence dispatches must exactly match the current evidence gate")
    if status in ACTIVE_MUTATION_STATUSES and not required_for_current.issubset(
        verified_set
    ):
        fail("current increment may not run before all required evidence verifies")

    if status == "COMPLETE" and cursor != len(sequence):
        fail("COMPLETE requires every sequence entry")
    if cursor == len(sequence) and status != "COMPLETE":
        fail("a fully consumed sequence must use COMPLETE status")
    if status == "COMPLETE" and verified_set != set(evidence_requirements):
        fail("COMPLETE requires exactly all declared evidence")
    if status == "COMPLETE" and checkpoints:
        fail("COMPLETE may not retain active_slice_checkpoints")

    print(f"VALID state {data['meta_workflow_id']} status={status} cursor={cursor}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
