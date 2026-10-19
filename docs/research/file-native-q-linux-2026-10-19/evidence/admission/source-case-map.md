# Q Linux admission and case map

Original start 2026-10-19 20:38:49 UTC; unchanged deadline 22:38:49 UTC.
Status: resource admission HOLD. Every executable qualification case below is UNPERFORMED in this task. No engine worktree, native root, build, fixture, commit or push has been created.

Final checkout f0904a63cb2500b9bea3a4034d19cbe38bfac729; core 07a8574e581e66ba58d7caac87238aa5f93918be. All 13 Rust and two Cargo files in the recovery worktree match final-code-build-binding.json. See local-source-binding.json for exact per-file and six controlling harness hashes. This local check does not qualify a native executable.

E = docs/research/file-native-incremental-engine-experiment-2026-10-18; F = docs/research/file-native-recovery-reclamation-2026-10-19, both at that checkout.

| Required case | Controlling existing entry point / additional work after GO |
| --- | --- |
| Build / meaningful bounds tests | E: cargo test --locked --offline --workspace, -j2 and fresh bounded target; build exact executable and bind hash. |
| Native filesystem / namespace | Native ext4 mount confirmed read-only. Case/Unicode namespace file probes remain unperformed. |
| File-lock lifecycle | E/deep_cases.py contains holder barrier, waiting cancellation, killed holder and successor audit. Run focused mechanism cases under 120s child bounds. |
| Immutable install / replacement / directory sync | E/src/io.rs is pinned; native source/API association and focused same/different/corrupt winner, interrupted install and directory-sync probes still required. Prior old-core native evidence is insufficient for exact-final qualification. |
| Complete strengthened recovery / backup | F/capsule.py --exe --scratch: A/B N16/N128; namespace transitions, recreated divergence, correcting successors, genuine pending A backup, fences, pins, packet bytes, eligible candidate, valid restored successor, half preparation, missing selector, selected-once B and transfer/checkpoint/retire/repeated GC. |
| External empty directory guard | Explicit supplemental probe required. It is not a named assertion in capsule.py; must establish external empty directory preservation on final core. |
| Missing / corrupt evidence and packet lifetime | F/focused_extra.py --exe --base: A/B missing/corrupt collection/packet backing files, missing tree node, case-ancestor refusal, damaged transport, packet owner across retirement/checkpoint/GC. |
| Legacy / new target replay | F/legacy_compatibility.py --exe --fixtures --base; requires authenticated existing small legacy A/B fixtures and eligible preparations; preserves old publication bytes and confirms new target profile2. No silent regeneration on new core. |
| Intent and replay dependencies | E/paired_intents.py and E/replay_lifetime.py; separately bounded invocations. |
| Independent operational checker | F/verify_operational.py --root --output, <=900s and2GiB, small restored stores only; no H101006 mutation or long new history. |
| Ordinary Bash discovery / copy / extraction | F/oracle.py has independent profile2 parser and extraction oracle. E/walkthrough.sh expects frozen NAVIGATION.md and is unsuitable unchanged for profile2; separate explicitly labeled external Bash harness is required. Bind B generation once and preserve exact authored bytes; no engine call during extraction. |
| Oracle supplement | F/oracle_supplement.py is tied to an earlier finished.json and its inherited deadline; do not launch unchanged as a new lane. Its controlling capsule/oracle calls above can be scheduled under this task's original clock with separate wrapper binding. |

No baseline portability patch is proposed. Any wrapper must separately record source, executable, exact arguments, original failures, finite child/aggregate RSS, logical bytes/files, free reserve and deadline. Existing capsule has180s internal subprocess timeout; external driver must enforce the authorized120s ordinary child ceiling without silently claiming the harness unchanged.

Only one native ext4 mount exists; no alternative native ext4 volume was found. Latest retained native census free6389047296B versus required21474836480B: shortfall15085789184B before any new allocation. Earlier mount receipt free4390662144B demonstrates fluctuation; neither passes admission. Existing native roots and archives total4178628683 logical bytes /207153 regular files. All preserved.

WSL accounting only:25395652281 logical bytes /1837621 files; legacy file-native-followup-20261017 separately contributes23402002010B /1768175 files. Free516225916928B. WSL is not a substitute qualification host.

Proposed initial native source/build/fixture ceiling512MiB, <=100000 new files, within global remaining capacity only. Full Windows checkout adds258361988 tracked bytes /4525 files plus metadata and must be admitted separately. Point reader256MiB; maintenance/checker2GiB; aggregate new Linux RSS3GiB; -j2. No permission to relax global32GiB/5000000 or reserved1GiB/100000.

GitNexus MCP unavailable. CLI fallback query failed argument parsing after unexpectedly initializing pnpm dlx cache; its residue is separately measured in gitnexus-cache-side-effect.json. No further launcher/bootstrap or deletion. Graph coverage UNKNOWN, no clean impact claim; no engine symbols edited.

Raw admission: native-root-census.raw.json, native-mount-admission.raw.json, wsl-root-census.raw.json. No physical power-loss, reboot or production durability claim.
