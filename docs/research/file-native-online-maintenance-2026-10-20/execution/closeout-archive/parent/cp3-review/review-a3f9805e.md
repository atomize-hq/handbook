# Independent CP3 remediation review: exact a3f9805e

**Bounded engineering disposition: INTEGRATE exact `a3f9805eccc19ede8dd5c735b54e675e98e63714`. Full experiment qualification: INCOMPLETE.** F1-F4 from the preserved f9 review are addressed. No new blocking defect was established in the complete remediation diff and its causal neighbors. This is the bounded source handoff decision, not a production/merge instruction, resource admission, clock reset or experiment pass.

Reviewed against `f9b762a1a0d352564542bb4850ad77ee14a7c6e4` in the same CP2/CP3 checkout. HEAD matched and status was clean at both initial and final checks. All25 source/Cargo entries match the manifest and the separate pre-build9 input map. All12 manifest run bindings match. Manifest SHA-256: `f21a616e822c906f75eabff04dd7e97435781ee7a4fa32171acd62d6f119317b`. Bound binary:3,217,408bytes, SHA-256 `ce8b4d5cc6981e3ec4ee24e7d3fc25447550bd7d68b440d04bc334635d6ee2da`.

Full f9-to-a3 diff, f9 findings/report, full updated README, control/remediation harnesses, source binding and raw build/red/green evidence were read. The controlling-document and unchanged paged-format/lifetime trace from review-f9b762a1.md remains applicable. The six changed engine files are io/lifetime/pack/packing/paged/physical_lifetime; rolling and semantic Store/API remain unchanged. The five-axis review was local under the no-subagent instruction. GitNexus remains UNKNOWN/unregistered; manual risk remains HIGH across ordinary eligible-record access, construction/conversion, activation and cleanup. No graph result is claimed.

No engine, Cargo, fixture write/copy/recovery, independent oracle execution, source edit, indexing/bootstrap, extra worktree, commit, subagent or new chat occurred. Only this report and compact evidence were written in the authorized directory. Source-read completion and freeze release were sent to meta before report generation; no source was reread after release. The original f9 red report/evidence and all original failures are preserved.

All abbreviated line references below are within `docs/research/file-native-incremental-engine-experiment-2026-10-18/src/` at exact a3.

## Findings disposition

### F1: addressed — authenticate all entries, enforce limits, then read bodies

`packing.rs:78-104` now locates entries in every relevant descriptor before any body read. It compares duplicate raw identities, then checks every authenticated raw length against the incoming caller limit at97. Only the following loop calls read_record with that same limit. All-route metadata failure still wins before loose fallback; successful duplicate bodies still decode/hash-check and compare exact raw bytes.

`read:124-130` forwards its caller's bound. Exists/mapped use an explicit lookup_bound, including64KiB for hints (`123,132-135,178-181`), so the existence probe preceding Store::select_file's bounded hint read cannot first decode an oversized hint. Other physical roles retain their existing8MiB record limit. No semantic acceptance/membership authority moves to physical existence.

The final selected unit covers raw/Zstd256-byte records refusing limit1 with zero record/decode counters, exact256-byte success, a later corrupt route before payload access, and a compressed65537-byte hint refusing its64KiB existence boundary with zero body counters. This discharges the original causal allocation regression; malformed-input/native/full-suite obligations remain distinct.

### F2: addressed structurally — reserve constructor/output memory before allocation

The physical ledger now adds borrowed_index_bytes, constructor_index_bytes, construction_aux_bytes and incoming load charge before eviction/refusal (`packing.rs:54-76`). Production construction goes through new_builder/push_builder/flush (`207-255`):

- Initial builder metadata is reserved before constructing the builder.
- push_bound reserves existing metadata plus potential new Vec backing while the old backing remains live, the incoming key and conservative scratch before push. It then records actual capacities.
- finish_bound includes live entry/key/hash metadata, the conservative complete serialized output bound, per-entry tree allowance and page/scratch allowance. Flush reserves this before finish_reserved calls paged construction.
- finish_reserved rejects an insufficient approval before construction. paged::build checks its actual serialized length against approved_output before allocating the output Vec.
- After finish returns and builder entries/tree drop, the ledger retains serialized capacity through installation. Both index and pack buffers explicitly drop before constructor charge is cleared and the verification Reader is reserved.
- A conversion's standalone legacy input Index remains in borrowed_index_bytes across construction and verification; flush adds its verifier reservation to that prior charge, then restores it. No Rc remains uncharged across an ordinary subsequent index load.
- Bounded owner/output/work/input-batch allowance is charged for construction. Existing64MiB cache/transient limit,16MiB index,8MiB record/codec and independent maintenance bookkeeping/RSS obligations remain unchanged.

This closes the demonstrated f9 output-Vec/entry peak omission. The reduced unit allocates real cached legacy indexes, an8KiB borrowed Vec and a48KiB output under a96KiB limit, verifies eviction before output allocation, verifies a too-large reservation refuses, and checks finish_reserved(bound-1) refusal. The max-key builder still passes exact serialized sizing.

**Qualification remains open:** this is source proof plus a real reduced-cap heap test, not the required near64MiB legacy/mixed cache plus large production constructor/native allocation boundary. The heavy unit remains filtered. Vec capacity plus conservative allowances is the accounting method; pack_cache_peak_bytes is not allocator instrumentation. The full admitted boundary must still run and reconcile live capacities/native memory.

Metadata capacity calculation currently scans all accumulated entries on each push (pack::Builder::metadata_memory via push_bound and after push). That makes its CPU cost quadratic within a pack. This review establishes no correctness failure from that finite bounded work, and does not block this handoff on an unmeasured latency prediction. Long conversion/construction time remains an explicit performance qualification risk; small/reduced tests do not settle it.

### F3: addressed — reuse only fully verified output created by this live owner

`packing.rs:206,214-229` introduces a bounded OutputOwner table tied to the exact current stage birth. An unknown name still goes through create_new; prior output/path presence never authorizes adoption. The table is populated only after successful creation/sync. A repeated name must match the recorded hash and length, then its complete on-disk bytes are streamed/hash-verified before reuse.

The already durable building journal owns this unique birth before output writes. The table is deliberately not recovered: a building crash cannot resume installation; it can be aborted and its owned files handed to a fresh successor for cleanup. Thus a crashed, incomplete, retired or arbitrary pre-existing path cannot enter same-invocation reuse.

`converted_output:366-369` deduplicates only complete equal rebuilt descriptors, ignoring their former segment number; the comparison includes physical instance, pack/index identity, root/page binding, role/range/count/codec/lengths. Each input partition and fully verified output correspondence was already checked by flush before this comparison. Same pack bytes with different index keys retain distinct index descriptors; their shared pack path remains referenced by the ordinary extension-specific retirement logic. This uses at most64 descriptors/128 created filenames, with no full key set or split-location fallback.

The bound red run reproduces f9's exact AlreadyExists on the structural duplicate/alias shape. Final green A/B covers identical independent input births and same pack bytes/different index keys:12 descriptors/36 input occurrences become7 descriptors/21 output occurrences;15 exact duplicate occurrences are explicitly reported. The independent harness's raw-map decoder compares all21 logical key/raw values before/after on these small raw leaf fixtures. That parser is not the production full Python oracle or a claim of complete long-history acceptance verification.

Actual pack-index deaths leave selected STORE unchanged; resume refuses incomplete-build-owned, abort/fresh birth succeeds, cleanup eventually removes the aborted birth and old pairs. This verifies the intended process-death ownership case, without adding a power-loss guarantee.

### F4: addressed — count the physical domain and separate legacy/paged work

Catalog descriptor stats, pin metadata, legacy pack-size stats, explicit physical pathname/recovery/removal probes now increment the physical counter. Shared I/O directory/read/copy probes and canonicalizations have separate counters. lifetime::remove_checked retains its exact containment/symlink/delete behavior with additive counters; no new deletion authority is introduced.

Legacy index read/hash/parse work is itemized, and legacy bytes contribute to total per-key index bytes. Paged bytes and legacy bytes have separate per-key maxima, with an explicit legacy-or-mixed lookup count even when warm cache avoids I/O. Only fully paged routes are judged against12reads/192KiB. Legacy eager verification behavior remains intact.

Reader root-open and full-audit successful parses now contribute to parse time/count/capacity alongside descent. Four cold height2 routes therefore require12 physical page reads and16 parses; no redundant root disk read is hidden. Failed parses retain whole-operation time and I/O observations rather than pretending they completed.

The final mixed-format unit reconciles a two-descriptor capture's7 physical metadata probes, then cold lookup total12probes/1page read/2parses and all legacy+paged index bytes. Warm lookup adds2body probes, zero index bytes and1descent parse. README states the counters' explicit API scope and exclusions (kernel/lock internals, environment/barrier polling and uninstrumented semantic lifecycle probes), rather than claiming a kernel trace.

## Verification independently inspected

The companion evidence-a3f9805e.json records exact binding/counts and guard summaries.

| Run | Independent parsing | Guard/sample observations |
|---|---|---|
| final build9 | Final source/Cargo25-entry input map equals committed manifest;11 selected units pass;3 units filtered (heavy64MiB plus two larger structural file tests) | exit0,errors[],censor null,two empty closure checks,remaining PIDs[];467,599,360B sampled peak;47ms max sample interval,16 over40ms |
| red |8events, all exact f9 binary;7exit0/1intendedexit2;zero violations; original file-exists(OS80) reproduced with selected STORE unchanged and building owner | exit0,errors[],censor null,two empty closures;26,374,144B peak;32ms max interval |
| green |70events, all exact final ce8b binary;42exit0/26intendedexit2/2actualexit86;zero violations | exit0,errors[],censor null,two empty closures;27,348,992B peak;46ms max interval,1 over40ms |

Of green's26 error results,14 are expected damaged-index hash refusals,10 damaged-pack full-hash refusals,2 incomplete-build resume refusals. The two actual deaths are convert at EXP_EXIT=pack-index, once per mode. Every output index and unique pack is damaged in turn, refused, restored and successfully reopened/audited by the harness.

Independently matched12 repeated resolve responses against exact earlier accepted results for the six existing requests (three per mode). Two new successors report status published/outcome accepted, making four accepted requests per mode. These successor calls are successful accepted publications, not a whole restored-successor oracle. The final receipts do not run backup/restore.

A preserves Discovery None; B preserves its existing discovery value. Green records output_same_owner_reuses12 and output_created_files12 per mode, with the36/21/15 occurrence accounting above. Small charged cache peak is3,145,728B and constructor peak1,088,008B, so it cannot stand in for the64MiB production boundary. The sampler gaps remain visible.

Build8's13 selected passes are intermediate evidence before the final hint probe adjustment. f9/build7 and historical build3 smoke keep their original bindings. Nothing in this report relabels them final-a3 executions. All old harness failures and resource/censor limitations remain preserved.

## Handoff and remaining qualification

**INTEGRATE exact a3 engineering source; full qualification INCOMPLETE.** Meta owns integration, continuation and admission. The four required source findings are closed for the bounded handoff; final qualification is not a prerequisite falsely imposed on this engineering verdict.

Mandatory remaining work includes actual64MiB cache/construction/native capacity testing, full final units (including the two filtered structural tests), independent complete v5 decoder/oracles, N64/H64 A/B and adversarial schedules, same-final-pin Windows/nativeLinux/macOS workspace/strict/packing/recovery/compatibility and initial-upgrade/held-reader lifetime checks,1000real mapping activations, scaled H1002/H10002 rolling/concurrency, all six lookup categories at exact H101007, full chosen long-history verification, long streaming conversion and actual coexistence/reaping/census, backup/restore with accepted successor, and ordinary B selected-once extraction/current graph/source/navigation controls.

No grant increases the64MiB limit, resource forecasts, protected disposal, source scope or fixed clocks. A required unavailable or censored qualification cell remains incomplete. The original f9 report remains the truthful red source review; this separate report closes its findings only at exact a3.

