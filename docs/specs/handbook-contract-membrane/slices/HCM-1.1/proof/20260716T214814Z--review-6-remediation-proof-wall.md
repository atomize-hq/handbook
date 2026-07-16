# HCM-1.1 Review 6 Remediation Proof Wall

**Captured:** 2026-07-16T21:48:14Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation lineage HEAD before remediation:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Review 6 result and disposition

Fresh Final Registry Review 6 used the immutable 40-entry dispatch at
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T213006Z--HCM-1-1--fresh-final-registry-review-6.json`
(dispatch-byte SHA-256
`0a7ca2e5a4a8c09e2c97e5e6249a99c9686fb43f93524302a7ec810d31382378`).
Reviewer `/root/hcm_1_1_final_review_6` replayed every entry and reproduced
`sha256:1a865c92c653ebd8120d325d509268b71abfd345a8b81bc27638456785b31f24`.
The reviewer made no repository edits and returned `CHANGES_REQUIRED` with one
Required finding: the validator-only absolute URI encoded a normalized document
path but appended the already-prewalked RFC 6901 fragment raw, so pointer tokens
with spaces or non-ASCII UTF-8 failed URI parsing during registry preparation.

The two repeated Optional findings remain unchanged and non-blocking: unknown-
kind validation still uses an empty error vector and the bounded reference
graph still performs a quadratic lookup.

The parent accepted and remediated the Required finding. Earlier proof walls
remain immutable lineage; this file supersedes their private-fragment claim.

## RED and impact evidence

The parent added two focused probes before behavior repair:

- live cross-document closures targeting `#/$defs/space field` and
  `#/$defs/café`, including exact closure, authored fingerprint, valid/invalid
  behavior, and decoded structural-location assertions; and
- an exact diagnostic projection from
  `#/$defs/caf%C3%A9%20field/type` back to
  `#/$defs/café field/type`.

Both failed at the reviewer-reported boundary. The 46-line RED stream is
`/tmp/hcm-1.1-review6-red.log`, SHA-256
`b5605d7e74ca391b665012619e9a4357a49a35baf371d9aafb1ce6a749f3ae50`.

GitNexus rated the diagnostic decoder isolated and LOW. The validator-clone
helper was too new for the current index; its sole caller is the already-
audited isolated LOW `build_validator` seam. No HIGH or CRITICAL symbol was
edited for this remediation.

## Bounded remediation

The same injective UTF-8 percent encoder now applies independently to normalized
private resource paths and already-prewalked RFC 6901 fragment bytes. ASCII
unreserved bytes and `/` remain literal; every other byte uses upper-case
percent encoding. Thus JSON Pointer separators and `~0`/`~1` retain their exact
meaning, while spaces, non-ASCII UTF-8, and reserved bytes form valid private
URI fragments without collision.

Diagnostic projection decodes the private path and fragment independently.
Invalid encoding still fails closed to the stable `schema_root` location, and
the existing 512-byte structural ceiling applies after decoding. The live
space and UTF-8 pointer cases load, preserve the exact two-document closure and
authored root fingerprint, accept valid strings, reject numbers, and report the
exact decoded child JSON Pointer without `%` drift.

Only the validator clone changes. Authored source bytes, reference policy,
document/closure/entry/kind fingerprints, and the existing refusal of authored
percent, query, backslash, scheme, authority, and traversal forms are unchanged.

## Complete post-remediation replay

All commands below exited `0` in one fail-fast run. The raw 1,966-line stream is
`/tmp/hcm-1.1-review6-remediation-proof.log`, with SHA-256
`9e91a4a4f45d54ad13f86bddd82885c04f484df97e367b562cc2e49372bf7a82`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 79 tests plus doc tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo check -p handbook-engine --tests --target x86_64-pc-windows-gnu` | PASS; non-Unix implementation and test branches compile |
| `cargo tree -p handbook-engine -e features` | PASS; `rustix 1.1.4` with `fs`; no HTTP/TLS/async/file resolver feature path |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 459.3 KiB uncompressed, 85.4 KiB compressed |
| exact package member checks | PASS; literal set equality for both stable-role assets and all eight custom-kind fixtures |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; current supported roots clean |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 34 records, 107 current dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The package archive contains exactly the required stable-role assets:

- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`
- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`

It also contains exactly the eight literal members under
`handbook-engine-0.1.1/tests/fixtures/hcm_1_1_custom_kind/`.

## Scope and next gate

The remediation is confined to HCM-1.1 validator-only private-fragment
translation/diagnostic decoding, focused schema tests, and packet/proof
clarification. No `CanonicalArtifactKind` variant, product callsite, layout,
setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate, profile, instance,
intake, renderer, lifecycle, Projection, publication, sibling gate, or HCM-1.2+
byte changed.

`PG-KIND-01` and `PG-KIND-02` remain open. The classification ceiling remains
`BoundaryLanded` for the owner-library `Artifact kind/schema registry` seam.

This proof is not approval. A seventh immutable complete-subject dispatch must
bind every current byte, all six prior dispatches, and all seven proof walls. A
seventh different fresh isolated read-only built-in `default` reviewer must
return `CLEAN`. No subject byte may change after that result, and no HCM-1.2
work is authorized.
