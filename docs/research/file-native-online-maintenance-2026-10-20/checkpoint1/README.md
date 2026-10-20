# Checkpoint 1 source review boundary

This is a reviewable implementation checkpoint, not completion of checkpoint 1 qualification or of the three-checkpoint experiment. The unchanged reference core is `d70f3864f2888d46d9112213b4a03f8d82745a8e`; the source checkout starts from `47d667f26a75fa1454dee2d1c50d238aec4cf6bd`. Existing authored bytes, semantic selection/COW/conflicts and the legacy whole-index `FNPIDX01` reader remain controlling.

The structured-format standard was read and remains proposed. These are private engine-managed JSON controls and an additive container-instance descriptor, not a source-format migration or public contract freeze. New STORE version 3 makes old binaries refuse. New code reads legacy STORE 1/2 and original whole-index containers.

## Locks and lifetime

| Path | Order and protection |
| --- | --- |
| Ordinary capture | Q shared; read STORE <=4KiB; acquire existing unique generation pin shared and validate <=1KiB identity; release Q before catalog/index/semantic I/O. Captured Metrics owns the pin through all synchronous I/O. |
| Legacy capture/conversion | Legacy operations retain pack-readers.lock shared. Conversion under Q tries legacy exclusive and W, dropping Q before any retry. Only this initial conversion drains old operations. |
| Build | M exclusive, P shared; complete selected-history eligibility, construction and raw/index correspondence outside Q/W. Still-reserved attempts remain excluded. Transfer recovery stays with semantic writers. |
| Activation | Prewritten STORE stage and immutable work plan; Q exclusive, try W once, exact bounded physical CAS and native replace, then drop W/Q. No journal/barrier/catalog/index work or deletion inside exclusion. Semantic head advancement does not itself invalidate CAS. |
| Cleanup | M only; nonblocking old-generation pins; <=1000 candidates/50ms cooperative work, with indivisible native overruns reported. No Q/W during deletion. |

Every retained old generation blocks dependent loose deletion, including previously unmapped fallback and the physical-before-semantic gap. Shared old containers and unfinished replacement jobs remain reachable. New containers use never-reused `packs/<birth-id>/<hash>.pack|idx` paths, including identical bytes.

## Selection and recovery

STORE selects physical `{id, ordinal, plan}` alongside its catalog. Immutable hashed plans hold bounded generation/descriptors/cursors; ordinary open never reads them. Mutable progress cannot change selected identity or introduce containers/generations. Progress omission of an immutable-plan generation requires a fresh native exclusive-pin drain proof. Required missing pins refuse. A drained pin remains until an authenticated successor plan omits it; bounded protected metadata is reported separately. Mutable garbage is restricted to immutable-plan garbage plus exact old-generation metadata derivations, with selected/retained-instance checks.

PACKING status is a hint. Death after selection/before journal advancement does not roll back or remove selected output. Birth ownership is persisted before creation; aborted unselected output remains charged until inherited cleanup actually removes it. A missing item on replay earns no second deletion credit. Windows sharing defers and earns zero credit. Successful logical removal is separate from native allocated-space credit (not inferred).

Quiescent backups must include physical-pins/*.lock **with their durable identity bytes**, all selected plans/progress and pending ownership. Exclude only the global ephemeral gates. Restore must not recreate a missing selected pin on ordinary open.

## Evidence and remaining work

`source-binding.json` binds exact Rust/Cargo/consumer bytes and the review executable. Raw guard, engine events, source manifests, RSS samples and preserved failures are under `C:/file-native-online-cp1-20261020/evidence`. `checkpoint-status.json` distinguishes passes and pending coverage. Four small workspace tests pass; the existing ~81MB cache fixture is explicitly deferred to the final pin and can be routed with EXP_PACK_TEST_ROOT.

The preserved first smoke failed its final harness assertion for an obsolete collection.yaml path. Its 44 engine calls/six audits passed; the corrected old B manifest/20 exact paths check passed separately. A 16.110s held-reader/two-successor proof and progress-corruption refusals passed on candidate2. Subsequent source binding records the narrow removal of maintenance-side transfer recovery under W.

CP1 retains historical full-cohort enumeration outside Q/W and frozen discovery. It makes no incremental-discovery or large-history bookkeeping claim. Full newly-unmapped-publication capture-gap plus later tail compaction remains explicitly pending CP2. Full A/B N64/H64 independent oracles, all death/cancel/stale-CAS/sharing/protected-owner schedules, 1000 activation stress and final same-pin native qualification remain required. A few successful activations do not establish p99.

GitNexus exact-worktree impact/detect attempts are UNKNOWN (unregistered); direct cached CLI only, no bootstrap. Manual blast radius is HIGH across ordinary APIs/consumer binaries, nested fresh Metrics under W, physical capture, packing/recovery and format consumers. Raw per-symbol attempts and manual caller trace are preserved. Main, prior integration and meta landing source were not edited.
