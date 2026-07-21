# HCM-2.2 exact-result implementation Review 3 remediation

**Proof ID:** `HCM-2.2-ERIR-R3-REMEDIATION`  
**Recorded:** `2026-07-21T20:57:54Z`  
**Authority class:** additive implementation remediation proof; no clean or
landing claim  
**Reviewed subject:**
[`20260721T190239Z--HCM-2-2--fresh-exact-result-implementation-review-3.json`](../../../handoffs/dispatches/20260721T190239Z--HCM-2-2--fresh-exact-result-implementation-review-3.json)

## Disposition

Fresh isolated implementation Review 3 admitted exact 48-path subject
`sha256:297f71b491bc499d662d4b3c4c6d500e58c21544b819fee73ab214e0286361d1`
and returned `CHANGES_REQUIRED` with one Required finding, `R3-001`.
It is accepted without waiver. All three review dispatches and the earlier
proof files remain immutable evidence.

Candidate `1.3` authenticated its result document but did not authenticate the
semantic provenance asserted by `field_sources`. An attacker could replace the
complete source map, recompute candidate subject identity, recompute the
semantic result and exact-document binding, then issue a new approval and
promotion. Structural source-map checks admitted that coherently resigned
chain because none of author replay, approval, promotion, or recovery
recomputed the exact populated-leaf provenance from the retained intake and
normalized Charter content.

## Test-first repair

The coherent-resigning regression was preserved RED at all three public
workflow seams before production repair. Eight variants covered empty,
missing, reordered, duplicated, unsafe-path, wrong-coverage, unsupported-source
kind, and source-free populated-leaf maps. Author replay incorrectly succeeded;
approval reached a later durability refusal instead of lineage refusal; and
promotion reached approval refusal instead of candidate refusal.

The repaired implementation has one central exact provenance gate. It parses
the retained normalized content, requires the exact ordered sixteen-row intake
coverage set, derives every populated candidate leaf through the production
source-kind mapping, and requires byte-semantic equality with the candidate's
closed typed `field_sources` sequence. The gate runs:

- before candidate bundle persistence;
- over every current candidate `1.3` during the two-scan locked author
  inventory;
- during selected candidate/result authority validation used by approval and
  promotion; and
- during pending-transaction recovery before any canonical mutation.

The author, approval, promotion, and recovery matrices are GREEN for all eight
variants. The recovery matrix coherently rebinds the candidate subject,
semantic result, exact result document, final candidate, approval, promotion,
lifecycle transition, and frozen intent-`1.2` identities, then proves refusal
while preserving the pending journal and every forged byte.

Normative vectors now carry all 113 exact populated-leaf source rows and the
sixteen exact coverage rows for both create and amendment chains. The runtime
validator independently recomputes the candidate subject, semantic result,
LF-inclusive exact result-document SHA-256 and byte length, and final candidate
identity. The create chain now ends at promotion fingerprint
`sha256:df1828939888358b9c0bc6b8a60ed2a14cf07aa08853efc5cb17cb6fd7014bf0`;
the amendment chain ends at promotion fingerprint
`sha256:fae45da33b50acb6e77408eb6bd8e58aa9d1bbd335315d2d50c464f1dc4c0b03`.
Lifecycle-validation result `1.0` and promotion intent `1.2` schema documents
remain byte-for-byte unchanged.

## Complete proof replay

| Proof | Result |
|---|---|
| Review 3 admitted subject | 48/48 paths; aggregate `sha256:297f71b491bc499d662d4b3c4c6d500e58c21544b819fee73ab214e0286361d1`; one Required finding accepted |
| provenance-forgery workflow matrices | PASS, 8 author + 8 approval + 8 promotion cases |
| provenance-forgery recovery matrix | PASS, 8 coherently rebound pending-transaction cases with byte preservation |
| `cargo test -p handbook-engine --test hcm_2_2_authority_repair` | PASS, 27 tests |
| approval/authority/lifecycle/lineage/promotion/vector/test-surface focused suites | PASS, 38 tests |
| `cargo test -p handbook-engine --lib --all-features` | PASS, 139 tests, including every W0-W15, R0-R9, and S0-S11 matrix |
| `cargo test --workspace --all-targets --all-features` | PASS, 299.9 seconds |
| `cargo check --workspace --all-targets --all-features` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --doc` | PASS, 2 compiler and 7 engine doctests |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` | PASS |
| `cargo fmt --all -- --check` and `git diff --check` | PASS |
| WSL installed-runtime and live-skill smokes | PASS, terminal `OK` |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS, 173 regular members; 170 source-identical including `Cargo.toml.orig`; all 62 definitions exact |
| packaged engine archive | 452,550 bytes; SHA-256 `eca76fb8ed0dd6673b82b9eba82831319685fc12e84e925027db4336b14c6a5e` |
| HCM-2.2 JSON/JSONL and Draft 2020-12 wall | PASS, 26 files, 36 duplicate-rejecting parse units, 13 valid schemas |
| runtime identity replay | PASS, 22 fingerprints, 2 candidate subjects, and 2 LF-inclusive exact result documents |
| lifecycle-result `1.0` schema | 15,430 bytes; SHA-256 `1d7d8733b19599804fcda32fe119766b1b910e2223aa981fcd9eb92cc4d5c03f`; unchanged |
| promotion-intent `1.2` schema | 16,019 bytes; SHA-256 `c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`; unchanged |
| archive boundary normal and negative self-test | PASS |
| handoff validator | PASS, 50 records, 254 current dispatches, 8 admitted legacy dispatches, 50 ledger entries |
| handoff v1-admission and orchestration negative self-tests | PASS |

## GitNexus and next-review boundary

The new provenance helper and new test matrices are not present in the
baseline index and report `UNKNOWN` with zero indexed upstream impacts. The
existing `field_sources` and coverage mapping helpers remain LOW and confined
to the reviewed Charter authoring path. The recovery regression is likewise a
new unindexed test symbol. No new or expanded HIGH/CRITICAL process or module
entered the subject. Final `detect-changes` remains a post-`CLEAN`, pre-commit
gate.

This proof does not declare the subject clean. The complete expanded exact
subject must go to another different fresh isolated read-only reviewer. Any
finding requires additive remediation, complete proof replay, a new exact
dispatch, and another different fresh reviewer. HCM-2.3 remains unauthorized.
