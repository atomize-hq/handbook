checks:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
  cargo check --workspace

checks-with-smoke:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  HANDBOOK_RUN_LIVE_AUTHOR_CHARTER_SMOKE=1 HANDBOOK_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL=gpt-5.3-codex-spark HANDBOOK_AUTHOR_CHARTER_CODEX_MODEL=gpt-5.3-codex-spark cargo test --workspace
  cargo check --workspace

handbook_pipeline_released_proof version="0.1.2":
  bash tools/proof/handbook_pipeline_released_boundary.sh --version {{version}}

handbook_published_import_set3_guardrails version="0.1.2":
  bash tools/proof/handbook_pipeline_released_boundary.sh --version {{version}}
  rg -n 'Set 2 packaged proof' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-{spec,plan,tasks}.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'Set 3 released external proof' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-{spec,plan,tasks}.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'Set 3 downstream Substrate proof' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-{spec,plan,tasks}.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'handbook-pipeline 0\.1\.2' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'substrate-packet-3-3-20260623-213135' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'crates/shell/src/execution/prompt_fulfillment\.rs' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'MAP objective is now satisfied through a reviewed, stable, published boundary' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'Packet 4\.2 remains explicitly `engine \+ flow` only' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-{spec,plan,tasks}.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  ! rg -n '^\s*-?\s*Packet 4\.2 (is|counts as|satisfies).*(handbook-pipeline|pipeline)|^\s*-?\s*Set 2 packaged proof (is|counts as|satisfies).*(released|external)' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-{spec,plan,tasks}.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  ! rg -n 'not\*{0,2}\s+yet|current gap|routing toward closure|gap is still open|incomplete pipeline proof' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
