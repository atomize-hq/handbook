#!/usr/bin/env python3
"""Run positive and negative tests for the Handbook-local validators."""

from __future__ import annotations

import copy
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "assets"
SCRIPTS = ROOT / "scripts"


def run(script: str, *arguments: str, success: bool = True) -> None:
    result = subprocess.run(
        [sys.executable, str(SCRIPTS / script), *arguments],
        text=True,
        capture_output=True,
        check=False,
    )
    if (result.returncode == 0) is not success:
        output = (result.stdout + result.stderr).strip()
        raise SystemExit(
            f"{script} unexpected return code {result.returncode}: {output}"
        )


def write_case(directory: Path, name: str, value: dict[str, Any]) -> Path:
    path = directory / name
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    return path


def load(name: str) -> dict[str, Any]:
    return json.loads((ASSETS / name).read_text(encoding="utf-8"))


def main() -> int:
    run("validate_state.py", str(ASSETS / "state.example.json"))
    run(
        "validate_receipt.py",
        str(ASSETS / "landing-receipt.example.json"),
        "--expected-next",
        "HCM-3.3",
    )
    run("validate_receipt.py", str(ASSETS / "blocked-receipt.example.json"))
    run("validate_evidence_receipt.py", str(ASSETS / "evidence-receipt.example.json"))

    with tempfile.TemporaryDirectory() as temporary:
        directory = Path(temporary)

        invalid_binding_state = load("state.example.json")
        invalid_binding_state["sequence"][0] = "phase-3-context-work"
        invalid_binding_path = write_case(
            directory, "invalid-binding-state.json", invalid_binding_state
        )
        run(
            "bind_meta_identity.py",
            str(invalid_binding_path),
            "--thread-id",
            "example-meta-thread",
            "--host-id",
            "example-host",
            success=False,
        )
        rejected_state = json.loads(invalid_binding_path.read_text(encoding="utf-8"))
        if rejected_state["status"] != "INITIALIZING" or rejected_state["meta"] != {
            "thread_id": None,
            "host_id": None,
        }:
            raise SystemExit("failed identity binding mutated invalid state")

        bound_state = directory / "bound-state.json"
        bound_state.write_text(
            (ASSETS / "state.example.json").read_text(encoding="utf-8"),
            encoding="utf-8",
        )
        run(
            "bind_meta_identity.py",
            str(bound_state),
            "--thread-id",
            "example-meta-thread",
            "--host-id",
            "example-host",
        )
        authorized_state = json.loads(bound_state.read_text(encoding="utf-8"))
        authorized_state["dispatch_authorized"] = True
        bound_state.write_text(
            json.dumps(authorized_state, indent=2) + "\n", encoding="utf-8"
        )
        run("validate_state.py", str(bound_state))

        rendered_prompt = directory / "rendered-prompt.md"
        run(
            "render_increment_prompt.py",
            "--state",
            str(bound_state),
            "--template",
            str(ASSETS / "increment-orchestrator-prompt-template.md"),
            "--variables",
            str(ASSETS / "increment-variables.example.json"),
            "--contract",
            str(ASSETS / "increment-contract.example.md"),
            "--output",
            str(rendered_prompt),
        )
        rendered = rendered_prompt.read_text(encoding="utf-8")
        if (
            "{{" in rendered
            or "}}" in rendered
            or "HCM-3.2 example increment contract" not in rendered
        ):
            raise SystemExit(
                "rendered prompt did not resolve the complete example contract"
            )

        for name, key, value in (
            ("abstract-render.json", "SLICE_ID", "phase-3-context-work"),
            ("mismatched-render.json", "INCREMENT", "HCM-3.3"),
            ("unenumerated-successor.json", "NEXT_INCREMENT", "HCM-9.9"),
        ):
            variables = load("increment-variables.example.json")
            variables[key] = value
            run(
                "render_increment_prompt.py",
                "--state",
                str(bound_state),
                "--template",
                str(ASSETS / "increment-orchestrator-prompt-template.md"),
                "--variables",
                str(write_case(directory, name, variables)),
                "--contract",
                str(ASSETS / "increment-contract.example.md"),
                "--output",
                str(directory / (name + ".md")),
                success=False,
            )

        receipt = load("landing-receipt.example.json")
        receipt["landed"]["push_performed"] = True
        run(
            "validate_receipt.py",
            str(write_case(directory, "pushed.json", receipt)),
            "--expected-next",
            "HCM-3.3",
            success=False,
        )

        receipt = load("landing-receipt.example.json")
        receipt["landed"]["live_remote"] = "5555555555555555555555555555555555555555"
        run(
            "validate_receipt.py",
            str(write_case(directory, "remote-moved.json", receipt)),
            "--expected-next",
            "HCM-3.3",
            success=False,
        )

        receipt = load("landing-receipt.example.json")
        receipt["review"]["subject_fingerprint"] = (
            "sha256:1111111111111111111111111111111111111111111111111111111111111111"
        )
        run(
            "validate_receipt.py",
            str(write_case(directory, "subject-mismatch.json", receipt)),
            "--expected-next",
            "HCM-3.3",
            success=False,
        )

        receipt = load("landing-receipt.example.json")
        receipt["review"]["causal_cadence"]["cycle_after_clean"] = True
        run(
            "validate_receipt.py",
            str(write_case(directory, "cycle-after-clean.json", receipt)),
            "--expected-next",
            "HCM-3.3",
            success=False,
        )

        receipt = load("landing-receipt.example.json")
        receipt["review"]["causal_cadence"]["max_supplemental_cycles_used"] = 3
        run(
            "validate_receipt.py",
            str(write_case(directory, "unbounded-supplementals.json", receipt)),
            "--expected-next",
            "HCM-3.3",
            success=False,
        )

        receipt["review"]["causal_cadence"]["default_budget_respected"] = False
        receipt["review"]["causal_cadence"]["explicit_budget_extension_ref"] = (
            "docs/specs/handbook-contract-membrane/slices/HCM-3.2/decision/"
            "bounded-review-budget-extension.md"
        )
        run(
            "validate_receipt.py",
            str(write_case(directory, "reviewed-budget-extension.json", receipt)),
            "--expected-next",
            "HCM-3.3",
        )

        blocked = load("blocked-receipt.example.json")
        del blocked["handbook_stop"]
        run(
            "validate_receipt.py",
            str(write_case(directory, "missing-durable-stop.json", blocked)),
            success=False,
        )
        blocked["blocker"]["durable_handoff_failure"] = (
            "Repository write capability failed before a durable handoff could be committed."
        )
        run(
            "validate_receipt.py",
            str(write_case(directory, "declared-durable-stop-failure.json", blocked)),
        )

        state = load("state.example.json")
        state["delegated_authority"]["expand_sequence"] = True
        run(
            "validate_state.py",
            str(write_case(directory, "expanded.json", state)),
            success=False,
        )

        state = load("state.example.json")
        state["sequence"][0] = "phase-3-context-work"
        run(
            "validate_state.py",
            str(write_case(directory, "abstract-sequence-entry.json", state)),
            success=False,
        )

        receipt = copy.deepcopy(load("landing-receipt.example.json"))
        run(
            "validate_receipt.py",
            str(write_case(directory, "wrong-successor.json", receipt)),
            "--expected-next",
            "HCM-9.9",
            success=False,
        )

    print("VALID Handbook orchestration validator self-test")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
