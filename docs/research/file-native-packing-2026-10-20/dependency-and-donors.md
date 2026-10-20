# Dependency and concept inspection

Selected codec: published zstd **0.14.3**, exact crates.io SHA-256
`db3ae9bf90af6e40197bef767eeefc46ad6397420fe873830bd85e5fbbfb3c70`.
The downloaded crate matches the prior consultation checksum. Its license is
BSD-3-Clause. Default features are disabled: no legacy codec, dictionary builder,
multithreading, bindgen, experimental API, or Rust allocator override. Level 3
independent ordinary frames are the single integrated layout. Decoder output is
a caller-sized fixed slice <=8 MiB, window log max 23, one frame, no skippable or
concatenated frames. Advertised decoded length never controls allocation.

Cargo.lock selects zstd-safe 8.2.0 and zstd-sys 2.1.2+zstd.1.5.7, compiling the
vendored native C library through cc with two build jobs. The complete resolved
manifest/license inventory and copied codec BSD/native notices are adjacent.
The new lock also records current transitive releases; the exact lock is part
of the experimental source identity and must be transported to native builds.

Read-only donor source was retrieved at Dolt
`48829716caa01d5c958b328cc6a93cb5ff1e50ce`: table.go, table_index.go and
file_manifest.go. The first three hashes match the parent's donor-boundaries
receipt. The index inspection distinguishes structural parse prerequisites
(sorted identities, legal ordinals/offsets) from complete input correspondence;
our index requires both and binds actual file sizes and SHA-256 identities.
Manifest inspection motivates private synchronized staging, expected physical
configuration and required-file presence before replacement. Our existing
Windows sync limitation remains explicit.

Read-drain exact head `06b4d9bbbf783f102d820f0880b46454b31a1c32` and relocation
head `c5dab4af005bfdf36c68f2ea1361f4418c543205` were read as exact patches.
Local tests must pause already active readers before/between lookups, drain
the entire operation, and show cancellation releases barriers. These upstream
fixes do not qualify this implementation. No Dolt implementation was copied;
only the invariant and fault-schedule concepts inform original code here.

[Git's versioned pack/index documentation](https://git-scm.com/docs/pack-format/2.37.0)
supplies sorted identity lookup and separate container/record checksum concepts.
No GPL implementation, Git object/ref semantics, delta chains, Dolt database or
Dolt format was adopted. Original source bytes and owner contracts remain exact.
