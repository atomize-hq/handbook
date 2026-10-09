# Foundation retained bytes

These JSON files were emitted by the original production writer at
`277674e2f9a6350e35c5544fbeb7857f51a6d45d`, before Packet-1 product edits. They are
unchanged examples of the ordinary descriptor, grant locator, committed request
locator and receipt. The default `work_state_retained` target requires exact
canonical round trips, original identity and receipt fingerprint domains, omitted
display/role values, and singleton ordinary interpretation.

`capture_baseline.rs` is the historical capture harness. It uses the API at that
commit deliberately; it is not a current public operating path. At that commit,
copy it to `crates/sdk/tests/work_state_v1_capture.rs` and build the target. Preserve
the resulting executable outside Cargo's target tree before changing product
source. Run it with `HANDBOOK_RETAINED_V1_CAPTURE` naming a fresh external directory.
It creates real committed, prepublication and publication-attempt stores through
the original SDK/native owner, including the original physical root and lock.

The ignored recovery test requires `HANDBOOK_RETAINED_V1_ROOTS` to point at those
fresh captured roots. Run with:

```text
cargo test -p handbook-sdk --test work_state_retained --config 'profile.test.opt-level=1' -- --include-ignored --test-threads=1
```

Keep captured roots in place during this test. Copying/rebasing a retained store
cannot establish its original physical-lock proof. The test consumes each original
prefix once, applies actual supported scope/G/revocation/reset transitions, checks
exact surviving retained records, and recovers through saved original locators.
It checks positive noncommit after authority loss and uncertainty after possible
publication, without substituting a successor. Use a newly captured directory for
another run; existing completed history is retained.
