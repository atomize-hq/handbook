# Bounded no-op loser release proposal

**Proposal for the coordinator and Rust source owner. Not implemented, approved,
or an execution grant.** The mixed/no-op tail decision remains pending. This
proposal only applies if the no-op method in
[history-tail-method-options.md](history-tail-method-options.md) is explicitly
approved. It does not establish a shortcut for mixed-operation or B candidates.

The source basis is exact provisional CP3
`f9b762a1a0d352564542bb4850ad77ee14a7c6e4`; every condition and affected caller
requires final-pin review. The current source has no bounded abandoned-reservation
release API: abandonment keeps the entry charged, and semantic collection walks
accepted membership/content. The existing collector remains the recovery path.

## Proposed operation

The source owner could consider one explicit private lifecycle operation that
handles **one caller-owned, definitively stale, sealed no-op attempt**. It would
delete only that attempt's known private candidate, and release its reservation
only after actual deletion and required persistence succeed. It would not scan
history, delete graph/projection/original/derived objects, retire an accepted
snapshot, invent a quota transfer, or infer ownership from a path prefix alone.
The exact API name and closed request schema belong to source-owner review.

Admission/proof requirements, all independently rechecked by the engine:

1. Hold the semantic writer lock and obtain exclusive preparation access with
   the existing nonblocking gate. A busy gate must refuse/defer without mutation.
   The caller must let its completed failed process exit before requesting release.
2. Validate the exact safe attempt ID, existing reservation, sealed candidate
   digest, store binding, normalized request intent and expected base. Restrict
   mode A, profile4, `ops=[]`, `packet=false`, no checkpoint, packet or B generation,
   no changed/removed documents and no replay object IDs. Prove both replay-file
   entries are the exact existing shared packet-not-requested derived files;
   retain those shared files.
3. The candidate's expected selector must be an authenticated accepted base and
   must differ from the current selected publication. Verify through the existing
   bounded membership/outcome lookup that this attempt is not accepted. An equal
   expected/current selector is a still-valid preparation and must refuse. Lost
   replies or uncertain acceptance cannot be treated as definitive conflicts.
4. Require graph/projection/changes identities to equal the authenticated base,
   and require the complete private attempt directory to contain exactly its
   candidate file. Refuse any allocation record, hint, result/publication side
   effect, generation, unknown child, reparse point, unexpected byte identity or
   damaged/missing ownership evidence. This avoids treating later-boundary deaths
   as the earlier `conflict:expected-publication` case.
5. Install the existing durable abandonment fence and keep the reservation charged.
   Remove the exact authenticated candidate with the existing checked removal
   primitive, including its required synchronization. Only then use the existing
   reservation transfer/release primitive. Return explicit observed removed bytes,
   files and resulting reservation count; caller-side allocation credit follows
   the actual release census, never the planned command.

The work is bounded by one small candidate, one base publication and point-index
proofs, one registry of at most eight entries, one fence and one private file
removal. This is not a proof that native fsync/lock maxima are below a latency
gate: actual waits/holds and errors must be retained and checked. The operation
must not run a hidden whole-history collector as fallback inside the timed call.
The previously derived 1,784-byte candidate bound is conditional on all stated
field-width/preparation preconditions; it is not a general candidate size limit.

## Crash and uncertainty behavior

The fence precedes removal; reservation release follows successful removal.
Failure before release retains the charge. If a crash leaves a fence and a
missing candidate but the registry entry remains, this bounded path must refuse
unless the source owner adds and reviews sufficient durable deletion proof.
The existing full semantic collector can then reconcile the abandoned owner in
an explicit recovery cell outside the latency workload. No proof format is added
by this proposal, and unknown missing candidate bytes are not permission to release.
If release completed but reply was lost, re-reading quota and the terminal marker
can establish that there is no remaining entry; it must not create another credit.

Required small controls before using a new operation include selected/winning
attempt refusal, valid expected-base refusal, wrong intent/digest/store refusal,
unknown extra child refusal, packet/mixed/B refusal, held-preparation deferral,
deletion/sharing refusal, death after fence, death after removal and death around
registry release. Full independent accepted outcomes/current sources/navigation
must remain exact. Any source change requires impact analysis and a fresh checkpoint.

## Offered-load method and accounting

Four publisher frontends must still actually overlap; no implementation may
replace them with one serialized publication loop. The method must predeclare
how offers stop at400/100 successes and how quota backpressure is handled. An
explicit drain of already in-flight callers before a narrow release operation
may be reviewable, but it changes the workload schedule and must be identical
between no-maintenance controls and online cycles. It must not be introduced
after seeing timings or described as continuous four-caller execution.

Retain every offered call, semantic conflict, successful publication, cleanup
call, busy refusal, owner proof, actual release census, process overlap interval,
engine clock and logical-slot end-to-end time. End-to-end/phase throughput includes
client backpressure and cleanup pauses; those pauses cannot disappear by reporting
only successful engine times. If the coordinator requires uninterrupted offered
load instead, this drain schedule is not an equivalent substitute.

Even if approved, admit every possible failed candidate/fence and temporary
before execution. The 2,400-loser footprint cannot be reduced by a promise that
this operation will collect them. Measured later removal may update subsequent
admission only after the resource owner accepts actual proof. Empty directories
and terminal markers remain charged; native directory/device allocation is not
inferred from logical candidate bytes.

This is the narrow source-owner option. With the engine unchanged, explicit
whole semantic collection is the available release method and requires a separate
method/accounting decision; abandonment alone supplies no eight-slot capacity
guarantee. The current experiment and all large-tail execution holds remain intact.
