# Frozen Linux final-suite independent source review

Verdict: READ COMPLETE; EXECUTION NO_GO. The finite eight-call v4 interface plan is reviewable source, but its stage controller is incomplete. The full 158-case engine body and final-engine build/units remain separate NO_GO decisions. Zero v4 calls, engine calls, builds, imports or tests were executed by this reviewer.

## Findings

1. **[P1] The stage deadline does not cover the complete stage.** In suite_driver.py:329, _guard_admit performs manifest/source reads, closed imports and validation before start/deadline initialization at332-333. At619-633 the guard stops and joins its timer before memory gzip closure; final inventories, raw hashes, JSON serialization and receipt write happen afterward at 639-640. A slow/stuck preflight or finalization can exceed the proposed 60s while the guard's recorded duration excludes some work. The parent confirms the proposed external launcher/config/owner manifest is not authored. Admission needs an exact reviewed bounded outer mechanism covering preflight, token wait, execution, shutdown and final artifact writes. This review does not authorize a post-freeze source fix.

2. **[P1] The final census cannot establish final resource compliance.** suite_driver.py:639 inventories the category trees after monitors stop, but does not compare those final values against category caps or prospective reservations. The newly written receipt at 640 is absent from that census. The return at642 can therefore succeed after growth missed by the last disk sample or caused by final serialization. No external post-receipt comparison/whole native+local final census is supplied. Prospective reservations alone do not close this gap. Preserve raw artifacts and require final data-only closure under an independently reviewed bounded mechanism before any PASS; the general controller acceptance is already withdrawn.

3. **[P2, full body only] Finalizer rejects valid delayed closure.** suite_main.py:195-196 requires exactly two observations and every observation empty. suite_driver.py:604-615 deliberately retains preceding nonempty observations until its final two are empty. A run that needs one or more reap observations can satisfy the controller yet fail this finalizer. This is a false-negative qualification gap; it grants no permission to omit observations or relax native closure. It does not qualify or invalidate unrun finite controls.

4. **[P2, proposal data] The copy subtotal is stale.** Current source_receipt.json runtime_closure correctly reports527316 new bytes; phased_proposal.SDK_copy still says526398, a918-byte undercount. The current40-path closure is626828 bytes, including99512 reused canonical native bytes. Reconcile the exact data-only plan and whole-root census before admission. This is below the stated640KiB copy ceiling, but is not a runtime grant.

## Reviewed identity and coverage

Read all 13 final-suite files and every one of the 40 logical closure paths in full:13 suite,5 reused native,4 shared,7 harness,11 historical research. The 38 source_files entries plus externally bound suite_manifest.json and source_receipt.json make40. All38 lengths/digests matched local canonical files or exact historical Git blobs. Sparse historical sources were read with git show at9ee0d66ec01ca15bc41f34366053fe1454ee1093; no checkout/index/bootstrap was performed. Three harness companions exist in the Matrix checkout; their exact bytes matched. Paths and all 40 digests are in evidence.json.

Current suite totals217032 bytes/13; new closure527316/35; reused99512/5; total626828/40. A sealed runtime GO manifest is one additional data path:36 new/41 logical, before separately charged phase/owner data. The additional25 Rust/Cargo source paths401579 bytes need separate engine review.

Suite algorithm bytes stayed at the supplied frozen hashes. During review, data metadata changed: initially suite_manifest.json0112c70c39b411879533e1843cd6e1d78f8cad79d46770927c0993bb97aaa5d4 and source_receipt.json6650/f62dd302ce93c44a501cb57e8677faf7d92a18ac425072491237d74105dc1456; parent then supplied revised manifest5daad2c9cf51387edb87437eccc42a564b1c8d064d316b6b3b13ac74d5e71469 and receipt7968/1c7a4abaa49c7040122597a9e3655391006d7f377f8bcc9769104806e0dcf493. Both revised files were reread; the final hash pass binds the revised set. The initial metadata identities are superseded.

## Finite eight-call control source assessment

suite_extensions.py:414-532 contains a finite interface-only sequence, with source omission tests for all 38 entries, wrong suite_driver digest and wrong owner policy before peers. It requires the existing fa2b5ae4 ELF at the exact native-peer-v3-targeted path and distinguishes its efd07cb5 producer from the future harness pin. The C peer source was read additionally; it provides the ready/release and early/partial stdin behaviors, is not an engine, and its current digest matches the prior build binding.

The proposed calls are normal full stdin, semantic exit2, valid early JSON with zero stdin sent, partial-write BrokenPipe, explicit86, reader cancellation, reader death and packing/maintainer death. Resolve is capped at7 and packing/build at1; a counter enforces8, no retries. Incomplete input remains a refusal despite valid JSON/exit0; resource censors cannot masquerade as semantic exit2. The three interruptions require real ready markers, positive native observations, retained token/PID/start identity, owned pidfd signaling and joined owner/callback/monitor state. The packing peer verifies transport interruption only; it proves zero engine maintainer obligations.

JsonDriver keeps a new v4 owner/source hash rather than relabeling unchanged4dd. Old4dd remains the gated child/identity utility and its source is checked. Exact input wire bytes are reserved and spooled before stdin delivery; output chunks retain offsets, base64 bytes and hashes, with per-call leases and final registry. Native v3 code preserves full-write counts, written-prefix hashes, explicit close/EOF failures, retained failure rows and sole reaping ownership. Reservations check claimed operation templates and actual rooted inventories; no deletion credit is inferred.

The 8 constructors alone have no additional identified blocking logic defect. They have NOT run under v4. An exact sealed claim, source/tool/ELF binding, closed owner manifest, argv, current96MiB policy, reserve/fresh whole-source census,20GiB free-floor, host3GiB and individual2GiB enforcement, target20ms sampling with recorded cadence, final two numeric empty observations, raw EOF closure, retained censors and fresh final census remain required. The missing launcher plus findings1-2 prevents admission.

The receipt's proposed F12 arithmetic is4097964 bytes/94 files: existing691320/62 +1677722/14 native +1677722/14 local return +51200/4 phase data, within4194304/96. It is a proposal and must be refreshed against all physical membership. No640MiB fixture grant is inferred from an allowed policy option; finite controls select96MiB only.

## Historical five-call raw evidence

Independently verified the original9 returned files298158 bytes against all recorded lengths/hashes; parent-verification.json is an additional 2620-byte local record, making10 local paths. Parsed14 events, exactly5 positive native identities and5 final calls. Rebuilt each exact ordered input wire and written prefix, and reconstructed all base64 stdout/stderr chunks by offsets/digests. Normal139/139, semantic140/140 exit2, early0/139, partial65536/132248 BrokenPipe, explicit141/141 exit86 all match. The two incomplete-input violations are retained.

Strictly decompressed a single complete gzip member containing50 memory rows with no trailing member/data. Receipt reports maximum monotonic sampling interval20.253ms and zero gaps over40ms; independently observed epoch gaps reach27.914ms. Raw rows contain epoch rather than monotonic sample times, so the20.253ms value is receipt-derived, not independently reproduced from monotonic rows. Sampled host RSS peaks112857088 bytes; minimum recorded disk free79113469952 bytes. Checked positive PID/start/binary equality, joined callbacks/monitors, retained registries and two numeric empty closure observations. These are historical v3 interface results only; they do not prove v4 cancellation/death, controller-wide resource coverage or any engine case.

## Full suite and build boundary

The default manifest remains go=false, final_pin=null, native policy reviewed=false; exact version/checkpoint/compatibility/boundary/oracle release and retained-growth fold are unresolved. The158 required IDs,31 actual unit functions,17 ordinary routes, donors and final two-pass drain obligations are retained. Coarse growth estimates cannot admit the full body under96/512/640MiB. The independent Python decoders are full quiescent oracles, not Rust reader memory proof. No legacy path, engine audit or old unit pass substitutes for final-source evidence.

The separate proposed offline final-engine copy/build/unit plan needs its own F01/target/temp/fixture/toolchain/cache admission and review. This source review grants no copy, build, unit run, full suite run or final ledger PASS. Keep the frozen source and all negative/censored evidence intact.

## Review method and report allocation

Static full read and standard-library data/hash parsing only. No private module import, source execution, syntax control execution, native execution, build, Git mutation, new chat or subagent. No function/class/method edited; GitNexus exact-worktree coverage remains UNKNOWN without indexing. The structured-data standard remains proposed and grants no migration authority.

Only this review.md and evidence.json are authored under the existing20480-byte/2-file META reservation. A fresh whole P plus closed Landing membership census and remaining forecast is recorded in evidence.json; source originals remain excluded by the published membership. The Linux allowance is consumed once, with all other pending amounts conservatively retained. No reserve increase, deletion, expiry or relabel credit.
