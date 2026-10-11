| Cohort / history / source / held observations I2–I4 | Output or item-output reconstructions | Artifact hits / misses | Canonical encodes / MiB | Bounded work encodes / MiB | Lists / entries | Returned bounded leaf reads / MiB |
| --- | --- | --- | --- | --- | --- | --- |
| explicit / 8 / baseline | 26 | 0 / 0 | 1147 / 26.99 | 695 / 24.07 | 1072 / 11974 | 306 / 7.07 |
| explicit / 8 / refactor | 8 | 18 / 8 | 449 / 5.65 | 305 / 4.71 | 1072 / 11974 | 306 / 7.07 |
| explicit / 16 / baseline | 50 | 0 / 0 | 1915 / 49.94 | 1223 / 45.71 | 1480 / 23670 | 450 / 12.78 |
| explicit / 16 / refactor | 16 | 34 / 16 | 697 / 10.08 | 473 / 8.70 | 1480 / 23670 | 450 / 12.78 |
| explicit / 31 / baseline | 95 | 0 / 0 | 3355 / 92.96 | 2213 / 86.28 | 2245 / 55950 | 720 / 23.48 |
| explicit / 31 / refactor | 31 | 64 / 31 | 1162 / 18.38 | 788 / 16.20 | 2245 / 55950 | 720 / 23.48 |
| kind / 8 / baseline | 26 | 0 / 0 | 1995 / 35.37 | 1113 / 30.11 | 1072 / 11974 | 306 / 8.06 |
| kind / 8 / refactor | 8 | 18 / 8 | 585 / 7.16 | 371 / 5.84 | 1072 / 11974 | 306 / 8.06 |
| kind / 16 / baseline | 50 | 0 / 0 | 3723 / 68.06 | 2121 / 58.88 | 1480 / 23670 | 450 / 14.97 |
| kind / 16 / refactor | 16 | 34 / 16 | 993 / 13.48 | 619 / 11.29 | 1480 / 23670 | 450 / 14.97 |
| kind / 31 / baseline | 95 | 0 / 0 | 6963 / 129.36 | 4011 / 112.83 | 2245 / 55950 | 720 / 27.93 |
| kind / 31 / refactor | 31 | 64 / 31 | 1758 / 25.34 | 1084 / 21.52 | 2245 / 55950 | 720 / 27.93 |

I1 has a separate temporary cache and remains separate in profile-tables.md. Each held artifact map also derives the new candidate outside scans once; it is already present for I3/I4 hits. Proof hit/miss counters are refactor-only; the baseline already had a per-scan historical-proof set. These counters are not physical disk I/O.
