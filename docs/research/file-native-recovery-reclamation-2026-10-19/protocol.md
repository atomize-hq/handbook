# Bounded recovery and reclamation follow-up

Standalone experimental engine; no Handbook production integration. Parent source
3396092cbf20f593e36b9640c6a76e5afd2f2a09 has identical Rust/Cargo bytes to
5a65a2155f9eb297d2cbb1b7c900fa749629605c. The shared checkout is untouched.

Navigation version is store-owned experimental configuration. Missing
`navigation_version` means the explicit frozen legacy profile 1, preserving its
existing hashes, pure candidate replay and audit renderer. New stores use profile
2: exact authored files, existing packet files and one `collection.yaml` for
stable identity, current title/kind/location and directed ordered relationships.
There are no generated Markdown navigation files or compulsory document guides
in profile 2. JSON-quoted YAML scalars preserve arbitrary fixture strings. This
small collection map deliberately performs whole-collection metadata work; large
layout/performance competition is outside this unit. Legacy stores remain a
versioned compatibility lane, not an implicit history rewrite or migration.
The independent legacy verifier now rejects dangling root destinations. A full
logical graph export and packet composition are separate from this discovery map.

A retains immutable selection plus a mutable tree that can lag/mix. Its sealed
lexical action list remains byte-compatible with old pending plans. Execution
removes obsolete files deepest first, then installs shallowest first; empty
parents are pruned without recursive deletion. Divergent files are retained by
stable digest; obstructing external subtrees move to a plan-bound divergence
directory before replacement. Pending plans finish before successors. New
namespace layouts with file ancestors (including case-folded graph paths) refuse
before selection. B still verifies a complete independent generation before
selection and supports retained selected-once ordinary reading.

Checkpoint and collection reconcile selected transfers before retiring/deleting
candidate evidence. Reservation transfer still requires exact accepted attempt,
intent and permanent allocation-owner binding. GC reuses a content traversal only
for equal `(graph tree ID, projection tree ID)` pairs. Each unique traversal still
uses the existing checked tree walker and parent/version bindings. Current
membership/outcome/change roots and replay dependencies retain their lifetimes.

Operational backup is explicitly quiescent and cooperating: no mutator runs
during capture. Durable prepared/half-prepared work can exist. The profile copies
every regular file, including controls, fences, pins, reservations, attempts,
packets, pending/completed A state, B generations and unrelated private stages.
Only the two OS lock files and destination-bound `DISPOSABLE.json` are excluded.
Per-file hashes, a full ordered inventory hash and transport hash bind a complete
manifest. No manifest means incomplete capture. Restore requires a new root,
verifies every record and whole-file digest and never imports deletion authority.
Successful byte restoration is explicitly insufficient: independent operational
verification and fresh engine reopening/recovery must pass before collection.
Permission for deletion is established anew with lifecycle setup at that exact
owned destination. Transport compression changes no live-history layout.

The old original-renaming continuation now refuses. Its proof-only caller cannot
authorize operational collection. The original H101003 store is read-only;
collection is allowed only on one independent full operational restore.

Bounds are in bounds.json; the execution clock is recorded once in evidence.
Process exits do not certify reboot/power-loss/SIGKILL durability. Windows
directory synchronization remains unsupported. Windows compressed-file-size API
values are not cluster allocation measurements; physical allocation is separately
unavailable in this census. Logical file bytes and conservative publication-owner
allowances must never be conflated.
