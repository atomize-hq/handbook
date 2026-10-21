# Online maintenance experiment closeout

Disposition: **ADJUST — INCOMPLETE; no operating envelope demonstrated.** The fixed experiment window ended before the mandatory final-source qualification and workloads ran. This is a failure to complete the experiment, not evidence that the fixed engine passed or failed its performance targets. Source review and static checks are recorded separately from native runtime results.

The engine source remains pinned to `0628322e4b48e0efad19cdc32238a622d4aaa7a8`. The isolated research archive was pushed as `9de962120e99c1b8dd32f8f2f3e13c4383b1d795` on `codex/file-native-online-maintenance-20261020`. No merge into the protected main or prior integration checkout was performed.

## Fixed clocks

| Event | UTC | Status |
|---|---|---|
| Original origin | 2026-10-20 19:10:14 | Unchanged |
| Intermediate source watchdog | 2026-10-21 04:10:14 | Missed; not retroactively passed |
| Native/matrix target | 2026-10-21 10:10:14 | Missed |
| Functional cutoff | 2026-10-21 11:10:14 | Closed; no new executable source/workload/build/copy starts |
| Protected closeout endpoint | 2026-10-21 13:10:14 | Evidence, census, cleanup and isolated publication only |

There was no clock reset or extension. The04:10watchdog was not a blanket functional stop; later bounded repairs were prospectively funded. None of those repairs or late reviews backdate runtime qualification.

## Native results

Windows Guard R4.3 ran from09:55:07Z and stopped09:55:58Z with exit2. Case01 completed. Case02 recorded45.9856ms cadence and the outer record42.3152ms, exceeding40ms. Cases03–06 recorded expected RSS-limit negatives; case07 is partial and08–27UNRUN. Its retained raw bundle is2,820,283bytes/45files and protocol93,523bytes/32files.

Windows Guard R4.4 ran10:41:48.536–10:42:43.854Z and stopped with exit2. It did not record a cadence breach: outer maximum33.8335ms, maximum sample5.6295ms. Case02 exceeded the256row collection-peaks cap: two128-entry maps fill256rows; the129th identity requires258. Cases03–10 recorded expected bound controls;11 is partial and12–27UNRUN. The outer error is only`outer-monitor:RuntimeError`; its unique cause is unobservable in this evidence. Do not relabel it a cadence, deadline or outer-cap failure. Retained raw is3,489,570bytes/62files and protocol99,592bytes/32files. Neither attempt produced complete27-case qualification.

R4.5 added bounded source-frame diagnostics. Its independent affected review began11:13:07Z after cutoff and found no blocking diagnostic-source issue. The third27-case attempt is **UNRUN**, both output roots absent. The proposed512row peak allowance was never executed; it does not repair or rescore R4.4 evidence.

Mac C1's only attempt stopped10:59:32.199Z after starting10:59:30.693Z: a2998byte launch exceeded the keeper2048byte control-message cap. It created no worker; worker exit is null and C1remains absent. The staged-bootstrap60s retry never launched. Guardian18877 birth[1792580370,646646] and keeper18878 birth[1792580370,718264] were reported closed/reaped with two numeric absences each, token released and registry empty. No Cargo/build/list/unit result exists.

Linux did not execute the final35file transfer, finite8, C1, final build/list/31 or full158. At11:15:57Z its role reported no owned root-process matches, heavy-session lock free and final-suite-v5/Cargo-private absent. Earlier five targeted peer controls and old-source compile evidence remain historical only.

## Required work that remains unqualified

| Required cell | Final status | Qualification limit |
|---|---|---|
| Final engine062:31 unit bodies, Windows/Linux/macOS | UNRUN on all three hosts | Earlier compile-only/old-source tests do not qualify this pin; five workspace IDs share one unfiltered31 proof per host. |
| Windows27 guard controls | Two unqualified partial attempts; thirdUNRUN | Case01 completed in each; neither produced complete27-case qualification. |
| Windows166 platform cases | AllUNRUN | Strict24/packing52/recovery24/focusednew43/nativepairedstream/ordinary and crosswalk obligations remain. |
| Linux finite8 and complete158 | All final-sourceUNRUN | Reviewed subset not copied; full158 never admitted. |
| Mac direct C1/cache | First attempt refused before worker; retryUNRUN | C1absent; no Cargo/build/list/31. |
| SmallA/B N64/H64 online schedules | UNRUN on final source | Threecycles, accepted revisions, writer/readers, old pin and successors required. |
| LargeA N128 H1002/H10002 | UNRUN on final source | 1600accepted/4000offers/2400exactNOOP losers and reused H1802/H10802 stores required. |
| Initial and warm/fresh lookup categories | UNRUN | Six categories,50fresh/500warm, legacy then explicit-paged with exact provenance. |
| Sixteen paced HISTORY phases | UNRUN | Fourprocesses, tenbatches,1000warm+40first perphase; fixed45s origin/publisher/readers/oldpin/lag constraints remain. |
| D document controls | UNRUN | FourA/B×N64/N1000pairs;8stores/16publications/8completeH3oracles, no third seed. |
| LONG H101007/H504833 | UNRUN | Alllookups, archive/246MBfullrestore/fiveoracles and accepted H101008 successor required. |
| Windows1000 physical mapping activations | UNRUN | Native64 is not a substitute; no available latency distribution. |
| Quota70/checker runtime controls | All70UNRUN | Source acceptance and4MiB/32forecast are not runtime/cap-fit proof. |
| Context/Archive38 controls and retirement | Actual38UNRUN; RETIRE NO_GO | Pure fixtures/source review only; actualkillproducer and166qualification adapter missing. |
| Cleanup/reclamation and target envelope | UNRUN | 1000/50ms cooperative controls, two actualpasses, fixedΔH10/H1≤2; no completePATH proof for unprotected0. |
| Activation/ordinary/warm/long performance gates | UNMEASURED | No valid p95/p99/max, matched1.25/1.5, warm20%/.2ms orlongfresh≥2x conclusion. |

No cell was dropped, replaced by a smaller workload, or inferred to pass from a static check. Earlier A60.71/B32.14 negative activation evidence remains retained; a new95gate was a separate proposed gate, not a replacement passing observation.

## Source review disposition

The final source manifest binds actual bytes and hashes, source-only review status and preserved negative snapshots. Q076's exact174byte addition accounts for17live unsettled checker-output files; the reviewer resolved the sole Q1finding. RuntimeR3+069, finalizer, checker CK5, corrected D, Context CONTROL and Archive source reviews are source-only acceptances. LinuxV5.4 accepts only the bounded finite8/C1/build/list/one31subset as source; full158 admission remained absent. WindowsW1–W3 repairs received an affected-source acceptance at11:14:16Z after cutoff. All actual manifests remain frozen, including their historical pending wording; this closeout record records subsequent review status.

One actionable source defect remains in the unexecuted Linux copy operator8188bytes/SHA`a90d63d412eda6f1cca2820b0c7fc42829bad9646e6ca4fb12e50e2a3da41f3b`: lines15–17 arm`ITIMER_REAL` without explicitly restoring the default`SIGALRM` disposition. An inherited ignored signal can defeat its remote self-timeout during blocking I/O. Full static closeout review is **SOURCE_DEFECTS_NO_GO**. The author's8185byte RAM repair was never published or reviewed, and no repair was made after cutoff.

## Resource and preservation record

The final conservative reservation remains47,156,051,614bytes against47,244,640,256bytes(44GiB), with88,588,642central unassigned;4,827,164files reserved and172,836unassigned. The1GiB/100,000file inside-closeout reserve remains held. These are conservative reservations, not a fresh summed all-host physical census. Claim079(SHA dd81832ae10bff18804c746dbbbaea543241a3b7fd04387ba74aeb12cd04f60a) and its MANDATORY reserve addendum(cd41880edf94b3f2f06504bc70027aa0d4f713d5ba621f5cbccda3b9e57aa63b) bind64KiB across exactlyfour publication names. The original079pending total112,617 was wrong;104,425legacy future+65,536publication=169,961 is authoritative. PrepublicationMETAactual11,689,643+future169,961=11,859,604≤11,862,016, leaving2,412bytes. The final machine record records publication consumption.

Only never-materialized forecasts were closed: the thirdGuard phase and unused review allowance. All actual evidence remains; backing global/native category ceilings were retained, with no physical-deletion credit. NativeMac reportsF01 3,074,809bytes/174files,F12 8,897,379/1298,C1zero; Linux reports source1,292,813/110,evidence4,401,269/196,target126,813,675/409,fixtures8,319/3. Protected Cargo stores were not changed. Windows raw Matrix evidence30,041,078/1555 and protocol7,015,726/1307 remain retained. Git preservation uses the existingF13 24MiB/5000positive-growth cap and4MiB/128local-admin subset; no extraMETAphysical archive copy was made.

A historical control-plane violation is retained separately: at06:54:54Z a worker invoked the forbidden GitNexus bootstrap wrapper. The newly created installation was1,050,910,072bytes/15,045files, producing a conservative forecast excess827,190,294bytes above44GiB. The bounded undo removed only newly created names and retained the old cache; no shared-inode freed-byte credit was asserted. Conservative residual18,048,297bytes/288files was funded. Unknown installer effects were not proven absent. Later admission headroom does not retroactively make that event compliant.

The archive retains rejected source versions, both Windows native negatives, source-review negatives, exact budgets/claims and the Mac returned source packet. Index-only Git preservation avoided an extra physical META copy. Files marked skip-worktree are deliberately absent locally; inspect them with`git show` from the committed archive or use a separately funded checkout. The exact9,286byte Mac refusal receipt was returned inline, hash verified, and retained as an index-only Git archive member. Fresh process/census observations are recorded separately from the original refusal receipt.

Cached GitNexus detect_changes was invoked against the exact isolated checkout before commits. It returned repository-not-registered; no bootstrap/index refresh was run. Graph scope is **UNKNOWN/manual HIGH**, not a successful automated check. Parent manually verified all staged paths stay in this experiment research subtree and every staged object matched its intended raw byte hash. No functional tests were started during closeout. Protected main`379af4eda9b95ee82be9d3afcfdcdca2d6d132f9` and prior integration`a0b92fe5dc64269508601dcddf7e2e2ca477e30d` remain unchanged.

## Consequence

This run supports improving admission/guard integration before a separately authorized experiment. A future run would need a complete final-source native guard result, bounded SDK/cache preparation and the complete31proof on each host before releasing the unreduced matrix. It must retain these failures and start with a new explicit budget and clock. No future run, retirement, deployment or clock extension is authorized by this closeout.
