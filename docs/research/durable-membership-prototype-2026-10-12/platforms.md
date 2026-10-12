# Platform and fault boundary

Both variants use the same `src/main.rs` and `src/transaction.rs` implementation
and exact checkpoints. Transaction preparation holds no writer lock. Commit
validates native owner/lock, exact expected root and recorded point dependencies
before mutation. Whole-root mismatch is a known nonpublication conflict; failed
object/root staging is known prepublication failure. Starting the root replacement
attempt conservatively makes later errors indeterminate, even if a particular
native error and subsequent observation show the old root. Cancel/drop never
repairs or rolls back a possible publication. Exact terminal replay still repairs
the selected-root boundary before acknowledgement.
The selected threat model is cooperating local processes on an established
private experiment directory. Native directory and lock identities are retained
and compared; replacing or deleting their positive evidence refuses. Metadata
checks reject symlinks/reparse points. This layer does not call the Handbook
retained-root/OperatingGuard APIs, inspect its full grants or validate its owner
proof closure. Same-user malicious concurrent namespace replacement and
noncooperating publishers are excluded.

## Selected primitives

| Boundary | Windows | Linux | macOS remaining |
| --- | --- | --- | --- |
| Process writer lock | fs2 0.4.3 `LockFileEx`, whole lock-file range | fs2 0.4.3 `flock(LOCK_EX)` | Native flock semantics and process termination qualification |
| Native identity | `GetFileInformationByHandle`: volume serial + file index, directory backup-semantics handle | metadata `dev` + `ino` on retained directory/file descriptors | Native identity/replacement qualification on APFS |
| Prerequisite bytes | `write_all`, `sync_all` (FlushFileBuffers) | `write_all`, `sync_all` (fsync) | Test fsync and select/qualify stronger device flush, e.g. F_FULLFSYNC, where required |
| Object name installation | Same-directory `MoveFileExW(WRITE_THROUGH)`, without REPLACE_EXISTING | Same-directory rename under cooperative lock | Same-filesystem rename/open-handle tests |
| Root replacement | `MoveFileExW(REPLACE_EXISTING\|WRITE_THROUGH)` under cooperative expected-root check | rename under cooperative expected-root check | Native root/held-reader cases |
| Name persistence | No directory-fsync claim; write-through rename and selected-root file flush | fsync touched shard dirs, objects dir, then store dir after root replacement | Directory name persistence and crash/power behavior on APFS |
| Reclamation | None; old readers, roots, proposals and staging remain | Same | Qualification of file deletion with open handles/mappings before adding any GC |

`rename` is not CAS. The retained native lock serializes the expected-root check
and replacement for cooperating publishers. Object IDs only supply integrity.
Windows same-volume no-replace uses the selected native call; the Unix absence
check followed by rename is only safe within this cooperative-writer boundary.
No claim is made against noncooperating writers.

Source evidence:
[fs2 0.4.3 Windows source](https://docs.rs/crate/fs2/0.4.3/source/src/windows.rs),
[Unix source](https://docs.rs/crate/fs2/0.4.3/source/src/unix.rs),
[MoveFileExW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw),
[FlushFileBuffers](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers),
[Linux fsync](https://man7.org/linux/man-pages/man2/fsync.2.html).
The Windows move documentation's explicit flush guarantee concerns copy/delete;
these experiments do not turn the same-volume write-through call into hardware
power-cut qualification. Linux's file fsync needs a separate directory fsync for
name persistence. Storage devices and WSL2's host/VHDX stack remain part of the
unqualified physical persistence boundary.

## Fault matrix and interpretation

| Injection/case | Expected observation | Evidence category |
| --- | --- | --- |
| object_write, object_sync, objects_dir_sync | Old selected root; unselected stage/objects may remain; acknowledged originals replay | exit(86) and injected I/O error |
| object_install | Old selected root; installed orphan may be reused only with its shard name boundary included in retry publication | exit(86) and injected I/O error, focused repair runner |
| root_stage, root_sync | Old selected root; incomplete/unselected metadata ignored without choosing newest survivor | exit(86) and injected I/O error |
| root_replace, publication_sync | Complete new selected state may already be visible; error supplies no acknowledgement | exit(86) and injected I/O error |
| before_reply | Committed state, lost reply; exact retry returns original outcome | exit(86) and injected I/O error |
| replay_sync | Exact retry reflushes the selected ROOT and store-name boundary under the retained lock before acknowledging; injected error suppresses the reply | Focused recovery-route verification |
| short_write | Incomplete unselected staging object; explicit error; old root preserved | Injected prefix write and error |
| external kill during prepared pause | No publication; acknowledged previous original retained | OS process termination |
| torn/missing/checksum-invalid root, missing/corrupt selected node | Explicit refusal; no survivor choice or membership rebuild | On-disk corruption/loss |
| lost OWNER/LOCK/object namespace or replaced LOCK | Explicit refusal; no recreated native proof | On-disk identity/loss |
| corrupt unrelated named object | Exact lookup may succeed; explicit broad audit fails | Experimental detection boundary, not production selection |
| valid ID placed in the wrong shard, including a corrupt duplicate of a canonical object | Broad audit checks shard/name and the actual enumerated regular-file boundary and refuses | Selected P3 repair; canonical selected membership itself remains intact |
| buffered transaction cancel/drop/conditional failure | No selected change or object staging; closed transaction refuses reuse | Native transaction API and lifecycle tests |
| stale prepared root | Explicit conflict; generic transaction closes; caller re-begins/revalidates, without loss or silent callback rerun | Native 1/2/4/8 process transaction cohorts |
| held snapshot reader while another publishes | Reader retains old root, later reader sees new state; no reclamation | Two OS processes |
| held ROOT handle | Windows handle excluding delete sharing blocks replacement and returns error; Unix open descriptor retains old root while path changes | Native open-handle behavior |
| coherent all-local old-root restoration | Accepted old root hides later history; no local external freshness witness | Explicit excluded threat demonstration |

All checkpoint faults are checked for actual reachability. Faults are injected
after the named call/checkpoint, and `short_write` writes a prefix. A real flush
may have completed before an injected error. These are conservative error-path
and process-interruption tests, not a volatile-filesystem persistence simulator.
The OS cache survives process death; no guest reboot, host reboot, storage cache
loss, hardware power cut, or device failure is claimed.

No mmap component is used. Held immutable snapshot/root tests establish reader
reachability under the no-GC policy, not mapping/truncation safety. No macOS
runner was available among the local Windows/WSL2 capabilities. Portable logical
encoding and narrow cfg boundaries are preparation, not native support evidence.
Before a production selection: run the same harness on native macOS/APFS;
qualify rename/share modes, directory persistence, F_FULLFSYNC/device semantics,
kill/reboot recovery, reader lifetime and any eventual reclamation.

The follow-up's executable `scripts/transaction_tests.py` uses the Unix held-ROOT
descriptor route on macOS, while Rust separates native identity/rename/sync at
small cfg boundaries. `transaction_measure.py --cohort-trials 0` can run ordinary
APFS measurements. Its existing overlap RSS helper is Linux/Windows-specific;
macOS reports unavailable RSS rather than fabricating a measurement. Environment
capture likewise records unavailable Linux-only inventory commands. This is a
concrete harness target, not native macOS proof. Qualify APFS directory fsync
errors, F_FULLFSYNC/device ordering, rename with open handles, process lock death,
owner/lock loss, old-descriptor/new-path readers, exact replay repair, injected
short writes and each publication boundary on that runner. No OS/service was
installed or reconfigured for this experiment.

The independent review found two sequencing defects in the first measured common
layer. The final code includes the reused shard in every proposal's name boundary
and re-establishes selected-root publication durability before terminal replay.
`scripts/review_regression.py` exercises both repair routes on Windows and Linux;
the original measured source remains in `evidence/measured-main.rs`. The new
`name_boundaries` counter counts directory-helper calls (validation only on Windows),
not Windows directory fsync. `replay_repairs` counts successful root repair routes.
