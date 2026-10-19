# Navigation worker admission design

Read-only handoff from task `01a155e3-52dc-7a53-8b44-4c1c99ba8d10`.
No source edits, builds or fixtures have been admitted. The original four-hour
deadline is October 19, 2026 at 8:38:51 p.m. America/Indianapolis
(`2026-10-20 00:38:51 UTC`), including closeout; follow-up messages do not reset it.

Manual tracing identified `Store::compute -> graph::transition -> projection::delta`.
Profile 2 calls the complete collection renderer unconditionally.
`Store::select_file` recomputes under the candidate's historical profile and
checks exact replay dependencies. Initialization uses `projection::full`;
B materializes a complete generation separately. A consumes changed projection
paths; packet replay, audit, checkpoints and GC retain projection/source obligations.
GitNexus evidence remains UNKNOWN, not a safe edit verdict.

The proposed implementation preserves profile-1/2 rendering and replay bytes,
and identifies a new profile explicitly in mandatory immutable metadata. It first
compares final exported document facts and ordered relationships to skip navigation
generation for payload-only or unrelated metadata edits. A separate bounded-parts
increment uses stable-ID routing, local splits/merges and measured large-adjacency
handling. Stable target IDs avoid rewriting incoming links on target rename.

Counters must include header and relationship range work, namespace scans,
emitted parts/bytes, changed paths and replay dependencies. Compare against an
independent complete graph-to-files oracle. B generation remains separately timed.

Existing N10k geometry contains 126,976,000 authored source bytes. A source/view
requires 253,952,000 bytes; B source with two generations requires 380,928,000,
before metadata. The worker proposes a sparse checkout, a 448 MiB bounded build
stage, retention of executables and sequential disposable fixtures, targeting
704 MiB / 80,000 files peak within a 768 MiB / 100,000-file allowance. This is a
forecast, not measured success. Resource admission remains pending.

A launcher availability check unexpectedly staged 1,050,883,003 logical bytes /
15,042 files. Reparse targets were excluded. Automatic approval review rejected
two exact-subtree native PowerShell cleanup attempts before execution, stating
"blocked by policy". The residue remains counted; no alternative deletion route
is authorized. Further package-manager launcher retries have been stopped.

The exact task-created subtree is recorded in the local before/after receipts at
`C:/file-native-navigation-20261019/admission/`. Its parent is preexisting and is
not disposable task ownership. The original stores, archive, final H101006 clone
and shared source have not been modified.
