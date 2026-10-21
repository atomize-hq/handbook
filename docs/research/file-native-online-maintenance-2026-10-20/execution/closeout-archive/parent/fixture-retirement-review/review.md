# Fixture retirement — independent source review

**REMEDIATE. No import/control/native/archive/restore/delete/Git GO.**

Fully read archive.py, controls.py, README.md and source-receipt.json in QL = C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/fixture-retirement. Exact supplied lengths/hashes match: 64707 bytes/four flat regular files. Budget 033 is 2087 bytes, SHA256 b503452ea68a514c6aecb34086bbf947eef08ec446d066caa7c801760159cc4a. Source acceptance is blocked by the concrete findings below, not merely missing native hooks.

The structured-data standard was read and remains PROPOSED. FNFXAR01/JSON receipts are private experiment contracts; no owner format migration is approved. Transitive review covered the dynamic-import route, caller Context interface and published Python/POSIX/Win32 ABI documentation. This freeze supplies no native Context implementation. All counterexamples are static inferences; no reviewed code was imported, parsed/compiled or executed, and no source was edited. GitNexus impact is UNKNOWN; no bootstrap/Git operation occurred.

## Required findings

### F1 [P1] Same-device bind mounts escape protected-root isolation

archive.py:157–166 compares protected paths lexically; nodes:438–458 rejects only changed device at 443. A pre-existing Linux bind mount of a protected sibling under an admitted root can have the same st_dev, ordinary directory mode and unchanged link counts. O_NOFOLLOW does not reject a mount.

Example: eligible /fixtures/new contains mirror, a same-filesystem bind mount of excluded /fixtures/baseline. A single-link baseline file passes device/link checks; census, capture and rewalk all agree. retire:893–938 then unlinks children through mirror's directory fd, removing baseline children before mountpoint rmdir fails. A namespace freeze prevents changes, not pre-existing aliases.

Reject mounted descendants/file aliases, including same-device bind mounts, before capture/unlink, or explicitly enforce a reviewed native mount/protected-identity proof. Bind root/descendant mount identities and refuse absent proof. Add a separately admitted protected-bind negative. This does not select the parent's future allowlist.

This trace is inferred from the helper and Linux [bind-mount semantics](https://man7.org/linux/man-pages/man2/mount.2.html); [statx](https://man7.org/linux/man-pages/man2/statx.2.html) distinguishes mount identity from device identity.

### F2 [P2] External-alias checks omit SPECIAL nodes

source_events:467–470 checks links>1 only inside FILE and counts only FILE aliases. A hardlinked symlink/FIFO with another name outside admitted roots passes archive/verify and can be retired, contrary to the documented unknown-external-alias refusal. Linux [link/linkat](https://man7.org/linux/man-pages/man2/link.2.html) supports nondirectory links and hard links to symlinks themselves.

Check identity/link closure for every supported hardlink-capable nondirectory kind, or refuse multiply linked SPECIAL nodes. Do not apply this rule to directory nlink. Add external special-alias negatives/internal closure cases where supported; C13/C14 currently cover regular files only.

### F3 [P2] Finite-control admission has no enforced result contract

controls.py:199–208 only checks that admit_controls exists, discards its result, imports archive.py and creates the root. before_control_write:215–218 also discards its result. A negative receipt/False/None therefore does not stop imports/prefix writes before archive.py's later Context.check.

Require positive bound admissions before import/first root write and each case, or explicitly select a reviewed pinned raise-on-refusal ABI. A throwing ABI is valid, but neither that contract nor its implementation is established here. Add failed/negative admission cases proving zero import and zero prefix creation. This concerns unchecked admission semantics, not the seven honestly pending native hooks.

### F4 [P2] Host-review decode uses bytes other than those hashed

evidence:210 hashes a referenced pathname; 212 reopens host_review and decodes that second read without verifying its hash. An edit/replacement between reads can substitute unpinned complete/case/closure flags. The declared source-root freeze does not itself establish an immutable handle for the separately located authority document.

Read bounded no-follow host-review bytes once, check their exact SHA and decode that same buffer, as load_plan already does. Establish a reviewed immutable-reference contract for the other authority files. Add a hash/decode substitution negative.

### F5 [P2] Foreign POSIX name validation uses the inspector's codec

unpack_name:107–117 calls os.fsdecode for POSIX bytes on a foreign inspection host. A legal Linux FF component encoded as /w== fails on normal Windows Python's UTF-8/surrogatepass codec. Raw bytes are retained, but the README's foreign-host metadata inspection promise fails. This is inferred from [PEP 529](https://peps.python.org/pep-0529/), not executed.

Validate POSIX components as bytes independently of the inspector; select explicit lossless display decoding. Reconstruct native paths only on the matching host. Add foreign non-UTF8 POSIX and Windows UTF16/surrogate decoder cases.

## Codec, filesystem and crash assessment

The main byte path is coherent on inspection: MAGIC8/<BIQ>, closed canonical ASCII JSON, exact FILE bytes/FILE_END digest, SPECIAL forensic bytes, sorted DIR/DIR_END hierarchy, END counts/tree/prefix hashes, full source census and lockstep rewalk. Empty directories and every regular hardlink alias's full bytes are retained. StrictReader checks zlib integrity, one member, inner/outer EOF and compressed SHA. Capture checks compressed/inflated caps. None of this is an executed archive or restore.

64 KiB chunks/name pages, bounded metadata and depth<=64 avoid a whole-tree inventory. Repeated directory paging and alias rescans can be expensive; names() and sha_file do not poll during a whole scan/hash. Independent native deadline/RSS enforcement must cover these gaps. archive_events is read-only, without generic extraction; normal compare checks compressed size, while direct parser calls need an admitted compressed-size/deadline policy.

POSIX opens regular files/directories with O_NOFOLLOW/O_NONBLOCK and anchored parent fds; devices/FIFOs are not read as data streams. Nonempty xattrs/forks, or absent absence proof, refuse. Windows uses OPEN_REPARSE_POINT/BACKUP_SEMANTICS, same-handle FileId/Basic/Standard/Stream metadata and raw reparse bytes. Nondefault ADS refuse. Unsupported native queries must refuse rather than silently omit metadata. The format retains declared NODE_KEYS/forensic bytes; it is not a portable ACL/UID/GID/security-descriptor backup or generic restore utility.

Windows unlink_expected:809–833 verifies content on a read handle, closes it, then opens a new read+DELETE handle, validates identity/selected metadata and calls FileDispositionInfo. Different-ID replacements are checked on the actual disposition handle. The verified-content handle is not retained through disposition: same-ID content changes during the gap are not excluded by that ID check (ctime is deliberately omitted). Before native qualification, verify bytes/special data on the retained DELETE handle or prove the real all-writer freeze across the reopen gap. Directory handles alone do not freeze child namespace writes.

Published [CreateFileW sharing/flags](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew), [FileIdInfo](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info), [FileStandardInfo](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_standard_info) and [FileStreamInfo](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_stream_info) support the inspected access/layout choices. [FILE_DISPOSITION_INFO](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_disposition_info) uses one-byte BOOLEAN; [SetFileInformationByHandle](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileinformationbyhandle) requires DELETE access and marks deletion for close. The subsequent actual-absence check is appropriate. This is documentation-level ABI review, not host qualification.

POSIX deletion explicitly relies on a cooperating namespace/writer freeze: parent dirfds anchor names, but stat/unlinkat is not an atomic expected-inode operation. A leaf can otherwise change between stat and unlink. The concrete Context must prove all-writer closure, ancestry/mount/alias scope and native archive protection; unknown state must refuse. This absent dependency may remain unqualified in source. The honestly recorded special-node guard-census incompatibility also needs reviewed accounting before execution.

Archive and retirement permits remain distinct/hash-bound. Full backup/source comparison precedes deletion; artifact membership rejects unknown files, links and cap excess. Parent must separately supply exact eligible roots/native IDs, complete final-source raw case/oracle/negative evidence, protected roots, a unique retained backup and funding. Old controls/CP4/baselines/build/source/rejected/LONG roots and sole backups receive no retirement authority here.

The journal fsyncs batch intent before <=4096 unlinks and commits observed ordinals. A crash after unlink but before ordinal/commit remains ambiguous; no uncommitted/reclaimed/expiry credit is granted, failure prefixes remain and an existing journal blocks automatic restart. Parent adjudication/new remaining-prefix admission is required. No resume reconstruction or power-loss durability qualification is established.

All 32 proposed controls remain unexecuted. C18/C22/C23/C27/C30/C31/C32 require actual native hooks; missing hooks or unsupported applicability stay PENDING. Toy control_only reviews cannot qualify final engine/host cases. Real race/reparse/fork/death/sharing/retained-handle schedules and raw native closures remain required. The 300 s/host, 8 MiB/256 body files and 1 MiB/40 evidence files are proposals, not admission.

## Budget and handoff

Individual caps fit: archive42695/43008, controls16222/16384, README4089/4096, receipt1701/2048. Whole64707/65536 has829 bytes slack; each actual member plus all other full caps fits. Any remediation needs bounded source authority and a fresh freeze; this review edits/transfers nothing.

Only P/fixture-retirement-review/review.md and evidence.json are added, combined<=16384 bytes. Exact meta-membership-v4.json governs ALL P, both Landing root-only sets, seven subtrees, only stream-snapshot-controls excluded, and twenty promoted harness files/203215 bytes. Original C3 source trees are excluded as stated; P copies remain charged. evidence.json records fresh whole membership plus full report allowance and 307200-byte future reserve. No compression/deletion/expiry/planned-promotion credit is used.

Resolve F1–F5 and re-review. Explicitly unqualified native dependencies may remain pending; source acceptance must never become control/archive/retirement GO.
