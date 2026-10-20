# Early replacement hazards — uncommitted code only

Observed 2026-10-20 around 20:44 UTC. This is not acceptance of a replacement pin or build. The committed 62ce963/candidate3 report remains unchanged.

Inspected live `physical_lifetime.rs`, SHA-256 `cf178ac242fbb14f1827e093dd35498cc779dbdb701467337b9b6f37e7ac183c`, and live `packing.rs`, SHA-256 `47cae0c0d3d472e014324725cdc74edb8c0d010ad49ea9635d89eb01f5c995d8`, under the CP1 worktree. Source can change after this observation. No engine calls, builds, fixture mutations, or source edits by this reviewer.

## P2 Required: selector-stage text authorizes unrelated plan deletion

`physical_lifetime.rs:156-171`: `recover_stage` parses an untrusted deterministic selector-stage as StoreConfig and compares store id, catalog, generation id and ordinal. It then uses the stage's `physical.plan` as a pathname to delete, excluding only the current and retained older plan hashes. It neither reads/authenticates the candidate Work's bytes/generation nor establishes that the named plan is owned by this transition. An aborted/build owner can have a valid unselected plan hash outside those exclusions.

Concrete proposed test: retain another aborted owner's existing plan, copy current STORE to `<current-id>.selector-stage`, replace only physical.plan with that owner's hash, invoke maintenance status/cleanup, and require integrity refusal with every owner file unchanged. Current source reaches `remove_checked` on that plan. Invalid JSON follows a different path and does not cover this case. Valid-JSON stage metadata must not create deletion authority. A candidate-content identity check is necessary; a defined narrow transition/ownership rule is also required so an unrelated same-generation owned root cannot be erased.

State: OPEN, confirmed statically and sent to meta before commit.

## Open concern: selected root never records the reported empty metadata state

`physical_lifetime.rs:183-210` adds the previous selected plan pathname to Work.garbage, serializes/selects the new immutable root, then removes that path and credits the deletion only in the in-memory Work. On disk the selected Work still names the now-absent predecessor. `packing.rs:431,463` unconditionally saves twice, even for an otherwise idle cleanup. Each later cleanup can therefore create two new metadata roots/CAS operations and delete two predecessor roots, then report complete from the in-memory value although the selected bytes still have a pending predecessor entry.

This trace does not itself prove unbounded physical plan-file growth: successful direct predecessor deletion can keep the actual file count bounded. It does prove there is no stable no-op path in the observed code and that response versus selected Work differ. Require repeated quiescent cleanup/status plus plan-file census and crash/restart between selection and deletion. Establish finite/no-op reconciliation semantics and honest residual metadata accounting. Reassess cleanup's cooperative 50 ms budget, including unconditional serialization, hashing, persistence and metadata CAS after the candidate loop ends.

The new `physical_work_holds_ns` field is separate from actual `physical_activation_holds_ns`, as required. Same-generation metadata updates preserve id/ordinal/catalog and the pin's identity because PinIdentity does not include plan. Flat abandoned-owner handover, checked ordinal birth ids and selected-stage directory cleanup require the replacement-pin review and deterministic receipts before being classified fixed.
