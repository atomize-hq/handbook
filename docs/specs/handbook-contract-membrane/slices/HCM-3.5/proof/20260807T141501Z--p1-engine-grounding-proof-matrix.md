# HCM-3.5 P1 engine grounding — blocked proof matrix

**Dispatch:** `20260807T141501Z--HCM-3-5--engine-grounding-bounded-delta-implementation`
**Status:** BLOCKED before implementation

## Test-first evidence

A new focused integration-test scaffold was created first and run with:

```text
cargo test -p handbook-engine --test hcm_3_5_grounding
```

It failed as expected before implementation with `E0432`: the selected public
`handbook_engine::grounding` module does not yet exist. The scaffold was then
removed rather than leave the crate in a known-failing state after the authority
gap below was established.

## Required proof status

| Required proof | Status | Blocking evidence |
|---|---|---|
| Exact current source pair produces a bounded grounded result with typed provenance and non-promoting evidence | BLOCKED | No selected summary definition or validated derived-delta input exists. |
| Missing, duplicate, substituted, incompatible, reversed, stale, malformed, and missing refs refuse | BLOCKED | The required exact ref-to-source mapping/layout is not selected. |
| Every currentness family and slot is enforced | BLOCKED | The fixed projection pair models five captured families, but the required independent current source and derived delta are absent. |
| Redaction occurs before source reading and leaves typed omission | BLOCKED | A compliant result needs the missing exact disclosure/summary definition; a fabricated policy would widen authority. |
| Resolution and summary overflow follow the fixed definition without widening the envelope | BLOCKED | No maximum, eligibility/order, or overflow behavior has been selected. |
| Summary is stably ordered, accountably redacted, and never exposes raw delta data | BLOCKED | No `handbook.snapshot-delta` / `signals` fixture is available for the P1 request to bind and reconcile. |
| No private HCM-3.4 type export or Flow/pipeline/CLI/compiler change | PASS (pre-edit) | No code paths were changed. |

## Safety checks

- `git diff --check` passed for the incomplete P1 subject. A final P1 code
  allowlist check remains deferred until code-bearing implementation is
  separately unblocked.
- The HIGH-risk existing projection executor was not edited or used. Its impact
  and the unavailable module-name impact lookup are recorded in the paired
  source/impact record.
- No Cargo, fixture, schema, configuration, Flow, pipeline, compiler, CLI,
  SDK, gate-runtime, protected-checkout, staging, commit, ref, or remote action
  was performed.
