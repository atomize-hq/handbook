# Observed prototype measurements

Fresh processes use an uncontrolled OS cache. These are isolated prototype results, not production capacity or power-loss qualification.

| Platform | Variant | Case | History size | n | Command median ms | Command p95 ms | Process median ms | Max RSS KiB |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| windows-tx-growth-20261012a | chunk | broad-audit | 1000 | 1 | 90.1683 | 90.1683 | 101.6707 | 6068 |
| windows-tx-growth-20261012a | chunk | broad-audit | 10000 | 1 | 268.5915 | 268.5915 | 281.4261 | 10416 |
| windows-tx-growth-20261012a | chunk | broad-audit | 100000 | 1 | 1199.8749 | 1199.8749 | 1211.3547 | 52032 |
| windows-tx-growth-20261012a | chunk | insert-fresh | 1000 | 30 | 24.4260 | 29.1616 | 36.1132 | 5848 |
| windows-tx-growth-20261012a | chunk | insert-fresh | 10000 | 30 | 58.5700 | 66.0652 | 71.6014 | 8348 |
| windows-tx-growth-20261012a | chunk | insert-fresh | 100000 | 30 | 386.8370 | 451.2829 | 398.6116 | 29524 |
| windows-tx-growth-20261012a | chunk | instrumentation-off | 1000 | 30 | 5.2034 | 6.8613 | 17.6310 | 5756 |
| windows-tx-growth-20261012a | chunk | instrumentation-on | 1000 | 30 | 5.2256 | 5.8700 | 18.2436 | 5760 |
| windows-tx-growth-20261012a | chunk | instrumentation-off | 10000 | 30 | 34.1776 | 42.9040 | 44.5138 | 8240 |
| windows-tx-growth-20261012a | chunk | instrumentation-on | 10000 | 30 | 35.1420 | 42.2094 | 45.9676 | 8264 |
| windows-tx-growth-20261012a | chunk | instrumentation-off | 100000 | 30 | 301.2919 | 336.2586 | 311.4759 | 29432 |
| windows-tx-growth-20261012a | chunk | instrumentation-on | 100000 | 30 | 299.7804 | 366.9804 | 310.5387 | 29312 |
| windows-tx-growth-20261012a | chunk | lookup-batch-64 | 1000 | 30 | 5.1579 | 6.5162 | 17.8128 | 5772 |
| windows-tx-growth-20261012a | chunk | lookup-batch-64 | 10000 | 30 | 34.3796 | 41.5837 | 45.1070 | 8280 |
| windows-tx-growth-20261012a | chunk | lookup-batch-64 | 100000 | 30 | 300.9132 | 317.9520 | 311.7364 | 29424 |
| windows-tx-growth-20261012a | chunk | negative-fresh | 1000 | 30 | 5.0664 | 5.7116 | 16.7429 | 5748 |
| windows-tx-growth-20261012a | chunk | negative-fresh | 10000 | 30 | 35.4571 | 56.4959 | 47.8207 | 8276 |
| windows-tx-growth-20261012a | chunk | negative-fresh | 100000 | 30 | 358.2998 | 377.8205 | 372.6578 | 29472 |
| windows-tx-growth-20261012a | chunk | positive-fresh | 1000 | 30 | 5.3120 | 6.1518 | 16.8910 | 5784 |
| windows-tx-growth-20261012a | chunk | positive-fresh | 10000 | 30 | 34.4455 | 44.6719 | 46.5620 | 8292 |
| windows-tx-growth-20261012a | chunk | positive-fresh | 100000 | 30 | 360.4819 | 416.5287 | 374.6722 | 29492 |
| windows-tx-growth-20261012a | chunk | replay-fresh | 1000 | 30 | 5.8158 | 7.6129 | 17.5065 | 5772 |
| windows-tx-growth-20261012a | chunk | replay-fresh | 10000 | 30 | 37.1359 | 44.7959 | 48.8183 | 8356 |
| windows-tx-growth-20261012a | chunk | replay-fresh | 100000 | 30 | 356.5211 | 434.7669 | 370.2871 | 29428 |
| windows-tx-growth-20261012a | chunk | replay-setup | 1000 | 1 | 25.6293 | 25.6293 | 38.4560 | 5792 |
| windows-tx-growth-20261012a | chunk | replay-setup | 10000 | 1 | 54.2961 | 54.2961 | 64.8286 | 8144 |
| windows-tx-growth-20261012a | chunk | replay-setup | 100000 | 1 | 418.0146 | 418.0146 | 429.0024 | 29428 |
| windows-tx-growth-20261012a | chunk | update-1 | 1000 | 30 | 21.8581 | 25.5253 | 34.2201 | 5868 |
| windows-tx-growth-20261012a | chunk | update-1 | 10000 | 30 | 61.0906 | 108.4912 | 76.8254 | 8312 |
| windows-tx-growth-20261012a | chunk | update-1 | 100000 | 30 | 368.4724 | 386.0051 | 380.6818 | 29420 |
| windows-tx-growth-20261012a | chunk | warm-repeat | 1000 | 30 | 5.2736 | 6.0292 | 19.5258 | 5764 |
| windows-tx-growth-20261012a | chunk | warm-repeat | 10000 | 30 | 34.0425 | 39.3894 | 44.9053 | 8228 |
| windows-tx-growth-20261012a | chunk | warm-repeat | 100000 | 30 | 300.9500 | 348.8588 | 311.2335 | 29492 |
| windows-tx-growth-20261012a | chunk | write-batch-64 | 1000 | 30 | 22.5110 | 25.9245 | 36.6801 | 5916 |
| windows-tx-growth-20261012a | chunk | write-batch-64 | 10000 | 30 | 65.3338 | 101.9717 | 80.3616 | 8368 |
| windows-tx-growth-20261012a | chunk | write-batch-64 | 100000 | 30 | 397.2951 | 434.3144 | 410.3730 | 29504 |
| windows-tx-growth-20261012a | disk | broad-audit | 1000 | 1 | 85.2217 | 85.2217 | 98.3924 | 5708 |
| windows-tx-growth-20261012a | disk | broad-audit | 10000 | 1 | 145.9457 | 145.9457 | 157.3727 | 5728 |
| windows-tx-growth-20261012a | disk | broad-audit | 100000 | 1 | 915.8290 | 915.8290 | 926.7578 | 5972 |
| windows-tx-growth-20261012a | disk | insert-fresh | 1000 | 30 | 16.9486 | 19.2269 | 29.3827 | 5824 |
| windows-tx-growth-20261012a | disk | insert-fresh | 10000 | 30 | 20.2047 | 25.0539 | 34.1000 | 5740 |
| windows-tx-growth-20261012a | disk | insert-fresh | 100000 | 30 | 22.3464 | 27.0653 | 35.6356 | 5908 |
| windows-tx-growth-20261012a | disk | instrumentation-off | 1000 | 30 | 1.6669 | 2.2995 | 13.8430 | 5432 |
| windows-tx-growth-20261012a | disk | instrumentation-on | 1000 | 30 | 1.6468 | 2.1676 | 13.8877 | 5436 |
| windows-tx-growth-20261012a | disk | instrumentation-off | 10000 | 30 | 1.7576 | 2.8931 | 12.6169 | 5460 |
| windows-tx-growth-20261012a | disk | instrumentation-on | 10000 | 30 | 1.7698 | 2.0993 | 12.5524 | 5464 |
| windows-tx-growth-20261012a | disk | instrumentation-off | 100000 | 30 | 1.9953 | 2.2774 | 14.7858 | 5464 |
| windows-tx-growth-20261012a | disk | instrumentation-on | 100000 | 30 | 1.9943 | 2.4899 | 15.0353 | 5464 |
| windows-tx-growth-20261012a | disk | lookup-batch-64 | 1000 | 30 | 29.1770 | 37.7627 | 40.9738 | 5764 |
| windows-tx-growth-20261012a | disk | lookup-batch-64 | 10000 | 30 | 35.7261 | 46.3877 | 46.4188 | 5740 |
| windows-tx-growth-20261012a | disk | lookup-batch-64 | 100000 | 30 | 48.8340 | 56.1334 | 61.6730 | 5788 |
| windows-tx-growth-20261012a | disk | negative-fresh | 1000 | 30 | 1.6275 | 2.0320 | 13.6387 | 5312 |
| windows-tx-growth-20261012a | disk | negative-fresh | 10000 | 30 | 1.6321 | 2.1118 | 14.3039 | 5156 |
| windows-tx-growth-20261012a | disk | negative-fresh | 100000 | 30 | 1.8490 | 2.4066 | 14.4348 | 5308 |
| windows-tx-growth-20261012a | disk | positive-fresh | 1000 | 30 | 1.7094 | 1.8910 | 13.6767 | 5428 |
| windows-tx-growth-20261012a | disk | positive-fresh | 10000 | 30 | 1.8599 | 2.1967 | 14.8526 | 5460 |
| windows-tx-growth-20261012a | disk | positive-fresh | 100000 | 30 | 1.9454 | 2.1836 | 14.5319 | 5464 |
| windows-tx-growth-20261012a | disk | replay-fresh | 1000 | 30 | 2.7359 | 13.6145 | 14.9571 | 5576 |
| windows-tx-growth-20261012a | disk | replay-fresh | 10000 | 30 | 3.0357 | 3.3953 | 16.1027 | 5624 |
| windows-tx-growth-20261012a | disk | replay-fresh | 100000 | 30 | 3.2742 | 4.0293 | 15.1325 | 5544 |
| windows-tx-growth-20261012a | disk | replay-setup | 1000 | 1 | 22.7542 | 22.7542 | 34.7537 | 5648 |
| windows-tx-growth-20261012a | disk | replay-setup | 10000 | 1 | 23.7699 | 23.7699 | 37.6288 | 5772 |
| windows-tx-growth-20261012a | disk | replay-setup | 100000 | 1 | 25.8681 | 25.8681 | 36.9886 | 5572 |
| windows-tx-growth-20261012a | disk | update-1 | 1000 | 30 | 13.9258 | 17.0408 | 24.1310 | 5436 |
| windows-tx-growth-20261012a | disk | update-1 | 10000 | 30 | 15.2855 | 18.0577 | 25.7643 | 5408 |
| windows-tx-growth-20261012a | disk | update-1 | 100000 | 30 | 17.2755 | 25.0734 | 27.9644 | 5500 |
| windows-tx-growth-20261012a | disk | warm-repeat | 1000 | 30 | 46.1136 | 54.9035 | 59.1262 | 5748 |
| windows-tx-growth-20261012a | disk | warm-repeat | 10000 | 30 | 54.9542 | 64.7302 | 65.6091 | 5784 |
| windows-tx-growth-20261012a | disk | warm-repeat | 100000 | 30 | 76.5880 | 89.5516 | 90.6219 | 5756 |
| windows-tx-growth-20261012a | disk | write-batch-64 | 1000 | 30 | 15.3697 | 19.9358 | 25.6180 | 5768 |
| windows-tx-growth-20261012a | disk | write-batch-64 | 10000 | 30 | 17.4069 | 22.5338 | 28.0448 | 5772 |
| windows-tx-growth-20261012a | disk | write-batch-64 | 100000 | 30 | 19.0924 | 23.7671 | 30.1110 | 5804 |
| linux-tx-growth-20261012a | chunk | broad-audit | 1000 | 1 | 12.9084 | 12.9084 | 14.1082 | 21064 |
| linux-tx-growth-20261012a | chunk | broad-audit | 10000 | 1 | 44.7468 | 44.7468 | 45.5409 | 21064 |
| linux-tx-growth-20261012a | chunk | broad-audit | 100000 | 1 | 315.1743 | 315.1743 | 317.0320 | 48400 |
| linux-tx-growth-20261012a | chunk | insert-fresh | 1000 | 30 | 21.1961 | 25.5863 | 21.8966 | 21064 |
| linux-tx-growth-20261012a | chunk | insert-fresh | 10000 | 30 | 29.8877 | 35.1230 | 30.5773 | 21064 |
| linux-tx-growth-20261012a | chunk | insert-fresh | 100000 | 30 | 158.2284 | 177.1047 | 159.4561 | 26008 |
| linux-tx-growth-20261012a | chunk | instrumentation-off | 1000 | 30 | 1.3827 | 1.5595 | 1.9562 | 21064 |
| linux-tx-growth-20261012a | chunk | instrumentation-on | 1000 | 30 | 1.3761 | 1.5390 | 1.9660 | 21064 |
| linux-tx-growth-20261012a | chunk | instrumentation-off | 10000 | 30 | 13.2259 | 13.7486 | 13.8725 | 21064 |
| linux-tx-growth-20261012a | chunk | instrumentation-on | 10000 | 30 | 12.9942 | 13.5245 | 13.7688 | 21064 |
| linux-tx-growth-20261012a | chunk | instrumentation-off | 100000 | 30 | 129.4548 | 141.2796 | 130.5630 | 25960 |
| linux-tx-growth-20261012a | chunk | instrumentation-on | 100000 | 30 | 129.4980 | 137.7684 | 130.8072 | 25996 |
| linux-tx-growth-20261012a | chunk | lookup-batch-64 | 1000 | 30 | 1.4247 | 1.6278 | 2.0101 | 21064 |
| linux-tx-growth-20261012a | chunk | lookup-batch-64 | 10000 | 30 | 13.0494 | 13.7745 | 13.7708 | 21064 |
| linux-tx-growth-20261012a | chunk | lookup-batch-64 | 100000 | 30 | 129.4085 | 135.0957 | 130.6116 | 25968 |
| linux-tx-growth-20261012a | chunk | negative-fresh | 1000 | 30 | 1.3999 | 1.5479 | 2.0396 | 21064 |
| linux-tx-growth-20261012a | chunk | negative-fresh | 10000 | 30 | 12.8861 | 13.4409 | 13.5832 | 21064 |
| linux-tx-growth-20261012a | chunk | negative-fresh | 100000 | 30 | 132.8012 | 142.6165 | 133.9229 | 25944 |
| linux-tx-growth-20261012a | chunk | positive-fresh | 1000 | 30 | 1.3870 | 1.5016 | 2.0263 | 21064 |
| linux-tx-growth-20261012a | chunk | positive-fresh | 10000 | 30 | 12.8234 | 13.6609 | 13.5536 | 21064 |
| linux-tx-growth-20261012a | chunk | positive-fresh | 100000 | 30 | 131.9742 | 147.0280 | 133.1954 | 25944 |
| linux-tx-growth-20261012a | chunk | replay-fresh | 1000 | 30 | 3.0228 | 4.0496 | 3.6394 | 21064 |
| linux-tx-growth-20261012a | chunk | replay-fresh | 10000 | 30 | 14.6130 | 15.8238 | 15.3542 | 21064 |
| linux-tx-growth-20261012a | chunk | replay-fresh | 100000 | 30 | 134.8933 | 151.3398 | 136.0203 | 25944 |
| linux-tx-growth-20261012a | chunk | replay-setup | 1000 | 1 | 24.1932 | 24.1932 | 24.9289 | 21064 |
| linux-tx-growth-20261012a | chunk | replay-setup | 10000 | 1 | 30.6156 | 30.6156 | 31.3888 | 21064 |
| linux-tx-growth-20261012a | chunk | replay-setup | 100000 | 1 | 159.8062 | 159.8062 | 161.1292 | 25752 |
| linux-tx-growth-20261012a | chunk | update-1 | 1000 | 30 | 11.7919 | 16.5309 | 12.3522 | 21064 |
| linux-tx-growth-20261012a | chunk | update-1 | 10000 | 30 | 28.8537 | 122.3793 | 29.5619 | 21064 |
| linux-tx-growth-20261012a | chunk | update-1 | 100000 | 30 | 153.7022 | 160.7744 | 154.8708 | 26008 |
| linux-tx-growth-20261012a | chunk | warm-repeat | 1000 | 30 | 1.3942 | 1.4832 | 1.9788 | 21064 |
| linux-tx-growth-20261012a | chunk | warm-repeat | 10000 | 30 | 13.0775 | 13.9361 | 13.7529 | 21064 |
| linux-tx-growth-20261012a | chunk | warm-repeat | 100000 | 30 | 128.8213 | 133.2009 | 130.1134 | 25948 |
| linux-tx-growth-20261012a | chunk | write-batch-64 | 1000 | 30 | 12.9075 | 18.4819 | 13.5292 | 21064 |
| linux-tx-growth-20261012a | chunk | write-batch-64 | 10000 | 30 | 29.5862 | 184.6026 | 30.3951 | 21064 |
| linux-tx-growth-20261012a | chunk | write-batch-64 | 100000 | 30 | 154.1253 | 159.2476 | 155.3361 | 25944 |
| linux-tx-growth-20261012a | disk | broad-audit | 1000 | 1 | 7.1375 | 7.1375 | 7.8384 | 21436 |
| linux-tx-growth-20261012a | disk | broad-audit | 10000 | 1 | 33.7995 | 33.7995 | 34.5893 | 21436 |
| linux-tx-growth-20261012a | disk | broad-audit | 100000 | 1 | 282.3554 | 282.3554 | 283.2179 | 22396 |
| linux-tx-growth-20261012a | disk | insert-fresh | 1000 | 30 | 9.1072 | 13.1465 | 9.6785 | 21244 |
| linux-tx-growth-20261012a | disk | insert-fresh | 10000 | 30 | 10.4805 | 14.1712 | 11.0436 | 21436 |
| linux-tx-growth-20261012a | disk | insert-fresh | 100000 | 30 | 12.7658 | 16.8669 | 13.3458 | 22396 |
| linux-tx-growth-20261012a | disk | instrumentation-off | 1000 | 30 | 0.2889 | 0.4029 | 0.8541 | 21244 |
| linux-tx-growth-20261012a | disk | instrumentation-on | 1000 | 30 | 0.2900 | 0.3481 | 0.8607 | 21244 |
| linux-tx-growth-20261012a | disk | instrumentation-off | 10000 | 30 | 0.2993 | 0.3862 | 0.8960 | 21436 |
| linux-tx-growth-20261012a | disk | instrumentation-on | 10000 | 30 | 0.3001 | 0.3584 | 0.8848 | 21436 |
| linux-tx-growth-20261012a | disk | instrumentation-off | 100000 | 30 | 0.3195 | 0.3610 | 0.9173 | 22396 |
| linux-tx-growth-20261012a | disk | instrumentation-on | 100000 | 30 | 0.3260 | 0.4108 | 0.9224 | 22396 |
| linux-tx-growth-20261012a | disk | lookup-batch-64 | 1000 | 30 | 10.2879 | 10.9121 | 10.9218 | 21244 |
| linux-tx-growth-20261012a | disk | lookup-batch-64 | 10000 | 30 | 11.4525 | 12.0469 | 12.0992 | 21436 |
| linux-tx-growth-20261012a | disk | lookup-batch-64 | 100000 | 30 | 12.9469 | 14.2593 | 13.6755 | 22396 |
| linux-tx-growth-20261012a | disk | negative-fresh | 1000 | 30 | 0.2172 | 0.2991 | 0.7424 | 21244 |
| linux-tx-growth-20261012a | disk | negative-fresh | 10000 | 30 | 0.1538 | 0.1895 | 0.7272 | 21436 |
| linux-tx-growth-20261012a | disk | negative-fresh | 100000 | 30 | 0.2259 | 0.2456 | 0.8154 | 22396 |
| linux-tx-growth-20261012a | disk | positive-fresh | 1000 | 30 | 0.2946 | 0.3449 | 0.8459 | 21244 |
| linux-tx-growth-20261012a | disk | positive-fresh | 10000 | 30 | 0.3056 | 0.3793 | 0.8879 | 21436 |
| linux-tx-growth-20261012a | disk | positive-fresh | 100000 | 30 | 0.3308 | 0.4050 | 0.9353 | 22396 |
| linux-tx-growth-20261012a | disk | replay-fresh | 1000 | 30 | 1.7107 | 1.9371 | 2.2967 | 21244 |
| linux-tx-growth-20261012a | disk | replay-fresh | 10000 | 30 | 1.6765 | 1.8962 | 2.2363 | 21436 |
| linux-tx-growth-20261012a | disk | replay-fresh | 100000 | 30 | 1.9281 | 2.9743 | 2.4983 | 22396 |
| linux-tx-growth-20261012a | disk | replay-setup | 1000 | 1 | 10.4189 | 10.4189 | 11.0487 | 21244 |
| linux-tx-growth-20261012a | disk | replay-setup | 10000 | 1 | 13.2690 | 13.2690 | 13.9791 | 21436 |
| linux-tx-growth-20261012a | disk | replay-setup | 100000 | 1 | 18.9237 | 18.9237 | 19.6998 | 22396 |
| linux-tx-growth-20261012a | disk | update-1 | 1000 | 30 | 8.2457 | 10.1372 | 8.8175 | 21244 |
| linux-tx-growth-20261012a | disk | update-1 | 10000 | 30 | 9.8206 | 12.2151 | 10.4381 | 21436 |
| linux-tx-growth-20261012a | disk | update-1 | 100000 | 30 | 11.5150 | 12.8630 | 12.0955 | 22396 |
| linux-tx-growth-20261012a | disk | warm-repeat | 1000 | 30 | 16.3571 | 18.3997 | 16.9869 | 21244 |
| linux-tx-growth-20261012a | disk | warm-repeat | 10000 | 30 | 18.1463 | 19.4737 | 18.7158 | 21436 |
| linux-tx-growth-20261012a | disk | warm-repeat | 100000 | 30 | 19.4572 | 21.2027 | 20.1286 | 22396 |
| linux-tx-growth-20261012a | disk | write-batch-64 | 1000 | 30 | 10.0673 | 11.5230 | 10.6231 | 21244 |
| linux-tx-growth-20261012a | disk | write-batch-64 | 10000 | 30 | 11.3928 | 15.1460 | 11.9542 | 21436 |
| linux-tx-growth-20261012a | disk | write-batch-64 | 100000 | 30 | 13.4143 | 18.9698 | 14.0987 | 22396 |

Instrumentation overhead (paired command-time enabled/disabled ratios; alternating order):

| Platform | Variant | History size | Pairs | Median ratio | Median delta ms |
| --- | --- | ---: | ---: | ---: | ---: |
| windows-tx-growth-20261012a | chunk | 1000 | 30 | 0.9928 | -0.0365 |
| windows-tx-growth-20261012a | chunk | 10000 | 30 | 1.0320 | 1.0679 |
| windows-tx-growth-20261012a | chunk | 100000 | 30 | 1.0007 | 0.2122 |
| windows-tx-growth-20261012a | disk | 1000 | 30 | 0.9934 | -0.0112 |
| windows-tx-growth-20261012a | disk | 10000 | 30 | 1.0083 | 0.0138 |
| windows-tx-growth-20261012a | disk | 100000 | 30 | 0.9804 | -0.0391 |
| linux-tx-growth-20261012a | chunk | 1000 | 30 | 0.9989 | -0.0015 |
| linux-tx-growth-20261012a | chunk | 10000 | 30 | 0.9935 | -0.0827 |
| linux-tx-growth-20261012a | chunk | 100000 | 30 | 0.9987 | -0.1594 |
| linux-tx-growth-20261012a | disk | 1000 | 30 | 0.9904 | -0.0028 |
| linux-tx-growth-20261012a | disk | 10000 | 30 | 0.9981 | -0.0004 |
| linux-tx-growth-20261012a | disk | 100000 | 30 | 0.9926 | -0.0026 |

Sibling enumeration stress (smallest history only):

| Platform | Variant | History size | Siblings | Guard | n | Median ms | p95 ms | Listed entries median |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | ---: |
| windows-tx-growth-20261012a | chunk | 1000 | 0 | exact | 30 | 5.1006 | 6.5647 | 0.0 |
| windows-tx-growth-20261012a | chunk | 1000 | 0 | list | 30 | 7.3990 | 8.2234 | 0.0 |
| windows-tx-growth-20261012a | chunk | 1000 | 1000 | exact | 30 | 5.1814 | 6.5027 | 0.0 |
| windows-tx-growth-20261012a | chunk | 1000 | 1000 | list | 30 | 26.4291 | 31.1646 | 57000.0 |
| windows-tx-growth-20261012a | chunk | 1000 | 10000 | exact | 30 | 4.9505 | 6.1802 | 0.0 |
| windows-tx-growth-20261012a | chunk | 1000 | 10000 | list | 30 | 180.7383 | 227.4477 | 570000.0 |
| windows-tx-growth-20261012a | disk | 1000 | 0 | exact | 30 | 1.6867 | 2.1344 | 0.0 |
| windows-tx-growth-20261012a | disk | 1000 | 0 | list | 30 | 2.2761 | 2.8137 | 0.0 |
| windows-tx-growth-20261012a | disk | 1000 | 1000 | exact | 30 | 1.6878 | 2.1089 | 0.0 |
| windows-tx-growth-20261012a | disk | 1000 | 1000 | list | 30 | 7.3456 | 7.9772 | 15000.0 |
| windows-tx-growth-20261012a | disk | 1000 | 10000 | exact | 30 | 1.7046 | 2.1411 | 0.0 |
| windows-tx-growth-20261012a | disk | 1000 | 10000 | list | 30 | 52.4549 | 60.4239 | 150000.0 |
| linux-tx-growth-20261012a | chunk | 1000 | 0 | exact | 30 | 1.3403 | 1.4278 | 0.0 |
| linux-tx-growth-20261012a | chunk | 1000 | 0 | list | 30 | 1.4735 | 1.6269 | 0.0 |
| linux-tx-growth-20261012a | chunk | 1000 | 1000 | exact | 30 | 1.3930 | 1.6525 | 0.0 |
| linux-tx-growth-20261012a | chunk | 1000 | 1000 | list | 30 | 9.7950 | 10.5669 | 57000.0 |
| linux-tx-growth-20261012a | chunk | 1000 | 10000 | exact | 30 | 1.4730 | 1.8121 | 0.0 |
| linux-tx-growth-20261012a | chunk | 1000 | 10000 | list | 30 | 86.4812 | 93.1283 | 570000.0 |
| linux-tx-growth-20261012a | disk | 1000 | 0 | exact | 30 | 0.2855 | 0.3814 | 0.0 |
| linux-tx-growth-20261012a | disk | 1000 | 0 | list | 30 | 0.3144 | 0.3465 | 0.0 |
| linux-tx-growth-20261012a | disk | 1000 | 1000 | exact | 30 | 0.2841 | 0.3086 | 0.0 |
| linux-tx-growth-20261012a | disk | 1000 | 1000 | list | 30 | 2.5312 | 2.6096 | 15000.0 |
| linux-tx-growth-20261012a | disk | 1000 | 10000 | exact | 30 | 0.2955 | 0.3719 | 0.0 |
| linux-tx-growth-20261012a | disk | 1000 | 10000 | list | 30 | 22.6312 | 23.0403 | 150000.0 |

| Platform | Passes | Failures | Skips |
| --- | ---: | ---: | ---: |
| windows-tx-growth-20261012a | 5924 | 0 | 0 |
| linux-tx-growth-20261012a | 5912 | 0 | 0 |

Unavailable metrics and fault/cache limits remain explicit in summary.json and environment.json. Raw responses and each assertion are in the input evidence directories.
