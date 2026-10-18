# Requirement-to-evidence matrix

This is a bounded process-interruption/storage experiment. It does not select a
production backend, browsing contract, owner migration or retention policy.

| Area | Executed mechanism / evidence location | Qualification |
| --- | --- | --- |
| Windows + WSL | Native MSVC/NTFS and GNU/Ubuntu-24.04 ext4 executables; exact final source/native pins in evidence/pins.json | WSL uses Linux kernel/filesystem APIs on a Windows-host/VHDX stack; no separate-host/macOS or power-loss claim |
| Publication/recovery | Full process-oracle suites on the Windows manifest/COW and WSL manifest/COW development builds; final affected-boundary regressions on v4 | Full suites and final regressions are separate datasets/binaries. Final source changes are reviewed and specifically tested; do not label all historical rows as a v4 full-suite rerun |
| Phase oracle | Preselection must-not-select, selector-visible uncertain, after-boundary must-retain beneath later successors; first/middle/last copy hooks and prepared COW root hooks | Process exit/error/kill leaves kernel/device caches alive; not every syscall or reboot/power cut |
| Unix immutable install | Eight OS writers, short stage, existing inode retry, differing/corrupt winner refusal, death after atomic name link | No-replace link on same filesystem; stage remains private, winner is checked; no selected repair |
| Nested mkdir | Interruption at creation then retry, successful native directory sync counts including pre-existing parent | Name-sync protocol exercised, not proof of storage-device persistence after power loss |
| Windows sharing refusal | Actual selector handle without delete sharing, qualified refusal/uncertainty then explicit retry | Cooperative local process/NTFS fixture, no arbitrary namespace attacker qualification |
| Graph semantics | Two initially valid candidates, exact-publication conflict, explicit revalidation of delete/edge, uniqueness, budget and stable-ID rename/edge | Both project and research consumers; no automatic merge or read-set OCC |
| Request outcomes | Same-ID/same-intent competing candidates and duplicate deletion race, different-intent refusal, lost reply beneath delete successor | Original result retained after snapshot retirement/reclamation; receipts are caches; no exactly-once external effect claim |
| Cancellation/progress | Waiting signal, durable fence, killed lock holder, independent store progress; 1/2/4/8 process waves | Busy waits include timeout path; per-request endpoints include pure retries. Lock is not FIFO and observed completion does not prove starvation freedom |
| Growth | Instrumented source/metadata/materialization/verification progress; paired A/B, manifest/COW; selected history tiers 20/10/3 when completed | Reached/censored cases and exact inventories are in final native evidence and operation tables. No snapshot deletion to improve growth |
| Commit vs assembly | Explicit packet=false, packet from selected revision, and commit+packet; small and 5,000-constituent closures | Same closure/provenance within each A/B workload. B still copies complete readable generation without a requested packet |
| COW comparison | Prepared pages and descriptor before application selector; lazy point, positive/negative key, historical reopen, full graph validation on commit | Point verification does not audit unrelated pages; end-to-end timing includes lineage. Actual page damage and broad audit check distinguish that boundary |
| Retained lifecycle | Pins cannot rebind names; retirement refusal for pins/packets/current head; shared preparation gate; pending roots retained | Explicit lifetimes only. No timeout/TTL proves reader death; no invisible pin registration for arbitrary tools |
| Actual reclamation | Disposable roots only; actual immutable originals and obsolete COW pages deleted; interrupted sweep/reopen and retirement between records | Publication/outcome/fence history retained indefinitely in this proof. Directories and emitted packet copies may remain; no production GC or retention horizon selected |
| Suspended ordinary reader | Actual copier remains paused while pin is active, collection occurs, then copied exact directory verifies; retirement follows explicit release | After explicit retirement an arbitrary tool-free reader no longer has a lifetime promise |
| Cross-store packets | Independently selected roots, retained copied packet bytes and agreement/vector outside swept stores | Independent captures, no global transaction/atomic snapshot |
| Tool-free discovery | PowerShell/Bash directory/text/search/copy/hash walkthroughs; positively selected binding; overview, kinds, stable IDs, relationships and deep attachment | Eight final walkthroughs on 16-document research fixtures; no GUI, unfamiliar-human study or 100k manual-discovery measurement |
| Ordinary copy during publication | Actual suspended userspace enumeration/read/copy | A may mix; B fixed path and retained packets remain coherent. Alias-per-file control mixes |
| Damaged A view | Missing source/packet output, external source/navigation edits after complete marker, explicit audit, preserved divergence, death after preservation, render-only resume | Marker does not prove continuing integrity. Corrupt non-retired prior metadata blocks rebuilding truthfully; no import or authority repair |
| Exact source bytes/names | LF/CRLF, exact JSON 1.00, YAML/Markdown, portable hostile titles, case/Unicode collisions, title/path moves with stable ID | Source bytes are not reserialized. Fixture grammar covers Markdown links and explicit NAVIGATION locators; incompatible cross-depth move refuses |
| Artifact/source scope | Source/harness/native hashes, deterministic raw archives, native platform/allocation sheets; focused causal review | GitNexus graph coverage is stale/degraded UNKNOWN; isolated imports, native execution and manual staged paths provide bounded independent checks |

## Evidence association

The raw event's executable SHA-256 identifies the actual binary. Development
archives are kept under explicitly named evidence/development-* directories;
they are not pooled into final latency tables. The final v4 native measurements
live under evidence/windows and evidence/wsl. Per-invocation
harness manifests identify the files used for final measured sessions. Source v3
and its pins are preserved for the full WSL COW suite; v4 adds derived-view damage
protection and its specific interruption regressions. Full publication/graph
algorithms otherwise remain the reviewed shared implementation.

The artifact verifier checks archive bytes/counts, final source/harness hashes and
the recorded phase classifications. It cannot turn an unreached case into a pass
or independently establish that a kernel/device honored a flush after power loss.
Positive local hashes also do not prove freshness after coherent rollback.
The pre-100k WSL budget amendment archives the five earlier operational harness
files and the exact native 10k raw-log prefix behind the decision. Verification
accepts only current pinned files or those exact archived bytes for recorded
harness manifests. The Rust core and both native executable hashes are unchanged;
later WSL large caps are 2,400 s init / 900 s publication, with the original disk,
worker, corpus and closure requirements preserved.

After both measured schedulers exited, a controller-only capture amendment
preserved exact stdout before malformed-response/timeout refusal. Its exact old
source remains in the budget ZIP. Original capped events have `output: null`;
their exact historical stdout bytes were not captured. New-controller sessions
require Base64 bytes, length/hash and a type-preserving parsed-result association.
Eight synthetic transport events are separate from real-engine native events.
Adversarial reporting checks reject wrong/missing source/session association,
stdout hash/length/result/type and failed/uncertain completion samples.

## Reached final checks

Windows completed all 16 scheduled phases. WSL completed 14 of 16, with both
100k A initializations capped at 2,400 s during view refresh after accepted
selection. Read-only native diagnosis confirmed authority before any continuation.
Separate post-cap resumes completed in 460.350 s (manifest) and 442.286 s (COW),
then both missing combined publications completed. Each store retains three
selected publications and a complete view bound to its current head. Subsequent
retained packet and twelve current/initial lookup probes passed. Neither later
success replaces the original censored initialization observation.

Final focused v4 regressions passed manifest/COW on both platforms. Restoration,
actual indirect-object reclamation and large-indirection/unrelated-page damage
checks passed on both platforms. PowerShell and Bash each completed all four
backend/mode extractions. Final accounting covers 24 growth/contention stores per
platform and validates 348 selected publications per platform. Eligible pending
candidate bytes are zero at closeout; stale rejected candidates remain retained
and separately counted. That final count is not a strict peak candidate quota.

[Archive verification](evidence/verification.json) passed 61 raw archives with
16,147 indexed records, including duplicate archives. Whole-file content
deduplication yields 60 distinct contents and 13,949 records; it does not
deduplicate cross-file events or test cases. The verifier associates 2,350 final
real-engine native events, eight separate synthetic transport events, 161
lossless-stdout events and 22 helper observations. These are association counts,
not independent test-case counts.

The nine generated tables passed consistent-column checks: 24 project operation
and status rows, 112 changed-batch rows, 48 lookup-binding rows, 32 retained growth
store rows, 40 amplification rows, 86 serial process-operation rows and 64
contention cohorts. Process lookup also has 48 binding rows. All 240 contention
requests met the planned 60 s observation budget (maximum 2.958 s); it remains an
observed result rather than a separately enforced aggregate watchdog.

Final scoped runtime/build plus repository artifact file allocation was
46.31 GiB at capture, below 48 GiB; host and guest free space were 211.23/482.90
GiB, above the 160 GiB reserve. Small subsequent textual closeout records are
explicitly qualified in [the resource snapshot](evidence/resource-final.json).
Directory/MFT allocation is excluded; host VHDX and guest file bytes are not
summed as separate physical use.

Both required GitNexus checks ran. The staged check reported 399 changed files
with no indexed-symbol overlap, preserving UNKNOWN graph coverage. Comparison
with `main` reported CRITICAL risk (2,518 files, 28,207 symbols, 296 processes;
symbol listing capped). Independent Git path sets reconcile 2,119 pre-existing
committed branch differences plus the disjoint 399-file research addition. That
broad branch result is retained, not presented as a safety pass for this change.
The CLI also warned that Node 22.15.0 is below its supported engine range.
Final staging adds only textual closeout records within the same new directory.
Exact transcript bytes include intentional authored whitespace; `.gitattributes`
narrowly disables whitespace lint for the captured Bash/PowerShell transcripts
while retaining their text diffs and exact hash checks.

## Explicit later qualification

Separate-host Linux and macOS, actual reboot/power-cut behavior, native owner/root
grants and lock identity hardening, noncooperating namespace/editor races, complete
reference parsing, retention policy/outcome horizon, rollback witnesses and
production integration remain later work. No SSH/system repair, GUI automation,
mandatory database or new scheduled service is part of this experiment.
