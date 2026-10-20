# Native B ordinary discovery counterexample

Fresh N16 B store initialized, lifecycle setup completed, then raw packing build with include_attempts=true and cleanup=true returned status complete, before any subsequent publication.

Ordinary Bash read SELECTED.json exactly once, parsed its publication ID with standard JSON, and required publications/<publication-id>.json to discover the B generation. That ordinary selected publication file is absent after packing cleanup. Bash exited21 at the missing control; listing/search/exact copy were not reached. No engine, custom pack decoder, directory scan, or alternate discovery was used during discovery/extraction. No helper or source repair was attempted.

Counterexample store: /home/spenser/file-native-p-linux-20261020/qualification/fixtures/bash-B16-immediate-pack/disposable/store

Exact missing publication path and selector ID are in native-closeout.json. The saved SELECTED bytes and expected-publication-path receipt are in sibling bash-B16-shell. Closeout used that saved selector receipt and did not reread SELECTED.

Frozen engine SHA256063342e6bbd6bfb49729ff856462b9fdaa4a8d7cdf51d941d2a60f356d2c0cbe; core4cb6b9fba9ffabc294a6006ec54fe75366968db4. All139source hashes and frozen executable still match. Engine operations were only init,lifecycle,packing. Packing cleanup status complete is bound by native-engine-events.json. Native proper whole-group/RSS guards remained enabled; raw process receipts remain under native qualification/process.

Fresh prospective admission retained old failed/censored prefixes and used the parent tiny-only component reallocation within280MiB: source8MiB,cache8MiB,retained synthetic84MiB,all fixtures160MiB,returns16MiB. Forecast20MiB maximum combined additional writes and2,000files fit. Per-operation init/lifecycle/pack/shell forecasts are in native-admission.json and native-pack-forecast.json. Actual tiny fixture growth337,795B; all fixtures128,692,325B, under160MiB. Total actual controller/native/cache/Git scope is recorded in closeout.json.

No recovery continuation, Cargo work, source edits, clock reset, failing-prefix deletion or further functional work. Native ordinary-control route failed; overall native qualification does not pass. Fixed functional cutoff04:39:12Z and closeout04:44:12Z remain unchanged. Parent owns core investigation and any further GO.
