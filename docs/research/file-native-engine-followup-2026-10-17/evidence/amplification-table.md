# One-document update amplification

Metadata counters cover state descriptors/tree pages and indirect document metadata, excluding publication/outcome records, readable navigation and packet manifests. Application totals include their instrumented I/O. Work remains fully validated. Peak native memory is Windows peak working set or Linux maximum RSS, reported by the engine before final stdout serialization; these platform measures are not identical.

| Platform | Backend | Documents | Mode | Packet | Metadata read | Metadata written | App read | App written | Peak native memory bytes | Measurement phase |
| --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| windows | cow | 1,000 | A | False | 5128806 | 81631 | 5628040 | 189015 | 9011200 | measured matrix |
| windows | cow | 1,000 | A | True | 5128847 | 81635 | 5889690 | 399176 | 8876032 | measured matrix |
| windows | cow | 1,000 | B | False | 2606002 | 81631 | 41258784 | 12996827 | 8392704 | measured matrix |
| windows | cow | 1,000 | B | True | 2606028 | 81635 | 41379567 | 13047118 | 8843264 | measured matrix |
| windows | cow | 10,000 | A | False | 52013647 | 378019 | 57326103 | 1687636 | 40411136 | measured matrix |
| windows | cow | 10,000 | A | True | 52013721 | 378029 | 183944202 | 66676919 | 39538688 | measured matrix |
| windows | cow | 10,000 | B | False | 26048308 | 378019 | 413133093 | 129824145 | 38662144 | measured matrix |
| windows | cow | 10,000 | B | True | 26048354 | 378029 | 412029872 | 129874439 | 38899712 | measured matrix |
| windows | cow | 100,000 | A | True | 512383092 | 380390 | 680110902 | 75505673 | 311271424 | measured matrix |
| windows | cow | 100,000 | B | True | 256234058 | 380390 | 4121483254 | 1292080652 | 313483264 | measured matrix |
| windows | manifest | 1,000 | A | False | 1876104 | 469026 | 2340071 | 576410 | 8589312 | measured matrix |
| windows | manifest | 1,000 | A | True | 1876104 | 469026 | 2622542 | 786567 | 8712192 | measured matrix |
| windows | manifest | 1,000 | B | False | 938052 | 469026 | 39555577 | 13384222 | 8892416 | measured matrix |
| windows | manifest | 1,000 | B | True | 938052 | 469026 | 39697196 | 13434509 | 8798208 | measured matrix |
| windows | manifest | 10,000 | A | False | 21131704 | 5282926 | 26102886 | 6592543 | 39727104 | measured matrix |
| windows | manifest | 10,000 | A | True | 21131704 | 5282926 | 153047729 | 71581813 | 38903808 | measured matrix |
| windows | manifest | 10,000 | B | False | 10565852 | 5282926 | 397309373 | 134729052 | 43716608 | measured matrix |
| windows | manifest | 10,000 | B | True | 10565852 | 5282926 | 396532974 | 134779336 | 43806720 | measured matrix |
| windows | manifest | 100,000 | A | True | 192167704 | 48041926 | 356494233 | 123167209 | 310652928 | measured matrix |
| windows | manifest | 100,000 | B | True | 96083852 | 48041926 | 3957931777 | 1339742188 | 362242048 | measured matrix |
| wsl | cow | 1,000 | A | False | 5128806 | 81631 | 5627977 | 188994 | 6529024 | measured matrix |
| wsl | cow | 1,000 | A | True | 5128847 | 81635 | 5889523 | 399155 | 6455296 | measured matrix |
| wsl | cow | 1,000 | B | False | 2606002 | 81631 | 41258736 | 12996812 | 6209536 | measured matrix |
| wsl | cow | 1,000 | B | True | 2606028 | 81635 | 41379439 | 13047103 | 6307840 | measured matrix |
| wsl | cow | 10,000 | A | False | 52013647 | 378019 | 57326040 | 1687615 | 38260736 | measured matrix |
| wsl | cow | 10,000 | A | True | 52013721 | 378029 | 183944035 | 66676898 | 36315136 | measured matrix |
| wsl | cow | 10,000 | B | False | 26048308 | 378019 | 413133045 | 129824130 | 36327424 | measured matrix |
| wsl | cow | 10,000 | B | True | 26048354 | 378029 | 412029744 | 129874424 | 36216832 | measured matrix |
| wsl | cow | 100,000 | A | True | 512383092 | 380390 | 680110895 | 75505655 | 325582848 | post-cap A continuation |
| wsl | cow | 100,000 | B | True | 256234058 | 380390 | 4121483254 | 1292080652 | 316583936 | measured matrix |
| wsl | manifest | 1,000 | A | False | 1876104 | 469026 | 2340014 | 576389 | 6311936 | measured matrix |
| wsl | manifest | 1,000 | A | True | 1876104 | 469026 | 2622383 | 786546 | 6307840 | measured matrix |
| wsl | manifest | 1,000 | B | False | 938052 | 469026 | 39555535 | 13384207 | 6643712 | measured matrix |
| wsl | manifest | 1,000 | B | True | 938052 | 469026 | 39697076 | 13434494 | 6696960 | measured matrix |
| wsl | manifest | 10,000 | A | False | 21131704 | 5282926 | 26102829 | 6592522 | 38195200 | measured matrix |
| wsl | manifest | 10,000 | A | True | 21131704 | 5282926 | 153047620 | 71581795 | 36687872 | measured matrix |
| wsl | manifest | 10,000 | B | False | 10565852 | 5282926 | 397309331 | 134729037 | 40329216 | measured matrix |
| wsl | manifest | 10,000 | B | True | 10565852 | 5282926 | 396532854 | 134779321 | 40513536 | measured matrix |
| wsl | manifest | 100,000 | A | True | 192167704 | 48041926 | 356494226 | 123167191 | 320532480 | post-cap A continuation |
| wsl | manifest | 100,000 | B | True | 96083852 | 48041926 | 3957931777 | 1339742188 | 364466176 | measured matrix |
