# Matched measurement harness

The original measurements and P0 container creation used
`4cb6b9fba9ffabc294a6006ec54fe75366968db4`. After the separately documented B
loose-control publication fix, the final engine is frozen at
`d70f3864f2888d46d9112213b4a03f8d82745a8e`. `fixed_P0_readonly.py` repeats P0
reads with that final reader on the unchanged original containers;
`final_long_matrix.py` builds, independently checks and measures final PZ.
`final_tables.py` compares those final readers against qualified loose results.
These scripts describe bounded experiment stages; their presence grants no stage
admission. The controlling dispatch clock, resource admission and preserved
failure/censor receipts continue to apply.

`long_matrix.py` uses one independently restored and verified copy of the exact
H101006 post-GC operational image. Its preparation uses the qualified old N
executable to publish one separately timed navigation upgrade, then records a
cancellation fence. This yields H101007 without changing the original graph.
The frozen lookup cohort uses actual original early, middle, recent and lost
reply IDs, plus separately identified missing and cancelled IDs. New loose tail
requests never replace these keys.

The qualified loose warm consumer compiles the exact old 4bde library and lock
with a separately identified resident lookup consumer. Fresh qualified loose
lookups use the retained original N executable. New loose, P0 and PZ use the
frozen P executable. Every engine event binds its executable path and SHA-256.
Warm samples mean 500 repeated lookups per category in one process after its
first lookup. Fresh samples mean 20 process starts per category with uncontrolled
OS/device caches. Neither is a claim of physical-device cold performance.
The first lookup reported for each category shares the resident process with
preceding categories; it is not an additional fresh-process sample.
`intent_probe.py` separately checks the actual original request's provided
matching intent and a changed intent; those checks do not enter warm samples.

The P0 raw build retains loose duplicates. The PZ conversion, if separately
admitted, must retain the identical mapped logical cohort and pack grouping.
Independent verification authenticates the original operational capture's raw
inventory and the five exact records added by the navigation publication. It
compares all mapped role/key/raw-length/raw-SHA tuples, then runs the independent
full accepted-history/ledger walker and current graph/source/navigation oracle.
No selector rewind or synthetic shortened operational history is supported.

`derive_tables.py` reports absolute warm/fresh distributions and P0/qualified-L
and PZ/P0 ratios. Driver-observed child time, engine time and controller time
remain separate. Instrumented opens and logical storage/decoded bytes do not
measure physical writes. Pack/index file counts do not establish the compacted
store footprint while loose or old-container duplicates remain. FileStandardInfo
allocation observations cover regular-file data allocation; directory/MFT
metadata and physical device writes remain unmeasured.

`operational_matrix.py` prepares authentic A/B operational images for ordinary
publication comparisons. Each cell is a fresh restore of the same image, uses
the same original cohort and retention pins, and applies the same seven real
requests. It compares the resulting logical state digests and independently
verifies the cell. New unpinned predecessor tails may be retired outside the
publication clocks. Large fixtures and all cleanup need their own admission.

`subset_corpus.py`, `sample-tool` and `verify_subset.py` are record-corpus
controls, never operational stores. They select H1k/H10k original publication
prefix records from the authenticated capture, retain their exact bytes and
identities, and compile the actual frozen pack/index implementation to measure
container geometry. The separate Python parser verifies the complete resulting
raw inventory. No STORE or SELECTED control is created for these subsets.

Every executable stage must run inside the independent whole-process-group
watchdog: ordinary probes 120 seconds, long packing/capture/restore 1800 seconds,
complete independent verification 900 seconds, all clamped to the original
functional cutoff. Heavy stages are serialized. Readers remain within 256 MiB
including index/cache; checkers remain within 2 GiB. Missing samples remain null.
The original early overlap in guarded build/verifier stages had individual Jobs
and sampled aggregate observations, not continuous host-wide Job enforcement;
the later serialized protocol does not retroactively remove that limitation.
