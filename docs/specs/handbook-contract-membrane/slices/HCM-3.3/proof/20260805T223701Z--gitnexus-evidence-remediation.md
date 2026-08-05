# HCM-3.3 GitNexus evidence remediation

## Closure finding

Different-fresh closure reviewer `/root/hcm33_i1_impl_closure` ran with
`gpt-5.6-sol`, `high` reasoning effort, and `fork_turns=none`. It confirmed all
four implementation discovery P2s were behaviorally closed and independently
passed the focused 13-test suite. It returned one directly unmasked evidence
P2, `HCM33-I1-IMPL-CLOSURE-001`: the first final GitNexus artifact described
102 entries as exact symbol identities even though the array contained
duplicate labels and File pseudo-entries; a replay returned 96 symbols.

No product code, test, fixture, control truth, original proof, or historical
dispatch was changed for this remediation.

## Exact causal correction

GitNexus raw staged replays were themselves row-nondeterministic, returning
between 96 and 105 exact-path unique non-File IDs across sixteen bounded runs.
The corrected artifact therefore records:

- the raw structured-output fingerprint and reported summary for every run;
- only IDs whose `filePath` is one of the four exact Rust implementation/test
  paths;
- File pseudo-entry exclusion and per-run ID deduplication;
- the sorted intersection present in all sixteen runs: exactly 96 unique symbol
  identities;
- the two affected private currentness-test processes present in every run;
- canonical population fingerprint
  `sha256:d8e7f019d50680e07fe798ec670a4715dd28111a11d22652163e63f4c770cade`;
- aggregate maximum risk `MEDIUM`, eight actual changed paths, and the honest
  unavailable final compare-to-main/FTS status.

The complete correction is
`20260805T223701Z--gitnexus-canonical-population-correction.json`. The prior
artifact remains immutable failed-closure evidence and is superseded only for
the final GitNexus population claim.

This is the first immediately causal supplemental repair for the implementation
stage. It addresses only `HCM33-I1-IMPL-CLOSURE-001`; it does not reopen general
discovery, alter the four closed behavioral triggers, or reset the causal
budget.
