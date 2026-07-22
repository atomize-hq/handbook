# HCM-2.3 Planning Review 7 Remediation

## Review authority

- reviewer: `/root/hcm_2_3_planning_review_7`
- verdict: `CHANGES_REQUIRED`
- admitted subject: 28 paths, aggregate
  `sha256:cf6e1b34a3bd796028c580f6e813ca999439d790be72488b30c66c391189f200`
- findings: zero Critical, three Required, zero Optional, zero Nit
- disposition: all three Required findings were accepted; none was waived

## Exact dispositions

| Finding | Accepted defect | Remediation |
|---|---|---|
| Review 7 Required 1 | promotion `1.2` cited the current context fingerprint while retaining stale, differently ordered kind/schema definition fingerprints | replaced promotion `resolved_definitions` with the operation context's exact UTF-8-sorted projection; regenerated promotion identity, persisted digest, output ref, success intent, both verified-stage alternatives, marker, evidence, result, and retained ledger; added mismatch/reorder semantic attacks and an exact cross-suite equality assertion |
| Review 7 Required 2 | the refused promotion result/replay cited the unauthorized `transactions/promotions/` alias and the schema did not discriminate evidence families | changed both refs to the sole `transactions/artifact-promotions/` family; regenerated refused result/replay and retained ledger; made all result evidence refs operation-family-specific in the schema; added committed/refused wrong-family schema rejections plus exact transaction-ID semantic checks |
| Review 7 Required 3 | selection/runtime exact-ref and safe/repository-path definitions admitted inherited-grammar violations | aligned all three HCM-2.3 schemas to the HCM-1.1 3–255-byte/two-segment/1–63-byte/lowercase identity grammar, byte-canonical full SemVer, and 1,024-ASCII-byte/64-component normalized relative-path grammar; added executable rejection and acceptance vectors at the grammar and length boundaries; aligned SPEC, runtime contract, plan, and todo |

## Regenerated promotion lineage

The repaired promotion repeats the operation-context definition array exactly:
artifact kind `222…`, intake `ccc…`, profile `999…`, and schema entry `444…`.
The resulting promotion identity is
`sha256:d3dfb97fa1e24ac8a16651fa733cf60d83bdae4ecdcaddce1c73c263146ff8ae`.
Its complete JCS+LF persisted bytes are 1,933 bytes with digest
`sha256:233ddaaefe0464d88426d23eeee81802def07e043d9d48f7bd0c83df1d71e26f`.

The dependent successful promotion chain is:

- intent `sha256:f04f164e6d1cc50bf0b40c9fe3b0372f618f4f89df3de3e3081507665d7ae31f`;
- staging verification `sha256:4ea350fdc047f07e69087322eb1829fc2da4f8badcdf7656d8ee8f2d424dd233`;
- installed-complete verification `sha256:59837bdc6f5b705273fe577dd4d2f21fab433947ead9f39c9f10d2979aecd8e8`;
- marker `sha256:13d0d2169c12f681d0b4877773c5fdecd9ccdabe2c197c0d529d878f74b32d01`;
- evidence `sha256:ccbf263155a0f3d651e602768df379c25178b6258727282eef7c644603226379`;
- result `sha256:c50ae16d857167e3a9cb1a67b816907ff5c29c497f94907f6c97a958488df85c`;
  and
- retained ledger `sha256:ea3ec3945c0aec76d1b70cee6f12032a8131a3aebcaa016d986d56292dbaa73b`.

The corrected refused promotion result/replay identity is
`sha256:a8e9c2104b4e653bb090329c1adeb4024d3caab5fb45fae3d8789115d6396549`
and its retained ledger identity is
`sha256:1d3df0e8a6c6a49c55c7c31eaf35601bad006d8b5e26d5c532df2b2596738b7e`.
All earlier remediation records remain immutable historical evidence; these
current identities supersede their promotion assertions.

## Schema boundary proof

The selection and runtime suites each now contain ten concrete rejection and
four concrete acceptance mutations. The control suite contains eight rejection
and four acceptance mutations. A runner copies the named positive base record,
applies the exact JSON Pointer replacement, and requires Draft 2020-12 rejection
or acceptance as declared.

Rejections cover one-segment, underscore, repeated-hyphen, over-63-byte segment,
over-255-byte identity, and noncanonical-SemVer refs; dot component, trailing
slash, 65 components, and 1,025-byte paths; plus committed/refused wrong evidence
families. Acceptances cover canonical prerelease/build SemVer, a 63-byte
identity segment, 64 path components, and a 1,024-byte path. All 36 boundary
mutations replayed as declared.

## Parent consistency hardening

A parent audit also removed any physical two-record interpretation of
establishment. `active_hold` is now solely the logical lookup projection of one
closed intent, never separately persisted. The runtime contract freezes the
same-filesystem `.intent.writing` → established `.intent` → pending
`intent.json` transitions, the first atomic publication as the sole
establishment point, the byte-preserving second rename, scratch-only cleanup,
every legal crash state, and all conflict states. This prevents a later
implementation from inventing an impossible atomic hold-plus-intent two-file
commit or a second hold authority.

## Verification replay

Independent duplicate-safe Draft 2020-12 and semantic replay reported:

- all three schemas pass meta-validation and every declared positive validates;
- all 36 schema boundary mutations accept/reject exactly as declared;
- both selection orders reproduce the same normalized fingerprint;
- all four runtime record identities and persisted byte claims reproduce;
- promotion definitions equal the operation-context projection byte-for-byte;
- all 39 control terminal fingerprints and all transaction IDs reproduce;
- every successful/refused intent, stage, marker, evidence, result, replay, and
  retained-ledger dependency agrees;
- every result evidence ref uses the exact operation family and identical
  transaction ID; and
- `git diff --check` passes.

## Scope and next review

The remediation changed only HCM-2.3 contracts, vectors, SPEC, plan, todo, and
this proof record. It did not edit Rust, Cargo, runtime tests, production assets,
HCM-2.2 authority, a handoff ledger, or a preservation location. The repaired
complete documentation subject requires a different fresh isolated read-only
reviewer and remains unauthorized for implementation or commit until that
complete subject is CLEAN.
