"""Control-only guard checks: does not build or invoke any engine/corpus."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path('/home/spenser/file-native-online-linux-20261020')
GUARD = ROOT/'source/native_guard.py'
OUT = ROOT/'evidence/control-smoke-v5'
CLAIM = ['--claim-id', 'CONTROL-SMOKE-F01-F12', '--ledger-sha256',
         '2ff4e277a44c3dea99f53d5b098250e78a50c40d63539022bf365f4cc048c82c',
         '--reserve', 'source=4096:2', '--reserve', 'evidence=262144:10']


def main():
    OUT.mkdir(exist_ok=False)
    rows = []
    tests = [
        ('normal', "import time; print('control'); time.sleep(.2)", 3, [], []),
        ('nested', "import subprocess,sys; subprocess.run([sys.executable,'-c','import time; time.sleep(.2)'],check=True)", 3, [], []),
        ('orphan-new-session', "import subprocess,sys,os; subprocess.Popen([sys.executable,'-c','import time; time.sleep(60)'],start_new_session=True); os._exit(0)", 3, [], ['root-exit-left-descendants']),
        ('timeout', 'import time; time.sleep(60)', .3, [], ['time-cutoff']),
        ('memory', 'import time; a=bytearray(64*1024**2); time.sleep(1)', 3,
         ['--process-limit', str(24*1024**2)], ['heavy-or-process-rss']),
        ('threaded-children-cpu', "import subprocess,sys,threading; jobs=[threading.Thread(target=lambda:subprocess.run([sys.executable,'-c','import time; end=time.monotonic()+2; a=bytearray(8*1024**2); exec(\"while time.monotonic()<end: sum(range(10000))\")'],check=True)) for _ in range(2)]; [t.start() for t in jobs]; [t.join() for t in jobs]", 5, [], []),
        ('reservation-overrun', "from pathlib import Path; import time; Path('/home/spenser/file-native-online-linux-20261020/fixtures/control-only-overrun').write_bytes(b'x'*8192); time.sleep(2)",
         3, [], ['reservation-overrun:fixtures']),
    ]
    for name, code, seconds, extra, expected in tests:
        receipt = OUT/(name+'.json')
        command = [sys.executable, str(GUARD), '--receipt', str(receipt),
                   '--seconds', str(seconds), '--pin', 'CONTROL-ONLY-NO-ENGINE', *CLAIM,
                   *extra, '--', sys.executable, '-c', code]
        result = subprocess.run(command, capture_output=True, timeout=20)
        row = json.loads(receipt.read_bytes())
        assert row['censors'] == expected, row
        assert row['process_closure_verified'] and not row['observation_errors'], row
        assert (result.returncode == 0) == (not expected), row
        assert row['memory_sample_count'] >= 2, row
        assert row['blocked_launch_positive_rss'] > 0 and row['monitors_joined'], row
        assert row['rss_sampling_cadence_qualified'], row
        assert row['execute_started_epoch'] < row['absolute_deadline_epoch'], row
        rows.append(dict(case=name, exit=result.returncode, censors=row['censors'],
                         process_closure_verified=True,
                         memory_max_interval_seconds=row['memory_max_interval_seconds'],
                         intervals_over_40ms=row['memory_intervals_over_40ms'],
                         receipt_sha256=hashlib.sha256(receipt.read_bytes()).hexdigest()))
        if name == 'threaded-children-cpu':
            import gzip
            with gzip.open(row['memory_samples_path'], 'rt') as stream:
                samples=[json.loads(line) for line in stream]
            assert max(len(s['members']) for s in samples)>=3
        if name == 'reservation-overrun':
            (ROOT/'fixtures/control-only-overrun').unlink()
    receipt = OUT/'free-refusal.json'
    result = subprocess.run([sys.executable, str(GUARD), '--receipt', str(receipt),
                             '--seconds', '3', '--pin', 'CONTROL-ONLY-NO-ENGINE',
                             *CLAIM,
                             '--free-floor', str(10**18), '--', sys.executable,
                             '-c', 'raise AssertionError("must not launch")'],
                            capture_output=True, timeout=10)
    assert result.returncode != 0 and b'preflight-free-floor' in result.stderr
    assert not receipt.exists()
    (OUT/'free-refusal.stderr').write_bytes(result.stderr)
    rows.append(dict(case='free-refusal', refused_before_launch=True))
    receipt = OUT/'setup-cutoff.json'
    result = subprocess.run([sys.executable, str(GUARD), '--receipt', str(receipt),
                             '--seconds', '.000001', '--pin', 'CONTROL-ONLY-NO-ENGINE',
                             *CLAIM, '--', sys.executable,
                             '-c', 'raise AssertionError("must not launch")'],
                            capture_output=True, timeout=10)
    assert result.returncode != 0 and b'setup-exhausted-deadline-before-spawn' in result.stderr
    assert not receipt.exists()
    (OUT/'setup-cutoff.stderr').write_bytes(result.stderr)
    rows.append(dict(case='setup-cutoff', refused_before_launch=True))
    summary = dict(status='passed-control-only', epoch=time.time(), cases=rows,
                   guard_sha256=hashlib.sha256(GUARD.read_bytes()).hexdigest(),
                   no_engine_build_or_invocation=True)
    (OUT/'summary.json').write_text(json.dumps(summary, indent=2)+'\n')
    print(json.dumps(summary, indent=2))


if __name__ == '__main__':
    main()
