# HCM-2.1 Planning Review 1 Remediation

## Review evidence

- Reviewer: `/root/hcm_2_1_planning_review_1`
- Built-in type: `default`
- Fresh/isolated/read-only: yes
- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260718T204425Z--HCM-2-1--fresh-planning-review-1.json`
- Reviewed subject:
  `sha256:f40f9281465b387a21ec3355144b39de6c8cdb3f893a620a7203ab23262da4e1`
- Verdict: `CHANGES_REQUIRED`
- Findings: six Required; no Critical, Optional, or Nit finding
- Review validation: manifest replay, diff, unchecked-task, absolute-path,
  relative-link, handoff-validator, and both validator self-tests passed; HEAD
  stayed at the planning entry and the reviewer made no mutation.

The parent accepted all six findings after rereading the cited live source. No
Rust, runtime, template, fixture, test, generated skill, or product file was
changed during remediation.

## Finding dispositions

### R1 — Environment Inventory legacy Project Context dependency

**Validated.** Live Environment Inventory preflight, core input validation,
renderer constants, template, runtime-smoke fixture, and all-three smoke require
or emit `.handbook/project_context/PROJECT_CONTEXT.md`. Removing that file while
forbidding the reference consumer change would leave a dangling dependency.

**Remediation.** The packet now authorizes one enumerated reference-only cutover
to the exact selected `.handbook/project/context.yaml` decision. It freezes
positive/refusal/no-mutation tests, template/fixture assertions, and all-three
smoke proof while preserving Environment Inventory's fixed Markdown authority,
schema, timestamp, headings, write policy, and flow source.

### R2 — Doctor combined-observation and reason gap

**Validated.** Live profile inspection discards source bytes after structural
validation, and a second typed load could combine observations. The current
status/reason set cannot explain typed-decode, render, or substitution failure
while forcing overall invalid readiness.

**Remediation.** The packet now requires one engine-private retained Project
Context observation through structure, typed roundtrip, render, fingerprints,
and a final Unix device/inode plus byte stability check. It adds exact
`typed_decode_failed`, `rendered_view_refused`, and
`observation_changed_during_inspection` reasons, fixed precedence, row/status
mapping, different-byte substitution proof, and identical-byte inode ABA proof.
Doctor projects the retained observation and never invokes a second load.

### R3 — Fingerprint-significant serializer and renderer ambiguity

**Validated.** The admitted schema permits YAML-sensitive and multiline strings;
general serializer and plain-Markdown prose allowed multiple valid byte results.
The prose registry call also passed an untyped string to a live API that requires
`&SymbolicId`.

**Remediation.** The packet now defines a closed eight-key YAML emitter using
RFC 8259 string escaping and manual mapping/list framing, plus literal boundary
golden bytes. It defines an exhaustive Markdown whitespace/control/punctuation
transform and matching golden bytes. Registry validation uses the selected
decision's typed `instance_id()`.

### R4 — Flow DTO, budget, freshness, and log ambiguity

**Validated.** Live source summaries expose one fingerprint, sections expose no
fingerprint, paths are static strings, and budget/freshness/log behavior is
bound to legacy source bytes. The earlier packet did not decide where rendered
identity or effective bytes belonged.

**Remediation.** The packet now freezes exact owned path changes, nullable source
and rendered fields, `Rendered` section mode, source-only manifest/freshness
identity, rendered-byte budget accounting for Project Context, source-byte
accounting for siblings, `BudgetByteDomain`, target/order/note semantics, an
exact extra decision-log line, and compiler shared/JSON plus CLI projections.
It requires boundary threshold and byte-golden tests.

### R5 — Cross-platform atomic/no-follow gap

**Validated.** The retained compiler writer is descriptor-relative and atomic
only on Unix; its non-Unix fallback is path-based. Engine strict reads already
refuse non-Unix. Authorizing the fallback would contradict the no-follow and
atomic guarantees.

**Remediation.** The packet chooses typed non-Unix fail-before-mutation behavior.
Validate-only remains platform-independent; authoring refuses before lock or
filesystem delta and never calls the fallback writer; doctor and flow preserve
the strict-read refusal. Cross-compilation and native Windows runtime proof are
both mandatory, with unavailable runtime proof an honest stop.

### R6 — Premature review-success claims

**Validated.** The first-review subject used `review-clean` wording before a
review result existed, and the result was not clean.

**Remediation.** `00`, `04`, `06`, and the planning proof now use outcome-neutral
contingent wording. Implementation authority requires an immutable clean review,
recorded parent closeout, and separate top-level implementation selection.

## Re-review gate

The immutable Review 1 dispatch is retained unchanged as historical evidence.
The parent must replay the complete planning validation wall, compute a new
manifest that includes the repaired packet, Review 1 dispatch, and this
remediation proof, write a new immutable dispatch, and use a different fresh
isolated read-only built-in `default` reviewer. No primary commit is permitted
unless that complete subject returns `CLEAN`.

## Post-remediation replay

The parent reran the complete planning validation wall after the last contract
edit:

```text
git diff --check
PASS

unchecked future-task scan under slices/HCM-2.1
PASS: no checked implementation item

exact docs-only changed-path scope
PASS: 00, 04, 06, HCM-2.1 packet/proof, and immutable review dispatch only

absolute machine-path scan
PASS

relative Markdown link checker
PASS: 8 Markdown files, 19 relative links

validate_handoffs.py
PASS: 42 records, 165 current dispatches, 8 admitted legacy dispatches,
42 ledger entries

validate_handoffs.py --self-test-v1-admission
PASS

validate_handoffs.py --self-test-orchestration-contract
PASS

cargo test -p handbook-engine --test author_core
PASS: 9 passed

cargo test -p handbook-compiler --test author
PASS: 47 passed

cargo test -p handbook-compiler --test setup
PASS: 11 passed

cargo test -p handbook-compiler --test doctor
PASS: 3 passed

cargo test -p handbook-flow --test resolver_core
PASS: 15 passed

cargo test -p handbook-cli --test author_cli
PASS: 22 passed
```

These tests replay current pre-HCM-2.1 behavior as baseline evidence only. They
do not claim that the future implementation contract has landed.
