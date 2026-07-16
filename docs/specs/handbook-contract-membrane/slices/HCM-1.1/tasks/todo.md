# HCM-1.1 Task Ledger

This ledger records the live HCM-1.1 execution. Completed subject-shaping work
is checked before final review. Review, byte replay, commit, and closeout remain
unchecked in this reviewed file because their later immutable evidence belongs
in the final dispatch and parent v1.2 handoff rather than a post-`CLEAN` subject
mutation. Canonical `07`/`08` owns those mechanics.

## Entry and context

- [x] Start a new top-level HCM-1.1 run with the exact completed HCM-0.7 handoff and this packet.
- [x] Validate branch, HEAD, clean/attributable status, packet bytes, handoff/ledger parity, and Phase 0 dependencies.
- [x] Reassemble the bounded authority/repo-truth/proof capsule from live files.
- [x] Confirm the HCM-1.1 additive boundary and prohibited product-path/sibling files.
- [x] Read and apply the packet's complete required skill chain.
- [x] Refresh/check GitNexus; run upstream impact analysis before every existing symbol edit and warn on HIGH/CRITICAL risk.

## Task 1 — Exact identity and fingerprint primitives

- [x] Write failing exact-ref, SemVer, fingerprint, JCS, duplicate-key, and limit tests.
- [x] Add only the four pinned registry/identity dependencies, then replace the existing direct `libc` dependency with reviewed `rustix` descriptor-relative filesystem access after the no-follow race finding; inspect the lockfile diff.
- [x] Implement `ExactDefinitionRef` and `DefinitionFingerprint` validation.
- [x] Enforce the exact lowercase-ASCII identity segment/length grammar, canonical SemVer roundtrip, and single `@` delimiter with boundary negatives.
- [x] Implement type-specific RFC 8785 normalization and SHA-256 derivation.
- [x] Implement duplicate-key and fixed source-limit refusal.
- [x] Implement closed typed decoding so unknown top-level/nested and wrong-record fields refuse before fingerprinting.
- [x] Run focused tests, format, affected engine tests, scoped diff, and staged change detection.
- [x] Commit the green atomic increment.

## Task 2 — Stable roles and trusted source loading

- [x] Write failing frozen-fingerprint and unsafe-source tests.
- [x] Add exact packaged core stable-role registry 1.0.0 and 1.1.0 assets.
- [x] Implement typed stable-role parsing, validation, exact selection, and fingerprint replay.
- [x] Refuse unknown registry/role fields at every nesting level before fingerprinting.
- [x] Add the strict descriptor-relative registry reader beside the existing canonical-product reader, preserving legacy non-Unix product behavior while registry loads fail closed there.
- [x] Treat the explicitly selected repository root as the caller-provided trust anchor while retaining no-follow enforcement for every repo-relative component.
- [x] Prove missing, directory, symlink, traversal, duplicate-role, invalid-category, and substitution refusal.
- [x] Bound every outward registry-error location and replace authored source/exact-ref/document locations with stable field labels.
- [x] Prove both assets are included in package contents.
- [x] Run focused/affected tests, format, scoped diff, and staged change detection.
- [x] Commit the green atomic increment.

## Task 3 — Local Draft 2020-12 schema registry

- [x] Write failing schema entry, local closure, meta-schema, and structural validation tests.
- [x] Implement exact schema tuple/ref and entry/document/closure fingerprinting.
- [x] Refuse unknown schema-entry fields and non-empty schema-entry extensions before fingerprinting.
- [x] Implement explicit allowed roots and bounded local `$ref` prewalk.
- [x] Refuse root/nested authored IDs, anchors, dynamic/recursive-reference keywords, and plain-name fragments under the frozen v1 resource policy.
- [x] Require exact root Draft 2020-12 `$schema`; refuse missing/wrong/nested declarations and every schema-position keyword outside the frozen v1 allowlist.
- [x] Prove schema-aware traversal does not treat object data under `const`, `enum`, `default`, or `examples` as keyword-bearing schema nodes.
- [x] Bind each file to one exact internal base and prove prewalk, closure member, validator target, and reported schema location equality.
- [x] Encode normalized UTF-8 paths injectively into private resource URIs, translate only prewalked refs in a validator clone, and decode reported locations exactly.
- [x] Encode prewalked RFC 6901 fragment bytes injectively in validator-only URIs and decode reported space/non-ASCII pointer tokens exactly.
- [x] Translate same-document fragment-only refs through the current exact private resource rather than leaving raw URI fragments.
- [x] Build only an in-memory Draft 2020-12 registry with resolver features disabled and linear regex.
- [x] Return deterministic structural validation locations.
- [x] Bound structural schema locations independently from authored document-ref length with a stable `schema_root` fallback.
- [x] Prove duplicate/conflicting identity and all remote/ambient/traversal/query/backslash/encoded/cycle/missing/limit refusal cases.
- [x] Prove identifier rebasing, duplicate internal resource identity/alias, invalid JSON Pointer, and prewalk/validator target mismatch refusal.
- [x] Assert the dependency graph contains no HTTP/TLS/async resolver path.
- [x] Run focused/affected tests, format, scoped diff, and staged change detection.
- [x] Commit the green atomic increment.

## Task 4 — Capability-free kind meta-validation

- [x] Write failing exact capability-free kind and later-owned-field refusal tests.
- [x] Implement `ArtifactKindDefinition`, registry load, lookup, and fingerprint closure.
- [x] Enforce exact stable-role/schema rules without instance state.
- [x] Refuse unknown top-level/nested kind fields, wrong-record fields, and non-empty kind extensions before fingerprinting.
- [x] Prove kind records cannot carry instance path/label/requiredness/setup/intake authority.
- [x] Refuse every non-empty semantic-capability, required-capability, semantic-validator, renderer, lifecycle, review-trigger, Projection, extension, or opaque dependency input.
- [x] Prove forged digest, changed bytes, wrong dependency type, incompatible pointer-shape claim, and source-order substitution refuse before entering a fingerprint closure.
- [x] Prove duplicate/conflict/missing/mismatch role/schema negatives.
- [x] Run focused/affected tests, format, scoped diff, and staged change detection.
- [x] Commit the green atomic increment.

## Task 5 — Repository custom-kind proof

- [x] Write the failing public-API end-to-end custom-kind test.
- [x] Add a primary capability-free exact kind plus one distinct proof-only companion, two schema entries/roots, a local-ref closure, and valid primary YAML fixture.
- [x] Prove both accepted source permutations yield identical schema/kind registry fingerprints, exact lookup sets, closures, and validation behavior.
- [x] Prove valid data passes and invalid data reports stable structural locations.
- [x] Prove no `CanonicalArtifactKind` variant or sibling product-path/CLI change exists.
- [x] Run complete engine tests, format, scoped diff, and staged change detection.
- [x] Commit the green atomic increment.

## Full proof and review

- [x] Run every focused positive case in SPEC.
- [x] Run every negative/fail-closed/security case in SPEC.
- [x] Run format, engine clippy/tests, full workspace tests, dependency-tree, package-content, archive, handoff-validator, and diff/scope proof.
- [x] Update only the exact crosswalk and bounded PG-KIND-01/PG-KIND-02 structural evidence earned; leave both gates and all siblings open.
- [x] Preserve a complete replayable HCM-1.1 proof wall.
- [x] Assemble an immutable complete-subject review dispatch.
- [x] Spawn a fresh isolated read-only built-in `default` reviewer and collect its structured result.
- [x] Validate every finding against live authority.
- [x] After any valid finding, remediate within scope, rerun the full proof wall, and use a different fresh reviewer.
- [ ] Require final CLEAN over the exact complete subject; do not mutate subject bytes afterward.

## Commit and parent closeout

- [ ] Replay the final clean manifest and full proof byte-identically.
- [ ] Stage only the reviewed HCM-1.1 subject and proof evidence.
- [ ] Run staged `git diff --check`, exact diff inspection, secret scan, and GitNexus change detection.
- [ ] Commit the reviewed HCM-1.1 implementation first.
- [ ] Create one completed v1.2 parent handoff bound to the primary commit and clean review lineage.
- [ ] Rebuild the ledger and run all validator/self-test modes.
- [ ] Stage only the mechanical handoff/ledger artifacts and run staged change detection.
- [ ] Commit the mechanical closeout separately.
- [ ] Stop without starting HCM-1.2.
