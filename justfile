checks:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
  cargo check --workspace

checks-with-smoke:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
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
  ! rg -nP '^(?!.*(?:\bnot\b|\bonly\b|\bdistinct\b|\bseparate\b|\bdistinguish\w*\b|\btier(?:s)?\b|\bnamed explicitly\b|\bcannot\b|\bmust not\b|\bdo not\b|\bmistaken\b|\bturn\b.*\binto\b|\bkeep(?:ing)?\b.*\bdistinct\b|\bbaseline context\b|\bVerify:\b|`rg -n|\bstill name\b))(?!\s*-?\s*(?:claim|classify)\b).*(?:Packet 4\.2.*(?:handbook-pipeline|pipeline proof)|(?:handbook-pipeline|pipeline proof).*Packet 4\.2|Set 2 packaged proof.*(?:(?:released(?: external)?|external) proof)|(?:(?:released(?: external)?|external) proof).*Set 2 packaged proof)' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-{spec,plan,tasks}.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  ! rg -n 'not\*{0,2}\s+yet|current gap|routing toward closure|gap is still open|incomplete pipeline proof' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
