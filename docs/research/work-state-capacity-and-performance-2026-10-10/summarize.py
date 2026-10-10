"""Summarize every observed sample, without percentile claims from tiny samples."""
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path

groups = defaultdict(list)
metadata = []
for line in Path(sys.argv[1]).read_text(encoding="utf-8-sig").splitlines():
    row = json.loads(line)
    if "elapsed_ms" not in row:
        metadata.append(row)
        continue
    key = (row["operation"], row["api"], row.get("history_slots_at_entry", row.get("synthetic_record_count")), row["outcome"])
    groups[key].append(row)

summary = []
for (operation, api, history, outcome), rows in sorted(groups.items()):
    values = [row["elapsed_ms"] for row in rows]
    counts_key = "synthetic_record_count" if api == "synthetic_repeat" else "history_slots_at_entry"
    bytes_key = "processed_input_bytes" if api == "synthetic_repeat" else "retained_work_bytes_at_entry"
    summary.append({
        "operation": operation, "api": api,
        counts_key: history, "outcome": outcome,
        "sample_count": len(values), "min_ms": min(values),
        "median_ms": statistics.median(values), "max_ms": max(values),
        "byte_basis": "processed synthetic input" if api == "synthetic_repeat" else "retained work tree files",
        "bytes_min": min(row[bytes_key] for row in rows),
        "bytes_max": max(row[bytes_key] for row in rows),
    })

result = {"groups": summary, "metadata": metadata,
          "sample_count": sum(len(rows) for rows in groups.values()),
          "interpretation": "Single observations remain individual observations. Read/component medians and ranges describe repeated same-store probes. Initial-activation and full32-revocation groups pool distinct fixtures: their medians/ranges are descriptive arithmetic, not repeated fixed-cohort estimates. No p95 confidence is claimed."}
Path(sys.argv[2]).write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
print(f"Verified {result['sample_count']} timed samples in {len(summary)} groups; {len(metadata)} metadata rows.")
