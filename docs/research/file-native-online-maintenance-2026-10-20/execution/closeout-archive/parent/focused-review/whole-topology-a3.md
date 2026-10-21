# Exact-a3 whole-topology admission dossier

Revision 2 below supersedes the original null tight F1/F2 bounds with complete logical sufficient caps; overall admission remains NO_GO.

Decision: **NO_GO for native execution under 512 MiB / 60,000 files.** This means a complete prospective fit has not been proved. It does not assert that actual execution necessarily consumes more than 512 MiB. Frozen harness v2 remains accepted as source; no engine, harness, fixture, Git state, or admission authority was changed.

## Binding and scope

Engine source: a3f9805eccc19ede8dd5c735b54e675e98e63714, checkout C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18. All 25 entries in checkpoint3/source-binding.json were checked against current file lengths and SHA256 values; manifest SHA256 f21a616e822c906f75eabff04dd7e97435781ee7a4fa32171acd62d6f119317b. Its historical base/review fields are not the current pin. Retained reviewed binary SHA256 ce8b4d5cc6981e3ec4ee24e7d3fc25447550bd7d68b440d04bc334635d6ee2da.

Harness v2 source receipt ae78d32a4a14d8327c3b4090fea3ea05f4f69b8542c0e3c281692ec7acc7b63a; eight-file manifest bad1bce648243a8c9d44b4b09cbcffda5ed5e3d9d4edabae26e71d789211936f; nine files / 188,865 bytes. No amendment is made to this receipt. Current dossier is read-only source analysis plus two newly authorized review documents.

All quantities below are logical regular-file lengths unless explicitly called RAM. They do not prove native allocation, directory/extent/inode overhead, filesystem compression, RSS, completion, cleanup credit, or a runtime PASS. Raw input/output/evidence files require a separate bounded owner. None of the byte ceilings below authorizes a write.

## Whole F1 and F2: what is and is not established

| Case | Required exact schedule | Earlier partial COW upper | Whole 16/32 MiB proof |
|---|---|---:|---|
| F1 | A/N10; genesis + 130 + 132 accepted = H263; 262 retired; three builds; at most 160 bootstrap cleanup calls plus exactly two final tail calls | 13,533,086 bytes / at most 1,572 newly written COW nodes | **Absent** |
| F2 | A/N10; H1 refusal; H31 owned refusal; H151 either admission or owned refusal; if refused, at most H551; four build attempts total; at most 160 bootstrap cleanup calls | **30,558,000 bytes / at most 3,300 COW nodes** for frozen-v2 requests | **Absent** |

F2 correction: old requests had maximum length 15 and compact OutcomeRef length 503. Frozen v2 uses F2-later-eligible-399 (21 bytes), so OutcomeRef can be 509. With at most 17 branch entries, outcome insertion upper becomes 40,445 rather than 40,055 bytes. Membership remains 15,115. Thus 550 * (15,115 + 40,445) = 30,558,000. The earlier 30,343,500 is not the correct v2 subtotal.

F1 derivation remains 262 * (13,009 membership + 38,644 outcomes) = 13,533,086. These bounds include intermediate leaf/split/root COW writes, not just final reachable nodes. A no-op transaction leaves graph, projection, and change roots unchanged: disk::persist returns the existing root; objects::update on empty input performs no put; projection compares static FileSpecs before updating. It can still write candidate/publication/result/hint/allocation records, reservation/selector controls and refresh plans. Those are outside the COW subtotal.

Retained older actual roots were F1 7,701,780 bytes / 1,781 files and F2 H551 16,544,850 / 2,149. Those are measured old executions, not prospective upper bounds for this exact harness. F1 historical cumulative bytes_written 9,221,629 is neither its future retained peak nor a fit proof.

### Earlier gross ceilings

The companion JSON retains obsolete cumulative write-quota ceilings for provenance. Revision2 replaces them with case-specific whole logical bounds. The 128KiB Work limit is a per-file refusal cap, not the F1/F2 serialization size. Ordinary cleanup offers two save sites per call, plus at most two F1 old-generation reap saves:326/320 total opportunities. Every resulting plan birth remains counted in Revision2.

A useful tighter proof must enumerate, by phase: originals, derived data, graph/projection bootstrap objects, every membership/outcome intermediate node, all initial and later candidate/publication/result/hint/allocation records, retired markers, root config/selector/reservations/disposable/view/pending and lock names, every owned declined pack/index, every current and stale catalog, immutable Work plan and pin, selector/work/object/view stage residue, temporary coexistence, and separate raw evidence. For F1 the three build outputs coexist before actual cleanup facts; for F2 both owned declines remain charged until actual removal. Catalog 59 KiB and Work 128 KiB limits are per file, not whole-directory limits. Old Work plans cannot be treated as automatically absent.

## Fourteen unit obligations, one fresh planned fixture run

Exact source has **14**, not 13, unit tests. Existing build9 stdout reports 11 passed / 3 filtered. It therefore does not prove the full set. The filtered obligations are paged_positive_negative_boundary_and_cache, paged_four_routes_negative_duplicates_and_conflict, and loading_indexes_evicts_before_transient_allocation.

| Source test | New retained logical fixture upper | Files | Boundary retained |
|---|---:|---:|---|
| bounds_tests::bounded_file_and_indirection_routes_refuse | 1,049 | 2 | oversized 1,025-byte file and malformed compact indirection |
| disk::codec_tests::compact_refuses_damage_before_allocating | 0 | 0 | RAM-only damage/refusal |
| pack::tests::decoder_never_uses_advertised_size | 0 | 0 | RAM-only advertised-length refusal |
| pack::tests::typed_keys_exclude_controls_and_active_payloads | 0 | 0 | RAM-only closed key types |
| packing::tests::caller_limit_precedes_raw_and_zstd_bodies_after_all_routes | 264,793 | 13 | raw/zstd early+late routes, caller-limit hint |
| packing::tests::constructor_reservation_evicts_before_allocating_output | 63,765 | 11 | existing **96 KiB** constructor fixture |
| packing::tests::mixed_capture_and_cold_warm_measurements_reconcile | 16,384 | 8 | small legacy/paged capture plus catalog/config/pin/lock |
| packing::tests::paged_cache_identity_preserves_physical_instance | 724 | 5 | same bytes, different physical instance |
| packing::tests::paged_four_routes_negative_duplicates_and_conflict | 1,658,660 | 11 | all five descriptor pairs and lease retained |
| packing::tests::loading_indexes_evicts_before_transient_allocation | 80,976,318 | 8 | actual 64 MiB read-cache cap; sixth negative index retained |
| paged::tests::paged_exact_geometry_and_closed_keys | 0 | 0 | RAM-only geometry/key rejection |
| paged::tests::paged_positive_negative_boundary_and_cache | 331,732 | 2 | 2,400 records; positive/negative boundary/cache |
| paged::tests::paged_touched_untouched_and_malformed_refusal | 16,957 | 2 | 120 records; touched and untouched damage |
| paged::tests::paged_maximum_keys_split_before_total_index_bound | 0 | 0 | RAM-only maximum-length keys and index split |
| **One-run total** | **83,330,382** | **62** | **No test omitted** |

This is a static source-length calculation for one exact planned run, not an executed census. It includes negative/temporary residues and lease/lock files. Existing protected unit roots stay separately charged. Any failed full run followed by another run needs another whole residue allowance; there is no blanket replacement credit.

Paged index arithmetic: for n equal-length keys k, leaves=ceil(n/48); repeatedly ceil(children/48) through root. Let p be total pages. Index bytes = (78+2k)p + n(k+50) + (p-1)(k+66). At n=2,400,k=77 this is 324,532; raw pack=7,200. At n=120,k=77 it is 16,597 plus 360. One k=77 index is 359. Five 2,400-record pairs therefore occupy 1,658,660. Maximum k=225 fixture fits 55,366 records with actual index 16,191,779 / size_bound 16,777,059; the next record exceeds the total index bound. This is not the 65,536-short-key test.

Heavy cache: each legacy index is 53+56,000(191+50)=13,496,053. Five baseline indexes plus the retained negative oversized sixth copy = 80,976,318. Empty-pack content identity shares one zero-byte pack, plus one lease, yielding 8 files. Constructor96KiB: five indexes each 53+100(77+50)=12,753 = 63,765; five distinct-path empty packs plus lease = 11 files. Caller-limit allowance conservatively bounds zstd frames by raw+65,536, retaining all raw and zstd descriptor pairs and the 65,537-byte hint case. The mixed case's 16 KiB is an explicit generous scalar upper for its tiny fixed fixture, not a measured size.

**Temporary root routing is required.** bounds_tests uses std::env::temp_dir(); the packing/paged tests use EXP_PACK_TEST_ROOT. TMP, TEMP and TMPDIR must all resolve to a registered fixture subtree for the native platform. Temp residues count there. Do not assume EXP_PACK_TEST_ROOT alone captures all 14 tests. No engine build/unit invocation was performed for this dossier.

## Separate actual 64 MiB constructor obligation remains missing

The existing constructor test sets limit=96 KiB, serialized output=48 KiB, borrowed=8 KiB. Its final 60 MiB + 16,191,779 > MAX_CACHE assertion is arithmetic only. The heavy cache test uses the real 64 MiB read-cache ceiling, but does not allocate constructor output through the production constructor path. Neither substitutes for the separate actual 64 MiB construction/cache interaction case.

A prospective minimal fixture could reuse the just-created five heavy legacy indexes read-only, construct the 55,366 maximum-length-key builder in RAM, and exercise real make_room/reserve_constructor/finish_reserved under MAX_CACHE. No additional persistent store or deep B clone is intrinsically required for this proposed shape. **It is not implemented or reviewed in current a3**, so its extra persistent bytes/files and peak RAM remain UNPROVEN, not zero.

Its proof must include builder Entry capacity, owned key/hash capacities, serialized index output capacity, borrowed legacy-index bytes/offsets, cache entries and construction auxiliary bytes together, positive eviction before allocating output, a genuine refused-over-cap variant, and exact native process/RSS evidence. The source nominal Entry and key arithmetic is only a planning aid; native Rust allocator capacities and platform sizes still need the implementation's explicit bound. Only the sole engine owner can propose the necessary source change; this dossier changes no private API.

## Whole focused/native topology and 512 MiB decision

Frozen diagnostic arithmetic is 529,480,133 bytes / 58,096 files: donor 261,739,670 / 15,596, conditional B 149,251,266, other native root proposals 68,157,440, coexistence 33,554,432, metadata 16,777,216, existing fake selector 109. Under 512 MiB=536,870,912, that leaves only 7,390,779 bytes / 1,904 files. It is **not a complete upper**: many roots still use proposals; F1/F2 caps are unproved; B excludes physical metadata; constructor and original boundary fixtures are unresolved. Do not relabel this arithmetic as fit.

If donor's 80,976,318-byte heavy test is the same exact one-run fixture, only its already owned portion can be reused in accounting. Adding the other 14-test fixture portions costs another 2,354,064 bytes / 54 files relative to that portion, leaving 5,036,715 bytes before all unresolved obligations. This is conditional bookkeeping, not permission to execute or a reason to omit the missing constructor.

Required roots remain primary A and the one deep B lineage; interval, reservation, cutoff, F1, F2, stable base and native pressure A roots; and explicitly bound shallow donor extensions. Existing 1/4/1 cycles, sixteen accepted per cycle, four readers and all 48 losing preparations are preserved. Windows 1,000-activation stress is platform-specific and remains separately mandatory for Windows. No accepted B generation is retired to finance the plan.

An independent functional block also prevents frozen v2 retain-all progress: lifetime.rs MAX_RESERVATIONS=8. Each four-publisher same-base round leaves three losing sealed reservations. Two rounds leave six; the next four candidates cannot coexist. recover_transfers releases only the exact accepted attempt, and abandon fences without freeing a slot. Old abandon+collect must not be enabled: a source-bound terminal contract is under independent review. A new reviewed contract and revised bounded harness may resolve this, but neither budget growth nor this dossier does.

Also frozen concurrent_cycle(publishers=1) publishes synchronously before starting maintenance, so its current source cannot prove publisher/maintenance overlap. The required minimal v3 source plan has been sent to meta: real physical-before-selection maintenance ready, positively started and candidate-ready publisher before releasing maintenance, transport-only on_started, all native exits and callback joins observed. No v3 edits were made under the frozen v2 receipt.

All unresolved sharing/death/path-reuse/corruption/paged/raw/deferred/segment/backup/compatibility obligations stay required. Original raw 8 MiB, pack 64 MiB, index 16 MiB, 65,536 records, 64 packs, Work 128 KiB, deferred 128/129 and 32,768/32,769, relevant-segment consolidation and collateral 64 MiB limits need exact per-constructor peaks and residue owners. A unit touching a boundary does not automatically fulfill a separate native protocol case. Crash deletion constructors still need exact a3 capability binding, at most two setup builds, at most eight continuations, real pre-existing owned targets and actual removal proofs. Observed exit86 alone is not PASS.

Source freeze, functional cutoff, protected closeout, the unchanged 44 GiB cumulative ceiling, inside reserve and raw evidence retention remain controlling. Admission requires an exact future source/schema/adapter/guard/binary binding, actual donor expiry/closure plus measured global census, tight peak next-command bytes/files/RAM, registered owner for every pending constructor and raw receipt, and explicit meta runtime GO. None is supplied by increasing the proposed scalar to 512 MiB.


## Revision 2: tighter whole-case sufficient caps (supersedes the null tight bounds above)

**Proposed logical caps from this derivation: F1 25,703,478 bytes / 3,633 files; F2 48,988,533 bytes / 6,531 files.** These are sufficient upper envelopes under the exact default schedules, not mathematical minimum storage requirements. Convenient rounded byte caps are 25 MiB / 47 MiB. No actual removal credit is used: every immutable file ever produced is counted even if later absent. Raw evidence and native allocated-space overhead remain separate owners.

The earlier gross write-quota ceilings are retained as historical reasoning only. They are replaced for logical case forecasting by the following source-specific calculation. Parent authorized only immutable git show a3f9805eccc19ede8dd5c735b54e675e98e63714:<path> after the checkout changed; pack.rs and packing.rs were read at that immutable revision. All earlier claims about 14 units refer to a3; later source adds units and requires its own full manifest. The mutable checkout is not represented as unchanged a3.

| Retained component | F1 bytes | F2 bytes |
|---|---:|---:|
| N10 initialization, including originals/derived/view/bootstrap objects and initial metadata/stage allowance | 524,288 | 524,288 |
| Later mutable controls, stage and live install-alias reserve | 131,072 | 131,072 |
| Every membership/outcome COW intermediate | 13,533,086 | 30,558,000 |
| All later loose candidate/publication/result/hint/allocation records | 1,341,440 | 2,816,000 |
| Every retirement selector | 52,924 | 0 |
| All successful and declined pack bodies | 800,525 | 4,017,010 |
| All associated paged indexes | 135,336 | 715,360 |
| All immutable Work plan birth opportunities | 9,152,590 | 10,219,998 |
| Every catalog | 26,073 | 4,757 |
| Every durable physical pin | 3,072 | 1,024 |
| Every activation STORE stage | 3,072 | 1,024 |
| **Total** | **25,703,478** | **48,988,533** |

Serialization method is an in-memory Python json.dumps(indent=2) calculation of ASCII upper templates matching serde_json pretty formatting. Templates intentionally contain mutually exclusive states, repeated placeholder identities and maximal counters; they are length supersets, not engine fixtures. IDs are 99 ASCII bytes, hashes 64, request 21, sequence at most three digits. Graph uses project, budget/cost at most1000, count10, hash roots. Optional changes roots are populated to overbound null. No-op candidate replay_objects is empty and replay_files has the two 75-byte derived paths with 64-byte digests. Resulting exact template lengths candidate1750/publication1644/result305/hint202/allocation202 sum4103. Per-role caps2048/2048/512/256/256 sum5120 provide further slack. Initial candidate gets8192. All accepted outcomes, including genesis, retain their input payload; no compact fixture profile is substituted.

Initialization allowance is justified by the fixed N10 payload126,976, derived2315, A view129,291, at most28 graph pairs with compact Header<=1024 and key<=225 (graph leaf<=35,154),14 projection FileSpecs<=512/key<=225 (leaf<=10,416), initial membership/outcome<=2000, initial record group<=11,264, initial refresh plan<=24,576, controls<=32,768, and one maximum65,536-byte live payload-install alias. Sum440,296 <524,288. The fixed baseline Header and FileSpec shapes are far below their scalar caps; none needs >4096-byte object indirection. The initial file allowance80 includes originals10, derived4, view14, four index roots, five accepted records, root controls/locks and owned stages.

Key maxima by role are publication117/result77/hint118/candidate123/allocation225. Applying the paged geometry formula above gives indexes135,336/715,360 for all cohorts. F1 cohort record counts are (131,131,131,1,1), then (2,2,2,2,2), then (130,130,130,0,0). F2 owned cohorts have all five roles at H31,H151,H551. H1 refuses before ownership. Each role fits one pack/index: largest raw role is under2MiB, largest index under16MiB; no hidden split or extra segment. F1 has three segments below default limit4, so no full-base consolidation and no retired-pack descriptors.

Compressed-body bound is the exact locked zstd1.5.7 formula C(n)=n+(n>>8)+((131072-n)>>11) for n<131072. It is read from local zstd-sys2.1.2+zstd.1.5.7/zstd/lib/zstd.h:249 (SHA256 9b4bc8245565c98ccfc61c07749928b57e7c0f6fddb0530c4f6aa1971893d88b); Cargo.lock binds this version. Compressed caps are publication2119/result577/hint320/candidate2119/allocation320, total5455. F1 body =261*(2119+577+320)+(2119+320)+2*5455=800,525. F2 =733*5455+3*(C(8192)-C(2048))=4,017,010, explicitly retaining both declined cohorts and initial candidate supplements.

**Work geometry, not the 128KiB cap:** maximal descriptor root lengths by the five roles are816/731/811/826/1031, using pack/index byte fields at their allowed maxima, records551, instance99, segment3 and paged root(height1,bytes16384). Work uses maximal u64 counters, transition hash and complete+in-progress discovery simultaneously; deferred remains empty in these serial accepted schedules. A 84-byte physical-plans/<hash>.json parent name adds exactly92 pretty JSON bytes to an already nonempty garbage array.

F1 first generation has5 descriptors and5 jobs, no older generation. Its template including one activation-stage garbage name is11,910 bytes. Initial birth plus320 bootstrap save opportunities produce321 retained plans: sum[11910+92*k,k=0..320]. Later maximal geometry has13 current descriptors, older5+10 descriptors, conservatively all13 jobs (no credit for drained jobs), zero retired descriptors and12 non-parent garbage strings of184 bytes, giving base45,277. Two later build births plus six final save opportunities yield at most8 plans; each is charged all329 possible parent references, even if not actually reachable in its garbage array. Thus F1 plan total =321*11910+92*51360+8*(45277+92*329)=9,152,590.

F2 selected generation has5 descriptors/5 jobs/no older/retired. Both aborted owners decline before catalog/Work creation. Each contributes one directory, ten pack/index names, three progress names, pin and STORE stage (16 garbage names); current activation stage adds one, total33. Exact base including all33 is17,118. Initial successful Work plus320 saves yields sum[17118+92*k,k=0..320]=10,219,998. Aborted cohorts' journals remain in the bounded current PACKING owner array, not an invented separate Work per decline. There are no terminal loser receipts in F1/F2, since all offered publishers are serial accepted.

Every save can leave a fixed work-stage instead of its immutable plan; its payload is counted as that birth. Unix install_original can momentarily leave both hard-link names; the single largest live duplicate plus current JSON controls is covered by131,072. Case execution is serial and unhandled integrity/transport failures stop it. Cooperative cleanup deferral can continue only within the frozen160/2 call ceilings. Fixed stage names, all stale immutable outputs, and failed prefixes remain counted; no absent old plan is assumed.

File bound F1=80 initial +262*5 later records +1572 COW nodes +262 retired +26 pack/index +329 plans +3 catalogs +3 pins +32 later controls/locks +16 possible stage/alias names =3633. F2=80+550*5+3300+30+321+1+1+32+16=6531. This supersedes proposed6000 F2 files. Root metadata/lock creation and temp aliases are explicit; raw receipt files are outside these roots.

Replacing only F1/F2 in the old provisional topology gives **553,840,496 bytes /58,260 files**, above512MiB by **16,969,584**. Adding the other-unit delta gives556,194,560 (19,323,648 above512MiB), before terminal v2 receipts and unresolved native constructor/adapters. This revised aggregate is still partly provisional and is not a closed final-suite upper or a claim of unavoidable consumption.

Terminal v2 is a separate future binding: actual small four-publisher cases have48 losing attempts per A/B root, not4096. Charge48 receipt/fence pairs per root plus v2 registry and stage geometry when the exact private contract is pinned; do not charge4096 files merely because that API count cap exists. Global large-history maximum2400 distinct failures across two roots must remain separately forecast. No legacy abandon+collect is activated.

### Per-platform scenarios: still incomplete

Native revised known subtotal plus the complete old-a3 unit delta is556,194,560 bytes/58,314 files. Candidate560MiB leaves31,008,000 bytes;576MiB leaves47,785,216, before terminal/fences and unresolved constructors or other-root proof gaps.

Windows must additionally retain parent-reported77,395,734 bytes/12,078 files of old CP1/2/3 fixtures unless exact legal reuse is proved. Known subtotal becomes633,590,294/70,392, before1000-activation stress replacement, terminal and unknowns. Windows640MiB leaves37,498,346 bytes;60,000 files is already below this known retained topology. Replacing native2MiB pressure with the unproved Windows32MiB proposal consumes30MiB, leaving6,041,066 bytes. Receipt+registry/stage allowance adds1,048,576 for two roots (96 receipts<=8KiB each plus64KiB registry and64KiB stage per root); fence bytes remain separately unbound. Receipt/fence pairs plus four registry/stage names would bring known files to70,588 before stress delta.

Windows H1001/request<=14 has a refined COW-only upper52,886,466 (membership14,352,979/outcomes38,533,487): for H2..64 sum7+H*pair; thereafter sum65*pair+14+floor(H/32)*branch+8. Pair sizes171/572, branch235/164; OutcomeRef502. It excludes every other stress component and is not a lower bound. No512/560/576/640MiB runtime GO, planned expiry credit or unit/fixture drop follows. A separate complete Windows stress peak and file cap remain mandatory.

## Source references

Line references are one-based in the bound a3 checkout: store.rs 202-255, 364-381, 409-428; disk.rs 436-443 and 592-594; objects.rs 15-26 and 56-63; projection.rs 117-145 and 368-384; lifetime.rs 6-49, 53-69 and 100-140; packing.rs 167-169, 214-239, 568-655 and 710-803; physical_lifetime.rs 9-10, 139-181 and 247-290; pack.rs 5-10 and 89-99; paged.rs fixture tests. Harness schedule references are in frozen focused_schedules.py 138-241. All references were read from files, without Git probes, engine runs, Cargo, native tests or index bootstrap.

