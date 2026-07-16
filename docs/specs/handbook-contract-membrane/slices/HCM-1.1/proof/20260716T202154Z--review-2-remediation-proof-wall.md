# HCM-1.1 Review 2 Remediation Proof Wall

**Captured:** 2026-07-16T20:21:54Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation lineage HEAD before remediation:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Review 2 result and parent disposition

Fresh Final Registry Review 2 used the immutable 31-entry dispatch at
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T200406Z--HCM-1-1--fresh-final-registry-review-2.json`
(dispatch-byte SHA-256
`3e2322b4d73c74079571c9c6c072339e616f516292e46a4b675c13893cbe4274`).
Reviewer `/root/hcm_1_1_final_review_2` replayed the complete manifest twice and
reproduced
`sha256:807c66f2bb42f87d32649754569c364fe0deb7b380737660688ac1028cd20977`.
The reviewer made no repository edits and returned `CHANGES_REQUIRED` with
three Required findings:

1. Draft 2020-12 meta-validation embedded upstream display text and could leak
   a large schema value;
2. the custom-kind proof did not literally compare both closure fingerprints,
   both closure member sets, and companion-kind validation; and
3. the claimed negative proof still lacked filesystem-loaded per-source limit,
   schema fingerprint-drift, media-type/compatibility, outside-root,
   invalid-meta-shape, and nested/absolute `$id` cases.

The two Optional findings—an empty error vector for unknown kind lookup and a
quadratic reference lookup—do not block this slice and were not used to widen
the accepted remediation. Both remain unchanged.

The parent validated and accepted all three Required findings. The prior
`20260716T200245Z--review-1-remediation-proof-wall.md` remains immutable review
lineage but its custom-kind and complete-negative-proof claims are superseded
by this literal correction.

## RED evidence and bounded behavior repair

The meta-validation sentinel test failed before repair because
`RegistryLoadError.detail` contained `SECRET_META`. Its 35-line output is
`/tmp/hcm-1.1-review2-red.log`, SHA-256
`a02afda7e92cc6cf0a9e978497e83dd454086ae6109f801b538e75519c8df30d`.

After replacing only that upstream display with a stable category, the
validator-construction sentinel test failed because the invalid regex error
contained `SECRET_PATTERN`. Its 22-line output is
`/tmp/hcm-1.1-review2-validator-red.log`, SHA-256
`79149f563b255afe02258589fa7578bc6b8bde7c7ad334c83ee5accbf4e631a2`.

The final behavior repair discards upstream schema display strings at
meta-validation, in-memory resource registration/preparation, and validator
construction boundaries. Errors retain typed discriminants, stable
repo-relative locations where applicable, and short static categories. The
focused sentinel test now proves neither a 100,000-byte invalid meta value nor
a 100,000-byte invalid regex appears in `detail` or `Display`; each detail is
under 256 bytes.

GitNexus was refreshed before the edit. Upstream impact for
`validate_schema_meta` was LOW: one direct caller (`load_document`) in one
process family. `build_validator` and every edited focused test had zero
upstream dependents and LOW risk.

## Literal proof correction

The checked-in focused tests now assert, rather than infer:

- primary forward/reverse closure member equality and the exact two-member
  primary closure;
- companion forward/reverse closure member equality and the exact one-member
  companion closure;
- primary and companion `closure_fingerprint` equality across permutations;
- primary and companion entry/kind fingerprint equality across permutations;
- valid and invalid structural validation behavior for both kinds, including
  deterministic primary and companion locations;
- a filesystem-loaded schema source of 1 MiB plus one byte returns typed
  `SourceLimitExceeded` before parsing;
- changed document, closure, and entry fingerprints each return typed
  `FingerprintMismatch`;
- unsupported media type and compatibility return their distinct typed
  failures;
- a selected document outside the explicit root returns
  `LocalReferenceOutsideRoot`;
- an invalid Draft 2020-12 meta-shape returns `UnsupportedDialect` without
  disclosing source content; and
- relative, absolute, and nested authored `$id` inputs return
  `UnsupportedSchemaIdentifier`.

The existing Review 1 corrections remain covered: 32/33 reference edges,
nested and mixed cycles/depth, 129-document refusal, aggregate-loader refusal,
identifier/dynamic/content keyword refusal, schema-position versus data
traversal, conflicting identities in both orders, retained descriptor-relative
no-follow access, and bounded sentinel reads.

## Complete post-remediation replay

All commands below exited `0` in one fail-fast run. The raw 1,964-line stream is
`/tmp/hcm-1.1-review2-remediation-proof.log`, with SHA-256
`c18fdbc161fdc1ff996110e9a96916c6fc0d6bcef160889c29f2db67b697ae8f`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 71 tests plus doc tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo tree -p handbook-engine -e features` | PASS; `rustix 1.1.4` with `fs`; no HTTP/TLS/async/file resolver feature path |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 433.5 KiB uncompressed, 80.9 KiB compressed |
| exact package member checks | PASS; both stable-role assets and all custom-kind fixture members present |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; current supported roots clean |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 34 records, 103 current dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The package members include the exact stable-role assets:

- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`
- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`

## Scope and next gate

The repair changes only the HCM-1.1 schema error boundary, focused schema and
custom-kind tests, and this proof evidence. It leaves the reviewer's two
Optional findings unchanged and does not touch product paths, the fixed
`CanonicalArtifactKind`, layout, setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, profile, instance, intake, renderer, lifecycle, Projection,
publication, sibling gates, or HCM-1.2+ packet bytes.

`PG-KIND-01` and `PG-KIND-02` remain open. The intended classification remains
only `BoundaryLanded` for the owner-library `Artifact kind/schema registry`
seam.

This proof is remediation evidence, not approval. A third immutable
complete-subject dispatch must bind every current byte, including both prior
dispatches and all three proof walls. A third different fresh isolated
read-only built-in `default` reviewer must return `CLEAN`. No subject byte may
change after that result, and no HCM-1.2 work is authorized.
