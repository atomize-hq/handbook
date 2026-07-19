# HCM-2.2 Planning Review 17 Remediation

- Recorded at: `2026-07-19T22:28:02Z`
- Review dispatch: `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T220642Z--HCM-2-2--fresh-planning-review-17.json`
- Reviewed subject: 63 exact paths, aggregate `sha256:67e4fe30e2504c5653a67ab67f3fb2e1464cd3167b4dd567c5b80d210eec9d93`
- Fresh reviewer: `/root/hcm_2_2_planning_review_17`
- Review verdict: `CHANGES_REQUIRED`
- Commit authorization: none

## Required findings accepted

1. The exhaustive security-bound fixture omitted the amendment-approval
   branch's waiver-array and nonce pointers after the challenge schema split.
2. The SPEC retained the pre-cascade lifecycle head and pre-update literal
   Charter-intake raw SHA even though both current producers had changed.
3. Complete admitted runtime rows retained placeholder external definition,
   profile, lifecycle-policy, trigger, and kind fingerprints without a direct
   current-producer equality table.

The Nit was also accepted: Review 16's targeted replay counted seven assertion
signatures even though the seven ceremonies contain two registrations and five
assertions.

## Remediation

- Added `/oneOf/2/properties/accepted_waiver_fingerprints` and
  `/oneOf/2/properties/nonce_base64` to the existing security-bound groups. The
  exact fixture remains 33 groups and now equals all 52 bounded schema pointers.
- Updated the normative lifecycle head to
  `sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3`
  and the 10,009-byte intake raw hash to
  `c845cd7012764645965bc54cabc66e65c91dac468b2f80123f4c62be31565f6a`.
- Rebuilt the runtime lineage from the current shipped-profile producer through
  intake, candidate, initial-approval challenge/assertion, approval, waiver,
  promotion, and lifecycle records. The affected initial-approval assertion was
  re-signed when its challenge changed.
- Published 17 exact external reference bindings covering every
  `handbook.*` definition, profile, policy, trigger, and schema ref in the
  fourteen complete runtime rows. Every binding resolves directly to the
  current definition or schema-entry producer and checks the retained in-record
  fingerprint where one exists.
- Corrected the Review 16 proof to say five assertion signatures across seven
  ceremonies.

## Targeted replay

Exact set equality now holds between all 52 bounded security-schema pointers and
all fixture pointers. All fourteen runtime records reproduce their IDs and
fingerprints; all 17 content-addressed record bindings and all 17 external
current-producer bindings pass. The external table covers every `handbook.*`
ref present in the runtime records with no missing or extra row. The SPEC's
closure table equals the current definition vectors and its intake literal
hash equals the current file bytes.

## Proof replay

The complete wall passed after remediation: all eleven literal schema entries
and eleven current-producer-closed definitions; fourteen runtime records with
seven published-schema validations, 17 internal record bindings, and 17 external
current-producer bindings; exact 33-group/52-pointer security-bound coverage;
thirteen Admin positives and seventeen negatives; seven ceremonies, two crossed
challenge negatives, five raw assertion successes, eighteen signed/raw
negatives, and four selection cases; all 255 received statuses; all profile/
security sources; and final-use/waiver/user-handle/signature gates. Handoff
validation passed 45 records, 200 current dispatches, eight legacy dispatches,
and 45 ledger rows in all three modes. Focused Cargo baselines passed `1`, `8`,
`49`, `14`, `17`, and `22` tests. Exact scope contained 65 paths; 25 Markdown
files and 68 relative links passed; the future todo remained entirely unchecked.
No product or definition implementation was executed.

This replay does not authorize a commit. The parent must dispatch the expanded
exact-path subject to a new fresh independent reviewer.
