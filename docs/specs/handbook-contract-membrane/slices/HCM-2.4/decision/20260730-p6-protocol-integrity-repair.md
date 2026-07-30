# HCM-2.4 P6 protocol-integrity repair

Status: accepted bounded repair authority

Date: 2026-07-30

Primary implementation commit preserved:
`b1edf060710bc96cd61631755519baba864ba30a`

New parent orchestration ID:
`20260730T174739Z--HCM-2-4--p6-protocol-integrity-repair-orchestration`

Consumed resume handoff:
`20260729T224742Z--HCM-2-4--orchestration--p6-selector-entry-partial`

## Context

The first P6 implementation attempt correctly preserved and committed the
reviewed production/test/control-pack subject at `b1edf060710bc96cd61631755519baba864ba30a`,
but eight new internal review dispatches reused the consumed handoff's already
closed parent orchestration ID:
`20260729T222320Z--HCM-2-4--p6-selector-entry-orchestration`.

Ordinary `validate_handoffs.py` therefore fails closed on the immutable selected
handoff because the eight July 30 dispatches appear after its
`2026-07-29T22:47:42Z` population cutoff. A later handoff or `supersedes` entry
cannot repair that population invariant.

The operator explicitly authorizes this bounded protocol-integrity repair. The
authorization preserves `b1edf060710bc96cd61631755519baba864ba30a` and forbids
amend, reset, rebase, history rewrite, validator-semantics changes, historical
admission-hash changes, and additional scope except for the two exact expanded
remediations below.

## Expanded remediation authority

The fresh causal discovery review at
`handoffs/dispatches/20260730T183600Z--HCM-2-4--p6-protocol-integrity-final-review.json`
returned exactly two P2 findings. Its dispatch is immutable discovery evidence
with SHA-256
`4ad28395b7de970d4047ff2e177c1ea0058a91ebd92578ad2a07465a1be48dbb`.
The operator expands this repair only to remediate:

- `HCM-2.4-P6-FIXED-KIND-AUTHORITY-REPAIR-001`: remove
  `CanonicalArtifactKind` from normal selection, loading, validation,
  rendering, packet, budget, fixture, blocker, refusal, ordering, and
  fingerprint authority. Normal identity comes from the admitted descriptor's
  instance ID, exact kind ref, label, and path. A compatibility enum may remain
  only outside normal-path authority. Compile propagation is limited to the
  original P6 production/test ceiling and accepted amendments. The three exact
  existing renderer tuples remain the only executable renderer dispatch.
- `HCM-2.4-P6-PROTOCOL-TRUTH-REPAIR-001`: correct current truth only in
  `00-README.md`, `03-seam-crosswalk.md`, `04-phase-slice-map.md`,
  `06-proof-and-regression-ledger.md`, HCM-2.4 `SPEC.md`, `tasks/plan.md`, and
  `tasks/todo.md`; classify the eight deleted dispatches and their results as
  invalid historical bytes, remove claims based on them or
  `sha256:814a1899c282511e1c3a5d9bb464308dea87f3dfd47825799bcfb2b672d93a38`,
  and keep P6 acceptance pending with P7 ineligible until a newly valid
  different-fresh closure is CLEAN.

The implementation must add or update bounded regression proof that an
admitted non-fixed kind cannot panic, acquire fixed-kind authority, or lose its
exact bytes or descriptor identity. Fresh GitNexus upstream impact is required
before every symbol edit. Any HIGH or CRITICAL surface must be reported before
editing.

### Bounded compile-propagation amendment

The operator additionally authorizes exactly two production match-arm edits
needed to propagate the descriptor-owned identity through the existing
compiler boundary:

| Production path | Symbol | Authorized arm-only edit |
| --- | --- | --- |
| `crates/compiler/src/resolver.rs` | `map_subject_ref` | Only the `CanonicalArtifact` arm may propagate the admitted descriptor's instance ID, exact kind ref, label, and path. Every unrelated arm remains byte-for-byte outside this amendment. |
| `crates/compiler/src/rendering/json.rs` | `render_subject_json` | Only the `CanonicalArtifact` arm may render the existing JSON structure and user-facing label from propagated descriptor data. Every unrelated arm and every other JSON field remains outside this amendment. |

This amendment adds no generic or configured renderer execution, fixed-kind
reconstruction, public abstraction, JSON field, protocol version, fixture,
schema, validator behavior, or additional production/test path. Its ancillary
allowance is frozen at exactly two production paths, two existing private
symbols, two `CanonicalArtifact` match arms, and no other mutation. A fresh
v1.4 selector-amendment review must be CLEAN before either Rust arm is edited.
If lossless propagation requires any other production or test path, symbol,
arm, helper, assertion, or field, the repair stops for further authority.

Fresh pre-edit GitNexus upstream impact on 2026-07-30 returned `UNKNOWN` with
zero indexed callers/processes/modules for both private functions, including
exact file disambiguation:

- `map_subject_ref` in `crates/compiler/src/resolver.rs`;
- `render_subject_json` in `crates/compiler/src/rendering/json.rs`.

The missing private-symbol graph coverage is not treated as LOW risk. The
selector-amendment review must use the exact textual consumers and compile
propagation to decide sufficiency, and any unexplained caller, process, module,
or required additional edit stops the repair.

The fresh selector-amendment review identified
`HCM-2.4-P6-JSON-PACKET-LABEL-PROPAGATION-001`. The operator authorizes its
smallest exact remediation in the already-added JSON production path:

| Production path | Symbol | Authorized expression-only edit |
| --- | --- | --- |
| `crates/compiler/src/rendering/json.rs` | `render_packet_sources_json` | Only the value expression for the existing `"kind"` key may change to use the descriptor-propagated label. |
| `crates/compiler/src/rendering/json.rs` | `render_packet_sections_json` | Only the value expression for the existing `"kind"` key may change to use the descriptor-propagated label. |

Both expressions must preserve the `"kind"` key, JSON structure, field order,
escaping, and existing first-party output values. Neither may call, retain, or
recreate a fixed-kind mapping. Every other expression in both functions remains
outside the amendment. Fresh pre-edit GitNexus upstream impact on 2026-07-30
returned `UNKNOWN` with zero indexed callers/processes/modules for both private
functions, so exact textual caller and compile-propagation review remains the
required authority check.

The immutable discovery dispatch remains SHA-256
`4ad28395b7de970d4047ff2e177c1ea0058a91ebd92578ad2a07465a1be48dbb`.
The first selector-amendment review dispatch remains unchanged at SHA-256
`a72bded3781710ce9779bb6647096f0c7abc8ce13ecad0bc4bb33347eb59593e`.
A different-fresh supplemental selector-amendment closure must be CLEAN before
any Rust edit. This authorization adds no production/test path; if compilation
requires one, the repair stops.

### JSON presentation-only compact label rule

The first supplemental selector closure exposed the semantic conflict now
tracked as `HCM-2.4-P6-JSON-PRESENTATION-LABEL-CONFLICT-001`: the admitted
descriptor labels for Project Context and Environment Context contain ASCII
spaces, while the existing JSON compatibility values do not. The operator
resolves that conflict with one exact presentation-only rule:

> For the three authorized JSON artifact-kind projections only, derive the
> output value from the admitted descriptor label by removing every ASCII SPACE
> character (U+0020), and perform no other transformation.

The examples are exact:

- `Charter` -> `Charter`;
- `Project Context` -> `ProjectContext`;
- `Environment Context` -> `EnvironmentContext`.

No trimming, case folding, punctuation removal, Unicode normalization, or
other character transformation is authorized. The rule applies only to:

- the `CanonicalArtifact` arm of `render_subject_json`;
- the existing `"kind"` value expression in `render_packet_sources_json`;
- the existing `"kind"` value expression in `render_packet_sections_json`.

The compact label is output compatibility only. It must never participate in
selection, loading, validation, ordering, renderer dispatch, budgets,
fingerprints, blockers, refusals, packet identity, or artifact identity. The
admitted descriptor's exact instance ID and kind ref remain authoritative, and
its exact label remains retained without mutation.

Bounded regression authority is frozen to the existing P6 test path
`crates/compiler/tests/rendering_surface.rs`. Tests in that file may prove all
three JSON projections preserve the existing first-party values and that only
U+0020 is removed from a descriptor label while case, punctuation, non-ASCII
whitespace, and Unicode code points remain unchanged. No fixture asset, helper
outside that test file, additional production/test path, JSON field, field
order, or protocol version is authorized. A second different-fresh
supplemental selector closure must be CLEAN before any Rust or test edit.

The immutable predecessor dispatches remain unchanged at SHA-256:

- discovery: `4ad28395b7de970d4047ff2e177c1ea0058a91ebd92578ad2a07465a1be48dbb`;
- first amendment review: `a72bded3781710ce9779bb6647096f0c7abc8ce13ecad0bc4bb33347eb59593e`;
- first supplemental closure: `c77a69490dd09ce5805f2b528de611f2ce5eb830e1358b54cd013dcb44f2b5fa`.

## Invalid active dispatch inventory

The following files are invalid as active protocol evidence because each binds
the closed orchestration ID above. Their exact bytes remain recoverable from
Git commit `b1edf060710bc96cd61631755519baba864ba30a`; this repair removes the
files from the active dispatch directory without modifying them and never
claims their results as valid evidence.

| Invalid active dispatch path | SHA-256 |
| --- | --- |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T021416Z--HCM-2-4--p6-pipeline-handoff-selector-amendment-review.json` | `d463f96c0deeea7b3ed8aa7d1df178a2c0dbd4b0e60769ecd1ac597bec86ae42` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T025942Z--HCM-2-4--p6-pipeline-handoff-selector-amendment-closure-review.json` | `f1ceece7202fd74f1e89ad4defe35b9e7d794e3fd5d4e7fd819cc02df80c215e` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T034241Z--HCM-2-4--p6-implementation-specification-review.json` | `91cf744c7b518494aea87428d7f54fccbb275fb096ebe5fb0f74b7c37bd9131f` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T034242Z--HCM-2-4--p6-implementation-standards-review.json` | `c56a5b1e19b6b181c4ee04418c3d24fbfaacb2366c99191f21b2bdb8eb2acb48` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T140720Z--HCM-2-4--p6-bounded-read-clippy-selector-amendment-review.json` | `a9b929468b6ab30d6817afbe1bb58ce92792aa4ca1c73e06093de36e06873344` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T141812Z--HCM-2-4--p6-bounded-read-clippy-selector-amendment-closure-review.json` | `84788495780a163eeddc2794c2433352e05cfa496278f4a3625a58f61a1fbf85` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T145250Z--HCM-2-4--p6-final-implementation-review.json` | `e5cb13fb18cff886585f00843e7fa5d683df1407d61d787680024029c7fef6f7` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260730T155751Z--HCM-2-4--p6-final-implementation-closure-review.json` | `b357615506ad1dc0b17f15b0918037b7befe14ea1f176552735047d90480b5eb` |

## Decision

1. Remove exactly the eight inventoried invalid files from the active
   `handoffs/dispatches/` directory. Do not edit their bytes or describe their
   reviewer results as valid proof.
2. Preserve commit `b1edf060710bc96cd61631755519baba864ba30a` unchanged.
3. Start the new unique parent orchestration
   `20260730T174739Z--HCM-2-4--p6-protocol-integrity-repair-orchestration`,
   consuming the selected P6-entry handoff only as resume context.
4. Create new v1.4 dispatch IDs/files before executing new review. Derive the
   new causal budget, outcome registry, subject manifest, fingerprints, and UTC
   timestamps from the new parent and live repaired subject.
5. Treat every prior P6 review result in the invalid dispatch set as unusable.
   Obtain a fresh built-in independent review of the complete final P6 subject,
   including this repair and active-directory deletion proof. Remediate all
   valid P1/P2 findings and obtain different-fresh closure when remediation
   changes the reviewed subject.
6. Replay the complete P6 proof wall, ordinary handoff validation, both
   self-tests, record/index parity, deterministic ledger rebuild, formatting,
   whitespace checks, and scoped GitNexus change detection.
7. Commit the repaired reviewed primary state, then create the new v1.4 P6
   handoff and rebuilt ledger in a separate mechanical closeout commit.

## Scope and stop conditions

This decision authorizes only this decision file, deletion of the eight named
invalid active dispatches, entirely new P6 review dispatches under the new
parent orchestration, the two exact expanded remediations above within the
original P6 ceiling, the two exact compiler match arms frozen by the bounded
compile-propagation amendment, the repaired primary commit, and the mechanical
handoff/ledger closeout.

Stop without further changes if repair requires:

- validator or schema semantics changes;
- historical record, dispatch, or admission-hash mutation;
- production/test behavior changes outside
  `HCM-2.4-P6-FIXED-KIND-AUTHORITY-REPAIR-001`;
- a production/test path beyond the original P6 selector and its accepted
  amendments, a fixture change, or any other additional path;
- P7, Phase 2 exit, HCM-3.x, task-gate runtime, automatic continuation, push,
  or release.

P7 remains unselected and unstarted. Full HCM-2.4 and Phase 2 remain
incomplete. HCM-3.x and future task-gate runtime remain unauthorized.
