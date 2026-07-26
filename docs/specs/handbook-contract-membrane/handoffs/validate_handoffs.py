#!/usr/bin/env python3
"""Validate canonical HCM handoffs and the rebuildable ledger."""

from __future__ import annotations

import copy
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path, PurePosixPath
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
RECORDS_DIR = ROOT / "records"
DISPATCHES_DIR = ROOT / "dispatches"
LEDGER_PATH = ROOT / "ledger.jsonl"
TEMPLATE_PATH = ROOT / "handoff-template.json"
INTERNAL_DISPATCH_TEMPLATE_PATH = ROOT / "internal-dispatch-template.json"
REVIEW_FINDING_INVENTORY_PATH = REPO_ROOT / (
    "docs/specs/handbook-contract-membrane/09-review-finding-inventory.md"
)
RECORD_SCHEMA_PATHS = {
    "1.0": ROOT / "handoff-record.schema.json",
    "1.1": ROOT / "handoff-record.v1.1.schema.json",
    "1.2": ROOT / "handoff-record.v1.2.schema.json",
    "1.3": ROOT / "handoff-record.v1.3.schema.json",
}
INTERNAL_DISPATCH_SCHEMA_PATHS = {
    "1.0": ROOT / "internal-dispatch.schema.json",
    "1.1": ROOT / "internal-dispatch.v1.1.schema.json",
    "1.2": ROOT / "internal-dispatch.v1.2.schema.json",
    "1.3": ROOT / "internal-dispatch.v1.3.schema.json",
}
LEDGER_SCHEMA_PATH = ROOT / "ledger-entry.schema.json"
HISTORICAL_V1_0_ADMISSION = {
    "20260712T175918Z--HCM-0-1--documentation--control-pack-bootstrap.json": (
        "20260712T175918Z--HCM-0-1--documentation--control-pack-bootstrap",
        "f6e7172d722ede4b202c0fd573c9258dfd07f31ebfd2e190b6cf339425a236e7",
    ),
    "20260712T235757Z--HCM-0-1--documentation--snapshot-memory-layering.json": (
        "20260712T235757Z--HCM-0-1--documentation--snapshot-memory-layering",
        "4eac8ec9196339a441dec0f25e52d362b0565bbf291d6e5d534277eb1f26e58f",
    ),
    "20260713T164052Z--HCM-0-1--documentation--artifact-intake-posture-layering.json": (
        "20260713T164052Z--HCM-0-1--documentation--artifact-intake-posture-layering",
        "1a9685246a133c1970d2e0c65f87fde4b3e4c55705cf5d4e10a0a62c01a225be",
    ),
    "20260714T004544Z--HCM-0-1--orchestration--dispatch-independent-review.json": (
        "20260714T004544Z--HCM-0-1--orchestration--dispatch-independent-review",
        "7d7c9d6050af9bde855180366ea7ad27c7effa39e26f3ee79dc1d7c45784635f",
    ),
    "20260714T004730Z--HCM-0-1--orchestration--correct-independent-review-dispatch.json": (
        "20260714T004730Z--HCM-0-1--orchestration--correct-independent-review-dispatch",
        "75c8746f46873733db0dd764c2a830bd8201546800a48c4c3616bc44d6896e19",
    ),
    "20260714T005400Z--HCM-0-1--review--control-pack-actionable-findings.json": (
        "20260714T005400Z--HCM-0-1--review--control-pack-actionable-findings",
        "03d21da871ba73519ef8f7d7918aa4d094b5fa06edb939370b0c0fa65b578d5a",
    ),
    "20260714T005900Z--HCM-0-1--orchestration--dispatch-review-remediation.json": (
        "20260714T005900Z--HCM-0-1--orchestration--dispatch-review-remediation",
        "1f33a9fef92b361a4f14a5237a5bf7f15ca3c2deb40d10c51f5ee2273e7f66d6",
    ),
}
HISTORICAL_V1_1_ADMISSION = {
    "20260714T010814Z--HCM-0-1--documentation--review-findings-remediated.json": (
        "20260714T010814Z--HCM-0-1--documentation--review-findings-remediated",
        "082e2ff245d2839776fd564227f6fc204b0f65cd5ba897ca2449a593dd11c1c9",
    ),
    "20260714T011043Z--HCM-0-1--orchestration--dispatch-post-remediation-review.json": (
        "20260714T011043Z--HCM-0-1--orchestration--dispatch-post-remediation-review",
        "793d2e62609307546739d75b9e8ac622a7cbe0af4580646a982fee9d1f96dd88",
    ),
    "20260714T011730Z--HCM-0-1--review--v1-routing-finding.json": (
        "20260714T011730Z--HCM-0-1--review--v1-routing-finding",
        "03609955b87b98ca9ad3d2f0663117b0d6a5149d635fd6a22d5bbf30ca2fdc88",
    ),
    "20260714T012423Z--HCM-0-1--orchestration--dispatch-v1-admission-remediation.json": (
        "20260714T012423Z--HCM-0-1--orchestration--dispatch-v1-admission-remediation",
        "048c78597c29906683fef244228100bcc004bff9800f1a25a87dd466a4fa4790",
    ),
    "20260714T012828Z--HCM-0-1--documentation--v1-historical-admission-remediated.json": (
        "20260714T012828Z--HCM-0-1--documentation--v1-historical-admission-remediated",
        "02c4e731ab1925917672ae3418f258ec683a68e76aa276495005c11b6f458148",
    ),
    "20260714T013253Z--HCM-0-1--orchestration--dispatch-final-review.json": (
        "20260714T013253Z--HCM-0-1--orchestration--dispatch-final-review",
        "b4b7b46b5d94b1f45ade98b120217488c69dfd16f36c04a5630b9ba522aeb3b4",
    ),
    "20260714T013942Z--HCM-0-1--review--final-control-pack-findings.json": (
        "20260714T013942Z--HCM-0-1--review--final-control-pack-findings",
        "7155c768c3867b132a99b2861087c2c8492d2ffba1d6111673310872eda1c002",
    ),
    "20260714T014519Z--HCM-0-1--orchestration--dispatch-final-findings-remediation.json": (
        "20260714T014519Z--HCM-0-1--orchestration--dispatch-final-findings-remediation",
        "305f6a20edcbf536160ea42d20f361c9fdd55f96140435b4790fddf9549e9898",
    ),
    "20260714T015043Z--HCM-0-1--documentation--renderer-projection-history-remediated.json": (
        "20260714T015043Z--HCM-0-1--documentation--renderer-projection-history-remediated",
        "690c02d51561d15a1e0fc73b7c683b067b6f89f7ed992a2cfbeb691da6b5c6e4",
    ),
    "20260714T015452Z--HCM-0-1--orchestration--dispatch-closure-review.json": (
        "20260714T015452Z--HCM-0-1--orchestration--dispatch-closure-review",
        "946ac63139a337a78083c9ebed61e84493ec3150f398f7b102c51fb833d3f232",
    ),
    "20260714T020014Z--HCM-0-1--review--clean-closure.json": (
        "20260714T020014Z--HCM-0-1--review--clean-closure",
        "7d4fb6225886c44344edf72acf35b687a3e4181040b09a493e80b17b616aaeb9",
    ),
}
LEGACY_DISPATCH_ADMISSION = {
    "20260714T004544Z--HCM-0-1--independent-control-pack-review.md": "176b8671eb936857d8fc0a776779f501e57d38336b7b619f61be61256d430a23",
    "20260714T004730Z--HCM-0-1--independent-control-pack-review-corrected.md": "beb8926db4a894a771ff6427ec0cd9932f4fa306cc6794d17f12d1a25efe03d2",
    "20260714T005900Z--HCM-0-1--control-pack-review-remediation.md": "fd90d939e1b96b84ab62b13142c22d676fc0026c3e00e22bdaa9e2841d26de9a",
    "20260714T011043Z--HCM-0-1--fresh-post-remediation-control-pack-review.md": "a555260a945ab71f32252351d995575fbe00293579319525185ecd3c5a9861cf",
    "20260714T012423Z--HCM-0-1--v1-historical-admission-remediation.md": "e7363ba1212a53c681fbd5a34f3f093cec9c85be94b551d328d381e472d0dedd",
    "20260714T013253Z--HCM-0-1--fresh-final-control-pack-review.md": "ea2cb05d9ff0370f2f672cc5db4065436a1bce03b30fd87dfd967dea2d1a73c1",
    "20260714T014519Z--HCM-0-1--renderer-projection-and-history-deletion-remediation.md": "5cb5de8b883c566c0066c635dbe9a8ef1ac58d6e0afab79d442e42f01d531df0",
    "20260714T015452Z--HCM-0-1--fresh-closure-review.md": "530976cbc73ade7b8a6cb03b38e587efee3a38493e3910e88c38ab0dcb4343a0",
}
HISTORICAL_INTERNAL_DISPATCH_V1_0_ADMISSION = {
    "20260714T132155Z--HCM-0-8--independent-orchestration-repair-review.json": (
        "20260714T132155Z--HCM-0-8--independent-orchestration-repair-review",
        "173fd34dfdc0e7f0dccd1061adce7219938c2aa881f4b162a10ab396f6f1adfc",
    )
}
IMMUTABLE_INTERNAL_DISPATCH_V1_1_MANIFEST_ORDER_ADMISSION = {
    "20260719T011843Z--HCM-2-1--fresh-implementation-review-1.json": (
        "7e72338480ec262014c06e6b8661d2ffd578368c3c6c4029458366bdc83b700f"
    )
}
IMMUTABLE_PREDECESSOR_DISPATCH_COUNT = 339
IMMUTABLE_PREDECESSOR_DISPATCH_CORPUS_FINGERPRINT = (
    "sha256:e20c957d98d94d09a6df8fc6ff8cb188dbc2e92d06af041807022eb467dada49"
)
IMMUTABLE_V1_2_RECORD_COUNT = 35
IMMUTABLE_V1_2_RECORD_CORPUS_FINGERPRINT = (
    "sha256:6d3e52cd295f827b18eb33b42ca8aee51cd2fc9a8ddd790a9f6a3f1505c8a228"
)
IMMUTABLE_V1_2_RECORD_FILENAMES = {
    "20260714T150800Z--HCM-0-8--orchestration--internal-delegation-control-plane-closed.json",
    "20260714T173436Z--HCM-0-2--orchestration--semantic-contracts-frozen.json",
    "20260714T221404Z--HCM-0-3--orchestration--resolution-snapshot-projection-contracts-frozen.json",
    "20260715T024728Z--HCM-0-4--orchestration--fresh-review-capability-blocked.json",
    "20260715T044452Z--HCM-0-4--orchestration--live-agent-registry-capacity-blocked.json",
    "20260715T141656Z--HCM-0-4--orchestration--sdk-transport-contracts-frozen.json",
    "20260715T164439Z--HCM-0-9--orchestration--planning-review-budget-exhausted.json",
    "20260715T181543Z--HCM-0-9--orchestration--continuation-review-2-not-clean.json",
    "20260715T191048Z--HCM-0-9--orchestration--decomposition-abandoned.json",
    "20260715T202049Z--HCM-0-5--orchestration--planning-review-budget-exhausted.json",
    "20260715T215528Z--HCM-0-5--orchestration--continuation-review-2-not-clean.json",
    "20260715T230600Z--HCM-0-5--orchestration--packet-approved-design-freeze-entry.json",
    "20260716T015949Z--HCM-0-5--orchestration--contract-dock-design-frozen.json",
    "20260716T030358Z--HCM-0-6--orchestration--default-set-decision-input-required.json",
    "20260716T153303Z--HCM-0-6--orchestration--shipped-defaults-frozen.json",
    "20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved.json",
    "20260716T222906Z--HCM-1-1--orchestration--registry-boundary-landed.json",
    "20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved.json",
    "20260717T125103Z--HCM-1-2--orchestration--profile-boundary-landed.json",
    "20260717T163627Z--HCM-1-3--orchestration--implementation-packet-approved.json",
    "20260717T183202Z--HCM-1-3--orchestration--artifact-registry-landed.json",
    "20260718T133219Z--HCM-1-4--orchestration--implementation-packet-approved.json",
    "20260718T134551Z--HCM-1-4--orchestration--windows-runtime-blocked.json",
    "20260718T191425Z--HCM-1-4--orchestration--profile-aware-setup-doctor-landed.json",
    "20260718T224708Z--HCM-2-1--orchestration--implementation-packet-approved.json",
    "20260718T231105Z--HCM-2-1--orchestration--windows-runtime-blocked.json",
    "20260719T104915Z--HCM-2-1--orchestration--project-context-canonical-yaml-landed.json",
    "20260719T230914Z--HCM-2-2--orchestration--implementation-packet-approved.json",
    "20260720T083855Z--HCM-2-2--orchestration--lifecycle-validation-authority-boundary.json",
    "20260720T160434Z--HCM-2-2--orchestration--authority-repair-planning-approved.json",
    "20260720T225541Z--HCM-2-2--orchestration--atomic-stage-authority-repair-approved.json",
    "20260721T143023Z--HCM-2-2--orchestration--exact-result-authority-repair-approved.json",
    "20260721T215654Z--HCM-2-2--orchestration--exact-result-authority-repair-landed.json",
    "20260722T042100Z--HCM-2-3--orchestration--planning-completed.json",
    "20260725T220053Z--HCM-2-3--orchestration--implementation-completed.json",
}
PRE_V1_3_DISPATCH_CLOSEOUT_ADMISSION = {
    "20260726T021305Z--HCM-0-8--orchestration--bounded-review-calibration-completed.json": (
        "20260726T021305Z--HCM-0-8--orchestration--bounded-review-calibration-completed",
        "1daecaefebe880af5c86b5bb08e6dd67c4e6859592eefbd56b10f4c8478733ae",
    ),
    "20260726T050447Z--HCM-2-4--orchestration--planning-completed.json": (
        "20260726T050447Z--HCM-2-4--orchestration--planning-completed",
        "fd4f633eb74915c781e622f53670d69b43bcbe86f0233b51a2079e0ec1d9f550",
    ),
    "20260726T131857Z--HCM-2-4--orchestration--planning-amendment-completed.json": (
        "20260726T131857Z--HCM-2-4--orchestration--planning-amendment-completed",
        "f71a077f822bce165ffdf670fbe6c6d9ce681f0a28ed215711df8c247eb87a8a",
    ),
}


class ValidationFailure(Exception):
    """A deterministic validation or parity failure."""


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise ValidationFailure(f"{path}: invalid JSON: {error}") from error
    if not isinstance(value, dict):
        raise ValidationFailure(f"{path}: expected a JSON object")
    return value


def load_review_inventory_entries(path: Path) -> dict[str, str]:
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as error:
        raise ValidationFailure(
            f"{path}: cannot read review finding inventory: {error}"
        ) from error

    def split_row(line: str, line_number: int) -> list[str]:
        if not line.startswith("|") or not line.rstrip().endswith("|"):
            raise ValidationFailure(
                f"{path}:{line_number}: malformed review inventory table row"
            )
        cells: list[str] = []
        current: list[str] = []
        body = line.strip()[1:-1]
        index = 0
        while index < len(body):
            if body[index : index + 2] == r"\|":
                current.append("|")
                index += 2
                continue
            if body[index] == "|":
                cells.append("".join(current).strip().strip("`"))
                current = []
            else:
                current.append(body[index])
            index += 1
        cells.append("".join(current).strip().strip("`"))
        return cells

    expected_header = [
        "Finding ID",
        "Priority",
        "Status",
        "Comparison key",
        "Summary",
        "Source",
        "Evidence refs",
        "Affected scope",
        "Disposition",
        "Target",
        "Occurrences",
        "Resolution refs",
    ]
    inventory_heading_indexes = [
        index for index, line in enumerate(lines) if line.strip() == "## Inventory"
    ]
    if len(inventory_heading_indexes) != 1:
        raise ValidationFailure(
            f"{path}: expected exactly one canonical Inventory section"
        )
    inventory_heading_index = inventory_heading_indexes[0]
    section_end_index = next(
        (
            index
            for index in range(inventory_heading_index + 1, len(lines))
            if lines[index].lstrip().startswith("#")
            and len(lines[index].lstrip()) > 1
            and lines[index].lstrip().split(maxsplit=1)[0].strip("#") == ""
        ),
        len(lines),
    )
    header_index = None
    for index in range(inventory_heading_index + 1, section_end_index):
        line = lines[index]
        if not line.startswith("|"):
            continue
        cells = split_row(line, index + 1)
        if cells == expected_header:
            header_index = index
            break
    if header_index is None or header_index + 1 >= len(lines):
        raise ValidationFailure(f"{path}: missing canonical Inventory table")
    separator = split_row(lines[header_index + 1], header_index + 2)
    if len(separator) != len(expected_header) or any(
        len(cell.strip(":")) < 3 or set(cell.strip(":")) != {"-"}
        for cell in separator
    ):
        raise ValidationFailure(
            f"{path}:{header_index + 2}: malformed Inventory table separator"
        )

    entries: dict[str, str] = {}
    allowed_statuses = {"open", "accepted", "scheduled", "resolved", "superseded"}
    for index in range(header_index + 2, len(lines)):
        line = lines[index]
        line_number = index + 1
        if not line.strip() or not line.startswith("|"):
            break
        cells = split_row(line, line_number)
        if len(cells) != len(expected_header) or any(not cell for cell in cells):
            raise ValidationFailure(
                f"{path}:{line_number}: inventory row must contain all "
                "12 canonical fields"
            )
        finding_id = cells[0]
        suffix = finding_id.removeprefix("HCM-RF-")
        if (
            not finding_id.startswith("HCM-RF-")
            or len(suffix) != 4
            or not suffix.isascii()
            or not suffix.isdigit()
        ):
            raise ValidationFailure(
                f"{path}:{line_number}: malformed review inventory ID"
            )
        priority = cells[1]
        if priority not in {"P1", "P2", "P3", "P4"}:
            raise ValidationFailure(
                f"{path}:{line_number}: invalid review inventory priority"
            )
        if cells[2] not in allowed_statuses:
            raise ValidationFailure(
                f"{path}:{line_number}: invalid review inventory status"
            )
        comparison_parts = [part.strip() for part in cells[3].split("|")]
        if len(comparison_parts) != 4 or any(not part for part in comparison_parts):
            raise ValidationFailure(
                f"{path}:{line_number}: invalid review inventory comparison key"
            )
        if finding_id in entries:
            raise ValidationFailure(
                f"{path}:{line_number}: duplicate review inventory ID {finding_id!r}"
            )
        entries[finding_id] = priority
    return entries


def validate_instance(
    instance: dict[str, Any], schema: dict[str, Any], label: str
) -> None:
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if not errors:
        return
    details = []
    for error in errors:
        location = "/" + "/".join(str(part) for part in error.absolute_path)
        details.append(f"{label}{location}: {error.message}")
    raise ValidationFailure("\n".join(details))


def expected_ledger_entry(record: dict[str, Any], record_path: Path) -> dict[str, Any]:
    return {
        "schema_id": "handbook.handoff-ledger-entry",
        "schema_version": "1.0",
        "handoff_id": record["handoff_id"],
        "created_at_utc": record["created_at_utc"],
        "status": record["status"],
        "session_kind": record["session"]["kind"],
        "phase_id": record["phase_id"],
        "slice_id": record["slice_id"],
        "packet_id": record["packet_id"],
        "record_path": record_path.relative_to(REPO_ROOT).as_posix(),
    }


def validate_historical_v1_0_admission(
    record: dict[str, Any], record_path: Path
) -> None:
    admitted = HISTORICAL_V1_0_ADMISSION.get(record_path.name)
    if admitted is None:
        raise ValidationFailure(
            f"{record_path}: schema_version 1.0 is historical-only; "
            "record filename is not admitted"
        )
    expected_id, expected_sha256 = admitted
    if record.get("handoff_id") != expected_id or record_path.stem != expected_id:
        raise ValidationFailure(
            f"{record_path}: historical v1.0 filename/handoff_id admission mismatch"
        )
    actual_sha256 = hashlib.sha256(record_path.read_bytes()).hexdigest()
    if actual_sha256 != expected_sha256:
        raise ValidationFailure(
            f"{record_path}: historical v1.0 SHA-256 mismatch; immutable bytes changed"
        )


def validate_historical_v1_1_admission(
    record: dict[str, Any], record_path: Path
) -> None:
    admitted = HISTORICAL_V1_1_ADMISSION.get(record_path.name)
    if admitted is None:
        raise ValidationFailure(
            f"{record_path}: schema_version 1.1 is historical-only; "
            "record filename is not admitted"
        )
    expected_id, expected_sha256 = admitted
    if record.get("handoff_id") != expected_id or record_path.stem != expected_id:
        raise ValidationFailure(
            f"{record_path}: historical v1.1 filename/handoff_id admission mismatch"
        )
    actual_sha256 = hashlib.sha256(record_path.read_bytes()).hexdigest()
    if actual_sha256 != expected_sha256:
        raise ValidationFailure(
            f"{record_path}: historical v1.1 SHA-256 mismatch; immutable bytes changed"
        )


def validate_legacy_dispatch_admission() -> None:
    paths = sorted(DISPATCHES_DIR.glob("*.md"))
    actual_names = {path.name for path in paths}
    expected_names = set(LEGACY_DISPATCH_ADMISSION)
    if actual_names != expected_names:
        missing = sorted(expected_names - actual_names)
        extra = sorted(actual_names - expected_names)
        raise ValidationFailure(
            "legacy dispatch filename set mismatch: "
            f"missing={missing}, extra={extra}"
        )
    for path in paths:
        actual_sha256 = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual_sha256 != LEGACY_DISPATCH_ADMISSION[path.name]:
            raise ValidationFailure(
                f"{path}: legacy dispatch SHA-256 mismatch; immutable bytes changed"
            )


def validate_historical_internal_dispatch_v1_0_admission(
    dispatch: dict[str, Any], dispatch_path: Path
) -> None:
    admitted = HISTORICAL_INTERNAL_DISPATCH_V1_0_ADMISSION.get(dispatch_path.name)
    if admitted is None:
        raise ValidationFailure(
            f"{dispatch_path}: internal dispatch schema_version 1.0 is "
            "historical-only; filename is not admitted"
        )
    expected_id, expected_sha256 = admitted
    if dispatch.get("dispatch_id") != expected_id or dispatch_path.stem != expected_id:
        raise ValidationFailure(
            f"{dispatch_path}: historical internal dispatch identity mismatch"
        )
    actual_sha256 = hashlib.sha256(dispatch_path.read_bytes()).hexdigest()
    if actual_sha256 != expected_sha256:
        raise ValidationFailure(
            f"{dispatch_path}: historical internal dispatch SHA-256 mismatch; "
            "immutable bytes changed"
        )


def validate_immutable_predecessor_dispatch_corpus(
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
) -> None:
    entries = sorted(
        (path.name, dispatch_sha256)
        for path, dispatch, dispatch_sha256 in dispatches.values()
        if dispatch["schema_version"] in {"1.1", "1.2"}
    )
    aggregate = "sha256:" + hashlib.sha256(
        "".join(
            f"{filename}\0{dispatch_sha256}\n"
            for filename, dispatch_sha256 in entries
        ).encode()
    ).hexdigest()
    if (
        len(entries) != IMMUTABLE_PREDECESSOR_DISPATCH_COUNT
        or aggregate != IMMUTABLE_PREDECESSOR_DISPATCH_CORPUS_FINGERPRINT
    ):
        raise ValidationFailure(
            "immutable internal-dispatch v1.1/v1.2 corpus mismatch: "
            f"count={len(entries)}, fingerprint={aggregate}"
        )


def validate_immutable_v1_2_record_corpus(
    records: list[tuple[Path, dict[str, Any]]],
) -> None:
    entries = sorted(
        (path.name, hashlib.sha256(path.read_bytes()).hexdigest())
        for path, record in records
        if record.get("schema_version") == "1.2"
    )
    actual_names = {filename for filename, _ in entries}
    if actual_names != IMMUTABLE_V1_2_RECORD_FILENAMES:
        missing = sorted(IMMUTABLE_V1_2_RECORD_FILENAMES - actual_names)
        extra = sorted(actual_names - IMMUTABLE_V1_2_RECORD_FILENAMES)
        raise ValidationFailure(
            "immutable handoff-record v1.2 filename set mismatch: "
            f"missing={missing}, extra={extra}"
        )
    aggregate = "sha256:" + hashlib.sha256(
        "".join(
            f"{filename}\0{record_sha256}\n"
            for filename, record_sha256 in entries
        ).encode()
    ).hexdigest()
    if (
        len(entries) != IMMUTABLE_V1_2_RECORD_COUNT
        or aggregate != IMMUTABLE_V1_2_RECORD_CORPUS_FINGERPRINT
    ):
        raise ValidationFailure(
            "immutable handoff-record v1.2 corpus mismatch: "
            f"count={len(entries)}, fingerprint={aggregate}"
        )


def validate_pre_v1_3_dispatch_closeout_admission(
    record: dict[str, Any],
    record_path: Path,
) -> bool:
    admitted = PRE_V1_3_DISPATCH_CLOSEOUT_ADMISSION.get(record_path.name)
    if admitted is None:
        return False
    expected_id, expected_sha256 = admitted
    if record.get("handoff_id") != expected_id or record_path.stem != expected_id:
        raise ValidationFailure(
            f"{record_path}: predecessor-dispatch closeout identity mismatch"
        )
    actual_sha256 = hashlib.sha256(record_path.read_bytes()).hexdigest()
    if actual_sha256 != expected_sha256:
        raise ValidationFailure(
            f"{record_path}: predecessor-dispatch closeout SHA-256 mismatch; "
            "immutable bytes changed"
        )
    return True


def validate_subject_whitespace(
    subject_bytes: bytes,
    entry_path: str,
    dispatch_path: Path,
) -> None:
    if b"\0" in subject_bytes:
        return
    try:
        subject_bytes.decode("utf-8")
    except UnicodeDecodeError:
        return
    for line_number, raw_line in enumerate(
        subject_bytes.splitlines(keepends=True),
        start=1,
    ):
        content = raw_line.rstrip(b"\r\n")
        if content.endswith((b" ", b"\t")):
            raise ValidationFailure(
                f"{dispatch_path}: subject manifest text has trailing "
                f"whitespace: {entry_path}:{line_number}"
            )


def validate_subject_manifest(
    dispatch: dict[str, Any],
    dispatch_path: Path,
    *,
    verify_live_files: bool = False,
    baseline_head: str | None = None,
    repo_root: Path = REPO_ROOT,
) -> None:
    if verify_live_files and baseline_head is not None:
        raise ValidationFailure(
            f"{dispatch_path}: choose live-file or baseline manifest validation"
        )
    if baseline_head is not None:
        result = subprocess.run(
            ["git", "cat-file", "-e", f"{baseline_head}^{{commit}}"],
            cwd=repo_root,
            capture_output=True,
            check=False,
        )
        if result.returncode != 0:
            raise ValidationFailure(
                f"{dispatch_path}: reviewed baseline commit is unavailable: "
                f"{baseline_head}"
            )
    manifest = dispatch["subject_manifest"]
    entries = manifest["entries"]
    paths = [entry["path"] for entry in entries]
    if len(paths) != len(set(paths)):
        raise ValidationFailure(
            f"{dispatch_path}: subject manifest paths must be unique and sorted"
        )
    if paths != sorted(paths):
        admitted_sha256 = IMMUTABLE_INTERNAL_DISPATCH_V1_1_MANIFEST_ORDER_ADMISSION.get(
            dispatch_path.name
        )
        actual_sha256 = hashlib.sha256(dispatch_path.read_bytes()).hexdigest()
        if admitted_sha256 is None or actual_sha256 != admitted_sha256:
            raise ValidationFailure(
                f"{dispatch_path}: subject manifest paths must be unique and sorted"
            )
    encoded: list[str] = []
    for entry in entries:
        entry_path = entry["path"]
        subject_bytes: bytes | None = None
        relative_path = PurePosixPath(entry_path)
        if (
            relative_path.is_absolute()
            or not relative_path.parts
            or "\0" in entry_path
            or "\\" in entry_path
            or ":" in entry_path
            or any(part in {"", ".", ".."} for part in relative_path.parts)
            or relative_path.as_posix() != entry_path
        ):
            raise ValidationFailure(
                f"{dispatch_path}: subject manifest path is not canonical "
                f"repository-relative: {entry_path!r}"
            )
        try:
            resolved_repo_root = repo_root.resolve()
            resolved_subject_path = (
                resolved_repo_root / Path(*relative_path.parts)
            ).resolve()
        except (OSError, ValueError) as error:
            raise ValidationFailure(
                f"{dispatch_path}: subject manifest path cannot be resolved: "
                f"{entry_path!r}"
            ) from error
        try:
            resolved_subject_path.relative_to(resolved_repo_root)
        except ValueError as error:
            raise ValidationFailure(
                f"{dispatch_path}: subject manifest path escapes repository: "
                f"{entry_path!r}"
            ) from error
        if baseline_head is not None:
            result = subprocess.run(
                ["git", "cat-file", "blob", f"{baseline_head}:{entry_path}"],
                cwd=repo_root,
                capture_output=True,
                check=False,
            )
            if result.returncode != 0:
                raise ValidationFailure(
                    f"{dispatch_path}: subject manifest path is absent from "
                    f"reviewed baseline {baseline_head}: {entry_path}"
                )
            subject_bytes = result.stdout
            actual_sha256 = hashlib.sha256(subject_bytes).hexdigest()
            if actual_sha256 != entry["sha256"]:
                raise ValidationFailure(
                    f"{dispatch_path}: reviewed baseline SHA-256 mismatch: "
                    f"{entry_path}"
                )
        elif verify_live_files:
            subject_path = resolved_subject_path
            if not subject_path.is_file():
                raise ValidationFailure(
                    f"{dispatch_path}: subject manifest path is missing: {entry_path}"
                )
            subject_bytes = subject_path.read_bytes()
            actual_sha256 = hashlib.sha256(subject_bytes).hexdigest()
            if actual_sha256 != entry["sha256"]:
                raise ValidationFailure(
                    f"{dispatch_path}: subject file SHA-256 mismatch: {entry_path}"
                )
        if (
            subject_bytes is not None
            and dispatch.get("schema_version") == "1.3"
            and dispatch.get("subject_hygiene", {}).get("whitespace_policy")
            == "text-files-no-trailing-whitespace-v1"
        ):
            validate_subject_whitespace(
                subject_bytes,
                entry_path,
                dispatch_path,
            )
        encoded.append(f"{entry_path}\0{entry['sha256']}\n")
    aggregate = "sha256:" + hashlib.sha256("".join(encoded).encode()).hexdigest()
    if manifest["aggregate_fingerprint"] != aggregate:
        raise ValidationFailure(
            f"{dispatch_path}: subject manifest aggregate fingerprint mismatch"
        )
    if dispatch["subject_fingerprint"] != aggregate:
        raise ValidationFailure(
            f"{dispatch_path}: subject_fingerprint does not match subject manifest"
        )


def validate_review_cycles(
    record_path: Path,
    runs: list[dict[str, Any]],
    run_by_id: dict[str, dict[str, Any]],
    run_order: dict[str, int],
    dispatch_by_run_id: dict[str, dict[str, Any]],
    findings_by_id: dict[str, dict[str, Any]],
    remediations: list[dict[str, Any]],
    *,
    require_typed: bool,
) -> None:
    review_runs = [run for run in runs if run["role"] == "review"]
    typed_review_runs = [
        run
        for run in review_runs
        if "review_cycle" in dispatch_by_run_id[run["run_id"]]
    ]
    if not typed_review_runs:
        if require_typed and review_runs:
            raise ValidationFailure(
                f"{record_path}: new v1.3 closeout requires typed v1.3 "
                "review-cycle lineage"
            )
        return
    if len(typed_review_runs) != len(review_runs):
        raise ValidationFailure(
            f"{record_path}: typed review-cycle lineage cannot mix typed and "
            "untyped review dispatches"
        )

    cycles: list[dict[str, Any]] = []
    cycle_by_id: dict[str, dict[str, Any]] = {}
    closed_cycle_ids: set[str] = set()
    active_cycle_id: str | None = None
    for run in review_runs:
        cycle = dispatch_by_run_id[run["run_id"]]["review_cycle"]
        cycle_id = cycle["cycle_id"]
        if cycle_id != active_cycle_id:
            if cycle_id in closed_cycle_ids:
                raise ValidationFailure(
                    f"{record_path}: review cycle {cycle_id!r} is not contiguous"
                )
            if active_cycle_id is not None:
                closed_cycle_ids.add(active_cycle_id)
            active_cycle_id = cycle_id
            cycle_entry = {
                "cycle_id": cycle_id,
                "kind": cycle["kind"],
                "trigger_run_ids": cycle["trigger_run_ids"],
                "finding_refs": cycle["finding_refs"],
                "subject_fingerprint": run["subject_fingerprint"],
                "runs": [run],
            }
            cycles.append(cycle_entry)
            cycle_by_id[cycle_id] = cycle_entry
            continue
        cycle_entry = cycle_by_id[cycle_id]
        expected = (
            cycle_entry["kind"],
            cycle_entry["trigger_run_ids"],
            cycle_entry["finding_refs"],
            cycle_entry["subject_fingerprint"],
        )
        actual = (
            cycle["kind"],
            cycle["trigger_run_ids"],
            cycle["finding_refs"],
            run["subject_fingerprint"],
        )
        if actual != expected:
            raise ValidationFailure(
                f"{record_path}: review cycle {cycle_id!r} has inconsistent "
                "kind, trigger, findings, or subject"
            )
        cycle_entry["runs"].append(run)

    kinds = [cycle["kind"] for cycle in cycles]
    if not kinds or kinds[0] != "discovery":
        raise ValidationFailure(
            f"{record_path}: typed review lineage must begin with discovery"
        )
    if kinds.count("discovery") != 1 or kinds.count("closure") > 1:
        raise ValidationFailure(
            f"{record_path}: typed review lineage permits one discovery and "
            "at most one closure cycle"
        )
    if "closure" in kinds and kinds.index("closure") != 1:
        raise ValidationFailure(
            f"{record_path}: closure must immediately follow discovery"
        )
    supplemental_count = kinds.count("supplemental_causal")
    if supplemental_count > 2:
        raise ValidationFailure(
            f"{record_path}: typed review lineage exceeds two supplemental "
            "causal cycles"
        )
    allowed_kinds = ["discovery"]
    if len(kinds) > 1:
        allowed_kinds.append("closure")
    allowed_kinds.extend(["supplemental_causal"] * max(0, len(kinds) - 2))
    if kinds != allowed_kinds:
        raise ValidationFailure(
            f"{record_path}: invalid typed review-cycle order {kinds!r}"
        )

    remediation_by_finding_run: dict[str, list[dict[str, Any]]] = {}
    for remediation in remediations:
        remediation_by_finding_run.setdefault(
            remediation["finding_run_id"], []
        ).append(remediation)
    cycle_index_by_run_id = {
        run["run_id"]: cycle_index
        for cycle_index, cycle in enumerate(cycles)
        for run in cycle["runs"]
    }
    for remediation in remediations:
        finding_cycle_index = cycle_index_by_run_id[
            remediation["finding_run_id"]
        ]
        re_review_cycle_index = cycle_index_by_run_id[
            remediation["re_review_run_id"]
        ]
        if re_review_cycle_index != finding_cycle_index + 1:
            raise ValidationFailure(
                f"{record_path}: remediation {remediation['remediation_id']!r} "
                "must cross into the immediately following review cycle"
            )

    for cycle_index, cycle in enumerate(cycles):
        if cycle["kind"] == "discovery":
            if cycle["trigger_run_ids"] or cycle["finding_refs"]:
                raise ValidationFailure(
                    f"{record_path}: discovery cycle cannot claim causal triggers"
                )
            continue

        prior_cycle = cycles[cycle_index - 1]
        if (
            cycle["subject_fingerprint"]
            == prior_cycle["subject_fingerprint"]
        ):
            raise ValidationFailure(
                f"{record_path}: review cycle {cycle['cycle_id']!r} must "
                "bind a changed post-remediation subject"
            )
        expected_trigger_ids = [
            run["run_id"]
            for run in prior_cycle["runs"]
            if run["verdict"] == "findings"
        ]
        if cycle["trigger_run_ids"] != expected_trigger_ids:
            raise ValidationFailure(
                f"{record_path}: review cycle {cycle['cycle_id']!r} must name "
                "exactly the preceding findings review runs"
            )
        if not expected_trigger_ids:
            raise ValidationFailure(
                f"{record_path}: review cycle {cycle['cycle_id']!r} cannot "
                "follow a CLEAN cycle"
            )
        expected_finding_refs = sorted(
            finding_id
            for finding_id, finding in findings_by_id.items()
            if finding["source_run_id"] in expected_trigger_ids
            and finding["priority"] in {"P1", "P2"}
        )
        if sorted(cycle["finding_refs"]) != expected_finding_refs:
            raise ValidationFailure(
                f"{record_path}: review cycle {cycle['cycle_id']!r} must name "
                "exactly the triggering P1/P2 findings"
            )
        current_run_ids = {run["run_id"] for run in cycle["runs"]}
        for trigger_run_id in expected_trigger_ids:
            trigger_run = run_by_id[trigger_run_id]
            if (
                run_order[trigger_run_id]
                >= min(run_order[run_id] for run_id in current_run_ids)
                or trigger_run["verdict"] != "findings"
            ):
                raise ValidationFailure(
                    f"{record_path}: review cycle {cycle['cycle_id']!r} has an "
                    "invalid causal trigger"
                )
            if not any(
                remediation["re_review_run_id"] in current_run_ids
                for remediation in remediation_by_finding_run.get(
                    trigger_run_id, []
                )
            ):
                raise ValidationFailure(
                    f"{record_path}: review cycle {cycle['cycle_id']!r} lacks "
                    f"typed remediation linkage for {trigger_run_id!r}"
                )


def validate_v1_2_semantics(
    record: dict[str, Any],
    record_path: Path,
    all_record_ids: set[str],
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
    *,
    verify_final_subject_baseline: bool = False,
    review_inventory_entries: dict[str, str] | None = None,
) -> None:
    if (
        record["schema_version"] == "1.2"
        and record_path.name not in IMMUTABLE_V1_2_RECORD_FILENAMES
    ):
        raise ValidationFailure(
            f"{record_path}: handoff-record v1.2 is immutable predecessor "
            "evidence and cannot be newly created"
        )
    legacy_predecessor_closeout = (
        record_path.name in PRE_V1_3_DISPATCH_CLOSEOUT_ADMISSION
    )
    for source_id in [*record["source_handoff_ids"], *record["supersedes"]]:
        if source_id not in all_record_ids:
            raise ValidationFailure(
                f"{record_path}: unknown source/superseded handoff_id {source_id!r}"
            )

    runs = record["delegated_runs"]
    run_by_id: dict[str, dict[str, Any]] = {}
    run_order: dict[str, int] = {}
    dispatch_by_run_id: dict[str, dict[str, Any]] = {}
    for index, run in enumerate(runs):
        run_id = run["run_id"]
        if run_id in run_by_id:
            raise ValidationFailure(f"{record_path}: duplicate delegated run_id {run_id!r}")
        run_by_id[run_id] = run
        run_order[run_id] = index

        dispatch_id = run["dispatch_id"]
        if dispatch_id not in dispatches:
            raise ValidationFailure(
                f"{record_path}: delegated run references unknown internal dispatch "
                f"{dispatch_id!r}"
            )
        dispatch_path, dispatch, dispatch_sha256 = dispatches[dispatch_id]
        dispatch_by_run_id[run_id] = dispatch
        if (
            record["schema_version"] == "1.3"
            and not legacy_predecessor_closeout
            and dispatch["schema_version"] != "1.3"
        ):
            raise ValidationFailure(
                f"{record_path}: new v1.3 closeout run {run_id!r} must use "
                "internal-dispatch v1.3"
            )
        expected_ref = dispatch_path.relative_to(REPO_ROOT).as_posix()
        if run["dispatch_ref"] != expected_ref:
            raise ValidationFailure(
                f"{record_path}: delegated run {run_id!r} dispatch_ref mismatch"
            )
        if run["dispatch_fingerprint"] != f"sha256:{dispatch_sha256}":
            raise ValidationFailure(
                f"{record_path}: delegated run {run_id!r} dispatch fingerprint mismatch"
            )
        matching_fields = {
            "parent_orchestration_id": record["orchestration_id"],
            "phase_id": record["phase_id"],
            "slice_id": record["slice_id"],
            "packet_id": record["packet_id"],
            "role": run["role"],
            "subject_fingerprint": run["subject_fingerprint"],
        }
        for field, expected in matching_fields.items():
            if dispatch[field] != expected:
                raise ValidationFailure(
                    f"{record_path}: delegated run {run_id!r} disagrees with "
                    f"dispatch field {field}"
                )
        if dispatch["required_skills"] != run["required_skills"]:
            raise ValidationFailure(
                f"{record_path}: delegated run {run_id!r} required_skills mismatch"
            )
        if run["role"] == "review" and run["result_subject_fingerprint"] != run["subject_fingerprint"]:
            raise ValidationFailure(
                f"{record_path}: read-only review run {run_id!r} changed its subject"
            )

    for run_id, run in run_by_id.items():
        predecessor = run["predecessor_run_id"]
        if predecessor is not None:
            if predecessor not in run_by_id or run_order[predecessor] >= run_order[run_id]:
                raise ValidationFailure(
                    f"{record_path}: delegated run {run_id!r} has invalid predecessor"
                )
        if run["remediation_for_run_ids"] and (
            run["role"] != "remediation" or run["final_status"] != "completed"
        ):
            raise ValidationFailure(
                f"{record_path}: delegated run {run_id!r} claims remediation "
                "without a completed remediation role"
            )
        for finding_run_id in run["remediation_for_run_ids"]:
            finding_run = run_by_id.get(finding_run_id)
            if (
                finding_run is None
                or run_order[finding_run_id] >= run_order[run_id]
                or finding_run["role"] != "review"
                or finding_run["verdict"] != "findings"
            ):
                raise ValidationFailure(
                    f"{record_path}: delegated run {run_id!r} has invalid remediation lineage"
                )

    remediations = record["remediations"]
    remediation_ids: set[str] = set()
    remediations_by_finding: dict[str, list[dict[str, Any]]] = {}
    for remediation in remediations:
        remediation_id = remediation["remediation_id"]
        if remediation_id in remediation_ids:
            raise ValidationFailure(
                f"{record_path}: duplicate remediation_id {remediation_id!r}"
            )
        remediation_ids.add(remediation_id)
        finding_run = run_by_id.get(remediation["finding_run_id"])
        if (
            finding_run is None
            or finding_run["role"] != "review"
            or finding_run["verdict"] != "findings"
        ):
            raise ValidationFailure(
                f"{record_path}: remediation {remediation_id!r} does not name "
                "a findings review"
            )
        re_review = run_by_id.get(remediation["re_review_run_id"])
        if (
            re_review is None
            or re_review["role"] != "review"
            or re_review["final_status"] != "completed"
            or run_order[re_review["run_id"]] <= run_order[finding_run["run_id"]]
            or re_review["subject_fingerprint"]
            != remediation["result_subject_fingerprint"]
        ):
            raise ValidationFailure(
                f"{record_path}: remediation {remediation_id!r} lacks a later "
                "completed re-review of its result subject"
            )
        if re_review["agent_id"] == finding_run["agent_id"]:
            raise ValidationFailure(
                f"{record_path}: reviewer {finding_run['agent_id']!r} reused "
                "after remediation"
            )
        if remediation["owner"] == "delegated_run":
            delegated_run = run_by_id.get(remediation["delegated_run_id"])
            if (
                delegated_run is None
                or delegated_run["role"] != "remediation"
                or delegated_run["final_status"] != "completed"
                or finding_run["run_id"]
                not in delegated_run["remediation_for_run_ids"]
                or delegated_run["result_subject_fingerprint"]
                != remediation["result_subject_fingerprint"]
                or run_order[delegated_run["run_id"]]
                >= run_order[re_review["run_id"]]
            ):
                raise ValidationFailure(
                    f"{record_path}: remediation {remediation_id!r} has invalid "
                    "delegated-run evidence"
                )
        remediations_by_finding.setdefault(finding_run["run_id"], []).append(
            remediation
        )

    findings_reviews = [
        run for run in runs if run["role"] == "review" and run["verdict"] == "findings"
    ]
    if record["schema_version"] == "1.3":
        findings_by_id: dict[str, dict[str, Any]] = {}
        findings_by_run: dict[str, list[dict[str, Any]]] = {}
        expected_severity = {
            "P1": "critical",
            "P2": "major",
            "P3": "warning",
            "P4": "info",
        }
        for finding in record["findings"]:
            finding_id = finding["finding_id"]
            if finding_id in findings_by_id:
                raise ValidationFailure(
                    f"{record_path}: duplicate finding_id {finding_id!r}"
                )
            findings_by_id[finding_id] = finding
            if finding["severity"] != expected_severity[finding["priority"]]:
                raise ValidationFailure(
                    f"{record_path}: finding {finding_id!r} priority/severity mismatch"
                )
            source_run = run_by_id.get(finding["source_run_id"])
            if (
                source_run is None
                or source_run["role"] != "review"
                or finding_id not in source_run["finding_refs"]
            ):
                raise ValidationFailure(
                    f"{record_path}: finding {finding_id!r} lacks exact review-run linkage"
                )
            findings_by_run.setdefault(source_run["run_id"], []).append(finding)

        for run in runs:
            if run["role"] != "review":
                continue
            for finding_id in run["finding_refs"]:
                if finding_id not in findings_by_id:
                    raise ValidationFailure(
                        f"{record_path}: review run {run['run_id']!r} references "
                        f"unknown finding {finding_id!r}"
                    )
                if findings_by_id[finding_id]["source_run_id"] != run["run_id"]:
                    raise ValidationFailure(
                        f"{record_path}: review run {run['run_id']!r} references "
                        f"finding {finding_id!r} owned by a different run"
                    )
            linked_findings = findings_by_run.get(run["run_id"], [])
            has_blocking = any(
                finding["priority"] in {"P1", "P2"}
                for finding in linked_findings
            )
            if run["verdict"] == "clean" and has_blocking:
                raise ValidationFailure(
                    f"{record_path}: clean review run {run['run_id']!r} carries "
                    "a P1/P2 finding"
                )
            if run["verdict"] == "findings" and not has_blocking:
                raise ValidationFailure(
                    f"{record_path}: findings review run {run['run_id']!r} "
                    "has no P1/P2 finding"
                )

        validate_review_cycles(
            record_path,
            runs,
            run_by_id,
            run_order,
            dispatch_by_run_id,
            findings_by_id,
            remediations,
            require_typed=not legacy_predecessor_closeout,
        )

        if record["status"] == "completed":
            if review_inventory_entries is None:
                review_inventory_entries = load_review_inventory_entries(
                    REVIEW_FINDING_INVENTORY_PATH
                )
            for finding in record["findings"]:
                if finding["priority"] in {"P1", "P2"}:
                    if finding["status"] not in {"remediated", "resolved"}:
                        raise ValidationFailure(
                            f"{record_path}: completed closeout leaves blocking "
                            f"finding {finding['finding_id']!r} unresolved"
                        )
                    if finding["source_run_id"] not in remediations_by_finding:
                        raise ValidationFailure(
                            f"{record_path}: completed closeout leaves blocking "
                            f"finding {finding['finding_id']!r} without typed remediation"
                        )
                elif finding["status"] == "open":
                    raise ValidationFailure(
                        f"{record_path}: completed closeout leaves advisory finding "
                        f"{finding['finding_id']!r} unregistered"
                    )
                elif finding["status"] == "inventoried":
                    inventory_priority = review_inventory_entries.get(
                        finding["finding_id"]
                    )
                    if inventory_priority != finding["priority"]:
                        raise ValidationFailure(
                            f"{record_path}: inventoried advisory "
                            f"{finding['finding_id']!r} lacks a matching durable "
                            "inventory row"
                        )

    if record["status"] == "completed":
        for finding_run in findings_reviews:
            if finding_run["run_id"] not in remediations_by_finding:
                raise ValidationFailure(
                    f"{record_path}: completed closeout leaves findings run "
                    f"{finding_run['run_id']!r} without typed remediation"
                )

    if record["status"] == "completed":
        if (
            record["reviewed_state"]["baseline_head"]
            != record["repo_state"]["head"]
        ):
            raise ValidationFailure(
                f"{record_path}: completed closeout reviewed baseline does not "
                "match the recorded primary slice commit"
            )
        completed_reviews = [
            run
            for run in runs
            if run["role"] == "review" and run["final_status"] == "completed"
        ]
        final_review = completed_reviews[-1]
        if final_review["verdict"] != "clean":
            raise ValidationFailure(
                f"{record_path}: final completed review verdict is not clean"
            )
        if final_review["subject_fingerprint"] != record["reviewed_state"]["subject_fingerprint"]:
            raise ValidationFailure(
                f"{record_path}: final clean review does not bind reviewed_state"
            )
        if final_review["dispatch_ref"] != record["reviewed_state"]["subject_manifest_ref"]:
            raise ValidationFailure(
                f"{record_path}: reviewed_state does not reference final review manifest"
            )
        final_dispatch_path, final_dispatch, _ = dispatches[final_review["dispatch_id"]]
        expected_dispatch_version = (
            "1.2"
            if legacy_predecessor_closeout
            and record["schema_version"] == "1.3"
            else "1.3"
            if record["schema_version"] == "1.3"
            else "1.1"
        )
        if final_dispatch["schema_version"] != expected_dispatch_version:
            raise ValidationFailure(
                f"{record_path}: final clean review lacks replayable "
                f"v{expected_dispatch_version} subject manifest"
            )
        if verify_final_subject_baseline:
            validate_subject_manifest(
                final_dispatch,
                final_dispatch_path,
                baseline_head=record["reviewed_state"]["baseline_head"],
            )


def isolate_historical_admission_fixture(temp_root: Path) -> None:
    """Remove current-protocol artifacts from an immutable-history fixture.

    The admission self-test copies only the handoff subtree. Replayable v1.1
    through v1.3 dispatches intentionally bind manifests that reach the wider
    repository, so they cannot be validated inside that reduced fixture. The
    test is about byte admission for historical records and dispatches; remove
    v1.2/v1.3 records and v1.1/v1.2/v1.3 dispatches before exercising it.
    """
    for path in (temp_root / "records").glob("*.json"):
        if load_json(path).get("schema_version") in {"1.2", "1.3"}:
            path.unlink()
    for path in (temp_root / "dispatches").glob("*.json"):
        if load_json(path).get("schema_version") in {"1.1", "1.2", "1.3"}:
            path.unlink()
    remaining_record_versions = {
        load_json(path).get("schema_version")
        for path in (temp_root / "records").glob("*.json")
    }
    remaining_dispatch_versions = {
        load_json(path).get("schema_version")
        for path in (temp_root / "dispatches").glob("*.json")
    }
    if remaining_record_versions & {"1.2", "1.3"}:
        raise ValidationFailure(
            "historical admission fixture retained a current-protocol record"
        )
    if remaining_dispatch_versions & {"1.1", "1.2"}:
        raise ValidationFailure(
            "historical admission fixture retained a current-protocol dispatch"
        )


def run_historical_v1_0_admission_self_test() -> int:
    scenarios = (
        ("unknown", "historical v1.0 canonical filename set mismatch"),
        ("modified", "historical v1.0 SHA-256 mismatch"),
        ("deleted", "historical v1.0 canonical filename set mismatch"),
    )
    with tempfile.TemporaryDirectory(prefix="hcm-v1-admission-") as temp_dir:
        for scenario, expected_failure in scenarios:
            temp_repo = Path(temp_dir) / scenario
            temp_root = (
                temp_repo
                / "docs"
                / "specs"
                / "handbook-contract-membrane"
                / "handoffs"
            )
            shutil.copytree(ROOT, temp_root)
            current_record_sentinel = (
                temp_root / "records" / "self-test-current-v1.3.json"
            )
            current_record_sentinel.write_text(
                json.dumps({"schema_version": "1.3"}) + "\n"
            )
            current_dispatch_sentinel = (
                temp_root / "dispatches" / "self-test-current-v1.3.json"
            )
            current_dispatch_sentinel.write_text(
                json.dumps({"schema_version": "1.3"}) + "\n"
            )
            isolate_historical_admission_fixture(temp_root)
            if current_record_sentinel.exists() or current_dispatch_sentinel.exists():
                print(
                    "historical v1.0 admission self-test failed: current-version "
                    "sentinel survived fixture isolation",
                    file=sys.stderr,
                )
                return 1
            temp_records = temp_root / "records"
            if scenario == "unknown":
                source = temp_records / next(iter(HISTORICAL_V1_0_ADMISSION))
                record = load_json(source)
                handoff_id = (
                    "99991231T235959Z--HCM-0-1--documentation--unauthorized-v1-record"
                )
                record["handoff_id"] = handoff_id
                record["created_at_utc"] = "9999-12-31T23:59:59Z"
                validate_instance(
                    record,
                    load_json(temp_root / "handoff-record.schema.json"),
                    "self-test otherwise-schema-valid v1.0 record",
                )
                unknown_path = temp_records / f"{handoff_id}.json"
                unknown_path.write_text(json.dumps(record, indent=2) + "\n")

            elif scenario == "modified":
                admitted_path = temp_records / next(iter(HISTORICAL_V1_0_ADMISSION))
                admitted_path.write_bytes(admitted_path.read_bytes() + b" ")
            else:
                admitted_path = temp_records / next(iter(HISTORICAL_V1_0_ADMISSION))
                admitted_path.unlink()

            rebuilt_entries = []
            for path in sorted(temp_records.glob("*.json")):
                candidate = load_json(path)
                rebuilt_entries.append(
                    {
                        "schema_id": "handbook.handoff-ledger-entry",
                        "schema_version": "1.0",
                        "handoff_id": candidate["handoff_id"],
                        "created_at_utc": candidate["created_at_utc"],
                        "status": candidate["status"],
                        "session_kind": candidate["session"]["kind"],
                        "phase_id": candidate["phase_id"],
                        "slice_id": candidate["slice_id"],
                        "packet_id": candidate["packet_id"],
                        "record_path": path.relative_to(temp_repo).as_posix(),
                    }
                )
            (temp_root / "ledger.jsonl").write_text(
                "".join(
                    json.dumps(entry, separators=(",", ":"), ensure_ascii=False)
                    + "\n"
                    for entry in rebuilt_entries
                )
            )

            result = subprocess.run(
                [
                    sys.executable,
                    str(temp_root / "validate_handoffs.py"),
                    "--validate-historical-fixture",
                ],
                cwd=temp_repo,
                capture_output=True,
                text=True,
                check=False,
            )
            if result.returncode == 0 or expected_failure not in result.stderr:
                print(
                    "historical v1.0 admission self-test failed: "
                    f"{scenario} scenario returned {result.returncode}; "
                    f"stderr={result.stderr!r}",
                    file=sys.stderr,
                )
                return 1

        extended_scenarios = (
            ("unknown-v1-1", "historical v1.1 canonical filename set mismatch"),
            ("modified-v1-1", "historical v1.1 SHA-256 mismatch"),
            ("deleted-dispatch", "legacy dispatch filename set mismatch"),
            ("modified-dispatch", "legacy dispatch SHA-256 mismatch"),
            (
                "deleted-internal-dispatch",
                "historical internal-dispatch v1.0 filename set mismatch",
            ),
            (
                "modified-internal-dispatch",
                "historical internal dispatch SHA-256 mismatch",
            ),
        )
        for scenario, expected_failure in extended_scenarios:
            temp_repo = Path(temp_dir) / scenario
            temp_root = (
                temp_repo
                / "docs"
                / "specs"
                / "handbook-contract-membrane"
                / "handoffs"
            )
            shutil.copytree(ROOT, temp_root)
            isolate_historical_admission_fixture(temp_root)
            temp_records = temp_root / "records"
            if scenario == "unknown-v1-1":
                source = temp_records / next(iter(HISTORICAL_V1_1_ADMISSION))
                record = load_json(source)
                handoff_id = "99991231T235958Z--HCM-0-8--orchestration--unauthorized-v1-1"
                record["handoff_id"] = handoff_id
                record["created_at_utc"] = "9999-12-31T23:59:58Z"
                unknown_path = temp_records / f"{handoff_id}.json"
                unknown_path.write_text(json.dumps(record, indent=2) + "\n")
            elif scenario == "modified-v1-1":
                admitted_path = temp_records / next(iter(HISTORICAL_V1_1_ADMISSION))
                admitted_path.write_bytes(admitted_path.read_bytes() + b" ")
            elif scenario == "deleted-dispatch":
                dispatch_path = temp_root / "dispatches" / next(iter(LEGACY_DISPATCH_ADMISSION))
                dispatch_path.unlink()
            elif scenario == "modified-dispatch":
                dispatch_path = temp_root / "dispatches" / next(iter(LEGACY_DISPATCH_ADMISSION))
                dispatch_path.write_bytes(dispatch_path.read_bytes() + b" ")
            elif scenario == "deleted-internal-dispatch":
                dispatch_path = temp_root / "dispatches" / next(
                    iter(HISTORICAL_INTERNAL_DISPATCH_V1_0_ADMISSION)
                )
                dispatch_path.unlink()
            else:
                dispatch_path = temp_root / "dispatches" / next(
                    iter(HISTORICAL_INTERNAL_DISPATCH_V1_0_ADMISSION)
                )
                dispatch_path.write_bytes(dispatch_path.read_bytes() + b" ")

            rebuilt_entries = []
            for path in sorted(temp_records.glob("*.json")):
                candidate = load_json(path)
                rebuilt_entries.append(
                    {
                        "schema_id": "handbook.handoff-ledger-entry",
                        "schema_version": "1.0",
                        "handoff_id": candidate["handoff_id"],
                        "created_at_utc": candidate["created_at_utc"],
                        "status": candidate["status"],
                        "session_kind": candidate["session"]["kind"],
                        "phase_id": candidate["phase_id"],
                        "slice_id": candidate["slice_id"],
                        "packet_id": candidate["packet_id"],
                        "record_path": path.relative_to(temp_repo).as_posix(),
                    }
                )
            (temp_root / "ledger.jsonl").write_text(
                "".join(
                    json.dumps(entry, separators=(",", ":"), ensure_ascii=False)
                    + "\n"
                    for entry in rebuilt_entries
                )
            )
            result = subprocess.run(
                [
                    sys.executable,
                    str(temp_root / "validate_handoffs.py"),
                    "--validate-historical-fixture",
                ],
                cwd=temp_repo,
                capture_output=True,
                text=True,
                check=False,
            )
            if result.returncode == 0 or expected_failure not in result.stderr:
                print(
                    "immutable-history self-test failed: "
                    f"{scenario} scenario returned {result.returncode}; "
                    f"stderr={result.stderr!r}",
                    file=sys.stderr,
                )
                return 1

    print(
        "historical v1.0 admission self-test passed: "
        "unknown v1.0 record rejected; byte-modified admitted v1.0 record rejected; "
        "deleted admitted v1.0 record rejected; unknown/modified v1.1 history "
        "rejected; deleted/modified legacy and internal v1.0 dispatches rejected; "
        "current record/dispatch sentinels isolated; "
        "exact ledger rebuilt for every record scenario"
    )
    return 0


def run_orchestration_contract_self_test() -> int:
    handoff_schema = load_json(RECORD_SCHEMA_PATHS["1.3"])
    dispatch_schema = load_json(INTERNAL_DISPATCH_SCHEMA_PATHS["1.3"])
    template = load_json(TEMPLATE_PATH)
    dispatch_template = load_json(INTERNAL_DISPATCH_TEMPLATE_PATH)
    validate_instance(template, handoff_schema, "v1.3 handoff template")
    validate_instance(
        dispatch_template, dispatch_schema, "internal dispatch template"
    )

    handoff_cases: list[tuple[str, dict[str, Any]]] = []
    case = copy.deepcopy(template)
    case["session"]["kind"] = "review"
    handoff_cases.append(("child-authored-handoff", case))
    case = copy.deepcopy(template)
    case["delegated_runs"][0]["agent_type"] = "worker"
    handoff_cases.append(("non-default-agent", case))
    case = copy.deepcopy(template)
    case["delegated_runs"][0]["fresh_context"] = False
    handoff_cases.append(("non-fresh-agent", case))
    case = copy.deepcopy(template)
    case["status"] = "completed"
    case["stop_reason"] = "completed"
    case["resume"]["execution_target"] = "none"
    handoff_cases.append(("completed-without-clean-review", case))
    case = copy.deepcopy(template)
    case["status"] = "completed"
    case["stop_reason"] = "capability_unavailable"
    case["delegation_capability"]["status"] = "unavailable"
    handoff_cases.append(("capability-failure-marked-complete", case))
    case = copy.deepcopy(template)
    case["status"] = "partial"
    case["stop_reason"] = "human_input"
    case["resume"]["execution_target"] = "none"
    handoff_cases.append(("human-input-without-human-target", case))
    case = copy.deepcopy(template)
    case["status"] = "review_required"
    handoff_cases.append(("queue-shaped-review-required", case))
    unavailable_stop_shapes = (
        ("human_input", "partial", "human_interactive"),
        ("external_blocker", "blocked", "top_level_resume"),
        ("authority_boundary", "escalation_required", "top_level_resume"),
        ("context_boundary", "partial", "top_level_resume"),
    )
    for stop_reason, status, execution_target in unavailable_stop_shapes:
        case = copy.deepcopy(template)
        case["stop_reason"] = stop_reason
        case["status"] = status
        case["resume"]["execution_target"] = execution_target
        case["delegation_capability"]["status"] = "unavailable"
        handoff_cases.append((f"unavailable-delegation-as-{stop_reason}", case))

    dispatch_cases: list[tuple[str, dict[str, Any]]] = []
    case = copy.deepcopy(dispatch_template)
    case["agent_type"] = "worker"
    dispatch_cases.append(("dispatch-non-default-agent", case))
    case = copy.deepcopy(dispatch_template)
    case["return_contract"]["global_handoff"] = "allowed"
    dispatch_cases.append(("child-global-handoff", case))
    case = copy.deepcopy(dispatch_template)
    case["return_contract"]["transport"] = "codex_exec"
    dispatch_cases.append(("external-review-transport", case))
    case = copy.deepcopy(dispatch_template)
    case["required_skills"].remove("using-agent-skills")
    dispatch_cases.append(("dispatch-missing-meta-skill", case))
    case = copy.deepcopy(dispatch_template)
    case["required_skills"] = [
        *case["required_skills"][1:],
        case["required_skills"][0],
    ]
    dispatch_cases.append(("dispatch-meta-skill-not-first", case))
    case = copy.deepcopy(dispatch_template)
    case["return_contract"]["required_result_fields"].remove("verdict")
    dispatch_cases.append(("dispatch-missing-result-field", case))
    case = copy.deepcopy(dispatch_template)
    case["return_contract"]["required_result_fields"].remove(
        "advisory_disposition"
    )
    dispatch_cases.append(("dispatch-missing-advisory-disposition", case))
    case = copy.deepcopy(dispatch_template)
    case["subject_fingerprint"] = "not-a-hash"
    dispatch_cases.append(("dispatch-invalid-subject-fingerprint", case))
    case = copy.deepcopy(dispatch_template)
    case["review_cycle"] = None
    dispatch_cases.append(("dispatch-review-without-cycle", case))
    case = copy.deepcopy(dispatch_template)
    case["review_cycle"]["trigger_run_ids"] = ["prior-review-run"]
    case["review_cycle"]["finding_refs"] = ["prior-review-run:P1-1"]
    dispatch_cases.append(("dispatch-discovery-with-causal-trigger", case))
    case = copy.deepcopy(dispatch_template)
    case["role"] = "implementation"
    dispatch_cases.append(("dispatch-non-review-with-cycle", case))
    case = copy.deepcopy(dispatch_template)
    del case["subject_hygiene"]
    dispatch_cases.append(("dispatch-missing-subject-hygiene", case))

    for label, candidate in [
        *[(name, value) for name, value in handoff_cases],
        *[(name, value) for name, value in dispatch_cases],
    ]:
        schema = dispatch_schema if label.startswith("dispatch-") or label in {
            "child-global-handoff",
            "external-review-transport",
        } else handoff_schema
        try:
            validate_instance(candidate, schema, label)
        except ValidationFailure:
            continue
        print(
            f"orchestration contract self-test failed: {label} unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    def self_test_dispatch(
        dispatch_id: str, role: str, subject: str
    ) -> dict[str, Any]:
        value = copy.deepcopy(dispatch_template)
        value["dispatch_id"] = dispatch_id
        value["parent_orchestration_id"] = template["orchestration_id"]
        value["phase_id"] = template["phase_id"]
        value["slice_id"] = template["slice_id"]
        value["packet_id"] = template["packet_id"]
        value["role"] = role
        value["subject_fingerprint"] = subject
        if role == "review":
            value["required_skills"] = [
                "using-agent-skills",
                "code-review-and-quality",
            ]
        else:
            value["review_cycle"] = None
        return value

    old_subject = "sha256:" + "1" * 64
    mid_subject = "sha256:" + "2" * 64
    repaired_subject = "sha256:" + "3" * 64
    finding_dispatch = self_test_dispatch(
        "self-test-findings-review", "review", old_subject
    )
    finding_dispatch["review_cycle"] = {
        "kind": "discovery",
        "cycle_id": "self-test-discovery",
        "trigger_run_ids": [],
        "finding_refs": [],
    }
    same_cycle_clean_dispatch = self_test_dispatch(
        "self-test-same-cycle-clean-review",
        "review",
        old_subject,
    )
    same_cycle_clean_dispatch["review_cycle"] = copy.deepcopy(
        finding_dispatch["review_cycle"]
    )
    remediation_dispatch = self_test_dispatch(
        "self-test-bad-remediation", "implementation", old_subject
    )
    mid_findings_dispatch = self_test_dispatch(
        "self-test-mid-findings-review", "review", mid_subject
    )
    mid_findings_dispatch["review_cycle"] = {
        "kind": "closure",
        "cycle_id": "self-test-closure",
        "trigger_run_ids": ["findings-review"],
        "finding_refs": ["findings-review-finding"],
    }
    clean_dispatch = self_test_dispatch(
        "self-test-clean-review", "review", repaired_subject
    )
    clean_dispatch["review_cycle"] = {
        "kind": "supplemental_causal",
        "cycle_id": "self-test-supplemental-1",
        "trigger_run_ids": ["mid-findings-review"],
        "finding_refs": ["mid-findings-review-finding"],
    }
    post_clean_subject = "sha256:" + "4" * 64
    post_clean_dispatch = self_test_dispatch(
        "self-test-post-clean-review", "review", post_clean_subject
    )
    post_clean_dispatch["review_cycle"] = {
        "kind": "supplemental_causal",
        "cycle_id": "self-test-supplemental-2",
        "trigger_run_ids": ["clean-review"],
        "finding_refs": [],
    }
    supplemental_two_subject = "sha256:" + "5" * 64
    supplemental_two_dispatch = self_test_dispatch(
        "self-test-supplemental-two-review",
        "review",
        supplemental_two_subject,
    )
    supplemental_two_dispatch["review_cycle"] = {
        "kind": "supplemental_causal",
        "cycle_id": "self-test-supplemental-2",
        "trigger_run_ids": ["clean-review"],
        "finding_refs": ["clean-review-finding"],
    }
    supplemental_three_subject = "sha256:" + "6" * 64
    supplemental_three_dispatch = self_test_dispatch(
        "self-test-supplemental-three-review",
        "review",
        supplemental_three_subject,
    )
    supplemental_three_dispatch["review_cycle"] = {
        "kind": "supplemental_causal",
        "cycle_id": "self-test-supplemental-3",
        "trigger_run_ids": ["supplemental-two-review"],
        "finding_refs": ["supplemental-two-review-finding"],
    }
    dispatch_data = {
        "self-test-findings-review": (
            DISPATCHES_DIR / "self-test-findings-review.json",
            finding_dispatch,
            "a" * 64,
        ),
        "self-test-same-cycle-clean-review": (
            DISPATCHES_DIR / "self-test-same-cycle-clean-review.json",
            same_cycle_clean_dispatch,
            "9" * 64,
        ),
        "self-test-bad-remediation": (
            DISPATCHES_DIR / "self-test-bad-remediation.json",
            remediation_dispatch,
            "b" * 64,
        ),
        "self-test-mid-findings-review": (
            DISPATCHES_DIR / "self-test-mid-findings-review.json",
            mid_findings_dispatch,
            "d" * 64,
        ),
        "self-test-clean-review": (
            DISPATCHES_DIR / "self-test-clean-review.json",
            clean_dispatch,
            "c" * 64,
        ),
        "self-test-post-clean-review": (
            DISPATCHES_DIR / "self-test-post-clean-review.json",
            post_clean_dispatch,
            "e" * 64,
        ),
        "self-test-supplemental-two-review": (
            DISPATCHES_DIR / "self-test-supplemental-two-review.json",
            supplemental_two_dispatch,
            "f" * 64,
        ),
        "self-test-supplemental-three-review": (
            DISPATCHES_DIR / "self-test-supplemental-three-review.json",
            supplemental_three_dispatch,
            "0" * 64,
        ),
    }

    def self_test_run(
        run_id: str,
        dispatch_id: str,
        role: str,
        agent_id: str,
        subject: str,
        result_subject: str,
        verdict: str,
        order: int,
        *,
        final_status: str = "completed",
        remediation_for: list[str] | None = None,
    ) -> dict[str, Any]:
        dispatch_path, dispatch, dispatch_sha256 = dispatch_data[dispatch_id]
        return {
            "run_id": run_id,
            "dispatch_id": dispatch_id,
            "dispatch_ref": dispatch_path.relative_to(REPO_ROOT).as_posix(),
            "dispatch_fingerprint": f"sha256:{dispatch_sha256}",
            "role": role,
            "agent_id": agent_id,
            "agent_type": "default",
            "fresh_context": True,
            "required_skills": dispatch["required_skills"],
            "subject_fingerprint": subject,
            "result_subject_fingerprint": result_subject,
            "review_round": order if role == "review" else None,
            "predecessor_run_id": None if order == 1 else "findings-review",
            "remediation_for_run_ids": remediation_for or [],
            "final_status": final_status,
            "verdict": verdict,
            "finding_refs": [f"{run_id}-finding"] if verdict == "findings" else [],
            "evidence_refs": ["self-test-evidence"],
        }

    findings_run = self_test_run(
        "findings-review",
        "self-test-findings-review",
        "review",
        "reviewer-a",
        old_subject,
        old_subject,
        "findings",
        1,
    )
    same_cycle_clean_run = self_test_run(
        "same-cycle-clean-review",
        "self-test-same-cycle-clean-review",
        "review",
        "reviewer-b",
        old_subject,
        old_subject,
        "clean",
        2,
    )
    same_cycle_clean_run["predecessor_run_id"] = "findings-review"
    mid_findings_run = self_test_run(
        "mid-findings-review",
        "self-test-mid-findings-review",
        "review",
        "reviewer-b",
        mid_subject,
        mid_subject,
        "findings",
        2,
    )
    clean_run = self_test_run(
        "clean-review",
        "self-test-clean-review",
        "review",
        "reviewer-c",
        repaired_subject,
        repaired_subject,
        "clean",
        3,
    )
    clean_run["finding_refs"] = ["HCM-RF-9000"]
    valid_parent_record = copy.deepcopy(template)
    valid_parent_record["status"] = "completed"
    valid_parent_record["stop_reason"] = "completed"
    valid_parent_record["resume"]["execution_target"] = "none"
    valid_parent_record["delegated_runs"] = [
        findings_run,
        mid_findings_run,
        clean_run,
    ]
    valid_parent_record["remediations"] = [
        {
            "remediation_id": "parent-fix-round-1",
            "finding_run_id": "findings-review",
            "owner": "parent_orchestrator",
            "delegated_run_id": None,
            "re_review_run_id": "mid-findings-review",
            "status": "completed",
            "result_subject_fingerprint": mid_subject,
            "evidence_refs": ["parent-remediation-proof-round-1"],
        },
        {
            "remediation_id": "parent-fix-round-2",
            "finding_run_id": "mid-findings-review",
            "owner": "parent_orchestrator",
            "delegated_run_id": None,
            "re_review_run_id": "clean-review",
            "status": "completed",
            "result_subject_fingerprint": repaired_subject,
            "evidence_refs": ["parent-remediation-proof-round-2"],
        }
    ]
    valid_parent_record["findings"] = [
        {
            "finding_id": "findings-review-finding",
            "classification": "local_remediation",
            "severity": "major",
            "priority": "P2",
            "status": "remediated",
            "source_run_id": "findings-review",
            "summary": "First self-test blocking finding.",
            "evidence_refs": ["parent-remediation-proof-round-1"],
        },
        {
            "finding_id": "mid-findings-review-finding",
            "classification": "local_remediation",
            "severity": "major",
            "priority": "P2",
            "status": "remediated",
            "source_run_id": "mid-findings-review",
            "summary": "Second self-test blocking finding.",
            "evidence_refs": ["parent-remediation-proof-round-2"],
        },
        {
            "finding_id": "HCM-RF-9000",
            "classification": "future_program",
            "severity": "warning",
            "priority": "P3",
            "status": "inventoried",
            "source_run_id": "clean-review",
            "summary": "Self-test advisory retained without blocking CLEAN.",
            "evidence_refs": ["advisory-inventory-proof"],
        },
    ]
    valid_parent_record["reviewed_state"]["subject_fingerprint"] = repaired_subject
    valid_parent_record["reviewed_state"]["subject_manifest_ref"] = clean_run[
        "dispatch_ref"
    ]
    valid_parent_record["repo_state"]["head"] = valid_parent_record[
        "reviewed_state"
    ]["baseline_head"]
    with tempfile.TemporaryDirectory(prefix="hcm-review-inventory-") as temp_dir:
        inventory_header = (
            "| Finding ID | Priority | Status | Comparison key | Summary | "
            "Source | Evidence refs | Affected scope | Disposition | Target | "
            "Occurrences | Resolution refs |"
        )
        inventory_separator = (
            "|---|---|---|---|---|---|---|---|---|---|---|---|"
        )
        valid_inventory_row = (
            "| HCM-RF-9000 | P3 | accepted | "
            r"validator \| inventory \| registration \| self-test"
            " | Self-test advisory | self-test review | self-test evidence | "
            "handoff validator | retained for parser proof | unassigned | "
            "none | none |"
        )
        self_test_inventory_path = Path(temp_dir) / "inventory.md"
        self_test_inventory_path.write_text(
            "## Inventory\n\n"
            f"{inventory_header}\n"
            f"{inventory_separator}\n"
            f"{valid_inventory_row}\n"
        )
        self_test_inventory_entries = load_review_inventory_entries(
            self_test_inventory_path
        )
        invalid_inventory_rows = {
            "short": "| HCM-RF-9000 | P3 |",
            "empty-field": valid_inventory_row.replace(
                "Self-test advisory", ""
            ),
            "malformed-id": valid_inventory_row.replace(
                "HCM-RF-9000", "HCM-RF-90"
            ),
            "arabic-indic-id": valid_inventory_row.replace(
                "HCM-RF-9000", "HCM-RF-١٢٣٤"
            ),
            "fullwidth-id": valid_inventory_row.replace(
                "HCM-RF-9000", "HCM-RF-１２３４"
            ),
            "superscript-id": valid_inventory_row.replace(
                "HCM-RF-9000", "HCM-RF-¹²³⁴"
            ),
            "duplicate-id": f"{valid_inventory_row}\n{valid_inventory_row}",
            "invalid-status": valid_inventory_row.replace(
                "| accepted |", "| deferred |"
            ),
            "unescaped-comparison-key": valid_inventory_row.replace(
                r"validator \| inventory \| registration \| self-test",
                "validator | inventory | registration | self-test",
            ),
        }
        for label, rows in invalid_inventory_rows.items():
            invalid_path = Path(temp_dir) / f"{label}.md"
            invalid_path.write_text(
                "## Inventory\n\n"
                f"{inventory_header}\n"
                f"{inventory_separator}\n"
                f"{rows}\n",
                encoding="utf-8",
            )
            try:
                load_review_inventory_entries(invalid_path)
            except ValidationFailure:
                pass
            else:
                print(
                    "orchestration contract self-test failed: malformed "
                    f"inventory case {label!r} unexpectedly parsed",
                    file=sys.stderr,
                )
                return 1

        later_section_path = Path(temp_dir) / "later-section.md"
        later_section_path.write_text(
            "## Inventory\n\n"
            "No unresolved findings.\n\n"
            "## Appendix\n\n"
            f"{inventory_header}\n"
            f"{inventory_separator}\n"
            f"{valid_inventory_row}\n",
            encoding="utf-8",
        )
        duplicate_section_path = Path(temp_dir) / "duplicate-section.md"
        duplicate_section_path.write_text(
            "## Inventory\n\n"
            f"{inventory_header}\n"
            f"{inventory_separator}\n"
            f"{valid_inventory_row}\n\n"
            "## Inventory\n\n"
            f"{inventory_header}\n"
            f"{inventory_separator}\n",
            encoding="utf-8",
        )
        for label, invalid_path in {
            "later-section-table": later_section_path,
            "duplicate-inventory-heading": duplicate_section_path,
        }.items():
            try:
                load_review_inventory_entries(invalid_path)
            except ValidationFailure:
                pass
            else:
                print(
                    "orchestration contract self-test failed: canonical "
                    f"Inventory boundary case {label!r} unexpectedly parsed",
                    file=sys.stderr,
                )
                return 1

        orphan_path = Path(temp_dir) / "orphan.md"
        orphan_path.write_text(
            f"{valid_inventory_row}\n\n"
            "## Inventory\n\n"
            f"{inventory_header}\n"
            f"{inventory_separator}\n"
        )
        if load_review_inventory_entries(orphan_path):
            print(
                "orchestration contract self-test failed: orphan inventory "
                "row unexpectedly parsed",
                file=sys.stderr,
            )
            return 1

        priority_mismatch_path = Path(temp_dir) / "priority-mismatch.md"
        priority_mismatch_path.write_text(
            "## Inventory\n\n"
            f"{inventory_header}\n"
            f"{inventory_separator}\n"
            f"{valid_inventory_row.replace('| P3 |', '| P4 |')}\n"
        )
        priority_mismatch_entries = load_review_inventory_entries(
            priority_mismatch_path
        )
    validate_instance(valid_parent_record, handoff_schema, "parent remediation positive")
    validate_v1_2_semantics(
        valid_parent_record,
        RECORDS_DIR / "self-test-parent-remediation.json",
        set(),
        dispatch_data,
        review_inventory_entries=self_test_inventory_entries,
    )

    same_cycle_laundering_record = copy.deepcopy(valid_parent_record)
    same_cycle_laundering_record["delegated_runs"] = [
        findings_run,
        same_cycle_clean_run,
    ]
    same_cycle_laundering_record["remediations"] = [
        {
            "remediation_id": "same-cycle-fix",
            "finding_run_id": "findings-review",
            "owner": "parent_orchestrator",
            "delegated_run_id": None,
            "re_review_run_id": "same-cycle-clean-review",
            "status": "completed",
            "result_subject_fingerprint": old_subject,
            "evidence_refs": ["same-cycle-remediation-proof"],
        }
    ]
    same_cycle_laundering_record["findings"] = [
        {
            "finding_id": "findings-review-finding",
            "classification": "local_remediation",
            "severity": "major",
            "priority": "P2",
            "status": "remediated",
            "source_run_id": "findings-review",
            "summary": "Same-cycle laundering must fail.",
            "evidence_refs": ["same-cycle-remediation-proof"],
        }
    ]
    same_cycle_laundering_record["reviewed_state"][
        "subject_fingerprint"
    ] = old_subject
    same_cycle_laundering_record["reviewed_state"]["subject_manifest_ref"] = (
        same_cycle_clean_run["dispatch_ref"]
    )
    try:
        validate_v1_2_semantics(
            same_cycle_laundering_record,
            RECORDS_DIR / "self-test-same-cycle-remediation.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: same-cycle findings-to-clean "
            "remediation laundering unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    unchanged_cycle_dispatch_data = copy.deepcopy(dispatch_data)
    unchanged_cycle_dispatch_data["self-test-mid-findings-review"][1][
        "subject_fingerprint"
    ] = old_subject
    unchanged_cycle_record = copy.deepcopy(valid_parent_record)
    unchanged_cycle_record["delegated_runs"][1][
        "subject_fingerprint"
    ] = old_subject
    unchanged_cycle_record["delegated_runs"][1][
        "result_subject_fingerprint"
    ] = old_subject
    unchanged_cycle_record["remediations"][0][
        "result_subject_fingerprint"
    ] = old_subject
    try:
        validate_v1_2_semantics(
            unchanged_cycle_record,
            RECORDS_DIR / "self-test-unchanged-causal-cycle.json",
            set(),
            unchanged_cycle_dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: adjacent causal cycles "
            "with an unchanged subject fingerprint unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    untyped_dispatch_data = copy.deepcopy(dispatch_data)
    for _, dispatch, _ in untyped_dispatch_data.values():
        if dispatch["role"] == "review":
            dispatch["schema_version"] = "1.2"
            del dispatch["review_cycle"]
            del dispatch["subject_hygiene"]
    try:
        validate_v1_2_semantics(
            valid_parent_record,
            RECORDS_DIR / "self-test-new-untyped-review-lineage.json",
            set(),
            untyped_dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: newly authored all-untyped "
            "review lineage unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    new_v1_2_record = copy.deepcopy(valid_parent_record)
    new_v1_2_record["schema_version"] = "1.2"
    try:
        validate_v1_2_semantics(
            new_v1_2_record,
            RECORDS_DIR / "self-test-new-v1-2-closeout.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: newly authored "
            "handoff-record v1.2 unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    post_clean_run = self_test_run(
        "post-clean-review",
        "self-test-post-clean-review",
        "review",
        "reviewer-d",
        post_clean_subject,
        post_clean_subject,
        "clean",
        4,
    )
    post_clean_run["predecessor_run_id"] = "clean-review"
    invalid_post_clean_record = copy.deepcopy(valid_parent_record)
    invalid_post_clean_record["delegated_runs"].append(post_clean_run)
    invalid_post_clean_record["reviewed_state"]["subject_fingerprint"] = (
        post_clean_subject
    )
    invalid_post_clean_record["reviewed_state"]["subject_manifest_ref"] = (
        post_clean_run["dispatch_ref"]
    )
    try:
        validate_v1_2_semantics(
            invalid_post_clean_record,
            RECORDS_DIR / "self-test-post-clean-supplemental-review.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: supplemental review "
            "triggered by CLEAN unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    max_budget_record = copy.deepcopy(valid_parent_record)
    max_budget_record["delegated_runs"][-1]["verdict"] = "findings"
    max_budget_record["delegated_runs"][-1]["finding_refs"].append(
        "clean-review-finding"
    )
    max_budget_record["findings"].append(
        {
            "finding_id": "clean-review-finding",
            "classification": "proof_gap",
            "severity": "major",
            "priority": "P2",
            "status": "remediated",
            "source_run_id": "clean-review",
            "summary": "First supplemental review exposes a causal P2.",
            "evidence_refs": ["supplemental-two-remediation-proof"],
        }
    )
    supplemental_two_run = self_test_run(
        "supplemental-two-review",
        "self-test-supplemental-two-review",
        "review",
        "reviewer-d",
        supplemental_two_subject,
        supplemental_two_subject,
        "clean",
        4,
    )
    supplemental_two_run["predecessor_run_id"] = "clean-review"
    max_budget_record["delegated_runs"].append(supplemental_two_run)
    max_budget_record["remediations"].append(
        {
            "remediation_id": "parent-fix-supplemental-2",
            "finding_run_id": "clean-review",
            "owner": "parent_orchestrator",
            "delegated_run_id": None,
            "re_review_run_id": "supplemental-two-review",
            "status": "completed",
            "result_subject_fingerprint": supplemental_two_subject,
            "evidence_refs": ["supplemental-two-remediation-proof"],
        }
    )
    max_budget_record["reviewed_state"]["subject_fingerprint"] = (
        supplemental_two_subject
    )
    max_budget_record["reviewed_state"]["subject_manifest_ref"] = (
        supplemental_two_run["dispatch_ref"]
    )
    validate_instance(
        max_budget_record,
        handoff_schema,
        "two supplemental causal cycles schema shape",
    )
    validate_v1_2_semantics(
        max_budget_record,
        RECORDS_DIR / "self-test-two-supplemental-reviews.json",
        set(),
        dispatch_data,
        review_inventory_entries=self_test_inventory_entries,
    )

    over_budget_record = copy.deepcopy(max_budget_record)
    over_budget_record["delegated_runs"][-1]["verdict"] = "findings"
    over_budget_record["delegated_runs"][-1]["finding_refs"] = [
        "supplemental-two-review-finding"
    ]
    supplemental_three_run = self_test_run(
        "supplemental-three-review",
        "self-test-supplemental-three-review",
        "review",
        "reviewer-e",
        supplemental_three_subject,
        supplemental_three_subject,
        "clean",
        5,
    )
    supplemental_three_run["predecessor_run_id"] = "supplemental-two-review"
    supplemental_three_run["finding_refs"] = []
    over_budget_record["delegated_runs"].append(supplemental_three_run)
    over_budget_record["findings"].append(
        {
            "finding_id": "supplemental-two-review-finding",
            "classification": "proof_gap",
            "severity": "major",
            "priority": "P2",
            "status": "remediated",
            "source_run_id": "supplemental-two-review",
            "summary": "Second supplemental review exposes another causal P2.",
            "evidence_refs": ["supplemental-three-remediation-proof"],
        }
    )
    over_budget_record["remediations"].append(
        {
            "remediation_id": "parent-fix-supplemental-3",
            "finding_run_id": "supplemental-two-review",
            "owner": "parent_orchestrator",
            "delegated_run_id": None,
            "re_review_run_id": "supplemental-three-review",
            "status": "completed",
            "result_subject_fingerprint": supplemental_three_subject,
            "evidence_refs": ["supplemental-three-remediation-proof"],
        }
    )
    over_budget_record["reviewed_state"]["subject_fingerprint"] = (
        supplemental_three_subject
    )
    over_budget_record["reviewed_state"]["subject_manifest_ref"] = (
        supplemental_three_run["dispatch_ref"]
    )
    try:
        validate_v1_2_semantics(
            over_budget_record,
            RECORDS_DIR / "self-test-excess-supplemental-review.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: third supplemental "
            "causal review unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    fabricated_inventory_record = copy.deepcopy(valid_parent_record)
    fabricated_inventory_record["delegated_runs"][-1]["finding_refs"] = [
        "HCM-RF-9999"
    ]
    fabricated_inventory_record["findings"][-1]["finding_id"] = "HCM-RF-9999"
    validate_instance(
        fabricated_inventory_record,
        handoff_schema,
        "fabricated inventoried advisory schema shape",
    )
    try:
        validate_v1_2_semantics(
            fabricated_inventory_record,
            RECORDS_DIR / "self-test-fabricated-inventoried-advisory.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: fabricated inventoried "
            "advisory unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    try:
        validate_v1_2_semantics(
            valid_parent_record,
            RECORDS_DIR / "self-test-inventory-priority-mismatch.json",
            set(),
            dispatch_data,
            review_inventory_entries=priority_mismatch_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: mismatched inventory "
            "priority unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    blocking_clean_record = copy.deepcopy(valid_parent_record)
    blocking_clean_record["delegated_runs"][-1]["finding_refs"].append(
        "clean-review-blocking-finding"
    )
    blocking_clean_record["findings"].append(
        {
            "finding_id": "clean-review-blocking-finding",
            "classification": "proof_gap",
            "severity": "major",
            "priority": "P2",
            "status": "open",
            "source_run_id": "clean-review",
            "summary": "A clean verdict must not carry an unresolved P2.",
            "evidence_refs": ["negative-self-test"],
        }
    )
    validate_instance(
        blocking_clean_record,
        handoff_schema,
        "clean review with unresolved P2 schema shape",
    )
    try:
        validate_v1_2_semantics(
            blocking_clean_record,
            RECORDS_DIR / "self-test-clean-with-blocking-finding.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: clean review with "
            "unresolved P2 unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    cross_run_blocking_record = copy.deepcopy(valid_parent_record)
    cross_run_blocking_record["delegated_runs"][-1]["finding_refs"].append(
        "findings-review-finding"
    )
    validate_instance(
        cross_run_blocking_record,
        handoff_schema,
        "clean review with cross-run P2 reference schema shape",
    )
    try:
        validate_v1_2_semantics(
            cross_run_blocking_record,
            RECORDS_DIR / "self-test-clean-with-cross-run-blocking-ref.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: clean review with "
            "cross-run P2 reference unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    wrong_primary_record = copy.deepcopy(valid_parent_record)
    wrong_primary_record["repo_state"]["head"] = "different-unreviewed-commit"
    try:
        validate_v1_2_semantics(
            wrong_primary_record,
            RECORDS_DIR / "self-test-wrong-primary-commit.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: reviewed baseline and "
            "recorded primary commit mismatch unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    bad_record = copy.deepcopy(template)
    bad_record["status"] = "completed"
    bad_record["stop_reason"] = "completed"
    bad_record["resume"]["execution_target"] = "none"
    bad_run = self_test_run(
        "bad-remediation",
        "self-test-bad-remediation",
        "implementation",
        "fixer",
        old_subject,
        repaired_subject,
        "not_applicable",
        2,
        final_status="failed",
        remediation_for=["findings-review"],
    )
    bad_record["delegated_runs"] = [findings_run, bad_run, clean_run]
    bad_record["remediations"] = [
        {
            "remediation_id": "bad-delegated-fix",
            "finding_run_id": "findings-review",
            "owner": "delegated_run",
            "delegated_run_id": "bad-remediation",
            "re_review_run_id": "clean-review",
            "status": "completed",
            "result_subject_fingerprint": repaired_subject,
            "evidence_refs": ["bad-remediation-proof"],
        }
    ]
    bad_record["reviewed_state"]["subject_fingerprint"] = repaired_subject
    bad_record["reviewed_state"]["subject_manifest_ref"] = clean_run["dispatch_ref"]
    bad_record["repo_state"]["head"] = bad_record["reviewed_state"]["baseline_head"]
    try:
        validate_v1_2_semantics(
            bad_record,
            RECORDS_DIR / "self-test-failed-remediation.json",
            set(),
            dispatch_data,
            review_inventory_entries=self_test_inventory_entries,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: failed/wrong-role "
            "remediation unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    manifest_dispatch = copy.deepcopy(dispatch_template)
    manifest_path = "docs/specs/handbook-contract-membrane/00-README.md"
    manifest_sha256 = hashlib.sha256((REPO_ROOT / manifest_path).read_bytes()).hexdigest()
    aggregate = "sha256:" + hashlib.sha256(
        f"{manifest_path}\0{manifest_sha256}\n".encode()
    ).hexdigest()
    manifest_dispatch["subject_manifest"]["entries"] = [
        {"path": manifest_path, "sha256": manifest_sha256}
    ]
    manifest_dispatch["subject_manifest"]["aggregate_fingerprint"] = aggregate
    manifest_dispatch["subject_fingerprint"] = aggregate
    validate_subject_manifest(
        manifest_dispatch, INTERNAL_DISPATCH_TEMPLATE_PATH
    )
    bad_manifest = copy.deepcopy(manifest_dispatch)
    bad_manifest["subject_manifest"]["entries"][0]["sha256"] = "f" * 64
    try:
        validate_subject_manifest(bad_manifest, INTERNAL_DISPATCH_TEMPLATE_PATH)
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: changed subject file "
            "unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    unsafe_manifest_paths = {
        "windows-drive": r"C:\Windows\win.ini",
        "windows-unc": r"\\server\share\subject.txt",
        "backslash-traversal": r"..\outside.txt",
        "posix-traversal": "../outside.txt",
        "absolute": "/absolute.txt",
        "literal-dot": ".",
        "embedded-nul": "a\0b",
        "repeated-separator": "a//b.txt",
        "current-directory": "a/./b.txt",
        "embedded-traversal": "a/../b.txt",
        "trailing-separator": "a/",
    }
    for label, unsafe_path in unsafe_manifest_paths.items():
        unsafe_manifest = copy.deepcopy(manifest_dispatch)
        unsafe_manifest["subject_manifest"]["entries"][0]["path"] = unsafe_path
        unsafe_aggregate = "sha256:" + hashlib.sha256(
            f"{unsafe_path}\0{manifest_sha256}\n".encode()
        ).hexdigest()
        unsafe_manifest["subject_manifest"]["aggregate_fingerprint"] = (
            unsafe_aggregate
        )
        unsafe_manifest["subject_fingerprint"] = unsafe_aggregate
        try:
            validate_instance(
                unsafe_manifest,
                dispatch_schema,
                f"unsafe manifest path {label}",
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: non-repository "
                f"manifest path {label!r} unexpectedly passed schema validation",
                file=sys.stderr,
            )
            return 1
        try:
            validate_subject_manifest(
                unsafe_manifest, INTERNAL_DISPATCH_TEMPLATE_PATH
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: non-repository "
                f"manifest path {label!r} unexpectedly passed semantic validation",
                file=sys.stderr,
            )
            return 1

    with tempfile.TemporaryDirectory(prefix="hcm-manifest-whitespace-") as temp_dir:
        temp_repo = Path(temp_dir)
        temp_dispatches = temp_repo / "dispatches"
        temp_dispatches.mkdir()
        clean_path = "subjects/a-clean.md"
        dirty_path = "subjects/b-dirty.md"
        clean_subject = temp_repo / clean_path
        dirty_subject = temp_repo / dirty_path
        clean_subject.parent.mkdir()

        for label, dirty_bytes in {
            "space": b"later manifest entry has trailing space \n",
            "tab": b"later manifest entry has trailing tab\t\n",
        }.items():
            clean_subject.write_bytes(b"clean first manifest entry\n")
            dirty_subject.write_bytes(dirty_bytes)
            entries = [
                {
                    "path": subject_path,
                    "sha256": hashlib.sha256(
                        (temp_repo / subject_path).read_bytes()
                    ).hexdigest(),
                }
                for subject_path in [clean_path, dirty_path]
            ]
            aggregate = "sha256:" + hashlib.sha256(
                "".join(
                    f"{entry['path']}\0{entry['sha256']}\n"
                    for entry in entries
                ).encode()
            ).hexdigest()
            whitespace_dispatch = copy.deepcopy(dispatch_template)
            whitespace_dispatch["dispatch_id"] = (
                f"self-test-manifest-whitespace-{label}"
            )
            whitespace_dispatch["subject_manifest"]["entries"] = entries
            whitespace_dispatch["subject_manifest"][
                "aggregate_fingerprint"
            ] = aggregate
            whitespace_dispatch["subject_fingerprint"] = aggregate
            dispatch_relative = (
                f"dispatches/{whitespace_dispatch['dispatch_id']}.json"
            )
            (temp_repo / dispatch_relative).write_text(
                json.dumps(whitespace_dispatch) + "\n",
                encoding="utf-8",
            )
            if (
                verify_dispatch_file(
                    dispatch_relative,
                    repo_root=temp_repo,
                    dispatches_dir=temp_dispatches,
                    quiet=True,
                )
                != 1
            ):
                print(
                    "orchestration contract self-test failed: later-entry "
                    f"trailing {label} unexpectedly passed public dispatch "
                    "verification",
                    file=sys.stderr,
                )
                return 1

        clean_subject.write_bytes(b"clean first manifest entry\n")
        dirty_subject.write_bytes(b"clean later manifest entry\n")
        predecessor_entries = [
            {
                "path": subject_path,
                "sha256": hashlib.sha256(
                    (temp_repo / subject_path).read_bytes()
                ).hexdigest(),
            }
            for subject_path in [clean_path, dirty_path]
        ]
        predecessor_aggregate = "sha256:" + hashlib.sha256(
            "".join(
                f"{entry['path']}\0{entry['sha256']}\n"
                for entry in predecessor_entries
            ).encode()
        ).hexdigest()
        predecessor_dispatch = copy.deepcopy(dispatch_template)
        predecessor_dispatch["schema_version"] = "1.2"
        predecessor_dispatch["dispatch_id"] = "self-test-new-v1-2-dispatch"
        del predecessor_dispatch["review_cycle"]
        del predecessor_dispatch["subject_hygiene"]
        predecessor_dispatch["subject_manifest"]["entries"] = predecessor_entries
        predecessor_dispatch["subject_manifest"][
            "aggregate_fingerprint"
        ] = predecessor_aggregate
        predecessor_dispatch["subject_fingerprint"] = predecessor_aggregate
        predecessor_relative = (
            f"dispatches/{predecessor_dispatch['dispatch_id']}.json"
        )
        (temp_repo / predecessor_relative).write_text(
            json.dumps(predecessor_dispatch) + "\n",
            encoding="utf-8",
        )
        if (
            verify_dispatch_file(
                predecessor_relative,
                repo_root=temp_repo,
                dispatches_dir=temp_dispatches,
                quiet=True,
            )
            != 1
        ):
            print(
                "orchestration contract self-test failed: newly authored "
                "predecessor dispatch unexpectedly passed execution verification",
                file=sys.stderr,
            )
            return 1

    with tempfile.TemporaryDirectory(prefix="hcm-two-commit-") as temp_dir:
        temp_repo = Path(temp_dir)
        subprocess.run(["git", "init", "-q"], cwd=temp_repo, check=True)
        subprocess.run(
            ["git", "config", "user.email", "self-test@example.invalid"],
            cwd=temp_repo,
            check=True,
        )
        subprocess.run(
            ["git", "config", "user.name", "HCM self-test"],
            cwd=temp_repo,
            check=True,
        )
        ledger_path = "docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
        subject_path = temp_repo / ledger_path
        subject_path.parent.mkdir(parents=True)
        subject_path.write_bytes(b'{"handoff_id":"pre-closeout"}\n')
        subprocess.run(["git", "add", ledger_path], cwd=temp_repo, check=True)
        subprocess.run(
            ["git", "commit", "-q", "-m", "reviewed primary state"],
            cwd=temp_repo,
            check=True,
        )
        baseline_head = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=temp_repo, text=True
        ).strip()
        ledger_sha256 = hashlib.sha256(subject_path.read_bytes()).hexdigest()
        ledger_aggregate = "sha256:" + hashlib.sha256(
            f"{ledger_path}\0{ledger_sha256}\n".encode()
        ).hexdigest()
        two_commit_dispatch = copy.deepcopy(dispatch_template)
        two_commit_dispatch["subject_manifest"]["entries"] = [
            {"path": ledger_path, "sha256": ledger_sha256}
        ]
        two_commit_dispatch["subject_manifest"]["aggregate_fingerprint"] = (
            ledger_aggregate
        )
        two_commit_dispatch["subject_fingerprint"] = ledger_aggregate
        subject_path.write_bytes(
            b'{"handoff_id":"pre-closeout"}\n'
            b'{"handoff_id":"parent-closeout"}\n'
        )
        subprocess.run(["git", "add", ledger_path], cwd=temp_repo, check=True)
        subprocess.run(
            ["git", "commit", "-q", "-m", "mechanical closeout"],
            cwd=temp_repo,
            check=True,
        )
        validate_subject_manifest(
            two_commit_dispatch,
            INTERNAL_DISPATCH_TEMPLATE_PATH,
            baseline_head=baseline_head,
            repo_root=temp_repo,
        )
        try:
            validate_subject_manifest(
                two_commit_dispatch,
                INTERNAL_DISPATCH_TEMPLATE_PATH,
                verify_live_files=True,
                repo_root=temp_repo,
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: post-closeout ledger "
                "unexpectedly matched the reviewed primary state",
                file=sys.stderr,
            )
            return 1

    print(
        "orchestration contract self-test passed: child handoff, non-default/fresh "
        "agents, stop/status/resume mismatches, incomplete completion, capability "
        "mislabeling, missing skills/results/advisory disposition, invalid "
        "fingerprints, missing/invalid typed review cycles or subject hygiene, "
        "forbidden global "
        "handoff, external transport, and failed/wrong-role remediation all fail "
        "closed; clean review with direct or cross-run P1/P2, malformed/orphan "
        "inventory rows, cross-section/duplicate Inventory tables, non-ASCII "
        "digit IDs, fabricated IDs, priority mismatch, and non-repository "
        "manifest paths fail closed; new predecessor dispatch/record use, "
        "all-untyped lineage, same/unchanged-cycle remediation laundering, "
        "post-CLEAN and over-budget causal review cycles, plus later-entry "
        "trailing spaces/tabs fail closed; "
        "chained findings/remediation/re-review, exactly two supplemental "
        "causal cycles, clean review with a complete durable P3/P4 inventory "
        "row, direct parent remediation, "
        "reviewed-baseline/primary-commit identity, and two-commit ledger "
        "mutation validate"
    )
    return 0


def verify_dispatch_file(
    dispatch_argument: str,
    *,
    repo_root: Path = REPO_ROOT,
    dispatches_dir: Path = DISPATCHES_DIR,
    quiet: bool = False,
) -> int:
    try:
        dispatch_path = (repo_root / dispatch_argument).resolve()
        dispatch_path.relative_to(dispatches_dir.resolve())
        if dispatch_path.suffix != ".json" or not dispatch_path.is_file():
            raise ValidationFailure(
                f"{dispatch_argument}: expected an existing JSON dispatch"
            )
        dispatch = load_json(dispatch_path)
        version = dispatch.get("schema_version")
        schema_path = INTERNAL_DISPATCH_SCHEMA_PATHS.get(version)
        if schema_path is None:
            raise ValidationFailure(
                f"{dispatch_path}: unsupported internal dispatch "
                f"schema_version {version!r}"
            )
        if version != "1.3":
            raise ValidationFailure(
                f"{dispatch_path}: internal-dispatch {version} is immutable "
                "predecessor evidence and cannot be executed"
            )
        schema = load_json(schema_path)
        validate_instance(dispatch, schema, str(dispatch_path))
        validate_subject_manifest(
            dispatch,
            dispatch_path,
            verify_live_files=True,
            repo_root=repo_root,
        )
    except (OSError, ValueError, KeyError, ValidationFailure) as error:
        if not quiet:
            print(f"dispatch verification failed: {error}", file=sys.stderr)
        return 1
    if not quiet:
        print(
            "dispatch verification passed: "
            f"{dispatch['dispatch_id']} {dispatch['subject_fingerprint']}"
        )
    return 0


def main() -> int:
    historical_fixture_validation = sys.argv[1:] == [
        "--validate-historical-fixture"
    ]
    if sys.argv[1:] == ["--self-test-v1-admission"]:
        return run_historical_v1_0_admission_self_test()
    if sys.argv[1:] == ["--self-test-orchestration-contract"]:
        return run_orchestration_contract_self_test()
    if len(sys.argv) == 3 and sys.argv[1] == "--verify-dispatch":
        return verify_dispatch_file(sys.argv[2])
    if sys.argv[1:] and not historical_fixture_validation:
        print(
            "usage: validate_handoffs.py "
            "[--self-test-v1-admission|--self-test-orchestration-contract|"
            "--verify-dispatch <repo-relative-json-path>]",
            file=sys.stderr,
        )
        return 2

    try:
        record_schemas = {
            version: load_json(path) for version, path in RECORD_SCHEMA_PATHS.items()
        }
        internal_dispatch_schemas = {
            version: load_json(path)
            for version, path in INTERNAL_DISPATCH_SCHEMA_PATHS.items()
        }
        ledger_schema = load_json(LEDGER_SCHEMA_PATH)
        for path, schema in [
            *[(RECORD_SCHEMA_PATHS[version], schema) for version, schema in record_schemas.items()],
            *[
                (INTERNAL_DISPATCH_SCHEMA_PATHS[version], schema)
                for version, schema in internal_dispatch_schemas.items()
            ],
            (LEDGER_SCHEMA_PATH, ledger_schema),
        ]:
            try:
                Draft202012Validator.check_schema(schema)
            except Exception as error:
                raise ValidationFailure(f"{path}: invalid Draft 2020-12 schema: {error}") from error

        template = load_json(TEMPLATE_PATH)
        template_version = template.get("schema_version")
        if template_version != "1.3":
            raise ValidationFailure(
                f"{TEMPLATE_PATH}: new-record template must route to schema_version 1.3"
            )
        validate_instance(template, record_schemas[template_version], str(TEMPLATE_PATH))

        internal_dispatch_template = load_json(INTERNAL_DISPATCH_TEMPLATE_PATH)
        internal_dispatch_template_version = internal_dispatch_template.get(
            "schema_version"
        )
        if internal_dispatch_template_version != "1.3":
            raise ValidationFailure(
                f"{INTERNAL_DISPATCH_TEMPLATE_PATH}: current internal dispatch "
                "template must route to schema_version 1.3"
            )
        validate_instance(
            internal_dispatch_template,
            internal_dispatch_schemas[internal_dispatch_template_version],
            str(INTERNAL_DISPATCH_TEMPLATE_PATH),
        )
        validate_legacy_dispatch_admission()
        present_internal_v1_0_names: set[str] = set()
        dispatches: dict[str, tuple[Path, dict[str, Any], str]] = {}
        for path in sorted(DISPATCHES_DIR.glob("*.json")):
            dispatch = load_json(path)
            version = dispatch.get("schema_version")
            if version not in internal_dispatch_schemas:
                raise ValidationFailure(
                    f"{path}: unsupported internal dispatch schema_version {version!r}"
                )
            validate_instance(dispatch, internal_dispatch_schemas[version], str(path))
            if version == "1.0":
                present_internal_v1_0_names.add(path.name)
                validate_historical_internal_dispatch_v1_0_admission(dispatch, path)
            else:
                validate_subject_manifest(dispatch, path)
            dispatch_id = dispatch["dispatch_id"]
            if dispatch_id != path.stem:
                raise ValidationFailure(
                    f"{path}: dispatch_id {dispatch_id!r} does not match filename"
                )
            if dispatch_id in dispatches:
                raise ValidationFailure(f"duplicate internal dispatch_id: {dispatch_id}")
            dispatches[dispatch_id] = (
                path,
                dispatch,
                hashlib.sha256(path.read_bytes()).hexdigest(),
            )
        expected_internal_v1_0_names = set(
            HISTORICAL_INTERNAL_DISPATCH_V1_0_ADMISSION
        )
        if present_internal_v1_0_names != expected_internal_v1_0_names:
            missing = sorted(expected_internal_v1_0_names - present_internal_v1_0_names)
            extra = sorted(present_internal_v1_0_names - expected_internal_v1_0_names)
            raise ValidationFailure(
                "historical internal-dispatch v1.0 filename set mismatch: "
                f"missing={missing}, extra={extra}"
            )
        if not historical_fixture_validation:
            validate_immutable_predecessor_dispatch_corpus(dispatches)

        record_paths = sorted(RECORDS_DIR.glob("*.json"))
        loaded_records = [(path, load_json(path)) for path in record_paths]
        if not historical_fixture_validation:
            validate_immutable_v1_2_record_corpus(loaded_records)
            present_pre_v1_3_closeouts = {
                path.name
                for path, _ in loaded_records
                if path.name in PRE_V1_3_DISPATCH_CLOSEOUT_ADMISSION
            }
            expected_pre_v1_3_closeouts = set(
                PRE_V1_3_DISPATCH_CLOSEOUT_ADMISSION
            )
            if present_pre_v1_3_closeouts != expected_pre_v1_3_closeouts:
                missing = sorted(
                    expected_pre_v1_3_closeouts - present_pre_v1_3_closeouts
                )
                extra = sorted(
                    present_pre_v1_3_closeouts - expected_pre_v1_3_closeouts
                )
                raise ValidationFailure(
                    "pre-v1.3-dispatch closeout admission mismatch: "
                    f"missing={missing}, extra={extra}"
                )
            for path, record in loaded_records:
                if path.name in PRE_V1_3_DISPATCH_CLOSEOUT_ADMISSION:
                    validate_pre_v1_3_dispatch_closeout_admission(record, path)
        present_v1_0_filenames = {
            path.name
            for path, record in loaded_records
            if record.get("schema_version") == "1.0"
        }
        expected_v1_0_filenames = set(HISTORICAL_V1_0_ADMISSION)
        if present_v1_0_filenames != expected_v1_0_filenames:
            missing = sorted(expected_v1_0_filenames - present_v1_0_filenames)
            extra = sorted(present_v1_0_filenames - expected_v1_0_filenames)
            raise ValidationFailure(
                "historical v1.0 canonical filename set mismatch: "
                f"missing={missing}, extra={extra}"
            )
        present_v1_1_filenames = {
            path.name
            for path, record in loaded_records
            if record.get("schema_version") == "1.1"
        }
        expected_v1_1_filenames = set(HISTORICAL_V1_1_ADMISSION)
        if present_v1_1_filenames != expected_v1_1_filenames:
            missing = sorted(expected_v1_1_filenames - present_v1_1_filenames)
            extra = sorted(present_v1_1_filenames - expected_v1_1_filenames)
            raise ValidationFailure(
                "historical v1.1 canonical filename set mismatch: "
                f"missing={missing}, extra={extra}"
            )

        records: list[tuple[Path, dict[str, Any]]] = []
        seen_record_ids: set[str] = set()
        for path, record in loaded_records:
            version = record.get("schema_version")
            if version not in record_schemas:
                raise ValidationFailure(
                    f"{path}: unsupported handoff schema_version {version!r}"
                )
            if path.name in HISTORICAL_V1_0_ADMISSION and version != "1.0":
                raise ValidationFailure(
                    f"{path}: admitted historical record must retain schema_version 1.0"
                )
            if version == "1.0" or path.name in HISTORICAL_V1_0_ADMISSION:
                validate_historical_v1_0_admission(record, path)
            if path.name in HISTORICAL_V1_1_ADMISSION and version != "1.1":
                raise ValidationFailure(
                    f"{path}: admitted historical record must retain schema_version 1.1"
                )
            if version == "1.1" or path.name in HISTORICAL_V1_1_ADMISSION:
                validate_historical_v1_1_admission(record, path)
            validate_instance(record, record_schemas[version], str(path))
            handoff_id = record["handoff_id"]
            if handoff_id != path.stem:
                raise ValidationFailure(
                    f"{path}: handoff_id {handoff_id!r} does not match filename"
                )
            if handoff_id in seen_record_ids:
                raise ValidationFailure(f"duplicate record handoff_id: {handoff_id}")
            seen_record_ids.add(handoff_id)
            records.append((path, record))

        for path, record in records:
            if record["schema_version"] in {"1.2", "1.3"}:
                validate_v1_2_semantics(
                    record,
                    path,
                    seen_record_ids,
                    dispatches,
                    verify_final_subject_baseline=True,
                )

        ledger_entries: list[dict[str, Any]] = []
        for line_number, raw_line in enumerate(LEDGER_PATH.read_text().splitlines(), start=1):
            try:
                entry = json.loads(raw_line)
            except json.JSONDecodeError as error:
                raise ValidationFailure(
                    f"{LEDGER_PATH}:{line_number}: invalid JSON: {error}"
                ) from error
            if not isinstance(entry, dict):
                raise ValidationFailure(
                    f"{LEDGER_PATH}:{line_number}: expected a JSON object"
                )
            validate_instance(
                entry, ledger_schema, f"{LEDGER_PATH}:{line_number}"
            )
            ledger_entries.append(entry)

        ledger_ids = [entry["handoff_id"] for entry in ledger_entries]
        if len(ledger_ids) != len(set(ledger_ids)):
            raise ValidationFailure("ledger contains duplicate handoff_id values")

        expected_entries = [
            expected_ledger_entry(record, path) for path, record in records
        ]
        expected_by_id = {entry["handoff_id"]: entry for entry in expected_entries}
        actual_by_id = {entry["handoff_id"]: entry for entry in ledger_entries}
        if set(expected_by_id) != set(actual_by_id):
            missing = sorted(set(expected_by_id) - set(actual_by_id))
            extra = sorted(set(actual_by_id) - set(expected_by_id))
            raise ValidationFailure(
                f"record/ledger ID mismatch: missing={missing}, extra={extra}"
            )
        for handoff_id, expected in expected_by_id.items():
            if actual_by_id[handoff_id] != expected:
                raise ValidationFailure(
                    f"ledger entry does not match canonical record: {handoff_id}"
                )

        rebuilt = "".join(
            json.dumps(entry, separators=(",", ":"), ensure_ascii=False) + "\n"
            for entry in expected_entries
        )
        if LEDGER_PATH.read_bytes() != rebuilt.encode("utf-8"):
            raise ValidationFailure(
                "ledger.jsonl differs byte-for-byte from the deterministic in-memory rebuild"
            )

    except (OSError, KeyError, ValidationFailure) as error:
        print(f"handoff validation failed: {error}", file=sys.stderr)
        return 1

    print(
        "handoff validation passed: "
        f"{len(record_schemas)} record schemas, "
        f"{len(internal_dispatch_schemas)} internal-dispatch schemas, "
        f"2 templates, {len(records)} records, {len(dispatches)} current internal "
        f"dispatches, {len(LEGACY_DISPATCH_ADMISSION)} admitted legacy dispatches, "
        f"{len(ledger_entries)} ledger entries"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
