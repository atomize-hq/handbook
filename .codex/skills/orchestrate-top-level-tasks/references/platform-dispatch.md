# Handbook Platform Dispatch and Human Handoff

## Direct evidence

Dispatch a fresh read-only top-level evidence task only when a selected Handbook slice declares an
exact native gate. Match exact project path, project ID, host ID, repository status, operating
system, commit, and tree. Titles and summaries are not identity.

The local-only checkpoint must already be accessible on the evidence host. Never push or broaden
publication merely to satisfy an evidence task. If the host cannot resolve the exact commit/tree,
use the human handoff path.

## Evidence contract

Bind the evidence task to:

- meta workflow ID and nonce;
- evidence ID and consuming slice;
- local integration ref, source commit, and tree;
- exact host/project/path/OS;
- allowed commands and observations;
- prohibited mutations and lifecycle actions;
- artifact path/digest and satisfied gates;
- meta return route.

Evidence tasks never edit, commit, update refs, or publish. Their receipt is external scheduling
evidence. The consuming increment must make the accepted proof durable in its v1.4 handoff.

## Unavailable platform

Return `BLOCKED_PLATFORM_HANDOFF_REQUIRED` with:

- unavailable platform/host/project;
- source ref, commit, and tree;
- blocked slice and proof gate;
- exact prerequisites and commands;
- evidence fields and prohibited actions;
- a complete fresh-session continuation prompt;
- meta task/thread/host identity and nonce;
- a safe local-only source-transfer requirement.

Do not push, simulate native proof, or claim the gated slice complete.
