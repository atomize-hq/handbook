# Incremental file-native engine experiment

Authorized standalone research continuing the exact baseline at 379af4e.
Work in progress: no completion or production-readiness claim.

Read [protocol](protocol.md), [allocation](allocation.md) and the byte-preserved
[accepted brief](inputs/brief.md). The control crate retains the baseline
adapter/validator/renderer with a separately selectable compact-page encoding.
New code is private experimental library/harness source, isolated from Handbook.

Both browsing contracts remain open, Dolt remains comparator-only, and authored
YAML/JSON/Markdown bytes remain exact. No migration/public API freeze is authorized.
