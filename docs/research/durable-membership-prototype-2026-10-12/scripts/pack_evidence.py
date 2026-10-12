"""Retain exact raw evidence compactly; gzip streams are deterministic and lossless."""
import gzip
from pathlib import Path
import shutil
import sys

boundary = Path(__file__).resolve().parents[1] / "evidence"
for argument in sys.argv[1:]:
    source = Path(argument).resolve()
    if not source.is_relative_to(boundary) or source.name not in {"samples.jsonl", "checks.jsonl", "summary.json"}:
        raise SystemExit("Only this experiment's raw/derived evidence files may be packed")
    target = Path(str(source) + ".gz")
    with source.open("rb") as original, target.open("wb") as packed:
        with gzip.GzipFile(filename="", mode="wb", fileobj=packed, mtime=0) as stream:
            shutil.copyfileobj(original, stream)
    with gzip.open(target, "rb") as check, source.open("rb") as original:
        while True:
            a, b = check.read(1024*1024), original.read(1024*1024)
            if a != b:
                raise SystemExit("Packed evidence did not round-trip")
            if not a:
                break
    print(source.name, source.stat().st_size, "->", target.stat().st_size)
