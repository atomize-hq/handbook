# Start Here

The Handbook product includes **contract membrane** / **executable-contract** architecture work. Tracked product source and tests are the truth for implemented behavior; the older reduced-v1 CLI/compiler docs are retained only as provenance.

## Read in this order

1. [`docs/specs/handbook-contract-membrane/README.md`](specs/handbook-contract-membrane/README.md) for current authority classification and navigation
2. the separately explicit, current slice-specific authority for the assigned work
3. only the live product sources and phase-specific lifecycle, review, prompt, code, tests, documentation, and skills needed for that unit
4. [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](ideas/handbook-contract-membrane-architecture-memo.md) for non-authoritative architecture lineage
5. [`docs/ideas/substrate_executable_contracts_architecture.md`](ideas/substrate_executable_contracts_architecture.md) for deeper non-authoritative design lineage
6. [`docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`](ideas/handbook-substrate-packet-4-2-proof-findings.md) only when current authority requires historical downstream proof context

## What is current

- The tracked source, declarative product assets, and tests establish implemented product truth.
- The Handbook Contract Membrane README establishes the current program-level authority boundary and navigation only.
- Future implementation requires separately explicit, current slice-specific authority; architecture ideas and historical materials do not supply it.
- Archived reduced-v1 command/contract docs are historical input only.

## What is not current

Do not start from these unless you are intentionally doing historical archaeology:

- `archived/reduced-v1/`
- `archived/reduced-v1/front-door/`
- `archived/specs/`
- `archived/root-authority/`

## If you need the old story

The prior reduced-v1 CLI/compiler material is still preserved:

- reduced-v1 contracts and legacy docs under [`archived/reduced-v1/`](../archived/reduced-v1/)
- prior root docs under [`archived/root-authority/`](../archived/root-authority/)

## Minimal repo map

- Active docs index: [`docs/README.md`](README.md)
- Current architecture ideas: [`docs/ideas/`](ideas/)
- Contract-membrane authority and navigation: [`docs/specs/handbook-contract-membrane/README.md`](specs/handbook-contract-membrane/README.md)
- Rust code: `../crates/`
- Core declarative assets: `../core/`
