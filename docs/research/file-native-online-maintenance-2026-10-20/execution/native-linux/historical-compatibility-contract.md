# Linux historical executable compatibility: actual contract and closed-grant plan

Status: read-only audit/proposal. No driver/guard/dispatcher edit, executable call,
build, corpus write or new control probe ran. Stable unified-core/API handoff and
explicit source/resource release remain required. Actual native fixture/target
caps stay96MiB/256MiB. The newer480MiB/60k fixture envelope is feasibility only.

## What the accepted Linux code actually enforces

`native_driver.py` hash remains
`dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce`.
Its constructor requires a live ancestor guard claim whose source pin equals the
caller-supplied `suite_pin`, then checks executable bytes against caller-supplied
`expected_sha256`. `binary_source_pin` and `role` are recorded, but the constructor
does not verify an action/case/producer allowlist. Omitting `binary_source_pin`
defaults it to the suite pin. That is adequate only under the already-reviewed
candidate build/manifest chain; it must not be used to relabel a historical SHA
as a final candidate merely by supplying final-pin strings.

The candidate dispatcher hash remains
`3bafbc8ac45de6ea2e4060d1a04c6fa5fc1ac278080e3ef1ffcececbd52481b7`.
It explicitly requires `executable.source_pin == suite_pin`, verifies its SHA and
owned target/source path, and exposes one candidate executable. Its legacy bridge
rejects a differing `exe_override`. Therefore **historical compatibility is not
currently a runnable closed path through that dispatcher**. A direct separately
constructed Driver could record a historical source pin, but lacks the requested
closed grant and must not be treated as already authorized compatibility plumbing.

The outer guard's source pin binds the overall workload/suite claim, not proof that
every executable under it was compiled from that pin. The final build binding and
per-executable source proof are distinct. Every historical event must clearly
separate `suite/qualification_pin=FINAL` from `binary_source_pin=HISTORICAL` and
role. Prefer unambiguous field names, or explicitly record the legacy meaning of
`source_pin`; consumers must never infer candidate qualification from the suite
label alone.

## Existing historical artifact, observed without execution

The retained native Linux baseline exists at
`/home/spenser/file-native-p-linux-20261020/bin/candidate-linux-final`:
3,349,520 bytes, SHA-256
`a8501737ca4fdd5cfc177620860c1ae103d56f2e9b2151104a922df511a3d84b`.
Its existing reviewed source/core binding is
`d70f3864f2888d46d9112213b4a03f8d82745a8e`, integrated evidence
`a0b92fe5dc64269508601dcddf7e2e2ca477e30d`. The preserved Linux qualification map
records final-source145/core18 equality and source authority manifest SHA-256
`0c636670f89bd11086b139b966452d4f852e537d06997aa1f6649af5ad01d29a`.
This is a historical producer/read/refusal control candidate, never the final
candidate binary. The original root/binary remain read only; all fixture roots
would be new admitted owned roots.

The preserved provisional62ce executable is a different artifact/source and has
its recorded compilation cadence gap. It must not silently become a qualified
Work2/catalog2 producer. Supported STORE3/catalog2/catalog3 fixtures or clients
need their own explicit source/image provenance and grants. A cross-platform
quiescent old image may be usable if its exact producer pin/bytes/transport and
native restore proof are reviewed; that does not invent a missing native producer.

## Recommended closed grant, without weakening candidate checks

Keep the candidate manifest equality and unbound-override refusal. Add a separately
reviewed **compatibility companion grant** to the source-bound workload capsule,
with exact bytes/hash anchored by the existing `source_files` manifest. The core's
versioned/native extension calls an adapter-owned historical factory, never raw
Popen or an arbitrary `exe_override`. Exact API names await the core owner.

Each grant is a closed record with:

- Final suite pin, immutable grant ID, approved source/build/producer receipt hash.
- Exact historical executable canonical path, SHA, actual historical source pin
  and fixed role (`historical-producer`, `historical-held-reader`, or
  `historical-refusal-control`). The factory supplies these; callers cannot
  override/default them to final.
- Exact allowed case IDs, phases, action plus relevant sub-operation, bounded
  invocation count, expected exit/refusal class and declared crash checkpoints.
  Proposed case families are `Compat-STORE1-STORE2-to-new` and
  `Compat-STORE3-to-final`, expanded to the final source-bound subcase IDs.
- Only owned new fixture/control namespaces. A read-only historical executable
  path may be outside the runtime; its **input root must never be an old protected
  operational root**. No old archive/target/source mutation is authorized.
- Exact permitted environment/checkpoints/control paths, independent reader256MiB
  or checker2GiB role, suite heavy claim, deadlines and positive process identity.

The historical Driver factory validates the active suite/case/phase grant, actual
file SHA and retained source receipt before creation and before each call. Native
opaque process handles inherit that exact binary/role identity. A request for an
unlisted action/root/phase or historical executable fails before child launch.
The shared core may use existing engine actions only; the grant does not invent
a new engine command, semantic admission rule or general plugin authorization API.

Retain raw historical events with explicit producer/action/case/grant, actual
binary SHA/source pin, suite pin, PID/start/role and selected state before/after.
The final-pin candidate then independently reads/restores/verifies the result and
accepts the required successor. Historical producer/refusal success alone never
passes the final native case or qualifies final-source code.

## Phase examples to bind when the exact core extension arrives

| Phase | Historical scope | Final-candidate obligation |
| --- | --- | --- |
| Legacy STORE1/2 construction | Only approved init/prepare/publish/packing sub-operations on a new shallow owned store, with genuine accepted outcomes and exact inputs | Verify old image using the reviewed legacy route and final consumer; no forged semantic root or assumed old image |
| Held old operation during initial conversion | Approved reader/barrier on exact historical image; source-bound callback/opaque handle controls; reader256MiB | Conversion respects the legacy live operation; after valid release conversion and new operations succeed |
| Prepared old candidate | Exact approved prepare before conversion; saved candidate remains persistently owned | Old-client post-conversion attempt refuses safely; final candidate's supported preparation/ownership semantics separately verified |
| Old open/refusal after new activation | Only declared reader/select attempts with exact expected refusal and no root mutation | Independent final-source before/after inventory/outcome checks and normal final operation |
| Old image restore and successor | Historical producer/source image or explicitly supplied reviewed image only | Native quiescent restore, full independent oracle and accepted final-candidate successor |

Allowed actions are frozen **per phase**, not the broad union from this table.
In particular no generic old `publish` permission survives activation merely
because old-client refusal is expected. A nonmatching error, mutation, unknown
exit or monitor failure remains failure; there is no general exception swallowing.

## Separate historical guard phases: useful but incomplete alternative

An old-source guard phase can construct an old image and close completely before
a final-source phase consumes it. A later separately bound old refusal phase can
be followed by a final-source integrity check. This avoids lying about binary
source pins and respects one heavy session at a time.

It **cannot by itself prove a held old reader across final activation**: closing
the old group ends its pin, while two simultaneous heavy guards violate current
host ownership. That mixed-lifetime case needs both explicitly granted binaries
inside the one suite-owned guard, with distinct actual binary/source roles. Thus
the closed companion grant is the recommended complete solution.

## Relation to the unified source handoff

The inspected original README describes an injected Driver plus adapter
`interrupt(handle,kind)`/`finish_interrupted` hooks and source-bound `extend`.
Reader-end starts currently use normal expected exit0, so adapter control handling
must recognize only the exact intentional termination/identity from its registered
call; it cannot treat every AssertionError or thread error as an expected death.
The source7/remediation interface is not final API permission. Candidate-call
binding remains final-pin strict; historical dispatch is an explicit extension.

Implementation and new targeted controls wait for that stable API and review.
No parser fork, payload-profile change, old-data deletion or workload release is
implied by this plan.
