#!/usr/bin/env python3
"""Validate a Handbook-local top-level task terminal receipt."""

from __future__ import annotations

import argparse
import re
from pathlib import Path
from typing import Any

from protocol_json import (
    is_canonical_branch_ref,
    is_canonical_repo_relative_path,
    load_json,
)

OID_RE = re.compile(r"^[0-9a-f]{40}$")
DIGEST_RE = re.compile(r"^sha256:[0-9a-f]{64}$")
BLOCKED_STATUSES = {
    "BLOCKED_CONTRADICTION",
    "BLOCKED_SCOPE_EXPANSION",
    "BLOCKED_REVIEW",
    "BLOCKED_NATIVE_EVIDENCE",
    "BLOCKED_PLATFORM_HANDOFF_REQUIRED",
    "BLOCKED_TASK_NOT_TERMINAL",
    "AUTHORITY_REQUIRED",
    "BASE_DRIFT",
}
HANDOFF_STATUSES = {"partial", "blocked", "escalation_required"}


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
        fail(f"{name} must be sha256 plus 64 lowercase hexadecimal characters")
    return text


def require_bool(value: Any, name: str, expected: bool | None = None) -> bool:
    if not isinstance(value, bool):
        fail(f"{name} must be a boolean")
    if expected is not None and value is not expected:
        fail(f"{name} must be {str(expected).lower()}")
    return value


def require_zero(value: Any, name: str) -> None:
    if type(value) is not int or value != 0:
        fail(f"{name} must be integer zero")


def require_nonnegative_integer(value: Any, name: str) -> int:
    if type(value) is not int or value < 0:
        fail(f"{name} must be a nonnegative integer")
    return value


def require_repo_path(value: Any, name: str, prefix: str | None = None) -> str:
    text = require_string(value, name)
    if not is_canonical_repo_relative_path(text):
        fail(f"{name} must be a canonical repository-relative POSIX path")
    if prefix is not None and not text.startswith(prefix):
        fail(f"{name} must start with {prefix}")
    return text


def require_oid_array(value: Any, name: str) -> list[str]:
    if not isinstance(value, list) or not value:
        fail(f"{name} must be a non-empty array")
    result = [require_oid(item, f"{name}[{index}]") for index, item in enumerate(value)]
    if len(result) != len(set(result)):
        fail(f"{name} must not contain duplicates")
    return result


def validate_handbook_stop(value: Any) -> None:
    stop = require_object(value, "handbook_stop")
    require_repo_path(
        stop.get("handoff_record_path"),
        "handbook_stop.handoff_record_path",
        "docs/specs/handbook-contract-membrane/handoffs/records/",
    )
    require_digest(
        stop.get("handoff_record_sha256"), "handbook_stop.handoff_record_sha256"
    )
    status = require_string(stop.get("status"), "handbook_stop.status")
    if status not in HANDOFF_STATUSES:
        fail("handbook_stop.status is unsupported")
    primary_commits = stop.get("primary_commits")
    if not isinstance(primary_commits, list):
        fail("handbook_stop.primary_commits must be an array")
    validated_commits = [
        require_oid(item, f"handbook_stop.primary_commits[{index}]")
        for index, item in enumerate(primary_commits)
    ]
    if len(validated_commits) != len(set(validated_commits)):
        fail("handbook_stop.primary_commits must not contain duplicates")
    require_oid(stop.get("closeout_commit"), "handbook_stop.closeout_commit")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("receipt", type=Path)
    parser.add_argument(
        "--expected-next",
        help="Require next_increment to match the preauthorized sequence value.",
    )
    args = parser.parse_args()
    try:
        data = require_object(load_json(args.receipt), "receipt")
    except ValueError as exc:
        fail(f"invalid receipt JSON: {exc}")

    if data.get("protocol") != "codex.top-level-task-receipt.v1":
        fail("unsupported receipt protocol")
    if data.get("completion_profile") != "handbook_v1_4":
        fail("completion_profile must be handbook_v1_4")
    if data.get("publication_mode") != "local_only":
        fail("publication_mode must be local_only")
    require_string(data.get("meta_workflow_id"), "meta_workflow_id")
    nonce = require_string(data.get("dispatch_nonce"), "dispatch_nonce")
    if not re.fullmatch(r"[0-9a-f]{32,}", nonce):
        fail("dispatch_nonce must be at least 32 lowercase hexadecimal characters")
    increment = require_string(data.get("increment"), "increment")
    status = require_string(data.get("status"), "status")
    increment_task = require_object(data.get("increment_task"), "increment_task")
    require_string(increment_task.get("thread_id"), "increment_task.thread_id")
    require_string(increment_task.get("host_id"), "increment_task.host_id")

    base = require_object(data.get("expected_base"), "expected_base")
    require_oid(base.get("commit"), "expected_base.commit")
    require_oid(base.get("tree"), "expected_base.tree")

    if status in BLOCKED_STATUSES:
        if "landed" in data:
            fail("blocked receipt must not contain landed")
        blocker = require_object(data.get("blocker"), "blocker")
        require_string(blocker.get("summary"), "blocker.summary")
        require_string(blocker.get("details"), "blocker.details")
        require_string(blocker.get("required_authority"), "blocker.required_authority")
        require_bool(
            blocker.get("same_slice_adjudicable"), "blocker.same_slice_adjudicable"
        )
        if (
            status == "BLOCKED_PLATFORM_HANDOFF_REQUIRED"
            or blocker.get("handoff_prompt") is not None
        ):
            require_string(blocker.get("handoff_prompt"), "blocker.handoff_prompt")
        if data.get("handbook_stop") is not None:
            validate_handbook_stop(data["handbook_stop"])
        else:
            require_string(
                blocker.get("durable_handoff_failure"),
                "blocker.durable_handoff_failure",
            )
        print(f"VALID receipt {increment} status={status}")
        return 0

    if status != "CLOSED_LOCAL_CLEAN":
        fail(f"unsupported receipt status: {status}")

    landed = require_object(data.get("landed"), "landed")
    primary_commits = require_oid_array(
        landed.get("primary_commits"), "landed.primary_commits"
    )
    primary_tip = require_oid(landed.get("primary_tip"), "landed.primary_tip")
    if primary_tip != primary_commits[-1]:
        fail("landed.primary_tip must equal the last primary commit")
    closeout_commit = require_oid(
        landed.get("closeout_commit"), "landed.closeout_commit"
    )
    commit = require_oid(landed.get("commit"), "landed.commit")
    if closeout_commit != commit:
        fail("landed.closeout_commit must equal landed.commit")
    if primary_tip == closeout_commit:
        fail("Handbook closeout commit must be separate from the primary tip")
    require_oid(landed.get("tree"), "landed.tree")
    target_ref = require_string(landed.get("target_ref"), "landed.target_ref")
    if not is_canonical_branch_ref(target_ref):
        fail("landed.target_ref must be a canonical refs/heads/ branch ref")
    if require_oid(landed.get("live_local"), "landed.live_local") != commit:
        fail("landed.live_local must equal landed.commit")
    remote_baseline = require_oid(
        landed.get("remote_baseline"), "landed.remote_baseline"
    )
    if require_oid(landed.get("live_remote"), "landed.live_remote") != remote_baseline:
        fail("landed.live_remote must equal landed.remote_baseline")
    require_bool(landed.get("push_performed"), "landed.push_performed", False)

    subject_fingerprint = require_digest(
        data.get("subject_fingerprint"), "subject_fingerprint"
    )
    changed_files = data.get("changed_files")
    if not isinstance(changed_files, list) or not changed_files:
        fail("changed_files must be a non-empty array")
    if not all(
        isinstance(path, str) and is_canonical_repo_relative_path(path)
        for path in changed_files
    ):
        fail("changed_files entries must be canonical repository-relative POSIX paths")
    if changed_files != sorted(set(changed_files)):
        fail("changed_files must be sorted and unique")

    review = require_object(data.get("review"), "review")
    if require_string(review.get("slice_id"), "review.slice_id") != increment:
        fail("review.slice_id must equal increment")
    require_repo_path(
        review.get("handoff_record_path"),
        "review.handoff_record_path",
        "docs/specs/handbook-contract-membrane/handoffs/records/",
    )
    require_digest(review.get("handoff_record_sha256"), "review.handoff_record_sha256")
    if review.get("handoff_schema_version") != "1.4":
        fail("review.handoff_schema_version must be 1.4")
    require_repo_path(
        review.get("final_dispatch_path"),
        "review.final_dispatch_path",
        "docs/specs/handbook-contract-membrane/handoffs/dispatches/",
    )
    require_digest(review.get("final_dispatch_sha256"), "review.final_dispatch_sha256")
    if review.get("subject_fingerprint") != subject_fingerprint:
        fail("review.subject_fingerprint must equal the receipt subject_fingerprint")
    if review.get("terminal") != "CLEAN":
        fail("review.terminal must be CLEAN")
    require_zero(review.get("p1"), "review.p1")
    require_zero(review.get("p2"), "review.p2")
    require_string(review.get("p3_p4_disposition"), "review.p3_p4_disposition")
    if (
        require_nonnegative_integer(
            review.get("dispatch_population_count"),
            "review.dispatch_population_count",
        )
        == 0
    ):
        fail("review.dispatch_population_count must be positive")
    require_digest(
        review.get("dispatch_population_fingerprint"),
        "review.dispatch_population_fingerprint",
    )
    cadence = require_object(review.get("causal_cadence"), "review.causal_cadence")
    require_bool(
        cadence.get("v1_4_validation_passed"),
        "review.causal_cadence.v1_4_validation_passed",
        True,
    )
    default_budget_respected = require_bool(
        cadence.get("default_budget_respected"),
        "review.causal_cadence.default_budget_respected",
    )
    supplemental_cycles = require_nonnegative_integer(
        cadence.get("max_supplemental_cycles_used"),
        "review.causal_cadence.max_supplemental_cycles_used",
    )
    extension_ref = cadence.get("explicit_budget_extension_ref")
    if default_budget_respected:
        if supplemental_cycles > 2:
            fail("default causal cadence permits at most two supplemental cycles")
        if extension_ref is not None:
            fail(
                "explicit budget extension must be null when the default budget was respected"
            )
    else:
        require_repo_path(
            extension_ref,
            "review.causal_cadence.explicit_budget_extension_ref",
        )
    require_bool(
        cadence.get("general_discovery_reopened"),
        "review.causal_cadence.general_discovery_reopened",
        False,
    )
    require_bool(
        cadence.get("cycle_after_clean"),
        "review.causal_cadence.cycle_after_clean",
        False,
    )
    require_bool(
        cadence.get("budget_reset_attempted"),
        "review.causal_cadence.budget_reset_attempted",
        False,
    )

    control = require_object(data.get("control_plane"), "control_plane")
    if require_repo_path(control.get("ledger_path"), "control_plane.ledger_path") != (
        "docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
    ):
        fail("control_plane.ledger_path must name the canonical ledger")
    require_digest(control.get("ledger_sha256"), "control_plane.ledger_sha256")
    for field in (
        "ordinary_validation_passed",
        "self_test_v1_admission_passed",
        "self_test_orchestration_contract_passed",
    ):
        require_bool(control.get(field), f"control_plane.{field}", True)

    verification = require_object(data.get("verification"), "verification")
    for field in (
        "required_checks_passed",
        "allowlist_passed",
        "proof_wall_passed",
        "formatting_passed",
        "selected_scope_clean",
        "protected_paths_unchanged",
        "local_ref_cas_passed",
    ):
        require_bool(verification.get(field), f"verification.{field}", True)
    if verification.get("gitnexus_scoped") != "passed":
        fail("verification.gitnexus_scoped must be passed")
    compare_status = require_string(
        verification.get("gitnexus_compare_to_main"),
        "verification.gitnexus_compare_to_main",
    )
    if compare_status not in {"passed", "unavailable"}:
        fail("verification.gitnexus_compare_to_main must be passed or unavailable")
    unavailable_reason = verification.get("gitnexus_unavailable_reason")
    if compare_status == "unavailable":
        require_string(unavailable_reason, "verification.gitnexus_unavailable_reason")
    elif unavailable_reason is not None:
        fail(
            "verification.gitnexus_unavailable_reason must be null or absent when comparison passed"
        )
    require_nonnegative_integer(verification.get("ahead"), "verification.ahead")
    require_zero(verification.get("behind"), "verification.behind")

    next_increment = require_string(data.get("next_increment"), "next_increment")
    if args.expected_next is None:
        fail("--expected-next is required for a CLOSED_LOCAL_CLEAN receipt")
    if next_increment != args.expected_next:
        fail("next_increment does not match the preauthorized expected successor")
    print(f"VALID receipt {increment} status=CLOSED_LOCAL_CLEAN commit={commit}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
