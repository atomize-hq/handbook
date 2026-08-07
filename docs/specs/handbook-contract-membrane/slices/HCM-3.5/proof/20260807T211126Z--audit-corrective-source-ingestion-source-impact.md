# HCM-3.5 audit-corrective source-ingestion source impact

**Corrective parent:** `handbook-hcm-3-5-audit-corrective-implementation-20260807`

The local GitNexus MCP graph and runner are unavailable in this checkout, so
no upstream-impact or change-detection result is claimed as GREEN. Manual
read-only caller inventory found `ground_resolution` to be the public P1
operation and `materialize_grounding_transition_refs` to be the private P4
materializer. The correction does not alter either signature or public JSON
transport.

| Changed boundary | Scope | Failure mapping |
| --- | --- | --- |
| `read_bounded_regular_source` in `definition_identity` | New crate-private adapter over the existing strict local source reader and `SourceByteBudget` | Internal `RegistryLoadErrorKind`; generic details only. |
| `grounding` readers/parser | P1's definition, disclosure, source pair, and currentness inputs | `MissingSource` for missing input; every other invalid input is existing `MalformedSource`; semantic mismatches retain their existing types. |
| `grounding_transition` readers/parser | Every P4 route, definition, disclosure, and projection input | Existing `GroundingTransitionMaterializationError::InvalidSource` only. |
| `snapshot_memory::json_value` | Existing HCM-3.5 derived source JSON parser | Existing internal `GroundingSourceError::InvalidSource` only. |

No function/class/method impact result exists because the required local
GitNexus tooling is absent. No HIGH or CRITICAL result was returned, and no
unavailable condition is reclassified as LOW risk. The complete final review
must reassess these exact boundaries plus all original P1–P4 code paths.
