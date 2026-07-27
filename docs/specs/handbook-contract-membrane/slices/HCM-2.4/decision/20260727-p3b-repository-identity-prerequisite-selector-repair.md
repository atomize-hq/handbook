# HCM-2.4 P3B repository-identity prerequisite selector repair

Status: **proposed exact test-only prerequisite amendment; implementation
paused for fresh review**

Date: 2026-07-27

## Trigger

The additive fixture selector review was CLEAN and the exact ten fixture inputs
were synchronized. The focused Stage 10 preview then advanced past profile and
output-contract admission but refused before authoring:

```text
Work Specification repository authority was not admitted: the durable
repository identity is unavailable or unsafe
```

Root-cause tracing established that P3 correctly requires
`.handbook/repository-identity.v1` before canonical artifact authoring. A
committed identity inside either fixture root would be invalid test authority:
every recursively copied repository would share one durable identity, contrary
to the HCM-2.2 setup contract.

## Feasibility evidence

The smallest non-production probe ran:

```text
cargo test -p handbook-engine --test hcm_2_4_definition_runtime \
  artifact_repository_open_semantically_admits_all_p1a_intakes \
  -- --exact --nocapture
```

It passed 1/1. That existing test writes repository profile selection, invokes
`RepositoryInvocationIdentityServiceV1::initialize_for_setup`, and then opens
the artifact repository. This proves the selected profile and fresh
per-temporary-repository identity can coexist without a fixture identity or
runtime change.

GitNexus could not resolve the two CLI test helpers by symbol name and reported
`risk=UNKNOWN`. Direct call-site inspection confines each helper to five
Stage-10 CLI tests in `crates/cli/tests/cli_surface.rs`; neither has a
production caller.

## Exact selector amendment

The existing `crates/cli/tests/cli_surface.rs` P3B selector additionally permits
exactly:

1. one private test helper that invokes
   `RepositoryInvocationIdentityServiceV1::initialize_for_setup` for a supplied
   temporary repository root and fails the test if initialization refuses;
2. one invocation after installing the foundation-flow temporary repository;
   and
3. one invocation during standalone Stage 10 capture preparation.

Each temporary repository must receive a fresh setup-owned identity. The helper
must not accept, derive, inject, persist, snapshot, normalize, or assert a
fixture identity value.

No new editable implementation path is added.

## Non-goals and exit proof

No committed `repository-identity.v1`, support-module edit, production/runtime
edit, setup behavior change, released definition, schema inference, public API,
dependency, Cargo/version, unsafe-policy, or sibling-slice change is selected.
Every existing negative case and every non-Stage-10 CLI test remains unchanged.

After this fresh selector review is CLEAN, implementation may resume within the
existing P3B packet. Exit still requires the eight formerly failing tests, all
98 `cli_surface` tests, the P3 packet wall, workspace wall, formatting, clippy,
archive boundary, diff checks, and different-fresh implementation review.
