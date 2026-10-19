# Derived measurement tables

one observation of each named operation in each cell; uncontrolled caches; engine wall clocks exclude independent oracle/GC/census; no SLA or latency distribution claim; full restored A10k comparators supersede extra deferred-view controls in the tables

## A payload dirtiness

| N | Profile | Prepare ms | Header points | D rows | Generated parts | Bytes | Replay objects/files |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 128 | 2 | 48.36 | 1 | 128 | 1 | 19530 | 5/4 |
| 128 | 3 | 26.68 | 1 | 0 | 0 | 0 | 5/3 |
| 128 | 4 | 126.87 | 1 | 0 | 0 | 0 | 5/3 |
| 1000 | 2 | 284.44 | 1 | 1000 | 1 | 153036 | 5/4 |
| 1000 | 3 | 29.91 | 1 | 0 | 0 | 0 | 5/3 |
| 1000 | 4 | 101.23 | 1 | 0 | 0 | 0 | 5/3 |
| 10000 | 2 | 2847.41 | 1 | 10000 | 1 | 1938936 | 7/4 |
| 10000 | 3 | 63.40 | 1 | 0 | 0 | 0 | 7/3 |
| 10000 | 4 | 39.04 | 1 | 0 | 0 | 0 | 7/3 |

## A title update locality

| N | Profile | Prepare ms | Header points | D rows | E rows | Generated parts/bytes | Changed nav paths |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: |
| 128 | 2 | 48.58 | 1 | 128 | 4 | 1/19540 | 1 |
| 128 | 3 | 52.72 | 1 | 128 | 4 | 1/19540 | 1 |
| 128 | 4 | 82.86 | 129 | 0 | 4 | 2/19980 | 1 |
| 1000 | 2 | 219.32 | 1 | 1000 | 4 | 1/153046 | 1 |
| 1000 | 3 | 225.77 | 1 | 1000 | 4 | 1/153046 | 1 |
| 1000 | 4 | 234.25 | 511 | 0 | 2 | 2/78407 | 1 |
| 10000 | 2 | 2800.65 | 1 | 10000 | 5004 | 1/1938946 | 1 |
| 10000 | 3 | 2776.49 | 1 | 10000 | 5004 | 1/1938946 | 1 |
| 10000 | 4 | 220.77 | 322 | 0 | 0 | 2/50090 | 1 |

## B complete materialization, payload case

| N | Profile | Prepare ms | Full generation ms | Selection ms | Generation verification ms |
| --- | --- | ---: | ---: | ---: | ---: |
| 128 | 2 | 396.57 | 349.39 | 52.03 | 11.74 |
| 128 | 3 | 376.61 | 350.35 | 32.84 | 11.21 |
| 128 | 4 | 387.30 | 360.31 | 30.90 | 10.71 |
| 1000 | 2 | 3522.62 | 3209.21 | 354.35 | 80.03 |
| 1000 | 3 | 3224.10 | 3195.76 | 110.11 | 87.05 |
| 1000 | 4 | 4570.77 | 4537.43 | 115.38 | 91.04 |

N10k B profile4 initialization reached its 120-second child cap before selection. Profiles2/3 were unperformed. No B10k timing is imputed.
