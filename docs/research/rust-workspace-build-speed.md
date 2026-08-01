# Rust workspace check/test/build speed research

Date: 2026-08-01

## Executive recommendation

Adopt in this order, measuring after each change:

1. Add `cargo-nextest` and use `cargo nextest run --workspace` for the normal full test pass, while retaining a separate `cargo test --workspace --doc` lane if doctests matter. Nextest globally schedules individual tests and can partition suites, but it does not currently run doctests.[1]
2. Benchmark reduced debug information for development and test profiles (`debug = 1`, then `debug = 0` for tests if acceptable). Cargo's default `dev` and `test` profiles use full debug information (`debug = 2`), and Cargo profiles can be configured at the workspace root.[2]
3. Benchmark `rust-lld` on Windows developer machines. For Linux CI/release linking, benchmark `mold` or `lld`; mold's supported platforms do not include Windows.[3][4]
4. Add `sccache` for CI and clean/branch-switch rebuilds. Do not assume it improves the tight local incremental loop: sccache explicitly does not cache Rust incremental compilation, and its documentation recommends disabling incremental compilation when using it.[5]
5. Use Cargo build timings before deeper changes (`cargo check --workspace --timings` and `cargo test --workspace --no-run --timings`). Cargo generates an HTML report that shows unit concurrency and per-unit duration.[6]

The likely largest non-tooling win is consolidating integration-test targets. Cargo treats every file directly under a package's `tests/` directory as a separate integration-test crate, so each creates compilation and linking work.[7]

## Repository-specific observations

- The workspace has 5 packages and 101 Cargo targets: 4 libraries, 1 binary, 1 build script, and 95 integration-test targets. `handbook-engine` alone has 56 integration-test targets.
- A source scan finds about 1,301 `#[test]`/`#[tokio::test]` attributes across 218 Rust files.
- The current `target/` directory is about 40.2 GiB. That makes debug-info and linker experiments especially relevant, although target size alone is not a timing measurement.
- The local `just checks` recipe runs clippy over all targets/features, then tests, then `cargo check --workspace`. These commands overlap substantially. The final `cargo check` is unlikely to add useful compile coverage after `cargo clippy --workspace --all-targets --all-features`; confirm that assumption with CI policy and remove it from the fast lane if confirmed.
- CI already uses `Swatinem/rust-cache@v2` independently in format, clippy, test, and smoke jobs. The action caches Cargo directories and compiled artifacts and exposes cache-scope controls.[8] Before replacing it, measure cache-hit behavior. A shared `sccache` backend may be more useful when jobs/runners cannot reuse Cargo target artifacts reliably.
- `cargo tree --workspace --duplicates` reports only a small set of duplicate versions (`getrandom`, `hashbrown`, and `windows-sys`). This does not indicate a large dependency-version duplication problem.
- There is no root `.cargo/config.toml`, no pinned `rust-toolchain.toml`, and no custom Cargo profile today.

## Tooling assessment

### 1. cargo-nextest: high priority for full test runs

Nextest builds the suite through Cargo, discovers tests, then schedules each test individually across the machine. Its runner supports retries, slow-test handling, JUnit output, and suite partitioning.[1] This repository is a strong candidate because it has roughly 1,300 tests spread across 95 integration-test binaries; Cargo/libtest's per-binary execution model cannot schedule all of those tests as one global pool.

Suggested first experiment:

```console
cargo nextest run --workspace
cargo test --workspace --doc
```

Compare warm and clean timings against `cargo test --workspace`. Nextest will not remove the cost of compiling and linking 95 integration-test crates. Very short tests can also expose process-start overhead because nextest runs each test in its own process.[1]

For CI, nextest archive/partition features can split a prebuilt test suite across runners, but start with a single runner so the team can distinguish scheduling gains from extra CI complexity.[9]

### 2. Faster linking: high priority, platform-specific

Rust's linker work can dominate builds with many test binaries. The Rust compiler supports selecting a linker with `-C linker`, and Cargo target configuration supports a `target.<triple>.linker` setting.[3][10]

- Windows/MSVC developers: benchmark the `rust-lld` shipped with the Rust toolchain. Treat this as an opt-in experiment first because linker compatibility must be verified against the CLI build script, native dependencies, debug workflow, and release artifacts.
- Linux CI/development: benchmark `mold` and `lld`. Mold describes itself as a high-performance drop-in linker and documents Linux/BSD support, not Windows.[4]
- macOS CI: do not prescribe mold from this evidence; keep the platform default unless a supported linker is separately evaluated.

Linker changes mostly help `cargo build`, test compilation, and the code-generation portion of clippy. They will help `cargo check` less because check avoids final machine-code generation/linking.

### 3. Cargo profiles: high priority and nearly free to trial

Cargo's default development and test profiles use `opt-level = 0`, `debug = 2`, `incremental = true`, and 256 codegen units.[2] Full debug info across dozens of test binaries can cost both disk and link time.

Trial at the workspace root:

```toml
[profile.dev]
debug = 1

[profile.test]
debug = 1
```

If stack traces and the debugger remain adequate, trial `debug = 0` for `[profile.test]`. Keep incremental compilation enabled for the normal local workflow. Measure both first-build and one-file rebuild behavior; profile changes invalidate artifacts once.

Do not reach first for `codegen-units`, optimization, or LTO changes: Cargo already uses high codegen parallelism in dev/test, while optimization and LTO generally trade more compile time for runtime performance.[2]

### 4. sccache: high priority for CI/clean builds, medium for local use

Configure sccache as Cargo's compiler wrapper (`build.rustc-wrapper` or `RUSTC_WRAPPER`). It can store compilation results locally or in remote object stores.[5][10]

Best fits here:

- CI runners that frequently start without a reusable `target/` directory;
- switching branches or toolchains where Cargo artifacts are invalidated;
- sharing a remote cache among equivalent runners.

Caveat: sccache does not cache Rust incremental compilation, and its official guidance is to set `CARGO_INCREMENTAL=0` when using sccache.[5] That makes a local always-on wrapper a benchmark question, not an automatic recommendation. Keep Cargo incremental builds for the default edit/check loop unless measurements show sccache wins for this codebase.

Rust compilations that invoke the linker (including binaries, dynamic libraries, and proc macros) are also not cacheable as complete sccache units, so a faster linker remains complementary.[5]

### 5. Faster feedback commands: high priority workflow change

Keep a full gate, but add focused commands for daily iteration:

```console
cargo check -p handbook-engine
cargo test -p handbook-engine --test vocabulary_registry
cargo nextest run -p handbook-engine -E 'test(vocabulary)'
```

Cargo package selection (`-p`) and workspace selection (`--workspace`) are first-class options.[11] `default-members` only affects commands run without explicit package selection, so it would not speed the current commands that explicitly pass `--workspace`.[12]

A practical command split would be:

- `just fast`: format check, package-scoped check/test selected from the changed area;
- `just checks`: format check, full clippy, nextest, doctests;
- CI: authoritative full gates.

The current trailing `cargo check --workspace` should be measured and probably removed from the full local recipe after clippy/test. The exact policy exception would be any target class or doctest configuration intentionally omitted by clippy and tests.

### 6. cargo-hakari: low priority for this workspace today

Hakari generates a workspace-hack crate to unify dependency features across workspace members, which can prevent Cargo from rebuilding shared dependencies with different feature sets when packages are built separately.[13] It is most useful when CI builds many workspace packages independently or feature skew is visible in Cargo timings.

This repository has only five members, uses resolver 2, and its slow commands build the full workspace together. The small duplicate-version report also gives no evidence that Hakari should be an early intervention. Reconsider it only if timings show the same heavy dependencies compiled repeatedly under different feature sets or CI moves to a per-package matrix.

### 7. CI caching and job layout: already partly implemented

`Swatinem/rust-cache` is already present. Its documented defaults cache Cargo's registry/git data and selected target artifacts; its cache key incorporates Rust and dependency state.[8] Improvements to test empirically:

1. Inspect restore/save logs and hit rates before adding another cache layer.
2. Give compatible jobs an intentional shared cache key only if their artifacts are actually reusable.
3. Use `sccache` with a shared backend when cross-job compilation-cache reuse is desired.
4. Keep fmt separate because it compiles nothing. Keep clippy/test parallel if elapsed CI wall time matters more than duplicated dependency work; combine them only if measurements show cache misses dominate.

`cargo-chef` is mainly for caching Rust dependency layers in Docker builds.[14] There is no Docker build in the inspected CI, so it is not a current priority.

## Measurement plan

Use at least one clean and three warm samples for each candidate, on the same machine and commit:

```console
cargo check --workspace --timings
cargo test --workspace --no-run --timings
cargo test --workspace
cargo nextest run --workspace
sccache --show-stats
```

Record separately:

- clean compile/link time;
- one-file rebuild time in a leaf crate and in `handbook-engine`;
- test execution time after a no-op build;
- peak concurrency and longest units from Cargo timings;
- cache hit rate;
- `target/` size;
- debugger/backtrace quality after profile changes.

This avoids adopting tools that improve clean CI builds while degrading the much more frequent local incremental loop.

## Sources

1. nextest documentation, running model and features: <https://nexte.st/docs/running/>; <https://nexte.st/docs/design/why-process-per-test/>; <https://nexte.st/docs/features/retries/>; doctest limitation: <https://nexte.st/docs/usage/#limitations>
2. Cargo Reference, Profiles: <https://doc.rust-lang.org/cargo/reference/profiles.html>
3. rustc Codegen Options (`linker`): <https://doc.rust-lang.org/rustc/codegen-options/index.html#linker>
4. mold project documentation and supported systems: <https://github.com/rui314/mold>
5. sccache project documentation, Rust usage and incremental-compilation caveat: <https://github.com/mozilla/sccache#usage>; <https://github.com/mozilla/sccache#rust>
6. Cargo Reference, Build Cache, `--timings`: <https://doc.rust-lang.org/cargo/reference/build-cache.html#build-timings>
7. Cargo Reference, Cargo Targets, integration tests: <https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests>
8. Swatinem/rust-cache action documentation: <https://github.com/Swatinem/rust-cache>
9. nextest documentation, partitioning and archives: <https://nexte.st/docs/features/partitioning/>; <https://nexte.st/docs/ci-features/archiving/>
10. Cargo Reference, Configuration (`build.rustc-wrapper`, target linker): <https://doc.rust-lang.org/cargo/reference/config.html#buildrustc-wrapper>; <https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinker>
11. Cargo Reference, Package Selection: <https://doc.rust-lang.org/cargo/commands/cargo-test.html#package-selection>
12. Cargo Reference, Workspaces (`default-members`): <https://doc.rust-lang.org/cargo/reference/workspaces.html#the-default-members-field>
13. cargo-hakari documentation: <https://docs.rs/cargo-hakari/latest/cargo_hakari/about/index.html>
14. cargo-chef project documentation: <https://github.com/LukeMathWalker/cargo-chef>
