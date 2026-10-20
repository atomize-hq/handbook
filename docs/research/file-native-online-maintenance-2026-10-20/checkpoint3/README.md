# CP3 provisional authenticated paged-index checkpoint

Private experiment, provisional independent-review boundary. Base is reviewed
CP2 `7a6ee9db03f5cd2e388129a9019c0d5f274a1844`. The structured format standard
remains proposed. No public format/API freeze or source-payload migration.

## Representation and point proofs

STORE3 generation capture remains unchanged. Catalog4 explicitly admits a
closed optional `Descriptor.page={version:2,hash,bytes,height}` discriminator;
absence means the unchanged FNPIDX01 reader. Catalogs1/2/3 remain readable.
Catalog4 may continuously retain old whole-index descriptors during adoption.
Unsupported old clients refuse before Store::open returns. A single fresh birth
contains each new pack/index pair. Split-location/index-only fallback is absent.

FNPIDX02 is one immutable variable-page index file, root at offset0, preorder
contiguous node extents, no padding or per-page files. Every integer is little
endian. Every node begins:

| Field | Encoding |
| --- | --- |
| magic | 8 bytes FNPIDX02 |
| height, role, codec, reserved | four u8; height0..2; reserved0 |
| subtree records, slots | u32,u16; slots1..48 |
| pack start/end | two u64 |
| pack identity | SHA25632 bytes, pack length u64 |
| first,last | each u16 length plus typed ASCII key |

Role IDs: publication0,result1,hint2,candidate3,allocation4. Leaf entries are
key(u16+bytes), pack offset(u64), encoded/raw lengths(two u32), raw SHA25632.
Branch entries are child first key, index offset(u64), length(u32), SHA25632,
subtree records(u32), pack start/end(two u64).

All admitted typed keys are at most225 bytes (allocation with two99-byte IDs);
the old512-byte parser guard is not tightened. New pages recheck the complete
typed grammar. Each page is <=16KiB and has <=48 slots. At most65536 records
require <=3 pages on a route. Exact checked builder sizing includes all roots,
branches and leaves, never fixed padding. Worst225-byte-key entries require an
earlier split:55366 records have conservative upper16777059 bytes;55367 exceed
16MiB. The RAM builder test exercises the complete55366-record output.

All relevant segments (<=4) are probed, including after a hit. Each touched node
authenticates its hash, pack binding, type/order, inherited sibling fences,
subtree counts, pack spans, lengths and complete page body. A leaf gap plus
authenticated sibling fences proves absence. All mapped duplicate bodies are
decoded/hash-checked and must agree; a valid loose duplicate cannot mask damage.
An owned first result is returned without rereading its route or record.

The point-read contract intentionally permits unrelated page corruption to wait
for full audit. New/consolidated candidates validate whole index SHA, every page,
global preorder extent/ordering/coverage, entire pack SHA and every raw record
outside Q/W. Explicit `packing operation=verify` performs a complete engine
physical audit; this is distinct from an independent Python oracle.

## Cache, cleanup and conversion

Physical cache identity includes index birth path and page root/version/length/
height; each page cache additionally uses offset/length/hash. A paged reader
reserves1MiB through the same64MiB cache/transient budget before allocation.
Constructor entries/keys, backing-Vec growth (including old/new transient backing),
serialized index, tree/page scratch, bounded ownership/work metadata and input-key
batch allowance are reserved before allocation through that same ledger. Borrowed
legacy input indexes stay charged until actually dropped. Installed serialized
index and pack buffers are dropped before verifier allocation.
It retains <=12 pages and evicts only pages without outstanding borrows. The
reserve covers page buffers, bounded parsers, descriptor/cache metadata and audit
scratch. Legacy full index/ordinal arrays and borrowed verification indexes share
the budget. `pack_cache_peak_bytes` reports charged upper bounds, not allocator
instrumentation. Page capacity counters measure Vec/string capacity with a
conservative cache-node allowance; Driver and Job receipts retain actual native
RSS. Successful `pack_pages_parsed` and dedicated parse timing/capacity now include
root-open binding, descent and full-audit parses. Failed operations retain their
whole-operation wall time and observed I/O/metadata counters.

Paged-only per-key page-read/index-byte maxima are enforced at12/192KiB. Per-key
index-byte totals now include legacy whole-index reads, with separate paged,
legacy and mixed-route counters. A legacy/mixed route is not a paged-gate pass,
even if its warm cache avoided I/O. Whole-operation totals remain visible.
Capture bytes include STORE/pin/catalog;
catalog4 refuses startup above65536 bytes. Root bytes count in per-key work, not
in the59KiB catalog allowance. Physical metadata counters include explicit file,
pin, catalog descriptor, pathname and cleanup/removal probes; shared I/O metadata
and path canonicalizations have separate counters. Counts represent explicit API
probe attempts, not kernel retries, lock internals, environment cancellation or
barrier polling, or uninstrumented semantic lifecycle control probes.

Cleanup and consolidation use authenticated ordinal seeks with a bounded leaf
cache, preserving CP2's35ms work/50ms entry budget and1000candidate cap. Legacy
whole-index calls remain honestly indivisible. No historical index/audit/delete
is moved under Q/W. Selected Work2 remains capped128KiB, Discovery32KiB/128 retry
entries. Terminal exclusions, automatic refusal/abort, stable-base retention,
transition-parent comparison and CP1 birth/pin rules are preserved.

Explicit `packing operation=convert,force=true` streams the entire captured mapped
representation descriptor-by-descriptor, resetting at each original boundary.
It preserves Discovery verbatim, including omission/None, and creates ordinary
fresh pack/index pairs. The already durable building journal owns the unique
output birth. Equal files can be shared only after successful creation earlier in
the same invocation, followed by full streamed length/SHA verification; an unknown
existing name still fails create_new. A building crash cannot resume installs.
Verified identical output descriptors can be deduplicated across segments;
same pack bytes under different index keys keep distinct indexes. Input record
occurrences, output descriptor-record occurrences and removed duplicates are
reported separately from the logical key/raw map. No hash search or adoption of
retiring/orphan paths is introduced. It neither collects504833 keys nor relaxes routine delta
collateral limits. Complete streaming key/raw-length/raw-SHA input identity is
compared with every fully verified output partition, avoiding a second cached
copy of a large legacy input index. Existing codec/raw bytes and logical keys
remain exact. It is full conversion work, not incremental discovery.

The first upgrade drains legacy operations or all pre-upgrade generation users.
Pin handles and identities are prepared outside Q/W; final native locks are
nonblocking under bounded capture exclusion. Routine catalog4 successors do not
drain old readers. Upgrade waits/holds are separate metrics. Existing clients,
durable owners and all native lock behaviors still require final qualification.

## Evidence and limits

The original f9 guarded build7 passed10 small units. The existing large64MiB cache unit
was explicitly filtered and remains mandatory on the final admitted fixture set.
New units cover positive/negative/page boundaries, malformed/truncated metadata,
untouched-versus-touched corruption, physical-instance cache identity, four
relevant height2 routes (exact12 cold page reads), duplicate conflict, and the
maximum-key full builder split.

Original f9 `cp3-conversion` proof passed actual legacy A/B conversion, exact4outcomes
per mode,20records per mode, Discovery None preservation and full engine physical
audits. `cp3-smoke3` on its earlier bound build3 binary additionally passed held
legacy-reader deferral/resume, actual obsolete-pair deletion, three+16 rolling
cycles per A/B mode,52 exact outcomes per mode, graph/source audits and mapped
root-corruption refusal with exact loose duplicates. These earlier results do
not become same-final-pin evidence after the conversion bookkeeping increment.

Preserved failures: smoke1 falsely required an integrity error prefix from the
correctly refusing old client; smoke2 assumed an emitted zero segment field;
build6 used test unwrap_err requiring an unrelated Debug implementation. Raw
failed prefixes and their corrective reruns remain in the runtime evidence root.

Independent source review, independent Python complete decoder/oracles, initial
generation-protocol upgrade schedules, full malformed cross-page admission,
backup/restore/successor, actual64MiB cache unit,1000physical activations, H1k/H10k,
the complete exact long cohort, performance gates and same-final-pin native
Windows/Linux/macOS/workspace/strict/packing/recovery qualification remain open.
This checkpoint does not complete CP3 or the full experiment.

Runtime receipts/harnesses are only under
`C:/file-native-online-cp1-20261020/evidence/cp3-*`; fixtures only under
`fixtures/cp3`. Same CP2 checkout and same F02 build target are reused. Frozen
guard/token/Job/closure, registered budget9c9c491a..., phase growth48MiB build /
32MiB fixtures /6MiB evidence and all fixed deadlines remain controlling. No
protected prior fixture, archive or binary copy was created/reclaimed. Normal
pair coexistence F14 remains charged pending actual allocator-reviewed expiry.

GitNexus cached exact-checkout impact/detect attempts are UNKNOWN because this
checkout is unregistered. Manual blast radius is HIGH across ordinary physical
reads, packing, activation and cleanup. No indexing/bootstrap was run.

## Independent f9 review remediation

The full independent review of f9 required F1 caller limits before raw allocation,
F2 complete preallocation index-constructor accounting, F3 supported conversion
collisions, and F4 complete physical-domain work counters. All were remediated
without changing page layout, Catalog4/STORE3, Work2/Discovery, semantic authority or
the64MiB limits. `lifetime::remove_checked` changed only additive counters.

Final build9 passed11 selected small units, including raw/Zstd256-byte records
refusing caller limit1 before body reads/decode; exact-bound success; a later
corrupt relevant route; compressed65537-byte hint refusal before its64KiB
existence/read boundary; real reduced96KiB cache eviction before48KiB allocation
with an8KiB borrowed buffer; constructor preallocation refusal; and cold/warm
mixed-format source-work reconciliation. Cold two-descriptor capture has7
physical probes; lookup reaches12, one page read and two parses; warm lookup adds
two body probes, no index bytes and one descent parse. The max-key builder unit
also passes. Build9 intentionally filters the two larger structural file tests
plus the heavy64MiB unit to stay within the4MiB remediation fixture forecast.
Build8 passed13 units before the final hint-probe adjustment; its results are
intermediate evidence, not a final-pin full-suite claim.

`cp3-remediation-red` reproduces F3 against the frozen f9 binary: same-content
separate input births plus same pack bytes/different index keys fail AlreadyExists
while selection stays unchanged. Final `cp3-remediation-green` passes A/B with
12 input descriptors/36 occurrences converting to7 descriptors/21 occurrences,
15 verified duplicate occurrences removed, and the complete21-key raw map exact.
The fixture explicitly adds structural physical aliases to a real accepted store;
it creates no synthetic accepted publications. Three existing accepted outcomes
and a new successor remain exact per mode. Discovery None(A) and retained
Discovery(B) are unchanged. Crash after owned pack write refuses installation
resume; abort/fresh birth succeeds and eventual cleanup actually reaps aborted
and old pairs. Every output index and unique pack is independently damaged in
turn, complete engine audit refuses, bytes are restored and reopening succeeds.
These are engine/fixture proofs, not an independent production full oracle.

The actual near64MiB legacy/mixed cache plus large constructor/native capacity
boundary remains mandatory under final fixture admission. Reduced-cap heap proof
and the exact60MiB+16191779=79106339-byte rejected-old-accounting witness complement
it; they do not qualify that full boundary. Fresh remediation review, final full
units, native, scaled history, long conversion, performance and restore remain
INCOMPLETE. f9/build7/build3 and all failed/raw receipts keep their original pins.
