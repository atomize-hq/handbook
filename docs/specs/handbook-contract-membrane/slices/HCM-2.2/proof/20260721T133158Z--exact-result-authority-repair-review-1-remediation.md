# HCM-2.2 exact-result authority-repair Review 1 remediation

**Proof ID:** `HCM-2.2-ERAR-R1-REMEDIATION`  
**Recorded:** `2026-07-21T13:31:58Z`  
**Authority class:** additive remediation proof; no implementation authority  
**Review subject:**
[`20260721T124756Z--HCM-2-2--fresh-exact-result-authority-repair-review-1.json`](../../../handoffs/dispatches/20260721T124756Z--HCM-2-2--fresh-exact-result-authority-repair-review-1.json)

## Disposition

The fresh isolated Review 1 returned `CHANGES_REQUIRED`. Every finding below is
accepted without waiver. The Review 1 dispatch and its reviewed bytes remain
immutable; this proof records an additive remediation and the next review uses
a new exact subject manifest.

| Finding | Severity | Accepted disposition |
|---|---|---|
| `HCM-2.2-ERAR-R1-001` | required | Replace every selected candidate-`1.3` create/amend approval, promotion, transition, and intent authority artifact; retain candidate-`1.2` values only inside the explicitly historical vector. |
| `HCM-2.2-ERAR-R1-002` | required | Replace the unbounded “complete inventory” abstraction with exact roots, name grammars, count/byte ceilings, two stable no-follow scans, and boundary vectors. |
| `HCM-2.2-ERAR-R1-003` | nit | Preserve the research note's original bytes externally, then replace its rejected-worktree-only link with a local immutable stop-proof link and hash-identify the rejected Rust source. |

## Preservation correction

The initial preservation pass covered every tracked and ordinary untracked
path and inventoried ignored paths. Before remediation, preservation was
extended to include every ignored path as bytes as well. The resulting closed
inventory contains 1,464 tracked paths, 13 ordinary untracked paths, and
25,235 ignored untracked paths: 26,712 paths total.

- complete manifest:
  `C:\Users\spmcc\Documents\__Project_Code\_hcm22ar_preservation\486458acfe8373a977e595a4854ac786166e3e76-20260721T121830Z\complete-manifest.json`
- manifest SHA-256:
  `cbcb29fe7ef50184cbc2efcaa9489d07c99d0d4b51c7e92836e7e2cc3d1cf44a`
- complete archive:
  `C:\Users\spmcc\Documents\__Project_Code\_hcm22ar_preservation\486458acfe8373a977e595a4854ac786166e3e76-20260721T121830Z\hcm22ar-complete-worktree-snapshot.tar`
- archive byte length: `30,789,857,280`
- archive SHA-256:
  `46dbed286dba9b3b6cedf7ef13340c43dcd56e068a5efa9af1deba0cabadd33d`
- extraction root: the sibling `verified-complete-extract/` directory
- equality result: all 26,712 extracted files matched their source byte
  length and SHA-256, and the source inventory remained stable

The rejected subject at `C:\hcm22ar` remained at
`486458acfe8373a977e595a4854ac786166e3e76`. No path there was modified,
cleaned, reset, staged, committed, or discarded.

## `HCM-2.2-ERAR-R1-001` — fresh authority lineage

Candidate `1.3` create and amendment retain their independently recomputed
candidate-subject, semantic-result, exact-result document, and final-candidate
identities. All candidate-dependent authority is newly derived from the final
candidate `1.3` fingerprint:

| Chain | Candidate subject | Result semantic fingerprint | Result exact JCS+LF | Final candidate |
|---|---|---|---|---|
| create | `sha256:846ddf4906a093f26472cefc0e671587dd7da1e08bb781efd02cea96f3640281` | `sha256:e10efe2f45b808d48b3b399649c9e585727b56ea793a1948894d4265d032d9b8` | `sha256:c6696b0f9e8a8e3d1acf3fe916a3e18922869bf2cb9b2bee5457955a9488d538` / 4,334 bytes | `sha256:f2e8c3736c39c38e2f1b41bbbac7cdd839ee1fc954e77fe15e4909841cda4961` |
| amendment | `sha256:8c87c62c7e95fafa12a307eadce3505d7974d97a80ea347f01fd0d1337b54747` | `sha256:08f67bbe6634d34aeaa07012353c50e73364079237036ab68413a8b0456a2aad` | `sha256:ce59629275937486506b3cc532bac5005a1f3ee93a7b0c4b277fc0d262b1fe0f` / 5,009 bytes | `sha256:723aa44c3a906511f6dc8c08984b99e89e581c5303e125b602059ab1bf0ffe99` |

The new create response/assertion/approval/promotion fingerprints are,
respectively,
`sha256:e706b48812252b4f155dc0cb4f9ba0a7c9904689d171a6f682e527c7b576d228`,
`sha256:2401b384b7fe9fce718095147b1612ff00eef030ffa4794c4dbe27f8181a7d2a`,
`sha256:ff88731faf9b327da39f9330dd71598ec22dd972f9126822b202db9d035e12f2`,
and
`sha256:8a1e5d73ac0163f4eb0acc53d39dbb4c7a027c5c2cae0cf3d80fd249733e1e63`.

The new amendment response/assertion/approval/promotion/clearance-transition
fingerprints are, respectively,
`sha256:3ac9067f11c034ffbfd5c76d727b1dc833e2bccefa53b986f8a74b7da7aa19d6`,
`sha256:6078c9eeb4da6b62091ac5c2d4c1a806366be8958b30f45c208373ec636f8cce`,
`sha256:eb437b1d85559a73cf77d5e0be2ec3ddb6aea3ef78884e58ecce03fc72d8f26f`,
`sha256:0385ccc7138543dbbd598bf43b947e881ddf0f3869dc2af784cae0356072b038`,
and
`sha256:5ece901327bae501487cf36c834bcc73f04601360950b1ba3cb039823de4ead7`.
The selected intent `1.2` fingerprint is
`sha256:0df781964c952255ebc273e91753dca68a22562b7fd46198e6725524f94ba3f6`;
its exact 7,591-byte JCS+LF document SHA-256 is
`sha256:1fdddef8cd61f67c024d5f24fc0dd169f1aca2d87045efe1814f2ef54f374076`,
and its marker remains 72 ASCII bytes including LF.

The signatures were independently verified against the registry's exact COSE
P-256 public key. Each raw CTAP response decodes to the published credential,
authenticator data, and DER signature; challenge JCS, client-data hash, RP ID
hash, flags, sign count, signed-preimage digest, and signature all match. The
candidate-`1.2` intent and approvals remain only in the named historical
contract and are forbidden as selected-product fallback.

## `HCM-2.2-ERAR-R1-002` — bounded stable discovery

The author lock now covers two exact roots:
`.handbook/evidence/charter/candidates/` and
`.handbook/evidence/charter/lifecycle-validation-results/`. Each root admits at
most 4,096 immediate entries, each regular file at most 262,144 bytes, at most
1,073,741,824 aggregate bytes, and at most 128 UTF-8 filename bytes. Candidate
filenames are exactly 79 ASCII bytes matching
`candidate_[0-9a-f]{64}.json`; result filenames are exactly 97 ASCII bytes
matching `lifecycle-validation-result_[0-9a-f]{64}.json`.

Every immediate entry is enumerated without glob filtering, opened no-follow,
boundedly loaded, duplicate-key rejected, closed-schema validated, and assigned
an independently recomputed identity. Unowned, malformed, nested, nonregular,
link, or reparse entries refuse even when historical or nonmatching. Safe
candidate `1.0`/`1.1`/`1.2`, safe nonmatching candidate `1.3`, and safe
nonexpected result records remain preserved nonmatches.

Two complete scans under the same author lock must have identical root identity
and sorted `(raw filename bytes, file identity, byte length, SHA-256)` tuples.
Any count, byte, name, arithmetic, root, entry, or second-scan mismatch refuses
before timestamp allocation or mutation and preserves every entry. Eighteen
boundary vectors cover below/at/above count, file bytes, aggregate bytes, and
filename bytes, plus unsafe historical, malformed unrelated, and equal/changed
stable scans.

## `HCM-2.2-ERAR-R1-003` — research provenance

The non-authoritative research note's pre-repair bytes are recoverable from the
complete archive with SHA-256
`b65ec0dd8a6cd9960b821fc08e0843045ad3559d77534a3c080123438f3aa5bd`.
Its broken link now targets the imported immutable Review 3 stop proof's RED
reproduction. The rejected source remains at
`C:\hcm22ar\crates\engine\tests\hcm_2_2_authority_repair.rs` and is identified
without copying or editing it by SHA-256
`a412165c1be6866fdf24f64abc3f35bcfd79fd446e53072ba527b2334ff846d3`.

## Remediation checks

- all 22 runtime record fingerprints and IDs recomputed from their preimages;
- both candidate subject/result/final chains and both exact result documents
  recomputed independently;
- 31 cross-record bindings and the selected intent transitive closure resolved;
- create and amendment ES256 signatures verified against the published key;
- result `1.0`, assertion/response, and intent `1.2` schema instances validated;
- selected and historical intent fingerprints/documents/markers recomputed;
- `git diff --check` passed after enforcing LF-only JSON vector bytes; and
- no Rust, Cargo, or HCM-2.3 path was edited.

This proof does not declare the subject clean. A different fresh isolated
reviewer must review the complete revised manifest and may return `CLEAN` only
with no actionable finding, including no nit.
