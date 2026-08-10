# HCM-4.1 SDK ownership and source-impact inventory

**Purpose:** read-only source/Cargo evidence for the HCM-4.1 planning subject.
It is not implementation authority and does not replace a fresh implementation
symbol-impact run.

## Repository truth

At planning base `cc44a84c0f5b75f336301dac0c502cfae909c17e`, the workspace
members are `handbook-engine` `0.2.0`, `handbook-flow` `0.1.1`,
`handbook-pipeline` `0.1.2`, `handbook-compiler` `0.1.0`, and
`handbook-cli` `0.1.0`. There is no `handbook-sdk` member.

`handbook-compiler` depends on engine, flow, and pipeline. CLI depends on
compiler and directly on each owner crate. Therefore the compiler is a
compatibility/composition seam, not evidence of semantic ownership. Future
SDK implementation must remove normal CLI composition through compiler without
inverting dependencies.

## Candidate owner and call-path evidence

| Candidate seam | Direct caller evidence at planning base | Affected process | Risk for later edit | HCM-4.1 planning conclusion |
|---|---|---|---|---|
| `CharterAuthorityTransactionServiceV1::apply_posture_transition` | Three direct call sites, all in `charter_posture_transaction_tests.rs`; no production caller was found. | Atomic Charter/posture/lifecycle commit and recovery. | CRITICAL: lock order, durable journal, promotion/lifecycle compatibility. | It stays private. A later SDK operation must reach it through a new reviewed owner-facing bridge and must not reuse tests, recovery, promotion, startup, or CLI. |
| `derive_project_posture_kernel` and `prepare_posture_change` | Transaction preflight/replay plus crate-local posture tests. | Exact one-leaf mapping, kernel replay, CAS validation. | HIGH: wrong exposure could split authority or accept stale bytes. | Preserve owner semantics and private records; expose only a bounded typed operation result/intent as later authority permits. |
| `handbook_engine::grounding::ground_resolution` | Definition is public; source search found no production call site. | Snapshot/currentness/redaction grounding. | HIGH semantic boundary, but outside posture ownership. | SDK composes its public typed outcome; it must not expose raw snapshot/delta payloads or recreate grounding. |
| `handbook_flow::adopt_grounding_outcome` | Definition only; no production call site found. | Flow typed packet adoption. | MEDIUM. | Preserve the separate owner path; future SDK may orchestrate an ordinary use case but cannot collapse Flow and engine ownership. |
| `handbook_pipeline::include_grounded_shared_resolution` | Definition only; no production call site found. | Namespaced shared inclusion. | MEDIUM. | Retain namespaced/no-fallback cutover and HCM-3.5 Packet 3 proof boundary. |
| `handbook_compiler::execute_charter_command` | CLI `author.rs` plus compiler cutover tests. | Current CLI Charter operation adapter. | HIGH compatibility/public output risk. | Move ordinary composition to SDK in a later selector; CLI retains parsing and rendering only. |
| `handbook_compiler::doctor` / `run_setup` | CLI doctor/setup plus compiler tests. | Repository UX/readiness and mutation shell. | HIGH because outcomes and exit mapping are operator-facing. | SDK owns composition; CLI keeps cwd discovery, args, rendering, stdout/stderr, and exit mapping. |
| `handbook_flow::resolve` / compiler resolver wrapper | CLI inspect/generate, compiler wrapper, and broad regression tests. | Existing resolver path. | HIGH regression surface. | Existing path is regression evidence, not an implicit SDK or posture ingress. New purpose-named use cases must not add required fields to it. |

## GitNexus status

The required GitNexus MCP `query`, `context`, `impact`, and `detect_changes`
tools are not exposed in this task environment. No index-specific callgraph or
compare-to-main result is represented as GREEN. The source-search rows above
are conservative read-only evidence only. Each future selector that changes an
existing symbol must run upstream GitNexus impact where available, record direct
callers/processes/risk, and stop for the required HIGH/CRITICAL review before
editing.

## Rejected posture ingress candidates

| Candidate | Rejection reason |
|---|---|
| Candidate promotion / Charter approval flow | It is a different immutable lifecycle with its own authority; using it would fabricate a recommendation-to-transition route. |
| Pending-journal recovery | Recovery validates/replays an already admitted private transaction; it cannot originate a new user/consumer mutation. |
| Setup, doctor, startup, reader, or constructor | These are observation/readiness paths and do not carry approved recommendation, policy, approval, reassessment, or exact mutation basis. |
| Tests or `#[cfg(test)]` reachability | Test reachability is not production adoption and is expressly prohibited as a strict-Clippy workaround. |
| Flow/pipeline grounding path | HCM-3.5 grounding is non-promoting and carries no posture approval/mutation authority. |
| Legacy CLI/compiler adapter | It would preserve the retiring compatibility seam as a domain owner and violate the SDK-first target. |

The only admitted future candidate is a typed SDK ordinary-consumer request
that delegates to the posture owner after all approved semantic inputs are
bound and revalidated by that owner.
