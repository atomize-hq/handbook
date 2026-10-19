# Packing guidance consultation — October 19, 2026

Completed using `chatgpt-pro-consult`, fresh independent conversation, visible **Pro, 5 of 5**. [Open the consultation](https://chatgpt.com/c/6ad66cc0-2a10-83e9-8d9a-444a226fd9c1).

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

The reconciled recommendation is to keep the disk COW core and use three separately reviewable tasks: exact-final native Linux/macOS qualification, bounded navigation incrementality, then an L/P0/PZ packing comparison on the resulting frozen baseline. Native qualification and navigation development can proceed concurrently; the navigation delta still needs its own native checks before packing. No successor task was dispatched.

Read:

- [Reconciled recommendations](reconciliation.md): verified source/research pointers, remaining limits, proposed abstractions and qualifications.
- [Concrete proposed task sequence and packing brief](next-experiment-brief.md): scope, compatibility, donor boundaries, lifetimes, recovery/restore, workloads, resource caps and decision rules.
- [Exact Pro answer](answer.md) and [sent prompt](prompt.md).
- [Bounded transmitted packet](file-native-packing-evidence.md), [transmission manifest](transmission-manifest.json), [completion receipt](consultation.json), [local integrity checks](verification.json) and [codec metadata check](codec-metadata-check.json).
- [Visual recommendation proof](pro-recommendation-detail.png).

`local-observations.md` and `experiment-options.md` preserve coordinator preparation written while Pro was responding. They are labeled pre-response drafts; the reconciled recommendation and concrete brief control this proposal.

The important new preservation gate is an exact post-GC operational backup/verified restore: the old archive contains 101,003 publications, whereas the current baseline contains 101,006 and the later retirements. This gate prepares the next experiment without invalidating the earlier preservation result.

Immediate pattern inspections are Git pack/index concepts and narrow Dolt NBS table/manifest/relocation/read-barrier boundaries. General backend replacement, consensus, mmap, automatic compaction and production API/disk-format decisions remain outside this proposal. Current authored sources and ordinary A/B readable surfaces remain unpacked.
