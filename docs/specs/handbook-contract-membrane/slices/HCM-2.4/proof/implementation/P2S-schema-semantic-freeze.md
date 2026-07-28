# P2S schema and semantic freeze proof

Status: one consolidated discovery-review remediation complete;
different-fresh immutable closure review is the packet closeout gate.

## Authority and boundary

- Authority: human-selected HCM-2.4 P2S only, sourced from completed planning
  handoff
  `20260727T215133Z--HCM-2-4--orchestration--p2-dual-ceiling-planning-completed`.
- Reviewed planning commit:
  `68ea22e11920b5d25262af8bdae0b067d689b587`.
- Reviewed planning fingerprint:
  `sha256:0108c07a1e32cabebe8c9abc509c83d60fd2cfb2488abc14e056a701e821ff53`.
- Implementation baseline:
  `78681ddd5a672b874ef38c4e754598f8288ca144`.
- Scope: eight additive Draft 2020-12 schemas, one additive evaluator
  definition, and one schema/definition integration test.
- Excluded and unchanged: `crates/engine/src`, all Cargo files, all released
  schemas and definitions, public APIs, runtime behavior, registration, and
  every existing fingerprint.

GitNexus was rebuilt at the implementation baseline. This additive
definition/test packet changes no existing production symbol, has no direct
caller or execution-flow blast radius, and therefore has no existing symbol
target on which an upstream impact query can operate. Registration would
require a separately authorized production change, so these refs remain
deliberately unregistered.

## TDD wall

RED was established before the definitions existed:

```text
cargo test -p handbook-engine --test hcm_2_4_project_condition_evidence_schemas
error: couldn't read each of the eight schema paths and the evaluator-definition path
```

GREEN:

```text
cargo test -p handbook-engine --test hcm_2_4_project_condition_evidence_schemas -- --nocapture
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All eight documents also pass `jsonschema.Draft202012Validator.check_schema`.

## Frozen artifacts

The schema set is exactly:

1. `handbook.schemas.project-condition-admitted-evidence-source@1.0.0`;
2. `handbook.schemas.project-condition-evidence@1.0.0`;
3. `handbook.schemas.project-condition-evidence-head@1.0.0`;
4. `handbook.schemas.project-condition-evidence-transaction@1.0.0`;
5. `handbook.schemas.project-condition-evidence-challenge@1.0.0`;
6. `handbook.schemas.project-condition-evidence-assertion@1.0.0`;
7. `handbook.schemas.project-condition-evidence-use-transition@1.0.0`; and
8. `handbook.schemas.project-condition-evidence-evaluation-closure@1.0.0`.

The evaluator is exactly
`handbook.condition-evaluator.managed-operational-surface@1.0.0`.
Its ordinary uniform definition fingerprint replays as:

```text
sha256:ca19395c8ef27aa353dc5918c43b6f3522ee365e204f25ba66f2a22aa793f105
```

The released condition pair embedded in the definition is:

```text
handbook.condition.project.managed-operational-surface@1.0.0
sha256:2ae25788c7860f3062f30659a7674c2ccd8f56b0f8809f1134003e04dea20b61
```

## Semantic proof matrix

| Contract surface | Executable proof |
| --- | --- |
| Draft, exact required keys, root/nested closure, unknown-field refusal | every schema accepts its exact fixture; removal of every root and distinct nested-object required field and injection at root and nested rows is refused |
| Constants, enums, regexes, integer and collection bounds | a frozen 197-row semantic-keyword inventory covers every `const`, `enum`, `pattern`, minimum, maximum, and length/count bound; every distinct regex has positive/negative operation vectors and every enum admits every member while rejecting an outside value |
| Cross-field and nullability rules | positive/non-positive source branches, sequence-one/successor pairs, four ordered closure fingerprint-prefix branches, and all seven outcome/reason pairs are accepted or refused exactly |
| Fingerprints and content-addressed refs | every source, record subject, record, head, transaction, challenge, assertion, use transition, producer verification, closure, and evaluator-definition preimage replays; ref suffixes are checked against fingerprints |
| Freshness | inclusive verified/observed/valid-until boundaries, both not-yet-valid branches, expiry, and the 2592000/2592001-second maximum-validity boundary replay |
| Dependency observations | all six transaction-store rows, retained-byte and absent tuple constraints, complete error enum, exact ordering annotation, and nested closed-key refusal replay |
| Count/byte ceilings | exact 4096/4097 count, Unix 1101/1102, Windows 557/558, exact 131072/131073-byte, long-name, and trap-backed arbitrary-suffix vectors reproduce the complete-envelope fingerprints and prove the offender and later suffix are not retained or requested |
| Evaluator definition | the exact 21-key closed set, every fixed value/ref/list, and the derived definition fingerprint replay; an executable scan proves no production source or Cargo manifest registers any new ref |

Rules that JSON Schema cannot compute by itself—RFC 8785 preimages, UTF-8 byte
length/NFC, temporal ordering, dependency ordering, content-addressed suffix
equality, bounded transaction-directory projection, and first-match
precedence—are frozen as explicit `x-handbook-*` semantic annotations and
proved with replayable test vectors. No runtime consumer is introduced by
P2S.

## Discovery-review remediation

Fresh reviewer `/root/hcm_2_4_p2s_review` replayed the immutable discovery
subject `sha256:fed0f1b588828d9bc21b4fa52baeb686ca125e25dd11365f02d2318f1e2d48a1`
and returned three Required findings. The governing packet review budget
permits one consolidated remediation followed by a different-fresh,
delta-focused closure review. This remediation:

1. excludes every transaction-store-only error from non-store dependency
   observations and adds negative cross-kind vectors;
2. narrows transaction refs to the exact zero-padded 1 through 4096 grammar
   and adds 0000, 4097, 9999, and N−1 semantic vectors; and
3. adds exhaustive distinct nested-required checks, a frozen operational
   pattern/enum/constant/bound inventory, the 2592000/2592001-second validity
   boundary, complete dual-ceiling/early-termination vectors, and an
   executable production-registration absence check.

No new path, production behavior, dependency, or authority entered the
remediation.

## Stop

P2S does not authorize P2A, P2P, a production native adapter, P2R, P2I, P2V,
P4, P6, Phase 2 exit, or another slice. P2A may begin only after this exact
subject passes fresh independent review.
