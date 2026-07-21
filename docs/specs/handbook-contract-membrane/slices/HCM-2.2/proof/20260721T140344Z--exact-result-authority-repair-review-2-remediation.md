# HCM-2.2 exact-result authority-repair Review 2 remediation

**Proof ID:** `HCM-2.2-ERAR-R2-REMEDIATION`  
**Recorded:** `2026-07-21T14:03:44Z`  
**Authority class:** additive remediation proof; no implementation authority  
**Review subject:**
[`20260721T133346Z--HCM-2-2--fresh-exact-result-authority-repair-review-2.json`](../../../handoffs/dispatches/20260721T133346Z--HCM-2-2--fresh-exact-result-authority-repair-review-2.json)

## Disposition

Fresh isolated Review 2 returned `CHANGES_REQUIRED` with four Required
findings. Every finding is accepted without waiver. The Review 1/2 dispatches,
their exact manifests, and both earlier planning/remediation proofs remain
byte-immutable historical evidence. This additive proof supersedes only their
incorrect current-result-root, selected-intent, and boundary-coverage claims.

| Finding | Accepted repair |
|---|---|
| `HCM-2.2-ERAR-R2-001` | Align author discovery with the unchanged authoritative result store `.handbook/state/lifecycle-validation-results/`; prohibit an evidence-root alias. |
| `HCM-2.2-ERAR-R2-002` | Bind selected intent outputs to independently recomputed promotion and lifecycle-transition JCS+LF digests, then recompute the intent fingerprint/document/marker. |
| `HCM-2.2-ERAR-R2-003` | Allocate selected candidate `1.3` transaction `promotion-transaction_0000000000000003`, distinct from retained historical candidate `1.2` transaction `...0002`. |
| `HCM-2.2-ERAR-R2-004` | Add mirrored filename-below and exact 79/97-byte owned-name positives; correct the inventory matrix from 18 to 21 rows. |

## Authoritative result-store alignment

The exact author inventory roots are now:

- candidate authority:
  `.handbook/evidence/charter/candidates/`; and
- lifecycle-validation result authority:
  `.handbook/state/lifecycle-validation-results/`.

The latter is exactly the directory parent of unchanged result `1.0`
`store_path_rule`. Semantic refs resolve to the same result basename in that
store. There is no result store, alias, fallback, discovery, or orphan source
under `.handbook/evidence/charter/lifecycle-validation-results/`.

The zero/no-result, exactly-one/exact-result, more-than-one, result-orphan, and
candidate-missing-result classifications therefore operate on the same bytes
that result publication and replay resolve. The two stable no-follow scans,
all count/byte ceilings, duplicate-key and closed-schema checks, and
preserve-before-mutation outcomes remain unchanged.

## Exact selected intent outputs and namespace

Independent RFC 8785 JCS+LF materialization of the fresh runtime amendment
records yields:

| Output | Runtime fingerprint | Exact JCS+LF byte length | Exact JCS+LF SHA-256 |
|---|---|---:|---|
| promotion | `sha256:0385ccc7138543dbbd598bf43b947e881ddf0f3869dc2af784cae0356072b038` | 2,068 | `sha256:e8bd5b7e12f5c05e49c5964771b4fa6a87791bc2769eb66f40539bd557b7701c` |
| lifecycle transition | `sha256:5ece901327bae501487cf36c834bcc73f04601360950b1ba3cb039823de4ead7` | 978 | `sha256:373767a345b92e5bd1e6422a7e5dbd6111877660df7b8c4af79d8c5a0a832a22` |

The selected intent binds those exact values. Its engine-allocated transaction
ID is `promotion-transaction_0000000000000003`; every writer, discovery,
recovery, and finalization path derives only
`.handbook/state/transactions/promotions/promotion-transaction_0000000000000003.pending/`.
Historical candidate `1.2` remains byte-immutable at transaction
`promotion-transaction_0000000000000002` and its distinct pending namespace.
No transaction ID or pending-directory carry-forward is allowed.

After these corrections, the selected intent `1.2` values are:

- fingerprint:
  `sha256:a6743e191fc72bee7b2a1a54e85b957a242f019033992d595941663392dd9037`;
- exact JCS+LF byte length: `7,591`;
- exact JCS+LF SHA-256:
  `sha256:1f93f81c116f546fd026a07c3d11d79ff8441d273e4259e01f166cc0122a4d38`;
- marker: the exact 71-byte digest string plus one LF, 72 ASCII bytes total.

The result `1.0` and intent `1.2` schemas remain byte-identical. Candidate
subjects, semantic results, exact result documents, final candidates,
assertions, approvals, promotions, and lifecycle-transition fingerprints do
not change in this remediation.

## Filename and store validation closure

Both normative vector mirrors now contain 21 byte-identical inventory rows:

- below/at/above candidate entry count;
- below/at/above result entry count;
- below/at/above per-file bytes;
- below/at/above aggregate bytes;
- below/at/above filename bytes;
- admitted exact 79-byte candidate owned name;
- admitted exact 97-byte result owned name;
- unsafe historical candidate;
- malformed unrelated result; and
- equal/changed stable scans.

The 127-byte below-limit unowned name still refuses: satisfying the byte ceiling
does not confer ownership. The 128-byte unowned and 129-byte oversized names
also refuse. Only the two exact ASCII grammars are admitted.

## Remediation checks

- result inventory root equals the parent of result `1.0` persisted paths;
- selected and historical transaction IDs and pending directories are distinct;
- selected output refs/fingerprints/digests/lengths resolve to the exact runtime
  records;
- selected intent fingerprint, 7,591-byte document digest, and marker replay;
- both vector mirrors have the same 21 unique inventory rows, including one
  filename below/at/above row and both exact owned-name positives;
- all 22 runtime record, candidate subject/result/final, exact-result, approval,
  promotion, transition, cross-record, schema, and ES256 checks remain green;
- `git diff --check`, documentation-only scope, immutable historical evidence,
  and the original worktree preservation boundary remain green.

This proof does not declare the subject clean. A third different fresh isolated
reviewer must replay the complete revised manifest and return `CLEAN` with no
Critical, Required, Optional, or Nit finding before any commit.
