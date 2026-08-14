# Handbook Contract Membrane Architecture Memo

## Status: historical design lineage — non-authoritative

This memo preserves a small set of early ideas that influenced the Handbook
contract-membrane program. It is provenance only: it does not report current
implementation, authorize work, define a schedule, or establish completion.

## Preserved lineage

- Handbook was envisioned as the canonical owner of contract meaning, while
  Substrate would consume Handbook results rather than become a peer authority.
- Specialized validators, docks, and adapters would contribute bounded evidence
  without becoming canonical truth or requiring Handbook to implement a
  universal validator.
- Semantic rules would remain transport-neutral, with parsing, framing,
  presentation, and other interface-specific concerns kept at adapters.

These ideas were design inputs, not proof that a contract owner, dock executor,
gate system, publication path, CLI contract surface, or downstream adoption was
implemented. They do not select a future owner, runtime, or toolchain.

## Current sources

- [`Handbook Contract Membrane`](../specs/handbook-contract-membrane/README.md)
  owns current authority classification and navigation.
- [`Implemented Baseline`](../specs/handbook-contract-membrane/implemented-baseline.md)
  records current product state.
- [`Architecture Context`](../specs/handbook-contract-membrane/architecture-context.md)
  owns current architecture boundaries.

Tracked source and tests supersede this memo for executable truth. The full
pre-reset historical version remains available at Git ref
`archive/handbook-contract-membrane-pre-clean-20260812`.
