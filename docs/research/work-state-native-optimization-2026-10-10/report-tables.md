All values are milliseconds unless marked otherwise. Current reads use four unprofiled observations per group; preparation uses six.

| Variant | Explicit current, 32 | Kind/D→G current, 32 | Explicit prepare, 31 | Kind/D→G prepare, 16 |
| --- | --- | --- | --- | --- |
| base | 1329.6 | 1694.4 | 3084.2 | 2404.6 |
| owner | 1333.1 | 1671.8 | 3025.1 | 2409.6 |
| binding | 1264.1 | 1617.6 | 2903.2 | 2304.8 |
| semantic | 1324.9 | 1692.6 | 3033.1 | 2402.9 |
| combined | 1268.2 | 1604.9 | 2894.5 | 2296.6 |
| direct | 723.9 | 1070.7 | 1860.4 | 1913.0 |
| combined-direct | 656.1 | 994.4 | 1718.7 | 1805.9 |

Fresh explicit workflows: six matched entry sizes 25–30 per variant. Paired workflow is the median of each preparation plus its actual invocation.

| Variant | Preparation | Invocation | Paired workflow | Workflow reduction |
| --- | --- | --- | --- | --- |
| base | 2774.1 | 6528.8 | 9303.0 | 0.0% |
| binding | 2624.8 | 6235.4 | 8860.2 | 4.8% |
| combined | 2611.6 | 5569.7 | 8181.3 | 12.1% |
| combined-direct | 1608.9 | 2990.4 | 4599.2 | 50.6% |
| semantic | 2793.6 | 5990.9 | 8784.5 | 5.6% |

Full-store original/current-control operations: six unprofiled observations each.

| Cohort / operation | Base | Combined | Combined-direct (departure) |
| --- | --- | --- | --- |
| explicit / historical_resolve | 1343.6 | 1309.8 | 640.4 |
| explicit / committed_recovery | 1402.5 | 1296.6 | 647.7 |
| explicit / current_grant | 1936.2 | 1743.5 | 1135.7 |
| explicit / governance_basis | 124.4 | 111.3 | 116.8 |
| kind-DG / historical_resolve | 1703.5 | 1630.6 | 1011.2 |
| kind-DG / committed_recovery | 1709.1 | 1630.4 | 1008.0 |
| kind-DG / current_grant | 2483.4 | 2359.4 | 1756.2 |
| kind-DG / governance_basis | 248.0 | 236.9 | 235.7 |

Illustrative profiled call counts (explicit current at sample 1; fresh invocation at entry 31). Repeated paths count again on every observation.

| Operation / variant | Scans | Lists | Listed entries | Directory opens | Handle info | Canonical encodings |
| --- | --- | --- | --- | --- | --- | --- |
| current_item / base | 1 | 758 | 18960 | 9656 | 36586 | 1166 |
| current_item / combined | 1 | 758 | 18960 | 5745 | 36589 | 1147 |
| current_item / combined-direct | 1 | 198 | 1240 | 3665 | 18629 | 1147 |
| fresh_invocation / base | 4 | 5527 | 100108 | 65491 | 226498 | 6398 |
| fresh_invocation / combined | 4 | 5527 | 100108 | 43218 | 226504 | 4295 |
| fresh_invocation / combined-direct | 4 | 2706 | 13695 | 32719 | 137787 | 4295 |
