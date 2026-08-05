# HCM-3.3 GitNexus population-fingerprint remediation

## Supplemental finding

Fresh supplemental reviewer `/root/hcm33_i1_gnx_closure` ran with
`gpt-5.6-sol`, `high` reasoning effort, and `fork_turns=none`. It independently
replayed the exact 96-symbol/two-process population and found no additional
P1/P2, but returned `HCM33-I1-GNX-SUPP-CLOSURE-001`: the first correction's
population fingerprint used an undocumented literal-backslash-`n` separator.
The declared value therefore did not match the natural LF-byte serialization.

## Exact second causal correction

`20260805T225121Z--gitnexus-final-population-correction.json` preserves the
verified 96 ordinal-sorted symbol IDs, two process IDs, sixteen raw replay
fingerprints, exact paths, and `MEDIUM` risk. It adds the explicit encoding
`utf8-ordinal-sorted-symbol-id-lf-terminal-lf-v1`:

- sort: the artifact array is JavaScript ordinal lexicographic order;
- separator: one byte LF (`0x0a`);
- terminal: one byte LF (`0x0a`);
- text encoding: UTF-8;
- byte length: 5,518;
- SHA-256:
  `22d7925a7d1fdcb860bba9d0bbc696e4c5f55bfdefabe9782cfc288b5ea30583`.

Node and PowerShell/.NET independently recomputed that value from the live
artifact. This evidence-only repair changes no code, tests, fixtures, control
truth, behavioral closure, raw replay population, risk, or unavailable
compare-to-main/FTS status.

This is the second and final immediately causal supplemental cycle permitted
for the implementation stage. It addresses only
`HCM33-I1-GNX-SUPP-CLOSURE-001`; no further review cycle may follow CLEAN.
