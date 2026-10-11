# Remaining work-registration invocation costs

Status: **completed investigation; recommended direction proposed for human review**.
Source checkpoint: `83fc8b6ac005a02ee4a1f5aa607dca2b654c47b0`,
`feat/handbook-contract-membrane-clean`, 2026-10-11.

## Finding in ordinary English

Registration is still expensive because it repeatedly obtains trustworthy history
through a filesystem API that repeatedly checks the paths around each file. A
warm history-31 invocation takes about seven seconds. Selecting the grant once
that history is available takes about **0.016 milliseconds**, not seconds.
Removing another permission comparison will not address the main cost.

The landed refactor removed eager preparation and repeated pure reconstruction
within the held invocation. It did not make history physically immutable. Work
and native locks serialize supported writers; they do not rule out in-place
filesystem edits, added siblings or all physical substitutions. Consequently,
each current observation still reads bytes, checks inventory and revalidates
physical identity. These facts are necessary under the selected policy; the
present algorithm for obtaining them is not a permanent semantic requirement.

**Recommended direction:** separately authorize bounded preparation of a
file-native invocation observation/choreography change. Its first question should
be how to avoid the normal fresh path's independent I1 reconstruction while
preserving full original-history classification, native-before-work locking,
replay availability, fresh observation and crash/reply semantics. Reject a blanket
native-first rewrite as an already-equivalent optimization: it introduces native
lock waits into committed-original replay. An opportunistic native acquisition
with immediate historical fallback on contention is worth feasibility assessment;
it is not an existing capability or a selected implementation. Keep the existing
history-first resolve/recovery paths as the reference contract.

In parallel within that *future preparation*, evaluate bounded reuse of pure
owner/slot expectations and borrowed parsed artifacts, and the extra reservation
observation. Those are smaller opportunities. Do not promise subsecond operation
from them. If the desired budget requires avoiding most whole-history traversal,
the human must separately choose a dependency-observation versus complete-audit
boundary; the present policy cannot be silently replaced with selected-path reads.
Practical capacity and revocation at saturation remain separate unresolved work.

## Authority, access and evidence boundary

The runtime supplied **`sandbox_mode: danger-full-access`**, unrestricted
filesystem/network access, and **`approval_policy: never`**. No access setting was
inferred from the assignment or changed. There was no permission-blocked operation.
This task authorizes investigation, reviewed research documentation and a local
commit; it authorizes no product changes or detailed successor packet plan.

The [HCM entry point](../specs/handbook-contract-membrane/README.md),
[lifecycle](../specs/handbook-contract-membrane/work-unit-lifecycle.md),
[review policy](../specs/handbook-contract-membrane/review-and-closeout-policy.md)
and [completed refactor authority](../specs/handbook-contract-membrane/slices/work-registration-phase-refactor/README.md)
bound this work. Supplied meta evidence establishes its aggregate closeout and
human recap complete, followed by human authorization of this investigation.
The narrow status corrections record that supplied result, not a new closeout or
product verification. The [format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
remains **proposed**, Contracts JSON remains fixed, and existing versioned bytes,
canonical preimages, bounds and receipts control. No migration is selected.

Primary measurements are the [current evidence README](work-registration-phase-refactor-2026-10-11/README.md),
[verification](work-registration-phase-refactor-2026-10-11/verification.md),
[summary](work-registration-phase-refactor-2026-10-11/summary.json),
[owner spans](work-registration-phase-refactor-2026-10-11/profile-owner-spans.json),
[observations](work-registration-phase-refactor-2026-10-11/profile-observations.json),
[held-history table](work-registration-phase-refactor-2026-10-11/profile-held-history-tables.md),
[lock lifetimes](work-registration-phase-refactor-2026-10-11/profile-lock-held.json)
and [cache gauges](work-registration-phase-refactor-2026-10-11/profile-cache-gauges.json).
The [duplication audit](work-registration-duplication-audit-2026-10-11.md) and
[native optimization investigation](work-state-native-optimization-investigation-2026-10-10.md)
are bounded explanatory inputs. Their older timings are not the current baseline.

GitNexus CLI `query`/`context` used the exact repository path
`C:/Users/spmcc/Documents/__Project_Code/handbook`, because five indexes share
`handbook`. Its index is at `a82f3b2`, two commits behind the inspected checkpoint.
Graph leads were checked against live source; old graph line numbers/call edges
are not claimed as current proof. No production function, class or method was
edited and no index refresh was needed for this documentation-only change.
Precommit change detection classified five documentation files and eleven
documentation sections, zero affected execution processes, with low risk. The
source diff confirms no product change; some section labels span adjacent prose.

## Concrete workflow and observation map

Source aliases below refer to [transaction](../../crates/engine/src/work_state/transaction.rs),
[store](../../crates/engine/src/work_state/store.rs),
[native work authority](../../crates/engine/src/local_authority/work_registration.rs),
[local check](../../crates/engine/src/local_authority/local_check.rs),
[local authority](../../crates/engine/src/local_authority.rs),
[local Charter](../../crates/engine/src/local_charter.rs),
[durability](../../crates/engine/src/repository_durability.rs) and
[SDK work state](../../crates/sdk/src/work_state.rs). Line references are for the
checkpoint above, not the instrumented variants.

A caller binds supplied explicit meaning, or reads one exact committed kind,
saves the complete original locator, then invokes. Invocation first asks whether
the original already has a retained outcome. For a fresh or resumable request it
then holds native authority, obtains the work lock, observes history again,
captures original authority and reserves a positive slot. It publishes a candidate
with a never-attempted checkpoint, constructs decision/snapshot/terminal-pending,
reobserves history, persists the publication-attempt checkpoint, validates guards,
installs the terminal, and proves the result before replying. Each observation
has a distinct consequence; none authorizes a cached absence.

| Observation / source | Inputs and facts established | Guards, possible intervening changes, omission consequence |
| --- | --- | --- |
| Explicit/kind binding; SDK 135–142, transaction 1121–1150 | Supplied exact grant/basis/actor/composition/scope; complete portable locator. Explicit: zero scans/live checks. Kind: one full scan proves exact committed definition/approval/receipt/terminal closure. | No guard survives caller delay. Preparation promises no live grant, absence, capacity or effect. Kind K1 cannot be inferred from a current K2 or parsed candidate. |
| Owner existence and I1; transaction 1390–1451 → `resolve` 1348–1370 | Strict positive owner observation; complete history, name/content conflict, exact committed/noncommit/attempted/pending/absence classification. Exact retained outcome takes priority over current permission. | Retained root + work lock, **no native current authority**. Scan tree/metadata guards and work lock all drop. Native/work administration or direct filesystem changes may occur before I2. Omission without a complete fallback can turn replay after revoke/reset/D→G into refusal or uncertainty into false absence. |
| Native admission; native 124–146 | Original initial/check/current D/G/Charter closure, native physical identity, exact current outcome fingerprint. | Existing initialization and native authority locks, then authority-tree guard. These precede acquisition of the work lock. Supported administration serializes here; external edits still need revalidation. Item has no bootstrap callback or native setup effect. |
| I2; transaction 1458–1514 | Complete current owner/inventory/manifest/history; exact request/item reservation, retained lineage, current grant and exact kind dependency. A race that committed the original during the I1 gap is seen here. | Native guards + new work lock. Per-scan tree/metadata guards drop when scan returns. Holding the work lock excludes supported work mutations, not direct bytes/namespace changes. Reusing I1's facts would miss gap changes. |
| Work checkpoint; transaction 1465–1488 | Exact physical/content checkpoint for every observed other transaction record plus owner/anchor/bootstrap. | Retained through effect and final body checks. Excludes own changing transaction and manifest slots. Captures positive paths, not all future names or absent files. Omission loses retained dependency substitution checks; it is not by itself an unchanged-history certificate. |
| Proof capture; native 57–117, transaction 1549–1564 | Bounded complete native authority trees, exact anchors, historical Charter dependencies and contained proof; derive/bound all original records before reservation. | Current checks before/after capture; native guards remain. Authority bytes can change during capture despite logical locks. Omission destroys original historical provenance; earlier proof reuse needs fresh equality/closure observation, not today's grant substituted for original authority. |
| Reservation inventory + direct manifest; store 438–534 | Extra full inventory plus all 32 required slots, displaced empty predecessors and prepared-slot absence; exact positive slot name/intent, complete peak byte budget. | Work/native locks held. No callback or owner mutation between the two reads, but direct filesystem mutation remains possible. These reads currently repeat the manifest. Cached empty slots cannot prove current absence. |
| Acceptance/decision/snapshot; transaction 1565–1612 | Slot replacement followed by candidate directory/intent, never-attempted checkpoint, no-replace candidate rename, exact decision/snapshot/terminal-pending. | Guarded durable operations validate their source/destination paths. Own namespace and slot bytes deliberately change. Crash after slot consumption but before complete candidate is retained uncertainty, not an unused request/slot. |
| I3; transaction 1613–1639 | Fresh full history after snapshot; reservation, current grant/kind closure and prefix validity; capture exact manifest-path checkpoint. | Native/work locks remain; new per-scan guards. Own transaction changed since I2, and direct damage/new siblings may appear. Omitting I3 without a replacement observation can publish using damaged dependency, reservation or prefix facts. |
| Attempt and preterminal validation; transaction 1640–1658 | Persist attempted checkpoint **before** terminal installation; revalidate native, work, other work files and manifest. | Guards/checkpoints remain; own attempt rename changes physical namespace after I3. Failure after attempted is uncertain even without terminal. Absence must not synthesize never-attempted/noncommit proof. |
| I4 and final body; transaction 1660–1675 | Fresh full scan proves exact installed terminal and original output; final native/work/work-file/manifest validation before Reply. | Same held guards, own terminal now changed. Final failure may hide a real commit from the reply; terminal recovery remains authoritative. A successful rename alone does not prove the returned closure. |
| Wrapper return; native 145, after transaction Reply | Final current native closure check after the consuming body. | Native guards still held; **work lock/work checkpoints have dropped**. Reply-boundary mutation can occur. Omitting this check loses tested detection. This does not prove every work file stayed unchanged after body release. |
| Historical recovery/noncommit; transaction 1713–1763 | Fresh full historical scan can replay committed original, retain attempted/staging uncertainty, or positively close a never-attempted original; rescans verify noncommit. | Separate work-lock scope, independent of usable current authority. No new acceptance on recovery of Unaccepted. Never borrow replacement grant/scope or treat missing proof as noncommit. |

All four scans use `scan_with_artifacts` (transaction 425–499): retain history-tree
and exact metadata guards, obtain inventory, compare exact expected record bytes,
classify prefixes, verify grant/kind dependencies and closed authority lineage,
then validate tree/metadata/work lock. These guards are local to each scan.
`Inventory` returns byte buffers, not a retained filesystem snapshot.

### Every native current check

`validate_current` (native 41–52) validates the authority tree, OperatingGuard,
and reobtains `local_check::work_registration_current`, comparing the exact
outcome fingerprint. That callee rechecks original initial/Charter/check history,
current and revoke heads, supported D/G relation and governance inventory. A
tiny scope comparison is not a replacement for this work.

| Call for successful fresh Item | Purpose and next possible change |
| --- | --- |
| 1. Native wrapper entry, 143 | Revalidate the outcome used to mint the held admission, after tree retention; I2 may then take substantial time. |
| 2. Capture entry, 60 | Fresh native basis after I2/work checkpoint and BeforeIntent callback. |
| 3. Capture exit, 116 | Ensure capture did not span an authority change; original proof validation itself is pure over contained bytes. |
| 4. Before reserve, transaction 1565 | Recheck after original encoding/derivation/bounds and before the first effect. |
| 5. After Snapshot, 1613 | Recheck after acceptance and decision/snapshot writes, before I3. |
| 6. After PublicationAttempt, 1649 | Recheck after attempt rename/callback, before terminal installation. |
| 7. After I4, 1670 | Recheck after proving terminal, before body Reply. |
| 8. Wrapper return, native 145 | Recheck after Reply callback/body release. |

The initial `work_registration_current` call at native 131 is **additional** to
these eight calls. The wrapper's initial outcome is used to bind admission, then
entry validation repeats the closure. This is a real adjacent duplication
candidate, but removing the second observation shifts the before/after-tree
capture window. It needs an explicit combined observation argument. Eight calls
are not eight independent permission-comparison research problems.

## Lower-level multiplication and duplication classification

`WorkLock::inventory` (store 197–342) validates owner/root/lock, lists owner
parents, reads a full manifest, lists each transaction and reads each leaf. Its
`manifest` (348–431) validates again, reads every slot and optional prepared/
displaced file, decodes and checks canonical slot bytes, then validates again.
`WorkLock::validate` (178–195) rereads owner, anchor and bootstrap and regenerates
their expected encodings. Reservation calls inventory **and then manifest again**.

Each required/optional leaf read passes through retained-root safe path handling.
On Windows the parent-pinning/listing path checks ancestor directories, sibling
kinds and physical file metadata, rather than just opening the selected leaf.
The same `.handbook`, work-state parent and `v1` directory therefore reappear
many times. Native tree/closure validation and work checkpoints add other
overlapping physical traversals. This is source-backed algorithm multiplication,
not evidence that disk throughput is exhausted.

Two less obvious repetitions are in native callees. Retained-tree validation
fingerprints non-lock files through both the retained and reopened current handle
(durability 243–288); those reads are outside the returned-leaf byte gauges.
Current Charter resolution reconstructs material through historical resolution
and again for current-byte comparison (local Charter 655–696). Material derivation
469–547 reopens the exact root through public preparation, reevaluates supplied
Charter meaning and constructs expected canonical records. Its pure expectations
could be reused, while its live path/registration/current-byte checks remain.
The retained native authority tree (durability 756–781) does **not** cover all
negative dependencies: the verified-Charter footprint in
`.handbook/state/transactions/{promotions,posture-transitions}` is observed
separately (local Charter 911–926). Tree validation alone cannot replace the full
current check without covering that negative evidence.

| Repetition | Classification and admissible reuse boundary |
| --- | --- |
| Same exact contained Intent → decode, historical proof validation, output, canonical expected records | Deterministic. Held I2–I4 reuse is already landed. I1 and I2 still independently reconstruct 31 rows at history 31. Carrying **pure** artifacts across the gap could save that reconstruction if current intent bytes and owner/name/project context are reread/rebound; no prior status, absence or authority may cross with them. Keep only current bounded entries. |
| Owner/bootstrap encodings and exact slot/empty predecessor expectations | Deterministic over exact validated owner/index/slot bytes. Compute once in a bounded scope while retaining every required current physical read/comparison. A mutable observed slot needs a new exact input key. The prior isolated owner-only result had no clear full-store gain. |
| `Parsed` intent/output clones (transaction 413–419), row vectors and old inventory buffers | Representation/copy work. Internal borrowed/Rc views might avoid large clones without changing bytes/status checks. Do not cache stale flags or retain two complete physical inventories merely to save allocation. Current profiles do not time clone/allocator cost separately. |
| History relationships/current grant/kind comparisons | Deterministic over the **complete freshly observed row set**, including status. Exact-row-set memoization is logically possible, but copying/keying the set may exceed savings at 32 rows. No authority/current grant may be reused just because logical locks are held. |
| Ancestor/sibling enumeration and directory/leaf identity checks | Fresh physical observations. Parent-handle reuse with per-use revalidation can save opens without omitting listings. Grouping enumerations into a stage-level observation changes the read-stage window unless all excluded mutations/refusals are proved equivalent. Retained directory identity is not immutable file content or a certificate of absent future siblings. |
| I2 vs reserve inventory; reserve inventory's manifest vs direct manifest | Fresh observations plus repeated pure decode/expectation work. A fused result could return manifest and inventory under a named retained observation boundary, but it must cover negative prepared-slot/candidate evidence and retain atomic exact replacement + postpublication rereads. Returning old Vec bytes alone is insufficient. |
| I3/I4 full rereads vs work/manifest checkpoints | Different coverage. Checkpoints pin observed positive files; full inventories detect unknown names, missing originals and conflicting prefixes. Retaining a checkpoint does not establish complete negative evidence or authorize replacing scans with it. Own staging/slot/attempt/terminal changes require explicit refresh, not a generic "unchanged" flag. |
| Native original closure/current heads across eight checks | Some expectation/contained-proof work is pure; heads, original retained physical bytes and tree versions remain fresh. Holding native locks excludes supported administrators, not every physical alteration. Capture brackets and final reply check have concrete independent purposes. |

The [earlier direct-read negative probe](work-state-native-optimization-2026-10-10/probes.jsonl)
already demonstrates the boundary: guarded ancestor reading refused an unrelated
hard-linked sibling while selected direct reading succeeded. Later complete scans
do not restore that removed read-stage refusal. Grouped/direct traversal is a
proposal about detection timing/coverage until proved otherwise.

## Current cost budget

These are **current refactor** history-31 observations. Pristine workflow medians
are 7,075.81 ms explicit / 8,895.76 ms kind; invocation medians are 7,026.99 /
7,291.56 ms. Kind still pays its preparation scan. Those six-pair medians are
not interchangeable with the following one-per-cell instrumented invocation.

| Inclusive instrumented span (ms) | Explicit | Kind |
| --- | ---: | ---: |
| Invocation (`execute` span) | 6,579.61 | 7,023.22 |
| I1 | 1,256.26 | 1,431.29 |
| I2 | 1,228.47 | 1,409.21 |
| I3 | 1,000.58 | 1,031.61 |
| I4 | 977.53 | 980.05 |
| Four scans total | 4,462.85 | 4,852.16 |
| Outside all scan spans, by subtraction | 2,116.76 | 2,171.06 |
| Extra reservation inventory, within outside-scan work | 730.92 | 744.77 |
| Two outside-scan manifest spans, nested in that work | 160.47 | 167.45 |
| Eight `validate_current` calls | 740.01 | 755.71 |
| Proof capture (includes two of those checks) | 275.19 | 278.79 |
| Two `current_grant` comparisons | 0.0153 | 0.0172 |

Scans, current checks, proof capture, inventories and filesystem spans overlap.
Do **not** add them as separate removable savings. The recorder's
[thread-local nested span accounting](work-registration-phase-refactor-2026-10-11/metrics.rs)
subtracts child time: summed invocation-exclusive spans reconcile to
6,579.61 / 7,023.22 ms. The following buckets are disjoint attribution, still not
guaranteed removable cost:

| Exclusive wall bucket (ms) | Explicit | Kind | Interpretation |
| --- | ---: | ---: | --- |
| Directory listing | 3,123.45 | 3,176.23 | 5,385 listing calls in each whole invocation; includes sibling entry work outside nested instrumented helpers. |
| Directory guard opening | 862.48 | 866.22 | 64,420 calls, repeated traversal/identity work. |
| Handle information | 801.42 | 809.38 | 223,573 calls. |
| Those three buckets total | 4,787.35 | 4,851.82 | 72.76% / 69.08% of instrumented invocation wall. |
| Canonical encoding | 490.31 | 713.28 | Already reduced; includes slot/owner/native work, not solely history-output construction. |
| Historical contained-proof validation | 62.03 | 27.61 | Pure expected-input work; nested canonical time excluded here. |

Existing path observations attribute 1,866 listings to `.handbook`, 1,417 to
`.handbook/work-state`, and 799 to `v1`. The `v1` listings alone return 76,563
entries, 76.93% of the 99,530 whole-invocation entries. I1/I2 each list `v1` 157
times; I3/I4 each 162. At histories 8/16/31, whole-invocation `v1` listings grow
224/424/799 and their returned entries grow 11,174/27,918/76,563. Each leaf's
ancestor observation therefore repeats a larger sibling set, beyond simply doing
four scans.
Within held I2–I4 alone, both forms still have **2,245 listings / 55,950 entries**
and **720 successful bounded leaf reads**, returning 23.48 MiB explicit / 27.93
MiB kind. Counts describe returned logical bytes, not physical disk I/O.
The landed held reconstruction reduction is 95→31 outputs, 64 hits; canonical
encodes are 3,355→1,162 explicit / 6,963→1,758 kind. Further reuse cannot recapture
those savings a second time.

An optimistic removal of *all* I1 time, with no replacement/fallback/lock cost,
would leave **5.32 / 5.59 seconds** in these profiled invocations (19.09% / 20.38%
reduction). This is an opportunity envelope, not measured successor performance.
Sharing I1/I2 pure derivations instead of removing I1 has a much smaller envelope:
I2 `Artifacts::expected` inclusive time is only 269.41 / 449.92 ms, already
including its canonical/proof children. Completely removing the extra reservation
inventory is at most another roughly 0.74 seconds in this sample, subject to a
replacement observation and overlapping work. Full elimination is not justified.
Remaining repeated physical work makes a subsecond promise implausible under
unchanged full-history/read-stage policy without a substantially different way
to establish equivalent observations.

Matched enabled/disabled history-31 refactor attribution controls were
6,579.63 versus 6,488.32 ms explicit (+91.31 ms) and 7,023.25 versus 6,814.59 ms
kind (+208.65 ms). Each is one matched pair; do not subtract a universal overhead
correction or claim these represent latency distributions. The old 7,494.87 ms
single profile and histories-25–30 mixed-size trials are not this trial baseline.
Measured permits spans of roughly 2.2–2.3 ms are only those comparisons, not all
validation. All 36 accepted pristine paired workflows improved; the six cells'
median paired reductions were 33.51–39.80%.
This investigation reuses that evidence rather than repeating unchanged suites.

### Memory and contention are part of the budget

History-31 private-peak medians increased approximately **13.3 MiB explicit / 21.5 MiB kind**
to 104.14 / 135.29 MiB after the landed cache. Raw artifact buffers are only
7.60 / 9.10 MiB: typed Intent/Output, `Parsed` clones, allocator bookkeeping,
temporary buffers and stacks are excluded. A cross-I1 cache or retained observation
must avoid overlapping duplicate inventories/outputs and report peak bytes, lifetime
and post-call residency. More pinned file handles can also prevent needed candidate
or terminal renames; mutable transaction parents cannot be retained indiscriminately.

The [resource evidence](work-registration-phase-refactor-2026-10-11/resource-summary.json)
shows 96 repeated calls returning handles/locks, but allocator residency persists.
It is not a long-run leak proof. The 128 MiB constant bounds store inventory,
not process memory. Lock spans overlap: native guards cover most held invocation;
work lock covers I2–I4 and writes. At history 31, initialization-lock hold time
is 5,298.68 / 5,557.63 ms and operating-lock hold time 5,284.33 / 5,545.64 ms;
the aggregate of two work-lock acquisitions is 6,240.56 / 6,685.39 ms. These
lifetimes overlap; uncontended wait is only 0.0314 / 0.0275 ms in this sample.
I1 saving may reduce total latency without
reducing the duration blocking native administrators. Moving I1 inside native
guards would instead increase that contention.

The current-read race measured child work-lock wait 3,404.98 / 3,528.27 ms in
baseline/refactor, then held time 943.95 / 1,023.39 ms. Its injected 200 ms callback
delay is not the full parent guard lifetime. It establishes actual queuing, not an
administrative latency distribution. No fair scheduling, timeout or revocation
availability at saturation is inferred.

## Feasible alternatives and their precise departures

| Alternative | Trust/mutation assumptions and failure behavior | Assessment |
| --- | --- | --- |
| Retain I1–I4; share pure I1/I2 artifacts, owner/slot encodings and borrowed rows; reuse safe parent handles per use | Exact fresh physical bytes, namespace checks, status and complete relationship set still observed; complete exact input keys/context rebound. Drop/prune buffers on every exit. No cached absence/currentness. Per-use physical revalidation/listings remain. | Lowest contract risk, modest ceilings and possible memory cost. Worth assessing as a fallback; older parent/owner experiments do not predict current matched gains. |
| Native-first fresh Item, one held history observation replacing I1+I2, historical fallback | Current seam is effect-free before callback, uses existing native locks, preserves native-before-work. Held classifier must reproduce `resolve`'s complete conflict/prefix/outcome priority before grant/reservation. On **any failure with no new mutation attempted**, release all guards then perform fresh full historical resolution, including a final native validation failure after finding an already committed original. Failed recovery of Pending keeps existing noncommit/uncertainty rules. Once a new mutation has been attempted, retain today's uncertain registration reply on final-validation/Reply failure and separate exact terminal recovery; do not convert that lost reply into unconditional success. | Removes normal I1 work in principle. **Changes replay contention/observation order**: native acquisition uses blocking locks, so a held administrator can delay replay before fallback is reachable. Earlier native acquisition can also change which racing registration/authority transition wins. Not an equivalent blanket change. Historical-only resolve/recover should keep their existing route. Returned original disposition/bytes and error priority still require proof, including same-name conflict and post-I1 commit races. |
| Opportunistic native-first fresh path, contention/current-admission fallback to full I1; then normal held reobservation if fresh/resumable | Needs a proven nonblocking acquisition route for the entire native lock set, releasing partial guards before standalone work history. Busy native locks must permit immediate historical replay; an Unaccepted/Pending fresh request may subsequently use today's blocking native-before-work route and rescan. Never hold work while acquiring native. All direct-FS observations and final-error meanings remain. | Preferred choreography feasibility question if preserving existing replay availability. Current seam does **not** offer this route; no measured gain or API design is claimed. Failure/contended path may retain both scans, which is acceptable. Complexity must earn its normal-path saving. |
| Retain full stages but fuse reservation inventory/manifest or validate history via retained observations | Supported writers serialized; exact positive dependencies **and negative namespace/slot facts** require coverage at use. Preserve source/destination exact replacement and postpublication verification. Account for own writes with explicit observation refresh. | Meaningful possible saving, but current checkpoint APIs/unguarded Inventory are insufficient as an automatic proof. Decode reuse is safer than raw-observation reuse. |
| Group repeated sibling/ancestor checks once per semantic stage; reread leaves/check versions before use | Requires explicit chosen observation interval, retained directory versions and fresh content checks, including absent/unsafe siblings and additions during traversal. Native, owner and mutable transaction trees have different mutation sets. | Can target the dominant listing cost. **Changes read-stage detection timing unless equivalence is demonstrated**; Windows/Unix identity/version behavior needs separate proof. Do not call a later full scan equivalent to every current guarded read. |
| Dependency-scoped ordinary operations plus separate complete audit, using file-native decision-bearing control/index | Must positively bind current lineage/selected kind/original and reservation/negative evidence; crash-safe publication and audit coverage/trust selected separately. Local hashes cannot prove external freshness or coherent administrator restore. | Largest structural opportunity, also a **different complete-audit/detection boundary**. Belongs with separate capacity design selections, not an invocation cleanup. No backend, history purge, capacity bump or index implementation is authorized. SQLite is neither prerequisite nor default. |

These alternatives are not additive. An omitted scan already removes some
encoding/listing/checkpoint work; a smaller observed row set changes a cache's
benefit and memory. A new snapshot/index must not claim both old complete
next-operation detection and selective reads without accounting for unobserved
damage. A file-native design is a reasonable preference, not a performance proof.

## Guarantees that constrain any next selection

The [foundation prefix contract](../specs/handbook-contract-membrane/slices/work-item-state-foundation/packet-1.md#durable-prefix-and-authority-transition-contract),
[authority lifecycle](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-1.md),
[kind contract](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-2.md)
and current source/tests remain controlling:

- Committed-original replay survives revoked work/native permission, reset and
  D→G without restoring permission. Fresh effects refuse unusable original
  authority; successor scopes are not unioned or borrowed. K1 remains exact after K2.
- Complete name/content conflicts, positive slots and negative namespace evidence
  distinguish absent from missing/partial/reserved. Noncommit releases only its
  item/head reservation, never original request identity or lifetime slot.
- Candidate, staged intent, accepted never-attempted, attempted, terminal and
  noncommit prefixes retain their meanings across process death. Attempted/staging
  uncertainty cannot become absence or permission to replace an original.
- Native locks precede work locks. Guards last through their established consuming
  boundaries. Final-validation/Reply failure may accompany a committed terminal;
  recovery replays exact original receipt bytes, with no new effect.
- Root/owner/anchor/lock substitution, unsafe siblings and unknown inventory entries
  retain their selected observation/refusal boundary unless explicitly changed.
  Held physical identity is not content immutability. No broader restore-trust or
  external freshness witness is selected.
- Original receipt framing, V1/V2/V3 representations and identity domains, exact
  authority/provenance, 1 MiB records, 32 retained transactions and 128 MiB inventory
  remain. Fresh revocation at saturation still fails; no optimized read fixes it.

In particular, [physical seam tests](../../crates/sdk/tests/work_state_integrity.rs)
251–375 mutate BeforeIntent, Snapshot, PublicationAttempt, Terminal and Reply.
The Reply Charter mutation requires the wrapper's last validation. Tests allow
either mutation denial by a retained handle or detected refusal, and repair only
test damage to demonstrate that an installed terminal still replays. The
[durability tests](../../crates/sdk/tests/work_state_durability.rs) cover every
retained prefix across successor/revocation/reset, serialization and real process
death. [Kind integrity](../../crates/sdk/tests/work_kind_integrity.rs) checks exact
dependencies and mutations. These are references to existing tests and retained
80-pass evidence, not new passes for unimplemented alternatives.

## Experiments, verification and next boundary

No new disposable product prototype was needed to settle the recommendation.
Existing observation paths and exclusive spans establish the physical multiplication;
source establishes the blocking-native replay counterexample and final-validation
boundary. Another performance run cannot decide whether changing those guarantees
is acceptable. No new instrumentation, production symbol edits, whole-suite reruns
or benchmark matrix were performed. No experimental variant was landed.

Once successor preparation is authorized, a consequential remaining feasibility
question is whether preserving replay availability admits a simple opportunistic
native route, and whether a retained/fused observation covers negative facts
without changing detection. A disposable test is justified only if source cannot
answer that selected question. It should compare current `83fc8b6` with the single
isolated candidate on independently populated fresh native roots at fixed history
31, explicit and kind; use matched inputs/closure bytes, alternate order, retain
raw timing and memory, and pair instrumented/disabled controls rather than
subtract old overhead. Native proofs must be captured independently. Exercise
blocked native administration with committed replay, direct in-place/substitution/
unsafe-sibling changes at the affected boundary, pending/attempted/terminal
originals, reset/revoke/D→G and final lost reply. Add other histories/dimensions
only if the candidate or uncertainty warrants them. This is an experiment criterion,
not a packet plan or authorization to edit. Symbol impact analysis and isolation
are required before any future experimental symbol edits.

Verification for this documentation change: checked branch/HEAD, current source
and tests, existing profile nesting/raw path counts, arithmetic and local report
links; reused valid paired/80-pass evidence with its limits. Two historical
physical-root fixtures remain unavailable. Warm Windows/MSVC results establish
neither cold-cache/p95, Unix behavior, hardware power-loss, large-population
performance nor long-run leak absence. Independent causal review returned
**APPROVE**, with no unresolved P1/P2, after clarifying zero-mutation historical
fallback versus newly attempted-effect reply uncertainty; numerical precision
advisories were also resolved. Precommit change detection reports only the
expected documentation scope, zero affected execution processes and low risk.
No push is authorized.

**Recommended NEXT planning boundary:** human review of this research, then one
bounded invocation observation/choreography preparation assignment if desired.
Select whether replay's current native-lock independence is mandatory (recommended:
preserve it), and whether the current per-read/per-stage physical detection
boundary is retained (recommended: retain it for that increment). Choose a
measurable latency/memory/administrative-wait envelope; none is an existing SLA.
Preparation should resolve feasibility and only then propose an implementation
increment for separate approval. If the required budget cannot be reached under
those preserved boundaries, return the explicit detection/audit selection to the
human instead of quietly relaxing checks. Broader capacity, retained-store
treatment and protected administration still need their own selections. No
detailed successor plan or product guarantee change is selected by this report.
