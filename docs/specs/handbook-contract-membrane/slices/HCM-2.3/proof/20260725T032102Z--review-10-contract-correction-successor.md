# HCM-2.3 Review 10 contract-correction successor proof

## Proof identity

- Successor authority: the operator's narrowly bounded Review 10 successor
  authorization on 2026-07-25.
- Frozen Review 10 dispatch:
  `20260723T154328Z--HCM-2-3--fresh-atomic-publication-documentation-review-10`
  at
  `sha256:8a593688c1573cc62b4ee21b8fa279edf03b74b2b65a08f2f9eded060f355921`.
- Frozen Review 10 subject:
  `sha256:8dac11d067a4fba0efea04a3ae1435ae1fa9c8a82307171676380e9aa8f1413a`.
- Frozen predecessor proof:
  `20260723T041308Z--atomic-canonical-publication-authority-repair.md` at
  `sha256:d8d183f0f121123326eeefa38e51821c7e14b8d3e5fea66d37a7a422fee3015f`.
- Immutability rule: Review 10, its manifest/hashes, the predecessor proof, and
  every prior review/selector artifact remain historical evidence and are not
  rewritten by this successor.
- Authorized delta: only object-ID completion authority and the local
  parent-relative `NtSetInformationFile(FileRenameInformation)` call inside
  `publish_replacement`.
- Persisted compatibility rule: existing
  `setfileinformationbyhandle_filerenameinfo_*` schema values remain stable
  protocol identifiers; they do not select or authorize the Win32 primitive.
- Stop rule: no additional Rust symbol, unsafe item, public API, dependency,
  Cargo/version edit, global, helper, fallback primitive, or broader completion
  rule.
- Authority-repair dispatch:
  `20260723T041308Z--HCM-2-3--atomic-canonical-publication-authority-repair`
- Review 1 remediation dispatch:
  `20260723T054138Z--HCM-2-3--atomic-publication-documentation-review-1-remediation`
- Review 2 remediation dispatch:
  `20260723T072155Z--HCM-2-3--atomic-publication-documentation-review-2-remediation`
- Review 3 remediation dispatch:
  `20260723T082558Z--HCM-2-3--atomic-publication-documentation-review-3-remediation`
- Review 4 remediation dispatch:
  `20260723T092555Z--HCM-2-3--atomic-publication-documentation-review-4-remediation`
- Review 5 remediation dispatch:
  `20260723T101749Z--HCM-2-3--atomic-publication-documentation-review-5-remediation`
- Review 6 remediation dispatch:
  `20260723T112810Z--HCM-2-3--atomic-publication-documentation-review-6-remediation`
- Review 7 remediation dispatch:
  `20260723T132711Z--HCM-2-3--atomic-publication-documentation-review-7-remediation`
- Review 8 remediation dispatch:
  `20260723T140846Z--HCM-2-3--atomic-publication-documentation-review-8-remediation`
- Review 9 remediation dispatch:
  `20260723T150820Z--HCM-2-3--atomic-publication-documentation-review-9-remediation`
- Selected handoff:
  `20260722T042100Z--HCM-2-3--orchestration--planning-completed`
- Branch: `codex/hcm-2-3-planning`
- Pinned HEAD: `3c49fa2c6d653f4b1a703d1d5c5196147b003533`
- Role: bounded Review 10 contract-correction proof and executable gate
- Implementation status: this proof does not widen the operator's exact
  authorization

## Successor correction decision

The frozen Review 10 subject remains historical evidence. This successor
changes only two contract decisions:

1. `NtFsControlFile` object-ID absence accepts the three exact traces recorded
   above and in the runtime contract/vector object. Immediate raw
   `STATUS_OBJECTID_NOT_FOUND` is authoritative only with the complete
   initialization sentinel unchanged. `STATUS_PENDING` is never absence by
   itself and must expose a completed `STATUS_OBJECTID_NOT_FOUND`/zero
   `IO_STATUS_BLOCK`. Every partial sentinel, disagreement, success/present, or
   other tuple remains fail-closed.
2. Windows parent-relative no-replace publication uses
   `ntdll!NtSetInformationFile` with the five-parameter `system` ABI:
   `FileHandle: HANDLE`, `IoStatusBlock: *mut IO_STATUS_BLOCK`,
   `FileInformation: *mut FILE_RENAME_INFORMATION`, `Length: ULONG`, and
   `FileInformationClass: FileRenameInformation (10)`, returning signed 32-bit
   `NTSTATUS`. `FILE_RENAME_INFORMATION` is the native `repr(C)` layout
   `BOOLEAN ReplaceIfExists`, `HANDLE RootDirectory`, `ULONG FileNameLength`,
   followed by the exact unterminated UTF-16 leaf. The passed length is the
   checked `FileName` field offset plus the counted leaf bytes. All declarations,
   layouts, buffer construction, and unsafe operations remain lexical locals of
   `publish_replacement`.

The Win32 `SetFileInformationByHandle(FileRenameInfo)` call is forbidden.
Existing lowercase persisted primitive labels are compatibility identifiers,
not authority for that API. No fallback, helper, dependency, Cargo/version,
global, public API, additional unsafe item, or additional production symbol is
part of this decision.

Except for the executable checker body, the inherited sections from “Accepted
findings” through the predecessor “Disposition” retain historical Review 10
provenance and are not present-tense successor results. Where inherited prose
conflicts with this decision, this successor decision and the final “Successor
validation evidence”/“Successor disposition” control.

## Accepted findings

Review 6 correctly found a CRITICAL lost-update window in
`publish_replacement`: the implementation verifies the canonical basis and then
performs an unconditional rename. A non-cooperating writer can change, delete,
or replace the canonical file between those operations and have that newer
basis silently overwritten. Retained handles and a coarse cooperative lock do
not make the pathname rename conditional.

The finding is not waived or narrowed to cooperating lock holders. The current
Rust remains non-authoritative and unchanged. Checkpoint C is reopened.

Fresh atomic-publication documentation Review 1 then returned one Critical and
four Required findings. All are accepted:

1. reverse `ReplaceFileW(C,D,R,...)` would merge canonical metadata/streams into
   the unexpected displaced object and could leave a partial 1177 mutation;
2. exact equality between pre-call and post-rename ctime/USN made ordinary
   expected-present success unsatisfiable;
3. a retained byte-readable replacement handle conflicts with
   `ReplaceFileW`'s documented no-sharing replacement open, and the file-ID
   citation named the wrong API;
4. the required `native-publication-result.json` is not admitted by the live
   transaction-inventory allowlist; and
5. the machine subject lacked fingerprinted expected-present and outputless
   conflict chains, exact expected-absent/interference/crash rows, and semantic
   binding negatives.

Fresh documentation Review 2 returned three Required findings. All are
accepted:

1. the authority stop named only `validate_transaction_inventory`, while the
   coupled `recover_pending` and `verify_committed` changes are independently
   CRITICAL and require explicit operator authorization before any selector;
2. prose and partial vectors did not constitute an executable exact
   Unix/Windows state matrix or verify the full positive/refusal record chains
   and every declared semantic rejection; and
3. the Windows text incorrectly implied `FlushFileBuffers` on read-only
   post-move observation handles, although only the write-capable producer
   handle can flush `R` before it closes.

Fresh documentation Review 3 returned one Critical and two Required findings.
All are accepted:

1. the classifier restored an unauthenticated `D=X` object, which could make an
   attack-controlled object canonical;
2. the claimed exhaustive matrix omitted deletion and same-byte/different-byte
   substitution of `R` after observation handles close, including distinct
   no-call, failed-call, successful-call, and crash-without-return rebounds;
   and
3. twelve existing Windows destination/share/other-error claim, publish, and
   restore cells omitted the required `MOVEFILE_WRITE_THROUGH` flag.

Fresh documentation Review 4 returned one Required finding, which is accepted:
the expected-absent adapter also closes the `R` observation handle before
`R -> C`, but the allegedly exhaustive classifier and checker covered source
deletion/substitution only for expected-present. The remediation adds the exact
15-row expected-absent deletion/same-byte-substitution/different-byte-
substitution cross-product, including no-call, failed-call, successful-call,
crash-unmoved, crash-moved, and identical recovery dispositions on both
platforms.

Fresh documentation Review 5 returned one Critical finding, which is accepted:
the persisted native-version token was opaque, so a writer could label an
arbitrary same-object metadata or side-stream mutation
`primitive_mutated_same_object` and satisfy the checker. This remediation
persists the complete platform observations, derives identity/version tokens
from canonical components, restricts permitted post-move deltas, refuses
unavailable or unstable observation, and executes Unix mode/xattr plus Windows
attribute/last-write/DACL/named-stream and arbitrary-token rejection vectors.

Fresh documentation Review 6 returned four Critical and one Required findings.
All are accepted:

1. path-based Windows `MoveFileExW` did not bind the mutation to retained source
   and destination-parent handles;
2. reusable numeric IDs plus permitted version deltas did not prove that the
   destination was the same live kernel object as the retained source;
3. an all-zero Windows `FILE_ID_INFORMATION` value was schema-admissible even
   though Microsoft requires it to be ignored when 128-bit IDs are unsupported;
4. Windows Object ID presence, unsupported query, access denial, and mutation
   were not fail-closed; and
5. seven unavoidable CRITICAL implementation seams were missing from the
   preserved authority stop.

Fresh documentation Review 7 returned one Critical status/proof-authority
finding and one Required stale-blocker finding. Both are accepted. A
documentation CLEAN cannot classify or commit the unchanged non-authoritative
production implementation, close `PG-KIND-02`, or bypass the ordered
documentation review, sixteen-surface approval, selector, atomic implementation,
proof, GitNexus, and implementation-review gates. The exact registry-brief
subset retains its current `TargetOnly` baseline and zero landed evidence.

Fresh documentation Review 8 returned one Critical and two Required findings.
All are accepted:

1. all-ones `FILE_ID_128` is an unusable sentinel that remained structurally
   admissible and lacked role-specific executed negatives;
2. `CreateFileW` cannot bind a child name to a retained parent handle, so the
   prior child-open trace did not implement the claimed retained-parent
   authority; and
3. “no classification” broke replay of the frozen future
   `TargetOnly -> RealPathAdopted` registry-brief diff.

Fresh documentation Review 9 returned zero Critical, six Required, and one Nit
findings. All are accepted:

1. SPEC opening prose incorrectly presented historical implementation selection
   as current implementation authority;
2. the SPEC and plan omitted the exact with-tests and without-tests GitNexus
   commands for the six structured-observation CRITICAL surfaces;
3. the completed Review 3 todo still presented `MoveFileExW` plus
   `MOVEFILE_WRITE_THROUGH` as current acceptance after Review 6 superseded it;
4. the `UNICODE_STRING.Buffer` representation did not freeze whether the
   counted child-name buffer included a terminator;
5. a `DeviceIoControl` object-ID query could not preserve the exact raw NTSTATUS
   needed to distinguish `STATUS_OBJECTID_NOT_FOUND`;
6. retained-directory `NtCreateFile` options combined
   `FILE_OPEN_REPARSE_POINT` with incompatible `FILE_DIRECTORY_FILE`; and
7. the todo opening status sentence was stale.

## Decision

The repaired authority is `atomic-displaced-basis-v1`, a shared journal state
machine with platform adapters:

- `C` is `.handbook/project/registry-brief.yaml`;
- for promotion transaction `T`, `R` is
  `.handbook/project/.registry-brief.yaml.generic-publish-<T>.candidate`; and
- `D` is
  `.handbook/project/.registry-brief.yaml.generic-publish-<T>.displaced`; and
- expected-present `B` is
  `.handbook/state/transactions/artifact-promotions/<T>.pending/expected-basis.backup`.

`R`, `D`, and `B` are bounded journal evidence, not canonical truth, semantic
records, subordinate closure, or disposable scratch. `B` is copied from the
retained expected-basis handle before the native boundary and binds exact bytes,
length, and structured metadata fingerprint. It is never automatically restored,
canonicalized, deleted, or cleaned.

The durable pre-boundary `native-publication.json` binds the protocol/platform
adapter, expected nullable basis bytes/fingerprint/length/identity/complete
platform observation/digest-derived pre-call version, exact `C/R/D/B` refs, and
replacement bytes/length/identity/complete observation/derived version. The
durable post-action
`native-publication-result.json` binds the exact native-call trace and, for
every move, the retained-source, retained source/destination-parent, relative
destination-open, same-live-kernel-object proof completed before result/marker
construction. Reopened numeric identity and permitted version deltas are never
substitutes.
Its fingerprint is repeated by promotion marker and internal evidence.

Native return values are diagnostic:

- Unix uses retained source and parent descriptors with
  `renameat2(..., RENAME_NOREPLACE)`, proves the destination from the retained
  parent is the same live source object, and syncs the parent;
- Windows retains every ancestor without `FILE_SHARE_DELETE`, enumerates each
  child, and opens every child plus the post-rename destination rebound with a
  private `ntdll!NtCreateFile` call whose `OBJECT_ATTRIBUTES.RootDirectory` is
  the role-specific retained parent and whose `ObjectName` is one validated
  child backed by a non-null unterminated counted UTF-16 buffer: even nonzero
  `Length <= 65534`, `MaximumLength == Length`, exactly `Length` bytes, and no
  terminator. Retained-directory options are exactly
  `FILE_OPEN_REPARSE_POINT | FILE_SYNCHRONOUS_IO_NONALERT` (`0x00200020`)
  without `FILE_DIRECTORY_FILE`, and post-open observations must prove directory
  type and reject every reparse point/tag. Regular-file roles use `0x00200060`.
  The exact ABI/layout/attributes/disposition/options/access/share/
  `NTSTATUS`/`IO_STATUS_BLOCK` trace is machine checked; `CreateFileW` is
  initial-root/volume-only and cannot prove a child open. Windows then invokes
  `NtSetInformationFile(FileRenameInformation)` on the DELETE-capable retained
  source with `ReplaceIfExists=FALSE`, retained destination-parent
  `RootDirectory`, and a simple child name;
- every destination is opened from that same retained parent and compared to the
  still-live source before completion; and
- destination interference never overwrites, while any parent/ancestor/source
  binding loss is mutation-free ambiguity without fallback.

Only `C=replacement, R=absent, D=expected-basis` for expected-present, or
`C=replacement, R=absent, D=absent` for expected-absent, plus the completed live
continuity record, authorizes a commit. A pre-call conflict proved while the
source remains live may terminalize outputlessly. A crash or any post-call state
without the durable completed continuity record is markerless, outputless,
non-retryable `refused_ambiguous`, even when bytes, numeric identity, and allowed
version deltas look exact. It permits no recovery move, restoration, adoption,
deletion, cleanup, result, or marker.

`ReplaceFileW` is rejected for both forward and reverse publication. Its
documented successful behavior merges creation time, DACLs,
encryption/compression, object identifiers, and named streams into the
replacement, while error 1177 can leave a partial merge. Those objects are not
canonical HCM-2.3 outputs, so merely recording the post-version would not
authorize them. Its 1175/1176/1177 outcomes are consequently rejection evidence,
not live protocol vectors.

## Reader and crash authority

Authorized readers acquire repository/generic locks, inventory every promotion
journal naming `C`, and complete recovery before opening the canonical path.
They never expose a markerless replacement or unresolved forward/rollback
state. A promotion-installed value requires the exact intent, pre-boundary
observation, verified stage, authorized native result, marker, evidence,
domain result, and retained-ledger chain plus a final no-follow
bytes/identity/path-to-handle check. A later external canonical edit becomes a
new current basis only after the older protocol has no pending state.

Unix syncs the destination parent after each successful rename. Windows does not
invent directory-fsync or write-through rename semantics. The producer flushes
`R`; the DELETE-capable source and retained destination parent remain live
through destination observation and proof construction. A crash that loses
those handles before the completed result is durable retains `C/R/D/B` and
refuses ambiguously; recovery cannot reconstruct continuity.

Windows identity uses volume serial plus complete nonsentinel `FILE_ID_INFO`.
Its retained observation issues `FSCTL_GET_OBJECT_ID` through a private
synchronous `ntdll!NtFsControlFile` ten-parameter ABI: the retained regular-file
handle; null Event/APC fields; initialized `IO_STATUS_BLOCK`; null/zero input;
and exact 64-byte `FILE_OBJECTID_BUFFER` output. Absence has three exact traces:
immediate raw `STATUS_OBJECTID_NOT_FOUND` with the complete I/O status block
unchanged at its initialized `STATUS_UNSUCCESSFUL`/`ULONG_PTR_MAX` sentinel;
matching raw/final `STATUS_OBJECTID_NOT_FOUND` with zero `Information`; or raw
`STATUS_PENDING` completed through the I/O status block as
`STATUS_OBJECTID_NOT_FOUND` with zero `Information`. Pending with either
sentinel unchanged is incomplete and fails closed. Dual success with
`Information == 64` means present and refuses; every other sentinel,
disagreement, status, or length fails closed.
`DeviceIoControl`/`GetLastError` is not raw-NTSTATUS authority and no fallback
exists. Present Object ID, unsupported query, access denial, mutation, zero or
all-ones file ID, or any other result fails closed.
Its structured observation additionally binds regular/reparse status, length,
link count, creation/last-write/change time, attributes, owner/group/DACL
digest, exact default-only named-stream inventory, and file USN. SACL is
explicitly excluded from observation, hashing, and canonical-byte authority.
Unix binds
retained-fd `statx` device/inode/type/length/link/mode/uid/gid/mtime/ctime and
two identical complete, raw-name-sorted `flistxattr`/`fgetxattr` passes.
Missing, unreadable, incomplete, or unstable required native support fails
closed.

`native-version-v1` is the SHA-256 of the RFC 8785 canonical complete
observation, not writer input. After a traced successful move, only Unix ctime
or Windows change time and USN may differ. All other components are exact move
invariants. `FSCTL_READ_FILE_USN_DATA` supplies only the last USN here; its
returned `Reason`, `TimeStamp`, and `SourceInfo` are documented invalid and
provide no causal authority. Metadata/xattr/security/stream delta before the
first move invokes no move; after a move it permits no further move or marker,
retains observations and `C/R/D`, and withholds readers as
`refused_ambiguous`.

## Rejected alternatives

- Verify then ordinary rename: retains the Review 6 lost-update window.
- Trust the native return value: Windows documents partial error states, and a
  crash can lose the return.
- Trust a reopened inode or Windows file ID: numeric identities can be reused
  and do not prove source-to-destination continuity after handle loss.
- Restore or canonicalize from `expected-basis.backup`: turns evidence into
  mutation authority; the backup is retained only for audit.
- Use `ReplaceFileW` in either direction: introduces unauthorised metadata/
  stream merge semantics, a no-sharing handle conflict, and partial 1177
  mutation.
- Delete candidate/displaced residue after success or conflict: widens cleanup
  authority and destroys recovery evidence.
- Restore, adopt, delete, or clean `D=X`: turns unauthenticated evidence into
  authority; every such tuple is mutation-free `refused_ambiguous`.
- Treat locks as the threat boundary: fails the explicit non-cooperating-writer
  requirement.
- Add a check-only Rust repair: does not make the publication boundary
  conditional and is outside this documentation dispatch.

## Primary native references

- Linux [`renameat2(2)`](https://man7.org/linux/man-pages/man2/renameat2.2.html)
- Linux [`fsync(2)`](https://man7.org/linux/man-pages/man2/fsync.2.html)
- Linux [`openat2(2)`](https://man7.org/linux/man-pages/man2/openat2.2.html)
- Linux [`statx(2)`](https://man7.org/linux/man-pages/man2/statx.2.html)
- Linux [`listxattr(2)` / `flistxattr`](https://man7.org/linux/man-pages/man2/listxattr.2.html)
- Linux [`getxattr(2)` / `fgetxattr`](https://man7.org/linux/man-pages/man2/getxattr.2.html)
- Microsoft [`ReplaceFileW`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
- Microsoft [`NtSetInformationFile`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntsetinformationfile)
- Microsoft [`FILE_RENAME_INFORMATION`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_file_rename_information), which requires a retained target-directory `RootDirectory` for a relative simple `FileName`
- Microsoft [`NtCreateFile`](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntcreatefile)
- Microsoft [`OBJECT_ATTRIBUTES`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_object_attributes)
- Microsoft [`InitializeObjectAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/nf-ntdef-initializeobjectattributes)
- Microsoft [`UNICODE_STRING`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_unicode_string)
- Microsoft [`IO_STATUS_BLOCK`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ns-wdm-_io_status_block)
- Microsoft [`128-bit file ID`](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/98860416-1caf-4c80-a9ab-8d61e1ccf5a5)
- Microsoft [`CreateFileW`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew), retained only for initial root/volume acquisition
- Microsoft [`GetFileInformationByHandleEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandleex)
- Microsoft [`FILE_ID_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info)
- Microsoft [`FILE_BASIC_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_basic_info)
- Microsoft [`FILE_STREAM_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_stream_info)
- Microsoft [`GetSecurityInfo`](https://learn.microsoft.com/en-us/windows/win32/api/aclapi/nf-aclapi-getsecurityinfo)
- Microsoft [`FSCTL_READ_FILE_USN_DATA`](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_read_file_usn_data)
- Microsoft [`NtFsControlFile`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntfscontrolfile)
- Microsoft [`FSCTL_GET_OBJECT_ID`](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_get_object_id)
- Microsoft [`FILE_OBJECTID_BUFFER`](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ns-winioctl-file_objectid_buffer)
- Microsoft [`DeviceIoControl`](https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-deviceiocontrol), used only to reject its Boolean/`GetLastError` boundary as raw-NTSTATUS authority
- Microsoft [`NTSTATUS values`](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/596a1078-e883-4972-9bbc-49e60bebca55)
- Microsoft [`FlushFileBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers)

## Executable exact matrix and chain checker

The checker below is part of this proof and is executed from the repository
root. It duplicate-rejects both JSON inputs, meta-validates the Draft 2020-12
schema, admits and fingerprint-replays every positive record, pins the exact
81-row matrix and four-chain aggregates, checks all 162 Unix/Windows cells, the
full 30-row source-race cross-product, crash ambiguity, the no-call empty trace,
mutation-free `D=X`, every Windows retained-handle move binding, all
record/fingerprint edges, every structured native-observation digest and
move-invariant component, and actually applies and rejects every declared
schema or semantic mutation. It pins and validates the exact private
`NtCreateFile` FFI/layout/constant/role/open trace and executes every wrong-ABI,
`CreateFileW` child, RootDirectory, child-name, reparse-defense,
disposition/options/access/share/status/`IO_STATUS_BLOCK`, sentinel-ID, and
continuity negative, including wrong `MaximumLength`, retained-directory
options, post-open type, and post-open reparse state. It also pins the exact
private synchronous `NtFsControlFile` ten-parameter ABI, layouts, null
Event/APC/input fields, initialized I/O status sentinel, control code, exact
64-byte output, and immediate-or-completed status policy, and executes every one
of its 36 field/status/length mutations. The remaining semantic negatives include arbitrary opaque
version tokens, Unix mode/xattr deltas, and Windows attribute/last-write/DACL/
named-stream deltas, numeric-only continuity, Object-ID failures,
ancestor/parent/junction races, and backup mismatch. It also executes
row-removal and disposition-weakening negatives.

Extract and execute the fenced body without maintaining a second checker:

```powershell
$proof = Get-Content -Raw docs/specs/handbook-contract-membrane/slices/HCM-2.3/proof/20260725T032102Z--review-10-contract-correction-successor.md
$null = $proof -match '(?s)<!-- atomic-checker:start -->\r?\n```python\r?\n(.*?)\r?\n```\r?\n<!-- atomic-checker:end -->'
$Matches[1] | uv run --with jsonschema==4.25.1 python -
```

<!-- atomic-checker:start -->
```python
import copy
import hashlib
import json
import re
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path.cwd()
BASE = ROOT / "docs/specs/handbook-contract-membrane/slices/HCM-2.3/contracts"
SCHEMA_PATH = BASE / "generic-artifact-control-records-1.0.0.schema.json"
VECTOR_PATH = BASE / "generic-artifact-control-vectors-v1.0.json"


def no_duplicates(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load(path):
    return json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=no_duplicates)


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def digest(value):
    return hashlib.sha256(canonical(value).encode("utf-8")).hexdigest()


def need(condition, message):
    if not condition:
        raise ValueError(message)


def mutate(record, mutations):
    changed = copy.deepcopy(record)
    for mutation in mutations:
        parts = mutation["json_pointer"].lstrip("/").split("/")
        target = changed
        for raw in parts[:-1]:
            token = raw.replace("~1", "/").replace("~0", "~")
            target = target[int(token)] if isinstance(target, list) else target[token]
        leaf = parts[-1].replace("~1", "/").replace("~0", "~")
        if isinstance(target, list):
            target[int(leaf)] = copy.deepcopy(mutation["replacement"])
        else:
            target[leaf] = copy.deepcopy(mutation["replacement"])
    return changed


schema = load(SCHEMA_PATH)
vectors = load(VECTOR_PATH)
Draft202012Validator.check_schema(schema)
validator = Draft202012Validator(schema)
positives = {item["vector_id"]: item for item in vectors["positive_vectors"]}
need(len(positives) == len(vectors["positive_vectors"]), "duplicate positive vector ID")

for vector_id, item in positives.items():
    record = item["record"]
    validator.validate(record)
    payload = copy.deepcopy(record)
    excluded = item["identity_excludes"]
    need(len(excluded) == 1, f"{vector_id}: exactly one fingerprint exclusion required")
    fingerprint_name = excluded[0]
    expected = payload.pop(fingerprint_name)
    actual = "sha256:" + digest(payload)
    need(actual == expected, f"{vector_id}: fingerprint mismatch")

checker = vectors["atomic_publication_matrix_checker"]
matrix = vectors["atomic_publication_exact_matrix"]
chains = vectors["publication_chain_contracts"]
nt_contract = vectors["windows_ntcreatefile_relative_open_contract"]
nt_rejections = vectors["windows_ntcreatefile_rejection_vectors"]
ntfs_contract = vectors["windows_ntfscontrolfile_object_id_contract"]
ntfs_rejections = vectors["windows_ntfscontrolfile_rejection_vectors"]
set_hash = lambda values: hashlib.sha256("\n".join(sorted(values)).encode("utf-8")).hexdigest()
need(len(checker["required_row_ids"]) == 81, "required row count changed")
need(set_hash(checker["required_row_ids"]) == "e43e35802e5b4179a999e3be3ace5f9b6c3ef637c507c6c55cd6e0113dac5c1a", "required row IDs changed")
need(len(checker["required_source_race_row_ids"]) == 30, "source-race row count changed")
need(set_hash(checker["required_source_race_row_ids"]) == "67f1002d00c8cd5ce9aec853f835d868a53437c107810ec5ab6149af58745271", "source-race row IDs changed")
need(len(checker["required_absent_source_race_row_ids"]) == 15, "absent-source row count changed")
need(set_hash(checker["required_absent_source_race_row_ids"]) == "5f24fa797a46e200796bb01b0fdb41f5954fe195eb76825b5867769579ad7308", "absent-source row IDs changed")
need(len(checker["required_ambiguous_displaced_row_ids"]) == 1, "ambiguous displaced row count changed")
need(set_hash(checker["required_ambiguous_displaced_row_ids"]) == "0834dfa2e530a65498d6e641ae02eab8855a733675f5d155628ba8dfc485f87a", "ambiguous displaced row IDs changed")
need(len(checker["required_chain_ids"]) == 4, "required chain count changed")
need(set_hash(checker["required_chain_ids"]) == "81f8d0542b3aaae9d47d5dabba6c1d0d81aa520b1ab1ab06ad34f3912dc4ac0c", "required chain IDs changed")
need(len(checker["required_semantic_rejection_ids"]) == 20, "required semantic rejection count changed")
need(set_hash(checker["required_semantic_rejection_ids"]) == "e5c03b9da7e987ff037456d669e48db75aca0682a51e592681226661b271e8cf", "required semantic rejection IDs changed")
need(len(checker["required_ntcreatefile_rejection_ids"]) == 39, "required NtCreateFile rejection count changed")
need(set_hash(checker["required_ntcreatefile_rejection_ids"]) == "a5c8461b1a3dd507acc9ee719d03b0bec8278cd2461ef63ab52aa2a1839d7f67", "required NtCreateFile rejection IDs changed")
need(len(checker["required_ntfscontrolfile_rejection_ids"]) == 36, "required NtFsControlFile rejection count changed")
need(set_hash(checker["required_ntfscontrolfile_rejection_ids"]) == "ef21e7829ad187f925f6e26f7654013f067fcea2f06845bad72ceb123acc1512", "required NtFsControlFile rejection IDs changed")
need(digest(matrix) == "382b71d415c39773f3389c2cf1700d89ea46608d6e0fec3f02c96871788f3e84", "exact matrix content changed")
need(digest(chains) == "eab765573370ebc0513e46ff23b18df44c46a984489e7b0e59a7bbe0971fd5f1", "exact chain content changed")
need(digest(nt_contract) == "a71ae9eb79d0c7c1ca78fa98b3890089d6471d2ff2c20de2590a5b4f9650865d", "exact NtCreateFile contract changed")
need(digest(ntfs_contract) == "c15f5c098cea470a4e637d2bc1ecb335c6641f9b052e8c37a87792b7a7403d0a", "exact NtFsControlFile contract changed")

EXPECTED_NTCREATE_FFI = {
    "dll": "ntdll.dll",
    "link_name": "ntdll",
    "symbol": "NtCreateFile",
    "calling_convention": "system",
    "return_abi": "NTSTATUS_i32",
    "parameters": [
        "FileHandle:*mut HANDLE",
        "DesiredAccess:ACCESS_MASK_u32",
        "ObjectAttributes:*mut OBJECT_ATTRIBUTES",
        "IoStatusBlock:*mut IO_STATUS_BLOCK",
        "AllocationSize:*mut LARGE_INTEGER_or_null",
        "FileAttributes:ULONG_u32",
        "ShareAccess:ULONG_u32",
        "CreateDisposition:ULONG_u32",
        "CreateOptions:ULONG_u32",
        "EaBuffer:PVOID_or_null",
        "EaLength:ULONG_u32",
    ],
}
EXPECTED_NTCREATE_LAYOUTS = {
    "UNICODE_STRING": {
        "repr": "C",
        "fields": ["Length:USHORT_u16", "MaximumLength:USHORT_u16", "Buffer:*mut_u16"],
        "size_x86": 8,
        "size_x64": 16,
    },
    "OBJECT_ATTRIBUTES": {
        "repr": "C",
        "fields": [
            "Length:ULONG_u32",
            "RootDirectory:HANDLE",
            "ObjectName:*mut_UNICODE_STRING",
            "Attributes:ULONG_u32",
            "SecurityDescriptor:PVOID",
            "SecurityQualityOfService:PVOID",
        ],
        "size_x86": 24,
        "size_x64": 48,
    },
    "IO_STATUS_BLOCK": {
        "repr": "C",
        "fields": [
            "Status:NTSTATUS_i32_or_Pointer:PVOID_union",
            "Information:ULONG_PTR_usize",
        ],
        "size_x86": 8,
        "size_x64": 16,
    },
}
EXPECTED_NTCREATE_CONSTANTS = {
    "STATUS_SUCCESS": "0x00000000",
    "FILE_OPENED": "0x00000001",
    "OBJ_CASE_INSENSITIVE": "0x00000040",
    "OBJ_DONT_REPARSE": "0x00001000",
    "OBJECT_ATTRIBUTES_REQUIRED": "0x00001040",
    "FILE_OPEN": "0x00000001",
    "FILE_SYNCHRONOUS_IO_NONALERT": "0x00000020",
    "FILE_NON_DIRECTORY_FILE": "0x00000040",
    "FILE_OPEN_REPARSE_POINT": "0x00200000",
    "DIRECTORY_CREATE_OPTIONS": "0x00200020",
    "FILE_CREATE_OPTIONS": "0x00200060",
    "FILE_SHARE_READ": "0x00000001",
    "FILE_SHARE_WRITE": "0x00000002",
    "FILE_SHARE_DELETE": "0x00000004",
    "RETAINED_SHARE_ACCESS": "0x00000003",
    "FILE_GENERIC_READ": "0x00120089",
    "DELETE": "0x00010000",
    "SOURCE_DESIRED_ACCESS": "0x00130089",
}
EXPECTED_NTCREATE_INITIALIZATION = {
    "UNICODE_STRING": {
        "Length": "checked_even_nonzero_utf16_code_unit_count_times_2_at_most_65534_bytes",
        "MaximumLength": "equal_to_Length",
        "Buffer": "non_null_unterminated_validated_child_buffer_exactly_Length_bytes_with_no_terminator",
    },
    "OBJECT_ATTRIBUTES": {
        "Length": "size_of_OBJECT_ATTRIBUTES_as_u32",
        "RootDirectory": "non_null_role_specific_retained_parent",
        "ObjectName": "non_null_pointer_to_initialized_UNICODE_STRING",
        "Attributes": "0x00001040",
        "SecurityDescriptor": None,
        "SecurityQualityOfService": None,
    },
    "IO_STATUS_BLOCK": {
        "storage": "zero_initialized_mutable",
        "accepted_Status": "0x00000000",
        "accepted_Information": "0x00000001",
    },
}
EXPECTED_NTCREATE_PROFILES = {
    "retained_directory_component": {
        "desired_access": "0x00120089",
        "share_access": "0x00000003",
        "create_disposition": "0x00000001",
        "create_options": "0x00200020",
    },
    "retained_source_file": {
        "desired_access": "0x00130089",
        "share_access": "0x00000003",
        "create_disposition": "0x00000001",
        "create_options": "0x00200060",
    },
    "enumerated_child_file": {
        "desired_access": "0x00120089",
        "share_access": "0x00000003",
        "create_disposition": "0x00000001",
        "create_options": "0x00200060",
    },
    "destination_rebound_file": {
        "desired_access": "0x00120089",
        "share_access": "0x00000003",
        "create_disposition": "0x00000001",
        "create_options": "0x00200060",
    },
}
EXPECTED_NTCREATE_ROOTS = {
    "retained_directory_component": "retained_repository_root_handle",
    "retained_source_file": "retained_source_parent_handle",
    "enumerated_child_file": "retained_source_parent_handle",
    "destination_rebound_file": "retained_destination_parent_handle",
}
EXPECTED_NTCREATE_REFUSALS = {
    "missing_ntdll_symbol_or_abi_layout": "unsupported_platform_no_CreateFileW_child_fallback",
    "invalid_pre_move_child_or_source_open": "unsupported_native_observation_no_move_no_result_no_marker",
    "post_move_destination_rebound_failure": "refused_ambiguous_no_further_move_no_result_no_marker",
    "non_success_or_unexpected_io_status_block": "reject_handle_before_authority",
}
EXPECTED_NTFSCONTROL_FFI = {
    "dll": "ntdll.dll",
    "link_name": "ntdll",
    "symbol": "NtFsControlFile",
    "calling_convention": "system",
    "return_abi": "NTSTATUS_i32",
    "parameters": [
        "FileHandle:HANDLE",
        "Event:HANDLE_or_null",
        "ApcRoutine:PIO_APC_ROUTINE_or_null",
        "ApcContext:PVOID_or_null",
        "IoStatusBlock:*mut IO_STATUS_BLOCK",
        "FsControlCode:ULONG_u32",
        "InputBuffer:PVOID_or_null",
        "InputBufferLength:ULONG_u32",
        "OutputBuffer:PVOID_non_null",
        "OutputBufferLength:ULONG_u32",
    ],
}
EXPECTED_NTFSCONTROL_LAYOUTS = {
    "IO_STATUS_BLOCK": {
        "repr": "C",
        "fields": [
            "Status:NTSTATUS_i32_or_Pointer:PVOID_union",
            "Information:ULONG_PTR_usize",
        ],
        "size_x86": 8,
        "size_x64": 16,
    },
    "FILE_OBJECTID_BUFFER": {
        "repr": "C",
        "fields": ["ObjectId:[u8;16]", "ExtendedInfo:[u8;48]"],
        "size": 64,
    },
}
EXPECTED_NTFSCONTROL_CALL = {
    "file_handle": "retained_regular_file_handle_opened_with_FILE_SYNCHRONOUS_IO_NONALERT_and_SYNCHRONIZE",
    "event": None,
    "apc_routine": None,
    "apc_context": None,
    "io_status_block_initial": {
        "Status": "0xC0000001",
        "Information": "ULONG_PTR_MAX",
    },
    "fs_control_code": "FSCTL_GET_OBJECT_ID_0x0009009C",
    "input_buffer": None,
    "input_buffer_length": 0,
    "output_buffer": "non_null_mutable_FILE_OBJECTID_BUFFER",
    "output_buffer_length": 64,
}
EXPECTED_NTFSCONTROL_STATUS = {
    "pending_incomplete": {
        "returned_ntstatus": "0x00000103",
        "io_status_block_status": "initial_0xC0000001_unchanged",
        "io_status_block_information": "initial_ULONG_PTR_MAX_unchanged",
        "disposition": "fail_closed_incomplete_io_status_block",
    },
    "object_id_absent": {
        "immediate_raw_with_unchanged_sentinel": {
            "returned_ntstatus": "0xC00002F0",
            "io_status_block_status": "initial_0xC0000001_unchanged",
            "io_status_block_information": "initial_ULONG_PTR_MAX_unchanged",
            "completion_authority": "immediate_raw_ntstatus",
        },
        "completed_through_io_status_block": {
            "accepted_returned_ntstatus": ["0xC00002F0", "0x00000103"],
            "io_status_block_status": "0xC00002F0",
            "io_status_block_information": 0,
            "completion_authority": "io_status_block",
        },
        "disposition": "admit_absent_object_id",
    },
    "object_id_present": {
        "returned_ntstatus": "0x00000000",
        "io_status_block_status": "0x00000000",
        "io_status_block_information": 64,
        "disposition": "refuse_present_object_id",
    },
    "all_other_or_inconsistent": "fail_closed_unsupported_native_observation_before_move_or_refused_ambiguous_after_move",
}
FILE_ID_SENTINELS = {
    "0x00000000000000000000000000000000",
    "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
}


def validate_ntcreatefile_contract(contract):
    need(contract["contract_id"] == "ntdll-ntcreatefile-retained-parent-relative-v1",
         "NtCreateFile contract identity changed")
    need(contract["future_helper"] == "open_retained_parent_relative_nt",
         "future helper identity changed")
    need(contract["helper_scope"] == "private_cfg_windows_artifact_lineage_store_no_dependency_no_public_api_no_global_or_thread_local_state",
         "future helper widened scope")
    need(contract["critical_scope"] == "unauthorized_child_of_existing_retained_handle_publication_boundary_not_a_seventeenth_surface",
         "future helper changed CRITICAL accounting")
    need(contract["api_policy"] == {
        "child_and_rebound_api": "NtCreateFile",
        "createfilew_allowed_only_for": "initial_already_authorized_root_or_volume_without_retained_parent",
        "createfilew_child_authority": False,
        "child_fallback": "forbidden",
    }, "NtCreateFile/CreateFileW API policy changed")
    need(contract["ffi"] == EXPECTED_NTCREATE_FFI, "NtCreateFile FFI ABI changed")
    need(contract["layouts"] == EXPECTED_NTCREATE_LAYOUTS, "NtCreateFile ABI layout changed")
    need(contract["structure_initialization"] == EXPECTED_NTCREATE_INITIALIZATION,
         "NtCreateFile ABI structure initialization changed")
    need(contract["constants"] == EXPECTED_NTCREATE_CONSTANTS, "NtCreateFile constants changed")
    need(contract["role_profiles"] == EXPECTED_NTCREATE_PROFILES, "NtCreateFile role profile changed")
    need(contract["refusal_classes"] == EXPECTED_NTCREATE_REFUSALS,
         "NtCreateFile refusal classes changed")
    trace = contract["authority_trace"]
    opens = trace["opens"]
    need([item["role"] for item in opens] == list(EXPECTED_NTCREATE_PROFILES),
         "NtCreateFile open role order changed")
    for item in opens:
        role = item["role"]
        need(item["api"] == "NtCreateFile", f"{role}: CreateFileW/path child open admitted")
        need(item["root_directory"] == EXPECTED_NTCREATE_ROOTS[role] and item["root_directory"],
             f"{role}: null or wrong retained RootDirectory")
        name = item["object_name"]
        need(item["object_name_component_count"] == 1 and name not in {".", ".."} and
             not any(token in name for token in ("/", "\\", ":", "\x00")),
             f"{role}: ObjectName is not one validated child")
        need(item["object_attributes"] == "0x00001040",
             f"{role}: missing OBJ_CASE_INSENSITIVE/OBJ_DONT_REPARSE")
        for key, value in EXPECTED_NTCREATE_PROFILES[role].items():
            need(item[key] == value, f"{role}: wrong {key}")
        need(item["file_attributes"] == "0x00000000" and
             item["allocation_size"] is None and item["ea_buffer"] is None and
             item["ea_length"] == 0, f"{role}: allocation/EA contract changed")
        need(item["returned_ntstatus"] == "0x00000000" and
             item["io_status_block_status"] == "0x00000000" and
             item["io_status_block_information"] == "0x00000001",
              f"{role}: unsuccessful or unexpected NtCreateFile completion")
    directory_open = opens[0]
    need(directory_open["post_open_file_type"] == "directory" and
         directory_open["post_open_directory"] is True and
         directory_open["post_open_reparse_point"] is False and
         directory_open["post_open_reparse_tag"] is None,
         "retained directory post-open type/reparse observation changed")
    for key in (
        "retained_source_file_id_hex",
        "enumerated_child_file_id_hex",
        "opened_child_file_id_hex",
        "destination_observation_file_id_hex",
        "same_live_source_file_id_hex",
        "same_live_destination_file_id_hex",
    ):
        value = trace[key]
        need(re.fullmatch(r"0x[0-9A-F]{32}", value) is not None and
             value not in FILE_ID_SENTINELS, f"{key}: sentinel/invalid FILE_ID_128")
    need(trace["enumerated_child_file_id_hex"] == trace["opened_child_file_id_hex"],
         "enumerated/opened child identity mismatch")
    need(trace["retained_source_file_id_hex"] ==
         trace["destination_observation_file_id_hex"] ==
         trace["same_live_source_file_id_hex"] ==
         trace["same_live_destination_file_id_hex"],
         "retained-source/destination same-live identity mismatch")
    need(trace["same_live_object_verified_while_source_handle_live"] is True and
         trace["numeric_identity_only"] is False,
         "numeric-only or post-handle continuity admitted")


validate_ntcreatefile_contract(nt_contract)
nt_executed = set()
for item in nt_rejections:
    vector_id = item["vector_id"]
    need(vector_id in checker["required_ntcreatefile_rejection_ids"],
         f"{vector_id}: undeclared NtCreateFile rejection")
    changed = mutate(nt_contract, [item])
    try:
        validate_ntcreatefile_contract(changed)
    except (KeyError, TypeError, ValueError):
        pass
    else:
        raise ValueError(f"{vector_id}: NtCreateFile mutation was not rejected")
    nt_executed.add(vector_id)
need(nt_executed == set(checker["required_ntcreatefile_rejection_ids"]),
     "declared NtCreateFile rejection was not executed")


def validate_ntfscontrolfile_contract(contract):
    need(contract["contract_id"] == "ntdll-ntfscontrolfile-object-id-absence-v1",
         "NtFsControlFile contract identity changed")
    need(contract["future_helper"] == "query_object_id_absence_nt",
         "future NtFsControlFile helper identity changed")
    need(contract["helper_scope"] == "private_cfg_windows_artifact_lineage_store_no_dependency_no_public_api_no_global_or_thread_local_state",
         "future NtFsControlFile helper widened scope")
    need(contract["critical_scope"] == "unauthorized_child_of_existing_structured_observation_boundary_not_a_seventeenth_surface",
         "future NtFsControlFile helper changed CRITICAL accounting")
    need(contract["api_policy"] == {
        "object_id_query_api": "NtFsControlFile",
        "deviceiocontrol_object_id_authority": False,
        "fallback": "forbidden",
    }, "NtFsControlFile/DeviceIoControl API policy changed")
    need(contract["ffi"] == EXPECTED_NTFSCONTROL_FFI,
         "NtFsControlFile FFI ABI changed")
    need(contract["layouts"] == EXPECTED_NTFSCONTROL_LAYOUTS,
         "NtFsControlFile ABI layout changed")
    need(contract["call"] == EXPECTED_NTFSCONTROL_CALL,
         "NtFsControlFile synchronous call contract changed")
    need(contract["status_policy"] == EXPECTED_NTFSCONTROL_STATUS,
         "NtFsControlFile raw/final status policy changed")


validate_ntfscontrolfile_contract(ntfs_contract)
ntfs_executed = set()
for item in ntfs_rejections:
    vector_id = item["vector_id"]
    need(vector_id in checker["required_ntfscontrolfile_rejection_ids"],
         f"{vector_id}: undeclared NtFsControlFile rejection")
    changed = mutate(ntfs_contract, [item])
    try:
        validate_ntfscontrolfile_contract(changed)
    except (KeyError, TypeError, ValueError):
        pass
    else:
        raise ValueError(f"{vector_id}: NtFsControlFile mutation was not rejected")
    ntfs_executed.add(vector_id)
need(ntfs_executed == set(checker["required_ntfscontrolfile_rejection_ids"]),
     "declared NtFsControlFile rejection was not executed")

rows = {row["row_id"]: row for row in matrix}
need(len(rows) == len(matrix), "duplicate matrix row ID")
need(set(rows) == set(checker["required_row_ids"]), "missing or extra matrix row ID")
row_fields = {"expected_basis", "phase", "before_tuple", "action", "after_tuple",
              "classifier", "result_record", "retry", "reader_result_before_marker",
              "cleanup_authority"}
platform_fields = {"primitive", "diagnostic", "durability"}
for row_id, row in rows.items():
    need(row_fields <= row.keys(), f"{row_id}: incomplete row")
    need(all(row[field] != "" for field in row_fields), f"{row_id}: empty row field")
    need(set(checker["required_platform_keys"]) == {"unix", "windows"}, "platform authority changed")
    for platform in ("unix", "windows"):
        need(platform_fields == set(row[platform]), f"{row_id}/{platform}: incomplete platform cell")
        need(all(row[platform][field] != "" for field in platform_fields), f"{row_id}/{platform}: empty platform field")
    need(not (set(row) & set(checker["forbidden_platform_keys"])), f"{row_id}: wildcard platform")
    windows_primitive = row["windows"]["primitive"]
    need("CreateFileW" not in windows_primitive,
         f"{row_id}/windows: CreateFileW child/rebound primitive")
    if any(token in row["action"] for token in ("publish_", "claim_", "restore_")):
        need("NtSetInformationFile(FileRenameInformation)" in windows_primitive,
             f"{row_id}/windows: pathname or non-handle primitive")
        need("RootDirectory_retained_parent" in windows_primitive and
             "ReplaceIfExists_FALSE" in windows_primitive,
             f"{row_id}/windows: incomplete FileRenameInformation binding")
    if "restore_D_to_C" in row["action"]:
        need("D=expected_post_move" in row["before_tuple"], f"{row_id}: restoration source is not authenticated expected basis")
need(all(item.get("platform") in {"unix", "windows"} for item in vectors["atomic_publication_vectors"]), "wildcard atomic vector")
for item in vectors["atomic_publication_vectors"]:
    primitive = item.get("primitive", "")
    need("MoveFileExW" not in primitive, f"{item['vector_id']}: forbidden pathname primitive")
    need("SetFileInformationByHandle" not in primitive,
         f"{item['vector_id']}: forbidden Win32 rename primitive")
need({
    "windows-ancestor-rename-race",
    "windows-destination-parent-rename-race",
    "windows-junction-substitution-race",
    "unix-same-id-aba",
    "windows-same-id-aba",
    "windows-zero-file-id",
    "windows-object-id-present",
    "windows-object-id-unqueryable",
    "windows-object-id-between-pass-mutation",
    "expected-basis-backup-preserved",
} <= {item["vector_id"] for item in vectors["atomic_publication_vectors"]},
     "Review 6 atomic vectors missing")
need({
    "windows-pathname-publication-primitive",
    "windows-win32-rename-publication-primitive",
} <= {item["vector_id"] for item in vectors["negative_vectors"]},
     "Windows forbidden rename-primitive negatives missing")

need(set(checker["required_source_race_row_ids"]) <= set(rows), "missing source-race row")
source_outcomes = {"no-call", "publish-failed", "publish-success", "publish-crash-unmoved", "publish-crash-moved"}
for expected_basis in ("absent", "present"):
    for source_kind in ("deleted", "same-byte-substituted", "different-byte-substituted"):
        expected_ids = {f"{expected_basis}-source-{source_kind}-{outcome}" for outcome in source_outcomes}
        need(expected_ids <= set(checker["required_source_race_row_ids"]),
             f"{expected_basis}/{source_kind}: incomplete exact source-race outcomes")


def validate_absent_source_contract(candidate_rows):
    required = set(checker["required_absent_source_race_row_ids"])
    need(required <= set(candidate_rows), "missing expected-absent source-race row")
    for row_id in required:
        row = candidate_rows[row_id]
        need(row["expected_basis"] == "absent", f"{row_id}: wrong expected basis")
        need(row["reader_result_before_marker"] == "refuse", f"{row_id}: unauthenticated object became readable")
        need(row["cleanup_authority"] == "forbidden", f"{row_id}: cleanup widened")
        if "crash" in row_id:
            need(row["execution_modes"] == ["recovery"], f"{row_id}: crash was treated as live execution")
            need(row["classifier"] == "refused_ambiguous_missing_completed_live_continuity",
                 f"{row_id}: crash inferred continuity")
            need(row["result_record"] == "absent" and not row["retry"] and row["action"] == "none",
                 f"{row_id}: crash authorized result, retry, or mutation")
        else:
            need(row["execution_modes"] == ["fresh", "recovery"], f"{row_id}: fresh/recovery divergence")
            need(row["classifier"] == "stable_source_conflict_outputless", f"{row_id}: source conflict weakened")
            need(row["result_record"] == "refused_basis_conflict" and not row["retry"],
                 f"{row_id}: source conflict is not terminal outputless refusal")
            need("D=absent" in row["before_tuple"] and "D=absent" in row["after_tuple"],
                 f"{row_id}: unexpected displaced authority")
            need(row["action"] in {"none", "publish_R_to_C_no_replace"}, f"{row_id}: unauthorized action")
            need(not any(token in row["action"] for token in ("adopt", "return", "delete", "restore", "cleanup")),
                 f"{row_id}: unauthenticated object gained authority")


validate_absent_source_contract(rows)
removed_rows = dict(rows)
removed_rows.pop("absent-source-deleted-no-call")
try:
    validate_absent_source_contract(removed_rows)
except ValueError:
    pass
else:
    raise ValueError("expected-absent source row removal was not rejected")
weakened_rows = copy.deepcopy(rows)
weakened_rows["absent-source-same-byte-substituted-publish-success"]["classifier"] = "authorized"
try:
    validate_absent_source_contract(weakened_rows)
except ValueError:
    pass
else:
    raise ValueError("expected-absent source disposition weakening was not rejected")

crash_rows = {row_id: row for row_id, row in rows.items() if "crash" in row_id}
need(crash_rows, "crash rows missing")
for row_id, row in crash_rows.items():
    need(row["execution_modes"] == ["recovery"], f"{row_id}: crash execution mode")
    need(row["action"] == "none" and row["after_tuple"] == "unchanged_evidence_retained",
         f"{row_id}: recovery mutated crash evidence")
    need(row["classifier"] == "refused_ambiguous_missing_completed_live_continuity",
         f"{row_id}: recovery inferred live continuity")
    need(row["result_record"] == "absent" and not row["retry"],
         f"{row_id}: recovery wrote/reconstructed a result or retried")
    need(row["reader_result_before_marker"] == "refuse" and
         row["cleanup_authority"] == "forbidden", f"{row_id}: crash widened authority")

for row_id in checker["required_ambiguous_displaced_row_ids"]:
    row = rows[row_id]
    need("D=unexpected" in row["before_tuple"], f"{row_id}: missing unauthenticated D=X tuple")
    need(row["action"] == "none" and row["after_tuple"] == "unchanged", f"{row_id}: D=X mutation attempted")
    need(row["classifier"] == "refused_ambiguous" and row["result_record"] == "absent", f"{row_id}: D=X did not refuse without result")
    need(row["cleanup_authority"] == "forbidden", f"{row_id}: D=X cleanup widened")

chain_map = {chain["chain_id"]: chain for chain in chains}
need(len(chain_map) == len(chains), "duplicate chain ID")
need(set(chain_map) == set(checker["required_chain_ids"]), "missing or extra chain ID")
for chain_id, chain in chain_map.items():
    ids = chain["record_vector_ids"]
    need(len(ids) == 8 and len(set(ids)) == 8, f"{chain_id}: chain must contain eight unique records")
    need(all(vector_id in positives for vector_id in ids), f"{chain_id}: missing record reference")
    intent, observation, verified, native, marker, evidence, result, ledger = [positives[vector_id]["record"] for vector_id in ids]
    need([record["schema_id"] for record in (intent, observation, verified, native, marker, evidence, result, ledger)] == [
        "handbook.generic-transaction-intent",
        "handbook.generic-native-publication-observation",
        "handbook.generic-verified-stage",
        "handbook.generic-native-publication-result",
        "handbook.generic-commit-marker",
        "handbook.generic-internal-transaction-evidence",
        "handbook.generic-domain-mutation-result",
        "handbook.generic-domain-ledger-entry",
    ], f"{chain_id}: record order/type mismatch")
    need(len({record["transaction_id"] for record in (intent, observation, verified, native, marker, evidence, result, ledger)}) == 1, f"{chain_id}: transaction mismatch")
    need(observation["intent_fingerprint"] == intent["intent_fingerprint"] == verified["intent_fingerprint"] == native["intent_fingerprint"] == marker["intent_fingerprint"] == evidence["intent_fingerprint"], f"{chain_id}: intent edge")
    need(verified["publication_observation_fingerprint"] == observation["observation_fingerprint"] == native["observation_fingerprint"], f"{chain_id}: observation edge")
    need(native["verified_fingerprint"] == verified["verified_fingerprint"] == marker["verified_fingerprint"] == evidence["verified_fingerprint"], f"{chain_id}: verified edge")
    need(marker["publication_result_fingerprint"] == native["result_fingerprint"] == evidence["publication_result_fingerprint"], f"{chain_id}: native-result edge")
    need(evidence["marker_fingerprint"] == marker["marker_fingerprint"], f"{chain_id}: marker edge")
    need(result["internal_transaction_evidence_fingerprint"] == evidence["evidence_fingerprint"], f"{chain_id}: evidence edge")
    need(ledger["result_fingerprint"] == result["result_fingerprint"], f"{chain_id}: domain-result edge")
    need(intent["request_fingerprint"] == evidence["request_fingerprint"] == result["request_fingerprint"] == ledger["request_fingerprint"], f"{chain_id}: request edge")
    need(observation["platform_family"] == native["platform_family"] == chain["platform"], f"{chain_id}: platform edge")
    need(marker["outcome"] == evidence["outcome"] == result["outcome"] == chain["terminal_outcome"], f"{chain_id}: outcome edge")
    need(marker["authoritative_outputs"] == evidence["authoritative_outputs"] == result["authoritative_outputs"], f"{chain_id}: output edge")
    if chain["terminal_outcome"] == "refused":
        need(marker["authoritative_outputs"] == [] and marker["subordinate_outputs"] == [] and evidence["authoritative_outputs"] == [] and evidence["subordinate_outputs"] == [] and result["authoritative_outputs"] == [], f"{chain_id}: conflict must be outputless")
        need(marker["refusal"] == evidence["refusal"] == result["refusal"], f"{chain_id}: refusal edge")
    if chain_id == "expected-absent-unix-source-deleted-conflict":
        need(native["native_call_trace"] == [], f"{chain_id}: no-call trace must be exact empty list")
        need(native["basis_disposition"] == "replacement_source_deleted", f"{chain_id}: wrong source disposition")
        need(native["publication_disposition"] == "refused_basis_conflict", f"{chain_id}: wrong publication disposition")
        need(marker["refusal"]["expected_fingerprint"] == native["replacement_bytes_sha256"], f"{chain_id}: replacement fingerprint not bound")
        need(marker["refusal"]["observed_fingerprint"] is None, f"{chain_id}: deleted source must bind null observation")


def native_identity_token(observation):
    if observation["platform_family"] == "unix":
        identity = {
            "platform_family": "unix",
            "file_type": observation["file_type"],
            "device_id": observation["device_id"],
            "inode": observation["inode"],
        }
    else:
        identity = {
            "platform_family": "windows",
            "file_type": observation["file_type"],
            "volume_serial_number_hex": observation["volume_serial_number_hex"],
            "file_id_hex": observation["file_id_hex"],
        }
    return "native-id-v1:" + digest(identity)


def native_version_token(observation):
    return "native-version-v1:" + digest(observation)


def windows_utf16_units(value):
    encoded = value.encode("utf-16-le")
    return tuple(encoded[index] | (encoded[index + 1] << 8) for index in range(0, len(encoded), 2))


def validate_platform_observation(observation, platform):
    need(observation["platform_family"] == platform, "native observation platform mismatch")
    need(observation["file_type"] == "regular" and observation["link_count"] == 1,
         "native observation regular/single-link invariant")
    if platform == "unix":
        entries = observation["xattrs"]["entries"]
        raw_names = [__import__("base64").b64decode(entry["name_base64"], validate=True) for entry in entries]
        need(raw_names == sorted(raw_names) and len(raw_names) == len(set(raw_names)),
             "Unix xattr names are not complete unique raw-byte order")
        need(observation["xattrs"]["entries_digest"] == "sha256:" + digest(entries),
             "Unix xattr inventory digest mismatch")
    else:
        need(observation["file_id_hex"] not in FILE_ID_SENTINELS,
             "Windows sentinel FILE_ID_INFORMATION admitted")
        need(observation["object_id_query"] == "absent_status_objectid_not_found",
             "Windows Object ID did not fail closed")
        security = copy.deepcopy(observation["security"])
        security_digest = security.pop("descriptor_digest")
        need(security_digest == "sha256:" + digest(security), "Windows owner/group/DACL digest mismatch")
        entries = observation["streams"]["entries"]
        names = [entry["name"] for entry in entries]
        need(names == sorted(names, key=windows_utf16_units) and len(names) == len(set(names)),
             "Windows stream names are not exact unique UTF-16 order")
        need(observation["streams"]["entries_digest"] == "sha256:" + digest(entries),
             "Windows stream inventory digest mismatch")
        need(observation["streams"]["non_default_stream_count"] == 0 and names == ["::$DATA"],
             "Windows non-default named stream admitted")
        need(observation["sacl_policy"] == "excluded_not_observed_not_trusted",
             "Windows SACL gained byte-authority status")


def move_invariant_projection(observation):
    projected = copy.deepcopy(observation)
    if projected["platform_family"] == "unix":
        projected.pop("ctime")
    else:
        projected.pop("change_time_100ns")
        projected.pop("usn")
    return projected


def validate_bound_observation(platform, observation, identity_token, version_token, byte_length):
    validate_platform_observation(observation, platform)
    need(observation["byte_length"] == byte_length, "native observation byte length mismatch")
    need(identity_token == native_identity_token(observation), "native identity token is not derived")
    need(version_token == native_version_token(observation), "native version token is not derived")


def validate_native_record_observations(record):
    platform = record["platform_family"]
    expected = record["expected_basis_platform_observation"]
    if record["expected_basis_presence"] == "absent":
        need(expected is None, "expected-absent record retained a native observation")
    else:
        validate_bound_observation(
            platform, expected, record["expected_basis_identity_token"],
            record["expected_basis_version_token"], record["expected_basis_byte_length"])
    replacement = record["replacement_platform_observation"]
    validate_bound_observation(
        platform, replacement, record["replacement_identity_token"],
        record["replacement_version_token"], record["replacement_byte_length"])
    need(record["continuity_protocol"] == "retained-live-handle-v1",
         "native record continuity protocol weakened")
    if record["expected_basis_presence"] == "absent":
        need(record["expected_basis_backup_ref"] is None and
             record["expected_basis_backup_bytes_sha256"] is None and
             record["expected_basis_backup_byte_length"] is None and
             record["expected_basis_backup_metadata_fingerprint"] is None,
             "expected-absent record gained backup authority")
    else:
        need(record["expected_basis_backup_ref"] ==
             f".handbook/state/transactions/artifact-promotions/{record['transaction_id']}.pending/expected-basis.backup",
             "expected-basis backup ref mismatch")
        need(record["expected_basis_backup_bytes_sha256"] == record["expected_basis_fingerprint"],
             "expected-basis backup bytes mismatch")
        need(record["expected_basis_backup_byte_length"] == record["expected_basis_byte_length"],
             "expected-basis backup length mismatch")
        need(record["expected_basis_backup_metadata_fingerprint"] ==
             "sha256:" + digest(expected), "expected-basis backup metadata mismatch")
    return expected, replacement


def semantic_validate(record):
    schema_id = record["schema_id"]
    if schema_id == "handbook.generic-transaction-intent" and record.get("expected_basis_presence") == "present":
        need(record["request_subject"]["expected_current_artifact_fingerprint"] == record["expected_basis_fingerprint"], "intent/request basis mismatch")
    if schema_id in {
        "handbook.generic-native-publication-observation",
        "handbook.generic-native-publication-result",
    }:
        expected_platform_observation, replacement_platform_observation = validate_native_record_observations(record)
    if schema_id == "handbook.generic-native-publication-result":
        trace = record["native_call_trace"]
        if trace:
            need(record["continuity_completion"] == "all_native_moves_proved_while_source_handles_live",
                 "native result lacks completed live continuity")
            need([step["ordinal"] for step in trace] == list(range(len(trace))), "trace ordinal discontinuity")
            for previous, current in zip(trace, trace[1:]):
                need(previous["after"] == current["before"], "trace tuple discontinuity")
            final = trace[-1]["after"]
            need(final["canonical"] == record["canonical_observation"] and final["candidate"] == record["candidate_observation"] and final["displaced"] == record["displaced_observation"], "final observation discontinuity")
        else:
            need(record["continuity_completion"] == "no_native_move_invoked",
                 "empty trace claimed a completed move")
            need(record["expected_basis_presence"] == "absent", "empty trace outside expected-absent")
            need(record["basis_disposition"] in {"replacement_source_deleted", "replacement_source_substitution_preserved"}, "empty trace outside source conflict")
        for step in trace:
            proof = step["continuity_proof"]
            need(proof["protocol"] == "retained-live-handle-v1" and
                 proof["platform_family"] == record["platform_family"],
                 "step continuity protocol/platform mismatch")
            need(proof["source_handle_retained_through_destination_observation"] and
                 proof["source_identity_reverified_from_retained_handle"] and
                 proof["source_parent_handle_retained"] and
                 proof["destination_parent_handle_retained"] and
                 proof["destination_opened_from_retained_parent"] and
                 not proof["numeric_identity_only"],
                 "numeric-only or incomplete live continuity proof")
            if step["outcome"] == "success":
                need(proof["continuity_outcome"] ==
                     "destination_is_same_live_source_object" and
                     proof["same_live_object_verified"],
                     "successful move lacks same-live-object proof")
            else:
                need(proof["continuity_outcome"] ==
                     "move_not_observed_source_remained_live" and
                     not proof["same_live_object_verified"],
                     "failed move did not preserve the live source binding")
            need("/" not in proof["destination_child_name"] and
                 "\\" not in proof["destination_child_name"] and
                 proof["destination_child_name"] not in {".", ".."},
                 "destination was not a simple retained-parent child")
            if record["platform_family"] == "windows":
                need(proof["ancestor_chain_binding"] ==
                     "windows_retained_ancestors_no_share_delete_enumerated_child_ids_no_reparse",
                     "Windows ancestor/parent/junction binding weakened")
                need(proof["platform_binding"] ==
                     "setfileinformationbyhandle_filerenameinfo_rootdirectory_simple_child_replace_false",
                     "Windows pathname primitive admitted")
            else:
                need(proof["platform_binding"] ==
                     "renameat2_retained_source_and_parent_dirfds",
                     "Unix retained-dirfd binding weakened")
            if step["action"] == "restore_displaced_to_canonical":
                restored = step["before"]["displaced"]
                need(restored["semantic_binding"] == "expected_basis", "restore source is not authenticated expected basis")
                need(restored["identity_token"] == record["expected_basis_identity_token"], "restore expected identity mismatch")
                need(restored["bytes_sha256"] == record["expected_basis_fingerprint"], "restore expected bytes mismatch")
        for path_observation in [
            item
            for step in trace
            for item in (*step["before"].values(), *step["after"].values())
        ] + [
            record["canonical_observation"],
            record["candidate_observation"],
            record["displaced_observation"],
        ]:
            if path_observation["presence"] == "absent":
                need(path_observation["platform_observation"] is None, "absent path retained native components")
                continue
            validate_bound_observation(
                record["platform_family"], path_observation["platform_observation"],
                path_observation["identity_token"], path_observation["version_token"],
                path_observation["byte_length"])
            if path_observation["semantic_binding"] == "expected_basis":
                baseline = expected_platform_observation
            elif path_observation["semantic_binding"] == "replacement":
                baseline = replacement_platform_observation
            else:
                baseline = None
            if baseline is not None:
                need(move_invariant_projection(path_observation["platform_observation"]) ==
                     move_invariant_projection(baseline),
                     "native move-invariant metadata/side-stream delta")
                if path_observation["version_transition"] == "unchanged_before_first_move":
                    need(path_observation["platform_observation"] == baseline,
                         "pre-move native observation changed")
        canonical_record = record["canonical_observation"]
        if canonical_record["semantic_binding"] == "replacement":
            need(canonical_record["identity_token"] == record["replacement_identity_token"], "canonical replacement identity mismatch")
            need(canonical_record["bytes_sha256"] == record["replacement_bytes_sha256"], "canonical replacement bytes mismatch")
            need(canonical_record["version_token"] != record["replacement_version_token"] and canonical_record["version_transition"] == "primitive_mutated_same_object", "canonical post-version not newly bound")
        displaced = record["displaced_observation"]
        if displaced["semantic_binding"] == "expected_basis":
            need(displaced["identity_token"] == record["expected_basis_identity_token"], "displaced stable identity mismatch")
            need(displaced["bytes_sha256"] == record["expected_basis_fingerprint"], "displaced basis bytes mismatch")
            need(displaced["version_token"] != record["expected_basis_version_token"] and displaced["version_transition"] == "primitive_mutated_same_object", "displaced post-version not newly bound")
    if schema_id == "handbook.generic-commit-marker" and record["outcome"] == "refused" and record["refusal"]["code"] == "publication_basis_conflict":
        native = next((item["record"] for item in positives.values() if item["record"].get("result_fingerprint") == record["publication_result_fingerprint"]), None)
        need(native is not None, "conflict native result missing")
        if native["basis_disposition"] in {"replacement_source_deleted", "replacement_source_substitution_preserved"}:
            expected = native["replacement_bytes_sha256"]
            observed = next((item["bytes_sha256"] for item in (native["canonical_observation"], native["candidate_observation"]) if item["semantic_binding"] == "unexpected_basis"), None)
        else:
            expected = native["expected_basis_fingerprint"]
            observed = native["candidate_observation"]["bytes_sha256"] if native["basis_disposition"] == "expected_basis_restored" else native["canonical_observation"]["bytes_sha256"]
        need(record["refusal"]["expected_fingerprint"] == expected, "conflict expected fingerprint mismatch")
        need(record["refusal"]["observed_fingerprint"] == observed, "conflict observed fingerprint mismatch")


for item in positives.values():
    semantic_validate(item["record"])

executed = set()
for item in vectors["semantic_rejection_vectors"]:
    vector_id = item["vector_id"]
    need(vector_id in checker["required_semantic_rejection_ids"], f"{vector_id}: undeclared semantic rejection")
    changed = mutate(positives[item["base_vector_id"]]["record"], item["mutations"])
    if item["expected"] == "schema_rejection":
        need(not validator.is_valid(changed), f"{vector_id}: schema unexpectedly accepted mutation")
    else:
        validator.validate(changed)
        try:
            semantic_validate(changed)
        except ValueError:
            pass
        else:
            raise ValueError(f"{vector_id}: semantic mutation was not rejected")
    executed.add(vector_id)
need(executed == set(checker["required_semantic_rejection_ids"]), "declared semantic rejection was not executed")

for item in vectors["schema_rejection_vectors"]:
    changed = mutate(positives[item["base_vector_id"]]["record"], [{
        "json_pointer": item["json_pointer"],
        "replacement": item["replacement"],
    }])
    need(not validator.is_valid(changed), f"{item['vector_id']}: schema mutation accepted")
for item in vectors["schema_acceptance_vectors"]:
    changed = mutate(positives[item["base_vector_id"]]["record"], [{
        "json_pointer": item["json_pointer"],
        "replacement": item["replacement"],
    }])
    validator.validate(changed)

print(f"atomic checker: PASS ({len(matrix)} rows, {len(matrix) * 2} platform cells, {len(chains)} chains, {len(executed)} semantic/schema mutations, {len(nt_executed)} NtCreateFile mutations, {len(ntfs_executed)} NtFsControlFile mutations)")
```
<!-- atomic-checker:end -->

## Preserved CRITICAL authority stop

Three coupled production surfaces remain explicit independent stops. Exact
depth-three evidence is:

| Symbol | UID | Direct callers | Affected without / with tests | Processes | Modules |
|---|---|---:|---:|---:|---:|
| `validate_transaction_inventory` | `Function:crates/engine/src/artifact_lineage_store.rs:validate_transaction_inventory` | 2 | 16 / 28 | 7 | 2 |
| `GenericArtifactLineageStoreV1::recover_pending` | `Function:crates/engine/src/artifact_lineage_store.rs:GenericArtifactLineageStoreV1.recover_pending#1` | 1 | 9 / 21 | 5 | 2 |
| `GenericArtifactLineageStoreV1::verify_committed` | `Function:crates/engine/src/artifact_lineage_store.rs:GenericArtifactLineageStoreV1.verify_committed#1` | 3 | 12 / 24 | 5 | 2 |

All three returned `CRITICAL`. Their include-tests commands, each also executed
without `--include-tests`, are:

```text
npx gitnexus impact validate_transaction_inventory --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact validate_transaction_inventory --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact recover_pending --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact recover_pending --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_committed --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --summary-only --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_committed --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --summary-only --repo C:\hcm22ar-doc-repair
```

Review 5 also made six existing production symbols newly required by any future
structured-observation implementation. GitNexus reported `CRITICAL` with
`epistemic: exact` for every target:

- `native_bound_tokens`
  (`Function:crates/engine/src/artifact_lineage_store.rs:native_bound_tokens`):
  direct callers `observe_retained_regular_file`,
  `observe_prepared_replacement`, `publish_replacement`,
  `require_prepared_replacement_unchanged`, and
  `require_installed_guards_unchanged`; 23 affected symbols at depths
  `5 / 11 / 7`, unchanged with tests; processes
  `expire_retained_result_at_for_testing`, `read_committed_authoritative`,
  `read_committed_closure`, `recover_pending`, `commit_new_locked`, and
  `evaluate_committed_read`; modules `Tests` (15 direct hits) and
  `Cluster_184` (8 indirect hits).
- `native_path_tokens`
  (`Function:crates/engine/src/artifact_lineage_store.rs:native_path_tokens`):
  direct caller `native_bound_tokens`; 17 affected symbols at depths
  `1 / 5 / 11`, unchanged with tests; processes `commit_new_locked`,
  `recover_pending`, `read_committed_closure`,
  `read_committed_authoritative`, and
  `expire_retained_result_at_for_testing`; modules `Tests` (14 direct hits)
  and `Cluster_184` (3 indirect hits).
- `native_metadata_subjects`
  (`Function:crates/engine/src/artifact_lineage_store.rs:native_metadata_subjects`):
  direct caller `native_bound_tokens`; 17 affected symbols at depths
  `1 / 5 / 11`, unchanged with tests; the same five processes and module/hit
  counts as `native_path_tokens`.
- `observe_retained_regular_file`
  (`Function:crates/engine/src/artifact_lineage_store.rs:observe_retained_regular_file`):
  direct callers
  `GenericArtifactLineageStoreV1.recovery_installed_output_set#4`,
  `verify_compare_and_write_path`, `replace_durable`,
  `observe_installed_output`, and `require_installed_guards_unchanged`; 18
  affected without tests at depths `5 / 9 / 4`, 20 with tests at depths
  `5 / 9 / 6`; processes `expire_retained_result_at_for_testing`,
  `read_committed_authoritative`, `read_committed_closure`, `recover_pending`,
  `commit_new_locked`, and `evaluate_committed_read`; without tests modules
  `Tests` (10 direct hits) and `Cluster_184` (8 direct hits), with tests
  `Tests` (12 direct hits) and `Cluster_184` (8 direct hits).
- `same_native_metadata`
  (`Function:crates/engine/src/artifact_lineage_store.rs:same_native_metadata`):
  direct callers `native_bound_tokens`, `observe_prepared_replacement`,
  `publish_replacement`, `require_prepared_replacement_unchanged`, and
  `require_installed_guards_unchanged`; 22 affected symbols at depths
  `5 / 9 / 8`, unchanged with tests; processes
  `expire_retained_result_at_for_testing`, `read_committed_authoritative`,
  `read_committed_closure`, `commit_new_locked`, `recover_pending`, and
  `evaluate_committed_read`; modules `Tests` (15 direct hits) and
  `Cluster_184` (7 indirect hits).
- `validate_native_publication_observation`
  (`Function:crates/engine/src/artifact_lineage_store.rs:validate_native_publication_observation`):
  direct callers `read_native_publication_observation` and
  `GenericArtifactLineageStoreV1.recover_pending#1`; 13 affected symbols at
  depths `2 / 4 / 7`, unchanged with tests; processes
  `expire_retained_result_at_for_testing`, `read_committed_closure`,
  `read_committed_authoritative`, `evaluate_committed_read`, and
  `recover_pending`; modules `Cluster_184` (9 direct hits) and `Tests`
  (4 indirect hits).

The exact depth-three commands executed for each newly required target, first
without and then with tests, were:

```text
npx gitnexus impact native_bound_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_bound_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_path_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_path_tokens --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_metadata_subjects --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact native_metadata_subjects --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_retained_regular_file --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_retained_regular_file --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact same_native_metadata --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact same_native_metadata --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact validate_native_publication_observation --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --repo C:\hcm22ar-doc-repair
npx gitnexus impact validate_native_publication_observation --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 3 --include-tests --repo C:\hcm22ar-doc-repair
```

These six symbols are a second explicit implementation-authority stop. This
documentation record does not authorize their edit, a substitute helper, a
selector, staging, or commit. Future implementation cannot proceed until exact
operator review authorizes the complete coupled production surface.

Review 6 makes seven further production symbols unavoidable. GitNexus reported
`CRITICAL` with exact UIDs for every target:

- `rename_store_path`
  (`Function:crates/engine/src/artifact_lineage_store.rs:rename_store_path`):
  direct callers `GenericArtifactLineageStoreV1.establish_intent#2`,
  `move_intent_to_pending#3`, `finish_suffix#6`, `recover_establishing#0`,
  `finish_suffix_recovery#5`, and `publish_replacement`; 28 affected without
  tests / 59 with tests; processes `expire_retained_result_at_for_testing`,
  `read_committed_closure`, `read_committed_authoritative`,
  `evaluate_committed_read`, `recover_pending`, `commit_new_locked`,
  `evaluate_intake_document`, and `recover_establishing`; modules `Tests` and
  `Cluster_184`.
- `publish_replacement`
  (`Function:crates/engine/src/artifact_lineage_store.rs:publish_replacement`):
  direct callers `install_output#7`, `install_descriptor#5`, and
  `replace_durable`; 17 / 19 affected; processes
  `expire_retained_result_at_for_testing`, `read_committed_closure`,
  `read_committed_authoritative`, `evaluate_committed_read`,
  `commit_new_locked`, and `recover_pending`; modules `Tests` and `Cluster_184`.
- `verify_compare_and_write_path`
  (`Function:crates/engine/src/artifact_lineage_store.rs:verify_compare_and_write_path`):
  direct callers `install_output#7`, `verify_compare_and_write_basis#2`,
  `install_descriptor#5`, and `publish_replacement`; 22 / 52 affected; the same
  six processes and two modules as `publish_replacement`.
- `require_installed_guards_unchanged`
  (`Function:crates/engine/src/artifact_lineage_store.rs:require_installed_guards_unchanged`):
  direct callers `commit_new_locked#6`, `recover_pending#1`, and
  `verify_committed#1`; 22 / 53 affected; the same six processes plus
  `evaluate_intake_document`; modules `Tests` and `Cluster_184`.
- `observe_installed_output`
  (`Function:crates/engine/src/artifact_lineage_store.rs:observe_installed_output`):
  direct callers `install_output#7`, `verify_installed_outputs#4`, and
  `install_descriptor#5`; 21 / 21 affected; the same six processes and two
  modules as `publish_replacement`.
- `observe_prepared_replacement`
  (`Function:crates/engine/src/artifact_lineage_store.rs:observe_prepared_replacement`):
  direct callers `create_replacement_scratch` and
  `open_existing_replacement_scratch`; 13 / 15 affected; the same six processes
  and two modules.
- `require_prepared_replacement_unchanged`
  (`Function:crates/engine/src/artifact_lineage_store.rs:require_prepared_replacement_unchanged`):
  direct callers `install_output#7` and `publish_replacement`; 13 / 15 affected;
  the same six processes and two modules.

The exact depth-four commands, each executed both without and with tests, were:

```text
npx gitnexus impact rename_store_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact rename_store_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact publish_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact publish_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_compare_and_write_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact verify_compare_and_write_path --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_installed_guards_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_installed_guards_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_installed_output --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_installed_output --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_prepared_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact observe_prepared_replacement --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_prepared_replacement_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --repo C:\hcm22ar-doc-repair
npx gitnexus impact require_prepared_replacement_unchanged --direction upstream --file crates/engine/src/artifact_lineage_store.rs --depth 4 --include-tests --repo C:\hcm22ar-doc-repair
```

The current top-level allowlist has five names and rejects the required
`native-publication-result.json`. A future change may admit that name only for
an exact promotion-commit transaction after the native observation exists and
while intent/observation/result/trace fingerprints agree. Every other
operation/state remains forbidden. Recovery and committed-read validation must
consume the same exact record and chain without weakening any other state.
This repair authorizes none of the sixteen CRITICAL edits. Explicit operator
approval for all three record-chain, six structured-observation, and seven
retained-handle publication surfaces is required before Rust, an implementation
selector, staging, or commit; partial approval is insufficient.

Reviews 8 and 9 freeze the future private
`open_retained_parent_relative_nt` and `query_object_id_absence_nt` helpers
inside `artifact_lineage_store.rs`. Neither exists in the indexed production
tree, so neither has a GitNexus UID or existing caller blast radius and neither
is a new existing seventeenth surface. Future implementation must nevertheless
treat both as CRITICAL children within the already stopped retained-handle
publication and structured-observation boundaries. They may add no dependency,
public signature/API, static/global/thread-local state, cleanup authority,
`CreateFileW` child fallback, or `DeviceIoControl` object-ID fallback. This
documentation repair authorizes no helper or production edit.

## Validation evidence

Completed from this worktree after all coupled edits:

- duplicate-safe JSON parse: PASS for the changed control schema/vector and
  exact Review 9 remediation dispatch;
- Draft 2020-12 schema meta-validation and vector admission: PASS for 67
  positive records, 36 schema-rejection mutations, and 4 schema-acceptance
  mutations;
- control fingerprint replay: PASS for every positive record, including the
  expected-present Unix and Windows chains and the outputless publication-
  conflict result/marker/evidence/result/ledger chain;
- executable exact matrix/chain checker: PASS for 81 rows, 162 explicit
  Unix/Windows cells, 4 complete chains, all 20 declared schema/semantic
  mutations, all 39 declared `NtCreateFile` mutations, and all 33 declared
  `NtFsControlFile` mutations; the exact matrix aggregate is
  `a5827529780955e3aafeb18261507abd296fe159e44eb42453405c738383a548`
  and the exact `NtCreateFile` contract aggregate is
  `a71ae9eb79d0c7c1ca78fa98b3890089d6471d2ff2c20de2590a5b4f9650865d`;
  the exact `NtFsControlFile` contract aggregate is
  `f4e967c0453ccd1f7dae0f842062c225b477ce8d36847ca18610f06a2334101e`;
  all 30
  source-race rows are exact, every crash without completed live continuity is
  markerless/non-retryable ambiguity, every outputless refusal keeps
  unauthenticated `C` and `R` withheld, every `D=X` path is mutation-free, and
  every Windows move uses exact retained-handle `FileRenameInformation`, every
  retained-parent child/rebound open uses the exact private `NtCreateFile`
  trace, every retained-directory post-open type/reparse observation is exact,
  every object-ID query uses the exact private synchronous `NtFsControlFile`
  raw-plus-final status trace, and every `CreateFileW` child-open or
  `DeviceIoControl` object-ID fallback rejects;
- the exact Rust frozen-control-vector test reproduces every terminal
  fingerprint and operation-derived transaction ID: PASS, including the
  expected-present transaction
  `artifact_candidate_promote_1b676fa12ce7912409dc0c529a320a628f0a8c6fa6360543ab862707490886b7`;
- semantic replay: PASS for all 67 positive records and 20 schema-or-semantic
  rejection mutations, including wrong `C/R/D` identity, bytes, component
  digests, move-invariant metadata/side-streams, opaque versions, backup
  mismatch, zero/all-ones/Object-ID failure, numeric-only continuity, and retained-
  ancestor/parent/junction cases;
- deterministic vectors: 61 explicit-platform atomic-publication vectors, 81
  exact matrix rows, 4 chain contracts, 55 negative vectors, 39
  `NtCreateFile` rejection vectors, and 33 `NtFsControlFile` rejection vectors
  parse and retain unique IDs;
- Markdown local links, cited local anchors, primary-reference links, and fence
  balance: PASS across all five changed Markdown files, including 12 local and
  57 external links;
- exact dispatch schema/meta admission: PASS. The repository-wide handoff
  validator passes using the repository-visible Windows worktree: 3 record
  schemas, 2 internal-dispatch schemas, 2 templates, 52 records, 292 current
  internal dispatches, 8 admitted legacy dispatches, and 52 ledger entries. Its
  historical-v1 admission and orchestration-contract self-tests also pass;
- 81-entry pre-remediation manifest ordering, uniqueness, encoding, aggregate
  replay, and active-dispatch self-exclusion: PASS at
  `sha256:7535dca0e75cf5b71280ccb56bb1491b6f42a995d99d45bc6c7b42fa4e21312a`;
- live preservation: PASS; only the seven authorized documentation paths
  differ from the dispatched subject, and no crate or fixture path changed;
- branch/HEAD replay: PASS for `codex/hcm-2-3-planning` at
  `3c49fa2c6d653f4b1a703d1d5c5196147b003533`;
- `git diff --check` and bounded seven-file trailing-whitespace/final-newline
  check: PASS;
- exact zero delta from the dispatch under `crates/**`: PASS for every manifest
  hash; no crate path appears in the seven-path remediation delta; and
- final GitNexus unstaged change detection reports the preserved broader dirty
  implementation subject as expected at CRITICAL: 27 indexed files, 122
  changed symbols, and 32 affected processes. This documentation remediation
  changes no manifest `crates/**` byte and does not waive that risk. Both future
  private helpers are absent from the index, so this repair discovers and edits
  no additional existing HIGH/CRITICAL symbol.

## Disposition

The documentation-only dispatch stop condition is not reached: the documented
state machine uses existing native platform APIs and introduces no dependency,
public signature, new byte/count limit, HCM-2.2 change, cleanup authority, or
threat-model narrowing.
The future-implementation stop condition remains reached because Review 6 adds
seven exact CRITICAL retained-handle publication surfaces to the prior nine.
Reviews 8 and 9 freeze future private `NtCreateFile` and `NtFsControlFile`
helpers inside the already stopped publication/observation boundaries without
creating a seventeenth existing surface. This record does not declare the
current implementation clean, does not authorize the sixteen-symbol
implementation surface, either helper, or any selector, and is only
`REVIEW_READY`: a different fresh complete-subject documentation review remains
required before operator authorization or Rust may resume.

## Successor validation evidence

- Executable checker: PASS for 81 rows, 162 explicit platform cells, 4 chains,
  20 semantic/schema mutations, 39 `NtCreateFile` mutations, and 36
  `NtFsControlFile` mutations.
- Exact corrected aggregates: matrix
  `382b71d415c39773f3389c2cf1700d89ea46608d6e0fec3f02c96871788f3e84`;
  `NtCreateFile`
  `a71ae9eb79d0c7c1ca78fa98b3890089d6471d2ff2c20de2590a5b4f9650865d`;
  `NtFsControlFile`
  `c15f5c098cea470a4e637d2bc1ecb335c6641f9b052e8c37a87792b7a7403d0a`.
- Duplicate-safe JSON and Draft 2020-12 schema/vector admission: PASS.
- Handoff validation: PASS for 3 record schemas, 2 internal-dispatch schemas,
  2 templates, 52 records, 309 current internal dispatches, 8 admitted legacy
  dispatches, and 52 ledger entries; both historical-v1 and orchestration
  contract self-tests pass.
- Markdown local links and fence balance: PASS across the four corrected
  Markdown authority files (69 links).
- Primary Microsoft authority: HTTP 200 for `NtFsControlFile`,
  `NtSetInformationFile`, `FILE_RENAME_INFORMATION`, and `IO_STATUS_BLOCK`.
- Frozen evidence: Review 10 remains
  `sha256:8a593688c1573cc62b4ee21b8fa279edf03b74b2b65a08f2f9eded060f355921`;
  the predecessor proof remains
  `sha256:d8d183f0f121123326eeefa38e51821c7e14b8d3e5fea66d37a7a422fee3015f`.
- `git diff --check`: PASS. Rust formatting is intentionally deferred to the
  reopened implementation wall because preserved implementation bytes are
  frozen in the pre-implementation successor subject; no production byte is
  changed by this documentation gate.
- `handbook-engine` remains `0.1.1`; no Cargo or package-version file changed.

## Successor disposition

The paragraph above is the inherited predecessor disposition and remains
historical. The current operator has already authorized the bounded
implementation lineage. This successor does not reopen or rewrite that lineage;
it corrects only the two enumerated contracts.

The corrected documentation is `REVIEW_READY` for the next frozen Review 10
successor. A CLEAN verdict may update only selector/dispatch references and then
admit edits to `query_object_id_absence_nt` and `publish_replacement`. Any need
for another production symbol, unsafe item, helper, dependency, Cargo/version
change, global, public API, fallback primitive, or broader completion rule is an
immediate stop. The final implementation proof wall and GitNexus change
detection remain required, and no staging, commit, merge, or handbook-engine
version change is authorized.
