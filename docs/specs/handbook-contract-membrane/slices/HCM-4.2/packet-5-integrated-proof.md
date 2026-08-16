# HCM-4.2 Packet 5 — integrated proof selection

This checkpoint records the bounded evidence selection made under the slice's
delegated Packet-5 preparation authority. It controls only Packet 5. The slice
README, Packets 1–4 checkpoints, and live source and tests remain controlling
where this checkpoint is silent.

## Selected proof surface

Packet 5 adds one SDK integration-test target with three invariant groups:

1. run the feature-gated schema generator twice in check mode, then bind the
   exact 53-entry checked-in manifest and schema bytes to the production
   witness, immutable roots, typed paging, and the one honest capability;
2. exercise one admitted posture request through the sealed raw-key bridge and
   compare schema-file, canonical-instance, scoped-key, request, semantic
   result, outer response, catalog entry/root/cursor, page, and receipt
   fingerprints without conflating their identity domains; and
3. compare SDK projections with Engine-, Flow-, and pipeline-owned source and
   version evidence, the exact 62-row admission inventory, and active adapter
   source/dependency/behavior evidence.

Cross-domain inequality excludes the intentional replay invariant:
`original_result_fingerprint` equals the byte-identical semantic result being
replayed. Changing only a valid request correlation preserves semantic request,
result, data, and receipts while changing the outer response fingerprint.

The raw-key vector uses one bounded literal. It may occur in the incoming wire
and the sealed owner borrow only. The integrated target checks sanitized state,
Debug output, responses, public witness/catalog values, manifest/schema assets,
and active CLI sources. The live direct-Rust predecessor test remains the
durable-byte and replay check over the same boundary.

## Reused predecessor anchors

The integration target deliberately does not duplicate every Packet 0–4
negative. Packet-5 verification reruns these focused targets together:

- `contract_membrane` for closed DTOs, terminal closure, canonical instance
  bytes, receipt identity, and secret-safe errors and Debug output;
- `contract_operation_admission` for the exact inventory, independent owner
  admission/default denial, envelope closure, and raw-key scoping;
- `contract_bootstrap_catalog` for immutable roots/pages/cursors and stale,
  tampered, expired, and cross-boundary refusal;
- `contract_schema_generation` for all-root Draft 2020-12 meta-validation,
  local-reference closure, exact tags/null/bounds, and parser/schema/typed
  admission separation;
- `contract_posture_projection` for the exact definition, two declared writes,
  honest capability, and no-write non-success projection; and
- `direct_typed_posture_api` plus the SDK posture unit tests for all 14 owner
  variants, two realized writes, applied/replayed semantic and receipt identity,
  correlation isolation, and actual durable raw-key absence.

Owner-native admission tests are rerun in the affected owner crates. The CLI
behavior anchor `help_lists_setup_first` proves the exact current command
surface excludes contract/bootstrap/posture routing. Active CLI sources and
Cargo manifests are also scanned for membrane authority or Tauri construction;
SDK-to-owner and CLI-to-SDK dependency direction must remain one-way.

## Package and documentation closure

External verification, rather than nested Cargo from the Rust test, checks that
the SDK package list contains `schemas/manifest.json` and exactly 53 Draft
2020-12 schema files, and builds SDK rustdoc with warnings denied. Schema-local
reference closure remains mechanical in `contract_schema_generation`.

Optional GitNexus FTS is unavailable in this checkout. Exact live source and the
refreshed call graph are the adopted fallback. This packet adds no adapter,
semantic owner, operation, capability, publication, handoff, proof wall, or
slice-closeout mechanism.
