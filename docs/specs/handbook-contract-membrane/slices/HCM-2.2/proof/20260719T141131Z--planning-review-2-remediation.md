# HCM-2.2 Planning Review 2 Remediation

## Review identity

- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T134309Z--HCM-2-2--fresh-planning-review-2.json`
- Raw dispatch SHA-256:
  `ee54f8455d6f0f9704b443b363e1b577451afb98aae559b0cdbb7926e5c85108`
- Reviewed subject fingerprint:
  `sha256:6e3cdeb2699c743089fda9f268c948742e40ccc42d30940a827b6f2bb3a9ac66`
- Fresh reviewer: `/root/hcm_2_2_planning_review_2`
- Built-in status: completed read-only review
- Verdict: `CHANGES_REQUIRED`
- Findings: eight Required; no Critical, Optional, or Nit findings

Review 2 reproduced all eleven entries, the aggregate fingerprint, validators,
links, fixture bytes, and planning-only scope without editing the repository.
Its verdict authorized neither implementation nor commit.

## Parent validation and remediation

### R1 — non-uniform definition fingerprint producer

**Disposition:** valid.

The custom field-order JSON plus dependency trailer contradicted `05`. The
packet now requires the one uniform producer: record-without-own-fingerprint
plus typed dependency closure entries as one JSON value, sorted per contract and
serialized once with RFC 8785 JCS. Type-specific rules may add typed closure
entries only. Missing producer, cycle, duplicate identity, substitution, and
independent-encoder proof are explicit.

### R2 — missing exact refs and validator compatibility

**Disposition:** valid.

Every subordinate record now carries exact capability-contract, schema,
validator, approval-policy, trigger, or evidence-schema refs rather than stable
IDs alone. The new kind retains capability-required semantic validator `1.0.0`
and adds compatible schema-specific validator `1.1.0`. A new exact trigger-
evidence schema and exact Charter-amendment review trigger are in the closure.

### R3 — overlapping mixed-authority coverage

**Disposition:** valid.

The broad `/project` plus `/posture` row is replaced by seven project-shape leaf
paths and a new normative `engineering_posture.baseline` row. The intake now has
sixteen rows. Meta-validation requires pairwise path non-overlap, one authority/
source rule per row, and a bijection between every populated semantic leaf and
one field source. Constants/generation are explicit deterministic derivations.

### R4 — optional amendment compare-and-write basis

**Disposition:** valid.

Expected-current null/omission is now create-only. Every amendment requires a
non-null exact fingerprint before lock/mutation; the same value binds
validation, approval checking, intent, promotion record, retry, and recovery.
Omission, null, mismatch, stale replacement, or ABA refuses.

### R5 — canonical key-order contradiction

**Disposition:** valid.

The packet now contains explicit ordered key lists for every object, matching
the authoritative 5,405-byte YAML fixture, rather than inferring order from
table prose. Independent emitter proof binds the lists and fixture together.

### R6 — incomplete lifecycle machinery

**Disposition:** valid.

The packet now freezes one exact candidate-amendment review trigger, both exact
reassessment triggers/evidence shape, a total state/event/precedence table,
duplicate/stale/conflict behavior, immutable observation and transition field
sets, state fingerprints, and promotion-based clearance validation. Promotion
installs the lifecycle transition in the same journal without cyclic record
fingerprints.

### R7 — waiver as missing normative authority

**Disposition:** valid.

Charter waivers now apply only to `minimum_specificity` after a structurally and
semantically valid user-declared value exists. The value retains
`source_kind: user_declaration`; `source_kind: waiver` refuses. Requiredness,
declaration, source, structure, semantics, unknowns, contradictions, approval,
and authority cannot be waived. Waiver records bind the value/candidate and
require explicit approval acceptance.

### R8 — disabled decision-record validation drift

**Disposition:** valid.

The schema now preserves the live disabled case, including empty path/format,
and ignores those values in the disabled renderer branch. Enabled path/format
remain concrete and required. The preservation crosswalk and proof explicitly
cover both cases.

## Re-review requirement

Review 2 remains immutable findings evidence. The parent must replay the full
planning wall, bind every remediated subject byte plus prior review evidence to
a new manifest, and use a third different fresh isolated reviewer. No prior
clean conclusion is available or inherited.
