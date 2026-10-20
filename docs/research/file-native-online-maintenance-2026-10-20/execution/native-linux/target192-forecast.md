# Conditional native target reassignment audit

Conclusion: **192 MiB looks credible, but is not released or guaranteed**. Keep the
existing 256 MiB guard and forecast through the final exact-pin build and complete
unit-executable compilation. Only afterward, if the complete retained target is
measured <=201,326,592 bytes with source/binary bindings, positive process closure
and no further builds admitted, can meta consider releasing 67,108,864 bytes of
unused forecast. This is not reclaimed data or a universal Rust-build bound.

Read-only current census, source pin `62ce963085ac054bcc34eb21c6733b94db160975`:

| Current target component | Logical bytes | Files |
| --- | ---: | ---: |
| Registry artifacts and Cargo administration | 71,326,130 | 274 |
| Registry build-script/native outputs | 16,006,826 | 100 |
| Local engine and two consumer package artifacts | 39,464,407 | 34 |
| **Total** | **126,797,363** | **408** |

Every file is individually hashed in `target192-forecast.json`. Rlib/executable
hardlinks are charged at every pathname; inode sharing is not a logical-byte
discount. Current own library rlib is 7,843,556 bytes and rmeta 2,475,693; the three
release executables are about 3.546 MB each. Local deps plus fingerprints total
20,973,250 bytes. Registry build products total 16,006,826 bytes and need not be
regenerated if the selected registry/lock/toolchain/profile contract stays fixed.

The preserved donor receipts show an actual release workspace target of
123,577,017 bytes /409 files, increasing to 129,336,661 /434 after all workspace
units: **5,759,644 bytes /25 files additional**. Unit stderr identifies one
library-test executable and three zero-test main/consumer harness executables;
only the engine and two consumer packages were recompiled, with no registry
package compilation listed. The independent **80,976,318-byte cache fixture is
outside those target totals** and remains F08. That old target path is now absent
as historical state: this audit relies on authenticated retained receipts, not a
fresh census of old artifacts, and performed no deletion.

Current and donor Cargo.lock SHA-256 both equal
`20e60d9dfc38e093a3926f6867b5519356685bdb6ca38c488857216f5bc9bb7d`.
Cargo.toml hashes also match. Current fingerprint records bind rustc, optimized
profile, empty rustflags/features and Cargo config; all 45 archives/32 Linux crate
sources and the native Cargo/rustc/library hashes were already independently bound.

| Prospective addition above the current actual target | Bytes | Basis |
| --- | ---: | --- |
| One possible extra local deps/fingerprint set | 20,973,250 | Measured current package cohort, conservatively retained if package source path changes once |
| Release test executables and metadata | 8,388,608 | Historical 5,759,644-byte delta rounded up |
| Final local code and test-artifact growth | 16,777,216 | Explicit prospective allowance, not measured final output |
| j2 object/archive/linker temporary coexistence | 16,777,216 | Explicit prospective allowance, not an asserted compiler maximum |
| **Conditional total** | **189,713,653** | **11,612,939 bytes below 192 MiB** |

The current library/object and executable sizes make that temporary allowance
plausible for the local packages. It is not proof for arbitrary new source,
features, linker configuration or additional dependencies. The file cap stays
5,000; neither a new file-cap release nor unmeasured file-count guarantee is made.

Source path discipline matters. Keep the old `cp1-62ce9630` source/archive and
preserved binary evidence. If needed, introduce **one** stable `source/build-work`
package root after authorization, then reuse that exact path for remaining pins,
preserving immutable source archives/manifests separately. This forecast charges
one possible duplicate local artifact set; it does not assume a sequence of
pin-named build paths creates no duplicate Cargo outputs. No stable work root was
created by this audit.

Required conditions before any eventual release:

- Exact source-pin release and unchanged Cargo.lock/manifests, features, optimized
  profile, rustflags, Cargo registry locations and compiler/Cargo versions. Any
  material change requires a new forecast.
- One existing owned target; `--locked --offline --workspace --release -j2`,
  `CARGO_INCREMENTAL=0`; no debug profile, clean build or manual artifact removal.
- Explicitly owned compiler scratch. The large cache unit uses external
  `EXP_PACK_TEST_ROOT` under F08. Unit-execution temporary data must also be charged;
  do not hide it in source or `/tmp`. If compilation and execution use different
  temp roots, record the split and both actual censuses.
- Compile all required final unit executables, verify source unchanged by build,
  bind final binary hashes, close all build processes and measure the complete
  retained target. Preserve every old artifact still present; no manual deletion
  finances the proposed headroom.
- Release only after the final build phase ends. Sampled transient peaks remain
  sampled observations; a future source/build after release requires fresh
  admission rather than silently consuming the reassigned forecast.

The 448 MiB fixture proposal remains intact; the rejected 64 MiB default-payload
proposal is not used to finance this reassignment. No engine build, unit run,
target-cap change, source mutation, dependency change or protected cleanup occurred.
