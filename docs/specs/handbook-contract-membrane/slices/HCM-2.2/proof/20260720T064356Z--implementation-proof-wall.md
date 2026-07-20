# HCM-2.2 Implementation Proof Wall

> **Terminal status (2026-07-20): non-authoritative blocked checkpoint.**
> This wall records proof obtained before Fresh Review 2. That review returned
> `CHANGES_REQUIRED`, and the bounded Critical remediation proved a frozen
> candidate/validation identity cycle. See
> [`20260720T083550Z--authority-boundary-stop.md`](20260720T083550Z--authority-boundary-stop.md).
> Nothing below is a completion or proof-gate closure claim.

## Subject and authority

- Phase / slice: `HCM-2` / `HCM-2.2`
- Runtime `ACTIVE_PACKET`: `none`
- Selected implementation authority:
  `20260719T230914Z--HCM-2-2--orchestration--implementation-packet-approved`
- Entry branch: `feat/handbook-contract-membrane`
- Entry HEAD: `ab0c2613f215979d9336340e47a2a0e19f00096b`
- Scope: the exact first-party constitutional-root Charter cutover
- Later-slice authority: none; HCM-2.3 was not started

The selected record validated as a completed planning transition and named the
review-clean HCM-2.2 packet as implementation authority. Live HCM-1.1 through
HCM-1.4 and HCM-2.1 code/tests, immutable released definition bytes, and the
selected Project Authority descriptor proved the dependency boundary. All
proof-relevant internal work used immutable current-schema dispatches and fresh
built-in `default` agents; the parent retained integration, verification,
review, commit, and handoff ownership.

## Checkpoint boundary

The implementation adds one closed engine-owned Charter vertical slice:

- 33 additive definition files complete the `1.1.0` Project Authority schema,
  kind, and shipped profile plus exact intake, renderer, lifecycle, approval,
  waiver, trigger, semantic-validator, and security-schema dependencies;
- typed canonical Charter parsing, semantic/capability validation, closed YAML
  emission, deterministic in-memory Markdown rendering, and distinct source /
  rendered-output fingerprints have one owner;
- guided-adaptive, express, and agent-assisted modes evaluate the same 16-item
  intake coverage and emit the same schema-bound immutable candidate form with
  leaf-source provenance and explicit unknown/contradiction/waiver state;
- immutable lineage, native CTAP2.1 registration/assertion authority, bounded
  quorum, approval use, registry mutation/observation, lifecycle reassessment,
  atomic promotion, journal recovery, and retained-authority comparison remain
  engine-owned and fail closed;
- setup alone create-news or byte-preserves the strict no-follow durable
  repository identity, while product operation IDs are allocated by a typed
  engine service and remain stable across request, native boundary, journal,
  result, and output;
- compiler/CLI are thin adapters for the frozen author/approve/promote/validate
  and approver-admin grammar and project exact typed machine/human results;
- installed skills collect explicit acquisition input without a nested model or
  terminal questionnaire and never write selected canonical truth directly;
- doctor advances additively to `1.2.0`; Environment Inventory uses the
  selected Charter descriptor reference; flow/compiler C04 advances exactly to
  `reduced-v1-m8.3` while C03 remains `reduced-v1-m8` generation `1`; and
- legacy Charter input and Markdown have no selected authoring, inspection,
  validation, Environment Inventory, flow, budget, manifest, freshness, log,
  fixture, or installed-skill influence.

Setup remains non-authoring: `.handbook/repository-identity.v1` is operational
identity, not canonical content, and `--reset-state` cannot select or regenerate
it. No persistent renderer output, compatibility importer, hidden inference,
self-approval, nested model call, arbitrary renderer, generic custom-kind
intake, HCM-2.3, Resolution, Projection, Snapshot Memory, posture-kernel, SDK,
Tauri, Substrate, dock, or publication work belongs to the subject.

## Test-first and remediation evidence

RED tests preceded each implementation packet. The parent or a bounded
remediation agent repaired only reproduced failures, then replayed the affected
positive, negative, fail-closed, concurrency, and public-path suites. Material
remediations included complete definition/profile closure, native
authenticator/security boundaries, lineage/promotion/lifecycle atomicity,
approval-use and registry recovery, product authority projection, strict lint,
Windows integration portability, and installed-skill real-path coverage.

The last installed Linux smoke exposed one production defect: a fresh
indeterminate setup did not initialize repository identity, so approver
bootstrap stopped at preflight instead of reaching native authentication. A
focused RED compiler setup test reproduced it. The repair makes every valid-
root setup create or preserve only the operational identity independent of
readiness, still writes no canonical artifact, and leaves reset-state selection
unchanged. Focused compiler/CLI tests passed, then the installed smoke reached
the exact retryable `AUTHENTICATOR_UNAVAILABLE` result with `changed_paths: []`
and an empty before/after repository-byte diff. The only two recovery lock files
were pre-existing, regular, zero-byte, and unchanged.

## Definition and package preservation

- all 29 definition files tracked at entry are byte-identical to entry HEAD;
- exactly 33 definition files are additive;
- the engine package contains 167 regular members, of which 164 replay exactly
  against source and 62 are exact definition members; the only generated-only
  members are `.cargo_vcs_info.json`, `Cargo.lock`, and normalized `Cargo.toml`;
- package archive name: `handbook-engine-0.1.1.crate`;
- package archive SHA-256:
  `7c27519e42a345d299c0710a9c4b33cec013b506ecee585593b9aa86ec095d6e`.

The transient build location is intentionally omitted. The durable claim is the
member/source replay and archive digest.

## Final proof replay

| Command or suite | Result |
|---|---|
| focused Charter definition/profile, canonical, intake, lineage, authenticator, approval, registry, promotion, lifecycle, observation, runtime-vector, compiler/CLI, Environment Inventory, flow, and repository-identity suites | PASS, including positive, negative, replay, stale/forged/ABA, crash, no-follow, crossed-state, and fail-before-delta cases |
| `cargo test --workspace --all-targets --all-features` | PASS on the complete implementation subject |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --doc` | PASS, 2 compiler plus 7 engine compile-fail doctests, including the sealed low-level promotion API |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `cargo package -p handbook-engine --allow-dirty --no-verify` plus exact member replay | PASS, 167 / 164 / 62 member counts above |
| `bash tools/ci/codex-skill-live-smoke.sh` in Linux WSL against installed current binary and generated skills | PASS, terminal `OK` |
| `bash tools/ci/install-smoke.sh` in Linux WSL | PASS, install/reinstall/dev/public-wrapper terminal `OK` |
| native Windows all-target integration and fail-before-mutation tests | PASS inside the workspace replay |
| `python tools/check_archive_boundary.py --self-test` and normal mode | PASS |
| handoff validator normal, v1-admission self-test, and orchestration-contract self-test | PASS: 46 records, 230 current dispatches, 8 admitted legacy dispatches, 46 ledger entries before fresh Review 2 dispatch and final closeout |
| old-definition byte scan | PASS, 29 unchanged tracked files and 33 additive files |
| relative links, slice scope, secret/machine-path, zero-byte, whitespace, formatting, and `git diff --check` | PASS |
| repository-required GitNexus change detection | PASS before primary commit; complete HCM-2.2 blast radius retained for review |

ShellCheck was unavailable in the installed-skill remediation environment; the
exact shell subject instead passed `bash -n`, static obsolete/new contract
assertions, diff checks, and both full Linux installed-runtime smokes. No
required platform/runtime proof remains unavailable.

## Review and promotion boundary

This proof wall is part of the exact complete final subject given to a fresh,
isolated, read-only built-in `default` reviewer. The immutable dispatch carries
the sorted path/SHA-256 manifest and aggregate fingerprint. Any valid finding
changes the subject, forces full relevant proof replay, and requires a different
fresh reviewer. The parent-owned completed v1.2 handoff records the final
reviewer identity/status/verdict and references the primary reviewed commit.

The supported promotion is intentionally narrow: `PG-INTAKE-01`,
`PG-INTAKE-02`, and `PG-CHARTER-01` close only for the exact first-party Charter,
and `PG-YAML-02` extends only through Project Context and Charter. Generic-kind,
remaining-artifact, program-wide YAML, Resolution/Projection, and later-slice
gates remain open.
