# HCM-2.2 Planning Review 16 Remediation

- Recorded at: `2026-07-19T22:05:06Z`
- Review dispatch: `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T214115Z--HCM-2-2--fresh-planning-review-16.json`
- Reviewed subject: 61 exact paths, aggregate `sha256:b28e278f66f7f71973ed8699b8e6ff0dd10fdfef927f0a8cbe054dc377060e37`
- Fresh reviewer: `/root/hcm_2_2_planning_review_16`
- Review verdict: `CHANGES_REQUIRED`
- Commit authorization: none

## Required findings accepted

1. The approval definition's semantic body carried the current CTAP status-
   fixture hash while its resolved dependency retained the prior producer hash,
   so the closure reproduced a stale dependency rather than proving producer
   equality.
2. The approval challenge branch independently admitted both operation values
   and both basis shapes, allowing initial approval with a non-null basis and
   amendment approval with null.
3. The complete runtime-record vectors self-hashed but retained friendly or
   mismatched cross-record refs and fingerprints, so they did not satisfy the
   four target/ref/fingerprint/byte equalities required for admission.

## Remediation

- Rebound the CTAP status dependency to the exact current raw producer hash
  `sha256:1ea8ebafb6f92ffd7fc7de32788da85520d7791dcc362b4843df5840c7dd70c5`
  and added direct current-producer equality to the normative closure gate.
- Split `handbook.schemas.security.authenticator-challenge@1.0.0` into three
  closed branches: registry mutation, initial approval with literal null basis,
  and amendment approval with a lowercase SHA-256 basis. Added two explicit
  crossed-operation pre-native-call negative documents and re-signed the
  changed approval transcripts.
- Rebuilt the runtime fixture as one coherent initial-approval lineage and
  replaced admitted friendly refs with exact partition refs. Published and
  reproduced 17 explicit cross-record bindings across candidate/intake,
  approval, promotion, registry, ceremony, and lifecycle targets.
- Recomputed the changed challenge schema entry and the full approval, waiver,
  lifecycle, kind, intake, and profile closure cascade. The challenge document
  hash is `sha256:8cb47ee9ca4b975704f782900c3850d29e7bdd6719af193ab0726f37ec311c29`;
  its registry-entry fingerprint is
  `sha256:0cf273fe522c267fa791c879a312805ca84be80b5568b3bacff599c555e21cb8`.
  The resulting approval and profile heads are
  `sha256:67466e0c26e48e7e25705b6a601e487b74923371d886d759fcd9face2e793c29`
  and `sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74`.

## Targeted replay

The three closed challenge branches validate every positive ceremony and
reject both crossed basis documents. All fourteen runtime records independently
recompute their fingerprint and exact ID; all 17 published bindings reproduce
the ref basename, target ID, declared target fingerprint when present, and
target preimage bytes. The admitted approval, candidate, and authenticator
assertion now share one exact initial-create null basis, candidate identity,
registry state, and transition head. All five assertion signatures across the
seven ceremonies still verify against the registry-resolved P-256 key.

## Proof replay

The complete wall passed after remediation: eleven literal schema entries and
eleven current-producer-closed definition/JCS vectors; fourteen runtime records
with seven published-schema validations and 17 exact cross-record bindings;
thirteen Admin positives and seventeen negatives; seven positive ceremonies,
two crossed challenge-schema negatives, five raw assertion successes, eighteen
signed/raw negatives, and four credential-selection cases; all 33 security
bounds; all 255 received statuses; final-use/waiver/user-handle/profile-source/
security-source gates; and every registry-resolved ES256 assertion. Handoff
validation passed 45 records, 199 current internal dispatches, eight legacy
dispatches, and 45 ledger rows in all three modes. Focused Cargo baselines
passed `1`, `8`, `49`, `14`, `17`, and `22` tests. Exact scope contained 63
paths; 24 Markdown files and 67 relative links passed; the future todo remained
entirely unchecked. No product or definition implementation was executed.

This replay does not authorize a commit. The parent must dispatch the expanded
exact-path subject to a new fresh independent reviewer.
