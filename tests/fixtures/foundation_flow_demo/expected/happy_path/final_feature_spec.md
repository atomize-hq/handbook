# Work Specification

## Objective

Build the M4 proof wedge for one pipeline\.foundation\_inputs journey using staged external model output\, canonical Work Specification YAML capture\, and a deterministic Markdown view\. A credible alternative is to retain Markdown as Stage 10 authority\; that is simpler for manual editing but forfeits schema\-selected validation\, stable canonical identity\, and byte\-exact handoff provenance\.

## Scope

- G1\: Prove a believable happy path that reaches stage 10 only after stage 06 and stage 07 complete\.
- G2\: Prove a believable skip path that leaves stage 06 skipped because both activation predicates are false\.
- G3\: Lock docs and proof to the same canonical Work Specification handoff contract\.

## Non-Goals

_None._

## Acceptance Criteria

- AC\-001\: A CLI happy\-path test resolves\, captures stages 04\/05\/06\/07\, compiles stage 10\, captures one completed Work Specification YAML document\, and writes the canonical YAML plus deterministic Markdown view\.
- AC\-002\: The happy\-path canonical Work Specification YAML exactly matches the admitted external model output and its Markdown view exactly matches the committed renderer golden\.
- AC\-003\: A CLI skip\-path test proves stage 06 is skipped because needs\_project\_context\=false and charter\_gaps\_detected\=false\.
- AC\-004\: No stage\-10 success\-path test captures raw compile payload\.
- AC\-005\: Docs and proof drift checks fail if stage 10 is described as direct compile\-to\-capture\.

## Status

approved
