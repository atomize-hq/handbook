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
from datetime import datetime, timezone
from pathlib import Path, PurePosixPath
from typing import Any, Callable

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
    "1.4": ROOT / "handoff-record.v1.4.schema.json",
}
INTERNAL_DISPATCH_SCHEMA_PATHS = {
    "1.0": ROOT / "internal-dispatch.schema.json",
    "1.1": ROOT / "internal-dispatch.v1.1.schema.json",
    "1.2": ROOT / "internal-dispatch.v1.2.schema.json",
    "1.3": ROOT / "internal-dispatch.v1.3.schema.json",
    "1.4": ROOT / "internal-dispatch.v1.4.schema.json",
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
IMMUTABLE_V1_3_DISPATCH_COUNT = 66
IMMUTABLE_V1_3_DISPATCH_CORPUS_FINGERPRINT = (
    "sha256:28b9d75ca1fbc0524db4ccddeff3f45cbea59c36e5ad21ad4dce6452b961377f"
)
IMMUTABLE_V1_3_RECORD_COUNT = 12
IMMUTABLE_V1_3_RECORD_CORPUS_FINGERPRINT = (
    "sha256:71f0946d65b72ab343bda0450a305c8e75358bc97b48923038a4c3255f9e545f"
)
PRE_REGISTRY_V1_4_DISPATCH_ADMISSION = {
    "20260728T024913Z--HCM-0-8--causal-review-budget-lineage-hardening-review.json": (
        "65221cfb4fba822b47305f11b92524d419d3698dc853aef08d7573750a828e99"
    )
}
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


def parse_utc_timestamp(value: str, source: Path) -> datetime:
    try:
        parsed = datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ").replace(
            tzinfo=timezone.utc
        )
    except ValueError as error:
        raise ValidationFailure(
            f"{source}: timestamp must use canonical UTC seconds with Z: {value!r}"
        ) from error
    return parsed


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise ValidationFailure(f"{path}: invalid JSON: {error}") from error
    if not isinstance(value, dict):
        raise ValidationFailure(f"{path}: expected a JSON object")
    return value


def load_dispatch_template_fixture() -> dict[str, Any]:
    """Return the ordinary historical/self-test template shape."""
    value = load_json(INTERNAL_DISPATCH_TEMPLATE_PATH)
    value.pop("authority_continuation", None)
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


def validate_immutable_corpus_entries(
    entries: list[tuple[str, str]],
    *,
    expected_count: int,
    expected_fingerprint: str,
    label: str,
) -> None:
    ordered_entries = sorted(entries)
    aggregate = "sha256:" + hashlib.sha256(
        "".join(
            f"{filename}\0{entry_sha256}\n"
            for filename, entry_sha256 in ordered_entries
        ).encode()
    ).hexdigest()
    if (
        len(ordered_entries) != expected_count
        or aggregate != expected_fingerprint
    ):
        raise ValidationFailure(
            f"immutable {label} corpus mismatch: count={len(ordered_entries)}, "
            f"fingerprint={aggregate}"
        )


def validate_immutable_v1_3_corpora(
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
    records: list[tuple[Path, dict[str, Any]]],
) -> None:
    validate_immutable_corpus_entries(
        [
            (path.name, dispatch_sha256)
            for path, dispatch, dispatch_sha256 in dispatches.values()
            if dispatch["schema_version"] == "1.3"
        ],
        expected_count=IMMUTABLE_V1_3_DISPATCH_COUNT,
        expected_fingerprint=IMMUTABLE_V1_3_DISPATCH_CORPUS_FINGERPRINT,
        label="internal-dispatch v1.3",
    )
    validate_immutable_corpus_entries(
        [
            (path.name, hashlib.sha256(path.read_bytes()).hexdigest())
            for path, record in records
            if record["schema_version"] == "1.3"
        ],
        expected_count=IMMUTABLE_V1_3_RECORD_COUNT,
        expected_fingerprint=IMMUTABLE_V1_3_RECORD_CORPUS_FINGERPRINT,
        label="handoff-record v1.3",
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
            and dispatch.get("schema_version") in {"1.3", "1.4"}
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
            {
                finding_id
                for run in prior_cycle["runs"]
                if run["run_id"] in expected_trigger_ids
                for finding_id in [
                    *run["finding_refs"],
                    *run.get("carried_finding_refs", []),
                ]
                if findings_by_id[finding_id]["priority"] in {"P1", "P2"}
            }
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


REVIEW_STAGE_ORDER = {
    "authority_admission": -1,
    "planning": 0,
    "implementation": 1,
    "proof": 2,
    "final_closeout": 3,
}
AUTHORITY_CONTINUATION_SLOTS = (
    "authority_admission",
    "implementation",
    "proof",
    "final_closeout",
)
RISK_ORDER = {"LOW": 0, "MEDIUM": 1, "HIGH": 2, "CRITICAL": 3}
PRE_REVIEW_CHECK_KINDS = {
    "complete_packet_wall",
    "recursive_fixture_consumer_inventory",
    "manifest_replay",
    "formatting",
    "whitespace",
}
CAUSAL_FOLLOWUP_REASONS = {
    "reviewer_finding",
    "remediation_unmasked_test_failure",
    "proof_gap",
    "manifest_scope_omission",
}
ANCILLARY_PATH_KINDS = {
    "test_fixture",
    "copied_fixture_authority",
    "deterministic_golden",
    "test_assertion",
}
ANCILLARY_PROHIBITED_NAMES = {
    "Cargo.lock",
    "Cargo.toml",
    "package.json",
    "package-lock.json",
    "pnpm-lock.yaml",
    "pyproject.toml",
    "requirements.txt",
}


def derive_causal_budget_id(
    parent_orchestration_id: str, integrated_outcome_id: str
) -> str:
    encoded = (
        f"{parent_orchestration_id}\0{integrated_outcome_id}\n".encode()
    )
    return "sha256:" + hashlib.sha256(encoded).hexdigest()


def canonical_jcs_json(value: Any) -> str:
    """Encode the grant's closed JSON domain using RFC 8785 ordering."""
    if value is None or isinstance(value, (bool, int)):
        return json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    if isinstance(value, list):
        return "[" + ",".join(canonical_jcs_json(item) for item in value) + "]"
    if isinstance(value, dict):
        if any(not isinstance(key, str) for key in value):
            raise ValidationFailure("canonical grant JSON requires string keys")
        items = sorted(
            value.items(),
            key=lambda item: item[0].encode("utf-16-be", "surrogatepass"),
        )
        return "{" + ",".join(
            canonical_jcs_json(key) + ":" + canonical_jcs_json(item)
            for key, item in items
        ) + "}"
    raise ValidationFailure(
        f"canonical grant JSON rejects unsupported value {type(value).__name__}"
    )


def derive_authority_grant_fingerprint(grant: dict[str, Any]) -> str:
    encoded = (canonical_jcs_json(grant) + "\n").encode("utf-8")
    return "sha256:" + hashlib.sha256(encoded).hexdigest()


def authority_artifact_from_grant(grant: dict[str, Any]) -> dict[str, Any]:
    authority = grant["authority_ref"]
    return {
        "schema_id": "handbook.authority-continuation-grant",
        "schema_version": "1.0",
        "extension_id": grant["extension_id"],
        "predecessor_handoff_id": grant["predecessor_handoff_id"],
        "predecessor_dispatch_population_fingerprint": grant[
            "predecessor_dispatch_population_fingerprint"
        ],
        "parent_orchestration_id": grant["parent_orchestration_id"],
        "integrated_outcome_id": grant["integrated_outcome_id"],
        "outcome_registry_fingerprint": grant["outcome_registry_fingerprint"],
        "causal_budget_id": grant["causal_budget_id"],
        "packet_ids": grant["packet_ids"],
        "authority_ref": {
            "issuer": authority["issuer"],
            "owner": authority["owner"],
            "issued_at_utc": authority["issued_at_utc"],
            "source": authority["source"],
        },
        "baseline_commit": grant["baseline_commit"],
        "baseline_tree": grant["baseline_tree"],
        "subject_path_ceiling": grant["subject_path_ceiling"],
        "symbol_delta": grant["symbol_delta"],
        "risk_ceiling": grant["risk_ceiling"],
        "scope_delta": grant["scope_delta"],
        "review_allowance": grant["review_allowance"],
    }


def validate_authority_continuation_dispatch(
    dispatch: dict[str, Any], dispatch_path: Path, *, repo_root: Path = REPO_ROOT
) -> None:
    continuation = dispatch.get("authority_continuation")
    causal = dispatch["causal_control"]
    if continuation is None:
        if causal["review_stage"] == "authority_admission" or causal[
            "stage_transition"
        ]["kind"] == "authority_extension":
            raise ValidationFailure(
                f"{dispatch_path}: authority stage/transition requires a continuation grant"
            )
        return

    grant = continuation["grant"]
    expected_fingerprint = derive_authority_grant_fingerprint(grant)
    if continuation["grant_fingerprint"] != expected_fingerprint:
        raise ValidationFailure(
            f"{dispatch_path}: authority continuation grant fingerprint mismatch"
        )
    stable_identity = {
        "parent_orchestration_id": dispatch["parent_orchestration_id"],
        "integrated_outcome_id": causal["integrated_outcome_id"],
        "outcome_registry_fingerprint": causal.get("outcome_registry_fingerprint"),
        "causal_budget_id": causal["causal_budget_id"],
    }
    for field, expected in stable_identity.items():
        if grant[field] != expected:
            raise ValidationFailure(
                f"{dispatch_path}: authority grant {field} drifts from dispatch identity"
            )
    if dispatch["packet_id"] not in grant["packet_ids"]:
        raise ValidationFailure(
            f"{dispatch_path}: authority grant does not register dispatch packet"
        )
    registry_packets = sorted(
        packet_id
        for outcome in dispatch["causal_outcome_registry"]["outcomes"]
        if outcome["integrated_outcome_id"] == grant["integrated_outcome_id"]
        for packet_id in outcome["packet_ids"]
    )
    if grant["packet_ids"] != registry_packets:
        raise ValidationFailure(
            f"{dispatch_path}: authority grant packet identity differs from registry"
        )
    if continuation["review_slot"] != causal["review_stage"]:
        raise ValidationFailure(
            f"{dispatch_path}: authority review slot and causal stage differ"
        )
    paths = grant["subject_path_ceiling"]
    if paths != sorted(paths) or len(paths) != len(set(paths)):
        raise ValidationFailure(
            f"{dispatch_path}: authority subject path ceiling must be unique and sorted"
        )
    symbols = grant["symbol_delta"]
    if symbols != sorted(symbols, key=lambda item: (item["symbol"], item["risk"])):
        raise ValidationFailure(
            f"{dispatch_path}: authority symbol delta must be sorted"
        )
    if len({item["symbol"] for item in symbols}) != len(symbols):
        raise ValidationFailure(
            f"{dispatch_path}: authority symbol delta contains duplicates"
        )
    if any(RISK_ORDER[item["risk"]] > RISK_ORDER[grant["risk_ceiling"]] for item in symbols):
        raise ValidationFailure(
            f"{dispatch_path}: authority symbol risk exceeds grant ceiling"
        )
    allowance = grant["review_allowance"]
    if [item["review_slot"] for item in allowance] != list(
        AUTHORITY_CONTINUATION_SLOTS
    ) or any(item["max_cycles"] != 4 for item in allowance):
        raise ValidationFailure(
            f"{dispatch_path}: authority continuation review allowance is not exact"
        )
    manifest = {
        item["path"]: item["sha256"]
        for item in dispatch["subject_manifest"]["entries"]
    }
    authority = grant["authority_ref"]
    if manifest.get(authority["path"]) != authority["sha256"]:
        raise ValidationFailure(
            f"{dispatch_path}: authority artifact is absent or stale in subject manifest"
        )
    attestation = authority["attestation"]
    if manifest.get(attestation["dispatch_ref"]) != attestation[
        "dispatch_sha256"
    ].removeprefix("sha256:"):
        raise ValidationFailure(
            f"{dispatch_path}: authority review dispatch is absent or stale in subject manifest"
        )
    if not set(manifest).issubset(set(paths)):
        raise ValidationFailure(
            f"{dispatch_path}: dispatch manifest exceeds authority subject ceiling"
        )
    if parse_utc_timestamp(authority["issued_at_utc"], dispatch_path) >= parse_utc_timestamp(
        dispatch["created_at_utc"], dispatch_path
    ):
        raise ValidationFailure(
            f"{dispatch_path}: authority artifact must predate continuation dispatch"
        )
    authority_path = repo_root / authority["path"]
    artifact = load_json(authority_path)
    expected_artifact = authority_artifact_from_grant(grant)
    if artifact != expected_artifact:
        raise ValidationFailure(
            f"{dispatch_path}: authority artifact and immutable grant fields differ"
        )


def is_pre_registry_v1_4_dispatch(dispatch_path: Path) -> bool:
    expected_sha256 = PRE_REGISTRY_V1_4_DISPATCH_ADMISSION.get(
        dispatch_path.name
    )
    return (
        expected_sha256 is not None
        and dispatch_path.is_file()
        and hashlib.sha256(dispatch_path.read_bytes()).hexdigest()
        == expected_sha256
    )


def derive_outcome_registry_fingerprint(outcomes: list[dict[str, Any]]) -> str:
    encoded = (
        json.dumps(
            outcomes,
            ensure_ascii=False,
            separators=(",", ":"),
            sort_keys=True,
        )
        + "\n"
    ).encode()
    return "sha256:" + hashlib.sha256(encoded).hexdigest()


def validate_v1_4_outcome_registry(
    dispatch: dict[str, Any],
    dispatch_path: Path,
) -> None:
    registry = dispatch.get("causal_outcome_registry")
    causal = dispatch["causal_control"]
    registry_fingerprint = causal.get("outcome_registry_fingerprint")
    if registry is None or registry_fingerprint is None:
        if registry is None and registry_fingerprint is None and (
            is_pre_registry_v1_4_dispatch(dispatch_path)
        ):
            return
        raise ValidationFailure(
            f"{dispatch_path}: current v1.4 dispatch lacks a frozen causal "
            "outcome registry"
        )

    created_at = parse_utc_timestamp(
        dispatch["created_at_utc"],
        dispatch_path,
    )
    declared_at = parse_utc_timestamp(
        registry["declared_at_utc"],
        dispatch_path,
    )
    if declared_at >= created_at:
        raise ValidationFailure(
            f"{dispatch_path}: causal outcome registry must be frozen before "
            "the dispatch is created"
        )
    outcomes = registry["outcomes"]
    outcome_ids = [entry["integrated_outcome_id"] for entry in outcomes]
    if outcome_ids != sorted(outcome_ids) or len(outcome_ids) != len(
        set(outcome_ids)
    ):
        raise ValidationFailure(
            f"{dispatch_path}: causal outcome registry entries must be unique "
            "and sorted"
        )
    for entry in outcomes:
        packet_ids = entry["packet_ids"]
        if packet_ids != sorted(
            packet_ids,
            key=lambda packet_id: "" if packet_id is None else packet_id,
        ) or len(packet_ids) != len(set(packet_ids)):
            raise ValidationFailure(
                f"{dispatch_path}: outcome packet IDs must be unique and sorted"
            )

    expected_fingerprint = derive_outcome_registry_fingerprint(outcomes)
    if (
        registry["fingerprint"] != expected_fingerprint
        or registry_fingerprint != expected_fingerprint
    ):
        raise ValidationFailure(
            f"{dispatch_path}: causal outcome registry fingerprint mismatch"
        )
    matching_outcomes = [
        entry
        for entry in outcomes
        if entry["integrated_outcome_id"]
        == causal["integrated_outcome_id"]
    ]
    if len(matching_outcomes) != 1:
        raise ValidationFailure(
            f"{dispatch_path}: integrated outcome is absent from its frozen "
            "registry"
        )
    if dispatch["packet_id"] not in matching_outcomes[0]["packet_ids"]:
        raise ValidationFailure(
            f"{dispatch_path}: packet is not authorized for its integrated "
            "outcome"
        )


def observed_git_changed_lines(
    baseline_ref: str,
    entry_path: str,
    *,
    repo_root: Path,
    target_ref: str | None,
) -> int:
    command = ["git", "diff", "--numstat", "--no-renames", baseline_ref]
    if target_ref is not None:
        command.append(target_ref)
    command.extend(["--", entry_path])
    result = subprocess.run(
        command,
        cwd=repo_root,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise ValidationFailure(
            f"{entry_path}: cannot compute ancillary diff from "
            f"{baseline_ref!r}: {result.stderr.strip()}"
        )
    rows = [line for line in result.stdout.splitlines() if line.strip()]
    if not rows and target_ref is None:
        untracked = subprocess.run(
            [
                "git",
                "ls-files",
                "--others",
                "--exclude-standard",
                "--",
                entry_path,
            ],
            cwd=repo_root,
            check=False,
            capture_output=True,
            text=True,
        )
        if untracked.returncode != 0:
            raise ValidationFailure(
                f"{entry_path}: cannot inspect untracked ancillary path"
            )
        if entry_path in untracked.stdout.splitlines():
            try:
                return len(
                    (repo_root / entry_path)
                    .read_text(encoding="utf-8")
                    .splitlines()
                )
            except (OSError, UnicodeDecodeError) as error:
                raise ValidationFailure(
                    f"{entry_path}: new ancillary path must be UTF-8 text"
                ) from error
    if len(rows) > 1:
        raise ValidationFailure(
            f"{entry_path}: ancillary diff produced multiple numstat rows"
        )
    if not rows:
        return 0
    additions, deletions, *_ = rows[0].split("\t")
    if additions == "-" or deletions == "-":
        raise ValidationFailure(
            f"{entry_path}: ancillary diff cannot use a binary path"
        )
    return int(additions) + int(deletions)


def validate_v1_4_dispatch_control(
    dispatch: dict[str, Any],
    dispatch_path: Path,
    *,
    verify_ancillary_diff: bool = False,
    ancillary_target_ref: str | None = None,
    repo_root: Path = REPO_ROOT,
) -> None:
    parse_utc_timestamp(dispatch["created_at_utc"], dispatch_path)
    validate_v1_4_outcome_registry(dispatch, dispatch_path)
    validate_authority_continuation_dispatch(
        dispatch, dispatch_path, repo_root=repo_root
    )
    causal = dispatch["causal_control"]
    expected_budget_id = derive_causal_budget_id(
        dispatch["parent_orchestration_id"],
        causal["integrated_outcome_id"],
    )
    if causal["causal_budget_id"] != expected_budget_id:
        raise ValidationFailure(
            f"{dispatch_path}: causal_budget_id is not derived from the "
            "parent orchestration and integrated outcome"
        )

    convergence = dispatch["pre_review_convergence"]
    if dispatch["role"] == "review":
        if convergence is None:
            raise ValidationFailure(
                f"{dispatch_path}: review dispatch lacks pre-review convergence"
            )
        checks = convergence["checks"]
        kinds = [check["kind"] for check in checks]
        if (
            len(kinds) != len(set(kinds))
            or set(kinds) != PRE_REVIEW_CHECK_KINDS
        ):
            raise ValidationFailure(
                f"{dispatch_path}: pre-review convergence must contain exactly "
                "the packet wall, recursive inventory, manifest replay, "
                "formatting, and whitespace checks"
            )
        if any(check["status"] != "passed" for check in checks):
            raise ValidationFailure(
                f"{dispatch_path}: pre-review convergence did not pass"
            )
        completed_at = parse_utc_timestamp(
            convergence["completed_at_utc"],
            dispatch_path,
        )
        if completed_at >= parse_utc_timestamp(
            dispatch["created_at_utc"],
            dispatch_path,
        ):
            raise ValidationFailure(
                f"{dispatch_path}: pre-review convergence must complete before "
                "the review dispatch is created"
            )
    elif convergence is not None:
        raise ValidationFailure(
            f"{dispatch_path}: non-review dispatch cannot claim a pre-review gate"
        )

    allowance = dispatch["ancillary_allowance"]
    entries = allowance["entries"]
    baseline_ref = allowance.get("baseline_ref")
    if allowance["kind"] == "none":
        if (
            entries
            or allowance["max_paths"] != 0
            or allowance["max_changed_lines"] != 0
            or allowance["risk_ceiling"] != "no_ancillary_surface"
            or baseline_ref is not None
            or (
                "baseline_ref" not in allowance
                and not is_pre_registry_v1_4_dispatch(dispatch_path)
            )
        ):
            raise ValidationFailure(
                f"{dispatch_path}: empty ancillary allowance has non-zero scope"
            )
        return

    if allowance["risk_ceiling"] != "test_proof_only":
        raise ValidationFailure(
            f"{dispatch_path}: ancillary allowance exceeds test/proof-only risk"
        )
    if not isinstance(baseline_ref, str) or not baseline_ref:
        raise ValidationFailure(
            f"{dispatch_path}: bounded ancillary allowance lacks a baseline ref"
        )
    paths = [entry["path"] for entry in entries]
    if paths != sorted(paths) or len(paths) != len(set(paths)):
        raise ValidationFailure(
            f"{dispatch_path}: ancillary paths must be unique and sorted"
        )
    if len(entries) > allowance["max_paths"]:
        raise ValidationFailure(
            f"{dispatch_path}: ancillary path count exceeds its ceiling"
        )
    if sum(entry["changed_lines"] for entry in entries) > allowance[
        "max_changed_lines"
    ]:
        raise ValidationFailure(
            f"{dispatch_path}: ancillary changed-line count exceeds its ceiling"
        )
    manifest_paths = {
        entry["path"] for entry in dispatch["subject_manifest"]["entries"]
    }
    for entry in entries:
        path = entry["path"]
        kind = entry["path_kind"]
        pure_path = PurePosixPath(path)
        path_parts = set(pure_path.parts)
        if kind not in ANCILLARY_PATH_KINDS:
            raise ValidationFailure(
                f"{dispatch_path}: unsupported ancillary path kind {kind!r}"
            )
        if path not in manifest_paths:
            raise ValidationFailure(
                f"{dispatch_path}: ancillary path is absent from the subject "
                f"manifest: {path}"
            )
        if pure_path.name in ANCILLARY_PROHIBITED_NAMES:
            raise ValidationFailure(
                f"{dispatch_path}: ancillary allowance cannot authorize "
                f"dependency/version path {path}"
            )
        is_test_path = "tests" in path_parts or "test" in path_parts
        is_fixture_path = (
            "fixtures" in path_parts or "goldens" in path_parts
        )
        if not is_test_path:
            raise ValidationFailure(
                f"{dispatch_path}: ancillary allowance cannot authorize a "
                f"runtime/API/schema/authority path: {path}"
            )
        if kind != "test_assertion" and not is_fixture_path:
            raise ValidationFailure(
                f"{dispatch_path}: ancillary fixture/golden is outside a "
                f"fixture path: {path}"
            )
        if verify_ancillary_diff:
            observed_lines = observed_git_changed_lines(
                baseline_ref,
                path,
                repo_root=repo_root,
                target_ref=ancillary_target_ref,
            )
            if entry["changed_lines"] != observed_lines:
                raise ValidationFailure(
                    f"{dispatch_path}: ancillary changed-line count for {path} "
                    f"is declared={entry['changed_lines']} "
                    f"observed={observed_lines}"
                )


def validate_v1_4_causal_sequence(
    record_path: Path,
    runs: list[dict[str, Any]],
    dispatch_by_run_id: dict[str, dict[str, Any]],
) -> None:
    ordered_dispatches = [
        dispatch_by_run_id[run["run_id"]]
        for run in runs
    ]
    registry_fingerprints = {
        dispatch["causal_control"].get("outcome_registry_fingerprint")
        for dispatch in ordered_dispatches
        if dispatch["causal_control"].get("outcome_registry_fingerprint")
        is not None
    }
    if len(registry_fingerprints) > 1:
        raise ValidationFailure(
            f"{record_path}: parent orchestration changed its frozen causal "
            "outcome registry"
        )
    admitted_pre_registry_ids = {
        Path(filename).stem
        for filename in PRE_REGISTRY_V1_4_DISPATCH_ADMISSION
    }
    missing_registry = [
        dispatch["dispatch_id"]
        for dispatch in ordered_dispatches
        if dispatch["causal_control"].get("outcome_registry_fingerprint")
        is None
        and dispatch["dispatch_id"] not in admitted_pre_registry_ids
    ]
    if missing_registry:
        raise ValidationFailure(
            f"{record_path}: dispatches lack the frozen causal outcome "
            f"registry: {missing_registry!r}"
        )
    if registry_fingerprints:
        registry_dispatch = next(
            dispatch
            for dispatch in ordered_dispatches
            if dispatch.get("causal_outcome_registry") is not None
        )
        registered_pairs = {
            (entry["integrated_outcome_id"], packet_id)
            for entry in registry_dispatch["causal_outcome_registry"][
                "outcomes"
            ]
            for packet_id in entry["packet_ids"]
        }
        for dispatch in ordered_dispatches:
            pair = (
                dispatch["causal_control"]["integrated_outcome_id"],
                dispatch["packet_id"],
            )
            if pair not in registered_pairs:
                raise ValidationFailure(
                    f"{record_path}: dispatch {dispatch['dispatch_id']!r} "
                    "uses an undeclared outcome/packet pair"
                )

    sequences: dict[str, list[tuple[dict[str, Any], dict[str, Any]]]] = {}
    for run in runs:
        dispatch = dispatch_by_run_id[run["run_id"]]
        causal = dispatch["causal_control"]
        expected_budget_id = derive_causal_budget_id(
            dispatch["parent_orchestration_id"],
            causal["integrated_outcome_id"],
        )
        if causal["causal_budget_id"] != expected_budget_id:
            raise ValidationFailure(
                f"{record_path}: dispatch {dispatch['dispatch_id']!r} has an "
                "invalid causal budget binding"
            )
        sequences.setdefault(causal["causal_budget_id"], []).append(
            (run, dispatch)
        )

    for causal_budget_id, sequence in sequences.items():
        prior_dispatch: dict[str, Any] | None = None
        prior_stage: str | None = None
        continuation_fingerprint: str | None = None
        continuation_started = False
        selector_clean = False
        selector_failed = False
        highest_continuation_slot = -1
        clean_continuation_slots: set[str] = set()
        continuation_slot_dispatch_counts: dict[str, int] = {}
        continuation_slot_noncycle_counts: dict[str, int] = {}
        stage_cycles: dict[
            str, list[tuple[dict[str, Any], list[dict[str, Any]]]]
        ] = {}
        for run, dispatch in sequence:
            causal = dispatch["causal_control"]
            stage = causal["review_stage"]
            transition = causal["stage_transition"]
            predecessor_id = causal["causal_predecessor_dispatch_id"]
            continuation = dispatch.get("authority_continuation")
            if continuation is not None:
                fingerprint = continuation["grant_fingerprint"]
                if continuation_fingerprint is None:
                    continuation_fingerprint = fingerprint
                elif fingerprint != continuation_fingerprint:
                    raise ValidationFailure(
                        f"{record_path}: parent repeats a non-equivalent authority grant"
                    )
                if selector_failed:
                    raise ValidationFailure(
                        f"{record_path}: continuation dispatch follows failed authority admission"
                    )
                slot = continuation["review_slot"]
                continuation_slot_dispatch_counts[slot] = (
                    continuation_slot_dispatch_counts.get(slot, 0) + 1
                )
                if continuation_slot_dispatch_counts[slot] > 5:
                    raise ValidationFailure(
                        f"{record_path}: authority continuation slot {slot!r} exceeds five dispatches"
                    )
                if dispatch["review_cycle"] is None:
                    continuation_slot_noncycle_counts[slot] = (
                        continuation_slot_noncycle_counts.get(slot, 0) + 1
                    )
                    if continuation_slot_noncycle_counts[slot] > 1:
                        raise ValidationFailure(
                            f"{record_path}: authority continuation slot {slot!r} repeats a non-cycle dispatch"
                        )
                slot_index = AUTHORITY_CONTINUATION_SLOTS.index(slot)
                if slot in clean_continuation_slots:
                    raise ValidationFailure(
                        f"{record_path}: continuation dispatch follows CLEAN in slot {slot!r}"
                    )
                if slot_index > highest_continuation_slot + 1 or slot_index < highest_continuation_slot:
                    raise ValidationFailure(
                        f"{record_path}: authority continuation slot order is invalid"
                    )
                highest_continuation_slot = max(highest_continuation_slot, slot_index)
                if continuation["review_slot"] != "authority_admission" and not selector_clean:
                    raise ValidationFailure(
                        f"{record_path}: continuation writes or advances before selector CLEAN"
                    )
            if prior_dispatch is None:
                if (
                    transition["kind"] != "enter"
                    or transition["from_stage"] is not None
                    or predecessor_id is not None
                ):
                    raise ValidationFailure(
                        f"{record_path}: causal budget {causal_budget_id!r} "
                        "must begin with an explicit stage entry"
                    )
            else:
                if predecessor_id != prior_dispatch["dispatch_id"]:
                    raise ValidationFailure(
                        f"{record_path}: dispatch {dispatch['dispatch_id']!r} "
                        "does not name the immediately preceding causal dispatch"
                    )
                is_authority_extension = (
                    continuation is not None
                    and not continuation_started
                    and stage == "authority_admission"
                    and transition == {
                        "kind": "authority_extension",
                        "from_stage": prior_stage,
                    }
                )
                if (
                    REVIEW_STAGE_ORDER[stage] < REVIEW_STAGE_ORDER[prior_stage]
                    and not is_authority_extension
                ):
                    raise ValidationFailure(
                        f"{record_path}: causal budget {causal_budget_id!r} "
                        "regresses its review stage"
                    )
                if is_authority_extension:
                    expected_transition = ("authority_extension", prior_stage)
                    continuation_started = True
                elif stage == prior_stage:
                    expected_transition = ("continue", prior_stage)
                else:
                    expected_transition = ("enter", prior_stage)
                actual_transition = (
                    transition["kind"],
                    transition["from_stage"],
                )
                if actual_transition != expected_transition:
                    raise ValidationFailure(
                        f"{record_path}: dispatch {dispatch['dispatch_id']!r} "
                        "has an inexact stage transition"
                    )

            review_cycle = dispatch["review_cycle"]
            event_reason = causal["event_reason"]
            if event_reason == "mechanical_closeout" and review_cycle is not None:
                raise ValidationFailure(
                    f"{record_path}: mechanical closeout cannot create a "
                    "review cycle"
                )
            if review_cycle is not None:
                cycle_group = (
                    f"authority:{continuation['review_slot']}"
                    if continuation is not None
                    else stage
                )
                cycles = stage_cycles.setdefault(cycle_group, [])
                if not cycles or cycles[-1][0]["cycle_id"] != review_cycle[
                    "cycle_id"
                ]:
                    if any(
                        cycle["cycle_id"] == review_cycle["cycle_id"]
                        for cycle, _ in cycles
                    ):
                        raise ValidationFailure(
                            f"{record_path}: review cycle "
                            f"{review_cycle['cycle_id']!r} is not contiguous"
                        )
                    if any(
                        prior_run["final_status"] == "completed"
                        and prior_run["verdict"] == "clean"
                        for _, cycle_runs in cycles
                        for prior_run in cycle_runs
                    ):
                        raise ValidationFailure(
                            f"{record_path}: review stage {cycle_group!r} creates a "
                            "cycle after CLEAN"
                        )
                    cycles.append((review_cycle, [run]))
                else:
                    prior_cycle, cycle_runs = cycles[-1]
                    if (
                        prior_cycle["kind"] != review_cycle["kind"]
                        or prior_cycle["trigger_run_ids"]
                        != review_cycle["trigger_run_ids"]
                        or prior_cycle["finding_refs"]
                        != review_cycle["finding_refs"]
                    ):
                        raise ValidationFailure(
                            f"{record_path}: same-cycle review burst has "
                            "inconsistent causal fields"
                        )
                    cycle_runs.append(run)

                if continuation is not None and len(cycles[-1][1]) != 1:
                    raise ValidationFailure(
                        f"{record_path}: authority continuation permits one dispatch per cycle"
                    )

                if review_cycle["kind"] == "discovery":
                    if event_reason not in {
                        "initial_stage_review",
                        "planned_stage_transition",
                    }:
                        raise ValidationFailure(
                            f"{record_path}: causal follow-up reason "
                            f"{event_reason!r} cannot become discovery"
                        )
                elif event_reason not in CAUSAL_FOLLOWUP_REASONS:
                    raise ValidationFailure(
                        f"{record_path}: review follow-up has non-causal event "
                        f"reason {event_reason!r}"
                    )

            prior_dispatch = dispatch
            prior_stage = stage
            if continuation is not None and stage == "authority_admission":
                if run["final_status"] == "completed" and run["verdict"] == "clean":
                    selector_clean = True
                elif run["final_status"] == "completed" and run["verdict"] == "findings":
                    selector_failed = True
            if (
                continuation is not None
                and run["final_status"] == "completed"
                and run["verdict"] == "clean"
            ):
                clean_continuation_slots.add(continuation["review_slot"])

        for stage, cycles in stage_cycles.items():
            kinds = [cycle["kind"] for cycle, _ in cycles]
            if not kinds or kinds[0] != "discovery":
                raise ValidationFailure(
                    f"{record_path}: review stage {stage!r} must begin with "
                    "discovery"
                )
            if kinds.count("discovery") != 1 or kinds.count("closure") > 1:
                raise ValidationFailure(
                    f"{record_path}: review stage {stage!r} permits one "
                    "discovery lineage and at most one closure"
                )
            if len(kinds) > 1 and kinds[1] != "closure":
                raise ValidationFailure(
                    f"{record_path}: review stage {stage!r} must enter closure "
                    "before a supplemental causal cycle"
                )
            if any(kind != "supplemental_causal" for kind in kinds[2:]):
                raise ValidationFailure(
                    f"{record_path}: review stage {stage!r} has invalid cycle "
                    f"order {kinds!r}"
                )
            if kinds.count("supplemental_causal") > 2:
                raise ValidationFailure(
                    f"{record_path}: review stage {stage!r} exceeds two "
                    "supplemental causal cycles"
                )


def validate_v1_4_dispatch_prefix(
    scope_path: Path,
    dispatches: list[dict[str, Any]],
) -> None:
    ordered_dispatches = sorted(
        dispatches,
        key=lambda dispatch: (
            dispatch["created_at_utc"],
            dispatch["dispatch_id"],
        ),
    )
    synthetic_runs = [
        {
            "run_id": dispatch["dispatch_id"],
            "final_status": "not_executed",
            "verdict": "not_applicable",
        }
        for dispatch in ordered_dispatches
    ]
    validate_v1_4_causal_sequence(
        scope_path,
        synthetic_runs,
        {
            dispatch["dispatch_id"]: dispatch
            for dispatch in ordered_dispatches
        },
    )


def validate_v1_4_dispatch_population(
    record: dict[str, Any],
    record_path: Path,
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
) -> None:
    manifest = record["dispatch_population"]
    if manifest["through_created_at_utc"] != record["created_at_utc"]:
        raise ValidationFailure(
            f"{record_path}: dispatch population cutoff must equal handoff time"
        )
    cutoff = parse_utc_timestamp(
        manifest["through_created_at_utc"],
        record_path,
    )
    parent_population = sorted(
        (
            value
            for value in dispatches.values()
            if value[1]["parent_orchestration_id"]
            == record["orchestration_id"]
        ),
        key=lambda item: (
            parse_utc_timestamp(item[1]["created_at_utc"], item[0]),
            item[1]["dispatch_id"],
        ),
    )
    predecessor_versions = sorted(
        {
            dispatch["schema_version"]
            for _, dispatch, _ in parent_population
            if dispatch["schema_version"] != "1.4"
        }
    )
    if predecessor_versions:
        raise ValidationFailure(
            f"{record_path}: v1.4 parent orchestration mixes immutable "
            f"predecessor dispatch versions {predecessor_versions!r}"
        )
    population = [
        value
        for value in parent_population
        if parse_utc_timestamp(value[1]["created_at_utc"], value[0]) <= cutoff
    ]
    expected_ids = [dispatch["dispatch_id"] for _, dispatch, _ in population]
    actual_ids = [run["dispatch_id"] for run in record["delegated_runs"]]
    if actual_ids != expected_ids:
        missing = sorted(set(expected_ids) - set(actual_ids))
        extra = sorted(set(actual_ids) - set(expected_ids))
        raise ValidationFailure(
            f"{record_path}: delegated_runs do not reconcile the complete "
            f"parent dispatch population: missing={missing}, extra={extra}"
        )
    if manifest["dispatch_count"] != len(population):
        raise ValidationFailure(
            f"{record_path}: dispatch population count mismatch"
        )
    expected_budget_ids = sorted(
        {
            dispatch["causal_control"]["causal_budget_id"]
            for _, dispatch, _ in population
        }
    )
    if manifest["causal_budget_ids"] != expected_budget_ids:
        raise ValidationFailure(
            f"{record_path}: dispatch population causal budget set mismatch"
        )
    encoded = "".join(
        f"{path.relative_to(REPO_ROOT).as_posix()}\0{dispatch_sha256}\n"
        for path, _, dispatch_sha256 in population
    ).encode()
    expected_aggregate = "sha256:" + hashlib.sha256(encoded).hexdigest()
    if manifest["aggregate_fingerprint"] != expected_aggregate:
        raise ValidationFailure(
            f"{record_path}: dispatch population aggregate fingerprint mismatch"
        )


def validate_authority_continuation_chains(
    records: list[tuple[Path, dict[str, Any]]],
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
    *,
    repo_root: Path = REPO_ROOT,
) -> None:
    record_by_id = {record["handoff_id"]: (path, record) for path, record in records}
    continuation_by_parent: dict[
        str, list[tuple[Path, dict[str, Any], dict[str, Any]]]
    ] = {}
    for dispatch_path, dispatch, _ in dispatches.values():
        continuation = dispatch.get("authority_continuation")
        if continuation is not None:
            continuation_by_parent.setdefault(
                dispatch["parent_orchestration_id"], []
            ).append((dispatch_path, dispatch, continuation))

    for parent_id, entries in continuation_by_parent.items():
        entries.sort(
            key=lambda item: (
                parse_utc_timestamp(item[1]["created_at_utc"], item[0]),
                item[1]["dispatch_id"],
            )
        )
        fingerprints = {item[2]["grant_fingerprint"] for item in entries}
        if len(fingerprints) != 1:
            raise ValidationFailure(
                f"{parent_id}: more than one authority continuation grant exists"
            )
        grant = entries[0][2]["grant"]
        baseline_tree = subprocess.run(
            ["git", "rev-parse", f"{grant['baseline_commit']}^{{tree}}"],
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=False,
        )
        if (
            baseline_tree.returncode != 0
            or baseline_tree.stdout.strip() != grant["baseline_tree"]
        ):
            raise ValidationFailure(
                f"{parent_id}: authority continuation baseline commit/tree is invalid"
            )
        predecessor_entry = record_by_id.get(grant["predecessor_handoff_id"])
        if predecessor_entry is None:
            raise ValidationFailure(
                f"{parent_id}: authority continuation predecessor handoff is missing"
            )
        predecessor_path, predecessor = predecessor_entry
        predecessor_commit = subprocess.run(
            ["git", "log", "-1", "--format=%H", "--", str(predecessor_path)],
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=False,
        )
        if (
            predecessor_commit.returncode != 0
            or predecessor_commit.stdout.strip() != grant["baseline_commit"]
        ):
            raise ValidationFailure(
                f"{parent_id}: authority baseline is not the direct predecessor handoff commit"
            )
        if (
            predecessor["orchestration_id"] != parent_id
            or predecessor["status"] == "completed"
            or predecessor["stop_reason"] != "authority_boundary"
        ):
            raise ValidationFailure(
                f"{parent_id}: authority continuation predecessor is not a same-parent non-completed authority stop"
            )
        if predecessor["dispatch_population"]["aggregate_fingerprint"] != grant[
            "predecessor_dispatch_population_fingerprint"
        ]:
            raise ValidationFailure(
                f"{parent_id}: authority continuation predecessor population changed"
            )
        predecessor_cutoff = parse_utc_timestamp(
            predecessor["created_at_utc"], predecessor_path
        )
        first_path, first_dispatch, first_continuation = entries[0]
        if parse_utc_timestamp(first_dispatch["created_at_utc"], first_path) <= predecessor_cutoff:
            raise ValidationFailure(
                f"{first_path}: authority continuation does not follow predecessor cutoff"
            )
        prefix = sorted(
            (
                dispatch
                for path, dispatch, _ in dispatches.values()
                if dispatch["parent_orchestration_id"] == parent_id
                and parse_utc_timestamp(dispatch["created_at_utc"], path)
                <= predecessor_cutoff
            ),
            key=lambda item: (item["created_at_utc"], item["dispatch_id"]),
        )
        if (
            not prefix
            or first_continuation["review_slot"] != "authority_admission"
            or first_dispatch["role"] != "review"
            or first_dispatch["causal_control"]["causal_predecessor_dispatch_id"]
            != prefix[-1]["dispatch_id"]
            or first_dispatch["causal_control"]["stage_transition"]["kind"]
            != "authority_extension"
        ):
            raise ValidationFailure(
                f"{first_path}: first continuation is not the direct read-only authority selector"
            )

        authority = grant["authority_ref"]
        authority_path = repo_root / authority["path"]
        if hashlib.sha256(authority_path.read_bytes()).hexdigest() != authority["sha256"]:
            raise ValidationFailure(
                f"{first_path}: authority artifact bytes do not match the grant"
            )
        attestation = authority["attestation"]
        attested_entry = record_by_id.get(attestation["handoff_id"])
        if attested_entry is None:
            raise ValidationFailure(
                f"{first_path}: authority attestation handoff is missing"
            )
        _, attested_handoff = attested_entry
        if (
            attested_handoff["status"] != "completed"
            or attested_handoff["orchestration_id"] == parent_id
        ):
            raise ValidationFailure(
                f"{first_path}: authority attestation must be a completed different-parent handoff"
            )
        attested_run = next(
            (
                run
                for run in attested_handoff["delegated_runs"]
                if run["run_id"] == attestation["run_id"]
            ),
            None,
        )
        attested_dispatch_entry = dispatches.get(attestation["dispatch_id"])
        if attested_run is None or attested_dispatch_entry is None:
            raise ValidationFailure(
                f"{first_path}: authority attestation run or dispatch is missing"
            )
        attested_dispatch_path, attested_dispatch, attested_sha256 = (
            attested_dispatch_entry
        )
        attested_ref = attested_dispatch_path.relative_to(repo_root).as_posix()
        attested_manifest = {
            item["path"]: item["sha256"]
            for item in attested_dispatch["subject_manifest"]["entries"]
        }
        attested_manifest_aggregate = "sha256:" + hashlib.sha256(
            "".join(
                f"{item['path']}\0{item['sha256']}\n"
                for item in attested_dispatch["subject_manifest"]["entries"]
            ).encode()
        ).hexdigest()
        if (
            attested_run["dispatch_id"] != attestation["dispatch_id"]
            or attested_run["role"] != "review"
            or attested_run["final_status"] != "completed"
            or attested_run["verdict"] != "clean"
            or attested_run["subject_fingerprint"]
            != attested_dispatch["subject_fingerprint"]
            or attested_run["result_subject_fingerprint"]
            != attested_run["subject_fingerprint"]
            or attested_dispatch["subject_fingerprint"]
            != attested_manifest_aggregate
            or attested_dispatch["subject_manifest"]["aggregate_fingerprint"]
            != attested_manifest_aggregate
            or attestation["dispatch_ref"] != attested_ref
            or attestation["dispatch_sha256"] != f"sha256:{attested_sha256}"
            or attested_manifest.get(authority["path"]) != authority["sha256"]
        ):
            raise ValidationFailure(
                f"{first_path}: authority attestation is incomplete, stale, or not CLEAN"
            )

        parent_records = sorted(
            (
                (path, record)
                for path, record in records
                if record.get("schema_version") == "1.4"
                and record["orchestration_id"] == parent_id
            ),
            key=lambda item: parse_utc_timestamp(item[1]["created_at_utc"], item[0]),
        )
        successor_records = [
            (path, record)
            for path, record in parent_records
            if parse_utc_timestamp(record["created_at_utc"], path) > predecessor_cutoff
        ]
        if not successor_records:
            raise ValidationFailure(
                f"{parent_id}: continuation dispatches lack a successor handoff"
            )
        prior_successor_id = predecessor["handoff_id"]
        for successor_index, (successor_path, successor) in enumerate(successor_records):
            expected_link = [prior_successor_id]
            if (
                successor.get("source_handoff_ids") != expected_link
                or successor.get("supersedes") != expected_link
            ):
                raise ValidationFailure(
                    f"{successor_path}: continuation successor is not a direct dual-link"
                )
            if successor_index + 1 < len(successor_records) and successor["status"] == "completed":
                raise ValidationFailure(
                    f"{successor_path}: consumed continuation has a second successor"
                )
            prior_successor_id = successor["handoff_id"]
        admission_agent: str | None = None
        executor_agents: set[str] = set()
        for record_path, record in successor_records:
            summaries = record.get("authority_continuations", [])
            if len(summaries) != 1:
                raise ValidationFailure(
                    f"{record_path}: continuation successor lacks exactly one summary"
                )
            summary = summaries[0]
            cutoff = parse_utc_timestamp(record["created_at_utc"], record_path)
            included = [
                (path, dispatch, continuation)
                for path, dispatch, continuation in entries
                if parse_utc_timestamp(dispatch["created_at_utc"], path) <= cutoff
            ]
            runs_by_dispatch = {
                run["dispatch_id"]: run for run in record["delegated_runs"]
            }
            selector_run = runs_by_dispatch.get(first_dispatch["dispatch_id"])
            if selector_run is None:
                raise ValidationFailure(
                    f"{record_path}: continuation summary omits selector run"
                )
            admission_agent = selector_run["agent_id"]
            for _, dispatch, _ in included:
                run = runs_by_dispatch.get(dispatch["dispatch_id"])
                if run is not None and dispatch["role"] != "review" and run["agent_id"]:
                    executor_agents.add(run["agent_id"])
            expected_slots = []
            for slot in AUTHORITY_CONTINUATION_SLOTS:
                slot_entries = [item for item in included if item[2]["review_slot"] == slot]
                if not slot_entries:
                    continue
                if len(slot_entries) > 5 or sum(
                    dispatch["review_cycle"] is None
                    for _, dispatch, _ in slot_entries
                ) > 1:
                    raise ValidationFailure(
                        f"{record_path}: authority continuation slot membership exceeds its dispatch ceiling"
                    )
                expected_slots.append(
                    {
                        "review_slot": slot,
                        "cycle_ids": [
                            dispatch["review_cycle"]["cycle_id"]
                            for _, dispatch, _ in slot_entries
                            if dispatch["review_cycle"] is not None
                        ],
                        "dispatch_ids": [
                            dispatch["dispatch_id"] for _, dispatch, _ in slot_entries
                        ],
                    }
                )
            static_summary = {
                "extension_id": grant["extension_id"],
                "grant_fingerprint": entries[0][2]["grant_fingerprint"],
                "predecessor_handoff_id": grant["predecessor_handoff_id"],
                "selector_run_id": selector_run["run_id"],
                "baseline_commit": grant["baseline_commit"],
                "baseline_tree": grant["baseline_tree"],
                "slots": expected_slots,
            }
            if any(summary[key] != value for key, value in static_summary.items()):
                raise ValidationFailure(
                    f"{record_path}: authority continuation summary parity mismatch"
                )
            if summary["actual_changed_paths"] != sorted(summary["actual_changed_paths"]):
                raise ValidationFailure(
                    f"{record_path}: actual authority path delta must be sorted"
                )
            actual_delta = subprocess.run(
                [
                    "git",
                    "diff",
                    "--name-only",
                    "--no-renames",
                    grant["baseline_commit"],
                    record["repo_state"]["head"],
                    "--",
                ],
                cwd=repo_root,
                capture_output=True,
                text=True,
                check=False,
            )
            if actual_delta.returncode != 0:
                raise ValidationFailure(
                    f"{record_path}: cannot replay authority baseline-to-tip Git delta"
                )
            if summary["actual_changed_paths"] != sorted(
                path for path in actual_delta.stdout.splitlines() if path
            ):
                raise ValidationFailure(
                    f"{record_path}: recorded authority path delta differs from Git"
                )
            if not set(summary["actual_changed_paths"]).issubset(
                set(grant["subject_path_ceiling"])
            ):
                raise ValidationFailure(
                    f"{record_path}: actual changed path exceeds authority ceiling"
                )
            granted_symbols = {
                (item["symbol"], item["risk"]) for item in grant["symbol_delta"]
            }
            observed_symbols = {
                (item["symbol"], item["risk"])
                for item in summary["changed_symbols"]
            }
            if (
                summary["changed_symbols"]
                != sorted(
                    summary["changed_symbols"],
                    key=lambda item: (item["symbol"], item["risk"]),
                )
                or not observed_symbols.issubset(granted_symbols)
            ):
                raise ValidationFailure(
                    f"{record_path}: GitNexus changed-symbol observations are unsorted or out of ceiling"
                )
            if RISK_ORDER[summary["observed_risk"]] > RISK_ORDER[grant["risk_ceiling"]]:
                raise ValidationFailure(
                    f"{record_path}: observed authority risk exceeds grant ceiling"
                )
            validate_gitnexus_change_detection_evidence(
                record_path,
                record,
                summary,
                grant,
                dispatches,
                repo_root=repo_root,
            )
            expected_status = "consumed" if record["status"] == "completed" else "active"
            if summary["status"] != expected_status:
                raise ValidationFailure(
                    f"{record_path}: authority continuation active/consumed status mismatch"
                )
            if record["status"] == "completed":
                final_entries = [item for item in included if item[2]["review_slot"] == "final_closeout"]
                if not final_entries:
                    raise ValidationFailure(
                        f"{record_path}: completed continuation lacks final-closeout review"
                    )
                final_run = runs_by_dispatch[final_entries[-1][1]["dispatch_id"]]
                if final_run["role"] != "review" or final_run["verdict"] != "clean":
                    raise ValidationFailure(
                        f"{record_path}: consumed continuation lacks final CLEAN"
                    )
                if any(
                    parse_utc_timestamp(dispatch["created_at_utc"], path) > cutoff
                    for path, dispatch, _ in entries
                ):
                    raise ValidationFailure(
                        f"{record_path}: continuation dispatch exists after consumption"
                    )

        identities = {
            authority["issuer"]["identity"],
            attested_run["agent_id"],
            admission_agent,
            *executor_agents,
        }
        expected_identity_count = 3 + len(executor_agents)
        if None in identities or len(identities) != expected_identity_count:
            raise ValidationFailure(
                f"{first_path}: issuer, attestation reviewer, admission reviewer, and executor roles collide"
            )


CODE_BEARING_SUFFIXES = (
    ".c",
    ".cc",
    ".cpp",
    ".cs",
    ".go",
    ".java",
    ".js",
    ".jsx",
    ".kt",
    ".kts",
    ".py",
    ".rb",
    ".rs",
    ".swift",
    ".ts",
    ".tsx",
)


def validate_gitnexus_change_detection_evidence(
    record_path: Path,
    record: dict[str, Any],
    summary: dict[str, Any],
    grant: dict[str, Any],
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
    *,
    repo_root: Path = REPO_ROOT,
) -> None:
    code_bearing = any(
        path.endswith(CODE_BEARING_SUFFIXES)
        for path in summary["actual_changed_paths"]
    )
    evidence_ref = summary.get("gitnexus_evidence")
    if evidence_ref is None:
        if code_bearing:
            raise ValidationFailure(
                f"{record_path}: code-bearing continuation lacks GitNexus evidence"
            )
        return

    evidence_path = repo_root / evidence_ref["path"]
    try:
        evidence_bytes = evidence_path.read_bytes()
    except OSError as error:
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence is unavailable: {error}"
        ) from error
    if hashlib.sha256(evidence_bytes).hexdigest() != evidence_ref["sha256"]:
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence bytes do not match the summary"
        )
    evidence = load_json(evidence_path)
    expected_keys = {
        "schema_id",
        "schema_version",
        "status",
        "provider",
        "command",
        "scope",
        "baseline_commit",
        "baseline_tree",
        "target_commit",
        "target_tree",
        "actual_changed_paths",
        "changed_symbols",
        "observed_risk",
        "raw_output_fingerprint",
    }
    if set(evidence) != expected_keys:
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence fields are incomplete or unknown"
        )
    string_fields = (
        "schema_id",
        "schema_version",
        "status",
        "command",
        "scope",
        "baseline_commit",
        "baseline_tree",
        "target_commit",
        "target_tree",
        "observed_risk",
        "raw_output_fingerprint",
    )
    commit_fields = ("baseline_commit", "baseline_tree", "target_commit", "target_tree")
    if (
        any(not isinstance(evidence[field], str) for field in string_fields)
        or any(
            len(evidence[field]) != 40
            or any(character not in "0123456789abcdef" for character in evidence[field])
            for field in commit_fields
        )
        or not isinstance(evidence["actual_changed_paths"], list)
        or any(
            not isinstance(path, str) or not path
            for path in evidence["actual_changed_paths"]
        )
        or not isinstance(evidence["changed_symbols"], list)
        or any(
            not isinstance(item, dict)
            or set(item) != {"symbol", "risk"}
            or not isinstance(item["symbol"], str)
            or not item["symbol"]
            or item["risk"] not in RISK_ORDER
            for item in evidence["changed_symbols"]
        )
        or evidence["observed_risk"] not in RISK_ORDER
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence field types or identities are invalid"
        )
    provider = evidence["provider"]
    if (
        evidence["schema_id"] != "handbook.gitnexus-change-detection-evidence"
        or evidence["schema_version"] != "1.0"
        or evidence["status"] != "available"
        or not isinstance(provider, dict)
        or set(provider) != {"name", "version"}
        or provider["name"] != "GitNexus"
        or not isinstance(provider["version"], str)
        or not provider["version"]
        or evidence["scope"] != "compare"
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence provider, version, status, or scope is invalid"
        )
    expected_command = (
        "detect_changes(scope=compare,base_ref="
        f"{evidence['baseline_commit']},target_ref={evidence['target_commit']})"
    )
    if evidence["command"] != expected_command:
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence command is not exact"
        )
    if (
        evidence["baseline_commit"] != grant["baseline_commit"]
        or evidence["baseline_tree"] != grant["baseline_tree"]
        or evidence["target_commit"] != record["repo_state"]["head"]
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence baseline or target identity differs"
        )
    target_tree = subprocess.run(
        ["git", "rev-parse", f"{evidence['target_commit']}^{{tree}}"],
        cwd=repo_root,
        capture_output=True,
        text=True,
        check=False,
    )
    if (
        target_tree.returncode != 0
        or target_tree.stdout.strip() != evidence["target_tree"]
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence target commit/tree is invalid"
        )
    if evidence["actual_changed_paths"] != summary["actual_changed_paths"]:
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence and actual path summary differ"
        )
    if evidence["changed_symbols"] != summary["changed_symbols"]:
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence and changed-symbol summary differ"
        )
    if code_bearing and not evidence["changed_symbols"]:
        raise ValidationFailure(
            f"{record_path}: code-bearing GitNexus evidence has empty symbol coverage"
        )
    if evidence["changed_symbols"] != sorted(
        evidence["changed_symbols"],
        key=lambda item: (item["symbol"], item["risk"]),
    ) or len({item["symbol"] for item in evidence["changed_symbols"]}) != len(
        evidence["changed_symbols"]
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence symbols must be unique and sorted"
        )
    granted_symbols = {
        (item["symbol"], item["risk"]) for item in grant["symbol_delta"]
    }
    evidence_symbols = {
        (item["symbol"], item["risk"]) for item in evidence["changed_symbols"]
    }
    if not evidence_symbols.issubset(granted_symbols):
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence symbol or risk exceeds the grant"
        )
    aggregate_risk = max(
        (item["risk"] for item in evidence["changed_symbols"]),
        key=RISK_ORDER.__getitem__,
        default="LOW",
    )
    if (
        evidence["observed_risk"] != aggregate_risk
        or summary["observed_risk"] != aggregate_risk
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus aggregate observed risk is understated or inconsistent"
        )
    raw_fingerprint = evidence["raw_output_fingerprint"]
    if (
        not isinstance(raw_fingerprint, str)
        or len(raw_fingerprint) != 71
        or not raw_fingerprint.startswith("sha256:")
        or any(character not in "0123456789abcdef" for character in raw_fingerprint[7:])
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus raw-output fingerprint is invalid"
        )

    attestation = evidence_ref["attestation"]
    attested_run = next(
        (
            run
            for run in record["delegated_runs"]
            if run["run_id"] == attestation["run_id"]
        ),
        None,
    )
    attested_dispatch_entry = dispatches.get(attestation["dispatch_id"])
    if attested_run is None or attested_dispatch_entry is None:
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence attestation run or dispatch is missing"
        )
    attested_path, attested_dispatch, attested_sha256 = attested_dispatch_entry
    attested_manifest_entries = attested_dispatch["subject_manifest"]["entries"]
    attested_manifest = {
        item["path"]: item["sha256"] for item in attested_manifest_entries
    }
    attested_subject = "sha256:" + hashlib.sha256(
        "".join(
            f"{item['path']}\0{item['sha256']}\n"
            for item in attested_manifest_entries
        ).encode()
    ).hexdigest()
    if (
        attested_run["dispatch_id"] != attestation["dispatch_id"]
        or attested_run["role"] != "review"
        or attested_run["final_status"] != "completed"
        or attested_run["verdict"] != "clean"
        or attested_run["subject_fingerprint"] != attested_subject
        or attested_run["result_subject_fingerprint"] != attested_subject
        or attested_dispatch["subject_fingerprint"] != attested_subject
        or attested_dispatch["subject_manifest"]["aggregate_fingerprint"]
        != attested_subject
        or attestation["dispatch_ref"]
        != attested_path.relative_to(repo_root).as_posix()
        or attestation["dispatch_sha256"] != f"sha256:{attested_sha256}"
        or attested_manifest.get(evidence_ref["path"]) != evidence_ref["sha256"]
    ):
        raise ValidationFailure(
            f"{record_path}: GitNexus evidence lacks a completed CLEAN same-handoff attestation"
        )


def validate_v1_4_handoff_chains(
    records: list[tuple[Path, dict[str, Any]]],
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
) -> None:
    validate_authority_continuation_chains(records, dispatches)
    v1_4_records = [
        (path, record)
        for path, record in records
        if record.get("schema_version") == "1.4"
    ]
    record_by_id = {
        record["handoff_id"]: (path, record) for path, record in v1_4_records
    }
    groups: dict[str, list[tuple[Path, dict[str, Any]]]] = {}
    for path, record in v1_4_records:
        validate_v1_4_dispatch_population(record, path, dispatches)
        groups.setdefault(record["orchestration_id"], []).append((path, record))

        dual_links = set(record["source_handoff_ids"]) & set(
            record["supersedes"]
        )
        for predecessor_id in dual_links:
            predecessor_entry = record_by_id.get(predecessor_id)
            if predecessor_entry is None:
                continue
            _, predecessor = predecessor_entry
            if (
                record["packet_id"] == predecessor["packet_id"]
                and record["orchestration_id"]
                != predecessor["orchestration_id"]
            ):
                raise ValidationFailure(
                    f"{path}: same-packet successor {record['handoff_id']!r} "
                    "cannot change parent orchestration"
                )

    identity_fields = (
        "program_id",
        "phase_id",
        "slice_id",
        "packet_id",
        "orchestration_id",
    )
    for orchestration_id, group in groups.items():
        ordered = sorted(
            group,
            key=lambda item: (
                parse_utc_timestamp(item[1]["created_at_utc"], item[0]),
                item[1]["handoff_id"],
            ),
        )
        group_ids = {record["handoff_id"] for _, record in ordered}
        first_path, first_record = ordered[0]
        selected_identity = tuple(
            first_record[field] for field in identity_fields
        )
        causal_identity: tuple[tuple[str, str], ...] | None = None

        for index, (path, record) in enumerate(ordered):
            if tuple(record[field] for field in identity_fields) != selected_identity:
                raise ValidationFailure(
                    f"{path}: same-orchestration successor "
                    f"{record['handoff_id']!r} changed selected identity"
                )

            same_parent_sources = set(record["source_handoff_ids"]) & group_ids
            same_parent_supersedes = set(record["supersedes"]) & group_ids
            if same_parent_sources != same_parent_supersedes:
                raise ValidationFailure(
                    f"{path}: same-orchestration successor "
                    f"{record['handoff_id']!r} must name its predecessor in "
                    "both source_handoff_ids and supersedes"
                )
            expected_predecessors = (
                set()
                if index == 0
                else {ordered[index - 1][1]["handoff_id"]}
            )
            if same_parent_sources != expected_predecessors:
                raise ValidationFailure(
                    f"{path}: same-orchestration successor "
                    f"{record['handoff_id']!r} does not directly continue the "
                    "immediately preceding handoff"
                )

            cutoff = parse_utc_timestamp(record["created_at_utc"], path)
            prefix_dispatches = [
                dispatch
                for dispatch_path, dispatch, _ in dispatches.values()
                if dispatch["parent_orchestration_id"] == orchestration_id
                and parse_utc_timestamp(
                    dispatch["created_at_utc"], dispatch_path
                )
                <= cutoff
            ]
            record_causal_identity = tuple(
                sorted(
                    {
                        (
                            dispatch["causal_control"]["integrated_outcome_id"],
                            dispatch["causal_control"]["causal_budget_id"],
                        )
                        for dispatch in prefix_dispatches
                    }
                )
            )
            if causal_identity is None:
                causal_identity = record_causal_identity
            elif record_causal_identity != causal_identity:
                raise ValidationFailure(
                    f"{path}: same-orchestration successor "
                    f"{record['handoff_id']!r} changed integrated outcome or "
                    "causal-budget identity"
                )

            future_dispatches = sorted(
                dispatch["dispatch_id"]
                for dispatch_path, dispatch, _ in dispatches.values()
                if dispatch["parent_orchestration_id"] == orchestration_id
                and parse_utc_timestamp(
                    dispatch["created_at_utc"], dispatch_path
                )
                > cutoff
            )
            has_successor = index + 1 < len(ordered)
            if record["status"] == "completed" and (
                has_successor or future_dispatches
            ):
                raise ValidationFailure(
                    f"{path}: completed v1.4 handoff "
                    f"{record['handoff_id']!r} is terminal for its orchestration"
                )
            if not has_successor and future_dispatches:
                raise ValidationFailure(
                    f"{path}: latest unsuperseded v1.4 handoff "
                    f"{record['handoff_id']!r} has later parent dispatches: "
                    f"{future_dispatches!r}"
                )


def validate_v1_4_ancillary_observations(
    record: dict[str, Any],
    record_path: Path,
    dispatches: dict[str, tuple[Path, dict[str, Any], str]],
    *,
    repo_root: Path = REPO_ROOT,
) -> None:
    handoff_cutoff = parse_utc_timestamp(record["created_at_utc"], record_path)
    parent_dispatches = [
        dispatch
        for dispatch_path, dispatch, _ in dispatches.values()
        if dispatch["parent_orchestration_id"] == record["orchestration_id"]
        and parse_utc_timestamp(
            dispatch["created_at_utc"],
            dispatch_path,
        )
        <= handoff_cutoff
    ]
    if record["status"] == "completed":
        completed_reviews = [
            run
            for run in record["delegated_runs"]
            if run["role"] == "review" and run["final_status"] == "completed"
        ]
        if not completed_reviews or completed_reviews[-1]["verdict"] != "clean":
            raise ValidationFailure(
                f"{record_path}: completed ancillary closeout lacks a final "
                "clean review cutoff"
            )
        final_review_dispatch = dispatches[
            completed_reviews[-1]["dispatch_id"]
        ][1]
        final_review_cutoff = (
            parse_utc_timestamp(
                final_review_dispatch["created_at_utc"],
                record_path,
            ),
            final_review_dispatch["dispatch_id"],
        )
        for dispatch in parent_dispatches:
            if (
                (
                    parse_utc_timestamp(
                        dispatch["created_at_utc"],
                        record_path,
                    ),
                    dispatch["dispatch_id"],
                )
                > final_review_cutoff
                and dispatch["ancillary_allowance"]["kind"] != "none"
            ):
                raise ValidationFailure(
                    f"{record_path}: dispatch {dispatch['dispatch_id']!r} "
                    "attempts to introduce or widen ancillary authority after "
                    "the final clean review"
                )
        parent_dispatches = [
            dispatch
            for dispatch in parent_dispatches
            if (
                parse_utc_timestamp(
                    dispatch["created_at_utc"],
                    record_path,
                ),
                dispatch["dispatch_id"],
            )
            <= final_review_cutoff
        ]
    authorized: dict[tuple[str, str], tuple[str, int]] = {}
    baseline_ceilings: dict[str, tuple[int, int]] = {}
    for dispatch in parent_dispatches:
        allowance = dispatch["ancillary_allowance"]
        if allowance["kind"] == "none":
            continue
        baseline_ref = allowance["baseline_ref"]
        prior_path_ceiling, prior_line_ceiling = baseline_ceilings.get(
            baseline_ref,
            (0, 0),
        )
        baseline_ceilings[baseline_ref] = (
            max(prior_path_ceiling, allowance["max_paths"]),
            max(prior_line_ceiling, allowance["max_changed_lines"]),
        )
        for entry in allowance["entries"]:
            key = (baseline_ref, entry["path"])
            prior_kind, prior_changed_lines = authorized.get(
                key,
                (entry["path_kind"], 0),
            )
            if prior_kind != entry["path_kind"]:
                raise ValidationFailure(
                    f"{record_path}: ancillary path {entry['path']!r} changed "
                    "its typed kind within one baseline"
                )
            authorized[key] = (
                entry["path_kind"],
                max(prior_changed_lines, entry["changed_lines"]),
            )

    observations = record["ancillary_diff_observations"]
    actual_keys = [
        (observation["baseline_ref"], observation["path"])
        for observation in observations
    ]
    if actual_keys != sorted(actual_keys) or len(actual_keys) != len(
        set(actual_keys)
    ):
        raise ValidationFailure(
            f"{record_path}: ancillary observations must be unique and sorted"
        )
    if set(actual_keys) != set(authorized):
        missing = sorted(set(authorized) - set(actual_keys))
        extra = sorted(set(actual_keys) - set(authorized))
        raise ValidationFailure(
            f"{record_path}: ancillary observations do not reconcile bounded "
            f"dispatch scope: missing={missing}, extra={extra}"
        )
    for observation in observations:
        key = (observation["baseline_ref"], observation["path"])
        authorized_kind, authorized_changed_lines = authorized[key]
        if observation["path_kind"] != authorized_kind:
            raise ValidationFailure(
                f"{record_path}: ancillary observation kind mismatch for "
                f"{observation['path']}"
            )
        observed_lines = observed_git_changed_lines(
            observation["baseline_ref"],
            observation["path"],
            repo_root=repo_root,
            target_ref=record["repo_state"]["head"],
        )
        if observation["changed_lines"] != observed_lines:
            raise ValidationFailure(
                f"{record_path}: ancillary closeout count for "
                f"{observation['path']} is "
                f"declared={observation['changed_lines']} "
                f"observed={observed_lines}"
            )
        if observed_lines > authorized_changed_lines:
            raise ValidationFailure(
                f"{record_path}: ancillary closeout count for "
                f"{observation['path']} exceeds the largest reviewed count: "
                f"authorized={authorized_changed_lines} "
                f"observed={observed_lines}"
            )
    for baseline_ref, (path_ceiling, line_ceiling) in baseline_ceilings.items():
        baseline_observations = [
            observation
            for observation in observations
            if observation["baseline_ref"] == baseline_ref
        ]
        if len(baseline_observations) > path_ceiling:
            raise ValidationFailure(
                f"{record_path}: ancillary closeout path count exceeds the "
                f"reviewed ceiling for {baseline_ref!r}"
            )
        observed_total = sum(
            observation["changed_lines"]
            for observation in baseline_observations
        )
        if observed_total > line_ceiling:
            raise ValidationFailure(
                f"{record_path}: ancillary closeout changed-line count exceeds "
                f"the reviewed ceiling for {baseline_ref!r}: "
                f"authorized={line_ceiling} observed={observed_total}"
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
            record["schema_version"] in {"1.3", "1.4"}
            and not legacy_predecessor_closeout
            and dispatch["schema_version"] != record["schema_version"]
        ):
            raise ValidationFailure(
                f"{record_path}: new v{record['schema_version']} closeout run "
                f"{run_id!r} must use internal-dispatch "
                f"v{record['schema_version']}"
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
            "role": run["role"],
            "subject_fingerprint": run["subject_fingerprint"],
        }
        if record["schema_version"] != "1.4":
            matching_fields["packet_id"] = record["packet_id"]
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
        if (
            run["role"] == "review"
            and run["final_status"] == "completed"
            and run["result_subject_fingerprint"]
            != run["subject_fingerprint"]
        ):
            raise ValidationFailure(
                f"{record_path}: read-only review run {run_id!r} changed its subject"
            )

    if record["schema_version"] == "1.4":
        validate_v1_4_dispatch_population(record, record_path, dispatches)
        validate_v1_4_ancillary_observations(
            record,
            record_path,
            dispatches,
        )
        validate_v1_4_causal_sequence(
            record_path,
            runs,
            dispatch_by_run_id,
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
    if record["schema_version"] in {"1.3", "1.4"}:
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
                if run.get("carried_finding_refs"):
                    raise ValidationFailure(
                        f"{record_path}: non-review run {run['run_id']!r} "
                        "cannot carry findings"
                    )
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
            carried_finding_refs = run.get("carried_finding_refs", [])
            if carried_finding_refs and (
                run["final_status"] != "completed"
                or run["verdict"] != "findings"
            ):
                raise ValidationFailure(
                    f"{record_path}: review run {run['run_id']!r} can carry "
                    "findings only when completed with verdict findings"
                )
            if set(carried_finding_refs).intersection(run["finding_refs"]):
                raise ValidationFailure(
                    f"{record_path}: review run {run['run_id']!r} cannot both "
                    "own and carry one finding"
                )
            dispatch_cycle = dispatch_by_run_id[run["run_id"]].get(
                "review_cycle"
            )
            trigger_run_ids = (
                dispatch_cycle["trigger_run_ids"]
                if carried_finding_refs and dispatch_cycle is not None
                else []
            )
            for finding_id in carried_finding_refs:
                finding = findings_by_id.get(finding_id)
                if finding is None:
                    raise ValidationFailure(
                        f"{record_path}: review run {run['run_id']!r} carries "
                        f"unknown finding {finding_id!r}"
                    )
                owner_run_id = finding["source_run_id"]
                if (
                    finding["priority"] not in {"P1", "P2"}
                    or owner_run_id == run["run_id"]
                    or owner_run_id not in trigger_run_ids
                    or run_order[owner_run_id] >= run_order[run["run_id"]]
                ):
                    raise ValidationFailure(
                        f"{record_path}: review run {run['run_id']!r} has "
                        f"invalid carried finding {finding_id!r}"
                    )
                if not any(
                    remediation["finding_run_id"] == owner_run_id
                    and remediation["re_review_run_id"] == run["run_id"]
                    and remediation["status"] == "completed"
                    for remediation in remediations
                ):
                    raise ValidationFailure(
                        f"{record_path}: review run {run['run_id']!r} carries "
                        f"finding {finding_id!r} without exact remediation lineage"
                    )
            linked_findings = findings_by_run.get(run["run_id"], [])
            has_blocking = any(
                finding["priority"] in {"P1", "P2"}
                for finding in linked_findings
            ) or bool(carried_finding_refs)
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

        if record["schema_version"] == "1.3":
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
        else:
            review_groups = sorted(
                {
                    (
                        dispatch_by_run_id[run["run_id"]]["causal_control"][
                            "causal_budget_id"
                        ],
                        dispatch_by_run_id[run["run_id"]]["causal_control"][
                            "review_stage"
                        ],
                    )
                    for run in runs
                    if run["role"] == "review"
                }
            )
            for causal_budget_id, review_stage in review_groups:
                grouped_runs = [
                    run
                    for run in runs
                    if (
                        dispatch_by_run_id[run["run_id"]][
                            "causal_control"
                        ]["causal_budget_id"],
                        dispatch_by_run_id[run["run_id"]][
                            "causal_control"
                        ]["review_stage"],
                    )
                    == (causal_budget_id, review_stage)
                ]
                grouped_run_ids = {
                    run["run_id"] for run in grouped_runs
                }
                grouped_remediations = [
                    remediation
                    for remediation in remediations
                    if remediation["finding_run_id"] in grouped_run_ids
                ]
                validate_review_cycles(
                    record_path,
                    grouped_runs,
                    run_by_id,
                    run_order,
                    dispatch_by_run_id,
                    findings_by_id,
                    grouped_remediations,
                    require_typed=True,
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
            else record["schema_version"]
            if record["schema_version"] in {"1.3", "1.4"}
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
    through v1.4 dispatches intentionally bind manifests that reach the wider
    repository, so they cannot be validated inside that reduced fixture. The
    test is about byte admission for historical records and dispatches; remove
    v1.2/v1.3/v1.4 records and v1.1/v1.2/v1.3/v1.4 dispatches before
    exercising it.
    """
    for path in (temp_root / "records").glob("*.json"):
        if load_json(path).get("schema_version") in {"1.2", "1.3", "1.4"}:
            path.unlink()
    for path in (temp_root / "dispatches").glob("*.json"):
        if load_json(path).get("schema_version") in {
            "1.1",
            "1.2",
            "1.3",
            "1.4",
        }:
            path.unlink()
    remaining_record_versions = {
        load_json(path).get("schema_version")
        for path in (temp_root / "records").glob("*.json")
    }
    remaining_dispatch_versions = {
        load_json(path).get("schema_version")
        for path in (temp_root / "dispatches").glob("*.json")
    }
    if remaining_record_versions & {"1.2", "1.3", "1.4"}:
        raise ValidationFailure(
            "historical admission fixture retained a current-protocol record"
        )
    if remaining_dispatch_versions & {"1.1", "1.2", "1.3", "1.4"}:
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


def run_v1_4_causal_contract_self_test() -> int:
    try:
        parse_utc_timestamp("2026-07-28T03:00:00+01:00", Path("offset.json"))
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: non-canonical UTC "
            "offset timestamp unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    continuation_schema_fixture = load_dispatch_template_fixture()
    continuation_schema_fixture["authority_continuation"] = {
        "grant": {
            "extension_id": "authority-continuation-self-test",
            "predecessor_handoff_id": "authority-stop-self-test",
            "predecessor_dispatch_population_fingerprint": "sha256:" + "1" * 64,
            "parent_orchestration_id": "authority-parent-self-test",
            "integrated_outcome_id": "authority-outcome-self-test",
            "outcome_registry_fingerprint": "sha256:" + "2" * 64,
            "causal_budget_id": "sha256:" + "3" * 64,
            "packet_ids": ["authority-packet-self-test"],
            "authority_ref": {
                "path": "docs/authority/self-test.json",
                "sha256": "4" * 64,
                "issuer": {"role": "product_authority", "identity": "issuer-self-test"},
                "owner": {"role": "lineage_owner", "identity": "owner-self-test"},
                "issued_at_utc": "2026-07-28T00:00:00Z",
                "source": {
                    "task_ref": "task-self-test",
                    "thread_ref": "thread-self-test",
                    "host_ref": "host-self-test",
                    "dispatch_nonce": "nonce-self-test"
                },
                "attestation": {
                    "handoff_id": "attestation-handoff-self-test",
                    "run_id": "attestation-run-self-test",
                    "dispatch_id": "attestation-dispatch-self-test",
                    "dispatch_ref": "docs/attestation/self-test.json",
                    "dispatch_sha256": "sha256:" + "5" * 64
                }
            },
            "baseline_commit": "a" * 40,
            "baseline_tree": "b" * 40,
            "subject_path_ceiling": [
                "docs/attestation/self-test.json",
                "docs/authority/self-test.json",
            ],
            "symbol_delta": [{"symbol": "self_test_symbol", "risk": "LOW"}],
            "risk_ceiling": "LOW",
            "scope_delta": "self-test authority continuation only",
            "review_allowance": [
                {"review_slot": slot, "max_cycles": 4}
                for slot in (
                    "authority_admission", "implementation", "proof", "final_closeout"
                )
            ]
        },
        "grant_fingerprint": "sha256:" + "6" * 64,
        "review_slot": "authority_admission"
    }
    try:
        validate_instance(
            continuation_schema_fixture,
            load_json(INTERNAL_DISPATCH_SCHEMA_PATHS["1.4"]),
            "authority continuation dispatch schema fixture",
        )
    except ValidationFailure as error:
        print(
            "orchestration contract self-test failed: authority continuation "
            f"dispatch schema rejected the positive fixture: {error}",
            file=sys.stderr,
        )
        return 1

    golden_value = {"a": 1, "emoji": "😀", "z": [True, None, "x"]}
    if derive_authority_grant_fingerprint(golden_value) != (
        "sha256:484926e249d6a3720a2b97f286faf125855a3bf35ee6fc7f0e8d67b3b8de1d32"
    ):
        print(
            "orchestration contract self-test failed: authority grant JCS golden vector drifted",
            file=sys.stderr,
        )
        return 1
    fixture_grant = continuation_schema_fixture["authority_continuation"]["grant"]
    fixture_fingerprint = derive_authority_grant_fingerprint(fixture_grant)
    for field in fixture_grant:
        mutated = copy.deepcopy(fixture_grant)
        value = mutated[field]
        if isinstance(value, str):
            mutated[field] = value + "-mutated"
        elif isinstance(value, list):
            mutated[field] = [*value, copy.deepcopy(value[-1])]
        else:
            mutated[field] = {**value, "mutation": True}
        if derive_authority_grant_fingerprint(mutated) == fixture_fingerprint:
            print(
                "orchestration contract self-test failed: authority grant stable "
                f"field {field!r} did not affect its fingerprint",
                file=sys.stderr,
            )
            return 1
    slot_variant = copy.deepcopy(continuation_schema_fixture["authority_continuation"])
    slot_variant["review_slot"] = "proof"
    if derive_authority_grant_fingerprint(slot_variant["grant"]) != fixture_fingerprint:
        print(
            "orchestration contract self-test failed: per-dispatch slot changed immutable grant fingerprint",
            file=sys.stderr,
        )
        return 1

    with tempfile.TemporaryDirectory() as temp_dir:
        authority_root = Path(temp_dir)
        authority_path = authority_root / fixture_grant["authority_ref"]["path"]
        authority_path.parent.mkdir(parents=True)
        artifact = authority_artifact_from_grant(fixture_grant)
        authority_bytes = (json.dumps(artifact, indent=2, ensure_ascii=False) + "\n").encode()
        authority_path.write_bytes(authority_bytes)
        fixture_grant["authority_ref"]["sha256"] = hashlib.sha256(authority_bytes).hexdigest()
        continuation_schema_fixture["authority_continuation"]["grant_fingerprint"] = (
            derive_authority_grant_fingerprint(fixture_grant)
        )
        continuation_schema_fixture["parent_orchestration_id"] = fixture_grant[
            "parent_orchestration_id"
        ]
        continuation_schema_fixture["packet_id"] = fixture_grant["packet_ids"][0]
        continuation_schema_fixture["causal_outcome_registry"]["outcomes"] = [
            {
                "integrated_outcome_id": fixture_grant["integrated_outcome_id"],
                "packet_ids": fixture_grant["packet_ids"],
                "authority_ref": "self-test",
            }
        ]
        continuation_schema_fixture["causal_control"].update(
            {
                "causal_budget_id": fixture_grant["causal_budget_id"],
                "integrated_outcome_id": fixture_grant["integrated_outcome_id"],
                "outcome_registry_fingerprint": fixture_grant[
                    "outcome_registry_fingerprint"
                ],
                "review_stage": "authority_admission",
                "stage_transition": {"kind": "authority_extension", "from_stage": "proof"},
            }
        )
        continuation_schema_fixture["created_at_utc"] = "2026-07-28T00:01:00Z"
        continuation_schema_fixture["subject_manifest"]["entries"] = [
            {
                "path": fixture_grant["authority_ref"]["path"],
                "sha256": fixture_grant["authority_ref"]["sha256"],
            },
            {
                "path": fixture_grant["authority_ref"]["attestation"][
                    "dispatch_ref"
                ],
                "sha256": fixture_grant["authority_ref"]["attestation"][
                    "dispatch_sha256"
                ].removeprefix("sha256:"),
            },
        ]
        try:
            validate_authority_continuation_dispatch(
                continuation_schema_fixture,
                Path("authority-positive.json"),
                repo_root=authority_root,
            )
        except ValidationFailure as error:
            print(
                "orchestration contract self-test failed: authority continuation positive "
                f"fixture rejected: {error}",
                file=sys.stderr,
            )
            return 1

        dispatch_negative_cases: list[tuple[str, dict[str, Any]]] = []
        stale = copy.deepcopy(continuation_schema_fixture)
        stale["subject_manifest"]["entries"][0]["sha256"] = "0" * 64
        dispatch_negative_cases.append(("stale-authority", stale))
        missing_review = copy.deepcopy(continuation_schema_fixture)
        missing_review["subject_manifest"]["entries"] = missing_review[
            "subject_manifest"
        ]["entries"][:1]
        dispatch_negative_cases.append(("missing-authority-review", missing_review))
        postdated = copy.deepcopy(continuation_schema_fixture)
        postdated["authority_continuation"]["grant"]["authority_ref"]["issued_at_utc"] = postdated["created_at_utc"]
        postdated["authority_continuation"]["grant_fingerprint"] = derive_authority_grant_fingerprint(postdated["authority_continuation"]["grant"])
        dispatch_negative_cases.append(("postdated-authority", postdated))
        drift = copy.deepcopy(continuation_schema_fixture)
        drift["authority_continuation"]["grant"]["causal_budget_id"] = "sha256:" + "9" * 64
        drift["authority_continuation"]["grant_fingerprint"] = derive_authority_grant_fingerprint(drift["authority_continuation"]["grant"])
        dispatch_negative_cases.append(("identity-drift", drift))
        parity = copy.deepcopy(continuation_schema_fixture)
        parity["authority_continuation"]["grant"]["scope_delta"] += " drift"
        parity["authority_continuation"]["grant_fingerprint"] = derive_authority_grant_fingerprint(parity["authority_continuation"]["grant"])
        dispatch_negative_cases.append(("authority-parity-drift", parity))
        for label, value in dispatch_negative_cases:
            try:
                validate_authority_continuation_dispatch(
                    value, Path(f"{label}.json"), repo_root=authority_root
                )
            except ValidationFailure:
                pass
            else:
                print(
                    "orchestration contract self-test failed: authority continuation "
                    f"negative {label!r} unexpectedly validated",
                    file=sys.stderr,
                )
                return 1

    exact_predecessor_path = RECORDS_DIR / (
        "20260802T012613Z--HCM-3-2--orchestration--"
        "authority-lineage-composition-required.json"
    )
    exact_predecessor = load_json(exact_predecessor_path)
    exact_dispatches: dict[str, tuple[Path, dict[str, Any], str]] = {}
    exact_dispatch_by_run: dict[str, dict[str, Any]] = {}
    for predecessor_run in exact_predecessor["delegated_runs"]:
        predecessor_dispatch_path = REPO_ROOT / predecessor_run["dispatch_ref"]
        predecessor_dispatch = load_json(predecessor_dispatch_path)
        exact_dispatches[predecessor_dispatch["dispatch_id"]] = (
            predecessor_dispatch_path,
            predecessor_dispatch,
            hashlib.sha256(predecessor_dispatch_path.read_bytes()).hexdigest(),
        )
        exact_dispatch_by_run[predecessor_run["run_id"]] = predecessor_dispatch

    exact_authority_relative = (
        "docs/specs/handbook-contract-membrane/slices/HCM-0.8/authority/"
        "20260802-hcm-3-2-lineage-composition-authority.json"
    )
    exact_authority_path = REPO_ROOT / exact_authority_relative
    exact_artifact = load_json(exact_authority_path)
    exact_authority_sha256 = hashlib.sha256(exact_authority_path.read_bytes()).hexdigest()
    attestation_dispatch_path = REPO_ROOT / (
        "docs/specs/handbook-contract-membrane/handoffs/dispatches/"
        "20260802T060000Z--HCM-0-8--authority-continuation-final-aggregate-supplemental.json"
    )
    attestation_dispatch = copy.deepcopy(
        exact_dispatch_by_run[exact_predecessor["delegated_runs"][-1]["run_id"]]
    )
    attestation_entries = [
        {"path": exact_authority_relative, "sha256": exact_authority_sha256}
    ]
    attestation_subject = "sha256:" + hashlib.sha256(
        "".join(
            f"{item['path']}\0{item['sha256']}\n" for item in attestation_entries
        ).encode()
    ).hexdigest()
    attestation_dispatch.update(
        {
            "dispatch_id": "20260802T060000Z--HCM-0-8--authority-continuation-final-aggregate-supplemental",
            "created_at_utc": "2026-08-02T06:00:00Z",
            "parent_orchestration_id": "20260802T025200Z--HCM-0-8--post-clean-authority-continuation",
            "subject_fingerprint": attestation_subject,
            "subject_manifest": {
                "algorithm": "sha256",
                "encoding": "repo-path-null-sha256-newline-v1",
                "entries": attestation_entries,
                "aggregate_fingerprint": attestation_subject,
            },
        }
    )
    attestation_dispatch_sha256 = hashlib.sha256(
        (json.dumps(attestation_dispatch, separators=(",", ":")) + "\n").encode()
    ).hexdigest()
    exact_dispatches[attestation_dispatch["dispatch_id"]] = (
        attestation_dispatch_path,
        attestation_dispatch,
        attestation_dispatch_sha256,
    )
    attestation_handoff = {
        "schema_version": "1.4",
        "handoff_id": "20260802T060100Z--HCM-0-8--orchestration--authority-continuation-completed",
        "created_at_utc": "2026-08-02T06:01:00Z",
        "status": "completed",
        "orchestration_id": "20260802T025200Z--HCM-0-8--post-clean-authority-continuation",
        "delegated_runs": [
            {
                "run_id": "hcm08_authority_continuation_final_review",
                "dispatch_id": attestation_dispatch["dispatch_id"],
                "role": "review",
                "agent_id": "attestation-reviewer-self-test",
                "final_status": "completed",
                "verdict": "clean",
                "subject_fingerprint": attestation_dispatch["subject_fingerprint"],
                "result_subject_fingerprint": attestation_dispatch["subject_fingerprint"],
            }
        ],
    }
    exact_grant = {
        key: copy.deepcopy(value)
        for key, value in exact_artifact.items()
        if key not in {"schema_id", "schema_version"}
    }
    exact_grant["authority_ref"] = {
        "path": exact_authority_relative,
        "sha256": exact_authority_sha256,
        **exact_grant["authority_ref"],
        "attestation": {
            "handoff_id": attestation_handoff["handoff_id"],
            "run_id": attestation_handoff["delegated_runs"][0]["run_id"],
            "dispatch_id": attestation_dispatch["dispatch_id"],
            "dispatch_ref": attestation_dispatch_path.relative_to(REPO_ROOT).as_posix(),
            "dispatch_sha256": "sha256:" + attestation_dispatch_sha256,
        },
    }
    exact_grant_fingerprint = derive_authority_grant_fingerprint(exact_grant)

    predecessor_last_dispatch = exact_dispatch_by_run[
        exact_predecessor["delegated_runs"][-1]["run_id"]
    ]

    def exact_continuation_dispatch(
        *,
        dispatch_id: str,
        created_at_utc: str,
        packet_id: str,
        role: str,
        slot: str,
        predecessor_dispatch: dict[str, Any],
    ) -> dict[str, Any]:
        value = copy.deepcopy(predecessor_last_dispatch)
        review_cycle = None
        if role == "review":
            review_cycle = {
                "kind": "discovery",
                "cycle_id": f"hcm-3.2-authority-{slot}-discovery",
                "trigger_run_ids": [],
                "finding_refs": [],
            }
        entries = sorted([
            {"path": exact_authority_relative, "sha256": exact_authority_sha256},
            {
                "path": attestation_dispatch_path.relative_to(REPO_ROOT).as_posix(),
                "sha256": attestation_dispatch_sha256,
            },
        ], key=lambda item: item["path"])
        aggregate = "sha256:" + hashlib.sha256(
            "".join(f"{item['path']}\0{item['sha256']}\n" for item in entries).encode()
        ).hexdigest()
        prior_stage = predecessor_dispatch["causal_control"]["review_stage"]
        value.update(
            {
                "dispatch_id": dispatch_id,
                "created_at_utc": created_at_utc,
                "parent_orchestration_id": exact_grant["parent_orchestration_id"],
                "packet_id": packet_id,
                "role": role,
                "review_cycle": review_cycle,
                "pre_review_convergence": (
                    copy.deepcopy(predecessor_last_dispatch["pre_review_convergence"])
                    if role == "review"
                    else None
                ),
                "subject_fingerprint": aggregate,
                "subject_manifest": {
                    "algorithm": "sha256",
                    "encoding": "repo-path-null-sha256-newline-v1",
                    "entries": entries,
                    "aggregate_fingerprint": aggregate,
                },
                "authority_continuation": {
                    "grant": copy.deepcopy(exact_grant),
                    "grant_fingerprint": exact_grant_fingerprint,
                    "review_slot": slot,
                },
            }
        )
        value["causal_control"] = {
            "causal_budget_id": exact_grant["causal_budget_id"],
            "integrated_outcome_id": exact_grant["integrated_outcome_id"],
            "outcome_registry_fingerprint": exact_grant["outcome_registry_fingerprint"],
            "review_stage": slot,
            "stage_transition": {
                "kind": "authority_extension" if slot == "authority_admission" else "enter",
                "from_stage": prior_stage,
            },
            "event_reason": "planned_stage_transition",
            "causal_predecessor_dispatch_id": predecessor_dispatch["dispatch_id"],
        }
        return value

    exact_admission = exact_continuation_dispatch(
        dispatch_id="20260802T060200Z--HCM-3-2--authority-admission",
        created_at_utc="2026-08-02T06:02:00Z",
        packet_id="HCM-3.2-P2-kernel-implementation",
        role="review",
        slot="authority_admission",
        predecessor_dispatch=predecessor_last_dispatch,
    )
    exact_implementation = exact_continuation_dispatch(
        dispatch_id="20260802T060300Z--HCM-3-2--authority-implementation",
        created_at_utc="2026-08-02T06:03:00Z",
        packet_id="HCM-3.2-P2-kernel-implementation",
        role="implementation",
        slot="implementation",
        predecessor_dispatch=exact_admission,
    )
    exact_proof = exact_continuation_dispatch(
        dispatch_id="20260802T060400Z--HCM-3-2--authority-proof",
        created_at_utc="2026-08-02T06:04:00Z",
        packet_id="HCM-3.2-P3-proof-control-closeout",
        role="review",
        slot="proof",
        predecessor_dispatch=exact_implementation,
    )
    exact_final = exact_continuation_dispatch(
        dispatch_id="20260802T060500Z--HCM-3-2--authority-final-closeout",
        created_at_utc="2026-08-02T06:05:00Z",
        packet_id="HCM-3.2-P3-proof-control-closeout",
        role="review",
        slot="final_closeout",
        predecessor_dispatch=exact_proof,
    )
    exact_continuation_values = [
        exact_admission,
        exact_implementation,
        exact_proof,
        exact_final,
    ]
    exact_continuation_runs: list[dict[str, Any]] = []
    exact_agent_ids = [
        "admission-reviewer-self-test",
        "continuation-executor-self-test",
        "proof-reviewer-self-test",
        "final-reviewer-self-test",
    ]
    for index, (value, agent_id) in enumerate(
        zip(exact_continuation_values, exact_agent_ids, strict=True), start=1
    ):
        dispatch_path = DISPATCHES_DIR / f"{value['dispatch_id']}.json"
        dispatch_sha256 = hashlib.sha256(
            (json.dumps(value, separators=(",", ":")) + "\n").encode()
        ).hexdigest()
        exact_dispatches[value["dispatch_id"]] = (
            dispatch_path,
            value,
            dispatch_sha256,
        )
        exact_continuation_runs.append(
            {
                "run_id": f"hcm32-authority-run-{index}",
                "dispatch_id": value["dispatch_id"],
                "role": value["role"],
                "agent_id": agent_id,
                "final_status": "completed",
                "verdict": "clean" if value["role"] == "review" else "not_applicable",
                "subject_fingerprint": value["subject_fingerprint"],
                "result_subject_fingerprint": value["subject_fingerprint"],
                "finding_refs": [],
            }
        )
        exact_dispatch_by_run[exact_continuation_runs[-1]["run_id"]] = value

    exact_temp_directory = tempfile.TemporaryDirectory(
        prefix="hcm-3-2-authority-evidence-"
    )
    exact_repo_root = Path(exact_temp_directory.name)
    subprocess.run(
        [
            "git",
            "clone",
            "-q",
            "--shared",
            "--no-checkout",
            str(REPO_ROOT),
            str(exact_repo_root),
        ],
        check=True,
    )
    subprocess.run(
        ["git", "checkout", "-q", exact_grant["baseline_commit"]],
        cwd=exact_repo_root,
        check=True,
    )
    subprocess.run(
        ["git", "config", "user.name", "HCM self-test"],
        cwd=exact_repo_root,
        check=True,
    )
    subprocess.run(
        ["git", "config", "user.email", "hcm-self-test@example.invalid"],
        cwd=exact_repo_root,
        check=True,
    )
    exact_code_path = "crates/engine/src/artifact_lineage_store.rs"
    exact_code_file = exact_repo_root / exact_code_path
    exact_code_file.write_text(
        exact_code_file.read_text(encoding="utf-8")
        + "\n// HCM authority-continuation GitNexus evidence fixture.\n",
        encoding="utf-8",
    )
    subprocess.run(
        ["git", "add", exact_code_path], cwd=exact_repo_root, check=True
    )
    subprocess.run(
        ["git", "commit", "-qm", "self-test code-bearing continuation"],
        cwd=exact_repo_root,
        check=True,
    )
    exact_target_commit = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=exact_repo_root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()
    exact_target_tree = subprocess.run(
        ["git", "rev-parse", "HEAD^{tree}"],
        cwd=exact_repo_root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()
    exact_evidence_relative = (
        "docs/specs/handbook-contract-membrane/slices/HCM-3.2/proof/"
        "implementation/authority-continuation-gitnexus-change-detection.json"
    )
    exact_changed_symbols = [
        {"symbol": "ContextResolutionStackDefinition", "risk": "HIGH"},
        {
            "symbol": "ContextResolutionStackDefinition::load_bytes",
            "risk": "CRITICAL",
        },
    ]
    exact_evidence = {
        "schema_id": "handbook.gitnexus-change-detection-evidence",
        "schema_version": "1.0",
        "status": "available",
        "provider": {"name": "GitNexus", "version": "self-test-1.0.0"},
        "command": (
            "detect_changes(scope=compare,base_ref="
            f"{exact_grant['baseline_commit']},target_ref={exact_target_commit})"
        ),
        "scope": "compare",
        "baseline_commit": exact_grant["baseline_commit"],
        "baseline_tree": exact_grant["baseline_tree"],
        "target_commit": exact_target_commit,
        "target_tree": exact_target_tree,
        "actual_changed_paths": [exact_code_path],
        "changed_symbols": exact_changed_symbols,
        "observed_risk": "CRITICAL",
        "raw_output_fingerprint": "sha256:"
        + hashlib.sha256(b"self-test GitNexus raw output\n").hexdigest(),
    }
    exact_evidence_path = exact_repo_root / exact_evidence_relative
    exact_evidence_path.parent.mkdir(parents=True, exist_ok=True)
    exact_evidence_bytes = (
        json.dumps(exact_evidence, indent=2, ensure_ascii=False) + "\n"
    ).encode()
    exact_evidence_path.write_bytes(exact_evidence_bytes)
    exact_evidence_sha256 = hashlib.sha256(exact_evidence_bytes).hexdigest()
    exact_authority_copy = exact_repo_root / exact_authority_relative
    exact_authority_copy.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(exact_authority_path, exact_authority_copy)

    exact_proof["subject_manifest"]["entries"] = sorted(
        [
            *exact_proof["subject_manifest"]["entries"],
            {"path": exact_evidence_relative, "sha256": exact_evidence_sha256},
        ],
        key=lambda item: item["path"],
    )
    exact_proof_subject = "sha256:" + hashlib.sha256(
        "".join(
            f"{item['path']}\0{item['sha256']}\n"
            for item in exact_proof["subject_manifest"]["entries"]
        ).encode()
    ).hexdigest()
    exact_proof["subject_fingerprint"] = exact_proof_subject
    exact_proof["subject_manifest"]["aggregate_fingerprint"] = exact_proof_subject
    exact_proof_run = exact_continuation_runs[2]
    exact_proof_run["subject_fingerprint"] = exact_proof_subject
    exact_proof_run["result_subject_fingerprint"] = exact_proof_subject
    exact_proof_sha256 = hashlib.sha256(
        (json.dumps(exact_proof, separators=(",", ":")) + "\n").encode()
    ).hexdigest()
    exact_dispatches[exact_proof["dispatch_id"]] = (
        DISPATCHES_DIR / f"{exact_proof['dispatch_id']}.json",
        exact_proof,
        exact_proof_sha256,
    )
    exact_evidence_ref = {
        "path": exact_evidence_relative,
        "sha256": exact_evidence_sha256,
        "attestation": {
            "run_id": exact_proof_run["run_id"],
            "dispatch_id": exact_proof["dispatch_id"],
            "dispatch_ref": (
                "docs/specs/handbook-contract-membrane/handoffs/dispatches/"
                f"{exact_proof['dispatch_id']}.json"
            ),
            "dispatch_sha256": "sha256:" + exact_proof_sha256,
        },
    }

    def exact_summary(
        included_values: list[dict[str, Any]], *, consumed: bool
    ) -> dict[str, Any]:
        slots = []
        for slot in AUTHORITY_CONTINUATION_SLOTS:
            values = [
                value
                for value in included_values
                if value["authority_continuation"]["review_slot"] == slot
            ]
            if values:
                slots.append(
                    {
                        "review_slot": slot,
                        "cycle_ids": [
                            value["review_cycle"]["cycle_id"]
                            for value in values
                            if value["review_cycle"] is not None
                        ],
                        "dispatch_ids": [value["dispatch_id"] for value in values],
                    }
                )
        return {
            "extension_id": exact_grant["extension_id"],
            "grant_fingerprint": exact_grant_fingerprint,
            "predecessor_handoff_id": exact_grant["predecessor_handoff_id"],
            "selector_run_id": exact_continuation_runs[0]["run_id"],
            "baseline_commit": exact_grant["baseline_commit"],
            "baseline_tree": exact_grant["baseline_tree"],
            "slots": slots,
            "actual_changed_paths": [exact_code_path],
            "changed_symbols": copy.deepcopy(exact_changed_symbols),
            "observed_risk": "CRITICAL",
            "gitnexus_evidence": copy.deepcopy(exact_evidence_ref),
            "status": "consumed" if consumed else "active",
        }

    active_successor = {
        "schema_version": "1.4",
        "handoff_id": "20260802T060450Z--HCM-3-2--orchestration--authority-active",
        "created_at_utc": "2026-08-02T06:04:50Z",
        "status": "escalation_required",
        "orchestration_id": exact_grant["parent_orchestration_id"],
        "source_handoff_ids": [exact_predecessor["handoff_id"]],
        "supersedes": [exact_predecessor["handoff_id"]],
        "delegated_runs": exact_continuation_runs[:3],
        "repo_state": {"head": exact_target_commit},
        "authority_continuations": [
            exact_summary(exact_continuation_values[:3], consumed=False)
        ],
    }
    completed_successor = copy.deepcopy(active_successor)
    completed_successor.update(
        {
            "handoff_id": "20260802T060600Z--HCM-3-2--orchestration--authority-consumed",
            "created_at_utc": "2026-08-02T06:06:00Z",
            "status": "completed",
            "source_handoff_ids": [active_successor["handoff_id"]],
            "supersedes": [active_successor["handoff_id"]],
            "delegated_runs": exact_continuation_runs,
            "authority_continuations": [
                exact_summary(exact_continuation_values, consumed=True)
            ],
        }
    )
    exact_dispatches = {
        dispatch_id: (
            exact_repo_root / path.relative_to(REPO_ROOT),
            dispatch,
            dispatch_sha256,
        )
        for dispatch_id, (path, dispatch, dispatch_sha256) in exact_dispatches.items()
    }
    exact_records = [
        (
            exact_repo_root / exact_predecessor_path.relative_to(REPO_ROOT),
            exact_predecessor,
        ),
        (exact_repo_root / "attestation-handoff.json", attestation_handoff),
        (exact_repo_root / "active-successor.json", active_successor),
        (exact_repo_root / "completed-successor.json", completed_successor),
    ]
    for value in exact_continuation_values:
        try:
            validate_authority_continuation_dispatch(
                value,
                exact_repo_root / f"{value['dispatch_id']}.json",
                repo_root=exact_repo_root,
            )
        except ValidationFailure as error:
            print(
                "orchestration contract self-test failed: exact HCM-3.2 continuation "
                f"dispatch rejected: {error}",
                file=sys.stderr,
            )
            return 1
    exact_runs = [*exact_predecessor["delegated_runs"], *exact_continuation_runs]
    try:
        validate_v1_4_causal_sequence(
            Path("exact-hcm-3.2-authority-sequence.json"),
            exact_runs,
            exact_dispatch_by_run,
        )
        validate_authority_continuation_chains(
            exact_records, exact_dispatches, repo_root=exact_repo_root
        )
    except ValidationFailure as error:
        print(
            "orchestration contract self-test failed: exact HCM-3.2 artifact-first "
            f"continuation rejected: {error}",
            file=sys.stderr,
        )
        return 1

    def expect_evidence_negative(
        label: str,
        *,
        evidence_value: dict[str, Any] | None,
        summary_mutator: Callable[[dict[str, Any]], None],
        record_mutator: Callable[[dict[str, Any]], None] | None = None,
        expected_message: str,
    ) -> bool:
        negative_record = copy.deepcopy(completed_successor)
        negative_summary = negative_record["authority_continuations"][0]
        summary_mutator(negative_summary)
        if record_mutator is not None:
            record_mutator(negative_record)
        try:
            if evidence_value is not None:
                negative_bytes = (
                    json.dumps(evidence_value, indent=2, ensure_ascii=False) + "\n"
                ).encode()
                exact_evidence_path.write_bytes(negative_bytes)
                negative_summary["gitnexus_evidence"]["sha256"] = hashlib.sha256(
                    negative_bytes
                ).hexdigest()
            validate_gitnexus_change_detection_evidence(
                Path(f"exact-hcm-3.2-{label}.json"),
                negative_record,
                negative_summary,
                exact_grant,
                exact_dispatches,
                repo_root=exact_repo_root,
            )
        except ValidationFailure as error:
            if expected_message in str(error):
                return True
            print(
                "orchestration contract self-test failed: exact HCM-3.2 evidence "
                f"negative {label!r} failed for the wrong reason: {error}",
                file=sys.stderr,
            )
            return False
        finally:
            exact_evidence_path.write_bytes(exact_evidence_bytes)
        print(
            "orchestration contract self-test failed: exact HCM-3.2 evidence "
            f"negative {label!r} unexpectedly validated",
            file=sys.stderr,
        )
        return False

    empty_evidence = copy.deepcopy(exact_evidence)
    empty_evidence["changed_symbols"] = []
    empty_evidence["observed_risk"] = "LOW"
    omitted_symbol_evidence = copy.deepcopy(exact_evidence)
    omitted_symbol_evidence["changed_symbols"] = exact_changed_symbols[:1]
    omitted_symbol_evidence["observed_risk"] = "HIGH"
    unauthorized_evidence = copy.deepcopy(exact_evidence)
    unauthorized_evidence["changed_symbols"].append(
        {"symbol": "Unauthorized::symbol", "risk": "CRITICAL"}
    )
    understated_evidence = copy.deepcopy(exact_evidence)
    understated_evidence["observed_risk"] = "HIGH"
    evidence_negative_cases = [
        (
            "missing",
            None,
            lambda summary: summary.pop("gitnexus_evidence"),
            None,
            "lacks GitNexus evidence",
        ),
        (
            "empty-symbols",
            empty_evidence,
            lambda summary: summary.update(
                {"changed_symbols": [], "observed_risk": "LOW"}
            ),
            None,
            "empty symbol coverage",
        ),
        (
            "omitted-symbol",
            omitted_symbol_evidence,
            lambda summary: summary.update(
                {
                    "changed_symbols": copy.deepcopy(exact_changed_symbols),
                    "observed_risk": "CRITICAL",
                }
            ),
            None,
            "changed-symbol summary differ",
        ),
        (
            "unauthorized-symbol",
            unauthorized_evidence,
            lambda summary: summary.update(
                {
                    "changed_symbols": copy.deepcopy(
                        unauthorized_evidence["changed_symbols"]
                    ),
                    "observed_risk": "CRITICAL",
                }
            ),
            None,
            "symbol or risk exceeds the grant",
        ),
        (
            "understated-risk",
            understated_evidence,
            lambda summary: summary.update({"observed_risk": "HIGH"}),
            None,
            "aggregate observed risk is understated",
        ),
        (
            "unattested",
            copy.deepcopy(exact_evidence),
            lambda summary: None,
            lambda record: record["delegated_runs"][2].update(
                {"verdict": "findings"}
            ),
            "lacks a completed CLEAN same-handoff attestation",
        ),
    ]
    for (
        label,
        evidence_value,
        summary_mutator,
        record_mutator,
        expected_message,
    ) in evidence_negative_cases:
        if not expect_evidence_negative(
            label,
            evidence_value=evidence_value,
            summary_mutator=summary_mutator,
            record_mutator=record_mutator,
            expected_message=expected_message,
        ):
            return 1

    baseline_drift_dispatches = copy.deepcopy(exact_dispatches)
    for _, value, _ in baseline_drift_dispatches.values():
        continuation = value.get("authority_continuation")
        if continuation is not None:
            continuation["grant"]["baseline_commit"] = "f4e06b146fc803f45d7e0e4c2b3f9b4f7b88d3d8"
            continuation["grant"]["baseline_tree"] = "76492b8cadb0e084ffe5e3b35983ef6f69df6d28"
            continuation["grant_fingerprint"] = derive_authority_grant_fingerprint(
                continuation["grant"]
            )
    negative_chain_cases = []
    negative_chain_cases.append(
        ("predecessor-baseline-drift", exact_records, baseline_drift_dispatches)
    )
    projection_drift_dispatches = copy.deepcopy(exact_dispatches)
    projection_value = projection_drift_dispatches[exact_admission["dispatch_id"]][1]
    projection_value["authority_continuation"]["grant"]["scope_delta"] += " drift"
    projection_value["authority_continuation"]["grant_fingerprint"] = (
        derive_authority_grant_fingerprint(
            projection_value["authority_continuation"]["grant"]
        )
    )
    negative_chain_cases.append(
        ("artifact-projection-mutation", exact_records, projection_drift_dispatches)
    )
    findings_records = copy.deepcopy(exact_records)
    findings_records[1][1]["delegated_runs"][0]["verdict"] = "findings"
    negative_chain_cases.append(("attestation-findings", findings_records, exact_dispatches))
    mismatched_subject_records = copy.deepcopy(exact_records)
    mismatched_subject_records[1][1]["delegated_runs"][0][
        "result_subject_fingerprint"
    ] = "sha256:" + "f" * 64
    negative_chain_cases.append(
        ("attestation-subject-mismatch", mismatched_subject_records, exact_dispatches)
    )
    same_parent_records = copy.deepcopy(exact_records)
    same_parent_records[1][1]["orchestration_id"] = exact_grant["parent_orchestration_id"]
    negative_chain_cases.append(("same-parent-attestation", same_parent_records, exact_dispatches))
    summary_records = copy.deepcopy(exact_records)
    summary_records[-1][1]["authority_continuations"][0]["actual_changed_paths"] = [
        "outside/ceiling.txt"
    ]
    negative_chain_cases.append(("summary-path-ceiling", summary_records, exact_dispatches))
    second_successor_records = copy.deepcopy(exact_records)
    second = copy.deepcopy(completed_successor)
    second["handoff_id"] += "-second"
    second["created_at_utc"] = "2026-08-02T06:07:00Z"
    second["status"] = "escalation_required"
    second["source_handoff_ids"] = [completed_successor["handoff_id"]]
    second["supersedes"] = [completed_successor["handoff_id"]]
    second["authority_continuations"][0]["status"] = "active"
    second_successor_records.append((Path("second-successor.json"), second))
    negative_chain_cases.append(
        ("second-successor-after-consumption", second_successor_records, exact_dispatches)
    )
    for label, record_values, dispatch_values in negative_chain_cases:
        try:
            validate_authority_continuation_chains(
                record_values, dispatch_values, repo_root=exact_repo_root
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: exact HCM-3.2 negative "
                f"{label!r} unexpectedly validated",
                file=sys.stderr,
            )
            return 1

    v1_3_dispatch_entries = sorted(
        (
            path.name,
            hashlib.sha256(path.read_bytes()).hexdigest(),
        )
        for path in DISPATCHES_DIR.glob("*.json")
        if load_json(path).get("schema_version") == "1.3"
    )
    v1_3_record_entries = sorted(
        (
            path.name,
            hashlib.sha256(path.read_bytes()).hexdigest(),
        )
        for path in RECORDS_DIR.glob("*.json")
        if load_json(path).get("schema_version") == "1.3"
    )
    immutable_corpus_cases = [
        (
            "dispatch-modified",
            [
                (filename, "0" * 64 if index == 0 else entry_sha256)
                for index, (filename, entry_sha256) in enumerate(
                    v1_3_dispatch_entries
                )
            ],
            IMMUTABLE_V1_3_DISPATCH_COUNT,
            IMMUTABLE_V1_3_DISPATCH_CORPUS_FINGERPRINT,
        ),
        (
            "dispatch-deleted",
            v1_3_dispatch_entries[:-1],
            IMMUTABLE_V1_3_DISPATCH_COUNT,
            IMMUTABLE_V1_3_DISPATCH_CORPUS_FINGERPRINT,
        ),
        (
            "dispatch-added",
            [
                *v1_3_dispatch_entries,
                ("unauthorized-v1.3-dispatch.json", "1" * 64),
            ],
            IMMUTABLE_V1_3_DISPATCH_COUNT,
            IMMUTABLE_V1_3_DISPATCH_CORPUS_FINGERPRINT,
        ),
        (
            "record-modified",
            [
                (filename, "0" * 64 if index == 0 else entry_sha256)
                for index, (filename, entry_sha256) in enumerate(
                    v1_3_record_entries
                )
            ],
            IMMUTABLE_V1_3_RECORD_COUNT,
            IMMUTABLE_V1_3_RECORD_CORPUS_FINGERPRINT,
        ),
        (
            "record-deleted",
            v1_3_record_entries[:-1],
            IMMUTABLE_V1_3_RECORD_COUNT,
            IMMUTABLE_V1_3_RECORD_CORPUS_FINGERPRINT,
        ),
        (
            "record-added",
            [
                *v1_3_record_entries,
                ("unauthorized-v1.3-record.json", "1" * 64),
            ],
            IMMUTABLE_V1_3_RECORD_COUNT,
            IMMUTABLE_V1_3_RECORD_CORPUS_FINGERPRINT,
        ),
    ]
    for label, entries, expected_count, expected_fingerprint in (
        immutable_corpus_cases
    ):
        try:
            validate_immutable_corpus_entries(
                entries,
                expected_count=expected_count,
                expected_fingerprint=expected_fingerprint,
                label=label,
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: immutable v1.3 "
                f"{label} unexpectedly validated",
                file=sys.stderr,
            )
            return 1

    parent_id = "20260728T000000Z--HCM-0-8--causal-control-self-test"
    fingerprint = "sha256:" + ("1" * 64)
    p3b_evidence_paths = [
        path
        for path in sorted(DISPATCHES_DIR.glob("*p3b*.json"))
        if "20260727T021420Z" <= path.name[:16] <= "20260727T061322Z"
    ]
    p3b_evidence = [load_json(path) for path in p3b_evidence_paths]
    if len(p3b_evidence) != 13:
        print(
            "orchestration contract self-test failed: observed P3B evidence "
            f"count is {len(p3b_evidence)}, expected 13",
            file=sys.stderr,
        )
        return 1

    outcome_packets: dict[str, list[str | None]] = {
        "integrated-outcome": ["packet-a"],
        "mechanical-closeout-outcome": ["packet-a"],
        "packet-one-outcome": ["packet-one"],
        "packet-two-outcome": ["packet-two"],
        "typed-stage-outcome": [
            "implementation-packet",
            "planning-packet",
        ],
        "P3B-cli-surface-proof-integration": sorted(
            {dispatch["packet_id"] for dispatch in p3b_evidence}
        ),
    }
    registry_outcomes = [
        {
            "integrated_outcome_id": outcome_id,
            "packet_ids": packet_ids,
            "authority_ref": (
                "causal-review-budget-and-lineage-hardening.md"
                "#deterministic-proof"
            ),
        }
        for outcome_id, packet_ids in sorted(outcome_packets.items())
    ]
    registry_fingerprint = derive_outcome_registry_fingerprint(
        registry_outcomes
    )
    outcome_registry = {
        "declared_at_utc": "2026-07-27T23:59:00Z",
        "algorithm": "sha256",
        "encoding": "canonical-json-newline-v1",
        "outcomes": registry_outcomes,
        "fingerprint": registry_fingerprint,
    }

    def budget_id(outcome_id: str) -> str:
        encoded = f"{parent_id}\0{outcome_id}\n".encode()
        return "sha256:" + hashlib.sha256(encoded).hexdigest()

    def dispatch(
        index: int,
        *,
        outcome_id: str = "integrated-outcome",
        packet_id: str = "packet-a",
        stage: str = "implementation",
        cycle_kind: str | None = "discovery",
        cycle_id: str | None = None,
        event_reason: str = "initial_stage_review",
        predecessor: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        dispatch_id = f"20260728T00{index:02d}00Z--HCM-0-8--self-test-{index}"
        prior_stage = (
            predecessor["causal_control"]["review_stage"]
            if predecessor is not None
            else None
        )
        same_stage = prior_stage == stage
        review_cycle = None
        if cycle_kind is not None:
            review_cycle = {
                "kind": cycle_kind,
                "cycle_id": cycle_id or f"{stage}-{cycle_kind}-{index}",
                "trigger_run_ids": (
                    []
                    if cycle_kind == "discovery"
                    else [f"run-{index - 1}"]
                ),
                "finding_refs": (
                    []
                    if cycle_kind == "discovery"
                    else [f"finding-{index - 1}"]
                ),
            }
        return {
            "schema_version": "1.4",
            "dispatch_id": dispatch_id,
            "created_at_utc": f"2026-07-28T00:{index:02d}:00Z",
            "parent_orchestration_id": parent_id,
            "phase_id": "HCM-0",
            "slice_id": "HCM-0.8",
            "packet_id": packet_id,
            "role": "review" if review_cycle is not None else "documentation",
            "subject_fingerprint": fingerprint,
            "review_cycle": review_cycle,
            "causal_outcome_registry": copy.deepcopy(outcome_registry),
            "causal_control": {
                "causal_budget_id": budget_id(outcome_id),
                "integrated_outcome_id": outcome_id,
                "outcome_registry_fingerprint": registry_fingerprint,
                "review_stage": stage,
                "stage_transition": {
                    "kind": "continue" if same_stage else "enter",
                    "from_stage": prior_stage,
                },
                "event_reason": event_reason,
                "causal_predecessor_dispatch_id": (
                    predecessor["dispatch_id"]
                    if predecessor is not None
                    else None
                ),
            },
        }

    def run(
        value: dict[str, Any],
        *,
        verdict: str = "findings",
        final_status: str = "completed",
    ) -> dict[str, Any]:
        index = int(value["dispatch_id"].rsplit("-", 1)[1])
        return {
            "run_id": f"run-{index}",
            "dispatch_id": value["dispatch_id"],
            "role": value["role"],
            "subject_fingerprint": value["subject_fingerprint"],
            "result_subject_fingerprint": (
                value["subject_fingerprint"]
                if final_status == "completed"
                else None
            ),
            "final_status": final_status,
            "verdict": verdict,
            "finding_refs": (
                [f"finding-{index}"] if verdict == "findings" else []
            ),
        }

    def sequence(
        values: list[dict[str, Any]],
        verdicts: list[str] | None = None,
        statuses: list[str] | None = None,
    ) -> tuple[list[dict[str, Any]], dict[str, dict[str, Any]]]:
        verdicts = verdicts or ["findings"] * len(values)
        statuses = statuses or ["completed"] * len(values)
        runs = [
            run(value, verdict=verdict, final_status=status)
            for value, verdict, status in zip(
                values, verdicts, statuses, strict=True
            )
        ]
        return runs, {
            run_value["run_id"]: dispatch_value
            for run_value, dispatch_value in zip(runs, values, strict=True)
        }

    def expect_sequence(
        label: str,
        values: list[dict[str, Any]],
        *,
        accepted: bool,
        verdicts: list[str] | None = None,
        statuses: list[str] | None = None,
    ) -> bool:
        runs, dispatch_by_run_id = sequence(values, verdicts, statuses)
        try:
            validate_v1_4_causal_sequence(
                Path(f"{label}.json"),
                runs,
                dispatch_by_run_id,
            )
        except ValidationFailure:
            return not accepted
        return accepted

    discovery = dispatch(1, cycle_id="renamed-discovery-a")
    renamed_discovery = dispatch(
        2,
        cycle_id="renamed-discovery-b",
        predecessor=discovery,
    )
    cases = [
        (
            "renamed-discovery-same-parent-packet",
            [discovery, renamed_discovery],
            False,
            None,
        ),
        (
            "discovery-after-clean-new-cycle-name",
            [discovery, renamed_discovery],
            False,
            ["clean", "clean"],
        ),
    ]

    remediation_discovery = dispatch(
        3,
        cycle_id="remediation-unmasked-discovery",
    )
    remediation_closure = dispatch(
        4,
        cycle_kind="closure",
        cycle_id="remediation-unmasked-closure",
        event_reason="remediation_unmasked_test_failure",
        predecessor=remediation_discovery,
    )
    cases.append(
        (
            "remediation-unmasked-next-causal-cycle",
            [remediation_discovery, remediation_closure],
            True,
            ["findings", "clean"],
        )
    )
    invalid_unmasked_discovery = dispatch(
        4,
        cycle_id="remediation-unmasked-renamed-discovery",
        event_reason="remediation_unmasked_test_failure",
        predecessor=remediation_discovery,
    )
    cases.append(
        (
            "remediation-unmasked-cannot-rediscover",
            [remediation_discovery, invalid_unmasked_discovery],
            False,
            None,
        )
    )

    budget_discovery = dispatch(5, cycle_id="budget-discovery")
    budget_closure = dispatch(
        6,
        cycle_kind="closure",
        cycle_id="budget-closure",
        event_reason="reviewer_finding",
        predecessor=budget_discovery,
    )
    supplemental_one = dispatch(
        7,
        cycle_kind="supplemental_causal",
        cycle_id="budget-supplemental-1",
        event_reason="remediation_unmasked_test_failure",
        predecessor=budget_closure,
    )
    supplemental_two = dispatch(
        8,
        cycle_kind="supplemental_causal",
        cycle_id="budget-supplemental-2",
        event_reason="proof_gap",
        predecessor=supplemental_one,
    )
    supplemental_three = dispatch(
        9,
        cycle_kind="supplemental_causal",
        cycle_id="budget-supplemental-3",
        event_reason="manifest_scope_omission",
        predecessor=supplemental_two,
    )
    cases.extend(
        [
            (
                "bounded-p2-lineage-representable",
                [
                    budget_discovery,
                    budget_closure,
                    supplemental_one,
                    supplemental_two,
                ],
                True,
                None,
            ),
            (
                "third-supplemental-rejected",
                [
                    budget_discovery,
                    budget_closure,
                    supplemental_one,
                    supplemental_two,
                    supplemental_three,
                ],
                False,
                None,
            ),
        ]
    )

    packet_one = dispatch(
        10,
        outcome_id="packet-one-outcome",
        packet_id="packet-one",
        stage="planning",
    )
    packet_two = dispatch(
        11,
        outcome_id="packet-two-outcome",
        packet_id="packet-two",
        stage="planning",
    )
    cases.append(
        (
            "legitimate-separate-packets",
            [packet_one, packet_two],
            True,
            ["clean", "clean"],
        )
    )

    planning = dispatch(
        12,
        outcome_id="typed-stage-outcome",
        packet_id="planning-packet",
        stage="planning",
    )
    implementation = dispatch(
        13,
        outcome_id="typed-stage-outcome",
        packet_id="implementation-packet",
        stage="implementation",
        event_reason="planned_stage_transition",
        predecessor=planning,
    )
    cases.append(
        (
            "planning-followed-by-implementation",
            [planning, implementation],
            True,
            ["clean", "clean"],
        )
    )

    authority_boundary = dispatch(40, stage="proof", cycle_id="authority-boundary-proof")
    authority_admission = dispatch(
        41,
        stage="authority_admission",
        cycle_id="authority-admission-discovery",
        event_reason="planned_stage_transition",
        predecessor=authority_boundary,
    )
    authority_admission["causal_control"]["stage_transition"] = {
        "kind": "authority_extension",
        "from_stage": "proof",
    }
    authority_admission["authority_continuation"] = copy.deepcopy(
        continuation_schema_fixture["authority_continuation"]
    )
    authority_admission["authority_continuation"]["review_slot"] = "authority_admission"
    authority_implementation = dispatch(
        42,
        stage="implementation",
        cycle_id="authority-implementation-discovery",
        event_reason="planned_stage_transition",
        predecessor=authority_admission,
    )
    authority_implementation["authority_continuation"] = copy.deepcopy(
        authority_admission["authority_continuation"]
    )
    authority_implementation["authority_continuation"]["review_slot"] = "implementation"
    cases.extend(
        [
            (
                "authority-continuation-selector-before-edit",
                [authority_boundary, authority_admission, authority_implementation],
                True,
                ["clean", "clean", "clean"],
            ),
            (
                "authority-continuation-write-before-clean",
                [authority_boundary, authority_admission, authority_implementation],
                False,
                ["clean", "findings", "clean"],
            ),
        ]
    )
    authority_proof_skip = dispatch(
        43,
        stage="proof",
        cycle_id="authority-proof-skip",
        event_reason="planned_stage_transition",
        predecessor=authority_admission,
    )
    authority_proof_skip["authority_continuation"] = copy.deepcopy(
        authority_admission["authority_continuation"]
    )
    authority_proof_skip["authority_continuation"]["review_slot"] = "proof"
    cases.append(
        (
            "authority-continuation-slot-skip",
            [authority_boundary, authority_admission, authority_proof_skip],
            False,
            ["clean", "clean", "clean"],
        )
    )
    authority_same_cycle = dispatch(
        42,
        stage="authority_admission",
        cycle_id="authority-admission-discovery",
        predecessor=authority_admission,
    )
    authority_same_cycle["authority_continuation"] = copy.deepcopy(
        authority_admission["authority_continuation"]
    )
    cases.append(
        (
            "authority-continuation-same-cycle-second-dispatch",
            [authority_boundary, authority_admission, authority_same_cycle],
            False,
            ["clean", "findings", "clean"],
        )
    )

    final_review = dispatch(
        14,
        outcome_id="mechanical-closeout-outcome",
        stage="final_closeout",
        cycle_id="final-review",
    )
    mechanical = dispatch(
        15,
        outcome_id="mechanical-closeout-outcome",
        stage="final_closeout",
        cycle_kind=None,
        event_reason="mechanical_closeout",
        predecessor=final_review,
    )
    reset_discovery = dispatch(
        16,
        outcome_id="mechanical-closeout-outcome",
        stage="final_closeout",
        cycle_id="mechanical-reset-discovery",
        predecessor=mechanical,
    )
    cases.append(
        (
            "mechanical-closeout-cannot-reset",
            [final_review, mechanical, reset_discovery],
            False,
            ["clean", "not_applicable", "clean"],
        )
    )

    observed_p3b: list[dict[str, Any]] = []
    prior: dict[str, Any] | None = None
    for offset, evidence_dispatch in enumerate(p3b_evidence, start=17):
        cycle_kind = evidence_dispatch["review_cycle"]["kind"]
        current = dispatch(
            offset,
            outcome_id="P3B-cli-surface-proof-integration",
            packet_id=evidence_dispatch["packet_id"],
            stage="proof",
            cycle_kind=cycle_kind,
            cycle_id=evidence_dispatch["review_cycle"]["cycle_id"],
            event_reason=(
                "initial_stage_review"
                if cycle_kind == "discovery"
                else "reviewer_finding"
            ),
            predecessor=prior,
        )
        observed_p3b.append(current)
        prior = current
    p3b_cycle_kinds = [
        dispatch["review_cycle"]["kind"]
        for dispatch in observed_p3b
    ]
    if (
        p3b_cycle_kinds.count("discovery") != 9
        or p3b_cycle_kinds.count("closure") != 4
    ):
        print(
            "orchestration contract self-test failed: observed P3B evidence "
            f"shape is {p3b_cycle_kinds!r}",
            file=sys.stderr,
        )
        return 1
    cases.append(
        (
            "observed-p3b-pattern",
            observed_p3b,
            False,
            None,
        )
    )
    packet_as_outcome = copy.deepcopy(observed_p3b)
    for value in packet_as_outcome:
        outcome_id = value["packet_id"]
        value["causal_control"]["integrated_outcome_id"] = outcome_id
        value["causal_control"]["causal_budget_id"] = budget_id(outcome_id)
    cases.append(
        (
            "observed-p3b-packet-as-outcome-reset",
            packet_as_outcome,
            False,
            None,
        )
    )

    for label, values, accepted, verdicts in cases:
        if not expect_sequence(
            label,
            values,
            accepted=accepted,
            verdicts=verdicts,
        ):
            expectation = "validate" if accepted else "fail closed"
            print(
                f"orchestration contract self-test failed: {label} did not "
                f"{expectation}",
                file=sys.stderr,
            )
            return 1

    try:
        validate_v1_4_dispatch_prefix(
            Path("renamed-discovery-prefix.json"),
            [discovery, renamed_discovery],
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: renamed discovery "
            "unexpectedly passed executable dispatch-prefix validation",
            file=sys.stderr,
        )
        return 1

    population_dispatches = {
        value["dispatch_id"]: (
            DISPATCHES_DIR / f"{value['dispatch_id']}.json",
            value,
            hashlib.sha256(value["dispatch_id"].encode()).hexdigest(),
        )
        for value in (packet_one, packet_two)
    }
    population_runs, _ = sequence(
        [packet_one, packet_two],
        ["clean", "clean"],
    )

    def population_record(
        runs: list[dict[str, Any]],
    ) -> dict[str, Any]:
        population = sorted(
            population_dispatches.values(),
            key=lambda item: (
                item[1]["created_at_utc"],
                item[1]["dispatch_id"],
            ),
        )
        encoded = "".join(
            f"{path.relative_to(REPO_ROOT).as_posix()}\0{dispatch_sha256}\n"
            for path, _, dispatch_sha256 in population
        ).encode()
        return {
            "schema_version": "1.4",
            "created_at_utc": "2026-07-28T01:00:00Z",
            "orchestration_id": parent_id,
            "delegated_runs": runs,
            "dispatch_population": {
                "scope": "parent_orchestration",
                "through_created_at_utc": "2026-07-28T01:00:00Z",
                "dispatch_count": len(population),
                "algorithm": "sha256",
                "encoding": "dispatch-ref-null-sha256-newline-v1",
                "causal_budget_ids": sorted(
                    {
                        item[1]["causal_control"]["causal_budget_id"]
                        for item in population
                    }
                ),
                "aggregate_fingerprint": (
                    "sha256:" + hashlib.sha256(encoded).hexdigest()
                ),
            },
        }

    omitted_record = population_record(population_runs[:1])
    try:
        validate_v1_4_dispatch_population(
            omitted_record,
            Path("omitted-dispatch.json"),
            population_dispatches,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: omitted dispatch validated",
            file=sys.stderr,
        )
        return 1

    offset_dispatch = copy.deepcopy(packet_one)
    offset_dispatch["created_at_utc"] = "2026-07-28T03:00:00+01:00"
    offset_dispatches = {
        offset_dispatch["dispatch_id"]: (
            DISPATCHES_DIR / f"{offset_dispatch['dispatch_id']}.json",
            offset_dispatch,
            hashlib.sha256(offset_dispatch["dispatch_id"].encode()).hexdigest(),
        )
    }
    offset_record = population_record([run(offset_dispatch)])
    offset_record["created_at_utc"] = "2026-07-28T02:30:00Z"
    offset_record["dispatch_population"]["through_created_at_utc"] = (
        "2026-07-28T02:30:00Z"
    )
    try:
        validate_v1_4_dispatch_population(
            offset_record,
            Path("offset-population-cutoff.json"),
            offset_dispatches,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: offset timestamp escaped "
            "the dispatch population cutoff",
            file=sys.stderr,
        )
        return 1

    mixed_dispatch = copy.deepcopy(packet_two)
    mixed_dispatch["schema_version"] = "1.3"
    mixed_dispatches = {
        packet_one["dispatch_id"]: population_dispatches[
            packet_one["dispatch_id"]
        ],
        mixed_dispatch["dispatch_id"]: (
            DISPATCHES_DIR / f"{mixed_dispatch['dispatch_id']}.json",
            mixed_dispatch,
            hashlib.sha256(mixed_dispatch["dispatch_id"].encode()).hexdigest(),
        ),
    }
    try:
        validate_v1_4_dispatch_population(
            population_record(population_runs),
            Path("mixed-version-parent.json"),
            mixed_dispatches,
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: mixed-version parent "
            "dispatch population unexpectedly validated",
            file=sys.stderr,
        )
        return 1

    abandoned_run = run(
        packet_one,
        verdict="not_applicable",
        final_status="abandoned",
    )
    abandoned_dispatches = {
        packet_one["dispatch_id"]: population_dispatches[
            packet_one["dispatch_id"]
        ]
    }
    abandoned_record = population_record([abandoned_run])
    abandoned_path, abandoned_value, abandoned_sha256 = next(
        iter(abandoned_dispatches.values())
    )
    abandoned_encoded = (
        f"{abandoned_path.relative_to(REPO_ROOT).as_posix()}"
        f"\0{abandoned_sha256}\n"
    ).encode()
    abandoned_record["dispatch_population"].update(
        {
            "dispatch_count": 1,
            "causal_budget_ids": [
                abandoned_value["causal_control"]["causal_budget_id"]
            ],
            "aggregate_fingerprint": (
                "sha256:" + hashlib.sha256(abandoned_encoded).hexdigest()
            ),
        }
    )
    try:
        validate_v1_4_dispatch_population(
            abandoned_record,
            Path("abandoned-dispatch.json"),
            abandoned_dispatches,
        )
    except ValidationFailure as error:
        print(
            "orchestration contract self-test failed: explicitly recorded "
            f"abandoned dispatch rejected: {error}",
            file=sys.stderr,
        )
        return 1

    abandoned_handoff = load_json(TEMPLATE_PATH)
    abandoned_handoff["delegated_runs"][0].update(
        {
            "agent_id": None,
            "final_status": "abandoned",
            "verdict": "not_applicable",
            "result_subject_fingerprint": None,
        }
    )
    try:
        validate_instance(
            abandoned_handoff,
            load_json(RECORD_SCHEMA_PATHS["1.4"]),
            "abandoned v1.4 handoff",
        )
    except ValidationFailure as error:
        print(
            "orchestration contract self-test failed: abandoned run failed "
            f"v1.4 schema validation: {error}",
            file=sys.stderr,
        )
        return 1

    chain_values = [budget_discovery, budget_closure, supplemental_one]
    chain_dispatches = {
        value["dispatch_id"]: (
            DISPATCHES_DIR / f"{value['dispatch_id']}.json",
            value,
            hashlib.sha256(value["dispatch_id"].encode()).hexdigest(),
        )
        for value in chain_values
    }

    def chain_record(
        label: str,
        cutoff: str,
        *,
        predecessor: str | None = None,
        status: str = "escalation_required",
        orchestration_id: str = parent_id,
        dispatch_data: dict[str, tuple[Path, dict[str, Any], str]]
        | None = None,
    ) -> tuple[Path, dict[str, Any]]:
        dispatch_data = dispatch_data or chain_dispatches
        population = sorted(
            (
                value
                for value in dispatch_data.values()
                if value[1]["parent_orchestration_id"] == orchestration_id
                and value[1]["created_at_utc"] <= cutoff
            ),
            key=lambda item: (item[1]["created_at_utc"], item[1]["dispatch_id"]),
        )
        encoded = "".join(
            f"{path.relative_to(REPO_ROOT).as_posix()}\0{dispatch_sha256}\n"
            for path, _, dispatch_sha256 in population
        ).encode()
        record = {
            "schema_version": "1.4",
            "handoff_id": label,
            "created_at_utc": cutoff,
            "status": status,
            "program_id": "handbook-contract-membrane",
            "phase_id": "HCM-0",
            "slice_id": "HCM-0.8",
            "packet_id": "packet-a",
            "orchestration_id": orchestration_id,
            "source_handoff_ids": [predecessor] if predecessor else [],
            "supersedes": [predecessor] if predecessor else [],
            "delegated_runs": [run(value) for _, value, _ in population],
            "dispatch_population": {
                "scope": "parent_orchestration",
                "through_created_at_utc": cutoff,
                "dispatch_count": len(population),
                "algorithm": "sha256",
                "encoding": "dispatch-ref-null-sha256-newline-v1",
                "causal_budget_ids": sorted(
                    {
                        value["causal_control"]["causal_budget_id"]
                        for _, value, _ in population
                    }
                ),
                "aggregate_fingerprint": (
                    "sha256:" + hashlib.sha256(encoded).hexdigest()
                ),
            },
        }
        return Path(f"{label}.json"), record

    first_path, first_stop = chain_record(
        "20260728T000500Z--HCM-0-8--self-test-first-stop",
        budget_discovery["created_at_utc"],
    )
    second_path, second_stop = chain_record(
        "20260728T000600Z--HCM-0-8--self-test-second-stop",
        budget_closure["created_at_utc"],
        predecessor=first_stop["handoff_id"],
    )
    terminal_path, terminal_stop = chain_record(
        "20260728T000700Z--HCM-0-8--self-test-terminal",
        supplemental_one["created_at_utc"],
        predecessor=second_stop["handoff_id"],
        status="completed",
    )

    try:
        validate_v1_4_dispatch_population(
            first_stop,
            first_path,
            chain_dispatches,
        )
    except ValidationFailure as error:
        print(
            "orchestration contract self-test failed: directly succeeded "
            f"interim prefix rejected: {error}",
            file=sys.stderr,
        )
        return 1

    def expect_handoff_chain(
        label: str,
        record_values: list[tuple[Path, dict[str, Any]]],
        dispatch_data: dict[str, tuple[Path, dict[str, Any], str]],
        *,
        accepted: bool,
        error_ref: str | None = None,
    ) -> bool:
        try:
            validate_v1_4_handoff_chains(record_values, dispatch_data)
        except ValidationFailure as error:
            return not accepted and (
                error_ref is None or error_ref in str(error)
            )
        return accepted

    first_two_dispatches = {
        dispatch_id: value
        for dispatch_id, value in chain_dispatches.items()
        if value[1]["created_at_utc"] <= second_stop["created_at_utc"]
    }
    first_only_dispatches = {
        budget_discovery["dispatch_id"]: chain_dispatches[
            budget_discovery["dispatch_id"]
        ]
    }
    chain_cases = [
        (
            "linked-two-stop-prefix",
            [(first_path, first_stop), (second_path, second_stop)],
            first_two_dispatches,
            True,
            None,
        ),
        (
            "latest-unsuperseded-cutoff",
            [(first_path, first_stop), (second_path, second_stop)],
            chain_dispatches,
            False,
            second_stop["handoff_id"],
        ),
        (
            "linked-two-stop-terminal-successor",
            [
                (first_path, first_stop),
                (second_path, second_stop),
                (terminal_path, terminal_stop),
            ],
            chain_dispatches,
            True,
            None,
        ),
        (
            "orphan-later-dispatch",
            [(first_path, first_stop)],
            first_two_dispatches,
            False,
            first_stop["handoff_id"],
        ),
    ]

    completed_first = copy.deepcopy(first_stop)
    completed_first["status"] = "completed"
    chain_cases.extend(
        [
            (
                "dispatch-after-completion",
                [(first_path, completed_first)],
                first_two_dispatches,
                False,
                completed_first["handoff_id"],
            ),
            (
                "handoff-after-completion",
                [(first_path, completed_first), (second_path, second_stop)],
                first_two_dispatches,
                False,
                completed_first["handoff_id"],
            ),
        ]
    )

    source_only = copy.deepcopy(second_stop)
    source_only["supersedes"] = []
    supersedes_only = copy.deepcopy(second_stop)
    supersedes_only["source_handoff_ids"] = []
    non_immediate = copy.deepcopy(terminal_stop)
    non_immediate["source_handoff_ids"] = [first_stop["handoff_id"]]
    non_immediate["supersedes"] = [first_stop["handoff_id"]]
    cross_parent_path, cross_parent = chain_record(
        "20260728T000600Z--HCM-0-8--self-test-cross-parent",
        budget_closure["created_at_utc"],
        orchestration_id=parent_id + "-other",
    )
    cross_parent["source_handoff_ids"] = [first_stop["handoff_id"]]
    cross_parent["supersedes"] = [first_stop["handoff_id"]]
    compound_cross_parent = copy.deepcopy(cross_parent)
    compound_cross_parent["handoff_id"] += "-phase-drift"
    compound_cross_parent["phase_id"] = "HCM-9"
    compound_cross_parent_path = Path(
        f"{compound_cross_parent['handoff_id']}.json"
    )
    cross_packet_transition = copy.deepcopy(compound_cross_parent)
    cross_packet_transition["handoff_id"] += "-cross-packet"
    cross_packet_transition["packet_id"] = "other-packet"
    cross_packet_transition_path = Path(
        f"{cross_packet_transition['handoff_id']}.json"
    )
    chain_cases.extend(
        [
            (
                "source-only-link",
                [(first_path, first_stop), (second_path, source_only)],
                first_two_dispatches,
                False,
                source_only["handoff_id"],
            ),
            (
                "supersedes-only-link",
                [(first_path, first_stop), (second_path, supersedes_only)],
                first_two_dispatches,
                False,
                supersedes_only["handoff_id"],
            ),
            (
                "non-immediate-link",
                [
                    (first_path, first_stop),
                    (second_path, second_stop),
                    (terminal_path, non_immediate),
                ],
                chain_dispatches,
                False,
                non_immediate["handoff_id"],
            ),
            (
                "cross-parent-successor",
                [(first_path, first_stop), (cross_parent_path, cross_parent)],
                first_two_dispatches,
                False,
                cross_parent["handoff_id"],
            ),
            (
                "cross-parent-plus-phase-drift",
                [
                    (first_path, first_stop),
                    (compound_cross_parent_path, compound_cross_parent),
                ],
                first_two_dispatches,
                False,
                compound_cross_parent["handoff_id"],
            ),
            (
                "completed-plus-cross-parent-phase-drift",
                [
                    (first_path, completed_first),
                    (compound_cross_parent_path, compound_cross_parent),
                ],
                first_two_dispatches,
                False,
                compound_cross_parent["handoff_id"],
            ),
            (
                "historical-cross-packet-transition",
                [
                    (first_path, first_stop),
                    (cross_packet_transition_path, cross_packet_transition),
                ],
                first_only_dispatches,
                True,
                None,
            ),
        ]
    )

    identity_fields = {
        "program_id": "other-program",
        "phase_id": "HCM-9",
        "slice_id": "HCM-9.9",
        "packet_id": "other-packet",
    }
    for field, changed_value in identity_fields.items():
        drifted = copy.deepcopy(second_stop)
        drifted[field] = changed_value
        chain_cases.append(
            (
                f"{field}-drift",
                [(first_path, first_stop), (second_path, drifted)],
                first_two_dispatches,
                False,
                drifted["handoff_id"],
            )
        )

    for drift_kind in ("integrated_outcome_id", "causal_budget_id"):
        drift_dispatches = copy.deepcopy(first_two_dispatches)
        drift_value = drift_dispatches[budget_closure["dispatch_id"]][1]
        if drift_kind == "integrated_outcome_id":
            drift_value["causal_control"][drift_kind] = (
                "mechanical-closeout-outcome"
            )
        else:
            drift_value["causal_control"][drift_kind] = "sha256:" + "9" * 64
        drift_second_path, drift_second = chain_record(
            f"20260728T000600Z--HCM-0-8--self-test-{drift_kind}-drift",
            budget_closure["created_at_utc"],
            predecessor=first_stop["handoff_id"],
            dispatch_data=drift_dispatches,
        )
        chain_cases.append(
            (
                f"{drift_kind}-drift",
                [(first_path, first_stop), (drift_second_path, drift_second)],
                drift_dispatches,
                False,
                drift_second["handoff_id"],
            )
        )

    added_prefix = copy.deepcopy(first_stop)
    added_prefix["delegated_runs"].append(run(budget_closure))
    chain_cases.append(
        (
            "interim-prefix-addition",
            [(first_path, added_prefix)],
            first_two_dispatches,
            False,
            added_prefix["handoff_id"],
        )
    )

    for label, record_values, dispatch_data, accepted, error_ref in chain_cases:
        if not expect_handoff_chain(
            label,
            record_values,
            dispatch_data,
            accepted=accepted,
            error_ref=error_ref,
        ):
            expectation = "validate" if accepted else "fail closed"
            print(
                f"orchestration contract self-test failed: {label} did not "
                f"{expectation}",
                file=sys.stderr,
            )
            return 1

    runtime_allowance = load_dispatch_template_fixture()
    runtime_allowance["subject_manifest"]["entries"][0]["path"] = (
        "src/fixtures/runtime-authority.json"
    )
    runtime_allowance["ancillary_allowance"] = {
        "kind": "bounded_test_support",
        "risk_ceiling": "test_proof_only",
        "baseline_ref": "HEAD",
        "max_paths": 1,
        "max_changed_lines": 10,
        "entries": [
            {
                "path": "src/fixtures/runtime-authority.json",
                "path_kind": "test_fixture",
                "changed_lines": 10,
            }
        ],
    }
    try:
        validate_v1_4_dispatch_control(
            runtime_allowance,
            Path("runtime-ancillary-allowance.json"),
        )
    except ValidationFailure:
        pass
    else:
        print(
            "orchestration contract self-test failed: ancillary allowance "
            "unexpectedly authorized a runtime fixture path",
            file=sys.stderr,
        )
        return 1

    with tempfile.TemporaryDirectory(prefix="hcm-ancillary-diff-") as temp_dir:
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
        ancillary_path = "tests/fixtures/observed.txt"
        ancillary_file = temp_repo / ancillary_path
        ancillary_file.parent.mkdir(parents=True)
        ancillary_file.write_text("baseline\n", encoding="utf-8")
        later_ancillary_path = "tests/fixtures/later-observed.txt"
        later_ancillary_file = temp_repo / later_ancillary_path
        later_ancillary_file.write_text("baseline\n", encoding="utf-8")
        subprocess.run(
            ["git", "add", ancillary_path, later_ancillary_path],
            cwd=temp_repo,
            check=True,
        )
        subprocess.run(
            ["git", "commit", "-qm", "baseline"],
            cwd=temp_repo,
            check=True,
        )
        baseline_ref = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=temp_repo,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        ancillary_file.write_text("baseline\nobserved\n", encoding="utf-8")
        later_ancillary_file.write_text(
            "baseline\nobserved\n",
            encoding="utf-8",
        )
        observed_allowance = load_dispatch_template_fixture()
        observed_allowance["subject_manifest"]["entries"][0]["path"] = (
            ancillary_path
        )
        observed_allowance["ancillary_allowance"] = {
            "kind": "bounded_test_support",
            "risk_ceiling": "test_proof_only",
            "baseline_ref": baseline_ref,
            "max_paths": 1,
            "max_changed_lines": 1,
            "entries": [
                {
                    "path": ancillary_path,
                    "path_kind": "test_fixture",
                    "changed_lines": 1,
                }
            ],
        }
        validate_v1_4_dispatch_control(
            observed_allowance,
            Path("observed-ancillary-allowance.json"),
            verify_ancillary_diff=True,
            repo_root=temp_repo,
        )
        understated_allowance = copy.deepcopy(observed_allowance)
        understated_allowance["ancillary_allowance"]["entries"][0][
            "changed_lines"
        ] = 2
        try:
            validate_v1_4_dispatch_control(
                understated_allowance,
                Path("understated-ancillary-allowance.json"),
                verify_ancillary_diff=True,
                repo_root=temp_repo,
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: false ancillary "
                "changed-line count unexpectedly validated",
                file=sys.stderr,
            )
            return 1
        subprocess.run(
            ["git", "add", ancillary_path, later_ancillary_path],
            cwd=temp_repo,
            check=True,
        )
        subprocess.run(
            ["git", "commit", "-qm", "observed"],
            cwd=temp_repo,
            check=True,
        )
        target_ref = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=temp_repo,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        ancillary_record = {
            "orchestration_id": observed_allowance[
                "parent_orchestration_id"
            ],
            "created_at_utc": "2026-07-14T00:00:02Z",
            "status": "completed",
            "delegated_runs": [
                {
                    "dispatch_id": observed_allowance["dispatch_id"],
                    "role": "review",
                    "final_status": "completed",
                    "verdict": "clean",
                }
            ],
            "repo_state": {"head": target_ref},
            "ancillary_diff_observations": [
                {
                    "baseline_ref": baseline_ref,
                    "path": ancillary_path,
                    "path_kind": "test_fixture",
                    "changed_lines": 1,
                }
            ],
        }
        ancillary_dispatches = {
            observed_allowance["dispatch_id"]: (
                Path("observed-ancillary-allowance.json"),
                observed_allowance,
                "0" * 64,
            )
        }
        validate_v1_4_ancillary_observations(
            ancillary_record,
            Path("observed-ancillary-closeout.json"),
            ancillary_dispatches,
            repo_root=temp_repo,
        )
        later_allowance = copy.deepcopy(observed_allowance)
        later_allowance.update(
            {
                "dispatch_id": (
                    "20260714T000001Z--HCM-X-Y--prefix-later-allowance"
                ),
                "created_at_utc": "2026-07-14T00:00:01Z",
            }
        )
        later_allowance["subject_manifest"]["entries"][0]["path"] = (
            later_ancillary_path
        )
        later_allowance["ancillary_allowance"]["entries"][0]["path"] = (
            later_ancillary_path
        )
        later_allowance["ancillary_allowance"]["max_paths"] = 2
        later_allowance["ancillary_allowance"]["max_changed_lines"] = 2
        prefix_dispatches = {
            **ancillary_dispatches,
            later_allowance["dispatch_id"]: (
                Path("prefix-later-allowance.json"),
                later_allowance,
                "2" * 64,
            ),
        }
        interim_record = copy.deepcopy(ancillary_record)
        interim_record["status"] = "partial"
        interim_record["created_at_utc"] = observed_allowance["created_at_utc"]
        try:
            validate_v1_4_ancillary_observations(
                interim_record,
                Path("interim-ancillary-prefix.json"),
                prefix_dispatches,
                repo_root=temp_repo,
            )
        except ValidationFailure as error:
            print(
                "orchestration contract self-test failed: later ancillary "
                f"allowance retroactively invalidated an interim prefix: {error}",
                file=sys.stderr,
            )
            return 1
        successor_record = copy.deepcopy(interim_record)
        successor_record["created_at_utc"] = later_allowance["created_at_utc"]
        successor_record["ancillary_diff_observations"].append(
            {
                "baseline_ref": baseline_ref,
                "path": later_ancillary_path,
                "path_kind": "test_fixture",
                "changed_lines": 1,
            }
        )
        successor_record["ancillary_diff_observations"].sort(
            key=lambda observation: (
                observation["baseline_ref"],
                observation["path"],
            )
        )
        validate_v1_4_ancillary_observations(
            successor_record,
            Path("successor-ancillary-prefix.json"),
            prefix_dispatches,
            repo_root=temp_repo,
        )
        omitted_prefix_observation = copy.deepcopy(successor_record)
        omitted_prefix_observation["ancillary_diff_observations"].pop()
        extra_prefix_observation = copy.deepcopy(interim_record)
        extra_prefix_observation["ancillary_diff_observations"].append(
            next(
                observation
                for observation in successor_record[
                    "ancillary_diff_observations"
                ]
                if observation["path"] == later_ancillary_path
            )
        )
        extra_prefix_observation["ancillary_diff_observations"].sort(
            key=lambda observation: (
                observation["baseline_ref"],
                observation["path"],
            )
        )
        for label, candidate in (
            ("omitted", omitted_prefix_observation),
            ("extra", extra_prefix_observation),
        ):
            try:
                validate_v1_4_ancillary_observations(
                    candidate,
                    Path(f"{label}-ancillary-prefix.json"),
                    prefix_dispatches,
                    repo_root=temp_repo,
                )
            except ValidationFailure:
                pass
            else:
                print(
                    "orchestration contract self-test failed: ancillary "
                    f"prefix {label} unexpectedly validated",
                    file=sys.stderr,
                )
                return 1
        false_observation = copy.deepcopy(ancillary_record)
        false_observation["ancillary_diff_observations"][0][
            "changed_lines"
        ] = 2
        try:
            validate_v1_4_ancillary_observations(
                false_observation,
                Path("false-ancillary-closeout.json"),
                ancillary_dispatches,
                repo_root=temp_repo,
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: false ancillary "
                "closeout observation unexpectedly validated",
                file=sys.stderr,
            )
            return 1
        ancillary_file.write_text(
            "baseline\nobserved\npost-review-growth\n",
            encoding="utf-8",
        )
        subprocess.run(["git", "add", ancillary_path], cwd=temp_repo, check=True)
        subprocess.run(
            ["git", "commit", "-qm", "post-review growth"],
            cwd=temp_repo,
            check=True,
        )
        expanded_target_ref = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=temp_repo,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        expanded_observation = copy.deepcopy(ancillary_record)
        expanded_observation["repo_state"]["head"] = expanded_target_ref
        expanded_observation["ancillary_diff_observations"][0][
            "changed_lines"
        ] = 2
        try:
            validate_v1_4_ancillary_observations(
                expanded_observation,
                Path("expanded-ancillary-closeout.json"),
                ancillary_dispatches,
                repo_root=temp_repo,
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: post-review "
                "ancillary growth exceeded its ceiling but validated",
                file=sys.stderr,
            )
            return 1
        late_widening_dispatch = copy.deepcopy(observed_allowance)
        late_widening_dispatch.update(
            {
                "dispatch_id": "20260714T000001Z--HCM-X-Y--late-widening",
                "created_at_utc": "2026-07-14T00:00:01Z",
                "orchestration_decision": "mechanical_closeout",
                "role": "proof",
            }
        )
        late_widening_dispatch["ancillary_allowance"][
            "max_changed_lines"
        ] = 2
        late_widening_dispatch["ancillary_allowance"]["entries"][0][
            "changed_lines"
        ] = 2
        widened_dispatches = dict(ancillary_dispatches)
        widened_dispatches[late_widening_dispatch["dispatch_id"]] = (
            Path("late-widening.json"),
            late_widening_dispatch,
            "1" * 64,
        )
        try:
            validate_v1_4_ancillary_observations(
                expanded_observation,
                Path("late-widening-closeout.json"),
                widened_dispatches,
                repo_root=temp_repo,
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: a post-review "
                "mechanical dispatch widened the frozen ancillary ceiling",
                file=sys.stderr,
            )
            return 1
        same_second_widening_dispatch = copy.deepcopy(late_widening_dispatch)
        same_second_widening_dispatch.update(
            {
                "dispatch_id": "ZZZZZZZZZZZZZZZ--HCM-X-Y--late-widening",
                "created_at_utc": observed_allowance["created_at_utc"],
            }
        )
        same_second_widened_dispatches = dict(ancillary_dispatches)
        same_second_widened_dispatches[
            same_second_widening_dispatch["dispatch_id"]
        ] = (
            Path("same-second-late-widening.json"),
            same_second_widening_dispatch,
            "3" * 64,
        )
        try:
            validate_v1_4_ancillary_observations(
                expanded_observation,
                Path("same-second-late-widening-closeout.json"),
                same_second_widened_dispatches,
                repo_root=temp_repo,
            )
        except ValidationFailure:
            pass
        else:
            print(
                "orchestration contract self-test failed: a same-second "
                "post-review dispatch widened the frozen ancillary ceiling",
                file=sys.stderr,
            )
            return 1

    print(
        "v1.4 causal contract self-test passed: renamed and post-CLEAN "
        "discovery rejected; remediation-unmasked failure consumes the next "
        "causal cycle; third supplemental, undeclared outcome reset, "
        "mixed-version parents, offset cutoffs, omitted dispatches, and "
        "executable-prefix subdivision rejected; cumulative ancillary prefixes "
        "accepted while retroactive authority, prefix omissions/extras, false "
        "counts, post-review ceiling widening, and same-second widening fail "
        "closed; v1.3 corpus "
        "mutations rejected; "
        "direct same-parent stop succession and terminal completion accepted; "
        "orphan dispatches, broken or cross-parent successors, identity drift, "
        "and resumption after completion rejected; abandoned dispatches, registered "
        "separate packets, typed stage advance, and bounded P2 lineage "
        "accepted; mechanical reset and exact observed P3B subdivision "
        "rejected"
    )
    return 0


def run_orchestration_contract_self_test() -> int:
    if run_v1_4_causal_contract_self_test() != 0:
        return 1
    current_handoff_schema = load_json(RECORD_SCHEMA_PATHS["1.4"])
    current_dispatch_schema = load_json(
        INTERNAL_DISPATCH_SCHEMA_PATHS["1.4"]
    )
    current_template = load_json(TEMPLATE_PATH)
    current_dispatch_template = load_dispatch_template_fixture()
    validate_instance(
        current_template,
        current_handoff_schema,
        "v1.4 handoff template",
    )
    validate_instance(
        current_dispatch_template,
        current_dispatch_schema,
        "v1.4 internal dispatch template",
    )
    validate_v1_4_dispatch_control(
        current_dispatch_template,
        INTERNAL_DISPATCH_TEMPLATE_PATH,
    )

    handoff_schema = load_json(RECORD_SCHEMA_PATHS["1.3"])
    dispatch_schema = load_json(INTERNAL_DISPATCH_SCHEMA_PATHS["1.3"])
    template = copy.deepcopy(current_template)
    template["schema_version"] = "1.3"
    del template["dispatch_population"]
    del template["ancillary_diff_observations"]
    template.pop("authority_continuations", None)
    for run in template["delegated_runs"]:
        run.pop("carried_finding_refs", None)
    dispatch_template = copy.deepcopy(current_dispatch_template)
    dispatch_template["schema_version"] = "1.3"
    del dispatch_template["causal_outcome_registry"]
    del dispatch_template["causal_control"]
    del dispatch_template["pre_review_convergence"]
    del dispatch_template["ancillary_allowance"]
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

    retained_dispatch_data = copy.deepcopy(dispatch_data)
    retained_dispatch_data["self-test-clean-review"][1]["review_cycle"][
        "finding_refs"
    ] = ["findings-review-finding"]
    retained_record = copy.deepcopy(valid_parent_record)
    retained_record["delegated_runs"][1]["finding_refs"] = []
    retained_record["delegated_runs"][1]["carried_finding_refs"] = [
        "findings-review-finding"
    ]
    retained_record["findings"] = [
        finding
        for finding in retained_record["findings"]
        if finding["finding_id"] != "mid-findings-review-finding"
    ]
    validate_v1_2_semantics(
        retained_record,
        RECORDS_DIR / "self-test-retained-finding-lineage.json",
        set(),
        retained_dispatch_data,
        review_inventory_entries=self_test_inventory_entries,
    )

    retained_negative_cases: list[
        tuple[str, dict[str, Any], dict[str, tuple[Path, dict[str, Any], str]]]
    ] = []
    case_record = copy.deepcopy(retained_record)
    case_record["delegated_runs"][1]["finding_refs"] = [
        "findings-review-finding"
    ]
    retained_negative_cases.append(
        ("duplicate-owner-carry", case_record, retained_dispatch_data)
    )
    case_record = copy.deepcopy(retained_record)
    case_record["delegated_runs"][1]["carried_finding_refs"] = [
        "fabricated-finding"
    ]
    retained_negative_cases.append(
        ("fabricated-carry", case_record, retained_dispatch_data)
    )
    case_record = copy.deepcopy(retained_record)
    case_record["findings"][0]["priority"] = "P3"
    case_record["findings"][0]["severity"] = "warning"
    retained_negative_cases.append(
        ("nonblocking-carry", case_record, retained_dispatch_data)
    )
    case_dispatch_data = copy.deepcopy(retained_dispatch_data)
    case_dispatch_data["self-test-mid-findings-review"][1]["review_cycle"][
        "trigger_run_ids"
    ] = ["clean-review"]
    retained_negative_cases.append(
        ("non-predecessor-carry", retained_record, case_dispatch_data)
    )
    case_record = copy.deepcopy(retained_record)
    case_record["remediations"] = case_record["remediations"][1:]
    retained_negative_cases.append(
        ("carry-without-remediation", case_record, retained_dispatch_data)
    )
    case_dispatch_data = copy.deepcopy(retained_dispatch_data)
    case_dispatch_data["self-test-clean-review"][1]["review_cycle"][
        "finding_refs"
    ] = []
    retained_negative_cases.append(
        ("trigger-array-laundering", retained_record, case_dispatch_data)
    )
    case_record = copy.deepcopy(retained_record)
    case_record["delegated_runs"][1]["final_status"] = "blocked"
    case_record["remediations"] = case_record["remediations"][1:]
    retained_negative_cases.append(
        ("blocked-carrier", case_record, retained_dispatch_data)
    )
    case_record = copy.deepcopy(retained_record)
    case_record["delegated_runs"][1]["verdict"] = "not_applicable"
    case_record["remediations"] = case_record["remediations"][:1]
    retained_negative_cases.append(
        ("not-applicable-carrier", case_record, retained_dispatch_data)
    )
    retained_exact_failures = {
        "blocked-carrier": "can carry findings only when completed with verdict findings",
        "not-applicable-carrier": (
            "can carry findings only when completed with verdict findings"
        ),
    }
    for label, candidate_record, candidate_dispatches in retained_negative_cases:
        try:
            validate_v1_2_semantics(
                candidate_record,
                RECORDS_DIR / f"self-test-{label}.json",
                set(),
                candidate_dispatches,
                review_inventory_entries=self_test_inventory_entries,
            )
        except ValidationFailure as exc:
            expected_failure = retained_exact_failures.get(label)
            if expected_failure is not None and expected_failure not in str(exc):
                print(
                    "orchestration contract self-test failed: retained-finding "
                    f"negative {label!r} raised the wrong failure: {exc}",
                    file=sys.stderr,
                )
                return 1
        else:
            print(
                "orchestration contract self-test failed: retained-finding "
                f"negative {label!r} unexpectedly validated",
                file=sys.stderr,
            )
            return 1

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
        "chained and retained findings/remediation/re-review, exactly two supplemental "
        "causal cycles, clean review with a complete durable P3/P4 inventory "
        "row, direct parent remediation, "
        "reviewed-baseline/primary-commit identity, and two-commit ledger "
        "mutation validate; duplicate ownership, fabricated/unowned carry, "
        "wrong-priority carry, non-predecessor carry, missing remediation, "
        "trigger-array laundering, and blocked/not-applicable carriers fail closed"
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
        if version != "1.4":
            raise ValidationFailure(
                f"{dispatch_path}: internal-dispatch {version} is immutable "
                "predecessor evidence and cannot be executed"
            )
        schema = load_json(schema_path)
        validate_instance(dispatch, schema, str(dispatch_path))
        validate_v1_4_dispatch_control(
            dispatch,
            dispatch_path,
            verify_ancillary_diff=True,
            repo_root=repo_root,
        )
        parent_dispatches: list[dict[str, Any]] = []
        dispatch_time = parse_utc_timestamp(
            dispatch["created_at_utc"],
            dispatch_path,
        )
        for candidate_path in sorted(dispatches_dir.glob("*.json")):
            candidate = load_json(candidate_path)
            if (
                candidate.get("parent_orchestration_id")
                != dispatch["parent_orchestration_id"]
            ):
                continue
            candidate_time = parse_utc_timestamp(
                candidate["created_at_utc"],
                candidate_path,
            )
            if candidate_time > dispatch_time:
                continue
            if candidate.get("schema_version") != "1.4":
                raise ValidationFailure(
                    f"{dispatch_path}: parent orchestration mixes immutable "
                    "predecessor dispatch versions"
                )
            parent_dispatches.append(candidate)
        validate_v1_4_dispatch_prefix(dispatch_path, parent_dispatches)
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
        if template_version != "1.4":
            raise ValidationFailure(
                f"{TEMPLATE_PATH}: new-record template must route to "
                "schema_version 1.4"
            )
        validate_instance(template, record_schemas[template_version], str(TEMPLATE_PATH))

        internal_dispatch_template = load_json(INTERNAL_DISPATCH_TEMPLATE_PATH)
        internal_dispatch_template_version = internal_dispatch_template.get(
            "schema_version"
        )
        if internal_dispatch_template_version != "1.4":
            raise ValidationFailure(
                f"{INTERNAL_DISPATCH_TEMPLATE_PATH}: current internal dispatch "
                "template must route to schema_version 1.4"
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
                if version == "1.4":
                    validate_v1_4_dispatch_control(dispatch, path)
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
        v1_4_parents = sorted(
            {
                dispatch["parent_orchestration_id"]
                for _, dispatch, _ in dispatches.values()
                if dispatch["schema_version"] == "1.4"
            }
        )
        for parent_orchestration_id in v1_4_parents:
            parent_dispatches = [
                dispatch
                for _, dispatch, _ in dispatches.values()
                if dispatch["parent_orchestration_id"]
                == parent_orchestration_id
            ]
            if any(
                dispatch["schema_version"] != "1.4"
                for dispatch in parent_dispatches
            ):
                raise ValidationFailure(
                    f"{parent_orchestration_id}: v1.4 parent orchestration "
                    "mixes immutable predecessor dispatch versions"
                )
            validate_v1_4_dispatch_prefix(
                Path(parent_orchestration_id),
                parent_dispatches,
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
            validate_immutable_v1_3_corpora(dispatches, loaded_records)
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

        validate_v1_4_handoff_chains(records, dispatches)

        for path, record in records:
            if record["schema_version"] in {"1.2", "1.3", "1.4"}:
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
