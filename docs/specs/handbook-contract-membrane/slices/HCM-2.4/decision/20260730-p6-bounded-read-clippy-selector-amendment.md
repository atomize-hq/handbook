# HCM-2.4 P6 bounded-read Clippy selector amendment

Status: additive amendment pending different-fresh v1.4 supplemental closure

Recorded: 2026-07-30

Parent selector: `20260729-p6-aggregate-flow-cleanup-selector.md`

Active packet: `HCM-2.4 P6 aggregate-flow cleanup`

## Trigger and additive authority

The P6 implementation discovery review returned
`HCM-2.4-P6-BOUNDED-STABLE-READ-001`: descriptor-selected canonical source
ingestion used the private unbounded `TrustedRepoFile::read_bytes` method
instead of the existing bounded stable retained-handle primitive. The parent
remediated the selected loader through `read_bytes_bounded_stable` and proved
oversize and unstable-identity RED/GREEN behavior in the already authorized
canonical-artifacts test path.

That correction leaves the private unbounded method with no production caller.
Strict engine Clippy now fails on `dead_code`. The method still has two unit-test
call expressions in its defining path, so deleting it without replacing those
expressions would weaken retained-handle and unsupported-platform coverage.

The operator authorizes this record as an additive P6 selector amendment. It
does not rewrite the immutable P6 selector or prior dispatches. Every original
P6 ceiling, stop, non-goal, and preservation obligation remains in force except
for the exact one-path allowance below.

## Exact ancillary allowance

The frozen allowance is exactly one additional path and three expressions:

- path: `crates/engine/src/canonical_repo_support.rs`
- production expression: delete only private
  `TrustedRepoFile::read_bytes(&self) -> Result<Vec<u8>, std::io::Error>`
- Unix unit test:
  `trusted_handle_survives_intermediate_and_final_path_substitution`
- Unix expression: replace `trusted.read_bytes().unwrap()` with the byte vector
  returned by `trusted.read_bytes_bounded_stable(1024).unwrap()`
- unsupported-platform unit test:
  `legacy_canonical_reads_remain_available_while_strict_registry_reads_fail_closed`
- unsupported-platform expression: replace `.read_bytes().unwrap()` with the
  byte vector returned by `.read_bytes_bounded(1024).unwrap()`
- classification: stale private unbounded-reader deletion plus proof-preserving
  bounded unit-call replacement
- risk ceiling: LOW
- path ceiling: one
- expression ceiling: one private method deletion and two unit-call replacements

No other function, test, assertion, path, fixture, public API, schema,
definition, profile, renderer, Cargo metadata, dependency, module, or runtime
behavior may change under this amendment. The existing bounded and stable read
implementations are reused unchanged. No lint suppression, dummy caller,
compatibility shim, or dual read is authorized.

## Impact and scope freeze

GitNexus upstream impact for exact UID
`Function:crates/engine/src/canonical_repo_support.rs:TrustedRepoFile.read_bytes#0`
is exact LOW with zero direct callers, zero affected processes, and zero
affected modules. Recursive source inventory finds only the two unit-test call
expressions named above and no production call after bounded selected-ingest
remediation.

The amendment is therefore frozen at one path, three expressions, LOW risk,
and no production behavior or public/API/fixture change. Any additional caller,
path, assertion change, or failure of either bounded replacement to preserve
its current proof is a stop.

## Pre-edit review gate

Before editing `canonical_repo_support.rs`, a fresh isolated built-in default
read-only reviewer must validate a current internal-dispatch v1.4 subject and
return CLEAN. The review must verify:

1. the unbounded method has no production caller and its deletion is the
   smallest honest correction for strict Clippy;
2. the Unix replacement preserves retained-handle path-substitution proof while
   adding the bounded stable observation contract;
3. the unsupported-platform replacement preserves the legacy bounded-read
   assertion while strict registry reads continue to fail closed; and
4. the amendment creates no API, fixture, renderer, policy, P7, or broader
   authority.

## Review finding consolidation and corrected lineage

The first bounded-amendment closure dispatch,
`20260730T140720Z--HCM-2-4--p6-bounded-read-clippy-selector-amendment-review`,
is immutable and abandoned after its reviewer returned two dispatch-only P2s:

- `HCM-2.4-P6-BOUNDED-AMENDMENT-AUTHREF-001`: replace its nonexistent
  `Finding classification and reconciliation` authority label with the live
  `Finding and escalation behavior` heading; and
- `HCM-2.4-P6-BOUNDED-AMENDMENT-LINEAGE-001`: bind actual findings-run
  identities and the complete same-burst P1/P2 set rather than one discovery
  cycle label and one finding.

The parent consolidates the actual findings runs as
`p6_specification_review`, `p6_standards_review`, and
`p6_bounded_read_amendment_review`. The complete unresolved P2 set entering the
replacement pre-edit closure is:

- `HCM-2.4-P6-BOUNDED-STABLE-READ-001`;
- `HCM-2.4-P6-FIXED-KIND-AUTHORITY-001`;
- `HCM-2.4-P6-INGEST-REFUSAL-001`;
- `HCM-2.4-P6-REVIEW-AUTHREF-001`;
- `HCM-2.4-P6-BOUNDED-AMENDMENT-AUTHREF-001`; and
- `HCM-2.4-P6-BOUNDED-AMENDMENT-LINEAGE-001`.

The replacement review must use the first proof-stage `supplemental_causal`
cycle because the immutable failed dispatch already consumed the stage's
closure slot. Its typed causal arrays name only the immediately preceding
`p6_bounded_read_amendment_review` run and its two P2 IDs, as v1.4 requires.
Its review scope must additionally replay the three actual findings runs and
complete six-P2 remediation set above, cite only live authority headings, and
validate the one-path amendment without claiming final implementation
acceptance. A later different-fresh final implementation review remains
mandatory after the reviewed edit and complete P6 convergence wall.

## Required proof and integration

After the reviewed edit:

- the two affected unit tests must pass on their supported platforms;
- recursive source inventory must report no `TrustedRepoFile::read_bytes` or
  `.read_bytes()` caller in `canonical_repo_support.rs`;
- strict engine Clippy across all targets and features must pass with
  `-D warnings`;
- bounded selected-ingest oversize and unstable-identity tests must remain
  GREEN;
- the existing P6 convergence, complete verification, different-fresh closure,
  control-pack, commit, and true-stop handoff sequence resumes; and
- this path must join the final P6 implementation subject and review.

This is an in-loop P6 proof and lint closure. It does not select P7 or authorize
Phase 2, HCM-3.x, task-gate runtime, automatic continuation, push, or release.
