# Observed process latency

Controller time includes process creation, request delivery, response serialization/collection and process exit. Engine clock ends before final stdout serialization. These are one-attempt serial observations, separate from retry-inclusive contention request latency.

| Platform | Backend | Documents | Mode | Operation | Engine ms | Observed process ms | Measurement phase |
| --- | --- | ---: | --- | --- | ---: | ---: | --- |
| windows | cow | 1,000 | A | initialization | 6,859.40 | 6,867.98 | measured matrix |
| windows | cow | 1,000 | A | commit without packet | 107.66 | 113.60 | measured matrix |
| windows | cow | 1,000 | A | commit with packet | 128.39 | 134.06 | measured matrix |
| windows | cow | 1,000 | A | selected packet assembly | 37.61 | 43.25 | measured matrix |
| windows | cow | 1,000 | B | initialization | 5,056.73 | 5,064.46 | measured matrix |
| windows | cow | 1,000 | B | commit without packet | 1,156.78 | 1,163.09 | measured matrix |
| windows | cow | 1,000 | B | commit with packet | 1,210.06 | 1,216.42 | measured matrix |
| windows | cow | 1,000 | B | selected packet assembly | 57.89 | 63.37 | measured matrix |
| windows | cow | 10,000 | A | initialization | 99,169.99 | 99,178.94 | measured matrix |
| windows | cow | 10,000 | A | commit without packet | 758.49 | 765.06 | measured matrix |
| windows | cow | 10,000 | A | commit with packet | 5,842.63 | 5,848.64 | measured matrix |
| windows | cow | 10,000 | A | selected packet assembly | 5,220.07 | 5,226.41 | measured matrix |
| windows | cow | 10,000 | B | initialization | 57,997.93 | 58,008.41 | measured matrix |
| windows | cow | 10,000 | B | commit without packet | 11,924.87 | 11,931.67 | measured matrix |
| windows | cow | 10,000 | B | commit with packet | 11,866.35 | 11,872.83 | measured matrix |
| windows | cow | 10,000 | B | selected packet assembly | 6,440.05 | 6,448.51 | measured matrix |
| windows | cow | 100,000 | A | initialization | 1,260,276.74 | 1,260,296.26 | measured matrix |
| windows | cow | 100,000 | A | commit with packet | 17,404.44 | 17,416.32 | measured matrix |
| windows | cow | 100,000 | A | selected packet assembly | 6,081.76 | 6,088.92 | measured matrix |
| windows | cow | 100,000 | B | initialization | 1,015,635.26 | 1,015,653.76 | measured matrix |
| windows | cow | 100,000 | B | commit with packet | 141,302.91 | 141,315.46 | measured matrix |
| windows | cow | 100,000 | B | selected packet assembly | 8,314.13 | 8,321.06 | measured matrix |
| windows | manifest | 1,000 | A | initialization | 7,631.19 | 7,638.97 | measured matrix |
| windows | manifest | 1,000 | A | commit without packet | 76.63 | 83.08 | measured matrix |
| windows | manifest | 1,000 | A | commit with packet | 99.55 | 105.88 | measured matrix |
| windows | manifest | 1,000 | A | selected packet assembly | 30.15 | 35.45 | measured matrix |
| windows | manifest | 1,000 | B | initialization | 4,994.87 | 5,002.86 | measured matrix |
| windows | manifest | 1,000 | B | commit without packet | 1,112.42 | 1,118.72 | measured matrix |
| windows | manifest | 1,000 | B | commit with packet | 1,144.94 | 1,152.21 | measured matrix |
| windows | manifest | 1,000 | B | selected packet assembly | 50.08 | 55.64 | measured matrix |
| windows | manifest | 10,000 | A | initialization | 103,871.94 | 103,880.83 | measured matrix |
| windows | manifest | 10,000 | A | commit without packet | 485.64 | 492.09 | measured matrix |
| windows | manifest | 10,000 | A | commit with packet | 5,655.34 | 5,661.24 | measured matrix |
| windows | manifest | 10,000 | A | selected packet assembly | 5,082.58 | 5,088.61 | measured matrix |
| windows | manifest | 10,000 | B | initialization | 62,411.44 | 62,420.69 | measured matrix |
| windows | manifest | 10,000 | B | commit without packet | 12,004.64 | 12,010.98 | measured matrix |
| windows | manifest | 10,000 | B | commit with packet | 14,940.08 | 14,966.71 | measured matrix |
| windows | manifest | 10,000 | B | selected packet assembly | 5,263.50 | 5,269.47 | measured matrix |
| windows | manifest | 100,000 | A | initialization | 1,430,435.00 | 1,430,453.18 | measured matrix |
| windows | manifest | 100,000 | A | commit with packet | 21,049.07 | 21,064.63 | measured matrix |
| windows | manifest | 100,000 | A | selected packet assembly | 5,290.49 | 5,297.14 | measured matrix |
| windows | manifest | 100,000 | B | initialization | 890,807.09 | 890,824.75 | measured matrix |
| windows | manifest | 100,000 | B | commit with packet | 525,021.91 | 525,029.32 | measured matrix |
| windows | manifest | 100,000 | B | selected packet assembly | 6,119.80 | 6,126.21 | measured matrix |
| wsl | cow | 1,000 | A | initialization | 9,082.66 | 9,084.18 | measured matrix |
| wsl | cow | 1,000 | A | commit without packet | 102.76 | 103.69 | measured matrix |
| wsl | cow | 1,000 | A | commit with packet | 131.21 | 132.02 | measured matrix |
| wsl | cow | 1,000 | A | selected packet assembly | 256.96 | 257.72 | measured matrix |
| wsl | cow | 1,000 | B | initialization | 6,004.57 | 6,005.64 | measured matrix |
| wsl | cow | 1,000 | B | commit without packet | 1,764.41 | 1,765.41 | measured matrix |
| wsl | cow | 1,000 | B | commit with packet | 1,558.22 | 1,559.19 | measured matrix |
| wsl | cow | 1,000 | B | selected packet assembly | 53.87 | 54.62 | measured matrix |
| wsl | cow | 10,000 | A | initialization | 97,199.00 | 97,202.50 | measured matrix |
| wsl | cow | 10,000 | A | commit without packet | 488.44 | 489.94 | measured matrix |
| wsl | cow | 10,000 | A | commit with packet | 10,214.99 | 10,216.49 | measured matrix |
| wsl | cow | 10,000 | A | selected packet assembly | 31,540.10 | 31,541.43 | measured matrix |
| wsl | cow | 10,000 | B | initialization | 69,108.68 | 69,112.24 | measured matrix |
| wsl | cow | 10,000 | B | commit without packet | 65,969.86 | 65,971.70 | measured matrix |
| wsl | cow | 10,000 | B | commit with packet | 66,922.27 | 66,924.20 | measured matrix |
| wsl | cow | 10,000 | B | selected packet assembly | 33,217.69 | 33,219.31 | measured matrix |
| wsl | cow | 100,000 | A | commit with packet | 40,213.51 | 40,220.05 | post-cap A continuation |
| wsl | cow | 100,000 | A | selected packet assembly | 8,054.95 | 8,057.83 | post-cap retained probe |
| wsl | cow | 100,000 | B | initialization | 2,306,620.83 | 2,306,642.63 | measured matrix |
| wsl | cow | 100,000 | B | commit with packet | 656,774.27 | 656,783.15 | measured matrix |
| wsl | cow | 100,000 | B | selected packet assembly | 33,227.97 | 33,233.51 | measured matrix |
| wsl | manifest | 1,000 | A | initialization | 7,992.41 | 7,993.95 | measured matrix |
| wsl | manifest | 1,000 | A | commit without packet | 94.08 | 95.29 | measured matrix |
| wsl | manifest | 1,000 | A | commit with packet | 102.90 | 104.11 | measured matrix |
| wsl | manifest | 1,000 | A | selected packet assembly | 44.11 | 44.81 | measured matrix |
| wsl | manifest | 1,000 | B | initialization | 6,080.00 | 6,081.48 | measured matrix |
| wsl | manifest | 1,000 | B | commit without packet | 1,710.97 | 1,711.90 | measured matrix |
| wsl | manifest | 1,000 | B | commit with packet | 1,589.82 | 1,590.77 | measured matrix |
| wsl | manifest | 1,000 | B | selected packet assembly | 47.09 | 47.87 | measured matrix |
| wsl | manifest | 10,000 | A | initialization | 188,728.82 | 188,731.63 | measured matrix |
| wsl | manifest | 10,000 | A | commit without packet | 383.85 | 386.31 | measured matrix |
| wsl | manifest | 10,000 | A | commit with packet | 32,093.01 | 32,094.49 | measured matrix |
| wsl | manifest | 10,000 | A | selected packet assembly | 31,293.58 | 31,295.86 | measured matrix |
| wsl | manifest | 10,000 | B | initialization | 70,048.11 | 70,051.68 | measured matrix |
| wsl | manifest | 10,000 | B | commit without packet | 62,992.42 | 62,994.53 | measured matrix |
| wsl | manifest | 10,000 | B | commit with packet | 64,692.43 | 64,694.36 | measured matrix |
| wsl | manifest | 10,000 | B | selected packet assembly | 33,255.35 | 33,258.02 | measured matrix |
| wsl | manifest | 100,000 | A | commit with packet | 43,616.19 | 43,635.15 | post-cap A continuation |
| wsl | manifest | 100,000 | A | selected packet assembly | 41,041.51 | 41,060.56 | post-cap retained probe |
| wsl | manifest | 100,000 | B | initialization | 2,247,491.05 | 2,247,519.66 | measured matrix |
| wsl | manifest | 100,000 | B | commit with packet | 657,077.35 | 657,090.69 | measured matrix |
| wsl | manifest | 100,000 | B | selected packet assembly | 32,647.54 | 32,677.76 | measured matrix |
