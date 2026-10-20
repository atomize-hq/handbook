# Native driver and capsule dispatch preparation

Status: control checks passed; parent source review and replacement engine/schema
pin pending. No old engine pin was rerun. The only real engine work so far remains
the preserved provisional `62ce9630` compile with its cadence limitation.

`native_driver.py` SHA-256:
`dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce`.
`native_capsule_adapter.py` SHA-256:
`3bafbc8ac45de6ea2e4060d1a04c6fa5fc1ac278080e3ef1ffcececbd52481b7`.
Both require accepted guard
`4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9`.

The driver preserves `Driver(executable, output)`, `call`, `head` and `publish`
behavior with explicit binary SHA/suite pin parameters, append-only raw events,
executable roles and source bindings. A blocked launch has positive RSS before
execution. Reader actions enforce 256 MiB and checkers 2 GiB independently of the
executable basename; reported process peaks are also checked. The per-call watcher
uses identity-bound pidfds, joins before resource closure and records root reaping.
The outer accepted guard remains authoritative for all descendants, aggregate
memory/deadlines/category claims and positive process-group closure.

The legacy bridge deliberately exposes `Driver(base)`, `EXE`, `events`, packet
publish and head operations without copying a decoder. Event files live under
owned `evidence/legacy-events`, not inside an engine store. Unbound executable
overrides refuse. A separately bound compatibility-role driver will be required
for old-client cases; the candidate capsule dispatcher requires binary source pin
equal to suite pin. This prevents an old decoder/binary being labeled replacement
qualification.

The capsule dispatcher requires an explicit machine-managed manifest with exact
source file hashes, candidate binary/source pin, guard hash, owned script/argument
paths, driver interface selection and case-map receipt. It verifies source and
binary before/after execution and retains failed dispatch receipts. It does not
stub Windows controls, replace an independent oracle, reinterpret JSON identity,
or infer a semantic pass from a case-map declaration. A successful dispatch only
means the selected bound script completed its assertions; final native case
coverage must still be checked against the source-bound shared capsule's actual
case results and all required legacy IDs.

`adapter-controls/` contains two guarded control runs using an explicitly fake
JSON protocol executable and fake capsule, never an engine binary:

- Driver: six interface checks/five fake peer events, including expected error,
  timeout refusal, legacy events, head/packet publish and unbound override refusal.
  Outer elapsed 0.9922 seconds; max RSS sampling interval 20.5327 ms, none >40 ms.
- Capsule dispatch: source/binary/manifest/legacy-interface binding, elapsed
  0.5071 seconds; max interval 20.0730 ms, none >40 ms. No semantic or Windows
  control substitutions. Both guards have positive closure and joined monitors.

The only fixture bytes are fake control selector JSON; no engine corpus exists.
The before-receipt census is `adapter-controls/native-adapter-preparation-census.json`.
No new supervisor ran outside registration during either heavy session. All helper
SSH reads/transfers and census calls were serialized outside those sessions.

Before actual CP1 use, require the replacement source pin and its updated Work2 /
transition-parent physical decoder from the matrix owner. The old `62ce9630`
format decoder cannot qualify replacement code. Preserve all 52 applicable packing
IDs and the separate workspace/strict/focused/recovery/ordinary sets; map each
changed expectation to the authorized new contract without silently removing a
case. Large cache unit remains final-pin only. Full same-final-pin native capsules,
small independent oracles after critical crashes, compatibility/restore/successor
and ordinary retained-B selected-once extraction remain pending.
