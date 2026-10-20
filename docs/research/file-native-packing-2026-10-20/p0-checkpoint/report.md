# P0 checkpoint — compression and cleanup remain gated

Subsequent review confirmed a B ordinary-shell discovery defect in this source
after packing cleanup. The P0 A observations below retain their exact old pin;
this checkpoint is not a final correctness qualification of the candidate.
The owning chat is preparing a separately pinned control-retention fix.

Frozen engine core: `4cb6b9fba9ffabc294a6006ec54fe75366968db4`.
Windows executable SHA-256:
`478dd9ba9f687d4de81c60ce55f9ddf9e4e2fafcc493d581b3c63e12e42f7fe5`.
The original functional cutoff is 2026-10-20 07:55:16 UTC; closeout ends
08:25:16 UTC. This checkpoint does not reset either clock.

P0 built and registered all 504,833 eligible records in 16 packs without deleting
loose records. Engine time was 1,323.3 seconds, peak RSS 181.2 MB. The 1,800-second
whole-Job watchdog passed and verified an empty process group. The complete
independent verifier matched all raw tuples, 101,007 accepted publications,
100,000 ledger rows, 101 intentional retired-content refusals, and the current
source/graph/navigation oracle. Its body took 53.2 seconds and peaked at 1.14 GB;
the 900-second watchdog passed with an empty group.

The matched store is the independently restored H101006 image plus one
separately timed old-N navigation upgrade (98.1 ms engine time), yielding
H101007. Original lookup keys remain fixed. Old N and new P loose baselines,
then P0, each have 500 warm samples/category and 20 fresh processes/category.
P0 warm medians are 0.968–0.998 times qualified loose across all six categories;
all outcome digests agree. Fresh engine medians rise from about 1.0–1.4 ms to
19–43 ms. Each fresh process loads/validates indexes; OS/device caches are
uncontrolled. The full absolute distributions, index/byte/open counters and
separate controller times are in the raw bundle.

Provided matching original intent returns the exact original outcome in L and
P0. Changed intent returns `refused:request-id-intent-mismatch`. P0 also passes
same-request deduplication and changed-request refusal, with SELECTED unchanged.
These checks are outside warm samples. Reader warm peak is 24.7 MB, including
19.3 MB charged index/cache, within the 256 MiB reader and 64 MiB cache limits.

| Observed representation | Files | Logical bytes | Regular-file data allocation |
|---|---:|---:|---:|
| Immutable eligible loose classes, still coexisting | 504,833 | 579,015,428 | 1,047,946,944 |
| P0 packs, indexes and catalog | 33 | 641,993,822 | 642,060,288 |
| Actual current engine, both representations, excluding test ledger | 510,641 | 1,283,415,930 | 1,761,774,040 |

Pack plus index alone is 641,986,037 bytes, 1.109 times eligible loose logical
bytes. Allocation uses the calibrated Windows FileStandardInfo API, measured
over every file in 79.5 seconds under a 120-second watchdog. Directory/MFT
metadata and device writes are unmeasured. Whole-store L allocation before
activation was not measured. The immutable loose classes remain byte-identical,
but actual file reduction and compacted footprint gates remain pending cleanup.

The authentic H1k record subset also passed raw-container and independent tuple
verification: 4,800 records / 5,298,334 raw bytes / 5,885,907 pack-index bytes.
It contains the initial publication and original ledger rows 1–999, plus only
their still-retained finalized attempts. It creates no STORE or SELECTED and is
never treated as a shortened operational history.

At the latest completed census ownership was 2,559,926,476 bytes / 531,623 files,
under 2,684,354,560 bytes / 560,000 files. The resource proposal retains all bytes
from the exact new 210.7 MB build target and 148.5 MB cache fixtures in a bounded,
round-trip-verified archive before any separately reviewed reclamation. Required
executables stay in place. With a conservative 64 MiB archive, 16 MiB retained
executables, 350 MB PZ reservation and 32 MiB reporting margin, the prospective
total is 2,668,206,117 bytes, leaving 16,148,443 bytes. Checkpoint Git/archive growth
must also be charged by the next fresh census. The proposal grants no deletion.

PZ build/measurements, H10k controls, matched ordinary publication measurements,
representative N8k–N10k A/B fixtures, large cleanup and full packed operational
capture/restore/reopen/successor remain unfinished. Native lanes are coordinated
by the parent and are not claimed here. Earlier failures/censors and the early
cross-Job aggregate-enforcement qualification remain preserved in the previous
candidate handoff. The frozen core has not changed.

`raw-evidence-index.json` authenticates 94 archive members, 3,508,190 expanded
bytes; the 569,099-byte archive has SHA-256
`0a6e60887bcb906f940d7acbb6f8bf78a41a21c2ef3f3e98c7700149e8a9ef57`.
All members were independently re-read and hashed after assembly. Large original
captures, current packs and H1k corpus/container bytes stay at their bound local
paths. `summary.json` carries the complete machine-readable observations and
the concrete next-stage resource proposal.
