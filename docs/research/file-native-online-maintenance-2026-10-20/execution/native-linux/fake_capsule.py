"""CONTROL ONLY capsule-shaped client of the explicit legacy Driver bridge."""
import argparse
from driver import Driver

parser = argparse.ArgumentParser()
parser.add_argument('--base', required=True)
args = parser.parse_args()
driver = Driver(args.base)
assert driver.call(args.base+'/disposable/store', 'resolve', request_id='adapter-control')['result']['control_only']
assert len(driver.events) == 1
