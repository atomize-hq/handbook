# Handbook

This repo contains the Handbook product and its **contract membrane / executable-contract** architecture work.

## Read this first

If you are trying to understand the current intended direction, start here:

1. [`docs/START_HERE.md`](docs/START_HERE.md)
2. [`docs/specs/handbook-contract-membrane/README.md`](docs/specs/handbook-contract-membrane/README.md)
3. [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](docs/ideas/handbook-contract-membrane-architecture-memo.md)
4. [`docs/ideas/substrate_executable_contracts_architecture.md`](docs/ideas/substrate_executable_contracts_architecture.md)

## Current authority posture

- The tracked product source and tests are the truth for implemented behavior.
- The contract-membrane README classifies current authority and routes readers to the live lifecycle, review, and prompt documents; it does not authorize implementation work.
- Future work requires separately explicit, current slice-specific authority.
- The membrane memo and Substrate executable-contracts document are non-authoritative architecture lineage.
- Older reduced-v1 CLI/compiler docs have been moved out of the active front door.
- Historical planning/spec artifacts are retained for provenance, not as current execution authority.

## Where things live now

- Active docs index: [`docs/README.md`](docs/README.md)
- Contract-membrane authority and navigation: [`docs/specs/handbook-contract-membrane/README.md`](docs/specs/handbook-contract-membrane/README.md)
- Current architecture memos: [`docs/ideas/`](docs/ideas/)
- Archived reduced-v1 docs: [`archived/reduced-v1/`](archived/reduced-v1/)
- Historical root authority docs: [`archived/root-authority/`](archived/root-authority/)
- Historical research dumps: [`archived/research/`](archived/research/)
- Historical specs/plans: [`archived/specs/`](archived/specs/)

## Repo landmarks

- Rust crates: `crates/`
- Declarative/core assets: `core/`
- Tests and fixtures: `tests/`
- Tools and packaging helpers: `tools/`, `install/`

## Important note about local `.handbook/`

You may have a local ignored `.handbook/` working area while developing. Do **not** treat that ignored local state as committed repo authority unless it is deliberately promoted later.
