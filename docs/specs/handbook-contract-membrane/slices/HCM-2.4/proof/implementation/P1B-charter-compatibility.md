# HCM-2.4 P1B Charter compatibility implementation proof

Status: review-clean. Discovery findings were remediated and a different-fresh
closure review found no P1/P2.

## Entry and authority

P1A closed review-clean at complete-subject fingerprint
`sha256:1bdb1bd3a912f590ab9af31586afbca4acb538235e26146ddd7a17efecdd8ad9`.
Its immutable successor profile fingerprint is
`sha256:63cd999c95efc3fe65ae3514c2915b5d1457290211cf6da7b57b2cd75bafaf83`.

P1B changes only the exact production selectors in `SPEC.md` and
`implementation-packet-path-manifest-v1.0.md`. The live shipped request remains
`handbook.profile.shipped-root@1.1.0`.

## Impact and caller ceiling

The refreshed GitNexus Rust parser resolves the two profile constants exactly
at LOW/0 and resolves the selected `impl`/file containers at LOW or MEDIUM, but
does not expose the frozen private function UIDs. The reviewed exact P0
baseline therefore remains the conservative authority:

- `evaluate_charter_intake`: HIGH, 42 impacted, 6 direct, 1 process, 4 modules;
- `validate_candidate_currentness`: HIGH, 17 impacted, 1 direct, 1 process,
  3 modules;
- `CharterAuthorityTransactionServiceV1::preflight`: HIGH, 27 impacted,
  2 direct, 1 process, 3 modules;
- `validate_candidate_contract`, `promote_at`, `build_result`, and
  `validate_definition_authority`: LOW at the frozen exact baseline; and
- the no-edit candidate and promotion-intent validators remain CRITICAL and
  HIGH preservation anchors.

Live source has exactly seven production
`validate_selected_decisions` callers: the six frozen callers plus the one
authorized `evaluate_charter_intake` edge. There is no eighth caller.

## Implemented membrane

`CharterDefinitionRegistry::validate_selected_decisions` now admits only:

1. released profile `1.1` with fingerprint
   `sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74`;
2. successor profile `1.2` with the exact P1A fingerprint above.

Both tuples must resolve the same complete Project Authority descriptor:
identity, kind, role, singleton capability, label, canonical path,
requiredness/condition, empty dependencies, lifecycle, intake, singleton
renderer, empty Projections/overlays/extensions, and present package-owned
subordinate definitions. `validate_selected_profile` remains 1.1-only and
unchanged.

The existing released profile constants are now `pub(crate)` without value
changes. The exact intake, lifecycle result, candidate-currentness,
promotion-candidate, and transaction-preflight branches use those constants
after registry validation. Thus generic selected decisions may be 1.2 while
all Charter records and comparisons retain the released HCM-2.2 pair.

No public signature/type, schema, vector, Cargo/dependency, package, lineage,
lifecycle, approval, promotion-intent, recovery/replay, or committed-authority
format changed.

## TDD evidence

The new
`crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs` target first
failed because:

- exact 1.2 decisions were rejected as unsupported;
- an unreviewed profile tuple reached Charter schema serialization instead of
  refusing at the authority boundary;
- successor authoring failed the direct 1.1 lifecycle gate;
- successor approval rejected the released candidate pair as stale; and
- promotion/transaction private preflight still compared records with generic
  decisions.

After the bounded implementation and discovery remediation, 8/8 tests pass.
They prove:

- exact 1.1/1.2 tuple admission and unreviewed-tuple refusal;
- fail-closed rejection of prefix/range selectors, crossed built-in identity,
  package-owned repository rebinding, a second source for the selected ref,
  and a near-version/source mismatch;
- a fresh second-read refusal matrix for every Project Authority descriptor
  field, subordinate-definition cardinality, and the selected profile
  fingerprint, paired with exact validator-predicate guards;
- byte-identical intake/candidate production with the released pair;
- same-repository 1.1-to-1.2 authoring replay with identical refs and zero-byte
  mutation;
- successor approval currentness over the released candidate pair;
- a real successor-authored native approval, promotion, transaction commit,
  and committed-authority readback; and
- exact private promotion/transaction selector posture.

## Packet proof wall

Engine wall:

```text
cargo test -q -p handbook-engine \
  --test hcm_2_4_charter_profile_compatibility \
  --test hcm_2_2_approval_use \
  --test hcm_2_2_authenticator_security \
  --test hcm_2_2_authority_repair \
  --test hcm_2_2_authority_workflow \
  --test hcm_2_2_charter \
  --test hcm_2_2_charter_observation \
  --test hcm_2_2_definition_profile \
  --test hcm_2_2_definition_registry \
  --test hcm_2_2_intake \
  --test hcm_2_2_lifecycle \
  --test hcm_2_2_lifecycle_store \
  --test hcm_2_2_lineage_store \
  --test hcm_2_2_promotion_workflow \
  --test hcm_2_2_runtime_vectors \
  --test hcm_2_2_test_surface \
  --test hcm_2_2_transaction_promotion
```

Result: exit 0; 97 passed, 0 failed.

Read-only compiler and CLI anchors:

```text
cargo test -q -p handbook-compiler \
  --test doctor \
  --test hcm_2_2_c04_version \
  --test hcm_2_2_product_cutover

cargo test -q -p handbook-cli \
  --test hcm_2_2_product_cutover \
  --test hcm_2_2_skill_assets \
  --test cli_surface
```

Result: exit 0; compiler 11/11 and CLI 108/108. Combined packet wall:
216 passed, 0 failed.

Hygiene and preservation:

```text
cargo fmt --all -- --check
git diff --check
git diff --exit-code -- \
  crates/compiler/src/doctor.rs \
  crates/engine/src/charter_lineage_store.rs \
  crates/engine/src/charter_promotion_intent_v12.rs \
  docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts
npx gitnexus detect-changes --scope unstaged --repo handbook --limit 200
```

Result: all commands exit 0. GitNexus 1.6.9 reproducibly reports the aggregate
open HCM-2.4 tracked delta as LOW, 16 files, and zero affected processes.
Changed-symbol enumeration is not stable across identical invocations and is
therefore not used as an exact proof count. Its changed-file result excludes
untracked definitions, tests, proof, and dispatches, which are bound separately
by review manifests.

## Independent discovery and remediation

Fresh isolated discovery review
`20260726T213335Z--HCM-2-4--p1b-charter-compatibility-review` returned two
P2 findings:

1. promotion construction still emitted the selected decisions' profile pair,
   which would emit 1.2 after P1C and conflict with the released transaction
   membrane; and
2. the negative proof did not independently exercise selector rebinding,
   near-version/exactness, descriptor-field, subordinate-cardinality, and
   second-read refusals.

Both findings were validated. The promotion record now emits
`SELECTED_PROFILE_REF` and `SELECTED_PROFILE_FINGERPRINT`; a source guard fails
if either generic decisions accessor returns to that construction. The public
selection and second-read matrix above closes the negative proof gap without
adding a production symbol, changing a released schema, or authorizing broader
profile inference.

Different-fresh closure reviewer
`/root/hcm_2_4_p1b_remediation_closure` replayed all 13 entries at
`sha256:ce1c35b3a2bda3121667c1765dce49ad49b827e6dbcaced72d75da7f041daa32`,
the 216-test wall, hygiene, caller ceiling, and live-selection guard. Verdict:
CLEAN with no P1/P2. Its one P3 observed that GitNexus changed-symbol
enumeration varied across identical runs; the proof now retains only the
stable LOW/16-files/zero-process facts. No P3/P4 remains to register in `09`.

## Proof boundary

P1B proves the compatibility membrane while the live shipped resolver remains
1.1. The real successor-authored path crosses intake, lifecycle, approval,
promotion, transaction preflight, and committed-authority readback without
rewriting the released Charter pair.

P1B does not select shipped-root 1.2, change any released HCM-2.2 format, or
authorize P1C/P2-P7 behavior. P1C owns the one shipped-request selection change
and must replay this entire wall on the actual live 1.2 resolver path.
