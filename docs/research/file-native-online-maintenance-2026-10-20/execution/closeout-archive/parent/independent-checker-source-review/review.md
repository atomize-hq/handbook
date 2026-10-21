# Independent checker 057: full source review

Verdict: **REMEDIATE**. The frozen CLI and backlog were read in full, as was physical_decoder_v6.py; this replaces the earlier repair-focused review. Four actionable findings remain. No runtime, Native integration, control, corpus, or platform qualification is granted.

Reviewed source: C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness/independent_checker_cli.py (32761 bytes), independent_backlog.py (12263 bytes), physical_decoder_v6.py (7162 bytes). Exact hashes and all 14 source/DLL bindings are in evidence.json. Receipt: independent-checker-057-review-repairs-source-freeze.json, 9789 bytes, SHA256 26e3ceef4a81be77036b8601475159b291aa3a9cd90333a0f7346cad63e18778. Engine pin stays 0628322e4b48e0efad19cdc32238a622d4aaa7a8.

## Findings

### CK1 [P1] Canonicalize output and audit paths before permitting writes
CLI lines 27-33, 358-359, 374, 382-383.

The store root is resolved, but the output and audited paths use Path.absolute() followed by lexical is_relative_to(). safe_path() rejects reparse ancestors but does not reject parent components or canonicalize the path. For example, with existing ordinary directories, root C:/owned/store and output C:/owned/other/../store/result.json pass the lexical outside-store check while the OS targets the store. Derived artifact and partial paths inherit the escape. The audit hook repeats the same lexical check and therefore does not stop these writes. A later changed-store CENSOR cannot undo mutation.

This is a static path derivation, not an executed fixture. Independently inspected Python 3.11.9 Lib/pathlib.py lines 971-979: absolute() explicitly does not normalize, and returns an already absolute path unchanged. Remedy: resolve/reject parent components consistently for output, artifacts, and every audited filesystem path before any write; enforce outside-root confinement on the canonical path and retain the reparse checks.

### CK2 [P1] Fail closed when a full snapshot cannot enumerate a subtree
CLI lines 168-185, 416, 441.

snapshot() calls os.walk(root, followlinks=False) without onerror. The default walk suppresses scandir errors and skips the failed subtree. An unreadable unrelated directory can consequently be omitted in both passes while complete=True and equal state hashes are returned. The final payload can then assert all_files_directories_hashed=True and an unchanged full root despite missing coverage. Geometry ceilings do not establish completeness.

Remedy: an enumeration error must raise Censor and prevent a full unchanged PASS; retain whatever partial evidence was physically written. The unchanged quota companion's NativeSnapshot also uses a walk without onerror, so its existence does not close this gap. That companion is outside the authorized 057 edit scope; repairs must respect the frozen contracts and obtain any necessary authority rather than silently editing old core.

### CK3 [P2] Bind the original request bytes into the durable index
CLI lines 345, 451-456.

The CLI reads original request bytes but does not persist their byte count/SHA256 or request reference in the index. Its stdout identifies only the output index, hash, and status. Case/scope identifiers and the source map do not bind the actual keys, limits, expected refusal, root, or byte-identical request consumed. This leaves the required request-original-SHA proof join absent.

Remedy: hash the original bounded raw bytes before they are overwritten, and put that digest and byte count in the durable index with an unambiguous request reference/root binding. A digest of reserialized JSON would not prove original byte identity. A caller's planned arguments alone do not prove which bytes the checker read.

### CK4 [P2] Validate mode field types before choosing verification behavior
CLI lines 282, 332, 347-351, 431.

Mode key sets are closed, but these security-relevant values are not typed: physical_only is used by truthiness; require_before_index is enforced only for the singleton True; expected_sha256 uses a truthy fallback. Thus physical_only=1 or a nonempty string selects the physical route, require_before_index=1 skips the index-open constraint, and a present empty/zero expected_sha256 can be treated as missing and replaced by the descriptor hash.

Remedy: require exact bool values for physical_only and require_before_index, and, when supplied, require expected_sha256 to be a lowercase 64-hex string. Reject malformed fields before decoder activity. Keep these technical validation failures distinct from a matched independent semantic refusal.

## Full flow assessment

The first-line literal CONTRACT was read directly and compared with the receipt; no AST/compile validation was performed. The CLI gate precedes request/corpus/decoder activity and binds nonce, PID, root, manifest echo, ready/release freshness, and fixed deadlines. The caller must continue to supply the exact isolated launch (-I -S -B -u) and owned job: the CLI checks bytecode/optimization/job flags but does not itself prove every caller flag. Mutable Native source was not read.

SourceOnly reads exact source size/hash rather than ordinary cached bytecode. The 14-entry closure includes the fixed DLL and relevant independent semantic, navigation, codec, ownership, and quota dependencies; all were hashed, and their relevant paths were read. Companion imports are normal named imports covered by the loader. Audit policy blocks subprocess/exec, unexpected source modules, and unbound DLLs, subject to CK1. No supplied module or DLL was loaded during review.

Output accounting checks per-part and aggregate limits before physical writes; gzip charges compressed bytes; completed names follow flush/fsync, and failed prefixes remain partial. V6 identity/result streams share the bounded artifact port and reserve. ArtifactCensor/OSError/Censor cannot count as an expected semantic refusal. The legacy v5 output is routed through fixed os.devnull and the returned result is republished through the bounded port. Output limits, source changes, unexpected modules, and incomplete artifacts censor the result.

RangeRef reads validate normalized path, same-handle identity, range length/hash and mandatory LF; offset zero is supported. Full proof applies to full/history/legacy/refuse/raw/entry. Mapping/backlog explicitly use the actual-open footprint; this is not a full-root unchanged claim. CK2 prevents accepting the full proof as currently implemented.

Full/raw/entry paths retain pack/cohort validation unless physical_only is explicitly selected. The physical orphan route reports cohort_pass=False and is limited to the intended raw/entry route. Expected refusal matches decoder-stage exact type/message and eligible exception classes; technical failures remain censoring. The before-index requirement is additionally subject to CK4.

Mapping holds the selected native shared pin while checking selected immutable metadata/index and store anchors, and records actual handle identity. Older pins are probed exclusively and released through the native wrapper. Backlog requires quiescence, independent full pack/cohort validation, no active registry owners, actual native probe coverage and anchor/evidence rechecks. Busy older pins cause CENSOR, not an invented protected-reader fact. Native absence/exclusive/released evidence is internally produced, not accepted as a request object. P0 counts are conditional on these restrictions and still require runtime controls.

The entire backlog algorithm was reviewed: independent Discovery E walks all accepted generations and covers attempt/deferred scope; selected live/current roots and loose publication controls are excluded; validated unmapped roles can enter U; C_W suffix/index/pack evidence and selected replacement equivalence are checked; retired, garbage, aborted journal, owner, directory and stage cases are independently classified; ambiguity censors. Proposal identity/kind/bytes and absent/existing evidence are rechecked. Mandatory query occurrences and all claimed control cases were not run.

V6 was read in full. Its diff against the preserved Landing version is limited to engine pin 062 and optional bounded identity/result output plumbing; semantic, cohort and ownership checks remain inherited. Relevant quota/operational/navigation/outcome paths and codec DLL resolution/frame restrictions were inspected. Direct v6 launch without the optional port retains its old behavior; this review covers the CLI's bounded invocation. It is not a new qualification of unchanged companion code.

## Method, limits, and budget

Only standard-library data/source reads, SHA256, JSON, diffs, and local standard-library source inspection were used. No supplied-module imports, AST, compile, tests/controls, corpus/fixtures, native calls, DLL loads, engine execution, Git/index/bootstrap, source changes, or source copies. The only authorized writes are this review/evidence pair.

GitNexus exact-checkout analysis was unavailable: blast radius remains UNKNOWN/manual HIGH. No symbol was edited and no commit is proposed. The structured-data standard remains PROPOSED; this review does not approve a format migration or new default.

Matrix source budget 057 remains 1115263 bytes /104 regular files, including its 82-byte checkout .git pointer; FOCUSED 225383/11, QPC 59673/4, OTHER 830207/89. All three reviewed sources fit their individual caps. The budget authority itself is 4026 bytes, SHA256 cafdaca13111ca5ccd361a44e5ff4eda8910831890d680e2ebdc801f1d448fce.

Immediately before writing, the entire META v9 physical membership is censused again and all 14 frozen bindings plus receipt rehashed. Its cap is 10878976 bytes. Pending reservation is exactly 136381 other +24576 Guard R4.1 +24576 this pair =185533; no obsolete generic runtime/future reserve is added. Evidence records this prewrite census. After writing, the pair is hashed, its combined <=24576 allowance checked, META recounted, and the remaining own allowance is reduced once by actual pair bytes. Source read hold is released only after these unchanged-hash and budget checks.

Required next step: authorize bounded repairs for CK1-CK4, refreeze, and obtain a fresh review. This result is REMEDIATE, not runtime GO. Windows qualification remains pending; no Linux/Mac approval, deadline reset, or engine-pin change is implied.
