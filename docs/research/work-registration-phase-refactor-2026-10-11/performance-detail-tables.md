| Cohort / history | Baseline pair range / SD (ms) | Refactor pair range / SD (ms) | Paired saving range / SD (ms) | Descriptive paired-median bootstrap 95% interval (ms) |
| --- | --- | --- | --- | --- |
| explicit / 8 | 4659.49–5524.44 / 440.50 | 2902.22–3328.08 / 215.82 | 1750.37–2196.36 / 225.47 | 1756.60–2186.90 |
| explicit / 16 | 6720.26–7619.05 / 440.00 | 4117.62–4522.38 / 190.32 | 2602.64–3107.72 / 253.89 | 2608.70–3106.18 |
| explicit / 31 | 11148.03–12129.94 / 463.62 | 6754.67–7259.10 / 214.98 | 4328.00–4951.32 / 259.20 | 4391.20–4913.72 |
| kind / 8 | 4999.57–5908.19 / 358.66 | 3397.89–3878.66 / 202.46 | 1601.68–2069.61 / 190.14 | 1647.49–2026.65 |
| kind / 16 | 7620.56–8558.07 / 383.69 | 5061.83–5535.40 / 188.95 | 2558.73–3022.67 / 204.79 | 2589.96–3006.38 |
| kind / 31 | 12939.13–14228.43 / 468.52 | 8569.92–9317.29 / 281.03 | 4369.21–4933.21 / 211.55 | 4500.17–4922.17 |

| Cohort / history | Baseline preparation / invocation / actual-pair CPU median (ms) | Refactor preparation / invocation / actual-pair CPU median (ms) |
| --- | --- | --- |
| explicit / 8 | 1601.56 / 3320.31 / 4937.50 | 46.88 / 2921.88 / 2976.56 |
| explicit / 16 | 2195.31 / 4742.19 / 6953.12 | 46.88 / 4140.62 / 4187.50 |
| explicit / 31 | 3539.06 / 7898.44 / 11437.50 | 46.88 / 6851.56 / 6898.44 |
| kind / 8 | 1671.88 / 3562.50 / 5242.19 | 484.38 / 3031.25 / 3523.44 |
| kind / 16 | 2484.38 / 5367.19 / 7875.00 | 812.50 / 4281.25 / 5101.56 |
| kind / 31 | 4187.50 / 9203.12 / 13398.44 | 1523.44 / 7117.19 / 8687.50 |

Six diverse matched fixture pairs per cell; intervals are descriptive seeded 10,000-resample percentile bootstrap intervals, not production latency guarantees. Every cell has six positive pair differences (exact two-sided sign-flip p=0.03125). Full phase wall/CPU min, median, mean, maximum and sample SD remain in summary.json.
