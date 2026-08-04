# Deferred Context Resolution continuous availability

Status: future work and explicitly non-authoritative for HCM-3.2.

## Motivation

The selected HCM-3.2 replacement protocol intentionally has a fail-closed
no-authority interval. After registry state H2 authorizes replacement O2 and
before generic promotion T2 durably commits O2, O1 is historical recovery/audit
evidence only and O2 is pending non-capability. Neither is operational. This
prevents an old artifact from regaining authority after publisher currentness
has advanced.

## Deferred design

A future continuous-availability protocol could separate:

1. H2 installation authorization, permitting O2 to be installed and verified
   without disabling O1 as current operational authority; and
2. H3 operational activation, committed only after O2's generic journal and
   publication evidence are durable, atomically making O2 current and O1
   historical.

That design could remove the no-authority window. It also adds registry states,
transition invariants, activation ownership, retry/rollback rules, and crash
proof for every boundary between H2, installation, generic commit, and H3.
Those semantics are not present authority and are not implemented by HCM-3.2.

## Trigger and boundary

Reconsider this design only after an explicit product requirement or SLO
demands uninterrupted Context Resolution across replacement and every supported
crash boundary. It then requires a separately reviewed slice/authority decision,
including registry/state-machine ownership and proportional recovery proof.
Until then, HCM-3.2 keeps safe retry, bounded diagnostics, and explicit
fail-closed quarantine while accepting temporary unavailability.
