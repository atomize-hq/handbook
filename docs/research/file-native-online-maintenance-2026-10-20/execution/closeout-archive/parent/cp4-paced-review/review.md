# CP4 paced source review — frozen 4434afbb

Verdict: **INTEGRATE the bounded engine pacing change, source review only. NO-INTEGRATE the frozen pace_controls.py control pending the P1 correction below. NOT GO for pacing execution or full qualification.**

Reviewed exact 30-file manifest `source-freeze-paced.json`, 12,601 bytes, SHA-256 `4434afbb3b5a4725f3d609001da93d9a4b55978bde3f7ce5cc148ab47caf2c4c`, base `a3f9805eccc19ede8dd5c735b54e675e98e63714`. Independent initial and final SHA/length comparisons found zero mismatches; total frozen bytes 452,222. Packing is 90,226 bytes/SHA `bebfdf8a3d5ec55eabc249054bfa50fd064419e03e2ad0fe915f73a7bf550bf5`. Its 13,589-byte growth fits 16,384. Five checkpoint controls total 65,460 bytes, within 65,536. The 24 other Rust/Cargo inputs exactly match the previously accepted stage freeze; protocol.py is unchanged.

## P1 — Drain bounded physical work between publisher builds

`checkpoint4/pace_controls.py:131–132` requests a physical build after each ten of 100 publishers with `cleanup=False`. No control path calls cleanup, including after the initial build. This cannot complete the prescribed ten successors while respecting the existing 128 KiB serialized Work limit.

Production `packing.rs:513–533` carries prior Work forward, appends the old generation, retains existing loose jobs, appends jobs for fresh descriptors and retains replaced descriptors. Four-segment consolidation at 331–352 only changes the selected catalog. It does not reap these Work entries. Build returns without cleanup at 964. `physical_lifetime.rs:178–179` rejects the candidate plan as `quota:physical-work-bytes` once its pretty JSON exceeds 131,072 bytes.

A conservative static size argument already exceeds that bound after successor ten. Count only the four publication/hint/candidate/allocation roles, omitting EVERY result descriptor. The older segment counts are [1,2,3,4,3,4,3,4,3,4], giving 124 old descriptors; current generation adds 12, eleven builds retain 44 loose jobs, and four consolidations retire 32 descriptors: 212 occurrences. A schema-shaped, two-space pretty serialization using one-character IDs, shortest role paths, one-digit lengths/ordinals, current plan null, 64-character hashes, and NO discovery/garbage is **145,423 bytes**. Real unique birth IDs, sizes, discovery, garbage and result descriptors increase it. Repeated dummy IDs and unrealistically short lengths intentionally make this only a serialization lower bound, not a valid Work object or an executed fixture. The initial five-role/no-op-reused-result calculation has 224 occurrences and is larger; this proof does not rely on it.

After the missing adapter is supplied, the publisher future will raise before PASS; this is a finite-control correctness defect, independent of the engine pacing implementation. Add explicitly budgeted cooperative cleanup at suitable intervals, or revise the bounded activation workload and its forecast/proof obligations. Account for the held reader until its pin drops. Do not raise the Work cap or suppress the refusal. Review the amended control as a new freeze before runtime admission.

## Engine findings and limits

No new engine defect found in the pacing delta. Entire 966-line packing.rs and all five checkpoint controls/contracts were read; unchanged protocol had been fully reviewed before and its hash was rechecked. The source-owned root gate, closed input/anchor schema, immutable origin and global ordinal arithmetic, first-lookups outside the warm timer, ten milestones, full 1,000-element warm arrays, and checked final ordinal delta are consistent. Four streams over ten batches yield 40,000 warm lookups and 40 separately timed first lookups; Driver's raw stdout logging is required by the adapter contract.

The held stream's due times span 16,000,000 through 44,997,100 microseconds relative to origin; ordinary streams finish at 44,995,500. Sleep occurs before per-query timing. Lag and successor excess produce failed deadline status; no counts are skipped and origin is never reset. Actual cadence, process startup gaps, OS scheduling, reader RSS, cache limits and query outcomes still require native raw evidence.

Cache accounting compares owned index memory with cache_bytes, rejects live Rc aliases/transient charges, destroys cache and sole physical lease, then records pins_dropped. The semantic snapshot is dropped first. No source path reacquires physical state after the reported drop. Header <=2,048 bytes, ready/anchor <=4,096 and <=10 milestones are explicit.

Windows permits only the closed CP4 disposable family and the two exact F05 donors. Unix permits the stated native-paced-cp4/native-final 40-lower-hex namespaces with safe components and DISPOSABLE identity; no Unix F05 donor is admitted. Windows QPC ABI and Unix 64-bit timespec/clock IDs are consistent with the declared targets. Apple CLOCK_MONOTONIC=6 and Linux CLOCK_MONOTONIC=1 were checked against primary headers:
- [Apple _time.h](https://raw.githubusercontent.com/apple-oss-distributions/Libc/main/include/_time.h)
- [Linux UAPI time.h](https://raw.githubusercontent.com/torvalds/linux/master/include/uapi/linux/time.h)

This header check does not establish installed native ABI, same-clock adapter binding or native execution qualification.

## Qualification and review scope

OwnedDriver/native-owned-v1 and the reviewed Native4 ownership registry are missing runtime prerequisites. The frozen control requires the adapter/contract before fixture initialization and has no PID-only fallback. A contract string alone is not implementation proof. Exact adapter path/SHA, actual child handles/tokens, image/creation/input/exit binding, same-clock bridges, live ownership and raw return behavior must be independently reviewed before RUN.

There are **23 literal #[test] declarations**, including three new pacing tests (closed, schedule, header). They remain **UNRUN on this freeze**; no build, test, native action, Python import, Git action, fixture mutation or indexing was performed here. Prior compile-only results are not execution evidence for this amended source. The actual heavy ~80 MiB constructor/cache test still needs its once-only final-source run under the separately admitted budget. Rolling 128/129, serialized 32768/32769, fresh-birth/stale-journal and raw/catalog/Work boundaries remain qualification obligations, not additional defects inferred solely from missing cases.

Five-axis review: correctness finds the control defect above; readability is dense but bounded and documented; architecture preserves opt-in pacing and existing capture/release ownership; security review found no new blocker within the closed disposable-fixture scope; performance remains unqualified pending native receipts. GitNexus was unavailable for this checkout: graph impact UNKNOWN, manual risk HIGH. No symbol was edited and no commit was made. The structured-data standard remains PROPOSED; this review approves no format migration or production default.

Source-read freeze was released to the coordinator only after the final all-30 hash check; no source rereads followed release.

## Resource accounting

Before this report, a fresh META10 v5 whole-membership census counted 1,053 regular files / 8,788,954 bytes, including all P regular files (archives/extracts too), both Landing roots, the seven included recursive execution subtrees with the declared stream-snapshot exclusion, and all twenty promoted harness files. Manifest v5 SHA `ce01f5bc510f5472feaac941b82da16bf78b3c09d737231c4d2f2c9e4d01b3be`; cap 10,485,760. Full 40,960-byte pair allowance plus 256,000-byte future reserve projects 9,085,914 bytes. The review folder initially contained only the 12,601-byte freeze; with the pair maximum it projects 53,561 bytes/3 files, within 65,536/4. Evidence records the fresh second prewrite census.

The earlier temporary five-control aggregate of 66,000 exceeded 65,536 by 464 bytes; correction to 65,460 preceded this freeze. That failure remains preserved and is not retroactively qualified. No deletion, expiry, donor or relabeling credit was taken. Existing negative reviews and failure evidence are retained.
