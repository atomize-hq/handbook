# Native positive-start and owned process-control interface proposal

Status: **read-only design, awaiting the unified core's exact stable API**. No
accepted guard/driver/dispatcher source was changed and no control, engine or build
ran. Native engine HOLD and existing resource caps remain. This document does not
claim implementation or authorize shared portable-case-source edits.

The inspected Windows `Driver.call` accepts `on_started=None` explicitly, verifies
positive live RSS after resume, invokes the callback before stdin, and records
start/resume/observation/stdin/done times. The accepted Linux driver currently
creates an owned blocked launch pipe with positive RSS and pidfd identity, but
does not expose that callback or a controlled reader-death/cancel handle. Passing
`on_started` through its current `**fields` would incorrectly enter the engine
payload; the eventual parameter must be explicit and excluded from JSON.

## Proposed portable contract to reconcile with the core owner

Keep `call(root, action, ..., on_started=None)` synchronous. It owns process creation,
stdin/stdout/stderr, timeout, monitoring, reaping and the final raw event. The core
may coordinate call threads but must not spawn or signal an unowned process.

An optional start callback receives a JSON-safe positive-start event. Preserve
the existing Windows fields (`pid`, `resumed_ns`, `observed_ns`, `positive_rss`,
`before_stdin_delivery=true`) and add identity/control metadata only as the stable
shared API permits. Proposed native metadata is `process_start_ticks`, `pgid`,
`guard_nonce`, binary SHA/source/role, RSS limit and an opaque control token. The
event's PID is diagnostic, never sufficient authority to kill a process.

Suggested capability operations, **names not yet implemented or fixed**:

| Operation | Native meaning and boundary |
| --- | --- |
| `kill(handle, reason)` | Identity-bound pidfd SIGKILL for that registered child only, with an intentional case reason/phase receipt. No bare PID, process name, foreign group or caller-selected signal. |
| `cancel(handle, reason)` | Driver cancellation of that owned call, with its explicitly selected termination policy. Proposed native process-cancel policy is SIGTERM with bounded SIGKILL escalation. This is not an API request-cancellation fence or proof of noncommit. The shared core must agree the semantic label before implementation. |
| `wait(handle, timeout)` | Wait for the call owner's completion event, including pipe collection, root reaping and joined watcher; return immutable identity/exit/control receipt data. It must not compete with `communicate()` or become a second pipe/reaping owner. |

If the core requires cooperative engine cancellation, register an exact owned
cancel path from the captured call environment and use that explicit channel;
unsupported reader cancellation must refuse rather than silently change into a
semantic fence or process death. Existing `EXP_PACK_CANCEL`/API cancel cases remain
separate from driver process cancellation. A signal after stdin may race acceptance
for writer/maintainer calls, so selected authority and outcome oracles still decide
what happened. The core never infers rollback from signal delivery.

## Native positive-start ordering

1. Resolve the exact executable/source/role and live ancestor guard claim. Create
   only a child inherited by the accepted owned stage group, retain its pidfd and
   process start ticks, then verify positive blocked-launch RSS under the role cap.
2. Arm per-call memory/deadline control while the child is blocked. All setup and
   callback time is inside `min(call deadline, stage deadline, fixed cutoff)`.
3. Release the launch pipe so the known executable can enter its pre-input read.
   Before announcing **engine** start, confirm the actual exec binding and a fresh
   positive live RSS sample: a Python launch-wrapper RSS sample alone is not proof
   that the ELF engine has started. For an ELF engine compare the `/proc/<pid>/exe`
   identity to the frozen executable; verify start ticks and inherited ownership.
   Script-based fake controls need a separately labeled script/interpreter binding,
   or use a bound harmless native ELF peer. Do not weaken the real engine check.
4. Register the opaque capability in the driver before publishing the callback
   event. Invoke exactly once before any JSON request byte is delivered. A callback
   exception/expired deadline kills and collects the owned child, retains the
   failure event and sends no request. Recheck the deadline before stdin.
5. The callback normally publishes its event/token to a core queue/latch and
   returns. Barrier waits, kill/cancel and completion waits occur in the core's
   coordination thread after callback return. Waiting for this same call's
   completion inside the before-stdin callback would deadlock.
6. If the final core API needs a blocking start rendezvous, define its short timeout
   and failure mechanics explicitly. A bounded callback runner/independent timer
   must continue enforcing the call cap; an unjoined callback cannot be reported
   clean. No Python thread is killed unsafely and no callback is allowed access to
   a closed/recycled pidfd. This behavior requires review before implementation.

## Handle identity, concurrency and lifetime

The registry binds a random token to the driver instance, suite/guard nonce,
PID/start ticks, pidfd, executable SHA/source role and call identity. A serialized
token contains no raw OS handle. Reject forged tokens, another driver's token,
another guard generation, and stale/released handles before any signal. Use the
pidfd for delivery and start ticks for observation; never reopen a recycled PID to
regain authority. Registry actions and completion/FD closure use one mutex/state
machine so a late callback cannot signal a closed or recycled descriptor.

Only the `call` owner collects pipes/reaps the child and writes the operation
receipt. Control actions append request/delivery observations to that call. A
controlled kill is accepted only when the declared case expects that operation and
identity; it does not suppress unrelated RSS/deadline/monitor errors. Duplicate
kill/cancel requests are idempotently recorded. Waiting after completion returns
the same immutable outcome. Active handles are bounded by declared concurrency;
completed entries keep only small receipt metadata until the case joins them.

The native outer guard still owns every descendant and escaped-child/subreaper
closure proof. Per-call completion is direct-root/I/O/watcher completion, not a
replacement for two empty numeric group observations. No new unregistered heavy
supervisor or independent process group is created. Callback/control threads are
part of the existing monitored stage process.

Readers retain the explicit action-based256MiB limit regardless of executable
basename; checkers/maintainers retain2GiB. Neither the callback nor a handle may
raise limits or change role after launch. Native outer heavy/supervisor/host caps,
free/disk/prospective claims, actual cadence counters and absolute cutoff are
unchanged. Record source/driver/guard/control hashes, initial and actual-exec RSS,
identity, start/callback/stdin/control/done timestamps, requested/delivered signals,
exit and watcher/group closure separately.

## Targeted controls after API handoff and source review

No probes ran during this plan. The identified interface gap justifies these
future focused controls rather than another repetition of all accepted guard tests:

- Positive actual-exec callback happens once before stdin, with valid identity and
  role RSS. An omitted callback preserves ordinary `call` behavior.
- A held fake reader is killed through its token after an owned ready barrier;
  the exact root is reaped, another held peer stays alive, and final group closure
  is positive. Separate owned process-cancel and cooperative-cancel cases keep
  their different meanings.
- Foreign, forged, stale and already-released tokens cannot signal another process.
  Concurrent wait/control does not duplicate pipe reads or reaping.
- Callback exception and callback timeout fail closed before stdin, with no clean
  result if callback/watcher lifetime remains outstanding. Fixed setup/call/stage
  deadlines still apply.
- Explicit 256MiB reader enforcement works for a renamed bound executable.
  Preserve all actual RSS interval maxima/counts and raw closure receipts.

These are process/interface controls only. Final-pin engine pin/drop/error/death,
owner, capture-gap and recovery schedules still require the unified source cases
and full independent boundary oracles. The core owner must relay the exact API;
only then may native-owned driver changes be proposed, reviewed and qualified.
