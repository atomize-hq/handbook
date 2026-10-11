| Cohort / history / source | Enabled invocation (ms) | Disabled invocation (ms) | Enabled minus disabled (ms) |
| --- | --- | --- | --- |
| explicit / 8 / baseline | 3157.4516 | 3094.8775 | 62.5741 |
| explicit / 8 / refactor | 2831.7420 | 2790.9475 | 40.7945 |
| kind / 8 / baseline | 3444.3936 | 3454.4550 | -10.0614 |
| kind / 8 / refactor | 2989.6705 | 2856.8549 | 132.8156 |
| explicit / 16 / baseline | 4582.8888 | 4506.6000 | 76.2888 |
| explicit / 16 / refactor | 4044.4877 | 3897.4869 | 147.0008 |
| kind / 16 / baseline | 5243.1176 | 5178.9308 | 64.1868 |
| kind / 16 / refactor | 4121.6776 | 4041.2314 | 80.4462 |
| explicit / 31 / baseline | 7580.7284 | 7469.9952 | 110.7332 |
| explicit / 31 / refactor | 6579.6296 | 6488.3170 | 91.3126 |
| kind / 31 / baseline | 9109.0394 | 9064.5184 | 44.5210 |
| kind / 31 / refactor | 7023.2493 | 6814.5944 | 208.6549 |

One independent enabled/disabled pair per cell/source; median 78.3675 ms, range -10.0614–208.6549 ms. The negative observation is retained as measurement noise. No overhead subtraction is applied to pristine release latency. Attribution pair timing includes preparation profile materialization.
