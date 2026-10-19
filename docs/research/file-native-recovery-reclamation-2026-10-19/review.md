# Focused causal review

Reviewed the changed projection ordering, version routing/replay, allocation
transfer/checkpoint/collection ordering, operational transport boundaries and
independent navigation/history oracles using the code-review-and-quality skill.
This is a focused author review, not an independent model or comprehensive audit.

Required findings corrected before the source pin:

- Namespace validation originally checked exact-case projection ancestors only.
  It now also checks the graph's case-folded path index. A focused negative case
  proves refusal before selection.
- The restore reader initially consumed the path length before checking its cap.
  It now bounds path/data lengths before reading or allocating their buffers.
- The first census mislabeled GetCompressedFileSizeW results as allocated file
  bytes. Raw receipts are retained with an explicit correction; corrected results
  expose compressed-size API values separately and Windows allocation as unknown.
- A first backup-test attempt used refresh on an already-current watermark and
  therefore reached no interruption hook. The corrected test uses explicit resume;
  the original failed attempt is retained.
- Preserving a directory at one fixed backup name could block replay if a user
  recreated it after an exit. Each incarnation now receives a distinct retained
  path. The changed-phase regression recreates the obstruction before resume.
- Per-store-only format routing would keep producing legacy Markdown after new
  edits to an old store. Routing now binds the reader to the immutable projection
  profile, replays sealed legacy candidates in their original profile, and targets
  YAML for every new ordinary preparation. Historical objects stay unchanged.

Regression evidence covers namespace replay and corrections, divergent files and
subtrees, semantic discovery/membership, relationship direction/type/order and
multiplicity, exact source extraction, selected-once B paths, missing/corrupt
evidence refusal before rendering, eligible replay dependencies, same-ID/different
intent and packet-owner protection across checkpoint/retirement/collection.

Known bounded tradeoff: profile-2 metadata rendering is O(N) into one YAML map.
This is acceptable for the assigned small surface, not a large-N performance
claim. Legacy compatibility intentionally retains the old reader/replay renderer;
new targets use profile 2 and historical Markdown/stored hashes are unchanged.
Hostile external namespace/root swaps,
power-loss durability and generalized restore security remain outside this
cooperating experimental contract.

GitNexus: the indexed shared checkout is c7f54e6 and predates this engine; exact
worktree identity is unregistered. Impact returns missing/UNKNOWN or a degraded
schema error. detect-changes is invoked against the actual worktree and fails
repository binding. No zero-impact/clean graph claim is made; direct source caller
tracing and focused recovery tests are the substitute evidence authorized by this
unit. No analyzer repair is attempted.
