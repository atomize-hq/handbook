# Observed fresh-process lookup latency

Three independent process observations per binding/key. Caches uncontrolled; these include process startup/delivery and differ from the engine timer.

| Platform | Backend | Documents | Mode | Binding | Positive median ms | Negative median ms | Measurement phase |
| --- | --- | ---: | --- | --- | ---: | ---: | --- |
| windows | cow | 1,000 | A | current | 7.335 | 6.764 | measured matrix |
| windows | cow | 1,000 | A | initial retained | 7.056 | 7.228 | measured matrix |
| windows | cow | 1,000 | B | current | 7.458 | 7.400 | measured matrix |
| windows | cow | 1,000 | B | initial retained | 7.342 | 6.949 | measured matrix |
| windows | cow | 10,000 | A | current | 8.532 | 7.590 | measured matrix |
| windows | cow | 10,000 | A | initial retained | 9.986 | 9.325 | measured matrix |
| windows | cow | 10,000 | B | current | 10.048 | 9.540 | measured matrix |
| windows | cow | 10,000 | B | initial retained | 11.767 | 11.572 | measured matrix |
| windows | cow | 100,000 | A | current | 22.186 | 21.587 | measured matrix |
| windows | cow | 100,000 | A | initial retained | 35.338 | 36.210 | measured matrix |
| windows | cow | 100,000 | B | current | 22.439 | 21.820 | measured matrix |
| windows | cow | 100,000 | B | initial retained | 39.982 | 39.126 | measured matrix |
| windows | manifest | 1,000 | A | current | 8.017 | 7.810 | measured matrix |
| windows | manifest | 1,000 | A | initial retained | 8.320 | 7.937 | measured matrix |
| windows | manifest | 1,000 | B | current | 8.698 | 8.515 | measured matrix |
| windows | manifest | 1,000 | B | initial retained | 8.750 | 8.669 | measured matrix |
| windows | manifest | 10,000 | A | current | 25.701 | 25.723 | measured matrix |
| windows | manifest | 10,000 | A | initial retained | 28.759 | 26.828 | measured matrix |
| windows | manifest | 10,000 | B | current | 27.680 | 27.923 | measured matrix |
| windows | manifest | 10,000 | B | initial retained | 28.937 | 28.786 | measured matrix |
| windows | manifest | 100,000 | A | current | 187.452 | 187.086 | measured matrix |
| windows | manifest | 100,000 | A | initial retained | 200.398 | 199.523 | measured matrix |
| windows | manifest | 100,000 | B | current | 187.382 | 185.833 | measured matrix |
| windows | manifest | 100,000 | B | initial retained | 212.694 | 204.872 | measured matrix |
| wsl | cow | 1,000 | A | current | 1.909 | 2.074 | measured matrix |
| wsl | cow | 1,000 | A | initial retained | 1.921 | 1.503 | measured matrix |
| wsl | cow | 1,000 | B | current | 2.029 | 1.431 | measured matrix |
| wsl | cow | 1,000 | B | initial retained | 1.784 | 1.453 | measured matrix |
| wsl | cow | 10,000 | A | current | 2.807 | 2.581 | measured matrix |
| wsl | cow | 10,000 | A | initial retained | 3.970 | 3.362 | measured matrix |
| wsl | cow | 10,000 | B | current | 2.938 | 2.635 | measured matrix |
| wsl | cow | 10,000 | B | initial retained | 4.236 | 3.740 | measured matrix |
| wsl | cow | 100,000 | A | current | 14.667 | 13.982 | post-cap retained probe |
| wsl | cow | 100,000 | A | initial retained | 25.710 | 25.711 | post-cap retained probe |
| wsl | cow | 100,000 | B | current | 15.846 | 15.469 | measured matrix |
| wsl | cow | 100,000 | B | initial retained | 29.865 | 29.226 | measured matrix |
| wsl | manifest | 1,000 | A | current | 2.454 | 2.281 | measured matrix |
| wsl | manifest | 1,000 | A | initial retained | 2.385 | 2.355 | measured matrix |
| wsl | manifest | 1,000 | B | current | 3.039 | 2.930 | measured matrix |
| wsl | manifest | 1,000 | B | initial retained | 2.480 | 2.601 | measured matrix |
| wsl | manifest | 10,000 | A | current | 21.215 | 19.040 | measured matrix |
| wsl | manifest | 10,000 | A | initial retained | 19.464 | 19.253 | measured matrix |
| wsl | manifest | 10,000 | B | current | 23.746 | 25.162 | measured matrix |
| wsl | manifest | 10,000 | B | initial retained | 20.965 | 19.583 | measured matrix |
| wsl | manifest | 100,000 | A | current | 175.623 | 194.167 | post-cap retained probe |
| wsl | manifest | 100,000 | A | initial retained | 178.557 | 176.098 | post-cap retained probe |
| wsl | manifest | 100,000 | B | current | 199.907 | 186.276 | measured matrix |
| wsl | manifest | 100,000 | B | initial retained | 194.796 | 189.895 | measured matrix |
