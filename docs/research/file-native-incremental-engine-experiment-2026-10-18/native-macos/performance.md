> F3-only observations on f3e942f6a2e84ff33fd6ea860caed49ad650f86f. No corrected-pin 1k performance rerun was allocated.

# Native 1k observations

Each row is one observation. Full-reference and incremental-preparation clocks have different scopes; do not interpret their ratio as end-to-end speedup. Inclusive engine timers overlap. Fresh process is not device-cold. Primary role bins are invalid; totals and objects metadata counters remain valid.

| Requested IDs | Full load + semantic validation ms | Complete incremental preparation ms | Documents visited full / incremental | Incremental edges visited | Metadata read / written B |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 6.276 | 159.600 | 1000 / 2 | 0 | 230947 / 45442 |
| 8 | 3.444 | 962.645 | 1000 / 16 | 0 | 1323293 / 224517 |
| 64 | 4.297 | 1299.496 | 1000 / 129 | 1 | 9409293 / 405357 |

Incremental preparation includes graph/projection persistence, reservation and sealing. Full reference includes full load plus semantic validation. Explicit comparison audit is excluded from both clocks.

| Requested IDs | Actually changed | Accepted publication ms | Explicit A refresh ms | Touched paths / copies |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 1 | 1054.204 | 137.959 | 1 / 1 |
| 8 | 8 | 1968.607 | 454.750 | 7 / 7 |
| 64 | 64 | 2000.510 | 972.354 | 56 / 56 |

The imports reuse exactly the same per-ID source bytes across overlapping batches, producing actual changed sets 1/7/56. Validation traces use label operations and remain separately scoped.

| Operation | Engine ms | Copies | Total read / written B | Metadata read / written B |
| --- | ---: | ---: | ---: | ---: |
| initial A | 69494.152 | 2003 | 174883924 / 27240977 | 136465313 / 900543 |
| initial B | 50968.802 | 2003 | 107894613 / 27091720 | 56164229 / 900543 |
| B one-label successor | 13415.319 | 2003 | 40129324 / 13365425 | 1116781 / 45994 |

B copied 2,003 files per complete generation in this corpus, including per-document navigation/derived files. Manifest creation is additional. Both B initialization and its successor were accepted/published and explicitly verified.

Raw `metrics`, storage tree occupancy/height/physical/tombstone keys, quotas, sync/namespace/page counts and all stage clocks remain in smoke-operations.json and the source-bound JSONL archives. Peak-memory telemetry is null on Mac because the unchanged helper reads /proc. No p95 or causal OS ranking is inferred.
