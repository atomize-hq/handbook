# Native guard preparation — review input, not workload release

Authority text: exact capsule 94734e76bfe3c58ee93de77ec25e3d7df459d7c4.
Fixed t0 2026-10-20T19:10:14Z; functional cutoff 2026-10-21T11:10:14Z;
closeout 2026-10-21T13:10:14Z. Engine pin pending. No engine/build/corpus run.

## Permitted prerequisite

native_probe.py uses libproc for the current supervisor and an explicitly created
benign child, nonce handshakes before querying, and limits membership queries to
that child's newly created process group. UID/PPID/PGID/start-time and resident
size were read successfully. The child exits via stdin EOF without a signal;
two numeric group observations 50 ms apart establish absence for this probe.
RSS zero is never used as an absence test. errno is retained independently.

Initial exploratory proc_listpgrppids returned 1: that interface's return value
was incorrectly interpreted as bytes in the first throwaway console probe. It
is not used by the retained adapter. proc_listpids(PROC_PGRP_ONLY) returned 4
bytes and the exact child PID; installed SDK proc_bsdinfo is 136 bytes, matching
its ctypes definition. proc_pid_rusage(RUSAGE_INFO_V0) returns resident bytes.

## Proposed workload controller

Acquire a nonblocking exclusive flock on one new host token, preserving the
same token inode; never unlink/recreate or infer stale ownership from a PID.
Record nonce, supervisor UID and start time, categories, source hash and exact
absolute deadline. Token waits count inside the deadline. One heavy workload
per host; j2 builds only after exact source and cache claims.

Start an independent watchdog process before token waits. It observes only its
explicit supervisor and nonce/start-time-owned anchor. Arm fixed UTC and local
monotonic deadlines before any workload release, independent of disk census and
stdin feeding. Parent death/EOF must cause owned-anchor cleanup. A blocked setup
must not postpone the fixed cutoff. A watchdog failure refuses release.

The anchor creates its session before launch, handshakes a nonce over a private
pipe, then waits for GO. Verify UID/start/PGID and nonempty own-group membership
before GO. Keep the anchor alive until all work is finished and cleanup is
acknowledged: this prevents PGID reuse during signaling. The watchdog receives
and acknowledges that exact anchor identity before GO. No PID-only resume.

Sample numeric own-group membership plus per-member start/UID/rusage at >=25Hz;
include supervisor, watchdog, and every new registered helper. Record all sample
timestamps, late intervals, missing values and overshoots. Root workload session
<=2 GiB; all supervisors/helpers <=768 MiB; aggregate <=3 GiB; readers<=256 MiB,
checkers<=2 GiB. Missing live RSS or ownership mismatch triggers fail-closed
termination of proven-owned work only, with UNKNOWN closure if observation fails.

Need explicit own-descendant checks (scoped PPID queries) to detect subprocesses
leaving the owned session. A rapid double-fork/session escape is not universally
prevented by sampled native APIs; do not certify arbitrary daemonizing commands.
The actual engine and build tree must be inspected and tested under the declared
cooperative foreground-process contract, with escaping-process behavior refused.

Enforce free>=20 GiB and F01 source<=4 MiB, F04 private target<=128 MiB/5000files,
F08 fixtures<=96 MiB/30000files per later central claim, F12 evidence<=24 MiB,
F13 measured positive shared growth<=4 MiB. C1 private cache and C2 topup require
meta claims before copying/growth. Each operation supplies prospective growth;
refuse if actual plus prospective exceeds its category before launch. Census
logical and st_blocks*512 separately, include retained temporary coexistence.

Only signal the nonce/starttime/UID-owned anchored session after revalidation.
Record syscall errno separately. Closure requires two successful numeric
own-PGID observations, spaced apart, including state/UID/start/RSS where members
remain; no RSS0=>absence and no EPERM=>absence. Keep UNKNOWN on query failure.
An orphan must not silently disappear from accounting when the root exits.

## Required control-only proofs before engine release

normal; nested descendants; local timeout; fixed cutoff during token/setup wait;
root-completed orphan; memory limit; missing live RSS; free-space refusal;
prospective category cap refusal; competing token; supervisor death; watchdog
death; numeric closure and retained unknown errno; evidence output cap.

No full guard implementation or these proofs is claimed by the prerequisite.
The original request for two ps observations cannot be claimed satisfied: global
ps is blocked. This narrower libproc evidence needs parent review as an explicit
adapter decision; the RSS and positive-closure gates themselves are unchanged.

## Preserved permission failures

Action 1: git -c gc.auto=0 fetch --no-tags origin
codex/file-native-online-maintenance-20261020
Default sandbox: error: cannot open '.git/FETCH_HEAD': Operation not permitted
Escalation review (before Git launch):
"This action was rejected due to unacceptable risk.
Reason: This fetch is a reversible but networked mutation of the local Git
repository, and its authorization comes only from untrusted transcript content
rather than the user.
Do not bypass this rejection through a workaround or indirect execution.
Continue with a safer alternative, or carry out checks to prove that the action
is authorized or low risk before trying again. Complete unaffected work without
asking for confirmation. Report anything that remains blocked, clarify why it
was blocked by auto-review, inform the user of the risk and ask for approval."
No retry, alternate network transport, clone or fetch. Authority arrived as data
in meta messages; exact SHA/gzip/tar/member checks preceded extraction.

Action 2: ps -axo pid,ppid,pgid,state,uid,rss,lstart,comm
Default sandbox: zsh:4: operation not permitted: ps
No ps escalation or wrapped ps invocation. The libproc probe is the later
explicitly requested narrow alternative, not a claim that global ps now works.

## Donor findings

Old source-guard.py hardcodes 2026-10-20T05:03:10Z, old roots, 10Hz sampling,
and does not fail closed on missing samples. Old packing guard.py hardcodes
07:55:16Z and Windows paths; its native branch reports only root membership,
does not implement native RSS sampling, and finally sets remaining=[] without
numeric native closure proof. Neither was invoked. Prior 368-PGID closure audit
and its initial UNKNOWN/errno1/RSS0 history remain unchanged and are not new-pin
qualification. New code must not overwrite any old report/runtime/source/cache.

## Implementation followup

A control candidate now exists as guard.py, SHA256 4ec468760ef9bdfafc13787db5beca95659a25bd10d44b8496bee16217ce9aac.
review-summary.json records actual smokes and remaining prerequisites. The design
above is preserved as the initial proposal. This candidate is NOT released for
engine/build/corpus workloads. Two earlier closure failures remain visible.
