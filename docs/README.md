# Handbook Docs

## Active docs

Start here for the current direction:

- [`docs/START_HERE.md`](START_HERE.md)
- [`docs/cli-json-protocol.md`](cli-json-protocol.md) for the current CLI
  machine protocol and legacy migration
- [`docs/specs/handbook-contract-membrane/README.md`](specs/handbook-contract-membrane/README.md)
- [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](ideas/handbook-contract-membrane-architecture-memo.md)
- [`docs/ideas/substrate_executable_contracts_architecture.md`](ideas/substrate_executable_contracts_architecture.md)

Helpful historical input:

- [`docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`](ideas/handbook-substrate-packet-4-2-proof-findings.md)

## Archived docs

- Reduced-v1 CLI/compiler docs: [`archived/reduced-v1/`](../archived/reduced-v1/)
- Archived front-door docs from that era: [`archived/reduced-v1/front-door/`](../archived/reduced-v1/front-door/)
- Historical spec and planning archive: [`archived/specs/`](../archived/specs/)

## How to read this repo now

- Treat [`docs/specs/handbook-contract-membrane/README.md`](specs/handbook-contract-membrane/README.md) as the current authority-classification and navigation entry point, not as implementation authority.
- Treat tracked product source and tests as the truth for implemented behavior. Begin future work only from separately explicit, current slice-specific authority.
- Load the bounded current authority and only the phase-specific lifecycle, review, prompt, code, tests, documentation, and skills needed for the assigned unit.
- Treat `docs/ideas/*membrane*` and `docs/ideas/*executable_contracts*` as non-authoritative architecture lineage and deeper design input.
- Treat `archived/reduced-v1/**`, `archived/specs/**`, and `archived/**` as provenance only unless explicitly revived later.
- Do not treat the archived reduced-v1 CLI command/contract stack as the active direction for the next workstream.
