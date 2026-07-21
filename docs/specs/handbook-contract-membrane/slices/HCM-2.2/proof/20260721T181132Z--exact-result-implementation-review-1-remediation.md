# HCM-2.2 exact-result implementation Review 1 remediation

**Proof ID:** `HCM-2.2-ERIR-R1-REMEDIATION`  
**Recorded:** `2026-07-21T18:11:32Z`  
**Authority class:** additive implementation remediation proof; no clean or
landing claim  
**Reviewed subject:**
[`20260721T165500Z--HCM-2-2--fresh-exact-result-implementation-review-1.json`](../../../handoffs/dispatches/20260721T165500Z--HCM-2-2--fresh-exact-result-implementation-review-1.json)

## Disposition

Fresh isolated implementation Review 1 admitted exact 41-path subject
`sha256:c154a2ef04ee858120324e091c48385c3e363d63a402679aceb641947e6bf38c`
and returned `CHANGES_REQUIRED` with three Required findings. Every finding is
accepted without waiver. The dispatch and original implementation proof wall
remain byte-immutable evidence.

| Finding | Accepted repair |
|---|---|
| Frozen lifecycle-result schema was not fully enforced | Embed a semantic copy of the unchanged result `1.0` schema in the engine package; compile it once with format checking; validate duplicate-free canonical result JSON against the complete schema before semantic admission. |
| Recovery did not recompute the exact approval quorum and lexical order | Resolve the current registry/profile and create/amend basis under pre-commit authority; derive the exact required `(approval_class_ref, authority_role_ref)` vector; require selected approvals and intent pairs to match it exactly in strict lexical order. |
| Promotion output accepted a subset or reorder of resolved definitions | Derive the exact ordered thirteen-definition lifecycle-result projection and require promotion, intent, and recovery outputs to equal it byte-semantically without sorting, subset admission, or deduplication. |

No released definition changed. The lifecycle-validation-result `1.0` and
promotion-transaction-intent `1.2` schema documents remain byte-identical to
baseline. Candidate `1.0`, `1.1`, and `1.2` remain historical-only.

## Test-first repair evidence

Before each production repair, a public production-path regression reproduced
the admitted state:

- changing only `validated_at_utc` to a schema-invalid value replayed as valid;
- a retained promotion coherently rebound to an unrequired approval pair
  committed; and
- recovery coherently rebound to a target-kind-only resolved-definition subset
  rolled forward.

All three tests were observed RED against the Review 1 bytes and are GREEN
after repair. Additional negatives reject malformed refs, invalid timestamps,
257 observations, 17 coverage IDs, duplicate/reordered approval pairs, and
subset/reordered recovery outputs. The retained Review 3 coherent result,
witness, and binding rewrite attack remains a GREEN refusal regression.

The corrected normative promotion fingerprint is
`sha256:48081ceb3d4ea4ed19bb32866faa5595ba86974cbf19df6db89b19179fb48900`.
Its record, preimage, candidate `1.3` binding, and target-kind external binding
all carry the exact ordered thirteen-definition projection.

## Complete proof replay

| Proof | Result |
|---|---|
| `cargo test -p handbook-engine --lib` | PASS, 138 tests, including every W0-W15, R0-R9, and S0-S11 matrix |
| focused HCM-2.2 integration suites | PASS, 48 tests across exact-result repair, approval use, promotion, transaction, authority, and definition registry |
| `cargo test -p handbook-engine --test hcm_2_2_runtime_vectors` | PASS |
| `cargo test --workspace --all-targets --all-features` | PASS, 236.1 seconds |
| `cargo check --workspace --all-targets --all-features` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --doc` | PASS, 2 compiler and 7 engine doctests |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` | PASS |
| `cargo fmt --all -- --check` and `git diff --check` | PASS |
| WSL installed-runtime and live-skill smokes | PASS, terminal `OK` |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS, 173 regular members; 170 source-identical members including `Cargo.toml.orig`; all 62 definitions exact |
| packaged engine archive | 445,472 bytes; SHA-256 `6508ea9ac219f0c938cf922d303c29d38aab7a0d5f80659444b08109d0ae6fff` |
| lifecycle-result `1.0` schema | 15,430 bytes; SHA-256 `1d7d8733b19599804fcda32fe119766b1b910e2223aa981fcd9eb92cc4d5c03f`; unchanged |
| promotion-intent `1.2` schema | 16,019 bytes; SHA-256 `c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`; unchanged |
| HCM-2.2 JSON/JSONL and Draft 2020-12 wall | PASS, 26 files, 36 duplicate-rejecting parse units, 13 valid schemas |
| handoff validator | PASS, 50 records, 252 current dispatches, 8 admitted legacy dispatches, 50 ledger entries |
| handoff v1-admission and orchestration self-tests | PASS |
| archive boundary normal and negative self-test | PASS |
| changed-subject local links | PASS, 9 Markdown files and 69 local links before this additive proof |
| scope and secret gates | PASS, 45 paths before this additive proof; no HCM-2.3 path or added-line secret signature |

The packaged embedded result schema is semantically equal to the frozen
documentation schema; the authoritative documentation schema itself is the
unchanged 15,430-byte document above. The generated package-only `Cargo.toml`
and `Cargo.lock` account for the remaining archive metadata differences.

## GitNexus scope and review boundary

Upstream impact was reproduced before every existing-symbol edit. Across 213
analyzed symbols, the approved envelope remained 18 CRITICAL, 20 HIGH, 5
MEDIUM, and 170 LOW. The principal elevated paths remained bounded to:

- `evaluate_charter_intake`: HIGH, 11 impacts, 7 direct callers, 2 processes,
  3 modules;
- generic lineage `validate_record`: CRITICAL, 73 impacts, 10 direct callers,
  20 processes, 16 modules;
- `current_promotion_anchor`: HIGH, 11 impacts, 1 direct caller, 3 processes,
  1 module; and
- promotion recovery: HIGH, 10 impacts, 1 direct caller, 3 processes, 3
  modules.

No new or expanded HIGH/CRITICAL process or module entered the subject. Final
GitNexus change detection remains a post-`CLEAN`, pre-commit gate.

This proof does not declare the subject clean. The next exact manifest must go
to a different fresh isolated read-only reviewer. Any finding requires another
additive repair, full proof replay, new exact dispatch, and another different
fresh reviewer. HCM-2.3 remains unauthorized.
