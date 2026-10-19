# Prospective conditional raw-ledger transport

Recorded before launch on 2026-10-19, approximately 01:51 UTC. If the existing
full history proof export fails or reaches its900-second cap, preserve its exact
partial files and disposition. Do not retry that full export, restore an incomplete
capsule, or arm history retirement/collection. Permit one separately named
ledger-only lossless transport under an actual300-second outer process-tree cap.
Its full cap must fit the original reporting deadline, epoch1792378745.5431294.

Read only the completed history's expected-ledger.jsonl from the owned existing
root. Write fresh16MiB gzip chunks and verify every chunk, whole-file SHA-256,
stable source length/mtime and exactly100,000 dense indexed rows. Keep this artifact
distinct from a selected-history proof or restartable backup. It preserves the raw
per-selection receipts used by the already completed independent state checks.
No engine invocation, expanded workload, deadline reset or deletion is included.
