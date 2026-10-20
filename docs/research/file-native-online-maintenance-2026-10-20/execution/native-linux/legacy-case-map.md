# Existing packing capsule case mapping

Plan only; every row remains pending on the final source pin. Baseline is the
read-only `a0b92fe5dc64269508601dcddf7e2e2ca477e30d` packing capsule, with
52 applicable native cases /267 events. The two Windows loose-sharing cases and
Windows old-container sharing branch are explicitly native Linux SKIPs. A new
event count is reported as measured, never forced to match the old count.

| Existing case IDs | Count | Final-pin treatment |
| --- | ---: | --- |
| `operational-A`, `operational-B` | 2 | Keep exact P0/PZ raw identity, persistent pin/preparation/packet/PENDING, backup/restore/successor and retired-attempt assertions. Replace frozen grouping equality with explicit coverage of the newly accepted loose tail under rolling policy. Version/instance-aware independent full decoder required. |
| `orphan-unregistered-backup-restore` | 1 | Retain unselected ownership, backup/restore, safe abort and successful new build proof. Include durable new generation/work metadata but exclude process locks under the reviewed backup contract. |
| `interruption-{A,B}-{pack-reservation,pack-build,pack-index,pack-install,pack-before-catalog,pack-catalog,pack-accounting}-1` and `interruption-{A,B}-pack-loose-delete-{1,3}` | 18 | Retain every interruption phase or an explicit final-source checkpoint correspondence. Full independent byte/outcome/graph/view oracle after recovery; selected authority wins over stale journal. Add two-successor inheritance and actual-deletion accounting schedules. |
| `missing-catalog`, `corrupt-catalog`, `missing-index`, `corrupt-index`, `missing-pack`, `truncated-pack`, `corrupt-record`, `offset-overflow`, `raw-length-bound`, `duplicate-key`, `omitted-entry`, `wrong-pack-binding`, `raw-hash-mismatch`, `bad-codec`, `too-many-packs`, `duplicate-json-key` | 16 | Keep each refusal property. Bind corruptions to the final version/instance/page schema. Required-route damage refuses even with a valid loose duplicate. Add distinct untouched-page/full-audit scope evidence; it does not replace any required-route case. |
| `unselected-installed-backup-restore` | 1 | Retain installed-but-unselected restoration and resumable activation without granting acceptance from physical output. |
| `protected-read-{A,B}-{pack-read-lease,pack-catalog-captured,resolve-history-captured,pack-record-located}` | 8 | Preserve old busy assertion as a legacy-conversion case where applicable. For v3, hold the corresponding captured generation, prove two successor activations and ordinary progress, then exact old-reader outcome and release/drain. Missing/renamed barriers require an explicit source mapping, not a skipped row. |
| `writer-and-competing-maintenance-{A,B}` | 2 | Keep publication during preparation, explicit maintenance contention and stale semantic candidate conflict. Add stale physical CAS as a distinct result. |
| `cancelled-drain-releases-barriers-{A,B}` | 2 | Preserve initial legacy conversion cancellation and pin/gate closure; version-scope v3 cancellation to short capture/activation acquisition and deferred cleanup. Demonstrate ordinary operations and safe resume afterward. |
| `repack-old-pack-deletion-interruption-{A,B}` | 2 | Preserve interruption after an actual old container removal, selected outcome, independent full verification and idempotent resume; use generation-scoped physical instances and inherited work. |
| **Total applicable native legacy case IDs** | **52** | All rows require final-pin evidence or a named failure/censor. |

The 24 strict supplement cases, eight focused cases, five existing workspace
units, 24/282 recovery baseline cases and ordinary A/B extraction routes are
separate required sets. They are not absorbed into this 52-case total. Exact
final-source manifests and raw cases will establish their individual mappings.

The existing raw packing capsule creates an >8 MiB oversize allocation journal
and clones it into several backup/restore roots; its aggregate footprint is not
admitted merely because one store is small. The old native unit receipt measured
80,976,318 bytes of synthetic files beneath the manifest's `target`, independently
of the 129,336,661-byte release test target. Both must be classified and admitted
before a full unit/capsule run. No such fixture has been created by this worker.
