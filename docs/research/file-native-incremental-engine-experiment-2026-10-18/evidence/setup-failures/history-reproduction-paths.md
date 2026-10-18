# Saved history reproduction paths

The already running history process uses its captured `base/store` root. Its
source ZIP is unchanged. A prospective script edit chose `base/disposable/store`
to satisfy the lifecycle guard, but left four oracle/ledger/candidate paths at
`base/store`. Inspection found this before any future invocation of that script.

The saved reproduction now derives every such path from one `store` value.
Python 3.11.9 syntax compilation passed, and direct inspection found no remaining
literal old-root accesses in `history_run.py`. No replacement history workload
was launched. The active lane's original setup refusal and separately bounded
quiescent relocation remain required observations.

GitNexus impact for `history_run` returned target-not-found and risk UNKNOWN.
The index does not cover this experiment. Direct inspection identifies
`serial_windows_v2.py` as the launcher; no Rust symbol or format changes.
