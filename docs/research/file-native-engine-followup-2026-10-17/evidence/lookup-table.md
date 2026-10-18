# Fresh-process lookup observations

Three process observations per key/binding; medians below. Positive key d000001 is ordinary metadata. Negative key not-present lies outside the existing d* key range; this is not an in-range-miss distribution. Each operation verifies positive selected lineage. Point path verification is not a full audit. End-to-end engine time includes lineage; point-inclusive stage excludes it. Phase labels distinguish post-cap probes; original initialization censorship is retained in the growth status table.

| Platform | Backend | Documents | Mode | Binding | Positive median ms | Negative median ms | Positive app bytes read | Positive metadata bytes read | Measurement phase |
| --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | --- |
| windows | cow | 1,000 | A | current | 2.291 | 2.012 | 178,481 | 83,462 | measured matrix |
| windows | cow | 1,000 | A | initial retained | 2.367 | 2.252 | 211,900 | 83,370 | measured matrix |
| windows | cow | 1,000 | B | current | 2.360 | 2.190 | 178,250 | 83,462 | measured matrix |
| windows | cow | 1,000 | B | initial retained | 2.471 | 2.252 | 211,675 | 83,370 | measured matrix |
| windows | cow | 10,000 | A | current | 3.495 | 2.770 | 468,431 | 83,244 | measured matrix |
| windows | cow | 10,000 | A | initial retained | 4.800 | 4.258 | 807,849 | 83,142 | measured matrix |
| windows | cow | 10,000 | B | current | 3.725 | 3.174 | 468,313 | 83,244 | measured matrix |
| windows | cow | 10,000 | B | initial retained | 5.299 | 4.847 | 807,731 | 83,142 | measured matrix |
| windows | cow | 100,000 | A | current | 16.197 | 15.621 | 3,516,345 | 85,139 | measured matrix |
| windows | cow | 100,000 | A | initial retained | 29.667 | 29.765 | 6,903,340 | 85,198 | measured matrix |
| windows | cow | 100,000 | B | current | 16.967 | 16.319 | 3,516,305 | 85,139 | measured matrix |
| windows | cow | 100,000 | B | initial retained | 34.086 | 32.949 | 6,903,300 | 85,198 | measured matrix |
| windows | manifest | 1,000 | A | current | 3.131 | 3.130 | 564,045 | 469,026 | measured matrix |
| windows | manifest | 1,000 | A | initial retained | 3.273 | 3.226 | 597,556 | 469,026 | measured matrix |
| windows | manifest | 1,000 | B | current | 3.177 | 3.172 | 563,819 | 469,026 | measured matrix |
| windows | manifest | 1,000 | B | initial retained | 3.354 | 3.361 | 597,330 | 469,026 | measured matrix |
| windows | manifest | 10,000 | A | current | 19.785 | 20.011 | 5,668,105 | 5,282,926 | measured matrix |
| windows | manifest | 10,000 | A | initial retained | 22.722 | 21.373 | 6,007,625 | 5,282,926 | measured matrix |
| windows | manifest | 10,000 | B | current | 21.170 | 21.449 | 5,668,003 | 5,282,926 | measured matrix |
| windows | manifest | 10,000 | B | initial retained | 22.736 | 22.744 | 6,007,523 | 5,282,926 | measured matrix |
| windows | manifest | 100,000 | A | current | 181.368 | 180.481 | 51,473,132 | 48,041,926 | measured matrix |
| windows | manifest | 100,000 | A | initial retained | 194.222 | 193.411 | 54,860,068 | 48,041,926 | measured matrix |
| windows | manifest | 100,000 | B | current | 180.728 | 179.682 | 51,473,092 | 48,041,926 | measured matrix |
| windows | manifest | 100,000 | B | initial retained | 206.206 | 198.738 | 54,860,028 | 48,041,926 | measured matrix |
| wsl | cow | 1,000 | A | current | 1.129 | 1.447 | 178,314 | 83,462 | measured matrix |
| wsl | cow | 1,000 | A | initial retained | 1.190 | 0.924 | 211,735 | 83,370 | measured matrix |
| wsl | cow | 1,000 | B | current | 1.388 | 0.840 | 178,104 | 83,462 | measured matrix |
| wsl | cow | 1,000 | B | initial retained | 1.159 | 0.878 | 211,525 | 83,370 | measured matrix |
| wsl | cow | 10,000 | A | current | 2.171 | 1.841 | 468,344 | 83,244 | measured matrix |
| wsl | cow | 10,000 | A | initial retained | 3.289 | 2.725 | 807,764 | 83,142 | measured matrix |
| wsl | cow | 10,000 | B | current | 2.208 | 1.972 | 468,281 | 83,244 | measured matrix |
| wsl | cow | 10,000 | B | initial retained | 3.433 | 2.972 | 807,693 | 83,142 | measured matrix |
| wsl | cow | 100,000 | A | current | 13.727 | 12.875 | 3,516,322 | 85,139 | post-cap retained probe |
| wsl | cow | 100,000 | A | initial retained | 24.646 | 24.724 | 6,903,325 | 85,198 | post-cap retained probe |
| wsl | cow | 100,000 | B | current | 14.665 | 14.230 | 3,516,305 | 85,139 | measured matrix |
| wsl | cow | 100,000 | B | initial retained | 28.176 | 27.690 | 6,903,300 | 85,198 | measured matrix |
| wsl | manifest | 1,000 | A | current | 1.724 | 1.619 | 563,878 | 469,026 | measured matrix |
| wsl | manifest | 1,000 | A | initial retained | 1.707 | 1.677 | 597,391 | 469,026 | measured matrix |
| wsl | manifest | 1,000 | B | current | 2.232 | 2.112 | 563,668 | 469,026 | measured matrix |
| wsl | manifest | 1,000 | B | initial retained | 1.801 | 1.869 | 597,181 | 469,026 | measured matrix |
| wsl | manifest | 10,000 | A | current | 19.423 | 16.435 | 5,668,026 | 5,282,926 | measured matrix |
| wsl | manifest | 10,000 | A | initial retained | 18.604 | 18.308 | 6,007,548 | 5,282,926 | measured matrix |
| wsl | manifest | 10,000 | B | current | 21.899 | 23.330 | 5,667,916 | 5,282,926 | measured matrix |
| wsl | manifest | 10,000 | B | initial retained | 19.860 | 18.659 | 6,007,438 | 5,282,926 | measured matrix |
| wsl | manifest | 100,000 | A | current | 156.822 | 179.039 | 51,473,109 | 48,041,926 | post-cap retained probe |
| wsl | manifest | 100,000 | A | initial retained | 176.449 | 174.085 | 54,860,053 | 48,041,926 | post-cap retained probe |
| wsl | manifest | 100,000 | B | current | 174.632 | 161.720 | 51,473,092 | 48,041,926 | measured matrix |
| wsl | manifest | 100,000 | B | initial retained | 192.579 | 186.712 | 54,860,028 | 48,041,926 | measured matrix |
