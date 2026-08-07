# HCM-3.5 audit-corrective source-ingestion proof matrix

**Phase / slice / packet:** HCM-3 / HCM-3.5 /
audit-corrective-source-ingestion-negative-proof-and-replayable-closeout

**Corrective parent:** `handbook-hcm-3-5-audit-corrective-implementation-20260807`

**Authority binding:** direct user authorization for task
`019fddfa-610b-7512-b591-654ba4f5130a`, host `local`, meta task
`019fda65-3892-7900-a692-2dd0a0fccc7a`, nonce
`dd04871aad668887df84520236dca6db213aa2975a5353f38f0ee3f7f11db63c`.

## Discovery findings and one remediation

Fresh discovery run `hcm35_discovery_review` returned:

- `HCM35-AC-DISC-001`: P1/P4 accepted unbounded direct reads and unsafe file
  types;
- `HCM35-AC-DISC-002`: live HCM-3.5 JSON paths bypassed duplicate-key
  rejection;
- `HCM35-AC-DISC-003`: the required negative matrix was missing; and
- `HCM35-AC-DISC-004`: the historical P1–P4 completed record had no replayable
  dispatch/review population despite asserting CLEAN.

`DISC-001` through `DISC-003` are reconciled by one in-scope code change:
`read_bounded_regular_source` composes the existing strict local regular-file
reader, `SourceByteBudget`, and existing duplicate-key-safe
`parse_schema_json`. P1 and P4 each allocate one operation-local budget and
map every adapter/parser refusal only to their existing typed surface. Shared
snapshot derivation uses the same parser. No raw source byte, path, or parser
detail is returned by a P1 refusal or P4 materialization error.

`DISC-004` is not a historical repair. The historical record remains
byte-for-byte unchanged; this new corrective parent supplies new v1.4
dispatches, delegated reviews, a final manifest, and its own completed record
and ledger entry only after the required CLEAN final review.

## Negative source matrix

The P1 test invokes `resolve_with_authority`, the exact source-ingestion branch
called by public `ground_resolution` after its pre-existing envelope admission.
It asserts the same public `GroundingOutcome::Refused` kind that
`ground_resolution` exposes, with no grounded resolution and no omission-backed
usable output. The pre-existing public integration test continues to validate
the opaque public reference boundary. P4 calls the private materializer and
asserts that no transition refs materialize.

| Input case | P1 existing typed refusal | P4 existing typed error | Output proof |
| --- | --- | --- | --- |
| malformed | `MalformedSource` | `InvalidSource` | No grounded resolution or refs. |
| mismatched | `SourceMismatch` | `InvalidSource` | No grounded resolution or refs. |
| reversed | `IncompatibleDelta` | `InvalidSource` | No grounded resolution or refs. |
| stale | `StaleCurrentness` | `InvalidSource` | No grounded resolution or refs. |
| partial | `MissingSource` | `InvalidSource` | No grounded resolution or refs. |
| tampered | `DefinitionMismatch` | `InvalidSource` | No grounded resolution or refs. |
| duplicate-key | `MalformedSource` | `InvalidSource` | Duplicate values are rejected before use. |
| oversized | `MalformedSource` | `InvalidSource` | 1 MiB document ceiling is enforced before parsing. |
| symlinked | `MalformedSource` | `InvalidSource` | Strict no-follow admission refuses the link. |

The initial table run was intentionally RED: a duplicate key produced a P1
grounded result and P4 refs. After the adapter/parser correction both focused
matrix tests passed locally. Test assertions contain only case labels and
typed results; no raw source data is emitted.

## Local validation state

| Check | Result | Evidence |
| --- | --- | --- |
| P1 table | PASS | `cargo test -p handbook-engine hcm_3_5_source_refusal_matrix_is_fail_closed_without_usable_output --lib` |
| P4 table | PASS | `cargo test -p handbook-engine hcm_3_5_transition_source_refusal_matrix_never_materializes_refs --lib` |
| Existing public HCM-3.5 ref boundary | PASS | `cargo test -p handbook-engine --test hcm_3_5_grounding` |
| Formatting | PASS | `cargo fmt --all -- --check` |
| GitNexus graph impact/detection | UNAVAILABLE / NOT GREEN | No local MCP graph tool, `.gitnexus/`, or `.gitnexus/run.cjs` is present. |

The retained Flow, pipeline, projection, resolver, and compiler seams were
read-only regression context. They were neither called as an implementation
shortcut nor changed. The protected checkout and all remote state remain
untouched.
