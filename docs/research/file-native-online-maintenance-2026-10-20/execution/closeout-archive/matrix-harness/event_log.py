"""Lossless event append; caller owns the lock and each completed gzip member."""
import gzip
import json
from pathlib import Path


def append_event(path,row):
    path = Path(path)
    line = json.dumps(row,separators=(',',':'))+'\n'
    if path.name.endswith('.jsonl.gz'):
        # Closing every append completes its CRC/length trailer. Standard gzip
        # readers consume all concatenated members without discarding raw rows.
        with gzip.open(path,'at',encoding='utf-8',compresslevel=1) as stream:
            stream.write(line)
            stream.flush()
    else:
        assert path.suffix == '.jsonl', 'explicit-jsonl-or-jsonl-gzip-output-required'
        # Preserve Driver's existing text-mode encoding/newline behavior.
        with path.open('a',encoding='utf-8') as stream:
            stream.write(line)
            stream.flush()
