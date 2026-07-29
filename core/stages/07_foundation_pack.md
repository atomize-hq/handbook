---
kind: stage
id: stage.07_foundation_pack
version: 0.1.0
title: Foundation Pack Synthesis
work_level: L1
description: 'Synthesizes project-specific foundation artifacts from Charter (+ optional
  Project Context), including a machine-executable quality gates configuration.

  '
includes:
- core/rules/p0_absolute.md
- core/rules/p1_pragmatic.md
- core/rules/traceability_policy.md
- core/rules/evidence_policy.md
- core/runners/${runner}.md
- core/profiles/${profile}/conventions.md
- core/profiles/${profile}/profile.yaml
- core/profiles/${profile}/commands.yaml
inputs:
  library:
  - path: core/library/foundation_pack/foundation_pack_directive.md
    required: true
  - path: core/library/foundation_pack/FOUNDATION_STRATEGY.md.tmpl
    required: true
  - path: core/library/foundation_pack/TECH_ARCH_BRIEF.md.tmpl
    required: true
  - path: core/library/foundation_pack/TEST_STRATEGY_BRIEF.md.tmpl
    required: true
  - path: core/library/foundation_pack/QUALITY_GATES_SPEC.md.tmpl
    required: true
  - path: core/library/foundation_pack/quality_gates.yaml.tmpl
    required: true
  artifacts:
  - path: artifacts/charter/CHARTER.md
    required: true
  - path: artifacts/project_context/PROJECT_CONTEXT.md
    required: false
  variables:
  - runner
  - profile
  - repo_root
  - project_name?
  - owner?
  - team?
  - repo_or_project_ref?
  - charter_ref?
  - project_context_ref?
outputs:
  artifacts:
  - path: artifacts/foundation/FOUNDATION_STRATEGY.md
  - path: artifacts/foundation/TECH_ARCH_BRIEF.md
  - path: artifacts/foundation/TEST_STRATEGY_BRIEF.md
  - path: artifacts/foundation/QUALITY_GATES_SPEC.md
  - path: artifacts/foundation/quality_gates.yaml
gating:
  mode: strict
  fail_on:
  - missing_required_inputs
  - output_missing
  - non_machine_testable_gates
tags:
- foundation
- synthesis
- quality-gates
- machine-testable
---
# core/stages/07_foundation_pack.md

<!--
Stage body intentionally minimal.
The directive + templates define the content.
Orchestrator injects:
- CHARTER.md (+ PROJECT_CONTEXT.md if present)
- selected profile id (or ask the stage to recommend one)
Outputs must be written to artifacts/foundation/.
-->
