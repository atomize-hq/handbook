# Independent full R4.2 source review

Overall verdict: REMEDIATE HYST. One P1 remains in its source-freeze timestamp validation.

Scoped verdicts: SOURCE_ACCEPTED_FOR_BOUNDED_PHASE_ADMISSION for the isolated exact native27 controls route and fixed native-ssh-phase finite8 transport route, subject to separate actual all-sink/runtime admission. This source judgment grants no runtime GO, native pass, remote qualification, cache/build/body execution, Context/RETIRE authority or HISTORY grant. The broken HYST grant path is unreachable from those two routes. HYST remains NO_GO until its affected source is repaired, frozen and independently reviewed.

## Exact frozen source read

All five files were read completely as text: job_guard_bounded.py1–1539, unlocked_capture.py1–1123, controls.py1–296, README and source receipt. No partial read is represented as full review.

| File | Bytes | SHA256 |
|---|---:|---|
| job_guard_bounded.py | 77668 | 48feda3aee5cabe4140ea6ca5604e7a4dce43276b82de86e5cf1d409d6fdd6a7 |
| unlocked_capture.py | 63277 | 082a2c3b6b22326988da17baab735c589a7c4fcc82adaf47e83890f05b7e84b8 |
| controls.py | 14902 | 24b88e544417d909cebdd5dd4e4cb993e35dcfa230b30d3f3059cb3238c2d9c5 |
| README.md | 4050 | 5331687bf0ce536fbfc42c6e9295ede0b92912af90a4cc530924d031e2939ae3 |
| source-receipt.json | 3225 | 9c50fc67635fed393deba287b801d631512588591c2b3846922f372464904813 |

Total163122 bytes/5, within058166912/5 and each individual file cap. Budget058 is3834 bytes, SHA256ea7e3478651f0dc53c8a04dd8d74e0a8d9b7073535eeb8c5d13e289e0ec967d1. Initial hashes matched the coordinator freeze09:15:08Z. Final pre/post report hash checks accompany delivery. Earlier R3/R4/R4.1 negatives remain historical and are not rescored.

The structured data format standard was read and remains proposed; existing owner/versioned contracts control. No migration or general defaults are adopted. Manual lifecycle impact is HIGH; GitNexus Landing remains UNKNOWN/unregistered and was not bootstrapped. Work was text/hash/metadata inspection only. No reviewed module import, AST/compile, native API, controls, SSH, Jobs, engine, Cargo, Git, source edit, subagent or new chat occurred.

## P1 — HYST's timestamp regex cannot accept a valid source freeze

Location: unlocked_capture.py:893–899, called unconditionally by _issue_postexit_grant:957.

The raw pattern is:

    2026-10-21Td{2}:d{2}:d{2}(?:.d{1,6})?Z

Here d is a literal letter, rather than an escaped digit class; the fractional separator is also an unescaped wildcard. A valid value such as2026-10-21T09:15:08Z cannot match. Values matching the required literal dd:dd:dd are rejected by datetime.fromisoformat immediately afterwards. Thus no harness timestamp can satisfy both stages, and the current HYST parent cannot issue a postexit grant.

The current finalizer's validator uses the intended escaped digit groups and literal decimal point at phase_finalizer.py:1386. Match that calendar/clock contract and validate representative valid fractional/nonfractional times, malformed input and hard cutoff refusal using a bounded pure-memory harness. Keep validation before publication; do not skip the source-freeze gate or weaken its approved-source association.

Scope: HYST grant issuance only. Controls and native-ssh-phase use their explicit separate dispatch paths and cannot call this HISTORY grant path. This is static source reasoning; no code was executed to manufacture a regression result.

## R4.1 allocation finding — repaired in source

Capture constructs the actual Budget at380 and compares its raw_hash with the reviewed config reference at381 before census, writers or native acquisition. The retained Budget object supplies Bounds, preflight, monitors and publication census. _config_ref is derived from the exact verified raw config at236. Actual budget_sha256, bounds_sha256 and config_ref are now present in outer OWNER482, stored result762, generic durable completion1030 and HYST durable parent completion860.

The current HYST adapter verifies sealed actual numeric inner/outer evidence_bounds, joins inner FILE_NATIVE_BUDGET/current-owner and outer budget SHA/nonce/guard/process hierarchy at windows_adapter.py:574–592. LaneHost calls this at61, before Evidence at65. The formerly missing outer budget field is now produced. The guard literal's parent_completion.required fields include all three new fields and agree with Capture's parent completion shape, which the current finalizer closes against that literal.

No new shared allocation-association blocker was found for finite27 or fixedSSH source closure. Their actual configs, budgets, receipts, summaries, completions and all other sinks still require separate finite runtime admission.

## R4.1 authority finding and input-cap alignment

Capture's exact finalizer-allocation-v1 now has12 keys: schema, run, source_pin, budget_sha256, parent_case_id, census, output, limits, caps, guard_bounds_ref, outer_guard_bounds_ref, source_freeze. The current finalizer closes precisely the same keys at1509–1510. Top-level postexit grant additions remain exactly authority_ref and profile_catalogue_ref.

Capture binds the authority inner whole-budget hash and ASCII insertion-order bounds hash to original returned completion; its outer whole budget/path/bytes/bounds hash join the actual old Capture and its result at958–975. Whole budget references support only absent offset or integer zero; arbitrary ranges cannot substitute. Machine source manifest/review metadata binds the fixed engine pin, acceptance verdict, exact manifest/source-file-set hashes and actual capture/guard/controls source entries. The bad timestamp pattern above prevents completing that validation; schema alignment alone is not HYST acceptance.

The inner receipt read ceiling is now the verified sealed inner budget's final.json_bytes, explicitly8192..33554432 at223–224. publish_parent_records reads the entire exact referenced receipt under that retained cap at840. reference enforces actual byte count/hash and bounded same-buffer read; it does not truncate a larger receipt into an apparently valid one. The worker's file_ref derives its cap from the inner budget and publication joins the returned actual budget hash. The current finalizer requires guard_json_bytes to equal that actual inner final cap, bounded to32MiB at1550–1551; outer receipt cap is likewise sealed-budget-derived and <=32MiB at1549. Raw/gzip/row/count parser limits remain joined to actual evidence_bounds at1552–1555. No old2MiB inner read check remains in Capture.

Current finalizer validate_native_lifetime reads outer receipt under the actual outer cap, checks its budget/bounds hash, creator/host/Job/closure provenance and errors. It compares original inner stored/returned/native lifetime records and requires actual parent reaping. These relevant current contracts were read; this report is not a full review or runtime acceptance of all unrelated HYST implementation.

## Lifecycle and bounded evidence reassessment

| Mechanism | Source assessment |
|---|---|
| Final native denial accounting | Both SAME still-held Jobs query final limits and cumulative limit-terminated count before checked last close: inner1294–1306, outer742–751. Snapshot is retained before validation; query failure/denial censors health while cleanup proceeds. Separate between-poll denial regression remains unrun. |
| Inner pending acquisition | _ROOT_PENDING set before Popen1149, cleared only after positive containment1157, blocks next entry915 and participates in IO-unknown1243/retained context1266. Unknown acquisition cannot obtain physical closure from an empty Job. |
| Outer unassigned root | creating_root fences unknown acquisition; retained Popen native handle is used for owned unassigned-root termination. Actual root wait/reap, creation/signalled proof and checked close remain required. No PID-only cleanup. |
| Whole-call watchdog | Initial3600s clipped to11:10:14Z; controls tighten<=900 before resources. Worker clamp3600. Verified config only tightens the actual endpoint. The whole old+new endpoint must be admitted initially; no later extension/reset. Watchdog remains through durable outputs, final census and stdout. +12s expiry cleanup never qualifies; unresolved acquisition stays fenced. |
| Terminal sampling | Actual root-end and Job-tail witnesses check start/completion/recorded gaps <=40ms; raw witnesses and both terminal coverage records are required for qualification. Positive suspended root RSS remains required before resume. |
| Grant capability gates | Actual old outer qualification, no fences/expiry, actual native held new suspended child and old Job closure are required. These are preserved; HYST's timestamp defect still blocks grant completion. |

Native flags8968, inner12/outer16 active-process limits and process2GiB/Job3GiB commitment caps remain set/read back before resume, during polling and finally. At most4 creation-checked live supervisors with positive RSS are required; outer Job+supervisors <=4GiB. Registry16/member2048 are capacities. Sequential128 remains cumulative and requires>=129 observed identities.

Stored gzip remains single-member stored-deflate with exact raw/stored hashes, prospective row/raw/stored-byte checks, finish reserve and no compression credit. Bounded collections, errors, final/emergency/completion envelopes and actual postwrite census remain. Physical native closure is independent of health. Stdio retains bounded prefix/tail with109 or actual zero-byte read EOF;233 censors. Unknown pipe/native closure retains actual references and blocks later controller entry.

All27 unchanged control IDs/order/fault mechanisms were read. Their summary is prospective and capped65536. Failed/skipped controls cannot turn into aggregate pass. Pipe-fence preserves its immutable nonqualifying row and requires separate actual owned-resource reconciliation before admission can resume. No actual control, denial regression or27/27 result was produced in this review.

## Fixed SSH finite8 source closure

The route is unchanged and isolated: pinned System32/OpenSSH/ssh.exe1251328 bytes/SHA2568e1212d6b10bbab3dfd85077c1a7d0b60373fde702dfa9ea8c57924793bdb317; fixed spenser-linux-codex alias/flags; /usr/bin/python3 -I -S -B; fixed final-suite-v5 bootstrap and --phase/--manifest/--sha256/--phase-deadline ABI. Generic inputs still require nlink1; the system-image nlink2 exception is limited to the exact pinned bytes and same-buffer identity/metadata. SSH bytes were hash rechecked read-only.

Safe parameter basename, shlex-quoted fixed tokens, shared actual native/Windows endpoint, stdinEOF and stdout16384 remain. Actual local root image/PID/creation, Windows Job/RSS/IO/start-to-final sampling/census/checked closure and terminal refs remain. Complete Linux bootstrap read from R4.1 is still hash-identical8ce97085723d2657f9148bfa81bf3d70e67759c86872e30bd87b09805562a135; ABI matches.

Scoped acceptance covers source readiness to admit the finite8 transport phase. It does not authorize SSH execution, accept Linux V5.3 source globally, prove remote child closure, or interpret transport qualified=true as remote phase success. Separate Linux native guard, exact native manifest/source deployment and terminal semantic verification remain mandatory. Finite8 must qualify before cache/build. This mode cannot mint a HISTORY grant.

## Disposition and accounting

Finite27 and fixedSSH finite8 source closure are accepted for separately bounded phase admission only. HYST requires the date validation repair and affected fresh review before grant admission. Engine/full-body/Context/RETIRE and all actual runtime outcomes remain pending separate authority. Parent may preserve this exact negative snapshot before changing it; no source mutation is performed here.

The only new artifacts are this review and evidence.json in the exact065 R4.2 root, jointly<=24576/2. META v11 cap11206656 and hashf516f1475d072cef530dfab3cea18891a148cadef4fc954960fcb4b8f73d51a1 control. Each fresh census includes all physical selected files once. Future=160472+812+sum(max(0,24576-current occupied bytes of each of the five065 pair roots)). Occupied pair bytes are not charged again as a full reserve. Unused R4.2 allowance remains held until the parent explicitly closes/releases it; no prior closed hold is revived.
