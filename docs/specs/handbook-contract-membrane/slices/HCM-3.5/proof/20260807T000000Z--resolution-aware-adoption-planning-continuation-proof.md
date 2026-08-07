# HCM-3.5 resolution-aware adoption — continuation proof

**Status:** documentation-planning proof candidate. It proves the plan's
selected boundaries and proof obligations only; it proves no runtime behavior,
public interface, package, consumer, or promotion result.

## Planned seam closure

| Seam | Selected future decision | Refusal/non-claim boundary |
|---|---|---|
| Engine grounding | Engine binds exact current snapshot and compatible relation-only delta, rechecks currentness, redacts before read, and produces one bounded Resolution Projection plus bounded/redacted signal summary. | No raw snapshot/delta signal route, no duplicate consumer model, no gate/promotion decision. |
| Flow adoption | A greenfield Flow path accepts only the engine result and forwards typed packet omissions/provenance. | It does not change retained resolver contracts or convert a byte budget into Resolution. |
| Pipeline inclusion | A greenfield pipeline path consumes namespaced shared engine Resolution. | Raw `work_level` is not final authority; unknown/stale/malformed/ambiguous/overbroad mappings refuse and never fall back. |
| Transition refs | Parent handoff cites validated prior-end/start/grounding/end/delta refs. | Snapshots remain descriptive; incomplete/unstable/stale/incompatible refs cannot support grounding or promotion. |
| Evidence boundary | Local and parent dimensions are separate typed evidence values. | Packet/pipeline/local success defaults neither dimension to promotion; omitted/redacted/stale/indeterminate evidence fails closed. |
| External composition | SDK composes owner operations, standalone CLI adapts it, and Substrate later consumes/wraps published crates.io SDK/library. | Tier 2 binary/JSON stays transitional; it does not prove registry publication or permanent direct adoption. |

## Required future proof matrix

1. Engine positive/fail-closed source-pair, currentness, redaction, bounded
   summary, overflow, omission, and no-raw-signal evidence.
2. Flow projection/packet provenance, byte-budget non-widening, refusal, and
   retained-resolver regression evidence.
3. Pipeline deterministic namespaced inclusion, mapping provenance, malformed
   and fallback refusal, and retained scoped-corpus regression evidence.
4. Parent-transition exact-reference and partial/unstable/stale/incompatible
   refusal corpus.
5. Local-versus-parent non-promotion matrix under the separately selected gate
   owner.
6. Separate SDK/CLI/Tier-2/publication/current-tip Substrate proofs under the
   existing program gates.

## Planning-only validation posture

The documentation subject is checked for Markdown integrity, whitespace, and
scoped diff only. GitNexus MCP/CLI/index is unavailable and is recorded as
unavailable, never GREEN. No runtime test, schema validation, registry query,
remote access, publication, or consumer-adoption evidence is asserted.

The parent must obtain the required fresh independent documentation review
before any closeout. This proof creates no durable parent result, ledger entry,
or completion status.
