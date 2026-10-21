# Independent paged-index oracle preparation

This is a design against the coordinator's provisional CP3 format relay. It is
not approval of an uncommitted format or a claim that any CP3 store has passed.
Implementation and fixture execution require an exact source pin and source
binding. Existing v1–v4 decoders and historical receipts remain unchanged.

## Authenticated entry points

Catalog4 remains physical metadata under STORE3. Descriptor.page is either
absent for a legacy whole index, or a closed object with version2, root SHA256,
root-page byte length and height0..2. Descriptor.index remains the whole-file
SHA256 used by full audit. A paged lookup must anchor its root bytes to the
catalog's root digest, rather than trusting an index file's self-description.
Mixed legacy/paged descriptors require explicit dispatch; they do not establish
the final bounded-lookup envelope until all relevant legacy routes are converted.

Every page starts with FNPIDX02. The relayed fixed header has these offsets:

| Offset | Field |
|---:|---|
| 0 | Eight-byte magic |
| 8 | Height u8 |
| 9 | Role u8: publication/result/hint/candidate/allocation =0..4 |
| 10 | Codec u8 |
| 11 | Reserved u8, exactly zero |
| 12 | Subtree record count u32LE |
| 16 | Slot count u16LE,1..48 |
| 18 | Pack-span start u64LE |
| 26 | Pack-span end u64LE |
| 34 | Pack SHA256,32 raw bytes |
| 66 | Pack byte length u64LE |
| 74 | First-key byte length u16LE, then typed ASCII key |
| 76+first_length | Last-key byte length u16LE, then typed ASCII key |

The resulting header is78+first_length+last_length bytes. Typed keys are at most
225 bytes and must preserve the expected role and safe component grammar.
Page length is at most16,384 bytes. These constants must be checked against the
actual committed implementation before the independent parser is frozen.

## Complete audit

Read an index only within the existing16MiB bound, verify Descriptor.index and
the root page digest, and recursively authenticate every referenced child.
Require root offset zero, exact preorder contiguous extents, no padding or
unreachable trailing bytes, no cycles/alias, exact page consumption and bounded
counts before allocation. Child height must equal parent height minus one.

Leaf entries contain key, pack offset, encoded/raw lengths and raw SHA256.
Require strict key order, exact first/last keys, count equality, contiguous pack
spans, codec agreement and bounded raw/encoded sizes. Never allocate from an
unchecked advertised size. Decode and hash each raw record independently.

Branch entries contain child first key, index offset/length, child digest,
subtree count and pack-span endpoints. Authenticate the entire branch, require
strict sibling fences and inherited upper bounds, exact span/count partition,
and the final child's maximum equal to the parent's last key. The recursive
preorder return cursor must end exactly at Descriptor.index_bytes.

Reuse the v4 semantic/cursor checks: accepted lineage establishes acceptance;
physical catalogs and discovery cursors do not. Check every variant of a mapped
key across all relevant segments, including a second corrupt duplicate when
the first and a loose duplicate are good. Preserve mandatory loose B publication
controls. Exact caller inputs are selected by request ID plus intent.

## Point-read evidence and independent negative controls

The engine's point operation should read only each relevant root and route.
All relevant routes must be authenticated after a hit as well as for a miss.
Negative proofs must establish the appropriate leaf/fence interval. Four
segments at three levels give at most12 pages, with the relayed16KiB page bound
matching the192KiB index-metadata envelope; startup/routing bytes remain a
separate64KiB gate. Full audit is intentionally stronger than point inspection.

Freeze negatives for unknown catalog/page versions and fields; nonzero reserved
bits; role/codec/pack binding errors; malformed key encoding; excessive height,
slots, length or counts; child overlap/alias/cycles/gaps; stale or forged fences;
wrong root/child hashes; pack-span and record-size overflow; corrupt required
record/page despite valid loose data; and corruption in another relevant
segment. Demonstrate separately that an untouched-page corruption can wait for
full audit under the explicitly authorized experimental integrity scope.

Conversion must preserve discovery exactly, including None. Compare its exact
source/semantic selectors and discovery bytes before/after; quantify reused pack
bytes, new index/coexistence bytes and actual removed bytes separately. Preserve
legacy verification and old-client refusal/captured-reader lifetimes. Do not
infer these from a passing new-reader lookup.
