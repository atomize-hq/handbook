# Native macOS strict audit correction — October 18, 2026

**Current tested source pin: `7e00d22b470c1eed476127d2f585789036e79cf3`.** Locked workspace tests (two Rust tests), release build and `audit_evidence.py --strict` passed on fresh **N128 A/B** fixtures. All **18 negative-evidence cases** returned errors without recreating missing evidence or repairing corrupt evidence. After each exact restoration, the subsequent audit succeeded. The lane contains **42 uncensored responses**: 24 exit 0 and 18 expected refusal exit 2. No requested check was left unperformed.

Executable SHA-256: `9991bd5640feb12fd4dbf5245cd61874d935f2f0a151e68b11a52a5a35856de6`. All **70 capsule files** match exact Git bytes. Only prior `native-macos/` report transport was excluded from the executed source capsule; no pinned engine/harness bytes were edited on Mac. Full source/session snapshots, raw Driver events, stdout/stderr byte bindings, wrapper logs and source/executable hashes are retained in [evidence/audit-7e00d22](evidence/audit-7e00d22/verification.json).

## Reached audit result

| Damaged selected evidence | Negative cases across A/B | Result |
| --- | ---: | --- |
| Projection child object missing | 2 | Error; object remains absent |
| Derived navigation source missing | 2 | Error; source remains absent |
| Projection root object missing | 2 | Error; root remains absent |
| Requested-packet navigation source missing/corrupt | 4 | Error; original bytes not recreated |
| Requested packet context source missing/corrupt | 4 | Error; original bytes not recreated |
| Requested packet provenance source missing/corrupt | 4 | Error; original bytes not recreated |
| Total | **18** | **18/18 refused without repair** |

All 18 audits after exact byte restoration passed. The remaining six successful responses initialize/setup the A/B fixtures and publish their requested packets. All **42 JSON responses** conserve total read/written bytes across payload/control/metadata/result roles. These are process-response/case counts, not independent statistical trials. [audit-verification.json](audit-verification.json) records each named case/error, restore success count, exact timing and caps.

The strict suite took **22.880 seconds** under its 900-second cap. Locked tests took 9.442 seconds and release build 9.187 seconds, each under 300 seconds. Compiler warnings are retained unchanged. Consumer binaries were built; no new standalone consumer run is claimed.

## Defect and correction boundary

The supplied 7ac reproducer showed that full audit could render its reference projection before reading selected child/navigation evidence, recreating absent evidence and concealing damage. Missing/corrupt requested-packet selected sources also lacked the required independent negative checks. The corrected core preflights selected projection/source evidence before rendering the full oracle. This audit-only correction does not change ordinary publication/history behavior or on-disk formats. The native strict regression proves the named missing/corrupt cases on the corrected pin. Mac did not rerun the old 7ac before-probe; the coordinator's Windows reproduction remains separate provenance.

No broad smoke/matrix, history, replay-lifetime, fault suite or production-file run was added in this correction followup. Earlier positive audits on 7ac do not establish the negative-evidence property tested here. Earlier observations remain qualified by their exact source pin; they are not relabeled as new-core measurements.

## Separate clock and finite caps

The prospectively recorded audit amendment in the pinned allocation.md authorizes this **new explicit <=1,800-second correction followup**, including locked build/tests and export. It does **not** reopen or reset the original native measurement window, which had already closed.

- Correction start: **2026-10-18 21:53:07 UTC** / epoch `1792360387` (first tool-time observation).
- Total correction deadline: **2026-10-18 22:23:07 UTC** / epoch `1792362187`.
- Locked tests/build: <=300 seconds each; strict N128 A/B suite <=900 seconds; each unchanged Driver call <=180 seconds.
- At postrun verification: 121.279 seconds elapsed and 1678.721 seconds remained. Export/report/commit/push continue within that same deadline; later receipts record their actual elapsed time.
- Whole owned allocation remains <=6 GiB, with >=20 GiB volume free reserve. The process wrapper checks the whole root and keeps a 512 MiB allocation margin; only its own process group can be stopped on a limit. No limit fired.

[allocation-audit-correction.json](allocation-audit-correction.json) was frozen before build and binds the authority allocation Git blob plus the unchanged original allocation hash. Actual argv/cwd/caps/start/end/exit and log hashes are in evidence/audit-7e00d22/outer--runs.jsonl. Resource snapshots include all prior and current source/worktrees/build/runtime/evidence/report, the conservative isolated transport Git database and all five tiny bounds-unit fixtures. No cleanup was performed.

## Version-qualified prior evidence

| Source/lane | Retained responses | Qualification |
| --- | ---: | --- |
| 7e00d22 current strict audit | **42** | N128 A/B negative evidence only |
| 7ac70f9 replay correction | 610 | N512 replay retention plus affected suites; [historical report](7ac70f9-replay-report.md) |
| a1108c6 affected lane | 570 | Ancestor-name collection/byte-role correction; [historical report](a1108c6-affected-report.md) |
| F3 main/companions | 726 | Earlier 1k smoke, consumer/copy and interruption controls; [historical snapshot](f3-bounded-path-report.md) |
| fb73e75 development | 117 | Partial pre-smoke run stopped on the common-pin hold |

All seven evidence lanes preserve **2,065 responses**, zero censors, under their own source/session/executable bindings. The earlier raw evidence is byte-identical to its predecessor report commit. This aggregate is an inventory, not one new-pin benchmark.

F3 primary payload/control/result byte-role bins remain invalid under its ancestor `results`; raw values are unchanged, total bytes/objects metadata/times remain separately qualified. F3's real ancestor-`objects` GC bug is not recast as unsupported path policy. Path-neutral companion role observations do not replace 1k values. performance.md and smoke JSON remain F3-only; no later-pin speedup, history, p95, device-cold or cross-host ranking claim follows. Holder interruption remains exit 86, not SIGKILL/reboot/power loss. The `/proc` memory helper remains null on Mac. Existing experimental owner/versioned contracts and the proposed status of the structured format standard remain unchanged; no production migration is authorized.

## Preserved paths and report transport

- Whole root: `/private/tmp/file-native-incremental-macos-20261018`
- New source worktree: `/private/tmp/file-native-incremental-macos-20261018/audit-checkout` at 7e00d22
- Executed capsule: `/private/tmp/file-native-incremental-macos-20261018/source-audit-core/docs/research/file-native-incremental-engine-experiment-2026-10-18`
- Build: `/private/tmp/file-native-incremental-macos-20261018/audit-lane/build`
- Disposable strict runtime: `/private/tmp/file-native-incremental-macos-20261018/audit-lane/strict`
- Local logs/pins: `/private/tmp/file-native-incremental-macos-20261018/evidence-audit`
- Result checkout: `/private/tmp/file-native-incremental-macos-20261018/corrected-checkout`

Every predecessor root is preserved. The shared Mac checkout remains read-only and clean at 379af4eda9b95ee82be9d3afcfdcdca2d6d132f9. Environment remains native Darwin 25.6.0/macOS 26.6.1 arm64, APFS, Rust/Cargo 1.89.0, Python 3.11.9 and Bash 3.2. Earlier fsync/F_FULLFSYNC/link evidence remains labeled with its original sampled version; no new power-loss qualification is inferred.

The report-only follow-up is parented to `fe6e01e287bb6d4592059b1a6a9170789ab1c631` on the existing Mac report branch. It changes only `native-macos/` artifacts and does not transplant newer engine files into that branch. The tested core is independently pinned at 7e00d22 in the fresh source capsule and archives. [helpers/launch_audit.py](helpers/launch_audit.py) preserves the actual finite launcher; [helpers/verify_audit_run.py](helpers/verify_audit_run.py) verifies reached cases without another engine call. Reproduction uses a fresh exact capsule/target/base and `audit_evidence.py --exe <binary> --base <fresh-base> --strict`; never rerun into retained roots.

Portable verification after exact-byte Git materialization:

```bash
python3 helpers/verify_export.py .
```

The verifier checks raw gzip hashes/counts, stdout/stderr hashes/lengths, JSON type/parse association, source/session ZIP/TAR bytes against per-lane pins and executable-hash associations. The separately copied strict summary is hash-bound in audit-summary-binding.json. Physical binaries stay on Mac and were rehashed locally; no reproducible-build claim is made. Local `.gitattributes` preserves bytes, including earlier exact CRLF helper/log evidence. Names remain portable/collision-checked. GitNexus cannot map this unregistered isolated report checkout; coverage is unknown and manual staged path/index/source proof is recorded. No PR, force push, shared-checkout mutation or production change occurred.
