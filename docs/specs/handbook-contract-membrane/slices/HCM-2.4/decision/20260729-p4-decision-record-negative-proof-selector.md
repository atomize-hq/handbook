# HCM-2.4 P4 Decision Record negative-proof selector

Status: completed bounded selector; v1.4 selector review and different-fresh
implementation closure review are CLEAN, and P4 is GREEN.

Date: 2026-07-29

## Trigger and authority

Commit `00dde0162fcb15576c83b6ed40ab7286488d0890` already provides the
review-clean P4 positive generic-operation and coverage-token prerequisite.
The remaining P4 gate is independent proof that Decision Record support adds
none of the five forbidden surfaces frozen below. This selector is limited to
that test/proof gap and does not reopen the reviewed production expression.

P5 evidence may guide the test shape only. Every assertion, fixture operation,
command result, and proof claim for this packet must execute against the P4
Decision Record target and its own selected repository profile.

## Exact implementation selector

Edit only `crates/engine/tests/hcm_2_4_decision_record.rs` for executable proof.
Add private test-only imports/helpers only when needed by the exact tests below.
Do not edit an existing positive test, the Decision fixture, production code,
definitions, schemas, renderer goldens, CLI code or snapshots, Cargo metadata,
dependencies, public APIs, or released bytes.

Add exactly these five independent negative tests:

1. `shipped_root_has_no_decision_record_instance_or_default`
   resolves the built-in `handbook.profile.shipped-root@1.2.0` through the
   P4 fixture selection inventory, proves `decision_record` is absent from the
   root, and proves the selected child adds exactly that one repository-owned
   descriptor.
2. `author_command_inventory_has_no_decision_record_surface`
   reads the authoritative `AuthorCommand` enum source and the consumed
   `handbook-author-help.txt` emitted-surface snapshot, proves the two admitted
   commands remain Charter and Project Context, and refuses Decision Record
   spellings in identifier, kebab-case, snake-case, and label form.
3. `missing_exact_decision_filename_refuses_inferred_and_dynamic_decoys`
   removes only `.handbook/records/decision.yaml`, installs plausible alternate
   YAML filenames, and proves generic repository read still returns the exact
   missing-artifact refusal rather than discovering or inferring a filename.
4. `decision_record_projection_widening_is_refused`
   proves the admitted descriptor has no projection refs, injects a Decision
   Projection ref into a cloned descriptor inventory, and proves the existing
   registry rejects the widened inventory.
5. `markdown_decoys_have_zero_decision_record_authority`
   proves the fixture begins with no Markdown mirror, generic read/validate do
   not emit one, malicious plausible Markdown decoys cannot change canonical
   content or fingerprint, and removing the canonical YAML still refuses
   rather than falling back to either decoy.

Add exactly one Decision-specific positive replay test,
`fixed_decision_renderer_golden_is_deterministic_and_resolution_free`, which
recomputes the existing renderer typed-closure fingerprint, validates the
existing golden input through the selected Decision schema, and proves the
existing Markdown byte length and SHA-256 with `resolution_input: null` and no
external inputs. This is retained positive proof, not a new renderer surface.

## Preserved positive proof

The complete target must retain and reverify without weakening:

- profile `example.profile.hcm-2-4-decision-record@1.0.0`;
- descriptor `decision_record`;
- canonical real path `.handbook/records/decision.yaml`;
- generic HCM-2.3 intake/candidate/promotion/read/validate behavior;
- the complete ordered coverage-token list;
- stale-basis and malformed-request refusal;
- deterministic renderer bytes; and
- byte-exact retained and promoted canonical YAML.

The unchanged `hcm_2_3_generic_lineage` target remains the generic lineage
preservation wall. The P4 integration target itself is the independent
five-part negative proof; no P5 test result counts toward P4 acceptance.

## Exact proof and review wall

Before implementation, obtain a fresh read-only v1.4 selector review over the
replayable selector subject. After implementation, run at minimum:

- `cargo test -p handbook-engine --test hcm_2_4_decision_record`;
- each of the five new negative tests independently with `--exact`;
- `cargo test -p handbook-engine --test hcm_2_3_generic_lineage`;
- strict Clippy for the engine library and Decision integration target;
- `cargo fmt --all -- --check`;
- the live supported engine verification required by current authority;
- JSON parsing, handoff validators, deterministic ledger proof, and
  `git diff --check` at closeout.

Freeze a v1.4 implementation-review dispatch only after the complete packet
wall, recursive consumer inventory, subject-manifest replay, formatting, and
whitespace convergence checks pass. A CLEAN result with no unresolved P1/P2 is
required before P4 status becomes GREEN.

## Non-goals and stop conditions

This selector does not authorize P6, P7, bridge deletion, full HCM-2.4 or
Phase 2 completion, HCM-3.x, capitalized Projection, Resolution, a persistent
view, a generated/dynamic command, filename inference, another artifact
family, a push, or automatic continuation.

Stop and report rather than implement if proof requires a production symbol,
fixture-content change, released definition/schema/golden change, public API,
dependency, Cargo metadata, token grammar, additional command, or a path
outside the exact test/proof/status/control-plane ceiling above.
