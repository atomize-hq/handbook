# Native 1k smoke observations

One observation per initialization/commit/packet. All engine stages are inclusive and can overlap. Controller latency includes process response serialization and exit; prelaunch allocation checks are outside that request clock. Darwin wait4 peak RSS includes the entire process, including serialization/exit. No device-cold or percentile claim.

| Backend | Mode | Operation | Engine ms | Process ms | Metadata read / written B | Copied files | Process peak MiB |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| manifest | A | initial | 29819.93 | 29829.08 | 938052 / 469026 | 1009 | 8.06 |
| manifest | A | commit-1-packet-False | 195.30 | 200.16 | 1876104 / 469026 | 1 | 7.59 |
| manifest | A | commit-8-packet-False | 408.59 | 415.17 | 1876104 / 469026 | 8 | 8.42 |
| manifest | A | commit-64-packet-False | 1972.85 | 1979.96 | 1876104 / 469026 | 64 | 7.61 |
| manifest | A | commit-1-packet-True | 259.25 | 264.06 | 1876104 / 469026 | 10 | 7.66 |
| manifest | A | commit-8-packet-True | 502.28 | 508.62 | 1876104 / 469026 | 17 | 8.19 |
| manifest | A | commit-64-packet-True | 2235.80 | 2243.83 | 1876104 / 469026 | 73 | 9.20 |
| manifest | A | current selected packet | 131.32 | 136.20 | 469026 / 0 | 9 | 5.03 |
| manifest | A | initial retained selected packet | 125.82 | 129.30 | 469026 / 0 | 9 | 5.19 |
| manifest | B | initial | 25221.05 | 25228.10 | 469026 / 469026 | 1000 | 7.55 |
| manifest | B | commit-1-packet-False | 6540.70 | 6545.47 | 938052 / 469026 | 1000 | 6.83 |
| manifest | B | commit-8-packet-False | 6517.76 | 6523.02 | 938052 / 469026 | 1000 | 6.94 |
| manifest | B | commit-64-packet-False | 7556.76 | 7561.91 | 938052 / 469026 | 1000 | 7.25 |
| manifest | B | commit-1-packet-True | 6578.70 | 6583.24 | 938052 / 469026 | 1000 | 7.22 |
| manifest | B | commit-8-packet-True | 6670.15 | 6674.69 | 938052 / 469026 | 1000 | 7.03 |
| manifest | B | commit-64-packet-True | 7989.57 | 7994.87 | 938052 / 469026 | 1000 | 7.88 |
| manifest | B | current selected packet | 154.71 | 159.00 | 469026 / 0 | 9 | 5.08 |
| manifest | B | initial retained selected packet | 112.30 | 115.69 | 469026 / 0 | 9 | 5.05 |
| cow | A | initial | 29807.02 | 29812.99 | 2522806 / 1259666 | 1009 | 9.05 |
| cow | A | commit-1-packet-False | 259.75 | 265.02 | 5128806 / 81631 | 1 | 8.83 |
| cow | A | commit-8-packet-False | 454.58 | 459.41 | 5128856 / 81659 | 8 | 9.16 |
| cow | A | commit-64-packet-False | 2147.44 | 2154.22 | 5128910 / 81644 | 64 | 9.98 |
| cow | A | commit-1-packet-True | 382.38 | 387.64 | 5128847 / 81635 | 10 | 9.02 |
| cow | A | commit-8-packet-True | 578.78 | 586.82 | 5128866 / 81658 | 17 | 9.91 |
| cow | A | commit-64-packet-True | 2239.40 | 2249.92 | 5129069 / 81725 | 73 | 9.91 |
| cow | A | current selected packet | 123.31 | 127.65 | 1261495 / 0 | 9 | 6.34 |
| cow | A | initial retained selected packet | 130.08 | 133.82 | 1261403 / 0 | 9 | 6.23 |
| cow | B | initial | 27471.81 | 27477.61 | 1261403 / 1259666 | 1000 | 9.25 |
| cow | B | commit-1-packet-False | 6694.21 | 6699.19 | 2606002 / 81631 | 1000 | 8.05 |
| cow | B | commit-8-packet-False | 6453.97 | 6459.82 | 2606026 / 81659 | 1000 | 8.02 |
| cow | B | commit-64-packet-False | 9861.74 | 9866.65 | 2606067 / 81644 | 1000 | 8.53 |
| cow | B | commit-1-packet-True | 6509.38 | 6514.21 | 2606028 / 81635 | 1000 | 8.23 |
| cow | B | commit-8-packet-True | 6568.55 | 6573.44 | 2606033 / 81658 | 1000 | 8.14 |
| cow | B | commit-64-packet-True | 7637.68 | 7642.58 | 2606146 / 81725 | 1000 | 8.52 |
| cow | B | current selected packet | 143.88 | 147.98 | 1261495 / 0 | 9 | 6.25 |
| cow | B | initial retained selected packet | 116.41 | 120.08 | 1261403 / 0 | 9 | 6.27 |

Current and initial-retained point medians below each use exactly three successful fresh processes. These are ordinary metadata key `d000001` and outside-range negative key `not-present`; they are not payload reads or full audits.

| Backend | Mode | Binding | Key | Samples | Engine median ms | Process median ms | Metadata read B |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| manifest | A | current | d000001 | 3 | 3.955 | 9.508 | 469026 |
| manifest | A | current | not-present | 3 | 3.425 | 7.628 | 469026 |
| manifest | A | initial retained | d000001 | 3 | 4.119 | 8.727 | 469026 |
| manifest | A | initial retained | not-present | 3 | 3.512 | 7.196 | 469026 |
| manifest | B | current | d000001 | 3 | 3.028 | 6.424 | 469026 |
| manifest | B | current | not-present | 3 | 2.990 | 6.302 | 469026 |
| manifest | B | initial retained | d000001 | 3 | 4.137 | 8.766 | 469026 |
| manifest | B | initial retained | not-present | 3 | 3.544 | 8.408 | 469026 |
| cow | A | current | d000001 | 3 | 1.974 | 6.079 | 83462 |
| cow | A | current | not-present | 3 | 1.427 | 4.924 | 54062 |
| cow | A | initial retained | d000001 | 3 | 2.437 | 7.127 | 83370 |
| cow | A | initial retained | not-present | 3 | 1.645 | 5.308 | 54062 |
| cow | B | current | d000001 | 3 | 2.115 | 6.592 | 83462 |
| cow | B | current | not-present | 3 | 1.483 | 5.288 | 54062 |
| cow | B | initial retained | d000001 | 3 | 2.211 | 7.047 | 83370 |
| cow | B | initial retained | not-present | 3 | 1.302 | 4.308 | 54062 |

| Backend | Mode | Selected publications | Logical bytes | Allocated file bytes | Files | Directories |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| manifest | A | 7 | 31601241 | 32780288 | 2274 | 313 |
| manifest | B | 7 | 108663181 | 110542848 | 8234 | 320 |
| cow | A | 7 | 30067677 | 31313920 | 2303 | 342 |
| cow | B | 7 | 107129574 | 109076480 | 8263 | 349 |
