"""Control-only check of the thin driver and explicit legacy interface bridge."""
import json
from pathlib import Path
import native_guard as guard
from native_driver import Driver, legacy_driver_module

ROOT = guard.ROOT
OUT = ROOT/'evidence/driver-control'
FAKE = ROOT/'source/fake_protocol.py'


def main():
    OUT.mkdir(exist_ok=False)
    suite = 'CONTROL-ONLY-DRIVER-NO-ENGINE'
    root = ROOT/'fixtures/control-driver/disposable/store'
    assert not root.exists()
    driver = Driver(FAKE, OUT/'events.jsonl', expected_sha256=guard.digest(FAKE),
                    suite_pin=suite, role='control-only-fake-peer')
    assert driver.call(root, 'resolve', request_id='control')['result']['control_only']
    assert driver.call(root, 'control-error', code=2)['result']['control_only']
    try:
        driver.call(root, 'resolve', control_sleep=10, timeout=.1)
    except AssertionError:
        assert driver.events[-1]['violations'] == ['time-censor']
        assert driver.events[-1]['engine_root_reaped'] and driver.events[-1]['engine_monitor_joined']
    else:
        raise AssertionError('timeout-did-not-refuse')
    module = legacy_driver_module(FAKE, expected_sha256=guard.digest(FAKE), suite_pin=suite)
    legacy = module.Driver(ROOT/'fixtures/control-driver-legacy')
    assert legacy.call(root, 'inspect')['result']['control_only']
    try:
        legacy.call(root, 'resolve', exe_override=Path('/usr/bin/true'))
    except AssertionError as e:
        assert 'unbound-executable-override' in str(e)
    else:
        raise AssertionError('unbound-override-did-not-refuse')
    root.mkdir(parents=True)
    selector = dict(publication='control-only', digest='0'*64)
    (root/'SELECTED.json').write_text(json.dumps(selector))
    assert legacy.head(root) == selector
    assert legacy.publish(root, 'fake-publish', packet=True)['result']['request']['packet']
    events = driver.events+legacy.events
    assert len(events) == 5
    assert all(x['engine_root_reaped'] and x['engine_monitor_joined'] for x in events)
    assert all(x['initial_blocked_rss'] > 0 for x in events)
    row = dict(status='passed-driver-control-only', no_engine_invocation=True,
               fake_peer_sha256=guard.digest(FAKE), driver_sha256=guard.digest(Path(__file__).with_name('native_driver.py')),
               guard_sha256=guard.digest(guard.__file__), events=len(events),
               cases=['resolve', 'expected-error-exit', 'timeout-refusal',
                      'legacy-call-events', 'unbound-override-refusal', 'legacy-head-publish-packet'])
    (OUT/'summary.json').write_text(json.dumps(row, indent=2)+'\n')
    print(json.dumps(row, indent=2))


if __name__ == '__main__':
    main()
