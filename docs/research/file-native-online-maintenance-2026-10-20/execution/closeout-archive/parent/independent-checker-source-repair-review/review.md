# Independent checker 066: full source repair review

Verdict: **REMEDIATE**. CK1-CK4 are repaired in this freeze. One additional full-proof completeness defect, CK5, remains. This is a full read of the complete CLI (466 lines), backlog (167 lines) and v6 decoder (113 lines), with the unchanged dependency context retained from the full 057 review. No decoder, engine, native API, DLL or control was executed.

Reviewed root: C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness
CLI: 33763 bytes / SHA256 186135a62d34e78b85c3a07fdff2013a3a8ac4f2c9037a1892b1d1a62d85fb37.
Backlog: 12263 / 07b2764c6e7aefbec44e3c2ff913b280fb3ab2d77f13bc5e02539461fe640116.
V6: 7162 / a355f3d524d91fd98865c35670c462c62e306d641ae52dc292a09e0467090c93.
Receipt: C:/file-native-online-matrix-20261020/receipts/independent-checker-066-CK-source-freeze.json, 9856 bytes / 76ed13569c7cc54a2f3b257d44c1007c0f776ffefa5b190a2af1e5055bfdea41. All 14 source/DLL bindings, plus the receipt's four preserved R1 bindings, match. Complete closure is in evidence.json.

## CK5 [P1] Reject directory symlinks before os.walk skips them

File: independent_checker_cli.py, lines 173-182, 189, 424, 449.

The full snapshot checks only each yielded parent and each name in files. It does not inspect the names in dirs. With followlinks=False, os.walk puts an existing directory symlink in dirs and silently declines to recurse into it. The symlink therefore receives neither the parent directory-reparse check nor the file-reparse check, and contributes no proof row. The new onerror callback does not fire for this normal skip.

Consequently, an unrelated directory symlink within an otherwise valid store can be omitted identically before and after, while snapshot returns complete=True and the final payload advertises all_files_directories_hashed=True. It can even be added/removed between these snapshots without affecting the serialized directory rows: only yielded directory names and files are recorded. This violates the advertised complete ordinary-directory/reparse-free root census. It does not require the decoder to open the unrelated path.

This is source-derived, not an executed fixture. Python 3.11.9's local standard-library source Lib/os.py lines 412-419 explicitly checks followlinks or not islink(new_path) before recursing; the CLI never validates those skipped directory entries.

Required repair: inspect every enumerated child directory with lstat before traversal can skip it, and CENSOR on a symbolic link, reparse point, wrong type, or metadata failure. Keep traversal non-following. Do not label a proof complete if any entry was deliberately skipped. Retain the existing enumeration onerror behavior and failed partial artifacts. A prospective source/control review should cover an unrelated directory symlink and equivalent directory reparse cases; none was constructed here.

## Disposition of previous findings

- CK1 [P1], repaired at source: safe_path lines 27-36 rejects parent components, checks original and resolved ancestors for symbolic/reparse paths, and resolves the result. Output confinement uses this result at 365-366. Audit paths use the same helper at 381/390. The old lexical parent-component escape is closed. Fixed os.devnull use remains explicitly scoped to legacy_full (389); there is no generic path bypass.
- CK2 [P1], repaired at source: snapshot lines 171-173 supplies an error callback that raises Censor. A scandir error can no longer silently produce complete=True. Exceptions keep before/after incomplete and technical_censor through 407-438. CK5 is a separate normal traversal skip, not an enumeration error.
- CK3 [P2], repaired at source: main retains the original bounded request bytes from 349 until index construction at 459; request_ref records resolved path, offset zero, exact length and SHA256 of those original bytes, plus requested_root. The buffer is overwritten only after that at 460. This is not a hash of reserialized request JSON.
- CK4 [P2], repaired at source: main 356-358 requires exact bool for physical_only/require_before_index and lowerhex64 string for any supplied expected_sha256. Raw expected-hash selection at 336 now uses key presence, not truthiness. These validations precede decoder activity and cannot count as semantic refusal.

The CLI additionally checks isolated and no_site at 348 along with bytecode disabled, optimization disabled and the owned-stage marker. The exact caller launch remains -I -S -B -u; this review does not validate the Native launcher, buffering, stdio, environment, or actual owned job.

## Complete flow assessment

The first-line literal CONTRACT was read directly against the receipt, including the launch_flags addition. The owner FIRSTAST claim was not reproduced: no AST or compile operation was performed. The startup gate binds nonce/PID/manifest/root and fixed deadlines before request/decoder activity; caller allocation and outside-store gate placement remain external obligations. Request shapes, limits, mode quiescence and case/scope bindings remain explicit.

SourceOnly obtains exact raw source size/hash rather than cached bytecode and all 14 bindings are recorded/rechecked. Existing companion paths were previously read for loader, codec, semantic, navigation, quota, ownership and cohort behavior; their exact unchanged hashes retain that context, not a new runtime qualification. The CLI audit blocks subprocess/exec, writes/mutations inside the resolved store, unexpected loaded source modules and unbound native libraries. DLL bytes were hashed only.

Sink checks physical part, payload-group and total limits before append, including compressed output. Failed prefixes are retained; successful artifact rename follows flush/fsync. V6 result and identity writes share a closed bounded artifact port and reserve. Actual final and partial bytes are charged. Error/normalized/progress/index reserves remain distinct; expected semantic refusals exclude Censor/OSError/ArtifactCensor, and incomplete artifacts or changed sources force technical_censor. The index has its 16384-byte cap and original request binding. These are source assessments, not measured artifact tests.

RangeRef validates closed shape, exact byte range/hash and mandatory final LF, including offset zero. Full proof is used for full/history_oracle/legacy_full/refuse/raw/entry; mapping/backlog advertise actual-open footprint proof instead. CK5 prevents accepting the full proof completeness claim. The unchanged quota NativeSnapshot is not a substitute for CLI coverage.

Full semantic paths call v6 or the fixed legacy port, check mapped keys, and return actual semantic/navigation/request-intent/cohort/quota results. Ordinary raw/entry retain full pack/cohort validation. The explicit physical-orphan route is limited to raw/entry and marks cohort_pass=False. Refusal matching remains restricted to the v6 decoder stage and exact permitted exception type/message; require_before_index is now well typed. No expected refusal or actual H is inferred from configuration alone.

Mapping holds the selected shared native pin while checking selected immutable metadata/index and STORE anchors; actual handle identity is recorded. Older generation pins are probed exclusively and release failures propagate. The native implementation was reread, including HANDLE signatures and identity checks, but never invoked.

Backlog was reread in full: quiescence, complete independent pack/cohort validation, inactive registry, accepted-chain Discovery E, attempt/deferred roles, B controls, selected live exclusions, job suffix/index/raw inventory and replacement equivalence, retired extensions, garbage, aborted owner/journal/stage containers, and proposal deduplication/type/identity/bytes all remain explicit. Ambiguous ownership or native protection censors. Internally generated native older-pin evidence must be exclusive/acquired/released; busy pins do not invent protected-reader counts. Existing and absent path evidence and SELECTED/STORE anchors are rechecked as the evidence iterator closes. The PATH partition is conditional on these restrictions; mandatory query/control cases remain unrun.

V6 was reread in full: companion checks before/after, quiescent decoder/full pack-cohort verification, quota and ownership, operational semantics, navigation and request intents, NativeSnapshot unchanged check, inventory digest, bounded identities and result. It is unchanged from the 057 freeze. Old v5/quota/semantic core remains unchanged; no arbitrary callback, circular CLI import or new I/O monkeypatch was introduced. D body is separate RAM work and not part of this freeze or review.

## Authority, accounting and limits

Only data/source reads, standard-library hashing/JSON and local stdlib source inspection were used. No supplied imports, AST/compile, tests/controls, fixtures/corpus, Win32/native/DLL/runtime, Git/index/bootstrap, source edit or source copy. Only this new report/evidence pair is written. The old negative review remains immutable.

GitNexus exact-checkout impact remains UNKNOWN/manual HIGH; no symbols were edited and no commit is proposed. The structured data standard remains PROPOSED; no migration, default or owner-contract change is approved.

Budget authority 066: 3883 bytes / 1d2116dd1a699fc58dd753f1c987c4fee293969ffdc882026b070326bbbd849c. Matrix source census is 1116265 bytes /104 files including the .git pointer82; FOCUSED225383/11, QPC59673/4, OTHER831209/89. CLI fits33792 with29 bytes remaining; OTHER has279 bytes. Quota subset47932/49152. Any future repair must fit or first obtain amended authority; this review grants no edit authority.

META uses v12, 4557 bytes / 00f1472015c30bb24f555a7dcddd94c2aa12ccec2bc6a10318ac36d97dc6154b, cap11206656. Evidence records a fresh whole-membership census immediately before writing. Pending =160472 prior +812 publication +remaining Checkerrepair +remaining Runtime. Parent explicitly closed GuardR4.2/LinuxV5.3/Contextrepair/Macv36 and released their combined15866 unused bytes, so their future is zero while every physical file stays counted. Actual bytes under each open pair consume its24576 allowance once; no old closed hold or generic future reserve is revived. This pair alone is bounded24576 bytes /2 files.

After writing, the pair is hashed, META recounted with actual consumption, and all14 bindings/receipt rechecked. Source read hold releases only after these checks. Unused own allowance remains held until parent closure.

Engine0628322e4b48e0efad19cdc32238a622d4aaa7a8 remains fixed. Functional cutoff11:10:14Z and closeout13:10:14Z remain fixed. Quota70 controls and Windows qualification are unrun/pending; no Linux/Mac or runtime approval is implied. Required next step is a bounded CK5 repair, new freeze, and fresh review.
