# File-native navigation: bounded implementation and qualification

This standalone research change removes navigation work from edits that do not
change navigation facts, then measures bounded grouped YAML for edits that do.
The completed matrix supports those work reductions. It does not establish a
universal latency improvement, a production format decision, or complete N10k B
coverage. Native/platform integration and the P gate belong to the coordinator.

## Revisions and reproducibility

Work was performed on `codex/file-native-navigation-20261019` in
`C:/Users/spmcc/.codex/worktrees/file-native-navigation-20261019/handbook`.
Owned runtime evidence is under `C:/file-native-navigation-20261019`.
The fixed original task deadline is 2026-10-20 00:38:51 UTC. No later continuation
reset that deadline or the 120-second ordinary-child limit.

| Purpose | Core Git commit | Windows executable SHA-256 |
| --- | --- | --- |
| Separately measured increment 1: fact dirtiness | `9ba24afc880d69883474a56af6b3973f66730a5e` | `75d32a83855e1fdd2e9f1c00608f64d20688f91cb6ed2307ac991b3be209e7fc` |
| Ordinary matrix and complete Windows focused suite | `6e66a7523612b80386cf9cc2b96d5341579b54fd` | `6336944d13967dfce04e4cc40b3459cdac4718b223dda147dcd202598ad50558` |
| Frozen final engine; new-profile genesis guidance correction | `4bde156323812ef18c0dea6798cfc2108aaad933` | `d4b7c495abfc029e8fabd6fcfc75501c5de04ce72dd38847a69bc8824022303f` |

The final engine differs from the measured matrix core only in omitting the
inherited root README at new profile-4 genesis. It retains that README byte for
byte for legacy profiles and upgrades. Eight focused final-executable checks
verify this delta; the ordinary matrix is **not relabelled** as a final-core run.
The final evidence commit changes harnesses, documentation and receipts only.
The complete 16-file Rust/Cargo source binding is in
[`results/source-binding.json`](results/source-binding.json).

Windows used Rust/Cargo 1.89.0, `x86_64-pc-windows-msvc`, LLVM 20.1.7,
Python 3.11.9, Windows build 26200 and NTFS with 4,096-byte allocation units.
Cargo test/build used locked, offline workspace builds with `-j2`. Both Rust
bounds/codec tests passed and consumers linked. Retained Cargo/compiler logs
identify each binary. No dependencies or Cargo contracts were changed.

The proposed structured-data-format standard, consultation answer, experiment
brief and recovery contracts were read. This experiment does not approve that
standard's proposed defaults, migrate Handbook sources or create a production
artifact family. The initial checkout was
`f0904a63cb2500b9bea3a4034d19cbe38bfac729`; original Rust core was
`07a8574e581e66ba58d7caac87238aa5f93918be`.

## The two increments

Profile 3 records navigation dirtiness from stable identity, title, kind, source
location and actual ordered relationship changes. Payload and label changes
preserve navigation output. It keeps the monolithic complete YAML renderer for
fact changes, providing a separately measured intermediate control.

The first increment was measured independently at N128 before grouped routing:
five operation types on each of profiles 2 and 3, with engine audit and the
recovery experiment's independent complete oracle after every operation.
Payload, label and batch-payload edits on profile 3 produced zero navigation
parts and zero D-range rows. Profile 2 scanned 128 headers and regenerated one
part. Title and relationship changes still scanned 128 headers on profile 3.
The first payload observation was 104.67 ms on profile 3 versus 53.22 ms on
profile 2; this stage establishes eliminated work, not a timing distribution.
Full original stage-1 stdout, metrics and oracle results are retained.

Profile 4 adds stable-ID SHA-256 binary-prefix routing. Groups contain at most
512 document records and 256 KiB. A fact update finds its previous group through
checked projection entries, reads its membership and point-reads the relevant
headers. Only affected groups split; leaf siblings merge when a shrinking
selected group and its sibling fit. Empty sibling probes and boundary-page
work are included in low-level counters. There is no global document directory
to rewrite for each fact update.

Relationships are directed, typed, ordered, duplicate-preserving, and retain
external status and ordinal. At most 128 small relationships are inline.
Larger adjacency uses ordinary YAML parts of at most 256 relationships and
256 KiB, referenced by a directory and part count. A single existing valid
relationship exceeding that byte target is an explicit counted exception,
bounded by the existing object limit plus 1 KiB framing. An oversized document
record fails closed. Rebuilding a high-degree document still reads and renders
its adjacency; it is not constant-time work.

Authored Markdown, YAML, JSON and text bytes remain exact. Relationships point
to IDs, so changing a target's path updates that target's location record rather
than rewriting incoming relationship text. A small static `navigation.yaml`
describes the supplementary layout when supplementation is needed. There is no
generated Markdown navigation or per-document guide.

Supplementation is omitted for the bounded `self-describing-json` dialect:
strict duplicate-free closed JSON must contain the exact ID, title, kind and
ordered typed relationships; the ordinary tree supplies location. Other kinds
still require supplementation. Whole-source-complete and mixed-source cases,
whitespace/key-order-only changes, completeness loss/restoration, packet replay
and checkpoint carry-forward were checked. This is a narrow completeness rule,
not a general parser for arbitrary authored formats.

New profile identity is explicit even when no navigation file exists. It is an
immutable optional field on the existing provenance FileSpec, omitted from all
legacy serialization. Both packet paths preserve it, as do checkpoints.
Replay uses the prepared immutable profile, never the currently requested
environment profile. Legacy profiles 1/2 retain their renderer and candidate,
plan and packet rules. A remains incremental; B still constructs a complete
generation before selection. No backend replacement, packing or production
adoption is included.

## Verification and coverage

`oracle.py` imports no engine renderer or resolver. It decodes checked complete
C1 trees using the earlier independent verifier, validates object hashes and
parent bounds, reconstructs graph indexes, parses emitted YAML, compares all
typed directed ordered relationships including duplicates/external targets,
and checks exact authored source bytes. It checks declared-current A views and
one selected B generation, including regular-file link counts. It does not
treat the engine's own audit as an independent oracle.

Windows focused qualification on the matrix core passed 39 cases / 153 engine
calls plus the actual recovery capsule's 24 cases / 282 calls. This includes
split/merge, adjacency collapse, target-location changes, deletion, external
edits and interruption, navigation absence, legacy frozen ordinary/packet
candidates and genuinely pending A work. The eight final-core genesis checks
passed separately. Ordinary PowerShell discovery and extraction on final-core
A/B fixtures used listing, YAML text/JSON scalar reading, Copy-Item and
Get-FileHash with no engine running during extraction. Both copied 4,096 exact
source bytes with SHA-256
`16f81f0e49aa829b7b6c41185950a50ae8a3e59ee3f0a8ecd7c27cb846379f1f`.

| Matrix scope | Completed cells | Operations per cell | Status |
| --- | ---: | ---: | --- |
| N128, A/B, profiles 2/3/4 | 6 | 13 | Complete views and immutable oracle |
| N1k, A/B, profiles 2/3/4 | 6 | 13 | Complete views and immutable oracle |
| N10k A, profiles 2/3/4 | 3 | 13 | Complete views; initialization qualifications below |
| N10k A, extra deferred-view profiles 2/3 | 2 | 12 | Additional preparation controls; no current-view claim |
| N10k B profile 4 | 0 | 0 | Initialization censored before selection |
| N10k B profiles 2/3 | 0 | 0 | Unperformed after B profile-4 initialization censor |

There are 195 primary complete-view operation rows and 24 additional deferred
control rows. All primary rows passed immutable semantics, current readable
tree and discovery/extraction checks. Each complete cell covered single/batch
payload, label, single/batch title, path, relationships, high-degree adjacency,
high-degree title, split, merge, external edit and delete. The external B case
records an expected independent-verifier refusal, then restores the bytes
before the operation; engine audit alone is not claimed to detect B edits.

Each measurement is one observation with uncontrolled caches, in the fixed
case order. Prepare, select and A refresh are separate clocks. Independent
oracle, census and predecessor retirement/collection are outside those clocks.
These are disposable short histories, approximately H14; the restored profile
comparators include one extra untimed profile-normalization publication.
No N100k run, new H100k history, percentile/SLA, power-loss qualification or
physical-write measurement is claimed.

## Initialization censors and retained state

Fresh N10k A profile-4 initialization hit the 120.016-second child cap after
an authentic selection, during A delivery. Its 10,000-document immutable graph
passed the independent oracle; its readable view was incomplete. The original
censor remains in raw evidence. An actual quiescent operational capture holds
15,163 files / 199,158,570 logical bytes, with inventory SHA-256
`f95a8bbf53c48f04eff9ccad73863ca2ff8e1abcc8c615b9e799fe1575758a49`
and compressed transport SHA-256
`f8ab19602d449adcb3d390046be6f5ca16e4ec5fd5e535a9e1c0751e9afb3627`.
The authorized A continuation reused the original cell admission epoch, resolved
the accepted request and finished delivery, then completed all 13 operations.
It did not manufacture a selector or reset the original initialization clock.

The full N10k A profile-3 and profile-2 comparator cells used the actual recovery
restore API on this capture, then a verified untimed publication to normalize
their profile. Restore, initial delivery and profile normalization remain in
their raw events, outside ordinary-operation timings. Their total cell times
were 322.188 and 328.266 seconds. They are restore-backed comparators, not fresh
initialization successes. Earlier deferred-view controls remain separately
labelled and are not used in the complete-view comparison tables.

The profile-3 full comparator's successful disposable store was removed by the
old launcher before a later retention instruction arrived. Its complete raw
measurements, checks, cleanup receipt and authentic input capture survive.
It was not reconstructed or rerun to recreate the fixture. The profile-2 full
fixture remains: 20,910 files / 268,705,817 bytes at its completed census. Its
old cleanup controller was suspended while the child finished, then stopped
after verifying success and no active measurement child; the retention receipt
records this explicitly.

N10k B profile-4 initialization was seeded only with authenticated original
payloads from the initial A corpus, at the existing lock-acquired barrier.
The engine still performed its own creation/validation. It too reached the
120.016-second limit, with **no SELECTED**. Its unselected prefix is preserved
under `matrix-final/n10000-B-p4/cell/disposable/store`; it was not restarted.
The preparing reservation is 3,067,108,864 conservative bytes with `used=0`.
That allowance is distinct from physical storage, and `used=0` does not mean
zero bytes were written. The compressed payload seed is 2,829,820 bytes,
SHA-256 `8cf62485fd216adbe527aea0aab3f45264c91858e0422c11398cfdcce5fc3fee`.
No B10k ordinary-operation timing or comparator result is imputed.

The protected common-pin store, operational backup, H101006 restored fixture,
original recovery checkout and shared project checkout were not mutation
targets. Disposable successful cells were collected/cleaned only inside owned
roots; original failure prefixes and retained archives remain.

## Work and timing results

The full derived tables are in [`results/tables.md`](results/tables.md); every
operation's counters and timing are in
[`results/measurement-summary.json`](results/measurement-summary.json).

| N10k A operation | Profile 2 prepare | Profile 3 prepare | Profile 4 prepare |
| --- | ---: | ---: | ---: |
| Single payload | 2,847.41 ms | 63.40 ms | 39.04 ms |
| Single title | 2,800.65 ms | 2,776.49 ms | 220.77 ms |

The payload case removes the complete navigation scan in profiles 3/4:
zero generated navigation parts/bytes and zero D-range rows, versus one
1,938,936-byte monolithic part and 10,000 D rows in profile 2. Exact source
replacement and immutable candidate/replay work still occur.

For the N10k single-title case profile 4 inspected 322 headers, generated two
parts / 50,090 bytes, and changed one ordinary navigation path. Profile 3
returned 10,000 D rows and generated one 1,938,946-byte part. A refresh touched
one path on profile 4. The low-level profile-4 counters still examined 10,272
E leaf pairs over 963 E page visits, despite returning zero E rows. Zero
returned rows is not zero underlying index work.

The N1k single-title observation was 234.25 ms for profile 4 versus 225.77 ms
for profile 3. Profile 4 inspected 511 headers and generated 78,407 bytes;
profile 3 returned 1,000 D rows and generated 153,046 bytes. Fixed overhead and
underlying range geometry matter at small sizes. No universal speedup is
claimed from the single observations.

High-degree work remains visible. The N10k high-degree-title case read 5,132
outgoing rows, generated 23 parts / 478,809 bytes, examined 17,825 E leaf pairs,
and changed one navigation path. The batch-title case inspected 2,627 headers,
read 5,002 outgoing rows, generated 36 parts / 828,806 bytes and changed eight
navigation paths. Generated candidates can include byte-identical adjacency
parts; generation counts are not changed-file counts or physical writes.
The split case changed one group into two local children; the merge case
collapsed that local geometry. These cases did not perform a D-range scan.

B construction remains complete. For the N1k payload case the measured
generation portion was 3,209.21 ms (profile 2), 3,195.76 ms (3), and 4,537.43 ms
(4), separate from selection and its 80.03/87.05/91.04 ms generation verification.
Profile-4 fact dirtiness does not remove B's full-generation cost. N10k B has
no corresponding successful measurement.

## Resources, platform status and limits

Ordinary children retained their 120-second limit, the actual capsule retained
its 1,800-second budget. Independent measurement-oracle checks assert a
900-second elapsed bound on return; this is not a preemptive checker watchdog.
Point-reader children used a 256 MiB RSS limit including the 64 MiB cache;
maintenance children used a 2 GiB RSS limit and checker roles had a 2 GiB budget.
The measurement checker records its own RSS after each check rather than
continuously enforcing that budget. Locked offline builds used two jobs.
Available per-child RSS samples and violations are in original receipts;
matrix child sampled peak was 52,924,416 bytes and checker boundary-sampled peak was
202,440,704 bytes. Sampling does not prove a continuously measured aggregate
process peak. No limit or deadline was increased to turn a censor into success.

Owned byte/file allowances were prospectively reallocated by the coordinator
from 768 MiB to 1 GiB and finally 1.5 GiB / 150,000 files, within the separately
authorized 40 GiB global ceiling and 5 million files. Actual full comparators
used the stricter 1 GiB / 100,000-file guard; the amended future launcher does
not relabel their original receipts. A prospective admission refusal was
retained before the later geometry/cap amendment. Historical aggregate disk
peak was not continuously sampled. There are 63 prepared-owned samples; their
largest combined runtime/worktree sample is 903,835,852 bytes / 78,266 files.
The final current census and available space are in
[`results/resource-closeout.json`](results/resource-closeout.json). These count
regular file lengths, exclude reparse points, and do not claim physical
allocation or write amplification. The 140 GiB Windows free-space reserve
remained available at closeout.

The coordinator reports a native Linux full-suite pass on final core 4bde:
39 focused cases / 153 calls, 24 recovery cases / 282 calls, and eight genesis
checks, with source-before/after verification. Its binary SHA-256 is
`da75d34e5fe162447e356aead2fba0f2a1ac21bc1055ebb3812bce80fda79846`;
the Rust source tree is `d22e997c7c75940a6b4077368549d28c1780ef49`.
That separate lane's raw archive SHA-256 is
`07ec4c40e36489424f23b05d58732c0baa511f2e28d4fa578254846b6e5e7548`,
in the coordinator's integration evidence. This report does not repackage it
as Windows evidence. Native Mac qualification was incomplete at the last
coordinator update (first full suite censored at its 95-second bound; a bounded
remainder was under review). The coordinator owns any later native receipts
and the P gate. This worker does not declare that gate passed.

GitNexus impact and pre-commit detect-changes were attempted against this exact
worktree. Its repository is unregistered, and the shared graph predates this
experiment. Blast radius is therefore **UNKNOWN**, not clean. Retained errors
and manual call tracing cover graph deltas, projection preparation/replay,
initialization, packet writing and A/B delivery. Later harness-only changes
affect fixture admission/continuation, independent verification and reporting;
they do not change the frozen Rust engine. A pre-commit textual diff and exact
16-file core comparison constrain the final evidence commit.

## Evidence and cleanup exceptions

[`results/raw-receipts.tar.gz`](results/raw-receipts.tar.gz) contains bounded
original events, measurements, summaries, compiler/build logs, admission and
cleanup/refusal receipts, ordinary extraction proof, and the operational
capture manifest/inventory. Its authenticated per-file index and archive hash
are in [`results/raw-receipts-index.json`](results/raw-receipts-index.json).
Large local store trees, binaries and authenticated transport/seed archives are
not embedded. Transport/seed paths, lengths and hashes are bound by the index;
binaries are bound by source manifests, and retained store paths/censuses are
recorded in receipts. Superseded prototype receipts remain locally retained and
are not reported as final qualification. Original failed tooling outputs,
including the repaired missing FileSpec field and command/setup failures,
are retained rather than presented as successful checks.

Automatic approval review rejected two authorized cleanups with “blocked by
policy”: the unintended new GitNexus/pnpm staging subtree (1,050,883,003 bytes /
15,042 files, separately charged in the coordinator's global baseline), and a
superseded focused-fixture directory after its verified archive was made.
Neither rejection was bypassed or retried through another deletion mechanism.
Both residues, and the prototype archive, remain counted. There is no pending
request for the user to approve those same blocked operations again.
