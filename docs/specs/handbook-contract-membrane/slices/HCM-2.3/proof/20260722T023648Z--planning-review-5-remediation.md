# HCM-2.3 Planning Review 5 Remediation

## Review authority

- reviewer: `/root/hcm_2_3_planning_review_5`
- verdict: `CHANGES_REQUIRED`
- admitted subject: 24 paths, aggregate
  `sha256:873289c98c4ce4a7cdb7d3bc37083afe0513210462a9f7837d2d0466fa17f71d`
- findings: zero Critical, two Required, zero Optional, zero Nit
- disposition: both Required findings were accepted; neither was waived

## Exact dispositions

| Finding | Accepted defect | Remediation |
|---|---|---|
| `HCM-2.3-R5-001` | the operation-context positive's `resolved_definitions` array did not equal the UTF-8 `definition_ref` order required by its own semantic rule | reordered the array to artifact kind, intake, profile, schema; recomputed the complete context fingerprint as `sha256:6247735430f2296162010902e7f6e1e20403f67ab6de7a4e4cc7fcdf9bd466c8`; regenerated every dependent control fingerprint; added an exact sorted-projection admission rule and a recomputed-but-reordered refusal vector |
| `HCM-2.3-R5-002` | all four runtime records bound a placeholder context fingerprint while the operation context, coverage evaluation, and transaction intents bound a derived fingerprint | bound the final derived context fingerprint into intake, validation, candidate, and promotion; regenerated the topological runtime identity/ref/persisted-byte chain and all dependent request, transaction, intent, stage, installed-complete, marker, evidence, result, and ledger identities; added cross-suite equality admission and divergence refusal rules |

## Regenerated normative lineage

The final runtime semantic identities are:

- intake `1.2`:
  `sha256:b88c7d71baafeef02f379be8e317df15d9ad77d6b8cc7599ce3b25a6b69d684c`;
- validation result `1.0`:
  `sha256:f6395dd6b06c2016c37e5393c09fb6ba63bf8fdc60a223d1a1727bccd4357e6e`;
- candidate `1.4`:
  `sha256:dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c`;
  and
- promotion `1.2`:
  `sha256:01047c1c5577160ea1dd253693e1319a874b377095fe9089f2aa064017e5724e`.

Their complete persisted-record JCS+LF digests and lengths are respectively
`sha256:82e97318e65dcec95162ae32ac13dbc78f08e2bf680281e278301e4dc23e6e16`
/ 2,052,
`sha256:f03a701c5fb30f8a70ce9ae63795566439c46d08d9e6416f6b99756ea8954cfd`
/ 1,982,
`sha256:448683872456a2a472b50f442abf9abf3c68b90f6c243b3cfa048e61c037f0f9`
/ 2,221, and
`sha256:f0e8b066370e5e58fe33807445eb3906b4bf657de7fed82b9f0a1efe0aa70817`
/ 1,933. The two value scalars, normalized content, and 67-byte canonical YAML
are unchanged.

These current identities supersede identity and persisted-byte assertions in
the immutable earlier remediation proofs. Those prior proof artifacts remain
unchanged as historical review evidence.

## Verification replay

Independent duplicate-safe Draft 2020-12 and semantic replay reported:

- the operation-context array equals its UTF-8-sorted projection;
- one exact context fingerprint across the operation context, coverage
  evaluation, four runtime records, and three transaction intents;
- all four runtime identities and all 29 control terminal fingerprints
  reproduced;
- intake-to-validation-to-candidate-to-promotion refs and fingerprints agree
  exactly;
- all three transaction IDs reproduce their closed preimages; and
- every staging and installed-complete digest/length tuple equals the newly
  regenerated runtime bytes, through result and retained-ledger closure.

## Scope and next review

The remediation changed only the two HCM-2.3 runtime/control vector artifacts
and this proof record. It did not edit Rust, Cargo, runtime tests, production
assets, HCM-2.2 authority, a handoff ledger, or a preservation location. The
repaired complete documentation subject requires a different fresh isolated
read-only reviewer and remains unauthorized for implementation or commit until
that complete subject is CLEAN.
