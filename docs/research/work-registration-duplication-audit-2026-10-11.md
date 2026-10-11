# Work-registration duplication and operation design

Status: **completed bounded research/preparation; recommendations proposed for human review**.
Source checkpoint: `2339f5fdd973918db099e9ae9e1e640f10cc8cc8`,
`feat/handbook-contract-membrane-clean`, 2026-10-11. No production behavior,
storage representation, integrity policy, trust boundary or capacity is selected
or implemented by this report.

## Finding and recommendation

The present workflow does substantial avoidable work. Successful fresh
preparation scans all work history **twice**, then successful fresh invocation
scans it **four more times**. Slot reservation additionally inventories the whole
store and rereads its manifest. The paired source path is **six parsed scans,
seven inventories and eight manifest reads**, before nested physical checks.
These are source-derived logical counts, not six new timed observations.

Some repetitions establish different facts at different times. Preparation's
conflict check does not reserve anything. Invocation must establish current
permission and complete conflict/absence evidence under its consuming guards;
historical replay must work without current permission. Conversely, recomputing
the same exact identities, expansions and expected record bytes supplies no new
freshness. The repeated whole-history algorithm also observes unrelated bytes
and repeatedly enumerates ancestors/siblings. The two-call interface explains
only part of the cost.

**Recommended operation contract:** preparation binds exact submitted/effective
meaning and produces the portable original locator before any effect. Invocation
owns authoritative current permission, durable reservation and verified
publication. For explicit composition, binding can be pure over exact supplied
grant/input bytes; kind-backed binding still resolves and verifies the exact
committed kind provenance from retained truth. Neither binding promises a usable
grant, item absence or eventual commit. This deliberately changes preparation's
early-error contract and needs human selection. Do not preserve two expensive
admission passes merely to preserve the existing method shape.

**Recommended next implementation scope, after review:** that preparation
contract plus operation-local reuse of successful pure derivations and a
non-bootstrap invocation seam. Preserve invocation's physical observations,
historical-priority classification, publication and original recovery initially.
Do not bundle new indexes, a backend, capacity increases or a freshness witness.
If early current-permission/conflict errors are retained instead, consolidate
preparation into **one** held authority/work-lock scope and one history snapshot;
that is a bounded fallback, not a reason for its present two scans. The remaining
whole-history cost requires the separate capacity/detection design below.

## Authority and evidence

The [HCM entry point](../specs/handbook-contract-membrane/README.md),
[lifecycle](../specs/handbook-contract-membrane/work-unit-lifecycle.md),
[review policy](../specs/handbook-contract-membrane/review-and-closeout-policy.md)
and [preparation prompt](../specs/handbook-contract-membrane/authority-preparation-prompt.md)
bound this assignment. This is an investigation and next-change recommendation,
not a new slice/packet authority or successor dispatch. Relevant selected contracts
are [foundation Packet 1](../specs/handbook-contract-membrane/slices/work-item-state-foundation/packet-1.md),
[composition Packet 1](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-1.md)
and [kind Packet 2](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-2.md).
The [kinds/relations amendment](../specs/handbook-contract-membrane/semantic-model/amendments/work-progression-kinds-and-relations.md)
and [operational-reliance ownership](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership)
keep exact meaning, current authority, eligibility and state-owner commit separate.
Registration still creates only `open` work; no completion/eligibility is added.

The [structured-data standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
is **proposed**, not general format approval. Current owner/versioned JSON bytes,
domains, bounds and original receipts control. Pure byte reuse is not re-encoding.

Existing [native optimization research](work-state-native-optimization-investigation-2026-10-10.md)
and its [raw evidence](work-state-native-optimization-2026-10-10/README.md) supply
measurements; [storage feasibility](scalable-work-state-storage-feasibility-2026-10-10.md)
and [capacity baseline](work-state-capacity-and-performance-investigation-2026-10-10.md)
supply earlier context. The retained, presently untracked
`work-state-capacity-pro-evaluation-2026-10-10/{README.md,answer.md,prompt.md,packet-index.md}`
is advisory evidence, not authority. Its downloaded Python was not executed.
Its proposed external monotonic witness, audit cadence and resource numbers are
not adopted. Earlier Pro consultations are historical advice; no new consultation
was necessary for this source-local question.

Product/Cargo source at this checkpoint equals experiment baseline
`8013948ebd3fbef6725f5d72f3d29120f7617009`. GitNexus's current index records the
same HEAD and absolute repository path
`C:/Users/spmcc/Documents/__Project_Code/handbook`; calls were bound to that path
because five registered repositories share the name `handbook`. CLI query/context
were used and leads checked in source. Keyword/graph results have malformed names
and overbroad transitive process reach; the graph is navigation, not proof.

## End-to-end timeline and lifetimes

Source aliases in this report refer to
[SDK work state](../../crates/sdk/src/work_state.rs),
[SDK local check](../../crates/sdk/src/local_check.rs),
[transaction](../../crates/engine/src/work_state/transaction.rs),
[store](../../crates/engine/src/work_state/store.rs),
[types](../../crates/engine/src/work_state/types.rs),
[kind](../../crates/engine/src/work_state/kind.rs),
[scope](../../crates/engine/src/work_state/scope.rs),
[native work authority](../../crates/engine/src/local_authority/work_registration.rs),
[native local check](../../crates/engine/src/local_authority/local_check.rs),
[local authority](../../crates/engine/src/local_authority.rs) and
[physical durability](../../crates/engine/src/repository_durability.rs).
Line numbers below are for the source checkpoint, not the instrumented patch.

| Stage / calls | Actual work and established fact | Held lifetime / observable return |
| --- | --- | --- |
| SDK `prepare_work_registration` (work_state:132–140) → `require_initial` (277) → `observe_local_check_basis` (local_check:254) | Pure initial/check equality/canonical validation. SDK resolves Contracts applicability before and after native basis observation, comparing the complete supported D/G basis. | Observation, not consuming authority. Returns preparation refusal if basis/input is unsupported. No work effect. |
| `prepare_registration` → `resolve_current_terms` (transaction:981–1000), scan **P1** at 995 | Validate grant/basis equality. `with_work_registration_current` acquires native OperatingGuard, current local-check observation and authority-tree guard; work store/lock opened; all retained history read and parsed; exact current committed grant selected. | Native guards before work lock. Validate before return. **All these guards drop** when `resolve_current_terms` returns. Work grant scope and local-check current basis are distinct propositions. |
| Remaining preparation (1002–1034), scan **P2** at 1018 | New retained root/work lock. Derive stable item identity. Kind form resolves exact committed receipt and stores expansion/provenance. Derive request identity; validate input, actor, declared grant scope and kind bounds; select current grant again; check request/item conflicts. | P2 has a work lock but no retained P1 authority guards. Read-only `reservation` is an observation. Sealed Prepared contains only its locator; it retains no lock, snapshot or reservation. Caller can save the locator before invocation. |
| SDK `register_work_item` (143–151) → Native `register` (1036) → `execute` (1256), historical scan **I1** via `resolve` (1237) | Validate submitted original; briefly open root for owner/anchor state, drop it, reopen owner/lock for complete history classification. Request equality, original record closure, replay/noncommit/pending/uncertainty take priority. | Historical work lock drops before live authority acquisition. Committed original returns `Replayed` even after live revocation, D→G or reset. Original noncommit/uncertainty/refusal also returns here. |
| `with_work_registration_establishment` (native authority:156–190) → consuming callback; scan **I2** (transaction:1347) | Acquire original OperatingGuard/check authority. Before/after bootstrap current checks; Item bootstrap callback is a no-op. Retain authority tree; acquire work lock; scan all history again; fresh conflicts, current grant and exact kind dependencies. Pin original work files and anchor/owner bytes (1349–1372), excluding this transaction. | Native locks/tree before shared work lock, held across acceptance through terminal and verification. I1 cannot authorize I2 after reacquisition. Work proof alone does not cover unknown names or this transaction. |
| New intent and acceptance (1423–1477) | Check lifetime/peak-byte capacity, capture original authority proof, construct and bound all potential derived records. `reserve_intent` first occupies a positive slot; create `accept-*`, write exact intent, write positive never-attempted checkpoint; no-replace directory rename publishes accepted original. | Reservation/AcceptanceCandidate may precede a recoverable complete original. Intent/decision/candidate are not committed item truth. Faults after possible effect yield uncertainty. |
| Candidate and preterminal checks (1478–1533), scan **I3** at 1494 | Write decision, snapshot and pending terminal. Fresh native/work validation; reread complete history, conflict/grant/kind checks excluding this original's own reservation. Retain current manifest checkpoint. Persist `not-attempted` → `attempted` before invoking terminal installation; validate native/lock/original-work/manifest proof. | Own writes legitimately change inventory and slot evidence. Old scan guards were dropped, so cached prior inventory is not automatically valid. Attempted evidence without terminal is uncertainty. |
| Terminal publication (1534–1540), scan **I4** at 1541, return (1551–1575) | No-replace pending-terminal → terminal rename is **semantic commit**. Complete scan proves exact committed row; final native/work/original-proof/manifest validation. | Successful fresh call returns `Committed` with exact receipt. Terminal may exist despite a later error; caller retains original locator and resolves/recoveries its truth. Wrapper's final validation occurs after consuming work lock drops, with native guards still held. |

`scan` (transaction:321–383) retains a work-history tree and root-level metadata
checkpoint, inventories, decodes/reconstructs all transactions, validates grant
and kind dependency relationships/lineage, validates tree/metadata/work lock,
then **drops its tree/checkpoint**. `inventory` (store:197) consults `manifest`
(228). `reserve_intent` (store:452) inventories again (458), then calls manifest
again (459). Hence P1/P2 = **2/2/2** scans/inventories/manifests; I1–I4 plus
reservation = **4/5/6**. These counts describe successful fresh Item registration
in an existing store. Error exits and recovery are different paths.

`OperatingGuard::acquire` (local authority:855–910) acquires initialization, then
promotion, registry, lifecycle and local-operating locks; they drop in reverse
order. Local-check currentness is resolved through those retained mechanisms,
not a new work permission. `store::open` (549–571) then acquires the separate
`.handbook/work-state/work.lock`. These participating-owner locks and the native
physical guards have different guarantees; regular files remain externally mutable.

Explicit and kind-backed registration use the same scans and transaction stages.
Kind adds immutable definition/approval/receipt validation and exact expansion,
not a new scan. Both kind bounds and original grant scope apply. K2 never replaces
K1; requirements-input Contracts locators are declarations, not newly followed
qualification or assessment dependencies.

D/G changes the native applicability closure and exact original basis, not those
scan counts. Preparation rejects a differing observed basis before P1; invocation
uses the original grant's basis. New G authority cannot finish an old D original.
Item registration requires an already established owner and activated grant; it
cannot bootstrap. Kind-first/grant-first establishment alone uses the bootstrap
origin (transaction:62–67), serialized creation and original bootstrap recovery.
Its genuine sibling/anchor changes justify current checks around bootstrap and
retaining the authority tree **after** creation. Grant activation may capture
proofs for compatibility in addition to final intent; do not charge those counts
to fresh Item registration.

### Original recovery, including process death

| Durable evidence | Existing behavior / repeated work |
| --- | --- |
| Exact terminal and complete matching original closure | Initial historical scan replays original result without fresh permission, new slot or effect. |
| Complete positive inventory, no original | Resolve reports `Unaccepted`; recovery returns it rather than starting a request. |
| Accepted never-attempted original with usable original authority | Resume that original through guarded I2/I3/I4. Skip new `reserve_intent`; own reservation is excluded only for its own continuation. |
| Original authority fails before supported terminal attempt | `close_noncommit` (1577) performs a historical scan and, only for neither attempted nor staging, writes positive noncommit and scans again to prove it. Immutable request key/history remain; affected item reservation can release. |
| Attempted without proven terminal, staged/partial/missing evidence | `Uncertain`; no absence, new-key retry or terminal guess. Early slot/candidate prefixes may fail scan itself and retain the slot indefinitely under supported recovery. |

The [integrity test](../../crates/sdk/tests/work_state_integrity.rs)
`actual_early_acceptance_interruptions_keep_positive_reservation_and_truthful_uncertainty`
(447–520) is the counterexample to guaranteed closure. The
[durability tests](../../crates/sdk/tests/work_state_durability.rs) at 198–284 and
679–721 establish original prefix/transition and process-death distinctions.
No extra admission slot for supported recovery means neither guaranteed terminal
completion nor physical write-space reservation. Request identity must be usable
and retained by the caller **before the first possible effect**; returning it only
in a combined call's reply does not survive process death/lost reply.

## Repetition inventory and semantic ownership

This inventory covers the logical repeated work reachable on preparation,
invocation and original recovery, including the nested physical multiplicative
costs. It is not a list of every syscall. Counts are exact source-path counts
where stated; independent timing is unavailable unless explicitly given in the
measurement section. “Reuse” means successful derivation over **complete exact
inputs**, scoped to one operation; it never means cached permission or absence.

| Repeated logical work / exact sites | Inputs, fact and guard lifetime | Correct owner; disposition and observable effect |
| --- | --- | --- |
| Initial/check/grant/input validation and identity encoding: SDK `require_initial`; `observe_local_check_basis`; transaction 984, 1029–1030, execute 1262, resolve 1233; types 355–430 | Exact supplied initial/grant/input/provenance bytes are immutable within sealed preparation/owned invocation. Portable decoded requests remain untrusted. Current filesystem is not consulted by these pure identity/shape rules. | Engine owns meaning/binding; SDK validates forwarding congruence. Validate once per newly received exact input, retain successful results and byte encodings. Reuse cannot validate different bytes, relax record bounds or mint a receipt. Stable invalid-input refusal remains before effects; reducing preparation changes which other error wins first. Independent cost unmeasured. |
| SDK applicability before/after native observation: local_check 254–289; Engine inward `work_registration_current` | Mutable Contracts D/G and governance inventory; SDK observation is unlocked relative to subsequent Engine consumption. The pair detects a change during observation. Native inward checks establish original basis under its own guards. | SDK owns outward applicability resolution, Engine owns consuming authority. Keep before/after when observation is requested. Binding-first preparation can omit this live observation entirely and use exact supplied original basis; live mismatch then refuses at invocation. That is an early-error change, not reuse of stale applicability. Full independent attribution unavailable. |
| P1/P2 current grant, kind/conflicts and root/store/lock acquisitions: transaction 993–999, 1016–1033 | All history/owner/manifest mutable between scopes; P1 authority/work guards drop before P2. Prepared retains no evidence. P2 grant repeat catches a between-phase administration change, but P1 native currentness does not remain held through P2. | Current terms read owns its current result. Preparation should own exact binding; authoritative conflicts/permission belong to invocation. If preserving preflight, one native callback/work lock and one scan can establish all preparation facts coherently. Do not reuse P1 after releasing its guards. Two scans measured as part of preparation, separate P1/P2 times unavailable. |
| I1 history classification then I2 admission: transaction 1306→1237, 1347 | I1 proves original durable truth without current permission. I2 reacquires authority/work locks and fresh inventory; another process or external change may intervene. | Engine transaction owns both propositions. Preserve replay-first semantics; the current lock choreography needs both observations. A combined API alone eliminates neither. A later guarded-first fast path with historical fallback is a possible redesign, not a proven safe deletion: revoked/reset replay, conflicting original keys and lock order must still work. |
| I2/I3/I4 complete scans: transaction 1347, 1494, 1541; scan 321 | Different transaction prefixes and current observations. Previous scan's tree/metadata guards have dropped; held work lock serializes participating writers but not arbitrary byte changes. Relationships/negative inventory are recomputed. | Transaction owns admission/preterminal/postcommit proof. Pure expected records may reuse across all scans; freshly observed record status and complete inventory cannot. Replacing whole scans with validated unchanged closure plus changed-row verification needs a complete retained name/absence/manifest predicate, currently absent. Postterminal proof must still establish exact published result. Four invocation scans have 5,316.79 ms inclusive in the diagnostic; no safe scan-deletion saving measured. |
| Reservation inventory and manifest: store 458–459, inventory 228 | Current complete slot inventory, bytes and positive empty successor slot. New reservation changes manifest by displacing empty slot into a transaction receipt. Prior complete parsed scan contains similar information but no retained complete-name witness. | Store owns finite accounting/positive slot acceptance. Consolidate one inventory's decoded manifest within `reserve_intent`; broader reuse of consuming snapshot requires revalidation against external changes. Preserve first possible-effect classification, monotonic slot accounting and exact intent fingerprint. Extra inventory/manifest times unmeasured. |
| Conflict/reservation checks: transaction 507, P2 1033, resolve 1238, I2 1348, I3 1495, close_noncommit | Request equality and item/kind/head uniqueness depend on complete current rows. Own pending original may be excluded only for original continuation. Noncommit releases affected key reservation, not immutable request identity. | Invocation/store admission owns durable conflicts; preparation's check is advisory. Retain fresh checks after time/reacquisition or changed rows. Within one proven unchanged snapshot, retain derived result. Removing preparation check defers `work_item_reserved`/request conflict until invocation; no permission or reservation is lost. Times unmeasured. |
| Current grant and scope: transaction 558, I2 1395, I3 1500; types 174, 358; scope 239 | Grant selection/head/no successor/no revocation is mutable history. Scope's structure and matching exact effective input are immutable. `scope.permits` validates scope again after grant terms already validate it. | Keep fresh current grant/head/administration checks. Reuse validated scope structure and pure membership/limits result for identical inputs; never union older scopes or use a fitting successor. Grant checking is a narrow operation, not all filesystem guards. Independent total unmeasured. |
| Kind receipt/expansion and full locator validation: P2 1019–1028; `validate_kind_dependency` 626 at I2/I3; types 388–430; kind 309–313 | Immutable exact receipt/definition/approval/template and submitted values; retained committed dependency presence is fresh. Dependency validator currently calls whole `locator.validate()` even for explicit input. Receipt validation rebuilds receipt and recursively validates definition/approval/binding. | Binding/expansion belongs before effect because it fixes original locator. Reuse exact validated expansion/receipt semantics; retain current physical committed-kind equality and same-project proof. Removing recursive pure revalidation should not remove kind lookup. No additional scans for kinds; separate timing unmeasured, kind/byte/diversity sweeps remain needed. |
| Historical proof validation: parsed 259–267; proof `validate_historical` 238; scan-local `proofs` set 335 | Captured immutable bytes in Intent, not live permission. Within a scan each distinct proof validates once, but each row canonicalizes proof identity; next scan resets deduplication. Original Charter and operating/local-check record encodings are rebuilt inside historical validation. | Original-history verifier owns this proof. Exact-intent/full-proof bytes may key successful pure reuse across scans, including reopened scopes when bytes are freshly read. Continue owner/project/check/basis/path/status/relationship checks. Hash alone, pathname or metadata does not justify a semantic cache hit. Experimental reuse is measured; persistent cache is not proposed. |
| Immediate duplicate proof shape: native authority capture 115–116, historical validation 243–244 | Capture directly validates shape, then immediately calls historical validation which starts with the same shape validation. Same immutable local value; no effect/intervening mutation. Shape hashes copied files, rebuilds governance identity and encodes proof (501–585). | Remove the first pure call while retaining the historical validator's first call and refusal ordering. No fresh observation is removed. Independent count/time and resulting gain are unmeasured; include this narrow deletion in the next refactor. |
| Owner/bootstrap expected bytes: store `WorkLock::validate` 178–195, inventory/manifest 198/349/426 | Same lock's immutable owner/bootstrap values are encoded repeatedly (owner twice, bootstrap once per validation), while anchor/owner/bootstrap physical bytes are reread. | Store owns expected origin bytes. Retain canonical owner/bootstrap encodings for the lock lifetime, preserving fresh reads/equality and record bounds. Prior owner-reuse prototype measured this class; no new result here. |
| Decision/output/terminal/checkpoint reconstruction: transaction 132–232, parsed 269–288, preflight 636; Item 1438–1457 and 1478 | Immutable complete Intent determines output/snapshot, fingerprints and seven expected record variants. Parsed eagerly encodes variants even when their files are absent. Item preflight constructs output, then publication constructs it again; intent encoding repeats for bounding/reservation/write. | Engine transaction owns deterministic artifacts and all preacceptance record bounds. Construct a bounded validated artifact bundle once; reuse same bytes for writes/comparison. Check new observed durable-prefix membership every scan. Do not defer a potentially oversize noncommit/terminal until after acceptance. Lazy encoding of absent variants needs proof of the bound/refusal contract; reuse is the safer first change. |
| Native live checks: native authority 41–53, capture 60/117, wrappers 144/146 and 186/188; Item 1447/1492/1530/1551 | `validate_current` validates authority tree, OperatingGuard and rereads exact native local-check outcome/currentness. Eight calls for fresh Item, plus two direct wrapper checks = ten local-check-current calls. Mutable native heads, revokes, pending state and physical bytes remain observable. | Native authority owns live applicability/currentness through effect. Keep fresh observations at acceptance, candidate, attempted terminal and verified return; split pure expected-outcome/encoding from fresh reads. Do not delete all eight because locks are held. Time spans include nested physical work; eight calls measured at 742.28 ms inclusive / 196.57 ms exclusive. |
| Establishment before/after no-op: native authority 164/168 and guard validation 165/167; Item bootstrap callback transaction 1338–1343 | Actual grant/kind bootstrap changes sibling tree/root anchors. Item cannot bootstrap, so callback is guaranteed no-op on this branch; guard acquisition and later validators still happen. | Select `with_work_registration_current` or equivalent non-bootstrap path for Item while retaining acquisition/current and lifecycle checks. This eliminates the extra direct observation and two explicit bootstrap guard validations by source comparison (ten→nine local-check-current calls), not a measured latency claim. Keep before/after for real bootstrap; exact error timing under races needs checking. |
| Fresh authority capture: native authority 58–118, fresh Item 1436 | Once per fresh Item, not once per scan. Traverses four native roots, six exact paths plus Charter historical dependencies, reads/copies/hashes files, validates shape/history then currentness. This duplicates observation already used by native current validators, but produces a new durable original proof. | Native owner owns complete original authority capture, transaction retains it. Capture bytes once per exact original; pure derivations may reuse. Do not replace capture by historic grant proof or a tree fingerprint. Complete capture set is broader than authority-tree guard; full physical reuse is unproven. Count is source-derived; capture's independent exclusive total was not instrumented comprehensively. |
| Work/authority tree, exact metadata/work/manifest checkpoints: scan 322–333/380–382; Item 1373–1376, 1512–1520, 1532/1553; physical 244–287 | Retained file/path identity and content observations. Own transaction and slot rename legitimately change portions of the store. Work proof covers every other known transaction plus owner/root anchors; I3 manifest proof covers current manifest. Authority tree begins at `.handbook`, not root siblings. | Physical mechanism owns physical validity; transaction selects complete sets and mutation-specific refresh. Retain/revalidate appropriate checkpoints, refresh changed domains, then test external additions/deletions/substitution. Current proofs do not collectively establish unchanged complete names/absence. Independent allocation/hash/checkpoint totals unmeasured. |
| Ancestor/sibling enumeration and optional-read paths: physical owner optional reads/store reads, directory listing, read guards | Each bounded owner read may validate ancestors and enumerate siblings, then read leaf bytes. Scan tree retention, inventory, historical/root/dependency checkpoints and native capture repeat those paths. Sibling unsafe links and unknown/partial entries are currently detected during observation. | Preserve observation policy until deliberately changed. Reuse retained directory handles with fresh handle information and checks; the measured parent-handle candidate does this. Direct leaf shortcuts remove observation and change error timing/detection: demonstrated unsafe unrelated hardlink counterexample. Listing dominates diagnostic exclusive cost; lower-level calls are amplification, not independent logical permissions. |
| Fresh byte/path/metadata validation versus retained handles | Windows directory handles deny DELETE sharing and resist directory/root rename; regular retained files share READ/WRITE/DELETE, permitting regular writes/substitution. Unix no-follow/openat handles do not freeze bytes. Retained tree validation reopens paths, compares versions and fingerprints retained/current files. | Physical current observations remain necessary. Held handles can avoid reacquisition, not substitute for bytes, metadata or complete negative evidence. Physical reads/fingerprints may be consolidated only for a completely specified lifetime and external-change model. Do not claim cooperative work/native locks freeze arbitrary filesystem changes. |
| Pairwise dependencies, duplicate-kind checks and authority lineage: scan 343–379, history 387–506, relation-kind parse 720 | Current full rows determine complete chain, committed grant/kind dependencies and local pending administration; relation tags derive from immutable proof bytes. Some loops are quadratic. | Reuse immutable relation decoding and derive indexes once per validated snapshot if justified. Fresh lineage/relationship assertions remain. Diagnostic pairwise/lineage cost 0.3193 ms inclusive; authority-history subspan 0.0066 ms. Maps changes are low priority at today's cap; large native lineage/populations unmeasured. |

### Complete guard predicate, not “under a lock”

Pure semantic reuse needs freshly obtained **identical complete bytes** and the
same versioned rules; each scan still checks path/owner/project/basis congruence,
durable prefix, record equality and cross-record relationships. The experimental
cache is keyed by full Intent bytes, only stores successful derivations, and is
operation-local. That is stronger than caching by path or SHA alone.

Physical snapshot reuse needs more: unchanged complete directory membership,
positive slot/control coverage and complete selected file bytes/physical identity,
with own mutation domains explicitly refreshed and final validation in the same
consuming lifetime. Source does **not** provide that predicate as one reusable
work snapshot today. Work lock alone is insufficient. The old scan's dropped
tree, the known-file work checkpoint and the later manifest checkpoint cannot be
assumed to supply fresh unknown-name or absence evidence in combination.

The final native checks serve a through-publication interval, not new permission
for historical replay. Regular files can change between a preterminal observation
and terminal rename; retained before/after validation detects a changed dependency
across that interval. It can conservatively return uncertainty even if a change
occurred only after terminal, with original history later resolved separately.
That observable distinction must be considered in any proposal to move or remove
postterminal checks. No requirement for today's exact count of eight is inferred;
deleting a call needs an equivalent interval/predicate or an explicit contract
change. Adjacent pure proof-shape calls have a different, immutable predicate.

Authority-tree traversal starts at `.handbook` (physical:856). Its exact-path list
includes root-level initialization records that this traversal cannot reach;
those are separately checked via initialization/Charter and copied proof.
Capture additionally covers the project marker and governance lock. A proposal
to reuse all captured bytes must enumerate these extras and missing/revoke
observations explicitly. No complete pure/physical split for all native current
validation has been measured; no blanket proof-capture cache is justified here.

## Quantified causes, reused evidence

These are the **2026-10-10 measurements**, independently reconciled during this
audit, not new native results. The illustrative profiled fresh invocation at 31
retained transactions took **7,494.8664 ms wall / 7,437.5 ms CPU**.

| Instrumented span | Calls | Inclusive ms | Exclusive ms |
| --- | ---: | ---: | ---: |
| `list_directory_bounded` | 5,527 | 3,882.9095 | 3,089.7616 |
| `canonical_json_bytes` | 6,398 | 1,152.5012 | 1,152.5012 |
| `open_windows_directory_guard` | 65,491 | 853.3324 | 853.3324 |
| `windows_handle_info` | 226,498 | 827.6510 | 827.6510 |
| Instrumented `scan_reusing` (baseline has reuse disabled) | 4 | 5,316.7941 | 272.6438 |
| `validate_current` | 8 | 742.2773 | 196.5731 |
| `read_regular_bounded` | 2,039 | 406.3136 | 138.7315 |
| `with_work_registration_establishment` | 1 | 6,172.9393 | 210.6718 |

The first four **exclusive** spans sum to 5,923.2462 ms, about **79.03%** of wall.
Do not add inclusive scans, native validation or the consuming wrapper to their
nested times. The wrapper includes acceptance/publication and is not “permission
checking time.” `current_grant`, scope matching and capture do not have a complete
independent total. These are code spans, not ETW/syscall/disk attribution.
Warm CPU near wall supports a CPU/kernel/metadata-heavy explanation for this
fixture; cold disk, flush, antivirus and contention remain separate questions.

The measured combined preserved-policy candidate retained **four scans, 5,527
listings, 100,108 listed entries and 42,462,447 returned bounded-read bytes over
273 paths**, while reducing canonical calls **6,398→4,295** and directory opens
**65,491→43,218**. Handle-information observations stayed **226,498→226,504**.
It reconstructs less, not observes fewer historical leaf bytes. Returned bytes
exclude retained checkpoint/fingerprint reads. Scan count does not capture that
additional physical amplification.

| Matched uninstrumented explicit entry sizes 25–30 | Baseline ms | Combined preserved-policy ms |
| --- | ---: | ---: |
| Preparation median | 2,774.12785 | 2,611.594 |
| Invocation median | 6,528.83145 | 5,569.6595 |
| Actual paired workflow median | 9,302.9593 | 8,181.2535 |

Paired gain is **12.06%**. These are six **different history sizes**, not six
repetitions at one size. The semantic-only candidate's invocation median was
5,990.9 ms versus 6,528.8 ms; its preparation was unchanged within sample
variation. Parent-handle acquisition reuse helped preparation but did not remove
its two scans. No new implementation gain for reduced preparation, wrapper
selection or full scan replacement is measured. Removing two preparation scans
does not imply saving the complete 2,774 ms: binding/kind/SDK work remains, and
cost moves only when its semantic owner changes.

The combined-direct paired median **4,599.2144 ms** (50.56% gain) changed unsafe
unrelated-path detection. It is not an equivalent-policy improvement. Merely
running a later whole-store scan does not restore the omitted observation stage.
Preserved-policy read costs still grow with unrelated history: explicit fixed
item combined median about **347 ms at 8** to **1,268 ms at 32 transactions**.
Pure reuse alone therefore leaves the main capacity scaling problem.

Instrumentation branches remain in the baseline binary; enabled/disabled read
controls differed about 0.6–3.5%, with small samples/order confounding an exact
overhead estimate. Release/MSVC, warm-cache Windows/NTFS, owner reopening and
32 MiB research worker stacks are the measured scope. There is no pristine-binary
new experiment, cold/p95/Unix/power-loss proof, or native capacity above 32.

## Credible operation patterns

| Pattern | Eliminates / retains | Semantic and resource consequences |
| --- | --- | --- |
| Retain split and present eager preflight | Keeps both preparation scans, SDK live observation and all four invocation scans. Pure reconstruction/parent-handle reuse still helps. | Preserves early stable-state errors but pays for observations that expire on return. No reservation or guaranteed invocation success. Not recommended as-is. |
| One coherent eager-preparation scope | P1/P2 become one scan under native guards then work lock; same rows supply current grant, exact kind and conflicts. Invocation still needs I1–I4. | Good fallback if early errors matter. Guards last longer than each present preparation scope; still release before caller delay. Race observation timing changes; hold no handles across caller think time. Preserve stable invalid-basis/current-grant/input refusal order where practical. Saving unmeasured. |
| **Reduced binding-first preparation + authoritative invocation** | Explicit preparation removes both work scans and live SDK applicability observation; kind preparation uses one exact retained-kind verification (currently a full scan). Retains pure grant declared-scope/input validation and exact original identity. I1–I4 initially remain. | Recommended phase ownership. Revocation/narrowing/D→G/conflict after or before binding is refused at invocation; preparation's response no longer asserts live usability/absence. Kind remains exact K1, never latest. No locks survive return. Caller retains locator before effects. Human must select deferred errors/changed preparation guarantee. |
| Operation-scoped exact semantic and held-parent reuse | Avoids repeated successful validation/encoding/expansion; fresh bytes and record status still read. Measured combined paired gain 12.06%; further bundle/wrapper candidates unmeasured. | Compatible with either split contract and recovery. Cache scope ends at invocation; cannot survive crash as authority. Bound memory/handles and preserve invalid-byte and size refusal. Full physical-history reuse additionally needs the missing complete snapshot predicate; do not treat it as already available. |
| Optional combined binding/invocation | Can share one short live scope and exact artifacts if internal lifecycle is actually redesigned. Simply calling current prepare then register saves zero scans. Kind resolution can share guarded admission history after original binding is made available. | Before effects, synchronously deliver the **complete exact locator** to a caller persistence callback, or take a locator the caller already retained. Callback failure refuses before any work slot/write; no owner ledger/capsule added. Process death after delivery uses ordinary original-key recovery. Running arbitrary callback under owner locks risks latency/reentrancy/deadlock; prefer before consuming locks and then revalidate. Sharing guarded kind binding with callback is a distinct tradeoff, not automatic free reuse. Optional only for demonstrated immediate-call convenience. |
| Change invocation's lock/classification choreography | May consolidate I1/I2 for fresh healthy calls, rather than assuming four scans forever. A guarded historical classification with retained-history fallback could preserve replay after unavailable live authority. | Credible later alternative, not selected for the first change. Must prove replay before permission decisions, no effect before identity/reservation classification, original conflicts, native-before-work lock order, race/fallback error precedence and no deadlock. Cannot keep I1's work lock then acquire native locks. No measured savings or established complete implementation. |

Fresh authorization across elapsed time, after reacquisition and through mutation
remains necessary in every credible pattern. The exact number/placement of
observations is an implementation property, not a fixed semantic requirement.
Changing the operation interface does not alone remove work proportional to all
unrelated historical bytes. A long-lived prepared guard is rejected: caller delay
would block authority administration/competing writers and retain unbounded
resources without solving crash recovery.

## Freshness primitives and remaining capacity work

Before adding anything new, the existing store already has:

| Existing primitive | Establishes | Does not establish |
| --- | --- | --- |
| `.handbook-work-state.v1.json`, bootstrap intent and work `owner.json` | Positive original establishment, project/initialization/physical-work-lock origin and supported original recovery. | A per-work-transaction externally protected generation. |
| Initialization lock/claim/receipt, repository identity and physical fingerprints | Exact physical/project origin and retained identity; substituted/missing roots/locks fail closed. | An independently authenticated actor or monotonic work history outside an administrator's rollback set. |
| Fixed slots, displaced slot receipts and original transaction checkpoints | Positive bounded complete accepted population, occupancy, no silent loss of a referenced transaction, terminal-attempt status. | A scalable point index or proof that a coherently restored earlier complete valid store is latest. |
| `LocalCurrentAuthorityWitnessV1`, OperatingGuard, retained native authority tree and local-check basis | Lifetime-sealed observation/guarded current native applicability; origin/heads/revocation/dependencies checked. | A witness updated by every work transaction, a durable work request journal, or frozen arbitrary regular-file bytes. |

The [current governance limit](../specs/handbook-contract-membrane/slices/admission-governance-establishment/README.md)
(261–267) explicitly excludes cryptographic authentication against an
administrator rewriting the entire store and total-anchor-erasure freshness.
Ordinary stale/incomplete indexes, partial loss and substituted roots must fail
closed under current integrity meaning. A coherent administrator restore of all
valid earlier bytes is different. An external monotonic witness would strengthen
the restore/trust boundary and needs separate human selection; it is not a
missing mandatory cache flag. Preserve and state today's limit for the proposed
phase refactor. A local generation/hash chain alone cannot solve coherent rollback.

The current **32 retained lifetime transactions / 128 MiB** are shared by grant,
administration, kinds, items and pending/noncommit history. Fresh revocation
refuses at saturation. Raising a constant alone solves neither availability nor
the nested scan/checkpoint/path bounds and history cost.

The user's file preference and operation-specific complete-dependency verification
direction support an **indexed-file candidate**, after selecting the changed
detection boundary. The larger necessary change is complete typed request/item/
kind/current-control/reservation membership with positive negative coverage,
discoverable accepted originals, crash-coherent publication and explicit full
audit. Unrelated B damage detected when B is accessed/audited changes today's
complete-inventory assertions; it is not a safe cache optimization. Point misses
must never become absence from a missing/stale/incomplete index. No generic graph
engine, SQLite selection, migrations or automatic audit cadence follows here.

Separate later choices remain detection/audit write-blocking semantics;
population/closure/lineage/proof diversity and resource/phase budgets; versioned
index/control publication and actual retained original treatment; and protected
administration/accepted-closure accounting. Ordinary saturation should leave
bounded revocation opportunity when its separate authority, logical reserve and
writable storage exist; logical quota cannot guarantee a successful durable flush.
These choices remain in the [capacity follow-up](../specs/handbook-contract-membrane/coverage-and-landing-map.md#work-state-capacity-and-performance-follow-up).

## Prioritized next change and acceptance

1. **Select phase semantics at human review.** Recommend binding-first preparation,
   live authoritative invocation and original historical recovery as above. Decide
   whether early permission/conflict errors are materially valuable. If so, use
   one coherent eager-preparation scan. Choose the combined convenience only if
   its pre-effect locator delivery and callback/resource contract are useful.
   These are proposals; existing selected packet documents still describe the
   current eager preparation contract.
2. **One bounded Engine/SDK phase-owner refactor.** Replace the selected preparation
   behavior, remove the immediate duplicate proof-shape call, consolidate immutable
   validation/effective input/owner and derived record bytes,
   reuse successful exact-intent semantics within invocation, and select the
   non-bootstrap seam for Item. Retain fresh inventory/status/authority checks,
   exact bytes/fingerprints, original replay/recovery, capacity and format contracts.
   A private validated artifact value is sufficient; no generalized cache platform
   or persistent preparation/dispatch record. Shared parent-handle reuse is a
   separate high-reach ingredient, justified by measured acquisition cost, to add
   only with its direct caller/lifetime tests. Do not blindly land the research patch.
3. **Measure before claiming the refactor's gain.** At fixed native entry sizes
   (8/16/31 and matched 25–30 controls), measure preparation, invocation and actual
   paired workflows with repeated effects in independently built roots; add both
   explicit and kind inputs. Record per-phase scan/inventory/manifest counts;
   native current/capture/grant/scope calls and inclusive/exclusive spans; path
   listings/entries, canonical bytes/calls, CPU/wall, lock wait and handle release.
   Compare pristine and instrumented binaries. Test independently growing kind,
   proof, lineage and near-bound bytes. Do not sum separate medians or infer p95
   from six size-varying samples. Suggested comparative acceptance: the selected
   reduced path reaches explicit **zero** preparation work scans / kind **one**,
   avoids repeated exact derived artifacts, preserves fresh observations, and
   improves paired time without moving hidden work outside timers. Latency targets
   require calibration and human acceptance; none is invented here.
4. **Return before capacity implementation.** Even a successful refactor leaves
   I1–I4 and unrelated-history reads. Use measured remaining cost to prepare the
   separate native indexed-file/detection/publication/admin-reserve outcome after
   its human choices. Do not advertise sustained-use capacity from this increment.

Acceptance examples for the bounded refactor:

- Bind an explicit ordinary item, save original locator, invoke, reopen and obtain
  the same `open` snapshot and exact receipt bytes. A kind-bound equivalent keeps
  exact K1 provenance and effective values even after K2 establishment.
- Bind, then narrow scope/revoke/accept pending administration/D→G before invoking:
  no fresh effect under old authority. Under reduced preparation these may be
  detected only at invocation. A committed original still replays after those
  events, preserving its original outcome and restoring no permission.
- Two independently bound originals target the same item: one winner or truthful
  original uncertainty/reservation, never two `open` creations or cached absence.
  Different bytes under an original request key always conflict.
- Modify an intent, terminal, exact kind/approval/grant dependency, unknown sibling,
  manifest slot or native/root/lock while reuse is active: preserve existing
  refusal/uncertainty and observation timing where the phase contract is unchanged.
  Test regular in-place writes and substitution, not only directory rename.
- Complete-record oversize still refuses before reservation, including derived
  terminal/noncommit/checkpoint bytes. Reuse does not allow changed UTF-8 bounds,
  wildcards, extra roles or grant scope expansion.
- Interrupt each existing acceptance/publication stage and restart with saved
  original. Early candidate uncertainty remains truthful; supported never-attempted
  original recovery consumes no new slot; attempted uncertainty is not retried as
  absent; exact terminal after lost reply replays. Combined locator-persistence
  callback failure, if selected, produces no possible work acceptance.
- Count resources returning to baseline, including errors/recovery and callback
  refusal. Keep authority-before-work lock ordering and exercise registration
  against revocation/administration under contention. Do not hold locks across
  caller delay. Process death checks are not hardware power-loss certification.

## Verification and review disposition

This audit made no experimental or production Rust edits. Additional native
timing/prototypes were unnecessary: retained measurements identify expensive
categories, and source establishes the repetitions/lifetimes and missing reuse
predicates. New safe-deletion speed claims would require the bounded experiments
listed above; none is asserted here.

Checks performed for this report:

- Verified branch/HEAD, exact-path current GitNexus index and source equality to
  `8013948` for `crates/`, `Cargo.toml` and `Cargo.lock`.
- Ran the reviewed, tracked `check_evidence.py` against the retained evidence:
  **passed 641 observations / 199 groups / 84 current-read control groups**;
  deterministic summary and run/patch hash consistency reconcile.
- Independently recomputed the three variant paired medians directly from
  `effects.jsonl`, diagnostic span counts/times and retained test log totals:
  **74 passed, zero failed, five ignored**. This recount is **not a test rerun**.
  Three ignored entries are parent-invoked helpers; two unavailable prior
  physical-root compatibility cohorts remain unverified.
- `git apply --check` for the preserved experiment patch succeeds without applying
  it. Existing untracked Pro research and six generated skill directories remain
  outside this commit; no cleanup or sweep was performed.
- GitNexus query/context and proposed-change upstream impact were inspected.
  `parsed`: **CRITICAL**, two direct callers, 45 symbols at depths 1–3,
  121 graph-reported processes. `with_work_registration_establishment`:
  **CRITICAL**, one direct caller (`execute`), 24 symbols, 120 reported processes.
  Warnings were reported. Transitive graph reach is overbroad and not a validated
  list of actual affected product flows. An initial incorrect UID returned UNKNOWN;
  exact context resolved the preparation UID. Correct-UID preparation impact:
  **CRITICAL**, two direct callers, 821 depth-1–3 symbols and 134 graph-reported
  processes; its broad transitive reach has the same graph limitations. No symbol
  was edited.
- Local Markdown target checks passed for **220 targets** across the three changed
  documents; the selected semantic ownership anchor also resolves. Diff hygiene
  and exact three-file staged scope passed. GitNexus staged `detect_changes`
  reports **low risk, seven documentation sections, zero affected processes**.

One focused independent causal reviewer returned **APPROVE**, with no P1 Critical
or P2 Required findings and no P3 Optional/Consider findings. One P4 Nit identified
two drifting source-line references; both are corrected without material changes
or another review round. The reviewer independently reconciled paired medians,
diagnostic spans/read/list counts, 74/0/5 log totals, source equality and patch
applicability, and checked phase/guard/currentness, exact kind binding and early
recovery uncertainty against source. No expensive unchanged suite was rerun.
Research/preparation is complete; human review is next. This verdict does not
approve the preparation semantic change, production optimization, a backend,
capacity or successor planning/implementation.
