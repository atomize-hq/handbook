Below are both reports in full. Their delivery notes describe the original audit; your acknowledgment confirms the coordinator has now received the summary.

[REPORT.md](/tmp/file-native-host-inventory-linux-20261024-01a16fd3/REPORT.md)

# Native Linux file-native experiment inventory — 2026-10-24

Read-only inventory for coordinator chat `01a16f73-5070-7431-868e-b9e0e174b06e` (host `local`). Inspection chat: `01a16fd3-c1d9-72d0-abc3-73533ad78d2f`. No experiment module was imported, no build/test/engine was run, and no existing file was deliberately changed. New files are confined to this report directory. No agents or other owners were started. All execution results below are **historical evidence inspected today**, not new test results.

## Findings that change the next decision

- This is native **`spenser-linux` / Manjaro / ext4**, and the expected `/home/spenser/file-native-online-linux-20261020` workspace exists. Historical Linux reports identify the same Manjaro kernel/platform. The supplied `spenser-linux-codex-tail` name is a connection label that I cannot independently map to machine identity; it is not the kernel hostname. There is no evidence here that the experiment roots are missing because this is a different Linux machine.
- The newest online engine copy is associated with **`0628322e4b48e0efad19cdc32238a622d4aaa7a8`** by `COPY_BUDGET.json`. All **25 imported Rust/Cargo/consumer files** match freeze SHA-256 **`464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`**. It is a standalone source copy, not a Git checkout. Its 31 tests are explicitly **UNRUN**. Five CP4 protocol/document files described by the larger freeze were not included in this Linux import.
- The native online binary is **earlier CP1 `62ce963085ac054bcc34eb21c6733b94db160975`**, SHA-256 **`11c306c4fa30d74a6f83274666024591e0e150674419d1e4dbe06e9a251a34f5`**. Both the target binary and its preserved copy match that recorded hash today. It is not a build of the final 31-test source.
- Later Linux **v3 interface controls actually ran** successfully after an earlier preparation blocker. Their raw report says **“INTERFACE CONTROLS ONLY; NO ENGINE CASE QUALIFIED.”** This is additional completed runner work, not final product evidence.
- Earlier packing core **`d70f3864f2888d46d9112213b4a03f8d82745a8e`** has substantial real native Linux execution: five unit tests, 52 packing cases/267 events, 24 strict cases, eight focused cases/25 events, four immediate cases, and a later complete recovery follow-up of **24 cases/282 events**. These results must remain attached to that older core. Its historical overall qualification fields remain null; this inventory does not promote them to a new global pass.
- No final online 37-call pilot, final 31-test execution, H1k/H10k accepted-history cohort, 1,000 physical activations, or complete online matrix result was found in the searched Linux evidence. This is a bounded host observation, not a claim about Windows/Mac or unsearched storage.

## Host and coverage

Observed 2026-10-24, with a recorded host observation at `2026-10-24T21:36:20Z` (17:36 EDT). Kernel: `Linux spenser-linux 6.16.8-1-MANJARO ... x86_64 GNU/Linux`; OS: Manjaro rolling. `findmnt` resolves the experiment path to `/dev/nvme0n1p2`, ext4. The audit sandbox presents that mount as `ro,nosuid,nodev,noatime`; this does not establish that the host's normal mount is read-only. This is not a WSL substitution.

The inventory shell's `python3` is 3.10.11. `/usr/bin/python3` resolves to `/usr/bin/python3.13`; the historical native N report records Python 3.13.7. The shared-v2 receipt's Python 3.11.9 is a historical source-validation record, not a measurement of the interpreter used by this inventory. No interpreter or environment was repaired.

Searched:

| Root | Coverage / result |
| --- | --- |
| `/home/spenser` | All immediate entries; 40 experiment/capsule/evidence matches. All `file-native*` directory families inspected through source manifests, top-level reports and bounded structure. Corpus trees were not exhaustively traversed. |
| `/home/spenser/__Active_code` | Immediate entries and bounded discovery up to three directory levels, stopping at Git roots and skipping build/data/cache trees. `atomize-hq` contains other projects; `/home/spenser/__Active_code/atomize-hq/handbook` is absent. |
| `/home/spenser/__Active_code/.worktrees` | 55 immediate entries; no Handbook/file-native names. Bounded Git discovery found no target worktree. |
| `/home/spenser/.codex/worktrees` | 45 immediate entries and their child identities; no Handbook/file-native worktree. Unrelated evidence families were not audited. |
| `/home/spenser/.codex/task-worktrees`, `/home/spenser/.codex/validation-checkouts`, `/home/spenser/.codex/archives`, `/home/spenser/.codex/handoffs` | Immediate inventories; no additional target checkout. |
| `/home/spenser/.codex/visualizations/2026/10/{18,19,20,21,22,23,24}` | Bounded names/content locations; no existing file-native reports found. The current app output directory is separate. |
| `/home/spenser/Documents`, `/home/spenser/archived_work` | Bounded source-directory discovery; no additional target Git checkout found. |
| `/home/spenser/Downloads`, `/tmp`, `/var/tmp` | Immediate name inventory; no extra named experiment source found. `/tmp` contains many unrelated historical test roots, whose contents were not scanned. |
| `/home/spenser/.cargo` | Cache directory structure plus existing dependency-binding receipt; no fresh whole-cache census or hashing. |

Missing: `/home/spenser/__Active_Code` (capital C), the `atomize-hq/handbook` path above, and the historical packing `target-final` and unit fixture directories described below. No selected root returned an access-denied error. Other users, external disks, remote hosts, arbitrary unnamed nested copies, and unreachable Windows/Mac paths are coverage gaps. No network fetch or remote host inspection was attempted.

`host-observation.json`, `root-discovery.json`, `bounded-source-discovery.json`, and `visualization-discovery.json` preserve the bounded discovery details. A targeted process listing found no visible process whose arguments referenced `/home/spenser/file-native-`; an empty lock file is not treated as proof of liveness or quiescence outside this observation.

## Repositories and copied source

The product source identifies its upstream as **`atomize-hq/handbook`**. The similarly named `project-handbook` checkout below points to **`Spenquatch/project-handbook`** and is not the experiment repository.

| Absolute path | Branch/HEAD or recorded source | Git relationship / state / source significance |
| --- | --- | --- |
| `/home/spenser/__Active_code/project-handbook` | `main`, `e6df715f7448e263597351cfdb9510233fa57658`, Jan 4, 2026 | Only registered worktree; common dir `/home/spenser/__Active_code/project-handbook/.git`. Local `main` equals locally recorded `origin/main`; no ahead commits. Six tracked modifications and one untracked guide, preserved. Unrelated older application checkout. |
| `/home/spenser/file-native-online-linux-20261020/source/final-engine` | Copy grant records `0628322e4b48e0efad19cdc32238a622d4aaa7a8`; freeze records base `a3f9805eccc19ede8dd5c735b54e675e98e63714` and last source write `2026-10-21T04:09:56.816601+00:00` | No `.git`; 25/25 imported code files match freeze. Candidate product source. Branch, ancestry and publication status unavailable locally. |
| `/home/spenser/file-native-online-linux-20261020/source/cp1-62ce9630` | `62ce963085ac054bcc34eb21c6733b94db160975` | Standalone earlier source copy and `cp1-62ce9630-source-only.tar`; build binding, not final source. |
| `/home/spenser/file-native-online-linux-20261020/source/shared-v2` and sibling `native_*v3*` files | Shared-v2 freeze plus v3 suite `efd07cb5dee46a335be8c03a2f2c0462d7f923ca`, harness clean pin `2b651e05aa265c76e0c8d8254ff1d34035573f52` in preparation receipt | Local launcher/protocol copies, no Git. Shared map has 158 Linux engine rows PENDING. Native interface controls have separate results. |
| `/home/spenser/file-native-p-linux-20261020/source-final` | Core and source manifest `d70f3864f2888d46d9112213b4a03f8d82745a8e` | No Git; **145/145 source files match manifest today**. Includes packing, recovery and navigation harnesses. Final native ELF retained separately in `bin/`. |
| `/home/spenser/file-native-p-linux-20261020/source` | Earlier core `4cb6b9fba9ffabc294a6006ec54fe75366968db4`, harness `59bbd168bfa67bbe68f6f4d669658018786ddaba` | Old 139-file source family preserved; old native executable also retained. No Git status available. |
| `/home/spenser/file-native-n-linux-20261019/final-4bde/source` | `4bde156323812ef18c0dea6798cfc2108aaad933` | No Git; **163/163 source files match manifest today**. Final navigation source, distinct from earlier N candidate. |
| `/home/spenser/file-native-n-linux-20261019/source` | Earlier N candidate; preservation records call it `6e66` | Standalone source; original binary/hash retained. Exact full initial commit was not recovered in this bounded readback. |
| `/home/spenser/file-native-q-linux-20261019-final/source` | Checkout `f0904a63cb2500b9bea3a4034d19cbe38bfac729`; core `07a8574e581e66ba58d7caac87238aa5f93918be` | Standalone 95-file manifest family; debug native binary, initial failed capsule and successful v2 continuation preserved. |
| `/home/spenser/file-native-recovery-20261019/file-native-incremental-engine-experiment-2026-10-18` | Multiple historical builds; latest compatible binding `fe1ee22a82d65dc88a14e38b7e77c921538c4bc4` | Mutable standalone source family with preserved previous binaries. A directory name or old `finished.json` cannot identify its current executable. |
| `/home/spenser/file-native-incremental-source-5a65a21` | `5a65a2155f9eb297d2cbb1b7c900fa749629605c` | Standalone incremental/accounting source capsule, old Linux exports and imported native-macos evidence. |
| `/home/spenser/file-native-incremental-source-7e00d22` | `7e00d22b470c1eed476127d2f585789036e79cf3` | Standalone audit source capsule. |
| `/home/spenser/file-native-incremental-source-7ac70f9` | `7ac70f93cce14ecd3d935852801f034c122000d8` | Standalone replay source capsule. |
| `/home/spenser/file-native-incremental-source-f3e942f` | `f3e942f6a2e84ff33fd6ea860caed49ad650f86f` | Standalone initial incremental source capsule. |
| `/home/spenser/file-native-incremental-source-rootfix`, `/home/spenser/file-native-incremental-source-recipe` | Precommit/rootfix/recipe labels | Standalone intermediate source; exact Git commits UNKNOWN. Their distinct archives and Linux builds remain. |
| `/home/spenser/file-native-prerequisite-linux-20261018/source` | Baseline `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9` | Standalone bounded archive including its own AGENTS.md and prerequisite engine/harness. |

Dirty files in the old application checkout are `.gitignore`, `docs/README.md`, `services/app.py`, `services/cli/dev.py`, `services/cli/governance.py`, `tests/services/test_cli_governance.py`; untracked `docs/guides/aph-smoke-test-commands.md`. No source diff was taken for those unrelated changes. Its only other local remote refs are old `origin/codex/research-tech-stack-decisions-and-options` and `origin/codex/update-ai_agent_platform_foundation.md`.

`git cat-file` in that application repo cannot resolve d70f, a0b92, 0628322, 6b689db5, CP1 62ce, or a3f9805. Therefore **no local Git ancestry or unpublished-commit conclusions about the experiment are possible**. Standalone copies have no meaningful dirty/untracked status; source-manifest comparisons are the available correspondence proof. `handbook-git-observation.json`, `final-source-manifest-check.json`, and `earlier-source-manifest-checks.json` record these checks.

## Builds, fixtures, archives and evidence families

Current native ELFs were identified by their ELF headers and small-file hashes, without executing them. Full paths, sizes, hashes, test binaries, library files and dependency files are in `binary-inventory.json`.

| Current native executable | Current SHA-256 | Recorded source / limitation |
| --- | --- | --- |
| `/home/spenser/file-native-online-linux-20261020/target/release/file-native-incremental` | `11c306c4fa30d74a6f83274666024591e0e150674419d1e4dbe06e9a251a34f5` | CP1 62ce; preserved identical at `evidence/cp1-62ce-native-binary`. No final build. |
| `/home/spenser/file-native-p-linux-20261020/bin/candidate-linux-final` | `a8501737ca4fdd5cfc177620860c1ae103d56f2e9b2151104a922df511a3d84b` | d70f packing core; 3,349,520 bytes. |
| `/home/spenser/file-native-p-linux-20261020/bin/candidate-linux` | `063342e6bbd6bfb49729ff856462b9fdaa4a8d7cdf51d941d2a60f356d2c0cbe` | Earlier P source; do not substitute for final. |
| `/home/spenser/file-native-n-linux-20261019/final-4bde/build/release/file-native-incremental` | `da75d34e5fe162447e356aead2fba0f2a1ac21bc1055ebb3812bce80fda79846` | 4bde final navigation. |
| `/home/spenser/file-native-n-linux-20261019/build/release/file-native-incremental` | `5295ce01770f9c4ba8b88bd4848f745c23c327ca78d6cd1a80770c2e1b69e89e` | Earlier N; distinct preserved source/binary. |
| `/home/spenser/file-native-q-linux-20261019-final/build/debug/file-native-incremental` | `e70d47a82e1c51a989395f0025f85859e74aed07c07e482e0a6b37913e76e854` | Q binding f090/07a8574. |
| `/home/spenser/file-native-recovery-20261019/results/build/release/file-native-incremental` | `13e368f6f46e3781f32bf73616477de9fa3828ec762e4e08001f8b0e769aa6db` | Latest compatible fe1ee22 binding; older reports describe other hashes. `results/previous-core-binary` and `compatible-core-previous-core-binary` also exist. |

Other distinct release builds remain in `/home/spenser/file-native-incremental-linux-{final-20261018,rootfix,recipe,replay-final,audit-final,accounting-final}/build`, plus `/home/spenser/file-native-prerequisite-linux-20261018/build`. Debug test executables exist in several retained build trees. The online target contains CP1 release output and a 16,312-byte `native-peer-v3-targeted` ELF (an interface peer). P's historical `/target-final` is absent; the copied final executable survives in `/bin`.

Cargo uses shared `/home/spenser/.cargo`; `CARGO_HOME`, `CARGO_TARGET_DIR`, and `RUSTUP_HOME` were unset in the inventory shell. Registry cache/source roots exist. `online.../evidence/cp1-dependency-binding.json` historically binds 45 locked archives, 32 relevant unpacked crates, and 1,530 source files; it is not a fresh audit of today's whole cache. No private cache was created. P retains `.global-cache` backups in `final-source-stage`; these are historical preservation artifacts, not instructions to restore global state.

Fixture/runtime families:

- `/home/spenser/file-native-online-linux-20261020/fixtures` contains control-driver/capsule-adapter and disposable v3 compiler/control directories. Its final-fixture preflight is **config-only**. No real final engine store or retained online cohort was found here.
- `/home/spenser/file-native-p-linux-20261020` retains original source, binaries and raw process receipts. Original P unit/qualification fixture roots and later final/recovery case roots were archived and then explicitly reclaimed historically. Their current absence is supported by preservation/reclamation records, not evidence of never having run.
- The actual **80,976,318-byte** unit fixture was produced for the earlier P five-test source. `preservation-artifact/two-roots.tar.gz` includes `unit-cache/` and `fixtures/`; archive metadata lists 17,967 members and 209,668,643 logical file bytes. The old fixture's existence **does not prove the final online source's heavy boundary test passed**.
- `preservation-three-20261020T060930Z/three-roots.tar.gz` contains `final-qualification-fixtures/`, `recovery-complete-fixture/`, and `recovery-bootstrap-fixture/`: 28,594 members and 215,825,212 declared file bytes. This is the retained physical copy of those removed runtime stores. The historical manifest SHA is `d20d83d160af8dd38c9b0430d81a7e5d0296e7ff4198cf0f2ffa47645febfbdb`. `reclamation-three-20261020T061950Z/reclamation-receipt.json` explains removal. No extraction, deletion, or full archive rehash was performed by this audit.
- `/home/spenser/file-native-n-linux-20261019/matrix/n{128,1000}-{A,B}-p4` has four successful navigation cells with 13 cases apiece. Each records 14 publication files in its final census; those are small navigation histories, not H1k/H10k. `/matrix/n10000-A-p4` is an interrupted initialization prefix with no successful summary; B was not started. Historical-profile-2 stores live under `historical2-data/{A,B}`; their summary records sealed profile-2 replay and unchanged old publications.
- `/home/spenser/file-native-q-linux-20261019-final` has capsule/capsule-v2, compatibility, replay, deep/paired/supplement, ordinary shell and native-probe data. `/home/spenser/file-native-recovery-20261019` retains small N16/N128 operational backup, restored/half-preparation, legacy A/B, paired/checks/replay and multiple versioned recovery results.
- `/home/spenser/file-native-prerequisite-linux-20261018/data` and `/packet-repair-supplement` are older prerequisite stores. Historical closeout inventories report roughly 123k files and 1.3 GB; these counts were not remeasured. Earlier incremental directories retain checks/adversarial/walk/namespace/replay/paired/audit/accounting fixtures. Recipe/rootfix `HISTORY-SOURCE.json` identifies a 4 KiB source document; the filename alone establishes no long-history completion.

Protected archive families, all retained: home-level `file-native-incremental-capsule-*.tar`, `capsule-{5a65a21,7ac70f9,7e00d22,recipe-precommit}.tar`, native-linux/incremental exported evidence tarballs; prerequisite source/native-evidence/packet-repair archives; recovery source, legacy-fixtures and four evidence generations; Q source/runtime-raw; N source/native-raw and final-4bde archives; P source transport and the two preservation archives; online CP1 source-only tar. `archive-listing-inventory.json` lists 29 selected archives with member counts, declared logical bytes and prefixes. Imported `native-macos/` reports inside older source capsules are **foreign historical evidence**, not current native Linux or Mac observations.

## Evidence matrix

| Version / family | Actual retained execution | Prepared-only, missing, failed or limited |
| --- | --- | --- |
| Final online 0628322 / freeze 464dcf… | Exact 25-file source correspondence checked today | `UNIT_CATALOG.json`: 31 tests UNRUN; no matching final native ELF or engine receipts found. The 80,976,318-byte final-source boundary test, 37-call pilot and integrated matrix remain unestablished on Linux. |
| Online CP1 62ce | Historical release build exit 0, 23.077s; source before/after equal; identical binary verified today | Build report marks resource sampling provisional (234 intervals >40ms). No engine invocation in that receipt. It is neither a final build nor a failing product test. |
| Shared-v2 + Linux native v3 efd07cb5 | Later targeted interface control run exit 0, 16 result rows, raw event SHA matches today; process closure true, no censors | 158 engine cases still PENDING. Fake-peer, stdin, rename-retry, missing-implementation and crash-control results are runner evidence only. Earlier static blocker is preserved. |
| P final d70f | Native build, **5/5 units** in `final-source-stage/unit.stdout`; compressed raw capsule report has **52 cases/267 events**, focus **8/25**, strict **24**, immediate **4** | Initial recovery stopped after three cases on external report prewrite-cap refusal. Windows-only no-delete-sharing cases skipped on Linux. No global P-pass assigned here. |
| P recovery follow-up d70f | `recovery-only-20261020T054348Z/result-complete.json`: **24 cases, 282 resource rows**, 282 events; native ELF SHA a850…; oracle SHA `877ff874cef9b9ec3867a3b7e6482a5130b4fc995f6d08ff401a96f321a6132f` | Earlier bootstrap attempt failed with oracle `KeyError: collection.yaml`; failed prefix retained. Completed run records expected refusal/fault exits (252 exit0, 19 exit86, 11 exit2), not 282 ordinary success exits. Runtime fixture now in three-roots archive. |
| N final 4bde | Two unit tests; review summary **39 focused cases/153 calls**, recovery, **8 README cases**; current source/ELF match | Earlier N four small navigation cells belong to the earlier candidate. N10k initialization was explicitly scope-cancelled, no complete N10k result. N1k means document count, not H1k accepted history. |
| Q f090/07a8574 | Build/two units; successful capsule-v2, I/O probe, deep/paired and other continuation stages | Original capsule exit1 retained in `closeout.json`. The successful later `continuation.json` must be read alongside it. Native API probes are not the full engine race suite. |
| Recovery 40509/ec544/fe1ee22 | Original 40509 lanes passed; ec544 final-core recovery/supplements recorded; fe1ee22 build/two units and legacy compatibility | fe1ee22 `compatible-core-binding.json` explicitly leaves extra/capsule unperformed. Current ELF 13e… is fe1ee22; old `final-core-binding.json` hash 1634… is an earlier executable, not a contradiction to erase. |
| Incremental f3e942f | `finished.json`: build/test/native exit0 | Earlier pin; not online-maintenance qualification. |
| Replay 7ac70f9 | Build/test, replay-lifetime, namespace, faults/checks, supplement, adversarial exit0 | Earlier pin. |
| Audit 7e00d22 | Build/test/strict audit-evidence exit0 | Earlier pin. |
| Accounting 5a65a21 | Build/test/paired/supplement/replay-lifetime exit0 | Earlier pin. |
| Prerequisite 379af4 | Correctness/regression/restoration/contention/smoke lanes plus packet supplement recorded | Original navigation wrappers exit1 remain; later four walkthroughs passed after wrapper created the missing extraction parent. |
| a0b92 integration, 6b689db5 closeout, CP2/CP3 claims supplied in task | Context supplied by coordinator | No local Git objects or Linux result bundle tying those exact pins to completed final online execution was recovered. Do not infer their state from older tests. |

Material claims were checked against raw/compressed result records rather than only the latest prose closeout. `material-raw-readbacks.json` records counts and bindings. `record-summaries.json` covers 127 selected historical report records; `evidence-record-paths.json` identifies the main later evidence locations. These are inventory aids, not a new qualification system.

Important sequencing differences: P's `final-source-stage/native-closeout.json` says its new binary had not yet been executed **at that stage**; later raw capsule and recovery receipts show it did execute. Likewise v3 preparation says execution was blocked, whereas later controls show it subsequently ran. Both earlier records remain valid historical snapshots.

## Remaining product work and reusable pieces

1. Establish an executable/test result for the **final exact source**, preserving the link from 0628322 → freeze 464dcf… → native binary → all 31 test results. The Rust source copy is complete for its declared Cargo workspace, but CP4 protocol files and the upstream Git graph are not local. The current binary cannot stand in for this.
2. Run the small real product pilot after that source is executable: publication between capture and activation; an old reader retained beyond 15 seconds; current and old lookup/outcome behavior; actual physical cleanup; full independent oracle; ordinary agent overlap. None of today's discovered controls substitutes for this product behavior. Recover the retained 37-call recipe from the authoritative host rather than inventing a new launcher project.
3. Only after the pilot resolves product uncertainty, measure accepted-history rolling compaction (H1k/H10k and a justified retained cohort), fresh-process authenticated lookup, concurrency and cleanup/activation behavior, plus backup/restore and performance. Existing N1k cells or 14-publication histories do not establish those scaling properties. The 1,000-activation and broader platform matrix remain prospective requirements where still relevant to the product decision, not automatic obligations to repeat every historic gate.

Reusable: exact final Rust source; shared Cargo dependencies; earlier native ELF baseline; d70f packing/recovery capsule and independent navigation oracle; old compatibility fixtures; N navigation oracle/README walkthrough; raw fault/refusal evidence; current native process/handle/stdin control implementation where useful. Keep their version-specific assumptions visible.

Concrete unresolved defects versus administration:

- No freshly demonstrated final engine defect was found because final-source runtime evidence is absent. Its correctness, telemetry accounting and physical-cleanup performance remain **UNKNOWN**.
- The P recovery interruption was an external report component/headroom refusal; a later complete product recovery follow-up exists. Rebuilding that old report-budget process is not a prerequisite to a small new product trial.
- The original Q capsule and prerequisite extraction failures have later successful bounded follow-ups; preserve the original failures without promoting them into current final-candidate defects.
- V3's earlier `runpy __main__` pre-import blocker was followed by successful targeted controls, so it is not an open blocker for those controls. This does not qualify all engine integrations.
- **`Path.is_junction()` / Python 3.11:** no such call occurs in the inspected imported native/shared Python source. I did not locate the originally reported failing helper or a repair receipt for that exact defect. Therefore repair status is UNKNOWN; no current failure was reproduced. Interpreter identity must be explicit when reusing any helper.
- **SIGALRM copy-helper:** the inspected online native helpers contain no SIGALRM/alarm calls. Old `final_resources.py` and `export_wsl_evidence.py` copies do contain `signal.alarm` in Linux/WSL administration. The exact reported copy-helper failure/repair was not recovered. A direct native Cargo/test route does not require importing those old resource/export helpers. No environment repair is indicated by this inventory.

## Smallest next product experiment action — recommendation only

On this native Linux host, first **build and run the unfiltered 31-test Cargo workspace for the exact imported final Rust freeze**, including the real 80,976,318-byte boundary fixture. Use a new owned scratch target/fixture location, explicit interpreter only if needed, existing dependencies, and a small direct command/result record. This produces the first missing final-source product fact without restarting launcher qualification. If it passes, the next behavioral action is the retained 37-call real pilot above. Do not start the broad matrix until that small trial is informative.

No part of this recommendation was executed.

## Delivery and limits

Report directory: `/tmp/file-native-host-inventory-linux-20261024-01a16fd3` on `spenser-linux`. The coordinator can retrieve this report over the existing native Linux connection. No `send_message_to_thread` tool is exposed in this session's available tool catalog, so direct cross-chat delivery could not be performed. The accompanying `COORDINATOR-HANDOFF.md` and final response provide the completion message. No unsupported messaging API was improvised.

Limits: no dataset-wide hashing, unpacking, live experiment, source edit, Git mutation/fetch, global cache modification, package installation or full machine census. Listed archive byte totals are member metadata, not a new integrity certification. Source/binary hashes checked today are distinguished from hashes merely read in old receipts. Other hosts' current product truth remains the responsibility of their separate inventories.

---

[COORDINATOR-HANDOFF.md](/tmp/file-native-host-inventory-linux-20261024-01a16fd3/COORDINATOR-HANDOFF.md)

# Linux inventory completion for coordinator 01a16f73-5070-7431-868e-b9e0e174b06e

Host inspected: **`spenser-linux`**, native Manjaro Linux kernel `6.16.8-1-MANJARO`, ext4 on `/dev/nvme0n1p2`. Date: **2026-10-24**. The supplied connection label was `spenser-linux-codex-tail`; kernel hostname differs, but expected experiment roots and historical matching Linux platform records are present. No evidence of a different machine holding all of this Linux work.

Full report: **`/tmp/file-native-host-inventory-linux-20261024-01a16fd3/REPORT.md`**. Supporting compact inventories are in the same directory.

Key findings:

1. `/home/spenser/file-native-online-linux-20261020/source/final-engine` is a **standalone final source copy**. Its copy grant identifies `0628322e4b48e0efad19cdc32238a622d4aaa7a8`. All 25 imported Rust/Cargo/consumer files match final freeze SHA `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`. Five CP4 documents/protocol files in the larger freeze were not imported. **31 tests explicitly UNRUN; no final native ELF or final product pilot/matrix evidence found.**
2. Online native target and preserved binary are earlier **CP1 62ce9630…**, SHA `11c306c4fa30d74a6f83274666024591e0e150674419d1e4dbe06e9a251a34f5`, verified today. Historical build succeeded, with provisional sampling qualification; no engine call in that build record.
3. Later **v3 interface controls DID run**. `evidence/v3-control-events-e361a15b3e85437bb09601061ea80a58-report.json` has 16 results and no failure; raw event hash matches. Guard receipt exit0/closure/no censors. It expressly says **no engine case qualified**. Do not mistake earlier preparation-blocker prose for the last state, or controls for engine qualification.
4. Earlier P core **d70f3864…** has real native product evidence: 5 units; 52 packing cases/267 events; 24 strict cases; 8 focused/25 events; 4 immediate cases; later fresh recovery **24 cases/282 events** at `/home/spenser/file-native-p-linux-20261020/recovery-only-20261020T054348Z/result-complete.json`. Readback counted 24 case rows and 282 resource rows. Current native final ELF is SHA `a8501737ca4fdd5cfc177620860c1ae103d56f2e9b2151104a922df511a3d84b`. All145 source files still match. Earlier bootstrap/report failures remain preserved; global qualification fields remain null.
5. P's missing fixture trees were archived then reclaimed historically. Preserve `preservation-artifact/two-roots.tar.gz` (contains earlier actual 80,976,318-byte unit fixture family) and `preservation-three-20261020T060930Z/three-roots.tar.gz` (215,825,212 logical bytes of final/recovery stores). Earlier boundary execution does not establish the final online boundary result.
6. Additional live families: N final **4bde1563…** (163 source files match; 39 focused cases/153 calls, 8 README cases), earlier N four successful N128/N1000 A/B navigation cells and cancelled N10k prefix; Q **f0904a63… / core07a8574…** with failed first capsule and successful v2 continuation; recovery **40509/ec544/fe1ee22** generations; incremental **f3e942f/7ac70f9/7e00d22/5a65a21**, prerequisite **379af4**, old source copies, separate native targets, archives and raw receipts.
7. No local Git checkout for `atomize-hq/handbook` was found. `/home/spenser/__Active_code/project-handbook` is a different, old `Spenquatch/project-handbook` repo at e6df715 with six dirty tracked files and one untracked guide. No experiment ancestry/unpublished commit conclusion can be drawn from it. No fetch performed.

Smallest next product action: **compile/run the exact final 31-test native Cargo workspace, including the real boundary fixture; then the retained 37-call pilot if it passes**. No need to restart the old launcher campaign first. Nothing was executed by this audit.

Coverage gaps: no Windows/Mac or other-host access; bounded source discovery rather than all unnamed nested directories; no dataset census/hashing; no exact original Path.is_junction()/SIGALRM copy-helper failure or repair receipt recovered. Those calls are absent in the inspected online native/shared source; old alarm-using administration exists separately. Current inventory Python is3.10.11, system Python symlink3.13, historical native report3.13.7, shared source-validation receipt3.11.9. No environment repair attempted.

Searched home immediate experiment roots; `__Active_code`, `.worktrees`, `.codex/worktrees`, task-worktrees/validation/archive/handoff locations; bounded Documents/archived_work; relevant visualizations dates; Downloads, `/tmp`, `/var/tmp` names. Capitalized `/home/spenser/__Active_Code` and `/home/spenser/__Active_code/atomize-hq/handbook` are absent. No selected access-denied root. No current visible experiment process found by targeted arguments.

Delivery limitation: **`send_message_to_thread` is not exposed in this session's tool catalog.** This prepared completion note and the task final are the available handoff; coordinator must retrieve the report through the existing Linux connection. Existing source/evidence/configuration, dirty files, archives and failures were preserved; no builds/tests/imports of experiment modules, Git mutations, cleanup, installation, or additional owners.
