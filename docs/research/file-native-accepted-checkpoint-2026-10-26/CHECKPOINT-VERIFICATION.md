# Checkpoint inclusion and byte verification

Commit1: 123afcae984d40ab47bac4091e5d58794c3c17d9, parent0628322e4b48e0efad19cdc32238a622d4aaa7a8, tree4891e6c9cf3d222c6e7f19c4073cbf7dd6f882f9. Exactly one telemetry fixture argument; production min10/assertions unchanged.

Commit2: a1b743d81bb12fbd538181a16fb8fa2da35368b7, parent123afcae984d40ab47bac4091e5d58794c3c17d9, tree9f5475a8f0c588c649016e6adbb5246f30a5fb43. Exactly physical_lifetime.rs and new physical_checkpoint_tests.rs differ. All26 source lengths/SHA match the accepted manifest43a298. Prior Linux independent accepted review was reused as source authority; no new behavior was implemented and no new runtime qualification was run.

GitNexus upstream impact and detect_changes were attempted against the exact persistent worktree before the source edits/each commit. The path is unregistered, so graph coverage is UNKNOWN. No bootstrap/indexing occurred; manual exact-file/diff/manifest scope is the authority here. Structured format standard was read and remains proposed; no migration or new format default was selected.

Imported evidence uses this subtree's narrow `* -text` attribute to preserve Git/Windows checkout bytes. Raw whitespace was retained through exact gzip representation where needed. New file and staged whitespace/scope checks, source/index drift guards, imported payload hash verification and a fresh read-only inclusion/provenance review precede the final research commit. Review observation is retained separately; it does not promote the incomplete product experiment.

`CONTENT-MANIFEST.json` binds all research payload files except itself by exact stored byte length/SHA256. `provenance.json` additionally binds imported original bytes and origins; gzip rows must decode to original hashes. Original export manifests are retained unchanged and may include source members intentionally represented by Git commits instead of duplicate files here. No claim is made that omitted corpora/builds/binaries or incomplete source closure are safe to delete.
