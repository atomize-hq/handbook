# HCM-2.2 validation-result exact-document anchor options

**Status:** non-authoritative decision note. Review 3 rejected the implementation;
HCM-2.2 and HCM-2.3 remain stopped pending a separately authorized,
fresh-review-clean contract repair.

## Finding

Review 3 proved that the result, witness, and binding can be rewritten as one
coherent mutable set. Changing only `validated_at_utc`, copying the changed bytes
to result and witness, and recomputing the binding made author replay accept the
forgery ([Review 3 stop and RED reproduction](../proof/20260721T053136Z--atomic-stage-implementation-review-3-stop.md#red-reproduction)).
The exact rejected Rust source remains only in preserved worktree
`C:\hcm22ar\crates\engine\tests\hcm_2_2_authority_repair.rs` and the external
complete archive, with SHA-256
`a412165c1be6866fdf24f64abc3f35bcfd79fd446e53072ba527b2334ff846d3`;
it is historical evidence, not a subject-local implementation input.

That is permitted by the current identity graph: candidate `1.2` carries one
semantic result ref, while result `1.0` excludes `validated_at_utc` from its
fingerprint ([SPEC, lines 85-149](../SPEC.md#L85-L149)). The closed result schema
contains the timestamp and semantic fingerprint but no raw-document digest
([schema, lines 7-40 and 219-228](../contracts/lifecycle-validation-result-1.0.0.schema.json#L7-L40)).
The Rust candidate has only `validation_result_refs`
(`crates/engine/src/charter_intake.rs:156-195`), and the result preimage removes
ID, fingerprint, and timestamp
(`crates/engine/src/charter_lifecycle_validation.rs:1186-1197`). A sidecar whose
identity is recomputed from the same mutable set supplies no independent anchor.

## Options

### 1. Candidate `1.3` downstream raw-document binding — recommended

Add one closed validation binding to final candidate `1.3`: exact result ref,
semantic fingerprint, `result_document_sha256`, and `result_byte_length`. Include
the complete binding in the final candidate fingerprint; exclude it from the
unchanged candidate-subject fingerprint. The graph stays acyclic:

```text
candidate subject -> semantic result -> exact result bytes/hash
                  -> final candidate 1.3 -> approval -> promotion
```

The candidate is downstream independent authority for which audit-byte variant
was selected. Promotion must verify exact JCS+LF bytes, SHA-256, length,
semantic ref/fingerprint, currentness, candidate fingerprint, and approval
before mutation. Intent `1.2` need not widen because it already binds the final
candidate fingerprint; recovery must reload and verify candidate `1.3` and its
exact result binding.

Author replay must perform bounded, no-follow discovery under the author lock:

- scan exact candidate `1.3` records and select by the pair of independently
  recomputed candidate-subject fingerprint and semantic result ref/fingerprint,
  so historical validations against a different lifecycle state are not
  mistaken for competing replay authority;
- zero matches plus no semantic result is first authoring;
- exactly one match is replay only when its result binding and retained result
  bytes are exact-equal;
- more than one match, an unsafe entry, a missing result, or any byte/binding
  disagreement refuses and preserves all evidence.

A result with no matching candidate `1.3` is an orphan, not authority. Never
adopt, overwrite, or delete it. Minimal repair may refuse and require separately
authorized forensic cleanup. If automatic crash completion is required, the
same repair must add a closed author-publication journal or candidate-bound raw
document object; it must not silently promote an orphan into authority.

### 2. Result self-binding — reject

A document hash included in the document must exclude itself from its preimage.
An attacker can then change the timestamp and recompute that field. Including
the field in its own hash is circular. Self-binding therefore does not provide
the independent expected bytes required by Review 3.

### 3. Make `validated_at_utc` semantic identity — not preferred

This would make timestamp changes alter the result ref, allowing the candidate
to select one exact record. It also turns wall-clock audit metadata into
constitutional identity, makes time allocation precede identity, and forces
bounded result/candidate discovery for replay. It contradicts the deliberately
audit-only rule and is a larger semantic migration than a downstream byte hash.

### 4. Add another sidecar — reject unless candidate-selected

The current witness/binding implementation at
`crates/engine/src/charter_lifecycle_validation.rs:466-602` demonstrates the
problem: mutually consistent companions can be rewritten together. A raw-SHA
content-addressed object can authenticate its own bytes, but cannot select which
semantic-equal audit variant is authoritative. It is useful only when candidate
`1.3` selects its digest/ref, at which point candidate `1.3` remains the anchor.

## Version, migration, and repair surface

- Add candidate `1.3`; keep result `1.0` byte/schema identity unchanged. Treat
  candidate `1.2`, its approvals, and current sidecars as historical
  non-authoritative evidence. Require re-authoring and new approvals; no dual
  read, auto-upgrade, or approval carry-forward.
- Repair `slices/HCM-2.2/SPEC.md`, `05-contracts-schemas-and-gates.md`,
  `tasks/plan.md`, and `tasks/todo.md`; add/update candidate/result exact-byte,
  cross-record, negative, replay/cardinality, orphan, and migration vectors in
  `contracts/authority-repair-runtime-vectors-v1.0.json` and
  `contracts/runtime-record-fingerprint-vectors-v1.0.json`.
- Revalidate the unchanged result schema and promotion-intent `1.2` schema/
  vectors. Update mutable `00`, `03`, `04`, and `06` status surfaces only as
  needed to keep HCM-2.2 stopped. Add new decision/review/handoff records; never
  rewrite immutable Review 3 evidence or earlier handoffs.
- Require RED tests for coherent result/witness/binding rewrite, forged second
  candidate, zero/one/many discovery, missing result, orphan result, unsafe
  entries, exact replay, promotion refusal, and evidence preservation.

## Preserve and resume the dirty implementation

Do not reset, amend, clean, or commit the rejected dirty subject. Inventory and
preserve its exact patch/untracked files, and perform the authority repair in a
separate clean worktree/branch from the recorded clean baseline. After a fresh
planning review returns `CLEAN`, selectively reapply the dirty implementation
hunks against candidate `1.3`; remove the unauthoritative witness/binding design
rather than treating it as approved. Refresh GitNexus impact before every symbol
edit, replay the complete proof wall, and require a new complete-subject fresh
implementation review. HCM-2.3 remains unauthorized.
