# Incremental file-native engine experiment

Completed bounded standalone research continuing baseline 379af4e. The experiment
reached 100,000 actual durable selections and independently verified 101,003
publications after checkpoint/lost-reply successors. Versioned correctness
capsules ran on Windows, WSL, native Linux and Mac. The final corrected core is
`5a65a2155f9eb297d2cbb1b7c900fa749629605c`; earlier measurements keep their own pins.

Read the [report](report.md), [verification/reproduction](verification.md),
[review](review.md) and [evidence guide](evidence/README.md). Original 100k
initialization censors, the independent-renderer censor and the full-proof export
censor remain. Dependent long-history restoration/GC was unperformed. All 100,000
expected ledger receipts are preserved separately with lossless checks.

Read [protocol](protocol.md), [allocation](allocation.md) and the byte-preserved
[accepted brief](inputs/brief.md). The control crate retains the baseline
adapter/validator/renderer with a separately selectable compact-page encoding.
New code is private experimental library/harness source, isolated from Handbook.

Both browsing contracts remain open, Dolt remains comparator-only, and authored
YAML/JSON/Markdown bytes remain exact. No migration/public API freeze is authorized.
