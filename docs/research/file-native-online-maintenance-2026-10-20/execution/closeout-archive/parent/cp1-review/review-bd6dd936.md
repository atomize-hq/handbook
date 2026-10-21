# Exact latest source review: bd6dd936 / candidate10

Source: `bd6dd9367acddaddcd7873c8043548502a170c3b`, parent `cd12ea85db214428998f5507ffdf81fc8708ba67`, branch `codex/file-native-online-cp1-20261020`. HEAD, clean worktree and pushed remote were independently checked. Executable `C:/file-native-online-cp1-20261020/evidence/candidate10.exe`: 2,979,328 bytes, SHA-256 `7d1157d5a2df08904e118ce86647fd45f1ae7fec209494c02afdb1fac4b0b22f`. All 23 committed source entries match checkpoint1/source-binding.json, candidate10-binding.json and the LF-restored build10b pre-build map.

**Final bounded engineering verdict: INTEGRATE this CP1 source checkpoint.** All identified required code defects are addressed; no new actionable blocking source defect remains. The corrected 14-death schedule and five requested latest-root independent full oracles have been reviewed and passed. **Full CP1/experiment qualification remains incomplete.** This is an engineering handoff to meta, not a merge instruction, CP2 release, resource admission or claim that the full small/native/history matrix has passed. This report does not rebind candidate9 stress, held-reader or authority evidence to candidate10.

## Narrow fix and retained source conclusions

The entire Rust/Cargo/consumer diff against cd12ea8 is one condition at `src/packing.rs:395`:

```rust
if name.strip_prefix("packs/").is_some_and(crate::model::safe_id) {
```

This exactly matches the extension-free safe birth-id directory grammar. Nested paths and flat legacy `<hash>.pack`/`<hash>.idx` names fail safe_id because slash/dot is prohibited, so they use checked file deletion. Explicit `packs/<birth-id>` tasks still use empty-only directory removal. This resolves native9 OS267 without changing capture, pins, Work selection, metadata budgets, semantic behavior, record decoding or accepted identities.

The prior detailed source review of cd12ea8 remains applicable outside this one condition: immutable authoritative Work2 closes mutable work omission/forged-done authority; flat durable aborted-owner handover closes interrupted/transitive ownership loss; fixed staged Work ownership, authenticated transition-parent recovery and explicit empty directory tasks close stage/directory leaks; checked ordinal and selected birth claims prevent re-adoption; immutable physical_parent plus frozen configuration/candidate-id/catalog/ordinal/pin checks constrain preselection; selected STORE remains recovery authority. Metadata CAS retains pin identity, tries Q/W once, propagates deferral before dependent deletion, and uses one shared 50 ms/1,000-phase cleanup-entry budget with a 35 ms/800-phase work cutoff. Legacy whole-index work remains explicitly timed/indivisible and unqualified for final large-history bounds until CP3.

No new graph result is claimed. GitNexus is unavailable/UNKNOWN and manual tracing remains HIGH risk across API/consumer/lock/cleanup routes. The proposed structured-data standard and existing owner/version contracts remain unchanged. No reviewer source edits, builds, engine calls, fixture mutation/recovery/copies, subagents or extra chats occurred.

## Exact candidate10 native evidence inspected

Native10's first four completed cells are recorded in its partial results and program, with candidate10 raw event hashes:

1. A real old executable creates STORE2/flat packs and holds an old catalog reader. Conversion returns busy:legacy-readers without changing STORE; new readers still resolve. Release permits conversion/resume, flat old containers are reclaimed, and the old executable refuses STORE3.
2. A native CreateFileW handle opened share mode 3 (read/write, no delete) holds an obsolete pack. Cleanup defers with blocked work and zero deletion credit for that batch/native allocation credit zero; a new ordinary resolve succeeds.
3. Releasing that handle and retrying removes the held obsolete pack.
4. A reader resolves successfully while maintenance is paused immediately before deletion outside Q/W.

Native10 then failed a harness assertion expecting the wrong generic busy label. The engine returned the correct `busy:competing-maintenance:...` result. Guard exit is 1, no guard errors/censor, and both closure checks are empty. Preserve it as a partially completed run with a harness failure, not a whole-suite pass. Its 18 recorded candidate10 events have 16 ordinary exits and two intended error results (legacy drain and contender); no event guard violations. Sampled peak RSS is 31,039,488 bytes; sample max 47 ms, 15 intervals over 40 ms.

Native10b separately corrects that expectation and passes three remaining cells on a fresh candidate10 fixture:

- The competing maintainer refuses promptly; a semantic publisher advances while a build is paused; the physical CAS still succeeds because semantic advancement does not alter STORE.
- Cancellation after complete construction leaves exact selected STORE bytes unchanged, preserves candidate ownership and permits resume/drain after cancellation is released.
- A labeled synthetic internally bound u64::MAX ordinal refuses before journal/birth changes, with all file hashes unchanged, then restores the fixture's original bytes. This is synthetic overflow proof, not a real long-running generation count.

Native10b has 14 exact candidate10 events: 11 normal exits, the intended contender/cancel/overflow errors, no event violations. Guard exit 0, no errors/censor and two empty closure checks. Peak sampled RSS is 30,793,728 bytes; max interval 63 ms and one interval over 40 ms.

Build10b is the controlling post-LF-restoration compile/unit receipt. It passes offline release build and four small units; heavy cache unit remains final-pin work. Full build/guard source hashes, timing and sampling belong in the companion evidence record.

## Preserved new schedule failure and remaining gate

The first faults10 attempt completed five A phases, then expected exit86 at pack-loose-delete but received exit0/complete. Its sequential reused fixture had already deleted mapped loose originals; new publications remain outside CP1's deliberately frozen cohort. Thus the loose-deletion hook was not reached. There is no engine error or event violation in that mismatch; it is an invalid crash schedule. A fresh bounded cohort or explicit authentic canonical loose duplicate is required before claiming that death window. The failure/raw partial rows are preserved; no 14-cell pass is claimed here.

Current-pin aborted-directory/death schedule proof and remaining independent full-oracle receipts are still awaited. Candidate9's 1,000 real activations, 16.109-second held reader, stage/configuration/ownership negatives and stable bookkeeping were independently reviewed under **cd12ea8/candidate9** only. The one predicate change can be reasoned about narrowly; those results remain prior-pin evidence and final shared CP3 pin stress/native qualification must be rerun.

The full A/B N64/H64 matrix, protected owners, same-final-pin Windows/Linux/macOS, heavy cache, restoration/accepted successor, B shell extraction/navigation, long-history work bounds, CP2 rolling/new-tail capture-gap/deferred work and CP3 authenticated paged lookup remain required. No resource admission, clock reset, CP2 release, automatic merge or complete experiment verdict is granted by this source review.

## Corrected death schedule: now verified

`faults10b` fixes the unreachable loose-deletion hook by saving the exact initial loose result bytes before compaction and reinserting that exact canonical duplicate only for the disposable loose-deletion case. This creates a real mapped duplicate to remove; it does not add new tail history to CP1's frozen cohort.

Independently parsed 270 raw events, all exact candidate10 SHA: 242 ordinary exit0, 14 intended protected-snapshot exit2 results, and **14 actual injected exit86 deaths**. The seven phases occur once in A and once in B: reservation, install, before catalog selection, after selection, accounting, loose deletion and old-container deletion. Every case checks exact SELECTED and all prior outcomes, preserved payload/readable path hashes, sealed prepared candidate bytes, saved packet manifest and persistent snapshot protection; it then selects the prepared candidate successfully and observes protected retirement refusal. A has 30 payload paths at each phase; B grows from 61 through 241 paths as accepted generations accumulate. Outcomes grow from 3 to 15 per mode. The source program and partial/result rows match the raw phase schedule.

Guard exits 0 with no errors/censor and two empty closure checks; sampled peak RSS 27,181,056 bytes, max interval 47 ms with four over-40 ms intervals. The earlier faults10 failure remains preserved and is not relabeled. Full N64/H64 and independent full-oracle qualification are distinct from these N10 focused death cells.

At this update the corrected schedule gate is satisfied. The engineering verdict awaits only the requested five latest-root independent oracle receipts; no additional blocking source finding is open. Known required source findings F1–F4, configuration/candidate/immutable-parent admission, deadline deferral and legacy flat-container regression are all addressed, with their exact-pin evidence and prior-pin limitations recorded above.

## Final independent oracle review and verdict

The requested latest-root batch is `C:/file-native-online-matrix-20261020/receipts/bd6-latest-full-oracles/`. All five checker subprocesses exit 0 with empty stderr: legacy10, sharing10, native10b, faults10bA and faults10bB. Their raw oracle outputs, before/after inventories, read-only receipts, batch rows and guard were independently read and hashed by this reviewer. Every before/after file hash/size/name and directory list is byte-identical; no engine calls or fixture copies occurred. Directory counts in these inventories include the root itself.

| Root | Accepted publications | Raw packed records | Files | Directories including root | Protected owners / B controls |
| --- | ---: | ---: | ---: | ---: | --- |
| legacy10 | 1 | 5 | 55 | 46 | No pending physical work |
| sharing10 | 1 | 5 | 54 | 43 | No pending physical work |
| native10b | 2 | 5 | 61 | 47 | No pending physical work |
| faults10bA | 15 | 10 | 164 | 106 | 1 pin, 1 reservation, 1 packet |
| faults10bB | 15 | 10 | 376 | 220 | 1 pin, 1 reservation, 1 packet; 15 readable generations and 15 exact loose publication controls |

Every root reports zero retained old generations, pending loose jobs, retired container jobs and pending garbage. Full semantic, navigation/discovery/extraction and exact request-intent/accepted-result verification are present, not just engine audit responses. The independent parser module SHA is `27fba357cef53f1775801d2e4bf4b63166356cf94edb8a0ec094453f9aab8a24`. Its schema token remains cd12ea8 because Work2/STORE3/catalog2/FNPIDX01/pin formats are unchanged; the separate source-equivalence binding names **ENGINE_SOURCE_PIN bd6dd936**, the exact candidate10 SHA and the one directory-predicate delta. This is schema compatibility, not rebinding old engine executions. All 24 files in the oracle source binding were independently rehashed and match, as does the schema-equivalence receipt.

Batch guard exit 0, no errors/censor, two empty process-closure checks; sampled aggregate peak RSS 43,311,104 bytes. Stage time is 13.072 seconds and total including setup/token wait is 24.727 seconds. Sample maximum is 47 ms with three over-40 ms intervals; these gaps are retained and do not become a new resource-envelope qualification.

**Fixed classification:** F1 aborted ownership handover/transitive work, F2 mutable progress authority, F3 stage/empty-directory ownership (including the latest legacy-file discriminator regression), F4 no-reuse/checked ordinal/birth claims, frozen configuration/candidate admission, immutable physical-parent CAS binding, and safe cooperative cleanup deferral are addressed. Forced generator nonce-collision injection remains unavailable; static constructor checks and actual durable claim retention are evidence, not a fabricated runtime collision pass.

**Open classification:** complete A/B N64/H64, final shared-pin stress/cache/native Linux/macOS/Windows and guard/resource gates, full backup/restore/accepted successor, long-history 128 KiB Work capacity/checkpoint timing, mandatory shell/current/navigation controls, CP2 rolling/deferred discovery/new-tail causal capture gap, and CP3 authenticated paged lookup remain required. Candidate9's 1,000 real activations and held-reader proofs remain exactly candidate9 evidence; final-pin repetition is still required. Original negative results/censors and all intermediate source bindings remain preserved.

**INTEGRATE** is limited to the reviewed bd6dd936/candidate10 engineering source handoff. Meta owns any integration, continuation and admission decisions. It does not complete the experiment, lower any frozen gate, authorize automatic merging, change semantic indexes or public formats, grant a new resource allocation, or reset the fixed clocks.
