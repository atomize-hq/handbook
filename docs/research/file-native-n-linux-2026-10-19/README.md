# Native Linux N source-delta qualification

The required bounded native delta passed on Linux/ext4 at exact candidate
`6e66a7523612b80386cf9cc2b96d5341579b54fd`. This is a fresh N lane, separate
from the closed Q qualification. No engine source, dependency, format contract,
production adoption or API freeze was changed here. The structured-format
standard remains proposed; existing owner/versioned contracts and Contracts
JSON remain controlling.

The parent narrowed native coverage to focused N128 plus a small geometry tier
above 512 records. N128/N1k A/B completed. N10k A had already started when that
steering arrived and was killed before selection; its prefix remains preserved
and **unqualified**. N10k B was never launched. Windows owns the main tier
measurements; its separate 120-second censor is not this lane's cancellation.

## Exact bindings

| Input/output | Binding |
| --- | --- |
| Candidate source | `6e66a7523612b80386cf9cc2b96d5341579b54fd` |
| Experimental `src/` Git tree | `6c6e475e677c4b36ab4b1cbc72bb5202e4668d86` |
| Native release executable SHA-256 | `5295ce01770f9c4ba8b88bd4848f745c23c327ca78d6cd1a80770c2e1b69e89e` |
| Source/harness manifest SHA-256 | `ef27b12471db7cc09d75b9914f934ec4ba4172e599ee09d8405da48ab68fb425` |
| Authenticated return SHA-256 | `248c413ea513a1f310b8064f29319767930ca75fbb9576ee9dbd5f5060b79895` |
| Preserved Q executable SHA-256, rechecked | `e70d47a82e1c51a989395f0025f85859e74aed07c07e482e0a6b37913e76e854` |

[Source manifest](evidence/source-manifest.json) records exact Git blobs, lengths
and SHA-256 for all 163 transferred files, including each Rust/Cargo file,
consumer, controlling Python harness and the packaged profile-1 fixture archive.
`prepare.py` exported committed blobs, not checkout-normalized source. Every blob
was checked on Linux before compilation and again at closeout. The native binary
is 2,324,504 bytes. Its hash intentionally differs from Windows' executable.

Native root: `/home/spenser/file-native-n-linux-20261019`, on
`/dev/nvme0n1p2`, ext4, x86_64 Linux 6.16.8-1-MANJARO/glibc 2.42.
The prior `/home/spenser/file-native-q-linux-20261019-final` remains intact.
The [environment receipt](evidence/native-raw/environment.json) binds compiler,
Cargo and Python versions. No network dependency resolution or Git fetch ran.

## Executed coverage

| Mechanism | Actual receipt/result |
| --- | --- |
| Locked offline workspace tests, `-j2` | Two meaningful bounds/codec tests passed; workspace consumers linked. [Output](evidence/native-raw/tests.stdout). |
| Fresh native release build, `-j2` | Passed in 11.10 s, fresh target; dev/test debug info and incremental compilation disabled. [Build stderr](evidence/native-raw/build.stderr). |
| Actual `focused.py --exe … --base … --capsule` | 39 navigation cases, 153 calls; full enclosing focused stage 130.46 s. [Summary](evidence/native-raw/focused-data/summary.json), [events](evidence/native-raw/focused-data/events.json). |
| Full adapted recovery capsule | All 24 cases / 282 calls passed in 72.63 s inside the focused outer cap. [Summary](evidence/native-raw/focused-data/recovery-capsule/summary.json), [resources](evidence/native-raw/focused-data/capsule-resources.json). |
| Independent complete graph/YAML/source oracle | Every completed focused and measurement boundary checked all graph indexes, ordered relationships, immutable projection and exact authored bytes. A-current and B-retained-generation materialization verified. |
| N128/N1k, A/B grouped-YAML measurements | Four cells, 13 changes each, 52 full independent checks. Payload/batch payload/label, single/batched title, path, relationships, high degree, split, merge, external edits and deletion. [Cells](evidence/native-raw/matrix-cells.json), [derived counters](evidence/native-raw/derived-growth.json). |
| Mixed self-describing JSON and absent navigation | A/B all-complete and mixed fixtures, whitespace-only and key-order stability, loss/restoration of completeness, absent-navigation packet/replay/checkpoint; covered by focused assertions. |
| Ordered typed duplicate/external high-degree adjacency | Added 600 relationships to the existing external edge, producing exactly 601 ordinals across three parts, A/B. Moving the target preserved the incoming record and all three adjacency digests. [Supplement](evidence/native-raw/supplement-data/summary.json). |
| Historical profiles 1/2/3/4 | Authenticated ordinary/packet profile-1 candidates; sealed profile-2 candidate and genuinely pending A plan; separate sealed profile-2/3 packet replay and checkpoint carry-forward, then profile-4 successors. Original publication/candidate bytes asserted unchanged. [Profile 2](evidence/native-raw/historical2-data/summary.json), [profile 3](evidence/native-raw/supplement-data/summary.json). |
| Fact dirtiness control | Ten native profile-2/profile-3 control rows at N128; profile 3 payload/label/batch-payload emitted zero parts and scanned zero D rows; profile 2 scanned 128. Both final roots passed the profile-aware oracle. [Control](evidence/native-raw/stage1-data/summary.json). |
| Strict missing/corrupt evidence and retained packet owner | A/B missing/corrupt grouped navigation, context/provenance and missing projection tree refused audit without repair; restored evidence audited successfully. Accepted packet outcome survived checkpoint/retire/collect. [Events](evidence/native-raw/supplement-data/events.json). |
| External edits and interrupted A plans | Navigation divergence preserved; actual capsule covered namespace inversion, recreated external divergence, correcting successors, genuinely pending-plan backup/restore and valid successors. |
| Full independent operational oracle | Two supplemental stores plus all four actual A/B N16/N128 restored stores checked complete accepted membership/outcomes, retained graphs/projections/sources, pins, candidates, packet and generation obligations. [Restored A128](evidence/native-raw/restored-operational-A-128.json), [B128](evidence/native-raw/restored-operational-B-128.json). |
| Ordinary Bash discovery/copy/extraction | `ordinary.sh` used listing, text search, sed/awk, cp and SHA-256 only. B path captured once before a successor; exact authored bytes and 23 copied projection files independently verified in each mode. [A discovery](evidence/native-raw/bash-A/discovery.txt), [B discovery](evidence/native-raw/bash-B/discovery.txt). |

The supplied recovery capsule was invoked through the candidate's focused
adapter: it substitutes the independent profile-4 oracle and 120-second sampled
child runner; engine and original recovery source were unchanged. Recovery
receipts include fences, durable pins, eligible preparations, packets, missing
selector refusal, allocation-transfer interruption, checkpoint/retire/repeated
collection, half-preparation restoration, skipped A work and selected-once B.

## What the local-update receipts show

These are individual observations, not latency distributions or an OS ranking.
Filesystem census/RSS observers overlapped the measurements; caches were not
controlled. Full B generation cost remains separate from navigation work.

| Case | N128 A | N1k A | N1k B |
| --- | ---: | ---: | ---: |
| Payload preparation | 0.233 s | 0.189 s | 6.990 s |
| Payload D rows / emitted navigation parts | 0 / 0 | 0 / 0 | 0 / 0 |
| Growing title: document visits | 129 | 511 | 511 |
| Growing title: D rows / E rows returned | 0 / 4 | 0 / 2 | 0 / 2 |
| Growing title: emitted navigation parts/bytes | 2 / 19,980 | 2 / 78,407 | 2 / 78,407 |
| Growing title: actual changed navigation files | 1 | 1 | 1 |
| Growing title: replay objects/files | 5 / 4 | 6 / 4 | 6 / 4 |

At N1k, a growing title rebuilt its 510-member group plus the directly touched
header; it did not load the 490 sibling records. It changed only
`navigation/groups/0.yaml`. The static descriptor is counted as emitted work even
when its bytes remain identical. Generated parts are not physical writes.

This still does substantial work: that N1k title preparation examined 16,830
E-range leaf pairs and made 1,530 E-range page visits to return two edges. These
repeated bounded range probes must not be called zero scanning. The high-degree
title inspected 132 outgoing rows and emitted three parts/88,619 bytes. The
601-edge supplement deliberately checks a larger multipart exception.

N1k B payload preparation includes 6.794 s of complete generation construction;
selection separately records 19.33 ms of generation verification. N1k B growing
title preparation was 6.711 s, including 6.465 s of complete B generation work.
The 52 raw rows preserve all range rows, examined pairs/pages, emitted bytes,
changed paths and replay-dependency counts. No native N10k/N100k performance
claim follows.

The original `matrix_native.py` is retained unchanged as executed before parent
steering; its planned N10k loop is superseded and must not be relaunched. A fresh
reproduction must select only the parent-approved N128/N1k scope, a new root and
its own prospective budget. The recorded deadlines here never reset. Inclusive
engine clocks overlap and must not be added.

## Bounds, preservation and original non-successes

Start was fixed at **2026-10-19T22:54:45Z**; execution cutoff
**23:44:45Z**, lane deadline **23:54:45Z**, with ten minutes reserved for
closeout. The original N deadline **2026-10-20T00:38:51Z** was not extended or
borrowed. Required execution completed by 23:07:20Z, and authentication began
immediately afterward. [Deadline/admission inputs](evidence/deadlines.json).

The parent supplied a combined global forecast of 40,828,300,920 bytes /
4,282,696 files including both native lane allowances and the internal 1 GiB /
100,000-file reserve, under the human-approved 40 GiB / 5,000,000-file ceiling.
This lane did not treat 768 MiB / 100,000 files as a new global grant. Its local
checkout, shared Git growth, transfer, fresh source/build, fixtures, archive and
return copies are counted. Initial full checkout was 258,508,264 bytes; exact
sparse source reduced it to 2,284,079 before fixture execution. No source fetch,
new dependency or H100k history was created.

The native monitor stopped admission above 700 MiB / 95,000 files, leaving space
inside the lane allowance for local/evidence/Git growth. The separate matrix
watcher observed 293,453,827 bytes / 14,024 files at peak, including retained
builds, fixtures, cancellation prefix and raw evidence. Its minimum free space
was 42,004,840,448 bytes, above 20 GiB. The 286,115,824-byte / 13,653-file native
archive-end census includes the 753,508-byte return archive; the final receipt
and its own output add small closing bytes. The one new temporary bounds-test
root outside the owned root is explicitly counted: two files / 1,049 bytes.
Earlier continuous logical-byte peaks were not retained and remain **missing**;
earlier stage-end censuses and enforced caps remain available.

All preserved prior native root/cached-registry byte/file censuses matched
before/after. [Comparison](evidence/preserved-scope-comparison.json). The shared
`.cargo` inventory is separately recorded in closeout; it overlaps the registry
scope and must not be added twice. Local initial/shared-Git and final source/
return censuses are retained separately.

Per-child limits were 120 s, point readers 256 MiB and other children/checkers
2 GiB. The entire focused process group, including the oracle and descendants,
had a hard 1,800-second cap. Largest completed ordinary driver call was 35.87 s;
all 769 returned sampled resource rows had measurements, with largest sampled
engine-child RSS 7,536,640 bytes. Native group peak was 634,134,528 bytes during
compilation, below the 3 GiB aggregate bound; focused group peak was 37,879,808.
These are sampled RSS observations. The original supervisor's own high-water
sample is missing after exit, not zero.

The extra matrix descendant guardian covers nested process groups, including
the checker, and would stop/kill all tracked groups on a censor. Its observed
descendant-plus-guardian RSS, including a conservative 64 MiB allowance for the
independent watcher, peaked at 164,646,912 bytes. Original per-stage samples
cover the earlier interval; the later guardian does not retrospectively measure
it. [Guardian](evidence/native-raw/descendant-guardian.json),
[coexistence](evidence/native-raw/coexistence.json),
[stage receipts](evidence/native-raw/stages.json).

Original non-successes remain visible:

- An extra conservative overlapping N1k-B-plus-N10k-B forecast, with a 64 MiB
  return/Git allowance, refused 805,964,746 bytes against 805,306,368. That
  hypothetical overlap was never started. The actual serial cells had separate
  admissions and removed only already-verified successful disposable fixtures.
  [Original refused snapshot](evidence/native-raw/n10k-conservative-admission.json).
- Parent steering stopped the already-running N10k A initialization at
  23:06:49Z. The controller was stopped and the owned measure/engine and controller
  process groups killed. Its 1,879-file / 23,765,782-byte prefix has no
  `SELECTED.json`, no summary and no qualification result. It is preserved; N10k
  B was not launched. The matrix controller's exit -9 remains in raw receipts.
  [Cancellation receipt](evidence/native-raw/scope-cancel-report.json).
- A cached direct GitNexus `detect-changes` attempt refused this unregistered
  worktree. Graph coverage is **UNKNOWN**, never clean. No bootstrap, package
  install or cache deletion was retried. A local inline metrics inspection also
  failed PowerShell parsing before any remote work; the separately bound
  `collect_metrics.py` replaced it. [Inspection attempts](evidence/inspection-attempts.json).

No engine defect was found in the completed scope. These receipts do not rerun
Q's complete native I/O/locking qualification, prove physical power-loss
durability, or replace the parent's Windows/Mac/integration review.

## Return and scope review

[The authenticated archive](evidence/native-raw.tar.gz) contains 120 raw files /
7,970,157 uncompressed bytes. Every inventory member and the archive digest was
verified before local extraction; native external harness bytes match the local
scripts. [Transfer verification](evidence/transfer-verification.json),
[review summary](evidence/review-summary.json),
[raw inventory](evidence/native-raw/return-inventory.json).
Build outputs and engine datasets remain on Linux. The original cancellation
receipt's unrelated host process arguments remain only at its preserved native
path; the bounded return includes task scope and the original receipt hash.

Manual scope tracing: external scripts call the pinned executable and independent
`focused.Driver`/`oracle.verify`/`verify_operational.verify`; no existing engine
symbol was edited. The relevant existing execution boundary remains
`Store::compute -> graph::transition -> projection::delta -> navigation::delta`,
with prepared-profile replay in selection, full B construction, A refresh,
packet/checkpoint and lifetime operations exercised by the actual capsules.
Only this new report/external-harness/evidence directory is committed. The parent
owns review, integration, remediation and landing.

Raw evidence retains exact authored padding and original output newlines. Initial
`diff --check` reported those raw-only bytes; the evidence-specific whitespace
attribute excludes them from style checks without changing any authenticated byte.
