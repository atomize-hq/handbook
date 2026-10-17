# Requirement-to-evidence matrix

This is a partial qualification, not a completed native-platform certification.
Current raw evidence is run 3 in [the index](evidence/raw-index.json).

| Requirement | Executed evidence | Result / limit |
| --- | --- | --- |
| Unique requests, attempts, generations, publications; acyclic hashes | Rust model/selection; 120 repeated races | Separate IDs and selector → publication → state/closure → payload digest chain; no manifest self-hash |
| Positive publication lineage versus content ancestry/orphans | Two complete candidates, explicit losing installed candidate, no-op content publication | Only selected predecessor chain used; full losing candidates retained; equal content still changes publication identity |
| Gate dedup and different-intent refusal | Same-ID candidates prepared by separate processes; selection recheck | One accepted original result, later same intent deduplicated; different intent refused |
| Terminal cancellation fences eligible attempts | Prepared candidate, cancel under same store lock, later selection | Fenced candidate refuses; ordinary absence is explicitly nonterminal/as-of |
| Exact lost-reply outcome beneath C edit/delete | 120 repeated runs plus post-boundary faults | Original accepted result recovered beneath successors without replay; original immutable payload/history retained |
| Uncertain visible predecessor followed by successor | selector-visible exit/error/kill and successor | Successor re-syncs visible predecessor selector/name before acknowledgment; Windows process-only qualification |
| Exact source bytes and immutable snapshots | LF/CRLF, exact JSON with `1.00`, capture then later editor edit | Stored bytes/digests and editor privacy verified; source is not JSON-reserialized |
| Import base on all routes / nested duplicate keys / captured moves | Nine supplemental validation checks plus two combined import/move probes | All routes reject null-base import; recursive duplicates rejected; later editor changes/moves remain private and both selected paths/retained packets verify |
| Graph constraints after combining separately valid commands | 480 semantic race observations: 30 × 4 cases × 4 option/store configurations | Delete/add-edge, label uniqueness and aggregate budget refuse after explicit revalidation; rename/add-edge retains stable target identity |
| Safe disjoint fixture reprepare | Main races and growth conflict resubmission | Explicit new base/full validation only; no general merge, read-set OCC or external effect replay |
| B full readable closure and relative navigation | Destination reread/hash/name check plus fixture link checks; deep attachment | Complete local fixture closure at selection, including packet/provenance; parser is intentionally narrower than arbitrary Markdown |
| A view freshness and render-only recovery | A view-copy/replace/refresh checkpoints; resume; status marker | Accepted state remains selected, refresh resumes without mutation; ordinary tree can mix and marker does not bind a reader |
| Paused ordinary reader | 120 main repeated runs | A observed old/new; B exact named generation old/old; alias per-open control mixes |
| Paused actual copy/enum | Two supplemental child-process copies | A copy mixes; B full copied manifest and ordered packet verify after successor publication |
| Retained assemblies/provenance | Main copied packets and independent root-vector packet | Ordered revision hashes, exact input paths, renderer/recipe, omissions/output digest; A includes reachable local dependency closure |
| Independent stores/processes | Project lock held at native barrier while research reads/publishes | Independent progress; root vector + retained copies/agreement, no global atomicity |
| Repeated failure in one process | Three interrupted candidate preparations followed by same-process success | Unique staging does not wedge retry; separate actual partial staged payload test retries successfully |
| Process fault oracle | 233 fault rows, including 51 must-retain post-boundary rows | Old authority before selection; qualified selector-visible point; accepted original retained after boundary; every matching helper instance is not exhaustively injected |
| Damage versus absence/projection lag | Missing selected payload/state/publication/selector; actual metadata-byte corruption; missing A view | Typed integrity errors under explicit check/audit; ordinary missing key is null; absent view does not erase authority |
| Windows no-delete-sharing handle | Actual native selector handle without delete sharing | Qualified refused/uncertain selection followed by successful retry after close |
| 1k/10k/100k per store and B 20/10/3 retained generations | Growth raw samples, summaries and actual inventories | See reached configurations, preserved rejected generations and unreached cases in comparison; no rolling deletion |
| 5,000-constituent entity, fanout, bounded larger source | 10k+ fixture graph; serial 64-doc batch includes captured 256 KiB attachment | Counted in state, navigation, verification/copy and A packet dependency retention |
| Speculative versus per-store admission | Allocated waves 1/2/4/8 at 1k; reached 1/2/4 at 10k; four separate 64-doc versus three staggered small-request cases | Conflict/busy/refusal samples and wait/hold metrics retained; not FIFO/starvation qualification or sustained fairness study |
| Stage costs, memory and storage | Raw metrics + FileStandardInfo allocation inventories | Inclusive stages overlap; diff/hash CPU/ack not all independently timed; directory/MFT allocation not attributed |
| Fresh-process lookup | Three inspect processes per completed growth configuration | Positive and negative state lookup, uncontrolled cache; not cold device; same whole manifest is loaded |
| Native Windows/NTFS | MSVC executable, native child processes and file handles | Process-fault evidence; directory sync explicitly unsupported; OS crash and hardware persistence unproven |
| Native Linux/ext4 | Existing SSH target authentication probes | **Unmet**; configured key/normalized temporary key could not authenticate; WSL not substituted |
| Explorer / ordinary Linux browser | Explorer helper observation and attempted navigation | **Unmet**; fixture navigation not observed; Linux desktop unavailable |
| macOS/APFS/Finder | No native run | Follow-on: naming, handles, directory sync/full flush and same logical suite required |
| Gate 3 lazy COW + at most one external comparator | Not entered | Earlier required platform/UI qualification incomplete; no integrated adapter or new comparator timings claimed; Dolt benchmark-only |
| Actual metadata-backend damage, retained-root reopen, pins/GC and retirement | No Gate 3 backend | **Unmet/conditional**; no retention defaults assumed, all file-only advertised bases retained |
| Coherent rollback freshness, OS reboot/power cut, production owner integration | Outside this process experiment | Unproven; no production selection, migration, reclamation or successor authorized |

The [causal review](review.md) documents initial defects, fixes and rechecks. The
Windows process suite does not promote proposed format defaults or certify
existing production owners. GitNexus was stale/degraded; new experimental symbols
had unknown impact coverage. Native builds, explicit process tests and the staged
file list provide bounded evidence; they do not replace missing graph coverage.

## Final artifact and scope check

`verify_evidence.py` verified eight compressed raw logs / 14,847 JSON records,
source/build pins, exact archived proposal bytes, fault phase counts, 10 completed
growth configurations and two censored 100k initializations. See
[verification.json](evidence/verification.json). The measured executable SHA-256
is `1b9a6caef1113d0c6bb62c52189feb2126bdb531c407b728cd6b3c36b0d9e584`.
The corrected Rust source was not changed after that measured build.

Final allocated file bytes across all three runs plus build files were
10,325,539,176 (9.62 GiB), with 247,240,355,840 bytes free at inventory. These
file-allocation totals exclude attributed MFT/directory-index space, as documented.
Both large commands ended on their 600-second timeout before initial selection.
The cutoff guard subsequently found no active growth process at 16:48:01 UTC;
its null PID arrays do not denote an additional killed process.

GitNexus `detect_changes --scope staged` reported touched files without indexed
symbol overlap, explicitly **not a clean tree**. The index remains stale and
new-symbol impact coverage unknown. The independent manual staged-path check
confirms all additions are beneath this experiment directory; no existing tracked
production file, root dependency or research input is modified. Original untracked
proposal/consultation inputs remain in place. No push/deployment or successor
task is included.
