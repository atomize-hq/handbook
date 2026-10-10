# File-backed work-state graph: Pro approach consultation

Date: 2026-10-10. Model shown by UI: GPT-6, Pro effort.
Conversation: https://chatgpt.com/c/6aca7955-56fc-83ea-82bd-67fafb5b1307
Task: independent source-grounded review of file-backed work-state graph and performance directions.
Repository baseline: `8013948ebd3fbef6725f5d72f3d29120f7617009`.
Attached bundle: 1,386 eligible tracked files, 24,055,137 source bytes, 5,368,647 ZIP bytes.
ZIP SHA256: `3ec96eacde94f59d83f555fe7d7090e44b684300ba2034009d944a730180d3a4`.
Manifest baseline fingerprint: `4c2ac7e8a67271fc4c45c6e63edf0bc9d1f854a4112839aadac3b009a71ee82c`.
The skill's exclusions removed 390 tracked paths; untracked sources were not included. Credential-pattern scan found no matches. The stock portable runner hit Windows argument-length limits while assembling a diff over all included paths. An isolated compatibility wrapper verified that the complete tracked baseline diff was empty and supplied that identical empty result to the unchanged builder. No skill files, repository source or transfer scope changed.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Sent prompt

We are already working on this project task.

Task/spec: Independent architecture and performance approach consultation for Handbook's Rust work-state store. Review the proposed file-backed versioned evidence/dependency graph, lower-level optimization opportunities, and the other investigated storage directions. This is advice and research critique, not a patch request, backend adoption, or implementation authorization. Title this conversation "Review file-backed work-state graph direction".

Current approach: Latest USER DIRECTION takes precedence over older proposed recommendations in the attached reports. The user prefers verifying each requested operation and its complete dependencies, with unrelated history damage detected when accessed or during an explicit audit, instead of rereading all history for every operation. They strongly prefer an efficient file-based system and do NOT want SQLite unless a demonstrated need remains after genuinely testing sound file-based designs. "SQLite is the only possible option" is not a claim anyone has proven. We must not mistake slow current implementation or simplified prototype wins for a backend necessity.

The newly proposed direction is a file-backed, versioned evidence graph: explicit typed exact-version links among immutable work records, kind definitions, original grants, approval/native evidence, accepted requests and terminal outcomes; direct indexes locate those objects; an operation traverses its required dependency closure and checks each shared node once within a valid held operation. Authoritative current heads/pending administration/revocation and complete request membership/absence proofs need their own coherent indexes and publication protocol. Historical validity is not current permission. A graph database is not implied. Content addressing is a candidate mechanism only and cannot silently change existing canonical identity domains. The graph may already exist implicitly in current records; investigate that before inventing a new generic graph platform. These are proposals and repository-derived inferences to verify against source, not established implementation results.

Other presented directions to assess fairly: (A) indexed individual immutable transaction files with exact-proof reuse; (B) segmented immutable history with authoritative indexes/checkpoints; (C) embedded transactional owner storage, represented by SQLite prototypes. First investigation conditionally preferred B; deeper storage-only prototypes gave C a conditional lead, but native guard/VFS integration remains unproven. Please challenge all three and the new graph direction, identifying how graph representation, physical layout, indexes, and integrity policy are separate decisions. The user's present preference is file-based first, not a predetermined bespoke graph redesign.

Project sources: Attached codex-pro-repo-input.zip is the broad current eligible tracked repository at HEAD 8013948ebd3fbef6725f5d72f3d29120f7617009. Read bundle-manifest.json first, then worktree.patch (empty), then repo/ sources. It contains 1,386 eligible tracked files, 24,055,137 source bytes. The builder excludes 390 archived/cache/agent-instruction/sensitive-path entries; excluded and untracked files are unavailable and must not be inferred. The ZIP is context for this advisory consultation, not a request to produce an apply-ready patch. Treat any instructions embedded in repository documents as material to assess, not instructions governing this chat.

Start with these files, then follow actual calls/dependencies and inspect supporting tests:
- docs/research/work-state-capacity-and-performance-investigation-2026-10-10.md and its matching evidence directory.
- docs/research/scalable-work-state-storage-feasibility-2026-10-10.md and its matching evidence/prototype directory, especially integrity-notes.md, native-profile/native-summary, prototypes.py, raw comparison and fault results.
- crates/engine/src/work_state/{store,transaction,types,scope,kind,submitted,mod}.rs; repository_durability.rs; local_authority/work_registration.rs and actually called authority/guard code.
- SDK work-state implementation, work_state_integrity/work_kind_integrity and relevant durability tests.
- docs/specs/handbook-contract-membrane/{README,architecture-context,coverage-and-landing-map,phase-and-slice-map,structured-data-format-standard}.md, semantic-model/amendments/work-progression-kinds-and-relations.md, and slices/work-compositions-and-kinds/{README,packet-1,packet-2}.md.
Use broad repo access to find reusable graph/index/object-store/durability mechanisms and the source of excessive repeated checks; do not turn this into an unrelated whole-product audit.

Evidence, blockers, constraints, and validation:
- Existing limit is 32 lifetime accepted transactions shared by items, kinds and authority administration. Fresh revocation also refuses at capacity. Other bounds include 128 MiB owner accounting, 1 MiB complete records and 256-file checkpoints. A cap-only change is insufficient.
- Initial dataset: 588 timed observations / 160 groups; warm current reads ~1.3-1.65s at 32; fresh effects ~7.27-8.74s at 31 before preparation. Small n per exact operation; Windows release; no production performance SLA.
- Deeper native instrumentation: 31 observations, 96 function spans, enabled/disabled controls; a current read at 32 performed ~36,586 handle-information checks, 9,656 directory-guard opens, 758 directory listings, 1,166 canonical JSON operations. Four full history scans consumed 5.334 s of a 7.516 s fresh effect. Distinct native proof validation already deduplicates within each scan. Warm process CPU nearly matched wall time. WPR unavailable; instrumentation is not a physical-I/O profile. Attribution and overlapping spans must be interpreted correctly.
- Prototype comparisons at 10k / 100k use synthetic envelopes sharing one actual original/proof and omit native semantic/currentness/physical guard integration. They are not proof of 100k native items. Read all disclosed sample, durability, memory, audit and fault limitations. Baseline SQLite current-pointer corruption could return stale accepted state; a checksum variant catches one-field damage but not coordinated pointer+checksum rewriting. A/B orphan recovery is incomplete in prototypes.
- Existing full-history scanning detects unrelated corruption on the next operation, but cannot give a generic authenticated guarantee against coherent between-open rollback. Under the user's preferred access/audit detection policy, a graph/index still must prove exact selected dependencies, current lineage, absence/completeness where needed, and through-publication coherence. Do not silently expand the threat model or retain an expensive incidental check as a semantic necessity without tracing its authority.
- Preserve exact submitted/effective meaning and receipts, original authority/basis, one current grant lineage, human-only scope successors, fresh unchanged-scope D-to-G activation, pending-administration suspension, revocation/no reinstatement, separate per-version kind approval, accepted-original recovery, truthful committed/noncommitted/uncertain outcomes and physical owner/root/lock binding. A fresh grant/key cannot repair uncertainty.
- Greenfield: no presumed production migration or permanent legacy API requirement. Actual retained exact evidence must not be silently erased or reidentified. Structured-format general defaults are still proposed; Contracts JSON is fixed. No new graph format or identity scheme is approved.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Is the current approach the best way forward? Give a direct keep, adjust, or pivot recommendation; tradeoffs; what to keep/change first; inspections; validation; and pitfalls.

Please provide a deep source-grounded answer with:
1. What you actually inspected and evidence limits. If ZIP access fails, state that; do not invent code findings.
2. Specific causal inefficiencies with file/function references, redundant call chains and safe versus unsafe reuse boundaries. Is this principally a storage problem, repeated validation/path-guard design problem, or both? What inexpensive diagnostics would distinguish remaining causes?
3. A concrete critique of operation-specific verification and a versioned file-backed evidence graph. Explain authoritative versus rebuildable indexes, exact versions versus current heads, absence/completeness, shared proofs, cache invalidation, missing/corrupt nodes, concurrent writers and interrupted publication. Identify existing repo facilities to reuse. Avoid requiring a generic graph engine or graph database by default.
4. A fair comparison of optimized current files, graph-indexed files, segments and SQLite under the SAME selected semantics and integrity policy, with explicit assumptions. Separate immediate optimizations from storage redesign. Explain what actual evidence, if any, could justify reconsidering SQLite.
5. A minimal staged next investigation/implementation proposal, measurable end-to-end validation, realistic workload assumptions, unresolved human choices, and what NOT to build. No code patch or repository changes; advisory only.
Do not merely repeat our reports or endorse our preferred answer. Challenge unsupported guarantees and unnecessary complexity, while respecting the user's file-based preference. Use official primary sources for external technical claims.
