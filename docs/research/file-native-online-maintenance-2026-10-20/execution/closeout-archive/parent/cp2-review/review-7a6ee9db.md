# Independent remediation review: exact 7a6ee9db

**Bounded engineering disposition: INTEGRATE exact `7a6ee9db03f5cd2e388129a9019c0d5f274a1844`. Full experiment qualification: INCOMPLETE.** Both required findings from b876 are addressed; no additional blocking defect was established in the complete remediation diff and its causal neighbors. This approves the engineering source handoff, not an automatic merge, production adoption, resource admission, clock reset or completed experiment.

Source was reviewed against `b876df9e01957be29b5c9cbd676d5a169aaa6c67` in `C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook`. HEAD and clean status were checked; all24 checkpoint2 source-binding file sizes and SHA-256 values independently match. Bound build3 binary:3,100,160bytes, SHA-256`6f77de3001f488d6dd57317b04f33f216856f11fbfd65d8236e8b45d28ebc4b5`. All new parsed engine events carry that exact SHA. Full new README/source binding and the entire two-Rust-file diff were read, with rolling eligibility/interval validation, construction/benefit policy, abandoned ownership/adoption and consolidation neighbors. The preceding review-b876df9e.md remains the full controlling-doc and unchanged causal-path review; its original disposition/evidence are preserved.

No engine/Cargo/oracle/source-edit/fixture-copy/indexing/spawn action occurred. Only the two newly authorized review artifacts were written. GitNexus remains UNKNOWN/unregistered and manual risk remains HIGH across discovery, all eligible-record lookup, immutable Work selection and cleanup. No graph result is claimed. The code-review skill's standards/specification checks were applied locally under the explicit no-spawn instruction. The proposed structured-data standard and owner/versioned contracts remain unchanged. Static-read completion and freeze release were communicated to meta before final evidence parsing; no source was reread after release.

## Findings disposition

All line citations below are relative to `docs/research/file-native-incremental-engine-experiment-2026-10-18/` at exact7a6ee9db.

### F1: addressed — permanent exclusions no longer occupy retry capacity

`src/rolling.rs:47-60` preserves temporary reservation deferral, but terminally skips retired attempt roles and counts/skips immutable oversized role bytes. Neither branch sets deferred bits. `discover:85-89` rechecks and removes an existing deferred item once it becomes terminal or mapped; it cannot perpetually requeue the prior retired/oversize masks. Completed/partial progress still advances only with the immutable Work selected beside the physical mapping. The existing fixed cohort, exact predecessor/selector/sequence checks, 128deferred/32KiB discovery/128KiB Work bounds remain intact.

Retirement remains authoritative for candidate/allocation exclusion; publication/result/hint evidence is still derived from accepted history and retained. Already mapped retired metadata stays physically selected until ordinary consolidation/lifetime work permits removal; this fix does not resurrect its snapshot. Oversize accepted records remain loose under the explicit terminal policy and are charged to per-operation exclusion counters. Those counters are observations, not a new durable per-record registry or a claim that excluded records have been packed.

Focused actual proof on the new pin:

- 131accepted publications with130legally retired old selectors before bootstrap; discovery visits131, deferred0.
- Another132real accepted publications; a two-publication partial discovery is selected, then old/current history is retired, including130still-undiscovered suffix publications and already mapped metadata. Remaining traversal visits130 and completes with deferred0. The corpus has263accepted publications and262retirements in total.
- The script checks every one of the263exact accepted outcomes after cleanup; independent parsing matched264resolve results including the repeated initial control, with zero mismatches. Two point reads of retired selectors return `retired:snapshot`, so retention of outcomes does not restore content access.
- A genuine selected-before-transfer death retains reservation mask24; a later publication releases transfer, deferred visit1 clears the mask and cleanup drains. Temporary eligibility was not lost while removing terminal holes.
- One labelled structural candidate control receives9MiB of JSON whitespace with unchanged JSON meaning. It is excluded once, leaves queue0, is not revisited on the no-delta build, and remains present until legal retirement/semantic collection. A129large authentic oversize corpus was not run; this single synthetic-control branch test is not that qualification.

The structural terminal policy plus actual >128retired progress discharges the perpetual-queue defect. Full final-pin lifecycle/crash/independent-oracle matrices remain separate.

### F2: addressed — ordinary policy refusal retains a recoverable owner state

`src/packing.rs:568-578` counts actual removable regular loose files, excluding B publication controls, and refuses fewer than128 before creating a new journal/birth/output. Discovery and possible consolidation reads still occur before that decision; this is an ownership/output preflight, not a promise of zero I/O.

For later ratio/charged-benefit rejection, `packing.rs:605-609` durably saves `status:"aborted-owned"`, actual used_bytes and unchanged expected-parent ownership before returning refusal. At that point target/catalog are absent and selection has not happened. `abandoned_owners:279-287` retains this birth plus inherited owners; `run_inner:550-552,586-588` permits the next ordinary build and transfers those owners; `inherit_aborted:289-320,623` flattens their exact file/metadata cleanup into the selected successor Work. No manual abort or re-adoption of retired instance paths is needed. An interrupted atomic journal write retains an owned prior state; it does not infer selection or credit deletion.

Actual unforced sequence on the same new-pin store:

1. H1 low-count automatic refusal: no PACKING owner is created; exact STORE/SELECTED and initial result checked by the harness.
2. H31 passes count but legitimately fails charged benefit after construction; journal is aborted-owned, with omitted optional target. The first harness then fails because it expected an explicit null target. Raw failure is retained.
3. Adding120real publications reachesH151. Another ordinary unforced build again legitimately fails positive benefit. This was a mistaken harness success expectation, not an unfinished-maintenance refusal; the new candidate retains both durable owners. The second failure is retained.
4. Adding400real publications reachesH551. The next automatic build registers successfully with `forced_correctness:false`, visits551publications, and records1,389,742charged used bytes. No manual abort and no force call occurs on this F2 store. Subsequent bounded cleanup batches remove all20files from the two declined owners, and the initial result remains exact; the engine audit succeeds.

The F2 store has45cleanup calls after successful admission, rather than a fabricated single-batch drain. Cooperative batches are distinct from an admitted maintenance pass; the final frozen drainage/work gates remain to be evaluated on their declared workloads. The present proof establishes eventual owned-output reclamation and normal automatic retry, not a new latency/admission threshold guarantee.

## Stable-base change and shared invariants

`packing.rs:164-185` now explicitly omits the earliest segment from consolidation candidates whenever multiple segments exist, then chooses up to two remaining tails by compressed byte size/segment id. Thus a small compressed base cannot accidentally become the smallest rewrite candidate while tails suffice. A one-segment reduced-limit case may still require replacing that sole segment. Existing <=64MiB raw collateral, <=16MiB conservative key bookkeeping, <=64packs and <=4segments remain enforced; failure to fit is explicit admission backpressure rather than unbounded rewriting.

Actual reduced-cap proof: initial base consists of10physical pack/index files. Across three real +16publication cycles using segment_limit3, its original paths remain selected and every file hash remains identical; final charged collateral is102,240raw bytes. The proof checks exactly that small cohort and reduced cap, not H10k/H101k amplification.

The remediation introduces no changes to segment routing/cache/native pin algorithms, STORE/Catalog versions, generation-frontier jobs, extension-specific retirement, expected semantic-head conflicts, intent/dedup resolution, permanent cursor proof, or bounded Q/W selection. The former b876 causal trace therefore applies narrowly to these byte-identical implementations; b876 runtime executions retain their old binding and are not relabelled new-pin tests. New preflight metadata checks and terminal-size reads remain outside Q/W. New exclusion counts use += when construction adds further exclusions, preserving the discovery counters. Existing whole-index cleanup remains explicitly indivisible under the cooperative50ms/1000phase budget, with800phase/35ms work cutoff; no hard native-call deadline is newly claimed.

## Evidence and preserved failures

The companion evidence-7a6ee9db.json contains independently parsed event counts/exact result comparisons, guard summaries and retained-input hashes. No source snapshots or copied raw traces are stored here.

| Retained run | Independent raw parsing | Guard and sampling |
| --- | --- | --- |
| build3 | Offline build succeeds; four small units pass, heavy cache unit filtered | exit0, no errors/censor, two empty closures; sampled aggregate peak439,410,688bytes; max interval63ms,15over40ms |
| remediate | 862events:857exit0,4exit2,1actual injected exit86; exact new binary, zero event violations. Four completed F1 cases; later optional-target assertion fails | exit1 due preserved harness KeyError; no guard errors/censor, two empty closures; peak36,446,208bytes;63ms max,3over40ms |
| finish | 122events:121exit0,1exit2; exact new binary, zero violations. H151 positive-benefit refusal under an incorrect harness success expectation | exit1; no guard errors/censor, two empty closures; peak26,329,088bytes;47ms max,1over40ms |
| finish2 | 511events, all exit0, exact new binary, zero violations. Unforced H551success/reclamation and stable-base assertions pass | exit0, no errors/censor, two empty closures; peak27,029,504bytes;63ms max,13over40ms |

Across the three new engine runs:1,495events, all exact new binary, zero event violations. Failed whole-run exits remain failures even though their completed cases and correctly refused engine operations provide useful bounded evidence. The partial remediate results file remains statusRUNNING because the harness failed; it is not a whole-run PASS. These sampled timing gaps remain visible and do not qualify a stricter sampler envelope. No independent full oracle was run or inferred from engine audit calls.

## Remaining qualification and handoff

**Engineering: INTEGRATE. Full qualification: INCOMPLETE.** Meta may carry this reviewed CP2 source checkpoint into the next already authorized stage. No additional mandatory source remediation remains from this review. Narrow missing injection variants and complete final-pin repetition remain qualification work; the report does not convert the focused proofs into an experiment pass.

Required outstanding work includes the full N64/H64 A/B three-tail and adversarial matrix, complete independent accepted-history/source/navigation/packet oracles after critical recovery, actual quota exhaustion/headroom/backlog/admission/drainage checks, final-pin Windows/nativeLinux/macOS lifetime/recovery/compatibility, heavy cache and1000activation stress, restored authenticated long-history representation, backup/restore plus accepted successor, ordinary B selected-once shell extraction, CP3 authenticated fresh-process lookup and the integrated final matrix. The earlier b87616.172-second capture-gap/two-successor result remains b876 evidence only. Existing file-gate failures and protected original runs remain unchanged.

Fixed clocks remain CP2watch2026-10-21T00:10:14Z, functional2026-10-21T11:10:14Z, protected closeout2026-10-21T13:10:14Z. Original review artifacts remain intact; all four review artifacts remain within the original12file/128KiB aggregate allowance.
