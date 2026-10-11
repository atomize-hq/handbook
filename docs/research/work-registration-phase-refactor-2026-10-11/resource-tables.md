| Source | Same-process operations / handles before-after | Sampled private / OS working-set peak (MiB) | Sampled handle peak | Initial fixed-history replay private-after range (MiB) | Final fixed-history refusal/replay private-after range (MiB) |
| --- | --- | --- | --- | --- | --- |
| baseline | 48 / 57→57 | 58.57 / 68.81 | 509 | 13.73–15.12 | 16.29–18.66 |
| refactor | 48 / 57→57 | 63.88 / 78.78 | 508 | 14.20–15.40 | 17.25–43.47 |

| Source | Injected parent delay (ms) | Child work-lock wait (ms) | Child acquired work-lock held (ms) | Full child current-read wall / process CPU (ms) |
| --- | --- | --- | --- | --- |
| baseline | 200 | 3404.9798 | 943.954 | 4356.1925 / 4218.75 |
| refactor | 200 | 3528.2715 | 1023.392 | 4561.0163 / 4437.50 |

The 200 ms delay occurs in the parent BeforeIntent callback; registration continues under its guard. Wait wall spans are child-thread-local, CPU is whole-process and includes both threads. Parent guard lifetime is not separately profiled. Private/handle peaks are sampled every 100 ms; working-set peak is OS reported. History deliberately grows 9→23 between the fixed-history tails. Tail values fluctuate; they do not establish absence of memory retention.
