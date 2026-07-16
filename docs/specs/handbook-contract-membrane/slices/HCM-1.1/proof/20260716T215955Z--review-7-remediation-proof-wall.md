# HCM-1.1 Review 7 Remediation Proof Wall

**Captured:** 2026-07-16T21:59:55Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation lineage HEAD before remediation:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Review 7 result and disposition

Fresh Final Registry Review 7 used the immutable 42-entry dispatch at
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T214858Z--HCM-1-1--fresh-final-registry-review-7.json`
(dispatch-byte SHA-256
`fe2ba186e0a6bfeb79a49d4cf5ede5eb9916d20c87225a44338e622e0a14be42`).
Reviewer `/root/hcm_1_1_final_review_7` replayed every entry and reproduced
`sha256:81f45a591a64551de7fc7dfd74daa26d5575cd36eadd89ed6b83060a7e23cd4f`.
The reviewer made no repository edits and returned `CHANGES_REQUIRED` with one
Required finding: fragment-only same-document refs were prewalked and validated
but skipped by validator-clone translation, so pointer tokens with spaces or
non-ASCII UTF-8 still reached validator construction as raw URI fragments.

The two repeated Optional findings remain unchanged and non-blocking: unknown-
kind validation still uses an empty error vector and the bounded reference
graph still performs a quadratic lookup.

The parent accepted and remediated the Required finding. Earlier proof walls
remain immutable lineage; this file supersedes their every-ref translation
claim.

## RED and impact evidence

The parent added one focused two-case probe before behavior repair. One-file
schemas target `#/$defs/space field` and `#/$defs/café`; each case asserts the
exact one-document closure, authored document fingerprint, valid/invalid
behavior, and decoded structural pointer.

The probe failed at validator construction while the prewalk/fingerprint steps
remained valid. The 21-line RED stream is
`/tmp/hcm-1.1-review7-red.log`, SHA-256
`e0a500452fd6e6bcddd45a3f6f2157472480dffa44a659fc99a50738b54086a8`.

The validator-clone helper remains a newly added isolated function whose sole
caller is the audited LOW `build_validator` seam. No HIGH or CRITICAL symbol
was edited for this remediation.

## Bounded remediation

Validator translation now processes every prewalked `$ref`. A cross-document
ref begins with the exact target document private URI. A fragment-only ref
begins with the current document's exact private URI. Both then append the same
independently encoded RFC 6901 fragment. The prewalk remains the authority for
source and target pointers, so translation cannot introduce an ambient target.

Both same-document space and UTF-8 pointer cases now load, preserve the exact
one-document closure and authored root fingerprint, accept strings, reject
numbers, and report the exact decoded JSON Pointer without percent drift. The
cross-document equivalents remain green. Authored source bytes, fingerprints,
reference policy, and authored-percent refusal are unchanged.

## Complete post-remediation replay

All commands below exited `0` in one fail-fast run. The raw 1,968-line stream is
`/tmp/hcm-1.1-review7-remediation-proof.log`, with SHA-256
`fbc899a06de187eebc6c4eaced1855ae00c8a7c20094613e1f58146f8bebabe5`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 80 tests plus doc tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo check -p handbook-engine --tests --target x86_64-pc-windows-gnu` | PASS; non-Unix implementation and test branches compile |
| `cargo tree -p handbook-engine -e features` | PASS; `rustix 1.1.4` with `fs`; no HTTP/TLS/async/file resolver feature path |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 461.3 KiB uncompressed, 85.5 KiB compressed |
| exact package member checks | PASS; literal set equality for both stable-role assets and all eight custom-kind fixtures |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; current supported roots clean |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 34 records, 108 current dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The package archive contains exactly the required stable-role assets and all
eight literal custom-kind fixture members.

## Scope and next gate

The remediation is confined to HCM-1.1 validator-only same-document reference
translation, one focused schema test, and packet/proof clarification. No
`CanonicalArtifactKind` variant, product callsite, layout, setup, doctor, flow,
compiler, CLI, SDK, Tauri, Substrate, profile, instance, intake, renderer,
lifecycle, Projection, publication, sibling gate, or HCM-1.2+ byte changed.

`PG-KIND-01` and `PG-KIND-02` remain open. The classification ceiling remains
`BoundaryLanded` for the owner-library `Artifact kind/schema registry` seam.

This proof is not approval. An eighth immutable complete-subject dispatch must
bind every current byte, all seven prior dispatches, and all eight proof walls.
An eighth different fresh isolated read-only built-in `default` reviewer must
return `CLEAN`. No subject byte may change after that result, and no HCM-1.2
work is authorized.
