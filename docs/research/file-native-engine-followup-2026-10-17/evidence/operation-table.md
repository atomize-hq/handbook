# Reached observations

Times are individual engine observations in ms unless a three-process lookup median is explicitly stated. OS/device caches are uncontrolled. No p95 or product SLA. Original initialization censors remain dashes; separate post-cap work is labeled here and in the process/batch tables.

| Platform | Backend | Documents/store | Mode | Init | Commit 1, no packet | Commit 1 + packet | Packet from selected | Phase: commit+packet / selected packet |
| --- | --- | ---: | --- | ---: | ---: | ---: | ---: | --- |
| windows | cow | 1,000 | A | 6,859.40 | 107.66 | 128.39 | 37.61 | measured matrix / measured matrix |
| windows | cow | 1,000 | B | 5,056.73 | 1,156.78 | 1,210.06 | 57.89 | measured matrix / measured matrix |
| windows | cow | 10,000 | A | 99,169.99 | 758.49 | 5,842.63 | 5,220.07 | measured matrix / measured matrix |
| windows | cow | 10,000 | B | 57,997.93 | 11,924.87 | 11,866.35 | 6,440.05 | measured matrix / measured matrix |
| windows | cow | 100,000 | A | 1,260,276.74 | not allocated | 17,404.44 | 6,081.76 | measured matrix / measured matrix |
| windows | cow | 100,000 | B | 1,015,635.26 | not allocated | 141,302.91 | 8,314.13 | measured matrix / measured matrix |
| windows | manifest | 1,000 | A | 7,631.19 | 76.63 | 99.55 | 30.15 | measured matrix / measured matrix |
| windows | manifest | 1,000 | B | 4,994.87 | 1,112.42 | 1,144.94 | 50.08 | measured matrix / measured matrix |
| windows | manifest | 10,000 | A | 103,871.94 | 485.64 | 5,655.34 | 5,082.58 | measured matrix / measured matrix |
| windows | manifest | 10,000 | B | 62,411.44 | 12,004.64 | 14,940.08 | 5,263.50 | measured matrix / measured matrix |
| windows | manifest | 100,000 | A | 1,430,435.00 | not allocated | 21,049.07 | 5,290.49 | measured matrix / measured matrix |
| windows | manifest | 100,000 | B | 890,807.09 | not allocated | 525,021.91 | 6,119.80 | measured matrix / measured matrix |
| wsl | cow | 1,000 | A | 9,082.66 | 102.76 | 131.21 | 256.96 | measured matrix / measured matrix |
| wsl | cow | 1,000 | B | 6,004.57 | 1,764.41 | 1,558.22 | 53.87 | measured matrix / measured matrix |
| wsl | cow | 10,000 | A | 97,199.00 | 488.44 | 10,214.99 | 31,540.10 | measured matrix / measured matrix |
| wsl | cow | 10,000 | B | 69,108.68 | 65,969.86 | 66,922.27 | 33,217.69 | measured matrix / measured matrix |
| wsl | cow | 100,000 | A | — | not allocated | 40,213.51 | 8,054.95 | post-cap A continuation / post-cap retained probe |
| wsl | cow | 100,000 | B | 2,306,620.83 | not allocated | 656,774.27 | 33,227.97 | measured matrix / measured matrix |
| wsl | manifest | 1,000 | A | 7,992.41 | 94.08 | 102.90 | 44.11 | measured matrix / measured matrix |
| wsl | manifest | 1,000 | B | 6,080.00 | 1,710.97 | 1,589.82 | 47.09 | measured matrix / measured matrix |
| wsl | manifest | 10,000 | A | 188,728.82 | 383.85 | 32,093.01 | 31,293.58 | measured matrix / measured matrix |
| wsl | manifest | 10,000 | B | 70,048.11 | 62,992.42 | 64,692.43 | 33,255.35 | measured matrix / measured matrix |
| wsl | manifest | 100,000 | A | — | not allocated | 43,616.19 | 41,041.51 | post-cap A continuation / post-cap retained probe |
| wsl | manifest | 100,000 | B | 2,247,491.05 | not allocated | 657,077.35 | 32,647.54 | measured matrix / measured matrix |
