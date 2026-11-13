"""Run a fixed six-pair native S06 registration benchmark."""
import csv
import hashlib
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time

ARMS = ("s06", "current")
PACKET = Path(__file__).resolve().parent
ROOT = PACKET.parent
EXAMPLE = Path("crates/sdk/examples/s06_work_compare.rs")
CARGO = ["cargo", "--color", "never"]
TARGET = ROOT / "target"
RUNTIME = ROOT / "runtime"
LOGS = ROOT / "logs"
EXE = "s06_work_compare.exe" if os.name == "nt" else "s06_work_compare"


def sha(path):
    with path.open("rb") as data:
        return hashlib.file_digest(data, "sha256").hexdigest()


def dump(path, value):
    with path.open("x", encoding="utf-8") as output:
        json.dump(value, output, indent=2)
        output.write("\n")


def verify_inputs():
    for row in (PACKET / "SHA256SUMS").read_text().splitlines():
        digest, name = row.split("  ", 1)
        if sha(PACKET / name) != digest:
            raise ValueError(f"packet checksum mismatch for {name}")


def prepare():
    verify_inputs()
    for arm in ARMS:
        source = ROOT / arm
        source.mkdir()
        subprocess.run(["tar", "-xf", str(PACKET / "s06-source.tar"), "-C", str(source)], check=True)
        if arm == "current":
            patch = str(PACKET / "retained-baseline.patch")
            subprocess.run(["git", "apply", "--check", patch], cwd=source, check=True)
            subprocess.run(["git", "apply", patch], cwd=source, check=True)
        (source / EXAMPLE).parent.mkdir(exist_ok=True, parents=True)
        shutil.copyfile(PACKET / EXAMPLE.name, source / EXAMPLE)


def verify_source(arm):
    inventory = json.loads((PACKET / "source-inventory.json").read_text())[arm]
    source = ROOT / arm
    actual = {p.relative_to(source).as_posix(): sha(p) for p in source.rglob("*") if p.is_file()}
    if actual != inventory:
        differing = sorted(k for k in actual.keys() | inventory.keys() if actual.get(k) != inventory.get(k))
        raise ValueError(f"{arm} frozen source mismatch: {differing[:20]}")


def cargo(name, arm, args):
    print(name, flush=True)
    command = CARGO + args
    with (LOGS / f"{name}.log").open("xb") as output:
        output.write((repr(command) + "\n").encode())
        result = subprocess.run(command, cwd=ROOT / arm, stdout=output, stderr=subprocess.STDOUT)
    if result.returncode:
        raise RuntimeError(f"{name} exited {result.returncode}. See {LOGS / (name + '.log')}")


def build():
    verify_inputs()
    for arm in ARMS:
        verify_source(arm)
    LOGS.mkdir(exist_ok=True)
    RUNTIME.mkdir(exist_ok=True)
    temp = ROOT / "tmp"
    temp.mkdir(exist_ok=True)
    os.environ.update(CARGO_TARGET_DIR=str(TARGET), TMPDIR=str(temp), TEMP=str(temp), TMP=str(temp))
    os.environ.pop("CARGO_BUILD_TARGET", None)
    locked = ["--release", "--locked", "--offline"]
    serial = ["--", "--test-threads=1"]
    suites = [
        ("file-native", ["-p", "file-native", "--test", "public_api"]),
        ("engine", ["-p", "handbook-engine", "--test", "local_charter", "--test", "hcm_2_4_charter_profile_compatibility", "--test", "repository_durability"]),
        ("sdk-native", ["-p", "handbook-sdk", "--test", "work_state_file_native", "--test", "admission_governance_file_native"]),
    ]
    for name, args in suites:
        cargo(name, "current", ["test"] + locked + args + serial)
    if os.name == "nt":
        cargo("windows-listing", "current", ["test"] + locked + ["-p", "handbook-engine", "--lib", "windows_listing"] + serial)
    binaries = {}
    for arm in ARMS:
        cargo(f"build-{arm}", arm, ["build"] + locked + ["-p", "handbook-sdk", "--example", EXAMPLE.stem])
        binary = RUNTIME / f"{arm}-{EXE}"
        if binary.exists():
            raise FileExistsError(binary)
        shutil.copy2(TARGET / "release/examples" / EXE, binary)
        binaries[arm] = sha(binary)
        verify_source(arm)
    dump(LOGS / "runners.json", binaries)


def native_path(path):
    resolved = str(path.resolve())
    if os.name == "nt" and not resolved.startswith("\\\\?\\"):
        return "\\\\?\\" + resolved
    return resolved


def observed_builds():
    if os.name == "nt":
        out = subprocess.check_output(["tasklist.exe", "/FO", "CSV", "/NH"], text=True, errors="replace", timeout=5)
        rows = csv.reader(out.splitlines())
        return [row[0] for row in rows if row and row[0].lower() in ("cargo.exe", "rustc.exe")]
    out = subprocess.check_output(["ps", "-axo", "comm="], text=True, timeout=5)
    return [line for line in out.splitlines() if Path(line.strip()).name in ("cargo", "rustc")]


def run_arm(slot, arm):
    proof, parent = slot / "proof", slot / "work"
    proof.mkdir(parents=True)
    parent.mkdir()
    binary = RUNTIME / f"{arm}-{EXE}"
    samples = []
    with (slot / "stdout.log").open("xb") as stdout, (slot / "stderr.log").open("xb") as stderr, (slot / "activity.jsonl").open("x") as activity:
        start = time.monotonic()
        process = subprocess.Popen([str(binary), native_path(parent) + os.sep + "project", native_path(proof)], stdout=stdout, stderr=stderr)
        while True:
            sample = {"elapsed_s": time.monotonic() - start}
            try:
                sample["builds"] = observed_builds()
            except (OSError, subprocess.SubprocessError) as e:
                sample["error"] = str(e)
            samples.append(sample)
            activity.write(json.dumps(sample) + "\n")
            activity.flush()
            try:
                status = process.wait(timeout=max(0.01, 1.0 - (time.monotonic() - start - sample["elapsed_s"])))
                break
            except subprocess.TimeoutExpired:
                pass
        wall = time.monotonic() - start
    if status:
        raise RuntimeError(f"{slot} exited {status}. See {slot / 'stderr.log'}")
    trace = [json.loads(line) for line in (proof / "trace.jsonl").read_text().splitlines()]
    if [r["stage"] for r in trace if r["stage"] != "maintenance"] != ["allocated", "grant_intent", "grant_returned", "prepared", "start_intent", "returned", "reopened"]:
        raise ValueError(f"{slot}: missing or repeated lifecycle stage")
    returned = next(r for r in trace if r["stage"] == "returned")
    reopened = trace[-1]
    if reopened["readback"] != 1 or returned["execution"]["outcome"] != reopened["original"]["outcome"]:
        raise ValueError(f"{slot}: exact replay mismatch")
    gaps = [b["elapsed_s"] - a["elapsed_s"] for a, b in zip(samples, samples[1:])]
    result = {
        "sdk_ms": int(returned["sdk_elapsed_ns"]) / 1_000_000,
        "wall_ms": wall * 1000,
        "activity_samples": len(samples),
        "activity_positive": sum(bool(s.get("builds")) for s in samples),
        "activity_errors": sum("error" in s for s in samples),
        "activity_gaps_gt_2_5": sum(g > 2.5 for g in gaps),
        "trace_sha256": sha(proof / "trace.jsonl"),
    }
    dump(slot / "result.json", result)
    return result


def timing(root):
    verify_inputs()
    binaries = json.loads((LOGS / "runners.json").read_text())
    for arm in ARMS:
        verify_source(arm)
        if sha(RUNTIME / f"{arm}-{EXE}") != binaries[arm]:
            raise ValueError(f"{arm}: runner checksum mismatch")
    root.mkdir()
    pairs = []
    for n in range(1, 7):
        order = ARMS if n % 2 else ARMS[::-1]
        pair = {"pair": n, "order": order}
        for arm in order:
            pair[arm] = run_arm(root / f"p{n:02}" / arm, arm)
            print(f"p{n:02} {arm}: {pair[arm]['sdk_ms']:.3f} ms", flush=True)
        pair["delta_ms"] = pair["current"]["sdk_ms"] - pair["s06"]["sdk_ms"]
        pair["delta_pct"] = 100 * pair["delta_ms"] / pair["s06"]["sdk_ms"]
        pairs.append(pair)
    dump(root / "summary.json", {
        "pairs": pairs,
        "sdk_ms": {arm: {
            "median": statistics.median(p[arm]["sdk_ms"] for p in pairs),
            "min": min(p[arm]["sdk_ms"] for p in pairs),
            "max": max(p[arm]["sdk_ms"] for p in pairs),
        } for arm in ARMS},
        "paired_median_ms": statistics.median(p["delta_ms"] for p in pairs),
        "paired_median_pct": statistics.median(p["delta_pct"] for p in pairs),
    })


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "prepare":
        prepare()
    elif mode == "build":
        build()
    elif mode == "run" and len(sys.argv) == 3:
        timing(Path(sys.argv[2]).resolve())
    else:
        raise SystemExit("usage: bench.py prepare | build | run ABSENT_TIMING_ROOT")
