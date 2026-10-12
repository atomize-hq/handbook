# Observed prototype measurements

Fresh processes use an uncontrolled OS cache. These are isolated prototype results, not production capacity or power-loss qualification.

| Platform | Variant | Case | History size | n | Command median ms | Command p95 ms | Process median ms | Max RSS KiB |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| windows | chunk | broad-audit | 1000 | 1 | 83.2444 | 83.2444 | 93.9502 | 6068 |
| windows | chunk | broad-audit | 10000 | 1 | 275.9097 | 275.9097 | 287.1379 | 10408 |
| windows | chunk | broad-audit | 100000 | 1 | 958.6003 | 958.6003 | 971.1157 | 52192 |
| windows | chunk | insert-fresh | 1000 | 30 | 22.4196 | 30.5656 | 33.0825 | 5820 |
| windows | chunk | insert-fresh | 10000 | 30 | 67.1596 | 89.8549 | 81.4507 | 8388 |
| windows | chunk | insert-fresh | 100000 | 30 | 410.4420 | 455.8559 | 425.1749 | 29464 |
| windows | chunk | instrumentation-off | 1000 | 30 | 5.1381 | 6.8391 | 17.5924 | 5684 |
| windows | chunk | instrumentation-on | 1000 | 30 | 5.0409 | 6.1991 | 17.5623 | 5708 |
| windows | chunk | instrumentation-off | 10000 | 30 | 42.1145 | 53.0179 | 55.8033 | 8396 |
| windows | chunk | instrumentation-on | 10000 | 30 | 40.2589 | 47.1621 | 53.5609 | 8280 |
| windows | chunk | instrumentation-off | 100000 | 30 | 400.6370 | 456.7911 | 414.1132 | 29304 |
| windows | chunk | instrumentation-on | 100000 | 30 | 399.2723 | 438.8500 | 414.1039 | 29360 |
| windows | chunk | lookup-batch-64 | 1000 | 30 | 5.1612 | 6.4709 | 18.2497 | 5716 |
| windows | chunk | lookup-batch-64 | 10000 | 30 | 41.7435 | 46.5175 | 54.6564 | 8300 |
| windows | chunk | lookup-batch-64 | 100000 | 30 | 362.4432 | 429.2483 | 375.7513 | 29360 |
| windows | chunk | negative-fresh | 1000 | 30 | 4.7172 | 6.3234 | 15.6814 | 5712 |
| windows | chunk | negative-fresh | 10000 | 30 | 36.8195 | 56.3141 | 49.8452 | 8348 |
| windows | chunk | negative-fresh | 100000 | 30 | 371.5113 | 417.0227 | 386.0569 | 29436 |
| windows | chunk | positive-fresh | 1000 | 30 | 4.5211 | 6.1750 | 15.3978 | 5732 |
| windows | chunk | positive-fresh | 10000 | 30 | 38.8595 | 53.1376 | 53.1171 | 8396 |
| windows | chunk | positive-fresh | 100000 | 30 | 373.6568 | 411.3811 | 385.7774 | 29416 |
| windows | chunk | replay-fresh | 1000 | 30 | 4.9889 | 6.6746 | 15.6709 | 5732 |
| windows | chunk | replay-fresh | 10000 | 30 | 38.2471 | 54.0543 | 51.4427 | 8372 |
| windows | chunk | replay-fresh | 100000 | 30 | 362.3111 | 428.4163 | 377.0928 | 29408 |
| windows | chunk | replay-setup | 1000 | 1 | 20.4397 | 20.4397 | 31.5359 | 5436 |
| windows | chunk | replay-setup | 10000 | 1 | 74.0335 | 74.0335 | 87.7565 | 8072 |
| windows | chunk | replay-setup | 100000 | 1 | 453.7238 | 453.7238 | 467.1197 | 29308 |
| windows | chunk | update-1 | 1000 | 30 | 21.4708 | 28.1954 | 33.2030 | 5804 |
| windows | chunk | update-1 | 10000 | 30 | 58.0222 | 75.4500 | 70.6172 | 8284 |
| windows | chunk | update-1 | 100000 | 30 | 358.2881 | 454.7458 | 370.6067 | 29460 |
| windows | chunk | warm-repeat | 1000 | 30 | 5.1860 | 6.8091 | 18.1789 | 5724 |
| windows | chunk | warm-repeat | 10000 | 30 | 37.0003 | 49.1991 | 52.0412 | 8324 |
| windows | chunk | warm-repeat | 100000 | 30 | 369.8105 | 470.5551 | 384.0170 | 29316 |
| windows | chunk | write-batch-64 | 1000 | 30 | 23.0771 | 29.4642 | 35.9126 | 5832 |
| windows | chunk | write-batch-64 | 10000 | 30 | 59.6605 | 79.8655 | 72.0049 | 8260 |
| windows | chunk | write-batch-64 | 100000 | 30 | 431.1986 | 456.5681 | 443.3417 | 29452 |
| windows | disk | broad-audit | 1000 | 1 | 542.0274 | 542.0274 | 552.5144 | 5656 |
| windows | disk | broad-audit | 10000 | 1 | 869.2701 | 869.2701 | 880.2357 | 5700 |
| windows | disk | broad-audit | 100000 | 1 | 1728.7258 | 1728.7258 | 1740.5873 | 5944 |
| windows | disk | insert-fresh | 1000 | 30 | 25.4300 | 44.0185 | 37.4397 | 5852 |
| windows | disk | insert-fresh | 10000 | 30 | 32.1767 | 39.6846 | 44.4105 | 5676 |
| windows | disk | insert-fresh | 100000 | 30 | 35.6816 | 45.5086 | 48.4876 | 5864 |
| windows | disk | instrumentation-off | 1000 | 30 | 1.3653 | 2.3012 | 11.3981 | 5400 |
| windows | disk | instrumentation-on | 1000 | 30 | 1.4032 | 2.0828 | 11.4037 | 5396 |
| windows | disk | instrumentation-off | 10000 | 30 | 1.7659 | 2.5689 | 14.1853 | 5400 |
| windows | disk | instrumentation-on | 10000 | 30 | 1.7476 | 2.5248 | 14.7201 | 5408 |
| windows | disk | instrumentation-off | 100000 | 30 | 1.8757 | 3.1392 | 14.6177 | 5412 |
| windows | disk | instrumentation-on | 100000 | 30 | 1.8962 | 2.9008 | 15.1334 | 5412 |
| windows | disk | lookup-batch-64 | 1000 | 30 | 26.1680 | 33.3507 | 36.6816 | 5704 |
| windows | disk | lookup-batch-64 | 10000 | 30 | 42.3141 | 50.3337 | 55.7039 | 5728 |
| windows | disk | lookup-batch-64 | 100000 | 30 | 52.4143 | 59.7034 | 66.2052 | 5728 |
| windows | disk | negative-fresh | 1000 | 30 | 1.5463 | 2.0396 | 13.9257 | 5288 |
| windows | disk | negative-fresh | 10000 | 30 | 1.5391 | 2.0743 | 14.2744 | 5108 |
| windows | disk | negative-fresh | 100000 | 30 | 1.6860 | 2.3457 | 13.7256 | 5256 |
| windows | disk | positive-fresh | 1000 | 30 | 1.6497 | 2.0197 | 13.7538 | 5388 |
| windows | disk | positive-fresh | 10000 | 30 | 1.7965 | 2.5488 | 14.3575 | 5408 |
| windows | disk | positive-fresh | 100000 | 30 | 1.8381 | 2.1859 | 13.8909 | 5404 |
| windows | disk | replay-fresh | 1000 | 30 | 1.7928 | 12.8363 | 14.2882 | 5340 |
| windows | disk | replay-fresh | 10000 | 30 | 1.9583 | 3.0327 | 14.7007 | 5312 |
| windows | disk | replay-fresh | 100000 | 30 | 2.1566 | 2.7756 | 14.5771 | 5372 |
| windows | disk | replay-setup | 1000 | 1 | 27.9678 | 27.9678 | 39.8158 | 5548 |
| windows | disk | replay-setup | 10000 | 1 | 43.2968 | 43.2968 | 56.9684 | 5508 |
| windows | disk | replay-setup | 100000 | 1 | 32.4322 | 32.4322 | 43.1505 | 5632 |
| windows | disk | update-1 | 1000 | 30 | 13.8161 | 18.4254 | 24.1371 | 5444 |
| windows | disk | update-1 | 10000 | 30 | 16.0424 | 21.2956 | 28.1835 | 5412 |
| windows | disk | update-1 | 100000 | 30 | 17.0742 | 23.5315 | 28.4028 | 5436 |
| windows | disk | warm-repeat | 1000 | 30 | 40.3938 | 53.6086 | 50.4560 | 5720 |
| windows | disk | warm-repeat | 10000 | 30 | 61.1435 | 73.2111 | 74.1502 | 5708 |
| windows | disk | warm-repeat | 100000 | 30 | 77.3375 | 88.5317 | 91.2112 | 5700 |
| windows | disk | write-batch-64 | 1000 | 30 | 485.3174 | 651.0647 | 495.3093 | 5828 |
| windows | disk | write-batch-64 | 10000 | 30 | 585.1174 | 755.7976 | 597.5477 | 5860 |
| windows | disk | write-batch-64 | 100000 | 30 | 672.0784 | 825.9210 | 682.1898 | 5864 |
| linux | chunk | broad-audit | 1000 | 1 | 11.5300 | 11.5300 | 12.2722 | 21112 |
| linux | chunk | broad-audit | 10000 | 1 | 48.3197 | 48.3197 | 49.2150 | 21228 |
| linux | chunk | broad-audit | 100000 | 1 | 312.4400 | 312.4400 | 314.0719 | 48428 |
| linux | chunk | insert-fresh | 1000 | 30 | 21.6180 | 26.1228 | 22.2697 | 21112 |
| linux | chunk | insert-fresh | 10000 | 30 | 36.8328 | 40.5645 | 37.5760 | 21228 |
| linux | chunk | insert-fresh | 100000 | 30 | 203.4337 | 236.2945 | 205.0623 | 25964 |
| linux | chunk | instrumentation-off | 1000 | 30 | 1.3301 | 1.5063 | 1.8817 | 21112 |
| linux | chunk | instrumentation-on | 1000 | 30 | 1.3458 | 1.4737 | 1.8821 | 21112 |
| linux | chunk | instrumentation-off | 10000 | 30 | 12.8575 | 13.6213 | 13.6572 | 21228 |
| linux | chunk | instrumentation-on | 10000 | 30 | 13.0299 | 13.8071 | 13.8713 | 21228 |
| linux | chunk | instrumentation-off | 100000 | 30 | 128.5652 | 148.5368 | 129.7890 | 25964 |
| linux | chunk | instrumentation-on | 100000 | 30 | 129.6866 | 156.5218 | 131.0304 | 25964 |
| linux | chunk | lookup-batch-64 | 1000 | 30 | 1.3707 | 1.5748 | 1.9368 | 21112 |
| linux | chunk | lookup-batch-64 | 10000 | 30 | 13.1516 | 13.8509 | 13.9686 | 21228 |
| linux | chunk | lookup-batch-64 | 100000 | 30 | 130.2510 | 152.1589 | 131.5424 | 25964 |
| linux | chunk | negative-fresh | 1000 | 30 | 1.4039 | 1.6173 | 2.0331 | 21112 |
| linux | chunk | negative-fresh | 10000 | 30 | 12.9601 | 14.6327 | 13.7363 | 21228 |
| linux | chunk | negative-fresh | 100000 | 30 | 152.6162 | 179.5890 | 154.2404 | 25964 |
| linux | chunk | positive-fresh | 1000 | 30 | 1.4150 | 1.6398 | 2.0711 | 21112 |
| linux | chunk | positive-fresh | 10000 | 30 | 12.9918 | 14.2589 | 13.7637 | 21228 |
| linux | chunk | positive-fresh | 100000 | 30 | 144.3986 | 170.7724 | 146.2173 | 25964 |
| linux | chunk | replay-fresh | 1000 | 30 | 1.4489 | 1.6317 | 2.0935 | 21112 |
| linux | chunk | replay-fresh | 10000 | 30 | 12.8956 | 14.2727 | 13.6554 | 21228 |
| linux | chunk | replay-fresh | 100000 | 30 | 146.2909 | 184.3567 | 147.8461 | 25964 |
| linux | chunk | replay-setup | 1000 | 1 | 21.0731 | 21.0731 | 21.8467 | 21112 |
| linux | chunk | replay-setup | 10000 | 1 | 37.9011 | 37.9011 | 38.7782 | 21228 |
| linux | chunk | replay-setup | 100000 | 1 | 198.4454 | 198.4454 | 200.3789 | 25964 |
| linux | chunk | update-1 | 1000 | 30 | 14.6935 | 20.4561 | 15.3048 | 21112 |
| linux | chunk | update-1 | 10000 | 30 | 37.1193 | 53.8382 | 37.8411 | 21228 |
| linux | chunk | update-1 | 100000 | 30 | 156.9019 | 165.3565 | 158.0168 | 25964 |
| linux | chunk | warm-repeat | 1000 | 30 | 1.3731 | 1.5361 | 1.9206 | 21112 |
| linux | chunk | warm-repeat | 10000 | 30 | 13.0546 | 14.4999 | 13.8964 | 21228 |
| linux | chunk | warm-repeat | 100000 | 30 | 129.7640 | 149.7462 | 131.0502 | 25992 |
| linux | chunk | write-batch-64 | 1000 | 30 | 15.8450 | 25.9106 | 16.4527 | 21112 |
| linux | chunk | write-batch-64 | 10000 | 30 | 36.3727 | 54.7042 | 37.2186 | 21228 |
| linux | chunk | write-batch-64 | 100000 | 30 | 158.7978 | 166.1941 | 160.0807 | 25992 |
| linux | disk | broad-audit | 1000 | 1 | 57.6690 | 57.6690 | 58.5081 | 22956 |
| linux | disk | broad-audit | 10000 | 1 | 93.6964 | 93.6964 | 94.4946 | 23532 |
| linux | disk | broad-audit | 100000 | 1 | 358.6048 | 358.6048 | 359.3658 | 23916 |
| linux | disk | insert-fresh | 1000 | 30 | 13.9441 | 18.9605 | 14.5143 | 21612 |
| linux | disk | insert-fresh | 10000 | 30 | 16.9477 | 21.3286 | 17.5423 | 22956 |
| linux | disk | insert-fresh | 100000 | 30 | 17.3023 | 21.7931 | 17.9660 | 23532 |
| linux | disk | instrumentation-off | 1000 | 30 | 0.2783 | 0.3120 | 0.8631 | 21612 |
| linux | disk | instrumentation-on | 1000 | 30 | 0.2875 | 0.3458 | 0.8469 | 21612 |
| linux | disk | instrumentation-off | 10000 | 30 | 0.2893 | 0.3892 | 0.8659 | 22956 |
| linux | disk | instrumentation-on | 10000 | 30 | 0.2840 | 0.3473 | 0.8503 | 22956 |
| linux | disk | instrumentation-off | 100000 | 30 | 0.3164 | 0.4004 | 0.9422 | 23532 |
| linux | disk | instrumentation-on | 100000 | 30 | 0.3153 | 0.3421 | 0.9164 | 23532 |
| linux | disk | lookup-batch-64 | 1000 | 30 | 10.2301 | 10.7317 | 10.8726 | 21612 |
| linux | disk | lookup-batch-64 | 10000 | 30 | 11.5650 | 11.9353 | 12.2618 | 22956 |
| linux | disk | lookup-batch-64 | 100000 | 30 | 12.7868 | 13.3131 | 13.4711 | 23532 |
| linux | disk | negative-fresh | 1000 | 30 | 0.2202 | 0.2697 | 0.7651 | 21612 |
| linux | disk | negative-fresh | 10000 | 30 | 0.1477 | 0.1805 | 0.7219 | 22956 |
| linux | disk | negative-fresh | 100000 | 30 | 0.2205 | 0.3098 | 0.8062 | 23532 |
| linux | disk | positive-fresh | 1000 | 30 | 0.2861 | 0.3560 | 0.8307 | 21612 |
| linux | disk | positive-fresh | 10000 | 30 | 0.2922 | 0.3555 | 0.8723 | 22956 |
| linux | disk | positive-fresh | 100000 | 30 | 0.3185 | 0.4229 | 0.9189 | 23532 |
| linux | disk | replay-fresh | 1000 | 30 | 0.2358 | 0.3257 | 0.8051 | 21612 |
| linux | disk | replay-fresh | 10000 | 30 | 0.2400 | 0.3241 | 0.8071 | 22956 |
| linux | disk | replay-fresh | 100000 | 30 | 0.2544 | 0.3577 | 0.8484 | 23532 |
| linux | disk | replay-setup | 1000 | 1 | 15.2930 | 15.2930 | 15.9712 | 21612 |
| linux | disk | replay-setup | 10000 | 1 | 20.0000 | 20.0000 | 20.6740 | 22956 |
| linux | disk | replay-setup | 100000 | 1 | 20.7944 | 20.7944 | 21.5398 | 23532 |
| linux | disk | update-1 | 1000 | 30 | 9.8541 | 15.3464 | 10.4626 | 21612 |
| linux | disk | update-1 | 10000 | 30 | 11.5514 | 15.8619 | 12.2505 | 22956 |
| linux | disk | update-1 | 100000 | 30 | 11.2528 | 16.7682 | 11.8422 | 23532 |
| linux | disk | warm-repeat | 1000 | 30 | 15.4438 | 16.3039 | 16.0251 | 21612 |
| linux | disk | warm-repeat | 10000 | 30 | 16.8752 | 17.6717 | 17.4542 | 22956 |
| linux | disk | warm-repeat | 100000 | 30 | 18.7803 | 20.2054 | 19.4047 | 23532 |
| linux | disk | write-batch-64 | 1000 | 30 | 240.7183 | 460.0820 | 241.3493 | 21612 |
| linux | disk | write-batch-64 | 10000 | 30 | 353.2319 | 576.9111 | 354.1609 | 22956 |
| linux | disk | write-batch-64 | 100000 | 30 | 380.1444 | 474.2481 | 380.7864 | 23532 |
| windows-repaired-run | chunk | broad-audit | 1000 | 1 | 77.2593 | 77.2593 | 87.5374 | 6020 |
| windows-repaired-run | chunk | broad-audit | 10000 | 1 | 213.3822 | 213.3822 | 225.7981 | 10372 |
| windows-repaired-run | chunk | broad-audit | 100000 | 1 | 1015.3637 | 1015.3637 | 1028.7331 | 52248 |
| windows-repaired-run | chunk | insert-fresh | 1000 | 30 | 22.1621 | 27.5669 | 32.2996 | 5824 |
| windows-repaired-run | chunk | insert-fresh | 10000 | 30 | 60.3675 | 67.1656 | 71.8846 | 8336 |
| windows-repaired-run | chunk | insert-fresh | 100000 | 30 | 367.6445 | 398.0884 | 378.8601 | 29440 |
| windows-repaired-run | chunk | instrumentation-off | 1000 | 30 | 4.9794 | 7.0061 | 19.3098 | 5712 |
| windows-repaired-run | chunk | instrumentation-on | 1000 | 30 | 4.9554 | 6.5897 | 17.2471 | 5704 |
| windows-repaired-run | chunk | instrumentation-off | 10000 | 30 | 31.9178 | 36.4932 | 42.4070 | 8188 |
| windows-repaired-run | chunk | instrumentation-on | 10000 | 30 | 31.6145 | 38.8446 | 41.6062 | 8300 |
| windows-repaired-run | chunk | instrumentation-off | 100000 | 30 | 293.1248 | 306.8224 | 303.6535 | 29344 |
| windows-repaired-run | chunk | instrumentation-on | 100000 | 30 | 288.7346 | 312.7240 | 299.0908 | 29324 |
| windows-repaired-run | chunk | lookup-batch-64 | 1000 | 30 | 5.0689 | 7.4342 | 18.3127 | 5748 |
| windows-repaired-run | chunk | lookup-batch-64 | 10000 | 30 | 32.4139 | 39.1713 | 42.7439 | 8212 |
| windows-repaired-run | chunk | lookup-batch-64 | 100000 | 30 | 290.8388 | 307.8595 | 301.5720 | 29336 |
| windows-repaired-run | chunk | negative-fresh | 1000 | 30 | 4.4478 | 5.4246 | 14.5501 | 5716 |
| windows-repaired-run | chunk | negative-fresh | 10000 | 30 | 33.9292 | 39.7342 | 45.9034 | 8264 |
| windows-repaired-run | chunk | negative-fresh | 100000 | 30 | 338.7423 | 370.2604 | 353.3447 | 29340 |
| windows-repaired-run | chunk | positive-fresh | 1000 | 30 | 4.4570 | 5.4954 | 14.6410 | 5728 |
| windows-repaired-run | chunk | positive-fresh | 10000 | 30 | 33.8913 | 38.1791 | 46.4393 | 8240 |
| windows-repaired-run | chunk | positive-fresh | 100000 | 30 | 336.8074 | 377.4401 | 350.3675 | 29388 |
| windows-repaired-run | chunk | replay-fresh | 1000 | 30 | 5.0834 | 6.5212 | 15.3759 | 5736 |
| windows-repaired-run | chunk | replay-fresh | 10000 | 30 | 33.8209 | 39.6037 | 46.9656 | 8328 |
| windows-repaired-run | chunk | replay-fresh | 100000 | 30 | 334.9592 | 368.6766 | 347.6003 | 29392 |
| windows-repaired-run | chunk | replay-setup | 1000 | 1 | 19.3373 | 19.3373 | 29.7291 | 5728 |
| windows-repaired-run | chunk | replay-setup | 10000 | 1 | 55.7528 | 55.7528 | 66.2820 | 8124 |
| windows-repaired-run | chunk | replay-setup | 100000 | 1 | 392.5206 | 392.5206 | 404.3993 | 29308 |
| windows-repaired-run | chunk | update-1 | 1000 | 30 | 19.8395 | 25.9380 | 30.9796 | 5796 |
| windows-repaired-run | chunk | update-1 | 10000 | 30 | 54.6629 | 65.0389 | 66.5719 | 8248 |
| windows-repaired-run | chunk | update-1 | 100000 | 30 | 354.4678 | 371.3505 | 367.6548 | 29368 |
| windows-repaired-run | chunk | warm-repeat | 1000 | 30 | 5.0796 | 7.7069 | 17.5716 | 5684 |
| windows-repaired-run | chunk | warm-repeat | 10000 | 30 | 31.1249 | 38.9223 | 41.6929 | 8248 |
| windows-repaired-run | chunk | warm-repeat | 100000 | 30 | 288.6975 | 311.1114 | 299.3102 | 29396 |
| windows-repaired-run | chunk | write-batch-64 | 1000 | 30 | 20.3683 | 26.5009 | 32.3847 | 5840 |
| windows-repaired-run | chunk | write-batch-64 | 10000 | 30 | 56.2387 | 67.9082 | 68.6790 | 8232 |
| windows-repaired-run | chunk | write-batch-64 | 100000 | 30 | 387.8107 | 446.7519 | 402.0844 | 29544 |
| windows-repaired-run | disk | broad-audit | 1000 | 1 | 467.0972 | 467.0972 | 477.3978 | 5692 |
| windows-repaired-run | disk | broad-audit | 10000 | 1 | 838.3063 | 838.3063 | 849.0262 | 5696 |
| windows-repaired-run | disk | broad-audit | 100000 | 1 | 1820.8915 | 1820.8915 | 1836.6883 | 5980 |
| windows-repaired-run | disk | insert-fresh | 1000 | 30 | 26.6378 | 37.5125 | 38.7065 | 5688 |
| windows-repaired-run | disk | insert-fresh | 10000 | 30 | 28.2702 | 36.5421 | 39.8353 | 5668 |
| windows-repaired-run | disk | insert-fresh | 100000 | 30 | 32.8289 | 40.8392 | 45.2390 | 5772 |
| windows-repaired-run | disk | instrumentation-off | 1000 | 30 | 1.5050 | 2.5020 | 13.9447 | 5384 |
| windows-repaired-run | disk | instrumentation-on | 1000 | 30 | 1.5331 | 2.0081 | 13.9833 | 5380 |
| windows-repaired-run | disk | instrumentation-off | 10000 | 30 | 1.5821 | 1.9078 | 12.5131 | 5404 |
| windows-repaired-run | disk | instrumentation-on | 10000 | 30 | 1.5255 | 2.0617 | 12.0404 | 5400 |
| windows-repaired-run | disk | instrumentation-off | 100000 | 30 | 1.6353 | 2.1696 | 12.2268 | 5404 |
| windows-repaired-run | disk | instrumentation-on | 100000 | 30 | 1.7192 | 2.0547 | 12.1914 | 5396 |
| windows-repaired-run | disk | lookup-batch-64 | 1000 | 30 | 28.4300 | 34.4923 | 40.6666 | 5732 |
| windows-repaired-run | disk | lookup-batch-64 | 10000 | 30 | 35.5376 | 40.3722 | 46.0298 | 5696 |
| windows-repaired-run | disk | lookup-batch-64 | 100000 | 30 | 41.2221 | 53.8048 | 52.5920 | 5716 |
| windows-repaired-run | disk | negative-fresh | 1000 | 30 | 1.4744 | 1.6961 | 13.1540 | 5272 |
| windows-repaired-run | disk | negative-fresh | 10000 | 30 | 1.4590 | 1.8551 | 13.4474 | 5120 |
| windows-repaired-run | disk | negative-fresh | 100000 | 30 | 1.7184 | 2.4883 | 14.2573 | 5184 |
| windows-repaired-run | disk | positive-fresh | 1000 | 30 | 1.6083 | 2.2427 | 14.0089 | 5392 |
| windows-repaired-run | disk | positive-fresh | 10000 | 30 | 1.6549 | 2.0205 | 12.8596 | 5404 |
| windows-repaired-run | disk | positive-fresh | 100000 | 30 | 1.8092 | 2.0871 | 14.3282 | 5408 |
| windows-repaired-run | disk | replay-fresh | 1000 | 30 | 2.0928 | 2.7628 | 14.4419 | 5356 |
| windows-repaired-run | disk | replay-fresh | 10000 | 30 | 2.0828 | 2.9431 | 13.9342 | 5348 |
| windows-repaired-run | disk | replay-fresh | 100000 | 30 | 2.2861 | 2.9871 | 14.6777 | 5348 |
| windows-repaired-run | disk | replay-setup | 1000 | 1 | 23.5265 | 23.5265 | 36.0262 | 5484 |
| windows-repaired-run | disk | replay-setup | 10000 | 1 | 44.2933 | 44.2933 | 54.4759 | 5612 |
| windows-repaired-run | disk | replay-setup | 100000 | 1 | 35.6782 | 35.6782 | 50.3028 | 5580 |
| windows-repaired-run | disk | update-1 | 1000 | 30 | 14.0365 | 20.5028 | 25.8306 | 5452 |
| windows-repaired-run | disk | update-1 | 10000 | 30 | 15.9695 | 21.9710 | 28.3078 | 5416 |
| windows-repaired-run | disk | update-1 | 100000 | 30 | 17.9471 | 21.0504 | 30.1519 | 5392 |
| windows-repaired-run | disk | warm-repeat | 1000 | 30 | 43.8094 | 54.5100 | 56.0868 | 5728 |
| windows-repaired-run | disk | warm-repeat | 10000 | 30 | 52.1635 | 64.2442 | 63.0273 | 5724 |
| windows-repaired-run | disk | warm-repeat | 100000 | 30 | 61.3001 | 73.6875 | 71.8332 | 5708 |
| windows-repaired-run | disk | write-batch-64 | 1000 | 30 | 504.5322 | 602.2655 | 518.9200 | 5872 |
| windows-repaired-run | disk | write-batch-64 | 10000 | 30 | 589.0417 | 1217.8759 | 600.7404 | 5880 |
| windows-repaired-run | disk | write-batch-64 | 100000 | 30 | 724.2953 | 781.4936 | 737.0618 | 5868 |
| linux-repaired-run | chunk | broad-audit | 1000 | 1 | 11.7347 | 11.7347 | 12.5264 | 21040 |
| linux-repaired-run | chunk | broad-audit | 10000 | 1 | 43.0486 | 43.0486 | 43.8741 | 21040 |
| linux-repaired-run | chunk | broad-audit | 100000 | 1 | 320.6906 | 320.6906 | 322.8212 | 48236 |
| linux-repaired-run | chunk | insert-fresh | 1000 | 30 | 14.8434 | 19.6642 | 15.4246 | 21040 |
| linux-repaired-run | chunk | insert-fresh | 10000 | 30 | 31.1645 | 35.1369 | 31.8406 | 21040 |
| linux-repaired-run | chunk | insert-fresh | 100000 | 30 | 155.1902 | 168.2102 | 156.3335 | 25984 |
| linux-repaired-run | chunk | instrumentation-off | 1000 | 30 | 1.3647 | 1.6535 | 1.9157 | 21040 |
| linux-repaired-run | chunk | instrumentation-on | 1000 | 30 | 1.3724 | 1.6119 | 1.9189 | 21040 |
| linux-repaired-run | chunk | instrumentation-off | 10000 | 30 | 12.6457 | 13.9692 | 13.3404 | 21040 |
| linux-repaired-run | chunk | instrumentation-on | 10000 | 30 | 12.7733 | 13.5044 | 13.4944 | 21040 |
| linux-repaired-run | chunk | instrumentation-off | 100000 | 30 | 127.8347 | 133.3158 | 129.1356 | 25964 |
| linux-repaired-run | chunk | instrumentation-on | 100000 | 30 | 128.5981 | 134.2087 | 129.8294 | 25964 |
| linux-repaired-run | chunk | lookup-batch-64 | 1000 | 30 | 1.4426 | 1.6512 | 1.9997 | 21040 |
| linux-repaired-run | chunk | lookup-batch-64 | 10000 | 30 | 12.8836 | 13.2654 | 13.5881 | 21040 |
| linux-repaired-run | chunk | lookup-batch-64 | 100000 | 30 | 130.0481 | 136.9563 | 131.2278 | 25984 |
| linux-repaired-run | chunk | negative-fresh | 1000 | 30 | 1.3691 | 1.5648 | 1.9902 | 21040 |
| linux-repaired-run | chunk | negative-fresh | 10000 | 30 | 12.8017 | 13.8361 | 13.5128 | 21040 |
| linux-repaired-run | chunk | negative-fresh | 100000 | 30 | 129.4512 | 140.4965 | 130.6880 | 25964 |
| linux-repaired-run | chunk | positive-fresh | 1000 | 30 | 1.3838 | 1.5909 | 1.9651 | 21040 |
| linux-repaired-run | chunk | positive-fresh | 10000 | 30 | 12.6994 | 14.1879 | 13.4119 | 21040 |
| linux-repaired-run | chunk | positive-fresh | 100000 | 30 | 129.2000 | 141.5575 | 130.7467 | 25984 |
| linux-repaired-run | chunk | replay-fresh | 1000 | 30 | 2.8265 | 5.7487 | 3.3918 | 21040 |
| linux-repaired-run | chunk | replay-fresh | 10000 | 30 | 14.3433 | 15.4194 | 15.0696 | 21040 |
| linux-repaired-run | chunk | replay-fresh | 100000 | 30 | 130.6084 | 148.1639 | 131.9032 | 25964 |
| linux-repaired-run | chunk | replay-setup | 1000 | 1 | 16.0391 | 16.0391 | 16.7926 | 21040 |
| linux-repaired-run | chunk | replay-setup | 10000 | 1 | 34.4366 | 34.4366 | 35.1586 | 21040 |
| linux-repaired-run | chunk | replay-setup | 100000 | 1 | 160.9979 | 160.9979 | 162.0838 | 25772 |
| linux-repaired-run | chunk | update-1 | 1000 | 30 | 12.7891 | 17.7855 | 13.4017 | 21040 |
| linux-repaired-run | chunk | update-1 | 10000 | 30 | 28.7861 | 34.8755 | 29.4795 | 21040 |
| linux-repaired-run | chunk | update-1 | 100000 | 30 | 152.2035 | 158.5889 | 153.4869 | 25992 |
| linux-repaired-run | chunk | warm-repeat | 1000 | 30 | 1.3834 | 1.5322 | 1.9166 | 21040 |
| linux-repaired-run | chunk | warm-repeat | 10000 | 30 | 12.8513 | 13.5800 | 13.5493 | 21040 |
| linux-repaired-run | chunk | warm-repeat | 100000 | 30 | 129.4936 | 133.9354 | 130.6398 | 25964 |
| linux-repaired-run | chunk | write-batch-64 | 1000 | 30 | 13.8749 | 21.2292 | 14.4479 | 21040 |
| linux-repaired-run | chunk | write-batch-64 | 10000 | 30 | 29.7036 | 31.3422 | 30.4004 | 21040 |
| linux-repaired-run | chunk | write-batch-64 | 100000 | 30 | 152.3877 | 162.7293 | 153.7037 | 26000 |
| linux-repaired-run | disk | broad-audit | 1000 | 1 | 63.3758 | 63.3758 | 64.3707 | 22696 |
| linux-repaired-run | disk | broad-audit | 10000 | 1 | 104.1464 | 104.1464 | 105.4004 | 23272 |
| linux-repaired-run | disk | broad-audit | 100000 | 1 | 423.4570 | 423.4570 | 424.7291 | 23848 |
| linux-repaired-run | disk | insert-fresh | 1000 | 30 | 12.6614 | 17.0376 | 13.2866 | 21352 |
| linux-repaired-run | disk | insert-fresh | 10000 | 30 | 18.9410 | 25.8578 | 19.7839 | 22696 |
| linux-repaired-run | disk | insert-fresh | 100000 | 30 | 17.4892 | 23.8932 | 18.2259 | 23272 |
| linux-repaired-run | disk | instrumentation-off | 1000 | 30 | 0.2946 | 0.3430 | 0.9233 | 21352 |
| linux-repaired-run | disk | instrumentation-on | 1000 | 30 | 0.2958 | 0.3747 | 0.9113 | 21352 |
| linux-repaired-run | disk | instrumentation-off | 10000 | 30 | 0.3110 | 0.3505 | 1.0210 | 22696 |
| linux-repaired-run | disk | instrumentation-on | 10000 | 30 | 0.3153 | 0.3812 | 1.0128 | 22696 |
| linux-repaired-run | disk | instrumentation-off | 100000 | 30 | 0.3144 | 0.4097 | 0.9693 | 23272 |
| linux-repaired-run | disk | instrumentation-on | 100000 | 30 | 0.3092 | 0.3972 | 0.9762 | 23272 |
| linux-repaired-run | disk | lookup-batch-64 | 1000 | 30 | 10.7065 | 11.4788 | 11.4022 | 21352 |
| linux-repaired-run | disk | lookup-batch-64 | 10000 | 30 | 12.2590 | 13.2284 | 13.0653 | 22696 |
| linux-repaired-run | disk | lookup-batch-64 | 100000 | 30 | 12.9809 | 14.7104 | 13.7022 | 23272 |
| linux-repaired-run | disk | negative-fresh | 1000 | 30 | 0.2248 | 0.2654 | 0.8272 | 21352 |
| linux-repaired-run | disk | negative-fresh | 10000 | 30 | 0.1696 | 0.3123 | 0.8968 | 22696 |
| linux-repaired-run | disk | negative-fresh | 100000 | 30 | 0.2140 | 0.2828 | 0.8056 | 23272 |
| linux-repaired-run | disk | positive-fresh | 1000 | 30 | 0.3004 | 0.3595 | 0.8863 | 21352 |
| linux-repaired-run | disk | positive-fresh | 10000 | 30 | 0.3372 | 0.6123 | 1.0585 | 22696 |
| linux-repaired-run | disk | positive-fresh | 100000 | 30 | 0.3267 | 0.4635 | 0.9525 | 23272 |
| linux-repaired-run | disk | replay-fresh | 1000 | 30 | 1.6829 | 1.8865 | 2.2999 | 21352 |
| linux-repaired-run | disk | replay-fresh | 10000 | 30 | 1.8698 | 3.4508 | 2.6822 | 22696 |
| linux-repaired-run | disk | replay-fresh | 100000 | 30 | 1.6756 | 2.0358 | 2.3042 | 23272 |
| linux-repaired-run | disk | replay-setup | 1000 | 1 | 14.0245 | 14.0245 | 14.7527 | 21352 |
| linux-repaired-run | disk | replay-setup | 10000 | 1 | 20.1665 | 20.1665 | 21.1314 | 22696 |
| linux-repaired-run | disk | replay-setup | 100000 | 1 | 21.2929 | 21.2929 | 22.1363 | 23272 |
| linux-repaired-run | disk | update-1 | 1000 | 30 | 8.3912 | 9.9897 | 9.0229 | 21352 |
| linux-repaired-run | disk | update-1 | 10000 | 30 | 10.2776 | 14.1945 | 10.9876 | 22696 |
| linux-repaired-run | disk | update-1 | 100000 | 30 | 11.3409 | 15.7354 | 11.9305 | 23464 |
| linux-repaired-run | disk | warm-repeat | 1000 | 30 | 17.0715 | 18.5925 | 17.7402 | 21352 |
| linux-repaired-run | disk | warm-repeat | 10000 | 30 | 19.1438 | 22.7444 | 19.8547 | 22696 |
| linux-repaired-run | disk | warm-repeat | 100000 | 30 | 19.9230 | 20.3744 | 20.5129 | 23272 |
| linux-repaired-run | disk | write-batch-64 | 1000 | 30 | 235.7930 | 418.8038 | 236.3999 | 21352 |
| linux-repaired-run | disk | write-batch-64 | 10000 | 30 | 298.9074 | 344.5656 | 299.5773 | 22696 |
| linux-repaired-run | disk | write-batch-64 | 100000 | 30 | 386.9678 | 669.9761 | 387.6785 | 23464 |

Instrumentation overhead (paired command-time enabled/disabled ratios; alternating order):

| Platform | Variant | History size | Pairs | Median ratio | Median delta ms |
| --- | --- | ---: | ---: | ---: | ---: |
| windows | chunk | 1000 | 30 | 0.9677 | -0.1613 |
| windows | chunk | 10000 | 30 | 0.9813 | -0.8198 |
| windows | chunk | 100000 | 30 | 0.9889 | -4.4483 |
| windows | disk | 1000 | 30 | 0.9910 | -0.0121 |
| windows | disk | 10000 | 30 | 1.0089 | 0.0151 |
| windows | disk | 100000 | 30 | 1.0116 | 0.0194 |
| linux | chunk | 1000 | 30 | 1.0043 | 0.0059 |
| linux | chunk | 10000 | 30 | 1.0089 | 0.1133 |
| linux | chunk | 100000 | 30 | 1.0112 | 1.4451 |
| linux | disk | 1000 | 30 | 1.0196 | 0.0056 |
| linux | disk | 10000 | 30 | 0.9954 | -0.0014 |
| linux | disk | 100000 | 30 | 0.9876 | -0.0039 |
| windows-repaired-run | chunk | 1000 | 30 | 0.9802 | -0.1022 |
| windows-repaired-run | chunk | 10000 | 30 | 0.9986 | -0.0429 |
| windows-repaired-run | chunk | 100000 | 30 | 0.9888 | -3.2626 |
| windows-repaired-run | disk | 1000 | 30 | 1.0198 | 0.0274 |
| windows-repaired-run | disk | 10000 | 30 | 1.0102 | 0.0155 |
| windows-repaired-run | disk | 100000 | 30 | 1.0115 | 0.0186 |
| linux-repaired-run | chunk | 1000 | 30 | 1.0008 | 0.0011 |
| linux-repaired-run | chunk | 10000 | 30 | 1.0043 | 0.0532 |
| linux-repaired-run | chunk | 100000 | 30 | 1.0017 | 0.2182 |
| linux-repaired-run | disk | 1000 | 30 | 1.0045 | 0.0014 |
| linux-repaired-run | disk | 10000 | 30 | 1.0266 | 0.0072 |
| linux-repaired-run | disk | 100000 | 30 | 0.9853 | -0.0046 |

Sibling enumeration stress (smallest history only):

| Platform | Variant | History size | Siblings | Guard | n | Median ms | p95 ms | Listed entries median |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | ---: |
| windows | chunk | 1000 | 0 | exact | 30 | 5.2095 | 7.2011 | 0.0 |
| windows | chunk | 1000 | 0 | list | 30 | 7.6775 | 10.4322 | 0.0 |
| windows | chunk | 1000 | 1000 | exact | 30 | 5.6580 | 11.4944 | 0.0 |
| windows | chunk | 1000 | 1000 | list | 30 | 31.1129 | 54.4752 | 56000.0 |
| windows | chunk | 1000 | 10000 | exact | 30 | 5.5030 | 6.9369 | 0.0 |
| windows | chunk | 1000 | 10000 | list | 30 | 228.3253 | 268.3848 | 560000.0 |
| windows | disk | 1000 | 0 | exact | 30 | 1.3958 | 1.5046 | 0.0 |
| windows | disk | 1000 | 0 | list | 30 | 1.8212 | 1.9980 | 0.0 |
| windows | disk | 1000 | 1000 | exact | 30 | 1.4768 | 2.5086 | 0.0 |
| windows | disk | 1000 | 1000 | list | 30 | 6.4004 | 8.8524 | 14000.0 |
| windows | disk | 1000 | 10000 | exact | 30 | 1.4357 | 1.6725 | 0.0 |
| windows | disk | 1000 | 10000 | list | 30 | 44.1683 | 48.0646 | 140000.0 |
| linux | chunk | 1000 | 0 | exact | 30 | 1.3155 | 1.4959 | 0.0 |
| linux | chunk | 1000 | 0 | list | 30 | 1.4453 | 1.6042 | 0.0 |
| linux | chunk | 1000 | 1000 | exact | 30 | 1.3665 | 1.5666 | 0.0 |
| linux | chunk | 1000 | 1000 | list | 30 | 9.8627 | 10.5389 | 56000.0 |
| linux | chunk | 1000 | 10000 | exact | 30 | 1.4165 | 1.4946 | 0.0 |
| linux | chunk | 1000 | 10000 | list | 30 | 86.1060 | 88.9351 | 560000.0 |
| linux | disk | 1000 | 0 | exact | 30 | 0.2879 | 0.4204 | 0.0 |
| linux | disk | 1000 | 0 | list | 30 | 0.3360 | 0.4271 | 0.0 |
| linux | disk | 1000 | 1000 | exact | 30 | 0.2860 | 0.3572 | 0.0 |
| linux | disk | 1000 | 1000 | list | 30 | 2.4510 | 2.6682 | 14000.0 |
| linux | disk | 1000 | 10000 | exact | 30 | 0.2843 | 0.3735 | 0.0 |
| linux | disk | 1000 | 10000 | list | 30 | 21.1527 | 21.8520 | 140000.0 |
| windows-repaired-run | chunk | 1000 | 0 | exact | 30 | 4.8335 | 6.2454 | 0.0 |
| windows-repaired-run | chunk | 1000 | 0 | list | 30 | 6.9131 | 10.1246 | 0.0 |
| windows-repaired-run | chunk | 1000 | 1000 | exact | 30 | 4.9679 | 7.0701 | 0.0 |
| windows-repaired-run | chunk | 1000 | 1000 | list | 30 | 25.8280 | 39.1282 | 56000.0 |
| windows-repaired-run | chunk | 1000 | 10000 | exact | 30 | 4.4422 | 6.1653 | 0.0 |
| windows-repaired-run | chunk | 1000 | 10000 | list | 30 | 172.7829 | 213.6068 | 560000.0 |
| windows-repaired-run | disk | 1000 | 0 | exact | 30 | 1.3928 | 1.6546 | 0.0 |
| windows-repaired-run | disk | 1000 | 0 | list | 30 | 1.8572 | 2.2054 | 0.0 |
| windows-repaired-run | disk | 1000 | 1000 | exact | 30 | 1.4548 | 1.7502 | 0.0 |
| windows-repaired-run | disk | 1000 | 1000 | list | 30 | 6.1494 | 7.4549 | 14000.0 |
| windows-repaired-run | disk | 1000 | 10000 | exact | 30 | 1.3829 | 1.6574 | 0.0 |
| windows-repaired-run | disk | 1000 | 10000 | list | 30 | 43.4319 | 52.4304 | 140000.0 |
| linux-repaired-run | chunk | 1000 | 0 | exact | 30 | 1.3668 | 1.6120 | 0.0 |
| linux-repaired-run | chunk | 1000 | 0 | list | 30 | 1.5020 | 1.7322 | 0.0 |
| linux-repaired-run | chunk | 1000 | 1000 | exact | 30 | 1.4877 | 1.9795 | 0.0 |
| linux-repaired-run | chunk | 1000 | 1000 | list | 30 | 9.8756 | 11.4799 | 56000.0 |
| linux-repaired-run | chunk | 1000 | 10000 | exact | 30 | 1.4152 | 1.5443 | 0.0 |
| linux-repaired-run | chunk | 1000 | 10000 | list | 30 | 83.5069 | 87.5889 | 560000.0 |
| linux-repaired-run | disk | 1000 | 0 | exact | 30 | 0.2998 | 0.3480 | 0.0 |
| linux-repaired-run | disk | 1000 | 0 | list | 30 | 0.3315 | 0.3888 | 0.0 |
| linux-repaired-run | disk | 1000 | 1000 | exact | 30 | 0.3069 | 0.6737 | 0.0 |
| linux-repaired-run | disk | 1000 | 1000 | list | 30 | 2.4867 | 4.2308 | 14000.0 |
| linux-repaired-run | disk | 1000 | 10000 | exact | 30 | 0.3453 | 0.7926 | 0.0 |
| linux-repaired-run | disk | 1000 | 10000 | list | 30 | 22.8805 | 39.9739 | 140000.0 |

| Platform | Passes | Failures | Skips |
| --- | ---: | ---: | ---: |
| windows | 5898 | 0 | 0 |
| linux | 5912 | 0 | 0 |
| windows-supplement | 984 | 0 | 0 |
| windows-agents | 428 | 0 | 0 |
| linux-agents | 428 | 0 | 0 |
| windows-repair | 116 | 0 | 0 |
| linux-repair | 116 | 0 | 0 |
| windows-repair-final | 116 | 0 | 0 |
| linux-repair-final | 116 | 0 | 0 |
| windows-repaired-run | 5924 | 0 | 0 |
| linux-repaired-run | 5912 | 0 | 0 |

Unavailable metrics and fault/cache limits remain explicit in summary.json and environment.json. Raw responses and each assertion are in the input evidence directories.
