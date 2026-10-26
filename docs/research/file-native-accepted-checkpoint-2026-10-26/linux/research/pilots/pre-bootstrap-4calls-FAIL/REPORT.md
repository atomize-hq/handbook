# Linux A/N64/H3 pilot: FAIL at initial packing precondition

The one real pilot attempted four engine invocations, then stopped. Initialization, lifecycle setup and resolution of the initial accepted result succeeded. Initial packing returned exit 2 with `refused:explicit-discovery-bootstrap-required`. The retained recipe invokes `h.pack(root)` on a new store without `bootstrap=True`; the SDK forwards no bootstrap field, and engine `src/rolling.rs:77` explicitly requires it for first discovery. This is a pilot fixture precondition omission; the requested online overlap was not exercised. No engine correction or subsequent engine invocation was made.

| Call | Action | Native PID | Exit |
|---|---|---:|---:|
| engine-01-init | init | 674919 | 0 |
| engine-02-lifecycle | lifecycle | 675230 | 0 |
| engine-03-resolve | resolve | 675233 | 0 |
| engine-04-packing | packing | 675236 | 2 |

N64 was supplied and the initial accepted result lists 64 changed documents. H2/H3, the three held actors, reader hold beyond 15 seconds, physical activation, old/fresh result comparison, two final drainage calls and actual loose deletion were not reached. The complete independent oracle ran zero times. No online gate receives passing credit; Linux remains PENDING and the full matrix is unreleased.

Each of the four calls has retained input, stdout, stderr, native PID/start identity, positive RSS observed before stdin delivery, complete stdin delivery, both output EOFs, native wait and empty process-group evidence. The host monitor joined, closure passed, the supervisor observed the owned tree empty twice, all four process identities are gone, and no conflicting holder was found on existing lock files. Worker exit 0 means its failure report was written; the semantic pilot result is FAIL.

The pilot used the freshly built CLI SHA-256 `b46977f9440b5dea662c0ce547a7ca9b9b581358bf7755abf0a54b6c4df5873b`, bound to baseline `0628322e4b48e0efad19cdc32238a622d4aaa7a8` plus the previously authorized telemetry test n1-to-n10 patch. All 25 corrected source files and all 26 delivered SDK members match after execution; original SOURCE_FREEZE remains unchanged. The earlier corrected 31/31 full-test result remains prior evidence and was not rerun. The source is not represented as unchanged 0628322.

About-200ms sampling observed 49,922,048 bytes peak for the pilot tree plus supervisor; no resource stop occurred. Native free space after final verification was 41,619,992,576 bytes, above the 20 GiB floor. The measured pilot allocation at stage close was 112,668,672 bytes / 648 files. These are samples and a bounded task footprint, not hard peak or 40ms qualification claims or a global experiment census.

Two adapter startup failures occurred before any engine call and are retained separately: unavailable native_guard import through a pure parser helper, then a stale admission-event reference. Only task-owned wiring was corrected: copied the original strict parser verbatim, replaced the administrative event with a native-start event, and retained the originally prepared exact-two-pass successor. Original SDK/oracle sources and engine sources were not changed by this pilot. The first actual engine failure was preserved and execution stopped.

The disposable store, raw logs, source bindings, resource samples, detailed failure, pre-engine prefixes and pending matrix ledger remain intact. Any revised pilot needs a separate coordinator disposition of the missing bootstrap precondition; this assignment did not retry or expand the matrix.

Evidence: [/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/result.json](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/result.json), [native calls and failure](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/pilot-result.json), [supervisor](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/pilot-native-run-supervision.json), [source check](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/corrected-source-final.json), [retained store inventory](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/retained-store-inventory.json), [adapter bindings](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/adapter-source-bindings.json), [pending matrix](/var/tmp/file-native-online-pilot-n64-h3-20261025-01a16fd3/matrix-ledger-pending.json).

Outbound `send_message_to_thread` is not exposed in this session (one completion-time discovery returned no tool). The coordinator handoff is saved locally; no delivery is claimed.
