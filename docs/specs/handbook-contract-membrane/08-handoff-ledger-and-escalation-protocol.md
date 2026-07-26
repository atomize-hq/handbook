# Handoff Ledger and Escalation Protocol

## Purpose

One top-level phase/slice orchestrator owns the canonical continuation record. Internal implementation, documentation, proof, remediation, and review subagents return structured results to that parent; they do not create global handoffs or ledger entries.

The protocol supports:

- review-clean slice completion;
- safe top-level continuation across a real context/runtime boundary;
- human-visible or interactive action;
- external blockers;
- Resolution/authority escalation;
- parent-managed documentation repair and finding-driven decomposition;
- built-in delegation evidence and review/remediation/re-review lineage;
- optional top-level start/end Snapshot Memory refs and deterministic deltas until snapshot capture becomes mandatory.

A dispatch is an execution/audit envelope. Creating an internal dispatch is not completion and normally does not return control to the user.

## Storage model

~~~text
handoffs/
├── handoff-record.schema.json          # immutable v1.0 historical schema
├── handoff-record.v1.1.schema.json     # immutable v1.1 historical schema
├── handoff-record.v1.2.schema.json     # immutable predecessor schema
├── handoff-record.v1.3.schema.json     # current parent-closeout schema
├── ledger-entry.schema.json
├── internal-dispatch.schema.json       # hash-admitted v1.0 first-review schema
├── internal-dispatch.v1.1.schema.json  # immutable predecessor envelope
├── internal-dispatch.v1.2.schema.json  # current advisory-aware envelope
├── handoff-template.json               # current v1.3 parent template
├── internal-dispatch-template.json     # current internal JSON dispatch template
├── dispatch-template.md                # human-readable field/instruction guide
├── validate_handoffs.py
├── ledger.jsonl
├── records/
│   └── <immutable-parent-or-historical-records>.json
└── dispatches/
    ├── <hash-admitted-legacy-dispatches>.md
    └── <current-internal-dispatches>.json
~~~

## Canonical truth and immutability

- Each file under records/ is an immutable canonical handoff record.
- ledger.jsonl is a byte-deterministic rebuildable query index, not a second authority.
- Corrections create a new record whose supersedes array names only the prior recommendations or facts it replaces.
- source_handoff_ids records consumed resume context without implying that every source is superseded.
- Handoffs reference snapshots/deltas and semantic records; they do not duplicate or change those records' authority.
- Internal JSON dispatches are immutable bounded execution envelopes.
- The eight pre-correction Markdown dispatches remain immutable evidence of the user-routed workflow defect; they are not migrated into the current internal format.
- Existing v1.0 through v1.2 records remain immutable evidence and are never rewritten into v1.3.
- The first HCM-0.8 internal-dispatch v1.0 review remains hash-admitted evidence of the findings that required v1.1; it is not rewritten.

validate_handoffs.py hash-admits the exact historical v1.0/v1.1 record filenames, IDs, versions, and bytes plus the exact eight legacy Markdown dispatch filenames and bytes. Unknown new historical-version records, missing history, or byte changes fail closed.

New v1.3 records and current internal-dispatch v1.2 JSON are validated against
their schemas, replayable subjects, identities, cross-record lineage,
structured finding disposition, and Git-reviewed diffs. Once committed, they
are immutable and corrections are additive.

## Handoff schema routing

handbook.session-handoff records route only by top-level schema_version:

| Version | Schema | Creation policy |
|---|---|---|
| 1.0 | handoff-record.schema.json | Historical-only exact admission; never create. |
| 1.1 | handoff-record.v1.1.schema.json | Historical-only exact admission; never create. |
| 1.2 | handoff-record.v1.2.schema.json | Immutable predecessor; validate but never create. |
| 1.3 | handoff-record.v1.3.schema.json | Required for every new top-level closeout. |

V1.3 requires:

- session.kind=orchestration;
- orchestration_id and source_handoff_ids;
- stop_reason and delegation-capability evidence;
- reviewed-state fingerprint/proof refs;
- proof-relevant delegated_runs, typed parent/delegated remediations, and their lineage;
- structured finding priority, status, and source-review-run linkage;
- snapshot_refs and semantic_refs;
- resume rather than the historical queue-shaped next_session object.

A completed v1.3 record requires stop_reason=completed, available built-in
delegation, a completed clean review, no unresolved P1/P2, registered P3/P4,
and resume.execution_target=none. capability_unavailable requires
status=blocked.
A clean review may carry validated P3/P4 advisory refs under
`09-review-finding-inventory.md`; it carries no unresolved P1/P2.

## Dispatch routing

The first HCM-0.8 review used hash-admitted handbook.internal-dispatch v1.0.
Replayable-subject dispatch v1.1 remains immutable predecessor evidence.
Current internal dispatches use handbook.internal-dispatch v1.2 JSON and
internal-dispatch-template.json.

The schema requires:

- parent orchestration, phase, slice, packet, and subject fingerprint;
- a sorted canonical repository-relative path/SHA-256 manifest whose forward-slash entries exclude drive, UNC, absolute, backslash, embedded-NUL, empty, dot, dot-dot, and trailing-separator forms, resolve beneath the repository root, have an aggregate that is always recomputed, are checked against live files when executed, and whose final clean subject is replayed from the primary `reviewed_state.baseline_head` at completed closeout;
- execution_target=internal_subagent;
- agent_type=default and fresh_context_required=true;
- closeout_owner=parent_orchestrator;
- ordered required_skills beginning with using-agent-skills;
- exact authority, repo truth, allowed scope, non-goals, tasks, gates, and stop conditions;
- review-result P1-P4 classification plus P3/P4 fix, inventory, or duplicate
  disposition;
- review dispatches require `advisory_disposition` in their structured return
  field set;
- built_in_subagent result transport;
- explicit prohibition of global handoff, ledger write, and user task hop.

dispatch-template.md is explanatory guidance for these fields and for rare top_level_resume/human_interactive presentation. New internal proof runs use JSON so the execution envelope can be schema-validated and fingerprint-bound.

## Snapshot, handoff, dispatch, and result roles

| Record | Question answered |
|---|---|
| Snapshot Memory | What selected state was observed at a top-level or strategic internal boundary? |
| SnapshotDelta | What changed between compatible observations? |
| Handoff | Why did top-level orchestration stop, what happened, and how may it resume? |
| Internal dispatch | What exact built-in subagent job is authorized by the active parent? |
| Delegated-run result | What did that built-in agent return, against which subject, and how did the parent dispose it? |

Delegated-run results are recorded in the parent handoff when proof-relevant. They do not become standalone global handoffs merely because a subagent turn ended.

Snapshot refs remain nullable until HCM-3.4 lands. V1.2 snapshot_refs still requires an honest capture_status:

- captured: all applicable refs exist;
- partial: a bounded subset exists and omissions are named;
- failed: required capture failed and a blocker/finding is recorded;
- not_available: capability has not landed and refs remain null.

Never invent a snapshot ref. semantic_refs remains mandatory and uses empty arrays/null when no applicable semantic record exists.

## True top-level stop reasons

| stop_reason | Required meaning |
|---|---|
| completed | Selected slice/top-level objective is proof-complete, review-clean, and committed in the reviewed-slice commit. |
| human_input | Exact user judgment, approval, or interactive observation is required. |
| external_blocker | Named state outside the repository prevents progress and has an exact recheck condition. |
| authority_boundary | Broader scope, Resolution, or decision authority is required. |
| context_boundary | Current top-level context/runtime capacity cannot safely finish the active loop. |
| capability_unavailable | Mandatory built-in subagent execution is unavailable; no external/self-review fallback is allowed. |

Local remediation, a child packet, review findings, cross-document repair, or a local proof gap is not a top-level stop reason by itself.

## Status model

| Status | Meaning |
|---|---|
| completed | The selected top-level objective and all local gates finished review-clean. |
| partial | Safe work remains in the same authority but a genuine top-level resume boundary was reached. |
| blocked | A concrete external/capability dependency prevents progress. |
| escalation_required | Broader scope, authority, Resolution, contract, or planning decision is required. |
| review_required | Historical status; do not use merely because a delegable built-in review has not yet been dispatched. |
| superseded | A later durable record replaces this record's recommendation or facts. |

Do not use completed to claim a broader phase/seam than the selected packet proved.

## Finding and escalation behavior

P1-P4 priority determines whether the current subject may close; the
classification below determines the parent's action. See
`09-review-finding-inventory.md`.

| Classification | Parent behavior |
|---|---|
| local_remediation | Fix within current authority, verify, and obtain fresh review. |
| child_packet_required | Create an independently reviewable child and execute it internally; keep the parent slice open. |
| cross_document_repair | Pause behavior-changing implementation, repair coupled authority docs, review them, then resume. |
| resolution_escalation | Stop only when the broader decision cannot be resolved inside current authorization. |
| external_blocker | Stop only when named external/human state prevents further work. |
| proof_gap | Dispatch bounded proof/review work internally and reconcile it. |
| future_program | Record the disposition and continue current authorized work. |

Creating a child packet does not complete its parent. An internal agent cannot promote its own finding into program authority.

## Built-in delegated-run protocol

For every proof-relevant internal run, the parent:

1. creates a schema-valid immutable JSON dispatch;
2. fingerprints the dispatch and subject state;
3. spawns a fresh built-in default subagent with isolated context;
4. supplies the exact bounded packet directly through the built-in spawn message;
5. waits using built-in wait/status operations;
6. closes completed or abandoned agents through built-in capabilities when exposed;
7. records agent ID/canonical task name and final built-in status;
8. validates results/findings against live truth;
9. reconciles edits or evidence into the active slice;
10. continues without asking the user to launch the internal dispatch.

Forbidden reviewer transports include shell-managed agents, codex exec, another Codex CLI, background/PTY agents, temporary-file prompt/output transport, filesystem identities, and filesystem polling.

### Review lineage

Review agents are read-only, fresh, and isolated from implementation reasoning and earlier conclusions. Their dispatch requires code-review-and-quality and binds the exact subject fingerprint.

Findings are ordered Critical, Required, Optional, Nit and include file/line,
violated contract/gate, reason, smallest valid remediation, and missing proof.
`09-review-finding-inventory.md` maps them to P1, P2, P3, and P4.

- A review uses `verdict: findings` when it contains at least one valid P1/P2.
- A review with no valid P1/P2 uses `verdict: clean`; it may retain P3/P4 IDs
  in `finding_refs`.
- Every valid unfixed P3/P4 is added to or deduplicated against `09` during
  true-stop closeout. P3/P4 alone creates no remediation lineage.
- An existing inventory entry never waives a new P1/P2.

When P1/P2 findings are valid:

1. the parent consolidates all same-subject review-burst results;
2. the parent repairs them or spawns a fresh bounded remediation agent;
3. the remediation run names every applicable findings review in
   remediation_for_run_ids;
4. affected verification reruns;
5. a different fresh reviewer receives the new subject fingerprint and may
   focus on the material delta plus affected contracts/call paths/proof;
6. CLEAN proceeds; a P1/P2 directly caused or unmasked by the preceding
   remediation may use one of at most two supplemental causal
   remediation/closure cycles, while an unrelated blocker, material scope/risk
   expansion, or exhausted allowance records a bounded partial/blocked stop.

A bounded review burst may use multiple fresh read-only agents with disjoint
lenses against one identical subject fingerprint. Consolidation happens before
remediation so independently discoverable issues are fixed in one pass.
Unless the selected plan explicitly authorizes otherwise, the review budget is
one such discovery review/burst, one consolidated remediation, and one
different-fresh delta-focused closure review plus at most two supplemental
causal remediation/closure cycles. Each supplemental cycle stays within the
selected scope, authority, and risk ceiling, consolidates causally related
findings, and does not reopen general discovery. Budget exhaustion never
converts a valid P1/P2 into accepted debt.

A completed v1.3 record fails semantic validation when a P1/P2 findings review
lacks typed successful parent/delegated remediation, delegated remediation is
failed/wrong-role, remediation lacks a completed different-fresh re-review of
its result fingerprint, a reviewer is reused after remediation, dispatch/result
lineage mismatches, the final completed review is not clean, or the final clean
review does not bind the replayable reviewed-state manifest/fingerprint. A
closure re-review may identify another P1/P2, but the default budget then
admits another remediation and fresh closure review only when the blocker was
directly caused or unmasked by the preceding remediation and a supplemental
causal cycle remains. Otherwise the record stops non-completed; continuing
requires explicit additional authority and a recorded budget extension.
It also fails when a finding lacks exact source-run linkage, a clean review
carries P1/P2, a `findings` verdict carries no P1/P2, a blocking finding is not
resolved/remediated, or an advisory remains open rather than inventoried or
otherwise disposed. An `inventoried` P3/P4 must use an `HCM-RF-####` ID and
priority that match a durable row in `09-review-finding-inventory.md`; the
status word alone is not registration evidence.

For multi-packet slices, perform bounded packet review as needed and use a
different fresh agent for final slice closeout. A single-packet review may serve
as final closeout review only when its dispatch covers the full final subject
and proof wall.

Only material changes invalidate independent review. A mechanical-only delta is
limited to deterministically proved whitespace/formatting, generated
fingerprint/manifest/ledger bytes, or exact P3/P4 inventory transcription. The
parent records the diff and deterministic checks; any semantic uncertainty
makes it material and requires review.

## Resolution escalation record

Every authority escalation states:

- current and required Resolution/authority horizon;
- trigger and exact missing decision;
- options and tradeoffs;
- recommended option;
- affected pack/spec/task sections;
- whether current work is safe to preserve;
- exact resumption condition.

Needs more context is insufficient.

When authority is reserved to a user/product decision, such as the shipped default artifact set, request that named research/decision boundary. Do not infer approval from current code, historical artifacts, or examples.

## Two-commit true-stop closeout

A handoff cannot contain the hash of the commit that contains itself. Use a deliberate two-commit protocol for a completed top-level slice:

1. Finish implementation/documentation, verification, fresh review, remediation/re-review, proof, and control-pack updates.
2. Run final scoped change detection and diff checks for the reviewed slice state.
3. Commit the reviewed slice state. A multi-packet slice may use a reviewed
   commit stack; its final primary tip represents the aggregate reviewed state.
4. Capture/verify the top-level end state and use the final primary tip as both
   `repo_state.head` and `reviewed_state.baseline_head`; completed validation
   rejects mismatched commit identities.
5. Create one v1.3 parent handoff. Record the final reviewed subject
   fingerprint, delegated runs, structured finding dispositions, stop reason,
   proof refs, and source/supersession truth.
6. Register or deduplicate validated unfixed P3/P4 advisories in
   `09-review-finding-inventory.md`.
7. Rebuild ledger.jsonl from canonical records.
8. Run all handoff/internal-dispatch schemas, historical admission checks, cross-record semantics, ledger parity, and self-tests.
9. Run git diff --check and scoped change detection for the mechanical closeout artifacts.
10. Commit only the new handoff, ledger entry, directly required closeout index
    artifacts, and exact advisory inventory registrations in a second closeout
    commit.
11. Report the reviewed commit/stack and closeout hash in chat. Do not start the next slice.

The final dispatch may include the pre-closeout `ledger.jsonl` because that file
is part of the reviewed primary state. Completed-record validation replays the
manifest from `reviewed_state.baseline_head`, not from the post-closeout working
tree; exact record/index parity independently validates the rebuilt ledger after
the parent record is added.

For a blocked/partial stop, commit only safe reviewed work when appropriate,
then create and commit the handoff separately. Open P1/P2 may also be
registered in `09` as blockers for comparison/resumption, never as accepted
debt. Never label an unreviewed or failed state completed.

## Create a v1.3 parent handoff

1. Copy handoff-template.json to a correctly named records/YYYYMMDDTHHMMSSZ--<phase-or-slice>--orchestration--<slug>.json file.
2. Fill every field; remove placeholders.
3. Use source_handoff_ids for consumed resume records.
4. Use supersedes only when replacing prior recommendations/facts.
5. Record the true stop_reason.
6. Record built-in delegation-capability evidence.
7. Record only proof-relevant delegated runs and exact JSON dispatch refs/fingerprints.
8. Bind the final clean review to reviewed_state.subject_fingerprint.
9. Record unresolved advisory IDs or state that none intersect the closed
   subject.
10. Reference snapshots, semantic records, and long evidence rather than embedding content.
11. Prefer repository-relative refs.
12. Rebuild and validate the ledger.

A jq syntax check is optional and never substitutes for schema/semantic validation.

## Ledger entry creation

For one new record:

~~~bash
record="docs/specs/handbook-contract-membrane/handoffs/records/<record>.json"
jq -c --arg record_path "$record" '{
  schema_id: "handbook.handoff-ledger-entry",
  schema_version: "1.0",
  handoff_id,
  created_at_utc,
  status,
  session_kind: .session.kind,
  phase_id,
  slice_id,
  packet_id,
  record_path: $record_path
}' "$record" >> docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
~~~

Before appending, confirm the ID is not indexed:

~~~bash
jq -e -s --arg id '<handoff-id>' 'any(.[]; .handoff_id == $id)'   docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl >/dev/null
~~~

Exit 1 means absent and appendable. Exit 0 means already present.

## Deterministic ledger rebuild

~~~bash
root="docs/specs/handbook-contract-membrane/handoffs"
tmp="$root/ledger.jsonl.tmp"
: > "$tmp"
find "$root/records" -type f -name '*.json' -print   | LC_ALL=C sort   | while IFS= read -r record; do
      jq -c --arg record_path "$record" '{
        schema_id: "handbook.handoff-ledger-entry",
        schema_version: "1.0",
        handoff_id,
        created_at_utc,
        status,
        session_kind: .session.kind,
        phase_id,
        slice_id,
        packet_id,
        record_path: $record_path
      }' "$record"
    done > "$tmp"
mv "$tmp" "$root/ledger.jsonl"
python3 "$root/validate_handoffs.py"
~~~

## Required validation

~~~bash
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
~~~

The normal command validates:

- all Draft 2020-12 handoff/internal-dispatch schemas and current templates;
- exact immutable v1.0/v1.1 and legacy-dispatch admission;
- all canonical records, the hash-admitted internal-dispatch v1.0 review,
  immutable v1.1 JSON dispatches, and current v1.2 JSON dispatches;
- v1.2/v1.3 source/supersession and delegated-run/dispatch lineage;
- clean-review/finding-priority/disposition/remediation/fresh-re-review
  semantics;
- record/index identity and parity;
- a byte-identical deterministic ledger rebuild.

## Common jq queries

### Latest handoff for the selected slice

~~~bash
slice="HCM-X.Y"
jq -s --arg slice "$slice"   '[.[] | select(.slice_id == $slice)] | sort_by(.created_at_utc) | last'   docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
~~~

### Read the latest full record for the selected slice

~~~bash
slice="HCM-X.Y"
ledger="docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
record="$(jq -rs --arg slice "$slice"   '[.[] | select(.slice_id == $slice)] | sort_by(.created_at_utc) | last | .record_path // empty'   "$ledger")"
test -n "$record" && jq . "$record"
~~~

### Select one exact handoff

~~~bash
id="<handoff-id>"
ledger="docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
record="$(jq -rs --arg id "$id"   'map(select(.handoff_id == $id)) | last | .record_path // empty' "$ledger")"
test -n "$record" && jq . "$record"
~~~

### Latest escalation for the selected slice

~~~bash
slice="HCM-X.Y"
jq -s --arg slice "$slice"   '[.[] | select(.slice_id == $slice and .status == "escalation_required")] |
   sort_by(.created_at_utc) | last'   docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
~~~

## Top-level orchestration protocol

The parent:

1. receives explicit phase, slice, optional packet, and optional handoff selector;
2. validates dependencies, authorization, worktree state, and selected resume truth;
3. assembles bounded authority/repo/proof context;
4. captures or verifies current grounding state;
5. repairs specification/plan authority when needed;
6. performs or internally delegates the selected work;
7. verifies every packet;
8. executes fresh built-in review and waits for results;
9. consolidates same-subject review bursts, remediates P1/P2, inventories
   unfixed P3/P4, and executes different-fresh re-review after material repair;
10. runs the full proof wall and updates canonical pack truth;
11. commits the reviewed slice state or reviewed multi-packet commit stack;
12. writes one v1.3 handoff only at a true stop;
13. commits the mechanical closeout record/index/advisory inventory;
14. returns a short durable closeout.

If selected handoff facts are stale, do not edit the record. Name it in source_handoff_ids and supersedes only when the new record actually replaces its recommendation/facts.

For artifact/intake/posture work, preserve kind/instance separation, intake candidate versus canonical authority, advisory posture recommendations, and fixed generic CLI operations.

## Short chat closeout

After the parent-owned record and closeout commit exist, return:

~~~text
STATUS: <status>
HANDOFF: <repo-relative parent-owned record path>
SUMMARY: <one or two sentences>
NEXT: <human action, exact top-level resume condition, or none>
READ: jq . <repo-relative record path>
COMMITS: <primary-slice-commit> <closeout-commit>
~~~

Include DISPATCH only for top_level_resume or human_interactive. Never return an internal JSON dispatch as a manual user prompt.

## Failure rules

- If mandatory built-in delegation is unavailable, stop blocked with stop_reason=capability_unavailable; do not self-review or launch an external agent.
- If a reviewer is slow, continue bounded built-in waits; slowness is not capability failure.
- If an internal subagent attempts a global handoff/ledger write, reject that result and remediate the dispatch boundary.
- If an immutable historical record/legacy dispatch is missing or byte-modified, fail closed; never repair by rewriting history or its admission hash without explicit historical-integrity authority.
- If a current JSON dispatch or v1.3 record fails schema/semantic lineage, the top-level run is not closed.
- If ledger and records disagree, rebuild the ledger from canonical records.
- If a snapshot is unstable, retry or record the bounded blocker; do not ground promotion on it.
- If projection exposes sensitive/out-of-Resolution state, omit it and record the omission.
- If scope must broaden beyond current authority, stop at an authority boundary rather than completing extra work.
- If a contract contradiction can be repaired inside current authority, repair/review it internally; do not create a user task hop.
- If a known P3/P4 recurs unchanged, update its occurrence evidence rather than
  create a duplicate or force remediation; if new evidence makes it P1/P2,
  treat it as blocking.
- If a supposedly mechanical delta changes behavior, authority, contract
  meaning, proof, API/scope, tests, or user-facing semantics, classify it as
  material and obtain independent review.
- If the record cannot be written, report the complete closeout in chat as a fallback and state that durable closeout failed.
