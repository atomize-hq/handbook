# HCM-3.5 P1 implementation-admission recovery selector

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P1

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**Bound task / host / nonce:** `019fdcd7-31ae-7881-8502-1b173d751c93` / `local` /
`f5fb20977fd6dad67ece2a8fd47f685de067b8cbf25cee9edd9c772e8a242d2e`

## Authority admission and lineage

The direct authority-admission grant is the user-authorized
`p1-definition-and-implementation-admission-recovery` packet bound to this
task and nonce. It directly resumes and supersedes the noncompleted same-parent
source handoff
`20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary`.
It does not rewrite P0, that handoff, its dispatches, or its ledger history.

The source checkout is exactly `925262b062cb6e7c3898db74f20d84772e46af1e`.
The final local integration ref remains independently fixed at
`7e0836a1c60f3992a80719b597f3fc18fabe807e` until P1 through P4 are CLEAN.

## Frozen P1 source artifacts

The P1 engine integration test owns a persisted fixture source route rooted at
`.handbook/grounding/hcm-3.5/v1` in its temporary test repository. No
production configuration, schema, or external consumer adopts this route.

| Artifact | Exact ref | Canonical JSON fingerprint |
| --- | --- | --- |
| Summary definition | `handbook.grounding.summary.hcm-3-5-p1@1.0.0` | `sha256:e553ba87b1182df8d4bc587259bcbe227e208b2e1ef670a8b6dbc35f4ed75e1d` |
| Disclosure policy | `handbook.grounding.disclosure.hcm-3-5-p1@1.0.0` | `sha256:ec46c655d094a69f7d46aa1788f337d2feb0acb72398526f4570d470d0e66948` |

The public opaque grounding-reference grammar is
`<exact-definition-ref>#sha256:<64-lowercase-hex>`. The caller supplies no
path, bytes, signal, filter, currentness tuple, or payload. The engine resolves
only the fixed source route and verifies the request fingerprint against the
immutable source artifact before processing it.

The persisted route contains prior/current capture inputs and snapshot records,
the exact drift catalog, and a currentness witness. The engine derives the
compatible delta from those exact endpoint records, checks the source-route
fingerprint, and retains the complete delta only inside the engine. This is the
separately reviewed P1-owned test fixture/source delta permitted by the recovery
grant; the HCM-3.4 fixture is read but never changed.

## Frozen summary and refusal semantics

- Maximum cardinality is **2**.
- Eligible kinds are exactly `expected_progress`, `proof_drift`, and
  `scope_expansion`.
- Stable order is ascending `(kind, signal_id)`.
- Excess eligible signals receive the typed `overflow` omission; they never
  widen the result or expose the complete delta.
- Noneligible signals receive the typed `ineligible` omission.
- The disclosure policy runs before affected-change/evidence extraction. A
  redacted signal receives the typed `redacted` omission.
- Included entries may contain only signal ID/kind, exact rule ref/fingerprint,
  affected after-fingerprints, and disclosure-permitted evidence or
  justification refs. They never contain raw changes, snapshot payloads,
  pointers, or unrestricted prose.
- The required currentness family set is exactly `evidence`, `git`,
  `handbook`, `session`, and `work`; all captured family revisions and the
  `work` source slots must match the persisted witness. Missing, extra,
  duplicate, substituted, or mismatched entries refuse.
- The definition's six minimum Resolution ranks are `[0, 0, 0, 0, 0, 0]`.
  An envelope that cannot produce a current authority view or falls below a
  selected minimum refuses; no raw-level or byte-pressure fallback exists.
- Unsupported/malformed source content, snapshot/delta incompatibility,
  reversed source endpoints, source fingerprint mismatch, and failed
  currentness are typed refusals. Operation-integrity errors remain payload-free
  outer errors.

`GroundingEvidence` remains non-promoting: both dimensions are only
`Unavailable` or `False` and `authority_effect` remains `none`.

## Implementation ceiling and review disposition

P1 may add the engine module and test fixture/source delta, and may add narrow
crate-private Snapshot Memory adapters. It may not touch Flow, pipeline,
compiler, CLI, SDK, Substrate, Cargo, dependencies, schemas, configuration,
gate runtime, a protected checkout, or a remote.

The retained `execute_projection_with_live_observer` seam has independently
reviewed HIGH upstream impact (four direct callers and four affected
currentness/projection processes). P1 leaves it unmodified and does not use it
as a shortcut. The Snapshot Memory owners have LOW impact and are the only
existing engine internals admitted for narrow adapter work.

The P1 review cadence is: one complete-subject discovery review, one
consolidated remediation if needed, then one different-fresh delta closure
review. P2 cannot begin until P1 is CLEAN.
