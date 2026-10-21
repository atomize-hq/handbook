# Shared runtime static review

Verdict: **REMEDIATE — source only, runtime NO_GO.**

Reviewed the complete frozen `runtime_contracts.py`, 65,394 bytes / 1,124 lines, SHA `959e9836aa04325df146e6ee305b9b72ca66326d467bb9208804bdfeb43aca8a`. Meta explicitly retargeted this hash; the initial assigned `92bc601b…` was not present and is not reviewed or qualified. The file was read in full without importing or compiling it. The structured-format standard was read and remains **PROPOSED**; no format migration or production approval follows.

## R1 — P2 Required: Return the complete 027 lookup representation shape

lookup appends only format/fresh/warm; exact027 requires source_pin,binary_sha256,actual_H,full_oracle_sha256 as well. A real completed initial lookup fails closed-shape before phases. Obtain actual per-format complete oracle receipts; do not invent hashes.

Source: `runtime_contracts.py:491,521`; consumer: `history_lane.py:validate_lookup_evidence`.

## R2 — P2 Required: Match the actual paced input, ready and anchor ABI

Pace requires family; PaceAnchor requires clock_kind; current ready includes clock_kind/family. _pace_fields omits family, anchor omits clock_kind, closed _ready rejects those emitted keys. Bind f05-selected and windows-qpc from the actual reviewed ABI and validate full captured physical tuple. Original4434 manifest was read; current accepted source hash differs and is separately bound.

Source: `runtime_contracts.py:672,770,799`; consumer: `accepted current packing.rs Pace/PaceAnchor/paced_probe`.

## R3 — P2 Required: Reconcile the closed runtime wrapper with the 035 flat binding contract

derive_whole_forecast passes closed BINDING_KEYS wrapper to035 derive_history_forecast; _derive_history_forecast reads bindings[source_pin], absent from that wrapper. No closed036 input can supply the flat field. Select one source-owned typed three-argument ABI consistently for whole and phase profiles; no arbitrary callbacks.

Source: `runtime_contracts.py:299,310,708`; consumer: `long_source_profiles.py:939,950,1058,1071`.

## R4 — P2 Required: Seal a final body consumable by the existing post-exit finalizer

036 writes phase/runtime_sha256/guard_sha256/finalizer_sha256/post_case shape, while035 requires phase_id,plan_sha256,budget_sha256,outer_run_id,source_profile_sha256,summary and three top-level closure/checkpoint refs. Exact key-set check rejects every036 body; external index shape also differs. Produce the actual final reference body after all attachments and case closure, keep its own hash external, and validate external index identity rather than synthesizing final PASS.

Source: `runtime_contracts.py:1039,1081,1106`; consumer: `phase_finalizer.py:PHASE_BODY_KEYS,_phase_windows439`.

## R5 — P2 Required: Admit the real phase call-index width before publishers write

finish writes all native call raw_refs plus40reader records in one bounded record. Every P4 phase has400 mandatory publish refs: their SHA strings alone occupy25600ASCII bytes, already exceeding16384 before all remaining reference/reader/maintenance fields. Current preflight checks only that width is an integer. Derive the actual source-owned width or stream references and include every sink in the whole prewrite forecast.

Source: `runtime_contracts.py:968,979`; consumer: `long_source_profiles.py:1078 call_index_record_bytes=16384`.

## Preserved design and honest dependencies

The source fixes sixteen ordered phases,100accepted publications each,4000offers/2400sealed losers, four native streams × ten batches ×1000warm queries plus40first queries per phase. Native origin is immutable and future, with four initial native-ready markers; the source waits through every finite reader response and does not renew or skip queries. Exact warm array lengths and outcome digests are checked, and raw response references preserve the arrays that the normalized rows omit. Mapped metrics are required from actual engine fields; no zero substitution was found. Terminal release uses an actual point operation, hashes the immutable fence and checks exact request/base/candidate charge. These are static observations, not execution passes.

The factory stores the host without reading its properties. Activation checks positive same-pin `history_authorization` before reading harness/native ports. The twenty frozen native port names are checked, but their implementation, actual registry handles/leases, callbacks, EOF/drop/joins and all live bindings still require separate reviewed platform code. Missing ports or final metric/profile/source/hash inputs must remain prewrite NO_GO.

The initial035 profile snapshot still has draft40/46/19second policy; meta selected44/48/23.6. Its mismatch is a known profile-owner update dependency, distinct from R3's incompatible data shape. Exact027 history code remains `RUNTIME_CONTRACTS=None`, an old callback-shaped adapter and historical QPC binding, and has not yet attached the new post-case sealer. That integration must be explicitly source-owned/reviewed; flags/hashes cannot fill it. Existing full forecasts and source-derived finalizer schemas remain controlling. No original-Job final PASS is claimed by this file; a real post-host-exit join remains mandatory.

## Scope and evidence

Read the exact027 history validator (`ca134929…`,54,942bytes),035 profile/finalizer snapshot sections needed for these call paths, and the4434afbb CP4 paced source-freeze manifest/review. The live accepted packing source is a later distinct hash, explicitly recorded in evidence; no claim that it equals the old4434 packing member is made. Current lifetime source matches the manifest's3172ba1b hash.

Five-axis result: correctness has the five required cross-module/width defects above; architecture preserves selected authority, bounded native ownership and provisional-only in-host records; readability is constrained by the nearly full one-file allowance; security/identity and performance remain unqualified until real ports and post-exit raw evidence exist. No delegated review, source edit, private import, compile, control/native/engine/Cargo run, Git command or indexing occurred.

Reports are only this `review.md` and `evidence.json`, together below24,576bytes. A fresh META10 v5 whole-membership census was checked before each write, including all P files, both closed Landing roots, the seven included subtrees with the declared stream-snapshot exclusion, and all twenty promoted harness files;250KiB future reserve remains inside10MiB. No deletion, expiry, planned-reclamation or relabeling credit was used. Source freeze is released to meta after the final hash check; remediation must receive a new exact freeze and review.

## R1 full-source re-review — 9be1a22b (76,873 bytes)

Verdict: **REMEDIATE, source only; runtime NO_GO.** Entire new file was read. Exact original959e negative archive SHA`e4baba7d…` was checked through gzip EOF; prior findings/results remain preserved above.

Previous R1 lookup fields and real per-format oracles are repaired. R2 now includes f05-selected/windows-qpc input/anchor/ready fields and compares full physical tuple and catalog. R3 is repaired by the current035 wrapper ABI. R4's18-field body now matches the current035 finalizer key set; body/index buffers are reread and SHA-checked, real completed case-index SHA is used for RAM validation, body hash remains external, and final receipt projection/recomputed summary exclude circular stored receipt hashes. R5 replaces16KiB with a source-derived width and defers the complete index until case closure, but the following ABI omission remains.

**R1-A — P2 Required: return the case-index geometry preflight requires.** runtime_contracts.py:476–478 requires case_index_scope equal closed-case-all-engine-and-checker-calls and an integer case_index_max_calls. _case_call_index:1138 also indexes that count. The inspected current035 producer (long_source_profiles.py SHAe8a97deb0298f16958b83747826bbffd5d945a5ea6225e7d36ac2da8ca0c8481, history_phase_profile1075+) returns neither field. Therefore preflight always refuses. Export the actual source-derived scope/count or reconcile the typed contract before RUN; a caller constant cannot replace this source proof.

**R1-B — P2 Required: initialize truthful control maintenance counters.** Phase.finish initializes boundary_cleanup_calls and final_unprotected_backlog to None for every phase. Exact027 validate_phase_evidence requires both integers for every phase, including control; control boundary calls must be zero. The exact027 coordinator fills these only for online phases. The new sealer invokes that validator directly, so the first control cannot seal. Use truthful zero for a control's absent boundary work and its proven no-backlog baseline; retain pending values until actual online drainage is attached.

Native observation validation now also requires callbacks_joined; complete actual query arrays remain reconstructible from the one raw capture. Source/binary/native clock, whole physical capture, point terminal charge and actual metadata references remain checked. Factory host-property authorization ordering and the twenty-port set are preserved. Real platform ports, final metrics, source-derived forecast admission, and027 coordinator integration remain dependencies. In particular the unchanged027 historical QPC constant/None runtime adapter must receive its separately owned integration; hashes do not supply implementation or runtime GO.

No imports, compilation, controls, native/engine/build/Git/indexing/delegation or source edits occurred. This is not a runtime proof. Review pair remains under24KiB, fresh META10v5+250KiB reserve checked before each write. New source freeze is released to meta after final hash verification for bounded remediation.
