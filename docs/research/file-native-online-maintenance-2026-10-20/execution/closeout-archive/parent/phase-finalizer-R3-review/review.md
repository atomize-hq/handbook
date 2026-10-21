# Finalizer R3 independent source review — request changes

Reviewed UTC: 2026-10-21T09:35:57.098Z
Verdict: REQUEST_CHANGES. The frozen source is not accepted for integration or runtime.

Source: C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/qualification-lanes/phase_finalizer.py
Exact bytes: 103738
SHA256: 82b23967df7270dbece193e2d8082f7a670aec1f6d2fbcdc09affbfa9796bea8

This fresh review read all 1678 lines of the finalizer and all 1512 lines of the actual shared runtime, 90542 bytes, SHA256 ae2694e4a36d8edb496257b4a0cf2bee5749f85bcccc2bdf40d23a952f9b3cd0. The reviewer did not author this finalizer. No earlier report was substituted for this read. Source remained held unchanged for this review. The separate Archive R3 authorship/acceptance is unaffected.

## Required findings

### F1 — P1: Join the original outer completion budget, bounds and config provenance

Location: phase_finalizer.py:1336–1346, validate_native_lifetime.

Actual G12 GUARD_WIRE_CONTRACT.parent_completion.required includes budget_sha256, bounds_sha256 and config_ref. Actual unlocked_capture.py:860 publishes these from the outer Capture's budget hash, bounds hash and config reference. The outer receipt also carries this tuple.

The finalizer validates outer_receipt.budget_sha256 and bounds_sha256 against the authority's original outer budget and bounds. It does not require the completion's corresponding fields to equal those in that actual outer receipt, and does not file-bind config_ref. These required producer-owned fields can differ without changing the finalizer's verdict. This leaves the completed outer-controller configuration provenance unjoined.

Required correction: compare all three completion fields with the actual outer receipt, then verify the exact referenced original config bytes/hash under the agreed 131072-byte ceiling. The author's 230-byte RAM proposal was acknowledged as required; its replacement bytes were not written or accepted by this review.

### F2 — P1: Fail closed when a physical-root census cannot enumerate completely

Location: phase_finalizer.py:1195–1208, census.

The traversal uses os.walk(root, followlinks=False) without onerror. Directory enumeration errors are ignored by default; an unreadable or I/O-failed subtree can be omitted. The resulting lower totals feed actual-plus-reservation and whole-experiment byte/file/directory checks and can reach QUALIFIED. A configured root that exists as a regular file similarly passes the root.exists branch but produces no directory rows and counts as zero.

Required correction: require an existing census root to be a directory and propagate every traversal/enumeration failure. Retain the no-link and all local/combined cap checks. An explicitly absent prospective category may be zero only where the authority contract permits that absence; an unknown census is not zero. This finding follows the static traversal/error path; no filesystem fault injection was run.

### F3 — P1: Validate root and Job endpoint coverage with their corresponding RSS markers

Location: phase_finalizer.py:583–587, scan_guard.

The variable last is the final RSS completion over the whole retained tape. The final predicate requires both root_end-last and job_end-last to be nonnegative and at most 40ms.

Actual G12 covers root end at job_guard_bounded.py:1205, then Job-drain coverage at1212. The sampler stays active until stop.set at1228, after timer cancellation/join. A healthy RSS completion after root coverage, or after Job coverage before sampler stop, makes last later than that endpoint. The consumer then rejects valid whole-Job coverage.

Required correction: join each endpoint to its own actual prior fully written positive RSS marker and check its 0..40ms gap. The producer's coverage rows already retain previous_recorded_perf_counter_ns. Preserve complete-tape cadence, slow-sample and source timing checks. Sampling must not be required to end before root exit. This is a source-backed consumer mismatch; no native control was executed.

## Source reconciliation performed

The actual runtime and finalizer agree on the selected 18-field provisional body, external body-hash projection and source/runtime/finalizer/guard/profile identities. Native reconstruction validates BEGIN/LAUNCH/START/DATA/RSS/END sequencing, call ownership, coalesced lossless stream chunks, per-stream and call hashes, chronology, positive RSS, caps and actual native closure. Retained refs must identify exact verified physical tape rows.

Case-index membership/pages, ready/origin refs, phase scope and owned-drain/checkpoint/case-close attachments were traced to actual runtime consumers. The checker consumer joins isolated argv, original request, INDEX stdout, artifact companions, source hashes, full-H intents/navigation/mapped-cohort results and raw native receipt. This is review of the finalizer's consumption path; it is not independent acceptance of the changing checker source.

The expanded HISTORY schedule reconciles 22 cases, 16 phases, 26 full oracles, 1188 mapping queries, 24 backlog queries, 4000 publishes, 2400 terminal releases and 802 quotas with the actual runtime. The 24 backlog queries are not a claim that all global HYST/LONG cells are qualified. This finalizer explicitly returns an original-HISTORY-only join; other matrix cells and LONG providers remain separate.

Both old native Jobs are joined through creator/host identities, noninheritable Job-handle source audit, two empty observations and checked close; the old lifetimes end before the grant and the finalizer's separately admitted actual parent/Job. The source-freeze/Machine acceptance path binds fixed engine062, the prospective harness window, parent-owned manifest/review refs, exact private helper hashes, canonical source-map digest and bounded raw review references. The authority's census/output/limits/caps are compared to config, both inner/outer guard budgets are referenced and actual raw/parser bounds are joined. Guard receipt bounds permit the source-approved maximum32MiB instead of an obsolete small cap.

Output requires a fresh owned path, one preheld file reservation and a prewrite byte bound, followed by inclusive postwrite census. Finalizer outer closure remains pending explicitly. F2 prevents acceptance of that census implementation as currently written.

## Dependency pins and limits

- Runtime: 90542 bytes, ae2694e4a36d8edb496257b4a0cf2bee5749f85bcccc2bdf40d23a952f9b3cd0; full source read.
- G12 job_guard_bounded.py: 77668 bytes, 48feda3aee5cabe4140ea6ca5604e7a4dce43276b82de86e5cf1d409d6fdd6a7; actual schema and sampling/closure paths read.
- G12 unlocked_capture.py: 63277 bytes, 082a2c3b6b22326988da17baab735c589a7c4fcc82adaf47e83890f05b7e84b8; actual completion/grant/authority paths read.
- windows_evidence.py changed during this read from32499 bytes/2ea593ee4cca5ac4308c738ab6cf74c99a7c2a56f2733940dd481783cc95c715 to32652/05c5131840aff3c461e781e3cebe4aaf413635cd8d55a629f90bfa97ffe821f1. It was not globally frozen or accepted here.
- Independent checker CLI is in the Matrix checkout at execution/harness/independent_checker_cli.py; parent identified old32761/f054... being replaced by CK1–4 repairs under066. It is WIP and separately reviewed.
- Startup gate producers are Landing windows_driver.py/windows_bootstrap.py, also WIP and separately reviewed.
- Profiles and LONG provider remain WIP. Parent's known G harness timestamp-regex issuer defect is a separate blocker, not presented as a new finalizer finding.

No complete final execution config, raw authority allocation or all-24-HYST/global producer freeze was admitted by this task. No numerical physical44GiB compliance or native execution success is asserted. A later changed finalizer needs its own exact hash/reread. New069 capacity and an author's RAM repair are prospective and do not change this negative verdict.

## Method and authority

Read-only source inspection; no private module import, native API/control, tests, Git, indexing, bootstrap, source edit or archive/retirement. Manual GitNexus UNKNOWN/HIGH annotation only; no exact-checkout impact graph was claimed. The structured format standard remains PROPOSED; no format migration.

Report/evidence publication only is authorized by claim067, SHA256 1d7b1f1686ff5a70d918cf182599a264611f4894e6c947d0ea675c4d36889099, and membership v13, SHA256 1e9edbc93cb752bfb1736e0ebaa905ae0aa860eaa7d672965a50186c27627222. This directory has at most24576 bytes/two files. Before each write, the exact authorities, held source hash, own caps, actual closed META membership and live future holds are checked. Existing reports/source snapshots remain preserved. evidence.json records source and report identities plus accounting observations; it excludes its own hash to avoid circularity.
