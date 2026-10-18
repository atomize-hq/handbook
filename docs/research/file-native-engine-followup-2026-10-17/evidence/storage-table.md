# Retained storage by store

Bytes describe actual retained files, with no deletion of growth histories. File allocation excludes directory/MFT metadata. Logical metadata combines candidate state/request records with actual COW objects; application publication/outcome control records are separate. Readable bytes combine current view, retained generation and packet files. Growth roots have no retired snapshots. Stale sealed candidates remain protected until explicit abandonment even when not included in the selected/eligible immutable-liveness buckets.

| Platform | Backend | Documents | Mode | Consumer | Selected publications | Logical bytes | Allocated file bytes | Files | Directories | Logical state/COW metadata | Logical readable bytes |
| --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| windows | manifest | 1,000 | A | project | 20 | 37591345 | 38842648 | 2378 | 346 | 9438607 | 13623180 |
| windows | manifest | 1,000 | A | research | 20 | 41978769 | 44606296 | 2707 | 368 | 9470566 | 14645253 |
| windows | manifest | 1,000 | B | project | 20 | 282538958 | 288024160 | 21338 | 405 | 9438397 | 258571513 |
| windows | manifest | 1,000 | B | research | 20 | 286073827 | 292555624 | 21604 | 399 | 9470361 | 258741017 |
| windows | manifest | 10,000 | A | project | 10 | 635192161 | 637215528 | 45248 | 319 | 53012369 | 452991413 |
| windows | manifest | 10,000 | B | project | 10 | 1541746564 | 1544597952 | 115240 | 338 | 53012274 | 1359546296 |
| windows | manifest | 100,000 | A | project | 3 | 2960799131 | 2961921448 | 220107 | 294 | 145638995 | 1539696549 |
| windows | manifest | 100,000 | B | project | 3 | 5361124225 | 5361972344 | 405100 | 289 | 145638965 | 3940022002 |
| windows | cow | 1,000 | A | project | 20 | 31029060 | 32345816 | 2433 | 396 | 2876323 | 13623180 |
| windows | cow | 1,000 | A | research | 20 | 35410169 | 38109464 | 2762 | 417 | 2901966 | 14645253 |
| windows | cow | 1,000 | B | project | 20 | 275976675 | 281527328 | 21393 | 455 | 2876114 | 258571513 |
| windows | cow | 1,000 | B | research | 20 | 279505255 | 286058792 | 21659 | 448 | 2901771 | 258741017 |
| windows | cow | 10,000 | A | project | 10 | 597866735 | 600107528 | 45445 | 455 | 15686934 | 452991413 |
| windows | cow | 10,000 | B | project | 10 | 1504421110 | 1507489952 | 115437 | 474 | 15686829 | 1359546296 |
| windows | cow | 100,000 | A | project | 3 | 2945508341 | 2947082936 | 221733 | 551 | 130348205 | 1539696549 |
| windows | cow | 100,000 | B | project | 3 | 5345833435 | 5347133832 | 406726 | 546 | 130348175 | 3940022002 |
| wsl | manifest | 1,000 | A | project | 20 | 37591062 | 39112704 | 2378 | 346 | 9438508 | 13623180 |
| wsl | manifest | 1,000 | A | research | 20 | 41978502 | 44773376 | 2707 | 368 | 9470472 | 14645253 |
| wsl | manifest | 1,000 | B | project | 20 | 282538706 | 288165888 | 21338 | 405 | 9438308 | 258571513 |
| wsl | manifest | 1,000 | B | research | 20 | 286073577 | 292651008 | 21604 | 399 | 9470272 | 258741017 |
| wsl | manifest | 10,000 | A | project | 10 | 635192032 | 637333504 | 45248 | 319 | 53012325 | 452991413 |
| wsl | manifest | 10,000 | B | project | 10 | 1541746424 | 1544663040 | 115240 | 338 | 53012225 | 1359546296 |
| wsl | manifest | 100,000 | A | project | 3 | 2959302958 | 2960445440 | 220107 | 297 | 145638986 | 1539696549 |
| wsl | manifest | 100,000 | B | project | 3 | 5361124225 | 5361991680 | 405100 | 289 | 145638965 | 3940022002 |
| wsl | cow | 1,000 | A | project | 20 | 31028778 | 32694272 | 2433 | 396 | 2876224 | 13623180 |
| wsl | cow | 1,000 | A | research | 20 | 35409902 | 38354944 | 2762 | 417 | 2901872 | 14645253 |
| wsl | cow | 1,000 | B | project | 20 | 275976422 | 281747456 | 21393 | 455 | 2876024 | 258571513 |
| wsl | cow | 1,000 | B | research | 20 | 279504977 | 286232576 | 21659 | 448 | 2901672 | 258741017 |
| wsl | cow | 10,000 | A | project | 10 | 597866592 | 600264704 | 45445 | 455 | 15686885 | 452991413 |
| wsl | cow | 10,000 | B | project | 10 | 1504421054 | 1507594240 | 115437 | 474 | 15686809 | 1359546296 |
| wsl | cow | 100,000 | A | project | 3 | 2944012168 | 2945630208 | 221733 | 554 | 130348196 | 1539696549 |
| wsl | cow | 100,000 | B | project | 3 | 5345833435 | 5347176448 | 406726 | 546 | 130348175 | 3940022002 |
