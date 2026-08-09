# HCM-3.6 strict-Clippy corrective authority stop

## Result

The fresh HCM-3.6 strict-Clippy corrective parent cannot complete under its
closed nine-path, no-public-surface authority. This is a same-slice
adjudicable authority stop, not a Phase-3 exit claim and not a waiver.

The implementation child applied the six behavior-preserving repairs that are
independently executable within the ceiling. Parent reproduction passed both
`cargo fmt --all -- --check` and `git diff --check`, but the exact gate still
failed:

```text
cargo clippy --workspace --all-targets --all-features -- -D warnings
exit 101: 20 original dead_code diagnostics remain
```

## Exact remaining boundary

The remaining diagnostics are the pre-existing, private-but-unreachable
posture implementation surface:

- `PreparedPostureWriteV1`, unused `CommittedAuthorityHeadV1` fields, and the
  posture-transition methods/helpers in
  `crates/engine/src/charter_authority_transaction.rs`;
- the lifecycle draft/constructor/marker in
  `crates/engine/src/charter_lifecycle_transition_v11.rs`;
- the posture draft/constructor/marker/intent/binding/encoder/private validator
  and allocator in
  `crates/engine/src/charter_posture_transaction_intent_v1.rs`; and
- the project-posture replay/CAS functions in
  `crates/engine/src/project_posture.rs`.

These items are `pub(crate)` or private under parent modules that are private
in `crates/engine/src/lib.rs`. They are used by tests but have no current
production reachability. A trial promotion of the warned items to `pub` inside
the allowed files produced 14 new `private_interfaces` diagnostics, because
their parameter and return types retain the private/crate-private effective
visibility. It also did not make the private free helpers live.

The available routes are all prohibited by the current contract:

1. alter `crates/engine/src/lib.rs` or other parent-module/public-surface
   reachability;
2. widen a graph of dependent types outside the diagnosed bounded correction;
3. wire the uncalled feature into a production flow, which needs a semantic
   decision and associated owner/test authority; or
4. suppress/waive the warnings, which is expressly forbidden.

No suppression was added, no lint configuration or test was weakened, no
remote or protected checkout was touched, and the implementation child did not
stage or commit. The partial six-file source delta is not a strict-Clippy
completion and must not be published as one.

## Required authority

Resume only with a fresh top-level authority that names the permitted
reachability strategy and any required parent module, public/API, semantic, or
test-owner surface. It must preserve the current no-suppression rule and start
from a clean reviewed baseline. The permanent ordinary-validator decision
remains unchanged: it applies only to
`20260806T202700Z--HCM-3-5--resolution-aware-adoption-planning: continuation
writes or advances before selector CLEAN`, not this additional strict-Clippy
failure.
