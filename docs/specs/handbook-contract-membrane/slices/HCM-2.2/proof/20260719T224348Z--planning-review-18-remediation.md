# HCM-2.2 Planning Review 18 Remediation

- Recorded at: `2026-07-19T22:43:48Z`
- Review dispatch: `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T222931Z--HCM-2-2--fresh-planning-review-18.json`
- Reviewed subject: 65 exact paths, aggregate `sha256:351e2cf8905340373bf97f193223994e0a318fb8bac83f7b1535949723bf12be`
- Fresh reviewer: `/root/hcm_2_2_planning_review_18`
- Review verdict: `CHANGES_REQUIRED`
- Commit authorization: none

## Required finding accepted

The initial and amendment approval vectors carried current challenge JCS hashes
and signatures but their command-`0x02` GetAssertion request CBOR key `2` still
contained the earlier challenge hashes. A native authenticator would therefore
sign different bytes from the published responses.

## Remediation

- Rebuilt all five GetAssertion request blobs from each vector's exact current
  challenge JCS SHA-256, ordered credential descriptor, RP ID, and `up`/`uv`
  options using the packet's canonical CBOR encoder.
- Added explicit request-key-`2` equality to the future implementation plan and
  todo proof obligations.
- Corrected the prior remediation wording to identify the one initial-approval
  assertion that was re-signed during Review 17 remediation.

## Targeted replay

All five command-`0x02` request blobs independently decode/re-encode to the
exact map `{1: "handbook.local", 2: <current clientDataHash>, 3:
<ordered exact allow-list>, 5: {up: true, uv: true}}`. Each key `2` equals
SHA-256 of that vector's literal challenge JCS UTF-8, and every published DER
ES256 response signature continues to verify over
`authenticatorData || clientDataHash` under the registry-resolved P-256 key.

## Proof replay

The complete wall passed after remediation. All five exact current GetAssertion
request hashes, seven ceremony JCS values, five response CBOR values, and five
DER ES256 signatures passed together with eleven literal schema entries,
eleven current-producer-closed definitions, fourteen runtime records/seven
schema validations, 17 internal and 17 external bindings, 33-group/52-pointer
security coverage, all Admin/status/raw-response/selection/profile/source/
final-use/waiver/user-handle gates, and all handoff modes. Handoff validation
passed 45 records, 201 current dispatches, eight legacy dispatches, and 45
ledger rows. Focused Cargo baselines passed `1`, `8`, `49`, `14`, `17`, and
`22` tests. Exact scope contained 67 paths; 26 Markdown files and 69 relative
links passed; the future todo remained entirely unchecked. No product or
definition implementation was executed.

This replay does not authorize a commit. The parent must dispatch the expanded
exact-path subject to a new fresh independent reviewer.
