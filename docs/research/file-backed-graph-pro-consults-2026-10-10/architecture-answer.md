# Architecture consultation: captured answer

Date: 2026-10-10. Model shown by UI: GPT-6, Pro effort.
Task: independent review of file-backed work-state graph and performance directions.
Conversation: https://chatgpt.com/c/6aca7955-56fc-83ea-82bd-67fafb5b1307
Current approach and exact prompt: [architecture-prompt.md](architecture-prompt.md).
Capture: completed assistant response, copied through its rendered response control. Claims below are the consultant's findings; this capture is not independent local validation or implementation approval.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

---

# Review file-backed work-state graph direction

## Recommendation: **ADJUST**

**Keep the file-based direction and operation-specific verification. Make the next candidate an indexed version of the existing work-state evidence model—not a new generic graph platform.** First remove demonstrably repeated work in the native implementation; then test a coherent, generation-bound index and publication design over individual transaction files. Keep segmentation as a physical-layout option and SQLite as a conditional challenger.

The evidence supports three conclusions:

1. **The present latency is substantially a validation and path-guard amplification problem.** It is not evidence that ordinary files are intrinsically inadequate.
2. **The present capacity problem does require more than local optimization.** Fixed lifetime slots, whole-history inventories, global checkpoints, and authority administration sharing ordinary capacity are architectural constraints.
3. **The proposed graph is useful primarily as an explicit dependency model and query plan.** Much of that graph already exists in typed records. Making it navigable is justified; inventing another identity system, generic edge store, or graph database is not.

The file-backed approach is therefore the strongest **next investigation**, not yet a demonstrated scalable implementation. Conversely, the SQLite prototypes establish a genuine advantage for their tested storage operations, but they do not establish native semantic equivalence or backend necessity.

---

## 1. What I inspected—and what this review does not establish

ZIP access succeeded. I read `bundle-manifest.json`, confirmed the empty `worktree.patch`, and verified the manifest’s **1,386 eligible tracked files and 24,055,137 source bytes** against their recorded sizes and hashes. The supplied repository identifies HEAD as `8013948ebd3fbef6725f5d72f3d29120f7617009`. I did not infer content from the 390 excluded entries or unavailable untracked files.

I inspected both research reports and their evidence: the initial raw observations and summary; native profiling records, summary, instrumentation and summarizer; prototype implementations, comparison results, current-pointer negative experiment, and fault results. I traced the Engine work-state store, transaction, type, kind, scope and submitted-form paths; the called work-registration authority seam, operating guard, local-check currentness and governance-inventory checks; and relevant retained-root, file, tree, checkpoint and publication primitives.

I also inspected SDK forwarding and preparation/recovery behavior, representative integrity and durability assertions, the requested specification sections, and adjacent registry, Contracts evidence and lifecycle-storage mechanisms for possible reuse.

**I did not build or execute the Rust workspace, rerun the Windows benchmarks, or reproduce the platform fault tests.** The performance findings below distinguish supplied measurements, independently inspected code behavior, and proposed changes. Inspection of test assertions is not a new passing test result.

Two evidence limitations materially affect the decision:

- The initial measurements and deeper profiles are useful but small, particularly individual native effects and audit/reopen samples. The native profiler measures instrumented spans and process CPU, **not physical disk activity**.
- The 10,000/100,000 prototype populations are synthetic envelopes sharing one actual original and one native proof. They are not equivalent populations of native work items, grants, kind approvals and changing authority epochs.  
  **Sources:** the two `docs/research/*investigation-2026-10-10.md` reports; deeper feasibility report §§ native instrumentation, prototype method and limitations; matching raw JSON/JSONL evidence.

The navigation documents also explicitly leave storage adoption and implementation unselected. Your latest direction supplies the controlling file-first/access-and-audit preference; older conditional recommendations do not override it.  
**Sources:** `coverage-and-landing-map.md:325–380`; `phase-and-slice-map.md:402–445`.

---

## 2. What is actually making the native implementation expensive?

### 2.1 The measured problem is repeated guarded traversal, not demonstrated slow storage media

The deeper evidence contains this representative comparison:

| Measured native operation | Current item, history 32 | Fresh registration effect, history 31 |
|---|---:|---:|
| Total wall time | 1,353 ms | 7,516 ms |
| Process CPU | 1,328 ms | 7,359 ms |
| Full `scan` calls | 1 | 4 |
| Inclusive time in those scans | 1,341 ms | 5,334 ms |
| Windows handle-information helper calls | 36,586 | 226,498 |
| Directory-guard opens | 9,656 | 65,491 |
| Directory listings | 758 | 5,527 |
| Canonical JSON operations | 1,166 | 6,398 |

These are the reported group values in `native-summary.json`, backed by `native-profile.jsonl`. The fresh effect is one observation and excludes preparation.

The four scans account for approximately **71% of that effect’s wall time**. That does not mean deleting three calls safely saves three quarters of the time: their roles differ, their underlying work overlaps other categories, and the replacement still needs verification.

Likewise, inclusive scan time cannot be added to the canonicalization or handle-check times nested inside it. The remaining approximately 2.18 seconds is not an irreducible storage floor.

**Interpretation:** warm execution spends nearly its elapsed time in process CPU, including filesystem-related kernel work. That supports investigating repeated computation and metadata operations first. It does not establish that physical I/O is absent, nor that a different storage layout cannot help.  
**Sources:** deeper report:131–176; `profile_metrics.rs`; `summarize_native.py`; `native-summary.json`.

### 2.2 Required-file reads repeatedly enumerate their ancestors

This is the clearest lower-level causal problem.

The work store’s `required`/`optional` path reaches:

```text
store::required / optional
  → read_local_owner_optional_bounded
    → list each ancestor directory to establish parent presence/type
    → read_regular_bounded
```

`read_local_owner_optional_bounded` lists each parent beneath `.handbook` before opening the requested file. For transaction records, that repeatedly includes directories containing other transactions.

On Windows, `platform::list_directory_bounded` additionally inspects children, opening regular-file handles and checking their properties. The operation therefore performs substantially more work than “open this known record and validate its bytes.”

**Sources:** `crates/engine/src/work_state/store.rs:692–714`; `crates/engine/src/repository_durability.rs:557–581,2395–2451`.

Two consequences follow.

**First, direct indexes alone will not cure this.** An index that returns an exact path, followed by the same ancestor-enumeration helper, can retain unnecessary sibling-dependent cost.

**Second, the path can amplify history growth.** Repeated reads across transactions repeatedly inspect directories whose populations grow with history. This is a structural amplification mechanism, not a claim that the small existing dataset proves a particular asymptotic timing curve.

**Change to investigate first:** a narrow read path for an already established work-owner operation, using held and validated parent bindings rather than rediscovering each parent through directory enumeration. Keep the existing optional-parent/bootstrap distinctions where they are genuinely needed. Do not globally weaken the shared durability helper.

A related measurement correction: `windows_handle_info` is a helper that performs three information queries on its successful path. Its invocation count is not a physical-I/O count or a count of single OS calls.  
**Source:** `repository_durability.rs:1635–1736`.

### 2.3 Fresh execution repeats whole-history work across distinct phases

For the measured fresh effect, the source explains the four scans:

```text
execute
  → resolve(original request)                 [scan 1]
  → acquire current native authority + work lock
      → scan                                 [scan 2]
      → reserve and stage original intent
      → write decision/snapshot/candidate terminal
      → scan                                 [scan 3]
      → record publication attempt
      → publish terminal
      → scan                                 [scan 4]
```

Reservation adds more traversal: `reserve_intent` calls `inventory()` and `manifest()` again, even though execution already obtained inventory information.

Preparation has its own duplication: `prepare_registration` first calls `resolve_current_terms`, then reopens the work owner and scans again for registration-specific checks. SDK basis observation is additional work.

**Sources:** `work_state/transaction.rs:981–1035,1225–1576`; `work_state/store.rs:438–459`; `crates/sdk/src/work_state.rs:118–149`.

**Important safety boundary:** scan 1 is not just redundant permission checking. It allows an already committed original request to replay **before requiring current permission**. Removing that behavior could incorrectly make historical recovery fail after revocation or governance change. That preliminary resolution also occurs in a different lock lifetime.

Inside the later held operation, however, repeatedly reconstructing unchanged historical semantics is a strong optimization candidate. A validated immutable evidence set plus an explicitly tracked set of changes made by the current operation can replace some repeated reconstruction—provided publication-time physical and currentness checks remain.

Preparation can similarly be consolidated within its own held session. It cannot become a permission cache for a later invocation.

### 2.4 Shared-proof validation is already deduplicated—but much surrounding work is not

`parsed` computes a fingerprint for the embedded authority proof and uses a set to call `validate_historical` once per distinct proof **within each scan**.

Thus “validate the common native proof only once” is partly implemented already. The current read at 32 has one historical-proof validation, not 32.

However:

- Each intent is still decoded with its embedded proof.
- Computing the proof’s deduplication key canonicalizes that proof again.
- Each scan starts a new deduplication set.
- `parsed` eagerly derives expected decision, output, terminal, publication and noncommit representations, including some whose corresponding files are absent.
- `WorkLock::validate` repeatedly encodes immutable owner/bootstrap values and rereads their files.

**Sources:** `work_state/transaction.rs:233–319,333–337`; `work_state/store.rs:178–196`; `work_state/mod.rs`, `encode` and `fp`.

Safe initial candidates are operation-local reuse of canonical bytes and derived expected records, computing each distinct expected representation once, and separating pure semantic validation from physical revalidation.

**Physical proof deduplication is a separate change.** Replacing an inline proof with a reference changes a record shape and potentially its fingerprint preimage. It must not be presented as transparent serialization cleanup.

### 2.5 There are latent quadratic semantic lookups

`scan` performs pairwise duplicate-request checks, searches the entire row set for required committed grants and kind receipts, and performs pairwise kind-version conflict checks.

`validate_authority_history` repeatedly searches the grant population while following the lineage.

**Sources:** `work_state/transaction.rs:341–501`.

Temporary maps keyed by exact request identity, grant fingerprint, kind binding and predecessor can remove repeated searches **without changing the stored format or integrity policy**. These maps must detect duplicate/conflicting insertions, not overwrite them.

This is worth doing before selecting a backend. But the profile does not support calling these searches today’s dominant cost: directory/guard work and canonicalization are better evidenced.

### 2.6 Native currentness remains a separate potentially expensive dependency

The work authority seam repeatedly validates its retained tree, operating guard and current local-check state. Local-check currentness reaches `require_basis`, which recomputes the complete admission-governance inventory fingerprint.

Captured historical proofs also expressly bind that complete governance population.

**Sources:** `local_authority/work_registration.rs:41–119,125–189,501–584`; `local_authority/local_check.rs:831–989`.

Therefore, **a work-state graph does not automatically make native authority verification cheap**. Some whole-population work belongs to the currently defined native basis, not merely to work-state’s accidental history scanning.

Measure that cost separately. Reuse an unchanged, held basis where its existing contract permits it; do not replace it with “the selected current file looked unchanged” without an owner-level justification.

### Safe versus unsafe reuse

| Reuse | Assessment |
|---|---|
| Canonical bytes and pure validation of the same exact immutable content and predicate | Good first candidate. |
| Parsed history across phases within one held operation | Conditional on retained bindings and correct accounting for that operation’s own mutations. |
| Current grant/current native authority across released locks | Unsafe without fresh generation/currentness validation. |
| Preparation result as later commit permission | Unsafe; the source expressly rejects this. |
| File identity, size or modification time as sufficient content proof | Insufficient for the current supported mutation checks. |
| “Immutable by convention” as a reason to stop physical validation | Unsafe. |

The Windows regular-file read path permits sharing that includes writes and deletion. Holding such a handle does not itself make its bytes immutable. Microsoft documents sharing modes as permissions for other opens; the repository separately tests in-place mutation detection.  
**Sources:** `repository_durability.rs:2370–2393,2845–2860`; Microsoft `CreateFileW`. [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)

**Overall diagnosis:** both validation design and storage architecture matter. Validation/path-guard amplification is the principal **measured immediate problem**. Indexed publication, bounded lookup and capacity accounting are the necessary **scaling work**.

---

## 3. The graph direction: useful, but only with explicit authority and completeness

### 3.1 Much of the graph already exists

The source already contains typed relationships:

- Item requests bind an original grant and, when applicable, an exact kind establishment receipt and expansion.
- Grants bind an exact predecessor grant fingerprint.
- Intents bind captured authority.
- Decisions bind intents; terminals bind decisions.
- Positive slots bind expected transaction names and exact intent-byte fingerprints.

`scan` currently discovers and verifies these relationships by reading the whole population.

**Sources:** `work_state/types.rs:194–265,335–431`; `work_state/kind.rs:206–335`; `work_state/transaction.rs:98–121,352–367`; `work_state/store.rs`, `SlotEntry` and `manifest`.

The minimum useful “graph” is therefore:

> **Typed dependency extraction and direct lookup for existing owner records, with operation-specific verification over a coherent published state.**

That needs neither a universal node/edge schema nor a graph query language. Existing domain records can remain the authoritative representation.

Also, current items have immutable initial `open` state; arbitrary item revisions, relations and closing effects are not delivered merely because grants and kinds are versioned. Do not turn storage work into implementing those future product semantics.  
**Sources:** compositions/kinds `README.md:51–76`; `packet-1.md:137–164`.

### 3.2 Keep four decisions separate

| Decision | What it answers |
|---|---|
| **Dependency representation** | Which exact facts does this operation require, and what proposition does each dependency establish? |
| **Physical layout** | Are records individual files, packed frames, segments or database rows? |
| **Indexes and publication** | How are exact objects, current selections, reservations and complete populations located coherently? |
| **Integrity policy** | Which damage is checked now, which is deferred to access/audit, and what interference is detected while the operation is held? |

Content addressing is an additional mechanism, not a substitute for any of these decisions.

Git’s documented pack/index formats are a useful existence example of keeping object identity separate from packed physical location. That is not a recommendation to adopt Git’s storage, delta chains or lifecycle for Handbook. [Git](https://git-scm.com/docs/gitformat-pack)

### 3.3 Define closure by the operation—not by every reference-shaped field

A concrete operation matrix should precede implementation:

| Operation | Required historical evidence | Additional current/completeness obligations |
|---|---|---|
| Resolve/replay an original request | Exact accepted original, required original grant/kind/native proof, matching terminal or supported incomplete prefix | Exact request membership/status; no live permission requirement merely to replay. |
| Read an existing item or exact kind | Its committed establishment/registration closure | Correct selected binding and reservation/terminal status; not current registration permission. |
| Register a fresh item | Original grant; selected committed kind and exact expansion when applicable | Current native basis, usable grant, no suspending administration/revocation, complete relevant request/item conflict checks. |
| Establish a kind version | Exact definition and separate per-version human approval | Current native basis and complete version/request reservation checks; no invented registration-grant prerequisite. |
| Change scope, activate D→G, revoke | Exact original lineage and operation approval | Expected predecessor, pending-administration exclusion, correct transition rule, current native basis and publication coherence. |

This separation is not speculative product behavior. The SDK documents historical reads independently of permission; Packet 2 separately establishes kind authority and specifies recovery without live permission for committed originals.  
**Sources:** SDK `work_state.rs:102–177`; kinds `packet-2.md:151–232`; `transaction.rs:558–719`.

A particularly important boundary: a requirements-input definition locator is currently a **declaration**, not a claim that Engine resolved the Contracts source, qualified it or assessed it. A generic “follow every link” graph walker must not introduce those dependencies or an Engine→Contracts ownership dependency.  
**Sources:** `work_state/scope.rs:9–71`; `packet-1.md:152–164`.

### 3.4 Authoritative and rebuildable indexes are different roles—not mutually exclusive labels

The design needs at least three categories.

**Object-location indexes** map an exact object identity to a file or segment location. They may be rebuildable accelerators, provided the located object is checked against the expected identity and bytes. A bad location must not silently select another version.

**Current-state indexes** identify selected item/request state, the grant lineage’s usable head, pending administration, and revocation. These are decision-bearing. They cannot be accepted merely because their JSON is well formed or their local checksum matches.

**Membership/reservation indexes** establish that a request or item/version is already reserved—or that a proposed key is genuinely absent in the relevant namespace. These are essential to preventing lost transactions from becoming fresh absence.

The key rule is:

> **A negative lookup is valid only against a complete, validated index for the operation’s selected published generation. Missing files or stale caches are not absence proofs.**

“Rebuildable” does not mean “non-authoritative.” An index may govern normal lookup and still be reconstructible from a positively complete publication history. During reconstruction it is not usable authority.

Conversely, a missing authoritative publication head cannot safely be recreated by listing surviving files and choosing the newest-looking record. Existing tests deliberately reject lost owners, locks, slots and whole transactions becoming fresh absence.  
**Sources:** `store.rs:197–432`; `work_state_integrity.rs:90–161,222–367`.

A flat append index loaded into a complete in-memory map is one implementation, not the inevitable file-based design. Bounded pages or buckets are candidates. But their routing, split/update and completeness rules must be specified and tested: they are not free complexity hidden behind the word “index.”

### 3.5 Exact versions do not establish currentness

An immutable original grant can be historically valid while unusable now. A committed kind remains valid after a work-grant revocation, but does not supply permission to register another item.

The current grant check explicitly treats non-noncommitted successor or revocation administration as blocking use. A design containing only `latest_committed_grant` would lose pending-administration suspension.

Likewise, a pending kind establishment is **not** registration-head administration and must not globally suspend unrelated permitted work.

**Sources:** `transaction.rs:502–621`; kinds `packet-2.md:171–179`; `work_state_durability.rs:886–966`.

The authoritative control state must therefore represent pending obligations and irreversible revocation, not just final snapshots.

### 3.6 “Check every shared node once” needs a precise qualification

Use operation-local memoization keyed by exact owner/domain/schema/content and the relevant validation predicate. Distinguish:

- valid canonical object;
- valid historical authority proof;
- actual committed establishment;
- presently usable permission.

Those are not interchangeable cached facts.

**Check pure semantics once; preserve physical/currentness checks at the necessary boundaries.** Rehashing or checking a retained binding at publication is not the same work as re-decoding and re-deriving all historical meaning.

Also, closure verification is **O(required closure), not automatically O(1)**. Long grant lineages and many distinct native proof populations can remain expensive. The present two-proof “distinct” cohort does not characterize thousands of authority epochs.

Do not make every transaction’s append-log predecessor a semantic dependency of every item. That would recreate whole-history traversal inside a graph. Publication ancestry and semantic grant ancestry have different roles. Any checkpoint that summarizes the latter must have explicit authority and validation semantics; it cannot simply declare old dependencies unnecessary.

### 3.7 Publication is the hardest part—not traversal

A suitable investigation should model a single coherent generation containing the relevant index roots and control state, published over immutable evidence. The exact format remains unselected.

The required sequence is conceptually:

1. Acquire native authority and work locks in the existing order.
2. Resolve the original request first; preserve historical replay and uncertainty semantics.
3. Validate the selected generation, dependency closure, current permissions and relevant absences.
4. Durably establish acceptance/reservation and its exact original evidence.
5. Stage the effect and its index/control-state changes.
6. Revalidate selected dependencies/currentness and publish a coherent terminal state.
7. Return a truthful result; recover interrupted work by its original identity.

**Acceptance and terminal publication remain distinct durable facts.** A single final database transaction, or a single final manifest replacement, must not erase previously accepted original-request obligations.

Nor does “atomic rename” make several files an atomic transaction. The design must enumerate failures between object writes, flushes, acceptance publication, terminal publication, index updates and reply. SQLite’s own atomic-commit documentation illustrates why ordered durability steps and filesystem assumptions matter; file flushing alone is not a multi-object commit protocol. [SQLite](https://sqlite.org/atomiccommit.html)

For files, incomplete publication must have an original-request recovery route. The prototypes’ “detect orphan and refuse future writes” behavior is fail-closed, but not a complete usable recovery design.

### 3.8 Bound the guarantee honestly

A checksum over `(target, sequence)` detects some accidental field damage. It does not prove that the sequence is the latest authorized state.

Binding selection to an independently retained generation/index commitment can detect inconsistent pointer changes. It still does not create a generic guarantee against an attacker coherently rewriting or rolling back the entire local authority population between opens.

That limitation applies to files, segments and SQLite. Do not introduce external authentication, signing infrastructure or rollback-resistant hardware unless a separate threat-model decision requires it.

Under your selected policy, missing or corrupt **selected dependencies** must fail their dependent operation; damage to a shared authoritative control structure may correctly block many operations. Unrelated historical body damage need not block an otherwise independent operation. An explicit audit must check the complete population and its consistency.

### 3.9 Reuse existing facilities—but not their scalability limitations

The strongest reuse candidates are:

- Engine’s duplicate-safe parsing, canonicalization, exact definition references and fingerprints;
- `RetainedRepositoryRootV1`, physical lock bindings, retained checkpoints and durable publication primitives;
- the lifetime-bound `HeldWorkRegistrationCurrentV1` callback pattern;
- existing typed locators, receipts and publication-boundary tests.

**Sources:** `definition_identity.rs:134–267,325–352`; `repository_durability.rs`; `local_authority/work_registration.rs:25–189`.

Adjacent code supplies precedents, not a ready-made scalable backend. The artifact registry has typed dependency maps and cycle/order handling. Contracts evidence storage has exact fingerprinted records, ledgers and snapshot-rooted paging. But its read/recovery paths still perform inventory work; its lifecycle head validation reconstructs history and compares the persisted head.

**Sources:** `artifact_registry.rs:17–79,588–636`; Contracts `evidence_repository.rs:268–344,446–469,699–755`; `full_definition_repository.rs:687–725`.

Reuse those ideas without importing generic Artifact Kind semantics into work kinds or claiming that the repository already contains a proven scalable graph store.

---

## 4. Fair comparison under the same semantics

The comparison must use the same operation predicates, currentness checks, accepted-original recovery, publication guarantees and access/audit policy.

| Candidate | Main strength | Main risk | Present assessment |
|---|---|---|---|
| **Optimized current files, same whole-history policy** | Smallest change; isolates guard, canonicalization and algorithmic waste | Still history-dependent; fixed manifests/checkpoints remain | Necessary control and worthwhile immediate work, not the final scaling solution. |
| **Indexed individual files with typed closure verification** | Closest to existing evidence ownership; direct inspection; selected dependencies can be isolated | Coherent indexes, negative proofs and interrupted publication must be engineered | **Preferred first native candidate.** |
| **Segments with the same logical indexes and verifier** | Fewer files; sequential audit and possible storage locality benefits | Active-tail recovery, offsets, compaction and verification granularity | Keep available; select when measured benefit justifies complexity. |
| **SQLite with the same verifier and native authority integration** | Transactional updates to related indexes/state; mature paged storage | Native binding/VFS integration and application-level integrity remain necessary | Strong conditional alternative, not justified for adoption yet. |

“Graph-indexed files” and option A are not fundamentally competing storage families. An improved A can implement the proposed graph relationships. B can pack the same logical objects later.

### What the prototypes establish

For the supplied 100,000-envelope cohorts:

| Storage measurement | A: individual files | B: segments | C: checksum variant |
|---|---:|---:|---:|
| Warm selected lookup | 6.312 ms | 4.532 ms | 0.884 ms |
| Durable acceptance transition | 15.291 ms | 13.066 ms | 1.037 ms |
| Durable terminal transition | 15.848 ms | 13.571 ms | 1.266 ms |
| Reopen plus selected lookup, warm OS cache | 690.031 ms | 719.602 ms | 3.044 ms |

These are reported medians, with 40 selected reads, 16 samples per mutation class and three reopen samples. They are **storage transitions, not complete native registration workflows**.

C’s advantage should not be dismissed: it was not merely tested without durable commits. The prototypes explicitly use WAL/FULL for C and durable data/head publication for A/B. SQLite documents WAL’s transaction, synchronization and checkpoint behavior; it still has one writer at a time. [SQLite](https://www.sqlite.org/wal.html)

But several comparisons remain confounded:

**Reopen and memory.** A/B reconstruct large in-memory maps; C uses paged database access. That is a genuine implementation difference, not a universal filesystem limitation.

**Audit.** B’s follow-up streamed audit improves the 100,000-envelope result from about 8.81 seconds to 4.12 seconds. A’s first audit took approximately **322 seconds**, versus roughly 10.4–10.6 seconds subsequently. That outlier demands investigation; neither the median nor an unsupported explanation such as antivirus should conceal it.

**Recovery.** A/B lack full original orphan/tail recovery. C’s SQL rollback behavior is a real benefit, but the comparison does not yet demonstrate all native accepted-original semantics.

**Currentness.** Baseline C returned an older accepted state after current-pointer corruption. Its checksum variant detects pointer-only alteration but still accepts coordinated pointer-plus-local-checksum rewriting. A/B validate their loaded mapping against a head-bound index chain—a different amount and kind of work.

**Sources:** `prototype-*-100000*.json`; `prototype-current-pointer-negative.json`; `prototype-faults-followup.json`; `prototypes.py:187–331,486–618,646–747`.

C needs a semantically adequate selected-current validation route before its latency is an equal-policy comparison. That might use indexed target history, stronger generation binding or another justified mechanism—not necessarily the same internal format as files.

### Segments have their own hidden verification trap

Packing 1,000 records into a segment helps little if every point read then hashes and validates the entire large segment. Conversely, checking only a frame without its proper publication/membership binding can weaken verification.

The prototype’s streamed audit also retains opened frame files and shared object bytes through its audit context. It is not proof that large native audits have bounded handle or proof-memory usage.

**Source:** `prototypes.py:224–305`.

Segment size, immutable/active-tail separation, frame checks, index membership and physical guard granularity must be tested together.

### What would justify reconsidering SQLite?

A reasonable reconsideration gate is:

**A sound native file-based candidate has been tested and still misses an agreed workload/resource objective; segmentation or simpler file improvements do not solve that economically; and SQLite demonstrates the same required semantics with working native integration.**

That could arise from reopen costs, file-count behavior, index-update complexity, contention, bounded memory or maintenance burden. SQLite need not prove files mathematically impossible; it would need to show a material practical advantage under the user’s actual requirements.

The integration proof must cover the database and relevant journal/WAL/shared-memory files, lock order, physical root/owner binding and through-publication authority. SQLite’s filesystem operations run through its VFS; ordinary application-level checks are not automatically a retained-handle integration. A custom VFS may or may not be necessary, but the actual compatible mechanism must be demonstrated. [SQLite](https://www.sqlite.org/vfs.html)

If both designs meet the agreed objectives, a synthetic 1 ms versus 15 ms storage result alone is not sufficient reason to override your file-based preference.

---

## 5. Minimal next work, validation and unresolved choices

### Stage 1: Isolate inexpensive native improvements

Keep the existing format and detection policy for this control experiment.

Concentrate on held-parent reads, repeated owner encoding, expected-record construction, temporary conflict/dependency maps, and safe operation-local reuse. Consolidate preparation where possible without carrying permission into later invocation.

Add inexpensive counters for:

- directory paths and child entries examined;
- actual bytes read, hashed and canonicalized, versus distinct required objects;
- validation work by phase and exact proof identity;
- lock wait versus time holding locks;
- flush count/time, peak handles and retained memory.

Measure single changes as well as the combined result. Include enabled/disabled instrumentation controls. This separates “files are expensive” from “this file-reading route performs the same work repeatedly.”

**Deliverable:** a small native experiment with unchanged retained receipts and fault behavior, identifying measured savings—not a cap-only release.

### Stage 2: Specify the operation/index/publication contract

Produce a bounded design note and independent fixture oracle for the existing operations.

It should fix the operation matrix, authoritative generation, complete request/item/kind reservation rules, grant/pending/revocation control state, dependency-validation boundaries and every interrupted-publication outcome.

This is also where the selected detection policy must be reconciled with current documents and tests. Existing Packet 2 language requires complete owner/inventory checks, and current tests intentionally reject unrelated unknown history entries. Adopting access/audit verification is not achieved by silently deleting those assertions.

Replace their broad operational requirement with precise selected-dependency and authoritative-population requirements, while retaining whole-population audit coverage.

**Sources:** kinds `packet-2.md:187–240`; `work_state_durability.rs:447–490`; integrity tests for missing slots and whole transactions.

Keep actual retained identities and bytes. Item identity explicitly depends on project and item key—not content, codec, descriptor or release. New reference-bearing records require purpose-specific versioning and deliberate identity treatment.  
**Sources:** `types.rs:371–438`; `packet-1.md:166–185`; `packet-2.md:361–372`.

### Stage 3: Build one native indexed-file vertical experiment

Implement the smallest meaningful complete route in an isolated investigation: exact lookup, a fresh kind-backed registration, pending/terminal recovery, and competing authority administration.

Use existing typed semantics. Exercise the authoritative indexes and publication protocol. Do not start by extracting a generic graph crate.

Test at 32, then approximately 1,000 and 10,000 **native accepted transactions**, recording their actual composition and bytes. Extend to 100,000 native transactions only when the preceding experiment is correct and its resource behavior understood.

Compare against Stage 1 so that savings from changed integrity policy are not mislabeled as physical-layout gains.

### Stage 4: Test packing only against an identified bottleneck

Keep the same logical dependency model and verifier. Change physical layout to bounded segments where file count, audit cost or locality actually matters.

Only commission a native SQLite integration comparison when it answers a remaining decision. The existing C results already establish that a database storage path can be fast; another synthetic benchmark is less useful than proving or disproving the integration gap.

### Workloads that distinguish the designs

The most useful scaling experiment is not simply “increase N.”

**Hold the selected dependency closure fixed while increasing unrelated history.** Point-operation reads, hashes and canonicalization should not grow proportionally with unrelated transaction bodies. Index traversal may grow with index structure.

Then independently increase kind diversity, grant-lineage length, distinct native authority proofs and selected closure bytes. That exposes genuine dependency cost instead of hiding it behind one shared proof.

Include explicit and kind-backed work, all three delivered compositions, same-basis scope successors, D→G, pending administration, revocation, original replay and recovery. Measure single-process use, fresh-process reopen and competing processes. Report preparation, invocation and their combined workflow separately.

Warm tests and fresh-process tests are different; neither establishes cold physical-cache performance. Randomize benchmark order and repeat on separate datasets rather than treating a small `p95` field as a production tail-latency guarantee.

There is no supplied production SLA. As an initial product discussion, **100 ms p95 for warm point reads and 500 ms p95 for complete ordinary preparation-plus-registration at 10,000 native transactions** would be useful candidate targets—not promises or existing acceptance criteria. Startup, contention, maximum dependency closure and audit need separately selected budgets.

### Correctness and failure validation

The minimum test expansion should cover five classes:

| Class | Required evidence |
|---|---|
| **Selected integrity** | Missing/corrupt original grant, kind, intent, terminal or native proof refuses the dependent operation; no fallback to a newer object or explicit composition. |
| **Completeness and currentness** | Deleted reservations do not become absence; stale grant/current pointers do not bypass pending administration or revocation; noncommit releases only its permitted reservation, never the original request key. |
| **Policy distinction** | Unrelated historical damage leaves independent point operations usable under the selected policy, while audit detects it; shared control damage blocks the genuinely dependent population. |
| **Publication and recovery** | Faults at every durable boundary, lost reply, duplicate creators and original-key retries preserve committed/noncommitted/uncertain distinctions. Include injected write/flush failures as well as process death. |
| **Physical/concurrent binding** | Root/lock replacement, directory substitution, in-place file changes, concurrent scope changes and D→G cannot invalidate an admitted effect unnoticed through publication. |

Existing tests provide a strong starting oracle: the process serialization and process-death tests, pending-administration tests, and kind-dependency substitution tests exercise precisely these distinctions.  
**Sources:** `work_state_durability.rs:515–580,679–720,886–966`; `work_kind_integrity.rs:324–377`; `repository_durability.rs:2790–2860`.

Passing process-death tests still must not be reported as demonstrated hardware power-loss durability.

### Capacity and revocation need an explicit promise

Replace the shared tiny lifetime ceiling with measured resource bounds and protected administration/accepted-request closure capacity.

But separate **logical reserved capacity** from actual physical disk availability. An accounting reserve does not guarantee a successful write when the filesystem cannot provide space or flushing fails.

The design should preserve accepted-original closure without requiring a fresh ordinary slot, protect necessary administration from ordinary saturation, and define refusal/maintenance behavior when administrative resources are themselves exhausted. It must never claim unlimited revocation availability on a failed disk.

**Sources:** `store.rs:12–15,438–535`; `transaction.rs:1423–1455`; `work_state_durability.rs:1104` onward; coverage map:337–348.

### Human choices that remain

Your file-first and access/audit direction is already clear. The remaining choices are the native workload and latency/resource envelope; protected administration/closure guarantees; audit invocation and behavior after unrelated corruption is discovered; and the precise new index/publication format.

Actual retained evidence must remain verifiable, but that does not imply permanent legacy public APIs or a mixed-binary writer guarantee. General format defaults remain proposed; Contracts JSON and existing identity contracts remain controlling.

**Do not build now:** a graph database, universal graph schema/query language, automatic garbage collector, compaction framework before a measured need, external tamper-proofing infrastructure, new authentication platform, or future hierarchy/eligibility/closing semantics.

---

## Bottom line

**The right adjustment is to separate efficient verification from storage selection.**

Start by making native file access and validation stop repeating avoidable work. Then replace global history reconstruction with **complete, operation-specific evidence verification supported by coherent authoritative indexes and publication**. Implement that first over the existing file-oriented model, treating its typed relationships as the graph.

The largest risk is not choosing files instead of SQLite. It is building a fast positive-object lookup while leaving currentness, reservations, negative proofs and interrupted publication underspecified. The opposite risk is retaining whole-history checks merely because they currently enforce those facts indirectly.

A successful next result is therefore not “100,000 graph nodes are fast.” It is:

> **A native operation verifies its exact required evidence and present authority, makes sound membership/absence decisions, publishes and recovers truthfully, and does substantially no work proportional to unrelated history—while preserving the file-based system you prefer.**
