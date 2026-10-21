# Focused replacement review: f6f9079 / candidate8

Exact pushed source: `f6f907943c54936e49123a136a5f0f87993758ab`, parent `62ce963085ac054bcc34eb21c6733b94db160975`, branch `codex/file-native-online-cp1-20261020`. Remote head was independently checked. Candidate8: 2,957,312 bytes, SHA-256 `323608968d7cceb5c49bead47043a68eebf6ff2ca8e1668f39e2bf2436e6f7ec`. All 23 committed source-binding entries match candidate8-binding and build8-source-before, and their committed bytes/hash were independently verified. Review uses committed blobs; later worktree changes do not amend this result.

This is a provisional replacement review, not acceptance of the latest eventual CP1 pin. Source F1/F2/F4 are addressed; F3 has an additional directory-recovery defect. Cleanup entrypoint deadline work remains required. Full qualification remains incomplete. Meta has routed both remaining issues; a further source pin is expected.

## Remaining actionable findings

### F3 continuation — P2 Required: aborted empty pack directories lose their reclamation task

`packing.rs:285-292` queues only existing files when importing an aborted birth's pack namespace. `physical_lifetime.rs:160-165,257-264` removes absent garbage paths from loaded Work; only physical-stages gets a parent-directory existence check. `packing.rs:472-477` removes each aborted file before removing its parent directory.

Two crash schedules strand an empty `packs/<aborted-birth>` directory: (a) death after its mkdir and before first file creation, then abort/adopt; (b) death after removal of the final queued aborted file and before parent remove_dir. In (a) directory enumeration finds no file to queue. In (b) native-absence normalization drops the last file's task before cleanup can remove the empty directory. Birth claims retain the id but no directory cleanup path consumes it; after a selected successor clears claims, the orphan persists. This differs from retired descriptors, whose records remain replayable across absent pack/index files, and from the fixed physical-stages empty-directory recovery.

Required fix: explicit bounded owned pack-directory work or an equivalent safe parent-existence task, including directories created before the first file. Preserve ownership before removal and verify actual file **and directory** drainage after both crash schedules and a successor. Existing file-only old-instance assertions do not prove this case.

State at f6f9079: **OPEN**, source-proven; reproduction not run by reviewer.

### Cleanup entrypoint budget — P2 Required: time/defer accounting excludes work and permits long waits

`packing.rs:499-503` adopts an aborted owner before cleanup starts its timer at `398`. Adoption includes native enumeration, complete aborted-plan validation, a Work metadata CAS and a PACKING write. `physical_lifetime.rs:206-246` can retry capture/writer access up to the default 15 seconds. Load/native absence normalization and save's canonical comparison perform additional metadata checks without cooperative deadline checks. Cleanup unconditionally calls save at `packing.rs:448,480` after the candidate budget can already have expired. Its returned batch_ns excludes adoption.

Start the declared entrypoint budget before adoption/recovery, preserve a durable safe defer point, count and check between bounded native/metadata steps, and report total/adoption/recovery/checkpoint work. One nonblocking Q/W attempt avoids the retry loop but does not alone close all paths. Do not hold Q while waiting for W. Pausing after a completed cursor/drain checkpoint must not allow old plan replay to access a removed pin/container.

CP1 must retain the whole-index reader. Legacy indivisible index loading/parsing can remain explicitly unqualified for the final long-history 50 ms gate until CP3, with truthful separate accounting. This finding does not require moving paged lookup into CP1 or lowering the final experiment gate.

State at f6f9079: **OPEN; already routed by meta**.

## F1–F4 source disposition

- **F1 addressed in source:** Journal.abandoned is a bounded flat closure computed before PACKING overwrite; direct birth ownership is included unless already transferred. Aborted-plan bytes are hash-checked, closed Work2 decoded and bound to store/birth/catalog before inherited garbage is flattened. Aborted adoption adds the birth claim and selects durable Work ownership before removing the journal reference. A crash between those writes leaves duplicate ownership. Runtime rem8 exercises immediate B death and transitive installed aborts.
- **F2 addressed in source:** Work2 is immutable and selected by STORE.physical.plan. Unexpected mutable `<id>.json` progress refuses. Generation identity/catalog checks and selected Work/catalog correspondence remain enforced. A metadata checkpoint preserves generation id/ordinal/catalog/pin and performs a bounded STORE CAS; older drain removal is selected before pin deletion and completed loose jobs are selected before old-container deletion.
- **F3 partly addressed:** deterministic selector/work stages replace untracked random progress stages. Complete synced Work bytes are installed under their hash with no replacement; birth journal ownership precedes initial creation. Candidate recovery requires exact hash, closed Work2, matching store/id/ordinal/catalog and transition_parent equal to the current selected plan. Malformed stage bytes refuse without deletion. Current/retained roots are excluded. All load/save/recovery callsites are under the M lock acquired by packing::run. Ordinary readers never own Work roots. The selected activation-stage empty-directory case is repaired and tested. The aborted empty-pack-directory continuation remains open above.
- **F4 addressed in source:** constructor uses checked next ordinal plus nonce, checks selected birth claims, journal owners and existing birth paths before birth writes. Adoption selects a claim before same-parent aborted reclamation. Metadata CAS preserves claims; append_work clears them only in a candidate with the next ordinal, leaving the selected parent untouched if construction/activation fails. Explicit reclaimed-name collision/claim-retention and overflow runtime proof remain to be supplied.

The earlier selector-stage deletion hazard is addressed: foreign-generation candidate plan deletion now fails exact authenticated Work identity, and a same-generation candidate additionally needs the current selected parent token. The token is compared, never traversed/retained as a predecessor chain. Same-generation unselected metadata checkpoints have no independent reader owner; M excludes active concurrent builders. The malformed completed stage refusal is present in rem8 raw evidence.

The earlier idle treadmill concern is addressed in source and focused evidence. Load normalizes natively absent immutable garbage without incrementing counters; save compares this normalized selected Work to in-memory Work before creating a stage. Metadata predecessor-root removal does not credit counters, so idle normalization does not itself create a new root. Selected bytes may retain one absent predecessor tombstone. Completion explicitly names native absence as its basis. This is bounded residual metadata rather than a retained plan chain.

## Existing evidence independently inspected

Build8 exits 0, four small tests pass, one heavy cache unit is filtered. Build guard sampled peak RSS is 428,707,840 bytes. Total guard time is 44.485 seconds (including acquisition/setup), not a 25.37-second total claim; the build stage timing must be identified separately. Sampling receipt records 11 intervals over 40 ms, maximum 62 ms; those gaps remain visible.

`rem8-results.json` reports eight focused cases and binds candidate8's exact SHA. Recorded program and raw events were inspected: A abort/B reservation death/C successor removes the two original owner files; two installed aborts plus a successor remove 22 observed owned file instances; ten idle cleanup/status repetitions preserve exact STORE bytes, all file hashes, file/directory counts and zero physical/work CAS samples; malformed completed stage refuses with files unchanged; fixed write-stage death recovers; three repeated installed-progress deaths keep plan counts [3,3,3] then drain; STORE-replace empty activation directory recovers; the old gap7 fixture failure is repaired. Guard exits 0, no censor/errors, sampled peak RSS 26,628,096 bytes, maximum sample interval 32 ms, no over-40 ms intervals. The rem8 artifact is focused proof and explicitly leaves independent full oracles pending.

`gap7-result.json` remains a preserved earlier failure: it reports complete while the activation directory still existed. Candidate8 repair is separately bound and does not relabel gap7 as an original pass. Rem8's pack ownership assertions concern files; they do not exercise either newly identified empty aborted-pack-directory crash.

Additional requested candidate8 raw proofs (valid forged foreign/same-generation stage, immutable Work/legacy progress corruption, claim collision/retention, 16-second reader across two successors) were still pending when this report was drafted. Candidate3 and earlier raw evidence remain bound to their own binaries. No inherited pass label substitutes for final-pin proof.

No engine execution, build, source/fixture mutation, recovery, copying, subagents or new chats by this reviewer. Only committed-source reads and existing-artifact hash/parse checks. GitNexus remains UNKNOWN/unavailable; manual callsite/lock tracing is not complete indexed graph evidence. All review output remains within the assigned directory/256 KiB budget.

## Boundary

Do not integrate f6f9079 as a fully qualified checkpoint. Await the directory/deadline replacement pin and bind the focused re-review and runtime receipts to it. Full small A/B schedules, independent full oracles, legacy/native sharing/death/cancel/protected-owner schedules, 1,000 activations, same-final-pin native capsules and restore/successor proof remain required. CP2's causal new-tail capture-gap, rolling/deferred discovery and CP3 paged lookup remain required; this review does not release CP2 or change clocks/resource authority.
