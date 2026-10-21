# CP4 independent source review: freeze 924bf794

**Bounded engineering disposition: REMEDIATE. Full qualification: INCOMPLETE.** Two required defects remain: interrupted receipt creation can permanently consume an active slot, and a hardlinked scratch stage can corrupt selected authority. No additional blocking defect was established. Existing exact-a3 engineering INTEGRATE and the original CP3 reviews remain unchanged.

Reviewed uncommitted four-Rust-file delta over `a3f9805eccc19ede8dd5c735b54e675e98e63714`, in the same CP2/CP3 checkout. Parent freeze `source-freeze.json`:6,384bytes, SHA-256 `924bf794bfa730b664acb9ddc1d1ef6871fbdca714c89d1f069640abffc9a9c1`. All29 entries independently match sizes/hashes:25 Rust/Cargo plus4 checkpoint controls/docs. HEAD is a3 with four modified Rust files and untracked checkpoint4; this is a frozen byte set, not a new committed Git pin.

Changed source hashes:
- lifetime.rs: `31ca252691bcbbad32b99796642c2af07f0b837e83f954323b61df0669b8a507`
- pack.rs: `a421e97d45168ae541e80ec6caada2475a159297b9286d47b0372b2c7f458887`
- packing.rs: `9b000069b2b5b6a6213e078871fedc28ccad4995f658e433ea5e4b4e5319aa6c`
- store.rs: `379d42d6fa30cf92c1accf4f78fc253c3adfb6c28fa3bf7c0aa17fec56098b7c`

Full four-file diff, complete lifetime implementation/tests, checkpoint4 README/control.py/protocol.py/forecast.json and causal selection/reserve/seal/grant/transfer/recovery/abandon/rolling/capture/collect routes were read. The structured standard remains proposed; only the explicit private disposable v2 owner contract is selected. Candidate/publication/result/page/pack/Cargo formats are unchanged.

Review applied the five axes locally. GitNexus remains UNKNOWN for this unregistered exact checkout; manual risk is HIGH across reservation admission, selection gates, failure ownership, collection and builder accounting. No source edit, engine/build/test/oracle invocation, fixture mutation/copy/recovery, indexing, new chat, worktree, subagent or commit occurred. Build/protocol controls are prepared evidence, not executed evidence. The19 units remain unrun at this source-review boundary; the prior ce8b binary does not qualify this delta.

Source-read completion and freeze release were sent to meta after the complete read/hash checks, before report writing. No source is reread after release. Only this report and compact evidence are written under the authorized40KiB/two-file report allowance.

All engine line citations below refer to `docs/research/file-native-incremental-engine-experiment-2026-10-18/src/` in this frozen byte set.

## F1 [P1] A receipt-create interruption has no recoverable owner path

Primary: `lifetime.rs:239,249-251`; sink `io.rs:141-149`; blocked recovery `lifetime.rs:111,285,307`, `store.rs:329`.

Release writes its immutable terminal receipt directly to abandoned/<attempt> using write_new. That helper creates the final pathname before write_all and file sync. A process can die after create_new succeeds and before the first write, leaving a zero-length receipt; a short write/error/death can leave a prefix. This is a process-interruption window between ordinary calls, not a power-loss hypothesis.

The full active allowance correctly remains charged because the envelope has not changed. However:
1. release retry rebuilds the expected canonical receipt, then rejects the existing zero/prefix bytes at239;
2. selection rejects the existence of abandoned/<attempt> at store329;
3. abandon refuses receipt bytes unequal to b"fenced" at307;
4. transfer refuses the fenced owner at111;
5. v2 collect refuses at entry at285.

There is no supported recovery path that can complete or reclaim this still-owned handoff. Eight such interrupted owners exhaust the unchanged active8 cap indefinitely even with zero committed terminals and ample quota. Keeping the bytes charged protects safety but does not discharge idempotent crash recovery/eventual admitted progress.

The prepared file-write death at protocol.py184-203 occurs AFTER write_all; its receipt is already exact. It tests a sync boundary, not the create-before-first-write or partial-receipt window. Editing a control stage prefix at197-198 does not cover receipt installation.

Required fix: make receipt construction durably owned and recoverable before its first create/write. A deterministic active-owner receipt stage with atomic immutable publication is one bounded option; a narrowly authenticated incomplete-receipt recovery rule while the exact active preselection owner still holds the allowance is another. Preserve exact mismatch refusal for committed immutable receipts, no arbitrary adoption, bounded files/margin, attempt-only fence, all proofs outside W and one active-removal+aggregate atomic commit.

Required proof: inject death after create-before-first-write and after an actual receipt-stage prefix, restart and finish safely without manual fixture repair/abandonment; exact active/control tokens, charges/count and every accepted outcome remain correct; repeated interruptions create only the declared bounded owned files. Add native I/O failure/uncertain-return variants as applicable. No runtime reproduction was performed by this reviewer.

## F2 [P1] A hardlinked owned scratch pathname is unsafe to truncate

Primary: `lifetime.rs:153-165,192-198,223-251`.

proof_paths rejects symlinks/reparse points and nonregular final endpoints, but a regular hardlink passes those checks. A canonical receipt identifies a pathname; it does not establish that the file has no other live aliases. terminal_control opens an existing stage with truncate(true).

Concrete static schedule:
1. A legitimate release dies at terminal-receipt-durable, leaving an exact receipt and its active owner.
2. Its deterministic stage-terminal-<attempt> pathname is supplied as a hardlink to SELECTED.json (or an exact protected original) in the same store/volume.
3. On retry, all current candidate/outcome/history, receipt and exact control proofs pass. The stage endpoint is a regular file, not a symlink/reparse point; the existing receipt satisfies the stage-ownership check.
4. Under W, SELECTED still equals the captured token, so CAS passes.
5. Opening the stage with truncate(true) truncates the shared underlying SELECTED/protected file, then writes reservation-envelope bytes into it. Renaming the stage does not restore the other hardlink's old bytes.

No attacker race after prevalidation, pathname escape, hash collision or semantic acceptance change is needed. This is the exact destructive endpoint that the new mutable-stage ownership proof must cover. Accepted authority/payload corruption can occur before replacement succeeds or fails.

Required fix: prove exclusive file identity before any destructive open/truncate, including regular-file hardlinks, or use a safely recreated/exclusively created owned instance that cannot alias retained data. A verified native handle with regular-file/link-count/identity checks, retained until use, is one approach. Keep platform portability and the selected no-extra-deletion/index-work-under-W constraints explicit; do not merely add another is_file/symlink check.

Required proof: native same-volume hardlinks to SELECTED and a protected original/other control, with an exact matching receipt, refuse before any bytes change. Include interrupted-stage recovery and ordinary single-link success. Preserve symlink/reparse/refused-type tests and report platform support truthfully. The current protocol contains no hardlink negative and does not qualify this path.

## Causal review with no further established blocker

**Whole registry and limits.** Registry reads are native bounded64KiB controls with duplicate-key rejection and typed closed v2 decoding. V1 map representation remains selectable, including an ordinary map entry named version; the old parser rejects the envelope at reservation boundaries, not necessarily Store::open. V2 validates store identity, active<=8, legacy subset/provenance, terminal<=4096, checked active allowances+terminal bytes+128KiB control margin<=32GiB, used<=allowance and minimum terminal-margin accounting. No receipt enumeration occurs at admission. The terminal count/bytes are owner-maintained aggregate authority, not a tamper-proof reconstruction claim.

**Provenance and selectors.** Explicit opt-in marks every existing active owner legacy, including a preparing owner that seals afterward. New ordinary candidates seal as sealed-preselection only; packet/checkpoint/nonordinary and initial/legacy cases cannot use release. Selection_owner preserves V1/legacy sealed-only behavior; new v2 legitimate nonordinary sealed owners can still use ordinary selection. New selection grants persist selection-started before acceptance allocation/member/outcome/result/publication/hint/selector writes and never reset it. File/prepared/checkpoint paths converge on select_locked's grant. A failed grant does not perform dependent selection writes.

**No active-removal bypass.** All reservation mutators read/preserve the entire Registry. transfer rejects a fenced v2 owner and requires started state for a nonlegacy active owner. Normal recover_transfers retains accepted same-attempt/intent/allocation-owner proof; another accepting attempt does not inherit this attempt's private allocations. Abandon refuses overwriting a typed receipt. V2 collection cannot remove receipt-bearing precommit or terminal allocations because it refuses before W/head/scan/delete. These checks make active presence a valid one-envelope commit discriminator for completely installed receipts; F1 is the missing receipt-construction state.

**Accepted outcomes and retry.** Release calls accepted with intent=None before deciding absence, refuses same-attempt acceptance and any other accepted attempt while this owner remains active. For an already terminal active-absent attempt, a later fresh same-request acceptance is allowed only with matching candidate/receipt identity and aggregate checks. Thus fresh retry is not request-cancelled. Candidate version/purpose/store/attempt/request hash/packet/checkpoint/expected identity and selected-history staleness are verified outside W. Current-base, initial, unknown/corrupt or ambiguous proofs refuse.

**Lock/proof boundary.** New handlers dispatch before the generic lifetime W/head prefix. Shared P and the already captured physical pin remain live; physical_token requires captured/current configuration correspondence. Native RESERVATIONS/SELECTED/STORE bytes are captured outside W and compared exactly under W before receipt/control writes. Any token change returns deferred rather than calling an index/hasher under W. No Q acquisition, history walk, deletion or full record verification moves inside this release interval. Bounded control/native sync calls remain indivisible and still need actual hold measurements.

**Collection compatibility.** V2 refuses before trying exclusive P, so held shared-P readers receive the explicit v2 result. V1 obtains try-exclusive P, rereads mode while P prevents opt-in, then tries W once. Contention drops P and reports busy:collect-writer; it cannot wait with P exclusive while refresh/checkpoint owns W and needs P shared. V1's prior full collector behavior/ownership checks remain otherwise intact. This is offline GC only; no v2 terminal GC or online deletion credit is introduced.

**Accounting/atomic handoff.** used+128KiB handoff margin must fit the original allowance before any receipt; the next whole Registry validates cap/quota and is serialized before W. After a complete durable receipt, ONE atomic envelope replacement removes exact active ownership and adds retained charge/count. Repeated committed calls do not add again; all replies report reclaimed_bytes=0 and separately report unused allowance release. A partial/stale receipt-owned control stage can be rewritten without becoming authority before replacement. F2 is the missing alias exclusion on that rewrite.

## Builder and prepared test review

The cached entry_heap_bytes records owned key/hash capacities with checked accumulation before moving the new Entry. It advances after entries.push succeeds; refused order/type/codec/overflow paths leave prior builder state intact. metadata_memory still includes actual Vec capacity and Builder/role storage. push_bound conservatively includes old+new Vec backing during growth, the incoming key and scratch; finish_bound/production reservation from a3 remains unchanged. The prior quadratic string-capacity scan is removed from ordinary pushes. Format bytes are unchanged.

The independent scanned-capacity test checks growth, deliberately over-capacity strings, rejected pushes, overflow and insufficient finish approval. The new maximum225-key test calls production new_builder/push_builder/reserve_constructor/finish_reserved for55,366 records, obtains16,191,779 output bytes, and compares a separate direct build's exact descriptor/index digest. These are meaningful prepared source tests, not execution receipts.

The enhanced heavy cache test genuinely combines:
- existing actual legacy index cache;
- a separately allocated/charged standalone source Index from the same fixture;
- production push_builder calls for its56,000 records;
- a pre-finish reservation that must evict real cached indexes before the output Vec exists;
- real finish_reserved allocation and overflow refusal before any impossible allocation.

It appends no fixture files, but the original full setup still allocates80,976,318 fixture bytes. A source-level 64-bit sizing calculation supports its intended boundary:191-byte keys produce an approximately13,948,149-byte conservative standalone source charge and36MiB constructor bound; one remaining index plus source plus approved constructor fits64MiB while two do not. This is not a substitute for the admitted actual test/native capacity evidence. No unit was run here.

## Prepared controls and qualification limits

The entire328-line protocol was read. It prepares A/B four-publisher contention rounds with protected preparation/packet and a reader held beyond15.1s;12 real losers/mode; exact accepted outcomes and fresh request retry; eight handoff death checkpoints; selection-started ordering on file/prepared/checkpoint paths; lost reply and another accepted owner/intent; head/registry/physical races; synthetic used/count/overflow refusal; candidate/receipt/index corruption; unchanged v2-collect refusal. Its cap4096 fixture is explicitly structural, not4096 actual attempts.

The protocol misses F1's create-before-first-write/partial receipt and F2's hardlink cases. Existing source tests number19 but are unrun. No complete independent v2 acceptance/retention oracle, native platform qualification or byte-restored-successor result is inferred from prepared scripts.

control.py binds QPC guard SHA50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378 and a supplied exact budget hash, records source/control inputs, compiles the same locked/offline workspace/target and schedules all tests serially with no filters. Future actual >40ms monitor gaps censor. This review does not issue RUN GO or read a future executable as matching these sources.

Forecast/new parent budget increases the admitted fixture category to192MiB and keeps target256MiB/evidence48MiB bounds. The source-only review grants no further allocation or planned reclamation credit. All original CP1/CP2/CP3 evidence/failures remain read-only and bound to their original sources.

## Required next action

Remediate F1/F2, freeze the new complete source/control byte set, and obtain a bounded remediation review before treating CP4 as accepted engineering. Preserve this source-negative report and any future red/censored evidence. Existing a3 acceptance is unaffected.

Full final units (including actual64MiB/native constructor interaction), focused receipt/alias/recovery/concurrency proof, complete independent v2-aware oracles, final native Windows/Linux/macOS, full small/scaled/history/long-cohort matrix,1000real activations, actual coexistence/census, backup/restore with accepted successor and B selected-once browsing remain mandatory. This review neither runs them nor converts their absence into a source pass. Full qualification remains INCOMPLETE.

