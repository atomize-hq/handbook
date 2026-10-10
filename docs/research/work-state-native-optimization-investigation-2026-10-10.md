# Native work-state optimization and the capacity proposal

2026-10-10; baseline `8013948ebd3fbef6725f5d72f3d29120f7617009`,
`feat/handbook-contract-membrane-clean`. **Research and proposal only.** The
human authorized isolated native optimization experiments and this reviewed
proposal; production optimization, a backend, a capacity increase and a successor
are not authorized. Experimental Rust edits remain outside the primary checkout.
The [evidence and reproduction recipe](work-state-native-optimization-2026-10-10/README.md)
retain the actual patch, harness, raw observations, controls and verification.

## Result and experimental boundary

The combined unchanged-policy candidate reduces full-store current-read medians
by about **5%**, and matched preparation-plus-registration workflow median from
**9.30 s to 8.18 s (12%)**. Native invocation alone falls from **6.53 s to 5.57 s
(15%)**. This reduces constants; it does not deliver project capacity. The same
selected item's combined read grows from **347 ms at 8 transactions to 1,268 ms
at 32**, because unrelated retained history is still observed. Owner encoding
alone shows no clear full-store read gain; semantic reuse helps repeated scans
inside invocation, not the one-scan current read.

The separately labelled direct-read departure reaches **4.60 s** paired workflow
and **656 ms** explicit full-store current read, but removes an existing
observation and is not approved as equivalent behavior. No production cap or
backend is selected. Technical evidence does not establish an equivalent-native
backend winner; the proposal conditionally recommends a native indexed-file
candidate after human review of detection, budgets, publication and administration.

The first combined control preserves storage bytes/preimages, exact receipts,
complete-inventory detection, all four fresh-invocation scans, original replay
before live permission, and before/after-publication currentness. It combines:

- **Owner encoding reuse:** cache the immutable decoded owner's canonical owner
  and bootstrap encodings for one work-lock lifetime; still reread and compare
  physical owner/anchor/bootstrap on every validation.
- **Held-parent reuse on Windows:** retain no-delete directory handles for only
  `.handbook`, `.handbook/work-state`, and its `v1` parent; recheck directory/
  reparse/delete-pending identity on each use and clone the held handles. Keep
  request/staging parents transient so original publication renames still work.
  Every inventory listing and leaf-content observation remains fresh.
- **Pure semantic reuse:** one invocation-local cache keyed by complete exact
  retained intent bytes reuses decoding, historical contained-proof validation,
  output and expected-record construction. Each pass rereads physical files and
  checks their current bytes/prefix, owner/project/name bindings and whole-history
  relationships. No current permission, physical currentness or negative proof is
  cached. The initial original-history resolution has a different lock lifetime
  and remains uncached.

Windows sharing restrictions persist for the handle lifetime; omitting delete
sharing prevents delete/rename access. This supports retained path bindings,
**not content immutability**. Fresh leaf bytes, added hardlinks, directory
substitution and release on drop are checked by the experiment. The root retains
its `Send + Sync` properties. See [CreateFileW sharing](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew),
[DuplicateHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)
and [File::try_clone](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone).
Unix reuse was not implemented or measured.

The separate `direct` experiment routes required reads directly to retained
bounded reads, bypassing repeated ancestor inventories. It is **not an
unchanged-policy optimization**: the [native negative probe](work-state-native-optimization-2026-10-10/probes.jsonl)
shows guarded ancestor reading refusing an unrelated hard-linked sibling while
the selected direct read succeeds. Complete native owner scans can still catch
that static damage later; this does not restore the removed read-stage guarantee.
Its timing measures the benefit at stake, not approved behavior or production
readiness. Optional/bootstrap reads remain unchanged even in that experiment.

## Measurements and interpretation

All values are milliseconds unless marked otherwise. Current reads use four unprofiled observations per group; preparation uses six.

| Variant | Explicit current, 32 | Kind/D→G current, 32 | Explicit prepare, 31 | Kind/D→G prepare, 16 |
| --- | --- | --- | --- | --- |
| base | 1329.6 | 1694.4 | 3084.2 | 2404.6 |
| owner | 1333.1 | 1671.8 | 3025.1 | 2409.6 |
| binding | 1264.1 | 1617.6 | 2903.2 | 2304.8 |
| semantic | 1324.9 | 1692.6 | 3033.1 | 2402.9 |
| combined | 1268.2 | 1604.9 | 2894.5 | 2296.6 |
| direct | 723.9 | 1070.7 | 1860.4 | 1913.0 |
| combined-direct | 656.1 | 994.4 | 1718.7 | 1805.9 |

Fresh explicit workflows: six matched entry sizes 25–30 per variant. Paired workflow is the median of each preparation plus its actual invocation.

| Variant | Preparation | Invocation | Paired workflow | Workflow reduction |
| --- | --- | --- | --- | --- |
| base | 2774.1 | 6528.8 | 9303.0 | 0.0% |
| binding | 2624.8 | 6235.4 | 8860.2 | 4.8% |
| combined | 2611.6 | 5569.7 | 8181.3 | 12.1% |
| combined-direct | 1608.9 | 2990.4 | 4599.2 | 50.6% |
| semantic | 2793.6 | 5990.9 | 8784.5 | 5.6% |

Full-store original/current-control operations: six unprofiled observations each.

| Cohort / operation | Base | Combined | Combined-direct (departure) |
| --- | --- | --- | --- |
| explicit / historical_resolve | 1343.6 | 1309.8 | 640.4 |
| explicit / committed_recovery | 1402.5 | 1296.6 | 647.7 |
| explicit / current_grant | 1936.2 | 1743.5 | 1135.7 |
| explicit / governance_basis | 124.4 | 111.3 | 116.8 |
| kind-DG / historical_resolve | 1703.5 | 1630.6 | 1011.2 |
| kind-DG / committed_recovery | 1709.1 | 1630.4 | 1008.0 |
| kind-DG / current_grant | 2483.4 | 2359.4 | 1756.2 |
| kind-DG / governance_basis | 248.0 | 236.9 | 235.7 |

Illustrative profiled call counts (explicit current at sample 1; fresh invocation at entry 31). Repeated paths count again on every observation.

| Operation / variant | Scans | Lists | Listed entries | Directory opens | Handle info | Canonical encodings |
| --- | --- | --- | --- | --- | --- | --- |
| current_item / base | 1 | 758 | 18960 | 9656 | 36586 | 1166 |
| current_item / combined | 1 | 758 | 18960 | 5745 | 36589 | 1147 |
| current_item / combined-direct | 1 | 198 | 1240 | 3665 | 18629 | 1147 |
| fresh_invocation / base | 4 | 5527 | 100108 | 65491 | 226498 | 6398 |
| fresh_invocation / combined | 4 | 5527 | 100108 | 43218 | 226504 | 4295 |
| fresh_invocation / combined-direct | 4 | 2706 | 13695 | 32719 | 137787 | 4295 |

Native explicit populations contain one activation and the remaining actual
registrations. The kind/D→G population has two activations, two separately approved
kind versions, and the remaining kind-backed items: two native authority proof
populations, not one independent epoch per item. Current-read populations and
repeated preparation use 256-byte questions; fresh effect requests use 27-byte
questions. This does not isolate payload sensitivity. The selected first item's grant/kind closure stays fixed in the explicit
8/16/31/32 series; only unrelated retained items increase. No production bound,
slot accounting, complete-record bound or checkpoint constraint was raised.

With the selected explicit item fixed, baseline current-read medians at
8/16/31/32 transactions are **379/649/1,273/1,330 ms**; combined medians are
**347/619/1,226/1,268 ms**. Retained work bytes grow from about **1.99 MB to
7.91 MB** (decimal units). This observes unrelated-history growth within today's
cap; it is not a fitted large-population forecast.

All five full-store revocation attempts refuse with `work_capacity` and no
outcome. The two accepted-pending recoveries, interrupted at snapshot publication
after using the last slot, both commit the exact original locator at 32 slots.
Their single profiled times are **6,610 ms baseline / 5,254 ms combined**;
these are illustrative, not repeated latency estimates. Logical accepted closure
fits today's budget; fresh revocation does not. No physical-disk reserve or
successful revocation under storage failure is established.

Trials use the same release binary, 32 MiB worker stack and interleaved variant
order, warm OS caches after population, and fresh native owner reopening per call. Baseline spans are
disabled but their branches remain; it is not a pristine binary. Current reads
alternate four enabled/four disabled observations; six unprofiled preparation,
historical/current-control and committed-recovery controls support the ordinary
results. Fresh workflows use matched separate physical stores at entry sizes
25–30, six unprofiled effects per variant; one diagnostic effect at 31 and one
full-store pending recovery per variant are illustrative samples. Process CPU
uses user+kernel time with approximately 15.625 ms resolution. Small groups do not
establish production p95, cold-disk latency or an SLA.
Repeated fresh-effect timings cover explicit registration, not every kind or
administrative effect. Native basis timing does not evaluate evidence freshness;
the configured 60-second value is an admitted freshness-policy ceiling, not a
governance lease that expires during a cohort.

In the illustrative explicit current-read profile, baseline exclusive directory
listing is **556 ms** and canonical encoding **266 ms** of a 1,331 ms call.
Combined reuse leaves listing at 553 ms and reduces canonical work to 254 ms.
It removes 3,911 directory opens while preserving fresh handle-information
observations (36,586→36,589); the three additional checks initialize cached
parents. Owner reuse removes only 19 canonical calls. The semantic cache is
inactive for these one-scan reads and preparations; their small timing variation
is not evidence of a semantic-cache gain.

The entry-31 fresh-invocation diagnostic keeps **four scans, 5,527 listings and
100,108 observed listed entries**. Baseline/combined both return **42,462,447
bounded-read bytes over 273 distinct paths**; current-read variants similarly
return 8,309,001 bytes over 227 paths. Repeated observations count repeatedly.
Combined reuse reduces canonical calls **6,398→4,295**, exclusive canonical time
**1,153→633 ms**, and directory opens **65,491→43,218**, while fresh
handle-information checks remain **226,498→226,504**. Exact-intent reuse removes
reconstruction, not physical observations or currentness. Semantic-only invocation
median improves about 8%; its preparation is unchanged within sample variation.

The direct departure still reads the same bounded leaf bytes, but current-read
listings fall **758→198** and observed listed entries **18,960→1,240**. Its
gain is largely removed ancestor/sibling observation, as the negative probe
demonstrates. An intact static whole-owner scan later in the call does not prove
equivalence of the omitted observation stage.

The pairwise/lineage span is **0.073 ms** for the profiled full current read and
**0.319 ms** total over the baseline invocation's four scans. No maps optimization
was implemented: it has negligible measured cost here, and larger native
populations were not tested. This does not establish large-N asymptotic behavior.
Enabled/disabled full current-read medians differ by roughly **0.6–3.5%** for
the baseline, combined and combined-direct controls across both cohorts; small
groups and ordering confound an exact instrumentation-cost estimate. Ordinary
latency tables use disabled measurements. Inclusive spans overlap; exclusive
time includes unmeasured work and is not physical-I/O or syscall attribution.

Fresh-effect boundary handle samples reach **358** baseline/semantic and **361**
for held-parent variants. Every measured invocation returns from 57 handles to
57; these samples show no accumulating handles, not an exact peak guarantee.
Across population and all variants, the read-mode process's observed OS-reported
working-set peak is **138 MiB**, with sampled private bytes **118 MiB**; effects
are **99 MiB / 88 MiB**. Sampling every 200 ms can miss a final-interval peak.
These are whole-process observations, not per-variant allocations or validated
production memory limits. Returned read bytes exclude fingerprint/checkpoint
reads and cannot establish physical disk I/O.

Avoidable work includes repeated pure reconstruction and reacquisition of already
held static parent handles. Required work includes current content/identity
observations, positive original membership, accepted/pending prefix classification,
current governance and administration checks, no-replace publication and exact
accepted-original recovery. Merely deleting another scan would cross several
lock/publication lifetimes. Semantic reuse is safe only for the same exact input
and predicate; permission cannot be inferred from historical proof validity.

Remaining growth follows the current whole-history contract: unrelated retained
files must be observed to detect their arbitrary corruption on the next operation.
An index, a segment checksum or a database does not certify unread physical bytes.
The existing 32 slots, 128 MiB accounting, 1 MiB records and 256-path exact
checkpoint still apply. Lower constants do not establish useful project capacity.
The measurements do not prove native fixed-closure behavior at 10k/100k history.

## Two distinct conclusions

**A — Technical assessment without a file preference.** Correctness and exact
native recovery are admission criteria, followed by selected-closure scaling,
measured workload/resources, operational recovery/concurrency, and implementation
complexity. No equivalent-native backend ranking is established. The earlier
[storage study](scalable-work-state-storage-feasibility-2026-10-10.md) favored
SQLite on simplified storage timings, while the earlier
[capacity study](work-state-capacity-and-performance-investigation-2026-10-10.md)
conditionally preferred segments. Neither exercised equivalent native authority,
currentness, negative proofs and publication on all backends. SQLite's atomic
transactions can simplify index/state updates, but DB/WAL/SHM physical binding,
original durable acceptance, coherent stale-root attacks and native recovery
remain unproved. Files reuse the existing physical boundary but must implement
complete decision-bearing indexes and coherent publication/recovery. Segments can
reduce physical file-count/audit costs, but require record framing, orphan recovery
and point verification that does not hash an entire segment per lookup. All three
retain proportional observation under the unchanged global detection contract.

**B — Recommendation including the user's preference.** Pursue one native indexed
individual-file candidate after human selection of the detection boundary. It
fits the user's file preference and reuses the current physical authority seam;
this is a conditional recommendation, not proof that files win. Keep SQLite as a
real challenger if native files miss selected budgets or index/recovery complexity
becomes excessive. Introduce segments only for demonstrated locality, file-count
or audit costs. The smallest additional backend discriminator, if needed, is the
same real accepted-original/pending-control/negative-proof operation implemented
through the competing native publication/recovery boundary, at matched bytes and
durable settings—not another synthetic envelope timing.

The [Pro architecture prompt](file-backed-graph-pro-consults-2026-10-10/architecture-prompt.md)
explicitly emphasized file-first/access-audit preference. Its
[answer](file-backed-graph-pro-consults-2026-10-10/architecture-answer.md)
ran no Rust workspace or optimization benchmark and is not preference-neutral.
Its wording that selective detection was “selected” supplies no product authority.
The [historical consultation](file-backed-graph-pro-consults-2026-10-10/history-answer.md)
offers limited leads: support-depth for dependency completeness/selective
invalidation, C3 for generated typed indexes, and weaker-provenance Lift for
held-view acquisition. `memsrc` is a file-to-graph compiler precedent. Snapshot
Memory already exists; only specifically demonstrated primitives/conventions may
be reused. None is presumed to solve work-state transactions or imported here.

## Proposed next implementation increment — human review required

**Problem/outcome:** support the existing native/SDK work-state operations over a
larger retained lifetime with work proportional to an operation's selected
dependency/control closure, and provide explicit complete audit. Propose an
Engine-owned indexed-file representation, preserving existing semantic identities
and receipt/original bytes. This is one capacity outcome, not a generic graph
system, eligibility/closing implementation, new Contracts authority or consumer
expansion. The unchanged-policy optimizations above are candidate ingredients,
subject to production design/verification; none is shipped by this report.

**Concrete behavior:** resolving committed item A validates A's exact registration,
original grant/native proof and optional exact approved kind, while later unrelated
item B is outside this read. Corruption of B is found when B is accessed or by
complete audit. Committed replay of A still works after revocation; it never grants
permission for a new item. A missing/stale/incomplete index generation or an
unresolved accepted publication refuses a new request as uncertain; it cannot
become absence, select the newest surviving file or borrow another grant.

**Owner/interface/persistence boundary:** Engine keeps work meaning, administration,
typed dependencies, index validation and durable state. SDK keeps typed forwarding
and explicit preparation/invocation/recovery responses. Governance still owns fresh
D/G and its complete currentness checks; a work index does not remove that owner's
cost. Existing exact original preimages survive; any new proof-reference/index/
publication representation needs purpose-specific versioning and explicit treatment
of real retained stores. General format defaults remain proposed; Contracts JSON
and current owner/versioned contracts control. No migration or new encoding is
selected here.

The implementation must establish these operation-specific sets:

| Operation | Complete selected set and additional decision proof |
| --- | --- |
| Exact original replay/recovery | Original accepted request, grant/kind/native proof, exact terminal or supported durable prefix; positive membership/status; replay precedes live permission. |
| Item/exact-kind read | Exact committed registration/definition/establishment closure, its reservation and terminal; validated membership/absence for the requested key. |
| Fresh registration | Exact usable grant and optional committed kind/expansion; current D/G, current lineage/head, no suspending administration/revocation, complete request/item conflicts and absence. |
| Kind establishment | Exact definition/content and per-version human approval, current D/G, complete version/request reservation; pending kind does not suspend unrelated work. |
| Scope/D→G/revoke | Exact lineage/predecessor/approval, selected transition predicate, pending-administration exclusion, current native basis and coherent publication. |

Location hints can be disposable; membership/reservation/current-control indexes
are decision-bearing. Their generation must bind exact owner/root/control state,
complete key domains, accepted originals and terminal publications. Negative
lookup requires validated coverage, not a missing file. Publication must keep
original reservation, acceptance and terminal distinct and crash-coherent with
indexes/current head, including bounded discovery of pending/orphan originals
before negative answers. Checksums do not make a coherently rewritten stale head
fresh. Design and prove this invariant before choosing index page/segment layout.
Requirements-input Contracts locators remain declarations, not newly followed
source/qualification authority.

**Required human selections before production preparation:**

1. Select operation-local dependency verification plus explicit complete audit,
   including audit scheduling/interface and whether unrelated audit damage blocks
   all writes or only affected operations. This changes existing complete-inventory
   assertions; they must be deliberately versioned, never silently removed.
2. Select a native population/closure/resource envelope and separate preparation,
   invocation and combined budgets. A proposed initial native target is 10k
   retained transactions, with fixed small closures plus independently growing
   grant lineage, kind/proof diversity and closure bytes. This is not a proven
   capacity, SLA or approved replacement cap. Pro's 100/500 ms examples are not
   approved targets.
3. Select current-control/index generation and crash-coherent publication semantics,
   representation/version boundaries and treatment of actual retained originals.
4. Select ordinary versus protected-administration accounting, finite headroom and
   peak writable accepted-closure reserves. Fresh revocation should remain possible
   at ordinary saturation while protected headroom and writable storage exist;
   accepted originals must close without consuming new ordinary admission. Define
   administrative exhaustion and fail-closed admission explicitly. Logical reserves
   are not physical disk reservation; physical I/O failure still prevents a durable
   promise and must return refusal/uncertainty rather than false revocation success.

**Execution/acceptance after those selections:** implement one vertical owner/SDK
candidate spanning current/history reads, fresh admission and administrative
publication/recovery; reuse the measured control only where its lifetime predicate
is proved. Demonstrate fixed selected closure with increasing unrelated native
history, then independently grow proof/kind/lineage/byte costs. Compare matched
preparation, invocation, combined workflows and audits against this control.
Preserve exact submitted/effective meaning, original byte receipts, pending
administration suspension, irreversible revocation, per-version kind approval,
unchanged-scope D→G, positive absence/reservation, root/owner/lock binding and
accepted-original recovery. Fault stale/missing/corrupt index/control generations,
competing requests, every acceptance/publication prefix, original recovery without
current permission, ordinary/admin saturation and injected write/flush failure.
Process-death tests are not hardware power-loss evidence. No cap-only production
change qualifies. Later physical segmentation/backend comparisons remain broad
options, not additional authorized work.

## Verification, review and disposition

The [bounded verification recipe](work-state-native-optimization-2026-10-10/verify.ps1)
passed **74 tests, zero failures** across the focused physical-guard checks,
Engine repository/work-kind/work-state tests and seven SDK behavior,
integrity/durability/retained-record suites. The
[collected results](work-state-native-optimization-2026-10-10/test-summary.json)
and raw logs retain counts and duration. Five tests are ignored: three child
helpers invoked by their passing parent process tests, and two tests requiring
previously captured original physical roots. Those roots were not reverified.
Passed cases cover all publication prefixes, process death and races, substituted
dependencies/root locks, loss never becoming absence, original history after
governance/revocation/reset, pending-administration suspension and accepted
closure at zero free slots. This is bounded Windows evidence, not formal proof,
Unix verification or hardware power-loss certification.

GitNexus initially had a stale/unusable index; rebuilding with bundled Node 24
restored graph queries with degraded keyword search. The retained
[upstream results](work-state-native-optimization-2026-10-10/gitnexus-impact-final.json)
show HIGH work-state impact and CRITICAL shared parent-handle impact (12 direct
callers, 30 processes). Warnings were reported; all product edits stayed isolated.
Unresolved receivers/process truncation and an UNKNOWN root-struct result limit
the graph; they are not evidence of no impact.

All four retained modes exit successfully with one executable SHA-256; the
[evidence check](work-state-native-optimization-2026-10-10/check_evidence.py)
verifies **641 observations / 199 groups**, matched effect histories, control
sample counts, consistent binary/patch hashes and deterministic summary
regeneration. The source patch applies cleanly to the primary baseline. Primary
product source and Cargo manifests/lock remain identical to that baseline.

One focused independent causal reviewer returned **proceed with the bounded
research/proposal closeout**, with no unresolved P1/P2 findings and no P3/P4
advisories. The reviewer independently reconciled the measurements and paired
medians, experimental source/harness/binary hashes, patch applicability, test
totals and absent primary product/Cargo changes; it also traced the retained
physical, original-history and publication/currentness safeguards. The verdict
does not approve production optimization, a backend, format migration, capacity
increase or successor implementation.

The experimental worktree is **archived as a recoverable managed snapshot**;
the measured patch, driver, source provenance and verification evidence remain
in this research directory. The local commit scope contains this report and its
evidence, the five referenced Pro consultation records and the two narrowly
reconciled roadmap documents. GitNexus staged change detection reports low risk
and zero affected processes; primary product/Cargo equality separately verifies
the product boundary. Six generated GitNexus skill folders remain untracked and
outside the commit because automatic approval review blocked their cleanup.
Human review of the capacity proposal is required before selecting or
implementing its proposed product changes.

The [coverage follow-up](../specs/handbook-contract-membrane/coverage-and-landing-map.md#work-state-capacity-and-performance-follow-up)
and [phase follow-up](../specs/handbook-contract-membrane/phase-and-slice-map.md#near-term-work-state-capacity-and-performance-follow-up)
separate completed research, user direction and unimplemented capability. Human
review of this proposal is the next boundary; this session executes no successor.
