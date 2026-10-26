# macOS accepted checkpoint workspace correctness result

PASS on 2026-10-26: native release workspace build exit0; complete unfiltered workspace suite **40 passed,0 failed,0 ignored,0 filtered**, plus zero-test workspace/doc targets. Original31 and all9 accepted checkpoint regression tests actually ran. No pilot ran.

Source HEAD `38e246500ff60e0d45821ec4de1ed11d886b3091`, exact accepted26 manifest SHA256 `43a298bb575b724187eea89de6d55176d67992c2ed3f025e78f8e46d90c981cb`. All26 source lengths/hashes matched both checkout and HEAD Git blobs before/after execution; worktree clean. Protected primary HEAD/tree/index/status also unchanged. No source edits, threshold changes, speculative repair, commits, push or dirty-worktree mutation.

Fresh target under this result directory; locked/offline Cargo1.89.0/rustc1.89.0 native arm64 using the existing exact45-package Cargo cache. Commands and selected environment are in build-command.json,test-command.json,environment.json; full toolchain and archive bindings retained. Build took 26.217221s; test invocation including fresh test compilation took 22.768120s.

Real boundary fixture totaled **80,976,318 bytes**, with actual file lengths/hashes and cache-boundary receipt retained in boundary-evidence.json. No smaller substituted fixture or extra-fixture simulation. Actual40 test names/summary rows are in actual-tests.json.

Native CLI SHA256 `c4c40071e094309595bb31dea74278272e0de39b4984c2b78d6f77671f8155f4`; complete40-test library binary SHA256 `919778ed385d168745fffa52cefa5a572bea6e19c7b19b70c21563c0d9971414`. binary-bindings.json binds all fresh Mach-O build/test artifacts to source commit/manifest/Cargo.lock. Native stdout/stderr, original child identity, exit, wait/EOF and two empty owned-group observations are retained per build/test command. All child handles closed. Native-launch-used.py is the exact task-owned process adapter used. Direct-child sampling is not full descendant,hard-peak,40ms or monitoring qualification. No memory/performance qualification is inferred from these tests.

Free disk after execution 132739387392 bytes. Existing targets,caches,fixtures and historical artifacts were not cleaned. This correctness result does not resolve the known two-call cleanup/drainage/timing issue; await reviewed Linux product repair for changed proof. HOLD.
