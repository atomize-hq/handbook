# Pinned donor boundary inspection

Read-only preparation on October 19, 2026. P remains gated on Q/N. No donor
implementation has been copied, no dependency has been adopted, and no disk
format is frozen by these notes.

The [Git pack/index documentation](https://git-scm.com/docs/pack-format/2.37.0)
describes independent pack checksums, sorted identity lookup and index-to-pack
binding. Those concepts support separately checked raw identity and physical
representation. Git's object types, refs and delta formats are unnecessary here.
Its GPL implementation is outside the selected source-copying allowance.

Dolt source was retrieved at exact commit
`48829716caa01d5c958b328cc6a93cb5ff1e50ce`. Local retrieval hashes:

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `go/store/nbs/table.go` | 13,027 | `a08b8931a9da16f9dda299d0ebcfcc2d40c0298d3ad9f4cfd19ffab1ef7b2002` |
| `go/store/nbs/table_index.go` | 23,480 | `9c39a079e789befcae9ad8e6941cfd428b20cd685c1cf3ae434972e1ab27dd47` |
| `go/store/nbs/file_manifest.go` | 17,817 | `999e2542595528d5fa84f202216ce4936e27fa7052da35244d373fba1cdef8b9` |

The [table boundary](https://github.com/dolthub/dolt/blob/48829716caa01d5c958b328cc6a93cb5ff1e50ce/go/store/nbs/table.go)
separates sorted logical addresses from physical record order and records decoded
length totals. Its 20-byte hashes and Snappy framing do not define our format.
The file carries Apache-2.0 and Dolt/Attic Labs attribution; any later copying
would require retaining applicable notices.

The [index validator](https://github.com/dolthub/dolt/blob/48829716caa01d5c958b328cc6a93cb5ff1e50ce/go/store/nbs/table_index.go)
checks sorted prefixes, ordinal ranges and increasing offsets on parse. Separate
deeper checks verify distinct ordinals and the name binding. This is a useful
distinction between safe read preconditions and complete input correspondence.
Our bounded pack experiment still requires both, checked integer arithmetic,
actual file-size binding and independent raw digest verification.

The [manifest installation boundary](https://github.com/dolthub/dolt/blob/48829716caa01d5c958b328cc6a93cb5ff1e50ce/go/store/nbs/file_manifest.go)
uses writer exclusion, synchronizes a private temporary file, checks expected
state and newly referenced table availability, replaces the manifest and syncs
the directory. These steps support our staged-install fault schedule; they do
not establish our cross-platform guarantees or acceptance authority.

| Upstream correction | Exact head | Merge |
| --- | --- | --- |
| [11891: read drain](https://github.com/dolthub/dolt/pull/11891) | `06b4d9bbbf783f102d820f0880b46454b31a1c32` | `ac3244dac46543d4e8440e567b9bc07db53c1651` |
| [11892: relocation lookup](https://github.com/dolthub/dolt/pull/11892) | `c5dab4af005bfdf36c68f2ea1361f4418c543205` | `67b1990fd92108bd3d4719784815961dfe231021` |

11891 motivates protecting an entire read across barrier installation, including
already active reads, and releasing parked work on cancellation. 11892 motivates
an adversarial reader paused between physical lookup stages while relocation
completes. Our inference is to capture a catalog only after acquiring lifetime
protection and retain that protection for the complete read. An upstream ordering
fix is not a substitute for proving our own catalog/deletion protocol.

These source observations are design inputs. Upstream tests, popularity and
platform claims do not qualify the experimental engine. Decoder bounds, Windows
sharing refusal, accounting after successful deletion, and packed operational
restore remain local implementation/verification requirements for task P.
