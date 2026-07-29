---
kind: stage
id: stage.10_feature_spec
version: 0.1.0
title: "Work Specification"
work_level: L1
description: >
  Produces descriptor-selected canonical Work Specification YAML and a deterministic Markdown review view.

includes:
  - core/rules/p0_absolute.md
  - core/rules/p1_pragmatic.md
  - core/rules/traceability_policy.md
  - core/rules/evidence_policy.md
  - core/runners/${runner}.md
  - core/profiles/${profile}/profile.yaml
  - core/profiles/${profile}/commands.yaml
  - core/profiles/${profile}/conventions.md

inputs:
  library:
    - path: core/library/feature_spec/feature_spec_architect_directive.md
      required: true
    - path: core/library/feature_spec/FEATURE_SPEC.md.tmpl
      required: true
  artifacts:
    - path: artifacts/base/BASE_CONTEXT.md
      required: true
    - path: artifacts/charter/CHARTER.md
      required: true
    - path: artifacts/project_context/PROJECT_CONTEXT.md
      required: false
    - path: artifacts/foundation/FOUNDATION_STRATEGY.md
      required: false
    - path: artifacts/foundation/TECH_ARCH_BRIEF.md
      required: false
    - path: artifacts/foundation/TEST_STRATEGY_BRIEF.md
      required: false
    - path: artifacts/foundation/QUALITY_GATES_SPEC.md
      required: false
    - path: artifacts/foundation/quality_gates.yaml
      required: false
  variables:
    - runner
    - profile
    - repo_root
    - now_utc
    - project_name?
    - owner?
    - team?
    - repo_or_project_ref?
    - charter_ref?

outputs:
  artifacts:
    - path: artifacts/work-specification/work-specification.yaml
  repo_files:
    - path: ${repo_root}/artifacts/feature_spec/FEATURE_SPEC.md
      required: true

gating:
  mode: strict
  fail_on:
    - missing_required_inputs
    - output_missing
  notes:
    - Capture input must be ONLY one completed Work Specification YAML document (no wrappers or commentary).
    - The Markdown view is derived deterministically during capture and is never authoritative.

tags:
  - work-specification
  - planning
---

# core/stages/10_feature_spec.md

<!--
Stage body intentionally minimal.
The directive + template define the content.
-->
