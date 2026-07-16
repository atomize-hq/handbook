# HCM-1.1 Review 1 Remediation Proof Wall

**Captured:** 2026-07-16T20:02:45Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation lineage HEAD before remediation:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Review 1 result and parent disposition

Fresh Final Registry Review 1 used the immutable 28-entry dispatch at
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T193424Z--HCM-1-1--fresh-final-registry-review-1.json`.
Reviewer `/root/hcm_1_1_final_review_1` replayed every entry and the exact
aggregate
`sha256:021e7f5090079a8ecd2c69be69d9a66102336da6b28896f243358c8d6bedffac`,
made no repository edit, and returned `CHANGES_REQUIRED` with five Required
findings:

1. same-document and mixed `$ref` depth/cycle guards were incomplete;
2. path-wide metadata followed by a final no-follow open admitted a component-
   swap race;
3. source-size limits were checked only after an unbounded read;
4. invalid-path and refused-reference errors disclosed absolute paths or source
   content; and
5. the initial proof overclaimed exact cases not literally exercised.

The parent validated and accepted every finding. Remediation remained inside
HCM-1.1 and did not alter any product path or sibling packet. The earlier
`20260716T193058Z--pre-final-review-proof-wall.md` is retained as immutable
pre-review lineage but is explicitly superseded for the five affected claims.

## RED evidence

The new tests were run against the pre-remediation behavior before fixes. The
raw 75-line RED stream is `/tmp/hcm-1.1-review1-red.log`, with SHA-256
`9d4dd8f2a56b4d87ddf1dae32fb39119847d078c69945bf7b46238a842c222a9`.
It records:

- the trusted source race test reading substituted bytes rather than the bytes
  behind the retained trust decision;
- an absolute-path sentinel appearing in a registry error;
- 33 same-document reference edges being admitted; and
- the first document-limit construction reaching the depth guard before the
  intended count guard, after which the test fixture was corrected to use 128
  sibling references and exercise the document-count boundary directly.

The remaining literal cases were also added before final replay. The proof is
the checked-in tests plus this captured RED stream; no claim is made that the
machine-local `/tmp` log is a durable repository artifact.

## Bounded remediation

### Unified reference graph

The schema loader now inserts validated documents before recursively
discovering targets, records explicit schema-containment edges and every local
`$ref` edge, and performs one global depth/cycle traversal over document,
fragment, and mixed references. Every `$ref` edge consumes the same 32-edge
budget. The focused suite proves exactly 32 same-document edges admit, 33
refuse, descendant recursion refuses, and a mixed cross-document/fragment chain
over the boundary refuses.

### Descriptor-relative retained handles

On Unix, trusted source traversal now uses `rustix` descriptor-relative
`openat` operations with `NOFOLLOW` on every component, `DIRECTORY` on
intermediate components, and a retained final regular-file handle. The
deterministic substitution test replaces the opened intermediate pathname and
then the final pathname; the retained handle still returns only the originally
opened bytes. Non-Unix builds fail closed rather than using an unsafe fallback.

This is the smallest dependency repair for Review 1: target-Unix
`rustix = 1.1.4` with the `fs` feature replaces the engine's previous direct
`libc` dependency. `rustix` was already present in the lockfile through the
existing development graph; the engine now uses its safe filesystem API.

### Bounded reads and redacted errors

Trusted reads consume at most the smaller of the 1 MiB document allowance and
the remaining 8 MiB aggregate allowance, plus one sentinel byte. Excess bytes
are rejected before they can be retained as an unbounded input. Loader-level
oversized-document and aggregate-source tests exercise the filesystem path.

Invalid input paths now use stable redacted locations. Remote/ambient refs and
pointer failures report bounded refusal categories without copying raw refs or
fragments. Sentinel tests prove that an absolute path, query secret, and large
fragment secret appear in neither typed location nor display output.

### Literal proof completion

The focused suite now additionally proves:

- a 129-document closure refuses the 128-document bound;
- aggregate bytes are refused in loader execution;
- conflicting schema-entry identities refuse in both input orders;
- `$dynamicAnchor`, `$recursiveAnchor`, `$recursiveRef`, `$vocabulary`,
  `contentEncoding`, `contentMediaType`, and `contentSchema` refuse;
- object-valued data under `const`, `enum`, `default`, and `examples` remains
  data rather than schema-keyword input; and
- both custom schema entries have equal definition and closure fingerprints
  across accepted source permutations, and both custom kinds have equal
  definition fingerprints and validation behavior across those permutations.

## Complete post-remediation proof replay

All commands below exited `0` in one fail-fast run. The raw 1,965-line stream is
`/tmp/hcm-1.1-review1-remediation-proof.log`, with SHA-256
`96e40df5f360147ec3cdf3219764cc892c65830351f942b381894a065b725447`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 71 tests plus doc tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo tree -p handbook-engine -e features` | PASS; `rustix 1.1.4` uses `fs`; no `reqwest`, `hyper`, `rustls`, `tokio`, HTTP/file/async resolver feature path |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 426.6 KiB uncompressed, 80.0 KiB compressed |
| package member equality checks | PASS; both exact stable-role assets and the complete custom-kind fixture corpus are members |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; current supported runtime roots clean |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 34 records, 102 current dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The packaged stable-role members are exactly:

- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`
- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`

## Scope and next gate

The remediation changes only the HCM-1.1 engine owner modules, direct
dependency declaration/lock edge, focused tests, and HCM-1.1 control/proof
evidence. It preserves the fixed `CanonicalArtifactKind`, canonical layout,
setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate, profiles, instances,
intake, rendering, lifecycle, Projection, publication, sibling gates, and all
HCM-1.2+ packet bytes.

`PG-KIND-01` and `PG-KIND-02` remain open. The only intended seam promotion is
the owner-library `Artifact kind/schema registry` boundary to
`BoundaryLanded`.

This proof records remediation, not approval. A new immutable complete-subject
dispatch must bind every current subject byte. A different fresh isolated
read-only built-in `default` reviewer must return `CLEAN`; the subject must then
remain byte-identical through replay and primary commit. No HCM-1.2 work is
authorized.
