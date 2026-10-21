# Checker CK5: affected source review

Verdict: **INTEGRATE, source only**. CK5 is repaired in the exact 070 freeze. No new actionable finding was found in the affected review. Runtime/Native/control/platform approval remains withheld.

Source: C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness/independent_checker_cli.py
34171 bytes, SHA256 a49175b347445f79360c9ab90be043377fb8683fc6d9b0c781dce6245cfcabd5.
Receipt: C:/file-native-online-matrix-20261020/receipts/independent-checker-070-CK5-source-freeze.json
11220 bytes, SHA256 a17e874eb952895c9197e1c0afe2d63b6e806c813b1a1e4b53258023f4c49ae2.

## Scope and comparison

The entire current CLI (472 lines) was read, including every line of snapshot (168-195), its callers, error handling, output and final status. Its whole-file comparison against the complete 066 review was independently closed by a RAM-only inverse text comparison: undoing just the added stat import and snapshot changes reproduces 33763 bytes / 186135a62d34e78b85c3a07fdff2013a3a8ac4f2c9037a1892b1d1a62d85fb37 exactly. No source file was reconstructed on disk or executed.

Backlog12263/07b2764c6e7aefbec44e3c2ff913b280fb3ab2d77f13bc5e02539461fe640116 and v67162/a355f3d524d91fd98865c35670c462c62e306d641ae52dc292a09e0467090c93 are unchanged from their complete 066 reads. All14 actual source/DLL bindings and the four preserved R1 hashes are rechecked, not inferred from filenames. The full057/066 semantic, cohort, quota, ownership, navigation, loader and native source context remains applicable at those exact unchanged hashes. Separate D body work is outside this freeze and review.

## Why CK5 is closed at source

- Every name in dirs is examined before the generator yields control back to os.walk: lstat, exact directory type, symbolic-link check and Windows reparse attribute rejection (176-179). An existing directory symlink/reparse can no longer be silently excluded by followlinks=False. Metadata failures propagate.
- pending starts with the root (173). Each yielded base must remove its exact pending entry (175); every validated child is added (179). If a validated child later becomes a symlink and os.walk silently skips it, the remaining pending entry causes Censor at193. A missing/denied child that produces an enumeration error is still rejected by onerror at171/174.
- A visited parent is independently required to remain an ordinary, non-reparse directory (180-181). File type checks now use the lstat result's S_ISREG (186-187), retaining symbolic/reparse rejection. No directory following was enabled.
- Directory/path/count and file/path/logical-byte ceilings, content hashes and same-file metadata checks remain. complete=True is reached only after the evidence iterator exhausts and pending is empty (193-195). Interrupted streams retain partial artifacts through the existing Output/Sink path.
- The before and after callers (414/426) remain full proofs for full/history_oracle/legacy_full/refuse/raw/entry. A failure cannot satisfy unchanged (430/444) or the full coverage flag (455). Before/after proof exceptions are technical CENSOR, not a decoder-stage expected refusal. Mapping/backlog continue to advertise their actual-open footprint scope.

The pending set closes the normal silent-descent skip identified by CK5; it is not an atomic filesystem snapshot. Full proof still relies on the declared quiescent/owned-store protocol. No concurrency guarantee beyond that protocol is approved.

## Retained behavior and qualification limits

The byte comparison establishes that canonical safe_path/output/audit confinement, exact mode booleans and expected hash validation, original raw request_ref, RangeRef LF/offset-zero rules, isolated/no-site/bytecode/job guard, startup gate, exact source loader, DLL restriction, bounded physical artifact accounting, expected-refusal restrictions, native pin and backlog routes are unchanged from 066. CK1-CK4 stay closed at source. No engine or old v5/quota/semantic core change is included.

The first CONTRACT line was read against receipt data; the owner's FIRSTAST result was not rerun. Only standard-library data/source reads, JSON/gzip/base64 decoding, hashes and in-memory text comparison were used. No supplied imports, AST/compile, tests/controls, fixtures/corpus, native APIs, DLL loads, runtime, Git/index/bootstrap, edits or disk source copies. Only this report/evidence pair is written. Earlier negative reports remain untouched.

The receipt's compressed impact records were decoded as data: all42 owner queries failed with exact checkout not found. Graph blast radius remains UNKNOWN; manual HIGH covers the full checker and Native166 D dispatch. This review makes no graph success claim and edits no symbols. The structured data standard remains PROPOSED; no migration/default approval.

## Budget and release

Source budget070: 4547 bytes / 4b88c191040252926b6c04173068168717031407c6cbd2770ceee8e6b97957e0. Whole cap1126400/200; CLI34816; OTHER832512; FOCUSED228352; QPC65536; quota subset49152. Evidence records the fresh full Matrix census, including .git pointer82 and all physical source membership. This live census can differ from freeze-prewrite values because separately authorized D work is outside this review.

META v14: 7348 bytes / 8f186a77435afa16723514b56af168532a6fb68e895fb5f14f8b00334ac81822; cap11599872; unchanged physical selectors. Immediately before writing, census all membership and each071 pair. Pending =base112617 +actual remaining Mac return +remaining six open071 review caps. GuardR4.3 is parent-closed future0 and publication future0; all prior065/066/067 pairs remain closed. Physical files always count once; open review actual bytes consume their own allowance once. The new FinalizerRepair and Macv37 review consumption are included, not omitted.

Only P/independent-checker-CK5-review/review.md and evidence.json may be created, combined <=16384 bytes /2 files. After writing, hash the pair, recount META and recheck all14/fourR1/receipt/source-budget bindings before releasing the source read hold. Unused own allowance remains held for parental closure.

This source result can proceed to the parent's separately authorized next phase. It does not authorize runtime/checker/control/corpus/native execution, D integration, platform qualification or changed deadlines. Engine0628322e4b48e0efad19cdc32238a622d4aaa7a8, functional11:10:14Z and closeout13:10:14Z stay fixed. Quota70 controls remain UNRUN.
